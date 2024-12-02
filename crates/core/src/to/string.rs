// SPDX-License-Identifier: 0BSD
use crate::{ByondError, ByondResult, ByondValue, ToByond};
use std::{
	borrow::Cow,
	ffi::{CStr, CString, OsStr, OsString},
	path::{Path, PathBuf},
};

impl ToByond for &String {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_str().to_byond()
	}
}

impl ToByond for String {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_str().to_byond()
	}
}

impl ToByond for &CStr {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_string(self.to_bytes()))
	}
}

impl ToByond for CString {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_c_str().to_byond()
	}
}

impl ToByond for &str {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_string(*self))
	}
}

impl ToByond for Cow<'_, str> {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::new_string(self.as_ref()))
	}
}

impl ToByond for &Path {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.to_str()
			.map(ByondValue::new_string)
			.ok_or(ByondError::NonUtf8String)
	}
}

impl ToByond for PathBuf {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_path().to_byond()
	}
}

impl ToByond for OsString {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.as_os_str().to_byond()
	}
}

impl ToByond for &OsStr {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.to_str()
			.map(ByondValue::new_string)
			.ok_or(ByondError::NonUtf8String)
	}
}
