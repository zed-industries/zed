use anyhow::{anyhow, Context as _};
use futures::channel::mpsc;
use futures::{SinkExt as _, StreamExt as _};
use git::{
    repository::{GitRepository, RepoPath},
    status::{GitSummary, TrackedSummary},
};
use gpui::{AppContext, Context as _, Model, ModelContext, SharedString, Subscription, WeakModel};
use language::{Buffer, LanguageRegistry};
use std::sync::Arc;
use text::Rope;
use worktree::RepositoryEntry;

use crate::worktree_store::{self, WorktreeStore, WorktreeStoreEvent};

pub struct GitState {
    repositories: Vec<RepositoryHandle>,
    active_index: Option<usize>,
    update_sender: mpsc::UnboundedSender<(Message, mpsc::Sender<anyhow::Error>)>,
    _subscription: Subscription,
}

#[derive(Clone)]
pub struct RepositoryHandle {
    git_state: WeakModel<GitState>,
    repository_entry: RepositoryEntry,
    git_repo: Arc<dyn GitRepository>,
    commit_message: Model<Buffer>,
    update_sender: mpsc::UnboundedSender<(Message, mpsc::Sender<anyhow::Error>)>,
}

enum Message {
    StageAndCommit(Arc<dyn GitRepository>, Rope, Vec<RepoPath>),
    Commit(Arc<dyn GitRepository>, Rope),
    Stage(Arc<dyn GitRepository>, Vec<RepoPath>),
    Unstage(Arc<dyn GitRepository>, Vec<RepoPath>),
}

impl GitState {
    pub fn new(
        worktree_store: &Model<WorktreeStore>,
        languages: Arc<LanguageRegistry>,
        cx: &mut ModelContext<'_, Self>,
    ) -> Self {
        let (update_sender, mut update_receiver) =
            mpsc::unbounded::<(Message, mpsc::Sender<anyhow::Error>)>();
        cx.spawn(|_, cx| async move {
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

        //let commit_message = cx.new_model(|cx| Buffer::local("", cx));
        //let markdown = languages.language_for_name("Markdown");
        //cx.spawn({
        //    let commit_message = commit_message.clone();
        //    |mut cx| async move {
        //        let markdown = markdown.await.context("failed to load Markdown language")?;
        //        commit_message.update(&mut cx, |commit_message, cx| {
        //            commit_message.set_language(Some(markdown), cx)
        //        })
        //    }
        //})
        //.detach_and_log_err(cx);

        let _subscription = cx.subscribe(worktree_store, Self::on_worktree_store_event);

        GitState {
            repositories: vec![],
            active_index: None,
            update_sender,
            _subscription,
        }
    }

    fn on_worktree_store_event(
        &mut self,
        worktree_store: Model<WorktreeStore>,
        _event: &WorktreeStoreEvent,
        cx: &mut ModelContext<'_, Self>,
    ) {
        let mut new_repositories = Vec::new();
        let mut new_active_index = None;
        let this = cx.weak_model();

        worktree_store.update(cx, |worktree_store, cx| {
            for worktree in worktree_store.worktrees() {
                worktree.update(cx, |worktree, cx| {
                    let snapshot = worktree.snapshot();
                    let Some(local) = worktree.as_local() else {
                        return;
                    };
                    for repo in snapshot.repositories().iter() {
                        let Some(local_repo) = local.get_local_repo(repo) else {
                            continue;
                        };
                        let existing =
                            self.repositories
                                .iter()
                                .enumerate()
                                .find(|(_, existing_handle)| {
                                    existing_handle.repository_entry.work_directory_id
                                        == repo.work_directory_id
                                });
                        let handle = if let Some((index, handle)) = existing {
                            if self.active_index == Some(index) {
                                new_active_index = Some(new_repositories.len());
                            }
                            // Update the statuses but keep everything else.
                            let mut existing_handle = handle.clone();
                            existing_handle.repository_entry = repo.clone();
                            existing_handle
                        } else {
                            RepositoryHandle {
                                git_state: this.clone(),
                                repository_entry: repo.clone(),
                                git_repo: local_repo.repo().clone(),
                                // FIXME set markdown
                                commit_message: cx.new_model(|cx| Buffer::local("", cx)),
                                update_sender: self.update_sender.clone(),
                            }
                        };
                        // FIXME extend
                        new_repositories.push(handle);
                    }
                })
            }
        });

        self.repositories = new_repositories;
        self.active_index = new_active_index;
    }
}

impl RepositoryHandle {
    pub fn display_name(&self) -> SharedString {
        todo!()
    }

    pub fn activate(&self) {
        todo!()
    }

    pub fn stage_entries(
        &self,
        entries: Vec<RepoPath>,
        err_sender: mpsc::Sender<anyhow::Error>,
    ) -> anyhow::Result<()> {
        if entries.is_empty() {
            return Ok(());
        }
        self.update_sender
            .unbounded_send((Message::Stage(self.git_repo.clone(), entries), err_sender))
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
        self.update_sender
            .unbounded_send((Message::Unstage(self.git_repo.clone(), entries), err_sender))
            .map_err(|_| anyhow!("Failed to submit unstage operation"))?;
        Ok(())
    }

    pub fn stage_all(&self, err_sender: mpsc::Sender<anyhow::Error>) -> anyhow::Result<()> {
        let to_stage = self
            .repository_entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.stage_entries(to_stage, err_sender)?;
        Ok(())
    }

    pub fn unstage_all(&self, err_sender: mpsc::Sender<anyhow::Error>) -> anyhow::Result<()> {
        let to_unstage = self
            .repository_entry
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
        self.repository_entry.status_len()
    }

    fn have_changes(&self) -> bool {
        self.repository_entry.status_summary() != GitSummary::UNCHANGED
    }

    fn have_staged_changes(&self) -> bool {
        self.repository_entry.status_summary().index != TrackedSummary::UNCHANGED
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
        let message = self.commit_message.read(cx).as_rope().clone();
        self.update_sender
            .unbounded_send((Message::Commit(self.git_repo.clone(), message), err_sender))
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
        let to_stage = self
            .repository_entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect::<Vec<_>>();
        let message = self.commit_message.read(cx).as_rope().clone();
        self.update_sender
            .unbounded_send((
                Message::StageAndCommit(self.git_repo.clone(), message, to_stage),
                err_sender,
            ))
            .map_err(|_| anyhow!("Failed to submit commit operation"))?;
        Ok(())
    }
}
