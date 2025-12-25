mod diff_tree;

use std::ops::Range;

pub use diff_tree::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SyntaxDiff {
    /// Ranges in the base/old text that were deleted or modified
    pub base_ranges: Vec<Range<usize>>,
    /// Ranges in the buffer/new text that were inserted or modified
    pub buffer_ranges: Vec<Range<usize>>,
}

impl SyntaxDiff {
    /// Compute syntax diff from two diff trees.
    /// `base_hunk_start` and `buffer_hunk_start` are the byte offsets where the hunks start.
    /// The ranges returned are relative to these hunk start positions.
    pub fn compute(
        base_tree: &DiffTree,
        buffer_tree: &DiffTree,
        base_hunk_start: usize,
        buffer_hunk_start: usize,
    ) -> Self {
        let matching = match_trees(base_tree, buffer_tree);
        let diff = generate_diff(base_tree, buffer_tree, &matching);

        let mut base_ranges = Vec::new();
        let mut buffer_ranges = Vec::new();

        for op in diff.operations {
            match op {
                DiffOperation::Delete(range) => {
                    base_ranges.push(
                        range.start.saturating_sub(base_hunk_start)
                            ..range.end.saturating_sub(base_hunk_start),
                    );
                }
                DiffOperation::Insert(range) => {
                    buffer_ranges.push(
                        range.start.saturating_sub(buffer_hunk_start)
                            ..range.end.saturating_sub(buffer_hunk_start),
                    );
                }
                DiffOperation::Update {
                    old_range,
                    new_range,
                } => {
                    base_ranges.push(
                        old_range.start.saturating_sub(base_hunk_start)
                            ..old_range.end.saturating_sub(base_hunk_start),
                    );
                    buffer_ranges.push(
                        new_range.start.saturating_sub(buffer_hunk_start)
                            ..new_range.end.saturating_sub(buffer_hunk_start),
                    );
                }
                DiffOperation::Move {
                    old_range,
                    new_range,
                } => {
                    base_ranges.push(
                        old_range.start.saturating_sub(base_hunk_start)
                            ..old_range.end.saturating_sub(base_hunk_start),
                    );
                    buffer_ranges.push(
                        new_range.start.saturating_sub(buffer_hunk_start)
                            ..new_range.end.saturating_sub(buffer_hunk_start),
                    );
                }
            }
        }

        // Sort and merge overlapping ranges
        Self::merge_ranges(&mut base_ranges);
        Self::merge_ranges(&mut buffer_ranges);

        Self {
            base_ranges,
            buffer_ranges,
        }
    }

    fn merge_ranges(ranges: &mut Vec<Range<usize>>) {
        if ranges.len() <= 1 {
            return;
        }

        ranges.sort_by_key(|r| r.start);

        let mut write_idx = 0;
        for read_idx in 1..ranges.len() {
            if ranges[read_idx].start <= ranges[write_idx].end {
                ranges[write_idx].end = ranges[write_idx].end.max(ranges[read_idx].end);
            } else {
                write_idx += 1;
                if write_idx != read_idx {
                    ranges[write_idx] = ranges[read_idx].clone();
                }
            }
        }

        ranges.truncate(write_idx + 1);
    }

    pub fn is_empty(&self) -> bool {
        self.base_ranges.is_empty() && self.buffer_ranges.is_empty()
    }
}
