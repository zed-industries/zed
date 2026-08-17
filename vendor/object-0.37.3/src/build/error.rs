use alloc::string::String;
use core::{fmt, result};
#[cfg(feature = "std")]
use std::error;

use crate::{read, write};

/// The error type used within the build module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub(super) String);

impl Error {
    pub(super) fn new(message: impl Into<String>) -> Self {
        Error(message.into())
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {}
#[cfg(all(not(feature = "std"), core_error))]
impl core::error::Error for Error {}

impl From<read::Error> for Error {
    fn from(error: read::Error) -> Error {
        Error(format!("{}", error))
    }
}

impl From<write::Error> for Error {
    fn from(error: write::Error) -> Error {
        Error(error.0)
    }
}

/// The result type used within the build module.
pub type Result<T> = result::Result<T, Error>;
