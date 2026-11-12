// SPDX-License-Identifier: 0BSD
use meowtonin::{ByondValue, bindings::generate_dm, byond_fn};

/// Adds two numbers.
///
/// Cats only.
#[byond_fn(name = "test_lib_add")]
fn add(
	/// the first number
	a: f32,
	/// the second number
	b: f32,
) -> f32 {
	a + b
}

#[byond_fn(variadic)]
fn log_all(args: Vec<ByondValue>) {
	let _ = args;
}

#[byond_fn]
fn ping() {}

/// Says hi.
#[byond_fn]
fn greet(
	name: String,
	/// what to say
	greeting: Option<String>,
	mark: Option<String>,
) -> String {
	format!("{greeting:?} {name}{mark:?}")
}

#[byond_fn]
fn ignore_first(_: f32, mut count: f32) -> f32 {
	count += 1.0;
	count
}

#[test]
fn generated_dm_matches() {
	assert_eq!(
		generate_dm("test_lib"),
		include_str!("bindings.expected.dm")
	);
}
