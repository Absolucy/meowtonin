// SPDX-License-Identifier: 0BSD
use crate::{byond, ByondResult, ByondValue, ByondXYZ};
use std::{cell::RefCell, mem::MaybeUninit};

const DEFAULT_BUFFER_CAPACITY: usize = 256;

pub fn block(corner_a: ByondXYZ, corner_b: ByondXYZ) -> ByondResult<Vec<ByondValue>> {
	thread_local! {
		static BLOCK_BUFFER: RefCell<Vec<ByondValue>> = RefCell::new(Vec::with_capacity(DEFAULT_BUFFER_CAPACITY));
	}
	BLOCK_BUFFER.with_borrow_mut(|buffer| unsafe {
		let mut needed_len = buffer.capacity();
		if byond().Byond_Block(
			&corner_a.0,
			&corner_b.0,
			buffer.as_mut_ptr().cast(),
			&mut needed_len,
		) {
			// Safety: if this returns true, then the buffer was large enough, and thus
			// needed_len <= capacity.
			buffer.set_len(needed_len);
			return Ok(std::mem::take(buffer));
		}

		buffer.reserve(needed_len.saturating_sub(buffer.len()));
		map_byond_error!(byond().Byond_Block(
			&corner_a.0,
			&corner_b.0,
			buffer.as_mut_ptr().cast(),
			&mut needed_len
		))?;
		// Safety: needed_len is always <= capacity here,
		// unless BYOND did a really bad fucky wucky.
		buffer.set_len(needed_len);
		Ok(std::mem::take(buffer))
	})
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
