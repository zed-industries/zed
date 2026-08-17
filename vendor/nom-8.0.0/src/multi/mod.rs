//! Combinators applying their child parser multiple times

#[cfg(test)]
mod tests;

use core::marker::PhantomData;

use crate::bytes::take;
use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::internal::{Err, Needed, Parser};
use crate::lib::std::num::NonZeroUsize;
#[cfg(feature = "alloc")]
use crate::lib::std::vec::Vec;
use crate::traits::ToUsize;
use crate::Check;
use crate::Emit;
use crate::Input;
use crate::Mode;
use crate::NomRange;
use crate::OutputM;
use crate::OutputMode;

/// Don't pre-allocate more than 64KiB when calling `Vec::with_capacity`.
///
/// Pre-allocating memory is a nice optimization but count fields can't
/// always be trusted. We should clamp initial capacities to some reasonable
/// amount. This reduces the risk of a bogus count value triggering a panic
/// due to an OOM error.
///
/// This does not affect correctness. Nom will always read the full number
/// of elements regardless of the capacity cap.
#[cfg(feature = "alloc")]
const MAX_INITIAL_CAPACITY_BYTES: usize = 65536;

/// Repeats the embedded parser, gathering the results in a `Vec`.
///
/// This stops on [`Err::Error`] and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
///
/// *Note*: if the parser passed in accepts empty inputs (like `alpha0` or `digit0`), `many0` will
/// return an error, to prevent going into an infinite loop
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::many0;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   many0(tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many0<I, F>(
  f: F,
) -> impl Parser<I, Output = Vec<<F as Parser<I>>::Output>, Error = <F as Parser<I>>::Error>
where
  I: Clone + Input,
  F: Parser<I>,
{
  Many0 { parser: f }
}

#[cfg(feature = "alloc")]
/// Parser implementation for the [many0] combinator
pub struct Many0<F> {
  parser: F,
}

#[cfg(feature = "alloc")]
impl<I, F> Parser<I> for Many0<F>
where
  I: Clone + Input,
  F: Parser<I>,
{
  type Output = crate::lib::std::vec::Vec<<F as Parser<I>>::Output>;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut i: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut acc = OM::Output::bind(|| crate::lib::std::vec::Vec::with_capacity(4));
    loop {
      let len = i.input_len();
      match self
        .parser
        .process::<OutputM<OM::Output, Check, OM::Incomplete>>(i.clone())
      {
        Err(Err::Error(_)) => return Ok((i, acc)),
        Err(Err::Failure(e)) => return Err(Err::Failure(e)),
        Err(Err::Incomplete(e)) => return Err(Err::Incomplete(e)),
        Ok((i1, o)) => {
          // infinite loop check: the parser must always consume
          if i1.input_len() == len {
            return Err(Err::Error(OM::Error::bind(|| {
              <F as Parser<I>>::Error::from_error_kind(i, ErrorKind::Many0)
            })));
          }

          i = i1;

          acc = OM::Output::combine(acc, o, |mut acc, o| {
            acc.push(o);
            acc
          })
        }
      }
    }
  }
}

/// Runs the embedded parser, gathering the results in a `Vec`.
///
/// This stops on [`Err::Error`] if there is at least one result,  and returns the results that were accumulated. To instead chain an error up,
/// see [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::multi::many1;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   many1(tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many1<I, F>(
  parser: F,
) -> impl Parser<I, Output = Vec<<F as Parser<I>>::Output>, Error = <F as Parser<I>>::Error>
where
  I: Clone + Input,
  F: Parser<I>,
{
  Many1 { parser }
}

#[cfg(feature = "alloc")]
/// Parser implementation for the [many1] combinator
pub struct Many1<F> {
  parser: F,
}

#[cfg(feature = "alloc")]
impl<I, F> Parser<I> for Many1<F>
where
  I: Clone + Input,
  F: Parser<I>,
{
  type Output = Vec<<F as Parser<I>>::Output>;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut i: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    match self
      .parser
      .process::<OutputM<OM::Output, Emit, OM::Incomplete>>(i.clone())
    {
      Err(Err::Error(err)) => Err(Err::Error(OM::Error::bind(|| {
        <F as Parser<I>>::Error::append(i, ErrorKind::Many1, err)
      }))),
      Err(Err::Failure(e)) => Err(Err::Failure(e)),
      Err(Err::Incomplete(i)) => Err(Err::Incomplete(i)),
      Ok((i1, o)) => {
        let mut acc = OM::Output::map(o, |o| {
          let mut acc = crate::lib::std::vec::Vec::with_capacity(4);
          acc.push(o);
          acc
        });

        i = i1;

        loop {
          let len = i.input_len();
          match self
            .parser
            .process::<OutputM<OM::Output, Check, OM::Incomplete>>(i.clone())
          {
            Err(Err::Error(_)) => return Ok((i, acc)),
            Err(Err::Failure(e)) => return Err(Err::Failure(e)),
            Err(Err::Incomplete(e)) => return Err(Err::Incomplete(e)),
            Ok((i1, o)) => {
              // infinite loop check: the parser must always consume
              if i1.input_len() == len {
                return Err(Err::Error(OM::Error::bind(|| {
                  <F as Parser<I>>::Error::from_error_kind(i, ErrorKind::Many0)
                })));
              }

              i = i1;

              acc = OM::Output::combine(acc, o, |mut acc, o| {
                acc.push(o);
                acc
              })
            }
          }
        }
      }
    }
  }
}

