//! General purpose combinators

#![allow(unused_imports)]

use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use crate::lib::std::boxed::Box;

use crate::error::{ErrorKind, FromExternalError, ParseError};
use crate::internal::*;
use crate::lib::std::borrow::Borrow;
use crate::lib::std::convert::Into;
#[cfg(feature = "std")]
use crate::lib::std::fmt::Debug;
use crate::lib::std::mem::transmute;
use crate::lib::std::ops::{Range, RangeFrom, RangeTo};
use crate::traits::{AsChar, Input, ParseTo};
use crate::traits::{Compare, CompareResult, Offset};

#[cfg(test)]
mod tests;

/// Return the remaining input.
///
/// ```rust
/// # use nom::error::ErrorKind;
/// use nom::combinator::rest;
/// assert_eq!(rest::<_,(_, ErrorKind)>("abc"), Ok(("", "abc")));
/// assert_eq!(rest::<_,(_, ErrorKind)>(""), Ok(("", "")));
/// ```
#[inline]
pub fn rest<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: Input,
{
  Ok(input.take_split(input.input_len()))
}

/// Return the length of the remaining input.
///
/// ```rust
/// # use nom::error::ErrorKind;
/// use nom::combinator::rest_len;
/// assert_eq!(rest_len::<_,(_, ErrorKind)>("abc"), Ok(("abc", 3)));
/// assert_eq!(rest_len::<_,(_, ErrorKind)>(""), Ok(("", 0)));
/// ```
#[inline]
pub fn rest_len<T, E: ParseError<T>>(input: T) -> IResult<T, usize, E>
where
  T: Input,
{
  let len = input.input_len();
  Ok((input, len))
}

/// Maps a function on the result of a parser.
///
/// ```rust
/// use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::character::complete::digit1;
/// use nom::combinator::map;
/// # fn main() {
///
/// let mut parser = map(digit1, |s: &str| s.len());
///
/// // the parser will count how many characters were returned by digit1
/// assert_eq!(parser.parse("123456"), Ok(("", 6)));
///
/// // this will fail if digit1 fails
/// assert_eq!(parser.parse("abc"), Err(Err::Error(("abc", ErrorKind::Digit))));
/// # }
/// ```
pub fn map<I, O, E: ParseError<I>, F, G>(parser: F, f: G) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Error = E>,
  G: FnMut(<F as Parser<I>>::Output) -> O,
{
  parser.map(f)
}

/// Applies a function returning a `Result` over the result of a parser.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::character::complete::digit1;
/// use nom::combinator::map_res;
/// # fn main() {
///
/// let mut parse = map_res(digit1, |s: &str| s.parse::<u8>());
///
/// // the parser will convert the result of digit1 to a number
/// assert_eq!(parse.parse("123"), Ok(("", 123)));
///
/// // this will fail if digit1 fails
/// assert_eq!(parse.parse("abc"), Err(Err::Error(("abc", ErrorKind::Digit))));
///
/// // this will fail if the mapped function fails (a `u8` is too small to hold `123456`)
/// assert_eq!(parse.parse("123456"), Err(Err::Error(("123456", ErrorKind::MapRes))));
/// # }
/// ```
pub fn map_res<I: Clone, O, E: ParseError<I> + FromExternalError<I, E2>, E2, F, G>(
  parser: F,
  f: G,
) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Error = E>,
  G: FnMut(<F as Parser<I>>::Output) -> Result<O, E2>,
{
  parser.map_res(f)
}

/// Applies a function returning an `Option` over the result of a parser.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::character::complete::digit1;
/// use nom::combinator::map_opt;
/// # fn main() {
///
/// let mut parse = map_opt(digit1, |s: &str| s.parse::<u8>().ok());
///
/// // the parser will convert the result of digit1 to a number
/// assert_eq!(parse.parse("123"), Ok(("", 123)));
///
/// // this will fail if digit1 fails
/// assert_eq!(parse.parse("abc"), Err(Err::Error(("abc", ErrorKind::Digit))));
///
/// // this will fail if the mapped function fails (a `u8` is too small to hold `123456`)
/// assert_eq!(parse.parse("123456"), Err(Err::Error(("123456", ErrorKind::MapOpt))));
/// # }
/// ```
pub fn map_opt<I: Clone, O, E: ParseError<I>, F, G>(
  parser: F,
  f: G,
) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Error = E>,
  G: FnMut(<F as Parser<I>>::Output) -> Option<O>,
{
  parser.map_opt(f)
}

