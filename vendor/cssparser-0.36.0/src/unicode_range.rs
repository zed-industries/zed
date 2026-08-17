/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! https://drafts.csswg.org/css-syntax/#urange

use crate::tokenizer::Token;
use crate::{BasicParseError, Parser, ToCss};
use std::char;
use std::fmt;

/// One contiguous range of code points.
///
/// Can not be empty. Can represent a single code point when start == end.
#[derive(PartialEq, Eq, Clone, Hash)]
#[repr(C)]
pub struct UnicodeRange {
    /// Inclusive start of the range. In [0, end].
    pub start: u32,

    /// Inclusive end of the range. In [0, 0x10FFFF].
    pub end: u32,
}

impl UnicodeRange {
    /// https://drafts.csswg.org/css-syntax/#urange-syntax
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, BasicParseError<'i>> {
        // <urange> =
        //   u '+' <ident-token> '?'* |
        //   u <dimension-token> '?'* |
        //   u <number-token> '?'* |
        //   u <number-token> <dimension-token> |
        //   u <number-token> <number-token> |
        //   u '+' '?'+

        input.expect_ident_matching("u")?;
        let after_u = input.position();
        parse_tokens(input)?;

        // This deviates from the spec in case there are CSS comments
        // between tokens in the middle of one <unicode-range>,
        // but oh well…
        let concatenated_tokens = input.slice_from(after_u);

        let range = match parse_concatenated(concatenated_tokens.as_bytes()) {
            Ok(range) => range,
            Err(()) => {
                return Err(input
                    .new_basic_unexpected_token_error(Token::Ident(concatenated_tokens.into())))
            }
        };
        if range.end > char::MAX as u32 || range.start > range.end {
            Err(input.new_basic_unexpected_token_error(Token::Ident(concatenated_tokens.into())))
        } else {
            Ok(range)
        }
    }
}

fn parse_tokens<'i>(input: &mut Parser<'i, '_>) -> Result<(), BasicParseError<'i>> {
    match input.next_including_whitespace()?.clone() {
        Token::Delim('+') => {
            match *input.next_including_whitespace()? {
                Token::Ident(_) => {}
                Token::Delim('?') => {}
                ref t => {
                    let t = t.clone();
                    return Err(input.new_basic_unexpected_token_error(t));
                }
            }
            parse_question_marks(input)
        }
        Token::Dimension { .. } => parse_question_marks(input),
        Token::Number { .. } => {
            let after_number = input.state();
            match input.next_including_whitespace() {
                Ok(&Token::Delim('?')) => parse_question_marks(input),
                Ok(&Token::Dimension { .. }) => {}
                Ok(&Token::Number { .. }) => {}
                _ => input.reset(&after_number),
            }
        }
        t => return Err(input.new_basic_unexpected_token_error(t)),
    }
    Ok(())
}

/// Consume as many '?' as possible
fn parse_question_marks(input: &mut Parser) {
    loop {
        let start = input.state();
        match input.next_including_whitespace() {
            Ok(&Token::Delim('?')) => {}
            _ => {
                input.reset(&start);
                return;
            }
        }
    }
}

fn parse_concatenated(text: &[u8]) -> Result<UnicodeRange, ()> {
    let mut text = match text.split_first() {
        Some((&b'+', text)) => text,
        _ => return Err(()),
    };
    let (first_hex_value, hex_digit_count) = consume_hex(&mut text, 6)?;
    let question_marks = consume_question_marks(&mut text);
    let consumed = hex_digit_count + question_marks;
    if consumed == 0 || consumed > 6 {
        return Err(());
    }

    if question_marks > 0 {
        if text.is_empty() {
            return Ok(UnicodeRange {
                start: first_hex_value << (question_marks * 4),
                end: ((first_hex_value + 1) << (question_marks * 4)) - 1,
            });
        }
    } else if text.is_empty() {
        return Ok(UnicodeRange {
            start: first_hex_value,
            end: first_hex_value,
        });
    } else if let Some((&b'-', mut text)) = text.split_first() {
        let (second_hex_value, hex_digit_count) = consume_hex(&mut text, 6)?;
        if hex_digit_count > 0 && hex_digit_count <= 6 && text.is_empty() {
            return Ok(UnicodeRange {
                start: first_hex_value,
                end: second_hex_value,
            });
        }
    }
    Err(())
}

// Consume hex digits, but return an error if more than digit_limit are found.
fn consume_hex(text: &mut &[u8], digit_limit: usize) -> Result<(u32, usize), ()> {
    let mut value = 0;
    let mut digits = 0;
    while let Some((&byte, rest)) = text.split_first() {
        if let Some(digit_value) = (byte as char).to_digit(16) {
            if digits == digit_limit {
                return Err(());
            }
            value = value * 0x10 + digit_value;
            digits += 1;
            *text = rest;
        } else {
            break;
        }
    }
    Ok((value, digits))
}

fn consume_question_marks(text: &mut &[u8]) -> usize {
    let mut question_marks = 0;
    while let Some((&b'?', rest)) = text.split_first() {
        question_marks += 1;
        *text = rest
    }
    question_marks
}

impl fmt::Debug for UnicodeRange {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.to_css(formatter)
    }
}

impl ToCss for UnicodeRange {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        write!(dest, "U+{:X}", self.start)?;
        if self.end != self.start {
            write!(dest, "-{:X}", self.end)?;
        }
        Ok(())
    }
}
