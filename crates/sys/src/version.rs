// SPDX-License-Identifier: 0BSD
use super::bindings::u4c;
use std::cmp::Ordering;

type Byond_GetVersion = unsafe extern "C" fn(version: *mut u4c, build: *mut u4c);

pub(crate) unsafe fn get_byond_version(library: &libloading::Library) -> ByondVersion {
	let byond_get_version =
		unsafe { library.get::<Byond_GetVersion>(c"Byond_GetVersion".to_bytes()) }
			.expect("Failed to find Byond_GetVersion");

	let mut version = 0;
	let mut build = 0;

	unsafe { byond_get_version(&mut version, &mut build) };

	ByondVersion { version, build }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ByondVersion {
	pub version: u4c,
	pub build: u4c,
}

impl PartialOrd for ByondVersion {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for ByondVersion {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.version.cmp(&other.version) {
			Ordering::Equal => self.build.cmp(&other.build),
			other => other,
		}
	}
}
