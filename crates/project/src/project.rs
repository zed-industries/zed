pub mod fs;
mod ignore;
pub mod worktree;

use anyhow::{anyhow, Result};
use client::{proto, Client, PeerId, TypedEnvelope, User, UserStore};
use clock::ReplicaId;
use collections::{hash_map, HashMap, HashSet};
use futures::Future;
use fuzzy::{PathMatch, PathMatchCandidate, PathMatchCandidateSet};
use gpui::{
    AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task,
    WeakModelHandle,
};
use language::{
    proto::{deserialize_anchor, serialize_anchor},
    range_from_lsp, Bias, Buffer, Diagnostic, DiagnosticEntry, File as _, Language,
    LanguageRegistry, PointUtf16, ToOffset, ToPointUtf16,
};
use lsp::{DiagnosticSeverity, LanguageServer};
use postage::{prelude::Stream, watch};
use smol::block_on;
use std::fmt::{Debug, Formatter};
use std::os::unix::ffi::OsStrExt;
use std::{
    convert::TryInto,
    ops::Range,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};
use util::{post_inc, ResultExt, TryFutureExt as _};

pub use fs::*;
pub use worktree::*;

pub struct Project {
    worktrees: Vec<WorktreeHandle>,
    active_entry: Option<ProjectEntry>,
    languages: Arc<LanguageRegistry>,
    language_servers: HashMap<(WorktreeId, String), Arc<LanguageServer>>,
    client: Arc<client::Client>,
    user_store: ModelHandle<UserStore>,
    fs: Arc<dyn Fs>,
    client_state: ProjectClientState,
    collaborators: HashMap<PeerId, Collaborator>,
    subscriptions: Vec<client::Subscription>,
    language_servers_with_diagnostics_running: isize,
    open_buffers: HashMap<usize, WeakModelHandle<Buffer>>,
    fs_tasks: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<FsTaskOutput, Arc<anyhow::Error>>>>,
    >,
    shared_buffers: HashMap<PeerId, HashMap<u64, ModelHandle<Buffer>>>,
}

#[derive(Clone)]
pub enum FsTaskOutput {
    LoadedBuffer(ModelHandle<Buffer>),
    CreatedDirectory,
    RemovedPath,
}

impl Debug for FsTaskOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FsTaskOutput::LoadedBuffer(_) => "LoadedBuffer",
                FsTaskOutput::CreatedDirectory => "CreatedDirectory",
                FsTaskOutput::RemovedPath => "RemovedPath",
            }
        )
    }
}

enum WorktreeHandle {
    Strong(ModelHandle<Worktree>),
    Weak(WeakModelHandle<Worktree>),
}

enum ProjectClientState {
    Local {
        is_shared: bool,
        remote_id_tx: watch::Sender<Option<u64>>,
        remote_id_rx: watch::Receiver<Option<u64>>,
        _maintain_remote_id_task: Task<Option<()>>,
    },
    Remote {
        sharing_has_stopped: bool,
        remote_id: u64,
        replica_id: ReplicaId,
    },
}

