// SPDX-License-Identifier: 0BSD
use super::byond_rawbind::u4c;

pub unsafe fn get_byond_version(library: &libloading::Library) -> (u32, u32) {
	let byond_get_version: unsafe extern "C" fn(version: *mut u4c, build: *mut u4c) = library
		.get(b"Byond_GetVersion\0")
		.map(|sym| *sym)
		.expect("Failed to find Byond_GetVersion");

	let mut version = 0;
	let mut build = 0;

	byond_get_version(&mut version, &mut build);

	(version, build)
}
