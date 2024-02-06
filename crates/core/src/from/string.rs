// SPDX-License-Identifier: 0BSD
use crate::{ByondResult, ByondValue, FromByond};
use std::{
	ffi::{CStr, CString},
	path::PathBuf,
};

impl FromByond for CString {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		Ok(CStr::from_bytes_until_nul(value.get_string_bytes()?.as_slice())?.to_owned())
	}
}

impl FromByond for String {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value.get_string()
	}
}

impl FromByond for PathBuf {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		String::from_byond(value).map(PathBuf::from)
	}
}
