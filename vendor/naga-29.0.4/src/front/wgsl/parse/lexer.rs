use super::{number::consume_number, Error, ExpectedToken, Result};
use crate::front::wgsl::error::NumberError;
use crate::front::wgsl::parse::directive::enable_extension::{
    EnableExtensions, ImplementedEnableExtension,
};
use crate::front::wgsl::parse::Number;
use crate::Span;

use alloc::{boxed::Box, vec::Vec};

pub type TokenSpan<'a> = (Token<'a>, Span);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token<'a> {
    /// A separator character: `:;,`, and `.` when not part of a numeric
    /// literal.
    Separator(char),

    /// A parenthesis-like character: `()[]{}`, and also `<>`.
    ///
    /// Note that `<>` representing template argument brackets are distinguished
    /// using WGSL's [template list discovery algorithm][tlda], and are returned
    /// as [`Token::TemplateArgsStart`] and [`Token::TemplateArgsEnd`]. That is,
    /// we use `Paren` for `<>` when they are *not* parens.
    ///
    /// [tlda]: https://gpuweb.github.io/gpuweb/wgsl/#template-list-discovery
    Paren(char),

    /// The attribute introduction character `@`.
    Attribute,

    /// A numeric literal, either integral or floating-point, including any
    /// type suffix.
    Number(core::result::Result<Number, NumberError>),

    /// An identifier, possibly a reserved word.
    Word(&'a str),

    /// A miscellaneous single-character operator, like an arithmetic unary or
    /// binary operator. This includes `=`, for assignment and initialization.
    Operation(char),

    /// Certain multi-character logical operators: `!=`, `==`, `&&`,
    /// `||`, `<=` and `>=`. The value gives the operator's first
    /// character.
    ///
    /// For `<` and `>` operators, see [`Token::Paren`].
    LogicalOperation(char),

    /// A shift operator: `>>` or `<<`.
    ShiftOperation(char),

    /// A compound assignment operator like `+=`.
    ///
    /// When the given character is `<` or `>`, those represent the left shift
    /// and right shift assignment operators, `<<=` and `>>=`.
    AssignmentOperation(char),

    /// The `++` operator.
    IncrementOperation,

    /// The `--` operator.
    DecrementOperation,

    /// The `->` token.
    Arrow,

    /// A `<` representing the start of a template argument list, according to
    /// WGSL's [template list discovery algorithm][tlda].
    ///
    /// [tlda]: https://gpuweb.github.io/gpuweb/wgsl/#template-list-discovery
    TemplateArgsStart,

    /// A `>` representing the end of a template argument list, according to
    /// WGSL's [template list discovery algorithm][tlda].
    ///
    /// [tlda]: https://gpuweb.github.io/gpuweb/wgsl/#template-list-discovery
    TemplateArgsEnd,

    /// A character that does not represent a legal WGSL token.
    Unknown(char),

    /// Comment or whitespace.
    Trivia,

    /// A doc comment, beginning with `///` or `/**`.
    DocComment(&'a str),

    /// A module-level doc comment, beginning with `//!` or `/*!`.
    ModuleDocComment(&'a str),

    /// The end of the input.
    End,
}

fn consume_any(input: &str, what: impl Fn(char) -> bool) -> (&str, &str) {
    let pos = input.find(|c| !what(c)).unwrap_or(input.len());
    input.split_at(pos)
}

struct UnclosedCandidate {
    index: usize,
    depth: usize,
}

/// Produce at least one token, distinguishing [template lists] from other uses
/// of `<` and `>`.
///
/// Consume one or more tokens from `input` and store them in `tokens`, updating
/// `input` to refer to the remaining text. Apply WGSL's [template list
/// discovery algorithm] to decide what sort of tokens `<` and `>` characters in
/// the input actually represent.
///
/// Store the tokens in `tokens` in the *reverse* of the order they appear in
/// the text, such that the caller can pop from the end of the vector to see the
/// tokens in textual order.
///
/// The `tokens` vector must be empty on entry. The idea is for the caller to
/// use it as a buffer of unconsumed tokens, and call this function to refill it
/// when it's empty.
///
/// The `source` argument must be the whole original source code, used to
/// compute spans.
///
/// If `ignore_doc_comments` is true, then doc comments are returned as
/// [`Token::Trivia`], like ordinary comments.
///
/// [template lists]: https://gpuweb.github.io/gpuweb/wgsl/#template-lists-sec
/// [template list discovery algorithm]: https://gpuweb.github.io/gpuweb/wgsl/#template-list-discovery
fn discover_template_lists<'a>(
    tokens: &mut Vec<(TokenSpan<'a>, &'a str)>,
    source: &'a str,
    mut input: &'a str,
    ignore_doc_comments: bool,
) {
    assert!(tokens.is_empty());

    let mut looking_for_template_start = false;
    let mut pending: Vec<UnclosedCandidate> = Vec::new();

    // Current nesting depth of `()` and `[]` brackets. (`{}` brackets
    // exit all template list processing.)
    let mut depth = 0;

    fn pop_until(pending: &mut Vec<UnclosedCandidate>, depth: usize) {
        while pending
            .last()
            .map(|candidate| candidate.depth >= depth)
            .unwrap_or(false)
        {
            pending.pop();
        }
    }

    loop {
        // Decide whether `consume_token` should treat a `>` character as
        // `TemplateArgsEnd`, without considering the characters that follow.
        //
        // This condition matches the one that determines whether the spec's
        // template list discovery algorithm looks past a `>` character for a
        // `=`. By passing this flag to `consume_token`, we ensure it follows
        // that behavior.
        let waiting_for_template_end = pending
            .last()
            .is_some_and(|candidate| candidate.depth == depth);

        // Ask `consume_token` for the next token and add it to `tokens`, along
        // with its span.
        //
        // This means that `<` enters the buffer as `Token::Paren('<')`, the
        // ordinary comparison operator. We'll change that to
        // `Token::TemplateArgsStart` later if appropriate.
        let (token, rest) = consume_token(input, waiting_for_template_end, ignore_doc_comments);
        let span = Span::from(source.len() - input.len()..source.len() - rest.len());
        tokens.push(((token, span), rest));
        input = rest;

        // Since `consume_token` treats `<<=`, `<<` and `<=` as operators, not
        // `Token::Paren`, that takes care of the WGSL algorithm's post-'<' lookahead
        // for us.
        match token {
            Token::Word(_) => {
                looking_for_template_start = true;
                continue;
            }
            Token::Trivia | Token::DocComment(_) | Token::ModuleDocComment(_)
                if looking_for_template_start =>
            {
                continue;
            }
            Token::Paren('<') if looking_for_template_start => {
                pending.push(UnclosedCandidate {
                    index: tokens.len() - 1,
                    depth,
                });
            }
            Token::TemplateArgsEnd => {
                // The `consume_token` function only returns `TemplateArgsEnd`
                // if `waiting_for_template_end` is true, so we know `pending`
                // has a top entry at the appropriate depth.
                //
                // Find the matching `<` token and change its type to
                // `TemplateArgsStart`.
                let candidate = pending.pop().unwrap();
                let &mut ((ref mut token, _), _) = tokens.get_mut(candidate.index).unwrap();
                *token = Token::TemplateArgsStart;
            }
            Token::Paren('(' | '[') => {
                depth += 1;
            }
            Token::Paren(')' | ']') => {
                pop_until(&mut pending, depth);
                depth = depth.saturating_sub(1);
            }
            Token::Operation('=') | Token::Separator(':' | ';') | Token::Paren('{') => {
                pending.clear();
                depth = 0;
            }
            Token::LogicalOperation('&') | Token::LogicalOperation('|') => {
                pop_until(&mut pending, depth);
            }
            Token::End => break,
            _ => {}
        }

        looking_for_template_start = false;

        // The WGSL spec's template list discovery algorithm processes the
        // entire source at once, but Naga would rather limit its lookahead to
        // the actual text that could possibly be a template parameter list.
        // This is usually less than a line.
        if pending.is_empty() {
            break;
        }
    }

    tokens.reverse();
}

/// Return the token at the start of `input`.
///
/// The `waiting_for_template_end` flag enables some special handling to help out
/// `discover_template_lists`:
///
/// - If `waiting_for_template_end` is `true`, then return text starting with
///   '>` as [`Token::TemplateArgsEnd`] and consume only the `>` character,
///   regardless of what characters follow it. This is required by the [template
///   list discovery algorithm][tlda] when the `>` would end a template argument list.
///
/// - If `waiting_for_template_end` is false, recognize multi-character tokens
///   beginning with `>` as usual.
///
/// If `ignore_doc_comments` is true, then doc comments are returned as
/// [`Token::Trivia`], like ordinary comments.
///
/// [tlda]: https://gpuweb.github.io/gpuweb/wgsl/#template-list-discovery
fn consume_token(
    input: &str,
    waiting_for_template_end: bool,
    ignore_doc_comments: bool,
) -> (Token<'_>, &str) {
    let mut chars = input.chars();
    let cur = match chars.next() {
        Some(c) => c,
        None => return (Token::End, ""),
    };
    match cur {
        ':' | ';' | ',' => (Token::Separator(cur), chars.as_str()),
        '.' => {
            let og_chars = chars.as_str();
            match chars.next() {
                Some('0'..='9') => consume_number(input),
                _ => (Token::Separator(cur), og_chars),
            }
        }
        '@' => (Token::Attribute, chars.as_str()),
        '(' | ')' | '{' | '}' | '[' | ']' => (Token::Paren(cur), chars.as_str()),
        '<' | '>' => {
            let og_chars = chars.as_str();
            if cur == '>' && waiting_for_template_end {
                return (Token::TemplateArgsEnd, og_chars);
            }
            match chars.next() {
                Some('=') => (Token::LogicalOperation(cur), chars.as_str()),
                Some(c) if c == cur => {
                    let og_chars = chars.as_str();
                    match chars.next() {
                        Some('=') => (Token::AssignmentOperation(cur), chars.as_str()),
                        _ => (Token::ShiftOperation(cur), og_chars),
                    }
                }
                _ => (Token::Paren(cur), og_chars),
            }
        }
        '0'..='9' => consume_number(input),
        '/' => {
            let og_chars = chars.as_str();
            match chars.next() {
                Some('/') => {
                    let mut input_chars = input.char_indices();
                    let doc_comment_end = input_chars
                        .find_map(|(index, c)| is_comment_end(c).then_some(index))
                        .unwrap_or(input.len());
                    let token = match chars.next() {
                        Some('/') if !ignore_doc_comments => {
                            Token::DocComment(&input[..doc_comment_end])
                        }
                        Some('!') if !ignore_doc_comments => {
                            Token::ModuleDocComment(&input[..doc_comment_end])
                        }
                        _ => Token::Trivia,
                    };
                    (token, input_chars.as_str())
                }
                Some('*') => {
                    let next_c = chars.next();

                    enum CommentType {
                        Doc,
                        ModuleDoc,
                        Normal,
                    }
                    let comment_type = match next_c {
                        Some('*') if !ignore_doc_comments => CommentType::Doc,
                        Some('!') if !ignore_doc_comments => CommentType::ModuleDoc,
                        _ => CommentType::Normal,
                    };

                    let mut depth = 1;
                    let mut prev = next_c;

                    for c in &mut chars {
                        match (prev, c) {
                            (Some('*'), '/') => {
                                prev = None;
                                depth -= 1;
                                if depth == 0 {
                                    let rest = chars.as_str();
                                    let token = match comment_type {
                                        CommentType::Doc => {
                                            let doc_comment_end = input.len() - rest.len();
                                            Token::DocComment(&input[..doc_comment_end])
                                        }
                                        CommentType::ModuleDoc => {
                                            let doc_comment_end = input.len() - rest.len();
                                            Token::ModuleDocComment(&input[..doc_comment_end])
                                        }
                                        CommentType::Normal => Token::Trivia,
                                    };
                                    return (token, rest);
                                }
                            }
                            (Some('/'), '*') => {
                                prev = None;
                                depth += 1;
                            }
                            _ => {
                                prev = Some(c);
                            }
                        }
                    }

                    (Token::End, "")
                }
                Some('=') => (Token::AssignmentOperation(cur), chars.as_str()),
                _ => (Token::Operation(cur), og_chars),
            }
        }
        '-' => {
            let og_chars = chars.as_str();
            match chars.next() {
                Some('>') => (Token::Arrow, chars.as_str()),
                Some('-') => (Token::DecrementOperation, chars.as_str()),
                Some('=') => (Token::AssignmentOperation(cur), chars.as_str()),
                _ => (Token::Operation(cur), og_chars),
            }
        }
        '+' => {
            let og_chars = chars.as_str();
            match chars.next() {
                Some('+') => (Token::IncrementOperation, chars.as_str()),
                Some('=') => (Token::AssignmentOperation(cur), chars.as_str()),
                _ => (Token::Operation(cur), og_chars),
            }
        }
        '*' | '%' | '^' => {
            let og_chars = chars.as_str();
            match chars.next() {
                Some('=') => (Token::AssignmentOperation(cur), chars.as_str()),
                _ => (Token::Operation(cur), og_chars),
            }
        }
        '~' => (Token::Operation(cur), chars.as_str()),
        '=' | '!' => {
            let og_chars = chars.as_str();
            match chars.next() {
                Some('=') => (Token::LogicalOperation(cur), chars.as_str()),
                _ => (Token::Operation(cur), og_chars),
            }
        }
        '&' | '|' => {
            let og_chars = chars.as_str();
            match chars.next() {
                Some(c) if c == cur => (Token::LogicalOperation(cur), chars.as_str()),
                Some('=') => (Token::AssignmentOperation(cur), chars.as_str()),
                _ => (Token::Operation(cur), og_chars),
            }
        }
        _ if is_blankspace(cur) => {
            let (_, rest) = consume_any(input, is_blankspace);
            (Token::Trivia, rest)
        }
        _ if is_word_start(cur) => {
            let (word, rest) = consume_any(input, is_word_part);
            (Token::Word(word), rest)
        }
        _ => (Token::Unknown(cur), chars.as_str()),
    }
}

/// Returns whether or not a char is a comment end
/// (Unicode Pattern_White_Space excluding U+0020, U+0009, U+200E and U+200F)
/// <https://www.w3.org/TR/WGSL/#line-break>
const fn is_comment_end(c: char) -> bool {
    match c {
        '\u{000a}'..='\u{000d}' | '\u{0085}' | '\u{2028}' | '\u{2029}' => true,
        _ => false,
    }
}

/// Returns whether or not a char is a blankspace (Unicode Pattern_White_Space)
const fn is_blankspace(c: char) -> bool {
    match c {
        '\u{0020}'
        | '\u{0009}'..='\u{000d}'
        | '\u{0085}'
        | '\u{200e}'
        | '\u{200f}'
        | '\u{2028}'
        | '\u{2029}' => true,
        _ => false,
    }
}

/// Returns whether or not a char is a word start (Unicode XID_Start + '_')
fn is_word_start(c: char) -> bool {
    c == '_' || unicode_ident::is_xid_start(c)
}

/// Returns whether or not a char is a word part (Unicode XID_Continue)
fn is_word_part(c: char) -> bool {
    unicode_ident::is_xid_continue(c)
}

pub(in crate::front::wgsl) struct Lexer<'a> {
    /// The remaining unconsumed input.
    input: &'a str,

    /// The full original source code.
    ///
    /// We compare `input` against this to compute the lexer's current offset in
    /// the source.
    pub(in crate::front::wgsl) source: &'a str,

    /// The byte offset of the end of the most recently returned non-trivia
    /// token.
    ///
    /// This is consulted by the `span_from` function, for finding the
    /// end of the span for larger structures like expressions or
    /// statements.
    last_end_offset: usize,

    /// A stack of unconsumed tokens to which template list discovery has been
    /// applied.
    ///
    /// This is a stack: the next token is at the *end* of the vector, not the
    /// start. So tokens appear here in the reverse of the order they appear in
    /// the source.
    ///
    /// This doesn't contain the whole source, only those tokens produced by
    /// [`discover_template_lists`]'s look-ahead, or that have been produced by
    /// other look-ahead functions like `peek` and `next_if`. When this is empty,
    /// we call [`discover_template_lists`] to get more.
    tokens: Vec<(TokenSpan<'a>, &'a str)>,

    /// Whether or not to ignore doc comments.
    /// If `true`, doc comments are treated as [`Token::Trivia`].
    ignore_doc_comments: bool,

    /// The set of [enable-extensions] present in the module, determined in a pre-pass.
    ///
    /// [enable-extensions]: https://gpuweb.github.io/gpuweb/wgsl/#enable-extensions-sec
    pub(in crate::front::wgsl) enable_extensions: EnableExtensions,
}

