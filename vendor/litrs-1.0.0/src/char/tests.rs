use super::CharLit;
use crate::{
    test_util::{assert_parse_ok_eq, assert_roundtrip},
    Literal,
};

// ===== Utility functions =======================================================================

macro_rules! check {
    ($lit:literal) => { check!($lit, stringify!($lit), "") };
    ($lit:literal, $input:expr, $suffix:literal) => {
        let input = $input;
        let expected = CharLit {
            raw: input,
            start_suffix: input.len() - $suffix.len(),
            value: $lit,
        };

        assert_parse_ok_eq(input, CharLit::parse(input), expected.clone(), "CharLit::parse");
        assert_parse_ok_eq(input, Literal::parse(input), Literal::Char(expected), "Literal::parse");
        let lit = CharLit::parse(input).unwrap();
        assert_eq!(lit.value(), $lit);
        assert_eq!(lit.suffix(), $suffix);
        assert_roundtrip(expected.to_owned(), input);
    };
}


// ===== Actual tests ============================================================================

#[test]
fn alphanumeric() {
    check!('a');
    check!('b');
    check!('y');
    check!('z');
    check!('A');
    check!('B');
    check!('Y');
    check!('Z');

    check!('0');
    check!('1');
    check!('8');
    check!('9');
}

#[test]
fn special_chars() {
    check!(' ');
    check!('!');
    check!('"');
    check!('#');
    check!('$');
    check!('%');
    check!('&');
    check!('(');
    check!(')');
    check!('*');
    check!('+');
    check!(',');
    check!('-');
    check!('.');
    check!('/');
    check!(':');
    check!(';');
    check!('<');
    check!('=');
    check!('>');
    check!('?');
    check!('@');
    check!('[');
    check!(']');
    check!('^');
    check!('_');
    check!('`');
    check!('{');
    check!('|');
    check!('}');
    check!('~');
}

#[test]
fn unicode() {
    check!('న');
    check!('犬');
    check!('🦊');
}

#[test]
fn quote_escapes() {
    check!('\'');
    check!('\"');
}

#[test]
fn ascii_escapes() {
    check!('\n');
    check!('\r');
    check!('\t');
    check!('\\');
    check!('\0');

    check!('\x00');
    check!('\x01');
    check!('\x0c');
    check!('\x0D');
    check!('\x13');
    check!('\x30');
    check!('\x30');
    check!('\x4B');
    check!('\x6b');
    check!('\x7F');
    check!('\x7f');
}

#[test]
fn unicode_escapes() {
    check!('\u{0}');
    check!('\u{00}');
    check!('\u{b}');
    check!('\u{B}');
    check!('\u{7e}');
    check!('\u{E4}');
    check!('\u{e4}');
    check!('\u{fc}');
    check!('\u{Fc}');
    check!('\u{fC}');
    check!('\u{FC}');
    check!('\u{b10}');
    check!('\u{B10}');
    check!('\u{0b10}');
    check!('\u{2764}');
    check!('\u{1f602}');
    check!('\u{1F602}');

    check!('\u{0}');
    check!('\u{0__}');
    check!('\u{3_b}');
    check!('\u{1_F_6_0_2}');
    check!('\u{1_F6_02_____}');
}

