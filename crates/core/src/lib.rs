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

pub mod byond;
#[macro_use]
pub mod error;
pub mod from;
pub mod init;
pub mod misc;
pub mod panic;
pub mod pixloc;
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
	byond::byond,
	error::{ByondError, ByondResult},
	from::FromByond,
	proc::call_global,
	sys::ByondVersion,
	to::ToByond,
	value::{typecheck::ByondValueType, ByondValue},
	xyz::ByondXYZ,
};
pub use inventory;
pub use meowtonin_impl::byond_fn;
use std::sync::Once;

/// A simple macro to create a [`ByondValue`](crate::value::ByondValue) from any
/// Rust value that implements [`ToByond`](crate::to::ToByond).
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
pub fn byond_version() -> ByondVersion {
	byond().get_version()
}

/// Returns the version number the current .dmb was built with
pub fn dmb_version() -> sys::u4c {
	unsafe { byond().Byond_GetDMBVersion() }
}

#[doc(hidden)]
pub fn setup_once() {
	static SETUP: Once = Once::new();

	SETUP.call_once(|| {
		let _ = sync::is_main_thread(); // initialize main thread OnceCell
		std::panic::set_hook(Box::new(panic::panic_hook))
	});
}
