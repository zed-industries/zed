/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://drafts.csswg.org/css-syntax/#tokenization

use self::Token::*;
use crate::cow_rc_str::CowRcStr;
use crate::parser::{ArbitrarySubstitutionFunctions, ParserState};
use std::char;
use std::ops::Range;

#[cfg(not(feature = "dummy_match_byte"))]
use cssparser_macros::match_byte;

#[cfg(feature = "dummy_match_byte")]
macro_rules! match_byte {
    ($value:expr, $($rest:tt)* ) => {
        match $value {
            $(
                $rest
            )+
        }
    };
}

/// One of the pieces the CSS input is broken into.
///
/// Some components use `Cow` in order to borrow from the original input string
/// and avoid allocating/copying when possible.
#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    /// A [`<ident-token>`](https://drafts.csswg.org/css-syntax/#ident-token-diagram)
    Ident(CowRcStr<'a>),

    /// A [`<at-keyword-token>`](https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram)
    ///
    /// The value does not include the `@` marker.
    AtKeyword(CowRcStr<'a>),

    /// A [`<hash-token>`](https://drafts.csswg.org/css-syntax/#hash-token-diagram) with the type flag set to "unrestricted"
    ///
    /// The value does not include the `#` marker.
    Hash(CowRcStr<'a>),

    /// A [`<hash-token>`](https://drafts.csswg.org/css-syntax/#hash-token-diagram) with the type flag set to "id"
    ///
    /// The value does not include the `#` marker.
    IDHash(CowRcStr<'a>), // Hash that is a valid ID selector.

    /// A [`<string-token>`](https://drafts.csswg.org/css-syntax/#string-token-diagram)
    ///
    /// The value does not include the quotes.
    QuotedString(CowRcStr<'a>),

    /// A [`<url-token>`](https://drafts.csswg.org/css-syntax/#url-token-diagram)
    ///
    /// The value does not include the `url(` `)` markers.  Note that `url( <string-token> )` is represented by a
    /// `Function` token.
    UnquotedUrl(CowRcStr<'a>),

    /// A `<delim-token>`
    Delim(char),

    /// A [`<number-token>`](https://drafts.csswg.org/css-syntax/#number-token-diagram)
    Number {
        /// Whether the number had a `+` or `-` sign.
        ///
        /// This is used is some cases like the <An+B> micro syntax. (See the `parse_nth` function.)
        has_sign: bool,

        /// The value as a float
        value: f32,

        /// If the origin source did not include a fractional part, the value as an integer.
        int_value: Option<i32>,
    },

    /// A [`<percentage-token>`](https://drafts.csswg.org/css-syntax/#percentage-token-diagram)
    Percentage {
        /// Whether the number had a `+` or `-` sign.
        has_sign: bool,

        /// The value as a float, divided by 100 so that the nominal range is 0.0 to 1.0.
        unit_value: f32,

        /// If the origin source did not include a fractional part, the value as an integer.
        /// It is **not** divided by 100.
        int_value: Option<i32>,
    },

    /// A [`<dimension-token>`](https://drafts.csswg.org/css-syntax/#dimension-token-diagram)
    Dimension {
        /// Whether the number had a `+` or `-` sign.
        ///
        /// This is used is some cases like the <An+B> micro syntax. (See the `parse_nth` function.)
        has_sign: bool,

        /// The value as a float
        value: f32,

        /// If the origin source did not include a fractional part, the value as an integer.
        int_value: Option<i32>,

        /// The unit, e.g. "px" in `12px`
        unit: CowRcStr<'a>,
    },

    /// A [`<whitespace-token>`](https://drafts.csswg.org/css-syntax/#whitespace-token-diagram)
    WhiteSpace(&'a str),

    /// A comment.
    ///
    /// The CSS Syntax spec does not generate tokens for comments,
    /// But we do, because we can (borrowed &str makes it cheap).
    ///
    /// The value does not include the `/*` `*/` markers.
    Comment(&'a str),

    /// A `:` `<colon-token>`
    Colon, // :

    /// A `;` `<semicolon-token>`
    Semicolon, // ;

    /// A `,` `<comma-token>`
    Comma, // ,

    /// A `~=` [`<include-match-token>`](https://drafts.csswg.org/css-syntax/#include-match-token-diagram)
    IncludeMatch,

    /// A `|=` [`<dash-match-token>`](https://drafts.csswg.org/css-syntax/#dash-match-token-diagram)
    DashMatch,

    /// A `^=` [`<prefix-match-token>`](https://drafts.csswg.org/css-syntax/#prefix-match-token-diagram)
    PrefixMatch,

    /// A `$=` [`<suffix-match-token>`](https://drafts.csswg.org/css-syntax/#suffix-match-token-diagram)
    SuffixMatch,

    /// A `*=` [`<substring-match-token>`](https://drafts.csswg.org/css-syntax/#substring-match-token-diagram)
    SubstringMatch,

    /// A `<!--` [`<CDO-token>`](https://drafts.csswg.org/css-syntax/#CDO-token-diagram)
    CDO,

    /// A `-->` [`<CDC-token>`](https://drafts.csswg.org/css-syntax/#CDC-token-diagram)
    CDC,

    /// A [`<function-token>`](https://drafts.csswg.org/css-syntax/#function-token-diagram)
    ///
    /// The value (name) does not include the `(` marker.
    Function(CowRcStr<'a>),

    /// A `<(-token>`
    ParenthesisBlock,

    /// A `<[-token>`
    SquareBracketBlock,

    /// A `<{-token>`
    CurlyBracketBlock,

    /// A `<bad-url-token>`
    ///
    /// This token always indicates a parse error.
    BadUrl(CowRcStr<'a>),

    /// A `<bad-string-token>`
    ///
    /// This token always indicates a parse error.
    BadString(CowRcStr<'a>),

    /// A `<)-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseParenthesis,

    /// A `<]-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseSquareBracket,

    /// A `<}-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseCurlyBracket,
}

impl Token<'_> {
    /// Return whether this token represents a parse error.
    ///
    /// `BadUrl` and `BadString` are tokenizer-level parse errors.
    ///
    /// `CloseParenthesis`, `CloseSquareBracket`, and `CloseCurlyBracket` are *unmatched*
    /// and therefore parse errors when returned by one of the `Parser::next*` methods.
    pub fn is_parse_error(&self) -> bool {
        matches!(
            *self,
            BadUrl(_) | BadString(_) | CloseParenthesis | CloseSquareBracket | CloseCurlyBracket
        )
    }
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
    input: &'a str,
    /// Counted in bytes, not code points. From 0.
    position: usize,
    /// The position at the start of the current line; but adjusted to
    /// ensure that computing the column will give the result in units
    /// of UTF-16 characters.
    current_line_start_position: usize,
    current_line_number: u32,
    arbitrary_substitution_functions: SeenStatus<'a>,
    source_map_url: Option<&'a str>,
    source_url: Option<&'a str>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum SeenStatus<'a> {
    DontCare,
    LookingForThem(ArbitrarySubstitutionFunctions<'a>),
    SeenAtLeastOne,
}

impl<'a> Tokenizer<'a> {
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            position: 0,
            current_line_start_position: 0,
            current_line_number: 0,
            arbitrary_substitution_functions: SeenStatus::DontCare,
            source_map_url: None,
            source_url: None,
        }
    }

    #[inline]
    pub fn look_for_arbitrary_substitution_functions(
        &mut self,
        fns: ArbitrarySubstitutionFunctions<'a>,
    ) {
        self.arbitrary_substitution_functions = SeenStatus::LookingForThem(fns);
    }

    #[inline]
    pub fn seen_arbitrary_substitution_functions(&mut self) -> bool {
        let seen = self.arbitrary_substitution_functions == SeenStatus::SeenAtLeastOne;
        self.arbitrary_substitution_functions = SeenStatus::DontCare;
        seen
    }

    #[inline]
    pub fn see_function(&mut self, name: &str) {
        if let SeenStatus::LookingForThem(fns) = self.arbitrary_substitution_functions {
            if fns.iter().any(|a| name.eq_ignore_ascii_case(a)) {
                self.arbitrary_substitution_functions = SeenStatus::SeenAtLeastOne;
            }
        }
    }

    #[inline]
    pub fn next(&mut self) -> Result<Token<'a>, ()> {
        next_token(self)
    }

    #[inline]
    pub fn position(&self) -> SourcePosition {
        debug_assert!(self.input.is_char_boundary(self.position));
        SourcePosition(self.position)
    }

    #[inline]
    pub fn current_source_location(&self) -> SourceLocation {
        SourceLocation {
            line: self.current_line_number,
            column: (self.position - self.current_line_start_position + 1) as u32,
        }
    }

    #[inline]
    pub fn current_source_map_url(&self) -> Option<&'a str> {
        self.source_map_url
    }

    #[inline]
    pub fn current_source_url(&self) -> Option<&'a str> {
        self.source_url
    }

    #[inline]
    pub fn state(&self) -> ParserState {
        ParserState {
            position: self.position,
            current_line_start_position: self.current_line_start_position,
            current_line_number: self.current_line_number,
            at_start_of: None,
        }
    }

    #[inline]
    pub fn reset(&mut self, state: &ParserState) {
        self.position = state.position;
        self.current_line_start_position = state.current_line_start_position;
        self.current_line_number = state.current_line_number;
    }

    #[inline]
    pub(crate) fn slice_from(&self, start_pos: SourcePosition) -> &'a str {
        self.slice(start_pos..self.position())
    }

    #[inline]
    pub(crate) fn slice(&self, range: Range<SourcePosition>) -> &'a str {
        debug_assert!(self.input.is_char_boundary(range.start.0));
        debug_assert!(self.input.is_char_boundary(range.end.0));
        unsafe { self.input.get_unchecked(range.start.0..range.end.0) }
    }

    pub fn current_source_line(&self) -> &'a str {
        let current = self.position();
        let start = self
            .slice(SourcePosition(0)..current)
            .rfind(['\r', '\n', '\x0C'])
            .map_or(0, |start| start + 1);
        let end = self
            .slice(current..SourcePosition(self.input.len()))
            .find(['\r', '\n', '\x0C'])
            .map_or(self.input.len(), |end| current.0 + end);
        self.slice(SourcePosition(start)..SourcePosition(end))
    }

    #[inline]
    pub fn next_byte(&self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            Some(self.input.as_bytes()[self.position])
        }
    }

    // If false, `tokenizer.next_char()` will not panic.
    #[inline]
    fn is_eof(&self) -> bool {
        !self.has_at_least(0)
    }

    // If true, the input has at least `n` bytes left *after* the current one.
    // That is, `tokenizer.char_at(n)` will not panic.
    #[inline]
    fn has_at_least(&self, n: usize) -> bool {
        self.position + n < self.input.len()
    }

    // Advance over N bytes in the input.  This function can advance
    // over ASCII bytes (excluding newlines), or UTF-8 sequence
    // leaders (excluding leaders for 4-byte sequences).
    #[inline]
    pub fn advance(&mut self, n: usize) {
        if cfg!(debug_assertions) {
            // Each byte must either be an ASCII byte or a sequence
            // leader, but not a 4-byte leader; also newlines are
            // rejected.
            for i in 0..n {
                let b = self.byte_at(i);
                debug_assert!(b.is_ascii() || (b & 0xF0 != 0xF0 && b & 0xC0 != 0x80));
                debug_assert!(b != b'\r' && b != b'\n' && b != b'\x0C');
            }
        }
        self.position += n
    }

    // Assumes non-EOF
    #[inline]
    fn next_byte_unchecked(&self) -> u8 {
        self.byte_at(0)
    }

    #[inline]
    fn byte_at(&self, offset: usize) -> u8 {
        self.input.as_bytes()[self.position + offset]
    }

    // Advance over a single byte; the byte must be a UTF-8 sequence
    // leader for a 4-byte sequence.
    #[inline]
    fn consume_4byte_intro(&mut self) {
        debug_assert!(self.next_byte_unchecked() & 0xF0 == 0xF0);
        // This takes two UTF-16 characters to represent, so we
        // actually have an undercount.
        self.current_line_start_position = self.current_line_start_position.wrapping_sub(1);
        self.position += 1;
    }

    // Advance over a single byte; the byte must be a UTF-8
    // continuation byte.
    #[inline]
    fn consume_continuation_byte(&mut self) {
        debug_assert!(self.next_byte_unchecked() & 0xC0 == 0x80);
        // Continuation bytes contribute to column overcount.  Note
        // that due to the special case for the 4-byte sequence intro,
        // we must use wrapping add here.
        self.current_line_start_position = self.current_line_start_position.wrapping_add(1);
        self.position += 1;
    }

    // Advance over any kind of byte, excluding newlines.
    #[inline(never)]
    fn consume_known_byte(&mut self, byte: u8) {
        debug_assert!(byte != b'\r' && byte != b'\n' && byte != b'\x0C');
        self.position += 1;
        // Continuation bytes contribute to column overcount.
        if byte & 0xF0 == 0xF0 {
            // This takes two UTF-16 characters to represent, so we
            // actually have an undercount.
            self.current_line_start_position = self.current_line_start_position.wrapping_sub(1);
        } else if byte & 0xC0 == 0x80 {
            // Note that due to the special case for the 4-byte
            // sequence intro, we must use wrapping add here.
            self.current_line_start_position = self.current_line_start_position.wrapping_add(1);
        }
    }

    #[inline]
    fn next_char(&self) -> char {
        unsafe { self.input.get_unchecked(self.position().0..) }
            .chars()
            .next()
            .unwrap()
    }

    // Given that a newline has been seen, advance over the newline
    // and update the state.
    #[inline]
    fn consume_newline(&mut self) {
        let byte = self.next_byte_unchecked();
        debug_assert!(byte == b'\r' || byte == b'\n' || byte == b'\x0C');
        self.position += 1;
        if byte == b'\r' && self.next_byte() == Some(b'\n') {
            self.position += 1;
        }
        self.current_line_start_position = self.position;
        self.current_line_number += 1;
    }

    #[inline]
    fn has_newline_at(&self, offset: usize) -> bool {
        self.position + offset < self.input.len()
            && matches!(self.byte_at(offset), b'\n' | b'\r' | b'\x0C')
    }

    #[inline]
    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        let len_utf8 = c.len_utf8();
        self.position += len_utf8;
        // Note that due to the special case for the 4-byte sequence
        // intro, we must use wrapping add here.
        self.current_line_start_position = self
            .current_line_start_position
            .wrapping_add(len_utf8 - c.len_utf16());
        c
    }

    #[inline]
    fn starts_with(&self, needle: &[u8]) -> bool {
        self.input.as_bytes()[self.position..].starts_with(needle)
    }

    pub fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte_unchecked(),
                b' ' | b'\t' => {
                    self.advance(1)
                },
                b'\n' | b'\x0C' | b'\r' => {
                    self.consume_newline();
                },
                b'/' => {
                    if self.starts_with(b"/*") {
                        consume_comment(self);
                    } else {
                        return
                    }
                }
                _ => return,
            }
        }
    }

    pub fn skip_cdc_and_cdo(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte_unchecked(),
                b' ' | b'\t' => {
                    self.advance(1)
                },
                b'\n' | b'\x0C' | b'\r' => {
                    self.consume_newline();
                },
                b'/' => {
                    if self.starts_with(b"/*") {
                        consume_comment(self);
                    } else {
                        return
                    }
                }
                b'<' => {
                    if self.starts_with(b"<!--") {
                        self.advance(4)
                    } else {
                        return
                    }
                }
                b'-' => {
                    if self.starts_with(b"-->") {
                        self.advance(3)
                    } else {
                        return
                    }
                }
                _ => {
                    return
                }
            }
        }
    }
}

