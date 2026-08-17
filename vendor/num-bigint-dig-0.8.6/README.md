# num-bigint-dig

[![crate](https://img.shields.io/crates/v/num-bigint-dig.svg)](https://crates.io/crates/num-bigint-dig)
[![documentation](https://docs.rs/num-bigint-dig/badge.svg)](https://docs.rs/num-bigint-dig)
![minimum rustc 1.56](https://img.shields.io/badge/rustc-1.56+-red.svg)
[![Travis status](https://travis-ci.org/dignifiedquire/num-bigint.svg?branch=master)](https://travis-ci.org/dignifiedquire/num-bigint)

Big integer types for Rust, `BigInt` and `BigUint`.

> **Warning** This is a fork of [`rust-num/num-bigint`](https://github.com/rust-num/num-bigint) with a focus on providing functionality, needed to implement cryptographic operations.


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = "^0.7"
```

and this to your crate root:

```rust
extern crate num_bigint_dig as num_bigint;
```

## Features

The `std` feature is enabled by default and mandatory to compile on older rust
version.

It is possible to use this crate on no_std target. If
you wish to compile for a target that does not have an `std` crate, you should
use `num-bigint` with `default-features = false`. All other sub-features should
be compatible with no_std. Note that in this mode, `num-bigint` still relies on
the alloc crate, so make sure you define a `global_allocator`.

Implementations for `i128` and `u128` are only available with Rust 1.26 and
later.  The build script automatically detects this, but you can make it
mandatory by enabling the `i128` crate feature.

The `u64_digit` feature enables usage of larger internal "digits" (or otherwise known as "limbs"). Speeeding up almost all operations on architectures that have native support for it.

The `prime` feature gate enables algorithms and support for dealing with large primes.

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## Compatibility

The `num-bigint` crate is tested for rustc 1.56 and greater.

## Alternatives

While `num-bigint` strives for good performance in pure Rust code, other
crates may offer better performance with different trade-offs.  The following
table offers a brief comparison to a few alternatives.

| Crate                | License        | Min rustc | Implementation |
| :------------------- | :------------- |:----------| :------------- |
| **`num-bigint-dig`** | MIT/Apache-2.0 | 1.56      | pure rust |
| [`num-bigint`]       | MIT/Apache-2.0 | 1.15      | pure rust |
| [`ramp`]             | Apache-2.0     | nightly   | rust and inline assembly |
| [`rug`]              | LGPL-3.0+      | 1.18      | bundles [GMP] via [`gmp-mpfr-sys`] |
| [`rust-gmp`]         | MIT            | stable?   | links to [GMP] |
| [`apint`]            | MIT/Apache-2.0 | 1.26      | pure rust (unfinished) |

[`num-bigint`]: https://crates.io/crates/num-bigint
[GMP]: https://gmplib.org/
[`gmp-mpfr-sys`]: https://crates.io/crates/gmp-mpfr-sys
[`rug`]: https://crates.io/crates/rug
[`rust-gmp`]: https://crates.io/crates/rust-gmp
[`ramp`]: https://crates.io/crates/ramp
[`apint`]: https://crates.io/crates/apint

## Benchmarks

```
cargo bench --features prime
```
