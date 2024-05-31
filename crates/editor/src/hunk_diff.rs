use std::{
    ops::{Range, RangeInclusive},
    sync::Arc,
};

use collections::{hash_map, HashMap, HashSet};
use git::diff::{DiffHunk, DiffHunkStatus};
use gpui::{AppContext, Hsla, Model, Task, View};
use language::Buffer;
use multi_buffer::{
    Anchor, ExcerptRange, MultiBuffer, MultiBufferRow, MultiBufferSnapshot, ToPoint,
};
use settings::{Settings, SettingsStore};
use text::{BufferId, Point};
use ui::{
    div, ActiveTheme, Context as _, IntoElement, ParentElement, Styled, ViewContext, VisualContext,
};
use util::{debug_panic, RangeExt};

use crate::{
    editor_settings::CurrentLineHighlight,
    git::{diff_hunk_to_display, DisplayDiffHunk},
    hunk_status, hunks_for_selections, BlockDisposition, BlockId, BlockProperties, BlockStyle,
    DiffRowHighlight, Editor, EditorSettings, EditorSnapshot, ExpandAllHunkDiffs, RangeToAnchorExt,
    RevertSelectedHunks, ToDisplayPoint, ToggleHunkDiff,
};

#[derive(Debug, Clone)]
pub(super) struct HunkToExpand {
    pub multi_buffer_range: Range<Anchor>,
    pub status: DiffHunkStatus,
    pub diff_base_byte_range: Range<usize>,
}

#[derive(Debug, Default)]
pub(super) struct ExpandedHunks {
    hunks: Vec<ExpandedHunk>,
    diff_base: HashMap<BufferId, DiffBaseBuffer>,
    hunk_update_tasks: HashMap<Option<BufferId>, Task<()>>,
}

#[derive(Debug)]
struct DiffBaseBuffer {
    buffer: Model<Buffer>,
    diff_base_version: usize,
}

impl ExpandedHunks {
    pub fn hunks(&self, include_folded: bool) -> impl Iterator<Item = &ExpandedHunk> {
        self.hunks
            .iter()
            .filter(move |hunk| include_folded || !hunk.folded)
    }
}

#[derive(Debug, Clone)]
pub(super) struct ExpandedHunk {
    pub block: Option<BlockId>,
    pub hunk_range: Range<Anchor>,
    pub diff_base_byte_range: Range<usize>,
    pub status: DiffHunkStatus,
    pub folded: bool,
}

impl Editor {
    pub fn toggle_hunk_diff(&mut self, _: &ToggleHunkDiff, cx: &mut ViewContext<Self>) {
        let multi_buffer_snapshot = self.buffer().read(cx).snapshot(cx);
        let selections = self.selections.disjoint_anchors();
        self.toggle_hunks_expanded(
            hunks_for_selections(&multi_buffer_snapshot, &selections),
            cx,
        );
    }

    pub fn expand_all_hunk_diffs(&mut self, _: &ExpandAllHunkDiffs, cx: &mut ViewContext<Self>) {
        let snapshot = self.snapshot(cx);
        let display_rows_with_expanded_hunks = self
            .expanded_hunks
            .hunks(false)
            .map(|hunk| &hunk.hunk_range)
            .map(|anchor_range| {
                (
                    anchor_range
                        .start
                        .to_display_point(&snapshot.display_snapshot)
                        .row(),
                    anchor_range
                        .end
                        .to_display_point(&snapshot.display_snapshot)
                        .row(),
                )
            })
            .collect::<HashMap<_, _>>();
        let hunks = snapshot
            .display_snapshot
            .buffer_snapshot
            .git_diff_hunks_in_range(MultiBufferRow::MIN..MultiBufferRow::MAX)
            .filter(|hunk| {
                let hunk_display_row_range = Point::new(hunk.associated_range.start.0, 0)
                    .to_display_point(&snapshot.display_snapshot)
                    ..Point::new(hunk.associated_range.end.0, 0)
                        .to_display_point(&snapshot.display_snapshot);
                let row_range_end =
                    display_rows_with_expanded_hunks.get(&hunk_display_row_range.start.row());
                row_range_end.is_none() || row_range_end != Some(&hunk_display_row_range.end.row())
            });
        self.toggle_hunks_expanded(hunks.collect(), cx);
    }

