// SPDX-License-Identifier: 0BSD
pub mod list;
pub mod num;
pub mod proc;
pub mod reference;
pub mod string;
pub mod typecheck;

use crate::{
	ByondError, ByondResult, ByondValueType, FromByond, ToByond, byond, pixloc::ByondPixLoc,
	strid::lookup_string_id, sys::CByondValue,
};
use std::{
	fmt,
	hash::{Hash, Hasher},
	mem::MaybeUninit,
};

#[must_use]
#[repr(transparent)]
pub struct ByondValue(pub CByondValue);

impl ByondValue {
	/// A null value.
	pub const NULL: Self = unsafe { Self::new_ref_unchecked(ByondValueType::NULL, 0) };

	/// A reference to the "global" object, equivalent to DM's `global.vars`.
	pub const GLOBAL: Self = unsafe { Self::new_ref_unchecked(ByondValueType::WORLD, 1) };

	/// A reference to the "world" object, equivalent to DM's `world`.
	pub const WORLD: Self = unsafe { Self::new_ref_unchecked(ByondValueType::WORLD, 0) };

	/// Returns the inner [CByondValue], without decrementing the refcount.
	/// You should only use this if you know what you're doing!
	pub unsafe fn into_inner(self) -> CByondValue {
		let inner = self.0;
		std::mem::forget(self);
		inner
	}

	/// Returns a null [ByondValue].
	#[deprecated(
		since = "0.2.0",
		note = "ByondValue::NULL is preferred over ByondValue::null()"
	)]
	pub const fn null() -> Self {
		Self::NULL
	}

	/// Shorthand for [ToByond::to_byond].
	pub fn new_value<Value>(value: Value) -> ByondResult<Self>
	where
		Value: ToByond,
	{
		value.to_byond()
	}

	/// Shorthand for [FromByond::from_byond].
	pub fn to<Return>(self) -> ByondResult<Return>
	where
		Return: FromByond,
	{
		Return::from_byond(self)
	}

	/// Creates a new [ByondValue], using the given path and arguments.
	///
	/// Equivalent to `new path(args...)` in DM.
	pub fn new<Path, Args>(path: Path, args: Args) -> ByondResult<Self>
	where
		Path: Into<String>,
		Args: AsRef<[Self]>,
	{
		unsafe {
			let mut result = MaybeUninit::uninit();
			let path = path.into().to_byond()?;
			let args = args.as_ref();
			map_byond_error!(byond().Byond_New(
				&path.0,
				args.as_ptr().cast(),
				args.len() as _,
				result.as_mut_ptr()
			))?;
			Ok(Self::initialize_refcounted(result))
		}
	}

	/// Returns the length of the value.
	///
	/// Equivalent to calling `length(self)` in DM.
	pub fn length(&self) -> ByondResult<usize> {
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_Length(&self.0, result.as_mut_ptr()))?;
			Self(unsafe { result.assume_init() })
				.get_number()
				.map(|size| size as usize)
		}
	}

	/// Gets the internal type of the value.
	#[inline]
	pub const fn get_type(&self) -> ByondValueType {
		ByondValueType(self.0.type_)
	}

	/// Returns the typepath of the value as a string, if it is a reference.
	pub fn typepath(&self) -> ByondResult<String> {
		self.read_var("type")
	}

	/// Read a variable through the ref. Fails if this isn't a ref type.
	pub fn read_var<Name, Return>(&self, name: Name) -> ByondResult<Return>
	where
		Name: AsRef<str>,
		Return: FromByond,
	{
		if !self.is_ref() {
			return Err(ByondError::NotReferenceable);
		}
		let name_id = lookup_string_id(name).ok_or(ByondError::InvalidVariable)?;
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_ReadVarByStrId(&self.0, name_id, result.as_mut_ptr()))?;
			Return::from_byond(Self::initialize_refcounted(result))
		}
	}

	/// Write to a variable through the ref. Fails if this isn't a ref type.
	pub fn write_var<Name, Value>(&mut self, name: Name, value: Value) -> ByondResult<()>
	where
		Name: AsRef<str>,
		Value: ToByond,
	{
		if !self.is_ref() {
			return Err(ByondError::NotReferenceable);
		}
		let name_id = lookup_string_id(name).ok_or(ByondError::InvalidVariable)?;
		let value = value.to_byond()?;
		map_byond_error!(byond().Byond_WriteVarByStrId(&self.0, name_id, &value.0))
	}

	pub fn read_pointer<Return>(&self) -> ByondResult<Return>
	where
		Return: FromByond,
	{
		if self.get_type() != ByondValueType::POINTER {
			return Err(ByondError::NotReferenceable);
		}
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_ReadPointer(&self.0, result.as_mut_ptr()))?;
			Return::from_byond(Self::initialize_refcounted(result))
		}
	}

	pub fn write_pointer<Value>(&mut self, value: Value) -> ByondResult<()>
	where
		Value: ToByond,
	{
		if self.get_type() != ByondValueType::POINTER {
			return Err(ByondError::NotReferenceable);
		}
		let value = value.to_byond()?;
		unsafe { map_byond_error!(byond().Byond_WritePointer(&self.0, &value.0)) }
	}

	/// Gets the pixloc coordinates of an atom.
	///
	/// Returns `None` if the value doesn't have pixloc coordinates, such as if
	/// value is not an atom.
	///
	/// If the atom is off-map, this will return [ByondPixLoc::ZERO].
	pub fn pixloc(&self) -> Option<ByondPixLoc> {
		let mut pixloc = MaybeUninit::uninit();
		if unsafe { byond().Byond_PixLoc(&self.0, pixloc.as_mut_ptr()) } {
			Some(ByondPixLoc(unsafe { pixloc.assume_init() }))
		} else {
			None
		}
	}
}

