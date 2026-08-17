use std::borrow::Cow;
use std::char;
use std::str;
use std::string;
use std::string::String as StdString;

use self::Token::*;

/// A span, designating a range of bytes where a token is located.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Span {
    /// The start of the range.
    pub start: usize,
    /// The end of the range (exclusive).
    pub end: usize,
}

impl From<Span> for (usize, usize) {
    fn from(Span { start, end }: Span) -> (usize, usize) {
        (start, end)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Token<'a> {
    Whitespace(&'a str),
    Newline,
    Comment(&'a str),

    Equals,
    Period,
    Comma,
    Colon,
    Plus,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Keylike(&'a str),
    String {
        src: &'a str,
        val: Cow<'a, str>,
        multiline: bool,
    },
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    InvalidCharInString(usize, char),
    InvalidEscape(usize, char),
    InvalidHexEscape(usize, char),
    InvalidEscapeValue(usize, u32),
    NewlineInString(usize),
    Unexpected(usize, char),
    UnterminatedString(usize),
    NewlineInTableKey(usize),
    MultilineStringKey(usize),
    Wanted {
        at: usize,
        expected: &'static str,
        found: &'static str,
    },
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: CrlfFold<'a>,
}

#[derive(Clone)]
struct CrlfFold<'a> {
    chars: str::CharIndices<'a>,
}

#[derive(Debug)]
enum MaybeString {
    NotEscaped(usize),
    Owned(string::String),
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        let mut t = Tokenizer {
            input,
            chars: CrlfFold {
                chars: input.char_indices(),
            },
        };
        // Eat utf-8 BOM
        t.eatc('\u{feff}');
        t
    }

    pub fn next(&mut self) -> Result<Option<(Span, Token<'a>)>, Error> {
        let (start, token) = match self.one() {
            Some((start, '\n')) => (start, Newline),
            Some((start, ' ')) => (start, self.whitespace_token(start)),
            Some((start, '\t')) => (start, self.whitespace_token(start)),
            Some((start, '#')) => (start, self.comment_token(start)),
            Some((start, '=')) => (start, Equals),
            Some((start, '.')) => (start, Period),
            Some((start, ',')) => (start, Comma),
            Some((start, ':')) => (start, Colon),
            Some((start, '+')) => (start, Plus),
            Some((start, '{')) => (start, LeftBrace),
            Some((start, '}')) => (start, RightBrace),
            Some((start, '[')) => (start, LeftBracket),
            Some((start, ']')) => (start, RightBracket),
            Some((start, '\'')) => {
                return self
                    .literal_string(start)
                    .map(|t| Some((self.step_span(start), t)))
            }
            Some((start, '"')) => {
                return self
                    .basic_string(start)
                    .map(|t| Some((self.step_span(start), t)))
            }
            Some((start, ch)) if is_keylike(ch) => (start, self.keylike(start)),

            Some((start, ch)) => return Err(Error::Unexpected(start, ch)),
            None => return Ok(None),
        };

        let span = self.step_span(start);
        Ok(Some((span, token)))
    }

    pub fn peek(&mut self) -> Result<Option<(Span, Token<'a>)>, Error> {
        self.clone().next()
    }

    pub fn eat(&mut self, expected: Token<'a>) -> Result<bool, Error> {
        self.eat_spanned(expected).map(|s| s.is_some())
    }

    /// Eat a value, returning it's span if it was consumed.
    pub fn eat_spanned(&mut self, expected: Token<'a>) -> Result<Option<Span>, Error> {
        let span = match self.peek()? {
            Some((span, ref found)) if expected == *found => span,
            Some(_) => return Ok(None),
            None => return Ok(None),
        };

        drop(self.next());
        Ok(Some(span))
    }

    pub fn expect(&mut self, expected: Token<'a>) -> Result<(), Error> {
        // ignore span
        let _ = self.expect_spanned(expected)?;
        Ok(())
    }

    /// Expect the given token returning its span.
    pub fn expect_spanned(&mut self, expected: Token<'a>) -> Result<Span, Error> {
        let current = self.current();
        match self.next()? {
            Some((span, found)) => {
                if expected == found {
                    Ok(span)
                } else {
                    Err(Error::Wanted {
                        at: current,
                        expected: expected.describe(),
                        found: found.describe(),
                    })
                }
            }
            None => Err(Error::Wanted {
                at: self.input.len(),
                expected: expected.describe(),
                found: "eof",
            }),
        }
    }

    pub fn table_key(&mut self) -> Result<(Span, Cow<'a, str>), Error> {
        let current = self.current();
        match self.next()? {
            Some((span, Token::Keylike(k))) => Ok((span, k.into())),
            Some((
                span,
                Token::String {
                    src,
                    val,
                    multiline,
                },
            )) => {
                let offset = self.substr_offset(src);
                if multiline {
                    return Err(Error::MultilineStringKey(offset));
                }
                match src.find('\n') {
                    None => Ok((span, val)),
                    Some(i) => Err(Error::NewlineInTableKey(offset + i)),
                }
            }
            Some((_, other)) => Err(Error::Wanted {
                at: current,
                expected: "a table key",
                found: other.describe(),
            }),
            None => Err(Error::Wanted {
                at: self.input.len(),
                expected: "a table key",
                found: "eof",
            }),
        }
    }

    pub fn eat_whitespace(&mut self) -> Result<(), Error> {
        while self.eatc(' ') || self.eatc('\t') {
            // ...
        }
        Ok(())
    }

    pub fn eat_comment(&mut self) -> Result<bool, Error> {
        if !self.eatc('#') {
            return Ok(false);
        }
        drop(self.comment_token(0));
        self.eat_newline_or_eof().map(|()| true)
    }

    pub fn eat_newline_or_eof(&mut self) -> Result<(), Error> {
        let current = self.current();
        match self.next()? {
            None | Some((_, Token::Newline)) => Ok(()),
            Some((_, other)) => Err(Error::Wanted {
                at: current,
                expected: "newline",
                found: other.describe(),
            }),
        }
    }

    pub fn skip_to_newline(&mut self) {
        loop {
            match self.one() {
                Some((_, '\n')) | None => break,
                _ => {}
            }
        }
    }

    fn eatc(&mut self, ch: char) -> bool {
        match self.chars.clone().next() {
            Some((_, ch2)) if ch == ch2 => {
                self.one();
                true
            }
            _ => false,
        }
    }

    pub fn current(&mut self) -> usize {
        self.chars
            .clone()
            .next()
            .map(|i| i.0)
            .unwrap_or_else(|| self.input.len())
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    fn whitespace_token(&mut self, start: usize) -> Token<'a> {
        while self.eatc(' ') || self.eatc('\t') {
            // ...
        }
        Whitespace(&self.input[start..self.current()])
    }

    fn comment_token(&mut self, start: usize) -> Token<'a> {
        while let Some((_, ch)) = self.chars.clone().next() {
            if ch != '\t' && !('\u{20}'..='\u{10ffff}').contains(&ch) {
                break;
            }
            self.one();
        }
        Comment(&self.input[start..self.current()])
    }

    #[allow(clippy::type_complexity)]
    fn read_string(
        &mut self,
        delim: char,
        start: usize,
        new_ch: &mut dyn FnMut(
            &mut Tokenizer<'_>,
            &mut MaybeString,
            bool,
            usize,
            char,
        ) -> Result<(), Error>,
    ) -> Result<Token<'a>, Error> {
        let mut multiline = false;
        if self.eatc(delim) {
            if self.eatc(delim) {
                multiline = true;
            } else {
                return Ok(String {
                    src: &self.input[start..start + 2],
                    val: Cow::Borrowed(""),
                    multiline: false,
                });
            }
        }
        let mut val = MaybeString::NotEscaped(self.current());
        let mut n = 0;
        'outer: loop {
            n += 1;
            match self.one() {
                Some((i, '\n')) => {
                    if multiline {
                        if self.input.as_bytes()[i] == b'\r' {
                            val.to_owned(&self.input[..i]);
                        }
                        if n == 1 {
                            val = MaybeString::NotEscaped(self.current());
                        } else {
                            val.push('\n');
                        }
                        continue;
                    } else {
                        return Err(Error::NewlineInString(i));
                    }
                }
                Some((mut i, ch)) if ch == delim => {
                    if multiline {
                        if !self.eatc(delim) {
                            val.push(delim);
                            continue 'outer;
                        }
                        if !self.eatc(delim) {
                            val.push(delim);
                            val.push(delim);
                            continue 'outer;
                        }
                        if self.eatc(delim) {
                            val.push(delim);
                            i += 1;
                        }
                        if self.eatc(delim) {
                            val.push(delim);
                            i += 1;
                        }
                    }
                    return Ok(String {
                        src: &self.input[start..self.current()],
                        val: val.into_cow(&self.input[..i]),
                        multiline,
                    });
                }
                Some((i, c)) => new_ch(self, &mut val, multiline, i, c)?,
                None => return Err(Error::UnterminatedString(start)),
            }
        }
    }

    fn literal_string(&mut self, start: usize) -> Result<Token<'a>, Error> {
        self.read_string('\'', start, &mut |_me, val, _multi, i, ch| {
            if ch == '\u{09}' || (('\u{20}'..='\u{10ffff}').contains(&ch) && ch != '\u{7f}') {
                val.push(ch);
                Ok(())
            } else {
                Err(Error::InvalidCharInString(i, ch))
            }
        })
    }

    fn basic_string(&mut self, start: usize) -> Result<Token<'a>, Error> {
        self.read_string('"', start, &mut |me, val, multi, i, ch| match ch {
            '\\' => {
                val.to_owned(&me.input[..i]);
                match me.chars.next() {
                    Some((_, '"')) => val.push('"'),
                    Some((_, '\\')) => val.push('\\'),
                    Some((_, 'b')) => val.push('\u{8}'),
                    Some((_, 'f')) => val.push('\u{c}'),
                    Some((_, 'n')) => val.push('\n'),
                    Some((_, 'r')) => val.push('\r'),
                    Some((_, 't')) => val.push('\t'),
                    Some((i, c @ 'u')) | Some((i, c @ 'U')) => {
                        let len = if c == 'u' { 4 } else { 8 };
                        val.push(me.hex(start, i, len)?);
                    }
                    Some((i, c @ ' ')) | Some((i, c @ '\t')) | Some((i, c @ '\n')) if multi => {
                        if c != '\n' {
                            while let Some((_, ch)) = me.chars.clone().next() {
                                match ch {
                                    ' ' | '\t' => {
                                        me.chars.next();
                                        continue;
                                    }
                                    '\n' => {
                                        me.chars.next();
                                        break;
                                    }
                                    _ => return Err(Error::InvalidEscape(i, c)),
                                }
                            }
                        }
                        while let Some((_, ch)) = me.chars.clone().next() {
                            match ch {
                                ' ' | '\t' | '\n' => {
                                    me.chars.next();
                                }
                                _ => break,
                            }
                        }
                    }
                    Some((i, c)) => return Err(Error::InvalidEscape(i, c)),
                    None => return Err(Error::UnterminatedString(start)),
                }
                Ok(())
            }
            ch if ch == '\u{09}' || (('\u{20}'..='\u{10ffff}').contains(&ch) && ch != '\u{7f}') => {
                val.push(ch);
                Ok(())
            }
            _ => Err(Error::InvalidCharInString(i, ch)),
        })
    }

    fn hex(&mut self, start: usize, i: usize, len: usize) -> Result<char, Error> {
        let mut buf = StdString::with_capacity(len);
        for _ in 0..len {
            match self.one() {
                Some((_, ch)) if ch as u32 <= 0x7F && ch.is_ascii_hexdigit() => buf.push(ch),
                Some((i, ch)) => return Err(Error::InvalidHexEscape(i, ch)),
                None => return Err(Error::UnterminatedString(start)),
            }
        }
        let val = u32::from_str_radix(&buf, 16).unwrap();
        match char::from_u32(val) {
            Some(ch) => Ok(ch),
            None => Err(Error::InvalidEscapeValue(i, val)),
        }
    }

    fn keylike(&mut self, start: usize) -> Token<'a> {
        while let Some((_, ch)) = self.peek_one() {
            if !is_keylike(ch) {
                break;
            }
            self.one();
        }
        Keylike(&self.input[start..self.current()])
    }

    pub fn substr_offset(&self, s: &'a str) -> usize {
        assert!(s.len() <= self.input.len());
        let a = self.input.as_ptr() as usize;
        let b = s.as_ptr() as usize;
        assert!(a <= b);
        b - a
    }

    /// Calculate the span of a single character.
    fn step_span(&mut self, start: usize) -> Span {
        let end = self
            .peek_one()
            .map(|t| t.0)
            .unwrap_or_else(|| self.input.len());
        Span { start, end }
    }

    /// Peek one char without consuming it.
    fn peek_one(&mut self) -> Option<(usize, char)> {
        self.chars.clone().next()
    }

    /// Take one char.
    pub fn one(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
}

