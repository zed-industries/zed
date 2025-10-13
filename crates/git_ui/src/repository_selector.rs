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
    workspace.toggle_modal(window, cx, |window, cx| {
        RepositorySelector::new(project, rems(34.), window, cx)
    })
}

pub struct RepositorySelector {
    width: Rems,
    picker: Entity<Picker<RepositorySelectorDelegate>>,
}

impl RepositorySelector {
    pub fn new(
        project_handle: Entity<Project>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let git_store = project_handle.read(cx).git_store().clone();
        let repository_entries = git_store.update(cx, |git_store, _cx| {
            let mut repos: Vec<_> = git_store.repositories().values().cloned().collect();

            repos.sort_by_key(|a| a.read(_cx).display_name());

            repos
        });
        let filtered_repositories = repository_entries.clone();

        let widest_item_ix = repository_entries.iter().position_max_by(|a, b| {
            a.read(cx)
                .display_name()
                .len()
                .cmp(&b.read(cx).display_name().len())
        });

        let delegate = RepositorySelectorDelegate {
            repository_selector: cx.entity().downgrade(),
            repository_entries,
            filtered_repositories,
            selected_index: 0,
        };

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .widest_item(widest_item_ix)
                .max_height(Some(rems(20.).into()))
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
    repository_entries: Vec<Entity<Repository>>,
    filtered_repositories: Vec<Entity<Repository>>,
    selected_index: usize,
}

impl RepositorySelectorDelegate {
    pub fn update_repository_entries(&mut self, all_repositories: Vec<Entity<Repository>>) {
        self.repository_entries = all_repositories.clone();
        self.filtered_repositories = all_repositories;
        self.selected_index = 0;
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
            .map(|repo| (repo.clone(), repo.read(cx).display_name().to_lowercase()))
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
                sorted_repositories.sort_by_key(|a| a.read(cx).display_name());
                this.delegate.filtered_repositories = sorted_repositories;
                this.delegate.set_selected_index(0, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_repo) = self.filtered_repositories.get(self.selected_index) else {
            return;
        };
        selected_repo.update(cx, |selected_repo, cx| {
            selected_repo.set_as_active_repository(cx)
        });
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
        let display_name = repo_info.read(cx).display_name();
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(display_name)),
        )
    }
}
