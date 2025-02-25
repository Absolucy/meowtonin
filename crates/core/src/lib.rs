// SPDX-License-Identifier: 0BSD
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#![allow(unused_unsafe, clippy::missing_safety_doc)]
#![cfg_attr(debug_assertions, allow(dead_code))]

#[macro_use]
pub mod error;
pub mod from;
pub mod init;
pub mod misc;
pub mod panic;
//#[doc(hidden)]
//pub mod panic_old;
pub mod proc;
pub mod strid;
pub mod sync;
pub mod to;
pub mod value;
pub mod xyz;

pub mod sys {
	pub use meowtonin_byondapi_sys::*;
}

pub use crate::{
	error::{ByondError, ByondResult},
	from::FromByond,
	proc::call_global,
	to::ToByond,
	value::{typecheck::ByondValueType, ByondValue},
	xyz::ByondXYZ,
};
pub use inventory;
pub use meowtonin_impl::byond_fn;

/// A simple macro to create a [`ByondValue`] from any Rust value that
/// implements [`ToByond`].
#[macro_export]
macro_rules! byondval {
	(const $value:expr) => {{
		static __BYONDVAL: ::std::sync::OnceLock<$crate::value::ByondValue> =
			::std::sync::OnceLock::new();
		__BYONDVAL
			.get_or_init(|| {
				$crate::ToByond::to_byond(&$value)
					.expect("failed to initialize const byondval")
					.persist()
			})
			.clone()
	}};
	($value:expr) => {
		$crate::ToByond::to_byond(&$value).unwrap()
	};
}

/// # Safety
/// Don't pass in a null argv pointer please god
/// Just give this what BYOND gives you and pray for the best
#[doc(hidden)]
pub unsafe fn parse_args(argc: sys::u4c, argv: *mut ByondValue) -> Vec<ByondValue> {
	if argc == 0 || argv.is_null() {
		return Vec::new();
	}
	unsafe { std::slice::from_raw_parts_mut(argv, argc as usize).to_vec() }
}

/// Returns the current major version and build version of BYOND.
pub fn byond_version() -> (u32, u32) {
	use std::sync::OnceLock;

	static VERSION: OnceLock<(u32, u32)> = OnceLock::new();
	*VERSION.get_or_init(|| unsafe { sys::get_byond_version() })
}
