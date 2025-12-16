use crate::{Keep, KeepAll, OpenAgentDiff, Reject, RejectAll};
use acp_thread::{AcpThread, AcpThreadEvent};
use action_log::ActionLogTelemetry;
use agent_settings::AgentSettings;
use anyhow::Result;
use buffer_diff::DiffHunkStatus;
use collections::{HashMap, HashSet};
use editor::{
    Direction, Editor, EditorEvent, EditorSettings, MultiBuffer, MultiBufferSnapshot,
    SelectionEffects, ToPoint,
    actions::{GoToHunk, GoToPreviousHunk},
    multibuffer_context_lines,
    scroll::Autoscroll,
};
use gpui::{
    Action, AnyElement, App, AppContext, Empty, Entity, EventEmitter, FocusHandle, Focusable,
    Global, SharedString, Subscription, Task, WeakEntity, Window, prelude::*,
};

use language::{Buffer, Capability, DiskState, OffsetRangeExt, Point};
use multi_buffer::PathKey;
use project::{Project, ProjectItem, ProjectPath};
use settings::{Settings, SettingsStore};
use std::{
    any::{Any, TypeId},
    collections::hash_map::Entry,
    ops::Range,
    sync::Arc,
};
use ui::{CommonAnimationExt, IconButtonShape, KeyBinding, Tooltip, prelude::*, vertical_divider};
use util::ResultExt;
use workspace::{
    Item, ItemHandle, ItemNavHistory, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace,
    item::{BreadcrumbText, ItemEvent, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};
use zed_actions::assistant::ToggleFocus;

pub struct AgentDiffPane {
    multibuffer: Entity<MultiBuffer>,
    editor: Entity<Editor>,
    thread: Entity<AcpThread>,
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    title: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl AgentDiffPane {
    pub fn deploy(
        thread: Entity<AcpThread>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Entity<Self>> {
        workspace.update(cx, |workspace, cx| {
            Self::deploy_in_workspace(thread, workspace, window, cx)
        })
    }

    pub fn deploy_in_workspace(
        thread: Entity<AcpThread>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let existing_diff = workspace
            .items_of_type::<AgentDiffPane>(cx)
            .find(|diff| diff.read(cx).thread == thread);

        if let Some(existing_diff) = existing_diff {
            workspace.activate_item(&existing_diff, true, true, window, cx);
            existing_diff
        } else {
            let agent_diff = cx
                .new(|cx| AgentDiffPane::new(thread.clone(), workspace.weak_handle(), window, cx));
            workspace.add_item_to_center(Box::new(agent_diff.clone()), window, cx);
            agent_diff
        }
    }

    pub fn new(
        thread: Entity<AcpThread>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));

        let project = thread.read(cx).project().clone();
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.disable_inline_diagnostics();
            editor.set_expand_all_diff_hunks(cx);
            editor.set_render_diff_hunk_controls(diff_hunk_controls(&thread), cx);
            editor.register_addon(AgentDiffAddon);
            editor
        });

        let action_log = thread.read(cx).action_log().clone();

        let mut this = Self {
            _subscriptions: vec![
                cx.observe_in(&action_log, window, |this, _action_log, window, cx| {
                    this.update_excerpts(window, cx)
                }),
                cx.subscribe(&thread, |this, _thread, event, cx| {
                    this.handle_acp_thread_event(event, cx)
                }),
            ],
            title: SharedString::default(),
            multibuffer,
            editor,
            thread,
            focus_handle,
            workspace,
        };
        this.update_excerpts(window, cx);
        this.update_title(cx);
        this
    }

    fn update_excerpts(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let changed_buffers = self
            .thread
            .read(cx)
            .action_log()
            .read(cx)
            .changed_buffers(cx);
        let mut paths_to_delete = self
            .multibuffer
            .read(cx)
            .paths()
            .cloned()
            .collect::<HashSet<_>>();

        for (buffer, diff_handle) in changed_buffers {
            if buffer.read(cx).file().is_none() {
                continue;
            }

            let path_key = PathKey::for_buffer(&buffer, cx);
            paths_to_delete.remove(&path_key);

            let snapshot = buffer.read(cx).snapshot();
            let diff = diff_handle.read(cx);

            let diff_hunk_ranges = diff
                .hunks_intersecting_range(
                    language::Anchor::min_max_range_for_buffer(snapshot.remote_id()),
                    &snapshot,
                    cx,
                )
                .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot))
                .collect::<Vec<_>>();

            let (was_empty, is_excerpt_newly_added) =
                self.multibuffer.update(cx, |multibuffer, cx| {
                    let was_empty = multibuffer.is_empty();
                    let (_, is_excerpt_newly_added) = multibuffer.set_excerpts_for_path(
                        path_key.clone(),
                        buffer.clone(),
                        diff_hunk_ranges,
                        multibuffer_context_lines(cx),
                        cx,
                    );
                    multibuffer.add_diff(diff_handle, cx);
                    (was_empty, is_excerpt_newly_added)
                });

            self.editor.update(cx, |editor, cx| {
                if was_empty {
                    let first_hunk = editor
                        .diff_hunks_in_ranges(
                            &[editor::Anchor::min()..editor::Anchor::max()],
                            &self.multibuffer.read(cx).read(cx),
                        )
                        .next();

                    if let Some(first_hunk) = first_hunk {
                        let first_hunk_start = first_hunk.multi_buffer_range().start;
                        editor.change_selections(Default::default(), window, cx, |selections| {
                            selections.select_anchor_ranges([first_hunk_start..first_hunk_start]);
                        })
                    }
                }

                if is_excerpt_newly_added
                    && buffer
                        .read(cx)
                        .file()
                        .is_some_and(|file| file.disk_state() == DiskState::Deleted)
                {
                    editor.fold_buffer(snapshot.text.remote_id(), cx)
                }
            });
        }

        self.multibuffer.update(cx, |multibuffer, cx| {
            for path in paths_to_delete {
                multibuffer.remove_excerpts_for_path(path, cx);
            }
        });

        if self.multibuffer.read(cx).is_empty()
            && self
                .editor
                .read(cx)
                .focus_handle(cx)
                .contains_focused(window, cx)
        {
            self.focus_handle.focus(window);
        } else if self.focus_handle.is_focused(window) && !self.multibuffer.read(cx).is_empty() {
            self.editor.update(cx, |editor, cx| {
                editor.focus_handle(cx).focus(window);
            });
        }
    }

    fn update_title(&mut self, cx: &mut Context<Self>) {
        let new_title = self.thread.read(cx).title();
        if new_title != self.title {
            self.title = new_title;
            cx.emit(EditorEvent::TitleChanged);
        }
    }

    fn handle_acp_thread_event(&mut self, event: &AcpThreadEvent, cx: &mut Context<Self>) {
        if let AcpThreadEvent::TitleUpdated = event {
            self.update_title(cx)
        }
    }

    pub fn move_to_path(&self, path_key: PathKey, window: &mut Window, cx: &mut App) {
        if let Some(position) = self.multibuffer.read(cx).location_for_path(&path_key, cx) {
            self.editor.update(cx, |editor, cx| {
                let first_hunk = editor
                    .diff_hunks_in_ranges(
                        &[position..editor::Anchor::max()],
                        &self.multibuffer.read(cx).read(cx),
                    )
                    .next();

                if let Some(first_hunk) = first_hunk {
                    let first_hunk_start = first_hunk.multi_buffer_range().start;
                    editor.change_selections(Default::default(), window, cx, |selections| {
                        selections.select_anchor_ranges([first_hunk_start..first_hunk_start]);
                    })
                }
            });
        }
    }

    fn keep(&mut self, _: &Keep, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            keep_edits_in_selection(editor, &snapshot, &self.thread, window, cx);
        });
    }

    fn reject(&mut self, _: &Reject, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            reject_edits_in_selection(editor, &snapshot, &self.thread, window, cx);
        });
    }

    fn reject_all(&mut self, _: &RejectAll, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            reject_edits_in_ranges(
                editor,
                &snapshot,
                &self.thread,
                vec![editor::Anchor::min()..editor::Anchor::max()],
                window,
                cx,
            );
        });
    }

    fn keep_all(&mut self, _: &KeepAll, _window: &mut Window, cx: &mut Context<Self>) {
        let telemetry = ActionLogTelemetry::from(self.thread.read(cx));
        let action_log = self.thread.read(cx).action_log().clone();
        action_log.update(cx, |action_log, cx| {
            action_log.keep_all_edits(Some(telemetry), cx)
        });
    }
}

