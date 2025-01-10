use ::settings::Settings;
use collections::HashMap;
use futures::{future::FusedFuture, select, FutureExt};
use git::repository::{GitFileStatus, GitRepository, RepoPath};
use gpui::{actions, AppContext, Context, Global, Hsla, Model, ModelContext};
use project::{Project, WorktreeId};
use settings::GitPanelSettings;
use std::sync::mpsc;
use std::{
    pin::{pin, Pin},
    sync::Arc,
    time::Duration,
};
use sum_tree::SumTree;
use ui::{Color, Icon, IconName, IntoElement, SharedString};
use worktree::RepositoryEntry;

pub mod git_panel;
mod settings;

const GIT_TASK_DEBOUNCE: Duration = Duration::from_millis(50);

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
    let git_state = cx.new_model(GitState::new);
    cx.set_global(GlobalGitState(git_state));
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum GitViewMode {
    #[default]
    List,
    Tree,
}

struct GlobalGitState(Model<GitState>);

impl Global for GlobalGitState {}

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

    updater_tx: mpsc::Sender<(Arc<dyn GitRepository>, Vec<RepoPath>, StatusAction)>,

    all_repositories: HashMap<WorktreeId, SumTree<RepositoryEntry>>,

    list_view_mode: GitViewMode,
}

impl GitState {
    pub fn new(cx: &mut ModelContext<'_, Self>) -> Self {
        let (updater_tx, updater_rx) = mpsc::channel();
        cx.spawn(|_, cx| async move {
            // Long-running task to periodically update git indices based on messages from the panel.

            // We read messages from the channel in batches that refer to the same repository.
            // When we read a message whose repository is different from the current batch's repository,
            // the batch is finished, and since we can't un-receive this last message, we save it
            // to begin the next batch.
            let mut leftover_message: Option<(
                Arc<dyn GitRepository>,
                Vec<RepoPath>,
                StatusAction,
            )> = None;
            let mut git_task = None;
            loop {
                let mut timer = cx.background_executor().timer(GIT_TASK_DEBOUNCE).fuse();
                let _result = {
                    let mut task: Pin<&mut dyn FusedFuture<Output = anyhow::Result<()>>> =
                        match git_task.as_mut() {
                            Some(task) => pin!(task),
                            // If no git task is running, just wait for the timeout.
                            None => pin!(std::future::pending().fuse()),
                        };
                    select! {
                        result = task => {
                            // Task finished.
                            git_task = None;
                            Some(result)
                        }
                        _ = timer => None,
                    }
                };

                // TODO handle failure of the git command

                if git_task.is_none() {
                    // No git task running now; let's see if we should launch a new one.
                    let mut to_stage = Vec::new();
                    let mut to_unstage = Vec::new();
                    let mut current_repo = leftover_message.as_ref().map(|msg| msg.0.clone());
                    for (git_repo, paths, action) in leftover_message
                        .take()
                        .into_iter()
                        .chain(updater_rx.try_iter())
                    {
                        if current_repo
                            .as_ref()
                            .map_or(false, |repo| !Arc::ptr_eq(repo, &git_repo))
                        {
                            // End of a batch, save this for the next one.
                            leftover_message = Some((git_repo.clone(), paths, action));
                            break;
                        } else if current_repo.is_none() {
                            // Start of a batch.
                            current_repo = Some(git_repo);
                        }

                        if action == StatusAction::Stage {
                            to_stage.extend(paths);
                        } else {
                            to_unstage.extend(paths);
                        }
                    }

                    // TODO handle the same path being staged and unstaged

                    if to_stage.is_empty() && to_unstage.is_empty() {
                        continue;
                    }

                    if let Some(git_repo) = current_repo {
                        git_task = Some(
                            cx.background_executor()
                                .spawn(async move { git_repo.update_index(&to_stage, &to_unstage) })
                                .fuse(),
                        );
                    }
                }
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

    pub fn get_global(cx: &mut AppContext) -> Model<GitState> {
        cx.global::<GlobalGitState>().0.clone()
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
            let _ = self
                .updater_tx
                .send((git_repo.clone(), vec![repo_path], StatusAction::Stage));
        }
    }

    pub fn unstage_entry(&mut self, repo_path: RepoPath) {
        if let Some((_, _, git_repo)) = self.active_repository.as_ref() {
            let _ =
                self.updater_tx
                    .send((git_repo.clone(), vec![repo_path], StatusAction::Unstage));
        }
    }

    pub fn stage_entries(&mut self, entries: Vec<RepoPath>) {
        if let Some((_, _, git_repo)) = self.active_repository.as_ref() {
            let _ = self
                .updater_tx
                .send((git_repo.clone(), entries, StatusAction::Stage));
        }
    }

    fn act_on_all(&mut self, action: StatusAction) {
        if let Some((_, active_repository, git_repo)) = self.active_repository.as_ref() {
            let _ = self.updater_tx.send((
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