/// A position from the start of the input, counted in UTF-8 bytes.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct SourcePosition(pub(crate) usize);

#[cfg(feature = "malloc_size_of")]
malloc_size_of::malloc_size_of_is_0!(SourcePosition);

impl SourcePosition {
    /// Returns the current byte index in the original input.
    #[inline]
    pub fn byte_index(&self) -> usize {
        self.0
    }
}

/// The line and column number for a given position within the input.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Default)]
pub struct SourceLocation {
    /// The line number, starting at 0 for the first line.
    pub line: u32,

    /// The column number within a line, starting at 1 for first the character of the line.
    /// Column numbers are counted in UTF-16 code units.
    pub column: u32,
}

#[cfg(feature = "malloc_size_of")]
malloc_size_of::malloc_size_of_is_0!(SourceLocation);

fn next_token<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Token<'a>, ()> {
    if tokenizer.is_eof() {
        return Err(());
    }
    let b = tokenizer.next_byte_unchecked();
    let token = match_byte! { b,
        b' ' | b'\t' => {
            consume_whitespace(tokenizer, false)
        },
        b'\n' | b'\x0C' | b'\r' => consume_whitespace(tokenizer, true),
        b'"' => consume_string(tokenizer, false),
        b'#' => {
            tokenizer.advance(1);
            if is_ident_start(tokenizer) { IDHash(consume_name(tokenizer)) }
            else if !tokenizer.is_eof() &&
                matches!(tokenizer.next_byte_unchecked(), b'0'..=b'9' | b'-') {
                // Any other valid case here already resulted in IDHash.
                Hash(consume_name(tokenizer))
            }
            else { Delim('#') }
        },
        b'$' => {
            if tokenizer.starts_with(b"$=") { tokenizer.advance(2); SuffixMatch }
            else { tokenizer.advance(1); Delim('$') }
        },
        b'\'' => consume_string(tokenizer, true),
        b'(' => { tokenizer.advance(1); ParenthesisBlock },
        b')' => { tokenizer.advance(1); CloseParenthesis },
        b'*' => {
            if tokenizer.starts_with(b"*=") { tokenizer.advance(2); SubstringMatch }
            else { tokenizer.advance(1); Delim('*') }
        },
        b'+' => {
            if (
                tokenizer.has_at_least(1)
                && tokenizer.byte_at(1).is_ascii_digit()
            ) || (
                tokenizer.has_at_least(2)
                && tokenizer.byte_at(1) == b'.'
                && tokenizer.byte_at(2).is_ascii_digit()
            ) {
                consume_numeric(tokenizer)
            } else {
                tokenizer.advance(1);
                Delim('+')
            }
        },
        b',' => { tokenizer.advance(1); Comma },
        b'-' => {
            if (
                tokenizer.has_at_least(1)
                && tokenizer.byte_at(1).is_ascii_digit()
            ) || (
                tokenizer.has_at_least(2)
                && tokenizer.byte_at(1) == b'.'
                && tokenizer.byte_at(2).is_ascii_digit()
            ) {
                consume_numeric(tokenizer)
            } else if tokenizer.starts_with(b"-->") {
                tokenizer.advance(3);
                CDC
            } else if is_ident_start(tokenizer) {
                consume_ident_like(tokenizer)
            } else {
                tokenizer.advance(1);
                Delim('-')
            }
        },
        b'.' => {
            if tokenizer.has_at_least(1)
                && tokenizer.byte_at(1).is_ascii_digit() {
                consume_numeric(tokenizer)
            } else {
                tokenizer.advance(1);
                Delim('.')
            }
        }
        b'/' => {
            if tokenizer.starts_with(b"/*") {
                Comment(consume_comment(tokenizer))
            } else {
                tokenizer.advance(1);
                Delim('/')
            }
        }
        b'0'..=b'9' => consume_numeric(tokenizer),
        b':' => { tokenizer.advance(1); Colon },
        b';' => { tokenizer.advance(1); Semicolon },
        b'<' => {
            if tokenizer.starts_with(b"<!--") {
                tokenizer.advance(4);
                CDO
            } else {
                tokenizer.advance(1);
                Delim('<')
            }
        },
        b'@' => {
            tokenizer.advance(1);
            if is_ident_start(tokenizer) { AtKeyword(consume_name(tokenizer)) }
            else { Delim('@') }
        },
        b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'\0' => consume_ident_like(tokenizer),
        b'[' => { tokenizer.advance(1); SquareBracketBlock },
        b'\\' => {
            if !tokenizer.has_newline_at(1) { consume_ident_like(tokenizer) }
            else { tokenizer.advance(1); Delim('\\') }
        },
        b']' => { tokenizer.advance(1); CloseSquareBracket },
        b'^' => {
            if tokenizer.starts_with(b"^=") { tokenizer.advance(2); PrefixMatch }
            else { tokenizer.advance(1); Delim('^') }
        },
        b'{' => { tokenizer.advance(1); CurlyBracketBlock },
        b'|' => {
            if tokenizer.starts_with(b"|=") { tokenizer.advance(2); DashMatch }
            else { tokenizer.advance(1); Delim('|') }
        },
        b'}' => { tokenizer.advance(1); CloseCurlyBracket },
        b'~' => {
            if tokenizer.starts_with(b"~=") { tokenizer.advance(2); IncludeMatch }
            else { tokenizer.advance(1); Delim('~') }
        },
        _ => {
            if !b.is_ascii() {
                consume_ident_like(tokenizer)
            } else {
                tokenizer.advance(1);
                Delim(b as char)
            }
        },
    };
    Ok(token)
}

