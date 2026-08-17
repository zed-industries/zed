use core::fmt::{self, Display};
use core::result;
#[cfg(feature = "std")]
use std::{error, io};

#[derive(Debug)]
/// A custom Scroll error
pub enum Error {
    /// The type you tried to read was too big
    TooBig {
        size: usize,
        len: usize,
    },
    /// The requested offset to read/write at is invalid
    BadOffset(usize),
    BadInput {
        size: usize,
        msg: &'static str,
    },
    /// A custom Scroll error for reporting messages to clients.
    /// For no-std, use [`Error::BadInput`] with a static string.
    #[cfg(feature = "std")]
    Custom(String),
    /// Returned when IO based errors are encountered
    #[cfg(feature = "std")]
    IO(io::Error),
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::TooBig { .. } => "TooBig",
            Error::BadOffset(_) => "BadOffset",
            Error::BadInput { .. } => "BadInput",
            Error::Custom(_) => "Custom",
            Error::IO(_) => "IO",
        }
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::TooBig { .. } => None,
            Error::BadOffset(_) => None,
            Error::BadInput { .. } => None,
            Error::Custom(_) => None,
            Error::IO(ref io) => io.source(),
        }
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TooBig { ref size, ref len } => {
                write!(fmt, "type is too big ({size}) for {len}")
            }
            Error::BadOffset(ref offset) => {
                write!(fmt, "bad offset {offset}")
            }
            Error::BadInput { ref msg, ref size } => {
                write!(fmt, "bad input {msg} ({size})")
            }
            #[cfg(feature = "std")]
            Error::Custom(ref msg) => {
                write!(fmt, "{msg}")
            }
            #[cfg(feature = "std")]
            Error::IO(ref err) => {
                write!(fmt, "{err}")
            }
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
