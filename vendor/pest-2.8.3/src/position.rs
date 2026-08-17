// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Range;
use core::ptr;
use core::str;

use crate::span;

/// A cursor position in a `&str` which provides useful methods to manually parse that string.
#[derive(Clone, Copy)]
pub struct Position<'i> {
    input: &'i str,
    pos: usize,
}

impl<'i> Position<'i> {
    /// Create a new `Position` without checking invariants. (Checked with `debug_assertions`.)
    pub(crate) fn new_internal(input: &str, pos: usize) -> Position<'_> {
        debug_assert!(input.get(pos..).is_some());
        Position { input, pos }
    }

    /// Attempts to create a new `Position` at the given position. If the specified position is
    /// an invalid index, or the specified position is not a valid UTF8 boundary, then None is
    /// returned.
    ///
    /// # Examples
    /// ```
    /// # use pest::Position;
    /// let cheart = '💖';
    /// let heart = "💖";
    /// assert_eq!(Position::new(heart, 1), None);
    /// assert_ne!(Position::new(heart, cheart.len_utf8()), None);
    /// ```
    pub fn new(input: &str, pos: usize) -> Option<Position<'_>> {
        input.get(pos..).map(|_| Position { input, pos })
    }

    /// Creates a `Position` at the start of a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let start = Position::from_start("");
    /// assert_eq!(start.pos(), 0);
    /// ```
    #[inline]
    pub fn from_start(input: &'i str) -> Position<'i> {
        // Position 0 is always safe because it's always a valid UTF-8 border.
        Position { input, pos: 0 }
    }

    /// Returns the byte position of this `Position` as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let mut start = Position::from_start(input);
    ///
    /// assert_eq!(start.pos(), 0);
    /// ```
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Creates a `Span` from two `Position`s.
    ///
    /// # Panics
    ///
    /// Panics if the positions come from different inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let span = start.span(&start.clone());
    ///
    /// assert_eq!(span.start(), 0);
    /// assert_eq!(span.end(), 0);
    /// ```
    #[inline]
    pub fn span(&self, other: &Position<'i>) -> span::Span<'i> {
        if ptr::eq(self.input, other.input)
        /* && self.input.get(self.pos..other.pos).is_some() */
        {
            span::Span::new_internal(self.input, self.pos, other.pos)
        } else {
            // TODO: maybe a panic if self.pos < other.pos
            panic!("span created from positions from different inputs")
        }
    }

    /// Returns the line and column number of this `Position`.
    ///
    /// This is an O(n) operation, where n is the number of chars in the input.
    /// You better use [`pair.line_col()`](struct.Pair.html#method.line_col) instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "\na";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.match_string("\na");
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().line_col(), (2, 2));
    /// ```
    #[inline]
    pub fn line_col(&self) -> (usize, usize) {
        if self.pos > self.input.len() {
            panic!("position out of bounds");
        }
        let mut pos = self.pos;
        let slice = &self.input[..pos];
        let mut chars = slice.chars().peekable();

        let mut line_col = (1, 1);

        while pos != 0 {
            match chars.next() {
                Some('\r') => {
                    if let Some(&'\n') = chars.peek() {
                        chars.next();

                        if pos == 1 {
                            pos -= 1;
                        } else {
                            pos -= 2;
                        }

                        line_col = (line_col.0 + 1, 1);
                    } else {
                        pos -= 1;
                        line_col = (line_col.0, line_col.1 + 1);
                    }
                }
                Some('\n') => {
                    pos -= 1;
                    line_col = (line_col.0 + 1, 1);
                }
                Some(c) => {
                    pos -= c.len_utf8();
                    line_col = (line_col.0, line_col.1 + 1);
                }
                None => unreachable!(),
            }
        }

        line_col
    }

    /// Returns the entire line of the input that contains this `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "\na";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.match_string("\na");
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().line_of(), "a");
    /// ```
    #[inline]
    pub fn line_of(&self) -> &'i str {
        if self.pos > self.input.len() {
            panic!("position out of bounds");
        };
        // Safe since start and end can only be valid UTF-8 borders.
        &self.input[self.find_line_start()..self.find_line_end()]
    }

    pub(crate) fn find_line_start(&self) -> usize {
        if self.input.is_empty() {
            return 0;
        };
        // Position's pos is always a UTF-8 border.
        let start = self
            .input
            .char_indices()
            .rev()
            .skip_while(|&(i, _)| i >= self.pos)
            .find(|&(_, c)| c == '\n');
        match start {
            Some((i, _)) => i + 1,
            None => 0,
        }
    }

    pub(crate) fn find_line_end(&self) -> usize {
        if self.input.is_empty() {
            0
        } else if self.pos == self.input.len() - 1 {
            self.input.len()
        } else {
            // Position's pos is always a UTF-8 border.
            let end = self
                .input
                .char_indices()
                .skip_while(|&(i, _)| i < self.pos)
                .find(|&(_, c)| c == '\n');
            match end {
                Some((i, _)) => i + 1,
                None => self.input.len(),
            }
        }
    }

    /// Returns `true` when the `Position` points to the start of the input `&str`.
    #[inline]
    pub(crate) fn at_start(&self) -> bool {
        self.pos == 0
    }

    /// Returns `true` when the `Position` points to the end of the input `&str`.
    #[inline]
    pub(crate) fn at_end(&self) -> bool {
        self.pos == self.input.len()
    }

    /// Skips `n` `char`s from the `Position` and returns `true` if the skip was possible or `false`
    /// otherwise. If the return value is `false`, `pos` will not be updated.
    #[inline]
    pub(crate) fn skip(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            // Position's pos is always a UTF-8 border.
            let mut chars = self.input[self.pos..].chars();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                } else {
                    return false;
                }
            }
            len
        };

        self.pos += skipped;
        true
    }

    /// Goes back `n` `char`s from the `Position` and returns `true` if the skip was possible or `false`
    /// otherwise. If the return value is `false`, `pos` will not be updated.
    #[inline]
    pub(crate) fn skip_back(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            // Position's pos is always a UTF-8 border.
            let mut chars = self.input[..self.pos].chars().rev();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                } else {
                    return false;
                }
            }
            len
        };

        self.pos -= skipped;
        true
    }

    /// Skips until one of the given `strings` is found. If none of the `strings` can be found,
    /// this function will return `false` but its `pos` will *still* be updated.
    #[inline]
    pub(crate) fn skip_until(&mut self, strings: &[&str]) -> bool {
        #[cfg(not(feature = "memchr"))]
        {
            self.skip_until_basic(strings)
        }
        #[cfg(feature = "memchr")]
        {
            match strings {
                [] => (),
                [s1] => {
                    if let Some(from) =
                        memchr::memmem::find(&self.input.as_bytes()[self.pos..], s1.as_bytes())
                    {
                        self.pos += from;
                        return true;
                    }
                }
                [s1, s2] if !s1.is_empty() && !s2.is_empty() => {
                    let b1 = s1.as_bytes()[0];
                    let b2 = s2.as_bytes()[0];
                    let miter = memchr::memchr2_iter(b1, b2, &self.input.as_bytes()[self.pos..]);
                    for from in miter {
                        let start = &self.input[self.pos + from..];
                        if start.starts_with(s1) || start.starts_with(s2) {
                            self.pos += from;
                            return true;
                        }
                    }
                }
                [s1, s2, s3] if !s1.is_empty() && !s2.is_empty() && s3.is_empty() => {
                    let b1 = s1.as_bytes()[0];
                    let b2 = s2.as_bytes()[0];
                    let b3 = s2.as_bytes()[0];
                    let miter =
                        memchr::memchr3_iter(b1, b2, b3, &self.input.as_bytes()[self.pos..]);
                    for from in miter {
                        let start = &self.input[self.pos + from..];
                        if start.starts_with(s1) || start.starts_with(s2) || start.starts_with(s3) {
                            self.pos += from;
                            return true;
                        }
                    }
                }
                _ => {
                    return self.skip_until_basic(strings);
                }
            }
            self.pos = self.input.len();
            false
        }
    }

    #[inline]
    fn skip_until_basic(&mut self, strings: &[&str]) -> bool {
        // TODO: optimize with Aho-Corasick, e.g. https://crates.io/crates/daachorse?
        for from in self.pos..self.input.len() {
            let bytes = if let Some(string) = self.input.get(from..) {
                string.as_bytes()
            } else {
                continue;
            };

            for slice in strings.iter() {
                let to = slice.len();
                if Some(slice.as_bytes()) == bytes.get(0..to) {
                    self.pos = from;
                    return true;
                }
            }
        }

        self.pos = self.input.len();
        false
    }

    /// Matches the char at the `Position` against a specified character and returns `true` if a match
    /// was made. If no match was made, returns `false`.
    /// `pos` will not be updated in either case.
    #[inline]
    pub(crate) fn match_char(&self, c: char) -> bool {
        matches!(self.input[self.pos..].chars().next(), Some(cc) if c == cc)
    }

    /// Matches the char at the `Position` against a filter function and returns `true` if a match
    /// was made. If no match was made, returns `false` and `pos` will not be updated.
    #[inline]
    pub(crate) fn match_char_by<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        if let Some(c) = self.input[self.pos..].chars().next() {
            if f(c) {
                self.pos += c.len_utf8();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Matches `string` from the `Position` and returns `true` if a match was made or `false`
    /// otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    pub(crate) fn match_string(&mut self, string: &str) -> bool {
        let to = self.pos + string.len();

        if Some(string.as_bytes()) == self.input.as_bytes().get(self.pos..to) {
            self.pos = to;
            true
        } else {
            false
        }
    }

    /// Case-insensitively matches `string` from the `Position` and returns `true` if a match was
    /// made or `false` otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    pub(crate) fn match_insensitive(&mut self, string: &str) -> bool {
        let matched = {
            let slice = &self.input[self.pos..];
            if let Some(slice) = slice.get(0..string.len()) {
                slice.eq_ignore_ascii_case(string)
            } else {
                false
            }
        };

        if matched {
            self.pos += string.len();
            true
        } else {
            false
        }
    }

    /// Matches `char` `range` from the `Position` and returns `true` if a match was made or `false`
    /// otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    pub(crate) fn match_range(&mut self, range: Range<char>) -> bool {
        if let Some(c) = self.input[self.pos..].chars().next() {
            if range.start <= c && c <= range.end {
                self.pos += c.len_utf8();
                return true;
            }
        }

        false
    }
}

