use proc_macro2::Span;

use std::{
    fmt::{self, Display},
    ops::Range,
};

#[derive(Debug, PartialEq)]
pub(crate) struct ParseError {
    pub(crate) pos: usize,
    pub(crate) kind: ParseErrorKind,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParseErrorKind {
    /// A `{` that wasn't closed.
    UnclosedArg,
    /// A `}` that doesn't close an argument.
    InvalidClosedArg,
    /// When parsing the number of a positional arguments
    NotANumber {
        what: String,
    },
    /// When parsing the identifier of a named argument
    NotAnIdent {
        what: String,
    },
    UnknownFormatting {
        what: String,
    },
}

#[allow(dead_code)]
impl ParseErrorKind {
    pub fn not_a_number(what: &str) -> Self {
        Self::NotANumber {
            what: what.to_string(),
        }
    }
    pub fn not_an_ident(what: &str) -> Self {
        Self::NotAnIdent {
            what: what.to_string(),
        }
    }
    pub fn unknown_formatting(what: &str) -> Self {
        Self::UnknownFormatting {
            what: what.to_string(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub(crate) struct DisplayParseError<'a> {
    pub(crate) str: &'a str,
    pub(crate) error_span: Range<usize>,
    pub(crate) kind: ParseErrorKind,
}
impl ParseError {
    fn error_span(&self) -> Range<usize> {
        let len = match &self.kind {
            ParseErrorKind::UnclosedArg => 0,
            ParseErrorKind::InvalidClosedArg => 0,
            ParseErrorKind::NotANumber { what } => what.len(),
            ParseErrorKind::NotAnIdent { what } => what.len(),
            ParseErrorKind::UnknownFormatting { what } => what.len(),
        };

        self.pos..self.pos + len
    }

    pub(crate) fn into_crate_err(self, span: Span, original_str: &str) -> crate::Error {
        let display = DisplayParseError {
            str: original_str,
            error_span: self.error_span(),
            kind: self.kind,
        };

        crate::Error::new(span, display)
    }
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::UnclosedArg => f.write_str("unclosed argument"),
            ParseErrorKind::InvalidClosedArg => f.write_str("`}` closing a nonexistent argument"),
            ParseErrorKind::NotANumber { what } => writeln!(f, "not a number: \"{}\"", what),
            ParseErrorKind::NotAnIdent { what } => {
                writeln!(f, "not a valid identifier: \"{}\"", what)
            }
            ParseErrorKind::UnknownFormatting { what } => {
                writeln!(f, "unknown formatting: \"{}\"", what)
            }
        }
    }
}

impl Display for DisplayParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("failed to parse the format string ")?;

        // Gets the amount of chars up to the error,
        // this is good enough for most cases,
        // but doesn't acount for multi-char characters.
        let chars = self.str[..self.error_span.start].chars().count();
        writeln!(f, "at the character number {}, ", chars)?;

        Display::fmt(&self.kind, f)?;

        Ok(())
    }
}
