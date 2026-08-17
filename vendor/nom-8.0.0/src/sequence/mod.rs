//! Combinators applying parsers in sequence

#[cfg(test)]
mod tests;

use crate::error::ParseError;
use crate::internal::{IResult, Parser};
use crate::{Check, OutputM, OutputMode, PResult};

/// Gets an object from the first parser,
/// then gets another object from the second parser.
///
/// # Arguments
/// * `first` The first parser to apply.
/// * `second` The second parser to apply.
///
/// # Example
/// ```rust
/// use nom::sequence::pair;
/// use nom::bytes::complete::tag;
/// use nom::{error::ErrorKind, Err, Parser};
///
/// let mut parser = pair(tag("abc"), tag("efg"));
///
/// assert_eq!(parser.parse("abcefg"), Ok(("", ("abc", "efg"))));
/// assert_eq!(parser.parse("abcefghij"), Ok(("hij", ("abc", "efg"))));
/// assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser.parse("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn pair<I, O1, O2, E: ParseError<I>, F, G>(
  first: F,
  second: G,
) -> impl Parser<I, Output = (O1, O2), Error = E>
where
  F: Parser<I, Output = O1, Error = E>,
  G: Parser<I, Output = O2, Error = E>,
{
  first.and(second)
}

/// Matches an object from the first parser and discards it,
/// then gets an object from the second parser.
///
/// # Arguments
/// * `first` The opening parser.
/// * `second` The second parser to get object.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, Parser};
/// # use nom::Needed::Size;
/// use nom::sequence::preceded;
/// use nom::bytes::complete::tag;
///
/// let mut parser = preceded(tag("abc"), tag("efg"));
///
/// assert_eq!(parser.parse("abcefg"), Ok(("", "efg")));
/// assert_eq!(parser.parse("abcefghij"), Ok(("hij", "efg")));
/// assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser.parse("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn preceded<I, O, E: ParseError<I>, F, G>(
  first: F,
  second: G,
) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Error = E>,
  G: Parser<I, Output = O, Error = E>,
{
  Preceded {
    f: first,
    g: second,
  }
}

/// a
pub struct Preceded<F, G> {
  f: F,
  g: G,
}

impl<I, E: ParseError<I>, F: Parser<I, Error = E>, G: Parser<I, Error = E>> Parser<I>
  for Preceded<F, G>
{
  type Output = <G as Parser<I>>::Output;
  type Error = E;

  #[inline(always)]
  fn process<OM: OutputMode>(&mut self, i: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let (i, _) = self
      .f
      .process::<OutputM<Check, OM::Error, OM::Incomplete>>(i)?;
    let (i, o2) = self.g.process::<OM>(i)?;

    Ok((i, o2))
  }
}

/// Gets an object from the first parser,
/// then matches an object from the second parser and discards it.
///
/// # Arguments
/// * `first` The first parser to apply.
/// * `second` The second parser to match an object.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, Parser};
/// # use nom::Needed::Size;
/// use nom::sequence::terminated;
/// use nom::bytes::complete::tag;
///
/// let mut parser = terminated(tag("abc"), tag("efg"));
///
/// assert_eq!(parser.parse("abcefg"), Ok(("", "abc")));
/// assert_eq!(parser.parse("abcefghij"), Ok(("hij", "abc")));
/// assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser.parse("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn terminated<I, O, E: ParseError<I>, F, G>(
  first: F,
  second: G,
) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Output = O, Error = E>,
  G: Parser<I, Error = E>,
{
  Terminated {
    f: first,
    g: second,
  }
}

/// a
pub struct Terminated<F, G> {
  f: F,
  g: G,
}

impl<I, E: ParseError<I>, F: Parser<I, Error = E>, G: Parser<I, Error = E>> Parser<I>
  for Terminated<F, G>
{
  type Output = <F as Parser<I>>::Output;
  type Error = E;

  #[inline(always)]
  fn process<OM: OutputMode>(&mut self, i: I) -> PResult<OM, I, Self::Output, Self::Error> {
    let (i, o1) = self.f.process::<OM>(i)?;
    let (i, _) = self
      .g
      .process::<OutputM<Check, OM::Error, OM::Incomplete>>(i)?;

    Ok((i, o1))
  }
}

/// Gets an object from the first parser,
/// then matches an object from the sep_parser and discards it,
/// then gets another object from the second parser.
///
/// # Arguments
/// * `first` The first parser to apply.
/// * `sep` The separator parser to apply.
/// * `second` The second parser to apply.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, Parser};
/// # use nom::Needed::Size;
/// use nom::sequence::separated_pair;
/// use nom::bytes::complete::tag;
///
/// let mut parser = separated_pair(tag("abc"), tag("|"), tag("efg"));
///
/// assert_eq!(parser.parse("abc|efg"), Ok(("", ("abc", "efg"))));
/// assert_eq!(parser.parse("abc|efghij"), Ok(("hij", ("abc", "efg"))));
/// assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser.parse("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn separated_pair<I, O1, O2, E: ParseError<I>, F, G, H>(
  first: F,
  sep: G,
  second: H,
) -> impl Parser<I, Output = (O1, O2), Error = E>
where
  F: Parser<I, Output = O1, Error = E>,
  G: Parser<I, Error = E>,
  H: Parser<I, Output = O2, Error = E>,
{
  first.and(preceded(sep, second))
}

