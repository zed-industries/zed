use crate::linereader::*;
use crate::ParseError;

fn test_lines(lines: &[(&'static str, Line<'static>)]) {
    for (line, expected) in lines {
        assert_eq!(parse_line(line).unwrap(), *expected)
    }
}

#[test]
fn valid_props() {
    use Line::Pair;
    test_lines(&[
        ("foo=bar", Pair("foo", "bar")),
        ("Foo=Bar", Pair("Foo", "Bar")),
        ("foo = bar", Pair("foo", "bar")),
        ("  foo   =   bar  ", Pair("foo", "bar")),
        ("foo=bar=baz", Pair("foo", "bar=baz")),
        ("  foo =  bar = baz  ", Pair("foo", "bar = baz")),
        ("foo = bar #baz", Pair("foo", "bar #baz")),
        ("foo = [bar]", Pair("foo", "[bar]")),
        #[cfg(feature = "allow-empty-values")]
        ("foo =", Pair("foo", "")),
        #[cfg(feature = "allow-empty-values")]
        ("foo = ", Pair("foo", "")),
    ])
}

#[test]
fn valid_sections() {
    use Line::Section;
    test_lines(&[
        ("[foo]", Section("foo")),
        ("[[foo]]", Section("[foo]")),
        ("[ foo ]", Section(" foo ")),
        ("[][]]", Section("][]")),
        ("[Foo]", Section("Foo")),
        (" [foo] ", Section("foo")),
        ("[a=b]", Section("a=b")),
        ("[#foo]", Section("#foo")),
        ("[foo] #comment", Section("foo")),
        ("[foo] ;comment", Section("foo")),
    ])
}

#[test]
fn valid_nothing() {
    use Line::Nothing;
    test_lines(&[
        ("\t", Nothing),
        ("\r", Nothing),
        ("", Nothing),
        ("   ", Nothing),
        (";comment", Nothing),
        ("#comment", Nothing),
        ("  # comment", Nothing),
        ("# [section]", Nothing),
        ("# foo=bar", Nothing),
    ])
}

#[test]
fn invalid() {
    let lines = [
        "[]",
        "[close",
        "open]",
        "][",
        "nonproperty",
        "=",
        "  = nokey",
        #[cfg(not(feature = "allow-empty-values"))]
        "noval = ",
    ];
    for line in lines {
        assert!(matches!(
            parse_line(line).unwrap_err(),
            ParseError::InvalidLine
        ))
    }
}
