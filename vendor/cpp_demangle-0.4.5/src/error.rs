//! Custom `Error` and `Result` types for the `cpp_demangle` crate.

use core::fmt;
#[cfg(feature = "std")]
use std::error;

/// Errors that can occur while demangling a symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// The mangled symbol ends abruptly.
    UnexpectedEnd,

    /// The mangled symbol is not well-formed.
    UnexpectedText,

    /// Found a back reference that is out-of-bounds of the substitution
    /// table.
    BadBackReference,

    /// Found a reference to a template arg that is either out-of-bounds, or in
    /// a context without template args.
    BadTemplateArgReference,

    /// Found a reference to a template arg from within the arg itself (or from
    /// within an earlier arg).
    ForwardTemplateArgReference,

    /// Found a reference to a function arg that is either out-of-bounds, or in
    /// a context without function args.
    BadFunctionArgReference,

    /// Found a reference to a leaf name in a context where there is no current
    /// leaf name.
    BadLeafNameReference,

    /// An overflow or underflow would occur when parsing an integer in a
    /// mangled symbol.
    Overflow,

    /// Encountered too much recursion when demangling symbol.
    TooMuchRecursion,
}

#[test]
fn size_of_error() {
    assert_eq!(
        core::mem::size_of::<Error>(),
        1,
        "We should keep the size of our Error type in check"
    );
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedEnd => write!(f, "mangled symbol ends abruptly"),
            Error::UnexpectedText => write!(f, "mangled symbol is not well-formed"),
            Error::BadBackReference => write!(
                f,
                "back reference that is out-of-bounds of the substitution table"
            ),
            Error::BadTemplateArgReference => write!(
                f,
                "reference to a template arg that is either out-of-bounds, or in a context \
                 without template args"
            ),
            Error::ForwardTemplateArgReference => write!(
                f,
                "reference to a template arg from itself or a later template arg"
            ),
            Error::BadFunctionArgReference => write!(
                f,
                "reference to a function arg that is either out-of-bounds, or in a context \
                 without function args"
            ),
            Error::BadLeafNameReference => write!(
                f,
                "reference to a leaf name in a context where there is no current leaf name"
            ),
            Error::Overflow => write!(
                f,
                "an overflow or underflow would occur when parsing an integer in a mangled \
                 symbol"
            ),
            Error::TooMuchRecursion => {
                write!(f, "encountered too much recursion when demangling symbol")
            }
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnexpectedEnd => "mangled symbol ends abruptly",
            Error::UnexpectedText => "mangled symbol is not well-formed",
            Error::BadBackReference => {
                "back reference that is out-of-bounds of the substitution table"
            }
            Error::BadTemplateArgReference => {
                "reference to a template arg that is either out-of-bounds, or in a context \
                 without template args"
            }
            Error::ForwardTemplateArgReference => {
                "reference to a template arg from itself or a later template arg"
            }
            Error::BadFunctionArgReference => {
                "reference to a function arg that is either out-of-bounds, or in a context \
                 without function args"
            }
            Error::BadLeafNameReference => {
                "reference to a leaf name in a context where there is no current leaf name"
            }
            Error::Overflow => {
                "an overflow or underflow would occur when parsing an integer in a mangled symbol"
            }
            Error::TooMuchRecursion => "encountered too much recursion when demangling symbol",
        }
    }
}

/// A demangling result of `T` or a `cpp_demangle::error::Error`.
pub type Result<T> = ::core::result::Result<T, Error>;
