// (C) Copyright 2016 Jethro G. Beekman
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Parsing C literals from byte slices.
//!
//! This will parse a representation of a C literal into a Rust type.
//!
//! # characters
//! Character literals are stored into the `CChar` type, which can hold values
//! that are not valid Unicode code points. ASCII characters are represented as
//! `char`, literal bytes with the high byte set are converted into the raw
//! representation. Escape sequences are supported. If hex and octal escapes
//! map to an ASCII character, that is used, otherwise, the raw encoding is
//! used, including for values over 255. Unicode escapes are checked for
//! validity and mapped to `char`. Character sequences are not supported. Width
//! prefixes are ignored.
//!
//! # strings
//! Strings are interpreted as byte vectors. Escape sequences are supported. If
//! hex and octal escapes map onto multi-byte characters, they are truncated to
//! one 8-bit character. Unicode escapes are converted into their UTF-8
//! encoding. Width prefixes are ignored.
//!
//! # integers
//! Integers are read into `i64`. Binary, octal, decimal and hexadecimal are
//! all supported. If the literal value is between `i64::MAX` and `u64::MAX`,
//! it is bit-cast to `i64`. Values over `u64::MAX` cannot be parsed. Width and
//! sign suffixes are ignored. Sign prefixes are not supported.
//!
//! # real numbers
//! Reals are read into `f64`. Width suffixes are ignored. Sign prefixes are
//! not supported in the significand. Hexadecimal floating points are not
//! supported.

use std::char;
use std::str::{self, FromStr};

use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{complete, map, map_opt, opt, recognize};
use nom::multi::{fold_many0, many0, many1, many_m_n};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::*;

use crate::expr::EvalResult;
use crate::ToCexprResult;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Representation of a C character
pub enum CChar {
    /// A character that can be represented as a `char`
    Char(char),
    /// Any other character (8-bit characters, unicode surrogates, etc.)
    Raw(u64),
}

impl From<u8> for CChar {
    fn from(i: u8) -> CChar {
        match i {
            0..=0x7f => CChar::Char(i as u8 as char),
            _ => CChar::Raw(i as u64),
        }
    }
}

// A non-allocating version of this would be nice...
impl std::convert::Into<Vec<u8>> for CChar {
    fn into(self) -> Vec<u8> {
        match self {
            CChar::Char(c) => {
                let mut s = String::with_capacity(4);
                s.extend(&[c]);
                s.into_bytes()
            }
            CChar::Raw(i) => {
                let mut v = Vec::with_capacity(1);
                v.push(i as u8);
                v
            }
        }
    }
}

/// ensures the child parser consumes the whole input
pub fn full<I: Clone, O, F>(
    f: F,
) -> impl Fn(I) -> nom::IResult<I, O>
where
    I: nom::InputLength,
    F: Fn(I) -> nom::IResult<I, O>,
{
    move |input| {
        let res = f(input);
        match res {
            Ok((i, o)) => {
                if i.input_len() == 0 {
                    Ok((i, o))
                } else {
                    Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)))
                }
            }
            r => r,
        }
    }
}

// =================================
// ======== matching digits ========
// =================================

macro_rules! byte {
	($($p: pat)|* ) => {{
        fn parser(i: &[u8]) -> crate::nom::IResult<&[u8], u8> {
            match i.split_first() {
                $(Some((&c @ $p,rest)))|* => Ok((rest,c)),
                Some(_) => Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::OneOf))),
                None => Err(nom::Err::Incomplete(Needed::new(1))),
            }
        }

        parser
	}}
}

fn binary(i: &[u8]) -> nom::IResult<&[u8], u8> {
    byte!(b'0'..=b'1')(i)
}

fn octal(i: &[u8]) -> nom::IResult<&[u8], u8> {
    byte!(b'0'..=b'7')(i)
}

fn decimal(i: &[u8]) -> nom::IResult<&[u8], u8> {
    byte!(b'0'..=b'9')(i)
}

fn hexadecimal(i: &[u8]) -> nom::IResult<&[u8], u8> {
    byte!(b'0' ..= b'9' | b'a' ..= b'f' | b'A' ..= b'F')(i)
}

// ========================================
// ======== characters and strings ========
// ========================================

fn escape2char(c: char) -> CChar {
    CChar::Char(match c {
        'a' => '\x07',
        'b' => '\x08',
        'f' => '\x0c',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'v' => '\x0b',
        _ => unreachable!("invalid escape {}", c),
    })
}

fn c_raw_escape(n: Vec<u8>, radix: u32) -> Option<CChar> {
    str::from_utf8(&n)
        .ok()
        .and_then(|i| u64::from_str_radix(i, radix).ok())
        .map(|i| match i {
            0..=0x7f => CChar::Char(i as u8 as char),
            _ => CChar::Raw(i),
        })
}

fn c_unicode_escape(n: Vec<u8>) -> Option<CChar> {
    str::from_utf8(&n)
        .ok()
        .and_then(|i| u32::from_str_radix(i, 16).ok())
        .and_then(char::from_u32)
        .map(CChar::Char)
}

