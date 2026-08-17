use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::streaming::digit1 as digit;
use nom::combinator::{map, map_res, opt, recognize};
use nom::error::ErrorKind;
use nom::number::complete::f32;
use nom::number::complete::f64;
use nom::number::Endianness;
use nom::sequence::{delimited, pair};
use nom::Err;
use nom::{IResult, Parser};
use std::str;
use std::str::FromStr;

fn unsigned_float(i: &[u8]) -> IResult<&[u8], f32> {
  let float_bytes = recognize(alt((
    delimited(digit, tag("."), opt(digit)),
    delimited(opt(digit), tag("."), digit),
  )));
  let float_str = map_res(float_bytes, str::from_utf8);
  map_res(float_str, FromStr::from_str).parse(i)
}

fn float(i: &[u8]) -> IResult<&[u8], f32> {
  map(
    pair(opt(alt((tag("+"), tag("-")))), unsigned_float),
    |(sign, value)| {
      sign
        .and_then(|s| if s[0] == b'-' { Some(-1f32) } else { None })
        .unwrap_or(1f32)
        * value
    },
  )
  .parse(i)
}

#[test]
fn unsigned_float_test() {
  assert_eq!(unsigned_float(&b"123.456;"[..]), Ok((&b";"[..], 123.456)));
  assert_eq!(unsigned_float(&b"0.123;"[..]), Ok((&b";"[..], 0.123)));
  assert_eq!(unsigned_float(&b"123.0;"[..]), Ok((&b";"[..], 123.0)));
  assert_eq!(unsigned_float(&b"123.;"[..]), Ok((&b";"[..], 123.0)));
  assert_eq!(unsigned_float(&b".123;"[..]), Ok((&b";"[..], 0.123)));
}

#[test]
fn float_test() {
  assert_eq!(float(&b"123.456;"[..]), Ok((&b";"[..], 123.456)));
  assert_eq!(float(&b"+123.456;"[..]), Ok((&b";"[..], 123.456)));
  assert_eq!(float(&b"-123.456;"[..]), Ok((&b";"[..], -123.456)));
}

#[test]
fn test_f32_big_endian() {
  let be_f32 = |s| f32::<_, (_, ErrorKind)>(Endianness::Big)(s);

  assert_eq!(
    be_f32(&[0x41, 0x48, 0x00, 0x00][..]),
    Ok((&[] as &[u8], 12.5))
  );
}

#[test]
fn test_f32_little_endian() {
  let le_f32 = |s| f32::<_, (_, ErrorKind)>(Endianness::Little)(s);

  assert_eq!(
    le_f32(&[0x00, 0x00, 0x48, 0x41][..]),
    Ok((&[] as &[u8], 12.5))
  );
}

#[test]
fn test_f64_big_endian() {
  let be_f64 = |s| f64::<&[u8], (&[u8], ErrorKind)>(Endianness::Big)(s);

  let input = &[0x40, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..];
  let expected = 12.5f64;
  match be_f64(input) {
    Ok((rest, value)) => {
      assert!(rest.is_empty());
      assert_eq!(value, expected);
    }
    Err(_) => assert!(false, "Failed to parse big-endian f64"),
  }

  let incomplete_input = &b"abc"[..];
  assert!(matches!(be_f64(incomplete_input), Err(Err::Error(_))));
}

#[test]
fn test_f64_little_endian() {
  let le_f64 = |s| f64::<&[u8], (&[u8], ErrorKind)>(Endianness::Little)(s);

  let input = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x40][..];
  let expected = 12.5f64;
  match le_f64(input) {
    Ok((rest, value)) => {
      assert!(rest.is_empty());
      assert_eq!(value, expected);
    }
    Err(_) => assert!(false, "Failed to parse little-endian f64"),
  }

  let incomplete_input = &b"abc"[..];
  assert!(matches!(le_f64(incomplete_input), Err(Err::Error(_))));
}
