//! Lexer for the ISLE language.

use std::borrow::Cow;

use crate::error::{Error, Span};
use crate::files::Files;

type Result<T> = std::result::Result<T, Error>;

/// The lexer.
///
/// Breaks source text up into a sequence of tokens (with source positions).
#[derive(Clone, Debug)]
pub struct Lexer<'src> {
    src: &'src str,
    pos: Pos,
    lookahead: Option<(Pos, Token)>,
}

/// A source position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub struct Pos {
    /// This source position's file.
    ///
    /// Indexes into `Lexer::filenames` early in the compiler pipeline, and
    /// later into `TypeEnv::filenames` once we get into semantic analysis.
    pub file: usize,
    /// This source position's byte offset in the file.
    pub offset: usize,
}

impl Pos {
    /// Create a new `Pos`.
    pub fn new(file: usize, offset: usize) -> Self {
        Self { file, offset }
    }

    /// Print this source position as `file.isle line 12`.
    pub fn pretty_print_line(&self, files: &Files) -> String {
        format!(
            "{} line {}",
            files.file_name(self.file).unwrap(),
            files.file_line_map(self.file).unwrap().line(self.offset)
        )
    }
}

/// A token of ISLE source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    /// Left paren.
    LParen,
    /// Right paren.
    RParen,
    /// A symbol, e.g. `Foo`.
    Symbol(String),
    /// An integer.
    Int(i128),
    /// `@`
    At,
}

