[![crates.io](https://img.shields.io/crates/v/heapless.svg)](https://crates.io/crates/heapless)
[![crates.io](https://img.shields.io/crates/d/heapless.svg)](https://crates.io/crates/heapless)

# `heapless`

> `static` friendly data structures that don't require dynamic memory allocation

This project is developed and maintained by the [libs team].

## [Documentation](https://docs.rs/heapless/latest/heapless)

## [Change log](CHANGELOG.md)

## Tests

``` console
$ # run all
$ cargo test --features serde

$ # run only for example histbuf tests
$ cargo test histbuf --features serde
```

# Formatting

Like most Rust projects, we use `rustfmt` to keep the formatting of code consistent. However, we
make use of cecertain options that are currently only available in the nightly version:

```console
$ cargo +nightly fmt --all
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## MSRV Policy

This crate is guaranteed to compile with the latest two stable releases of Rust. For example, if the
latest stable Rust release is 1.70, then this crate is guaranteed to compile with Rust 1.69 and 1.70.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[libs team]: https://github.com/rust-embedded/wg#the-libs-team
