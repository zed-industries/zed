use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use crate::{Project, ProjectPath};
use anyhow::{anyhow, Context as _};
use client::ProjectId;
use futures::channel::mpsc;
use futures::{SinkExt as _, StreamExt as _};
use git::{
    repository::{GitRepository, RepoPath},
    status::{GitSummary, TrackedSummary},
};
use gpui::{App, Context, Entity, EventEmitter, SharedString, Subscription, WeakEntity};
use rpc::{proto, AnyProtoClient};
use settings::WorktreeId;
use std::sync::Arc;
use util::{maybe, ResultExt};
use worktree::{ProjectEntryId, RepositoryEntry, StatusEntry};

pub struct GitState {
    project_id: Option<ProjectId>,
    client: Option<AnyProtoClient>,
    repositories: Vec<RepositoryHandle>,
    active_index: Option<usize>,
    update_sender: mpsc::UnboundedSender<(Message, mpsc::Sender<anyhow::Error>)>,
    _subscription: Subscription,
}

#[derive(Clone)]
pub struct RepositoryHandle {
    git_state: WeakEntity<GitState>,
    pub worktree_id: WorktreeId,
    pub repository_entry: RepositoryEntry,
    pub git_repo: GitRepo,
    update_sender: mpsc::UnboundedSender<(Message, mpsc::Sender<anyhow::Error>)>,
}

#[derive(Clone)]
pub enum GitRepo {
    Local(Arc<dyn GitRepository>),
    Remote {
        project_id: ProjectId,
        client: AnyProtoClient,
        worktree_id: WorktreeId,
        work_directory_id: ProjectEntryId,
    },
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
    StageAndCommit {
        git_repo: GitRepo,
        paths: Vec<RepoPath>,
        name_and_email: Option<(SharedString, SharedString)>,
    },
    Commit {
        git_repo: GitRepo,
        name_and_email: Option<(SharedString, SharedString)>,
    },
    Stage(GitRepo, Vec<RepoPath>),
    Unstage(GitRepo, Vec<RepoPath>),
}

pub enum Event {
    RepositoriesUpdated,
}

impl EventEmitter<Event> for GitState {}

