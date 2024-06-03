use crate::ProjectIndex;
use gpui::{
    canvas, div, list, uniform_list, AnyElement, AppContext, CursorStyle, EventEmitter,
    FocusHandle, FocusableView, IntoElement, ListOffset, ListState, Model, MouseMoveEvent, Render,
    UniformListScrollHandle, View,
};
use project::WorktreeId;
use settings::Settings;
use std::{path::Path, sync::Arc};
use theme::ThemeSettings;
use ui::prelude::*;
use workspace::item::{Item, TabContentParams};

pub struct ProjectIndexDebugView {
    index: Model<ProjectIndex>,
    rows: Vec<Row>,
    selected_path: Option<PathState>,
    hovered_row_ix: Option<usize>,
    focus_handle: FocusHandle,
    list_scroll_handle: UniformListScrollHandle,
    _subscription: gpui::Subscription,
}

struct PathState {
    path: Arc<Path>,
    chunks: Vec<SharedString>,
    list_state: ListState,
}

enum Row {
    Worktree(Arc<Path>),
    Entry(WorktreeId, Arc<Path>),
}

impl ProjectIndexDebugView {
    pub fn new(index: Model<ProjectIndex>, cx: &mut ViewContext<Self>) -> Self {
        let mut this = Self {
            rows: Vec::new(),
            list_scroll_handle: UniformListScrollHandle::new(),
            selected_path: None,
            hovered_row_ix: None,
            focus_handle: cx.focus_handle(),
            _subscription: cx.subscribe(&index, |this, _, _, cx| this.update_rows(cx)),
            index,
        };
        this.update_rows(cx);
        this
    }

    fn update_rows(&mut self, cx: &mut ViewContext<Self>) {
        let worktree_indices = self.index.read(cx).worktree_indices(cx);
        cx.spawn(|this, mut cx| async move {
            let mut rows = Vec::new();

            for index in worktree_indices {
                let (root_path, worktree_id, worktree_paths) =
                    index.read_with(&cx, |index, cx| {
                        let worktree = index.worktree.read(cx);
                        (worktree.abs_path(), worktree.id(), index.paths(cx))
                    })?;
                rows.push(Row::Worktree(root_path));
                rows.extend(
                    worktree_paths
                        .await?
                        .into_iter()
                        .map(|path| Row::Entry(worktree_id, path)),
                );
            }

            this.update(&mut cx, |this, cx| {
                this.rows = rows;
                cx.notify();
            })
        })
        .detach();
    }

    fn handle_path_click(
        &mut self,
        worktree_id: WorktreeId,
        file_path: Arc<Path>,
        cx: &mut ViewContext<Self>,
    ) -> Option<()> {
        let project_index = self.index.read(cx);
        let fs = project_index.fs.clone();
        let worktree_index = project_index.worktree_index(worktree_id, cx)?.read(cx);
        let root_path = worktree_index.worktree.read(cx).abs_path();
        let chunks = worktree_index.chunks_for_path(file_path.clone(), cx);

        cx.spawn(|this, mut cx| async move {
            let chunks = chunks.await?;
            let content = fs.load(&root_path.join(&file_path)).await?;
            let chunks = chunks
                .into_iter()
                .map(|chunk| {
                    let mut start = chunk.chunk.range.start.min(content.len());
                    let mut end = chunk.chunk.range.end.min(content.len());
                    while !content.is_char_boundary(start) {
                        start += 1;
                    }
                    while !content.is_char_boundary(end) {
                        end -= 1;
                    }
                    content[start..end].to_string().into()
                })
                .collect::<Vec<_>>();

            this.update(&mut cx, |this, cx| {
                let view = cx.view().downgrade();
                this.selected_path = Some(PathState {
                    path: file_path,
                    list_state: ListState::new(
                        chunks.len(),
                        gpui::ListAlignment::Top,
                        px(100.),
                        move |ix, cx| {
                            if let Some(view) = view.upgrade() {
                                view.update(cx, |view, cx| view.render_chunk(ix, cx))
                            } else {
                                div().into_any()
                            }
                        },
                    ),
                    chunks,
                });
                cx.notify();
            })
        })
        .detach();
        None
    }

