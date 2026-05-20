use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::estimate_tokens;

/// Pre-computed byte offset ranges within `cursor_excerpt` for different
/// editable and context token budgets. Allows the server to select the
/// appropriate ranges for whichever model it uses.
#[derive(Clone, Debug, Default, PartialEq, Hash, Serialize, Deserialize)]
pub struct ExcerptRanges {
    /// Editable region computed with a 150-token budget.
    pub editable_150: Range<usize>,
    /// Editable region computed with a 180-token budget.
    pub editable_180: Range<usize>,
    /// Editable region computed with a 350-token budget.
    pub editable_350: Range<usize>,
    /// Editable region computed with a 350-token budget.
    pub editable_512: Option<Range<usize>>,
    /// Context boundary when using editable_150 with 350 tokens of additional context.
    pub editable_150_context_350: Range<usize>,
    /// Context boundary when using editable_180 with 350 tokens of additional context.
    pub editable_180_context_350: Range<usize>,
    /// Context boundary when using editable_350 with 150 tokens of additional context.
    pub editable_350_context_150: Range<usize>,
    pub editable_350_context_512: Option<Range<usize>>,
    pub editable_350_context_1024: Option<Range<usize>>,
    pub context_4096: Option<Range<usize>>,
    pub context_8192: Option<Range<usize>>,
}

/// Builds an `ExcerptRanges` by computing editable and context ranges for each
/// budget combination, using the syntax-aware logic in
/// `compute_editable_and_context_ranges`.
pub fn compute_legacy_excerpt_ranges(
    cursor_excerpt: &str,
    cursor_offset: usize,
    syntax_ranges: &[Range<usize>],
) -> ExcerptRanges {
    let compute = |editable_tokens, context_tokens| {
        compute_editable_and_context_ranges(
            cursor_excerpt,
            cursor_offset,
            syntax_ranges,
            editable_tokens,
            context_tokens,
        )
    };

    let (editable_150, editable_150_context_350) = compute(150, 350);
    let (editable_180, editable_180_context_350) = compute(180, 350);
    let (editable_350, editable_350_context_150) = compute(350, 150);
    let (editable_512, _) = compute(512, 0);
    let (_, editable_350_context_512) = compute(350, 512);
    let (_, editable_350_context_1024) = compute(350, 1024);
    let (_, context_4096) = compute(350, 4096);
    let (_, context_8192) = compute(350, 8192);

    ExcerptRanges {
        editable_150,
        editable_180,
        editable_350,
        editable_512: Some(editable_512),
        editable_150_context_350,
        editable_180_context_350,
        editable_350_context_150,
        editable_350_context_512: Some(editable_350_context_512),
        editable_350_context_1024: Some(editable_350_context_1024),
        context_4096: Some(context_4096),
        context_8192: Some(context_8192),
    }
}

/// Given the cursor excerpt text, cursor offset, and the syntax node ranges
/// containing the cursor (innermost to outermost), compute the editable range
/// and context range as byte offset ranges within `cursor_excerpt`.
///
/// This is the server-side equivalent of `compute_excerpt_ranges` in
/// `edit_prediction::cursor_excerpt`, but operates on plain text with
/// pre-computed syntax boundaries instead of a `BufferSnapshot`.
pub fn compute_editable_and_context_ranges(
    cursor_excerpt: &str,
    cursor_offset: usize,
    syntax_ranges: &[Range<usize>],
    editable_token_limit: usize,
    context_token_limit: usize,
) -> (Range<usize>, Range<usize>) {
    let line_starts = compute_line_starts(cursor_excerpt);
    let cursor_row = offset_to_row(&line_starts, cursor_offset);
    let max_row = line_starts.len().saturating_sub(1) as u32;

    let editable_range = compute_editable_range_from_text(
        cursor_excerpt,
        &line_starts,
        cursor_row,
        max_row,
        syntax_ranges,
        editable_token_limit,
    );

    let context_range = expand_context_from_text(
        cursor_excerpt,
        &line_starts,
        max_row,
        &editable_range,
        syntax_ranges,
        context_token_limit,
    );

    (editable_range, context_range)
}

fn compute_line_starts(text: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (index, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(index + 1);
        }
    }
    starts
}

fn offset_to_row(line_starts: &[usize], offset: usize) -> u32 {
    match line_starts.binary_search(&offset) {
        Ok(row) => row as u32,
        Err(row) => (row.saturating_sub(1)) as u32,
    }
}

