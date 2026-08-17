use std::borrow::Cow;
use std::fmt::Display;
use std::sync::Arc;
use utf8_chars::BufReadCharsExt;

#[derive(Clone, Debug)]
pub(crate) enum TokenEndReason {
    /// End of input was reached.
    EndOfInput,
    /// An unescaped newline char was reached.
    UnescapedNewLine,
    /// Specified terminating char.
    SpecifiedTerminatingChar,
    /// A non-newline blank char was reached.
    NonNewLineBlank,
    /// A here-document's body is starting.
    HereDocumentBodyStart,
    /// A here-document's body was terminated.
    HereDocumentBodyEnd,
    /// A here-document's end tag was reached.
    HereDocumentEndTag,
    /// An operator was started.
    OperatorStart,
    /// An operator was terminated.
    OperatorEnd,
    /// Some other condition was reached.
    Other,
}

/// Represents a position in a source shell script.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "fuzz-testing", derive(arbitrary::Arbitrary))]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
#[cfg_attr(test, serde(rename = "Pos"))]
pub struct SourcePosition {
    /// The 0-based index of the character in the input stream.
    #[cfg_attr(test, serde(rename = "idx"))]
    pub index: usize,
    /// The 1-based line number.
    pub line: usize,
    /// The 1-based column number.
    #[cfg_attr(test, serde(rename = "col"))]
    pub column: usize,
}

impl Display for SourcePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("line {} col {}", self.line, self.column))
    }
}

#[cfg(feature = "diagnostics")]
impl From<&SourcePosition> for miette::SourceOffset {
    #[allow(clippy::cast_sign_loss)]
    fn from(position: &SourcePosition) -> Self {
        position.index.into()
    }
}

/// Represents the location of a token in its source shell script.
#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "fuzz-testing", derive(arbitrary::Arbitrary))]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
#[cfg_attr(test, serde(rename = "Loc"))]
pub struct TokenLocation {
    /// The start position of the token.
    pub start: Arc<SourcePosition>,
    /// The end position of the token (exclusive).
    pub end: Arc<SourcePosition>,
}

impl TokenLocation {
    /// Returns the length of the token in characters.
    pub fn length(&self) -> usize {
        self.end.index - self.start.index
    }
    pub(crate) fn within(start: &Self, end: &Self) -> Self {
        Self {
            start: start.start.clone(),
            end: end.end.clone(),
        }
    }
}

/// Represents a token extracted from a shell script.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fuzz-testing", derive(arbitrary::Arbitrary))]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum Token {
    /// An operator token.
    #[cfg_attr(test, serde(rename = "Op"))]
    Operator(String, TokenLocation),
    /// A word token.
    #[cfg_attr(test, serde(rename = "W"))]
    Word(String, TokenLocation),
}

impl Token {
    /// Returns the string value of the token.
    pub fn to_str(&self) -> &str {
        match self {
            Self::Operator(s, _) => s,
            Self::Word(s, _) => s,
        }
    }

    /// Returns the location of the token in the source script.
    pub const fn location(&self) -> &TokenLocation {
        match self {
            Self::Operator(_, l) => l,
            Self::Word(_, l) => l,
        }
    }
}

#[cfg(feature = "diagnostics")]
impl From<&Token> for miette::SourceSpan {
    fn from(token: &Token) -> Self {
        let start = token.location().start.as_ref();
        Self::new(start.into(), token.location().length())
    }
}

/// Encapsulates the result of tokenizing a shell script.
#[derive(Clone, Debug)]
pub(crate) struct TokenizeResult {
    /// Reason for tokenization ending.
    pub reason: TokenEndReason,
    /// The token that was extracted, if any.
    pub token: Option<Token>,
}

/// Represents an error that occurred during tokenization.
#[derive(thiserror::Error, Debug)]
pub enum TokenizerError {
    /// An unterminated escape sequence was encountered at the end of the input stream.
    #[error("unterminated escape sequence")]
    UnterminatedEscapeSequence,

    /// An unterminated single-quoted substring was encountered at the end of the input stream.
    #[error("unterminated single quote at {0}")]
    UnterminatedSingleQuote(SourcePosition),

    /// An unterminated ANSI C-quoted substring was encountered at the end of the input stream.
    #[error("unterminated ANSI C quote at {0}")]
    UnterminatedAnsiCQuote(SourcePosition),

    /// An unterminated double-quoted substring was encountered at the end of the input stream.
    #[error("unterminated double quote at {0}")]
    UnterminatedDoubleQuote(SourcePosition),

    /// An unterminated back-quoted substring was encountered at the end of the input stream.
    #[error("unterminated backquote near {0}")]
    UnterminatedBackquote(SourcePosition),

    /// An unterminated extended glob (extglob) pattern was encountered at the end of the input
    /// stream.
    #[error("unterminated extglob near {0}")]
    UnterminatedExtendedGlob(SourcePosition),

    /// An unterminated variable expression was encountered at the end of the input stream.
    #[error("unterminated variable expression")]
    UnterminatedVariable,

    /// An unterminated command substitiion was encountered at the end of the input stream.
    #[error("unterminated command substitution")]
    UnterminatedCommandSubstitution,

    /// An error occurred decoding UTF-8 characters in the input stream.
    #[error("failed to decode UTF-8 characters")]
    FailedDecoding,

    /// An I/O here tag was missing.
    #[error("missing here tag for here document body")]
    MissingHereTagForDocumentBody,

    /// The indicated I/O here tag was missing.
    #[error("missing here tag '{0}'")]
    MissingHereTag(String),

    /// An unterminated here document sequence was encountered at the end of the input stream.
    #[error("unterminated here document sequence; tag(s) [{0}] found at: [{1}]")]
    UnterminatedHereDocuments(String, String),

