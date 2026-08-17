//! Error type.

use core::fmt::{self, Display};

#[cfg(feature = "pkcs8")]
use crate::pkcs8;

/// Result type with the `elliptic-curve` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// Elliptic curve errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("crypto error")
    }
}

impl From<base16ct::Error> for Error {
    fn from(_: base16ct::Error) -> Error {
        Error
    }
}

#[cfg(feature = "pkcs8")]
impl From<pkcs8::Error> for Error {
    fn from(_: pkcs8::Error) -> Error {
        Error
    }
}

#[cfg(feature = "sec1")]
impl From<sec1::Error> for Error {
    fn from(_: sec1::Error) -> Error {
        Error
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
