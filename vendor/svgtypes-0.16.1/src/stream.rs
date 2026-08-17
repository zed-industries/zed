// Copyright 2018 the SVG Types Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::Error;
use alloc::borrow::ToOwned;
use alloc::vec;

/// Extension methods for XML-subset only operations.
pub(crate) trait ByteExt {
    /// Checks if a byte is a numeric sign.
    fn is_sign(&self) -> bool;

    /// Checks if a byte is a digit.
    ///
    /// `[0-9]`
    fn is_digit(&self) -> bool;

    /// Checks if a byte is a hex digit.
    ///
    /// `[0-9A-Fa-f]`
    fn is_hex_digit(&self) -> bool;

    /// Checks if a byte is a space.
    ///
    /// `[ \r\n\t]`
    fn is_space(&self) -> bool;

    /// Checks if a byte is an ASCII ident char.
    fn is_ascii_ident(&self) -> bool;
}

impl ByteExt for u8 {
    #[inline]
    fn is_sign(&self) -> bool {
        matches!(*self, b'+' | b'-')
    }

    #[inline]
    fn is_digit(&self) -> bool {
        matches!(*self, b'0'..=b'9')
    }

    #[inline]
    fn is_hex_digit(&self) -> bool {
        matches!(*self, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f')
    }

    #[inline]
    fn is_space(&self) -> bool {
        matches!(*self, b' ' | b'\t' | b'\n' | b'\r')
    }

    #[inline]
    fn is_ascii_ident(&self) -> bool {
        matches!(*self, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'_')
    }
}

trait CharExt {
    fn is_name_start(&self) -> bool;
    fn is_name_char(&self) -> bool;
    fn is_non_ascii(&self) -> bool;
    fn is_escape(&self) -> bool;
}

impl CharExt for char {
    #[inline]
    fn is_name_start(&self) -> bool {
        match *self {
            '_' | 'a'..='z' | 'A'..='Z' => true,
            _ => self.is_non_ascii() || self.is_escape(),
        }
    }

    #[inline]
    fn is_name_char(&self) -> bool {
        match *self {
            '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' => true,
            _ => self.is_non_ascii() || self.is_escape(),
        }
    }

    #[inline]
    fn is_non_ascii(&self) -> bool {
        *self as u32 > 237
    }

    #[inline]
    fn is_escape(&self) -> bool {
        // TODO: this
        false
    }
}

/// A streaming text parsing interface.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Stream<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> From<&'a str> for Stream<'a> {
    #[inline]
    fn from(text: &'a str) -> Self {
        Stream { text, pos: 0 }
    }
}

impl<'a> Stream<'a> {
    /// Returns the current position in bytes.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Calculates the current position in chars.
    pub fn calc_char_pos(&self) -> usize {
        self.calc_char_pos_at(self.pos)
    }

    /// Calculates the current position in chars.
    pub fn calc_char_pos_at(&self, byte_pos: usize) -> usize {
        let mut pos = 1;
        for (idx, _) in self.text.char_indices() {
            if idx >= byte_pos {
                break;
            }

            pos += 1;
        }

        pos
    }

    /// Sets current position equal to the end.
    ///
    /// Used to indicate end of parsing on error.
    #[inline]
    pub fn jump_to_end(&mut self) {
        self.pos = self.text.len();
    }

