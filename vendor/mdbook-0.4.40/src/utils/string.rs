use once_cell::sync::Lazy;
use regex::Regex;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;

/// Take a range of lines from a string.
pub fn take_lines<R: RangeBounds<usize>>(s: &str, range: R) -> String {
    let start = match range.start_bound() {
        Excluded(&n) => n + 1,
        Included(&n) => n,
        Unbounded => 0,
    };
    let lines = s.lines().skip(start);
    match range.end_bound() {
        Excluded(end) => lines
            .take(end.saturating_sub(start))
            .collect::<Vec<_>>()
            .join("\n"),
        Included(end) => lines
            .take((end + 1).saturating_sub(start))
            .collect::<Vec<_>>()
            .join("\n"),
        Unbounded => lines.collect::<Vec<_>>().join("\n"),
    }
}

static ANCHOR_START: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"ANCHOR:\s*(?P<anchor_name>[\w_-]+)").unwrap());
static ANCHOR_END: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"ANCHOR_END:\s*(?P<anchor_name>[\w_-]+)").unwrap());

/// Take anchored lines from a string.
/// Lines containing anchor are ignored.
pub fn take_anchored_lines(s: &str, anchor: &str) -> String {
    let mut retained = Vec::<&str>::new();
    let mut anchor_found = false;

    for l in s.lines() {
        if anchor_found {
            match ANCHOR_END.captures(l) {
                Some(cap) => {
                    if &cap["anchor_name"] == anchor {
                        break;
                    }
                }
                None => {
                    if !ANCHOR_START.is_match(l) {
                        retained.push(l);
                    }
                }
            }
        } else if let Some(cap) = ANCHOR_START.captures(l) {
            if &cap["anchor_name"] == anchor {
                anchor_found = true;
            }
        }
    }

    retained.join("\n")
}

/// Keep lines contained within the range specified as-is.
/// For any lines not in the range, include them but use `#` at the beginning. This will hide the
/// lines from initial display but include them when expanding the code snippet or testing with
/// rustdoc.
pub fn take_rustdoc_include_lines<R: RangeBounds<usize>>(s: &str, range: R) -> String {
    let mut output = String::with_capacity(s.len());

    for (index, line) in s.lines().enumerate() {
        if !range.contains(&index) {
            output.push_str("# ");
        }
        output.push_str(line);
        output.push('\n');
    }
    output.pop();
    output
}

/// Keep lines between the anchor comments specified as-is.
/// For any lines not between the anchors, include them but use `#` at the beginning. This will
/// hide the lines from initial display but include them when expanding the code snippet or testing
/// with rustdoc.
pub fn take_rustdoc_include_anchored_lines(s: &str, anchor: &str) -> String {
    let mut output = String::with_capacity(s.len());
    let mut within_anchored_section = false;

    for l in s.lines() {
        if within_anchored_section {
            match ANCHOR_END.captures(l) {
                Some(cap) => {
                    if &cap["anchor_name"] == anchor {
                        within_anchored_section = false;
                    }
                }
                None => {
                    if !ANCHOR_START.is_match(l) {
                        output.push_str(l);
                        output.push('\n');
                    }
                }
            }
        } else if let Some(cap) = ANCHOR_START.captures(l) {
            if &cap["anchor_name"] == anchor {
                within_anchored_section = true;
            }
        } else if !ANCHOR_END.is_match(l) {
            output.push_str("# ");
            output.push_str(l);
            output.push('\n');
        }
    }

    output.pop();
    output
}

#[cfg(test)]
mod tests {
    use super::{
        take_anchored_lines, take_lines, take_rustdoc_include_anchored_lines,
        take_rustdoc_include_lines,
    };

    #[test]
    #[allow(clippy::reversed_empty_ranges)] // Intentionally checking that those are correctly handled
    fn take_lines_test() {
        let s = "Lorem\nipsum\ndolor\nsit\namet";
        assert_eq!(take_lines(s, 1..3), "ipsum\ndolor");
        assert_eq!(take_lines(s, 3..), "sit\namet");
        assert_eq!(take_lines(s, ..3), "Lorem\nipsum\ndolor");
        assert_eq!(take_lines(s, ..), s);
        // corner cases
        assert_eq!(take_lines(s, 4..3), "");
        assert_eq!(take_lines(s, ..100), s);
    }

