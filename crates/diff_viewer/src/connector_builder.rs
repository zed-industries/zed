use crate::connector::{ConnectorCurve, ConnectorKind};
use buffer_diff::DiffHunkStatusKind;
use std::ops::Range;

/// Represents a simplified diff block for connector rendering
#[derive(Debug, Clone)]
pub struct DiffBlock {
    pub left_range: Range<usize>,
    pub right_range: Range<usize>,
    pub kind: DiffHunkStatusKind,
}

/// Build connector ribbons between matching change ranges on left and right panes.
pub fn build_connector_curves(blocks: &[DiffBlock]) -> Vec<ConnectorCurve> {
    blocks
        .iter()
        .filter_map(|block| {
            if block.left_range.is_empty() && block.right_range.is_empty() {
                return None;
            }

            let kind = match block.kind {
                DiffHunkStatusKind::Modified => ConnectorKind::Modify,
                DiffHunkStatusKind::Added => ConnectorKind::Insert,
                DiffHunkStatusKind::Deleted => ConnectorKind::Delete,
            };

            let left_crushed = block.left_range.is_empty();
            let right_crushed = block.right_range.is_empty();

            let left_start = block.left_range.start;
            let left_end = if left_crushed {
                left_start
            } else {
                block
                    .left_range
                    .end
                    .saturating_sub(1)
                    .max(block.left_range.start)
            };

            let right_start = block.right_range.start;
            let right_end = if right_crushed {
                right_start
            } else {
                block
                    .right_range
                    .end
                    .saturating_sub(1)
                    .max(block.right_range.start)
            };

            let focus_line = match (left_crushed, right_crushed) {
                (true, false) => right_start,
                (false, true) => right_start.saturating_sub(1),
                _ => left_start.min(right_start),
            };

            Some(ConnectorCurve::new(
                focus_line,
                left_start,
                left_end,
                right_start,
                right_end,
                kind,
                left_crushed,
                right_crushed,
            ))
        })
        .collect()
}
