// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, ToByond};
use std::borrow::Cow;

///////////////////////
// Unsigned integers //
///////////////////////

impl ToByond for u8 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (u8)");
		Ok(ByondValue::new_num(f32::from(*self)))
	}
}

impl ToByond for u16 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (u16)");
		Ok(ByondValue::new_num(f32::from(*self)))
	}
}

impl ToByond for u32 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		const MAX_VALUE: u32 = f32::MAX as u32;

		tracy::zone!("to_byond (u32)");
		let num = *self as f32;
		if num.fract() != 0.0 || !(0..=MAX_VALUE).contains(self) {
			Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit unsigned integer"),
				got: Cow::Borrowed("32-bit unsigned integer"),
			})
		} else {
			Ok(ByondValue::new_num(num))
		}
	}
}

impl ToByond for usize {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		const MAX_VALUE: usize = f32::MAX as usize;

		tracy::zone!("to_byond (usize)");
		let num = *self as f32;
		if num.fract() != 0.0 || !(0..=MAX_VALUE).contains(self) {
			Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit unsigned integer"),
				got: Cow::Borrowed("pointer-sized unsigned integer"),
			})
		} else {
			Ok(ByondValue::new_num(num))
		}
	}
}

////////////
// Floats //
////////////

impl ToByond for i8 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (i8)");
		Ok(ByondValue::new_num(f32::from(*self)))
	}
}

impl ToByond for i16 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (i16)");
		Ok(ByondValue::new_num(f32::from(*self)))
	}
}

impl ToByond for i32 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		const MIN_VALUE: i32 = f32::MIN as i32;
		const MAX_VALUE: i32 = f32::MAX as i32;

		tracy::zone!("to_byond (i32)");
		let num = *self as f32;
		if num.fract() != 0.0 || !(MIN_VALUE..=MAX_VALUE).contains(self) {
			Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit signed integer"),
				got: Cow::Borrowed("32-bit signed integer"),
			})
		} else {
			Ok(ByondValue::new_num(num))
		}
	}
}

impl ToByond for isize {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		const MIN_VALUE: isize = f32::MIN as isize;
		const MAX_VALUE: isize = f32::MAX as isize;

		tracy::zone!("to_byond (isize)");
		let num = *self as f32;
		if num.fract() != 0.0 || !(MIN_VALUE..=MAX_VALUE).contains(self) {
			Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit signed integer"),
				got: Cow::Borrowed("pointer-sized signed integer"),
			})
		} else {
			Ok(ByondValue::new_num(num))
		}
	}
}

//////////
// Misc //
//////////

impl ToByond for bool {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (bool)");
		u8::from(*self).to_byond()
	}
}

impl ToByond for f32 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		tracy::zone!("to_byond (f32)");
		Ok(ByondValue::new_num(*self))
	}
}
