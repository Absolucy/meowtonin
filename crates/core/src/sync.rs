// SPDX-License-Identifier: 0BSD
use crate::{ByondValue, byond, panic::byond_crash, sys::CByondValue};
use std::{
	any::Any,
	cell::Cell,
	os::raw::c_void,
	panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
	sync::atomic::{AtomicUsize, Ordering},
	thread,
};

struct CallbackData<F: FnOnce() -> ByondValue + Send> {
	callback: F,
	caller: usize,
}

// 0 until the first export runs, and never a real thread ID on either OS
static MAIN_THREAD: AtomicUsize = AtomicUsize::new(0);

thread_local! {
	static IN_PLACE_PANIC: Cell<Option<Box<dyn Any + Send>>> = const { Cell::new(None) };
}

extern "C-unwind" fn trampoline<F: FnOnce() -> ByondValue + Send>(
	data: *mut c_void,
) -> CByondValue {
	let data = unsafe { Box::from_raw(data as *mut CallbackData<F>) };
	let CallbackData { callback, caller } = *data;
	// BYOND drops a queued callback's return value without a decref
	let queued = os_thread_id() != caller;
	match (catch_unwind(AssertUnwindSafe(callback)), queued) {
		(Ok(value), false) => value.detach(),
		(Ok(_value), true) => ByondValue::NULL.detach(),
		(Err(panic), false) => {
			IN_PLACE_PANIC.set(Some(panic));
			ByondValue::NULL.detach()
		}
		(Err(_panic), true) => byond_crash(
			crate::panic::get_stack_trace()
				.unwrap_or_else(|| "thread_sync callback panicked".to_owned()),
		),
	}
}

/// Runs `callback` on BYOND's main thread.
///
/// A non-blocking call from another thread returns null right away; the
/// callback's value is dropped and a panic is logged as a runtime error.
pub fn thread_sync<F>(callback: F, block: bool) -> ByondValue
where
	F: FnOnce() -> ByondValue + Send + 'static,
{
	if is_main_thread() {
		return callback();
	}
	if block {
		return sync_blocking(callback);
	}
	sync_queued(callback)
}

#[cold]
#[inline(never)]
fn sync_queued<F>(callback: F) -> ByondValue
where
	F: FnOnce() -> ByondValue + Send + 'static,
{
	let data = Box::new(CallbackData {
		callback,
		caller: os_thread_id(),
	});
	let data_ptr = Box::into_raw(data) as *mut c_void;

	let value =
		ByondValue(unsafe { byond().Byond_ThreadSync(Some(trampoline::<F>), data_ptr, false) });
	if let Some(panic) = IN_PLACE_PANIC.take() {
		resume_unwind(panic);
	}
	value
}

struct MainThreadJob<F, R> {
	job: Option<F>,
	output: Option<thread::Result<R>>,
}

extern "C-unwind" fn main_thread_trampoline<F: FnOnce() -> R, R>(data: *mut c_void) -> CByondValue {
	let slot = unsafe { &mut *(data as *mut MainThreadJob<F, R>) };
	if let Some(job) = slot.job.take() {
		slot.output = Some(catch_unwind(AssertUnwindSafe(job)));
	}
	ByondValue::NULL.detach()
}

/// Runs `job` on the main thread and waits for it.
pub(crate) fn on_main_thread<F, R>(job: F) -> R
where
	F: FnOnce() -> R + Send,
	R: Send,
{
	if is_main_thread() {
		return job();
	}
	sync_blocking(job)
}

#[cold]
#[inline(never)]
fn sync_blocking<F, R>(job: F) -> R
where
	F: FnOnce() -> R + Send,
	R: Send,
{
	let mut slot = MainThreadJob {
		job: Some(job),
		output: None,
	};
	unsafe {
		byond().Byond_ThreadSync(
			Some(main_thread_trampoline::<F, R>),
			(&raw mut slot).cast(),
			true,
		)
	};
	match slot
		.output
		.expect("blocking Byond_ThreadSync returned without running the job")
	{
		Ok(output) => output,
		Err(panic) => resume_unwind(panic),
	}
}

// BYOND only calls exports on the main thread, so the first one tells us which
// that is. before then, everything just goes through Byond_ThreadSync.
pub(crate) fn record_main_thread() {
	MAIN_THREAD.store(os_thread_id(), Ordering::Relaxed);
}

#[inline]
fn is_main_thread() -> bool {
	MAIN_THREAD.load(Ordering::Relaxed) == os_thread_id()
}

// not thread::current() or a thread_local: TLS in a dlopen'd .so costs ~120ns
// per access on glibc 2.41+, while pthread_self is ~1ns
#[inline]
fn os_thread_id() -> usize {
	#[cfg(unix)]
	{
		unsafe extern "C" {
			fn pthread_self() -> usize;
		}
		unsafe { pthread_self() }
	}
	#[cfg(windows)]
	{
		#[link(name = "kernel32")]
		unsafe extern "system" {
			fn GetCurrentThreadId() -> u32;
		}
		unsafe { GetCurrentThreadId() as usize }
	}
}

#[doc(hidden)]
#[repr(transparent)]
pub struct ByondCallee(CByondValue);

impl ByondCallee {
	pub fn finish_and_return(self, return_value: &CByondValue) -> bool {
		unsafe { byond().Byond_Return(&self.0, return_value) }
	}
}
