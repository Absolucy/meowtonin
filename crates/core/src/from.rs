// SPDX-License-Identifier: 0BSD
pub mod container;
pub mod list;
pub mod num;
pub mod string;

use crate::{ByondResult, ByondValue};

/// A simple trait for trying to convert a [ByondValue] into a Rust type.
pub trait FromByond: Sized {
	/// `true` if [`from_byond_ref`](Self::from_byond_ref) never runs DM code,
	/// which lets plain `/list` reads skip referencing each item.
	const CONVERTS_WITHOUT_DM: bool = false;

	/// Convert a [ByondValue] into a Rust type.
	fn from_byond(value: ByondValue) -> ByondResult<Self>;

	/// Convert a borrowed [ByondValue] into a Rust type. Defaults to a clone.
	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		Self::from_byond(value.clone())
	}
}

impl FromByond for ByondValue {
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Ok(value)
	}
}

impl FromByond for () {
	fn from_byond(_value: ByondValue) -> ByondResult<Self> {
		Ok(())
	}
}
