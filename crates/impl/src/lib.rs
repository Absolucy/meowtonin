// SPDX-License-Identifier: 0BSD
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{FnArg, ItemFn, PatType, ReturnType, parse_macro_input, spanned::Spanned};

/// Generates argument parsing code for a function parameter
fn generate_arg_parser(input: &FnArg, idx: usize) -> TokenStream2 {
	if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
		let mutability = attrs.iter().find(|attr| attr.path().is_ident("mut"));
		let arg_name = syn::Ident::new(&format!("__arg_{}", idx), pat.span());
		let error_message = format!(
			"failed to parse argument {idx} ({pat}: {ty})",
			idx = idx + 1,
			pat = pat.to_token_stream(),
			ty = ty.to_token_stream(),
		);
		quote! {
			let #mutability #pat: #ty = ::meowtonin::FromByond::from_byond(#arg_name)
				.expect(#error_message);
		}
	} else {
		quote!()
	}
}

/// Generates the return type conversion code based on the function's return
/// type
fn generate_return_conversion(ret_type: &ReturnType) -> (TokenStream2, TokenStream2) {
	match ret_type {
		ReturnType::Default => (quote!(()), quote! {
			Ok(::meowtonin::ByondValue::NULL)
		}),
		ReturnType::Type(_, ty) => {
			let ty_name = quote!(#ty).to_string();
			let conversion = if ty_name.contains("Result") {
				quote! {
					ret
						.map_err(::std::boxed::Box::from)
						.and_then(|inner_ret| ::meowtonin::ByondValue::new_value(inner_ret)
							.map_err(::std::boxed::Box::from))
				}
			} else {
				quote! {
					::meowtonin::ByondValue::new_value(ret).map_err(::std::boxed::Box::from)
				}
			};
			(quote!(#ty), conversion)
		}
	}
}

/// Generates the wrapper function that handles argument parsing and return
/// conversion
fn generate_wrapper_fn(
	wrapper_ident: &syn::Ident,
	parse_args: &[TokenStream2],
	return_type: &TokenStream2,
	return_conversion: &TokenStream2,
	body: &syn::Block,
	arg_count: usize,
) -> TokenStream2 {
	let args_ident = if arg_count > 0 {
		let arg_params: Vec<_> = (0..arg_count)
			.map(|i| {
				let arg_name =
					syn::Ident::new(&format!("__arg_{}", i), proc_macro2::Span::call_site());
				quote! { #arg_name: ::meowtonin::ByondValue }
			})
			.collect();
		quote! { #(#arg_params),* }
	} else {
		quote! {}
	};

	quote! {
		fn #wrapper_ident(#args_ident)
			-> ::std::result::Result<::meowtonin::ByondValue, ::std::boxed::Box<dyn ::std::error::Error>>
		{
			#(#parse_args)*

			let mut __func = move || -> #return_type {
				#body
			};
			let ret = __func();

			#return_conversion
		}
	}
}

/// Generates the FFI export function that handles panic catching and error
/// conversion
fn generate_export_fn(
	func_name: &syn::Ident,
	wrapper_ident: &syn::Ident,
	length: usize,
) -> TokenStream2 {
	let func_name_str = func_name.to_string();

	let let_args = if length > 0 {
		quote! {
			let mut __args = unsafe { ::meowtonin::parse_args(__argc, __argv) };
		}
	} else {
		quote! {}
	};

	let do_call = if length > 0 {
		let args: Vec<_> = (0..length)
			.map(|_| quote! { __args_iter.next().unwrap_or(::meowtonin::ByondValue::NULL) })
			.collect();
		quote! {
			let mut __args_iter = __args.into_iter();
			#wrapper_ident(#(#args),*)
		}
	} else {
		quote! {
			#wrapper_ident()
		}
	};

	quote! {
		#[unsafe(no_mangle)]
		#[inline(never)]
		pub unsafe extern "C-unwind" fn #func_name(
			__argc: ::meowtonin::sys::u4c,
			__argv: *mut ::meowtonin::ByondValue
		) -> ::meowtonin::ByondValue {
			::meowtonin::setup_once();
			let __retval: std::result::Result<::meowtonin::ByondValue, std::string::String>;
			{
				#let_args

				match ::std::panic::catch_unwind(move || {
					#do_call
				}) {
					Ok(Ok(value)) => {
						__retval = Ok(value);
					},
					Ok(Err(err)) => {
						__retval = Err(format!(
							"panic at {source}: {error}",
							error = err.to_string(),
							source = #func_name_str.to_string()
						));
					},
					Err(_err) => match ::meowtonin::panic::get_stack_trace() {
						Some(message) => {
							__retval = Err(message);
						}
						None => {
							__retval = Err("unknown error".to_owned());
						}
					}
				}
			}
			match __retval {
				Ok(value) => value,
				Err(error) => ::meowtonin::panic::byond_crash(error)
			}
		}
	}
}

/// Main proc macro attribute that generates BYOND FFI bindings
#[proc_macro_attribute]
pub fn byond_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let func = parse_macro_input!(item as ItemFn);

	let func_name = &func.sig.ident;
	let wrapper_name = format!("__byond_{func_name}_inner");
	let wrapper_ident = syn::Ident::new(&wrapper_name, func_name.span());

	let mod_name = format!("__byond_export_{func_name}");
	let mod_ident = syn::Ident::new(&mod_name, func_name.span());

	// Generate argument parsing code for each parameter
	let parse_args: Vec<_> = func
		.sig
		.inputs
		.iter()
		.enumerate()
		.map(|(idx, input)| generate_arg_parser(input, idx))
		.collect();

	// Generate return type handling code
	let (return_type, return_conversion) = generate_return_conversion(&func.sig.output);

	// Generate the wrapper function
	let wrapper_fn = generate_wrapper_fn(
		&wrapper_ident,
		&parse_args,
		&return_type,
		&return_conversion,
		&func.block,
		func.sig.inputs.len(),
	);

	// Generate the exported FFI function
	let export_fn = generate_export_fn(func_name, &wrapper_ident, func.sig.inputs.len());

	// Combine everything into the final output
	quote! {
		#func

		#[doc(hidden)]
		mod #mod_ident {
			use super::*;

			#wrapper_fn
			#export_fn
		}
	}
	.into()
}
