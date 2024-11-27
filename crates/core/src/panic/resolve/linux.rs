// SPDX-License-Identifier: 0BSD
use libc::{dladdr, Dl_info};
use smol_str::SmolStr;
use std::{
	ffi::{c_void, CStr},
	mem::MaybeUninit,
	path::PathBuf,
};

pub(super) fn resolve_module_name(base_address: *mut c_void) -> Option<SmolStr> {
	// Safety: Dl_info can be safely represented as all zeroes.
	let mut dl_info = unsafe { MaybeUninit::<Dl_info>::zeroed().assume_init() };
	if unsafe { dladdr(base_address as _, &mut dl_info) } == 0 {
		return None;
	}
	if dl_info.dli_fname.is_null() {
		return None;
	}
	unsafe { CStr::from_ptr(dl_info.dli_fname) }
		.to_str()
		.ok()
		.map(PathBuf::from)
		.and_then(|module_path| {
			module_path
				.file_name()
				.map(|name| SmolStr::from(name.to_string_lossy()))
		})
}
