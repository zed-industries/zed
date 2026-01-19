use std::ops::Range;

use buffer_diff::BufferDiff;
use collections::HashMap;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    Action, AppContext as _, Entity, EventEmitter, Focusable, NoAction, Subscription, WeakEntity,
};
use language::{Buffer, Capability};
use multi_buffer::{
    Anchor, ExcerptId, ExcerptRange, ExpandExcerptDirection, MultiBuffer, PathKey, ToPoint as _,
};
use project::Project;
use rope::Point;
use text::OffsetRangeExt as _;
use ui::{
    App, Context, InteractiveElement as _, IntoElement as _, ParentElement as _, Render,
    Styled as _, Window, div,
};

use crate::split_editor_view::{SplitEditorState, SplitEditorView};
use workspace::{
    ActivatePaneLeft, ActivatePaneRight, Item, ItemHandle, Pane, PaneGroup, SplitDirection,
    Workspace,
};

use crate::{
    Autoscroll, DisplayMap, Editor, EditorEvent,
    display_map::{Companion, convert_lhs_rows_to_rhs, convert_rhs_rows_to_lhs},
};

struct SplitDiffFeatureFlag;

impl FeatureFlag for SplitDiffFeatureFlag {
    const NAME: &'static str = "split-diff";

    fn enabled_for_staff() -> bool {
        true
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Action, Default)]
#[action(namespace = editor)]
struct SplitDiff;

#[derive(Clone, Copy, PartialEq, Eq, Action, Default)]
#[action(namespace = editor)]
struct UnsplitDiff;

#[derive(Clone, Copy, PartialEq, Eq, Action, Default)]
#[action(namespace = editor)]
struct JumpToCorrespondingRow;

pub struct SplittableEditor {
    primary_multibuffer: Entity<MultiBuffer>,
    primary_editor: Entity<Editor>,
    secondary: Option<SecondaryEditor>,
    panes: PaneGroup,
    workspace: WeakEntity<Workspace>,
    split_state: Entity<SplitEditorState>,
    _subscriptions: Vec<Subscription>,
}

struct SecondaryEditor {
    multibuffer: Entity<MultiBuffer>,
    editor: Entity<Editor>,
    pane: Entity<Pane>,
    has_latest_selection: bool,
    _subscriptions: Vec<Subscription>,
}

impl SplittableEditor {
    pub fn primary_editor(&self) -> &Entity<Editor> {
        &self.primary_editor
    }

    pub fn secondary_editor(&self) -> Option<&Entity<Editor>> {
        self.secondary.as_ref().map(|s| &s.editor)
    }

    pub fn last_selected_editor(&self) -> &Entity<Editor> {
        if let Some(secondary) = &self.secondary
            && secondary.has_latest_selection
        {
            &secondary.editor
        } else {
            &self.primary_editor
        }
    }

