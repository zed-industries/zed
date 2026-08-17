// Copyright 2016 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::str;

use crate::{Error, TextPos};

trait CssCharExt {
    fn is_name_start(&self) -> bool;
    fn is_name_char(&self) -> bool;
    fn is_non_ascii(&self) -> bool;
    fn is_escape(&self) -> bool;
}

impl CssCharExt for char {
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Stream<'a> {
    text: &'a str,
    pos: usize,
    end: usize,
}

impl<'a> From<&'a str> for Stream<'a> {
    fn from(text: &'a str) -> Self {
        Stream::new(text)
    }
}

impl<'a> Stream<'a> {
    pub fn new(text: &'a str) -> Self {
        Stream {
            text,
            pos: 0,
            end: text.len(),
        }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn jump_to_end(&mut self) {
        self.pos = self.end;
    }

    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos >= self.end
    }

    #[inline]
    pub fn curr_byte(&self) -> Result<u8, Error> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.curr_byte_unchecked())
    }

    #[inline]
    pub fn curr_byte_unchecked(&self) -> u8 {
        self.text.as_bytes()[self.pos]
    }

    #[inline]
    pub fn next_byte(&self) -> Result<u8, Error> {
        if self.pos + 1 >= self.end {
            return Err(Error::UnexpectedEndOfStream);
        }

        Ok(self.text.as_bytes()[self.pos + 1])
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.end);
        self.pos += n;
    }

    pub fn consume_byte(&mut self, c: u8) -> Result<(), Error> {
        if self.curr_byte()? != c {
            return Err(Error::InvalidByte {
                expected: c,
                actual: self.curr_byte()?,
                pos: self.gen_text_pos(),
            });
        }

        self.advance(1);
        Ok(())
    }

    pub fn try_consume_byte(&mut self, c: u8) {
        if self.curr_byte() == Ok(c) {
            self.advance(1);
        }
    }

    pub fn consume_bytes<F>(&mut self, f: F) -> &'a str
    where
        F: Fn(u8) -> bool,
    {
        let start = self.pos;
        self.skip_bytes(f);
        self.slice_back(start)
    }

    pub fn skip_bytes<F>(&mut self, f: F)
    where
        F: Fn(u8) -> bool,
    {
        while !self.at_end() && f(self.curr_byte_unchecked()) {
            self.advance(1);
        }
    }

    #[inline]
    fn chars(&self) -> str::Chars<'a> {
        self.text[self.pos..self.end].chars()
    }

    #[inline]
    pub fn slice_range(&self, start: usize, end: usize) -> &'a str {
        &self.text[start..end]
    }

    #[inline]
    pub fn slice_back(&self, pos: usize) -> &'a str {
        &self.text[pos..self.pos]
    }

    #[inline]
    pub fn slice_tail(&self) -> &'a str {
        &self.text[self.pos..]
    }

    #[inline]
    pub fn skip_spaces(&mut self) {
        while !self.at_end() {
            match self.curr_byte_unchecked() {
                b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' => self.advance(1),
                _ => break,
            }
        }
    }

    #[inline]
    pub fn skip_spaces_and_comments(&mut self) -> Result<(), Error> {
        self.skip_spaces();
        while self.curr_byte() == Ok(b'/') && self.next_byte() == Ok(b'*') {
            self.skip_comment()?;
            self.skip_spaces();
        }

        Ok(())
    }

    pub fn consume_ident(&mut self) -> Result<&'a str, Error> {
        let start = self.pos();

        if self.curr_byte() == Ok(b'-') {
            self.advance(1);
        }

        let mut iter = self.chars();
        if let Some(c) = iter.next() {
            if c.is_name_start() {
                self.advance(c.len_utf8());
            } else {
                return Err(Error::InvalidIdent(self.gen_text_pos_from(start)));
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
            return Err(Error::InvalidIdent(self.gen_text_pos_from(start)));
        }

        let name = self.slice_back(start);
        Ok(name)
    }

    pub fn consume_string(&mut self) -> Result<&'a str, Error> {
        // Check for opening quote.
        let quote = self.curr_byte()?;
        if quote == b'\'' || quote == b'"' {
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
        } else {
            self.consume_ident()
        }
    }

    pub fn skip_comment(&mut self) -> Result<(), Error> {
        let start = self.pos();
        self.skip_comment_impl()
            .map_err(|_| Error::InvalidComment(self.gen_text_pos_from(start)))?;
        Ok(())
    }

    fn skip_comment_impl(&mut self) -> Result<(), Error> {
        self.consume_byte(b'/')?;
        self.consume_byte(b'*')?;

        while !self.at_end() {
            let curr = self.curr_byte_unchecked();
            if curr == b'*' && self.next_byte() == Ok(b'/') {
                break;
            }

            self.advance(1);
        }

        self.consume_byte(b'*')?;
        self.consume_byte(b'/')?;
        Ok(())
    }

    #[inline(never)]
    pub fn gen_text_pos(&self) -> TextPos {
        let row = Self::calc_curr_row(self.text, self.pos);
        let col = Self::calc_curr_col(self.text, self.pos);
        TextPos::new(row, col)
    }

    #[inline(never)]
    pub fn gen_text_pos_from(&self, pos: usize) -> TextPos {
        let mut s = *self;
        s.pos = core::cmp::min(pos, self.text.len());
        s.gen_text_pos()
    }

    fn calc_curr_row(text: &str, end: usize) -> u32 {
        let mut row = 1;
        for c in &text.as_bytes()[..end] {
            if *c == b'\n' {
                row += 1;
            }
        }

        row
    }

    fn calc_curr_col(text: &str, end: usize) -> u32 {
        let mut col = 1;
        for c in text[..end].chars().rev() {
            if c == '\n' {
                break;
            } else {
                col += 1;
            }
        }

        col
    }
}
