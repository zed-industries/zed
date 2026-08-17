use core::fmt;

use crate::error_kind::ErrorKind;
#[cfg(not(feature = "std"))]
use crate::strings::ErrString;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
/// This crate's error type.
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Constructs a new [`Error`] with kind [`ErrorKind::Other`].
    ///
    /// [`Error`]: struct.Error.html
    /// [`ErrorKind::Other`]: enum.ErrorKind.html#variant.Other
    pub fn new<S>(message: S) -> Error
    where
        S: AsRef<str>,
    {
        #[cfg(feature = "std")]
        return Error {
            kind: ErrorKind::Other(message.as_ref().into()),
        };

        #[cfg(not(feature = "std"))]
        return Error {
            kind: ErrorKind::Other(ErrString::truncated(message).into()),
        };
    }

    /// Returns the [`ErrorKind`].
    ///
    /// [`ErrorKind`]: enum.ErrorKind.html
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl Error {
    pub(crate) fn capacity(len: usize, cap: usize) -> Error {
        Error {
            kind: ErrorKind::Capacity { len, cap },
        }
    }

    #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
    pub(crate) fn interior_nul_byte<S>(locale_name: S) -> Error
    where
        S: Into<String>,
    {
        Error {
            kind: ErrorKind::InteriorNulByte(locale_name.into()),
        }
    }

    pub(crate) fn parse_locale<S>(input: S) -> Error
    where
        S: AsRef<str>,
    {
        #[cfg(feature = "std")]
        return Error {
            kind: ErrorKind::ParseLocale(input.as_ref().into()),
        };

        #[cfg(not(feature = "std"))]
        return Error {
            kind: ErrorKind::ParseLocale(ErrString::truncated(input.as_ref()).into()),
        };
    }

    pub(crate) fn parse_number<S>(input: S) -> Error
    where
        S: AsRef<str>,
    {
        #[cfg(feature = "std")]
        return Error {
            kind: ErrorKind::ParseNumber(input.as_ref().into()),
        };

        #[cfg(not(feature = "std"))]
        return Error {
            kind: ErrorKind::ParseNumber(ErrString::truncated(input.as_ref()).into()),
        };
    }

    #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
    pub(crate) fn system_invalid_return<S, T>(function_name: S, message: T) -> Error
    where
        S: Into<String>,
        T: Into<String>,
    {
        Error {
            kind: ErrorKind::SystemInvalidReturn {
                function_name: function_name.into(),
                message: message.into(),
            },
        }
    }

    #[cfg(all(feature = "with-system-locale", unix))]
    pub(crate) fn system_unsupported_encoding<S>(encoding_name: S) -> Error
    where
        S: Into<String>,
    {
        Error {
            kind: ErrorKind::SystemUnsupportedEncoding(encoding_name.into()),
        }
    }

    #[cfg(all(feature = "with-system-locale", any(unix, windows)))]
    pub(crate) fn system_unsupported_grouping<B>(bytes: B) -> Error
    where
        B: Into<Vec<u8>>,
    {
        Error {
            kind: ErrorKind::SystemUnsupportedGrouping(bytes.into()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

#[cfg(feature = "std")]
mod standard {
    use super::*;

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }
}
