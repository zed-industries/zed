// SPDX-License-Identifier: Apache-2.0

use alloc::string::{String, ToString};
use core::fmt::{Debug, Display, Formatter, Result};

use serde::ser::{Error as SerError, StdError};

/// An error occurred during serialization
#[derive(Debug)]
pub enum Error<T> {
    /// An error occurred while writing bytes
    ///
    /// Contains the underlying error reaturned while writing.
    Io(T),

    /// An error indicating a value that cannot be serialized
    ///
    /// Contains a description of the problem.
    Value(String),
}

impl<T> From<T> for Error<T> {
    #[inline]
    fn from(value: T) -> Self {
        Error::Io(value)
    }
}

impl<T: Debug> Display for Error<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Debug> StdError for Error<T> {}

impl<T: Debug> SerError for Error<T> {
    fn custom<U: Display>(msg: U) -> Self {
        Error::Value(msg.to_string())
    }
}
