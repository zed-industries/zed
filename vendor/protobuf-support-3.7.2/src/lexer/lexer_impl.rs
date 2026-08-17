use std::char;
use std::num::ParseFloatError;
use std::num::ParseIntError;

use crate::lexer::float;
use crate::lexer::float::ProtobufFloatParseError;
use crate::lexer::json_number_lit::JsonNumberLit;
use crate::lexer::loc::Loc;
use crate::lexer::loc::FIRST_COL;
use crate::lexer::parser_language::ParserLanguage;
use crate::lexer::str_lit::StrLit;
use crate::lexer::str_lit::StrLitDecodeError;
use crate::lexer::token::Token;
use crate::lexer::token::TokenWithLocation;

#[derive(Debug, thiserror::Error)]
pub enum LexerError {
    // TODO: something better than this
    #[error("Incorrect input")]
    IncorrectInput,
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("Expecting char: {:?}", .0)]
    ExpectChar(char),
    #[error("Parse int error")]
    ParseIntError,
    #[error("Parse float error")]
    ParseFloatError,
    // TODO: how it is different from ParseFloatError?
    #[error("Incorrect float literal")]
    IncorrectFloatLit,
    #[error("Incorrect JSON escape")]
    IncorrectJsonEscape,
    #[error("Incorrect JSON number")]
    IncorrectJsonNumber,
    #[error("Incorrect Unicode character")]
    IncorrectUnicodeChar,
    #[error("Expecting hex digit")]
    ExpectHexDigit,
    #[error("Expecting oct digit")]
    ExpectOctDigit,
    #[error("Expecting dec digit")]
    ExpectDecDigit,
    #[error(transparent)]
    StrLitDecodeError(#[from] StrLitDecodeError),
    #[error("Expecting identifier")]
    ExpectedIdent,
}

pub type LexerResult<T> = Result<T, LexerError>;

impl From<ParseIntError> for LexerError {
    fn from(_: ParseIntError) -> Self {
        LexerError::ParseIntError
    }
}

impl From<ParseFloatError> for LexerError {
    fn from(_: ParseFloatError) -> Self {
        LexerError::ParseFloatError
    }
}

impl From<ProtobufFloatParseError> for LexerError {
    fn from(_: ProtobufFloatParseError) -> Self {
        LexerError::IncorrectFloatLit
    }
}

/// The raw bytes for a single char or escape sequence in a string literal
///
/// The raw bytes are available via an `into_iter` implementation.
pub(crate) struct DecodedBytes {
    // a single char can be up to 4-bytes when encoded in utf-8
    buf: [u8; 4],
    len: usize,
}

impl DecodedBytes {
    fn byte(b: u8) -> DecodedBytes {
        DecodedBytes {
            buf: [b, 0, 0, 0],
            len: 1,
        }
    }

