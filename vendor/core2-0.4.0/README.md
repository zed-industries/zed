# core2

[![Actions Status](https://github.com/bbqsrc/core2/workflows/CI/badge.svg)](https://github.com/bbqsrc/core2/actions)
[![Documentation](https://docs.rs/core2/badge.svg)](https://docs.rs/core2)
![Minimum Supported Rust Version (MSRV)](https://img.shields.io/badge/rust-v1.47.0+-blue)

Ever wanted a `Cursor` or the `Error` trait in `no_std`? Well now you can have it. A 'fork' of Rust's `std` modules for `no_std` environments, with the added benefit of optionally taking advantage of `alloc`.

The goal of this crate is to provide a stable interface for building I/O and error trait functionality in
`no_std` environments. The current code corresponds to the most recent stable API of Rust 1.47.0. 
It is also a goal to achieve a true alloc-less experience, with opt-in alloc support.

This crate works on `stable` with some limitations in functionality, and `nightly` without limitations by adding
the relevant feature flag.

This crate is `std` by default -- use no default features to get `no_std` mode.

## Usage

```toml
[dependencies]
core2 = "0.3"
```

Add the crate, use the things you would usually want from `std::io`, but instead from `core2::io`, and
use `core2::error::Error` in place of `std::error::Error`.

### Features

- **std**: enables `std` pass-throughs for the polyfilled types, but allows accessing the new types
- **alloc**: enable aspects of the `Read` and `Write` traits that require `alloc` support (WIP)
- **nightly**: enables **nightly**-only features, such as `BufReader` and `BufWriter` with const generic buffers.

### Differences to `std::io`

- No `std::io::Error`, so we have our own copy without any `Os` error functions
- `IoSlice` and the `*_vectored` family of functions are not implemented.
- `BufReader` and `BufWriter` have a different signature, as they now use a const generic bounded array for the internal buffer. (Requires **nightly** feature)

Other than items perhaps being entirely missing or certain functions unavailable on some traits, no function signatures have been changed.

### Limitations

- Using the buffer types currently requires **nightly** due to the use of const generics.
- Using `copy` or the buffer types with `std` support currently requires **nightly** due to the `initializer` API.

## Where is it used?

All of the below are works in progress, but should help with demonstrating how to use this crate.

- [thiserror_core2](https://github.com/bbqsrc/thiserror-core2): fork of `thiserror` using the `core2::error::Error` trait.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

Almost all of the code in this repository is a copy of the [Rust language codebase](https://github.com/rust-lang/rust) with minor modifications.

For attributions, see https://thanks.rust-lang.org/.
