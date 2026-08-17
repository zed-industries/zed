use anyhow::{Result, bail};
use std::char;
use std::fmt;
use std::str;
use unicode_xid::UnicodeXID;

use self::Token::*;

#[derive(Clone)]
pub struct Tokenizer<'a> {
    input: &'a str,
    span_offset: u32,
    chars: CrlfFold<'a>,
    require_f32_f64: bool,
}

#[derive(Clone)]
struct CrlfFold<'a> {
    chars: str::CharIndices<'a>,
}

/// A span, designating a range of bytes where a token is located.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Span {
    /// The start of the range.
    pub start: u32,
    /// The end of the range (exclusive).
    pub end: u32,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Token {
    Whitespace,
    Comment,

    Equals,
    Comma,
    Colon,
    Period,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LessThan,
    GreaterThan,
    RArrow,
    Star,
    At,
    Slash,
    Plus,
    Minus,

    Use,
    Type,
    Func,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F32,
    F64,
    Char,
    Record,
    Resource,
    Own,
    Borrow,
    Flags,
    Variant,
    Enum,
    Bool,
    String_,
    Option_,
    Result_,
    Future,
    Stream,
    ErrorContext,
    List,
    Map,
    Underscore,
    As,
    From_,
    Static,
    Interface,
    Tuple,
    Import,
    Export,
    World,
    Package,
    Constructor,
    Async,

    Id,
    ExplicitId,

    Integer,

    Include,
    With,
}

#[derive(Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Error {
    InvalidCharInId(u32, char),
    IdPartEmpty(u32),
    InvalidEscape(u32, char),
    Unexpected(u32, char),
    UnterminatedComment(u32),
    Wanted {
        at: u32,
        expected: &'static str,
        found: &'static str,
    },
}

// NB: keep in sync with `crates/wit-component/src/printing.rs`.
const REQUIRE_F32_F64_BY_DEFAULT: bool = true;

impl<'a> Tokenizer<'a> {
    pub fn new(
        input: &'a str,
        span_offset: u32,
        require_f32_f64: Option<bool>,
    ) -> Result<Tokenizer<'a>> {
        detect_invalid_input(input)?;

