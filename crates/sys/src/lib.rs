// SPDX-License-Identifier: 0BSD
#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
	clippy::missing_safety_doc
)]
mod version;

#[allow(dead_code, rustdoc::broken_intra_doc_links)]
mod byond_rawbind {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[doc(inline)]
pub use byond_rawbind::*;
#[doc(inline)]
pub use version::get_byond_version;

pub const NONE: u2c = u2c::MAX;
pub const NOCH: u1c = u1c::MAX;

cfg_if::cfg_if! {
	if #[cfg(feature = "bytemuck")] {
		unsafe impl bytemuck::Zeroable for ByondValueData {}
		unsafe impl bytemuck::Pod for ByondValueData {}

		unsafe impl bytemuck::Zeroable for CByondValue {}
		unsafe impl bytemuck::Pod for CByondValue {}

		unsafe impl bytemuck::Zeroable for CByondXYZ {}
		unsafe impl bytemuck::Pod for CByondXYZ {}
	}
}
