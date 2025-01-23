use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use fuzzy::PathMatch;
use gpui::{AppContext, DismissEvent, FocusHandle, FocusableView, Task, View, WeakModel, WeakView};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, ProjectPath, WorktreeId};
use ui::{prelude::*, ListItem};
use util::ResultExt as _;
use workspace::{notifications::NotifyResultExt, Workspace};

use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::ContextStore;

pub struct DirectoryContextPicker {
    picker: View<Picker<DirectoryContextPickerDelegate>>,
}

impl DirectoryContextPicker {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        context_store: WeakModel<ContextStore>,
        confirm_behavior: ConfirmBehavior,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = DirectoryContextPickerDelegate::new(
            context_picker,
            workspace,
            context_store,
            confirm_behavior,
        );
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));

        Self { picker }
    }
}

impl FocusableView for DirectoryContextPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for DirectoryContextPicker {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

pub struct DirectoryContextPickerDelegate {
    context_picker: WeakView<ContextPicker>,
    workspace: WeakView<Workspace>,
    context_store: WeakModel<ContextStore>,
    confirm_behavior: ConfirmBehavior,
    matches: Vec<PathMatch>,
    selected_index: usize,
}

impl DirectoryContextPickerDelegate {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        context_store: WeakModel<ContextStore>,
        confirm_behavior: ConfirmBehavior,
    ) -> Self {
        Self {
            context_picker,
            workspace,
            context_store,
            confirm_behavior,
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
            let directory_matches = project.worktrees(cx).flat_map(|worktree| {
                let worktree = worktree.read(cx);
                let path_prefix: Arc<str> = worktree.root_name().into();
                worktree.directories(false, 0).map(move |entry| PathMatch {
                    score: 0.,
                    positions: Vec::new(),
                    worktree_id: worktree.id().to_usize(),
                    path: entry.path.clone(),
                    path_prefix: path_prefix.clone(),
                    distance_to_relative_ancestor: 0,
                    is_dir: true,
                })
            });

            Task::ready(directory_matches.collect())
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
                        candidates: project::Candidates::Directories,
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

impl PickerDelegate for DirectoryContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search folders…".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Task::ready(());
        };

        let search_task = self.search(query, Arc::<AtomicBool>::default(), &workspace, cx);

        cx.spawn(|this, mut cx| async move {
            let mut paths = search_task.await;
            let empty_path = Path::new("");
            paths.retain(|path_match| path_match.path.as_ref() != empty_path);

            this.update(&mut cx, |this, _cx| {
                this.delegate.matches = paths;
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };

        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(mat.worktree_id),
            path: mat.path.clone(),
        };

        let Some(task) = self
            .context_store
            .update(cx, |context_store, cx| {
                context_store.add_directory(project_path, cx)
            })
            .ok()
        else {
            return;
        };

        let confirm_behavior = self.confirm_behavior;
        cx.spawn(|this, mut cx| async move {
            match task.await.notify_async_err(&mut cx) {
                None => anyhow::Ok(()),
                Some(()) => this.update(&mut cx, |this, cx| match confirm_behavior {
                    ConfirmBehavior::KeepOpen => {}
                    ConfirmBehavior::Close => this.delegate.dismissed(cx),
                }),
            }
        })
        .detach_and_log_err(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.context_picker
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let path_match = &self.matches[ix];
        let directory_name = path_match.path.to_string_lossy().to_string();

        let added = self.context_store.upgrade().map_or(false, |context_store| {
            context_store
                .read(cx)
                .includes_directory(&path_match.path)
                .is_some()
        });

        Some(
            ListItem::new(ix)
                .inset(true)
                .toggle_state(selected)
                .child(h_flex().gap_2().child(Label::new(directory_name)))
                .when(added, |el| {
                    el.end_slot(
                        h_flex()
                            .gap_1()
                            .child(
                                Icon::new(IconName::Check)
                                    .size(IconSize::Small)
                                    .color(Color::Success),
                            )
                            .child(Label::new("Added").size(LabelSize::Small)),
                    )
                }),
        )
    }
}