    /// An I/O error occurred while reading from the input stream.
    #[error("failed to read input")]
    ReadError(#[from] std::io::Error),
}

impl TokenizerError {
    /// Returns true if the error represents an error that could possibly be due
    /// to an incomplete input stream.
    pub const fn is_incomplete(&self) -> bool {
        matches!(
            self,
            Self::UnterminatedEscapeSequence
                | Self::UnterminatedAnsiCQuote(..)
                | Self::UnterminatedSingleQuote(..)
                | Self::UnterminatedDoubleQuote(..)
                | Self::UnterminatedBackquote(..)
                | Self::UnterminatedCommandSubstitution
                | Self::UnterminatedVariable
                | Self::UnterminatedExtendedGlob(..)
                | Self::UnterminatedHereDocuments(..)
        )
    }
}

/// Encapsulates a sequence of tokens.
#[derive(Debug)]
pub(crate) struct Tokens<'a> {
    /// Sequence of tokens.
    pub tokens: &'a [Token],
}

#[derive(Clone, Debug)]
enum QuoteMode {
    None,
    AnsiC(SourcePosition),
    Single(SourcePosition),
    Double(SourcePosition),
}

#[derive(Clone, Debug, Default)]
enum HereState {
    /// In this state, we are not currently tracking any here-documents.
    #[default]
    None,
    /// In this state, we expect that the next token will be a here tag.
    NextTokenIsHereTag { remove_tabs: bool },
    /// In this state, the *current* token is a here tag.
    CurrentTokenIsHereTag {
        remove_tabs: bool,
        operator_token_result: TokenizeResult,
    },
    /// In this state, we expect that the *next line* will be the body of
    /// a here-document.
    NextLineIsHereDoc,
    /// In this state, we are in the set of lines that comprise 1 or more
    /// consecutive here-document bodies.
    InHereDocs,
}

#[derive(Clone, Debug)]
struct HereTag {
    tag: String,
    tag_was_escaped_or_quoted: bool,
    remove_tabs: bool,
    position: SourcePosition,
    tokens: Vec<TokenizeResult>,
    pending_tokens_after: Vec<TokenizeResult>,
}

#[derive(Clone, Debug)]
struct CrossTokenParseState {
    /// Cursor within the overall token stream; used for error reporting.
    cursor: SourcePosition,
    /// Current state of parsing here-documents.
    here_state: HereState,
    /// Ordered queue of here tags for which we're still looking for matching here-document bodies.
    current_here_tags: Vec<HereTag>,
    /// Tokens already tokenized that should be used first to serve requests for tokens.
    queued_tokens: Vec<TokenizeResult>,
    /// Are we in an arithmetic expansion?
    arithmetic_expansion: bool,
}

/// Options controlling how the tokenizer operates.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct TokenizerOptions {
    /// Whether or not to enable extended globbing patterns (extglob).
    pub enable_extended_globbing: bool,
    /// Whether or not to operate in POSIX compliance mode.
    pub posix_mode: bool,
    /// Whether or not we're running in SH emulation mode.
    pub sh_mode: bool,
}

impl Default for TokenizerOptions {
    fn default() -> Self {
        Self {
            enable_extended_globbing: true,
            posix_mode: false,
            sh_mode: false,
        }
    }
}

/// A tokenizer for shell scripts.
pub(crate) struct Tokenizer<'a, R: ?Sized + std::io::BufRead> {
    char_reader: std::iter::Peekable<utf8_chars::Chars<'a, R>>,
    cross_state: CrossTokenParseState,
    options: TokenizerOptions,
}

/// Encapsulates the current token parsing state.
#[derive(Clone, Debug)]
struct TokenParseState {
    pub start_position: SourcePosition,
    pub token_so_far: String,
    pub token_is_operator: bool,
    pub in_escape: bool,
    pub quote_mode: QuoteMode,
}

impl TokenParseState {
    pub fn new(start_position: &SourcePosition) -> Self {
        Self {
            start_position: start_position.to_owned(),
            token_so_far: String::new(),
            token_is_operator: false,
            in_escape: false,
            quote_mode: QuoteMode::None,
        }
    }

    pub fn pop(&mut self, end_position: &SourcePosition) -> Token {
        let end = Arc::new(end_position.to_owned());
        let token_location = TokenLocation {
            start: Arc::new(std::mem::take(&mut self.start_position)),
            end,
        };

        let token = if std::mem::take(&mut self.token_is_operator) {
            Token::Operator(std::mem::take(&mut self.token_so_far), token_location)
        } else {
            Token::Word(std::mem::take(&mut self.token_so_far), token_location)
        };

        end_position.clone_into(&mut self.start_position);
        self.in_escape = false;
        self.quote_mode = QuoteMode::None;

        token
    }

    pub const fn started_token(&self) -> bool {
        !self.token_so_far.is_empty()
    }

    pub fn append_char(&mut self, c: char) {
        self.token_so_far.push(c);
    }

    pub fn append_str(&mut self, s: &str) {
        self.token_so_far.push_str(s);
    }

    pub const fn unquoted(&self) -> bool {
        !self.in_escape && matches!(self.quote_mode, QuoteMode::None)
    }

    pub fn current_token(&self) -> &str {
        &self.token_so_far
    }

    pub fn is_specific_operator(&self, operator: &str) -> bool {
        self.token_is_operator && self.current_token() == operator
    }

    pub const fn in_operator(&self) -> bool {
        self.token_is_operator
    }

    fn is_newline(&self) -> bool {
        self.token_so_far == "\n"
    }

    fn replace_with_here_doc(&mut self, s: String) {
        self.token_so_far = s;
    }

