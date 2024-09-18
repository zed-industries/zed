use editor::{scroll::Autoscroll, Bias, Editor};
use gpui::{
    actions, rems, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::{bookmark_store::Bookmark, Item, Project};
use std::sync::Arc;
use text::{Point, ToPoint};
use ui::{prelude::*, HighlightedLabel, ListItem};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(
    bookmarks,
    [
        SelectPrev,
        Toggle,
        JumpPrevious,
        JumpNext,
        ClearCurrentBuffer,
        ClearCurrentWorktree,
        ClearAll,
        ListCurrentBuffer,
        ListCurrentWorktree,
        ListAll,
    ]
);

impl ModalView for Bookmarks {}

pub struct Bookmarks {
    picker: View<Picker<BookmarkDelegate>>,
}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(Bookmarks::register).detach();
}

enum OperatorType {
    Buffer,
    Worktree,
    Workspace,
}

impl Bookmarks {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, _: &Toggle, cx| {
            if let Some(active_item) = workspace.active_item(cx) {
                if let Some(editor) = active_item.downcast::<Editor>() {
                    editor.update(cx, |editor, cx| editor.toggle_bookmark(cx))
                }
            }
        });
        workspace.register_action(|workspace, _: &ClearCurrentBuffer, cx| {
            Self::clear(workspace, OperatorType::Buffer, cx)
        });
        workspace.register_action(|workspace, _: &ClearCurrentWorktree, cx| {
            Self::clear(workspace, OperatorType::Worktree, cx)
        });
        workspace.register_action(|workspace, _: &ClearAll, cx| {
            Self::clear(workspace, OperatorType::Workspace, cx)
        });
        workspace.register_action(|workspace, _: &JumpPrevious, cx| {
            Self::open_bookmarks(workspace, true, cx)
        });
        workspace.register_action(|workspace, _: &JumpNext, cx| {
            Self::open_bookmarks(workspace, false, cx)
        });
        workspace.register_action(|workspace, _: &ListCurrentBuffer, cx| {
            Self::open(workspace, OperatorType::Buffer, cx)
        });
        workspace.register_action(|workspace, _: &ListCurrentWorktree, cx| {
            Self::open(workspace, OperatorType::Worktree, cx)
        });
        workspace.register_action(|workspace, _: &ListAll, cx| {
            Self::open(workspace, OperatorType::Workspace, cx)
        });
    }

    fn clear(
        workspace: &mut Workspace,
        operator_type: OperatorType,
        cx: &mut ViewContext<Workspace>,
    ) {
        let project = workspace.project().clone();
        project.update(cx, |project, cx| {
            let bookmark_store = project.bookmark_store();
            match operator_type {
                OperatorType::Buffer => {
                    if let Some(buffer_id) = workspace
                        .active_item_as::<Editor>(cx)
                        .and_then(|editor| editor.read(cx).buffer().read(cx).as_singleton())
                        .and_then(|buffer| Some(buffer.read(cx).remote_id()))
                    {
                        bookmark_store
                            .update(cx, |store, cx| store.clear_current_editor(buffer_id, cx));
                    }
                }
                OperatorType::Worktree => {
                    if let Some(project_path) = workspace
                        .active_item(cx)
                        .and_then(|item| item.project_path(cx))
                    {
                        bookmark_store.update(cx, |store, cx| {
                            store.clear_current_worktree(project_path.worktree_id, cx);
                        });
                    }
                }
                OperatorType::Workspace => bookmark_store.update(cx, |store, cx| store.clear_all()),
            }
        });
    }

    fn open_bookmarks(workspace: &mut Workspace, reverse: bool, cx: &mut ViewContext<Workspace>) {
        let project = workspace.project().clone();
        let bm = project.update(cx, |project, cx| {
            let bookmark_store = project.bookmark_store();
            bookmark_store.update(cx, |store, cx| {
                if reverse {
                    store.prev().clone()
                } else {
                    store.next().clone()
                }
            })
        });

        if let Some(bm) = bm {
            let bm = bm.clone();
            let buffer = bm.buffer.clone();
            let anchor = bm.anchor.clone();
            cx.spawn(|workspace, mut cx| async move {
                let _ = workspace.update(&mut cx, |workspace, cx| {
                    let active_panel = workspace.active_pane().clone();
                    let editor = workspace.open_project_item::<Editor>(
                        active_panel,
                        bm.buffer,
                        true,
                        true,
                        cx,
                    );
                    editor.update(cx, |editor, cx| {
                        let buffer = buffer.read(cx);
                        let cursor = language::ToPoint::to_point(&anchor, buffer);

                        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                            s.select_ranges([cursor..cursor]);
                        });
                    });
                });
            })
            .detach();
        }
    }

    fn open(
        workspace: &mut Workspace,
        operator_type: OperatorType,
        cx: &mut ViewContext<Workspace>,
    ) {
        let project = workspace.project().clone();
        let weak_workspace = cx.view().downgrade();
        let bookmark_store = project.read(cx).bookmark_store();
        let bookmarks = match operator_type {
            OperatorType::Buffer => {
                if let Some(buffer_id) = workspace
                    .active_item_as::<Editor>(cx)
                    .and_then(|editor| editor.read(cx).buffer().read(cx).as_singleton())
                    .and_then(|buffer| Some(buffer.read(cx).remote_id()))
                {
                    bookmark_store.read(cx).get_current_editor(buffer_id, cx)
                }
            }
            OperatorType::Worktree => {
                if let Some(project_path) = workspace
                    .active_item(cx)
                    .and_then(|item| item.project_path(cx))
                {
                    bookmark_store
                        .read(cx)
                        .get_current_worktree(project_path.worktree_id, cx)
                }
            }
            OperatorType::Workspace => bookmark_store.read(cx).get_all(),
        };
        workspace.toggle_modal(cx, |cx| {
            let delegate = BookmarkDelegate::new(
                cx.view().downgrade(),
                weak_workspace,
                project,
                bookmarks,
                cx,
            );

            Bookmarks::new(delegate, cx)
        });
    }

    fn new(delegate: BookmarkDelegate, cx: &mut ViewContext<Self>) -> Self {
        Self {
            picker: cx.new_view(|cx| Picker::uniform_list(delegate, cx)),
        }
    }

    fn handle_select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        cx.dispatch_action(Box::new(menu::SelectPrev));
    }
}

