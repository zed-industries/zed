//! Home to the YAML Scanner.
//!
//! The scanner is the lowest-level parsing utility. It is the lexer / tokenizer, reading input a
//! character at a time and emitting tokens that can later be interpreted by the [`crate::parser`]
//! to check for more context and validity.
//!
//! Due to the grammar of YAML, the scanner has to have some context and is not error-free.

#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use std::{char, collections::VecDeque, error::Error, fmt};

use arraydeque::ArrayDeque;

use crate::char_traits::{
    as_hex, is_alpha, is_anchor_char, is_blank, is_blank_or_breakz, is_break, is_breakz, is_digit,
    is_flow, is_hex, is_tag_char, is_uri_char, is_z,
};

/// The encoding of the input. Currently, only UTF-8 is supported.
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum TEncoding {
    /// UTF-8 encoding.
    Utf8,
}

/// The style as which the scalar was written in the YAML document.
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum TScalarStyle {
    /// A YAML plain scalar.
    Plain,
    /// A YAML single quoted scalar.
    SingleQuoted,
    /// A YAML double quoted scalar.
    DoubleQuoted,

    /// A YAML literal block (`|` block).
    Literal,
    /// A YAML folded block (`>` block).
    Folded,
}

/// A location in a yaml document.
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub struct Marker {
    /// The index (in chars) in the input string.
    index: usize,
    /// The line (1-indexed).
    line: usize,
    /// The column (1-indexed).
    col: usize,
}

impl Marker {
    fn new(index: usize, line: usize, col: usize) -> Marker {
        Marker { index, line, col }
    }

    /// Return the index (in bytes) of the marker in the source.
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Return the line of the marker in the source.
    #[must_use]
    pub fn line(&self) -> usize {
        self.line
    }

    /// Return the column of the marker in the source.
    #[must_use]
    pub fn col(&self) -> usize {
        self.col
    }
}

/// An error that occurred while scanning.
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct ScanError {
    /// The position at which the error happened in the source.
    mark: Marker,
    /// Human-readable details about the error.
    info: String,
}

impl ScanError {
    /// Create a new error from a location and an error string.
    #[must_use]
    pub fn new(loc: Marker, info: &str) -> ScanError {
        ScanError {
            mark: loc,
            info: info.to_owned(),
        }
    }

    /// Create a new error from a location and an error string.
    #[must_use]
    pub fn new_string(loc: Marker, info: String) -> ScanError {
        ScanError { mark: loc, info }
    }

    /// Return the marker pointing to the error in the source.
    #[must_use]
    pub fn marker(&self) -> &Marker {
        &self.mark
    }

    /// Return the information string describing the error that happened.
    #[must_use]
    pub fn info(&self) -> &str {
        self.info.as_ref()
    }
}

impl Error for ScanError {
    fn description(&self) -> &str {
        self.info.as_ref()
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for ScanError {
    // col starts from 0
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{} at byte {} line {} column {}",
            self.info,
            self.mark.index,
            self.mark.line,
            self.mark.col + 1,
        )
    }
}

/// The contents of a scanner token.
#[derive(Clone, PartialEq, Debug, Eq)]
pub enum TokenType {
    /// The start of the stream. Sent first, before even [`TokenType::DocumentStart`].
    StreamStart(TEncoding),
    /// The end of the stream, EOF.
    StreamEnd,
    /// A YAML version directive.
    VersionDirective(
        /// Major
        u32,
        /// Minor
        u32,
    ),
    /// A YAML tag directive (e.g.: `!!str`, `!foo!bar`, ...).
    TagDirective(
        /// Handle
        String,
        /// Prefix
        String,
    ),
    /// The start of a YAML document (`---`).
    DocumentStart,
    /// The end of a YAML document (`...`).
    DocumentEnd,
    /// The start of a sequence block.
    ///
    /// Sequence blocks are arrays starting with a `-`.
    BlockSequenceStart,
    /// The start of a sequence mapping.
    ///
    /// Sequence mappings are "dictionaries" with "key: value" entries.
    BlockMappingStart,
    /// End of the corresponding `BlockSequenceStart` or `BlockMappingStart`.
    BlockEnd,
    /// Start of an inline array (`[ a, b ]`).
    FlowSequenceStart,
    /// End of an inline array.
    FlowSequenceEnd,
    /// Start of an inline mapping (`{ a: b, c: d }`).
    FlowMappingStart,
    /// End of an inline mapping.
    FlowMappingEnd,
    /// An entry in a block sequence (c.f.: [`TokenType::BlockSequenceStart`]).
    BlockEntry,
    /// An entry in a flow sequence (c.f.: [`TokenType::FlowSequenceStart`]).
    FlowEntry,
    /// A key in a mapping.
    Key,
    /// A value in a mapping.
    Value,
    /// A reference to an anchor.
    Alias(String),
    /// A YAML anchor (`&`/`*`).
    Anchor(String),
    /// A YAML tag (starting with bangs `!`).
    Tag(
        /// The handle of the tag.
        String,
        /// The suffix of the tag.
        String,
    ),
    /// A regular YAML scalar.
    Scalar(TScalarStyle, String),
}

/// A scanner token.
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Token(pub Marker, pub TokenType);

/// A scalar that was parsed and may correspond to a simple key.
///
/// Upon scanning the following yaml:
/// ```yaml
/// a: b
/// ```
/// We do not know that `a` is a key for a map until we have reached the following `:`. For this
/// YAML, we would store `a` as a scalar token in the [`Scanner`], but not emit it yet. It would be
/// kept inside the scanner until more context is fetched and we are able to know whether it is a
/// plain scalar or a key.
///
/// For example, see the following 2 yaml documents:
/// ```yaml
/// ---
/// a: b # Here, `a` is a key.
/// ...
/// ---
/// a # Here, `a` is a plain scalar.
/// ...
/// ```
/// An instance of [`SimpleKey`] is created in the [`Scanner`] when such ambiguity occurs.
///
/// In both documents, scanning `a` would lead to the creation of a [`SimpleKey`] with
/// [`Self::possible`] set to `true`. The token for `a` would be pushed in the [`Scanner`] but not
/// yet emitted. Instead, more context would be fetched (through [`Scanner::fetch_more_tokens`]).
///
/// In the first document, upon reaching the `:`, the [`SimpleKey`] would be inspected and our
/// scalar `a` since it is a possible key, would be "turned" into a key. This is done by prepending
/// a [`TokenType::Key`] to our scalar token in the [`Scanner`]. This way, the
/// [`crate::parser::Parser`] would read the [`TokenType::Key`] token before the
/// [`TokenType::Scalar`] token.
///
/// In the second document however, reaching the EOF would stale the [`SimpleKey`] and no
/// [`TokenType::Key`] would be emitted by the scanner.
#[derive(Clone, PartialEq, Debug, Eq)]
struct SimpleKey {
    /// Whether the token this [`SimpleKey`] refers to may still be a key.
    ///
    /// Sometimes, when we have more context, we notice that what we thought could be a key no
    /// longer can be. In that case, [`Self::possible`] is set to `false`.
    ///
    /// For instance, let us consider the following invalid YAML:
    /// ```yaml
    /// key
    ///   : value
    /// ```
    /// Upon reading the `\n` after `key`, the [`SimpleKey`] that was created for `key` is staled
    /// and [`Self::possible`] set to `false`.
    possible: bool,
    /// Whether the token this [`SimpleKey`] refers to is required to be a key.
    ///
    /// With more context, we may know for sure that the token must be a key. If the YAML is
    /// invalid, it may happen that the token be deemed not a key. In such event, an error has to
    /// be raised. This boolean helps us know when to raise such error.
    ///
    /// TODO(ethiraric, 30/12/2023): Example of when this happens.
    required: bool,
    /// The index of the token referred to by the [`SimpleKey`].
    ///
    /// This is the index in the scanner, which takes into account both the tokens that have been
    /// emitted and those about to be emitted. See [`Scanner::tokens_parsed`] and
    /// [`Scanner::tokens`] for more details.
    token_number: usize,
    /// The position at which the token the [`SimpleKey`] refers to is.
    mark: Marker,
}

impl SimpleKey {
    /// Create a new [`SimpleKey`] at the given `Marker` and with the given flow level.
    fn new(mark: Marker) -> SimpleKey {
        SimpleKey {
            possible: false,
            required: false,
            token_number: 0,
            mark,
        }
    }
}

/// An indentation level on the stack of indentations.
#[derive(Clone, Debug, Default)]
struct Indent {
    /// The former indentation level.
    indent: isize,
    /// Whether, upon closing, this indents generates a `BlockEnd` token.
    ///
    /// There are levels of indentation which do not start a block. Examples of this would be:
    /// ```yaml
    /// -
    ///   foo # ok
    /// -
    /// bar # ko, bar needs to be indented further than the `-`.
    /// - [
    ///  baz, # ok
    /// quux # ko, quux needs to be indented further than the '-'.
    /// ] # ko, the closing bracket needs to be indented further than the `-`.
    /// ```
    ///
    /// The indentation level created by the `-` is for a single entry in the sequence. Emitting a
    /// `BlockEnd` when this indentation block ends would generate one `BlockEnd` per entry in the
    /// sequence, although we must have exactly one to end the sequence.
    needs_block_end: bool,
}

/// The size of the [`Scanner`] buffer.
///
/// The buffer is statically allocated to avoid conditions for reallocations each time we
/// consume/push a character. As of now, almost all lookaheads are 4 characters maximum, except:
///   - Escape sequences parsing: some escape codes are 8 characters
///   - Scanning indent in scalars: this looks ahead `indent + 2` characters
/// This constant must be set to at least 8. When scanning indent in scalars, the lookahead is done
/// in a single call if and only if the indent is `BUFFER_LEN - 2` or less. If the indent is higher
/// than that, the code will fall back to a loop of lookaheads.
const BUFFER_LEN: usize = 16;

