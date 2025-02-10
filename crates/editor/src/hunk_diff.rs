use collections::{HashMap, HashSet};
use git::diff::DiffHunkStatus;
use gpui::{
    Action, AppContext, Corner, CursorStyle, Focusable as _, Hsla, Model, MouseButton,
    Subscription, Task,
};
use language::{Buffer, BufferId, Point};
use multi_buffer::{
    Anchor, AnchorRangeExt, ExcerptRange, MultiBuffer, MultiBufferDiffHunk, MultiBufferRow,
    MultiBufferSnapshot, ToOffset, ToPoint,
};
use project::buffer_store::BufferChangeSet;
use std::{ops::Range, sync::Arc};
use sum_tree::TreeMap;
use text::OffsetRangeExt;
use ui::{
    prelude::*, ActiveTheme, Context, Context, ContextMenu, IconButtonShape, InteractiveElement,
    IntoElement, ParentElement, PopoverMenu, Styled, Tooltip, Window,
};
use util::RangeExt;
use workspace::Item;

use crate::{
    editor_settings::CurrentLineHighlight, hunk_status, hunks_for_selections, ApplyAllDiffHunks,
    ApplyDiffHunk, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId, DiffRowHighlight,
    DisplayRow, DisplaySnapshot, Editor, EditorElement, ExpandAllHunkDiffs, GoToHunk, GoToPrevHunk,
    RevertFile, RevertSelectedHunks, ToDisplayPoint, ToggleHunkDiff,
};

#[derive(Debug, Clone)]
pub(super) struct HoveredHunk {
    pub multi_buffer_range: Range<Anchor>,
    pub status: DiffHunkStatus,
    pub diff_base_byte_range: Range<usize>,
}

#[derive(Default)]
pub(super) struct DiffMap {
    pub(crate) hunks: Vec<ExpandedHunk>,
    pub(crate) diff_bases: HashMap<BufferId, DiffBaseState>,
    pub(crate) snapshot: DiffMapSnapshot,
    hunk_update_tasks: HashMap<Option<BufferId>, Task<()>>,
    expand_all: bool,
}

#[derive(Debug, Clone)]
pub(super) struct ExpandedHunk {
    pub blocks: Vec<CustomBlockId>,
    pub hunk_range: Range<Anchor>,
    pub diff_base_byte_range: Range<usize>,
    pub status: DiffHunkStatus,
    pub folded: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct DiffMapSnapshot(TreeMap<BufferId, git::diff::BufferDiff>);

pub(crate) struct DiffBaseState {
    pub(crate) diff: Model<BufferChangeSet>,
    pub(crate) last_version: Option<usize>,
    _subscription: Subscription,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayDiffHunk {
    Folded {
        display_row: DisplayRow,
    },

    Unfolded {
        diff_base_byte_range: Range<usize>,
        display_row_range: Range<DisplayRow>,
        multi_buffer_range: Range<Anchor>,
        status: DiffHunkStatus,
    },
}

impl DiffMap {
    pub fn snapshot(&self) -> DiffMapSnapshot {
        self.snapshot.clone()
    }

    pub fn add_diff(
        &mut self,
        diff: Model<BufferChangeSet>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let buffer_id = diff.read(cx).buffer_id;
        self.snapshot
            .0
            .insert(buffer_id, diff.read(cx).diff_to_buffer.clone());
        self.diff_bases.insert(
            buffer_id,
            DiffBaseState {
                last_version: None,
                _subscription: cx.observe_in(&diff, window, move |editor, diff, window, cx| {
                    editor
                        .diff_map
                        .snapshot
                        .0
                        .insert(buffer_id, diff.read(cx).diff_to_buffer.clone());
                    Editor::sync_expanded_diff_hunks(&mut editor.diff_map, buffer_id, window, cx);
                }),
                diff,
            },
        );
        Editor::sync_expanded_diff_hunks(self, buffer_id, window, cx);
    }

    pub fn hunks(&self, include_folded: bool) -> impl Iterator<Item = &ExpandedHunk> {
        self.hunks
            .iter()
            .filter(move |hunk| include_folded || !hunk.folded)
    }
}

impl DiffMapSnapshot {
    pub fn is_empty(&self) -> bool {
        self.0.values().all(|diff| diff.is_empty())
    }

    pub fn diff_hunks<'a>(
        &'a self,
        buffer_snapshot: &'a MultiBufferSnapshot,
    ) -> impl Iterator<Item = MultiBufferDiffHunk> + 'a {
        self.diff_hunks_in_range(0..buffer_snapshot.len(), buffer_snapshot)
    }

