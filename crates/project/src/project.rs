pub mod fs;
mod ignore;
mod lsp_command;
pub mod search;
pub mod worktree;

use anyhow::{anyhow, Context, Result};
use client::{proto, Client, PeerId, TypedEnvelope, User, UserStore};
use clock::ReplicaId;
use collections::{hash_map, BTreeMap, HashMap, HashSet};
use futures::{future::Shared, Future, FutureExt, StreamExt, TryFutureExt};
use fuzzy::{PathMatch, PathMatchCandidate, PathMatchCandidateSet};
use gpui::{
    AnyModelHandle, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle,
    MutableAppContext, Task, UpgradeModelHandle, WeakModelHandle,
};
use language::{
    point_to_lsp,
    proto::{deserialize_anchor, deserialize_version, serialize_anchor, serialize_version},
    range_from_lsp, range_to_lsp, Anchor, Bias, Buffer, CodeAction, CodeLabel, Completion,
    Diagnostic, DiagnosticEntry, DiagnosticSet, Event as BufferEvent, File as _, Language,
    LanguageRegistry, LanguageServerName, LocalFile, LspAdapter, OffsetRangeExt, Operation, Patch,
    PointUtf16, TextBufferSnapshot, ToOffset, ToPointUtf16, Transaction,
};
use lsp::{DiagnosticSeverity, DiagnosticTag, DocumentHighlightKind, LanguageServer};
use lsp_command::*;
use parking_lot::Mutex;
use postage::stream::Stream;
use postage::watch;
use rand::prelude::*;
use search::SearchQuery;
use serde::Serialize;
use settings::Settings;
use sha2::{Digest, Sha256};
use similar::{ChangeTag, TextDiff};
use std::{
    cell::RefCell,
    cmp::{self, Ordering},
    convert::TryInto,
    ffi::OsString,
    hash::Hash,
    mem,
    ops::Range,
    os::unix::{ffi::OsStrExt, prelude::OsStringExt},
    path::{Component, Path, PathBuf},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::Instant,
};
use thiserror::Error;
use util::{post_inc, ResultExt, TryFutureExt as _};

pub use fs::*;
pub use worktree::*;

pub trait Item: Entity {
    fn entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId>;
}

#[derive(Default)]
pub struct ProjectStore {
    projects: Vec<WeakModelHandle<Project>>,
}

pub struct Project {
    worktrees: Vec<WorktreeHandle>,
    active_entry: Option<ProjectEntryId>,
    languages: Arc<LanguageRegistry>,
    language_servers:
        HashMap<(WorktreeId, LanguageServerName), (Arc<dyn LspAdapter>, Arc<LanguageServer>)>,
    started_language_servers:
        HashMap<(WorktreeId, LanguageServerName), Task<Option<Arc<LanguageServer>>>>,
    language_server_statuses: BTreeMap<usize, LanguageServerStatus>,
    language_server_settings: Arc<Mutex<serde_json::Value>>,
    last_workspace_edits_by_language_server: HashMap<usize, ProjectTransaction>,
    next_language_server_id: usize,
    client: Arc<client::Client>,
    next_entry_id: Arc<AtomicUsize>,
    next_diagnostic_group_id: usize,
    user_store: ModelHandle<UserStore>,
    project_store: ModelHandle<ProjectStore>,
    fs: Arc<dyn Fs>,
    client_state: ProjectClientState,
    collaborators: HashMap<PeerId, Collaborator>,
    subscriptions: Vec<client::Subscription>,
    opened_buffer: (Rc<RefCell<watch::Sender<()>>>, watch::Receiver<()>),
    shared_buffers: HashMap<PeerId, HashSet<u64>>,
    loading_buffers: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<ModelHandle<Buffer>, Arc<anyhow::Error>>>>,
    >,
    loading_local_worktrees:
        HashMap<Arc<Path>, Shared<Task<Result<ModelHandle<Worktree>, Arc<anyhow::Error>>>>>,
    opened_buffers: HashMap<u64, OpenBuffer>,
    buffer_snapshots: HashMap<u64, Vec<(i32, TextBufferSnapshot)>>,
    nonce: u128,
}

#[derive(Error, Debug)]
pub enum JoinProjectError {
    #[error("host declined join request")]
    HostDeclined,
    #[error("host closed the project")]
    HostClosedProject,
    #[error("host went offline")]
    HostWentOffline,
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

enum OpenBuffer {
    Strong(ModelHandle<Buffer>),
    Weak(WeakModelHandle<Buffer>),
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
        public_tx: watch::Sender<bool>,
        public_rx: watch::Receiver<bool>,
        _maintain_remote_id_task: Task<Option<()>>,
    },
    Remote {
        sharing_has_stopped: bool,
        remote_id: u64,
        replica_id: ReplicaId,
        _detect_unshare_task: Task<Option<()>>,
    },
}

#[derive(Clone, Debug)]
pub struct Collaborator {
    pub user: Arc<User>,
    pub peer_id: PeerId,
    pub replica_id: ReplicaId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    ActiveEntryChanged(Option<ProjectEntryId>),
    WorktreeAdded,
    WorktreeRemoved(WorktreeId),
    DiskBasedDiagnosticsStarted,
    DiskBasedDiagnosticsUpdated,
    DiskBasedDiagnosticsFinished,
    DiagnosticsUpdated(ProjectPath),
    RemoteIdChanged(Option<u64>),
    CollaboratorLeft(PeerId),
    ContactRequestedJoin(Arc<User>),
    ContactCancelledJoinRequest(Arc<User>),
}

#[derive(Serialize)]
pub struct LanguageServerStatus {
    pub name: String,
    pub pending_work: BTreeMap<String, LanguageServerProgress>,
    pub pending_diagnostic_updates: isize,
}

#[derive(Clone, Debug, Serialize)]
pub struct LanguageServerProgress {
    pub message: Option<String>,
    pub percentage: Option<usize>,
    #[serde(skip_serializing)]
    pub last_update_at: Instant,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ProjectPath {
    pub worktree_id: WorktreeId,
    pub path: Arc<Path>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize)]
pub struct DiagnosticSummary {
    pub error_count: usize,
    pub warning_count: usize,
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
    pub language_server_name: LanguageServerName,
    pub path: PathBuf,
    pub label: CodeLabel,
    pub name: String,
    pub kind: lsp::SymbolKind,
    pub range: Range<PointUtf16>,
    pub signature: [u8; 32],
}

#[derive(Default)]
pub struct ProjectTransaction(pub HashMap<ModelHandle<Buffer>, language::Transaction>);

impl DiagnosticSummary {
    fn new<'a, T: 'a>(diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<T>>) -> Self {
        let mut this = Self {
            error_count: 0,
            warning_count: 0,
        };

        for entry in diagnostics {
            if entry.diagnostic.is_primary {
                match entry.diagnostic.severity {
                    DiagnosticSeverity::ERROR => this.error_count += 1,
                    DiagnosticSeverity::WARNING => this.warning_count += 1,
                    _ => {}
                }
            }
        }

        this
    }

    pub fn is_empty(&self) -> bool {
        self.error_count == 0 && self.warning_count == 0
    }

