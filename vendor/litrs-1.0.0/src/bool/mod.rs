use std::fmt;

use crate::{
    err::{perr, ParseErrorKind::*},
    ParseError,
};


/// A bool literal: `true` or `false`. Also see [the reference][ref].
///
/// Notice that, strictly speaking, from Rust point of view "boolean literals" are not
/// actual literals but [keywords].
///
/// [ref]: https://doc.rust-lang.org/reference/expressions/literal-expr.html#boolean-literal-expressions
/// [keywords]: https://doc.rust-lang.org/reference/keywords.html#strict-keywords
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolLit {
    False,
    True,
}

impl BoolLit {
    /// Parses the input as a bool literal. Returns an error if the input is
    /// invalid or represents a different kind of literal.
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        match s {
            "false" => Ok(Self::False),
            "true" => Ok(Self::True),
            _ => Err(perr(None, InvalidLiteral)),
        }
    }

    /// Returns the actual Boolean value of this literal.
    pub fn value(self) -> bool {
        match self {
            Self::False => false,
            Self::True => true,
        }
    }

    /// Returns the literal as string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::False => "false",
            Self::True => "true",
        }
    }
}

impl fmt::Display for BoolLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_str())
    }
}


#[cfg(test)]
mod tests;
