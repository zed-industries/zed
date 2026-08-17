//! Error types
use serde::de::Error as SerdeError;
use std::{error::Error as StdError, fmt};

/// Types of errors that may result from failed attempts
/// to deserialize a type from env vars
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MissingValue(&'static str),
    Custom(String),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut fmt::Formatter,
    ) -> fmt::Result {
        match *self {
            Error::MissingValue(field) => write!(fmt, "missing value for field {}", field),
            Error::Custom(ref msg) => write!(fmt, "{}", msg),
        }
    }
}

impl SerdeError for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg))
    }

    fn missing_field(field: &'static str) -> Error {
        Error::MissingValue(field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn impl_std_error<E: StdError>(_: E) {}

    #[test]
    fn error_impl_std_error() {
        impl_std_error(Error::MissingValue("foo_bar"));
        impl_std_error(Error::Custom("whoops".into()))
    }

    #[test]
    fn error_display() {
        assert_eq!(
            format!("{}", Error::MissingValue("foo_bar")),
            "missing value for field foo_bar"
        );

        assert_eq!(format!("{}", Error::Custom("whoops".into())), "whoops")
    }
}
