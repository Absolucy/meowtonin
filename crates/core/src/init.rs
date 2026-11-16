// SPDX-License-Identifier: 0BSD
inventory::collect!(InitFunc);

/// This function will be ran once, on the first byondapi call, right after
/// byondcore is loaded. It may call [`byond()`](crate::byond()).
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
	for func in inventory::iter::<InitFunc> {
		func.0();
	}
}
