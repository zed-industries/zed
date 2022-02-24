pub mod fs;
mod ignore;
mod lsp_command;
pub mod worktree;

use aho_corasick::AhoCorasickBuilder;
use anyhow::{anyhow, Context, Result};
use client::{proto, Client, PeerId, TypedEnvelope, User, UserStore};
use clock::ReplicaId;
use collections::{hash_map, HashMap, HashSet};
use futures::{future::Shared, Future, FutureExt, StreamExt};
use fuzzy::{PathMatch, PathMatchCandidate, PathMatchCandidateSet};
use gpui::{
    AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task,
    UpgradeModelHandle, WeakModelHandle,
};
use language::{
    range_from_lsp, Anchor, AnchorRangeExt, Bias, Buffer, CodeAction, CodeLabel, Completion,
    Diagnostic, DiagnosticEntry, File as _, Language, LanguageRegistry, Operation, PointUtf16,
    ToLspPosition, ToOffset, ToPointUtf16, Transaction,
};
use lsp::{DiagnosticSeverity, DocumentHighlightKind, LanguageServer};
use lsp_command::*;
use postage::{broadcast, prelude::Stream, sink::Sink, watch};
use rand::prelude::*;
use sha2::{Digest, Sha256};
use smol::block_on;
use std::{
    cell::RefCell,
    convert::TryInto,
    hash::Hash,
    mem,
    ops::Range,
    path::{Component, Path, PathBuf},
    rc::Rc,
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};
use util::{post_inc, ResultExt, TryFutureExt as _};

pub use fs::*;
pub use worktree::*;

pub struct Project {
    worktrees: Vec<WorktreeHandle>,
    active_entry: Option<ProjectEntry>,
    languages: Arc<LanguageRegistry>,
    language_servers: HashMap<(WorktreeId, String), Arc<LanguageServer>>,
    started_language_servers:
        HashMap<(WorktreeId, String), Shared<Task<Option<Arc<LanguageServer>>>>>,
    client: Arc<client::Client>,
    user_store: ModelHandle<UserStore>,
    fs: Arc<dyn Fs>,
    client_state: ProjectClientState,
    collaborators: HashMap<PeerId, Collaborator>,
    subscriptions: Vec<client::Subscription>,
    language_servers_with_diagnostics_running: isize,
    opened_buffer: broadcast::Sender<()>,
    loading_buffers: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<ModelHandle<Buffer>, Arc<anyhow::Error>>>>,
    >,
    buffers_state: Rc<RefCell<ProjectBuffers>>,
    shared_buffers: HashMap<PeerId, HashMap<u64, ModelHandle<Buffer>>>,
    nonce: u128,
}

#[derive(Default)]
struct ProjectBuffers {
    buffer_request_count: usize,
    preserved_buffers: Vec<ModelHandle<Buffer>>,
    open_buffers: HashMap<u64, OpenBuffer>,
}

enum OpenBuffer {
    Loaded(WeakModelHandle<Buffer>),
    Loading(Vec<Operation>),
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
pub struct Location {
    pub buffer: ModelHandle<Buffer>,
    pub range: Range<language::Anchor>,
}

#[derive(Debug)]
pub struct DocumentHighlight {
    pub range: Range<language::Anchor>,
    pub kind: DocumentHighlightKind,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub source_worktree_id: WorktreeId,
    pub worktree_id: WorktreeId,
    pub language_name: String,
    pub path: PathBuf,
    pub label: CodeLabel,
    pub name: String,
    pub kind: lsp::SymbolKind,
    pub range: Range<PointUtf16>,
    pub signature: [u8; 32],
}

pub enum SearchQuery {
    Plain(String),
}

pub struct BufferRequestHandle(Rc<RefCell<ProjectBuffers>>);

#[derive(Default)]
pub struct ProjectTransaction(pub HashMap<ModelHandle<Buffer>, language::Transaction>);

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

