// SPDX-License-Identifier: 0BSD
use crate::byond;
use std::{
	borrow::Cow,
	convert::Infallible,
	ffi::{CString, FromBytesUntilNulError, IntoStringError, NulError},
	str::Utf8Error,
};

pub type ByondResult<T> = std::result::Result<T, ByondError>;

#[derive(Debug, thiserror::Error)]
pub enum ByondError {
	/// This error is thrown when you try to convert a
	/// [`ByondValue`](crate::ByondValue) into a type which it does not
	/// represent, or the value failed to convert
	/// to a [`ByondValue`](crate::ByondValue).
	#[error("Cannot convert value to target type: expected {}, got {}", .expected, .got)]
	InvalidConversion {
		expected: Cow<'static, str>,
		got: Cow<'static, str>,
	},
	/// This error is thrown from call when you try to call something that isn't
	/// in BYOND's string tree (thus is not a valid proc)
	#[error("Attempted to call invalid proc")]
	InvalidProc,
	/// This error is thrown from call when you try to set a var name that isn't
	/// in BYOND's string tree (thus is not a valid variable)
	#[error("Attempted to read/write invalid variable")]
	InvalidVariable,
	/// Thrown when trying to get a [String] from a
	/// [`ByondValue`](crate::ByondValue).
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
	NotReferenceable,
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
		let last_error: ByondResult<CString> = unsafe {
			crate::misc::with_buffer::<_, u8, _, _>(
				None,
				|ptr, len| byond().Byond_LastError(ptr.cast(), len),
				|buffer| CString::from_vec_with_nul(buffer).unwrap_or_default(),
			)
		};
		last_error.ok().map(Self)
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
