//! Parsers recognizing bytes streams

pub mod complete;
pub mod streaming;
#[cfg(test)]
mod tests;

use core::marker::PhantomData;

use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::internal::{Err, Needed, Parser};
use crate::lib::std::result::Result::*;
use crate::traits::{Compare, CompareResult};
use crate::AsChar;
use crate::Check;
use crate::ExtendInto;
use crate::FindSubstring;
use crate::FindToken;
use crate::Input;
use crate::IsStreaming;
use crate::Mode;
use crate::OutputM;
use crate::OutputMode;
use crate::ToUsize;

/// Recognizes a pattern.
///
/// The input data will be compared to the tag combinator's argument and will return the part of
/// the input that matches the argument.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::tag;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///   tag("Hello")(s)
/// }
///
/// assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
/// assert_eq!(parser("Something"), Err(Err::Error(Error::new("Something", ErrorKind::Tag))));
/// assert_eq!(parser("S"), Err(Err::Error(Error::new("S", ErrorKind::Tag))));
/// assert_eq!(parser("H"), Err(Err::Incomplete(Needed::new(4))));
/// ```
pub fn tag<T, I, Error: ParseError<I>>(tag: T) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input + Compare<T>,
  T: Input + Clone,
{
  Tag {
    tag,
    e: PhantomData,
  }
}

/// Tag implementation
pub struct Tag<T, E> {
  tag: T,
  e: PhantomData<E>,
}

impl<I, Error: ParseError<I>, T> Parser<I> for Tag<T, Error>
where
  I: Input + Compare<T>,
  T: Input + Clone,
{
  type Output = I;

  type Error = Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let tag_len = self.tag.input_len();
    let t = self.tag.clone();

    match i.compare(t) {
      CompareResult::Ok => Ok((i.take_from(tag_len), OM::Output::bind(|| i.take(tag_len)))),
      CompareResult::Incomplete => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::new(tag_len - i.input_len())))
        } else {
          Err(Err::Error(OM::Error::bind(|| {
            let e: ErrorKind = ErrorKind::Tag;
            Error::from_error_kind(i, e)
          })))
        }
      }
      CompareResult::Error => Err(Err::Error(OM::Error::bind(|| {
        let e: ErrorKind = ErrorKind::Tag;
        Error::from_error_kind(i, e)
      }))),
    }
  }
}

/// Recognizes a case insensitive pattern.
///
/// The input data will be compared to the tag combinator's argument and will return the part of
/// the input that matches the argument with no regard to case.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::tag_no_case;
///
/// fn parser(s: &str) -> IResult<&str, &str> {
///   tag_no_case("hello")(s)
/// }
///
/// assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
/// assert_eq!(parser("hello, World!"), Ok((", World!", "hello")));
/// assert_eq!(parser("HeLlO, World!"), Ok((", World!", "HeLlO")));
/// assert_eq!(parser("Something"), Err(Err::Error(Error::new("Something", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Incomplete(Needed::new(5))));
/// ```
pub fn tag_no_case<T, I, Error: ParseError<I>>(tag: T) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input + Compare<T>,
  T: Input + Clone,
{
  TagNoCase {
    tag,
    e: PhantomData,
  }
}

/// Case insensitive Tag implementation
pub struct TagNoCase<T, E> {
  tag: T,
  e: PhantomData<E>,
}

impl<I, Error: ParseError<I>, T> Parser<I> for TagNoCase<T, Error>
where
  I: Input + Compare<T>,
  T: Input + Clone,
{
  type Output = I;

  type Error = Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let tag_len = self.tag.input_len();
    let t = self.tag.clone();

    match i.compare_no_case(t) {
      CompareResult::Ok => Ok((i.take_from(tag_len), OM::Output::bind(|| i.take(tag_len)))),
      CompareResult::Incomplete => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::new(tag_len - i.input_len())))
        } else {
          Err(Err::Error(OM::Error::bind(|| {
            let e: ErrorKind = ErrorKind::Tag;
            Error::from_error_kind(i, e)
          })))
        }
      }
      CompareResult::Error => Err(Err::Error(OM::Error::bind(|| {
        let e: ErrorKind = ErrorKind::Tag;
        Error::from_error_kind(i, e)
      }))),
    }
  }
}

