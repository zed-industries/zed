# The `libfuzzer-sys` Crate

Barebones wrapper around LLVM's libFuzzer runtime library.

The CPP parts are extracted from compiler-rt git repository with `git filter-branch`.

libFuzzer relies on LLVM sanitizer support. The Rust compiler has built-in support for LLVM sanitizer support, for now, it's limited to Linux. As a result, `libfuzzer-sys` only works on Linux.

## Usage

### Use `cargo fuzz`!

[The recommended way to use this crate with `cargo fuzz`!][cargo-fuzz].

[cargo-fuzz]: https://github.com/rust-fuzz/cargo-fuzz

### Manual Usage

This crate can also be used manually as following:

First create a new cargo project:

```
$ cargo new --bin fuzzed
$ cd fuzzed
```

Then add a dependency on the `fuzzer-sys` crate and your own crate:

```toml
[dependencies]
libfuzzer-sys = "0.4.0"
your_crate = { path = "../path/to/your/crate" }
```

Change the `fuzzed/src/main.rs` to fuzz your code:

```rust
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // code to fuzz goes here
});
```

Build by running the following command:

```sh
$ cargo rustc -- \
    -C passes='sancov-module' \
    -C llvm-args='-sanitizer-coverage-level=3' \
    -C llvm-args='-sanitizer-coverage-inline-8bit-counters' \
    -Z sanitizer=address
```

And finally, run the fuzzer:

```sh
$ ./target/debug/fuzzed
```

### Linking to a local libfuzzer

When using `libfuzzer-sys`, you can provide your own `libfuzzer` runtime in two ways.

If you are developing a fuzzer, you can set the `CUSTOM_LIBFUZZER_PATH` environment variable to the path of your local
`libfuzzer` runtime, which will then be linked instead of building libfuzzer as part of the build stage of `libfuzzer-sys`.
For an example, to link to a prebuilt LLVM 16 `libfuzzer`, you could use:

```bash
$ export CUSTOM_LIBFUZZER_PATH=/usr/lib64/clang/16/lib/libclang_rt.fuzzer-x86_64.a
$ cargo fuzz run ...
```

Alternatively, you may also disable the default `link_libfuzzer` feature:

In `Cargo.toml`:
```toml
[dependencies]
libfuzzer-sys = { path = "../../libfuzzer", default-features = false }
```

Then link to your own runtime in your `build.rs`.

## Updating libfuzzer from upstream

* Update the `COMMIT=...` variable in `./update-libfuzzer.sh` with the new
  commit hash from [llvm-mirror/llvm-project](github.com/llvm-mirror/llvm-project)
  that you are vendoring.

* Re-run the script:

  ```
  $ ./update-libfuzzer.sh <github.com/llvm-mirror/llvm-project SHA1>
  ```

## License

All files in the `libfuzzer` directory are licensed NCSA.

Everything else is dual-licensed Apache 2.0 and MIT.
