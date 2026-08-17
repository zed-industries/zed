use fancy_regex::{Captures, CompileError, Error, Expander, Match, Result};
use matches::assert_matches;
use std::borrow::Cow;
use std::ops::Index;
use std::ops::Range;

mod common;

#[test]
fn capture_names() {
    let regex = common::regex("(?<foo>)()(?P<bar>)");
    let capture_names = regex.capture_names().collect::<Vec<_>>();
    assert_eq!(capture_names, vec![None, Some("foo"), None, Some("bar")]);
}

#[test]
fn captures_fancy() {
    let captures = captures(r"\s*(\w+)(?=\.)", "foo bar.");
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), " bar", 3, 7);
    assert_match(captures.get(1), "bar", 4, 7);
    assert!(captures.get(2).is_none());
}

#[test]
fn captures_fancy_named() {
    let captures = captures(r"\s*(?<name>\w+)(?=\.)", "foo bar.");
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), " bar", 3, 7);
    assert_match(captures.name("name"), "bar", 4, 7);
    assert_eq!(captures.index(0), " bar");
    assert_eq!(captures.index("name"), "bar");
    assert!(captures.get(2).is_none());
}

#[test]
fn captures_fancy_unmatched_group() {
    let captures = captures(r"(\w+)(?=\.)|(\w+)(?=!)", "foo! bar.");
    assert_eq!(captures.len(), 3);
    assert_match(captures.get(0), "foo", 0, 3);
    assert!(captures.get(1).is_none());
    assert_match(captures.get(2), "foo", 0, 3);
}