/// Parser wrapper for `split_at_position`
pub struct SplitPosition<F, E> {
  predicate: F,
  error: PhantomData<E>,
}

impl<I, Error: ParseError<I>, F> Parser<I> for SplitPosition<F, Error>
where
  I: Input,
  F: Fn(<I as Input>::Item) -> bool,
{
  type Output = I;

  type Error = Error;

  #[inline(always)]
  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    i.split_at_position_mode::<OM, _, _>(|c| (self.predicate)(c))
  }
}

/// Parser wrapper for `split_at_position1`
pub struct SplitPosition1<F, E> {
  e: ErrorKind,
  predicate: F,
  error: PhantomData<E>,
}

impl<I, Error: ParseError<I>, F> Parser<I> for SplitPosition1<F, Error>
where
  I: Input,
  F: Fn(<I as Input>::Item) -> bool,
{
  type Output = I;

  type Error = Error;

  #[inline(always)]
  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    i.split_at_position_mode1::<OM, _, _>(|c| (self.predicate)(c), self.e)
  }
}

/// Parse till certain characters are met.
///
/// The parser will return the longest slice till one of the characters of the combinator's argument are met.
///
/// It doesn't consume the matched character.
///
/// It will return a `Err::Error(("", ErrorKind::IsNot))` if the pattern wasn't met.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::complete::is_not;
///
/// fn not_space(s: &str) -> IResult<&str, &str> {
///   is_not(" \t\r\n")(s)
/// }
///
/// assert_eq!(not_space("Hello, World!"), Ok((" World!", "Hello,")));
/// assert_eq!(not_space("Sometimes\t"), Ok(("\t", "Sometimes")));
/// assert_eq!(not_space("Nospace"), Ok(("", "Nospace")));
/// assert_eq!(not_space(""), Err(Err::Error(Error::new("", ErrorKind::IsNot))));
/// ```
pub fn is_not<T, I, Error: ParseError<I>>(arr: T) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input,
  T: FindToken<<I as Input>::Item>,
{
  SplitPosition1 {
    e: ErrorKind::IsNot,
    predicate: move |c| arr.find_token(c),
    error: PhantomData,
  }
}

/// Returns the longest slice of the matches the pattern.
///
/// The parser will return the longest slice consisting of the characters in provided in the
/// combinator's argument.
///
/// It will return a `Err(Err::Error((_, ErrorKind::IsA)))` if the pattern wasn't met.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::complete::is_a;
///
/// fn hex(s: &str) -> IResult<&str, &str> {
///   is_a("1234567890ABCDEF")(s)
/// }
///
/// assert_eq!(hex("123 and voila"), Ok((" and voila", "123")));
/// assert_eq!(hex("DEADBEEF and others"), Ok((" and others", "DEADBEEF")));
/// assert_eq!(hex("BADBABEsomething"), Ok(("something", "BADBABE")));
/// assert_eq!(hex("D15EA5E"), Ok(("", "D15EA5E")));
/// assert_eq!(hex(""), Err(Err::Error(Error::new("", ErrorKind::IsA))));
/// ```
pub fn is_a<T, I, Error: ParseError<I>>(arr: T) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input,
  T: FindToken<<I as Input>::Item>,
{
  SplitPosition1 {
    e: ErrorKind::IsA,
    predicate: move |c| !arr.find_token(c),
    error: PhantomData,
  }
}

/// Returns the longest input slice (if any) that matches the predicate.
///
/// The parser will return the longest slice that matches the given predicate *(a function that
/// takes the input and returns a bool)*.
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::complete::take_while;
/// use nom::character::is_alphabetic;
///
/// fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   take_while(is_alphabetic)(s)
/// }
///
/// assert_eq!(alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
/// assert_eq!(alpha(b"12345"), Ok((&b"12345"[..], &b""[..])));
/// assert_eq!(alpha(b"latin"), Ok((&b""[..], &b"latin"[..])));
/// assert_eq!(alpha(b""), Ok((&b""[..], &b""[..])));
/// ```
pub fn take_while<F, I, Error: ParseError<I>>(cond: F) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input,
  F: Fn(<I as Input>::Item) -> bool,
{
  SplitPosition {
    predicate: move |c| !cond(c),
    error: PhantomData,
  }
}