impl Default for ByondValue {
	fn default() -> Self {
		unsafe { Self::NULL }
	}
}

impl PartialEq for ByondValue {
	fn eq(&self, other: &Self) -> bool {
		unsafe { byond().ByondValue_Equals(&self.0, &other.0) }
	}
}

impl PartialEq<&ByondValue> for ByondValue {
	fn eq(&self, other: &&Self) -> bool {
		unsafe { byond().ByondValue_Equals(&self.0, &other.0) }
	}
}

impl PartialEq<bool> for ByondValue {
	fn eq(&self, other: &bool) -> bool {
		self.is_true() == *other
	}
}

impl PartialEq<ByondValue> for bool {
	fn eq(&self, other: &ByondValue) -> bool {
		*self == other.is_true()
	}
}

impl Eq for ByondValue {}

impl Hash for ByondValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let value_type = self.get_type();
		value_type.0.hash(state);
		unsafe {
			match value_type {
				ByondValueType::NULL => (),
				ByondValueType::NUMBER => self.0.data.num.to_bits().hash(state),
				_ => self.0.data.ref_.hash(state),
			}
		}
	}
}

impl fmt::Display for ByondValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let value_type = self.get_type();
		match value_type {
			ByondValueType::NULL => write!(f, "null"),
			ByondValueType::STRING | ByondValueType::NUMBER => {
				let string = self.get_string().unwrap_or_else(|_| String::from("???"));
				write!(f, "{string}")
			}
			ByondValueType::LIST => {
				let length = self.length().unwrap_or(0);
				write!(f, "list[len={length}]")
			}
			ByondValueType::MOB_TYPEPATH
			| ByondValueType::OBJ_TYPEPATH
			| ByondValueType::TURF_TYPEPATH
			| ByondValueType::AREA_TYPEPATH
			| ByondValueType::DATUM_TYPEPATH => {
				let string = self.get_string().unwrap_or_else(|_| String::from("???"));
				write!(f, "{string}")
			}
			_ => {
				let type_name = value_type.name();
				let string = self.get_string().unwrap_or_else(|_| String::from("???"));
				write!(f, "<{type_name}>: {string}")
			}
		}
	}
}

impl Clone for ByondValue {
	fn clone(&self) -> Self {
		if self.get_type().should_ref_count() {
			unsafe { self.inc_ref() };
		}
		Self(self.0)
	}
}

impl Drop for ByondValue {
	fn drop(&mut self) {
		if self.get_type().should_ref_count() {
			unsafe { self.dec_ref() };
		}
	}
}

#[doc(hidden)]
pub fn test_byondvalue_clear_is_zero() {
	assert!(
		std::mem::size_of::<ByondValue>() > 0,
		"ByondValue must not be zero-sized"
	);

	let value_zeroed = unsafe { ByondValue(MaybeUninit::zeroed().assume_init()) };
	let value_cleared = unsafe {
		let mut value = MaybeUninit::uninit();
		byond().ByondValue_Clear(value.as_mut_ptr());
		ByondValue(value.assume_init())
	};

	let value_zeroed = unsafe {
		std::slice::from_raw_parts(
			&value_zeroed as *const _ as *const u8,
			std::mem::size_of::<ByondValue>(),
		)
	};

	let value_cleared = unsafe {
		std::slice::from_raw_parts(
			&value_cleared as *const _ as *const u8,
			std::mem::size_of::<ByondValue>(),
		)
	};

	// Compare the memory representations
	assert_eq!(
		value_zeroed, value_cleared,
		"Memory representations differ: \nzeroed: {value_zeroed:?}\ncleared: {value_cleared:?}",
	);
}
