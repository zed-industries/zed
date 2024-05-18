use language::Rope;
use std::ops::Range;

/// Search the given buffer for the given substring, ignoring any differences
/// in line indentation between the query and the buffer.
///
/// Returns a vector of ranges of byte offsets in the buffer corresponding
/// to the entire lines of the buffer.
pub fn fuzzy_search_lines(haystack: &Rope, needle: &str) -> Vec<Range<usize>> {
    let mut matches = Vec::new();
    let mut haystack_lines = haystack.chunks().lines();
    let mut haystack_line_start = 0;
    while let Some(haystack_line) = haystack_lines.next() {
        let next_haystack_line_start = haystack_line_start + haystack_line.len() + 1;
        let mut trimmed_needle_lines = needle.lines().map(|line| line.trim());
        if Some(haystack_line.trim()) == trimmed_needle_lines.next() {
            let match_start = haystack_line_start;
            let mut match_end = next_haystack_line_start;
            let matched = loop {
                match (haystack_lines.next(), trimmed_needle_lines.next()) {
                    (Some(haystack_line), Some(needle_line)) => {
                        // Haystack line differs from needle line: not a match.
                        if haystack_line.trim() == needle_line {
                            match_end = haystack_lines.offset();
                        } else {
                            break false;
                        }
                    }
                    // We exhausted the haystack but not the query: not a match.
                    (None, Some(_)) => break false,
                    // We exhausted the query: it's a match.
                    (_, None) => break true,
                }
            };

            if matched {
                matches.push(match_start..match_end)
            }

            // Advance to the next line.
            haystack_lines.seek(next_haystack_line_start);
        }

        haystack_line_start = next_haystack_line_start;
    }
    matches
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::{AppContext, Context as _};
    use language::{Buffer, OffsetRangeExt};
    use unindent::Unindent as _;
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_fuzzy_search_lines(cx: &mut AppContext) {
        let (text, expected_ranges) = marked_text_ranges(
            &r#"
            fn main() {
                if a() {
                    assert_eq!(
                        1 + 2,
                        does_not_match,
                    );
                }

                println!("hi");

                assert_eq!(
                    1 + 2,
                    3,
                ); // this last line does not match

            «    assert_eq!(
                    1 + 2,
                    3,
                );
            »

                assert_eq!(
                    "something",
                    "else",
                );

                if b {
            «        assert_eq!(
                        1 + 2,
                        3,
                    );
            »    }
            }
            "#
            .unindent(),
            false,
        );

        let buffer = cx.new_model(|cx| Buffer::local(&text, cx));
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let actual_ranges = fuzzy_search_lines(
            snapshot.as_rope(),
            &"
            assert_eq!(
                1 + 2,
                3,
            );
            "
            .unindent(),
        );
        assert_eq!(
            actual_ranges,
            expected_ranges,
            "actual: {:?}, expected: {:?}",
            actual_ranges
                .iter()
                .map(|range| range.to_point(&snapshot))
                .collect::<Vec<_>>(),
            expected_ranges
                .iter()
                .map(|range| range.to_point(&snapshot))
                .collect::<Vec<_>>()
        );

        let actual_ranges = fuzzy_search_lines(
            snapshot.as_rope(),
            &"
            assert_eq!(
                1 + 2,
                3,
                );
            "
            .unindent(),
        );
        assert_eq!(
            actual_ranges,
            expected_ranges,
            "actual: {:?}, expected: {:?}",
            actual_ranges
                .iter()
                .map(|range| range.to_point(&snapshot))
                .collect::<Vec<_>>(),
            expected_ranges
                .iter()
                .map(|range| range.to_point(&snapshot))
                .collect::<Vec<_>>()
        );
    }
}
