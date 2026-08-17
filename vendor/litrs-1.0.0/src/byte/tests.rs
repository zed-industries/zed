use crate::{
    test_util::{assert_parse_ok_eq, assert_roundtrip},
    ByteLit, Literal,
};

// ===== Utility functions =======================================================================

macro_rules! check {
    ($lit:literal) => { check!($lit, stringify!($lit), "") };
    ($lit:literal, $input:expr, $suffix:literal) => {
        let input = $input;
        let expected = ByteLit {
            raw: input,
            start_suffix: input.len() - $suffix.len(),
            value: $lit,
        };

        assert_parse_ok_eq(input, ByteLit::parse(input), expected.clone(), "ByteLit::parse");
        assert_parse_ok_eq(input, Literal::parse(input), Literal::Byte(expected), "Literal::parse");
        let lit = ByteLit::parse(input).unwrap();
        assert_eq!(lit.value(), $lit);
        assert_eq!(lit.suffix(), $suffix);
        assert_roundtrip(expected.to_owned(), input);
    };
}


// ===== Actual tests ============================================================================

#[test]
fn alphanumeric() {
    check!(b'a');
    check!(b'b');
    check!(b'y');
    check!(b'z');
    check!(b'A');
    check!(b'B');
    check!(b'Y');
    check!(b'Z');

    check!(b'0');
    check!(b'1');
    check!(b'8');
    check!(b'9');
}

#[test]
fn special_chars() {
    check!(b' ');
    check!(b'!');
    check!(b'"');
    check!(b'#');
    check!(b'$');
    check!(b'%');
    check!(b'&');
    check!(b'(');
    check!(b')');
    check!(b'*');
    check!(b'+');
    check!(b',');
    check!(b'-');
    check!(b'.');
    check!(b'/');
    check!(b':');
    check!(b';');
    check!(b'<');
    check!(b'=');
    check!(b'>');
    check!(b'?');
    check!(b'@');
    check!(b'[');
    check!(b']');
    check!(b'^');
    check!(b'_');
    check!(b'`');
    check!(b'{');
    check!(b'|');
    check!(b'}');
    check!(b'~');
}

#[test]
fn quote_escapes() {
    check!(b'\'');
    check!(b'\"');
}

#[test]
fn ascii_escapes() {
    check!(b'\n');
    check!(b'\r');
    check!(b'\t');
    check!(b'\\');
    check!(b'\0');

    check!(b'\x00');
    check!(b'\x01');
    check!(b'\x0c');
    check!(b'\x0D');
    check!(b'\x13');
    check!(b'\x30');
    check!(b'\x30');
    check!(b'\x4B');
    check!(b'\x6b');
    check!(b'\x7F');
    check!(b'\x7f');
}

#[test]
fn byte_escapes() {
    check!(b'\x80');
    check!(b'\x8a');
    check!(b'\x8C');
    check!(b'\x99');
    check!(b'\xa0');
    check!(b'\xAd');
    check!(b'\xfe');
    check!(b'\xFe');
    check!(b'\xfF');
    check!(b'\xFF');
}

#[test]
fn suffixes() {
    check!(b'a', r##"b'a'peter"##, "peter");
    check!(b'#', r##"b'#'peter"##, "peter");
    check!(b'\n', r##"b'\n'peter"##, "peter");
    check!(b'\'', r##"b'\''peter"##, "peter");
    check!(b'\"', r##"b'\"'peter"##, "peter");
    check!(b'\xFF', r##"b'\xFF'peter"##, "peter");
}

#[test]
fn invald_escapes() {
    assert_err!(ByteLit, r"b'\a'", UnknownEscape, 2..4);
    assert_err!(ByteLit, r"b'\y'", UnknownEscape, 2..4);
    assert_err!(ByteLit, r"b'\", UnterminatedEscape, 2..3);
    assert_err!(ByteLit, r"b'\x'", UnterminatedEscape, 2..5);
    assert_err!(ByteLit, r"b'\x1'", InvalidXEscape, 2..6);
    assert_err!(ByteLit, r"b'\xaj'", InvalidXEscape, 2..6);
    assert_err!(ByteLit, r"b'\xjb'", InvalidXEscape, 2..6);
}

#[test]
fn unicode_escape_not_allowed() {
    assert_err!(ByteLit, r"b'\u{0}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{00}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{b}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{B}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{7e}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{E4}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{e4}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{fc}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{Fc}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{fC}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{FC}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{b10}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{B10}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{0b10}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{2764}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{1f602}'", UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteLit, r"b'\u{1F602}'", UnicodeEscapeInByteLiteral, 2..4);
}

#[test]
fn parse_err() {
    assert_err!(ByteLit, r"b''", EmptyByteLiteral, None);
    assert_err!(ByteLit, r"b' ''", UnexpectedChar, 4..5);

    assert_err!(ByteLit, r"b'", UnterminatedByteLiteral, None);
    assert_err!(ByteLit, r"b'a", UnterminatedByteLiteral, None);
    assert_err!(ByteLit, r"b'\n", UnterminatedByteLiteral, None);
    assert_err!(ByteLit, r"b'\x35", UnterminatedByteLiteral, None);

    assert_err!(ByteLit, r"b'ab'", OverlongByteLiteral, None);
    assert_err!(ByteLit, r"b'a _'", OverlongByteLiteral, None);
    assert_err!(ByteLit, r"b'\n3'", OverlongByteLiteral, None);

    assert_err!(ByteLit, r"", Empty, None);

    assert_err!(ByteLit, r"b'''", UnescapedSingleQuote, 2);
    assert_err!(ByteLit, r"b''''", UnescapedSingleQuote, 2);

    assert_err!(ByteLit, "b'\n'", UnescapedSpecialWhitespace, 2);
    assert_err!(ByteLit, "b'\t'", UnescapedSpecialWhitespace, 2);
    assert_err!(ByteLit, "b'\r'", UnescapedSpecialWhitespace, 2);

    assert_err!(ByteLit, "b'న'", NonAsciiInByteLiteral, 2);
    assert_err!(ByteLit, "b'犬'", NonAsciiInByteLiteral, 2);
    assert_err!(ByteLit, "b'🦊'", NonAsciiInByteLiteral, 2);
}
