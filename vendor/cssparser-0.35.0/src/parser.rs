/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::cow_rc_str::CowRcStr;
use crate::tokenizer::{SourceLocation, SourcePosition, Token, Tokenizer};
use smallvec::SmallVec;
use std::fmt;
use std::ops::BitOr;
use std::ops::Range;

/// A capture of the internal state of a `Parser` (including the position within the input),
/// obtained from the `Parser::position` method.
///
/// Can be used with the `Parser::reset` method to restore that state.
/// Should only be used with the `Parser` instance it came from.
#[derive(Debug, Clone)]
pub struct ParserState {
    pub(crate) position: usize,
    pub(crate) current_line_start_position: usize,
    pub(crate) current_line_number: u32,
    pub(crate) at_start_of: Option<BlockType>,
}

impl ParserState {
    /// The position from the start of the input, counted in UTF-8 bytes.
    #[inline]
    pub fn position(&self) -> SourcePosition {
        SourcePosition(self.position)
    }

    /// The line number and column number
    #[inline]
    pub fn source_location(&self) -> SourceLocation {
        SourceLocation {
            line: self.current_line_number,
            column: (self.position - self.current_line_start_position + 1) as u32,
        }
    }
}

/// When parsing until a given token, sometimes the caller knows that parsing is going to restart
/// at some earlier point, and consuming until we find a top level delimiter is just wasted work.
///
/// In that case, callers can pass ParseUntilErrorBehavior::Stop to avoid doing all that wasted
/// work.
///
/// This is important for things like CSS nesting, where something like:
///
///   foo:is(..) {
///     ...
///   }
///
/// Would need to scan the whole {} block to find a semicolon, only for parsing getting restarted
/// as a qualified rule later.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseUntilErrorBehavior {
    /// Consume until we see the relevant delimiter or the end of the stream.
    Consume,
    /// Eagerly error.
    Stop,
}

/// Details about a `BasicParseError`
#[derive(Clone, Debug, PartialEq)]
pub enum BasicParseErrorKind<'i> {
    /// An unexpected token was encountered.
    UnexpectedToken(Token<'i>),
    /// The end of the input was encountered unexpectedly.
    EndOfInput,
    /// An `@` rule was encountered that was invalid.
    AtRuleInvalid(CowRcStr<'i>),
    /// The body of an '@' rule was invalid.
    AtRuleBodyInvalid,
    /// A qualified rule was encountered that was invalid.
    QualifiedRuleInvalid,
}

impl fmt::Display for BasicParseErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BasicParseErrorKind::UnexpectedToken(token) => {
                write!(f, "unexpected token: {:?}", token)
            }
            BasicParseErrorKind::EndOfInput => write!(f, "unexpected end of input"),
            BasicParseErrorKind::AtRuleInvalid(rule) => {
                write!(f, "invalid @ rule encountered: '@{}'", rule)
            }
            BasicParseErrorKind::AtRuleBodyInvalid => write!(f, "invalid @ rule body encountered"),
            BasicParseErrorKind::QualifiedRuleInvalid => {
                write!(f, "invalid qualified rule encountered")
            }
        }
    }
}

/// The fundamental parsing errors that can be triggered by built-in parsing routines.
#[derive(Clone, Debug, PartialEq)]
pub struct BasicParseError<'i> {
    /// Details of this error
    pub kind: BasicParseErrorKind<'i>,
    /// Location where this error occurred
    pub location: SourceLocation,
}

impl<'i, T> From<BasicParseError<'i>> for ParseError<'i, T> {
    #[inline]
    fn from(this: BasicParseError<'i>) -> ParseError<'i, T> {
        ParseError {
            kind: ParseErrorKind::Basic(this.kind),
            location: this.location,
        }
    }
}

