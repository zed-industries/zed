use crate::Decimal;
use alloc::string::String;
use core::fmt;

/// Error type for the library.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// A generic error from Rust Decimal with the `String` containing more information as to what
    /// went wrong.
    ///
    /// This is a legacy/deprecated error type retained for backwards compatibility.  
    ErrorString(String),
    /// The value provided exceeds `Decimal::MAX`.
    ExceedsMaximumPossibleValue,
    /// The value provided is less than `Decimal::MIN`.
    LessThanMinimumPossibleValue,
    /// An underflow is when there are more fractional digits than can be represented within `Decimal`.
    Underflow,
    /// The scale provided exceeds the maximum scale that `Decimal` can represent.
    ScaleExceedsMaximumPrecision(u32),
    /// Represents a failure to convert to/from `Decimal` to the specified type. This is typically
    /// due to type constraints (e.g. `Decimal::MAX` cannot be converted into `i32`).
    ConversionTo(String),
}

impl<S> From<S> for Error
where
    S: Into<String>,
{
    #[inline]
    fn from(from: S) -> Self {
        Self::ErrorString(from.into())
    }
}

#[cold]
pub(crate) fn tail_error(from: &'static str) -> Result<Decimal, Error> {
    Err(from.into())
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ErrorString(ref err) => f.pad(err),
            Self::ExceedsMaximumPossibleValue => {
                write!(f, "Number exceeds maximum value that can be represented.")
            }
            Self::LessThanMinimumPossibleValue => {
                write!(f, "Number less than minimum value that can be represented.")
            }
            Self::Underflow => {
                write!(f, "Number has a high precision that can not be represented.")
            }
            Self::ScaleExceedsMaximumPrecision(ref scale) => {
                write!(
                    f,
                    "Scale exceeds the maximum precision allowed: {scale} > {}",
                    Decimal::MAX_SCALE
                )
            }
            Self::ConversionTo(ref type_name) => {
                write!(f, "Error while converting to {type_name}")
            }
        }
    }
}
