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
	const CONVERTS_WITHOUT_DM: bool = Value::CONVERTS_WITHOUT_DM;

	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		if value.is_null() {
			Ok(None)
		} else {
			Value::from_byond(value).map(Some)
		}
	}

	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		if value.is_null() {
			Ok(None)
		} else {
			Value::from_byond_ref(value).map(Some)
		}
	}
}

impl<Value> FromByond for Box<Value>
where
	Value: FromByond,
{
	const CONVERTS_WITHOUT_DM: bool = Value::CONVERTS_WITHOUT_DM;

	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Box::new)
	}

	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		Value::from_byond_ref(value).map(Box::new)
	}
}

impl<Value> FromByond for Rc<Value>
where
	Value: FromByond,
{
	const CONVERTS_WITHOUT_DM: bool = Value::CONVERTS_WITHOUT_DM;

	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Rc::new)
	}

	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		Value::from_byond_ref(value).map(Rc::new)
	}
}

impl<Value> FromByond for Arc<Value>
where
	Value: FromByond,
{
	const CONVERTS_WITHOUT_DM: bool = Value::CONVERTS_WITHOUT_DM;

	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Arc::new)
	}

	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		Value::from_byond_ref(value).map(Arc::new)
	}
}

impl<Value> FromByond for RefCell<Value>
where
	Value: FromByond,
{
	const CONVERTS_WITHOUT_DM: bool = Value::CONVERTS_WITHOUT_DM;

	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(RefCell::new)
	}

	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		Value::from_byond_ref(value).map(RefCell::new)
	}
}

impl<Value> FromByond for Cow<'_, Value>
where
	Value: ToOwned,
	Value::Owned: FromByond,
{
	const CONVERTS_WITHOUT_DM: bool = Value::Owned::CONVERTS_WITHOUT_DM;

	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::Owned::from_byond(value).map(Cow::Owned)
	}

	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		Value::Owned::from_byond_ref(value).map(Cow::Owned)
	}
}

impl<Value> FromByond for Wrapping<Value>
where
	Value: FromByond,
{
	const CONVERTS_WITHOUT_DM: bool = Value::CONVERTS_WITHOUT_DM;

	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Wrapping)
	}

	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		Value::from_byond_ref(value).map(Wrapping)
	}
}

impl<Value> FromByond for Saturating<Value>
where
	Value: FromByond,
{
	const CONVERTS_WITHOUT_DM: bool = Value::CONVERTS_WITHOUT_DM;

	fn from_byond(value: ByondValue) -> ByondResult<Self> {
		Value::from_byond(value).map(Saturating)
	}

	fn from_byond_ref(value: &ByondValue) -> ByondResult<Self> {
		Value::from_byond_ref(value).map(Saturating)
	}
}
