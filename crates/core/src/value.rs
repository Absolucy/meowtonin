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
#[derive(Clone)]
pub struct ByondValue(pub CByondValue);

impl ByondValue {
	/// A null value.
	pub const NULL: Self = unsafe { Self::new_ref_unchecked(ByondValueType::Null, 0) };

	/// Returns a null [ByondValue].
	#[deprecated(
		since = "0.2.0",
		note = "ByondValue::NULL is preferred over ByondValue::null()"
	)]
	pub const fn null() -> Self {
		Self::NULL
	}

	/// A reference to the "world" object, equivalent to DM's `world`.
	pub const fn world() -> &'static Self {
		static WORLD: ByondValue =
			unsafe { ByondValue::new_ref_unchecked(ByondValueType::World, 0) };

		&WORLD
	}

	/// Returns a reference to the "global" object, equivalent to DM's
	/// `global.vars`.
	pub const fn global() -> &'static Self {
		static GLOBAL: ByondValue =
			unsafe { ByondValue::new_ref_unchecked(ByondValueType::World, 1) };

		&GLOBAL
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
		tracy::zone!("ByondValue::new");
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
			Ok(Self(unsafe { result.assume_init() }))
		}
	}

	/// Returns the length of the value.
	///
	/// Equivalent to calling `length(self)` in DM.
	pub fn length(&self) -> ByondResult<usize> {
		tracy::zone!("ByondValue::length");
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
		tracy::zone!("ByondValue::typepath");
		self.read_var("type")
	}

	/// Read a variable through the ref. Fails if this isn't a ref type.
	pub fn read_var<Name, Return>(&self, name: Name) -> ByondResult<Return>
	where
		Name: AsRef<str>,
		Return: FromByond,
	{
		tracy::zone!("ByondValue::read_var");
		if !self.is_ref() {
			return Err(ByondError::NotReferenceable);
		}
		let name_id = lookup_string_id(name).ok_or(ByondError::InvalidVariable)?;
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_ReadVarByStrId(&self.0, name_id, result.as_mut_ptr()))?;
			Return::from_byond(Self(unsafe { result.assume_init() }))
		}
	}

	/// Write to a variable through the ref. Fails if this isn't a ref type.
	pub fn write_var<Name, Value>(&mut self, name: Name, value: Value) -> ByondResult<()>
	where
		Name: AsRef<str>,
		Value: ToByond,
	{
		tracy::zone!("ByondValue::write_var");
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
		tracy::zone!("ByondValue::read_pointer");
		if self.get_type() != ByondValueType::Pointer {
			return Err(ByondError::NotReferenceable);
		}
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_ReadPointer(&self.0, result.as_mut_ptr()))?;
			Return::from_byond(Self(unsafe { result.assume_init() }))
		}
	}

	pub fn write_pointer<Value>(&mut self, value: Value) -> ByondResult<()>
	where
		Value: ToByond,
	{
		tracy::zone!("ByondValue::write_pointer");
		if self.get_type() != ByondValueType::Pointer {
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
		tracy::zone!("ByondValue::pixloc");
		let mut pixloc = MaybeUninit::uninit();
		if unsafe { byond().Byond_PixLoc(&self.0, pixloc.as_mut_ptr()) } {
			Some(ByondPixLoc(unsafe { pixloc.assume_init() }))
		} else {
			None
		}
	}

	/// Equivalent to calling `istype(src, text2path(typepath))``.
	#[cfg(feature = "byond-1664")]
	pub fn is_type<Str>(&self, typepath: Str) -> bool
	where
		Str: AsRef<str>,
	{
		tracy::zone!("ByondValue::is_type");
		match std::ffi::CString::new(typepath.as_ref()) {
			Ok(typepath) => unsafe { byond().ByondValue_IsType(&self.0, typepath.as_ptr()) },
			Err(_) => false,
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
				ByondValueType::Null => (),
				ByondValueType::Number => self.0.data.num.to_bits().hash(state),
				_ => self.0.data.ref_.hash(state),
			}
		}
	}
}

impl fmt::Display for ByondValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let value_type = self.get_type();
		match value_type {
			ByondValueType::Null => write!(f, "null"),
			ByondValueType::String | ByondValueType::Number => {
				let string = self.get_string().unwrap_or_else(|_| String::from("???"));
				write!(f, "{string}")
			}
			ByondValueType::List => {
				let length = self.length().unwrap_or(0);
				write!(f, "list[len={length}]")
			}
			ByondValueType::Alist => {
				let length = self.length().unwrap_or(0);
				write!(f, "alist[len={length}]")
			}
			_ => {
				let type_name = value_type.name();
				let string = self.get_string().unwrap_or_else(|_| String::from("???"));
				write!(f, "<{type_name}>: {string}")
			}
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
