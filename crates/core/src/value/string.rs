// SPDX-License-Identifier: 0BSD
use crate::{ByondResult, ByondValue, byond};
use std::{ffi::CStr, mem::MaybeUninit};

impl ByondValue {
	pub fn new_string<Str>(string: Str) -> Self
	where
		Str: Into<Vec<u8>>,
	{
		tracy::zone!("ByondValue::new_string");
		let mut string = string.into();
		string.push(0);
		unsafe {
			let mut value = MaybeUninit::uninit();
			byond().ByondValue_SetStr(value.as_mut_ptr(), string.as_ptr().cast());
			Self(unsafe { value.assume_init() })
		}
	}

	pub fn get_string_bytes(&self) -> ByondResult<Vec<u8>> {
		tracy::zone!("ByondValue::get_string_bytes");
		unsafe {
			crate::misc::with_buffer::<_, u8, _, _>(
				None,
				|ptr, len| byond().Byond_ToString(&self.0, ptr.cast(), len),
				|buffer| buffer,
			)
		}
	}

	pub fn get_string(&self) -> ByondResult<String> {
		buffer_to_string(&self.get_string_bytes()?)
	}
}

fn buffer_to_string(buffer: &[u8]) -> ByondResult<String> {
	let cstr = CStr::from_bytes_until_nul(buffer)?;
	if cfg!(feature = "lossy-utf8") {
		Ok(cstr.to_string_lossy().into_owned())
	} else {
		Ok(cstr.to_str().map(str::to_owned)?)
	}
}
