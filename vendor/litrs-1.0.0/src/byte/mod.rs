use core::fmt;

use crate::{
    err::{perr, ParseErrorKind::*},
    escape::unescape,
    parse::check_suffix,
    Buffer, ParseError,
};


/// A (single) byte literal, e.g. `b'k'` or `b'!'`.
///
/// See [the reference][ref] for more information.
///
/// [ref]: https://doc.rust-lang.org/reference/tokens.html#byte-literals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteLit<B: Buffer> {
    raw: B,
    /// Start index of the suffix or `raw.len()` if there is no suffix.
    start_suffix: usize,
    value: u8,
}

impl<B: Buffer> ByteLit<B> {
    /// Parses the input as a byte literal. Returns an error if the input is
    /// invalid or represents a different kind of literal.
    pub fn parse(input: B) -> Result<Self, ParseError> {
        if input.is_empty() {
            return Err(perr(None, Empty));
        }
        if !input.starts_with("b'") {
            return Err(perr(None, InvalidByteLiteralStart));
        }

        let (value, start_suffix) = parse_impl(&input)?;
        Ok(Self { raw: input, value, start_suffix })
    }

    /// Returns the byte value that this literal represents.
    pub fn value(&self) -> u8 {
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

impl ByteLit<&str> {
    /// Makes a copy of the underlying buffer and returns the owned version of
    /// `Self`.
    pub fn to_owned(&self) -> ByteLit<String> {
        ByteLit {
            raw: self.raw.to_owned(),
            start_suffix: self.start_suffix,
            value: self.value,
        }
    }
}

impl<B: Buffer> fmt::Display for ByteLit<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.raw)
    }
}

/// Precondition: must start with `b'`.
#[inline(never)]
pub(crate) fn parse_impl(input: &str) -> Result<(u8, usize), ParseError> {
    let input_bytes = input.as_bytes();
    let first = input_bytes
        .get(2)
        .ok_or(perr(None, UnterminatedByteLiteral))?;
    let (c, len) = match first {
        b'\'' if input_bytes.get(3) == Some(&b'\'') => return Err(perr(2, UnescapedSingleQuote)),
        b'\'' => return Err(perr(None, EmptyByteLiteral)),
        b'\n' | b'\t' | b'\r' => return Err(perr(2, UnescapedSpecialWhitespace)),
        b'\\' => {
            let (v, len) = unescape(&input[2..], false, true, true).map_err(|e| e.offset_span(2))?;
            (v.unwrap_byte(), len)
        }
        other if other.is_ascii() => (*other, 1),
        _ => return Err(perr(2, NonAsciiInByteLiteral)),
    };

    match input[2 + len..].find('\'') {
        Some(0) => {}
        Some(_) => return Err(perr(None, OverlongByteLiteral)),
        None => return Err(perr(None, UnterminatedByteLiteral)),
    }

    let start_suffix = 2 + len + 1;
    let suffix = &input[start_suffix..];
    check_suffix(suffix).map_err(|kind| perr(start_suffix, kind))?;

    Ok((c, start_suffix))
}

#[cfg(test)]
mod tests;