fn keep_edits_in_selection(
    editor: &mut Editor,
    buffer_snapshot: &MultiBufferSnapshot,
    thread: &Entity<AcpThread>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let ranges = editor
        .selections
        .disjoint_anchor_ranges()
        .collect::<Vec<_>>();

    keep_edits_in_ranges(editor, buffer_snapshot, thread, ranges, window, cx)
}

fn reject_edits_in_selection(
    editor: &mut Editor,
    buffer_snapshot: &MultiBufferSnapshot,
    thread: &Entity<AcpThread>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let ranges = editor
        .selections
        .disjoint_anchor_ranges()
        .collect::<Vec<_>>();
    reject_edits_in_ranges(editor, buffer_snapshot, thread, ranges, window, cx)
}

fn keep_edits_in_ranges(
    editor: &mut Editor,
    buffer_snapshot: &MultiBufferSnapshot,
    thread: &Entity<AcpThread>,
    ranges: Vec<Range<editor::Anchor>>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let diff_hunks_in_ranges = editor
        .diff_hunks_in_ranges(&ranges, buffer_snapshot)
        .collect::<Vec<_>>();

    update_editor_selection(editor, buffer_snapshot, &diff_hunks_in_ranges, window, cx);

    let multibuffer = editor.buffer().clone();
    for hunk in &diff_hunks_in_ranges {
        let buffer = multibuffer.read(cx).buffer(hunk.buffer_id);
        if let Some(buffer) = buffer {
            let action_log = thread.read(cx).action_log().clone();
            let telemetry = ActionLogTelemetry::from(thread.read(cx));
            action_log.update(cx, |action_log, cx| {
                action_log.keep_edits_in_range(
                    buffer,
                    hunk.buffer_range.clone(),
                    Some(telemetry),
                    cx,
                )
            });
        }
    }
}

fn reject_edits_in_ranges(
    editor: &mut Editor,
    buffer_snapshot: &MultiBufferSnapshot,
    thread: &Entity<AcpThread>,
    ranges: Vec<Range<editor::Anchor>>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let diff_hunks_in_ranges = editor
        .diff_hunks_in_ranges(&ranges, buffer_snapshot)
        .collect::<Vec<_>>();

    update_editor_selection(editor, buffer_snapshot, &diff_hunks_in_ranges, window, cx);

    let multibuffer = editor.buffer().clone();

    let mut ranges_by_buffer = HashMap::default();
    for hunk in &diff_hunks_in_ranges {
        let buffer = multibuffer.read(cx).buffer(hunk.buffer_id);
        if let Some(buffer) = buffer {
            ranges_by_buffer
                .entry(buffer.clone())
                .or_insert_with(Vec::new)
                .push(hunk.buffer_range.clone());
        }
    }

    let action_log = thread.read(cx).action_log().clone();
    let telemetry = ActionLogTelemetry::from(thread.read(cx));
    for (buffer, ranges) in ranges_by_buffer {
        action_log
            .update(cx, |action_log, cx| {
                action_log.reject_edits_in_ranges(buffer, ranges, Some(telemetry.clone()), cx)
            })
            .detach_and_log_err(cx);
    }
}

fn update_editor_selection(
    editor: &mut Editor,
    buffer_snapshot: &MultiBufferSnapshot,
    diff_hunks: &[multi_buffer::MultiBufferDiffHunk],
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let newest_cursor = editor
        .selections
        .newest::<Point>(&editor.display_snapshot(cx))
        .head();

    if !diff_hunks.iter().any(|hunk| {
        hunk.row_range
            .contains(&multi_buffer::MultiBufferRow(newest_cursor.row))
    }) {
        return;
    }

    let target_hunk = {
        diff_hunks
            .last()
            .and_then(|last_kept_hunk| {
                let last_kept_hunk_end = last_kept_hunk.multi_buffer_range().end;
                editor
                    .diff_hunks_in_ranges(
                        &[last_kept_hunk_end..editor::Anchor::max()],
                        buffer_snapshot,
                    )
                    .nth(1)
            })
            .or_else(|| {
                let first_kept_hunk = diff_hunks.first()?;
                let first_kept_hunk_start = first_kept_hunk.multi_buffer_range().start;
                editor
                    .diff_hunks_in_ranges(
                        &[editor::Anchor::min()..first_kept_hunk_start],
                        buffer_snapshot,
                    )
                    .next()
            })
    };

    if let Some(target_hunk) = target_hunk {
        editor.change_selections(Default::default(), window, cx, |selections| {
            let next_hunk_start = target_hunk.multi_buffer_range().start;
            selections.select_anchor_ranges([next_hunk_start..next_hunk_start]);
        })
    }
}

impl EventEmitter<EditorEvent> for AgentDiffPane {}

impl Focusable for AgentDiffPane {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.multibuffer.read(cx).is_empty() {
            self.focus_handle.clone()
        } else {
            self.editor.focus_handle(cx)
        }
    }
}

impl Item for AgentDiffPane {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::ZedAssistant).color(Color::Muted))
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn navigate(
        &mut self,
        data: Box<dyn Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some("Agent Diff".into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        let title = self.thread.read(cx).title();
        Label::new(format!("Review: {}", title))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Assistant Diff Opened")
    }

    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| {
            Self::new(self.thread.clone(), self.workspace.clone(), window, cx)
        })))
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).has_conflict(cx)
    }

    fn can_save(&self, _: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(options, project, window, cx)
    }

    fn save_as(
        &mut self,
        _: Entity<Project>,
        _: ProjectPath,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unreachable!()
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.reload(project, window, cx)
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.clone().into())
        } else {
            None
        }
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Agent Diff".into()
    }
}

