use language::BufferSnapshot;
use project::search::SearchQuery;
use std::ops::Range;
use util::ResultExt as _;

/// Search the given buffer for the given substring, ignoring any differences
/// in indentation between the query and the buffer.
///
/// Although indentation differences are allowed, the relative indentation
/// between lines must match.
///
/// Returns a vector of ranges of byte offsets in the buffer corresponding
/// to the entire lines of the buffer.
pub async fn search_buffer_ignoring_indentation(
    buffer: &BufferSnapshot,
    query: &str,
) -> Vec<Range<usize>> {
    let mut result = Vec::new();
    let query_lines = query.trim_end().split('\n');
    let first_query_line = query_lines.clone().next().unwrap();
    let Some(first_query_line) = SearchQuery::text(
        first_query_line.trim_start(),
        false,
        true,
        false,
        Vec::new(),
        Vec::new(),
    )
    .log_err() else {
        return result;
    };

    let mut buffer_lines = buffer.as_rope().chunks().lines();
    'matches: for match_range in first_query_line.search(&buffer, None).await {
        let match_start_point = buffer.offset_to_point(match_range.start);
        let start_offset = match_range.start - match_start_point.column as usize;
        buffer_lines.seek(start_offset);

        let mut first_line_indent_difference = None;
        for query_line in query_lines.clone() {
            let Some(buffer_line) = buffer_lines.next() else {
                continue 'matches;
            };

            let trimmed_query_line = query_line.trim_start();
            let trimmed_buffer_line = buffer_line.trim_start();
            if trimmed_query_line != trimmed_buffer_line {
                continue 'matches;
            }

            let query_indent = query_line.len() - trimmed_query_line.len();
            let buffer_indent = buffer_line.len() - trimmed_buffer_line.len();
            let indent_difference = query_indent as i32 - buffer_indent as i32;
            if let Some(first_indent_difference) = first_line_indent_difference {
                if indent_difference != first_indent_difference {
                    continue 'matches;
                }
            } else {
                first_line_indent_difference = Some(indent_difference);
            }
        }

        result.push(start_offset..buffer_lines.offset());
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::{Context as _, TestAppContext};
    use language::Buffer;
    use unindent::Unindent as _;
    use util::test::marked_text_ranges;

    #[gpui::test]
    async fn test_search_ignoring_indentation(cx: &mut TestAppContext) {
        let (text, ranges) = marked_text_ranges(
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

        assert_eq!(
            search_buffer_ignoring_indentation(
                &snapshot,
                &"
                assert_eq!(
                    1 + 2,
                    3,
                );
                "
                .unindent()
            )
            .await,
            ranges,
        );

        // Relative indentation must match.
        assert_eq!(
            search_buffer_ignoring_indentation(
                &snapshot,
                &"
                assert_eq!(
                    1 + 2,
                    3,
                    );
                "
                .unindent()
            )
            .await,
            Vec::<Range<usize>>::new(),
        );
    }
}
