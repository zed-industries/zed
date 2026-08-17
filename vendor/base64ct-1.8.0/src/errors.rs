//! Error types

use core::fmt;

const INVALID_ENCODING_MSG: &str = "invalid Base64 encoding";
const INVALID_LENGTH_MSG: &str = "invalid Base64 length";

/// Insufficient output buffer length.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InvalidLengthError;

impl fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(INVALID_LENGTH_MSG)
    }
}

impl core::error::Error for InvalidLengthError {}

/// Invalid encoding of provided Base64 string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InvalidEncodingError;

impl fmt::Display for InvalidEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(INVALID_ENCODING_MSG)
    }
}

impl core::error::Error for InvalidEncodingError {}

/// Generic error, union of [`InvalidLengthError`] and [`InvalidEncodingError`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Invalid encoding of provided Base64 string.
    InvalidEncoding,

    /// Insufficient output buffer length.
    InvalidLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s = match self {
            Self::InvalidEncoding => INVALID_ENCODING_MSG,
            Self::InvalidLength => INVALID_LENGTH_MSG,
        };
        f.write_str(s)
    }
}

impl From<InvalidEncodingError> for Error {
    #[inline]
    fn from(_: InvalidEncodingError) -> Error {
        Error::InvalidEncoding
    }
}

impl From<InvalidLengthError> for Error {
    #[inline]
    fn from(_: InvalidLengthError) -> Error {
        Error::InvalidLength
    }
}

impl From<core::str::Utf8Error> for Error {
    #[inline]
    fn from(_: core::str::Utf8Error) -> Error {
        Error::InvalidEncoding
    }
}

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    fn from(err: Error) -> std::io::Error {
        // TODO(tarcieri): better customize `ErrorKind`?
        std::io::Error::new(std::io::ErrorKind::InvalidData, err)
    }
}

impl core::error::Error for Error {}