#[derive(Clone, Debug)]
pub struct Collaborator {
    pub user: Arc<User>,
    pub peer_id: PeerId,
    pub replica_id: ReplicaId,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    ActiveEntryChanged(Option<ProjectEntry>),
    WorktreeRemoved(WorktreeId),
    DiskBasedDiagnosticsStarted,
    DiskBasedDiagnosticsUpdated,
    DiskBasedDiagnosticsFinished,
    DiagnosticsUpdated(ProjectPath),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ProjectPath {
    pub worktree_id: WorktreeId,
    pub path: Arc<Path>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiagnosticSummary {
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub hint_count: usize,
}

#[derive(Debug)]
pub struct Definition {
    pub target_buffer: ModelHandle<Buffer>,
    pub target_range: Range<language::Anchor>,
}

impl DiagnosticSummary {
    fn new<'a, T: 'a>(diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<T>>) -> Self {
        let mut this = Self {
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            hint_count: 0,
        };

        for entry in diagnostics {
            if entry.diagnostic.is_primary {
                match entry.diagnostic.severity {
                    DiagnosticSeverity::ERROR => this.error_count += 1,
                    DiagnosticSeverity::WARNING => this.warning_count += 1,
                    DiagnosticSeverity::INFORMATION => this.info_count += 1,
                    DiagnosticSeverity::HINT => this.hint_count += 1,
                    _ => {}
                }
            }
        }

        this
    }

    pub fn to_proto(&self, path: Arc<Path>) -> proto::DiagnosticSummary {
        proto::DiagnosticSummary {
            path: path.to_string_lossy().to_string(),
            error_count: self.error_count as u32,
            warning_count: self.warning_count as u32,
            info_count: self.info_count as u32,
            hint_count: self.hint_count as u32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProjectEntry {
    pub worktree_id: WorktreeId,
    pub entry_id: usize,
}

impl Project {
    pub fn local(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut MutableAppContext,
    ) -> ModelHandle<Self> {
        cx.add_model(|cx: &mut ModelContext<Self>| {
            let (remote_id_tx, remote_id_rx) = watch::channel();
            let _maintain_remote_id_task = cx.spawn_weak({
                let rpc = client.clone();
                move |this, mut cx| {
                    async move {
                        let mut status = rpc.status();
                        while let Some(status) = status.recv().await {
                            if let Some(this) = this.upgrade(&cx) {
                                let remote_id = if let client::Status::Connected { .. } = status {
                                    let response = rpc.request(proto::RegisterProject {}).await?;
                                    Some(response.project_id)
                                } else {
                                    None
                                };

                                if let Some(project_id) = remote_id {
                                    let mut registrations = Vec::new();
                                    this.update(&mut cx, |this, cx| {
                                        for worktree in this.worktrees(cx).collect::<Vec<_>>() {
                                            registrations.push(worktree.update(
                                                cx,
                                                |worktree, cx| {
                                                    let worktree = worktree.as_local_mut().unwrap();
                                                    worktree.register(project_id, cx)
                                                },
                                            ));
                                        }
                                    });
                                    for registration in registrations {
                                        registration.await?;
                                    }
                                }
                                this.update(&mut cx, |this, cx| this.set_remote_id(remote_id, cx));
                            }
                        }
                        Ok(())
                    }
                    .log_err()
                }
            });

            Self {
                worktrees: Default::default(),
                collaborators: Default::default(),
                open_buffers: Default::default(),
                fs_tasks: Default::default(),
                shared_buffers: Default::default(),
                client_state: ProjectClientState::Local {
                    is_shared: false,
                    remote_id_tx,
                    remote_id_rx,
                    _maintain_remote_id_task,
                },
                subscriptions: Vec::new(),
                active_entry: None,
                languages,
                client,
                user_store,
                fs,
                language_servers_with_diagnostics_running: 0,
                language_servers: Default::default(),
            }
        })
    }

    pub async fn remote(
        remote_id: u64,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        client.authenticate_and_connect(&cx).await?;

        let response = client
            .request(proto::JoinProject {
                project_id: remote_id,
            })
            .await?;

        let replica_id = response.replica_id as ReplicaId;

        let mut worktrees = Vec::new();
        for worktree in response.worktrees {
            let (worktree, load_task) = cx
                .update(|cx| Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx));
            worktrees.push(worktree);
            load_task.detach();
        }

        let user_ids = response
            .collaborators
            .iter()
            .map(|peer| peer.user_id)
            .collect();
        user_store
            .update(cx, |user_store, cx| user_store.load_users(user_ids, cx))
            .await?;
        let mut collaborators = HashMap::default();
        for message in response.collaborators {
            let collaborator = Collaborator::from_proto(message, &user_store, cx).await?;
            collaborators.insert(collaborator.peer_id, collaborator);
        }

        Ok(cx.add_model(|cx| {
            let mut this = Self {
                worktrees: Vec::new(),
                open_buffers: Default::default(),
                fs_tasks: Default::default(),
                shared_buffers: Default::default(),
                active_entry: None,
                collaborators,
                languages,
                user_store,
                fs,
                subscriptions: vec![
                    client.subscribe_to_entity(remote_id, cx, Self::handle_unshare_project),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_add_collaborator),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_remove_collaborator),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_share_worktree),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_unregister_worktree),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_update_worktree),
                    client.subscribe_to_entity(
                        remote_id,
                        cx,
                        Self::handle_update_diagnostic_summary,
                    ),
                    client.subscribe_to_entity(
                        remote_id,
                        cx,
                        Self::handle_disk_based_diagnostics_updating,
                    ),
                    client.subscribe_to_entity(
                        remote_id,
                        cx,
                        Self::handle_disk_based_diagnostics_updated,
                    ),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_update_buffer),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_update_buffer_file),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_buffer_reloaded),
                    client.subscribe_to_entity(remote_id, cx, Self::handle_buffer_saved),
                ],
                client,
                client_state: ProjectClientState::Remote {
                    sharing_has_stopped: false,
                    remote_id,
                    replica_id,
                },
                language_servers_with_diagnostics_running: 0,
                language_servers: Default::default(),
            };
            for worktree in worktrees {
                this.add_worktree(&worktree, cx);
            }
            this
        }))
    }

    fn set_remote_id(&mut self, remote_id: Option<u64>, cx: &mut ModelContext<Self>) {
        if let ProjectClientState::Local { remote_id_tx, .. } = &mut self.client_state {
            *remote_id_tx.borrow_mut() = remote_id;
        }

        self.subscriptions.clear();
        if let Some(remote_id) = remote_id {
            let client = &self.client;
            self.subscriptions.extend([
                client.subscribe_to_entity(remote_id, cx, Self::handle_open_buffer),
                client.subscribe_to_entity(remote_id, cx, Self::handle_close_buffer),
                client.subscribe_to_entity(remote_id, cx, Self::handle_add_collaborator),
                client.subscribe_to_entity(remote_id, cx, Self::handle_remove_collaborator),
                client.subscribe_to_entity(remote_id, cx, Self::handle_update_worktree),
                client.subscribe_to_entity(remote_id, cx, Self::handle_update_buffer),
                client.subscribe_to_entity(remote_id, cx, Self::handle_save_buffer),
                client.subscribe_to_entity(remote_id, cx, Self::handle_buffer_saved),
                client.subscribe_to_entity(remote_id, cx, Self::handle_format_buffer),
                client.subscribe_to_entity(remote_id, cx, Self::handle_get_completions),
                client.subscribe_to_entity(
                    remote_id,
                    cx,
                    Self::handle_apply_additional_edits_for_completion,
                ),
                client.subscribe_to_entity(remote_id, cx, Self::handle_get_definition),
            ]);
        }
    }

    pub fn remote_id(&self) -> Option<u64> {
        match &self.client_state {
            ProjectClientState::Local { remote_id_rx, .. } => *remote_id_rx.borrow(),
            ProjectClientState::Remote { remote_id, .. } => Some(*remote_id),
        }
    }

    pub fn next_remote_id(&self) -> impl Future<Output = u64> {
        let mut id = None;
        let mut watch = None;
        match &self.client_state {
            ProjectClientState::Local { remote_id_rx, .. } => watch = Some(remote_id_rx.clone()),
            ProjectClientState::Remote { remote_id, .. } => id = Some(*remote_id),
        }

        async move {
            if let Some(id) = id {
                return id;
            }
            let mut watch = watch.unwrap();
            loop {
                let id = *watch.borrow();
                if let Some(id) = id {
                    return id;
                }
                watch.recv().await;
            }
        }
    }

    pub fn replica_id(&self) -> ReplicaId {
        match &self.client_state {
            ProjectClientState::Local { .. } => 0,
            ProjectClientState::Remote { replica_id, .. } => *replica_id,
        }
    }

    pub fn collaborators(&self) -> &HashMap<PeerId, Collaborator> {
        &self.collaborators
    }

    pub fn worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = ModelHandle<Worktree>> {
        self.worktrees
            .iter()
            .filter_map(move |worktree| worktree.upgrade(cx))
    }

    pub fn worktree_for_id(
        &self,
        id: WorktreeId,
        cx: &AppContext,
    ) -> Option<ModelHandle<Worktree>> {
        self.worktrees(cx)
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn share(&self, cx: &mut ModelContext<Self>) -> Task<anyhow::Result<()>> {
        let rpc = self.client.clone();
        cx.spawn(|this, mut cx| async move {
            let project_id = this.update(&mut cx, |this, _| {
                if let ProjectClientState::Local {
                    is_shared,
                    remote_id_rx,
                    ..
                } = &mut this.client_state
                {
                    *is_shared = true;
                    remote_id_rx
                        .borrow()
                        .ok_or_else(|| anyhow!("no project id"))
                } else {
                    Err(anyhow!("can't share a remote project"))
                }
            })?;

            rpc.request(proto::ShareProject { project_id }).await?;
            let mut tasks = Vec::new();
            this.update(&mut cx, |this, cx| {
                for worktree in this.worktrees(cx).collect::<Vec<_>>() {
                    worktree.update(cx, |worktree, cx| {
                        let worktree = worktree.as_local_mut().unwrap();
                        tasks.push(worktree.share(project_id, cx));
                    });
                }
            });
            for task in tasks {
                task.await?;
            }
            this.update(&mut cx, |_, cx| cx.notify());
            Ok(())
        })
    }

    pub fn unshare(&self, cx: &mut ModelContext<Self>) -> Task<anyhow::Result<()>> {
        let rpc = self.client.clone();
        cx.spawn(|this, mut cx| async move {
            let project_id = this.update(&mut cx, |this, _| {
                if let ProjectClientState::Local {
                    is_shared,
                    remote_id_rx,
                    ..
                } = &mut this.client_state
                {
                    *is_shared = false;
                    remote_id_rx
                        .borrow()
                        .ok_or_else(|| anyhow!("no project id"))
                } else {
                    Err(anyhow!("can't share a remote project"))
                }
            })?;

            rpc.send(proto::UnshareProject { project_id })?;
            this.update(&mut cx, |this, cx| {
                this.collaborators.clear();
                this.shared_buffers.clear();
                for worktree in this.worktrees(cx).collect::<Vec<_>>() {
                    worktree.update(cx, |worktree, _| {
                        worktree.as_local_mut().unwrap().unshare();
                    });
                }
                cx.notify()
            });
            Ok(())
        })
    }

    pub fn is_read_only(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local { .. } => false,
            ProjectClientState::Remote {
                sharing_has_stopped,
                ..
            } => *sharing_has_stopped,
        }
    }

    pub fn is_local(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local { .. } => true,
            ProjectClientState::Remote { .. } => false,
        }
    }

    pub fn open_buffer(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        self.open_buffer_with_options(path, OpenOptions::default(), cx)
    }

    pub fn open_buffer_with_options(
        &mut self,
        path: impl Into<ProjectPath>,
        options: OpenOptions,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let project_path = path.into();
        let worktree = if let Some(worktree) = self.worktree_for_id(project_path.worktree_id, cx) {
            worktree
        } else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };

        // If there is already a buffer for the given path, then return it.
        let existing_buffer = self.get_open_buffer(&project_path, cx);
        if let Some(existing_buffer) = existing_buffer {
            return Task::ready(Ok(existing_buffer));
        }

        let mut loading_watch = match self.fs_tasks.entry(project_path.clone()) {
            // If the given path is already being acted upon, then wait for that existing task to
            // complete. If it was a buffer loading task, the loaded buffer will be returned.
            hash_map::Entry::Occupied(e) => e.get().clone(),

            // Otherwise, record the fact that this path is now being loaded.
            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let load_buffer = if worktree.read(cx).is_local() {
                    self.open_local_buffer_with_options(&project_path.path, &worktree, options, cx)
                } else {
                    self.open_remote_buffer_with_options(&project_path.path, &worktree, options, cx)
                };

                cx.spawn(move |this, mut cx| async move {
                    let load_result = load_buffer.await;
                    *tx.borrow_mut() = Some(this.update(&mut cx, |this, _| {
                        // Record the fact that the buffer is no longer loading.
                        this.fs_tasks.remove(&project_path);
                        let buffer = load_result.map_err(Arc::new)?;
                        Ok(FsTaskOutput::LoadedBuffer(buffer))
                    }));
                })
                .detach();
                rx
            }
        };

        cx.foreground().spawn(async move {
            loop {
                if let Some(result) = loading_watch.borrow().as_ref() {
                    return match result {
                        Ok(out) => match out {
                            FsTaskOutput::LoadedBuffer(buffer) => Ok(buffer.clone()),
                            // TODO: should this really raise an error?
                            // for now, returning an Error, because all current calls to this
                            // method assume that a buffer should be returned, unless there was
                            // an error.
                            other => Err(anyhow!("expected loaded buffer, received {:?}", other)),
                        },
                        Err(error) => Err(anyhow!("{}", error)),
                    };
                }
                loading_watch.recv().await;
            }
        })
    }

    pub fn create_dir(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let project_path = path.into();
        let worktree = if let Some(worktree) = self.worktree_for_id(project_path.worktree_id, cx) {
            worktree
        } else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };

        let mut loading_watch = match self.fs_tasks.entry(project_path.clone()) {
            // If the given directory is already being created, then do nothing.
            hash_map::Entry::Occupied(e) => e.get().clone(),

            // Otherwise, record the fact that this path is now being created.
            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let create_dir_task = if worktree.read(cx).is_local() {
                    self.create_local_dir(&project_path.path, &worktree, cx)
                } else {
                    self.create_remote_dir(&project_path.path, &worktree, cx)
                };

                cx.spawn(move |this, mut cx| async move {
                    let create_dir_result = create_dir_task.await;
                    *tx.borrow_mut() = Some(this.update(&mut cx, |this, _| {
                        // Record the fact that the path is no longer being created.
                        this.fs_tasks.remove(&project_path);
                        create_dir_result?;
                        Ok(FsTaskOutput::CreatedDirectory)
                    }));
                })
                .detach();
                rx
            }
        };

        cx.foreground().spawn(async move {
            loop {
                if let Some(result) = loading_watch.borrow().as_ref() {
                    return match result {
                        Ok(_) => {
                            // We were trying to create a directory. If instead something was
                            // created/loaded there, then do nothing further.
                            // TODO: is this the right behaviour?
                            Ok(())
                        }
                        Err(error) => Err(anyhow!("{}", error)),
                    };
                }
                loading_watch.recv().await;
            }
        })
    }

    pub fn remove_path(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let project_path = path.into();
        let worktree = if let Some(worktree) = self.worktree_for_id(project_path.worktree_id, cx) {
            worktree
        } else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };

        let mut removed_watch = match self.fs_tasks.entry(project_path.clone()) {
            // If the given path is already being acted upon, then do nothing.
            hash_map::Entry::Occupied(e) => e.get().clone(),

            // Otherwise, record the fact that this path is now being removed.
            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let remove_task = if worktree.read(cx).is_local() {
                    self.remove_local_path(&project_path.path, &worktree, cx)
                } else {
                    self.remove_remote_path(&project_path.path, &worktree, cx)
                };

                cx.spawn(move |this, mut cx| async move {
                    let remove_result = remove_task.await;
                    *tx.borrow_mut() = Some(this.update(&mut cx, |this, _| {
                        // Record the fact that the path is no longer being acted upon.
                        this.fs_tasks.remove(&project_path);
                        remove_result?;
                        Ok(FsTaskOutput::RemovedPath)
                    }));
                })
                .detach();
                rx
            }
        };

        cx.foreground().spawn(async move {
            loop {
                if let Some(result) = removed_watch.borrow().as_ref() {
                    return match result {
                        Ok(_) => {
                            // We were trying to remove a path. If instead something was
                            // created/loaded there, then do nothing further
                            // TODO: is this the right behaviour?
                            Ok(())
                        }
                        Err(error) => Err(anyhow!("{}", error)),
                    };
                }
                removed_watch.recv().await;
            }
        })
    }

    fn open_local_buffer_with_options(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        options: OpenOptions,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let load_buffer = worktree.update(cx, |worktree, cx| {
            let worktree = worktree.as_local_mut().unwrap();
            worktree.load_buffer_with_options(path, options, cx)
        });
        let worktree = worktree.downgrade();
        cx.spawn(|this, mut cx| async move {
            let buffer = load_buffer.await?;
            let worktree = worktree
                .upgrade(&cx)
                .ok_or_else(|| anyhow!("worktree was removed"))?;
            this.update(&mut cx, |this, cx| {
                this.register_buffer(&buffer, Some(&worktree), cx)
            })?;
            Ok(buffer)
        })
    }

    fn create_local_dir(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let create_dir = worktree.update(cx, |worktree, cx| {
            let worktree = worktree.as_local_mut().unwrap();
            worktree.create_dir(path, cx)
        });
        cx.spawn(|this, mut cx| async move {
            create_dir.await?;
            this.update(&mut cx, |_, cx| cx.notify());
            Ok(())
        })
    }

    fn remove_local_path(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let create_dir = worktree.update(cx, |worktree, cx| {
            let worktree = worktree.as_local_mut().unwrap();
            worktree.remove_path(path, cx)
        });
        cx.spawn(|this, mut cx| async move {
            create_dir.await?;
            this.update(&mut cx, |_, cx| cx.notify());
            Ok(())
        })
    }

    fn open_remote_buffer_with_options(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        options: OpenOptions,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let rpc = self.client.clone();
        let project_id = self.remote_id().unwrap();
        let remote_worktree_id = worktree.read(cx).id();
        let path = path.clone();
        let path_string = path.to_string_lossy().to_string();
        cx.spawn(|this, mut cx| async move {
            let response = rpc
                .request(proto::OpenBuffer {
                    project_id,
                    worktree_id: remote_worktree_id.to_proto(),
                    path: path_string,
                    options: Some(options.into()),
                })
                .await?;
            let buffer = response.buffer.ok_or_else(|| anyhow!("missing buffer"))?;
            this.update(&mut cx, |this, cx| {
                this.deserialize_remote_buffer(buffer, cx)
            })
        })
    }

    fn create_remote_dir(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let rpc = self.client.clone();
        let project_id = self.remote_id().unwrap();
        let remote_worktree_id = worktree.read(cx).id();
        let path = path.as_os_str().as_bytes().into();
        cx.spawn(|this, mut cx| async move {
            let response = rpc
                .request(proto::CreateDir {
                    project_id,
                    worktree_id: remote_worktree_id.to_proto(),
                    path,
                })
                .await?;
            response
                .success
                .then(|| {})
                .ok_or_else(|| anyhow!("could not create directory"))?;
            this.update(&mut cx, |_, cx| cx.notify());
            Ok(())
        })
    }

    fn remove_remote_path(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let rpc = self.client.clone();
        let project_id = self.remote_id().unwrap();
        let remote_worktree_id = worktree.read(cx).id();
        let path = path.as_os_str().as_bytes().into();
        cx.spawn(|this, mut cx| async move {
            let response = rpc
                .request(proto::RemovePath {
                    project_id,
                    worktree_id: remote_worktree_id.to_proto(),
                    path,
                })
                .await?;
            response
                .success
                .then(|| {})
                .ok_or_else(|| anyhow!("could not remove path"))?;
            this.update(&mut cx, |_, cx| cx.notify());
            Ok(())
        })
    }

    pub fn save_buffer_as(
        &self,
        buffer: ModelHandle<Buffer>,
        abs_path: PathBuf,
        cx: &mut ModelContext<Project>,
    ) -> Task<Result<()>> {
        let worktree_task = self.find_or_create_local_worktree(&abs_path, false, cx);
        cx.spawn(|this, mut cx| async move {
            let (worktree, path) = worktree_task.await?;
            worktree
                .update(&mut cx, |worktree, cx| {
                    worktree
                        .as_local_mut()
                        .unwrap()
                        .save_buffer_as(buffer.clone(), path, cx)
                })
                .await?;
            this.update(&mut cx, |this, cx| {
                this.assign_language_to_buffer(&buffer, Some(&worktree), cx);
            });
            Ok(())
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn has_open_buffer(&self, path: impl Into<ProjectPath>, cx: &AppContext) -> bool {
        let path = path.into();
        if let Some(worktree) = self.worktree_for_id(path.worktree_id, cx) {
            self.open_buffers.iter().any(|(_, buffer)| {
                if let Some(buffer) = buffer.upgrade(cx) {
                    if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
                        if file.worktree == worktree && file.path() == &path.path {
                            return true;
                        }
                    }
                }
                false
            })
        } else {
            false
        }
    }

    fn get_open_buffer(
        &mut self,
        path: &ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Option<ModelHandle<Buffer>> {
        let mut result = None;
        let worktree = self.worktree_for_id(path.worktree_id, cx)?;
        self.open_buffers.retain(|_, buffer| {
            if let Some(buffer) = buffer.upgrade(cx) {
                if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
                    if file.worktree == worktree && file.path() == &path.path {
                        result = Some(buffer);
                    }
                }
                true
            } else {
                false
            }
        });
        result
    }

    fn register_buffer(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        worktree: Option<&ModelHandle<Worktree>>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if self
            .open_buffers
            .insert(buffer.read(cx).remote_id() as usize, buffer.downgrade())
            .is_some()
        {
            return Err(anyhow!("registered the same buffer twice"));
        }
        self.assign_language_to_buffer(&buffer, worktree, cx);
        Ok(())
    }

    fn assign_language_to_buffer(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        worktree: Option<&ModelHandle<Worktree>>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let (path, full_path) = {
            let file = buffer.read(cx).file()?;
            (file.path().clone(), file.full_path(cx))
        };

        // If the buffer has a language, set it and start/assign the language server
        if let Some(language) = self.languages.select_language(&full_path) {
            buffer.update(cx, |buffer, cx| {
                buffer.set_language(Some(language.clone()), cx);
            });

            // For local worktrees, start a language server if needed.
            // Also assign the language server and any previously stored diagnostics to the buffer.
            if let Some(local_worktree) = worktree.and_then(|w| w.read(cx).as_local()) {
                let worktree_id = local_worktree.id();
                let worktree_abs_path = local_worktree.abs_path().clone();

                let language_server = match self
                    .language_servers
                    .entry((worktree_id, language.name().to_string()))
                {
                    hash_map::Entry::Occupied(e) => Some(e.get().clone()),
                    hash_map::Entry::Vacant(e) => Self::start_language_server(
                        self.client.clone(),
                        language.clone(),
                        &worktree_abs_path,
                        cx,
                    )
                    .map(|server| e.insert(server).clone()),
                };

                buffer.update(cx, |buffer, cx| {
                    buffer.set_language_server(language_server, cx);
                });
            }
        }

        if let Some(local_worktree) = worktree.and_then(|w| w.read(cx).as_local()) {
            if let Some(diagnostics) = local_worktree.diagnostics_for_path(&path) {
                buffer.update(cx, |buffer, cx| {
                    buffer.update_diagnostics(None, diagnostics, cx).log_err();
                });
            }
        }

        None
    }

    fn start_language_server(
        rpc: Arc<Client>,
        language: Arc<Language>,
        worktree_path: &Path,
        cx: &mut ModelContext<Self>,
    ) -> Option<Arc<LanguageServer>> {
        enum LspEvent {
            DiagnosticsStart,
            DiagnosticsUpdate(lsp::PublishDiagnosticsParams),
            DiagnosticsFinish,
        }

        let language_server = language
            .start_server(worktree_path, cx)
            .log_err()
            .flatten()?;
        let disk_based_sources = language
            .disk_based_diagnostic_sources()
            .cloned()
            .unwrap_or_default();
        let disk_based_diagnostics_progress_token =
            language.disk_based_diagnostics_progress_token().cloned();
        let has_disk_based_diagnostic_progress_token =
            disk_based_diagnostics_progress_token.is_some();
        let (diagnostics_tx, diagnostics_rx) = smol::channel::unbounded();

        // Listen for `PublishDiagnostics` notifications.
        language_server
            .on_notification::<lsp::notification::PublishDiagnostics, _>({
                let diagnostics_tx = diagnostics_tx.clone();
                move |params| {
                    if !has_disk_based_diagnostic_progress_token {
                        block_on(diagnostics_tx.send(LspEvent::DiagnosticsStart)).ok();
                    }
                    block_on(diagnostics_tx.send(LspEvent::DiagnosticsUpdate(params))).ok();
                    if !has_disk_based_diagnostic_progress_token {
                        block_on(diagnostics_tx.send(LspEvent::DiagnosticsFinish)).ok();
                    }
                }
            })
            .detach();

        // Listen for `Progress` notifications. Send an event when the language server
        // transitions between running jobs and not running any jobs.
        let mut running_jobs_for_this_server: i32 = 0;
        language_server
            .on_notification::<lsp::notification::Progress, _>(move |params| {
                let token = match params.token {
                    lsp::NumberOrString::Number(_) => None,
                    lsp::NumberOrString::String(token) => Some(token),
                };

                if token == disk_based_diagnostics_progress_token {
                    match params.value {
                        lsp::ProgressParamsValue::WorkDone(progress) => match progress {
                            lsp::WorkDoneProgress::Begin(_) => {
                                running_jobs_for_this_server += 1;
                                if running_jobs_for_this_server == 1 {
                                    block_on(diagnostics_tx.send(LspEvent::DiagnosticsStart)).ok();
                                }
                            }
                            lsp::WorkDoneProgress::End(_) => {
                                running_jobs_for_this_server -= 1;
                                if running_jobs_for_this_server == 0 {
                                    block_on(diagnostics_tx.send(LspEvent::DiagnosticsFinish)).ok();
                                }
                            }
                            _ => {}
                        },
                    }
                }
            })
            .detach();

        // Process all the LSP events.
        cx.spawn_weak(|this, mut cx| async move {
            while let Ok(message) = diagnostics_rx.recv().await {
                let this = cx.read(|cx| this.upgrade(cx))?;
                match message {
                    LspEvent::DiagnosticsStart => {
                        this.update(&mut cx, |this, cx| {
                            this.disk_based_diagnostics_started(cx);
                            if let Some(project_id) = this.remote_id() {
                                rpc.send(proto::DiskBasedDiagnosticsUpdating { project_id })
                                    .log_err();
                            }
                        });
                    }
                    LspEvent::DiagnosticsUpdate(mut params) => {
                        language.process_diagnostics(&mut params);
                        this.update(&mut cx, |this, cx| {
                            this.update_diagnostics(params, &disk_based_sources, cx)
                                .log_err();
                        });
                    }
                    LspEvent::DiagnosticsFinish => {
                        this.update(&mut cx, |this, cx| {
                            this.disk_based_diagnostics_finished(cx);
                            if let Some(project_id) = this.remote_id() {
                                rpc.send(proto::DiskBasedDiagnosticsUpdated { project_id })
                                    .log_err();
                            }
                        });
                    }
                }
            }
            Some(())
        })
        .detach();

        Some(language_server)
    }

    pub fn update_diagnostics(
        &mut self,
        params: lsp::PublishDiagnosticsParams,
        disk_based_sources: &HashSet<String>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let abs_path = params
            .uri
            .to_file_path()
            .map_err(|_| anyhow!("URI is not a file"))?;
        let mut next_group_id = 0;
        let mut diagnostics = Vec::default();
        let mut primary_diagnostic_group_ids = HashMap::default();
        let mut sources_by_group_id = HashMap::default();
        let mut supporting_diagnostic_severities = HashMap::default();
        for diagnostic in &params.diagnostics {
            let source = diagnostic.source.as_ref();
            let code = diagnostic.code.as_ref().map(|code| match code {
                lsp::NumberOrString::Number(code) => code.to_string(),
                lsp::NumberOrString::String(code) => code.clone(),
            });
            let range = range_from_lsp(diagnostic.range);
            let is_supporting = diagnostic
                .related_information
                .as_ref()
                .map_or(false, |infos| {
                    infos.iter().any(|info| {
                        primary_diagnostic_group_ids.contains_key(&(
                            source,
                            code.clone(),
                            range_from_lsp(info.location.range),
                        ))
                    })
                });

            if is_supporting {
                if let Some(severity) = diagnostic.severity {
                    supporting_diagnostic_severities
                        .insert((source, code.clone(), range), severity);
                }
            } else {
                let group_id = post_inc(&mut next_group_id);
                let is_disk_based =
                    source.map_or(false, |source| disk_based_sources.contains(source));

                sources_by_group_id.insert(group_id, source);
                primary_diagnostic_group_ids
                    .insert((source, code.clone(), range.clone()), group_id);

                diagnostics.push(DiagnosticEntry {
                    range,
                    diagnostic: Diagnostic {
                        code: code.clone(),
                        severity: diagnostic.severity.unwrap_or(DiagnosticSeverity::ERROR),
                        message: diagnostic.message.clone(),
                        group_id,
                        is_primary: true,
                        is_valid: true,
                        is_disk_based,
                    },
                });
                if let Some(infos) = &diagnostic.related_information {
                    for info in infos {
                        if info.location.uri == params.uri && !info.message.is_empty() {
                            let range = range_from_lsp(info.location.range);
                            diagnostics.push(DiagnosticEntry {
                                range,
                                diagnostic: Diagnostic {
                                    code: code.clone(),
                                    severity: DiagnosticSeverity::INFORMATION,
                                    message: info.message.clone(),
                                    group_id,
                                    is_primary: false,
                                    is_valid: true,
                                    is_disk_based,
                                },
                            });
                        }
                    }
                }
            }
        }

        for entry in &mut diagnostics {
            let diagnostic = &mut entry.diagnostic;
            if !diagnostic.is_primary {
                let source = *sources_by_group_id.get(&diagnostic.group_id).unwrap();
                if let Some(&severity) = supporting_diagnostic_severities.get(&(
                    source,
                    diagnostic.code.clone(),
                    entry.range.clone(),
                )) {
                    diagnostic.severity = severity;
                }
            }
        }

        self.update_diagnostic_entries(abs_path, params.version, diagnostics, cx)?;
        Ok(())
    }

    pub fn update_diagnostic_entries(
        &mut self,
        abs_path: PathBuf,
        version: Option<i32>,
        diagnostics: Vec<DiagnosticEntry<PointUtf16>>,
        cx: &mut ModelContext<Project>,
    ) -> Result<(), anyhow::Error> {
        let (worktree, relative_path) = self
            .find_local_worktree(&abs_path, cx)
            .ok_or_else(|| anyhow!("no worktree found for diagnostics"))?;
        let project_path = ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: relative_path.into(),
        };

        for buffer in self.open_buffers.values() {
            if let Some(buffer) = buffer.upgrade(cx) {
                if buffer
                    .read(cx)
                    .file()
                    .map_or(false, |file| *file.path() == project_path.path)
                {
                    buffer.update(cx, |buffer, cx| {
                        buffer.update_diagnostics(version, diagnostics.clone(), cx)
                    })?;
                    break;
                }
            }
        }
        worktree.update(cx, |worktree, cx| {
            worktree
                .as_local_mut()
                .ok_or_else(|| anyhow!("not a local worktree"))?
                .update_diagnostics(project_path.path.clone(), diagnostics, cx)
        })?;
        cx.emit(Event::DiagnosticsUpdated(project_path));
        Ok(())
    }

    pub fn definition<T: ToOffset>(
        &self,
        source_buffer_handle: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Definition>>> {
        let source_buffer_handle = source_buffer_handle.clone();
        let source_buffer = source_buffer_handle.read(cx);
        let worktree;
        let buffer_abs_path;
        if let Some(file) = File::from_dyn(source_buffer.file()) {
            worktree = file.worktree.clone();
            buffer_abs_path = file.as_local().map(|f| f.abs_path(cx));
        } else {
            return Task::ready(Err(anyhow!("buffer does not belong to any worktree")));
        };

        if worktree.read(cx).as_local().is_some() {
            let point = source_buffer.offset_to_point_utf16(position.to_offset(source_buffer));
            let buffer_abs_path = buffer_abs_path.unwrap();
            let lang_name;
            let lang_server;
            if let Some(lang) = source_buffer.language() {
                lang_name = lang.name().to_string();
                if let Some(server) = self
                    .language_servers
                    .get(&(worktree.read(cx).id(), lang_name.clone()))
                {
                    lang_server = server.clone();
                } else {
                    return Task::ready(Err(anyhow!("buffer does not have a language server")));
                };
            } else {
                return Task::ready(Err(anyhow!("buffer does not have a language")));
            }

            cx.spawn(|this, mut cx| async move {
                let response = lang_server
                    .request::<lsp::request::GotoDefinition>(lsp::GotoDefinitionParams {
                        text_document_position_params: lsp::TextDocumentPositionParams {
                            text_document: lsp::TextDocumentIdentifier::new(
                                lsp::Url::from_file_path(&buffer_abs_path).unwrap(),
                            ),
                            position: lsp::Position::new(point.row, point.column),
                        },
                        work_done_progress_params: Default::default(),
                        partial_result_params: Default::default(),
                    })
                    .await?;

                let mut definitions = Vec::new();
                if let Some(response) = response {
                    let mut unresolved_locations = Vec::new();
                    match response {
                        lsp::GotoDefinitionResponse::Scalar(loc) => {
                            unresolved_locations.push((loc.uri, loc.range));
                        }
                        lsp::GotoDefinitionResponse::Array(locs) => {
                            unresolved_locations.extend(locs.into_iter().map(|l| (l.uri, l.range)));
                        }
                        lsp::GotoDefinitionResponse::Link(links) => {
                            unresolved_locations.extend(
                                links
                                    .into_iter()
                                    .map(|l| (l.target_uri, l.target_selection_range)),
                            );
                        }
                    }

                    for (target_uri, target_range) in unresolved_locations {
                        let abs_path = target_uri
                            .to_file_path()
                            .map_err(|_| anyhow!("invalid target path"))?;

                        let (worktree, relative_path) = if let Some(result) =
                            this.read_with(&cx, |this, cx| this.find_local_worktree(&abs_path, cx))
                        {
                            result
                        } else {
                            let worktree = this
                                .update(&mut cx, |this, cx| {
                                    this.create_local_worktree(&abs_path, true, cx)
                                })
                                .await?;
                            this.update(&mut cx, |this, cx| {
                                this.language_servers.insert(
                                    (worktree.read(cx).id(), lang_name.clone()),
                                    lang_server.clone(),
                                );
                            });
                            (worktree, PathBuf::new())
                        };

                        let project_path = ProjectPath {
                            worktree_id: worktree.read_with(&cx, |worktree, _| worktree.id()),
                            path: relative_path.into(),
                        };
                        let target_buffer_handle = this
                            .update(&mut cx, |this, cx| this.open_buffer(project_path, cx))
                            .await?;
                        cx.read(|cx| {
                            let target_buffer = target_buffer_handle.read(cx);
                            let target_start = target_buffer
                                .clip_point_utf16(target_range.start.to_point_utf16(), Bias::Left);
                            let target_end = target_buffer
                                .clip_point_utf16(target_range.end.to_point_utf16(), Bias::Left);
                            definitions.push(Definition {
                                target_buffer: target_buffer_handle,
                                target_range: target_buffer.anchor_after(target_start)
                                    ..target_buffer.anchor_before(target_end),
                            });
                        });
                    }
                }

                Ok(definitions)
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
            let request = proto::GetDefinition {
                project_id,
                buffer_id: source_buffer.remote_id(),
                position: Some(serialize_anchor(&source_buffer.anchor_before(position))),
            };
            cx.spawn(|this, mut cx| async move {
                let response = client.request(request).await?;
                this.update(&mut cx, |this, cx| {
                    let mut definitions = Vec::new();
                    for definition in response.definitions {
                        let target_buffer = this.deserialize_remote_buffer(
                            definition.buffer.ok_or_else(|| anyhow!("missing buffer"))?,
                            cx,
                        )?;
                        let target_start = definition
                            .target_start
                            .and_then(deserialize_anchor)
                            .ok_or_else(|| anyhow!("missing target start"))?;
                        let target_end = definition
                            .target_end
                            .and_then(deserialize_anchor)
                            .ok_or_else(|| anyhow!("missing target end"))?;
                        definitions.push(Definition {
                            target_buffer,
                            target_range: target_start..target_end,
                        })
                    }

                    Ok(definitions)
                })
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    pub fn find_or_create_local_worktree(
        &self,
        abs_path: impl AsRef<Path>,
        weak: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(ModelHandle<Worktree>, PathBuf)>> {
        let abs_path = abs_path.as_ref();
        if let Some((tree, relative_path)) = self.find_local_worktree(abs_path, cx) {
            Task::ready(Ok((tree.clone(), relative_path.into())))
        } else {
            let worktree = self.create_local_worktree(abs_path, weak, cx);
            cx.foreground()
                .spawn(async move { Ok((worktree.await?, PathBuf::new())) })
        }
    }

    fn find_local_worktree(
        &self,
        abs_path: &Path,
        cx: &AppContext,
    ) -> Option<(ModelHandle<Worktree>, PathBuf)> {
        for tree in self.worktrees(cx) {
            if let Some(relative_path) = tree
                .read(cx)
                .as_local()
                .and_then(|t| abs_path.strip_prefix(t.abs_path()).ok())
            {
                return Some((tree.clone(), relative_path.into()));
            }
        }
        None
    }

    pub fn is_shared(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local { is_shared, .. } => *is_shared,
            ProjectClientState::Remote { .. } => false,
        }
    }

    fn create_local_worktree(
        &self,
        abs_path: impl AsRef<Path>,
        weak: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        let fs = self.fs.clone();
        let client = self.client.clone();
        let path = Arc::from(abs_path.as_ref());
        cx.spawn(|project, mut cx| async move {
            let worktree = Worktree::local(client.clone(), path, weak, fs, &mut cx).await?;

            let (remote_project_id, is_shared) = project.update(&mut cx, |project, cx| {
                project.add_worktree(&worktree, cx);
                (project.remote_id(), project.is_shared())
            });

            if let Some(project_id) = remote_project_id {
                worktree
                    .update(&mut cx, |worktree, cx| {
                        worktree.as_local_mut().unwrap().register(project_id, cx)
                    })
                    .await?;
                if is_shared {
                    worktree
                        .update(&mut cx, |worktree, cx| {
                            worktree.as_local_mut().unwrap().share(project_id, cx)
                        })
                        .await?;
                }
            }

            Ok(worktree)
        })
    }

    pub fn remove_worktree(&mut self, id: WorktreeId, cx: &mut ModelContext<Self>) {
        self.worktrees.retain(|worktree| {
            worktree
                .upgrade(cx)
                .map_or(false, |w| w.read(cx).id() != id)
        });
        cx.notify();
    }

    fn add_worktree(&mut self, worktree: &ModelHandle<Worktree>, cx: &mut ModelContext<Self>) {
        cx.observe(&worktree, |_, _, cx| cx.notify()).detach();
        if worktree.read(cx).is_local() {
            cx.subscribe(&worktree, |this, worktree, _, cx| {
                this.update_local_worktree_buffers(worktree, cx);
            })
            .detach();
        }

        let push_weak_handle = {
            let worktree = worktree.read(cx);
            worktree.is_local() && worktree.is_weak()
        };
        if push_weak_handle {
            cx.observe_release(&worktree, |this, cx| {
                this.worktrees
                    .retain(|worktree| worktree.upgrade(cx).is_some());
                cx.notify();
            })
            .detach();
            self.worktrees
                .push(WorktreeHandle::Weak(worktree.downgrade()));
        } else {
            self.worktrees
                .push(WorktreeHandle::Strong(worktree.clone()));
        }
        cx.notify();
    }

    fn update_local_worktree_buffers(
        &mut self,
        worktree_handle: ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();
        let mut buffers_to_delete = Vec::new();
        for (buffer_id, buffer) in &self.open_buffers {
            if let Some(buffer) = buffer.upgrade(cx) {
                buffer.update(cx, |buffer, cx| {
                    if let Some(old_file) = File::from_dyn(buffer.file()) {
                        if old_file.worktree != worktree_handle {
                            return;
                        }

                        let new_file = if let Some(entry) = old_file
                            .entry_id
                            .and_then(|entry_id| snapshot.entry_for_id(entry_id))
                        {
                            File {
                                is_local: true,
                                entry_id: Some(entry.id),
                                mtime: entry.mtime,
                                path: entry.path.clone(),
                                worktree: worktree_handle.clone(),
                            }
                        } else if let Some(entry) =
                            snapshot.entry_for_path(old_file.path().as_ref())
                        {
                            File {
                                is_local: true,
                                entry_id: Some(entry.id),
                                mtime: entry.mtime,
                                path: entry.path.clone(),
                                worktree: worktree_handle.clone(),
                            }
                        } else {
                            File {
                                is_local: true,
                                entry_id: None,
                                path: old_file.path().clone(),
                                mtime: old_file.mtime(),
                                worktree: worktree_handle.clone(),
                            }
                        };

                        if let Some(project_id) = self.remote_id() {
                            self.client
                                .send(proto::UpdateBufferFile {
                                    project_id,
                                    buffer_id: *buffer_id as u64,
                                    file: Some(new_file.to_proto()),
                                })
                                .log_err();
                        }
                        buffer.file_updated(Box::new(new_file), cx).detach();
                    }
                });
            } else {
                buffers_to_delete.push(*buffer_id);
            }
        }

        for buffer_id in buffers_to_delete {
            self.open_buffers.remove(&buffer_id);
        }
    }

    pub fn set_active_path(&mut self, entry: Option<ProjectPath>, cx: &mut ModelContext<Self>) {
        let new_active_entry = entry.and_then(|project_path| {
            let worktree = self.worktree_for_id(project_path.worktree_id, cx)?;
            let entry = worktree.read(cx).entry_for_path(project_path.path)?;
            Some(ProjectEntry {
                worktree_id: project_path.worktree_id,
                entry_id: entry.id,
            })
        });
        if new_active_entry != self.active_entry {
            self.active_entry = new_active_entry;
            cx.emit(Event::ActiveEntryChanged(new_active_entry));
        }
    }

    pub fn is_running_disk_based_diagnostics(&self) -> bool {
        self.language_servers_with_diagnostics_running > 0
    }

    pub fn diagnostic_summary(&self, cx: &AppContext) -> DiagnosticSummary {
        let mut summary = DiagnosticSummary::default();
        for (_, path_summary) in self.diagnostic_summaries(cx) {
            summary.error_count += path_summary.error_count;
            summary.warning_count += path_summary.warning_count;
            summary.info_count += path_summary.info_count;
            summary.hint_count += path_summary.hint_count;
        }
        summary
    }

    pub fn diagnostic_summaries<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = (ProjectPath, DiagnosticSummary)> + 'a {
        self.worktrees(cx).flat_map(move |worktree| {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            worktree
                .diagnostic_summaries()
                .map(move |(path, summary)| (ProjectPath { worktree_id, path }, summary))
        })
    }

    pub fn disk_based_diagnostics_started(&mut self, cx: &mut ModelContext<Self>) {
        self.language_servers_with_diagnostics_running += 1;
        if self.language_servers_with_diagnostics_running == 1 {
            cx.emit(Event::DiskBasedDiagnosticsStarted);
        }
    }

    pub fn disk_based_diagnostics_finished(&mut self, cx: &mut ModelContext<Self>) {
        cx.emit(Event::DiskBasedDiagnosticsUpdated);
        self.language_servers_with_diagnostics_running -= 1;
        if self.language_servers_with_diagnostics_running == 0 {
            cx.emit(Event::DiskBasedDiagnosticsFinished);
        }
    }

    pub fn active_entry(&self) -> Option<ProjectEntry> {
        self.active_entry
    }

    // RPC message handlers

    fn handle_unshare_project(
        &mut self,
        _: TypedEnvelope<proto::UnshareProject>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if let ProjectClientState::Remote {
            sharing_has_stopped,
            ..
        } = &mut self.client_state
        {
            *sharing_has_stopped = true;
            self.collaborators.clear();
            cx.notify();
            Ok(())
        } else {
            unreachable!()
        }
    }

    fn handle_add_collaborator(
        &mut self,
        mut envelope: TypedEnvelope<proto::AddProjectCollaborator>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let user_store = self.user_store.clone();
        let collaborator = envelope
            .payload
            .collaborator
            .take()
            .ok_or_else(|| anyhow!("empty collaborator"))?;

        cx.spawn(|this, mut cx| {
            async move {
                let collaborator =
                    Collaborator::from_proto(collaborator, &user_store, &mut cx).await?;
                this.update(&mut cx, |this, cx| {
                    this.collaborators
                        .insert(collaborator.peer_id, collaborator);
                    cx.notify();
                });
                Ok(())
            }
            .log_err()
        })
        .detach();

        Ok(())
    }

    fn handle_remove_collaborator(
        &mut self,
        envelope: TypedEnvelope<proto::RemoveProjectCollaborator>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let peer_id = PeerId(envelope.payload.peer_id);
        let replica_id = self
            .collaborators
            .remove(&peer_id)
            .ok_or_else(|| anyhow!("unknown peer {:?}", peer_id))?
            .replica_id;
        self.shared_buffers.remove(&peer_id);
        for (_, buffer) in &self.open_buffers {
            if let Some(buffer) = buffer.upgrade(cx) {
                buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
            }
        }
        cx.notify();
        Ok(())
    }

    fn handle_share_worktree(
        &mut self,
        envelope: TypedEnvelope<proto::ShareWorktree>,
        client: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let remote_id = self.remote_id().ok_or_else(|| anyhow!("invalid project"))?;
        let replica_id = self.replica_id();
        let worktree = envelope
            .payload
            .worktree
            .ok_or_else(|| anyhow!("invalid worktree"))?;
        let (worktree, load_task) = Worktree::remote(remote_id, replica_id, worktree, client, cx);
        self.add_worktree(&worktree, cx);
        load_task.detach();
        Ok(())
    }

    fn handle_unregister_worktree(
        &mut self,
        envelope: TypedEnvelope<proto::UnregisterWorktree>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        self.remove_worktree(worktree_id, cx);
        Ok(())
    }

    fn handle_update_worktree(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateWorktree>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            worktree.update(cx, |worktree, cx| {
                let worktree = worktree.as_remote_mut().unwrap();
                worktree.update_from_remote(envelope, cx)
            })?;
        }
        Ok(())
    }

    fn handle_update_diagnostic_summary(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateDiagnosticSummary>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            if let Some(summary) = envelope.payload.summary {
                let project_path = ProjectPath {
                    worktree_id,
                    path: Path::new(&summary.path).into(),
                };
                worktree.update(cx, |worktree, _| {
                    worktree
                        .as_remote_mut()
                        .unwrap()
                        .update_diagnostic_summary(project_path.path.clone(), &summary);
                });
                cx.emit(Event::DiagnosticsUpdated(project_path));
            }
        }
        Ok(())
    }

    fn handle_disk_based_diagnostics_updating(
        &mut self,
        _: TypedEnvelope<proto::DiskBasedDiagnosticsUpdating>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.disk_based_diagnostics_started(cx);
        Ok(())
    }

    fn handle_disk_based_diagnostics_updated(
        &mut self,
        _: TypedEnvelope<proto::DiskBasedDiagnosticsUpdated>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.disk_based_diagnostics_finished(cx);
        Ok(())
    }

    pub fn handle_update_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let payload = envelope.payload.clone();
        let buffer_id = payload.buffer_id as usize;
        let ops = payload
            .operations
            .into_iter()
            .map(|op| language::proto::deserialize_operation(op))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(buffer) = self.open_buffers.get_mut(&buffer_id) {
            if let Some(buffer) = buffer.upgrade(cx) {
                buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
            }
        }
        Ok(())
    }

    pub fn handle_update_buffer_file(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateBufferFile>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let payload = envelope.payload.clone();
        let buffer_id = payload.buffer_id as usize;
        let file = payload.file.ok_or_else(|| anyhow!("invalid file"))?;
        let worktree = self
            .worktree_for_id(WorktreeId::from_proto(file.worktree_id), cx)
            .ok_or_else(|| anyhow!("no such worktree"))?;
        let file = File::from_proto(file, worktree.clone(), cx)?;
        let buffer = self
            .open_buffers
            .get_mut(&buffer_id)
            .and_then(|b| b.upgrade(cx))
            .ok_or_else(|| anyhow!("no such buffer"))?;
        buffer.update(cx, |buffer, cx| {
            buffer.file_updated(Box::new(file), cx).detach();
        });

        Ok(())
    }

    pub fn handle_save_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let sender_id = envelope.original_sender_id()?;
        let project_id = self.remote_id().ok_or_else(|| anyhow!("not connected"))?;
        let buffer = self
            .shared_buffers
            .get(&sender_id)
            .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
            .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;
        let receipt = envelope.receipt();
        let buffer_id = envelope.payload.buffer_id;
        let save = cx.spawn(|_, mut cx| async move {
            buffer.update(&mut cx, |buffer, cx| buffer.save(cx)).await
        });

        cx.background()
            .spawn(
                async move {
                    let (version, mtime) = save.await?;

                    rpc.respond(
                        receipt,
                        proto::BufferSaved {
                            project_id,
                            buffer_id,
                            version: (&version).into(),
                            mtime: Some(mtime.into()),
                        },
                    )?;

                    Ok(())
                }
                .log_err(),
            )
            .detach();
        Ok(())
    }

    pub fn handle_format_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::FormatBuffer>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let receipt = envelope.receipt();
        let sender_id = envelope.original_sender_id()?;
        let buffer = self
            .shared_buffers
            .get(&sender_id)
            .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
            .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;
        cx.spawn(|_, mut cx| async move {
            let format = buffer.update(&mut cx, |buffer, cx| buffer.format(cx)).await;
            // We spawn here in order to enqueue the sending of `Ack` *after* transmission of edits
            // associated with formatting.
            cx.spawn(|_| async move {
                match format {
                    Ok(()) => rpc.respond(receipt, proto::Ack {})?,
                    Err(error) => rpc.respond_with_error(
                        receipt,
                        proto::Error {
                            message: error.to_string(),
                        },
                    )?,
                }
                Ok::<_, anyhow::Error>(())
            })
            .await
            .log_err();
        })
        .detach();
        Ok(())
    }

    fn handle_get_completions(
        &mut self,
        envelope: TypedEnvelope<proto::GetCompletions>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let receipt = envelope.receipt();
        let sender_id = envelope.original_sender_id()?;
        let buffer = self
            .shared_buffers
            .get(&sender_id)
            .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
            .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;
        let position = envelope
            .payload
            .position
            .and_then(language::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        cx.spawn(|_, mut cx| async move {
            match buffer
                .update(&mut cx, |buffer, cx| buffer.completions(position, cx))
                .await
            {
                Ok(completions) => rpc.respond(
                    receipt,
                    proto::GetCompletionsResponse {
                        completions: completions
                            .iter()
                            .map(language::proto::serialize_completion)
                            .collect(),
                    },
                ),
                Err(error) => rpc.respond_with_error(
                    receipt,
                    proto::Error {
                        message: error.to_string(),
                    },
                ),
            }
        })
        .detach_and_log_err(cx);
        Ok(())
    }

    fn handle_apply_additional_edits_for_completion(
        &mut self,
        envelope: TypedEnvelope<proto::ApplyCompletionAdditionalEdits>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let receipt = envelope.receipt();
        let sender_id = envelope.original_sender_id()?;
        let buffer = self
            .shared_buffers
            .get(&sender_id)
            .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
            .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;
        let language = buffer.read(cx).language();
        let completion = language::proto::deserialize_completion(
            envelope
                .payload
                .completion
                .ok_or_else(|| anyhow!("invalid position"))?,
            language,
        )?;
        cx.spawn(|_, mut cx| async move {
            match buffer
                .update(&mut cx, |buffer, cx| {
                    buffer.apply_additional_edits_for_completion(completion, false, cx)
                })
                .await
            {
                Ok(edit_ids) => rpc.respond(
                    receipt,
                    proto::ApplyCompletionAdditionalEditsResponse {
                        additional_edits: edit_ids
                            .into_iter()
                            .map(|edit_id| proto::AdditionalEdit {
                                replica_id: edit_id.replica_id as u32,
                                local_timestamp: edit_id.value,
                            })
                            .collect(),
                    },
                ),
                Err(error) => rpc.respond_with_error(
                    receipt,
                    proto::Error {
                        message: error.to_string(),
                    },
                ),
            }
        })
        .detach_and_log_err(cx);
        Ok(())
    }

    pub fn handle_get_definition(
        &mut self,
        envelope: TypedEnvelope<proto::GetDefinition>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let receipt = envelope.receipt();
        let sender_id = envelope.original_sender_id()?;
        let source_buffer = self
            .shared_buffers
            .get(&sender_id)
            .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
            .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;
        let position = envelope
            .payload
            .position
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        if !source_buffer.read(cx).can_resolve(&position) {
            return Err(anyhow!("cannot resolve position"));
        }

        let definitions = self.definition(&source_buffer, position, cx);
        cx.spawn(|this, mut cx| async move {
            let definitions = definitions.await?;
            let mut response = proto::GetDefinitionResponse {
                definitions: Default::default(),
            };
            this.update(&mut cx, |this, cx| {
                for definition in definitions {
                    let buffer =
                        this.serialize_buffer_for_peer(&definition.target_buffer, sender_id, cx);
                    response.definitions.push(proto::Definition {
                        target_start: Some(serialize_anchor(&definition.target_range.start)),
                        target_end: Some(serialize_anchor(&definition.target_range.end)),
                        buffer: Some(buffer),
                    });
                }
            });
            rpc.respond(receipt, response)?;
            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);

        Ok(())
    }

    pub fn handle_open_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::OpenBuffer>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<()> {
        let receipt = envelope.receipt();
        let peer_id = envelope.original_sender_id()?;
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let open_buffer = self.open_buffer(
            ProjectPath {
                worktree_id,
                path: PathBuf::from(envelope.payload.path).into(),
            },
            cx,
        );
        cx.spawn(|this, mut cx| {
            async move {
                let buffer = open_buffer.await?;
                let buffer = this.update(&mut cx, |this, cx| {
                    this.serialize_buffer_for_peer(&buffer, peer_id, cx)
                });
                rpc.respond(
                    receipt,
                    proto::OpenBufferResponse {
                        buffer: Some(buffer),
                    },
                )
            }
            .log_err()
        })
        .detach();
        Ok(())
    }

    fn serialize_buffer_for_peer(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        peer_id: PeerId,
        cx: &AppContext,
    ) -> proto::Buffer {
        let buffer_id = buffer.read(cx).remote_id();
        let shared_buffers = self.shared_buffers.entry(peer_id).or_default();
        match shared_buffers.entry(buffer_id) {
            hash_map::Entry::Occupied(_) => proto::Buffer {
                variant: Some(proto::buffer::Variant::Id(buffer_id)),
            },
            hash_map::Entry::Vacant(entry) => {
                entry.insert(buffer.clone());
                proto::Buffer {
                    variant: Some(proto::buffer::Variant::State(buffer.read(cx).to_proto())),
                }
            }
        }
    }

    fn deserialize_remote_buffer(
        &mut self,
        buffer: proto::Buffer,
        cx: &mut ModelContext<Self>,
    ) -> Result<ModelHandle<Buffer>> {
        match buffer.variant.ok_or_else(|| anyhow!("missing buffer"))? {
            proto::buffer::Variant::Id(id) => self
                .open_buffers
                .get(&(id as usize))
                .and_then(|buffer| buffer.upgrade(cx))
                .ok_or_else(|| anyhow!("no buffer exists for id {}", id)),
            proto::buffer::Variant::State(mut buffer) => {
                let mut buffer_worktree = None;
                let mut buffer_file = None;
                if let Some(file) = buffer.file.take() {
                    let worktree_id = WorktreeId::from_proto(file.worktree_id);
                    let worktree = self
                        .worktree_for_id(worktree_id, cx)
                        .ok_or_else(|| anyhow!("no worktree found for id {}", file.worktree_id))?;
                    buffer_file = Some(Box::new(File::from_proto(file, worktree.clone(), cx)?)
                        as Box<dyn language::File>);
                    buffer_worktree = Some(worktree);
                }

                let buffer = cx.add_model(|cx| {
                    Buffer::from_proto(self.replica_id(), buffer, buffer_file, cx).unwrap()
                });
                self.register_buffer(&buffer, buffer_worktree.as_ref(), cx)?;
                Ok(buffer)
            }
        }
    }

    pub fn handle_close_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<()> {
        if let Some(shared_buffers) = self.shared_buffers.get_mut(&envelope.original_sender_id()?) {
            shared_buffers.remove(&envelope.payload.buffer_id);
            cx.notify();
        }
        Ok(())
    }

    pub fn handle_buffer_saved(
        &mut self,
        envelope: TypedEnvelope<proto::BufferSaved>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let payload = envelope.payload.clone();
        let buffer = self
            .open_buffers
            .get(&(payload.buffer_id as usize))
            .and_then(|buffer| buffer.upgrade(cx));
        if let Some(buffer) = buffer {
            buffer.update(cx, |buffer, cx| {
                let version = payload.version.try_into()?;
                let mtime = payload
                    .mtime
                    .ok_or_else(|| anyhow!("missing mtime"))?
                    .into();
                buffer.did_save(version, mtime, None, cx);
                Result::<_, anyhow::Error>::Ok(())
            })?;
        }
        Ok(())
    }

    pub fn handle_buffer_reloaded(
        &mut self,
        envelope: TypedEnvelope<proto::BufferReloaded>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let payload = envelope.payload.clone();
        let buffer = self
            .open_buffers
            .get(&(payload.buffer_id as usize))
            .and_then(|buffer| buffer.upgrade(cx));
        if let Some(buffer) = buffer {
            buffer.update(cx, |buffer, cx| {
                let version = payload.version.try_into()?;
                let mtime = payload
                    .mtime
                    .ok_or_else(|| anyhow!("missing mtime"))?
                    .into();
                buffer.did_reload(version, mtime, cx);
                Result::<_, anyhow::Error>::Ok(())
            })?;
        }
        Ok(())
    }

    pub fn match_paths<'a>(
        &self,
        query: &'a str,
        include_ignored: bool,
        smart_case: bool,
        max_results: usize,
        cancel_flag: &'a AtomicBool,
        cx: &AppContext,
    ) -> impl 'a + Future<Output = Vec<PathMatch>> {
        let worktrees = self
            .worktrees(cx)
            .filter(|worktree| !worktree.read(cx).is_weak())
            .collect::<Vec<_>>();
        let include_root_name = worktrees.len() > 1;
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| CandidateSet {
                snapshot: worktree.read(cx).snapshot(),
                include_ignored,
                include_root_name,
            })
            .collect::<Vec<_>>();

        let background = cx.background().clone();
        async move {
            fuzzy::match_paths(
                candidate_sets.as_slice(),
                query,
                smart_case,
                max_results,
                cancel_flag,
                background,
            )
            .await
        }
    }
}