fn consume_whitespace<'a>(tokenizer: &mut Tokenizer<'a>, newline: bool) -> Token<'a> {
    let start_position = tokenizer.position();
    if newline {
        tokenizer.consume_newline();
    } else {
        tokenizer.advance(1);
    }
    while !tokenizer.is_eof() {
        let b = tokenizer.next_byte_unchecked();
        match_byte! { b,
            b' ' | b'\t' => {
                tokenizer.advance(1);
            }
            b'\n' | b'\x0C' | b'\r' => {
                tokenizer.consume_newline();
            }
            _ => {
                break
            }
        }
    }
    WhiteSpace(tokenizer.slice_from(start_position))
}

// Check for sourceMappingURL or sourceURL comments and update the
// tokenizer appropriately.
fn check_for_source_map<'a>(tokenizer: &mut Tokenizer<'a>, contents: &'a str) {
    let directive = "# sourceMappingURL=";
    let directive_old = "@ sourceMappingURL=";

    // If there is a source map directive, extract the URL.
    if contents.starts_with(directive) || contents.starts_with(directive_old) {
        let contents = &contents[directive.len()..];
        tokenizer.source_map_url = contents.split([' ', '\t', '\x0C', '\r', '\n']).next();
    }

    let directive = "# sourceURL=";
    let directive_old = "@ sourceURL=";

    // If there is a source map directive, extract the URL.
    if contents.starts_with(directive) || contents.starts_with(directive_old) {
        let contents = &contents[directive.len()..];
        tokenizer.source_url = contents.split([' ', '\t', '\x0C', '\r', '\n']).next()
    }
}

