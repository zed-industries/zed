<div align="center">
  <h1>bit-vec</h1>
  <p>
    <strong>A compact vector of bits.</strong>
  </p>
  <p>

[![crates.io][crates.io shield]][crates.io link]
[![Documentation][docs.rs badge]][docs.rs link]
![Rust CI][github ci badge]
![MSRV][rustc 1.82+]
<br />
<br />
[![Dependency Status][deps.rs status]][deps.rs link]
[![Download Status][shields.io download count]][crates.io link]

  </p>
</div>

[crates.io shield]: https://img.shields.io/crates/v/bit-vec?label=latest
[crates.io link]: https://crates.io/crates/bit-vec
[docs.rs badge]: https://docs.rs/bit-vec/badge.svg?version=0.9.1
[docs.rs link]: https://docs.rs/bit-vec/0.9.1/bit_vec/
[github ci badge]: https://github.com/contain-rs/bit-vec/actions/workflows/rust.yml/badge.svg
[rustc 1.82+]: https://img.shields.io/badge/rustc-1.82%2B-blue.svg
[deps.rs status]: https://deps.rs/crate/bit-vec/0.9.1/status.svg
[deps.rs link]: https://deps.rs/crate/bit-vec/0.9.1
[shields.io download count]: https://img.shields.io/crates/d/bit-vec.svg

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
bit-vec = "0.9"
```

If you want [serde](https://github.com/serde-rs/serde) support, include the feature like this:

```toml
[dependencies]
bit-vec = { version = "0.9", features = ["serde"] }
```

If you want to use bit-vec in a program that has `#![no_std]`, just drop default features:

```toml
[dependencies]
bit-vec = { version = "0.9", default-features = false }
```

If you want to use serde with the alloc crate instead of std, just use the `serde_no_std` feature:

```toml
[dependencies]
bit-vec = { version = "0.9", default-features = false, features = ["serde", "serde_no_std"] }
```

If you want [borsh-rs](https://github.com/near/borsh-rs) support, include it like this:

```toml
[dependencies]
bit-vec = { version = "0.9", features = ["borsh"] }
```

Other available serialization libraries can be enabled with the
[`miniserde`](https://github.com/dtolnay/miniserde) and
[`nanoserde`](https://github.com/not-fl3/nanoserde) features.

<!-- cargo-rdme start -->

### Description

Dynamic collections implemented with compact bit vectors.

### Examples

This is a simple example of the [Sieve of Eratosthenes][sieve]
which calculates prime numbers up to a given limit.

[sieve]: http://en.wikipedia.org/wiki/Sieve_of_Eratosthenes

```rust
use bit_vec::BitVec;

let max_prime = 10000;

// Store the primes as a BitVec
let primes = {
    // Assume all numbers are prime to begin, and then we
    // cross off non-primes progressively
    let mut bv = BitVec::from_elem(max_prime, true);

    // Neither 0 nor 1 are prime
    bv.set(0, false);
    bv.set(1, false);

    for i in 2.. 1 + (max_prime as f64).sqrt() as usize {
        // if i is a prime
        if bv[i] {
            // Mark all multiples of i as non-prime (any multiples below i * i
            // will have been marked as non-prime previously)
            for j in i.. {
                if i * j >= max_prime {
                    break;
                }
                bv.set(i * j, false)
            }
        }
    }
    bv
};

// Simple primality tests below our max bound
let print_primes = 20;
print!("The primes below {} are: ", print_primes);
for x in 0..print_primes {
    if primes.get(x).unwrap_or(false) {
        print!("{} ", x);
    }
}
println!();

let num_primes = primes.iter().filter(|x| *x).count();
println!("There are {} primes below {}", num_primes, max_prime);
assert_eq!(num_primes, 1_229);
```

<!-- cargo-rdme end -->

## License

Dual-licensed for compatibility with the Rust project.

Licensed under the Apache License Version 2.0: http://www.apache.org/licenses/LICENSE-2.0,
or the MIT license: http://opensource.org/licenses/MIT, at your option.