impl EventEmitter<DismissEvent> for Bookmarks {}

impl FocusableView for Bookmarks {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for Bookmarks {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("Bookmarks")
            .w(rems(34.))
            .on_action(cx.listener(Self::handle_select_prev))
            .child(self.picker.clone())
    }
}

pub struct BookmarkDelegate {
    bookmarks: WeakView<Bookmarks>,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    matches: Vec<Bookmark>,
    selected_index: usize,
}

impl BookmarkDelegate {
    fn new(
        bookmarks: WeakView<Bookmarks>,
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        matches: Vec<Bookmark>,
        _cx: &mut ViewContext<Bookmarks>,
    ) -> Self {
        Self {
            bookmarks,
            workspace,
            project,
            matches,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for BookmarkDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "List Bookmarks...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn update_matches(
        &mut self,
        _raw_query: String,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Task<()> {
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<BookmarkDelegate>>) {
        if let Some(m) = self.matches.get(self.selected_index()) {
            if let Some(workspace) = self.workspace.upgrade() {
                let finder = self.bookmarks.clone();
                let bm_id = m.id;
                let buffer = m.buffer.clone();

                workspace.update(cx, |workspace, cx| {
                    let active_panel = workspace.active_pane().clone();
                    let editor = workspace.open_project_item::<Editor>(
                        active_panel,
                        buffer.clone(),
                        true,
                        true,
                        cx,
                    );
                    let buffer = buffer.clone();
                    let anchor = m.anchor;
                    editor.update(cx, |editor, cx| {
                        let buffer = buffer.read(cx);
                        let cursor = language::ToPoint::to_point(&anchor, buffer);
                        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                            s.select_ranges([cursor..cursor]);
                        });
                    });
                    let project = workspace.project();
                    // project.read(cx).bookmark_store().update(cx, |store, cx| {
                    //     store.update_current_id(bm_id, cx);
                    // });
                    finder.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                })
            }
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<BookmarkDelegate>>) {
        self.bookmarks
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let bookmark_match = self
            .matches
            .get(ix)
            .expect("Invalid matches state: no element for index {ix}");

        let file_name = bookmark_match
            .buffer
            .read(cx)
            .file()?
            .file_name(cx)
            .to_string_lossy()
            .to_string();

        let annotation = bookmark_match
            .annotation
            .clone()
            .unwrap_or_else(|| "title".to_string());
        let text = format!("{}(row:{}, col: 0)", file_name, 0);
        Some(
            ListItem::new(ix).inset(true).selected(selected).child(
                v_flex()
                    .gap_0()
                    .px_px()
                    .child(
                        ListItem::new("annotation")
                            .start_slot(Icon::new(IconName::Bookmark))
                            .child(SharedString::from(annotation)),
                    )
                    .child(
                        HighlightedLabel::new(text, vec![])
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            ),
        )
    }
}
