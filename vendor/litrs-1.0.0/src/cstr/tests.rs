use std::ffi::CString;

use crate::{
    test_util::{assert_parse_ok_eq, assert_roundtrip},
    CStringLit, Literal,
};

// ===== Utility functions =======================================================================

macro_rules! check {
    ($lit:literal, $has_escapes:expr, $num_hashes:expr) => {
        check!($lit, stringify!($lit), $has_escapes, $num_hashes, "")
    };
    ($lit:literal, $input:expr, $has_escapes:expr, $num_hashes:expr, $suffix:literal) => {
        let input = $input;
        let expected = CStringLit {
            raw: input,
            value: $lit.to_owned(),
            num_hashes: $num_hashes,
            start_suffix: input.len() - $suffix.len(),
        };

        assert_parse_ok_eq(
            input, CStringLit::parse(input), expected.clone(), "CStringLit::parse");
        assert_parse_ok_eq(
            input, Literal::parse(input), Literal::CString(expected.clone()), "Literal::parse");
        let lit = CStringLit::parse(input).unwrap();
        assert_eq!(lit.value(), $lit);
        assert_eq!(lit.suffix(), $suffix);
        assert_eq!(lit.into_value().as_ref(), $lit);
        assert_roundtrip(expected.into_owned(), input);
    };
}


// ===== Actual tests ============================================================================

#[test]
fn simple() {
    check!(c"", false, None);
    check!(c"a", false, None);
    check!(c"peter", false, None);
    check!(c"Sei gegrüßt, Bärthelt!", false, None);
    check!(c"أنا لا أتحدث العربية", false, None);
    check!(c"お前はもう死んでいる", false, None);
    check!(c"Пушки - интересные музыкальные инструменты", false, None);
    check!(c"lit 👌 😂 af", false, None);
}

