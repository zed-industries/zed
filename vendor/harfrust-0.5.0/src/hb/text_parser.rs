use super::{common::TagExt, hb_tag_t};

pub struct TextParser<'a> {
    pos: usize,
    text: &'a str,
}

impl<'a> TextParser<'a> {
    #[inline]
    pub fn new(text: &'a str) -> Self {
        TextParser { pos: 0, text }
    }

    #[inline]
    pub fn at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    #[inline]
    pub fn curr_byte(&self) -> Option<u8> {
        if !self.at_end() {
            Some(self.curr_byte_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn curr_byte_unchecked(&self) -> u8 {
        self.text.as_bytes()[self.pos]
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.text.len());
        self.pos += n;
    }

    pub fn consume_byte(&mut self, c: u8) -> Option<()> {
        let curr = self.curr_byte()?;
        if curr != c {
            return None;
        }

        self.advance(1);
        Some(())
    }

    #[inline]
    pub fn skip_spaces(&mut self) {
        // Unlike harfbuzz::ISSPACE, is_ascii_whitespace doesn't includes `\v`, but whatever.
        while !self.at_end() && self.curr_byte_unchecked().is_ascii_whitespace() {
            self.advance(1);
        }
    }

    pub fn consume_quote(&mut self) -> Option<u8> {
        let c = self.curr_byte()?;
        if matches!(c, b'\'' | b'"') {
            self.advance(1);
            Some(c)
        } else {
            None
        }
    }

    #[inline]
    pub fn consume_bytes<F>(&mut self, f: F) -> &'a str
    where
        F: Fn(u8) -> bool,
    {
        let start = self.pos;
        self.skip_bytes(f);
        &self.text[start..self.pos]
    }

    pub fn skip_bytes<F>(&mut self, f: F)
    where
        F: Fn(u8) -> bool,
    {
        while !self.at_end() && f(self.curr_byte_unchecked()) {
            self.advance(1);
        }
    }

    pub fn consume_tag(&mut self) -> Option<hb_tag_t> {
        let tag = self.consume_bytes(|c| c.is_ascii_alphanumeric() || c == b'_');
        if tag.len() > 4 {
            return None;
        }

        Some(hb_tag_t::from_bytes_lossy(tag.as_bytes()))
    }

    pub fn consume_i32(&mut self) -> Option<i32> {
        let start = self.pos;

        if matches!(self.curr_byte(), Some(b'-') | Some(b'+')) {
            self.advance(1);
        }

        self.skip_bytes(|c| c.is_ascii_digit());
        self.text[start..self.pos].parse::<i32>().ok()
    }

    pub fn consume_f32(&mut self) -> Option<f32> {
        let start = self.pos;

        // TODO: does number like 1-e2 required?

        if matches!(self.curr_byte(), Some(b'-') | Some(b'+')) {
            self.advance(1);
        }

        self.skip_bytes(|c| c.is_ascii_digit());

        if self.consume_byte(b'.').is_some() {
            self.skip_bytes(|c| c.is_ascii_digit());
        }

        self.text[start..self.pos].parse::<f32>().ok()
    }

    pub fn consume_bool(&mut self) -> Option<bool> {
        self.skip_spaces();

        let value = self.consume_bytes(|c| c.is_ascii_alphabetic()).as_bytes();
        if value.len() == 2 {
            if value[0].eq_ignore_ascii_case(&b'o') && value[1].eq_ignore_ascii_case(&b'n') {
                return Some(true);
            }
        } else if value.len() == 3 {
            if value[0].eq_ignore_ascii_case(&b'o')
                && value[1].eq_ignore_ascii_case(&b'f')
                && value[2].eq_ignore_ascii_case(&b'f')
            {
                return Some(false);
            }
        }

        None
    }
}