    fn toggle_hunks_expanded(
        &mut self,
        hunks_to_toggle: Vec<DiffHunk<MultiBufferRow>>,
        cx: &mut ViewContext<Self>,
    ) {
        let previous_toggle_task = self.expanded_hunks.hunk_update_tasks.remove(&None);
        let new_toggle_task = cx.spawn(move |editor, mut cx| async move {
            if let Some(task) = previous_toggle_task {
                task.await;
            }

            editor
                .update(&mut cx, |editor, cx| {
                    let snapshot = editor.snapshot(cx);
                    let mut hunks_to_toggle = hunks_to_toggle.into_iter().fuse().peekable();
                    let mut highlights_to_remove =
                        Vec::with_capacity(editor.expanded_hunks.hunks.len());
                    let mut blocks_to_remove = HashSet::default();
                    let mut hunks_to_expand = Vec::new();
                    editor.expanded_hunks.hunks.retain(|expanded_hunk| {
                        if expanded_hunk.folded {
                            return true;
                        }
                        let expanded_hunk_row_range = expanded_hunk
                            .hunk_range
                            .start
                            .to_display_point(&snapshot)
                            .row()
                            ..expanded_hunk
                                .hunk_range
                                .end
                                .to_display_point(&snapshot)
                                .row();
                        let mut retain = true;
                        while let Some(hunk_to_toggle) = hunks_to_toggle.peek() {
                            match diff_hunk_to_display(hunk_to_toggle, &snapshot) {
                                DisplayDiffHunk::Folded { .. } => {
                                    hunks_to_toggle.next();
                                    continue;
                                }
                                DisplayDiffHunk::Unfolded {
                                    diff_base_byte_range,
                                    display_row_range,
                                    multi_buffer_range,
                                    status,
                                } => {
                                    let hunk_to_toggle_row_range = display_row_range;
                                    if hunk_to_toggle_row_range.start > expanded_hunk_row_range.end
                                    {
                                        break;
                                    } else if expanded_hunk_row_range == hunk_to_toggle_row_range {
                                        highlights_to_remove.push(expanded_hunk.hunk_range.clone());
                                        blocks_to_remove.extend(expanded_hunk.block);
                                        hunks_to_toggle.next();
                                        retain = false;
                                        break;
                                    } else {
                                        hunks_to_expand.push(HunkToExpand {
                                            status,
                                            multi_buffer_range,
                                            diff_base_byte_range,
                                        });
                                        hunks_to_toggle.next();
                                        continue;
                                    }
                                }
                            }
                        }

                        retain
                    });
                    for remaining_hunk in hunks_to_toggle {
                        let remaining_hunk_point_range =
                            Point::new(remaining_hunk.associated_range.start.0, 0)
                                ..Point::new(remaining_hunk.associated_range.end.0, 0);
                        hunks_to_expand.push(HunkToExpand {
                            status: hunk_status(&remaining_hunk),
                            multi_buffer_range: remaining_hunk_point_range
                                .to_anchors(&snapshot.buffer_snapshot),
                            diff_base_byte_range: remaining_hunk.diff_base_byte_range.clone(),
                        });
                    }

                    for removed_rows in highlights_to_remove {
                        editor.highlight_rows::<DiffRowHighlight>(
                            to_inclusive_row_range(removed_rows, &snapshot),
                            None,
                            false,
                            cx,
                        );
                    }
                    editor.remove_blocks(blocks_to_remove, None, cx);
                    for hunk in hunks_to_expand {
                        editor.expand_diff_hunk(None, &hunk, cx);
                    }
                    cx.notify();
                })
                .ok();
        });

        self.expanded_hunks
            .hunk_update_tasks
            .insert(None, cx.background_executor().spawn(new_toggle_task));
    }

