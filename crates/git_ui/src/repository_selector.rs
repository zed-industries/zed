use crate::git_status_icon;
use git::status::{FileStatus, StatusCode, TrackedStatus, UnmergedStatus, UnmergedStatusCode};
use gpui::{App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity};
use itertools::Itertools;
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::{Project, git_store::Repository};
use std::sync::Arc;
use ui::{ListItem, ListItemSpacing, prelude::*};
use workspace::{ModalView, Workspace};

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(open);
}

pub fn open(
    workspace: &mut Workspace,
    _: &zed_actions::git::SelectRepo,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().clone();
    let workspace_handle = workspace.weak_handle();
    workspace.toggle_modal(window, cx, |window, cx| {
        RepositorySelector::new(project, workspace_handle, rems(34.), window, cx)
    })
}

pub struct RepositorySelector {
    width: Rems,
    picker: Entity<Picker<RepositorySelectorDelegate>>,
}

impl RepositorySelector {
    pub fn new(
        project_handle: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let git_store = project_handle.read(cx).git_store().clone();
        let repository_entries = git_store.update(cx, |git_store, _cx| {
            let mut repos: Vec<_> = git_store.repositories().values().cloned().collect();

            repos.sort_by(|a, b| {
                repository_label(&a.read(_cx))
                    .to_lowercase()
                    .cmp(&repository_label(&b.read(_cx)).to_lowercase())
            });

            repos
        });
        let filtered_repositories = repository_entries.clone();

        let widest_item_ix = repository_entries.iter().position_max_by(|a, b| {
            repository_label(&a.read(cx))
                .len()
                .cmp(&repository_label(&b.read(cx)).len())
        });

        let active_repository = git_store.read(cx).active_repository();
        let selected_index = active_repository
            .as_ref()
            .and_then(|active| filtered_repositories.iter().position(|repo| repo == active))
            .unwrap_or(0);
        let delegate = RepositorySelectorDelegate {
            repository_selector: cx.entity().downgrade(),
            workspace,
            repository_entries,
            filtered_repositories,
            active_repository,
            selected_index,
        };

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .widest_item(widest_item_ix)
                .max_height(Some(rems(20.).into()))
                .show_scrollbar(true)
        });

        RepositorySelector { picker, width }
    }
}

//pub(crate) fn filtered_repository_entries(
//    git_store: &GitStore,
//    cx: &App,
//) -> Vec<Entity<Repository>> {
//    let repositories = git_store
//        .repositories()
//        .values()
//        .sorted_by_key(|repo| {
//            let repo = repo.read(cx);
//            (
//                repo.dot_git_abs_path.clone(),
//                repo.worktree_abs_path.clone(),
//            )
//        })
//        .collect::<Vec<&Entity<Repository>>>();
//
//    repositories
//        .chunk_by(|a, b| a.read(cx).dot_git_abs_path == b.read(cx).dot_git_abs_path)
//        .flat_map(|chunk| {
//            let has_non_single_file_worktree = chunk
//                .iter()
//                .any(|repo| !repo.read(cx).is_from_single_file_worktree);
//            chunk.iter().filter(move |repo| {
//                // Remove any entry that comes from a single file worktree and represents a repository that is also represented by a non-single-file worktree.
//                !repo.read(cx).is_from_single_file_worktree || !has_non_single_file_worktree
//            })
//        })
//        .map(|&repo| repo.clone())
//        .collect()
//}

impl EventEmitter<DismissEvent> for RepositorySelector {}

impl Focusable for RepositorySelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RepositorySelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("GitRepositorySelector")
            .w(self.width)
            .child(self.picker.clone())
    }
}

impl ModalView for RepositorySelector {}

pub struct RepositorySelectorDelegate {
    repository_selector: WeakEntity<RepositorySelector>,
    workspace: WeakEntity<Workspace>,
    repository_entries: Vec<Entity<Repository>>,
    filtered_repositories: Vec<Entity<Repository>>,
    active_repository: Option<Entity<Repository>>,
    selected_index: usize,
}

impl RepositorySelectorDelegate {
    pub fn update_repository_entries(&mut self, all_repositories: Vec<Entity<Repository>>) {
        self.repository_entries = all_repositories.clone();
        self.filtered_repositories = all_repositories;
        self.selected_index = self
            .active_repository
            .as_ref()
            .and_then(|active| {
                self.filtered_repositories
                    .iter()
                    .position(|repo| repo == active)
            })
            .unwrap_or(0);
    }
}