    pub fn new_unsplit(
        primary_multibuffer: Entity<MultiBuffer>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let primary_editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(
                primary_multibuffer.clone(),
                Some(project.clone()),
                window,
                cx,
            );
            editor.set_expand_all_diff_hunks(cx);
            editor
        });
        let pane = cx.new(|cx| {
            let mut pane = Pane::new(
                workspace.downgrade(),
                project,
                Default::default(),
                None,
                NoAction.boxed_clone(),
                true,
                window,
                cx,
            );
            pane.set_should_display_tab_bar(|_, _| false);
            pane.add_item(primary_editor.boxed_clone(), true, true, None, window, cx);
            pane
        });
        let panes = PaneGroup::new(pane);
        // TODO(split-diff) we might want to tag editor events with whether they came from primary/secondary
        let subscriptions = vec![cx.subscribe(
            &primary_editor,
            |this, _, event: &EditorEvent, cx| match event {
                EditorEvent::ExpandExcerptsRequested {
                    excerpt_ids,
                    lines,
                    direction,
                } => {
                    this.expand_excerpts(excerpt_ids.iter().copied(), *lines, *direction, cx);
                }
                EditorEvent::SelectionsChanged { .. } => {
                    if let Some(secondary) = &mut this.secondary {
                        secondary.has_latest_selection = false;
                    }
                    cx.emit(event.clone());
                }
                _ => cx.emit(event.clone()),
            },
        )];

        window.defer(cx, {
            let workspace = workspace.downgrade();
            let primary_editor = primary_editor.downgrade();
            move |window, cx| {
                workspace
                    .update(cx, |workspace, cx| {
                        primary_editor.update(cx, |editor, cx| {
                            editor.added_to_workspace(workspace, window, cx);
                        })
                    })
                    .ok();
            }
        });
        let split_state = cx.new(|cx| SplitEditorState::new(cx));
        Self {
            primary_editor,
            primary_multibuffer,
            secondary: None,
            panes,
            workspace: workspace.downgrade(),
            split_state,
            _subscriptions: subscriptions,
        }
    }

    fn split(&mut self, _: &SplitDiff, window: &mut Window, cx: &mut Context<Self>) {
        if !cx.has_flag::<SplitDiffFeatureFlag>() {
            return;
        }
        if self.secondary.is_some() {
            return;
        }
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let project = workspace.read(cx).project().clone();

        let secondary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadOnly);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        let secondary_editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(
                secondary_multibuffer.clone(),
                Some(project.clone()),
                window,
                cx,
            );
            editor.number_deleted_lines = true;
            editor.set_delegate_expand_excerpts(true);
            editor
        });
        let secondary_pane = cx.new(|cx| {
            let mut pane = Pane::new(
                workspace.downgrade(),
                workspace.read(cx).project().clone(),
                Default::default(),
                None,
                NoAction.boxed_clone(),
                true,
                window,
                cx,
            );
            pane.set_should_display_tab_bar(|_, _| false);
            pane.add_item(
                ItemHandle::boxed_clone(&secondary_editor),
                false,
                false,
                None,
                window,
                cx,
            );
            pane
        });

        let subscriptions = vec![cx.subscribe(
            &secondary_editor,
            |this, _, event: &EditorEvent, cx| match event {
                EditorEvent::ExpandExcerptsRequested {
                    excerpt_ids,
                    lines,
                    direction,
                } => {
                    if this.secondary.is_some() {
                        let primary_display_map = this.primary_editor.read(cx).display_map.read(cx);
                        let primary_ids: Vec<_> = excerpt_ids
                            .iter()
                            .filter_map(|id| {
                                primary_display_map.companion_excerpt_to_my_excerpt(*id, cx)
                            })
                            .collect();
                        this.expand_excerpts(primary_ids.into_iter(), *lines, *direction, cx);
                    }
                }
                EditorEvent::SelectionsChanged { .. } => {
                    if let Some(secondary) = &mut this.secondary {
                        secondary.has_latest_selection = true;
                    }
                    cx.emit(event.clone());
                }
                _ => cx.emit(event.clone()),
            },
        )];
        let mut secondary = SecondaryEditor {
            editor: secondary_editor,
            multibuffer: secondary_multibuffer,
            pane: secondary_pane.clone(),
            has_latest_selection: false,
            _subscriptions: subscriptions,
        };
        let primary_display_map = self.primary_editor.read(cx).display_map.clone();
        let secondary_display_map = secondary.editor.read(cx).display_map.clone();
        let rhs_display_map_id = primary_display_map.entity_id();

        self.primary_editor.update(cx, |editor, cx| {
            editor.set_delegate_expand_excerpts(true);
            editor.buffer().update(cx, |primary_multibuffer, cx| {
                primary_multibuffer.set_show_deleted_hunks(false, cx);
            })
        });

        let path_diffs: Vec<_> = {
            let primary_multibuffer = self.primary_multibuffer.read(cx);
            primary_multibuffer
                .paths()
                .filter_map(|path| {
                    let excerpt_id = primary_multibuffer.excerpts_for_path(path).next()?;
                    let snapshot = primary_multibuffer.snapshot(cx);
                    let buffer = snapshot.buffer_for_excerpt(excerpt_id)?;
                    let diff = primary_multibuffer.diff_for(buffer.remote_id())?;
                    Some((path.clone(), diff))
                })
                .collect()
        };

        let mut companion = Companion::new(
            rhs_display_map_id,
            convert_rhs_rows_to_lhs,
            convert_lhs_rows_to_rhs,
        );

        for (path, diff) in path_diffs {
            for mapping in secondary.update_path_excerpts_from_primary(
                path,
                &self.primary_multibuffer,
                diff.clone(),
                cx,
            ) {
                companion.add_excerpt_mapping(mapping.left, mapping.right);
            }
            companion.add_buffer_mapping(
                diff.read(cx).base_text(cx).remote_id(),
                diff.read(cx).buffer_id,
            );
        }

        let companion = cx.new(|_| companion);

        primary_display_map.update(cx, |dm, cx| {
            dm.set_companion(
                Some((secondary_display_map.downgrade(), companion.clone())),
                cx,
            );
        });
        secondary_display_map.update(cx, |dm, cx| {
            dm.set_companion(Some((primary_display_map.downgrade(), companion)), cx);
        });

        let primary_weak = self.primary_editor.downgrade();
        let secondary_weak = secondary.editor.downgrade();

        self.primary_editor.update(cx, |editor, _cx| {
            editor.set_scroll_companion(Some(secondary_weak));
        });
        secondary.editor.update(cx, |editor, _cx| {
            editor.set_scroll_companion(Some(primary_weak));
        });

        let primary_scroll_position = self
            .primary_editor
            .update(cx, |editor, cx| editor.scroll_position(cx));
        secondary.editor.update(cx, |editor, cx| {
            editor.set_scroll_position_internal(primary_scroll_position, false, false, window, cx);
        });

        self.secondary = Some(secondary);

        let primary_pane = self.panes.first_pane();
        self.panes
            .split(&primary_pane, &secondary_pane, SplitDirection::Left, cx)
            .unwrap();
        cx.notify();
    }

    fn activate_pane_left(
        &mut self,
        _: &ActivatePaneLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(secondary) = &self.secondary {
            if !secondary.has_latest_selection {
                secondary.editor.read(cx).focus_handle(cx).focus(window, cx);
                secondary.editor.update(cx, |editor, cx| {
                    editor.request_autoscroll(Autoscroll::fit(), cx);
                });
                cx.notify();
            } else {
                cx.propagate();
            }
        } else {
            cx.propagate();
        }
    }

    fn activate_pane_right(
        &mut self,
        _: &ActivatePaneRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(secondary) = &self.secondary {
            if secondary.has_latest_selection {
                self.primary_editor
                    .read(cx)
                    .focus_handle(cx)
                    .focus(window, cx);
                self.primary_editor.update(cx, |editor, cx| {
                    editor.request_autoscroll(Autoscroll::fit(), cx);
                });
                cx.notify();
            } else {
                cx.propagate();
            }
        } else {
            cx.propagate();
        }
    }

    fn jump_to_corresponding_row(
        &mut self,
        _: &JumpToCorrespondingRow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(secondary) = &self.secondary else {
            return;
        };

        let is_on_left = secondary.has_latest_selection;
        let (source_editor, target_editor) = if is_on_left {
            (&secondary.editor, &self.primary_editor)
        } else {
            (&self.primary_editor, &secondary.editor)
        };

        let (source_multibuffer, target_multibuffer) = if is_on_left {
            (&secondary.multibuffer, &self.primary_multibuffer)
        } else {
            (&self.primary_multibuffer, &secondary.multibuffer)
        };

        let source_snapshot = source_multibuffer.read(cx).snapshot(cx);
        let target_snapshot = target_multibuffer.read(cx).snapshot(cx);

        let source_point = source_editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(window, cx).display_snapshot;
            editor.selections.newest::<Point>(&snapshot).head()
        });

        let Some(source_excerpt) = source_snapshot.excerpt_containing(source_point..source_point)
        else {
            return;
        };

        let source_excerpt_id = source_excerpt.id();
        let source_buffer = source_excerpt.buffer();

        let Some(source_context_range) = source_snapshot.context_range_for_excerpt(source_excerpt_id)
        else {
            return;
        };

        let source_context_start =
            language::ToPoint::to_point(&source_context_range.start, source_buffer);

        let source_excerpt_start_row = source_excerpt.start_anchor().to_point(&source_snapshot).row;

        let row_within_excerpt = source_point.row.saturating_sub(source_excerpt_start_row);
        let local_buffer_row = source_context_start.row + row_within_excerpt;

        let Some(diff_snapshot) = source_snapshot.diff_for_buffer_id(source_buffer.remote_id())
        else {
            return;
        };

        let target_row_range = if is_on_left {
            // For inverted diffs (LHS showing base text), the diff's anchors still refer to the
            // main buffer (RHS), not the base text buffer. We need to use the main buffer when
            // seeking in the diff's sum tree.
            let Some(main_buffer) =
                source_snapshot.inverted_diff_main_buffer(source_buffer.remote_id())
            else {
                return;
            };
            diff_snapshot
                .base_text_rows_to_rows(local_buffer_row..local_buffer_row, main_buffer)
                .next()
        } else {
            diff_snapshot
                .rows_to_base_text_rows(local_buffer_row..local_buffer_row, source_buffer)
                .next()
        };

        let Some(target_row_range) = target_row_range else {
            return;
        };

        let target_local_row = target_row_range.start;

        let Some(target_path) = source_multibuffer.read(cx).path_for_excerpt(source_excerpt_id)
        else {
            return;
        };

        let target_excerpt_id = target_multibuffer
            .read(cx)
            .excerpts_for_path(&target_path)
            .find(|excerpt_id| {
                if let Some(context_range) = target_snapshot.context_range_for_excerpt(*excerpt_id) {
                    if let Some(buffer) = target_snapshot.buffer_for_excerpt(*excerpt_id) {
                        let start =
                            language::ToPoint::to_point(&context_range.start, buffer).row;
                        let end = language::ToPoint::to_point(&context_range.end, buffer).row;
                        return target_local_row >= start && target_local_row <= end;
                    }
                }
                false
            });

        let Some(target_excerpt_id) = target_excerpt_id else {
            return;
        };

        let Some(target_buffer) = target_snapshot.buffer_for_excerpt(target_excerpt_id) else {
            return;
        };

        let Some(target_context_range) =
            target_snapshot.context_range_for_excerpt(target_excerpt_id)
        else {
            return;
        };

        let target_context_start =
            language::ToPoint::to_point(&target_context_range.start, target_buffer);

        let Some(target_anchor) =
            target_snapshot.anchor_in_excerpt(target_excerpt_id, text::Anchor::MIN)
        else {
            return;
        };
        let Some(target_excerpt) =
            target_snapshot.excerpt_containing(target_anchor..target_anchor)
        else {
            return;
        };
        let target_excerpt_start_row = target_excerpt.start_anchor().to_point(&target_snapshot).row;

        let row_within_target_excerpt = target_local_row.saturating_sub(target_context_start.row);
        let target_multibuffer_row = target_excerpt_start_row + row_within_target_excerpt;

        let target_column = if target_row_range.start == target_row_range.end {
            let target_line_len = target_buffer.line_len(target_local_row);
            source_point.column.min(target_line_len)
        } else {
            0
        };

        let target_point = Point::new(target_multibuffer_row, target_column);

        target_editor.update(cx, |editor, cx| {
            editor.change_selections(
                crate::SelectionEffects::scroll(crate::Autoscroll::center()),
                window,
                cx,
                |s| {
                    s.select_ranges([target_point..target_point]);
                },
            );
        });

        target_editor.read(cx).focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn unsplit(&mut self, _: &UnsplitDiff, _: &mut Window, cx: &mut Context<Self>) {
        let Some(secondary) = self.secondary.take() else {
            return;
        };
        self.panes.remove(&secondary.pane, cx).unwrap();
        self.primary_editor.update(cx, |primary, cx| {
            primary.set_scroll_companion(None);
            primary.set_delegate_expand_excerpts(false);
            primary.buffer().update(cx, |buffer, cx| {
                buffer.set_show_deleted_hunks(true, cx);
            });
            primary.display_map.update(cx, |dm, cx| {
                dm.set_companion(None, cx);
            });
        });
        secondary.editor.update(cx, |editor, _cx| {
            editor.set_scroll_companion(None);
        });
        cx.notify();
    }

    pub fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace = workspace.weak_handle();
        self.primary_editor.update(cx, |primary_editor, cx| {
            primary_editor.added_to_workspace(workspace, window, cx);
        });
        if let Some(secondary) = &self.secondary {
            secondary.editor.update(cx, |secondary_editor, cx| {
                secondary_editor.added_to_workspace(workspace, window, cx);
            });
        }
    }

    pub fn set_excerpts_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = Range<Point>> + Clone,
        context_line_count: u32,
        diff: Entity<BufferDiff>,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let primary_display_map = self.primary_editor.read(cx).display_map.clone();
        let secondary_display_map = self
            .secondary
            .as_ref()
            .map(|s| s.editor.read(cx).display_map.clone());

        let (anchors, added_a_new_excerpt) =
            self.primary_multibuffer
                .update(cx, |primary_multibuffer, cx| {
                    let (anchors, added_a_new_excerpt) = primary_multibuffer.set_excerpts_for_path(
                        path.clone(),
                        buffer.clone(),
                        ranges,
                        context_line_count,
                        cx,
                    );
                    if !anchors.is_empty()
                        && primary_multibuffer
                            .diff_for(buffer.read(cx).remote_id())
                            .is_none_or(|old_diff| old_diff.entity_id() != diff.entity_id())
                    {
                        primary_multibuffer.add_diff(diff.clone(), cx);
                    }
                    (anchors, added_a_new_excerpt)
                });

        if let Some(secondary) = &mut self.secondary {
            if let Some(secondary_display_map) = &secondary_display_map {
                secondary.sync_path_excerpts(
                    path,
                    &self.primary_multibuffer,
                    diff,
                    &primary_display_map,
                    secondary_display_map,
                    cx,
                );
            }
        }

        (anchors, added_a_new_excerpt)
    }

    fn expand_excerpts(
        &mut self,
        excerpt_ids: impl Iterator<Item = ExcerptId> + Clone,
        lines: u32,
        direction: ExpandExcerptDirection,
        cx: &mut Context<Self>,
    ) {
        let mut corresponding_paths = HashMap::default();
        self.primary_multibuffer.update(cx, |multibuffer, cx| {
            let snapshot = multibuffer.snapshot(cx);
            if self.secondary.is_some() {
                corresponding_paths = excerpt_ids
                    .clone()
                    .map(|excerpt_id| {
                        let path = multibuffer.path_for_excerpt(excerpt_id).unwrap();
                        let buffer = snapshot.buffer_for_excerpt(excerpt_id).unwrap();
                        let diff = multibuffer.diff_for(buffer.remote_id()).unwrap();
                        (path, diff)
                    })
                    .collect::<HashMap<_, _>>();
            }
            multibuffer.expand_excerpts(excerpt_ids.clone(), lines, direction, cx);
        });

        if let Some(secondary) = &mut self.secondary {
            let primary_display_map = self.primary_editor.read(cx).display_map.clone();
            let secondary_display_map = secondary.editor.read(cx).display_map.clone();
            for (path, diff) in corresponding_paths {
                secondary.sync_path_excerpts(
                    path,
                    &self.primary_multibuffer,
                    diff,
                    &primary_display_map,
                    &secondary_display_map,
                    cx,
                );
            }
        }
    }

    pub fn remove_excerpts_for_path(&mut self, path: PathKey, cx: &mut Context<Self>) {
        self.primary_multibuffer.update(cx, |buffer, cx| {
            buffer.remove_excerpts_for_path(path.clone(), cx)
        });
        if let Some(secondary) = &self.secondary {
            let primary_display_map = self.primary_editor.read(cx).display_map.clone();
            let secondary_display_map = secondary.editor.read(cx).display_map.clone();
            secondary.remove_mappings_for_path(
                &path,
                &self.primary_multibuffer,
                &primary_display_map,
                &secondary_display_map,
                cx,
            );
            secondary
                .multibuffer
                .update(cx, |buffer, cx| buffer.remove_excerpts_for_path(path, cx))
        }
    }
}

