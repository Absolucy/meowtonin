// SPDX-License-Identifier: 0BSD
inventory::collect!(InitFunc);

/// This function will be ran to set up things before the lib is loaded
/// The lib is only loaded when any byondapi functions are called from BYOND.
///
/// To submit a function to be ran by meowtonin when it loads, do this:
///
/// ```no_run
/// fn do_thing_on_init() {
///     println!("mrrrp mrrrp mrrow");
/// }
///
/// meowtonin::inventory::submit! { meowtonin::init::InitFunc(do_thing_on_init) }
/// ```
pub struct InitFunc(pub fn() -> ());

#[doc(hidden)]
pub fn do_init() {
	// Clear string ID cache, just in case anything's changed.
	crate::strid::STRID_CACHE.pin().clear();

	// Run any custom initialization functions.
	for func in inventory::iter::<InitFunc> {
		func.0();
	}
}
