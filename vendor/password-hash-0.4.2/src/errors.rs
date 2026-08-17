//! Error types.

pub use base64ct::Error as B64Error;

use core::fmt;

/// Result type.
pub type Result<T> = core::result::Result<T, Error>;

/// Password hashing errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Unsupported algorithm.
    Algorithm,

    /// "B64" encoding error.
    B64Encoding(B64Error),

    /// Cryptographic error.
    Crypto,

    /// Output too short (min 10-bytes).
    OutputTooShort,

    /// Output too long (max 64-bytes).
    OutputTooLong,

    /// Duplicate parameter name encountered.
    ParamNameDuplicated,

    /// Invalid parameter name.
    ParamNameInvalid,

    /// Invalid parameter value.
    ParamValueInvalid(InvalidValue),

    /// Maximum number of parameters exceeded.
    ParamsMaxExceeded,

    /// Invalid password.
    Password,

    /// Password hash string contains invalid characters.
    PhcStringInvalid,

    /// Password hash string too short.
    PhcStringTooShort,

    /// Password hash string too long.
    PhcStringTooLong,

    /// Salt invalid.
    SaltInvalid(InvalidValue),

    /// Invalid algorithm version.
    Version,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error> {
        match self {
            Self::Algorithm => write!(f, "unsupported algorithm"),
            Self::B64Encoding(err) => write!(f, "{}", err),
            Self::Crypto => write!(f, "cryptographic error"),
            Self::OutputTooShort => f.write_str("PHF output too short (min 10-bytes)"),
            Self::OutputTooLong => f.write_str("PHF output too long (max 64-bytes)"),
            Self::ParamNameDuplicated => f.write_str("duplicate parameter"),
            Self::ParamNameInvalid => f.write_str("invalid parameter name"),
            Self::ParamValueInvalid(val_err) => write!(f, "invalid parameter value: {}", val_err),
            Self::ParamsMaxExceeded => f.write_str("maximum number of parameters reached"),
            Self::Password => write!(f, "invalid password"),
            Self::PhcStringInvalid => write!(f, "password hash string invalid"),
            Self::PhcStringTooShort => write!(f, "password hash string too short"),
            Self::PhcStringTooLong => write!(f, "password hash string too long"),
            Self::SaltInvalid(val_err) => write!(f, "salt invalid: {}", val_err),
            Self::Version => write!(f, "invalid algorithm version"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<B64Error> for Error {
    fn from(err: B64Error) -> Error {
        Error::B64Encoding(err)
    }
}

impl From<base64ct::InvalidLengthError> for Error {
    fn from(_: base64ct::InvalidLengthError) -> Error {
        Error::B64Encoding(B64Error::InvalidLength)
    }
}

/// Parse errors relating to invalid parameter values or salts.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvalidValue {
    /// Character is not in the allowed set.
    InvalidChar(char),

    /// Format is invalid.
    InvalidFormat,

    /// Value is malformed.
    Malformed,

    /// Value exceeds the maximum allowed length.
    TooLong,

    /// Value does not satisfy the minimum length.
    TooShort,
}

impl InvalidValue {
    /// Create an [`Error::ParamValueInvalid`] which warps this error.
    pub fn param_error(self) -> Error {
        Error::ParamValueInvalid(self)
    }

    /// Create an [`Error::SaltInvalid`] which wraps this error.
    pub fn salt_error(self) -> Error {
        Error::SaltInvalid(self)
    }
}

impl fmt::Display for InvalidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error> {
        match self {
            Self::InvalidChar(c) => write!(f, "contains invalid character: '{}'", c),
            Self::InvalidFormat => f.write_str("value format is invalid"),
            Self::Malformed => f.write_str("value malformed"),
            Self::TooLong => f.write_str("value to long"),
            Self::TooShort => f.write_str("value to short"),
        }
    }
}
