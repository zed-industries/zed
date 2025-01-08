use ::settings::Settings;
use collections::HashSet;
use git::repository::{GitFileStatus, RepoPath};
use gpui::{actions, AppContext, Context, Global, Hsla, Model};
use project::ProjectEntryId;
use settings::GitPanelSettings;
use ui::{Color, Icon, IconName, IntoElement, SharedString};

pub mod git_panel;
mod settings;

actions!(
    git_ui,
    [
        StageAll,
        UnstageAll,
        RevertAll,
        CommitStagedChanges,
        CommitAllChanges,
        ClearMessage
    ]
);

pub fn init(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
    let git_state = cx.new_model(|_cx| GitState::new());
    cx.set_global(GlobalGitState(git_state));
}

struct GlobalGitState(Model<GitState>);

impl Global for GlobalGitState {}

pub struct GitState {
    /// The current commit message being composed.
    commit_message: Option<SharedString>,

    /// The ProjectEntryId of the currently selected git repository's work directory.
    /// This uniquely identifies a directory entry in a worktree that contains the root
    /// of a git repository.
    ///
    /// When a git repository is selected, this ID is used to track which repository's changes
    /// are currently being viewed or modified in the UI.
    active_repository: Option<ProjectEntryId>,

    /// The set of staged entries in the current git repository.
    /// Each entry is identified by its [`ProjectEntryId`] and [`RepoPath`].
    staged_entries: HashSet<(ProjectEntryId, RepoPath)>,
}

impl GitState {
    pub fn new() -> Self {
        GitState {
            commit_message: None,
            active_repository: None,
            staged_entries: HashSet::default(),
        }
    }

    pub fn get_global(cx: &mut AppContext) -> Model<GitState> {
        cx.global::<GlobalGitState>().0.clone()
    }

    pub fn activate_repository(&mut self, active_repository: ProjectEntryId) {
        self.active_repository = Some(active_repository);
    }

    pub fn active_repository(&self) -> Option<ProjectEntryId> {
        self.active_repository
    }

    pub fn commit_message(&mut self, message: Option<SharedString>) {
        self.commit_message = message;
    }

    pub fn clear_commit_message(&mut self) {
        self.commit_message = None;
    }

    pub fn stage_entry(&mut self, repo_path: RepoPath) {
        if let Some(active_repository) = self.active_repository {
            self.staged_entries.insert((active_repository, repo_path));
        }
    }

    pub fn unstage_entry(&mut self, repo_path: RepoPath) {
        if let Some(active_repository) = self.active_repository {
            self.staged_entries.remove(&(active_repository, repo_path));
        }
    }

    pub fn stage_entries(&mut self, entries: Vec<RepoPath>) {
        if let Some(active_repository) = self.active_repository {
            self.staged_entries
                .extend(entries.into_iter().map(|path| (active_repository, path)));
        }
    }

    pub fn unstage_all_entries(&mut self) {
        if let Some(active_repository) = self.active_repository {
            self.staged_entries
                .retain(|(id, _)| id != &active_repository);
        }
    }

    pub fn toggle_staged_entry(&mut self, repo_path: RepoPath) {
        if self.is_staged(repo_path.clone()) {
            self.unstage_entry(repo_path);
        } else {
            self.stage_entry(repo_path);
        }
    }

    pub fn is_staged(&self, repo_path: RepoPath) -> bool {
        if let Some(active_repository) = self.active_repository {
            self.staged_entries
                .contains(&(active_repository, repo_path))
        } else {
            false
        }
    }
}

const ADDED_COLOR: Hsla = Hsla {
    h: 142. / 360.,
    s: 0.68,
    l: 0.45,
    a: 1.0,
};
const MODIFIED_COLOR: Hsla = Hsla {
    h: 48. / 360.,
    s: 0.76,
    l: 0.47,
    a: 1.0,
};
const REMOVED_COLOR: Hsla = Hsla {
    h: 355. / 360.,
    s: 0.65,
    l: 0.65,
    a: 1.0,
};

// TODO: Add updated status colors to theme
pub fn git_status_icon(status: GitFileStatus) -> impl IntoElement {
    match status {
        GitFileStatus::Added | GitFileStatus::Untracked => {
            Icon::new(IconName::SquarePlus).color(Color::Custom(ADDED_COLOR))
        }
        GitFileStatus::Modified => {
            Icon::new(IconName::SquareDot).color(Color::Custom(MODIFIED_COLOR))
        }
        GitFileStatus::Conflict => Icon::new(IconName::Warning).color(Color::Custom(REMOVED_COLOR)),
        GitFileStatus::Deleted => {
            Icon::new(IconName::SquareMinus).color(Color::Custom(REMOVED_COLOR))
        }
    }
}
