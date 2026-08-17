// Copyright 2017 Jonathan Creekmore
//
// Licensed under the MIT license <LICENSE.md or
// http://opensource.org/licenses/MIT>. This file may not be
// copied, modified, or distributed except according to those terms.
use core::fmt;

#[cfg(any(feature = "std", test))]
use std::error::Error;

#[cfg(not(any(feature = "std", test)))]
use alloc::string::String;

/// The `pem` error type.
#[derive(Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum PemError {
    MismatchedTags(String, String),
    MalformedFraming,
    MissingBeginTag,
    MissingEndTag,
    MissingData,
    InvalidData(::base64::DecodeError),
    InvalidHeader(String),
    NotUtf8(::core::str::Utf8Error),
}

impl fmt::Display for PemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PemError::MismatchedTags(b, e) => {
                write!(f, "mismatching BEGIN (\"{b}\") and END (\"{e}\") tags")
            }
            PemError::MalformedFraming => write!(f, "malformedframing"),
            PemError::MissingBeginTag => write!(f, "missing BEGIN tag"),
            PemError::MissingEndTag => write!(f, "missing END tag"),
            PemError::MissingData => write!(f, "missing data"),
            PemError::InvalidData(e) => write!(f, "invalid data: {e}"),
            PemError::InvalidHeader(hdr) => write!(f, "invalid header: {hdr}"),
            PemError::NotUtf8(e) => write!(f, "invalid utf-8 value: {e}"),
        }
    }
}

#[cfg(any(feature = "std", test))]
impl Error for PemError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // Errors originating from other libraries.
            PemError::InvalidData(e) => Some(e),
            PemError::NotUtf8(e) => Some(e),
            // Errors directly originating from `pem-rs`.
            _ => None,
        }
    }
}

/// The `pem` result type.
pub type Result<T> = ::core::result::Result<T, PemError>;

#[allow(missing_docs)]
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}
