[![crates.io](https://img.shields.io/crates/d/embedded-io.svg)](https://crates.io/crates/embedded-io)
[![crates.io](https://img.shields.io/crates/v/embedded-io.svg)](https://crates.io/crates/embedded-io)
[![Documentation](https://docs.rs/embedded-io/badge.svg)](https://docs.rs/embedded-io)

# `embedded-io`

This project is developed and maintained by the [HAL team](https://github.com/rust-embedded/wg#the-hal-team).

Input/Output traits for embedded systems.

Rust's `std::io` traits are not available in `no_std` targets, mainly because `std::io::Error`
requires allocation. This crate contains replacement equivalent traits, usable in `no_std`
targets.

## Differences with `std::io`

- `Error` is an associated type. This allows each implementor to return its own error type,
while avoiding `dyn` or `Box`. This is consistent with how errors are handled in [`embedded-hal`](https://github.com/rust-embedded/embedded-hal/).
- In `std::io`, the `Read`/`Write` traits might be blocking or non-blocking (i.e. returning `WouldBlock` errors) depending on the file descriptor's mode, which is only known at run-time. This allows passing a non-blocking stream to code that expects a blocking
stream, causing unexpected errors. To solve this, `embedded-io` specifies `Read`/`Write` are always blocking, and adds new `ReadReady`/`WriteReady` traits to allow using streams in a non-blocking way.

## Optional Cargo features

- **`std`**: Adds `From` impls to convert to/from `std::io` structs, adds `std::error::Error` impls.
- **`alloc`**: Adds blanket impls for `Box`, adds `Write` impl to `Vec`.
- **`defmt-03`**: Derive `defmt::Format` from `defmt` 0.3 for enums and structs.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.60 and up. It *might*
compile with older versions but that may change in any new patch release.

See [here](../docs/msrv.md) for details on how the MSRV may be upgraded.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