#[test]
fn captures_after_lookbehind() {
    let captures = captures(
        r"\s*(?<=[() ])(@\w+)(\([^)]*\))?\s*",
        " @another(foo bar)   ",
    );
    assert_match(captures.get(1), "@another", 1, 9);
    assert_match(captures.get(2), "(foo bar)", 9, 18);
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn non_empty_captures_inside_variable_lookbehind() {
    let captures = captures(r"(?<=(a+)(b+))x", "aaabbbx");
    assert_eq!(captures.len(), 3);
    assert_match(captures.get(0), "x", 6, 7);
    assert_match(captures.get(1), "aaa", 0, 3);
    assert_match(captures.get(2), "bbb", 3, 6);
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn empty_capture_inside_variable_lookbehind() {
    let captures = captures(r"(?<=(a+)(b*))c", "aaac");
    assert_eq!(captures.len(), 3);
    assert_match(captures.get(0), "c", 3, 4);
    assert_match(captures.get(1), "aaa", 0, 3);
    // Empty match for b* since there are no b's
    assert_match(captures.get(2), "", 3, 3);
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn named_captures_inside_variable_lookbehind() {
    let captures = captures(r"(?<=(?<first>a+)(?<second>b+))x", "aaabbbx");
    assert_eq!(captures.len(), 3);
    assert_match(captures.get(0), "x", 6, 7);
    assert_match(captures.name("first"), "aaa", 0, 3);
    assert_match(captures.name("second"), "bbb", 3, 6);
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn captures_both_inside_and_outside_variable_lookbehind() {
    let captures = captures(r"(?<=(a+)(b+))(c+)(d+)", "aaabbbcccddd");
    assert_eq!(captures.len(), 5);
    assert_match(captures.get(0), "cccddd", 6, 12);
    assert_match(captures.get(1), "aaa", 0, 3);
    assert_match(captures.get(2), "bbb", 3, 6);
    assert_match(captures.get(3), "ccc", 6, 9);
    assert_match(captures.get(4), "ddd", 9, 12);
}

#[test]
fn captures_with_keepout_inside_at_end() {
    let captures = captures(r"\s*(\w+\K)(?=\.)", "foo bar.");
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "", 7, 7);
    assert_match(captures.get(1), "bar", 4, 7);
    assert!(captures.get(2).is_none());
}

#[test]
fn captures_with_keepout_inside_in_middle() {
    let captures = captures(r"\s*(b\Kar)(?=\.)", "foo bar.");
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "ar", 5, 7);
    assert_match(captures.get(1), "bar", 4, 7);
    assert!(captures.get(2).is_none());
}

#[test]
fn captures_with_keepout_between() {
    let captures = captures(r"(\w+)\K\s*(\w+)(?=\.)", "foo bar.");
    assert_eq!(captures.len(), 3);
    assert_match(captures.get(0), " bar", 3, 7);
    assert_match(captures.get(1), "foo", 0, 3);
    assert_match(captures.get(2), "bar", 4, 7);
    assert!(captures.get(3).is_none());
}

#[test]
fn captures_with_nested_keepout() {
    let captures = captures(r"(\w\K)+\s*(\w+)(?=\.)", "foo bar.");
    assert_eq!(captures.len(), 3);
    assert_match(captures.get(0), " bar", 3, 7);
    assert_match(captures.get(1), "o", 2, 3);
    assert_match(captures.get(2), "bar", 4, 7);
    assert!(captures.get(3).is_none());
}

#[test]
fn captures_with_conditional() {
    let captures = captures(r"^(?((ab))c|d)$", "abc");
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "abc", 0, 3);
    assert_match(captures.get(1), "ab", 0, 2);
    assert!(captures.get(2).is_none());
}

#[test]
fn capture_with_crlf_flag() {
    // The following test cases are adopted from the test cases of the `regex-automata` crate:
    // https://github.com/rust-lang/regex/blob/c79c40a39ebc4ffdc9b886e34806b9085583b769/testdata/crlf.toml
    assert_capture_with_crlf_flag(
        r"(?mR)^[a-z]+$",
        "abc\r\ndef\r\nxyz",
        vec!["abc", "def", "xyz"],
        vec![(0..3), (5..8), (10..13)],
    );
    assert_capture_with_crlf_flag(r"(?mR)^$", "abc\r\ndef\r\nxyz", vec![], vec![]);
    assert_capture_with_crlf_flag(r"(?mR)^$", "", vec![""], vec![(0..0)]);
    assert_capture_with_crlf_flag(r"(?mR)^$", "\r\n", vec!["", ""], vec![(0..0), (2..2)]);
    assert_capture_with_crlf_flag(
        r"(?mR)^",
        "abc\r\ndef\r\nxyz",
        vec!["", "", ""],
        vec![(0..0), (5..5), (10..10)],
    );
    assert_capture_with_crlf_flag(
        r"(?mR)^",
        "\r\n\r\n\r\n",
        vec!["", "", "", ""],
        vec![(0..0), (2..2), (4..4), (6..6)],
    );
    assert_capture_with_crlf_flag(
        r"(?mR)^",
        "\r\r\r",
        vec!["", "", "", ""],
        vec![(0..0), (1..1), (2..2), (3..3)],
    );
    assert_capture_with_crlf_flag(
        r"(?mR)^",
        "\n\n\n",
        vec!["", "", "", ""],
        vec![(0..0), (1..1), (2..2), (3..3)],
    );
    assert_capture_with_crlf_flag(
        r"(?mR)$",
        "abc\r\ndef\r\nxyz",
        vec!["", "", ""],
        vec![(3..3), (8..8), (13..13)],
    );
    assert_capture_with_crlf_flag(
        r"(?mR)$",
        "\r\n\r\n\r\n",
        vec!["", "", "", ""],
        vec![(0..0), (2..2), (4..4), (6..6)],
    );
    assert_capture_with_crlf_flag(
        r"(?mR)$",
        "\r\r\r",
        vec!["", "", "", ""],
        vec![(0..0), (1..1), (2..2), (3..3)],
    );
    assert_capture_with_crlf_flag(
        r"(?mR)$",
        "\n\n\n",
        vec!["", "", "", ""],
        vec![(0..0), (1..1), (2..2), (3..3)],
    );
    // In CRLF mode, `.` should not match `\r` or `\n`
    assert_capture_with_crlf_flag(r"(?R).", "\r\n\r\n\r\n", vec![], vec![]);
}

#[cfg_attr(feature = "track_caller", track_caller)]
fn assert_capture_with_crlf_flag(
    regex_src: &str,
    text: &str,
    expected_texts: Vec<&str>,
    expected_spans: Vec<Range<usize>>,
) {
    fn assert_capture_matches(
        regex_src: &str,
        text: &str,
        expected_texts: &[&str],
        expected_spans: &[Range<usize>],
    ) {
        let regex = common::regex(regex_src);
        let capture_matches: Vec<_> = regex.captures_iter(text).map(|c| c.unwrap()).collect();
        assert_eq!(
            capture_matches.len(),
            expected_texts.len(),
            "Pattern '{}' on '{}': expected {} matches, got {}",
            regex_src,
            text.escape_default(),
            expected_texts.len(),
            capture_matches.len(),
        );
        for (captures, (&expected_text, expected_span)) in capture_matches
            .iter()
            .zip(expected_texts.iter().zip(expected_spans.iter()))
        {
            assert_match(
                captures.get(0),
                expected_text,
                expected_span.start,
                expected_span.end,
            );
        }
    }

    fn make_harder(regex_src: &str) -> String {
        // Wrap the original regex source in an atomic group to make it `hard` so that the execution of the regex is
        // handled by the backtracking VM engine implemented in `fancy-regex` rather than delegated to the engine of
        // `regex-automata`.
        format!(r"(?>{})", regex_src)
    }

    // Verify that the regex-automata's engine is able to handle the CRLF flag.
    assert_capture_matches(regex_src, text, &expected_texts, &expected_spans);

    // Verify that the fancy-regex's backtracking VM engine is able to handle the CRLF flag.
    assert_capture_matches(
        make_harder(regex_src).as_str(),
        text,
        &expected_texts,
        &expected_spans,
    );
}

#[test]
fn captures_iter() {
    let text = "11 21 33";

    for (i, captures) in common::regex(r"(?P<num>\d)\d")
        .captures_iter(text)
        .enumerate()
    {
        let captures = captures.unwrap();

        match i {
            0 => {
                assert_eq!(captures.len(), 2);
                assert_match(captures.get(0), "11", 0, 2);
                assert_match(captures.name("num"), "1", 0, 1);
            }
            1 => {
                assert_eq!(captures.len(), 2);
                assert_match(captures.get(0), "21", 3, 5);
                assert_match(captures.name("num"), "2", 3, 4);
            }
            2 => {
                assert_eq!(captures.len(), 2);
                assert_match(captures.get(0), "33", 6, 8);
                assert_match(captures.name("num"), "3", 6, 7);
            }
            i => panic!("Expected 3 captures, got {}", i + 1),
        }
    }
}

#[test]
fn captures_iter_attributes() {
    let text = "11 21 33";
    let regex = common::regex(r"(?P<num>\d)\d");

    let all_captures = regex.captures_iter(text);

    assert_eq!(all_captures.text(), text);
    assert_eq!(regex.as_str(), all_captures.regex().as_str());
}

#[test]
fn captures_iter_continue_from_previous_match_end() {
    let text = "1122 33";

    for (i, caps) in common::regex(r"\G(\d)\d").captures_iter(text).enumerate() {
        let caps = caps.unwrap();

        match i {
            0 => {
                assert_eq!(caps.get(0).unwrap().start(), 0);
                assert_eq!(caps.get(0).unwrap().end(), 2);
                assert_eq!(caps.get(1).unwrap().start(), 0);
                assert_eq!(caps.get(1).unwrap().end(), 1);
            }
            1 => {
                assert_eq!(caps.get(0).unwrap().start(), 2);
                assert_eq!(caps.get(0).unwrap().end(), 4);
                assert_eq!(caps.get(1).unwrap().start(), 2);
                assert_eq!(caps.get(1).unwrap().end(), 3);
            }
            i => panic!("Expected 2 results, got {}", i + 1),
        }
    }
}

#[test]
fn captures_iter_continue_from_previous_match_end_with_zero_width_match() {
    let text = "1122 33";

    for (i, caps) in common::regex(r"\G\d*").captures_iter(text).enumerate() {
        let caps = caps.unwrap();

        match i {
            0 => {
                assert_eq!(caps.get(0).unwrap().start(), 0);
                assert_eq!(caps.get(0).unwrap().end(), 4);
            }
            i => panic!("Expected 1 result, got {}", i + 1),
        }
    }
}

#[test]
fn captures_iter_continue_from_previous_match_end_single_match() {
    let text = "123 456 789";

    for (i, caps) in common::regex(r"\G(\d+)").captures_iter(text).enumerate() {
        let caps = caps.unwrap();

        match i {
            0 => {
                assert_eq!(caps.get(0).unwrap().start(), 0);
                assert_eq!(caps.get(0).unwrap().end(), 3);
                assert_eq!(caps.get(1).unwrap().start(), 0);
                assert_eq!(caps.get(1).unwrap().end(), 3);
            }
            i => panic!("Expected 1 result, got {}", i + 1),
        }
    }
}

#[test]
fn captures_iter_continue_from_previous_match_end_with_keepout() {
    // \G..\K(?=(.)) — \K resets the match start to after two consumed bytes, producing
    // zero-length group 0 matches. Iteration must not stop after the first match because
    // the VM actually consumed real bytes before \K fired, so \G should keep succeeding.
    let text = "aabbcc";

    // Each match: group 0 is zero-length at the position after the two consumed chars,
    // group 1 captures the next single char via the lookahead.
    for (i, caps) in common::regex(r"\G..\K(?=(.))")
        .captures_iter(text)
        .enumerate()
    {
        let caps = caps.unwrap();

        match i {
            0 => {
                assert_eq!(caps.get(0).unwrap().start(), 2);
                assert_eq!(caps.get(0).unwrap().end(), 2);
                assert_eq!(caps.get(1).unwrap().start(), 2);
                assert_eq!(caps.get(1).unwrap().end(), 3);
            }
            1 => {
                assert_eq!(caps.get(0).unwrap().start(), 5);
                assert_eq!(caps.get(0).unwrap().end(), 5);
                assert_eq!(caps.get(1).unwrap().start(), 5);
                assert_eq!(caps.get(1).unwrap().end(), 6);
            }
            i => panic!("Expected 2 results, got {}", i + 1),
        }
    }
}

#[test]
fn captures_from_pos() {
    let text = "11 21 33";

    let regex = common::regex(r"(\d)\d");
    let captures = assert_captures(regex.captures_from_pos(text, 3));
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "21", 3, 5);
    assert_match(captures.get(1), "2", 3, 4);
    let matches: Vec<_> = captures.iter().collect();
    assert_eq!(matches.len(), 2);
    assert_match(matches[0], "21", 3, 5);
    assert_match(matches[1], "2", 3, 4);

    let regex = common::regex(r"(\d+)\1");
    let captures = assert_captures(regex.captures_from_pos(text, 3));
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "33", 6, 8);
    assert_match(captures.get(1), "3", 6, 7);
    let matches: Vec<_> = captures.iter().collect();
    assert_eq!(matches.len(), 2);
    assert_match(matches[0], "33", 6, 8);
    assert_match(matches[1], "3", 6, 7);

    let regex = common::regex(r"(?P<foo>\d+)\k<foo>");
    let captures = assert_captures(regex.captures_from_pos(text, 3));
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "33", 6, 8);
    assert_match(captures.name("foo"), "3", 6, 7);
    let matches: Vec<_> = captures.iter().collect();
    assert_eq!(matches.len(), 2);
    assert_match(matches[0], "33", 6, 8);
    assert_match(matches[1], "3", 6, 7);

    let regex = common::regex(r"(?P<foo>\d+)(?P=foo)");
    let captures = assert_captures(regex.captures_from_pos(text, 3));
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "33", 6, 8);
    assert_match(captures.name("foo"), "3", 6, 7);
    let matches: Vec<_> = captures.iter().collect();
    assert_eq!(matches.len(), 2);
    assert_match(matches[0], "33", 6, 8);
    assert_match(matches[1], "3", 6, 7);
}

