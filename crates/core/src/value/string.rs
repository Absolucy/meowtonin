// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, byond, sys::u4c};
use std::{mem::MaybeUninit, slice};

impl ByondValue {
	pub fn new_string<Str>(string: Str) -> Self
	where
		Str: Into<Vec<u8>>,
	{
		let mut string = string.into();
		string.push(0);
		unsafe {
			let mut value = MaybeUninit::zeroed();
			byond().ByondValue_SetStr(value.as_mut_ptr(), string.as_ptr().cast());
			Self(unsafe { value.assume_init() })
		}
	}

	pub fn get_string_bytes(&self) -> ByondResult<Vec<u8>> {
		unsafe {
			crate::misc::with_buffer::<_, u8, _, _>(
				Some(256),
				|ptr, len| byond().Byond_ToString(&self.0, ptr.cast(), len),
				|buffer| buffer,
			)
		}
	}

	pub fn get_string(&self) -> ByondResult<String> {
		let mut stack = [MaybeUninit::<u8>::uninit(); 256];
		let mut len = stack.len() as u4c;
		if unsafe { byond().Byond_ToString(&self.0, stack.as_mut_ptr().cast(), &mut len) } {
			let written = unsafe { slice::from_raw_parts(stack.as_ptr().cast(), len as usize) };
			return bytes_to_string(written);
		}
		if len == 0 {
			return Err(ByondError::get_last_byond_error());
		}
		let buffer = unsafe {
			crate::misc::with_buffer::<_, u8, _, _>(
				Some(len as usize),
				|ptr, len| byond().Byond_ToString(&self.0, ptr.cast(), len),
				|buffer| buffer,
			)
		}?;
		bytes_to_string(&buffer)
	}
}

fn bytes_to_string(bytes: &[u8]) -> ByondResult<String> {
	let nul = bytes
		.iter()
		.position(|&byte| byte == 0)
		.ok_or(ByondError::NonUtf8String)?;
	let bytes = &bytes[..nul];
	match std::str::from_utf8(bytes) {
		Ok(string) => Ok(string.to_owned()),
		Err(_) if cfg!(feature = "lossy-utf8") => Ok(String::from_utf8_lossy(bytes).into_owned()),
		Err(_) => Err(ByondError::NonUtf8String),
	}
}
