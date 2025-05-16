use std::any::{Any, TypeId};

use collections::HashMap;
use dap::StackFrameId;
use editor::{
    Anchor, Bias, DebugStackFrameLine, Editor, EditorEvent, ExcerptId, ExcerptRange, MultiBuffer,
    RowHighlightOptions, ToPoint, scroll::Autoscroll,
};
use gpui::{
    AnyView, App, AppContext, Entity, EventEmitter, Focusable, IntoElement, Render, SharedString,
    Subscription, Task, WeakEntity, Window,
};
use language::{BufferSnapshot, Capability, Point, Selection, SelectionGoal, TreeSitterOptions};
use project::{Project, ProjectPath};
use ui::{ActiveTheme as _, Context, ParentElement as _, Styled as _, div};
use util::ResultExt as _;
use workspace::{
    Item, ItemHandle as _, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, ItemEvent},
    searchable::SearchableItemHandle,
};

use crate::session::running::stack_frame_list::{StackFrameList, StackFrameListEvent};
use anyhow::Result;

pub(crate) struct StackTraceView {
    editor: Entity<Editor>,
    multibuffer: Entity<MultiBuffer>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    stack_frame_list: Entity<StackFrameList>,
    selected_stack_frame_id: Option<StackFrameId>,
    highlights: Vec<(StackFrameId, Anchor)>,
    excerpt_for_frames: collections::HashMap<ExcerptId, StackFrameId>,
    refresh_task: Option<Task<Result<()>>>,
    _subscription: Option<Subscription>,
}

impl StackTraceView {
    pub(crate) fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        stack_frame_list: Entity<StackFrameList>,
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

        cx.subscribe_in(&editor, window, |this, editor, event, window, cx| {
            if let EditorEvent::SelectionsChanged { local: true } = event {
                let excerpt_id = editor.update(cx, |editor, cx| {
                    let position: Point = editor
                        .selections
                        .newest(&editor.selections.display_map(cx))
                        .head();

                    editor
                        .snapshot(window, cx)
                        .buffer_snapshot
                        .excerpt_containing(position..position)
                        .map(|excerpt| excerpt.id())
                });

                if let Some(stack_frame_id) = excerpt_id
                    .and_then(|id| this.excerpt_for_frames.get(&id))
                    .filter(|id| Some(**id) != this.selected_stack_frame_id)
                {
                    this.stack_frame_list.update(cx, |list, cx| {
                        list.go_to_stack_frame(*stack_frame_id, window, cx).detach();
                    });
                }
            }
        })
        .detach();

        cx.subscribe_in(
            &stack_frame_list,
            window,
            |this, stack_frame_list, event, window, cx| match event {
                StackFrameListEvent::BuiltEntries => {
                    this.selected_stack_frame_id =
                        stack_frame_list.read(cx).opened_stack_frame_id();
                    this.update_excerpts(window, cx);
                }
                StackFrameListEvent::SelectedStackFrameChanged(selected_frame_id) => {
                    this.selected_stack_frame_id = Some(*selected_frame_id);
                    this.update_highlights(window, cx);

                    if let Some(frame_anchor) = this
                        .highlights
                        .iter()
                        .find(|(frame_id, _)| frame_id == selected_frame_id)
                        .map(|highlight| highlight.1)
                    {
                        this.editor.update(cx, |editor, cx| {
                            if frame_anchor.excerpt_id
                                != editor.selections.newest_anchor().head().excerpt_id
                            {
                                let auto_scroll =
                                    Some(Autoscroll::center().for_anchor(frame_anchor));

                                editor.change_selections(auto_scroll, window, cx, |selections| {
                                    let selection_id = selections.new_selection_id();

                                    let selection = Selection {
                                        id: selection_id,
                                        start: frame_anchor,
                                        end: frame_anchor,
                                        goal: SelectionGoal::None,
                                        reversed: false,
                                    };

                                    selections.select_anchors(vec![selection]);
                                })
                            }
                        });
                    }
                }
            },
        )
        .detach();

