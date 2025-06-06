// SPDX-License-Identifier: 0BSD
use crate::{
	ByondResult, ByondValue, ByondValueType, byond,
	sys::{ByondValueData, CByondValue},
};
use std::mem::MaybeUninit;

impl ByondValue {
	/// Creates a new reference with the given value type and reference ID.
	/// This will return `None` if the provided reference is invalid.
	pub fn new_ref(value_type: ByondValueType, ref_id: u32) -> Option<Self> {
		unsafe { Self::new_ref_unchecked(value_type, ref_id) }.test_ref()
	}

	/// Creates a new reference with the given value type and reference ID.
	///
	/// This is unsafe because it does not check if the provided reference is
	/// valid, you should normally use [`new_ref()`](Self::new_ref) instead.
	pub const unsafe fn new_ref_unchecked(value_type: ByondValueType, ref_id: u32) -> Self {
		Self(CByondValue {
			type_: value_type.0,
			junk1: 0,
			junk2: 0,
			junk3: 0,
			data: ByondValueData { ref_: ref_id },
		})
	}

	pub(crate) fn initialize_refcounted(value: MaybeUninit<CByondValue>) -> Self {
		let value = Self(unsafe { value.assume_init() });
		value.setup_ref_counting();
		value
	}

	/// Returns the reference count of the value.
	pub fn ref_count(&self) -> ByondResult<usize> {
		let mut result = 0;
		map_byond_error!(byond().Byond_Refcount(&self.0, &mut result))?;
		Ok(result as usize)
	}

	/// Gets the reference ID of the value, provided it is a reference.
	///
	/// This can later be used with [`new_ref()`](Self::new_ref) alongside the
	/// value type to get the value back.
	pub fn ref_id(&self) -> Option<u32> {
		let result = unsafe { byond().ByondValue_GetRef(&self.0) };
		if result == 0 { None } else { Some(result) }
	}

	/// Increments the reference count of the value.
	///
	/// This function is marked as unsafe because in most cases, you should not
	/// be manually handling refcounting.
	pub unsafe fn inc_ref(&self) {
		unsafe { byond().ByondValue_IncRef(&self.0) }
	}

	/// De-increments the reference count of the value.
	///
	/// This function is marked as unsafe because in most cases, you should not
	/// be manually handling refcounting.
	pub unsafe fn dec_ref(&self) {
		unsafe { byond().ByondValue_DecRef(&self.0) }
	}

	/// Marks a temporary reference as no longer in use.
	///
	/// Temporary references are automatically created for values created on the
	/// main thread (and not from within [crate::sync::thread_sync]), which
	/// expire at the end of the tick.
	///
	/// This function is marked as unsafe because in most cases, you should not
	/// be manually handling refcounting.
	pub unsafe fn dec_temp_ref(&self) {
		unsafe { byond().ByondValue_DecTempRef(&self.0) }
	}

	/// Tests if the given value is a valid reference.
	///
	/// This will return `None` if the value is not a valid reference,
	/// or give back the original input if it is.
	pub fn test_ref(mut self) -> Option<Self> {
		if unsafe { byond().Byond_TestRef(&mut self.0) } {
			Some(self)
		} else {
			None
		}
	}

	#[doc(hidden)]
	pub fn setup_ref_counting(&self) {
		if self.get_type().should_ref_count() && crate::sync::should_setup_ref_counting() {
			unsafe { self.inc_ref() };
		}
	}
}
