// SPDX-License-Identifier: 0BSD
use crate::{ByondResult, ByondValue, ToByond};
use std::{
	num::{Saturating, Wrapping},
	rc::Rc,
	sync::Arc,
};

impl<Value> ToByond for Option<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		match self {
			Some(value) => value.to_byond(),
			None => Ok(ByondValue::default()),
		}
	}
}

impl<Value> ToByond for Box<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		(**self).to_byond()
	}
}

impl<Value> ToByond for Rc<Value>
where
	Value: ToByond + Clone,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		(**self).clone().to_byond()
	}
}

impl<Value> ToByond for Arc<Value>
where
	Value: ToByond + Clone,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		(**self).clone().to_byond()
	}
}

impl<Value> ToByond for Wrapping<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.0.to_byond()
	}
}

impl<Value> ToByond for Saturating<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		self.0.to_byond()
	}
}