fn row_start_offset(line_starts: &[usize], row: u32) -> usize {
    line_starts.get(row as usize).copied().unwrap_or(0)
}

fn row_end_offset(text: &str, line_starts: &[usize], row: u32) -> usize {
    if let Some(&next_start) = line_starts.get(row as usize + 1) {
        // End before the newline of this row.
        next_start.saturating_sub(1).min(text.len())
    } else {
        text.len()
    }
}

fn row_range_to_byte_range(
    text: &str,
    line_starts: &[usize],
    start_row: u32,
    end_row: u32,
) -> Range<usize> {
    let start = row_start_offset(line_starts, start_row);
    let end = row_end_offset(text, line_starts, end_row);
    start..end
}

fn estimate_tokens_for_row_range(
    text: &str,
    line_starts: &[usize],
    start_row: u32,
    end_row: u32,
) -> usize {
    let mut tokens = 0;
    for row in start_row..end_row {
        let row_len = row_end_offset(text, line_starts, row)
            .saturating_sub(row_start_offset(line_starts, row));
        tokens += estimate_tokens(row_len).max(1);
    }
    tokens
}

fn line_token_count_from_text(text: &str, line_starts: &[usize], row: u32) -> usize {
    let row_len =
        row_end_offset(text, line_starts, row).saturating_sub(row_start_offset(line_starts, row));
    estimate_tokens(row_len).max(1)
}

/// Returns syntax boundaries (as row ranges) that contain the given row range
/// and extend beyond it, ordered from smallest to largest.
fn containing_syntax_boundaries_from_ranges(
    line_starts: &[usize],
    syntax_ranges: &[Range<usize>],
    start_row: u32,
    end_row: u32,
) -> Vec<(u32, u32)> {
    let mut boundaries = Vec::new();
    let mut last: Option<(u32, u32)> = None;

    // syntax_ranges is innermost to outermost, so iterate in order.
    for range in syntax_ranges {
        let node_start_row = offset_to_row(line_starts, range.start);
        let node_end_row = offset_to_row(line_starts, range.end);

        // Skip nodes that don't extend beyond the current range.
        if node_start_row >= start_row && node_end_row <= end_row {
            continue;
        }

        let rows = (node_start_row, node_end_row);
        if last == Some(rows) {
            continue;
        }

        last = Some(rows);
        boundaries.push(rows);
    }

    boundaries
}

fn compute_editable_range_from_text(
    text: &str,
    line_starts: &[usize],
    cursor_row: u32,
    max_row: u32,
    syntax_ranges: &[Range<usize>],
    token_limit: usize,
) -> Range<usize> {
    // Phase 1: Expand symmetrically from cursor using 75% of budget.
    let initial_budget = (token_limit * 3) / 4;
    let (mut start_row, mut end_row, mut remaining_tokens) =
        expand_symmetric(text, line_starts, cursor_row, max_row, initial_budget);

    remaining_tokens += token_limit.saturating_sub(initial_budget);

    let original_start = start_row;
    let original_end = end_row;

    // Phase 2: Expand to syntax boundaries that fit within budget.
    let boundaries =
        containing_syntax_boundaries_from_ranges(line_starts, syntax_ranges, start_row, end_row);
    for (boundary_start, boundary_end) in &boundaries {
        let tokens_for_start = if *boundary_start < start_row {
            estimate_tokens_for_row_range(text, line_starts, *boundary_start, start_row)
        } else {
            0
        };
        let tokens_for_end = if *boundary_end > end_row {
            estimate_tokens_for_row_range(text, line_starts, end_row + 1, *boundary_end + 1)
        } else {
            0
        };

        let total_needed = tokens_for_start + tokens_for_end;
        if total_needed <= remaining_tokens {
            if *boundary_start < start_row {
                start_row = *boundary_start;
            }
            if *boundary_end > end_row {
                end_row = *boundary_end;
            }
            remaining_tokens = remaining_tokens.saturating_sub(total_needed);
        } else {
            break;
        }
    }

    // Phase 3: Continue line-wise in the direction we expanded least.
    let expanded_up = original_start.saturating_sub(start_row);
    let expanded_down = end_row.saturating_sub(original_end);
    let prefer_up = expanded_up <= expanded_down;

    (start_row, end_row, _) = expand_linewise(
        text,
        line_starts,
        start_row,
        end_row,
        max_row,
        remaining_tokens,
        prefer_up,
    );

    row_range_to_byte_range(text, line_starts, start_row, end_row)
}