impl SourceLocation {
    /// Create a new BasicParseError at this location for an unexpected token
    #[inline]
    pub fn new_basic_unexpected_token_error(self, token: Token<'_>) -> BasicParseError<'_> {
        self.new_basic_error(BasicParseErrorKind::UnexpectedToken(token))
    }

    /// Create a new BasicParseError at this location
    #[inline]
    pub fn new_basic_error(self, kind: BasicParseErrorKind<'_>) -> BasicParseError<'_> {
        BasicParseError {
            kind,
            location: self,
        }
    }

    /// Create a new ParseError at this location for an unexpected token
    #[inline]
    pub fn new_unexpected_token_error<E>(self, token: Token<'_>) -> ParseError<'_, E> {
        self.new_error(BasicParseErrorKind::UnexpectedToken(token))
    }

    /// Create a new basic ParseError at the current location
    #[inline]
    pub fn new_error<E>(self, kind: BasicParseErrorKind<'_>) -> ParseError<'_, E> {
        ParseError {
            kind: ParseErrorKind::Basic(kind),
            location: self,
        }
    }

    /// Create a new custom ParseError at this location
    #[inline]
    pub fn new_custom_error<'i, E1: Into<E2>, E2>(self, error: E1) -> ParseError<'i, E2> {
        ParseError {
            kind: ParseErrorKind::Custom(error.into()),
            location: self,
        }
    }
}

/// Details of a `ParseError`
#[derive(Clone, Debug, PartialEq)]
pub enum ParseErrorKind<'i, T: 'i> {
    /// A fundamental parse error from a built-in parsing routine.
    Basic(BasicParseErrorKind<'i>),
    /// A parse error reported by downstream consumer code.
    Custom(T),
}

impl<'i, T> ParseErrorKind<'i, T> {
    /// Like `std::convert::Into::into`
    pub fn into<U>(self) -> ParseErrorKind<'i, U>
    where
        T: Into<U>,
    {
        match self {
            ParseErrorKind::Basic(basic) => ParseErrorKind::Basic(basic),
            ParseErrorKind::Custom(custom) => ParseErrorKind::Custom(custom.into()),
        }
    }
}

impl<E: fmt::Display> fmt::Display for ParseErrorKind<'_, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErrorKind::Basic(ref basic) => basic.fmt(f),
            ParseErrorKind::Custom(ref custom) => custom.fmt(f),
        }
    }
}

/// Extensible parse errors that can be encountered by client parsing implementations.
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError<'i, E> {
    /// Details of this error
    pub kind: ParseErrorKind<'i, E>,
    /// Location where this error occurred
    pub location: SourceLocation,
}

impl<'i, T> ParseError<'i, T> {
    /// Extract the fundamental parse error from an extensible error.
    pub fn basic(self) -> BasicParseError<'i> {
        match self.kind {
            ParseErrorKind::Basic(kind) => BasicParseError {
                kind,
                location: self.location,
            },
            ParseErrorKind::Custom(_) => panic!("Not a basic parse error"),
        }
    }

    /// Like `std::convert::Into::into`
    pub fn into<U>(self) -> ParseError<'i, U>
    where
        T: Into<U>,
    {
        ParseError {
            kind: self.kind.into(),
            location: self.location,
        }
    }
}

impl<E: fmt::Display> fmt::Display for ParseError<'_, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl<E: fmt::Display + fmt::Debug> std::error::Error for ParseError<'_, E> {}

/// The owned input for a parser.
pub struct ParserInput<'i> {
    tokenizer: Tokenizer<'i>,
    cached_token: Option<CachedToken<'i>>,
}

struct CachedToken<'i> {
    token: Token<'i>,
    start_position: SourcePosition,
    end_state: ParserState,
}

impl<'i> ParserInput<'i> {
    /// Create a new input for a parser.
    pub fn new(input: &'i str) -> ParserInput<'i> {
        ParserInput {
            tokenizer: Tokenizer::new(input),
            cached_token: None,
        }
    }

    #[inline]
    fn cached_token_ref(&self) -> &Token<'i> {
        &self.cached_token.as_ref().unwrap().token
    }
}

/// A CSS parser that borrows its `&str` input,
/// yields `Token`s,
/// and keeps track of nested blocks and functions.
pub struct Parser<'i, 't> {
    input: &'t mut ParserInput<'i>,
    /// If `Some(_)`, .parse_nested_block() can be called.
    at_start_of: Option<BlockType>,
    /// For parsers from `parse_until` or `parse_nested_block`
    stop_before: Delimiters,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum BlockType {
    Parenthesis,
    SquareBracket,
    CurlyBracket,
}

impl BlockType {
    fn opening(token: &Token) -> Option<BlockType> {
        match *token {
            Token::Function(_) | Token::ParenthesisBlock => Some(BlockType::Parenthesis),
            Token::SquareBracketBlock => Some(BlockType::SquareBracket),
            Token::CurlyBracketBlock => Some(BlockType::CurlyBracket),
            _ => None,
        }
    }

    fn closing(token: &Token) -> Option<BlockType> {
        match *token {
            Token::CloseParenthesis => Some(BlockType::Parenthesis),
            Token::CloseSquareBracket => Some(BlockType::SquareBracket),
            Token::CloseCurlyBracket => Some(BlockType::CurlyBracket),
            _ => None,
        }
    }
}

/// A set of characters, to be used with the `Parser::parse_until*` methods.
///
/// The union of two sets can be obtained with the `|` operator. Example:
///
/// ```{rust,ignore}
/// input.parse_until_before(Delimiter::CurlyBracketBlock | Delimiter::Semicolon)
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Delimiters {
    bits: u8,
}

/// `Delimiters` constants.
#[allow(non_upper_case_globals, non_snake_case)]
pub mod Delimiter {
    use super::Delimiters;