        let mut t = Tokenizer {
            input,
            span_offset,
            chars: CrlfFold {
                chars: input.char_indices(),
            },
            require_f32_f64: require_f32_f64.unwrap_or_else(|| {
                match std::env::var("WIT_REQUIRE_F32_F64") {
                    Ok(s) => s == "1",
                    Err(_) => REQUIRE_F32_F64_BY_DEFAULT,
                }
            }),
        };
        // Eat utf-8 BOM
        t.eatc('\u{feff}');
        Ok(t)
    }

    pub fn expect_semicolon(&mut self) -> Result<()> {
        self.expect(Token::Semicolon)?;
        Ok(())
    }

    pub fn get_span(&self, span: Span) -> &'a str {
        let start = usize::try_from(span.start - self.span_offset).unwrap();
        let end = usize::try_from(span.end - self.span_offset).unwrap();
        &self.input[start..end]
    }

    pub fn parse_id(&self, span: Span) -> Result<&'a str> {
        let ret = self.get_span(span);
        validate_id(span.start, &ret)?;
        Ok(ret)
    }

    pub fn parse_explicit_id(&self, span: Span) -> Result<&'a str> {
        let token = self.get_span(span);
        let id_part = token.strip_prefix('%').unwrap();
        validate_id(span.start, id_part)?;
        Ok(id_part)
    }

    pub fn next(&mut self) -> Result<Option<(Span, Token)>, Error> {
        loop {
            match self.next_raw()? {
                Some((_, Token::Whitespace)) | Some((_, Token::Comment)) => {}
                other => break Ok(other),
            }
        }
    }

    /// Three possibilities when calling this method: an `Err(...)` indicates that lexing failed, an
    /// `Ok(Some(...))` produces the next token, and `Ok(None)` indicates that there are no more
    /// tokens available.
    pub fn next_raw(&mut self) -> Result<Option<(Span, Token)>, Error> {
        let (str_start, ch) = match self.chars.next() {
            Some(pair) => pair,
            None => return Ok(None),
        };
        let start = self.span_offset + u32::try_from(str_start).unwrap();
        let token = match ch {
            '\n' | '\t' | ' ' => {
                // Eat all contiguous whitespace tokens
                while self.eatc(' ') || self.eatc('\t') || self.eatc('\n') {}
                Whitespace
            }
            '/' => {
                // Eat a line comment if it's `//...`
                if self.eatc('/') {
                    for (_, ch) in &mut self.chars {
                        if ch == '\n' {
                            break;
                        }
                    }
                    Comment
                // eat a block comment if it's `/*...`
                } else if self.eatc('*') {
                    let mut depth = 1;
                    while depth > 0 {
                        let (_, ch) = match self.chars.next() {
                            Some(pair) => pair,
                            None => return Err(Error::UnterminatedComment(start)),
                        };
                        match ch {
                            '/' if self.eatc('*') => depth += 1,
                            '*' if self.eatc('/') => depth -= 1,
                            _ => {}
                        }
                    }
                    Comment
                } else {
                    Slash
                }
            }
            '=' => Equals,
            ',' => Comma,
            ':' => Colon,
            '.' => Period,
            ';' => Semicolon,
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '<' => LessThan,
            '>' => GreaterThan,
            '*' => Star,
            '@' => At,
            '-' => {
                if self.eatc('>') {
                    RArrow
                } else {
                    Minus
                }
            }
            '+' => Plus,
            '%' => {
                let mut iter = self.chars.clone();
                if let Some((_, ch)) = iter.next() {
                    if is_keylike_start(ch) {
                        self.chars = iter.clone();
                        while let Some((_, ch)) = iter.next() {
                            if !is_keylike_continue(ch) {
                                break;
                            }
                            self.chars = iter.clone();
                        }
                    }
                }
                ExplicitId
            }
            ch if is_keylike_start(ch) => {
                let remaining = self.chars.chars.as_str().len();
                let mut iter = self.chars.clone();
                while let Some((_, ch)) = iter.next() {
                    if !is_keylike_continue(ch) {
                        break;
                    }
                    self.chars = iter.clone();
                }
                let str_end =
                    str_start + ch.len_utf8() + (remaining - self.chars.chars.as_str().len());
                match &self.input[str_start..str_end] {
                    "use" => Use,
                    "type" => Type,
                    "func" => Func,
                    "u8" => U8,
                    "u16" => U16,
                    "u32" => U32,
                    "u64" => U64,
                    "s8" => S8,
                    "s16" => S16,
                    "s32" => S32,
                    "s64" => S64,
                    "f32" => F32,
                    "f64" => F64,
                    "float32" if !self.require_f32_f64 => F32,
                    "float64" if !self.require_f32_f64 => F64,
                    "char" => Char,
                    "resource" => Resource,
                    "own" => Own,
                    "borrow" => Borrow,
                    "record" => Record,
                    "flags" => Flags,
                    "variant" => Variant,
                    "enum" => Enum,
                    "bool" => Bool,
                    "string" => String_,
                    "option" => Option_,
                    "result" => Result_,
                    "future" => Future,
                    "stream" => Stream,
                    "error-context" => ErrorContext,
                    "list" => List,
                    "map" => Map,
                    "_" => Underscore,
                    "as" => As,
                    "from" => From_,
                    "static" => Static,
                    "interface" => Interface,
                    "tuple" => Tuple,
                    "world" => World,
                    "import" => Import,
                    "export" => Export,
                    "package" => Package,
                    "constructor" => Constructor,
                    "include" => Include,
                    "with" => With,
                    "async" => Async,
                    _ => Id,
                }
            }

            ch if ch.is_ascii_digit() => {
                let mut iter = self.chars.clone();
                while let Some((_, ch)) = iter.next() {
                    if !ch.is_ascii_digit() {
                        break;
                    }
                    self.chars = iter.clone();
                }

                Integer
            }

            ch => return Err(Error::Unexpected(start, ch)),
        };
        let end = match self.chars.clone().next() {
            Some((i, _)) => i,
            None => self.input.len(),
        };

        let end = self.span_offset + u32::try_from(end).unwrap();
        Ok(Some((Span { start, end }, token)))
    }

    pub fn eat(&mut self, expected: Token) -> Result<bool, Error> {
        let mut other = self.clone();
        match other.next()? {
            Some((_span, found)) if expected == found => {
                *self = other;
                Ok(true)
            }
            Some(_) => Ok(false),
            None => Ok(false),
        }
    }

    pub fn expect(&mut self, expected: Token) -> Result<Span, Error> {
        match self.next()? {
            Some((span, found)) => {
                if expected == found {
                    Ok(span)
                } else {
                    Err(Error::Wanted {
                        at: span.start,
                        expected: expected.describe(),
                        found: found.describe(),
                    })
                }
            }
            None => Err(Error::Wanted {
                at: self.span_offset + u32::try_from(self.input.len()).unwrap(),
                expected: expected.describe(),
                found: "eof",
            }),
        }
    }

    fn eatc(&mut self, ch: char) -> bool {
        let mut iter = self.chars.clone();
        match iter.next() {
            Some((_, ch2)) if ch == ch2 => {
                self.chars = iter;
                true
            }
            _ => false,
        }
    }

    pub fn eof_span(&self) -> Span {
        let end = self.span_offset + u32::try_from(self.input.len()).unwrap();
        Span { start: end, end }
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

fn detect_invalid_input(input: &str) -> Result<()> {
    // Disallow specific codepoints.
    let mut line = 1;
    for ch in input.chars() {
        match ch {
            '\n' => line += 1,
            '\r' | '\t' => {}

            // Bidirectional override codepoints can be used to craft source code that
            // appears to have a different meaning than its actual meaning. See
            // [CVE-2021-42574] for background and motivation.
            //
            // [CVE-2021-42574]: https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-42574
            '\u{202a}' | '\u{202b}' | '\u{202c}' | '\u{202d}' | '\u{202e}' | '\u{2066}'
            | '\u{2067}' | '\u{2068}' | '\u{2069}' => {
                bail!(
                    "Input contains bidirectional override codepoint {:?} at line {}",
                    ch.escape_unicode(),
                    line
                );
            }

            // Disallow several characters which are deprecated or discouraged in Unicode.
            //
            // U+149 deprecated; see Unicode 13.0.0, sec. 7.1 Latin, Compatibility Digraphs.
            // U+673 deprecated; see Unicode 13.0.0, sec. 9.2 Arabic, Additional Vowel Marks.
            // U+F77 and U+F79 deprecated; see Unicode 13.0.0, sec. 13.4 Tibetan, Vowels.
            // U+17A3 and U+17A4 deprecated, and U+17B4 and U+17B5 discouraged; see
            // Unicode 13.0.0, sec. 16.4 Khmer, Characters Whose Use Is Discouraged.
            '\u{149}' | '\u{673}' | '\u{f77}' | '\u{f79}' | '\u{17a3}' | '\u{17a4}'
            | '\u{17b4}' | '\u{17b5}' => {
                bail!(
                    "Codepoint {:?} at line {} is discouraged by Unicode",
                    ch.escape_unicode(),
                    line
                );
            }

            // Disallow control codes other than the ones explicitly recognized above,
            // so that viewing a wit file on a terminal doesn't have surprising side
            // effects or appear to have a different meaning than its actual meaning.
            ch if ch.is_control() => {
                bail!("Control code '{}' at line {}", ch.escape_unicode(), line);
            }

            _ => {}
        }
    }

    Ok(())
}

fn is_keylike_start(ch: char) -> bool {
    // Lex any XID start, `_`, or '-'. These aren't all valid identifier chars,
    // but we'll diagnose that after we've lexed the full string.
    UnicodeXID::is_xid_start(ch) || ch == '_' || ch == '-'
}

fn is_keylike_continue(ch: char) -> bool {
    // Lex any XID continue (which includes `_`) or '-'.
    UnicodeXID::is_xid_continue(ch) || ch == '-'
}

pub fn validate_id(start: u32, id: &str) -> Result<(), Error> {
    // IDs must have at least one part.
    if id.is_empty() {
        return Err(Error::IdPartEmpty(start));
    }

    // Ids consist of parts separated by '-'s.
    for (idx, part) in id.split('-').enumerate() {
        // Parts must be non-empty and contain either all ASCII lowercase or
        // all ASCII uppercase. Non-first segment can also start with a digit.
        let Some(first_char) = part.chars().next() else {
            return Err(Error::IdPartEmpty(start));
        };
        if idx == 0 && !first_char.is_ascii_alphabetic() {
            return Err(Error::InvalidCharInId(start, first_char));
        }
        let mut upper = None;
        for ch in part.chars() {
            if ch.is_ascii_digit() {
                // Digits are accepted in both uppercase and lowercase segments.
            } else if ch.is_ascii_uppercase() {
                if upper.is_none() {
                    upper = Some(true);
                } else if let Some(false) = upper {
                    return Err(Error::InvalidCharInId(start, ch));
                }
            } else if ch.is_ascii_lowercase() {
                if upper.is_none() {
                    upper = Some(false);
                } else if let Some(true) = upper {
                    return Err(Error::InvalidCharInId(start, ch));
                }
            } else {
                return Err(Error::InvalidCharInId(start, ch));
            }
        }
    }

    Ok(())
}

impl Token {
    pub fn describe(&self) -> &'static str {
        match self {
            Whitespace => "whitespace",
            Comment => "a comment",
            Equals => "'='",
            Comma => "','",
            Colon => "':'",
            Period => "'.'",
            Semicolon => "';'",
            LeftParen => "'('",
            RightParen => "')'",
            LeftBrace => "'{'",
            RightBrace => "'}'",
            LessThan => "'<'",
            GreaterThan => "'>'",
            Use => "keyword `use`",
            Type => "keyword `type`",
            Func => "keyword `func`",
            U8 => "keyword `u8`",
            U16 => "keyword `u16`",
            U32 => "keyword `u32`",
            U64 => "keyword `u64`",
            S8 => "keyword `s8`",
            S16 => "keyword `s16`",
            S32 => "keyword `s32`",
            S64 => "keyword `s64`",
            F32 => "keyword `f32`",
            F64 => "keyword `f64`",
            Char => "keyword `char`",
            Own => "keyword `own`",
            Borrow => "keyword `borrow`",
            Resource => "keyword `resource`",
            Record => "keyword `record`",
            Flags => "keyword `flags`",
            Variant => "keyword `variant`",
            Enum => "keyword `enum`",
            Bool => "keyword `bool`",
            String_ => "keyword `string`",
            Option_ => "keyword `option`",
            Result_ => "keyword `result`",
            Future => "keyword `future`",
            Stream => "keyword `stream`",
            ErrorContext => "keyword `error-context`",
            List => "keyword `list`",
            Map => "keyword `map`",
            Underscore => "keyword `_`",
            Id => "an identifier",
            ExplicitId => "an '%' identifier",
            RArrow => "`->`",
            Star => "`*`",
            At => "`@`",
            Slash => "`/`",
            Plus => "`+`",
            Minus => "`-`",
            As => "keyword `as`",
            From_ => "keyword `from`",
            Static => "keyword `static`",
            Interface => "keyword `interface`",
            Tuple => "keyword `tuple`",
            Import => "keyword `import`",
            Export => "keyword `export`",
            World => "keyword `world`",
            Package => "keyword `package`",
            Constructor => "keyword `constructor`",
            Integer => "an integer",
            Include => "keyword `include`",
            With => "keyword `with`",
            Async => "keyword `async`",
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unexpected(_, ch) => write!(f, "unexpected character {ch:?}"),
            Error::UnterminatedComment(_) => write!(f, "unterminated block comment"),
            Error::Wanted {
                expected, found, ..
            } => write!(f, "expected {expected}, found {found}"),
            Error::InvalidCharInId(_, ch) => write!(f, "invalid character in identifier {ch:?}"),
            Error::IdPartEmpty(_) => write!(f, "identifiers must have characters between '-'s"),
            Error::InvalidEscape(_, ch) => write!(f, "invalid escape in string {ch:?}"),
        }
    }
}

