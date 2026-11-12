# meowtonin

![Meowtonin Logo](.github/assets/meowtonin-logo.png)

**Meowtonin** is a safe Rust crate for interacting with the BYOND API, present in the latest versions of BYOND 516.

## Setup

BYOND is 32-bit, so build for `i686`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
meowtonin = "0.3"
```

```sh
cargo build --release --target i686-pc-windows-msvc
cargo build --release --target i686-unknown-linux-gnu
```

don't set `panic = "abort"`. panics become DM runtime errors, but abort kills Dream Daemon.

## Usage

slap `#[byond_fn]` on a function, call it from DM:

```rust
use meowtonin::byond_fn;

#[byond_fn]
fn shout(text: String) -> String {
    text.to_uppercase()
}
```

```dm
world.log << call_ext("mylib", "byond:shout")("hello")
```

args and return values convert for you:
- numbers, `bool`, `String`
- `Option<T>` (null is `None`)
- `Vec`, `HashMap` etc. for lists

BYOND numbers are `f32`, so big integers lose precision. return a `Result` and an `Err` becomes a DM runtime error.

variadic? take one `Vec<ByondValue>`:

```rust
use meowtonin::{ByondValue, byond_fn};

#[byond_fn(variadic)]
fn count_args(args: Vec<ByondValue>) -> usize {
    args.len()
}
```

### Working with BYOND values

`ByondValue` is any BYOND value. take one to poke at objects and lists directly:

```rust
use meowtonin::{ByondError, ByondValue, byond_fn};

#[byond_fn]
fn rename(mut target: ByondValue, new_name: String) -> Result<String, ByondError> {
    let old: String = target.read_var("name")?;
    target.write_var("name", new_name)?;
    target.call::<_, _, _, ()>("on_renamed", [old.as_str()])?;
    Ok(old)
}

#[byond_fn]
fn sum_list(list: ByondValue) -> Result<f32, ByondError> {
    list.values()?.map(|value| value.get_number()).sum()
}
```

global procs: `meowtonin::call_global("proc_name", [args])`. all of this only works inside Dream Daemon.

run something when the library loads:

```rust
fn on_load() {}

meowtonin::inventory::submit! { meowtonin::init::InitFunc(on_load) }
```

### Generating the DM file

doc comments (on the function and on each param) carry over to a generated DM file:

```rust
use meowtonin::byond_fn;

/// Adds two numbers.
#[byond_fn(name = "mylib_add")]
fn add(
    /// the first number
    a: f32,
    /// the second number
    b: f32,
) -> f32 {
    a + b
}

#[test]
fn generate_dm() {
    std::fs::write("mylib.dm", meowtonin::bindings::generate_dm("mylib")).unwrap();
}
```

`cargo test` writes `mylib.dm`, one `#define` per function:

```dm
/// Adds two numbers.
///
/// Arguments:
/// * a - the first number
/// * b - the second number
///
/// Rust: `fn add(a: f32, b: f32) -> f32`
#define mylib_add(a, b) call_ext(__mylib_add ||= load_ext(MYLIB, "byond:add"))(a, b)
var/static/__mylib_add
```

- defines use the rust function name, so add a prefix (or use `name`). otherwise an `add(...)` elsewhere in your code gets rewritten.
- trailing `Option` params can be left off the call.
- define `MYLIB` before including the file to change where the library loads from.

## Credits

- Logo by [EuphoriaEnby](https://twitter.com/EuphoriaEnby)

## License: BSD Zero Clause License (SPDX: 0BSD)

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
