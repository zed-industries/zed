use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use fuzzy::PathMatch;
use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, ManagedView, Task, View,
    WeakView,
};
use picker::{Picker, PickerDelegate};
use project::PathMatchCandidateSet;
use ui::{prelude::*, ListItem, ListItemSpacing};
use util::ResultExt as _;
use workspace::Workspace;

use crate::context::ContextKind;
use crate::message_editor::MessageEditor;

pub struct FileContextPicker {
    picker: View<Picker<FileContextPickerDelegate>>,
}

impl FileContextPicker {
    pub fn new(
        workspace: WeakView<Workspace>,
        message_editor: WeakView<MessageEditor>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = FileContextPickerDelegate::new(workspace, message_editor, cx);
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));

        Self { picker }
    }
}

impl FocusableView for FileContextPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for FileContextPicker {}

impl Render for FileContextPicker {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

pub struct FileContextPickerDelegate {
    workspace: WeakView<Workspace>,
    message_editor: WeakView<MessageEditor>,
    matches: Vec<PathMatch>,
    selected_index: usize,
}

impl FileContextPickerDelegate {
    pub fn new(
        workspace: WeakView<Workspace>,
        message_editor: WeakView<MessageEditor>,
        cx: &mut ViewContext<FileContextPicker>,
    ) -> Self {
        Self {
            workspace,
            message_editor,
            matches: Vec::new(),
            selected_index: 0,
        }
    }

    fn search(
        &mut self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: &View<Workspace>,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Task<Vec<PathMatch>> {
        if query.is_empty() {
            let workspace = workspace.read(cx);
            let project = workspace.project().read(cx);
            let entries = workspace.recent_navigation_history(Some(10), cx);

            let entries = entries
                .into_iter()
                .map(|entries| (entries.0, false))
                .chain(project.worktrees(cx).flat_map(|worktree| {
                    let worktree = worktree.read(cx);
                    let id = worktree.id();
                    worktree.child_entries(Path::new("")).map(move |entry| {
                        (
                            project::ProjectPath {
                                worktree_id: id,
                                path: entry.path.clone(),
                            },
                            entry.kind.is_dir(),
                        )
                    })
                }))
                .collect::<Vec<_>>();

            let path_prefix: Arc<str> = Arc::default();
            Task::ready(
                entries
                    .into_iter()
                    .filter_map(|(entry, is_dir)| {
                        let worktree = project.worktree_for_id(entry.worktree_id, cx)?;
                        let mut full_path = PathBuf::from(worktree.read(cx).root_name());
                        full_path.push(&entry.path);
                        Some(PathMatch {
                            score: 0.,
                            positions: Vec::new(),
                            worktree_id: entry.worktree_id.to_usize(),
                            path: full_path.into(),
                            path_prefix: path_prefix.clone(),
                            distance_to_relative_ancestor: 0,
                            is_dir,
                        })
                    })
                    .collect(),
            )
        } else {
            let worktrees = workspace.read(cx).visible_worktrees(cx).collect::<Vec<_>>();
            let candidate_sets = worktrees
                .into_iter()
                .map(|worktree| {
                    let worktree = worktree.read(cx);

                    PathMatchCandidateSet {
                        snapshot: worktree.snapshot(),
                        include_ignored: worktree
                            .root_entry()
                            .map_or(false, |entry| entry.is_ignored),
                        include_root_name: true,
                        candidates: project::Candidates::Entries,
                    }
                })
                .collect::<Vec<_>>();

            let executor = cx.background_executor().clone();
            cx.foreground_executor().spawn(async move {
                fuzzy::match_path_sets(
                    candidate_sets.as_slice(),
                    query.as_str(),
                    None,
                    false,
                    100,
                    &cancellation_flag,
                    executor,
                )
                .await
            })
        }
    }
}

impl PickerDelegate for FileContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search files…".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(());
        };

        let search_task = self.search(query, Arc::<AtomicBool>::default(), &workspace, cx);

        cx.spawn(|this, mut cx| async move {
            // TODO: This should be probably be run in the background.
            let paths = search_task.await;

            this.update(&mut cx, |this, cx| {
                this.delegate.matches = paths;
            })
            .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        self.message_editor.update(cx, |message_editor, cx| {
            let mat = &self.matches[self.selected_index];

            message_editor.insert_context(
                ContextKind::File,
                mat.path.to_string_lossy().to_string(),
                "",
            );
        });
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(mat.path.to_string_lossy().to_string()),
        )
    }
}
