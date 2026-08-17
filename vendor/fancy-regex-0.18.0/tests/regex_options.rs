use fancy_regex::{CompileError, Error, Regex, RegexBuilder};

fn build_regex(builder: &RegexBuilder) -> Regex {
    let result = builder.build();
    assert!(
        result.is_ok(),
        "Expected regex to build successfully, got {:?}",
        result.err()
    );
    result.unwrap()
}

#[test]
fn check_casing_option() {
    let regex = build_regex(RegexBuilder::new(r"TEST foo").case_insensitive(false));

    assert!(regex.is_match(r"TEST foo").unwrap_or_default());
    assert!(!regex.is_match(r"test foo").unwrap_or_default());
}

#[test]
fn check_override_casing_option() {
    let regex = build_regex(RegexBuilder::new(r"FOO(?i:bar)quux").case_insensitive(false));

    assert!(!regex.is_match("FoObarQuUx").unwrap_or_default());
    assert!(!regex.is_match("fooBARquux").unwrap_or_default());
    assert!(regex.is_match("FOObarquux").unwrap_or_default());
}

#[test]
fn check_casing_insensitive_option() {
    let regex = build_regex(RegexBuilder::new(r"TEST FOO").case_insensitive(true));

    assert!(regex.is_match(r"test foo").unwrap_or_default());
}

#[test]
fn check_multi_line_option() {
    let test_text = r"test
hugo
test";

    let regex = build_regex(RegexBuilder::new(r"^test$").multi_line(true));
    assert!(regex.is_match(test_text).unwrap_or_default());
}

#[test]
fn check_ignore_whitespace_option() {
    let regex = build_regex(RegexBuilder::new(r"test    foo").ignore_whitespace(true));

    let test_text = r"testfoo";
    assert!(regex.is_match(test_text).unwrap_or_default());
}

#[test]
fn check_dot_matches_new_line_option() {
    let regex = build_regex(RegexBuilder::new(r"<div>(.*?)<\/div>").dot_matches_new_line(true));

    let test_text = r"<div>
    hello</div>";

    assert!(regex.is_match(test_text).unwrap_or_default());
}

#[test]
fn check_casing_insensitive_option_hard() {
    let regex = build_regex(RegexBuilder::new(r"[a-z](?<=[^f])").case_insensitive(true));

    assert!(regex.is_match(r"J").unwrap_or_default());
    assert!(!regex.is_match(r"F").unwrap_or_default());
    assert!(regex.is_match(r"j").unwrap_or_default());
}

#[test]
fn check_ignore_whitespace_option_fancy() {
    let regex = build_regex(RegexBuilder::new(r"(?=test    foo)").ignore_whitespace(true));

    let test_text = r"testfoo";

    assert!(regex.is_match(test_text).unwrap_or_default());
}

#[test]
fn check_ignore_whitespace_with_lookahead_matches() {
    let regex = build_regex(RegexBuilder::new(r"(?=test    foo)").ignore_whitespace(true));

    let test_text = r"test    foo";

    assert!(!regex.is_match(test_text).unwrap_or_default());
}

#[test]
fn check_verbose_mode_option() {
    let pattern = "
test  foo #hugo
";
    let regex = build_regex(RegexBuilder::new(pattern).verbose_mode(true));

    let test_text = r"test    foo";

    assert!(!regex.is_match(test_text).unwrap_or_default());
}

#[test]
fn issue_163_fancy_email_test() {
    let pattern =
        r"^(?!\.)(?!.*\.\.)([a-z0-9_'+\-\.]*)[a-z0-9_'+\-]@([a-z0-9][a-z0-9\-]*\.)+[a-z]{2,}$";

    let regex = build_regex(RegexBuilder::new(pattern).case_insensitive(true));

    let test_text = "VALID@domain.com";
    assert!(regex.is_match(test_text).unwrap());
}

