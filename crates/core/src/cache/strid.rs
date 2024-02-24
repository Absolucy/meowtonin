// SPDX-License-Identifier: 0BSD
use crate::{byond, sys::u4c};
use ahash::AHasher;
use nohash_hasher::{BuildNoHashHasher, IntMap};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::{
	ffi::CString,
	hash::{Hash, Hasher},
};

const DEFAULT_CACHE_CAPACITY: usize = 512;
pub(crate) static STRID_CACHE: Lazy<RwLock<IntMap<u64, u4c>>> = Lazy::new(|| {
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
	if id == u4c::MAX {
		panic!("attempted to get/create id of invalid string");
	}
	STRID_CACHE.write().insert(hash, id);
	id
}