impl WorktreeHandle {
    pub fn upgrade(&self, cx: &AppContext) -> Option<ModelHandle<Worktree>> {
        match self {
            WorktreeHandle::Strong(handle) => Some(handle.clone()),
            WorktreeHandle::Weak(handle) => handle.upgrade(cx),
        }
    }
}

struct CandidateSet {
    snapshot: Snapshot,
    include_ignored: bool,
    include_root_name: bool,
}

impl<'a> PathMatchCandidateSet<'a> for CandidateSet {
    type Candidates = CandidateSetIter<'a>;

    fn id(&self) -> usize {
        self.snapshot.id().to_usize()
    }

    fn len(&self) -> usize {
        if self.include_ignored {
            self.snapshot.file_count()
        } else {
            self.snapshot.visible_file_count()
        }
    }

    fn prefix(&self) -> Arc<str> {
        if self.snapshot.root_entry().map_or(false, |e| e.is_file()) {
            self.snapshot.root_name().into()
        } else if self.include_root_name {
            format!("{}/", self.snapshot.root_name()).into()
        } else {
            "".into()
        }
    }

    fn candidates(&'a self, start: usize) -> Self::Candidates {
        CandidateSetIter {
            traversal: self.snapshot.files(self.include_ignored, start),
        }
    }
}

