// SPDX-License-Identifier: 0BSD
use crate::ByondValue;
use backtrace::Backtrace;
use serde::Serialize;
use std::{borrow::Cow, cell::RefCell, panic::PanicInfo, sync::Once};

/// A panic that occurred in the code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Panic {
	/// The panic message.
	pub message: Cow<'static, str>,
	/// The location of the panic.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<PanicLocation>,
	/// The backtrace of the panic.
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub backtrace: Vec<PanicFrame>,
}

/// Information about the origin of the code that caused a panic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PanicLocation {
	/// The original source file containing the code that resulted in the panic.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file: Option<String>,
	/// The line number of the file containing the code that resulted in the
	/// panic.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub line: Option<u32>,
}

/// A frame in a panic backtrace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PanicFrame {
	/// The name of the function called in this frame.
	pub name: String,
	/// The source file containing the code of this frame.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file: Option<String>,
	/// The line number of this frame.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub line: Option<u32>,
	/// The memory address of this frame.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub address: Option<String>,
}

thread_local! {
	static LAST_PANIC: RefCell<Option<Panic>> = RefCell::new(None);
}

static SET_HOOK: Once = Once::new();

const BLACKLIST: &[&str] = &[
	"libbyond",
	"byondcore",
	"ntdll",
	"backtrace",
	"boxed::impl",
	"std::panic",
	"core::panic",
	"std::rt",
	"__scrt",
	"Rtl",
	"BaseThread",
	"Thunk",
	"invoke_main",
];

fn panic_hook(panic_info: &PanicInfo) {
	let message = panic_info
		.payload()
		.downcast_ref::<&'static str>()
		.map(|payload| Cow::Borrowed(*payload))
		.or_else(|| {
			panic_info
				.payload()
				.downcast_ref::<String>()
				.map(|payload| Cow::Owned(payload.clone()))
		});
	let message = match message {
		Some(message) => message,
		None => {
			LAST_PANIC.set(None);
			return;
		}
	};
	let location = panic_info.location().map(|location| PanicLocation {
		file: Some(location.file().to_owned()),
		line: Some(location.line()),
	});
	let backtrace = Backtrace::new()
		.frames()
		.iter()
		.flat_map(|frame| frame.symbols())
		.filter_map(|frame| frame.name().map(|name| (frame, name.to_string())))
		.filter(|(_frame, name)| !BLACKLIST.iter().any(|term| name.contains(term)))
		.map(|(frame, name)| PanicFrame {
			name,
			file: frame.filename().map(|file| file.display().to_string()),
			line: frame.lineno(),
			address: frame.addr().map(|addr| format!("{:p}", addr)),
		})
		.collect::<Vec<_>>();
	let panic = Panic {
		message,
		location,
		backtrace,
	};
	if cfg!(debug_assertions) {
		let timestamp = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_secs();
		let filename = format!("meowtonin-panic-{}.json", timestamp);
		let file = std::io::BufWriter::new(std::fs::File::create(filename).unwrap());
		serde_json::to_writer_pretty(file, &panic).unwrap();
	}
	LAST_PANIC.set(Some(panic));
}

/// Sets up a panic hook to capture a backtrace.
pub fn setup_panic_hook() {
	SET_HOOK.call_once(|| {
		std::panic::set_hook(Box::new(panic_hook));
	});
}

/// Gets the last panic that occurred, resetting it to `None`.
#[inline]
pub fn get_last_panic() -> ByondValue {
	#[cfg(all(target_os = "linux", debug_assertions))]
	LAST_PANIC.with_borrow(|panic| eprintln!("=== PANIC! ===\n{:#?}\n=== PANIC! ===", panic));
	match LAST_PANIC
		.take()
		.and_then(|panic| serde_json::to_string(&panic).ok())
		.map(|panic_json| ByondValue::new_string(format!("PANIC:{panic_json}")))
	{
		Some(panic_json) => panic_json,
		None => ByondValue::null(),
	}
}