#[test]
fn special_whitespace() {
    let strings = ["\n", "\t", "foo\tbar", "baz\n"];

    for &s in &strings {
        let input = format!(r#"c"{}""#, s);
        let input_raw = format!(r#"cr"{}""#, s);
        let value = CString::new(s).unwrap();
        for (input, num_hashes) in vec![(input, None), (input_raw, Some(0))] {
            let expected = CStringLit {
                raw: &*input,
                value: value.clone(),
                num_hashes,
                start_suffix: input.len(),
            };
            assert_parse_ok_eq(
                &input, CStringLit::parse(&*input), expected.clone(), "CStringLit::parse");
            assert_parse_ok_eq(
                &input, Literal::parse(&*input), Literal::CString(expected), "Literal::parse");
            assert_eq!(CStringLit::parse(&*input).unwrap().value(), value.as_c_str());
            assert_eq!(CStringLit::parse(&*input).unwrap().into_value(), value);
        }
    }
}

#[test]
fn simple_escapes() {
    check!(c"a\nb", true, None);
    check!(c"\nb", true, None);
    check!(c"a\n", true, None);
    check!(c"\n", true, None);

    check!(c"\x60foo \t bar\rbaz\n banana", true, None);
    check!(c"foo \\ferris", true, None);
    check!(c"baz \\ferris\"box", true, None);
    check!(c"\\foo\\ banana\" baz\"", true, None);
    check!(c"\"foo \\ferris \" baz\\", true, None);

    check!(c" \x01", true, None);
    check!(c"\x0c foo", true, None);
    check!(c" foo\x0D ", true, None);
    check!(c"\\x13", true, None);
    check!(c"\"x30", true, None);
}

#[test]
fn unicode_escapes() {    check!(c"\u{b} ", true, None);
    check!(c" \u{B} ", true, None);
    check!(c"\u{7e}", true, None);
    check!(c"నక్క\u{E4}", true, None);
    check!(c"\u{e4} నక్క", true, None);
    check!(c" \u{fc}నక్క ", true, None);
    check!(c"\u{Fc}", true, None);
    check!(c"\u{fC}🦊\nлиса", true, None);
    check!(c"лиса\u{FC}", true, None);
    check!(c"лиса\u{b10}నక్క🦊", true, None);
    check!(c"\"నక్క\u{B10}", true, None);
    check!(c"лиса\\\u{0b10}", true, None);
    check!(c"ли🦊са\\\"\u{0b10}", true, None);
    check!(c"నక్క\\\\u{0b10}", true, None);
    check!(c"\u{2764}Füchsin", true, None);
    check!(c"Füchse \u{1f602}", true, None);
    check!(c"cd\u{1F602}ab", true, None);

    check!(c"\\🦊\u{3_b}", true, None);
    check!(c"🦊\u{1_F_6_0_2}Füchsin", true, None);
    check!(c"నక్క\\\u{1_F6_02_____}నక్క", true, None);
}

#[test]
fn string_continue() {
    check!(c"foo\
        bar", true, None);
    check!(c"foo\
bar", true, None);

    check!(c"foo\

        banana", true, None);

    // Weird whitespace characters
    let lit = CStringLit::parse("c\"foo\\\n\t\n \n\tbar\"").expect("failed to parse");
    assert_eq!(lit.value(), c"foobar");

    // Raw strings do not handle "string continues"
    check!(cr"foo\
        bar", false, Some(0));
}

#[test]
fn raw_c_string() {
    check!(cr"", false, Some(0));
    check!(cr"a", false, Some(0));
    check!(cr"peter", false, Some(0));
    check!(cr"Greetings jason!", false, Some(0));

    check!(cr#""#, false, Some(1));
    check!(cr#"a"#, false, Some(1));
    check!(cr##"peter"##, false, Some(2));
    check!(cr###"Greetings # Jason!"###, false, Some(3));
    check!(cr########"we ## need #### more ####### hashtags"########, false, Some(8));

    check!(cr#"foo " bar"#, false, Some(1));
    check!(cr##"foo " bar"##, false, Some(2));
    check!(cr#"foo """" '"'" bar"#, false, Some(1));
    check!(cr#""foo""#, false, Some(1));
    check!(cr###""foo'"###, false, Some(3));
    check!(cr#""x'#_#s'"#, false, Some(1));
    check!(cr"#", false, Some(0));
    check!(cr"foo#", false, Some(0));
    check!(cr"##bar", false, Some(0));
    check!(cr###""##foo"##bar'"###, false, Some(3));

    check!(cr"foo\n\t\r\0\\x60\u{123}doggo", false, Some(0));
    check!(cr#"cat\n\t\r\0\\x60\u{123}doggo"#, false, Some(1));
}

#[test]
fn suffixes() {
    check!(c"hello", r###"c"hello"suffix"###, false, None, "suffix");
    check!(c"fox", r#"c"fox"peter"#, false, None, "peter");
    check!(c"a\x0cb\\", r#"c"a\x0cb\\"_jürgen"#, true, None, "_jürgen");
    check!(cr"a\x0cb\\", r###"cr#"a\x0cb\\"#_jürgen"###, false, Some(1), "_jürgen");
}

#[test]
fn parse_err() {
    assert_err!(CStringLit, r#"c""#, UnterminatedString, None);
    assert_err!(CStringLit, r#"c"cat"#, UnterminatedString, None);
    assert_err!(CStringLit, r#"c"Jurgen"#, UnterminatedString, None);
    assert_err!(CStringLit, r#"c"foo bar baz"#, UnterminatedString, None);

    assert_err!(CStringLit, r#"c"fox"peter""#, InvalidSuffix, 6);
    assert_err!(CStringLit, r###"cr#"foo "# bar"#"###, UnexpectedChar, 10);

    assert_err!(CStringLit, "c\"\r\"", CarriageReturn, 2);
    assert_err!(CStringLit, "c\"fo\rx\"", CarriageReturn, 4);
    assert_err!(CStringLit, "cr\"\r\"", CarriageReturn, 3);
    assert_err!(CStringLit, "cr\"fo\rx\"", CarriageReturn, 5);
    assert_err!(CStringLit, "c\"a\\\r\"", UnknownEscape, 3..5);
    assert_err!(CStringLit, "cr\"a\\\r\"", CarriageReturn, 5);

    assert_err!(CStringLit, r##"cr####""##, UnterminatedRawString, None);
    assert_err!(CStringLit, r#####"cr##"foo"#bar"#####, UnterminatedRawString, None);
    assert_err!(CStringLit, r##"cr####"##, InvalidLiteral, None);
    assert_err!(CStringLit, r##"cr####x"##, InvalidLiteral, None);
}

#[test]
fn null_byte() {
    assert_err!(CStringLit, r#"c"\x00""#, DisallowedNulEscape, 2..6);
    assert_err!(CStringLit, r#"c"\u{0}""#, DisallowedNulEscape, 2..7);
    assert_err!(CStringLit, r#"c"\u{00}""#, DisallowedNulEscape, 2..8);
    assert_err!(CStringLit, r#"c"\u{000}""#, DisallowedNulEscape, 2..9);
    assert_err!(CStringLit, r#"c"\u{0000}""#, DisallowedNulEscape, 2..10);
    assert_err!(CStringLit, r#"c"\u{00000}""#, DisallowedNulEscape, 2..11);
    assert_err!(CStringLit, r#"c"\u{000000}""#, DisallowedNulEscape, 2..12);
    assert_err!(CStringLit, r#"c" \u{00}""#, DisallowedNulEscape, 3..9);
    assert_err!(CStringLit, r#"c"\u{0}🦊""#, DisallowedNulEscape, 2..7);
    assert_err!(CStringLit, r#"c"лиса\u{0__}""#, DisallowedNulEscape, 10..17);

}

#[test]
fn invalid_escapes() {
    assert_err!(CStringLit, r#"c"\a""#, UnknownEscape, 2..4);
    assert_err!(CStringLit, r#"c"foo\y""#, UnknownEscape, 5..7);
    assert_err!(CStringLit, r#"c"\"#, UnterminatedEscape, 2);
    assert_err!(CStringLit, r#"c"\x""#, UnterminatedEscape, 2..4);
    assert_err!(CStringLit, r#"c"foo\x1""#, UnterminatedEscape, 5..8);
    assert_err!(CStringLit, r#"c" \xaj""#, InvalidXEscape, 3..7);
    assert_err!(CStringLit, r#"c"\xjbbaz""#, InvalidXEscape, 2..6);
}
