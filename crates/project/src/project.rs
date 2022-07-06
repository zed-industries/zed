mod db;
pub mod fs;
mod ignore;
mod lsp_command;
pub mod search;
pub mod worktree;

#[cfg(test)]
mod project_tests;

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
    proto::{
        deserialize_anchor, deserialize_line_ending, deserialize_version, serialize_anchor,
        serialize_version,
    },
    range_from_lsp, range_to_lsp, Anchor, Bias, Buffer, CharKind, CodeAction, CodeLabel,
    Completion, Diagnostic, DiagnosticEntry, DiagnosticSet, Event as BufferEvent, File as _,
    Language, LanguageRegistry, LanguageServerName, LocalFile, LspAdapter, OffsetRangeExt,
    Operation, Patch, PointUtf16, TextBufferSnapshot, ToOffset, ToPointUtf16, Transaction,
};
use lsp::{
    DiagnosticSeverity, DiagnosticTag, DocumentHighlightKind, LanguageServer, LanguageString,
    MarkedString,
};
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

pub use db::Db;
pub use fs::*;
pub use worktree::*;

pub trait Item: Entity {
    fn entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId>;
}

pub struct ProjectStore {
    db: Arc<Db>,
    projects: Vec<WeakModelHandle<Project>>,
}

// Language server state is stored across 3 collections:
//     language_servers =>
//         a mapping from unique server id to LanguageServerState which can either be a task for a
//         server in the process of starting, or a running server with adapter and language server arcs
//     language_server_ids => a mapping from worktreeId and server name to the unique server id
//     language_server_statuses => a mapping from unique server id to the current server status
//
// Multiple worktrees can map to the same language server for example when you jump to the definition
// of a file in the standard library. So language_server_ids is used to look up which server is active
// for a given worktree and language server name
//
// When starting a language server, first the id map is checked to make sure a server isn't already available
// for that worktree. If there is one, it finishes early. Otherwise, a new id is allocated and and
// the Starting variant of LanguageServerState is stored in the language_servers map.
pub struct Project {
    worktrees: Vec<WorktreeHandle>,
    active_entry: Option<ProjectEntryId>,
    languages: Arc<LanguageRegistry>,
    language_servers: HashMap<usize, LanguageServerState>,
    language_server_ids: HashMap<(WorktreeId, LanguageServerName), usize>,
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
    client_subscriptions: Vec<client::Subscription>,
    _subscriptions: Vec<gpui::Subscription>,
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
    initialized_persistent_state: bool,
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
        online_tx: watch::Sender<bool>,
        online_rx: watch::Receiver<bool>,
        _maintain_remote_id: Task<Option<()>>,
        _maintain_online_status: Task<Option<()>>,
    },
    Remote {
        sharing_has_stopped: bool,
        remote_id: u64,
        replica_id: ReplicaId,
        _detect_unshare: Task<Option<()>>,
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
    DiskBasedDiagnosticsStarted {
        language_server_id: usize,
    },
    DiskBasedDiagnosticsFinished {
        language_server_id: usize,
    },
    DiagnosticsUpdated {
        path: ProjectPath,
        language_server_id: usize,
    },
    RemoteIdChanged(Option<u64>),
    CollaboratorLeft(PeerId),
    ContactRequestedJoin(Arc<User>),
    ContactCancelledJoinRequest(Arc<User>),
}

pub enum LanguageServerState {
    Starting(Task<Option<Arc<LanguageServer>>>),
    Running {
        adapter: Arc<dyn LspAdapter>,
        server: Arc<LanguageServer>,
    },
}

#[derive(Serialize)]
pub struct LanguageServerStatus {
    pub name: String,
    pub pending_work: BTreeMap<String, LanguageServerProgress>,
    pub has_pending_diagnostic_updates: bool,
    progress_tokens: HashSet<String>,
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
    pub language_server_id: usize,
    pub error_count: usize,
    pub warning_count: usize,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub buffer: ModelHandle<Buffer>,
    pub range: Range<language::Anchor>,
}