    /// The empty delimiter set
    pub const None: Delimiters = Delimiters { bits: 0 };
    /// The delimiter set with only the `{` opening curly bracket
    pub const CurlyBracketBlock: Delimiters = Delimiters { bits: 1 << 1 };
    /// The delimiter set with only the `;` semicolon
    pub const Semicolon: Delimiters = Delimiters { bits: 1 << 2 };
    /// The delimiter set with only the `!` exclamation point
    pub const Bang: Delimiters = Delimiters { bits: 1 << 3 };
    /// The delimiter set with only the `,` comma
    pub const Comma: Delimiters = Delimiters { bits: 1 << 4 };
}

#[allow(non_upper_case_globals, non_snake_case)]
mod ClosingDelimiter {
    use super::Delimiters;

    pub const CloseCurlyBracket: Delimiters = Delimiters { bits: 1 << 5 };
    pub const CloseSquareBracket: Delimiters = Delimiters { bits: 1 << 6 };
    pub const CloseParenthesis: Delimiters = Delimiters { bits: 1 << 7 };
}

impl BitOr<Delimiters> for Delimiters {
    type Output = Delimiters;

    #[inline]
    fn bitor(self, other: Delimiters) -> Delimiters {
        Delimiters {
            bits: self.bits | other.bits,
        }
    }
}

impl Delimiters {
    #[inline]
    fn contains(self, other: Delimiters) -> bool {
        (self.bits & other.bits) != 0
    }

    #[inline]
    pub(crate) fn from_byte(byte: Option<u8>) -> Delimiters {
        const TABLE: [Delimiters; 256] = {
            let mut table = [Delimiter::None; 256];
            table[b';' as usize] = Delimiter::Semicolon;
            table[b'!' as usize] = Delimiter::Bang;
            table[b',' as usize] = Delimiter::Comma;
            table[b'{' as usize] = Delimiter::CurlyBracketBlock;
            table[b'}' as usize] = ClosingDelimiter::CloseCurlyBracket;
            table[b']' as usize] = ClosingDelimiter::CloseSquareBracket;
            table[b')' as usize] = ClosingDelimiter::CloseParenthesis;
            table
        };

        assert_eq!(TABLE[0], Delimiter::None);
        TABLE[byte.unwrap_or(0) as usize]
    }
}

/// Used in some `fn expect_*` methods
macro_rules! expect {
    ($parser: ident, $($branches: tt)+) => {
        {
            let start_location = $parser.current_source_location();
            match *$parser.next()? {
                $($branches)+
                ref token => {
                    return Err(start_location.new_basic_unexpected_token_error(token.clone()))
                }
            }
        }
    }
}

impl<'i: 't, 't> Parser<'i, 't> {
    /// Create a new parser
    #[inline]
    pub fn new(input: &'t mut ParserInput<'i>) -> Parser<'i, 't> {
        Parser {
            input,
            at_start_of: None,
            stop_before: Delimiter::None,
        }
    }