/// The YAML scanner.
///
/// This corresponds to the low-level interface when reading YAML. The scanner emits token as they
/// are read (akin to a lexer), but it also holds sufficient context to be able to disambiguate
/// some of the constructs. It has understanding of indentation and whitespace and is able to
/// generate error messages for some invalid YAML constructs.
///
/// It is however not a full parser and needs [`crate::parser::Parser`] to fully detect invalid
/// YAML documents.
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Scanner<T> {
    /// The reader, providing with characters.
    rdr: T,
    /// The position of the cursor within the reader.
    mark: Marker,
    /// Buffer for tokens to be returned.
    ///
    /// This buffer can hold some temporary tokens that are not yet ready to be returned. For
    /// instance, if we just read a scalar, it can be a value or a key if an implicit mapping
    /// follows. In this case, the token stays in the `VecDeque` but cannot be returned from
    /// [`Self::next`] until we have more context.
    tokens: VecDeque<Token>,
    /// Buffer for the next characters to consume.
    buffer: ArrayDeque<char, BUFFER_LEN>,
    /// The last error that happened.
    error: Option<ScanError>,

    /// Whether we have already emitted the `StreamStart` token.
    stream_start_produced: bool,
    /// Whether we have already emitted the `StreamEnd` token.
    stream_end_produced: bool,
    /// In some flow contexts, the value of a mapping is allowed to be adjacent to the `:`. When it
    /// is, the index at which the `:` may be must be stored in `adjacent_value_allowed_at`.
    adjacent_value_allowed_at: usize,
    /// Whether a simple key could potentially start at the current position.
    ///
    /// Simple keys are the opposite of complex keys which are keys starting with `?`.
    simple_key_allowed: bool,
    /// A stack of potential simple keys.
    ///
    /// Refer to the documentation of [`SimpleKey`] for a more in-depth explanation of what they
    /// are.
    simple_keys: Vec<SimpleKey>,
    /// The current indentation level.
    indent: isize,
    /// List of all block indentation levels we are in (except the current one).
    indents: Vec<Indent>,
    /// Level of nesting of flow sequences.
    flow_level: u8,
    /// The number of tokens that have been returned from the scanner.
    ///
    /// This excludes the tokens from [`Self::tokens`].
    tokens_parsed: usize,
    /// Whether a token is ready to be taken from [`Self::tokens`].
    token_available: bool,
    /// Whether all characters encountered since the last newline were whitespace.
    leading_whitespace: bool,
    /// Whether we started a flow mapping.
    ///
    /// This is used to detect implicit flow mapping starts such as:
    /// ```yaml
    /// [ : foo ] # { null: "foo" }
    /// ```
    flow_mapping_started: bool,
    /// Whether we currently are in an implicit flow mapping.
    implicit_flow_mapping: bool,
}

impl<T: Iterator<Item = char>> Iterator for Scanner<T> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if self.error.is_some() {
            return None;
        }
        match self.next_token() {
            Ok(Some(tok)) => {
                debug_print!(
                    "    \x1B[;32m\u{21B3} {:?} \x1B[;36m{:?}\x1B[;m",
                    tok.1,
                    tok.0
                );
                Some(tok)
            }
            Ok(tok) => tok,
            Err(e) => {
                self.error = Some(e);
                None
            }
        }
    }
}

/// A convenience alias for scanner functions that may fail without returning a value.
pub type ScanResult = Result<(), ScanError>;

impl<T: Iterator<Item = char>> Scanner<T> {
    /// Creates the YAML tokenizer.
    pub fn new(rdr: T) -> Scanner<T> {
        Scanner {
            rdr,
            buffer: ArrayDeque::new(),
            mark: Marker::new(0, 1, 0),
            tokens: VecDeque::new(),
            error: None,

            stream_start_produced: false,
            stream_end_produced: false,
            adjacent_value_allowed_at: 0,
            simple_key_allowed: true,
            simple_keys: Vec::new(),
            indent: -1,
            indents: Vec::new(),
            flow_level: 0,
            tokens_parsed: 0,
            token_available: false,
            leading_whitespace: true,
            flow_mapping_started: false,
            implicit_flow_mapping: false,
        }
    }

    /// Get a copy of the last error that was encountered, if any.
    ///
    /// This does not clear the error state and further calls to [`Self::get_error`] will return (a
    /// clone of) the same error.
    #[inline]
    pub fn get_error(&self) -> Option<ScanError> {
        self.error.clone()
    }

    /// Fill `self.buffer` with at least `count` characters.
    ///
    /// The characters that are extracted this way are not consumed but only placed in the buffer.
    #[inline]
    fn lookahead(&mut self, count: usize) {
        if self.buffer.len() >= count {
            return;
        }
        for _ in 0..(count - self.buffer.len()) {
            self.buffer
                .push_back(self.rdr.next().unwrap_or('\0'))
                .unwrap();
        }
    }

    /// Consume the next character. It is assumed the next character is a blank.
    #[inline]
    fn skip_blank(&mut self) {
        self.buffer.pop_front();

        self.mark.index += 1;
        self.mark.col += 1;
    }

    /// Consume the next character. It is assumed the next character is not a blank.
    #[inline]
    fn skip_non_blank(&mut self) {
        self.buffer.pop_front();

        self.mark.index += 1;
        self.mark.col += 1;
        self.leading_whitespace = false;
    }

    /// Consume the next characters. It is assumed none of the next characters are blanks.
    #[inline]
    fn skip_n_non_blank(&mut self, n: usize) {
        self.buffer.drain(0..n);

        self.mark.index += n;
        self.mark.col += n;
        self.leading_whitespace = false;
    }

    /// Consume the next character. It is assumed the next character is a newline.
    #[inline]
    fn skip_nl(&mut self) {
        self.buffer.pop_front();

        self.mark.index += 1;
        self.mark.col = 0;
        self.mark.line += 1;
        self.leading_whitespace = true;
    }

    /// Consume a linebreak (either CR, LF or CRLF), if any. Do nothing if there's none.
    #[inline]
    fn skip_linebreak(&mut self) {
        if self.buffer[0] == '\r' && self.buffer[1] == '\n' {
            // While technically not a blank, this does not matter as `self.leading_whitespace`
            // will be reset by `skip_nl`.
            self.skip_blank();
            self.skip_nl();
        } else if is_break(self.buffer[0]) {
            self.skip_nl();
        }
    }

    /// Return the next character in the buffer.
    ///
    /// The character is not consumed.
    #[inline]
    fn ch(&self) -> char {
        self.buffer[0]
    }

    /// Look for the next character and return it.
    ///
    /// The character is not consumed.
    /// Equivalent to calling [`Self::lookahead`] and [`Self::ch`].
    #[inline]
    fn look_ch(&mut self) -> char {
        self.lookahead(1);
        self.ch()
    }

    /// Read a character from the input stream, returning it directly.
    ///
    /// The buffer is bypassed and `self.mark` needs to be updated manually.
    #[inline]
    #[must_use]
    fn raw_read_ch(&mut self) -> char {
        self.rdr.next().unwrap_or('\0')
    }

    /// Return whether the next character is `c`.
    #[inline]
    fn ch_is(&self, c: char) -> bool {
        self.buffer[0] == c
    }

    /// Return whether the [`TokenType::StreamStart`] event has been emitted.
    #[inline]
    pub fn stream_started(&self) -> bool {
        self.stream_start_produced
    }

    /// Return whether the [`TokenType::StreamEnd`] event has been emitted.
    #[inline]
    pub fn stream_ended(&self) -> bool {
        self.stream_end_produced
    }

    /// Get the current position in the input stream.
    #[inline]
    pub fn mark(&self) -> Marker {
        self.mark
    }

    // Read and consume a line break (either `\r`, `\n` or `\r\n`).
    //
    // A `\n` is pushed into `s`.
    //
    // # Panics (in debug)
    // If the next characters do not correspond to a line break.
    #[inline]
    fn read_break(&mut self, s: &mut String) {
        let c = self.buffer[0];
        let nc = self.buffer[1];
        debug_assert!(is_break(c));
        if c == '\r' && nc == '\n' {
            self.skip_blank();
        }
        self.skip_nl();

        s.push('\n');
    }

    /// Check whether the next characters correspond to an end of document.
    ///
    /// [`Self::lookahead`] must have been called before calling this function.
    fn next_is_document_end(&self) -> bool {
        assert!(self.buffer.len() >= 4);
        self.buffer[0] == '.'
            && self.buffer[1] == '.'
            && self.buffer[2] == '.'
            && is_blank_or_breakz(self.buffer[3])
    }

    /// Check whether the next characters correspond to a document indicator.
    ///
    /// [`Self::lookahead`] must have been called before calling this function.
    #[inline]
    fn next_is_document_indicator(&self) -> bool {
        assert!(self.buffer.len() >= 4);
        self.mark.col == 0
            && (((self.buffer[0] == '-') && (self.buffer[1] == '-') && (self.buffer[2] == '-'))
                || ((self.buffer[0] == '.') && (self.buffer[1] == '.') && (self.buffer[2] == '.')))
            && is_blank_or_breakz(self.buffer[3])
    }

    /// Insert a token at the given position.
    fn insert_token(&mut self, pos: usize, tok: Token) {
        let old_len = self.tokens.len();
        assert!(pos <= old_len);
        self.tokens.insert(pos, tok);
    }

    fn allow_simple_key(&mut self) {
        self.simple_key_allowed = true;
    }

    fn disallow_simple_key(&mut self) {
        self.simple_key_allowed = false;
    }

