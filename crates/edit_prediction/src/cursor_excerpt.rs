use language::{BufferSnapshot, Point, ToPoint as _};
use std::ops::Range;
use text::OffsetRangeExt as _;

const CURSOR_EXCERPT_TOKEN_BUDGET: usize = 8192;

/// Computes a cursor excerpt as the largest linewise symmetric region around
/// the cursor that fits within an 8192-token budget. Returns the point range,
/// byte offset range, and the cursor offset relative to the excerpt start.
pub fn compute_cursor_excerpt(
    snapshot: &BufferSnapshot,
    cursor_offset: usize,
) -> (Range<Point>, Range<usize>, usize) {
    let cursor_point = cursor_offset.to_point(snapshot);
    let cursor_row = cursor_point.row;
    let (start_row, end_row, _) =
        expand_symmetric_from_cursor(snapshot, cursor_row, CURSOR_EXCERPT_TOKEN_BUDGET);

    let excerpt_range = Point::new(start_row, 0)..Point::new(end_row, snapshot.line_len(end_row));
    let excerpt_offset_range = excerpt_range.to_offset(snapshot);
    let cursor_offset_in_excerpt = cursor_offset - excerpt_offset_range.start;

    (
        excerpt_range,
        excerpt_offset_range,
        cursor_offset_in_excerpt,
    )
}

/// Expands symmetrically from cursor, one line at a time, alternating down then up.
/// Returns (start_row, end_row, remaining_tokens).
fn expand_symmetric_from_cursor(
    snapshot: &BufferSnapshot,
    cursor_row: u32,
    mut token_budget: usize,
) -> (u32, u32, usize) {
    let mut start_row = cursor_row;
    let mut end_row = cursor_row;

    let cursor_line_tokens = line_token_count(snapshot, cursor_row);
    token_budget = token_budget.saturating_sub(cursor_line_tokens);

    loop {
        let can_expand_up = start_row > 0;
        let can_expand_down = end_row < snapshot.max_point().row;

        if token_budget == 0 || (!can_expand_up && !can_expand_down) {
            break;
        }

        if can_expand_down {
            let next_row = end_row + 1;
            let line_tokens = line_token_count(snapshot, next_row);
            if line_tokens <= token_budget {
                end_row = next_row;
                token_budget = token_budget.saturating_sub(line_tokens);
            } else {
                break;
            }
        }

        if can_expand_up && token_budget > 0 {
            let next_row = start_row - 1;
            let line_tokens = line_token_count(snapshot, next_row);
            if line_tokens <= token_budget {
                start_row = next_row;
                token_budget = token_budget.saturating_sub(line_tokens);
            } else {
                break;
            }
        }
    }

    (start_row, end_row, token_budget)
}

/// Typical number of string bytes per token for the purposes of limiting model input. This is
/// intentionally low to err on the side of underestimating limits.
pub(crate) const BYTES_PER_TOKEN_GUESS: usize = 3;

pub fn guess_token_count(bytes: usize) -> usize {
    bytes / BYTES_PER_TOKEN_GUESS
}

fn line_token_count(snapshot: &BufferSnapshot, row: u32) -> usize {
    guess_token_count(snapshot.line_len(row) as usize).max(1)
}

