// SPDX-License-Identifier: 0BSD
use crate::{byond, strid::lookup_string_id, ByondResult, ByondValue, FromByond, ToByond};
use std::mem::MaybeUninit;

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
		let name_id = lookup_string_id(name);
		let args = args
			.into_iter()
			.map(|arg| arg.to_byond())
			.collect::<ByondResult<Vec<_>>>()?;
		unsafe {
			let mut result = MaybeUninit::uninit();
			map_byond_error!(byond().Byond_CallProcByStrId(
				&self.0,
				name_id,
				args.as_ptr().cast(),
				args.len() as _,
				result.as_mut_ptr(),
			))?;
			let result = Self(result.assume_init());
			Return::from_byond(&result)
		}
	}
}