impl<'a> Lexer<'a> {
    pub(in crate::front::wgsl) const fn new(input: &'a str, ignore_doc_comments: bool) -> Self {
        Lexer {
            input,
            source: input,
            last_end_offset: 0,
            tokens: Vec::new(),
            enable_extensions: EnableExtensions::empty(),
            ignore_doc_comments,
        }
    }

    /// Check that `extension` is enabled in `self`.
    pub(in crate::front::wgsl) fn require_enable_extension(
        &self,
        extension: ImplementedEnableExtension,
        span: Span,
    ) -> Result<'static, ()> {
        self.enable_extensions.require(extension, span)
    }

    /// Calls the function with a lexer and returns the result of the function as well as the span for everything the function parsed
    ///
    /// # Examples
    /// ```ignore
    /// let lexer = Lexer::new("5");
    /// let (value, span) = lexer.capture_span(Lexer::next_uint_literal);
    /// assert_eq!(value, 5);
    /// ```
    #[inline]
    pub fn capture_span<T, E>(
        &mut self,
        inner: impl FnOnce(&mut Self) -> core::result::Result<T, E>,
    ) -> core::result::Result<(T, Span), E> {
        let start = self.current_byte_offset();
        let res = inner(self)?;
        let end = self.current_byte_offset();
        Ok((res, Span::from(start..end)))
    }

    pub(in crate::front::wgsl) fn start_byte_offset(&mut self) -> usize {
        loop {
            // Eat all trivia because `next` doesn't eat trailing trivia.
            let (token, rest) = consume_token(self.input, false, true);
            if let Token::Trivia = token {
                self.input = rest;
            } else {
                return self.current_byte_offset();
            }
        }
    }

    /// Collect all module doc comments until a non doc token is found.
    pub(in crate::front::wgsl) fn accumulate_module_doc_comments(&mut self) -> Vec<&'a str> {
        let mut doc_comments = Vec::new();
        loop {
            // ignore blankspace
            self.input = consume_any(self.input, is_blankspace).1;

            let (token, rest) = consume_token(self.input, false, self.ignore_doc_comments);
            if let Token::ModuleDocComment(doc_comment) = token {
                self.input = rest;
                doc_comments.push(doc_comment);
            } else {
                return doc_comments;
            }
        }
    }

    /// Collect all doc comments until a non doc token is found.
    pub(in crate::front::wgsl) fn accumulate_doc_comments(&mut self) -> Vec<&'a str> {
        let mut doc_comments = Vec::new();
        loop {
            // ignore blankspace
            self.input = consume_any(self.input, is_blankspace).1;

            let (token, rest) = consume_token(self.input, false, self.ignore_doc_comments);
            if let Token::DocComment(doc_comment) = token {
                self.input = rest;
                doc_comments.push(doc_comment);
            } else {
                return doc_comments;
            }
        }
    }

    const fn current_byte_offset(&self) -> usize {
        self.source.len() - self.input.len()
    }

    pub(in crate::front::wgsl) fn span_from(&self, offset: usize) -> Span {
        Span::from(offset..self.last_end_offset)
    }
    pub(in crate::front::wgsl) fn span_with_start(&self, span: Span) -> Span {
        span.until(&Span::from(0..self.last_end_offset))
    }

    /// Return the next non-whitespace token from `self`.
    ///
    /// Assume we are a parse state where bit shift operators may
    /// occur, but not angle brackets.
    #[must_use]
    pub(in crate::front::wgsl) fn next(&mut self) -> TokenSpan<'a> {
        self.next_impl(true)
    }

    #[cfg(test)]
    pub fn next_with_unignored_doc_comments(&mut self) -> TokenSpan<'a> {
        self.next_impl(false)
    }

    /// Return the next non-whitespace token from `self`, with a span.
    fn next_impl(&mut self, ignore_doc_comments: bool) -> TokenSpan<'a> {
        loop {
            if self.tokens.is_empty() {
                discover_template_lists(
                    &mut self.tokens,
                    self.source,
                    self.input,
                    ignore_doc_comments || self.ignore_doc_comments,
                );
            }
            assert!(!self.tokens.is_empty());
            let (token, rest) = self.tokens.pop().unwrap();

            self.input = rest;
            self.last_end_offset = self.current_byte_offset();

            match token.0 {
                Token::Trivia => {}
                _ => return token,
            }
        }
    }

    #[must_use]
    pub(in crate::front::wgsl) fn peek(&mut self) -> TokenSpan<'a> {
        let input = self.input;
        let last_end_offset = self.last_end_offset;
        let token = self.next();
        self.tokens.push((token, self.input));
        self.input = input;
        self.last_end_offset = last_end_offset;
        token
    }

    /// If the next token matches it's consumed and true is returned
    pub(in crate::front::wgsl) fn next_if(&mut self, what: Token<'_>) -> bool {
        let input = self.input;
        let last_end_offset = self.last_end_offset;
        let token = self.next();
        if token.0 == what {
            true
        } else {
            self.tokens.push((token, self.input));
            self.input = input;
            self.last_end_offset = last_end_offset;
            false
        }
    }

    pub(in crate::front::wgsl) fn expect_span(&mut self, expected: Token<'a>) -> Result<'a, Span> {
        let next = self.next();
        if next.0 == expected {
            Ok(next.1)
        } else {
            Err(Box::new(Error::Unexpected(
                next.1,
                ExpectedToken::Token(expected),
            )))
        }
    }

    pub(in crate::front::wgsl) fn expect(&mut self, expected: Token<'a>) -> Result<'a, ()> {
        self.expect_span(expected)?;
        Ok(())
    }

    pub(in crate::front::wgsl) fn next_ident_with_span(&mut self) -> Result<'a, (&'a str, Span)> {
        match self.next() {
            (Token::Word("_"), span) => Err(Box::new(Error::InvalidIdentifierUnderscore(span))),
            (Token::Word(word), span) => {
                if word.starts_with("__") {
                    Err(Box::new(Error::ReservedIdentifierPrefix(span)))
                } else {
                    Ok((word, span))
                }
            }
            (_, span) => Err(Box::new(Error::Unexpected(span, ExpectedToken::Identifier))),
        }
    }

    pub(in crate::front::wgsl) fn next_ident(&mut self) -> Result<'a, super::ast::Ident<'a>> {
        self.next_ident_with_span()
            .and_then(|(word, span)| Self::word_as_ident(word, span))
            .map(|(name, span)| super::ast::Ident { name, span })
    }

    fn word_as_ident(word: &'a str, span: Span) -> Result<'a, (&'a str, Span)> {
        if crate::keywords::wgsl::RESERVED.contains(&word) {
            Err(Box::new(Error::ReservedKeyword(span)))
        } else {
            Ok((word, span))
        }
    }

    pub(in crate::front::wgsl) fn open_arguments(&mut self) -> Result<'a, ()> {
        self.expect(Token::Paren('('))
    }

    pub(in crate::front::wgsl) fn next_argument(&mut self) -> Result<'a, bool> {
        let paren = Token::Paren(')');
        if self.next_if(Token::Separator(',')) {
            Ok(!self.next_if(paren))
        } else {
            self.expect(paren).map(|()| false)
        }
    }
}