impl<'a> Iterator for CrlfFold<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(i, c)| {
            if c == '\r' {
                let mut attempt = self.chars.clone();
                if let Some((_, '\n')) = attempt.next() {
                    self.chars = attempt;
                    return (i, '\n');
                }
            }
            (i, c)
        })
    }
}

impl MaybeString {
    fn push(&mut self, ch: char) {
        match *self {
            MaybeString::NotEscaped(..) => {}
            MaybeString::Owned(ref mut s) => s.push(ch),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_owned(&mut self, input: &str) {
        match *self {
            MaybeString::NotEscaped(start) => {
                *self = MaybeString::Owned(input[start..].to_owned());
            }
            MaybeString::Owned(..) => {}
        }
    }

    fn into_cow(self, input: &str) -> Cow<'_, str> {
        match self {
            MaybeString::NotEscaped(start) => Cow::Borrowed(&input[start..]),
            MaybeString::Owned(s) => Cow::Owned(s),
        }
    }
}

fn is_keylike(ch: char) -> bool {
    ('A'..='Z').contains(&ch)
        || ('a'..='z').contains(&ch)
        || ('0'..='9').contains(&ch)
        || ch == '-'
        || ch == '_'
}

impl<'a> Token<'a> {
    pub fn describe(&self) -> &'static str {
        match *self {
            Token::Keylike(_) => "an identifier",
            Token::Equals => "an equals",
            Token::Period => "a period",
            Token::Comment(_) => "a comment",
            Token::Newline => "a newline",
            Token::Whitespace(_) => "whitespace",
            Token::Comma => "a comma",
            Token::RightBrace => "a right brace",
            Token::LeftBrace => "a left brace",
            Token::RightBracket => "a right bracket",
            Token::LeftBracket => "a left bracket",
            Token::String { multiline, .. } => {
                if multiline {
                    "a multiline string"
                } else {
                    "a string"
                }
            }
            Token::Colon => "a colon",
            Token::Plus => "a plus",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Token, Tokenizer};
    use std::borrow::Cow;