impl Render for AgentDiffPane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.multibuffer.read(cx).is_empty();
        let focus_handle = &self.focus_handle;

        div()
            .track_focus(focus_handle)
            .key_context(if is_empty { "EmptyPane" } else { "AgentDiff" })
            .on_action(cx.listener(Self::keep))
            .on_action(cx.listener(Self::reject))
            .on_action(cx.listener(Self::reject_all))
            .on_action(cx.listener(Self::keep_all))
            .bg(cx.theme().colors().editor_background)
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .when(is_empty, |el| {
                el.child(
                    v_flex()
                        .items_center()
                        .gap_2()
                        .child("No changes to review")
                        .child(
                            Button::new("continue-iterating", "Continue Iterating")
                                .style(ButtonStyle::Filled)
                                .icon(IconName::ForwardArrow)
                                .icon_position(IconPosition::Start)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .full_width()
                                .key_binding(KeyBinding::for_action_in(
                                    &ToggleFocus,
                                    &focus_handle.clone(),
                                    cx,
                                ))
                                .on_click(|_event, window, cx| {
                                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                                }),
                        ),
                )
            })
            .when(!is_empty, |el| el.child(self.editor.clone()))
    }
}

fn diff_hunk_controls(thread: &Entity<AcpThread>) -> editor::RenderDiffHunkControlsFn {
    let thread = thread.clone();

    Arc::new(
        move |row, status, hunk_range, is_created_file, line_height, editor, _, cx| {
            {
                render_diff_hunk_controls(
                    row,
                    status,
                    hunk_range,
                    is_created_file,
                    line_height,
                    &thread,
                    editor,
                    cx,
                )
            }
        },
    )
}

fn render_diff_hunk_controls(
    row: u32,
    _status: &DiffHunkStatus,
    hunk_range: Range<editor::Anchor>,
    is_created_file: bool,
    line_height: Pixels,
    thread: &Entity<AcpThread>,
    editor: &Entity<Editor>,
    cx: &mut App,
) -> AnyElement {
    let editor = editor.clone();

    h_flex()
        .h(line_height)
        .mr_0p5()
        .gap_1()
        .px_0p5()
        .pb_1()
        .border_x_1()
        .border_b_1()
        .border_color(cx.theme().colors().border)
        .rounded_b_md()
        .bg(cx.theme().colors().editor_background)
        .gap_1()
        .block_mouse_except_scroll()
        .shadow_md()
        .children(vec![
            Button::new(("reject", row as u64), "Reject")
                .disabled(is_created_file)
                .key_binding(
                    KeyBinding::for_action_in(&Reject, &editor.read(cx).focus_handle(cx), cx)
                        .map(|kb| kb.size(rems_from_px(12.))),
                )
                .on_click({
                    let editor = editor.clone();
                    let thread = thread.clone();
                    move |_event, window, cx| {
                        editor.update(cx, |editor, cx| {
                            let snapshot = editor.buffer().read(cx).snapshot(cx);
                            reject_edits_in_ranges(
                                editor,
                                &snapshot,
                                &thread,
                                vec![hunk_range.start..hunk_range.start],
                                window,
                                cx,
                            );
                        })
                    }
                }),
            Button::new(("keep", row as u64), "Keep")
                .key_binding(
                    KeyBinding::for_action_in(&Keep, &editor.read(cx).focus_handle(cx), cx)
                        .map(|kb| kb.size(rems_from_px(12.))),
                )
                .on_click({
                    let editor = editor.clone();
                    let thread = thread.clone();
                    move |_event, window, cx| {
                        editor.update(cx, |editor, cx| {
                            let snapshot = editor.buffer().read(cx).snapshot(cx);
                            keep_edits_in_ranges(
                                editor,
                                &snapshot,
                                &thread,
                                vec![hunk_range.start..hunk_range.start],
                                window,
                                cx,
                            );
                        });
                    }
                }),
        ])
        .when(
            !editor.read(cx).buffer().read(cx).all_diff_hunks_expanded(),
            |el| {
                el.child(
                    IconButton::new(("next-hunk", row as u64), IconName::ArrowDown)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        // .disabled(!has_multiple_hunks)
                        .tooltip({
                            let focus_handle = editor.focus_handle(cx);
                            move |_window, cx| {
                                Tooltip::for_action_in("Next Hunk", &GoToHunk, &focus_handle, cx)
                            }
                        })
                        .on_click({
                            let editor = editor.clone();
                            move |_event, window, cx| {
                                editor.update(cx, |editor, cx| {
                                    let snapshot = editor.snapshot(window, cx);
                                    let position =
                                        hunk_range.end.to_point(&snapshot.buffer_snapshot());
                                    editor.go_to_hunk_before_or_after_position(
                                        &snapshot,
                                        position,
                                        Direction::Next,
                                        window,
                                        cx,
                                    );
                                    editor.expand_selected_diff_hunks(cx);
                                });
                            }
                        }),
                )
                .child(
                    IconButton::new(("prev-hunk", row as u64), IconName::ArrowUp)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        // .disabled(!has_multiple_hunks)
                        .tooltip({
                            let focus_handle = editor.focus_handle(cx);
                            move |_window, cx| {
                                Tooltip::for_action_in(
                                    "Previous Hunk",
                                    &GoToPreviousHunk,
                                    &focus_handle,
                                    cx,
                                )
                            }
                        })
                        .on_click({
                            let editor = editor.clone();
                            move |_event, window, cx| {
                                editor.update(cx, |editor, cx| {
                                    let snapshot = editor.snapshot(window, cx);
                                    let point =
                                        hunk_range.start.to_point(&snapshot.buffer_snapshot());
                                    editor.go_to_hunk_before_or_after_position(
                                        &snapshot,
                                        point,
                                        Direction::Prev,
                                        window,
                                        cx,
                                    );
                                    editor.expand_selected_diff_hunks(cx);
                                });
                            }
                        }),
                )
            },
        )
        .into_any_element()
}

struct AgentDiffAddon;

impl editor::Addon for AgentDiffAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn extend_key_context(&self, key_context: &mut gpui::KeyContext, _: &App) {
        key_context.add("agent_diff");
    }
}

pub struct AgentDiffToolbar {
    active_item: Option<AgentDiffToolbarItem>,
    _settings_subscription: Subscription,
}

pub enum AgentDiffToolbarItem {
    Pane(WeakEntity<AgentDiffPane>),
    Editor {
        editor: WeakEntity<Editor>,
        state: EditorState,
        _diff_subscription: Subscription,
    },
}

