use crate::{
    test_util::{assert_parse_ok_eq, assert_roundtrip},
    ByteStringLit, Literal,
};

// ===== Utility functions =======================================================================

macro_rules! check {
    ($lit:literal, $has_escapes:expr, $num_hashes:expr) => {
        check!($lit, stringify!($lit), $has_escapes, $num_hashes, "")
    };
    ($lit:literal, $input:expr, $has_escapes:expr, $num_hashes:expr, $suffix:literal) => {
        let input = $input;
        let expected = ByteStringLit {
            raw: input,
            value: if $has_escapes { Some($lit.to_vec()) } else { None },
            num_hashes: $num_hashes,
            start_suffix: input.len() - $suffix.len(),
        };

        assert_parse_ok_eq(
            input,
            ByteStringLit::parse(input),
            expected.clone(),
            "ByteStringLit::parse",
        );
        assert_parse_ok_eq(
            input,
            Literal::parse(input),
            Literal::ByteString(expected.clone()),
            "Literal::parse",
        );
        let lit = ByteStringLit::parse(input).unwrap();
        assert_eq!(lit.value(), $lit);
        assert_eq!(lit.suffix(), $suffix);
        assert_eq!(lit.into_value().as_ref(), $lit);
        assert_roundtrip(expected.into_owned(), input);
    };
}


// ===== Actual tests ============================================================================

#[test]
fn simple() {
    check!(b"", false, None);
    check!(b"a", false, None);
    check!(b"peter", false, None);
}

