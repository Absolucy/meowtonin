// SPDX-License-Identifier: 0BSD
use crate::{
	byond,
	sys::{u4c, NONE},
	ByondValue, ByondValueType,
};
use ahash::AHasher;
use nohash_hasher::{BuildNoHashHasher, IntMap};
use parking_lot::RwLock;
use std::{
	ffi::CString,
	hash::{Hash, Hasher},
	sync::LazyLock,
};

const DEFAULT_CACHE_CAPACITY: usize = 512;
pub(crate) static STRID_CACHE: LazyLock<RwLock<IntMap<u64, u4c>>> = LazyLock::new(|| {
	RwLock::new(IntMap::with_capacity_and_hasher(
		DEFAULT_CACHE_CAPACITY,
		BuildNoHashHasher::default(),
	))
});

fn string_hash(string: impl AsRef<str>) -> u64 {
	let mut hasher = AHasher::default();
	string.as_ref().hash(&mut hasher);
	hasher.finish()
}

/// Looks up the ID of a given string, caching the result.
pub fn lookup_string_id(string: impl AsRef<str>) -> u4c {
	let string = string.as_ref();
	let hash = string_hash(string);
	if let Some(id) = STRID_CACHE.read().get(&hash) {
		return *id;
	}
	let string = match CString::new(string) {
		Ok(string) => string,
		Err(_) => panic!("attempted to get id of invalid string"),
	};
	let id = unsafe { byond().Byond_AddGetStrId(string.as_ptr().cast()) };
	if id == NONE as u32 {
		panic!("attempted to get/create id of invalid string");
	}
	STRID_CACHE.write().insert(hash, id);
	id
}

/// Returns the bytes of the string with the given string ID.
/// Returns None if the string ID is invalid.
pub fn get_string_bytes_from_id(id: u4c) -> Option<Vec<u8>> {
	unsafe { ByondValue::new_ref_unchecked(ByondValueType::STRING, id) }
		.get_string_bytes()
		.ok()
}

/// Returns the string with the given string ID.
/// Returns None if the string ID is invalid.
pub fn get_string_from_id(id: u4c) -> Option<String> {
	unsafe { ByondValue::new_ref_unchecked(ByondValueType::STRING, id) }
		.get_string()
		.ok()
}