    fn err(input: &str, err: Error) {
        let mut t = Tokenizer::new(input);
        let token = t.next().unwrap_err();
        assert_eq!(token, err);
        assert!(t.next().unwrap().is_none());
    }

    #[test]
    fn literal_strings() {
        fn t(input: &str, val: &str, multiline: bool) {
            let mut t = Tokenizer::new(input);
            let (_, token) = t.next().unwrap().unwrap();
            assert_eq!(
                token,
                Token::String {
                    src: input,
                    val: Cow::Borrowed(val),
                    multiline,
                }
            );
            assert!(t.next().unwrap().is_none());
        }

        t("''", "", false);
        t("''''''", "", true);
        t("'''\n'''", "", true);
        t("'a'", "a", false);
        t("'\"a'", "\"a", false);
        t("''''a'''", "'a", true);
        t("'''\n'a\n'''", "'a\n", true);
        t("'''a\n'a\r\n'''", "a\n'a\n", true);
    }

    #[test]
    fn basic_strings() {
        fn t(input: &str, val: &str, multiline: bool) {
            let mut t = Tokenizer::new(input);
            let (_, token) = t.next().unwrap().unwrap();
            assert_eq!(
                token,
                Token::String {
                    src: input,
                    val: Cow::Borrowed(val),
                    multiline,
                }
            );
            assert!(t.next().unwrap().is_none());
        }

        t(r#""""#, "", false);
        t(r#""""""""#, "", true);
        t(r#""a""#, "a", false);
        t(r#""""a""""#, "a", true);
        t(r#""\t""#, "\t", false);
        t(r#""\u0000""#, "\0", false);
        t(r#""\U00000000""#, "\0", false);
        t(r#""\U000A0000""#, "\u{A0000}", false);
        t(r#""\\t""#, "\\t", false);
        t("\"\t\"", "\t", false);
        t("\"\"\"\n\t\"\"\"", "\t", true);
        t("\"\"\"\\\n\"\"\"", "", true);
        t(
            "\"\"\"\\\n     \t   \t  \\\r\n  \t \n  \t \r\n\"\"\"",
            "",
            true,
        );
        t(r#""\r""#, "\r", false);
        t(r#""\n""#, "\n", false);
        t(r#""\b""#, "\u{8}", false);
        t(r#""a\fa""#, "a\u{c}a", false);
        t(r#""\"a""#, "\"a", false);
        t("\"\"\"\na\"\"\"", "a", true);
        t("\"\"\"\n\"\"\"", "", true);
        t(r#""""a\"""b""""#, "a\"\"\"b", true);
        err(r#""\a"#, Error::InvalidEscape(2, 'a'));
        err("\"\\\n", Error::InvalidEscape(2, '\n'));
        err("\"\\\r\n", Error::InvalidEscape(2, '\n'));
        err("\"\\", Error::UnterminatedString(0));
        err("\"\u{0}", Error::InvalidCharInString(1, '\u{0}'));
        err(r#""\U00""#, Error::InvalidHexEscape(5, '"'));
        err(r#""\U00"#, Error::UnterminatedString(0));
        err(r#""\uD800"#, Error::InvalidEscapeValue(2, 0xd800));
        err(r#""\UFFFFFFFF"#, Error::InvalidEscapeValue(2, 0xffff_ffff));
    }

    #[test]
    fn keylike() {
        fn t(input: &str) {
            let mut t = Tokenizer::new(input);
            let (_, token) = t.next().unwrap().unwrap();
            assert_eq!(token, Token::Keylike(input));
            assert!(t.next().unwrap().is_none());
        }
        t("foo");
        t("0bar");
        t("bar0");
        t("1234");
        t("a-b");
        t("a_B");
        t("-_-");
        t("___");
    }

    #[test]
    fn all() {
        fn t(input: &str, expected: &[((usize, usize), Token<'_>, &str)]) {
            let mut tokens = Tokenizer::new(input);
            let mut actual: Vec<((usize, usize), Token<'_>, &str)> = Vec::new();
            while let Some((span, token)) = tokens.next().unwrap() {
                actual.push((span.into(), token, &input[span.start..span.end]));
            }
            for (a, b) in actual.iter().zip(expected) {
                assert_eq!(a, b);
            }
            assert_eq!(actual.len(), expected.len());
        }

        t(
            " a ",
            &[
                ((0, 1), Token::Whitespace(" "), " "),
                ((1, 2), Token::Keylike("a"), "a"),
                ((2, 3), Token::Whitespace(" "), " "),
            ],
        );

        t(
            " a\t [[]] \t [] {} , . =\n# foo \r\n#foo \n ",
            &[
                ((0, 1), Token::Whitespace(" "), " "),
                ((1, 2), Token::Keylike("a"), "a"),
                ((2, 4), Token::Whitespace("\t "), "\t "),
                ((4, 5), Token::LeftBracket, "["),
                ((5, 6), Token::LeftBracket, "["),
                ((6, 7), Token::RightBracket, "]"),
                ((7, 8), Token::RightBracket, "]"),
                ((8, 11), Token::Whitespace(" \t "), " \t "),
                ((11, 12), Token::LeftBracket, "["),
                ((12, 13), Token::RightBracket, "]"),
                ((13, 14), Token::Whitespace(" "), " "),
                ((14, 15), Token::LeftBrace, "{"),
                ((15, 16), Token::RightBrace, "}"),
                ((16, 17), Token::Whitespace(" "), " "),
                ((17, 18), Token::Comma, ","),
                ((18, 19), Token::Whitespace(" "), " "),
                ((19, 20), Token::Period, "."),
                ((20, 21), Token::Whitespace(" "), " "),
                ((21, 22), Token::Equals, "="),
                ((22, 23), Token::Newline, "\n"),
                ((23, 29), Token::Comment("# foo "), "# foo "),
                ((29, 31), Token::Newline, "\r\n"),
                ((31, 36), Token::Comment("#foo "), "#foo "),
                ((36, 37), Token::Newline, "\n"),
                ((37, 38), Token::Whitespace(" "), " "),
            ],
        );
    }

    #[test]
    fn bare_cr_bad() {
        err("\r", Error::Unexpected(0, '\r'));
        err("'\n", Error::NewlineInString(1));
        err("'\u{0}", Error::InvalidCharInString(1, '\u{0}'));
        err("'", Error::UnterminatedString(0));
        err("\u{0}", Error::Unexpected(0, '\u{0}'));
    }

    #[test]
    fn bad_comment() {
        let mut t = Tokenizer::new("#\u{0}");
        t.next().unwrap().unwrap();
        assert_eq!(t.next(), Err(Error::Unexpected(1, '\u{0}')));
        assert!(t.next().unwrap().is_none());
    }
}
