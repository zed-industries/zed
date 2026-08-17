//! Error types

use core::fmt;

/// Result type with the `pem-rfc7468` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// PEM errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Base64-related errors.
    Base64(base64ct::Error),

    /// Character encoding-related errors.
    CharacterEncoding,

    /// Errors in the encapsulated text (which aren't specifically Base64-related).
    EncapsulatedText,

    /// Header detected in the encapsulated text.
    HeaderDisallowed,

    /// Invalid label.
    Label,

    /// Invalid length.
    Length,

    /// "Preamble" (text before pre-encapsulation boundary) contains invalid data.
    Preamble,

    /// Errors in the pre-encapsulation boundary.
    PreEncapsulationBoundary,

    /// Errors in the post-encapsulation boundary.
    PostEncapsulationBoundary,

    /// Unexpected PEM type label.
    UnexpectedTypeLabel {
        /// Type label that was expected.
        expected: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Base64(err) => write!(f, "PEM Base64 error: {}", err),
            Error::CharacterEncoding => f.write_str("PEM character encoding error"),
            Error::EncapsulatedText => f.write_str("PEM error in encapsulated text"),
            Error::HeaderDisallowed => f.write_str("PEM headers disallowed by RFC7468"),
            Error::Label => f.write_str("PEM type label invalid"),
            Error::Length => f.write_str("PEM length invalid"),
            Error::Preamble => f.write_str("PEM preamble contains invalid data (NUL byte)"),
            Error::PreEncapsulationBoundary => {
                f.write_str("PEM error in pre-encapsulation boundary")
            }
            Error::PostEncapsulationBoundary => {
                f.write_str("PEM error in post-encapsulation boundary")
            }
            Error::UnexpectedTypeLabel { expected } => {
                write!(f, "unexpected PEM type label: expecting \"{}\"", expected)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<base64ct::Error> for Error {
    fn from(err: base64ct::Error) -> Error {
        Error::Base64(err)
    }
}

impl From<base64ct::InvalidLengthError> for Error {
    fn from(_: base64ct::InvalidLengthError) -> Error {
        Error::Length
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(_: core::str::Utf8Error) -> Error {
        Error::CharacterEncoding
    }
}

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    fn from(err: Error) -> std::io::Error {
        let kind = match err {
            Error::Base64(err) => return err.into(), // Use existing conversion
            Error::CharacterEncoding
            | Error::EncapsulatedText
            | Error::Label
            | Error::Preamble
            | Error::PreEncapsulationBoundary
            | Error::PostEncapsulationBoundary => std::io::ErrorKind::InvalidData,
            Error::Length => std::io::ErrorKind::UnexpectedEof,
            _ => std::io::ErrorKind::Other,
        };

        std::io::Error::new(kind, err)
    }
}