impl<'src> Lexer<'src> {
    /// Create a new lexer for the given source contents
    pub fn new(file: usize, src: &'src str) -> Result<Lexer<'src>> {
        let mut l = Lexer {
            src,
            pos: Pos::new(file, 0),
            lookahead: None,
        };
        l.reload()?;
        Ok(l)
    }

    /// Get the lexer's current source position.
    pub fn pos(&self) -> Pos {
        self.pos
    }

    fn advance_pos(&mut self) {
        self.advance_by(1)
    }

    fn advance_by(&mut self, n: usize) {
        self.pos.offset += n;
    }

    fn error(&self, pos: Pos, msg: impl Into<String>) -> Error {
        Error::ParseError {
            msg: msg.into(),
            span: Span::new_single(pos),
        }
    }

    fn next_token(&mut self) -> Result<Option<(Pos, Token)>> {
        fn is_sym_first_char(c: u8) -> bool {
            match c {
                b'-' | b'0'..=b'9' | b'(' | b')' | b';' => false,
                c if c.is_ascii_whitespace() => false,
                _ => true,
            }
        }
        fn is_sym_other_char(c: u8) -> bool {
            match c {
                b'(' | b')' | b';' | b'@' => false,
                c if c.is_ascii_whitespace() => false,
                _ => true,
            }
        }

        // Skip any whitespace and any comments.
        while let Some(c) = self.peek_byte() {
            match c {
                b' ' | b'\t' | b'\n' | b'\r' => self.advance_pos(),
                b';' => {
                    while let Some(c) = self.peek_byte() {
                        match c {
                            b'\n' | b'\r' => break,
                            _ => self.advance_pos(),
                        }
                    }
                }
                b'(' if self.lookahead_byte(1) == Some(b';') => {
                    let pos = self.pos();
                    self.advance_by(2);
                    let mut depth = 1usize;
                    loop {
                        match self.peek_byte() {
                            None => return Err(self.error(pos, "unterminated block comment")),
                            Some(b'(') if self.lookahead_byte(1) == Some(b';') => {
                                self.advance_by(2);
                                depth += 1;
                            }
                            Some(b';') if self.lookahead_byte(1) == Some(b')') => {
                                self.advance_by(2);
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            Some(_) => self.advance_pos(),
                        }
                    }
                }
                _ => break,
            }
        }

        let Some(c) = self.peek_byte() else {
            return Ok(None);
        };
        let char_pos = self.pos();
        match c {
            b'(' => {
                self.advance_pos();
                Ok(Some((char_pos, Token::LParen)))
            }
            b')' => {
                self.advance_pos();
                Ok(Some((char_pos, Token::RParen)))
            }
            b'@' => {
                self.advance_pos();
                Ok(Some((char_pos, Token::At)))
            }
            c if is_sym_first_char(c) => {
                let start = self.pos.offset;
                let start_pos = self.pos();
                while let Some(c) = self.peek_byte() {
                    match c {
                        c if is_sym_other_char(c) => self.advance_pos(),
                        _ => break,
                    }
                }
                let end = self.pos.offset;
                let s = &self.src[start..end];
                debug_assert!(!s.is_empty());
                Ok(Some((start_pos, Token::Symbol(s.to_string()))))
            }
            c @ (b'0'..=b'9' | b'-') => {
                let start_pos = self.pos();
                let mut neg = false;
                if c == b'-' {
                    self.advance_pos();
                    neg = true;
                }

                let mut radix = 10;

                // Check for prefixed literals.
                match (
                    self.src.as_bytes().get(self.pos.offset),
                    self.src.as_bytes().get(self.pos.offset + 1),
                ) {
                    (Some(b'0'), Some(b'x' | b'X')) => {
                        self.advance_by(2);
                        radix = 16;
                    }
                    (Some(b'0'), Some(b'o' | b'O')) => {
                        self.advance_by(2);
                        radix = 8;
                    }
                    (Some(b'0'), Some(b'b' | b'B')) => {
                        self.advance_by(2);
                        radix = 2;
                    }
                    _ => {}
                }

                // Find the range in the buffer for this integer literal. We'll
                // pass this range to `i64::from_str_radix` to do the actual
                // string-to-integer conversion.
                let start = self.pos.offset;
                while let Some(c) = self.peek_byte() {
                    match c {
                        b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'_' => self.advance_pos(),
                        _ => break,
                    }
                }
                let end = self.pos.offset;
                let s = &self.src[start..end];
                let s = if s.contains('_') {
                    Cow::Owned(s.replace('_', ""))
                } else {
                    Cow::Borrowed(s)
                };

                // Support either signed range (-2^127..2^127) or
                // unsigned range (0..2^128).
                let num = match u128::from_str_radix(&s, radix) {
                    Ok(num) => num,
                    Err(err) => return Err(self.error(start_pos, err.to_string())),
                };

                let num = match (neg, num) {
                    (true, 0x80000000000000000000000000000000) => {
                        return Err(self.error(start_pos, "integer literal cannot fit in i128"));
                    }
                    (true, _) => -(num as i128),
                    (false, _) => num as i128,
                };
                let tok = Token::Int(num);

                Ok(Some((start_pos, tok)))
            }
            c => Err(self.error(self.pos, format!("Unexpected character '{c}'"))),
        }
    }

    /// Get the next token from this lexer's token stream, if any.
    pub fn next(&mut self) -> Result<Option<(Pos, Token)>> {
        let tok = self.lookahead.take();
        self.reload()?;
        Ok(tok)
    }

    fn reload(&mut self) -> Result<()> {
        if self.lookahead.is_none() && self.pos.offset < self.src.len() {
            self.lookahead = self.next_token()?;
        }
        Ok(())
    }

    /// Peek ahead at the next token.
    pub fn peek(&self) -> Option<&(Pos, Token)> {
        self.lookahead.as_ref()
    }

    /// Are we at the end of the source input?
    pub fn eof(&self) -> bool {
        self.lookahead.is_none()
    }

    fn peek_byte(&self) -> Option<u8> {
        self.lookahead_byte(0)
    }

    fn lookahead_byte(&self, n: usize) -> Option<u8> {
        self.src.as_bytes().get(self.pos.offset + n).copied()
    }
}

impl Token {
    /// Is this an `Int` token?
    pub fn is_int(&self) -> bool {
        matches!(self, Token::Int(_))
    }

    /// Is this a `Sym` token?
    pub fn is_sym(&self) -> bool {
        matches!(self, Token::Symbol(_))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[track_caller]
    fn lex(src: &str) -> Vec<Token> {
        let mut toks = vec![];
        let mut lexer = Lexer::new(0, src).unwrap();
        while let Some((_, tok)) = lexer.next().unwrap() {
            toks.push(tok);
        }
        toks
    }

    #[test]
    fn lexer_basic() {
        assert_eq!(
            lex(
                ";; comment\n; another\r\n   \t(one two three (; block comment ;) 23 (; nested (; block ;) comment ;) -568  )\n"
            ),
            [
                Token::LParen,
                Token::Symbol("one".to_string()),
                Token::Symbol("two".to_string()),
                Token::Symbol("three".to_string()),
                Token::Int(23),
                Token::Int(-568),
                Token::RParen
            ]
        );
    }

    #[test]
    fn ends_with_sym() {
        assert_eq!(lex("asdf"), [Token::Symbol("asdf".to_string())]);
    }

    #[test]
    fn ends_with_num() {
        assert_eq!(lex("23"), [Token::Int(23)]);
    }

    #[test]
    fn weird_syms() {
        assert_eq!(
            lex("(+ [] => !! _test!;comment\n)"),
            [
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Symbol("[]".to_string()),
                Token::Symbol("=>".to_string()),
                Token::Symbol("!!".to_string()),
                Token::Symbol("_test!".to_string()),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn integers() {
        assert_eq!(
            lex("0 1 -1"),
            [Token::Int(0), Token::Int(1), Token::Int(-1)]
        );

        assert_eq!(
            lex("340_282_366_920_938_463_463_374_607_431_768_211_455"),
            [Token::Int(-1)]
        );

        assert_eq!(
            lex("170_141_183_460_469_231_731_687_303_715_884_105_727"),
            [Token::Int(i128::MAX)]
        );

        assert!(Lexer::new(0, "-170_141_183_460_469_231_731_687_303_715_884_105_728").is_err())
    }
}
