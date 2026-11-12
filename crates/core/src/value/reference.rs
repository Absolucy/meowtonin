// SPDX-License-Identifier: 0BSD
use crate::{
	ByondResult, ByondValue, ByondValueType, byond,
	sys::{ByondValueData, CByondValue},
};

impl ByondValue {
	/// Creates a new reference with the given value type and reference ID.
	/// This will return `None` if the provided reference is invalid.
	pub fn new_ref(value_type: ByondValueType, ref_id: u32) -> Option<Self> {
		crate::sync::on_main_thread(|| {
			let mut raw = unsafe { Self::new_ref_unchecked(value_type, ref_id) }.detach();
			if !unsafe { byond().Byond_TestRef(&mut raw) } {
				return None;
			}
			let value = Self(raw);
			if value.get_type().is_ref_counted() {
				value.inc_ref();
			}
			Some(value)
		})
	}

	/// Creates a new reference with the given value type and reference ID.
	///
	/// # Safety
	/// Doesn't validate or take a reference, but dropping the result releases
	/// one. Only wrap a reference you already own.
	pub const unsafe fn new_ref_unchecked(value_type: ByondValueType, ref_id: u32) -> Self {
		Self(CByondValue {
			type_: value_type.0,
			junk1: 0,
			junk2: 0,
			junk3: 0,
			data: ByondValueData { ref_: ref_id },
		})
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
	pub fn inc_ref(&self) {
		unsafe { byond().ByondValue_IncRef(&self.0) };
	}

	/// De-increments the reference count of the value.
	pub fn dec_ref(&self) {
		unsafe { byond().ByondValue_DecRef(&self.0) };
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
}
