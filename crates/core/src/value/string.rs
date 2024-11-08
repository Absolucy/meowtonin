// SPDX-License-Identifier: 0BSD
use crate::{
	sys::{ByondValue_SetStr, Byond_ToString},
	ByondResult, ByondValue,
};
use std::{ffi::CStr, mem::MaybeUninit};

impl ByondValue {
	pub fn new_string<Str>(string: Str) -> Self
	where
		Str: Into<Vec<u8>>,
	{
		let mut string = string.into();
		string.push(0);
		unsafe {
			let mut value = MaybeUninit::uninit();
			ByondValue_SetStr(value.as_mut_ptr(), string.as_ptr().cast());
			Self(value.assume_init())
		}
	}

	pub fn set_string<Str>(&mut self, string: Str)
	where
		Str: Into<Vec<u8>>,
	{
		let mut string = string.into();
		string.push(0);
		unsafe { ByondValue_SetStr(&mut self.0, string.as_ptr().cast()) }
	}

	pub fn get_string_bytes(&self) -> ByondResult<Vec<u8>> {
		unsafe {
			let mut buffer = Vec::<u8>::new();
			let mut needed_len = 0;
			if Byond_ToString(&self.0, buffer.as_mut_ptr().cast(), &mut needed_len) {
				// Safety: if this returns true, then the buffer was large enough, and thus
				// needed_len <= capacity.
				buffer.set_len(needed_len);
				return Ok(buffer);
			}
			buffer.reserve(needed_len.saturating_sub(buffer.len()));
			map_byond_error!(Byond_ToString(
				&self.0,
				buffer.as_mut_ptr().cast(),
				&mut needed_len
			))?;
			// Safety: needed_len is always <= capacity here,
			// unless BYOND did a really bad fucky wucky.
			buffer.set_len(needed_len);
			Ok(buffer)
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