#[cfg(test)]
impl SplittableEditor {
    fn check_invariants(&self, quiesced: bool, cx: &mut App) {
        use buffer_diff::DiffHunkStatusKind;
        use collections::HashSet;
        use multi_buffer::MultiBufferOffset;
        use multi_buffer::MultiBufferRow;
        use multi_buffer::MultiBufferSnapshot;

        fn format_diff(snapshot: &MultiBufferSnapshot) -> String {
            let text = snapshot.text();
            let row_infos = snapshot.row_infos(MultiBufferRow(0)).collect::<Vec<_>>();
            let boundary_rows = snapshot
                .excerpt_boundaries_in_range(MultiBufferOffset(0)..)
                .map(|b| b.row)
                .collect::<HashSet<_>>();

            text.split('\n')
                .enumerate()
                .zip(row_infos)
                .map(|((ix, line), info)| {
                    let marker = match info.diff_status.map(|status| status.kind) {
                        Some(DiffHunkStatusKind::Added) => "+ ",
                        Some(DiffHunkStatusKind::Deleted) => "- ",
                        Some(DiffHunkStatusKind::Modified) => unreachable!(),
                        None => {
                            if !line.is_empty() {
                                "  "
                            } else {
                                ""
                            }
                        }
                    };
                    let boundary_row = if boundary_rows.contains(&MultiBufferRow(ix as u32)) {
                        "  ----------\n"
                    } else {
                        ""
                    };
                    let expand = info
                        .expand_info
                        .map(|expand_info| match expand_info.direction {
                            ExpandExcerptDirection::Up => " [↑]",
                            ExpandExcerptDirection::Down => " [↓]",
                            ExpandExcerptDirection::UpAndDown => " [↕]",
                        })
                        .unwrap_or_default();

                    format!("{boundary_row}{marker}{line}{expand}")
                })
                .collect::<Vec<_>>()
                .join("\n")
        }

        let Some(secondary) = &self.secondary else {
            return;
        };

        log::info!(
            "primary:\n\n{}",
            format_diff(&self.primary_multibuffer.read(cx).snapshot(cx))
        );

        log::info!(
            "secondary:\n\n{}",
            format_diff(&secondary.multibuffer.read(cx).snapshot(cx))
        );

        let primary_excerpts = self.primary_multibuffer.read(cx).excerpt_ids();
        let secondary_excerpts = secondary.multibuffer.read(cx).excerpt_ids();
        assert_eq!(secondary_excerpts.len(), primary_excerpts.len());

        if quiesced {
            // todo! re-enable
            // self.check_sides_match(cx, Self::unmodified_rows);

            self.check_sides_match(cx, |snapshot| {
                snapshot
                    .diff_hunks()
                    .map(|hunk| hunk.diff_base_byte_range)
                    .collect::<Vec<_>>()
            });

            // Filtering out empty lines is a bit of a hack, to work around a case where
            // the base text has a trailing newline but the current text doesn't, or vice versa.
            // In this case, we get the additional newline on one side, but that line is not
            // marked as added/deleted by rowinfos.
            self.check_sides_match(cx, |snapshot| {
                snapshot
                    .text()
                    .split("\n")
                    .zip(snapshot.row_infos(MultiBufferRow(0)))
                    .filter(|(line, row_info)| !line.is_empty() && row_info.diff_status.is_none())
                    .map(|(line, _)| line.to_owned())
                    .collect::<Vec<_>>()
            });
        }
    }

    fn check_sides_match<T: std::fmt::Debug + PartialEq>(
        &self,
        cx: &mut App,
        mut extract: impl FnMut(&multi_buffer::MultiBufferSnapshot) -> T,
    ) {
        let primary_snapshot = self.primary_multibuffer.read(cx).snapshot(cx);
        let secondary_snapshot = self
            .secondary
            .as_ref()
            .expect("requires split")
            .multibuffer
            .read(cx)
            .snapshot(cx);

        let primary_t = extract(&primary_snapshot);
        let secondary_t = extract(&secondary_snapshot);

        if primary_t != secondary_t {
            self.debug_print(cx);
            pretty_assertions::assert_eq!(primary_t, secondary_t);
        }
    }

    fn unmodified_rows(snapshot: &multi_buffer::MultiBufferSnapshot) -> Vec<Vec<String>> {
        use multi_buffer::MultiBufferRow;

        let mut result: Vec<Vec<String>> = Vec::new();
        let mut current_group: Vec<String> = Vec::new();

        for (line, row_info) in snapshot
            .text()
            .split("\n")
            .zip(snapshot.row_infos(MultiBufferRow(0)))
        {
            if row_info.diff_status.is_none() {
                current_group.push(line.to_owned());
            } else {
                if !current_group.is_empty() {
                    result.push(std::mem::take(&mut current_group));
                }
            }
        }

        if !current_group.is_empty() {
            result.push(current_group);
        }

        result
    }