/// Returns the longest (at least 1) input slice that matches the predicate.
///
/// The parser will return the longest slice that matches the given predicate *(a function that
/// takes the input and returns a bool)*.
///
/// It will return an `Err(Err::Error((_, ErrorKind::TakeWhile1)))` if the pattern wasn't met.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))` or if the pattern reaches the end of the input.
///
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::take_while1;
/// use nom::character::is_alphabetic;
///
/// fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   take_while1(is_alphabetic)(s)
/// }
///
/// assert_eq!(alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
/// assert_eq!(alpha(b"latin"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(alpha(b"12345"), Err(Err::Error(Error::new(&b"12345"[..], ErrorKind::TakeWhile1))));
/// ```
pub fn take_while1<F, I, Error: ParseError<I>>(cond: F) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input,
  F: Fn(<I as Input>::Item) -> bool,
{
  SplitPosition1 {
    e: ErrorKind::TakeWhile1,
    predicate: move |c| !cond(c),
    error: PhantomData,
  }
}

/// Returns the longest (m <= len <= n) input slice  that matches the predicate.
///
/// The parser will return the longest slice that matches the given predicate *(a function that
/// takes the input and returns a bool)*.
///
/// It will return an `Err::Error((_, ErrorKind::TakeWhileMN))` if the pattern wasn't met.
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))`  if the pattern reaches the end of the input or is too short.
///
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::take_while_m_n;
/// use nom::character::is_alphabetic;
///
/// fn short_alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   take_while_m_n(3, 6, is_alphabetic)(s)
/// }
///
/// assert_eq!(short_alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
/// assert_eq!(short_alpha(b"lengthy"), Ok((&b"y"[..], &b"length"[..])));
/// assert_eq!(short_alpha(b"latin"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(short_alpha(b"ed"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(short_alpha(b"12345"), Err(Err::Error(Error::new(&b"12345"[..], ErrorKind::TakeWhileMN))));
/// ```
pub fn take_while_m_n<F, I, Error: ParseError<I>>(
  m: usize,
  n: usize,
  predicate: F,
) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input,
  F: Fn(<I as Input>::Item) -> bool,
{
  TakeWhileMN {
    m,
    n,
    predicate,
    e: PhantomData,
  }
}

/// Parser implementation for [take_while_m_n]
pub struct TakeWhileMN<F, E> {
  m: usize,
  n: usize,
  predicate: F,
  e: PhantomData<E>,
}

impl<I, Error: ParseError<I>, F> Parser<I> for TakeWhileMN<F, Error>
where
  I: Input,
  F: Fn(<I as Input>::Item) -> bool,
{
  type Output = I;
  type Error = Error;

  fn process<OM: OutputMode>(
    &mut self,
    input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut count = 0;
    for (i, (index, item)) in input.iter_indices().enumerate() {
      if i == self.n {
        return Ok((
          input.take_from(index),
          OM::Output::bind(|| input.take(index)),
        ));
      }

      if !(self.predicate)(item) {
        if i >= self.m {
          return Ok((
            input.take_from(index),
            OM::Output::bind(|| input.take(index)),
          ));
        } else {
          return Err(Err::Error(OM::Error::bind(|| {
            Error::from_error_kind(input, ErrorKind::TakeWhileMN)
          })));
        }
      }
      count += 1;
    }

    let input_len = input.input_len();
    if OM::Incomplete::is_streaming() {
      let needed = if self.m > input_len {
        self.m - input_len
      } else {
        1
      };
      Err(Err::Incomplete(Needed::new(needed)))
    } else if count >= self.m {
      Ok((
        input.take_from(input_len),
        OM::Output::bind(|| input.take(input_len)),
      ))
    } else {
      Err(Err::Error(OM::Error::bind(|| {
        Error::from_error_kind(input, ErrorKind::TakeWhileMN)
      })))
    }
  }
}