#[cfg(test)]
#[track_caller]
fn sub_test(source: &str, expected_tokens: &[Token]) {
    sub_test_with(true, source, expected_tokens);
}

#[cfg(test)]
#[track_caller]
fn sub_test_with_and_without_doc_comments(source: &str, expected_tokens: &[Token]) {
    sub_test_with(false, source, expected_tokens);
    sub_test_with(
        true,
        source,
        expected_tokens
            .iter()
            .filter(|v| !matches!(**v, Token::DocComment(_) | Token::ModuleDocComment(_)))
            .cloned()
            .collect::<Vec<_>>()
            .as_slice(),
    );
}

#[cfg(test)]
#[track_caller]
fn sub_test_with(ignore_doc_comments: bool, source: &str, expected_tokens: &[Token]) {
    let mut lex = Lexer::new(source, ignore_doc_comments);
    for &token in expected_tokens {
        assert_eq!(lex.next_with_unignored_doc_comments().0, token);
    }
    assert_eq!(lex.next().0, Token::End);
}

#[test]
fn test_numbers() {
    use half::f16;
    // WGSL spec examples //

    // decimal integer
    sub_test(
        "0x123 0X123u 1u 123 0 0i 0x3f",
        &[
            Token::Number(Ok(Number::AbstractInt(291))),
            Token::Number(Ok(Number::U32(291))),
            Token::Number(Ok(Number::U32(1))),
            Token::Number(Ok(Number::AbstractInt(123))),
            Token::Number(Ok(Number::AbstractInt(0))),
            Token::Number(Ok(Number::I32(0))),
            Token::Number(Ok(Number::AbstractInt(63))),
        ],
    );
    // decimal floating point
    sub_test(
        "0.e+4f 01. .01 12.34 .0f 0h 1e-3 0xa.fp+2 0x1P+4f 0X.3 0x3p+2h 0X1.fp-4 0x3.2p+2h",
        &[
            Token::Number(Ok(Number::F32(0.))),
            Token::Number(Ok(Number::AbstractFloat(1.))),
            Token::Number(Ok(Number::AbstractFloat(0.01))),
            Token::Number(Ok(Number::AbstractFloat(12.34))),
            Token::Number(Ok(Number::F32(0.))),
            Token::Number(Ok(Number::F16(f16::from_f32(0.)))),
            Token::Number(Ok(Number::AbstractFloat(0.001))),
            Token::Number(Ok(Number::AbstractFloat(43.75))),
            Token::Number(Ok(Number::F32(16.))),
            Token::Number(Ok(Number::AbstractFloat(0.1875))),
            // https://github.com/gfx-rs/wgpu/issues/7046
            Token::Number(Err(NumberError::NotRepresentable)), // Should be 0.75
            Token::Number(Ok(Number::AbstractFloat(0.12109375))),
            // https://github.com/gfx-rs/wgpu/issues/7046
            Token::Number(Err(NumberError::NotRepresentable)), // Should be 12.5
        ],
    );

    // MIN / MAX //

    // min / max decimal integer
    sub_test(
        "0i 2147483647i 2147483648i",
        &[
            Token::Number(Ok(Number::I32(0))),
            Token::Number(Ok(Number::I32(i32::MAX))),
            Token::Number(Err(NumberError::NotRepresentable)),
        ],
    );
    // min / max decimal unsigned integer
    sub_test(
        "0u 4294967295u 4294967296u",
        &[
            Token::Number(Ok(Number::U32(u32::MIN))),
            Token::Number(Ok(Number::U32(u32::MAX))),
            Token::Number(Err(NumberError::NotRepresentable)),
        ],
    );

    // min / max hexadecimal signed integer
    sub_test(
        "0x0i 0x7FFFFFFFi 0x80000000i",
        &[
            Token::Number(Ok(Number::I32(0))),
            Token::Number(Ok(Number::I32(i32::MAX))),
            Token::Number(Err(NumberError::NotRepresentable)),
        ],
    );
    // min / max hexadecimal unsigned integer
    sub_test(
        "0x0u 0xFFFFFFFFu 0x100000000u",
        &[
            Token::Number(Ok(Number::U32(u32::MIN))),
            Token::Number(Ok(Number::U32(u32::MAX))),
            Token::Number(Err(NumberError::NotRepresentable)),
        ],
    );

    // min/max decimal abstract int
    sub_test(
        "0 9223372036854775807 9223372036854775808",
        &[
            Token::Number(Ok(Number::AbstractInt(0))),
            Token::Number(Ok(Number::AbstractInt(i64::MAX))),
            Token::Number(Err(NumberError::NotRepresentable)),
        ],
    );

    // min/max hexadecimal abstract int
    sub_test(
        "0 0x7fffffffffffffff 0x8000000000000000",
        &[
            Token::Number(Ok(Number::AbstractInt(0))),
            Token::Number(Ok(Number::AbstractInt(i64::MAX))),
            Token::Number(Err(NumberError::NotRepresentable)),
        ],
    );

    /// ≈ 2^-126 * 2^−23 (= 2^−149)
    const SMALLEST_POSITIVE_SUBNORMAL_F32: f32 = 1e-45;
    /// ≈ 2^-126 * (1 − 2^−23)
    const LARGEST_SUBNORMAL_F32: f32 = 1.1754942e-38;
    /// ≈ 2^-126
    const SMALLEST_POSITIVE_NORMAL_F32: f32 = f32::MIN_POSITIVE;
    /// ≈ 1 − 2^−24
    const LARGEST_F32_LESS_THAN_ONE: f32 = 0.99999994;
    /// ≈ 1 + 2^−23
    const SMALLEST_F32_LARGER_THAN_ONE: f32 = 1.0000001;
    /// ≈ 2^127 * (2 − 2^−23)
    const LARGEST_NORMAL_F32: f32 = f32::MAX;

    // decimal floating point
    sub_test(
        "1e-45f 1.1754942e-38f 1.17549435e-38f 0.99999994f 1.0000001f 3.40282347e+38f",
        &[
            Token::Number(Ok(Number::F32(SMALLEST_POSITIVE_SUBNORMAL_F32))),
            Token::Number(Ok(Number::F32(LARGEST_SUBNORMAL_F32))),
            Token::Number(Ok(Number::F32(SMALLEST_POSITIVE_NORMAL_F32))),
            Token::Number(Ok(Number::F32(LARGEST_F32_LESS_THAN_ONE))),
            Token::Number(Ok(Number::F32(SMALLEST_F32_LARGER_THAN_ONE))),
            Token::Number(Ok(Number::F32(LARGEST_NORMAL_F32))),
        ],
    );
    sub_test(
        "3.40282367e+38f",
        &[
            Token::Number(Err(NumberError::NotRepresentable)), // ≈ 2^128
        ],
    );

    // hexadecimal floating point
    sub_test(
        "0x1p-149f 0x7FFFFFp-149f 0x1p-126f 0xFFFFFFp-24f 0x800001p-23f 0xFFFFFFp+104f",
        &[
            Token::Number(Ok(Number::F32(SMALLEST_POSITIVE_SUBNORMAL_F32))),
            Token::Number(Ok(Number::F32(LARGEST_SUBNORMAL_F32))),
            Token::Number(Ok(Number::F32(SMALLEST_POSITIVE_NORMAL_F32))),
            Token::Number(Ok(Number::F32(LARGEST_F32_LESS_THAN_ONE))),
            Token::Number(Ok(Number::F32(SMALLEST_F32_LARGER_THAN_ONE))),
            Token::Number(Ok(Number::F32(LARGEST_NORMAL_F32))),
        ],
    );
    sub_test(
        "0x1p128f 0x1.000001p0f",
        &[
            Token::Number(Err(NumberError::NotRepresentable)), // = 2^128
            Token::Number(Err(NumberError::NotRepresentable)),
        ],
    );
}