#[test]
fn captures_from_pos_past_end_wrap() {
    let re = fancy_regex::RegexBuilder::new(r"^(<{7})(?:\s+(\S.*?))?$\n?")
        .oniguruma_mode(true)
        .build()
        .unwrap();
    let result = re.captures_from_pos("ab", 12);
    assert_matches!(result, Ok(None));
}

#[test]
fn captures_from_pos_past_end_fancy() {
    let re = fancy_regex::RegexBuilder::new(r"(?<=a)b")
        .oniguruma_mode(true)
        .build()
        .unwrap();
    let result = re.captures_from_pos("ab", 12);
    assert_matches!(result, Ok(None));
}

#[test]
fn captures_from_pos_at_end_still_runs() {
    let re = fancy_regex::RegexBuilder::new("$")
        .oniguruma_mode(true)
        .build()
        .unwrap();
    let result = re.captures_from_pos("ab", 2).unwrap();
    assert!(result.is_some(), "pattern `$` must match at end-of-text");
}

#[test]
fn captures_from_pos_looking_left() {
    let regex = common::regex(r"\b(\w)");

    // This should *not* match because `\b` doesn't match between a and x
    let result = regex.captures_from_pos("ax", 1).unwrap();
    assert!(result.is_none());

    let captures = assert_captures(regex.captures_from_pos(".x", 1));
    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "x", 1, 2);
    assert_match(captures.get(1), "x", 1, 2);
}