/// Applies the parser `f` until the parser `g` produces a result.
///
/// Returns a tuple of the results of `f` in a `Vec` and the result of `g`.
///
/// `f` keeps going so long as `g` produces [`Err::Error`]. To instead chain an error up, see [`cut`][crate::combinator::cut].
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::multi::many_till;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {
///   many_till(tag("abc"), tag("end")).parse(s)
/// };
///
/// assert_eq!(parser("abcabcend"), Ok(("", (vec!["abc", "abc"], "end"))));
/// assert_eq!(parser("abc123end"), Err(Err::Error(Error::new("123end", ErrorKind::Tag))));
/// assert_eq!(parser("123123end"), Err(Err::Error(Error::new("123123end", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("abcendefg"), Ok(("efg", (vec!["abc"], "end"))));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many_till<I, E, F, G>(
  f: F,
  g: G,
) -> impl Parser<I, Output = (Vec<<F as Parser<I>>::Output>, <G as Parser<I>>::Output), Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
  E: ParseError<I>,
{
  ManyTill {
    f,
    g,
    e: PhantomData,
  }
}

#[cfg(feature = "alloc")]
/// Parser implementation for the [many_till] combinator
pub struct ManyTill<F, G, E> {
  f: F,
  g: G,
  e: PhantomData<E>,
}

#[cfg(feature = "alloc")]
impl<I, F, G, E> Parser<I> for ManyTill<F, G, E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
  E: ParseError<I>,
{
  type Output = (Vec<<F as Parser<I>>::Output>, <G as Parser<I>>::Output);
  type Error = E;

  fn process<OM: OutputMode>(
    &mut self,
    mut i: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut res = OM::Output::bind(crate::lib::std::vec::Vec::new);
    loop {
      let len = i.input_len();
      match self
        .g
        .process::<OutputM<OM::Output, Check, OM::Incomplete>>(i.clone())
      {
        Ok((i1, o)) => return Ok((i1, OM::Output::combine(res, o, |res, o| (res, o)))),
        Err(Err::Failure(e)) => return Err(Err::Failure(e)),
        Err(Err::Incomplete(i)) => return Err(Err::Incomplete(i)),
        Err(Err::Error(_)) => {
          match self.f.process::<OM>(i.clone()) {
            Err(Err::Error(err)) => {
              return Err(Err::Error(OM::Error::map(err, |err| {
                E::append(i, ErrorKind::ManyTill, err)
              })))
            }
            Err(Err::Failure(e)) => return Err(Err::Failure(e)),
            Err(Err::Incomplete(e)) => return Err(Err::Incomplete(e)),
            Ok((i1, o)) => {
              // infinite loop check: the parser must always consume
              if i1.input_len() == len {
                return Err(Err::Error(OM::Error::bind(|| {
                  E::from_error_kind(i, ErrorKind::Many0)
                })));
              }

              i = i1;

              res = OM::Output::combine(res, o, |mut acc, o| {
                acc.push(o);
                acc
              })
            }
          }
        }
      }
    }
  }
}

/// Alternates between two parsers to produce a list of elements.
///
/// This stops when either parser returns [`Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `sep` Parses the separator between list elements.
/// * `f` Parses the elements of the list.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::separated_list0;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   separated_list0(tag("|"), tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser("abc|abc|abc"), Ok(("", vec!["abc", "abc", "abc"])));
/// assert_eq!(parser("abc123abc"), Ok(("123abc", vec!["abc"])));
/// assert_eq!(parser("abc|def"), Ok(("|def", vec!["abc"])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("def|abc"), Ok(("def|abc", vec![])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn separated_list0<I, E, F, G>(
  sep: G,
  f: F,
) -> impl Parser<I, Output = Vec<<F as Parser<I>>::Output>, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
  E: ParseError<I>,
{
  SeparatedList0 {
    parser: f,
    separator: sep,
  }
}

#[cfg(feature = "alloc")]
/// Parser implementation for the [separated_list0] combinator
pub struct SeparatedList0<F, G> {
  parser: F,
  separator: G,
}

#[cfg(feature = "alloc")]
impl<I, E: ParseError<I>, F, G> Parser<I> for SeparatedList0<F, G>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
{
  type Output = Vec<<F as Parser<I>>::Output>;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut i: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut res = OM::Output::bind(crate::lib::std::vec::Vec::new);

    match self
      .parser
      .process::<OutputM<OM::Output, Check, OM::Incomplete>>(i.clone())
    {
      Err(Err::Error(_)) => return Ok((i, res)),
      Err(Err::Failure(e)) => return Err(Err::Failure(e)),
      Err(Err::Incomplete(e)) => return Err(Err::Incomplete(e)),
      Ok((i1, o)) => {
        res = OM::Output::combine(res, o, |mut res, o| {
          res.push(o);
          res
        });
        i = i1;
      }
    }

    loop {
      let len = i.input_len();
      match self
        .separator
        .process::<OutputM<Check, Check, OM::Incomplete>>(i.clone())
      {
        Err(Err::Error(_)) => return Ok((i, res)),
        Err(Err::Failure(e)) => return Err(Err::Failure(e)),
        Err(Err::Incomplete(e)) => return Err(Err::Incomplete(e)),
        Ok((i1, _)) => {
          match self
            .parser
            .process::<OutputM<OM::Output, Check, OM::Incomplete>>(i1.clone())
          {
            Err(Err::Error(_)) => return Ok((i, res)),
            Err(Err::Failure(e)) => return Err(Err::Failure(e)),
            Err(Err::Incomplete(e)) => return Err(Err::Incomplete(e)),
            Ok((i2, o)) => {
              // infinite loop check: the parser must always consume
              if i2.input_len() == len {
                return Err(Err::Error(OM::Error::bind(|| {
                  <F as Parser<I>>::Error::from_error_kind(i, ErrorKind::SeparatedList)
                })));
              }

              res = OM::Output::combine(res, o, |mut res, o| {
                res.push(o);
                res
              });

              i = i2;
            }
          }
        }
      }
    }
  }
}

