// SPDX-License-Identifier: 0BSD
#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
	clippy::missing_safety_doc
)]
mod version;

#[allow(
	dead_code,
	rustdoc::broken_intra_doc_links,
	clippy::suspicious_doc_comments
)]
pub mod byond_rawbind {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(doc)]
pub use byond_rawbind::ByondApi as RawByondApi;

// According to acrimon, in theory slightly faster...
#[cfg_attr(target_pointer_width = "32", repr(align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(align(128)))]
pub struct ByondApi {
	internal: byond_rawbind::ByondApi,
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
		let internal = byond_rawbind::ByondApi::from_library(lib)?;
		Ok(ByondApi { internal, version })
	}

	/// Get the version of the ByondApi library.
	#[must_use]
	pub fn get_version(&self) -> ByondVersion {
		self.version
	}
}

impl std::ops::Deref for ByondApi {
	type Target = byond_rawbind::ByondApi;

	fn deref(&self) -> &Self::Target {
		&self.internal
	}
}

// Stabilized types
pub use self::version::ByondVersion;
pub use byond_rawbind::{
	s1c, s2c, s4c, s8c, u1c, u2c, u4c, u4cOrPointer, u8c, ByondValueData, ByondValueType,
	CByondValue, CByondXYZ,
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
