use crate::combinator::trace;
use crate::error::ParserError;
use crate::stream::Stream;
use crate::{Parser, Result};

/// Helper trait for the [`alt()`] combinator.
///
/// This trait is implemented for tuples of up to 21 elements
pub trait Alt<I, O, E> {
    /// Tests each parser in the tuple and returns the result of the first one that succeeds
    fn choice(&mut self, input: &mut I) -> Result<O, E>;
}

/// Pick the first successful parser
///
/// To stop on an error, rather than trying further cases, see
/// [`cut_err`][crate::combinator::cut_err] ([example][crate::_tutorial::chapter_7]).
///
/// For tight control over the error when no match is found, add a final case using [`fail`][crate::combinator::fail].
/// Alternatively, with a [custom error type][crate::_topic::error], it is possible to track all
/// errors or return the error of the parser that went the farthest in the input data.
///
/// When the alternative cases have unique prefixes, [`dispatch`][crate::combinator::dispatch] can offer better performance.
///
/// # Example
///
/// ```rust
/// # #[cfg(feature = "ascii")] {
/// # use winnow::{error::ErrMode, error::Needed};
/// # use winnow::prelude::*;
/// use winnow::ascii::{alpha1, digit1};
/// use winnow::combinator::alt;
/// fn parser<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
///   alt((alpha1, digit1)).parse_next(input)
/// };
///
/// // the first parser, alpha1, takes the input
/// assert_eq!(parser.parse_peek("abc"), Ok(("", "abc")));
///
/// // the first parser returns an error, so alt tries the second one
/// assert_eq!(parser.parse_peek("123456"), Ok(("", "123456")));
///
/// // both parsers failed, and with the default error type, alt will return the last error
/// assert!(parser.parse_peek(" ").is_err());
/// # }
/// ```
#[doc(alias = "choice")]
#[inline(always)]
pub fn alt<Input: Stream, Output, Error, Alternatives>(
    mut alternatives: Alternatives,
) -> impl Parser<Input, Output, Error>
where
    Alternatives: Alt<Input, Output, Error>,
    Error: ParserError<Input>,
{
    trace("alt", move |i: &mut Input| alternatives.choice(i))
}

impl<const N: usize, I: Stream, O, E: ParserError<I>, P: Parser<I, O, E>> Alt<I, O, E> for [P; N] {
    fn choice(&mut self, input: &mut I) -> Result<O, E> {
        let mut error: Option<E> = None;

        let start = input.checkpoint();
        for branch in self {
            input.reset(&start);
            match branch.parse_next(input) {
                Err(e) if e.is_backtrack() => {
                    error = match error {
                        Some(error) => Some(error.or(e)),
                        None => Some(e),
                    };
                }
                res => return res,
            }
        }

        match error {
            Some(e) => Err(e.append(input, &start)),
            None => Err(ParserError::assert(
                input,
                "`alt` needs at least one parser",
            )),
        }
    }
}

impl<I: Stream, O, E: ParserError<I>, P: Parser<I, O, E>> Alt<I, O, E> for &mut [P] {
    fn choice(&mut self, input: &mut I) -> Result<O, E> {
        let mut error: Option<E> = None;

        let start = input.checkpoint();
        for branch in self.iter_mut() {
            input.reset(&start);
            match branch.parse_next(input) {
                Err(e) if e.is_backtrack() => {
                    error = match error {
                        Some(error) => Some(error.or(e)),
                        None => Some(e),
                    };
                }
                res => return res,
            }
        }

        match error {
            Some(e) => Err(e.append(input, &start)),
            None => Err(ParserError::assert(
                input,
                "`alt` needs at least one parser",
            )),
        }
    }
}

macro_rules! alt_trait(
  ($first:ident $second:ident $($id: ident)+) => (
    alt_trait!(__impl $first $second; $($id)+);
  );
  (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
    alt_trait_impl!($($current)*);

    alt_trait!(__impl $($current)* $head; $($id)+);
  );
  (__impl $($current:ident)*; $head:ident) => (
    alt_trait_impl!($($current)*);
    alt_trait_impl!($($current)* $head);
  );
);

macro_rules! alt_trait_impl(
  ($($id:ident)+) => (
    impl<
      I: Stream, Output, Error: ParserError<I>,
      $($id: Parser<I, Output, Error>),+
    > Alt<I, Output, Error> for ( $($id),+ ) {

      fn choice(&mut self, input: &mut I) -> Result<Output, Error> {
        let start = input.checkpoint();
        match self.0.parse_next(input) {
          Err(e) if e.is_backtrack() => alt_trait_inner!(1, self, input, start, e, $($id)+),
          res => res,
        }
      }
    }
  );
);

macro_rules! succ (
    (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
    (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
    (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
    (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
    (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
    (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
    (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
    (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
);

macro_rules! alt_trait_inner(
    ($it:tt, $self:expr, $input:expr, $start:ident, $err:expr, $head:ident $($id:ident)+) => ({
        $input.reset(&$start);
        match $self.$it.parse_next($input) {
            Err(e) if e.is_backtrack() => {
                let err = $err.or(e);
                succ!($it, alt_trait_inner!($self, $input, $start, err, $($id)+))
            }
            res => res,
        }
    });
    ($it:tt, $self:expr, $input:expr, $start:ident, $err:expr, $head:ident) => ({
        Err($err.append($input, &$start))
    });
);

alt_trait!(Alt2 Alt3 Alt4 Alt5 Alt6 Alt7 Alt8 Alt9 Alt10);

// Manually implement Alt for (A,), the 1-tuple type
impl<I: Stream, O, E: ParserError<I>, A: Parser<I, O, E>> Alt<I, O, E> for (A,) {
    fn choice(&mut self, input: &mut I) -> Result<O, E> {
        self.0.parse_next(input)
    }
}
