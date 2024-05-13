// SPDX-License-Identifier: 0BSD
use crate::{byond, ByondResult, ByondValue, ByondValueType};
use std::mem::MaybeUninit;

impl ByondValue {
	/// Creates a new reference with the given value type and reference ID.
	/// This will return `None` if the provided reference is invalid.
	#[inline]
	pub fn new_ref(value_type: ByondValueType, ref_id: u32) -> Option<Self> {
		unsafe { Self::new_ref_unchecked(value_type, ref_id) }.test_ref()
	}

	/// Creates a new reference with the given value type and reference ID.
	/// This is unsafe because it does not check if the provided reference is
	/// valid, you should normally use [Self::new_ref] instead.
	#[inline]
	pub unsafe fn new_ref_unchecked(value_type: ByondValueType, ref_id: u32) -> Self {
		unsafe {
			let mut value = MaybeUninit::uninit();
			byond().ByondValue_SetRef(value.as_mut_ptr(), value_type.0, ref_id);
			Self(value.assume_init())
		}
	}

	/// Returns the reference count of the value.
	#[inline]
	pub fn ref_count(&self) -> ByondResult<usize> {
		let mut result = 0;
		map_byond_error!(byond().Byond_Refcount(&self.0, &mut result))?;
		Ok(result)
	}

	/// Gets the reference ID of the value, provided it is a reference.
	/// This can later be used with [Self::new_ref] alongside the value type to
	/// get the value back.
	#[inline]
	pub fn ref_id(&self) -> Option<u32> {
		let result = unsafe { byond().ByondValue_GetRef(&self.0) };
		if result == 0 {
			None
		} else {
			Some(result)
		}
	}

	/// Increments the reference count of the value.
	#[inline]
	pub fn inc_ref(&self) {
		unsafe { byond().ByondValue_IncRef(&self.0) };
	}

	#[inline]
	pub fn persist(self) -> Self {
		self.inc_ref();
		self
	}

	/// De-increments the reference count of the value.
	#[inline]
	pub fn dec_ref(&self) {
		unsafe { byond().ByondValue_DecRef(&self.0) };
	}

	/// Tests if the given value is a valid reference.
	/// This will return `None` if the value is not a valid reference,
	/// or give back the original input if it is.
	#[inline]
	pub fn test_ref(mut self) -> Option<Self> {
		if unsafe { byond().Byond_TestRef(&mut self.0) } {
			Some(self)
		} else {
			None
		}
	}
}
