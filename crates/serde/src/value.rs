// SPDX-License-Identifier: 0BSD
use meowtonin::{ByondError, ByondResult, ByondValue, FromByond, ToByond};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
pub struct ByondSerde<T>(T);

impl<T> ByondSerde<T> {
	#[inline]
	pub fn new(value: T) -> Self {
		Self(value)
	}

	#[inline]
	pub fn into_inner(self) -> T {
		self.0
	}
}

impl<T> Deref for ByondSerde<T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for ByondSerde<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T> AsRef<T> for ByondSerde<T> {
	#[inline]
	fn as_ref(&self) -> &T {
		&self.0
	}
}

impl<T> AsMut<T> for ByondSerde<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut T {
		&mut self.0
	}
}

impl<T> FromByond for ByondSerde<T>
where
	T: DeserializeOwned,
{
	#[inline]
	fn from_byond(value: &ByondValue) -> ByondResult<Self> {
		crate::deserialize(value.clone())
			.map(Self::new)
			.map_err(ByondError::boxed)
	}
}

impl<T> ToByond for ByondSerde<T>
where
	T: Serialize,
{
	#[inline]
	fn to_byond(&self) -> ByondResult<ByondValue> {
		crate::serialize(&self.0).map_err(ByondError::boxed)
	}
}

impl<T> Clone for ByondSerde<T>
where
	T: Clone,
{
	#[inline]
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T> Copy for ByondSerde<T> where T: Copy {}

impl<T> std::fmt::Debug for ByondSerde<T>
where
	T: std::fmt::Debug,
{
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}