#[test]
fn double_floats() {
    sub_test(
        "0x1.2p4lf 0x1p8lf 0.0625lf 625e-4lf 10lf 10l",
        &[
            Token::Number(Ok(Number::F64(18.0))),
            Token::Number(Ok(Number::F64(256.0))),
            Token::Number(Ok(Number::F64(0.0625))),
            Token::Number(Ok(Number::F64(0.0625))),
            Token::Number(Ok(Number::F64(10.0))),
            Token::Number(Ok(Number::AbstractInt(10))),
            Token::Word("l"),
        ],
    )
}

#[test]
fn test_tokens() {
    sub_test("id123_OK", &[Token::Word("id123_OK")]);
    sub_test(
        "92No",
        &[
            Token::Number(Ok(Number::AbstractInt(92))),
            Token::Word("No"),
        ],
    );
    sub_test(
        "2u3o",
        &[
            Token::Number(Ok(Number::U32(2))),
            Token::Number(Ok(Number::AbstractInt(3))),
            Token::Word("o"),
        ],
    );
    sub_test(
        "2.4f44po",
        &[
            Token::Number(Ok(Number::F32(2.4))),
            Token::Number(Ok(Number::AbstractInt(44))),
            Token::Word("po"),
        ],
    );
    sub_test(
        "Δέλτα réflexion Кызыл 𐰓𐰏𐰇 朝焼け سلام 검정 שָׁלוֹם गुलाबी փիրուզ",
        &[
            Token::Word("Δέλτα"),
            Token::Word("réflexion"),
            Token::Word("Кызыл"),
            Token::Word("𐰓𐰏𐰇"),
            Token::Word("朝焼け"),
            Token::Word("سلام"),
            Token::Word("검정"),
            Token::Word("שָׁלוֹם"),
            Token::Word("गुलाबी"),
            Token::Word("փիրուզ"),
        ],
    );
    sub_test("æNoø", &[Token::Word("æNoø")]);
    sub_test("No¾", &[Token::Word("No"), Token::Unknown('¾')]);
    sub_test("No好", &[Token::Word("No好")]);
    sub_test("_No", &[Token::Word("_No")]);

    sub_test_with_and_without_doc_comments(
        "*/*/***/*//=/*****//",
        &[
            Token::Operation('*'),
            Token::AssignmentOperation('/'),
            Token::DocComment("/*****/"),
            Token::Operation('/'),
        ],
    );

    // Type suffixes are only allowed on hex float literals
    // if you provided an exponent.
    sub_test(
        "0x1.2f 0x1.2f 0x1.2h 0x1.2H 0x1.2lf",
        &[
            // The 'f' suffixes are taken as a hex digit:
            // the fractional part is 0x2f / 256.
            Token::Number(Ok(Number::AbstractFloat(1.0 + 0x2f as f64 / 256.0))),
            Token::Number(Ok(Number::AbstractFloat(1.0 + 0x2f as f64 / 256.0))),
            Token::Number(Ok(Number::AbstractFloat(1.125))),
            Token::Word("h"),
            Token::Number(Ok(Number::AbstractFloat(1.125))),
            Token::Word("H"),
            Token::Number(Ok(Number::AbstractFloat(1.125))),
            Token::Word("lf"),
        ],
    )
}

