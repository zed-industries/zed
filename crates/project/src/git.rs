use std::sync::Arc;

use futures::channel::mpsc;
use futures::StreamExt as _;
use git::repository::{GitRepository, RepoPath};
use gpui::{AppContext, SharedString};
use settings::WorktreeId;
use util::ResultExt as _;
use worktree::RepositoryEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusAction {
    Stage,
    Unstage,
}

pub struct GitState {
    /// The current commit message being composed.
    pub commit_message: Option<SharedString>,

    /// When a git repository is selected, this is used to track which repository's changes
    /// are currently being viewed or modified in the UI.
    pub active_repository: Option<(WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)>,

    pub update_sender: mpsc::UnboundedSender<(Arc<dyn GitRepository>, Vec<RepoPath>, StatusAction)>,
}

impl GitState {
    pub fn new(cx: &AppContext) -> Self {
        let (tx, mut rx) =
            mpsc::unbounded::<(Arc<dyn GitRepository>, Vec<RepoPath>, StatusAction)>();
        cx.spawn(|cx| async move {
            while let Some((git_repo, paths, action)) = rx.next().await {
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
            update_sender: tx,
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

    fn act_on_entries(&self, entries: Vec<RepoPath>, action: StatusAction) {
        if entries.is_empty() {
            return;
        }
        if let Some((_, _, git_repo)) = self.active_repository.as_ref() {
            let _ = self
                .update_sender
                .unbounded_send((git_repo.clone(), entries, action));
        }
    }

    pub fn stage_entries(&self, entries: Vec<RepoPath>) {
        self.act_on_entries(entries, StatusAction::Stage);
    }

    pub fn unstage_entries(&self, entries: Vec<RepoPath>) {
        self.act_on_entries(entries, StatusAction::Unstage);
    }

    pub fn stage_all(&self) {
        let Some((_, entry, _)) = self.active_repository.as_ref() else {
            return;
        };
        let to_stage = entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.stage_entries(to_stage);
    }

    pub fn unstage_all(&self) {
        let Some((_, entry, _)) = self.active_repository.as_ref() else {
            return;
        };
        let to_unstage = entry
            .status()
            .filter(|entry| entry.status.is_staged().unwrap_or(true))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.unstage_entries(to_unstage);
    }

    /// Get a count of all entries in the active repository, including
    /// untracked files.
    pub fn entry_count(&self) -> usize {
        self.active_repository
            .as_ref()
            .map_or(0, |(_, entry, _)| entry.status_len())
    }
}
