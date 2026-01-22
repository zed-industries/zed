use std::ops::{Bound, Range};

use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use collections::HashMap;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    Action, AppContext as _, Entity, EventEmitter, Focusable, NoAction, Subscription, WeakEntity,
};
use language::{Buffer, Capability};
use multi_buffer::{
    Anchor, BufferOffset, ExcerptId, ExcerptRange, ExpandExcerptDirection, MultiBuffer,
    MultiBufferPoint, MultiBufferSnapshot, PathKey,
};
use project::Project;
use rope::Point;
use text::{OffsetRangeExt as _, ToPoint as _};
use ui::{
    App, Context, InteractiveElement as _, IntoElement as _, ParentElement as _, Render,
    Styled as _, Window, div,
};

use crate::{
    display_map::MultiBufferRowMapping,
    split_editor_view::{SplitEditorState, SplitEditorView},
};
use workspace::{
    ActivatePaneLeft, ActivatePaneRight, Item, ItemHandle, Pane, PaneGroup, SplitDirection,
    Workspace,
};

use crate::{
    Autoscroll, DisplayMap, Editor, EditorEvent, ToggleCodeActions, ToggleSoftWrap,
    actions::{DisableBreakpoint, EditLogBreakpoint, EnableBreakpoint, ToggleBreakpoint},
    display_map::Companion,
};
use zed_actions::assistant::InlineAssist;

pub(crate) fn convert_lhs_rows_to_rhs(
    lhs_excerpt_to_rhs_excerpt: &HashMap<ExcerptId, ExcerptId>,
    rhs_snapshot: &MultiBufferSnapshot,
    lhs_snapshot: &MultiBufferSnapshot,
    lhs_bounds: (Bound<MultiBufferPoint>, Bound<MultiBufferPoint>),
) -> Vec<MultiBufferRowMapping> {
    convert_rows(
        lhs_excerpt_to_rhs_excerpt,
        lhs_snapshot,
        rhs_snapshot,
        lhs_bounds,
        |diff, points, buffer| {
            let (points, first_group, prev_boundary) =
                diff.base_text_points_to_points(points, buffer);
            (points.collect(), first_group, prev_boundary)
        },
    )
}

pub(crate) fn convert_rhs_rows_to_lhs(
    rhs_excerpt_to_lhs_excerpt: &HashMap<ExcerptId, ExcerptId>,
    lhs_snapshot: &MultiBufferSnapshot,
    rhs_snapshot: &MultiBufferSnapshot,
    rhs_bounds: (Bound<MultiBufferPoint>, Bound<MultiBufferPoint>),
) -> Vec<MultiBufferRowMapping> {
    convert_rows(
        rhs_excerpt_to_lhs_excerpt,
        rhs_snapshot,
        lhs_snapshot,
        rhs_bounds,
        |diff, points, buffer| {
            let (points, first_group, prev_boundary) =
                diff.points_to_base_text_points(points, buffer);
            (points.collect(), first_group, prev_boundary)
        },
    )
}

fn convert_rows<F>(
    excerpt_map: &HashMap<ExcerptId, ExcerptId>,
    source_snapshot: &MultiBufferSnapshot,
    target_snapshot: &MultiBufferSnapshot,
    source_bounds: (Bound<MultiBufferPoint>, Bound<MultiBufferPoint>),
    translate_fn: F,
) -> Vec<MultiBufferRowMapping>
where
    F: Fn(
        &BufferDiffSnapshot,
        Vec<Point>,
        &text::BufferSnapshot,
    ) -> (
        Vec<Range<Point>>,
        Option<Range<Point>>,
        Option<(Point, Range<Point>)>,
    ),
{
    let mut result = Vec::new();

    for (buffer, buffer_offset_range, source_excerpt_id) in
        source_snapshot.range_to_buffer_ranges(source_bounds)
    {
        if let Some(translation) = convert_excerpt_rows(
            excerpt_map,
            source_snapshot,
            target_snapshot,
            source_excerpt_id,
            buffer,
            buffer_offset_range,
            &translate_fn,
        ) {
            result.push(translation);
        }
    }

    result
}

