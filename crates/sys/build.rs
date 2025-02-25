// SPDX-License-Identifier: 0BSD
use bindgen::{callbacks::ParseCallbacks, Abi};
use std::path::PathBuf;

fn link() {
	let link_dir = std::env::var("CARGO_MANIFEST_DIR")
		.map(PathBuf::from)
		.expect("CARGO_MANIFEST_DIR not set")
		.join("link");
	let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set");
	let target_env = std::env::var("CARGO_CFG_TARGET_ENV").expect("CARGO_CFG_TARGET_ENV not set");

	match (target_os.as_str(), target_env.as_str()) {
		("windows", "msvc") => {
			println!(
				"cargo:rustc-link-search=native={}",
				link_dir.join("windows").display()
			);
			println!("cargo:rustc-link-lib=dylib=byondapi");
		}
		("linux", _) => {
			println!(
				"cargo:rustc-link-search=native={}",
				link_dir.join("linux").display()
			);
			println!("cargo:rustc-link-lib=dylib=byond");
		}
		_ => panic!("Unsupported platform"),
	}
}

fn generate_bindings() {
	println!("cargo:rerun-if-changed=bindings/byondapi.h");
	println!("cargo:rerun-if-changed=bindings/wrapper.hpp");
	let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not defined"));
	let bindings = bindgen::Builder::default()
		.header("bindings/wrapper.hpp")
		.allowlist_item("C?Byond.*")
		.allowlist_item("[su][1-9].*")
		.override_abi(Abi::CUnwind, "Byond.*")
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

fn main() {
	generate_bindings();
	link();
}

#[derive(Debug)]
struct DoxygenCallbacks;

impl ParseCallbacks for DoxygenCallbacks {
	fn process_comment(&self, comment: &str) -> Option<String> {
		Some(doxygen_rs::transform(comment))
	}
}
