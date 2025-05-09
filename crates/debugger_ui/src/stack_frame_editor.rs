use std::any::{Any, TypeId};

use dap::StackFrameId;
use editor::{
    Bias, DebugStackFrameLine, Editor, EditorEvent, ExcerptRange, MultiBuffer, RowHighlightOptions,
    ToPoint,
};
use futures::SinkExt;
use gpui::{
    AnyView, App, AppContext, Entity, EventEmitter, Focusable, IntoElement, Render, SharedString,
    Subscription, Task, WeakEntity, Window,
};
use language::{Capability, Point};
use project::{Project, ProjectPath};
use ui::{ActiveTheme as _, Context, ParentElement as _, Styled as _, div};
use util::ResultExt as _;
use workspace::{
    Item, ItemHandle as _, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, ItemEvent},
    searchable::SearchableItemHandle,
};

use crate::session::{DebugSession, running::stack_frame_list::StackFrameList};
use anyhow::Result;

pub(crate) struct StackFrameViewer {
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    active_session: Option<Entity<DebugSession>>,
    selected_stack_frame_id: Option<StackFrameId>,
    refresh_task: Option<Task<Result<()>>>,
    _subscription: Option<Subscription>,
}

impl StackFrameViewer {
    pub(crate) fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor
        });

        Self {
            editor,
            multibuffer,
            workspace,
            project,
            active_session: None,
            selected_stack_frame_id: None,
            refresh_task: None,
            _subscription: None,
        }
    }

    pub(crate) fn set_active_session(
        &mut self,
        session: Entity<DebugSession>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let stack_frame_list = session
            .read(cx)
            .running_state()
            .read(cx)
            .stack_frame_list()
            .clone();

        let stack_frame_id = stack_frame_list.read(cx).selected_stack_frame_id();
        let subscription = cx.subscribe_in(
            &stack_frame_list,
            window,
            |this, stack_frame_list, _, window, cx| {
                this.selected_stack_frame_id = stack_frame_list.read(cx).selected_stack_frame_id();
                this.update_excerpts(window, cx);
            },
        );

        self._subscription = Some(subscription);
        self.active_session = Some(session);
        self.selected_stack_frame_id = stack_frame_id;
        self.update_excerpts(window, cx);
    }

    fn update_excerpts(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.refresh_task.take();
        self.editor.update(cx, |editor, cx| {
            editor.clear_highlights::<DebugStackFrameLine>(cx)
        });

        let Some(session) = self.active_session.clone() else {
            self.multibuffer.update(cx, |buffer, cx| buffer.clear(cx));
            return;
        };

        let Some(thread_id) = session
            .read(cx)
            .running_state()
            .read(cx)
            .selected_thread_id()
        else {
            self.multibuffer.update(cx, |buffer, cx| buffer.clear(cx));
            return;
        };

        let mut stack_frames = session.update(cx, |session, cx| {
            session.running_state().update(cx, |state, cx| {
                state
                    .session()
                    .update(cx, |session, cx| session.stack_frames(thread_id, cx))
            })
        });

        if let Some(idx) = self.selected_stack_frame_id.and_then(|id| {
            stack_frames.iter().enumerate().find_map(|(idx, frame)| {
                if frame.dap.id == id { Some(idx) } else { None }
            })
        }) {
            stack_frames.drain(0..idx);
        }

        let frames_to_open: Vec<_> = stack_frames
            .into_iter()
            .filter_map(|frame| {
                Some((
                    frame.dap.line as u32 - 1,
                    StackFrameList::abs_path_from_stack_frame(&frame.dap)?,
                ))
            })
            .collect();

        self.multibuffer
            .update(cx, |multi_buffer, cx| multi_buffer.clear(cx));

        let task = cx.spawn_in(window, async move |this, cx| {
            let mut to_highlights = Vec::default();

            for (line, abs_path) in frames_to_open {
                let (worktree, relative_path) = this
                    .update(cx, |this, cx| {
                        this.workspace.update(cx, |workspace, cx| {
                            workspace.project().update(cx, |this, cx| {
                                this.find_or_create_worktree(&abs_path, false, cx)
                            })
                        })
                    })??
                    .await?;

                let project_path = ProjectPath {
                    worktree_id: worktree.read_with(cx, |tree, _| tree.id())?,
                    path: relative_path.into(),
                };

                if let Some(buffer) = this
                    .read_with(cx, |this, _| this.project.clone())?
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                    .await
                    .log_err()
                {
                    this.update(cx, |this, cx| {
                        this.multibuffer.update(cx, |multi_buffer, cx| {
                            let line_point = Point::new(line, 0);

                            let range = ExcerptRange {
                                context: Point::new(line.saturating_sub(4), 0)
                                    ..Point::new(line.saturating_add(4), 0),
                                primary: line_point..line_point,
                            };
                            multi_buffer.push_excerpts(buffer.clone(), vec![range], cx);

                            let line_anchor =
                                multi_buffer.buffer_point_to_anchor(&buffer, line_point, cx);

                            if let Some(line_anchor) = line_anchor {
                                to_highlights.push(line_anchor);
                            }
                        });
                    })
                    .ok();
                }
            }

            this.update_in(cx, |this, window, cx| {
                this.editor.update(cx, |editor, cx| {
                    let snapshot = editor.snapshot(window, cx).display_snapshot;
                    let color = cx
                        .theme()
                        .colors()
                        .editor_debugger_active_line_background
                        .opacity(0.5);

                    for highlight in to_highlights.iter().skip(1) {
                        let position = highlight.to_point(&snapshot.buffer_snapshot);

                        let start = snapshot
                            .buffer_snapshot
                            .clip_point(Point::new(position.row, 0), Bias::Left);
                        let end = start + Point::new(1, 0);
                        let start = snapshot.buffer_snapshot.anchor_before(start);
                        let end = snapshot.buffer_snapshot.anchor_before(end);
                        editor.highlight_rows::<DebugStackFrameLine>(
                            start..end,
                            color,
                            RowHighlightOptions::default(),
                            cx,
                        );
                    }
                })
            })
            .ok();

            anyhow::Ok(())
        });

        self.refresh_task = Some(task);
    }
}

impl Render for StackFrameViewer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.editor.clone())
    }
}

impl EventEmitter<EditorEvent> for StackFrameViewer {}
impl Focusable for StackFrameViewer {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for StackFrameViewer {
    type Event = EditorEvent;

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
        Some("Stack Frame Viewer".into())
    }

    fn tab_content_text(&self, _detail: usize, _: &App) -> SharedString {
        "Stack Frames".into()
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
        None
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).is_dirty(cx)
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.multibuffer.read(cx).has_deleted_file(cx)
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

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
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