fn consume_comment<'a>(tokenizer: &mut Tokenizer<'a>) -> &'a str {
    tokenizer.advance(2); // consume "/*"
    let start_position = tokenizer.position();
    while !tokenizer.is_eof() {
        match_byte! { tokenizer.next_byte_unchecked(),
            b'*' => {
                let end_position = tokenizer.position();
                tokenizer.advance(1);
                if tokenizer.next_byte() == Some(b'/') {
                    tokenizer.advance(1);
                    let contents = tokenizer.slice(start_position..end_position);
                    check_for_source_map(tokenizer, contents);
                    return contents
                }
            }
            b'\n' | b'\x0C' | b'\r' => {
                tokenizer.consume_newline();
            }
            b'\x80'..=b'\xBF' => { tokenizer.consume_continuation_byte(); }
            b'\xF0'..=b'\xFF' => { tokenizer.consume_4byte_intro(); }
            _ => {
                // ASCII or other leading byte.
                tokenizer.advance(1);
            }
        }
    }
    let contents = tokenizer.slice_from(start_position);
    check_for_source_map(tokenizer, contents);
    contents
}

fn consume_string<'a>(tokenizer: &mut Tokenizer<'a>, single_quote: bool) -> Token<'a> {
    match consume_quoted_string(tokenizer, single_quote) {
        Ok(value) => QuotedString(value),
        Err(value) => BadString(value),
    }
}