    pub(super) fn expand_diff_hunk(
        &mut self,
        diff_base_buffer: Option<Model<Buffer>>,
        hunk: &HunkToExpand,
        cx: &mut ViewContext<'_, Editor>,
    ) -> Option<()> {
        let multi_buffer_snapshot = self.buffer().read(cx).snapshot(cx);
        let multi_buffer_row_range = hunk
            .multi_buffer_range
            .start
            .to_point(&multi_buffer_snapshot)
            ..hunk.multi_buffer_range.end.to_point(&multi_buffer_snapshot);
        let hunk_start = hunk.multi_buffer_range.start;
        let hunk_end = hunk.multi_buffer_range.end;

        let buffer = self.buffer().clone();
        let snapshot = self.snapshot(cx);
        let (diff_base_buffer, deleted_text_lines) = buffer.update(cx, |buffer, cx| {
            let hunk = buffer_diff_hunk(&snapshot.buffer_snapshot, multi_buffer_row_range.clone())?;
            let mut buffer_ranges = buffer.range_to_buffer_ranges(multi_buffer_row_range, cx);
            if buffer_ranges.len() == 1 {
                let (buffer, _, _) = buffer_ranges.pop()?;
                let diff_base_buffer = diff_base_buffer
                    .or_else(|| self.current_diff_base_buffer(&buffer, cx))
                    .or_else(|| create_diff_base_buffer(&buffer, cx))?;
                let buffer = buffer.read(cx);
                let deleted_text_lines = buffer.diff_base().map(|diff_base| {
                    let diff_start_row = diff_base
                        .offset_to_point(hunk.diff_base_byte_range.start)
                        .row;
                    let diff_end_row = diff_base.offset_to_point(hunk.diff_base_byte_range.end).row;
                    let line_count = diff_end_row - diff_start_row;
                    line_count as u8
                })?;
                Some((diff_base_buffer, deleted_text_lines))
            } else {
                None
            }
        })?;

        let block_insert_index = match self.expanded_hunks.hunks.binary_search_by(|probe| {
            probe
                .hunk_range
                .start
                .cmp(&hunk_start, &multi_buffer_snapshot)
        }) {
            Ok(_already_present) => return None,
            Err(ix) => ix,
        };

        let block = match hunk.status {
            DiffHunkStatus::Removed => {
                self.insert_deleted_text_block(diff_base_buffer, deleted_text_lines, &hunk, cx)
            }
            DiffHunkStatus::Added => {
                self.highlight_rows::<DiffRowHighlight>(
                    to_inclusive_row_range(hunk_start..hunk_end, &snapshot),
                    Some(added_hunk_color(cx)),
                    false,
                    cx,
                );
                None
            }
            DiffHunkStatus::Modified => {
                self.highlight_rows::<DiffRowHighlight>(
                    to_inclusive_row_range(hunk_start..hunk_end, &snapshot),
                    Some(added_hunk_color(cx)),
                    false,
                    cx,
                );
                self.insert_deleted_text_block(diff_base_buffer, deleted_text_lines, &hunk, cx)
            }
        };
        self.expanded_hunks.hunks.insert(
            block_insert_index,
            ExpandedHunk {
                block,
                hunk_range: hunk_start..hunk_end,
                status: hunk.status,
                folded: false,
                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
            },
        );

        Some(())
    }

    fn insert_deleted_text_block(
        &mut self,
        diff_base_buffer: Model<Buffer>,
        deleted_text_height: u8,
        hunk: &HunkToExpand,
        cx: &mut ViewContext<'_, Self>,
    ) -> Option<BlockId> {
        let deleted_hunk_color = deleted_hunk_color(cx);
        let (editor_height, editor_with_deleted_text) =
            editor_with_deleted_text(diff_base_buffer, deleted_hunk_color, hunk, cx);
        let editor_model = cx.model().clone();
        let mut new_block_ids = self.insert_blocks(
            Some(BlockProperties {
                position: hunk.multi_buffer_range.start,
                height: editor_height.max(deleted_text_height),
                style: BlockStyle::Flex,
                render: Box::new(move |cx| {
                    let gutter_dimensions = editor_model.read(cx).gutter_dimensions;
                    div()
                        .bg(deleted_hunk_color)
                        .size_full()
                        .pl(gutter_dimensions.width + gutter_dimensions.margin)
                        .child(editor_with_deleted_text.clone())
                        .into_any_element()
                }),
                disposition: BlockDisposition::Above,
            }),
            None,
            cx,
        );
        if new_block_ids.len() == 1 {
            new_block_ids.pop()
        } else {
            debug_panic!(
                "Inserted one editor block but did not receive exactly one block id: {new_block_ids:?}"
            );
            None
        }
    }

