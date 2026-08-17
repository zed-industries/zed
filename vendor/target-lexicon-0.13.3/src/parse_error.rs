use alloc::string::String;

use core::fmt;

/// An error returned from parsing a triple.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ParseError {
    UnrecognizedArchitecture(String),
    UnrecognizedVendor(String),
    UnrecognizedOperatingSystem(String),
    UnrecognizedEnvironment(String),
    UnrecognizedBinaryFormat(String),
    UnrecognizedField(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use ParseError::*;
        match self {
            UnrecognizedArchitecture(msg) => write!(fmt, "Unrecognized architecture: {}", msg),
            UnrecognizedVendor(msg) => write!(fmt, "Unrecognized vendor: {}", msg),
            UnrecognizedOperatingSystem(msg) => {
                write!(fmt, "Unrecognized operating system: {}", msg)
            }
            UnrecognizedEnvironment(msg) => write!(fmt, "Unrecognized environment: {}", msg),
            UnrecognizedBinaryFormat(msg) => write!(fmt, "Unrecognized binary format: {}", msg),
            UnrecognizedField(msg) => write!(fmt, "Unrecognized field: {}", msg),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}