    pub fn to_proto(&self, path: &Path) -> proto::DiagnosticSummary {
        proto::DiagnosticSummary {
            path: path.to_string_lossy().to_string(),
            error_count: self.error_count as u32,
            warning_count: self.warning_count as u32,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectEntryId(usize);

impl ProjectEntryId {
    pub const MAX: Self = Self(usize::MAX);

    pub fn new(counter: &AtomicUsize) -> Self {
        Self(counter.fetch_add(1, SeqCst))
    }

    pub fn from_proto(id: u64) -> Self {
        Self(id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl Project {
    pub fn init(client: &Arc<Client>) {
        client.add_model_message_handler(Self::handle_request_join_project);
        client.add_model_message_handler(Self::handle_add_collaborator);
        client.add_model_message_handler(Self::handle_buffer_reloaded);
        client.add_model_message_handler(Self::handle_buffer_saved);
        client.add_model_message_handler(Self::handle_start_language_server);
        client.add_model_message_handler(Self::handle_update_language_server);
        client.add_model_message_handler(Self::handle_remove_collaborator);
        client.add_model_message_handler(Self::handle_join_project_request_cancelled);
        client.add_model_message_handler(Self::handle_update_project);
        client.add_model_message_handler(Self::handle_unregister_project);
        client.add_model_message_handler(Self::handle_project_unshared);
        client.add_model_message_handler(Self::handle_update_buffer_file);
        client.add_model_message_handler(Self::handle_update_buffer);
        client.add_model_message_handler(Self::handle_update_diagnostic_summary);
        client.add_model_message_handler(Self::handle_update_worktree);
        client.add_model_request_handler(Self::handle_create_project_entry);
        client.add_model_request_handler(Self::handle_rename_project_entry);
        client.add_model_request_handler(Self::handle_copy_project_entry);
        client.add_model_request_handler(Self::handle_delete_project_entry);
        client.add_model_request_handler(Self::handle_apply_additional_edits_for_completion);
        client.add_model_request_handler(Self::handle_apply_code_action);
        client.add_model_request_handler(Self::handle_reload_buffers);
        client.add_model_request_handler(Self::handle_format_buffers);
        client.add_model_request_handler(Self::handle_get_code_actions);
        client.add_model_request_handler(Self::handle_get_completions);
        client.add_model_request_handler(Self::handle_lsp_command::<GetDefinition>);
        client.add_model_request_handler(Self::handle_lsp_command::<GetDocumentHighlights>);
        client.add_model_request_handler(Self::handle_lsp_command::<GetReferences>);
        client.add_model_request_handler(Self::handle_lsp_command::<PrepareRename>);
        client.add_model_request_handler(Self::handle_lsp_command::<PerformRename>);
        client.add_model_request_handler(Self::handle_search_project);
        client.add_model_request_handler(Self::handle_get_project_symbols);
        client.add_model_request_handler(Self::handle_open_buffer_for_symbol);
        client.add_model_request_handler(Self::handle_open_buffer_by_id);
        client.add_model_request_handler(Self::handle_open_buffer_by_path);
        client.add_model_request_handler(Self::handle_save_buffer);
    }

    pub fn local(
        public: bool,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        project_store: ModelHandle<ProjectStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut MutableAppContext,
    ) -> ModelHandle<Self> {
        cx.add_model(|cx: &mut ModelContext<Self>| {
            let (public_tx, public_rx) = watch::channel_with(public);
            let (remote_id_tx, remote_id_rx) = watch::channel();
            let _maintain_remote_id_task = cx.spawn_weak({
                let status_rx = client.clone().status();
                let public_rx = public_rx.clone();
                move |this, mut cx| async move {
                    let mut stream = Stream::map(status_rx.clone(), drop)
                        .merge(Stream::map(public_rx.clone(), drop));
                    while stream.recv().await.is_some() {
                        let this = this.upgrade(&cx)?;
                        if status_rx.borrow().is_connected() && *public_rx.borrow() {
                            this.update(&mut cx, |this, cx| this.register(cx))
                                .await
                                .log_err()?;
                        } else {
                            this.update(&mut cx, |this, cx| this.unregister(cx))
                                .await
                                .log_err();
                        }
                    }
                    None
                }
            });

            let handle = cx.weak_handle();
            project_store.update(cx, |store, cx| store.add_project(handle, cx));

            let (opened_buffer_tx, opened_buffer_rx) = watch::channel();
            Self {
                worktrees: Default::default(),
                collaborators: Default::default(),
                opened_buffers: Default::default(),
                shared_buffers: Default::default(),
                loading_buffers: Default::default(),
                loading_local_worktrees: Default::default(),
                buffer_snapshots: Default::default(),
                client_state: ProjectClientState::Local {
                    is_shared: false,
                    remote_id_tx,
                    remote_id_rx,
                    public_tx,
                    public_rx,
                    _maintain_remote_id_task,
                },
                opened_buffer: (Rc::new(RefCell::new(opened_buffer_tx)), opened_buffer_rx),
                subscriptions: Vec::new(),
                active_entry: None,
                languages,
                client,
                user_store,
                project_store,
                fs,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                language_servers: Default::default(),
                started_language_servers: Default::default(),
                language_server_statuses: Default::default(),
                last_workspace_edits_by_language_server: Default::default(),
                language_server_settings: Default::default(),
                next_language_server_id: 0,
                nonce: StdRng::from_entropy().gen(),
            }
        })
    }

    pub async fn remote(
        remote_id: u64,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        project_store: ModelHandle<ProjectStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        mut cx: AsyncAppContext,
    ) -> Result<ModelHandle<Self>, JoinProjectError> {
        client.authenticate_and_connect(true, &cx).await?;

        let response = client
            .request(proto::JoinProject {
                project_id: remote_id,
            })
            .await?;

        let response = match response.variant.ok_or_else(|| anyhow!("missing variant"))? {
            proto::join_project_response::Variant::Accept(response) => response,
            proto::join_project_response::Variant::Decline(decline) => {
                match proto::join_project_response::decline::Reason::from_i32(decline.reason) {
                    Some(proto::join_project_response::decline::Reason::Declined) => {
                        Err(JoinProjectError::HostDeclined)?
                    }
                    Some(proto::join_project_response::decline::Reason::Closed) => {
                        Err(JoinProjectError::HostClosedProject)?
                    }
                    Some(proto::join_project_response::decline::Reason::WentOffline) => {
                        Err(JoinProjectError::HostWentOffline)?
                    }
                    None => Err(anyhow!("missing decline reason"))?,
                }
            }
        };

        let replica_id = response.replica_id as ReplicaId;

        let mut worktrees = Vec::new();
        for worktree in response.worktrees {
            let (worktree, load_task) = cx
                .update(|cx| Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx));
            worktrees.push(worktree);
            load_task.detach();
        }

        let (opened_buffer_tx, opened_buffer_rx) = watch::channel();
        let this = cx.add_model(|cx: &mut ModelContext<Self>| {
            let handle = cx.weak_handle();
            project_store.update(cx, |store, cx| store.add_project(handle, cx));

            let mut this = Self {
                worktrees: Vec::new(),
                loading_buffers: Default::default(),
                opened_buffer: (Rc::new(RefCell::new(opened_buffer_tx)), opened_buffer_rx),
                shared_buffers: Default::default(),
                loading_local_worktrees: Default::default(),
                active_entry: None,
                collaborators: Default::default(),
                languages,
                user_store: user_store.clone(),
                project_store,
                fs,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                subscriptions: vec![client.add_model_for_remote_entity(remote_id, cx)],
                client: client.clone(),
                client_state: ProjectClientState::Remote {
                    sharing_has_stopped: false,
                    remote_id,
                    replica_id,
                    _detect_unshare_task: cx.spawn_weak(move |this, mut cx| {
                        async move {
                            let mut status = client.status();
                            let is_connected =
                                status.next().await.map_or(false, |s| s.is_connected());
                            // Even if we're initially connected, any future change of the status means we momentarily disconnected.
                            if !is_connected || status.next().await.is_some() {
                                if let Some(this) = this.upgrade(&cx) {
                                    this.update(&mut cx, |this, cx| this.removed_from_project(cx))
                                }
                            }
                            Ok(())
                        }
                        .log_err()
                    }),
                },
                language_servers: Default::default(),
                started_language_servers: Default::default(),
                language_server_settings: Default::default(),
                language_server_statuses: response
                    .language_servers
                    .into_iter()
                    .map(|server| {
                        (
                            server.id as usize,
                            LanguageServerStatus {
                                name: server.name,
                                pending_work: Default::default(),
                                pending_diagnostic_updates: 0,
                            },
                        )
                    })
                    .collect(),
                last_workspace_edits_by_language_server: Default::default(),
                next_language_server_id: 0,
                opened_buffers: Default::default(),
                buffer_snapshots: Default::default(),
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
            .update(&mut cx, |user_store, cx| user_store.get_users(user_ids, cx))
            .await?;
        let mut collaborators = HashMap::default();
        for message in response.collaborators {
            let collaborator = Collaborator::from_proto(message, &user_store, &mut cx).await?;
            collaborators.insert(collaborator.peer_id, collaborator);
        }

        this.update(&mut cx, |this, _| {
            this.collaborators = collaborators;
        });

        Ok(this)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn test(
        fs: Arc<dyn Fs>,
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut gpui::TestAppContext,
    ) -> ModelHandle<Project> {
        let languages = Arc::new(LanguageRegistry::test());
        let http_client = client::test::FakeHttpClient::with_404_response();
        let client = client::Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let project_store = cx.add_model(|_| ProjectStore::default());
        let project = cx.update(|cx| {
            Project::local(true, client, user_store, project_store, languages, fs, cx)
        });
        for path in root_paths {
            let (tree, _) = project
                .update(cx, |project, cx| {
                    project.find_or_create_local_worktree(path, true, cx)
                })
                .await
                .unwrap();
            tree.read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .await;
        }
        project
    }

    pub fn buffer_for_id(&self, remote_id: u64, cx: &AppContext) -> Option<ModelHandle<Buffer>> {
        self.opened_buffers
            .get(&remote_id)
            .and_then(|buffer| buffer.upgrade(cx))
    }

    pub fn languages(&self) -> &Arc<LanguageRegistry> {
        &self.languages
    }

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn user_store(&self) -> ModelHandle<UserStore> {
        self.user_store.clone()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn check_invariants(&self, cx: &AppContext) {
        if self.is_local() {
            let mut worktree_root_paths = HashMap::default();
            for worktree in self.worktrees(cx) {
                let worktree = worktree.read(cx);
                let abs_path = worktree.as_local().unwrap().abs_path().clone();
                let prev_worktree_id = worktree_root_paths.insert(abs_path.clone(), worktree.id());
                assert_eq!(
                    prev_worktree_id,
                    None,
                    "abs path {:?} for worktree {:?} is not unique ({:?} was already registered with the same path)",
                    abs_path,
                    worktree.id(),
                    prev_worktree_id
                )
            }
        } else {
            let replica_id = self.replica_id();
            for buffer in self.opened_buffers.values() {
                if let Some(buffer) = buffer.upgrade(cx) {
                    let buffer = buffer.read(cx);
                    assert_eq!(
                        buffer.deferred_ops_len(),
                        0,
                        "replica {}, buffer {} has deferred operations",
                        replica_id,
                        buffer.remote_id()
                    );
                }
            }
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn has_open_buffer(&self, path: impl Into<ProjectPath>, cx: &AppContext) -> bool {
        let path = path.into();
        if let Some(worktree) = self.worktree_for_id(path.worktree_id, cx) {
            self.opened_buffers.iter().any(|(_, buffer)| {
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

    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    pub fn set_public(&mut self, is_public: bool, cx: &mut ModelContext<Self>) {
        if let ProjectClientState::Local { public_tx, .. } = &mut self.client_state {
            *public_tx.borrow_mut() = is_public;
            self.metadata_changed(cx);
        }
    }

    pub fn is_public(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local { public_rx, .. } => *public_rx.borrow(),
            ProjectClientState::Remote { .. } => true,
        }
    }

    fn unregister(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        self.unshared(cx);
        if let ProjectClientState::Local { remote_id_rx, .. } = &mut self.client_state {
            if let Some(remote_id) = *remote_id_rx.borrow() {
                let request = self.client.request(proto::UnregisterProject {
                    project_id: remote_id,
                });
                return cx.spawn(|this, mut cx| async move {
                    let response = request.await;
                    this.update(&mut cx, |this, cx| {
                        if let ProjectClientState::Local { remote_id_tx, .. } =
                            &mut this.client_state
                        {
                            *remote_id_tx.borrow_mut() = None;
                        }
                        this.subscriptions.clear();
                        this.metadata_changed(cx);
                    });
                    response.map(drop)
                });
            }
        }
        Task::ready(Ok(()))
    }

    fn register(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if let ProjectClientState::Local { remote_id_rx, .. } = &self.client_state {
            if remote_id_rx.borrow().is_some() {
                return Task::ready(Ok(()));
            }
        }

        let response = self.client.request(proto::RegisterProject {});
        cx.spawn(|this, mut cx| async move {
            let remote_id = response.await?.project_id;
            this.update(&mut cx, |this, cx| {
                if let ProjectClientState::Local { remote_id_tx, .. } = &mut this.client_state {
                    *remote_id_tx.borrow_mut() = Some(remote_id);
                }

                this.metadata_changed(cx);
                cx.emit(Event::RemoteIdChanged(Some(remote_id)));
                this.subscriptions
                    .push(this.client.add_model_for_remote_entity(remote_id, cx));
                Ok(())
            })
        })
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
                watch.next().await;
            }
        }
    }

    pub fn shared_remote_id(&self) -> Option<u64> {
        match &self.client_state {
            ProjectClientState::Local {
                remote_id_rx,
                is_shared,
                ..
            } => {
                if *is_shared {
                    *remote_id_rx.borrow()
                } else {
                    None
                }
            }
            ProjectClientState::Remote { remote_id, .. } => Some(*remote_id),
        }
    }

    pub fn replica_id(&self) -> ReplicaId {
        match &self.client_state {
            ProjectClientState::Local { .. } => 0,
            ProjectClientState::Remote { replica_id, .. } => *replica_id,
        }
    }

    fn metadata_changed(&mut self, cx: &mut ModelContext<Self>) {
        cx.notify();
        self.project_store.update(cx, |_, cx| cx.notify());

        if let ProjectClientState::Local {
            remote_id_rx,
            public_rx,
            ..
        } = &self.client_state
        {
            if let (Some(project_id), true) = (*remote_id_rx.borrow(), *public_rx.borrow()) {
                self.client
                    .send(proto::UpdateProject {
                        project_id,
                        worktrees: self
                            .worktrees
                            .iter()
                            .filter_map(|worktree| {
                                worktree.upgrade(&cx).map(|worktree| {
                                    worktree.read(cx).as_local().unwrap().metadata_proto()
                                })
                            })
                            .collect(),
                    })
                    .log_err();
            }
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

    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = ModelHandle<Worktree>> {
        self.worktrees.iter().filter_map(|worktree| {
            worktree.upgrade(cx).and_then(|worktree| {
                if worktree.read(cx).is_visible() {
                    Some(worktree)
                } else {
                    None
                }
            })
        })
    }

    pub fn worktree_root_names<'a>(&'a self, cx: &'a AppContext) -> impl Iterator<Item = &'a str> {
        self.visible_worktrees(cx)
            .map(|tree| tree.read(cx).root_name())
    }

    pub fn worktree_for_id(
        &self,
        id: WorktreeId,
        cx: &AppContext,
    ) -> Option<ModelHandle<Worktree>> {
        self.worktrees(cx)
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<ModelHandle<Worktree>> {
        self.worktrees(cx)
            .find(|worktree| worktree.read(cx).contains_entry(entry_id))
    }

    pub fn worktree_id_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<WorktreeId> {
        self.worktree_for_entry(entry_id, cx)
            .map(|worktree| worktree.read(cx).id())
    }

    pub fn contains_paths(&self, paths: &[PathBuf], cx: &AppContext) -> bool {
        paths.iter().all(|path| self.contains_path(&path, cx))
    }

    pub fn contains_path(&self, path: &Path, cx: &AppContext) -> bool {
        for worktree in self.worktrees(cx) {
            let worktree = worktree.read(cx).as_local();
            if worktree.map_or(false, |w| w.contains_abs_path(path)) {
                return true;
            }
        }
        false
    }

    pub fn create_entry(
        &mut self,
        project_path: impl Into<ProjectPath>,
        is_directory: bool,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<Entry>>> {
        let project_path = project_path.into();
        let worktree = self.worktree_for_id(project_path.worktree_id, cx)?;
        if self.is_local() {
            Some(worktree.update(cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .create_entry(project_path.path, is_directory, cx)
            }))
        } else {
            let client = self.client.clone();
            let project_id = self.remote_id().unwrap();
            Some(cx.spawn_weak(|_, mut cx| async move {
                let response = client
                    .request(proto::CreateProjectEntry {
                        worktree_id: project_path.worktree_id.to_proto(),
                        project_id,
                        path: project_path.path.as_os_str().as_bytes().to_vec(),
                        is_directory,
                    })
                    .await?;
                let entry = response
                    .entry
                    .ok_or_else(|| anyhow!("missing entry in response"))?;
                worktree
                    .update(&mut cx, |worktree, cx| {
                        worktree.as_remote().unwrap().insert_entry(
                            entry,
                            response.worktree_scan_id as usize,
                            cx,
                        )
                    })
                    .await
            }))
        }
    }

    pub fn copy_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<Entry>>> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        let new_path = new_path.into();
        if self.is_local() {
            worktree.update(cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .copy_entry(entry_id, new_path, cx)
            })
        } else {
            let client = self.client.clone();
            let project_id = self.remote_id().unwrap();

            Some(cx.spawn_weak(|_, mut cx| async move {
                let response = client
                    .request(proto::CopyProjectEntry {
                        project_id,
                        entry_id: entry_id.to_proto(),
                        new_path: new_path.as_os_str().as_bytes().to_vec(),
                    })
                    .await?;
                let entry = response
                    .entry
                    .ok_or_else(|| anyhow!("missing entry in response"))?;
                worktree
                    .update(&mut cx, |worktree, cx| {
                        worktree.as_remote().unwrap().insert_entry(
                            entry,
                            response.worktree_scan_id as usize,
                            cx,
                        )
                    })
                    .await
            }))
        }
    }

    pub fn rename_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<Entry>>> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        let new_path = new_path.into();
        if self.is_local() {
            worktree.update(cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .rename_entry(entry_id, new_path, cx)
            })
        } else {
            let client = self.client.clone();
            let project_id = self.remote_id().unwrap();

            Some(cx.spawn_weak(|_, mut cx| async move {
                let response = client
                    .request(proto::RenameProjectEntry {
                        project_id,
                        entry_id: entry_id.to_proto(),
                        new_path: new_path.as_os_str().as_bytes().to_vec(),
                    })
                    .await?;
                let entry = response
                    .entry
                    .ok_or_else(|| anyhow!("missing entry in response"))?;
                worktree
                    .update(&mut cx, |worktree, cx| {
                        worktree.as_remote().unwrap().insert_entry(
                            entry,
                            response.worktree_scan_id as usize,
                            cx,
                        )
                    })
                    .await
            }))
        }
    }

    pub fn delete_entry(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        if self.is_local() {
            worktree.update(cx, |worktree, cx| {
                worktree.as_local_mut().unwrap().delete_entry(entry_id, cx)
            })
        } else {
            let client = self.client.clone();
            let project_id = self.remote_id().unwrap();
            Some(cx.spawn_weak(|_, mut cx| async move {
                let response = client
                    .request(proto::DeleteProjectEntry {
                        project_id,
                        entry_id: entry_id.to_proto(),
                    })
                    .await?;
                worktree
                    .update(&mut cx, move |worktree, cx| {
                        worktree.as_remote().unwrap().delete_entry(
                            entry_id,
                            response.worktree_scan_id as usize,
                            cx,
                        )
                    })
                    .await
            }))
        }
    }

    fn share(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let project_id;
        if let ProjectClientState::Local {
            remote_id_rx,
            is_shared,
            ..
        } = &mut self.client_state
        {
            if *is_shared {
                return Task::ready(Ok(()));
            }
            *is_shared = true;
            if let Some(id) = *remote_id_rx.borrow() {
                project_id = id;
            } else {
                return Task::ready(Err(anyhow!("project hasn't been registered")));
            }
        } else {
            return Task::ready(Err(anyhow!("can't share a remote project")));
        };

        for open_buffer in self.opened_buffers.values_mut() {
            match open_buffer {
                OpenBuffer::Strong(_) => {}
                OpenBuffer::Weak(buffer) => {
                    if let Some(buffer) = buffer.upgrade(cx) {
                        *open_buffer = OpenBuffer::Strong(buffer);
                    }
                }
                OpenBuffer::Loading(_) => unreachable!(),
            }
        }

        for worktree_handle in self.worktrees.iter_mut() {
            match worktree_handle {
                WorktreeHandle::Strong(_) => {}
                WorktreeHandle::Weak(worktree) => {
                    if let Some(worktree) = worktree.upgrade(cx) {
                        *worktree_handle = WorktreeHandle::Strong(worktree);
                    }
                }
            }
        }

        let mut tasks = Vec::new();
        for worktree in self.worktrees(cx).collect::<Vec<_>>() {
            worktree.update(cx, |worktree, cx| {
                let worktree = worktree.as_local_mut().unwrap();
                tasks.push(worktree.share(project_id, cx));
            });
        }

        cx.spawn(|this, mut cx| async move {
            for task in tasks {
                task.await?;
            }
            this.update(&mut cx, |_, cx| cx.notify());
            Ok(())
        })
    }

    fn unshared(&mut self, cx: &mut ModelContext<Self>) {
        if let ProjectClientState::Local { is_shared, .. } = &mut self.client_state {
            if !*is_shared {
                return;
            }

            *is_shared = false;
            self.collaborators.clear();
            self.shared_buffers.clear();
            for worktree_handle in self.worktrees.iter_mut() {
                if let WorktreeHandle::Strong(worktree) = worktree_handle {
                    let is_visible = worktree.update(cx, |worktree, _| {
                        worktree.as_local_mut().unwrap().unshare();
                        worktree.is_visible()
                    });
                    if !is_visible {
                        *worktree_handle = WorktreeHandle::Weak(worktree.downgrade());
                    }
                }
            }

            for open_buffer in self.opened_buffers.values_mut() {
                match open_buffer {
                    OpenBuffer::Strong(buffer) => {
                        *open_buffer = OpenBuffer::Weak(buffer.downgrade());
                    }
                    _ => {}
                }
            }

            cx.notify();
        } else {
            log::error!("attempted to unshare a remote project");
        }
    }

    pub fn respond_to_join_request(
        &mut self,
        requester_id: u64,
        allow: bool,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(project_id) = self.remote_id() {
            let share = self.share(cx);
            let client = self.client.clone();
            cx.foreground()
                .spawn(async move {
                    share.await?;
                    client.send(proto::RespondToJoinProjectRequest {
                        requester_id,
                        project_id,
                        allow,
                    })
                })
                .detach_and_log_err(cx);
        }
    }

    fn removed_from_project(&mut self, cx: &mut ModelContext<Self>) {
        if let ProjectClientState::Remote {
            sharing_has_stopped,
            ..
        } = &mut self.client_state
        {
            *sharing_has_stopped = true;
            self.collaborators.clear();
            cx.notify();
        }
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

    pub fn create_buffer(
        &mut self,
        text: &str,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Result<ModelHandle<Buffer>> {
        if self.is_remote() {
            return Err(anyhow!("creating buffers as a guest is not supported yet"));
        }

        let buffer = cx.add_model(|cx| {
            Buffer::new(self.replica_id(), text, cx)
                .with_language(language.unwrap_or(language::PLAIN_TEXT.clone()), cx)
        });
        self.register_buffer(&buffer, cx)?;
        Ok(buffer)
    }

    pub fn open_path(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(ProjectEntryId, AnyModelHandle)>> {
        let task = self.open_buffer(path, cx);
        cx.spawn_weak(|_, cx| async move {
            let buffer = task.await?;
            let project_entry_id = buffer
                .read_with(&cx, |buffer, cx| {
                    File::from_dyn(buffer.file()).and_then(|file| file.project_entry_id(cx))
                })
                .ok_or_else(|| anyhow!("no project entry"))?;
            Ok((project_entry_id, buffer.into()))
        })
    }

    pub fn open_local_buffer(
        &mut self,
        abs_path: impl AsRef<Path>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        if let Some((worktree, relative_path)) = self.find_local_worktree(abs_path.as_ref(), cx) {
            self.open_buffer((worktree.read(cx).id(), relative_path), cx)
        } else {
            Task::ready(Err(anyhow!("no such path")))
        }
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
                    self.open_local_buffer_internal(&project_path.path, &worktree, cx)
                } else {
                    self.open_remote_buffer_internal(&project_path.path, &worktree, cx)
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
                loading_watch.next().await;
            }
        })
    }

    fn open_local_buffer_internal(
        &mut self,
        path: &Arc<Path>,
        worktree: &ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let load_buffer = worktree.update(cx, |worktree, cx| {
            let worktree = worktree.as_local_mut().unwrap();
            worktree.load_buffer(path, cx)
        });
        cx.spawn(|this, mut cx| async move {
            let buffer = load_buffer.await?;
            this.update(&mut cx, |this, cx| this.register_buffer(&buffer, cx))?;
            Ok(buffer)
        })
    }

    fn open_remote_buffer_internal(
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
        cx.spawn(|this, mut cx| async move {
            let response = rpc
                .request(proto::OpenBufferByPath {
                    project_id,
                    worktree_id: remote_worktree_id.to_proto(),
                    path: path_string,
                })
                .await?;
            let buffer = response.buffer.ok_or_else(|| anyhow!("missing buffer"))?;
            this.update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
                .await
        })
    }

    fn open_local_buffer_via_lsp(
        &mut self,
        abs_path: lsp::Url,
        lsp_adapter: Arc<dyn LspAdapter>,
        lsp_server: Arc<LanguageServer>,
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
                        this.create_local_worktree(&abs_path, false, cx)
                    })
                    .await?;
                this.update(&mut cx, |this, cx| {
                    this.language_servers.insert(
                        (worktree.read(cx).id(), lsp_adapter.name()),
                        (lsp_adapter, lsp_server),
                    );
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

    pub fn open_buffer_by_id(
        &mut self,
        id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        if let Some(buffer) = self.buffer_for_id(id, cx) {
            Task::ready(Ok(buffer))
        } else if self.is_local() {
            Task::ready(Err(anyhow!("buffer {} does not exist", id)))
        } else if let Some(project_id) = self.remote_id() {
            let request = self
                .client
                .request(proto::OpenBufferById { project_id, id });
            cx.spawn(|this, mut cx| async move {
                let buffer = request
                    .await?
                    .buffer
                    .ok_or_else(|| anyhow!("invalid buffer"))?;
                this.update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
                    .await
            })
        } else {
            Task::ready(Err(anyhow!("cannot open buffer while disconnected")))
        }
    }

    pub fn save_buffer_as(
        &mut self,
        buffer: ModelHandle<Buffer>,
        abs_path: PathBuf,
        cx: &mut ModelContext<Project>,
    ) -> Task<Result<()>> {
        let worktree_task = self.find_or_create_local_worktree(&abs_path, true, cx);
        let old_path =
            File::from_dyn(buffer.read(cx).file()).and_then(|f| Some(f.as_local()?.abs_path(cx)));
        cx.spawn(|this, mut cx| async move {
            if let Some(old_path) = old_path {
                this.update(&mut cx, |this, cx| {
                    this.unregister_buffer_from_language_server(&buffer, old_path, cx);
                });
            }
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
                this.assign_language_to_buffer(&buffer, cx);
                this.register_buffer_with_language_server(&buffer, cx);
            });
            Ok(())
        })
    }

    pub fn get_open_buffer(
        &mut self,
        path: &ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Option<ModelHandle<Buffer>> {
        let worktree = self.worktree_for_id(path.worktree_id, cx)?;
        self.opened_buffers.values().find_map(|buffer| {
            let buffer = buffer.upgrade(cx)?;
            let file = File::from_dyn(buffer.read(cx).file())?;
            if file.worktree == worktree && file.path() == &path.path {
                Some(buffer)
            } else {
                None
            }
        })
    }

    fn register_buffer(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let remote_id = buffer.read(cx).remote_id();
        let open_buffer = if self.is_remote() || self.is_shared() {
            OpenBuffer::Strong(buffer.clone())
        } else {
            OpenBuffer::Weak(buffer.downgrade())
        };

        match self.opened_buffers.insert(remote_id, open_buffer) {
            None => {}
            Some(OpenBuffer::Loading(operations)) => {
                buffer.update(cx, |buffer, cx| buffer.apply_ops(operations, cx))?
            }
            Some(OpenBuffer::Weak(existing_handle)) => {
                if existing_handle.upgrade(cx).is_some() {
                    Err(anyhow!(
                        "already registered buffer with remote id {}",
                        remote_id
                    ))?
                }
            }
            Some(OpenBuffer::Strong(_)) => Err(anyhow!(
                "already registered buffer with remote id {}",
                remote_id
            ))?,
        }
        cx.subscribe(buffer, |this, buffer, event, cx| {
            this.on_buffer_event(buffer, event, cx);
        })
        .detach();

        self.assign_language_to_buffer(buffer, cx);
        self.register_buffer_with_language_server(buffer, cx);
        cx.observe_release(buffer, |this, buffer, cx| {
            if let Some(file) = File::from_dyn(buffer.file()) {
                if file.is_local() {
                    let uri = lsp::Url::from_file_path(file.abs_path(cx)).unwrap();
                    if let Some((_, server)) = this.language_server_for_buffer(buffer, cx) {
                        server
                            .notify::<lsp::notification::DidCloseTextDocument>(
                                lsp::DidCloseTextDocumentParams {
                                    text_document: lsp::TextDocumentIdentifier::new(uri.clone()),
                                },
                            )
                            .log_err();
                    }
                }
            }
        })
        .detach();

        Ok(())
    }

    fn register_buffer_with_language_server(
        &mut self,
        buffer_handle: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id();
        if let Some(file) = File::from_dyn(buffer.file()) {
            if file.is_local() {
                let uri = lsp::Url::from_file_path(file.abs_path(cx)).unwrap();
                let initial_snapshot = buffer.text_snapshot();

                let mut language_server = None;
                let mut language_id = None;
                if let Some(language) = buffer.language() {
                    let worktree_id = file.worktree_id(cx);
                    if let Some(adapter) = language.lsp_adapter() {
                        language_id = adapter.id_for_language(language.name().as_ref());
                        language_server = self
                            .language_servers
                            .get(&(worktree_id, adapter.name()))
                            .cloned();
                    }
                }

                if let Some(local_worktree) = file.worktree.read(cx).as_local() {
                    if let Some(diagnostics) = local_worktree.diagnostics_for_path(file.path()) {
                        self.update_buffer_diagnostics(&buffer_handle, diagnostics, None, cx)
                            .log_err();
                    }
                }

                if let Some((_, server)) = language_server {
                    server
                        .notify::<lsp::notification::DidOpenTextDocument>(
                            lsp::DidOpenTextDocumentParams {
                                text_document: lsp::TextDocumentItem::new(
                                    uri,
                                    language_id.unwrap_or_default(),
                                    0,
                                    initial_snapshot.text(),
                                ),
                            }
                            .clone(),
                        )
                        .log_err();
                    buffer_handle.update(cx, |buffer, cx| {
                        buffer.set_completion_triggers(
                            server
                                .capabilities()
                                .completion_provider
                                .as_ref()
                                .and_then(|provider| provider.trigger_characters.clone())
                                .unwrap_or(Vec::new()),
                            cx,
                        )
                    });
                    self.buffer_snapshots
                        .insert(buffer_id, vec![(0, initial_snapshot)]);
                }
            }
        }
    }

    fn unregister_buffer_from_language_server(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        old_path: PathBuf,
        cx: &mut ModelContext<Self>,
    ) {
        buffer.update(cx, |buffer, cx| {
            buffer.update_diagnostics(Default::default(), cx);
            self.buffer_snapshots.remove(&buffer.remote_id());
            if let Some((_, language_server)) = self.language_server_for_buffer(buffer, cx) {
                language_server
                    .notify::<lsp::notification::DidCloseTextDocument>(
                        lsp::DidCloseTextDocumentParams {
                            text_document: lsp::TextDocumentIdentifier::new(
                                lsp::Url::from_file_path(old_path).unwrap(),
                            ),
                        },
                    )
                    .log_err();
            }
        });
    }

    fn on_buffer_event(
        &mut self,
        buffer: ModelHandle<Buffer>,
        event: &BufferEvent,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        match event {
            BufferEvent::Operation(operation) => {
                if let Some(project_id) = self.shared_remote_id() {
                    let request = self.client.request(proto::UpdateBuffer {
                        project_id,
                        buffer_id: buffer.read(cx).remote_id(),
                        operations: vec![language::proto::serialize_operation(&operation)],
                    });
                    cx.background().spawn(request).detach_and_log_err(cx);
                }
            }
            BufferEvent::Edited { .. } => {
                let (_, language_server) = self
                    .language_server_for_buffer(buffer.read(cx), cx)?
                    .clone();
                let buffer = buffer.read(cx);
                let file = File::from_dyn(buffer.file())?;
                let abs_path = file.as_local()?.abs_path(cx);
                let uri = lsp::Url::from_file_path(abs_path).unwrap();
                let buffer_snapshots = self.buffer_snapshots.get_mut(&buffer.remote_id())?;
                let (version, prev_snapshot) = buffer_snapshots.last()?;
                let next_snapshot = buffer.text_snapshot();
                let next_version = version + 1;

                let content_changes = buffer
                    .edits_since::<(PointUtf16, usize)>(prev_snapshot.version())
                    .map(|edit| {
                        let edit_start = edit.new.start.0;
                        let edit_end = edit_start + (edit.old.end.0 - edit.old.start.0);
                        let new_text = next_snapshot
                            .text_for_range(edit.new.start.1..edit.new.end.1)
                            .collect();
                        lsp::TextDocumentContentChangeEvent {
                            range: Some(lsp::Range::new(
                                point_to_lsp(edit_start),
                                point_to_lsp(edit_end),
                            )),
                            range_length: None,
                            text: new_text,
                        }
                    })
                    .collect();

                buffer_snapshots.push((next_version, next_snapshot));

                language_server
                    .notify::<lsp::notification::DidChangeTextDocument>(
                        lsp::DidChangeTextDocumentParams {
                            text_document: lsp::VersionedTextDocumentIdentifier::new(
                                uri,
                                next_version,
                            ),
                            content_changes,
                        },
                    )
                    .log_err();
            }
            BufferEvent::Saved => {
                let file = File::from_dyn(buffer.read(cx).file())?;
                let worktree_id = file.worktree_id(cx);
                let abs_path = file.as_local()?.abs_path(cx);
                let text_document = lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(abs_path).unwrap(),
                };

                for (_, server) in self.language_servers_for_worktree(worktree_id) {
                    server
                        .notify::<lsp::notification::DidSaveTextDocument>(
                            lsp::DidSaveTextDocumentParams {
                                text_document: text_document.clone(),
                                text: None,
                            },
                        )
                        .log_err();
                }
            }
            _ => {}
        }

        None
    }

    fn language_servers_for_worktree(
        &self,
        worktree_id: WorktreeId,
    ) -> impl Iterator<Item = &(Arc<dyn LspAdapter>, Arc<LanguageServer>)> {
        self.language_servers.iter().filter_map(
            move |((language_server_worktree_id, _), server)| {
                if *language_server_worktree_id == worktree_id {
                    Some(server)
                } else {
                    None
                }
            },
        )
    }

    fn assign_language_to_buffer(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        // If the buffer has a language, set it and start the language server if we haven't already.
        let full_path = buffer.read(cx).file()?.full_path(cx);
        let language = self.languages.select_language(&full_path)?;
        buffer.update(cx, |buffer, cx| {
            buffer.set_language(Some(language.clone()), cx);
        });

        let file = File::from_dyn(buffer.read(cx).file())?;
        let worktree = file.worktree.read(cx).as_local()?;
        let worktree_id = worktree.id();
        let worktree_abs_path = worktree.abs_path().clone();
        self.start_language_server(worktree_id, worktree_abs_path, language, cx);

        None
    }

    fn start_language_server(
        &mut self,
        worktree_id: WorktreeId,
        worktree_path: Arc<Path>,
        language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        let adapter = if let Some(adapter) = language.lsp_adapter() {
            adapter
        } else {
            return;
        };
        let key = (worktree_id, adapter.name());
        self.started_language_servers
            .entry(key.clone())
            .or_insert_with(|| {
                let server_id = post_inc(&mut self.next_language_server_id);
                let language_server = self.languages.start_language_server(
                    server_id,
                    language.clone(),
                    worktree_path,
                    self.client.http_client(),
                    cx,
                );
                cx.spawn_weak(|this, mut cx| async move {
                    let language_server = language_server?.await.log_err()?;
                    let language_server = language_server
                        .initialize(adapter.initialization_options())
                        .await
                        .log_err()?;
                    let this = this.upgrade(&cx)?;
                    let disk_based_diagnostics_progress_token =
                        adapter.disk_based_diagnostics_progress_token();

                    language_server
                        .on_notification::<lsp::notification::PublishDiagnostics, _>({
                            let this = this.downgrade();
                            let adapter = adapter.clone();
                            move |params, mut cx| {
                                if let Some(this) = this.upgrade(&cx) {
                                    this.update(&mut cx, |this, cx| {
                                        this.on_lsp_diagnostics_published(
                                            server_id,
                                            params,
                                            &adapter,
                                            disk_based_diagnostics_progress_token,
                                            cx,
                                        );
                                    });
                                }
                            }
                        })
                        .detach();

                    language_server
                        .on_request::<lsp::request::WorkspaceConfiguration, _, _>({
                            let settings = this
                                .read_with(&cx, |this, _| this.language_server_settings.clone());
                            move |params, _| {
                                let settings = settings.lock().clone();
                                async move {
                                    Ok(params
                                        .items
                                        .into_iter()
                                        .map(|item| {
                                            if let Some(section) = &item.section {
                                                settings
                                                    .get(section)
                                                    .cloned()
                                                    .unwrap_or(serde_json::Value::Null)
                                            } else {
                                                settings.clone()
                                            }
                                        })
                                        .collect())
                                }
                            }
                        })
                        .detach();

                    language_server
                        .on_request::<lsp::request::ApplyWorkspaceEdit, _, _>({
                            let this = this.downgrade();
                            let adapter = adapter.clone();
                            let language_server = language_server.clone();
                            move |params, cx| {
                                Self::on_lsp_workspace_edit(
                                    this,
                                    params,
                                    server_id,
                                    adapter.clone(),
                                    language_server.clone(),
                                    cx,
                                )
                            }
                        })
                        .detach();

                    language_server
                        .on_notification::<lsp::notification::Progress, _>({
                            let this = this.downgrade();
                            move |params, mut cx| {
                                if let Some(this) = this.upgrade(&cx) {
                                    this.update(&mut cx, |this, cx| {
                                        this.on_lsp_progress(
                                            params,
                                            server_id,
                                            disk_based_diagnostics_progress_token,
                                            cx,
                                        );
                                    });
                                }
                            }
                        })
                        .detach();

                    this.update(&mut cx, |this, cx| {
                        this.language_servers
                            .insert(key.clone(), (adapter.clone(), language_server.clone()));
                        this.language_server_statuses.insert(
                            server_id,
                            LanguageServerStatus {
                                name: language_server.name().to_string(),
                                pending_work: Default::default(),
                                pending_diagnostic_updates: 0,
                            },
                        );
                        language_server
                            .notify::<lsp::notification::DidChangeConfiguration>(
                                lsp::DidChangeConfigurationParams {
                                    settings: this.language_server_settings.lock().clone(),
                                },
                            )
                            .ok();

                        if let Some(project_id) = this.shared_remote_id() {
                            this.client
                                .send(proto::StartLanguageServer {
                                    project_id,
                                    server: Some(proto::LanguageServer {
                                        id: server_id as u64,
                                        name: language_server.name().to_string(),
                                    }),
                                })
                                .log_err();
                        }

                        // Tell the language server about every open buffer in the worktree that matches the language.
                        for buffer in this.opened_buffers.values() {
                            if let Some(buffer_handle) = buffer.upgrade(cx) {
                                let buffer = buffer_handle.read(cx);
                                let file = if let Some(file) = File::from_dyn(buffer.file()) {
                                    file
                                } else {
                                    continue;
                                };
                                let language = if let Some(language) = buffer.language() {
                                    language
                                } else {
                                    continue;
                                };
                                if file.worktree.read(cx).id() != key.0
                                    || language.lsp_adapter().map(|a| a.name())
                                        != Some(key.1.clone())
                                {
                                    continue;
                                }

                                let file = file.as_local()?;
                                let versions = this
                                    .buffer_snapshots
                                    .entry(buffer.remote_id())
                                    .or_insert_with(|| vec![(0, buffer.text_snapshot())]);
                                let (version, initial_snapshot) = versions.last().unwrap();
                                let uri = lsp::Url::from_file_path(file.abs_path(cx)).unwrap();
                                let language_id = adapter.id_for_language(language.name().as_ref());
                                language_server
                                    .notify::<lsp::notification::DidOpenTextDocument>(
                                        lsp::DidOpenTextDocumentParams {
                                            text_document: lsp::TextDocumentItem::new(
                                                uri,
                                                language_id.unwrap_or_default(),
                                                *version,
                                                initial_snapshot.text(),
                                            ),
                                        },
                                    )
                                    .log_err()?;
                                buffer_handle.update(cx, |buffer, cx| {
                                    buffer.set_completion_triggers(
                                        language_server
                                            .capabilities()
                                            .completion_provider
                                            .as_ref()
                                            .and_then(|provider| {
                                                provider.trigger_characters.clone()
                                            })
                                            .unwrap_or(Vec::new()),
                                        cx,
                                    )
                                });
                            }
                        }

                        cx.notify();
                        Some(())
                    });

                    Some(language_server)
                })
            });
    }

    pub fn restart_language_servers_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = ModelHandle<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let language_server_lookup_info: HashSet<(WorktreeId, Arc<Path>, PathBuf)> = buffers
            .into_iter()
            .filter_map(|buffer| {
                let file = File::from_dyn(buffer.read(cx).file())?;
                let worktree = file.worktree.read(cx).as_local()?;
                let worktree_id = worktree.id();
                let worktree_abs_path = worktree.abs_path().clone();
                let full_path = file.full_path(cx);
                Some((worktree_id, worktree_abs_path, full_path))
            })
            .collect();
        for (worktree_id, worktree_abs_path, full_path) in language_server_lookup_info {
            let language = self.languages.select_language(&full_path)?;
            self.restart_language_server(worktree_id, worktree_abs_path, language, cx);
        }

        None
    }

    fn restart_language_server(
        &mut self,
        worktree_id: WorktreeId,
        worktree_path: Arc<Path>,
        language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        let adapter = if let Some(adapter) = language.lsp_adapter() {
            adapter
        } else {
            return;
        };
        let key = (worktree_id, adapter.name());
        let server_to_shutdown = self.language_servers.remove(&key);
        self.started_language_servers.remove(&key);
        server_to_shutdown
            .as_ref()
            .map(|(_, server)| self.language_server_statuses.remove(&server.server_id()));
        cx.spawn_weak(|this, mut cx| async move {
            if let Some(this) = this.upgrade(&cx) {
                if let Some((_, server_to_shutdown)) = server_to_shutdown {
                    if let Some(shutdown_task) = server_to_shutdown.shutdown() {
                        shutdown_task.await;
                    }
                }

                this.update(&mut cx, |this, cx| {
                    this.start_language_server(worktree_id, worktree_path, language, cx);
                });
            }
        })
        .detach();
    }

    fn on_lsp_diagnostics_published(
        &mut self,
        server_id: usize,
        mut params: lsp::PublishDiagnosticsParams,
        adapter: &Arc<dyn LspAdapter>,
        disk_based_diagnostics_progress_token: Option<&str>,
        cx: &mut ModelContext<Self>,
    ) {
        adapter.process_diagnostics(&mut params);
        if disk_based_diagnostics_progress_token.is_none() {
            self.disk_based_diagnostics_started(cx);
            self.broadcast_language_server_update(
                server_id,
                proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(
                    proto::LspDiskBasedDiagnosticsUpdating {},
                ),
            );
        }
        self.update_diagnostics(params, adapter.disk_based_diagnostic_sources(), cx)
            .log_err();
        if disk_based_diagnostics_progress_token.is_none() {
            self.disk_based_diagnostics_finished(cx);
            self.broadcast_language_server_update(
                server_id,
                proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                    proto::LspDiskBasedDiagnosticsUpdated {},
                ),
            );
        }
    }

    fn on_lsp_progress(
        &mut self,
        progress: lsp::ProgressParams,
        server_id: usize,
        disk_based_diagnostics_progress_token: Option<&str>,
        cx: &mut ModelContext<Self>,
    ) {
        let token = match progress.token {
            lsp::NumberOrString::String(token) => token,
            lsp::NumberOrString::Number(token) => {
                log::info!("skipping numeric progress token {}", token);
                return;
            }
        };
        let progress = match progress.value {
            lsp::ProgressParamsValue::WorkDone(value) => value,
        };
        let language_server_status =
            if let Some(status) = self.language_server_statuses.get_mut(&server_id) {
                status
            } else {
                return;
            };
        match progress {
            lsp::WorkDoneProgress::Begin(_) => {
                if Some(token.as_str()) == disk_based_diagnostics_progress_token {
                    language_server_status.pending_diagnostic_updates += 1;
                    if language_server_status.pending_diagnostic_updates == 1 {
                        self.disk_based_diagnostics_started(cx);
                        self.broadcast_language_server_update(
                            server_id,
                            proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(
                                proto::LspDiskBasedDiagnosticsUpdating {},
                            ),
                        );
                    }
                } else {
                    self.on_lsp_work_start(server_id, token.clone(), cx);
                    self.broadcast_language_server_update(
                        server_id,
                        proto::update_language_server::Variant::WorkStart(proto::LspWorkStart {
                            token,
                        }),
                    );
                }
            }
            lsp::WorkDoneProgress::Report(report) => {
                if Some(token.as_str()) != disk_based_diagnostics_progress_token {
                    self.on_lsp_work_progress(
                        server_id,
                        token.clone(),
                        LanguageServerProgress {
                            message: report.message.clone(),
                            percentage: report.percentage.map(|p| p as usize),
                            last_update_at: Instant::now(),
                        },
                        cx,
                    );
                    self.broadcast_language_server_update(
                        server_id,
                        proto::update_language_server::Variant::WorkProgress(
                            proto::LspWorkProgress {
                                token,
                                message: report.message,
                                percentage: report.percentage.map(|p| p as u32),
                            },
                        ),
                    );
                }
            }
            lsp::WorkDoneProgress::End(_) => {
                if Some(token.as_str()) == disk_based_diagnostics_progress_token {
                    language_server_status.pending_diagnostic_updates -= 1;
                    if language_server_status.pending_diagnostic_updates == 0 {
                        self.disk_based_diagnostics_finished(cx);
                        self.broadcast_language_server_update(
                            server_id,
                            proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                                proto::LspDiskBasedDiagnosticsUpdated {},
                            ),
                        );
                    }
                } else {
                    self.on_lsp_work_end(server_id, token.clone(), cx);
                    self.broadcast_language_server_update(
                        server_id,
                        proto::update_language_server::Variant::WorkEnd(proto::LspWorkEnd {
                            token,
                        }),
                    );
                }
            }
        }
    }

    fn on_lsp_work_start(
        &mut self,
        language_server_id: usize,
        token: String,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            status.pending_work.insert(
                token,
                LanguageServerProgress {
                    message: None,
                    percentage: None,
                    last_update_at: Instant::now(),
                },
            );
            cx.notify();
        }
    }

    fn on_lsp_work_progress(
        &mut self,
        language_server_id: usize,
        token: String,
        progress: LanguageServerProgress,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            status.pending_work.insert(token, progress);
            cx.notify();
        }
    }

    fn on_lsp_work_end(
        &mut self,
        language_server_id: usize,
        token: String,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            status.pending_work.remove(&token);
            cx.notify();
        }
    }

    async fn on_lsp_workspace_edit(
        this: WeakModelHandle<Self>,
        params: lsp::ApplyWorkspaceEditParams,
        server_id: usize,
        adapter: Arc<dyn LspAdapter>,
        language_server: Arc<LanguageServer>,
        mut cx: AsyncAppContext,
    ) -> Result<lsp::ApplyWorkspaceEditResponse> {
        let this = this
            .upgrade(&cx)
            .ok_or_else(|| anyhow!("project project closed"))?;
        let transaction = Self::deserialize_workspace_edit(
            this.clone(),
            params.edit,
            true,
            adapter.clone(),
            language_server.clone(),
            &mut cx,
        )
        .await
        .log_err();
        this.update(&mut cx, |this, _| {
            if let Some(transaction) = transaction {
                this.last_workspace_edits_by_language_server
                    .insert(server_id, transaction);
            }
        });
        Ok(lsp::ApplyWorkspaceEditResponse {
            applied: true,
            failed_change: None,
            failure_reason: None,
        })
    }

    fn broadcast_language_server_update(
        &self,
        language_server_id: usize,
        event: proto::update_language_server::Variant,
    ) {
        if let Some(project_id) = self.shared_remote_id() {
            self.client
                .send(proto::UpdateLanguageServer {
                    project_id,
                    language_server_id: language_server_id as u64,
                    variant: Some(event),
                })
                .log_err();
        }
    }

    pub fn set_language_server_settings(&mut self, settings: serde_json::Value) {
        for (_, server) in self.language_servers.values() {
            server
                .notify::<lsp::notification::DidChangeConfiguration>(
                    lsp::DidChangeConfigurationParams {
                        settings: settings.clone(),
                    },
                )
                .ok();
        }
        *self.language_server_settings.lock() = settings;
    }

    pub fn language_server_statuses(
        &self,
    ) -> impl DoubleEndedIterator<Item = &LanguageServerStatus> {
        self.language_server_statuses.values()
    }

    pub fn update_diagnostics(
        &mut self,
        params: lsp::PublishDiagnosticsParams,
        disk_based_sources: &[&str],
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let abs_path = params
            .uri
            .to_file_path()
            .map_err(|_| anyhow!("URI is not a file"))?;
        let mut diagnostics = Vec::default();
        let mut primary_diagnostic_group_ids = HashMap::default();
        let mut sources_by_group_id = HashMap::default();
        let mut supporting_diagnostics = HashMap::default();
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

            let is_unnecessary = diagnostic.tags.as_ref().map_or(false, |tags| {
                tags.iter().any(|tag| *tag == DiagnosticTag::UNNECESSARY)
            });

            if is_supporting {
                supporting_diagnostics.insert(
                    (source, code.clone(), range),
                    (diagnostic.severity, is_unnecessary),
                );
            } else {
                let group_id = post_inc(&mut self.next_diagnostic_group_id);
                let is_disk_based = source.map_or(false, |source| {
                    disk_based_sources.contains(&source.as_str())
                });

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
                        is_unnecessary,
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
                                    is_unnecessary: false,
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
                if let Some(&(severity, is_unnecessary)) = supporting_diagnostics.get(&(
                    source,
                    diagnostic.code.clone(),
                    entry.range.clone(),
                )) {
                    if let Some(severity) = severity {
                        diagnostic.severity = severity;
                    }
                    diagnostic.is_unnecessary = is_unnecessary;
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
        if !worktree.read(cx).is_visible() {
            return Ok(());
        }

        let project_path = ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: relative_path.into(),
        };
        if let Some(buffer) = self.get_open_buffer(&project_path, cx) {
            self.update_buffer_diagnostics(&buffer, diagnostics.clone(), version, cx)?;
        }

        let updated = worktree.update(cx, |worktree, cx| {
            worktree
                .as_local_mut()
                .ok_or_else(|| anyhow!("not a local worktree"))?
                .update_diagnostics(project_path.path.clone(), diagnostics, cx)
        })?;
        if updated {
            cx.emit(Event::DiagnosticsUpdated(project_path));
        }
        Ok(())
    }

    fn update_buffer_diagnostics(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        mut diagnostics: Vec<DiagnosticEntry<PointUtf16>>,
        version: Option<i32>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        fn compare_diagnostics(a: &Diagnostic, b: &Diagnostic) -> Ordering {
            Ordering::Equal
                .then_with(|| b.is_primary.cmp(&a.is_primary))
                .then_with(|| a.is_disk_based.cmp(&b.is_disk_based))
                .then_with(|| a.severity.cmp(&b.severity))
                .then_with(|| a.message.cmp(&b.message))
        }

        let snapshot = self.buffer_snapshot_for_lsp_version(buffer, version, cx)?;

        diagnostics.sort_unstable_by(|a, b| {
            Ordering::Equal
                .then_with(|| a.range.start.cmp(&b.range.start))
                .then_with(|| b.range.end.cmp(&a.range.end))
                .then_with(|| compare_diagnostics(&a.diagnostic, &b.diagnostic))
        });

        let mut sanitized_diagnostics = Vec::new();
        let edits_since_save = Patch::new(
            snapshot
                .edits_since::<PointUtf16>(buffer.read(cx).saved_version())
                .collect(),
        );
        for entry in diagnostics {
            let start;
            let end;
            if entry.diagnostic.is_disk_based {
                // Some diagnostics are based on files on disk instead of buffers'
                // current contents. Adjust these diagnostics' ranges to reflect
                // any unsaved edits.
                start = edits_since_save.old_to_new(entry.range.start);
                end = edits_since_save.old_to_new(entry.range.end);
            } else {
                start = entry.range.start;
                end = entry.range.end;
            }

            let mut range = snapshot.clip_point_utf16(start, Bias::Left)
                ..snapshot.clip_point_utf16(end, Bias::Right);

            // Expand empty ranges by one character
            if range.start == range.end {
                range.end.column += 1;
                range.end = snapshot.clip_point_utf16(range.end, Bias::Right);
                if range.start == range.end && range.end.column > 0 {
                    range.start.column -= 1;
                    range.start = snapshot.clip_point_utf16(range.start, Bias::Left);
                }
            }

            sanitized_diagnostics.push(DiagnosticEntry {
                range,
                diagnostic: entry.diagnostic,
            });
        }
        drop(edits_since_save);

        let set = DiagnosticSet::new(sanitized_diagnostics, &snapshot);
        buffer.update(cx, |buffer, cx| buffer.update_diagnostics(set, cx));
        Ok(())
    }

    pub fn reload_buffers(
        &self,
        buffers: HashSet<ModelHandle<Buffer>>,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        let mut local_buffers = Vec::new();
        let mut remote_buffers = None;
        for buffer_handle in buffers {
            let buffer = buffer_handle.read(cx);
            if buffer.is_dirty() {
                if let Some(file) = File::from_dyn(buffer.file()) {
                    if file.is_local() {
                        local_buffers.push(buffer_handle);
                    } else {
                        remote_buffers.get_or_insert(Vec::new()).push(buffer_handle);
                    }
                }
            }
        }

        let remote_buffers = self.remote_id().zip(remote_buffers);
        let client = self.client.clone();

        cx.spawn(|this, mut cx| async move {
            let mut project_transaction = ProjectTransaction::default();

            if let Some((project_id, remote_buffers)) = remote_buffers {
                let response = client
                    .request(proto::ReloadBuffers {
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
                        this.deserialize_project_transaction(response, push_to_history, cx)
                    })
                    .await?;
            }

            for buffer in local_buffers {
                let transaction = buffer
                    .update(&mut cx, |buffer, cx| buffer.reload(cx))
                    .await?;
                buffer.update(&mut cx, |buffer, cx| {
                    if let Some(transaction) = transaction {
                        if !push_to_history {
                            buffer.forget_transaction(transaction.id);
                        }
                        project_transaction.0.insert(cx.handle(), transaction);
                    }
                });
            }

            Ok(project_transaction)
        })
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
            if let Some(file) = File::from_dyn(buffer.file()) {
                if let Some(buffer_abs_path) = file.as_local().map(|f| f.abs_path(cx)) {
                    if let Some((_, server)) = self.language_server_for_buffer(buffer, cx) {
                        local_buffers.push((buffer_handle, buffer_abs_path, server.clone()));
                    }
                } else {
                    remote_buffers.get_or_insert(Vec::new()).push(buffer_handle);
                }
            } else {
                return Task::ready(Ok(Default::default()));
            }
        }

        let remote_buffers = self.remote_id().zip(remote_buffers);
        let client = self.client.clone();

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
                        this.deserialize_project_transaction(response, push_to_history, cx)
                    })
                    .await?;
            }

            for (buffer, buffer_abs_path, language_server) in local_buffers {
                let text_document = lsp::TextDocumentIdentifier::new(
                    lsp::Url::from_file_path(&buffer_abs_path).unwrap(),
                );
                let capabilities = &language_server.capabilities();
                let tab_size = cx.update(|cx| {
                    let language_name = buffer.read(cx).language().map(|language| language.name());
                    cx.global::<Settings>().tab_size(language_name.as_deref())
                });
                let lsp_edits = if capabilities
                    .document_formatting_provider
                    .as_ref()
                    .map_or(false, |provider| *provider != lsp::OneOf::Left(false))
                {
                    language_server
                        .request::<lsp::request::Formatting>(lsp::DocumentFormattingParams {
                            text_document,
                            options: lsp::FormattingOptions {
                                tab_size,
                                insert_spaces: true,
                                insert_final_newline: Some(true),
                                ..Default::default()
                            },
                            work_done_progress_params: Default::default(),
                        })
                        .await?
                } else if capabilities
                    .document_range_formatting_provider
                    .as_ref()
                    .map_or(false, |provider| *provider != lsp::OneOf::Left(false))
                {
                    let buffer_start = lsp::Position::new(0, 0);
                    let buffer_end =
                        buffer.read_with(&cx, |buffer, _| point_to_lsp(buffer.max_point_utf16()));
                    language_server
                        .request::<lsp::request::RangeFormatting>(
                            lsp::DocumentRangeFormattingParams {
                                text_document,
                                range: lsp::Range::new(buffer_start, buffer_end),
                                options: lsp::FormattingOptions {
                                    tab_size: 4,
                                    insert_spaces: true,
                                    insert_final_newline: Some(true),
                                    ..Default::default()
                                },
                                work_done_progress_params: Default::default(),
                            },
                        )
                        .await?
                } else {
                    continue;
                };

                if let Some(lsp_edits) = lsp_edits {
                    let edits = this
                        .update(&mut cx, |this, cx| {
                            this.edits_from_lsp(&buffer, lsp_edits, None, cx)
                        })
                        .await?;
                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();
                        for (range, text) in edits {
                            buffer.edit([(range, text)], cx);
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
            let mut requests = Vec::new();
            for ((worktree_id, _), (lsp_adapter, language_server)) in self.language_servers.iter() {
                let worktree_id = *worktree_id;
                if let Some(worktree) = self
                    .worktree_for_id(worktree_id, cx)
                    .and_then(|worktree| worktree.read(cx).as_local())
                {
                    let lsp_adapter = lsp_adapter.clone();
                    let worktree_abs_path = worktree.abs_path().clone();
                    requests.push(
                        language_server
                            .request::<lsp::request::WorkspaceSymbol>(lsp::WorkspaceSymbolParams {
                                query: query.to_string(),
                                ..Default::default()
                            })
                            .log_err()
                            .map(move |response| {
                                (
                                    lsp_adapter,
                                    worktree_id,
                                    worktree_abs_path,
                                    response.unwrap_or_default(),
                                )
                            }),
                    );
                }
            }

            cx.spawn_weak(|this, cx| async move {
                let responses = futures::future::join_all(requests).await;
                let this = if let Some(this) = this.upgrade(&cx) {
                    this
                } else {
                    return Ok(Default::default());
                };
                this.read_with(&cx, |this, cx| {
                    let mut symbols = Vec::new();
                    for (adapter, source_worktree_id, worktree_abs_path, response) in responses {
                        symbols.extend(response.into_iter().flatten().filter_map(|lsp_symbol| {
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

                            let label = this
                                .languages
                                .select_language(&path)
                                .and_then(|language| {
                                    language.label_for_symbol(&lsp_symbol.name, lsp_symbol.kind)
                                })
                                .unwrap_or_else(|| CodeLabel::plain(lsp_symbol.name.clone(), None));
                            let signature = this.symbol_signature(worktree_id, &path);

                            Some(Symbol {
                                source_worktree_id,
                                worktree_id,
                                language_server_name: adapter.name(),
                                name: lsp_symbol.name,
                                kind: lsp_symbol.kind,
                                label,
                                path,
                                range: range_from_lsp(lsp_symbol.location.range),
                                signature,
                            })
                        }));
                    }
                    Ok(symbols)
                })
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
            let (lsp_adapter, language_server) = if let Some(server) = self.language_servers.get(&(
                symbol.source_worktree_id,
                symbol.language_server_name.clone(),
            )) {
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

            self.open_local_buffer_via_lsp(symbol_uri, lsp_adapter, language_server, cx)
        } else if let Some(project_id) = self.remote_id() {
            let request = self.client.request(proto::OpenBufferForSymbol {
                project_id,
                symbol: Some(serialize_symbol(symbol)),
            });
            cx.spawn(|this, mut cx| async move {
                let response = request.await?;
                let buffer = response.buffer.ok_or_else(|| anyhow!("invalid buffer"))?;
                this.update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
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
            let (_, lang_server) =
                if let Some(server) = self.language_server_for_buffer(source_buffer, cx) {
                    server.clone()
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
                            point_to_lsp(position),
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
                    let snapshot = this.snapshot();
                    let clipped_position = this.clip_point_utf16(position, Bias::Left);
                    let mut range_for_token = None;
                    Ok(completions
                        .into_iter()
                        .filter_map(|lsp_completion| {
                            let (old_range, new_text) = match lsp_completion.text_edit.as_ref() {
                                // If the language server provides a range to overwrite, then
                                // check that the range is valid.
                                Some(lsp::CompletionTextEdit::Edit(edit)) => {
                                    let range = range_from_lsp(edit.range);
                                    let start = snapshot.clip_point_utf16(range.start, Bias::Left);
                                    let end = snapshot.clip_point_utf16(range.end, Bias::Left);
                                    if start != range.start || end != range.end {
                                        log::info!("completion out of expected range");
                                        return None;
                                    }
                                    (
                                        snapshot.anchor_before(start)..snapshot.anchor_after(end),
                                        edit.new_text.clone(),
                                    )
                                }
                                // If the language server does not provide a range, then infer
                                // the range based on the syntax tree.
                                None => {
                                    if position != clipped_position {
                                        log::info!("completion out of expected range");
                                        return None;
                                    }
                                    let Range { start, end } = range_for_token
                                        .get_or_insert_with(|| {
                                            let offset = position.to_offset(&snapshot);
                                            snapshot
                                                .range_for_word_token_at(offset)
                                                .unwrap_or_else(|| offset..offset)
                                        })
                                        .clone();
                                    let text = lsp_completion
                                        .insert_text
                                        .as_ref()
                                        .unwrap_or(&lsp_completion.label)
                                        .clone();
                                    (
                                        snapshot.anchor_before(start)..snapshot.anchor_after(end),
                                        text.clone(),
                                    )
                                }
                                Some(lsp::CompletionTextEdit::InsertAndReplace(_)) => {
                                    log::info!("unsupported insert/replace completion");
                                    return None;
                                }
                            };

                            Some(Completion {
                                old_range,
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
                version: serialize_version(&source_buffer.version()),
            };
            cx.spawn_weak(|_, mut cx| async move {
                let response = rpc.request(message).await?;

                source_buffer_handle
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(deserialize_version(response.version))
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
            let (_, lang_server) = if let Some(server) = self.language_server_for_buffer(buffer, cx)
            {
                server.clone()
            } else {
                return Task::ready(Ok(Default::default()));
            };

            cx.spawn(|this, mut cx| async move {
                let resolved_completion = lang_server
                    .request::<lsp::request::ResolveCompletionItem>(completion.lsp_completion)
                    .await?;
                if let Some(edits) = resolved_completion.additional_text_edits {
                    let edits = this
                        .update(&mut cx, |this, cx| {
                            this.edits_from_lsp(&buffer_handle, edits, None, cx)
                        })
                        .await?;
                    buffer_handle.update(&mut cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();
                        for (range, text) in edits {
                            buffer.edit([(range, text)], cx);
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

    pub fn code_actions<T: Clone + ToOffset>(
        &self,
        buffer_handle: &ModelHandle<Buffer>,
        range: Range<T>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        let buffer_handle = buffer_handle.clone();
        let buffer = buffer_handle.read(cx);
        let snapshot = buffer.snapshot();
        let relevant_diagnostics = snapshot
            .diagnostics_in_range::<usize, usize>(range.to_offset(&snapshot), false)
            .map(|entry| entry.to_lsp_diagnostic_stub())
            .collect();
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
            let (_, lang_server) = if let Some(server) = self.language_server_for_buffer(buffer, cx)
            {
                server.clone()
            } else {
                return Task::ready(Ok(Default::default()));
            };

            let lsp_range = range_to_lsp(range.to_point_utf16(buffer));
            cx.foreground().spawn(async move {
                if !lang_server.capabilities().code_action_provider.is_some() {
                    return Ok(Default::default());
                }

                Ok(lang_server
                    .request::<lsp::request::CodeActionRequest>(lsp::CodeActionParams {
                        text_document: lsp::TextDocumentIdentifier::new(
                            lsp::Url::from_file_path(buffer_abs_path).unwrap(),
                        ),
                        range: lsp_range,
                        work_done_progress_params: Default::default(),
                        partial_result_params: Default::default(),
                        context: lsp::CodeActionContext {
                            diagnostics: relevant_diagnostics,
                            only: Some(vec![
                                lsp::CodeActionKind::QUICKFIX,
                                lsp::CodeActionKind::REFACTOR,
                                lsp::CodeActionKind::REFACTOR_EXTRACT,
                                lsp::CodeActionKind::SOURCE,
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
            let version = buffer.version();
            cx.spawn_weak(|_, mut cx| async move {
                let response = rpc
                    .request(proto::GetCodeActions {
                        project_id,
                        buffer_id,
                        start: Some(language::proto::serialize_anchor(&range.start)),
                        end: Some(language::proto::serialize_anchor(&range.end)),
                        version: serialize_version(&version),
                    })
                    .await?;

                buffer_handle
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(deserialize_version(response.version))
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
            let (lsp_adapter, lang_server) =
                if let Some(server) = self.language_server_for_buffer(buffer, cx) {
                    server.clone()
                } else {
                    return Task::ready(Ok(Default::default()));
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
                    *lsp_range = serde_json::to_value(&range_to_lsp(range)).unwrap();
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
                        lsp_adapter,
                        lang_server,
                        &mut cx,
                    )
                    .await
                } else if let Some(command) = action.lsp_action.command {
                    this.update(&mut cx, |this, _| {
                        this.last_workspace_edits_by_language_server
                            .remove(&lang_server.server_id());
                    });
                    lang_server
                        .request::<lsp::request::ExecuteCommand>(lsp::ExecuteCommandParams {
                            command: command.command,
                            arguments: command.arguments.unwrap_or_default(),
                            ..Default::default()
                        })
                        .await?;
                    Ok(this.update(&mut cx, |this, _| {
                        this.last_workspace_edits_by_language_server
                            .remove(&lang_server.server_id())
                            .unwrap_or_default()
                    }))
                } else {
                    Ok(ProjectTransaction::default())
                }
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
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
                    this.deserialize_project_transaction(response, push_to_history, cx)
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
        lsp_adapter: Arc<dyn LspAdapter>,
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
                                lsp_adapter.clone(),
                                language_server.clone(),
                                cx,
                            )
                        })
                        .await?;

                    let edits = this
                        .update(cx, |this, cx| {
                            let edits = op.edits.into_iter().map(|edit| match edit {
                                lsp::OneOf::Left(edit) => edit,
                                lsp::OneOf::Right(edit) => edit.text_edit,
                            });
                            this.edits_from_lsp(
                                &buffer_to_edit,
                                edits,
                                op.text_document.version,
                                cx,
                            )
                        })
                        .await?;

                    let transaction = buffer_to_edit.update(cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();
                        for (range, text) in edits {
                            buffer.edit([(range, text)], cx);
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
    ) -> Task<Result<HashMap<ModelHandle<Buffer>, Vec<Range<Anchor>>>>> {
        if self.is_local() {
            let snapshots = self
                .visible_worktrees(cx)
                .filter_map(|tree| {
                    let tree = tree.read(cx).as_local()?;
                    Some(tree.snapshot())
                })
                .collect::<Vec<_>>();

            let background = cx.background().clone();
            let path_count: usize = snapshots.iter().map(|s| s.visible_file_count()).sum();
            if path_count == 0 {
                return Task::ready(Ok(Default::default()));
            }
            let workers = background.num_cpus().min(path_count);
            let (matching_paths_tx, mut matching_paths_rx) = smol::channel::bounded(1024);
            cx.background()
                .spawn({
                    let fs = self.fs.clone();
                    let background = cx.background().clone();
                    let query = query.clone();
                    async move {
                        let fs = &fs;
                        let query = &query;
                        let matching_paths_tx = &matching_paths_tx;
                        let paths_per_worker = (path_count + workers - 1) / workers;
                        let snapshots = &snapshots;
                        background
                            .scoped(|scope| {
                                for worker_ix in 0..workers {
                                    let worker_start_ix = worker_ix * paths_per_worker;
                                    let worker_end_ix = worker_start_ix + paths_per_worker;
                                    scope.spawn(async move {
                                        let mut snapshot_start_ix = 0;
                                        let mut abs_path = PathBuf::new();
                                        for snapshot in snapshots {
                                            let snapshot_end_ix =
                                                snapshot_start_ix + snapshot.visible_file_count();
                                            if worker_end_ix <= snapshot_start_ix {
                                                break;
                                            } else if worker_start_ix > snapshot_end_ix {
                                                snapshot_start_ix = snapshot_end_ix;
                                                continue;
                                            } else {
                                                let start_in_snapshot = worker_start_ix
                                                    .saturating_sub(snapshot_start_ix);
                                                let end_in_snapshot =
                                                    cmp::min(worker_end_ix, snapshot_end_ix)
                                                        - snapshot_start_ix;

                                                for entry in snapshot
                                                    .files(false, start_in_snapshot)
                                                    .take(end_in_snapshot - start_in_snapshot)
                                                {
                                                    if matching_paths_tx.is_closed() {
                                                        break;
                                                    }

                                                    abs_path.clear();
                                                    abs_path.push(&snapshot.abs_path());
                                                    abs_path.push(&entry.path);
                                                    let matches = if let Some(file) =
                                                        fs.open_sync(&abs_path).await.log_err()
                                                    {
                                                        query.detect(file).unwrap_or(false)
                                                    } else {
                                                        false
                                                    };

                                                    if matches {
                                                        let project_path =
                                                            (snapshot.id(), entry.path.clone());
                                                        if matching_paths_tx
                                                            .send(project_path)
                                                            .await
                                                            .is_err()
                                                        {
                                                            break;
                                                        }
                                                    }
                                                }

                                                snapshot_start_ix = snapshot_end_ix;
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
            let open_buffers = self
                .opened_buffers
                .values()
                .filter_map(|b| b.upgrade(cx))
                .collect::<HashSet<_>>();
            cx.spawn(|this, cx| async move {
                for buffer in &open_buffers {
                    let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot());
                    buffers_tx.send((buffer.clone(), snapshot)).await?;
                }

                let open_buffers = Rc::new(RefCell::new(open_buffers));
                while let Some(project_path) = matching_paths_rx.next().await {
                    if buffers_tx.is_closed() {
                        break;
                    }

                    let this = this.clone();
                    let open_buffers = open_buffers.clone();
                    let buffers_tx = buffers_tx.clone();
                    cx.spawn(|mut cx| async move {
                        if let Some(buffer) = this
                            .update(&mut cx, |this, cx| this.open_buffer(project_path, cx))
                            .await
                            .log_err()
                        {
                            if open_buffers.borrow_mut().insert(buffer.clone()) {
                                let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot());
                                buffers_tx.send((buffer, snapshot)).await?;
                            }
                        }

                        Ok::<_, anyhow::Error>(())
                    })
                    .detach();
                }

                Ok::<_, anyhow::Error>(())
            })
            .detach_and_log_err(cx);

            let background = cx.background().clone();
            cx.background().spawn(async move {
                let query = &query;
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
                                    let buffer_matches = query
                                        .search(snapshot.as_rope())
                                        .await
                                        .iter()
                                        .map(|range| {
                                            snapshot.anchor_before(range.start)
                                                ..snapshot.anchor_after(range.end)
                                        })
                                        .collect::<Vec<_>>();
                                    if !buffer_matches.is_empty() {
                                        worker_matched_buffers
                                            .insert(buffer.clone(), buffer_matches);
                                    }
                                }
                            });
                        }
                    })
                    .await;
                Ok(matched_buffers.into_iter().flatten().collect())
            })
        } else if let Some(project_id) = self.remote_id() {
            let request = self.client.request(query.to_proto(project_id));
            cx.spawn(|this, mut cx| async move {
                let response = request.await?;
                let mut result = HashMap::default();
                for location in response.locations {
                    let buffer = location.buffer.ok_or_else(|| anyhow!("missing buffer"))?;
                    let target_buffer = this
                        .update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
                        .await?;
                    let start = location
                        .start
                        .and_then(deserialize_anchor)
                        .ok_or_else(|| anyhow!("missing target start"))?;
                    let end = location
                        .end
                        .and_then(deserialize_anchor)
                        .ok_or_else(|| anyhow!("missing target end"))?;
                    result
                        .entry(target_buffer)
                        .or_insert(Vec::new())
                        .push(start..end)
                }
                Ok(result)
            })
        } else {
            Task::ready(Ok(Default::default()))
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
            if let Some((file, (_, language_server))) =
                file.zip(self.language_server_for_buffer(buffer, cx).cloned())
            {
                let lsp_params = request.to_lsp(&file.abs_path(cx), cx);
                return cx.spawn(|this, cx| async move {
                    if !request.check_capabilities(&language_server.capabilities()) {
                        return Ok(Default::default());
                    }

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
            let message = request.to_proto(project_id, buffer);
            return cx.spawn(|this, cx| async move {
                let response = rpc.request(message).await?;
                request
                    .response_from_proto(response, this, buffer_handle, cx)
                    .await
            });
        }
        Task::ready(Ok(Default::default()))
    }

    pub fn find_or_create_local_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(ModelHandle<Worktree>, PathBuf)>> {
        let abs_path = abs_path.as_ref();
        if let Some((tree, relative_path)) = self.find_local_worktree(abs_path, cx) {
            Task::ready(Ok((tree.clone(), relative_path.into())))
        } else {
            let worktree = self.create_local_worktree(abs_path, visible, cx);
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
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Worktree>>> {
        let fs = self.fs.clone();
        let client = self.client.clone();
        let next_entry_id = self.next_entry_id.clone();
        let path: Arc<Path> = abs_path.as_ref().into();
        let task = self
            .loading_local_worktrees
            .entry(path.clone())
            .or_insert_with(|| {
                cx.spawn(|project, mut cx| {
                    async move {
                        let worktree = Worktree::local(
                            client.clone(),
                            path.clone(),
                            visible,
                            fs,
                            next_entry_id,
                            &mut cx,
                        )
                        .await;
                        project.update(&mut cx, |project, _| {
                            project.loading_local_worktrees.remove(&path);
                        });
                        let worktree = worktree?;

                        let project_id = project.update(&mut cx, |project, cx| {
                            project.add_worktree(&worktree, cx);
                            project.shared_remote_id()
                        });

                        if let Some(project_id) = project_id {
                            worktree
                                .update(&mut cx, |worktree, cx| {
                                    worktree.as_local_mut().unwrap().share(project_id, cx)
                                })
                                .await
                                .log_err();
                        }

                        Ok(worktree)
                    }
                    .map_err(|err| Arc::new(err))
                })
                .shared()
            })
            .clone();
        cx.foreground().spawn(async move {
            match task.await {
                Ok(worktree) => Ok(worktree),
                Err(err) => Err(anyhow!("{}", err)),
            }
        })
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut ModelContext<Self>) {
        self.worktrees.retain(|worktree| {
            if let Some(worktree) = worktree.upgrade(cx) {
                let id = worktree.read(cx).id();
                if id == id_to_remove {
                    cx.emit(Event::WorktreeRemoved(id));
                    false
                } else {
                    true
                }
            } else {
                false
            }
        });
        self.metadata_changed(cx);
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

        let push_strong_handle = {
            let worktree = worktree.read(cx);
            self.is_shared() || worktree.is_visible() || worktree.is_remote()
        };
        if push_strong_handle {
            self.worktrees
                .push(WorktreeHandle::Strong(worktree.clone()));
        } else {
            cx.observe_release(&worktree, |this, _, cx| {
                this.worktrees
                    .retain(|worktree| worktree.upgrade(cx).is_some());
                cx.notify();
            })
            .detach();
            self.worktrees
                .push(WorktreeHandle::Weak(worktree.downgrade()));
        }
        self.metadata_changed(cx);
        cx.emit(Event::WorktreeAdded);
        cx.notify();
    }

    fn update_local_worktree_buffers(
        &mut self,
        worktree_handle: ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();
        let mut buffers_to_delete = Vec::new();
        let mut renamed_buffers = Vec::new();
        for (buffer_id, buffer) in &self.opened_buffers {
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

                        let old_path = old_file.abs_path(cx);
                        if new_file.abs_path(cx) != old_path {
                            renamed_buffers.push((cx.handle(), old_path));
                        }

                        if let Some(project_id) = self.shared_remote_id() {
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
            self.opened_buffers.remove(&buffer_id);
        }

        for (buffer, old_path) in renamed_buffers {
            self.unregister_buffer_from_language_server(&buffer, old_path, cx);
            self.assign_language_to_buffer(&buffer, cx);
            self.register_buffer_with_language_server(&buffer, cx);
        }
    }

    pub fn set_active_path(&mut self, entry: Option<ProjectPath>, cx: &mut ModelContext<Self>) {
        let new_active_entry = entry.and_then(|project_path| {
            let worktree = self.worktree_for_id(project_path.worktree_id, cx)?;
            let entry = worktree.read(cx).entry_for_path(project_path.path)?;
            Some(entry.id)
        });
        if new_active_entry != self.active_entry {
            self.active_entry = new_active_entry;
            cx.emit(Event::ActiveEntryChanged(new_active_entry));
        }
    }

    pub fn is_running_disk_based_diagnostics(&self) -> bool {
        self.language_server_statuses
            .values()
            .any(|status| status.pending_diagnostic_updates > 0)
    }

    pub fn diagnostic_summary(&self, cx: &AppContext) -> DiagnosticSummary {
        let mut summary = DiagnosticSummary::default();
        for (_, path_summary) in self.diagnostic_summaries(cx) {
            summary.error_count += path_summary.error_count;
            summary.warning_count += path_summary.warning_count;
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
        if self
            .language_server_statuses
            .values()
            .map(|status| status.pending_diagnostic_updates)
            .sum::<isize>()
            == 1
        {
            cx.emit(Event::DiskBasedDiagnosticsStarted);
        }
    }

    pub fn disk_based_diagnostics_finished(&mut self, cx: &mut ModelContext<Self>) {
        cx.emit(Event::DiskBasedDiagnosticsUpdated);
        if self
            .language_server_statuses
            .values()
            .map(|status| status.pending_diagnostic_updates)
            .sum::<isize>()
            == 0
        {
            cx.emit(Event::DiskBasedDiagnosticsFinished);
        }
    }

    pub fn active_entry(&self) -> Option<ProjectEntryId> {
        self.active_entry
    }

    pub fn entry_for_path(&self, path: &ProjectPath, cx: &AppContext) -> Option<ProjectEntryId> {
        self.worktree_for_id(path.worktree_id, cx)?
            .read(cx)
            .entry_for_path(&path.path)
            .map(|entry| entry.id)
    }

    pub fn path_for_entry(&self, entry_id: ProjectEntryId, cx: &AppContext) -> Option<ProjectPath> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        let worktree = worktree.read(cx);
        let worktree_id = worktree.id();
        let path = worktree.entry_for_id(entry_id)?.path.clone();
        Some(ProjectPath { worktree_id, path })
    }

    // RPC message handlers

    async fn handle_request_join_project(
        this: ModelHandle<Self>,
        message: TypedEnvelope<proto::RequestJoinProject>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let user_id = message.payload.requester_id;
        if this.read_with(&cx, |project, _| {
            project.collaborators.values().any(|c| c.user.id == user_id)
        }) {
            this.update(&mut cx, |this, cx| {
                this.respond_to_join_request(user_id, true, cx)
            });
        } else {
            let user_store = this.read_with(&cx, |this, _| this.user_store.clone());
            let user = user_store
                .update(&mut cx, |store, cx| store.fetch_user(user_id, cx))
                .await?;
            this.update(&mut cx, |_, cx| cx.emit(Event::ContactRequestedJoin(user)));
        }
        Ok(())
    }

    async fn handle_unregister_project(
        this: ModelHandle<Self>,
        _: TypedEnvelope<proto::UnregisterProject>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| this.removed_from_project(cx));
        Ok(())
    }

    async fn handle_project_unshared(
        this: ModelHandle<Self>,
        _: TypedEnvelope<proto::ProjectUnshared>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| this.unshared(cx));
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
            for (_, buffer) in &this.opened_buffers {
                if let Some(buffer) = buffer.upgrade(cx) {
                    buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
                }
            }

            cx.emit(Event::CollaboratorLeft(peer_id));
            cx.notify();
            Ok(())
        })
    }

    async fn handle_join_project_request_cancelled(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::JoinProjectRequestCancelled>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let user = this
            .update(&mut cx, |this, cx| {
                this.user_store.update(cx, |user_store, cx| {
                    user_store.fetch_user(envelope.payload.requester_id, cx)
                })
            })
            .await?;

        this.update(&mut cx, |_, cx| {
            cx.emit(Event::ContactCancelledJoinRequest(user));
        });

        Ok(())
    }

    async fn handle_update_project(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateProject>,
        client: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let replica_id = this.replica_id();
            let remote_id = this.remote_id().ok_or_else(|| anyhow!("invalid project"))?;

            let mut old_worktrees_by_id = this
                .worktrees
                .drain(..)
                .filter_map(|worktree| {
                    let worktree = worktree.upgrade(cx)?;
                    Some((worktree.read(cx).id(), worktree))
                })
                .collect::<HashMap<_, _>>();

            for worktree in envelope.payload.worktrees {
                if let Some(old_worktree) =
                    old_worktrees_by_id.remove(&WorktreeId::from_proto(worktree.id))
                {
                    this.worktrees.push(WorktreeHandle::Strong(old_worktree));
                } else {
                    let worktree = proto::Worktree {
                        id: worktree.id,
                        root_name: worktree.root_name,
                        entries: Default::default(),
                        diagnostic_summaries: Default::default(),
                        visible: worktree.visible,
                        scan_id: 0,
                    };
                    let (worktree, load_task) =
                        Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx);
                    this.add_worktree(&worktree, cx);
                    load_task.detach();
                }
            }

            this.metadata_changed(cx);
            for (id, _) in old_worktrees_by_id {
                cx.emit(Event::WorktreeRemoved(id));
            }

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

    async fn handle_create_project_entry(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::CreateProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let worktree = this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            this.worktree_for_id(worktree_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })?;
        let worktree_scan_id = worktree.read_with(&cx, |worktree, _| worktree.scan_id());
        let entry = worktree
            .update(&mut cx, |worktree, cx| {
                let worktree = worktree.as_local_mut().unwrap();
                let path = PathBuf::from(OsString::from_vec(envelope.payload.path));
                worktree.create_entry(path, envelope.payload.is_directory, cx)
            })
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: Some((&entry).into()),
            worktree_scan_id: worktree_scan_id as u64,
        })
    }

    async fn handle_rename_project_entry(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::RenameProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.read_with(&cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })?;
        let worktree_scan_id = worktree.read_with(&cx, |worktree, _| worktree.scan_id());
        let entry = worktree
            .update(&mut cx, |worktree, cx| {
                let new_path = PathBuf::from(OsString::from_vec(envelope.payload.new_path));
                worktree
                    .as_local_mut()
                    .unwrap()
                    .rename_entry(entry_id, new_path, cx)
                    .ok_or_else(|| anyhow!("invalid entry"))
            })?
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: Some((&entry).into()),
            worktree_scan_id: worktree_scan_id as u64,
        })
    }

    async fn handle_copy_project_entry(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::CopyProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.read_with(&cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })?;
        let worktree_scan_id = worktree.read_with(&cx, |worktree, _| worktree.scan_id());
        let entry = worktree
            .update(&mut cx, |worktree, cx| {
                let new_path = PathBuf::from(OsString::from_vec(envelope.payload.new_path));
                worktree
                    .as_local_mut()
                    .unwrap()
                    .copy_entry(entry_id, new_path, cx)
                    .ok_or_else(|| anyhow!("invalid entry"))
            })?
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: Some((&entry).into()),
            worktree_scan_id: worktree_scan_id as u64,
        })
    }

    async fn handle_delete_project_entry(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::DeleteProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.read_with(&cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })?;
        let worktree_scan_id = worktree.read_with(&cx, |worktree, _| worktree.scan_id());
        worktree
            .update(&mut cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .delete_entry(entry_id, cx)
                    .ok_or_else(|| anyhow!("invalid entry"))
            })?
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: None,
            worktree_scan_id: worktree_scan_id as u64,
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

    async fn handle_start_language_server(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::StartLanguageServer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let server = envelope
            .payload
            .server
            .ok_or_else(|| anyhow!("invalid server"))?;
        this.update(&mut cx, |this, cx| {
            this.language_server_statuses.insert(
                server.id as usize,
                LanguageServerStatus {
                    name: server.name,
                    pending_work: Default::default(),
                    pending_diagnostic_updates: 0,
                },
            );
            cx.notify();
        });
        Ok(())
    }

    async fn handle_update_language_server(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateLanguageServer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let language_server_id = envelope.payload.language_server_id as usize;
        match envelope
            .payload
            .variant
            .ok_or_else(|| anyhow!("invalid variant"))?
        {
            proto::update_language_server::Variant::WorkStart(payload) => {
                this.update(&mut cx, |this, cx| {
                    this.on_lsp_work_start(language_server_id, payload.token, cx);
                })
            }
            proto::update_language_server::Variant::WorkProgress(payload) => {
                this.update(&mut cx, |this, cx| {
                    this.on_lsp_work_progress(
                        language_server_id,
                        payload.token,
                        LanguageServerProgress {
                            message: payload.message,
                            percentage: payload.percentage.map(|p| p as usize),
                            last_update_at: Instant::now(),
                        },
                        cx,
                    );
                })
            }
            proto::update_language_server::Variant::WorkEnd(payload) => {
                this.update(&mut cx, |this, cx| {
                    this.on_lsp_work_end(language_server_id, payload.token, cx);
                })
            }
            proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(_) => {
                this.update(&mut cx, |this, cx| {
                    this.disk_based_diagnostics_started(cx);
                })
            }
            proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(_) => {
                this.update(&mut cx, |this, cx| this.disk_based_diagnostics_finished(cx));
            }
        }

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
            match this.opened_buffers.entry(buffer_id) {
                hash_map::Entry::Occupied(mut e) => match e.get_mut() {
                    OpenBuffer::Strong(buffer) => {
                        buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
                    }
                    OpenBuffer::Loading(operations) => operations.extend_from_slice(&ops),
                    OpenBuffer::Weak(_) => {}
                },
                hash_map::Entry::Vacant(e) => {
                    assert!(
                        is_remote,
                        "received buffer update from {:?}",
                        envelope.original_sender_id
                    );
                    e.insert(OpenBuffer::Loading(ops));
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
                .opened_buffers
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
        let requested_version = deserialize_version(envelope.payload.version);

        let (project_id, buffer) = this.update(&mut cx, |this, cx| {
            let project_id = this.remote_id().ok_or_else(|| anyhow!("not connected"))?;
            let buffer = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade(cx))
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?;
            Ok::<_, anyhow::Error>((project_id, buffer))
        })?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(requested_version)
            })
            .await;

        let (saved_version, mtime) = buffer.update(&mut cx, |buffer, cx| buffer.save(cx)).await?;
        Ok(proto::BufferSaved {
            project_id,
            buffer_id,
            version: serialize_version(&saved_version),
            mtime: Some(mtime.into()),
        })
    }

    async fn handle_reload_buffers(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::ReloadBuffers>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ReloadBuffersResponse> {
        let sender_id = envelope.original_sender_id()?;
        let reload = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                buffers.insert(
                    this.opened_buffers
                        .get(buffer_id)
                        .and_then(|buffer| buffer.upgrade(cx))
                        .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?,
                );
            }
            Ok::<_, anyhow::Error>(this.reload_buffers(buffers, false, cx))
        })?;

        let project_transaction = reload.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::ReloadBuffersResponse {
            transaction: Some(project_transaction),
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
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                buffers.insert(
                    this.opened_buffers
                        .get(buffer_id)
                        .and_then(|buffer| buffer.upgrade(cx))
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
        let position = envelope
            .payload
            .position
            .and_then(language::proto::deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid position"))?;
        let version = deserialize_version(envelope.payload.version);
        let buffer = this.read_with(&cx, |this, cx| {
            this.opened_buffers
                .get(&envelope.payload.buffer_id)
                .and_then(|buffer| buffer.upgrade(cx))
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))
        })?;
        buffer
            .update(&mut cx, |buffer, _| buffer.wait_for_version(version))
            .await;
        let version = buffer.read_with(&cx, |buffer, _| buffer.version());
        let completions = this
            .update(&mut cx, |this, cx| this.completions(&buffer, position, cx))
            .await?;

        Ok(proto::GetCompletionsResponse {
            completions: completions
                .iter()
                .map(language::proto::serialize_completion)
                .collect(),
            version: serialize_version(&version),
        })
    }

    async fn handle_apply_additional_edits_for_completion(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::ApplyCompletionAdditionalEdits>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ApplyCompletionAdditionalEditsResponse> {
        let apply_additional_edits = this.update(&mut cx, |this, cx| {
            let buffer = this
                .opened_buffers
                .get(&envelope.payload.buffer_id)
                .and_then(|buffer| buffer.upgrade(cx))
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
        let buffer = this.update(&mut cx, |this, cx| {
            this.opened_buffers
                .get(&envelope.payload.buffer_id)
                .and_then(|buffer| buffer.upgrade(cx))
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))
        })?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(envelope.payload.version))
            })
            .await;

        let version = buffer.read_with(&cx, |buffer, _| buffer.version());
        let code_actions = this.update(&mut cx, |this, cx| {
            Ok::<_, anyhow::Error>(this.code_actions(&buffer, start..end, cx))
        })?;

        Ok(proto::GetCodeActionsResponse {
            actions: code_actions
                .await?
                .iter()
                .map(language::proto::serialize_code_action)
                .collect(),
            version: serialize_version(&version),
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
                .opened_buffers
                .get(&envelope.payload.buffer_id)
                .and_then(|buffer| buffer.upgrade(cx))
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
        let buffer_id = T::buffer_id_from_proto(&envelope.payload);
        let buffer_handle = this.read_with(&cx, |this, _| {
            this.opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade(&cx))
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))
        })?;
        let request = T::from_proto(
            envelope.payload,
            this.clone(),
            buffer_handle.clone(),
            cx.clone(),
        )
        .await?;
        let buffer_version = buffer_handle.read_with(&cx, |buffer, _| buffer.version());
        let response = this
            .update(&mut cx, |this, cx| {
                this.request_lsp(buffer_handle, request, cx)
            })
            .await?;
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

    async fn handle_search_project(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::SearchProject>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::SearchProjectResponse> {
        let peer_id = envelope.original_sender_id()?;
        let query = SearchQuery::from_proto(envelope.payload)?;
        let result = this
            .update(&mut cx, |this, cx| this.search(query, cx))
            .await?;

        this.update(&mut cx, |this, cx| {
            let mut locations = Vec::new();
            for (buffer, ranges) in result {
                for range in ranges {
                    let start = serialize_anchor(&range.start);
                    let end = serialize_anchor(&range.end);
                    let buffer = this.serialize_buffer_for_peer(&buffer, peer_id, cx);
                    locations.push(proto::Location {
                        buffer: Some(buffer),
                        start: Some(start),
                        end: Some(end),
                    });
                }
            }
            Ok(proto::SearchProjectResponse { locations })
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

    async fn handle_open_buffer_by_id(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::OpenBufferById>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferResponse> {
        let peer_id = envelope.original_sender_id()?;
        let buffer = this
            .update(&mut cx, |this, cx| {
                this.open_buffer_by_id(envelope.payload.id, cx)
            })
            .await?;
        this.update(&mut cx, |this, cx| {
            Ok(proto::OpenBufferResponse {
                buffer: Some(this.serialize_buffer_for_peer(&buffer, peer_id, cx)),
            })
        })
    }

    async fn handle_open_buffer_by_path(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::OpenBufferByPath>,
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
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        cx.spawn(|this, mut cx| async move {
            let mut project_transaction = ProjectTransaction::default();
            for (buffer, transaction) in message.buffers.into_iter().zip(message.transactions) {
                let buffer = this
                    .update(&mut cx, |this, cx| this.deserialize_buffer(buffer, cx))
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
        if shared_buffers.insert(buffer_id) {
            proto::Buffer {
                variant: Some(proto::buffer::Variant::State(buffer.read(cx).to_proto())),
            }
        } else {
            proto::Buffer {
                variant: Some(proto::buffer::Variant::Id(buffer_id)),
            }
        }
    }

    fn deserialize_buffer(
        &mut self,
        buffer: proto::Buffer,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let replica_id = self.replica_id();

        let opened_buffer_tx = self.opened_buffer.0.clone();
        let mut opened_buffer_rx = self.opened_buffer.1.clone();
        cx.spawn(|this, mut cx| async move {
            match buffer.variant.ok_or_else(|| anyhow!("missing buffer"))? {
                proto::buffer::Variant::Id(id) => {
                    let buffer = loop {
                        let buffer = this.read_with(&cx, |this, cx| {
                            this.opened_buffers
                                .get(&id)
                                .and_then(|buffer| buffer.upgrade(cx))
                        });
                        if let Some(buffer) = buffer {
                            break buffer;
                        }
                        opened_buffer_rx
                            .next()
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

                    this.update(&mut cx, |this, cx| this.register_buffer(&buffer, cx))?;

                    *opened_buffer_tx.borrow_mut().borrow_mut() = ();
                    Ok(buffer)
                }
            }
        })
    }

    fn deserialize_symbol(&self, serialized_symbol: proto::Symbol) -> Result<Symbol> {
        let source_worktree_id = WorktreeId::from_proto(serialized_symbol.source_worktree_id);
        let worktree_id = WorktreeId::from_proto(serialized_symbol.worktree_id);
        let start = serialized_symbol
            .start
            .ok_or_else(|| anyhow!("invalid start"))?;
        let end = serialized_symbol
            .end
            .ok_or_else(|| anyhow!("invalid end"))?;
        let kind = unsafe { mem::transmute(serialized_symbol.kind) };
        let path = PathBuf::from(serialized_symbol.path);
        let language = self.languages.select_language(&path);
        Ok(Symbol {
            source_worktree_id,
            worktree_id,
            language_server_name: LanguageServerName(serialized_symbol.language_server_name.into()),
            label: language
                .and_then(|language| language.label_for_symbol(&serialized_symbol.name, kind))
                .unwrap_or_else(|| CodeLabel::plain(serialized_symbol.name.clone(), None)),
            name: serialized_symbol.name,
            path,
            range: PointUtf16::new(start.row, start.column)..PointUtf16::new(end.row, end.column),
            kind,
            signature: serialized_symbol
                .signature
                .try_into()
                .map_err(|_| anyhow!("invalid signature"))?,
        })
    }

    async fn handle_buffer_saved(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::BufferSaved>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let version = deserialize_version(envelope.payload.version);
        let mtime = envelope
            .payload
            .mtime
            .ok_or_else(|| anyhow!("missing mtime"))?
            .into();

        this.update(&mut cx, |this, cx| {
            let buffer = this
                .opened_buffers
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
        let version = deserialize_version(payload.version);
        let mtime = payload
            .mtime
            .ok_or_else(|| anyhow!("missing mtime"))?
            .into();
        this.update(&mut cx, |this, cx| {
            let buffer = this
                .opened_buffers
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
            .filter(|worktree| worktree.read(cx).is_visible())
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

    fn edits_from_lsp(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        lsp_edits: impl 'static + Send + IntoIterator<Item = lsp::TextEdit>,
        version: Option<i32>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<(Range<Anchor>, String)>>> {
        let snapshot = self.buffer_snapshot_for_lsp_version(buffer, version, cx);
        cx.background().spawn(async move {
            let snapshot = snapshot?;
            let mut lsp_edits = lsp_edits
                .into_iter()
                .map(|edit| (range_from_lsp(edit.range), edit.new_text))
                .peekable();

            let mut edits = Vec::new();
            while let Some((mut range, mut new_text)) = lsp_edits.next() {
                // Combine any LSP edits that are adjacent.
                //
                // Also, combine LSP edits that are separated from each other by only
                // a newline. This is important because for some code actions,
                // Rust-analyzer rewrites the entire buffer via a series of edits that
                // are separated by unchanged newline characters.
                //
                // In order for the diffing logic below to work properly, any edits that
                // cancel each other out must be combined into one.
                while let Some((next_range, next_text)) = lsp_edits.peek() {
                    if next_range.start > range.end {
                        if next_range.start.row > range.end.row + 1
                            || next_range.start.column > 0
                            || snapshot.clip_point_utf16(
                                PointUtf16::new(range.end.row, u32::MAX),
                                Bias::Left,
                            ) > range.end
                        {
                            break;
                        }
                        new_text.push('\n');
                    }
                    range.end = next_range.end;
                    new_text.push_str(&next_text);
                    lsp_edits.next();
                }

                if snapshot.clip_point_utf16(range.start, Bias::Left) != range.start
                    || snapshot.clip_point_utf16(range.end, Bias::Left) != range.end
                {
                    return Err(anyhow!("invalid edits received from language server"));
                }

                // For multiline edits, perform a diff of the old and new text so that
                // we can identify the changes more precisely, preserving the locations
                // of any anchors positioned in the unchanged regions.
                if range.end.row > range.start.row {
                    let mut offset = range.start.to_offset(&snapshot);
                    let old_text = snapshot.text_for_range(range).collect::<String>();

                    let diff = TextDiff::from_lines(old_text.as_str(), &new_text);
                    let mut moved_since_edit = true;
                    for change in diff.iter_all_changes() {
                        let tag = change.tag();
                        let value = change.value();
                        match tag {
                            ChangeTag::Equal => {
                                offset += value.len();
                                moved_since_edit = true;
                            }
                            ChangeTag::Delete => {
                                let start = snapshot.anchor_after(offset);
                                let end = snapshot.anchor_before(offset + value.len());
                                if moved_since_edit {
                                    edits.push((start..end, String::new()));
                                } else {
                                    edits.last_mut().unwrap().0.end = end;
                                }
                                offset += value.len();
                                moved_since_edit = false;
                            }
                            ChangeTag::Insert => {
                                if moved_since_edit {
                                    let anchor = snapshot.anchor_after(offset);
                                    edits.push((anchor.clone()..anchor, value.to_string()));
                                } else {
                                    edits.last_mut().unwrap().1.push_str(value);
                                }
                                moved_since_edit = false;
                            }
                        }
                    }
                } else if range.end == range.start {
                    let anchor = snapshot.anchor_after(range.start);
                    edits.push((anchor.clone()..anchor, new_text));
                } else {
                    let edit_start = snapshot.anchor_after(range.start);
                    let edit_end = snapshot.anchor_before(range.end);
                    edits.push((edit_start..edit_end, new_text));
                }
            }

            Ok(edits)
        })
    }

    fn buffer_snapshot_for_lsp_version(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        version: Option<i32>,
        cx: &AppContext,
    ) -> Result<TextBufferSnapshot> {
        const OLD_VERSIONS_TO_RETAIN: i32 = 10;

        if let Some(version) = version {
            let buffer_id = buffer.read(cx).remote_id();
            let snapshots = self
                .buffer_snapshots
                .get_mut(&buffer_id)
                .ok_or_else(|| anyhow!("no snapshot found for buffer {}", buffer_id))?;
            let mut found_snapshot = None;
            snapshots.retain(|(snapshot_version, snapshot)| {
                if snapshot_version + OLD_VERSIONS_TO_RETAIN < version {
                    false
                } else {
                    if *snapshot_version == version {
                        found_snapshot = Some(snapshot.clone());
                    }
                    true
                }
            });

            found_snapshot.ok_or_else(|| {
                anyhow!(
                    "snapshot not found for buffer {} at version {}",
                    buffer_id,
                    version
                )
            })
        } else {
            Ok((buffer.read(cx)).text_snapshot())
        }
    }

    fn language_server_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Option<&(Arc<dyn LspAdapter>, Arc<LanguageServer>)> {
        if let Some((file, language)) = File::from_dyn(buffer.file()).zip(buffer.language()) {
            let worktree_id = file.worktree_id(cx);
            self.language_servers
                .get(&(worktree_id, language.lsp_adapter()?.name()))
        } else {
            None
        }
    }
}

impl ProjectStore {
    pub fn projects<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = ModelHandle<Project>> {
        self.projects
            .iter()
            .filter_map(|project| project.upgrade(cx))
    }

    fn add_project(&mut self, project: WeakModelHandle<Project>, cx: &mut ModelContext<Self>) {
        if let Err(ix) = self
            .projects
            .binary_search_by_key(&project.id(), WeakModelHandle::id)
        {
            self.projects.insert(ix, project);
        }
        cx.notify();
    }

    fn prune_projects(&mut self, cx: &mut ModelContext<Self>) {
        let mut did_change = false;
        self.projects.retain(|project| {
            if project.is_upgradable(cx) {
                true
            } else {
                did_change = true;
                false
            }
        });
        if did_change {
            cx.notify();
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
            OpenBuffer::Strong(handle) => Some(handle.clone()),
            OpenBuffer::Weak(handle) => handle.upgrade(cx),
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

impl Entity for ProjectStore {
    type Event = ();
}

impl Entity for Project {
    type Event = Event;

    fn release(&mut self, cx: &mut gpui::MutableAppContext) {
        self.project_store.update(cx, ProjectStore::prune_projects);

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
            .filter_map(|(_, (_, server))| server.shutdown())
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
        language_server_name: symbol.language_server_name.0.to_string(),
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

impl Item for Buffer {
    fn entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId> {
        File::from_dyn(self.file()).and_then(|file| file.project_entry_id(cx))
    }
}

#[cfg(test)]
mod tests {
    use crate::worktree::WorktreeHandle;

    use super::{Event, *};
    use fs::RealFs;
    use futures::{future, StreamExt};
    use gpui::test::subscribe;
    use language::{
        tree_sitter_rust, tree_sitter_typescript, Diagnostic, FakeLspAdapter, LanguageConfig,
        OffsetRangeExt, Point, ToPoint,
    };
    use lsp::Url;
    use serde_json::json;
    use std::{cell::RefCell, os::unix, path::PathBuf, rc::Rc, task::Poll};
    use unindent::Unindent as _;
    use util::{assert_set_eq, test::temp_tree};

    #[gpui::test]
    async fn test_populate_and_search(cx: &mut gpui::TestAppContext) {
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

        let project = Project::test(Arc::new(RealFs), [root_link_path.as_ref()], cx).await;

        project.read_with(cx, |project, cx| {
            let tree = project.worktrees(cx).next().unwrap().read(cx);
            assert_eq!(tree.file_count(), 5);
            assert_eq!(
                tree.inode_for_path("fennel/grape"),
                tree.inode_for_path("finnochio/grape")
            );
        });

        let cancel_flag = Default::default();
        let results = project
            .read_with(cx, |project, cx| {
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
    async fn test_managing_language_servers(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let mut rust_language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut json_language = Language::new(
            LanguageConfig {
                name: "JSON".into(),
                path_suffixes: vec!["json".to_string()],
                ..Default::default()
            },
            None,
        );
        let mut fake_rust_servers = rust_language.set_fake_lsp_adapter(FakeLspAdapter {
            name: "the-rust-language-server",
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), "::".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        });
        let mut fake_json_servers = json_language.set_fake_lsp_adapter(FakeLspAdapter {
            name: "the-json-language-server",
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        });

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/the-root",
            json!({
                "test.rs": "const A: i32 = 1;",
                "test2.rs": "",
                "Cargo.toml": "a = 1",
                "package.json": "{\"a\": 1}",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;
        project.update(cx, |project, _| {
            project.languages.add(Arc::new(rust_language));
            project.languages.add(Arc::new(json_language));
        });

        // Open a buffer without an associated language server.
        let toml_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/the-root/Cargo.toml", cx)
            })
            .await
            .unwrap();

        // Open a buffer with an associated language server.
        let rust_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/the-root/test.rs", cx)
            })
            .await
            .unwrap();

        // A server is started up, and it is notified about Rust files.
        let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
        assert_eq!(
            fake_rust_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path("/the-root/test.rs").unwrap(),
                version: 0,
                text: "const A: i32 = 1;".to_string(),
                language_id: Default::default()
            }
        );

        // The buffer is configured based on the language server's capabilities.
        rust_buffer.read_with(cx, |buffer, _| {
            assert_eq!(
                buffer.completion_triggers(),
                &[".".to_string(), "::".to_string()]
            );
        });
        toml_buffer.read_with(cx, |buffer, _| {
            assert!(buffer.completion_triggers().is_empty());
        });

        // Edit a buffer. The changes are reported to the language server.
        rust_buffer.update(cx, |buffer, cx| buffer.edit([(16..16, "2")], cx));
        assert_eq!(
            fake_rust_server
                .receive_notification::<lsp::notification::DidChangeTextDocument>()
                .await
                .text_document,
            lsp::VersionedTextDocumentIdentifier::new(
                lsp::Url::from_file_path("/the-root/test.rs").unwrap(),
                1
            )
        );

        // Open a third buffer with a different associated language server.
        let json_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/the-root/package.json", cx)
            })
            .await
            .unwrap();

        // A json language server is started up and is only notified about the json buffer.
        let mut fake_json_server = fake_json_servers.next().await.unwrap();
        assert_eq!(
            fake_json_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path("/the-root/package.json").unwrap(),
                version: 0,
                text: "{\"a\": 1}".to_string(),
                language_id: Default::default()
            }
        );

        // This buffer is configured based on the second language server's
        // capabilities.
        json_buffer.read_with(cx, |buffer, _| {
            assert_eq!(buffer.completion_triggers(), &[":".to_string()]);
        });

        // When opening another buffer whose language server is already running,
        // it is also configured based on the existing language server's capabilities.
        let rust_buffer2 = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/the-root/test2.rs", cx)
            })
            .await
            .unwrap();
        rust_buffer2.read_with(cx, |buffer, _| {
            assert_eq!(
                buffer.completion_triggers(),
                &[".".to_string(), "::".to_string()]
            );
        });

        // Changes are reported only to servers matching the buffer's language.
        toml_buffer.update(cx, |buffer, cx| buffer.edit([(5..5, "23")], cx));
        rust_buffer2.update(cx, |buffer, cx| buffer.edit([(0..0, "let x = 1;")], cx));
        assert_eq!(
            fake_rust_server
                .receive_notification::<lsp::notification::DidChangeTextDocument>()
                .await
                .text_document,
            lsp::VersionedTextDocumentIdentifier::new(
                lsp::Url::from_file_path("/the-root/test2.rs").unwrap(),
                1
            )
        );

        // Save notifications are reported to all servers.
        toml_buffer
            .update(cx, |buffer, cx| buffer.save(cx))
            .await
            .unwrap();
        assert_eq!(
            fake_rust_server
                .receive_notification::<lsp::notification::DidSaveTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentIdentifier::new(
                lsp::Url::from_file_path("/the-root/Cargo.toml").unwrap()
            )
        );
        assert_eq!(
            fake_json_server
                .receive_notification::<lsp::notification::DidSaveTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentIdentifier::new(
                lsp::Url::from_file_path("/the-root/Cargo.toml").unwrap()
            )
        );

        // Renames are reported only to servers matching the buffer's language.
        fs.rename(
            Path::new("/the-root/test2.rs"),
            Path::new("/the-root/test3.rs"),
            Default::default(),
        )
        .await
        .unwrap();
        assert_eq!(
            fake_rust_server
                .receive_notification::<lsp::notification::DidCloseTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentIdentifier::new(
                lsp::Url::from_file_path("/the-root/test2.rs").unwrap()
            ),
        );
        assert_eq!(
            fake_rust_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path("/the-root/test3.rs").unwrap(),
                version: 0,
                text: rust_buffer2.read_with(cx, |buffer, _| buffer.text()),
                language_id: Default::default()
            },
        );

        rust_buffer2.update(cx, |buffer, cx| {
            buffer.update_diagnostics(
                DiagnosticSet::from_sorted_entries(
                    vec![DiagnosticEntry {
                        diagnostic: Default::default(),
                        range: Anchor::MIN..Anchor::MAX,
                    }],
                    &buffer.snapshot(),
                ),
                cx,
            );
            assert_eq!(
                buffer
                    .snapshot()
                    .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                    .count(),
                1
            );
        });

        // When the rename changes the extension of the file, the buffer gets closed on the old
        // language server and gets opened on the new one.
        fs.rename(
            Path::new("/the-root/test3.rs"),
            Path::new("/the-root/test3.json"),
            Default::default(),
        )
        .await
        .unwrap();
        assert_eq!(
            fake_rust_server
                .receive_notification::<lsp::notification::DidCloseTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentIdentifier::new(
                lsp::Url::from_file_path("/the-root/test3.rs").unwrap(),
            ),
        );
        assert_eq!(
            fake_json_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path("/the-root/test3.json").unwrap(),
                version: 0,
                text: rust_buffer2.read_with(cx, |buffer, _| buffer.text()),
                language_id: Default::default()
            },
        );

        // We clear the diagnostics, since the language has changed.
        rust_buffer2.read_with(cx, |buffer, _| {
            assert_eq!(
                buffer
                    .snapshot()
                    .diagnostics_in_range::<_, usize>(0..buffer.len(), false)
                    .count(),
                0
            );
        });

        // The renamed file's version resets after changing language server.
        rust_buffer2.update(cx, |buffer, cx| buffer.edit([(0..0, "// ")], cx));
        assert_eq!(
            fake_json_server
                .receive_notification::<lsp::notification::DidChangeTextDocument>()
                .await
                .text_document,
            lsp::VersionedTextDocumentIdentifier::new(
                lsp::Url::from_file_path("/the-root/test3.json").unwrap(),
                1
            )
        );

        // Restart language servers
        project.update(cx, |project, cx| {
            project.restart_language_servers_for_buffers(
                vec![rust_buffer.clone(), json_buffer.clone()],
                cx,
            );
        });

        let mut rust_shutdown_requests = fake_rust_server
            .handle_request::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
        let mut json_shutdown_requests = fake_json_server
            .handle_request::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
        futures::join!(rust_shutdown_requests.next(), json_shutdown_requests.next());

        let mut fake_rust_server = fake_rust_servers.next().await.unwrap();
        let mut fake_json_server = fake_json_servers.next().await.unwrap();

        // Ensure rust document is reopened in new rust language server
        assert_eq!(
            fake_rust_server
                .receive_notification::<lsp::notification::DidOpenTextDocument>()
                .await
                .text_document,
            lsp::TextDocumentItem {
                uri: lsp::Url::from_file_path("/the-root/test.rs").unwrap(),
                version: 1,
                text: rust_buffer.read_with(cx, |buffer, _| buffer.text()),
                language_id: Default::default()
            }
        );

        // Ensure json documents are reopened in new json language server
        assert_set_eq!(
            [
                fake_json_server
                    .receive_notification::<lsp::notification::DidOpenTextDocument>()
                    .await
                    .text_document,
                fake_json_server
                    .receive_notification::<lsp::notification::DidOpenTextDocument>()
                    .await
                    .text_document,
            ],
            [
                lsp::TextDocumentItem {
                    uri: lsp::Url::from_file_path("/the-root/package.json").unwrap(),
                    version: 0,
                    text: json_buffer.read_with(cx, |buffer, _| buffer.text()),
                    language_id: Default::default()
                },
                lsp::TextDocumentItem {
                    uri: lsp::Url::from_file_path("/the-root/test3.json").unwrap(),
                    version: 1,
                    text: rust_buffer2.read_with(cx, |buffer, _| buffer.text()),
                    language_id: Default::default()
                }
            ]
        );

        // Close notifications are reported only to servers matching the buffer's language.
        cx.update(|_| drop(json_buffer));
        let close_message = lsp::DidCloseTextDocumentParams {
            text_document: lsp::TextDocumentIdentifier::new(
                lsp::Url::from_file_path("/the-root/package.json").unwrap(),
            ),
        };
        assert_eq!(
            fake_json_server
                .receive_notification::<lsp::notification::DidCloseTextDocument>()
                .await,
            close_message,
        );
    }

    #[gpui::test]
    async fn test_single_file_worktrees_diagnostics(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.rs": "let a = 1;",
                "b.rs": "let b = 2;"
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir/a.rs".as_ref(), "/dir/b.rs".as_ref()], cx).await;

        let buffer_a = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
            .await
            .unwrap();
        let buffer_b = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/b.rs", cx))
            .await
            .unwrap();

        project.update(cx, |project, cx| {
            project
                .update_diagnostics(
                    lsp::PublishDiagnosticsParams {
                        uri: Url::from_file_path("/dir/a.rs").unwrap(),
                        version: None,
                        diagnostics: vec![lsp::Diagnostic {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 4),
                                lsp::Position::new(0, 5),
                            ),
                            severity: Some(lsp::DiagnosticSeverity::ERROR),
                            message: "error 1".to_string(),
                            ..Default::default()
                        }],
                    },
                    &[],
                    cx,
                )
                .unwrap();
            project
                .update_diagnostics(
                    lsp::PublishDiagnosticsParams {
                        uri: Url::from_file_path("/dir/b.rs").unwrap(),
                        version: None,
                        diagnostics: vec![lsp::Diagnostic {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 4),
                                lsp::Position::new(0, 5),
                            ),
                            severity: Some(lsp::DiagnosticSeverity::WARNING),
                            message: "error 2".to_string(),
                            ..Default::default()
                        }],
                    },
                    &[],
                    cx,
                )
                .unwrap();
        });

        buffer_a.read_with(cx, |buffer, _| {
            let chunks = chunks_with_diagnostics(&buffer, 0..buffer.len());
            assert_eq!(
                chunks
                    .iter()
                    .map(|(s, d)| (s.as_str(), *d))
                    .collect::<Vec<_>>(),
                &[
                    ("let ", None),
                    ("a", Some(DiagnosticSeverity::ERROR)),
                    (" = 1;", None),
                ]
            );
        });
        buffer_b.read_with(cx, |buffer, _| {
            let chunks = chunks_with_diagnostics(&buffer, 0..buffer.len());
            assert_eq!(
                chunks
                    .iter()
                    .map(|(s, d)| (s.as_str(), *d))
                    .collect::<Vec<_>>(),
                &[
                    ("let ", None),
                    ("b", Some(DiagnosticSeverity::WARNING)),
                    (" = 2;", None),
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_disk_based_diagnostics_progress(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let progress_token = "the-progress-token";
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            disk_based_diagnostics_progress_token: Some(progress_token),
            disk_based_diagnostics_sources: &["disk"],
            ..Default::default()
        });

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.rs": "fn a() { A }",
                "b.rs": "const y: i32 = 1",
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages.add(Arc::new(language)));
        let worktree_id =
            project.read_with(cx, |p, cx| p.worktrees(cx).next().unwrap().read(cx).id());

        // Cause worktree to start the fake language server
        let _buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/b.rs", cx))
            .await
            .unwrap();

        let mut events = subscribe(&project, cx);

        let mut fake_server = fake_servers.next().await.unwrap();
        fake_server.start_progress(progress_token).await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsStarted
        );

        fake_server.start_progress(progress_token).await;
        fake_server.end_progress(progress_token).await;
        fake_server.start_progress(progress_token).await;

        fake_server.notify::<lsp::notification::PublishDiagnostics>(
            lsp::PublishDiagnosticsParams {
                uri: Url::from_file_path("/dir/a.rs").unwrap(),
                version: None,
                diagnostics: vec![lsp::Diagnostic {
                    range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    message: "undefined variable 'A'".to_string(),
                    ..Default::default()
                }],
            },
        );
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiagnosticsUpdated((worktree_id, Path::new("a.rs")).into())
        );

        fake_server.end_progress(progress_token).await;
        fake_server.end_progress(progress_token).await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsUpdated
        );
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsFinished
        );

        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/a.rs", cx))
            .await
            .unwrap();

        buffer.read_with(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            let diagnostics = snapshot
                .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
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

        // Ensure publishing empty diagnostics twice only results in one update event.
        fake_server.notify::<lsp::notification::PublishDiagnostics>(
            lsp::PublishDiagnosticsParams {
                uri: Url::from_file_path("/dir/a.rs").unwrap(),
                version: None,
                diagnostics: Default::default(),
            },
        );
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiagnosticsUpdated((worktree_id, Path::new("a.rs")).into())
        );

        fake_server.notify::<lsp::notification::PublishDiagnostics>(
            lsp::PublishDiagnosticsParams {
                uri: Url::from_file_path("/dir/a.rs").unwrap(),
                version: None,
                diagnostics: Default::default(),
            },
        );
        cx.foreground().run_until_parked();
        assert_eq!(futures::poll!(events.next()), Poll::Pending);
    }

    #[gpui::test]
    async fn test_restarting_server_with_diagnostics_running(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let progress_token = "the-progress-token";
        let mut language = Language::new(
            LanguageConfig {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            None,
        );
        let mut fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            disk_based_diagnostics_sources: &["disk"],
            disk_based_diagnostics_progress_token: Some(progress_token),
            ..Default::default()
        });

        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/dir", json!({ "a.rs": "" })).await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages.add(Arc::new(language)));

        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
            .await
            .unwrap();

        // Simulate diagnostics starting to update.
        let mut fake_server = fake_servers.next().await.unwrap();
        fake_server.start_progress(progress_token).await;

        // Restart the server before the diagnostics finish updating.
        project.update(cx, |project, cx| {
            project.restart_language_servers_for_buffers([buffer], cx);
        });
        let mut events = subscribe(&project, cx);

        // Simulate the newly started server sending more diagnostics.
        let mut fake_server = fake_servers.next().await.unwrap();
        fake_server.start_progress(progress_token).await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsStarted
        );

        // All diagnostics are considered done, despite the old server's diagnostic
        // task never completing.
        fake_server.end_progress(progress_token).await;
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsUpdated
        );
        assert_eq!(
            events.next().await.unwrap(),
            Event::DiskBasedDiagnosticsFinished
        );
        project.read_with(cx, |project, _| {
            assert!(!project.is_running_disk_based_diagnostics());
        });
    }

    #[gpui::test]
    async fn test_transforming_diagnostics(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            disk_based_diagnostics_sources: &["disk"],
            ..Default::default()
        });

        let text = "
            fn a() { A }
            fn b() { BB }
            fn c() { CCC }
        "
        .unindent();

        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/dir", json!({ "a.rs": text })).await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages.add(Arc::new(language)));

        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
            .await
            .unwrap();

        let mut fake_server = fake_servers.next().await.unwrap();
        let open_notification = fake_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;

        // Edit the buffer, moving the content down
        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "\n\n")], cx));
        let change_notification_1 = fake_server
            .receive_notification::<lsp::notification::DidChangeTextDocument>()
            .await;
        assert!(
            change_notification_1.text_document.version > open_notification.text_document.version
        );

        // Report some diagnostics for the initial version of the buffer
        fake_server.notify::<lsp::notification::PublishDiagnostics>(
            lsp::PublishDiagnosticsParams {
                uri: lsp::Url::from_file_path("/dir/a.rs").unwrap(),
                version: Some(open_notification.text_document.version),
                diagnostics: vec![
                    lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: "undefined variable 'A'".to_string(),
                        source: Some("disk".to_string()),
                        ..Default::default()
                    },
                    lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(1, 9), lsp::Position::new(1, 11)),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: "undefined variable 'BB'".to_string(),
                        source: Some("disk".to_string()),
                        ..Default::default()
                    },
                    lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(2, 9), lsp::Position::new(2, 12)),
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("disk".to_string()),
                        message: "undefined variable 'CCC'".to_string(),
                        ..Default::default()
                    },
                ],
            },
        );

        // The diagnostics have moved down since they were created.
        buffer.next_notification(cx).await;
        buffer.read_with(cx, |buffer, _| {
            assert_eq!(
                buffer
                    .snapshot()
                    .diagnostics_in_range::<_, Point>(Point::new(3, 0)..Point::new(5, 0), false)
                    .collect::<Vec<_>>(),
                &[
                    DiagnosticEntry {
                        range: Point::new(3, 9)..Point::new(3, 11),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'BB'".to_string(),
                            is_disk_based: true,
                            group_id: 1,
                            is_primary: true,
                            ..Default::default()
                        },
                    },
                    DiagnosticEntry {
                        range: Point::new(4, 9)..Point::new(4, 12),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'CCC'".to_string(),
                            is_disk_based: true,
                            group_id: 2,
                            is_primary: true,
                            ..Default::default()
                        }
                    }
                ]
            );
            assert_eq!(
                chunks_with_diagnostics(buffer, 0..buffer.len()),
                [
                    ("\n\nfn a() { ".to_string(), None),
                    ("A".to_string(), Some(DiagnosticSeverity::ERROR)),
                    (" }\nfn b() { ".to_string(), None),
                    ("BB".to_string(), Some(DiagnosticSeverity::ERROR)),
                    (" }\nfn c() { ".to_string(), None),
                    ("CCC".to_string(), Some(DiagnosticSeverity::ERROR)),
                    (" }\n".to_string(), None),
                ]
            );
            assert_eq!(
                chunks_with_diagnostics(buffer, Point::new(3, 10)..Point::new(4, 11)),
                [
                    ("B".to_string(), Some(DiagnosticSeverity::ERROR)),
                    (" }\nfn c() { ".to_string(), None),
                    ("CC".to_string(), Some(DiagnosticSeverity::ERROR)),
                ]
            );
        });

        // Ensure overlapping diagnostics are highlighted correctly.
        fake_server.notify::<lsp::notification::PublishDiagnostics>(
            lsp::PublishDiagnosticsParams {
                uri: lsp::Url::from_file_path("/dir/a.rs").unwrap(),
                version: Some(open_notification.text_document.version),
                diagnostics: vec![
                    lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: "undefined variable 'A'".to_string(),
                        source: Some("disk".to_string()),
                        ..Default::default()
                    },
                    lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 12)),
                        severity: Some(DiagnosticSeverity::WARNING),
                        message: "unreachable statement".to_string(),
                        source: Some("disk".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );

        buffer.next_notification(cx).await;
        buffer.read_with(cx, |buffer, _| {
            assert_eq!(
                buffer
                    .snapshot()
                    .diagnostics_in_range::<_, Point>(Point::new(2, 0)..Point::new(3, 0), false)
                    .collect::<Vec<_>>(),
                &[
                    DiagnosticEntry {
                        range: Point::new(2, 9)..Point::new(2, 12),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::WARNING,
                            message: "unreachable statement".to_string(),
                            is_disk_based: true,
                            group_id: 4,
                            is_primary: true,
                            ..Default::default()
                        }
                    },
                    DiagnosticEntry {
                        range: Point::new(2, 9)..Point::new(2, 10),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'A'".to_string(),
                            is_disk_based: true,
                            group_id: 3,
                            is_primary: true,
                            ..Default::default()
                        },
                    }
                ]
            );
            assert_eq!(
                chunks_with_diagnostics(buffer, Point::new(2, 0)..Point::new(3, 0)),
                [
                    ("fn a() { ".to_string(), None),
                    ("A".to_string(), Some(DiagnosticSeverity::ERROR)),
                    (" }".to_string(), Some(DiagnosticSeverity::WARNING)),
                    ("\n".to_string(), None),
                ]
            );
            assert_eq!(
                chunks_with_diagnostics(buffer, Point::new(2, 10)..Point::new(3, 0)),
                [
                    (" }".to_string(), Some(DiagnosticSeverity::WARNING)),
                    ("\n".to_string(), None),
                ]
            );
        });

        // Keep editing the buffer and ensure disk-based diagnostics get translated according to the
        // changes since the last save.
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "    ")], cx);
            buffer.edit([(Point::new(2, 8)..Point::new(2, 10), "(x: usize)")], cx);
            buffer.edit([(Point::new(3, 10)..Point::new(3, 10), "xxx")], cx);
        });
        let change_notification_2 = fake_server
            .receive_notification::<lsp::notification::DidChangeTextDocument>()
            .await;
        assert!(
            change_notification_2.text_document.version
                > change_notification_1.text_document.version
        );

        // Handle out-of-order diagnostics
        fake_server.notify::<lsp::notification::PublishDiagnostics>(
            lsp::PublishDiagnosticsParams {
                uri: lsp::Url::from_file_path("/dir/a.rs").unwrap(),
                version: Some(change_notification_2.text_document.version),
                diagnostics: vec![
                    lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(1, 9), lsp::Position::new(1, 11)),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: "undefined variable 'BB'".to_string(),
                        source: Some("disk".to_string()),
                        ..Default::default()
                    },
                    lsp::Diagnostic {
                        range: lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                        severity: Some(DiagnosticSeverity::WARNING),
                        message: "undefined variable 'A'".to_string(),
                        source: Some("disk".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );

        buffer.next_notification(cx).await;
        buffer.read_with(cx, |buffer, _| {
            assert_eq!(
                buffer
                    .snapshot()
                    .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
                    .collect::<Vec<_>>(),
                &[
                    DiagnosticEntry {
                        range: Point::new(2, 21)..Point::new(2, 22),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::WARNING,
                            message: "undefined variable 'A'".to_string(),
                            is_disk_based: true,
                            group_id: 6,
                            is_primary: true,
                            ..Default::default()
                        }
                    },
                    DiagnosticEntry {
                        range: Point::new(3, 9)..Point::new(3, 14),
                        diagnostic: Diagnostic {
                            severity: DiagnosticSeverity::ERROR,
                            message: "undefined variable 'BB'".to_string(),
                            is_disk_based: true,
                            group_id: 5,
                            is_primary: true,
                            ..Default::default()
                        },
                    }
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_empty_diagnostic_ranges(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let text = concat!(
            "let one = ;\n", //
            "let two = \n",
            "let three = 3;\n",
        );

        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/dir", json!({ "a.rs": text })).await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
            .await
            .unwrap();

        project.update(cx, |project, cx| {
            project
                .update_buffer_diagnostics(
                    &buffer,
                    vec![
                        DiagnosticEntry {
                            range: PointUtf16::new(0, 10)..PointUtf16::new(0, 10),
                            diagnostic: Diagnostic {
                                severity: DiagnosticSeverity::ERROR,
                                message: "syntax error 1".to_string(),
                                ..Default::default()
                            },
                        },
                        DiagnosticEntry {
                            range: PointUtf16::new(1, 10)..PointUtf16::new(1, 10),
                            diagnostic: Diagnostic {
                                severity: DiagnosticSeverity::ERROR,
                                message: "syntax error 2".to_string(),
                                ..Default::default()
                            },
                        },
                    ],
                    None,
                    cx,
                )
                .unwrap();
        });

        // An empty range is extended forward to include the following character.
        // At the end of a line, an empty range is extended backward to include
        // the preceding character.
        buffer.read_with(cx, |buffer, _| {
            let chunks = chunks_with_diagnostics(&buffer, 0..buffer.len());
            assert_eq!(
                chunks
                    .iter()
                    .map(|(s, d)| (s.as_str(), *d))
                    .collect::<Vec<_>>(),
                &[
                    ("let one = ", None),
                    (";", Some(DiagnosticSeverity::ERROR)),
                    ("\nlet two =", None),
                    (" ", Some(DiagnosticSeverity::ERROR)),
                    ("\nlet three = 3;\n", None)
                ]
            );
        });
    }

    #[gpui::test]
    async fn test_edits_from_lsp_with_past_version(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_servers = language.set_fake_lsp_adapter(Default::default());

        let text = "
            fn a() {
                f1();
            }
            fn b() {
                f2();
            }
            fn c() {
                f3();
            }
        "
        .unindent();

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.rs": text.clone(),
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages.add(Arc::new(language)));
        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
            .await
            .unwrap();

        let mut fake_server = fake_servers.next().await.unwrap();
        let lsp_document_version = fake_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await
            .text_document
            .version;

        // Simulate editing the buffer after the language server computes some edits.
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(
                    Point::new(0, 0)..Point::new(0, 0),
                    "// above first function\n",
                )],
                cx,
            );
            buffer.edit(
                [(
                    Point::new(2, 0)..Point::new(2, 0),
                    "    // inside first function\n",
                )],
                cx,
            );
            buffer.edit(
                [(
                    Point::new(6, 4)..Point::new(6, 4),
                    "// inside second function ",
                )],
                cx,
            );

            assert_eq!(
                buffer.text(),
                "
                    // above first function
                    fn a() {
                        // inside first function
                        f1();
                    }
                    fn b() {
                        // inside second function f2();
                    }
                    fn c() {
                        f3();
                    }
                "
                .unindent()
            );
        });

        let edits = project
            .update(cx, |project, cx| {
                project.edits_from_lsp(
                    &buffer,
                    vec![
                        // replace body of first function
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 0),
                                lsp::Position::new(3, 0),
                            ),
                            new_text: "
                                fn a() {
                                    f10();
                                }
                            "
                            .unindent(),
                        },
                        // edit inside second function
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(4, 6),
                                lsp::Position::new(4, 6),
                            ),
                            new_text: "00".into(),
                        },
                        // edit inside third function via two distinct edits
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(7, 5),
                                lsp::Position::new(7, 5),
                            ),
                            new_text: "4000".into(),
                        },
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(7, 5),
                                lsp::Position::new(7, 6),
                            ),
                            new_text: "".into(),
                        },
                    ],
                    Some(lsp_document_version),
                    cx,
                )
            })
            .await
            .unwrap();

        buffer.update(cx, |buffer, cx| {
            for (range, new_text) in edits {
                buffer.edit([(range, new_text)], cx);
            }
            assert_eq!(
                buffer.text(),
                "
                    // above first function
                    fn a() {
                        // inside first function
                        f10();
                    }
                    fn b() {
                        // inside second function f200();
                    }
                    fn c() {
                        f4000();
                    }
                "
                .unindent()
            );
        });
    }

    #[gpui::test]
    async fn test_edits_from_lsp_with_edits_on_adjacent_lines(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let text = "
            use a::b;
            use a::c;

            fn f() {
                b();
                c();
            }
        "
        .unindent();

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.rs": text.clone(),
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/a.rs", cx))
            .await
            .unwrap();

        // Simulate the language server sending us a small edit in the form of a very large diff.
        // Rust-analyzer does this when performing a merge-imports code action.
        let edits = project
            .update(cx, |project, cx| {
                project.edits_from_lsp(
                    &buffer,
                    [
                        // Replace the first use statement without editing the semicolon.
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 4),
                                lsp::Position::new(0, 8),
                            ),
                            new_text: "a::{b, c}".into(),
                        },
                        // Reinsert the remainder of the file between the semicolon and the final
                        // newline of the file.
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 9),
                                lsp::Position::new(0, 9),
                            ),
                            new_text: "\n\n".into(),
                        },
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 9),
                                lsp::Position::new(0, 9),
                            ),
                            new_text: "
                                fn f() {
                                    b();
                                    c();
                                }"
                            .unindent(),
                        },
                        // Delete everything after the first newline of the file.
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(1, 0),
                                lsp::Position::new(7, 0),
                            ),
                            new_text: "".into(),
                        },
                    ],
                    None,
                    cx,
                )
            })
            .await
            .unwrap();

        buffer.update(cx, |buffer, cx| {
            let edits = edits
                .into_iter()
                .map(|(range, text)| {
                    (
                        range.start.to_point(&buffer)..range.end.to_point(&buffer),
                        text,
                    )
                })
                .collect::<Vec<_>>();

            assert_eq!(
                edits,
                [
                    (Point::new(0, 4)..Point::new(0, 8), "a::{b, c}".into()),
                    (Point::new(1, 0)..Point::new(2, 0), "".into())
                ]
            );

            for (range, new_text) in edits {
                buffer.edit([(range, new_text)], cx);
            }
            assert_eq!(
                buffer.text(),
                "
                    use a::{b, c};

                    fn f() {
                        b();
                        c();
                    }
                "
                .unindent()
            );
        });
    }

    fn chunks_with_diagnostics<T: ToOffset + ToPoint>(
        buffer: &Buffer,
        range: Range<T>,
    ) -> Vec<(String, Option<DiagnosticSeverity>)> {
        let mut chunks: Vec<(String, Option<DiagnosticSeverity>)> = Vec::new();
        for chunk in buffer.snapshot().chunks(range, true) {
            if chunks.last().map_or(false, |prev_chunk| {
                prev_chunk.1 == chunk.diagnostic_severity
            }) {
                chunks.last_mut().unwrap().0.push_str(chunk.text);
            } else {
                chunks.push((chunk.text.to_string(), chunk.diagnostic_severity));
            }
        }
        chunks
    }

    #[gpui::test]
    async fn test_search_worktree_without_files(cx: &mut gpui::TestAppContext) {
        let dir = temp_tree(json!({
            "root": {
                "dir1": {},
                "dir2": {
                    "dir3": {}
                }
            }
        }));

        let project = Project::test(Arc::new(RealFs), [dir.path()], cx).await;
        let cancel_flag = Default::default();
        let results = project
            .read_with(cx, |project, cx| {
                project.match_paths("dir", false, false, 10, &cancel_flag, cx)
            })
            .await;

        assert!(results.is_empty());
    }

    #[gpui::test(iterations = 10)]
    async fn test_definition(cx: &mut gpui::TestAppContext) {
        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_servers = language.set_fake_lsp_adapter(Default::default());

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.rs": "const fn a() { A }",
                "b.rs": "const y: i32 = crate::a()",
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir/b.rs".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages.add(Arc::new(language)));

        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/dir/b.rs", cx))
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        fake_server.handle_request::<lsp::request::GotoDefinition, _, _>(|params, _| async move {
            let params = params.text_document_position_params;
            assert_eq!(
                params.text_document.uri.to_file_path().unwrap(),
                Path::new("/dir/b.rs"),
            );
            assert_eq!(params.position, lsp::Position::new(0, 22));

            Ok(Some(lsp::GotoDefinitionResponse::Scalar(
                lsp::Location::new(
                    lsp::Url::from_file_path("/dir/a.rs").unwrap(),
                    lsp::Range::new(lsp::Position::new(0, 9), lsp::Position::new(0, 10)),
                ),
            )))
        });

        let mut definitions = project
            .update(cx, |project, cx| project.definition(&buffer, 22, cx))
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
                [("/dir/b.rs".as_ref(), true), ("/dir/a.rs".as_ref(), false)]
            );

            drop(definition);
        });
        cx.read(|cx| {
            assert_eq!(list_worktrees(&project, cx), [("/dir/b.rs".as_ref(), true)]);
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
                        worktree.is_visible(),
                    )
                })
                .collect::<Vec<_>>()
        }
    }

    #[gpui::test]
    async fn test_completions_without_edit_ranges(cx: &mut gpui::TestAppContext) {
        let mut language = Language::new(
            LanguageConfig {
                name: "TypeScript".into(),
                path_suffixes: vec!["ts".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_typescript::language_typescript()),
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.ts": "",
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages.add(Arc::new(language)));
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/a.ts", cx))
            .await
            .unwrap();

        let fake_server = fake_language_servers.next().await.unwrap();

        let text = "let a = b.fqn";
        buffer.update(cx, |buffer, cx| buffer.set_text(text, cx));
        let completions = project.update(cx, |project, cx| {
            project.completions(&buffer, text.len(), cx)
        });

        fake_server
            .handle_request::<lsp::request::Completion, _, _>(|_, _| async move {
                Ok(Some(lsp::CompletionResponse::Array(vec![
                    lsp::CompletionItem {
                        label: "fullyQualifiedName?".into(),
                        insert_text: Some("fullyQualifiedName".into()),
                        ..Default::default()
                    },
                ])))
            })
            .next()
            .await;
        let completions = completions.await.unwrap();
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
        assert_eq!(completions.len(), 1);
        assert_eq!(completions[0].new_text, "fullyQualifiedName");
        assert_eq!(
            completions[0].old_range.to_offset(&snapshot),
            text.len() - 3..text.len()
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_apply_code_actions_with_commands(cx: &mut gpui::TestAppContext) {
        let mut language = Language::new(
            LanguageConfig {
                name: "TypeScript".into(),
                path_suffixes: vec!["ts".to_string()],
                ..Default::default()
            },
            None,
        );
        let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default());

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.ts": "a",
            }),
        )
        .await;

        let project = Project::test(fs, ["/dir".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages.add(Arc::new(language)));
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/a.ts", cx))
            .await
            .unwrap();

        let fake_server = fake_language_servers.next().await.unwrap();

        // Language server returns code actions that contain commands, and not edits.
        let actions = project.update(cx, |project, cx| project.code_actions(&buffer, 0..0, cx));
        fake_server
            .handle_request::<lsp::request::CodeActionRequest, _, _>(|_, _| async move {
                Ok(Some(vec![
                    lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                        title: "The code action".into(),
                        command: Some(lsp::Command {
                            title: "The command".into(),
                            command: "_the/command".into(),
                            arguments: Some(vec![json!("the-argument")]),
                        }),
                        ..Default::default()
                    }),
                    lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                        title: "two".into(),
                        ..Default::default()
                    }),
                ]))
            })
            .next()
            .await;

        let action = actions.await.unwrap()[0].clone();
        let apply = project.update(cx, |project, cx| {
            project.apply_code_action(buffer.clone(), action, true, cx)
        });

        // Resolving the code action does not populate its edits. In absence of
        // edits, we must execute the given command.
        fake_server.handle_request::<lsp::request::CodeActionResolveRequest, _, _>(
            |action, _| async move { Ok(action) },
        );

        // While executing the command, the language server sends the editor
        // a `workspaceEdit` request.
        fake_server
            .handle_request::<lsp::request::ExecuteCommand, _, _>({
                let fake = fake_server.clone();
                move |params, _| {
                    assert_eq!(params.command, "_the/command");
                    let fake = fake.clone();
                    async move {
                        fake.server
                            .request::<lsp::request::ApplyWorkspaceEdit>(
                                lsp::ApplyWorkspaceEditParams {
                                    label: None,
                                    edit: lsp::WorkspaceEdit {
                                        changes: Some(
                                            [(
                                                lsp::Url::from_file_path("/dir/a.ts").unwrap(),
                                                vec![lsp::TextEdit {
                                                    range: lsp::Range::new(
                                                        lsp::Position::new(0, 0),
                                                        lsp::Position::new(0, 0),
                                                    ),
                                                    new_text: "X".into(),
                                                }],
                                            )]
                                            .into_iter()
                                            .collect(),
                                        ),
                                        ..Default::default()
                                    },
                                },
                            )
                            .await
                            .unwrap();
                        Ok(Some(json!(null)))
                    }
                }
            })
            .next()
            .await;

        // Applying the code action returns a project transaction containing the edits
        // sent by the language server in its `workspaceEdit` request.
        let transaction = apply.await.unwrap();
        assert!(transaction.0.contains_key(&buffer));
        buffer.update(cx, |buffer, cx| {
            assert_eq!(buffer.text(), "Xa");
            buffer.undo(cx);
            assert_eq!(buffer.text(), "a");
        });
    }

    #[gpui::test]
    async fn test_save_file(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "file1": "the old contents",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
            .await
            .unwrap();
        buffer
            .update(cx, |buffer, cx| {
                assert_eq!(buffer.text(), "the old contents");
                buffer.edit([(0..0, "a line of text.\n".repeat(10 * 1024))], cx);
                buffer.save(cx)
            })
            .await
            .unwrap();

        let new_text = fs.load(Path::new("/dir/file1")).await.unwrap();
        assert_eq!(new_text, buffer.read_with(cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_save_in_single_file_worktree(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "file1": "the old contents",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/dir/file1".as_ref()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
            .await
            .unwrap();
        buffer
            .update(cx, |buffer, cx| {
                buffer.edit([(0..0, "a line of text.\n".repeat(10 * 1024))], cx);
                buffer.save(cx)
            })
            .await
            .unwrap();

        let new_text = fs.load(Path::new("/dir/file1")).await.unwrap();
        assert_eq!(new_text, buffer.read_with(cx, |buffer, _| buffer.text()));
    }

    #[gpui::test]
    async fn test_save_as(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree("/dir", json!({})).await;

        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let buffer = project.update(cx, |project, cx| {
            project.create_buffer("", None, cx).unwrap()
        });
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "abc")], cx);
            assert!(buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        project
            .update(cx, |project, cx| {
                project.save_buffer_as(buffer.clone(), "/dir/file1".into(), cx)
            })
            .await
            .unwrap();
        assert_eq!(fs.load(Path::new("/dir/file1")).await.unwrap(), "abc");
        buffer.read_with(cx, |buffer, cx| {
            assert_eq!(buffer.file().unwrap().full_path(cx), Path::new("dir/file1"));
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });

        let opened_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/dir/file1", cx)
            })
            .await
            .unwrap();
        assert_eq!(opened_buffer, buffer);
    }

    #[gpui::test(retries = 5)]
    async fn test_rescan_and_remote_updates(cx: &mut gpui::TestAppContext) {
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

        let project = Project::test(Arc::new(RealFs), [dir.path()], cx).await;
        let rpc = project.read_with(cx, |p, _| p.client.clone());

        let buffer_for_path = |path: &'static str, cx: &mut gpui::TestAppContext| {
            let buffer = project.update(cx, |p, cx| p.open_local_buffer(dir.path().join(path), cx));
            async move { buffer.await.unwrap() }
        };
        let id_for_path = |path: &'static str, cx: &gpui::TestAppContext| {
            project.read_with(cx, |project, cx| {
                let tree = project.worktrees(cx).next().unwrap();
                tree.read(cx)
                    .entry_for_path(path)
                    .expect(&format!("no entry for path {}", path))
                    .id
            })
        };

        let buffer2 = buffer_for_path("a/file2", cx).await;
        let buffer3 = buffer_for_path("a/file3", cx).await;
        let buffer4 = buffer_for_path("b/c/file4", cx).await;
        let buffer5 = buffer_for_path("b/c/file5", cx).await;

        let file2_id = id_for_path("a/file2", &cx);
        let file3_id = id_for_path("a/file3", &cx);
        let file4_id = id_for_path("b/c/file4", &cx);

        // Create a remote copy of this worktree.
        let tree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
        let initial_snapshot = tree.read_with(cx, |tree, _| tree.as_local().unwrap().snapshot());
        let (remote, load_task) = cx.update(|cx| {
            Worktree::remote(
                1,
                1,
                initial_snapshot.to_proto(&Default::default(), true),
                rpc.clone(),
                cx,
            )
        });
        // tree
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
        remote.update(cx, |remote, cx| {
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
    async fn test_buffer_deduping(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

        // Spawn multiple tasks to open paths, repeating some paths.
        let (buffer_a_1, buffer_b, buffer_a_2) = project.update(cx, |p, cx| {
            (
                p.open_local_buffer("/dir/a.txt", cx),
                p.open_local_buffer("/dir/b.txt", cx),
                p.open_local_buffer("/dir/a.txt", cx),
            )
        });

        let buffer_a_1 = buffer_a_1.await.unwrap();
        let buffer_a_2 = buffer_a_2.await.unwrap();
        let buffer_b = buffer_b.await.unwrap();
        assert_eq!(buffer_a_1.read_with(cx, |b, _| b.text()), "a-contents");
        assert_eq!(buffer_b.read_with(cx, |b, _| b.text()), "b-contents");

        // There is only one buffer per path.
        let buffer_a_id = buffer_a_1.id();
        assert_eq!(buffer_a_2.id(), buffer_a_id);

        // Open the same path again while it is still open.
        drop(buffer_a_1);
        let buffer_a_3 = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/a.txt", cx))
            .await
            .unwrap();

        // There's still only one buffer per path.
        assert_eq!(buffer_a_3.id(), buffer_a_id);
    }

    #[gpui::test]
    async fn test_buffer_is_dirty(cx: &mut gpui::TestAppContext) {
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "file1": "abc",
                "file2": "def",
                "file3": "ghi",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;

        let buffer1 = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/file1", cx))
            .await
            .unwrap();
        let events = Rc::new(RefCell::new(Vec::new()));

        // initially, the buffer isn't dirty.
        buffer1.update(cx, |buffer, cx| {
            cx.subscribe(&buffer1, {
                let events = events.clone();
                move |_, _, event, _| match event {
                    BufferEvent::Operation(_) => {}
                    _ => events.borrow_mut().push(event.clone()),
                }
            })
            .detach();

            assert!(!buffer.is_dirty());
            assert!(events.borrow().is_empty());

            buffer.edit([(1..2, "")], cx);
        });

        // after the first edit, the buffer is dirty, and emits a dirtied event.
        buffer1.update(cx, |buffer, cx| {
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
        buffer1.update(cx, |buffer, cx| {
            assert!(!buffer.is_dirty());
            assert_eq!(*events.borrow(), &[language::Event::Saved]);
            events.borrow_mut().clear();

            buffer.edit([(1..1, "B")], cx);
            buffer.edit([(2..2, "D")], cx);
        });

        // after editing again, the buffer is dirty, and emits another dirty event.
        buffer1.update(cx, |buffer, cx| {
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
            buffer.edit([(1..3, "")], cx);
            assert!(buffer.text() == "ac");
            assert!(buffer.is_dirty());
        });

        assert_eq!(*events.borrow(), &[language::Event::Edited]);

        // When a file is deleted, the buffer is considered dirty.
        let events = Rc::new(RefCell::new(Vec::new()));
        let buffer2 = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/file2", cx))
            .await
            .unwrap();
        buffer2.update(cx, |_, cx| {
            cx.subscribe(&buffer2, {
                let events = events.clone();
                move |_, _, event, _| events.borrow_mut().push(event.clone())
            })
            .detach();
        });

        fs.remove_file("/dir/file2".as_ref(), Default::default())
            .await
            .unwrap();
        buffer2.condition(&cx, |b, _| b.is_dirty()).await;
        assert_eq!(
            *events.borrow(),
            &[language::Event::Dirtied, language::Event::FileHandleChanged]
        );

        // When a file is already dirty when deleted, we don't emit a Dirtied event.
        let events = Rc::new(RefCell::new(Vec::new()));
        let buffer3 = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/file3", cx))
            .await
            .unwrap();
        buffer3.update(cx, |_, cx| {
            cx.subscribe(&buffer3, {
                let events = events.clone();
                move |_, _, event, _| events.borrow_mut().push(event.clone())
            })
            .detach();
        });

        buffer3.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "x")], cx);
        });
        events.borrow_mut().clear();
        fs.remove_file("/dir/file3".as_ref(), Default::default())
            .await
            .unwrap();
        buffer3
            .condition(&cx, |_, _| !events.borrow().is_empty())
            .await;
        assert_eq!(*events.borrow(), &[language::Event::FileHandleChanged]);
        cx.read(|cx| assert!(buffer3.read(cx).is_dirty()));
    }

    #[gpui::test]
    async fn test_buffer_file_changes_on_disk(cx: &mut gpui::TestAppContext) {
        let initial_contents = "aaa\nbbbbb\nc\n";
        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "the-file": initial_contents,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer("/dir/the-file", cx))
            .await
            .unwrap();

        let anchors = (0..3)
            .map(|row| buffer.read_with(cx, |b, _| b.anchor_before(Point::new(row, 1))))
            .collect::<Vec<_>>();

        // Change the file on disk, adding two new lines of text, and removing
        // one line.
        buffer.read_with(cx, |buffer, _| {
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        let new_contents = "AAAA\naaa\nBB\nbbbbb\n";
        fs.save("/dir/the-file".as_ref(), &new_contents.into())
            .await
            .unwrap();

        // Because the buffer was not modified, it is reloaded from disk. Its
        // contents are edited according to the diff between the old and new
        // file contents.
        buffer
            .condition(&cx, |buffer, _| buffer.text() == new_contents)
            .await;

        buffer.update(cx, |buffer, _| {
            assert_eq!(buffer.text(), new_contents);
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());

            let anchor_positions = anchors
                .iter()
                .map(|anchor| anchor.to_point(&*buffer))
                .collect::<Vec<_>>();
            assert_eq!(
                anchor_positions,
                [Point::new(1, 1), Point::new(3, 1), Point::new(4, 0)]
            );
        });

        // Modify the buffer
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, " ")], cx);
            assert!(buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });

        // Change the file on disk again, adding blank lines to the beginning.
        fs.save(
            "/dir/the-file".as_ref(),
            &"\n\n\nAAAA\naaa\nBB\nbbbbb\n".into(),
        )
        .await
        .unwrap();

        // Because the buffer is modified, it doesn't reload from disk, but is
        // marked as having a conflict.
        buffer
            .condition(&cx, |buffer, _| buffer.has_conflict())
            .await;
    }

    #[gpui::test]
    async fn test_grouped_diagnostics(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

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

        let project = Project::test(fs.clone(), ["/the-dir".as_ref()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer("/the-dir/a.rs", cx))
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
            .update(cx, |p, cx| p.update_diagnostics(message, &[], cx))
            .unwrap();
        let buffer = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        assert_eq!(
            buffer
                .diagnostics_in_range::<_, Point>(0..buffer.len(), false)
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
    async fn test_rename(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let mut language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        );
        let mut fake_servers = language.set_fake_lsp_adapter(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                ..Default::default()
            },
            ..Default::default()
        });

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages.add(Arc::new(language)));
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/dir/one.rs", cx)
            })
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();

        let response = project.update(cx, |project, cx| {
            project.prepare_rename(buffer.clone(), 7, cx)
        });
        fake_server
            .handle_request::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
                assert_eq!(params.text_document.uri.as_str(), "file:///dir/one.rs");
                assert_eq!(params.position, lsp::Position::new(0, 7));
                Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                    lsp::Position::new(0, 6),
                    lsp::Position::new(0, 9),
                ))))
            })
            .next()
            .await
            .unwrap();
        let range = response.await.unwrap().unwrap();
        let range = buffer.read_with(cx, |buffer, _| range.to_offset(buffer));
        assert_eq!(range, 6..9);

        let response = project.update(cx, |project, cx| {
            project.perform_rename(buffer.clone(), 7, "THREE".to_string(), true, cx)
        });
        fake_server
            .handle_request::<lsp::request::Rename, _, _>(|params, _| async move {
                assert_eq!(
                    params.text_document_position.text_document.uri.as_str(),
                    "file:///dir/one.rs"
                );
                assert_eq!(
                    params.text_document_position.position,
                    lsp::Position::new(0, 7)
                );
                assert_eq!(params.new_name, "THREE");
                Ok(Some(lsp::WorkspaceEdit {
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
                }))
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
                .read_with(cx, |buffer, _| buffer.text()),
            "const THREE: usize = 1;"
        );
        assert_eq!(
            transaction
                .into_keys()
                .next()
                .unwrap()
                .read_with(cx, |buffer, _| buffer.text()),
            "const TWO: usize = one::THREE + one::THREE;"
        );
    }

    #[gpui::test]
    async fn test_search(cx: &mut gpui::TestAppContext) {
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
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        assert_eq!(
            search(&project, SearchQuery::text("TWO", false, true), cx)
                .await
                .unwrap(),
            HashMap::from_iter([
                ("two.rs".to_string(), vec![6..9]),
                ("three.rs".to_string(), vec![37..40])
            ])
        );

        let buffer_4 = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/dir/four.rs", cx)
            })
            .await
            .unwrap();
        buffer_4.update(cx, |buffer, cx| {
            let text = "two::TWO";
            buffer.edit([(20..28, text), (31..43, text)], cx);
        });

        assert_eq!(
            search(&project, SearchQuery::text("TWO", false, true), cx)
                .await
                .unwrap(),
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
        ) -> Result<HashMap<String, Vec<Range<usize>>>> {
            let results = project
                .update(cx, |project, cx| project.search(query, cx))
                .await?;

            Ok(results
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
                .collect())
        }
    }
}