fn convert_excerpt_rows<F>(
    excerpt_map: &HashMap<ExcerptId, ExcerptId>,
    source_snapshot: &MultiBufferSnapshot,
    target_snapshot: &MultiBufferSnapshot,
    source_excerpt_id: ExcerptId,
    source_buffer: &text::BufferSnapshot,
    source_buffer_range: Range<BufferOffset>,
    translate_fn: F,
) -> Option<MultiBufferRowMapping>
where
    F: Fn(
        &BufferDiffSnapshot,
        Vec<Point>,
        &text::BufferSnapshot,
    ) -> (
        Vec<Range<Point>>,
        Option<Range<Point>>,
        Option<(Point, Range<Point>)>,
    ),
{
    let target_excerpt_id = excerpt_map.get(&source_excerpt_id).copied()?;
    let target_buffer = target_snapshot.buffer_for_excerpt(target_excerpt_id)?;

    let diff = source_snapshot.diff_for_buffer_id(source_buffer.remote_id())?;
    let rhs_buffer = if source_buffer.remote_id() == diff.base_text().remote_id() {
        &target_buffer
    } else {
        source_buffer
    };

    let local_start = source_buffer.offset_to_point(source_buffer_range.start.0);
    let local_end = source_buffer.offset_to_point(source_buffer_range.end.0);

    let mut input_points: Vec<Point> = (local_start.row..=local_end.row)
        .map(|row| Point::new(row, 0))
        .collect();
    if local_end.column > 0 {
        input_points.push(local_end);
    }

    let (translated_ranges, first_group, prev_boundary) =
        translate_fn(&diff, input_points.clone(), rhs_buffer);

    let source_multibuffer_range = source_snapshot.range_for_excerpt(source_excerpt_id)?;
    let source_excerpt_start_in_multibuffer = source_multibuffer_range.start;
    let source_context_range = source_snapshot.context_range_for_excerpt(source_excerpt_id)?;
    let source_excerpt_start_in_buffer = source_context_range.start.to_point(&source_buffer);
    let source_excerpt_end_in_buffer = source_context_range.end.to_point(&source_buffer);
    let target_multibuffer_range = target_snapshot.range_for_excerpt(target_excerpt_id)?;
    let target_excerpt_start_in_multibuffer = target_multibuffer_range.start;
    let target_context_range = target_snapshot.context_range_for_excerpt(target_excerpt_id)?;
    let target_excerpt_start_in_buffer = target_context_range.start.to_point(&target_buffer);
    let target_excerpt_end_in_buffer = target_context_range.end.to_point(&target_buffer);

    let boundaries: Vec<_> = input_points
        .into_iter()
        .zip(translated_ranges)
        .map(|(source_buffer_point, target_range)| {
            let source_multibuffer_point = source_excerpt_start_in_multibuffer
                + (source_buffer_point - source_excerpt_start_in_buffer.min(source_buffer_point));

            let clamped_target_start = target_range
                .start
                .max(target_excerpt_start_in_buffer)
                .min(target_excerpt_end_in_buffer);
            let clamped_target_end = target_range
                .end
                .max(target_excerpt_start_in_buffer)
                .min(target_excerpt_end_in_buffer);

            let target_multibuffer_start = target_excerpt_start_in_multibuffer
                + (clamped_target_start - target_excerpt_start_in_buffer);

            let target_multibuffer_end = target_excerpt_start_in_multibuffer
                + (clamped_target_end - target_excerpt_start_in_buffer);

            (
                source_multibuffer_point,
                target_multibuffer_start..target_multibuffer_end,
            )
        })
        .collect();
    let first_group = first_group.map(|first_group| {
        let start = source_excerpt_start_in_multibuffer
            + (first_group.start - source_excerpt_start_in_buffer.min(first_group.start));
        let end = source_excerpt_start_in_multibuffer
            + (first_group.end - source_excerpt_start_in_buffer.min(first_group.end));
        start..end
    });

    let prev_boundary = prev_boundary.map(|(source_buffer_point, target_range)| {
        let source_multibuffer_point = source_excerpt_start_in_multibuffer
            + (source_buffer_point - source_excerpt_start_in_buffer.min(source_buffer_point));

        let clamped_target_start = target_range
            .start
            .max(target_excerpt_start_in_buffer)
            .min(target_excerpt_end_in_buffer);
        let clamped_target_end = target_range
            .end
            .max(target_excerpt_start_in_buffer)
            .min(target_excerpt_end_in_buffer);

        let target_multibuffer_start = target_excerpt_start_in_multibuffer
            + (clamped_target_start - target_excerpt_start_in_buffer);
        let target_multibuffer_end = target_excerpt_start_in_multibuffer
            + (clamped_target_end - target_excerpt_start_in_buffer);

        (
            source_multibuffer_point,
            target_multibuffer_start..target_multibuffer_end,
        )
    });

    Some(MultiBufferRowMapping {
        boundaries,
        first_group,
        prev_boundary,
        source_excerpt_end: source_excerpt_start_in_multibuffer
            + (source_excerpt_end_in_buffer - source_excerpt_start_in_buffer),
        target_excerpt_end: target_excerpt_start_in_multibuffer
            + (target_excerpt_end_in_buffer - target_excerpt_start_in_buffer),
    })
}

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
pub struct ToggleSplitDiff;

#[derive(Clone, Copy, PartialEq, Eq, Action, Default)]
#[action(namespace = editor)]
struct JumpToCorrespondingRow;

/// When locked cursors mode is enabled, cursor movements in one editor will
/// update the cursor position in the other editor to the corresponding row.
#[derive(Clone, Copy, PartialEq, Eq, Action, Default)]
#[action(namespace = editor)]
pub struct ToggleLockedCursors;

pub struct SplittableEditor {
    primary_multibuffer: Entity<MultiBuffer>,
    primary_editor: Entity<Editor>,
    secondary: Option<SecondaryEditor>,
    panes: PaneGroup,
    workspace: WeakEntity<Workspace>,
    split_state: Entity<SplitEditorState>,
    locked_cursors: bool,
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