    fn char(value: char) -> Self {
        let mut buf = [0; 4];
        let len = value.encode_utf8(&mut buf).len();
        DecodedBytes { buf, len }
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

#[derive(Copy, Clone)]
pub struct Lexer<'a> {
    language: ParserLanguage,
    input: &'a str,
    pos: usize,
    pub loc: Loc,
}

fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, language: ParserLanguage) -> Lexer<'a> {
        Lexer {
            language,
            input,
            pos: 0,
            loc: Loc::start(),
        }
    }

    /// No more chars
    pub fn eof(&self) -> bool {
        self.pos == self.input.len()
    }

    /// Remaining chars
    fn rem_chars(&self) -> &'a str {
        &self.input[self.pos..]
    }

    pub fn lookahead_char_is<P: FnOnce(char) -> bool>(&self, p: P) -> bool {
        self.lookahead_char().map_or(false, p)
    }

    fn lookahead_char_is_in(&self, alphabet: &str) -> bool {
        self.lookahead_char_is(|c| alphabet.contains(c))
    }

    fn next_char_opt(&mut self) -> Option<char> {
        let rem = self.rem_chars();
        if rem.is_empty() {
            None
        } else {
            let mut char_indices = rem.char_indices();
            let (_, c) = char_indices.next().unwrap();
            let c_len = char_indices.next().map(|(len, _)| len).unwrap_or(rem.len());
            self.pos += c_len;
            if c == '\n' {
                self.loc.line += 1;
                self.loc.col = FIRST_COL;
            } else {
                self.loc.col += 1;
            }
            Some(c)
        }
    }

    fn next_char(&mut self) -> LexerResult<char> {
        self.next_char_opt().ok_or(LexerError::UnexpectedEof)
    }

    /// Skip whitespaces
    fn skip_whitespaces(&mut self) {
        self.take_while(|c| c.is_whitespace());
    }

    fn skip_c_comment(&mut self) -> LexerResult<()> {
        if self.skip_if_lookahead_is_str("/*") {
            let end = "*/";
            match self.rem_chars().find(end) {
                None => Err(LexerError::UnexpectedEof),
                Some(len) => {
                    let new_pos = self.pos + len + end.len();
                    self.skip_to_pos(new_pos);
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }

    fn skip_cpp_comment(&mut self) {
        if self.skip_if_lookahead_is_str("//") {
            loop {
                match self.next_char_opt() {
                    Some('\n') | None => break,
                    _ => {}
                }
            }
        }
    }

    fn skip_sh_comment(&mut self) {
        if self.skip_if_lookahead_is_str("#") {
            loop {
                match self.next_char_opt() {
                    Some('\n') | None => break,
                    _ => {}
                }
            }
        }
    }

    fn skip_comment(&mut self) -> LexerResult<()> {
        match self.language {
            ParserLanguage::Proto => {
                self.skip_c_comment()?;
                self.skip_cpp_comment();
            }
            ParserLanguage::TextFormat => {
                self.skip_sh_comment();
            }
            ParserLanguage::Json => {}
        }
        Ok(())
    }

    pub fn skip_ws(&mut self) -> LexerResult<()> {
        loop {
            let pos = self.pos;
            self.skip_whitespaces();
            self.skip_comment()?;
            if pos == self.pos {
                // Did not advance
                return Ok(());
            }
        }
    }

    pub fn take_while<F>(&mut self, f: F) -> &'a str
    where
        F: Fn(char) -> bool,
    {
        let start = self.pos;
        while self.lookahead_char().map(&f) == Some(true) {
            self.next_char_opt().unwrap();
        }
        let end = self.pos;
        &self.input[start..end]
    }

    fn lookahead_char(&self) -> Option<char> {
        self.clone().next_char_opt()
    }

    fn lookahead_is_str(&self, s: &str) -> bool {
        self.rem_chars().starts_with(s)
    }

    fn skip_if_lookahead_is_str(&mut self, s: &str) -> bool {
        if self.lookahead_is_str(s) {
            let new_pos = self.pos + s.len();
            self.skip_to_pos(new_pos);
            true
        } else {
            false
        }
    }

    fn next_char_if<P>(&mut self, p: P) -> Option<char>
    where
        P: FnOnce(char) -> bool,
    {
        let mut clone = self.clone();
        match clone.next_char_opt() {
            Some(c) if p(c) => {
                *self = clone;
                Some(c)
            }
            _ => None,
        }
    }

    pub fn next_char_if_eq(&mut self, expect: char) -> bool {
        self.next_char_if(|c| c == expect) != None
    }

    fn next_char_if_in(&mut self, alphabet: &str) -> Option<char> {
        for c in alphabet.chars() {
            if self.next_char_if_eq(c) {
                return Some(c);
            }
        }
        None
    }

    fn next_char_expect_eq(&mut self, expect: char) -> LexerResult<()> {
        if self.next_char_if_eq(expect) {
            Ok(())
        } else {
            Err(LexerError::ExpectChar(expect))
        }
    }

    fn next_char_expect<P>(&mut self, expect: P, err: LexerError) -> LexerResult<char>
    where
        P: FnOnce(char) -> bool,
    {
        self.next_char_if(expect).ok_or(err)
    }

    // str functions

    /// properly update line and column
    fn skip_to_pos(&mut self, new_pos: usize) -> &'a str {
        assert!(new_pos >= self.pos);
        assert!(new_pos <= self.input.len());
        let pos = self.pos;
        while self.pos != new_pos {
            self.next_char_opt().unwrap();
        }
        &self.input[pos..new_pos]
    }

    // Protobuf grammar

    // char functions

    // letter = "A" … "Z" | "a" … "z"
    // https://github.com/google/protobuf/issues/4565
    fn next_letter_opt(&mut self) -> Option<char> {
        self.next_char_if(is_letter)
    }

    // capitalLetter =  "A" … "Z"
    fn _next_capital_letter_opt(&mut self) -> Option<char> {
        self.next_char_if(|c| c >= 'A' && c <= 'Z')
    }

    fn next_ident_part(&mut self) -> Option<char> {
        self.next_char_if(|c| c.is_ascii_alphanumeric() || c == '_')
    }

    // Identifiers

    // ident = letter { letter | decimalDigit | "_" }
    fn next_ident_opt(&mut self) -> LexerResult<Option<String>> {
        if let Some(c) = self.next_letter_opt() {
            let mut ident = String::new();
            ident.push(c);
            while let Some(c) = self.next_ident_part() {
                ident.push(c);
            }
            Ok(Some(ident))
        } else {
            Ok(None)
        }
    }

    // Integer literals

    // hexLit     = "0" ( "x" | "X" ) hexDigit { hexDigit }
    fn next_hex_lit_opt(&mut self) -> LexerResult<Option<u64>> {
        Ok(
            if self.skip_if_lookahead_is_str("0x") || self.skip_if_lookahead_is_str("0X") {
                let s = self.take_while(|c| c.is_ascii_hexdigit());
                Some(u64::from_str_radix(s, 16)? as u64)
            } else {
                None
            },
        )
    }

    // decimalLit = ( "1" … "9" ) { decimalDigit }
    // octalLit   = "0" { octalDigit }
    fn next_decimal_octal_lit_opt(&mut self) -> LexerResult<Option<u64>> {
        // do not advance on number parse error
        let mut clone = self.clone();

        let pos = clone.pos;

        Ok(if clone.next_char_if(|c| c.is_ascii_digit()) != None {
            clone.take_while(|c| c.is_ascii_digit());
            let value = clone.input[pos..clone.pos].parse()?;
            *self = clone;
            Some(value)
        } else {
            None
        })
    }

    // hexDigit     = "0" … "9" | "A" … "F" | "a" … "f"
    fn next_hex_digit(&mut self) -> LexerResult<u32> {
        let mut clone = self.clone();
        let r = match clone.next_char()? {
            c if c >= '0' && c <= '9' => c as u32 - b'0' as u32,
            c if c >= 'A' && c <= 'F' => c as u32 - b'A' as u32 + 10,
            c if c >= 'a' && c <= 'f' => c as u32 - b'a' as u32 + 10,
            _ => return Err(LexerError::ExpectHexDigit),
        };
        *self = clone;
        Ok(r)
    }

    // octalDigit   = "0" … "7"
    fn next_octal_digit(&mut self) -> LexerResult<u32> {
        self.next_char_expect(|c| c >= '0' && c <= '9', LexerError::ExpectOctDigit)
            .map(|c| c as u32 - '0' as u32)
    }

    // decimalDigit = "0" … "9"
    fn next_decimal_digit(&mut self) -> LexerResult<u32> {
        self.next_char_expect(|c| c >= '0' && c <= '9', LexerError::ExpectDecDigit)
            .map(|c| c as u32 - '0' as u32)
    }

    // decimals  = decimalDigit { decimalDigit }
    fn next_decimal_digits(&mut self) -> LexerResult<()> {
        self.next_decimal_digit()?;
        self.take_while(|c| c >= '0' && c <= '9');
        Ok(())
    }

    // intLit     = decimalLit | octalLit | hexLit
    pub fn next_int_lit_opt(&mut self) -> LexerResult<Option<u64>> {
        assert_ne!(ParserLanguage::Json, self.language);

        self.skip_ws()?;
        if let Some(i) = self.next_hex_lit_opt()? {
            return Ok(Some(i));
        }
        if let Some(i) = self.next_decimal_octal_lit_opt()? {
            return Ok(Some(i));
        }
        Ok(None)
    }

    // Floating-point literals

    // exponent  = ( "e" | "E" ) [ "+" | "-" ] decimals
    fn next_exponent_opt(&mut self) -> LexerResult<Option<()>> {
        if self.next_char_if_in("eE") != None {
            self.next_char_if_in("+-");
            self.next_decimal_digits()?;
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    // floatLit = ( decimals "." [ decimals ] [ exponent ] | decimals exponent | "."decimals [ exponent ] ) | "inf" | "nan"
    fn next_float_lit(&mut self) -> LexerResult<()> {
        assert_ne!(ParserLanguage::Json, self.language);

        // "inf" and "nan" are handled as part of ident
        if self.next_char_if_eq('.') {
            self.next_decimal_digits()?;
            self.next_exponent_opt()?;
        } else {
            self.next_decimal_digits()?;
            if self.next_char_if_eq('.') {
                self.next_decimal_digits()?;
                self.next_exponent_opt()?;
            } else {
                if self.next_exponent_opt()? == None {
                    return Err(LexerError::IncorrectFloatLit);
                }
            }
        }
        Ok(())
    }

    // String literals

    // charValue = hexEscape | octEscape | charEscape | /[^\0\n\\]/
    // hexEscape = '\' ( "x" | "X" ) hexDigit hexDigit
    // https://github.com/google/protobuf/issues/4560
    // octEscape = '\' octalDigit octalDigit octalDigit
    // charEscape = '\' ( "a" | "b" | "f" | "n" | "r" | "t" | "v" | '\' | "'" | '"' )
    // quote = "'" | '"'
    pub(crate) fn next_str_lit_bytes(&mut self) -> LexerResult<DecodedBytes> {
        match self.next_char()? {
            '\\' => {
                match self.next_char()? {
                    '\'' => Ok(DecodedBytes::byte(b'\'')),
                    '"' => Ok(DecodedBytes::byte(b'"')),
                    '\\' => Ok(DecodedBytes::byte(b'\\')),
                    'a' => Ok(DecodedBytes::byte(b'\x07')),
                    'b' => Ok(DecodedBytes::byte(b'\x08')),
                    'f' => Ok(DecodedBytes::byte(b'\x0c')),
                    'n' => Ok(DecodedBytes::byte(b'\n')),
                    'r' => Ok(DecodedBytes::byte(b'\r')),
                    't' => Ok(DecodedBytes::byte(b'\t')),
                    'v' => Ok(DecodedBytes::byte(b'\x0b')),
                    'x' => {
                        let d1 = self.next_hex_digit()? as u8;
                        let d2 = self.next_hex_digit()? as u8;
                        Ok(DecodedBytes::byte((d1 << 4) | d2))
                    }
                    d if d >= '0' && d <= '7' => {
                        let mut r = d as u8 - b'0';
                        for _ in 0..2 {
                            match self.next_octal_digit() {
                                Err(_) => break,
                                Ok(d) => r = (r << 3) + d as u8,
                            }
                        }
                        Ok(DecodedBytes::byte(r))
                    }
                    // https://github.com/google/protobuf/issues/4562
                    c => Ok(DecodedBytes::char(c)),
                }
            }
            '\n' | '\0' => Err(LexerError::IncorrectInput),
            c => Ok(DecodedBytes::char(c)),
        }
    }

    fn char_try_from(i: u32) -> LexerResult<char> {
        char::try_from(i).map_err(|_| LexerError::IncorrectUnicodeChar)
    }

    pub fn next_json_char_value(&mut self) -> LexerResult<char> {
        match self.next_char()? {
            '\\' => match self.next_char()? {
                '"' => Ok('"'),
                '\'' => Ok('\''),
                '\\' => Ok('\\'),
                '/' => Ok('/'),
                'b' => Ok('\x08'),
                'f' => Ok('\x0c'),
                'n' => Ok('\n'),
                'r' => Ok('\r'),
                't' => Ok('\t'),
                'u' => {
                    let mut v = 0;
                    for _ in 0..4 {
                        let digit = self.next_hex_digit()?;
                        v = v * 16 + digit;
                    }
                    Self::char_try_from(v)
                }
                _ => Err(LexerError::IncorrectJsonEscape),
            },
            c => Ok(c),
        }
    }

    // https://github.com/google/protobuf/issues/4564
    // strLit = ( "'" { charValue } "'" ) | ( '"' { charValue } '"' )
    fn next_str_lit_raw(&mut self) -> LexerResult<String> {
        let mut raw = String::new();

        let mut first = true;
        loop {
            if !first {
                self.skip_ws()?;
            }

            let start = self.pos;

            let q = match self.next_char_if_in("'\"") {
                Some(q) => q,
                None if !first => break,
                None => return Err(LexerError::IncorrectInput),
            };
            first = false;
            while self.lookahead_char() != Some(q) {
                self.next_str_lit_bytes()?;
            }
            self.next_char_expect_eq(q)?;

            raw.push_str(&self.input[start + 1..self.pos - 1]);
        }
        Ok(raw)
    }

    fn next_str_lit_raw_opt(&mut self) -> LexerResult<Option<String>> {
        if self.lookahead_char_is_in("'\"") {
            Ok(Some(self.next_str_lit_raw()?))
        } else {
            Ok(None)
        }
    }

    /// Parse next token as JSON number
    fn next_json_number_opt(&mut self) -> LexerResult<Option<JsonNumberLit>> {
        assert_eq!(ParserLanguage::Json, self.language);

        fn is_digit(c: char) -> bool {
            c >= '0' && c <= '9'
        }

        fn is_digit_1_9(c: char) -> bool {
            c >= '1' && c <= '9'
        }

        if !self.lookahead_char_is_in("-0123456789") {
            return Ok(None);
        }

        let mut s = String::new();
        if self.next_char_if_eq('-') {
            s.push('-');
        }

        if self.next_char_if_eq('0') {
            s.push('0');
        } else {
            s.push(self.next_char_expect(is_digit_1_9, LexerError::IncorrectJsonNumber)?);
            while let Some(c) = self.next_char_if(is_digit) {
                s.push(c);
            }
        }

        if self.next_char_if_eq('.') {
            s.push('.');
            s.push(self.next_char_expect(is_digit, LexerError::IncorrectJsonNumber)?);
            while let Some(c) = self.next_char_if(is_digit) {
                s.push(c);
            }
        }

        if let Some(c) = self.next_char_if_in("eE") {
            s.push(c);
            if let Some(c) = self.next_char_if_in("+-") {
                s.push(c);
            }
            s.push(self.next_char_expect(is_digit, LexerError::IncorrectJsonNumber)?);
            while let Some(c) = self.next_char_if(is_digit) {
                s.push(c);
            }
        }

        Ok(Some(JsonNumberLit(s)))
    }

    fn next_token_inner(&mut self) -> LexerResult<Token> {
        if self.language == ParserLanguage::Json {
            if let Some(v) = self.next_json_number_opt()? {
                return Ok(Token::JsonNumber(v));
            }
        }

        if let Some(ident) = self.next_ident_opt()? {
            let token = if self.language != ParserLanguage::Json && ident == float::PROTOBUF_NAN {
                Token::FloatLit(f64::NAN)
            } else if self.language != ParserLanguage::Json && ident == float::PROTOBUF_INF {
                Token::FloatLit(f64::INFINITY)
            } else {
                Token::Ident(ident.to_owned())
            };
            return Ok(token);
        }

        if self.language != ParserLanguage::Json {
            let mut clone = self.clone();
            let pos = clone.pos;
            if let Ok(_) = clone.next_float_lit() {
                let f = float::parse_protobuf_float(&self.input[pos..clone.pos])?;
                *self = clone;
                return Ok(Token::FloatLit(f));
            }

            if let Some(lit) = self.next_int_lit_opt()? {
                return Ok(Token::IntLit(lit));
            }
        }

        if let Some(escaped) = self.next_str_lit_raw_opt()? {
            return Ok(Token::StrLit(StrLit { escaped }));
        }

        // This branch must be after str lit
        if let Some(c) = self.next_char_if(|c| c.is_ascii_punctuation()) {
            return Ok(Token::Symbol(c));
        }

        if let Some(ident) = self.next_ident_opt()? {
            return Ok(Token::Ident(ident));
        }

        Err(LexerError::IncorrectInput)
    }

    pub fn next_token(&mut self) -> LexerResult<Option<TokenWithLocation>> {
        self.skip_ws()?;
        let loc = self.loc;

        Ok(if self.eof() {
            None
        } else {
            let token = self.next_token_inner()?;
            // Skip whitespace here to update location
            // to the beginning of the next token
            self.skip_ws()?;
            Some(TokenWithLocation { token, loc })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn lex<P, R>(input: &str, parse_what: P) -> R
    where
        P: FnOnce(&mut Lexer) -> LexerResult<R>,
    {
        let mut lexer = Lexer::new(input, ParserLanguage::Proto);
        let r = parse_what(&mut lexer).expect(&format!("lexer failed at {}", lexer.loc));
        assert!(lexer.eof(), "check eof failed at {}", lexer.loc);
        r
    }

    fn lex_opt<P, R>(input: &str, parse_what: P) -> R
    where
        P: FnOnce(&mut Lexer) -> LexerResult<Option<R>>,
    {
        let mut lexer = Lexer::new(input, ParserLanguage::Proto);
        let o = parse_what(&mut lexer).expect(&format!("lexer failed at {}", lexer.loc));
        let r = o.expect(&format!("lexer returned none at {}", lexer.loc));
        assert!(lexer.eof(), "check eof failed at {}", lexer.loc);
        r
    }

    #[test]
    fn test_lexer_int_lit() {
        let msg = r#"10"#;
        let mess = lex_opt(msg, |p| p.next_int_lit_opt());
        assert_eq!(10, mess);
    }

    #[test]
    fn test_lexer_float_lit() {
        let msg = r#"12.3"#;
        let mess = lex(msg, |p| p.next_token_inner());
        assert_eq!(Token::FloatLit(12.3), mess);
    }

    #[test]
    fn test_lexer_float_lit_leading_zeros_in_exp() {
        let msg = r#"1e00009"#;
        let mess = lex(msg, |p| p.next_token_inner());
        assert_eq!(Token::FloatLit(1_000_000_000.0), mess);
    }
}
