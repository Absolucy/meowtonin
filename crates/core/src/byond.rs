// SPDX-License-Identifier: 0BSD
use crate::init::InitFunc;
use byondapi_sys::ByondApi;
use std::sync::OnceLock;

static BYOND: OnceLock<ByondApi> = OnceLock::new();

fn init_lib() -> ByondApi {
	// Clear string ID cache, just in case anything's changed.
	crate::strid::STRID_CACHE.write().clear();

	// Run any custom initialization functions.
	for func in inventory::iter::<InitFunc> {
		func.0();
	}
	// Dynamically load the byondcore dylib.
	let library = {
		cfg_if! {
			if #[cfg(windows)] {
				libloading::os::windows::Library::open_already_loaded("byondcore.dll")
					.expect("byondcore.dll is not loaded into the process as expected")
			} else {
				libloading::os::unix::Library::this()
			}
		}
	};
	// Initialize ByondApi with the loaded dylib.
	unsafe { byondapi_sys::ByondApi::init_from_library(library) }
		.expect("failed to initialize byondapi")
}

/// Gets the global [`ByondApi`] instance, initializing it if necessary.
#[inline(always)]
#[must_use]
pub fn byond() -> &'static ByondApi {
	BYOND.get_or_init(init_lib)
}
