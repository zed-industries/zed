// SPDX-License-Identifier: Apache-2.0

use alloc::string::{String, ToString};

/// The error when serializing to/from a `Value`
#[derive(Debug)]
pub enum Error {
    /// A custom error string produced by serde
    Custom(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl serde::de::StdError for Error {}

impl serde::de::Error for Error {
    #[inline]
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    #[inline]
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}
