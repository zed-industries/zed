use crate::buffer_store::BufferStore;
use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use crate::{Project, ProjectPath};
use anyhow::Context as _;
use client::ProjectId;
use futures::channel::{mpsc, oneshot};
use futures::StreamExt as _;
use git::{
    repository::{GitRepository, RepoPath},
    status::{GitSummary, TrackedSummary},
};
use gpui::{
    App, AppContext, Context, Entity, EventEmitter, SharedString, Subscription, Task, WeakEntity,
};
use language::{Buffer, LanguageRegistry};
use rpc::{proto, AnyProtoClient};
use settings::WorktreeId;
use std::sync::Arc;
use text::BufferId;
use util::{maybe, ResultExt};
use worktree::{ProjectEntryId, RepositoryEntry, StatusEntry};

pub struct GitState {
    project_id: Option<ProjectId>,
    client: Option<AnyProtoClient>,
    repositories: Vec<Entity<Repository>>,
    active_index: Option<usize>,
    update_sender: mpsc::UnboundedSender<(Message, oneshot::Sender<anyhow::Result<()>>)>,
    _subscription: Subscription,
}

pub struct Repository {
    commit_message_buffer: Option<Entity<Buffer>>,
    git_state: WeakEntity<GitState>,
    pub worktree_id: WorktreeId,
    pub repository_entry: RepositoryEntry,
    pub git_repo: GitRepo,
    update_sender: mpsc::UnboundedSender<(Message, oneshot::Sender<anyhow::Result<()>>)>,
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

enum Message {
    Commit {
        git_repo: GitRepo,
        name_and_email: Option<(SharedString, SharedString)>,
    },
    Stage(GitRepo, Vec<RepoPath>),
    Unstage(GitRepo, Vec<RepoPath>),
}

pub enum GitEvent {
    ActiveRepositoryChanged,
    FileSystemUpdated,
    GitStateUpdated,
}

impl EventEmitter<GitEvent> for GitState {}

impl GitState {
    pub fn new(
        worktree_store: &Entity<WorktreeStore>,
        client: Option<AnyProtoClient>,
        project_id: Option<ProjectId>,
        cx: &mut Context<'_, Self>,
    ) -> Self {
        let update_sender = Self::spawn_git_worker(cx);
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

    pub fn active_repository(&self) -> Option<Entity<Repository>> {
        self.active_index
            .map(|index| self.repositories[index].clone())
    }

    fn on_worktree_store_event(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
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
                worktree.update(cx, |worktree, cx| {
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
                        let worktree_id = worktree.id();
                        let existing =
                            self.repositories
                                .iter()
                                .enumerate()
                                .find(|(_, existing_handle)| {
                                    existing_handle.read(cx).id()
                                        == (worktree_id, repo.work_directory_id())
                                });
                        let handle = if let Some((index, handle)) = existing {
                            if self.active_index == Some(index) {
                                new_active_index = Some(new_repositories.len());
                            }
                            // Update the statuses but keep everything else.
                            let existing_handle = handle.clone();
                            existing_handle.update(cx, |existing_handle, _| {
                                existing_handle.repository_entry = repo.clone();
                            });
                            existing_handle
                        } else {
                            cx.new(|_| Repository {
                                git_state: this.clone(),
                                worktree_id,
                                repository_entry: repo.clone(),
                                git_repo,
                                update_sender: self.update_sender.clone(),
                                commit_message_buffer: None,
                            })
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

        match event {
            WorktreeStoreEvent::WorktreeUpdatedGitRepositories(_) => {
                cx.emit(GitEvent::GitStateUpdated);
            }
            _ => {
                cx.emit(GitEvent::FileSystemUpdated);
            }
        }
    }

    pub fn all_repositories(&self) -> Vec<Entity<Repository>> {
        self.repositories.clone()
    }

    fn spawn_git_worker(
        cx: &mut Context<'_, GitState>,
    ) -> mpsc::UnboundedSender<(Message, oneshot::Sender<anyhow::Result<()>>)> {
        let (update_sender, mut update_receiver) =
            mpsc::unbounded::<(Message, oneshot::Sender<anyhow::Result<()>>)>();
        cx.spawn(|_, cx| async move {
            while let Some((msg, respond)) = update_receiver.next().await {
                let result = cx
                    .background_executor()
                    .spawn(Self::process_git_msg(msg))
                    .await;
                respond.send(result).ok();
            }
        })
        .detach();
        update_sender
    }

    async fn process_git_msg(msg: Message) -> Result<(), anyhow::Error> {
        match msg {
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
                    GitRepo::Local(repo) => repo.commit(
                        name_and_email
                            .as_ref()
                            .map(|(name, email)| (name.as_ref(), email.as_ref())),
                    )?,
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
    }
}

impl Repository {
    fn id(&self) -> (WorktreeId, ProjectEntryId) {
        (self.worktree_id, self.repository_entry.work_directory_id())
    }

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
                .find(|(_, handle)| handle.read(cx).id() == self.id())
            else {
                return;
            };
            git_state.active_index = Some(index);
            cx.emit(GitEvent::ActiveRepositoryChanged);
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

    pub fn open_commit_buffer(
        &mut self,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Entity<Buffer>>> {
        match &self.git_repo {
            GitRepo::Local(_) => {
                let project = project.read(cx);
                self.open_local_commit_buffer(
                    project.languages().clone(),
                    project.buffer_store().clone(),
                    cx,
                )
            }
            GitRepo::Remote {
                project_id,
                client,
                worktree_id,
                work_directory_id,
            } => {
                let request = client.request(proto::OpenCommitMessageBuffer {
                    project_id: project_id.0,
                    worktree_id: worktree_id.to_proto(),
                    work_directory_id: work_directory_id.to_proto(),
                });
                let git_commit_language =
                    project.read(cx).languages().language_for_name("gitcommit");
                let project = project.downgrade();
                cx.spawn(|_, mut cx| async move {
                    let response = request.await.context("requesting to open commit buffer")?;
                    let buffer_id = BufferId::new(response.buffer_id)?;
                    let buffer = project
                        .update(&mut cx, {
                            |project, cx| project.wait_for_remote_buffer(buffer_id, cx)
                        })?
                        .await?;
                    let git_commit_language = git_commit_language.await?;
                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.set_language(Some(git_commit_language), cx)
                    })?;
                    Ok(buffer)
                })
            }
        }
    }

    fn open_local_commit_buffer(
        &mut self,
        language_registry: Arc<LanguageRegistry>,
        buffer_store: Entity<BufferStore>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Entity<Buffer>>> {
        if let Some(buffer) = self.commit_message_buffer.clone() {
            return Task::ready(Ok(buffer));
        }

        cx.spawn(|this, mut cx| async move {
            let git_commit_language = language_registry.language_for_name("gitcommit").await?;

            let buffer = buffer_store
                .update(&mut cx, |buffer_store, cx| buffer_store.create_buffer(cx))?
                .await?;
            buffer.update(&mut cx, |buffer, cx| {
                buffer.set_language(Some(git_commit_language), cx);
            })?;
            this.update(&mut cx, |this, _| {
                this.commit_message_buffer = Some(buffer.clone());
            })?;
            Ok(buffer)
        })
    }

    pub fn stage_entries(&self, entries: Vec<RepoPath>) -> oneshot::Receiver<anyhow::Result<()>> {
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        if entries.is_empty() {
            result_tx.send(Ok(())).ok();
            return result_rx;
        }
        self.update_sender
            .unbounded_send((Message::Stage(self.git_repo.clone(), entries), result_tx))
            .ok();
        result_rx
    }

    pub fn unstage_entries(&self, entries: Vec<RepoPath>) -> oneshot::Receiver<anyhow::Result<()>> {
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        if entries.is_empty() {
            result_tx.send(Ok(())).ok();
            return result_rx;
        }
        self.update_sender
            .unbounded_send((Message::Unstage(self.git_repo.clone(), entries), result_tx))
            .ok();
        result_rx
    }

    pub fn stage_all(&self) -> oneshot::Receiver<anyhow::Result<()>> {
        let to_stage = self
            .repository_entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.stage_entries(to_stage)
    }

    pub fn unstage_all(&self) -> oneshot::Receiver<anyhow::Result<()>> {
        let to_unstage = self
            .repository_entry
            .status()
            .filter(|entry| entry.status.is_staged().unwrap_or(true))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.unstage_entries(to_unstage)
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
    ) -> oneshot::Receiver<anyhow::Result<()>> {
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        self.update_sender
            .unbounded_send((
                Message::Commit {
                    git_repo: self.git_repo.clone(),
                    name_and_email,
                },
                result_tx,
            ))
            .ok();
        result_rx
    }
}