#[test]
fn test_variable_decl() {
    sub_test(
        "@group(0 ) var< uniform> texture:   texture_multisampled_2d <f32 >;",
        &[
            Token::Attribute,
            Token::Word("group"),
            Token::Paren('('),
            Token::Number(Ok(Number::AbstractInt(0))),
            Token::Paren(')'),
            Token::Word("var"),
            Token::TemplateArgsStart,
            Token::Word("uniform"),
            Token::TemplateArgsEnd,
            Token::Word("texture"),
            Token::Separator(':'),
            Token::Word("texture_multisampled_2d"),
            Token::TemplateArgsStart,
            Token::Word("f32"),
            Token::TemplateArgsEnd,
            Token::Separator(';'),
        ],
    );
    sub_test(
        "var<storage,read_write> buffer: array<u32>;",
        &[
            Token::Word("var"),
            Token::TemplateArgsStart,
            Token::Word("storage"),
            Token::Separator(','),
            Token::Word("read_write"),
            Token::TemplateArgsEnd,
            Token::Word("buffer"),
            Token::Separator(':'),
            Token::Word("array"),
            Token::TemplateArgsStart,
            Token::Word("u32"),
            Token::TemplateArgsEnd,
            Token::Separator(';'),
        ],
    );
}

#[test]
fn test_template_list() {
    sub_test(
        "A<B||C>D",
        &[
            Token::Word("A"),
            Token::Paren('<'),
            Token::Word("B"),
            Token::LogicalOperation('|'),
            Token::Word("C"),
            Token::Paren('>'),
            Token::Word("D"),
        ],
    );
    sub_test(
        "A(B<C,D>(E))",
        &[
            Token::Word("A"),
            Token::Paren('('),
            Token::Word("B"),
            Token::TemplateArgsStart,
            Token::Word("C"),
            Token::Separator(','),
            Token::Word("D"),
            Token::TemplateArgsEnd,
            Token::Paren('('),
            Token::Word("E"),
            Token::Paren(')'),
            Token::Paren(')'),
        ],
    );
    sub_test(
        "array<i32,select(2,3,A>B)>",
        &[
            Token::Word("array"),
            Token::TemplateArgsStart,
            Token::Word("i32"),
            Token::Separator(','),
            Token::Word("select"),
            Token::Paren('('),
            Token::Number(Ok(Number::AbstractInt(2))),
            Token::Separator(','),
            Token::Number(Ok(Number::AbstractInt(3))),
            Token::Separator(','),
            Token::Word("A"),
            Token::Paren('>'),
            Token::Word("B"),
            Token::Paren(')'),
            Token::TemplateArgsEnd,
        ],
    );
    sub_test(
        "A[B<C]>D",
        &[
            Token::Word("A"),
            Token::Paren('['),
            Token::Word("B"),
            Token::Paren('<'),
            Token::Word("C"),
            Token::Paren(']'),
            Token::Paren('>'),
            Token::Word("D"),
        ],
    );
    sub_test(
        "A<B<<C>",
        &[
            Token::Word("A"),
            Token::TemplateArgsStart,
            Token::Word("B"),
            Token::ShiftOperation('<'),
            Token::Word("C"),
            Token::TemplateArgsEnd,
        ],
    );
    sub_test(
        "A<(B>=C)>",
        &[
            Token::Word("A"),
            Token::TemplateArgsStart,
            Token::Paren('('),
            Token::Word("B"),
            Token::LogicalOperation('>'),
            Token::Word("C"),
            Token::Paren(')'),
            Token::TemplateArgsEnd,
        ],
    );
    sub_test(
        "A<B>=C>",
        &[
            Token::Word("A"),
            Token::TemplateArgsStart,
            Token::Word("B"),
            Token::TemplateArgsEnd,
            Token::Operation('='),
            Token::Word("C"),
            Token::Paren('>'),
        ],
    );
}