    pub fn delimit_current_token(
        &mut self,
        reason: TokenEndReason,
        cross_token_state: &mut CrossTokenParseState,
    ) -> Result<Option<TokenizeResult>, TokenizerError> {
        // If we don't have anything in the token, then don't yield an empty string token
        // *unless* it's the body of a here document.
        if !self.started_token() && !matches!(reason, TokenEndReason::HereDocumentBodyEnd) {
            return Ok(Some(TokenizeResult {
                reason,
                token: None,
            }));
        }

        // TODO: Make sure the here-tag meets criteria (and isn't a newline).
        let current_here_state = std::mem::take(&mut cross_token_state.here_state);
        match current_here_state {
            HereState::NextTokenIsHereTag { remove_tabs } => {
                // Don't yield the operator as a token yet. We need to make sure we collect
                // up everything we need for all the here-documents with tags on this line.
                let operator_token_result = TokenizeResult {
                    reason,
                    token: Some(self.pop(&cross_token_state.cursor)),
                };

                cross_token_state.here_state = HereState::CurrentTokenIsHereTag {
                    remove_tabs,
                    operator_token_result,
                };

                return Ok(None);
            }
            HereState::CurrentTokenIsHereTag {
                remove_tabs,
                operator_token_result,
            } => {
                if self.is_newline() {
                    return Err(TokenizerError::MissingHereTag(
                        self.current_token().to_owned(),
                    ));
                }

                cross_token_state.here_state = HereState::NextLineIsHereDoc;

                // Include the trailing \n in the here tag so it's easier to check against.
                let tag = std::format!("{}\n", self.current_token().trim_ascii_start());
                let tag_was_escaped_or_quoted = tag.contains(is_quoting_char);

                let tag_token_result = TokenizeResult {
                    reason,
                    token: Some(self.pop(&cross_token_state.cursor)),
                };

                cross_token_state.current_here_tags.push(HereTag {
                    tag,
                    tag_was_escaped_or_quoted,
                    remove_tabs,
                    position: cross_token_state.cursor.clone(),
                    tokens: vec![operator_token_result, tag_token_result],
                    pending_tokens_after: vec![],
                });

                return Ok(None);
            }
            HereState::NextLineIsHereDoc => {
                if self.is_newline() {
                    cross_token_state.here_state = HereState::InHereDocs;
                } else {
                    cross_token_state.here_state = HereState::NextLineIsHereDoc;
                }

                if let Some(last_here_tag) = cross_token_state.current_here_tags.last_mut() {
                    let token = self.pop(&cross_token_state.cursor);
                    let result = TokenizeResult {
                        reason,
                        token: Some(token),
                    };

                    last_here_tag.pending_tokens_after.push(result);
                } else {
                    return Err(TokenizerError::MissingHereTagForDocumentBody);
                }

                return Ok(None);
            }
            HereState::InHereDocs => {
                // We hit the end of the current here-document.
                let completed_here_tag = cross_token_state.current_here_tags.remove(0);

                // First queue the redirection operator and (start) here-tag.
                for here_token in completed_here_tag.tokens {
                    cross_token_state.queued_tokens.push(here_token);
                }

                // Leave a hint that we are about to start a here-document.
                cross_token_state.queued_tokens.push(TokenizeResult {
                    reason: TokenEndReason::HereDocumentBodyStart,
                    token: None,
                });

                // Then queue the body document we just finished.
                cross_token_state.queued_tokens.push(TokenizeResult {
                    reason,
                    token: Some(self.pop(&cross_token_state.cursor)),
                });

                // Then queue up the (end) here-tag.
                self.append_str(completed_here_tag.tag.trim_end_matches('\n'));
                cross_token_state.queued_tokens.push(TokenizeResult {
                    reason: TokenEndReason::HereDocumentEndTag,
                    token: Some(self.pop(&cross_token_state.cursor)),
                });

                // Now we're ready to queue up any tokens that came between the completed
                // here tag and the next here tag (or newline after it if it was the last).
                for pending_token in completed_here_tag.pending_tokens_after {
                    cross_token_state.queued_tokens.push(pending_token);
                }

                if cross_token_state.current_here_tags.is_empty() {
                    cross_token_state.here_state = HereState::None;
                } else {
                    cross_token_state.here_state = HereState::InHereDocs;
                }

                return Ok(None);
            }
            HereState::None => (),
        }

        let token = self.pop(&cross_token_state.cursor);
        let result = TokenizeResult {
            reason,
            token: Some(token),
        };

        Ok(Some(result))
    }
}

/// Break the given input shell script string into tokens, returning the tokens.
///
/// # Arguments
///
/// * `input` - The shell script to tokenize.
pub fn tokenize_str(input: &str) -> Result<Vec<Token>, TokenizerError> {
    tokenize_str_with_options(input, &TokenizerOptions::default())
}

/// Break the given input shell script string into tokens, returning the tokens.
///
/// # Arguments
///
/// * `input` - The shell script to tokenize.
/// * `options` - Options controlling how the tokenizer operates.
pub fn tokenize_str_with_options(
    input: &str,
    options: &TokenizerOptions,
) -> Result<Vec<Token>, TokenizerError> {
    uncached_tokenize_string(input.to_owned(), options.to_owned())
}

#[cached::proc_macro::cached(name = "TOKENIZE_CACHE", size = 64, result = true)]
fn uncached_tokenize_string(
    input: String,
    options: TokenizerOptions,
) -> Result<Vec<Token>, TokenizerError> {
    uncached_tokenize_str(input.as_str(), &options)
}

/// Break the given input shell script string into tokens, returning the tokens.
/// No caching is performed.
///
/// # Arguments
///
/// * `input` - The shell script to tokenize.
pub fn uncached_tokenize_str(
    input: &str,
    options: &TokenizerOptions,
) -> Result<Vec<Token>, TokenizerError> {
    let mut reader = std::io::BufReader::new(input.as_bytes());
    let mut tokenizer = crate::tokenizer::Tokenizer::new(&mut reader, options);

    let mut tokens = vec![];
    loop {
        match tokenizer.next_token()? {
            TokenizeResult {
                token: Some(token), ..
            } => tokens.push(token),
            TokenizeResult {
                reason: TokenEndReason::EndOfInput,
                ..
            } => break,
            _ => (),
        }
    }

    Ok(tokens)
}

impl<'a, R: ?Sized + std::io::BufRead> Tokenizer<'a, R> {
    pub fn new(reader: &'a mut R, options: &TokenizerOptions) -> Self {
        Tokenizer {
            options: options.clone(),
            char_reader: reader.chars().peekable(),
            cross_state: CrossTokenParseState {
                cursor: SourcePosition {
                    index: 0,
                    line: 1,
                    column: 1,
                },
                here_state: HereState::None,
                current_here_tags: vec![],
                queued_tokens: vec![],
                arithmetic_expansion: false,
            },
        }
    }

