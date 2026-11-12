// SPDX-License-Identifier: 0BSD
mod metadata;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{ItemFn, PatType, ReturnType, parse_macro_input, spanned::Spanned};

#[derive(Debug, FromMeta, Clone)]
#[darling(derive_syn_parse)]
struct ByondFnArgs {
	#[darling(default)]
	variadic: bool,
	#[darling(default)]
	debug_log: bool,
	name: Option<String>,
}

/// Generates the code that parses one argument into a temporary, which the
/// wrapper then passes to the original function.
fn generate_arg_parser(param: &PatType, idx: usize) -> TokenStream2 {
	let PatType { pat, ty, .. } = param;
	let arg_name = syn::Ident::new(&format!("__arg_{idx}"), pat.span());
	let parsed_name = syn::Ident::new(&format!("__parsed_{idx}"), pat.span());
	let error_message = format!(
		"failed to parse argument {idx} ({pat}: {ty})",
		idx = idx + 1,
		pat = pat.to_token_stream(),
		ty = ty.to_token_stream(),
	);
	quote! {
		let #parsed_name: #ty = ::meowtonin::FromByond::from_byond(#arg_name)
			.expect(#error_message);
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

/// Generates the wrapper function that handles argument parsing, calls the
/// original function, and converts its return value
fn generate_wrapper_fn(
	wrapper_ident: &syn::Ident,
	func_name: &syn::Ident,
	params: &[&PatType],
	return_type: &TokenStream2,
	return_conversion: &TokenStream2,
	variadic: bool,
) -> TokenStream2 {
	let (wrapper_params, parse_block, call_args) = if variadic {
		(
			quote! { __args: ::std::vec::Vec<::meowtonin::ByondValue> },
			quote! {},
			quote! { __args },
		)
	} else {
		let arg_names = (0..params.len())
			.map(|idx| syn::Ident::new(&format!("__arg_{idx}"), proc_macro2::Span::call_site()))
			.collect::<Vec<_>>();
		let parsed_names = (0..params.len())
			.map(|idx| syn::Ident::new(&format!("__parsed_{idx}"), proc_macro2::Span::call_site()))
			.collect::<Vec<_>>();
		let parsers = params
			.iter()
			.enumerate()
			.map(|(idx, param)| generate_arg_parser(param, idx));
		(
			quote! { #(#arg_names: ::meowtonin::ByondValue),* },
			quote! { #(#parsers)* },
			quote! { #(#parsed_names),* },
		)
	};

	// has to be `super::`, a bare name would find the export below it instead
	quote! {
		fn #wrapper_ident(#wrapper_params)
			-> ::std::result::Result<::meowtonin::ByondValue, ::std::boxed::Box<dyn ::std::error::Error>>
		{
			#parse_block

			let ret: #return_type = super::#func_name(#call_args);

			#return_conversion
		}
	}
}

fn generate_debug_msg(func_name: &str, msg_type: &str, args: &ByondFnArgs) -> TokenStream2 {
	if !args.debug_log {
		return quote! {};
	}
	let msg = format!("debug: {func_name} {msg_type}");
	quote! {
		{ println!(#msg) };
	}
}

/// Generates the FFI export function that handles panic catching and error
/// conversion
fn generate_export_fn(
	func_name: &syn::Ident,
	wrapper_ident: &syn::Ident,
	length: usize,
	args: &ByondFnArgs,
) -> TokenStream2 {
	let func_name_str = func_name.to_string();

	let debug_start = generate_debug_msg(&func_name_str, "start", args);
	let debug_end = generate_debug_msg(&func_name_str, "end", args);
	let debug_crash = generate_debug_msg(&func_name_str, "CRASH!!!", args);

	let let_args = if args.variadic || length > 0 {
		quote! {
			let mut __args = unsafe { ::meowtonin::parse_args(__argc, __argv) };
		}
	} else {
		quote! {}
	};

	let do_call = if args.variadic {
		quote! {
			#wrapper_ident(__args)
		}
	} else if length > 0 {
		let args: Vec<_> = (0..length)
			.map(|_| {
				quote! {
					__args_iter
						.next()
						.unwrap_or(::meowtonin::ByondValue::NULL)
				}
			})
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
			__argv: *mut ::meowtonin::sys::CByondValue
		) -> ::meowtonin::sys::CByondValue {
			::meowtonin::setup_once();
			let __retval: std::result::Result<::meowtonin::ByondValue, std::string::String>;
			{
				#debug_start
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
				Ok(value) => {
					#debug_end
					value.detach()
				},
				Err(error) => {
					#debug_crash
					::meowtonin::panic::byond_crash(error)
				}
			}
		}
	}
}

/// Main proc macro attribute that generates BYOND FFI bindings
#[proc_macro_attribute]
pub fn byond_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
	let args: ByondFnArgs = match syn::parse(attr) {
		Ok(v) => v,
		Err(e) => {
			return e.to_compile_error().into();
		}
	};

	let mut func = parse_macro_input!(item as ItemFn);
	let param_docs = metadata::take_param_docs(&mut func);

	let func_name = &func.sig.ident;
	let wrapper_name = format!("__byond_{func_name}_inner");
	let wrapper_ident = syn::Ident::new(&wrapper_name, func_name.span());

	let mod_name = format!("__byond_export_{func_name}");
	let mod_ident = syn::Ident::new(&mod_name, func_name.span());

	let (return_type, return_conversion) = generate_return_conversion(&func.sig.output);

	let params = metadata::typed_params(&func);
	let wrapper_fn = generate_wrapper_fn(
		&wrapper_ident,
		func_name,
		&params,
		&return_type,
		&return_conversion,
		args.variadic,
	);

	let export_fn = generate_export_fn(func_name, &wrapper_ident, params.len(), &args);

	let dm_name = args.name.clone().unwrap_or_else(|| func_name.to_string());
	let submit = metadata::submit_tokens(&func, &param_docs, &dm_name, args.variadic);

	quote! {
		#func

		#[doc(hidden)]
		mod #mod_ident {
			use super::*;

			#wrapper_fn
			#export_fn
			#submit
		}
	}
	.into()
}