impl AgentDiffToolbar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            active_item: None,
            _settings_subscription: cx.observe_global::<SettingsStore>(Self::update_location),
        }
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_item) = self.active_item.as_ref() else {
            return;
        };

        match active_item {
            AgentDiffToolbarItem::Pane(agent_diff) => {
                if let Some(agent_diff) = agent_diff.upgrade() {
                    agent_diff.focus_handle(cx).focus(window);
                }
            }
            AgentDiffToolbarItem::Editor { editor, .. } => {
                if let Some(editor) = editor.upgrade() {
                    editor.read(cx).focus_handle(cx).focus(window);
                }
            }
        }

        let action = action.boxed_clone();
        cx.defer(move |cx| {
            cx.dispatch_action(action.as_ref());
        })
    }

    fn handle_diff_notify(&mut self, agent_diff: Entity<AgentDiff>, cx: &mut Context<Self>) {
        let Some(AgentDiffToolbarItem::Editor { editor, state, .. }) = self.active_item.as_mut()
        else {
            return;
        };

        *state = agent_diff.read(cx).editor_state(editor);
        self.update_location(cx);
        cx.notify();
    }

    fn update_location(&mut self, cx: &mut Context<Self>) {
        let location = self.location(cx);
        cx.emit(ToolbarItemEvent::ChangeLocation(location));
    }

    fn location(&self, cx: &App) -> ToolbarItemLocation {
        if !EditorSettings::get_global(cx).toolbar.agent_review {
            return ToolbarItemLocation::Hidden;
        }

        match &self.active_item {
            None => ToolbarItemLocation::Hidden,
            Some(AgentDiffToolbarItem::Pane(_)) => ToolbarItemLocation::PrimaryRight,
            Some(AgentDiffToolbarItem::Editor { state, .. }) => match state {
                EditorState::Reviewing => ToolbarItemLocation::PrimaryRight,
                EditorState::Idle => ToolbarItemLocation::Hidden,
            },
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for AgentDiffToolbar {}

impl ToolbarItemView for AgentDiffToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(item) = active_pane_item {
            if let Some(pane) = item.act_as::<AgentDiffPane>(cx) {
                self.active_item = Some(AgentDiffToolbarItem::Pane(pane.downgrade()));
                return self.location(cx);
            }

            if let Some(editor) = item.act_as::<Editor>(cx)
                && editor.read(cx).mode().is_full()
            {
                let agent_diff = AgentDiff::global(cx);

                self.active_item = Some(AgentDiffToolbarItem::Editor {
                    editor: editor.downgrade(),
                    state: agent_diff.read(cx).editor_state(&editor.downgrade()),
                    _diff_subscription: cx.observe(&agent_diff, Self::handle_diff_notify),
                });

                return self.location(cx);
            }
        }

        self.active_item = None;
        self.location(cx)
    }

    fn pane_focus_update(
        &mut self,
        _pane_focused: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

impl Render for AgentDiffToolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let spinner_icon = div()
            .px_0p5()
            .id("generating")
            .tooltip(Tooltip::text("Generating Changes…"))
            .child(
                Icon::new(IconName::LoadCircle)
                    .size(IconSize::Small)
                    .color(Color::Accent)
                    .with_rotate_animation(3),
            )
            .into_any();

        let Some(active_item) = self.active_item.as_ref() else {
            return Empty.into_any();
        };

        match active_item {
            AgentDiffToolbarItem::Editor { editor, state, .. } => {
                let Some(editor) = editor.upgrade() else {
                    return Empty.into_any();
                };

                let editor_focus_handle = editor.read(cx).focus_handle(cx);

                let content = match state {
                    EditorState::Idle => return Empty.into_any(),
                    EditorState::Reviewing => vec![
                        h_flex()
                            .child(
                                IconButton::new("hunk-up", IconName::ArrowUp)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::for_action_title_in(
                                        "Previous Hunk",
                                        &GoToPreviousHunk,
                                        &editor_focus_handle,
                                    ))
                                    .on_click({
                                        let editor_focus_handle = editor_focus_handle.clone();
                                        move |_, window, cx| {
                                            editor_focus_handle.dispatch_action(
                                                &GoToPreviousHunk,
                                                window,
                                                cx,
                                            );
                                        }
                                    }),
                            )
                            .child(
                                IconButton::new("hunk-down", IconName::ArrowDown)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::for_action_title_in(
                                        "Next Hunk",
                                        &GoToHunk,
                                        &editor_focus_handle,
                                    ))
                                    .on_click({
                                        let editor_focus_handle = editor_focus_handle.clone();
                                        move |_, window, cx| {
                                            editor_focus_handle
                                                .dispatch_action(&GoToHunk, window, cx);
                                        }
                                    }),
                            )
                            .into_any_element(),
                        vertical_divider().into_any_element(),
                        h_flex()
                            .gap_0p5()
                            .child(
                                Button::new("reject-all", "Reject All")
                                    .key_binding({
                                        KeyBinding::for_action_in(
                                            &RejectAll,
                                            &editor_focus_handle,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(12.)))
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.dispatch_action(&RejectAll, window, cx)
                                    })),
                            )
                            .child(
                                Button::new("keep-all", "Keep All")
                                    .key_binding({
                                        KeyBinding::for_action_in(
                                            &KeepAll,
                                            &editor_focus_handle,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(12.)))
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.dispatch_action(&KeepAll, window, cx)
                                    })),
                            )
                            .into_any_element(),
                    ],
                };

                h_flex()
                    .track_focus(&editor_focus_handle)
                    .size_full()
                    .px_1()
                    .mr_1()
                    .gap_1()
                    .children(content)
                    .child(vertical_divider())
                    .when_some(editor.read(cx).workspace(), |this, _workspace| {
                        this.child(
                            IconButton::new("review", IconName::ListTodo)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Review All Files",
                                    &OpenAgentDiff,
                                    &editor_focus_handle,
                                ))
                                .on_click({
                                    cx.listener(move |this, _, window, cx| {
                                        this.dispatch_action(&OpenAgentDiff, window, cx);
                                    })
                                }),
                        )
                    })
                    .child(vertical_divider())
                    .on_action({
                        let editor = editor.clone();
                        move |_action: &OpenAgentDiff, window, cx| {
                            AgentDiff::global(cx).update(cx, |agent_diff, cx| {
                                agent_diff.deploy_pane_from_editor(&editor, window, cx);
                            });
                        }
                    })
                    .into_any()
            }
            AgentDiffToolbarItem::Pane(agent_diff) => {
                let Some(agent_diff) = agent_diff.upgrade() else {
                    return Empty.into_any();
                };

                let has_pending_edit_tool_use = agent_diff
                    .read(cx)
                    .thread
                    .read(cx)
                    .has_pending_edit_tool_calls();

                if has_pending_edit_tool_use {
                    return div().px_2().child(spinner_icon).into_any();
                }

                let is_empty = agent_diff.read(cx).multibuffer.read(cx).is_empty();
                if is_empty {
                    return Empty.into_any();
                }

                let focus_handle = agent_diff.focus_handle(cx);

                h_group_xl()
                    .my_neg_1()
                    .py_1()
                    .items_center()
                    .flex_wrap()
                    .child(
                        h_group_sm()
                            .child(
                                Button::new("reject-all", "Reject All")
                                    .key_binding({
                                        KeyBinding::for_action_in(&RejectAll, &focus_handle, cx)
                                            .map(|kb| kb.size(rems_from_px(12.)))
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.dispatch_action(&RejectAll, window, cx)
                                    })),
                            )
                            .child(
                                Button::new("keep-all", "Keep All")
                                    .key_binding({
                                        KeyBinding::for_action_in(&KeepAll, &focus_handle, cx)
                                            .map(|kb| kb.size(rems_from_px(12.)))
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.dispatch_action(&KeepAll, window, cx)
                                    })),
                            ),
                    )
                    .into_any()
            }
        }
    }
}