/// Applies a parser over the result of another one.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::character::complete::digit1;
/// use nom::bytes::complete::take;
/// use nom::combinator::map_parser;
/// # fn main() {
///
/// let mut parse = map_parser(take(5u8), digit1);
///
/// assert_eq!(parse.parse("12345"), Ok(("", "12345")));
/// assert_eq!(parse.parse("123ab"), Ok(("", "123")));
/// assert_eq!(parse.parse("123"), Err(Err::Error(("123", ErrorKind::Eof))));
/// # }
/// ```
pub fn map_parser<I, O, E: ParseError<I>, F, G>(
  parser: F,
  applied_parser: G,
) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Error = E>,
  G: Parser<<F as Parser<I>>::Output, Output = O, Error = E>,
{
  parser.and_then(applied_parser)
}

/// Creates a new parser from the output of the first parser, then apply that parser over the rest of the input.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::bytes::complete::take;
/// use nom::number::complete::u8;
/// use nom::combinator::flat_map;
/// # fn main() {
///
/// let mut parse = flat_map(u8, take);
///
/// assert_eq!(parse.parse(&[2, 0, 1, 2][..]), Ok((&[2][..], &[0, 1][..])));
/// assert_eq!(parse.parse(&[4, 0, 1, 2][..]), Err(Err::Error((&[0, 1, 2][..], ErrorKind::Eof))));
/// # }
/// ```
pub fn flat_map<I, O, E: ParseError<I>, F, G, H>(
  parser: F,
  applied_parser: G,
) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Error = E>,
  G: FnMut(<F as Parser<I>>::Output) -> H,
  H: Parser<I, Output = O, Error = E>,
{
  parser.flat_map(applied_parser)
}

/// Optional parser, will return `None` on [`Err::Error`].
///
/// To chain an error up, see [`cut`].
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::combinator::opt;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// fn parser(i: &str) -> IResult<&str, Option<&str>> {
///   opt(alpha1).parse(i)
/// }
///
/// assert_eq!(parser("abcd;"), Ok((";", Some("abcd"))));
/// assert_eq!(parser("123;"), Ok(("123;", None)));
/// # }
/// ```
pub fn opt<I: Clone, E: ParseError<I>, F>(
  f: F,
) -> impl Parser<I, Output = Option<<F as Parser<I>>::Output>, Error = E>
where
  F: Parser<I, Error = E>,
{
  Opt { parser: f }
}

/// Parser implementation for [opt]
pub struct Opt<F> {
  parser: F,
}

impl<I, F: Parser<I>> Parser<I> for Opt<F>
where
  I: Clone,
{
  type Output = Option<<F as Parser<I>>::Output>;

  type Error = <F as Parser<I>>::Error;

  #[inline(always)]
  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let i = input.clone();
    match self
      .parser
      .process::<OutputM<OM::Output, Check, OM::Incomplete>>(input)
    {
      Ok((i, o)) => Ok((i, OM::Output::map(o, Some))),
      Err(Err::Error(_)) => Ok((i, OM::Output::bind(|| None))),
      Err(Err::Failure(e)) => Err(Err::Failure(e)),
      Err(Err::Incomplete(i)) => Err(Err::Incomplete(i)),
    }
  }
}

/// Calls the parser if the condition is met.
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Parser};
/// use nom::combinator::cond;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// fn parser(b: bool, i: &str) -> IResult<&str, Option<&str>> {
///   cond(b, alpha1).parse(i)
/// }
///
/// assert_eq!(parser(true, "abcd;"), Ok((";", Some("abcd"))));
/// assert_eq!(parser(false, "abcd;"), Ok(("abcd;", None)));
/// assert_eq!(parser(true, "123;"), Err(Err::Error(Error::new("123;", ErrorKind::Alpha))));
/// assert_eq!(parser(false, "123;"), Ok(("123;", None)));
/// # }
/// ```
pub fn cond<I, E: ParseError<I>, F>(
  b: bool,
  f: F,
) -> impl Parser<I, Output = Option<<F as Parser<I>>::Output>, Error = E>
where
  F: Parser<I, Error = E>,
{
  Cond {
    parser: if b { Some(f) } else { None },
  }
}