    pub fn to_proto(&self, path: &Path) -> proto::DiagnosticSummary {
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
    pub fn init(client: &Arc<Client>) {
        client.add_entity_message_handler(Self::handle_add_collaborator);
        client.add_entity_message_handler(Self::handle_buffer_reloaded);
        client.add_entity_message_handler(Self::handle_buffer_saved);
        client.add_entity_message_handler(Self::handle_close_buffer);
        client.add_entity_message_handler(Self::handle_disk_based_diagnostics_updated);
        client.add_entity_message_handler(Self::handle_disk_based_diagnostics_updating);
        client.add_entity_message_handler(Self::handle_remove_collaborator);
        client.add_entity_message_handler(Self::handle_register_worktree);
        client.add_entity_message_handler(Self::handle_unregister_worktree);
        client.add_entity_message_handler(Self::handle_unshare_project);
        client.add_entity_message_handler(Self::handle_update_buffer_file);
        client.add_entity_message_handler(Self::handle_update_buffer);
        client.add_entity_message_handler(Self::handle_update_diagnostic_summary);
        client.add_entity_message_handler(Self::handle_update_worktree);
        client.add_entity_request_handler(Self::handle_apply_additional_edits_for_completion);
        client.add_entity_request_handler(Self::handle_apply_code_action);
        client.add_entity_request_handler(Self::handle_format_buffers);
        client.add_entity_request_handler(Self::handle_get_code_actions);
        client.add_entity_request_handler(Self::handle_get_completions);
        client.add_entity_request_handler(Self::handle_lsp_command::<GetDefinition>);
        client.add_entity_request_handler(Self::handle_lsp_command::<GetDocumentHighlights>);
        client.add_entity_request_handler(Self::handle_lsp_command::<GetReferences>);
        client.add_entity_request_handler(Self::handle_lsp_command::<PrepareRename>);
        client.add_entity_request_handler(Self::handle_lsp_command::<PerformRename>);
        client.add_entity_request_handler(Self::handle_get_project_symbols);
        client.add_entity_request_handler(Self::handle_open_buffer_for_symbol);
        client.add_entity_request_handler(Self::handle_open_buffer);
        client.add_entity_request_handler(Self::handle_save_buffer);
    }

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
                buffers_state: Default::default(),
                loading_buffers: Default::default(),
                shared_buffers: Default::default(),
                client_state: ProjectClientState::Local {
                    is_shared: false,
                    remote_id_tx,
                    remote_id_rx,
                    _maintain_remote_id_task,
                },
                opened_buffer: broadcast::channel(1).0,
                subscriptions: Vec::new(),
                active_entry: None,
                languages,
                client,
                user_store,
                fs,
                language_servers_with_diagnostics_running: 0,
                language_servers: Default::default(),
                started_language_servers: Default::default(),
                nonce: StdRng::from_entropy().gen(),
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

        let this = cx.add_model(|cx| {
            let mut this = Self {
                worktrees: Vec::new(),
                loading_buffers: Default::default(),
                opened_buffer: broadcast::channel(1).0,
                shared_buffers: Default::default(),
                active_entry: None,
                collaborators: Default::default(),
                languages,
                user_store: user_store.clone(),
                fs,
                subscriptions: vec![client.add_model_for_remote_entity(remote_id, cx)],
                client,
                client_state: ProjectClientState::Remote {
                    sharing_has_stopped: false,
                    remote_id,
                    replica_id,
                },
                language_servers_with_diagnostics_running: 0,
                language_servers: Default::default(),
                started_language_servers: Default::default(),
                buffers_state: Default::default(),
                nonce: StdRng::from_entropy().gen(),
            };
            for worktree in worktrees {
                this.add_worktree(&worktree, cx);
            }
            this
        });

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

        this.update(cx, |this, _| {
            this.collaborators = collaborators;
        });

        Ok(this)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(fs: Arc<dyn Fs>, cx: &mut gpui::TestAppContext) -> ModelHandle<Project> {
        let languages = Arc::new(LanguageRegistry::new());
        let http_client = client::test::FakeHttpClient::with_404_response();
        let client = client::Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        cx.update(|cx| Project::local(client, user_store, languages, fs, cx))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn shared_buffer(&self, peer_id: PeerId, remote_id: u64) -> Option<ModelHandle<Buffer>> {
        self.shared_buffers
            .get(&peer_id)
            .and_then(|buffers| buffers.get(&remote_id))
            .cloned()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn has_buffered_operations(&self) -> bool {
        self.buffers_state
            .borrow()
            .open_buffers
            .values()
            .any(|buffer| matches!(buffer, OpenBuffer::Loading(_)))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn languages(&self) -> &Arc<LanguageRegistry> {
        &self.languages
    }

    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    fn set_remote_id(&mut self, remote_id: Option<u64>, cx: &mut ModelContext<Self>) {
        if let ProjectClientState::Local { remote_id_tx, .. } = &mut self.client_state {
            *remote_id_tx.borrow_mut() = remote_id;
        }

        self.subscriptions.clear();
        if let Some(remote_id) = remote_id {
            self.subscriptions
                .push(self.client.add_model_for_remote_entity(remote_id, cx));
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

    pub fn strong_worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = ModelHandle<Worktree>> {
        self.worktrees.iter().filter_map(|worktree| {
            worktree.upgrade(cx).and_then(|worktree| {
                if worktree.read(cx).is_weak() {
                    None
                } else {
                    Some(worktree)
                }
            })
        })
    }

    pub fn worktree_for_id(
        &self,
        id: WorktreeId,
        cx: &AppContext,
    ) -> Option<ModelHandle<Worktree>> {
        self.worktrees(cx)
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn share(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
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

    pub fn unshare(&self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
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

    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }

    pub fn open_buffer(
        &mut self,
        path: impl Into<ProjectPath>,
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

        let mut loading_watch = match self.loading_buffers.entry(project_path.clone()) {
            // If the given path is already being loaded, then wait for that existing
            // task to complete and return the same buffer.
            hash_map::Entry::Occupied(e) => e.get().clone(),

            // Otherwise, record the fact that this path is now being loaded.
            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let load_buffer = if worktree.read(cx).is_local() {
                    self.open_local_buffer(&project_path.path, &worktree, cx)
                } else {
                    self.open_remote_buffer(&project_path.path, &worktree, cx)
                };

                cx.spawn(move |this, mut cx| async move {
                    let load_result = load_buffer.await;
                    *tx.borrow_mut() = Some(this.update(&mut cx, |this, _| {
                        // Record the fact that the buffer is no longer loading.
                        this.loading_buffers.remove(&project_path);
                        let buffer = load_result.map_err(Arc::new)?;
                        Ok(buffer)
                    }));
                })
                .detach();
                rx
            }
        };

        cx.foreground().spawn(async move {
            loop {
                if let Some(result) = loading_watch.borrow().as_ref() {
                    match result {
                        Ok(buffer) => return Ok(buffer.clone()),
                        Err(error) => return Err(anyhow!("{}", error)),
                    }
                }
                loading_watch.recv().await;
            }
        })
    }

    fn open_local_buffer(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let load_buffer = worktree.update(cx, |worktree, cx| {
            let worktree = worktree.as_local_mut().unwrap();
            worktree.load_buffer(path, cx)
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

    fn open_remote_buffer(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let rpc = self.client.clone();
        let project_id = self.remote_id().unwrap();
        let remote_worktree_id = worktree.read(cx).id();
        let path = path.clone();
        let path_string = path.to_string_lossy().to_string();
        let request_handle = self.start_buffer_request(cx);
        cx.spawn(|this, mut cx| async move {
            let response = rpc
                .request(proto::OpenBuffer {
                    project_id,
                    worktree_id: remote_worktree_id.to_proto(),
                    path: path_string,
                })
                .await?;
            let buffer = response.buffer.ok_or_else(|| anyhow!("missing buffer"))?;

            this.update(&mut cx, |this, cx| {
                this.deserialize_buffer(buffer, request_handle, cx)
            })
            .await
        })
    }

    fn open_local_buffer_via_lsp(
        &mut self,
        abs_path: lsp::Url,
        lang_name: String,
        lang_server: Arc<LanguageServer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        cx.spawn(|this, mut cx| async move {
            let abs_path = abs_path
                .to_file_path()
                .map_err(|_| anyhow!("can't convert URI to path"))?;
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
                    this.language_servers
                        .insert((worktree.read(cx).id(), lang_name), lang_server);
                });
                (worktree, PathBuf::new())
            };

            let project_path = ProjectPath {
                worktree_id: worktree.read_with(&cx, |worktree, _| worktree.id()),
                path: relative_path.into(),
            };
            this.update(&mut cx, |this, cx| this.open_buffer(project_path, cx))
                .await
        })
    }

    fn start_buffer_request(&self, cx: &AppContext) -> BufferRequestHandle {
        BufferRequestHandle::new(self.buffers_state.clone(), cx)
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
            self.buffers_state
                .borrow()
                .open_buffers
                .iter()
                .any(|(_, buffer)| {
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

    pub fn get_open_buffer(
        &mut self,
        path: &ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Option<ModelHandle<Buffer>> {
        let mut result = None;
        let worktree = self.worktree_for_id(path.worktree_id, cx)?;
        self.buffers_state
            .borrow_mut()
            .open_buffers
            .retain(|_, buffer| {
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
        let remote_id = buffer.read(cx).remote_id();
        match self
            .buffers_state
            .borrow_mut()
            .open_buffers
            .insert(remote_id, OpenBuffer::Loaded(buffer.downgrade()))
        {
            None => {}
            Some(OpenBuffer::Loading(operations)) => {
                buffer.update(cx, |buffer, cx| buffer.apply_ops(operations, cx))?
            }
            Some(OpenBuffer::Loaded(existing_handle)) => {
                if existing_handle.upgrade(cx).is_some() {
                    Err(anyhow!(
                        "already registered buffer with remote id {}",
                        remote_id
                    ))?
                }
            }
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
                let buffer = buffer.downgrade();
                let language_server =
                    self.start_language_server(worktree_id, worktree_abs_path, language, cx);

                cx.spawn_weak(|_, mut cx| async move {
                    if let Some(language_server) = language_server.await {
                        if let Some(buffer) = buffer.upgrade(&cx) {
                            buffer.update(&mut cx, |buffer, cx| {
                                buffer.set_language_server(Some(language_server), cx);
                            });
                        }
                    }
                })
                .detach();
            }
        }

        if let Some(local_worktree) = worktree.and_then(|w| w.read(cx).as_local()) {
            if let Some(diagnostics) = local_worktree.diagnostics_for_path(&path) {
                buffer.update(cx, |buffer, cx| {
                    buffer.update_diagnostics(diagnostics, None, cx).log_err();
                });
            }
        }

        None
    }

    fn start_language_server(
        &mut self,
        worktree_id: WorktreeId,
        worktree_path: Arc<Path>,
        language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) -> Shared<Task<Option<Arc<LanguageServer>>>> {
        enum LspEvent {
            DiagnosticsStart,
            DiagnosticsUpdate(lsp::PublishDiagnosticsParams),
            DiagnosticsFinish,
        }

        let key = (worktree_id, language.name().to_string());
        self.started_language_servers
            .entry(key.clone())
            .or_insert_with(|| {
                let language_server = self.languages.start_language_server(
                    &language,
                    worktree_path,
                    self.client.http_client(),
                    cx,
                );
                let rpc = self.client.clone();
                cx.spawn_weak(|this, mut cx| async move {
                    let language_server = language_server?.await.log_err()?;
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(&mut cx, |this, _| {
                            this.language_servers.insert(key, language_server.clone());
                        });
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

                    // Listen for `PublishDiagnostics` notifications.
                    language_server
                        .on_notification::<lsp::notification::PublishDiagnostics, _>({
                            let diagnostics_tx = diagnostics_tx.clone();
                            move |params| {
                                if !has_disk_based_diagnostic_progress_token {
                                    block_on(diagnostics_tx.send(LspEvent::DiagnosticsStart)).ok();
                                }
                                block_on(diagnostics_tx.send(LspEvent::DiagnosticsUpdate(params)))
                                    .ok();
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
                                    lsp::ProgressParamsValue::WorkDone(progress) => {
                                        match progress {
                                            lsp::WorkDoneProgress::Begin(_) => {
                                                running_jobs_for_this_server += 1;
                                                if running_jobs_for_this_server == 1 {
                                                    block_on(
                                                        diagnostics_tx
                                                            .send(LspEvent::DiagnosticsStart),
                                                    )
                                                    .ok();
                                                }
                                            }
                                            lsp::WorkDoneProgress::End(_) => {
                                                running_jobs_for_this_server -= 1;
                                                if running_jobs_for_this_server == 0 {
                                                    block_on(
                                                        diagnostics_tx
                                                            .send(LspEvent::DiagnosticsFinish),
                                                    )
                                                    .ok();
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        })
                        .detach();

                    // Process all the LSP events.
                    cx.spawn(|mut cx| async move {
                        while let Ok(message) = diagnostics_rx.recv().await {
                            let this = this.upgrade(&cx)?;
                            match message {
                                LspEvent::DiagnosticsStart => {
                                    this.update(&mut cx, |this, cx| {
                                        this.disk_based_diagnostics_started(cx);
                                        if let Some(project_id) = this.remote_id() {
                                            rpc.send(proto::DiskBasedDiagnosticsUpdating {
                                                project_id,
                                            })
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
                                            rpc.send(proto::DiskBasedDiagnosticsUpdated {
                                                project_id,
                                            })
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
                })
                .shared()
            })
            .clone()
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

        for buffer in self.buffers_state.borrow().open_buffers.values() {
            if let Some(buffer) = buffer.upgrade(cx) {
                if buffer
                    .read(cx)
                    .file()
                    .map_or(false, |file| *file.path() == project_path.path)
                {
                    buffer.update(cx, |buffer, cx| {
                        buffer.update_diagnostics(diagnostics.clone(), version, cx)
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

    pub fn format(
        &self,
        buffers: HashSet<ModelHandle<Buffer>>,
        push_to_history: bool,
        cx: &mut ModelContext<Project>,
    ) -> Task<Result<ProjectTransaction>> {
        let mut local_buffers = Vec::new();
        let mut remote_buffers = None;
        for buffer_handle in buffers {
            let buffer = buffer_handle.read(cx);
            let worktree;
            if let Some(file) = File::from_dyn(buffer.file()) {
                worktree = file.worktree.clone();
                if let Some(buffer_abs_path) = file.as_local().map(|f| f.abs_path(cx)) {
                    let lang_server;
                    if let Some(lang) = buffer.language() {
                        if let Some(server) = self
                            .language_servers
                            .get(&(worktree.read(cx).id(), lang.name().to_string()))
                        {
                            lang_server = server.clone();
                        } else {
                            return Task::ready(Ok(Default::default()));
                        };
                    } else {
                        return Task::ready(Ok(Default::default()));
                    }

                    local_buffers.push((buffer_handle, buffer_abs_path, lang_server));
                } else {
                    remote_buffers.get_or_insert(Vec::new()).push(buffer_handle);
                }
            } else {
                return Task::ready(Ok(Default::default()));
            }
        }

        let remote_buffers = self.remote_id().zip(remote_buffers);
        let client = self.client.clone();
        let request_handle = self.start_buffer_request(cx);

        cx.spawn(|this, mut cx| async move {
            let mut project_transaction = ProjectTransaction::default();

            if let Some((project_id, remote_buffers)) = remote_buffers {
                let response = client
                    .request(proto::FormatBuffers {
                        project_id,
                        buffer_ids: remote_buffers
                            .iter()
                            .map(|buffer| buffer.read_with(&cx, |buffer, _| buffer.remote_id()))
                            .collect(),
                    })
                    .await?
                    .transaction
                    .ok_or_else(|| anyhow!("missing transaction"))?;
                project_transaction = this
                    .update(&mut cx, |this, cx| {
                        this.deserialize_project_transaction(
                            response,
                            push_to_history,
                            request_handle,
                            cx,
                        )
                    })
                    .await?;
            }

            for (buffer, buffer_abs_path, lang_server) in local_buffers {
                let lsp_edits = lang_server
                    .request::<lsp::request::Formatting>(lsp::DocumentFormattingParams {
                        text_document: lsp::TextDocumentIdentifier::new(
                            lsp::Url::from_file_path(&buffer_abs_path).unwrap(),
                        ),
                        options: Default::default(),
                        work_done_progress_params: Default::default(),
                    })
                    .await?;

                if let Some(lsp_edits) = lsp_edits {
                    let edits = buffer
                        .update(&mut cx, |buffer, cx| {
                            buffer.edits_from_lsp(lsp_edits, None, cx)
                        })
                        .await?;
                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();
                        for (range, text) in edits {
                            buffer.edit([range], text, cx);
                        }
                        if buffer.end_transaction(cx).is_some() {
                            let transaction = buffer.finalize_last_transaction().unwrap().clone();
                            if !push_to_history {
                                buffer.forget_transaction(transaction.id);
                            }
                            project_transaction.0.insert(cx.handle(), transaction);
                        }
                    });
                }
            }

            Ok(project_transaction)
        })
    }

    pub fn definition<T: ToPointUtf16>(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Location>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(buffer.clone(), GetDefinition { position }, cx)
    }

    pub fn references<T: ToPointUtf16>(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Location>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(buffer.clone(), GetReferences { position }, cx)
    }

    pub fn document_highlights<T: ToPointUtf16>(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<DocumentHighlight>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(buffer.clone(), GetDocumentHighlights { position }, cx)
    }

    pub fn symbols(&self, query: &str, cx: &mut ModelContext<Self>) -> Task<Result<Vec<Symbol>>> {
        if self.is_local() {
            let mut language_servers = HashMap::default();
            for ((worktree_id, language_name), language_server) in self.language_servers.iter() {
                if let Some((worktree, language)) = self
                    .worktree_for_id(*worktree_id, cx)
                    .and_then(|worktree| worktree.read(cx).as_local())
                    .zip(self.languages.get_language(language_name))
                {
                    language_servers
                        .entry(Arc::as_ptr(language_server))
                        .or_insert((
                            language_server.clone(),
                            *worktree_id,
                            worktree.abs_path().clone(),
                            language.clone(),
                        ));
                }
            }

            let mut requests = Vec::new();
            for (language_server, _, _, _) in language_servers.values() {
                requests.push(language_server.request::<lsp::request::WorkspaceSymbol>(
                    lsp::WorkspaceSymbolParams {
                        query: query.to_string(),
                        ..Default::default()
                    },
                ));
            }

            cx.spawn_weak(|this, cx| async move {
                let responses = futures::future::try_join_all(requests).await?;

                let mut symbols = Vec::new();
                if let Some(this) = this.upgrade(&cx) {
                    this.read_with(&cx, |this, cx| {
                        for ((_, source_worktree_id, worktree_abs_path, language), lsp_symbols) in
                            language_servers.into_values().zip(responses)
                        {
                            symbols.extend(lsp_symbols.into_iter().flatten().filter_map(
                                |lsp_symbol| {
                                    let abs_path = lsp_symbol.location.uri.to_file_path().ok()?;
                                    let mut worktree_id = source_worktree_id;
                                    let path;
                                    if let Some((worktree, rel_path)) =
                                        this.find_local_worktree(&abs_path, cx)
                                    {
                                        worktree_id = worktree.read(cx).id();
                                        path = rel_path;
                                    } else {
                                        path = relativize_path(&worktree_abs_path, &abs_path);
                                    }

                                    let label = language
                                        .label_for_symbol(&lsp_symbol.name, lsp_symbol.kind)
                                        .unwrap_or_else(|| {
                                            CodeLabel::plain(lsp_symbol.name.clone(), None)
                                        });
                                    let signature = this.symbol_signature(worktree_id, &path);

                                    Some(Symbol {
                                        source_worktree_id,
                                        worktree_id,
                                        language_name: language.name().to_string(),
                                        name: lsp_symbol.name,
                                        kind: lsp_symbol.kind,
                                        label,
                                        path,
                                        range: range_from_lsp(lsp_symbol.location.range),
                                        signature,
                                    })
                                },
                            ));
                        }
                    })
                }

                Ok(symbols)
            })
        } else if let Some(project_id) = self.remote_id() {
            let request = self.client.request(proto::GetProjectSymbols {
                project_id,
                query: query.to_string(),
            });
            cx.spawn_weak(|this, cx| async move {
                let response = request.await?;
                let mut symbols = Vec::new();
                if let Some(this) = this.upgrade(&cx) {
                    this.read_with(&cx, |this, _| {
                        symbols.extend(
                            response
                                .symbols
                                .into_iter()
                                .filter_map(|symbol| this.deserialize_symbol(symbol).log_err()),
                        );
                    })
                }
                Ok(symbols)
            })
        } else {
            Task::ready(Ok(Default::default()))
        }
    }

    pub fn open_buffer_for_symbol(
        &mut self,
        symbol: &Symbol,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        if self.is_local() {
            let language_server = if let Some(server) = self
                .language_servers
                .get(&(symbol.source_worktree_id, symbol.language_name.clone()))
            {
                server.clone()
            } else {
                return Task::ready(Err(anyhow!(
                    "language server for worktree and language not found"
                )));
            };

            let worktree_abs_path = if let Some(worktree_abs_path) = self
                .worktree_for_id(symbol.worktree_id, cx)
                .and_then(|worktree| worktree.read(cx).as_local())
                .map(|local_worktree| local_worktree.abs_path())
            {
                worktree_abs_path
            } else {
                return Task::ready(Err(anyhow!("worktree not found for symbol")));
            };
            let symbol_abs_path = worktree_abs_path.join(&symbol.path);
            let symbol_uri = if let Ok(uri) = lsp::Url::from_file_path(symbol_abs_path) {
                uri
            } else {
                return Task::ready(Err(anyhow!("invalid symbol path")));
            };

            self.open_local_buffer_via_lsp(
                symbol_uri,
                symbol.language_name.clone(),
                language_server,
                cx,
            )
        } else if let Some(project_id) = self.remote_id() {
            let request_handle = self.start_buffer_request(cx);
            let request = self.client.request(proto::OpenBufferForSymbol {
                project_id,
                symbol: Some(serialize_symbol(symbol)),
            });
            cx.spawn(|this, mut cx| async move {
                let response = request.await?;
                let buffer = response.buffer.ok_or_else(|| anyhow!("invalid buffer"))?;
                this.update(&mut cx, |this, cx| {
                    this.deserialize_buffer(buffer, request_handle, cx)
                })
                .await
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    pub fn completions<T: ToPointUtf16>(
        &self,
        source_buffer_handle: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        let source_buffer_handle = source_buffer_handle.clone();
        let source_buffer = source_buffer_handle.read(cx);
        let buffer_id = source_buffer.remote_id();
        let language = source_buffer.language().cloned();
        let worktree;
        let buffer_abs_path;
        if let Some(file) = File::from_dyn(source_buffer.file()) {
            worktree = file.worktree.clone();
            buffer_abs_path = file.as_local().map(|f| f.abs_path(cx));
        } else {
            return Task::ready(Ok(Default::default()));
        };

        let position = position.to_point_utf16(source_buffer);
        let anchor = source_buffer.anchor_after(position);

        if worktree.read(cx).as_local().is_some() {
            let buffer_abs_path = buffer_abs_path.unwrap();
            let lang_server = if let Some(server) = source_buffer.language_server().cloned() {
                server
            } else {
                return Task::ready(Ok(Default::default()));
            };

            cx.spawn(|_, cx| async move {
                let completions = lang_server
                    .request::<lsp::request::Completion>(lsp::CompletionParams {
                        text_document_position: lsp::TextDocumentPositionParams::new(
                            lsp::TextDocumentIdentifier::new(
                                lsp::Url::from_file_path(buffer_abs_path).unwrap(),
                            ),
                            position.to_lsp_position(),
                        ),
                        context: Default::default(),
                        work_done_progress_params: Default::default(),
                        partial_result_params: Default::default(),
                    })
                    .await
                    .context("lsp completion request failed")?;

                let completions = if let Some(completions) = completions {
                    match completions {
                        lsp::CompletionResponse::Array(completions) => completions,
                        lsp::CompletionResponse::List(list) => list.items,
                    }
                } else {
                    Default::default()
                };

                source_buffer_handle.read_with(&cx, |this, _| {
                    Ok(completions
                        .into_iter()
                        .filter_map(|lsp_completion| {
                            let (old_range, new_text) = match lsp_completion.text_edit.as_ref()? {
                                lsp::CompletionTextEdit::Edit(edit) => {
                                    (range_from_lsp(edit.range), edit.new_text.clone())
                                }
                                lsp::CompletionTextEdit::InsertAndReplace(_) => {
                                    log::info!("unsupported insert/replace completion");
                                    return None;
                                }
                            };

                            let clipped_start = this.clip_point_utf16(old_range.start, Bias::Left);
                            let clipped_end = this.clip_point_utf16(old_range.end, Bias::Left);
                            if clipped_start == old_range.start && clipped_end == old_range.end {
                                Some(Completion {
                                    old_range: this.anchor_before(old_range.start)
                                        ..this.anchor_after(old_range.end),
                                    new_text,
                                    label: language
                                        .as_ref()
                                        .and_then(|l| l.label_for_completion(&lsp_completion))
                                        .unwrap_or_else(|| {
                                            CodeLabel::plain(
                                                lsp_completion.label.clone(),
                                                lsp_completion.filter_text.as_deref(),
                                            )
                                        }),
                                    lsp_completion,
                                })
                            } else {
                                None
                            }
                        })
                        .collect())
                })
            })
        } else if let Some(project_id) = self.remote_id() {
            let rpc = self.client.clone();
            let message = proto::GetCompletions {
                project_id,
                buffer_id,
                position: Some(language::proto::serialize_anchor(&anchor)),
                version: (&source_buffer.version()).into(),
            };
            cx.spawn_weak(|_, mut cx| async move {
                let response = rpc.request(message).await?;

                source_buffer_handle
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(response.version.into())
                    })
                    .await;

                response
                    .completions
                    .into_iter()
                    .map(|completion| {
                        language::proto::deserialize_completion(completion, language.as_ref())
                    })
                    .collect()
            })
        } else {
            Task::ready(Ok(Default::default()))
        }
    }

    pub fn apply_additional_edits_for_completion(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        completion: Completion,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id();

        if self.is_local() {
            let lang_server = if let Some(language_server) = buffer.language_server() {
                language_server.clone()
            } else {
                return Task::ready(Err(anyhow!("buffer does not have a language server")));
            };

            cx.spawn(|_, mut cx| async move {
                let resolved_completion = lang_server
                    .request::<lsp::request::ResolveCompletionItem>(completion.lsp_completion)
                    .await?;
                if let Some(edits) = resolved_completion.additional_text_edits {
                    let edits = buffer_handle
                        .update(&mut cx, |buffer, cx| buffer.edits_from_lsp(edits, None, cx))
                        .await?;
                    buffer_handle.update(&mut cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();
                        for (range, text) in edits {
                            buffer.edit([range], text, cx);
                        }
                        let transaction = if buffer.end_transaction(cx).is_some() {
                            let transaction = buffer.finalize_last_transaction().unwrap().clone();
                            if !push_to_history {
                                buffer.forget_transaction(transaction.id);
                            }
                            Some(transaction)
                        } else {
                            None
                        };
                        Ok(transaction)
                    })
                } else {
                    Ok(None)
                }
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
            cx.spawn(|_, mut cx| async move {
                let response = client
                    .request(proto::ApplyCompletionAdditionalEdits {
                        project_id,
                        buffer_id,
                        completion: Some(language::proto::serialize_completion(&completion)),
                    })
                    .await?;

                if let Some(transaction) = response.transaction {
                    let transaction = language::proto::deserialize_transaction(transaction)?;
                    buffer_handle
                        .update(&mut cx, |buffer, _| {
                            buffer.wait_for_edits(transaction.edit_ids.iter().copied())
                        })
                        .await;
                    if push_to_history {
                        buffer_handle.update(&mut cx, |buffer, _| {
                            buffer.push_transaction(transaction.clone(), Instant::now());
                        });
                    }
                    Ok(Some(transaction))
                } else {
                    Ok(None)
                }
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    pub fn code_actions<T: ToOffset>(
        &self,
        buffer_handle: &ModelHandle<Buffer>,
        range: Range<T>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        let buffer_handle = buffer_handle.clone();
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id();
        let worktree;
        let buffer_abs_path;
        if let Some(file) = File::from_dyn(buffer.file()) {
            worktree = file.worktree.clone();
            buffer_abs_path = file.as_local().map(|f| f.abs_path(cx));
        } else {
            return Task::ready(Ok(Default::default()));
        };
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);

        if worktree.read(cx).as_local().is_some() {
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
                    return Task::ready(Ok(Default::default()));
                };
            } else {
                return Task::ready(Ok(Default::default()));
            }

            let lsp_range = lsp::Range::new(
                range.start.to_point_utf16(buffer).to_lsp_position(),
                range.end.to_point_utf16(buffer).to_lsp_position(),
            );
            cx.foreground().spawn(async move {
                Ok(lang_server
                    .request::<lsp::request::CodeActionRequest>(lsp::CodeActionParams {
                        text_document: lsp::TextDocumentIdentifier::new(
                            lsp::Url::from_file_path(buffer_abs_path).unwrap(),
                        ),
                        range: lsp_range,
                        work_done_progress_params: Default::default(),
                        partial_result_params: Default::default(),
                        context: lsp::CodeActionContext {
                            diagnostics: Default::default(),
                            only: Some(vec![
                                lsp::CodeActionKind::QUICKFIX,
                                lsp::CodeActionKind::REFACTOR,
                                lsp::CodeActionKind::REFACTOR_EXTRACT,
                            ]),
                        },
                    })
                    .await?
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|entry| {
                        if let lsp::CodeActionOrCommand::CodeAction(lsp_action) = entry {
                            Some(CodeAction {
                                range: range.clone(),
                                lsp_action,
                            })
                        } else {
                            None
                        }
                    })
                    .collect())
            })
        } else if let Some(project_id) = self.remote_id() {
            let rpc = self.client.clone();
            cx.spawn_weak(|_, mut cx| async move {
                let response = rpc
                    .request(proto::GetCodeActions {
                        project_id,
                        buffer_id,
                        start: Some(language::proto::serialize_anchor(&range.start)),
                        end: Some(language::proto::serialize_anchor(&range.end)),
                    })
                    .await?;

                buffer_handle
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(response.version.into())
                    })
                    .await;

                response
                    .actions
                    .into_iter()
                    .map(language::proto::deserialize_code_action)
                    .collect()
            })
        } else {
            Task::ready(Ok(Default::default()))
        }
    }

    pub fn apply_code_action(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        mut action: CodeAction,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        if self.is_local() {
            let buffer = buffer_handle.read(cx);
            let lang_name = if let Some(lang) = buffer.language() {
                lang.name().to_string()
            } else {
                return Task::ready(Ok(Default::default()));
            };
            let lang_server = if let Some(language_server) = buffer.language_server() {
                language_server.clone()
            } else {
                return Task::ready(Err(anyhow!("buffer does not have a language server")));
            };
            let range = action.range.to_point_utf16(buffer);

            cx.spawn(|this, mut cx| async move {
                if let Some(lsp_range) = action
                    .lsp_action
                    .data
                    .as_mut()
                    .and_then(|d| d.get_mut("codeActionParams"))
                    .and_then(|d| d.get_mut("range"))
                {
                    *lsp_range = serde_json::to_value(&lsp::Range::new(
                        range.start.to_lsp_position(),
                        range.end.to_lsp_position(),
                    ))
                    .unwrap();
                    action.lsp_action = lang_server
                        .request::<lsp::request::CodeActionResolveRequest>(action.lsp_action)
                        .await?;
                } else {
                    let actions = this
                        .update(&mut cx, |this, cx| {
                            this.code_actions(&buffer_handle, action.range, cx)
                        })
                        .await?;
                    action.lsp_action = actions
                        .into_iter()
                        .find(|a| a.lsp_action.title == action.lsp_action.title)
                        .ok_or_else(|| anyhow!("code action is outdated"))?
                        .lsp_action;
                }

                if let Some(edit) = action.lsp_action.edit {
                    Self::deserialize_workspace_edit(
                        this,
                        edit,
                        push_to_history,
                        lang_name,
                        lang_server,
                        &mut cx,
                    )
                    .await
                } else {
                    Ok(ProjectTransaction::default())
                }
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
            let request_handle = self.start_buffer_request(cx);
            let request = proto::ApplyCodeAction {
                project_id,
                buffer_id: buffer_handle.read(cx).remote_id(),
                action: Some(language::proto::serialize_code_action(&action)),
            };
            cx.spawn(|this, mut cx| async move {
                let response = client
                    .request(request)
                    .await?
                    .transaction
                    .ok_or_else(|| anyhow!("missing transaction"))?;
                this.update(&mut cx, |this, cx| {
                    this.deserialize_project_transaction(
                        response,
                        push_to_history,
                        request_handle,
                        cx,
                    )
                })
                .await
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    async fn deserialize_workspace_edit(
        this: ModelHandle<Self>,
        edit: lsp::WorkspaceEdit,
        push_to_history: bool,
        language_name: String,
        language_server: Arc<LanguageServer>,
        cx: &mut AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        let fs = this.read_with(cx, |this, _| this.fs.clone());
        let mut operations = Vec::new();
        if let Some(document_changes) = edit.document_changes {
            match document_changes {
                lsp::DocumentChanges::Edits(edits) => {
                    operations.extend(edits.into_iter().map(lsp::DocumentChangeOperation::Edit))
                }
                lsp::DocumentChanges::Operations(ops) => operations = ops,
            }
        } else if let Some(changes) = edit.changes {
            operations.extend(changes.into_iter().map(|(uri, edits)| {
                lsp::DocumentChangeOperation::Edit(lsp::TextDocumentEdit {
                    text_document: lsp::OptionalVersionedTextDocumentIdentifier {
                        uri,
                        version: None,
                    },
                    edits: edits.into_iter().map(lsp::OneOf::Left).collect(),
                })
            }));
        }

        let mut project_transaction = ProjectTransaction::default();
        for operation in operations {
            match operation {
                lsp::DocumentChangeOperation::Op(lsp::ResourceOp::Create(op)) => {
                    let abs_path = op
                        .uri
                        .to_file_path()
                        .map_err(|_| anyhow!("can't convert URI to path"))?;

                    if let Some(parent_path) = abs_path.parent() {
                        fs.create_dir(parent_path).await?;
                    }
                    if abs_path.ends_with("/") {
                        fs.create_dir(&abs_path).await?;
                    } else {
                        fs.create_file(&abs_path, op.options.map(Into::into).unwrap_or_default())
                            .await?;
                    }
                }
                lsp::DocumentChangeOperation::Op(lsp::ResourceOp::Rename(op)) => {
                    let source_abs_path = op
                        .old_uri
                        .to_file_path()
                        .map_err(|_| anyhow!("can't convert URI to path"))?;
                    let target_abs_path = op
                        .new_uri
                        .to_file_path()
                        .map_err(|_| anyhow!("can't convert URI to path"))?;
                    fs.rename(
                        &source_abs_path,
                        &target_abs_path,
                        op.options.map(Into::into).unwrap_or_default(),
                    )
                    .await?;
                }
                lsp::DocumentChangeOperation::Op(lsp::ResourceOp::Delete(op)) => {
                    let abs_path = op
                        .uri
                        .to_file_path()
                        .map_err(|_| anyhow!("can't convert URI to path"))?;
                    let options = op.options.map(Into::into).unwrap_or_default();
                    if abs_path.ends_with("/") {
                        fs.remove_dir(&abs_path, options).await?;
                    } else {
                        fs.remove_file(&abs_path, options).await?;
                    }
                }
                lsp::DocumentChangeOperation::Edit(op) => {
                    let buffer_to_edit = this
                        .update(cx, |this, cx| {
                            this.open_local_buffer_via_lsp(
                                op.text_document.uri,
                                language_name.clone(),
                                language_server.clone(),
                                cx,
                            )
                        })
                        .await?;

                    let edits = buffer_to_edit
                        .update(cx, |buffer, cx| {
                            let edits = op.edits.into_iter().map(|edit| match edit {
                                lsp::OneOf::Left(edit) => edit,
                                lsp::OneOf::Right(edit) => edit.text_edit,
                            });
                            buffer.edits_from_lsp(edits, op.text_document.version, cx)
                        })
                        .await?;

                    let transaction = buffer_to_edit.update(cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();
                        for (range, text) in edits {
                            buffer.edit([range], text, cx);
                        }
                        let transaction = if buffer.end_transaction(cx).is_some() {
                            let transaction = buffer.finalize_last_transaction().unwrap().clone();
                            if !push_to_history {
                                buffer.forget_transaction(transaction.id);
                            }
                            Some(transaction)
                        } else {
                            None
                        };

                        transaction
                    });
                    if let Some(transaction) = transaction {
                        project_transaction.0.insert(buffer_to_edit, transaction);
                    }
                }
            }
        }

        Ok(project_transaction)
    }

    pub fn prepare_rename<T: ToPointUtf16>(
        &self,
        buffer: ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Range<Anchor>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(buffer, PrepareRename { position }, cx)
    }

    pub fn perform_rename<T: ToPointUtf16>(
        &self,
        buffer: ModelHandle<Buffer>,
        position: T,
        new_name: String,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(
            buffer,
            PerformRename {
                position,
                new_name,
                push_to_history,
            },
            cx,
        )
    }

    pub fn search(
        &self,
        query: SearchQuery,
        cx: &mut ModelContext<Self>,
    ) -> Task<HashMap<ModelHandle<Buffer>, Vec<Range<Anchor>>>> {
        if self.is_local() {
            let (paths_to_search_tx, paths_to_search_rx) = smol::channel::bounded(1024);

            let snapshots = self
                .strong_worktrees(cx)
                .filter_map(|tree| {
                    let tree = tree.read(cx).as_local()?;
                    Some((tree.abs_path().clone(), tree.snapshot()))
                })
                .collect::<Vec<_>>();
            cx.background()
                .spawn(async move {
                    for (snapshot_abs_path, snapshot) in snapshots {
                        for file in snapshot.files(false, 0) {
                            if paths_to_search_tx
                                .send((snapshot.id(), snapshot_abs_path.clone(), file.path.clone()))
                                .await
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                })
                .detach();

            let SearchQuery::Plain(query) = query;
            let search = Arc::new(
                AhoCorasickBuilder::new()
                    .auto_configure(&[&query])
                    // .ascii_case_insensitive(!case_sensitive)
                    .build(&[&query]),
            );
            let (matching_paths_tx, mut matching_paths_rx) = smol::channel::bounded(1024);
            let workers = cx.background().num_cpus();
            cx.background()
                .spawn({
                    let fs = self.fs.clone();
                    let background = cx.background().clone();
                    let search = search.clone();
                    async move {
                        let fs = &fs;
                        let search = &search;
                        let matching_paths_tx = &matching_paths_tx;
                        background
                            .scoped(|scope| {
                                for _ in 0..workers {
                                    let mut paths_to_search_rx = paths_to_search_rx.clone();
                                    scope.spawn(async move {
                                        let mut path = PathBuf::new();
                                        while let Some((
                                            worktree_id,
                                            snapshot_abs_path,
                                            file_path,
                                        )) = paths_to_search_rx.next().await
                                        {
                                            if matching_paths_tx.is_closed() {
                                                break;
                                            }

                                            path.clear();
                                            path.push(&snapshot_abs_path);
                                            path.push(&file_path);
                                            let matches = if let Some(file) =
                                                fs.open_sync(&path).await.log_err()
                                            {
                                                search
                                                    .stream_find_iter(file)
                                                    .next()
                                                    .map_or(false, |mat| mat.is_ok())
                                            } else {
                                                false
                                            };

                                            if matches {
                                                if matching_paths_tx
                                                    .send((worktree_id, file_path))
                                                    .await
                                                    .is_err()
                                                {
                                                    break;
                                                }
                                            }
                                        }
                                    });
                                }
                            })
                            .await;
                    }
                })
                .detach();

            let (buffers_tx, buffers_rx) = smol::channel::bounded(1024);
            let buffers = self
                .buffers_state
                .borrow()
                .open_buffers
                .values()
                .filter_map(|b| b.upgrade(cx))
                .collect::<HashSet<_>>();
            cx.spawn(|this, mut cx| async move {
                for buffer in buffers {
                    let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot());
                    buffers_tx.send((buffer, snapshot)).await?;
                }

                while let Some(project_path) = matching_paths_rx.next().await {
                    if let Some(buffer) = this
                        .update(&mut cx, |this, cx| this.open_buffer(project_path, cx))
                        .await
                        .log_err()
                    {
                        let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot());
                        buffers_tx.send((buffer, snapshot)).await?;
                    }
                }

                Ok::<_, anyhow::Error>(())
            })
            .detach_and_log_err(cx);

            let background = cx.background().clone();
            cx.background().spawn(async move {
                let search = &search;
                let mut matched_buffers = Vec::new();
                for _ in 0..workers {
                    matched_buffers.push(HashMap::default());
                }
                background
                    .scoped(|scope| {
                        for worker_matched_buffers in matched_buffers.iter_mut() {
                            let mut buffers_rx = buffers_rx.clone();
                            scope.spawn(async move {
                                while let Some((buffer, snapshot)) = buffers_rx.next().await {
                                    for mat in search.stream_find_iter(
                                        snapshot.as_rope().bytes_in_range(0..snapshot.len()),
                                    ) {
                                        let mat = mat.unwrap();
                                        let range = snapshot.anchor_before(mat.start())
                                            ..snapshot.anchor_after(mat.end());
                                        worker_matched_buffers
                                            .entry(buffer.clone())
                                            .or_insert(Vec::new())
                                            .push(range);
                                    }
                                }
                            });
                        }
                    })
                    .await;
                matched_buffers.into_iter().flatten().collect()
            })
        } else {
            todo!()
        }
    }

    fn request_lsp<R: LspCommand>(
        &self,
        buffer_handle: ModelHandle<Buffer>,
        request: R,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<R::Response>>
    where
        <R::LspRequest as lsp::request::Request>::Result: Send,
    {
        let buffer = buffer_handle.read(cx);
        if self.is_local() {
            let file = File::from_dyn(buffer.file()).and_then(File::as_local);
            if let Some((file, language_server)) = file.zip(buffer.language_server().cloned()) {
                let lsp_params = request.to_lsp(&file.abs_path(cx), cx);
                return cx.spawn(|this, cx| async move {
                    let response = language_server
                        .request::<R::LspRequest>(lsp_params)
                        .await
                        .context("lsp request failed")?;
                    request
                        .response_from_lsp(response, this, buffer_handle, cx)
                        .await
                });
            }
        } else if let Some(project_id) = self.remote_id() {
            let rpc = self.client.clone();
            let request_handle = self.start_buffer_request(cx);
            let message = request.to_proto(project_id, buffer);
            return cx.spawn(|this, cx| async move {
                let response = rpc.request(message).await?;
                request
                    .response_from_proto(response, this, buffer_handle, request_handle, cx)
                    .await
            });
        }
        Task::ready(Ok(Default::default()))
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

    pub fn find_local_worktree(
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
        for (buffer_id, buffer) in &self.buffers_state.borrow().open_buffers {
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
            self.buffers_state
                .borrow_mut()
                .open_buffers
                .remove(&buffer_id);
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

    async fn handle_unshare_project(
        this: ModelHandle<Self>,
        _: TypedEnvelope<proto::UnshareProject>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            if let ProjectClientState::Remote {
                sharing_has_stopped,
                ..
            } = &mut this.client_state
            {
                *sharing_has_stopped = true;
                this.collaborators.clear();
                cx.notify();
            } else {
                unreachable!()
            }
        });

        Ok(())
    }

    async fn handle_add_collaborator(
        this: ModelHandle<Self>,
        mut envelope: TypedEnvelope<proto::AddProjectCollaborator>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let user_store = this.read_with(&cx, |this, _| this.user_store.clone());
        let collaborator = envelope
            .payload
            .collaborator
            .take()
            .ok_or_else(|| anyhow!("empty collaborator"))?;

        let collaborator = Collaborator::from_proto(collaborator, &user_store, &mut cx).await?;
        this.update(&mut cx, |this, cx| {
            this.collaborators
                .insert(collaborator.peer_id, collaborator);
            cx.notify();
        });

        Ok(())
    }

    async fn handle_remove_collaborator(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::RemoveProjectCollaborator>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let peer_id = PeerId(envelope.payload.peer_id);
            let replica_id = this
                .collaborators
                .remove(&peer_id)
                .ok_or_else(|| anyhow!("unknown peer {:?}", peer_id))?
                .replica_id;
            this.shared_buffers.remove(&peer_id);
            for (_, buffer) in &this.buffers_state.borrow().open_buffers {
                if let Some(buffer) = buffer.upgrade(cx) {
                    buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
                }
            }
            cx.notify();
            Ok(())
        })
    }

    async fn handle_register_worktree(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::RegisterWorktree>,
        client: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let remote_id = this.remote_id().ok_or_else(|| anyhow!("invalid project"))?;
            let replica_id = this.replica_id();
            let worktree = proto::Worktree {
                id: envelope.payload.worktree_id,
                root_name: envelope.payload.root_name,
                entries: Default::default(),
                diagnostic_summaries: Default::default(),
                weak: envelope.payload.weak,
            };
            let (worktree, load_task) =
                Worktree::remote(remote_id, replica_id, worktree, client, cx);
            this.add_worktree(&worktree, cx);
            load_task.detach();
            Ok(())
        })
    }

    async fn handle_unregister_worktree(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UnregisterWorktree>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            this.remove_worktree(worktree_id, cx);
            Ok(())
        })
    }

    async fn handle_update_worktree(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateWorktree>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            if let Some(worktree) = this.worktree_for_id(worktree_id, cx) {
                worktree.update(cx, |worktree, _| {
                    let worktree = worktree.as_remote_mut().unwrap();
                    worktree.update_from_remote(envelope)
                })?;
            }
            Ok(())
        })
    }

    async fn handle_update_diagnostic_summary(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateDiagnosticSummary>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            if let Some(worktree) = this.worktree_for_id(worktree_id, cx) {
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
        })
    }

    async fn handle_disk_based_diagnostics_updating(
        this: ModelHandle<Self>,
        _: TypedEnvelope<proto::DiskBasedDiagnosticsUpdating>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| this.disk_based_diagnostics_started(cx));
        Ok(())
    }

    async fn handle_disk_based_diagnostics_updated(
        this: ModelHandle<Self>,
        _: TypedEnvelope<proto::DiskBasedDiagnosticsUpdated>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| this.disk_based_diagnostics_finished(cx));
        Ok(())
    }

    async fn handle_update_buffer(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let payload = envelope.payload.clone();
            let buffer_id = payload.buffer_id;
            let ops = payload
                .operations
                .into_iter()
                .map(|op| language::proto::deserialize_operation(op))
                .collect::<Result<Vec<_>, _>>()?;
            let is_remote = this.is_remote();
            let mut buffers_state = this.buffers_state.borrow_mut();
            let buffer_request_count = buffers_state.buffer_request_count;
            match buffers_state.open_buffers.entry(buffer_id) {
                hash_map::Entry::Occupied(mut e) => match e.get_mut() {
                    OpenBuffer::Loaded(buffer) => {
                        if let Some(buffer) = buffer.upgrade(cx) {
                            buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
                        } else if is_remote && buffer_request_count > 0 {
                            e.insert(OpenBuffer::Loading(ops));
                        }
                    }
                    OpenBuffer::Loading(operations) => operations.extend_from_slice(&ops),
                },
                hash_map::Entry::Vacant(e) => {
                    if is_remote && buffer_request_count > 0 {
                        e.insert(OpenBuffer::Loading(ops));
                    }
                }
            }
            Ok(())
        })
    }

    async fn handle_update_buffer_file(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateBufferFile>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let payload = envelope.payload.clone();
            let buffer_id = payload.buffer_id;
            let file = payload.file.ok_or_else(|| anyhow!("invalid file"))?;
            let worktree = this
                .worktree_for_id(WorktreeId::from_proto(file.worktree_id), cx)
                .ok_or_else(|| anyhow!("no such worktree"))?;
            let file = File::from_proto(file, worktree.clone(), cx)?;
            let buffer = this
                .buffers_state
                .borrow_mut()
                .open_buffers
                .get_mut(&buffer_id)
                .and_then(|b| b.upgrade(cx))
                .ok_or_else(|| anyhow!("no such buffer"))?;
            buffer.update(cx, |buffer, cx| {
                buffer.file_updated(Box::new(file), cx).detach();
            });
            Ok(())
        })
    }

    async fn handle_save_buffer(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::BufferSaved> {
        let buffer_id = envelope.payload.buffer_id;
        let sender_id = envelope.original_sender_id()?;
        let requested_version = envelope.payload.version.try_into()?;

        let (project_id, buffer) = this.update(&mut cx, |this, _| {
            let project_id = this.remote_id().ok_or_else(|| anyhow!("not connected"))?;
            let buffer = this
                .shared_buffers
                .get(&sender_id)
                .and_then(|shared_buffers| shared_buffers.get(&buffer_id).cloned())
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?;
            Ok::<_, anyhow::Error>((project_id, buffer))
        })?;

        if !buffer
            .read_with(&cx, |buffer, _| buffer.version())
            .observed_all(&requested_version)
        {
            Err(anyhow!("save request depends on unreceived edits"))?;
        }

        let (saved_version, mtime) = buffer.update(&mut cx, |buffer, cx| buffer.save(cx)).await?;
        Ok(proto::BufferSaved {
            project_id,
            buffer_id,
            version: (&saved_version).into(),
            mtime: Some(mtime.into()),
        })
    }

    async fn handle_format_buffers(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::FormatBuffers>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FormatBuffersResponse> {
        let sender_id = envelope.original_sender_id()?;
        let format = this.update(&mut cx, |this, cx| {
            let shared_buffers = this
                .shared_buffers
                .get(&sender_id)
                .ok_or_else(|| anyhow!("peer has no buffers"))?;
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                buffers.insert(
                    shared_buffers
                        .get(buffer_id)
                        .cloned()
                        .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?,
                );
            }
            Ok::<_, anyhow::Error>(this.format(buffers, false, cx))
        })?;

        let project_transaction = format.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::FormatBuffersResponse {
            transaction: Some(project_transaction),
        })
    }

    async fn handle_get_completions(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::GetCompletions>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::GetCompletionsResponse> {
        let sender_id = envelope.original_sender_id()?;
        let position = envelope
            .payload
            .position
            .and_then(language::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        let version = clock::Global::from(envelope.payload.version);
        let buffer = this.read_with(&cx, |this, _| {
            this.shared_buffers
                .get(&sender_id)
                .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))
        })?;
        if !buffer
            .read_with(&cx, |buffer, _| buffer.version())
            .observed_all(&version)
        {
            Err(anyhow!("completion request depends on unreceived edits"))?;
        }
        let version = buffer.read_with(&cx, |buffer, _| buffer.version());
        let completions = this
            .update(&mut cx, |this, cx| this.completions(&buffer, position, cx))
            .await?;

        Ok(proto::GetCompletionsResponse {
            completions: completions
                .iter()
                .map(language::proto::serialize_completion)
                .collect(),
            version: (&version).into(),
        })
    }

    async fn handle_apply_additional_edits_for_completion(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::ApplyCompletionAdditionalEdits>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ApplyCompletionAdditionalEditsResponse> {
        let sender_id = envelope.original_sender_id()?;
        let apply_additional_edits = this.update(&mut cx, |this, cx| {
            let buffer = this
                .shared_buffers
                .get(&sender_id)
                .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;
            let language = buffer.read(cx).language();
            let completion = language::proto::deserialize_completion(
                envelope
                    .payload
                    .completion
                    .ok_or_else(|| anyhow!("invalid completion"))?,
                language,
            )?;
            Ok::<_, anyhow::Error>(
                this.apply_additional_edits_for_completion(buffer, completion, false, cx),
            )
        })?;

        Ok(proto::ApplyCompletionAdditionalEditsResponse {
            transaction: apply_additional_edits
                .await?
                .as_ref()
                .map(language::proto::serialize_transaction),
        })
    }

    async fn handle_get_code_actions(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::GetCodeActions>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::GetCodeActionsResponse> {
        let sender_id = envelope.original_sender_id()?;
        let start = envelope
            .payload
            .start
            .and_then(language::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid start"))?;
        let end = envelope
            .payload
            .end
            .and_then(language::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid end"))?;
        let buffer = this.update(&mut cx, |this, _| {
            this.shared_buffers
                .get(&sender_id)
                .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))
        })?;
        let version = buffer.read_with(&cx, |buffer, _| buffer.version());
        if !version.observed(start.timestamp) || !version.observed(end.timestamp) {
            Err(anyhow!("code action request references unreceived edits"))?;
        }
        let code_actions = this.update(&mut cx, |this, cx| {
            Ok::<_, anyhow::Error>(this.code_actions(&buffer, start..end, cx))
        })?;

        Ok(proto::GetCodeActionsResponse {
            actions: code_actions
                .await?
                .iter()
                .map(language::proto::serialize_code_action)
                .collect(),
            version: (&version).into(),
        })
    }

    async fn handle_apply_code_action(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeAction>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ApplyCodeActionResponse> {
        let sender_id = envelope.original_sender_id()?;
        let action = language::proto::deserialize_code_action(
            envelope
                .payload
                .action
                .ok_or_else(|| anyhow!("invalid action"))?,
        )?;
        let apply_code_action = this.update(&mut cx, |this, cx| {
            let buffer = this
                .shared_buffers
                .get(&sender_id)
                .and_then(|shared_buffers| shared_buffers.get(&envelope.payload.buffer_id).cloned())
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;
            Ok::<_, anyhow::Error>(this.apply_code_action(buffer, action, false, cx))
        })?;

        let project_transaction = apply_code_action.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::ApplyCodeActionResponse {
            transaction: Some(project_transaction),
        })
    }

    async fn handle_lsp_command<T: LspCommand>(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<T::ProtoRequest>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<<T::ProtoRequest as proto::RequestMessage>::Response>
    where
        <T::LspRequest as lsp::request::Request>::Result: Send,
    {
        let sender_id = envelope.original_sender_id()?;
        let (request, buffer_version) = this.update(&mut cx, |this, cx| {
            let buffer_id = T::buffer_id_from_proto(&envelope.payload);
            let buffer_handle = this
                .shared_buffers
                .get(&sender_id)
                .and_then(|shared_buffers| shared_buffers.get(&buffer_id).cloned())
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?;
            let buffer = buffer_handle.read(cx);
            let buffer_version = buffer.version();
            let request = T::from_proto(envelope.payload, this, buffer)?;
            Ok::<_, anyhow::Error>((this.request_lsp(buffer_handle, request, cx), buffer_version))
        })?;
        let response = request.await?;
        this.update(&mut cx, |this, cx| {
            Ok(T::response_to_proto(
                response,
                this,
                sender_id,
                &buffer_version,
                cx,
            ))
        })
    }

    async fn handle_get_project_symbols(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::GetProjectSymbols>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::GetProjectSymbolsResponse> {
        let symbols = this
            .update(&mut cx, |this, cx| {
                this.symbols(&envelope.payload.query, cx)
            })
            .await?;

        Ok(proto::GetProjectSymbolsResponse {
            symbols: symbols.iter().map(serialize_symbol).collect(),
        })
    }

    async fn handle_open_buffer_for_symbol(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::OpenBufferForSymbol>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferForSymbolResponse> {
        let peer_id = envelope.original_sender_id()?;
        let symbol = envelope
            .payload
            .symbol
            .ok_or_else(|| anyhow!("invalid symbol"))?;
        let symbol = this.read_with(&cx, |this, _| {
            let symbol = this.deserialize_symbol(symbol)?;
            let signature = this.symbol_signature(symbol.worktree_id, &symbol.path);
            if signature == symbol.signature {
                Ok(symbol)
            } else {
                Err(anyhow!("invalid symbol signature"))
            }
        })?;
        let buffer = this
            .update(&mut cx, |this, cx| this.open_buffer_for_symbol(&symbol, cx))
            .await?;

        Ok(proto::OpenBufferForSymbolResponse {
            buffer: Some(this.update(&mut cx, |this, cx| {
                this.serialize_buffer_for_peer(&buffer, peer_id, cx)
            })),
        })
    }

    fn symbol_signature(&self, worktree_id: WorktreeId, path: &Path) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(worktree_id.to_proto().to_be_bytes());
        hasher.update(path.to_string_lossy().as_bytes());
        hasher.update(self.nonce.to_be_bytes());
        hasher.finalize().as_slice().try_into().unwrap()
    }

    async fn handle_open_buffer(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::OpenBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferResponse> {
        let peer_id = envelope.original_sender_id()?;
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let open_buffer = this.update(&mut cx, |this, cx| {
            this.open_buffer(
                ProjectPath {
                    worktree_id,
                    path: PathBuf::from(envelope.payload.path).into(),
                },
                cx,
            )
        });

        let buffer = open_buffer.await?;
        this.update(&mut cx, |this, cx| {
            Ok(proto::OpenBufferResponse {
                buffer: Some(this.serialize_buffer_for_peer(&buffer, peer_id, cx)),
            })
        })
    }

    fn serialize_project_transaction_for_peer(
        &mut self,
        project_transaction: ProjectTransaction,
        peer_id: PeerId,
        cx: &AppContext,
    ) -> proto::ProjectTransaction {
        let mut serialized_transaction = proto::ProjectTransaction {
            buffers: Default::default(),
            transactions: Default::default(),
        };
        for (buffer, transaction) in project_transaction.0 {
            serialized_transaction
                .buffers
                .push(self.serialize_buffer_for_peer(&buffer, peer_id, cx));
            serialized_transaction
                .transactions
                .push(language::proto::serialize_transaction(&transaction));
        }
        serialized_transaction
    }

    fn deserialize_project_transaction(
        &mut self,
        message: proto::ProjectTransaction,
        push_to_history: bool,
        request_handle: BufferRequestHandle,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        cx.spawn(|this, mut cx| async move {
            let mut project_transaction = ProjectTransaction::default();
            for (buffer, transaction) in message.buffers.into_iter().zip(message.transactions) {
                let buffer = this
                    .update(&mut cx, |this, cx| {
                        this.deserialize_buffer(buffer, request_handle.clone(), cx)
                    })
                    .await?;
                let transaction = language::proto::deserialize_transaction(transaction)?;
                project_transaction.0.insert(buffer, transaction);
            }

            for (buffer, transaction) in &project_transaction.0 {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_edits(transaction.edit_ids.iter().copied())
                    })
                    .await;

                if push_to_history {
                    buffer.update(&mut cx, |buffer, _| {
                        buffer.push_transaction(transaction.clone(), Instant::now());
                    });
                }
            }

            Ok(project_transaction)
        })
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

    fn deserialize_buffer(
        &mut self,
        buffer: proto::Buffer,
        request_handle: BufferRequestHandle,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let replica_id = self.replica_id();

        let mut opened_buffer_tx = self.opened_buffer.clone();
        let mut opened_buffer_rx = self.opened_buffer.subscribe();
        cx.spawn(|this, mut cx| async move {
            match buffer.variant.ok_or_else(|| anyhow!("missing buffer"))? {
                proto::buffer::Variant::Id(id) => {
                    let buffer = loop {
                        let buffer = this.read_with(&cx, |this, cx| {
                            this.buffers_state
                                .borrow()
                                .open_buffers
                                .get(&id)
                                .and_then(|buffer| buffer.upgrade(cx))
                        });
                        if let Some(buffer) = buffer {
                            break buffer;
                        }
                        opened_buffer_rx
                            .recv()
                            .await
                            .ok_or_else(|| anyhow!("project dropped while waiting for buffer"))?;
                    };
                    Ok(buffer)
                }
                proto::buffer::Variant::State(mut buffer) => {
                    let mut buffer_worktree = None;
                    let mut buffer_file = None;
                    if let Some(file) = buffer.file.take() {
                        this.read_with(&cx, |this, cx| {
                            let worktree_id = WorktreeId::from_proto(file.worktree_id);
                            let worktree =
                                this.worktree_for_id(worktree_id, cx).ok_or_else(|| {
                                    anyhow!("no worktree found for id {}", file.worktree_id)
                                })?;
                            buffer_file =
                                Some(Box::new(File::from_proto(file, worktree.clone(), cx)?)
                                    as Box<dyn language::File>);
                            buffer_worktree = Some(worktree);
                            Ok::<_, anyhow::Error>(())
                        })?;
                    }

                    let buffer = cx.add_model(|cx| {
                        Buffer::from_proto(replica_id, buffer, buffer_file, cx).unwrap()
                    });

                    request_handle.preserve_buffer(buffer.clone());
                    this.update(&mut cx, |this, cx| {
                        this.register_buffer(&buffer, buffer_worktree.as_ref(), cx)
                    })?;

                    let _ = opened_buffer_tx.send(()).await;
                    Ok(buffer)
                }
            }
        })
    }

    fn deserialize_symbol(&self, serialized_symbol: proto::Symbol) -> Result<Symbol> {
        let language = self
            .languages
            .get_language(&serialized_symbol.language_name);
        let start = serialized_symbol
            .start
            .ok_or_else(|| anyhow!("invalid start"))?;
        let end = serialized_symbol
            .end
            .ok_or_else(|| anyhow!("invalid end"))?;
        let kind = unsafe { mem::transmute(serialized_symbol.kind) };
        Ok(Symbol {
            source_worktree_id: WorktreeId::from_proto(serialized_symbol.source_worktree_id),
            worktree_id: WorktreeId::from_proto(serialized_symbol.worktree_id),
            language_name: serialized_symbol.language_name.clone(),
            label: language
                .and_then(|language| language.label_for_symbol(&serialized_symbol.name, kind))
                .unwrap_or_else(|| CodeLabel::plain(serialized_symbol.name.clone(), None)),
            name: serialized_symbol.name,
            path: PathBuf::from(serialized_symbol.path),
            range: PointUtf16::new(start.row, start.column)..PointUtf16::new(end.row, end.column),
            kind,
            signature: serialized_symbol
                .signature
                .try_into()
                .map_err(|_| anyhow!("invalid signature"))?,
        })
    }

    async fn handle_close_buffer(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            if let Some(shared_buffers) =
                this.shared_buffers.get_mut(&envelope.original_sender_id()?)
            {
                shared_buffers.remove(&envelope.payload.buffer_id);
                cx.notify();
            }
            Ok(())
        })
    }

    async fn handle_buffer_saved(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::BufferSaved>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let version = envelope.payload.version.try_into()?;
        let mtime = envelope
            .payload
            .mtime
            .ok_or_else(|| anyhow!("missing mtime"))?
            .into();

        this.update(&mut cx, |this, cx| {
            let buffer = this
                .buffers_state
                .borrow()
                .open_buffers
                .get(&envelope.payload.buffer_id)
                .and_then(|buffer| buffer.upgrade(cx));
            if let Some(buffer) = buffer {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_save(version, mtime, None, cx);
                });
            }
            Ok(())
        })
    }

    async fn handle_buffer_reloaded(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::BufferReloaded>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let payload = envelope.payload.clone();
        let version = payload.version.try_into()?;
        let mtime = payload
            .mtime
            .ok_or_else(|| anyhow!("missing mtime"))?
            .into();
        this.update(&mut cx, |this, cx| {
            let buffer = this
                .buffers_state
                .borrow()
                .open_buffers
                .get(&payload.buffer_id)
                .and_then(|buffer| buffer.upgrade(cx));
            if let Some(buffer) = buffer {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_reload(version, mtime, cx);
                });
            }
            Ok(())
        })
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

impl BufferRequestHandle {
    fn new(state: Rc<RefCell<ProjectBuffers>>, cx: &AppContext) -> Self {
        {
            let state = &mut *state.borrow_mut();
            state.buffer_request_count += 1;
            if state.buffer_request_count == 1 {
                state.preserved_buffers.extend(
                    state
                        .open_buffers
                        .values()
                        .filter_map(|buffer| buffer.upgrade(cx)),
                )
            }
        }
        Self(state)
    }

    fn preserve_buffer(&self, buffer: ModelHandle<Buffer>) {
        self.0.borrow_mut().preserved_buffers.push(buffer);
    }
}

impl Clone for BufferRequestHandle {
    fn clone(&self) -> Self {
        self.0.borrow_mut().buffer_request_count += 1;
        Self(self.0.clone())
    }
}

impl Drop for BufferRequestHandle {
    fn drop(&mut self) {
        let mut state = self.0.borrow_mut();
        state.buffer_request_count -= 1;
        if state.buffer_request_count == 0 {
            state.preserved_buffers.clear();
            state
                .open_buffers
                .retain(|_, buffer| matches!(buffer, OpenBuffer::Loaded(_)))
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

impl OpenBuffer {
    pub fn upgrade(&self, cx: &impl UpgradeModelHandle) -> Option<ModelHandle<Buffer>> {
        match self {
            OpenBuffer::Loaded(handle) => handle.upgrade(cx),
            OpenBuffer::Loading(_) => None,
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

impl From<lsp::CreateFileOptions> for fs::CreateOptions {
    fn from(options: lsp::CreateFileOptions) -> Self {
        Self {
            overwrite: options.overwrite.unwrap_or(false),
            ignore_if_exists: options.ignore_if_exists.unwrap_or(false),
        }
    }
}

impl From<lsp::RenameFileOptions> for fs::RenameOptions {
    fn from(options: lsp::RenameFileOptions) -> Self {
        Self {
            overwrite: options.overwrite.unwrap_or(false),
            ignore_if_exists: options.ignore_if_exists.unwrap_or(false),
        }
    }
}

impl From<lsp::DeleteFileOptions> for fs::RemoveOptions {
    fn from(options: lsp::DeleteFileOptions) -> Self {
        Self {
            recursive: options.recursive.unwrap_or(false),
            ignore_if_not_exists: options.ignore_if_not_exists.unwrap_or(false),
        }
    }
}

fn serialize_symbol(symbol: &Symbol) -> proto::Symbol {
    proto::Symbol {
        source_worktree_id: symbol.source_worktree_id.to_proto(),
        worktree_id: symbol.worktree_id.to_proto(),
        language_name: symbol.language_name.clone(),
        name: symbol.name.clone(),
        kind: unsafe { mem::transmute(symbol.kind) },
        path: symbol.path.to_string_lossy().to_string(),
        start: Some(proto::Point {
            row: symbol.range.start.row,
            column: symbol.range.start.column,
        }),
        end: Some(proto::Point {
            row: symbol.range.end.row,
            column: symbol.range.end.column,
        }),
        signature: symbol.signature.to_vec(),
    }
}

fn relativize_path(base: &Path, path: &Path) -> PathBuf {
    let mut path_components = path.components();
    let mut base_components = base.components();
    let mut components: Vec<Component> = Vec::new();
    loop {
        match (path_components.next(), base_components.next()) {
            (None, None) => break,
            (Some(a), None) => {
                components.push(a);
                components.extend(path_components.by_ref());
                break;
            }
            (None, _) => components.push(Component::ParentDir),
            (Some(a), Some(b)) if components.is_empty() && a == b => (),
            (Some(a), Some(b)) if b == Component::CurDir => components.push(a),
            (Some(a), Some(_)) => {
                components.push(Component::ParentDir);
                for _ in base_components {
                    components.push(Component::ParentDir);
                }
                components.push(a);
                components.extend(path_components.by_ref());
                break;
            }
        }
    }
    components.iter().map(|c| c.as_os_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::{Event, *};
    use fs::RealFs;
    use futures::StreamExt;
    use gpui::test::subscribe;
    use language::{
        tree_sitter_rust, AnchorRangeExt, Diagnostic, LanguageConfig, LanguageServerConfig, Point,
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

        let project = Project::test(Arc::new(RealFs), &mut cx);

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
        let (language_server_config, mut fake_servers) = LanguageServerConfig::fake();
        let progress_token = language_server_config
            .disk_based_diagnostics_progress_token
            .clone()
            .unwrap();

        let language = Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                language_server: Some(language_server_config),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ));

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.rs": "fn a() { A }",
                "b.rs": "const y: i32 = 1",
            }),
        )
        .await;

        let project = Project::test(fs, &mut cx);
        project.update(&mut cx, |project, _| {
            Arc::get_mut(&mut project.languages).unwrap().add(language);
        });

        let (tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree("/dir", false, cx)
            })
            .await
            .unwrap();
        let worktree_id = tree.read_with(&cx, |tree, _| tree.id());

        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        // Cause worktree to start the fake language server
        let _buffer = project
            .update(&mut cx, |project, cx| {
                project.open_buffer((worktree_id, Path::new("b.rs")), cx)
            })
            .await
            .unwrap();

        let mut events = subscribe(&project, &mut cx);

        let mut fake_server = fake_servers.next().await.unwrap();
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
                uri: Url::from_file_path("/dir/a.rs").unwrap(),
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
            Event::DiagnosticsUpdated((worktree_id, Path::new("a.rs")).into())
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

        let project = Project::test(Arc::new(RealFs), &mut cx);
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
        let (language_server_config, mut fake_servers) = LanguageServerConfig::fake();
        let language = Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                language_server: Some(language_server_config),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ));

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.rs": "const fn a() { A }",
                "b.rs": "const y: i32 = crate::a()",
            }),
        )
        .await;

        let project = Project::test(fs, &mut cx);
        project.update(&mut cx, |project, _| {
            Arc::get_mut(&mut project.languages).unwrap().add(language);
        });

        let (tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree("/dir/b.rs", false, cx)
            })
            .await
            .unwrap();
        let worktree_id = tree.read_with(&cx, |tree, _| tree.id());
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

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

        let mut fake_server = fake_servers.next().await.unwrap();
        fake_server.handle_request::<lsp::request::GotoDefinition, _>(move |params, _| {
            let params = params.text_document_position_params;
            assert_eq!(
                params.text_document.uri.to_file_path().unwrap(),
                Path::new("/dir/b.rs"),
            );
            assert_eq!(params.position, lsp::Position::new(0, 22));

            Some(lsp::GotoDefinitionResponse::Scalar(lsp::Location::new(
                lsp::Url::from_file_path("/dir/a.rs").unwrap(),
                lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
            )))
        });

        let mut definitions = project
            .update(&mut cx, |project, cx| project.definition(&buffer, 22, cx))
            .await
            .unwrap();

        assert_eq!(definitions.len(), 1);
        let definition = definitions.pop().unwrap();
        cx.update(|cx| {
            let target_buffer = definition.buffer.read(cx);
            assert_eq!(
                target_buffer
                    .file()
                    .unwrap()
                    .as_local()
                    .unwrap()
                    .abs_path(cx),
                Path::new("/dir/a.rs"),
            );
            assert_eq!(definition.range.to_offset(target_buffer), 9..10);
            assert_eq!(
                list_worktrees(&project, cx),
                [("/dir/b.rs".as_ref(), false), ("/dir/a.rs".as_ref(), true)]
            );

            drop(definition);
        });
        cx.read(|cx| {
            assert_eq!(
                list_worktrees(&project, cx),
                [("/dir/b.rs".as_ref(), false)]
            );
        });

        fn list_worktrees<'a>(
            project: &'a ModelHandle<Project>,
            cx: &'a AppContext,
        ) -> Vec<(&'a Path, bool)> {
            project
                .read(cx)
                .worktrees(cx)
                .map(|worktree| {
                    let worktree = worktree.read(cx);
                    (
                        worktree.as_local().unwrap().abs_path().as_ref(),
                        worktree.is_weak(),
                    )
                })
                .collect::<Vec<_>>()
        }
    }

    #[gpui::test]
    async fn test_save_file(mut cx: gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "file1": "the old contents",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), &mut cx);
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
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "file1": "the old contents",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), &mut cx);
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

        let project = Project::test(Arc::new(RealFs), &mut cx);
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
        let initial_snapshot = tree.read_with(&cx, |tree, _| tree.as_local().unwrap().snapshot());
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
            let update_message = tree.read(cx).as_local().unwrap().snapshot().build_update(
                &initial_snapshot,
                1,
                1,
                true,
            );
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
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/the-dir",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), &mut cx);
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

        let project = Project::test(Arc::new(RealFs), &mut cx);
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

        let project = Project::test(Arc::new(RealFs), &mut cx);
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
        let fs = FakeFs::new(cx.background());
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

        let project = Project::test(fs.clone(), &mut cx);
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

    #[gpui::test]
    async fn test_rename(mut cx: gpui::TestAppContext) {
        let (language_server_config, mut fake_servers) = LanguageServerConfig::fake();
        let language = Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                language_server: Some(language_server_config),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ));

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), &mut cx);
        project.update(&mut cx, |project, _| {
            Arc::get_mut(&mut project.languages).unwrap().add(language);
        });

        let (tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree("/dir", false, cx)
            })
            .await
            .unwrap();
        let worktree_id = tree.read_with(&cx, |tree, _| tree.id());
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        let buffer = project
            .update(&mut cx, |project, cx| {
                project.open_buffer((worktree_id, Path::new("one.rs")), cx)
            })
            .await
            .unwrap();

        let mut fake_server = fake_servers.next().await.unwrap();

        let response = project.update(&mut cx, |project, cx| {
            project.prepare_rename(buffer.clone(), 7, cx)
        });
        fake_server
            .handle_request::<lsp::request::PrepareRenameRequest, _>(|params, _| {
                assert_eq!(params.text_document.uri.as_str(), "file:///dir/one.rs");
                assert_eq!(params.position, lsp::Position::new(0, 7));
                Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                    lsp::Position::new(0, 6),
                    lsp::Position::new(0, 9),
                )))
            })
            .next()
            .await
            .unwrap();
        let range = response.await.unwrap().unwrap();
        let range = buffer.read_with(&cx, |buffer, _| range.to_offset(buffer));
        assert_eq!(range, 6..9);

        let response = project.update(&mut cx, |project, cx| {
            project.perform_rename(buffer.clone(), 7, "THREE".to_string(), true, cx)
        });
        fake_server
            .handle_request::<lsp::request::Rename, _>(|params, _| {
                assert_eq!(
                    params.text_document_position.text_document.uri.as_str(),
                    "file:///dir/one.rs"
                );
                assert_eq!(
                    params.text_document_position.position,
                    lsp::Position::new(0, 7)
                );
                assert_eq!(params.new_name, "THREE");
                Some(lsp::WorkspaceEdit {
                    changes: Some(
                        [
                            (
                                lsp::Url::from_file_path("/dir/one.rs").unwrap(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 6),
                                        lsp::Position::new(0, 9),
                                    ),
                                    "THREE".to_string(),
                                )],
                            ),
                            (
                                lsp::Url::from_file_path("/dir/two.rs").unwrap(),
                                vec![
                                    lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(0, 24),
                                            lsp::Position::new(0, 27),
                                        ),
                                        "THREE".to_string(),
                                    ),
                                    lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(0, 35),
                                            lsp::Position::new(0, 38),
                                        ),
                                        "THREE".to_string(),
                                    ),
                                ],
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    ..Default::default()
                })
            })
            .next()
            .await
            .unwrap();
        let mut transaction = response.await.unwrap().0;
        assert_eq!(transaction.len(), 2);
        assert_eq!(
            transaction
                .remove_entry(&buffer)
                .unwrap()
                .0
                .read_with(&cx, |buffer, _| buffer.text()),
            "const THREE: usize = 1;"
        );
        assert_eq!(
            transaction
                .into_keys()
                .next()
                .unwrap()
                .read_with(&cx, |buffer, _| buffer.text()),
            "const TWO: usize = one::THREE + one::THREE;"
        );
    }

    #[gpui::test]
    async fn test_search(mut cx: gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), &mut cx);
        let (tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_local_worktree("/dir", false, cx)
            })
            .await
            .unwrap();
        let worktree_id = tree.read_with(&cx, |tree, _| tree.id());
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        assert_eq!(
            search(&project, SearchQuery::Plain("TWO".to_string()), &mut cx).await,
            HashMap::from_iter([
                ("two.rs".to_string(), vec![6..9]),
                ("three.rs".to_string(), vec![37..40])
            ])
        );

        let buffer_4 = project
            .update(&mut cx, |project, cx| {
                project.open_buffer((worktree_id, "four.rs"), cx)
            })
            .await
            .unwrap();
        buffer_4.update(&mut cx, |buffer, cx| {
            buffer.edit([20..28, 31..43], "two::TWO", cx);
        });

        assert_eq!(
            search(&project, SearchQuery::Plain("TWO".to_string()), &mut cx).await,
            HashMap::from_iter([
                ("two.rs".to_string(), vec![6..9]),
                ("three.rs".to_string(), vec![37..40]),
                ("four.rs".to_string(), vec![25..28, 36..39])
            ])
        );

        async fn search(
            project: &ModelHandle<Project>,
            query: SearchQuery,
            cx: &mut gpui::TestAppContext,
        ) -> HashMap<String, Vec<Range<usize>>> {
            project
                .update(cx, |project, cx| project.search(query, cx))
                .await
                .into_iter()
                .map(|(buffer, ranges)| {
                    buffer.read_with(cx, |buffer, _| {
                        let path = buffer.file().unwrap().path().to_string_lossy().to_string();
                        let ranges = ranges
                            .into_iter()
                            .map(|range| range.to_offset(buffer))
                            .collect::<Vec<_>>();
                        (path, ranges)
                    })
                })
                .collect()
        }
    }
}
