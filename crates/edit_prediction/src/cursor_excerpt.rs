use language::{BufferSnapshot, Point};
use std::ops::Range;

pub fn editable_and_context_ranges_for_cursor_position(
    position: Point,
    snapshot: &BufferSnapshot,
    editable_region_token_limit: usize,
    context_token_limit: usize,
) -> (Range<Point>, Range<Point>) {
    let editable_range = compute_editable_range(snapshot, position, editable_region_token_limit);

    let context_range = expand_context_syntactically_then_linewise(
        snapshot,
        editable_range.clone(),
        context_token_limit,
    );

    (editable_range, context_range)
}

/// Computes the editable range using a three-phase approach:
/// 1. Expand symmetrically from cursor (75% of budget)
/// 2. Expand to syntax boundaries
/// 3. Continue line-wise in the least-expanded direction
fn compute_editable_range(
    snapshot: &BufferSnapshot,
    cursor: Point,
    token_limit: usize,
) -> Range<Point> {
    // Phase 1: Expand symmetrically from cursor using 75% of budget.
    let initial_budget = (token_limit * 3) / 4;
    let (mut start_row, mut end_row, mut remaining_tokens) =
        expand_symmetric_from_cursor(snapshot, cursor.row, initial_budget);

    // Add remaining budget from phase 1.
    remaining_tokens += token_limit.saturating_sub(initial_budget);

    let original_start = start_row;
    let original_end = end_row;

    // Phase 2: Expand to syntax boundaries that fit within budget.
    for (boundary_start, boundary_end) in containing_syntax_boundaries(snapshot, start_row, end_row)
    {
        let tokens_for_start = if boundary_start < start_row {
            estimate_tokens_for_rows(snapshot, boundary_start, start_row)
        } else {
            0
        };
        let tokens_for_end = if boundary_end > end_row {
            estimate_tokens_for_rows(snapshot, end_row + 1, boundary_end + 1)
        } else {
            0
        };

        let total_needed = tokens_for_start + tokens_for_end;

        if total_needed <= remaining_tokens {
            if boundary_start < start_row {
                start_row = boundary_start;
            }
            if boundary_end > end_row {
                end_row = boundary_end;
            }
            remaining_tokens = remaining_tokens.saturating_sub(total_needed);
        } else {
            break;
        }
    }

    // Phase 3: Continue line-wise in the direction we expanded least during syntax phase.
    let expanded_up = original_start.saturating_sub(start_row);
    let expanded_down = end_row.saturating_sub(original_end);

    (start_row, end_row, _) = expand_linewise_biased(
        snapshot,
        start_row,
        end_row,
        remaining_tokens,
        expanded_up <= expanded_down, // prefer_up if we expanded less upward
    );

    let start = Point::new(start_row, 0);
    let end = Point::new(end_row, snapshot.line_len(end_row));
    start..end
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

    // Account for the cursor's line.
    let cursor_line_tokens = line_token_count(snapshot, cursor_row);
    token_budget = token_budget.saturating_sub(cursor_line_tokens);

    loop {
        let can_expand_up = start_row > 0;
        let can_expand_down = end_row < snapshot.max_point().row;

        if token_budget == 0 || (!can_expand_up && !can_expand_down) {
            break;
        }

        // Expand down first (slight forward bias for edit prediction).
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

        // Then expand up.
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

/// Expands line-wise with a bias toward one direction.
/// Returns (start_row, end_row, remaining_tokens).
fn expand_linewise_biased(
    snapshot: &BufferSnapshot,
    mut start_row: u32,
    mut end_row: u32,
    mut remaining_tokens: usize,
    prefer_up: bool,
) -> (u32, u32, usize) {
    loop {
        let can_expand_up = start_row > 0;
        let can_expand_down = end_row < snapshot.max_point().row;

        if remaining_tokens == 0 || (!can_expand_up && !can_expand_down) {
            break;
        }

        let mut expanded = false;

        // Try preferred direction first.
        if prefer_up {
            if can_expand_up {
                let next_row = start_row - 1;
                let line_tokens = line_token_count(snapshot, next_row);
                if line_tokens <= remaining_tokens {
                    start_row = next_row;
                    remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
                    expanded = true;
                }
            }
            if can_expand_down && remaining_tokens > 0 {
                let next_row = end_row + 1;
                let line_tokens = line_token_count(snapshot, next_row);
                if line_tokens <= remaining_tokens {
                    end_row = next_row;
                    remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
                    expanded = true;
                }
            }
        } else {
            if can_expand_down {
                let next_row = end_row + 1;
                let line_tokens = line_token_count(snapshot, next_row);
                if line_tokens <= remaining_tokens {
                    end_row = next_row;
                    remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
                    expanded = true;
                }
            }
            if can_expand_up && remaining_tokens > 0 {
                let next_row = start_row - 1;
                let line_tokens = line_token_count(snapshot, next_row);
                if line_tokens <= remaining_tokens {
                    start_row = next_row;
                    remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
                    expanded = true;
                }
            }
        }

        if !expanded {
            break;
        }
    }

    (start_row, end_row, remaining_tokens)
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

/// Estimates token count for rows in range [start_row, end_row).
fn estimate_tokens_for_rows(snapshot: &BufferSnapshot, start_row: u32, end_row: u32) -> usize {
    let mut tokens = 0;
    for row in start_row..end_row {
        tokens += line_token_count(snapshot, row);
    }
    tokens
}

/// Returns an iterator of (start_row, end_row) for successively larger syntax nodes
/// containing the given row range. Smallest containing node first.
fn containing_syntax_boundaries(
    snapshot: &BufferSnapshot,
    start_row: u32,
    end_row: u32,
) -> impl Iterator<Item = (u32, u32)> {
    let range = Point::new(start_row, 0)..Point::new(end_row, snapshot.line_len(end_row));
    let mut current = snapshot.syntax_ancestor(range);
    let mut last_rows: Option<(u32, u32)> = None;

    std::iter::from_fn(move || {
        while let Some(node) = current.take() {
            let node_start_row = node.start_position().row as u32;
            let node_end_row = node.end_position().row as u32;
            let rows = (node_start_row, node_end_row);

            current = node.parent();

            // Skip nodes that don't extend beyond our range.
            if node_start_row >= start_row && node_end_row <= end_row {
                continue;
            }

            // Skip if same as last returned (some nodes have same span).
            if last_rows == Some(rows) {
                continue;
            }

            last_rows = Some(rows);
            return Some(rows);
        }
        None
    })
}

/// Expands context by first trying to reach syntax boundaries,
/// then expanding line-wise only if no syntax expansion occurred.
fn expand_context_syntactically_then_linewise(
    snapshot: &BufferSnapshot,
    editable_range: Range<Point>,
    context_token_limit: usize,
) -> Range<Point> {
    let mut start_row = editable_range.start.row;
    let mut end_row = editable_range.end.row;
    let mut remaining_tokens = context_token_limit;
    let mut did_syntax_expand = false;

    // Phase 1: Try to expand to containing syntax boundaries, picking the largest that fits.
    for (boundary_start, boundary_end) in containing_syntax_boundaries(snapshot, start_row, end_row)
    {
        let tokens_for_start = if boundary_start < start_row {
            estimate_tokens_for_rows(snapshot, boundary_start, start_row)
        } else {
            0
        };
        let tokens_for_end = if boundary_end > end_row {
            estimate_tokens_for_rows(snapshot, end_row + 1, boundary_end + 1)
        } else {
            0
        };

        let total_needed = tokens_for_start + tokens_for_end;

        if total_needed <= remaining_tokens {
            if boundary_start < start_row {
                start_row = boundary_start;
            }
            if boundary_end > end_row {
                end_row = boundary_end;
            }
            remaining_tokens = remaining_tokens.saturating_sub(total_needed);
            did_syntax_expand = true;
        } else {
            break;
        }
    }

    // Phase 2: Only expand line-wise if no syntax expansion occurred.
    if !did_syntax_expand {
        (start_row, end_row, _) =
            expand_linewise_biased(snapshot, start_row, end_row, remaining_tokens, true);
    }

    let start = Point::new(start_row, 0);
    let end = Point::new(end_row, snapshot.line_len(end_row));
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{App, AppContext};
    use indoc::indoc;
    use language::{Buffer, rust_lang};
    use util::test::{TextRangeMarker, marked_text_ranges_by};

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
                name: "cursor near end of function - expands to syntax boundaries",
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
                // 18 tokens - expands symmetrically then to syntax boundaries
                editable_token_limit: 18,
                context_token_limit: 35,
            },
            TestCase {
                name: "cursor at function start - expands to syntax boundaries",
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
                // 25 tokens - expands symmetrically then to syntax boundaries
                editable_token_limit: 25,
                context_token_limit: 50,
            },
            TestCase {
                name: "tiny budget - just lines around cursor",
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
                // 12 tokens (~36 bytes) = just the cursor line with tiny budget
                editable_token_limit: 12,
                context_token_limit: 24,
            },
            TestCase {
                name: "small function fits entirely",
                marked_text: indoc! {r#"
                    [«fn foo() {
                        let x = 1;ˇ
                        let y = 2;
                    }»]
                "#},
                // Plenty of budget for this small function
                editable_token_limit: 30,
                context_token_limit: 60,
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
                // Small editable, larger context
                editable_token_limit: 25,
                context_token_limit: 45,
            },
            // Tests for syntax-aware editable and context expansion
            TestCase {
                name: "cursor in first if-statement - expands to syntax boundaries",
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
                // 35 tokens allows expansion to include function header and first two if blocks
                editable_token_limit: 35,
                // 60 tokens allows context to include the whole file
                context_token_limit: 60,
            },
            TestCase {
                name: "cursor in middle if-statement - expands to syntax boundaries",
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
                // 40 tokens allows expansion to surrounding if blocks
                editable_token_limit: 40,
                // 60 tokens allows context to include the whole file
                context_token_limit: 60,
            },
            TestCase {
                name: "cursor near bottom of long function - editable expands toward syntax, context reaches function",
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
                // 40 tokens for editable - allows several lines plus syntax expansion
                editable_token_limit: 40,
                // 55 tokens - enough for function but not whole file
                context_token_limit: 55,
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
            assert_eq!(expected_editable.len(), 1);
            assert_eq!(expected_context.len(), 1);

            cx.new(|cx| {
                let text = text.trim_end_matches('\n');
                let buffer = Buffer::local(text, cx).with_language(rust_lang(), cx);
                let snapshot = buffer.snapshot();

                let cursor_offset = cursor_ranges[0].start;
                let cursor_point = snapshot.offset_to_point(cursor_offset);
                let expected_editable_start = snapshot.offset_to_point(expected_editable[0].start);
                let expected_editable_end = snapshot.offset_to_point(expected_editable[0].end);
                let expected_context_start = snapshot.offset_to_point(expected_context[0].start);
                let expected_context_end = snapshot.offset_to_point(expected_context[0].end);

                let (actual_editable, actual_context) =
                    editable_and_context_ranges_for_cursor_position(
                        cursor_point,
                        &snapshot,
                        test_case.editable_token_limit,
                        test_case.context_token_limit,
                    );

                let range_text = |start: Point, end: Point| -> String {
                    snapshot.text_for_range(start..end).collect()
                };

                let editable_match = actual_editable.start == expected_editable_start
                    && actual_editable.end == expected_editable_end;
                let context_match = actual_context.start == expected_context_start
                    && actual_context.end == expected_context_end;

                if !editable_match || !context_match {
                    println!("\n=== FAILED: {} ===", test_case.name);
                    if !editable_match {
                        println!(
                            "\nExpected editable ({:?}..{:?}):",
                            expected_editable_start, expected_editable_end
                        );
                        println!(
                            "---\n{}---",
                            range_text(expected_editable_start, expected_editable_end)
                        );
                        println!(
                            "\nActual editable ({:?}..{:?}):",
                            actual_editable.start, actual_editable.end
                        );
                        println!(
                            "---\n{}---",
                            range_text(actual_editable.start, actual_editable.end)
                        );
                    }
                    if !context_match {
                        println!(
                            "\nExpected context ({:?}..{:?}):",
                            expected_context_start, expected_context_end
                        );
                        println!(
                            "---\n{}---",
                            range_text(expected_context_start, expected_context_end)
                        );
                        println!(
                            "\nActual context ({:?}..{:?}):",
                            actual_context.start, actual_context.end
                        );
                        println!(
                            "---\n{}---",
                            range_text(actual_context.start, actual_context.end)
                        );
                    }
                    panic!("Test '{}' failed - see output above", test_case.name);
                }

                buffer
            });
        }
    }
}
