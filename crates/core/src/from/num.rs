// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, FromByond};
use std::borrow::Cow;

///////////////////////
// Unsigned integers //
///////////////////////

impl FromByond for u8 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MAX_VALUE: f32 = u8::MAX as f32;
		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(0.0..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("8-bit unsigned integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for u16 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MAX_VALUE: f32 = u16::MAX as f32;
		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(0.0..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("16-bit unsigned integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for u32 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MAX_VALUE: f32 = u32::MAX as f32;
		let num = value.get_number()?.round();
		if !(0.0..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("32-bit unsigned integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for u64 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MAX_VALUE: f32 = u32::MAX as f32;
		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(0.0..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("32-bit unsigned integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for u128 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MAX_VALUE: f32 = u32::MAX as f32;
		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(0.0..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("32-bit unsigned integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for usize {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MAX_VALUE: f32 = u32::MAX as f32;

		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(0.0..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("pointer-sized unsigned integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

/////////////////////
// Signed integers //
/////////////////////

impl FromByond for i8 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MIN_VALUE: f32 = i8::MIN as f32;
		const MAX_VALUE: f32 = i8::MAX as f32;

		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(MIN_VALUE..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("8-bit signed integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for i16 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MIN_VALUE: f32 = i16::MIN as f32;
		const MAX_VALUE: f32 = i16::MAX as f32;

		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(MIN_VALUE..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("16-bit signed integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for i32 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MIN_VALUE: f32 = i32::MIN as f32;
		const MAX_VALUE: f32 = i32::MAX as f32;

		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(MIN_VALUE..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("32-bit signed integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for i64 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MIN_VALUE: f32 = i32::MIN as f32;
		const MAX_VALUE: f32 = i32::MAX as f32;

		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(MIN_VALUE..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("32-bit signed integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for i128 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MIN_VALUE: f32 = i32::MIN as f32;
		const MAX_VALUE: f32 = i32::MAX as f32;

		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(MIN_VALUE..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("32-bit signed integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

impl FromByond for isize {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		const MIN_VALUE: f32 = i32::MIN as f32;
		const MAX_VALUE: f32 = i32::MAX as f32;

		let num = value.get_number()?.round();
		if num.fract() != 0.0 || !(MIN_VALUE..=MAX_VALUE).contains(&num) {
			return Err(ByondError::InvalidConversion {
				expected: Cow::Borrowed("pointer-sized signed integer"),
				got: Cow::Borrowed("float"),
			});
		}
		Ok(num as Self)
	}
}

////////////
// Floats //
////////////

impl FromByond for f32 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		value.get_number()
	}
}

impl FromByond for f64 {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		value.get_number().map(f64::from)
	}
}

//////////
// Bool //
//////////

impl FromByond for bool {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Ok(value.is_true())
	}
}