/// Parser implementation for [cond]
pub struct Cond<F> {
  parser: Option<F>,
}

impl<I, F> Parser<I> for Cond<F>
where
  F: Parser<I>,
{
  type Output = Option<<F as Parser<I>>::Output>;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    match &mut self.parser {
      None => Ok((input, OM::Output::bind(|| None))),
      Some(f) => f
        .process::<OM>(input)
        .map(|(i, o)| (i, OM::Output::map(o, Some))),
    }
  }
}

/// Tries to apply its parser without consuming the input.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::combinator::peek;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = peek(alpha1);
///
/// assert_eq!(parser.parse("abcd;"), Ok(("abcd;", "abcd")));
/// assert_eq!(parser.parse("123;"), Err(Err::Error(("123;", ErrorKind::Alpha))));
/// # }
/// ```
pub fn peek<I: Clone, F>(
  parser: F,
) -> impl Parser<I, Output = <F as Parser<I>>::Output, Error = <F as Parser<I>>::Error>
where
  F: Parser<I>,
{
  Peek { parser }
}

/// Parsr implementation for [peek]
pub struct Peek<F> {
  parser: F,
}

impl<I, F> Parser<I> for Peek<F>
where
  I: Clone,
  F: Parser<I>,
{
  type Output = <F as Parser<I>>::Output;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let i = input.clone();
    match self.parser.process::<OM>(input) {
      Ok((_, o)) => Ok((i, o)),
      Err(e) => Err(e),
    }
  }
}

/// returns its input if it is at the end of input data
///
/// When we're at the end of the data, this combinator
/// will succeed
///
/// ```
/// # use std::str;
/// # use nom::{Err, error::ErrorKind, IResult};
/// # use nom::combinator::eof;
///
/// # fn main() {
/// let parser = eof;
/// assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Eof))));
/// assert_eq!(parser(""), Ok(("", "")));
/// # }
/// ```
pub fn eof<I: Input + Clone, E: ParseError<I>>(input: I) -> IResult<I, I, E> {
  if input.input_len() == 0 {
    let clone = input.clone();
    Ok((input, clone))
  } else {
    Err(Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
  }
}

/// Transforms Incomplete into `Error`.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::bytes::streaming::take;
/// use nom::combinator::complete;
/// # fn main() {
///
/// let mut parser = complete(take(5u8));
///
/// assert_eq!(parser.parse("abcdefg"), Ok(("fg", "abcde")));
/// assert_eq!(parser.parse("abcd"), Err(Err::Error(("abcd", ErrorKind::Complete))));
/// # }
/// ```
pub fn complete<I: Clone, O, E: ParseError<I>, F>(
  parser: F,
) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Output = O, Error = E>,
{
  MakeComplete { parser }
}

/// Parser implementation for [complete]
pub struct MakeComplete<F> {
  parser: F,
}

impl<I, F> Parser<I> for MakeComplete<F>
where
  I: Clone,
  F: Parser<I>,
{
  type Output = <F as Parser<I>>::Output;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let i = input.clone();

    match self
      .parser
      .process::<OutputM<OM::Output, OM::Error, Complete>>(input)
    {
      Err(Err::Incomplete(_)) => Err(Err::Error(OM::Error::bind(|| {
        <F as Parser<I>>::Error::from_error_kind(i, ErrorKind::Complete)
      }))),
      Err(e) => Err(e),
      Ok(o) => Ok(o),
    }
  }
}

/// Succeeds if all the input has been consumed by its child parser.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::combinator::all_consuming;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = all_consuming(alpha1);
///
/// assert_eq!(parser.parse("abcd"), Ok(("", "abcd")));
/// assert_eq!(parser.parse("abcd;"),Err(Err::Error((";", ErrorKind::Eof))));
/// assert_eq!(parser.parse("123abcd;"),Err(Err::Error(("123abcd;", ErrorKind::Alpha))));
/// # }
/// ```
pub fn all_consuming<I, E: ParseError<I>, F>(
  parser: F,
) -> impl Parser<I, Output = <F as Parser<I>>::Output, Error = E>
where
  I: Input,
  F: Parser<I, Error = E>,
{
  AllConsuming { parser }
}

/// Parser implementation for [all_consuming]
pub struct AllConsuming<F> {
  parser: F,
}