    #[expect(clippy::unnecessary_wraps)]
    pub fn current_location(&self) -> Option<SourcePosition> {
        Some(self.cross_state.cursor.clone())
    }

    fn next_char(&mut self) -> Result<Option<char>, TokenizerError> {
        let c = self
            .char_reader
            .next()
            .transpose()
            .map_err(TokenizerError::ReadError)?;

        if let Some(ch) = c {
            if ch == '\n' {
                self.cross_state.cursor.line += 1;
                self.cross_state.cursor.column = 1;
            } else {
                self.cross_state.cursor.column += 1;
            }
            self.cross_state.cursor.index += 1;
        }

        Ok(c)
    }

    fn consume_char(&mut self) -> Result<(), TokenizerError> {
        let _ = self.next_char()?;
        Ok(())
    }

    fn peek_char(&mut self) -> Result<Option<char>, TokenizerError> {
        match self.char_reader.peek() {
            Some(result) => match result {
                Ok(c) => Ok(Some(*c)),
                Err(_) => Err(TokenizerError::FailedDecoding),
            },
            None => Ok(None),
        }
    }

    pub fn next_token(&mut self) -> Result<TokenizeResult, TokenizerError> {
        self.next_token_until(None, false /* include space? */)
    }

    /// Returns the next token from the input stream, optionally stopping early when a specified
    /// terminating character is encountered.
    ///
    /// # Arguments
    ///
    /// * `terminating_char` - An optional character that, if encountered, will stop the
    ///   tokenization process and return the token up to that character.
    /// * `include_space` - If true, include spaces in the tokenization process. This is not
    ///   typically the case, but can be helpful when needing to preserve the original source text
    ///   embedded within a command substitution or similar construct.
    #[expect(clippy::cognitive_complexity)]
    #[expect(clippy::if_same_then_else)]
    #[expect(clippy::panic_in_result_fn)]
    #[expect(clippy::too_many_lines)]
    #[allow(clippy::unwrap_in_result)]
    fn next_token_until(
        &mut self,
        terminating_char: Option<char>,
        include_space: bool,
    ) -> Result<TokenizeResult, TokenizerError> {
        let mut state = TokenParseState::new(&self.cross_state.cursor);
        let mut result: Option<TokenizeResult> = None;

        while result.is_none() {
            // First satisfy token results from our queue. Once we exhaust the queue then
            // we'll look at the input stream.
            if !self.cross_state.queued_tokens.is_empty() {
                return Ok(self.cross_state.queued_tokens.remove(0));
            }

            let next = self.peek_char()?;
            let c = next.unwrap_or('\0');

            // When we hit the end of the input, then we're done with the current token (if there is
            // one).
            if next.is_none() {
                // TODO: Verify we're not waiting on some terminating character?
                // Verify we're out of all quotes.
                if state.in_escape {
                    return Err(TokenizerError::UnterminatedEscapeSequence);
                }
                match state.quote_mode {
                    QuoteMode::None => (),
                    QuoteMode::AnsiC(pos) => {
                        return Err(TokenizerError::UnterminatedAnsiCQuote(pos));
                    }
                    QuoteMode::Single(pos) => {
                        return Err(TokenizerError::UnterminatedSingleQuote(pos));
                    }
                    QuoteMode::Double(pos) => {
                        return Err(TokenizerError::UnterminatedDoubleQuote(pos));
                    }
                }

                // Verify we're not in a here document.
                if !matches!(self.cross_state.here_state, HereState::None) {
                    if self.remove_here_end_tag(&mut state, &mut result, false)? {
                        // If we hit end tag without a trailing newline, try to get next token.
                        continue;
                    }

                    let tag_names = self
                        .cross_state
                        .current_here_tags
                        .iter()
                        .map(|tag| tag.tag.trim())
                        .collect::<Vec<_>>()
                        .join(", ");
                    let tag_positions = self
                        .cross_state
                        .current_here_tags
                        .iter()
                        .map(|tag| std::format!("{}", tag.position))
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(TokenizerError::UnterminatedHereDocuments(
                        tag_names,
                        tag_positions,
                    ));
                }

                result = state
                    .delimit_current_token(TokenEndReason::EndOfInput, &mut self.cross_state)?;
            //
            // Look for the specially specified terminating char.
            //
            } else if state.unquoted() && terminating_char == Some(c) {
                result = state.delimit_current_token(
                    TokenEndReason::SpecifiedTerminatingChar,
                    &mut self.cross_state,
                )?;
            //
            // Handle being in a here document.
            //
            } else if matches!(self.cross_state.here_state, HereState::InHereDocs) {
                //
                // For now, just include the character in the current token. We also check
                // if there are leading tabs to be removed.
                //
                if !self.cross_state.current_here_tags.is_empty()
                    && self.cross_state.current_here_tags[0].remove_tabs
                    && (!state.started_token() || state.current_token().ends_with('\n'))
                    && c == '\t'
                {
                    // Consume it but don't include it.
                    self.consume_char()?;
                } else {
                    self.consume_char()?;
                    state.append_char(c);

                    // See if this was a newline character following the terminating here tag.
                    if c == '\n' {
                        self.remove_here_end_tag(&mut state, &mut result, true)?;
                    }
                }
            } else if state.in_operator() {
                //
                // We're in an operator. See if this character continues an operator, or if it
                // must be a separate token (because it wouldn't make a prefix of an operator).
                //

                let mut hypothetical_token = state.current_token().to_owned();
                hypothetical_token.push(c);

                if state.unquoted() && self.is_operator(hypothetical_token.as_ref()) {
                    self.consume_char()?;
                    state.append_char(c);
                } else {
                    assert!(state.started_token());

                    //
                    // N.B. If the completed operator indicates a here-document, then keep
                    // track that the *next* token should be the here-tag.
                    //
                    if self.cross_state.arithmetic_expansion {
                        //
                        // We're in an arithmetic context; don't consider << and <<-
                        // special. They're not here-docs, they're either a left-shift
                        // operator or a left-shift operator followed by a unary
                        // minus operator.
                        //

                        if state.is_specific_operator(")") && c == ')' {
                            self.cross_state.arithmetic_expansion = false;
                        }
                    } else if state.is_specific_operator("<<") {
                        self.cross_state.here_state =
                            HereState::NextTokenIsHereTag { remove_tabs: false };
                    } else if state.is_specific_operator("<<-") {
                        self.cross_state.here_state =
                            HereState::NextTokenIsHereTag { remove_tabs: true };
                    } else if state.is_specific_operator("(") && c == '(' {
                        self.cross_state.arithmetic_expansion = true;
                    }

                    let reason = if state.current_token() == "\n" {
                        TokenEndReason::UnescapedNewLine
                    } else {
                        TokenEndReason::OperatorEnd
                    };

                    result = state.delimit_current_token(reason, &mut self.cross_state)?;
                }
            //
            // See if this is a character that changes the current escaping/quoting state.
            //
            } else if does_char_newly_affect_quoting(&state, c) {
                if c == '\\' {
                    // Consume the backslash ourselves so we can peek past it.
                    self.consume_char()?;

                    if matches!(self.peek_char()?, Some('\n')) {
                        // Make sure the newline char gets consumed too.
                        self.consume_char()?;

                        // Make sure to include neither the backslash nor the newline character.
                    } else {
                        state.in_escape = true;
                        state.append_char(c);
                    }
                } else if c == '\'' {
                    if state.token_so_far.ends_with('$') {
                        state.quote_mode = QuoteMode::AnsiC(self.cross_state.cursor.clone());
                    } else {
                        state.quote_mode = QuoteMode::Single(self.cross_state.cursor.clone());
                    }

                    self.consume_char()?;
                    state.append_char(c);
                } else if c == '\"' {
                    state.quote_mode = QuoteMode::Double(self.cross_state.cursor.clone());
                    self.consume_char()?;
                    state.append_char(c);
                }
            }
            //
            // Handle end of single-quote, double-quote, or ANSI-C quote.
            else if !state.in_escape
                && matches!(
                    state.quote_mode,
                    QuoteMode::Single(..) | QuoteMode::AnsiC(..)
                )
                && c == '\''
            {
                state.quote_mode = QuoteMode::None;
                self.consume_char()?;
                state.append_char(c);
            } else if !state.in_escape
                && matches!(state.quote_mode, QuoteMode::Double(..))
                && c == '\"'
            {
                state.quote_mode = QuoteMode::None;
                self.consume_char()?;
                state.append_char(c);
            }
            //
            // Handle end of escape sequence.
            // TODO: Handle double-quote specific escape sequences.
            else if state.in_escape {
                state.in_escape = false;
                self.consume_char()?;
                state.append_char(c);
            } else if (state.unquoted()
                || (matches!(state.quote_mode, QuoteMode::Double(_)) && !state.in_escape))
                && (c == '$' || c == '`')
            {
                // TODO: handle quoted $ or ` in a double quote
                if c == '$' {
                    // Consume the '$' so we can peek beyond.
                    self.consume_char()?;

                    // Now peek beyond to see what we have.
                    let char_after_dollar_sign = self.peek_char()?;
                    match char_after_dollar_sign {
                        Some('(') => {
                            // Add the '$' we already consumed to the token.
                            state.append_char('$');

                            // Consume the '(' and add it to the token.
                            state.append_char(self.next_char()?.unwrap());

                            // Check to see if this is possibly an arithmetic expression
                            // (i.e., one that starts with `$((`).
                            let mut required_end_parens = 1;
                            if matches!(self.peek_char()?, Some('(')) {
                                // Consume the second '(' and add it to the token.
                                state.append_char(self.next_char()?.unwrap());
                                // Keep track that we'll need to see *2* end parentheses
                                // to leave this construct.
                                required_end_parens = 2;
                                // Keep track that we're in an arithmetic expression, since
                                // some text will be interpreted differently as a result
                                // (e.g., << is a left shift operator and not a here doc
                                // input redirection operator).
                                self.cross_state.arithmetic_expansion = true;
                            }

                            let mut pending_here_doc_tokens = vec![];
                            let mut drain_here_doc_tokens = false;

                            loop {
                                let cur_token = if drain_here_doc_tokens
                                    && !pending_here_doc_tokens.is_empty()
                                {
                                    if pending_here_doc_tokens.len() == 1 {
                                        drain_here_doc_tokens = false;
                                    }

                                    pending_here_doc_tokens.remove(0)
                                } else {
                                    let cur_token = self.next_token_until(
                                        Some(')'),
                                        true, /* include space? */
                                    )?;

                                    // See if this is a here-document-related token we need to hold
                                    // onto until after we've seen all the tokens that need to show
                                    // up before we get to the body.
                                    if matches!(
                                        cur_token.reason,
                                        TokenEndReason::HereDocumentBodyStart
                                            | TokenEndReason::HereDocumentBodyEnd
                                            | TokenEndReason::HereDocumentEndTag
                                    ) {
                                        pending_here_doc_tokens.push(cur_token);
                                        continue;
                                    }

                                    cur_token
                                };

                                if matches!(cur_token.reason, TokenEndReason::UnescapedNewLine)
                                    && !pending_here_doc_tokens.is_empty()
                                {
                                    pending_here_doc_tokens.push(cur_token);
                                    drain_here_doc_tokens = true;
                                    continue;
                                }

                                if let Some(cur_token_value) = cur_token.token {
                                    state.append_str(cur_token_value.to_str());

                                    // If we encounter an embedded open parenthesis, then note that
                                    // we'll have to see the matching end to it before we worry
                                    // about the end of the
                                    // containing construct.
                                    if matches!(cur_token_value, Token::Operator(o, _) if o == "(")
                                    {
                                        required_end_parens += 1;
                                    }
                                }

                                match cur_token.reason {
                                    TokenEndReason::HereDocumentBodyStart => {
                                        state.append_char('\n');
                                    }
                                    TokenEndReason::NonNewLineBlank => state.append_char(' '),
                                    TokenEndReason::SpecifiedTerminatingChar => {
                                        // We hit the ')' we were looking for. If this is the last
                                        // end parenthesis we needed to find, then we'll exit the
                                        // loop and consume
                                        // and append it.
                                        required_end_parens -= 1;
                                        if required_end_parens == 0 {
                                            break;
                                        }

                                        // This wasn't the *last* end parenthesis char, so let's
                                        // consume and append it here before we loop around again.
                                        state.append_char(self.next_char()?.unwrap());
                                    }
                                    TokenEndReason::EndOfInput => {
                                        return Err(
                                            TokenizerError::UnterminatedCommandSubstitution,
                                        );
                                    }
                                    _ => (),
                                }
                            }

                            self.cross_state.arithmetic_expansion = false;

                            state.append_char(self.next_char()?.unwrap());
                        }

                        Some('{') => {
                            // Add the '$' we already consumed to the token.
                            state.append_char('$');

                            // Consume the '{' and add it to the token.
                            state.append_char(self.next_char()?.unwrap());

                            let mut pending_here_doc_tokens = vec![];
                            let mut drain_here_doc_tokens = false;

                            loop {
                                let cur_token = if drain_here_doc_tokens
                                    && !pending_here_doc_tokens.is_empty()
                                {
                                    if pending_here_doc_tokens.len() == 1 {
                                        drain_here_doc_tokens = false;
                                    }

                                    pending_here_doc_tokens.remove(0)
                                } else {
                                    let cur_token = self.next_token_until(
                                        Some('}'),
                                        false, /* include space? */
                                    )?;

                                    // See if this is a here-document-related token we need to hold
                                    // onto until after we've seen all the tokens that need to show
                                    // up before we get to the body.
                                    if matches!(
                                        cur_token.reason,
                                        TokenEndReason::HereDocumentBodyStart
                                            | TokenEndReason::HereDocumentBodyEnd
                                            | TokenEndReason::HereDocumentEndTag
                                    ) {
                                        pending_here_doc_tokens.push(cur_token);
                                        continue;
                                    }

                                    cur_token
                                };

                                if matches!(cur_token.reason, TokenEndReason::UnescapedNewLine)
                                    && !pending_here_doc_tokens.is_empty()
                                {
                                    pending_here_doc_tokens.push(cur_token);
                                    drain_here_doc_tokens = true;
                                    continue;
                                }

                                if let Some(cur_token_value) = cur_token.token {
                                    state.append_str(cur_token_value.to_str());
                                }

                                match cur_token.reason {
                                    TokenEndReason::HereDocumentBodyStart => {
                                        state.append_char('\n');
                                    }
                                    TokenEndReason::NonNewLineBlank => state.append_char(' '),
                                    TokenEndReason::SpecifiedTerminatingChar => {
                                        // We hit the end brace we were looking for but did not
                                        // yet consume it. Do so now.
                                        state.append_char(self.next_char()?.unwrap());
                                        break;
                                    }
                                    TokenEndReason::EndOfInput => {
                                        return Err(TokenizerError::UnterminatedVariable);
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => {
                            // This is either a different character, or else the end of the string.
                            // Either way, add the '$' we already consumed to the token.
                            state.append_char('$');
                        }
                    }
                } else {
                    // We look for the terminating backquote. First disable normal consumption and
                    // consume the starting backquote.
                    let backquote_pos = self.cross_state.cursor.clone();
                    self.consume_char()?;

                    // Add the opening backquote to the token.
                    state.append_char(c);

                    // Now continue until we see an unescaped backquote.
                    let mut escaping_enabled = false;
                    let mut done = false;
                    while !done {
                        // Read (and consume) the next char.
                        let next_char_in_backquote = self.next_char()?;
                        if let Some(cib) = next_char_in_backquote {
                            // Include it in the token no matter what.
                            state.append_char(cib);

                            // Watch out for escaping.
                            if !escaping_enabled && cib == '\\' {
                                escaping_enabled = true;
                            } else {
                                // Look for an unescaped backquote to terminate.
                                if !escaping_enabled && cib == '`' {
                                    done = true;
                                }
                                escaping_enabled = false;
                            }
                        } else {
                            return Err(TokenizerError::UnterminatedBackquote(backquote_pos));
                        }
                    }
                }
            }
            //
            // [Extension]
            // If extended globbing is enabled, the last consumed character is an
            // unquoted start of an extglob pattern, *and* if the current character
            // is an open parenthesis, then this begins an extglob pattern.
            else if c == '('
                && self.options.enable_extended_globbing
                && state.unquoted()
                && !state.in_operator()
                && state
                    .current_token()
                    .ends_with(|x| Self::can_start_extglob(x))
            {
                // Consume the '(' and append it.
                self.consume_char()?;
                state.append_char(c);

                let mut paren_depth = 1;

                // Keep consuming until we see the matching end ')'.
                while paren_depth > 0 {
                    if let Some(extglob_char) = self.next_char()? {
                        // Include it in the token.
                        state.append_char(extglob_char);

                        // Look for ')' to terminate.
                        // TODO: handle escaping?
                        if extglob_char == '(' {
                            paren_depth += 1;
                        } else if extglob_char == ')' {
                            paren_depth -= 1;
                        }
                    } else {
                        return Err(TokenizerError::UnterminatedExtendedGlob(
                            self.cross_state.cursor.clone(),
                        ));
                    }
                }
            //
            // If the character *can* start an operator, then it will.
            //
            } else if state.unquoted() && Self::can_start_operator(c) {
                if state.started_token() {
                    result = state.delimit_current_token(
                        TokenEndReason::OperatorStart,
                        &mut self.cross_state,
                    )?;
                } else {
                    state.token_is_operator = true;
                    self.consume_char()?;
                    state.append_char(c);
                }
            //
            // Whitespace gets discarded (and delimits tokens).
            //
            } else if state.unquoted() && is_blank(c) {
                if state.started_token() {
                    result = state.delimit_current_token(
                        TokenEndReason::NonNewLineBlank,
                        &mut self.cross_state,
                    )?;
                } else if include_space {
                    state.append_char(c);
                } else {
                    // Make sure we don't include this char in the token range.
                    state.start_position.column += 1;
                    state.start_position.index += 1;
                }

                self.consume_char()?;
            }
            //
            // N.B. We need to remember if we were recursively called in a variable
            // expansion expression; in that case we won't think a token was started but...
            // we'd be wrong.
            else if !state.token_is_operator
                && (state.started_token() || matches!(terminating_char, Some('}')))
            {
                self.consume_char()?;
                state.append_char(c);
            } else if c == '#' {
                // Consume the '#'.
                self.consume_char()?;

                let mut done = false;
                while !done {
                    done = match self.peek_char()? {
                        Some('\n') => true,
                        None => true,
                        _ => {
                            // Consume the peeked char; it's part of the comment.
                            self.consume_char()?;
                            false
                        }
                    };
                }
                // Re-start loop as if the comment never happened.
            } else if state.started_token() {
                // In all other cases where we have an in-progress token, we delimit here.
                result =
                    state.delimit_current_token(TokenEndReason::Other, &mut self.cross_state)?;
            } else {
                // If we got here, then we don't have a token in progress and we're not starting an
                // operator. Add the character to a new token.
                self.consume_char()?;
                state.append_char(c);
            }
        }

        let result = result.unwrap();

        Ok(result)
    }

    fn remove_here_end_tag(
        &mut self,
        state: &mut TokenParseState,
        result: &mut Option<TokenizeResult>,
        ends_with_newline: bool,
    ) -> Result<bool, TokenizerError> {
        // Bail immediately if we don't even have a *starting* here tag.
        if self.cross_state.current_here_tags.is_empty() {
            return Ok(false);
        }

        let next_here_tag = &self.cross_state.current_here_tags[0];

        let tag_str: Cow<'_, str> = if next_here_tag.tag_was_escaped_or_quoted {
            unquote_str(next_here_tag.tag.as_str()).into()
        } else {
            next_here_tag.tag.as_str().into()
        };

        let tag_str = if !ends_with_newline {
            tag_str
                .strip_suffix('\n')
                .unwrap_or_else(|| tag_str.as_ref())
        } else {
            tag_str.as_ref()
        };

        if let Some(current_token_without_here_tag) = state.current_token().strip_suffix(tag_str) {
            // Make sure that was either the start of the here document, or there
            // was a newline between the preceding part
            // and the tag.
            if current_token_without_here_tag.is_empty()
                || current_token_without_here_tag.ends_with('\n')
            {
                state.replace_with_here_doc(current_token_without_here_tag.to_owned());

                // Delimit the end of the here-document body.
                *result = state.delimit_current_token(
                    TokenEndReason::HereDocumentBodyEnd,
                    &mut self.cross_state,
                )?;

                return Ok(true);
            }
        }
        Ok(false)
    }

    const fn can_start_extglob(c: char) -> bool {
        matches!(c, '@' | '!' | '?' | '+' | '*')
    }

    const fn can_start_operator(c: char) -> bool {
        matches!(c, '&' | '(' | ')' | ';' | '\n' | '|' | '<' | '>')
    }

    fn is_operator(&self, s: &str) -> bool {
        // Handle non-POSIX operators.
        if !self.options.sh_mode && matches!(s, "<<<" | "&>" | "&>>" | ";;&" | ";&" | "|&") {
            return true;
        }

        matches!(
            s,
            "&" | "&&"
                | "("
                | ")"
                | ";"
                | ";;"
                | "\n"
                | "|"
                | "||"
                | "<"
                | ">"
                | ">|"
                | "<<"
                | ">>"
                | "<&"
                | ">&"
                | "<<-"
                | "<>"
        )
    }
}

impl<R: ?Sized + std::io::BufRead> Iterator for Tokenizer<'_, R> {
    type Item = Result<TokenizeResult, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            #[expect(clippy::manual_map)]
            Ok(result) => match result.token {
                Some(_) => Some(Ok(result)),
                None => None,
            },
            Err(e) => Some(Err(e)),
        }
    }
}