#[test]
fn test_comments() {
    sub_test("// Single comment", &[]);

    sub_test(
        "/* multi
    line
    comment */",
        &[],
    );
    sub_test(
        "/* multi
    line
    comment */
    // and another",
        &[],
    );
}

#[test]
fn test_doc_comments() {
    sub_test_with_and_without_doc_comments(
        "/// Single comment",
        &[Token::DocComment("/// Single comment")],
    );

    sub_test_with_and_without_doc_comments(
        "/** multi
    line
    comment */",
        &[Token::DocComment(
            "/** multi
    line
    comment */",
        )],
    );
    sub_test_with_and_without_doc_comments(
        "/** multi
    line
    comment */
    /// and another",
        &[
            Token::DocComment(
                "/** multi
    line
    comment */",
            ),
            Token::DocComment("/// and another"),
        ],
    );
}

#[test]
fn test_doc_comment_nested() {
    sub_test_with_and_without_doc_comments(
        "/**
    a comment with nested one /**
        nested comment
    */
    */
    const a : i32 = 2;",
        &[
            Token::DocComment(
                "/**
    a comment with nested one /**
        nested comment
    */
    */",
            ),
            Token::Word("const"),
            Token::Word("a"),
            Token::Separator(':'),
            Token::Word("i32"),
            Token::Operation('='),
            Token::Number(Ok(Number::AbstractInt(2))),
            Token::Separator(';'),
        ],
    );
}