struct CandidateSetIter<'a> {
    traversal: Traversal<'a>,
}

impl<'a> Iterator for CandidateSetIter<'a> {
    type Item = PathMatchCandidate<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.traversal.next().map(|entry| {
            if let EntryKind::File(char_bag) = entry.kind {
                PathMatchCandidate {
                    path: &entry.path,
                    char_bag,
                }
            } else {
                unreachable!()
            }
        })
    }
}

impl Entity for Project {
    type Event = Event;

    fn release(&mut self, _: &mut gpui::MutableAppContext) {
        match &self.client_state {
            ProjectClientState::Local { remote_id_rx, .. } => {
                if let Some(project_id) = *remote_id_rx.borrow() {
                    self.client
                        .send(proto::UnregisterProject { project_id })
                        .log_err();
                }
            }
            ProjectClientState::Remote { remote_id, .. } => {
                self.client
                    .send(proto::LeaveProject {
                        project_id: *remote_id,
                    })
                    .log_err();
            }
        }
    }

    fn app_will_quit(
        &mut self,
        _: &mut MutableAppContext,
    ) -> Option<std::pin::Pin<Box<dyn 'static + Future<Output = ()>>>> {
        use futures::FutureExt;

        let shutdown_futures = self
            .language_servers
            .drain()
            .filter_map(|(_, server)| server.shutdown())
            .collect::<Vec<_>>();
        Some(
            async move {
                futures::future::join_all(shutdown_futures).await;
            }
            .boxed(),
        )
    }
}