impl<I, F> Parser<I> for AllConsuming<F>
where
  I: Input,
  F: Parser<I>,
{
  type Output = <F as Parser<I>>::Output;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let (input, res) = self.parser.process::<OM>(input)?;
    if input.input_len() == 0 {
      Ok((input, res))
    } else {
      Err(Err::Error(OM::Error::bind(|| {
        <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::Eof)
      })))
    }
  }
}

/// Returns the result of the child parser if it satisfies a verification function.
///
/// The verification function takes as argument a reference to the output of the
/// parser.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::combinator::verify;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = verify(alpha1, |s: &str| s.len() == 4);
///
/// assert_eq!(parser.parse("abcd"), Ok(("", "abcd")));
/// assert_eq!(parser.parse("abcde"), Err(Err::Error(("abcde", ErrorKind::Verify))));
/// assert_eq!(parser.parse("123abcd;"),Err(Err::Error(("123abcd;", ErrorKind::Alpha))));
/// # }
/// ```
pub fn verify<I: Clone, O2, E: ParseError<I>, F, G>(
  first: F,
  second: G,
) -> impl Parser<I, Output = <F as Parser<I>>::Output, Error = E>
where
  F: Parser<I, Error = E>,
  G: Fn(&O2) -> bool,
  <F as Parser<I>>::Output: Borrow<O2>,
  O2: ?Sized,
{
  Verify {
    first,
    second,
    o2: PhantomData,
  }
}

/// Parser iplementation for verify
pub struct Verify<F, G, O2: ?Sized> {
  first: F,
  second: G,
  o2: PhantomData<O2>,
}

impl<I, F: Parser<I>, G, O2> Parser<I> for Verify<F, G, O2>
where
  I: Clone,
  G: Fn(&O2) -> bool,
  <F as Parser<I>>::Output: Borrow<O2>,
  O2: ?Sized,
{
  type Output = <F as Parser<I>>::Output;

  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let (i, o) = self
      .first
      .process::<OutputM<Emit, OM::Error, OM::Incomplete>>(input.clone())?;

    if (self.second)(o.borrow()) {
      Ok((i, OM::Output::bind(|| o)))
    } else {
      Err(Err::Error(OM::Error::bind(move || {
        let e: ErrorKind = ErrorKind::Verify;
        <F as Parser<I>>::Error::from_error_kind(input, e)
      })))
    }
  }
}

/// Returns the provided value if the child parser succeeds.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::combinator::value;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = value(1234, alpha1);
///
/// assert_eq!(parser.parse("abcd"), Ok(("", 1234)));
/// assert_eq!(parser.parse("123abcd;"), Err(Err::Error(("123abcd;", ErrorKind::Alpha))));
/// # }
/// ```
pub fn value<I, O1: Clone, E: ParseError<I>, F>(
  val: O1,
  parser: F,
) -> impl Parser<I, Output = O1, Error = E>
where
  F: Parser<I, Error = E>,
{
  parser.map(move |_| val.clone())
}

/// Succeeds if the child parser returns an error.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::combinator::not;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// let mut parser = not(alpha1);
///
/// assert_eq!(parser.parse("123"), Ok(("123", ())));
/// assert_eq!(parser.parse("abcd"), Err(Err::Error(("abcd", ErrorKind::Not))));
/// # }
/// ```
pub fn not<I: Clone, E: ParseError<I>, F>(parser: F) -> impl Parser<I, Output = (), Error = E>
where
  F: Parser<I, Error = E>,
{
  Not { parser }
}

/// Parser implementation for [not]
pub struct Not<F> {
  parser: F,
}

impl<I, F> Parser<I> for Not<F>
where
  I: Clone,
  F: Parser<I>,
{
  type Output = ();
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let i = input.clone();
    match self.parser.process::<OM>(input) {
      Ok(_) => Err(Err::Error(OM::Error::bind(|| {
        <F as Parser<I>>::Error::from_error_kind(i, ErrorKind::Not)
      }))),
      Err(Err::Error(_)) => Ok((i, OM::Output::bind(|| ()))),
      Err(e) => Err(e),
    }
  }
}