/// Return `Err(())` on syntax error (ie. unescaped newline)
fn consume_quoted_string<'a>(
    tokenizer: &mut Tokenizer<'a>,
    single_quote: bool,
) -> Result<CowRcStr<'a>, CowRcStr<'a>> {
    tokenizer.advance(1); // Skip the initial quote
                          // start_pos is at code point boundary, after " or '
    let start_pos = tokenizer.position();
    let mut string_bytes;
    loop {
        if tokenizer.is_eof() {
            return Ok(tokenizer.slice_from(start_pos).into());
        }
        match_byte! { tokenizer.next_byte_unchecked(),
            b'"' => {
                if !single_quote {
                    let value = tokenizer.slice_from(start_pos);
                    tokenizer.advance(1);
                    return Ok(value.into())
                }
                tokenizer.advance(1);
            }
            b'\'' => {
                if single_quote {
                    let value = tokenizer.slice_from(start_pos);
                    tokenizer.advance(1);
                    return Ok(value.into())
                }
                tokenizer.advance(1);
            }
            b'\\' | b'\0' => {
                // * The tokenizer’s input is UTF-8 since it’s `&str`.
                // * start_pos is at a code point boundary
                // * so is the current position (which is before '\\' or '\0'
                //
                // So `string_bytes` is well-formed UTF-8.
                string_bytes = tokenizer.slice_from(start_pos).as_bytes().to_owned();
                break
            }
            b'\n' | b'\r' | b'\x0C' => {
                return Err(tokenizer.slice_from(start_pos).into())
            },
            b'\x80'..=b'\xBF' => { tokenizer.consume_continuation_byte(); }
            b'\xF0'..=b'\xFF' => { tokenizer.consume_4byte_intro(); }
            _ => {
                // ASCII or other leading byte.
                tokenizer.advance(1);
            }
        }
    }

    while !tokenizer.is_eof() {
        let b = tokenizer.next_byte_unchecked();
        match_byte! { b,
            b'\n' | b'\r' | b'\x0C' => {
                return Err(
                    // string_bytes is well-formed UTF-8, see other comments.
                    unsafe {
                        from_utf8_release_unchecked(string_bytes)
                    }.into()
                );
            }
            b'"' => {
                tokenizer.advance(1);
                if !single_quote {
                    break;
                }
            }
            b'\'' => {
                tokenizer.advance(1);
                if single_quote {
                    break;
                }
            }
            b'\\' => {
                tokenizer.advance(1);
                if !tokenizer.is_eof() {
                    match tokenizer.next_byte_unchecked() {
                        // Escaped newline
                        b'\n' | b'\x0C' | b'\r' => {
                            tokenizer.consume_newline();
                        }
                        // This pushes one well-formed code point
                        _ => consume_escape_and_write(tokenizer, &mut string_bytes)
                    }
                }
                // else: escaped EOF, do nothing.
                continue;
            }
            b'\0' => {
                tokenizer.advance(1);
                string_bytes.extend("\u{FFFD}".as_bytes());
                continue;
            }
            b'\x80'..=b'\xBF' => { tokenizer.consume_continuation_byte(); }
            b'\xF0'..=b'\xFF' => { tokenizer.consume_4byte_intro(); }
            _ => {
                // ASCII or other leading byte.
                tokenizer.advance(1);
            },
        }

        // If this byte is part of a multi-byte code point,
        // we’ll end up copying the whole code point before this loop does something else.
        string_bytes.push(b);
    }

    Ok(
        // string_bytes is well-formed UTF-8, see other comments.
        unsafe { from_utf8_release_unchecked(string_bytes) }.into(),
    )
}

