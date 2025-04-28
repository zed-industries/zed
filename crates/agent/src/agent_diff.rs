use crate::{Keep, KeepAll, Reject, RejectAll, Thread, ThreadEvent, ui::AnimatedLabel};
use anyhow::Result;
use buffer_diff::DiffHunkStatus;
use collections::{HashMap, HashSet};
use editor::{
    Direction, Editor, EditorEvent, MultiBuffer, ToPoint,
    actions::{GoToHunk, GoToPreviousHunk},
    scroll::Autoscroll,
};
use gpui::{
    Action, AnyElement, AnyView, App, Empty, Entity, EventEmitter, FocusHandle, Focusable,
    SharedString, Subscription, Task, WeakEntity, Window, prelude::*,
};
use language::{Capability, DiskState, OffsetRangeExt, Point};
use multi_buffer::PathKey;
use project::{Project, ProjectPath};
use std::{
    any::{Any, TypeId},
    ops::Range,
    sync::Arc,
};
use ui::{IconButtonShape, KeyBinding, Tooltip, prelude::*};
use workspace::{
    Item, ItemHandle, ItemNavHistory, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace,
    item::{BreadcrumbText, ItemEvent, TabContentParams},
    searchable::SearchableItemHandle,
};
use zed_actions::assistant::ToggleFocus;

