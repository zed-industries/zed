use std::{ops::Range, time::Duration};

use collections::HashSet;
use gpui::{AppContext, Task};
use language::BufferRow;
use multi_buffer::{MultiBufferIndentGuide, MultiBufferRow};
use text::{BufferId, LineIndent, Point};
use ui::ViewContext;
use util::ResultExt;

use crate::{DisplaySnapshot, Editor};

struct ActiveIndentedRange {
    buffer_id: BufferId,
    row_range: Range<BufferRow>,
    indent: LineIndent,
}

#[derive(Default)]
pub struct ActiveIndentGuidesState {
    pub dirty: bool,
    cursor_row: MultiBufferRow,
    pending_refresh: Option<Task<()>>,
    active_indent_range: Option<ActiveIndentedRange>,
}

impl ActiveIndentGuidesState {
    pub fn should_refresh(&self) -> bool {
        self.pending_refresh.is_none() && self.dirty
    }
}

impl Editor {
    pub fn indent_guides(
        &self,
        visible_buffer_range: Range<MultiBufferRow>,
        snapshot: &DisplaySnapshot,
        cx: &mut ViewContext<Editor>,
    ) -> Option<Vec<MultiBufferIndentGuide>> {
        let enabled = self.should_show_indent_guides(cx);

        if enabled {
            Some(indent_guides_in_range(visible_buffer_range, snapshot, cx))
        } else {
            None
        }
    }

    pub fn find_active_indent_guide_indices(
        &mut self,
        indent_guides: &[MultiBufferIndentGuide],
        snapshot: &DisplaySnapshot,
        cx: &mut ViewContext<Editor>,
    ) -> Option<HashSet<usize>> {
        let selection = self.selections.newest::<Point>(cx);
        let cursor_row = MultiBufferRow(selection.head().row);

        let state = &mut self.active_indent_guides_state;

        if state
            .active_indent_range
            .as_ref()
            .map(|active_indent_range| {
                should_recalculate_indented_range(
                    state.cursor_row,
                    cursor_row,
                    active_indent_range,
                    snapshot,
                )
            })
            .unwrap_or(true)
        {
            state.dirty = true;
        } else {
            state.cursor_row = cursor_row;
        }

        if state.should_refresh() {
            state.cursor_row = cursor_row;
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
            .filter(|(_, indent_guide)| {
                indent_guide.buffer_id == active_indent_range.buffer_id
                    && indent_guide.indent_level()
                        == active_indent_range.indent.len(indent_guide.tab_size)
            });

        let mut matches = HashSet::default();
        for (i, indent) in candidates {
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
    visible_buffer_range: Range<MultiBufferRow>,
    snapshot: &DisplaySnapshot,
    cx: &AppContext,
) -> Vec<MultiBufferIndentGuide> {
    let start_anchor = snapshot
        .buffer_snapshot
        .anchor_before(Point::new(visible_buffer_range.start.0, 0));
    let end_anchor = snapshot
        .buffer_snapshot
        .anchor_after(Point::new(visible_buffer_range.end.0, 0));

    snapshot
        .buffer_snapshot
        .indent_guides_in_range(start_anchor..end_anchor, cx)
        .into_iter()
        .filter(|indent_guide| {
            // Filter out indent guides that are inside a fold
            !snapshot.is_line_folded(MultiBufferRow(
                indent_guide.multibuffer_row_range.start.0.saturating_sub(1),
            ))
        })
        .collect()
}

async fn resolve_indented_range(
    snapshot: DisplaySnapshot,
    buffer_row: MultiBufferRow,
) -> Option<ActiveIndentedRange> {
    let (buffer_row, buffer_snapshot, buffer_id) =
        if let Some((_, buffer_id, snapshot)) = snapshot.buffer_snapshot.as_singleton() {
            (buffer_row.0, snapshot, buffer_id)
        } else {
            let (snapshot, point) = snapshot.buffer_snapshot.buffer_line_for_row(buffer_row)?;

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

fn should_recalculate_indented_range(
    prev_row: MultiBufferRow,
    new_row: MultiBufferRow,
    current_indent_range: &ActiveIndentedRange,
    snapshot: &DisplaySnapshot,
) -> bool {
    if prev_row.0 == new_row.0 {
        return false;
    }
    if let Some((_, _, snapshot)) = snapshot.buffer_snapshot.as_singleton() {
        if !current_indent_range.row_range.contains(&new_row.0) {
            return true;
        }

        let old_line_indent = snapshot.line_indent_for_row(prev_row.0);
        let new_line_indent = snapshot.line_indent_for_row(new_row.0);

        if old_line_indent.is_line_empty()
            || new_line_indent.is_line_empty()
            || old_line_indent != new_line_indent
            || snapshot.max_point().row == new_row.0
        {
            return true;
        }

        let next_line_indent = snapshot.line_indent_for_row(new_row.0 + 1);
        next_line_indent.is_line_empty() || next_line_indent != old_line_indent
    } else {
        true
    }
}
