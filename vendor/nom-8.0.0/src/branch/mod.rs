//! Choice combinators

#[cfg(test)]
mod tests;

use core::marker::PhantomData;

use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::internal::{Err, Mode, Parser};

/// Tests a list of parsers one by one until one succeeds.
///
/// It takes as argument either a tuple or an array of parsers. If using a
/// tuple, there is a maximum of 21 parsers. If you need more, it is possible to
/// use an array.
///
/// ```rust
/// # use nom::error_position;
/// # use nom::{Err,error::ErrorKind, Needed, IResult, Parser};
/// use nom::character::complete::{alpha1, digit1};
/// use nom::branch::alt;
/// # fn main() {
/// fn parser(input: &str) -> IResult<&str, &str> {
///   alt((alpha1, digit1)).parse(input)
/// };
///
/// // the first parser, alpha1, recognizes the input
/// assert_eq!(parser("abc"), Ok(("", "abc")));
///
/// // the first parser returns an error, so alt tries the second one
/// assert_eq!(parser("123456"), Ok(("", "123456")));
///
/// // both parsers failed, and with the default error type, alt will return the last error
/// assert_eq!(parser(" "), Err(Err::Error(error_position!(" ", ErrorKind::Digit))));
/// # }
/// ```
///
/// With a custom error type, it is possible to have alt return the error of the parser
/// that went the farthest in the input data
pub fn alt<List>(l: List) -> Choice<List> {
  Choice { parser: l }
}

/// Applies a list of parsers in any order.
///
/// Permutation will succeed if all of the child parsers succeeded.
/// It takes as argument a tuple of parsers, and returns a
/// tuple of the parser results.
///
/// ```rust
/// # use nom::{Err,error::{Error, ErrorKind}, Needed, IResult, Parser};
/// use nom::character::complete::{alpha1, digit1};
/// use nom::branch::permutation;
/// # fn main() {
/// fn parser(input: &str) -> IResult<&str, (&str, &str)> {
///   permutation((alpha1, digit1)).parse(input)
/// }
///
/// // permutation recognizes alphabetic characters then digit
/// assert_eq!(parser("abc123"), Ok(("", ("abc", "123"))));
///
/// // but also in inverse order
/// assert_eq!(parser("123abc"), Ok(("", ("abc", "123"))));
///
/// // it will fail if one of the parsers failed
/// assert_eq!(parser("abc;"), Err(Err::Error(Error::new(";", ErrorKind::Digit))));
/// # }
/// ```
///
/// The parsers are applied greedily: if there are multiple unapplied parsers
/// that could parse the next slice of input, the first one is used.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, IResult, Parser};
/// use nom::branch::permutation;
/// use nom::character::complete::{anychar, char};
///
/// fn parser(input: &str) -> IResult<&str, (char, char)> {
///   permutation((anychar, char('a'))).parse(input)
/// }
///
/// // anychar parses 'b', then char('a') parses 'a'
/// assert_eq!(parser("ba"), Ok(("", ('b', 'a'))));
///
/// // anychar parses 'a', then char('a') fails on 'b',
/// // even though char('a') followed by anychar would succeed
/// assert_eq!(parser("ab"), Err(Err::Error(Error::new("b", ErrorKind::Char))));
/// ```
///
pub fn permutation<I: Clone, E: ParseError<I>, List>(list: List) -> Permutation<List, E> {
  Permutation {
    parser: list,
    e: PhantomData,
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

/// Wrapping structure for the [alt()] combinator implementation
pub struct Choice<T> {
  parser: T,
}

macro_rules! alt_trait_impl(
  ($($id:ident)+) => (
    impl<
      Input: Clone, Output, Error: ParseError<Input>,
      $($id: Parser<Input, Output = Output, Error = Error>),+
    > Parser<Input> for Choice< ( $($id),+ )> {
      type Output = Output;
      type Error = Error;

      #[inline(always)]
      fn process<OM: crate::OutputMode>(
        &mut self,
        input: Input,
      ) -> crate::PResult<OM, Input, Self::Output, Self::Error> {
        match self.parser.0.process::<OM>(input.clone()) {
          Ok(res) => Ok(res),
          Err(Err::Failure(e))=> Err(Err::Failure(e)),
          Err(Err::Incomplete(i))=> Err(Err::Incomplete(i)),
          Err(Err::Error(e)) => alt_trait_inner!(1, self, input, e, $($id)+),
        }
      }
    }
  );
);

macro_rules! alt_trait_inner(
  ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident $($id:ident)+) => (
    match $self.parser.$it.process::<OM>($input.clone()) {
      Ok(res) => Ok(res),
      Err(Err::Failure(e))=>Err(Err::Failure(e)),
      Err(Err::Incomplete(i))=> Err(Err::Incomplete(i)),
      Err(Err::Error(e)) => {
        succ!($it, alt_trait_inner!($self, $input, <OM::Error as crate::Mode>::combine($err, e, |e1, e2| e1.or(e2)), $($id)+))
      }
    }
  );
  ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident) => (
    Err(Err::Error(<OM::Error as crate::Mode>::map($err, |err| Error::append($input, ErrorKind::Alt, err))))
  );
);

