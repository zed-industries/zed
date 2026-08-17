# Decimal &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Badge]][docs]

[Build Status]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fpaupino%2Frust-decimal%2Fbadge&label=build&logo=none

[actions]: https://actions-badge.atrox.dev/paupino/rust-decimal/goto

[Latest Version]: https://img.shields.io/crates/v/rust-decimal.svg

[crates.io]: https://crates.io/crates/rust-decimal

[Docs Badge]: https://docs.rs/rust_decimal/badge.svg

[docs]: https://docs.rs/rust_decimal

A Decimal number implementation written in pure Rust suitable for financial calculations that require significant
integral and fractional digits with no round-off errors.

The binary representation consists of a 96 bit integer number, a scaling factor used to specify the decimal fraction and
a 1 bit sign. Because of this representation, trailing zeros are preserved and may be exposed when in string form. These
can be truncated using the `normalize` or `round_dp` functions.

## Installing

```sh
$ cargo add rust_decimal
```

Alternatively, you can edit your `Cargo.toml` directly and run `cargo update`:

```toml
[dependencies]
rust_decimal = "1.39"
```

To enable macro support, you can enable the `macros` feature:

```sh
$ cargo add rust_decimal --features macros
```

## Usage

Decimal numbers can be created in a few distinct ways. The easiest and most efficient method of creating a Decimal is to
use the macro:

```rust
// Import via use rust_decimal_macros or use the `macros` feature to import at the crate level
// `use rust_decimal_macros::dec;`
// or
// `use rust_decimal::dec;`

let number = dec!(-1.23) + dec!(3.45);
assert_eq!(number, dec!(2.22));
assert_eq!(number.to_string(), "2.22");
```

