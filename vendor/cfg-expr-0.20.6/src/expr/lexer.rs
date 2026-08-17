use crate::error::{ParseError, Reason};

/// A single token in a cfg expression
/// <https://doc.rust-lang.org/reference/conditional-compilation.html>
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    /// A single contiguous term
    Key(&'a str),
    /// A single contiguous value, without its surrounding quotes
    Value(&'a str),
    /// A '=', joining a key and a value
    Equals,
    /// Beginning of an `all()` predicate list
    All,
    /// Beginning of an `any()` predicate list
    Any,
    /// Beginning of a `not()` predicate
    Not,
    /// A `(` for starting a predicate list
    OpenParen,
    /// A `)` for ending a predicate list
    CloseParen,
    /// A `,` for separating predicates in a predicate list
    Comma,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Token<'_> {
    #[inline]
    fn len(&self) -> usize {
        match self {
            Token::Key(s) => s.len(),
            Token::Value(s) => s.len() + 2,
            Token::Equals | Token::OpenParen | Token::CloseParen | Token::Comma => 1,
            Token::All | Token::Any | Token::Not => 3,
        }
    }
}

/// Allows iteration through a cfg expression, yielding
/// a token or a `ParseError`.
///
/// Prefer to use `Expression::parse` rather than directly
/// using the lexer
pub struct Lexer<'a> {
    pub(super) inner: &'a str,
    original: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a Lexer over a cfg expression, it can either be
    /// a raw expression eg `key` or in attribute form, eg `cfg(key)`
    pub fn new(text: &'a str) -> Self {
        let text = if text.starts_with("cfg(") && text.ends_with(')') {
            &text[4..text.len() - 1]
        } else {
            text
        };

        Self {
            inner: text,
            original: text,
            offset: 0,
        }
    }
}

/// A wrapper around a particular token that includes the span of the characters
/// in the original string, for diagnostic purposes
#[derive(Debug)]
pub struct LexerToken<'a> {
    /// The token that was lexed
    pub token: Token<'a>,
    /// The range of the token characters in the original license expression
    pub span: std::ops::Range<usize>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<LexerToken<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Jump over any whitespace, updating `self.inner` and `self.offset` appropriately
        let non_whitespace_index = match self.inner.find(|c: char| !c.is_whitespace()) {
            Some(idx) => idx,
            None => self.inner.len(),
        };

        self.inner = &self.inner[non_whitespace_index..];
        self.offset += non_whitespace_index;

        #[inline]
        fn is_ident_start(ch: char) -> bool {
            ch == '_' || ch.is_ascii_lowercase() || ch.is_ascii_uppercase()
        }

        #[inline]
        fn is_ident_rest(ch: char) -> bool {
            is_ident_start(ch) || ch.is_ascii_digit()
        }

        match self.inner.chars().next() {
            None => None,
            Some('=') => Some(Ok(Token::Equals)),
            Some('(') => Some(Ok(Token::OpenParen)),
            Some(')') => Some(Ok(Token::CloseParen)),
            Some(',') => Some(Ok(Token::Comma)),
            Some(c) => {
                if c == '"' {
                    match self.inner[1..].find('"') {
                        Some(ind) => Some(Ok(Token::Value(&self.inner[1..=ind]))),
                        None => Some(Err(ParseError {
                            original: self.original.to_owned(),
                            span: self.offset..self.original.len(),
                            reason: Reason::UnclosedQuotes,
                        })),
                    }
                } else if is_ident_start(c) {
                    let substr = match self.inner[1..].find(|c: char| !is_ident_rest(c)) {
                        Some(ind) => &self.inner[..=ind],
                        None => self.inner,
                    };

                    match substr {
                        "all" => Some(Ok(Token::All)),
                        "any" => Some(Ok(Token::Any)),
                        "not" => Some(Ok(Token::Not)),
                        other => Some(Ok(Token::Key(other))),
                    }
                } else {
                    // clippy tries to help here, but we need
                    // a Range here, not a RangeInclusive<>
                    #[allow(clippy::range_plus_one)]
                    Some(Err(ParseError {
                        original: self.original.to_owned(),
                        span: self.offset..self.offset + 1,
                        reason: Reason::Unexpected(&["<key>", "all", "any", "not"]),
                    }))
                }
            }
        }
        .map(|tok| {
            tok.map(|tok| {
                let len = tok.len();

                let start = self.offset;
                self.inner = &self.inner[len..];
                self.offset += len;

                LexerToken {
                    token: tok,
                    span: start..self.offset,
                }
            })
        })
    }
}