alt_trait!(A B C D E F G H I J K L M N O P Q R S T U);

// Manually implement Alt for (A,), the 1-tuple type
impl<Input, Output, Error: ParseError<Input>, A: Parser<Input, Output = Output, Error = Error>>
  Parser<Input> for Choice<(A,)>
{
  type Output = Output;
  type Error = Error;

  #[inline]
  fn process<OM: crate::OutputMode>(
    &mut self,
    input: Input,
  ) -> crate::PResult<OM, Input, Self::Output, Self::Error> {
    self.parser.0.process::<OM>(input)
  }
}

impl<
    const N: usize,
    Input: Clone,
    Output,
    Error: ParseError<Input>,
    A: Parser<Input, Output = Output, Error = Error>,
  > Parser<Input> for Choice<[A; N]>
{
  type Output = Output;
  type Error = Error;

  #[inline]
  fn process<OM: crate::OutputMode>(
    &mut self,
    input: Input,
  ) -> crate::PResult<OM, Input, Self::Output, Self::Error> {
    let mut error = None;

    for branch in &mut self.parser {
      match branch.process::<OM>(input.clone()) {
        Err(Err::Error(e)) => match error {
          None => error = Some(e),
          Some(err) => error = Some(OM::Error::combine(err, e, |e1, e2| e1.or(e2))),
        },
        res => return res,
      }
    }

    match error {
      Some(e) => Err(Err::Error(OM::Error::map(e, |err| {
        Error::append(input, ErrorKind::Alt, err)
      }))),
      None => Err(Err::Error(OM::Error::bind(|| {
        Error::from_error_kind(input, ErrorKind::Alt)
      }))),
    }
  }
}

impl<
    Input: Clone,
    Output,
    Error: ParseError<Input>,
    A: Parser<Input, Output = Output, Error = Error>,
  > Parser<Input> for Choice<&mut [A]>
{
  type Output = Output;
  type Error = Error;

  #[inline]
  fn process<OM: crate::OutputMode>(
    &mut self,
    input: Input,
  ) -> crate::PResult<OM, Input, Self::Output, Self::Error> {
    let mut error = None;

    for branch in self.parser.iter_mut() {
      match branch.process::<OM>(input.clone()) {
        Err(Err::Error(e)) => match error {
          None => error = Some(e),
          Some(err) => error = Some(OM::Error::combine(err, e, |e1, e2| e1.or(e2))),
        },
        res => return res,
      }
    }

    match error {
      Some(e) => Err(Err::Error(OM::Error::map(e, |err| {
        Error::append(input, ErrorKind::Alt, err)
      }))),
      None => Err(Err::Error(OM::Error::bind(|| {
        Error::from_error_kind(input, ErrorKind::Alt)
      }))),
    }
  }
}