const fn is_blank(c: char) -> bool {
    c == ' ' || c == '\t'
}

const fn does_char_newly_affect_quoting(state: &TokenParseState, c: char) -> bool {
    // If we're currently escaped, then nothing affects quoting.
    if state.in_escape {
        return false;
    }

    match state.quote_mode {
        // When we're in a double quote or ANSI-C quote, only a subset of escape
        // sequences are recognized.
        QuoteMode::Double(_) | QuoteMode::AnsiC(_) => {
            if c == '\\' {
                // TODO: handle backslash in double quote
                true
            } else {
                false
            }
        }
        // When we're in a single quote, nothing affects quoting.
        QuoteMode::Single(_) => false,
        // When we're not already in a quote, then we can straightforwardly look for a
        // quote mark or backslash.
        QuoteMode::None => is_quoting_char(c),
    }
}

const fn is_quoting_char(c: char) -> bool {
    matches!(c, '\\' | '\'' | '\"')
}

/// Return a string with all the quoting removed.
///
/// # Arguments
///
/// * `s` - The string to unquote.
pub fn unquote_str(s: &str) -> String {
    let mut result = String::new();

    let mut in_escape = false;
    for c in s.chars() {
        match c {
            c if in_escape => {
                result.push(c);
                in_escape = false;
            }
            '\\' => in_escape = true,
            c if is_quoting_char(c) => (),
            c => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use insta::assert_ron_snapshot;
    use pretty_assertions::{assert_eq, assert_matches};

    #[derive(serde::Serialize)]
    struct TokenizerResult<'a> {
        input: &'a str,
        result: Vec<Token>,
    }

    fn test_tokenizer(input: &str) -> Result<TokenizerResult<'_>> {
        Ok(TokenizerResult {
            input,
            result: tokenize_str(input)?,
        })
    }

    #[test]
    fn tokenize_empty() -> Result<()> {
        let tokens = tokenize_str("")?;
        assert_eq!(tokens.len(), 0);
        Ok(())
    }

    #[test]
    fn tokenize_line_continuation() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"a\
bc"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_operators() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("a>>b")?);
        Ok(())
    }

    #[test]
    fn tokenize_comment() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"a #comment