/// If the child parser was successful, return the consumed input as produced value.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::combinator::recognize;
/// use nom::character::complete::{char, alpha1};
/// use nom::sequence::separated_pair;
/// # fn main() {
///
/// let mut parser = recognize(separated_pair(alpha1, char(','), alpha1));
///
/// assert_eq!(parser.parse("abcd,efgh"), Ok(("", "abcd,efgh")));
/// assert_eq!(parser.parse("abcd;"),Err(Err::Error((";", ErrorKind::Char))));
/// # }
/// ```
pub fn recognize<I: Clone + Offset + Input, E: ParseError<I>, F>(
  parser: F,
) -> impl Parser<I, Output = I, Error = E>
where
  F: Parser<I, Error = E>,
{
  Recognize { parser }
}

/// Parser implementation for [recognize]
pub struct Recognize<F> {
  parser: F,
}

impl<I, F> Parser<I> for Recognize<F>
where
  I: Clone + Offset + Input,
  F: Parser<I>,
{
  type Output = I;
  type Error = <F as Parser<I>>::Error;

  #[inline(always)]
  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let i = input.clone();
    match self
      .parser
      .process::<OutputM<Check, OM::Error, OM::Incomplete>>(i)
    {
      Ok((i, _)) => {
        let index = input.offset(&i);
        Ok((i, OM::Output::bind(|| input.take(index))))
      }
      Err(e) => Err(e),
    }
  }
}

/// if the child parser was successful, return the consumed input with the output
/// as a tuple. Functions similarly to [recognize](fn.recognize.html) except it
/// returns the parser output as well.
///
/// This can be useful especially in cases where the output is not the same type
/// as the input, or the input is a user defined type.
///
/// Returned tuple is of the format `(consumed input, produced output)`.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::combinator::{consumed, value, recognize, map};
/// use nom::character::complete::{char, alpha1};
/// use nom::bytes::complete::tag;
/// use nom::sequence::separated_pair;
///
/// fn inner_parser(input: &str) -> IResult<&str, bool> {
///     value(true, tag("1234")).parse(input)
/// }
///
/// # fn main() {
///
/// let mut consumed_parser = consumed(value(true, separated_pair(alpha1, char(','), alpha1)));
///
/// assert_eq!(consumed_parser.parse("abcd,efgh1"), Ok(("1", ("abcd,efgh", true))));
/// assert_eq!(consumed_parser.parse("abcd;"),Err(Err::Error((";", ErrorKind::Char))));
///
///
/// // the first output (representing the consumed input)
/// // should be the same as that of the `recognize` parser.
/// let mut recognize_parser = recognize(inner_parser);
/// let mut consumed_parser = map(consumed(inner_parser), |(consumed, output)| consumed);
///
/// assert_eq!(recognize_parser.parse("1234"), consumed_parser.parse("1234"));
/// assert_eq!(recognize_parser.parse("abcd"), consumed_parser.parse("abcd"));
/// # }
/// ```
pub fn consumed<I, F, E>(
  parser: F,
) -> impl Parser<I, Output = (I, <F as Parser<I>>::Output), Error = E>
where
  I: Clone + Offset + Input,
  E: ParseError<I>,
  F: Parser<I, Error = E>,
{
  Consumed { parser }
}

/// Parser implementation for [consumed]
pub struct Consumed<F> {
  parser: F,
}

impl<I, F> Parser<I> for Consumed<F>
where
  I: Clone + Offset + Input,
  F: Parser<I>,
{
  type Output = (I, <F as Parser<I>>::Output);
  type Error = <F as Parser<I>>::Error;

  #[inline(always)]
  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let i = input.clone();
    match self.parser.process::<OM>(i) {
      Ok((remaining, result)) => {
        let index = input.offset(&remaining);

        Ok((
          remaining,
          OM::Output::map(result, |res| {
            let consumed = input.take(index);
            (consumed, res)
          }),
        ))
      }
      Err(e) => Err(e),
    }
  }
}