#[test]
fn special_whitespace() {
    let strings = ["\n", "\t", "foo\tbar", "baz\n"];

    for &s in &strings {
        let input = format!(r#"b"{}""#, s);
        let input_raw = format!(r#"br"{}""#, s);
        for (input, num_hashes) in vec![(input, None), (input_raw, Some(0))] {
            let expected = ByteStringLit {
                raw: &*input,
                value: None,
                num_hashes,
                start_suffix: input.len(),
            };
            assert_parse_ok_eq(
                &input,
                ByteStringLit::parse(&*input),
                expected.clone(),
                "ByteStringLit::parse",
            );
            assert_parse_ok_eq(
                &input,
                Literal::parse(&*input),
                Literal::ByteString(expected),
                "Literal::parse",
            );
            assert_eq!(ByteStringLit::parse(&*input).unwrap().value(), s.as_bytes());
            assert_eq!(ByteStringLit::parse(&*input).unwrap().into_value(), s.as_bytes());
        }
    }
}

#[test]
fn simple_escapes() {
    check!(b"a\nb", true, None);
    check!(b"\nb", true, None);
    check!(b"a\n", true, None);
    check!(b"\n", true, None);

    check!(b"\x60foo \t bar\rbaz\n banana \0kiwi", true, None);
    check!(b"foo \\ferris", true, None);
    check!(b"baz \\ferris\"box", true, None);
    check!(b"\\foo\\ banana\" baz\"", true, None);
    check!(b"\"foo \\ferris \" baz\\", true, None);

    check!(b"\x00", true, None);
    check!(b" \x01", true, None);
    check!(b"\x0c foo", true, None);
    check!(b" foo\x0D ", true, None);
    check!(b"\\x13", true, None);
    check!(b"\"x30", true, None);
}

#[test]
fn non_ascii_escapes() {
    check!(b"\x80", true, None);
    check!(b"\x8a", true, None);
    check!(b"\x8C", true, None);
    check!(b"\x99", true, None);
    check!(b"\xa0", true, None);
    check!(b"\xAd", true, None);
    check!(b"\xfe", true, None);
    check!(b"\xFe", true, None);
    check!(b"\xfF", true, None);
    check!(b"\xFF", true, None);
}

#[test]
fn string_continue() {
    check!(b"foo\
        bar", true, None);
    check!(b"foo\
bar", true, None);

    check!(b"foo\

        banana", true, None);

    // Weird whitespace characters
    let lit = ByteStringLit::parse("b\"foo\\\n\t\n \n\tbar\"").expect("failed to parse");
    assert_eq!(lit.value(), b"foobar");

    // Raw strings do not handle "string continues"
    check!(br"foo\
        bar", false, Some(0));
}

#[test]
fn raw_byte_string() {
    check!(br"", false, Some(0));
    check!(br"a", false, Some(0));
    check!(br"peter", false, Some(0));
    check!(br"Greetings jason!", false, Some(0));

    check!(br#""#, false, Some(1));
    check!(br#"a"#, false, Some(1));
    check!(br##"peter"##, false, Some(2));
    check!(br###"Greetings # Jason!"###, false, Some(3));
    check!(br########"we ## need #### more ####### hashtags"########, false, Some(8));

    check!(br#"foo " bar"#, false, Some(1));
    check!(br##"foo " bar"##, false, Some(2));
    check!(br#"foo """" '"'" bar"#, false, Some(1));
    check!(br#""foo""#, false, Some(1));
    check!(br###""foo'"###, false, Some(3));
    check!(br#""x'#_#s'"#, false, Some(1));
    check!(br"#", false, Some(0));
    check!(br"foo#", false, Some(0));
    check!(br"##bar", false, Some(0));
    check!(br###""##foo"##bar'"###, false, Some(3));

    check!(br"foo\n\t\r\0\\x60\u{123}doggo", false, Some(0));
    check!(br#"cat\n\t\r\0\\x60\u{123}doggo"#, false, Some(1));
}

#[test]
fn suffixes() {
    check!(b"hello", r###"b"hello"suffix"###, false, None, "suffix");
    check!(b"fox", r#"b"fox"peter"#, false, None, "peter");
    check!(b"a\x0cb\\", r#"b"a\x0cb\\"_jürgen"#, true, None, "_jürgen");
    check!(br"a\x0cb\\", r###"br#"a\x0cb\\"#_jürgen"###, false, Some(1), "_jürgen");
}

#[test]
fn parse_err() {
    assert_err!(ByteStringLit, r#"b""#, UnterminatedString, None);
    assert_err!(ByteStringLit, r#"b"cat"#, UnterminatedString, None);
    assert_err!(ByteStringLit, r#"b"Jurgen"#, UnterminatedString, None);
    assert_err!(ByteStringLit, r#"b"foo bar baz"#, UnterminatedString, None);

    assert_err!(ByteStringLit, r#"b"fox"peter""#, InvalidSuffix, 6);
    assert_err!(ByteStringLit, r###"br#"foo "# bar"#"###, UnexpectedChar, 10);

    assert_err!(ByteStringLit, "b\"\r\"", CarriageReturn, 2);
    assert_err!(ByteStringLit, "b\"fo\rx\"", CarriageReturn, 4);
    assert_err!(ByteStringLit, "br\"\r\"", CarriageReturn, 3);
    assert_err!(ByteStringLit, "br\"fo\rx\"", CarriageReturn, 5);
    assert_err!(ByteStringLit, "b\"a\\\r\"", UnknownEscape, 3..5);
    assert_err!(ByteStringLit, "br\"a\\\r\"", CarriageReturn, 5);

    assert_err!(ByteStringLit, r##"br####""##, UnterminatedRawString, None);
    assert_err!(ByteStringLit, r#####"br##"foo"#bar"#####, UnterminatedRawString, None);
    assert_err!(ByteStringLit, r##"br####"##, InvalidLiteral, None);
    assert_err!(ByteStringLit, r##"br####x"##, InvalidLiteral, None);
}

#[test]
fn non_ascii() {
    assert_err!(ByteStringLit, r#"b"న""#, NonAsciiInByteLiteral, 2);
    assert_err!(ByteStringLit, r#"b"foo犬""#, NonAsciiInByteLiteral, 5);
    assert_err!(ByteStringLit, r#"b"x🦊baz""#, NonAsciiInByteLiteral, 3);
    assert_err!(ByteStringLit, r#"br"న""#, NonAsciiInByteLiteral, 3);
    assert_err!(ByteStringLit, r#"br"foo犬""#, NonAsciiInByteLiteral, 6);
    assert_err!(ByteStringLit, r#"br"x🦊baz""#, NonAsciiInByteLiteral, 4);
}

#[test]
fn invalid_escapes() {
    assert_err!(ByteStringLit, r#"b"\a""#, UnknownEscape, 2..4);
    assert_err!(ByteStringLit, r#"b"foo\y""#, UnknownEscape, 5..7);
    assert_err!(ByteStringLit, r#"b"\"#, UnterminatedEscape, 2);
    assert_err!(ByteStringLit, r#"b"\x""#, UnterminatedEscape, 2..4);
    assert_err!(ByteStringLit, r#"b"foo\x1""#, UnterminatedEscape, 5..8);
    assert_err!(ByteStringLit, r#"b" \xaj""#, InvalidXEscape, 3..7);
    assert_err!(ByteStringLit, r#"b"\xjbbaz""#, InvalidXEscape, 2..6);
}

#[test]
fn unicode_escape_not_allowed() {
    assert_err!(ByteStringLit, r#"b"\u{0}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{00}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{b}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{B}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{7e}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{E4}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{e4}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{fc}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{Fc}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{fC}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{FC}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{b10}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{B10}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{0b10}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{2764}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{1f602}""#, UnicodeEscapeInByteLiteral, 2..4);
    assert_err!(ByteStringLit, r#"b"\u{1F602}""#, UnicodeEscapeInByteLiteral, 2..4);
}
