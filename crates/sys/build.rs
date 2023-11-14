// SPDX-License-Identifier: 0BSD
use bindgen::callbacks::ParseCallbacks;
use std::path::PathBuf;

fn main() {
	println!("cargo:rerun-if-changed=bindings/byondapi.h");
	println!("cargo:rerun-if-changed=bindings/wrapper.hpp");
	let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not defined"));
	let bindings = bindgen::Builder::default()
		.header("bindings/wrapper.hpp")
		.dynamic_library_name("ByondApi")
		.dynamic_link_require_all(true)
		.allowlist_item("C?Byond.*")
		.allowlist_item("[su][1-9].*")
		.generate_block(true)
		.derive_default(true)
		.derive_debug(true)
		.derive_copy(true)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.parse_callbacks(Box::new(DoxygenCallbacks))
		.generate()
		.expect("failed to generate byondapi bindings");
	bindings
		.write_to_file(out_dir.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}

#[derive(Debug)]
struct DoxygenCallbacks;

impl ParseCallbacks for DoxygenCallbacks {
	fn process_comment(&self, comment: &str) -> Option<String> {
		Some(doxygen_rs::transform(comment))
	}
}
