use std::str::from_utf8_unchecked;

use memchr::memchr;

use crate::TokenSource;

/// Returns a [`TokenSource`] that uses the lines in `data` as Tokens. The newline
/// separator (`\r\n` or `\n`) is included in the emitted tokens. This means that changing
/// the newline separator from `\r\n` to `\n` (or omitting it fully on the last line) is
/// detected by [`Diff`](crate::Diff).
pub fn lines(data: &str) -> Lines<'_> {
    Lines(ByteLines(data.as_bytes()))
}

/// Returns a [`TokenSource`] that uses the lines in `data` as Tokens. The newline
/// separator (`\r\n` or `\n`) is included in the emitted tokens. This means that changing
/// the newline separator from `\r\n` to `\n` (or omitting it fully on the last line) is
/// detected when computing a [`Diff`](crate::Diff).
pub fn byte_lines(data: &[u8]) -> ByteLines<'_> {
    ByteLines(data)
}

/// By default, a line diff is produced for a string
impl<'a> TokenSource for &'a str {
    type Token = &'a str;

    type Tokenizer = Lines<'a>;

    fn tokenize(&self) -> Self::Tokenizer {
        lines(self)
    }

    fn estimate_tokens(&self) -> u32 {
        lines(self).estimate_tokens()
    }
}

/// By default, a line diff is produced for a bytes
impl<'a> TokenSource for &'a [u8] {
    type Token = Self;
    type Tokenizer = ByteLines<'a>;

    fn tokenize(&self) -> Self::Tokenizer {
        byte_lines(self)
    }

    fn estimate_tokens(&self) -> u32 {
        byte_lines(self).estimate_tokens()
    }
}

/// A [`TokenSource`] that returns the lines of a `str` as tokens. See [`lines`] for
/// details.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lines<'a>(ByteLines<'a>);

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // safety invariant: this struct may only contain valid utf8
        // dividing valid utf8 bytes by ascii characters always produces valid utf-8
        self.0.next().map(|it| unsafe { from_utf8_unchecked(it) })
    }
}

/// By default a line diff is produced for a string
impl<'a> TokenSource for Lines<'a> {
    type Token = &'a str;

    type Tokenizer = Self;

    fn tokenize(&self) -> Self::Tokenizer {
        *self
    }

    fn estimate_tokens(&self) -> u32 {
        self.0.estimate_tokens()
    }
}

/// A [`TokenSource`] that returns the lines of a byte slice as tokens. See [`byte_lines`]
/// for details.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ByteLines<'a>(&'a [u8]);

impl<'a> Iterator for ByteLines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        let line_len = memchr(b'\n', self.0).map_or(self.0.len(), |len| len + 1);
        let (line, rem) = self.0.split_at(line_len);
        self.0 = rem;
        Some(line)
    }
}

/// By default a line diff is produced for a string
impl<'a> TokenSource for ByteLines<'a> {
    type Token = &'a [u8];

    type Tokenizer = Self;

    fn tokenize(&self) -> Self::Tokenizer {
        *self
    }

    fn estimate_tokens(&self) -> u32 {
        let len: usize = self.take(20).map(|line| line.len()).sum();
        if len == 0 {
            100
        } else {
            (self.0.len() * 20 / len) as u32
        }
    }
}