/// Returns the longest input slice (if any) till a predicate is met.
///
/// The parser will return the longest slice till the given predicate *(a function that
/// takes the input and returns a bool)*.
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::complete::take_till;
///
/// fn till_colon(s: &str) -> IResult<&str, &str> {
///   take_till(|c| c == ':')(s)
/// }
///
/// assert_eq!(till_colon("latin:123"), Ok((":123", "latin")));
/// assert_eq!(till_colon(":empty matched"), Ok((":empty matched", ""))); //allowed
/// assert_eq!(till_colon("12345"), Ok(("", "12345")));
/// assert_eq!(till_colon(""), Ok(("", "")));
/// ```
#[allow(clippy::redundant_closure)]
pub fn take_till<F, I, Error: ParseError<I>>(cond: F) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input,
  F: Fn(<I as Input>::Item) -> bool,
{
  SplitPosition {
    predicate: cond,
    error: PhantomData,
  }
}

/// Returns the longest (at least 1) input slice till a predicate is met.
///
/// The parser will return the longest slice till the given predicate *(a function that
/// takes the input and returns a bool)*.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(1))` if the match reaches the
/// end of input or if there was not match.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::take_till1;
///
/// fn till_colon(s: &str) -> IResult<&str, &str> {
///   take_till1(|c| c == ':')(s)
/// }
///
/// assert_eq!(till_colon("latin:123"), Ok((":123", "latin")));
/// assert_eq!(till_colon(":empty matched"), Err(Err::Error(Error::new(":empty matched", ErrorKind::TakeTill1))));
/// assert_eq!(till_colon("12345"), Err(Err::Incomplete(Needed::new(1))));
/// assert_eq!(till_colon(""), Err(Err::Incomplete(Needed::new(1))));
/// ```
#[allow(clippy::redundant_closure)]
pub fn take_till1<F, I, Error: ParseError<I>>(cond: F) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input,
  F: Fn(<I as Input>::Item) -> bool,
{
  SplitPosition1 {
    e: ErrorKind::TakeTill1,
    predicate: cond,
    error: PhantomData,
  }
}

/// Returns an input slice containing the first N input elements (Input[..N]).
///
/// # Streaming Specific
/// *Streaming version* if the input has less than N elements, `take` will
/// return a `Err::Incomplete(Needed::new(M))` where M is the number of
/// additional bytes the parser would need to succeed.
/// It is well defined for `&[u8]` as the number of elements is the byte size,
/// but for types like `&str`, we cannot know how many bytes correspond for
/// the next few chars, so the result will be `Err::Incomplete(Needed::Unknown)`
///
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::streaming::take;
///
/// fn take6(s: &str) -> IResult<&str, &str> {
///   take(6usize)(s)
/// }
///
/// assert_eq!(take6("1234567"), Ok(("7", "123456")));
/// assert_eq!(take6("things"), Ok(("", "things")));
/// assert_eq!(take6("short"), Err(Err::Incomplete(Needed::Unknown)));
/// ```
pub fn take<C, I, Error: ParseError<I>>(count: C) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input,
  C: ToUsize,
{
  Take {
    length: count.to_usize(),
    e: PhantomData,
  }
}

/// Parser implementation for [take]
pub struct Take<E> {
  length: usize,
  e: PhantomData<E>,
}

impl<I, Error: ParseError<I>> Parser<I> for Take<Error>
where
  I: Input,
{
  type Output = I;
  type Error = Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    match i.slice_index(self.length) {
      Err(needed) => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(needed))
        } else {
          Err(Err::Error(OM::Error::bind(|| {
            let e: ErrorKind = ErrorKind::Eof;
            Error::from_error_kind(i, e)
          })))
        }
      }
      Ok(index) => Ok((i.take_from(index), OM::Output::bind(|| i.take(index)))),
    }
  }
}

/// Returns the input slice up to the first occurrence of the pattern.
///
/// It doesn't consume the pattern.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(N))` if the input doesn't
/// contain the pattern or if the input is smaller than the pattern.
/// # Example
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::bytes::streaming::take_until;
///
/// fn until_eof(s: &str) -> IResult<&str, &str> {
///   take_until("eof")(s)
/// }
///
/// assert_eq!(until_eof("hello, worldeof"), Ok(("eof", "hello, world")));
/// assert_eq!(until_eof("hello, world"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(until_eof("hello, worldeo"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(until_eof("1eof2eof"), Ok(("eof2eof", "1")));
/// ```
pub fn take_until<T, I, Error: ParseError<I>>(tag: T) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input + FindSubstring<T>,
  T: Clone,
{
  TakeUntil {
    tag,
    e: PhantomData,
  }
}