    fn render_chunk(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let buffer_font = ThemeSettings::get_global(cx).buffer_font.clone();
        let Some(state) = &self.selected_path else {
            return div().into_any();
        };

        let colors = cx.theme().colors();
        let chunk = &state.chunks[ix];

        div()
            .text_ui(cx)
            .w_full()
            .font(buffer_font)
            .child(
                h_flex()
                    .justify_between()
                    .child(format!(
                        "chunk {} of {}. length: {}",
                        ix + 1,
                        state.chunks.len(),
                        chunk.len(),
                    ))
                    .child(
                        h_flex()
                            .child(
                                Button::new(("prev", ix), "prev")
                                    .disabled(ix == 0)
                                    .on_click(cx.listener(move |this, _, _| {
                                        this.scroll_to_chunk(ix.saturating_sub(1))
                                    })),
                            )
                            .child(
                                Button::new(("next", ix), "next")
                                    .disabled(ix + 1 == state.chunks.len())
                                    .on_click(
                                        cx.listener(move |this, _, _| this.scroll_to_chunk(ix + 1)),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .bg(colors.editor_background)
                    .text_xs()
                    .child(chunk.clone()),
            )
            .into_any_element()
    }

    fn scroll_to_chunk(&mut self, ix: usize) {
        if let Some(state) = self.selected_path.as_mut() {
            state.list_state.scroll_to(ListOffset {
                item_ix: ix,
                offset_in_item: px(0.),
            })
        }
    }
}

impl Render for ProjectIndexDebugView {
    fn render(&mut self, cx: &mut gpui::ViewContext<'_, Self>) -> impl IntoElement {
        if let Some(selected_path) = self.selected_path.as_ref() {
            v_flex()
                .child(
                    div()
                        .id("selected-path-name")
                        .child(
                            h_flex()
                                .justify_between()
                                .child(selected_path.path.to_string_lossy().to_string())
                                .child("x"),
                        )
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .cursor(CursorStyle::PointingHand)
                        .on_click(cx.listener(|this, _, cx| {
                            this.selected_path.take();
                            cx.notify();
                        })),
                )
                .child(list(selected_path.list_state.clone()).size_full())
                .size_full()
                .into_any_element()
        } else {
            let mut list = uniform_list(
                cx.view().clone(),
                "ProjectIndexDebugView",
                self.rows.len(),
                move |this, range, cx| {
                    this.rows[range]
                        .iter()
                        .enumerate()
                        .map(|(ix, row)| match row {
                            Row::Worktree(root_path) => div()
                                .id(ix)
                                .child(Label::new(root_path.to_string_lossy().to_string())),
                            Row::Entry(worktree_id, file_path) => div()
                                .id(ix)
                                .pl_8()
                                .child(Label::new(file_path.to_string_lossy().to_string()))
                                .on_mouse_move(cx.listener(move |this, _: &MouseMoveEvent, cx| {
                                    if this.hovered_row_ix != Some(ix) {
                                        this.hovered_row_ix = Some(ix);
                                        cx.notify();
                                    }
                                }))
                                .cursor(CursorStyle::PointingHand)
                                .on_click(cx.listener({
                                    let worktree_id = *worktree_id;
                                    let file_path = file_path.clone();
                                    move |this, _, cx| {
                                        this.handle_path_click(worktree_id, file_path.clone(), cx);
                                    }
                                })),
                        })
                        .collect()
                },
            )
            .track_scroll(self.list_scroll_handle.clone())
            .size_full()
            .text_bg(cx.theme().colors().background)
            .into_any_element();

            canvas(
                move |bounds, cx| {
                    list.prepaint_as_root(bounds.origin, bounds.size.into(), cx);
                    list
                },
                |_, mut list, cx| list.paint(cx),
            )
            .size_full()
            .into_any_element()
        }
    }
}

impl EventEmitter<()> for ProjectIndexDebugView {}

impl Item for ProjectIndexDebugView {
    type Event = ();

    fn tab_content(&self, params: TabContentParams, _: &WindowContext<'_>) -> AnyElement {
        Label::new("Project Index (Debug)")
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn clone_on_split(
        &self,
        _: Option<workspace::WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(|cx| Self::new(self.index.clone(), cx)))
    }
}

impl FocusableView for ProjectIndexDebugView {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
