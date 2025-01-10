use collections::{HashMap, HashSet};
use git::diff::DiffHunkStatus;
use gpui::{
    Action, AppContext, Corner, CursorStyle, Hsla, Model, MouseButton, Subscription, Task, View,
};
use language::{Buffer, BufferId, Point};
use multi_buffer::{
    Anchor, AnchorRangeExt, ExcerptRange, MultiBuffer, MultiBufferDiffHunk, ToPoint,
};
use project::buffer_store::BufferChangeSet;
use std::{ops::Range, sync::Arc};
use text::OffsetRangeExt;
use ui::{
    prelude::*, ActiveTheme, ContextMenu, IconButtonShape, InteractiveElement, IntoElement,
    ParentElement, PopoverMenu, Styled, Tooltip, ViewContext, VisualContext,
};
use util::RangeExt;
use workspace::Item;

use crate::{
    editor_settings::CurrentLineHighlight, ApplyAllDiffHunks, ApplyDiffHunk, BlockPlacement,
    BlockProperties, BlockStyle, CustomBlockId, DiffRowHighlight, DisplayRow, DisplaySnapshot,
    Editor, EditorElement, GoToHunk, GoToPrevHunk, RevertFile, RevertSelectedHunks, ToDisplayPoint,
    ToggleHunkDiff,
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

pub(crate) struct DiffBaseState {
    pub(crate) change_set: Model<BufferChangeSet>,
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

impl Editor {
    fn toggle_hunks_expanded(
        &mut self,
        hunks_to_toggle: Vec<MultiBufferDiffHunk>,
        cx: &mut ViewContext<Self>,
    ) {
        if self.diff_map.expand_all {
            return;
        }

        let previous_toggle_task = self.diff_map.hunk_update_tasks.remove(&None);
        let new_toggle_task = cx.spawn(move |editor, mut cx| async move {
            if let Some(task) = previous_toggle_task {
                task.await;
            }

            editor
                .update(&mut cx, |editor, cx| {
                    let snapshot = editor.snapshot(cx);
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
                            status: hunk.status(),
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

        self.diff_map
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
        let buffer_id = hunk_range.start.buffer_id?;
        let diff_base_buffer = diff_base_buffer.or_else(|| {
            self.diff_map
                .diff_bases
                .get(&buffer_id)?
                .change_set
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
        let snapshot = self.snapshot(cx);
        let ranges = self.selections.all(cx).into_iter().map(|s| s.range());
        let hunks = snapshot.hunks_for_ranges(ranges);
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

    fn has_multiple_hunks(&self, cx: &mut WindowContext) -> bool {
        self.buffer.read(cx).has_multiple_hunks(cx)
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
                let editor = cx.view().clone();
                let hunk = hunk.clone();
                let has_multiple_hunks = self.has_multiple_hunks(cx);

                move |cx| {
                    let hunk_controls_menu_handle =
                        editor.read(cx).hunk_controls_menu_handle.clone();

                    h_flex()
                        .id(cx.block_id)
                        .block_mouse_down()
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
                                .cursor(CursorStyle::PointingHand), // .on_click({
                                                                    //     let editor = editor.clone();
                                                                    //     let hunk = hunk.clone();
                                                                    //     move |_event, cx| {
                                                                    //         // editor.update(cx, |editor, cx| {
                                                                    //         //     editor.toggle_hovered_hunk(&hunk, cx);
                                                                    //         // });
                                                                    //     }
                                                                    // }),
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
                                                    .disabled(!has_multiple_hunks)
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
                                                        editor.update(cx, |editor, cx| {
                                                            editor.revert_hunk(hunk.clone(), cx);
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
                                                            .toggle_state(
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
                                                        .anchor(Corner::TopRight)
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
                                                    // editor.update(cx, |editor, cx| {
                                                    //     editor.toggle_hovered_hunk(&hunk, cx);
                                                    // });
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
            render: Arc::new(move |cx| {
                let width = EditorElement::diff_hunk_strip_width(cx.line_height());
                let gutter_dimensions = editor.read(cx.context).gutter_dimensions;

                h_flex()
                    .id(cx.block_id)
                    .block_mouse_down()
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
                                            // editor.update(cx, |editor, cx| {
                                            //     editor.toggle_hovered_hunk(&hunk, cx);
                                            // });
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
        self.buffer.update(cx, |buffer, cx| {
            let ranges = vec![Anchor::min()..Anchor::max()];
            if buffer.has_expanded_diff_hunks_in_ranges(&ranges, cx) {
                buffer.collapse_diff_hunks(ranges, cx);
                true
            } else {
                false
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
                    status: hunk.status(),
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
                    status: hunk.status(),
                    diff_base_byte_range: hunk.diff_base_byte_range,
                },
                cx,
            );
        }
    }
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

        editor
            .register_action::<RevertSelectedHunks>({
                let hunk = hunk.clone();
                let parent_editor = parent_editor.clone();
                move |_, cx| {
                    parent_editor
                        .update(cx, |editor, cx| editor.revert_hunk(hunk.clone(), cx))
                        .ok();
                }
            })
            .detach();
        editor
            .register_action::<ToggleHunkDiff>({
                let hunk = hunk.clone();
                move |_, cx| {
                    // parent_editor
                    //     .update(cx, |editor, cx| {
                    //         editor.toggle_hovered_hunk(&hunk, cx);
                    //     })
                    //     .ok();
                }
            })
            .detach();
        editor
    });

    let editor_height = editor.update(cx, |editor, cx| editor.max_point(cx).row().0);
    (editor_height, editor)
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

    let status = hunk.status();
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
