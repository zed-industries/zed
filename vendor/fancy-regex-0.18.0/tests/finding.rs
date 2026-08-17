mod common;

use fancy_regex::{Match, RegexBuilder};
use matches::assert_matches;
use std::ops::Range;

#[test]
fn match_api() {
    let m = find_match(r"(\w+)", "... test").unwrap();
    assert_eq!(m.range(), (4..8));
    assert_eq!(Range::from(m), (4..8));
    assert_eq!(m.as_str(), "test");
}

#[test]
fn find_wrap() {
    assert_eq!(find(r"(\w+)", "... test"), Some((4, 8)));
    assert_eq!(find(r"(?m)^yes$", "foo\nyes\n"), Some((4, 7)));
}

#[test]
fn find_fancy_case_insensitive() {
    assert_eq!(find(r"(x|xy)\1", "XX"), None);
    assert_eq!(find(r"(x|xy)\1", "xx"), Some((0, 2)));
    assert_eq!(find(r"((?i:x|xy))\1", "XX"), Some((0, 2)));
}

#[test]
fn lookahead_grouping_single_expression() {
    // These would fail if the delegate expression was `^x|a` (if we didn't
    // group as `^(?:x|a)`).
    assert_eq!(find(r"(?=x|a)", "a"), Some((0, 0)));
    assert_eq!(find(r"(?=x|a)", "bbba"), Some((3, 3)));
}

#[test]
fn lookahead_grouping_multiple_expressions() {
    // These would fail if the delegate expression was `^ab|Bc` (if we didn't
    // preserve grouping of `(?:b|B)`).
    assert_eq!(find(r"(?=(?!x)a(?:b|B)c)", "aBc"), Some((0, 0)));
    assert_eq!(find(r"(?=(?!x)a(?:b|B)c)", "Bc"), None);
}

#[test]
fn lookbehind_grouping_single_expression() {
    assert_eq!(find(r"(?<=x|a)", "a"), Some((1, 1)));
    assert_eq!(find(r"(?<=x|a)", "ba"), Some((2, 2)));
    assert_eq!(find(r"(?<=^a)", "a"), Some((1, 1)));
    assert_eq!(find(r"(?<=^a)", "ba"), None);
}

#[test]
fn lookbehind_variable_sized_alt() {
    // These get compiled into multiple alternative lookbehinds
    assert_eq!(find(r"(?<=a|bc)", "xxa"), Some((3, 3)));
    assert_eq!(find(r"(?<=a|bc)", "xxbc"), Some((4, 4)));
    assert_eq!(find(r"(?<=a|bc)", "xx"), None);
    assert_eq!(find(r"(?<=a|bc)", "xxb"), None);
    assert_eq!(find(r"(?<=a|bc)", "xxc"), None);
}

