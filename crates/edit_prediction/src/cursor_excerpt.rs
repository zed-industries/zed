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