Alternatively you can also use one of the Decimal number convenience
functions ([see the docs](https://docs.rs/rust_decimal/) for more details):

```rust
// Using the prelude can help importing trait based functions (e.g. core::str::FromStr).
use rust_decimal::prelude::*;

// Using an integer followed by the decimal points
let scaled = Decimal::new(202, 2);
assert_eq!("2.02", scaled.to_string());

// From a 128 bit integer
let balance = Decimal::from_i128_with_scale(5_897_932_384_626_433_832, 2);
assert_eq!("58979323846264338.32", balance.to_string());

// From a string representation
let from_string = Decimal::from_str("2.02").unwrap();
assert_eq!("2.02", from_string.to_string());

// From a string representation in a different base
let from_string_base16 = Decimal::from_str_radix("ffff", 16).unwrap();
assert_eq!("65535", from_string_base16.to_string());

// From scientific notation
let sci = Decimal::from_scientific("9.7e-7").unwrap();
assert_eq!("0.00000097", sci.to_string());

// Using the `Into` trait
let my_int: Decimal = 3_i32.into();
assert_eq!("3", my_int.to_string());

// Using the raw decimal representation
let pi = Decimal::from_parts(1_102_470_952, 185_874_565, 1_703_060_790, false, 28);
assert_eq!("3.1415926535897932384626433832", pi.to_string());

// If the `macros` feature is enabled, it also allows for the `dec!` macro
let amount = dec!(25.12);
assert_eq!("25.12", amount.to_string());
```

Once you have instantiated your `Decimal` number you can perform calculations with it just like any other number:

```rust
use rust_decimal::prelude::*; // Includes the `dec` macro when feature specified

let amount = dec!(25.12);
let tax_percentage = dec!(0.085);
let total = amount + (amount * tax_percentage).round_dp(2);
assert_eq!(total, dec!(27.26));
```

## Features

**Behavior / Functionality**

* [borsh](#borsh)
* [c-repr](#c-repr)
* [legacy-ops](#legacy-ops)
* [macros](#macros)
* [maths](#maths)
* [ndarray](#ndarray)
* [rkyv](#rkyv)
* [rocket-traits](#rocket-traits)
* [rust-fuzz](#rust-fuzz)
* [std](#std)

**Database**

* [db-postgres](#db-postgres)
* [db-tokio-postgres](#db-tokio-postgres)
* [db-diesel-postgres](#db-diesel-postgres)
* [db-diesel-mysql](#db-diesel-mysql)

**Serde**

* [serde-float](#serde-float)
* [serde-str](#serde-str)
* [serde-arbitrary-precision](#serde-arbitrary-precision)
* [serde-with-float](#serde-with-float)
* [serde-with-str](#serde-with-str)
* [serde-with-arbitrary-precision](#serde-with-arbitrary-precision)

### `align16`

Forces `Decimal`'s alignment to 16 bytes (128 bits). This is identical to `u128` and `i128`'s alignment on x86 platforms.

### `borsh`

Enables [Borsh](https://borsh.io/) serialization for `Decimal`.

### `c-repr`

Forces `Decimal` to use `[repr(C)]`.

### `db-postgres`

Enables a PostgreSQL communication module. It allows for reading and writing the `Decimal`
type by transparently serializing/deserializing into the `NUMERIC` data type within PostgreSQL.

### `db-tokio-postgres`

Enables the tokio postgres module allowing for async communication with PostgreSQL.

### `db-diesel-postgres`

Enable [`diesel`](https://diesel.rs) PostgreSQL support.

### `db-diesel-mysql`

Enable [`diesel`](https://diesel.rs) MySQL support.

### `legacy-ops`

**Warning:** This is deprecated and will be removed from a future versions.

As of `1.10` the algorithms used to perform basic operations have changed which has benefits of significant speed
improvements.
To maintain backwards compatibility this can be opted out of by enabling the `legacy-ops` feature.

### `macros`

The `macros` feature enables a compile time macro `dec` to be available at both the crate root, and via prelude.

This parses the input at compile time and converts it to an optimized `Decimal` representation. Invalid inputs will
cause a compile time error.

Any Rust number format is supported, including scientific notation and alternate bases.

```rust
use rust_decimal::prelude::*;

assert_eq!(dec!(1.23), Decimal::new(123, 2));
```

### `maths`

The `maths` feature enables additional complex mathematical functions such as `pow`, `ln`, `enf`, `exp` etc.
Documentation detailing the additional functions can be found on the
[`MathematicalOps`](https://docs.rs/rust_decimal/latest/rust_decimal/trait.MathematicalOps.html) trait.

Please note that `ln` and `log10` will panic on invalid input with `checked_ln` and `checked_log10` the preferred
functions
to curb against this. When the `maths` feature was first developed the library would instead return `0` on invalid
input. To re-enable this
non-panicking behavior, please use the feature: `maths-nopanic`.

### `ndarray`

Enables arithmetic operations using [`ndarray`](https://github.com/rust-ndarray/ndarray) on arrays of `Decimal`.

### `proptest`

Enables a [`proptest`](https://github.com/proptest-rs/proptest) strategy to generate values for Rust Decimal.

### `rand`

Implements `rand::distributions::Distribution<Decimal>` to allow the creation of random instances.

Note: When using `rand::Rng` trait to generate a decimal between a range of two other decimals, the scale of the
randomly-generated
decimal will be the same as the scale of the input decimals (or, if the inputs have different scales, the higher of the
two).

### `rkyv`

Enables [rkyv](https://github.com/rkyv/rkyv) serialization for `Decimal`. In order to avoid breaking changes, this is
currently locked at version `0.7`.

Supports rkyv's safe API when the `rkyv-safe` feature is enabled as well.

If `rkyv` support for versions `0.8` of greater is desired, `rkyv`'
s [remote derives](https://rkyv.org/derive-macro-features/remote-derive.html) should be used instead. See
`examples/rkyv-remote`.

### `rocket-traits`

Enable support for Rocket forms by implementing the `FromFormField` trait.

### `rust-fuzz`

Enable `rust-fuzz` support by implementing the `Arbitrary` trait.

### `serde-float`

> **Note:** This feature applies float serialization/deserialization rules as the default method for handling `Decimal`
> numbers.
> See also the `serde-with-*` features for greater flexibility.

Enable this so that JSON serialization of `Decimal` types are sent as a float instead of a string (default).

e.g. with this turned on, JSON serialization would output:

```json
{
  "value": 1.234
}
```

### `serde-str`

> **Note:** This feature applies string serialization/deserialization rules as the default method for handling `Decimal`
> numbers.
> See also the `serde-with-*` features for greater flexibility.

This is typically useful for `bincode` or `csv` like implementations.

Since `bincode` does not specify type information, we need to ensure that a type hint is provided in order to
correctly be able to deserialize. Enabling this feature on its own will force deserialization to use `deserialize_str`
instead of `deserialize_any`.

If, for some reason, you also have `serde-float` enabled then this will use `deserialize_f64` as a type hint. Because
converting to `f64` _loses_ precision, it's highly recommended that you do NOT enable this feature when working with
`bincode`. That being said, this will only use 8 bytes so is slightly more efficient in terms of storage size.

### `serde-arbitrary-precision`

> **Note:** This feature applies arbitrary serialization/deserialization rules as the default method for
> handling `Decimal` numbers.
> See also the `serde-with-*` features for greater flexibility.

This is used primarily with `serde_json` and consequently adds it as a "weak dependency". This supports the
`arbitrary_precision` feature inside `serde_json` when parsing decimals.

This is recommended when parsing "float" looking data as it will prevent data loss.

Please note, this currently serializes numbers in a float like format by default, which can be an unexpected
consequence. For greater
control over the serialization format, please use the `serde-with-arbitrary-precision` feature.

### `serde-with-float`

Enable this to access the module for serializing `Decimal` types to a float. This can be used in `struct` definitions
like so:

```rust
#[derive(Serialize, Deserialize)]
pub struct FloatExample {
    #[serde(with = "rust_decimal::serde::float")]
    value: Decimal,
}
```

```rust
#[derive(Serialize, Deserialize)]
pub struct OptionFloatExample {
    #[serde(with = "rust_decimal::serde::float_option")]
    value: Option<Decimal>,
}
```

Alternatively, if only the serialization feature is desired (e.g. to keep flexibility while deserialization):

```rust
#[derive(Serialize, Deserialize)]
pub struct FloatExample {
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    value: Decimal,
}
```

### `serde-with-str`

Enable this to access the module for serializing `Decimal` types to a `String`. This can be used in `struct` definitions
like so:

```rust
#[derive(Serialize, Deserialize)]
pub struct StrExample {
    #[serde(with = "rust_decimal::serde::str")]
    value: Decimal,
}
```

```rust
#[derive(Serialize, Deserialize)]
pub struct OptionStrExample {
    #[serde(with = "rust_decimal::serde::str_option")]
    value: Option<Decimal>,
}
```

This feature isn't typically required for serialization however can be useful for deserialization purposes since it does
not require
a type hint. Consequently, you can force this for just deserialization by:

```rust
#[derive(Serialize, Deserialize)]
pub struct StrExample {
    #[serde(deserialize_with = "rust_decimal::serde::str::deserialize")]
    value: Decimal,
}
```

### `serde-with-arbitrary-precision`

Enable this to access the module for deserializing `Decimal` types using the `serde_json/arbitrary_precision` feature.
This can be used in `struct` definitions like so:

```rust
#[derive(Serialize, Deserialize)]
pub struct ArbitraryExample {
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    value: Decimal,
}
```

```rust
#[derive(Serialize, Deserialize)]
pub struct OptionArbitraryExample {
    #[serde(with = "rust_decimal::serde::arbitrary_precision_option")]
    value: Option<Decimal>,
}
```

An unexpected consequence of this feature is that it will serialize as a float like number. To prevent this, you can
target the struct to only deserialize with the `arbitrary_precision` feature:

```rust
#[derive(Serialize, Deserialize)]
pub struct ArbitraryExample {
    #[serde(deserialize_with = "rust_decimal::serde::arbitrary_precision::deserialize")]
    value: Decimal,
}
```

This will ensure that serialization still occurs as a string.

Please see the `examples` directory for more information regarding `serde_json` scenarios.

### `std`

Enable `std` library support. This is enabled by default, however in the future will be opt in. For now, to
support `no_std`
libraries, this crate can be compiled with `--no-default-features`.

## Building

Please refer to the [Build document](BUILD.md) for more information on building and testing Rust Decimal.

## Minimum Rust Compiler Version

The current _minimum_ compiler version is `1.67.1` which was released on `2023-02-09`.

This library maintains support for rust compiler versions that are 4 minor versions away from the current stable rust
compiler version.
For example, if the current stable compiler version is `1.50.0` then we will guarantee support up to and
including `1.46.0`.
Of note, we will only update the minimum supported version if and when required.

## Comparison to other Decimal implementations

During the development of this library, there were various design decisions made to ensure that decimal calculations
would
be quick, accurate and efficient. Some decisions, however, put limitations on what this library can do and ultimately
what
it is suitable for. One such decision was the structure of the internal decimal representation.

This library uses a mantissa of 96 bits made up of three 32-bit unsigned integers with a fourth 32-bit unsigned integer
to represent the scale/sign
(similar to the C and .NET Decimal implementations).
This structure allows us to make use of algorithmic optimizations to implement basic arithmetic; ultimately this gives
us the ability
to squeeze out performance and make it one of the fastest implementations available. The downside of this approach
however is that
the maximum number of significant digits that can be represented is roughly 28 base-10 digits (29 in some cases).

While this constraint is not an issue for many applications (e.g. when dealing with money), some applications may
require a higher number of significant digits to be represented. Fortunately,
there are alternative implementations that may be worth investigating, such as:

* [bigdecimal](https://crates.io/crates/bigdecimal)
* [decimal-rs](https://crates.io/crates/decimal-rs)

If you have further questions about the suitability of this library for your project, then feel free to either start a
[discussion](https://github.com/paupino/rust-decimal/discussions) or open
an [issue](https://github.com/paupino/rust-decimal/issues) and we'll
do our best to help.
