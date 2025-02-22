// SPDX-License-Identifier: 0BSD
use crate::{byond, ByondError, ByondResult, ByondValue, ByondXYZ};
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
		Ok(ByondValue(result.assume_init()))
	}
}

pub fn locate(
	typepath: ByondValue,
	list: impl Into<Option<ByondValue>>,
) -> ByondResult<ByondValue> {
	unsafe {
		let mut result = MaybeUninit::uninit();
		let list = list.into();
		let list = list
			.map(|list| &list.0 as *const _)
			.unwrap_or_else(std::ptr::null);
		map_byond_error!(byond().Byond_LocateIn(&typepath.0, list, result.as_mut_ptr()))?;
		Ok(ByondValue(result.assume_init()))
	}
}

/// Returns if this is likely an associative list or not.
/// Do not rely on this being 100% accurate.
pub fn is_likely_assoc(list: &[[ByondValue; 2]]) -> bool {
	let mut found_keys = ahash::AHashSet::<&ByondValue>::with_capacity(list.len());
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
	W: Fn(*mut std::ffi::c_void, &mut usize) -> bool,
{
	let mut buffer: Vec<B> = match initial_capacity {
		Some(cap) => Vec::with_capacity(cap),
		None => Vec::new(),
	};
	let mut needed_len = buffer.capacity();

	if writer(buffer.as_mut_ptr().cast(), &mut needed_len) {
		// Safety: if this returns true, then the buffer was large enough, and thus
		// needed_len <= capacity.
		buffer.set_len(needed_len);
		return Ok(transform(buffer));
	}

	// Reallocate and try again
	buffer.reserve(needed_len.saturating_sub(buffer.len()));
	if !writer(buffer.as_mut_ptr().cast(), &mut needed_len) {
		return Err(ByondError::get_last_byond_error());
	}

	// Safety: needed_len is always <= capacity here,
	// unless BYOND did a really bad fucky wucky.
	buffer.set_len(needed_len);
	Ok(transform(buffer))
}
