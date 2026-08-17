use crate::std::fmt;

/// An error encountered while working with structured data.
#[derive(Debug)]
pub struct Error {
    inner: Inner,
}

#[derive(Debug)]
enum Inner {
    #[cfg(feature = "std")]
    Boxed(std_support::BoxedError),
    Msg(&'static str),
    Fmt,
}

impl Error {
    /// Create an error from a message.
    pub fn msg(msg: &'static str) -> Self {
        Error {
            inner: Inner::Msg(msg),
        }
    }

    #[cfg(feature = "serde1")]
    pub(crate) fn try_boxed(msg: &'static str, e: impl fmt::Display) -> Self {
        #[cfg(feature = "std")]
        {
            Error::boxed(format!("{msg}: {e}"))
        }
        #[cfg(not(feature = "std"))]
        {
            let _ = e;
            Error::msg(msg)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Inner::*;
        match self.inner {
            #[cfg(feature = "std")]
            Boxed(ref err) => err.fmt(f),
            Msg(ref msg) => msg.fmt(f),
            Fmt => fmt::Error.fmt(f),
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Error { inner: Inner::Fmt }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;
    use crate::std::{boxed::Box, error, io};

    pub(crate) type BoxedError = Box<dyn error::Error + Send + Sync>;

    impl Error {
        /// Create an error from a standard error type.
        pub fn boxed<E>(err: E) -> Self
        where
            E: Into<BoxedError>,
        {
            Error {
                inner: Inner::Boxed(err.into()),
            }
        }
    }

    impl error::Error for Error {}

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::boxed(err)
        }
    }
}