pub struct AgentDiff {
    multibuffer: Entity<MultiBuffer>,
    editor: Entity<Editor>,
    thread: Entity<Thread>,
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    title: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl AgentDiff {
    pub fn deploy(
        thread: Entity<Thread>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Entity<Self>> {
        workspace.update(cx, |workspace, cx| {
            Self::deploy_in_workspace(thread, workspace, window, cx)
        })
    }

    pub fn deploy_in_workspace(
        thread: Entity<Thread>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let existing_diff = workspace
            .items_of_type::<AgentDiff>(cx)
            .find(|diff| diff.read(cx).thread == thread);
        if let Some(existing_diff) = existing_diff {
            workspace.activate_item(&existing_diff, true, true, window, cx);
            existing_diff
        } else {
            let agent_diff =
                cx.new(|cx| AgentDiff::new(thread.clone(), workspace.weak_handle(), window, cx));
            workspace.add_item_to_center(Box::new(agent_diff.clone()), window, cx);
            agent_diff
        }
    }

    pub fn new(
        thread: Entity<Thread>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));

        let project = thread.read(cx).project().clone();
        let render_diff_hunk_controls = Arc::new({
            let agent_diff = cx.entity();
            move |row,
                  status: &DiffHunkStatus,
                  hunk_range,
                  is_created_file,
                  line_height,
                  editor: &Entity<Editor>,
                  window: &mut Window,
                  cx: &mut App| {
                render_diff_hunk_controls(
                    row,
                    status,
                    hunk_range,
                    is_created_file,
                    line_height,
                    &agent_diff,
                    editor,
                    window,
                    cx,
                )
            }
        });
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.disable_inline_diagnostics();
            editor.set_expand_all_diff_hunks(cx);
            editor.set_render_diff_hunk_controls(render_diff_hunk_controls, cx);
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
                    this.handle_thread_event(event, cx)
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
        let thread = self.thread.read(cx);
        let changed_buffers = thread.action_log().read(cx).changed_buffers(cx);
        let mut paths_to_delete = self.multibuffer.read(cx).paths().collect::<HashSet<_>>();

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
                    language::Anchor::MIN..language::Anchor::MAX,
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
                        editor::DEFAULT_MULTIBUFFER_CONTEXT,
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
                        editor.change_selections(
                            Some(Autoscroll::fit()),
                            window,
                            cx,
                            |selections| {
                                selections
                                    .select_anchor_ranges([first_hunk_start..first_hunk_start]);
                            },
                        )
                    }
                }

                if is_excerpt_newly_added
                    && buffer
                        .read(cx)
                        .file()
                        .map_or(false, |file| file.disk_state() == DiskState::Deleted)
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
        let new_title = self
            .thread
            .read(cx)
            .summary()
            .unwrap_or("Assistant Changes".into());
        if new_title != self.title {
            self.title = new_title;
            cx.emit(EditorEvent::TitleChanged);
        }
    }

    fn handle_thread_event(&mut self, event: &ThreadEvent, cx: &mut Context<Self>) {
        match event {
            ThreadEvent::SummaryGenerated => self.update_title(cx),
            _ => {}
        }
    }

    pub fn move_to_path(&mut self, path_key: PathKey, window: &mut Window, cx: &mut Context<Self>) {
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
                    editor.change_selections(Some(Autoscroll::fit()), window, cx, |selections| {
                        selections.select_anchor_ranges([first_hunk_start..first_hunk_start]);
                    })
                }
            });
        }
    }

    fn keep(&mut self, _: &crate::Keep, window: &mut Window, cx: &mut Context<Self>) {
        let ranges = self
            .editor
            .read(cx)
            .selections
            .disjoint_anchor_ranges()
            .collect::<Vec<_>>();
        self.keep_edits_in_ranges(ranges, window, cx);
    }

    fn reject(&mut self, _: &crate::Reject, window: &mut Window, cx: &mut Context<Self>) {
        let ranges = self
            .editor
            .read(cx)
            .selections
            .disjoint_anchor_ranges()
            .collect::<Vec<_>>();
        self.reject_edits_in_ranges(ranges, window, cx);
    }

    fn reject_all(&mut self, _: &crate::RejectAll, window: &mut Window, cx: &mut Context<Self>) {
        self.reject_edits_in_ranges(
            vec![editor::Anchor::min()..editor::Anchor::max()],
            window,
            cx,
        );
    }

    fn keep_all(&mut self, _: &crate::KeepAll, _window: &mut Window, cx: &mut Context<Self>) {
        self.thread
            .update(cx, |thread, cx| thread.keep_all_edits(cx));
    }

    fn keep_edits_in_ranges(
        &mut self,
        ranges: Vec<Range<editor::Anchor>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.thread.read(cx).is_generating() {
            return;
        }

        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let diff_hunks_in_ranges = self
            .editor
            .read(cx)
            .diff_hunks_in_ranges(&ranges, &snapshot)
            .collect::<Vec<_>>();
        let newest_cursor = self.editor.update(cx, |editor, cx| {
            editor.selections.newest::<Point>(cx).head()
        });
        if diff_hunks_in_ranges.iter().any(|hunk| {
            hunk.row_range
                .contains(&multi_buffer::MultiBufferRow(newest_cursor.row))
        }) {
            self.update_selection(&diff_hunks_in_ranges, window, cx);
        }

        for hunk in &diff_hunks_in_ranges {
            let buffer = self.multibuffer.read(cx).buffer(hunk.buffer_id);
            if let Some(buffer) = buffer {
                self.thread.update(cx, |thread, cx| {
                    thread.keep_edits_in_range(buffer, hunk.buffer_range.clone(), cx)
                });
            }
        }
    }

    fn reject_edits_in_ranges(
        &mut self,
        ranges: Vec<Range<editor::Anchor>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.thread.read(cx).is_generating() {
            return;
        }

        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let diff_hunks_in_ranges = self
            .editor
            .read(cx)
            .diff_hunks_in_ranges(&ranges, &snapshot)
            .collect::<Vec<_>>();
        let newest_cursor = self.editor.update(cx, |editor, cx| {
            editor.selections.newest::<Point>(cx).head()
        });
        if diff_hunks_in_ranges.iter().any(|hunk| {
            hunk.row_range
                .contains(&multi_buffer::MultiBufferRow(newest_cursor.row))
        }) {
            self.update_selection(&diff_hunks_in_ranges, window, cx);
        }

        let mut ranges_by_buffer = HashMap::default();
        for hunk in &diff_hunks_in_ranges {
            let buffer = self.multibuffer.read(cx).buffer(hunk.buffer_id);
            if let Some(buffer) = buffer {
                ranges_by_buffer
                    .entry(buffer.clone())
                    .or_insert_with(Vec::new)
                    .push(hunk.buffer_range.clone());
            }
        }

        for (buffer, ranges) in ranges_by_buffer {
            self.thread
                .update(cx, |thread, cx| {
                    thread.reject_edits_in_ranges(buffer, ranges, cx)
                })
                .detach_and_log_err(cx);
        }
    }

    fn update_selection(
        &mut self,
        diff_hunks: &[multi_buffer::MultiBufferDiffHunk],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let target_hunk = diff_hunks
            .last()
            .and_then(|last_kept_hunk| {
                let last_kept_hunk_end = last_kept_hunk.multi_buffer_range().end;
                self.editor
                    .read(cx)
                    .diff_hunks_in_ranges(&[last_kept_hunk_end..editor::Anchor::max()], &snapshot)
                    .skip(1)
                    .next()
            })
            .or_else(|| {
                let first_kept_hunk = diff_hunks.first()?;
                let first_kept_hunk_start = first_kept_hunk.multi_buffer_range().start;
                self.editor
                    .read(cx)
                    .diff_hunks_in_ranges(
                        &[editor::Anchor::min()..first_kept_hunk_start],
                        &snapshot,
                    )
                    .next()
            });

        if let Some(target_hunk) = target_hunk {
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |selections| {
                    let next_hunk_start = target_hunk.multi_buffer_range().start;
                    selections.select_anchor_ranges([next_hunk_start..next_hunk_start]);
                })
            });
        }
    }
}