/// Alternates between two parsers to produce a list of elements until [`Err::Error`].
///
/// Fails if the element parser does not produce at least one element.
///
/// This stops when either parser returns [`Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `sep` Parses the separator between list elements.
/// * `f` Parses the elements of the list.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::multi::separated_list1;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   separated_list1(tag("|"), tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser("abc|abc|abc"), Ok(("", vec!["abc", "abc", "abc"])));
/// assert_eq!(parser("abc123abc"), Ok(("123abc", vec!["abc"])));
/// assert_eq!(parser("abc|def"), Ok(("|def", vec!["abc"])));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("def|abc"), Err(Err::Error(Error::new("def|abc", ErrorKind::Tag))));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn separated_list1<I, E, F, G>(
  separator: G,
  parser: F,
) -> impl Parser<I, Output = Vec<<F as Parser<I>>::Output>, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
  E: ParseError<I>,
{
  SeparatedList1 { parser, separator }
}

#[cfg(feature = "alloc")]
/// Parser implementation for the [separated_list1] combinator
pub struct SeparatedList1<F, G> {
  parser: F,
  separator: G,
}

#[cfg(feature = "alloc")]
impl<I, E: ParseError<I>, F, G> Parser<I> for SeparatedList1<F, G>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
{
  type Output = Vec<<F as Parser<I>>::Output>;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut i: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut res = OM::Output::bind(crate::lib::std::vec::Vec::new);

    match self.parser.process::<OM>(i.clone()) {
      Err(e) => return Err(e),
      Ok((i1, o)) => {
        res = OM::Output::combine(res, o, |mut res, o| {
          res.push(o);
          res
        });
        i = i1;
      }
    }

    loop {
      let len = i.input_len();
      match self
        .separator
        .process::<OutputM<Check, Check, OM::Incomplete>>(i.clone())
      {
        Err(Err::Error(_)) => return Ok((i, res)),
        Err(Err::Failure(e)) => return Err(Err::Failure(e)),
        Err(Err::Incomplete(e)) => return Err(Err::Incomplete(e)),
        Ok((i1, _)) => {
          match self
            .parser
            .process::<OutputM<OM::Output, Check, OM::Incomplete>>(i1.clone())
          {
            Err(Err::Error(_)) => return Ok((i, res)),
            Err(Err::Failure(e)) => return Err(Err::Failure(e)),
            Err(Err::Incomplete(e)) => return Err(Err::Incomplete(e)),
            Ok((i2, o)) => {
              // infinite loop check: the parser must always consume
              if i2.input_len() == len {
                return Err(Err::Error(OM::Error::bind(|| {
                  <F as Parser<I>>::Error::from_error_kind(i, ErrorKind::SeparatedList)
                })));
              }

              res = OM::Output::combine(res, o, |mut res, o| {
                res.push(o);
                res
              });
              i = i2;
            }
          }
        }
      }
    }
  }
}

/// Repeats the embedded parser `m..=n` times
///
/// This stops before `n` when the parser returns [`Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `m` The minimum number of iterations.
/// * `n` The maximum number of iterations.
/// * `f` The parser to apply.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::many_m_n;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   many_m_n(0, 2, tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many_m_n<I, E, F>(
  min: usize,
  max: usize,
  parser: F,
) -> impl Parser<I, Output = Vec<<F as Parser<I>>::Output>, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  E: ParseError<I>,
{
  ManyMN { parser, min, max }
}

#[cfg(feature = "alloc")]
/// Parser implementation for the [many_m_n] combinator
pub struct ManyMN<F> {
  parser: F,
  min: usize,
  max: usize,
}

#[cfg(feature = "alloc")]
impl<I, F> Parser<I> for ManyMN<F>
where
  I: Clone + Input,
  F: Parser<I>,
{
  type Output = Vec<<F as Parser<I>>::Output>;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    if self.min > self.max {
      return Err(Err::Failure(<F as Parser<I>>::Error::from_error_kind(
        input,
        ErrorKind::ManyMN,
      )));
    }

    let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES
      / crate::lib::std::mem::size_of::<<F as Parser<I>>::Output>().max(1);
    let mut res = OM::Output::bind(|| {
      crate::lib::std::vec::Vec::with_capacity(self.min.min(max_initial_capacity))
    });
    for count in 0..self.max {
      let len = input.input_len();
      match self.parser.process::<OM>(input.clone()) {
        Ok((tail, value)) => {
          // infinite loop check: the parser must always consume
          if tail.input_len() == len {
            return Err(Err::Error(OM::Error::bind(|| {
              <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::ManyMN)
            })));
          }

          res = OM::Output::combine(res, value, |mut res, value| {
            res.push(value);
            res
          });
          input = tail;
        }
        Err(Err::Error(e)) => {
          if count < self.min {
            return Err(Err::Error(OM::Error::map(e, |e| {
              <F as Parser<I>>::Error::append(input, ErrorKind::ManyMN, e)
            })));
          } else {
            return Ok((input, res));
          }
        }
        Err(e) => {
          return Err(e);
        }
      }
    }

    Ok((input, res))
  }
}

