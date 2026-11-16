// SPDX-License-Identifier: 0BSD
use meowtonin_byondapi_sys::ByondApi;
use std::sync::OnceLock;

static BYOND: OnceLock<ByondApi> = OnceLock::new();

fn load_api() -> ByondApi {
	// Dynamically load the byondcore dylib.
	let library = {
		cfg_if::cfg_if! {
			if #[cfg(windows)] {
				libloading::os::windows::Library::open_already_loaded("byondcore.dll")
					.expect("byondcore.dll is not loaded into the process as expected")
			} else {
				libloading::os::unix::Library::this()
			}
		}
	};
	// Initialize ByondApi with the loaded dylib.
	unsafe { meowtonin_byondapi_sys::ByondApi::init_from_library(library) }
		.expect("failed to initialize byondapi")
}

#[cold]
#[inline(never)]
fn init_lib() -> &'static ByondApi {
	let mut loaded_here = false;
	let api = BYOND.get_or_init(|| {
		loaded_here = true;
		load_api()
	});
	// outside the OnceLock init, so an init func can call byond()
	if loaded_here {
		crate::init::do_init();
	}
	api
}

/// Gets the global [ByondApi] instance, initializing it if necessary.
#[inline(always)]
#[must_use]
pub fn byond() -> &'static ByondApi {
	match BYOND.get() {
		Some(api) => api,
		None => init_lib(),
	}
}
