use ::settings::Settings;
use git::repository::{GitFileStatus, RepoPath};
use gpui::{actions, AppContext, Context, Global, Hsla, Model};
use project::{Project, ProjectEntryId, WorktreeId};
use settings::GitPanelSettings;
use sum_tree::TreeMap;
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

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum GitViewMode {
    #[default]
    List,
    Tree,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GitListEntry {
    depth: usize,
    display_name: String,
    repo_path: RepoPath,
    status: GitFileStatus,
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

    /// The ProjectEntryId of the currently selected git repository's work directory.
    /// This uniquely identifies a directory entry in a worktree that contains the root
    /// of a git repository.
    ///
    /// When a git repository is selected, this ID is used to track which repository's changes
    /// are currently being viewed or modified in the UI.
    active_repository: Option<ProjectEntryId>,

    /// Task to update the actual git state.
    git_task_rx: Option<async_broadcast::Receiver<()>>,

    /// Actions that have been taken since the last task was launched,
    /// that will be flushed out when we launch the next task.
    status_actions_since_task: TreeMap<(ProjectEntryId, RepoPath), StatusAction>,

    list_view_mode: GitViewMode,
}

impl GitState {
    pub fn new() -> Self {
        GitState {
            commit_message: None,
            active_repository: None,
            git_task_rx: None,
            status_actions_since_task: TreeMap::default(),
            list_view_mode: GitViewMode::default(),
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
            self.status_actions_since_task
                .insert((active_repository, repo_path), StatusAction::Stage);
        }
    }

    pub fn unstage_entry(&mut self, repo_path: RepoPath) {
        if let Some(active_repository) = self.active_repository {
            self.status_actions_since_task
                .insert((active_repository, repo_path), StatusAction::Unstage);
        }
    }

    pub fn stage_entries(&mut self, entries: Vec<RepoPath>) {
        if let Some(active_repository) = self.active_repository {
            for entry in entries {
                self.status_actions_since_task
                    .insert((active_repository, entry), StatusAction::Stage);
            }
        }
    }

    fn act_on_all(&mut self, action: StatusAction, project: &Model<Project>, cx: &AppContext) {
        // FIXME this performs suboptimally, we might want to only collect actions
        // for entries that we think actually need to be acted upon
        if let Some(active_repository) = self.active_repository {
            // FIXME give TreeMap a clear method
            self.status_actions_since_task.retain(|_, _| false);
            let Some(worktree) = project.read(cx).worktree_for_entry(active_repository, cx) else {
                // FIXME maybe should handle this differently
                return;
            };
            let snapshot = worktree.read(cx).snapshot();
            let Some(repo) = snapshot
                .repositories()
                .find(|repo| repo.work_directory_id() == active_repository)
            else {
                // FIXME maybe should handle this differently
                return;
            };
            for status in repo.status() {
                self.status_actions_since_task
                    .insert((active_repository, status.repo_path), action);
            }
        }
    }

    pub fn stage_all(&mut self, project: &Model<Project>, cx: &AppContext) {
        self.act_on_all(StatusAction::Stage, project, cx);
    }

    pub fn unstage_all(&mut self, project: &Model<Project>, cx: &AppContext) {
        self.act_on_all(StatusAction::Unstage, project, cx);
    }

    pub fn toggle_staged_entry(
        &mut self,
        repo_path: RepoPath,
        project: &Model<Project>,
        cx: &AppContext,
    ) {
        // FIXME can make this faster
        if self.is_staged(repo_path.clone(), project, cx) {
            self.unstage_entry(repo_path);
        } else {
            self.stage_entry(repo_path);
        }
    }

    pub fn is_staged(
        &self,
        repo_path: RepoPath,
        project: &Model<Project>,
        cx: &AppContext,
    ) -> bool {
        let Some(active_repository) = self.active_repository else {
            return false;
        };
        if let Some(action) = self
            .status_actions_since_task
            .get(&(active_repository, repo_path.clone()))
        {
            return action == &StatusAction::Stage;
        }
        // FIXME what follows is ungainly
        let Some(worktree) = project.read(cx).worktree_for_entry(active_repository, cx) else {
            return false;
        };
        let snapshot = worktree.read(cx).snapshot();
        let Some(repo) = snapshot
            .repositories()
            .find(|repo| repo.work_directory_id() == active_repository)
        else {
            return false;
        };
        // FIXME this logic is wrong
        snapshot
            .status_for_file(repo.work_directory.unrelativize(&repo_path).unwrap())
            .is_none()
    }
}

pub fn first_worktree_repository(
    project: &Model<Project>,
    worktree_id: WorktreeId,
    cx: &mut AppContext,
) -> Option<ProjectEntryId> {
    project
        .read(cx)
        .worktree_for_id(worktree_id, cx)
        .and_then(|worktree| {
            let snapshot = worktree.read(cx).snapshot();
            let mut repositories = snapshot.repositories();
            repositories.next().map(|repo| repo.work_directory_id())
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