#[test]
fn captures_iter_collect_when_backtrack_limit_hit() {
    use fancy_regex::RegexBuilder;
    let r = RegexBuilder::new("(x+x+)+(?>y)")
        .backtrack_limit(1)
        .build()
        .unwrap();
    let result: Vec<_> = r.captures_iter("xxxxxxxxxxy").collect();
    println!("{:?}", result);
    assert_eq!(result.len(), 1);
    assert!(result[0].is_err());
}

#[test]
fn captures_in_lookbehind_subroutine_call() {
    let captures = captures(r"(?<=(?<A>.)\g<A>)b", "axyabc");

    assert_eq!(captures.len(), 2);
    assert_match(captures.get(0), "b", 4, 5);

    // right most capture takes precedence
    assert_match(captures.get(1), "a", 3, 4);
}

#[test]
#[cfg(feature = "variable-lookbehinds")]
fn captures_in_variable_lookbehind_subroutine_call() {
    let regex = common::regex(r"(?<=(?<A>[^b])(?<B>b+)\g<A>)d");

    let captures = assert_captures(regex.captures_from_pos("aaabbcd", 5));
    assert_eq!(captures.len(), 3);
    assert_match(captures.get(0), "d", 6, 7);
    assert_match(captures.get(2), "bb", 3, 5);

    // right most capture takes precedence
    assert_match(captures.get(1), "c", 5, 6);
}

