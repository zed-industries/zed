use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::anyhow;
use fuzzy::PathMatch;
use gpui::{AppContext, DismissEvent, FocusHandle, FocusableView, Task, View, WeakModel, WeakView};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, ProjectPath, Worktree, WorktreeId};
use ui::{prelude::*, ListItem};
use util::ResultExt as _;
use workspace::Workspace;

use crate::context_picker::{ConfirmBehavior, ContextPicker};
use crate::context_store::{push_fenced_codeblock, ContextStore};

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

        let workspace = self.workspace.clone();
        let Some(project) = workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().clone())
        else {
            return;
        };
        let path = mat.path.clone();

        if self
            .context_store
            .update(cx, |context_store, _cx| {
                if let Some(context_id) = context_store.included_directory(&path) {
                    context_store.remove_context(&context_id);
                    true
                } else {
                    false
                }
            })
            .unwrap_or(true)
        {
            return;
        }

        let worktree_id = WorktreeId::from_usize(mat.worktree_id);
        let confirm_behavior = self.confirm_behavior;
        cx.spawn(|this, mut cx| async move {
            let worktree = project.update(&mut cx, |project, cx| {
                project
                    .worktree_for_id(worktree_id, cx)
                    .ok_or_else(|| anyhow!("no worktree found for {worktree_id:?}"))
            })??;

            let files = worktree.update(&mut cx, |worktree, _cx| {
                collect_files_in_path(worktree, &path)
            })?;

            let open_buffer_tasks = project.update(&mut cx, |project, cx| {
                files
                    .into_iter()
                    .map(|file_path| {
                        project.open_buffer(
                            ProjectPath {
                                worktree_id,
                                path: file_path.clone(),
                            },
                            cx,
                        )
                    })
                    .collect::<Vec<_>>()
            })?;

            let open_all_buffers_tasks = cx.background_executor().spawn(async move {
                let mut buffers = Vec::with_capacity(open_buffer_tasks.len());

                for open_buffer_task in open_buffer_tasks {
                    let buffer = open_buffer_task.await?;

                    buffers.push(buffer);
                }

                anyhow::Ok(buffers)
            });

            let buffers = open_all_buffers_tasks.await?;

            this.update(&mut cx, |this, cx| {
                let mut text = String::new();

                for buffer in buffers {
                    let buffer = buffer.read(cx);
                    let path = buffer.file().map_or(&path, |file| file.path());
                    push_fenced_codeblock(&path, buffer.text(), &mut text);
                }

                this.delegate
                    .context_store
                    .update(cx, |context_store, _cx| {
                        context_store.insert_directory(&path, text);
                    })?;

                match confirm_behavior {
                    ConfirmBehavior::KeepOpen => {}
                    ConfirmBehavior::Close => this.delegate.dismissed(cx),
                }

                anyhow::Ok(())
            })??;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx)
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.context_picker
            .update(cx, |this, cx| {
                this.reset_mode();
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
                .included_directory(&path_match.path)
                .is_some()
        });

        Some(
            ListItem::new(ix)
                .inset(true)
                .toggle_state(selected)
                .child(h_flex().gap_2().child(Label::new(directory_name)))
                .when(added, |el| {
                    el.end_slot(Label::new("Added").size(LabelSize::XSmall))
                }),
        )
    }
}

fn collect_files_in_path(worktree: &Worktree, path: &Path) -> Vec<Arc<Path>> {
    let mut files = Vec::new();

    for entry in worktree.child_entries(path) {
        if entry.is_dir() {
            files.extend(collect_files_in_path(worktree, &entry.path));
        } else if entry.is_file() {
            files.push(entry.path.clone());
        }
    }

    files
}
