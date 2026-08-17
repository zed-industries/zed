use std::{fmt, ops::Range};


/// An error signaling that a different kind of token was expected. Returned by
/// the various `TryFrom` impls.
#[derive(Debug, Clone, Copy)]
pub struct InvalidToken {
    pub(crate) expected: TokenKind,
    pub(crate) actual: TokenKind,
    pub(crate) span: Span,
}

impl InvalidToken {
    /// Returns a token stream representing `compile_error!("msg");` where
    /// `"msg"` is the output of `self.to_string()`. **Panics if called outside
    /// of a proc-macro context!**
    pub fn to_compile_error(&self) -> proc_macro::TokenStream {
        use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, TokenTree};

        let span = match self.span {
            Span::One(s) => s,
            #[cfg(feature = "proc-macro2")]
            Span::Two(s) => s.unwrap(),
        };
        let msg = self.to_string();
        let tokens = vec![
            TokenTree::from(Ident::new("compile_error", span)),
            TokenTree::from(Punct::new('!', Spacing::Alone)),
            TokenTree::from(Group::new(
                Delimiter::Parenthesis,
                TokenTree::from(proc_macro::Literal::string(&msg)).into(),
            )),
        ];


        tokens.into_iter().map(|mut t| { t.set_span(span); t }).collect()
    }

    /// Like [`to_compile_error`][Self::to_compile_error], but returns a token
    /// stream from `proc_macro2` and does not panic outside of a proc-macro
    /// context.
    #[cfg(feature = "proc-macro2")]
    pub fn to_compile_error2(&self) -> proc_macro2::TokenStream {
        use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, TokenTree};

        let span = match self.span {
            Span::One(s) => proc_macro2::Span::from(s),
            Span::Two(s) => s,
        };
        let msg = self.to_string();
        let tokens = vec![
            TokenTree::from(Ident::new("compile_error", span)),
            TokenTree::from(Punct::new('!', Spacing::Alone)),
            TokenTree::from(Group::new(
                Delimiter::Parenthesis,
                TokenTree::from(proc_macro2::Literal::string(&msg)).into(),
            )),
        ];


        tokens.into_iter().map(|mut t| { t.set_span(span); t }).collect()
    }
}

impl std::error::Error for InvalidToken {}

impl fmt::Display for InvalidToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn kind_desc(kind: TokenKind) -> &'static str {
            match kind {
                TokenKind::Punct => "a punctuation character",
                TokenKind::Ident => "an identifier",
                TokenKind::Group => "a group",
                TokenKind::Literal => "a literal",
                TokenKind::BoolLit => "a bool literal (`true` or `false`)",
                TokenKind::ByteLit => "a byte literal (e.g. `b'r')",
                TokenKind::ByteStringLit => r#"a byte string literal (e.g. `b"fox"`)"#,
                TokenKind::CharLit => "a character literal (e.g. `'P'`)",
                TokenKind::FloatLit => "a float literal (e.g. `3.14`)",
                TokenKind::IntegerLit => "an integer literal (e.g. `27`)",
                TokenKind::StringLit => r#"a string literal (e.g. "Ferris")"#,
                TokenKind::CStringLit => r#"a C string literal (e.g. c"Ferris")"#,
            }
        }

        write!(f, "expected {}, but found {}", kind_desc(self.expected), kind_desc(self.actual))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Punct,
    Ident,
    Group,
    Literal,
    BoolLit,
    ByteLit,
    ByteStringLit,
    CharLit,
    FloatLit,
    IntegerLit,
    StringLit,
    CStringLit,
}

/// Unfortunately, we have to deal with both cases.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Span {
    One(proc_macro::Span),
    #[cfg(feature = "proc-macro2")]
    Two(proc_macro2::Span),
}

impl From<proc_macro::Span> for Span {
    fn from(src: proc_macro::Span) -> Self {
        Self::One(src)
    }
}

#[cfg(feature = "proc-macro2")]
impl From<proc_macro2::Span> for Span {
    fn from(src: proc_macro2::Span) -> Self {
        Self::Two(src)
    }
}

