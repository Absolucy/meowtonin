// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, FromByond};
use std::{ffi::CString, path::PathBuf};

impl FromByond for CString {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		value
			.get_string()
			.and_then(|s| CString::new(s).map_err(|_| ByondError::NonUtf8String))
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