    pub(super) fn clear_expanded_diff_hunks(&mut self, cx: &mut ViewContext<'_, Editor>) {
        self.expanded_hunks.hunk_update_tasks.clear();
        let to_remove = self
            .expanded_hunks
            .hunks
            .drain(..)
            .filter_map(|expanded_hunk| expanded_hunk.block)
            .collect();
        self.clear_row_highlights::<DiffRowHighlight>();
        self.remove_blocks(to_remove, None, cx);
    }

    pub(super) fn sync_expanded_diff_hunks(
        &mut self,
        buffer: Model<Buffer>,
        cx: &mut ViewContext<'_, Self>,
    ) {
        let buffer_id = buffer.read(cx).remote_id();
        let buffer_diff_base_version = buffer.read(cx).diff_base_version();
        self.expanded_hunks
            .hunk_update_tasks
            .remove(&Some(buffer_id));
        let diff_base_buffer = self.current_diff_base_buffer(&buffer, cx);
        let new_sync_task = cx.spawn(move |editor, mut cx| async move {
            let diff_base_buffer_unchanged = diff_base_buffer.is_some();
            let Ok(diff_base_buffer) =
                cx.update(|cx| diff_base_buffer.or_else(|| create_diff_base_buffer(&buffer, cx)))
            else {
                return;
            };
            editor
                .update(&mut cx, |editor, cx| {
                    if let Some(diff_base_buffer) = &diff_base_buffer {
                        editor.expanded_hunks.diff_base.insert(
                            buffer_id,
                            DiffBaseBuffer {
                                buffer: diff_base_buffer.clone(),
                                diff_base_version: buffer_diff_base_version,
                            },
                        );
                    }

                    let snapshot = editor.snapshot(cx);
                    let mut recalculated_hunks = snapshot
                        .buffer_snapshot
                        .git_diff_hunks_in_range(MultiBufferRow::MIN..MultiBufferRow::MAX)
                        .filter(|hunk| hunk.buffer_id == buffer_id)
                        .fuse()
                        .peekable();
                    let mut highlights_to_remove =
                        Vec::with_capacity(editor.expanded_hunks.hunks.len());
                    let mut blocks_to_remove = HashSet::default();
                    let mut hunks_to_reexpand =
                        Vec::with_capacity(editor.expanded_hunks.hunks.len());
                    editor.expanded_hunks.hunks.retain_mut(|expanded_hunk| {
                        if expanded_hunk.hunk_range.start.buffer_id != Some(buffer_id) {
                            return true;
                        };

                        let mut retain = false;
                        if diff_base_buffer_unchanged {
                            let expanded_hunk_display_range = expanded_hunk
                                .hunk_range
                                .start
                                .to_display_point(&snapshot)
                                .row()
                                ..expanded_hunk
                                    .hunk_range
                                    .end
                                    .to_display_point(&snapshot)
                                    .row();
                            while let Some(buffer_hunk) = recalculated_hunks.peek() {
                                match diff_hunk_to_display(&buffer_hunk, &snapshot) {
                                    DisplayDiffHunk::Folded { display_row } => {
                                        recalculated_hunks.next();
                                        if !expanded_hunk.folded
                                            && expanded_hunk_display_range
                                                .to_inclusive()
                                                .contains(&display_row)
                                        {
                                            retain = true;
                                            expanded_hunk.folded = true;
                                            highlights_to_remove
                                                .push(expanded_hunk.hunk_range.clone());
                                            if let Some(block) = expanded_hunk.block.take() {
                                                blocks_to_remove.insert(block);
                                            }
                                            break;
                                        } else {
                                            continue;
                                        }
                                    }
                                    DisplayDiffHunk::Unfolded {
                                        diff_base_byte_range,
                                        display_row_range,
                                        multi_buffer_range,
                                        status,
                                    } => {
                                        let hunk_display_range = display_row_range;
                                        if expanded_hunk_display_range.start
                                            > hunk_display_range.end
                                        {
                                            recalculated_hunks.next();
                                            continue;
                                        } else if expanded_hunk_display_range.end
                                            < hunk_display_range.start
                                        {
                                            break;
                                        } else {
                                            if !expanded_hunk.folded
                                                && expanded_hunk_display_range == hunk_display_range
                                                && expanded_hunk.status == hunk_status(buffer_hunk)
                                                && expanded_hunk.diff_base_byte_range
                                                    == buffer_hunk.diff_base_byte_range
                                            {
                                                recalculated_hunks.next();
                                                retain = true;
                                            } else {
                                                hunks_to_reexpand.push(HunkToExpand {
                                                    status,
                                                    multi_buffer_range,
                                                    diff_base_byte_range,
                                                });
                                            }
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if !retain {
                            blocks_to_remove.extend(expanded_hunk.block);
                            highlights_to_remove.push(expanded_hunk.hunk_range.clone());
                        }
                        retain
                    });

                    for removed_rows in highlights_to_remove {
                        editor.highlight_rows::<DiffRowHighlight>(
                            to_inclusive_row_range(removed_rows, &snapshot),
                            None,
                            false,
                            cx,
                        );
                    }
                    editor.remove_blocks(blocks_to_remove, None, cx);

                    if let Some(diff_base_buffer) = &diff_base_buffer {
                        for hunk in hunks_to_reexpand {
                            editor.expand_diff_hunk(Some(diff_base_buffer.clone()), &hunk, cx);
                        }
                    }
                })
                .ok();
        });

        self.expanded_hunks.hunk_update_tasks.insert(
            Some(buffer_id),
            cx.background_executor().spawn(new_sync_task),
        );
    }

    fn current_diff_base_buffer(
        &mut self,
        buffer: &Model<Buffer>,
        cx: &mut AppContext,
    ) -> Option<Model<Buffer>> {
        buffer.update(cx, |buffer, _| {
            match self.expanded_hunks.diff_base.entry(buffer.remote_id()) {
                hash_map::Entry::Occupied(o) => {
                    if o.get().diff_base_version != buffer.diff_base_version() {
                        o.remove();
                        None
                    } else {
                        Some(o.get().buffer.clone())
                    }
                }
                hash_map::Entry::Vacant(_) => None,
            }
        })
    }
}

fn create_diff_base_buffer(buffer: &Model<Buffer>, cx: &mut AppContext) -> Option<Model<Buffer>> {
    buffer
        .update(cx, |buffer, _| {
            let language = buffer.language().cloned();
            let diff_base = buffer.diff_base()?.clone();
            Some((buffer.line_ending(), diff_base, language))
        })
        .map(|(line_ending, diff_base, language)| {
            cx.new_model(|cx| {
                let buffer = Buffer::local_normalized(diff_base, line_ending, cx);
                match language {
                    Some(language) => buffer.with_language(language, cx),
                    None => buffer,
                }
            })
        })
}

fn added_hunk_color(cx: &AppContext) -> Hsla {
    let mut created_color = cx.theme().status().git().created;
    created_color.fade_out(0.7);
    created_color
}

fn deleted_hunk_color(cx: &AppContext) -> Hsla {
    let mut deleted_color = cx.theme().status().git().deleted;
    deleted_color.fade_out(0.7);
    deleted_color
}

fn editor_with_deleted_text(
    diff_base_buffer: Model<Buffer>,
    deleted_color: Hsla,
    hunk: &HunkToExpand,
    cx: &mut ViewContext<'_, Editor>,
) -> (u8, View<Editor>) {
    let parent_editor = cx.view().downgrade();
    let editor = cx.new_view(|cx| {
        let multi_buffer =
            cx.new_model(|_| MultiBuffer::without_headers(0, language::Capability::ReadOnly));
        multi_buffer.update(cx, |multi_buffer, cx| {
            multi_buffer.push_excerpts(
                diff_base_buffer,
                Some(ExcerptRange {
                    context: hunk.diff_base_byte_range.clone(),
                    primary: None,
                }),
                cx,
            );
        });

        let mut editor = Editor::for_multibuffer(multi_buffer, None, true, cx);
        editor.soft_wrap_mode_override = Some(language::language_settings::SoftWrap::None);
        editor.show_wrap_guides = Some(false);
        editor.show_gutter = false;
        editor.scroll_manager.set_forbid_vertical_scroll(true);
        editor.set_read_only(true);

        let editor_snapshot = editor.snapshot(cx);
        let start = editor_snapshot.buffer_snapshot.anchor_before(0);
        let end = editor_snapshot
            .buffer_snapshot
            .anchor_after(editor.buffer.read(cx).len(cx));

        editor.highlight_rows::<DiffRowHighlight>(start..=end, Some(deleted_color), false, cx);

        let subscription_editor = parent_editor.clone();
        editor._subscriptions.extend([
            cx.on_blur(&editor.focus_handle, |editor, cx| {
                editor.set_current_line_highlight(CurrentLineHighlight::None);
                editor.change_selections(None, cx, |s| {
                    s.try_cancel();
                });
                cx.notify();
            }),
            cx.on_focus(&editor.focus_handle, move |editor, cx| {
                let restored_highlight = if let Some(parent_editor) = subscription_editor.upgrade()
                {
                    parent_editor.read(cx).current_line_highlight
                } else {
                    EditorSettings::get_global(cx).current_line_highlight
                };
                editor.set_current_line_highlight(restored_highlight);
                cx.notify();
            }),
            cx.observe_global::<SettingsStore>(|editor, cx| {
                if !editor.is_focused(cx) {
                    editor.set_current_line_highlight(CurrentLineHighlight::None);
                }
            }),
        ]);
        let original_multi_buffer_range = hunk.multi_buffer_range.clone();
        let diff_base_range = hunk.diff_base_byte_range.clone();
        editor.register_action::<RevertSelectedHunks>(move |_, cx| {
            parent_editor
                .update(cx, |editor, cx| {
                    let Some((buffer, original_text)) = editor.buffer().update(cx, |buffer, cx| {
                        let (_, buffer, _) =
                            buffer.excerpt_containing(original_multi_buffer_range.start, cx)?;
                        let original_text =
                            buffer.read(cx).diff_base()?.slice(diff_base_range.clone());
                        Some((buffer, Arc::from(original_text.to_string())))
                    }) else {
                        return;
                    };
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit(
                            Some((
                                original_multi_buffer_range.start.text_anchor
                                    ..original_multi_buffer_range.end.text_anchor,
                                original_text,
                            )),
                            None,
                            cx,
                        )
                    });
                })
                .ok();
        });
        editor
    });

    let editor_height = editor.update(cx, |editor, cx| editor.max_point(cx).row().0 as u8);
    (editor_height, editor)
}

fn buffer_diff_hunk(
    buffer_snapshot: &MultiBufferSnapshot,
    row_range: Range<Point>,
) -> Option<DiffHunk<MultiBufferRow>> {
    let mut hunks = buffer_snapshot.git_diff_hunks_in_range(
        MultiBufferRow(row_range.start.row)..MultiBufferRow(row_range.end.row),
    );
    let hunk = hunks.next()?;
    let second_hunk = hunks.next();
    if second_hunk.is_none() {
        return Some(hunk);
    }
    None
}

fn to_inclusive_row_range(
    row_range: Range<Anchor>,
    snapshot: &EditorSnapshot,
) -> RangeInclusive<Anchor> {
    let mut display_row_range =
        row_range.start.to_display_point(snapshot)..row_range.end.to_display_point(snapshot);
    if display_row_range.end.row() > display_row_range.start.row() {
        *display_row_range.end.row_mut() -= 1;
    }
    let point_range = display_row_range.start.to_point(&snapshot.display_snapshot)
        ..display_row_range.end.to_point(&snapshot.display_snapshot);
    let new_range = point_range.to_anchors(&snapshot.buffer_snapshot);
    new_range.start..=new_range.end
}
