use std::error;
use std::fmt::{self, Display, Formatter};
use std::io::{self, ErrorKind};

use rmp::decode::{MarkerReadError, ValueReadError};

pub mod value;
pub mod value_ref;

pub use self::value::{read_value, read_value_with_max_depth};
pub use self::value_ref::{read_value_ref, read_value_ref_with_max_depth};

/// The maximum recursion depth before [`Error::DepthLimitExceeded`] is returned.
pub const MAX_DEPTH: usize = 1024;

/// This type represents all possible errors that can occur when deserializing a value.
#[derive(Debug)]
pub enum Error {
    /// Error while reading marker byte.
    InvalidMarkerRead(io::Error),
    /// Error while reading data.
    InvalidDataRead(io::Error),
    /// The depth limit [`MAX_DEPTH`] was exceeded.
    DepthLimitExceeded,
}

#[inline]
fn decrement_depth(depth: u16) -> Result<u16, Error> {
    depth.checked_sub(1).ok_or(Error::DepthLimitExceeded)
}

impl Error {
    #[cold]
    #[must_use] pub fn kind(&self) -> ErrorKind {
        match *self {
            Error::InvalidMarkerRead(ref err) => err.kind(),
            Error::InvalidDataRead(ref err) => err.kind(),
            Error::DepthLimitExceeded => ErrorKind::Unsupported,
        }
    }
}

impl error::Error for Error {
    #[cold]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::InvalidMarkerRead(ref err) => Some(err),
            Error::InvalidDataRead(ref err) => Some(err),
            Error::DepthLimitExceeded => None,
        }
    }
}

impl Display for Error {
    #[cold]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Error::InvalidMarkerRead(ref err) => {
                write!(fmt, "I/O error while reading marker byte: {err}")
            }
            Error::InvalidDataRead(ref err) => {
                write!(fmt, "I/O error while reading non-marker bytes: {err}")
            }
            Error::DepthLimitExceeded => {
                write!(fmt, "depth limit exceeded")
            }
        }
    }
}

impl From<MarkerReadError> for Error {
    #[cold]
    fn from(err: MarkerReadError) -> Error {
        Error::InvalidMarkerRead(err.0)
    }
}

impl From<ValueReadError> for Error {
    #[cold]
    fn from(err: ValueReadError) -> Error {
        match err {
            ValueReadError::InvalidMarkerRead(err) => Error::InvalidMarkerRead(err),
            ValueReadError::InvalidDataRead(err) => Error::InvalidDataRead(err),
            ValueReadError::TypeMismatch(..) => {
                Error::InvalidMarkerRead(io::Error::new(ErrorKind::Other, "type mismatch"))
            }
        }
    }
}

impl From<Error> for io::Error {
    #[cold]
    fn from(val: Error) -> Self {
        match val {
            Error::InvalidMarkerRead(err) |
            Error::InvalidDataRead(err) => err,
            Error::DepthLimitExceeded => io::Error::new(val.kind(), val),
        }
    }
}
