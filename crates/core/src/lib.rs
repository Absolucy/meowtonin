// SPDX-License-Identifier: 0BSD
#![warn(
	clippy::correctness,
	clippy::suspicious,
	clippy::complexity,
	clippy::perf,
	clippy::style
)]
#![allow(unused_unsafe, clippy::missing_safety_doc)]

#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate log;

pub mod byond;
#[macro_use]
pub mod error;
pub mod from;
pub mod init;
pub mod misc;
pub mod panic;
pub mod proc;
pub mod strid;
pub mod sync;
pub mod to;
pub mod value;
pub mod xyz;

pub mod sys {
	pub use byondapi_sys::*;
}

pub use crate::{
	byond::byond,
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
	byond().get_version()
}

/// Returns the version number the current .dmb was built with
pub fn dmb_version() -> u32 {
	unsafe { byond().Byond_GetDMBVersion() }
}