    /// Return the current line that is being parsed.
    pub fn current_line(&self) -> &'i str {
        self.input.tokenizer.current_source_line()
    }

    /// Check whether the input is exhausted. That is, if `.next()` would return a token.
    ///
    /// This ignores whitespace and comments.
    #[inline]
    pub fn is_exhausted(&mut self) -> bool {
        self.expect_exhausted().is_ok()
    }

    /// Check whether the input is exhausted. That is, if `.next()` would return a token.
    /// Return a `Result` so that the `?` operator can be used: `input.expect_exhausted()?`
    ///
    /// This ignores whitespace and comments.
    #[inline]
    pub fn expect_exhausted(&mut self) -> Result<(), BasicParseError<'i>> {
        let start = self.state();
        let result = match self.next() {
            Err(BasicParseError {
                kind: BasicParseErrorKind::EndOfInput,
                ..
            }) => Ok(()),
            Err(e) => unreachable!("Unexpected error encountered: {:?}", e),
            Ok(t) => Err(start
                .source_location()
                .new_basic_unexpected_token_error(t.clone())),
        };
        self.reset(&start);
        result
    }

    /// Return the current position within the input.
    ///
    /// This can be used with the `Parser::slice` and `slice_from` methods.
    #[inline]
    pub fn position(&self) -> SourcePosition {
        self.input.tokenizer.position()
    }

    /// The current line number and column number.
    #[inline]
    pub fn current_source_location(&self) -> SourceLocation {
        self.input.tokenizer.current_source_location()
    }

    /// The source map URL, if known.
    ///
    /// The source map URL is extracted from a specially formatted
    /// comment.  The last such comment is used, so this value may
    /// change as parsing proceeds.
    pub fn current_source_map_url(&self) -> Option<&str> {
        self.input.tokenizer.current_source_map_url()
    }

    /// The source URL, if known.
    ///
    /// The source URL is extracted from a specially formatted
    /// comment.  The last such comment is used, so this value may
    /// change as parsing proceeds.
    pub fn current_source_url(&self) -> Option<&str> {
        self.input.tokenizer.current_source_url()
    }

    /// Create a new BasicParseError at the current location
    #[inline]
    pub fn new_basic_error(&self, kind: BasicParseErrorKind<'i>) -> BasicParseError<'i> {
        self.current_source_location().new_basic_error(kind)
    }

    /// Create a new basic ParseError at the current location
    #[inline]
    pub fn new_error<E>(&self, kind: BasicParseErrorKind<'i>) -> ParseError<'i, E> {
        self.current_source_location().new_error(kind)
    }

    /// Create a new custom BasicParseError at the current location
    #[inline]
    pub fn new_custom_error<E1: Into<E2>, E2>(&self, error: E1) -> ParseError<'i, E2> {
        self.current_source_location().new_custom_error(error)
    }

    /// Create a new unexpected token BasicParseError at the current location
    #[inline]
    pub fn new_basic_unexpected_token_error(&self, token: Token<'i>) -> BasicParseError<'i> {
        self.new_basic_error(BasicParseErrorKind::UnexpectedToken(token))
    }

    /// Create a new unexpected token ParseError at the current location
    #[inline]
    pub fn new_unexpected_token_error<E>(&self, token: Token<'i>) -> ParseError<'i, E> {
        self.new_error(BasicParseErrorKind::UnexpectedToken(token))
    }

    /// Create a new unexpected token or EOF ParseError at the current location
    #[inline]
    pub fn new_error_for_next_token<E>(&mut self) -> ParseError<'i, E> {
        let token = match self.next() {
            Ok(token) => token.clone(),
            Err(e) => return e.into(),
        };
        self.new_error(BasicParseErrorKind::UnexpectedToken(token))
    }

    /// Return the current internal state of the parser (including position within the input).
    ///
    /// This state can later be restored with the `Parser::reset` method.
    #[inline]
    pub fn state(&self) -> ParserState {
        ParserState {
            at_start_of: self.at_start_of,
            ..self.input.tokenizer.state()
        }
    }

    /// Advance the input until the next token that’s not whitespace or a comment.
    #[inline]
    pub fn skip_whitespace(&mut self) {
        if let Some(block_type) = self.at_start_of.take() {
            consume_until_end_of_block(block_type, &mut self.input.tokenizer);
        }

        self.input.tokenizer.skip_whitespace()
    }

    #[inline]
    pub(crate) fn skip_cdc_and_cdo(&mut self) {
        if let Some(block_type) = self.at_start_of.take() {
            consume_until_end_of_block(block_type, &mut self.input.tokenizer);
        }

        self.input.tokenizer.skip_cdc_and_cdo()
    }

    #[inline]
    pub(crate) fn next_byte(&self) -> Option<u8> {
        let byte = self.input.tokenizer.next_byte();
        if self.stop_before.contains(Delimiters::from_byte(byte)) {
            return None;
        }
        byte
    }

    /// Restore the internal state of the parser (including position within the input)
    /// to what was previously saved by the `Parser::position` method.
    ///
    /// Should only be used with `SourcePosition` values from the same `Parser` instance.
    #[inline]
    pub fn reset(&mut self, state: &ParserState) {
        self.input.tokenizer.reset(state);
        self.at_start_of = state.at_start_of;
    }

    /// Start looking for `var()` / `env()` functions. (See the
    /// `.seen_var_or_env_functions()` method.)
    #[inline]
    pub fn look_for_var_or_env_functions(&mut self) {
        self.input.tokenizer.look_for_var_or_env_functions()
    }

    /// Return whether a `var()` or `env()` function has been seen by the
    /// tokenizer since either `look_for_var_or_env_functions` was called, and
    /// stop looking.
    #[inline]
    pub fn seen_var_or_env_functions(&mut self) -> bool {
        self.input.tokenizer.seen_var_or_env_functions()
    }

    /// The old name of `try_parse`, which requires raw identifiers in the Rust 2018 edition.
    #[inline]
    pub fn r#try<F, T, E>(&mut self, thing: F) -> Result<T, E>
    where
        F: FnOnce(&mut Parser<'i, 't>) -> Result<T, E>,
    {
        self.try_parse(thing)
    }

    /// Execute the given closure, passing it the parser.
    /// If the result (returned unchanged) is `Err`,
    /// the internal state of the parser  (including position within the input)
    /// is restored to what it was before the call.
    #[inline]
    pub fn try_parse<F, T, E>(&mut self, thing: F) -> Result<T, E>
    where
        F: FnOnce(&mut Parser<'i, 't>) -> Result<T, E>,
    {
        let start = self.state();
        let result = thing(self);
        if result.is_err() {
            self.reset(&start)
        }
        result
    }

    /// Return a slice of the CSS input
    #[inline]
    pub fn slice(&self, range: Range<SourcePosition>) -> &'i str {
        self.input.tokenizer.slice(range)
    }

    /// Return a slice of the CSS input, from the given position to the current one.
    #[inline]
    pub fn slice_from(&self, start_position: SourcePosition) -> &'i str {
        self.input.tokenizer.slice_from(start_position)
    }

    /// Return the next token in the input that is neither whitespace or a comment,
    /// and advance the position accordingly.
    ///
    /// After returning a `Function`, `ParenthesisBlock`,
    /// `CurlyBracketBlock`, or `SquareBracketBlock` token,
    /// the next call will skip until after the matching `CloseParenthesis`,
    /// `CloseCurlyBracket`, or `CloseSquareBracket` token.
    ///
    /// See the `Parser::parse_nested_block` method to parse the content of functions or blocks.
    ///
    /// This only returns a closing token when it is unmatched (and therefore an error).
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<&Token<'i>, BasicParseError<'i>> {
        self.skip_whitespace();
        self.next_including_whitespace_and_comments()
    }

    /// Same as `Parser::next`, but does not skip whitespace tokens.
    pub fn next_including_whitespace(&mut self) -> Result<&Token<'i>, BasicParseError<'i>> {
        loop {
            match self.next_including_whitespace_and_comments() {
                Err(e) => return Err(e),
                Ok(&Token::Comment(_)) => {}
                _ => break,
            }
        }
        Ok(self.input.cached_token_ref())
    }

    /// Same as `Parser::next`, but does not skip whitespace or comment tokens.
    ///
    /// **Note**: This should only be used in contexts like a CSS pre-processor
    /// where comments are preserved.
    /// When parsing higher-level values, per the CSS Syntax specification,
    /// comments should always be ignored between tokens.
    pub fn next_including_whitespace_and_comments(
        &mut self,
    ) -> Result<&Token<'i>, BasicParseError<'i>> {
        if let Some(block_type) = self.at_start_of.take() {
            consume_until_end_of_block(block_type, &mut self.input.tokenizer);
        }

        let byte = self.input.tokenizer.next_byte();
        if self.stop_before.contains(Delimiters::from_byte(byte)) {
            return Err(self.new_basic_error(BasicParseErrorKind::EndOfInput));
        }

        let token_start_position = self.input.tokenizer.position();
        let using_cached_token = self
            .input
            .cached_token
            .as_ref()
            .map_or(false, |cached_token| {
                cached_token.start_position == token_start_position
            });
        let token = if using_cached_token {
            let cached_token = self.input.cached_token.as_ref().unwrap();
            self.input.tokenizer.reset(&cached_token.end_state);
            if let Token::Function(ref name) = cached_token.token {
                self.input.tokenizer.see_function(name)
            }
            &cached_token.token
        } else {
            let new_token = self
                .input
                .tokenizer
                .next()
                .map_err(|()| self.new_basic_error(BasicParseErrorKind::EndOfInput))?;
            self.input.cached_token = Some(CachedToken {
                token: new_token,
                start_position: token_start_position,
                end_state: self.input.tokenizer.state(),
            });
            self.input.cached_token_ref()
        };

        if let Some(block_type) = BlockType::opening(token) {
            self.at_start_of = Some(block_type);
        }
        Ok(token)
    }

    /// Have the given closure parse something, then check the the input is exhausted.
    /// The result is overridden to an `Err(..)` if some input remains.
    ///
    /// This can help tell e.g. `color: green;` from `color: green 4px;`
    #[inline]
    pub fn parse_entirely<F, T, E>(&mut self, parse: F) -> Result<T, ParseError<'i, E>>
    where
        F: FnOnce(&mut Parser<'i, 't>) -> Result<T, ParseError<'i, E>>,
    {
        let result = parse(self)?;
        self.expect_exhausted()?;
        Ok(result)
    }

    /// Parse a list of comma-separated values, all with the same syntax.
    ///
    /// The given closure is called repeatedly with a "delimited" parser
    /// (see the `Parser::parse_until_before` method) so that it can over
    /// consume the input past a comma at this block/function nesting level.
    ///
    /// Successful results are accumulated in a vector.
    ///
    /// This method returns an`Err(..)` the first time that a closure call does,
    /// or if a closure call leaves some input before the next comma or the end
    /// of the input.
    #[inline]
    pub fn parse_comma_separated<F, T, E>(
        &mut self,
        parse_one: F,
    ) -> Result<Vec<T>, ParseError<'i, E>>
    where
        F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        self.parse_comma_separated_internal(parse_one, /* ignore_errors = */ false)
    }

    /// Like `parse_comma_separated`, but ignores errors on unknown components,
    /// rather than erroring out in the whole list.
    ///
    /// Caller must deal with the fact that the resulting list might be empty,
    /// if there's no valid component on the list.
    #[inline]
    pub fn parse_comma_separated_ignoring_errors<F, T, E: 'i>(&mut self, parse_one: F) -> Vec<T>
    where
        F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        match self.parse_comma_separated_internal(parse_one, /* ignore_errors = */ true) {
            Ok(values) => values,
            Err(..) => unreachable!(),
        }
    }

    #[inline]
    fn parse_comma_separated_internal<F, T, E>(
        &mut self,
        mut parse_one: F,
        ignore_errors: bool,
    ) -> Result<Vec<T>, ParseError<'i, E>>
    where
        F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        // Vec grows from 0 to 4 by default on first push().  So allocate with
        // capacity 1, so in the somewhat common case of only one item we don't
        // way overallocate.  Note that we always push at least one item if
        // parsing succeeds.
        let mut values = Vec::with_capacity(1);
        loop {
            self.skip_whitespace(); // Unnecessary for correctness, but may help try() in parse_one rewind less.
            match self.parse_until_before(Delimiter::Comma, &mut parse_one) {
                Ok(v) => values.push(v),
                Err(e) if !ignore_errors => return Err(e),
                Err(_) => {}
            }
            match self.next() {
                Err(_) => return Ok(values),
                Ok(&Token::Comma) => continue,
                Ok(_) => unreachable!(),
            }
        }
    }

    /// Parse the content of a block or function.
    ///
    /// This method panics if the last token yielded by this parser
    /// (from one of the `next*` methods)
    /// is not a on that marks the start of a block or function:
    /// a `Function`, `ParenthesisBlock`, `CurlyBracketBlock`, or `SquareBracketBlock`.
    ///
    /// The given closure is called with a "delimited" parser
    /// that stops at the end of the block or function (at the matching closing token).
    ///
    /// The result is overridden to an `Err(..)` if the closure leaves some input before that point.
    #[inline]
    pub fn parse_nested_block<F, T, E>(&mut self, parse: F) -> Result<T, ParseError<'i, E>>
    where
        F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        parse_nested_block(self, parse)
    }

    /// Limit parsing to until a given delimiter or the end of the input. (E.g.
    /// a semicolon for a property value.)
    ///
    /// The given closure is called with a "delimited" parser
    /// that stops before the first character at this block/function nesting level
    /// that matches the given set of delimiters, or at the end of the input.
    ///
    /// The result is overridden to an `Err(..)` if the closure leaves some input before that point.
    #[inline]
    pub fn parse_until_before<F, T, E>(
        &mut self,
        delimiters: Delimiters,
        parse: F,
    ) -> Result<T, ParseError<'i, E>>
    where
        F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        parse_until_before(self, delimiters, ParseUntilErrorBehavior::Consume, parse)
    }

    /// Like `parse_until_before`, but also consume the delimiter token.
    ///
    /// This can be useful when you don’t need to know which delimiter it was
    /// (e.g. if these is only one in the given set)
    /// or if it was there at all (as opposed to reaching the end of the input).
    #[inline]
    pub fn parse_until_after<F, T, E>(
        &mut self,
        delimiters: Delimiters,
        parse: F,
    ) -> Result<T, ParseError<'i, E>>
    where
        F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        parse_until_after(self, delimiters, ParseUntilErrorBehavior::Consume, parse)
    }

    /// Parse a <whitespace-token> and return its value.
    #[inline]
    pub fn expect_whitespace(&mut self) -> Result<&'i str, BasicParseError<'i>> {
        let start_location = self.current_source_location();
        match *self.next_including_whitespace()? {
            Token::WhiteSpace(value) => Ok(value),
            ref t => Err(start_location.new_basic_unexpected_token_error(t.clone())),
        }
    }

    /// Parse a <ident-token> and return the unescaped value.
    #[inline]
    pub fn expect_ident(&mut self) -> Result<&CowRcStr<'i>, BasicParseError<'i>> {
        expect! {self,
            Token::Ident(ref value) => Ok(value),
        }
    }

    /// expect_ident, but clone the CowRcStr
    #[inline]
    pub fn expect_ident_cloned(&mut self) -> Result<CowRcStr<'i>, BasicParseError<'i>> {
        self.expect_ident().cloned()
    }

    /// Parse a <ident-token> whose unescaped value is an ASCII-insensitive match for the given value.
    #[inline]
    pub fn expect_ident_matching(
        &mut self,
        expected_value: &str,
    ) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::Ident(ref value) if value.eq_ignore_ascii_case(expected_value) => Ok(()),
        }
    }

    /// Parse a <string-token> and return the unescaped value.
    #[inline]
    pub fn expect_string(&mut self) -> Result<&CowRcStr<'i>, BasicParseError<'i>> {
        expect! {self,
            Token::QuotedString(ref value) => Ok(value),
        }
    }

    /// expect_string, but clone the CowRcStr
    #[inline]
    pub fn expect_string_cloned(&mut self) -> Result<CowRcStr<'i>, BasicParseError<'i>> {
        self.expect_string().cloned()
    }

    /// Parse either a <ident-token> or a <string-token>, and return the unescaped value.
    #[inline]
    pub fn expect_ident_or_string(&mut self) -> Result<&CowRcStr<'i>, BasicParseError<'i>> {
        expect! {self,
            Token::Ident(ref value) => Ok(value),
            Token::QuotedString(ref value) => Ok(value),
        }
    }

    /// Parse a <url-token> and return the unescaped value.
    #[inline]
    pub fn expect_url(&mut self) -> Result<CowRcStr<'i>, BasicParseError<'i>> {
        expect! {self,
            Token::UnquotedUrl(ref value) => Ok(value.clone()),
            Token::Function(ref name) if name.eq_ignore_ascii_case("url") => {
                self.parse_nested_block(|input| {
                    input.expect_string().map_err(Into::into).cloned()
                })
                .map_err(ParseError::<()>::basic)
            }
        }
    }

    /// Parse either a <url-token> or a <string-token>, and return the unescaped value.
    #[inline]
    pub fn expect_url_or_string(&mut self) -> Result<CowRcStr<'i>, BasicParseError<'i>> {
        expect! {self,
            Token::UnquotedUrl(ref value) => Ok(value.clone()),
            Token::QuotedString(ref value) => Ok(value.clone()),
            Token::Function(ref name) if name.eq_ignore_ascii_case("url") => {
                self.parse_nested_block(|input| {
                    input.expect_string().map_err(Into::into).cloned()
                })
                .map_err(ParseError::<()>::basic)
            }
        }
    }

    /// Parse a <number-token> and return the integer value.
    #[inline]
    pub fn expect_number(&mut self) -> Result<f32, BasicParseError<'i>> {
        expect! {self,
            Token::Number { value, .. } => Ok(value),
        }
    }

    /// Parse a <number-token> that does not have a fractional part, and return the integer value.
    #[inline]
    pub fn expect_integer(&mut self) -> Result<i32, BasicParseError<'i>> {
        expect! {self,
            Token::Number { int_value: Some(int_value), .. } => Ok(int_value),
        }
    }

    /// Parse a <percentage-token> and return the value.
    /// `0%` and `100%` map to `0.0` and `1.0` (not `100.0`), respectively.
    #[inline]
    pub fn expect_percentage(&mut self) -> Result<f32, BasicParseError<'i>> {
        expect! {self,
            Token::Percentage { unit_value, .. } => Ok(unit_value),
        }
    }

    /// Parse a `:` <colon-token>.
    #[inline]
    pub fn expect_colon(&mut self) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::Colon => Ok(()),
        }
    }

    /// Parse a `;` <semicolon-token>.
    #[inline]
    pub fn expect_semicolon(&mut self) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::Semicolon => Ok(()),
        }
    }

    /// Parse a `,` <comma-token>.
    #[inline]
    pub fn expect_comma(&mut self) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::Comma => Ok(()),
        }
    }

    /// Parse a <delim-token> with the given value.
    #[inline]
    pub fn expect_delim(&mut self, expected_value: char) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::Delim(value) if value == expected_value => Ok(()),
        }
    }

    /// Parse a `{ /* ... */ }` curly brackets block.
    ///
    /// If the result is `Ok`, you can then call the `Parser::parse_nested_block` method.
    #[inline]
    pub fn expect_curly_bracket_block(&mut self) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::CurlyBracketBlock => Ok(()),
        }
    }

    /// Parse a `[ /* ... */ ]` square brackets block.
    ///
    /// If the result is `Ok`, you can then call the `Parser::parse_nested_block` method.
    #[inline]
    pub fn expect_square_bracket_block(&mut self) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::SquareBracketBlock => Ok(()),
        }
    }

    /// Parse a `( /* ... */ )` parenthesis block.
    ///
    /// If the result is `Ok`, you can then call the `Parser::parse_nested_block` method.
    #[inline]
    pub fn expect_parenthesis_block(&mut self) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::ParenthesisBlock => Ok(()),
        }
    }

    /// Parse a <function> token and return its name.
    ///
    /// If the result is `Ok`, you can then call the `Parser::parse_nested_block` method.
    #[inline]
    pub fn expect_function(&mut self) -> Result<&CowRcStr<'i>, BasicParseError<'i>> {
        expect! {self,
            Token::Function(ref name) => Ok(name),
        }
    }

    /// Parse a <function> token whose name is an ASCII-insensitive match for the given value.
    ///
    /// If the result is `Ok`, you can then call the `Parser::parse_nested_block` method.
    #[inline]
    pub fn expect_function_matching(
        &mut self,
        expected_name: &str,
    ) -> Result<(), BasicParseError<'i>> {
        expect! {self,
            Token::Function(ref name) if name.eq_ignore_ascii_case(expected_name) => Ok(()),
        }
    }

    /// Parse the input until exhaustion and check that it contains no “error” token.
    ///
    /// See `Token::is_parse_error`. This also checks nested blocks and functions recursively.
    #[inline]
    pub fn expect_no_error_token(&mut self) -> Result<(), BasicParseError<'i>> {
        loop {
            match self.next_including_whitespace_and_comments() {
                Ok(&Token::Function(_))
                | Ok(&Token::ParenthesisBlock)
                | Ok(&Token::SquareBracketBlock)
                | Ok(&Token::CurlyBracketBlock) => self
                    .parse_nested_block(|input| input.expect_no_error_token().map_err(Into::into))
                    .map_err(ParseError::<()>::basic)?,
                Ok(t) => {
                    // FIXME: maybe these should be separate variants of
                    // BasicParseError instead?
                    if t.is_parse_error() {
                        let token = t.clone();
                        return Err(self.new_basic_unexpected_token_error(token));
                    }
                }
                Err(_) => return Ok(()),
            }
        }
    }
}

