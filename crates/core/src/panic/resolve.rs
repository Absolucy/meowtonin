// SPDX-License-Identifier: 0BSD
cfg_if::cfg_if! {
	if #[cfg(target_os = "windows")] {
		mod windows;
		use windows::resolve_module_name as resolve_module_name_inner;
	} else if #[cfg(target_os = "linux")] {
		mod linux;
		use linux::resolve_module_name as resolve_module_name_inner;
	}
}

use nohash_hasher::{BuildNoHashHasher, IntMap};
use parking_lot::Mutex;
use smol_str::SmolStr;
use std::{ffi::c_void, sync::LazyLock};

const DEFAULT_CACHE_CAPACITY: usize = 8;
static MODULE_CACHE: LazyLock<Mutex<IntMap<usize, Option<SmolStr>>>> = LazyLock::new(|| {
	Mutex::new(IntMap::with_capacity_and_hasher(
		DEFAULT_CACHE_CAPACITY,
		BuildNoHashHasher::default(),
	))
});

pub(crate) fn resolve_module_name(base_address: *mut c_void) -> Option<SmolStr> {
	if base_address.is_null() {
		return None;
	}
	MODULE_CACHE
		.lock()
		.entry(base_address as usize)
		.or_insert_with(|| resolve_module_name_inner(base_address))
		.clone()
}
