// SPDX-License-Identifier: 0BSD
inventory::collect!(InitFunc);

/// This function will be ran to set up things before the lib is loaded
/// The lib is only loaded when any byondapi functions are called from byond
/// To submit a function (func) to be ran by byondapi when it loads, do:
/// ```no_run
/// meowtonin::inventory::submit! {InitFunc(func)}
/// ```
pub struct InitFunc(pub fn() -> ());

#[cfg(any(debug_assertions, feature = "rel-debugging"))]
inventory::submit! {
	InitFunc(|| unsafe {
		#[cfg(all(windows, not(feature = "rel-debugging")))] // don't allocate a console in release mode
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
