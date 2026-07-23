// SPDX-License-Identifier: 0BSD
use crate::{ByondValue, byond, sys::CByondValue};
use std::os::raw::c_void;

struct CallbackData<F: FnOnce() -> ByondValue + Send> {
	callback: F,
}

extern "C-unwind" fn trampoline<F: FnOnce() -> ByondValue + Send>(
	data: *mut c_void,
) -> CByondValue {
	let data = unsafe { Box::from_raw(data as *mut CallbackData<F>) };
	unsafe { (data.callback)().detach() }
}

pub fn thread_sync<F>(callback: F, block: bool) -> ByondValue
where
	F: FnOnce() -> ByondValue + Send + 'static,
{
	let data = Box::new(CallbackData { callback });
	let data_ptr = Box::into_raw(data) as *mut c_void;

	ByondValue(unsafe { byond().Byond_ThreadSync(Some(trampoline::<F>), data_ptr, block) })
}

#[doc(hidden)]
#[repr(transparent)]
pub struct ByondCallee(CByondValue);

impl ByondCallee {
	pub fn finish_and_return(self, return_value: &CByondValue) -> bool {
		unsafe { byond().Byond_Return(&self.0, return_value) }
	}
}
