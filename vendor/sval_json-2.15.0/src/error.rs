use core::fmt;

/**
An error encountered streaming JSON.
*/
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Generic,
    #[cfg(feature = "std")]
    IO(std::io::Error),
    InvalidKey,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Generic => write!(f, "an error occurred serializing a value to JSON"),
            #[cfg(feature = "std")]
            ErrorKind::IO(_) => write!(f, "failed to write JSON"),
            ErrorKind::InvalidKey => write!(f, "attempt to serialize a non-string key"),
        }
    }
}

impl Error {
    pub(crate) fn generic() -> Self {
        Error {
            kind: ErrorKind::Generic,
        }
    }

    pub(crate) fn invalid_key() -> Self {
        Error {
            kind: ErrorKind::InvalidKey,
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use std::error;

    impl error::Error for Error {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match self.kind {
                ErrorKind::IO(ref err) => Some(err),
                _ => None,
            }
        }
    }
}
