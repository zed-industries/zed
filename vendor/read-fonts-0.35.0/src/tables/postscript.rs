//! PostScript (CFF and CFF2) common tables.

use std::fmt;

mod blend;
mod charset;
mod encoding;
mod fd_select;
mod index;
mod stack;
mod string;

pub mod charstring;
pub mod dict;

include!("../../generated/generated_postscript.rs");

pub use blend::BlendState;
pub use charset::{Charset, CharsetIter};
pub use index::Index;
pub use stack::{Number, Stack};
pub use string::{Latin1String, StringId, STANDARD_STRINGS};

/// Errors that are specific to PostScript processing.
#[derive(Clone, Debug)]
pub enum Error {
    InvalidIndexOffsetSize(u8),
    ZeroOffsetInIndex,
    InvalidVariationStoreIndex(u16),
    StackOverflow,
    StackUnderflow,
    InvalidStackAccess(usize),
    ExpectedI32StackEntry(usize),
    InvalidNumber,
    InvalidDictOperator(u8),
    InvalidCharstringOperator(u8),
    CharstringNestingDepthLimitExceeded,
    MissingSubroutines,
    MissingBlendState,
    MissingPrivateDict,
    MissingCharstrings,
    MissingCharset,
    InvalidSeacCode(i32),
    Read(ReadError),
}

impl From<ReadError> for Error {
    fn from(value: ReadError) -> Self {
        Self::Read(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIndexOffsetSize(size) => {
                write!(f, "invalid offset size of {size} for INDEX (expected 1-4)")
            }
            Self::ZeroOffsetInIndex => {
                write!(f, "invalid offset of 0 in INDEX (must be >= 1)")
            }
            Self::InvalidVariationStoreIndex(index) => {
                write!(
                    f,
                    "variation store index {index} referenced an invalid variation region"
                )
            }
            Self::StackOverflow => {
                write!(f, "attempted to push a value to a full stack")
            }
            Self::StackUnderflow => {
                write!(f, "attempted to pop a value from an empty stack")
            }
            Self::InvalidStackAccess(index) => {
                write!(f, "invalid stack access for index {index}")
            }
            Self::ExpectedI32StackEntry(index) => {
                write!(f, "attempted to read an integer at stack index {index}, but found a fixed point value")
            }
            Self::InvalidNumber => {
                write!(f, "number is in an invalid format")
            }
            Self::InvalidDictOperator(operator) => {
                write!(f, "dictionary operator {operator} is invalid")
            }
            Self::InvalidCharstringOperator(operator) => {
                write!(f, "charstring operator {operator} is invalid")
            }
            Self::CharstringNestingDepthLimitExceeded => {
                write!(
                    f,
                    "exceeded subroutine nesting depth limit {} while evaluating a charstring",
                    charstring::NESTING_DEPTH_LIMIT
                )
            }
            Self::MissingSubroutines => {
                write!(
                    f,
                    "encountered a callsubr operator but no subroutine index was provided"
                )
            }
            Self::MissingBlendState => {
                write!(
                    f,
                    "encountered a blend operator but no blend state was provided"
                )
            }
            Self::MissingPrivateDict => {
                write!(f, "CFF table does not contain a private dictionary")
            }
            Self::MissingCharstrings => {
                write!(f, "CFF table does not contain a charstrings index")
            }
            Self::MissingCharset => {
                write!(f, "CFF table does not contain a valid charset")
            }
            Self::InvalidSeacCode(code) => {
                write!(f, "seac code {code} is not valid")
            }
            Self::Read(err) => write!(f, "{err}"),
        }
    }
}
