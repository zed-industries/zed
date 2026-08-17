/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::{BasicParseError, Parser, ParserInput, Token};

/// Parse the *An+B* notation, as found in the `:nth-child()` selector.
/// The input is typically the arguments of a function,
/// in which case the caller needs to check if the arguments’ parser is exhausted.
/// Return `Ok((A, B))`, or an `Err(..)` for a syntax error.
pub fn parse_nth<'i>(input: &mut Parser<'i, '_>) -> Result<(i32, i32), BasicParseError<'i>> {
    match *input.next()? {
        Token::Number {
            int_value: Some(b), ..
        } => Ok((0, b)),
        Token::Dimension {
            int_value: Some(a),
            ref unit,
            ..
        } => {
            match_ignore_ascii_case! {
                unit,
                "n" => Ok(parse_b(input, a)?),
                "n-" => Ok(parse_signless_b(input, a, -1)?),
                _ => match parse_n_dash_digits(unit) {
                    Ok(b) => Ok((a, b)),
                    Err(()) => {
                        let unit = unit.clone();
                        Err(input.new_basic_unexpected_token_error(Token::Ident(unit)))
                    }
                }
            }
        }
        Token::Ident(ref value) => {
            match_ignore_ascii_case! { value,
                "even" => Ok((2, 0)),
                "odd" => Ok((2, 1)),
                "n" => Ok(parse_b(input, 1)?),
                "-n" => Ok(parse_b(input, -1)?),
                "n-" => Ok(parse_signless_b(input, 1, -1)?),
                "-n-" => Ok(parse_signless_b(input, -1, -1)?),
                _ => {
                    let (slice, a) = if let Some(stripped) = value.strip_prefix('-') {
                        (stripped, -1)
                    } else {
                        (&**value, 1)
                    };
                    match parse_n_dash_digits(slice) {
                        Ok(b) => Ok((a, b)),
                        Err(()) => {
                            let value = value.clone();
                            Err(input.new_basic_unexpected_token_error(Token::Ident(value)))
                        }
                    }
                }
            }
        }
        Token::Delim('+') => match *input.next_including_whitespace()? {
            Token::Ident(ref value) => {
                match_ignore_ascii_case! { value,
                    "n" => parse_b(input, 1),
                    "n-" => parse_signless_b(input, 1, -1),
                    _ => match parse_n_dash_digits(value) {
                        Ok(b) => Ok((1, b)),
                        Err(()) => {
                            let value = value.clone();
                            Err(input.new_basic_unexpected_token_error(Token::Ident(value)))
                        }
                    }
                }
            }
            ref token => {
                let token = token.clone();
                Err(input.new_basic_unexpected_token_error(token))
            }
        },
        ref token => {
            let token = token.clone();
            Err(input.new_basic_unexpected_token_error(token))
        }
    }
}

fn parse_b<'i>(input: &mut Parser<'i, '_>, a: i32) -> Result<(i32, i32), BasicParseError<'i>> {
    let start = input.state();
    match input.next() {
        Ok(&Token::Delim('+')) => parse_signless_b(input, a, 1),
        Ok(&Token::Delim('-')) => parse_signless_b(input, a, -1),
        Ok(&Token::Number {
            has_sign: true,
            int_value: Some(b),
            ..
        }) => Ok((a, b)),
        _ => {
            input.reset(&start);
            Ok((a, 0))
        }
    }
}

fn parse_signless_b<'i>(
    input: &mut Parser<'i, '_>,
    a: i32,
    b_sign: i32,
) -> Result<(i32, i32), BasicParseError<'i>> {
    // FIXME: remove .clone() when lifetimes are non-lexical.
    match input.next()?.clone() {
        Token::Number {
            has_sign: false,
            int_value: Some(b),
            ..
        } => Ok((a, b_sign * b)),
        token => Err(input.new_basic_unexpected_token_error(token)),
    }
}

fn parse_n_dash_digits(string: &str) -> Result<i32, ()> {
    let bytes = string.as_bytes();
    if bytes.len() >= 3
        && bytes[..2].eq_ignore_ascii_case(b"n-")
        && bytes[2..].iter().all(|&c| c.is_ascii_digit())
    {
        Ok(parse_number_saturate(&string[1..]).unwrap()) // Include the minus sign
    } else {
        Err(())
    }
}

fn parse_number_saturate(string: &str) -> Result<i32, ()> {
    let mut input = ParserInput::new(string);
    let mut parser = Parser::new(&mut input);
    let int = if let Ok(&Token::Number {
        int_value: Some(int),
        ..
    }) = parser.next_including_whitespace_and_comments()
    {
        int
    } else {
        return Err(());
    };
    if !parser.is_exhausted() {
        return Err(());
    }
    Ok(int)
}