    #[test]
    fn take_anchored_lines_test() {
        let s = "Lorem\nipsum\ndolor\nsit\namet";
        assert_eq!(take_anchored_lines(s, "test"), "");

        let s = "Lorem\nipsum\ndolor\nANCHOR_END: test\nsit\namet";
        assert_eq!(take_anchored_lines(s, "test"), "");

        let s = "Lorem\nipsum\nANCHOR: test\ndolor\nsit\namet";
        assert_eq!(take_anchored_lines(s, "test"), "dolor\nsit\namet");
        assert_eq!(take_anchored_lines(s, "something"), "");

        let s = "Lorem\nipsum\nANCHOR: test\ndolor\nsit\namet\nANCHOR_END: test\nlorem\nipsum";
        assert_eq!(take_anchored_lines(s, "test"), "dolor\nsit\namet");
        assert_eq!(take_anchored_lines(s, "something"), "");

        let s = "Lorem\nANCHOR: test\nipsum\nANCHOR: test\ndolor\nsit\namet\nANCHOR_END: test\nlorem\nipsum";
        assert_eq!(take_anchored_lines(s, "test"), "ipsum\ndolor\nsit\namet");
        assert_eq!(take_anchored_lines(s, "something"), "");

        let s = "Lorem\nANCHOR:    test2\nipsum\nANCHOR: test\ndolor\nsit\namet\nANCHOR_END: test\nlorem\nANCHOR_END:test2\nipsum";
        assert_eq!(
            take_anchored_lines(s, "test2"),
            "ipsum\ndolor\nsit\namet\nlorem"
        );
        assert_eq!(take_anchored_lines(s, "test"), "dolor\nsit\namet");
        assert_eq!(take_anchored_lines(s, "something"), "");
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)] // Intentionally checking that those are correctly handled
    fn take_rustdoc_include_lines_test() {
        let s = "Lorem\nipsum\ndolor\nsit\namet";
        assert_eq!(
            take_rustdoc_include_lines(s, 1..3),
            "# Lorem\nipsum\ndolor\n# sit\n# amet"
        );
        assert_eq!(
            take_rustdoc_include_lines(s, 3..),
            "# Lorem\n# ipsum\n# dolor\nsit\namet"
        );
        assert_eq!(
            take_rustdoc_include_lines(s, ..3),
            "Lorem\nipsum\ndolor\n# sit\n# amet"
        );
        assert_eq!(take_rustdoc_include_lines(s, ..), s);
        // corner cases
        assert_eq!(
            take_rustdoc_include_lines(s, 4..3),
            "# Lorem\n# ipsum\n# dolor\n# sit\n# amet"
        );
        assert_eq!(take_rustdoc_include_lines(s, ..100), s);
    }

    #[test]
    fn take_rustdoc_include_anchored_lines_test() {
        let s = "Lorem\nipsum\ndolor\nsit\namet";
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "test"),
            "# Lorem\n# ipsum\n# dolor\n# sit\n# amet"
        );

        let s = "Lorem\nipsum\ndolor\nANCHOR_END: test\nsit\namet";
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "test"),
            "# Lorem\n# ipsum\n# dolor\n# sit\n# amet"
        );

        let s = "Lorem\nipsum\nANCHOR: test\ndolor\nsit\namet";
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "test"),
            "# Lorem\n# ipsum\ndolor\nsit\namet"
        );
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "something"),
            "# Lorem\n# ipsum\n# dolor\n# sit\n# amet"
        );

        let s = "Lorem\nipsum\nANCHOR: test\ndolor\nsit\namet\nANCHOR_END: test\nlorem\nipsum";
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "test"),
            "# Lorem\n# ipsum\ndolor\nsit\namet\n# lorem\n# ipsum"
        );
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "something"),
            "# Lorem\n# ipsum\n# dolor\n# sit\n# amet\n# lorem\n# ipsum"
        );

        let s = "Lorem\nANCHOR: test\nipsum\nANCHOR: test\ndolor\nsit\namet\nANCHOR_END: test\nlorem\nipsum";
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "test"),
            "# Lorem\nipsum\ndolor\nsit\namet\n# lorem\n# ipsum"
        );
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "something"),
            "# Lorem\n# ipsum\n# dolor\n# sit\n# amet\n# lorem\n# ipsum"
        );

        let s = "Lorem\nANCHOR:    test2\nipsum\nANCHOR: test\ndolor\nsit\namet\nANCHOR_END: test\nlorem\nANCHOR_END:test2\nipsum";
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "test2"),
            "# Lorem\nipsum\ndolor\nsit\namet\nlorem\n# ipsum"
        );
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "test"),
            "# Lorem\n# ipsum\ndolor\nsit\namet\n# lorem\n# ipsum"
        );
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "something"),
            "# Lorem\n# ipsum\n# dolor\n# sit\n# amet\n# lorem\n# ipsum"
        );

        let s = "Lorem\nANCHOR: test\nipsum\nANCHOR_END: test\ndolor\nANCHOR: test\nsit\nANCHOR_END: test\namet";
        assert_eq!(
            take_rustdoc_include_anchored_lines(s, "test"),
            "# Lorem\nipsum\n# dolor\nsit\n# amet"
        );
    }
}
