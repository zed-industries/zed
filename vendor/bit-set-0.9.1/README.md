<div align="center">
  <h1>bit-set</h1>
  <p>
    <strong>A compact set of bits.</strong>
  </p>
  <p>

[![crates.io][crates.io shield]][crates.io link]
[![Documentation][docs.rs badge]][docs.rs link]
![Rust CI][github ci badge]
![rustc 1.82+]
<br />
<br />
[![Dependency Status][deps.rs status]][deps.rs link]
[![Download Status][shields.io download count]][crates.io link]

  </p>
</div>

[crates.io shield]: https://img.shields.io/crates/v/bit-set?label=latest
[crates.io link]: https://crates.io/crates/bit-set
[docs.rs badge]: https://docs.rs/bit-set/badge.svg?version=0.9.1
[docs.rs link]: https://docs.rs/bit-set/0.9.1/bit_set/
[github ci badge]: https://github.com/contain-rs/bit-set/actions/workflows/rust.yml/badge.svg
[rustc 1.82+]: https://img.shields.io/badge/rustc-1.82%2B-blue.svg
[deps.rs status]: https://deps.rs/crate/bit-set/0.9.1/status.svg
[deps.rs link]: https://deps.rs/crate/bit-set/0.9.1
[shields.io download count]: https://img.shields.io/crates/d/bit-set.svg

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
bit-set = "0.9"
```

Since Rust 2018, `extern crate` is no longer mandatory. If your edition is old (Rust 2015),
add this to your crate root:

```rust
extern crate bit_set;
```

If you want to use `serde`, enable it with the `serde` feature:

```toml
[dependencies]
bit-set = { version = "0.9", features = ["serde"] }
```

If you want to use bit-set in a program that has `#![no_std]`, just drop default features:

```toml
[dependencies]
bit-set = { version = "0.9", default-features = false }
```

If you want to use serde with the alloc crate instead of std, just use the `serde_no_std` feature:

```toml
[dependencies]
bit-set = { version = "0.9", default-features = false, features = ["serde", "serde_no_std"] }
```

If you want [borsh-rs](https://github.com/near/borsh-rs) support, include it like this:

```toml
[dependencies]
bit-set = { version = "0.9", features = ["borsh"] }
```

Other available serialization libraries can be enabled with the
[`miniserde`](https://github.com/dtolnay/miniserde) and
[`nanoserde`](https://github.com/not-fl3/nanoserde) features. (nanoserde is temporarily unavailable!)


<!-- cargo-rdme start -->

### Description

An implementation of a set using a bit vector as an underlying
representation for holding unsigned numerical elements.

It should also be noted that the amount of storage necessary for holding a
set of objects is proportional to the maximum of the objects when viewed
as a `usize`.

### Examples

```rust
use bit_set::BitSet;

// It's a regular set
let mut s = BitSet::new();
s.insert(0);
s.insert(3);
s.insert(7);

s.remove(7);

if !s.contains(7) {
    println!("There is no 7");
}

// Can initialize from a `BitVec`
let other = BitSet::from_bytes(&[0b11010000]);

s.union_with(&other);

// Print 0, 1, 3 in some order
for x in s.iter() {
    println!("{}", x);
}

// Can convert back to a `BitVec`
let bv = s.into_bit_vec();
assert!(bv[3]);
```

<!-- cargo-rdme end -->

## License

Dual-licensed for compatibility with the Rust project.

Licensed under the Apache License Version 2.0: http://www.apache.org/licenses/LICENSE-2.0,
or the MIT license: http://opensource.org/licenses/MIT, at your option.
