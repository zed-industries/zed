# bytecount

Counting bytes really fast

[![Continuous integration](https://github.com/llogiq/bytecount/actions/workflows/ci.yml/badge.svg)](https://github.com/llogiq/bytecount/actions/workflows/ci.yml)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/llogiq/bytecount?svg=true)](https://ci.appveyor.com/project/llogiq/bytecount)
[![Current Version](https://img.shields.io/crates/v/bytecount.svg)](https://crates.io/crates/bytecount)
[![License: Apache 2.0/MIT](https://img.shields.io/crates/l/bytecount.svg)](#license)

This uses the "hyperscreamingcount" algorithm by Joshua Landau to count bytes faster than anything else.
The [newlinebench](https://github.com/llogiq/newlinebench) repository has further benchmarks for old versions of this repository.

To use bytecount in your crate, if you have [cargo-edit](https://github.com/killercup/cargo-edit), just type
`cargo add bytecount` in a terminal with the crate root as the current path. Otherwise you can manually edit your
`Cargo.toml` to add `bytecount = 0.6.9` to your `[dependencies]` section.

In your crate root (`lib.rs` or `main.rs`, depending on if you are writing a
library or application), add `extern crate bytecount;`. Now you can simply use
`bytecount::count` as follows:

```Rust
extern crate bytecount;

fn main() {
    let mytext = "some potentially large text, perhaps read from disk?";
    let spaces = bytecount::count(mytext.as_bytes(), b' ');
    ..
}
```

bytecount supports two features to make use of modern CPU's features to speed up counting considerably. To allow your
users to use them, add the following to your `Cargo.toml`:

```
[features]
runtime-dispatch-simd = ["bytecount/runtime-dispatch-simd"]
generic-simd = ["bytecount/generic-simd"]
```

The first, `runtime-dispatch-simd`, enables detection of SIMD capabilities at runtime, which allows using the SSE2 and
AVX2 codepaths, but cannot be used with `no_std`.

Your users can then compile with runtime dispatch using:

```
cargo build --release --features runtime-dispatch-simd
```

The second, `generic-simd`, uses [`std::simd`](https://doc.rust-lang.org/std/simd/index.html) and [`#![feature(portable_simd)]`](https://github.com/rust-lang/rust/issues/86656) to provide a fast
architecture-agnostic SIMD codepath, but requires running on nightly.

Your users can compile with this codepath using:

```
cargo build --release --features generic-simd
```

Building for a more specific architecture will also improve performance.
You can do this with

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

The scalar algorithm is explained in depth [here](https://llogiq.github.io/2016/09/27/count.html).

**Note: Versions until 0.4.0 worked with Rust as of 1.20.0. Version 0.5.0 until 0.6.0 requires Rust 1.26 or later,
and at least 1.27.2 to use SIMD. Versions from 0.6.0 require Rust 1.32.0 or later.**

## License

Licensed under either of at your discretion:

- [Apache 2.0](LICENSE.Apache2)
- [MIT](LICENSE.MIT)
