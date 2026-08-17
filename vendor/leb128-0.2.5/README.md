# `leb128`

[![](https://img.shields.io/crates/v/leb128.svg) ![](https://img.shields.io/crates/d/leb128.png)](https://crates.io/crates/leb128) [![Build Status](https://travis-ci.org/gimli-rs/leb128.png?branch=master)](https://travis-ci.org/gimli-rs/leb128) [![Coverage Status](https://coveralls.io/repos/github/gimli-rs/leb128/badge.svg?branch=master)](https://coveralls.io/github/gimli-rs/leb128?branch=master)

Read and write DWARF's "Little Endian Base 128" (LEB128) variable length integer
encoding.

The implementation is a direct translation of the pseudocode in the DWARF 4
standard's appendix C.

## Install

Either

    $ cargo add leb128

or add this to your `Cargo.toml`:

    [dependencies]
    leb128 = "0.2.1"

## Example

```rust
use leb128;

let mut buf = [0; 1024];

// Write to anything that implements `std::io::Write`.
{
    let mut writable = &mut buf[..];
    leb128::write::signed(&mut writable, -12345).expect("Should write number");
}

// Read from anything that implements `std::io::Read`.
let mut readable = &buf[..];
let val = leb128::read::signed(&mut readable).expect("Should read number");
assert_eq!(val, -12345);
```

## Documentation

[Documentation](https://gimli-rs.github.io/leb128/leb128/index.html)

## Read-Eval-Print-Loop for LEB128

This crate comes with a `leb128-repl` program that you can use after `cargo
install leb128` or by running `cargo run` in clone of this repository.

```
$ leb128-repl
LEB128 Read-Eval-Print-Loop!

Converts numbers to signed and unsigned LEB128 and displays the results in
base-10, hex, and binary.

> 42
# unsigned LEB128
[42]
[2a]
[00101010]

# signed LEB128
[42]
[2a]
[00101010]

> -42
# unsigned LEB128
error

# signed LEB128
[86]
[56]
[01010110]

> 9001
# unsigned LEB128
[169, 70]
[a9, 46]
[10101001, 01000110]

# signed LEB128
[169, 198, 0]
[a9, c6, 0]
[10101001, 11000110, 00000000]

> -9001
# unsigned LEB128
error

# signed LEB128
[215, 185, 127]
[d7, b9, 7f]
[11010111, 10111001, 01111111]

>
```

## License

Licensed under either of

  * Apache License, Version 2.0 ([`LICENSE-APACHE`](./LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([`LICENSE-MIT`](./LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
