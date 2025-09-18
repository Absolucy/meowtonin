// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, byond};
use std::{borrow::Cow, mem::MaybeUninit};

impl ByondValue {
	pub fn new_num<Num>(num: Num) -> Self
	where
		Num: Into<f32>,
	{
		tracy::zone!("ByondValue::new_num");
		unsafe {
			let mut value = MaybeUninit::uninit();
			byond().ByondValue_SetNum(value.as_mut_ptr(), num.into());
			Self(value.assume_init()) // numbers are not refcounted
		}
	}

	pub fn get_number(&self) -> ByondResult<f32> {
		tracy::zone!("ByondValue::get_number");
		if self.is_number() {
			Ok(unsafe { byond().ByondValue_GetNum(&self.0) })
		} else {
			Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("number"),
				got: self.get_type().name(),
			})
		}
	}
}
