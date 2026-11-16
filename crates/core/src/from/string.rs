// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, ByondValueType, FromByond};
use std::{
	borrow::Cow,
	ffi::{CStr, CString},
	path::PathBuf,
};

fn not_a_string(got: ByondValueType) -> ByondError {
	ByondError::InvalidConversion {
		expected: Cow::Borrowed("string"),
		got: got.name(),
	}
}

impl FromByond for CString {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		match value.get_type() {
			ByondValueType::String => {
				Ok(CStr::from_bytes_until_nul(value.get_string_bytes()?.as_slice())?.to_owned())
			}
			ByondValueType::Null => Ok(CString::default()),
			other => Err(not_a_string(other)),
		}
	}
}

impl FromByond for String {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		match value.get_type() {
			ByondValueType::String => value.get_string(),
			ByondValueType::Null => Ok(String::new()),
			other => Err(not_a_string(other)),
		}
	}
}

impl FromByond for PathBuf {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		String::from_byond(value).map(PathBuf::from)
	}
}
