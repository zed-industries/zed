use std::{ops::Range, time::Duration};

use collections::HashSet;
use gpui::{AppContext, Task};
use language::IndentGuide;
use multi_buffer::MultiBufferRow;
use text::{BufferId, Point};
use ui::ViewContext;
use util::ResultExt;

use crate::{DisplaySnapshot, Editor};

struct ActiveIndentedRange {
    buffer_id: BufferId,
    row_range: Range<u32>,
    indent: u32,
}

pub struct ActiveIndentGuidesState {
    pub dirty: bool,
    cursor_row: u32,
    pending_refresh: Option<Task<()>>,
    active_indent_range: Option<ActiveIndentedRange>,
}

impl ActiveIndentGuidesState {
    pub fn should_refresh(&self, cursor_row: u32) -> bool {
        self.pending_refresh.is_none() && (self.cursor_row != cursor_row || self.dirty)
    }
}

impl Default for ActiveIndentGuidesState {
    fn default() -> Self {
        Self {
            cursor_row: 0,
            dirty: false,
            pending_refresh: None,
            active_indent_range: None,
        }
    }
}

impl Editor {
    pub fn find_active_indent_guide_indices(
        &mut self,
        indent_guides: &[(Range<usize>, IndentGuide)],
        snapshot: &DisplaySnapshot,
        cx: &mut ViewContext<Editor>,
    ) -> Option<HashSet<usize>> {
        let selection = self.selections.newest::<Point>(cx);
        let cursor_row = selection.head().row;

        let state = &mut self.active_indent_guides_state;
        if state.cursor_row != cursor_row {
            state.cursor_row = cursor_row;
            state.dirty = true;
        }

        if state.should_refresh(cursor_row) {
            let snapshot = snapshot.clone();
            state.dirty = false;

            let task = cx
                .background_executor()
                .spawn(resolve_indented_range(snapshot, cursor_row));

            // Try to resolve the indent in a short amount of time, otherwise move it to a background task.
            match cx
                .background_executor()
                .block_with_timeout(Duration::from_micros(200), task)
            {
                Ok(result) => state.active_indent_range = result,
                Err(future) => {
                    state.pending_refresh = Some(cx.spawn(|editor, mut cx| async move {
                        let result = cx.background_executor().spawn(future).await;
                        editor
                            .update(&mut cx, |editor, _| {
                                editor.active_indent_guides_state.active_indent_range = result;
                                editor.active_indent_guides_state.pending_refresh = None;
                            })
                            .log_err();
                    }));
                    return None;
                }
            }
        }

        let active_indent_range = state.active_indent_range.as_ref()?;

        let candidates = indent_guides
            .iter()
            .enumerate()
            .filter(|(_, (_, indent_guide))| {
                indent_guide.buffer_id == active_indent_range.buffer_id
                    && indent_guide.indent_width() == active_indent_range.indent
            });

        let mut matches = HashSet::default();
        for (i, (_, indent)) in candidates {
            // Find matches that are either an exact match, partially on screen, or inside the enclosing indent
            if active_indent_range.row_range.start <= indent.end_row
                && indent.start_row <= active_indent_range.row_range.end
            {
                matches.insert(i);
            }
        }
        Some(matches)
    }
}

pub fn indent_guides_in_range(
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

async fn resolve_indented_range(
    snapshot: DisplaySnapshot,
    buffer_row: u32,
) -> Option<ActiveIndentedRange> {
    let (buffer_row, buffer_snapshot, buffer_id) =
        if let Some((_, buffer_id, snapshot)) = snapshot.buffer_snapshot.as_singleton() {
            (buffer_row, snapshot, buffer_id)
        } else {
            let (snapshot, point) = snapshot
                .buffer_snapshot
                .buffer_line_for_row(MultiBufferRow(buffer_row))?;

            let buffer_id = snapshot.remote_id();
            (point.start.row, snapshot, buffer_id)
        };

    buffer_snapshot
        .enclosing_indent(buffer_row)
        .await
        .map(|(row_range, indent)| ActiveIndentedRange {
            row_range,
            indent,
            buffer_id,
        })
}
