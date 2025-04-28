// SPDX-License-Identifier: 0BSD
inventory::collect!(InitFunc);

/// This function will be ran to set up things before the lib is loaded
/// The lib is only loaded when any byondapi functions are called from BYOND.
///
/// To submit a function to be ran by meowtonin when it loads, do this:
///
/// ```ignore
/// fn do_thing_on_init() {
///     println!("mrrrp mrrrp mrrow");
/// }
///
/// meowtonin::inventory::submit! { InitFunc(do_thing_on_init) }
/// ```
pub struct InitFunc(pub fn() -> ());

#[cfg(debug_assertions)]
inventory::submit! {
	InitFunc(|| unsafe {
		#[cfg(windows)]
		let _ = windows::Win32::System::Console::AllocConsole();
		use simplelog::*;
		let timestamp = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_secs();
		CombinedLogger::init(
			vec![
				TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
				WriteLogger::new(LevelFilter::Debug, Config::default(), std::fs::File::create(format!("meowtonin-{timestamp}.log")).unwrap()),
			]
		).unwrap();
	})
}

#[doc(hidden)]
pub fn do_init() {
	// Clear string ID cache, just in case anything's changed.
	crate::strid::STRID_CACHE.write().clear();

	// Run any custom initialization functions.
	for func in inventory::iter::<InitFunc> {
		func.0();
	}
}
