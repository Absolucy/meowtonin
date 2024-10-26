// SPDX-License-Identifier: 0BSD
use crate::sys::Byond_LastError;
use std::{
	borrow::Cow,
	convert::Infallible,
	ffi::{CStr, CString, FromBytesUntilNulError, IntoStringError, NulError},
	str::Utf8Error,
};

pub type ByondResult<T> = std::result::Result<T, ByondError>;

#[derive(Debug, thiserror::Error)]
pub enum ByondError {
	/// This error is thrown when you try to convert a [`crate::ByondValue`]
	/// into a type which it does not represent, or the value failed to convert
	/// to a [`crate::ByondValue`].
	#[error("Cannot convert value to target type: expected {}, got {}", .expected, .got)]
	InvalidConversion {
		expected: Cow<'static, str>,
		got: Cow<'static, str>,
	},
	/// This error is thrown from call when you try to call something that isn't
	/// in BYOND's string tree (thus is not a valid proc)
	#[error("Attempted to call invalid proc")]
	InvalidProc,
	/// Thrown when trying to get a [`String`] from a [`crate::ByondValue`].
	#[error("BYOND string was invalid UTF-8")]
	NonUtf8String,
	/// When the BYOND API doesn't tell us what the error is.
	#[error("Unknown internal BYOND error")]
	UnknownByondError,
	/// Internal BYOND API error
	#[error("Internal BYOND error: {:#?}", (.0).0)]
	ByondError(ByondApiError),
	/// Thrown by us when we know this type is not indexable because it's not a
	/// list
	#[error("Type is not a list")]
	NotAList,
	/// Thrown by us when we know this type does not have a refnumber
	#[error("Cannot get a ref from this value")]
	NotReferencable,
	#[error(transparent)]
	Boxed(Box<dyn std::error::Error + Send + 'static>),
}

impl ByondError {
	pub fn get_last_byond_error() -> Self {
		match ByondApiError::get_last() {
			Some(err) => Self::ByondError(err),
			None => Self::UnknownByondError,
		}
	}

	pub fn boxed<Err>(err: Err) -> Self
	where
		Err: std::error::Error + Send + 'static,
	{
		Self::Boxed(Box::new(err))
	}
}

impl From<Infallible> for ByondError {
	#[cold]
	fn from(_: Infallible) -> Self {
		unreachable!()
	}
}

impl From<NulError> for ByondError {
	fn from(_: NulError) -> Self {
		Self::NonUtf8String
	}
}

impl From<Utf8Error> for ByondError {
	fn from(_: Utf8Error) -> Self {
		Self::NonUtf8String
	}
}

impl From<FromBytesUntilNulError> for ByondError {
	fn from(_: FromBytesUntilNulError) -> Self {
		Self::NonUtf8String
	}
}

impl From<IntoStringError> for ByondError {
	fn from(_: IntoStringError) -> Self {
		Self::NonUtf8String
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByondApiError(pub CString);

impl ByondApiError {
	pub fn get_last() -> Option<Self> {
		// Safety: It's always safe to call Byond_LastError
		let ptr = unsafe { Byond_LastError() };
		if !ptr.is_null() {
			// Safety: We just have to trust that Byond gave us a valid cstring...
			let cstr = unsafe { CStr::from_ptr(ptr) };
			Some(ByondApiError(cstr.to_owned()))
		} else {
			None
		}
	}
}

macro_rules! map_byond_error {
	($x:expr) => {
		unsafe {
			let result = $x;
			if result {
				Ok(())
			} else {
				Err($crate::ByondError::get_last_byond_error())
			}
		}
	};
}
