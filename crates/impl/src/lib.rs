// SPDX-License-Identifier: 0BSD
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, PatType, ReturnType};

/// Generates argument parsing code for a function parameter
fn generate_arg_parser(input: &FnArg, idx: usize) -> TokenStream2 {
	if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
		let mutability = attrs.iter().find(|attr| attr.path().is_ident("mut"));
		quote! {
			let #mutability #pat: #ty = ::meowtonin::FromByond::from_byond(&__byond_args[#idx])
				.expect("failed to parse argument");
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
			Ok(::meowtonin::ByondValue::null())
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
) -> TokenStream2 {
	quote! {
		fn #wrapper_ident(mut __byond_args: &[::meowtonin::ByondValue])
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
	quote! {
		#[no_mangle]
		#[inline(never)]
		pub unsafe extern "C" fn #func_name(
			__argc: ::meowtonin::sys::u4c,
			__argv: *mut ::meowtonin::ByondValue
		) -> ::meowtonin::ByondValue {
			::meowtonin::panic::setup_panic_hook();
			let mut __args = unsafe { ::meowtonin::parse_args(__argc, __argv) };
			if __args.len() < #length {
				__args.extend((0..#length - __args.len())
					.map(|_| ::meowtonin::ByondValue::default()))
			}

			match ::std::panic::catch_unwind(|| #wrapper_ident(&__args)) {
				Ok(Ok(value)) => value,
				Ok(Err(err)) => {
					let error = err.to_string();
					let source = #func_name_str.to_string();
					let _ = ::meowtonin::call_global::<_, _, _, ()>("meowtonin_stack_trace", [error, source]);
					::meowtonin::ByondValue::null()
				},
				Err(_err) => {
					::meowtonin::panic::stack_trace_if_panic();
					::meowtonin::ByondValue::null()
				}
			}
		}
	}
}

/// Main proc macro attribute that generates BYOND FFI bindings
#[proc_macro_attribute]
pub fn byond_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let func = parse_macro_input!(item as ItemFn);

	let func_name = &func.sig.ident;
	let wrapper_name = format!("__byond_{}_inner", func_name);
	let wrapper_ident = syn::Ident::new(&wrapper_name, func_name.span());

	let mod_name = format!("__byond_export_{}", func_name);
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
	);

	// Generate the exported FFI function
	let export_fn = generate_export_fn(func_name, &wrapper_ident, func.sig.inputs.len());

	// Combine everything into the final output
	let gen = quote! {
		#func

		#[doc(hidden)]
		mod #mod_ident {
			use super::*;

			#wrapper_fn
			#export_fn
		}
	};

	gen.into()
}