/// Repeats the embedded parser, counting the results
///
/// This stops on [`Err::Error`]. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
///
/// *Note*: if the parser passed in accepts empty inputs (like `alpha0` or `digit0`), `many0` will
/// return an error, to prevent going into an infinite loop
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::many0_count;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, usize> {
///   many0_count(tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", 2)));
/// assert_eq!(parser("abc123"), Ok(("123", 1)));
/// assert_eq!(parser("123123"), Ok(("123123", 0)));
/// assert_eq!(parser(""), Ok(("", 0)));
/// ```
pub fn many0_count<I, E, F>(parser: F) -> impl Parser<I, Output = usize, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  E: ParseError<I>,
{
  Many0Count { parser }
}

/// Parser implementation for the [many0_count] combinator
pub struct Many0Count<F> {
  parser: F,
}

impl<I, F> Parser<I> for Many0Count<F>
where
  I: Clone + Input,
  F: Parser<I>,
{
  type Output = usize;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut count = 0;

    loop {
      let input_ = input.clone();
      let len = input.input_len();
      match self
        .parser
        .process::<OutputM<Check, Check, OM::Incomplete>>(input_)
      {
        Ok((i, _)) => {
          // infinite loop check: the parser must always consume
          if i.input_len() == len {
            return Err(Err::Error(OM::Error::bind(|| {
              <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::Many0Count)
            })));
          }

          input = i;
          count += 1;
        }

        Err(Err::Error(_)) => return Ok((input, OM::Output::bind(|| count))),
        Err(Err::Failure(e)) => return Err(Err::Failure(e)),
        Err(Err::Incomplete(i)) => return Err(Err::Incomplete(i)),
      }
    }
  }
}

/// Runs the embedded parser, counting the results.
///
/// This stops on [`Err::Error`] if there is at least one result. To instead chain an error up,
/// see [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::multi::many1_count;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, usize> {
///   many1_count(tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", 2)));
/// assert_eq!(parser("abc123"), Ok(("123", 1)));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Many1Count))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Many1Count))));
/// ```
pub fn many1_count<I, E, F>(parser: F) -> impl Parser<I, Output = usize, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  E: ParseError<I>,
{
  Many1Count { parser }
}

/// Parser implementation for the [many1_count] combinator
pub struct Many1Count<F> {
  parser: F,
}

impl<I, F> Parser<I> for Many1Count<F>
where
  I: Clone + Input,
  F: Parser<I>,
{
  type Output = usize;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut count = 0;

    match self
      .parser
      .process::<OutputM<Check, Check, OM::Incomplete>>(input.clone())
    {
      Err(Err::Error(_)) => Err(Err::Error(OM::Error::bind(move || {
        <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::Many1Count)
      }))),
      Err(Err::Failure(e)) => Err(Err::Failure(e)),
      Err(Err::Incomplete(i)) => Err(Err::Incomplete(i)),
      Ok((mut input, _)) => {
        count += 1;

        loop {
          let input_ = input.clone();
          let len = input.input_len();
          match self
            .parser
            .process::<OutputM<Check, Check, OM::Incomplete>>(input_)
          {
            Ok((i, _)) => {
              // infinite loop check: the parser must always consume
              if i.input_len() == len {
                return Err(Err::Error(OM::Error::bind(|| {
                  <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::Many1Count)
                })));
              }

              input = i;
              count += 1;
            }

            Err(Err::Error(_)) => return Ok((input, OM::Output::bind(|| count))),
            Err(Err::Failure(e)) => return Err(Err::Failure(e)),
            Err(Err::Incomplete(i)) => return Err(Err::Incomplete(i)),
          }
        }
      }
    }
  }
}

/// Runs the embedded parser `count` times, gathering the results in a `Vec`
///
/// # Arguments
/// * `f` The parser to apply.
/// * `count` How often to apply the parser.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::multi::count;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   count(tag("abc"), 2).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Err(Err::Error(Error::new("123", ErrorKind::Tag))));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn count<I, F>(
  parser: F,
  count: usize,
) -> impl Parser<I, Output = Vec<<F as Parser<I>>::Output>, Error = <F as Parser<I>>::Error>
where
  I: Clone,
  F: Parser<I>,
{
  Count { parser, count }
}

#[cfg(feature = "alloc")]
/// Parser implementation for the [count] combinator
pub struct Count<F> {
  parser: F,
  count: usize,
}

#[cfg(feature = "alloc")]
impl<I, F> Parser<I> for Count<F>
where
  I: Clone,
  F: Parser<I>,
{
  type Output = Vec<<F as Parser<I>>::Output>;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut input = i.clone();
    let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES
      / crate::lib::std::mem::size_of::<<F as Parser<I>>::Output>().max(1);
    let mut res = OM::Output::bind(|| {
      crate::lib::std::vec::Vec::with_capacity(self.count.min(max_initial_capacity))
    });

    for _ in 0..self.count {
      let input_ = input.clone();
      match self.parser.process::<OM>(input_) {
        Ok((i, o)) => {
          res = OM::Output::combine(res, o, |mut res, o| {
            res.push(o);
            res
          });
          input = i;
        }
        Err(Err::Error(e)) => {
          return Err(Err::Error(OM::Error::map(e, |e| {
            <F as Parser<I>>::Error::append(i, ErrorKind::Count, e)
          })));
        }
        Err(e) => {
          return Err(e);
        }
      }
    }

    Ok((input, res))
  }
}

