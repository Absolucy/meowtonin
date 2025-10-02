// SPDX-License-Identifier: 0BSD
use crate::{ByondResult, ByondValue, FromByond};
use std::{
	borrow::Cow,
	cell::RefCell,
	num::{Saturating, Wrapping},
	rc::Rc,
	sync::Arc,
};

impl<Value> FromByond for Option<Value>
where
	Value: FromByond,
{
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		if value.is_null() {
			Ok(None)
		} else {
			Value::from_byond(value).map(Some)
		}
	}
}

impl<Value> FromByond for Box<Value>
where
	Value: FromByond,
{
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Box::new)
	}
}

impl<Value> FromByond for Rc<Value>
where
	Value: FromByond,
{
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Rc::new)
	}
}

impl<Value> FromByond for Arc<Value>
where
	Value: FromByond,
{
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Arc::new)
	}
}

impl<Value> FromByond for RefCell<Value>
where
	Value: FromByond,
{
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(RefCell::new)
	}
}

impl<Value> FromByond for Cow<'_, Value>
where
	Value: ToOwned,
	Value::Owned: FromByond,
{
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::Owned::from_byond(value).map(Cow::Owned)
	}
}

impl<Value> FromByond for Wrapping<Value>
where
	Value: FromByond,
{
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Wrapping)
	}
}

impl<Value> FromByond for Saturating<Value>
where
	Value: FromByond,
{
	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Saturating)
	}
}
