use std::sync::Arc;

use anyhow::anyhow;
use futures::channel::mpsc;
use futures::StreamExt as _;
use git::{
    repository::{GitRepository, RepoPath},
    status::{GitSummary, TrackedSummary},
};
use gpui::{AppContext, SharedString};
use settings::WorktreeId;
use util::ResultExt as _;
use worktree::RepositoryEntry;

pub struct GitState {
    /// The current commit message being composed.
    pub commit_message: SharedString,

    /// When a git repository is selected, this is used to track which repository's changes
    /// are currently being viewed or modified in the UI.
    pub active_repository: Option<(WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)>,

    update_sender: mpsc::UnboundedSender<Message>,
}

enum Message {
    StageAndCommit(Arc<dyn GitRepository>, SharedString, Vec<RepoPath>),
    Commit(Arc<dyn GitRepository>, SharedString),
    Stage(Arc<dyn GitRepository>, Vec<RepoPath>),
    Unstage(Arc<dyn GitRepository>, Vec<RepoPath>),
}

impl GitState {
    pub fn new(cx: &AppContext) -> Self {
        let (tx, mut rx) = mpsc::unbounded();
        cx.spawn(|cx| async move {
            while let Some(msg) = rx.next().await {
                cx.background_executor()
                    .spawn(async move {
                        match msg {
                            Message::StageAndCommit(repo, message, paths) => {
                                repo.stage_paths(&paths)?;
                                repo.commit(&message)?;
                                Ok(())
                            }
                            Message::Stage(repo, paths) => repo.stage_paths(&paths),
                            Message::Unstage(repo, paths) => repo.unstage_paths(&paths),
                            Message::Commit(repo, message) => repo.commit(&message),
                        }
                    })
                    .await
                    .log_err();
            }
        })
        .detach();
        GitState {
            commit_message: SharedString::default(),
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

    pub fn stage_entries(&self, entries: Vec<RepoPath>) -> anyhow::Result<()> {
        if entries.is_empty() {
            return Ok(());
        }
        let Some((_, _, git_repo)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        self.update_sender
            .unbounded_send(Message::Stage(git_repo.clone(), entries))
            .map_err(|_| anyhow!("Failed to submit stage operation"))?;
        Ok(())
    }

    pub fn unstage_entries(&self, entries: Vec<RepoPath>) -> anyhow::Result<()> {
        if entries.is_empty() {
            return Ok(());
        }
        let Some((_, _, git_repo)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        self.update_sender
            .unbounded_send(Message::Unstage(git_repo.clone(), entries))
            .map_err(|_| anyhow!("Failed to submit unstage operation"))?;
        Ok(())
    }

    pub fn stage_all(&self) -> anyhow::Result<()> {
        let Some((_, entry, _)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        let to_stage = entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.stage_entries(to_stage)?;
        Ok(())
    }

    pub fn unstage_all(&self) -> anyhow::Result<()> {
        let Some((_, entry, _)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        let to_unstage = entry
            .status()
            .filter(|entry| entry.status.is_staged().unwrap_or(true))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.unstage_entries(to_unstage)?;
        Ok(())
    }

    /// Get a count of all entries in the active repository, including
    /// untracked files.
    pub fn entry_count(&self) -> usize {
        self.active_repository
            .as_ref()
            .map_or(0, |(_, entry, _)| entry.status_len())
    }

    fn have_changes(&self) -> bool {
        let Some((_, entry, _)) = self.active_repository.as_ref() else {
            return false;
        };
        entry.status_summary() != GitSummary::UNCHANGED
    }

    fn have_staged_changes(&self) -> bool {
        let Some((_, entry, _)) = self.active_repository.as_ref() else {
            return false;
        };
        entry.status_summary().index != TrackedSummary::UNCHANGED
    }

    pub fn can_commit(&self, commit_all: bool) -> bool {
        return !self.commit_message.trim().is_empty()
            && self.have_changes()
            && (commit_all || self.have_staged_changes());
    }

    pub fn commit(&mut self) -> anyhow::Result<()> {
        if !self.can_commit(false) {
            return Err(anyhow!("No staged changes to commit"));
        }
        let Some((_, _, git_repo)) = self.active_repository() else {
            return Err(anyhow!("No active repository"));
        };
        let git_repo = git_repo.clone();
        let message = std::mem::take(&mut self.commit_message);
        self.update_sender
            .unbounded_send(Message::Commit(git_repo, message))
            .map_err(|_| anyhow!("Failed to submit commit operation"))?;
        Ok(())
    }

    pub fn commit_all(&mut self) -> anyhow::Result<()> {
        if !self.can_commit(true) {
            return Err(anyhow!("No changes to commit"));
        }
        let Some((_, entry, git_repo)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        let to_stage = entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect::<Vec<_>>();
        let message = std::mem::take(&mut self.commit_message);
        self.update_sender
            .unbounded_send(Message::StageAndCommit(git_repo.clone(), message, to_stage))
            .map_err(|_| anyhow!("Failed to submit commit operation"))?;
        Ok(())
    }
}
