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
};
use language::{Buffer, DiagnosticEntry, Language, LanguageRegistry};
use lsp::{DiagnosticSeverity, LanguageServer};
use postage::{prelude::Stream, watch};
use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};
use util::{ResultExt, TryFutureExt as _};

pub use fs::*;
pub use worktree::*;

pub struct Project {
    worktrees: Vec<ModelHandle<Worktree>>,
    active_entry: Option<ProjectEntry>,
    languages: Arc<LanguageRegistry>,
    language_servers: HashMap<(WorktreeId, String), Arc<LanguageServer>>,
    client: Arc<client::Client>,
    user_store: ModelHandle<UserStore>,
    fs: Arc<dyn Fs>,
    client_state: ProjectClientState,
    collaborators: HashMap<PeerId, Collaborator>,
    subscriptions: Vec<client::Subscription>,
    pending_disk_based_diagnostics: isize,
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

#[derive(Debug)]
pub enum Event {
    ActiveEntryChanged(Option<ProjectEntry>),
    WorktreeRemoved(WorktreeId),
    DiskBasedDiagnosticsStarted,
    DiskBasedDiagnosticsUpdated { worktree_id: WorktreeId },
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
                                    this.read_with(&cx, |this, cx| {
                                        for worktree in &this.worktrees {
                                            let worktree_id = worktree.id() as u64;
                                            let worktree = worktree.read(cx).as_local().unwrap();
                                            registrations.push(rpc.request(
                                                proto::RegisterWorktree {
                                                    project_id,
                                                    worktree_id,
                                                    root_name: worktree.root_name().to_string(),
                                                    authorized_logins: worktree.authorized_logins(),
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
                pending_disk_based_diagnostics: 0,
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
            worktrees.push(
                Worktree::remote(
                    remote_id,
                    replica_id,
                    worktree,
                    client.clone(),
                    user_store.clone(),
                    cx,
                )
                .await?,
            );
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
                    client.subscribe_to_entity(remote_id, cx, Self::handle_buffer_saved),
                ],
                client,
                client_state: ProjectClientState::Remote {
                    sharing_has_stopped: false,
                    remote_id,
                    replica_id,
                },
                pending_disk_based_diagnostics: 0,
                language_servers: Default::default(),
            };
            for worktree in worktrees {
                this.add_worktree(worktree, cx);
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

    pub fn worktrees(&self) -> &[ModelHandle<Worktree>] {
        &self.worktrees
    }

    pub fn worktree_for_id(
        &self,
        id: WorktreeId,
        cx: &AppContext,
    ) -> Option<ModelHandle<Worktree>> {
        self.worktrees
            .iter()
            .find(|worktree| worktree.read(cx).id() == id)
            .cloned()
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
                for worktree in &this.worktrees {
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

            rpc.send(proto::UnshareProject { project_id }).await?;
            this.update(&mut cx, |this, cx| {
                this.collaborators.clear();
                for worktree in &this.worktrees {
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
        path: ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let worktree = if let Some(worktree) = self.worktree_for_id(path.worktree_id, cx) {
            worktree
        } else {
            return cx.spawn(|_, _| async move { Err(anyhow!("no such worktree")) });
        };
        let buffer_task = worktree.update(cx, |worktree, cx| worktree.open_buffer(path.path, cx));
        cx.spawn(|this, mut cx| async move {
            let (buffer, buffer_is_new) = buffer_task.await?;
            if buffer_is_new {
                this.update(&mut cx, |this, cx| {
                    this.buffer_added(worktree, buffer.clone(), cx)
                });
            }
            Ok(buffer)
        })
    }

    pub fn save_buffer_as(
        &self,
        buffer: ModelHandle<Buffer>,
        abs_path: PathBuf,
        cx: &mut ModelContext<Project>,
    ) -> Task<Result<()>> {
        let worktree_task = self.worktree_for_abs_path(&abs_path, cx);
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
            this.update(&mut cx, |this, cx| this.buffer_added(worktree, buffer, cx));
            Ok(())
        })
    }

    fn buffer_added(
        &mut self,
        worktree: ModelHandle<Worktree>,
        buffer: ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        // Set the buffer's language
        let full_path = buffer.read(cx).file()?.full_path();
        let language = self.languages.select_language(&full_path)?.clone();
        buffer.update(cx, |buffer, cx| {
            buffer.set_language(Some(language.clone()), cx);
        });

        // For local worktrees, start a language server if needed.
        let worktree = worktree.read(cx);
        let worktree_id = worktree.id();
        let worktree_abs_path = worktree.as_local()?.abs_path().clone();
        let language_server = match self
            .language_servers
            .entry((worktree_id, language.name().to_string()))
        {
            hash_map::Entry::Occupied(e) => Some(e.get().clone()),
            hash_map::Entry::Vacant(e) => Self::start_language_server(
                self.client.clone(),
                language,
                worktree_id,
                &worktree_abs_path,
                cx,
            )
            .map(|server| e.insert(server).clone()),
        };

        buffer.update(cx, |buffer, cx| {
            buffer.set_language_server(language_server, cx)
        });

        None
    }

    fn start_language_server(
        rpc: Arc<Client>,
        language: Arc<Language>,
        worktree_id: WorktreeId,
        worktree_path: &Path,
        cx: &mut ModelContext<Self>,
    ) -> Option<Arc<LanguageServer>> {
        let language_server = language
            .start_server(worktree_path, cx)
            .log_err()
            .flatten()?;

        enum DiagnosticProgress {
            Updating,
            Publish(lsp::PublishDiagnosticsParams),
            Updated,
        }

        let disk_based_sources = language
            .disk_based_diagnostic_sources()
            .cloned()
            .unwrap_or_default();
        let disk_based_diagnostics_progress_token =
            language.disk_based_diagnostics_progress_token().cloned();
        let has_disk_based_diagnostic_progress_token =
            disk_based_diagnostics_progress_token.is_some();
        let (diagnostics_tx, diagnostics_rx) = smol::channel::unbounded();

        language_server
            .on_notification::<lsp::notification::PublishDiagnostics, _>({
                let diagnostics_tx = diagnostics_tx.clone();
                move |params| {
                    if !has_disk_based_diagnostic_progress_token {
                        smol::block_on(diagnostics_tx.send(DiagnosticProgress::Updating)).ok();
                    }
                    smol::block_on(diagnostics_tx.send(DiagnosticProgress::Publish(params))).ok();
                    if !has_disk_based_diagnostic_progress_token {
                        smol::block_on(diagnostics_tx.send(DiagnosticProgress::Updated)).ok();
                    }
                }
            })
            .detach();

        let mut pending_disk_based_diagnostics: i32 = 0;
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
                                if pending_disk_based_diagnostics == 0 {
                                    smol::block_on(
                                        diagnostics_tx.send(DiagnosticProgress::Updating),
                                    )
                                    .ok();
                                }
                                pending_disk_based_diagnostics += 1;
                            }
                            lsp::WorkDoneProgress::End(_) => {
                                pending_disk_based_diagnostics -= 1;
                                if pending_disk_based_diagnostics == 0 {
                                    smol::block_on(
                                        diagnostics_tx.send(DiagnosticProgress::Updated),
                                    )
                                    .ok();
                                }
                            }
                            _ => {}
                        },
                    }
                }
            })
            .detach();

        cx.spawn_weak(|this, mut cx| async move {
            while let Ok(message) = diagnostics_rx.recv().await {
                let this = cx.read(|cx| this.upgrade(cx))?;
                match message {
                    DiagnosticProgress::Updating => {
                        let project_id = this.update(&mut cx, |this, cx| {
                            cx.emit(Event::DiskBasedDiagnosticsStarted);
                            this.remote_id()
                        });
                        if let Some(project_id) = project_id {
                            rpc.send(proto::DiskBasedDiagnosticsUpdating {
                                project_id,
                                worktree_id: worktree_id.to_proto(),
                            })
                            .await
                            .log_err();
                        }
                    }
                    DiagnosticProgress::Publish(params) => {
                        this.update(&mut cx, |this, cx| {
                            this.update_diagnostics(params, &disk_based_sources, cx)
                                .log_err();
                        });
                    }
                    DiagnosticProgress::Updated => {
                        let project_id = this.update(&mut cx, |this, cx| {
                            cx.emit(Event::DiskBasedDiagnosticsFinished);
                            this.remote_id()
                        });
                        if let Some(project_id) = project_id {
                            rpc.send(proto::DiskBasedDiagnosticsUpdated {
                                project_id,
                                worktree_id: worktree_id.to_proto(),
                            })
                            .await
                            .log_err();
                        }
                    }
                }
            }
            Some(())
        })
        .detach();

        Some(language_server)
    }

    fn update_diagnostics(
        &mut self,
        diagnostics: lsp::PublishDiagnosticsParams,
        disk_based_sources: &HashSet<String>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let path = diagnostics
            .uri
            .to_file_path()
            .map_err(|_| anyhow!("URI is not a file"))?;
        for tree in &self.worktrees {
            let relative_path = tree.update(cx, |tree, _| {
                path.strip_prefix(tree.as_local()?.abs_path()).ok()
            });
            if let Some(relative_path) = relative_path {
                return tree.update(cx, |tree, cx| {
                    tree.as_local_mut().unwrap().update_diagnostics(
                        relative_path.into(),
                        diagnostics,
                        disk_based_sources,
                        cx,
                    )
                });
            }
        }
        todo!()
    }

    pub fn worktree_for_abs_path(
        &self,
        abs_path: &Path,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(ModelHandle<Worktree>, PathBuf)>> {
        for tree in &self.worktrees {
            if let Some(relative_path) = tree
                .read(cx)
                .as_local()
                .and_then(|t| abs_path.strip_prefix(t.abs_path()).ok())
            {
                return Task::ready(Ok((tree.clone(), relative_path.into())));
            }
        }

        let worktree = self.add_local_worktree(abs_path, cx);
        cx.background().spawn(async move {
            let worktree = worktree.await?;
            Ok((worktree, PathBuf::new()))
        })
    }

    pub fn is_shared(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local { is_shared, .. } => *is_shared,
            ProjectClientState::Remote { .. } => false,
        }
    }

    pub fn add_local_worktree(
        &self,
        abs_path: impl AsRef<Path>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        let fs = self.fs.clone();
        let client = self.client.clone();
        let user_store = self.user_store.clone();
        let path = Arc::from(abs_path.as_ref());
        cx.spawn(|project, mut cx| async move {
            let worktree =
                Worktree::open_local(client.clone(), user_store, path, fs, &mut cx).await?;

            let (remote_project_id, is_shared) = project.update(&mut cx, |project, cx| {
                project.add_worktree(worktree.clone(), cx);
                (project.remote_id(), project.is_shared())
            });

            if let Some(project_id) = remote_project_id {
                let worktree_id = worktree.id() as u64;
                let register_message = worktree.update(&mut cx, |worktree, _| {
                    let worktree = worktree.as_local_mut().unwrap();
                    proto::RegisterWorktree {
                        project_id,
                        worktree_id,
                        root_name: worktree.root_name().to_string(),
                        authorized_logins: worktree.authorized_logins(),
                    }
                });
                client.request(register_message).await?;
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

    fn add_worktree(&mut self, worktree: ModelHandle<Worktree>, cx: &mut ModelContext<Self>) {
        cx.observe(&worktree, |_, _, cx| cx.notify()).detach();
        cx.subscribe(&worktree, move |this, worktree, event, cx| match event {
            worktree::Event::DiagnosticsUpdated(path) => {
                cx.emit(Event::DiagnosticsUpdated(ProjectPath {
                    worktree_id: worktree.read(cx).id(),
                    path: path.clone(),
                }));
            }
            worktree::Event::DiskBasedDiagnosticsUpdating => {
                if this.pending_disk_based_diagnostics == 0 {
                    cx.emit(Event::DiskBasedDiagnosticsStarted);
                }
                this.pending_disk_based_diagnostics += 1;
            }
            worktree::Event::DiskBasedDiagnosticsUpdated => {
                this.pending_disk_based_diagnostics -= 1;
                cx.emit(Event::DiskBasedDiagnosticsUpdated {
                    worktree_id: worktree.read(cx).id(),
                });
                if this.pending_disk_based_diagnostics == 0 {
                    if this.pending_disk_based_diagnostics == 0 {
                        cx.emit(Event::DiskBasedDiagnosticsFinished);
                    }
                }
            }
        })
        .detach();
        self.worktrees.push(worktree);
        cx.notify();
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

    pub fn path_for_entry(&self, entry: ProjectEntry, cx: &AppContext) -> Option<ProjectPath> {
        let worktree = self.worktree_for_id(entry.worktree_id, cx)?.read(cx);
        Some(ProjectPath {
            worktree_id: entry.worktree_id,
            path: worktree.entry_for_id(entry.entry_id)?.path.clone(),
        })
    }

    pub fn is_running_disk_based_diagnostics(&self) -> bool {
        self.pending_disk_based_diagnostics > 0
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
        self.worktrees.iter().flat_map(move |worktree| {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            worktree
                .diagnostic_summaries()
                .map(move |(path, summary)| (ProjectPath { worktree_id, path }, summary))
        })
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
        for worktree in &self.worktrees {
            worktree.update(cx, |worktree, cx| {
                worktree.remove_collaborator(peer_id, replica_id, cx);
            })
        }
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
        let user_store = self.user_store.clone();
        cx.spawn(|this, mut cx| {
            async move {
                let worktree =
                    Worktree::remote(remote_id, replica_id, worktree, client, user_store, &mut cx)
                        .await?;
                this.update(&mut cx, |this, cx| this.add_worktree(worktree, cx));
                Ok(())
            }
            .log_err()
        })
        .detach();
        Ok(())
    }

    fn handle_unregister_worktree(
        &mut self,
        envelope: TypedEnvelope<proto::UnregisterWorktree>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        self.worktrees
            .retain(|worktree| worktree.read(cx).as_remote().unwrap().id() != worktree_id);
        cx.notify();
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
            worktree.update(cx, |worktree, cx| {
                worktree
                    .as_remote_mut()
                    .unwrap()
                    .update_diagnostic_summary(envelope, cx);
            });
        }
        Ok(())
    }

    fn handle_disk_based_diagnostics_updating(
        &mut self,
        envelope: TypedEnvelope<proto::DiskBasedDiagnosticsUpdating>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            worktree.update(cx, |worktree, cx| {
                worktree
                    .as_remote()
                    .unwrap()
                    .disk_based_diagnostics_updating(cx);
            });
        }
        Ok(())
    }

    fn handle_disk_based_diagnostics_updated(
        &mut self,
        envelope: TypedEnvelope<proto::DiskBasedDiagnosticsUpdated>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            worktree.update(cx, |worktree, cx| {
                worktree
                    .as_remote()
                    .unwrap()
                    .disk_based_diagnostics_updated(cx);
            });
        }
        Ok(())
    }

    pub fn handle_update_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            worktree.update(cx, |worktree, cx| {
                worktree.handle_update_buffer(envelope, cx)
            })?;
        }
        Ok(())
    }

    pub fn handle_save_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            worktree.update(cx, |worktree, cx| {
                worktree.handle_save_buffer(envelope, rpc, cx)
            })?;
        }
        Ok(())
    }