        let mut this = Self {
            editor,
            multibuffer,
            workspace,
            project,
            excerpt_for_frames: HashMap::default(),
            highlights: Vec::default(),
            stack_frame_list,
            selected_stack_frame_id: None,
            refresh_task: None,
            _subscription: None,
        };

        this.update_excerpts(window, cx);
        this
    }

    fn update_excerpts(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.refresh_task.take();
        self.editor.update(cx, |editor, cx| {
            editor.clear_highlights::<DebugStackFrameLine>(cx)
        });

        let stack_frames = self
            .stack_frame_list
            .update(cx, |list, _| list.flatten_entries(false));

        let frames_to_open: Vec<_> = stack_frames
            .into_iter()
            .filter_map(|frame| {
                Some((
                    frame.id,
                    frame.line as u32 - 1,
                    StackFrameList::abs_path_from_stack_frame(&frame)?,
                ))
            })
            .collect();

        self.multibuffer
            .update(cx, |multi_buffer, cx| multi_buffer.clear(cx));

        let task = cx.spawn_in(window, async move |this, cx| {
            let mut to_highlights = Vec::default();

            for (stack_frame_id, line, abs_path) in frames_to_open {
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
                            let start_context = Self::heuristic_syntactic_expand(
                                &buffer.read(cx).snapshot(),
                                line_point,
                            );

                            // Users will want to see what happened before an active debug line in most cases
                            let range = ExcerptRange {
                                context: start_context..Point::new(line.saturating_add(1), 0),
                                primary: line_point..line_point,
                            };
                            multi_buffer.push_excerpts(buffer.clone(), vec![range], cx);

                            let line_anchor =
                                multi_buffer.buffer_point_to_anchor(&buffer, line_point, cx);

                            if let Some(line_anchor) = line_anchor {
                                this.excerpt_for_frames
                                    .insert(line_anchor.excerpt_id, stack_frame_id);
                                to_highlights.push((stack_frame_id, line_anchor));
                            }
                        });
                    })
                    .ok();
                }
            }

            this.update_in(cx, |this, window, cx| {
                this.highlights = to_highlights;
                this.update_highlights(window, cx);
            })
            .ok();

            anyhow::Ok(())
        });

        self.refresh_task = Some(task);
    }

    fn update_highlights(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, _| {
            editor.clear_row_highlights::<DebugStackFrameLine>()
        });

        let stack_frames = self
            .stack_frame_list
            .update(cx, |session, _| session.flatten_entries(false));

        let active_idx = self
            .selected_stack_frame_id
            .and_then(|id| {
                stack_frames
                    .iter()
                    .enumerate()
                    .find_map(|(idx, frame)| if frame.id == id { Some(idx) } else { None })
            })
            .unwrap_or(0);

        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(window, cx).display_snapshot;
            let first_color = cx.theme().colors().editor_debugger_active_line_background;

            let color = first_color.opacity(0.5);

            let mut is_first = true;

            for (_, highlight) in self.highlights.iter().skip(active_idx) {
                let position = highlight.to_point(&snapshot.buffer_snapshot);
                let color = if is_first {
                    is_first = false;
                    first_color
                } else {
                    color
                };

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
    }

    fn heuristic_syntactic_expand(snapshot: &BufferSnapshot, selected_point: Point) -> Point {
        let mut text_objects = snapshot.text_object_ranges(
            selected_point..selected_point,
            TreeSitterOptions::max_start_depth(4),
        );

        let mut start_position = text_objects
            .find(|(_, obj)| matches!(obj, language::TextObject::AroundFunction))
            .map(|(range, _)| snapshot.offset_to_point(range.start))
            .map(|point| Point::new(point.row.max(selected_point.row.saturating_sub(8)), 0))
            .unwrap_or(selected_point);

        if start_position.row == selected_point.row {
            start_position.row = start_position.row.saturating_sub(1);
        }

        start_position
    }
}

impl Render for StackTraceView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.editor.clone())
    }
}

impl EventEmitter<EditorEvent> for StackTraceView {}
impl Focusable for StackTraceView {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for StackTraceView {
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
