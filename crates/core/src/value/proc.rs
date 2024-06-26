// SPDX-License-Identifier: 0BSD
use crate::{
	byond, ByondError, ByondResult, ByondValue, FromByond,
	ToByond, /* cache::lookup_string_id, */
};
use std::{ffi::CString, mem::MaybeUninit};

impl ByondValue {
	/// Calls a proc on the given value.
	///
	/// Implicitly set waitfor=0, will never block.
	pub fn call<Name, Arg, ArgList, Return>(&self, name: Name, args: ArgList) -> ByondResult<Return>
	where
		Name: AsRef<str>,
		Arg: ToByond,
		ArgList: IntoIterator<Item = Arg>,
		Return: FromByond,
	{
		/* let name_id = lookup_string_id(name); */
		let name = CString::new(name.as_ref()).map_err(|_| ByondError::NonUtf8String)?;
		let args = args
			.into_iter()
			.map(|arg| arg.to_byond())
			.collect::<ByondResult<Vec<_>>>()?;
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_CallProc(
				&self.0,
				name.as_ptr(),
				args.as_ptr().cast(),
				args.len() as _,
				result.as_mut_ptr(),
			))?;
			let result = Self(result.assume_init());
			Return::from_byond(&result)
		}
	}
}
