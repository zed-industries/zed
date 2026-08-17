Generic `Atomic<T>` for Rust
============================

[![Build Status](https://travis-ci.org/Amanieu/atomic-rs.svg?branch=master)](https://travis-ci.org/Amanieu/atomic-rs) [![Crates.io](https://img.shields.io/crates/v/atomic.svg)](https://crates.io/crates/atomic)

A Rust library which provides a generic `Atomic<T>` type for all `T: Copy` types, unlike the standard library which only provides a few fixed atomic types (`AtomicBool`, `AtomicIsize`, `AtomicUsize`, `AtomicPtr`).

This library will use native atomic instructions if possible, and will otherwise fall back to a lock-based mechanism. You can use the `Atomic::<T>::is_lock_free()` function to check whether native atomic operations are supported for a given type. Note that a type must have a power-of-2 size and alignment in order to be used by native atomic instructions.

This crate uses `#![no_std]` and only depends on libcore.

[Documentation](https://amanieu.github.io/atomic-rs/atomic/index.html)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
atomic = "0.5"
```

and this to your crate root:

```rust
extern crate atomic;
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