macro_rules! permutation_trait(
  (
    $name1:ident $ty1:ident $item1:ident
    $name2:ident $ty2:ident $item2:ident
    $($name3:ident $ty3:ident $item3:ident)*
  ) => (
    permutation_trait!(__impl $name1 $ty1 $item1, $name2 $ty2 $item2; $($name3 $ty3 $item3)*);
  );
  (
    __impl $($name:ident $ty:ident $item:ident),+;
    $name1:ident $ty1:ident $item1:ident $($name2:ident $ty2:ident $item2:ident)*
  ) => (
    permutation_trait_impl!($($name $ty $item),+);
    permutation_trait!(__impl $($name $ty $item),+ , $name1 $ty1 $item1; $($name2 $ty2 $item2)*);
  );
  (__impl $($name:ident $ty:ident $item:ident),+;) => (
    permutation_trait_impl!($($name $ty $item),+);
  );
);

/// Wrapping structure for the [permutation] combinator implementation
pub struct Permutation<T, Error> {
  parser: T,
  e: PhantomData<Error>,
}

macro_rules! permutation_trait_impl(
  ($($name:ident $ty:ident $item:ident),+) => (
    impl<
      Input, Error, $($ty),+ , $($name),+
    > Parser<Input> for Permutation< ( $($name),+ ), Error>
    where
    Input: Clone,
    Error: ParseError<Input>,
    $($name: Parser<Input, Output = $ty, Error = Error>),+
    {
      type Output = ( $($ty),+ );
      type Error = Error;

      #[inline(always)]
      fn process<OM: crate::OutputMode>(
        &mut self,
        mut input: Input,
      ) -> crate::PResult<OM, Input, Self::Output, Self::Error> {
        let mut res = OM::Output::bind(|| ($(Option::<$ty>::None),+));
        $(let mut $item = false;)+

        loop {
          let mut err: Option<<OM::Error as Mode>::Output<Error>> = None;
          permutation_trait_inner!(0, self, input, res,  err, $($item)+);

          // If we reach here, every iterator has either been applied before,
          // or errored on the remaining input
          if let Some(err) = err {
            // There are remaining parsers, and all errored on the remaining input
            return Err(Err::Error(OM::Error::map(err, |err| Error::append(input, ErrorKind::Permutation, err))));
          }

          return Ok((input,OM::Output::map(res, |res| {
            // All parsers were applied
            match res {
              ($(Some($item)),+) =>  ($($item),+),
              _ => unreachable!(),
            }
          })))
        }
      }

    }
  );
);

macro_rules! permutation_trait_inner(
  ($it:tt, $self:expr, $input:ident, $res:expr,  $err:expr, $head:ident $($item:ident)*) => (
    if !$head {

      match $self.parser.$it.process::<OM>($input.clone()) {
        Ok((i, o)) => {
          $input = i;
          $res = OM::Output::combine($res, o, |mut res, o | {res.$it = Some(o);res });
          $head = true;
          continue;
        }
        Err(Err::Error(e)) => {
          $err = Some(match $err {
            None => e,
            Some(err) => OM::Error::combine(err, e, |err, e| err.or(e))
          });
        }
        Err(e) => return Err(e),
      };
    }
    succ!($it, permutation_trait_inner!($self, $input, $res, $err, $($item)*));
  );
  ($it:tt, $self:expr, $input:ident, $res:expr, $err:expr,) => ();
);

permutation_trait!(
  FnA A a
  FnB B b
  FnC C c
  FnD D d
  FnE E e
  FnF F f
  FnG G g
  FnH H h
  FnI I i
  FnJ J j
  FnK K k
  FnL L l
  FnM M m
  FnN N n
  FnO O o
  FnP P p
  FnQ Q q
  FnR R r
  FnS S s
  FnT T t
  FnU U u
);
