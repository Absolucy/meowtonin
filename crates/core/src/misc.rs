// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, ByondXYZ, byond, sys::u4c};
use rustc_hash::FxBuildHasher;
use std::mem::MaybeUninit;

pub fn block(corner_a: ByondXYZ, corner_b: ByondXYZ) -> ByondResult<Vec<ByondValue>> {
	unsafe {
		let initial_capacity = corner_a.total_block_size(&corner_b) as usize;
		with_buffer::<_, ByondValue, _, _>(
			Some(initial_capacity),
			|ptr, len| byond().Byond_Block(&corner_a.0, &corner_b.0, ptr.cast(), len),
			|buffer| buffer,
		)
	}
}

pub fn locate_xyz(location: ByondXYZ) -> ByondResult<ByondValue> {
	unsafe {
		let mut result = MaybeUninit::uninit();
		map_byond_error!(byond().Byond_LocateXYZ(&location.0, result.as_mut_ptr()))?;
		Ok(ByondValue(result.assume_init())) // turfs are not refcounted
	}
}

pub fn locate(
	typepath: ByondValue,
	list: impl Into<Option<ByondValue>>,
) -> ByondResult<ByondValue> {
	unsafe {
		let mut result = MaybeUninit::uninit();
		let list = list.into();
		let list_ptr = list
			.as_ref()
			.map_or(std::ptr::null(), |list| &list.0 as *const _);
		map_byond_error!(byond().Byond_LocateIn(&typepath.0, list_ptr, result.as_mut_ptr()))?;
		Ok(ByondValue(unsafe { result.assume_init() }))
	}
}

/// Returns if this is likely an associative list or not.
///
/// Do not rely on this being 100% accurate.
pub fn is_likely_assoc(list: &[[ByondValue; 2]]) -> bool {
	let mut found_keys =
		rustc_hash::FxHashSet::<&ByondValue>::with_capacity_and_hasher(list.len(), FxBuildHasher);
	for [key, value] in list {
		if !value.is_null() {
			return true;
		}
		if key.is_number() || found_keys.insert(key) {
			return false;
		}
	}
	false
}

pub(crate) unsafe fn with_buffer<T, B, F, W>(
	initial_capacity: Option<usize>,
	writer: W,
	transform: F,
) -> ByondResult<T>
where
	B: Default,
	F: FnOnce(Vec<B>) -> T,
	W: Fn(*mut std::ffi::c_void, &mut u4c) -> bool,
{
	let mut buffer: Vec<B> = Vec::new();
	buffer.resize_with(initial_capacity.unwrap_or(0), B::default);

	// maximum of 16 iterations - which should never happen, but well,
	// whenever a coder says "this should never happen", it almost
	// guarantees that somehow, said thing will happen.
	for _ in 0..16 {
		let mut len = buffer.len() as u4c;
		if writer(buffer.as_mut_ptr().cast(), &mut len) {
			buffer.truncate(len as usize);
			return Ok(transform(buffer));
		}
		// false with a length of 0 is a real error, not "too small"
		if len == 0 {
			return Err(ByondError::get_last_byond_error());
		}
		// drops refs a contents/vars walk wrote before running out
		buffer.clear();
		buffer.resize_with(len as usize, B::default);
	}
	Err(ByondError::get_last_byond_error())
}
