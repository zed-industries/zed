//! Crate error types
//!
//! Contains error types utilised by other modules within the crate

use std::error::Error;
use std::fmt;
use std::io;

/// Happens when we parse stuff from strings
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    /// Not enough capacity in underlying integer to perform a math operation
    OverflowError,

    /// Could not convert a character into a digit or a string into a number
    ParseIntError,

    /// The base is not supported. E.g. a type only supports base 10, but we try to
    /// parse with the base 7.
    UnsupportedBase,
}

unsafe impl Send for ParseError {}
unsafe impl Sync for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::OverflowError => write!(f, "Overflow"),
            ParseError::ParseIntError => write!(f, "Could not parse integer"),
            ParseError::UnsupportedBase => write!(f, "Unsupported base"),
        }
    }
}

impl Error for ParseError {}

/// Could not perform division, or fill in the resulting buffer
#[derive(Debug)]
pub enum DivisionError {
    /// Trying to divide something by Zero
    DivisionByZero,

    /// Incapsulates [fmt::Error]
    FmtError(fmt::Error),

    /// Incapsulates [io::Error]
    IoError(io::Error),

    /// Errors external to the division algorithm still may be passed
    /// through the co-routines wrapped up with this constructor
    ExternalError(Box<dyn Error + Send + Sync>),
}

unsafe impl Sync for DivisionError {}
unsafe impl Send for DivisionError {}

impl fmt::Display for DivisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DivisionError::DivisionByZero => write!(f, "Division by zero"),

            DivisionError::FmtError(e) => write!(f, "Fmt error: {}", e),
            DivisionError::IoError(e) => write!(f, "IO error: {}", e),

            DivisionError::ExternalError(e) => write!(f, "External error: {}", e),
        }
    }
}

impl Error for DivisionError {}

impl From<io::Error> for DivisionError {
    fn from(error: io::Error) -> DivisionError {
        DivisionError::IoError(error)
    }
}

impl From<fmt::Error> for DivisionError {
    fn from(error: fmt::Error) -> DivisionError {
        DivisionError::FmtError(error)
    }
}