#[test]
fn test_validate_id() {
    validate_id(0, "apple").unwrap();
    validate_id(0, "apple-pear").unwrap();
    validate_id(0, "apple-pear-grape").unwrap();
    validate_id(0, "a0").unwrap();
    validate_id(0, "a").unwrap();
    validate_id(0, "a-a").unwrap();
    validate_id(0, "bool").unwrap();
    validate_id(0, "APPLE").unwrap();
    validate_id(0, "APPLE-PEAR").unwrap();
    validate_id(0, "APPLE-PEAR-GRAPE").unwrap();
    validate_id(0, "apple-PEAR-grape").unwrap();
    validate_id(0, "APPLE-pear-GRAPE").unwrap();
    validate_id(0, "ENOENT").unwrap();
    validate_id(0, "is-XML").unwrap();
    validate_id(0, "apple-0").unwrap();
    validate_id(0, "a0-000-3d4a-54FF").unwrap();

    assert!(validate_id(0, "").is_err());
    assert!(validate_id(0, "0").is_err());
    assert!(validate_id(0, "%").is_err());
    assert!(validate_id(0, "$").is_err());
    assert!(validate_id(0, "0a").is_err());
    assert!(validate_id(0, ".").is_err());
    assert!(validate_id(0, "·").is_err());
    assert!(validate_id(0, "a a").is_err());
    assert!(validate_id(0, "_").is_err());
    assert!(validate_id(0, "-").is_err());
    assert!(validate_id(0, "a-").is_err());
    assert!(validate_id(0, "-a").is_err());
    assert!(validate_id(0, "Apple").is_err());
    assert!(validate_id(0, "applE").is_err());
    assert!(validate_id(0, "-apple-pear").is_err());
    assert!(validate_id(0, "apple-pear-").is_err());
    assert!(validate_id(0, "apple_pear").is_err());
    assert!(validate_id(0, "apple.pear").is_err());
    assert!(validate_id(0, "apple pear").is_err());
    assert!(validate_id(0, "apple/pear").is_err());
    assert!(validate_id(0, "apple|pear").is_err());
    assert!(validate_id(0, "apple-Pear").is_err());
    assert!(validate_id(0, "()()").is_err());
    assert!(validate_id(0, "").is_err());
    assert!(validate_id(0, "*").is_err());
    assert!(validate_id(0, "apple\u{5f3}pear").is_err());
    assert!(validate_id(0, "apple\u{200c}pear").is_err());
    assert!(validate_id(0, "apple\u{200d}pear").is_err());
    assert!(validate_id(0, "apple--pear").is_err());
    assert!(validate_id(0, "_apple").is_err());
    assert!(validate_id(0, "apple_").is_err());
    assert!(validate_id(0, "_Znwj").is_err());
    assert!(validate_id(0, "__i386").is_err());
    assert!(validate_id(0, "__i386__").is_err());
    assert!(validate_id(0, "Москва").is_err());
    assert!(validate_id(0, "garçon-hühnervögel-Москва-東京").is_err());
    assert!(validate_id(0, "a0-000-3d4A-54Ff").is_err());
    assert!(validate_id(0, "😼").is_err(), "non-identifier");
    assert!(validate_id(0, "\u{212b}").is_err(), "non-ascii");
}

