// SPDX-License-Identifier: 0BSD
use crate::{ByondValue, RcByondValue, byond, sys::CByondValue};
use std::{cell::Cell, os::raw::c_void, sync::OnceLock, thread::ThreadId};

struct CallbackData<F: FnOnce() -> ByondValue + Send> {
	callback: F,
}

extern "C-unwind" fn trampoline<F: FnOnce() -> ByondValue + Send>(
	data: *mut c_void,
) -> CByondValue {
	let _guard = ThreadSyncGuard::new();
	let data = unsafe { Box::from_raw(data as *mut CallbackData<F>) };
	unsafe { (data.callback)().0 }
}

pub fn thread_sync<F>(callback: F, block: bool) -> RcByondValue
where
	F: FnOnce() -> ByondValue + Send + 'static,
{
	let data = Box::new(CallbackData { callback });
	let data_ptr = Box::into_raw(data) as *mut c_void;

	tracy::zone!("thread_sync");
	RcByondValue::new_from_persistent(ByondValue(unsafe {
		byond().Byond_ThreadSync(Some(trampoline::<F>), data_ptr, block)
	}))
}

thread_local! {
	static THREAD_SYNC_DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// Checks to see if we're in a ThreadSync call or not.
pub fn is_in_thread_sync() -> bool {
	tracy::zone!("is_in_thread_sync");
	THREAD_SYNC_DEPTH.with(|depth| depth.get() > 0)
}

/// Simple RAII counter to mark if we're in a ThreadSync call or not, due to
/// differing refcounting behavior.
///
/// This uses a counter instead of a bool, to account for weird cases involving
/// potentially nested ThreadSync calls.
///
/// I hate that this is needed.
struct ThreadSyncGuard;

impl ThreadSyncGuard {
	fn new() -> Self {
		THREAD_SYNC_DEPTH.with(|depth| depth.set(depth.get() + 1));
		Self
	}
}

impl Drop for ThreadSyncGuard {
	fn drop(&mut self) {
		THREAD_SYNC_DEPTH.with(|depth| {
			let current = depth.get();
			assert!(
				current > 0,
				"ThreadSyncGuard somehow dropped more than created, WTF DID YOU DO???"
			);
			depth.set(current - 1);
		});
	}
}

/// Checks to see if we're on the main thread or not.
pub fn is_main_thread() -> bool {
	static MAIN_THREAD_ID: OnceLock<ThreadId> = OnceLock::new();

	tracy::zone!("is_main_thread");
	let thread_id = std::thread::current().id();
	*MAIN_THREAD_ID.get_or_init(|| thread_id) == thread_id
}

/// Returns if we should manually increment a persistent ref.
/// This returns true if we're on the main thread, and NOT in threadsync.
pub fn should_setup_ref_counting() -> bool {
	tracy::zone!("should_setup_ref_counting");
	is_main_thread() && !is_in_thread_sync()
}
