//#![feature(trace_macros)]
#![allow(dead_code)]
#![allow(clippy::redundant_closure)]

use nom::{error::ErrorKind, Err, IResult, Needed, Parser};

#[allow(dead_code)]
struct Range {
  start: char,
  end: char,
}

pub fn take_char(input: &[u8]) -> IResult<&[u8], char> {
  if !input.is_empty() {
    Ok((&input[1..], input[0] as char))
  } else {
    Err(Err::Incomplete(Needed::new(1)))
  }
}

#[cfg(feature = "std")]
mod parse_int {
  use nom::{
    character::streaming::{digit1 as digit, space1 as space},
    combinator::{complete, map, opt},
    multi::many,
    IResult,
  };
  use nom::{HexDisplay, Parser};
  use std::str;

  fn parse_ints(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    many(0.., spaces_or_int).parse(input)
  }

  fn spaces_or_int(input: &[u8]) -> IResult<&[u8], i32> {
    println!("{}", input.to_hex(8));
    let (i, _) = opt(complete(space)).parse(input)?;
    let (i, res) = map(complete(digit), |x| {
      println!("x: {:?}", x);
      let result = str::from_utf8(x).unwrap();
      println!("Result: {}", result);
      println!("int is empty?: {}", x.is_empty());
      match result.parse() {
        Ok(i) => i,
        Err(e) => panic!("UH OH! NOT A DIGIT! {:?}", e),
      }
    })
    .parse(i)?;

    Ok((i, res))
  }

  #[test]
  fn issue_142() {
    let subject = parse_ints(&b"12 34 5689a"[..]);
    let expected = Ok((&b"a"[..], vec![12, 34, 5689]));
    assert_eq!(subject, expected);

    let subject = parse_ints(&b"12 34 5689 "[..]);
    let expected = Ok((&b" "[..], vec![12, 34, 5689]));
    assert_eq!(subject, expected)
  }
}

#[test]
fn usize_length_bytes_issue() {
  use nom::multi::length_data;
  use nom::number::streaming::be_u16;
  let _: IResult<&[u8], &[u8], (&[u8], ErrorKind)> = length_data(be_u16).parse(b"012346");
}

#[test]
fn take_till_issue() {
  use nom::bytes::streaming::take_till;

  fn nothing(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till(|_| true)(i)
  }

  assert_eq!(nothing(b""), Err(Err::Incomplete(Needed::new(1))));
  assert_eq!(nothing(b"abc"), Ok((&b"abc"[..], &b""[..])));
}

#[test]
fn issue_655() {
  use nom::character::streaming::{line_ending, not_line_ending};
  fn twolines(i: &str) -> IResult<&str, (&str, &str)> {
    let (i, l1) = not_line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, l2) = not_line_ending(i)?;
    let (i, _) = line_ending(i)?;

    Ok((i, (l1, l2)))
  }

  assert_eq!(twolines("foo\nbar\n"), Ok(("", ("foo", "bar"))));
  assert_eq!(twolines("féo\nbar\n"), Ok(("", ("féo", "bar"))));
  assert_eq!(twolines("foé\nbar\n"), Ok(("", ("foé", "bar"))));
  assert_eq!(twolines("foé\r\nbar\n"), Ok(("", ("foé", "bar"))));
}

#[cfg(feature = "alloc")]
fn issue_717(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
  use nom::bytes::complete::{is_not, tag};
  use nom::multi::separated_list0;

  separated_list0(tag(&[0x0][..]), is_not([0x0u8])).parse(i)
}

mod issue_647 {
  use nom::bytes::streaming::tag;
  use nom::combinator::complete;
  use nom::multi::separated_list0;
  use nom::Parser;
  use nom::{error::Error, number::streaming::be_f64, Err, IResult};
  pub type Input<'a> = &'a [u8];

  #[derive(PartialEq, Debug, Clone)]
  struct Data {
    c: f64,
    v: Vec<f64>,
  }

