// SPDX-License-Identifier: 0BSD
use crate::{byond, sys::CByondValue, ByondValue};
use std::ffi::c_void;

type RustByondCallback = Box<dyn FnOnce() -> ByondValue + Send + 'static>;

unsafe extern "C" fn thread_sync_callback(data: *mut c_void) -> CByondValue {
	let data: Box<RustByondCallback> = Box::from_raw(data as *mut RustByondCallback);
	(*data)().into_inner()
}

pub fn thread_sync<Callback>(callback: Callback, blocking: bool) -> ByondValue
where
	Callback: FnOnce() -> ByondValue + Send + 'static,
{
	let callback = Box::into_raw(Box::new(callback));
	unsafe {
		byond()
			.Byond_ThreadSync(Some(thread_sync_callback), callback.cast(), blocking)
			.into()
	}
}
