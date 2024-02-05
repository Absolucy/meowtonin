// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, ToByond};
use std::{
	borrow::Cow,
	ffi::{CStr, CString, OsStr, OsString},
	path::{Path, PathBuf},
};

impl ToByond for &String {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_str().to_byond()
	}
}

impl ToByond for String {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_str().to_byond()
	}
}

impl ToByond for &CStr {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_string(self.to_bytes()))
	}
}

impl ToByond for CString {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_c_str().to_byond()
	}
}

impl ToByond for &str {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_string(*self))
	}
}

impl<'a> ToByond for Cow<'a, str> {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_string(self.as_ref()))
	}
}

impl ToByond for &Path {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.to_str()
			.map(ByondValue::new_string)
			.ok_or(ByondError::NonUtf8String)
	}
}

impl ToByond for PathBuf {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_path().to_byond()
	}
}

impl ToByond for OsString {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_os_str().to_byond()
	}
}

impl ToByond for &OsStr {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.to_str()
			.map(ByondValue::new_string)
			.ok_or(ByondError::NonUtf8String)
	}
}