#[inline]
fn is_ident_start(tokenizer: &Tokenizer) -> bool {
    !tokenizer.is_eof()
        && match_byte! { tokenizer.next_byte_unchecked(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'\0' => true,
            b'-' => {
                tokenizer.has_at_least(1) && match_byte! { tokenizer.byte_at(1),
                    b'a'..=b'z' | b'A'..=b'Z' | b'-' | b'_' | b'\0' => {
                        true
                    }
                    b'\\' => !tokenizer.has_newline_at(1),
                    b => !b.is_ascii(),
                }
            },
            b'\\' => !tokenizer.has_newline_at(1),
            b => !b.is_ascii(),
        }
}

fn consume_ident_like<'a>(tokenizer: &mut Tokenizer<'a>) -> Token<'a> {
    let value = consume_name(tokenizer);
    if !tokenizer.is_eof() && tokenizer.next_byte_unchecked() == b'(' {
        tokenizer.advance(1);
        if value.eq_ignore_ascii_case("url") {
            consume_unquoted_url(tokenizer).unwrap_or(Function(value))
        } else {
            tokenizer.see_function(&value);
            Function(value)
        }
    } else {
        Ident(value)
    }
}

fn consume_name<'a>(tokenizer: &mut Tokenizer<'a>) -> CowRcStr<'a> {
    // start_pos is the end of the previous token, therefore at a code point boundary
    let start_pos = tokenizer.position();
    let mut value_bytes;
    loop {
        if tokenizer.is_eof() {
            return tokenizer.slice_from(start_pos).into();
        }
        match_byte! { tokenizer.next_byte_unchecked(),
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-' => tokenizer.advance(1),
            b'\\' | b'\0' => {
                // * The tokenizer’s input is UTF-8 since it’s `&str`.
                // * start_pos is at a code point boundary
                // * so is the current position (which is before '\\' or '\0'
                //
                // So `value_bytes` is well-formed UTF-8.
                value_bytes = tokenizer.slice_from(start_pos).as_bytes().to_owned();
                break
            }
            b'\x80'..=b'\xBF' => { tokenizer.consume_continuation_byte(); }
            b'\xC0'..=b'\xEF' => { tokenizer.advance(1); }
            b'\xF0'..=b'\xFF' => { tokenizer.consume_4byte_intro(); }
            _b => {
                return tokenizer.slice_from(start_pos).into();
            }
        }
    }

    while !tokenizer.is_eof() {
        let b = tokenizer.next_byte_unchecked();
        match_byte! { b,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-'  => {
                tokenizer.advance(1);
                value_bytes.push(b)  // ASCII
            }
            b'\\' => {
                if tokenizer.has_newline_at(1) { break }
                tokenizer.advance(1);
                // This pushes one well-formed code point
                consume_escape_and_write(tokenizer, &mut value_bytes)
            }
            b'\0' => {
                tokenizer.advance(1);
                value_bytes.extend("\u{FFFD}".as_bytes());
            },
            b'\x80'..=b'\xBF' => {
                // This byte *is* part of a multi-byte code point,
                // we’ll end up copying the whole code point before this loop does something else.
                tokenizer.consume_continuation_byte();
                value_bytes.push(b)
            }
            b'\xC0'..=b'\xEF' => {
                // This byte *is* part of a multi-byte code point,
                // we’ll end up copying the whole code point before this loop does something else.
                tokenizer.advance(1);
                value_bytes.push(b)
            }
            b'\xF0'..=b'\xFF' => {
                tokenizer.consume_4byte_intro();
                value_bytes.push(b)
            }
            _ => {
                // ASCII
                break;
            }
        }
    }
    // string_bytes is well-formed UTF-8, see other comments.
    unsafe { from_utf8_release_unchecked(value_bytes) }.into()
}

fn byte_to_hex_digit(b: u8) -> Option<u32> {
    Some(match_byte! { b,
        b'0' ..= b'9' => b - b'0',
        b'a' ..= b'f' => b - b'a' + 10,
        b'A' ..= b'F' => b - b'A' + 10,
        _ => {
            return None
        }
    } as u32)
}

fn byte_to_decimal_digit(b: u8) -> Option<u32> {
    if b.is_ascii_digit() {
        Some((b - b'0') as u32)
    } else {
        None
    }
}

