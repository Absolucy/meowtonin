// SPDX-License-Identifier: 0BSD
use crate::{byond, cache::lookup_string_id, ByondResult, ByondValue};
use std::mem::MaybeUninit;

/// Calls a global proc.
///
/// Implicitly set waitfor=0, will never block.
pub fn call_global<Name, ArgList>(name: Name, args: ArgList) -> ByondResult<ByondValue>
where
	Name: AsRef<str>,
	ArgList: IntoIterator<Item = ByondValue>,
{
	let name_id = lookup_string_id(name);
	let args = args.into_iter().collect::<Vec<_>>();
	unsafe {
		let mut result = MaybeUninit::uninit();
		map_byond_error!(byond().Byond_CallGlobalProcByStrId(
			name_id,
			args.as_ptr().cast(),
			args.len() as _,
			result.as_mut_ptr(),
		))
		.map(|_| ByondValue(result.assume_init()))
	}
}