/// Matches an object from the first parser and discards it,
/// then gets an object from the second parser,
/// and finally matches an object from the third parser and discards it.
///
/// # Arguments
/// * `first` The first parser to apply and discard.
/// * `second` The second parser to apply.
/// * `third` The third parser to apply and discard.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, Parser};
/// # use nom::Needed::Size;
/// use nom::sequence::delimited;
/// use nom::bytes::complete::tag;
///
/// let mut parser = delimited(tag("("), tag("abc"), tag(")"));
///
/// assert_eq!(parser.parse("(abc)"), Ok(("", "abc")));
/// assert_eq!(parser.parse("(abc)def"), Ok(("def", "abc")));
/// assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
/// assert_eq!(parser.parse("123"), Err(Err::Error(("123", ErrorKind::Tag))));
/// ```
pub fn delimited<I, O, E: ParseError<I>, F, G, H>(
  first: F,
  second: G,
  third: H,
) -> impl Parser<I, Output = O, Error = E>
where
  F: Parser<I, Error = E>,
  G: Parser<I, Output = O, Error = E>,
  H: Parser<I, Error = E>,
{
  preceded(first, terminated(second, third))
}

/// Helper trait for the tuple combinator.
///
/// This trait is implemented for tuples of parsers of up to 21 elements.
#[deprecated(since = "8.0.0", note = "`Parser` is directly implemented for tuples")]
#[allow(deprecated)]
pub trait Tuple<I, O, E> {
  /// Parses the input and returns a tuple of results of each parser.
  fn parse_tuple(&mut self, input: I) -> IResult<I, O, E>;
}

#[allow(deprecated)]
impl<Input, Output, Error: ParseError<Input>, F: Parser<Input, Output = Output, Error = Error>>
  Tuple<Input, (Output,), Error> for (F,)
{
  fn parse_tuple(&mut self, input: Input) -> IResult<Input, (Output,), Error> {
    self.0.parse(input).map(|(i, o)| (i, (o,)))
  }
}

macro_rules! tuple_trait(
  ($name1:ident $ty1:ident, $name2: ident $ty2:ident, $($name:ident $ty:ident),*) => (
    tuple_trait!(__impl $name1 $ty1, $name2 $ty2; $($name $ty),*);
  );
  (__impl $($name:ident $ty: ident),+; $name1:ident $ty1:ident, $($name2:ident $ty2:ident),*) => (
    tuple_trait_impl!($($name $ty),+);
    tuple_trait!(__impl $($name $ty),+ , $name1 $ty1; $($name2 $ty2),*);
  );
  (__impl $($name:ident $ty: ident),+; $name1:ident $ty1:ident) => (
    tuple_trait_impl!($($name $ty),+);
    tuple_trait_impl!($($name $ty),+, $name1 $ty1);
  );
);

macro_rules! tuple_trait_impl(
  ($($name:ident $ty: ident),+) => (
    #[allow(deprecated)]
    impl<
      Input: Clone, $($ty),+ , Error: ParseError<Input>,
      $($name: Parser<Input, Output = $ty, Error = Error>),+
    > Tuple<Input, ( $($ty),+ ), Error> for ( $($name),+ ) {
      fn parse_tuple(&mut self, input: Input) -> IResult<Input, ( $($ty),+ ), Error> {
        tuple_trait_inner!(0, self, input, (), $($name)+)

      }
    }
  );
);

macro_rules! tuple_trait_inner(
  ($it:tt, $self:expr, $input:expr, (), $head:ident $($id:ident)+) => ({
    let (i, o) = $self.$it.parse($input)?;

    succ!($it, tuple_trait_inner!($self, i, ( o ), $($id)+))
  });
  ($it:tt, $self:expr, $input:expr, ($($parsed:tt)*), $head:ident $($id:ident)+) => ({
    let (i, o) = $self.$it.parse($input)?;

    succ!($it, tuple_trait_inner!($self, i, ($($parsed)* , o), $($id)+))
  });
  ($it:tt, $self:expr, $input:expr, ($($parsed:tt)*), $head:ident) => ({
    let (i, o) = $self.$it.parse($input)?;

    Ok((i, ($($parsed)* , o)))
  });
);

tuple_trait!(FnA A, FnB B, FnC C, FnD D, FnE E, FnF F, FnG G, FnH H, FnI I, FnJ J, FnK K, FnL L,
  FnM M, FnN N, FnO O, FnP P, FnQ Q, FnR R, FnS S, FnT T, FnU U);

// Special case: implement `Tuple` for `()`, the unit type.
// This can come up in macros which accept a variable number of arguments.
// Literally, `()` is an empty tuple, so it should simply parse nothing.
#[allow(deprecated)]
impl<I, E: ParseError<I>> Tuple<I, (), E> for () {
  fn parse_tuple(&mut self, input: I) -> IResult<I, (), E> {
    Ok((input, ()))
  }
}

///Applies a tuple of parsers one by one and returns their results as a tuple.
///There is a maximum of 21 parsers
/// ```rust
/// # use nom::{Err, error::ErrorKind};
/// use nom::sequence::tuple;
/// use nom::character::complete::{alpha1, digit1};
/// let mut parser = tuple((alpha1, digit1, alpha1));
///
/// assert_eq!(parser("abc123def"), Ok(("", ("abc", "123", "def"))));
/// assert_eq!(parser("123def"), Err(Err::Error(("123def", ErrorKind::Alpha))));
/// ```
#[deprecated(since = "8.0.0", note = "`Parser` is directly implemented for tuples")]
#[allow(deprecated)]
pub fn tuple<I, O, E: ParseError<I>, List: Tuple<I, O, E>>(
  mut l: List,
) -> impl FnMut(I) -> IResult<I, O, E> {
  move |i: I| l.parse_tuple(i)
}