#[test]
fn self_recursive_capture_groups() {
    // Test recursive pattern with capture groups
    let caps1 = captures(r"(?<foo>a|\(\g<foo>\))", "((((((a))))))");

    // According to Oniguruma behavior, capture group 1 should match the same as group 0
    // for the input "((((((a))))))"
    assert_eq!(caps1.len(), 2);
    assert_match(caps1.get(0), "((((((a))))))", 0, 13);
    assert_match(caps1.get(1), "((((((a))))))", 0, 13);

    // Test with simpler input "(a)"
    let caps2 = captures(r"(?<foo>a|\(\g<foo>\))", "(a)");
    assert_eq!(caps2.len(), 2);
    assert_match(caps2.get(0), "(a)", 0, 3);
    assert_match(caps2.get(1), "(a)", 0, 3);
}

#[test]
fn forward_reference_subroutine_capture_groups() {
    // Test that a subroutine call updates the capture group
    let captures = captures(r"\g<_B>\g<_B>|\zEND(?<_A>.a.)(?<_B>.b.)", "xbxyby");

    assert_eq!(captures.len(), 3);
    assert_match(captures.get(0), "xbxyby", 0, 6);
    assert!(captures.name("_A").is_none());
    assert_match(captures.name("_B"), "yby", 3, 6);
}

