# bigdecimal-rs


[![crate](https://img.shields.io/crates/v/bigdecimal.svg)](https://crates.io/crates/bigdecimal)
[![Documentation](https://docs.rs/bigdecimal/badge.svg)](https://docs.rs/bigdecimal)

[![minimum rustc 1.43](https://img.shields.io/badge/rustc-1.43+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)

[![codecov](https://codecov.io/gh/akubera/bigdecimal-rs/branch/feature/circleci/graph/badge.svg?token=YTwyxrxJ3S)](https://codecov.io/gh/akubera/bigdecimal-rs)
[![build status - master](https://gitlab.com/akubera/bigdecimal-rs/badges/master/pipeline.svg?ignore_skipped=true&key_text=status:master&key_width=96)](https://gitlab.com/akubera/bigdecimal-rs/-/pipelines)
[![build status - trunk](https://gitlab.com/akubera/bigdecimal-rs/badges/trunk/pipeline.svg?ignore_skipped=true&key_text=status:trunk&key_width=96)](https://gitlab.com/akubera/bigdecimal-rs/-/pipelines)



Arbitrary-precision decimal numbers implemented in pure Rust.

##  Community

Join the conversation on Zulip: https://bigdecimal-rs.zulipchat.com

Please share important stuff like use-cases, issues, benchmarks, and
naming-convention preferences.

This project is currently being re-written, so if performance or flexibility
is lacking, check again soon and it may be fixed.

## Usage

Add bigdecimal as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
bigdecimal = "0.4"
```

Import and use the `BigDecimal` struct to solve your problems:

```rust
use bigdecimal::BigDecimal;

fn main() {
    let two = BigDecimal::from(2);
    println!("sqrt(2) = {}", two.sqrt().unwrap());
}
```

this code will print

```
sqrt(2) = 1.414213562373095048801688724209698078569671875376948073176679737990732478462107038850387534327641573
```

### Serialization

If you are passing BigDecimals between systems, be sure to use a serialization format
which explicitly supports decimal numbers and does not require transformations to
floating-point binary numbers, or there will be information loss.

Text formats like JSON should work ok as long as the receiver will also parse
numbers as decimals so complete precision is kept accurate.
Typically JSON-parsing implementations do not do this by default, and need special
configuration.

Binary formats like msgpack may expect/require representing numbers as 64-bit IEEE-754
floating-point, and will likely lose precision by default unless you explicitly format
the decimal as a string, bytes, or some custom structure.

By default, this will serialize the decimal _as a string_.

To use `serde_json` with this crate it is recommended to enable the `serde-json` feature
(note `serde -dash- json` , not `serde_json`) and that will add support for
serializing & deserializing values to BigDecimal.
By default it will parse numbers and strings using normal conventions.
If you want to serialize to a number, rather than a string, you can use the
`serde(with)` annotation as shown in the following example:

```toml
[dependencies]
bigdecimal = { version = "0.4", features = [ "serde-json" ] }  # '-' not '_'
```

```rust
use bigdecimal::BigDecimal;
use serde::*;
use serde_json;

#[derive(Debug,Serialize,Deserialize)]
struct MyStruct {
    name: String,
    // this will be written to json as string
    value: BigDecimal,
    // this will be written to json as number
    #[serde(with = "bigdecimal::serde::json_num")]
    number: BigDecimal,
}

fn main() {
    let json_src = r#"
        { "name": "foo", "value": 1234567e-3, "number": 3.14159 }
    "#;

    let my_struct: MyStruct = serde_json::from_str(&json_src).unwrap();
    dbg!(my_struct);
    // MyStruct { name: "foo", value: BigDecimal("1234.567"), BigDecimal("3.1459") }

    println!("{}", serde_json::to_string(&my_struct));
    // {"name":"foo","value":"1234.567","number":3.1459}
}
```

If you have suggestions for improving serialization, please bring them
to the Zulip chat.

### Formatting

Until a more sophisticated formatting solution is implemented (currently work in progress),
we are restricted to Rust's `fmt::Display` formatting options.
This is how this crate formats BigDecimals:

- `{}` - Default Display
    - Format as "human readable" number
    - "Small" fractional numbers (close to zero) are printed in scientific notation
        - Number is considered "small" by number of leading zeros exceeding a threshold
        - Configurable by the compile-time environment variable:
          `RUST_BIGDECIMAL_FMT_EXPONENTIAL_LOWER_THRESHOLD`
            - Default 5
        - Example: `1.23e-3` will print as `0.00123` but `1.23e-10` will be `1.23E-10`
    - Trailing zeros will be added to "small" integers, avoiding scientific notation
        - May appear to have more precision than they do
        - Example: decimal `1e1` would be rendered as `10`
        - The threshold for "small" is configured by compile-time environment variable:
          `RUST_BIGDECIMAL_FMT_EXPONENTIAL_UPPER_THRESHOLD`
            - Default 15
        - `1e15` => `1000000000000000`
        - Large integers (e.g. `1e50000000`) will print in scientific notation, not
          a 1 followed by fifty million zeros
    - All other numbers are printed in standard decimal notation

- `{:.<PREC>}` - Display with precision
    - Format number with exactly `PREC` digits after the decimal place
    - Numbers with fractional components will be rounded at precision point, or have
      zeros padded to precision point
    - Integers will have zeros padded to the precision point
        - To prevent unreasonably sized output, a threshold limits the number
          of padded zeros
            - Greater than the default case, since specific precision was requested
            - Configurable by the compile-time environment variable:
              `RUST_BIGDECIMAL_FMT_MAX_INTEGER_PADDING`
                - Default 1000
        - If digits exceed this threshold, they are printed without decimal-point,
          suffixed with scale of the big decimal

- `{:e}` / `{:E}` - Exponential format
    - Formats in scientific notation with either `e` or `E` as exponent delimiter
    - Precision is kept exactly

- `{:.<PREC>e}` - formats in scientific notation, keeping number
    - Number is rounded / zero padded until

- `{:?}` - Debug
    - Shows internal representation of BigDecimal
        - `123.456` => `BigDecimal(sign=Plus, scale=3, digits=[123456])`
        - `-1e10000` => `BigDecimal(sign=Minus, scale=-10000, digits=[1])`

- `{:#?}` - Alternate Debug (used by `dbg!()`)
    - Shows simple int+exponent string representation of BigDecimal
        - `123.456` => `BigDecimal("123456e-3")`
        - `-1e10000` => `BigDecimal("-1e10000")`

There is a [formatting-example](examples/formatting-examples.rs) script in the
`examples/` directory that demonstrates the formatting options and comparison
with Rust's standard floating point Display.

It is recommended you include unit tests in your code to guarantee that future
versions of BigDecimal continue to format numbers the way you expect.
The rules above are not likely to change, but they are probably the only part
of this library that are relatively subjective, and could change behavior
without indication from the compiler.

Also, check for changes to the configuration environment variables, those
may change name until 1.0.

### Compile-Time Configuration

You can set a few default parameters at _compile-time_ via environment variables:

|  Environment Variable                              | Default    |
|----------------------------------------------------|------------|
|  `RUST_BIGDECIMAL_DEFAULT_PRECISION`               |  100       |
|  `RUST_BIGDECIMAL_DEFAULT_ROUNDING_MODE`           | `HalfEven` |
|  `RUST_BIGDECIMAL_FMT_EXPONENTIAL_LOWER_THRESHOLD` |  5         |
|  `RUST_BIGDECIMAL_FMT_EXPONENTIAL_UPPER_THRESHOLD` |  15        |
|  `RUST_BIGDECIMAL_FMT_MAX_INTEGER_PADDING`         |  1000      |

These allow setting the default [Context] fields globally without incurring a runtime lookup,
or having to pass Context parameters through all calculations.
(If you want runtime control over these fields, you will have to pass Contexts to your functions.)
Examine [build.rs] for how those are converted to constants in the code (if interested).

[Context]: https://docs.rs/bigdecimal/latest/bigdecimal/struct.Context.html
[build.rs]: ./build.rs


#### Default precision

Default precision may be set at compile time with the environment variable `RUST_BIGDECIMAL_DEFAULT_PRECISION`.
The default value of this variable is 100.

This will be used as maximum precision for operations which may produce infinite digits (inverse, sqrt, ...).

Note that other operations, such as multiplication, will preserve all digits;
so multiplying two 70 digit numbers will result in one 140 digit number.
The user will have to manually trim the number of digits after calculations to
reasonable amounts using the `x.with_prec(30)` method.

A new set of methods with explicit precision and rounding modes is being worked
on, but even after those are introduced the default precision will have to be
used as the implicit value.

#### Rounding mode

The default Context uses this value for rounding.
Valid values are the variants of the [RoundingMode] enum.

Defaults to `HalfEven`.

[RoundingMode]: https://docs.rs/bigdecimal/latest/bigdecimal/rounding/enum.RoundingMode.html


#### Exponential Format Threshold

The maximum number of leading zeros after the decimal place before
the formatter uses exponential form (i.e. scientific notation).

There is currently no mechanism to change this during runtime.
If you know of a good solution for number formatting in Rust, please let me know!


#### Example Compile time configuration

Given the program:

```rust
fn main() {
    let n = BigDecimal::from(700);
    println!("1/{n} = {}", n.inverse());
}
```

Compiling with different environment variables prints different results

```
$ export BIG_DECIMAL_DEFAULT_PRECISION=8
$ cargo run
1/700 = 0.0014285714

$ export RUST_BIGDECIMAL_DEFAULT_PRECISION=5
$ cargo run
1/700 = 0.0014286

$ export RUST_BIGDECIMAL_DEFAULT_ROUNDING_MODE=Down
$ cargo run
1/700 = 0.0014285

$ export RUST_BIGDECIMAL_EXPONENTIAL_FORMAT_THRESHOLD=2
$ cargo run
1/700 = 1.4285E-3
```

> [!NOTE]
>
> These are **compile time** environment variables, and the BigDecimal
> library is not configurable at **runtime** via environment variable, or
> any kind of global variables, by default.
>
> This is for flexibility and performance.


## About

This repository contains code originally meant for a bigdecimal module
in the popular [num](https://crates.io/crates/num) crate, but was not
merged due to uncertainty of what the best design for such a crate
should be.


## License

This code is dual-licensed under the permissive
[MIT](https://opensource.org/licenses/MIT) &
[Apache 2.0](https://opensource.org/licenses/Apache-2.0) licenses.

###  Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
