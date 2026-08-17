fn validate<'a>(
    text: &str,
    should_be_root: bool,
    expected: impl IntoIterator<Item = &'a [(&'a str, &'a str, usize)]>,
) {
    let path = std::sync::Arc::<std::path::Path>::from(std::path::Path::new(".editorconfig"));
    let mut parser = crate::ConfigParser::new_buffered_with_path(text.as_bytes(), Some(path))
        .expect("Should have created the parser");
    assert_eq!(parser.is_root, should_be_root);
    for section_expected in expected {
        let section = parser.next().unwrap().unwrap();
        let mut iter = section.props().iter();
        #[allow(unused)]
        for (key, value, line_no) in section_expected {
            let (key_test, value_test) = iter.next().expect("Unexpected end of section");
            assert_eq!(key_test, *key, "unexpected key");
            assert_eq!(value_test.into_str(), *value, "unexpected value");
            #[cfg(feature = "track-source")]
            assert_eq!(
                value_test.source().map(|(_, idx)| idx),
                Some(*line_no),
                "unexpected line number"
            )
        }
        assert!(iter.next().is_none());
    }
    assert!(parser.next().is_none());
}

macro_rules! expect {
	[$([$(($key:literal, $value:literal, $line_no:literal)),*]),*] => {
		[$(&[$(($key, $value, $line_no)),*][..]),*]
	}
}

#[test]
fn empty() {
    validate("", false, expect![]);
}

#[test]
fn prelude() {
    validate("root = true\nroot = false", false, expect![]);
    validate("root = true", true, expect![]);
    validate("Root = True", true, expect![]);
    validate("# hello world", false, expect![]);
}

#[test]
fn prelude_unknown() {
    validate("foo = bar", false, expect![]);
    validate("foo = bar\nroot = true", true, expect![]);
}

#[test]
fn sections_empty() {
    validate("[foo]", false, expect![[]]);
    validate("[foo]\n[bar]", false, expect![[], []]);
}

#[test]
fn sections() {
    validate(
        "[foo]\nbk=bv\nak=av",
        false,
        expect![[("bk", "bv", 2), ("ak", "av", 3)]],
    );
    validate(
        "[foo]\nbk=bv\n[bar]\nak=av",
        false,
        expect![[("bk", "bv", 2)], [("ak", "av", 4)]],
    );
    validate(
        "[foo]\nk=a\n[bar]\nk=b",
        false,
        expect![[("k", "a", 2)], [("k", "b", 4)]],
    );
}

#[test]
fn trailing_newline() {
    validate("[foo]\nbar=baz\n", false, expect![[("bar", "baz", 2)]]);
    validate("[foo]\nbar=baz\n\n", false, expect![[("bar", "baz", 2)]]);
}

#[test]
fn section_with_comment_after_it() {
    validate(
        "[/*] # ignore this comment\nk=v",
        false,
        expect![[("k", "v", 2)]],
    );
}

#[test]
fn duplicate_key() {
    validate("[*]\nfoo=bar\nfoo=baz", false, expect![[("foo", "baz", 3)]]);
}