#[test]
fn suffixes() {
    check!('a', r##"'a'peter"##, "peter");
    check!('#', r##"'#'peter"##, "peter");
    check!('\n', r##"'\n'peter"##, "peter");
    check!('\'', r##"'\''peter"##, "peter");
    check!('\"', r##"'\"'peter"##, "peter");
}

#[test]
fn invald_ascii_escapes() {
    assert_err!(CharLit, r"'\x80'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\x81'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\x8a'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\x8F'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\xa0'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\xB0'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\xc3'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\xDf'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\xff'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\xfF'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\xFf'", NonAsciiXEscape, 1..5);
    assert_err!(CharLit, r"'\xFF'", NonAsciiXEscape, 1..5);
}

#[test]
fn invalid_escapes() {
    assert_err!(CharLit, r"'\a'", UnknownEscape, 1..3);
    assert_err!(CharLit, r"'\y'", UnknownEscape, 1..3);
    assert_err!(CharLit, r"'\", UnterminatedEscape, 1);
    assert_err!(CharLit, r"'\x'", UnterminatedEscape, 1..4);
    assert_err!(CharLit, r"'\x1'", InvalidXEscape, 1..5);
    assert_err!(CharLit, r"'\xaj'", InvalidXEscape, 1..5);
    assert_err!(CharLit, r"'\xjb'", InvalidXEscape, 1..5);
}

#[test]
fn invalid_unicode_escapes() {
    assert_err!(CharLit, r"'\u'", UnicodeEscapeWithoutBrace, 1..3);
    assert_err!(CharLit, r"'\u '", UnicodeEscapeWithoutBrace, 1..3);
    assert_err!(CharLit, r"'\u3'", UnicodeEscapeWithoutBrace, 1..3);

    assert_err!(CharLit, r"'\u{'", UnterminatedUnicodeEscape, 1..5);
    assert_err!(CharLit, r"'\u{12'", UnterminatedUnicodeEscape, 1..7);
    assert_err!(CharLit, r"'\u{a0b'", UnterminatedUnicodeEscape, 1..8);
    assert_err!(CharLit, r"'\u{a0_b  '", UnterminatedUnicodeEscape, 1..11);

    assert_err!(CharLit, r"'\u{_}'", InvalidStartOfUnicodeEscape, 4);
    assert_err!(CharLit, r"'\u{_5f}'", InvalidStartOfUnicodeEscape, 4);

    assert_err!(CharLit, r"'\u{x}'", NonHexDigitInUnicodeEscape, 4);
    assert_err!(CharLit, r"'\u{0x}'", NonHexDigitInUnicodeEscape, 5);
    assert_err!(CharLit, r"'\u{3bx}'", NonHexDigitInUnicodeEscape, 6);
    assert_err!(CharLit, r"'\u{3b_x}'", NonHexDigitInUnicodeEscape, 7);
    assert_err!(CharLit, r"'\u{4x_}'", NonHexDigitInUnicodeEscape, 5);

    assert_err!(CharLit, r"'\u{1234567}'", TooManyDigitInUnicodeEscape, 10);
    assert_err!(CharLit, r"'\u{1234567}'", TooManyDigitInUnicodeEscape, 10);
    assert_err!(CharLit, r"'\u{1_23_4_56_7}'", TooManyDigitInUnicodeEscape, 14);
    assert_err!(CharLit, r"'\u{abcdef123}'", TooManyDigitInUnicodeEscape, 10);

    assert_err!(CharLit, r"'\u{110000}'", InvalidUnicodeEscapeChar, 1..11);
}

#[test]
fn parse_err() {
    assert_err!(CharLit, r"''", EmptyCharLiteral, None);
    assert_err!(CharLit, r"' ''", UnexpectedChar, 3);

    assert_err!(CharLit, r"'", UnterminatedCharLiteral, None);
    assert_err!(CharLit, r"'a", UnterminatedCharLiteral, None);
    assert_err!(CharLit, r"'\n", UnterminatedCharLiteral, None);
    assert_err!(CharLit, r"'\x35", UnterminatedCharLiteral, None);

    assert_err!(CharLit, r"'ab'", OverlongCharLiteral, None);
    assert_err!(CharLit, r"'a _'", OverlongCharLiteral, None);
    assert_err!(CharLit, r"'\n3'", OverlongCharLiteral, None);

    assert_err!(CharLit, r"", Empty, None);

    assert_err!(CharLit, r"'''", UnescapedSingleQuote, 1);
    assert_err!(CharLit, r"''''", UnescapedSingleQuote, 1);

    assert_err!(CharLit, "'\n'", UnescapedSpecialWhitespace, 1);
    assert_err!(CharLit, "'\t'", UnescapedSpecialWhitespace, 1);
    assert_err!(CharLit, "'\r'", UnescapedSpecialWhitespace, 1);
}