#[derive(Default)]
pub struct AgentDiff {
    reviewing_editors: HashMap<WeakEntity<Editor>, EditorState>,
    workspace_threads: HashMap<WeakEntity<Workspace>, WorkspaceThread>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorState {
    Idle,
    Reviewing,
}

struct WorkspaceThread {
    thread: WeakEntity<AcpThread>,
    _thread_subscriptions: (Subscription, Subscription),
    singleton_editors: HashMap<WeakEntity<Buffer>, HashMap<WeakEntity<Editor>, Subscription>>,
    _settings_subscription: Subscription,
    _workspace_subscription: Option<Subscription>,
}

struct AgentDiffGlobal(Entity<AgentDiff>);

impl Global for AgentDiffGlobal {}

impl AgentDiff {
    fn global(cx: &mut App) -> Entity<Self> {
        cx.try_global::<AgentDiffGlobal>()
            .map(|global| global.0.clone())
            .unwrap_or_else(|| {
                let entity = cx.new(|_cx| Self::default());
                let global = AgentDiffGlobal(entity.clone());
                cx.set_global(global);
                entity
            })
    }

    pub fn set_active_thread(
        workspace: &WeakEntity<Workspace>,
        thread: Entity<AcpThread>,
        window: &mut Window,
        cx: &mut App,
    ) {
        Self::global(cx).update(cx, |this, cx| {
            this.register_active_thread_impl(workspace, thread, window, cx);
        });
    }

    fn register_active_thread_impl(
        &mut self,
        workspace: &WeakEntity<Workspace>,
        thread: Entity<AcpThread>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let action_log = thread.read(cx).action_log().clone();

        let action_log_subscription = cx.observe_in(&action_log, window, {
            let workspace = workspace.clone();
            move |this, _action_log, window, cx| {
                this.update_reviewing_editors(&workspace, window, cx);
            }
        });

        let thread_subscription = cx.subscribe_in(&thread, window, {
            let workspace = workspace.clone();
            move |this, thread, event, window, cx| {
                this.handle_acp_thread_event(&workspace, thread, event, window, cx)
            }
        });

        if let Some(workspace_thread) = self.workspace_threads.get_mut(workspace) {
            // replace thread and action log subscription, but keep editors
            workspace_thread.thread = thread.downgrade();
            workspace_thread._thread_subscriptions = (action_log_subscription, thread_subscription);
            self.update_reviewing_editors(workspace, window, cx);
            return;
        }

        let settings_subscription = cx.observe_global_in::<SettingsStore>(window, {
            let workspace = workspace.clone();
            let mut was_active = AgentSettings::get_global(cx).single_file_review;
            move |this, window, cx| {
                let is_active = AgentSettings::get_global(cx).single_file_review;
                if was_active != is_active {
                    was_active = is_active;
                    this.update_reviewing_editors(&workspace, window, cx);
                }
            }
        });

        let workspace_subscription = workspace
            .upgrade()
            .map(|workspace| cx.subscribe_in(&workspace, window, Self::handle_workspace_event));

        self.workspace_threads.insert(
            workspace.clone(),
            WorkspaceThread {
                thread: thread.downgrade(),
                _thread_subscriptions: (action_log_subscription, thread_subscription),
                singleton_editors: HashMap::default(),
                _settings_subscription: settings_subscription,
                _workspace_subscription: workspace_subscription,
            },
        );

        let workspace = workspace.clone();
        cx.defer_in(window, move |this, window, cx| {
            if let Some(workspace) = workspace.upgrade() {
                this.register_workspace(workspace, window, cx);
            }
        });
    }

    fn register_workspace(
        &mut self,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let agent_diff = cx.entity();

        let editors = workspace.update(cx, |workspace, cx| {
            let agent_diff = agent_diff.clone();

            Self::register_review_action::<Keep>(workspace, Self::keep, &agent_diff);
            Self::register_review_action::<Reject>(workspace, Self::reject, &agent_diff);
            Self::register_review_action::<KeepAll>(workspace, Self::keep_all, &agent_diff);
            Self::register_review_action::<RejectAll>(workspace, Self::reject_all, &agent_diff);

            workspace.items_of_type(cx).collect::<Vec<_>>()
        });

        let weak_workspace = workspace.downgrade();

        for editor in editors {
            if let Some(buffer) = Self::full_editor_buffer(editor.read(cx), cx) {
                self.register_editor(weak_workspace.clone(), buffer, editor, window, cx);
            };
        }

        self.update_reviewing_editors(&weak_workspace, window, cx);
    }

    fn register_review_action<T: Action>(
        workspace: &mut Workspace,
        review: impl Fn(&Entity<Editor>, &Entity<AcpThread>, &mut Window, &mut App) -> PostReviewState
        + 'static,
        this: &Entity<AgentDiff>,
    ) {
        let this = this.clone();
        workspace.register_action(move |workspace, _: &T, window, cx| {
            let review = &review;
            let task = this.update(cx, |this, cx| {
                this.review_in_active_editor(workspace, review, window, cx)
            });

            if let Some(task) = task {
                task.detach_and_log_err(cx);
            } else {
                cx.propagate();
            }
        });
    }

    fn handle_acp_thread_event(
        &mut self,
        workspace: &WeakEntity<Workspace>,
        thread: &Entity<AcpThread>,
        event: &AcpThreadEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            AcpThreadEvent::NewEntry => {
                if thread
                    .read(cx)
                    .entries()
                    .last()
                    .is_some_and(|entry| entry.diffs().next().is_some())
                {
                    self.update_reviewing_editors(workspace, window, cx);
                }
            }
            AcpThreadEvent::EntryUpdated(ix) => {
                if thread
                    .read(cx)
                    .entries()
                    .get(*ix)
                    .is_some_and(|entry| entry.diffs().next().is_some())
                {
                    self.update_reviewing_editors(workspace, window, cx);
                }
            }
            AcpThreadEvent::Stopped
            | AcpThreadEvent::Error
            | AcpThreadEvent::LoadError(_)
            | AcpThreadEvent::Refusal => {
                self.update_reviewing_editors(workspace, window, cx);
            }
            AcpThreadEvent::TitleUpdated
            | AcpThreadEvent::TokenUsageUpdated
            | AcpThreadEvent::EntriesRemoved(_)
            | AcpThreadEvent::ToolAuthorizationRequired
            | AcpThreadEvent::PromptCapabilitiesUpdated
            | AcpThreadEvent::AvailableCommandsUpdated(_)
            | AcpThreadEvent::Retry(_)
            | AcpThreadEvent::ModeUpdated(_) => {}
        }
    }

