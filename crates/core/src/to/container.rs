// SPDX-License-Identifier: 0BSD
use crate::{ByondResult, ByondValue, ToByond};
use std::{
	borrow::Cow,
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

	fn to_byond_borrowed(&self) -> ByondResult<Cow<'_, ByondValue>> {
		match self {
			Some(value) => value.to_byond_borrowed(),
			None => Ok(Cow::Owned(ByondValue::default())),
		}
	}

	fn into_byond_cow<'a>(self) -> ByondResult<Cow<'a, ByondValue>>
	where
		Self: 'a,
	{
		match self {
			Some(value) => value.into_byond_cow(),
			None => Ok(Cow::Owned(ByondValue::default())),
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

	fn to_byond_borrowed(&self) -> ByondResult<Cow<'_, ByondValue>> {
		(**self).to_byond_borrowed()
	}

	fn into_byond_cow<'a>(self) -> ByondResult<Cow<'a, ByondValue>>
	where
		Self: 'a,
	{
		(*self).into_byond_cow()
	}
}

impl<Value> ToByond for Rc<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		(**self).to_byond()
	}

	fn to_byond_borrowed(&self) -> ByondResult<Cow<'_, ByondValue>> {
		(**self).to_byond_borrowed()
	}
}

impl<Value> ToByond for Arc<Value>
where
	Value: ToByond,
{
	fn to_byond(&self) -> ByondResult<ByondValue> {
		(**self).to_byond()
	}

	fn to_byond_borrowed(&self) -> ByondResult<Cow<'_, ByondValue>> {
		(**self).to_byond_borrowed()
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
