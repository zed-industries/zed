use multi_buffer::{MultiBufferRow, MultiBufferSnapshot};

/// Whole-document statistics: total line count, total character count, and the
/// number of blocks (paragraphs) in the document.
///
/// This module intentionally has no dependency on `editor`, `gpui`, or `settings`,
/// so that the aggregation logic can be reused by status bar items other than
/// [`crate::cursor_position::CursorPosition`] without pulling in its rendering code.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentStatsValues {
    /// The total number of lines in the document, based on line breaks in the
    /// buffer contents (not on soft wrapping).
    pub lines: u32,
    /// The total number of characters (Unicode code points) in the document.
    pub characters: usize,
    /// The number of blocks (paragraphs) in the document. A block is a maximal
    /// run of one or more non-empty lines. Two or more consecutive empty lines
    /// separate blocks; a single line break does not. A line containing only
    /// whitespace (spaces/tabs) is not considered empty for this purpose.
    pub blocks: usize,
}

/// Computes whole-document statistics for `snapshot`.
///
/// Total line and character counts are read from the buffer's cached rope
/// summary (`O(log n)`), so they are cheap even for large documents. Counting
/// blocks requires a single pass over every row (`O(number of lines)`) since
/// blank-line runs are not tracked by the rope summary; callers computing this
/// for large documents on every keystroke should debounce/background the call.
pub fn compute(snapshot: &MultiBufferSnapshot) -> DocumentStatsValues {
    let text_summary = snapshot.text_summary();
    let max_row = text_summary.lines.row;
    let lines = max_row + 1;
    let characters = text_summary.chars;
    let blocks = count_blocks(snapshot, max_row);

    DocumentStatsValues {
        lines,
        characters,
        blocks,
    }
}

fn count_blocks(snapshot: &MultiBufferSnapshot, max_row: u32) -> usize {
    let mut blocks = 0usize;
    let mut in_block = false;
    for row in 0..=max_row {
        let is_empty_line = snapshot.line_len(MultiBufferRow(row)) == 0;
        if is_empty_line {
            in_block = false;
        } else if !in_block {
            blocks += 1;
            in_block = true;
        }
    }
    blocks
}
