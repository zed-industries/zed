use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use crate::{Project, ProjectPath};
use anyhow::anyhow;
use futures::channel::mpsc;
use futures::{SinkExt as _, StreamExt as _};
use git::{
    repository::{GitRepository, RepoPath},
    status::{GitSummary, TrackedSummary},
};
use gpui::{
    AppContext, Context as _, EventEmitter, Model, ModelContext, SharedString, Subscription,
    WeakModel,
};
use language::{Buffer, LanguageRegistry};
use settings::WorktreeId;
use std::sync::Arc;
use text::Rope;
use util::maybe;
use worktree::{RepositoryEntry, StatusEntry};

pub struct GitState {
    repositories: Vec<RepositoryHandle>,
    active_index: Option<usize>,
    update_sender: mpsc::UnboundedSender<(Message, mpsc::Sender<anyhow::Error>)>,
    languages: Arc<LanguageRegistry>,
    _subscription: Subscription,
}

#[derive(Clone)]
pub struct RepositoryHandle {
    git_state: WeakModel<GitState>,
    worktree_id: WorktreeId,
    repository_entry: RepositoryEntry,
    git_repo: Arc<dyn GitRepository>,
    commit_message: Model<Buffer>,
    update_sender: mpsc::UnboundedSender<(Message, mpsc::Sender<anyhow::Error>)>,
}

impl PartialEq<Self> for RepositoryHandle {
    fn eq(&self, other: &Self) -> bool {
        self.worktree_id == other.worktree_id
            && self.repository_entry.work_directory_id()
                == other.repository_entry.work_directory_id()
    }
}

impl Eq for RepositoryHandle {}

impl PartialEq<RepositoryEntry> for RepositoryHandle {
    fn eq(&self, other: &RepositoryEntry) -> bool {
        self.repository_entry.work_directory_id() == other.work_directory_id()
    }
}

enum Message {
    StageAndCommit(Arc<dyn GitRepository>, Rope, Vec<RepoPath>),
    Commit(Arc<dyn GitRepository>, Rope),
    Stage(Arc<dyn GitRepository>, Vec<RepoPath>),
    Unstage(Arc<dyn GitRepository>, Vec<RepoPath>),
}

pub enum Event {
    RepositoriesUpdated,
}

impl EventEmitter<Event> for GitState {}

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

        let _subscription = cx.subscribe(worktree_store, Self::on_worktree_store_event);

        GitState {
            languages,
            repositories: vec![],
            active_index: None,
            update_sender,
            _subscription,
        }
    }

    pub fn active_repository(&self) -> Option<RepositoryHandle> {
        self.active_index
            .map(|index| self.repositories[index].clone())
    }

    fn on_worktree_store_event(
        &mut self,
        worktree_store: Model<WorktreeStore>,
        _event: &WorktreeStoreEvent,
        cx: &mut ModelContext<'_, Self>,
    ) {
        // TODO inspect the event

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
                        let existing = self
                            .repositories
                            .iter()
                            .enumerate()
                            .find(|(_, existing_handle)| existing_handle == &repo);
                        let handle = if let Some((index, handle)) = existing {
                            if self.active_index == Some(index) {
                                new_active_index = Some(new_repositories.len());
                            }
                            // Update the statuses but keep everything else.
                            let mut existing_handle = handle.clone();
                            existing_handle.repository_entry = repo.clone();
                            existing_handle
                        } else {
                            let commit_message = cx.new_model(|cx| Buffer::local("", cx));
                            cx.spawn({
                                let commit_message = commit_message.downgrade();
                                let languages = self.languages.clone();
                                |_, mut cx| async move {
                                    let markdown = languages.language_for_name("Markdown").await?;
                                    commit_message.update(&mut cx, |commit_message, cx| {
                                        commit_message.set_language(Some(markdown), cx);
                                    })?;
                                    anyhow::Ok(())
                                }
                            })
                            .detach_and_log_err(cx);
                            RepositoryHandle {
                                git_state: this.clone(),
                                worktree_id: worktree.id(),
                                repository_entry: repo.clone(),
                                git_repo: local_repo.repo().clone(),
                                commit_message,
                                update_sender: self.update_sender.clone(),
                            }
                        };
                        new_repositories.push(handle);
                    }
                })
            }
        });

        if new_active_index == None && new_repositories.len() > 0 {
            new_active_index = Some(0);
        }

        self.repositories = new_repositories;
        self.active_index = new_active_index;

        cx.emit(Event::RepositoriesUpdated);
    }

    pub fn all_repositories(&self) -> Vec<RepositoryHandle> {
        self.repositories.clone()
    }
}

impl RepositoryHandle {
    pub fn display_name(&self, project: &Project, cx: &AppContext) -> SharedString {
        maybe!({
            let path = self.unrelativize(&"".into())?;
            Some(
                project
                    .absolute_path(&path, cx)?
                    .file_name()?
                    .to_string_lossy()
                    .to_string()
                    .into(),
            )
        })
        .unwrap_or("".into())
    }

    pub fn activate(&self, cx: &mut AppContext) {
        let Some(git_state) = self.git_state.upgrade() else {
            return;
        };
        git_state.update(cx, |git_state, cx| {
            let Some((index, _)) = git_state
                .repositories
                .iter()
                .enumerate()
                .find(|(_, handle)| handle == &self)
            else {
                return;
            };
            git_state.active_index = Some(index);
            cx.emit(Event::RepositoriesUpdated);
        });
    }

    pub fn status(&self) -> impl '_ + Iterator<Item = StatusEntry> {
        self.repository_entry.status()
    }

    pub fn unrelativize(&self, path: &RepoPath) -> Option<ProjectPath> {
        let path = self.repository_entry.unrelativize(path)?;
        Some((self.worktree_id, path).into())
    }

    pub fn commit_message(&self) -> Model<Buffer> {
        self.commit_message.clone()
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
        &self,
        err_sender: mpsc::Sender<anyhow::Error>,
        cx: &mut AppContext,
    ) -> anyhow::Result<()> {
        if !self.can_commit(false, cx) {
            return Err(anyhow!("Unable to commit"));
        }
        let message = self.commit_message.read(cx).as_rope().clone();
        self.update_sender
            .unbounded_send((Message::Commit(self.git_repo.clone(), message), err_sender))
            .map_err(|_| anyhow!("Failed to submit commit operation"))?;
        self.commit_message.update(cx, |commit_message, cx| {
            commit_message.set_text("", cx);
        });
        Ok(())
    }

    pub fn commit_all(
        &self,
        err_sender: mpsc::Sender<anyhow::Error>,
        cx: &mut AppContext,
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
        self.commit_message.update(cx, |commit_message, cx| {
            commit_message.set_text("", cx);
        });
        Ok(())
    }
}