/// Transforms an [`Err::Error`] (recoverable) to [`Err::Failure`] (unrecoverable)
///
/// This commits the parse result, preventing alternative branch paths like with
/// [`nom::branch::alt`][crate::branch::alt].
///
/// # Example
///
/// Without `cut`:
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// # use nom::character::complete::{one_of, digit1};
/// # use nom::combinator::rest;
/// # use nom::branch::alt;
/// # use nom::sequence::preceded;
/// # fn main() {
///
/// fn parser(input: &str) -> IResult<&str, &str> {
///   alt((
///     preceded(one_of("+-"), digit1),
///     rest
///   )).parse(input)
/// }
///
/// assert_eq!(parser("+10 ab"), Ok((" ab", "10")));
/// assert_eq!(parser("ab"), Ok(("", "ab")));
/// assert_eq!(parser("+"), Ok(("", "+")));
/// # }
/// ```
///
/// With `cut`:
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser, error::Error};
/// # use nom::character::complete::{one_of, digit1};
/// # use nom::combinator::rest;
/// # use nom::branch::alt;
/// # use nom::sequence::preceded;
/// use nom::combinator::cut;
/// # fn main() {
///
/// fn parser(input: &str) -> IResult<&str, &str> {
///   alt((
///     preceded(one_of("+-"), cut(digit1)),
///     rest
///   )).parse(input)
/// }
///
/// assert_eq!(parser("+10 ab"), Ok((" ab", "10")));
/// assert_eq!(parser("ab"), Ok(("", "ab")));
/// assert_eq!(parser("+"), Err(Err::Failure(Error { input: "", code: ErrorKind::Digit })));
/// # }
/// ```
pub fn cut<I, E: ParseError<I>, F>(
  parser: F,
) -> impl Parser<I, Output = <F as Parser<I>>::Output, Error = E>
where
  F: Parser<I, Error = E>,
{
  Cut { parser }
}

/// Parser implementation for [cut]
pub struct Cut<F> {
  parser: F,
}

impl<I, F> Parser<I> for Cut<F>
where
  F: Parser<I>,
{
  type Output = <F as Parser<I>>::Output;

  type Error = <F as Parser<I>>::Error;

  #[inline(always)]
  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    match self
      .parser
      .process::<OutputM<OM::Output, Emit, OM::Incomplete>>(input)
    {
      Err(Err::Error(e)) => Err(Err::Failure(e)),
      Err(Err::Failure(e)) => Err(Err::Failure(e)),
      Err(Err::Incomplete(i)) => Err(Err::Incomplete(i)),
      Ok((i, o)) => Ok((i, o)),
    }
  }
}

/// automatically converts the child parser's result to another type
///
/// it will be able to convert the output value and the error value
/// as long as the `Into` implementations are available
///
/// ```rust
/// # use nom::{IResult, Parser};
/// use nom::combinator::into;
/// use nom::character::complete::alpha1;
/// # fn main() {
///
/// fn parser1(i: &str) -> IResult<&str, &str> {
///   alpha1(i)
/// }
///
/// let mut parser2 = into(parser1);
///
/// // the parser converts the &str output of the child parser into a Vec<u8>
/// let bytes: IResult<&str, Vec<u8>> = parser2.parse("abcd");
/// assert_eq!(bytes, Ok(("", vec![97, 98, 99, 100])));
/// # }
/// ```
pub fn into<I, O1, O2, E1, E2, F>(parser: F) -> impl Parser<I, Output = O2, Error = E2>
where
  O2: From<O1>,
  E2: From<E1>,
  E1: ParseError<I>,
  E2: ParseError<I>,
  F: Parser<I, Output = O1, Error = E1>,
{
  parser.into::<O2, E2>()
}

/// Creates an iterator from input data and a parser.
///
/// Call the iterator's [ParserIterator::finish] method to get the remaining input if successful,
/// or the error value if we encountered an error.
///
/// On [`Err::Error`], iteration will stop. To instead chain an error up, see [`cut`].
///
/// ```rust
/// use nom::{combinator::iterator, IResult, bytes::complete::tag, character::complete::alpha1, sequence::terminated};
/// use std::collections::HashMap;
///
/// let data = "abc|defg|hijkl|mnopqr|123";
/// let mut it = iterator(data, terminated(alpha1, tag("|")));
///
/// let parsed = it.by_ref().map(|v| (v, v.len())).collect::<HashMap<_,_>>();
/// let res: IResult<_,_> = it.finish();
///
/// assert_eq!(parsed, [("abc", 3usize), ("defg", 4), ("hijkl", 5), ("mnopqr", 6)].iter().cloned().collect());
/// assert_eq!(res, Ok(("123", ())));
/// ```
pub fn iterator<Input, Error, F>(input: Input, f: F) -> ParserIterator<Input, Error, F>
where
  F: Parser<Input>,
  Error: ParseError<Input>,
{
  ParserIterator {
    iterator: f,
    input,
    state: Some(State::Running),
  }
}

