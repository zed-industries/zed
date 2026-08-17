rand
====

A Rust library for random number generators and other randomness functionality.

[![Build Status](https://travis-ci.org/rust-lang-nursery/rand.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/rand)
[![Build status](https://ci.appveyor.com/api/projects/status/rm5c9o33k3jhchbw?svg=true)](https://ci.appveyor.com/project/alexcrichton/rand)

[Documentation](https://docs.rs/rand)

## Compatibility upgrade

Version 0.3 has been replaced by a compatibility wrapper around `rand` 0.4. It
is recommended to update to 0.4.

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
