// SPDX-License-Identifier: 0BSD
mod resolve;

use aho_corasick::AhoCorasick;
use backtrace::{Backtrace, BacktraceSymbol};
use parking_lot::RwLock;
use serde::Serialize;
use smol_str::SmolStr;
use std::{
	borrow::Cow,
	cell::RefCell,
	ffi::{CString, c_void},
	panic::PanicHookInfo,
	path::{Path, PathBuf},
	sync::LazyLock,
};

use crate::{ByondValue, byond};

static INTERNAL_PATTERNS: LazyLock<AhoCorasick> = LazyLock::new(|| {
	AhoCorasick::new([
		"std::rt::",
		"std::panicking",
		"core::ops",
		"std::sys::",
		"runtime::",
		"<core::",
		"__rust_",
		"sys_common::",
		"panic_fmt",
		"rust_begin_unwind",
		"catch_unwind",
		"panic::",
		"lang_start",
		"libc_start_main",
		"_start",
		"try::do_call",
		"function::impls",
		"setup_panic_hook",
		"Ordinal",
	])
	.expect("failed to build internal pattern matcher")
});

static PATH_PATTERNS: LazyLock<AhoCorasick> = LazyLock::new(|| {
	AhoCorasick::new([
		"/std/src/",
		"/core/src/",
		"/libc/",
		r"\std\src\",
		r"\core\src\",
		r"\libc\",
	])
	.expect("failed to build internal path matcher")
});

static MODULE_PATTERNS: LazyLock<AhoCorasick> = LazyLock::new(|| {
	AhoCorasick::builder()
		.ascii_case_insensitive(true)
		.build([
			"user32.dll",
			"byondcore.dll",
			"ntdll.dll",
			"kernel32.dll",
			"libbyond.so",
		])
		.expect("failed to build internal module matcher")
});

fn is_relevant_symbol(symbol: &BacktraceSymbol) -> bool {
	match symbol.name() {
		Some(name) => {
			let name = name.to_string();
			name != "main"
				&& !INTERNAL_PATTERNS.is_match(&name)
				&& !symbol
					.filename()
					.map(Path::to_string_lossy)
					.is_some_and(|file| PATH_PATTERNS.is_match(file.as_ref()))
		}
		None => false,
	}
}

fn is_relevant_module(module: Option<&SmolStr>) -> bool {
	match module {
		Some(module) => !MODULE_PATTERNS.is_match(module),
		None => true,
	}
}

static PANIC_OUTPUT_FOLDER: LazyLock<RwLock<PathBuf>> =
	LazyLock::new(|| RwLock::new(PathBuf::from(".")));

/// Sets the folder where panic output files will be written.
pub fn set_panic_output_folder(path: impl AsRef<Path>) {
	let path = path.as_ref().to_path_buf();
	if path.exists() || std::fs::create_dir_all(&path).is_ok() {
		*PANIC_OUTPUT_FOLDER.write() = path;
	} else {
		*PANIC_OUTPUT_FOLDER.write() = PathBuf::from(".");
	}
}

/// A panic that occurred in the code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Panic {
	/// The panic message.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<Cow<'static, str>>,
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
	pub file: String,
	/// The line number of the file containing the code that resulted in the
	/// panic.
	pub line: u32,
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
	/// The module of this frame.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub module: Option<SmolStr>,
}

fn encode_panic(panic_info: &PanicHookInfo) -> Panic {
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
	let location = panic_info.location().map(|location| PanicLocation {
		file: location.file().to_owned(),
		line: location.line(),
	});
	let backtrace = Backtrace::new()
		.frames()
		.iter()
		.flat_map(|frame| {
			let module: Option<SmolStr> = frame
				.module_base_address()
				.and_then(resolve::resolve_module_name);
			frame.symbols().iter().map(move |sym| (sym, module.clone()))
		})
		.filter(|(symbol, module)| {
			is_relevant_module(module.as_ref()) && is_relevant_symbol(symbol)
		})
		.filter_map(|(symbol, module)| {
			Some(PanicFrame {
				name: symbol.name()?.to_string(),
				file: symbol
					.filename()
					.map(|file| file.to_string_lossy().into_owned()),
				line: symbol.lineno(),
				address: symbol.addr().map(|addr| {
					const POINTER_HEX_WIDTH: usize = std::mem::size_of::<*mut c_void>() * 2;
					format!("{addr:0POINTER_HEX_WIDTH$p}")
				}),
				module,
			})
		})
		.collect::<Vec<_>>();
	Panic {
		message,
		location,
		backtrace,
	}
}

thread_local! {
	static LAST_PANIC: RefCell<Option<Panic>> = const { RefCell::new(None) };
}

pub(crate) fn panic_hook(panic_info: &PanicHookInfo) {
	let panic_info = encode_panic(panic_info);

	if cfg!(any(debug_assertions, feature = "rel-debugging")) {
		use std::{
			fs::File,
			io::{BufWriter, Write},
			time::{SystemTime, UNIX_EPOCH},
		};

		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map(|timestamp| timestamp.as_secs())
			.unwrap_or(0);
		let filename = PANIC_OUTPUT_FOLDER
			.read()
			.join(format!("meowtonin-panic-{timestamp}.json"));

		// simplify code here by just ignoring errors,
		// as we MUST avoid panicking if possible.
		#[allow(unused_must_use)]
		if let Ok(mut file) = File::create(filename).map(BufWriter::new) {
			serde_json::to_writer_pretty(&mut file, &panic_info);
			file.flush();
			if let Ok(file) = file.into_inner() {
				file.sync_all();
			}
		}
	}

	LAST_PANIC.set(Some(panic_info));
}

#[doc(hidden)]
pub fn stack_trace_if_panic() -> bool {
	match LAST_PANIC.take() {
		Some(last_panic) => {
			let panic_json = serde_json::to_string(&last_panic)
				.map(ByondValue::new_string)
				.unwrap_or_default();
			let message = match last_panic.message {
				Some(message) => ByondValue::new_string(message.as_ref()),
				None => ByondValue::NULL,
			};
			let (file, line) = match last_panic.location {
				Some(loc) => (
					ByondValue::new_string(loc.file),
					ByondValue::new_num(loc.line as f32),
				),
				None => (ByondValue::NULL, ByondValue::new_num(0.0)),
			};
			let _ = crate::call_global::<_, _, _, ()>("meowtonin_stack_trace", [
				message, file, line, panic_json,
			]);
			true
		}
		None => false,
	}
}

thread_local! {
	static CRASH_REASON: RefCell<CString> = RefCell::new(CString::default());
}

#[doc(hidden)]
pub fn byond_crash(reason: String) -> ! {
	let reason = CString::new(reason).unwrap_or_else(|error| {
		let safe_len = error.nul_position();
		let mut reason = error.into_vec();
		reason.truncate(safe_len);
		CString::new(reason).unwrap_or_default()
	});
	let reason_ptr = CRASH_REASON.with_borrow_mut(move |return_string| {
		*return_string = reason;
		return_string.as_ptr()
	});
	unsafe {
		byond().Byond_CRASH(reason_ptr); // this does a longjmp - any subsequent code will be UNREACHABLE
		std::hint::unreachable_unchecked()
	}
}