    /// Fetch the next token in the stream.
    /// # Errors
    /// Returns `ScanError` when the scanner does not find the next expected token.
    pub fn fetch_next_token(&mut self) -> ScanResult {
        self.lookahead(1);
        // eprintln!("--> fetch_next_token Cur {:?} {:?}", self.mark, self.ch());

        if !self.stream_start_produced {
            self.fetch_stream_start();
            return Ok(());
        }
        self.skip_to_next_token()?;

        debug_print!(
            "  \x1B[38;5;244m\u{2192} fetch_next_token after whitespace {:?} {:?}\x1B[m",
            self.mark,
            self.ch()
        );

        self.stale_simple_keys()?;

        let mark = self.mark;
        self.unroll_indent(mark.col as isize);

        self.lookahead(4);

        if is_z(self.ch()) {
            self.fetch_stream_end()?;
            return Ok(());
        }

        // Is it a directive?
        if self.mark.col == 0 && self.ch_is('%') {
            return self.fetch_directive();
        }

        if self.mark.col == 0
            && self.buffer[0] == '-'
            && self.buffer[1] == '-'
            && self.buffer[2] == '-'
            && is_blank_or_breakz(self.buffer[3])
        {
            self.fetch_document_indicator(TokenType::DocumentStart)?;
            return Ok(());
        }

        if self.mark.col == 0
            && self.buffer[0] == '.'
            && self.buffer[1] == '.'
            && self.buffer[2] == '.'
            && is_blank_or_breakz(self.buffer[3])
        {
            self.fetch_document_indicator(TokenType::DocumentEnd)?;
            self.skip_ws_to_eol(SkipTabs::Yes)?;
            if !is_breakz(self.ch()) {
                return Err(ScanError::new(
                    self.mark,
                    "invalid content after document end marker",
                ));
            }
            return Ok(());
        }

        if (self.mark.col as isize) < self.indent {
            return Err(ScanError::new(self.mark, "invalid indentation"));
        }

        let c = self.buffer[0];
        let nc = self.buffer[1];
        match c {
            '[' => self.fetch_flow_collection_start(TokenType::FlowSequenceStart),
            '{' => self.fetch_flow_collection_start(TokenType::FlowMappingStart),
            ']' => self.fetch_flow_collection_end(TokenType::FlowSequenceEnd),
            '}' => self.fetch_flow_collection_end(TokenType::FlowMappingEnd),
            ',' => self.fetch_flow_entry(),
            '-' if is_blank_or_breakz(nc) => self.fetch_block_entry(),
            '?' if is_blank_or_breakz(nc) => self.fetch_key(),
            ':' if is_blank_or_breakz(nc)
                || (self.flow_level > 0
                    && (is_flow(nc) || self.mark.index == self.adjacent_value_allowed_at)) =>
            {
                self.fetch_value()
            }
            // Is it an alias?
            '*' => self.fetch_anchor(true),
            // Is it an anchor?
            '&' => self.fetch_anchor(false),
            '!' => self.fetch_tag(),
            // Is it a literal scalar?
            '|' if self.flow_level == 0 => self.fetch_block_scalar(true),
            // Is it a folded scalar?
            '>' if self.flow_level == 0 => self.fetch_block_scalar(false),
            '\'' => self.fetch_flow_scalar(true),
            '"' => self.fetch_flow_scalar(false),
            // plain scalar
            '-' if !is_blank_or_breakz(nc) => self.fetch_plain_scalar(),
            ':' | '?' if !is_blank_or_breakz(nc) && self.flow_level == 0 => {
                self.fetch_plain_scalar()
            }
            '%' | '@' | '`' => Err(ScanError::new(
                self.mark,
                &format!("unexpected character: `{c}'"),
            )),
            _ => self.fetch_plain_scalar(),
        }
    }

    /// Return the next token in the stream.
    /// # Errors
    /// Returns `ScanError` when scanning fails to find an expected next token.
    pub fn next_token(&mut self) -> Result<Option<Token>, ScanError> {
        if self.stream_end_produced {
            return Ok(None);
        }

        if !self.token_available {
            self.fetch_more_tokens()?;
        }
        let Some(t) = self.tokens.pop_front() else {
            return Err(ScanError::new(
                self.mark,
                "did not find expected next token",
            ));
        };
        self.token_available = false;
        self.tokens_parsed += 1;

        if let TokenType::StreamEnd = t.1 {
            self.stream_end_produced = true;
        }
        Ok(Some(t))
    }

    /// Fetch tokens from the token stream.
    /// # Errors
    /// Returns `ScanError` when loading fails.
    pub fn fetch_more_tokens(&mut self) -> ScanResult {
        let mut need_more;
        loop {
            if self.tokens.is_empty() {
                need_more = true;
            } else {
                need_more = false;
                // Stale potential keys that we know won't be keys.
                self.stale_simple_keys()?;
                // If our next token to be emitted may be a key, fetch more context.
                for sk in &self.simple_keys {
                    if sk.possible && sk.token_number == self.tokens_parsed {
                        need_more = true;
                        break;
                    }
                }
            }

            if !need_more {
                break;
            }
            self.fetch_next_token()?;
        }
        self.token_available = true;

        Ok(())
    }

    /// Mark simple keys that can no longer be keys as such.
    ///
    /// This function sets `possible` to `false` to each key that, now we have more context, we
    /// know will not be keys.
    ///
    /// # Errors
    /// This function returns an error if one of the key we would stale was required to be a key.
    fn stale_simple_keys(&mut self) -> ScanResult {
        for sk in &mut self.simple_keys {
            if sk.possible
                // If not in a flow construct, simple keys cannot span multiple lines.
                && self.flow_level == 0
                    && (sk.mark.line < self.mark.line || sk.mark.index + 1024 < self.mark.index)
            {
                if sk.required {
                    return Err(ScanError::new(self.mark, "simple key expect ':'"));
                }
                sk.possible = false;
            }
        }
        Ok(())
    }