pub fn parse_until_before<'i: 't, 't, F, T, E>(
    parser: &mut Parser<'i, 't>,
    delimiters: Delimiters,
    error_behavior: ParseUntilErrorBehavior,
    parse: F,
) -> Result<T, ParseError<'i, E>>
where
    F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
{
    let delimiters = parser.stop_before | delimiters;
    let result;
    // Introduce a new scope to limit duration of nested_parser’s borrow
    {
        let mut delimited_parser = Parser {
            input: parser.input,
            at_start_of: parser.at_start_of.take(),
            stop_before: delimiters,
        };
        result = delimited_parser.parse_entirely(parse);
        if error_behavior == ParseUntilErrorBehavior::Stop && result.is_err() {
            return result;
        }
        if let Some(block_type) = delimited_parser.at_start_of {
            consume_until_end_of_block(block_type, &mut delimited_parser.input.tokenizer);
        }
    }
    // FIXME: have a special-purpose tokenizer method for this that does less work.
    loop {
        if delimiters.contains(Delimiters::from_byte(parser.input.tokenizer.next_byte())) {
            break;
        }
        if let Ok(token) = parser.input.tokenizer.next() {
            if let Some(block_type) = BlockType::opening(&token) {
                consume_until_end_of_block(block_type, &mut parser.input.tokenizer);
            }
        } else {
            break;
        }
    }
    result
}