/// Runs the embedded parser repeatedly, filling the given slice with results.
///
/// This parser fails if the input runs out before the given slice is full.
///
/// # Arguments
/// * `f` The parser to apply.
/// * `buf` The slice to fill
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::multi::fill;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, [&str; 2]> {
///   let mut buf = ["", ""];
///   let (rest, ()) = fill(tag("abc"), &mut buf).parse(s)?;
///   Ok((rest, buf))
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", ["abc", "abc"])));
/// assert_eq!(parser("abc123"), Err(Err::Error(Error::new("123", ErrorKind::Tag))));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", ["abc", "abc"])));
/// ```
pub fn fill<'a, I, E, F>(
  parser: F,
  buf: &'a mut [<F as Parser<I>>::Output],
) -> impl Parser<I, Output = (), Error = E> + 'a
where
  I: Clone,
  F: Parser<I, Error = E> + 'a,
  E: ParseError<I>,
{
  Fill { parser, buf }
}

/// Parser implementation for the [fill] combinator
pub struct Fill<'a, F, O> {
  parser: F,
  buf: &'a mut [O],
}

impl<'a, I, F, O> Parser<I> for Fill<'a, F, O>
where
  I: Clone,
  F: Parser<I, Output = O>,
{
  type Output = ();
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut input = i.clone();

    for elem in self.buf.iter_mut() {
      let input_ = input.clone();
      match self.parser.process::<OM>(input_) {
        Ok((i, o)) => {
          OM::Output::map(o, |o| *elem = o);
          input = i;
        }
        Err(Err::Error(e)) => {
          return Err(Err::Error(OM::Error::map(e, |e| {
            <F as Parser<I>>::Error::append(i, ErrorKind::Count, e)
          })));
        }
        Err(e) => {
          return Err(e);
        }
      }
    }

    Ok((input, OM::Output::bind(|| ())))
  }
}

/// Repeats the embedded parser, calling `g` to gather the results.
///
/// This stops on [`Err::Error`]. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
/// * `init` A function returning the initial value.
/// * `g` The function that combines a result of `f` with
///       the current accumulator.
///
/// *Note*: if the parser passed in accepts empty inputs (like `alpha0` or `digit0`), `many0` will
/// return an error, to prevent going into an infinite loop
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::fold_many0;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold_many0(
///     tag("abc"),
///     Vec::new,
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   ).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// ```
pub fn fold_many0<I, E, F, G, H, R>(
  parser: F,
  init: H,
  g: G,
) -> impl Parser<I, Output = R, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: FnMut(R, <F as Parser<I>>::Output) -> R,
  H: FnMut() -> R,
  E: ParseError<I>,
{
  FoldMany0 {
    parser,
    g,
    init,
    r: PhantomData,
  }
}

/// Parser implementation for the [fold_many0] combinator
pub struct FoldMany0<F, G, Init, R> {
  parser: F,
  g: G,
  init: Init,
  r: PhantomData<R>,
}

impl<I, F, G, Init, R> Parser<I> for FoldMany0<F, G, Init, R>
where
  I: Clone + Input,
  F: Parser<I>,
  G: FnMut(R, <F as Parser<I>>::Output) -> R,
  Init: FnMut() -> R,
{
  type Output = R;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut res = OM::Output::bind(|| (self.init)());
    let mut input = i;

    loop {
      let i_ = input.clone();
      let len = input.input_len();
      match self.parser.process::<OM>(i_) {
        Ok((i, o)) => {
          // infinite loop check: the parser must always consume
          if i.input_len() == len {
            return Err(Err::Error(OM::Error::bind(|| {
              <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::Many0)
            })));
          }

          res = OM::Output::combine(res, o, |res, o| (self.g)(res, o));
          input = i;
        }
        Err(Err::Error(_)) => {
          return Ok((input, res));
        }
        Err(e) => {
          return Err(e);
        }
      }
    }
  }
}

/// Repeats the embedded parser, calling `g` to gather the results.
///
/// This stops on [`Err::Error`] if there is at least one result. To instead chain an error up,
/// see [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
/// * `init` A function returning the initial value.
/// * `g` The function that combines a result of `f` with
///       the current accumulator.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::multi::fold_many1;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold_many1(
///     tag("abc"),
///     Vec::new,
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   ).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Many1))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Many1))));
/// ```
pub fn fold_many1<I, E, F, G, H, R>(
  parser: F,
  init: H,
  g: G,
) -> impl Parser<I, Output = R, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: FnMut(R, <F as Parser<I>>::Output) -> R,
  H: FnMut() -> R,
  E: ParseError<I>,
{
  FoldMany1 {
    parser,
    g,
    init,
    r: PhantomData,
  }
}

/// Parser implementation for the [fold_many1] combinator
pub struct FoldMany1<F, G, Init, R> {
  parser: F,
  g: G,
  init: Init,
  r: PhantomData<R>,
}

