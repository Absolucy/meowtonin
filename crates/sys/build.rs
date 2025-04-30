// SPDX-License-Identifier: 0BSD

#[cfg(feature = "generate-bindings")]
mod inner {
	use bindgen::{Abi, callbacks::ParseCallbacks};

	pub fn run() {
		println!("cargo:rerun-if-changed=bindings/byondapi.h");
		println!("cargo:rerun-if-changed=bindings/wrapper.hpp");
		let bindings = bindgen::Builder::default()
			.header("bindings/wrapper.hpp")
			.dynamic_library_name("ByondApi")
			.dynamic_link_require_all(true)
			.allowlist_item("C?Byond.*")
			.allowlist_item("[su][1-9].*")
			.override_abi(Abi::CUnwind, "Byond.*")
			.generate_block(true)
			.derive_default(true)
			.derive_debug(true)
			.derive_copy(true)
			.layout_tests(false)
			.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
			.parse_callbacks(Box::new(DoxygenCallbacks))
			.generate()
			.expect("failed to generate byondapi bindings");
		bindings
			.write_to_file("src/bindings.rs")
			.expect("Couldn't write bindings!");
	}

	#[derive(Debug)]
	struct DoxygenCallbacks;

	impl ParseCallbacks for DoxygenCallbacks {
		fn process_comment(&self, comment: &str) -> Option<String> {
			Some(doxygen_rs::transform(comment))
		}
	}
}

#[cfg(not(feature = "generate-bindings"))]
mod inner {
	pub fn run() {}
}

fn main() {
	inner::run();
}
