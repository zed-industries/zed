use std::{fmt, ops::Range};

use crate::{
    err::{perr, ParseErrorKind::*},
    escape::{scan_raw_string, unescape_string},
    parse::first_byte_or_empty,
    Buffer, ParseError,
};


/// A string or raw string literal, e.g. `"foo"`, `"Grüße"` or `r#"a🦊c"d🦀f"#`.
///
/// See [the reference][ref] for more information.
///
/// [ref]: https://doc.rust-lang.org/reference/tokens.html#string-literals
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLit<B: Buffer> {
    /// The raw input.
    raw: B,

    /// The string value (with all escapes unescaped), or `None` if there were
    /// no escapes. In the latter case, the string value is in `raw`.
    value: Option<String>,

    /// The number of hash signs in case of a raw string literal, or `None` if
    /// it's not a raw string literal.
    num_hashes: Option<u8>,

    /// Start index of the suffix or `raw.len()` if there is no suffix.
    start_suffix: usize,
}

impl<B: Buffer> StringLit<B> {
    /// Parses the input as a (raw) string literal. Returns an error if the
    /// input is invalid or represents a different kind of literal.
    pub fn parse(input: B) -> Result<Self, ParseError> {
        match first_byte_or_empty(&input)? {
            b'r' | b'"' => {
                let (value, num_hashes, start_suffix) = parse_impl(&input)?;
                Ok(Self { raw: input, value, num_hashes, start_suffix })
            }
            _ => Err(perr(0, InvalidStringLiteralStart)),
        }
    }

    /// Returns the string value this literal represents (where all escapes have
    /// been turned into their respective values).
    pub fn value(&self) -> &str {
        self.value
            .as_deref()
            .unwrap_or(&self.raw[self.inner_range()])
    }

    /// Like `value` but returns a potentially owned version of the value.
    ///
    /// The return value is either `String` if `B = String`, or
    /// `Cow<'a, str>` if `B = &'a str`.
    pub fn into_value(self) -> B::Cow {
        let inner_range = self.inner_range();
        let Self { raw, value, .. } = self;
        value
            .map(B::Cow::from)
            .unwrap_or_else(|| raw.cut(inner_range).into_cow())
    }

    /// The optional suffix. Returns `""` if the suffix is empty/does not exist.
    pub fn suffix(&self) -> &str {
        &(*self.raw)[self.start_suffix..]
    }

    /// Returns whether this literal is a raw string literal (starting with
    /// `r`).
    pub fn is_raw_string(&self) -> bool {
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
            None => 1..self.start_suffix - 1,
            Some(n) => 1 + n as usize + 1..self.start_suffix - n as usize - 1,
        }
    }
}

impl StringLit<&str> {
    /// Makes a copy of the underlying buffer and returns the owned version of
    /// `Self`.
    pub fn into_owned(self) -> StringLit<String> {
        StringLit {
            raw: self.raw.to_owned(),
            value: self.value,
            num_hashes: self.num_hashes,
            start_suffix: self.start_suffix,
        }
    }
}

impl<B: Buffer> fmt::Display for StringLit<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.raw)
    }
}

/// Precondition: input has to start with either `"` or `r`.
#[inline(never)]
pub(crate) fn parse_impl(input: &str) -> Result<(Option<String>, Option<u8>, usize), ParseError> {
    if input.starts_with('r') {
        scan_raw_string(input, 1, true, true)
            .map(|(hashes, start_suffix)| (None, Some(hashes), start_suffix))
    } else {
        unescape_string::<String>(input, 1, true, false, true)
            .map(|(v, start_suffix)| (v, None, start_suffix))
    }
}


#[cfg(test)]
mod tests;