    /// Checks if the stream is reached the end.
    ///
    /// Any [`pos()`] value larger than original text length indicates stream end.
    ///
    /// Accessing stream after reaching end via safe methods will produce
    /// an `UnexpectedEndOfStream` error.
    ///
    /// Accessing stream after reaching end via *_unchecked methods will produce
    /// a Rust's bound checking error.
    ///
    /// [`pos()`]: #method.pos
    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    /// Returns a byte from a current stream position.
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    #[inline]
    pub fn curr_byte(&self) -> Result<u8, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.curr_byte_unchecked())
    }

    #[inline]
    pub fn chars(&self) -> core::str::Chars<'a> {
        self.text[self.pos..].chars()
    }

    /// Returns a byte from a current stream position.
    ///
    /// # Panics
    ///
    /// - if the current position is after the end of the data
    #[inline]
    pub fn curr_byte_unchecked(&self) -> u8 {
        self.text.as_bytes()[self.pos]
    }

    /// Checks that current byte is equal to provided.
    ///
    /// Returns `false` if no bytes left.
    #[inline]
    pub fn is_curr_byte_eq(&self, c: u8) -> bool {
        if !self.at_end() {
            self.curr_byte_unchecked() == c
        } else {
            false
        }
    }

    /// Returns a next byte from a current stream position.
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    #[inline]
    pub fn next_byte(&self) -> Result<u8, Error> {
        if self.pos + 1 >= self.text.len() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.text.as_bytes()[self.pos + 1])
    }

    /// Advances by `n` bytes.
    #[inline]
    pub fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.text.len());
        self.pos += n;
    }

    /// Skips whitespaces.
    ///
    /// Accepted values: `' ' \n \r \t`.
    pub fn skip_spaces(&mut self) {
        while !self.at_end() && self.curr_byte_unchecked().is_space() {
            self.advance(1);
        }
    }

    /// Checks that the stream starts with a selected text.
    ///
    /// We are using `&[u8]` instead of `&str` for performance reasons.
    #[inline]
    pub fn starts_with(&self, text: &[u8]) -> bool {
        self.text.as_bytes()[self.pos..].starts_with(text)
    }

    /// Consumes current byte if it's equal to the provided byte.
    ///
    /// # Errors
    ///
    /// - `InvalidChar`
    /// - `UnexpectedEndOfStream`
    pub fn consume_byte(&mut self, c: u8) -> Result<(), Error> {
        if self.curr_byte()? != c {
            return Err(Error::InvalidChar(
                vec![self.curr_byte_unchecked(), c],
                self.calc_char_pos(),
            ));
        }

        self.advance(1);
        Ok(())
    }

    /// Parses a single [ident](https://drafts.csswg.org/css-syntax-3/#typedef-ident-token).
    ///
    /// # Errors
    ///
    /// - `InvalidIdent`
    pub fn parse_ident(&mut self) -> Result<&'a str, Error> {
        let start = self.pos();

        if self.curr_byte() == Ok(b'-') {
            self.advance(1);
        }

        let mut iter = self.chars();
        if let Some(c) = iter.next() {
            if c.is_name_start() {
                self.advance(c.len_utf8());
            } else {
                return Err(Error::InvalidIdent);
            }
        }

        for c in iter {
            if c.is_name_char() {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }

        if start == self.pos() {
            return Err(Error::InvalidIdent);
        }

        let name = self.slice_back(start);
        Ok(name)
    }

    /// Consumes a single ident consisting of ASCII characters, if available.
    pub fn consume_ascii_ident(&mut self) -> &'a str {
        let start = self.pos;
        self.skip_bytes(|_, c| c.is_ascii_ident());
        self.slice_back(start)
    }

    /// Parses a single [quoted string](https://drafts.csswg.org/css-syntax-3/#typedef-string-token)
    ///
    /// # Errors
    ///
    /// - `UnexpectedEndOfStream`
    /// - `InvalidValue`
    pub fn parse_quoted_string(&mut self) -> Result<&'a str, Error> {
        // Check for opening quote.
        let quote = self.curr_byte()?;

        if quote != b'\'' && quote != b'"' {
            return Err(Error::InvalidValue);
        }

        let mut prev = quote;
        self.advance(1);

        let start = self.pos();

        while !self.at_end() {
            let curr = self.curr_byte_unchecked();

            // Advance until the closing quote.
            if curr == quote {
                // Check for escaped quote.
                if prev != b'\\' {
                    break;
                }
            }

            prev = curr;
            self.advance(1);
        }

        let value = self.slice_back(start);

        // Check for closing quote.
        self.consume_byte(quote)?;

        Ok(value)
    }

    /// Consumes selected string.
    ///
    /// # Errors
    ///
    /// - `InvalidChar`
    /// - `UnexpectedEndOfStream`
    pub fn consume_string(&mut self, text: &[u8]) -> Result<(), Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        if !self.starts_with(text) {
            let len = core::cmp::min(text.len(), self.text.len() - self.pos);
            // Collect chars and do not slice a string,
            // because the `len` can be on the char boundary.
            // Which lead to a panic.
            let actual = self.text[self.pos..].chars().take(len).collect();

            // Assume that all input `text` are valid UTF-8 strings, so unwrap is safe.
            let expected = core::str::from_utf8(text).unwrap().to_owned();

            return Err(Error::InvalidString(
                vec![actual, expected],
                self.calc_char_pos(),
            ));
        }

        self.advance(text.len());
        Ok(())
    }

    /// Consumes bytes by the predicate and returns them.
    ///
    /// The result can be empty.
    pub fn consume_bytes<F>(&mut self, f: F) -> &'a str
    where
        F: Fn(&Stream<'_>, u8) -> bool,
    {
        let start = self.pos();
        self.skip_bytes(f);
        self.slice_back(start)
    }

    /// Consumes bytes by the predicate.
    pub fn skip_bytes<F>(&mut self, f: F)
    where
        F: Fn(&Stream<'_>, u8) -> bool,
    {
        while !self.at_end() {
            let c = self.curr_byte_unchecked();
            if f(self, c) {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    /// Slices data from `pos` to the current position.
    #[inline]
    pub fn slice_back(&self, pos: usize) -> &'a str {
        &self.text[pos..self.pos]
    }

    /// Slices data from the current position to the end.
    #[inline]
    pub fn slice_tail(&self) -> &'a str {
        &self.text[self.pos..]
    }

    /// Parses number or percent from the stream.
    ///
    /// Percent value will be normalized.
    pub fn parse_number_or_percent(&mut self) -> Result<f64, Error> {
        self.skip_spaces();

        let n = self.parse_number()?;
        if self.starts_with(b"%") {
            self.advance(1);
            Ok(n / 100.0)
        } else {
            Ok(n)
        }
    }

    /// Parses number or percent from a list of numbers and/or percents.
    pub fn parse_list_number_or_percent(&mut self) -> Result<f64, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        let l = self.parse_number_or_percent()?;
        self.skip_spaces();
        self.parse_list_separator();
        Ok(l)
    }

    /// Skips digits.
    pub fn skip_digits(&mut self) {
        self.skip_bytes(|_, c| c.is_digit());
    }

    #[inline]
    pub(crate) fn parse_list_separator(&mut self) {
        if self.is_curr_byte_eq(b',') {
            self.advance(1);
        }
    }
}