#[cfg_attr(feature = "track_caller", track_caller)]
fn captures<'a>(re: &str, text: &'a str) -> Captures<'a> {
    let regex = common::regex(re);
    let result = regex.captures(text);
    assert_captures(result)
}

#[cfg_attr(feature = "track_caller", track_caller)]
fn assert_captures(result: Result<Option<Captures<'_>>>) -> Captures<'_> {
    assert!(
        result.is_ok(),
        "Expected captures to succeed, but was {:?}",
        result
    );
    let captures = result.unwrap();
    assert!(
        captures.is_some(),
        "Expected captures, but was {:?}",
        captures
    );
    captures.unwrap()
}

#[cfg_attr(feature = "track_caller", track_caller)]
fn assert_match(m: Option<Match<'_>>, expected_text: &str, start: usize, end: usize) {
    assert!(m.is_some(), "Expected match, but was {:?}", m);
    let m = m.unwrap();
    assert_eq!(m.as_str(), expected_text);
    assert_eq!(m.start(), start);
    assert_eq!(m.end(), end);
}

#[test]
fn expand() {
    let regex = common::regex("(a)(b)(?<π>c)(?P<x>d)");
    let cap = regex.captures("abcd").unwrap().expect("matched");
    assert_expansion(&cap, "$0", "abcd");
    assert_expansion(&cap, "$1", "a");
    assert_expansion(&cap, "$2", "b");
    assert_expansion(&cap, "$3", "c");
    assert_expansion(&cap, "$4", "d");
    assert_expansion(&cap, "$π", "c");
    assert_expansion(&cap, "$x", "d");
    assert_expansion(&cap, "$0π", "");
    assert_expansion(&cap, "$1π", "");
    assert_expansion(&cap, "$2π", "");
    assert_expansion(&cap, "$3π", "");
    assert_expansion(&cap, "$4π", "");
    assert_expansion(&cap, "$ππ", "");
    assert_expansion(&cap, "$xπ", "");
    assert_expansion(&cap, "${0}π", "abcdπ");
    assert_expansion(&cap, "${1}π", "aπ");
    assert_expansion(&cap, "${2}π", "bπ");
    assert_expansion(&cap, "${3}π", "cπ");
    assert_expansion(&cap, "${4}π", "dπ");
    assert_expansion(&cap, "${π}π", "cπ");
    assert_expansion(&cap, "${x}π", "dπ");
    assert_expansion(&cap, "$", "$");
    assert_expansion(&cap, "$π√", "c√");
    assert_expansion(&cap, "$x√", "d√");
    assert_expansion(&cap, "$$π", "$π");
    assert_expansion(&cap, "${π", "${π");
    assert_python_expansion(&cap, "\\0", "abcd");
    assert_python_expansion(&cap, "\\1", "a");
    assert_python_expansion(&cap, "\\2", "b");
    assert_python_expansion(&cap, "\\3", "c");
    assert_python_expansion(&cap, "\\4", "d");
    assert_python_expansion(&cap, "\\π", "\\π");
    assert_python_expansion(&cap, "\\x", "\\x");
    assert_python_expansion(&cap, "\\0π", "abcdπ");
    assert_python_expansion(&cap, "\\1π", "aπ");
    assert_python_expansion(&cap, "\\2π", "bπ");
    assert_python_expansion(&cap, "\\3π", "cπ");
    assert_python_expansion(&cap, "\\4π", "dπ");
    assert_python_expansion(&cap, "\\ππ", "\\ππ");
    assert_python_expansion(&cap, "\\xπ", "\\xπ");
    assert_python_expansion(&cap, "\\g<0>π", "abcdπ");
    assert_python_expansion(&cap, "\\g<1>π", "aπ");
    assert_python_expansion(&cap, "\\g<2>π", "bπ");
    assert_python_expansion(&cap, "\\g<3>π", "cπ");
    assert_python_expansion(&cap, "\\g<4>π", "dπ");
    assert_python_expansion(&cap, "\\g<π>π", "cπ");
    assert_python_expansion(&cap, "\\g<x>π", "dπ");
    assert_python_expansion(&cap, "\\", "\\");
    assert_python_expansion(&cap, "\\\\π", "\\π");
    assert_python_expansion(&cap, "\\g<π", "\\g<π");
}

#[cfg_attr(feature = "track_caller", track_caller)]
fn assert_expansion(cap: &Captures, replacement: &str, text: &str) {
    let mut buf = "before".to_string();
    cap.expand(replacement, &mut buf);
    assert_eq!(buf, format!("before{}", text));
}

#[cfg_attr(feature = "track_caller", track_caller)]
fn assert_python_expansion(cap: &Captures, replacement: &str, text: &str) {
    assert_eq!(Expander::python().expansion(replacement, cap), text);
}

#[test]
fn expander_escape() {
    match Expander::default().escape("hello") {
        Cow::Borrowed(s) => assert_eq!(s, "hello"),
        _ => panic!("string should be borrowed"),
    }
    assert_eq!(Expander::default().escape("a$b\\c"), "a$$b\\c");
    assert_eq!(Expander::python().escape("a$b\\c"), "a$b\\\\c");
}

#[test]
fn expander_errors() {
    let with_names = common::regex("(?<x>a)");
    let without_names = common::regex("(a)");
    let exp = Expander::default();

    macro_rules! assert_err {
        ($expr:expr, $err:pat) => {
            match $expr {
                Err($err) => {}
                x => panic!("wrong result: {:?}", x),
            }
        };
    }

    // Substitution char at end of template.
    assert_err!(exp.check("$", &with_names), Error::ParseError(_, _));

    // Substitution char not followed by a name or number.
    assert_err!(exp.check("$.", &with_names), Error::ParseError(_, _));

    // Empty delimiter pair.
    assert_err!(exp.check("${}", &with_names), Error::ParseError(_, _));

    // Unterminated delimiter pair.
    assert_err!(exp.check("${", &with_names), Error::ParseError(_, _));

    // Group 0 is always OK.
    assert!(exp.check("$0", &with_names).is_ok());
    assert!(exp.check("$0", &without_names).is_ok());

    // Can't use numbers with named groups.
    assert!(matches!(
        exp.check("$1", &with_names),
        Err(Error::CompileError(ref box_err)) if matches!(**box_err, CompileError::NamedBackrefOnly)
    ));
    assert!(matches!(
        exp.check("${1}", &with_names),
        Err(Error::CompileError(ref box_err)) if matches!(**box_err, CompileError::NamedBackrefOnly)
    ));

    // Unmatched group number.
    assert!(matches!(
        exp.check("$2", &without_names),
        Err(Error::CompileError(ref box_err)) if matches!(**box_err, CompileError::InvalidBackref(2))
    ));
    assert!(matches!(
        exp.check("${2}", &without_names),
        Err(Error::CompileError(ref box_err)) if matches!(**box_err, CompileError::InvalidBackref(2))
    ));

    // Unmatched group name.
    assert!(
        matches!(exp.check("$xx", &with_names), Err(Error::CompileError(ref box_err)) if matches!(**box_err, CompileError::InvalidGroupNameBackref(ref name) if name == "xx")),
    );
    assert!(matches!(
        exp.check("${xx}", &with_names),
        Err(Error::CompileError(ref box_err)) if matches!(**box_err, CompileError::InvalidGroupNameBackref(ref name) if name == "xx")));
}

#[test]
fn absent_repeater_hard_captures_opening_fence() {
    // Matches a markdown code fence where the opening backticks are captured in group 1,
    // then the body is matched while those backticks are absent, then the same backticks close.
    // Pattern: (`{3,})(?~\1)\1  - capture 3+ backticks, absent-repeat until those backticks,
    // then match the same backticks to close.
    let re = common::regex(r"(`{3,})(?~\1)\1");

    // Three-backtick fence
    let cap = re.captures("```code```").unwrap().unwrap();
    assert_match(cap.get(0), "```code```", 0, 10);
    assert_match(cap.get(1), "```", 0, 3);

    // Four-backtick fence - allows backticks inside
    let cap = re.captures("````has ``` inside````").unwrap().unwrap();
    assert_match(cap.get(0), "````has ``` inside````", 0, 22);
    assert_match(cap.get(1), "````", 0, 4);

    // Three backticks should match the shorter fence and stop at the first ``` occurrence
    let cap = re
        .captures("```first``` and ```second```")
        .unwrap()
        .unwrap();
    assert_match(cap.get(0), "```first```", 0, 11);
    assert_match(cap.get(1), "```", 0, 3);
}

#[test]
fn captures_with_unrestricted_group_names() {
    // Hyphenated group name
    let regex = common::regex(r"(?<foo-bar>\w+)");
    let cap = regex.captures("hello").unwrap().expect("matched");
    let m = cap.name("foo-bar").expect("group found");
    assert_eq!(m.as_str(), "hello");

    // CSS-style name
    let regex = common::regex(r"(?<data-value>\d+)");
    let cap = regex.captures("42").unwrap().expect("matched");
    let m = cap.name("data-value").expect("group found");
    assert_eq!(m.as_str(), "42");

    // Python syntax with hyphen
    let regex = common::regex(r"(?P<my-group>[a-z]+)");
    let cap = regex.captures("hello").unwrap().expect("matched");
    let m = cap.name("my-group").expect("group found");
    assert_eq!(m.as_str(), "hello");

    // Single-quote syntax with hyphen
    let regex = common::regex(r"(?'my-group'[a-z]+)");
    let cap = regex.captures("hello").unwrap().expect("matched");
    let m = cap.name("my-group").expect("group found");
    assert_eq!(m.as_str(), "hello");

    // Name with special characters
    let regex = common::regex(r"(?<#tag>\w+)");
    let cap = regex.captures("hello").unwrap().expect("matched");
    let m = cap.name("#tag").expect("group found");
    assert_eq!(m.as_str(), "hello");

    // Multiple unrestricted groups
    let regex = common::regex(r"(?<first-name>\w+)\s+(?<last-name>\w+)");
    let cap = regex.captures("John Doe").unwrap().expect("matched");
    assert_eq!(cap.name("first-name").unwrap().as_str(), "John");
    assert_eq!(cap.name("last-name").unwrap().as_str(), "Doe");
}

#[test]
fn capture_names_with_unrestricted_group_names() {
    let regex = common::regex(r"(?<foo-bar>\w+)\s+(?<baz-qux>\w+)");
    let names: Vec<_> = regex.capture_names().collect();
    assert_eq!(names[0], None); // group 0 is unnamed
                                // The order of named groups is: foo-bar=1, baz-qux=2
    assert!(names.contains(&Some("foo-bar")));
    assert!(names.contains(&Some("baz-qux")));
}
