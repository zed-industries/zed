use crate::GitState;
use gpui::*;
use picker::{Picker, PickerDelegate};
use project::Project;
use std::sync::Arc;
use ui::{prelude::*, Button, ListItem, ListItemSpacing, Tooltip};

actions!(repo_selector, [Confirm]);

pub struct RepoSelector {
    picker: View<Picker<RepoSelectorDelegate>>,
    project: Model<Project>,
    git_state: Model<GitState>,
}

impl RepoSelector {
    pub fn new(
        project: Model<Project>,
        git_state: Model<GitState>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = RepoSelectorDelegate {
            project: project.clone(),
            git_state: git_state.clone(),
            repositories: Vec::new(),
            selected_index: 0,
        };
        let picker =
            cx.new_view(|cx| Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into())));

        Self {
            picker,
            project,
            git_state,
        }
    }
}

impl FocusableView for RepoSelector {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for RepoSelector {}

impl Render for RepoSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child(self.picker.clone())
    }
}

pub struct RepoSelectorDelegate {
    project: Model<Project>,
    git_state: Model<GitState>,
    repositories: Vec<SharedString>,
    selected_index: usize,
}

impl PickerDelegate for RepoSelectorDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.repositories.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        let worktree = self.project.read(cx).worktrees(cx).nth(ix);

        if let Some(worktree) = worktree.clone() {
            let worktree_id = worktree.read(cx).id();
            if let Some((repo, git_repo)) =
                crate::first_worktree_repository(&self.project, worktree_id, cx)
            {
                self.git_state.update(cx, |state, _| {
                    state.activate_repository(worktree_id, repo, git_repo);
                });
            }
        }
    }

    fn placeholder_text(&self, cx: &mut WindowContext) -> Arc<str> {
        "Select a repository...".into()
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        self.project.read(cx).worktrees(cx).nth(ix).map(|worktree| {
            let root_name = worktree.read(cx).root_name().to_string();
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .child(root_name)
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        Task::ready(())
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        cx.emit(DismissEvent);
    }
}

#[derive(IntoElement)]
pub struct RepoSelectorTrigger {
    git_state: Model<GitState>,
    cursor_style: CursorStyle,
    selected: bool,
}

impl RepoSelectorTrigger {
    pub fn new(git_state: Model<GitState>) -> Self {
        Self {
            git_state,
            cursor_style: CursorStyle::PointingHand,
            selected: false,
        }
    }
}

impl Clickable for RepoSelectorTrigger {
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        // TODO:
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl Toggleable for RepoSelectorTrigger {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for RepoSelectorTrigger {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let active_repo = self.git_state.read(cx).active_repository();

        let repo_name = active_repo
            .and_then(|(_, repo, _)| {
                // Extract the repository name from the work_directory path
                repo.work_directory
                    .path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(String::from)
            })
            .unwrap_or_else(|| "No repository".to_string());

        Button::new("repo-selector", repo_name)
            .icon(IconName::GitBranch)
            .tooltip(|cx| Tooltip::text("Select repository", cx))
    }
}
