use super::{length_data, length_value, many0_count, many1_count};
use crate::{
  bytes::streaming::tag,
  character::streaming::digit1 as digit,
  error::{ErrorKind, ParseError},
  internal::{Err, IResult, Needed},
  lib::std::str::{self, FromStr},
  number::streaming::{be_u16, be_u8},
  sequence::pair,
  Parser,
};
#[cfg(feature = "alloc")]
use crate::{
  lib::std::vec::Vec,
  multi::{
    count, fold, fold_many0, fold_many1, fold_many_m_n, length_count, many, many0, many1, many_m_n,
    many_till, separated_list0, separated_list1,
  },
};

#[test]
#[cfg(feature = "alloc")]
fn separated_list0_test() {
  use core::num::NonZeroUsize;

  fn multi(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list0(tag(","), tag("abcd")).parse(i)
  }
  fn multi_empty(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list0(tag(","), tag("")).parse(i)
  }
  fn empty_sep(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list0(tag(""), tag("abc")).parse(i)
  }
  fn multi_longsep(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list0(tag(".."), tag("abcd")).parse(i)
  }
  fn empty_both(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list0(tag(""), tag("")).parse(i)
  }

  let a = &b"abcdef"[..];
  let b = &b"abcd,abcdef"[..];
  let c = &b"azerty"[..];
  let d = &b",,abc"[..];
  let e = &b"abcd,abcd,ef"[..];
  let f = &b"abc"[..];
  let g = &b"abcd."[..];
  let h = &b"abcd,abc"[..];
  let i = &b"abcabc"[..];

  let res1 = vec![&b"abcd"[..]];
  assert_eq!(multi(a), Ok((&b"ef"[..], res1)));
  let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
  assert_eq!(multi(b), Ok((&b"ef"[..], res2)));
  assert_eq!(multi(c), Ok((&b"azerty"[..], Vec::new())));
  let res3 = vec![&b""[..], &b""[..], &b""[..]];
  assert_eq!(multi_empty(d), Ok((&b"abc"[..], res3)));
  assert_eq!(
    empty_sep(i),
    Err(Err::Incomplete(Needed::Size(NonZeroUsize::new(3).unwrap())))
  );

  assert_eq!(
    empty_both(i),
    Err(Err::Error(error_position!(i, ErrorKind::SeparatedList)))
  );
  let res4 = vec![&b"abcd"[..], &b"abcd"[..]];
  assert_eq!(multi(e), Ok((&b",ef"[..], res4)));

  assert_eq!(multi(f), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(multi_longsep(g), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(multi(h), Err(Err::Incomplete(Needed::new(1))));
}

#[test]
#[cfg(feature = "alloc")]
fn separated_list1_test() {
  fn multi(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list1(tag(","), tag("abcd")).parse(i)
  }
  fn multi_longsep(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list1(tag(".."), tag("abcd")).parse(i)
  }
  fn empty_both(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list0(tag(""), tag("")).parse(i)
  }

  let a = &b"abcdef"[..];
  let b = &b"abcd,abcdef"[..];
  let c = &b"azerty"[..];
  let d = &b"abcd,abcd,ef"[..];

  let f = &b"abc"[..];
  let g = &b"abcd."[..];
  let h = &b"abcd,abc"[..];

  let res1 = vec![&b"abcd"[..]];
  assert_eq!(multi(a), Ok((&b"ef"[..], res1)));
  let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
  assert_eq!(multi(b), Ok((&b"ef"[..], res2)));
  assert_eq!(
    multi(c),
    Err(Err::Error(error_position!(c, ErrorKind::Tag)))
  );
  assert_eq!(
    empty_both(f),
    Err(Err::Error(error_position!(f, ErrorKind::SeparatedList)))
  );
  let res3 = vec![&b"abcd"[..], &b"abcd"[..]];
  assert_eq!(multi(d), Ok((&b",ef"[..], res3)));

  assert_eq!(multi(f), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(multi_longsep(g), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(multi(h), Err(Err::Incomplete(Needed::new(1))));
}

#[test]
#[cfg(feature = "alloc")]
fn many0_test() {
  fn multi(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many0(tag("abcd")).parse(i)
  }
  fn multi_empty(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many0(tag("")).parse(i)
  }

  assert_eq!(multi(&b"abcdef"[..]), Ok((&b"ef"[..], vec![&b"abcd"[..]])));
  assert_eq!(
    multi(&b"abcdabcdefgh"[..]),
    Ok((&b"efgh"[..], vec![&b"abcd"[..], &b"abcd"[..]]))
  );
  assert_eq!(multi(&b"azerty"[..]), Ok((&b"azerty"[..], Vec::new())));
  assert_eq!(multi(&b"abcdab"[..]), Err(Err::Incomplete(Needed::new(2))));
  assert_eq!(multi(&b"abcd"[..]), Err(Err::Incomplete(Needed::new(4))));
  assert_eq!(multi(&b""[..]), Err(Err::Incomplete(Needed::new(4))));
  assert_eq!(
    multi_empty(&b"abcdef"[..]),
    Err(Err::Error(error_position!(
      &b"abcdef"[..],
      ErrorKind::Many0
    )))
  );
}

#[test]
#[cfg(feature = "alloc")]
fn many1_test() {
  fn multi(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many1(tag("abcd")).parse(i)
  }

  let a = &b"abcdef"[..];
  let b = &b"abcdabcdefgh"[..];
  let c = &b"azerty"[..];
  let d = &b"abcdab"[..];

  let res1 = vec![&b"abcd"[..]];
  assert_eq!(multi(a), Ok((&b"ef"[..], res1)));
  let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
  assert_eq!(multi(b), Ok((&b"efgh"[..], res2)));
  assert_eq!(
    multi(c),
    Err(Err::Error(error_position!(c, ErrorKind::Tag)))
  );
  assert_eq!(multi(d), Err(Err::Incomplete(Needed::new(2))));
}

#[test]
#[cfg(feature = "alloc")]
fn many_till_test() {
  #[allow(clippy::type_complexity)]
  fn multi(i: &[u8]) -> IResult<&[u8], (Vec<&[u8]>, &[u8])> {
    many_till(tag("abcd"), tag("efgh")).parse(i)
  }

  let a = b"abcdabcdefghabcd";
  let b = b"efghabcd";
  let c = b"azerty";

  let res_a = (vec![&b"abcd"[..], &b"abcd"[..]], &b"efgh"[..]);
  let res_b: (Vec<&[u8]>, &[u8]) = (Vec::new(), &b"efgh"[..]);
  assert_eq!(multi(&a[..]), Ok((&b"abcd"[..], res_a)));
  assert_eq!(multi(&b[..]), Ok((&b"abcd"[..], res_b)));
  assert_eq!(
    multi(&c[..]),
    Err(Err::Error(error_node_position!(
      &c[..],
      ErrorKind::ManyTill,
      error_position!(&c[..], ErrorKind::Tag)
    )))
  );
}

#[test]
#[cfg(feature = "std")]
fn infinite_many() {
  fn tst(input: &[u8]) -> IResult<&[u8], &[u8]> {
    Err(Err::Error(error_position!(input, ErrorKind::Tag)))
  }

  // should not go into an infinite loop
  fn multi0(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many0(tst).parse(i)
  }
  let a = &b"abcdef"[..];
  assert_eq!(multi0(a), Ok((a, Vec::new())));

  fn multi1(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many1(tst).parse(i)
  }
  let a = &b"abcdef"[..];
  assert_eq!(
    multi1(a),
    Err(Err::Error(error_position!(a, ErrorKind::Tag)))
  );
}

#[test]
#[cfg(feature = "alloc")]
fn many_m_n_test() {
  fn multi(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many_m_n(2, 4, tag("Abcd")).parse(i)
  }

  let a = &b"Abcdef"[..];
  let b = &b"AbcdAbcdefgh"[..];
  let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
  let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
  let e = &b"AbcdAb"[..];

  assert_eq!(
    multi(a),
    Err(Err::Error(error_position!(&b"ef"[..], ErrorKind::Tag)))
  );
  let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(multi(b), Ok((&b"efgh"[..], res1)));
  let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(multi(c), Ok((&b"efgh"[..], res2)));
  let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(multi(d), Ok((&b"Abcdefgh"[..], res3)));
  assert_eq!(multi(e), Err(Err::Incomplete(Needed::new(2))));
}

#[test]
#[cfg(feature = "alloc")]
fn count_test() {
  const TIMES: usize = 2;
  fn cnt_2(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    count(tag("abc"), TIMES).parse(i)
  }

  assert_eq!(
    cnt_2(&b"abcabcabcdef"[..]),
    Ok((&b"abcdef"[..], vec![&b"abc"[..], &b"abc"[..]]))
  );
  assert_eq!(cnt_2(&b"ab"[..]), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(cnt_2(&b"abcab"[..]), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(
    cnt_2(&b"xxx"[..]),
    Err(Err::Error(error_position!(&b"xxx"[..], ErrorKind::Tag)))
  );
  assert_eq!(
    cnt_2(&b"xxxabcabcdef"[..]),
    Err(Err::Error(error_position!(
      &b"xxxabcabcdef"[..],
      ErrorKind::Tag
    )))
  );
  assert_eq!(
    cnt_2(&b"abcxxxabcdef"[..]),
    Err(Err::Error(error_position!(
      &b"xxxabcdef"[..],
      ErrorKind::Tag
    )))
  );
}

#[test]
#[cfg(feature = "alloc")]
fn count_zero() {
  const TIMES: usize = 0;
  fn counter_2(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    count(tag("abc"), TIMES).parse(i)
  }

  let done = &b"abcabcabcdef"[..];
  let parsed_done = Vec::new();
  let rest = done;
  let incomplete_1 = &b"ab"[..];
  let parsed_incompl_1 = Vec::new();
  let incomplete_2 = &b"abcab"[..];
  let parsed_incompl_2 = Vec::new();
  let error = &b"xxx"[..];
  let error_remain = &b"xxx"[..];
  let parsed_err = Vec::new();
  let error_1 = &b"xxxabcabcdef"[..];
  let parsed_err_1 = Vec::new();
  let error_1_remain = &b"xxxabcabcdef"[..];
  let error_2 = &b"abcxxxabcdef"[..];
  let parsed_err_2 = Vec::new();
  let error_2_remain = &b"abcxxxabcdef"[..];

  assert_eq!(counter_2(done), Ok((rest, parsed_done)));
  assert_eq!(
    counter_2(incomplete_1),
    Ok((incomplete_1, parsed_incompl_1))
  );
  assert_eq!(
    counter_2(incomplete_2),
    Ok((incomplete_2, parsed_incompl_2))
  );
  assert_eq!(counter_2(error), Ok((error_remain, parsed_err)));
  assert_eq!(counter_2(error_1), Ok((error_1_remain, parsed_err_1)));
  assert_eq!(counter_2(error_2), Ok((error_2_remain, parsed_err_2)));
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NilError;

impl<I> From<(I, ErrorKind)> for NilError {
  fn from(_: (I, ErrorKind)) -> Self {
    NilError
  }
}

impl<I> ParseError<I> for NilError {
  fn from_error_kind(_: I, _: ErrorKind) -> NilError {
    NilError
  }
  fn append(_: I, _: ErrorKind, _: NilError) -> NilError {
    NilError
  }
}

fn number(i: &[u8]) -> IResult<&[u8], u32> {
  use crate::combinator::map_res;

  map_res(map_res(digit, str::from_utf8), FromStr::from_str).parse(i)
}

#[test]
#[cfg(feature = "alloc")]
fn length_count_test() {
  fn cnt(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    length_count(number, tag("abc")).parse(i)
  }

  assert_eq!(
    cnt(&b"2abcabcabcdef"[..]),
    Ok((&b"abcdef"[..], vec![&b"abc"[..], &b"abc"[..]]))
  );
  assert_eq!(cnt(&b"2ab"[..]), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(cnt(&b"3abcab"[..]), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(
    cnt(&b"xxx"[..]),
    Err(Err::Error(error_position!(&b"xxx"[..], ErrorKind::Digit)))
  );
  assert_eq!(
    cnt(&b"2abcxxx"[..]),
    Err(Err::Error(error_position!(&b"xxx"[..], ErrorKind::Tag)))
  );
}

#[test]
fn length_data_test() {
  fn take(i: &[u8]) -> IResult<&[u8], &[u8]> {
    length_data(number).parse(i)
  }

  assert_eq!(
    take(&b"6abcabcabcdef"[..]),
    Ok((&b"abcdef"[..], &b"abcabc"[..]))
  );
  assert_eq!(take(&b"3ab"[..]), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(
    take(&b"xxx"[..]),
    Err(Err::Error(error_position!(&b"xxx"[..], ErrorKind::Digit)))
  );
  assert_eq!(take(&b"2abcxxx"[..]), Ok((&b"cxxx"[..], &b"ab"[..])));
}

#[test]
fn length_value_test() {
  fn length_value_1(i: &[u8]) -> IResult<&[u8], u16> {
    length_value(be_u8, be_u16).parse(i)
  }
  fn length_value_2(i: &[u8]) -> IResult<&[u8], (u8, u8)> {
    length_value(be_u8, (be_u8, be_u8)).parse(i)
  }

  let i1 = [0, 5, 6];
  assert_eq!(
    length_value_1(&i1),
    Err(Err::Error(error_position!(&b""[..], ErrorKind::Complete)))
  );
  assert_eq!(
    length_value_2(&i1),
    Err(Err::Error(error_position!(&b""[..], ErrorKind::Complete)))
  );

  let i2 = [1, 5, 6, 3];
  assert_eq!(
    length_value_1(&i2),
    Err(Err::Error(error_position!(&i2[1..2], ErrorKind::Complete)))
  );
  assert_eq!(
    length_value_2(&i2),
    Err(Err::Error(error_position!(&i2[1..2], ErrorKind::Complete)))
  );

  let i3 = [2, 5, 6, 3, 4, 5, 7];
  assert_eq!(length_value_1(&i3), Ok((&i3[3..], 1286)));
  assert_eq!(length_value_2(&i3), Ok((&i3[3..], (5, 6))));

  let i4 = [3, 5, 6, 3, 4, 5];
  assert_eq!(length_value_1(&i4), Ok((&i4[4..], 1286)));
  assert_eq!(length_value_2(&i4), Ok((&i4[4..], (5, 6))));
}

#[test]
#[cfg(feature = "alloc")]
fn fold_many0_test() {
  fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
    acc.push(item);
    acc
  }
  fn multi(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold_many0(tag("abcd"), Vec::new, fold_into_vec).parse(i)
  }
  fn multi_empty(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold_many0(tag(""), Vec::new, fold_into_vec).parse(i)
  }

  assert_eq!(multi(&b"abcdef"[..]), Ok((&b"ef"[..], vec![&b"abcd"[..]])));
  assert_eq!(
    multi(&b"abcdabcdefgh"[..]),
    Ok((&b"efgh"[..], vec![&b"abcd"[..], &b"abcd"[..]]))
  );
  assert_eq!(multi(&b"azerty"[..]), Ok((&b"azerty"[..], Vec::new())));
  assert_eq!(multi(&b"abcdab"[..]), Err(Err::Incomplete(Needed::new(2))));
  assert_eq!(multi(&b"abcd"[..]), Err(Err::Incomplete(Needed::new(4))));
  assert_eq!(multi(&b""[..]), Err(Err::Incomplete(Needed::new(4))));
  assert_eq!(
    multi_empty(&b"abcdef"[..]),
    Err(Err::Error(error_position!(
      &b"abcdef"[..],
      ErrorKind::Many0
    )))
  );
}

#[test]
#[cfg(feature = "alloc")]
fn fold_many1_test() {
  fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
    acc.push(item);
    acc
  }
  fn multi(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold_many1(tag("abcd"), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"abcdef"[..];
  let b = &b"abcdabcdefgh"[..];
  let c = &b"azerty"[..];
  let d = &b"abcdab"[..];

  let res1 = vec![&b"abcd"[..]];
  assert_eq!(multi(a), Ok((&b"ef"[..], res1)));
  let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
  assert_eq!(multi(b), Ok((&b"efgh"[..], res2)));
  assert_eq!(
    multi(c),
    Err(Err::Error(error_position!(c, ErrorKind::Many1)))
  );
  assert_eq!(multi(d), Err(Err::Incomplete(Needed::new(2))));
}

#[test]
#[cfg(feature = "alloc")]
fn fold_many_m_n_test() {
  fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
    acc.push(item);
    acc
  }
  fn multi(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold_many_m_n(2, 4, tag("Abcd"), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"Abcdef"[..];
  let b = &b"AbcdAbcdefgh"[..];
  let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
  let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
  let e = &b"AbcdAb"[..];

  assert_eq!(
    multi(a),
    Err(Err::Error(error_position!(&b"ef"[..], ErrorKind::Tag)))
  );
  let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(multi(b), Ok((&b"efgh"[..], res1)));
  let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(multi(c), Ok((&b"efgh"[..], res2)));
  let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(multi(d), Ok((&b"Abcdefgh"[..], res3)));
  assert_eq!(multi(e), Err(Err::Incomplete(Needed::new(2))));
}

#[test]
fn many0_count_test() {
  fn count0_nums(i: &[u8]) -> IResult<&[u8], usize> {
    many0_count(pair(digit, tag(","))).parse(i)
  }

  assert_eq!(count0_nums(&b"123,junk"[..]), Ok((&b"junk"[..], 1)));

  assert_eq!(count0_nums(&b"123,45,junk"[..]), Ok((&b"junk"[..], 2)));

  assert_eq!(
    count0_nums(&b"1,2,3,4,5,6,7,8,9,0,junk"[..]),
    Ok((&b"junk"[..], 10))
  );

  assert_eq!(count0_nums(&b"hello"[..]), Ok((&b"hello"[..], 0)));
}

#[test]
fn many1_count_test() {
  fn count1_nums(i: &[u8]) -> IResult<&[u8], usize> {
    many1_count(pair(digit, tag(","))).parse(i)
  }

  assert_eq!(count1_nums(&b"123,45,junk"[..]), Ok((&b"junk"[..], 2)));

  assert_eq!(
    count1_nums(&b"1,2,3,4,5,6,7,8,9,0,junk"[..]),
    Ok((&b"junk"[..], 10))
  );

  assert_eq!(
    count1_nums(&b"hello"[..]),
    Err(Err::Error(error_position!(
      &b"hello"[..],
      ErrorKind::Many1Count
    )))
  );
}

#[test]
#[cfg(feature = "alloc")]
fn many_test() {
  // should not go into an infinite loop
  fn many_error_0(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fn tst(input: &[u8]) -> IResult<&[u8], &[u8]> {
      Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    }
    many(0.., tst).parse(i)
  }

  let a = &b"abcdef"[..];
  assert_eq!(many_error_0(a), Ok((a, Vec::new())));

  fn many_error_1(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fn tst(input: &[u8]) -> IResult<&[u8], &[u8]> {
      Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    }
    many(1.., tst).parse(i)
  }

  let a = &b"abcdef"[..];
  assert_eq!(
    many_error_1(a),
    Err(Err::Error(error_position!(a, ErrorKind::Tag)))
  );

  fn many_error(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many(0.., tag("")).parse(i)
  }

  let a = &b"abcdef"[..];
  assert_eq!(
    many_error(a),
    Err(Err::Error(error_position!(a, ErrorKind::Many)))
  );

  fn many_invalid(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many(crate::lib::std::ops::Range::default(), tag("a")).parse(i)
  }

  let a = &b"a"[..];
  let b = &b"b"[..];
  assert_eq!(
    many_invalid(a),
    Err(Err::Failure(error_position!(a, ErrorKind::Many)))
  );
  assert_eq!(
    many_invalid(b),
    Err(Err::Failure(error_position!(b, ErrorKind::Many)))
  );

  fn many_any(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many(0.., tag("abcd")).parse(i)
  }

  let a = &b"abcdef"[..];
  let b = &b"abcdabcdefgh"[..];
  let c = &b"azerty"[..];
  let d = &b"abcdab"[..];
  let e = &b"abcd"[..];
  let f = &b""[..];
  assert_eq!(many_any(a), Ok((&b"ef"[..], vec![&b"abcd"[..]])));
  assert_eq!(
    many_any(b),
    Ok((&b"efgh"[..], vec![&b"abcd"[..], &b"abcd"[..]]))
  );
  assert_eq!(many_any(c), Ok((&b"azerty"[..], Vec::new())));
  assert_eq!(many_any(d), Err(Err::Incomplete(Needed::new(2))));
  assert_eq!(many_any(e), Err(Err::Incomplete(Needed::new(4))));
  assert_eq!(many_any(f), Err(Err::Incomplete(Needed::new(4))));

  fn many_1(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many(1.., tag("abcd")).parse(i)
  }

  let a = &b"abcdef"[..];
  let b = &b"abcdabcdefgh"[..];
  let c = &b"azerty"[..];
  let d = &b"abcdab"[..];
  let res1 = vec![&b"abcd"[..]];
  assert_eq!(many_1(a), Ok((&b"ef"[..], res1)));
  let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
  assert_eq!(many_1(b), Ok((&b"efgh"[..], res2)));
  assert_eq!(
    many_1(c),
    Err(Err::Error(error_position!(c, ErrorKind::Tag)))
  );
  assert_eq!(many_1(d), Err(Err::Incomplete(Needed::new(2))));

  fn many_m_n(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many(2..=4, tag("Abcd")).parse(i)
  }

  let a = &b"Abcdef"[..];
  let b = &b"AbcdAbcdefgh"[..];
  let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
  let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
  let e = &b"AbcdAb"[..];
  assert_eq!(
    many_m_n(a),
    Err(Err::Error(error_position!(&b"ef"[..], ErrorKind::Tag)))
  );
  let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(many_m_n(b), Ok((&b"efgh"[..], res1)));
  let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(many_m_n(c), Ok((&b"efgh"[..], res2)));
  let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(many_m_n(d), Ok((&b"Abcdefgh"[..], res3)));
  assert_eq!(many_m_n(e), Err(Err::Incomplete(Needed::new(2))));

  fn many_fixed(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many(2, tag("Abcd")).parse(i)
  }

  let a = &b"Abcdef"[..];
  let b = &b"AbcdAbcdefgh"[..];
  let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
  let d = &b"AbcdAb"[..];
  assert_eq!(
    many_fixed(a),
    Err(Err::Error(error_position!(&b"ef"[..], ErrorKind::Tag)))
  );
  let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(many_fixed(b), Ok((&b"efgh"[..], res1)));
  let res2 = vec![&b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(many_fixed(c), Ok((&b"AbcdAbcdefgh"[..], res2)));
  assert_eq!(many_fixed(d), Err(Err::Incomplete(Needed::new(2))));

  fn multi_exclusive(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many(0..2, tag("Abcd")).parse(i)
  }

  let a = &b"AbcdAbcdAbcd"[..];
  let b = &b"AAA"[..];
  let res1 = vec![&b"Abcd"[..]];
  assert_eq!(multi_exclusive(a), Ok((&b"AbcdAbcd"[..], res1)));
  let res2 = vec![];
  assert_eq!(multi_exclusive(b), Ok((&b"AAA"[..], res2)));

  fn many_never(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many(0..=0, tag("A")).parse(i)
  }

  let a = &b"AA"[..];
  let b = &b"B"[..];
  assert_eq!(many_never(a), Ok((&b"AA"[..], Vec::new())));
  assert_eq!(many_never(b), Ok((&b"B"[..], Vec::new())));
}

#[test]
#[cfg(feature = "alloc")]
fn fold_test() {
  fn fold_into_vec<T>(mut acc: Vec<T>, item: T) -> Vec<T> {
    acc.push(item);
    acc
  }

  // should not go into an infinite loop
  fn fold_error_0(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fn tst(input: &[u8]) -> IResult<&[u8], &[u8]> {
      Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    }
    fold(0.., tst, Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"abcdef"[..];
  assert_eq!(fold_error_0(a), Ok((a, Vec::new())));

  fn fold_error_1(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fn tst(input: &[u8]) -> IResult<&[u8], &[u8]> {
      Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    }
    fold(1.., tst, Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"abcdef"[..];
  assert_eq!(
    fold_error_1(a),
    Err(Err::Error(error_position!(a, ErrorKind::Tag)))
  );

  fn fold_error(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold(0.., tag(""), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"abcdef"[..];
  assert_eq!(
    fold_error(a),
    Err(Err::Error(error_position!(a, ErrorKind::Fold)))
  );

  fn fold_invalid(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold(
      crate::lib::std::ops::Range::default(),
      tag("a"),
      Vec::new,
      fold_into_vec,
    )
    .parse(i)
  }

  let a = &b"a"[..];
  let b = &b"b"[..];
  assert_eq!(
    fold_invalid(a),
    Err(Err::Failure(error_position!(a, ErrorKind::Fold)))
  );
  assert_eq!(
    fold_invalid(b),
    Err(Err::Failure(error_position!(b, ErrorKind::Fold)))
  );

  fn fold_any(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold(0.., tag("abcd"), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"abcdef"[..];
  let b = &b"abcdabcdefgh"[..];
  let c = &b"azerty"[..];
  let d = &b"abcdab"[..];
  let e = &b"abcd"[..];
  let f = &b""[..];
  assert_eq!(fold_any(a), Ok((&b"ef"[..], vec![&b"abcd"[..]])));
  assert_eq!(
    fold_any(b),
    Ok((&b"efgh"[..], vec![&b"abcd"[..], &b"abcd"[..]]))
  );
  assert_eq!(fold_any(c), Ok((c, Vec::new())));
  assert_eq!(fold_any(d), Err(Err::Incomplete(Needed::new(2))));
  assert_eq!(fold_any(e), Err(Err::Incomplete(Needed::new(4))));
  assert_eq!(fold_any(f), Err(Err::Incomplete(Needed::new(4))));

  fn fold_1(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold(1.., tag("abcd"), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"abcdef"[..];
  let b = &b"abcdabcdefgh"[..];
  let c = &b"azerty"[..];
  let d = &b"abcdab"[..];
  let res1 = vec![&b"abcd"[..]];
  assert_eq!(fold_1(a), Ok((&b"ef"[..], res1)));
  let res2 = vec![&b"abcd"[..], &b"abcd"[..]];
  assert_eq!(fold_1(b), Ok((&b"efgh"[..], res2)));
  assert_eq!(
    fold_1(c),
    Err(Err::Error(error_position!(c, ErrorKind::Tag)))
  );
  assert_eq!(fold_1(d), Err(Err::Incomplete(Needed::new(2))));

  fn fold_m_n(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold(2..=4, tag("Abcd"), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"Abcdef"[..];
  let b = &b"AbcdAbcdefgh"[..];
  let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
  let d = &b"AbcdAbcdAbcdAbcdAbcdefgh"[..];
  let e = &b"AbcdAb"[..];
  assert_eq!(
    fold_m_n(a),
    Err(Err::Error(error_position!(&b"ef"[..], ErrorKind::Tag)))
  );
  let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(fold_m_n(b), Ok((&b"efgh"[..], res1)));
  let res2 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(fold_m_n(c), Ok((&b"efgh"[..], res2)));
  let res3 = vec![&b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(fold_m_n(d), Ok((&b"Abcdefgh"[..], res3)));
  assert_eq!(fold_m_n(e), Err(Err::Incomplete(Needed::new(2))));

  fn fold_fixed(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold(2, tag("Abcd"), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"Abcdef"[..];
  let b = &b"AbcdAbcdefgh"[..];
  let c = &b"AbcdAbcdAbcdAbcdefgh"[..];
  let d = &b"AbcdAb"[..];
  assert_eq!(
    fold_fixed(a),
    Err(Err::Error(error_position!(&b"ef"[..], ErrorKind::Tag)))
  );
  let res1 = vec![&b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(fold_fixed(b), Ok((&b"efgh"[..], res1)));
  let res2 = vec![&b"Abcd"[..], &b"Abcd"[..]];
  assert_eq!(fold_fixed(c), Ok((&b"AbcdAbcdefgh"[..], res2)));
  assert_eq!(fold_fixed(d), Err(Err::Incomplete(Needed::new(2))));

  fn fold_exclusive(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold(0..2, tag("Abcd"), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"AbcdAbcdAbcd"[..];
  let b = &b"AAA"[..];
  let res1 = vec![&b"Abcd"[..]];
  assert_eq!(fold_exclusive(a), Ok((&b"AbcdAbcd"[..], res1)));
  let res2 = vec![];
  assert_eq!(fold_exclusive(b), Ok((&b"AAA"[..], res2)));

  fn fold_never(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    fold(0..=0, tag("A"), Vec::new, fold_into_vec).parse(i)
  }

  let a = &b"AAA"[..];
  let b = &b"B"[..];
  assert_eq!(fold_never(a), Ok((&b"AAA"[..], Vec::new())));
  assert_eq!(fold_never(b), Ok((&b"B"[..], Vec::new())));
}