    fn debug_print(&self, cx: &mut App) {
        use crate::DisplayRow;
        use crate::display_map::Block;
        use buffer_diff::DiffHunkStatusKind;

        assert!(
            self.secondary.is_some(),
            "debug_print is only useful when secondary editor exists"
        );

        let secondary = self.secondary.as_ref().unwrap();

        // Get terminal width, default to 80 if unavailable
        let terminal_width = std::env::var("COLUMNS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(80);

        // Each side gets half the terminal width minus the separator
        let separator = " │ ";
        let side_width = (terminal_width - separator.len()) / 2;

        // Get display snapshots for both editors
        let secondary_snapshot = secondary.editor.update(cx, |editor, cx| {
            editor.display_map.update(cx, |map, cx| map.snapshot(cx))
        });
        let primary_snapshot = self.primary_editor.update(cx, |editor, cx| {
            editor.display_map.update(cx, |map, cx| map.snapshot(cx))
        });

        let secondary_max_row = secondary_snapshot.max_point().row().0;
        let primary_max_row = primary_snapshot.max_point().row().0;
        let max_row = secondary_max_row.max(primary_max_row);

        // Build a map from display row -> block type string
        // Each row of a multi-row block gets an entry with the same block type
        // For spacers, the ID is included in brackets
        fn build_block_map(
            snapshot: &crate::DisplaySnapshot,
            max_row: u32,
        ) -> std::collections::HashMap<u32, String> {
            let mut block_map = std::collections::HashMap::new();
            for (start_row, block) in
                snapshot.blocks_in_range(DisplayRow(0)..DisplayRow(max_row + 1))
            {
                let (block_type, height) = match block {
                    Block::Spacer { id, height } => (format!("SPACER[{}]", id.0), *height),
                    Block::ExcerptBoundary { height, .. } => {
                        ("EXCERPT_BOUNDARY".to_string(), *height)
                    }
                    Block::BufferHeader { height, .. } => ("BUFFER_HEADER".to_string(), *height),
                    Block::FoldedBuffer { height, .. } => ("FOLDED_BUFFER".to_string(), *height),
                    Block::Custom(custom) => {
                        ("CUSTOM_BLOCK".to_string(), custom.height.unwrap_or(1))
                    }
                };
                for offset in 0..height {
                    block_map.insert(start_row.0 + offset, block_type.clone());
                }
            }
            block_map
        }

        let secondary_blocks = build_block_map(&secondary_snapshot, secondary_max_row);
        let primary_blocks = build_block_map(&primary_snapshot, primary_max_row);

        // Helper to truncate a line with ellipsis if too long
        fn truncate_line(line: &str, max_width: usize) -> String {
            let char_count = line.chars().count();
            if char_count <= max_width {
                return line.to_string();
            }
            if max_width < 9 {
                return line.chars().take(max_width).collect();
            }
            let prefix_len = 3;
            let suffix_len = 3;
            let ellipsis = "...";
            let prefix: String = line.chars().take(prefix_len).collect();
            let suffix: String = line
                .chars()
                .rev()
                .take(suffix_len)
                .collect::<String>()
                .chars()
                .rev()
                .collect();
            format!("{}{}{}", prefix, ellipsis, suffix)
        }

        // Helper to format a single row for one side
        // Format: "ln# diff bytes(cumul) text" or block info
        // Line numbers come from buffer_row in RowInfo (1-indexed for display)
        fn format_row(
            row: u32,
            max_row: u32,
            snapshot: &crate::DisplaySnapshot,
            blocks: &std::collections::HashMap<u32, String>,
            row_infos: &[multi_buffer::RowInfo],
            cumulative_bytes: &[usize],
            side_width: usize,
        ) -> String {
            // Get row info if available
            let row_info = row_infos.get(row as usize);

            // Line number prefix (3 chars + space)
            // Use buffer_row from RowInfo, which is None for block rows
            let line_prefix = if row > max_row {
                "    ".to_string()
            } else if let Some(buffer_row) = row_info.and_then(|info| info.buffer_row) {
                format!("{:>3} ", buffer_row + 1) // 1-indexed for display
            } else {
                "    ".to_string() // block rows have no line number
            };
            let content_width = side_width.saturating_sub(line_prefix.len());

            if row > max_row {
                return format!("{}{}", line_prefix, " ".repeat(content_width));
            }

            // Check if this row is a block row
            if let Some(block_type) = blocks.get(&row) {
                let block_str = format!("~~~[{}]~~~", block_type);
                let formatted = format!("{:^width$}", block_str, width = content_width);
                return format!(
                    "{}{}",
                    line_prefix,
                    truncate_line(&formatted, content_width)
                );
            }

            // Get line text
            let line_text = snapshot.line(DisplayRow(row));
            let line_bytes = line_text.len();

            // Diff status marker
            let diff_marker = match row_info.and_then(|info| info.diff_status.as_ref()) {
                Some(status) => match status.kind {
                    DiffHunkStatusKind::Added => "+",
                    DiffHunkStatusKind::Deleted => "-",
                    DiffHunkStatusKind::Modified => "~",
                },
                None => " ",
            };

            // Cumulative bytes
            let cumulative = cumulative_bytes.get(row as usize).copied().unwrap_or(0);

            // Format: "diff bytes(cumul) text" - use 3 digits for bytes, 4 for cumulative
            let info_prefix = format!("{}{:>3}({:>4}) ", diff_marker, line_bytes, cumulative);
            let text_width = content_width.saturating_sub(info_prefix.len());
            let truncated_text = truncate_line(&line_text, text_width);

            format!(
                "{}{}{:<width$}",
                line_prefix,
                info_prefix,
                truncated_text,
                width = text_width
            )
        }

        // Collect row infos for both sides
        let secondary_row_infos: Vec<_> = secondary_snapshot
            .row_infos(DisplayRow(0))
            .take((secondary_max_row + 1) as usize)
            .collect();
        let primary_row_infos: Vec<_> = primary_snapshot
            .row_infos(DisplayRow(0))
            .take((primary_max_row + 1) as usize)
            .collect();

        // Calculate cumulative bytes for each side (only counting non-block rows)
        let mut secondary_cumulative = Vec::with_capacity((secondary_max_row + 1) as usize);
        let mut cumulative = 0usize;
        for row in 0..=secondary_max_row {
            if !secondary_blocks.contains_key(&row) {
                cumulative += secondary_snapshot.line(DisplayRow(row)).len() + 1; // +1 for newline
            }
            secondary_cumulative.push(cumulative);
        }

        let mut primary_cumulative = Vec::with_capacity((primary_max_row + 1) as usize);
        cumulative = 0;
        for row in 0..=primary_max_row {
            if !primary_blocks.contains_key(&row) {
                cumulative += primary_snapshot.line(DisplayRow(row)).len() + 1;
            }
            primary_cumulative.push(cumulative);
        }

        // Print header
        eprintln!();
        eprintln!("{}", "═".repeat(terminal_width));
        let header_left = format!("{:^width$}", "SECONDARY (LEFT)", width = side_width);
        let header_right = format!("{:^width$}", "PRIMARY (RIGHT)", width = side_width);
        eprintln!("{}{}{}", header_left, separator, header_right);
        eprintln!(
            "{:^width$}{}{:^width$}",
            "ln# diff len(cum) text",
            separator,
            "ln# diff len(cum) text",
            width = side_width
        );
        eprintln!("{}", "─".repeat(terminal_width));

        // Print each row
        for row in 0..=max_row {
            let left = format_row(
                row,
                secondary_max_row,
                &secondary_snapshot,
                &secondary_blocks,
                &secondary_row_infos,
                &secondary_cumulative,
                side_width,
            );
            let right = format_row(
                row,
                primary_max_row,
                &primary_snapshot,
                &primary_blocks,
                &primary_row_infos,
                &primary_cumulative,
                side_width,
            );
            eprintln!("{}{}{}", left, separator, right);
        }

        eprintln!("{}", "═".repeat(terminal_width));
        eprintln!("Legend: + added, - deleted, ~ modified, ~~~ block/spacer row");
        eprintln!();
    }

    fn randomly_edit_excerpts(
        &mut self,
        rng: &mut impl rand::Rng,
        mutation_count: usize,
        cx: &mut Context<Self>,
    ) {
        use collections::HashSet;
        use rand::prelude::*;
        use std::env;
        use util::RandomCharIter;

        let max_excerpts = env::var("MAX_EXCERPTS")
            .map(|i| i.parse().expect("invalid `MAX_EXCERPTS` variable"))
            .unwrap_or(5);

        for _ in 0..mutation_count {
            let paths = self
                .primary_multibuffer
                .read(cx)
                .paths()
                .cloned()
                .collect::<Vec<_>>();
            let excerpt_ids = self.primary_multibuffer.read(cx).excerpt_ids();

            if rng.random_bool(0.1) && !excerpt_ids.is_empty() {
                let mut excerpts = HashSet::default();
                for _ in 0..rng.random_range(0..excerpt_ids.len()) {
                    excerpts.extend(excerpt_ids.choose(rng).copied());
                }

                let line_count = rng.random_range(0..5);

                log::info!("Expanding excerpts {excerpts:?} by {line_count} lines");

                self.expand_excerpts(
                    excerpts.iter().cloned(),
                    line_count,
                    ExpandExcerptDirection::UpAndDown,
                    cx,
                );
                continue;
            }

            if excerpt_ids.is_empty() || (rng.random() && excerpt_ids.len() < max_excerpts) {
                let len = rng.random_range(100..500);
                let text = RandomCharIter::new(&mut *rng).take(len).collect::<String>();
                let buffer = cx.new(|cx| Buffer::local(text, cx));
                log::info!(
                    "Creating new buffer {} with text: {:?}",
                    buffer.read(cx).remote_id(),
                    buffer.read(cx).text()
                );
                let buffer_snapshot = buffer.read(cx).snapshot();
                let diff = cx.new(|cx| BufferDiff::new_unchanged(&buffer_snapshot, cx));
                // Create some initial diff hunks.
                buffer.update(cx, |buffer, cx| {
                    buffer.randomly_edit(rng, 1, cx);
                });
                let buffer_snapshot = buffer.read(cx).text_snapshot();
                let ranges = diff.update(cx, |diff, cx| {
                    diff.recalculate_diff_sync(&buffer_snapshot, cx);
                    diff.snapshot(cx)
                        .hunks(&buffer_snapshot)
                        .map(|hunk| hunk.buffer_range.to_point(&buffer_snapshot))
                        .collect::<Vec<_>>()
                });
                let path = PathKey::for_buffer(&buffer, cx);
                self.set_excerpts_for_path(path, buffer, ranges, 2, diff, cx);
            } else {
                let remove_count = rng.random_range(1..=paths.len());
                let paths_to_remove = paths
                    .choose_multiple(rng, remove_count)
                    .cloned()
                    .collect::<Vec<_>>();
                for path in paths_to_remove {
                    self.remove_excerpts_for_path(path.clone(), cx);
                }
            }
        }
    }
}

impl EventEmitter<EditorEvent> for SplittableEditor {}
impl Focusable for SplittableEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.primary_editor.read(cx).focus_handle(cx)
    }
}

