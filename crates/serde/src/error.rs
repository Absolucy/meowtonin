use std::borrow::Cow;

use meowtonin::ByondError;
use serde::{de, ser};

#[derive(Debug, thiserror::Error)]
pub enum SerializeError {
	#[error("byondapi error: {0}")]
	Byond(#[from] ByondError),
	#[error("expected {0}")]
	Expected(&'static str),
	#[error("{0}")]
	Custom(String),
}

impl ser::Error for SerializeError {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		Self::Custom(msg.to_string())
	}
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializeError {
	#[error("byondapi error: {0}")]
	Byond(#[from] ByondError),
	#[error("expected {0}, got {1}")]
	Unexpected(Cow<'static, str>, Cow<'static, str>),
	#[error("attempted to fetch element from end of array")]
	EndOfArray,
	#[error("{0}")]
	Custom(String),
}

impl de::Error for DeserializeError {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		Self::Custom(msg.to_string())
	}
}