    /// Skip over all whitespace and comments until the next token.
    ///
    /// # Errors
    /// This function returns an error if a tabulation is encountered where there should not be
    /// one.
    fn skip_to_next_token(&mut self) -> ScanResult {
        loop {
            // TODO(chenyh) BOM
            match self.look_ch() {
                // Tabs may not be used as indentation.
                // "Indentation" only exists as long as a block is started, but does not exist
                // inside of flow-style constructs. Tabs are allowed as part of leading
                // whitespaces outside of indentation.
                // If a flow-style construct is in an indented block, its contents must still be
                // indented. Also, tabs are allowed anywhere in it if it has no content.
                '\t' if self.is_within_block()
                    && self.leading_whitespace
                    && (self.mark.col as isize) < self.indent =>
                {
                    self.skip_ws_to_eol(SkipTabs::Yes)?;
                    // If we have content on that line with a tab, return an error.
                    if !is_breakz(self.ch()) {
                        return Err(ScanError::new(
                            self.mark,
                            "tabs disallowed within this context (block indentation)",
                        ));
                    }
                }
                '\t' | ' ' => self.skip_blank(),
                '\n' | '\r' => {
                    self.lookahead(2);
                    self.skip_linebreak();
                    if self.flow_level == 0 {
                        self.allow_simple_key();
                    }
                }
                '#' => {
                    while !is_breakz(self.look_ch()) {
                        self.skip_non_blank();
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    /// Skip over YAML whitespace (` `, `\n`, `\r`).
    ///
    /// # Errors
    /// This function returns an error if no whitespace was found.
    fn skip_yaml_whitespace(&mut self) -> ScanResult {
        let mut need_whitespace = true;
        loop {
            match self.look_ch() {
                ' ' => {
                    self.skip_blank();

                    need_whitespace = false;
                }
                '\n' | '\r' => {
                    self.lookahead(2);
                    self.skip_linebreak();
                    if self.flow_level == 0 {
                        self.allow_simple_key();
                    }
                    need_whitespace = false;
                }
                '#' => {
                    while !is_breakz(self.look_ch()) {
                        self.skip_non_blank();
                    }
                }
                _ => break,
            }
        }

        if need_whitespace {
            Err(ScanError::new(self.mark(), "expected whitespace"))
        } else {
            Ok(())
        }
    }

    /// Skip yaml whitespace at most up to eol. Also skips comments.
    fn skip_ws_to_eol(&mut self, skip_tabs: SkipTabs) -> Result<SkipTabs, ScanError> {
        let mut encountered_tab = false;
        let mut has_yaml_ws = false;
        loop {
            match self.look_ch() {
                ' ' => {
                    has_yaml_ws = true;
                    self.skip_blank();
                }
                '\t' if skip_tabs != SkipTabs::No => {
                    encountered_tab = true;
                    self.skip_blank();
                }
                // YAML comments must be preceded by whitespace.
                '#' if !encountered_tab && !has_yaml_ws => {
                    return Err(ScanError::new(
                        self.mark,
                        "comments must be separated from other tokens by whitespace",
                    ));
                }
                '#' => {
                    while !is_breakz(self.look_ch()) {
                        self.skip_non_blank();
                    }
                }
                _ => break,
            }
        }

        Ok(SkipTabs::Result(encountered_tab, has_yaml_ws))
    }

    fn fetch_stream_start(&mut self) {
        let mark = self.mark;
        self.indent = -1;
        self.stream_start_produced = true;
        self.allow_simple_key();
        self.tokens
            .push_back(Token(mark, TokenType::StreamStart(TEncoding::Utf8)));
        self.simple_keys.push(SimpleKey::new(Marker::new(0, 0, 0)));
    }

    fn fetch_stream_end(&mut self) -> ScanResult {
        // force new line
        if self.mark.col != 0 {
            self.mark.col = 0;
            self.mark.line += 1;
        }

        // If the stream ended, we won't have more context. We can stall all the simple keys we
        // had. If one was required, however, that was an error and we must propagate it.
        for sk in &mut self.simple_keys {
            if sk.required && sk.possible {
                return Err(ScanError::new(self.mark, "simple key expected"));
            }
            sk.possible = false;
        }

        self.unroll_indent(-1);
        self.remove_simple_key()?;
        self.disallow_simple_key();

        self.tokens
            .push_back(Token(self.mark, TokenType::StreamEnd));
        Ok(())
    }

    fn fetch_directive(&mut self) -> ScanResult {
        self.unroll_indent(-1);
        self.remove_simple_key()?;

        self.disallow_simple_key();

        let tok = self.scan_directive()?;
        self.tokens.push_back(tok);

        Ok(())
    }

    fn scan_directive(&mut self) -> Result<Token, ScanError> {
        let start_mark = self.mark;
        self.skip_non_blank();

        let name = self.scan_directive_name()?;
        let tok = match name.as_ref() {
            "YAML" => self.scan_version_directive_value(&start_mark)?,
            "TAG" => self.scan_tag_directive_value(&start_mark)?,
            // XXX This should be a warning instead of an error
            _ => {
                // skip current line
                while !is_breakz(self.look_ch()) {
                    self.skip_non_blank();
                }
                // XXX return an empty TagDirective token
                Token(
                    start_mark,
                    TokenType::TagDirective(String::new(), String::new()),
                )
                // return Err(ScanError::new(start_mark,
                //     "while scanning a directive, found unknown directive name"))
            }
        };

        self.skip_ws_to_eol(SkipTabs::Yes)?;

        if is_breakz(self.ch()) {
            self.lookahead(2);
            self.skip_linebreak();
            Ok(tok)
        } else {
            Err(ScanError::new(
                start_mark,
                "while scanning a directive, did not find expected comment or line break",
            ))
        }
    }

    fn scan_version_directive_value(&mut self, mark: &Marker) -> Result<Token, ScanError> {
        while is_blank(self.look_ch()) {
            self.skip_blank();
        }

        let major = self.scan_version_directive_number(mark)?;

        if self.ch() != '.' {
            return Err(ScanError::new(
                *mark,
                "while scanning a YAML directive, did not find expected digit or '.' character",
            ));
        }
        self.skip_non_blank();

        let minor = self.scan_version_directive_number(mark)?;

        Ok(Token(*mark, TokenType::VersionDirective(major, minor)))
    }

    fn scan_directive_name(&mut self) -> Result<String, ScanError> {
        let start_mark = self.mark;
        let mut string = String::new();
        while is_alpha(self.look_ch()) {
            string.push(self.ch());
            self.skip_non_blank();
        }

        if string.is_empty() {
            return Err(ScanError::new(
                start_mark,
                "while scanning a directive, could not find expected directive name",
            ));
        }

        if !is_blank_or_breakz(self.ch()) {
            return Err(ScanError::new(
                start_mark,
                "while scanning a directive, found unexpected non-alphabetical character",
            ));
        }

        Ok(string)
    }

    fn scan_version_directive_number(&mut self, mark: &Marker) -> Result<u32, ScanError> {
        let mut val = 0u32;
        let mut length = 0usize;
        while let Some(digit) = self.look_ch().to_digit(10) {
            if length + 1 > 9 {
                return Err(ScanError::new(
                    *mark,
                    "while scanning a YAML directive, found extremely long version number",
                ));
            }
            length += 1;
            val = val * 10 + digit;
            self.skip_non_blank();
        }

        if length == 0 {
            return Err(ScanError::new(
                *mark,
                "while scanning a YAML directive, did not find expected version number",
            ));
        }

        Ok(val)
    }

    fn scan_tag_directive_value(&mut self, mark: &Marker) -> Result<Token, ScanError> {
        /* Eat whitespaces. */
        while is_blank(self.look_ch()) {
            self.skip_blank();
        }
        let handle = self.scan_tag_handle(true, mark)?;

        /* Eat whitespaces. */
        while is_blank(self.look_ch()) {
            self.skip_blank();
        }

        let prefix = self.scan_tag_prefix(mark)?;

        self.lookahead(1);

        if is_blank_or_breakz(self.ch()) {
            Ok(Token(*mark, TokenType::TagDirective(handle, prefix)))
        } else {
            Err(ScanError::new(
                *mark,
                "while scanning TAG, did not find expected whitespace or line break",
            ))
        }
    }

    fn fetch_tag(&mut self) -> ScanResult {
        self.save_simple_key();
        self.disallow_simple_key();

        let tok = self.scan_tag()?;
        self.tokens.push_back(tok);
        Ok(())
    }

    fn scan_tag(&mut self) -> Result<Token, ScanError> {
        let start_mark = self.mark;
        let mut handle = String::new();
        let mut suffix;

        // Check if the tag is in the canonical form (verbatim).
        self.lookahead(2);

        if self.buffer[1] == '<' {
            suffix = self.scan_verbatim_tag(&start_mark)?;
        } else {
            // The tag has either the '!suffix' or the '!handle!suffix'
            handle = self.scan_tag_handle(false, &start_mark)?;
            // Check if it is, indeed, handle.
            if handle.len() >= 2 && handle.starts_with('!') && handle.ends_with('!') {
                // A tag handle starting with "!!" is a secondary tag handle.
                let is_secondary_handle = handle == "!!";
                suffix =
                    self.scan_tag_shorthand_suffix(false, is_secondary_handle, "", &start_mark)?;
            } else {
                suffix = self.scan_tag_shorthand_suffix(false, false, &handle, &start_mark)?;
                "!".clone_into(&mut handle);
                // A special case: the '!' tag.  Set the handle to '' and the
                // suffix to '!'.
                if suffix.is_empty() {
                    handle.clear();
                    suffix = "!".to_owned();
                }
            }
        }

        if is_blank_or_breakz(self.look_ch()) || (self.flow_level > 0 && is_flow(self.ch())) {
            // XXX: ex 7.2, an empty scalar can follow a secondary tag
            Ok(Token(start_mark, TokenType::Tag(handle, suffix)))
        } else {
            Err(ScanError::new(
                start_mark,
                "while scanning a tag, did not find expected whitespace or line break",
            ))
        }
    }

    fn scan_tag_handle(&mut self, directive: bool, mark: &Marker) -> Result<String, ScanError> {
        let mut string = String::new();
        if self.look_ch() != '!' {
            return Err(ScanError::new(
                *mark,
                "while scanning a tag, did not find expected '!'",
            ));
        }

        string.push(self.ch());
        self.skip_non_blank();

        while is_alpha(self.look_ch()) {
            string.push(self.ch());
            self.skip_non_blank();
        }

        // Check if the trailing character is '!' and copy it.
        if self.ch() == '!' {
            string.push(self.ch());
            self.skip_non_blank();
        } else if directive && string != "!" {
            // It's either the '!' tag or not really a tag handle.  If it's a %TAG
            // directive, it's an error.  If it's a tag token, it must be a part of
            // URI.
            return Err(ScanError::new(
                *mark,
                "while parsing a tag directive, did not find expected '!'",
            ));
        }
        Ok(string)
    }

    /// Scan for a tag prefix (6.8.2.2).
    ///
    /// There are 2 kinds of tag prefixes:
    ///   - Local: Starts with a `!`, contains only URI chars (`!foo`)
    ///   - Global: Starts with a tag char, contains then URI chars (`!foo,2000:app/`)
    fn scan_tag_prefix(&mut self, start_mark: &Marker) -> Result<String, ScanError> {
        let mut string = String::new();

        if self.look_ch() == '!' {
            // If we have a local tag, insert and skip `!`.
            string.push(self.ch());
            self.skip_non_blank();
        } else if !is_tag_char(self.ch()) {
            // Otherwise, check if the first global tag character is valid.
            return Err(ScanError::new(*start_mark, "invalid global tag character"));
        } else if self.ch() == '%' {
            // If it is valid and an escape sequence, escape it.
            string.push(self.scan_uri_escapes(start_mark)?);
        } else {
            // Otherwise, push the first character.
            string.push(self.ch());
            self.skip_non_blank();
        }

        while is_uri_char(self.look_ch()) {
            if self.ch() == '%' {
                string.push(self.scan_uri_escapes(start_mark)?);
            } else {
                string.push(self.ch());
                self.skip_non_blank();
            }
        }

        Ok(string)
    }

    /// Scan for a verbatim tag.
    ///
    /// The prefixing `!<` must _not_ have been skipped.
    fn scan_verbatim_tag(&mut self, start_mark: &Marker) -> Result<String, ScanError> {
        // Eat `!<`
        self.skip_non_blank();
        self.skip_non_blank();

        let mut string = String::new();
        while is_uri_char(self.look_ch()) {
            if self.ch() == '%' {
                string.push(self.scan_uri_escapes(start_mark)?);
            } else {
                string.push(self.ch());
                self.skip_non_blank();
            }
        }

        if self.ch() != '>' {
            return Err(ScanError::new(
                *start_mark,
                "while scanning a verbatim tag, did not find the expected '>'",
            ));
        }
        self.skip_non_blank();

        Ok(string)
    }

    fn scan_tag_shorthand_suffix(
        &mut self,
        _directive: bool,
        _is_secondary: bool,
        head: &str,
        mark: &Marker,
    ) -> Result<String, ScanError> {
        let mut length = head.len();
        let mut string = String::new();

        // Copy the head if needed.
        // Note that we don't copy the leading '!' character.
        if length > 1 {
            string.extend(head.chars().skip(1));
        }

        while is_tag_char(self.look_ch()) {
            // Check if it is a URI-escape sequence.
            if self.ch() == '%' {
                string.push(self.scan_uri_escapes(mark)?);
            } else {
                string.push(self.ch());
                self.skip_non_blank();
            }

            length += 1;
        }

        if length == 0 {
            return Err(ScanError::new(
                *mark,
                "while parsing a tag, did not find expected tag URI",
            ));
        }

        Ok(string)
    }

    fn scan_uri_escapes(&mut self, mark: &Marker) -> Result<char, ScanError> {
        let mut width = 0usize;
        let mut code = 0u32;
        loop {
            self.lookahead(3);

            if !(self.ch() == '%' && is_hex(self.buffer[1]) && is_hex(self.buffer[2])) {
                return Err(ScanError::new(
                    *mark,
                    "while parsing a tag, did not find URI escaped octet",
                ));
            }

            let octet = (as_hex(self.buffer[1]) << 4) + as_hex(self.buffer[2]);
            if width == 0 {
                width = match octet {
                    _ if octet & 0x80 == 0x00 => 1,
                    _ if octet & 0xE0 == 0xC0 => 2,
                    _ if octet & 0xF0 == 0xE0 => 3,
                    _ if octet & 0xF8 == 0xF0 => 4,
                    _ => {
                        return Err(ScanError::new(
                            *mark,
                            "while parsing a tag, found an incorrect leading UTF-8 octet",
                        ));
                    }
                };
                code = octet;
            } else {
                if octet & 0xc0 != 0x80 {
                    return Err(ScanError::new(
                        *mark,
                        "while parsing a tag, found an incorrect trailing UTF-8 octet",
                    ));
                }
                code = (code << 8) + octet;
            }

            self.skip_n_non_blank(3);

            width -= 1;
            if width == 0 {
                break;
            }
        }

        match char::from_u32(code) {
            Some(ch) => Ok(ch),
            None => Err(ScanError::new(
                *mark,
                "while parsing a tag, found an invalid UTF-8 codepoint",
            )),
        }
    }

    fn fetch_anchor(&mut self, alias: bool) -> ScanResult {
        self.save_simple_key();
        self.disallow_simple_key();

        let tok = self.scan_anchor(alias)?;

        self.tokens.push_back(tok);

        Ok(())
    }

    fn scan_anchor(&mut self, alias: bool) -> Result<Token, ScanError> {
        let mut string = String::new();
        let start_mark = self.mark;

        self.skip_non_blank();
        while is_anchor_char(self.look_ch()) {
            string.push(self.ch());
            self.skip_non_blank();
        }

        if string.is_empty() {
            return Err(ScanError::new(start_mark, "while scanning an anchor or alias, did not find expected alphabetic or numeric character"));
        }

        if alias {
            Ok(Token(start_mark, TokenType::Alias(string)))
        } else {
            Ok(Token(start_mark, TokenType::Anchor(string)))
        }
    }

    fn fetch_flow_collection_start(&mut self, tok: TokenType) -> ScanResult {
        // The indicators '[' and '{' may start a simple key.
        self.save_simple_key();

        self.roll_one_col_indent();
        self.increase_flow_level()?;

        self.allow_simple_key();

        let start_mark = self.mark;
        self.skip_non_blank();

        if tok == TokenType::FlowMappingStart {
            self.flow_mapping_started = true;
        }

        self.skip_ws_to_eol(SkipTabs::Yes)?;

        self.tokens.push_back(Token(start_mark, tok));
        Ok(())
    }

    fn fetch_flow_collection_end(&mut self, tok: TokenType) -> ScanResult {
        self.remove_simple_key()?;
        self.decrease_flow_level();

        self.disallow_simple_key();

        self.end_implicit_mapping(self.mark);

        let start_mark = self.mark;
        self.skip_non_blank();
        self.skip_ws_to_eol(SkipTabs::Yes)?;

        // A flow collection within a flow mapping can be a key. In that case, the value may be
        // adjacent to the `:`.
        // ```yaml
        // - [ {a: b}:value ]
        // ```
        if self.flow_level > 0 {
            self.adjacent_value_allowed_at = self.mark.index;
        }

        self.tokens.push_back(Token(start_mark, tok));
        Ok(())
    }

    /// Push the `FlowEntry` token and skip over the `,`.
    fn fetch_flow_entry(&mut self) -> ScanResult {
        self.remove_simple_key()?;
        self.allow_simple_key();

        self.end_implicit_mapping(self.mark);

        let start_mark = self.mark;
        self.skip_non_blank();
        self.skip_ws_to_eol(SkipTabs::Yes)?;

        self.tokens
            .push_back(Token(start_mark, TokenType::FlowEntry));
        Ok(())
    }

    fn increase_flow_level(&mut self) -> ScanResult {
        self.simple_keys.push(SimpleKey::new(Marker::new(0, 0, 0)));
        self.flow_level = self
            .flow_level
            .checked_add(1)
            .ok_or_else(|| ScanError::new(self.mark, "recursion limit exceeded"))?;
        Ok(())
    }

    fn decrease_flow_level(&mut self) {
        if self.flow_level > 0 {
            self.flow_level -= 1;
            self.simple_keys.pop().unwrap();
        }
    }

    /// Push the `Block*` token(s) and skip over the `-`.
    ///
    /// Add an indentation level and push a `BlockSequenceStart` token if needed, then push a
    /// `BlockEntry` token.
    /// This function only skips over the `-` and does not fetch the entry value.
    fn fetch_block_entry(&mut self) -> ScanResult {
        if self.flow_level > 0 {
            // - * only allowed in block
            return Err(ScanError::new(
                self.mark,
                r#""-" is only valid inside a block"#,
            ));
        }
        // Check if we are allowed to start a new entry.
        if !self.simple_key_allowed {
            return Err(ScanError::new(
                self.mark,
                "block sequence entries are not allowed in this context",
            ));
        }

        // ???, fixes test G9HC.
        if let Some(Token(mark, TokenType::Anchor(..) | TokenType::Tag(..))) = self.tokens.back() {
            if self.mark.col == 0 && mark.col == 0 && self.indent > -1 {
                return Err(ScanError::new(*mark, "invalid indentation for anchor"));
            }
        }

        // Skip over the `-`.
        let mark = self.mark;
        self.skip_non_blank();

        // generate BLOCK-SEQUENCE-START if indented
        self.roll_indent(mark.col, None, TokenType::BlockSequenceStart, mark);
        let found_tabs = self.skip_ws_to_eol(SkipTabs::Yes)?.found_tabs();
        self.lookahead(2);
        if found_tabs && self.buffer[0] == '-' && is_blank_or_breakz(self.buffer[1]) {
            return Err(ScanError::new(
                self.mark,
                "'-' must be followed by a valid YAML whitespace",
            ));
        }

        self.skip_ws_to_eol(SkipTabs::No)?;
        if is_break(self.look_ch()) || is_flow(self.ch()) {
            self.roll_one_col_indent();
        }

        self.remove_simple_key()?;
        self.allow_simple_key();

        self.tokens
            .push_back(Token(self.mark, TokenType::BlockEntry));

        Ok(())
    }

    fn fetch_document_indicator(&mut self, t: TokenType) -> ScanResult {
        self.unroll_indent(-1);
        self.remove_simple_key()?;
        self.disallow_simple_key();

        let mark = self.mark;

        self.skip_n_non_blank(3);

        self.tokens.push_back(Token(mark, t));
        Ok(())
    }

    fn fetch_block_scalar(&mut self, literal: bool) -> ScanResult {
        self.save_simple_key();
        self.allow_simple_key();
        let tok = self.scan_block_scalar(literal)?;

        self.tokens.push_back(tok);
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn scan_block_scalar(&mut self, literal: bool) -> Result<Token, ScanError> {
        let start_mark = self.mark;
        let mut chomping = Chomping::Clip;
        let mut increment: usize = 0;
        let mut indent: usize = 0;
        let mut trailing_blank: bool;
        let mut leading_blank: bool = false;
        let style = if literal {
            TScalarStyle::Literal
        } else {
            TScalarStyle::Folded
        };

        let mut string = String::new();
        let mut leading_break = String::new();
        let mut trailing_breaks = String::new();
        let mut chomping_break = String::new();

        // skip '|' or '>'
        self.skip_non_blank();
        self.unroll_non_block_indents();

        if self.look_ch() == '+' || self.ch() == '-' {
            if self.ch() == '+' {
                chomping = Chomping::Keep;
            } else {
                chomping = Chomping::Strip;
            }
            self.skip_non_blank();
            if is_digit(self.look_ch()) {
                if self.ch() == '0' {
                    return Err(ScanError::new(
                        start_mark,
                        "while scanning a block scalar, found an indentation indicator equal to 0",
                    ));
                }
                increment = (self.ch() as usize) - ('0' as usize);
                self.skip_non_blank();
            }
        } else if is_digit(self.ch()) {
            if self.ch() == '0' {
                return Err(ScanError::new(
                    start_mark,
                    "while scanning a block scalar, found an indentation indicator equal to 0",
                ));
            }

            increment = (self.ch() as usize) - ('0' as usize);
            self.skip_non_blank();
            self.lookahead(1);
            if self.ch() == '+' || self.ch() == '-' {
                if self.ch() == '+' {
                    chomping = Chomping::Keep;
                } else {
                    chomping = Chomping::Strip;
                }
                self.skip_non_blank();
            }
        }

        self.skip_ws_to_eol(SkipTabs::Yes)?;

        // Check if we are at the end of the line.
        if !is_breakz(self.look_ch()) {
            return Err(ScanError::new(
                start_mark,
                "while scanning a block scalar, did not find expected comment or line break",
            ));
        }

        if is_break(self.ch()) {
            self.lookahead(2);
            self.read_break(&mut chomping_break);
        }

        if self.look_ch() == '\t' {
            return Err(ScanError::new(
                start_mark,
                "a block scalar content cannot start with a tab",
            ));
        }

        if increment > 0 {
            indent = if self.indent >= 0 {
                (self.indent + increment as isize) as usize
            } else {
                increment
            }
        }

        // Scan the leading line breaks and determine the indentation level if needed.
        if indent == 0 {
            self.skip_block_scalar_first_line_indent(&mut indent, &mut trailing_breaks);
        } else {
            self.skip_block_scalar_indent(indent, &mut trailing_breaks);
        }

        // We have an end-of-stream with no content, e.g.:
        // ```yaml
        // - |+
        // ```
        if is_z(self.ch()) {
            let contents = match chomping {
                // We strip trailing linebreaks. Nothing remain.
                Chomping::Strip => String::new(),
                // There was no newline after the chomping indicator.
                _ if self.mark.line == start_mark.line() => String::new(),
                // We clip lines, and there was a newline after the chomping indicator.
                // All other breaks are ignored.
                Chomping::Clip => chomping_break,
                // We keep lines. There was a newline after the chomping indicator but nothing
                // else.
                Chomping::Keep if trailing_breaks.is_empty() => chomping_break,
                // Otherwise, the newline after chomping is ignored.
                Chomping::Keep => trailing_breaks,
            };
            return Ok(Token(start_mark, TokenType::Scalar(style, contents)));
        }

        if self.mark.col < indent && (self.mark.col as isize) > self.indent {
            return Err(ScanError::new(
                self.mark,
                "wrongly indented line in block scalar",
            ));
        }

        let mut line_buffer = String::with_capacity(100);
        let start_mark = self.mark;
        while self.mark.col == indent && !is_z(self.ch()) {
            if indent == 0 {
                self.lookahead(4);
                if self.next_is_document_end() {
                    break;
                }
            }

            // We are at the first content character of a content line.
            trailing_blank = is_blank(self.ch());
            if !literal && !leading_break.is_empty() && !leading_blank && !trailing_blank {
                string.push_str(&trailing_breaks);
                if trailing_breaks.is_empty() {
                    string.push(' ');
                }
            } else {
                string.push_str(&leading_break);
                string.push_str(&trailing_breaks);
            }

            leading_break.clear();
            trailing_breaks.clear();

            leading_blank = is_blank(self.ch());

            self.scan_block_scalar_content_line(&mut string, &mut line_buffer);

            // break on EOF
            if is_z(self.ch()) {
                break;
            }

            self.lookahead(2);
            self.read_break(&mut leading_break);

            // Eat the following indentation spaces and line breaks.
            self.skip_block_scalar_indent(indent, &mut trailing_breaks);
        }

        // Chomp the tail.
        if chomping != Chomping::Strip {
            string.push_str(&leading_break);
            // If we had reached an eof but the last character wasn't an end-of-line, check if the
            // last line was indented at least as the rest of the scalar, then we need to consider
            // there is a newline.
            if is_z(self.ch()) && self.mark.col >= indent.max(1) {
                string.push('\n');
            }
        }

        if chomping == Chomping::Keep {
            string.push_str(&trailing_breaks);
        }

        Ok(Token(start_mark, TokenType::Scalar(style, string)))
    }

    /// Retrieve the contents of the line, parsing it as a block scalar.
    ///
    /// The contents will be appended to `string`. `line_buffer` is used as a temporary buffer to
    /// store bytes before pushing them to `string` and thus avoiding reallocating more than
    /// necessary. `line_buffer` is assumed to be empty upon calling this function. It will be
    /// `clear`ed before the end of the function.
    ///
    /// This function assumed the first character to read is the first content character in the
    /// line. This function does not consume the line break character(s) after the line.
    fn scan_block_scalar_content_line(&mut self, string: &mut String, line_buffer: &mut String) {
        // Start by evaluating characters in the buffer.
        while !self.buffer.is_empty() && !is_breakz(self.ch()) {
            string.push(self.ch());
            // We may technically skip non-blank characters. However, the only distinction is
            // to determine what is leading whitespace and what is not. Here, we read the
            // contents of the line until either eof or a linebreak. We know we will not read
            // `self.leading_whitespace` until the end of the line, where it will be reset.
            // This allows us to call a slightly less expensive function.
            self.skip_blank();
        }

        // All characters that were in the buffer were consumed. We need to check if more
        // follow.
        if self.buffer.is_empty() {
            // We will read all consecutive non-breakz characters. We push them into a
            // temporary buffer. The main difference with going through `self.buffer` is that
            // characters are appended here as their real size (1B for ascii, or up to 4 bytes for
            // UTF-8). We can then use the internal `line_buffer` `Vec` to push data into `string`
            // (using `String::push_str`).
            let mut c = self.raw_read_ch();
            while !is_breakz(c) {
                line_buffer.push(c);
                c = self.raw_read_ch();
            }

            // Our last character read is stored in `c`. It is either an EOF or a break. In any
            // case, we need to push it back into `self.buffer` so it may be properly read
            // after. We must not insert it in `string`.
            self.buffer.push_back(c).unwrap();

            // We need to manually update our position; we haven't called a `skip` function.
            self.mark.col += line_buffer.len();
            self.mark.index += line_buffer.len();

            // We can now append our bytes to our `string`.
            string.reserve(line_buffer.as_bytes().len());
            string.push_str(line_buffer);
            // This clears the _contents_ without touching the _capacity_.
            line_buffer.clear();
        }
    }

    /// Skip the block scalar indentation and empty lines.
    fn skip_block_scalar_indent(&mut self, indent: usize, breaks: &mut String) {
        loop {
            // Consume all spaces. Tabs cannot be used as indentation.
            if indent < BUFFER_LEN - 2 {
                self.lookahead(BUFFER_LEN);
                while self.mark.col < indent && self.ch() == ' ' {
                    self.skip_blank();
                }
            } else {
                loop {
                    self.lookahead(BUFFER_LEN);
                    while !self.buffer.is_empty() && self.mark.col < indent && self.ch() == ' ' {
                        self.skip_blank();
                    }
                    // If we reached our indent, we can break. We must also break if we have
                    // reached content or EOF; that is, the buffer is not empty and the next
                    // character is not a space.
                    if self.mark.col == indent || (!self.buffer.is_empty() && self.ch() != ' ') {
                        break;
                    }
                }
                self.lookahead(2);
            }

            // If our current line is empty, skip over the break and continue looping.
            if is_break(self.ch()) {
                self.read_break(breaks);
            } else {
                // Otherwise, we have a content line. Return control.
                break;
            }
        }
    }

    /// Determine the indentation level for a block scalar from the first line of its contents.
    ///
    /// The function skips over whitespace-only lines and sets `indent` to the the longest
    /// whitespace line that was encountered.
    fn skip_block_scalar_first_line_indent(&mut self, indent: &mut usize, breaks: &mut String) {
        let mut max_indent = 0;
        loop {
            // Consume all spaces. Tabs cannot be used as indentation.
            while self.look_ch() == ' ' {
                self.skip_blank();
            }

            if self.mark.col > max_indent {
                max_indent = self.mark.col;
            }

            if is_break(self.ch()) {
                // If our current line is empty, skip over the break and continue looping.
                self.lookahead(2);
                self.read_break(breaks);
            } else {
                // Otherwise, we have a content line. Return control.
                break;
            }
        }

        // In case a yaml looks like:
        // ```yaml
        // |
        // foo
        // bar
        // ```
        // We need to set the indent to 0 and not 1. In all other cases, the indent must be at
        // least 1. When in the above example, `self.indent` will be set to -1.
        *indent = max_indent.max((self.indent + 1) as usize);
        if self.indent > 0 {
            *indent = (*indent).max(1);
        }
    }

    fn fetch_flow_scalar(&mut self, single: bool) -> ScanResult {
        self.save_simple_key();
        self.disallow_simple_key();

        let tok = self.scan_flow_scalar(single)?;

        // From spec: To ensure JSON compatibility, if a key inside a flow mapping is JSON-like,
        // YAML allows the following value to be specified adjacent to the “:”.
        self.skip_to_next_token()?;
        self.adjacent_value_allowed_at = self.mark.index;

        self.tokens.push_back(tok);
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn scan_flow_scalar(&mut self, single: bool) -> Result<Token, ScanError> {
        let start_mark = self.mark;

        let mut string = String::new();
        let mut leading_break = String::new();
        let mut trailing_breaks = String::new();
        let mut whitespaces = String::new();
        let mut leading_blanks;

        /* Eat the left quote. */
        self.skip_non_blank();

        loop {
            /* Check for a document indicator. */
            self.lookahead(4);

            if self.mark.col == 0
                && (((self.buffer[0] == '-') && (self.buffer[1] == '-') && (self.buffer[2] == '-'))
                    || ((self.buffer[0] == '.')
                        && (self.buffer[1] == '.')
                        && (self.buffer[2] == '.')))
                && is_blank_or_breakz(self.buffer[3])
            {
                return Err(ScanError::new(
                    start_mark,
                    "while scanning a quoted scalar, found unexpected document indicator",
                ));
            }

            if is_z(self.ch()) {
                return Err(ScanError::new(
                    start_mark,
                    "while scanning a quoted scalar, found unexpected end of stream",
                ));
            }

            if (self.mark.col as isize) < self.indent {
                return Err(ScanError::new(
                    start_mark,
                    "invalid indentation in quoted scalar",
                ));
            }

            leading_blanks = false;
            self.consume_flow_scalar_non_whitespace_chars(
                single,
                &mut string,
                &mut leading_blanks,
                &start_mark,
            )?;

            match self.look_ch() {
                '\'' if single => break,
                '"' if !single => break,
                _ => {}
            }

            // Consume blank characters.
            while is_blank(self.ch()) || is_break(self.ch()) {
                if is_blank(self.ch()) {
                    // Consume a space or a tab character.
                    if leading_blanks {
                        if self.ch() == '\t' && (self.mark.col as isize) < self.indent {
                            return Err(ScanError::new(
                                self.mark,
                                "tab cannot be used as indentation",
                            ));
                        }
                        self.skip_blank();
                    } else {
                        whitespaces.push(self.ch());
                        self.skip_blank();
                    }
                } else {
                    self.lookahead(2);
                    // Check if it is a first line break.
                    if leading_blanks {
                        self.read_break(&mut trailing_breaks);
                    } else {
                        whitespaces.clear();
                        self.read_break(&mut leading_break);
                        leading_blanks = true;
                    }
                }
                self.lookahead(1);
            }

            // Join the whitespaces or fold line breaks.
            if leading_blanks {
                if leading_break.is_empty() {
                    string.push_str(&leading_break);
                    string.push_str(&trailing_breaks);
                    trailing_breaks.clear();
                    leading_break.clear();
                } else {
                    if trailing_breaks.is_empty() {
                        string.push(' ');
                    } else {
                        string.push_str(&trailing_breaks);
                        trailing_breaks.clear();
                    }
                    leading_break.clear();
                }
            } else {
                string.push_str(&whitespaces);
                whitespaces.clear();
            }
        } // loop

        // Eat the right quote.
        self.skip_non_blank();
        // Ensure there is no invalid trailing content.
        self.skip_ws_to_eol(SkipTabs::Yes)?;
        match self.ch() {
            // These can be encountered in flow sequences or mappings.
            ',' | '}' | ']' if self.flow_level > 0 => {}
            // An end-of-line / end-of-stream is fine. No trailing content.
            c if is_breakz(c) => {}
            // ':' can be encountered if our scalar is a key.
            // Outside of flow contexts, keys cannot span multiple lines
            ':' if self.flow_level == 0 && start_mark.line == self.mark.line => {}
            // Inside a flow context, this is allowed.
            ':' if self.flow_level > 0 => {}
            _ => {
                return Err(ScanError::new(
                    self.mark,
                    "invalid trailing content after double-quoted scalar",
                ));
            }
        }

        let style = if single {
            TScalarStyle::SingleQuoted
        } else {
            TScalarStyle::DoubleQuoted
        };
        Ok(Token(start_mark, TokenType::Scalar(style, string)))
    }

    /// Consume successive non-whitespace characters from a flow scalar.
    ///
    /// This function resolves escape sequences and stops upon encountering a whitespace, the end
    /// of the stream or the closing character for the scalar (`'` for single quoted scalars, `"`
    /// for double quoted scalars).
    ///
    /// # Errors
    /// Return an error if an invalid escape sequence is found.
    fn consume_flow_scalar_non_whitespace_chars(
        &mut self,
        single: bool,
        string: &mut String,
        leading_blanks: &mut bool,
        start_mark: &Marker,
    ) -> Result<(), ScanError> {
        self.lookahead(2);
        while !is_blank_or_breakz(self.ch()) {
            match self.ch() {
                // Check for an escaped single quote.
                '\'' if self.buffer[1] == '\'' && single => {
                    string.push('\'');
                    self.skip_n_non_blank(2);
                }
                // Check for the right quote.
                '\'' if single => break,
                '"' if !single => break,
                // Check for an escaped line break.
                '\\' if !single && is_break(self.buffer[1]) => {
                    self.lookahead(3);
                    self.skip_non_blank();
                    self.skip_linebreak();
                    *leading_blanks = true;
                    break;
                }
                // Check for an escape sequence.
                '\\' if !single => {
                    string.push(self.resolve_flow_scalar_escape_sequence(start_mark)?);
                }
                c => {
                    string.push(c);
                    self.skip_non_blank();
                }
            }
            self.lookahead(2);
        }
        Ok(())
    }

    /// Escape the sequence we encounter in a flow scalar.
    ///
    /// `self.ch()` must point to the `\` starting the escape sequence.
    ///
    /// # Errors
    /// Return an error if an invalid escape sequence is found.
    fn resolve_flow_scalar_escape_sequence(
        &mut self,
        start_mark: &Marker,
    ) -> Result<char, ScanError> {
        let mut code_length = 0usize;
        let mut ret = '\0';

        match self.buffer[1] {
            '0' => ret = '\0',
            'a' => ret = '\x07',
            'b' => ret = '\x08',
            't' | '\t' => ret = '\t',
            'n' => ret = '\n',
            'v' => ret = '\x0b',
            'f' => ret = '\x0c',
            'r' => ret = '\x0d',
            'e' => ret = '\x1b',
            ' ' => ret = '\x20',
            '"' => ret = '"',
            '/' => ret = '/',
            '\\' => ret = '\\',
            // Unicode next line (#x85)
            'N' => ret = char::from_u32(0x85).unwrap(),
            // Unicode non-breaking space (#xA0)
            '_' => ret = char::from_u32(0xA0).unwrap(),
            // Unicode line separator (#x2028)
            'L' => ret = char::from_u32(0x2028).unwrap(),
            // Unicode paragraph separator (#x2029)
            'P' => ret = char::from_u32(0x2029).unwrap(),
            'x' => code_length = 2,
            'u' => code_length = 4,
            'U' => code_length = 8,
            _ => {
                return Err(ScanError::new(
                    *start_mark,
                    "while parsing a quoted scalar, found unknown escape character",
                ))
            }
        }
        self.skip_n_non_blank(2);

        // Consume an arbitrary escape code.
        if code_length > 0 {
            self.lookahead(code_length);
            let mut value = 0u32;
            for i in 0..code_length {
                if !is_hex(self.buffer[i]) {
                    return Err(ScanError::new(
                        *start_mark,
                        "while parsing a quoted scalar, did not find expected hexadecimal number",
                    ));
                }
                value = (value << 4) + as_hex(self.buffer[i]);
            }

            let Some(ch) = char::from_u32(value) else {
                return Err(ScanError::new(
                    *start_mark,
                    "while parsing a quoted scalar, found invalid Unicode character escape code",
                ));
            };
            ret = ch;

            self.skip_n_non_blank(code_length);
        }
        Ok(ret)
    }

    fn fetch_plain_scalar(&mut self) -> ScanResult {
        self.save_simple_key();
        self.disallow_simple_key();

        let tok = self.scan_plain_scalar()?;

        self.tokens.push_back(tok);
        Ok(())
    }

    /// Scan for a plain scalar.
    ///
    /// Plain scalars are the most readable but restricted style. They may span multiple lines in
    /// some contexts.
    #[allow(clippy::too_many_lines)]
    fn scan_plain_scalar(&mut self) -> Result<Token, ScanError> {
        self.unroll_non_block_indents();
        let indent = self.indent + 1;
        let start_mark = self.mark;

        if self.flow_level > 0 && (start_mark.col as isize) < indent {
            return Err(ScanError::new(
                start_mark,
                "invalid indentation in flow construct",
            ));
        }

        let mut string = String::with_capacity(32);
        let mut leading_break = String::with_capacity(32);
        let mut trailing_breaks = String::with_capacity(32);
        let mut whitespaces = String::with_capacity(32);

        loop {
            self.lookahead(4);
            if self.next_is_document_indicator() || self.ch() == '#' {
                break;
            }

            if self.flow_level > 0 && self.ch() == '-' && is_flow(self.buffer[1]) {
                return Err(ScanError::new(
                    self.mark,
                    "plain scalar cannot start with '-' followed by ,[]{}",
                ));
            }

            if !is_blank_or_breakz(self.ch()) && self.next_can_be_plain_scalar() {
                if self.leading_whitespace {
                    if leading_break.is_empty() {
                        string.push_str(&leading_break);
                        string.push_str(&trailing_breaks);
                        trailing_breaks.clear();
                        leading_break.clear();
                    } else {
                        if trailing_breaks.is_empty() {
                            string.push(' ');
                        } else {
                            string.push_str(&trailing_breaks);
                            trailing_breaks.clear();
                        }
                        leading_break.clear();
                    }
                    self.leading_whitespace = false;
                } else if !whitespaces.is_empty() {
                    string.push_str(&whitespaces);
                    whitespaces.clear();
                }

                // We can unroll the first iteration of the loop.
                string.push(self.ch());
                self.skip_non_blank();
                self.lookahead(2);

                // Add content non-blank characters to the scalar.
                while !is_blank_or_breakz(self.ch()) {
                    if !self.next_can_be_plain_scalar() {
                        break;
                    }

                    string.push(self.ch());
                    self.skip_non_blank();
                    self.lookahead(2);
                }
            }

            // We may reach the end of a plain scalar if:
            //  - We reach eof
            //  - We reach ": "
            //  - We find a flow character in a flow context
            if !(is_blank(self.ch()) || is_break(self.ch())) {
                break;
            }

            // Process blank characters.
            while is_blank(self.look_ch()) || is_break(self.ch()) {
                if is_blank(self.ch()) {
                    if !self.leading_whitespace {
                        whitespaces.push(self.ch());
                        self.skip_blank();
                    } else if (self.mark.col as isize) < indent && self.ch() == '\t' {
                        // Tabs in an indentation columns are allowed if and only if the line is
                        // empty. Skip to the end of the line.
                        self.skip_ws_to_eol(SkipTabs::Yes)?;
                        if !is_breakz(self.ch()) {
                            return Err(ScanError::new(
                                start_mark,
                                "while scanning a plain scalar, found a tab",
                            ));
                        }
                    } else {
                        self.skip_blank();
                    }
                } else {
                    self.lookahead(2);
                    // Check if it is a first line break
                    if self.leading_whitespace {
                        self.read_break(&mut trailing_breaks);
                    } else {
                        whitespaces.clear();
                        self.read_break(&mut leading_break);
                        self.leading_whitespace = true;
                    }
                }
            }

            // check indentation level
            if self.flow_level == 0 && (self.mark.col as isize) < indent {
                break;
            }
        }

        if self.leading_whitespace {
            self.allow_simple_key();
        }

        Ok(Token(
            start_mark,
            TokenType::Scalar(TScalarStyle::Plain, string),
        ))
    }

    fn fetch_key(&mut self) -> ScanResult {
        let start_mark = self.mark;
        if self.flow_level == 0 {
            // Check if we are allowed to start a new key (not necessarily simple).
            if !self.simple_key_allowed {
                return Err(ScanError::new(
                    self.mark,
                    "mapping keys are not allowed in this context",
                ));
            }
            self.roll_indent(
                start_mark.col,
                None,
                TokenType::BlockMappingStart,
                start_mark,
            );
        } else {
            // The parser, upon receiving a `Key`, will insert a `MappingStart` event.
            self.flow_mapping_started = true;
        }

        self.remove_simple_key()?;

        if self.flow_level == 0 {
            self.allow_simple_key();
        } else {
            self.disallow_simple_key();
        }

        self.skip_non_blank();
        self.skip_yaml_whitespace()?;
        if self.ch() == '\t' {
            return Err(ScanError::new(
                self.mark(),
                "tabs disallowed in this context",
            ));
        }
        self.tokens.push_back(Token(start_mark, TokenType::Key));
        Ok(())
    }

    /// Fetch a value from a mapping (after a `:`).
    fn fetch_value(&mut self) -> ScanResult {
        let sk = self.simple_keys.last().unwrap().clone();
        let start_mark = self.mark;
        self.implicit_flow_mapping = self.flow_level > 0 && !self.flow_mapping_started;

        // Skip over ':'.
        self.skip_non_blank();
        if self.look_ch() == '\t'
            && !self.skip_ws_to_eol(SkipTabs::Yes)?.has_valid_yaml_ws()
            && (self.ch() == '-' || is_alpha(self.ch()))
        {
            return Err(ScanError::new(
                self.mark,
                "':' must be followed by a valid YAML whitespace",
            ));
        }

        if sk.possible {
            // insert simple key
            let tok = Token(sk.mark, TokenType::Key);
            self.insert_token(sk.token_number - self.tokens_parsed, tok);
            if self.implicit_flow_mapping {
                if sk.mark.line < start_mark.line {
                    return Err(ScanError::new(
                        start_mark,
                        "illegal placement of ':' indicator",
                    ));
                }
                self.insert_token(
                    sk.token_number - self.tokens_parsed,
                    Token(self.mark, TokenType::FlowMappingStart),
                );
            }

            // Add the BLOCK-MAPPING-START token if needed.
            self.roll_indent(
                sk.mark.col,
                Some(sk.token_number),
                TokenType::BlockMappingStart,
                start_mark,
            );
            self.roll_one_col_indent();

            self.simple_keys.last_mut().unwrap().possible = false;
            self.disallow_simple_key();
        } else {
            if self.implicit_flow_mapping {
                self.tokens
                    .push_back(Token(self.mark, TokenType::FlowMappingStart));
            }
            // The ':' indicator follows a complex key.
            if self.flow_level == 0 {
                if !self.simple_key_allowed {
                    return Err(ScanError::new(
                        start_mark,
                        "mapping values are not allowed in this context",
                    ));
                }

                self.roll_indent(
                    start_mark.col,
                    None,
                    TokenType::BlockMappingStart,
                    start_mark,
                );
            }
            self.roll_one_col_indent();

            if self.flow_level == 0 {
                self.allow_simple_key();
            } else {
                self.disallow_simple_key();
            }
        }
        self.tokens.push_back(Token(start_mark, TokenType::Value));

        Ok(())
    }

    /// Add an indentation level to the stack with the given block token, if needed.
    ///
    /// An indentation level is added only if:
    ///   - We are not in a flow-style construct (which don't have indentation per-se).
    ///   - The current column is further indented than the last indent we have registered.
    fn roll_indent(&mut self, col: usize, number: Option<usize>, tok: TokenType, mark: Marker) {
        if self.flow_level > 0 {
            return;
        }

        // If the last indent was a non-block indent, remove it.
        // This means that we prepared an indent that we thought we wouldn't use, but realized just
        // now that it is a block indent.
        if self.indent <= col as isize {
            if let Some(indent) = self.indents.last() {
                if !indent.needs_block_end {
                    self.indent = indent.indent;
                    self.indents.pop();
                }
            }
        }

        if self.indent < col as isize {
            self.indents.push(Indent {
                indent: self.indent,
                needs_block_end: true,
            });
            self.indent = col as isize;
            let tokens_parsed = self.tokens_parsed;
            match number {
                Some(n) => self.insert_token(n - tokens_parsed, Token(mark, tok)),
                None => self.tokens.push_back(Token(mark, tok)),
            }
        }
    }

    /// Pop indentation levels from the stack as much as needed.
    ///
    /// Indentation levels are popped from the stack while they are further indented than `col`.
    /// If we are in a flow-style construct (which don't have indentation per-se), this function
    /// does nothing.
    fn unroll_indent(&mut self, col: isize) {
        if self.flow_level > 0 {
            return;
        }
        while self.indent > col {
            let indent = self.indents.pop().unwrap();
            self.indent = indent.indent;
            if indent.needs_block_end {
                self.tokens.push_back(Token(self.mark, TokenType::BlockEnd));
            }
        }
    }

    /// Add an indentation level of 1 column that does not start a block.
    ///
    /// See the documentation of [`Indent::needs_block_end`] for more details.
    /// An indentation is not added if we are inside a flow level or if the last indent is already
    /// a non-block indent.
    fn roll_one_col_indent(&mut self) {
        if self.flow_level == 0 && self.indents.last().map_or(false, |x| x.needs_block_end) {
            self.indents.push(Indent {
                indent: self.indent,
                needs_block_end: false,
            });
            self.indent += 1;
        }
    }

    /// Unroll all last indents created with [`Self::roll_one_col_indent`].
    fn unroll_non_block_indents(&mut self) {
        while let Some(indent) = self.indents.last() {
            if indent.needs_block_end {
                break;
            }
            self.indent = indent.indent;
            self.indents.pop();
        }
    }

    /// Mark the next token to be inserted as a potential simple key.
    fn save_simple_key(&mut self) {
        if self.simple_key_allowed {
            let required = self.flow_level == 0
                && self.indent == (self.mark.col as isize)
                && self.indents.last().unwrap().needs_block_end;
            let mut sk = SimpleKey::new(self.mark);
            sk.possible = true;
            sk.required = required;
            sk.token_number = self.tokens_parsed + self.tokens.len();

            self.simple_keys.pop();
            self.simple_keys.push(sk);
        }
    }

    fn remove_simple_key(&mut self) -> ScanResult {
        let last = self.simple_keys.last_mut().unwrap();
        if last.possible && last.required {
            return Err(ScanError::new(self.mark, "simple key expected"));
        }

        last.possible = false;
        Ok(())
    }

    /// Check whether the next characters may be part of a plain scalar.
    ///
    /// This function assumes we are not given a blankz character.
    // For some reason, `#[inline]` is not enough.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn next_can_be_plain_scalar(&self) -> bool {
        match self.ch() {
            // indicators can end a plain scalar, see 7.3.3. Plain Style
            ':' if is_blank_or_breakz(self.buffer[1])
                || (self.flow_level > 0 && is_flow(self.buffer[1])) =>
            {
                false
            }
            c if self.flow_level > 0 && is_flow(c) => false,
            _ => true,
        }
    }

    /// Return whether the scanner is inside a block but outside of a flow sequence.
    fn is_within_block(&self) -> bool {
        !self.indents.is_empty()
    }

    /// If an implicit mapping had started, end it.
    fn end_implicit_mapping(&mut self, mark: Marker) {
        if self.implicit_flow_mapping {
            self.implicit_flow_mapping = false;
            self.flow_mapping_started = false;
            self.tokens
                .push_back(Token(mark, TokenType::FlowMappingEnd));
        }
    }
}

/// Behavior to adopt regarding treating tabs as whitespace.
///
/// Although tab is a valid yaml whitespace, it doesn't always behave the same as a space.
#[derive(Copy, Clone, Eq, PartialEq)]
enum SkipTabs {
    /// Skip all tabs as whitespace.
    Yes,
    /// Don't skip any tab. Return from the function when encountering one.
    No,
    /// Return value from the function.
    Result(
        /// Whether tabs were encountered.
        bool,
        /// Whether at least 1 valid yaml whitespace has been encountered.
        bool,
    ),
}

impl SkipTabs {
    /// Whether tabs were found while skipping whitespace.
    ///
    /// This function must be called after a call to `skip_ws_to_eol`.
    fn found_tabs(self) -> bool {
        matches!(self, SkipTabs::Result(true, _))
    }

    /// Whether a valid YAML whitespace has been found in skipped-over content.
    ///
    /// This function must be called after a call to `skip_ws_to_eol`.
    fn has_valid_yaml_ws(self) -> bool {
        matches!(self, SkipTabs::Result(_, true))
    }
}

/// Chomping, how final line breaks and trailing empty lines are interpreted.
///
/// See YAML spec 8.1.1.2.
#[derive(PartialEq, Eq)]
pub enum Chomping {
    /// The final line break and any trailing empty lines are excluded.
    Strip,
    /// The final line break is preserved, but trailing empty lines are excluded.
    Clip,
    /// The final line break and trailing empty lines are included.
    Keep,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_is_anchor_char() {
        use super::is_anchor_char;
        assert!(is_anchor_char('x'));
    }
}
