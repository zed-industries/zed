use std::{
    ffi::{CStr, CString},
    fmt,
    ops::Range,
};

use crate::{
    err::{perr, ParseErrorKind::*},
    escape::{scan_raw_string, unescape_string},
    Buffer, ParseError,
};


/// A C string or raw C string literal, e.g. `c"hello"` or `cr#"abc"def"#`.
///
/// See [the reference][ref] for more information.
///
/// [ref]: https://doc.rust-lang.org/reference/tokens.html#c-string-and-raw-c-string-literals
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CStringLit<B: Buffer> {
    /// The raw input.
    raw: B,

    /// The string value (with all escaped unescaped) as CString. This is not an
    /// `Option` as we always have to add the trailing zero byte.
    value: CString,

    /// The number of hash signs in case of a raw string literal, or `None` if
    /// it's not a raw string literal.
    num_hashes: Option<u8>,

    /// Start index of the suffix or `raw.len()` if there is no suffix.
    start_suffix: usize,
}

impl<B: Buffer> CStringLit<B> {
    /// Parses the input as a (raw) byte string literal. Returns an error if the
    /// input is invalid or represents a different kind of literal.
    pub fn parse(input: B) -> Result<Self, ParseError> {
        if input.is_empty() {
            return Err(perr(None, Empty));
        }
        if !input.starts_with(r#"c""#) && !input.starts_with("cr") {
            return Err(perr(None, InvalidCStringLiteralStart));
        }

        let (value, num_hashes, start_suffix) = parse_impl(&input)?;
        Ok(Self { raw: input, value, num_hashes, start_suffix })
    }

    /// Returns the string value this literal represents (where all escapes have
    /// been turned into their respective values).
    pub fn value(&self) -> &CStr {
        &self.value
    }

    /// Like `value` but returns an owned version of the value.
    pub fn into_value(self) -> CString {
        self.value
    }

    /// The optional suffix. Returns `""` if the suffix is empty/does not exist.
    pub fn suffix(&self) -> &str {
        &(*self.raw)[self.start_suffix..]
    }

    /// Returns whether this literal is a raw string literal (starting with
    /// `cr`).
    pub fn is_raw_c_string(&self) -> bool {
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
}

/// The range within `self.raw` that excludes the quotes and potential `r#`.
fn inner_range(num_hashes: Option<u8>, start_suffix: usize) -> Range<usize> {
    match num_hashes {
        None => 2..start_suffix - 1,
        Some(n) => 2 + n as usize + 1..start_suffix - n as usize - 1,
    }
}

impl CStringLit<&str> {
    /// Makes a copy of the underlying buffer and returns the owned version of
    /// `Self`.
    pub fn into_owned(self) -> CStringLit<String> {
        CStringLit {
            raw: self.raw.to_owned(),
            value: self.value,
            num_hashes: self.num_hashes,
            start_suffix: self.start_suffix,
        }
    }
}

impl<B: Buffer> fmt::Display for CStringLit<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.raw)
    }
}


/// Precondition: input has to start with either `b"` or `br`.
#[inline(never)]
fn parse_impl(input: &str) -> Result<(CString, Option<u8>, usize), ParseError> {
    let (vec, num_hashes, start_suffix) = if input.starts_with("cr") {
        scan_raw_string(input, 2, true, false)
            .map(|(num, start_suffix)| (None, Some(num), start_suffix))?
    } else {
        unescape_string::<Vec<u8>>(input, 2, true, true, false)
            .map(|(v, start_suffix)| (v, None, start_suffix))?
    };


    let inner_range = inner_range(num_hashes, start_suffix);
    let vec = vec.unwrap_or_else(|| input[inner_range].as_bytes().to_vec());
    let value = CString::new(vec).unwrap(); // we already checked for nul bytes

    Ok((value, num_hashes, start_suffix))
}


#[cfg(test)]
mod tests;
