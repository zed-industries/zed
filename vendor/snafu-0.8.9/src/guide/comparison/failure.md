# SNAFU vs. Failure

This comparison was made against the examples in [the guide for
failure 0.1.8][failure-guide].

[failure-guide]: https://boats.gitlab.io/failure/

## "Strings as errors"

If you wanted to do something similar with SNAFU, you can use the
[`Whatever`] type:

```rust
use snafu::{prelude::*, Whatever};
use std::ops::Range;

fn check_range(x: usize, range: Range<usize>) -> Result<usize, Whatever> {
    if x < range.start {
        whatever!("{x} is below {}", range.start);
    }
    if x >= range.end {
        whatever!("{x} is above {}", range.end);
    }
    Ok(x)
}
```

## "A Custom Fail type" and "Using the Error type"

These two idioms from Failure are combined into one primary use case
in SNAFU. Additionally, SNAFU avoids the downsides listed in the
Failure guide.

You can represent multiple types of errors, allocation is not
required, and you can include any extra information relevant to the
error:

```rust
use snafu::prelude::*;
use std::ops::Range;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("{value} is below {bound}"))]
    Below { value: usize, bound: usize },

    #[snafu(display("{value} is above {bound}"))]
    Above { value: usize, bound: usize },
}

fn check_range(value: usize, range: Range<usize>) -> Result<usize, Error> {
    ensure!(
        value >= range.start,
        BelowSnafu {
            value,
            bound: range.start,
        },
    );
    ensure!(
        value < range.end,
        AboveSnafu {
            value,
            bound: range.end,
        },
    );
    Ok(value)
}
```

You do not have to have a one-to-one relationship between an
underlying error and an error variant:

```rust
use snafu::prelude::*;
use std::num::ParseIntError;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display(r#"Could not parse the area code from "{value}""#))]
    AreaCodeInvalid {
        value: String,
        source: ParseIntError,
    },

    #[snafu(display(r#"Could not parse the phone exchange from "{value}""#))]
    PhoneExchangeInvalid {
        value: String,
        source: ParseIntError,
    },
}

fn two_errors_from_same_underlying_error(
    area_code: &str,
    exchange: &str,
) -> Result<(i32, i32), Error> {
    let area_code: i32 = area_code
        .parse()
        .context(AreaCodeInvalidSnafu { value: area_code })?;
    let exchange: i32 = exchange
        .parse()
        .context(PhoneExchangeInvalidSnafu { value: exchange })?;
    Ok((area_code, exchange))
}
```

## "An Error and ErrorKind pair"

If you choose to make your error type [opaque][], you can implement
methods on the opaque type, allowing you to selectively choose what
your public API is.

This includes the ability to return a different public enum that
users can match on without knowing the details of your error
implementation.

```rust
use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum InnerError {
    MyError1 { username: String },
    MyError2 { username: String },
    MyError3 { address: String },
}

#[derive(Debug, Snafu)]
pub struct Error(InnerError);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Authorization,
    Network,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        use InnerError::*;

        match self.0 {
            MyError1 { .. } | MyError2 { .. } => ErrorKind::Authorization,
            MyError3 { .. } => ErrorKind::Network,
        }
    }

    pub fn username(&self) -> Option<&str> {
        use InnerError::*;

        match &self.0 {
            MyError1 { username } | MyError2 { username } => Some(username),
            _ => None,
        }
    }
}

# fn main() {}
```

[opaque]: crate::guide::opaque
