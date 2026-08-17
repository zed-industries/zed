use crate::std::fmt;

/**
An error encountered while streaming a value.

Errors don't capture details of failures, that responsibility is left
to the stream to surface.
*/
#[derive(Debug)]
pub struct Error(());

impl Error {
    /**
    Create a new error.

    More detailed diagnostic information will need to be stored elsewhere.
    */
    #[inline(always)]
    pub fn new() -> Self {
        Error(())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to stream data")
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error;

    impl error::Error for Error {}
}

/**
A streaming result with a generic failure.

More detailed diagnostic information will need to be stored elsewhere.
*/
#[inline(always)]
pub fn error<T>() -> crate::Result<T> {
    Err(Error::new())
}
