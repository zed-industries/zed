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
use language::{
    Bias, Buffer, DiagnosticEntry, File as _, Language, LanguageRegistry, ToOffset, ToPointUtf16,
};
use lsp::{DiagnosticSeverity, LanguageServer};
use postage::{prelude::Stream, watch};
use smol::block_on;
use std::{
    ops::Range,
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
    language_servers_with_diagnostics_running: isize,
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
    pub source_range: Option<Range<language::Anchor>>,
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
                                        for worktree in &this.worktrees {
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
                language_servers_with_diagnostics_running: 0,
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
                        tasks.push(worktree.share(cx));
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
                    this.assign_language_to_buffer(worktree, buffer.clone(), cx)
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
        let worktree_task = self.find_or_create_worktree_for_abs_path(&abs_path, cx);
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
                this.assign_language_to_buffer(worktree, buffer, cx)
            });
            Ok(())
        })
    }

    fn assign_language_to_buffer(
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
            hash_map::Entry::Vacant(e) => {
                Self::start_language_server(self.client.clone(), language, &worktree_abs_path, cx)
                    .map(|server| e.insert(server).clone())
            }
        };

        buffer.update(cx, |buffer, cx| {
            buffer.set_language_server(language_server, cx)
        });

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
                        let send = this.update(&mut cx, |this, cx| {
                            this.disk_based_diagnostics_started(cx);
                            this.remote_id().map(|project_id| {
                                rpc.send(proto::DiskBasedDiagnosticsUpdating { project_id })
                            })
                        });
                        if let Some(send) = send {
                            send.await.log_err();
                        }
                    }
                    LspEvent::DiagnosticsUpdate(params) => {
                        this.update(&mut cx, |this, cx| {
                            this.update_diagnostics(params, &disk_based_sources, cx)
                                .log_err();
                        });
                    }
                    LspEvent::DiagnosticsFinish => {
                        let send = this.update(&mut cx, |this, cx| {
                            this.disk_based_diagnostics_finished(cx);
                            this.remote_id().map(|project_id| {
                                rpc.send(proto::DiskBasedDiagnosticsUpdated { project_id })
                            })
                        });
                        if let Some(send) = send {
                            send.await.log_err();
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
        let (worktree, relative_path) = self
            .find_worktree_for_abs_path(&path, cx)
            .ok_or_else(|| anyhow!("no worktree found for diagnostics"))?;
        let project_path = ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: relative_path.into(),
        };
        worktree.update(cx, |worktree, cx| {
            worktree.as_local_mut().unwrap().update_diagnostics(
                project_path.path.clone(),
                diagnostics,
                disk_based_sources,
                cx,
            )
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
        let buffer = source_buffer_handle.read(cx);
        let worktree;
        let buffer_abs_path;
        if let Some(file) = File::from_dyn(buffer.file()) {
            worktree = file.worktree.clone();
            buffer_abs_path = file.abs_path();
        } else {
            return Task::ready(Err(anyhow!("buffer does not belong to any worktree")));
        };

        if worktree.read(cx).as_local().is_some() {
            let point = buffer.offset_to_point_utf16(position.to_offset(buffer));
            let buffer_abs_path = buffer_abs_path.unwrap();
            let lang_name;
            let lang_server;
            if let Some(lang) = buffer.language() {
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
                            unresolved_locations.push((None, loc.uri, loc.range));
                        }
                        lsp::GotoDefinitionResponse::Array(locs) => {
                            unresolved_locations
                                .extend(locs.into_iter().map(|l| (None, l.uri, l.range)));
                        }
                        lsp::GotoDefinitionResponse::Link(links) => {
                            unresolved_locations.extend(links.into_iter().map(|l| {
                                (
                                    l.origin_selection_range,
                                    l.target_uri,
                                    l.target_selection_range,
                                )
                            }));
                        }
                    }

                    for (source_range, target_uri, target_range) in unresolved_locations {
                        let abs_path = target_uri
                            .to_file_path()
                            .map_err(|_| anyhow!("invalid target path"))?;

                        let (worktree, relative_path) = if let Some(result) = this
                            .read_with(&cx, |this, cx| {
                                this.find_worktree_for_abs_path(&abs_path, cx)
                            }) {
                            result
                        } else {
                            let (worktree, relative_path) = this
                                .update(&mut cx, |this, cx| {
                                    this.create_worktree_for_abs_path(&abs_path, cx)
                                })
                                .await?;
                            this.update(&mut cx, |this, cx| {
                                this.language_servers.insert(
                                    (worktree.read(cx).id(), lang_name.clone()),
                                    lang_server.clone(),
                                );
                            });
                            (worktree, relative_path)
                        };

                        let project_path = ProjectPath {
                            worktree_id: worktree.read_with(&cx, |worktree, _| worktree.id()),
                            path: relative_path.into(),
                        };
                        let target_buffer_handle = this
                            .update(&mut cx, |this, cx| this.open_buffer(project_path, cx))
                            .await?;
                        cx.read(|cx| {
                            let source_buffer = source_buffer_handle.read(cx);
                            let target_buffer = target_buffer_handle.read(cx);
                            let source_range = source_range.map(|range| {
                                let start = source_buffer
                                    .clip_point_utf16(range.start.to_point_utf16(), Bias::Left);
                                let end = source_buffer
                                    .clip_point_utf16(range.end.to_point_utf16(), Bias::Left);
                                source_buffer.anchor_after(start)..source_buffer.anchor_before(end)
                            });
                            let target_start = target_buffer
                                .clip_point_utf16(target_range.start.to_point_utf16(), Bias::Left);
                            let target_end = target_buffer
                                .clip_point_utf16(target_range.end.to_point_utf16(), Bias::Left);
                            definitions.push(Definition {
                                source_range,
                                target_buffer: target_buffer_handle,
                                target_range: target_buffer.anchor_after(target_start)
                                    ..target_buffer.anchor_before(target_end),
                            });
                        });
                    }
                }

                Ok(definitions)
            })
        } else {
            todo!()
        }
    }

    pub fn find_or_create_worktree_for_abs_path(
        &self,
        abs_path: &Path,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(ModelHandle<Worktree>, PathBuf)>> {
        if let Some((tree, relative_path)) = self.find_worktree_for_abs_path(abs_path, cx) {
            Task::ready(Ok((tree.clone(), relative_path.into())))
        } else {
            self.create_worktree_for_abs_path(abs_path, cx)
        }
    }

    fn create_worktree_for_abs_path(
        &self,
        abs_path: &Path,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(ModelHandle<Worktree>, PathBuf)>> {
        let worktree = self.add_local_worktree(abs_path, cx);
        cx.background().spawn(async move {
            let worktree = worktree.await?;
            Ok((worktree, PathBuf::new()))
        })
    }

    fn find_worktree_for_abs_path(
        &self,
        abs_path: &Path,
        cx: &AppContext,
    ) -> Option<(ModelHandle<Worktree>, PathBuf)> {
        for tree in &self.worktrees {
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
                worktree
                    .update(&mut cx, |worktree, cx| {
                        worktree.as_local_mut().unwrap().register(project_id, cx)
                    })
                    .await?;
                if is_shared {
                    worktree
                        .update(&mut cx, |worktree, cx| {
                            worktree.as_local_mut().unwrap().share(cx)
                        })
                        .await?;
                }
            }

            Ok(worktree)
        })
    }

    pub fn remove_worktree(&mut self, id: WorktreeId, cx: &mut ModelContext<Self>) {
        self.worktrees
            .retain(|worktree| worktree.read(cx).id() != id);
        cx.notify();
    }

    fn add_worktree(&mut self, worktree: ModelHandle<Worktree>, cx: &mut ModelContext<Self>) {
        cx.observe(&worktree, |_, _, cx| cx.notify()).detach();
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
        self.worktrees.iter().flat_map(move |worktree| {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            worktree
                .diagnostic_summaries()
                .map(move |(path, summary)| (ProjectPath { worktree_id, path }, summary))
        })
    }

    fn disk_based_diagnostics_started(&mut self, cx: &mut ModelContext<Self>) {
        self.language_servers_with_diagnostics_running += 1;
        if self.language_servers_with_diagnostics_running == 1 {
            cx.emit(Event::DiskBasedDiagnosticsStarted);
        }
    }

    fn disk_based_diagnostics_finished(&mut self, cx: &mut ModelContext<Self>) {
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
    use super::{Event, *};
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

        let project = cx.update(|cx| {
            Project::local(
                client,
                user_store,
                Arc::new(languages),
                Arc::new(RealFs),
                cx,
            )
        });

        let tree = project
            .update(&mut cx, |project, cx| {
                project.add_local_worktree(dir.path(), cx)
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
