use anyhow::{anyhow, Context as _};
use futures::channel::mpsc;
use futures::{SinkExt as _, StreamExt as _};
use git::{
    repository::{GitRepository, RepoPath},
    status::{GitSummary, TrackedSummary},
};
use gpui::{AppContext, Context as _, Model};
use language::{Buffer, LanguageRegistry};
use settings::WorktreeId;
use std::sync::Arc;
use text::Rope;
use worktree::RepositoryEntry;

pub struct GitState {
    pub commit_message: Model<Buffer>,

    /// When a git repository is selected, this is used to track which repository's changes
    /// are currently being viewed or modified in the UI.
    pub active_repository: Option<(WorktreeId, RepositoryEntry, Arc<dyn GitRepository>)>,

    update_sender: mpsc::UnboundedSender<(Message, mpsc::Sender<anyhow::Error>)>,
}

enum Message {
    StageAndCommit(Arc<dyn GitRepository>, Rope, Vec<RepoPath>),
    Commit(Arc<dyn GitRepository>, Rope),
    Stage(Arc<dyn GitRepository>, Vec<RepoPath>),
    Unstage(Arc<dyn GitRepository>, Vec<RepoPath>),
}

impl GitState {
    pub fn new(languages: Arc<LanguageRegistry>, cx: &mut AppContext) -> Self {
        let (update_sender, mut update_receiver) =
            mpsc::unbounded::<(Message, mpsc::Sender<anyhow::Error>)>();
        cx.spawn(|cx| async move {
            while let Some((msg, mut err_sender)) = update_receiver.next().await {
                let result = cx
                    .background_executor()
                    .spawn(async move {
                        match msg {
                            Message::StageAndCommit(repo, message, paths) => {
                                repo.stage_paths(&paths)?;
                                repo.commit(&message.to_string())?;
                                Ok(())
                            }
                            Message::Stage(repo, paths) => repo.stage_paths(&paths),
                            Message::Unstage(repo, paths) => repo.unstage_paths(&paths),
                            Message::Commit(repo, message) => repo.commit(&message.to_string()),
                        }
                    })
                    .await;
                if let Err(e) = result {
                    err_sender.send(e).await.ok();
                }
            }
        })
        .detach();

        let commit_message = cx.new_model(|cx| Buffer::local("", cx));
        let markdown = languages.language_for_name("Markdown");
        cx.spawn({
            let commit_message = commit_message.clone();
            |mut cx| async move {
                let markdown = markdown.await.context("failed to load Markdown language")?;
                commit_message.update(&mut cx, |commit_message, cx| {
                    commit_message.set_language(Some(markdown), cx)
                })
            }
        })
        .detach_and_log_err(cx);

        GitState {
            commit_message,
            active_repository: None,
            update_sender,
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

    pub fn stage_entries(
        &self,
        entries: Vec<RepoPath>,
        err_sender: mpsc::Sender<anyhow::Error>,
    ) -> anyhow::Result<()> {
        if entries.is_empty() {
            return Ok(());
        }
        let Some((_, _, git_repo)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        self.update_sender
            .unbounded_send((Message::Stage(git_repo.clone(), entries), err_sender))
            .map_err(|_| anyhow!("Failed to submit stage operation"))?;
        Ok(())
    }

    pub fn unstage_entries(
        &self,
        entries: Vec<RepoPath>,
        err_sender: mpsc::Sender<anyhow::Error>,
    ) -> anyhow::Result<()> {
        if entries.is_empty() {
            return Ok(());
        }
        let Some((_, _, git_repo)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        self.update_sender
            .unbounded_send((Message::Unstage(git_repo.clone(), entries), err_sender))
            .map_err(|_| anyhow!("Failed to submit unstage operation"))?;
        Ok(())
    }

    pub fn stage_all(&self, err_sender: mpsc::Sender<anyhow::Error>) -> anyhow::Result<()> {
        let Some((_, entry, _)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        let to_stage = entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.stage_entries(to_stage, err_sender)?;
        Ok(())
    }

    pub fn unstage_all(&self, err_sender: mpsc::Sender<anyhow::Error>) -> anyhow::Result<()> {
        let Some((_, entry, _)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        let to_unstage = entry
            .status()
            .filter(|entry| entry.status.is_staged().unwrap_or(true))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.unstage_entries(to_unstage, err_sender)?;
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

    pub fn can_commit(&self, commit_all: bool, cx: &AppContext) -> bool {
        return self
            .commit_message
            .read(cx)
            .chars()
            .any(|c| !c.is_ascii_whitespace())
            && self.have_changes()
            && (commit_all || self.have_staged_changes());
    }

    pub fn commit(
        &mut self,
        err_sender: mpsc::Sender<anyhow::Error>,
        cx: &AppContext,
    ) -> anyhow::Result<()> {
        if !self.can_commit(false, cx) {
            return Err(anyhow!("Unable to commit"));
        }
        let Some((_, _, git_repo)) = self.active_repository() else {
            return Err(anyhow!("No active repository"));
        };
        let git_repo = git_repo.clone();
        let message = self.commit_message.read(cx).as_rope().clone();
        self.update_sender
            .unbounded_send((Message::Commit(git_repo, message), err_sender))
            .map_err(|_| anyhow!("Failed to submit commit operation"))?;
        Ok(())
    }

    pub fn commit_all(
        &mut self,
        err_sender: mpsc::Sender<anyhow::Error>,
        cx: &AppContext,
    ) -> anyhow::Result<()> {
        if !self.can_commit(true, cx) {
            return Err(anyhow!("Unable to commit"));
        }
        let Some((_, entry, git_repo)) = self.active_repository.as_ref() else {
            return Err(anyhow!("No active repository"));
        };
        let to_stage = entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect::<Vec<_>>();
        let message = self.commit_message.read(cx).as_rope().clone();
        self.update_sender
            .unbounded_send((
                Message::StageAndCommit(git_repo.clone(), message, to_stage),
                err_sender,
            ))
            .map_err(|_| anyhow!("Failed to submit commit operation"))?;
        Ok(())
    }
}