#[test]
fn check_oniguruma_mode_changes_wordbounds() {
    fn find_all_matches(regex: &Regex, text: &'static str) -> Vec<&'static str> {
        regex.find_iter(text).map(|m| m.unwrap().as_str()).collect()
    }

    let pattern = r"\<prefix_\w*\>";
    let test_text = "not_prefix_oops prefix_with_suffix <prefix_>";

    // By default the pattern is interpretted as a left word-bound followed by the literal "prefix_"
    // followed by any number of word characters ending on a right word-bound
    let default_mode = Regex::new(pattern).unwrap();
    let default_matches = find_all_matches(&default_mode, test_text);
    assert_eq!(default_matches, ["prefix_with_suffix", "prefix_"]);

    // When Oniguruma-mode is used the pattern is instead a literal "<prefix_" followed by any
    // number of word characters followed by a literal ">"
    let oniguruma_mode = build_regex(RegexBuilder::new(pattern).oniguruma_mode(true));
    let oniguruma_matches = find_all_matches(&oniguruma_mode, test_text);
    assert_eq!(oniguruma_matches, ["<prefix_>"]);
}

#[test]
fn empty_noncapturing_group_with_quantifier_oniguruma_mode() {
    // Oniguruma silently accepts quantifiers on empty non-capturing groups.
    // These patterns should compile and match any input (they are zero-width).
    let patterns = ["(?:)*", "(?:)+", "(?:)?", "(?:){3}", "(?:)*?", "(?:)++"];
    for pat in patterns {
        let regex = build_regex(RegexBuilder::new(pat).oniguruma_mode(true));
        assert!(
            regex.is_match("anything").unwrap(),
            "Expected '{}' to match",
            pat
        );
        assert!(
            regex.is_match("").unwrap(),
            "Expected '{}' to match empty",
            pat
        );
    }
}

#[test]
fn empty_noncapturing_group_with_quantifier_default_mode_errors() {
    // In default mode, quantifiers on empty non-capturing groups should fail.
    let result = RegexBuilder::new("(?:)*").build();
    assert!(result.is_err(), "Expected (?:)* to fail in default mode");
}

#[test]
fn markdown_fenced_code_block_empty_group_oniguruma_mode() {
    // Repro of a real-world pattern that syntect produces for markdown
    // fenced-code-block closing, where a backreference substitution
    // expands to the empty string.
    let pattern =
        "^(?x:\n  [ \\t]*\n  (\n    ```\n    (?:(?:`)*|(?:)*)\n  )\n  (\\s*(?m:$)\\n?)\n)";
    let regex = build_regex(RegexBuilder::new(pattern).oniguruma_mode(true));
    assert!(regex.is_match("   ```\n").unwrap());
}

#[test]
fn check_crlf_option() {
    // With CRLF mode and multi-line enabled, ^ and $ should treat \r\n as a single line ending
    let regex = build_regex(RegexBuilder::new(r"^test$").multi_line(true).crlf(true));
    assert!(regex.is_match("test").unwrap_or_default());
    assert!(regex.is_match("\r\ntest\r\n").unwrap_or_default());
    assert!(regex.is_match("test\r\n").unwrap_or_default());
    assert!(regex.is_match("\r\ntest").unwrap_or_default());
}

#[test]
fn check_crlf_flag_in_pattern() {
    // The (?mR) flag combination in the pattern itself should enable CRLF mode
    let regex = build_regex(&RegexBuilder::new(r"(?mR)^test$"));
    assert!(regex.is_match("test").unwrap_or_default());
    assert!(regex.is_match("\r\ntest\r\n").unwrap_or_default());
    assert!(regex.is_match("test\r\n").unwrap_or_default());
    assert!(regex.is_match("\r\ntest").unwrap_or_default());
}

#[test]
fn can_build_multiple_regexes_with_same_options() {
    let mut builder = RegexBuilder::new(r"^[a-z@.]+$");
    builder.case_insensitive(true);
    let regex1 = build_regex(&builder);

    let test_text = "VALID@domain.com";
    assert!(regex1.is_match(test_text).unwrap());

    let regex2 = build_regex(&builder.pattern(r"AB.$".to_string()));

    let test_text = "abc";
    assert!(regex2.is_match(test_text).unwrap());
}

#[test]
fn changing_builder_options_has_no_effect_on_already_built_regexes() {
    let pattern1 = r"^[a-z@.]+$";
    let pattern2 = r"AB.$";

    let mut builder = RegexBuilder::new(pattern1);
    builder.case_insensitive(true);
    let regex1 = build_regex(&builder);
    let regex2 = build_regex(&builder.case_insensitive(false));
    let regex3 = build_regex(&builder.pattern(pattern2.to_string()));
    let regex4 = build_regex(&builder.case_insensitive(true));

    let test_text1 = "VALID@domain.com";
    let test_text2 = "abc";

    assert!(regex1.is_match(test_text1).unwrap());
    assert!(!regex2.is_match(test_text1).unwrap());
    assert!(!regex3.is_match(test_text2).unwrap());
    assert!(regex4.is_match(test_text2).unwrap());
}

#[test]
fn check_find_not_empty_option() {
    // \d* can match empty, so without find_not_empty it matches at position 0 with zero length.
    // With find_not_empty enabled, that zero-length match is rejected and find returns None.
    let pattern = r"\d*";
    let text = "hello";

    let normal = build_regex(RegexBuilder::new(pattern).find_not_empty(false));
    assert_eq!(
        normal.find(text).unwrap().map(|m| (m.start(), m.end())),
        Some((0, 0))
    );

    let not_empty = build_regex(RegexBuilder::new(pattern).find_not_empty(true));
    assert_eq!(
        not_empty.find(text).unwrap().map(|m| (m.start(), m.end())),
        None
    );
}

#[test]
fn check_find_not_empty_rejects_zero_width_only_patterns() {
    // Patterns that can only ever produce zero-length matches should fail to compile
    // when find_not_empty is set, since they can never return a result.
    let zero_width_patterns = vec![
        r"^", r"$", r"\b", r"(?!bar)", r"(?<=x)", r"(?<!y)", r"(?=.)", r"\G",
    ];

    for pattern in zero_width_patterns {
        let result = RegexBuilder::new(pattern).find_not_empty(true).build();
        assert!(
            matches!(
                result,
                Err(Error::CompileError(ref e)) if matches!(**e, CompileError::PatternCanNeverMatch)
            ),
            "Expected PatternCanNeverMatch for pattern {:?}, got {:?}",
            pattern,
            result,
        );
    }
}

#[test]
fn check_find_not_empty_allows_patterns_that_can_match_non_empty() {
    // Patterns that can produce non-empty matches should compile fine even with find_not_empty
    let patterns = vec![
        r"\d*",
        r"a?",
        r"\d+",
        r"[a-z]+",
        r"(?:foo)?",
        r"\d*(?=x)",
        r"a(?=b)",
    ];

    for pattern in patterns {
        let result = RegexBuilder::new(pattern).find_not_empty(true).build();
        assert!(
            result.is_ok(),
            "Expected pattern {:?} to compile with find_not_empty, got {:?}",
            pattern,
            result.err(),
        );
    }
}

#[test]
fn check_find_not_empty_is_match() {
    // is_match should respect find_not_empty: \d* normally matches empty at position 0
    // in "hello", but with find_not_empty it should return false.
    let pattern = r"\d*";
    let text = "hello";

    let normal = build_regex(RegexBuilder::new(pattern).find_not_empty(false));
    assert!(normal.is_match(text).unwrap());

    let not_empty = build_regex(RegexBuilder::new(pattern).find_not_empty(true));
    assert!(!not_empty.is_match(text).unwrap());
}

#[test]
fn check_find_not_empty_allows_trailing_lookahead_with_content() {
    // a?(?=b) optimizes to (a?)b — group 0 is "a?" which is not always empty, so it should compile
    let result = RegexBuilder::new(r"a(?=b)").find_not_empty(true).build();
    assert!(
        result.is_ok(),
        "Expected a?(?=b) to compile with find_not_empty"
    );

    let regex = result.unwrap();
    assert!(regex.is_match("ab").unwrap());
    assert!(!regex.is_match("ac").unwrap());
}
