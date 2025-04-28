// SPDX-License-Identifier: 0BSD
#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
	clippy::missing_safety_doc
)]
#[allow(
	dead_code,
	rustdoc::broken_intra_doc_links,
	clippy::suspicious_doc_comments
)]
#[rustfmt::skip]
pub mod bindings;
mod version;

#[cfg(doc)]
pub use bindings::ByondApi as RawByondApi;

// According to acrimon, in theory slightly faster...
#[cfg_attr(target_pointer_width = "32", repr(align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(align(128)))]
pub struct ByondApi {
	internal: bindings::ByondApi,
	version: ByondVersion,
}

unsafe impl Sync for ByondApi {}
unsafe impl Send for ByondApi {}

impl ByondApi {
	/// Initialize [ByondApi], using the given library.
	pub unsafe fn init_from_library<Lib>(library: Lib) -> Result<ByondApi, libloading::Error>
	where
		Lib: Into<libloading::Library>,
	{
		let lib = library.into();
		let version = version::get_byond_version(&lib);
		let internal = bindings::ByondApi::from_library(lib)?;
		Ok(ByondApi { internal, version })
	}

	/// Get the version of the ByondApi library.
	#[must_use]
	pub fn get_version(&self) -> ByondVersion {
		self.version
	}
}

impl std::ops::Deref for ByondApi {
	type Target = bindings::ByondApi;

	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

pub use crate::version::ByondVersion;
// Stabilized types
pub use crate::bindings::{
	s1c, s2c, s4c, s8c, u1c, u2c, u4c, u4cOrPointer, u8c, ByondValueData, ByondValueType,
	CByondPixLoc, CByondValue, CByondXYZ,
};

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

#[cfg(test)]
mod tests {
	#[test]
	fn ensure_typedefs_match() {
		use crate::bindings::{s1c, s2c, s4c, s8c, u1c, u2c, u4c, u8c};
		use std::mem::size_of;

		assert_eq!(size_of::<u1c>(), size_of::<u8>(), "u1c != u8");
		assert_eq!(size_of::<s1c>(), size_of::<i8>(), "s1c != i8");

		assert_eq!(size_of::<u2c>(), size_of::<u16>(), "u2c != u16");
		assert_eq!(size_of::<s2c>(), size_of::<i16>(), "s2c != i16");

		assert_eq!(size_of::<u4c>(), size_of::<u32>(), "u4c != u32");
		assert_eq!(size_of::<s4c>(), size_of::<i32>(), "s4c != i32");

		assert_eq!(size_of::<u8c>(), size_of::<u64>(), "u8c != u64");
		assert_eq!(size_of::<s8c>(), size_of::<i64>(), "s8c != i64");
	}
}
