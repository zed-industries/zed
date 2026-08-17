//! A custom Goblin error
//!

use alloc::string::String;
use core::fmt;
use core::num::TryFromIntError;
use core::result;
#[cfg(feature = "std")]
use std::{error, io};

#[non_exhaustive]
#[derive(Debug)]
/// A custom Goblin error
pub enum Error {
    /// The binary is malformed somehow
    Malformed(String),
    /// The binary's magic is unknown or bad
    BadMagic(u64),
    /// An error emanating from reading and interpreting bytes
    Scroll(scroll::Error),
    /// An IO based error
    #[cfg(feature = "std")]
    IO(io::Error),
    /// Buffer is too short to hold N items
    BufferTooShort(usize, &'static str),
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::IO(ref io) => Some(io),
            Error::Scroll(ref scroll) => Some(scroll),
            _ => None,
        }
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<TryFromIntError> for Error {
    fn from(err: TryFromIntError) -> Error {
        Error::Malformed(format!("Integer do not fit: {err}"))
    }
}

impl From<scroll::Error> for Error {
    fn from(err: scroll::Error) -> Error {
        Error::Scroll(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            #[cfg(feature = "std")]
            Error::IO(ref err) => write!(fmt, "{}", err),
            Error::Scroll(ref err) => write!(fmt, "{}", err),
            Error::BadMagic(magic) => write!(fmt, "Invalid magic number: 0x{:x}", magic),
            Error::Malformed(ref msg) => write!(fmt, "Malformed entity: {}", msg),
            Error::BufferTooShort(n, item) => write!(fmt, "Buffer is too short for {} {}", n, item),
        }
    }
}

/// An impish result
pub type Result<T> = result::Result<T, Error>;