    pub fn diff_hunks_in_range<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        buffer_snapshot: &'a MultiBufferSnapshot,
    ) -> impl Iterator<Item = MultiBufferDiffHunk> + 'a {
        let range = range.start.to_offset(buffer_snapshot)..range.end.to_offset(buffer_snapshot);
        buffer_snapshot
            .excerpts_for_range(range.clone())
            .filter_map(move |excerpt| {
                let buffer = excerpt.buffer();
                let buffer_id = buffer.remote_id();
                let diff = self.0.get(&buffer_id)?;
                let buffer_range = excerpt.map_range_to_buffer(range.clone());
                let buffer_range =
                    buffer.anchor_before(buffer_range.start)..buffer.anchor_after(buffer_range.end);
                Some(
                    diff.hunks_intersecting_range(buffer_range, excerpt.buffer())
                        .map(move |hunk| {
                            let start =
                                excerpt.map_point_from_buffer(Point::new(hunk.row_range.start, 0));
                            let end =
                                excerpt.map_point_from_buffer(Point::new(hunk.row_range.end, 0));
                            MultiBufferDiffHunk {
                                row_range: MultiBufferRow(start.row)..MultiBufferRow(end.row),
                                buffer_id,
                                buffer_range: hunk.buffer_range.clone(),
                                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                            }
                        }),
                )
            })
            .flatten()
    }

    pub fn diff_hunks_in_range_rev<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        buffer_snapshot: &'a MultiBufferSnapshot,
    ) -> impl Iterator<Item = MultiBufferDiffHunk> + 'a {
        let range = range.start.to_offset(buffer_snapshot)..range.end.to_offset(buffer_snapshot);
        buffer_snapshot
            .excerpts_for_range_rev(range.clone())
            .filter_map(move |excerpt| {
                let buffer = excerpt.buffer();
                let buffer_id = buffer.remote_id();
                let diff = self.0.get(&buffer_id)?;
                let buffer_range = excerpt.map_range_to_buffer(range.clone());
                let buffer_range =
                    buffer.anchor_before(buffer_range.start)..buffer.anchor_after(buffer_range.end);
                Some(
                    diff.hunks_intersecting_range_rev(buffer_range, excerpt.buffer())
                        .map(move |hunk| {
                            let start_row = excerpt
                                .map_point_from_buffer(Point::new(hunk.row_range.start, 0))
                                .row;
                            let end_row = excerpt
                                .map_point_from_buffer(Point::new(hunk.row_range.end, 0))
                                .row;
                            MultiBufferDiffHunk {
                                row_range: MultiBufferRow(start_row)..MultiBufferRow(end_row),
                                buffer_id,
                                buffer_range: hunk.buffer_range.clone(),
                                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                            }
                        }),
                )
            })
            .flatten()
    }
}

impl Editor {
    pub fn set_expand_all_diff_hunks(&mut self) {
        self.diff_map.expand_all = true;
    }

    pub(super) fn toggle_hovered_hunk(
        &mut self,
        hovered_hunk: &HoveredHunk,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let editor_snapshot = self.snapshot(window, cx);
        if let Some(diff_hunk) = to_diff_hunk(hovered_hunk, &editor_snapshot.buffer_snapshot) {
            self.toggle_hunks_expanded(vec![diff_hunk], window, cx);
            self.change_selections(None, window, cx, |selections| selections.refresh());
        }
    }

    pub fn toggle_hunk_diff(
        &mut self,
        _: &ToggleHunkDiff,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot(window, cx);
        let selections = self.selections.all(cx);
        self.toggle_hunks_expanded(hunks_for_selections(&snapshot, &selections), window, cx);
    }

    pub fn expand_all_hunk_diffs(
        &mut self,
        _: &ExpandAllHunkDiffs,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot(window, cx);
        let display_rows_with_expanded_hunks = self
            .diff_map
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
        let hunks = self
            .diff_map
            .snapshot
            .diff_hunks(&snapshot.display_snapshot.buffer_snapshot)
            .filter(|hunk| {
                let hunk_display_row_range = Point::new(hunk.row_range.start.0, 0)
                    .to_display_point(&snapshot.display_snapshot)
                    ..Point::new(hunk.row_range.end.0, 0)
                        .to_display_point(&snapshot.display_snapshot);
                let row_range_end =
                    display_rows_with_expanded_hunks.get(&hunk_display_row_range.start.row());
                row_range_end.is_none() || row_range_end != Some(&hunk_display_row_range.end.row())
            });
        self.toggle_hunks_expanded(hunks.collect(), window, cx);
    }

