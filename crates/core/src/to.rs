// SPDX-License-Identifier: 0BSD
pub mod container;
pub mod list;
pub mod num;
pub mod string;

use crate::{ByondResult, ByondValue};

pub trait ToByond {
	fn to_byond(&self) -> ByondResult<ByondValue>;
}

impl ToByond for ByondValue {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(self.clone())
	}
}

impl ToByond for &ByondValue {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok((*self).clone())
	}
}

impl ToByond for () {
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::default())
	}
}
