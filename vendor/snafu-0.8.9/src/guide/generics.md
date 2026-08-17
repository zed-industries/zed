# Using generic types

Error types enhanced by SNAFU may contain generic type and lifetime parameters.

## Types

```rust
# use snafu::prelude::*;
#
#[derive(Debug, Snafu)]
enum Error<T>
where
    T: std::fmt::Display,
{
    #[snafu(display("The value {value} was too large"))]
    TooLarge { value: T, limit: u32 },

    #[snafu(display("The value {value} was too small"))]
    TooSmall { value: T, limit: u32 },
}

fn validate_number(value: u8) -> Result<u8, Error<u8>> {
    ensure!(
        value <= 200,
        TooLargeSnafu {
            value,
            limit: 100u32,
        },
    );
    ensure!(
        value >= 100,
        TooSmallSnafu {
            value,
            limit: 200u32,
        },
    );
    Ok(value)
}

fn validate_string(value: &str) -> Result<&str, Error<String>> {
    ensure!(
        value.len() <= 20,
        TooLargeSnafu {
            value,
            limit: 10u32,
        },
    );
    ensure!(
        value.len() >= 10,
        TooSmallSnafu {
            value,
            limit: 20u32,
        },
    );
    Ok(value)
}
```

## Lifetimes

```rust
# use snafu::prelude::*;
#
#[derive(Debug, Snafu)]
enum Error<'a> {
    #[snafu(display("The username {value} contains the bad word {word}"))]
    BadWord { value: &'a str, word: &'static str },
}

fn validate_username<'a>(value: &'a str) -> Result<&'a str, Error<'a>> {
    ensure!(
        !value.contains("stinks"),
        BadWordSnafu {
            value,
            word: "stinks",
        },
    );
    ensure!(
        !value.contains("smells"),
        BadWordSnafu {
            value,
            word: "smells",
        },
    );
    Ok(value)
}
```

## Caveats

A SNAFU [opaque type](crate::guide::opaque) requires that the
contained type implements several traits, such as
`Display`. However, type constraints cannot be automatically added
to the opaque type because they are not allowed to reference the
inner type without also exposing it publicly.

The best option is to avoid using a generic opaque error. If you
choose to expose a generic opaque error, you will likely need to add
explicit duplicate type constraints:

```rust
use snafu::prelude::*;

#[derive(Debug, Snafu)]
struct ApiError<T>(Error<T>)
// The bound is required to ensure that delegation can work.
where
    T: std::fmt::Debug;

#[derive(Debug, Snafu)]
enum Error<T>
where
    T: std::fmt::Debug,
{
    #[snafu(display("Boom: {value:?}"))]
    Boom { value: T },
}
```
