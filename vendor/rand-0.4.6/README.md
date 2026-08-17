rand
====

A Rust library for random number generators and other randomness functionality.

[![Build Status](https://travis-ci.org/rust-lang-nursery/rand.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/rand)
[![Build status](https://ci.appveyor.com/api/projects/status/rm5c9o33k3jhchbw?svg=true)](https://ci.appveyor.com/project/alexcrichton/rand)

[Documentation](https://docs.rs/rand)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.4"
```

and this to your crate root:

```rust
extern crate rand;
```

### Versions

Version `0.4`was released in December 2017. It contains almost no breaking
changes since the `0.3` series, but nevertheless contains some significant
new code, including a new "external" entropy source (`JitterRng`) and `no_std`
support.

Version `0.5` is in development and contains significant performance
improvements for the ISAAC random number generators.

## Examples

There is built-in support for a random number generator (RNG) associated with each thread stored in thread-local storage. This RNG can be accessed via thread_rng, or used implicitly via random. This RNG is normally randomly seeded from an operating-system source of randomness, e.g. /dev/urandom on Unix systems, and will automatically reseed itself from this source after generating 32 KiB of random data.

```rust
let tuple = rand::random::<(f64, char)>();
println!("{:?}", tuple)
```

```rust
use rand::Rng;

let mut rng = rand::thread_rng();
if rng.gen() { // random bool
    println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
}
```

It is also possible to use other RNG types, which have a similar interface. The following uses the "ChaCha" algorithm instead of the default.

```rust
use rand::{Rng, ChaChaRng};

let mut rng = rand::ChaChaRng::new_unseeded();
println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
```

## Features

By default, `rand` is built with all stable features available. The following
optional features are available:

-   `i128_support` enables support for generating `u128` and `i128` values
-   `nightly` enables all unstable features (`i128_support`)
-   `std` enabled by default; by setting "default-features = false" `no_std`
    mode is activated; this removes features depending on `std` functionality:
    
        -   `OsRng` is entirely unavailable
        -   `JitterRng` code is still present, but a nanosecond timer must be
            provided via `JitterRng::new_with_timer`
        -   Since no external entropy is available, it is not possible to create
            generators with fresh seeds (user must provide entropy)
        -   `thread_rng`, `weak_rng` and `random` are all disabled
        -   exponential, normal and gamma type distributions are unavailable
            since `exp` and `log` functions are not provided in `core`
        -   any code requiring `Vec` or `Box`
-   `alloc` can be used instead of `std` to provide `Vec` and `Box`

## Testing

Unfortunately, `cargo test` does not test everything. The following tests are
recommended:

```
# Basic tests for rand and sub-crates
cargo test --all

# Test no_std support (build only since nearly all tests require std)
cargo build --all --no-default-features

# Test 128-bit support (requires nightly)
cargo test --all --features nightly

# Benchmarks (requires nightly)
cargo bench
# or just to test the benchmark code:
cargo test --benches
```

# `derive(Rand)`

You can derive the `Rand` trait for your custom type via the `#[derive(Rand)]`
directive. To use this first add this to your Cargo.toml:

```toml
rand = "0.4"
rand_derive = "0.3"
```

Next in your crate:

```rust
extern crate rand;
#[macro_use]
extern crate rand_derive;

#[derive(Rand, Debug)]
struct MyStruct {
    a: i32,
    b: u32,
}

fn main() {
    println!("{:?}", rand::random::<MyStruct>());
}
```


# License

`rand` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.