impl Render for SplittableEditor {
    fn render(
        &mut self,
        _window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let inner = if self.secondary.is_some() {
            let style = self.primary_editor.read(cx).create_style(cx);
            SplitEditorView::new(cx.entity().clone(), style, self.split_state.clone())
                .into_any_element()
        } else {
            self.primary_editor.clone().into_any_element()
        };
        div()
            .id("splittable-editor")
            .on_action(cx.listener(Self::split))
            .on_action(cx.listener(Self::unsplit))
            .on_action(cx.listener(Self::activate_pane_left))
            .on_action(cx.listener(Self::activate_pane_right))
            .on_action(cx.listener(Self::jump_to_corresponding_row))
            .size_full()
            .child(inner)
    }
}

struct ExcerptMapping {
    left: ExcerptId,
    right: ExcerptId,
}

impl SecondaryEditor {
    fn update_path_excerpts_from_primary(
        &mut self,
        path_key: PathKey,
        primary_multibuffer: &Entity<MultiBuffer>,
        diff: Entity<BufferDiff>,
        cx: &mut App,
    ) -> Vec<ExcerptMapping> {
        let primary_multibuffer_ref = primary_multibuffer.read(cx);
        let primary_excerpt_ids: Vec<ExcerptId> = primary_multibuffer_ref
            .excerpts_for_path(&path_key)
            .collect();

        let Some(excerpt_id) = primary_multibuffer_ref.excerpts_for_path(&path_key).next() else {
            self.multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.remove_excerpts_for_path(path_key, cx);
            });
            return Vec::new();
        };

        let primary_multibuffer_snapshot = primary_multibuffer_ref.snapshot(cx);
        let main_buffer = primary_multibuffer_snapshot
            .buffer_for_excerpt(excerpt_id)
            .unwrap();
        let base_text_buffer = diff.read(cx).base_text_buffer();
        let diff_snapshot = diff.read(cx).snapshot(cx);
        let base_text_buffer_snapshot = base_text_buffer.read(cx).snapshot();
        let new = primary_multibuffer_ref
            .excerpts_for_buffer(main_buffer.remote_id(), cx)
            .into_iter()
            .map(|(_, excerpt_range)| {
                let point_range_to_base_text_point_range = |range: Range<Point>| {
                    let start_row = diff_snapshot
                        .rows_to_base_text_rows(range.start.row..range.start.row, main_buffer)
                        .next()
                        .unwrap()
                        .start;
                    let end_row = diff_snapshot
                        .rows_to_base_text_rows(range.end.row..range.end.row, main_buffer)
                        .next()
                        .unwrap()
                        .end;
                    let end_column = diff_snapshot.base_text().line_len(end_row);
                    Point::new(start_row, 0)..Point::new(end_row, end_column)
                };
                let primary = excerpt_range.primary.to_point(main_buffer);
                let context = excerpt_range.context.to_point(main_buffer);
                ExcerptRange {
                    primary: point_range_to_base_text_point_range(primary),
                    context: point_range_to_base_text_point_range(context),
                }
            })
            .collect();

        let main_buffer = primary_multibuffer_ref
            .buffer(main_buffer.remote_id())
            .unwrap();

        self.editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                let (ids, _) = buffer.update_path_excerpts(
                    path_key.clone(),
                    base_text_buffer.clone(),
                    &base_text_buffer_snapshot,
                    new,
                    cx,
                );
                if !ids.is_empty()
                    && buffer
                        .diff_for(base_text_buffer.read(cx).remote_id())
                        .is_none_or(|old_diff| old_diff.entity_id() != diff.entity_id())
                {
                    buffer.add_inverted_diff(diff, main_buffer, cx);
                }
            })
        });

        let secondary_excerpt_ids: Vec<ExcerptId> = self
            .multibuffer
            .read(cx)
            .excerpts_for_path(&path_key)
            .collect();

        debug_assert_eq!(primary_excerpt_ids.len(), secondary_excerpt_ids.len());

        primary_excerpt_ids
            .into_iter()
            .zip(secondary_excerpt_ids)
            .map(|(right, left)| ExcerptMapping { right, left })
            .collect()
    }

    fn sync_path_excerpts(
        &mut self,
        path_key: PathKey,
        primary_multibuffer: &Entity<MultiBuffer>,
        diff: Entity<BufferDiff>,
        primary_display_map: &Entity<DisplayMap>,
        secondary_display_map: &Entity<DisplayMap>,
        cx: &mut App,
    ) {
        self.remove_mappings_for_path(
            &path_key,
            primary_multibuffer,
            primary_display_map,
            secondary_display_map,
            cx,
        );

        let mappings =
            self.update_path_excerpts_from_primary(path_key, primary_multibuffer, diff.clone(), cx);

        let secondary_buffer_id = diff.read(cx).base_text(cx).remote_id();
        let primary_buffer_id = diff.read(cx).buffer_id;

        if let Some(companion) = primary_display_map.read(cx).companion().cloned() {
            companion.update(cx, |c, _| {
                for mapping in mappings {
                    c.add_excerpt_mapping(mapping.left, mapping.right);
                }
                c.add_buffer_mapping(secondary_buffer_id, primary_buffer_id);
            });
        }
    }

    fn remove_mappings_for_path(
        &self,
        path_key: &PathKey,
        primary_multibuffer: &Entity<MultiBuffer>,
        primary_display_map: &Entity<DisplayMap>,
        _secondary_display_map: &Entity<DisplayMap>,
        cx: &mut App,
    ) {
        let primary_excerpt_ids: Vec<ExcerptId> = primary_multibuffer
            .read(cx)
            .excerpts_for_path(path_key)
            .collect();
        let secondary_excerpt_ids: Vec<ExcerptId> = self
            .multibuffer
            .read(cx)
            .excerpts_for_path(path_key)
            .collect();

        if let Some(companion) = primary_display_map.read(cx).companion().cloned() {
            companion.update(cx, |c, _| {
                c.remove_excerpt_mappings(secondary_excerpt_ids, primary_excerpt_ids);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use fs::FakeFs;
    use gpui::AppContext as _;
    use language::Capability;
    use multi_buffer::{MultiBuffer, PathKey};
    use project::Project;
    use rand::rngs::StdRng;
    use settings::SettingsStore;
    use ui::VisualContext as _;
    use workspace::Workspace;

    use crate::SplittableEditor;

    fn init_test(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
    }

    #[gpui::test(iterations = 100)]
    async fn test_random_split_editor(mut rng: StdRng, cx: &mut gpui::TestAppContext) {
        use rand::prelude::*;

        init_test(cx);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        let editor = cx.new_window_entity(|window, cx| {
            let mut editor =
                SplittableEditor::new_unsplit(primary_multibuffer, project, workspace, window, cx);
            editor.split(&Default::default(), window, cx);
            editor
        });

        let operations = std::env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(20);
        let rng = &mut rng;
        for _ in 0..operations {
            let buffers = editor.update(cx, |editor, cx| {
                editor
                    .primary_editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .all_buffers()
            });

            if buffers.is_empty() {
                editor.update(cx, |editor, cx| {
                    editor.randomly_edit_excerpts(rng, 2, cx);
                    editor.check_invariants(true, cx);
                });
                continue;
            }

            let mut quiesced = false;

            match rng.random_range(0..100) {
                0..=69 if !buffers.is_empty() => {
                    let buffer = buffers.iter().choose(rng).unwrap();
                    buffer.update(cx, |buffer, cx| {
                        if rng.random() {
                            log::info!("randomly editing single buffer");
                            buffer.randomly_edit(rng, 5, cx);
                        } else {
                            log::info!("randomly undoing/redoing in single buffer");
                            buffer.randomly_undo_redo(rng, cx);
                        }
                    });
                }
                70..=79 => {
                    log::info!("mutating excerpts");
                    editor.update(cx, |editor, cx| {
                        editor.randomly_edit_excerpts(rng, 2, cx);
                    });
                }
                80..=89 if !buffers.is_empty() => {
                    log::info!("recalculating buffer diff");
                    let buffer = buffers.iter().choose(rng).unwrap();
                    editor.update(cx, |editor, cx| {
                        let diff = editor
                            .primary_multibuffer
                            .read(cx)
                            .diff_for(buffer.read(cx).remote_id())
                            .unwrap();
                        let buffer_snapshot = buffer.read(cx).text_snapshot();
                        diff.update(cx, |diff, cx| {
                            diff.recalculate_diff_sync(&buffer_snapshot, cx);
                        });
                    });
                }
                _ => {
                    log::info!("quiescing");
                    for buffer in buffers {
                        let buffer_snapshot =
                            buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
                        let diff = editor.update(cx, |editor, cx| {
                            editor
                                .primary_multibuffer
                                .read(cx)
                                .diff_for(buffer.read(cx).remote_id())
                                .unwrap()
                        });
                        diff.update(cx, |diff, cx| {
                            diff.recalculate_diff_sync(&buffer_snapshot, cx);
                        });
                        cx.run_until_parked();
                        let diff_snapshot = diff.read_with(cx, |diff, cx| diff.snapshot(cx));
                        let ranges = diff_snapshot
                            .hunks(&buffer_snapshot)
                            .map(|hunk| hunk.range)
                            .collect::<Vec<_>>();
                        editor.update(cx, |editor, cx| {
                            let path = PathKey::for_buffer(&buffer, cx);
                            editor.set_excerpts_for_path(path, buffer, ranges, 2, diff, cx);
                        });
                        quiesced = true;
                    }
                }
            }

            editor.update(cx, |editor, cx| {
                editor.check_invariants(quiesced, cx);
            });
        }
    }

    #[gpui::test]
    async fn test_split_editor_block_alignment(cx: &mut gpui::TestAppContext) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let base_text = "
            aaa
            bbb
            ccc
            ddd
            eee
            fff
        "
        .unindent();
        let current_text = "
            aaa
            ddd
            eee
            fff
        "
        .unindent();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        // Add excerpts covering the whole buffer
        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee
            fff"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
           § <no file>
           § -----
           aaa
           bbb
           ccc
           ddd
           eee
           fff"
            .unindent()
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(3, 0)..Point::new(3, 3), "FFF")], None, cx);
        });

        cx.run_until_parked();

        // Check state after edit
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee
            FFF"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );

        // Recalculate diff to include the new lines in the diff
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee
            FFF"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_block_alignment_after_deleting_unmodified_line(
        cx: &mut gpui::TestAppContext,
    ) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // Start with base text and current text that are identical
        // This means no diff hunks initially
        let base_text = "
            aaa
            bbb
            ccc
            ddd
            eee
            fff
        "
        .unindent();
        let current_text = base_text.clone();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        // Add excerpts covering the whole buffer
        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        // Initial state: both sides should be identical with no spacers
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );

        // Delete an unmodified line (delete "ccc" which is row 2, 0-indexed)
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(3, 0), "")], None, cx);
        });

        cx.run_until_parked();

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        cx.update(|_window, cx| {
            editor.update(cx, |editor, cx| {
                editor.debug_print(cx);
            });
        });

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            § spacer
            ddd
            eee
            fff"
            .unindent()
        );
        // Secondary should still show "ccc" because diff hasn't been recalculated yet
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );

        // Now recalculate the diff - this should mark "ccc" as deleted in the diff
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        // After diff recalculation: the primary should show a spacer for the deleted "ccc" line
        // and the secondary should still show "ccc"
        // BUG: The secondary erroneously removes its spacer after diff recalculation,
        // causing misalignment
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            § spacer
            ddd
            eee
            fff"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_block_alignment_after_undoing_deleted_unmodified_line(
        cx: &mut gpui::TestAppContext,
    ) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // Start with a diff where some lines are already deleted from base.
        // This creates an initial state with spacers on the primary (RHS) side.
        let base_text = "
            aaa
            bbb
            ccc
            ddd
            eee
            fff
        "
        .unindent();
        // Current text is missing "bbb" and "ccc" compared to base
        let current_text = "
            aaa
            ddd
            eee
            fff
        "
        .unindent();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        // Initial state: primary (RHS) has spacers for deleted "bbb" and "ccc"
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee
            fff"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );

        // Delete an unmodified line ("eee" at row 2 in current text)
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(3, 0), "")], None, cx);
        });

        cx.run_until_parked();

        // After deletion: primary has an additional spacer for deleted "eee"
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            § spacer
            fff"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );

        // Recalculate the diff to reflect the deletion.
        // This simulates what happens after the 250ms debounce timer fires in real usage.
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        // After diff recalculation: diff now includes "eee" as a deleted hunk
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            § spacer
            fff"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent()
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "xxx\n")], None, cx);
        });

        cx.run_until_parked();

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        cx.update(|_window, cx| {
            editor.update(cx, |editor, cx| {
                editor.debug_print(cx);
            });
        });

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            § spacer
            xxx
            fff"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            § spacer
            fff"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_block_alignment_after_deleting_added_line(
        cx: &mut gpui::TestAppContext,
    ) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // Base text has 4 lines
        let base_text = "
            aaa
            bbb
            ccc
            ddd
        "
        .unindent();

        // Current text:
        // - "bbb" is deleted from base
        // - two new lines "NEW1" and "NEW2" are added after "aaa"
        let current_text = "
            aaa
            NEW1
            NEW2
            ccc
            ddd
        "
        .unindent();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        // Add excerpts covering the whole buffer
        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        // Initial state:
        // Primary (RHS) shows current text: aaa, NEW1, NEW2, ccc, ddd
        // Secondary (LHS) shows base text: aaa, bbb, ccc, ddd
        // Since 1 line (bbb) was replaced with 2 lines (NEW1, NEW2), secondary needs 1 spacer
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            NEW1
            NEW2
            ccc
            ddd"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            § spacer
            ccc
            ddd"
            .unindent()
        );

        cx.update(|_window, cx| {
            editor.update(cx, |editor, cx| {
                editor.debug_print(cx);
            });
        });

        // Delete the second added line ("NEW2" at row 2 in current text)
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(3, 0), "")], None, cx);
        });

        cx.run_until_parked();

        // After deletion but before diff recalculation:
        // Primary now shows "aaa", "NEW1", "ccc", "ddd" (4 lines)
        // Secondary still shows "aaa", "bbb", "ccc", "ddd" (4 lines)
        // Since primary and secondary now have equal lines, secondary spacer should be removed
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        cx.update(|_window, cx| {
            editor.update(cx, |editor, cx| {
                editor.debug_print(cx);
            });
        });

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            NEW1
            ccc
            ddd"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd"
            .unindent()
        );

        // Recalculate the diff to reflect the deletion
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        // After diff recalculation: should still be aligned
        // Both sides now have equal content lines in the hunk (bbb vs NEW1)
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            NEW1
            ccc
            ddd"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd"
            .unindent()
        );
    }

    // #[gpui::test]
    // async fn test_split_row_translation(cx: &mut gpui::TestAppContext) {
    //     use buffer_diff::BufferDiff;
    //     use language::Buffer;
    //     use text::Bias;

    //     use multi_buffer::MultiBufferRow;
    //     use rope::Point;
    //     use unindent::Unindent as _;

    //     init_test(cx);

    //     let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
    //     let (workspace, cx) =
    //         cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    //     // Buffer A: has insertion + modification
    //     // Base: "one\ntwo\nthree\nfour\n"
    //     // Buffer: "one\nTWO\nINSERTED\nthree\nfour\n"
    //     let buffer_a_base = "
    //         one
    //         two
    //         three
    //         four
    //     "
    //     .unindent();
    //     let buffer_a_text = "
    //         one
    //         TWO
    //         INSERTED
    //         three
    //         four
    //     "
    //     .unindent();

    //     // Buffer B: has deletion
    //     // Base: "alpha\nbeta\ngamma\ndelta\n"
    //     // Buffer: "alpha\ngamma\ndelta\n"
    //     let buffer_b_base = "
    //         alpha
    //         beta
    //         gamma
    //         delta
    //     "
    //     .unindent();
    //     let buffer_b_text = "
    //         alpha
    //         gamma
    //         delta
    //     "
    //     .unindent();

    //     let buffer_a = cx.new(|cx| Buffer::local(buffer_a_text.clone(), cx));
    //     let buffer_b = cx.new(|cx| Buffer::local(buffer_b_text.clone(), cx));

    //     let diff_a = cx.new(|cx| {
    //         BufferDiff::new_with_base_text(&buffer_a_base, &buffer_a.read(cx).text_snapshot(), cx)
    //     });
    //     let diff_b = cx.new(|cx| {
    //         BufferDiff::new_with_base_text(&buffer_b_base, &buffer_b.read(cx).text_snapshot(), cx)
    //     });

    //     let primary_multibuffer = cx.new(|cx| {
    //         let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
    //         multibuffer.set_all_diff_hunks_expanded(cx);
    //         multibuffer
    //     });

    //     let editor = cx.new_window_entity(|window, cx| {
    //         let mut editor = SplittableEditor::new_unsplit(
    //             primary_multibuffer.clone(),
    //             project,
    //             workspace,
    //             window,
    //             cx,
    //         );
    //         editor.split(&Default::default(), window, cx);
    //         editor
    //     });

    //     // Add excerpts for both buffers
    //     let (path_a, path_b) = cx.update(|_, cx| {
    //         (
    //             PathKey::for_buffer(&buffer_a, cx),
    //             PathKey::for_buffer(&buffer_b, cx),
    //         )
    //     });

    //     editor.update(cx, |editor, cx| {
    //         // Buffer A: excerpt covering the whole buffer
    //         editor.set_excerpts_for_path(
    //             path_a.clone(),
    //             buffer_a.clone(),
    //             vec![Point::new(0, 0)..Point::new(4, 0)],
    //             0,
    //             diff_a.clone(),
    //             cx,
    //         );
    //         // Buffer B: excerpt covering the whole buffer
    //         editor.set_excerpts_for_path(
    //             path_b.clone(),
    //             buffer_b.clone(),
    //             vec![Point::new(0, 0)..Point::new(3, 0)],
    //             0,
    //             diff_b.clone(),
    //             cx,
    //         );
    //     });

    //     // Now test the row translation
    //     editor.update(cx, |editor, cx| {
    //         let primary_display_map = editor.primary_editor.read(cx).display_map.clone();
    //         let secondary = editor.secondary.as_ref().unwrap();
    //         let secondary_display_map = secondary.editor.read(cx).display_map.clone();

    //         // Get snapshots and closures
    //         // The closure on primary_display_map converts secondary (companion) rows -> primary rows
    //         // The closure on secondary_display_map converts primary (companion) rows -> secondary rows
    //         let (
    //             primary_buffer,
    //             secondary_to_primary_excerpt_mapping,
    //             convert_secondary_to_primary,
    //         ) = primary_display_map.update(cx, |dm, cx| {
    //             let snapshot = dm.snapshot(cx);
    //             let companion = dm.companion().unwrap().read(cx);
    //             let excerpt_mapping = companion
    //                 .companion_excerpt_to_excerpt(dm.entity_id())
    //                 .clone();
    //             let convert = companion.convert_row_from_companion(dm.entity_id());
    //             (snapshot.buffer_snapshot().clone(), excerpt_mapping, convert)
    //         });

    //         let (
    //             secondary_buffer,
    //             primary_to_secondary_excerpt_mapping,
    //             convert_primary_to_secondary,
    //         ) = secondary_display_map.update(cx, |dm, cx| {
    //             let snapshot = dm.snapshot(cx);
    //             let companion = dm.companion().unwrap().read(cx);
    //             let excerpt_mapping = companion
    //                 .companion_excerpt_to_excerpt(dm.entity_id())
    //                 .clone();
    //             let convert = companion.convert_row_from_companion(dm.entity_id());
    //             (snapshot.buffer_snapshot().clone(), excerpt_mapping, convert)
    //         });

    //         // Primary shows modified text: "one\nTWO\nINSERTED\nthree\nfour\n"
    //         // Secondary shows base text: "one\ntwo\nthree\nfour\n"

    //         // Test primary -> secondary translation
    //         // Primary row 0 ("one") -> Secondary row 0 ("one")
    //         assert_eq!(
    //             convert_primary_to_secondary(
    //                 &primary_to_secondary_excerpt_mapping,
    //                 &secondary_buffer,
    //                 &primary_buffer,
    //                 MultiBufferRow(0),
    //                 Bias::Left
    //             ),
    //             MultiBufferRow(0),
    //             "primary row 0 (one) -> secondary row 0 (one)"
    //         );

    //         // Primary row 1 ("TWO") -> Secondary row 1 ("two")
    //         assert_eq!(
    //             convert_primary_to_secondary(
    //                 &primary_to_secondary_excerpt_mapping,
    //                 &secondary_buffer,
    //                 &primary_buffer,
    //                 MultiBufferRow(1),
    //                 Bias::Left
    //             ),
    //             MultiBufferRow(1),
    //             "primary row 1 (TWO) -> secondary row 1 (two)"
    //         );

    //         // Primary row 2 ("INSERTED") is an inserted line, should map to row 1 or 2
    //         let inserted_row_left = convert_primary_to_secondary(
    //             &primary_to_secondary_excerpt_mapping,
    //             &secondary_buffer,
    //             &primary_buffer,
    //             MultiBufferRow(2),
    //             Bias::Left,
    //         );
    //         assert!(
    //             inserted_row_left.0 == 1 || inserted_row_left.0 == 2,
    //             "primary row 2 (INSERTED) with Bias::Left should map to 1 or 2, got {}",
    //             inserted_row_left.0
    //         );
    //         let inserted_row_right = convert_primary_to_secondary(
    //             &primary_to_secondary_excerpt_mapping,
    //             &secondary_buffer,
    //             &primary_buffer,
    //             MultiBufferRow(2),
    //             Bias::Right,
    //         );
    //         assert!(
    //             inserted_row_right.0 == 1 || inserted_row_right.0 == 2,
    //             "primary row 2 (INSERTED) with Bias::Right should map to 1 or 2, got {}",
    //             inserted_row_right.0
    //         );

    //         // Primary row 3 ("three") -> Secondary row 2 ("three")
    //         assert_eq!(
    //             convert_primary_to_secondary(
    //                 &primary_to_secondary_excerpt_mapping,
    //                 &secondary_buffer,
    //                 &primary_buffer,
    //                 MultiBufferRow(3),
    //                 Bias::Left
    //             ),
    //             MultiBufferRow(2),
    //             "primary row 3 (three) -> secondary row 2 (three)"
    //         );

    //         // Test secondary -> primary translation
    //         // Secondary row 0 ("one") -> Primary row 0 ("one")
    //         assert_eq!(
    //             convert_secondary_to_primary(
    //                 &secondary_to_primary_excerpt_mapping,
    //                 &primary_buffer,
    //                 &secondary_buffer,
    //                 MultiBufferRow(0),
    //                 Bias::Left
    //             ),
    //             MultiBufferRow(0),
    //             "secondary row 0 (one) -> primary row 0 (one)"
    //         );

    //         // Secondary row 1 ("two") -> Primary row 1 ("TWO")
    //         assert_eq!(
    //             convert_secondary_to_primary(
    //                 &secondary_to_primary_excerpt_mapping,
    //                 &primary_buffer,
    //                 &secondary_buffer,
    //                 MultiBufferRow(1),
    //                 Bias::Left
    //             ),
    //             MultiBufferRow(1),
    //             "secondary row 1 (two) -> primary row 1 (TWO)"
    //         );

    //         // Secondary row 2 ("three") -> Primary row 3 ("three")
    //         assert_eq!(
    //             convert_secondary_to_primary(
    //                 &secondary_to_primary_excerpt_mapping,
    //                 &primary_buffer,
    //                 &secondary_buffer,
    //                 MultiBufferRow(2),
    //                 Bias::Left
    //             ),
    //             MultiBufferRow(3),
    //             "secondary row 2 (three) -> primary row 3 (three)"
    //         );
    //     });
    // }

    #[gpui::test]
    async fn test_split_editor_newline_insertion_at_line_boundary(cx: &mut gpui::TestAppContext) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let base_text = "
            aaa
            bbb

            ccc
            ddd
        "
        .unindent();
        let current_text = base_text.clone();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        // Insert a newline at the start of the existing blank line (row 2)
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "\n")], None, cx);
        });

        cx.run_until_parked();

        eprintln!("=== State after inserting newline (before diff recalculation) ===");
        cx.update(|_window, cx| {
            editor.update(cx, |editor, cx| {
                editor.debug_print(cx);
            });
        });

        // Before diff recalculation: the primary has an extra blank line,
        // and the secondary should have one spacer to compensate
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb


            ccc
            ddd"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            § spacer

            ccc
            ddd"
            .unindent()
        );

        // Recalculate diff
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        eprintln!("=== State after diff recalculation ===");
        cx.update(|_window, cx| {
            editor.update(cx, |editor, cx| {
                editor.debug_print(cx);
            });
        });

        // After diff recalculation: the state should remain aligned.
        // The new blank line is now recognized as an addition by the diff,
        // but the spacer arrangement should stay the same.
        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb


            ccc
            ddd"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            § spacer

            ccc
            ddd"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_revert_deletion_hunk(cx: &mut gpui::TestAppContext) {
        use buffer_diff::BufferDiff;
        use git::Restore;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let base_text = "
            aaa
            bbb
            ccc
            ddd
            eee
        "
        .unindent();
        let current_text = "
            aaa
            ddd
            eee
        "
        .unindent();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent()
        );

        cx.update_window_entity(&primary_editor, |editor, window, cx| {
            editor.change_selections(crate::SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([Point::new(1, 0)..Point::new(1, 0)]);
            });
            editor.git_restore(&Restore, window, cx);
        });

        cx.run_until_parked();

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent()
        );

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_delete_added_lines_from_balanced_hunk(
        cx: &mut gpui::TestAppContext,
    ) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let base_text = "
            aaa
            old1
            old2
            old3
            old4
            zzz
        "
        .unindent();

        let current_text = "
            aaa
            new1
            new2
            new3
            new4
            zzz
        "
        .unindent();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(4, 0)..Point::new(5, 0), "")], None, cx);
        });
        cx.run_until_parked();

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });
        cx.run_until_parked();

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(3, 0), "")], None, cx);
        });
        cx.run_until_parked();

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            new1
            new3
            § spacer
            § spacer
            zzz"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            old1
            old2
            old3
            old4
            zzz"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_deletion_at_beginning(cx: &mut gpui::TestAppContext) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let base_text = "
            aaa
            bbb
            ccc
            ddd
            eee
        "
        .unindent();
        let current_text = "
            ccc
            ddd
            eee
        "
        .unindent();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            § spacer
            § spacer
            ccc
            ddd
            eee"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_deletion_at_end(cx: &mut gpui::TestAppContext) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let base_text1 = "
            aaa
            bbb
            ccc
            ddd
            eee
        "
        .unindent();
        let current_text1 = "
            aaa
            bbb
            ccc
        "
        .unindent();

        let base_text2 = "
            xxx
            yyy
        "
        .unindent();
        let current_text2 = base_text2.clone();

        let buffer1 = cx.new(|cx| Buffer::local(current_text1.clone(), cx));
        let diff1 = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text1, &buffer1.read(cx).text_snapshot(), cx)
        });

        let buffer2 = cx.new(|cx| Buffer::local(current_text2.clone(), cx));
        let diff2 = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text2, &buffer2.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        let path1 = cx.update(|_, cx| PathKey::for_buffer(&buffer1, cx));
        let path2 = cx.update(|_, cx| PathKey::for_buffer(&buffer2, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path1.clone(),
                buffer1.clone(),
                vec![Point::new(0, 0)..buffer1.read(cx).max_point()],
                0,
                diff1.clone(),
                cx,
            );
            editor.set_excerpts_for_path(
                path2.clone(),
                buffer2.clone(),
                vec![Point::new(0, 0)..buffer2.read(cx).max_point()],
                1,
                diff2.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            § spacer
            § spacer

            § <no file>
            § -----
            xxx
            yyy"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee

            § <no file>
            § -----
            xxx
            yyy"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_deletion_at_excerpt_boundary(cx: &mut gpui::TestAppContext) {
        use buffer_diff::BufferDiff;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::test::editor_content_with_blocks;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, mut cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let base_text = "
            aaa
            bbb
            ccc
            deleted1
            deleted2
            zzz
        "
        .unindent();
        let current_text = "
            aaa
            bbb
            ccc
            zzz
        "
        .unindent();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![
                    Point::new(0, 0)..Point::new(3, 0),
                    Point::new(3, 0)..buffer.read(cx).max_point(),
                ],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        let primary_content = editor_content_with_blocks(&primary_editor, &mut cx);
        let secondary_content = editor_content_with_blocks(&secondary_editor, &mut cx);

        assert_eq!(
            primary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            § spacer
            § spacer
            § -----
            zzz"
            .unindent()
        );
        assert_eq!(
            secondary_content,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            deleted1
            deleted2
            § -----
            zzz"
            .unindent()
        );
    }

    #[gpui::test]
    async fn test_split_editor_soft_wrap_at_excerpt_end(cx: &mut gpui::TestAppContext) {
        use buffer_diff::BufferDiff;
        use gpui::px;
        use language::Buffer;
        use rope::Point;
        use unindent::Unindent as _;

        use crate::DisplayRow;
        use crate::display_map::Block;

        init_test(cx);

        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let base_text = "
            aaa
            this is a long line that will soft wrap when the viewport is narrow enough
        "
        .unindent();
        let current_text = base_text.clone();

        let buffer = cx.new(|cx| Buffer::local(current_text.clone(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(&base_text, &buffer.read(cx).text_snapshot(), cx)
        });

        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = SplittableEditor::new_unsplit(
                primary_multibuffer.clone(),
                project.clone(),
                workspace,
                window,
                cx,
            );
            editor.split(&Default::default(), window, cx);
            editor
        });

        let path = cx.update(|_, cx| PathKey::for_buffer(&buffer, cx));

        editor.update(cx, |editor, cx| {
            editor.set_excerpts_for_path(
                path.clone(),
                buffer.clone(),
                vec![Point::new(0, 0)..Point::new(2, 0)],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        primary_editor.update(cx, |editor, cx| {
            editor.set_wrap_width(Some(px(50.0)), cx);
        });

        cx.run_until_parked();

        let primary_wrap_row_count = primary_editor.update(cx, |editor, cx| {
            editor
                .display_map
                .update(cx, |map, cx| map.snapshot(cx).max_point().row().0 + 1)
        });

        let secondary_spacer_count = secondary_editor.update(cx, |editor, cx| {
            editor.display_map.update(cx, |map, cx| {
                let snapshot = map.snapshot(cx);
                snapshot
                    .blocks_in_range(DisplayRow(0)..snapshot.max_point().row())
                    .filter(|(_, block)| matches!(block, Block::Spacer { .. }))
                    .count()
            })
        });

        assert!(
            primary_wrap_row_count > 4,
            "Primary should have soft-wrapped lines (got {} rows)",
            primary_wrap_row_count
        );

        let expected_spacers = primary_wrap_row_count - 4;
        assert_eq!(
            secondary_spacer_count, expected_spacers as usize,
            "Secondary (LHS) should have {} spacer(s) to match the extra wrap rows on primary (RHS)",
            expected_spacers
        );
    }
}
