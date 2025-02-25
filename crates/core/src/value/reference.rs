// SPDX-License-Identifier: 0BSD
use crate::{
	sys::{
		ByondValue_DecRef, ByondValue_GetRef, ByondValue_IncRef, ByondValue_SetRef, Byond_Refcount,
		Byond_TestRef, CByondValue,
	},
	ByondResult, ByondValue, ByondValueType,
};
use std::{
	mem::MaybeUninit,
	ops::{Deref, DerefMut},
};

impl ByondValue {
	/// Creates a new reference with the given value type and reference ID.
	/// This will return `None` if the provided reference is invalid.
	pub fn new_ref(value_type: ByondValueType, ref_id: u32) -> Option<Self> {
		unsafe { Self::new_ref_unchecked(value_type, ref_id) }.test_ref()
	}

	/// Creates a new reference with the given value type and reference ID.
	/// This is unsafe because it does not check if the provided reference is
	/// valid, you should normally use [Self::new_ref] instead.
	pub unsafe fn new_ref_unchecked(value_type: ByondValueType, ref_id: u32) -> Self {
		unsafe {
			let mut value = MaybeUninit::uninit();
			ByondValue_SetRef(value.as_mut_ptr(), value_type.0, ref_id);
			Self(value.assume_init())
		}
	}

	/// Returns the reference count of the value.
	pub fn ref_count(&self) -> ByondResult<usize> {
		let mut result = 0;
		map_byond_error!(Byond_Refcount(&self.0, &mut result))?;
		Ok(result)
	}

	/// Gets the reference ID of the value, provided it is a reference.
	/// This can later be used with [Self::new_ref] alongside the value type to
	/// get the value back.
	pub fn ref_id(&self) -> Option<u32> {
		let result = unsafe { ByondValue_GetRef(&self.0) };
		if result == 0 {
			None
		} else {
			Some(result)
		}
	}

	/// Increments the reference count of the value.
	pub fn inc_ref(&self) {
		unsafe { ByondValue_IncRef(&self.0) };
	}

	/// Increments this value's ref count and returns it as an [RcByondValue],
	/// which will decrement the ref count when dropped.
	pub fn referenced(self) -> RcByondValue {
		self.inc_ref();
		RcByondValue(self)
	}

	/// De-increments the reference count of the value.
	pub fn dec_ref(&self) {
		unsafe { ByondValue_DecRef(&self.0) };
	}

	/// Tests if the given value is a valid reference.
	/// This will return `None` if the value is not a valid reference,
	/// or give back the original input if it is.
	pub fn test_ref(mut self) -> Option<Self> {
		if unsafe { Byond_TestRef(&mut self.0) } {
			Some(self)
		} else {
			None
		}
	}
}

/// A [ByondValue] that increments its ref upon creation,
/// and decrements the ref when dropped.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RcByondValue(ByondValue);

impl Deref for RcByondValue {
	type Target = ByondValue;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for RcByondValue {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl AsRef<ByondValue> for RcByondValue {
	fn as_ref(&self) -> &ByondValue {
		&self.0
	}
}

impl AsMut<ByondValue> for RcByondValue {
	fn as_mut(&mut self) -> &mut ByondValue {
		&mut self.0
	}
}

impl PartialEq<ByondValue> for RcByondValue {
	fn eq(&self, other: &ByondValue) -> bool {
		self.0.eq(other)
	}
}

impl PartialEq<RcByondValue> for ByondValue {
	fn eq(&self, other: &RcByondValue) -> bool {
		self.eq(&other.0)
	}
}

impl From<ByondValue> for RcByondValue {
	fn from(value: ByondValue) -> Self {
		value.referenced()
	}
}

impl From<CByondValue> for RcByondValue {
	fn from(value: CByondValue) -> Self {
		ByondValue::from(value).referenced()
	}
}

impl Drop for RcByondValue {
	fn drop(&mut self) {
		self.0.dec_ref();
	}
}

impl Clone for RcByondValue {
	fn clone(&self) -> Self {
		self.0.clone().referenced()
	}
}
