use crate::PropertyKey;

#[test]
fn standard_keys_matches() {
    use crate::property::*;
    macro_rules! contained {
        ($prop:ident) => {
            assert!(
                STANDARD_KEYS.contains(&$prop::key()),
                "STANDARD_KEYS is missing {}",
                $prop::key()
            )
        };
    }
    contained!(IndentStyle);
    contained!(IndentSize);
    contained!(TabWidth);
    contained!(EndOfLine);
    contained!(Charset);
    contained!(TrimTrailingWs);
    contained!(FinalNewline);
    #[cfg(feature = "language-tags")]
    contained!(SpellingLanguage);
    assert!(!STANDARD_KEYS.contains(&MaxLineLen::key())); // Not MaxLineLen
}

#[cfg(feature = "language-tags")]
#[test]
fn spelling_language() {
    use crate::property::SpellingLanguage;
    use crate::rawvalue::RawValue;
    use crate::PropertyValue;
    // This is more testing language-tags than anything,
    // but for language-tags to be useful here,
    let testcase_en = RawValue::from("en");
    let parsed = match SpellingLanguage::parse(&testcase_en) {
        Ok(SpellingLanguage::Value(v)) => v,
        e => {
            let v = e.expect("parsing should succeed");
            panic!("unexpected value {v:?}");
        }
    };
    assert_eq!(parsed.primary_language(), "en");
    let testcase_en_us = RawValue::from("en-US");
    let parsed = match SpellingLanguage::parse(&testcase_en_us) {
        Ok(SpellingLanguage::Value(v)) => v,
        e => {
            let v = e.expect("parsing should succeed");
            panic!("unexpected value {v:?}");
        }
    };
    assert_eq!(parsed.primary_language(), "en");
    assert_eq!(parsed.region(), Some("US"));
}