pub fn parse_until_after<'i: 't, 't, F, T, E>(
    parser: &mut Parser<'i, 't>,
    delimiters: Delimiters,
    error_behavior: ParseUntilErrorBehavior,
    parse: F,
) -> Result<T, ParseError<'i, E>>
where
    F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
{
    let result = parse_until_before(parser, delimiters, error_behavior, parse);
    if error_behavior == ParseUntilErrorBehavior::Stop && result.is_err() {
        return result;
    }
    let next_byte = parser.input.tokenizer.next_byte();
    if next_byte.is_some()
        && !parser
            .stop_before
            .contains(Delimiters::from_byte(next_byte))
    {
        debug_assert!(delimiters.contains(Delimiters::from_byte(next_byte)));
        // We know this byte is ASCII.
        parser.input.tokenizer.advance(1);
        if next_byte == Some(b'{') {
            consume_until_end_of_block(BlockType::CurlyBracket, &mut parser.input.tokenizer);
        }
    }
    result
}

pub fn parse_nested_block<'i: 't, 't, F, T, E>(
    parser: &mut Parser<'i, 't>,
    parse: F,
) -> Result<T, ParseError<'i, E>>
where
    F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
{
    let block_type = parser.at_start_of.take().expect(
        "\
         A nested parser can only be created when a Function, \
         ParenthesisBlock, SquareBracketBlock, or CurlyBracketBlock \
         token was just consumed.\
         ",
    );
    let closing_delimiter = match block_type {
        BlockType::CurlyBracket => ClosingDelimiter::CloseCurlyBracket,
        BlockType::SquareBracket => ClosingDelimiter::CloseSquareBracket,
        BlockType::Parenthesis => ClosingDelimiter::CloseParenthesis,
    };
    let result;
    // Introduce a new scope to limit duration of nested_parser’s borrow
    {
        let mut nested_parser = Parser {
            input: parser.input,
            at_start_of: None,
            stop_before: closing_delimiter,
        };
        result = nested_parser.parse_entirely(parse);
        if let Some(block_type) = nested_parser.at_start_of {
            consume_until_end_of_block(block_type, &mut nested_parser.input.tokenizer);
        }
    }
    consume_until_end_of_block(block_type, &mut parser.input.tokenizer);
    result
}

#[inline(never)]
#[cold]
fn consume_until_end_of_block(block_type: BlockType, tokenizer: &mut Tokenizer) {
    let mut stack = SmallVec::<[BlockType; 16]>::new();
    stack.push(block_type);

    // FIXME: have a special-purpose tokenizer method for this that does less work.
    while let Ok(ref token) = tokenizer.next() {
        if let Some(b) = BlockType::closing(token) {
            if *stack.last().unwrap() == b {
                stack.pop();
                if stack.is_empty() {
                    return;
                }
            }
        }

        if let Some(block_type) = BlockType::opening(token) {
            stack.push(block_type);
        }
    }
}