impl<'i> fmt::Debug for Position<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position").field("pos", &self.pos).finish()
    }
}

impl<'i> PartialEq for Position<'i> {
    fn eq(&self, other: &Position<'i>) -> bool {
        ptr::eq(self.input, other.input) && self.pos == other.pos
    }
}

impl<'i> Eq for Position<'i> {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<'i> PartialOrd for Position<'i> {
    fn partial_cmp(&self, other: &Position<'i>) -> Option<Ordering> {
        if ptr::eq(self.input, other.input) {
            self.pos.partial_cmp(&other.pos)
        } else {
            None
        }
    }
}

impl<'i> Ord for Position<'i> {
    fn cmp(&self, other: &Position<'i>) -> Ordering {
        self.partial_cmp(other)
            .expect("cannot compare positions from different strs")
    }
}

impl<'i> Hash for Position<'i> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.input as *const str).hash(state);
        self.pos.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let input = "";
        assert!(Position::new(input, 0).unwrap().match_string(""));
        assert!(!Position::new(input, 0).unwrap().match_string("a"));
    }

    #[test]
    fn parts() {
        let input = "asdasdf";

        assert!(Position::new(input, 0).unwrap().match_string("asd"));
        assert!(Position::new(input, 3).unwrap().match_string("asdf"));
    }

    #[test]
    fn line_col() {
        let input = "a\rb\nc\r\nd嗨";

        assert_eq!(Position::new(input, 0).unwrap().line_col(), (1, 1));
        assert_eq!(Position::new(input, 1).unwrap().line_col(), (1, 2));
        assert_eq!(Position::new(input, 2).unwrap().line_col(), (1, 3));
        assert_eq!(Position::new(input, 3).unwrap().line_col(), (1, 4));
        assert_eq!(Position::new(input, 4).unwrap().line_col(), (2, 1));
        assert_eq!(Position::new(input, 5).unwrap().line_col(), (2, 2));
        assert_eq!(Position::new(input, 6).unwrap().line_col(), (2, 3));
        assert_eq!(Position::new(input, 7).unwrap().line_col(), (3, 1));
        assert_eq!(Position::new(input, 8).unwrap().line_col(), (3, 2));
        assert_eq!(Position::new(input, 11).unwrap().line_col(), (3, 3));
        let input = "abcd嗨";
        assert_eq!(Position::new(input, 7).unwrap().line_col(), (1, 6));
    }

    #[test]
    fn line_of() {
        let input = "a\rb\nc\r\nd嗨";

        assert_eq!(Position::new(input, 0).unwrap().line_of(), "a\rb\n");
        assert_eq!(Position::new(input, 1).unwrap().line_of(), "a\rb\n");
        assert_eq!(Position::new(input, 2).unwrap().line_of(), "a\rb\n");
        assert_eq!(Position::new(input, 3).unwrap().line_of(), "a\rb\n");
        assert_eq!(Position::new(input, 4).unwrap().line_of(), "c\r\n");
        assert_eq!(Position::new(input, 5).unwrap().line_of(), "c\r\n");
        assert_eq!(Position::new(input, 6).unwrap().line_of(), "c\r\n");
        assert_eq!(Position::new(input, 7).unwrap().line_of(), "d嗨");
        assert_eq!(Position::new(input, 8).unwrap().line_of(), "d嗨");
        assert_eq!(Position::new(input, 11).unwrap().line_of(), "d嗨");
    }

    #[test]
    fn line_of_empty() {
        let input = "";

        assert_eq!(Position::new(input, 0).unwrap().line_of(), "");
    }

    #[test]
    fn line_of_new_line() {
        let input = "\n";

        assert_eq!(Position::new(input, 0).unwrap().line_of(), "\n");
    }

    #[test]
    fn line_of_between_new_line() {
        let input = "\n\n";

        assert_eq!(Position::new(input, 1).unwrap().line_of(), "\n");
    }

    fn measure_skip(input: &str, pos: usize, n: usize) -> Option<usize> {
        let mut p = Position::new(input, pos).unwrap();
        if p.skip(n) {
            Some(p.pos - pos)
        } else {
            None
        }
    }

    #[test]
    fn skip_empty() {
        let input = "";

        assert_eq!(measure_skip(input, 0, 0), Some(0));
        assert_eq!(measure_skip(input, 0, 1), None);
    }

    #[test]
    fn skip() {
        let input = "d嗨";

        assert_eq!(measure_skip(input, 0, 0), Some(0));
        assert_eq!(measure_skip(input, 0, 1), Some(1));
        assert_eq!(measure_skip(input, 1, 1), Some(3));
    }

    #[test]
    fn skip_until() {
        let input = "ab ac";
        let pos = Position::from_start(input);

        let mut test_pos = pos;
        test_pos.skip_until(&["a", "b"]);
        assert_eq!(test_pos.pos(), 0);

        test_pos = pos;
        test_pos.skip_until(&["b"]);
        assert_eq!(test_pos.pos(), 1);

        test_pos = pos;
        test_pos.skip_until(&["ab"]);
        assert_eq!(test_pos.pos(), 0);

        test_pos = pos;
        test_pos.skip_until(&["ac", "z"]);
        assert_eq!(test_pos.pos(), 3);

        test_pos = pos;
        assert!(!test_pos.skip_until(&["z"]));
        assert_eq!(test_pos.pos(), 5);
    }

    #[test]
    fn match_range() {
        let input = "b";

        assert!(Position::new(input, 0).unwrap().match_range('a'..'c'));
        assert!(Position::new(input, 0).unwrap().match_range('b'..'b'));
        assert!(!Position::new(input, 0).unwrap().match_range('a'..'a'));
        assert!(!Position::new(input, 0).unwrap().match_range('c'..'c'));
        assert!(Position::new(input, 0).unwrap().match_range('a'..'嗨'));
    }

    #[test]
    fn match_insensitive() {
        let input = "AsdASdF";

        assert!(Position::new(input, 0).unwrap().match_insensitive("asd"));
        assert!(Position::new(input, 3).unwrap().match_insensitive("asdf"));
    }

    #[test]
    fn cmp() {
        let input = "a";
        let start = Position::from_start(input);
        let mut end = start;

        assert!(end.skip(1));
        let result = start.cmp(&end);

        assert_eq!(result, Ordering::Less);
    }

    #[test]
    #[should_panic]
    fn cmp_panic() {
        let input1 = "a";
        let input2 = "b";
        let pos1 = Position::from_start(input1);
        let pos2 = Position::from_start(input2);

        let _ = pos1.cmp(&pos2);
    }

    #[test]
    #[cfg(feature = "std")]
    fn hash() {
        use std::collections::HashSet;

        let input = "a";
        let start = Position::from_start(input);
        let mut positions = HashSet::new();

        positions.insert(start);
    }
}
