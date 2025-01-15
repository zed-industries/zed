use ::settings::Settings;
use collections::HashMap;
use futures::channel::mpsc;
use futures::StreamExt as _;
use git::repository::{GitFileStatus, GitRepository, RepoPath};
use git_panel_settings::GitPanelSettings;
use gpui::{actions, AppContext, Hsla, Model};
use project::{Project, WorktreeId};
use std::sync::Arc;
use sum_tree::SumTree;
use ui::{Color, Icon, IconName, IntoElement, SharedString};
use util::ResultExt as _;
use worktree::RepositoryEntry;

pub mod git_panel;
mod git_panel_settings;

actions!(
    git,
    [
        StageFile,
        UnstageFile,
        ToggleStaged,
        // Revert actions are currently in the editor crate:
        // editor::RevertFile,
        // editor::RevertSelectedHunks
        StageAll,
        UnstageAll,
        RevertAll,
        CommitChanges,
        CommitAllChanges,
        ClearCommitMessage
    ]
);

pub fn init(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum GitViewMode {
    #[default]
    List,
    Tree,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StatusAction {
    Stage,
    Unstage,
}

pub struct GitState {
    /// The current commit message being composed.
    commit_message: Option<SharedString>,

    /// When a git repository is selected, this is used to track which repository's changes
    /// are currently being viewed or modified in the UI.
    active_repository: Option<(WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)>,

    updater_tx: mpsc::UnboundedSender<(Arc<dyn GitRepository>, Vec<RepoPath>, StatusAction)>,

    all_repositories: HashMap<WorktreeId, SumTree<RepositoryEntry>>,

    list_view_mode: GitViewMode,
}

impl GitState {
    pub fn new(cx: &AppContext) -> Self {
        let (updater_tx, mut updater_rx) =
            mpsc::unbounded::<(Arc<dyn GitRepository>, Vec<RepoPath>, StatusAction)>();
        cx.spawn(|cx| async move {
            while let Some((git_repo, paths, action)) = updater_rx.next().await {
                cx.background_executor()
                    .spawn(async move {
                        match action {
                            StatusAction::Stage => git_repo.stage_paths(&paths),
                            StatusAction::Unstage => git_repo.unstage_paths(&paths),
                        }
                    })
                    .await
                    .log_err();
            }
        })
        .detach();
        GitState {
            commit_message: None,
            active_repository: None,
            updater_tx,
            list_view_mode: GitViewMode::default(),
            all_repositories: HashMap::default(),
        }
    }

    pub fn activate_repository(
        &mut self,
        worktree_id: WorktreeId,
        active_repository: RepositoryEntry,
        git_repo: Arc<dyn GitRepository>,
    ) {
        self.active_repository = Some((worktree_id, active_repository, git_repo));
    }

    pub fn active_repository(
        &self,
    ) -> Option<&(WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)> {
        self.active_repository.as_ref()
    }

    pub fn commit_message(&mut self, message: Option<SharedString>) {
        self.commit_message = message;
    }

    pub fn clear_commit_message(&mut self) {
        self.commit_message = None;
    }

    pub fn stage_entry(&mut self, repo_path: RepoPath) {
        if let Some((_, _, git_repo)) = self.active_repository.as_ref() {
            let _ = self.updater_tx.unbounded_send((
                git_repo.clone(),
                vec![repo_path],
                StatusAction::Stage,
            ));
        }
    }

    pub fn unstage_entry(&mut self, repo_path: RepoPath) {
        if let Some((_, _, git_repo)) = self.active_repository.as_ref() {
            let _ = self.updater_tx.unbounded_send((
                git_repo.clone(),
                vec![repo_path],
                StatusAction::Unstage,
            ));
        }
    }

    pub fn stage_entries(&mut self, entries: Vec<RepoPath>) {
        if let Some((_, _, git_repo)) = self.active_repository.as_ref() {
            let _ =
                self.updater_tx
                    .unbounded_send((git_repo.clone(), entries, StatusAction::Stage));
        }
    }

    fn act_on_all(&mut self, action: StatusAction) {
        if let Some((_, active_repository, git_repo)) = self.active_repository.as_ref() {
            let _ = self.updater_tx.unbounded_send((
                git_repo.clone(),
                active_repository
                    .status()
                    .map(|entry| entry.repo_path)
                    .collect(),
                action,
            ));
        }
    }

    pub fn stage_all(&mut self) {
        self.act_on_all(StatusAction::Stage);
    }

    pub fn unstage_all(&mut self) {
        self.act_on_all(StatusAction::Unstage);
    }
}

pub fn first_worktree_repository(
    project: &Model<Project>,
    worktree_id: WorktreeId,
    cx: &mut AppContext,
) -> Option<(RepositoryEntry, Arc<dyn GitRepository>)> {
    project
        .read(cx)
        .worktree_for_id(worktree_id, cx)
        .and_then(|worktree| {
            let snapshot = worktree.read(cx).snapshot();
            let repo = snapshot.repositories().iter().next()?.clone();
            let git_repo = worktree
                .read(cx)
                .as_local()?
                .get_local_repo(&repo)?
                .repo()
                .clone();
            Some((repo, git_repo))
        })
}

pub fn first_repository_in_project(
    project: &Model<Project>,
    cx: &mut AppContext,
) -> Option<(WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)> {
    project.read(cx).worktrees(cx).next().and_then(|worktree| {
        let snapshot = worktree.read(cx).snapshot();
        let repo = snapshot.repositories().iter().next()?.clone();
        let git_repo = worktree
            .read(cx)
            .as_local()?
            .get_local_repo(&repo)?
            .repo()
            .clone();
        Some((snapshot.id(), repo, git_repo))
    })
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