    fn handle_workspace_event(
        &mut self,
        workspace: &Entity<Workspace>,
        event: &workspace::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let workspace::Event::ItemAdded { item } = event
            && let Some(editor) = item.downcast::<Editor>()
            && let Some(buffer) = Self::full_editor_buffer(editor.read(cx), cx)
        {
            self.register_editor(workspace.downgrade(), buffer, editor, window, cx);
        }
    }

    fn full_editor_buffer(editor: &Editor, cx: &App) -> Option<WeakEntity<Buffer>> {
        if editor.mode().is_full() {
            editor
                .buffer()
                .read(cx)
                .as_singleton()
                .map(|buffer| buffer.downgrade())
        } else {
            None
        }
    }

    fn register_editor(
        &mut self,
        workspace: WeakEntity<Workspace>,
        buffer: WeakEntity<Buffer>,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace_thread) = self.workspace_threads.get_mut(&workspace) else {
            return;
        };

        let weak_editor = editor.downgrade();

        workspace_thread
            .singleton_editors
            .entry(buffer.clone())
            .or_default()
            .entry(weak_editor.clone())
            .or_insert_with(|| {
                let workspace = workspace.clone();
                cx.observe_release(&editor, move |this, _, _cx| {
                    let Some(active_thread) = this.workspace_threads.get_mut(&workspace) else {
                        return;
                    };

                    if let Entry::Occupied(mut entry) =
                        active_thread.singleton_editors.entry(buffer)
                    {
                        let set = entry.get_mut();
                        set.remove(&weak_editor);

                        if set.is_empty() {
                            entry.remove();
                        }
                    }
                })
            });

        self.update_reviewing_editors(&workspace, window, cx);
    }

    fn update_reviewing_editors(
        &mut self,
        workspace: &WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !AgentSettings::get_global(cx).single_file_review {
            for (editor, _) in self.reviewing_editors.drain() {
                editor
                    .update(cx, |editor, cx| {
                        editor.end_temporary_diff_override(cx);
                        editor.unregister_addon::<EditorAgentDiffAddon>();
                    })
                    .ok();
            }
            return;
        }

        let Some(workspace_thread) = self.workspace_threads.get_mut(workspace) else {
            return;
        };

        let Some(thread) = workspace_thread.thread.upgrade() else {
            return;
        };

        let action_log = thread.read(cx).action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);

        let mut unaffected = self.reviewing_editors.clone();

        for (buffer, diff_handle) in changed_buffers {
            if buffer.read(cx).file().is_none() {
                continue;
            }

            let Some(buffer_editors) = workspace_thread.singleton_editors.get(&buffer.downgrade())
            else {
                continue;
            };

            for weak_editor in buffer_editors.keys() {
                let Some(editor) = weak_editor.upgrade() else {
                    continue;
                };

                let multibuffer = editor.read(cx).buffer().clone();
                multibuffer.update(cx, |multibuffer, cx| {
                    multibuffer.add_diff(diff_handle.clone(), cx);
                });

                let reviewing_state = EditorState::Reviewing;

                let previous_state = self
                    .reviewing_editors
                    .insert(weak_editor.clone(), reviewing_state.clone());

                if previous_state.is_none() {
                    editor.update(cx, |editor, cx| {
                        editor.start_temporary_diff_override();
                        editor.set_render_diff_hunk_controls(diff_hunk_controls(&thread), cx);
                        editor.set_expand_all_diff_hunks(cx);
                        editor.register_addon(EditorAgentDiffAddon);
                    });
                } else {
                    unaffected.remove(weak_editor);
                }

                if reviewing_state == EditorState::Reviewing
                    && previous_state != Some(reviewing_state)
                {
                    // Jump to first hunk when we enter review mode
                    editor.update(cx, |editor, cx| {
                        let snapshot = multibuffer.read(cx).snapshot(cx);
                        if let Some(first_hunk) = snapshot.diff_hunks().next() {
                            let first_hunk_start = first_hunk.multi_buffer_range().start;

                            editor.change_selections(
                                SelectionEffects::scroll(Autoscroll::center()),
                                window,
                                cx,
                                |selections| {
                                    selections.select_ranges([first_hunk_start..first_hunk_start])
                                },
                            );
                        }
                    });
                }
            }
        }

        // Remove editors from this workspace that are no longer under review
        for (editor, _) in unaffected {
            // Note: We could avoid this check by storing `reviewing_editors` by Workspace,
            // but that would add another lookup in `AgentDiff::editor_state`
            // which gets called much more frequently.
            let in_workspace = editor
                .read_with(cx, |editor, _cx| editor.workspace())
                .ok()
                .flatten()
                .is_some_and(|editor_workspace| {
                    editor_workspace.entity_id() == workspace.entity_id()
                });

            if in_workspace {
                editor
                    .update(cx, |editor, cx| {
                        editor.end_temporary_diff_override(cx);
                        editor.unregister_addon::<EditorAgentDiffAddon>();
                    })
                    .ok();
                self.reviewing_editors.remove(&editor);
            }
        }

        cx.notify();
    }

    fn editor_state(&self, editor: &WeakEntity<Editor>) -> EditorState {
        self.reviewing_editors
            .get(editor)
            .cloned()
            .unwrap_or(EditorState::Idle)
    }

    fn deploy_pane_from_editor(&self, editor: &Entity<Editor>, window: &mut Window, cx: &mut App) {
        let Some(workspace) = editor.read(cx).workspace() else {
            return;
        };

        let Some(WorkspaceThread { thread, .. }) =
            self.workspace_threads.get(&workspace.downgrade())
        else {
            return;
        };

        let Some(thread) = thread.upgrade() else {
            return;
        };

        AgentDiffPane::deploy(thread, workspace.downgrade(), window, cx).log_err();
    }

    fn keep_all(
        editor: &Entity<Editor>,
        thread: &Entity<AcpThread>,
        window: &mut Window,
        cx: &mut App,
    ) -> PostReviewState {
        editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            keep_edits_in_ranges(
                editor,
                &snapshot,
                thread,
                vec![editor::Anchor::min()..editor::Anchor::max()],
                window,
                cx,
            );
        });
        PostReviewState::AllReviewed
    }

    fn reject_all(
        editor: &Entity<Editor>,
        thread: &Entity<AcpThread>,
        window: &mut Window,
        cx: &mut App,
    ) -> PostReviewState {
        editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            reject_edits_in_ranges(
                editor,
                &snapshot,
                thread,
                vec![editor::Anchor::min()..editor::Anchor::max()],
                window,
                cx,
            );
        });
        PostReviewState::AllReviewed
    }

    fn keep(
        editor: &Entity<Editor>,
        thread: &Entity<AcpThread>,
        window: &mut Window,
        cx: &mut App,
    ) -> PostReviewState {
        editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            keep_edits_in_selection(editor, &snapshot, thread, window, cx);
            Self::post_review_state(&snapshot)
        })
    }

    fn reject(
        editor: &Entity<Editor>,
        thread: &Entity<AcpThread>,
        window: &mut Window,
        cx: &mut App,
    ) -> PostReviewState {
        editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            reject_edits_in_selection(editor, &snapshot, thread, window, cx);
            Self::post_review_state(&snapshot)
        })
    }

    fn post_review_state(snapshot: &MultiBufferSnapshot) -> PostReviewState {
        for (i, _) in snapshot.diff_hunks().enumerate() {
            if i > 0 {
                return PostReviewState::Pending;
            }
        }
        PostReviewState::AllReviewed
    }

    fn review_in_active_editor(
        &mut self,
        workspace: &mut Workspace,
        review: impl Fn(&Entity<Editor>, &Entity<AcpThread>, &mut Window, &mut App) -> PostReviewState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let active_item = workspace.active_item(cx)?;
        let editor = active_item.act_as::<Editor>(cx)?;

        if !matches!(
            self.editor_state(&editor.downgrade()),
            EditorState::Reviewing
        ) {
            return None;
        }

        let WorkspaceThread { thread, .. } =
            self.workspace_threads.get(&workspace.weak_handle())?;

        let thread = thread.upgrade()?;

        if let PostReviewState::AllReviewed = review(&editor, &thread, window, cx)
            && let Some(curr_buffer) = editor.read(cx).buffer().read(cx).as_singleton()
        {
            let changed_buffers = thread.read(cx).action_log().read(cx).changed_buffers(cx);

            let mut keys = changed_buffers.keys().cycle();
            keys.find(|k| *k == &curr_buffer);
            let next_project_path = keys
                .next()
                .filter(|k| *k != &curr_buffer)
                .and_then(|after| after.read(cx).project_path(cx));

            if let Some(path) = next_project_path {
                let task = workspace.open_path(path, None, true, window, cx);
                let task = cx.spawn(async move |_, _cx| task.await.map(|_| ()));
                return Some(task);
            }
        }

        Some(Task::ready(Ok(())))
    }
}

