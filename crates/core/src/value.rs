// SPDX-License-Identifier: 0BSD
pub mod list;
pub mod num;
pub mod proc;
pub mod reference;
pub mod string;
pub mod typecheck;

use crate::{
	sys::{
		ByondValue_Equals, Byond_Length, Byond_New, Byond_ReadPointer, Byond_ReadVar,
		Byond_WritePointer, Byond_WriteVar, CByondValue,
	},
	ByondError, ByondResult, ByondValueType, FromByond, ToByond,
};
use std::{
	ffi::CString,
	fmt,
	hash::{Hash, Hasher},
	mem::MaybeUninit,
};

#[must_use]
#[repr(transparent)]
#[derive(Clone)]
pub struct ByondValue(pub CByondValue);

impl ByondValue {
	pub fn into_inner(self) -> CByondValue {
		self.0
	}

	pub fn null() -> Self {
		Self::default()
	}

	/// Shorthand for [ToByond::to_byond].
	pub fn new_value<Value>(value: Value) -> ByondResult<Self>
	where
		Value: ToByond,
	{
		value.to_byond()
	}

	/// Shorthand for [FromByond::from_byond].
	pub fn to<Return>(&self) -> ByondResult<Return>
	where
		Return: FromByond,
	{
		Return::from_byond(self)
	}

	/// Creates a new [ByondValue], using the given path and arguments.
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
			map_byond_error!(Byond_New(
				&path.0,
				args.as_ptr().cast(),
				args.len() as _,
				result.as_mut_ptr()
			))?;
			Ok(Self(result.assume_init()))
		}
	}

	/// Returns a reference to the "global" object.
	pub fn global() -> Self {
		// SAFETY: cross your fingers and pray
		unsafe { Self::new_ref_unchecked(ByondValueType::WORLD, 1) }
	}

	/// Returns the length of the value.
	/// Equivalent to calling `length(self)` in DM.
	pub fn length<Type>(&self) -> ByondResult<Type>
	where
		Type: FromByond,
	{
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(Byond_Length(&self.0, result.as_mut_ptr()))
				.and_then(|_| Type::from_byond(&Self(result.assume_init())))
		}
	}

	/// Gets the internal type of the value.
	pub fn get_type(&self) -> ByondValueType {
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
		let c_string = CString::new(name.as_ref()).map_err(|_| ByondError::NonUtf8String)?;
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(Byond_ReadVar(
				&self.0,
				c_string.as_c_str().as_ptr(),
				result.as_mut_ptr()
			))?;
			let result = Self(result.assume_init());
			Return::from_byond(&result)
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
		let value = value.to_byond()?;
		let c_string = CString::new(name.as_ref()).map_err(|_| ByondError::NonUtf8String)?;
		map_byond_error!(Byond_WriteVar(
			&self.0,
			c_string.as_c_str().as_ptr(),
			&value.0
		))
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
			map_byond_error!(Byond_ReadPointer(&self.0, result.as_mut_ptr()))?;
			let result = Self(result.assume_init());
			Return::from_byond(&result)
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
		unsafe { map_byond_error!(Byond_WritePointer(&self.0, &value.0)) }
	}
}

impl Default for ByondValue {
	fn default() -> Self {
		unsafe { Self(MaybeUninit::zeroed().assume_init()) }
	}
}

impl PartialEq for ByondValue {
	fn eq(&self, other: &Self) -> bool {
		unsafe { ByondValue_Equals(&self.0, &other.0) }
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

impl From<CByondValue> for ByondValue {
	fn from(value: CByondValue) -> Self {
		Self(value)
	}
}

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
				let length = self.length::<usize>().unwrap_or(0);
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

#[doc(hidden)]
pub fn test_byondvalue_clear_is_zero() {
	// Verify assumptions about the type
	assert!(
		!std::mem::needs_drop::<ByondValue>(),
		"ByondValue must not need dropping"
	);
	assert!(
		std::mem::size_of::<ByondValue>() > 0,
		"ByondValue must not be zero-sized"
	);

	let value_zeroed = unsafe { ByondValue(MaybeUninit::zeroed().assume_init()) };
	let value_cleared = unsafe {
		let mut value = MaybeUninit::uninit();
		crate::sys::ByondValue_Clear(value.as_mut_ptr());
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
