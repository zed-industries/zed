# Nano ID

[![Package version](https://img.shields.io/crates/v/nanoid.svg)](https://crates.io/crates/nanoid)
[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/nikolay-govorov/nanoid/blob/master/LICENSE)
[![Travis build status ](https://travis-ci.org/nikolay-govorov/nanoid.svg?branch=master)](https://travis-ci.org/nikolay-govorov/nanoid)
[![Appveyor build status](https://ci.appveyor.com/api/projects/status/github/nikolay-govorov/nanoid?svg=true&amp;branch=master)](https://ci.appveyor.com/project/nikolay-govorov/nanoid)
![Maintenance intention for this crate](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

A tiny, secure, URL-friendly, unique string ID generator for Rust

```rust
use nanoid::nanoid;

fn main() {
   let id = nanoid!(); //=> "Yo1Tr9F3iF-LFHX9i9GvA"
}
```

**Safe.** It uses cryptographically strong random APIs
and guarantees a proper distribution of symbols.

**Compact.** It uses a larger alphabet than UUID (`A-Za-z0-9_-`)
and has a similar number of unique IDs in just 21 symbols instead of 36.

## Usage

### Install

```toml
[dependencies]
nanoid = "0.4.0"
```

### Simple

The main module uses URL-friendly symbols (`A-Za-z0-9_-`) and returns an ID
with 21 characters.

```rust
use nanoid::nanoid;

fn main() {
   let id = nanoid!(); //=> "Yo1Tr9F3iF-LFHX9i9GvA"
}
```

Symbols `-,.()` are not encoded in the URL. If used at the end of a link
they could be identified as a punctuation symbol.

### Custom length

If you want to reduce ID length (and increase collisions probability),
you can pass the length as an argument generate function:

```rust
use nanoid::nanoid;

fn main() {
   let id = nanoid!(10); //=> "IRFa-VaY2b"
}
```

### Custom Alphabet or Length

If you want to change the ID's alphabet or length, you can simply pass the
custom alphabet to the `nanoid!()` macro as the second parameter:

```rust
use nanoid::nanoid;

fn main() {
    let alphabet: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
    ];

   let id = nanoid!(10, &alphabet); //=> "4f90d13a42"
}
```

Alphabet must contain 256 symbols or less.
Otherwise, the generator will not be secure.

### Custom Random Bytes Generator

You can replace the default safe random generator using the `complex` module.
For instance, to use a seed-based generator.

```rust
use nanoid::nanoid;

fn random_byte () -> u8 { 0 }

fn main() {
    fn random (size: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0; size];

        for i in 0..size {
            bytes[i] = random_byte();
        }

        bytes
    }

    nanoid!(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
}
```

`random` function must accept the array size and return an vector
with random numbers.

If you want to use the same URL-friendly symbols with `format`,
you can get the default alphabet from the `url` module:

```rust
use nanoid::nanoid;

fn random (size: usize) -> Vec<u8> {
    let result: Vec<u8> = vec![0; size];

    result
}

fn main() {
    nanoid!(10, &nanoid::alphabet::SAFE, random); //=> "93ce_Ltuub"
}
```

## Other Programming Languages

* [JS](https://github.com/ai/nanoid)
* [C#](https://github.com/codeyu/nanoid-net)
* [Crystal](https://github.com/mamantoha/nanoid.cr)
* [Go](https://github.com/matoous/go-nanoid)
* [Elixir](https://github.com/railsmechanic/nanoid)
* [Haskell](https://github.com/4e6/nanoid-hs)
* [Java](https://github.com/aventrix/jnanoid)
* [Nim](https://github.com/icyphox/nanoid.nim)
* [PHP](https://github.com/hidehalo/nanoid-php)
* [Python](https://github.com/puyuan/py-nanoid)
* [Ruby](https://github.com/radeno/nanoid.rb)