impl Collaborator {
    fn from_proto(
        message: proto::Collaborator,
        user_store: &ModelHandle<UserStore>,
        cx: &mut AsyncAppContext,
    ) -> impl Future<Output = Result<Self>> {
        let user = user_store.update(cx, |user_store, cx| {
            user_store.fetch_user(message.user_id, cx)
        });

        async move {
            Ok(Self {
                peer_id: PeerId(message.peer_id),
                user: user.await?,
                replica_id: message.replica_id as ReplicaId,
            })
        }
    }
}

impl<P: AsRef<Path>> From<(WorktreeId, P)> for ProjectPath {
    fn from((worktree_id, path): (WorktreeId, P)) -> Self {
        Self {
            worktree_id,
            path: path.as_ref().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, *};
    use client::test::FakeHttpClient;
    use fs::RealFs;
    use futures::StreamExt;
    use gpui::{test::subscribe, TestAppContext};
    use language::{
        tree_sitter_rust, AnchorRangeExt, Diagnostic, LanguageConfig, LanguageRegistry,
        LanguageServerConfig, Point,
    };
    use lsp::Url;
    use serde_json::json;
    use std::{cell::RefCell, os::unix, path::PathBuf, rc::Rc};
    use unindent::Unindent as _;
    use util::test::temp_tree;
    use worktree::WorktreeHandle as _;

    #[gpui::test]
    async fn test_populate_and_search(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "root": {
                "apple": "",
                "banana": {
                    "carrot": {
                        "date": "",
                        "endive": "",
                    }
                },
                "fennel": {
                    "grape": "",
                }
            }
        }));

        let root_link_path = dir.path().join("root_link");
        unix::fs::symlink(&dir.path().join("root"), &root_link_path).unwrap();
        unix::fs::symlink(
            &dir.path().join("root/fennel"),
            &dir.path().join("root/finnochio"),
        )
        .unwrap();

        let project = build_project(Arc::new(RealFs), &mut cx);

        let (tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree(&root_link_path, false, cx)
            })
            .await
            .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.read(|cx| {
            let tree = tree.read(cx);
            assert_eq!(tree.file_count(), 5);
            assert_eq!(
                tree.inode_for_path("fennel/grape"),
                tree.inode_for_path("finnochio/grape")
            );
        });

        let cancel_flag = Default::default();
        let results = project
            .read_with(&cx, |project, cx| {
                project.match_paths("bna", false, false, 10, &cancel_flag, cx)
            })
            .await;
        assert_eq!(
            results
                .into_iter()
                .map(|result| result.path)
                .collect::<Vec<Arc<Path>>>(),
            vec![
                PathBuf::from("banana/carrot/date").into(),
                PathBuf::from("banana/carrot/endive").into(),
            ]
        );
    }

    #[gpui::test]
    async fn test_language_server_diagnostics(mut cx: gpui::TestAppContext) {
        let (language_server_config, mut fake_server) =
            LanguageServerConfig::fake(cx.background()).await;
        let progress_token = language_server_config
            .disk_based_diagnostics_progress_token
            .clone()
            .unwrap();

        let mut languages = LanguageRegistry::new();
        languages.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".to_string(),
                path_suffixes: vec!["rs".to_string()],
                language_server: Some(language_server_config),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )));

        let dir = temp_tree(json!({
            "a.rs": "fn a() { A }",
            "b.rs": "const y: i32 = 1",
        }));

        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let project = cx.update(|cx| {
            Project::local(
                client,
                user_store,
                Arc::new(languages),
                Arc::new(RealFs),
                cx,
            )
        });

        let (tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree(dir.path(), false, cx)
            })
            .await
            .unwrap();
        let worktree_id = tree.read_with(&cx, |tree, _| tree.id());

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        // Cause worktree to start the fake language server
        let _buffer = project
            .update(&mut cx, |project, cx| {
                project.open_buffer(
                    ProjectPath {
                        worktree_id,
                        path: Path::new("b.rs").into(),
                    },
                    cx,
                )
            })
            .await
            .unwrap();

        let mut events = subscribe(&project, &mut cx);

        fake_server.start_progress(&progress_token).await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsStarted
        );

        fake_server.start_progress(&progress_token).await;
        fake_server.end_progress(&progress_token).await;
        fake_server.start_progress(&progress_token).await;

        fake_server
            .notify::<lsp::notification::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
                uri: Url::from_file_path(dir.path().join("a.rs")).unwrap(),
                version: None,
                diagnostics: vec![lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    message: "undefined variable 'A'".to_string(),
                    ..Default::default()
                }],
            })
            .await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiagnosticsUpdated(ProjectPath {
                worktree_id,
                path: Arc::from(Path::new("a.rs"))
            })
        );

        fake_server.end_progress(&progress_token).await;
        fake_server.end_progress(&progress_token).await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsUpdated
        );
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsFinished
        );

        let buffer = project
            .update(&mut cx, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx))
            .await
            .unwrap();

        buffer.read_with(&cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            let diagnostics = snapshot
                .diagnostics_in_range::<_, Point>(0..buffer.len())
                .collect::<Vec<_>>();
            assert_eq!(
                diagnostics,
                &[DiagnosticEntry {
                    range: Point::new(0, 9)..Point::new(0, 10),
                    diagnostic: Diagnostic {
                        severity: lsp::DiagnosticSeverity::ERROR,
                        message: "undefined variable 'A'".to_string(),
                        group_id: 0,
                        is_primary: true,
                        ..Default::default()
                    }
                }]
            )
        });
    }

    #[gpui::test]
    async fn test_search_worktree_without_files(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "root": {
                "dir1": {},
                "dir2": {
                    "dir3": {}
                }
            }
        }));

        let project = build_project(Arc::new(RealFs), &mut cx);
        let (tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree(&dir.path(), false, cx)
            })
            .await
            .unwrap();

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        let cancel_flag = Default::default();
        let results = project
            .read_with(&cx, |project, cx| {
                project.match_paths("dir", false, false, 10, &cancel_flag, cx)
            })
            .await;

        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_definition(mut cx: gpui::TestAppContext) {
        let (language_server_config, mut fake_server) =
            LanguageServerConfig::fake(cx.background()).await;

        let mut languages = LanguageRegistry::new();
        languages.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".to_string(),
                path_suffixes: vec!["rs".to_string()],
                language_server: Some(language_server_config),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )));

        let dir = temp_tree(json!({
            "a.rs": "const fn a() { A }",
            "b.rs": "const y: i32 = crate::a()",
        }));

        let http_client = FakeHttpClient::with_404_response();
        let client = Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let project = cx.update(|cx| {
            Project::local(
                client,
                user_store,
                Arc::new(languages),
                Arc::new(RealFs),
                cx,
            )
        });

        let (tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree(dir.path().join("b.rs"), false, cx)
            })
            .await
            .unwrap();
        let worktree_id = tree.read_with(&cx, |tree, _| tree.id());
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        // Cause worktree to start the fake language server
        let buffer = project
            .update(&mut cx, |project, cx| {
                project.open_buffer(
                    ProjectPath {
                        worktree_id,
                        path: Path::new("").into(),
                    },
                    cx,
                )
            })
            .await
            .unwrap();
        let definitions =
            project.update(&mut cx, |project, cx| project.definition(&buffer, 22, cx));
        let (request_id, request) = fake_server
            .receive_request::<lsp::request::GotoDefinition>()
            .await;
        let request_params = request.text_document_position_params;
        assert_eq!(
            request_params.text_document.uri.to_file_path().unwrap(),
            dir.path().join("b.rs")
        );
        assert_eq!(request_params.position, lsp::Position::new(0, 22));

        fake_server
            .respond(
                request_id,
                Some(lsp::GotoDefinitionResponse::Scalar(lsp::Location::new(
                    lsp::Url::from_file_path(dir.path().join("a.rs")).unwrap(),
                    lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                ))),
            )
            .await;
        let mut definitions = definitions.await.unwrap();
        assert_eq!(definitions.len(), 1);
        let definition = definitions.pop().unwrap();
        cx.update(|cx| {
            let target_buffer = definition.target_buffer.read(cx);
            assert_eq!(
                target_buffer
                    .file()
                    .unwrap()
                    .as_local()
                    .unwrap()
                    .abs_path(cx),
                dir.path().join("a.rs")
            );
            assert_eq!(definition.target_range.to_offset(target_buffer), 9..10);
            assert_eq!(
                list_worktrees(&project, cx),
                [
                    (dir.path().join("b.rs"), false),
                    (dir.path().join("a.rs"), true)
                ]
            );

            drop(definition);
        });
        cx.read(|cx| {
            assert_eq!(
                list_worktrees(&project, cx),
                [(dir.path().join("b.rs"), false)]
            );
        });

        fn list_worktrees(project: &ModelHandle<Project>, cx: &AppContext) -> Vec<(PathBuf, bool)> {
            project
                .read(cx)
                .worktrees(cx)
                .map(|worktree| {
                    let worktree = worktree.read(cx);
                    (
                        worktree.as_local().unwrap().abs_path().to_path_buf(),
                        worktree.is_weak(),
                    )
                })
                .collect::<Vec<_>>()
        }
    }

    #[gpui::test]
    async fn test_save_file(mut cx: gpui::TestAppContext) {
        let fs = Arc::new(FakeFs::new(cx.background()));
        fs.insert_tree(
            "/dir",
            json!({
                "file1": "the old contents",
            }),
        )
        .await;

        let project = build_project(fs.clone(), &mut cx);
        let worktree_id = project
            .update(&mut cx, |p, cx| {
                p.find_or_create_local_worktree("/dir", false, cx)
            })
            .await
            .unwrap()
            .0
            .read_with(&cx, |tree, _| tree.id());

        let buffer = project
            .update(&mut cx, |p, cx| p.open_buffer((worktree_id, "file1"), cx))
            .await
            .unwrap();
        buffer
            .update(&mut cx, |buffer, cx| {
                assert_eq!(buffer.text(), "the old contents");
                buffer.edit(Some(0..0), "a line of text.\n".repeat(10 * 1024), cx);
                buffer.save(cx)
            })
            .await
            .unwrap();

        let new_text = fs.load(Path::new("/dir/file1")).await.unwrap();
        assert_eq!(new_text, buffer.read_with(&cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_save_in_single_file_worktree(mut cx: gpui::TestAppContext) {
        let fs = Arc::new(FakeFs::new(cx.background()));
        fs.insert_tree(
            "/dir",
            json!({
                "file1": "the old contents",
            }),
        )
        .await;

        let project = build_project(fs.clone(), &mut cx);
        let worktree_id = project
            .update(&mut cx, |p, cx| {
                p.find_or_create_local_worktree("/dir/file1", false, cx)
            })
            .await
            .unwrap()
            .0
            .read_with(&cx, |tree, _| tree.id());

        let buffer = project
            .update(&mut cx, |p, cx| p.open_buffer((worktree_id, ""), cx))
            .await
            .unwrap();
        buffer
            .update(&mut cx, |buffer, cx| {
                buffer.edit(Some(0..0), "a line of text.\n".repeat(10 * 1024), cx);
                buffer.save(cx)
            })
            .await
            .unwrap();

        let new_text = fs.load(Path::new("/dir/file1")).await.unwrap();
        assert_eq!(new_text, buffer.read_with(&cx, |buffer, _| buffer.text()));
    }

    #[gpui::test(retries = 5)]
    async fn test_rescan_and_remote_updates(mut cx: gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "a": {
                "file1": "",
                "file2": "",
                "file3": "",
            },
            "b": {
                "c": {
                    "file4": "",
                    "file5": "",
                }
            }
        }));

        let project = build_project(Arc::new(RealFs), &mut cx);
        let rpc = project.read_with(&cx, |p, _| p.client.clone());

        let (tree, _) = project
            .update(&mut cx, |p, cx| {
                p.find_or_create_local_worktree(dir.path(), false, cx)
            })
            .await
            .unwrap();
        let worktree_id = tree.read_with(&cx, |tree, _| tree.id());

        let buffer_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
            let buffer = project.update(cx, |p, cx| p.open_buffer((worktree_id, path), cx));
            async move { buffer.await.unwrap() }
        };
        let id_for_path = |path: &'static str, cx: &gpui::TestAppContext| {
            tree.read_with(cx, |tree, _| {
                tree.entry_for_path(path)
                    .expect(&format!("no entry for path {}", path))
                    .id
            })
        };

        let buffer2 = buffer_for_path("a/file2", &mut cx).await;
        let buffer3 = buffer_for_path("a/file3", &mut cx).await;
        let buffer4 = buffer_for_path("b/c/file4", &mut cx).await;
        let buffer5 = buffer_for_path("b/c/file5", &mut cx).await;

        let file2_id = id_for_path("a/file2", &cx);
        let file3_id = id_for_path("a/file3", &cx);
        let file4_id = id_for_path("b/c/file4", &cx);

        // Wait for the initial scan.
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        // Create a remote copy of this worktree.
        let initial_snapshot = tree.read_with(&cx, |tree, _| tree.snapshot());
        let (remote, load_task) = cx.update(|cx| {
            Worktree::remote(
                1,
                1,
                initial_snapshot.to_proto(&Default::default(), Default::default()),
                rpc.clone(),
                cx,
            )
        });
        load_task.await;

        cx.read(|cx| {
            assert!(!buffer2.read(cx).is_dirty());
            assert!(!buffer3.read(cx).is_dirty());
            assert!(!buffer4.read(cx).is_dirty());
            assert!(!buffer5.read(cx).is_dirty());
        });

        // Rename and delete files and directories.
        tree.flush_fs_events(&cx).await;
        std::fs::rename(dir.path().join("a/file3"), dir.path().join("b/c/file3")).unwrap();
        std::fs::remove_file(dir.path().join("b/c/file5")).unwrap();
        std::fs::rename(dir.path().join("b/c"), dir.path().join("d")).unwrap();
        std::fs::rename(dir.path().join("a/file2"), dir.path().join("a/file2.new")).unwrap();
        tree.flush_fs_events(&cx).await;

        let expected_paths = vec![
            "a",
            "a/file1",
            "a/file2.new",
            "b",
            "d",
            "d/file3",
            "d/file4",
        ];

        cx.read(|app| {
            assert_eq!(
                tree.read(app)
                    .paths()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>(),
                expected_paths
            );

            assert_eq!(id_for_path("a/file2.new", &cx), file2_id);
            assert_eq!(id_for_path("d/file3", &cx), file3_id);
            assert_eq!(id_for_path("d/file4", &cx), file4_id);

            assert_eq!(
                buffer2.read(app).file().unwrap().path().as_ref(),
                Path::new("a/file2.new")
            );
            assert_eq!(
                buffer3.read(app).file().unwrap().path().as_ref(),
                Path::new("d/file3")
            );
            assert_eq!(
                buffer4.read(app).file().unwrap().path().as_ref(),
                Path::new("d/file4")
            );
            assert_eq!(
                buffer5.read(app).file().unwrap().path().as_ref(),
                Path::new("b/c/file5")
            );

            assert!(!buffer2.read(app).file().unwrap().is_deleted());
            assert!(!buffer3.read(app).file().unwrap().is_deleted());
            assert!(!buffer4.read(app).file().unwrap().is_deleted());
            assert!(buffer5.read(app).file().unwrap().is_deleted());
        });

        // Update the remote worktree. Check that it becomes consistent with the
        // local worktree.
        remote.update(&mut cx, |remote, cx| {
            let update_message =
                tree.read(cx)
                    .snapshot()
                    .build_update(&initial_snapshot, 1, 1, true);
            remote
                .as_remote_mut()
                .unwrap()
                .snapshot
                .apply_remote_update(update_message)
                .unwrap();

            assert_eq!(
                remote
                    .paths()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>(),
                expected_paths
            );
        });
    }

    #[gpui::test]
    async fn test_buffer_deduping(mut cx: gpui::TestAppContext) {
        let fs = Arc::new(FakeFs::new(cx.background()));
        fs.insert_tree(
            "/the-dir",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;

        let project = build_project(fs.clone(), &mut cx);
        let worktree_id = project
            .update(&mut cx, |p, cx| {
                p.find_or_create_local_worktree("/the-dir", false, cx)
            })
            .await
            .unwrap()
            .0
            .read_with(&cx, |tree, _| tree.id());

        // Spawn multiple tasks to open paths, repeating some paths.
        let (buffer_a_1, buffer_b, buffer_a_2) = project.update(&mut cx, |p, cx| {
            (
                p.open_buffer((worktree_id, "a.txt"), cx),
                p.open_buffer((worktree_id, "b.txt"), cx),
                p.open_buffer((worktree_id, "a.txt"), cx),
            )
        });

        let buffer_a_1 = buffer_a_1.await.unwrap();
        let buffer_a_2 = buffer_a_2.await.unwrap();
        let buffer_b = buffer_b.await.unwrap();
        assert_eq!(buffer_a_1.read_with(&cx, |b, _| b.text()), "a-contents");
        assert_eq!(buffer_b.read_with(&cx, |b, _| b.text()), "b-contents");

        // There is only one buffer per path.
        let buffer_a_id = buffer_a_1.id();
        assert_eq!(buffer_a_2.id(), buffer_a_id);

        // Open the same path again while it is still open.
        drop(buffer_a_1);
        let buffer_a_3 = project
            .update(&mut cx, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
            .await
            .unwrap();

        // There's still only one buffer per path.
        assert_eq!(buffer_a_3.id(), buffer_a_id);
    }

    #[gpui::test]
    async fn test_buffer_is_dirty(mut cx: gpui::TestAppContext) {
        use std::fs;

        let dir = temp_tree(json!({
            "file1": "abc",
            "file2": "def",
            "file3": "ghi",
        }));

        let project = build_project(Arc::new(RealFs), &mut cx);
        let (worktree, _) = project
            .update(&mut cx, |p, cx| {
                p.find_or_create_local_worktree(dir.path(), false, cx)
            })
            .await
            .unwrap();
        let worktree_id = worktree.read_with(&cx, |worktree, _| worktree.id());

        worktree.flush_fs_events(&cx).await;
        worktree
            .read_with(&cx, |t, _| t.as_local().unwrap().scan_complete())
            .await;

        let buffer1 = project
            .update(&mut cx, |p, cx| p.open_buffer((worktree_id, "file1"), cx))
            .await
            .unwrap();
        let events = Rc::new(RefCell::new(Vec::new()));

        // initially, the buffer isn't dirty.
        buffer1.update(&mut cx, |buffer, cx| {
            cx.subscribe(&buffer1, {
                let events = events.clone();
                move |_, _, event, _| events.borrow_mut().push(event.clone())
            })
            .detach();

            assert!(!buffer.is_dirty());
            assert!(events.borrow().is_empty());

            buffer.edit(vec![1..2], "", cx);
        });

        // after the first edit, the buffer is dirty, and emits a dirtied event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(buffer.text() == "ac");
            assert!(buffer.is_dirty());
            assert_eq!(
                *events.borrow(),
                &[language::Event::Edited, language::Event::Dirtied]
            );
            events.borrow_mut().clear();
            buffer.did_save(buffer.version(), buffer.file().unwrap().mtime(), None, cx);
        });

        // after saving, the buffer is not dirty, and emits a saved event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(!buffer.is_dirty());
            assert_eq!(*events.borrow(), &[language::Event::Saved]);
            events.borrow_mut().clear();

            buffer.edit(vec![1..1], "B", cx);
            buffer.edit(vec![2..2], "D", cx);
        });

        // after editing again, the buffer is dirty, and emits another dirty event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(buffer.text() == "aBDc");
            assert!(buffer.is_dirty());
            assert_eq!(
                *events.borrow(),
                &[
                    language::Event::Edited,
                    language::Event::Dirtied,
                    language::Event::Edited,
                ],
            );
            events.borrow_mut().clear();

            // TODO - currently, after restoring the buffer to its
            // previously-saved state, the is still considered dirty.
            buffer.edit([1..3], "", cx);
            assert!(buffer.text() == "ac");
            assert!(buffer.is_dirty());
        });

        assert_eq!(*events.borrow(), &[language::Event::Edited]);

        // When a file is deleted, the buffer is considered dirty.
        let events = Rc::new(RefCell::new(Vec::new()));
        let buffer2 = project
            .update(&mut cx, |p, cx| p.open_buffer((worktree_id, "file2"), cx))
            .await
            .unwrap();
        buffer2.update(&mut cx, |_, cx| {
            cx.subscribe(&buffer2, {
                let events = events.clone();
                move |_, _, event, _| events.borrow_mut().push(event.clone())
            })
            .detach();
        });

        fs::remove_file(dir.path().join("file2")).unwrap();
        buffer2.condition(&cx, |b, _| b.is_dirty()).await;
        assert_eq!(
            *events.borrow(),
            &[language::Event::Dirtied, language::Event::FileHandleChanged]
        );

        // When a file is already dirty when deleted, we don't emit a Dirtied event.
        let events = Rc::new(RefCell::new(Vec::new()));
        let buffer3 = project
            .update(&mut cx, |p, cx| p.open_buffer((worktree_id, "file3"), cx))
            .await
            .unwrap();
        buffer3.update(&mut cx, |_, cx| {
            cx.subscribe(&buffer3, {
                let events = events.clone();
                move |_, _, event, _| events.borrow_mut().push(event.clone())
            })
            .detach();
        });

        worktree.flush_fs_events(&cx).await;
        buffer3.update(&mut cx, |buffer, cx| {
            buffer.edit(Some(0..0), "x", cx);
        });
        events.borrow_mut().clear();
        fs::remove_file(dir.path().join("file3")).unwrap();
        buffer3
            .condition(&cx, |_, _| !events.borrow().is_empty())
            .await;
        assert_eq!(*events.borrow(), &[language::Event::FileHandleChanged]);
        cx.read(|cx| assert!(buffer3.read(cx).is_dirty()));
    }

    #[gpui::test]
    async fn test_buffer_file_changes_on_disk(mut cx: gpui::TestAppContext) {
        use std::fs;

        let initial_contents = "aaa\nbbbbb\nc\n";
        let dir = temp_tree(json!({ "the-file": initial_contents }));

        let project = build_project(Arc::new(RealFs), &mut cx);
        let (worktree, _) = project
            .update(&mut cx, |p, cx| {
                p.find_or_create_local_worktree(dir.path(), false, cx)
            })
            .await
            .unwrap();
        let worktree_id = worktree.read_with(&cx, |tree, _| tree.id());

        worktree
            .read_with(&cx, |t, _| t.as_local().unwrap().scan_complete())
            .await;

        let abs_path = dir.path().join("the-file");
        let buffer = project
            .update(&mut cx, |p, cx| {
                p.open_buffer((worktree_id, "the-file"), cx)
            })
            .await
            .unwrap();

        // TODO
        // Add a cursor on each row.
        // let selection_set_id = buffer.update(&mut cx, |buffer, cx| {
        //     assert!(!buffer.is_dirty());
        //     buffer.add_selection_set(
        //         &(0..3)
        //             .map(|row| Selection {
        //                 id: row as usize,
        //                 start: Point::new(row, 1),
        //                 end: Point::new(row, 1),
        //                 reversed: false,
        //                 goal: SelectionGoal::None,
        //             })
        //             .collect::<Vec<_>>(),
        //         cx,
        //     )
        // });

        // Change the file on disk, adding two new lines of text, and removing
        // one line.
        buffer.read_with(&cx, |buffer, _| {
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        let new_contents = "AAAA\naaa\nBB\nbbbbb\n";
        fs::write(&abs_path, new_contents).unwrap();

        // Because the buffer was not modified, it is reloaded from disk. Its
        // contents are edited according to the diff between the old and new
        // file contents.
        buffer
            .condition(&cx, |buffer, _| buffer.text() == new_contents)
            .await;

        buffer.update(&mut cx, |buffer, _| {
            assert_eq!(buffer.text(), new_contents);
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());

            // TODO
            // let cursor_positions = buffer
            //     .selection_set(selection_set_id)
            //     .unwrap()
            //     .selections::<Point>(&*buffer)
            //     .map(|selection| {
            //         assert_eq!(selection.start, selection.end);
            //         selection.start
            //     })
            //     .collect::<Vec<_>>();
            // assert_eq!(
            //     cursor_positions,
            //     [Point::new(1, 1), Point::new(3, 1), Point::new(4, 0)]
            // );
        });

        // Modify the buffer
        buffer.update(&mut cx, |buffer, cx| {
            buffer.edit(vec![0..0], " ", cx);
            assert!(buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });

        // Change the file on disk again, adding blank lines to the beginning.
        fs::write(&abs_path, "\n\n\nAAAA\naaa\nBB\nbbbbb\n").unwrap();

        // Because the buffer is modified, it doesn't reload from disk, but is
        // marked as having a conflict.
        buffer
            .condition(&cx, |buffer, _| buffer.has_conflict())
            .await;
    }

    #[gpui::test]
    async fn test_grouped_diagnostics(mut cx: gpui::TestAppContext) {
        let fs = Arc::new(FakeFs::new(cx.background()));
        fs.insert_tree(
            "/the-dir",
            json!({
                "a.rs": "
                    fn foo(mut v: Vec<usize>) {
                        for x in &v {
                            v.push(1);
                        }
                    }
                "
                .unindent(),
            }),
        )
        .await;

        let project = build_project(fs.clone(), &mut cx);
        let (worktree, _) = project
            .update(&mut cx, |p, cx| {
                p.find_or_create_local_worktree("/the-dir", false, cx)
            })
            .await
            .unwrap();
        let worktree_id = worktree.read_with(&cx, |tree, _| tree.id());

        let buffer = project
            .update(&mut cx, |p, cx| p.open_buffer((worktree_id, "a.rs"), cx))
            .await
            .unwrap();

        let buffer_uri = Url::from_file_path("/the-dir/a.rs").unwrap();
        let message = lsp::PublishDiagnosticsParams {
            uri: buffer_uri.clone(),
            diagnostics: vec![
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                    severity: Some(DiagnosticSeverity::WARNING),
                    message: "error 1".to_string(),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 8),
                                lsp::Position::new(1, 9),
                            ),
                        },
                        message: "error 1 hint 1".to_string(),
                    }]),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(1, 8), lsp::Position::new(1, 9)),
                    severity: Some(DiagnosticSeverity::HINT),
                    message: "error 1 hint 1".to_string(),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 8),
                                lsp::Position::new(1, 9),
                            ),
                        },
                        message: "original diagnostic".to_string(),
                    }]),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 17)),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: "error 2".to_string(),
                    related_information: Some(vec![
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location {
                                uri: buffer_uri.clone(),
                                range: lsp::Range::new(
                                    lsp::Position::new(1, 13),
                                    lsp::Position::new(1, 15),
                                ),
                            },
                            message: "error 2 hint 1".to_string(),
                        },
                        lsp::DiagnosticRelatedInformation {
                            location: lsp::Location {
                                uri: buffer_uri.clone(),
                                range: lsp::Range::new(
                                    lsp::Position::new(1, 13),
                                    lsp::Position::new(1, 15),
                                ),
                            },
                            message: "error 2 hint 2".to_string(),
                        },
                    ]),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 15)),
                    severity: Some(DiagnosticSeverity::HINT),
                    message: "error 2 hint 1".to_string(),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(2, 8),
                                lsp::Position::new(2, 17),
                            ),
                        },
                        message: "original diagnostic".to_string(),
                    }]),
                    ..Default::default()
                },
                lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 15)),
                    severity: Some(DiagnosticSeverity::HINT),
                    message: "error 2 hint 2".to_string(),
                    related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                        location: lsp::Location {
                            uri: buffer_uri.clone(),
                            range: lsp::Range::new(
                                lsp::Position::new(2, 8),
                                lsp::Position::new(2, 17),
                            ),
                        },
                        message: "original diagnostic".to_string(),
                    }]),
                    ..Default::default()
                },
            ],
            version: None,
        };

        project
            .update(&mut cx, |p, cx| {
                p.update_diagnostics(message, &Default::default(), cx)
            })
            .unwrap();
        let buffer = buffer.read_with(&cx, |buffer, _| buffer.snapshot());

        assert_eq!(
            buffer
                .diagnostics_in_range::<_, Point>(0..buffer.len())
                .collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(1, 8)..Point::new(1, 9),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::WARNING,
                        message: "error 1".to_string(),
                        group_id: 0,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(1, 8)..Point::new(1, 9),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 1 hint 1".to_string(),
                        group_id: 0,
                        is_primary: false,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(1, 13)..Point::new(1, 15),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 2 hint 1".to_string(),
                        group_id: 1,
                        is_primary: false,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(1, 13)..Point::new(1, 15),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 2 hint 2".to_string(),
                        group_id: 1,
                        is_primary: false,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(2, 8)..Point::new(2, 17),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "error 2".to_string(),
                        group_id: 1,
                        is_primary: true,
                        ..Default::default()
                    }
                }
            ]
        );

        assert_eq!(
            buffer.diagnostic_group::<Point>(0).collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(1, 8)..Point::new(1, 9),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::WARNING,
                        message: "error 1".to_string(),
                        group_id: 0,
                        is_primary: true,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(1, 8)..Point::new(1, 9),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 1 hint 1".to_string(),
                        group_id: 0,
                        is_primary: false,
                        ..Default::default()
                    }
                },
            ]
        );
        assert_eq!(
            buffer.diagnostic_group::<Point>(1).collect::<Vec<_>>(),
            &[
                DiagnosticEntry {
                    range: Point::new(1, 13)..Point::new(1, 15),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 2 hint 1".to_string(),
                        group_id: 1,
                        is_primary: false,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(1, 13)..Point::new(1, 15),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::HINT,
                        message: "error 2 hint 2".to_string(),
                        group_id: 1,
                        is_primary: false,
                        ..Default::default()
                    }
                },
                DiagnosticEntry {
                    range: Point::new(2, 8)..Point::new(2, 17),
                    diagnostic: Diagnostic {
                        severity: DiagnosticSeverity::ERROR,
                        message: "error 2".to_string(),
                        group_id: 1,
                        is_primary: true,
                        ..Default::default()
                    }
                }
            ]
        );
    }

    fn build_project(fs: Arc<dyn Fs>, cx: &mut TestAppContext) -> ModelHandle<Project> {
        let languages = Arc::new(LanguageRegistry::new());
        let http_client = FakeHttpClient::with_404_response();
        let client = client::Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        cx.update(|cx| Project::local(client, user_store, languages, fs, cx))
    }
}
