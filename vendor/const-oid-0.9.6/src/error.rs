//! Error types

use crate::Arc;
use core::fmt;

/// Result type
pub type Result<T> = core::result::Result<T, Error>;

/// OID errors.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    /// Arc exceeds allowed range (i.e. for first or second OID)
    ArcInvalid {
        /// Arc value that is erroneous.
        arc: Arc,
    },

    /// Arc is too big (exceeds 32-bit limits of this library).
    ///
    /// Technically the size of an arc is not constrained by X.660, however
    /// this library has elected to use `u32` as the arc representation as
    /// sufficient for PKIX/PKCS usages.
    ArcTooBig,

    /// Base 128 encoding error (used in BER/DER serialization of arcs).
    Base128,

    /// Expected a digit, but was provided something else.
    DigitExpected {
        /// What was found instead of a digit
        actual: u8,
    },

    /// Input data is empty.
    Empty,

    /// OID length is invalid (too short or too long).
    Length,

    /// Minimum 3 arcs required.
    NotEnoughArcs,

    /// Trailing `.` character at end of input.
    TrailingDot,
}

impl Error {
    /// Escalate this error into a panic.
    ///
    /// This is a workaround until `Result::unwrap` is allowed in `const fn`.
    #[allow(clippy::panic)]
    pub(crate) const fn panic(self) -> ! {
        match self {
            Error::ArcInvalid { .. } | Error::ArcTooBig => panic!("OID contains invalid arc"),
            Error::Base128 => panic!("OID contains arc with invalid base 128 encoding"),
            Error::DigitExpected { .. } => panic!("OID expected to start with digit"),
            Error::Empty => panic!("OID value is empty"),
            Error::Length => panic!("OID length invalid"),
            Error::NotEnoughArcs => panic!("OID requires minimum of 3 arcs"),
            Error::TrailingDot => panic!("OID ends with invalid trailing '.'"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ArcInvalid { arc } => write!(f, "OID contains out-of-range arc: {}", arc),
            Error::ArcTooBig => f.write_str("OID contains arc which is larger than 32-bits"),
            Error::Base128 => f.write_str("OID contains arc with invalid base 128 encoding"),
            Error::DigitExpected { actual } => {
                write!(f, "expected digit, got '{}'", char::from(actual))
            }
            Error::Empty => f.write_str("OID value is empty"),
            Error::Length => f.write_str("OID length invalid"),
            Error::NotEnoughArcs => f.write_str("OID requires minimum of 3 arcs"),
            Error::TrailingDot => f.write_str("OID ends with invalid trailing '.'"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
