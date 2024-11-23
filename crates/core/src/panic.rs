// SPDX-License-Identifier: 0BSD
use aho_corasick::AhoCorasick;
use backtrace::BacktraceFrame;
use std::{path::Path, sync::LazyLock};

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

fn is_relevant_frame(frame: &BacktraceFrame) -> bool {
	frame.symbols().iter().any(|sym| match sym.name() {
		Some(name) => {
			let name = name.to_string();
			name != "main"
				&& !INTERNAL_PATTERNS.is_match(&name)
				&& !sym
					.filename()
					.map(Path::to_string_lossy)
					.is_some_and(|file| PATH_PATTERNS.is_match(file.as_ref()))
		}
		None => false,
	})
}