#[test]
fn test_doc_comment_long_character() {
    sub_test_with_and_without_doc_comments(
        "/// π/2
        ///     D(𝐡) = ───────────────────────────────────────────────────
///            παₜα_b((𝐡 ⋅ 𝐭)² / αₜ²) + (𝐡 ⋅ 𝐛)² / α_b² +`
    const a : i32 = 2;",
        &[
            Token::DocComment("/// π/2"),
            Token::DocComment("///     D(𝐡) = ───────────────────────────────────────────────────"),
            Token::DocComment("///            παₜα_b((𝐡 ⋅ 𝐭)² / αₜ²) + (𝐡 ⋅ 𝐛)² / α_b² +`"),
            Token::Word("const"),
            Token::Word("a"),
            Token::Separator(':'),
            Token::Word("i32"),
            Token::Operation('='),
            Token::Number(Ok(Number::AbstractInt(2))),
            Token::Separator(';'),
        ],
    );
}

#[test]
fn test_doc_comments_module() {
    sub_test_with_and_without_doc_comments(
        "//! Comment Module
        //! Another one.
        /*! Different module comment */
        /// Trying to break module comment
        // Trying to break module comment again
        //! After a regular comment is ok.
        /*! Different module comment again */

        //! After a break is supported.
        const
        //! After anything else is not.",
        &[
            Token::ModuleDocComment("//! Comment Module"),
            Token::ModuleDocComment("//! Another one."),
            Token::ModuleDocComment("/*! Different module comment */"),
            Token::DocComment("/// Trying to break module comment"),
            Token::ModuleDocComment("//! After a regular comment is ok."),
            Token::ModuleDocComment("/*! Different module comment again */"),
            Token::ModuleDocComment("//! After a break is supported."),
            Token::Word("const"),
            Token::ModuleDocComment("//! After anything else is not."),
        ],
    );
}