/// Parser implementation for [take_until]
pub struct TakeUntil<T, E> {
  tag: T,
  e: PhantomData<E>,
}

impl<I, T, Error: ParseError<I>> Parser<I> for TakeUntil<T, Error>
where
  I: Input + FindSubstring<T>,
  T: Clone,
{
  type Output = I;
  type Error = Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    match i.find_substring(self.tag.clone()) {
      None => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::Unknown))
        } else {
          Err(Err::Error(OM::Error::bind(|| {
            let e: ErrorKind = ErrorKind::TakeUntil;
            Error::from_error_kind(i, e)
          })))
        }
      }
      Some(index) => Ok((i.take_from(index), OM::Output::bind(|| i.take(index)))),
    }
  }
}

/// Returns the non empty input slice up to the first occurrence of the pattern.
///
/// It doesn't consume the pattern.
///
/// # Streaming Specific
/// *Streaming version* will return a `Err::Incomplete(Needed::new(N))` if the input doesn't
/// contain the pattern or if the input is smaller than the pattern.
/// # Example
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::streaming::take_until1;
///
/// fn until_eof(s: &str) -> IResult<&str, &str> {
///   take_until1("eof")(s)
/// }
///
/// assert_eq!(until_eof("hello, worldeof"), Ok(("eof", "hello, world")));
/// assert_eq!(until_eof("hello, world"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(until_eof("hello, worldeo"), Err(Err::Incomplete(Needed::Unknown)));
/// assert_eq!(until_eof("1eof2eof"), Ok(("eof2eof", "1")));
/// assert_eq!(until_eof("eof"),  Err(Err::Error(Error::new("eof", ErrorKind::TakeUntil))));
/// ```
pub fn take_until1<T, I, Error: ParseError<I>>(tag: T) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input + FindSubstring<T>,
  T: Clone,
{
  TakeUntil1 {
    tag,
    e: PhantomData,
  }
}

/// Parser implementation for take_until1
pub struct TakeUntil1<T, E> {
  tag: T,
  e: PhantomData<E>,
}

impl<I, T, Error: ParseError<I>> Parser<I> for TakeUntil1<T, Error>
where
  I: Input + FindSubstring<T>,
  T: Clone,
{
  type Output = I;
  type Error = Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    match i.find_substring(self.tag.clone()) {
      None => {
        if OM::Incomplete::is_streaming() {
          Err(Err::Incomplete(Needed::Unknown))
        } else {
          Err(Err::Error(OM::Error::bind(|| {
            let e: ErrorKind = ErrorKind::TakeUntil;
            Error::from_error_kind(i, e)
          })))
        }
      }
      Some(0) => Err(Err::Error(OM::Error::bind(|| {
        Error::from_error_kind(i, ErrorKind::TakeUntil)
      }))),

      Some(index) => Ok((i.take_from(index), OM::Output::bind(|| i.take(index)))),
    }
  }
}

/// Matches a byte string with escaped characters.
///
/// * The first argument matches the normal characters (it must not accept the control character)
/// * The second argument is the control character (like `\` in most languages)
/// * The third argument matches the escaped characters
/// # Example
/// ```
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// # use nom::character::complete::digit1;
/// use nom::bytes::streaming::escaped;
/// use nom::character::streaming::one_of;
///
/// fn esc(s: &str) -> IResult<&str, &str> {
///   escaped(digit1, '\\', one_of("\"n\\"))(s)
/// }
///
/// assert_eq!(esc("123;"), Ok((";", "123")));
/// assert_eq!(esc("12\\\"34;"), Ok((";", "12\\\"34")));
/// ```
///
pub fn escaped<I, Error, F, G>(
  normal: F,
  control_char: char,
  escapable: G,
) -> impl Parser<I, Output = I, Error = Error>
where
  I: Input + Clone + crate::traits::Offset,
  <I as Input>::Item: crate::traits::AsChar,
  F: Parser<I, Error = Error>,
  G: Parser<I, Error = Error>,
  Error: ParseError<I>,
{
  Escaped {
    normal,
    escapable,
    control_char,
    e: PhantomData,
  }
}

/// Parser implementation for [escaped]
pub struct Escaped<F, G, E> {
  normal: F,
  escapable: G,
  control_char: char,
  e: PhantomData<E>,
}