impl<I, F, G, Init, R> Parser<I> for FoldMany1<F, G, Init, R>
where
  I: Clone + Input,
  F: Parser<I>,
  G: FnMut(R, <F as Parser<I>>::Output) -> R,
  Init: FnMut() -> R,
{
  type Output = R;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(&mut self, i: I) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let mut res = OM::Output::bind(|| (self.init)());
    let input = i.clone();

    match self.parser.process::<OM>(input) {
      Err(Err::Error(_)) => Err(Err::Error(OM::Error::bind(|| {
        <F as Parser<I>>::Error::from_error_kind(i, ErrorKind::Many1)
      }))),
      Err(e) => Err(e),
      Ok((i1, o1)) => {
        res = OM::Output::combine(res, o1, |res, o| (self.g)(res, o));

        let mut input = i1;
        loop {
          let i_ = input.clone();
          let len = input.input_len();
          match self.parser.process::<OM>(i_) {
            Ok((i, o)) => {
              // infinite loop check: the parser must always consume
              if i.input_len() == len {
                return Err(Err::Error(OM::Error::bind(|| {
                  <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::Many1)
                })));
              }

              res = OM::Output::combine(res, o, |res, o| (self.g)(res, o));
              input = i;
            }
            Err(Err::Error(_)) => {
              return Ok((input, res));
            }
            Err(e) => {
              return Err(e);
            }
          }
        }
      }
    }
  }
}

/// Repeats the embedded parser `m..=n` times, calling `g` to gather the results
///
/// This stops before `n` when the parser returns [`Err::Error`]. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `m` The minimum number of iterations.
/// * `n` The maximum number of iterations.
/// * `f` The parser to apply.
/// * `init` A function returning the initial value.
/// * `g` The function that combines a result of `f` with
///       the current accumulator.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::fold_many_m_n;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold_many_m_n(
///     0,
///     2,
///     tag("abc"),
///     Vec::new,
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   ).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
pub fn fold_many_m_n<I, E, F, G, H, R>(
  min: usize,
  max: usize,
  parser: F,
  init: H,
  g: G,
) -> impl Parser<I, Output = R, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: FnMut(R, <F as Parser<I>>::Output) -> R,
  H: FnMut() -> R,
  E: ParseError<I>,
{
  FoldManyMN {
    parser,
    g,
    init,
    min,
    max,
    r: PhantomData,
  }
}

/// Parser implementation for the [fold_many_m_n] combinator
pub struct FoldManyMN<F, G, Init, R> {
  parser: F,
  g: G,
  init: Init,
  r: PhantomData<R>,
  min: usize,
  max: usize,
}