enum PostReviewState {
    AllReviewed,
    Pending,
}

pub struct EditorAgentDiffAddon;

impl editor::Addon for EditorAgentDiffAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn extend_key_context(&self, key_context: &mut gpui::KeyContext, _: &App) {
        key_context.add("agent_diff");
        key_context.add("editor_agent_diff");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Keep;
    use acp_thread::AgentConnection as _;
    use editor::EditorSettings;
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use std::{path::Path, rc::Rc};
    use util::path;

    #[gpui::test]
    async fn test_multibuffer_agent_diff(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            prompt_store::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            language_model::init_settings(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({"file1": "abc\ndef\nghi\njkl\nmno\npqr\nstu\nvwx\nyz"}),
        )
        .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
        let buffer_path = project
            .read_with(cx, |project, cx| {
                project.find_project_path("test/file1", cx)
            })
            .unwrap();

        let connection = Rc::new(acp_thread::StubAgentConnection::new());
        let thread = cx
            .update(|cx| {
                connection
                    .clone()
                    .new_thread(project.clone(), Path::new(path!("/test")), cx)
            })
            .await
            .unwrap();

        let action_log = cx.read(|cx| thread.read(cx).action_log().clone());

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let agent_diff = cx.new_window_entity(|window, cx| {
            AgentDiffPane::new(thread.clone(), workspace.downgrade(), window, cx)
        });
        let editor = agent_diff.read_with(cx, |diff, _cx| diff.editor.clone());

        let buffer = project
            .update(cx, |project, cx| project.open_buffer(buffer_path, cx))
            .await
            .unwrap();
        cx.update(|_, cx| {
            action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit(
                        [
                            (Point::new(1, 1)..Point::new(1, 2), "E"),
                            (Point::new(3, 2)..Point::new(3, 3), "L"),
                            (Point::new(5, 0)..Point::new(5, 1), "P"),
                            (Point::new(7, 1)..Point::new(7, 2), "W"),
                        ],
                        None,
                        cx,
                    )
                    .unwrap()
            });
            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
        });
        cx.run_until_parked();

        // When opening the assistant diff, the cursor is positioned on the first hunk.
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndef\ndEf\nghi\njkl\njkL\nmno\npqr\nPqr\nstu\nvwx\nvWx\nyz"
        );
        assert_eq!(
            editor
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(1, 0)..Point::new(1, 0)
        );

        // After keeping a hunk, the cursor should be positioned on the second hunk.
        agent_diff.update_in(cx, |diff, window, cx| diff.keep(&Keep, window, cx));
        cx.run_until_parked();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndEf\nghi\njkl\njkL\nmno\npqr\nPqr\nstu\nvwx\nvWx\nyz"
        );
        assert_eq!(
            editor
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );

        // Rejecting a hunk also moves the cursor to the next hunk, possibly cycling if it's at the end.
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(10, 0)..Point::new(10, 0)])
            });
        });
        agent_diff.update_in(cx, |diff, window, cx| {
            diff.reject(&crate::Reject, window, cx)
        });
        cx.run_until_parked();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndEf\nghi\njkl\njkL\nmno\npqr\nPqr\nstu\nvwx\nyz"
        );
        assert_eq!(
            editor
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );

        // Keeping a range that doesn't intersect the current selection doesn't move it.
        agent_diff.update_in(cx, |_diff, window, cx| {
            let position = editor
                .read(cx)
                .buffer()
                .read(cx)
                .read(cx)
                .anchor_before(Point::new(7, 0));
            editor.update(cx, |editor, cx| {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                keep_edits_in_ranges(
                    editor,
                    &snapshot,
                    &thread,
                    vec![position..position],
                    window,
                    cx,
                )
            });
        });
        cx.run_until_parked();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndEf\nghi\njkl\njkL\nmno\nPqr\nstu\nvwx\nyz"
        );
        assert_eq!(
            editor
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );
    }

    #[gpui::test]
    async fn test_singleton_agent_diff(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            prompt_store::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            language_model::init_settings(cx);
            workspace::register_project_item::<Editor>(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({"file1": "abc\ndef\nghi\njkl\nmno\npqr\nstu\nvwx\nyz"}),
        )
        .await;
        fs.insert_tree(path!("/test"), json!({"file2": "abc\ndef\nghi"}))
            .await;

        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
        let buffer_path1 = project
            .read_with(cx, |project, cx| {
                project.find_project_path("test/file1", cx)
            })
            .unwrap();
        let buffer_path2 = project
            .read_with(cx, |project, cx| {
                project.find_project_path("test/file2", cx)
            })
            .unwrap();

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // Add the diff toolbar to the active pane
        let diff_toolbar = cx.new_window_entity(|_, cx| AgentDiffToolbar::new(cx));

        workspace.update_in(cx, {
            let diff_toolbar = diff_toolbar.clone();

            move |workspace, window, cx| {
                workspace.active_pane().update(cx, |pane, cx| {
                    pane.toolbar().update(cx, |toolbar, cx| {
                        toolbar.add_item(diff_toolbar, window, cx);
                    });
                })
            }
        });

        let connection = Rc::new(acp_thread::StubAgentConnection::new());
        let thread = cx
            .update(|_, cx| {
                connection
                    .clone()
                    .new_thread(project.clone(), Path::new(path!("/test")), cx)
            })
            .await
            .unwrap();
        let action_log = thread.read_with(cx, |thread, _| thread.action_log().clone());

        // Set the active thread
        cx.update(|window, cx| {
            AgentDiff::set_active_thread(&workspace.downgrade(), thread.clone(), window, cx)
        });

        let buffer1 = project
            .update(cx, |project, cx| {
                project.open_buffer(buffer_path1.clone(), cx)
            })
            .await
            .unwrap();
        let buffer2 = project
            .update(cx, |project, cx| {
                project.open_buffer(buffer_path2.clone(), cx)
            })
            .await
            .unwrap();

        // Open an editor for buffer1
        let editor1 = cx.new_window_entity(|window, cx| {
            Editor::for_buffer(buffer1.clone(), Some(project.clone()), window, cx)
        });

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(editor1.clone()), None, true, window, cx);
        });
        cx.run_until_parked();

        // Toolbar knows about the current editor, but it's hidden since there are no changes yet
        assert!(diff_toolbar.read_with(cx, |toolbar, _cx| matches!(
            toolbar.active_item,
            Some(AgentDiffToolbarItem::Editor {
                state: EditorState::Idle,
                ..
            })
        )));
        assert_eq!(
            diff_toolbar.read_with(cx, |toolbar, cx| toolbar.location(cx)),
            ToolbarItemLocation::Hidden
        );

        // Make changes
        cx.update(|_, cx| {
            action_log.update(cx, |log, cx| log.buffer_read(buffer1.clone(), cx));
            buffer1.update(cx, |buffer, cx| {
                buffer
                    .edit(
                        [
                            (Point::new(1, 1)..Point::new(1, 2), "E"),
                            (Point::new(3, 2)..Point::new(3, 3), "L"),
                            (Point::new(5, 0)..Point::new(5, 1), "P"),
                            (Point::new(7, 1)..Point::new(7, 2), "W"),
                        ],
                        None,
                        cx,
                    )
                    .unwrap()
            });
            action_log.update(cx, |log, cx| log.buffer_edited(buffer1.clone(), cx));

            action_log.update(cx, |log, cx| log.buffer_read(buffer2.clone(), cx));
            buffer2.update(cx, |buffer, cx| {
                buffer
                    .edit(
                        [
                            (Point::new(0, 0)..Point::new(0, 1), "A"),
                            (Point::new(2, 1)..Point::new(2, 2), "H"),
                        ],
                        None,
                        cx,
                    )
                    .unwrap();
            });
            action_log.update(cx, |log, cx| log.buffer_edited(buffer2.clone(), cx));
        });
        cx.run_until_parked();

        // The already opened editor displays the diff and the cursor is at the first hunk
        assert_eq!(
            editor1.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndef\ndEf\nghi\njkl\njkL\nmno\npqr\nPqr\nstu\nvwx\nvWx\nyz"
        );
        assert_eq!(
            editor1
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(1, 0)..Point::new(1, 0)
        );

        // The toolbar is displayed in the right state
        assert_eq!(
            diff_toolbar.read_with(cx, |toolbar, cx| toolbar.location(cx)),
            ToolbarItemLocation::PrimaryRight
        );
        assert!(diff_toolbar.read_with(cx, |toolbar, _cx| matches!(
            toolbar.active_item,
            Some(AgentDiffToolbarItem::Editor {
                state: EditorState::Reviewing,
                ..
            })
        )));

        // The toolbar respects its setting
        override_toolbar_agent_review_setting(false, cx);
        assert_eq!(
            diff_toolbar.read_with(cx, |toolbar, cx| toolbar.location(cx)),
            ToolbarItemLocation::Hidden
        );
        override_toolbar_agent_review_setting(true, cx);
        assert_eq!(
            diff_toolbar.read_with(cx, |toolbar, cx| toolbar.location(cx)),
            ToolbarItemLocation::PrimaryRight
        );

        // After keeping a hunk, the cursor should be positioned on the second hunk.
        workspace.update(cx, |_, cx| {
            cx.dispatch_action(&Keep);
        });
        cx.run_until_parked();
        assert_eq!(
            editor1.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndEf\nghi\njkl\njkL\nmno\npqr\nPqr\nstu\nvwx\nvWx\nyz"
        );
        assert_eq!(
            editor1
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );

        // Rejecting a hunk also moves the cursor to the next hunk, possibly cycling if it's at the end.
        editor1.update_in(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(10, 0)..Point::new(10, 0)])
            });
        });
        workspace.update(cx, |_, cx| {
            cx.dispatch_action(&Reject);
        });
        cx.run_until_parked();
        assert_eq!(
            editor1.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndEf\nghi\njkl\njkL\nmno\npqr\nPqr\nstu\nvwx\nyz"
        );
        assert_eq!(
            editor1
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );

        // Keeping a range that doesn't intersect the current selection doesn't move it.
        editor1.update_in(cx, |editor, window, cx| {
            let buffer = editor.buffer().read(cx);
            let position = buffer.read(cx).anchor_before(Point::new(7, 0));
            let snapshot = buffer.snapshot(cx);
            keep_edits_in_ranges(
                editor,
                &snapshot,
                &thread,
                vec![position..position],
                window,
                cx,
            )
        });
        cx.run_until_parked();
        assert_eq!(
            editor1.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndEf\nghi\njkl\njkL\nmno\nPqr\nstu\nvwx\nyz"
        );
        assert_eq!(
            editor1
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );

        // Reviewing the last change opens the next changed buffer
        workspace
            .update_in(cx, |workspace, window, cx| {
                AgentDiff::global(cx).update(cx, |agent_diff, cx| {
                    agent_diff.review_in_active_editor(workspace, AgentDiff::keep, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();

        cx.run_until_parked();

        let editor2 = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<Editor>(cx).unwrap()
        });

        let editor2_path = editor2
            .read_with(cx, |editor, cx| editor.project_path(cx))
            .unwrap();
        assert_eq!(editor2_path, buffer_path2);

        assert_eq!(
            editor2.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\nAbc\ndef\nghi\ngHi"
        );
        assert_eq!(
            editor2
                .update(cx, |editor, cx| editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx)))
                .range(),
            Point::new(0, 0)..Point::new(0, 0)
        );

        // Editor 1 toolbar is hidden since all changes have been reviewed
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.activate_item(&editor1, true, true, window, cx)
        });

        assert!(diff_toolbar.read_with(cx, |toolbar, _cx| matches!(
            toolbar.active_item,
            Some(AgentDiffToolbarItem::Editor {
                state: EditorState::Idle,
                ..
            })
        )));
        assert_eq!(
            diff_toolbar.read_with(cx, |toolbar, cx| toolbar.location(cx)),
            ToolbarItemLocation::Hidden
        );
    }

    fn override_toolbar_agent_review_setting(active: bool, cx: &mut VisualTestContext) {
        cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, _cx| {
                let mut editor_settings = store.get::<EditorSettings>(None).clone();
                editor_settings.toolbar.agent_review = active;
                store.override_global(editor_settings);
            })
        });
        cx.run_until_parked();
    }
}