impl<I, Error: ParseError<I>, F, G> Parser<I> for Escaped<F, G, Error>
where
  I: Input + Clone + crate::traits::Offset,
  <I as Input>::Item: crate::traits::AsChar,
  F: Parser<I, Error = Error>,
  G: Parser<I, Error = Error>,
  Error: ParseError<I>,
{
  type Output = I;
  type Error = Error;

  fn process<OM: OutputMode>(
    &mut self,
    input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut i = input.clone();

    while i.input_len() > 0 {
      let current_len = i.input_len();

      match self
        .normal
        .process::<OutputM<Check, Check, OM::Incomplete>>(i.clone())
      {
        Ok((i2, _)) => {
          if i2.input_len() == 0 {
            if OM::Incomplete::is_streaming() {
              return Err(Err::Incomplete(Needed::Unknown));
            } else {
              let index = input.input_len();
              return Ok((
                input.take_from(index),
                OM::Output::bind(|| input.take(index)),
              ));
            }
          } else if i2.input_len() == current_len {
            let index = input.offset(&i2);
            return Ok((
              input.take_from(index),
              OM::Output::bind(|| input.take(index)),
            ));
          } else {
            i = i2;
          }
        }
        Err(Err::Error(_)) => {
          // unwrap() should be safe here since index < $i.input_len()
          if i.iter_elements().next().unwrap().as_char() == self.control_char {
            let next = self.control_char.len_utf8();
            if next >= i.input_len() {
              if OM::Incomplete::is_streaming() {
                return Err(Err::Incomplete(Needed::new(1)));
              } else {
                return Err(Err::Error(OM::Error::bind(|| {
                  Error::from_error_kind(input, ErrorKind::Escaped)
                })));
              }
            } else {
              match self
                .escapable
                .process::<OutputM<Check, OM::Error, OM::Incomplete>>(i.take_from(next))
              {
                Ok((i2, _)) => {
                  if i2.input_len() == 0 {
                    if OM::Incomplete::is_streaming() {
                      return Err(Err::Incomplete(Needed::Unknown));
                    } else {
                      let index = input.input_len();
                      return Ok((
                        input.take_from(index),
                        OM::Output::bind(|| input.take(index)),
                      ));
                    }
                  } else {
                    i = i2;
                  }
                }
                Err(e) => return Err(e),
              }
            }
          } else {
            let index = input.offset(&i);
            if index == 0 {
              return Err(Err::Error(OM::Error::bind(|| {
                Error::from_error_kind(input, ErrorKind::Escaped)
              })));
            } else {
              return Ok((
                input.take_from(index),
                OM::Output::bind(|| input.take(index)),
              ));
            }
          }
        }
        Err(Err::Failure(e)) => {
          return Err(Err::Failure(e));
        }
        Err(Err::Incomplete(i)) => {
          return Err(Err::Incomplete(i));
        }
      }
    }

    if OM::Incomplete::is_streaming() {
      Err(Err::Incomplete(Needed::Unknown))
    } else {
      let index = input.input_len();
      Ok((
        input.take_from(index),
        OM::Output::bind(|| input.take(index)),
      ))
    }
  }
}

/// Matches a byte string with escaped characters.
///
/// * The first argument matches the normal characters (it must not match the control character)
/// * The second argument is the control character (like `\` in most languages)
/// * The third argument matches the escaped characters and transforms them
///
/// As an example, the chain `abc\tdef` could be `abc    def` (it also consumes the control character)
///
/// ```
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// # use std::str::from_utf8;
/// use nom::bytes::streaming::{escaped_transform, tag};
/// use nom::character::streaming::alpha1;
/// use nom::branch::alt;
/// use nom::combinator::value;
///
/// fn parser(input: &str) -> IResult<&str, String> {
///   escaped_transform(
///     alpha1,
///     '\\',
///     alt((
///       value("\\", tag("\\")),
///       value("\"", tag("\"")),
///       value("\n", tag("n")),
///     ))
///   )(input)
/// }
///
/// assert_eq!(parser("ab\\\"cd\""), Ok(("\"", String::from("ab\"cd"))));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn escaped_transform<I, Error, F, G, ExtendItem, Output>(
  normal: F,
  control_char: char,
  transform: G,
) -> impl Parser<I, Output = Output, Error = Error>
where
  I: Clone + crate::traits::Offset + Input,
  I: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
  <F as Parser<I>>::Output: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
  <G as Parser<I>>::Output: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
  <I as Input>::Item: crate::traits::AsChar,
  F: Parser<I, Error = Error>,
  G: Parser<I, Error = Error>,
  Error: ParseError<I>,
{
  EscapedTransform {
    normal,
    control_char,
    transform,
    e: PhantomData,
    extend: PhantomData,
    o: PhantomData,
  }
}

