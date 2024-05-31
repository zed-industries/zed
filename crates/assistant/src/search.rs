use language::Rope;
use std::ops::Range;

/// Search the given buffer for the given substring, ignoring any differences
/// in line indentation between the query and the buffer.
///
/// Returns a vector of ranges of byte offsets in the buffer corresponding
/// to the entire lines of the buffer.
pub fn fuzzy_search_lines(haystack: &Rope, needle: &str) -> Option<Range<usize>> {
    const SIMILARITY_THRESHOLD: f64 = 0.8;

    let mut best_match: Option<(Range<usize>, f64)> = None; // (range, score)
    let mut haystack_lines = haystack.chunks().lines();
    let mut haystack_line_start = 0;
    while let Some(mut haystack_line) = haystack_lines.next() {
        let next_haystack_line_start = haystack_line_start + haystack_line.len() + 1;
        let mut advanced_to_next_haystack_line = false;

        let mut matched = true;
        let match_start = haystack_line_start;
        let mut match_end = next_haystack_line_start;
        let mut match_score = 0.0;
        let mut needle_lines = needle.lines().peekable();
        while let Some(needle_line) = needle_lines.next() {
            let similarity = line_similarity(haystack_line, needle_line);
            if similarity >= SIMILARITY_THRESHOLD {
                match_end = haystack_lines.offset();
                match_score += similarity;

                if needle_lines.peek().is_some() {
                    if let Some(next_haystack_line) = haystack_lines.next() {
                        advanced_to_next_haystack_line = true;
                        haystack_line = next_haystack_line;
                    } else {
                        matched = false;
                        break;
                    }
                } else {
                    break;
                }
            } else {
                matched = false;
                break;
            }
        }

        if matched
            && best_match
                .as_ref()
                .map(|(_, best_score)| match_score > *best_score)
                .unwrap_or(true)
        {
            best_match = Some((match_start..match_end, match_score));
        }

        if advanced_to_next_haystack_line {
            haystack_lines.seek(next_haystack_line_start);
        }
        haystack_line_start = next_haystack_line_start;
    }

    best_match.map(|(range, _)| range)
}

/// Calculates the similarity between two lines, ignoring leading and trailing whitespace,
/// using the Jaro-Winkler distance.
///
/// Returns a value between 0.0 and 1.0, where 1.0 indicates an exact match.
fn line_similarity(line1: &str, line2: &str) -> f64 {
    strsim::jaro_winkler(line1.trim(), line2.trim())
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::{AppContext, Context as _};
    use language::Buffer;
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

            «    assert_eq!(
                    "something",
                    "else",
                );
            »
            }
            "#
            .unindent(),
            false,
        );

        let buffer = cx.new_model(|cx| Buffer::local(&text, cx));
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let actual_range = fuzzy_search_lines(
            snapshot.as_rope(),
            &"
            assert_eq!(
                1 + 2,
                3,
            );
            "
            .unindent(),
        )
        .unwrap();
        assert_eq!(actual_range, expected_ranges[0]);

        let actual_range = fuzzy_search_lines(
            snapshot.as_rope(),
            &"
            assert_eq!(
                1 + 2,
                3,
            );
            "
            .unindent(),
        )
        .unwrap();
        assert_eq!(actual_range, expected_ranges[0]);

        let actual_range = fuzzy_search_lines(
            snapshot.as_rope(),
            &"
            asst_eq!(
                \"something\",
                \"els\"
            )
            "
            .unindent(),
        )
        .unwrap();
        assert_eq!(actual_range, expected_ranges[1]);

        let actual_range = fuzzy_search_lines(
            snapshot.as_rope(),
            &"
            assert_eq!(
                2 + 1,
                3,
            );
            "
            .unindent(),
        );
        assert_eq!(actual_range, None);
    }
}
