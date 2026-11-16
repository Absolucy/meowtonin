// SPDX-License-Identifier: 0BSD
pub mod container;
pub mod list;
pub mod num;
pub mod string;

use crate::{ByondResult, ByondValue};
use std::{borrow::Cow, marker::PhantomData, mem};

pub trait ToByond {
	fn to_byond(&self) -> ByondResult<ByondValue>;

	/// Like [`to_byond`](Self::to_byond), but borrows a [`ByondValue`] instead
	/// of cloning it.
	fn to_byond_borrowed(&self) -> ByondResult<Cow<'_, ByondValue>> {
		self.to_byond().map(Cow::Owned)
	}

	/// Like [`to_byond`](Self::to_byond), but moves a [`ByondValue`] and
	/// borrows a `&ByondValue` instead of cloning.
	fn into_byond_cow<'a>(self) -> ByondResult<Cow<'a, ByondValue>>
	where
		Self: Sized + 'a,
	{
		self.to_byond().map(Cow::Owned)
	}
}

impl ToByond for ByondValue {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(self.clone())
	}

	fn to_byond_borrowed(&self) -> ByondResult<Cow<'_, ByondValue>> {
		Ok(Cow::Borrowed(self))
	}

	fn into_byond_cow<'a>(self) -> ByondResult<Cow<'a, ByondValue>> {
		Ok(Cow::Owned(self))
	}
}

impl ToByond for &ByondValue {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok((*self).clone())
	}

	fn to_byond_borrowed(&self) -> ByondResult<Cow<'_, ByondValue>> {
		Ok(Cow::Borrowed(*self))
	}

	fn into_byond_cow<'a>(self) -> ByondResult<Cow<'a, ByondValue>>
	where
		Self: 'a,
	{
		Ok(Cow::Borrowed(self))
	}
}

impl ToByond for () {
	fn to_byond(&self) -> ByondResult<ByondValue> {
		Ok(ByondValue::default())
	}
}

/// Values laid out for BYOND. Borrowed ones are unreferenced bit copies.
pub(crate) struct RawValues<'a> {
	values: Vec<ByondValue>,
	/// bit n set: `values[n]` is borrowed, never drop it
	borrowed: u64,
	_borrows: PhantomData<&'a ByondValue>,
}

impl<'a> RawValues<'a> {
	pub(crate) fn collect(
		items: impl Iterator<Item = ByondResult<Cow<'a, ByondValue>>>,
	) -> ByondResult<Self> {
		let mut raw = Self {
			values: Vec::with_capacity(items.size_hint().0),
			borrowed: 0,
			_borrows: PhantomData,
		};
		for item in items {
			let idx = raw.values.len();
			let value = match item? {
				Cow::Owned(value) => value,
				// out of mask bits
				Cow::Borrowed(value) if idx >= 64 => value.clone(),
				Cow::Borrowed(value) => {
					raw.borrowed |= 1 << idx;
					ByondValue(value.0)
				}
			};
			raw.values.push(value);
		}
		Ok(raw)
	}

	pub(crate) fn as_slice(&self) -> &[ByondValue] {
		&self.values
	}
}

impl Drop for RawValues<'_> {
	fn drop(&mut self) {
		for (idx, value) in self.values.iter_mut().enumerate().take(64) {
			if self.borrowed & (1 << idx) != 0 {
				mem::forget(mem::take(value));
			}
		}
	}
}
