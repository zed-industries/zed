use language::{BufferSnapshot, Point};
use std::ops::Range;

pub fn editable_and_context_ranges_for_cursor_position(
    position: Point,
    snapshot: &BufferSnapshot,
    editable_region_token_limit: usize,
    context_token_limit: usize,
) -> (Range<Point>, Range<Point>) {
    let mut scope_range = position..position;
    let mut remaining_edit_tokens = editable_region_token_limit;

    while let Some(parent) = snapshot.syntax_ancestor(scope_range.clone()) {
        let parent_tokens = guess_token_count(parent.byte_range().len());
        let parent_point_range = Point::new(
            parent.start_position().row as u32,
            parent.start_position().column as u32,
        )
            ..Point::new(
                parent.end_position().row as u32,
                parent.end_position().column as u32,
            );
        if parent_point_range == scope_range {
            break;
        } else if parent_tokens <= editable_region_token_limit {
            scope_range = parent_point_range;
            remaining_edit_tokens = editable_region_token_limit - parent_tokens;
        } else {
            break;
        }
    }

    let editable_range = expand_range(snapshot, scope_range, remaining_edit_tokens);
    let context_range = expand_range(snapshot, editable_range.clone(), context_token_limit);
    (editable_range, context_range)
}

fn expand_range(
    snapshot: &BufferSnapshot,
    range: Range<Point>,
    mut remaining_tokens: usize,
) -> Range<Point> {
    let mut expanded_range = range;
    expanded_range.start.column = 0;
    expanded_range.end.column = snapshot.line_len(expanded_range.end.row);
    loop {
        let mut expanded = false;

        if remaining_tokens > 0 && expanded_range.start.row > 0 {
            expanded_range.start.row -= 1;
            let line_tokens =
                guess_token_count(snapshot.line_len(expanded_range.start.row) as usize);
            remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
            expanded = true;
        }

        if remaining_tokens > 0 && expanded_range.end.row < snapshot.max_point().row {
            expanded_range.end.row += 1;
            expanded_range.end.column = snapshot.line_len(expanded_range.end.row);
            let line_tokens = guess_token_count(expanded_range.end.column as usize);
            remaining_tokens = remaining_tokens.saturating_sub(line_tokens);
            expanded = true;
        }

        if !expanded {
            break;
        }
    }
    expanded_range
}

/// Typical number of string bytes per token for the purposes of limiting model input. This is
/// intentionally low to err on the side of underestimating limits.
pub(crate) const BYTES_PER_TOKEN_GUESS: usize = 3;

pub fn guess_token_count(bytes: usize) -> usize {
    bytes / BYTES_PER_TOKEN_GUESS
}
