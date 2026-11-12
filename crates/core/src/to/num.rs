// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, ToByond};
use std::borrow::Cow;

// f32 has a 24-bit mantissa, so every integer up to 2^24 is exact and the
// next one up already rounds. anything past that would reach DM changed.

///////////////////////
// Unsigned integers //
///////////////////////

impl ToByond for u8 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_num(f32::from(*self)))
	}
}

impl ToByond for u16 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_num(f32::from(*self)))
	}
}

impl ToByond for u32 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		if *self > 1 << 24 {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit unsigned integer"),
				got: Cow::Borrowed("32-bit unsigned integer"),
			});
		}
		Ok(ByondValue::new_num(*self as f32))
	}
}

impl ToByond for u64 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		if *self > 1 << 24 {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit unsigned integer"),
				got: Cow::Borrowed("64-bit unsigned integer"),
			});
		}
		Ok(ByondValue::new_num(*self as f32))
	}
}

impl ToByond for usize {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		if *self > 1 << 24 {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit unsigned integer"),
				got: Cow::Borrowed("pointer-sized unsigned integer"),
			});
		}
		Ok(ByondValue::new_num(*self as f32))
	}
}

/////////////////////
// Signed integers //
/////////////////////

impl ToByond for i8 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_num(f32::from(*self)))
	}
}

impl ToByond for i16 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_num(f32::from(*self)))
	}
}

impl ToByond for i32 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		if self.unsigned_abs() > 1 << 24 {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit signed integer"),
				got: Cow::Borrowed("32-bit signed integer"),
			});
		}
		Ok(ByondValue::new_num(*self as f32))
	}
}

impl ToByond for i64 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		if self.unsigned_abs() > 1 << 24 {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit signed integer"),
				got: Cow::Borrowed("64-bit signed integer"),
			});
		}
		Ok(ByondValue::new_num(*self as f32))
	}
}

impl ToByond for isize {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		if self.unsigned_abs() > 1 << 24 {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("24-bit signed integer"),
				got: Cow::Borrowed("pointer-sized signed integer"),
			});
		}
		Ok(ByondValue::new_num(*self as f32))
	}
}

//////////
// Misc //
//////////

impl ToByond for bool {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		u8::from(*self).to_byond()
	}
}

impl ToByond for f32 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_num(*self))
	}
}

impl ToByond for f64 {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_num(*self as f32))
	}
}

#[cfg(test)]
mod tests {
	use crate::ToByond;

	// the check runs before any engine call, so these work outside dream daemon
	#[test]
	fn integers_past_2_pow_24_are_rejected() {
		assert!(16_777_217_u32.to_byond().is_err());
		assert!(16_777_217_u64.to_byond().is_err());
		assert!((-16_777_217_i32).to_byond().is_err());
		assert!((-5_000_000_000_i64).to_byond().is_err());
	}
}