/// Computes the byte offset ranges of all syntax nodes containing the cursor,
/// ordered from innermost to outermost. The offsets are relative to
/// `excerpt_offset_range.start`.
pub fn compute_syntax_ranges(
    snapshot: &BufferSnapshot,
    cursor_offset: usize,
    excerpt_offset_range: &Range<usize>,
) -> Vec<Range<usize>> {
    let cursor_point = cursor_offset.to_point(snapshot);
    let range = cursor_point..cursor_point;
    let mut current = snapshot.syntax_ancestor(range);
    let mut ranges = Vec::new();
    let mut last_range: Option<(usize, usize)> = None;

    while let Some(node) = current.take() {
        let node_start = node.start_byte();
        let node_end = node.end_byte();
        let key = (node_start, node_end);

        current = node.parent();

        if last_range == Some(key) {
            continue;
        }
        last_range = Some(key);

        let start = node_start.saturating_sub(excerpt_offset_range.start);
        let end = node_end
            .min(excerpt_offset_range.end)
            .saturating_sub(excerpt_offset_range.start);
        ranges.push(start..end);
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{App, AppContext as _};
    use indoc::indoc;
    use language::{Buffer, rust_lang};
    use util::test::{TextRangeMarker, marked_text_ranges_by};
    use zeta_prompt::compute_editable_and_context_ranges;

    struct TestCase {
        name: &'static str,
        marked_text: &'static str,
        editable_token_limit: usize,
        context_token_limit: usize,
    }

    #[gpui::test]
    fn test_editable_and_context_ranges(cx: &mut App) {
        // Markers:
        // ˇ = cursor position
        // « » = expected editable range
        // [ ] = expected context range
        let test_cases = vec![
            TestCase {
                name: "small function fits entirely in editable and context",
                marked_text: indoc! {r#"
                    [«fn foo() {
                        let x = 1;ˇ
                        let y = 2;
                    }»]
                "#},
                editable_token_limit: 30,
                context_token_limit: 60,
            },
            TestCase {
                name: "cursor near end of function - editable expands to syntax boundaries",
                marked_text: indoc! {r#"
                    [fn first() {
                        let a = 1;
                        let b = 2;
                    }

                    fn foo() {
                    «    let x = 1;
                        let y = 2;
                        println!("{}", x + y);ˇ
                    }»]
                "#},
                editable_token_limit: 18,
                context_token_limit: 35,
            },
            TestCase {
                name: "cursor at function start - editable expands to syntax boundaries",
                marked_text: indoc! {r#"
                    [fn before() {
                    «    let a = 1;
                    }

                    fn foo() {ˇ
                        let x = 1;
                        let y = 2;
                        let z = 3;
                    }
                    »
                    fn after() {
                        let b = 2;
                    }]
                "#},
                editable_token_limit: 25,
                context_token_limit: 50,
            },
            TestCase {
                name: "tiny budget - just lines around cursor, no syntax expansion",
                marked_text: indoc! {r#"
                    fn outer() {
                    [    let line1 = 1;
                        let line2 = 2;
                    «    let line3 = 3;
                        let line4 = 4;ˇ»
                        let line5 = 5;
                        let line6 = 6;]
                        let line7 = 7;
                    }
                "#},
                editable_token_limit: 12,
                context_token_limit: 24,
            },
            TestCase {
                name: "context extends beyond editable",
                marked_text: indoc! {r#"
                    [fn first() { let a = 1; }
                    «fn second() { let b = 2; }
                    fn third() { let c = 3; }ˇ
                    fn fourth() { let d = 4; }»
                    fn fifth() { let e = 5; }]
                "#},
                editable_token_limit: 25,
                context_token_limit: 45,
            },
            TestCase {
                name: "cursor in first if-block - editable expands to syntax boundaries",
                marked_text: indoc! {r#"
                    [«fn before() { }

                    fn process() {
                        if condition1 {
                            let a = 1;ˇ
                            let b = 2;
                        }
                        if condition2 {»
                            let c = 3;
                            let d = 4;
                        }
                        if condition3 {
                            let e = 5;
                            let f = 6;
                        }
                    }

                    fn after() { }]
                "#},
                editable_token_limit: 35,
                context_token_limit: 60,
            },
            TestCase {
                name: "cursor in middle if-block - editable spans surrounding blocks",
                marked_text: indoc! {r#"
                    [fn before() { }

                    fn process() {
                        if condition1 {
                            let a = 1;
                    «        let b = 2;
                        }
                        if condition2 {
                            let c = 3;ˇ
                            let d = 4;
                        }
                        if condition3 {
                            let e = 5;»
                            let f = 6;
                        }
                    }

                    fn after() { }]
                "#},
                editable_token_limit: 40,
                context_token_limit: 60,
            },
            TestCase {
                name: "cursor near bottom of long function - context reaches function boundary",
                marked_text: indoc! {r#"
                    [fn other() { }

                    fn long_function() {
                        let line1 = 1;
                        let line2 = 2;
                        let line3 = 3;
                        let line4 = 4;
                        let line5 = 5;
                        let line6 = 6;
                    «    let line7 = 7;
                        let line8 = 8;
                        let line9 = 9;
                        let line10 = 10;ˇ
                        let line11 = 11;
                    }

                    fn another() { }»]
                "#},
                editable_token_limit: 40,
                context_token_limit: 55,
            },
            TestCase {
                name: "zero context budget - context equals editable",
                marked_text: indoc! {r#"
                    fn before() {
                        let p = 1;
                        let q = 2;
                    [«}

                    fn foo() {
                        let x = 1;ˇ
                        let y = 2;
                    }
                    »]
                    fn after() {
                        let r = 3;
                        let s = 4;
                    }
                "#},
                editable_token_limit: 15,
                context_token_limit: 0,
            },
        ];

        for test_case in test_cases {
            let cursor_marker: TextRangeMarker = 'ˇ'.into();
            let editable_marker: TextRangeMarker = ('«', '»').into();
            let context_marker: TextRangeMarker = ('[', ']').into();

            let (text, mut ranges) = marked_text_ranges_by(
                test_case.marked_text,
                vec![
                    cursor_marker.clone(),
                    editable_marker.clone(),
                    context_marker.clone(),
                ],
            );

            let cursor_ranges = ranges.remove(&cursor_marker).unwrap_or_default();
            let expected_editable = ranges.remove(&editable_marker).unwrap_or_default();
            let expected_context = ranges.remove(&context_marker).unwrap_or_default();
            assert_eq!(expected_editable.len(), 1, "{}", test_case.name);
            assert_eq!(expected_context.len(), 1, "{}", test_case.name);

            cx.new(|cx: &mut gpui::Context<Buffer>| {
                let text = text.trim_end_matches('\n');
                let buffer = Buffer::local(text, cx).with_language(rust_lang(), cx);
                let snapshot = buffer.snapshot();

                let cursor_offset = cursor_ranges[0].start;

                let (_, excerpt_offset_range, cursor_offset_in_excerpt) =
                    compute_cursor_excerpt(&snapshot, cursor_offset);
                let excerpt_text: String = snapshot
                    .text_for_range(excerpt_offset_range.clone())
                    .collect();
                let syntax_ranges =
                    compute_syntax_ranges(&snapshot, cursor_offset, &excerpt_offset_range);

                let (actual_editable, actual_context) = compute_editable_and_context_ranges(
                    &excerpt_text,
                    cursor_offset_in_excerpt,
                    &syntax_ranges,
                    test_case.editable_token_limit,
                    test_case.context_token_limit,
                );

                let to_buffer_range = |range: Range<usize>| -> Range<usize> {
                    (excerpt_offset_range.start + range.start)
                        ..(excerpt_offset_range.start + range.end)
                };

                let actual_editable = to_buffer_range(actual_editable);
                let actual_context = to_buffer_range(actual_context);

                let expected_editable_range = expected_editable[0].clone();
                let expected_context_range = expected_context[0].clone();

                let editable_match = actual_editable == expected_editable_range;
                let context_match = actual_context == expected_context_range;

                if !editable_match || !context_match {
                    let range_text = |range: &Range<usize>| {
                        snapshot.text_for_range(range.clone()).collect::<String>()
                    };

                    println!("\n=== FAILED: {} ===", test_case.name);
                    if !editable_match {
                        println!("\nExpected editable ({:?}):", expected_editable_range);
                        println!("---\n{}---", range_text(&expected_editable_range));
                        println!("\nActual editable ({:?}):", actual_editable);
                        println!("---\n{}---", range_text(&actual_editable));
                    }
                    if !context_match {
                        println!("\nExpected context ({:?}):", expected_context_range);
                        println!("---\n{}---", range_text(&expected_context_range));
                        println!("\nActual context ({:?}):", actual_context);
                        println!("---\n{}---", range_text(&actual_context));
                    }
                    panic!("Test '{}' failed - see output above", test_case.name);
                }

                buffer
            });
        }
    }
}