"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_comment_at_eof() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r"a #comment")?);
        Ok(())
    }

    #[test]
    fn tokenize_empty_here_doc() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"cat <<HERE
HERE
"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_here_doc() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"cat <<HERE
SOMETHING
HERE
echo after
"
        )?);
        assert_ron_snapshot!(test_tokenizer(
            r"cat <<HERE
SOMETHING
HERE
"
        )?);
        assert_ron_snapshot!(test_tokenizer(
            r"cat <<HERE
SOMETHING
HERE

"
        )?);
        assert_ron_snapshot!(test_tokenizer(
            r"cat <<HERE
SOMETHING
HERE"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_here_doc_with_tab_removal() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"cat <<-HERE
	SOMETHING
	HERE
"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_here_doc_with_other_tokens() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"cat <<EOF | wc -l
A B C
1 2 3
D E F
EOF
"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_multiple_here_docs() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"cat <<HERE1 <<HERE2
SOMETHING
HERE1
OTHER
HERE2
echo after
"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_unterminated_here_doc() {
        let result = tokenize_str(
            r"cat <<HERE
SOMETHING
",
        );
        assert!(result.is_err());
    }

    #[test]
    fn tokenize_missing_here_tag() {
        let result = tokenize_str(
            r"cat <<
",
        );
        assert!(result.is_err());
    }

    #[test]
    fn tokenize_here_doc_in_command_substitution() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"echo $(cat <<HERE
TEXT
HERE
)"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_here_doc_in_double_quoted_command_substitution() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r#"echo "$(cat <<HERE
TEXT
HERE
)""#
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_here_doc_in_double_quoted_command_substitution_with_space() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r#"echo "$(cat << HERE
TEXT
HERE
)""#
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_complex_here_docs_in_command_substitution() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(
            r"echo $(cat <<HERE1 <<HERE2 | wc -l
TEXT
HERE1
OTHER
HERE2
)"
        )?);
        Ok(())
    }

    #[test]
    fn tokenize_simple_backquote() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r"echo `echo hi`")?);
        Ok(())
    }

    #[test]
    fn tokenize_backquote_with_escape() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r"echo `echo\`hi`")?);
        Ok(())
    }

    #[test]
    fn tokenize_unterminated_backquote() {
        assert_matches!(
            tokenize_str("`"),
            Err(TokenizerError::UnterminatedBackquote(_))
        );
    }

    #[test]
    fn tokenize_unterminated_command_substitution() {
        assert_matches!(
            tokenize_str("$("),
            Err(TokenizerError::UnterminatedCommandSubstitution)
        );
    }

    #[test]
    fn tokenize_command_substitution() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("a$(echo hi)b c")?);
        Ok(())
    }

    #[test]
    fn tokenize_command_substitution_with_subshell() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("$( (:) )")?);
        Ok(())
    }

    #[test]
    fn tokenize_command_substitution_containing_extglob() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("echo $(echo !(x))")?);
        Ok(())
    }

    #[test]
    fn tokenize_arithmetic_expression() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("a$((1+2))b c")?);
        Ok(())
    }

    #[test]
    fn tokenize_arithmetic_expression_with_space() -> Result<()> {
        // N.B. The spacing comes out a bit odd, but it gets processed okay
        // by later stages.
        assert_ron_snapshot!(test_tokenizer("$(( 1 ))")?);
        Ok(())
    }
    #[test]
    fn tokenize_arithmetic_expression_with_parens() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("$(( (0) ))")?);
        Ok(())
    }

    #[test]
    fn tokenize_special_parameters() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("$$")?);
        assert_ron_snapshot!(test_tokenizer("$@")?);
        assert_ron_snapshot!(test_tokenizer("$!")?);
        assert_ron_snapshot!(test_tokenizer("$?")?);
        assert_ron_snapshot!(test_tokenizer("$*")?);
        Ok(())
    }

    #[test]
    fn tokenize_unbraced_parameter_expansion() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("$x")?);
        assert_ron_snapshot!(test_tokenizer("a$x")?);
        Ok(())
    }

    #[test]
    fn tokenize_unterminated_parameter_expansion() {
        assert_matches!(
            tokenize_str("${x"),
            Err(TokenizerError::UnterminatedVariable)
        );
    }

    #[test]
    fn tokenize_braced_parameter_expansion() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("${x}")?);
        assert_ron_snapshot!(test_tokenizer("a${x}b")?);
        Ok(())
    }

    #[test]
    fn tokenize_braced_parameter_expansion_with_escaping() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r"a${x\}}b")?);
        Ok(())
    }

    #[test]
    fn tokenize_whitespace() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer("1 2 3")?);
        Ok(())
    }

    #[test]
    fn tokenize_escaped_whitespace() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r"1\ 2 3")?);
        Ok(())
    }

    #[test]
    fn tokenize_single_quote() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r"x'a b'y")?);
        Ok(())
    }

    #[test]
    fn tokenize_double_quote() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r#"x"a b"y"#)?);
        Ok(())
    }

    #[test]
    fn tokenize_double_quoted_command_substitution() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r#"x"$(echo hi)"y"#)?);
        Ok(())
    }

    #[test]
    fn tokenize_double_quoted_arithmetic_expression() -> Result<()> {
        assert_ron_snapshot!(test_tokenizer(r#"x"$((1+2))"y"#)?);
        Ok(())
    }

    #[test]
    fn test_quote_removal() {
        assert_eq!(unquote_str(r#""hello""#), "hello");
        assert_eq!(unquote_str(r"'hello'"), "hello");
        assert_eq!(unquote_str(r#""hel\"lo""#), r#"hel"lo"#);
        assert_eq!(unquote_str(r"'hel\'lo'"), r"hel'lo");
    }
}
