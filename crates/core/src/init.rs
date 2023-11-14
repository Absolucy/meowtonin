// SPDX-License-Identifier: 0BSD
inventory::collect!(InitFunc);

/// This function will be ran to set up things before the lib is loaded
/// The lib is only loaded when any byondapi functions are called from byond
/// To submit a function (func) to be ran by byondapi when it loads, do:
/// ```no_run
/// meowtonin::inventory::submit! {InitFunc(func)}
/// ```
pub struct InitFunc(pub fn() -> ());
