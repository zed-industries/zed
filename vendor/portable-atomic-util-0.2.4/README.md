# portable-atomic-util

[![crates.io](https://img.shields.io/crates/v/portable-atomic-util?style=flat-square&logo=rust)](https://crates.io/crates/portable-atomic-util)
[![docs.rs](https://img.shields.io/badge/docs.rs-portable--atomic--util-blue?style=flat-square&logo=docs.rs)](https://docs.rs/portable-atomic-util)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.34-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/portable-atomic/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/portable-atomic/actions)
[![cirrus ci](https://img.shields.io/cirrus/github/taiki-e/portable-atomic/main?style=flat-square&logo=cirrusci)](https://cirrus-ci.com/github/taiki-e/portable-atomic)

<!-- tidy:crate-doc:start -->
Synchronization primitives built with [portable-atomic].

- Provide `Arc`. (optional, requires the `std` or `alloc` feature)
- Provide `task::Wake`. (optional, requires the `std` or `alloc` feature)
<!-- - Provide generic `Atomic<T>` type. (optional, requires the `generic` feature) -->

See [#1] for other primitives being considered for addition to this crate.

## Optional features

- **`std`**<br>
  Use `std`.

  Note:
  - This implicitly enables the `alloc` feature.

- **`alloc`**<br>
  Use `alloc`.

  Note:
  - The MSRV when this feature is enabled and the `std` feature is *not* enabled is Rust 1.36 that `alloc` crate stabilized.

<!-- TODO: https://github.com/taiki-e/portable-atomic/issues/1
- **`generic`**<br>
  Provides generic `Atomic<T>` type.
-->

[portable-atomic]: https://github.com/taiki-e/portable-atomic
[#1]: https://github.com/taiki-e/portable-atomic/issues/1

## Optional cfg

One of the ways to enable cfg is to set [rustflags in the cargo config](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerustflags):

```toml
# .cargo/config.toml
[target.<target>]
rustflags = ["--cfg", "portable_atomic_unstable_coerce_unsized"]
```

Or set environment variable:

```sh
RUSTFLAGS="--cfg portable_atomic_unstable_coerce_unsized" cargo ...
```

- <a name="optional-cfg-unstable-coerce-unsized"></a>**`--cfg portable_atomic_unstable_coerce_unsized`**<br>
  Support coercing of `Arc<T>` to `Arc<U>` as in `std::sync::Arc`.

  <!-- TODO: add coercing of `Weak<T>` to `Weak<U>` as well, with testing & documentation updates -->

  This cfg requires Rust nightly because this coercing requires [unstable `CoerceUnsized` trait](https://doc.rust-lang.org/nightly/core/ops/trait.CoerceUnsized.html).

  See [this issue comment](https://github.com/taiki-e/portable-atomic/issues/143#issuecomment-1866488569) for another known workaround.

  **Note:** This cfg is unstable and outside of the normal semver guarantees and minor or patch versions of portable-atomic-util may make breaking changes to them at any time.

<!-- tidy:crate-doc:end -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
