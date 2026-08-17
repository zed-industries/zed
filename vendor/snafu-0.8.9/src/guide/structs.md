# Struct errors

You may not always need the flexibility of an enum for your error
type. In those cases, you can use the familiar SNAFU attributes with a
struct:

```rust
# use std::convert::TryFrom;
# use snafu::prelude::*;
#[derive(Debug, Snafu)]
#[snafu(display("Unable to parse {value} as MyEnum"))]
struct ParseError {
    value: u8,
}

// That's all it takes! The rest is demonstration of how to use it.

#[derive(Debug)]
enum MyEnum {
    Alpha,
    Beta,
    Gamma,
}

impl TryFrom<u8> for MyEnum {
    type Error = ParseError;

    fn try_from(other: u8) -> Result<Self, Self::Error> {
        match other {
            0 => Ok(Self::Alpha),
            1 => Ok(Self::Beta),
            2 => Ok(Self::Gamma),
            value => ParseSnafu { value }.fail(),
        }
    }
}
```
