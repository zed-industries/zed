// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
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
*/

#![no_std]
#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![cfg_attr(not(portable_atomic_no_unsafe_op_in_unsafe_fn), warn(unsafe_op_in_unsafe_fn))] // unsafe_op_in_unsafe_fn requires Rust 1.52
#![cfg_attr(portable_atomic_no_unsafe_op_in_unsafe_fn, allow(unused_unsafe))]
#![warn(
    // Lints that may help when writing public library.
    missing_debug_implementations,
    missing_docs,
    clippy::alloc_instead_of_core,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::impl_trait_in_params,
    // clippy::missing_inline_in_public_items,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
)]
#![allow(clippy::inline_always)]
// docs.rs only (cfg is enabled by docs.rs, not build script)
#![cfg_attr(docsrs, feature(doc_cfg))]
// Enable custom unsized coercions if the user explicitly opts-in to unstable cfg
#![cfg_attr(portable_atomic_unstable_coerce_unsized, feature(coerce_unsized, unsize))]

#[cfg(all(feature = "alloc", not(portable_atomic_no_alloc)))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;
#[cfg(all(feature = "std", portable_atomic_no_alloc))]
extern crate std as alloc;

#[cfg(any(all(feature = "alloc", not(portable_atomic_no_alloc)), feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
mod arc;
#[cfg(any(all(feature = "alloc", not(portable_atomic_no_alloc)), feature = "std"))]
pub use self::arc::{Arc, Weak};

#[cfg(not(portable_atomic_no_futures_api))]
#[cfg(any(all(feature = "alloc", not(portable_atomic_no_alloc)), feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
pub mod task;