/// Parser implementation for [escaped_transform]
pub struct EscapedTransform<F, G, E, ExtendItem, Output> {
  normal: F,
  transform: G,
  control_char: char,
  e: PhantomData<E>,
  extend: PhantomData<ExtendItem>,
  o: PhantomData<Output>,
}

impl<I, Error: ParseError<I>, F, G, ExtendItem, Output> Parser<I>
  for EscapedTransform<F, G, Error, ExtendItem, Output>
where
  I: Clone + crate::traits::Offset + Input,
  I: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
  <F as Parser<I>>::Output: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
  <G as Parser<I>>::Output: crate::traits::ExtendInto<Item = ExtendItem, Extender = Output>,
  <I as Input>::Item: crate::traits::AsChar,
  F: Parser<I, Error = Error>,
  G: Parser<I, Error = Error>,
  Error: ParseError<I>,
{
  type Output = Output;
  type Error = Error;

  fn process<OM: OutputMode>(
    &mut self,
    input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut index = 0;
    let mut res = OM::Output::bind(|| input.new_builder());

    while index < input.input_len() {
      let current_len = input.input_len();
      let remainder = input.take_from(index);
      match self.normal.process::<OM>(remainder.clone()) {
        Ok((i2, o)) => {
          res = OM::Output::combine(o, res, |o, mut res| {
            o.extend_into(&mut res);
            res
          });
          if i2.input_len() == 0 {
            if OM::Incomplete::is_streaming() {
              return Err(Err::Incomplete(Needed::Unknown));
            } else {
              let index = input.input_len();
              return Ok((input.take_from(index), res));
            }
          } else if i2.input_len() == current_len {
            return Ok((remainder, res));
          } else {
            index = input.offset(&i2);
          }
        }
        Err(Err::Error(_)) => {
          // unwrap() should be safe here since index < $i.input_len()
          if remainder.iter_elements().next().unwrap().as_char() == self.control_char {
            let next = index + self.control_char.len_utf8();
            let input_len = input.input_len();

            if next >= input_len {
              if OM::Incomplete::is_streaming() {
                return Err(Err::Incomplete(Needed::Unknown));
              } else {
                return Err(Err::Error(OM::Error::bind(|| {
                  Error::from_error_kind(remainder, ErrorKind::EscapedTransform)
                })));
              }
            } else {
              match self.transform.process::<OM>(input.take_from(next)) {
                Ok((i2, o)) => {
                  res = OM::Output::combine(o, res, |o, mut res| {
                    o.extend_into(&mut res);
                    res
                  });
                  if i2.input_len() == 0 {
                    if OM::Incomplete::is_streaming() {
                      return Err(Err::Incomplete(Needed::Unknown));
                    } else {
                      return Ok((input.take_from(input.input_len()), res));
                    }
                  } else {
                    index = input.offset(&i2);
                  }
                }
                Err(Err::Error(e)) => return Err(Err::Error(e)),
                Err(Err::Failure(e)) => {
                  return Err(Err::Failure(e));
                }
                Err(Err::Incomplete(i)) => {
                  return Err(Err::Incomplete(i));
                }
              }
            }
          } else {
            if index == 0 {
              return Err(Err::Error(OM::Error::bind(|| {
                Error::from_error_kind(remainder, ErrorKind::EscapedTransform)
              })));
            }
            return Ok((remainder, res));
          }
        }
        Err(Err::Failure(e)) => {
          return Err(Err::Failure(e));
        }
        Err(Err::Incomplete(i)) => {
          return Err(Err::Incomplete(i));
        }
      }
    }

    if OM::Incomplete::is_streaming() {
      Err(Err::Incomplete(Needed::Unknown))
    } else {
      Ok((input.take_from(index), res))
    }
  }
}
