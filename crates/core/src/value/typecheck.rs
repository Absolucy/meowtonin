// SPDX-License-Identifier: 0BSD
use crate::{byond, sys::ByondValueType as InternalByondValueType, ByondValue};
use std::{
	borrow::Cow,
	fmt::{self, Display},
	ops::Deref,
};

impl ByondValue {
	/// Determines if the [`ByondValue`] is a null value.
	///
	/// # Returns
	/// `true` if the value is null, `false` otherwise.
	pub fn is_null(&self) -> bool {
		unsafe { byond().ByondValue_IsNull(&self.0) }
	}

	/// Checks if the [`ByondValue`] is a number.
	///
	/// # Returns
	/// `true` if the value is a number, `false` otherwise.
	pub fn is_number(&self) -> bool {
		unsafe { byond().ByondValue_IsNum(&self.0) }
	}

	/// Checks if the [`ByondValue`] is a string.
	///
	/// # Returns
	/// `true` if the value is a string, `false` otherwise.
	pub fn is_string(&self) -> bool {
		unsafe { byond().ByondValue_IsStr(&self.0) }
	}

	/// Determines if the [`ByondValue`] represents a list.
	///
	/// # Returns
	/// `true` if the value is a list, `false` otherwise.
	pub fn is_list(&self) -> bool {
		unsafe { byond().ByondValue_IsList(&self.0) }
	}

	/// Evaluates whether the [`ByondValue`] is considered "true" or not.
	///
	/// # Returns
	/// `true` if the value is logically true, `false` otherwise.
	pub fn is_true(&self) -> bool {
		unsafe { byond().ByondValue_IsTrue(&self.0) }
	}

	/// Evaluates whether the [`ByondValue`] is a reference (object) type or
	/// not. Does not check validity.
	///
	/// # Returns
	/// `true` if the value is a reference, `false` otherwise.
	pub fn is_ref(&self) -> bool {
		unsafe { byond().ByondValue_GetRef(&self.0) != 0 }
	}
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ByondValueType(pub InternalByondValueType);

#[rustfmt::skip]
impl ByondValueType {
	/// The type of a null value.
	pub const NULL: Self = Self(0x00);
	/// A value that represents a `/turf` object.
	pub const TURF: Self = Self(0x01);
	/// A value that represents an `/obj` object.
	pub const OBJ: Self = Self(0x02);
	/// A value that represents a `/mob` object.
	pub const MOB: Self = Self(0x03);
	/// A value that represents an `/area` object.
	pub const AREA: Self = Self(0x04);
	/// A value that represents an `/client` object.
	pub const CLIENT: Self = Self(0x05);
	/// A value that represents a string.
	pub const STRING: Self = Self(0x06);
	/// A value that represents an `/mob` typepath;
	pub const MOB_TYPEPATH: Self = Self(0x08);
	/// A value that represents an `/obj` typepath;
	pub const OBJ_TYPEPATH: Self = Self(0x09);
	/// A value that represents an `/turf` typepath;
	pub const TURF_TYPEPATH: Self = Self(0x0A);
	/// A value that represents an `/area` typepath;
	pub const AREA_TYPEPATH: Self = Self(0x0B);
	/// A value that represents an `/image` object.
	pub const IMAGE: Self = Self(0x0D);
	/// A value that represents the `/world` object.
	pub const WORLD: Self = Self(0x0E);
	/// A value that represents a `/list` object.
	pub const LIST: Self = Self(0x0F);
	/// A value that represents a `/datum` typepath.
	pub const DATUM_TYPEPATH: Self = Self(0x20);
	/// A value that represents a `/datum` object.
	pub const DATUM: Self = Self(0x21);
	/// A value that represents a number.
	pub const NUMBER: Self = Self(0x2A);
	/// A pointer value.
	pub const POINTER: Self = Self(0x3C);
}

impl ByondValueType {
	/// Returns a simple string representation of the type.
	#[must_use]
	pub fn name(&self) -> Cow<'static, str> {
		match *self {
			Self::NULL => Cow::Borrowed("null"),
			Self::TURF => Cow::Borrowed("turf"),
			Self::OBJ => Cow::Borrowed("obj"),
			Self::MOB => Cow::Borrowed("mob"),
			Self::AREA => Cow::Borrowed("area"),
			Self::CLIENT => Cow::Borrowed("client"),
			Self::STRING => Cow::Borrowed("string"),
			Self::MOB_TYPEPATH => Cow::Borrowed("mob typepath"),
			Self::OBJ_TYPEPATH => Cow::Borrowed("obj typepath"),
			Self::TURF_TYPEPATH => Cow::Borrowed("turf typepath"),
			Self::AREA_TYPEPATH => Cow::Borrowed("area typepath"),
			Self::IMAGE => Cow::Borrowed("image"),
			Self::WORLD => Cow::Borrowed("world"),
			Self::LIST => Cow::Borrowed("list"),
			Self::DATUM_TYPEPATH => Cow::Borrowed("datum typepath"),
			Self::DATUM => Cow::Borrowed("datum"),
			Self::NUMBER => Cow::Borrowed("number"),
			Self::POINTER => Cow::Borrowed("pointer"),
			_ => Cow::Owned(format!("unknown type {:X}", self.0)),
		}
	}

	/// Returns if this type is reference counted or not.
	///
	/// # Returns
	/// `true` if the value is reference counted, `false` otherwise.
	///
	/// Currently, this only returns `false`` for [Self::NULL] and
	/// [Self::NUMBER].
	#[inline]
	pub const fn is_ref_counted(&self) -> bool {
		!matches!(*self, Self::NULL | Self::NUMBER)
	}

	/// Returns if this type SHOULD be reference counted.
	///
	/// The difference between this and [Self::is_ref_counted] is that this also
	/// checks to see if this type SHOULDN'T be refcounted, even if it is
	/// technically a reference.
	///
	/// # Returns
	/// `true` if the value should be reference counted, `false` otherwise.
	///
	/// Currently, this only returns `false` for [Self::NULL],
	/// [Self::NUMBER], and [Self::WORLD].
	#[inline]
	pub const fn should_ref_count(self) -> bool {
		// we have to compare the inner values for the world check to keep this const.
		// it's dumb, I know.
		self.is_ref_counted() && self.0 != Self::WORLD.0
	}
}

impl Display for ByondValueType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}

impl AsRef<InternalByondValueType> for ByondValueType {
	fn as_ref(&self) -> &InternalByondValueType {
		&self.0
	}
}

impl Deref for ByondValueType {
	type Target = InternalByondValueType;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl PartialEq<InternalByondValueType> for ByondValueType {
	fn eq(&self, other: &InternalByondValueType) -> bool {
		self.0 == *other
	}
}

impl PartialEq<ByondValueType> for InternalByondValueType {
	fn eq(&self, other: &ByondValueType) -> bool {
		*self == other.0
	}
}

impl From<InternalByondValueType> for ByondValueType {
	fn from(value: InternalByondValueType) -> Self {
		Self(value)
	}
}

impl From<ByondValueType> for InternalByondValueType {
	fn from(value: ByondValueType) -> Self {
		value.0
	}
}
