// SPDX-License-Identifier: 0BSD
pub mod container;
pub mod list;
pub mod num;
pub mod string;

use crate::{ByondResult, ByondValue};

/// A simple trait for trying to convert a [`ByondValue`] into a Rust type.
pub trait FromByond: Sized {
	/// Convert a [`ByondValue`] into a Rust type.
	fn from_byond(value: &ByondValue) -> ByondResult<Self>;
}

impl FromByond for ByondValue {
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		Ok(value.clone())
	}
}

impl FromByond for () {
	#[inline]
	fn from_byond(_value: &ByondValue) -> ByondResult<Self> {
		Ok(())
	}
}