impl PickerDelegate for RepositorySelectorDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.filtered_repositories.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix.min(self.filtered_repositories.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a repository...".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::End
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let all_repositories = self.repository_entries.clone();

        let repo_names: Vec<(Entity<Repository>, String)> = all_repositories
            .iter()
            .map(|repo| {
                (
                    repo.clone(),
                    repository_match_text(&repo.read(cx)).to_lowercase(),
                )
            })
            .collect();

        cx.spawn_in(window, async move |this, cx| {
            let filtered_repositories = cx
                .background_spawn(async move {
                    if query.is_empty() {
                        all_repositories
                    } else {
                        let query_lower = query.to_lowercase();
                        repo_names
                            .into_iter()
                            .filter(|(_, display_name)| display_name.contains(&query_lower))
                            .map(|(repo, _)| repo)
                            .collect()
                    }
                })
                .await;

            this.update_in(cx, |this, window, cx| {
                let mut sorted_repositories = filtered_repositories;
                sorted_repositories.sort_by(|a, b| {
                    repository_label(&a.read(cx))
                        .to_lowercase()
                        .cmp(&repository_label(&b.read(cx)).to_lowercase())
                });
                let selected_index = this
                    .delegate
                    .active_repository
                    .as_ref()
                    .and_then(|active| sorted_repositories.iter().position(|repo| repo == active))
                    .unwrap_or(0);
                this.delegate.filtered_repositories = sorted_repositories;
                this.delegate.set_selected_index(selected_index, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_repo) = self.filtered_repositories.get(self.selected_index) else {
            return;
        };
        let selected_repo_path = selected_repo.read(cx).work_directory_abs_path.clone();
        selected_repo.update(cx, |selected_repo, cx| {
            selected_repo.set_as_active_repository(cx)
        });
        self.workspace
            .update(cx, |workspace, cx| {
                let project = workspace.project();
                let active_worktree_id = project
                    .read(cx)
                    .visible_worktrees(cx)
                    .find(|worktree| {
                        let worktree_path = worktree.read(cx).abs_path();
                        worktree_path.as_ref() == selected_repo_path.as_ref()
                            || worktree_path.starts_with(selected_repo_path.as_ref())
                    })
                    .map(|worktree| worktree.read(cx).id());

                workspace.set_active_worktree_override_and_serialize(
                    active_worktree_id,
                    window,
                    cx,
                );
            })
            .ok();
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.repository_selector
            .update(cx, |_this, cx| cx.emit(DismissEvent))
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let repo_info = self.filtered_repositories.get(ix)?;
        let repo = repo_info.read(cx);
        let display_name = repository_label(&repo);
        let sublabel = repo.work_directory_abs_path.to_string_lossy().to_string();
        let summary = repo.status_summary();
        let is_active = self
            .active_repository
            .as_ref()
            .is_some_and(|active| active == repo_info);

        let mut item = ListItem::new(ix)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .child(
                v_flex()
                    .gap_0p5()
                    .child(h_flex().gap_1().child(Label::new(display_name)).when(
                        is_active,
                        |this| {
                            this.child(
                                Icon::new(IconName::Check)
                                    .size(IconSize::Small)
                                    .color(Color::Accent),
                            )
                        },
                    ))
                    .child(
                        Label::new(sublabel)
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .truncate(),
                    ),
            );

        if summary.count > 0 {
            let status = if summary.conflict > 0 {
                FileStatus::Unmerged(UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                })
            } else if summary.worktree.deleted > 0 || summary.index.deleted > 0 {
                FileStatus::Tracked(TrackedStatus {
                    index_status: StatusCode::Deleted,
                    worktree_status: StatusCode::Unmodified,
                })
            } else if summary.worktree.modified > 0 || summary.index.modified > 0 {
                FileStatus::Tracked(TrackedStatus {
                    index_status: StatusCode::Modified,
                    worktree_status: StatusCode::Unmodified,
                })
            } else {
                FileStatus::Tracked(TrackedStatus {
                    index_status: StatusCode::Added,
                    worktree_status: StatusCode::Unmodified,
                })
            };
            item = item.end_slot(div().pr_2().child(git_status_icon(status)));
        }

        Some(item)
    }
}

fn repository_label(repo: &Repository) -> String {
    let original_name = repo
        .original_repo_abs_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let worktree_name = repo.display_name();

    if repo.original_repo_abs_path == repo.work_directory_abs_path {
        worktree_name.to_string()
    } else if let Some(branch_name) = repo.branch.as_ref().map(|branch| branch.name()) {
        format!("{original_name} / {branch_name}")
    } else {
        format!("{original_name} / {worktree_name}")
    }
}

fn repository_match_text(repo: &Repository) -> String {
    let label = repository_label(repo);
    let path = repo.work_directory_abs_path.to_string_lossy();
    format!("{label} {path}")
}