impl EventEmitter<EditorEvent> for AgentDiff {}

impl Focusable for AgentDiff {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.multibuffer.read(cx).is_empty() {
            self.focus_handle.clone()
        } else {
            self.editor.focus_handle(cx)
        }
    }
}

impl Item for AgentDiff {
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
        let summary = self
            .thread
            .read(cx)
            .summary()
            .unwrap_or("Assistant Changes".into());
        Label::new(format!("Review: {}", summary))
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

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn is_singleton(&self, _: &App) -> bool {
        false
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

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| Self::new(self.thread.clone(), self.workspace.clone(), window, cx)))
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
        format: bool,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.save(format, project, window, cx)
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
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
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

impl Render for AgentDiff {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                                    window,
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

fn render_diff_hunk_controls(
    row: u32,
    _status: &DiffHunkStatus,
    hunk_range: Range<editor::Anchor>,
    is_created_file: bool,
    line_height: Pixels,
    agent_diff: &Entity<AgentDiff>,
    editor: &Entity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let editor = editor.clone();

    if agent_diff.read(cx).thread.read(cx).is_generating() {
        return Empty.into_any();
    }

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
        .occlude()
        .shadow_md()
        .children(vec![
            Button::new(("reject", row as u64), "Reject")
                .disabled(is_created_file)
                .key_binding(
                    KeyBinding::for_action_in(
                        &Reject,
                        &editor.read(cx).focus_handle(cx),
                        window,
                        cx,
                    )
                    .map(|kb| kb.size(rems_from_px(12.))),
                )
                .on_click({
                    let agent_diff = agent_diff.clone();
                    move |_event, window, cx| {
                        agent_diff.update(cx, |diff, cx| {
                            diff.reject_edits_in_ranges(
                                vec![hunk_range.start..hunk_range.start],
                                window,
                                cx,
                            );
                        });
                    }
                }),
            Button::new(("keep", row as u64), "Keep")
                .key_binding(
                    KeyBinding::for_action_in(&Keep, &editor.read(cx).focus_handle(cx), window, cx)
                        .map(|kb| kb.size(rems_from_px(12.))),
                )
                .on_click({
                    let agent_diff = agent_diff.clone();
                    move |_event, window, cx| {
                        agent_diff.update(cx, |diff, cx| {
                            diff.keep_edits_in_ranges(
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
                            move |_event, window, cx| {
                                editor.update(cx, |editor, cx| {
                                    let snapshot = editor.snapshot(window, cx);
                                    let position =
                                        hunk_range.end.to_point(&snapshot.buffer_snapshot);
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
                            move |window, cx| {
                                Tooltip::for_action_in(
                                    "Previous Hunk",
                                    &GoToPreviousHunk,
                                    &focus_handle,
                                    window,
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
                                        hunk_range.start.to_point(&snapshot.buffer_snapshot);
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
    agent_diff: Option<WeakEntity<AgentDiff>>,
}

impl AgentDiffToolbar {
    pub fn new() -> Self {
        Self { agent_diff: None }
    }

    fn agent_diff(&self, _: &App) -> Option<Entity<AgentDiff>> {
        self.agent_diff.as_ref()?.upgrade()
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(agent_diff) = self.agent_diff(cx) {
            agent_diff.focus_handle(cx).focus(window);
        }
        let action = action.boxed_clone();
        cx.defer(move |cx| {
            cx.dispatch_action(action.as_ref());
        })
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
        self.agent_diff = active_pane_item
            .and_then(|item| item.act_as::<AgentDiff>(cx))
            .map(|entity| entity.downgrade());
        if self.agent_diff.is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let agent_diff = match self.agent_diff(cx) {
            Some(ad) => ad,
            None => return div(),
        };

        let is_generating = agent_diff.read(cx).thread.read(cx).is_generating();
        if is_generating {
            return div()
                .w(rems(6.5625)) // Arbitrary 105px size—so the label doesn't dance around
                .child(AnimatedLabel::new("Generating"));
        }

        let is_empty = agent_diff.read(cx).multibuffer.read(cx).is_empty();
        if is_empty {
            return div();
        }

        let focus_handle = agent_diff.focus_handle(cx);

        h_group_xl()
            .my_neg_1()
            .items_center()
            .p_1()
            .flex_wrap()
            .justify_between()
            .child(
                h_group_sm()
                    .child(
                        Button::new("reject-all", "Reject All")
                            .key_binding({
                                KeyBinding::for_action_in(&RejectAll, &focus_handle, window, cx)
                                    .map(|kb| kb.size(rems_from_px(12.)))
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dispatch_action(&RejectAll, window, cx)
                            })),
                    )
                    .child(
                        Button::new("keep-all", "Keep All")
                            .key_binding({
                                KeyBinding::for_action_in(&KeepAll, &focus_handle, window, cx)
                                    .map(|kb| kb.size(rems_from_px(12.)))
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dispatch_action(&KeepAll, window, cx)
                            })),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ThreadStore, thread_store};
    use assistant_settings::AssistantSettings;
    use assistant_tool::ToolWorkingSet;
    use context_server::ContextServerSettings;
    use editor::EditorSettings;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use prompt_store::PromptBuilder;
    use serde_json::json;
    use settings::{Settings, SettingsStore};
    use std::sync::Arc;
    use theme::ThemeSettings;
    use util::path;

    #[gpui::test]
    async fn test_agent_diff(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            AssistantSettings::register(cx);
            prompt_store::init(cx);
            thread_store::init(cx);
            workspace::init_settings(cx);
            ThemeSettings::register(cx);
            ContextServerSettings::register(cx);
            EditorSettings::register(cx);
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

        let prompt_store = None;
        let thread_store = cx
            .update(|cx| {
                ThreadStore::load(
                    project.clone(),
                    cx.new(|_| ToolWorkingSet::default()),
                    prompt_store,
                    Arc::new(PromptBuilder::new(None).unwrap()),
                    cx,
                )
            })
            .await
            .unwrap();
        let thread = thread_store.update(cx, |store, cx| store.create_thread(cx));
        let action_log = thread.read_with(cx, |thread, _| thread.action_log().clone());

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let agent_diff = cx.new_window_entity(|window, cx| {
            AgentDiff::new(thread.clone(), workspace.downgrade(), window, cx)
        });
        let editor = agent_diff.read_with(cx, |diff, _cx| diff.editor.clone());

        let buffer = project
            .update(cx, |project, cx| project.open_buffer(buffer_path, cx))
            .await
            .unwrap();
        cx.update(|_, cx| {
            action_log.update(cx, |log, cx| log.track_buffer(buffer.clone(), cx));
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
                .update(cx, |editor, cx| editor.selections.newest::<Point>(cx))
                .range(),
            Point::new(1, 0)..Point::new(1, 0)
        );

        // After keeping a hunk, the cursor should be positioned on the second hunk.
        agent_diff.update_in(cx, |diff, window, cx| diff.keep(&crate::Keep, window, cx));
        cx.run_until_parked();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndEf\nghi\njkl\njkL\nmno\npqr\nPqr\nstu\nvwx\nvWx\nyz"
        );
        assert_eq!(
            editor
                .update(cx, |editor, cx| editor.selections.newest::<Point>(cx))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );

        // Rejecting a hunk also moves the cursor to the next hunk, possibly cycling if it's at the end.
        editor.update_in(cx, |editor, window, cx| {
            editor.change_selections(None, window, cx, |selections| {
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
                .update(cx, |editor, cx| editor.selections.newest::<Point>(cx))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );

        // Keeping a range that doesn't intersect the current selection doesn't move it.
        agent_diff.update_in(cx, |diff, window, cx| {
            let position = editor
                .read(cx)
                .buffer()
                .read(cx)
                .read(cx)
                .anchor_before(Point::new(7, 0));
            diff.keep_edits_in_ranges(vec![position..position], window, cx)
        });
        cx.run_until_parked();
        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.text(cx)),
            "abc\ndEf\nghi\njkl\njkL\nmno\nPqr\nstu\nvwx\nyz"
        );
        assert_eq!(
            editor
                .update(cx, |editor, cx| editor.selections.newest::<Point>(cx))
                .range(),
            Point::new(3, 0)..Point::new(3, 0)
        );
    }
}