#[test]
fn test_tokenizer() {
    fn collect(s: &str) -> Result<Vec<Token>> {
        let mut t = Tokenizer::new(s, 0, None)?;
        let mut tokens = Vec::new();
        while let Some(token) = t.next()? {
            tokens.push(token.1);
        }
        Ok(tokens)
    }

    assert_eq!(collect("").unwrap(), vec![]);
    assert_eq!(collect("_").unwrap(), vec![Token::Underscore]);
    assert_eq!(collect("apple").unwrap(), vec![Token::Id]);
    assert_eq!(collect("apple-pear").unwrap(), vec![Token::Id]);
    assert_eq!(collect("apple--pear").unwrap(), vec![Token::Id]);
    assert_eq!(collect("apple-Pear").unwrap(), vec![Token::Id]);
    assert_eq!(collect("apple-pear-grape").unwrap(), vec![Token::Id]);
    assert_eq!(collect("apple pear").unwrap(), vec![Token::Id, Token::Id]);
    assert_eq!(collect("_a_p_p_l_e_").unwrap(), vec![Token::Id]);
    assert_eq!(collect("garçon").unwrap(), vec![Token::Id]);
    assert_eq!(collect("hühnervögel").unwrap(), vec![Token::Id]);
    assert_eq!(collect("москва").unwrap(), vec![Token::Id]);
    assert_eq!(collect("東京").unwrap(), vec![Token::Id]);
    assert_eq!(
        collect("garçon-hühnervögel-москва-東京").unwrap(),
        vec![Token::Id]
    );
    assert_eq!(collect("a0").unwrap(), vec![Token::Id]);
    assert_eq!(collect("a").unwrap(), vec![Token::Id]);
    assert_eq!(collect("%a").unwrap(), vec![Token::ExplicitId]);
    assert_eq!(collect("%a-a").unwrap(), vec![Token::ExplicitId]);
    assert_eq!(collect("%bool").unwrap(), vec![Token::ExplicitId]);
    assert_eq!(collect("%").unwrap(), vec![Token::ExplicitId]);
    assert_eq!(collect("APPLE").unwrap(), vec![Token::Id]);
    assert_eq!(collect("APPLE-PEAR").unwrap(), vec![Token::Id]);
    assert_eq!(collect("APPLE-PEAR-GRAPE").unwrap(), vec![Token::Id]);
    assert_eq!(collect("apple-PEAR-grape").unwrap(), vec![Token::Id]);
    assert_eq!(collect("APPLE-pear-GRAPE").unwrap(), vec![Token::Id]);
    assert_eq!(collect("ENOENT").unwrap(), vec![Token::Id]);
    assert_eq!(collect("is-XML").unwrap(), vec![Token::Id]);

    assert_eq!(collect("func").unwrap(), vec![Token::Func]);
    assert_eq!(
        collect("a: func()").unwrap(),
        vec![
            Token::Id,
            Token::Colon,
            Token::Func,
            Token::LeftParen,
            Token::RightParen
        ]
    );

    assert_eq!(collect("resource").unwrap(), vec![Token::Resource]);

    assert_eq!(collect("own").unwrap(), vec![Token::Own]);
    assert_eq!(
        collect("own<some-id>").unwrap(),
        vec![Token::Own, Token::LessThan, Token::Id, Token::GreaterThan]
    );

    assert_eq!(collect("borrow").unwrap(), vec![Token::Borrow]);
    assert_eq!(
        collect("borrow<some-id>").unwrap(),
        vec![
            Token::Borrow,
            Token::LessThan,
            Token::Id,
            Token::GreaterThan
        ]
    );

    assert!(collect("\u{149}").is_err(), "strongly discouraged");
    assert!(collect("\u{673}").is_err(), "strongly discouraged");
    assert!(collect("\u{17a3}").is_err(), "strongly discouraged");
    assert!(collect("\u{17a4}").is_err(), "strongly discouraged");
    assert!(collect("\u{202a}").is_err(), "bidirectional override");
    assert!(collect("\u{2068}").is_err(), "bidirectional override");
    assert!(collect("\u{0}").is_err(), "control code");
    assert!(collect("\u{b}").is_err(), "control code");
    assert!(collect("\u{c}").is_err(), "control code");
    assert!(collect("\u{85}").is_err(), "control code");
}
