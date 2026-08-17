use std::{fmt, ops::Range};

use crate::{
    err::{perr, ParseErrorKind::*},
    escape::{scan_raw_string, unescape_string},
    Buffer, ParseError,
};


/// A byte string or raw byte string literal, e.g. `b"hello"` or `br#"abc"def"#`.
///
/// See [the reference][ref] for more information.
///
/// [ref]: https://doc.rust-lang.org/reference/tokens.html#byte-string-literals
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteStringLit<B: Buffer> {
    /// The raw input.
    raw: B,

    /// The string value (with all escaped unescaped), or `None` if there were
    /// no escapes. In the latter case, `input` is the string value.
    value: Option<Vec<u8>>,

    /// The number of hash signs in case of a raw string literal, or `None` if
    /// it's not a raw string literal.
    num_hashes: Option<u8>,

    /// Start index of the suffix or `raw.len()` if there is no suffix.
    start_suffix: usize,
}

impl<B: Buffer> ByteStringLit<B> {
    /// Parses the input as a (raw) byte string literal. Returns an error if the
    /// input is invalid or represents a different kind of literal.
    pub fn parse(input: B) -> Result<Self, ParseError> {
        if input.is_empty() {
            return Err(perr(None, Empty));
        }
        if !input.starts_with(r#"b""#) && !input.starts_with("br") {
            return Err(perr(None, InvalidByteStringLiteralStart));
        }

        let (value, num_hashes, start_suffix) = parse_impl(&input)?;
        Ok(Self { raw: input, value, num_hashes, start_suffix })
    }

    /// Returns the string value this literal represents (where all escapes have
    /// been turned into their respective values).
    pub fn value(&self) -> &[u8] {
        self.value
            .as_deref()
            .unwrap_or(&self.raw.as_bytes()[self.inner_range()])
    }

    /// Like `value` but returns a potentially owned version of the value.
    ///
    /// The return value is either `Vec<u8>` if `B = String`, or
    /// `Cow<'a, [u8]>` if `B = &'a str`.
    pub fn into_value(self) -> B::ByteCow {
        let inner_range = self.inner_range();
        let Self { raw, value, .. } = self;
        value
            .map(B::ByteCow::from)
            .unwrap_or_else(|| raw.cut(inner_range).into_byte_cow())
    }

    /// The optional suffix. Returns `""` if the suffix is empty/does not exist.
    pub fn suffix(&self) -> &str {
        &(*self.raw)[self.start_suffix..]
    }

    /// Returns whether this literal is a raw string literal (starting with
    /// `r`).
    pub fn is_raw_byte_string(&self) -> bool {
        self.num_hashes.is_some()
    }

    /// Returns the raw input that was passed to `parse`.
    pub fn raw_input(&self) -> &str {
        &self.raw
    }

    /// Returns the raw input that was passed to `parse`, potentially owned.
    pub fn into_raw_input(self) -> B {
        self.raw
    }

    /// The range within `self.raw` that excludes the quotes and potential `r#`.
    fn inner_range(&self) -> Range<usize> {
        match self.num_hashes {
            None => 2..self.start_suffix - 1,
            Some(n) => 2 + n as usize + 1..self.start_suffix - n as usize - 1,
        }
    }
}

impl ByteStringLit<&str> {
    /// Makes a copy of the underlying buffer and returns the owned version of
    /// `Self`.
    pub fn into_owned(self) -> ByteStringLit<String> {
        ByteStringLit {
            raw: self.raw.to_owned(),
            value: self.value,
            num_hashes: self.num_hashes,
            start_suffix: self.start_suffix,
        }
    }
}

impl<B: Buffer> fmt::Display for ByteStringLit<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.raw)
    }
}


/// Precondition: input has to start with either `b"` or `br`.
#[inline(never)]
fn parse_impl(input: &str) -> Result<(Option<Vec<u8>>, Option<u8>, usize), ParseError> {
    if input.starts_with("br") {
        scan_raw_string(input, 2, false, true)
            .map(|(num, start_suffix)| (None, Some(num), start_suffix))
    } else {
        unescape_string::<Vec<u8>>(input, 2, false, true, true)
            .map(|(v, start_suffix)| (v, None, start_suffix))
    }
}

#[cfg(test)]
mod tests;
