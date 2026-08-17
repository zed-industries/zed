use std::fmt;

use crate::{
    err::{perr, ParseErrorKind::*},
    escape::unescape,
    parse::{check_suffix, first_byte_or_empty},
    Buffer, ParseError,
};


/// A character literal, e.g. `'g'` or `'🦊'`.
///
/// See [the reference][ref] for more information.
///
/// [ref]: https://doc.rust-lang.org/reference/tokens.html#character-literals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharLit<B: Buffer> {
    raw: B,
    /// Start index of the suffix or `raw.len()` if there is no suffix.
    start_suffix: usize,
    value: char,
}

impl<B: Buffer> CharLit<B> {
    /// Parses the input as a character literal. Returns an error if the input
    /// is invalid or represents a different kind of literal.
    pub fn parse(input: B) -> Result<Self, ParseError> {
        match first_byte_or_empty(&input)? {
            b'\'' => {
                let (value, start_suffix) = parse_impl(&input)?;
                Ok(Self { raw: input, value, start_suffix })
            }
            _ => Err(perr(0, DoesNotStartWithQuote)),
        }
    }

    /// Returns the character value that this literal represents.
    pub fn value(&self) -> char {
        self.value
    }

    /// The optional suffix. Returns `""` if the suffix is empty/does not exist.
    pub fn suffix(&self) -> &str {
        &(*self.raw)[self.start_suffix..]
    }

    /// Returns the raw input that was passed to `parse`.
    pub fn raw_input(&self) -> &str {
        &self.raw
    }

    /// Returns the raw input that was passed to `parse`, potentially owned.
    pub fn into_raw_input(self) -> B {
        self.raw
    }
}

impl CharLit<&str> {
    /// Makes a copy of the underlying buffer and returns the owned version of
    /// `Self`.
    pub fn to_owned(&self) -> CharLit<String> {
        CharLit {
            raw: self.raw.to_owned(),
            start_suffix: self.start_suffix,
            value: self.value,
        }
    }
}

impl<B: Buffer> fmt::Display for CharLit<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.raw)
    }
}

/// Precondition: first character in input must be `'`.
#[inline(never)]
pub(crate) fn parse_impl(input: &str) -> Result<(char, usize), ParseError> {
    let first = input.chars().nth(1).ok_or(perr(None, UnterminatedCharLiteral))?;
    let (c, len) = match first {
        '\'' if input.chars().nth(2) == Some('\'') => return Err(perr(1, UnescapedSingleQuote)),
        '\'' => return Err(perr(None, EmptyCharLiteral)),
        '\n' | '\t' | '\r' => return Err(perr(1, UnescapedSpecialWhitespace)),

        '\\' => {
            let (v, len) = unescape(&input[1..], true, false, true).map_err(|e| e.offset_span(1))?;
            (v.unwrap_char(), len)
        }
        other => (other, other.len_utf8()),
    };

    match input[1 + len..].find('\'') {
        Some(0) => {}
        Some(_) => return Err(perr(None, OverlongCharLiteral)),
        None => return Err(perr(None, UnterminatedCharLiteral)),
    }

    let start_suffix = 1 + len + 1;
    let suffix = &input[start_suffix..];
    check_suffix(suffix).map_err(|kind| perr(start_suffix, kind))?;

    Ok((c, start_suffix))
}

#[cfg(test)]
mod tests;
