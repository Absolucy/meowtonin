// SPDX-License-Identifier: 0BSD
use crate::{
	sys::{ByondValue_GetNum, ByondValue_SetNum},
	ByondError, ByondResult, ByondValue,
};
use std::{borrow::Cow, mem::MaybeUninit};

impl ByondValue {
	pub fn new_num<Num>(num: Num) -> Self
	where
		Num: Into<f32>,
	{
		unsafe {
			let mut value = MaybeUninit::uninit();
			ByondValue_SetNum(value.as_mut_ptr(), num.into());
			Self(value.assume_init())
		}
	}

	pub fn set_num<Num>(&mut self, num: Num)
	where
		Num: Into<f32>,
	{
		unsafe { ByondValue_SetNum(&mut self.0, num.into()) }
	}

	pub fn get_number(&self) -> ByondResult<f32> {
		if self.is_number() {
			Ok(unsafe { ByondValue_GetNum(&self.0) })
		} else {
			Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("number"),
				got: self.get_type().name(),
			})
		}
	}
}