#[derive(Debug, Clone)]
pub struct LocationLink {
    pub origin: Option<Location>,
    pub target: Location,
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

#[derive(Clone, Debug, PartialEq)]
pub struct HoverBlock {
    pub text: String,
    pub language: Option<String>,
}

impl HoverBlock {
    fn try_new(marked_string: MarkedString) -> Option<Self> {
        let result = match marked_string {
            MarkedString::LanguageString(LanguageString { language, value }) => HoverBlock {
                text: value,
                language: Some(language),
            },
            MarkedString::String(text) => HoverBlock {
                text,
                language: None,
            },
        };
        if result.text.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

#[derive(Debug)]
pub struct Hover {
    pub contents: Vec<HoverBlock>,
    pub range: Option<Range<language::Anchor>>,
}

#[derive(Default)]
pub struct ProjectTransaction(pub HashMap<ModelHandle<Buffer>, language::Transaction>);

impl DiagnosticSummary {
    fn new<'a, T: 'a>(
        language_server_id: usize,
        diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<T>>,
    ) -> Self {
        let mut this = Self {
            language_server_id,
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
            language_server_id: self.language_server_id as u64,
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
        client.add_model_request_handler(Self::handle_lsp_command::<GetHover>);
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
        online: bool,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        project_store: ModelHandle<ProjectStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut MutableAppContext,
    ) -> ModelHandle<Self> {
        cx.add_model(|cx: &mut ModelContext<Self>| {
            let (remote_id_tx, remote_id_rx) = watch::channel();
            let _maintain_remote_id = cx.spawn_weak({
                let mut status_rx = client.clone().status();
                move |this, mut cx| async move {
                    while let Some(status) = status_rx.recv().await {
                        let this = this.upgrade(&cx)?;
                        if status.is_connected() {
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

            let (online_tx, online_rx) = watch::channel_with(online);
            let _maintain_online_status = cx.spawn_weak({
                let mut online_rx = online_rx.clone();
                move |this, mut cx| async move {
                    while let Some(online) = online_rx.recv().await {
                        let this = this.upgrade(&cx)?;
                        this.update(&mut cx, |this, cx| {
                            if !online {
                                this.unshared(cx);
                            }
                            this.metadata_changed(false, cx)
                        });
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
                    online_tx,
                    online_rx,
                    _maintain_remote_id,
                    _maintain_online_status,
                },
                opened_buffer: (Rc::new(RefCell::new(opened_buffer_tx)), opened_buffer_rx),
                client_subscriptions: Vec::new(),
                _subscriptions: vec![cx.observe_global::<Settings, _>(Self::on_settings_changed)],
                active_entry: None,
                languages,
                client,
                user_store,
                project_store,
                fs,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                language_servers: Default::default(),
                language_server_ids: Default::default(),
                language_server_statuses: Default::default(),
                last_workspace_edits_by_language_server: Default::default(),
                language_server_settings: Default::default(),
                next_language_server_id: 0,
                nonce: StdRng::from_entropy().gen(),
                initialized_persistent_state: false,
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
            let worktree = cx
                .update(|cx| Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx));
            worktrees.push(worktree);
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
                client_subscriptions: vec![client.add_model_for_remote_entity(remote_id, cx)],
                _subscriptions: Default::default(),
                client: client.clone(),
                client_state: ProjectClientState::Remote {
                    sharing_has_stopped: false,
                    remote_id,
                    replica_id,
                    _detect_unshare: cx.spawn_weak(move |this, mut cx| {
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
                language_server_ids: Default::default(),
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
                                has_pending_diagnostic_updates: false,
                                progress_tokens: Default::default(),
                            },
                        )
                    })
                    .collect(),
                last_workspace_edits_by_language_server: Default::default(),
                next_language_server_id: 0,
                opened_buffers: Default::default(),
                buffer_snapshots: Default::default(),
                nonce: StdRng::from_entropy().gen(),
                initialized_persistent_state: false,
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
        if !cx.read(|cx| cx.has_global::<Settings>()) {
            cx.update(|cx| cx.set_global(Settings::test(cx)));
        }

        let languages = Arc::new(LanguageRegistry::test());
        let http_client = client::test::FakeHttpClient::with_404_response();
        let client = client::Client::new(http_client.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let project_store = cx.add_model(|_| ProjectStore::new(Db::open_fake()));
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

    pub fn restore_state(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if self.is_remote() {
            return Task::ready(Ok(()));
        }

        let db = self.project_store.read(cx).db.clone();
        let keys = self.db_keys_for_online_state(cx);
        let online_by_default = cx.global::<Settings>().projects_online_by_default;
        let read_online = cx.background().spawn(async move {
            let values = db.read(keys)?;
            anyhow::Ok(
                values
                    .into_iter()
                    .all(|e| e.map_or(online_by_default, |e| e == [true as u8])),
            )
        });
        cx.spawn(|this, mut cx| async move {
            let online = read_online.await.log_err().unwrap_or(false);
            this.update(&mut cx, |this, cx| {
                this.initialized_persistent_state = true;
                if let ProjectClientState::Local { online_tx, .. } = &mut this.client_state {
                    let mut online_tx = online_tx.borrow_mut();
                    if *online_tx != online {
                        *online_tx = online;
                        drop(online_tx);
                        this.metadata_changed(false, cx);
                    }
                }
            });
            Ok(())
        })
    }

    fn persist_state(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if self.is_remote() || !self.initialized_persistent_state {
            return Task::ready(Ok(()));
        }

        let db = self.project_store.read(cx).db.clone();
        let keys = self.db_keys_for_online_state(cx);
        let is_online = self.is_online();
        cx.background().spawn(async move {
            let value = &[is_online as u8];
            db.write(keys.into_iter().map(|key| (key, value)))
        })
    }

    fn on_settings_changed(&mut self, cx: &mut ModelContext<Self>) {
        let settings = cx.global::<Settings>();

        let mut language_servers_to_start = Vec::new();
        for buffer in self.opened_buffers.values() {
            if let Some(buffer) = buffer.upgrade(cx) {
                let buffer = buffer.read(cx);
                if let Some((file, language)) = File::from_dyn(buffer.file()).zip(buffer.language())
                {
                    if settings.enable_language_server(Some(&language.name())) {
                        let worktree = file.worktree.read(cx);
                        language_servers_to_start.push((
                            worktree.id(),
                            worktree.as_local().unwrap().abs_path().clone(),
                            language.clone(),
                        ));
                    }
                }
            }
        }

        let mut language_servers_to_stop = Vec::new();
        for language in self.languages.to_vec() {
            if let Some(lsp_adapter) = language.lsp_adapter() {
                if !settings.enable_language_server(Some(&language.name())) {
                    let lsp_name = lsp_adapter.name();
                    for (worktree_id, started_lsp_name) in self.language_server_ids.keys() {
                        if lsp_name == *started_lsp_name {
                            language_servers_to_stop.push((*worktree_id, started_lsp_name.clone()));
                        }
                    }
                }
            }
        }

        // Stop all newly-disabled language servers.
        for (worktree_id, adapter_name) in language_servers_to_stop {
            self.stop_language_server(worktree_id, adapter_name, cx)
                .detach();
        }

        // Start all the newly-enabled language servers.
        for (worktree_id, worktree_path, language) in language_servers_to_start {
            self.start_language_server(worktree_id, worktree_path, language, cx);
        }

        cx.notify();
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

    pub fn project_store(&self) -> ModelHandle<ProjectStore> {
        self.project_store.clone()
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

    pub fn set_online(&mut self, online: bool, _: &mut ModelContext<Self>) {
        if let ProjectClientState::Local { online_tx, .. } = &mut self.client_state {
            let mut online_tx = online_tx.borrow_mut();
            if *online_tx != online {
                *online_tx = online;
            }
        }
    }

    pub fn is_online(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local { online_rx, .. } => *online_rx.borrow(),
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

                    // Unregistering the project causes the server to send out a
                    // contact update removing this project from the host's list
                    // of online projects. Wait until this contact update has been
                    // processed before clearing out this project's remote id, so
                    // that there is no moment where this project appears in the
                    // contact metadata and *also* has no remote id.
                    this.update(&mut cx, |this, cx| {
                        this.user_store()
                            .update(cx, |store, _| store.contact_updates_done())
                    })
                    .await;

                    this.update(&mut cx, |this, cx| {
                        if let ProjectClientState::Local { remote_id_tx, .. } =
                            &mut this.client_state
                        {
                            *remote_id_tx.borrow_mut() = None;
                        }
                        this.client_subscriptions.clear();
                        this.metadata_changed(false, cx);
                    });
                    response.map(drop)
                });
            }
        }
        Task::ready(Ok(()))
    }

    fn register(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        if let ProjectClientState::Local {
            remote_id_rx,
            online_rx,
            ..
        } = &self.client_state
        {
            if remote_id_rx.borrow().is_some() {
                return Task::ready(Ok(()));
            }

            let response = self.client.request(proto::RegisterProject {
                online: *online_rx.borrow(),
            });
            cx.spawn(|this, mut cx| async move {
                let remote_id = response.await?.project_id;
                this.update(&mut cx, |this, cx| {
                    if let ProjectClientState::Local { remote_id_tx, .. } = &mut this.client_state {
                        *remote_id_tx.borrow_mut() = Some(remote_id);
                    }

                    this.metadata_changed(false, cx);
                    cx.emit(Event::RemoteIdChanged(Some(remote_id)));
                    this.client_subscriptions
                        .push(this.client.add_model_for_remote_entity(remote_id, cx));
                    Ok(())
                })
            })
        } else {
            Task::ready(Err(anyhow!("can't register a remote project")))
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

    fn metadata_changed(&mut self, persist: bool, cx: &mut ModelContext<Self>) {
        if let ProjectClientState::Local {
            remote_id_rx,
            online_rx,
            ..
        } = &self.client_state
        {
            // Broadcast worktrees only if the project is online.
            let worktrees = if *online_rx.borrow() {
                self.worktrees
                    .iter()
                    .filter_map(|worktree| {
                        worktree
                            .upgrade(&cx)
                            .map(|worktree| worktree.read(cx).as_local().unwrap().metadata_proto())
                    })
                    .collect()
            } else {
                Default::default()
            };
            if let Some(project_id) = *remote_id_rx.borrow() {
                let online = *online_rx.borrow();
                self.client
                    .send(proto::UpdateProject {
                        project_id,
                        worktrees,
                        online,
                    })
                    .log_err();

                if online {
                    let worktrees = self.visible_worktrees(cx).collect::<Vec<_>>();
                    let scans_complete =
                        futures::future::join_all(worktrees.iter().filter_map(|worktree| {
                            Some(worktree.read(cx).as_local()?.scan_complete())
                        }));

                    let worktrees = worktrees.into_iter().map(|handle| handle.downgrade());
                    cx.spawn_weak(move |_, cx| async move {
                        scans_complete.await;
                        cx.read(|cx| {
                            for worktree in worktrees {
                                if let Some(worktree) = worktree
                                    .upgrade(cx)
                                    .and_then(|worktree| worktree.read(cx).as_local())
                                {
                                    worktree.send_extension_counts(project_id);
                                }
                            }
                        })
                    })
                    .detach();
                }
            }

            self.project_store.update(cx, |_, cx| cx.notify());
            if persist {
                self.persist_state(cx).detach_and_log_err(cx);
            }
            cx.notify();
        }
    }

    pub fn collaborators(&self) -> &HashMap<PeerId, Collaborator> {
        &self.collaborators
    }

    pub fn worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + DoubleEndedIterator<Item = ModelHandle<Worktree>> {
        self.worktrees
            .iter()
            .filter_map(move |worktree| worktree.upgrade(cx))
    }

    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + DoubleEndedIterator<Item = ModelHandle<Worktree>> {
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

    fn db_keys_for_online_state(&self, cx: &AppContext) -> Vec<String> {
        self.worktrees
            .iter()
            .filter_map(|worktree| {
                let worktree = worktree.upgrade(&cx)?.read(cx);
                if worktree.is_visible() {
                    Some(format!(
                        "project-path-online:{}",
                        worktree.as_local().unwrap().abs_path().to_string_lossy()
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
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
                        worktree.as_remote_mut().unwrap().insert_entry(
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
                        worktree.as_remote_mut().unwrap().insert_entry(
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
                        worktree.as_remote_mut().unwrap().insert_entry(
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
                        worktree.as_remote_mut().unwrap().delete_entry(
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
        if !self.is_online() {
            return Task::ready(Err(anyhow!("can't share an offline project")));
        }

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

        for (server_id, status) in &self.language_server_statuses {
            self.client
                .send(proto::StartLanguageServer {
                    project_id,
                    server: Some(proto::LanguageServer {
                        id: *server_id as u64,
                        name: status.name.clone(),
                    }),
                })
                .log_err();
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
            let share = if self.is_online() && allow {
                Some(self.share(cx))
            } else {
                None
            };
            let client = self.client.clone();
            cx.foreground()
                .spawn(async move {
                    client.send(proto::RespondToJoinProjectRequest {
                        requester_id,
                        project_id,
                        allow,
                    })?;
                    if let Some(share) = share {
                        share.await?;
                    }
                    anyhow::Ok(())
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
            for worktree in &self.worktrees {
                if let Some(worktree) = worktree.upgrade(cx) {
                    worktree.update(cx, |worktree, _| {
                        if let Some(worktree) = worktree.as_remote_mut() {
                            worktree.disconnected_from_host();
                        }
                    });
                }
            }
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
        language_server_id: usize,
        language_server_name: LanguageServerName,
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
                    this.language_server_ids.insert(
                        (worktree.read(cx).id(), language_server_name),
                        language_server_id,
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
                            .language_server_ids
                            .get(&(worktree_id, adapter.name()))
                            .and_then(|id| self.language_servers.get(&id))
                            .and_then(|server_state| {
                                if let LanguageServerState::Running { server, .. } = server_state {
                                    Some(server.clone())
                                } else {
                                    None
                                }
                            });
                    }
                }

                if let Some(local_worktree) = file.worktree.read(cx).as_local() {
                    if let Some(diagnostics) = local_worktree.diagnostics_for_path(file.path()) {
                        self.update_buffer_diagnostics(&buffer_handle, diagnostics, None, cx)
                            .log_err();
                    }
                }

                if let Some(server) = language_server {
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
                } else if let Some(project_id) = self.remote_id() {
                    let _ = self
                        .client
                        .send(proto::RegisterProjectActivity { project_id });
                }
            }
            BufferEvent::Edited { .. } => {
                let language_server = self
                    .language_server_for_buffer(buffer.read(cx), cx)
                    .map(|(_, server)| server.clone())?;
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

                // After saving a buffer, simulate disk-based diagnostics being finished for languages
                // that don't support a disk-based progress token.
                let (lsp_adapter, language_server) =
                    self.language_server_for_buffer(buffer.read(cx), cx)?;
                if lsp_adapter
                    .disk_based_diagnostics_progress_token()
                    .is_none()
                {
                    let server_id = language_server.server_id();
                    self.disk_based_diagnostics_finished(server_id, cx);
                    self.broadcast_language_server_update(
                        server_id,
                        proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                            proto::LspDiskBasedDiagnosticsUpdated {},
                        ),
                    );
                }
            }
            _ => {}
        }

        None
    }

    fn language_servers_for_worktree(
        &self,
        worktree_id: WorktreeId,
    ) -> impl Iterator<Item = (&Arc<dyn LspAdapter>, &Arc<LanguageServer>)> {
        self.language_server_ids
            .iter()
            .filter_map(move |((language_server_worktree_id, _), id)| {
                if *language_server_worktree_id == worktree_id {
                    if let Some(LanguageServerState::Running { adapter, server }) =
                        self.language_servers.get(&id)
                    {
                        return Some((adapter, server));
                    }
                }
                None
            })
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
        if !cx
            .global::<Settings>()
            .enable_language_server(Some(&language.name()))
        {
            return;
        }

        let adapter = if let Some(adapter) = language.lsp_adapter() {
            adapter
        } else {
            return;
        };
        let key = (worktree_id, adapter.name());

        self.language_server_ids
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
                self.language_servers.insert(
                    server_id,
                    LanguageServerState::Starting(cx.spawn_weak(|this, mut cx| async move {
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
                                                server_id, params, &adapter, cx,
                                            );
                                        });
                                    }
                                }
                            })
                            .detach();

                        language_server
                            .on_request::<lsp::request::WorkspaceConfiguration, _, _>({
                                let settings = this.read_with(&cx, |this, _| {
                                    this.language_server_settings.clone()
                                });
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

                        // Even though we don't have handling for these requests, respond to them to
                        // avoid stalling any language server like `gopls` which waits for a response
                        // to these requests when initializing.
                        language_server
                            .on_request::<lsp::request::WorkDoneProgressCreate, _, _>({
                                let this = this.downgrade();
                                move |params, mut cx| async move {
                                    if let Some(this) = this.upgrade(&cx) {
                                        this.update(&mut cx, |this, _| {
                                            if let Some(status) =
                                                this.language_server_statuses.get_mut(&server_id)
                                            {
                                                if let lsp::NumberOrString::String(token) =
                                                    params.token
                                                {
                                                    status.progress_tokens.insert(token);
                                                }
                                            }
                                        });
                                    }
                                    Ok(())
                                }
                            })
                            .detach();
                        language_server
                            .on_request::<lsp::request::RegisterCapability, _, _>(|_, _| async {
                                Ok(())
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
                            // If the language server for this key doesn't match the server id, don't store the
                            // server. Which will cause it to be dropped, killing the process
                            if this
                                .language_server_ids
                                .get(&key)
                                .map(|id| id != &server_id)
                                .unwrap_or(false)
                            {
                                return None;
                            }

                            // Update language_servers collection with Running variant of LanguageServerState
                            // indicating that the server is up and running and ready
                            this.language_servers.insert(
                                server_id,
                                LanguageServerState::Running {
                                    adapter: adapter.clone(),
                                    server: language_server.clone(),
                                },
                            );
                            this.language_server_statuses.insert(
                                server_id,
                                LanguageServerStatus {
                                    name: language_server.name().to_string(),
                                    pending_work: Default::default(),
                                    has_pending_diagnostic_updates: false,
                                    progress_tokens: Default::default(),
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
                                    let language_id =
                                        adapter.id_for_language(language.name().as_ref());
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
                            Some(language_server)
                        })
                    })),
                );

                server_id
            });
    }

    // Returns a list of all of the worktrees which no longer have a language server and the root path
    // for the stopped server
    fn stop_language_server(
        &mut self,
        worktree_id: WorktreeId,
        adapter_name: LanguageServerName,
        cx: &mut ModelContext<Self>,
    ) -> Task<(Option<PathBuf>, Vec<WorktreeId>)> {
        let key = (worktree_id, adapter_name);
        if let Some(server_id) = self.language_server_ids.remove(&key) {
            // Remove other entries for this language server as well
            let mut orphaned_worktrees = vec![worktree_id];
            let other_keys = self.language_server_ids.keys().cloned().collect::<Vec<_>>();
            for other_key in other_keys {
                if self.language_server_ids.get(&other_key) == Some(&server_id) {
                    self.language_server_ids.remove(&other_key);
                    orphaned_worktrees.push(other_key.0);
                }
            }

            self.language_server_statuses.remove(&server_id);
            cx.notify();

            let server_state = self.language_servers.remove(&server_id);
            cx.spawn_weak(|this, mut cx| async move {
                let mut root_path = None;

                let server = match server_state {
                    Some(LanguageServerState::Starting(started_language_server)) => {
                        started_language_server.await
                    }
                    Some(LanguageServerState::Running { server, .. }) => Some(server),
                    None => None,
                };

                if let Some(server) = server {
                    root_path = Some(server.root_path().clone());
                    if let Some(shutdown) = server.shutdown() {
                        shutdown.await;
                    }
                }

                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        this.language_server_statuses.remove(&server_id);
                        cx.notify();
                    });
                }

                (root_path, orphaned_worktrees)
            })
        } else {
            Task::ready((None, Vec::new()))
        }
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
        fallback_path: Arc<Path>,
        language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        let adapter = if let Some(adapter) = language.lsp_adapter() {
            adapter
        } else {
            return;
        };

        let server_name = adapter.name();
        let stop = self.stop_language_server(worktree_id, server_name.clone(), cx);
        cx.spawn_weak(|this, mut cx| async move {
            let (original_root_path, orphaned_worktrees) = stop.await;
            if let Some(this) = this.upgrade(&cx) {
                this.update(&mut cx, |this, cx| {
                    // Attempt to restart using original server path. Fallback to passed in
                    // path if we could not retrieve the root path
                    let root_path = original_root_path
                        .map(|path_buf| Arc::from(path_buf.as_path()))
                        .unwrap_or(fallback_path);

                    this.start_language_server(worktree_id, root_path, language, cx);

                    // Lookup new server id and set it for each of the orphaned worktrees
                    if let Some(new_server_id) = this
                        .language_server_ids
                        .get(&(worktree_id, server_name.clone()))
                        .cloned()
                    {
                        for orphaned_worktree in orphaned_worktrees {
                            this.language_server_ids.insert(
                                (orphaned_worktree, server_name.clone()),
                                new_server_id.clone(),
                            );
                        }
                    }
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
        cx: &mut ModelContext<Self>,
    ) {
        adapter.process_diagnostics(&mut params);
        self.update_diagnostics(
            server_id,
            params,
            adapter.disk_based_diagnostic_sources(),
            cx,
        )
        .log_err();
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

        if !language_server_status.progress_tokens.contains(&token) {
            return;
        }

        match progress {
            lsp::WorkDoneProgress::Begin(report) => {
                if Some(token.as_str()) == disk_based_diagnostics_progress_token {
                    language_server_status.has_pending_diagnostic_updates = true;
                    self.disk_based_diagnostics_started(server_id, cx);
                    self.broadcast_language_server_update(
                        server_id,
                        proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(
                            proto::LspDiskBasedDiagnosticsUpdating {},
                        ),
                    );
                } else {
                    self.on_lsp_work_start(
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
                        proto::update_language_server::Variant::WorkStart(proto::LspWorkStart {
                            token,
                            message: report.message,
                            percentage: report.percentage.map(|p| p as u32),
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
                language_server_status.progress_tokens.remove(&token);

                if Some(token.as_str()) == disk_based_diagnostics_progress_token {
                    language_server_status.has_pending_diagnostic_updates = false;
                    self.disk_based_diagnostics_finished(server_id, cx);
                    self.broadcast_language_server_update(
                        server_id,
                        proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                            proto::LspDiskBasedDiagnosticsUpdated {},
                        ),
                    );
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
        progress: LanguageServerProgress,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            status.pending_work.insert(token, progress);
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
            let entry = status
                .pending_work
                .entry(token)
                .or_insert(LanguageServerProgress {
                    message: Default::default(),
                    percentage: Default::default(),
                    last_update_at: progress.last_update_at,
                });
            if progress.message.is_some() {
                entry.message = progress.message;
            }
            if progress.percentage.is_some() {
                entry.percentage = progress.percentage;
            }
            entry.last_update_at = progress.last_update_at;
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
        for server_state in self.language_servers.values() {
            if let LanguageServerState::Running { server, .. } = server_state {
                server
                    .notify::<lsp::notification::DidChangeConfiguration>(
                        lsp::DidChangeConfigurationParams {
                            settings: settings.clone(),
                        },
                    )
                    .ok();
            }
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
        language_server_id: usize,
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

        self.update_diagnostic_entries(
            language_server_id,
            abs_path,
            params.version,
            diagnostics,
            cx,
        )?;
        Ok(())
    }

    pub fn update_diagnostic_entries(
        &mut self,
        language_server_id: usize,
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
        if let Some(buffer) = self.get_open_buffer(&project_path, cx) {
            self.update_buffer_diagnostics(&buffer, diagnostics.clone(), version, cx)?;
        }

        let updated = worktree.update(cx, |worktree, cx| {
            worktree
                .as_local_mut()
                .ok_or_else(|| anyhow!("not a local worktree"))?
                .update_diagnostics(
                    language_server_id,
                    project_path.path.clone(),
                    diagnostics,
                    cx,
                )
        })?;
        if updated {
            cx.emit(Event::DiagnosticsUpdated {
                language_server_id,
                path: project_path,
            });
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
                                tab_size: tab_size.into(),
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
                                    tab_size: tab_size.into(),
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
    ) -> Task<Result<Vec<LocationLink>>> {
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
            for ((worktree_id, _), server_id) in self.language_server_ids.iter() {
                let worktree_id = *worktree_id;
                if let Some(worktree) = self
                    .worktree_for_id(worktree_id, cx)
                    .and_then(|worktree| worktree.read(cx).as_local())
                {
                    if let Some(LanguageServerState::Running { adapter, server }) =
                        self.language_servers.get(server_id)
                    {
                        let adapter = adapter.clone();
                        let worktree_abs_path = worktree.abs_path().clone();
                        requests.push(
                            server
                                .request::<lsp::request::WorkspaceSymbol>(
                                    lsp::WorkspaceSymbolParams {
                                        query: query.to_string(),
                                        ..Default::default()
                                    },
                                )
                                .log_err()
                                .map(move |response| {
                                    (
                                        adapter,
                                        worktree_id,
                                        worktree_abs_path,
                                        response.unwrap_or_default(),
                                    )
                                }),
                        );
                    }
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
            let language_server_id = if let Some(id) = self.language_server_ids.get(&(
                symbol.source_worktree_id,
                symbol.language_server_name.clone(),
            )) {
                *id
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
                language_server_id,
                symbol.language_server_name.clone(),
                cx,
            )
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

    pub fn hover<T: ToPointUtf16>(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Hover>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(buffer.clone(), GetHover { position }, cx)
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
            let lang_server =
                if let Some((_, server)) = self.language_server_for_buffer(source_buffer, cx) {
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
                            // For now, we can only handle additional edits if they are returned
                            // when resolving the completion, not if they are present initially.
                            if lsp_completion
                                .additional_text_edits
                                .as_ref()
                                .map_or(false, |edits| !edits.is_empty())
                            {
                                return None;
                            }

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
                                            let (range, kind) = snapshot.surrounding_word(offset);
                                            if kind == Some(CharKind::Word) {
                                                range
                                            } else {
                                                offset..offset
                                            }
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
            let lang_server = if let Some((_, server)) = self.language_server_for_buffer(buffer, cx)
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
            let lang_server = if let Some((_, server)) = self.language_server_for_buffer(buffer, cx)
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
                if let Some((adapter, server)) = self.language_server_for_buffer(buffer, cx) {
                    (adapter.clone(), server.clone())
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
                        lsp_adapter.clone(),
                        lang_server.clone(),
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
                                language_server.server_id(),
                                lsp_adapter.name(),
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
            if let Some((file, language_server)) = file.zip(
                self.language_server_for_buffer(buffer, cx)
                    .map(|(_, server)| server.clone()),
            ) {
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
        for tree in &self.worktrees {
            if let Some(tree) = tree.upgrade(cx) {
                if let Some(relative_path) = tree
                    .read(cx)
                    .as_local()
                    .and_then(|t| abs_path.strip_prefix(t.abs_path()).ok())
                {
                    return Some((tree.clone(), relative_path.into()));
                }
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
        self.metadata_changed(true, cx);
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
            self.worktrees
                .push(WorktreeHandle::Weak(worktree.downgrade()));
        }

        self.metadata_changed(true, cx);
        cx.observe_release(&worktree, |this, worktree, cx| {
            this.remove_worktree(worktree.id(), cx);
            cx.notify();
        })
        .detach();

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
                        buffer.file_updated(Arc::new(new_file), cx).detach();
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

    pub fn language_servers_running_disk_based_diagnostics<'a>(
        &'a self,
    ) -> impl 'a + Iterator<Item = usize> {
        self.language_server_statuses
            .iter()
            .filter_map(|(id, status)| {
                if status.has_pending_diagnostic_updates {
                    Some(*id)
                } else {
                    None
                }
            })
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
        self.visible_worktrees(cx).flat_map(move |worktree| {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            worktree
                .diagnostic_summaries()
                .map(move |(path, summary)| (ProjectPath { worktree_id, path }, summary))
        })
    }

    pub fn disk_based_diagnostics_started(
        &mut self,
        language_server_id: usize,
        cx: &mut ModelContext<Self>,
    ) {
        cx.emit(Event::DiskBasedDiagnosticsStarted { language_server_id });
    }

    pub fn disk_based_diagnostics_finished(
        &mut self,
        language_server_id: usize,
        cx: &mut ModelContext<Self>,
    ) {
        cx.emit(Event::DiskBasedDiagnosticsFinished { language_server_id });
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
                    let worktree =
                        Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx);
                    this.add_worktree(&worktree, cx);
                }
            }

            this.metadata_changed(true, cx);
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
                    worktree.update_from_remote(envelope.payload);
                });
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
                    cx.emit(Event::DiagnosticsUpdated {
                        language_server_id: summary.language_server_id as usize,
                        path: project_path,
                    });
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
                    has_pending_diagnostic_updates: false,
                    progress_tokens: Default::default(),
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
                    this.on_lsp_work_start(
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
                    this.disk_based_diagnostics_started(language_server_id, cx);
                })
            }
            proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(_) => {
                this.update(&mut cx, |this, cx| {
                    this.disk_based_diagnostics_finished(language_server_id, cx)
                });
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
                buffer.file_updated(Arc::new(file), cx).detach();
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

        let (saved_version, fingerprint, mtime) =
            buffer.update(&mut cx, |buffer, cx| buffer.save(cx)).await?;
        Ok(proto::BufferSaved {
            project_id,
            buffer_id,
            version: serialize_version(&saved_version),
            mtime: Some(mtime.into()),
            fingerprint,
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
                                Some(Arc::new(File::from_proto(file, worktree.clone(), cx)?)
                                    as Arc<dyn language::File>);
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
                    buffer.did_save(version, envelope.payload.fingerprint, mtime, None, cx);
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
        let payload = envelope.payload;
        let version = deserialize_version(payload.version);
        let line_ending = deserialize_line_ending(
            proto::LineEnding::from_i32(payload.line_ending)
                .ok_or_else(|| anyhow!("missing line ending"))?,
        );
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
                    buffer.did_reload(version, payload.fingerprint, line_ending, mtime, cx);
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
                .collect::<Vec<_>>();
            lsp_edits.sort_by_key(|(range, _)| range.start);

            let mut lsp_edits = lsp_edits.into_iter().peekable();
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
    ) -> Option<(&Arc<dyn LspAdapter>, &Arc<LanguageServer>)> {
        if let Some((file, language)) = File::from_dyn(buffer.file()).zip(buffer.language()) {
            let worktree_id = file.worktree_id(cx);
            let key = (worktree_id, language.lsp_adapter()?.name());

            if let Some(server_id) = self.language_server_ids.get(&key) {
                if let Some(LanguageServerState::Running { adapter, server }) =
                    self.language_servers.get(&server_id)
                {
                    return Some((adapter, server));
                }
            }
        }

        None
    }
}

impl ProjectStore {
    pub fn new(db: Arc<Db>) -> Self {
        Self {
            db,
            projects: Default::default(),
        }
    }

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
            .map(|(_, server_state)| async {
                match server_state {
                    LanguageServerState::Running { server, .. } => server.shutdown()?.await,
                    LanguageServerState::Starting(starting_server) => {
                        starting_server.await?.shutdown()?.await
                    }
                }
            })
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
