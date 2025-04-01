use crate::{Thread, ThreadEvent, ToggleKeep};
use anyhow::Result;
use buffer_diff::DiffHunkStatus;
use collections::HashSet;
use editor::{
    Direction, Editor, EditorEvent, MultiBuffer, ToPoint,
    actions::{GoToHunk, GoToPreviousHunk},
};
use gpui::{
    Action, AnyElement, AnyView, App, Entity, EventEmitter, FocusHandle, Focusable, SharedString,
    Subscription, Task, WeakEntity, Window, prelude::*,
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

pub struct AssistantDiff {
    multibuffer: Entity<MultiBuffer>,
    editor: Entity<Editor>,
    thread: Entity<Thread>,
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    title: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl AssistantDiff {
    pub fn deploy(
        thread: Entity<Thread>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        let existing_diff = workspace.update(cx, |workspace, cx| {
            workspace
                .items_of_type::<AssistantDiff>(cx)
                .find(|diff| diff.read(cx).thread == thread)
        })?;
        if let Some(existing_diff) = existing_diff {
            workspace.update(cx, |workspace, cx| {
                workspace.activate_item(&existing_diff, true, true, window, cx);
            })
        } else {
            let assistant_diff =
                cx.new(|cx| AssistantDiff::new(thread.clone(), workspace.clone(), window, cx));
            workspace.update(cx, |workspace, cx| {
                workspace.add_item_to_center(Box::new(assistant_diff), window, cx);
            })
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
            let assistant_diff = cx.entity();
            move |row,
                  status: &DiffHunkStatus,
                  hunk_range,
                  is_created_file,
                  line_height,
                  _editor: &Entity<Editor>,
                  window: &mut Window,
                  cx: &mut App| {
                render_diff_hunk_controls(
                    row,
                    status,
                    hunk_range,
                    is_created_file,
                    line_height,
                    &assistant_diff,
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
            editor.register_addon(AssistantDiffAddon);
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

        for (buffer, changed) in changed_buffers {
            let Some(file) = buffer.read(cx).file().cloned() else {
                continue;
            };

            let path_key = PathKey::namespaced("", file.full_path(cx).into());
            paths_to_delete.remove(&path_key);

            let snapshot = buffer.read(cx).snapshot();
            let diff = changed.diff.read(cx);
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
                    let is_excerpt_newly_added = multibuffer.set_excerpts_for_path(
                        path_key.clone(),
                        buffer.clone(),
                        diff_hunk_ranges,
                        editor::DEFAULT_MULTIBUFFER_CONTEXT,
                        cx,
                    );
                    multibuffer.add_diff(changed.diff.clone(), cx);
                    (was_empty, is_excerpt_newly_added)
                });

            self.editor.update(cx, |editor, cx| {
                if was_empty {
                    editor.change_selections(None, window, cx, |selections| {
                        selections.select_ranges([0..0])
                    });
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
            ThreadEvent::SummaryChanged => self.update_title(cx),
            _ => {}
        }
    }

    fn toggle_keep(&mut self, _: &crate::ToggleKeep, _window: &mut Window, cx: &mut Context<Self>) {
        let ranges = self
            .editor
            .read(cx)
            .selections
            .disjoint_anchor_ranges()
            .collect::<Vec<_>>();

        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let diff_hunks_in_ranges = self
            .editor
            .read(cx)
            .diff_hunks_in_ranges(&ranges, &snapshot)
            .collect::<Vec<_>>();

        for hunk in diff_hunks_in_ranges {
            let buffer = self.multibuffer.read(cx).buffer(hunk.buffer_id);
            if let Some(buffer) = buffer {
                self.thread.update(cx, |thread, cx| {
                    let accept = hunk.status().has_secondary_hunk();
                    thread.review_edits_in_range(buffer, hunk.buffer_range, accept, cx)
                });
            }
        }
    }

    fn reject(&mut self, _: &crate::Reject, window: &mut Window, cx: &mut Context<Self>) {
        let ranges = self
            .editor
            .update(cx, |editor, cx| editor.selections.ranges(cx));
        self.editor.update(cx, |editor, cx| {
            editor.restore_hunks_in_ranges(ranges, window, cx)
        })
    }

    fn reject_all(&mut self, _: &crate::RejectAll, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let max_point = editor.buffer().read(cx).read(cx).max_point();
            editor.restore_hunks_in_ranges(vec![Point::zero()..max_point], window, cx)
        })
    }

    fn keep_all(&mut self, _: &crate::KeepAll, _window: &mut Window, cx: &mut Context<Self>) {
        self.thread
            .update(cx, |thread, cx| thread.keep_all_edits(cx));
    }

    fn review_diff_hunks(
        &mut self,
        hunk_ranges: Vec<Range<editor::Anchor>>,
        accept: bool,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let diff_hunks_in_ranges = self
            .editor
            .read(cx)
            .diff_hunks_in_ranges(&hunk_ranges, &snapshot)
            .collect::<Vec<_>>();

        for hunk in diff_hunks_in_ranges {
            let buffer = self.multibuffer.read(cx).buffer(hunk.buffer_id);
            if let Some(buffer) = buffer {
                self.thread.update(cx, |thread, cx| {
                    thread.review_edits_in_range(buffer, hunk.buffer_range, accept, cx)
                });
            }
        }
    }
}

impl EventEmitter<EditorEvent> for AssistantDiff {}

impl Focusable for AssistantDiff {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.multibuffer.read(cx).is_empty() {
            self.focus_handle.clone()
        } else {
            self.editor.focus_handle(cx)
        }
    }
}

impl Item for AssistantDiff {
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
        Some("Assistant Diff".into())
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
}

impl Render for AssistantDiff {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.multibuffer.read(cx).is_empty();

        div()
            .track_focus(&self.focus_handle)
            .key_context(if is_empty {
                "EmptyPane"
            } else {
                "AssistantDiff"
            })
            .on_action(cx.listener(Self::toggle_keep))
            .on_action(cx.listener(Self::reject))
            .on_action(cx.listener(Self::reject_all))
            .on_action(cx.listener(Self::keep_all))
            .bg(cx.theme().colors().editor_background)
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .when(is_empty, |el| el.child("No changes to review"))
            .when(!is_empty, |el| el.child(self.editor.clone()))
    }
}

fn render_diff_hunk_controls(
    row: u32,
    status: &DiffHunkStatus,
    hunk_range: Range<editor::Anchor>,
    is_created_file: bool,
    line_height: Pixels,
    assistant_diff: &Entity<AssistantDiff>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let editor = assistant_diff.read(cx).editor.clone();

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
        .children(if status.has_secondary_hunk() {
            vec![
                Button::new("reject", "Reject")
                    .disabled(is_created_file)
                    .key_binding(
                        KeyBinding::for_action_in(
                            &crate::Reject,
                            &editor.read(cx).focus_handle(cx),
                            window,
                            cx,
                        )
                        .map(|kb| kb.size(rems_from_px(12.))),
                    )
                    .on_click({
                        let editor = editor.clone();
                        move |_event, window, cx| {
                            editor.update(cx, |editor, cx| {
                                let snapshot = editor.snapshot(window, cx);
                                let point = hunk_range.start.to_point(&snapshot.buffer_snapshot);
                                editor.restore_hunks_in_ranges(vec![point..point], window, cx);
                            });
                        }
                    }),
                Button::new(("keep", row as u64), "Keep")
                    .key_binding(
                        KeyBinding::for_action_in(
                            &crate::ToggleKeep,
                            &editor.read(cx).focus_handle(cx),
                            window,
                            cx,
                        )
                        .map(|kb| kb.size(rems_from_px(12.))),
                    )
                    .on_click({
                        let assistant_diff = assistant_diff.clone();
                        move |_event, _window, cx| {
                            assistant_diff.update(cx, |diff, cx| {
                                diff.review_diff_hunks(
                                    vec![hunk_range.start..hunk_range.start],
                                    true,
                                    cx,
                                );
                            });
                        }
                    }),
            ]
        } else {
            vec![
                Button::new(("review", row as u64), "Review")
                    .key_binding(KeyBinding::for_action_in(
                        &ToggleKeep,
                        &editor.read(cx).focus_handle(cx),
                        window,
                        cx,
                    ))
                    .on_click({
                        let assistant_diff = assistant_diff.clone();
                        move |_event, _window, cx| {
                            assistant_diff.update(cx, |diff, cx| {
                                diff.review_diff_hunks(
                                    vec![hunk_range.start..hunk_range.start],
                                    false,
                                    cx,
                                );
                            });
                        }
                    }),
            ]
        })
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

struct AssistantDiffAddon;

impl editor::Addon for AssistantDiffAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn extend_key_context(&self, key_context: &mut gpui::KeyContext, _: &App) {
        key_context.add("assistant_diff");
    }
}

pub struct AssistantDiffToolbar {
    assistant_diff: Option<WeakEntity<AssistantDiff>>,
    _workspace: WeakEntity<Workspace>,
}

impl AssistantDiffToolbar {
    pub fn new(workspace: &Workspace, _: &mut Context<Self>) -> Self {
        Self {
            assistant_diff: None,
            _workspace: workspace.weak_handle(),
        }
    }

    fn assistant_diff(&self, _: &App) -> Option<Entity<AssistantDiff>> {
        self.assistant_diff.as_ref()?.upgrade()
    }

    fn dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(assistant_diff) = self.assistant_diff(cx) {
            assistant_diff.focus_handle(cx).focus(window);
        }
        let action = action.boxed_clone();
        cx.defer(move |cx| {
            cx.dispatch_action(action.as_ref());
        })
    }
}

impl EventEmitter<ToolbarItemEvent> for AssistantDiffToolbar {}

impl ToolbarItemView for AssistantDiffToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.assistant_diff = active_pane_item
            .and_then(|item| item.act_as::<AssistantDiff>(cx))
            .map(|entity| entity.downgrade());
        if self.assistant_diff.is_some() {
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

impl Render for AssistantDiffToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let assistant_diff = match self.assistant_diff(cx) {
            Some(ad) => ad,
            None => return div(),
        };

        let is_empty = assistant_diff.read(cx).multibuffer.read(cx).is_empty();

        if is_empty {
            return div();
        }

        h_group_xl()
            .my_neg_1()
            .items_center()
            .p_1()
            .flex_wrap()
            .justify_between()
            .child(
                h_group_sm()
                    .child(
                        Button::new("reject-all", "Reject All").on_click(cx.listener(
                            |this, _, window, cx| {
                                this.dispatch_action(&crate::RejectAll, window, cx)
                            },
                        )),
                    )
                    .child(Button::new("keep-all", "Keep All").on_click(cx.listener(
                        |this, _, window, cx| this.dispatch_action(&crate::KeepAll, window, cx),
                    ))),
            )
    }
}
