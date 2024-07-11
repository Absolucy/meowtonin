// SPDX-License-Identifier: 0BSD
pub mod list;
pub mod num;
pub mod proc;
pub mod reference;
pub mod string;
pub mod typecheck;

use crate::{byond, sys::CByondValue, ByondError, ByondResult, ByondValueType};
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
	#[inline]
	pub fn into_inner(self) -> CByondValue {
		self.0
	}

	#[inline]
	pub fn null() -> Self {
		Self::default()
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
			let path = ByondValue::new_string(path.into());
			let args = args.as_ref();
			map_byond_error!(byond().Byond_New(
				&path.0,
				args.as_ptr().cast(),
				args.len() as _,
				result.as_mut_ptr()
			))
			.map(|_| Self(result.assume_init()))
		}
	}

	/// Returns a reference to the "global" object.
	#[inline]
	pub fn global() -> Self {
		// SAFETY: cross your fingers and pray
		unsafe { Self::new_ref_unchecked(ByondValueType::WORLD, 1) }
	}

	/// Returns the length of the value.
	/// Equivalent to calling `length(self)` in DM.
	pub fn length(&self) -> ByondResult<ByondValue> {
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_Length(&self.0, result.as_mut_ptr()))
				.map(|_| Self(result.assume_init()))
		}
	}

	/// Gets the internal type of the value.
	#[inline]
	pub fn get_type(&self) -> ByondValueType {
		// Safety: This operation only fails if our CByondValue is invalid, which cannot
		// happen.
		ByondValueType(unsafe { byond().ByondValue_Type(&self.0) })
	}

	/// Returns the typepath of the value as a string, if it is a reference.
	#[inline]
	pub fn typepath(&self) -> ByondResult<String> {
		self.read_var("type").and_then(|var| var.get_string())
	}

	/// Read a variable through the ref. Fails if this isn't a ref type.
	pub fn read_var<Name>(&self, name: Name) -> ByondResult<ByondValue>
	where
		Name: AsRef<str>,
	{
		if self.is_number() || self.is_string() || self.is_null() || self.is_list() || self.is_ref()
		{
			return Err(ByondError::NotReferencable);
		}
		let c_string = CString::new(name.as_ref()).map_err(|_| ByondError::NonUtf8String)?;
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_ReadVar(
				&self.0,
				c_string.as_c_str().as_ptr(),
				result.as_mut_ptr()
			))
			.map(|_| Self(result.assume_init()))
		}
	}

	/// Write to a variable through the ref. Fails if this isn't a ref type.
	pub fn write_var<Name, Value>(&mut self, name: Name, value: ByondValue) -> ByondResult<()>
	where
		Name: AsRef<str>,
	{
		let c_string = CString::new(name.as_ref()).map_err(|_| ByondError::NonUtf8String)?;
		map_byond_error!(byond().Byond_WriteVar(&self.0, c_string.as_c_str().as_ptr(), &value.0))
	}

	pub fn read_pointer<Return>(&self) -> ByondResult<ByondValue> {
		if !self.is_ref() {
			return Err(ByondError::NotReferencable);
		}
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_ReadPointer(&self.0, result.as_mut_ptr()))
				.map(|_| Self(result.assume_init()))
		}
	}

	pub fn write_pointer<Value>(&mut self, value: ByondValue) -> ByondResult<()> {
		if !self.is_ref() {
			return Err(ByondError::NotReferencable);
		}
		unsafe { map_byond_error!(byond().Byond_WritePointer(&self.0, &value.0)) }
	}
}

impl Default for ByondValue {
	#[inline]
	fn default() -> Self {
		unsafe {
			let mut value = MaybeUninit::uninit();
			byond().ByondValue_Clear(value.as_mut_ptr());
			Self(value.assume_init())
		}
	}
}

impl PartialEq for ByondValue {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		unsafe { byond().ByondValue_Equals(&self.0, &other.0) }
	}
}

impl Eq for ByondValue {}

impl From<CByondValue> for ByondValue {
	#[inline]
	fn from(value: CByondValue) -> Self {
		Self(value)
	}
}

impl Hash for ByondValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let value_type = self.get_type();
		value_type.0.hash(state);
		match value_type {
			ByondValueType::NULL => {}
			ByondValueType::CLIENT => {
				if let Ok(ckey) = self.read_var("ckey").and_then(|value| value.get_string()) {
					ckey.hash(state);
				}
			}
			ByondValueType::NUMBER => self.get_number().unwrap_or(0.0).to_le_bytes().hash(state),
			_ => {
				if let Ok(value) = self.get_string() {
					value.hash(state);
				}
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
				let length = self
					.length()
					.and_then(|val| val.get_number())
					.unwrap_or(0.0) as usize;
				write!(f, "list[len={length}]")
			}
			ByondValueType::POINTER => {
				let ref_id = self.ref_id().unwrap_or(0);
				let ref_count = self.ref_count().unwrap_or(0);
				let path = self.typepath().unwrap_or_else(|_| String::from("???"));
				write!(f, "ref[id={ref_id:#04x}, count={ref_count}, type={path}]")
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
