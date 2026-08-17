#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use core::str;

#[cfg(feature = "std")]
use std::str;

use combine::{
    error::UnexpectedParse,
    parser::{
        byte::digit,
        choice::optional,
        range::recognize,
        repeat::{skip_many, skip_many1},
        token::token,
    },
    Parser,
};

fn main() {
    let mut parser = recognize((
        skip_many1(digit()),
        optional((token(b'.'), skip_many(digit()))),
    ))
    .and_then(|bs: &[u8]| {
        // `bs` only contains digits which are ascii and thus UTF-8
        let s = unsafe { str::from_utf8_unchecked(bs) };
        s.parse::<f64>().map_err(|_| UnexpectedParse::Unexpected)
    });
    let result = parser.parse(&b"123.45"[..]);
    assert_eq!(result, Ok((123.45, &b""[..])));
}