    pub fn handle_format_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::FormatBuffer>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            worktree.update(cx, |worktree, cx| {
                worktree.handle_format_buffer(envelope, rpc, cx)
            })?;
        }
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
        let worktree = self
            .worktree_for_id(worktree_id, cx)
            .ok_or_else(|| anyhow!("no such worktree"))?;

        let task = self.open_buffer(
            ProjectPath {
                worktree_id,
                path: PathBuf::from(envelope.payload.path).into(),
            },
            cx,
        );
        cx.spawn(|_, mut cx| {
            async move {
                let buffer = task.await?;
                let response = worktree.update(&mut cx, |worktree, cx| {
                    worktree
                        .as_local_mut()
                        .unwrap()
                        .open_remote_buffer(peer_id, buffer, cx)
                });
                rpc.respond(receipt, response).await?;
                Ok(())
            }
            .log_err()
        })
        .detach();
        Ok(())
    }

    pub fn handle_close_buffer(
        &mut self,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            worktree.update(cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .close_remote_buffer(envelope, cx)
            })?;
        }
        Ok(())
    }

    pub fn handle_buffer_saved(
        &mut self,
        envelope: TypedEnvelope<proto::BufferSaved>,
        _: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        if let Some(worktree) = self.worktree_for_id(worktree_id, cx) {
            worktree.update(cx, |worktree, cx| {
                worktree.handle_buffer_saved(envelope, cx)
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
        let include_root_name = self.worktrees.len() > 1;
        let candidate_sets = self
            .worktrees
            .iter()
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

    fn release(&mut self, cx: &mut gpui::MutableAppContext) {
        match &self.client_state {
            ProjectClientState::Local { remote_id_rx, .. } => {
                if let Some(project_id) = *remote_id_rx.borrow() {
                    let rpc = self.client.clone();
                    cx.spawn(|_| async move {
                        if let Err(err) = rpc.send(proto::UnregisterProject { project_id }).await {
                            log::error!("error unregistering project: {}", err);
                        }
                    })
                    .detach();
                }
            }
            ProjectClientState::Remote { remote_id, .. } => {
                let rpc = self.client.clone();
                let project_id = *remote_id;
                cx.spawn(|_| async move {
                    if let Err(err) = rpc.send(proto::LeaveProject { project_id }).await {
                        log::error!("error leaving project: {}", err);
                    }
                })
                .detach();
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

#[cfg(test)]
mod tests {
    use super::*;
    use client::test::FakeHttpClient;
    use fs::RealFs;
    use futures::StreamExt;
    use gpui::{test::subscribe, TestAppContext};
    use language::{
        tree_sitter_rust, Diagnostic, LanguageConfig, LanguageRegistry, LanguageServerConfig, Point,
    };
    use lsp::Url;
    use serde_json::json;
    use std::{os::unix, path::PathBuf};
    use util::test::temp_tree;

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

        let project = build_project(&mut cx);

        let tree = project
            .update(&mut cx, |project, cx| {
                project.add_local_worktree(&root_link_path, cx)
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

        let tree = Worktree::open_local(
            client,
            user_store,
            dir.path(),
            Arc::new(RealFs),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        // Cause worktree to start the fake language server
        let _buffer = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("b.rs", cx))
            .await
            .unwrap();

        let mut events = subscribe(&tree, &mut cx);

        fake_server.start_progress(&progress_token).await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsUpdating
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
            Event::DiagnosticsUpdated(Arc::from(Path::new("a.rs")))
        );

        fake_server.end_progress(&progress_token).await;
        fake_server.end_progress(&progress_token).await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsUpdated
        );

        let (buffer, _) = tree
            .update(&mut cx, |tree, cx| tree.open_buffer("a.rs", cx))
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

        let project = build_project(&mut cx);
        let tree = project
            .update(&mut cx, |project, cx| {
                project.add_local_worktree(&dir.path(), cx)
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

    fn build_project(cx: &mut TestAppContext) -> ModelHandle<Project> {
        let languages = Arc::new(LanguageRegistry::new());
        let fs = Arc::new(RealFs);
        let http_client = FakeHttpClient::with_404_response();
        let client = client::Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        cx.update(|cx| Project::local(client, user_store, languages, fs, cx))
    }
}