/// Main structure associated to the [iterator] function.
pub struct ParserIterator<I, E, F> {
  iterator: F,
  input: I,
  state: Option<State<E>>,
}

impl<I: Clone, E, F> ParserIterator<I, E, F> {
  /// Returns the remaining input if parsing was successful, or the error if we encountered an error.
  pub fn finish(mut self) -> IResult<I, (), E> {
    match self.state.take().unwrap() {
      State::Running | State::Done => Ok((self.input, ())),
      State::Failure(e) => Err(Err::Failure(e)),
      State::Incomplete(i) => Err(Err::Incomplete(i)),
    }
  }
}

impl<Input, Output, Error, F> core::iter::Iterator for ParserIterator<Input, Error, F>
where
  F: Parser<Input, Output = Output, Error = Error>,
  Input: Clone,
{
  type Item = Output;

  fn next(&mut self) -> Option<Self::Item> {
    if let State::Running = self.state.take().unwrap() {
      let input = self.input.clone();

      match (self.iterator).parse(input) {
        Ok((i, o)) => {
          self.input = i;
          self.state = Some(State::Running);
          Some(o)
        }
        Err(Err::Error(_)) => {
          self.state = Some(State::Done);
          None
        }
        Err(Err::Failure(e)) => {
          self.state = Some(State::Failure(e));
          None
        }
        Err(Err::Incomplete(i)) => {
          self.state = Some(State::Incomplete(i));
          None
        }
      }
    } else {
      None
    }
  }
}

enum State<E> {
  Running,
  Done,
  Failure(E),
  Incomplete(Needed),
}

/// a parser which always succeeds with given value without consuming any input.
///
/// It can be used for example as the last alternative in `alt` to
/// specify the default case.
///
/// ```rust
/// # use nom::{Err,error::ErrorKind, IResult, Parser};
/// use nom::branch::alt;
/// use nom::combinator::{success, value};
/// use nom::character::complete::char;
/// # fn main() {
///
/// let mut parser = success::<_,_,(_,ErrorKind)>(10);
/// assert_eq!(parser.parse("xyz"), Ok(("xyz", 10)));
///
/// let mut sign = alt((value(-1, char('-')), value(1, char('+')), success::<_,_,(_,ErrorKind)>(1)));
/// assert_eq!(sign.parse("+10"), Ok(("10", 1)));
/// assert_eq!(sign.parse("-10"), Ok(("10", -1)));
/// assert_eq!(sign.parse("10"), Ok(("10", 1)));
/// # }
/// ```
pub fn success<I, O: Clone, E: ParseError<I>>(val: O) -> impl Parser<I, Output = O, Error = E> {
  Success {
    val,
    e: PhantomData,
  }
}

/// Parser implementation for [success]
pub struct Success<O: Clone, E> {
  val: O,
  e: PhantomData<E>,
}

impl<I, O, E> Parser<I> for Success<O, E>
where
  O: Clone,
  E: ParseError<I>,
{
  type Output = O;
  type Error = E;

  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    Ok((input, OM::Output::bind(|| self.val.clone())))
  }
}

/// A parser which always fails.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, IResult, Parser};
/// use nom::combinator::fail;
///
/// let s = "string";
/// assert_eq!(fail::<_, &str, _>().parse(s), Err(Err::Error((s, ErrorKind::Fail))));
/// ```
pub fn fail<I, O, E: ParseError<I>>() -> impl Parser<I, Output = O, Error = E> {
  Fail {
    o: PhantomData,
    e: PhantomData,
  }
}

/// Parser implementation for [fail]
pub struct Fail<O, E> {
  o: PhantomData<O>,
  e: PhantomData<E>,
}

impl<I, O, E> Parser<I> for Fail<O, E>
where
  E: ParseError<I>,
{
  type Output = O;
  type Error = E;

  fn process<OM: OutputMode>(&mut self, input: I) -> PResult<OM, I, Self::Output, Self::Error> {
    Err(Err::Error(OM::Error::bind(|| {
      E::from_error_kind(input, ErrorKind::Fail)
    })))
  }
}
