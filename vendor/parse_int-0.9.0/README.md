# parse_int

[![crates.io](https://img.shields.io/crates/v/parse_int?logo=rust)](https://crates.io/crates/parse_int/)
[![CI pipeline](https://gitlab.com/dns2utf8/parse_int/badges/main/pipeline.svg)](https://gitlab.com/dns2utf8/parse_int/)

Parse `&str` with common prefixes to integer and float values:

```rust
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {
// Detect the type automatically
let d = parse_int::parse("42")?;
assert_eq!(42_usize, d);

let pi = parse_int::parse("3.141_592_653_589_793").expect("floats have a different error type");
assert_eq!(std::f64::consts::PI, pi);

// import the function for multiple uses
use parse_int::parse;
assert_eq!(66, parse::<isize>("0x42")?);

// you can use underscores for more readable inputs, just like in rust
assert_eq!(1_111_638_594, parse::<isize>("0x42_42__42_42")?);

// and negative values are recognised too
assert_eq!(-128, parse::<isize>("-0x80")?);

// parse octal
assert_eq!(34_u8, parse("0o42")?);
#[cfg(feature = "implicit-octal")]
{
    // Can enable implicit octal parsing, for correct IPv4 parsing
    assert_eq!(34, parse::<u8>("042")?);
}

// parse binary
assert_eq!(0x86_u16, parse("0b1000_0110")?);
# Ok(())
# }
```

## Pretty print numbers

The reverse is also possible

```rust
let pretty = parse_int::format_pretty_dec(1024);
assert_eq!("1_024", pretty);

assert_eq!("0x4_00", parse_int::format_pretty_hex(1024));
assert_eq!("0o2_000", parse_int::format_pretty_octal(1024));
assert_eq!("0b100_0000_0000", parse_int::format_pretty_bin(1024));
```

Negative numbers are represented like a human would spell it,  not in the memory representation or the two complement.
This makes the output not type dependent, see std::fmt for that representation.

```rust
assert_eq!("-1_024.102_4", parse_int::format_pretty_dec(-1024.1024));

assert_eq!("-0x4_00", parse_int::format_pretty_hex(-1024));
assert_eq!("-0o2_000", parse_int::format_pretty_octal(-1024));
assert_eq!("-0b100_0000_0000", parse_int::format_pretty_bin(-1024));
```

[Documentation](https://docs.rs/parse_int).

## Enable the "implicit-octal" feature

Specify the crate like this:

```yaml
[dependencies]
parse_int = { version = "0.9", features = ["implicit-octal"] }
```

Then this code will return `Hello, Ok(34)!`:

```rust
use parse_int::parse;
fn main() {
    println!("Hello, {:?}!", parse::<i128>("00042"));
}
```

## License

This work is distributed under the super-Rust quad-license:

[Apache-2.0]/[MIT]/[BSL-1.0]/[CC0-1.0]

This is equivalent to public domain in jurisdictions that allow it (CC0-1.0).
Otherwise it is compatible with the Rust license, plus the option of the
runtime-exception-containing BSL-1. This means that, outside of public domain
jurisdictions, the source must be distributed along with author attribution and
at least one of the licenses; but in binary form no attribution or license
distribution is required.

[Apache-2.0]: https://opensource.org/licenses/Apache-2.0
[MIT]: https://www.opensource.org/licenses/MIT
[BSL-1.0]: https://opensource.org/licenses/BSL-1.0
[CC0-1.0]: https://creativecommons.org/publicdomain/zero/1.0
