// SPDX-License-Identifier: 0BSD
use super::byond_rawbind::Byond_GetVersion;

pub unsafe fn get_byond_version() -> (u32, u32) {
	let mut version = 0;
	let mut build = 0;

	unsafe { Byond_GetVersion(&mut version, &mut build) };

	(version, build)
}
