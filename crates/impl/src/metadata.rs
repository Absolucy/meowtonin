// SPDX-License-Identifier: 0BSD
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Attribute, Expr, ExprLit, FnArg, ItemFn, Lit, Meta, Pat, PatType, Type, ext::IdentExt};

fn doc_lines(attrs: &[Attribute]) -> Vec<String> {
	attrs
		.iter()
		.filter_map(|attr| {
			let Meta::NameValue(name_value) = &attr.meta else {
				return None;
			};
			if !name_value.path.is_ident("doc") {
				return None;
			}
			let Expr::Lit(ExprLit {
				lit: Lit::Str(text),
				..
			}) = &name_value.value
			else {
				return None;
			};
			Some(text.value())
		})
		.flat_map(|text| {
			text.split('\n')
				.map(|line| line.strip_prefix(' ').unwrap_or(line).trim_end().to_owned())
				.collect::<Vec<_>>()
		})
		.collect()
}

/// rustc rejects `///` on a parameter, so these have to be gone before the
/// function is emitted. Returns one entry per typed parameter, in order.
pub fn take_param_docs(func: &mut ItemFn) -> Vec<Vec<String>> {
	func.sig
		.inputs
		.iter_mut()
		.filter_map(|input| match input {
			FnArg::Typed(param) => Some(param),
			FnArg::Receiver(_) => None,
		})
		.map(|param| {
			let docs = doc_lines(&param.attrs);
			param.attrs.retain(|attr| !attr.path().is_ident("doc"));
			docs
		})
		.collect()
}

pub fn typed_params(func: &ItemFn) -> Vec<&PatType> {
	func.sig
		.inputs
		.iter()
		.filter_map(|input| match input {
			FnArg::Typed(param) => Some(param),
			FnArg::Receiver(_) => None,
		})
		.collect()
}

fn is_option(ty: &Type) -> bool {
	let Type::Path(path) = ty else {
		return false;
	};
	path.qself.is_none()
		&& path
			.path
			.segments
			.last()
			.is_some_and(|segment| segment.ident == "Option")
}

/// `quote` prints `Vec < u8 >` and `a : u8`; squash that back into
/// something that reads like Rust.
fn tidy_signature(tokens: &TokenStream2) -> String {
	let mut text = tokens
		.to_string()
		.split_whitespace()
		.collect::<Vec<_>>()
		.join(" ");
	for (from, to) in [
		(",)", ")"),
		(" :: ", "::"),
		(" : ", ": "),
		(" < ", "<"),
		(" <", "<"),
		("< ", "<"),
		(" >", ">"),
		(" ,", ","),
		(" (", "("),
		("( ", "("),
		(" )", ")"),
		("[ ", "["),
		(" ]", "]"),
		("& ", "&"),
		("->(", "-> ("),
	] {
		text = text.replace(from, to);
	}
	text
}

pub fn submit_tokens(
	func: &ItemFn,
	param_docs: &[Vec<String>],
	dm_name: &str,
	variadic: bool,
) -> TokenStream2 {
	let params = typed_params(func);
	let symbol = func.sig.ident.to_string();
	let docs = doc_lines(&func.attrs);
	let signature = tidy_signature(&func.sig.to_token_stream());

	let trailing_options = params
		.iter()
		.rev()
		.take_while(|param| is_option(&param.ty))
		.count();
	let optional_from = if variadic || trailing_options == 0 {
		quote!(::core::option::Option::None)
	} else {
		let first = params.len() - trailing_options;
		quote!(::core::option::Option::Some(#first))
	};

	let param_entries = params
		.iter()
		.zip(param_docs)
		.enumerate()
		.map(|(idx, (param, docs))| {
			let name = match &*param.pat {
				Pat::Ident(ident) => ident.ident.unraw().to_string(),
				_ => format!("arg_{}", idx + 1),
			};
			quote! {
				::meowtonin::bindings::ExportedParam {
					name: #name,
					docs: &[#(#docs),*],
				}
			}
		});

	quote! {
		::meowtonin::inventory::submit! {
			::meowtonin::bindings::ExportedFn {
				symbol: #symbol,
				dm_name: #dm_name,
				docs: &[#(#docs),*],
				params: &[#(#param_entries),*],
				signature: #signature,
				variadic: #variadic,
				optional_from: #optional_from,
			}
		}
	}
}
