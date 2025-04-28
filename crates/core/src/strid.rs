// SPDX-License-Identifier: 0BSD
use crate::{
	ByondValue, ByondValueType, byond,
	sys::{NONE, u4c},
};
use ahash::AHasher;
use nohash_hasher::BuildNoHashHasher;
use papaya::HashMap;
use std::{
	ffi::CString,
	hash::{Hash, Hasher},
	sync::LazyLock,
};

const DEFAULT_CACHE_CAPACITY: usize = 512;
pub(crate) static STRID_CACHE: LazyLock<HashMap<u64, u4c, BuildNoHashHasher<u64>>> =
	LazyLock::new(|| {
		HashMap::with_capacity_and_hasher(DEFAULT_CACHE_CAPACITY, BuildNoHashHasher::default())
	});

fn string_hash(string: impl AsRef<str>) -> u64 {
	let mut hasher = AHasher::default();
	string.as_ref().hash(&mut hasher);
	hasher.finish()
}

fn add_get_str_id(string: &str) -> u4c {
	let string = match CString::new(string) {
		Ok(string) => string,
		Err(_) => panic!("attempted to get id of invalid string"),
	};
	let id = unsafe { byond().Byond_AddGetStrId(string.as_ptr().cast()) };
	if id == NONE as u32 {
		panic!("attempted to get/create id of invalid string");
	}
	id
}

/// Looks up the ID of a given string, caching the result.
pub fn lookup_string_id(string: impl AsRef<str>) -> u4c {
	let string = string.as_ref();
	let hash = string_hash(string);
	*STRID_CACHE
		.pin()
		.get_or_insert_with(hash, || add_get_str_id(string))
}

/// Returns the bytes of the string with the given string ID, or `None` if the
/// string ID is invalid.
pub fn get_string_bytes_from_id(id: u4c) -> Option<Vec<u8>> {
	unsafe { ByondValue::new_ref_unchecked(ByondValueType::STRING, id) }
		.get_string_bytes()
		.ok()
}

/// Returns the string with the given string ID, or `None`` if the string ID is
/// invalid.
pub fn get_string_from_id(id: u4c) -> Option<String> {
	unsafe { ByondValue::new_ref_unchecked(ByondValueType::STRING, id) }
		.get_string()
		.ok()
}
