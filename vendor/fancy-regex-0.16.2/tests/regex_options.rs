use fancy_regex::Regex;
use fancy_regex::RegexBuilder;

pub fn build_regex(builder: &RegexBuilder) -> Regex {
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