/// Errors during parsing.
///
/// This type should be seen primarily for error reporting and not for catching
/// specific cases. The span and error kind are not guaranteed to be stable
/// over different versions of this library, meaning that a returned error can
/// change from one version to the next. There are simply too many fringe cases
/// that are not easy to classify as a specific error kind. It depends entirely
/// on the specific parser code how an invalid input is categorized.
///
/// Consider these examples:
/// - `'\` can be seen as
///     - invalid escape in character literal, or
///     - unterminated character literal.
/// - `'''` can be seen as
///     - empty character literal, or
///     - unescaped quote character in character literal.
/// - `0b64` can be seen as
///     - binary integer literal with invalid digit 6, or
///     - binary integer literal with invalid digit 4, or
///     - decimal integer literal with invalid digit b, or
///     - decimal integer literal 0 with unknown type suffix `b64`.
///
/// If you want to see more if these examples, feel free to check out the unit
/// tests of this library.
///
/// While this library does its best to emit sensible and precise errors, and to
/// keep the returned errors as stable as possible, full stability cannot be
/// guaranteed.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub(crate) span: Option<Range<usize>>,
    pub(crate) kind: ParseErrorKind,
}

impl ParseError {
    /// Returns a span of this error, if available. **Note**: the returned span
    /// might change in future versions of this library. See [the documentation
    /// of this type][ParseError] for more information.
    pub fn span(&self) -> Option<Range<usize>> {
        self.span.clone()
    }

    /// Adds `offset` to the start and endpoint of the inner span.
    pub(crate) fn offset_span(self, offset: usize) -> Self {
        Self {
            span: self.span.map(|span| span.start + offset..span.end + offset),
            ..self
        }
    }
}

/// This is a free standing function instead of an associated one to reduce
/// noise around parsing code. There are lots of places that create errors, we
/// I wanna keep them as short as possible.
pub(crate) fn perr(span: impl SpanLike, kind: ParseErrorKind) -> ParseError {
    ParseError {
        span: span.into_span(),
        kind,
    }
}

pub(crate) trait SpanLike {
    fn into_span(self) -> Option<Range<usize>>;
}

impl SpanLike for Option<Range<usize>> {
    #[inline(always)]
    fn into_span(self) -> Option<Range<usize>> {
        self
    }
}
impl SpanLike for Range<usize> {
    #[inline(always)]
    fn into_span(self) -> Option<Range<usize>> {
        Some(self)
    }
}
impl SpanLike for usize {
    #[inline(always)]
    fn into_span(self) -> Option<Range<usize>> {
        Some(self..self + 1)
    }
}