    pub fn is_split(&self) -> bool {
        self.secondary.is_some()
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
            locked_cursors: false,
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
                primary_multibuffer.set_use_extended_diff_range(true, cx);
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
            for (lhs, rhs) in secondary.update_path_excerpts_from_primary(
                path,
                &self.primary_multibuffer,
                diff.clone(),
                cx,
            ) {
                companion.add_excerpt_mapping(lhs, rhs);
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

        let this = cx.entity().downgrade();
        self.primary_editor.update(cx, |editor, _cx| {
            editor.set_scroll_companion(Some(secondary_weak));
            let this = this.clone();
            editor.set_on_local_selections_changed(Some(Box::new(
                move |cursor_position, window, cx| {
                    let this = this.clone();
                    window.defer(cx, move |window, cx| {
                        this.update(cx, |this, cx| {
                            if this.locked_cursors {
                                this.sync_cursor_to_other_side(true, cursor_position, window, cx);
                            }
                        })
                        .ok();
                    })
                },
            )));
        });
        secondary.editor.update(cx, |editor, _cx| {
            editor.set_scroll_companion(Some(primary_weak));
            let this = this.clone();
            editor.set_on_local_selections_changed(Some(Box::new(
                move |cursor_position, window, cx| {
                    let this = this.clone();
                    window.defer(cx, move |window, cx| {
                        this.update(cx, |this, cx| {
                            if this.locked_cursors {
                                this.sync_cursor_to_other_side(false, cursor_position, window, cx);
                            }
                        })
                        .ok();
                    })
                },
            )));
        });

        let primary_scroll_position = self
            .primary_editor
            .update(cx, |editor, cx| editor.scroll_position(cx));
        secondary.editor.update(cx, |editor, cx| {
            editor.set_scroll_position_internal(primary_scroll_position, false, false, window, cx);
        });

        // Copy soft wrap state from primary (source of truth) to secondary
        let primary_soft_wrap_override = self.primary_editor.read(cx).soft_wrap_mode_override;
        secondary.editor.update(cx, |editor, cx| {
            editor.soft_wrap_mode_override = primary_soft_wrap_override;
            cx.notify();
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
        if let Some(secondary) = &mut self.secondary {
            if !secondary.has_latest_selection {
                secondary.editor.read(cx).focus_handle(cx).focus(window, cx);
                secondary.editor.update(cx, |editor, cx| {
                    editor.request_autoscroll(Autoscroll::fit(), cx);
                });
                secondary.has_latest_selection = true;
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
        if let Some(secondary) = &mut self.secondary {
            if secondary.has_latest_selection {
                self.primary_editor
                    .read(cx)
                    .focus_handle(cx)
                    .focus(window, cx);
                self.primary_editor.update(cx, |editor, cx| {
                    editor.request_autoscroll(Autoscroll::fit(), cx);
                });
                secondary.has_latest_selection = false;
                cx.notify();
            } else {
                cx.propagate();
            }
        } else {
            cx.propagate();
        }
    }

    fn toggle_locked_cursors(
        &mut self,
        _: &ToggleLockedCursors,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.locked_cursors = !self.locked_cursors;
        cx.notify();
    }

    pub fn locked_cursors(&self) -> bool {
        self.locked_cursors
    }

    fn sync_cursor_to_other_side(
        &mut self,
        from_primary: bool,
        source_point: Point,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(secondary) = &self.secondary else {
            return;
        };

        let target_editor = if from_primary {
            &secondary.editor
        } else {
            &self.primary_editor
        };

        let (source_multibuffer, target_multibuffer) = if from_primary {
            (&self.primary_multibuffer, &secondary.multibuffer)
        } else {
            (&secondary.multibuffer, &self.primary_multibuffer)
        };

        let source_snapshot = source_multibuffer.read(cx).snapshot(cx);
        let target_snapshot = target_multibuffer.read(cx).snapshot(cx);

        let target_point = target_editor.update(cx, |target_editor, cx| {
            target_editor.display_map.update(cx, |display_map, cx| {
                let display_map_id = cx.entity_id();
                display_map.companion().unwrap().update(cx, |companion, _| {
                    companion
                        .convert_rows_from_companion(
                            display_map_id,
                            &target_snapshot,
                            &source_snapshot,
                            (Bound::Included(source_point), Bound::Included(source_point)),
                        )
                        .first()
                        .unwrap()
                        .boundaries
                        .first()
                        .unwrap()
                        .1
                        .start
                })
            })
        });

        target_editor.update(cx, |editor, cx| {
            editor.set_suppress_selection_callback(true);
            editor.change_selections(crate::SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([target_point..target_point]);
            });
            editor.set_suppress_selection_callback(false);
        });
    }

    fn toggle_split(&mut self, _: &ToggleSplitDiff, window: &mut Window, cx: &mut Context<Self>) {
        if self.secondary.is_some() {
            self.unsplit(&UnsplitDiff, window, cx);
        } else {
            self.split(&SplitDiff, window, cx);
        }
    }

    fn intercept_toggle_code_actions(
        &mut self,
        _: &ToggleCodeActions,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.secondary.is_some() {
            cx.stop_propagation();
        } else {
            cx.propagate();
        }
    }

    fn intercept_toggle_breakpoint(
        &mut self,
        _: &ToggleBreakpoint,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Only block breakpoint actions when the left (secondary) editor has focus
        if let Some(secondary) = &self.secondary {
            if secondary.has_latest_selection {
                cx.stop_propagation();
            } else {
                cx.propagate();
            }
        } else {
            cx.propagate();
        }
    }

    fn intercept_enable_breakpoint(
        &mut self,
        _: &EnableBreakpoint,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Only block breakpoint actions when the left (secondary) editor has focus
        if let Some(secondary) = &self.secondary {
            if secondary.has_latest_selection {
                cx.stop_propagation();
            } else {
                cx.propagate();
            }
        } else {
            cx.propagate();
        }
    }

    fn intercept_disable_breakpoint(
        &mut self,
        _: &DisableBreakpoint,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Only block breakpoint actions when the left (secondary) editor has focus
        if let Some(secondary) = &self.secondary {
            if secondary.has_latest_selection {
                cx.stop_propagation();
            } else {
                cx.propagate();
            }
        } else {
            cx.propagate();
        }
    }

    fn intercept_edit_log_breakpoint(
        &mut self,
        _: &EditLogBreakpoint,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Only block breakpoint actions when the left (secondary) editor has focus
        if let Some(secondary) = &self.secondary {
            if secondary.has_latest_selection {
                cx.stop_propagation();
            } else {
                cx.propagate();
            }
        } else {
            cx.propagate();
        }
    }

    fn intercept_inline_assist(
        &mut self,
        _: &InlineAssist,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.secondary.is_some() {
            cx.stop_propagation();
        } else {
            cx.propagate();
        }
    }

    fn toggle_soft_wrap(
        &mut self,
        _: &ToggleSoftWrap,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(secondary) = &self.secondary {
            cx.stop_propagation();

            let is_secondary_focused = secondary.has_latest_selection;
            let (focused_editor, other_editor) = if is_secondary_focused {
                (&secondary.editor, &self.primary_editor)
            } else {
                (&self.primary_editor, &secondary.editor)
            };

            // Toggle the focused editor
            focused_editor.update(cx, |editor, cx| {
                editor.toggle_soft_wrap(&ToggleSoftWrap, window, cx);
            });

            // Copy the soft wrap state from the focused editor to the other editor
            let soft_wrap_override = focused_editor.read(cx).soft_wrap_mode_override;
            other_editor.update(cx, |editor, cx| {
                editor.soft_wrap_mode_override = soft_wrap_override;
                cx.notify();
            });
        } else {
            cx.propagate();
        }
    }

    fn unsplit(&mut self, _: &UnsplitDiff, _: &mut Window, cx: &mut Context<Self>) {
        let Some(secondary) = self.secondary.take() else {
            return;
        };
        self.panes.remove(&secondary.pane, cx).unwrap();
        self.primary_editor.update(cx, |primary, cx| {
            primary.set_on_local_selections_changed(None);
            primary.set_scroll_companion(None);
            primary.set_delegate_expand_excerpts(false);
            primary.buffer().update(cx, |buffer, cx| {
                buffer.set_show_deleted_hunks(true, cx);
                buffer.set_use_extended_diff_range(false, cx);
            });
            primary.display_map.update(cx, |dm, cx| {
                dm.set_companion(None, cx);
            });
        });
        secondary.editor.update(cx, |editor, _cx| {
            editor.set_on_local_selections_changed(None);
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
        use multi_buffer::MultiBufferRow;
        use text::Bias;

        use crate::display_map::Block;
        use crate::display_map::DisplayRow;

        self.debug_print(cx);

        let secondary = self.secondary.as_ref().unwrap();
        let primary_excerpts = self.primary_multibuffer.read(cx).excerpt_ids();
        let secondary_excerpts = secondary.multibuffer.read(cx).excerpt_ids();
        assert_eq!(
            secondary_excerpts.len(),
            primary_excerpts.len(),
            "mismatch in excerpt count"
        );

        if quiesced {
            let rhs_snapshot = secondary
                .editor
                .update(cx, |editor, cx| editor.display_snapshot(cx));
            let lhs_snapshot = self
                .primary_editor
                .update(cx, |editor, cx| editor.display_snapshot(cx));

            let lhs_max_row = lhs_snapshot.max_point().row();
            let rhs_max_row = rhs_snapshot.max_point().row();
            assert_eq!(lhs_max_row, rhs_max_row, "mismatch in display row count");

            let lhs_excerpt_block_rows = lhs_snapshot
                .blocks_in_range(DisplayRow(0)..lhs_max_row + 1)
                .filter(|(_, block)| {
                    matches!(
                        block,
                        Block::BufferHeader { .. } | Block::ExcerptBoundary { .. }
                    )
                })
                .map(|(row, _)| row)
                .collect::<Vec<_>>();
            let rhs_excerpt_block_rows = rhs_snapshot
                .blocks_in_range(DisplayRow(0)..rhs_max_row + 1)
                .filter(|(_, block)| {
                    matches!(
                        block,
                        Block::BufferHeader { .. } | Block::ExcerptBoundary { .. }
                    )
                })
                .map(|(row, _)| row)
                .collect::<Vec<_>>();
            assert_eq!(lhs_excerpt_block_rows, rhs_excerpt_block_rows);

            for (lhs_hunk, rhs_hunk) in lhs_snapshot.diff_hunks().zip(rhs_snapshot.diff_hunks()) {
                assert_eq!(
                    lhs_hunk.diff_base_byte_range, rhs_hunk.diff_base_byte_range,
                    "mismatch in hunks"
                );
                assert_eq!(
                    lhs_hunk.status, rhs_hunk.status,
                    "mismatch in hunk statuses"
                );

                let (lhs_point, rhs_point) =
                    if lhs_hunk.row_range.is_empty() || rhs_hunk.row_range.is_empty() {
                        (
                            Point::new(lhs_hunk.row_range.end.0, 0),
                            Point::new(rhs_hunk.row_range.end.0, 0),
                        )
                    } else {
                        (
                            Point::new(lhs_hunk.row_range.start.0, 0),
                            Point::new(rhs_hunk.row_range.start.0, 0),
                        )
                    };
                let lhs_point = lhs_snapshot.point_to_display_point(lhs_point, Bias::Left);
                let rhs_point = rhs_snapshot.point_to_display_point(rhs_point, Bias::Left);
                assert_eq!(
                    lhs_point.row(),
                    rhs_point.row(),
                    "mismatch in hunk position"
                );
            }

            // Filtering out empty lines is a bit of a hack, to work around a case where
            // the base text has a trailing newline but the current text doesn't, or vice versa.
            // In this case, we get the additional newline on one side, but that line is not
            // marked as added/deleted by rowinfos.
            self.check_sides_match(cx, |snapshot| {
                snapshot
                    .buffer_snapshot()
                    .text()
                    .split("\n")
                    .zip(snapshot.buffer_snapshot().row_infos(MultiBufferRow(0)))
                    .filter(|(line, row_info)| !line.is_empty() && row_info.diff_status.is_none())
                    .map(|(line, _)| line.to_owned())
                    .collect::<Vec<_>>()
            });
        }
    }

    #[track_caller]
    fn check_sides_match<T: std::fmt::Debug + PartialEq>(
        &self,
        cx: &mut App,
        mut extract: impl FnMut(&crate::DisplaySnapshot) -> T,
    ) {
        let secondary = self.secondary.as_ref().expect("requires split");
        let primary_snapshot = self.primary_editor.update(cx, |editor, cx| {
            editor.display_map.update(cx, |map, cx| map.snapshot(cx))
        });
        let secondary_snapshot = secondary.editor.update(cx, |editor, cx| {
            editor.display_map.update(cx, |map, cx| map.snapshot(cx))
        });

        let primary_t = extract(&primary_snapshot);
        let secondary_t = extract(&secondary_snapshot);

        if primary_t != secondary_t {
            self.debug_print(cx);
            pretty_assertions::assert_eq!(primary_t, secondary_t);
        }
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
                    Block::Spacer {
                        id,
                        height,
                        is_below: _,
                    } => (format!("SPACER[{}]", id.0), *height),
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

        fn display_width(s: &str) -> usize {
            unicode_width::UnicodeWidthStr::width(s)
        }

        fn truncate_line(line: &str, max_width: usize) -> String {
            let line_width = display_width(line);
            if line_width <= max_width {
                return line.to_string();
            }
            if max_width < 9 {
                let mut result = String::new();
                let mut width = 0;
                for c in line.chars() {
                    let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                    if width + c_width > max_width {
                        break;
                    }
                    result.push(c);
                    width += c_width;
                }
                return result;
            }
            let ellipsis = "...";
            let target_prefix_width = 3;
            let target_suffix_width = 3;

            let mut prefix = String::new();
            let mut prefix_width = 0;
            for c in line.chars() {
                let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                if prefix_width + c_width > target_prefix_width {
                    break;
                }
                prefix.push(c);
                prefix_width += c_width;
            }

            let mut suffix_chars: Vec<char> = Vec::new();
            let mut suffix_width = 0;
            for c in line.chars().rev() {
                let c_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                if suffix_width + c_width > target_suffix_width {
                    break;
                }
                suffix_chars.push(c);
                suffix_width += c_width;
            }
            suffix_chars.reverse();
            let suffix: String = suffix_chars.into_iter().collect();

            format!("{}{}{}", prefix, ellipsis, suffix)
        }

        fn pad_to_width(s: &str, target_width: usize) -> String {
            let current_width = display_width(s);
            if current_width >= target_width {
                s.to_string()
            } else {
                format!("{}{}", s, " ".repeat(target_width - current_width))
            }
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

            let text_part = pad_to_width(&truncated_text, text_width);
            format!("{}{}{}", line_prefix, info_prefix, text_part)
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

        let max_buffers = env::var("MAX_BUFFERS")
            .map(|i| i.parse().expect("invalid `MAX_BUFFERS` variable"))
            .unwrap_or(4);

        for _ in 0..mutation_count {
            let paths = self
                .primary_multibuffer
                .read(cx)
                .paths()
                .cloned()
                .collect::<Vec<_>>();
            let excerpt_ids = self.primary_multibuffer.read(cx).excerpt_ids();

            if rng.random_bool(0.2) && !excerpt_ids.is_empty() {
                let mut excerpts = HashSet::default();
                for _ in 0..rng.random_range(0..excerpt_ids.len()) {
                    excerpts.extend(excerpt_ids.choose(rng).copied());
                }

                let line_count = rng.random_range(1..5);

                log::info!("Expanding excerpts {excerpts:?} by {line_count} lines");

                self.expand_excerpts(
                    excerpts.iter().cloned(),
                    line_count,
                    ExpandExcerptDirection::UpAndDown,
                    cx,
                );
                continue;
            }

            if excerpt_ids.is_empty() || (rng.random_bool(0.8) && paths.len() < max_buffers) {
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
                diff.update(cx, |diff, cx| {
                    diff.recalculate_diff_sync(&buffer_snapshot, cx);
                });
                let path = PathKey::for_buffer(&buffer, cx);
                let ranges = diff.update(cx, |diff, cx| {
                    diff.snapshot(cx)
                        .hunks(&buffer_snapshot)
                        .map(|hunk| hunk.buffer_range.to_point(&buffer_snapshot))
                        .collect::<Vec<_>>()
                });
                self.set_excerpts_for_path(path, buffer, ranges, 2, diff, cx);
            } else {
                log::info!("removing excerpts");
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
            SplitEditorView::new(cx.entity(), style, self.split_state.clone()).into_any_element()
        } else {
            self.primary_editor.clone().into_any_element()
        };
        div()
            .id("splittable-editor")
            .on_action(cx.listener(Self::split))
            .on_action(cx.listener(Self::unsplit))
            .on_action(cx.listener(Self::toggle_split))
            .on_action(cx.listener(Self::activate_pane_left))
            .on_action(cx.listener(Self::activate_pane_right))
            .on_action(cx.listener(Self::toggle_locked_cursors))
            .on_action(cx.listener(Self::intercept_toggle_code_actions))
            .on_action(cx.listener(Self::intercept_toggle_breakpoint))
            .on_action(cx.listener(Self::intercept_enable_breakpoint))
            .on_action(cx.listener(Self::intercept_disable_breakpoint))
            .on_action(cx.listener(Self::intercept_edit_log_breakpoint))
            .on_action(cx.listener(Self::intercept_inline_assist))
            .capture_action(cx.listener(Self::toggle_soft_wrap))
            .size_full()
            .child(inner)
    }
}

impl SecondaryEditor {
    fn update_path_excerpts_from_primary(
        &mut self,
        path_key: PathKey,
        primary_multibuffer: &Entity<MultiBuffer>,
        diff: Entity<BufferDiff>,
        cx: &mut App,
    ) -> Vec<(ExcerptId, ExcerptId)> {
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
                    let (mut translated, _, _) = diff_snapshot.points_to_base_text_points(
                        [Point::new(range.start.row, 0), Point::new(range.end.row, 0)],
                        main_buffer,
                    );
                    let start_row = translated.next().unwrap().start.row;
                    let end_row = translated.next().unwrap().end.row;
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

        secondary_excerpt_ids
            .into_iter()
            .zip(primary_excerpt_ids)
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
                for (lhs, rhs) in mappings {
                    c.add_excerpt_mapping(lhs, rhs);
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
    use buffer_diff::BufferDiff;
    use fs::FakeFs;
    use gpui::{AppContext as _, Entity, Pixels, VisualTestContext};
    use language::language_settings::SoftWrap;
    use language::{Buffer, Capability};
    use multi_buffer::{MultiBuffer, PathKey};
    use pretty_assertions::assert_eq;
    use project::Project;
    use rand::rngs::StdRng;
    use settings::SettingsStore;
    use ui::{VisualContext as _, px};
    use workspace::Workspace;

    use crate::SplittableEditor;
    use crate::test::editor_content_with_blocks_and_width;

    async fn init_test(
        cx: &mut gpui::TestAppContext,
    ) -> (Entity<SplittableEditor>, &mut VisualTestContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
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
            editor.primary_editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            });
            editor
                .secondary
                .as_ref()
                .unwrap()
                .editor
                .update(cx, |editor, cx| {
                    editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                });
            editor
        });
        (editor, cx)
    }

    fn buffer_with_diff(
        base_text: &str,
        current_text: &str,
        cx: &mut VisualTestContext,
    ) -> (Entity<Buffer>, Entity<BufferDiff>) {
        let buffer = cx.new(|cx| Buffer::local(current_text.to_string(), cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(base_text, &buffer.read(cx).text_snapshot(), cx)
        });
        (buffer, diff)
    }

    #[track_caller]
    fn assert_split_content(
        editor: &Entity<SplittableEditor>,
        expected_primary: String,
        expected_secondary: String,
        cx: &mut VisualTestContext,
    ) {
        assert_split_content_with_widths(
            editor,
            px(3000.0),
            px(3000.0),
            expected_primary,
            expected_secondary,
            cx,
        );
    }

    #[track_caller]
    fn assert_split_content_with_widths(
        editor: &Entity<SplittableEditor>,
        primary_width: Pixels,
        secondary_width: Pixels,
        expected_primary: String,
        expected_secondary: String,
        cx: &mut VisualTestContext,
    ) {
        let (primary_editor, secondary_editor) = editor.update(cx, |editor, _cx| {
            let secondary = editor
                .secondary
                .as_ref()
                .expect("should have secondary editor");
            (editor.primary_editor.clone(), secondary.editor.clone())
        });

        // Make sure both sides learn if the other has soft-wrapped
        let _ = editor_content_with_blocks_and_width(&primary_editor, primary_width, cx);
        cx.run_until_parked();
        let _ = editor_content_with_blocks_and_width(&secondary_editor, secondary_width, cx);
        cx.run_until_parked();

        let primary_content =
            editor_content_with_blocks_and_width(&primary_editor, primary_width, cx);
        let secondary_content =
            editor_content_with_blocks_and_width(&secondary_editor, secondary_width, cx);

        if primary_content != expected_primary || secondary_content != expected_secondary {
            editor.update(cx, |editor, cx| editor.debug_print(cx));
        }

        assert_eq!(primary_content, expected_primary, "rhs");
        assert_eq!(secondary_content, expected_secondary, "lhs");
    }

    #[gpui::test(iterations = 100)]
    async fn test_random_split_editor(mut rng: StdRng, cx: &mut gpui::TestAppContext) {
        use rand::prelude::*;

        let (editor, cx) = init_test(cx).await;
        let operations = std::env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
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
                log::info!("adding excerpts to empty multibuffer");
                editor.update(cx, |editor, cx| {
                    editor.randomly_edit_excerpts(rng, 2, cx);
                    editor.check_invariants(true, cx);
                });
                continue;
            }

            let mut quiesced = false;

            match rng.random_range(0..100) {
                0..=44 => {
                    log::info!("randomly editing multibuffer");
                    editor.update(cx, |editor, cx| {
                        editor.primary_multibuffer.update(cx, |multibuffer, cx| {
                            multibuffer.randomly_edit(rng, 5, cx);
                        })
                    })
                }
                45..=64 => {
                    log::info!("randomly undoing/redoing in single buffer");
                    let buffer = buffers.iter().choose(rng).unwrap();
                    buffer.update(cx, |buffer, cx| {
                        buffer.randomly_undo_redo(rng, cx);
                    });
                }
                65..=79 => {
                    log::info!("mutating excerpts");
                    editor.update(cx, |editor, cx| {
                        editor.randomly_edit_excerpts(rng, 2, cx);
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
                    }
                    quiesced = true;
                }
            }

            editor.update(cx, |editor, cx| {
                editor.check_invariants(quiesced, cx);
            });
        }
    }

    #[gpui::test]
    async fn test_basic_alignment(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

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

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee
            fff"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent(),
            &mut cx,
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(3, 0)..Point::new(3, 3), "FFF")], None, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee
            FFF"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent(),
            &mut cx,
        );

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee
            FFF"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee
            fff"
            .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_deleting_unmodified_lines(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let base_text1 = "
            aaa
            bbb
            ccc
            ddd
            eee"
        .unindent();

        let base_text2 = "
            fff
            ggg
            hhh
            iii
            jjj"
        .unindent();

        let (buffer1, diff1) = buffer_with_diff(&base_text1, &base_text1, &mut cx);
        let (buffer2, diff2) = buffer_with_diff(&base_text2, &base_text2, &mut cx);

        editor.update(cx, |editor, cx| {
            let path1 = PathKey::for_buffer(&buffer1, cx);
            editor.set_excerpts_for_path(
                path1,
                buffer1.clone(),
                vec![Point::new(0, 0)..buffer1.read(cx).max_point()],
                0,
                diff1.clone(),
                cx,
            );
            let path2 = PathKey::for_buffer(&buffer2, cx);
            editor.set_excerpts_for_path(
                path2,
                buffer2.clone(),
                vec![Point::new(0, 0)..buffer2.read(cx).max_point()],
                1,
                diff2.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        buffer1.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    (Point::new(0, 0)..Point::new(1, 0), ""),
                    (Point::new(3, 0)..Point::new(4, 0), ""),
                ],
                None,
                cx,
            );
        });
        buffer2.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    (Point::new(0, 0)..Point::new(1, 0), ""),
                    (Point::new(3, 0)..Point::new(4, 0), ""),
                ],
                None,
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            § spacer
            bbb
            ccc
            § spacer
            eee
            § <no file>
            § -----
            § spacer
            ggg
            hhh
            § spacer
            jjj"
            .unindent(),
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
            fff
            ggg
            hhh
            iii
            jjj"
            .unindent(),
            &mut cx,
        );

        let buffer1_snapshot = buffer1.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff1.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer1_snapshot, cx);
        });
        let buffer2_snapshot = buffer2.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff2.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer2_snapshot, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            § spacer
            bbb
            ccc
            § spacer
            eee
            § <no file>
            § -----
            § spacer
            ggg
            hhh
            § spacer
            jjj"
            .unindent(),
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
            fff
            ggg
            hhh
            iii
            jjj"
            .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_deleting_added_line(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let base_text = "
            aaa
            bbb
            ccc
            ddd
        "
        .unindent();

        let current_text = "
            aaa
            NEW1
            NEW2
            ccc
            ddd
        "
        .unindent();

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            NEW1
            NEW2
            ccc
            ddd"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            § spacer
            ccc
            ddd"
            .unindent(),
            &mut cx,
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(3, 0), "")], None, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            NEW1
            ccc
            ddd"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd"
            .unindent(),
            &mut cx,
        );

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            NEW1
            ccc
            ddd"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd"
            .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_inserting_consecutive_blank_line(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let base_text = "
            aaa
            bbb





            ccc
            ddd
        "
        .unindent();
        let current_text = "
            aaa
            bbb





            CCC
            ddd
        "
        .unindent();

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 3)..Point::new(1, 3), "\n")], None, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            bbb






            CCC
            ddd"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            § spacer





            ccc
            ddd"
            .unindent(),
            &mut cx,
        );

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            bbb






            CCC
            ddd"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb





            ccc
            § spacer
            ddd"
            .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_reverting_deletion_hunk(cx: &mut gpui::TestAppContext) {
        use git::Restore;
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

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

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            ddd
            eee"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent(),
            &mut cx,
        );

        let primary_editor = editor.update(cx, |editor, _cx| editor.primary_editor.clone());
        cx.update_window_entity(&primary_editor, |editor, window, cx| {
            editor.change_selections(crate::SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([Point::new(1, 0)..Point::new(1, 0)]);
            });
            editor.git_restore(&Restore, window, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent(),
            &mut cx,
        );

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_deleting_added_lines(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

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

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    (Point::new(2, 0)..Point::new(3, 0), ""),
                    (Point::new(4, 0)..Point::new(5, 0), ""),
                ],
                None,
                cx,
            );
        });
        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            new1
            new3
            § spacer
            § spacer
            zzz"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            old1
            old2
            old3
            old4
            zzz"
            .unindent(),
            &mut cx,
        );

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            new1
            new3
            § spacer
            § spacer
            zzz"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            old1
            old2
            old3
            old4
            zzz"
            .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_soft_wrap_at_end_of_excerpt(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let text = "aaaa bbbb cccc dddd eeee ffff";

        let (buffer1, diff1) = buffer_with_diff(text, text, &mut cx);
        let (buffer2, diff2) = buffer_with_diff(text, text, &mut cx);

        editor.update(cx, |editor, cx| {
            let end = Point::new(0, text.len() as u32);
            let path1 = PathKey::for_buffer(&buffer1, cx);
            editor.set_excerpts_for_path(
                path1,
                buffer1.clone(),
                vec![Point::new(0, 0)..end],
                0,
                diff1.clone(),
                cx,
            );
            let path2 = PathKey::for_buffer(&buffer2, cx);
            editor.set_excerpts_for_path(
                path2,
                buffer2.clone(),
                vec![Point::new(0, 0)..end],
                0,
                diff2.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content_with_widths(
            &editor,
            px(200.0),
            px(400.0),
            "
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff"
                .unindent(),
            "
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer"
                .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_soft_wrap_before_modification_hunk(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let base_text = "
            aaaa bbbb cccc dddd eeee ffff
            old line one
            old line two
        "
        .unindent();

        let current_text = "
            aaaa bbbb cccc dddd eeee ffff
            new line
        "
        .unindent();

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content_with_widths(
            &editor,
            px(200.0),
            px(400.0),
            "
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff
            new line
            § spacer"
                .unindent(),
            "
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer
            old line one
            old line two"
                .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_soft_wrap_before_deletion_hunk(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let base_text = "
            aaaa bbbb cccc dddd eeee ffff
            deleted line one
            deleted line two
            after
        "
        .unindent();

        let current_text = "
            aaaa bbbb cccc dddd eeee ffff
            after
        "
        .unindent();

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content_with_widths(
            &editor,
            px(400.0),
            px(200.0),
            "
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer
            § spacer
            § spacer
            § spacer
            § spacer
            after"
                .unindent(),
            "
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff
            deleted\x20
            line one
            deleted\x20
            line two
            after"
                .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_soft_wrap_spacer_after_editing_second_line(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let text = "
            aaaa bbbb cccc dddd eeee ffff
            short
        "
        .unindent();

        let (buffer, diff) = buffer_with_diff(&text, &text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content_with_widths(
            &editor,
            px(400.0),
            px(200.0),
            "
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer
            short"
                .unindent(),
            "
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff
            short"
                .unindent(),
            &mut cx,
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 0)..Point::new(1, 5), "modified")], None, cx);
        });

        cx.run_until_parked();

        assert_split_content_with_widths(
            &editor,
            px(400.0),
            px(200.0),
            "
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer
            modified"
                .unindent(),
            "
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff
            short"
                .unindent(),
            &mut cx,
        );

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        assert_split_content_with_widths(
            &editor,
            px(400.0),
            px(200.0),
            "
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer
            modified"
                .unindent(),
            "
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff
            short"
                .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_no_base_text(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let (buffer1, diff1) = buffer_with_diff("xxx\nyyy", "xxx\nyyy", &mut cx);

        let current_text = "
            aaa
            bbb
            ccc
        "
        .unindent();

        let buffer2 = cx.new(|cx| Buffer::local(current_text.to_string(), cx));
        let diff2 = cx.new(|cx| BufferDiff::new(&buffer2.read(cx).text_snapshot(), cx));

        editor.update(cx, |editor, cx| {
            let path1 = PathKey::for_buffer(&buffer1, cx);
            editor.set_excerpts_for_path(
                path1,
                buffer1.clone(),
                vec![Point::new(0, 0)..buffer1.read(cx).max_point()],
                0,
                diff1.clone(),
                cx,
            );

            let path2 = PathKey::for_buffer(&buffer2, cx);
            editor.set_excerpts_for_path(
                path2,
                buffer2.clone(),
                vec![Point::new(0, 0)..buffer2.read(cx).max_point()],
                1,
                diff2.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            xxx
            yyy
            § <no file>
            § -----
            aaa
            bbb
            ccc"
            .unindent(),
            "
            § <no file>
            § -----
            xxx
            yyy
            § <no file>
            § -----
            § spacer
            § spacer
            § spacer"
                .unindent(),
            &mut cx,
        );

        buffer1.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 3)..Point::new(0, 3), "z")], None, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            xxxz
            yyy
            § <no file>
            § -----
            aaa
            bbb
            ccc"
            .unindent(),
            "
            § <no file>
            § -----
            xxx
            yyy
            § <no file>
            § -----
            § spacer
            § spacer
            § spacer"
                .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_deleting_char_in_added_line(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let base_text = "
            aaa
            bbb
            ccc
        "
        .unindent();

        let current_text = "
            NEW1
            NEW2
            ccc
        "
        .unindent();

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            NEW1
            NEW2
            ccc"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc"
            .unindent(),
            &mut cx,
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 3)..Point::new(1, 4), "")], None, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            NEW1
            NEW
            ccc"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc"
            .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_soft_wrap_spacer_before_added_line(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

        let base_text = "aaaa bbbb cccc dddd eeee ffff\n";

        let current_text = "
            aaaa bbbb cccc dddd eeee ffff
            added line
        "
        .unindent();

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content_with_widths(
            &editor,
            px(400.0),
            px(200.0),
            "
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer
            added line"
                .unindent(),
            "
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff
            § spacer"
                .unindent(),
            &mut cx,
        );

        assert_split_content_with_widths(
            &editor,
            px(200.0),
            px(400.0),
            "
            § <no file>
            § -----
            aaaa bbbb\x20
            cccc dddd\x20
            eeee ffff
            added line"
                .unindent(),
            "
            § <no file>
            § -----
            aaaa bbbb cccc dddd eeee ffff
            § spacer
            § spacer
            § spacer"
                .unindent(),
            &mut cx,
        );
    }

    #[gpui::test]
    #[ignore]
    async fn test_joining_added_line_with_unmodified_line(cx: &mut gpui::TestAppContext) {
        use rope::Point;
        use unindent::Unindent as _;

        let (editor, mut cx) = init_test(cx).await;

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
            NEW
            eee
        "
        .unindent();

        let (buffer, diff) = buffer_with_diff(&base_text, &current_text, &mut cx);

        editor.update(cx, |editor, cx| {
            let path = PathKey::for_buffer(&buffer, cx);
            editor.set_excerpts_for_path(
                path,
                buffer.clone(),
                vec![Point::new(0, 0)..buffer.read(cx).max_point()],
                0,
                diff.clone(),
                cx,
            );
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            NEW
            § spacer
            § spacer
            eee"
            .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent(),
            &mut cx,
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 3)..Point::new(2, 0), "")], None, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            § spacer
            § spacer
            § spacer
            NEWeee"
                .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent(),
            &mut cx,
        );

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx);
        });

        cx.run_until_parked();

        assert_split_content(
            &editor,
            "
            § <no file>
            § -----
            aaa
            NEWeee
            § spacer
            § spacer
            § spacer"
                .unindent(),
            "
            § <no file>
            § -----
            aaa
            bbb
            ccc
            ddd
            eee"
            .unindent(),
            &mut cx,
        );
    }
}
