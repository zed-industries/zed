#![allow(dead_code)]
// #![allow(unused_variables)]

use std::str;

use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::{map, map_res};
use nom::multi::fold;
use nom::sequence::delimited;
use nom::{IResult, Parser};

fn atom(_tomb: &mut ()) -> impl for<'a> FnMut(&'a [u8]) -> IResult<&'a [u8], String> {
  move |input| {
    map(
      map_res(is_not(" \t\r\n"), str::from_utf8),
      ToString::to_string,
    )
    .parse(input)
  }
}

// FIXME: should we support the use case of borrowing data mutably in a parser?
fn list<'a>(i: &'a [u8], tomb: &mut ()) -> IResult<&'a [u8], String> {
  delimited(
    char('('),
    fold(0.., atom(tomb), String::new, |acc: String, next: String| {
      acc + next.as_str()
    }),
    char(')'),
  )
  .parse(i)
}