fn expand_context_from_text(
    text: &str,
    line_starts: &[usize],
    max_row: u32,
    editable_range: &Range<usize>,
    syntax_ranges: &[Range<usize>],
    context_token_limit: usize,
) -> Range<usize> {
    let mut start_row = offset_to_row(line_starts, editable_range.start);
    let mut end_row = offset_to_row(line_starts, editable_range.end);
    let mut remaining_tokens = context_token_limit;
    let mut did_syntax_expand = false;

    let boundaries =
        containing_syntax_boundaries_from_ranges(line_starts, syntax_ranges, start_row, end_row);
    for (boundary_start, boundary_end) in &boundaries {
        let tokens_for_start = if *boundary_start < start_row {
            estimate_tokens_for_row_range(text, line_starts, *boundary_start, start_row)
        } else {
            0
        };
        let tokens_for_end = if *boundary_end > end_row {
            estimate_tokens_for_row_range(text, line_starts, end_row + 1, *boundary_end + 1)
        } else {
            0
        };

        let total_needed = tokens_for_start + tokens_for_end;
        if total_needed <= remaining_tokens {
            if *boundary_start < start_row {
                start_row = *boundary_start;
            }
            if *boundary_end > end_row {
                end_row = *boundary_end;
            }
            remaining_tokens = remaining_tokens.saturating_sub(total_needed);
            did_syntax_expand = true;
        } else {
            break;
        }
    }

    // Only expand line-wise if no syntax expansion occurred.
    if !did_syntax_expand {
        (start_row, end_row, _) = expand_linewise(
            text,
            line_starts,
            start_row,
            end_row,
            max_row,
            remaining_tokens,
            true,
        );
    }

    row_range_to_byte_range(text, line_starts, start_row, end_row)
}

fn expand_symmetric(
    text: &str,
    line_starts: &[usize],
    cursor_row: u32,
    max_row: u32,
    mut token_budget: usize,
) -> (u32, u32, usize) {
    let mut start_row = cursor_row;
    let mut end_row = cursor_row;

    let cursor_line_tokens = line_token_count_from_text(text, line_starts, cursor_row);
    token_budget = token_budget.saturating_sub(cursor_line_tokens);

    loop {
        let can_expand_up = start_row > 0;
        let can_expand_down = end_row < max_row;

        if token_budget == 0 || (!can_expand_up && !can_expand_down) {
            break;
        }

        if can_expand_down {
            let next_row = end_row + 1;
            let line_tokens = line_token_count_from_text(text, line_starts, next_row);
            if line_tokens <= token_budget {
                end_row = next_row;
                token_budget = token_budget.saturating_sub(line_tokens);
            } else {
                break;
            }
        }

        if can_expand_up && token_budget > 0 {
            let next_row = start_row - 1;
            let line_tokens = line_token_count_from_text(text, line_starts, next_row);
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

fn expand_linewise(
    text: &str,
    line_starts: &[usize],
    mut start_row: u32,
    mut end_row: u32,
    max_row: u32,
    mut remaining_tokens: usize,
    prefer_up: bool,
) -> (u32, u32, usize) {
    loop {
        let can_expand_up = start_row > 0;
        let can_expand_down = end_row < max_row;

        if remaining_tokens == 0 || (!can_expand_up && !can_expand_down) {
            break;
        }

        let mut expanded = false;

        if prefer_up {
            if can_expand_up {
                let next_row = start_row - 1;
                let line_tokens = line_token_count_from_text(text, line_starts, next_row);
                if line_tokens <= remaining_tokens {
                    start_row = next_row;
                    remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
                    expanded = true;
                }
            }
            if can_expand_down && remaining_tokens > 0 {
                let next_row = end_row + 1;
                let line_tokens = line_token_count_from_text(text, line_starts, next_row);
                if line_tokens <= remaining_tokens {
                    end_row = next_row;
                    remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
                    expanded = true;
                }
            }
        } else {
            if can_expand_down {
                let next_row = end_row + 1;
                let line_tokens = line_token_count_from_text(text, line_starts, next_row);
                if line_tokens <= remaining_tokens {
                    end_row = next_row;
                    remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
                    expanded = true;
                }
            }
            if can_expand_up && remaining_tokens > 0 {
                let next_row = start_row - 1;
                let line_tokens = line_token_count_from_text(text, line_starts, next_row);
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
