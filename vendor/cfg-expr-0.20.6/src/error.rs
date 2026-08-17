use std::{error::Error, fmt};

/// An error related to parsing of a cfg expression
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    /// The string that was parsed
    pub original: String,
    /// The range of characters in the original string that result
    /// in this error
    pub span: std::ops::Range<usize>,
    /// The specific reason for the error
    pub reason: Reason,
}

/// The particular reason for a `ParseError`
#[derive(Debug, PartialEq, Eq)]
pub enum Reason {
    /// `not()` takes exactly 1 predicate, unlike `all()` and `any()`
    InvalidNot(usize),
    /// The characters are not valid in an cfg expression
    InvalidCharacters,
    /// An opening parens was unmatched with a closing parens
    UnclosedParens,
    /// A closing parens was unmatched with an opening parens
    UnopenedParens,
    /// An opening quotes was unmatched with a closing quotes
    UnclosedQuotes,
    /// A closing quotes was unmatched with an opening quotes
    UnopenedQuotes,
    /// The expression does not contain any valid terms
    Empty,
    /// Found an unexpected term, which wasn't one of the expected terms that
    /// is listed
    Unexpected(&'static [&'static str]),
    /// Failed to parse an integer value
    InvalidInteger,
    /// The root `cfg()` may only contain a single predicate
    MultipleRootPredicates,
    /// A `target_has_atomic` predicate didn't correctly parse.
    InvalidHasAtomic,
    /// An element was not part of the builtin information in rustc
    UnknownBuiltin,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.original)?;
        f.write_str("\n")?;

        for _ in 0..self.span.start {
            f.write_str(" ")?;
        }

        // Mismatched parens/quotes have a slightly different output
        // than the other errors
        match &self.reason {
            r @ (Reason::UnclosedParens | Reason::UnclosedQuotes) => {
                f.write_fmt(format_args!("- {r}"))
            }
            r @ (Reason::UnopenedParens | Reason::UnopenedQuotes) => {
                f.write_fmt(format_args!("^ {r}"))
            }
            other => {
                for _ in self.span.start..self.span.end {
                    f.write_str("^")?;
                }

                f.write_fmt(format_args!(" {other}"))
            }
        }
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Reason::{
            Empty, InvalidCharacters, InvalidHasAtomic, InvalidInteger, InvalidNot,
            MultipleRootPredicates, UnclosedParens, UnclosedQuotes, Unexpected, UnknownBuiltin,
            UnopenedParens, UnopenedQuotes,
        };

        match self {
            InvalidCharacters => f.write_str("invalid character(s)"),
            UnclosedParens => f.write_str("unclosed parens"),
            UnopenedParens => f.write_str("unopened parens"),
            UnclosedQuotes => f.write_str("unclosed quotes"),
            UnopenedQuotes => f.write_str("unopened quotes"),
            Empty => f.write_str("empty expression"),
            Unexpected(expected) => {
                if expected.len() > 1 {
                    f.write_str("expected one of ")?;

                    for (i, exp) in expected.iter().enumerate() {
                        f.write_fmt(format_args!("{}`{exp}`", if i > 0 { ", " } else { "" }))?;
                    }
                    f.write_str(" here")
                } else if !expected.is_empty() {
                    f.write_fmt(format_args!("expected a `{}` here", expected[0]))
                } else {
                    f.write_str("the term was not expected here")
                }
            }
            InvalidNot(np) => f.write_fmt(format_args!("not() takes 1 predicate, found {np}")),
            InvalidInteger => f.write_str("invalid integer"),
            MultipleRootPredicates => f.write_str("multiple root predicates"),
            InvalidHasAtomic => f.write_str("expected integer or \"ptr\""),
            UnknownBuiltin => f.write_str("unknown built-in"),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        use Reason::{
            Empty, InvalidCharacters, InvalidHasAtomic, InvalidInteger, InvalidNot,
            MultipleRootPredicates, UnclosedParens, UnclosedQuotes, Unexpected, UnknownBuiltin,
            UnopenedParens, UnopenedQuotes,
        };

        match self.reason {
            InvalidCharacters => "invalid character(s)",
            UnclosedParens => "unclosed parens",
            UnopenedParens => "unopened parens",
            UnclosedQuotes => "unclosed quotes",
            UnopenedQuotes => "unopened quotes",
            Empty => "empty expression",
            Unexpected(_) => "unexpected term",
            InvalidNot(_) => "not() takes 1 predicate",
            InvalidInteger => "invalid integer",
            MultipleRootPredicates => "multiple root predicates",
            InvalidHasAtomic => "expected integer or \"ptr\"",
            UnknownBuiltin => "unknown built-in",
        }
    }
}

/// Error parsing a `target_has_atomic` predicate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HasAtomicParseError {
    pub(crate) input: String,
}

impl fmt::Display for HasAtomicParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expected integer or \"ptr\", found {}", self.input)
    }
}

impl Error for HasAtomicParseError {}
