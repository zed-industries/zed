use core::fmt;

/// Result type with the `base16ct` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Error {
    /// Invalid encoding of provided Base16 string.
    InvalidEncoding,

    /// Insufficient output buffer length.
    InvalidLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidEncoding => f.write_str("invalid Base16 encoding"),
            Error::InvalidLength => f.write_str("invalid Base16 length"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<Error> for core::fmt::Error {
    fn from(_: Error) -> core::fmt::Error {
        core::fmt::Error::default()
    }
}
