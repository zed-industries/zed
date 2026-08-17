//! Contains a parser for an XML comment.

use crate::errors::SyntaxError;
use crate::parser::Parser;

/// A parser that search a `-->` sequence in the slice.
///
/// To use a parser create an instance of parser and [`feed`] data into it.
/// After successful search the parser will return [`Some`] with position where
/// comment is ended (the position after `-->`). If search was unsuccessful,
/// a [`None`] will be returned. You typically would expect positive result of
/// search, so that you should feed new data until yo'll get it.
///
/// NOTE: after successful match the parser does not returned to the initial
/// state and should not be used anymore. Create a new parser if you want to perform
/// new search.
///
/// # Example
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use quick_xml::parser::{CommentParser, Parser};
///
/// let mut parser = CommentParser::default();
///
/// // Parse `<!-- comment with some -> and --- inside-->and the text follow...`
/// // splitted into three chunks
/// assert_eq!(parser.feed(b"<!-- comment"), None);
/// // ...get new chunk of data
/// assert_eq!(parser.feed(b" with some -> and -"), None);
/// // ...get another chunk of data
/// assert_eq!(parser.feed(b"-- inside-->and the text follow..."), Some(12));
/// //                       ^          ^
/// //                       0          11
/// ```
///
/// [`feed`]: Self::feed()
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommentParser {
    /// The parser does not yet seen any dashes at the end of previous slice.
    Seen0,
    /// The parser already seen one dash on the end of previous slice.
    Seen1,
    /// The parser already seen two dashes on the end of previous slice.
    Seen2,
}

impl Default for CommentParser {
    #[inline]
    fn default() -> Self {
        Self::Seen0
    }
}

impl Parser for CommentParser {
    /// Determines the end position of an XML comment in the provided slice.
    /// Comments is a pieces of text enclosed in `<!--` and `-->` braces.
    /// Comment ends on the first occurrence of `-->` which cannot be escaped.
    ///
    /// Returns position after the `-->` or `None` if such sequence was not found.
    ///
    /// # Parameters
    /// - `bytes`: a slice to find the end of a comment.
    ///   Should contain text in ASCII-compatible encoding
    #[inline]
    fn feed(&mut self, bytes: &[u8]) -> Option<usize> {
        let result = match self {
            Self::Seen0 => seen0(bytes),
            Self::Seen1 => seen1(bytes),
            Self::Seen2 => seen2(bytes),
        };
        if let Some(r) = result {
            return Some(r);
        }
        if bytes.ends_with(b"--") {
            *self = Self::Seen2;
        } else {
            self.next_state(bytes.last().copied());
        }
        None
    }

    #[inline]
    fn eof_error(self, _content: &[u8]) -> SyntaxError {
        SyntaxError::UnclosedComment
    }
}

impl CommentParser {
    #[inline]
    fn next_state(&mut self, last: Option<u8>) {
        match (*self, last) {
            (Self::Seen0, Some(b'-')) => *self = Self::Seen1,

            (Self::Seen1, Some(b'-')) => *self = Self::Seen2,
            (Self::Seen1, Some(_)) => *self = Self::Seen0,

            (Self::Seen2, Some(b'-')) => {}
            (Self::Seen2, Some(_)) => *self = Self::Seen0,

            _ => {}
        }
    }
}

#[inline]
fn seen0(bytes: &[u8]) -> Option<usize> {
    for i in memchr::memchr_iter(b'>', bytes) {
        if bytes[..i].ends_with(b"--") {
            // +1 for `>` which should be included in event
            return Some(i + 1);
        }
    }
    None
}

#[inline]
fn seen1(bytes: &[u8]) -> Option<usize> {
    // -|->
    if bytes.starts_with(b"->") {
        return Some(2);
    }
    // Even if the first character is `-` it cannot be part of close sequence,
    // because we checked that condition above. That means that we can forgot that
    // we seen one `-` at the end of the previous chunk.
    // -|x...
    seen0(bytes)
}

#[inline]
fn seen2(bytes: &[u8]) -> Option<usize> {
    match bytes.get(0) {
        // --|
        None => None,
        // --|>
        Some(b'>') => Some(1),
        // The end sequence here can be matched only if bytes starts with `->`
        // which is handled in seen1().
        // --|x...
        Some(_) => seen1(bytes),
    }
}

#[test]
fn parse() {
    use pretty_assertions::assert_eq;
    use CommentParser::*;

    /// Returns `Ok(pos)` with the position in the buffer where element is ended.
    ///
    /// Returns `Err(internal_state)` if parsing was not done yet.
    fn parse_comment(bytes: &[u8], mut parser: CommentParser) -> Result<usize, CommentParser> {
        match parser.feed(bytes) {
            Some(i) => Ok(i),
            None => Err(parser),
        }
    }

    assert_eq!(parse_comment(b"", Seen0), Err(Seen0)); // xx|
    assert_eq!(parse_comment(b"", Seen1), Err(Seen1)); // x-|
    assert_eq!(parse_comment(b"", Seen2), Err(Seen2)); // --|

    assert_eq!(parse_comment(b"-", Seen0), Err(Seen1)); // xx|-
    assert_eq!(parse_comment(b"-", Seen1), Err(Seen2)); // x-|-
    assert_eq!(parse_comment(b"-", Seen2), Err(Seen2)); // --|-

    assert_eq!(parse_comment(b">", Seen0), Err(Seen0)); // xx|>
    assert_eq!(parse_comment(b">", Seen1), Err(Seen0)); // x-|>
    assert_eq!(parse_comment(b">", Seen2), Ok(1)); // --|>

    assert_eq!(parse_comment(b"--", Seen0), Err(Seen2)); // xx|--
    assert_eq!(parse_comment(b"--", Seen1), Err(Seen2)); // x-|--
    assert_eq!(parse_comment(b"--", Seen2), Err(Seen2)); // --|--

    assert_eq!(parse_comment(b"->", Seen0), Err(Seen0)); // xx|->
    assert_eq!(parse_comment(b"->", Seen1), Ok(2)); // x-|->
    assert_eq!(parse_comment(b"->", Seen2), Ok(2)); // --|->

    assert_eq!(parse_comment(b"-->", Seen0), Ok(3)); // xx|-->
    assert_eq!(parse_comment(b"-->", Seen1), Ok(3)); // x-|-->
    assert_eq!(parse_comment(b"-->", Seen2), Ok(3)); // --|-->

    assert_eq!(parse_comment(b">-->", Seen0), Ok(4)); // xx|>-->
    assert_eq!(parse_comment(b">-->", Seen1), Ok(4)); // x-|>-->
    assert_eq!(parse_comment(b">-->", Seen2), Ok(1)); // --|>-->

    assert_eq!(parse_comment(b"->-->", Seen0), Ok(5)); // xx|->-->
    assert_eq!(parse_comment(b"->-->", Seen1), Ok(2)); // x-|->-->
    assert_eq!(parse_comment(b"->-->", Seen2), Ok(2)); // --|->-->
}
