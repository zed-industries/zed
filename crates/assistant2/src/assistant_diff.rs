use crate::{Thread, ThreadEvent};
use anyhow::Result;
use buffer_diff::DiffHunkStatus;
use collections::HashSet;
use editor::{Editor, EditorEvent, MultiBuffer};
use futures::future;
use gpui::{
    prelude::*, AnyElement, AnyView, App, Entity, EventEmitter, FocusHandle, Focusable,
    SharedString, Subscription, Task, WeakEntity, Window,
};
use language::{Capability, OffsetRangeExt};
use multi_buffer::PathKey;
use project::{Project, ProjectPath};
use std::{
    any::{Any, TypeId},
    ops::Range,
    sync::Arc,
};
use ui::{prelude::*, IconButtonShape};
use util::TryFutureExt;
use workspace::{
    item::{BreadcrumbText, ItemEvent, TabContentParams},
    searchable::SearchableItemHandle,
    Item, ItemHandle, ItemNavHistory, ToolbarItemLocation, Workspace,
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
                  cx: &mut App| {
                render_diff_hunk_controls(
                    row,
                    status,
                    hunk_range,
                    is_created_file,
                    line_height,
                    &assistant_diff,
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
        let unreviewed_buffers = thread.action_log().read(cx).unreviewed_buffers();
        let mut paths_to_delete = self.multibuffer.read(cx).paths().collect::<HashSet<_>>();

        for (buffer, tracked) in unreviewed_buffers {
            let Some(file) = buffer.read(cx).file().cloned() else {
                continue;
            };

            let path_key = PathKey::namespaced("", file.full_path(cx).into());
            paths_to_delete.remove(&path_key);

            let snapshot = buffer.read(cx).snapshot();
            let diff = tracked.diff().read(cx);
            let diff_hunk_ranges = diff
                .hunks_intersecting_range(
                    language::Anchor::MIN..language::Anchor::MAX,
                    &snapshot,
                    cx,
                )
                .map(|diff_hunk| diff_hunk.buffer_range.to_point(&snapshot))
                .collect::<Vec<_>>();

            let was_empty = self.multibuffer.update(cx, |multibuffer, cx| {
                let was_empty = multibuffer.is_empty();
                multibuffer.set_excerpts_for_path(
                    path_key.clone(),
                    buffer,
                    diff_hunk_ranges,
                    editor::DEFAULT_MULTIBUFFER_CONTEXT,
                    cx,
                );
                multibuffer.add_diff(tracked.diff().clone(), cx);
                was_empty
            });

            self.editor.update(cx, |editor, cx| {
                if was_empty {
                    editor.change_selections(None, window, cx, |selections| {
                        // TODO select the very beginning (possibly inside a deletion)
                        selections.select_ranges([0..0])
                    });
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

    fn review_diff_hunks(
        &mut self,
        hunk_ranges: Vec<Range<editor::Anchor>>,
        accept: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.multibuffer.read(cx).snapshot(cx);
        let diff_hunks_in_ranges = self
            .editor
            .read(cx)
            .diff_hunks_in_ranges(&hunk_ranges, &snapshot)
            .collect::<Vec<_>>();

        let mut tasks = Vec::new();
        for hunk in diff_hunks_in_ranges {
            let buffer = self.multibuffer.read(cx).buffer(hunk.buffer_id);
            if let Some(buffer) = buffer {
                let task = self.thread.update(cx, |thread, cx| {
                    thread.review_edits_in_range(buffer, hunk.buffer_range, accept, cx)
                });
                tasks.push(task.log_err());
            }
        }

        cx.spawn_in(window, async move |this, cx| {
            future::join_all(tasks).await;
            this.update_in(cx, |this, window, cx| this.update_excerpts(window, cx))
        })
        .detach_and_log_err(cx);
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
        Some("Project Diff".into())
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
        Some("Project Diff Opened")
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
    cx: &mut App,
) -> AnyElement {
    let editor = assistant_diff.read(cx).editor.clone();
    h_flex()
        .h(line_height)
        .mr_1()
        .gap_1()
        .px_0p5()
        .pb_1()
        .border_x_1()
        .border_b_1()
        .border_color(cx.theme().colors().border_variant)
        .rounded_b_lg()
        .bg(cx.theme().colors().editor_background)
        .gap_1()
        .occlude()
        .shadow_md()
        .children(if status.has_secondary_hunk() {
            vec![
                Button::new(("stage", row as u64), "Accept")
                    .alpha(if status.is_pending() { 0.66 } else { 1.0 })
                    // TODO: add tooltip
                    // .tooltip({
                    //     let focus_handle = editor.focus_handle(cx);
                    //     move |window, cx| {
                    //         Tooltip::for_action_in(
                    //             "Stage Hunk",
                    //             &::git::ToggleStaged,
                    //             &focus_handle,
                    //             window,
                    //             cx,
                    //         )
                    //     }
                    // })
                    .on_click({
                        let assistant_diff = assistant_diff.clone();
                        move |_event, window, cx| {
                            assistant_diff.update(cx, |diff, cx| {
                                diff.review_diff_hunks(
                                    vec![hunk_range.start..hunk_range.start],
                                    true,
                                    window,
                                    cx,
                                );
                            });
                        }
                    }),
                Button::new("undo", "Undo")
                    // TODO: add tooltip
                    // .tooltip({
                    //     let focus_handle = editor.focus_handle(cx);
                    //     move |window, cx| {
                    //         Tooltip::for_action_in("Undo Hunk", &::git::Undo, &focus_handle, window, cx)
                    //     }
                    // })
                    .on_click({
                        let _editor = editor.clone();
                        move |_event, _window, _cx| {
                            // editor.update(cx, |editor, cx| {
                            //     let snapshot = editor.snapshot(window, cx);
                            //     let point = hunk_range.start.to_point(&snapshot.buffer_snapshot);
                            //     editor.undo_hunks_in_ranges(vec![point..point], window, cx);
                            // });
                        }
                    })
                    .disabled(is_created_file),
            ]
        } else {
            vec![Button::new(("review", row as u64), "Review")
                .alpha(if status.is_pending() { 0.66 } else { 1.0 })
                // TODO: add tooltip
                // .tooltip({
                //     let focus_handle = editor.focus_handle(cx);
                //     move |window, cx| {
                //         Tooltip::for_action_in(
                //             "Review",
                //             &::git::ToggleStaged,
                //             &focus_handle,
                //             window,
                //             cx,
                //         )
                //     }
                // })
                .on_click({
                    let assistant_diff = assistant_diff.clone();
                    move |_event, window, cx| {
                        assistant_diff.update(cx, |diff, cx| {
                            diff.review_diff_hunks(
                                vec![hunk_range.start..hunk_range.start],
                                false,
                                window,
                                cx,
                            );
                        });
                    }
                })]
        })
        .when(
            !editor.read(cx).buffer().read(cx).all_diff_hunks_expanded(),
            |el| {
                el.child(
                    IconButton::new(("next-hunk", row as u64), IconName::ArrowDown)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        // .disabled(!has_multiple_hunks)
                        // TODO: add tooltip
                        // .tooltip({
                        //     let focus_handle = editor.focus_handle(cx);
                        //     move |window, cx| {
                        //         Tooltip::for_action_in(
                        //             "Next Hunk",
                        //             &GoToHunk,
                        //             &focus_handle,
                        //             window,
                        //             cx,
                        //         )
                        //     }
                        // })
                        .on_click({
                            let _editor = editor.clone();
                            move |_event, _window, _cx| {
                                // TODO: wire this up
                                // editor.update(cx, |editor, cx| {
                                //     let snapshot = editor.snapshot(window, cx);
                                //     let position =
                                //         hunk_range.end.to_point(&snapshot.buffer_snapshot);
                                //     editor.go_to_hunk_before_or_after_position(
                                //         &snapshot,
                                //         position,
                                //         Direction::Next,
                                //         window,
                                //         cx,
                                //     );
                                //     editor.expand_selected_diff_hunks(cx);
                                // });
                            }
                        }),
                )
                .child(
                    IconButton::new(("prev-hunk", row as u64), IconName::ArrowUp)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        // .disabled(!has_multiple_hunks)
                        // TODO: add tooltip
                        // .tooltip({
                        //     let focus_handle = editor.focus_handle(cx);
                        //     move |window, cx| {
                        //         Tooltip::for_action_in(
                        //             "Previous Hunk",
                        //             &GoToPreviousHunk,
                        //             &focus_handle,
                        //             window,
                        //             cx,
                        //         )
                        //     }
                        // })
                        .on_click({
                            let _editor = editor.clone();
                            move |_event, _window, _cx| {
                                // TODO: wire this up
                                // editor.update(cx, |editor, cx| {
                                //     let snapshot = editor.snapshot(window, cx);
                                //     let point =
                                //         hunk_range.start.to_point(&snapshot.buffer_snapshot);
                                //     editor.go_to_hunk_before_or_after_position(
                                //         &snapshot,
                                //         point,
                                //         Direction::Prev,
                                //         window,
                                //         cx,
                                //     );
                                //     editor.expand_selected_diff_hunks(cx);
                                // });
                            }
                        }),
                )
            },
        )
        .into_any_element()
}