  #[allow(clippy::type_complexity)]
  fn list<'a>(
    input: Input<'a>,
    _cs: &'_ f64,
  ) -> Result<(Input<'a>, Vec<f64>), Err<Error<&'a [u8]>>> {
    separated_list0(complete(tag(",")), complete(be_f64)).parse(input)
  }

  fn data(input: Input<'_>) -> IResult<Input<'_>, Data> {
    let (i, c) = be_f64(input)?;
    let (i, _) = tag("\n")(i)?;
    let (i, v) = list(i, &c)?;
    Ok((i, Data { c, v }))
  }
}

#[test]
fn issue_848_overflow_incomplete_bits_to_bytes() {
  fn take(i: &[u8]) -> IResult<&[u8], &[u8]> {
    use nom::bytes::streaming::take;
    take(0x2000000000000000_usize)(i)
  }
  fn parser(i: &[u8]) -> IResult<&[u8], &[u8]> {
    use nom::bits::{bits, bytes};

    bits(bytes(take))(i)
  }
  assert_eq!(
    parser(&b""[..]),
    Err(Err::Failure(nom::error_position!(
      &b""[..],
      ErrorKind::TooLarge
    )))
  );
}

#[test]
fn issue_942() {
  use nom::error::{ContextError, ParseError};
  pub fn parser<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
  ) -> IResult<&'a str, usize, E> {
    use nom::{character::complete::char, error::context, multi::many0_count};
    many0_count(context("char_a", char('a'))).parse(i)
  }
  assert_eq!(parser::<()>("aaa"), Ok(("", 3)));
}

#[test]
fn issue_many_m_n_with_zeros() {
  use nom::character::complete::char;
  use nom::multi::many;
  let mut parser = many::<_, (), Vec<char>, _, _>(0..=0, char('a'));
  assert_eq!(parser.parse("aaa"), Ok(("aaa", vec!())));
}

#[test]
fn issue_1231_bits_expect_fn_closure() {
  use nom::bits::{bits, complete::take};
  use nom::error::Error;
  pub fn example(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
    bits::<_, _, Error<_>, _, _>((take(1usize), take(1usize)))(input)
  }
  assert_eq!(example(&[0xff]), Ok((&b""[..], (1, 1))));
}

#[test]
fn issue_1282_findtoken_char() {
  use nom::character::complete::one_of;
  use nom::error::Error;
  let mut parser = one_of::<_, _, Error<_>>(&['a', 'b', 'c'][..]);
  assert_eq!(parser("aaa"), Ok(("aa", 'a')));
}

#[test]
fn issue_x_looser_fill_bounds() {
  use nom::{
    bytes::streaming::tag, character::streaming::digit1, error_position, multi::fill,
    sequence::terminated,
  };

  fn fill_pair(i: &[u8]) -> IResult<&[u8], [&[u8]; 2]> {
    let mut buf = [&[][..], &[][..]];
    let (i, _) = fill(terminated(digit1, tag(",")), &mut buf).parse(i)?;
    Ok((i, buf))
  }

  assert_eq!(
    fill_pair(b"123,456,"),
    Ok((&b""[..], [&b"123"[..], &b"456"[..]]))
  );
  assert_eq!(
    fill_pair(b"123,456,789"),
    Ok((&b"789"[..], [&b"123"[..], &b"456"[..]]))
  );
  assert_eq!(
    fill_pair(b"123,,"),
    Err(Err::Error(error_position!(&b","[..], ErrorKind::Digit)))
  );
}

#[test]
fn issue_1459_clamp_capacity() {
  use nom::character::complete::char;

  // shouldn't panic
  use nom::multi::many_m_n;
  let mut parser = many_m_n::<_, (), _>(usize::MAX, usize::MAX, char('a'));
  assert_eq!(parser.parse("a"), Err(nom::Err::Error(())));

  // shouldn't panic
  use nom::multi::count;
  let mut parser = count(char('a'), usize::MAX);
  assert_eq!(parser.parse("a"), Err(nom::Err::Error(())));
}

#[test]
fn issue_1617_count_parser_returning_zero_size() {
  use nom::{bytes::complete::tag, combinator::map, error::Error, multi::count};

  // previously, `count()` panicked if the parser had type `O = ()`
  let parser = map(tag::<_, _, Error<&str>>("abc"), |_| ());
  // shouldn't panic
  let result = count(parser, 3)
    .parse("abcabcabcdef")
    .expect("parsing should succeed");
  assert_eq!(result, ("def", vec![(), (), ()]));
}

#[test]
fn issue_1586_parser_iterator_impl() {
  use nom::{
    character::complete::{digit1, newline},
    combinator::{iterator, opt},
    sequence::terminated,
    IResult,
  };
  fn parse_line(i: &str) -> IResult<&str, &str> {
    terminated(digit1, opt(newline)).parse(i)
  }

  fn parse_input(i: &str) -> impl Iterator<Item = i32> + '_ {
    iterator(i, parse_line).map(|x| x.parse::<i32>().unwrap())
  }

  assert_eq!(parse_input("123\n456").collect::<Vec<_>>(), vec![123, 456]);
}