fn consume_numeric<'a>(tokenizer: &mut Tokenizer<'a>) -> Token<'a> {
    // Parse [+-]?\d*(\.\d+)?([eE][+-]?\d+)?
    // But this is always called so that there is at least one digit in \d*(\.\d+)?

    // Do all the math in f64 so that large numbers overflow to +/-inf
    // and i32::{MIN, MAX} are within range.

    let (has_sign, sign) = match tokenizer.next_byte_unchecked() {
        b'-' => (true, -1.),
        b'+' => (true, 1.),
        _ => (false, 1.),
    };
    if has_sign {
        tokenizer.advance(1);
    }

    let mut integral_part: f64 = 0.;
    while let Some(digit) = byte_to_decimal_digit(tokenizer.next_byte_unchecked()) {
        integral_part = integral_part * 10. + digit as f64;
        tokenizer.advance(1);
        if tokenizer.is_eof() {
            break;
        }
    }

    let mut is_integer = true;

    let mut fractional_part: f64 = 0.;
    if tokenizer.has_at_least(1)
        && tokenizer.next_byte_unchecked() == b'.'
        && tokenizer.byte_at(1).is_ascii_digit()
    {
        is_integer = false;
        tokenizer.advance(1); // Consume '.'
        let mut factor = 0.1;
        while let Some(digit) = byte_to_decimal_digit(tokenizer.next_byte_unchecked()) {
            fractional_part += digit as f64 * factor;
            factor *= 0.1;
            tokenizer.advance(1);
            if tokenizer.is_eof() {
                break;
            }
        }
    }

    let mut value = sign * (integral_part + fractional_part);

    if tokenizer.has_at_least(1)
        && matches!(tokenizer.next_byte_unchecked(), b'e' | b'E')
        && (tokenizer.byte_at(1).is_ascii_digit()
            || (tokenizer.has_at_least(2)
                && matches!(tokenizer.byte_at(1), b'+' | b'-')
                && tokenizer.byte_at(2).is_ascii_digit()))
    {
        is_integer = false;
        tokenizer.advance(1);
        let (has_sign, sign) = match tokenizer.next_byte_unchecked() {
            b'-' => (true, -1.),
            b'+' => (true, 1.),
            _ => (false, 1.),
        };
        if has_sign {
            tokenizer.advance(1);
        }
        let mut exponent: f64 = 0.;
        while let Some(digit) = byte_to_decimal_digit(tokenizer.next_byte_unchecked()) {
            exponent = exponent * 10. + digit as f64;
            tokenizer.advance(1);
            if tokenizer.is_eof() {
                break;
            }
        }
        value *= f64::powf(10., sign * exponent);
    }

    let int_value = if is_integer {
        Some(if value >= i32::MAX as f64 {
            i32::MAX
        } else if value <= i32::MIN as f64 {
            i32::MIN
        } else {
            value as i32
        })
    } else {
        None
    };

    if !tokenizer.is_eof() && tokenizer.next_byte_unchecked() == b'%' {
        tokenizer.advance(1);
        return Percentage {
            unit_value: (value / 100.) as f32,
            int_value,
            has_sign,
        };
    }
    let value = value as f32;
    if is_ident_start(tokenizer) {
        let unit = consume_name(tokenizer);
        Dimension {
            value,
            int_value,
            has_sign,
            unit,
        }
    } else {
        Number {
            value,
            int_value,
            has_sign,
        }
    }
}

#[inline]
unsafe fn from_utf8_release_unchecked(string_bytes: Vec<u8>) -> String {
    if cfg!(debug_assertions) {
        String::from_utf8(string_bytes).unwrap()
    } else {
        String::from_utf8_unchecked(string_bytes)
    }
}

