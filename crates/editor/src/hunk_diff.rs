use collections::{hash_map, HashMap, HashSet};
use git::diff::DiffHunkStatus;
use gpui::{Action, AnchorCorner, AppContext, CursorStyle, Hsla, Model, MouseButton, Task, View};
use language::{Buffer, BufferId, Point};
use multi_buffer::{
    Anchor, AnchorRangeExt, ExcerptRange, MultiBuffer, MultiBufferDiffHunk, MultiBufferRow,
    MultiBufferSnapshot, ToPoint,
};
use std::{ops::Range, sync::Arc};
use text::OffsetRangeExt;
use ui::{
    prelude::*, ActiveTheme, ContextMenu, IconButtonShape, InteractiveElement, IntoElement,
    ParentElement, PopoverMenu, Styled, Tooltip, ViewContext, VisualContext,
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

#[derive(Debug, Default)]
pub(super) struct ExpandedHunks {
    pub(crate) hunks: Vec<ExpandedHunk>,
    diff_base: HashMap<BufferId, DiffBaseBuffer>,
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

#[derive(Debug)]
struct DiffBaseBuffer {
    buffer: Model<Buffer>,
    diff_base_version: usize,
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

impl ExpandedHunks {
    pub fn hunks(&self, include_folded: bool) -> impl Iterator<Item = &ExpandedHunk> {
        self.hunks
            .iter()
            .filter(move |hunk| include_folded || !hunk.folded)
    }
}

impl Editor {
    pub fn set_expand_all_diff_hunks(&mut self) {
        self.expanded_hunks.expand_all = true;
    }

    pub(super) fn toggle_hovered_hunk(
        &mut self,
        hovered_hunk: &HoveredHunk,
        cx: &mut ViewContext<Editor>,
    ) {
        let editor_snapshot = self.snapshot(cx);
        if let Some(diff_hunk) = to_diff_hunk(hovered_hunk, &editor_snapshot.buffer_snapshot) {
            self.toggle_hunks_expanded(vec![diff_hunk], cx);
            self.change_selections(None, cx, |selections| selections.refresh());
        }
    }

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
                let hunk_display_row_range = Point::new(hunk.row_range.start.0, 0)
                    .to_display_point(&snapshot.display_snapshot)
                    ..Point::new(hunk.row_range.end.0, 0)
                        .to_display_point(&snapshot.display_snapshot);
                let row_range_end =
                    display_rows_with_expanded_hunks.get(&hunk_display_row_range.start.row());
                row_range_end.is_none() || row_range_end != Some(&hunk_display_row_range.end.row())
            });
        self.toggle_hunks_expanded(hunks.collect(), cx);
    }

    fn toggle_hunks_expanded(
        &mut self,
        hunks_to_toggle: Vec<MultiBufferDiffHunk>,
        cx: &mut ViewContext<Self>,
    ) {
        if self.expanded_hunks.expand_all {
            return;
        }

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
        hunk: &HoveredHunk,
        cx: &mut ViewContext<'_, Editor>,
    ) -> Option<()> {
        let buffer = self.buffer.clone();
        let multi_buffer_snapshot = buffer.read(cx).snapshot(cx);
        let hunk_range = hunk.multi_buffer_range.clone();
        let (diff_base_buffer, deleted_text_lines) = buffer.update(cx, |buffer, cx| {
            let buffer = buffer.buffer(hunk_range.start.buffer_id?)?;
            let diff_base_buffer = diff_base_buffer
                .or_else(|| self.current_diff_base_buffer(&buffer, cx))
                .or_else(|| create_diff_base_buffer(&buffer, cx))?;
            let deleted_text_lines = buffer.read(cx).diff_base().map(|diff_base| {
                let diff_start_row = diff_base
                    .offset_to_point(hunk.diff_base_byte_range.start)
                    .row;
                let diff_end_row = diff_base.offset_to_point(hunk.diff_base_byte_range.end).row;
                diff_end_row - diff_start_row
            })?;
            Some((diff_base_buffer, deleted_text_lines))
        })?;

        let block_insert_index = match self.expanded_hunks.hunks.binary_search_by(|probe| {
            probe
                .hunk_range
                .start
                .cmp(&hunk_range.start, &multi_buffer_snapshot)
        }) {
            Ok(_already_present) => return None,
            Err(ix) => ix,
        };

        let blocks;
        match hunk.status {
            DiffHunkStatus::Removed => {
                blocks = self.insert_blocks(
                    [
                        self.hunk_header_block(&hunk, cx),
                        Self::deleted_text_block(hunk, diff_base_buffer, deleted_text_lines, cx),
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
                        Self::deleted_text_block(hunk, diff_base_buffer, deleted_text_lines, cx),
                    ],
                    None,
                    cx,
                );
            }
        };
        self.expanded_hunks.hunks.insert(
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
        cx: &mut ViewContext<'_, Editor>,
    ) -> Option<()> {
        let (buffer, range, _) = self
            .buffer
            .read(cx)
            .range_to_buffer_ranges(range, cx)
            .into_iter()
            .next()?;

        buffer.update(cx, |branch_buffer, cx| {
            branch_buffer.merge_into_base(vec![range], cx);
        });

        if let Some(project) = self.project.clone() {
            self.save(true, project, cx).detach_and_log_err(cx);
        }

        None
    }

    pub(crate) fn apply_all_diff_hunks(
        &mut self,
        _: &ApplyAllDiffHunks,
        cx: &mut ViewContext<Self>,
    ) {
        let buffers = self.buffer.read(cx).all_buffers();
        for branch_buffer in buffers {
            branch_buffer.update(cx, |branch_buffer, cx| {
                branch_buffer.merge_into_base(Vec::new(), cx);
            });
        }

        if let Some(project) = self.project.clone() {
            self.save(true, project, cx).detach_and_log_err(cx);
        }
    }

    pub(crate) fn apply_selected_diff_hunks(
        &mut self,
        _: &ApplyDiffHunk,
        cx: &mut ViewContext<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let hunks = hunks_for_selections(&snapshot, &self.selections.disjoint_anchors());
        let mut ranges_by_buffer = HashMap::default();
        self.transact(cx, |editor, cx| {
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
            self.save(true, project, cx).detach_and_log_err(cx);
        }
    }

    fn hunk_header_block(
        &self,
        hunk: &HoveredHunk,
        cx: &mut ViewContext<'_, Editor>,
    ) -> BlockProperties<Anchor> {
        let is_branch_buffer = self
            .buffer
            .read(cx)
            .point_to_buffer_offset(hunk.multi_buffer_range.start, cx)
            .map_or(false, |(buffer, _, _)| {
                buffer.read(cx).diff_base_buffer().is_some()
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
            render: Box::new({
                let editor = cx.view().clone();
                let hunk = hunk.clone();

                move |cx| {
                    let hunk_controls_menu_handle =
                        editor.read(cx).hunk_controls_menu_handle.clone();

                    h_flex()
                        .id(cx.block_id)
                        .h(cx.line_height())
                        .w_full()
                        .border_t_1()
                        .border_color(border_color)
                        .bg(bg_color)
                        .child(
                            div()
                                .id("gutter-strip")
                                .w(EditorElement::diff_hunk_strip_width(cx.line_height()))
                                .h_full()
                                .bg(gutter_color)
                                .cursor(CursorStyle::PointingHand)
                                .on_click({
                                    let editor = editor.clone();
                                    let hunk = hunk.clone();
                                    move |_event, cx| {
                                        editor.update(cx, |editor, cx| {
                                            editor.toggle_hovered_hunk(&hunk, cx);
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
                                                    .tooltip({
                                                        let focus_handle = editor.focus_handle(cx);
                                                        move |cx| {
                                                            Tooltip::for_action_in(
                                                                "Next Hunk",
                                                                &GoToHunk,
                                                                &focus_handle,
                                                                cx,
                                                            )
                                                        }
                                                    })
                                                    .on_click({
                                                        let editor = editor.clone();
                                                        let hunk = hunk.clone();
                                                        move |_event, cx| {
                                                            editor.update(cx, |editor, cx| {
                                                                editor.go_to_subsequent_hunk(
                                                                    hunk.multi_buffer_range.end,
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
                                                    .tooltip({
                                                        let focus_handle = editor.focus_handle(cx);
                                                        move |cx| {
                                                            Tooltip::for_action_in(
                                                                "Previous Hunk",
                                                                &GoToPrevHunk,
                                                                &focus_handle,
                                                                cx,
                                                            )
                                                        }
                                                    })
                                                    .on_click({
                                                        let editor = editor.clone();
                                                        let hunk = hunk.clone();
                                                        move |_event, cx| {
                                                            editor.update(cx, |editor, cx| {
                                                                editor.go_to_preceding_hunk(
                                                                    hunk.multi_buffer_range.start,
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
                                                    move |cx| {
                                                        Tooltip::for_action_in(
                                                            "Discard Hunk",
                                                            &RevertSelectedHunks,
                                                            &focus_handle,
                                                            cx,
                                                        )
                                                    }
                                                })
                                                .on_click({
                                                    let editor = editor.clone();
                                                    let hunk = hunk.clone();
                                                    move |_event, cx| {
                                                        let multi_buffer =
                                                            editor.read(cx).buffer().clone();
                                                        let multi_buffer_snapshot =
                                                            multi_buffer.read(cx).snapshot(cx);
                                                        let mut revert_changes = HashMap::default();
                                                        if let Some(hunk) =
                                                            crate::hunk_diff::to_diff_hunk(
                                                                &hunk,
                                                                &multi_buffer_snapshot,
                                                            )
                                                        {
                                                            Editor::prepare_revert_change(
                                                                &mut revert_changes,
                                                                &multi_buffer,
                                                                &hunk,
                                                                cx,
                                                            );
                                                        }
                                                        if !revert_changes.is_empty() {
                                                            editor.update(cx, |editor, cx| {
                                                                editor.revert(revert_changes, cx)
                                                            });
                                                        }
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
                                                            move |cx| {
                                                                Tooltip::for_action_in(
                                                                    "Apply Hunk",
                                                                    &ApplyDiffHunk,
                                                                    &focus_handle,
                                                                    cx,
                                                                )
                                                            }
                                                        })
                                                        .on_click({
                                                            let editor = editor.clone();
                                                            let hunk = hunk.clone();
                                                            move |_event, cx| {
                                                                editor.update(cx, |editor, cx| {
                                                                    editor
                                                                        .apply_diff_hunks_in_range(
                                                                            hunk.multi_buffer_range
                                                                                .clone(),
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
                                                        .trigger(
                                                            IconButton::new(
                                                                "toggle_editor_selections_icon",
                                                                IconName::EllipsisVertical,
                                                            )
                                                            .shape(IconButtonShape::Square)
                                                            .icon_size(IconSize::Small)
                                                            .style(ButtonStyle::Subtle)
                                                            .selected(
                                                                hunk_controls_menu_handle
                                                                    .is_deployed(),
                                                            )
                                                            .when(
                                                                !hunk_controls_menu_handle
                                                                    .is_deployed(),
                                                                |this| {
                                                                    this.tooltip(|cx| {
                                                                        Tooltip::text(
                                                                            "Hunk Controls",
                                                                            cx,
                                                                        )
                                                                    })
                                                                },
                                                            ),
                                                        )
                                                        .anchor(AnchorCorner::TopRight)
                                                        .with_handle(hunk_controls_menu_handle)
                                                        .menu(move |cx| {
                                                            let focus = focus.clone();
                                                            let menu = ContextMenu::build(
                                                                cx,
                                                                move |menu, _| {
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
                                                move |cx| {
                                                    Tooltip::for_action_in(
                                                        "Collapse Hunk",
                                                        &ToggleHunkDiff,
                                                        &focus_handle,
                                                        cx,
                                                    )
                                                }
                                            })
                                            .on_click({
                                                let editor = editor.clone();
                                                let hunk = hunk.clone();
                                                move |_event, cx| {
                                                    editor.update(cx, |editor, cx| {
                                                        editor.toggle_hovered_hunk(&hunk, cx);
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
        cx: &mut ViewContext<'_, Editor>,
    ) -> BlockProperties<Anchor> {
        let gutter_color = match hunk.status {
            DiffHunkStatus::Added => unreachable!(),
            DiffHunkStatus::Modified => cx.theme().status().modified,
            DiffHunkStatus::Removed => cx.theme().status().deleted,
        };
        let deleted_hunk_color = deleted_hunk_color(cx);
        let (editor_height, editor_with_deleted_text) =
            editor_with_deleted_text(diff_base_buffer, deleted_hunk_color, hunk, cx);
        let editor = cx.view().clone();
        let hunk = hunk.clone();
        let height = editor_height.max(deleted_text_height);
        BlockProperties {
            placement: BlockPlacement::Above(hunk.multi_buffer_range.start),
            height,
            style: BlockStyle::Flex,
            priority: 0,
            render: Box::new(move |cx| {
                let width = EditorElement::diff_hunk_strip_width(cx.line_height());
                let gutter_dimensions = editor.read(cx.context).gutter_dimensions;

                h_flex()
                    .id(cx.block_id)
                    .bg(deleted_hunk_color)
                    .h(height as f32 * cx.line_height())
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
                                        move |_event, cx| {
                                            editor.update(cx, |editor, cx| {
                                                editor.toggle_hovered_hunk(&hunk, cx);
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

    pub(super) fn clear_expanded_diff_hunks(&mut self, cx: &mut ViewContext<'_, Editor>) -> bool {
        if self.expanded_hunks.expand_all {
            return false;
        }
        self.expanded_hunks.hunk_update_tasks.clear();
        self.clear_row_highlights::<DiffRowHighlight>();
        let to_remove = self
            .expanded_hunks
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
                                            if editor.expanded_hunks.expand_all {
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

                    if editor.expanded_hunks.expand_all {
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
                    }

                    editor.remove_highlighted_rows::<DiffRowHighlight>(highlights_to_remove, cx);
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

    fn go_to_subsequent_hunk(&mut self, position: Anchor, cx: &mut ViewContext<Self>) {
        let snapshot = self.snapshot(cx);
        let position = position.to_point(&snapshot.buffer_snapshot);
        if let Some(hunk) = self.go_to_hunk_after_position(&snapshot, position, cx) {
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
                cx,
            );
        }
    }

    fn go_to_preceding_hunk(&mut self, position: Anchor, cx: &mut ViewContext<Self>) {
        let snapshot = self.snapshot(cx);
        let position = position.to_point(&snapshot.buffer_snapshot);
        let hunk = self.go_to_hunk_before_position(&snapshot, position, cx);
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
                cx,
            );
        }
    }
}

fn to_diff_hunk(
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
    let mut deleted_color = cx.theme().status().deleted;
    deleted_color.fade_out(0.7);
    deleted_color
}

fn editor_with_deleted_text(
    diff_base_buffer: Model<Buffer>,
    deleted_color: Hsla,
    hunk: &HoveredHunk,
    cx: &mut ViewContext<'_, Editor>,
) -> (u32, View<Editor>) {
    let parent_editor = cx.view().downgrade();
    let editor = cx.new_view(|cx| {
        let multi_buffer =
            cx.new_model(|_| MultiBuffer::without_headers(language::Capability::ReadOnly));
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
        editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
        editor.set_show_wrap_guides(false, cx);
        editor.set_show_gutter(false, cx);
        editor.scroll_manager.set_forbid_vertical_scroll(true);
        editor.set_read_only(true);
        editor.set_show_inline_completions(Some(false), cx);

        enum DeletedBlockRowHighlight {}
        editor.highlight_rows::<DeletedBlockRowHighlight>(
            Anchor::min()..Anchor::max(),
            deleted_color,
            false,
            cx,
        );
        editor.set_current_line_highlight(Some(CurrentLineHighlight::None)); //
        editor
            ._subscriptions
            .extend([cx.on_blur(&editor.focus_handle, |editor, cx| {
                editor.change_selections(None, cx, |s| {
                    s.try_cancel();
                });
            })]);

        let original_multi_buffer_range = hunk.multi_buffer_range.clone();
        let diff_base_range = hunk.diff_base_byte_range.clone();
        editor
            .register_action::<RevertSelectedHunks>({
                let parent_editor = parent_editor.clone();
                move |_, cx| {
                    parent_editor
                        .update(cx, |editor, cx| {
                            let Some((buffer, original_text)) =
                                editor.buffer().update(cx, |buffer, cx| {
                                    let (_, buffer, _) = buffer.excerpt_containing(
                                        original_multi_buffer_range.start,
                                        cx,
                                    )?;
                                    let original_text =
                                        buffer.read(cx).diff_base()?.slice(diff_base_range.clone());
                                    Some((buffer, Arc::from(original_text.to_string())))
                                })
                            else {
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
                }
            })
            .detach();
        let hunk = hunk.clone();
        editor
            .register_action::<ToggleHunkDiff>(move |_, cx| {
                parent_editor
                    .update(cx, |editor, cx| {
                        editor.toggle_hovered_hunk(&hunk, cx);
                    })
                    .ok();
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
        let buffer_1 = project.update(cx, |project, cx| {
            project.create_local_buffer(
                "
                        1.zero
                        1.ONE
                        1.TWO
                        1.three
                        1.FOUR
                        1.FIVE
                        1.six
                    "
                .unindent()
                .as_str(),
                None,
                cx,
            )
        });
        buffer_1.update(cx, |buffer, cx| {
            buffer.set_diff_base(
                Some(
                    "
                        1.zero
                        1.one
                        1.two
                        1.three
                        1.four
                        1.five
                        1.six
                    "
                    .unindent(),
                ),
                cx,
            );
        });

        // buffer has a deletion hunk and an insertion hunk
        let buffer_2 = project.update(cx, |project, cx| {
            project.create_local_buffer(
                "
                        2.zero
                        2.one
                        2.two
                        2.three
                        2.four
                        2.five
                        2.six
                    "
                .unindent()
                .as_str(),
                None,
                cx,
            )
        });
        buffer_2.update(cx, |buffer, cx| {
            buffer.set_diff_base(
                Some(
                    "
                        2.zero
                        2.one
                        2.one-and-a-half
                        2.two
                        2.three
                        2.four
                        2.six
                    "
                    .unindent(),
                ),
                cx,
            );
        });

        cx.background_executor.run_until_parked();

        let multibuffer = cx.new_model(|cx| {
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

        let snapshot = multibuffer.read_with(cx, |b, cx| b.snapshot(cx));

        assert_eq!(
            snapshot.text(),
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
                .git_diff_hunks_in_range(MultiBufferRow(0)..MultiBufferRow(12))
                .map(|hunk| (hunk_status(&hunk), hunk.row_range))
                .collect::<Vec<_>>(),
            &expected,
        );

        assert_eq!(
            snapshot
                .git_diff_hunks_in_range_rev(MultiBufferRow(0)..MultiBufferRow(12))
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
