[![crates.io](https://img.shields.io/crates/v/fallible-iterator.svg)](https://crates.io/crates/fallible-iterator)
[![docs.rs](https://docs.rs/fallible-iterator/badge.svg)](https://docs.rs/fallible-iterator)

![Continuous integration](https://github.com/sfackler/rust-fallible-iterator/actions/workflows/rust.yml/badge.svg?branch=master&event=push)

# rust-fallible-iterator

"Fallible" iterators for Rust.

## Features

If the `std` or `alloc` features are enabled, this crate provides implementations for
`Box`, `Vec`, `BTreeMap`, and `BTreeSet`. If the `std` feature is enabled, this crate
additionally provides implementations for `HashMap` and `HashSet`.

If the `std` feature is disabled, this crate does not depend on `libstd`.