fn consume_unquoted_url<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Token<'a>, ()> {
    // This is only called after "url(", so the current position is a code point boundary.
    let start_position = tokenizer.position;
    let from_start = &tokenizer.input[tokenizer.position..];
    let mut newlines = 0;
    let mut last_newline = 0;
    let mut found_printable_char = false;
    let mut iter = from_start.bytes().enumerate();
    loop {
        let (offset, b) = match iter.next() {
            Some(item) => item,
            None => {
                tokenizer.position = tokenizer.input.len();
                break;
            }
        };
        match_byte! { b,
            b' ' | b'\t' => {},
            b'\n' | b'\x0C' => {
                newlines += 1;
                last_newline = offset;
            }
            b'\r' => {
                if from_start.as_bytes().get(offset + 1) != Some(&b'\n') {
                    newlines += 1;
                    last_newline = offset;
                }
            }
            b'"' | b'\'' => return Err(()),  // Do not advance
            b')' => {
                // Don't use advance, because we may be skipping
                // newlines here, and we want to avoid the assert.
                tokenizer.position += offset + 1;
                break
            }
            _ => {
                // Don't use advance, because we may be skipping
                // newlines here, and we want to avoid the assert.
                tokenizer.position += offset;
                found_printable_char = true;
                break
            }
        }
    }

    if newlines > 0 {
        tokenizer.current_line_number += newlines;
        // No need for wrapping_add here, because there's no possible
        // way to wrap.
        tokenizer.current_line_start_position = start_position + last_newline + 1;
    }

    if found_printable_char {
        // This function only consumed ASCII (whitespace) bytes,
        // so the current position is a code point boundary.
        return Ok(consume_unquoted_url_internal(tokenizer));
    } else {
        return Ok(UnquotedUrl("".into()));
    }

    fn consume_unquoted_url_internal<'a>(tokenizer: &mut Tokenizer<'a>) -> Token<'a> {
        // This function is only called with start_pos at a code point boundary.
        let start_pos = tokenizer.position();
        let mut string_bytes: Vec<u8>;
        loop {
            if tokenizer.is_eof() {
                return UnquotedUrl(tokenizer.slice_from(start_pos).into());
            }
            match_byte! { tokenizer.next_byte_unchecked(),
                b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' => {
                    let value = tokenizer.slice_from(start_pos);
                    return consume_url_end(tokenizer, start_pos, value.into())
                }
                b')' => {
                    let value = tokenizer.slice_from(start_pos);
                    tokenizer.advance(1);
                    return UnquotedUrl(value.into())
                }
                b'\x01'..=b'\x08' | b'\x0B' | b'\x0E'..=b'\x1F' | b'\x7F'  // non-printable
                    | b'"' | b'\'' | b'(' => {
                    tokenizer.advance(1);
                    return consume_bad_url(tokenizer, start_pos)
                },
                b'\\' | b'\0' => {
                    // * The tokenizer’s input is UTF-8 since it’s `&str`.
                    // * start_pos is at a code point boundary
                    // * so is the current position (which is before '\\' or '\0'
                    //
                    // So `string_bytes` is well-formed UTF-8.
                    string_bytes = tokenizer.slice_from(start_pos).as_bytes().to_owned();
                    break
                }
                b'\x80'..=b'\xBF' => { tokenizer.consume_continuation_byte(); }
                b'\xF0'..=b'\xFF' => { tokenizer.consume_4byte_intro(); }
                _ => {
                    // ASCII or other leading byte.
                    tokenizer.advance(1);
                }
            }
        }
        while !tokenizer.is_eof() {
            let b = tokenizer.next_byte_unchecked();
            match_byte! { b,
                b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' => {
                    // string_bytes is well-formed UTF-8, see other comments.
                    let string = unsafe { from_utf8_release_unchecked(string_bytes) }.into();
                    return consume_url_end(tokenizer, start_pos, string)
                }
                b')' => {
                    tokenizer.advance(1);
                    break;
                }
                b'\x01'..=b'\x08' | b'\x0B' | b'\x0E'..=b'\x1F' | b'\x7F'  // non-printable
                    | b'"' | b'\'' | b'(' => {
                    tokenizer.advance(1);
                    return consume_bad_url(tokenizer, start_pos);
                }
                b'\\' => {
                    tokenizer.advance(1);
                    if tokenizer.has_newline_at(0) {
                        return consume_bad_url(tokenizer, start_pos)
                    }

                    // This pushes one well-formed code point to string_bytes
                    consume_escape_and_write(tokenizer, &mut string_bytes)
                },
                b'\0' => {
                    tokenizer.advance(1);
                    string_bytes.extend("\u{FFFD}".as_bytes());
                }
                b'\x80'..=b'\xBF' => {
                    // We’ll end up copying the whole code point
                    // before this loop does something else.
                    tokenizer.consume_continuation_byte();
                    string_bytes.push(b);
                }
                b'\xF0'..=b'\xFF' => {
                    // We’ll end up copying the whole code point
                    // before this loop does something else.
                    tokenizer.consume_4byte_intro();
                    string_bytes.push(b);
                }
                // If this byte is part of a multi-byte code point,
                // we’ll end up copying the whole code point before this loop does something else.
                b => {
                    // ASCII or other leading byte.
                    tokenizer.advance(1);
                    string_bytes.push(b)
                }
            }
        }
        UnquotedUrl(
            // string_bytes is well-formed UTF-8, see other comments.
            unsafe { from_utf8_release_unchecked(string_bytes) }.into(),
        )
    }

    fn consume_url_end<'a>(
        tokenizer: &mut Tokenizer<'a>,
        start_pos: SourcePosition,
        string: CowRcStr<'a>,
    ) -> Token<'a> {
        while !tokenizer.is_eof() {
            match_byte! { tokenizer.next_byte_unchecked(),
                b')' => {
                    tokenizer.advance(1);
                    break
                }
                b' ' | b'\t' => { tokenizer.advance(1); }
                b'\n' | b'\x0C' | b'\r' => {
                    tokenizer.consume_newline();
                }
                b => {
                    tokenizer.consume_known_byte(b);
                    return consume_bad_url(tokenizer, start_pos);
                }
            }
        }
        UnquotedUrl(string)
    }

    fn consume_bad_url<'a>(tokenizer: &mut Tokenizer<'a>, start_pos: SourcePosition) -> Token<'a> {
        // Consume up to the closing )
        while !tokenizer.is_eof() {
            match_byte! { tokenizer.next_byte_unchecked(),
                b')' => {
                    let contents = tokenizer.slice_from(start_pos).into();
                    tokenizer.advance(1);
                    return BadUrl(contents)
                }
                b'\\' => {
                    tokenizer.advance(1);
                    if matches!(tokenizer.next_byte(), Some(b')') | Some(b'\\')) {
                        tokenizer.advance(1); // Skip an escaped ')' or '\'
                    }
                }
                b'\n' | b'\x0C' | b'\r' => {
                    tokenizer.consume_newline();
                }
                b => {
                    tokenizer.consume_known_byte(b);
                }
            }
        }
        BadUrl(tokenizer.slice_from(start_pos).into())
    }
}

// (value, number of digits up to 6)
fn consume_hex_digits(tokenizer: &mut Tokenizer<'_>) -> (u32, u32) {
    let mut value = 0;
    let mut digits = 0;
    while digits < 6 && !tokenizer.is_eof() {
        match byte_to_hex_digit(tokenizer.next_byte_unchecked()) {
            Some(digit) => {
                value = value * 16 + digit;
                digits += 1;
                tokenizer.advance(1);
            }
            None => break,
        }
    }
    (value, digits)
}

// Same constraints as consume_escape except it writes into `bytes` the result
// instead of returning it.
fn consume_escape_and_write(tokenizer: &mut Tokenizer, bytes: &mut Vec<u8>) {
    bytes.extend(
        consume_escape(tokenizer)
            .encode_utf8(&mut [0; 4])
            .as_bytes(),
    )
}

// Assumes that the U+005C REVERSE SOLIDUS (\) has already been consumed
// and that the next input character has already been verified
// to not be a newline.
fn consume_escape(tokenizer: &mut Tokenizer) -> char {
    if tokenizer.is_eof() {
        return '\u{FFFD}';
    } // Escaped EOF
    match_byte! { tokenizer.next_byte_unchecked(),
        b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f' => {
            let (c, _) = consume_hex_digits(tokenizer);
            if !tokenizer.is_eof() {
                match_byte! { tokenizer.next_byte_unchecked(),
                    b' ' | b'\t' => {
                        tokenizer.advance(1)
                    }
                    b'\n' | b'\x0C' | b'\r' => {
                        tokenizer.consume_newline();
                    }
                    _ => {}
                }
            }
            static REPLACEMENT_CHAR: char = '\u{FFFD}';
            if c != 0 {
                let c = char::from_u32(c);
                c.unwrap_or(REPLACEMENT_CHAR)
            } else {
                REPLACEMENT_CHAR
            }
        },
        b'\0' => {
            tokenizer.advance(1);
            '\u{FFFD}'
        }
        _ => tokenizer.consume_char(),
    }
}