impl GitState {
    pub fn new(
        worktree_store: &Entity<WorktreeStore>,
        client: Option<AnyProtoClient>,
        project_id: Option<ProjectId>,
        cx: &mut Context<'_, Self>,
    ) -> Self {
        let (update_sender, mut update_receiver) =
            mpsc::unbounded::<(Message, mpsc::Sender<anyhow::Error>)>();
        cx.spawn(|_, cx| async move {
            while let Some((msg, mut err_sender)) = update_receiver.next().await {
                let result =
                    cx.background_executor()
                        .spawn(async move {
                            match msg {
                                Message::StageAndCommit {
                                    git_repo,
                                    name_and_email,
                                    paths,
                                } => {
                                    match git_repo {
                                        GitRepo::Local(repo) => {
                                            repo.stage_paths(&paths)?;
                                            repo.commit(name_and_email.as_ref().map(
                                                |(name, email)| (name.as_ref(), email.as_ref()),
                                            ))?;
                                        }
                                        GitRepo::Remote {
                                            project_id,
                                            client,
                                            worktree_id,
                                            work_directory_id,
                                        } => {
                                            client
                                                .request(proto::Stage {
                                                    project_id: project_id.0,
                                                    worktree_id: worktree_id.to_proto(),
                                                    work_directory_id: work_directory_id.to_proto(),
                                                    paths: paths
                                                        .into_iter()
                                                        .map(|repo_path| repo_path.to_proto())
                                                        .collect(),
                                                })
                                                .await
                                                .context("sending stage request")?;
                                            let (name, email) = name_and_email.unzip();
                                            client
                                                .request(proto::Commit {
                                                    project_id: project_id.0,
                                                    worktree_id: worktree_id.to_proto(),
                                                    work_directory_id: work_directory_id.to_proto(),
                                                    name: name.map(String::from),
                                                    email: email.map(String::from),
                                                })
                                                .await
                                                .context("sending commit request")?;
                                        }
                                    }

                                    Ok(())
                                }
                                Message::Stage(repo, paths) => {
                                    match repo {
                                        GitRepo::Local(repo) => repo.stage_paths(&paths)?,
                                        GitRepo::Remote {
                                            project_id,
                                            client,
                                            worktree_id,
                                            work_directory_id,
                                        } => {
                                            client
                                                .request(proto::Stage {
                                                    project_id: project_id.0,
                                                    worktree_id: worktree_id.to_proto(),
                                                    work_directory_id: work_directory_id.to_proto(),
                                                    paths: paths
                                                        .into_iter()
                                                        .map(|repo_path| repo_path.to_proto())
                                                        .collect(),
                                                })
                                                .await
                                                .context("sending stage request")?;
                                        }
                                    }
                                    Ok(())
                                }
                                Message::Unstage(repo, paths) => {
                                    match repo {
                                        GitRepo::Local(repo) => repo.unstage_paths(&paths)?,
                                        GitRepo::Remote {
                                            project_id,
                                            client,
                                            worktree_id,
                                            work_directory_id,
                                        } => {
                                            client
                                                .request(proto::Unstage {
                                                    project_id: project_id.0,
                                                    worktree_id: worktree_id.to_proto(),
                                                    work_directory_id: work_directory_id.to_proto(),
                                                    paths: paths
                                                        .into_iter()
                                                        .map(|repo_path| repo_path.to_proto())
                                                        .collect(),
                                                })
                                                .await
                                                .context("sending unstage request")?;
                                        }
                                    }
                                    Ok(())
                                }
                                Message::Commit {
                                    git_repo,
                                    name_and_email,
                                } => {
                                    match git_repo {
                                        GitRepo::Local(repo) => {
                                            repo.commit(name_and_email.as_ref().map(
                                                |(name, email)| (name.as_ref(), email.as_ref()),
                                            ))?
                                        }
                                        GitRepo::Remote {
                                            project_id,
                                            client,
                                            worktree_id,
                                            work_directory_id,
                                        } => {
                                            let (name, email) = name_and_email.unzip();
                                            client
                                                .request(proto::Commit {
                                                    project_id: project_id.0,
                                                    worktree_id: worktree_id.to_proto(),
                                                    work_directory_id: work_directory_id.to_proto(),
                                                    name: name.map(String::from),
                                                    email: email.map(String::from),
                                                })
                                                .await
                                                .context("sending commit request")?;
                                        }
                                    }
                                    Ok(())
                                }
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
            project_id,
            client,
            repositories: Vec::new(),
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
        worktree_store: Entity<WorktreeStore>,
        _event: &WorktreeStoreEvent,
        cx: &mut Context<'_, Self>,
    ) {
        // TODO inspect the event

        let mut new_repositories = Vec::new();
        let mut new_active_index = None;
        let this = cx.weak_entity();
        let client = self.client.clone();
        let project_id = self.project_id;

        worktree_store.update(cx, |worktree_store, cx| {
            for worktree in worktree_store.worktrees() {
                worktree.update(cx, |worktree, _| {
                    let snapshot = worktree.snapshot();
                    for repo in snapshot.repositories().iter() {
                        let git_repo = worktree
                            .as_local()
                            .and_then(|local_worktree| local_worktree.get_local_repo(repo))
                            .map(|local_repo| local_repo.repo().clone())
                            .map(GitRepo::Local)
                            .or_else(|| {
                                let client = client.clone()?;
                                let project_id = project_id?;
                                Some(GitRepo::Remote {
                                    project_id,
                                    client,
                                    worktree_id: worktree.id(),
                                    work_directory_id: repo.work_directory_id(),
                                })
                            });
                        let Some(git_repo) = git_repo else {
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
                            RepositoryHandle {
                                git_state: this.clone(),
                                worktree_id: worktree.id(),
                                repository_entry: repo.clone(),
                                git_repo,
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
    pub fn display_name(&self, project: &Project, cx: &App) -> SharedString {
        maybe!({
            let path = self.repo_path_to_project_path(&"".into())?;
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

    pub fn activate(&self, cx: &mut App) {
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

    pub fn repo_path_to_project_path(&self, path: &RepoPath) -> Option<ProjectPath> {
        let path = self.repository_entry.unrelativize(path)?;
        Some((self.worktree_id, path).into())
    }

    pub fn project_path_to_repo_path(&self, path: &ProjectPath) -> Option<RepoPath> {
        if path.worktree_id != self.worktree_id {
            return None;
        }
        self.repository_entry.relativize(&path.path).log_err()
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

    pub fn can_commit(&self, commit_all: bool) -> bool {
        return self.have_changes() && (commit_all || self.have_staged_changes());
    }

    pub fn commit(
        &self,
        name_and_email: Option<(SharedString, SharedString)>,
        mut err_sender: mpsc::Sender<anyhow::Error>,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        let result = self.update_sender.unbounded_send((
            Message::Commit {
                git_repo: self.git_repo.clone(),
                name_and_email,
            },
            err_sender.clone(),
        ));
        if result.is_err() {
            cx.spawn(|_| async move {
                err_sender
                    .send(anyhow!("Failed to submit commit operation"))
                    .await
                    .ok();
            })
            .detach();
            anyhow::bail!("Failed to submit commit operation");
        } else {
            Ok(())
        }
    }

    pub fn commit_all(
        &self,
        name_and_email: Option<(SharedString, SharedString)>,
        mut err_sender: mpsc::Sender<anyhow::Error>,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        let to_stage = self
            .repository_entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect();
        let result = self.update_sender.unbounded_send((
            Message::StageAndCommit {
                git_repo: self.git_repo.clone(),
                paths: to_stage,
                name_and_email,
            },
            err_sender.clone(),
        ));
        if result.is_err() {
            cx.spawn(|_| async move {
                err_sender
                    .send(anyhow!("Failed to submit commit all operation"))
                    .await
                    .ok();
            })
            .detach();
            anyhow::bail!("Failed to submit commit all operation");
        } else {
            Ok(())
        }
    }
}
