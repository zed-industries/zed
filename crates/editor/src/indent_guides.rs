use std::ops::Range;

use collections::HashSet;
use gpui::AppContext;
use language::IndentGuide;
use multi_buffer::MultiBufferRow;
use text::Point;

use crate::{DisplaySnapshot, Editor};

impl Editor {
    pub fn indent_guides_in_range(
        &self,
        visible_buffer_range: Range<u32>,
        snapshot: &DisplaySnapshot,
        cx: &AppContext,
    ) -> Vec<(Range<usize>, IndentGuide)> {
        let start_anchor = snapshot
            .buffer_snapshot
            .anchor_before(Point::new(visible_buffer_range.start, 0));
        let end_anchor = snapshot
            .buffer_snapshot
            .anchor_after(Point::new(visible_buffer_range.end, 0));

        snapshot
            .buffer_snapshot
            .indent_guides_in_range(start_anchor..end_anchor, cx)
            .into_iter()
            .filter(|(_, indent_guide)| {
                // Filter out indent guides that are inside a fold
                !snapshot.is_line_folded(MultiBufferRow(indent_guide.start_row))
            })
            .collect()
    }

    pub fn find_active_indent_guide_indices(
        &self,
        indent_guides: &[(Range<usize>, IndentGuide)],
        snapshot: &DisplaySnapshot,
        cx: &AppContext,
    ) -> Option<HashSet<usize>> {
        let selection = self.selections.newest::<Point>(cx);
        let cursor_row = selection.head().row;

        crate::indent_guides::find_active_indent_guide_indices(cursor_row, indent_guides, snapshot)
    }
}

fn find_active_indent_guide_indices(
    cursor_row: u32,
    indent_guides: &[(Range<usize>, IndentGuide)],
    snapshot: &DisplaySnapshot,
) -> Option<HashSet<usize>> {
    let (buffer_row, buffer_snapshot, buffer_id) =
        if let Some((_, buffer_id, snapshot)) = snapshot.buffer_snapshot.as_singleton() {
            (cursor_row, snapshot, buffer_id)
        } else {
            let (snapshot, point) = snapshot
                .buffer_snapshot
                .buffer_line_for_row(MultiBufferRow(cursor_row))?;

            let buffer_id = snapshot.remote_id();
            (point.start.row, snapshot, buffer_id)
        };

    let Some((row_range, target_indent)) = buffer_snapshot.enclosing_indent(buffer_row) else {
        return None;
    };

    let candidates = indent_guides
        .iter()
        .enumerate()
        .filter(|(_, (_, indent_guide))| {
            indent_guide.buffer_id == buffer_id && indent_guide.indent_width() == target_indent
        });

    let mut matches = HashSet::default();
    for (i, (_, indent)) in candidates {
        // Find matches that are either an exact match, partially on screen, or inside the enclosing indent
        if row_range.start <= indent.end_row && indent.start_row <= row_range.end {
            matches.insert(i);
        }
    }
    Some(matches)
}
