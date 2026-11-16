// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, byond};
use std::mem::MaybeUninit;

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
		self.get_string_bytes().and_then(buffer_to_string)
	}
}

fn buffer_to_string(mut buffer: Vec<u8>) -> ByondResult<String> {
	let nul = buffer
		.iter()
		.position(|&byte| byte == 0)
		.ok_or(ByondError::NonUtf8String)?;
	buffer.truncate(nul);
	match String::from_utf8(buffer) {
		Ok(string) => Ok(string),
		Err(err) if cfg!(feature = "lossy-utf8") => {
			Ok(String::from_utf8_lossy(err.as_bytes()).into_owned())
		}
		Err(_) => Err(ByondError::NonUtf8String),
	}
}
