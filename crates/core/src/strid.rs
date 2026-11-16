// SPDX-License-Identifier: 0BSD
use crate::{
	ByondValue, ByondValueType, byond,
	sys::{NONE, u4c},
};
use parking_lot::Mutex;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{ffi::CString, mem::ManuallyDrop, sync::LazyLock};

const DEFAULT_CACHE_CAPACITY: usize = 512;

type StrIdMap = FxHashMap<String, Option<u4c>>;
static STRID_CACHE: LazyLock<Mutex<StrIdMap>> = LazyLock::new(|| {
	Mutex::new(FxHashMap::with_capacity_and_hasher(
		DEFAULT_CACHE_CAPACITY,
		FxBuildHasher,
	))
});

fn get_str_id(string: &str) -> Option<u4c> {
	let string = match CString::new(string) {
		Ok(string) => string,
		Err(_) => return None,
	};
	let id = unsafe { byond().Byond_GetStrId(string.as_ptr().cast()) };
	if id == NONE as u32 { None } else { Some(id) }
}

/// Looks up the ID of a given string, caching the result.
pub fn lookup_string_id(string: impl AsRef<str>) -> Option<u4c> {
	let string = string.as_ref();
	let cached = STRID_CACHE.lock().get(string).copied();
	if let Some(id) = cached {
		return id;
	}
	// off the main thread this waits for the next tick, so the lock can't be
	// held here
	let id = get_str_id(string);
	STRID_CACHE.lock().insert(string.to_owned(), id);
	id
}

/// Returns the bytes of the string with the given string ID, or `None` if the
/// string ID is invalid.
pub fn get_string_bytes_from_id(id: u4c) -> Option<Vec<u8>> {
	borrowed_string(id).get_string_bytes().ok()
}

/// Returns the string with the given string ID, or `None`` if the string ID is
/// invalid.
pub fn get_string_from_id(id: u4c) -> Option<String> {
	borrowed_string(id).get_string().ok()
}

fn borrowed_string(id: u4c) -> ManuallyDrop<ByondValue> {
	ManuallyDrop::new(unsafe { ByondValue::new_ref_unchecked(ByondValueType::String, id) })
}
