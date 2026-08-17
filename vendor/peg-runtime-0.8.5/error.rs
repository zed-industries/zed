//! Parse error reporting

use crate::{Parse, RuleResult};
use std::fmt::{self, Debug, Display};

#[cfg(feature = "std")]
use std::collections::BTreeSet;

#[cfg(not(feature = "std"))]
use {alloc::collections::BTreeSet, alloc::vec::Vec};

/// A set of literals or names that failed to match
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExpectedSet {
    expected: BTreeSet<&'static str>,
}

impl ExpectedSet {
    /// Iterator of expected literals
    pub fn tokens<'a>(&'a self) -> impl Iterator<Item = &'static str> + 'a {
        self.expected.iter().map(|x| *x)
    }
}

impl Display for ExpectedSet {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.expected.is_empty() {
            write!(fmt, "<unreported>")?;
        } else if self.expected.len() == 1 {
            write!(fmt, "{}", self.expected.iter().next().unwrap())?;
        } else {
            let mut errors = self.tokens().collect::<Vec<_>>();
            errors.sort();
            let mut iter = errors.into_iter();

            write!(fmt, "one of {}", iter.next().unwrap())?;
            for elem in iter {
                write!(fmt, ", {}", elem)?;
            }
        }

        Ok(())
    }
}

/// A parse failure.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParseError<L> {
    /// The furthest position the parser reached in the input before failing.
    pub location: L,

    /// The set of literals that failed to match at that position.
    pub expected: ExpectedSet,
}

impl<L: Display> Display for ParseError<L> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        write!(
            fmt,
            "error at {}: expected {}",
            self.location, self.expected
        )
    }
}

#[cfg(any(feature = "std", feature = "unstable"))]
impl<L: Display + Debug> ::std::error::Error for ParseError<L> {
    fn description(&self) -> &str {
        "parse error"
    }
}

#[doc(hidden)]
pub struct ErrorState {
    /// Furthest failure we've hit so far.
    pub max_err_pos: usize,

    /// Are we inside a lookahead/quiet block? If so, failure is disabled.
    /// Non-zero => yes, to support nested blocks.
    pub suppress_fail: usize,

    /// Are we reparsing after a failure? If so, compute and store expected set of all alternative expectations
    /// when we are at offset `max_err_pos`.
    pub reparsing_on_error: bool,

    /// The set of tokens we expected to find when we hit the failure. Updated when `reparsing_on_error`.
    pub expected: ExpectedSet,
}

impl ErrorState {
    pub fn new(initial_pos: usize) -> Self {
        ErrorState {
            max_err_pos: initial_pos,
            suppress_fail: 0,
            reparsing_on_error: false,
            expected: ExpectedSet {
                expected: BTreeSet::new(),
            },
        }
    }

    /// Set up for reparsing to record the details of the furthest failure.
    pub fn reparse_for_error(&mut self) {
        self.suppress_fail = 0;
        self.reparsing_on_error = true;
    }

    #[inline(never)]
    pub fn mark_failure_slow_path(&mut self, pos: usize, expected: &'static str) {
        if pos == self.max_err_pos {
            self.expected.expected.insert(expected);
        }
    }

    /// Flag a failure.
    #[inline(always)]
    pub fn mark_failure(&mut self, pos: usize, expected: &'static str) -> RuleResult<()> {
        if self.suppress_fail == 0 {
            if self.reparsing_on_error {
                self.mark_failure_slow_path(pos, expected);
            } else if pos > self.max_err_pos {
                self.max_err_pos = pos;
            }
        }
        RuleResult::Failed
    }

    pub fn into_parse_error<I: Parse + ?Sized>(self, input: &I) -> ParseError<I::PositionRepr> {
        ParseError {
            location: Parse::position_repr(input, self.max_err_pos.into()),
            expected: self.expected,
        }
    }
}
