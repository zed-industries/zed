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