#[test]
fn negative_lookbehind_variable_sized_alt() {
    assert_eq!(find(r"(?<!a|bc)x", "axx"), Some((2, 3)));
    assert_eq!(find(r"(?<!a|bc)x", "bcxx"), Some((3, 4)));
    assert_eq!(find(r"(?<!a|bc)x", "x"), Some((0, 1)));
    assert_eq!(find(r"(?<!a|bc)x", "bx"), Some((1, 2)));
    assert_eq!(find(r"(?<!a|bc)x", "cx"), Some((1, 2)));
    assert_eq!(find(r"(?<!a|bc)x", "ax"), None);
    assert_eq!(find(r"(?<!a|bc)x", "bcx"), None);
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn lookbehind_positive_variable_sized_functionality_easy() {
    assert_eq!(find(r"(?<=a(?:b|cd))x", "abx"), Some((2, 3)));
    assert_eq!(find(r"(?<=a(?:b|cd))x", "acdx"), Some((3, 4)));
    assert_eq!(find(r"(?<=a(?:b|cd))x", "ax"), None);
    assert_eq!(find(r"(?<=a(?:b|cd))x", "bcx"), None);

    assert_eq!(find(r"(?<=a+b+)x", "abx"), Some((2, 3)));
    assert_eq!(find(r"(?<=a+b+)x", "aabbx"), Some((4, 5)));
    assert_eq!(find(r"(?<=a+b+)x", "aaabbbx"), Some((6, 7)));
    assert_eq!(find(r"(?<=a+b+)x", "ax"), None);
    assert_eq!(find(r"(?<=a+b+)x", "bx"), None);
    assert_eq!(find(r"(?<=a+b+)x", "abcx"), None);
    assert_eq!(find(r"(?<=a+b+)c", "bc aabbc"), Some((7, 8)));

    assert_eq!(find(r"\b(?<=\|\s{0,9})(?:[gG]pu)\b", "|gpu"), Some((1, 4)));
    assert_eq!(
        find(r"\b(?<=\|\s{0,9})(?:[gG]pu)\b", "|  gpu"),
        Some((3, 6))
    );
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn lookbehind_negative_variable_sized_functionality_easy() {
    assert_eq!(find(r"(?<!a(?:b|cd))x", "abx"), None);
    assert_eq!(find(r"(?<!a(?:b|cd))x", "acdx"), None);
    assert_eq!(find(r"(?<!a(?:b|cd))x", "ax"), Some((1, 2)));
    assert_eq!(find(r"(?<!a(?:b|cd))x", "bcx"), Some((2, 3)));

    assert_eq!(find(r"(?<!a+b+)x", "abx"), None);
    assert_eq!(find(r"(?<!a+b+)x", "aabbx"), None);
    assert_eq!(find(r"(?<!a+b+)x", "aaabbbx"), None);
    assert_eq!(find(r"(?<!a+b+)x", "ax"), Some((1, 2)));
    assert_eq!(find(r"(?<!a+b+)x", "bx"), Some((1, 2)));
    assert_eq!(find(r"(?<!a+b+)x", "abcx"), Some((3, 4)));
    assert_eq!(find(r"(?<!a+b+)c", "aabbc bc"), Some((7, 8)));

    assert_eq!(find(r"\b(?<!\|\s{0,9})(?:[gG]pu)\b", "|gpu"), None);
    assert_eq!(find(r"\b(?<!\|\s{0,9})(?:[gG]pu)\b", "|  gpu"), None);
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn lookbehind_with_word_boundary_and_variable_length() {
    // Test (?<=\ba+)b - word boundary followed by variable-length a's
    assert_eq!(find(r"(?<=\ba+)b", "aaab"), Some((3, 4)));
    assert_eq!(find(r"(?<=\ba+)b", " aaab"), Some((4, 5)));
    assert_eq!(find(r"(?<=\ba+)b", "ab"), Some((1, 2)));
    assert_eq!(find(r"(?<=\ba+)b", "xaaab"), None); // No word boundary
    assert_eq!(find(r"(?<=\ba+)b", "b"), None); // No 'a's before 'b'

    // Test (?<=\Ba+)b - NOT word boundary followed by variable-length a's
    assert_eq!(find(r"(?<=\Ba+)b", "a_aab"), Some((4, 5)));

    // Test negative lookbehind with word boundary
    assert_eq!(find(r"(?<!\ba+)b", "xaaab"), Some((4, 5))); // 'a's not preceded by word boundary
    assert_eq!(find(r"(?<!\ba+)b", "aaab"), None); // Word boundary before 'a's
    assert_eq!(find(r"(?<!\ba+)b", " aaab"), None); // Word boundary after space
    assert_eq!(find(r"(?<!\ba+)b", "b"), Some((0, 1))); // lookbehind doesn't match

    assert_eq!(
        find(r"(?=fuly)(?<=\b(?:[A-Z][a-z]*|[a-z]+))fuly\b", "Carefuly"),
        Some((4, 8))
    );

    assert_eq!(find(r"(.)b+(?<=\1\1b+)x", "aabbx"), Some((1, 5)));
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn lookbehind_positive_variable_sized_functionality_unicode() {
    assert_eq!(find(r"(?<=\b\w+\b)", "ežeras"), Some((7, 7)));
    assert_eq!(find(r"(?<=\b\w+[ ]\d)", "ežeras 123"), Some((9, 9)));
}

#[test]
fn lookbehind_containing_const_size_backref() {
    assert_eq!(find(r"(..)(?<=\1\1)", "yyxxxx"), Some((4, 6)));
}

#[test]
fn lookahead_looks_left() {
    assert_eq!(find(r"a(?=\b)", "ab"), None);
    assert_eq!(find(r"a(?=\b)", "a."), Some((0, 1)));
    assert_eq!(find(r"a(?=\b|_)", "a."), Some((0, 1)));
    assert_eq!(find(r"a(?=_|\b)", "a."), Some((0, 1)));
}

#[test]
fn negative_lookahead_fail() {
    // This was a tricky one. There's a negative lookahead that contains a
    // "hard" alternative (because of the lookahead). When the VM gets to the
    // point where the body of the negative lookahead matched, it needs to fail
    // the negative lookahead match. That means it needs to pop the stack until
    // before the negative lookahead, and then fail. But how many times it has
    // to pop is not fixed, it depends on the body/VM state.
    assert_eq!(find(r"(?!a(?=b)|x)", "ab"), Some((1, 1)));
    assert_eq!(find(r"(?!`(?:[^`]+(?=`)|x)`)", "`a`"), Some((1, 1)));
}

#[test]
fn backref_for_unmatched_group() {
    assert_eq!(find(r"(a)?\1", "bb"), None);
}

#[test]
fn backref_with_multibyte() {
    assert_eq!(
        find(r"(.+)\1+", "x\u{1F431}\u{1F436}\u{1F431}\u{1F436}"),
        Some((1, 17))
    );
}

#[test]
fn case_insensitive_backref_with_non_ascii() {
    assert_eq!(
        find(r"(?i)(?<word>\w+)\s+\k<word>", "Greek : δ Δ"),
        Some((8, 13))
    );
    assert_eq!(
        find(r"(?i)(?<word>\w+)\s+\k<word>", "foo 🎯 test bar BaR"),
        Some((14, 21))
    );
}

#[test]
fn repeat_non_greedy() {
    // (?=a) to make it fancy and use VM
    assert_eq!(find(r"(a(?=a)){2,}?", "aaa"), Some((0, 2)));
    assert_eq!(find(r"(a(?=a)){2}?a", "aaa"), Some((0, 3)));
}

#[test]
fn empty_repeat_non_greedy() {
    // (?=b) to make it fancy and use VM
    assert_eq!(find(r"(a(?=b)|)+?", "ab"), Some((0, 1)));
    // This tests the "prevent zero-length match on repeat" logic
    assert_eq!(find(r"(a(?=b)|)+?x", "ab"), None);
}

#[test]
fn any_match_unicode_scalar_value() {
    assert_eq!(find(r"(.)\1", "\u{1F60A}\u{1F60A}"), Some((0, 8)));
    assert_eq!(find(r"(?s)(.)\1", "\u{1F60A}\u{1F60A}"), Some((0, 8)));
}

#[test]
fn delegates_match_unicode_scalar_value() {
    assert_eq!(find(r".(?=a)", "\u{1F60A}a"), Some((0, 4)));
    assert_eq!(find(r".(?=\ba+)", "\u{1F60A}a"), Some((0, 4)));
}

#[test]
fn keepout_matches_in_correct_place() {
    assert_eq!(find(r"a\Kb", "aaab"), Some((3, 4)));
    assert_eq!(find(r".+\Kb", "aaab"), Some((3, 4)));
    assert_eq!(find(r"(?:aaa\K)b", "aaab"), Some((3, 4)));
}

#[test]
fn keepout_in_lookarounds_match_in_correct_place() {
    assert_eq!(find(r"(?<=a\Kb)c", "abc"), Some((1, 3)));
    assert_eq!(find(r"(?<!a\Kb)c", "axc"), Some((2, 3)));
    assert_eq!(find(r"a(?=b\Kc)", "abc"), Some((1, 1)));
    assert_eq!(find(r"a(?=b\Kc)..", "abc"), Some((2, 3)));
    assert_eq!(find(r"a(?!b\Kc)", "abx"), Some((0, 1)));
}

#[test]
fn find_no_matches_when_continuing_from_previous_match_end_and_no_match_at_start_of_text() {
    assert_eq!(find(r"\G(\d)\d", " 1122 33"), None);
}

#[test]
fn find_iter() {
    let text = "11 22 33";

    for (i, mat) in common::regex(r"(\d)\d").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 2)),
            1 => assert_eq!((mat.start(), mat.end()), (3, 5)),
            2 => assert_eq!((mat.start(), mat.end()), (6, 8)),
            i => panic!("Expected 3 captures, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_overlapping_lookahead() {
    let text = "abcdef";

    for (i, mat) in common::regex(r"[a-z]{2}(?=[a-z])")
        .find_iter(text)
        .enumerate()
    {
        let mat = mat.unwrap();

        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 2)),
            1 => assert_eq!((mat.start(), mat.end()), (2, 4)),
            i => panic!("Expected 2 captures, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_zero_length() {
    let text = "ab1c2";

    for (i, mat) in common::regex(r"\d*(?=[a-z])").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 0)),
            1 => assert_eq!((mat.start(), mat.end()), (1, 1)),
            2 => assert_eq!((mat.start(), mat.end()), (2, 3)),
            i => panic!("Expected 3 captures, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_zero_length_longer_codepoint() {
    let text = "é1é";

    for (i, mat) in common::regex(r"\d*(?=é)").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 0)),
            1 => assert_eq!((mat.start(), mat.end()), (2, 3)),
            i => panic!("Expected 2 captures, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_continue_from_previous_match_end() {
    let text = "1122 33";

    for (i, mat) in common::regex(r"\G(\d)\d").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 2)),
            1 => assert_eq!((mat.start(), mat.end()), (2, 4)),
            i => panic!("Expected 2 results, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_continue_from_previous_match_end_with_zero_width_match() {
    let text = "1122 33";

    for (i, mat) in common::regex(r"\G\d*").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 4)),
            i => panic!("Expected 1 result, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_continue_from_previous_match_end_inside_alternation_continue_last() {
    // Test that \G in alternation doesn't cause overzealous early bailout
    // Pattern: abc|\G1
    // The alternation allows matching "abc" at any position,
    // while \G1 only matches "1" at the continuation position
    let text = "1hello1 abc1";

    for (i, mat) in common::regex(r"abc|\G1").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        // Expected matches:
        // 1. "1" at position 0 (via \G1 branch at start)
        // 2. "abc" at position 8 (via abc branch)
        // 3. "1" at position 11 (via \G1 branch continuing from previous match end)
        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 1)),
            1 => assert_eq!((mat.start(), mat.end()), (8, 11)),
            2 => assert_eq!((mat.start(), mat.end()), (11, 12)),
            i => panic!("Expected 3 results, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_continue_from_previous_match_end_inside_alternation_continue_first() {
    // Test with \G branch first in alternation: \G\d+|abc
    // This ensures the optimization doesn't incorrectly bail out
    let text = "123abc456";

    for (i, mat) in common::regex(r"\G\d+|abc").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        // Expected matches:
        // 1. "123" at position 0 (via \G\d+ branch at start)
        // 2. "abc" at position 3 (via abc branch continuing from previous match end)
        // 3. "456" at position 6 (via \G\d+ branch continuing from previous match end)
        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 3)),
            1 => assert_eq!((mat.start(), mat.end()), (3, 6)),
            2 => assert_eq!((mat.start(), mat.end()), (6, 9)),
            i => panic!("Expected 3 results, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_continue_from_previous_match_end_inside_conditional_lookahead() {
    let text = "1a23ba456a";

    for (i, mat) in common::regex(r"(?((?=\G))a|b)").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        // Expected matches:
        // 1. "b" at position 4 (because \G didn't match)
        // 2. "a" at position 5 (because \G matches)
        match i {
            0 => assert_eq!((mat.start(), mat.end()), (4, 5)),
            1 => assert_eq!((mat.start(), mat.end()), (5, 6)),
            i => panic!("Expected 2 results, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_continue_from_previous_match_end_inside_conditional_lookbehind() {
    let text = "1a23bab456a";

    for (i, mat) in common::regex(r"(?((?!\G))a|b)").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        // Expected matches:
        // 1. "a" at position 1 (because \G didn't match)
        // 2. "a" at position 5 (because \G didn't match)
        // 3. "b" at position 6 (because \G did match)
        // 4. "a" at position 10 (because \G didn't match)
        match i {
            0 => assert_eq!((mat.start(), mat.end()), (1, 2)),
            1 => assert_eq!((mat.start(), mat.end()), (5, 6)),
            2 => assert_eq!((mat.start(), mat.end()), (6, 7)),
            3 => assert_eq!((mat.start(), mat.end()), (10, 11)),
            i => panic!("Expected 4 results, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_continue_from_previous_match_end_single_match() {
    // Test basic \G behavior with find_iter - should only match at start
    let text = "123 456 789";

    // Should only match at position 0
    for (i, mat) in common::regex(r"\G\d+").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        match i {
            0 => assert_eq!((mat.start(), mat.end()), (0, 3)),
            i => panic!("Expected 1 result, got {}", i + 1),
        }
    }
}

#[test]
fn find_iter_attributes() {
    let text = "ab1c2";
    let regex = common::regex(r"\d*(?=[a-z])");

    let matches = regex.find_iter(text);

    assert_eq!(matches.text(), text);
    assert_eq!(regex.as_str(), matches.regex().as_str());
}

#[test]
fn find_iter_empty_repeat_issue70() {
    fn assert_expected_matches(pattern: &str) {
        let text = "a\naaa\n";
        let regex = common::regex(pattern);

        let matches: Vec<_> = regex.find_iter(text).collect();
        assert_eq!(matches.len(), 4);

        for i in 0..matches.len() {
            let mat = &matches[i].as_ref().unwrap();
            match i {
                0 => assert_eq!((mat.start(), mat.end()), (0, 0)),
                1 => assert_eq!((mat.start(), mat.end()), (2, 2)),
                2 => assert_eq!((mat.start(), mat.end()), (3, 5)),
                3 => assert_eq!((mat.start(), mat.end()), (6, 6)),
                i => panic!("Expected 4 results, got {}", i + 1),
            }
        }
    }

    assert_expected_matches(r"(?m)(?:^|a)+");
    assert_expected_matches(r"(?m)(?:^|a)(?:^|a)*");
    assert_expected_matches(r"(?m)(?>)(?:^|a)+");
    assert_expected_matches(r"(?m)(?>)(?:^|a)(?:^|a)*");
}

#[test]
fn find_iter_empty_repeat_non_greedy_issue70() {
    fn assert_expected_matches(pattern: &str) {
        let text = "a\naaa\n";
        let regex = common::regex(pattern);

        let matches: Vec<_> = regex.find_iter(text).collect();
        assert_eq!(matches.len(), 5);

        for i in 0..matches.len() {
            let mat = &matches[i].as_ref().unwrap();
            match i {
                0 => assert_eq!((mat.start(), mat.end()), (0, 0)),
                1 => assert_eq!((mat.start(), mat.end()), (2, 2)),
                2 => assert_eq!((mat.start(), mat.end()), (3, 4)),
                3 => assert_eq!((mat.start(), mat.end()), (4, 5)),
                4 => assert_eq!((mat.start(), mat.end()), (6, 6)),
                i => panic!("Expected 4 results, got {}", i + 1),
            }
        }
    }

    assert_expected_matches(r"(?m)(?:^|a)+?");
    assert_expected_matches(r"(?m)(?:^|a)(?:^|a)*?");
    assert_expected_matches(r"(?m)(?>)(?:^|a)+?");
    assert_expected_matches(r"(?m)(?>)(?:^|a)(?:^|a)*?");
}

#[test]
fn find_iter_empty_repeat_anchored_non_greedy_issue70() {
    fn assert_expected_matches(pattern: &str) {
        let text = "a\naaa\n";
        let regex = common::regex(pattern);

        let matches: Vec<_> = regex.find_iter(text).collect();
        assert_eq!(matches.len(), 3);

        for i in 0..matches.len() {
            let mat = &matches[i].as_ref().unwrap();
            match i {
                0 => assert_eq!((mat.start(), mat.end()), (0, 1)),
                1 => assert_eq!((mat.start(), mat.end()), (2, 5)),
                2 => assert_eq!((mat.start(), mat.end()), (6, 6)),
                i => panic!("Expected 3 results, got {}", i + 1),
            }
        }
    }

    assert_expected_matches(r"(?m)(?:^|a)+?$");
    assert_expected_matches(r"(?m)(?:^|a)(?:^|a)*?$");
    assert_expected_matches(r"(?m)(?>)(?:^|a)+?$");
    assert_expected_matches(r"(?m)(?>)(?:^|a)(?:^|a)*?$");
}

#[test]
fn find_iter_collect_when_backtrack_limit_hit() {
    use fancy_regex::Error;
    use fancy_regex::RuntimeError;

    let r = RegexBuilder::new("(x+x+)+(?>y)")
        .backtrack_limit(1)
        .build()
        .unwrap();
    let result: Vec<_> = r.find_iter("xxxxxxxxxxy").collect();
    assert_eq!(result.len(), 1);
    assert!(result[0].is_err());
    match &result[0].as_ref().err() {
        Some(Error::RuntimeError(RuntimeError::BacktrackLimitExceeded)) => {}
        _ => panic!("Expected RuntimeError::BacktrackLimitExceeded"),
    }
}

#[test]
fn find_conditional() {
    assert_eq!(find(r"(?(ab)c|d)", "acd"), Some((2, 3)));
    assert_eq!(find(r"(a)?b(?(1)c|d)", "abc"), Some((0, 3)));
    assert_eq!(find(r"(a)?b(?(1)c|d)", "abd"), Some((1, 3)));
}

#[test]
fn find_endtext_before_newlines() {
    assert_eq!(find(r"\Z", "hello\nworld\n\n\n"), Some((11, 11)));
}

#[test]
fn find_endtext_ignore_trailing_newlines_non_crlf() {
    // \Z without CRLF mode: only bare \n chars are skipped at the end.
    // "abc\n\n" — \Z matches at position 3 (before the trailing newlines)
    assert_eq!(find(r"\Z", "abc\n\n"), Some((3, 3)));
    // No trailing newlines: \Z matches at the very end
    assert_eq!(find(r"\Z", "abc"), Some((3, 3)));
    // \r\n at the end: in non-CRLF mode the \r is NOT treated as a newline for \Z,
    // so \Z matches just before the trailing \n, not before the \r\n pair.
    assert_eq!(find(r"\Z", "abc\r\n"), Some((4, 4)));
}

#[test]
fn find_endtext_ignore_trailing_newlines_crlf() {
    // (?R) enables CRLF mode; \Z should then treat \r as part of trailing newlines too.
    assert_eq!(
        RegexBuilder::new(r"\Z")
            .crlf(true)
            .build()
            .unwrap()
            .find("abc\r\n")
            .unwrap()
            .map(|m| (m.start(), m.end())),
        Some((3, 3))
    );
    assert_eq!(
        RegexBuilder::new(r"\Z")
            .crlf(true)
            .build()
            .unwrap()
            .find("abc\r\r")
            .unwrap()
            .map(|m| (m.start(), m.end())),
        Some((3, 3))
    );
    // bare \n still works in CRLF mode
    assert_eq!(
        RegexBuilder::new(r"\Z")
            .crlf(true)
            .build()
            .unwrap()
            .find("abc\n")
            .unwrap()
            .map(|m| (m.start(), m.end())),
        Some((3, 3))
    );
}

#[test]
fn continue_from_previous_match_with_lookahead_before() {
    let text = "123abc";

    // Lookahead doesn't consume, so \G should still be at position 0
    assert_eq!(find(r"(?=\d)\G\d+", text), Some((0, 3)));

    // Lookahead fails, so whole match fails
    assert_eq!(find(r"(?=[a-z])\G\d+", text), None);

    // Negative lookahead passes (not a letter), \G continues
    assert_eq!(find(r"(?![a-z])\G\d+", text), Some((0, 3)));

    // Negative lookahead fails (is a digit), match fails
    assert_eq!(find(r"(?!\d)\G\d+", text), None);
}

#[test]
fn continue_from_previous_match_in_alternation() {
    // Pattern: \G in alternation - tests whether \G behaves correctly in branches
    let text = "123abc";

    // First branch with \G should match
    assert_eq!(find(r"\G\d+|xyz", text), Some((0, 3)));

    // Second branch should also work when first fails
    assert_eq!(find(r"\Gxyz|\d+", text), Some((0, 3)));
    assert_eq!(find(r"\D+|\G123", text), Some((0, 3)));
}

#[test]
fn continue_from_previous_match_fails_when_not_at_continue_position() {
    // \G should fail when we're not at the position where the last match ended
    let text = " 123";

    // Starting match at position 0, but there's a space, so \G at position 0 followed by \d
    // should not match because we'd need to skip the space
    assert_eq!(find(r"\G\d+", text), None);
}

#[test]
fn continue_from_previous_match_at_min_position_zero() {
    let text = "123abc";

    assert_eq!(find(r"^x?\b(\G\d+)", text), Some((0, 3)));
    assert_eq!(find(r"(?:^|\b)x?(\G\d+)", text), Some((0, 3)));
    assert_eq!(find(r"(?:\G\d+|xyz)|abc", text), Some((0, 3)));
    assert_eq!(find(r"(?=\d)(?=1)\G\d+", text), Some((0, 3)));
    // Atomic group doesn't affect \G position checking
    assert_eq!(find(r"(?>\G)\d+", "123abc"), Some((0, 3)));
}

#[test]
fn find_iter_continue_from_previous_match_end_with_keepout() {
    // \G..\K produces zero-length matches because \K resets the reported start to
    // after the two consumed characters. Iteration should continue matching across
    // the whole string, advancing two bytes at a time, not stop after the first match.
    let text = "aabbcc";

    for (i, mat) in common::regex(r"\G..\K").find_iter(text).enumerate() {
        let mat = mat.unwrap();

        match i {
            0 => assert_eq!((mat.start(), mat.end()), (2, 2)),
            1 => assert_eq!((mat.start(), mat.end()), (5, 5)),
            i => panic!("Expected 2 results, got {}", i + 1),
        }
    }
}

#[test]
fn find_not_empty_skips_zero_length_match() {
    // \d* on text that doesn't start with a digit: normal mode matches empty at position 0,
    // find_not_empty mode rejects the zero-length match.
    let pattern = r"\d*";
    let text = "hello";

    let normal = common::regex(pattern);
    assert_eq!(
        normal.find(text).unwrap().map(|m| (m.start(), m.end())),
        Some((0, 0)),
    );

    let not_empty = RegexBuilder::new(pattern)
        .find_not_empty(true)
        .build()
        .unwrap();
    assert_eq!(
        not_empty.find(text).unwrap().map(|m| (m.start(), m.end())),
        None,
    );
}

#[test]
fn find_not_empty_find_iter() {
    // With find_iter, find_not_empty should keep non-empty matches but stop
    // when a zero-length match is encountered (returning None stops the iterator).
    let pattern = r"\d*";
    let text = "1a2";

    // Normal mode: "1" at 0-1, empty at 1-1 skipped (adjacent to last match end),
    // "2" at 2-3, empty at 3-3 skipped (adjacent to last match end), done.
    let normal: Vec<(usize, usize)> = common::regex(pattern)
        .find_iter(text)
        .map(|m| {
            let m = m.unwrap();
            (m.start(), m.end())
        })
        .collect();
    assert_eq!(normal, vec![(0, 1), (2, 3)]);

    // find_not_empty: "1" at (0,1) is non-empty → kept. Next search at pos 1: \d* matches ""
    // at (1,1), find_not_empty rejects and advances past the empty match. At pos 2: \d*
    // matches "2" at (2,3), which is non-empty → kept. Both non-empty matches are yielded.
    let not_empty = RegexBuilder::new(pattern)
        .find_not_empty(true)
        .build()
        .unwrap();
    let filtered: Vec<(usize, usize)> = not_empty
        .find_iter(text)
        .map(|m| {
            let m = m.unwrap();
            (m.start(), m.end())
        })
        .collect();
    assert_eq!(filtered, vec![(0, 1), (2, 3)]);
}

#[test]
fn find_not_empty_captures_iter() {
    // Same as find_not_empty_find_iter but using captures_iter to verify
    // the captures path also advances past empty matches.
    let pattern = r"(\d*)";
    let text = "1a2";

    let not_empty = RegexBuilder::new(pattern)
        .find_not_empty(true)
        .build()
        .unwrap();
    let filtered: Vec<(usize, usize)> = not_empty
        .captures_iter(text)
        .map(|c| {
            let c = c.unwrap();
            let m = c.get(0).unwrap();
            (m.start(), m.end())
        })
        .collect();
    assert_eq!(filtered, vec![(0, 1), (2, 3)]);
}

#[test]
fn find_not_empty_fancy_path() {
    // Use a lookbehind to force the fancy/VM path, then verify find_not_empty works there too.
    // (?<=^)\d* matches an empty string of digits at the start of text.
    let pattern = r"(?<=^)\d*";
    let text = "hello";

    let normal = common::regex(pattern);
    assert_eq!(
        normal.find(text).unwrap().map(|m| (m.start(), m.end())),
        Some((0, 0)),
    );

    let not_empty = RegexBuilder::new(pattern)
        .find_not_empty(true)
        .build()
        .unwrap();
    assert_eq!(
        not_empty.find(text).unwrap().map(|m| (m.start(), m.end())),
        None,
    );
}

#[test]
fn find_not_empty_fancy_find_iter() {
    // Atomic group forces the fancy/VM path.
    // (?>\d*) behaves like \d* but via the backtracking VM.
    let pattern = r"(?>\d*)";
    let text = "1a2";

    // Normal mode: same as \d* — "1" at 0-1, "2" at 2-3
    // (empty matches adjacent to the previous match end are skipped by find_iter)
    let normal: Vec<(usize, usize)> = common::regex(pattern)
        .find_iter(text)
        .map(|m| {
            let m = m.unwrap();
            (m.start(), m.end())
        })
        .collect();
    assert_eq!(normal, vec![(0, 1), (2, 3)]);

    // find_not_empty with the fancy/VM path: the VM's unanchored search loop
    // (Split/Any/Jmp preamble) allows it to advance past positions where the
    // pattern would produce an empty match. So unlike the Wrap path, the VM
    // at pos 1 rejects the empty match via OPTION_FIND_NOT_EMPTY but then
    // advances and finds "2" at (2,3). Both non-empty matches are yielded.
    let not_empty = RegexBuilder::new(pattern)
        .find_not_empty(true)
        .build()
        .unwrap();
    let filtered: Vec<(usize, usize)> = not_empty
        .find_iter(text)
        .map(|m| {
            let m = m.unwrap();
            (m.start(), m.end())
        })
        .collect();
    assert_eq!(filtered, vec![(0, 1), (2, 3)]);
}

#[test]
fn find_not_empty_trailing_lookahead_with_variable_group0() {
    // \d*(?=x) optimizes to (\d*)x with explicit_capture_group_0.
    // The user-visible match is explicit group 0 (\d*). When \d* matches empty,
    // find_not_empty should reject the match even though the physical match
    // (including the "x") is non-empty.
    let pattern = r"\d*(?=x)";
    let text = "xhello";

    // Normal mode: matches empty \d* followed by lookahead for x → match at (0, 0)
    let normal = common::regex(pattern);
    assert_eq!(
        normal.find(text).unwrap().map(|m| (m.start(), m.end())),
        Some((0, 0)),
    );

    // find_not_empty: the user-visible match (group 0) is empty → rejected
    let not_empty = RegexBuilder::new(pattern)
        .find_not_empty(true)
        .build()
        .unwrap();
    assert_eq!(
        not_empty.find(text).unwrap().map(|m| (m.start(), m.end())),
        None,
    );

    // But with digits before the x, the match is non-empty → accepted
    let text_with_digits = "12xhello";
    assert_eq!(
        not_empty
            .find(text_with_digits)
            .unwrap()
            .map(|m| (m.start(), m.end())),
        Some((0, 2)),
    );
}

#[test]
fn find_not_empty_alternation_with_empty_branch() {
    // (?:|a|b) can match empty, "a", or "b". Without find_not_empty, the empty branch wins
    // at position 0 (leftmost, first alternative). With find_not_empty, the empty match at
    // position 0 must be skipped, and the pattern should then find "a" or "b".
    let pattern = r"(?:|a|b)";

    // Normal mode: empty match at position 0
    let normal = common::regex(pattern);
    assert_eq!(
        normal.find("a").unwrap().map(|m| (m.start(), m.end())),
        Some((0, 0)),
    );

    // find_not_empty mode: empty match rejected; "a" found instead
    let not_empty = RegexBuilder::new(pattern)
        .find_not_empty(true)
        .build()
        .unwrap();
    assert_eq!(
        not_empty.find("a").unwrap().map(|m| (m.start(), m.end())),
        Some((0, 1)),
    );
    assert_eq!(
        not_empty.find("b").unwrap().map(|m| (m.start(), m.end())),
        Some((0, 1)),
    );
    assert_eq!(
        not_empty.find("c").unwrap().map(|m| (m.start(), m.end())),
        None,
    );
}

#[test]
fn find_not_empty_with_keep_out() {
    // The find_not_empty check compares what the engine consumed, regardless of the match reported,
    // which could be affected by \K.
    let not_empty = RegexBuilder::new(r"a\Kb")
        .find_not_empty(true)
        .build()
        .unwrap();
    assert_eq!(
        not_empty.find("ab").unwrap().map(|m| (m.start(), m.end())),
        Some((1, 2)),
    );

    // `ab\K` physically consumes "ab" (ix advances to 2), so it is accepted as non-empty.
    // `\K` resets saves[0] to 2, giving the reported span (2, 2).
    let not_empty_abk = RegexBuilder::new(r"ab\K")
        .find_not_empty(true)
        .build()
        .unwrap();
    assert_eq!(
        not_empty_abk
            .find("ab")
            .unwrap()
            .map(|m| (m.start(), m.end())),
        Some((2, 2)),
    );
}

#[test]
fn find_not_empty_with_continue_from_previous_match_end() {
    // \G\d+ with find_not_empty: only matches where the previous match ended and
    // the match is non-empty. On "1122 33", \G matches at position 0; "1122" is
    // consumed (non-empty → accepted). The next call starts at pos=4 (space),
    // \G succeeds but \d+ fails → no further matches.
    let not_empty = RegexBuilder::new(r"\G\d+")
        .find_not_empty(true)
        .build()
        .unwrap();
    let matches: Vec<_> = not_empty
        .find_iter("1122 33")
        .map(|m| {
            let m = m.unwrap();
            (m.start(), m.end())
        })
        .collect();
    assert_eq!(matches, vec![(0, 4)]);

    // \G\d* can produce an empty match after the digits run out. find_not_empty
    // must reject that empty match. On "1122 33", after the non-empty "1122"
    // match the next attempt is at position 4 (space); \G\d* would match empty
    // there, but find_not_empty rejects it, so no further matches are produced.
    let not_empty_star = RegexBuilder::new(r"\G\d*")
        .find_not_empty(true)
        .build()
        .unwrap();
    let matches_star: Vec<_> = not_empty_star
        .find_iter("1122 33")
        .map(|m| {
            let m = m.unwrap();
            (m.start(), m.end())
        })
        .collect();
    assert_eq!(matches_star, vec![(0, 4)]);
}

#[test]
fn find_not_empty_with_continue_from_previous_match_end_and_keep_out() {
    // \G..\K produces a zero-length reported span (saves[0] reset to after the
    // two consumed bytes), but physically consumes two characters. With
    // find_not_empty the check is `ix == match_attempt_start`: since two bytes
    // were consumed, ix > match_attempt_start and the match is accepted despite
    // the reported span being zero-length.
    let not_empty = RegexBuilder::new(r"\G..\K")
        .find_not_empty(true)
        .build()
        .unwrap();
    let matches: Vec<_> = not_empty
        .find_iter("aabbcc")
        .map(|m| {
            let m = m.unwrap();
            (m.start(), m.end())
        })
        .collect();
    // Each match physically consumes 2 bytes; \K resets the reported start to
    // the physical end, yielding zero-length spans. The iterator advances by 1
    // after each zero-length match (OPTION_SKIPPED_EMPTY_MATCH), so the pattern
    // matches at positions (2,2) and (5,5), mirroring the behavior without find_not_empty.
    assert_eq!(matches, vec![(2, 2), (5, 5)]);
}

#[test]
fn find_not_empty_with_anchored_pattern() {
    // ^-anchored patterns with find_not_empty go through the VM path (because
    // find_not_empty forces the VM) but do NOT emit a SplitUnanchored preamble
    // (the pattern is compiled as anchored). match_attempt_start stays at pos,
    // so ix == match_attempt_start still correctly detects the empty match.
    let not_empty = RegexBuilder::new(r"^\d*")
        .find_not_empty(true)
        .build()
        .unwrap();

    // No leading digits → empty match at position 0 → rejected
    assert_eq!(
        not_empty
            .find("hello")
            .unwrap()
            .map(|m| (m.start(), m.end())),
        None,
    );

    // Leading digits → non-empty match at position 0 → accepted
    assert_eq!(
        not_empty
            .find("123hello")
            .unwrap()
            .map(|m| (m.start(), m.end())),
        Some((0, 3)),
    );
}

#[test]
fn find_from_pos_past_end_wrap() {
    let re = RegexBuilder::new(r"^(<{7})(?:\s+(\S.*?))?$\n?")
        .oniguruma_mode(true)
        .build()
        .unwrap();
    let result = re.find_from_pos("ab", 12);
    assert_matches!(result, Ok(None));
}

#[test]
fn find_from_pos_past_end_fancy() {
    let re = RegexBuilder::new(r"(?<=a)b")
        .oniguruma_mode(true)
        .build()
        .unwrap();
    let result = re.find_from_pos("ab", 12);
    assert_matches!(result, Ok(None));
}

#[test]
fn find_from_pos_at_end_still_runs() {
    let re = RegexBuilder::new("$").oniguruma_mode(true).build().unwrap();
    let result = re.find_from_pos("ab", 2).unwrap();
    assert!(result.is_some(), "pattern `$` must match at end-of-text");
}

fn find(re: &str, text: &str) -> Option<(usize, usize)> {
    find_match(re, text).map(|m| (m.start(), m.end()))
}

fn find_match<'t>(re: &str, text: &'t str) -> Option<Match<'t>> {
    let regex = common::regex(re);
    let result = regex.find(text);
    assert!(
        result.is_ok(),
        "Expected find to succeed, but was {:?}",
        result
    );
    result.unwrap()
}
