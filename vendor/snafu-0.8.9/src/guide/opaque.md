# How to create opaque error types for public APIs

While creating error types on top of Rust enums allows for great
flexibility inside your code, that same flexibility may not be
desired in a public API. Public enums also expose their variants
and the variant's fields, allowing consumers to rely on those
details.

The most conservative approach is to create an *opaque* error type
that only implements a handful of traits. This can be done by
deriving `Snafu` for a newtype struct that contains another SNAFU
error:

```rust
# use snafu::prelude::*;
#[derive(Debug, Snafu)]
pub struct Error(InnerError);

// That's all it takes! The rest is demonstration of how to use it.

pub fn login(id: i32) -> Result<(), Error> {
    validate_user(id)?;
    is_user_locked(id)?;
    Ok(())
}

#[derive(Debug, Snafu)]
enum InnerError {
    #[snafu(display("User ID {user_id} is invalid"))]
    InvalidUser { user_id: i32 },
    #[snafu(display("User ID {user_id} is locked"))]
    UserLocked { user_id: i32 },
}

fn validate_user(user_id: i32) -> Result<(), InnerError> {
    InvalidUserSnafu { user_id }.fail()
}

fn is_user_locked(user_id: i32) -> Result<(), InnerError> {
    UserLockedSnafu { user_id }.fail()
}
```

## Delegated traits

- [`Error`][]
- [`Display`][]
- [`ErrorCompat`][]

[`Error`]: std::error::Error
[`Display`]: std::fmt::Display
[`ErrorCompat`]: crate::ErrorCompat

## `From`

The `From` trait is also implemented to convert the inner type into
the opaque type. This makes converting from internal errors to public
errors very easy.