impl<I, F, G, Init, R> Parser<I> for FoldManyMN<F, G, Init, R>
where
  I: Clone + Input,
  F: Parser<I>,
  G: FnMut(R, <F as Parser<I>>::Output) -> R,
  Init: FnMut() -> R,
{
  type Output = R;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    if self.min > self.max {
      return Err(Err::Error(OM::Error::bind(|| {
        <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::ManyMN)
      })));
    }

    let mut res = OM::Output::bind(|| (self.init)());
    for count in 0..self.max {
      let len = input.input_len();
      match self.parser.process::<OM>(input.clone()) {
        Ok((tail, value)) => {
          // infinite loop check: the parser must always consume
          if tail.input_len() == len {
            return Err(Err::Error(OM::Error::bind(|| {
              <F as Parser<I>>::Error::from_error_kind(tail, ErrorKind::ManyMN)
            })));
          }

          res = OM::Output::combine(res, value, |res, o| (self.g)(res, o));
          input = tail;
        }
        Err(Err::Error(err)) => {
          if count < self.min {
            return Err(Err::Error(OM::Error::map(err, |err| {
              <F as Parser<I>>::Error::append(input, ErrorKind::ManyMN, err)
            })));
          } else {
            break;
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok((input, res))
  }
}

/// Gets a number from the parser and returns a
/// subslice of the input of that size.
/// If the parser returns `Incomplete`,
/// `length_data` will return an error.
/// # Arguments
/// * `f` The parser to apply.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::number::complete::be_u16;
/// use nom::multi::length_data;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   length_data(be_u16).parse(s)
/// }
///
/// assert_eq!(parser(b"\x00\x03abcefg"), Ok((&b"efg"[..], &b"abc"[..])));
/// assert_eq!(parser(b"\x00\x03a"), Err(Err::Incomplete(Needed::new(2))));
/// ```
pub fn length_data<I, E, F>(f: F) -> impl Parser<I, Output = I, Error = E>
where
  I: Input,
  <F as Parser<I>>::Output: ToUsize,
  F: Parser<I, Error = E>,
  E: ParseError<I>,
{
  f.flat_map(|size| take(size))
}

/// Gets a number from the first parser,
/// takes a subslice of the input of that size,
/// then applies the second parser on that subslice.
/// If the second parser returns `Incomplete`,
/// `length_value` will return an error.
/// # Arguments
/// * `f` The parser to apply.
/// * `g` The parser to apply on the subslice.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::number::complete::be_u16;
/// use nom::multi::length_value;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   length_value(be_u16, tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser(b"\x00\x03abcefg"), Ok((&b"efg"[..], &b"abc"[..])));
/// assert_eq!(parser(b"\x00\x03123123"), Err(Err::Error(Error::new(&b"123"[..], ErrorKind::Tag))));
/// assert_eq!(parser(b"\x00\x03a"), Err(Err::Incomplete(Needed::new(2))));
/// ```
pub fn length_value<I, E, F, G>(
  f: F,
  g: G,
) -> impl Parser<I, Output = <G as Parser<I>>::Output, Error = E>
where
  I: Clone + Input,
  <F as Parser<I>>::Output: ToUsize,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
  E: ParseError<I>,
{
  /*f.flat_map(|size| {
    println!("got size: {size}");
    take(size)
  })
  .and_then(g)*/
  LengthValue {
    length: f,
    parser: g,
    e: PhantomData,
  }
}

/// Parser implementation for the [length_value] combinator
pub struct LengthValue<F, G, E> {
  length: F,
  parser: G,
  e: PhantomData<E>,
}

impl<I, F, G, E> Parser<I> for LengthValue<F, G, E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
  <F as Parser<I>>::Output: ToUsize,
  E: ParseError<I>,
{
  type Output = <G as Parser<I>>::Output;
  type Error = E;

  fn process<OM: OutputMode>(
    &mut self,
    input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    let (i, length) = self
      .length
      .process::<OutputM<Emit, OM::Error, OM::Incomplete>>(input)?;

    let length: usize = length.to_usize();

    if let Some(needed) = length
      .checked_sub(i.input_len())
      .and_then(NonZeroUsize::new)
    {
      Err(Err::Incomplete(Needed::Size(needed)))
    } else {
      let (rest, i) = i.take_split(length);
      match self.parser.process::<OM>(i.clone()) {
        Err(Err::Incomplete(_)) => Err(Err::Error(OM::Error::bind(|| {
          E::from_error_kind(i, ErrorKind::Complete)
        }))),
        Err(e) => Err(e),
        Ok((_, o)) => Ok((rest, o)),
      }
    }
  }
}

/// Gets a number from the first parser,
/// then applies the second parser that many times.
/// # Arguments
/// * `f` The parser to apply to obtain the count.
/// * `g` The parser to apply repeatedly.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::number::complete::u8;
/// use nom::multi::length_count;
/// use nom::bytes::complete::tag;
/// use nom::combinator::map;
///
/// fn parser(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
///   length_count(map(u8, |i| {
///      println!("got number: {}", i);
///      i
///   }), tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser(&b"\x02abcabcabc"[..]), Ok(((&b"abc"[..], vec![&b"abc"[..], &b"abc"[..]]))));
/// assert_eq!(parser(b"\x03123123123"), Err(Err::Error(Error::new(&b"123123123"[..], ErrorKind::Tag))));
/// ```
#[cfg(feature = "alloc")]
pub fn length_count<I, E, F, G>(
  f: F,
  g: G,
) -> impl Parser<I, Output = Vec<<G as Parser<I>>::Output>, Error = E>
where
  I: Clone,
  <F as Parser<I>>::Output: ToUsize,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
  E: ParseError<I>,
{
  LengthCount {
    length: f,
    parser: g,
    e: PhantomData,
  }
}

#[cfg(feature = "alloc")]
/// Parser implementation for the [length_count] combinator
pub struct LengthCount<F, G, E> {
  length: F,
  parser: G,
  e: PhantomData<E>,
}

#[cfg(feature = "alloc")]
impl<I, F, G, E> Parser<I> for LengthCount<F, G, E>
where
  I: Clone,
  F: Parser<I, Error = E>,
  G: Parser<I, Error = E>,
  <F as Parser<I>>::Output: ToUsize,
  E: ParseError<I>,
{
  type Output = Vec<<G as Parser<I>>::Output>;
  type Error = E;

  fn process<OM: OutputMode>(
    &mut self,
    input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    match self
      .length
      .process::<OutputM<Emit, OM::Error, OM::Incomplete>>(input)
    {
      Err(e) => Err(e),
      Ok((i, count)) => {
        let count = count.to_usize();
        let mut input = i.clone();
        let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES
          / crate::lib::std::mem::size_of::<<F as Parser<I>>::Output>().max(1);
        let mut res = OM::Output::bind(|| {
          crate::lib::std::vec::Vec::with_capacity(count.min(max_initial_capacity))
        });

        for _ in 0..count {
          let input_ = input.clone();
          match self.parser.process::<OM>(input_) {
            Ok((i, o)) => {
              res = OM::Output::combine(res, o, |mut res, o| {
                res.push(o);
                res
              });
              input = i;
            }
            Err(Err::Error(e)) => {
              return Err(Err::Error(OM::Error::map(e, |e| {
                <F as Parser<I>>::Error::append(i, ErrorKind::Count, e)
              })));
            }
            Err(e) => {
              return Err(e);
            }
          }
        }

        Ok((input, res))
      }
    }
  }
}

/// Repeats the embedded parser and collects the results in a type implementing `Extend + Default`.
/// Fails if the amount of time the embedded parser is run is not
/// within the specified range.
/// # Arguments
/// * `range` Constrains the number of iterations.
///   * A range without an upper bound `a..` is equivalent to a range of `a..=usize::MAX`.
///   * A single `usize` value is equivalent to `value..=value`.
///   * An empty range is invalid.
/// * `parse` The parser to apply.
///
/// ```rust
/// # #[macro_use] extern crate nom;
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::many;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   many(0..=2, tag("abc")).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
///
/// This is not limited to `Vec`, other collections like `HashMap`
/// can be used:
///
/// ```rust
/// # #[macro_use] extern crate nom;
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::many;
/// use nom::bytes::complete::{tag, take_while};
/// use nom::sequence::{separated_pair, terminated};
/// use nom::AsChar;
///
/// use std::collections::HashMap;
///
/// fn key_value(s: &str) -> IResult<&str, HashMap<&str, &str>> {
///   many(0.., terminated(
///     separated_pair(
///       take_while(AsChar::is_alpha),
///       tag("="),
///       take_while(AsChar::is_alpha)
///     ),
///     tag(";")
///   )).parse(s)
/// }
///
/// assert_eq!(
///   key_value("a=b;c=d;"),
///   Ok(("", HashMap::from([("a", "b"), ("c", "d")])))
/// );
/// ```
///
/// If more control is needed on the default value, [fold] can
/// be used instead:
///
/// ```rust
/// # #[macro_use] extern crate nom;
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::fold;
/// use nom::bytes::complete::tag;
///
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold(
///     0..=4,
///     tag("abc"),
///     // preallocates a vector of the max size
///     || Vec::with_capacity(4),
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   ).parse(s)
/// }
///
///
/// assert_eq!(parser("abcabcabcabc"), Ok(("", vec!["abc", "abc", "abc", "abc"])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many<I, E, Collection, F, G>(
  range: G,
  parser: F,
) -> impl Parser<I, Output = Collection, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  Collection: Extend<<F as Parser<I>>::Output> + Default,
  E: ParseError<I>,
  G: NomRange<usize>,
{
  Many {
    parser,
    range,
    c: PhantomData,
  }
}

/// Parser implementation for the [many] combinator
pub struct Many<F, R, Collection> {
  parser: F,
  range: R,
  c: PhantomData<Collection>,
}

impl<I, F, R, Collection> Parser<I> for Many<F, R, Collection>
where
  I: Clone + Input,
  F: Parser<I>,
  Collection: Extend<<F as Parser<I>>::Output> + Default,
  R: NomRange<usize>,
{
  type Output = Collection;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    if self.range.is_inverted() {
      return Err(Err::Failure(<F as Parser<I>>::Error::from_error_kind(
        input,
        ErrorKind::Many,
      )));
    }

    let mut res = OM::Output::bind(Collection::default);

    for count in self.range.bounded_iter() {
      let len = input.input_len();
      match self.parser.process::<OM>(input.clone()) {
        Ok((tail, value)) => {
          // infinite loop check: the parser must always consume
          if tail.input_len() == len {
            return Err(Err::Error(OM::Error::bind(|| {
              <F as Parser<I>>::Error::from_error_kind(input, ErrorKind::Many)
            })));
          }

          res = OM::Output::combine(res, value, |mut res, value| {
            res.extend(Some(value));
            res
          });
          input = tail;
        }
        Err(Err::Error(e)) => {
          if !self.range.contains(&count) {
            return Err(Err::Error(OM::Error::map(e, |e| {
              <F as Parser<I>>::Error::append(input, ErrorKind::Many, e)
            })));
          } else {
            return Ok((input, res));
          }
        }
        Err(e) => {
          return Err(e);
        }
      }
    }

    Ok((input, res))
  }
}

