use std::{cmp::Ordering, ops::Range, time::Duration};

use collections::HashSet;
use gpui::{App, AppContext as _, Context, Task, Window};
use language::language_settings::language_settings;
use multi_buffer::{IndentGuide, MultiBufferRow, ToPoint};
use text::{LineIndent, Point};
use util::ResultExt;

use crate::{DisplaySnapshot, Editor};

struct ActiveIndentedRange {
    row_range: Range<MultiBufferRow>,
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
        cx: &mut Context<Editor>,
    ) -> Option<Vec<IndentGuide>> {
        let show_indent_guides = self.should_show_indent_guides().unwrap_or_else(|| {
            if let Some(buffer) = self.buffer().read(cx).as_singleton() {
                language_settings(
                    buffer.read(cx).language().map(|l| l.name()),
                    buffer.read(cx).file(),
                    cx,
                )
                .indent_guides
                .enabled
            } else {
                true
            }
        });

        if !show_indent_guides {
            return None;
        }

        Some(indent_guides_in_range(
            self,
            visible_buffer_range,
            self.should_show_indent_guides() == Some(true),
            snapshot,
            cx,
        ))
    }

    pub fn find_active_indent_guide_indices(
        &mut self,
        indent_guides: &[IndentGuide],
        snapshot: &DisplaySnapshot,
        window: &mut Window,
        cx: &mut Context<Editor>,
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
            state.dirty = false;

            if indent_guides.is_empty() {
                return None;
            }

            let snapshot = snapshot.clone();

            let task = cx.background_spawn(resolve_indented_range(snapshot, cursor_row));

            // Try to resolve the indent in a short amount of time, otherwise move it to a background task.
            match cx
                .background_executor()
                .block_with_timeout(Duration::from_micros(200), task)
            {
                Ok(result) => state.active_indent_range = result,
                Err(future) => {
                    state.pending_refresh = Some(cx.spawn_in(window, async move |editor, cx| {
                        let result = cx.background_spawn(future).await;
                        editor
                            .update(cx, |editor, _| {
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
                indent_guide.indent_level() == active_indent_range.indent.len(indent_guide.tab_size)
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
    editor: &Editor,
    visible_buffer_range: Range<MultiBufferRow>,
    ignore_disabled_for_language: bool,
    snapshot: &DisplaySnapshot,
    cx: &App,
) -> Vec<IndentGuide> {
    let start_offset = snapshot
        .buffer_snapshot()
        .point_to_offset(Point::new(visible_buffer_range.start.0, 0));
    let end_offset = snapshot
        .buffer_snapshot()
        .point_to_offset(Point::new(visible_buffer_range.end.0, 0));
    let start_anchor = snapshot.buffer_snapshot().anchor_before(start_offset);
    let end_anchor = snapshot.buffer_snapshot().anchor_after(end_offset);

    let mut fold_ranges = Vec::<Range<Point>>::new();
    let folds = snapshot.folds_in_range(start_offset..end_offset).peekable();
    for fold in folds {
        let start = fold.range.start.to_point(&snapshot.buffer_snapshot());
        let end = fold.range.end.to_point(&snapshot.buffer_snapshot());
        if let Some(last_range) = fold_ranges.last_mut()
            && last_range.end >= start
        {
            last_range.end = last_range.end.max(end);
            continue;
        }
        fold_ranges.push(start..end);
    }

    snapshot
        .buffer_snapshot()
        .indent_guides_in_range(start_anchor..end_anchor, ignore_disabled_for_language, cx)
        .filter(|indent_guide| {
            if editor.is_buffer_folded(indent_guide.buffer_id, cx) {
                return false;
            }

            let has_containing_fold = fold_ranges
                .binary_search_by(|fold_range| {
                    if fold_range.start >= Point::new(indent_guide.start_row.0, 0) {
                        Ordering::Greater
                    } else if fold_range.end < Point::new(indent_guide.end_row.0, 0) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                })
                .is_ok();

            !has_containing_fold
        })
        .collect()
}

async fn resolve_indented_range(
    snapshot: DisplaySnapshot,
    buffer_row: MultiBufferRow,
) -> Option<ActiveIndentedRange> {
    snapshot
        .buffer_snapshot()
        .enclosing_indent(buffer_row)
        .await
        .map(|(row_range, indent)| ActiveIndentedRange { row_range, indent })
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
    if snapshot.buffer_snapshot().is_singleton() {
        if !current_indent_range.row_range.contains(&new_row) {
            return true;
        }

        let old_line_indent = snapshot.buffer_snapshot().line_indent_for_row(prev_row);
        let new_line_indent = snapshot.buffer_snapshot().line_indent_for_row(new_row);

        if old_line_indent.is_line_empty()
            || new_line_indent.is_line_empty()
            || old_line_indent != new_line_indent
            || snapshot.buffer_snapshot().max_point().row == new_row.0
        {
            return true;
        }

        let next_line_indent = snapshot.buffer_snapshot().line_indent_for_row(new_row + 1);
        next_line_indent.is_line_empty() || next_line_indent != old_line_indent
    } else {
        true
    }
}