/// Kinds of errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum ParseErrorKind {
    /// The input was an empty string
    Empty,

    /// An unexpected char was encountered.
    UnexpectedChar,

    /// Literal was not recognized.
    InvalidLiteral,

    /// Input does not start with decimal digit when trying to parse an integer.
    DoesNotStartWithDigit,

    /// A digit invalid for the specified integer base was found.
    InvalidDigit,

    /// Integer literal does not contain any valid digits.
    NoDigits,

    /// Exponent of a float literal does not contain any digits.
    NoExponentDigits,

    /// An unknown escape code, e.g. `\b`.
    UnknownEscape,

    /// A started escape sequence where the input ended before the escape was
    /// finished.
    UnterminatedEscape,

    /// An `\x` escape where the two digits are not valid hex digits.
    InvalidXEscape,

    /// A string or character literal using the `\xNN` escape where `NN > 0x7F`.
    NonAsciiXEscape,

    /// A `\u{...}` escape in a byte or byte string literal.
    UnicodeEscapeInByteLiteral,

    /// A Unicode escape that does not start with a hex digit.
    InvalidStartOfUnicodeEscape,

    /// A `\u{...}` escape that lacks the opening brace.
    UnicodeEscapeWithoutBrace,

    /// In a `\u{...}` escape, a non-hex digit and non-underscore character was
    /// found.
    NonHexDigitInUnicodeEscape,

    /// More than 6 digits found in unicode escape.
    TooManyDigitInUnicodeEscape,

    /// The value from a unicode escape does not represent a valid character.
    InvalidUnicodeEscapeChar,

    /// A `\u{..` escape that is not terminated (lacks the closing brace).
    UnterminatedUnicodeEscape,

    /// A character literal that's not terminated.
    UnterminatedCharLiteral,

    /// A character literal that contains more than one character.
    OverlongCharLiteral,

    /// An empty character literal, i.e. `''`.
    EmptyCharLiteral,

    UnterminatedByteLiteral,
    OverlongByteLiteral,
    EmptyByteLiteral,
    NonAsciiInByteLiteral,

    /// A `'` character was not escaped in a character or byte literal, or a `"`
    /// character was not escaped in a string or byte string literal.
    UnescapedSingleQuote,

    /// A \n, \t or \r raw character in a char or byte literal.
    UnescapedSpecialWhitespace,

    /// When parsing a character, byte, string or byte string literal directly
    /// and the input does not start with the corresponding quote character
    /// (plus optional raw string prefix).
    DoesNotStartWithQuote,

    /// Unterminated raw string literal.
    UnterminatedRawString,

    /// String literal without a `"` at the end.
    UnterminatedString,

    /// Invalid start for a string literal.
    InvalidStringLiteralStart,

    /// Invalid start for a byte literal.
    InvalidByteLiteralStart,

    InvalidByteStringLiteralStart,

    /// Not starting with `c"` or `cr`.
    InvalidCStringLiteralStart,

    /// A `\0` escape inside a C string.
    DisallowedNulEscape,

    /// A nul byte inside a C String.
    NulByte,

    /// `\r` in a (raw) string or (raw) byte string literal.
    CarriageReturn,

    /// Rust only allows 256 hashes in raw * string literals.
    TooManyHashes,

    /// Literal suffix is not a valid identifier.
    InvalidSuffix,

    /// Returned by `Float::parse` if an integer literal (no fractional nor
    /// exponent part) is passed.
    UnexpectedIntegerLit,

    /// Integer suffixes cannot start with `e` or `E` as this conflicts with the
    /// grammar for float literals.
    IntegerSuffixStartingWithE,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseErrorKind::*;

        let description = match self.kind {
            Empty => "input is empty",
            UnexpectedChar => "unexpected character",
            InvalidLiteral => "invalid literal",
            DoesNotStartWithDigit => "number literal does not start with decimal digit",
            InvalidDigit => "integer literal contains a digit invalid for its base",
            NoDigits => "integer literal does not contain any digits",
            NoExponentDigits => "exponent of floating point literal does not contain any digits",
            UnknownEscape => "unknown escape",
            UnterminatedEscape => "unterminated escape: input ended too soon",
            InvalidXEscape => r"invalid `\x` escape: not followed by two hex digits",
            NonAsciiXEscape => r"`\x` escape in char/string literal exceed ASCII range",
            UnicodeEscapeInByteLiteral => r"`\u{...}` escape in byte (string) literal not allowed",
            InvalidStartOfUnicodeEscape => r"invalid start of `\u{...}` escape",
            UnicodeEscapeWithoutBrace => r"`Unicode \u{...}` escape without opening brace",
            NonHexDigitInUnicodeEscape => r"non-hex digit found in `\u{...}` escape",
            TooManyDigitInUnicodeEscape => r"more than six digits in `\u{...}` escape",
            InvalidUnicodeEscapeChar => r"value specified in `\u{...}` escape is not a valid char",
            UnterminatedUnicodeEscape => r"unterminated `\u{...}` escape",
            UnterminatedCharLiteral => "character literal is not terminated",
            OverlongCharLiteral => "character literal contains more than one character",
            EmptyCharLiteral => "empty character literal",
            UnterminatedByteLiteral => "byte literal is not terminated",
            OverlongByteLiteral => "byte literal contains more than one byte",
            EmptyByteLiteral => "empty byte literal",
            NonAsciiInByteLiteral => "non ASCII character in byte (string) literal",
            UnescapedSingleQuote => "character literal contains unescaped ' character",
            UnescapedSpecialWhitespace => r"unescaped newline (\n), tab (\t) or cr (\r) character",
            DoesNotStartWithQuote => "invalid start for char/byte/string literal",
            UnterminatedRawString => "unterminated raw (byte) string literal",
            UnterminatedString => "unterminated (byte) string literal",
            InvalidStringLiteralStart => "invalid start for string literal",
            InvalidByteLiteralStart => "invalid start for byte literal",
            InvalidByteStringLiteralStart => "invalid start for byte string literal",
            InvalidCStringLiteralStart => "invalid start for C string literal",
            DisallowedNulEscape => r"`\0` escape not allowed inside C string literal",
            NulByte => r"nul byte not allowed inside C string literal",
            CarriageReturn => r"`\r` not allowed in string literals",
            TooManyHashes => "raw string literal has too many # symbols (max 256)",
            InvalidSuffix => "literal suffix is not a valid identifier",
            UnexpectedIntegerLit => "expected float literal, but found integer",
            IntegerSuffixStartingWithE => "integer literal suffix must not start with 'e' or 'E'",
        };

        description.fmt(f)?;
        if let Some(span) = &self.span {
            write!(f, " (at {}..{})", span.start, span.end)?;
        }

        Ok(())
    }
}