fn escaped_char(i: &[u8]) -> nom::IResult<&[u8], CChar> {
    preceded(
        char('\\'),
        alt((
            map(one_of(r#"'"?\"#), CChar::Char),
            map(one_of("abfnrtv"), escape2char),
            map_opt(many_m_n(1, 3, octal), |v| c_raw_escape(v, 8)),
            map_opt(preceded(char('x'), many1(hexadecimal)), |v| {
                c_raw_escape(v, 16)
            }),
            map_opt(
                preceded(char('u'), many_m_n(4, 4, hexadecimal)),
                c_unicode_escape,
            ),
            map_opt(
                preceded(char('U'), many_m_n(8, 8, hexadecimal)),
                c_unicode_escape,
            ),
        )),
    )(i)
}

fn c_width_prefix(i: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    alt((tag("u8"), tag("u"), tag("U"), tag("L")))(i)
}

fn c_char(i: &[u8]) -> nom::IResult<&[u8], CChar> {
    delimited(
        terminated(opt(c_width_prefix), char('\'')),
        alt((
            escaped_char,
            map(byte!(0 ..= 91 /* \=92 */ | 93 ..= 255), CChar::from),
        )),
        char('\''),
    )(i)
}

fn c_string(i: &[u8]) -> nom::IResult<&[u8], Vec<u8>> {
    delimited(
        alt((preceded(c_width_prefix, char('"')), char('"'))),
        fold_many0(
            alt((
                map(escaped_char, |c: CChar| c.into()),
                map(is_not([b'\\', b'"']), |c: &[u8]| c.into()),
            )),
            Vec::new,
            |mut v: Vec<u8>, res: Vec<u8>| {
                v.extend_from_slice(&res);
                v
            },
        ),
        char('"'),
    )(i)
}

// ================================
// ======== parse integers ========
// ================================

fn c_int_radix(n: Vec<u8>, radix: u32) -> Option<u64> {
    str::from_utf8(&n)
        .ok()
        .and_then(|i| u64::from_str_radix(i, radix).ok())
}

fn take_ul(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let r = input.split_at_position(|c| c != b'u' && c != b'U' && c != b'l' && c != b'L');
    match r {
        Err(Err::Incomplete(_)) => Ok((&input[input.len()..], input)),
        res => res,
    }
}

fn c_int(i: &[u8]) -> nom::IResult<&[u8], i64> {
    map(
        terminated(
            alt((
                map_opt(preceded(tag("0x"), many1(complete(hexadecimal))), |v| {
                    c_int_radix(v, 16)
                }),
                map_opt(preceded(tag("0X"), many1(complete(hexadecimal))), |v| {
                    c_int_radix(v, 16)
                }),
                map_opt(preceded(tag("0b"), many1(complete(binary))), |v| {
                    c_int_radix(v, 2)
                }),
                map_opt(preceded(tag("0B"), many1(complete(binary))), |v| {
                    c_int_radix(v, 2)
                }),
                map_opt(preceded(char('0'), many1(complete(octal))), |v| {
                    c_int_radix(v, 8)
                }),
                map_opt(many1(complete(decimal)), |v| c_int_radix(v, 10)),
                |input| Err(crate::nom::Err::Error(nom::error::Error::new(input, crate::nom::ErrorKind::Fix))),
            )),
            opt(take_ul),
        ),
        |i| i as i64,
    )(i)
}

// ==============================
// ======== parse floats ========
// ==============================

fn float_width(i: &[u8]) -> nom::IResult<&[u8], u8> {
    nom::combinator::complete(byte!(b'f' | b'l' | b'F' | b'L'))(i)
}

fn float_exp(i: &[u8]) -> nom::IResult<&[u8], (Option<u8>, Vec<u8>)> {
    preceded(
        byte!(b'e' | b'E'),
        pair(opt(byte!(b'-' | b'+')), many1(complete(decimal))),
    )(i)
}

fn c_float(i: &[u8]) -> nom::IResult<&[u8], f64> {
    map_opt(
        alt((
            terminated(
                recognize(tuple((
                    many1(complete(decimal)),
                    byte!(b'.'),
                    many0(complete(decimal)),
                ))),
                opt(float_width),
            ),
            terminated(
                recognize(tuple((
                    many0(complete(decimal)),
                    byte!(b'.'),
                    many1(complete(decimal)),
                ))),
                opt(float_width),
            ),
            terminated(
                recognize(tuple((
                    many0(complete(decimal)),
                    opt(byte!(b'.')),
                    many1(complete(decimal)),
                    float_exp,
                ))),
                opt(float_width),
            ),
            terminated(
                recognize(tuple((
                    many1(complete(decimal)),
                    opt(byte!(b'.')),
                    many0(complete(decimal)),
                    float_exp,
                ))),
                opt(float_width),
            ),
            terminated(recognize(many1(complete(decimal))), float_width),
        )),
        |v| str::from_utf8(v).ok().and_then(|i| f64::from_str(i).ok()),
    )(i)
}

// ================================
// ======== main interface ========
// ================================

fn one_literal(input: &[u8]) -> nom::IResult<&[u8], EvalResult, crate::Error<&[u8]>> {
    alt((
        map(full(c_char), EvalResult::Char),
        map(full(c_int), |i| EvalResult::Int(::std::num::Wrapping(i))),
        map(full(c_float), EvalResult::Float),
        map(full(c_string), EvalResult::Str),
    ))(input)
    .to_cexpr_result()
}

/// Parse a C literal.
///
/// The input must contain exactly the representation of a single literal
/// token, and in particular no whitespace or sign prefixes.
pub fn parse(input: &[u8]) -> IResult<&[u8], EvalResult, crate::Error<&[u8]>> {
    crate::assert_full_parse(one_literal(input))
}
