// SPDX-License-Identifier: 0BSD
use smol_str::SmolStr;
use std::{ffi::c_void, path::PathBuf};
use windows::Win32::{
	Foundation::{HMODULE, MAX_PATH},
	System::LibraryLoader::GetModuleFileNameW,
};

pub(super) fn resolve_module_name(base_address: *mut c_void) -> Option<SmolStr> {
	let mut buffer = [0_u16; MAX_PATH as usize];
	let length = unsafe { GetModuleFileNameW(HMODULE(base_address), &mut buffer) };
	if length == 0 {
		return None;
	}
	String::from_utf16(&buffer[..length as usize])
		.ok()
		.map(PathBuf::from)
		.and_then(|module_path| {
			module_path
				.file_name()
				.map(|name| SmolStr::from(name.to_string_lossy()))
		})
}