    fn toggle_hunks_expanded(
        &mut self,
        hunks_to_toggle: Vec<MultiBufferDiffHunk>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.diff_map.expand_all {
            return;
        }

        let previous_toggle_task = self.diff_map.hunk_update_tasks.remove(&None);
        let new_toggle_task = cx.spawn_in(window, move |editor, mut cx| async move {
            if let Some(task) = previous_toggle_task {
                task.await;
            }

            editor
                .update_in(&mut cx, |editor, window, cx| {
                    let snapshot = editor.snapshot(window, cx);
                    let mut hunks_to_toggle = hunks_to_toggle.into_iter().fuse().peekable();
                    let mut highlights_to_remove = Vec::with_capacity(editor.diff_map.hunks.len());
                    let mut blocks_to_remove = HashSet::default();
                    let mut hunks_to_expand = Vec::new();
                    editor.diff_map.hunks.retain(|expanded_hunk| {
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
                                        blocks_to_remove
                                            .extend(expanded_hunk.blocks.iter().copied());
                                        hunks_to_toggle.next();
                                        retain = false;
                                        break;
                                    } else {
                                        hunks_to_expand.push(HoveredHunk {
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
                    for hunk in hunks_to_toggle {
                        let remaining_hunk_point_range = Point::new(hunk.row_range.start.0, 0)
                            ..Point::new(hunk.row_range.end.0, 0);
                        let hunk_start = snapshot
                            .buffer_snapshot
                            .anchor_before(remaining_hunk_point_range.start);
                        let hunk_end = snapshot
                            .buffer_snapshot
                            .anchor_in_excerpt(hunk_start.excerpt_id, hunk.buffer_range.end)
                            .unwrap();
                        hunks_to_expand.push(HoveredHunk {
                            status: hunk_status(&hunk),
                            multi_buffer_range: hunk_start..hunk_end,
                            diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                        });
                    }

                    editor.remove_highlighted_rows::<DiffRowHighlight>(highlights_to_remove, cx);
                    editor.remove_blocks(blocks_to_remove, None, cx);
                    for hunk in hunks_to_expand {
                        editor.expand_diff_hunk(None, &hunk, window, cx);
                    }
                    cx.notify();
                })
                .ok();
        });

        self.diff_map
            .hunk_update_tasks
            .insert(None, cx.background_executor().spawn(new_toggle_task));
    }

    pub(super) fn expand_diff_hunk(
        &mut self,
        diff_base_buffer: Option<Model<Buffer>>,
        hunk: &HoveredHunk,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<()> {
        let buffer = self.buffer.clone();
        let multi_buffer_snapshot = buffer.read(cx).snapshot(cx);
        let hunk_range = hunk.multi_buffer_range.clone();
        let buffer_id = hunk_range.start.buffer_id?;
        let diff_base_buffer = diff_base_buffer.or_else(|| {
            self.diff_map
                .diff_bases
                .get(&buffer_id)?
                .diff
                .read(cx)
                .base_text
                .clone()
        })?;

        let diff_base = diff_base_buffer.read(cx);
        let diff_start_row = diff_base
            .offset_to_point(hunk.diff_base_byte_range.start)
            .row;
        let diff_end_row = diff_base.offset_to_point(hunk.diff_base_byte_range.end).row;
        let deleted_text_lines = diff_end_row - diff_start_row;

        let block_insert_index = self
            .diff_map
            .hunks
            .binary_search_by(|probe| {
                probe
                    .hunk_range
                    .start
                    .cmp(&hunk_range.start, &multi_buffer_snapshot)
            })
            .err()?;

        let blocks;
        match hunk.status {
            DiffHunkStatus::Removed => {
                blocks = self.insert_blocks(
                    [
                        self.hunk_header_block(&hunk, cx),
                        Self::deleted_text_block(
                            hunk,
                            diff_base_buffer,
                            deleted_text_lines,
                            window,
                            cx,
                        ),
                    ],
                    None,
                    cx,
                );
            }
            DiffHunkStatus::Added => {
                self.highlight_rows::<DiffRowHighlight>(
                    hunk_range.clone(),
                    added_hunk_color(cx),
                    false,
                    cx,
                );
                blocks = self.insert_blocks([self.hunk_header_block(&hunk, cx)], None, cx);
            }
            DiffHunkStatus::Modified => {
                self.highlight_rows::<DiffRowHighlight>(
                    hunk_range.clone(),
                    added_hunk_color(cx),
                    false,
                    cx,
                );
                blocks = self.insert_blocks(
                    [
                        self.hunk_header_block(&hunk, cx),
                        Self::deleted_text_block(
                            hunk,
                            diff_base_buffer,
                            deleted_text_lines,
                            window,
                            cx,
                        ),
                    ],
                    None,
                    cx,
                );
            }
        };
        self.diff_map.hunks.insert(
            block_insert_index,
            ExpandedHunk {
                blocks,
                hunk_range,
                status: hunk.status,
                folded: false,
                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
            },
        );

        Some(())
    }

    fn apply_diff_hunks_in_range(
        &mut self,
        range: Range<Anchor>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<()> {
        let multi_buffer = self.buffer.read(cx);
        let multi_buffer_snapshot = multi_buffer.snapshot(cx);
        let (excerpt, range) = multi_buffer_snapshot
            .range_to_buffer_ranges(range)
            .into_iter()
            .next()?;

        multi_buffer
            .buffer(excerpt.buffer_id())
            .unwrap()
            .update(cx, |branch_buffer, cx| {
                branch_buffer.merge_into_base(vec![range], cx);
            });

        if let Some(project) = self.project.clone() {
            self.save(true, project, window, cx).detach_and_log_err(cx);
        }

        None
    }

    pub(crate) fn apply_all_diff_hunks(
        &mut self,
        _: &ApplyAllDiffHunks,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffers = self.buffer.read(cx).all_buffers();
        for branch_buffer in buffers {
            branch_buffer.update(cx, |branch_buffer, cx| {
                branch_buffer.merge_into_base(Vec::new(), cx);
            });
        }

        if let Some(project) = self.project.clone() {
            self.save(true, project, window, cx).detach_and_log_err(cx);
        }
    }

    pub(crate) fn apply_selected_diff_hunks(
        &mut self,
        _: &ApplyDiffHunk,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot(window, cx);
        let hunks = hunks_for_selections(&snapshot, &self.selections.all(cx));
        let mut ranges_by_buffer = HashMap::default();
        self.transact(window, cx, |editor, _, cx| {
            for hunk in hunks {
                if let Some(buffer) = editor.buffer.read(cx).buffer(hunk.buffer_id) {
                    ranges_by_buffer
                        .entry(buffer.clone())
                        .or_insert_with(Vec::new)
                        .push(hunk.buffer_range.to_offset(buffer.read(cx)));
                }
            }

            for (buffer, ranges) in ranges_by_buffer {
                buffer.update(cx, |buffer, cx| {
                    buffer.merge_into_base(ranges, cx);
                });
            }
        });

        if let Some(project) = self.project.clone() {
            self.save(true, project, window, cx).detach_and_log_err(cx);
        }
    }

    fn has_multiple_hunks(&self, cx: &AppContext) -> bool {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let mut hunks = self.diff_map.snapshot.diff_hunks(&snapshot);
        hunks.nth(1).is_some()
    }

    fn hunk_header_block(
        &self,
        hunk: &HoveredHunk,
        cx: &mut Context<Editor>,
    ) -> BlockProperties<Anchor> {
        let is_branch_buffer = self
            .buffer
            .read(cx)
            .point_to_buffer_offset(hunk.multi_buffer_range.start, cx)
            .map_or(false, |(buffer, _, _)| {
                buffer.read(cx).base_buffer().is_some()
            });

        let border_color = cx.theme().colors().border_variant;
        let bg_color = cx.theme().colors().editor_background;
        let gutter_color = match hunk.status {
            DiffHunkStatus::Added => cx.theme().status().created,
            DiffHunkStatus::Modified => cx.theme().status().modified,
            DiffHunkStatus::Removed => cx.theme().status().deleted,
        };

        BlockProperties {
            placement: BlockPlacement::Above(hunk.multi_buffer_range.start),
            height: 1,
            style: BlockStyle::Sticky,
            priority: 0,
            render: Arc::new({
                let editor = cx.entity().clone();
                let hunk = hunk.clone();
                let has_multiple_hunks = self.has_multiple_hunks(cx);

                move |cx| {
                    let hunk_controls_menu_handle =
                        editor.read(cx).hunk_controls_menu_handle.clone();

                    h_flex()
                        .id(cx.block_id)
                        .block_mouse_down()
                        .h(cx.window.line_height())
                        .w_full()
                        .border_t_1()
                        .border_color(border_color)
                        .bg(bg_color)
                        .child(
                            div()
                                .id("gutter-strip")
                                .w(EditorElement::diff_hunk_strip_width(
                                    cx.window.line_height(),
                                ))
                                .h_full()
                                .bg(gutter_color)
                                .cursor(CursorStyle::PointingHand)
                                .on_click({
                                    let editor = editor.clone();
                                    let hunk = hunk.clone();
                                    move |_event, window, cx| {
                                        editor.update(cx, |editor, cx| {
                                            editor.toggle_hovered_hunk(&hunk, window, cx);
                                        });
                                    }
                                }),
                        )
                        .child(
                            h_flex()
                                .px_6()
                                .size_full()
                                .justify_end()
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .when(!is_branch_buffer, |row| {
                                            row.child(
                                                IconButton::new("next-hunk", IconName::ArrowDown)
                                                    .shape(IconButtonShape::Square)
                                                    .icon_size(IconSize::Small)
                                                    .disabled(!has_multiple_hunks)
                                                    .tooltip({
                                                        let focus_handle = editor.focus_handle(cx);
                                                        move |window, cx| {
                                                            Tooltip::for_action_in(
                                                                "Next Hunk",
                                                                &GoToHunk,
                                                                &focus_handle,
                                                                window,
                                                                cx,
                                                            )
                                                        }
                                                    })
                                                    .on_click({
                                                        let editor = editor.clone();
                                                        let hunk = hunk.clone();
                                                        move |_event, window, cx| {
                                                            editor.update(cx, |editor, cx| {
                                                                editor.go_to_subsequent_hunk(
                                                                    hunk.multi_buffer_range.end,
                                                                    window,
                                                                    cx,
                                                                );
                                                            });
                                                        }
                                                    }),
                                            )
                                            .child(
                                                IconButton::new("prev-hunk", IconName::ArrowUp)
                                                    .shape(IconButtonShape::Square)
                                                    .icon_size(IconSize::Small)
                                                    .disabled(!has_multiple_hunks)
                                                    .tooltip({
                                                        let focus_handle = editor.focus_handle(cx);
                                                        move |window, cx| {
                                                            Tooltip::for_action_in(
                                                                "Previous Hunk",
                                                                &GoToPrevHunk,
                                                                &focus_handle,
                                                                window,
                                                                cx,
                                                            )
                                                        }
                                                    })
                                                    .on_click({
                                                        let editor = editor.clone();
                                                        let hunk = hunk.clone();
                                                        move |_event, window, cx| {
                                                            editor.update(cx, |editor, cx| {
                                                                editor.go_to_preceding_hunk(
                                                                    hunk.multi_buffer_range.start,
                                                                    window,
                                                                    cx,
                                                                );
                                                            });
                                                        }
                                                    }),
                                            )
                                        })
                                        .child(
                                            IconButton::new("discard", IconName::Undo)
                                                .shape(IconButtonShape::Square)
                                                .icon_size(IconSize::Small)
                                                .tooltip({
                                                    let focus_handle = editor.focus_handle(cx);
                                                    move |window, cx| {
                                                        Tooltip::for_action_in(
                                                            "Discard Hunk",
                                                            &RevertSelectedHunks,
                                                            &focus_handle,
                                                            window,
                                                            cx,
                                                        )
                                                    }
                                                })
                                                .on_click({
                                                    let editor = editor.clone();
                                                    let hunk = hunk.clone();
                                                    move |_event, window, cx| {
                                                        editor.update(cx, |editor, cx| {
                                                            editor.revert_hunk(
                                                                hunk.clone(),
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    }
                                                }),
                                        )
                                        .map(|this| {
                                            if is_branch_buffer {
                                                this.child(
                                                    IconButton::new("apply", IconName::Check)
                                                        .shape(IconButtonShape::Square)
                                                        .icon_size(IconSize::Small)
                                                        .tooltip({
                                                            let focus_handle =
                                                                editor.focus_handle(cx);
                                                            move |window, cx| {
                                                                Tooltip::for_action_in(
                                                                    "Apply Hunk",
                                                                    &ApplyDiffHunk,
                                                                    &focus_handle,
                                                                    window,
                                                                    cx,
                                                                )
                                                            }
                                                        })
                                                        .on_click({
                                                            let editor = editor.clone();
                                                            let hunk = hunk.clone();
                                                            move |_event, window, cx| {
                                                                editor.update(cx, |editor, cx| {
                                                                    editor
                                                                        .apply_diff_hunks_in_range(
                                                                            hunk.multi_buffer_range
                                                                                .clone(),
                                                                            window,
                                                                            cx,
                                                                        );
                                                                });
                                                            }
                                                        }),
                                                )
                                            } else {
                                                this.child({
                                                    let focus = editor.focus_handle(cx);
                                                    PopoverMenu::new("hunk-controls-dropdown")
                                                        .trigger_with_tooltip(
                                                            IconButton::new(
                                                                "toggle_editor_selections_icon",
                                                                IconName::EllipsisVertical,
                                                            )
                                                            .shape(IconButtonShape::Square)
                                                            .icon_size(IconSize::Small)
                                                            .style(ButtonStyle::Subtle)
                                                            .toggle_state(
                                                                hunk_controls_menu_handle
                                                                    .is_deployed(),
                                                            ),
                                                            Tooltip::simple("Hunk Controls", cx),
                                                        )
                                                        .anchor(Corner::TopRight)
                                                        .with_handle(hunk_controls_menu_handle)
                                                        .menu(move |window, cx| {
                                                            let focus = focus.clone();
                                                            let menu = ContextMenu::build(
                                                                window,
                                                                cx,
                                                                move |menu, _, _| {
                                                                    menu.context(focus.clone())
                                                                        .action(
                                                                            "Discard All Hunks",
                                                                            RevertFile
                                                                                .boxed_clone(),
                                                                        )
                                                                },
                                                            );
                                                            Some(menu)
                                                        })
                                                })
                                            }
                                        }),
                                )
                                .when(!is_branch_buffer, |div| {
                                    div.child(
                                        IconButton::new("collapse", IconName::Close)
                                            .shape(IconButtonShape::Square)
                                            .icon_size(IconSize::Small)
                                            .tooltip({
                                                let focus_handle = editor.focus_handle(cx);
                                                move |window, cx| {
                                                    Tooltip::for_action_in(
                                                        "Collapse Hunk",
                                                        &ToggleHunkDiff,
                                                        &focus_handle,
                                                        window,
                                                        cx,
                                                    )
                                                }
                                            })
                                            .on_click({
                                                let editor = editor.clone();
                                                let hunk = hunk.clone();
                                                move |_event, window, cx| {
                                                    editor.update(cx, |editor, cx| {
                                                        editor
                                                            .toggle_hovered_hunk(&hunk, window, cx);
                                                    });
                                                }
                                            }),
                                    )
                                }),
                        )
                        .into_any_element()
                }
            }),
        }
    }

    fn deleted_text_block(
        hunk: &HoveredHunk,
        diff_base_buffer: Model<Buffer>,
        deleted_text_height: u32,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> BlockProperties<Anchor> {
        let gutter_color = match hunk.status {
            DiffHunkStatus::Added => unreachable!(),
            DiffHunkStatus::Modified => cx.theme().status().modified,
            DiffHunkStatus::Removed => cx.theme().status().deleted,
        };
        let deleted_hunk_color = deleted_hunk_color(cx);
        let (editor_height, editor_with_deleted_text) =
            editor_with_deleted_text(diff_base_buffer, deleted_hunk_color, hunk, window, cx);
        let editor = cx.entity().clone();
        let hunk = hunk.clone();
        let height = editor_height.max(deleted_text_height);
        BlockProperties {
            placement: BlockPlacement::Above(hunk.multi_buffer_range.start),
            height,
            style: BlockStyle::Flex,
            priority: 0,
            render: Arc::new(move |cx| {
                let width = EditorElement::diff_hunk_strip_width(cx.window.line_height());
                let gutter_dimensions = editor.read(cx.app).gutter_dimensions;

                h_flex()
                    .id(cx.block_id)
                    .block_mouse_down()
                    .bg(deleted_hunk_color)
                    .h(height as f32 * cx.window.line_height())
                    .w_full()
                    .child(
                        h_flex()
                            .id("gutter")
                            .max_w(gutter_dimensions.full_width())
                            .min_w(gutter_dimensions.full_width())
                            .size_full()
                            .child(
                                h_flex()
                                    .id("gutter hunk")
                                    .bg(gutter_color)
                                    .pl(gutter_dimensions.margin
                                        + gutter_dimensions
                                            .git_blame_entries_width
                                            .unwrap_or_default())
                                    .max_w(width)
                                    .min_w(width)
                                    .size_full()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(MouseButton::Left, {
                                        let editor = editor.clone();
                                        let hunk = hunk.clone();
                                        move |_event, window, cx| {
                                            editor.update(cx, |editor, cx| {
                                                editor.toggle_hovered_hunk(&hunk, window, cx);
                                            });
                                        }
                                    }),
                            ),
                    )
                    .child(editor_with_deleted_text.clone())
                    .into_any_element()
            }),
        }
    }

    pub(super) fn clear_expanded_diff_hunks(&mut self, cx: &mut Context<Editor>) -> bool {
        if self.diff_map.expand_all {
            return false;
        }
        self.diff_map.hunk_update_tasks.clear();
        self.clear_row_highlights::<DiffRowHighlight>();
        let to_remove = self
            .diff_map
            .hunks
            .drain(..)
            .flat_map(|expanded_hunk| expanded_hunk.blocks.into_iter())
            .collect::<HashSet<_>>();
        if to_remove.is_empty() {
            false
        } else {
            self.remove_blocks(to_remove, None, cx);
            true
        }
    }

    pub(super) fn sync_expanded_diff_hunks(
        diff_map: &mut DiffMap,
        buffer_id: BufferId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let diff_base_state = diff_map.diff_bases.get_mut(&buffer_id);
        let mut diff_base_buffer = None;
        let mut diff_base_buffer_unchanged = true;
        if let Some(diff_base_state) = diff_base_state {
            diff_base_state.diff.update(cx, |diff, _| {
                if diff_base_state.last_version != Some(diff.base_text_version) {
                    diff_base_state.last_version = Some(diff.base_text_version);
                    diff_base_buffer_unchanged = false;
                }
                diff_base_buffer = diff.base_text.clone();
            })
        }

        diff_map.hunk_update_tasks.remove(&Some(buffer_id));

        let new_sync_task = cx.spawn_in(window, move |editor, mut cx| async move {
            editor
                .update_in(&mut cx, |editor, window, cx| {
                    let snapshot = editor.snapshot(window, cx);
                    let mut recalculated_hunks = snapshot
                        .diff_map
                        .diff_hunks(&snapshot.buffer_snapshot)
                        .filter(|hunk| hunk.buffer_id == buffer_id)
                        .fuse()
                        .peekable();
                    let mut highlights_to_remove = Vec::with_capacity(editor.diff_map.hunks.len());
                    let mut blocks_to_remove = HashSet::default();
                    let mut hunks_to_reexpand = Vec::with_capacity(editor.diff_map.hunks.len());
                    editor.diff_map.hunks.retain_mut(|expanded_hunk| {
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
                                match diff_hunk_to_display(buffer_hunk, &snapshot) {
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
                                            for block in expanded_hunk.blocks.drain(..) {
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
                                            if editor.diff_map.expand_all {
                                                hunks_to_reexpand.push(HoveredHunk {
                                                    status,
                                                    multi_buffer_range,
                                                    diff_base_byte_range,
                                                });
                                            }
                                            continue;
                                        }

                                        if expanded_hunk_display_range.end
                                            < hunk_display_range.start
                                        {
                                            break;
                                        }

                                        if !expanded_hunk.folded
                                            && expanded_hunk_display_range == hunk_display_range
                                            && expanded_hunk.status == hunk_status(buffer_hunk)
                                            && expanded_hunk.diff_base_byte_range
                                                == buffer_hunk.diff_base_byte_range
                                        {
                                            recalculated_hunks.next();
                                            retain = true;
                                        } else {
                                            hunks_to_reexpand.push(HoveredHunk {
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
                        if !retain {
                            blocks_to_remove.extend(expanded_hunk.blocks.drain(..));
                            highlights_to_remove.push(expanded_hunk.hunk_range.clone());
                        }
                        retain
                    });

                    if editor.diff_map.expand_all {
                        for hunk in recalculated_hunks {
                            match diff_hunk_to_display(&hunk, &snapshot) {
                                DisplayDiffHunk::Folded { .. } => {}
                                DisplayDiffHunk::Unfolded {
                                    diff_base_byte_range,
                                    multi_buffer_range,
                                    status,
                                    ..
                                } => {
                                    hunks_to_reexpand.push(HoveredHunk {
                                        status,
                                        multi_buffer_range,
                                        diff_base_byte_range,
                                    });
                                }
                            }
                        }
                    } else {
                        drop(recalculated_hunks);
                    }

                    editor.remove_highlighted_rows::<DiffRowHighlight>(highlights_to_remove, cx);
                    editor.remove_blocks(blocks_to_remove, None, cx);

                    if let Some(diff_base_buffer) = &diff_base_buffer {
                        for hunk in hunks_to_reexpand {
                            editor.expand_diff_hunk(
                                Some(diff_base_buffer.clone()),
                                &hunk,
                                window,
                                cx,
                            );
                        }
                    }
                })
                .ok();
        });

        diff_map.hunk_update_tasks.insert(
            Some(buffer_id),
            cx.background_executor().spawn(new_sync_task),
        );
    }

    fn go_to_subsequent_hunk(
        &mut self,
        position: Anchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot(window, cx);
        let position = position.to_point(&snapshot.buffer_snapshot);
        if let Some(hunk) = self.go_to_hunk_after_position(&snapshot, position, window, cx) {
            let multi_buffer_start = snapshot
                .buffer_snapshot
                .anchor_before(Point::new(hunk.row_range.start.0, 0));
            let multi_buffer_end = snapshot
                .buffer_snapshot
                .anchor_after(Point::new(hunk.row_range.end.0, 0));
            self.expand_diff_hunk(
                None,
                &HoveredHunk {
                    multi_buffer_range: multi_buffer_start..multi_buffer_end,
                    status: hunk_status(&hunk),
                    diff_base_byte_range: hunk.diff_base_byte_range,
                },
                window,
                cx,
            );
        }
    }

    fn go_to_preceding_hunk(
        &mut self,
        position: Anchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot(window, cx);
        let position = position.to_point(&snapshot.buffer_snapshot);
        let hunk = self.go_to_hunk_before_position(&snapshot, position, window, cx);
        if let Some(hunk) = hunk {
            let multi_buffer_start = snapshot
                .buffer_snapshot
                .anchor_before(Point::new(hunk.row_range.start.0, 0));
            let multi_buffer_end = snapshot
                .buffer_snapshot
                .anchor_after(Point::new(hunk.row_range.end.0, 0));
            self.expand_diff_hunk(
                None,
                &HoveredHunk {
                    multi_buffer_range: multi_buffer_start..multi_buffer_end,
                    status: hunk_status(&hunk),
                    diff_base_byte_range: hunk.diff_base_byte_range,
                },
                window,
                cx,
            );
        }
    }
}

pub(crate) fn to_diff_hunk(
    hovered_hunk: &HoveredHunk,
    multi_buffer_snapshot: &MultiBufferSnapshot,
) -> Option<MultiBufferDiffHunk> {
    let buffer_id = hovered_hunk
        .multi_buffer_range
        .start
        .buffer_id
        .or(hovered_hunk.multi_buffer_range.end.buffer_id)?;
    let buffer_range = hovered_hunk.multi_buffer_range.start.text_anchor
        ..hovered_hunk.multi_buffer_range.end.text_anchor;
    let point_range = hovered_hunk
        .multi_buffer_range
        .to_point(multi_buffer_snapshot);
    Some(MultiBufferDiffHunk {
        row_range: MultiBufferRow(point_range.start.row)..MultiBufferRow(point_range.end.row),
        buffer_id,
        buffer_range,
        diff_base_byte_range: hovered_hunk.diff_base_byte_range.clone(),
    })
}

fn added_hunk_color(cx: &AppContext) -> Hsla {
    let mut created_color = cx.theme().status().git().created;
    created_color.fade_out(0.7);
    created_color
}

fn deleted_hunk_color(cx: &AppContext) -> Hsla {
    let mut deleted_color = cx.theme().status().deleted;
    deleted_color.fade_out(0.7);
    deleted_color
}

fn editor_with_deleted_text(
    diff_base_buffer: Model<Buffer>,
    deleted_color: Hsla,
    hunk: &HoveredHunk,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> (u32, Model<Editor>) {
    let parent_editor = cx.entity().downgrade();
    let editor = cx.new(|cx| {
        let multi_buffer = cx.new(|_| MultiBuffer::without_headers(language::Capability::ReadOnly));
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

        let mut editor = Editor::for_multibuffer(multi_buffer, None, true, window, cx);
        editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
        editor.set_show_wrap_guides(false, cx);
        editor.set_show_gutter(false, cx);
        editor.set_show_line_numbers(false, cx);
        editor.set_show_scrollbars(false, cx);
        editor.set_show_runnables(false, cx);
        editor.set_show_git_diff_gutter(false, cx);
        editor.set_show_code_actions(false, cx);
        editor.scroll_manager.set_forbid_vertical_scroll(true);
        editor.set_read_only(true);
        editor.set_show_inline_completions(Some(false), window, cx);

        enum DeletedBlockRowHighlight {}
        editor.highlight_rows::<DeletedBlockRowHighlight>(
            Anchor::min()..Anchor::max(),
            deleted_color,
            false,
            cx,
        );
        editor.set_current_line_highlight(Some(CurrentLineHighlight::None));
        editor._subscriptions.extend([cx.on_blur(
            &editor.focus_handle,
            window,
            |editor, window, cx| {
                editor.change_selections(None, window, cx, |s| {
                    s.try_cancel();
                });
            },
        )]);

        editor
            .register_action::<RevertSelectedHunks>({
                let hunk = hunk.clone();
                let parent_editor = parent_editor.clone();
                move |_, window, cx| {
                    parent_editor
                        .update(cx, |editor, cx| {
                            editor.revert_hunk(hunk.clone(), window, cx)
                        })
                        .ok();
                }
            })
            .detach();
        editor
            .register_action::<ToggleHunkDiff>({
                let hunk = hunk.clone();
                move |_, window, cx| {
                    parent_editor
                        .update(cx, |editor, cx| {
                            editor.toggle_hovered_hunk(&hunk, window, cx);
                        })
                        .ok();
                }
            })
            .detach();
        editor
    });

    let editor_height = editor.update(cx, |editor, cx| editor.max_point(cx).row().0);
    (editor_height, editor)
}

impl DisplayDiffHunk {
    pub fn start_display_row(&self) -> DisplayRow {
        match self {
            &DisplayDiffHunk::Folded { display_row } => display_row,
            DisplayDiffHunk::Unfolded {
                display_row_range, ..
            } => display_row_range.start,
        }
    }

    pub fn contains_display_row(&self, display_row: DisplayRow) -> bool {
        let range = match self {
            &DisplayDiffHunk::Folded { display_row } => display_row..=display_row,

            DisplayDiffHunk::Unfolded {
                display_row_range, ..
            } => display_row_range.start..=display_row_range.end,
        };

        range.contains(&display_row)
    }
}

pub fn diff_hunk_to_display(
    hunk: &MultiBufferDiffHunk,
    snapshot: &DisplaySnapshot,
) -> DisplayDiffHunk {
    let hunk_start_point = Point::new(hunk.row_range.start.0, 0);
    let hunk_start_point_sub = Point::new(hunk.row_range.start.0.saturating_sub(1), 0);
    let hunk_end_point_sub = Point::new(
        hunk.row_range
            .end
            .0
            .saturating_sub(1)
            .max(hunk.row_range.start.0),
        0,
    );

    let status = hunk_status(hunk);
    let is_removal = status == DiffHunkStatus::Removed;

    let folds_start = Point::new(hunk.row_range.start.0.saturating_sub(2), 0);
    let folds_end = Point::new(hunk.row_range.end.0 + 2, 0);
    let folds_range = folds_start..folds_end;

    let containing_fold = snapshot.folds_in_range(folds_range).find(|fold| {
        let fold_point_range = fold.range.to_point(&snapshot.buffer_snapshot);
        let fold_point_range = fold_point_range.start..=fold_point_range.end;

        let folded_start = fold_point_range.contains(&hunk_start_point);
        let folded_end = fold_point_range.contains(&hunk_end_point_sub);
        let folded_start_sub = fold_point_range.contains(&hunk_start_point_sub);

        (folded_start && folded_end) || (is_removal && folded_start_sub)
    });

    if let Some(fold) = containing_fold {
        let row = fold.range.start.to_display_point(snapshot).row();
        DisplayDiffHunk::Folded { display_row: row }
    } else {
        let start = hunk_start_point.to_display_point(snapshot).row();

        let hunk_end_row = hunk.row_range.end.max(hunk.row_range.start);
        let hunk_end_point = Point::new(hunk_end_row.0, 0);

        let multi_buffer_start = snapshot.buffer_snapshot.anchor_before(hunk_start_point);
        let multi_buffer_end = snapshot
            .buffer_snapshot
            .anchor_in_excerpt(multi_buffer_start.excerpt_id, hunk.buffer_range.end)
            .unwrap();
        let end = hunk_end_point.to_display_point(snapshot).row();

        DisplayDiffHunk::Unfolded {
            display_row_range: start..end,
            multi_buffer_range: multi_buffer_start..multi_buffer_end,
            status,
            diff_base_byte_range: hunk.diff_base_byte_range.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor_tests::init_test, hunk_status};
    use gpui::{Context, TestAppContext};
    use language::Capability::ReadWrite;
    use multi_buffer::{ExcerptRange, MultiBuffer, MultiBufferRow};
    use project::{FakeFs, Project};
    use unindent::Unindent as _;

    #[gpui::test]
    async fn test_diff_hunks_in_range(cx: &mut TestAppContext) {
        use git::diff::DiffHunkStatus;
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.background_executor.clone());
        let project = Project::test(fs, [], cx).await;

        // buffer has two modified hunks with two rows each
        let diff_base_1 = "
            1.zero
            1.one
            1.two
            1.three
            1.four
            1.five
            1.six
        "
        .unindent();

        let text_1 = "
            1.zero
            1.ONE
            1.TWO
            1.three
            1.FOUR
            1.FIVE
            1.six
        "
        .unindent();

        // buffer has a deletion hunk and an insertion hunk
        let diff_base_2 = "
            2.zero
            2.one
            2.one-and-a-half
            2.two
            2.three
            2.four
            2.six
        "
        .unindent();

        let text_2 = "
            2.zero
            2.one
            2.two
            2.three
            2.four
            2.five
            2.six
        "
        .unindent();

        let buffer_1 = project.update(cx, |project, cx| {
            project.create_local_buffer(text_1.as_str(), None, cx)
        });
        let buffer_2 = project.update(cx, |project, cx| {
            project.create_local_buffer(text_2.as_str(), None, cx)
        });

        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(ReadWrite);
            multibuffer.push_excerpts(
                buffer_1.clone(),
                [
                    // excerpt ends in the middle of a modified hunk
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(1, 5),
                        primary: Default::default(),
                    },
                    // excerpt begins in the middle of a modified hunk
                    ExcerptRange {
                        context: Point::new(5, 0)..Point::new(6, 5),
                        primary: Default::default(),
                    },
                ],
                cx,
            );
            multibuffer.push_excerpts(
                buffer_2.clone(),
                [
                    // excerpt ends at a deletion
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(1, 5),
                        primary: Default::default(),
                    },
                    // excerpt starts at a deletion
                    ExcerptRange {
                        context: Point::new(2, 0)..Point::new(2, 5),
                        primary: Default::default(),
                    },
                    // excerpt fully contains a deletion hunk
                    ExcerptRange {
                        context: Point::new(1, 0)..Point::new(2, 5),
                        primary: Default::default(),
                    },
                    // excerpt fully contains an insertion hunk
                    ExcerptRange {
                        context: Point::new(4, 0)..Point::new(6, 5),
                        primary: Default::default(),
                    },
                ],
                cx,
            );
            multibuffer
        });

        let editor = cx
            .add_window(|window, cx| Editor::for_multibuffer(multibuffer, None, false, window, cx));
        editor
            .update(cx, |editor, window, cx| {
                for (buffer, diff_base) in [
                    (buffer_1.clone(), diff_base_1),
                    (buffer_2.clone(), diff_base_2),
                ] {
                    let diff = cx.new(|cx| {
                        BufferChangeSet::new_with_base_text(
                            diff_base.to_string(),
                            buffer.read(cx).text_snapshot(),
                            cx,
                        )
                    });
                    editor.diff_map.add_diff(diff, window, cx)
                }
            })
            .unwrap();
        cx.background_executor.run_until_parked();

        let snapshot = editor
            .update(cx, |editor, window, cx| editor.snapshot(window, cx))
            .unwrap();

        assert_eq!(
            snapshot.buffer_snapshot.text(),
            "
                1.zero
                1.ONE
                1.FIVE
                1.six
                2.zero
                2.one
                2.two
                2.one
                2.two
                2.four
                2.five
                2.six"
                .unindent()
        );

        let expected = [
            (
                DiffHunkStatus::Modified,
                MultiBufferRow(1)..MultiBufferRow(2),
            ),
            (
                DiffHunkStatus::Modified,
                MultiBufferRow(2)..MultiBufferRow(3),
            ),
            //TODO: Define better when and where removed hunks show up at range extremities
            (
                DiffHunkStatus::Removed,
                MultiBufferRow(6)..MultiBufferRow(6),
            ),
            (
                DiffHunkStatus::Removed,
                MultiBufferRow(8)..MultiBufferRow(8),
            ),
            (
                DiffHunkStatus::Added,
                MultiBufferRow(10)..MultiBufferRow(11),
            ),
        ];

        assert_eq!(
            snapshot
                .diff_map
                .diff_hunks_in_range(Point::zero()..Point::new(12, 0), &snapshot.buffer_snapshot)
                .map(|hunk| (hunk_status(&hunk), hunk.row_range))
                .collect::<Vec<_>>(),
            &expected,
        );

        assert_eq!(
            snapshot
                .diff_map
                .diff_hunks_in_range_rev(
                    Point::zero()..Point::new(12, 0),
                    &snapshot.buffer_snapshot
                )
                .map(|hunk| (hunk_status(&hunk), hunk.row_range))
                .collect::<Vec<_>>(),
            expected
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .as_slice(),
        );
    }
}