/// Applies a parser and accumulates the results using a given
/// function and initial value.
/// Fails if the amount of time the embedded parser is run is not
/// within the specified range.
///
/// # Arguments
/// * `range` Constrains the number of iterations.
///   * A range without an upper bound `a..` allows the parser to run until it fails.
///   * A single `usize` value is equivalent to `value..=value`.
///   * An empty range is invalid.
/// * `parse` The parser to apply.
/// * `init` A function returning the initial value.
/// * `fold` The function that combines a result of `f` with
///       the current accumulator.
/// ```rust
/// # #[macro_use] extern crate nom;
/// # use nom::{Err, error::ErrorKind, Needed, IResult, Parser};
/// use nom::multi::fold;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold(
///     0..=2,
///     tag("abc"),
///     Vec::new,
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   ).parse(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
pub fn fold<I, E, F, G, H, J, R>(
  range: J,
  parser: F,
  init: H,
  fold: G,
) -> impl Parser<I, Output = R, Error = E>
where
  I: Clone + Input,
  F: Parser<I, Error = E>,
  G: FnMut(R, <F as Parser<I>>::Output) -> R,
  H: FnMut() -> R,
  E: ParseError<I>,
  J: NomRange<usize>,
{
  Fold {
    parser,
    init,
    fold,
    range,
  }
}

/// Parser implementation for the [fold] combinator
pub struct Fold<F, G, H, Range> {
  parser: F,
  init: H,
  fold: G,
  range: Range,
}

impl<I, F, G, H, Range, Res> Parser<I> for Fold<F, G, H, Range>
where
  I: Clone + Input,
  F: Parser<I>,
  G: FnMut(Res, <F as Parser<I>>::Output) -> Res,
  H: FnMut() -> Res,
  Range: NomRange<usize>,
{
  type Output = Res;
  type Error = <F as Parser<I>>::Error;

  fn process<OM: OutputMode>(
    &mut self,
    mut input: I,
  ) -> crate::PResult<OM, I, Self::Output, Self::Error> {
    if self.range.is_inverted() {
      return Err(Err::Failure(<F as Parser<I>>::Error::from_error_kind(
        input,
        ErrorKind::Fold,
      )));
    }

    let mut acc = OM::Output::bind(|| (self.init)());

    for count in self.range.saturating_iter() {
      let len = input.input_len();
      match self.parser.process::<OM>(input.clone()) {
        Ok((tail, value)) => {
          // infinite loop check: the parser must always consume
          if tail.input_len() == len {
            return Err(Err::Error(OM::Error::bind(|| {
              <F as Parser<I>>::Error::from_error_kind(tail, ErrorKind::Fold)
            })));
          }

          acc = OM::Output::combine(acc, value, |acc, value| (self.fold)(acc, value));
          input = tail;
        }
        Err(Err::Error(err)) => {
          if !self.range.contains(&count) {
            return Err(Err::Error(OM::Error::map(err, |err| {
              <F as Parser<I>>::Error::append(input, ErrorKind::Fold, err)
            })));
          } else {
            break;
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok((input, acc))
  }
}
