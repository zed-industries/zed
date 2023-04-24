mod ignore;
mod lsp_command;
mod lsp_glob_set;
pub mod search;
pub mod terminals;
pub mod worktree;

#[cfg(test)]
mod project_tests;

use anyhow::{anyhow, Context, Result};
use client::{proto, Client, TypedEnvelope, UserStore};
use clock::ReplicaId;
use collections::{hash_map, BTreeMap, HashMap, HashSet};
use copilot::Copilot;
use futures::{
    channel::mpsc::{self, UnboundedReceiver},
    future::{try_join_all, Shared},
    AsyncWriteExt, Future, FutureExt, StreamExt, TryFutureExt,
};
use gpui::{
    AnyModelHandle, AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task,
    UpgradeModelHandle, WeakModelHandle,
};
use language::{
    point_to_lsp,
    proto::{
        deserialize_anchor, deserialize_fingerprint, deserialize_line_ending, deserialize_version,
        serialize_anchor, serialize_version,
    },
    range_from_lsp, range_to_lsp, Anchor, Bias, Buffer, CachedLspAdapter, CodeAction, CodeLabel,
    Completion, Diagnostic, DiagnosticEntry, DiagnosticSet, Diff, Event as BufferEvent, File as _,
    Language, LanguageRegistry, LanguageServerName, LocalFile, OffsetRangeExt, Operation, Patch,
    PendingLanguageServer, PointUtf16, RopeFingerprint, TextBufferSnapshot, ToOffset, ToPointUtf16,
    Transaction, Unclipped,
};
use lsp::{
    DiagnosticSeverity, DiagnosticTag, DidChangeWatchedFilesRegistrationOptions,
    DocumentHighlightKind, LanguageServer, LanguageServerId, LanguageString, MarkedString,
};
use lsp_command::*;
use lsp_glob_set::LspGlobSet;
use postage::watch;
use rand::prelude::*;
use search::SearchQuery;
use serde::Serialize;
use settings::{FormatOnSave, Formatter, Settings};
use sha2::{Digest, Sha256};
use similar::{ChangeTag, TextDiff};
use std::{
    cell::RefCell,
    cmp::{self, Ordering},
    convert::TryInto,
    hash::Hash,
    mem,
    num::NonZeroU32,
    ops::Range,
    path::{Component, Path, PathBuf},
    rc::Rc,
    str,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::{Duration, Instant, SystemTime},
};
use terminals::Terminals;

use util::{debug_panic, defer, merge_json_value_into, post_inc, ResultExt, TryFutureExt as _};

pub use fs::*;
pub use worktree::*;

pub trait Item {
    fn entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId>;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
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
    buffer_ordered_messages_tx: mpsc::UnboundedSender<BufferOrderedMessage>,
    languages: Arc<LanguageRegistry>,
    language_servers: HashMap<LanguageServerId, LanguageServerState>,
    language_server_ids: HashMap<(WorktreeId, LanguageServerName), LanguageServerId>,
    language_server_statuses: BTreeMap<LanguageServerId, LanguageServerStatus>,
    last_workspace_edits_by_language_server: HashMap<LanguageServerId, ProjectTransaction>,
    client: Arc<client::Client>,
    next_entry_id: Arc<AtomicUsize>,
    join_project_response_message_id: u32,
    next_diagnostic_group_id: usize,
    user_store: ModelHandle<UserStore>,
    fs: Arc<dyn Fs>,
    client_state: Option<ProjectClientState>,
    collaborators: HashMap<proto::PeerId, Collaborator>,
    client_subscriptions: Vec<client::Subscription>,
    _subscriptions: Vec<gpui::Subscription>,
    opened_buffer: (watch::Sender<()>, watch::Receiver<()>),
    shared_buffers: HashMap<proto::PeerId, HashSet<u64>>,
    #[allow(clippy::type_complexity)]
    loading_buffers_by_path: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<ModelHandle<Buffer>, Arc<anyhow::Error>>>>,
    >,
    #[allow(clippy::type_complexity)]
    loading_local_worktrees:
        HashMap<Arc<Path>, Shared<Task<Result<ModelHandle<Worktree>, Arc<anyhow::Error>>>>>,
    opened_buffers: HashMap<u64, OpenBuffer>,
    /// A mapping from a buffer ID to None means that we've started waiting for an ID but haven't finished loading it.
    /// Used for re-issuing buffer requests when peers temporarily disconnect
    incomplete_remote_buffers: HashMap<u64, Option<ModelHandle<Buffer>>>,
    buffer_snapshots: HashMap<u64, HashMap<LanguageServerId, Vec<LspBufferSnapshot>>>, // buffer_id -> server_id -> vec of snapshots
    buffers_being_formatted: HashSet<usize>,
    nonce: u128,
    _maintain_buffer_languages: Task<()>,
    _maintain_workspace_config: Task<()>,
    terminals: Terminals,
    copilot_enabled: bool,
}

struct LspBufferSnapshot {
    version: i32,
    snapshot: TextBufferSnapshot,
}

/// Message ordered with respect to buffer operations
enum BufferOrderedMessage {
    Operation {
        buffer_id: u64,
        operation: proto::Operation,
    },
    LanguageServerUpdate {
        language_server_id: LanguageServerId,
        message: proto::update_language_server::Variant,
    },
    Resync,
}

enum LocalProjectUpdate {
    WorktreesChanged,
    CreateBufferForPeer {
        peer_id: proto::PeerId,
        buffer_id: u64,
    },
}

enum OpenBuffer {
    Strong(ModelHandle<Buffer>),
    Weak(WeakModelHandle<Buffer>),
    Operations(Vec<Operation>),
}

enum WorktreeHandle {
    Strong(ModelHandle<Worktree>),
    Weak(WeakModelHandle<Worktree>),
}

enum ProjectClientState {
    Local {
        remote_id: u64,
        updates_tx: mpsc::UnboundedSender<LocalProjectUpdate>,
        _send_updates: Task<()>,
    },
    Remote {
        sharing_has_stopped: bool,
        remote_id: u64,
        replica_id: ReplicaId,
    },
}

#[derive(Clone, Debug)]
pub struct Collaborator {
    pub peer_id: proto::PeerId,
    pub replica_id: ReplicaId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    ActiveEntryChanged(Option<ProjectEntryId>),
    WorktreeAdded,
    WorktreeRemoved(WorktreeId),
    DiskBasedDiagnosticsStarted {
        language_server_id: LanguageServerId,
    },
    DiskBasedDiagnosticsFinished {
        language_server_id: LanguageServerId,
    },
    DiagnosticsUpdated {
        path: ProjectPath,
        language_server_id: LanguageServerId,
    },
    RemoteIdChanged(Option<u64>),
    DisconnectedFromHost,
    Closed,
    CollaboratorUpdated {
        old_peer_id: proto::PeerId,
        new_peer_id: proto::PeerId,
    },
    CollaboratorLeft(proto::PeerId),
}

pub enum LanguageServerState {
    Starting(Task<Option<Arc<LanguageServer>>>),
    Running {
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        server: Arc<LanguageServer>,
        watched_paths: LspGlobSet,
        simulate_disk_based_diagnostics_completion: Option<Task<()>>,
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
    pub language_server_name: LanguageServerName,
    pub source_worktree_id: WorktreeId,
    pub path: ProjectPath,
    pub label: CodeLabel,
    pub name: String,
    pub kind: lsp::SymbolKind,
    pub range: Range<Unclipped<PointUtf16>>,
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

    pub fn to_proto(
        &self,
        language_server_id: LanguageServerId,
        path: &Path,
    ) -> proto::DiagnosticSummary {
        proto::DiagnosticSummary {
            path: path.to_string_lossy().to_string(),
            language_server_id: language_server_id.0 as u64,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatTrigger {
    Save,
    Manual,
}

impl FormatTrigger {
    fn from_proto(value: i32) -> FormatTrigger {
        match value {
            0 => FormatTrigger::Save,
            1 => FormatTrigger::Manual,
            _ => FormatTrigger::Save,
        }
    }
}

impl Project {
    pub fn init(client: &Arc<Client>) {
        client.add_model_message_handler(Self::handle_add_collaborator);
        client.add_model_message_handler(Self::handle_update_project_collaborator);
        client.add_model_message_handler(Self::handle_remove_collaborator);
        client.add_model_message_handler(Self::handle_buffer_reloaded);
        client.add_model_message_handler(Self::handle_buffer_saved);
        client.add_model_message_handler(Self::handle_start_language_server);
        client.add_model_message_handler(Self::handle_update_language_server);
        client.add_model_message_handler(Self::handle_update_project);
        client.add_model_message_handler(Self::handle_unshare_project);
        client.add_model_message_handler(Self::handle_create_buffer_for_peer);
        client.add_model_message_handler(Self::handle_update_buffer_file);
        client.add_model_request_handler(Self::handle_update_buffer);
        client.add_model_message_handler(Self::handle_update_diagnostic_summary);
        client.add_model_message_handler(Self::handle_update_worktree);
        client.add_model_request_handler(Self::handle_create_project_entry);
        client.add_model_request_handler(Self::handle_rename_project_entry);
        client.add_model_request_handler(Self::handle_copy_project_entry);
        client.add_model_request_handler(Self::handle_delete_project_entry);
        client.add_model_request_handler(Self::handle_apply_additional_edits_for_completion);
        client.add_model_request_handler(Self::handle_apply_code_action);
        client.add_model_request_handler(Self::handle_reload_buffers);
        client.add_model_request_handler(Self::handle_synchronize_buffers);
        client.add_model_request_handler(Self::handle_format_buffers);
        client.add_model_request_handler(Self::handle_lsp_command::<GetCodeActions>);
        client.add_model_request_handler(Self::handle_lsp_command::<GetCompletions>);
        client.add_model_request_handler(Self::handle_lsp_command::<GetHover>);
        client.add_model_request_handler(Self::handle_lsp_command::<GetDefinition>);
        client.add_model_request_handler(Self::handle_lsp_command::<GetTypeDefinition>);
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
        client.add_model_message_handler(Self::handle_update_diff_base);
    }

    pub fn local(
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut AppContext,
    ) -> ModelHandle<Self> {
        cx.add_model(|cx: &mut ModelContext<Self>| {
            let (tx, rx) = mpsc::unbounded();
            cx.spawn_weak(|this, cx| Self::send_buffer_ordered_messages(this, rx, cx))
                .detach();
            Self {
                worktrees: Default::default(),
                buffer_ordered_messages_tx: tx,
                collaborators: Default::default(),
                opened_buffers: Default::default(),
                shared_buffers: Default::default(),
                incomplete_remote_buffers: Default::default(),
                loading_buffers_by_path: Default::default(),
                loading_local_worktrees: Default::default(),
                buffer_snapshots: Default::default(),
                join_project_response_message_id: 0,
                client_state: None,
                opened_buffer: watch::channel(),
                client_subscriptions: Vec::new(),
                _subscriptions: vec![cx.observe_global::<Settings, _>(Self::on_settings_changed)],
                _maintain_buffer_languages: Self::maintain_buffer_languages(&languages, cx),
                _maintain_workspace_config: Self::maintain_workspace_config(languages.clone(), cx),
                active_entry: None,
                languages,
                client,
                user_store,
                fs,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                language_servers: Default::default(),
                language_server_ids: Default::default(),
                language_server_statuses: Default::default(),
                last_workspace_edits_by_language_server: Default::default(),
                buffers_being_formatted: Default::default(),
                nonce: StdRng::from_entropy().gen(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                copilot_enabled: Copilot::global(cx).is_some(),
            }
        })
    }

    pub async fn remote(
        remote_id: u64,
        client: Arc<Client>,
        user_store: ModelHandle<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        mut cx: AsyncAppContext,
    ) -> Result<ModelHandle<Self>> {
        client.authenticate_and_connect(true, &cx).await?;

        let subscription = client.subscribe_to_entity(remote_id)?;
        let response = client
            .request_envelope(proto::JoinProject {
                project_id: remote_id,
            })
            .await?;
        let this = cx.add_model(|cx| {
            let replica_id = response.payload.replica_id as ReplicaId;

            let mut worktrees = Vec::new();
            for worktree in response.payload.worktrees {
                let worktree = cx.update(|cx| {
                    Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx)
                });
                worktrees.push(worktree);
            }

            let (tx, rx) = mpsc::unbounded();
            cx.spawn_weak(|this, cx| Self::send_buffer_ordered_messages(this, rx, cx))
                .detach();
            let mut this = Self {
                worktrees: Vec::new(),
                buffer_ordered_messages_tx: tx,
                loading_buffers_by_path: Default::default(),
                opened_buffer: watch::channel(),
                shared_buffers: Default::default(),
                incomplete_remote_buffers: Default::default(),
                loading_local_worktrees: Default::default(),
                active_entry: None,
                collaborators: Default::default(),
                join_project_response_message_id: response.message_id,
                _maintain_buffer_languages: Self::maintain_buffer_languages(&languages, cx),
                _maintain_workspace_config: Self::maintain_workspace_config(languages.clone(), cx),
                languages,
                user_store: user_store.clone(),
                fs,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                client_subscriptions: Default::default(),
                _subscriptions: Default::default(),
                client: client.clone(),
                client_state: Some(ProjectClientState::Remote {
                    sharing_has_stopped: false,
                    remote_id,
                    replica_id,
                }),
                language_servers: Default::default(),
                language_server_ids: Default::default(),
                language_server_statuses: response
                    .payload
                    .language_servers
                    .into_iter()
                    .map(|server| {
                        (
                            LanguageServerId(server.id as usize),
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
                opened_buffers: Default::default(),
                buffers_being_formatted: Default::default(),
                buffer_snapshots: Default::default(),
                nonce: StdRng::from_entropy().gen(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                copilot_enabled: Copilot::global(cx).is_some(),
            };
            for worktree in worktrees {
                let _ = this.add_worktree(&worktree, cx);
            }
            this
        });
        let subscription = subscription.set_model(&this, &mut cx);

        let user_ids = response
            .payload
            .collaborators
            .iter()
            .map(|peer| peer.user_id)
            .collect();
        user_store
            .update(&mut cx, |user_store, cx| user_store.get_users(user_ids, cx))
            .await?;

        this.update(&mut cx, |this, cx| {
            this.set_collaborators_from_proto(response.payload.collaborators, cx)?;
            this.client_subscriptions.push(subscription);
            anyhow::Ok(())
        })?;

        Ok(this)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn test(
        fs: Arc<dyn Fs>,
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut gpui::TestAppContext,
    ) -> ModelHandle<Project> {
        if !cx.read(|cx| cx.has_global::<Settings>()) {
            cx.update(|cx| {
                cx.set_global(Settings::test(cx));
            });
        }

        let mut languages = LanguageRegistry::test();
        languages.set_executor(cx.background());
        let http_client = util::http::FakeHttpClient::with_404_response();
        let client = cx.update(|cx| client::Client::new(http_client.clone(), cx));
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));
        let project =
            cx.update(|cx| Project::local(client, user_store, Arc::new(languages), fs, cx));
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
            for lsp_adapter in language.lsp_adapters() {
                if !settings.enable_language_server(Some(&language.name())) {
                    let lsp_name = &lsp_adapter.name;
                    for (worktree_id, started_lsp_name) in self.language_server_ids.keys() {
                        if lsp_name == started_lsp_name {
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
            self.start_language_servers(worktree_id, worktree_path, language, cx);
        }

        if !self.copilot_enabled && Copilot::global(cx).is_some() {
            self.copilot_enabled = true;
            for buffer in self.opened_buffers.values() {
                if let Some(buffer) = buffer.upgrade(cx) {
                    self.register_buffer_with_copilot(&buffer, cx);
                }
            }
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn opened_buffers(&self, cx: &AppContext) -> Vec<ModelHandle<Buffer>> {
        self.opened_buffers
            .values()
            .filter_map(|b| b.upgrade(cx))
            .collect()
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

    pub fn remote_id(&self) -> Option<u64> {
        match self.client_state.as_ref()? {
            ProjectClientState::Local { remote_id, .. }
            | ProjectClientState::Remote { remote_id, .. } => Some(*remote_id),
        }
    }

    pub fn replica_id(&self) -> ReplicaId {
        match &self.client_state {
            Some(ProjectClientState::Remote { replica_id, .. }) => *replica_id,
            _ => 0,
        }
    }

    fn metadata_changed(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(ProjectClientState::Local { updates_tx, .. }) = &mut self.client_state {
            updates_tx
                .unbounded_send(LocalProjectUpdate::WorktreesChanged)
                .ok();
        }
        cx.notify();
    }

    pub fn collaborators(&self) -> &HashMap<proto::PeerId, Collaborator> {
        &self.collaborators
    }

    /// Collect all worktrees, including ones that don't appear in the project panel
    pub fn worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + DoubleEndedIterator<Item = ModelHandle<Worktree>> {
        self.worktrees
            .iter()
            .filter_map(move |worktree| worktree.upgrade(cx))
    }

    /// Collect all user-visible worktrees, the ones that appear in the project panel
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
        paths.iter().all(|path| self.contains_path(path, cx))
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
                        path: project_path.path.to_string_lossy().into(),
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
                        new_path: new_path.to_string_lossy().into(),
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
                        new_path: new_path.to_string_lossy().into(),
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

    pub fn shared(&mut self, project_id: u64, cx: &mut ModelContext<Self>) -> Result<()> {
        if self.client_state.is_some() {
            return Err(anyhow!("project was already shared"));
        }
        self.client_subscriptions.push(
            self.client
                .subscribe_to_entity(project_id)?
                .set_model(&cx.handle(), &mut cx.to_async()),
        );

        for open_buffer in self.opened_buffers.values_mut() {
            match open_buffer {
                OpenBuffer::Strong(_) => {}
                OpenBuffer::Weak(buffer) => {
                    if let Some(buffer) = buffer.upgrade(cx) {
                        *open_buffer = OpenBuffer::Strong(buffer);
                    }
                }
                OpenBuffer::Operations(_) => unreachable!(),
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

        for (server_id, status) in &self.language_server_statuses {
            self.client
                .send(proto::StartLanguageServer {
                    project_id,
                    server: Some(proto::LanguageServer {
                        id: server_id.0 as u64,
                        name: status.name.clone(),
                    }),
                })
                .log_err();
        }

        let (updates_tx, mut updates_rx) = mpsc::unbounded();
        let client = self.client.clone();
        self.client_state = Some(ProjectClientState::Local {
            remote_id: project_id,
            updates_tx,
            _send_updates: cx.spawn_weak(move |this, mut cx| async move {
                while let Some(update) = updates_rx.next().await {
                    let Some(this) = this.upgrade(&cx) else { break };

                    match update {
                        LocalProjectUpdate::WorktreesChanged => {
                            let worktrees = this
                                .read_with(&cx, |this, cx| this.worktrees(cx).collect::<Vec<_>>());
                            let update_project = this
                                .read_with(&cx, |this, cx| {
                                    this.client.request(proto::UpdateProject {
                                        project_id,
                                        worktrees: this.worktree_metadata_protos(cx),
                                    })
                                })
                                .await;
                            if update_project.is_ok() {
                                for worktree in worktrees {
                                    worktree.update(&mut cx, |worktree, cx| {
                                        let worktree = worktree.as_local_mut().unwrap();
                                        worktree.share(project_id, cx).detach_and_log_err(cx)
                                    });
                                }
                            }
                        }
                        LocalProjectUpdate::CreateBufferForPeer { peer_id, buffer_id } => {
                            let buffer = this.update(&mut cx, |this, _| {
                                let buffer = this.opened_buffers.get(&buffer_id).unwrap();
                                let shared_buffers =
                                    this.shared_buffers.entry(peer_id).or_default();
                                if shared_buffers.insert(buffer_id) {
                                    if let OpenBuffer::Strong(buffer) = buffer {
                                        Some(buffer.clone())
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            });

                            let Some(buffer) = buffer else { continue };
                            let operations =
                                buffer.read_with(&cx, |b, cx| b.serialize_ops(None, cx));
                            let operations = operations.await;
                            let state = buffer.read_with(&cx, |buffer, _| buffer.to_proto());

                            let initial_state = proto::CreateBufferForPeer {
                                project_id,
                                peer_id: Some(peer_id),
                                variant: Some(proto::create_buffer_for_peer::Variant::State(state)),
                            };
                            if client.send(initial_state).log_err().is_some() {
                                let client = client.clone();
                                cx.background()
                                    .spawn(async move {
                                        let mut chunks = split_operations(operations).peekable();
                                        while let Some(chunk) = chunks.next() {
                                            let is_last = chunks.peek().is_none();
                                            client.send(proto::CreateBufferForPeer {
                                                project_id,
                                                peer_id: Some(peer_id),
                                                variant: Some(
                                                    proto::create_buffer_for_peer::Variant::Chunk(
                                                        proto::BufferChunk {
                                                            buffer_id,
                                                            operations: chunk,
                                                            is_last,
                                                        },
                                                    ),
                                                ),
                                            })?;
                                        }
                                        anyhow::Ok(())
                                    })
                                    .await
                                    .log_err();
                            }
                        }
                    }
                }
            }),
        });

        self.metadata_changed(cx);
        cx.emit(Event::RemoteIdChanged(Some(project_id)));
        cx.notify();
        Ok(())
    }

    pub fn reshared(
        &mut self,
        message: proto::ResharedProject,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.shared_buffers.clear();
        self.set_collaborators_from_proto(message.collaborators, cx)?;
        self.metadata_changed(cx);
        Ok(())
    }

    pub fn rejoined(
        &mut self,
        message: proto::RejoinedProject,
        message_id: u32,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.join_project_response_message_id = message_id;
        self.set_worktrees_from_proto(message.worktrees, cx)?;
        self.set_collaborators_from_proto(message.collaborators, cx)?;
        self.language_server_statuses = message
            .language_servers
            .into_iter()
            .map(|server| {
                (
                    LanguageServerId(server.id as usize),
                    LanguageServerStatus {
                        name: server.name,
                        pending_work: Default::default(),
                        has_pending_diagnostic_updates: false,
                        progress_tokens: Default::default(),
                    },
                )
            })
            .collect();
        self.buffer_ordered_messages_tx
            .unbounded_send(BufferOrderedMessage::Resync)
            .unwrap();
        cx.notify();
        Ok(())
    }

    pub fn unshare(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        self.unshare_internal(cx)?;
        self.metadata_changed(cx);
        cx.notify();
        Ok(())
    }

    fn unshare_internal(&mut self, cx: &mut AppContext) -> Result<()> {
        if self.is_remote() {
            return Err(anyhow!("attempted to unshare a remote project"));
        }

        if let Some(ProjectClientState::Local { remote_id, .. }) = self.client_state.take() {
            self.collaborators.clear();
            self.shared_buffers.clear();
            self.client_subscriptions.clear();

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
                // Wake up any tasks waiting for peers' edits to this buffer.
                if let Some(buffer) = open_buffer.upgrade(cx) {
                    buffer.update(cx, |buffer, _| buffer.give_up_waiting());
                }

                if let OpenBuffer::Strong(buffer) = open_buffer {
                    *open_buffer = OpenBuffer::Weak(buffer.downgrade());
                }
            }

            self.client.send(proto::UnshareProject {
                project_id: remote_id,
            })?;

            Ok(())
        } else {
            Err(anyhow!("attempted to unshare an unshared project"))
        }
    }

    pub fn disconnected_from_host(&mut self, cx: &mut ModelContext<Self>) {
        self.disconnected_from_host_internal(cx);
        cx.emit(Event::DisconnectedFromHost);
        cx.notify();
    }

    fn disconnected_from_host_internal(&mut self, cx: &mut AppContext) {
        if let Some(ProjectClientState::Remote {
            sharing_has_stopped,
            ..
        }) = &mut self.client_state
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

            for open_buffer in self.opened_buffers.values_mut() {
                // Wake up any tasks waiting for peers' edits to this buffer.
                if let Some(buffer) = open_buffer.upgrade(cx) {
                    buffer.update(cx, |buffer, _| buffer.give_up_waiting());
                }

                if let OpenBuffer::Strong(buffer) = open_buffer {
                    *open_buffer = OpenBuffer::Weak(buffer.downgrade());
                }
            }

            // Wake up all futures currently waiting on a buffer to get opened,
            // to give them a chance to fail now that we've disconnected.
            *self.opened_buffer.0.borrow_mut() = ();
        }
    }

    pub fn close(&mut self, cx: &mut ModelContext<Self>) {
        cx.emit(Event::Closed);
    }

    pub fn is_read_only(&self) -> bool {
        match &self.client_state {
            Some(ProjectClientState::Remote {
                sharing_has_stopped,
                ..
            }) => *sharing_has_stopped,
            _ => false,
        }
    }

    pub fn is_local(&self) -> bool {
        match &self.client_state {
            Some(ProjectClientState::Remote { .. }) => false,
            _ => true,
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
                .with_language(language.unwrap_or_else(|| language::PLAIN_TEXT.clone()), cx)
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

            let buffer: &AnyModelHandle = &buffer;
            Ok((project_entry_id, buffer.clone()))
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

        let mut loading_watch = match self.loading_buffers_by_path.entry(project_path.clone()) {
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
                        this.loading_buffers_by_path.remove(&project_path);
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
            this.update(&mut cx, |this, cx| {
                this.wait_for_remote_buffer(response.buffer_id, cx)
            })
            .await
        })
    }

    /// LanguageServerName is owned, because it is inserted into a map
    fn open_local_buffer_via_lsp(
        &mut self,
        abs_path: lsp::Url,
        language_server_id: LanguageServerId,
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
                let buffer_id = request.await?.buffer_id;
                this.update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(buffer_id, cx)
                })
                .await
            })
        } else {
            Task::ready(Err(anyhow!("cannot open buffer while disconnected")))
        }
    }

    pub fn save_buffers(
        &self,
        buffers: HashSet<ModelHandle<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        cx.spawn(|this, mut cx| async move {
            let save_tasks = buffers
                .into_iter()
                .map(|buffer| this.update(&mut cx, |this, cx| this.save_buffer(buffer, cx)));
            try_join_all(save_tasks).await?;
            Ok(())
        })
    }

    pub fn save_buffer(
        &self,
        buffer: ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(clock::Global, RopeFingerprint, SystemTime)>> {
        let Some(file) = File::from_dyn(buffer.read(cx).file()) else {
            return Task::ready(Err(anyhow!("buffer doesn't have a file")));
        };
        let worktree = file.worktree.clone();
        let path = file.path.clone();
        worktree.update(cx, |worktree, cx| match worktree {
            Worktree::Local(worktree) => worktree.save_buffer(buffer, path, false, cx),
            Worktree::Remote(worktree) => worktree.save_buffer(buffer, cx),
        })
    }

    pub fn save_buffer_as(
        &mut self,
        buffer: ModelHandle<Buffer>,
        abs_path: PathBuf,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let worktree_task = self.find_or_create_local_worktree(&abs_path, true, cx);
        let old_file = File::from_dyn(buffer.read(cx).file())
            .filter(|f| f.is_local())
            .cloned();
        cx.spawn(|this, mut cx| async move {
            if let Some(old_file) = &old_file {
                this.update(&mut cx, |this, cx| {
                    this.unregister_buffer_from_language_servers(&buffer, old_file, cx);
                });
            }
            let (worktree, path) = worktree_task.await?;
            worktree
                .update(&mut cx, |worktree, cx| match worktree {
                    Worktree::Local(worktree) => {
                        worktree.save_buffer(buffer.clone(), path.into(), true, cx)
                    }
                    Worktree::Remote(_) => panic!("cannot remote buffers as new files"),
                })
                .await?;
            this.update(&mut cx, |this, cx| {
                this.detect_language_for_buffer(&buffer, cx);
                this.register_buffer_with_language_servers(&buffer, cx);
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
        buffer.update(cx, |buffer, _| {
            buffer.set_language_registry(self.languages.clone())
        });

        let remote_id = buffer.read(cx).remote_id();
        let is_remote = self.is_remote();
        let open_buffer = if is_remote || self.is_shared() {
            OpenBuffer::Strong(buffer.clone())
        } else {
            OpenBuffer::Weak(buffer.downgrade())
        };

        match self.opened_buffers.entry(remote_id) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(open_buffer);
            }
            hash_map::Entry::Occupied(mut entry) => {
                if let OpenBuffer::Operations(operations) = entry.get_mut() {
                    buffer.update(cx, |b, cx| b.apply_ops(operations.drain(..), cx))?;
                } else if entry.get().upgrade(cx).is_some() {
                    if is_remote {
                        return Ok(());
                    } else {
                        debug_panic!("buffer {} was already registered", remote_id);
                        Err(anyhow!("buffer {} was already registered", remote_id))?;
                    }
                }
                entry.insert(open_buffer);
            }
        }
        cx.subscribe(buffer, |this, buffer, event, cx| {
            this.on_buffer_event(buffer, event, cx);
        })
        .detach();

        self.detect_language_for_buffer(buffer, cx);
        self.register_buffer_with_language_servers(buffer, cx);
        self.register_buffer_with_copilot(buffer, cx);
        cx.observe_release(buffer, |this, buffer, cx| {
            if let Some(file) = File::from_dyn(buffer.file()) {
                if file.is_local() {
                    let uri = lsp::Url::from_file_path(file.abs_path(cx)).unwrap();
                    for server in this.language_servers_for_buffer(buffer, cx) {
                        server
                            .1
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

        *self.opened_buffer.0.borrow_mut() = ();
        Ok(())
    }

    fn register_buffer_with_language_servers(
        &mut self,
        buffer_handle: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id();

        if let Some(file) = File::from_dyn(buffer.file()) {
            if !file.is_local() {
                return;
            }

            let uri = lsp::Url::from_file_path(file.abs_path(cx)).unwrap();
            let initial_snapshot = buffer.text_snapshot();
            let language = buffer.language().cloned();
            let worktree_id = file.worktree_id(cx);

            if let Some(local_worktree) = file.worktree.read(cx).as_local() {
                for (server_id, diagnostics) in local_worktree.diagnostics_for_path(file.path()) {
                    self.update_buffer_diagnostics(buffer_handle, server_id, None, diagnostics, cx)
                        .log_err();
                }
            }

            if let Some(language) = language {
                for adapter in language.lsp_adapters() {
                    let language_id = adapter.language_ids.get(language.name().as_ref()).cloned();
                    let server = self
                        .language_server_ids
                        .get(&(worktree_id, adapter.name.clone()))
                        .and_then(|id| self.language_servers.get(id))
                        .and_then(|server_state| {
                            if let LanguageServerState::Running { server, .. } = server_state {
                                Some(server.clone())
                            } else {
                                None
                            }
                        });
                    let server = match server {
                        Some(server) => server,
                        None => continue,
                    };

                    server
                        .notify::<lsp::notification::DidOpenTextDocument>(
                            lsp::DidOpenTextDocumentParams {
                                text_document: lsp::TextDocumentItem::new(
                                    uri.clone(),
                                    language_id.unwrap_or_default(),
                                    0,
                                    initial_snapshot.text(),
                                ),
                            },
                        )
                        .log_err();

                    buffer_handle.update(cx, |buffer, cx| {
                        buffer.set_completion_triggers(
                            server
                                .capabilities()
                                .completion_provider
                                .as_ref()
                                .and_then(|provider| provider.trigger_characters.clone())
                                .unwrap_or_default(),
                            cx,
                        );
                    });

                    let snapshot = LspBufferSnapshot {
                        version: 0,
                        snapshot: initial_snapshot.clone(),
                    };
                    self.buffer_snapshots
                        .entry(buffer_id)
                        .or_default()
                        .insert(server.server_id(), vec![snapshot]);
                }
            }
        }
    }

    fn unregister_buffer_from_language_servers(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        old_file: &File,
        cx: &mut ModelContext<Self>,
    ) {
        let old_path = match old_file.as_local() {
            Some(local) => local.abs_path(cx),
            None => return,
        };

        buffer.update(cx, |buffer, cx| {
            let worktree_id = old_file.worktree_id(cx);
            let ids = &self.language_server_ids;

            let language = buffer.language().cloned();
            let adapters = language.iter().flat_map(|language| language.lsp_adapters());
            for &server_id in adapters.flat_map(|a| ids.get(&(worktree_id, a.name.clone()))) {
                buffer.update_diagnostics(server_id, Default::default(), cx);
            }

            self.buffer_snapshots.remove(&buffer.remote_id());
            let file_url = lsp::Url::from_file_path(old_path).unwrap();
            for (_, language_server) in self.language_servers_for_buffer(buffer, cx) {
                language_server
                    .notify::<lsp::notification::DidCloseTextDocument>(
                        lsp::DidCloseTextDocumentParams {
                            text_document: lsp::TextDocumentIdentifier::new(file_url.clone()),
                        },
                    )
                    .log_err();
            }
        });
    }

    fn register_buffer_with_copilot(
        &self,
        buffer_handle: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(copilot) = Copilot::global(cx) {
            copilot.update(cx, |copilot, cx| copilot.register_buffer(buffer_handle, cx));
        }
    }

    async fn send_buffer_ordered_messages(
        this: WeakModelHandle<Self>,
        rx: UnboundedReceiver<BufferOrderedMessage>,
        mut cx: AsyncAppContext,
    ) -> Option<()> {
        const MAX_BATCH_SIZE: usize = 128;

        let mut operations_by_buffer_id = HashMap::default();
        async fn flush_operations(
            this: &ModelHandle<Project>,
            operations_by_buffer_id: &mut HashMap<u64, Vec<proto::Operation>>,
            needs_resync_with_host: &mut bool,
            is_local: bool,
            cx: &AsyncAppContext,
        ) {
            for (buffer_id, operations) in operations_by_buffer_id.drain() {
                let request = this.read_with(cx, |this, _| {
                    let project_id = this.remote_id()?;
                    Some(this.client.request(proto::UpdateBuffer {
                        buffer_id,
                        project_id,
                        operations,
                    }))
                });
                if let Some(request) = request {
                    if request.await.is_err() && !is_local {
                        *needs_resync_with_host = true;
                        break;
                    }
                }
            }
        }

        let mut needs_resync_with_host = false;
        let mut changes = rx.ready_chunks(MAX_BATCH_SIZE);

        while let Some(changes) = changes.next().await {
            let this = this.upgrade(&mut cx)?;
            let is_local = this.read_with(&cx, |this, _| this.is_local());

            for change in changes {
                match change {
                    BufferOrderedMessage::Operation {
                        buffer_id,
                        operation,
                    } => {
                        if needs_resync_with_host {
                            continue;
                        }

                        operations_by_buffer_id
                            .entry(buffer_id)
                            .or_insert(Vec::new())
                            .push(operation);
                    }

                    BufferOrderedMessage::Resync => {
                        operations_by_buffer_id.clear();
                        if this
                            .update(&mut cx, |this, cx| this.synchronize_remote_buffers(cx))
                            .await
                            .is_ok()
                        {
                            needs_resync_with_host = false;
                        }
                    }

                    BufferOrderedMessage::LanguageServerUpdate {
                        language_server_id,
                        message,
                    } => {
                        flush_operations(
                            &this,
                            &mut operations_by_buffer_id,
                            &mut needs_resync_with_host,
                            is_local,
                            &cx,
                        )
                        .await;

                        this.read_with(&cx, |this, _| {
                            if let Some(project_id) = this.remote_id() {
                                this.client
                                    .send(proto::UpdateLanguageServer {
                                        project_id,
                                        language_server_id: language_server_id.0 as u64,
                                        variant: Some(message),
                                    })
                                    .log_err();
                            }
                        });
                    }
                }
            }

            flush_operations(
                &this,
                &mut operations_by_buffer_id,
                &mut needs_resync_with_host,
                is_local,
                &cx,
            )
            .await;
        }

        None
    }

    fn on_buffer_event(
        &mut self,
        buffer: ModelHandle<Buffer>,
        event: &BufferEvent,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        match event {
            BufferEvent::Operation(operation) => {
                self.buffer_ordered_messages_tx
                    .unbounded_send(BufferOrderedMessage::Operation {
                        buffer_id: buffer.read(cx).remote_id(),
                        operation: language::proto::serialize_operation(operation),
                    })
                    .ok();
            }

            BufferEvent::Edited { .. } => {
                let buffer = buffer.read(cx);
                let file = File::from_dyn(buffer.file())?;
                let abs_path = file.as_local()?.abs_path(cx);
                let uri = lsp::Url::from_file_path(abs_path).unwrap();
                let next_snapshot = buffer.text_snapshot();

                let language_servers: Vec<_> = self
                    .language_servers_iter_for_buffer(buffer, cx)
                    .map(|i| i.1.clone())
                    .collect();

                for language_server in language_servers {
                    let language_server = language_server.clone();

                    let buffer_snapshots = self
                        .buffer_snapshots
                        .get_mut(&buffer.remote_id())
                        .and_then(|m| m.get_mut(&language_server.server_id()))?;
                    let previous_snapshot = buffer_snapshots.last()?;
                    let next_version = previous_snapshot.version + 1;

                    let content_changes = buffer
                        .edits_since::<(PointUtf16, usize)>(previous_snapshot.snapshot.version())
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

                    buffer_snapshots.push(LspBufferSnapshot {
                        version: next_version,
                        snapshot: next_snapshot.clone(),
                    });

                    language_server
                        .notify::<lsp::notification::DidChangeTextDocument>(
                            lsp::DidChangeTextDocumentParams {
                                text_document: lsp::VersionedTextDocumentIdentifier::new(
                                    uri.clone(),
                                    next_version,
                                ),
                                content_changes,
                            },
                        )
                        .log_err();
                }
            }

            BufferEvent::Saved => {
                let file = File::from_dyn(buffer.read(cx).file())?;
                let worktree_id = file.worktree_id(cx);
                let abs_path = file.as_local()?.abs_path(cx);
                let text_document = lsp::TextDocumentIdentifier {
                    uri: lsp::Url::from_file_path(abs_path).unwrap(),
                };

                for (_, _, server) in self.language_servers_for_worktree(worktree_id) {
                    server
                        .notify::<lsp::notification::DidSaveTextDocument>(
                            lsp::DidSaveTextDocumentParams {
                                text_document: text_document.clone(),
                                text: None,
                            },
                        )
                        .log_err();
                }

                let language_server_ids = self.language_server_ids_for_buffer(buffer.read(cx), cx);
                for language_server_id in language_server_ids {
                    if let Some(LanguageServerState::Running {
                        adapter,
                        simulate_disk_based_diagnostics_completion,
                        ..
                    }) = self.language_servers.get_mut(&language_server_id)
                    {
                        // After saving a buffer using a language server that doesn't provide
                        // a disk-based progress token, kick off a timer that will reset every
                        // time the buffer is saved. If the timer eventually fires, simulate
                        // disk-based diagnostics being finished so that other pieces of UI
                        // (e.g., project diagnostics view, diagnostic status bar) can update.
                        // We don't emit an event right away because the language server might take
                        // some time to publish diagnostics.
                        if adapter.disk_based_diagnostics_progress_token.is_none() {
                            const DISK_BASED_DIAGNOSTICS_DEBOUNCE: Duration =
                                Duration::from_secs(1);

                            let task = cx.spawn_weak(|this, mut cx| async move {
                                cx.background().timer(DISK_BASED_DIAGNOSTICS_DEBOUNCE).await;
                                if let Some(this) = this.upgrade(&cx) {
                                    this.update(&mut cx, |this, cx| {
                                        this.disk_based_diagnostics_finished(
                                            language_server_id,
                                            cx,
                                        );
                                        this.buffer_ordered_messages_tx
                                            .unbounded_send(
                                                BufferOrderedMessage::LanguageServerUpdate {
                                                    language_server_id,
                                                    message:proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(Default::default())
                                                },
                                            )
                                            .ok();
                                    });
                                }
                            });
                            *simulate_disk_based_diagnostics_completion = Some(task);
                        }
                    }
                }
            }

            _ => {}
        }

        None
    }

    fn language_servers_for_worktree(
        &self,
        worktree_id: WorktreeId,
    ) -> impl Iterator<Item = (&Arc<CachedLspAdapter>, &Arc<Language>, &Arc<LanguageServer>)> {
        self.language_server_ids
            .iter()
            .filter_map(move |((language_server_worktree_id, _), id)| {
                if *language_server_worktree_id == worktree_id {
                    if let Some(LanguageServerState::Running {
                        adapter,
                        language,
                        server,
                        ..
                    }) = self.language_servers.get(id)
                    {
                        return Some((adapter, language, server));
                    }
                }
                None
            })
    }

    fn maintain_buffer_languages(
        languages: &LanguageRegistry,
        cx: &mut ModelContext<Project>,
    ) -> Task<()> {
        let mut subscription = languages.subscribe();
        cx.spawn_weak(|project, mut cx| async move {
            while let Some(()) = subscription.next().await {
                if let Some(project) = project.upgrade(&cx) {
                    project.update(&mut cx, |project, cx| {
                        let mut plain_text_buffers = Vec::new();
                        let mut buffers_with_unknown_injections = Vec::new();
                        for buffer in project.opened_buffers.values() {
                            if let Some(handle) = buffer.upgrade(cx) {
                                let buffer = &handle.read(cx);
                                if buffer.language().is_none()
                                    || buffer.language() == Some(&*language::PLAIN_TEXT)
                                {
                                    plain_text_buffers.push(handle);
                                } else if buffer.contains_unknown_injections() {
                                    buffers_with_unknown_injections.push(handle);
                                }
                            }
                        }

                        for buffer in plain_text_buffers {
                            project.detect_language_for_buffer(&buffer, cx);
                            project.register_buffer_with_language_servers(&buffer, cx);
                        }

                        for buffer in buffers_with_unknown_injections {
                            buffer.update(cx, |buffer, cx| buffer.reparse(cx));
                        }
                    });
                }
            }
        })
    }

    fn maintain_workspace_config(
        languages: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Project>,
    ) -> Task<()> {
        let (mut settings_changed_tx, mut settings_changed_rx) = watch::channel();
        let _ = postage::stream::Stream::try_recv(&mut settings_changed_rx);

        let settings_observation = cx.observe_global::<Settings, _>(move |_, _| {
            *settings_changed_tx.borrow_mut() = ();
        });
        cx.spawn_weak(|this, mut cx| async move {
            while let Some(_) = settings_changed_rx.next().await {
                let workspace_config = cx.update(|cx| languages.workspace_configuration(cx)).await;
                if let Some(this) = this.upgrade(&cx) {
                    this.read_with(&cx, |this, _| {
                        for server_state in this.language_servers.values() {
                            if let LanguageServerState::Running { server, .. } = server_state {
                                server
                                    .notify::<lsp::notification::DidChangeConfiguration>(
                                        lsp::DidChangeConfigurationParams {
                                            settings: workspace_config.clone(),
                                        },
                                    )
                                    .ok();
                            }
                        }
                    })
                } else {
                    break;
                }
            }

            drop(settings_observation);
        })
    }

    fn detect_language_for_buffer(
        &mut self,
        buffer_handle: &ModelHandle<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        // If the buffer has a language, set it and start the language server if we haven't already.
        let buffer = buffer_handle.read(cx);
        let full_path = buffer.file()?.full_path(cx);
        let content = buffer.as_rope();
        let new_language = self
            .languages
            .language_for_file(&full_path, Some(content))
            .now_or_never()?
            .ok()?;
        self.set_language_for_buffer(buffer_handle, new_language, cx);
        None
    }

    pub fn set_language_for_buffer(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        new_language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        buffer.update(cx, |buffer, cx| {
            if buffer.language().map_or(true, |old_language| {
                !Arc::ptr_eq(old_language, &new_language)
            }) {
                buffer.set_language(Some(new_language.clone()), cx);
            }
        });

        if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
            if let Some(worktree) = file.worktree.read(cx).as_local() {
                let worktree_id = worktree.id();
                let worktree_abs_path = worktree.abs_path().clone();
                self.start_language_servers(worktree_id, worktree_abs_path, new_language, cx);
            }
        }
    }

    fn start_language_servers(
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

        for adapter in language.lsp_adapters() {
            let key = (worktree_id, adapter.name.clone());
            if self.language_server_ids.contains_key(&key) {
                continue;
            }

            let pending_server = match self.languages.start_language_server(
                language.clone(),
                adapter.clone(),
                worktree_path.clone(),
                self.client.http_client(),
                cx,
            ) {
                Some(pending_server) => pending_server,
                None => continue,
            };

            let lsp = &cx.global::<Settings>().lsp.get(&adapter.name.0);
            let override_options = lsp.map(|s| s.initialization_options.clone()).flatten();

            let mut initialization_options = adapter.initialization_options.clone();
            match (&mut initialization_options, override_options) {
                (Some(initialization_options), Some(override_options)) => {
                    merge_json_value_into(override_options, initialization_options);
                }
                (None, override_options) => initialization_options = override_options,
                _ => {}
            }

            let server_id = pending_server.server_id;
            let state = self.setup_pending_language_server(
                initialization_options,
                pending_server,
                adapter.clone(),
                language.clone(),
                key.clone(),
                cx,
            );
            self.language_servers.insert(server_id, state);
            self.language_server_ids.insert(key.clone(), server_id);
        }
    }

    fn setup_pending_language_server(
        &mut self,
        initialization_options: Option<serde_json::Value>,
        pending_server: PendingLanguageServer,
        adapter: Arc<CachedLspAdapter>,
        language: Arc<Language>,
        key: (WorktreeId, LanguageServerName),
        cx: &mut ModelContext<Project>,
    ) -> LanguageServerState {
        let server_id = pending_server.server_id;
        let languages = self.languages.clone();

        LanguageServerState::Starting(cx.spawn_weak(|this, mut cx| async move {
            let workspace_config = cx.update(|cx| languages.workspace_configuration(cx)).await;
            let language_server = pending_server.task.await.log_err()?;
            let language_server = language_server
                .initialize(initialization_options)
                .await
                .log_err()?;
            let this = this.upgrade(&cx)?;

            language_server
                .on_notification::<lsp::notification::PublishDiagnostics, _>({
                    let this = this.downgrade();
                    let adapter = adapter.clone();
                    move |mut params, cx| {
                        let this = this;
                        let adapter = adapter.clone();
                        cx.spawn(|mut cx| async move {
                            adapter.process_diagnostics(&mut params).await;
                            if let Some(this) = this.upgrade(&cx) {
                                this.update(&mut cx, |this, cx| {
                                    this.update_diagnostics(
                                        server_id,
                                        params,
                                        &adapter.disk_based_diagnostic_sources,
                                        cx,
                                    )
                                    .log_err();
                                });
                            }
                        })
                        .detach();
                    }
                })
                .detach();

            language_server
                .on_request::<lsp::request::WorkspaceConfiguration, _, _>({
                    let languages = languages.clone();
                    move |params, mut cx| {
                        let languages = languages.clone();
                        async move {
                            let workspace_config =
                                cx.update(|cx| languages.workspace_configuration(cx)).await;
                            Ok(params
                                .items
                                .into_iter()
                                .map(|item| {
                                    if let Some(section) = &item.section {
                                        workspace_config
                                            .get(section)
                                            .cloned()
                                            .unwrap_or(serde_json::Value::Null)
                                    } else {
                                        workspace_config.clone()
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
                                    if let lsp::NumberOrString::String(token) = params.token {
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
                .on_request::<lsp::request::RegisterCapability, _, _>({
                    let this = this.downgrade();
                    move |params, mut cx| async move {
                        let this = this
                            .upgrade(&cx)
                            .ok_or_else(|| anyhow!("project dropped"))?;
                        for reg in params.registrations {
                            if reg.method == "workspace/didChangeWatchedFiles" {
                                if let Some(options) = reg.register_options {
                                    let options = serde_json::from_value(options)?;
                                    this.update(&mut cx, |this, cx| {
                                        this.on_lsp_did_change_watched_files(
                                            server_id, options, cx,
                                        );
                                    });
                                }
                            }
                        }
                        Ok(())
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

            let disk_based_diagnostics_progress_token =
                adapter.disk_based_diagnostics_progress_token.clone();

            language_server
                .on_notification::<lsp::notification::Progress, _>({
                    let this = this.downgrade();
                    move |params, mut cx| {
                        if let Some(this) = this.upgrade(&cx) {
                            this.update(&mut cx, |this, cx| {
                                this.on_lsp_progress(
                                    params,
                                    server_id,
                                    disk_based_diagnostics_progress_token.clone(),
                                    cx,
                                );
                            });
                        }
                    }
                })
                .detach();

            language_server
                .notify::<lsp::notification::DidChangeConfiguration>(
                    lsp::DidChangeConfigurationParams {
                        settings: workspace_config,
                    },
                )
                .ok();

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
                        language: language.clone(),
                        watched_paths: Default::default(),
                        server: language_server.clone(),
                        simulate_disk_based_diagnostics_completion: None,
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

                if let Some(project_id) = this.remote_id() {
                    this.client
                        .send(proto::StartLanguageServer {
                            project_id,
                            server: Some(proto::LanguageServer {
                                id: server_id.0 as u64,
                                name: language_server.name().to_string(),
                            }),
                        })
                        .log_err();
                }

                // Tell the language server about every open buffer in the worktree that matches the language.
                for buffer in this.opened_buffers.values() {
                    if let Some(buffer_handle) = buffer.upgrade(cx) {
                        let buffer = buffer_handle.read(cx);
                        let file = match File::from_dyn(buffer.file()) {
                            Some(file) => file,
                            None => continue,
                        };
                        let language = match buffer.language() {
                            Some(language) => language,
                            None => continue,
                        };

                        if file.worktree.read(cx).id() != key.0
                            || !language.lsp_adapters().iter().any(|a| a.name == key.1)
                        {
                            continue;
                        }

                        let file = file.as_local()?;
                        let versions = this
                            .buffer_snapshots
                            .entry(buffer.remote_id())
                            .or_default()
                            .entry(server_id)
                            .or_insert_with(|| {
                                vec![LspBufferSnapshot {
                                    version: 0,
                                    snapshot: buffer.text_snapshot(),
                                }]
                            });

                        let snapshot = versions.last().unwrap();
                        let version = snapshot.version;
                        let initial_snapshot = &snapshot.snapshot;
                        let uri = lsp::Url::from_file_path(file.abs_path(cx)).unwrap();
                        language_server
                            .notify::<lsp::notification::DidOpenTextDocument>(
                                lsp::DidOpenTextDocumentParams {
                                    text_document: lsp::TextDocumentItem::new(
                                        uri,
                                        adapter
                                            .language_ids
                                            .get(language.name().as_ref())
                                            .cloned()
                                            .unwrap_or_default(),
                                        version,
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
                                    .and_then(|provider| provider.trigger_characters.clone())
                                    .unwrap_or_default(),
                                cx,
                            )
                        });
                    }
                }

                cx.notify();
                Some(language_server)
            })
        }))
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
        let language_server_lookup_info: HashSet<(WorktreeId, Arc<Path>, Arc<Language>)> = buffers
            .into_iter()
            .filter_map(|buffer| {
                let buffer = buffer.read(cx);
                let file = File::from_dyn(buffer.file())?;
                let worktree = file.worktree.read(cx).as_local()?;
                let full_path = file.full_path(cx);
                let language = self
                    .languages
                    .language_for_file(&full_path, Some(buffer.as_rope()))
                    .now_or_never()?
                    .ok()?;
                Some((worktree.id(), worktree.abs_path().clone(), language))
            })
            .collect();
        for (worktree_id, worktree_abs_path, language) in language_server_lookup_info {
            self.restart_language_servers(worktree_id, worktree_abs_path, language, cx);
        }

        None
    }

    // TODO This will break in the case where the adapter's root paths and worktrees are not equal
    fn restart_language_servers(
        &mut self,
        worktree_id: WorktreeId,
        fallback_path: Arc<Path>,
        language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        let mut stops = Vec::new();
        for adapter in language.lsp_adapters() {
            stops.push(self.stop_language_server(worktree_id, adapter.name.clone(), cx));
        }

        if stops.is_empty() {
            return;
        }
        let mut stops = stops.into_iter();

        cx.spawn_weak(|this, mut cx| async move {
            let (original_root_path, mut orphaned_worktrees) = stops.next().unwrap().await;
            for stop in stops {
                let (_, worktrees) = stop.await;
                orphaned_worktrees.extend_from_slice(&worktrees);
            }

            let this = match this.upgrade(&cx) {
                Some(this) => this,
                None => return,
            };

            this.update(&mut cx, |this, cx| {
                // Attempt to restart using original server path. Fallback to passed in
                // path if we could not retrieve the root path
                let root_path = original_root_path
                    .map(|path_buf| Arc::from(path_buf.as_path()))
                    .unwrap_or(fallback_path);

                this.start_language_servers(worktree_id, root_path, language.clone(), cx);

                // Lookup new server ids and set them for each of the orphaned worktrees
                for adapter in language.lsp_adapters() {
                    if let Some(new_server_id) = this
                        .language_server_ids
                        .get(&(worktree_id, adapter.name.clone()))
                        .cloned()
                    {
                        for &orphaned_worktree in &orphaned_worktrees {
                            this.language_server_ids
                                .insert((orphaned_worktree, adapter.name.clone()), new_server_id);
                        }
                    }
                }
            });
        })
        .detach();
    }

    fn on_lsp_progress(
        &mut self,
        progress: lsp::ProgressParams,
        language_server_id: LanguageServerId,
        disk_based_diagnostics_progress_token: Option<String>,
        cx: &mut ModelContext<Self>,
    ) {
        let token = match progress.token {
            lsp::NumberOrString::String(token) => token,
            lsp::NumberOrString::Number(token) => {
                log::info!("skipping numeric progress token {}", token);
                return;
            }
        };
        let lsp::ProgressParamsValue::WorkDone(progress) = progress.value;
        let language_server_status =
            if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
                status
            } else {
                return;
            };

        if !language_server_status.progress_tokens.contains(&token) {
            return;
        }

        let is_disk_based_diagnostics_progress = disk_based_diagnostics_progress_token
            .as_ref()
            .map_or(false, |disk_based_token| {
                token.starts_with(disk_based_token)
            });

        match progress {
            lsp::WorkDoneProgress::Begin(report) => {
                if is_disk_based_diagnostics_progress {
                    language_server_status.has_pending_diagnostic_updates = true;
                    self.disk_based_diagnostics_started(language_server_id, cx);
                    self.buffer_ordered_messages_tx
                        .unbounded_send(BufferOrderedMessage::LanguageServerUpdate {
                            language_server_id,
                            message: proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(Default::default())
                        })
                        .ok();
                } else {
                    self.on_lsp_work_start(
                        language_server_id,
                        token.clone(),
                        LanguageServerProgress {
                            message: report.message.clone(),
                            percentage: report.percentage.map(|p| p as usize),
                            last_update_at: Instant::now(),
                        },
                        cx,
                    );
                    self.buffer_ordered_messages_tx
                        .unbounded_send(BufferOrderedMessage::LanguageServerUpdate {
                            language_server_id,
                            message: proto::update_language_server::Variant::WorkStart(
                                proto::LspWorkStart {
                                    token,
                                    message: report.message,
                                    percentage: report.percentage.map(|p| p as u32),
                                },
                            ),
                        })
                        .ok();
                }
            }
            lsp::WorkDoneProgress::Report(report) => {
                if !is_disk_based_diagnostics_progress {
                    self.on_lsp_work_progress(
                        language_server_id,
                        token.clone(),
                        LanguageServerProgress {
                            message: report.message.clone(),
                            percentage: report.percentage.map(|p| p as usize),
                            last_update_at: Instant::now(),
                        },
                        cx,
                    );
                    self.buffer_ordered_messages_tx
                        .unbounded_send(BufferOrderedMessage::LanguageServerUpdate {
                            language_server_id,
                            message: proto::update_language_server::Variant::WorkProgress(
                                proto::LspWorkProgress {
                                    token,
                                    message: report.message,
                                    percentage: report.percentage.map(|p| p as u32),
                                },
                            ),
                        })
                        .ok();
                }
            }
            lsp::WorkDoneProgress::End(_) => {
                language_server_status.progress_tokens.remove(&token);

                if is_disk_based_diagnostics_progress {
                    language_server_status.has_pending_diagnostic_updates = false;
                    self.disk_based_diagnostics_finished(language_server_id, cx);
                    self.buffer_ordered_messages_tx
                        .unbounded_send(BufferOrderedMessage::LanguageServerUpdate {
                            language_server_id,
                            message:
                                proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                                    Default::default(),
                                ),
                        })
                        .ok();
                } else {
                    self.on_lsp_work_end(language_server_id, token.clone(), cx);
                    self.buffer_ordered_messages_tx
                        .unbounded_send(BufferOrderedMessage::LanguageServerUpdate {
                            language_server_id,
                            message: proto::update_language_server::Variant::WorkEnd(
                                proto::LspWorkEnd { token },
                            ),
                        })
                        .ok();
                }
            }
        }
    }

    fn on_lsp_work_start(
        &mut self,
        language_server_id: LanguageServerId,
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
        language_server_id: LanguageServerId,
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
        language_server_id: LanguageServerId,
        token: String,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            status.pending_work.remove(&token);
            cx.notify();
        }
    }

    fn on_lsp_did_change_watched_files(
        &mut self,
        language_server_id: LanguageServerId,
        params: DidChangeWatchedFilesRegistrationOptions,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(LanguageServerState::Running { watched_paths, .. }) =
            self.language_servers.get_mut(&language_server_id)
        {
            watched_paths.clear();
            for watcher in params.watchers {
                watched_paths.add_pattern(&watcher.glob_pattern).log_err();
            }
            cx.notify();
        }
    }

    async fn on_lsp_workspace_edit(
        this: WeakModelHandle<Self>,
        params: lsp::ApplyWorkspaceEditParams,
        server_id: LanguageServerId,
        adapter: Arc<CachedLspAdapter>,
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

    pub fn language_server_statuses(
        &self,
    ) -> impl DoubleEndedIterator<Item = &LanguageServerStatus> {
        self.language_server_statuses.values()
    }

    pub fn update_diagnostics(
        &mut self,
        language_server_id: LanguageServerId,
        mut params: lsp::PublishDiagnosticsParams,
        disk_based_sources: &[String],
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

        // Ensure that primary diagnostics are always the most severe
        params.diagnostics.sort_by_key(|item| item.severity);

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
        server_id: LanguageServerId,
        abs_path: PathBuf,
        version: Option<i32>,
        diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
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
            self.update_buffer_diagnostics(&buffer, server_id, version, diagnostics.clone(), cx)?;
        }

        let updated = worktree.update(cx, |worktree, cx| {
            worktree
                .as_local_mut()
                .ok_or_else(|| anyhow!("not a local worktree"))?
                .update_diagnostics(server_id, project_path.path.clone(), diagnostics, cx)
        })?;
        if updated {
            cx.emit(Event::DiagnosticsUpdated {
                language_server_id: server_id,
                path: project_path,
            });
        }
        Ok(())
    }

    fn update_buffer_diagnostics(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        server_id: LanguageServerId,
        version: Option<i32>,
        mut diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        fn compare_diagnostics(a: &Diagnostic, b: &Diagnostic) -> Ordering {
            Ordering::Equal
                .then_with(|| b.is_primary.cmp(&a.is_primary))
                .then_with(|| a.is_disk_based.cmp(&b.is_disk_based))
                .then_with(|| a.severity.cmp(&b.severity))
                .then_with(|| a.message.cmp(&b.message))
        }

        let snapshot = self.buffer_snapshot_for_lsp_version(buffer, server_id, version, cx)?;

        diagnostics.sort_unstable_by(|a, b| {
            Ordering::Equal
                .then_with(|| a.range.start.cmp(&b.range.start))
                .then_with(|| b.range.end.cmp(&a.range.end))
                .then_with(|| compare_diagnostics(&a.diagnostic, &b.diagnostic))
        });

        let mut sanitized_diagnostics = Vec::new();
        let edits_since_save = Patch::new(
            snapshot
                .edits_since::<Unclipped<PointUtf16>>(buffer.read(cx).saved_version())
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

            // Expand empty ranges by one codepoint
            if range.start == range.end {
                // This will be go to the next boundary when being clipped
                range.end.column += 1;
                range.end = snapshot.clip_point_utf16(Unclipped(range.end), Bias::Right);
                if range.start == range.end && range.end.column > 0 {
                    range.start.column -= 1;
                    range.end = snapshot.clip_point_utf16(Unclipped(range.end), Bias::Left);
                }
            }

            sanitized_diagnostics.push(DiagnosticEntry {
                range,
                diagnostic: entry.diagnostic,
            });
        }
        drop(edits_since_save);

        let set = DiagnosticSet::new(sanitized_diagnostics, &snapshot);
        buffer.update(cx, |buffer, cx| {
            buffer.update_diagnostics(server_id, set, cx)
        });
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
        trigger: FormatTrigger,
        cx: &mut ModelContext<Project>,
    ) -> Task<Result<ProjectTransaction>> {
        if self.is_local() {
            let mut buffers_with_paths_and_servers = buffers
                .into_iter()
                .filter_map(|buffer_handle| {
                    let buffer = buffer_handle.read(cx);
                    let file = File::from_dyn(buffer.file())?;
                    let buffer_abs_path = file.as_local().map(|f| f.abs_path(cx));
                    let server = self
                        .primary_language_servers_for_buffer(buffer, cx)
                        .map(|s| s.1.clone());
                    Some((buffer_handle, buffer_abs_path, server))
                })
                .collect::<Vec<_>>();

            cx.spawn(|this, mut cx| async move {
                // Do not allow multiple concurrent formatting requests for the
                // same buffer.
                this.update(&mut cx, |this, _| {
                    buffers_with_paths_and_servers
                        .retain(|(buffer, _, _)| this.buffers_being_formatted.insert(buffer.id()));
                });

                let _cleanup = defer({
                    let this = this.clone();
                    let mut cx = cx.clone();
                    let buffers = &buffers_with_paths_and_servers;
                    move || {
                        this.update(&mut cx, |this, _| {
                            for (buffer, _, _) in buffers {
                                this.buffers_being_formatted.remove(&buffer.id());
                            }
                        });
                    }
                });

                let mut project_transaction = ProjectTransaction::default();
                for (buffer, buffer_abs_path, language_server) in &buffers_with_paths_and_servers {
                    let (
                        format_on_save,
                        remove_trailing_whitespace,
                        ensure_final_newline,
                        formatter,
                        tab_size,
                    ) = buffer.read_with(&cx, |buffer, cx| {
                        let settings = cx.global::<Settings>();
                        let language_name = buffer.language().map(|language| language.name());
                        (
                            settings.format_on_save(language_name.as_deref()),
                            settings.remove_trailing_whitespace_on_save(language_name.as_deref()),
                            settings.ensure_final_newline_on_save(language_name.as_deref()),
                            settings.formatter(language_name.as_deref()),
                            settings.tab_size(language_name.as_deref()),
                        )
                    });

                    // First, format buffer's whitespace according to the settings.
                    let trailing_whitespace_diff = if remove_trailing_whitespace {
                        Some(
                            buffer
                                .read_with(&cx, |b, cx| b.remove_trailing_whitespace(cx))
                                .await,
                        )
                    } else {
                        None
                    };
                    let whitespace_transaction_id = buffer.update(&mut cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();
                        if let Some(diff) = trailing_whitespace_diff {
                            buffer.apply_diff(diff, cx);
                        }
                        if ensure_final_newline {
                            buffer.ensure_final_newline(cx);
                        }
                        buffer.end_transaction(cx)
                    });

                    // Currently, formatting operations are represented differently depending on
                    // whether they come from a language server or an external command.
                    enum FormatOperation {
                        Lsp(Vec<(Range<Anchor>, String)>),
                        External(Diff),
                    }

                    // Apply language-specific formatting using either a language server
                    // or external command.
                    let mut format_operation = None;
                    match (formatter, format_on_save) {
                        (_, FormatOnSave::Off) if trigger == FormatTrigger::Save => {}

                        (Formatter::LanguageServer, FormatOnSave::On | FormatOnSave::Off)
                        | (_, FormatOnSave::LanguageServer) => {
                            if let Some((language_server, buffer_abs_path)) =
                                language_server.as_ref().zip(buffer_abs_path.as_ref())
                            {
                                format_operation = Some(FormatOperation::Lsp(
                                    Self::format_via_lsp(
                                        &this,
                                        &buffer,
                                        buffer_abs_path,
                                        &language_server,
                                        tab_size,
                                        &mut cx,
                                    )
                                    .await
                                    .context("failed to format via language server")?,
                                ));
                            }
                        }

                        (
                            Formatter::External { command, arguments },
                            FormatOnSave::On | FormatOnSave::Off,
                        )
                        | (_, FormatOnSave::External { command, arguments }) => {
                            if let Some(buffer_abs_path) = buffer_abs_path {
                                format_operation = Self::format_via_external_command(
                                    &buffer,
                                    &buffer_abs_path,
                                    &command,
                                    &arguments,
                                    &mut cx,
                                )
                                .await
                                .context(format!(
                                    "failed to format via external command {:?}",
                                    command
                                ))?
                                .map(FormatOperation::External);
                            }
                        }
                    };

                    buffer.update(&mut cx, |b, cx| {
                        // If the buffer had its whitespace formatted and was edited while the language-specific
                        // formatting was being computed, avoid applying the language-specific formatting, because
                        // it can't be grouped with the whitespace formatting in the undo history.
                        if let Some(transaction_id) = whitespace_transaction_id {
                            if b.peek_undo_stack()
                                .map_or(true, |e| e.transaction_id() != transaction_id)
                            {
                                format_operation.take();
                            }
                        }

                        // Apply any language-specific formatting, and group the two formatting operations
                        // in the buffer's undo history.
                        if let Some(operation) = format_operation {
                            match operation {
                                FormatOperation::Lsp(edits) => {
                                    b.edit(edits, None, cx);
                                }
                                FormatOperation::External(diff) => {
                                    b.apply_diff(diff, cx);
                                }
                            }

                            if let Some(transaction_id) = whitespace_transaction_id {
                                b.group_until_transaction(transaction_id);
                            }
                        }

                        if let Some(transaction) = b.finalize_last_transaction().cloned() {
                            if !push_to_history {
                                b.forget_transaction(transaction.id);
                            }
                            project_transaction.0.insert(buffer.clone(), transaction);
                        }
                    });
                }

                Ok(project_transaction)
            })
        } else {
            let remote_id = self.remote_id();
            let client = self.client.clone();
            cx.spawn(|this, mut cx| async move {
                let mut project_transaction = ProjectTransaction::default();
                if let Some(project_id) = remote_id {
                    let response = client
                        .request(proto::FormatBuffers {
                            project_id,
                            trigger: trigger as i32,
                            buffer_ids: buffers
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
                Ok(project_transaction)
            })
        }
    }

    async fn format_via_lsp(
        this: &ModelHandle<Self>,
        buffer: &ModelHandle<Buffer>,
        abs_path: &Path,
        language_server: &Arc<LanguageServer>,
        tab_size: NonZeroU32,
        cx: &mut AsyncAppContext,
    ) -> Result<Vec<(Range<Anchor>, String)>> {
        let text_document =
            lsp::TextDocumentIdentifier::new(lsp::Url::from_file_path(abs_path).unwrap());
        let capabilities = &language_server.capabilities();
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
                buffer.read_with(cx, |buffer, _| point_to_lsp(buffer.max_point_utf16()));
            language_server
                .request::<lsp::request::RangeFormatting>(lsp::DocumentRangeFormattingParams {
                    text_document,
                    range: lsp::Range::new(buffer_start, buffer_end),
                    options: lsp::FormattingOptions {
                        tab_size: tab_size.into(),
                        insert_spaces: true,
                        insert_final_newline: Some(true),
                        ..Default::default()
                    },
                    work_done_progress_params: Default::default(),
                })
                .await?
        } else {
            None
        };

        if let Some(lsp_edits) = lsp_edits {
            this.update(cx, |this, cx| {
                this.edits_from_lsp(buffer, lsp_edits, language_server.server_id(), None, cx)
            })
            .await
        } else {
            Ok(Default::default())
        }
    }

    async fn format_via_external_command(
        buffer: &ModelHandle<Buffer>,
        buffer_abs_path: &Path,
        command: &str,
        arguments: &[String],
        cx: &mut AsyncAppContext,
    ) -> Result<Option<Diff>> {
        let working_dir_path = buffer.read_with(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file())?;
            let worktree = file.worktree.read(cx).as_local()?;
            let mut worktree_path = worktree.abs_path().to_path_buf();
            if worktree.root_entry()?.is_file() {
                worktree_path.pop();
            }
            Some(worktree_path)
        });

        if let Some(working_dir_path) = working_dir_path {
            let mut child =
                smol::process::Command::new(command)
                    .args(arguments.iter().map(|arg| {
                        arg.replace("{buffer_path}", &buffer_abs_path.to_string_lossy())
                    }))
                    .current_dir(&working_dir_path)
                    .stdin(smol::process::Stdio::piped())
                    .stdout(smol::process::Stdio::piped())
                    .stderr(smol::process::Stdio::piped())
                    .spawn()?;
            let stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| anyhow!("failed to acquire stdin"))?;
            let text = buffer.read_with(cx, |buffer, _| buffer.as_rope().clone());
            for chunk in text.chunks() {
                stdin.write_all(chunk.as_bytes()).await?;
            }
            stdin.flush().await?;

            let output = child.output().await?;
            if !output.status.success() {
                return Err(anyhow!(
                    "command failed with exit code {:?}:\nstdout: {}\nstderr: {}",
                    output.status.code(),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr),
                ));
            }

            let stdout = String::from_utf8(output.stdout)?;
            Ok(Some(
                buffer
                    .read_with(cx, |buffer, cx| buffer.diff(stdout, cx))
                    .await,
            ))
        } else {
            Ok(None)
        }
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

    pub fn type_definition<T: ToPointUtf16>(
        &self,
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(buffer.clone(), GetTypeDefinition { position }, cx)
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
                    if let Some(LanguageServerState::Running {
                        adapter,
                        language,
                        server,
                        ..
                    }) = self.language_servers.get(server_id)
                    {
                        let adapter = adapter.clone();
                        let language = language.clone();
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
                                        language,
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
                let symbols = this.read_with(&cx, |this, cx| {
                    let mut symbols = Vec::new();
                    for (
                        adapter,
                        adapter_language,
                        source_worktree_id,
                        worktree_abs_path,
                        response,
                    ) in responses
                    {
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

                            let project_path = ProjectPath {
                                worktree_id,
                                path: path.into(),
                            };
                            let signature = this.symbol_signature(&project_path);
                            let adapter_language = adapter_language.clone();
                            let language = this
                                .languages
                                .language_for_file(&project_path.path, None)
                                .unwrap_or_else(move |_| adapter_language);
                            let language_server_name = adapter.name.clone();
                            Some(async move {
                                let language = language.await;
                                let label = language
                                    .label_for_symbol(&lsp_symbol.name, lsp_symbol.kind)
                                    .await;

                                Symbol {
                                    language_server_name,
                                    source_worktree_id,
                                    path: project_path,
                                    label: label.unwrap_or_else(|| {
                                        CodeLabel::plain(lsp_symbol.name.clone(), None)
                                    }),
                                    kind: lsp_symbol.kind,
                                    name: lsp_symbol.name,
                                    range: range_from_lsp(lsp_symbol.location.range),
                                    signature,
                                }
                            })
                        }));
                    }
                    symbols
                });
                Ok(futures::future::join_all(symbols).await)
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
                    let new_symbols = this.read_with(&cx, |this, _| {
                        response
                            .symbols
                            .into_iter()
                            .map(|symbol| this.deserialize_symbol(symbol))
                            .collect::<Vec<_>>()
                    });
                    symbols = futures::future::join_all(new_symbols)
                        .await
                        .into_iter()
                        .filter_map(|symbol| symbol.log_err())
                        .collect::<Vec<_>>();
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
                .worktree_for_id(symbol.path.worktree_id, cx)
                .and_then(|worktree| worktree.read(cx).as_local())
                .map(|local_worktree| local_worktree.abs_path())
            {
                worktree_abs_path
            } else {
                return Task::ready(Err(anyhow!("worktree not found for symbol")));
            };
            let symbol_abs_path = worktree_abs_path.join(&symbol.path.path);
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
                this.update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(response.buffer_id, cx)
                })
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
        buffer: &ModelHandle<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(buffer.clone(), GetCompletions { position }, cx)
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
            let lang_server = match self.primary_language_servers_for_buffer(buffer, cx) {
                Some((_, server)) => server.clone(),
                _ => return Task::ready(Ok(Default::default())),
            };

            cx.spawn(|this, mut cx| async move {
                let resolved_completion = lang_server
                    .request::<lsp::request::ResolveCompletionItem>(completion.lsp_completion)
                    .await?;

                if let Some(edits) = resolved_completion.additional_text_edits {
                    let edits = this
                        .update(&mut cx, |this, cx| {
                            this.edits_from_lsp(
                                &buffer_handle,
                                edits,
                                lang_server.server_id(),
                                None,
                                cx,
                            )
                        })
                        .await?;

                    buffer_handle.update(&mut cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();

                        for (range, text) in edits {
                            let primary = &completion.old_range;
                            let start_within = primary.start.cmp(&range.start, buffer).is_le()
                                && primary.end.cmp(&range.start, buffer).is_ge();
                            let end_within = range.start.cmp(&primary.end, buffer).is_le()
                                && range.end.cmp(&primary.end, buffer).is_ge();

                            //Skip addtional edits which overlap with the primary completion edit
                            //https://github.com/zed-industries/zed/pull/1871
                            if !start_within && !end_within {
                                buffer.edit([(range, text)], None, cx);
                            }
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
                        .await?;
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
        let buffer = buffer_handle.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);
        self.request_lsp(buffer_handle.clone(), GetCodeActions { range }, cx)
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
            let (lsp_adapter, lang_server) = if let Some((adapter, server)) =
                self.language_server_for_buffer(buffer, action.server_id, cx)
            {
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
                    if edit.changes.is_some() || edit.document_changes.is_some() {
                        return Self::deserialize_workspace_edit(
                            this,
                            edit,
                            push_to_history,
                            lsp_adapter.clone(),
                            lang_server.clone(),
                            &mut cx,
                        )
                        .await;
                    }
                }

                if let Some(command) = action.lsp_action.command {
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
                    return Ok(this.update(&mut cx, |this, _| {
                        this.last_workspace_edits_by_language_server
                            .remove(&lang_server.server_id())
                            .unwrap_or_default()
                    }));
                }

                Ok(ProjectTransaction::default())
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
        lsp_adapter: Arc<CachedLspAdapter>,
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
                                lsp_adapter.name.clone(),
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
                                language_server.server_id(),
                                op.text_document.version,
                                cx,
                            )
                        })
                        .await?;

                    let transaction = buffer_to_edit.update(cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();
                        for (range, text) in edits {
                            buffer.edit([(range, text)], None, cx);
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

    #[allow(clippy::type_complexity)]
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
                    let target_buffer = this
                        .update(&mut cx, |this, cx| {
                            this.wait_for_remote_buffer(location.buffer_id, cx)
                        })
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

    // TODO: Wire this up to allow selecting a server?
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
                self.primary_language_servers_for_buffer(buffer, cx)
                    .map(|(_, server)| server.clone()),
            ) {
                let lsp_params = request.to_lsp(&file.abs_path(cx), buffer, &language_server, cx);
                return cx.spawn(|this, cx| async move {
                    if !request.check_capabilities(language_server.capabilities()) {
                        return Ok(Default::default());
                    }

                    let response = language_server
                        .request::<R::LspRequest>(lsp_params)
                        .await
                        .context("lsp request failed")?;
                    request
                        .response_from_lsp(
                            response,
                            this,
                            buffer_handle,
                            language_server.server_id(),
                            cx,
                        )
                        .await
                });
            }
        } else if let Some(project_id) = self.remote_id() {
            let rpc = self.client.clone();
            let message = request.to_proto(project_id, buffer);
            return cx.spawn_weak(|this, cx| async move {
                // Ensure the project is still alive by the time the task
                // is scheduled.
                this.upgrade(&cx)
                    .ok_or_else(|| anyhow!("project dropped"))?;

                let response = rpc.request(message).await?;

                let this = this
                    .upgrade(&cx)
                    .ok_or_else(|| anyhow!("project dropped"))?;
                if this.read_with(&cx, |this, _| this.is_read_only()) {
                    Err(anyhow!("disconnected before completing request"))
                } else {
                    request
                        .response_from_proto(response, this, buffer_handle, cx)
                        .await
                }
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
            Task::ready(Ok((tree, relative_path)))
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
            Some(ProjectClientState::Local { .. }) => true,
            _ => false,
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
                        project.update(&mut cx, |project, cx| project.add_worktree(&worktree, cx));
                        Ok(worktree)
                    }
                    .map_err(Arc::new)
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
    }

    fn add_worktree(&mut self, worktree: &ModelHandle<Worktree>, cx: &mut ModelContext<Self>) {
        cx.observe(worktree, |_, _, cx| cx.notify()).detach();
        if worktree.read(cx).is_local() {
            cx.subscribe(worktree, |this, worktree, event, cx| match event {
                worktree::Event::UpdatedEntries(changes) => {
                    this.update_local_worktree_buffers(&worktree, cx);
                    this.update_local_worktree_language_servers(&worktree, changes, cx);
                }
                worktree::Event::UpdatedGitRepositories(updated_repos) => {
                    this.update_local_worktree_buffers_git_repos(worktree, updated_repos, cx)
                }
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

        cx.observe_release(worktree, |this, worktree, cx| {
            let _ = this.remove_worktree(worktree.id(), cx);
        })
        .detach();

        cx.emit(Event::WorktreeAdded);
        self.metadata_changed(cx);
    }

    fn update_local_worktree_buffers(
        &mut self,
        worktree_handle: &ModelHandle<Worktree>,
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();

        let mut buffers_to_delete = Vec::new();
        let mut renamed_buffers = Vec::new();

        for (buffer_id, buffer) in &self.opened_buffers {
            if let Some(buffer) = buffer.upgrade(cx) {
                buffer.update(cx, |buffer, cx| {
                    if let Some(old_file) = File::from_dyn(buffer.file()) {
                        if old_file.worktree != *worktree_handle {
                            return;
                        }

                        let new_file = if let Some(entry) = snapshot.entry_for_id(old_file.entry_id)
                        {
                            File {
                                is_local: true,
                                entry_id: entry.id,
                                mtime: entry.mtime,
                                path: entry.path.clone(),
                                worktree: worktree_handle.clone(),
                                is_deleted: false,
                            }
                        } else if let Some(entry) =
                            snapshot.entry_for_path(old_file.path().as_ref())
                        {
                            File {
                                is_local: true,
                                entry_id: entry.id,
                                mtime: entry.mtime,
                                path: entry.path.clone(),
                                worktree: worktree_handle.clone(),
                                is_deleted: false,
                            }
                        } else {
                            File {
                                is_local: true,
                                entry_id: old_file.entry_id,
                                path: old_file.path().clone(),
                                mtime: old_file.mtime(),
                                worktree: worktree_handle.clone(),
                                is_deleted: true,
                            }
                        };

                        let old_path = old_file.abs_path(cx);
                        if new_file.abs_path(cx) != old_path {
                            renamed_buffers.push((cx.handle(), old_file.clone()));
                        }

                        if new_file != *old_file {
                            if let Some(project_id) = self.remote_id() {
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
                    }
                });
            } else {
                buffers_to_delete.push(*buffer_id);
            }
        }

        for buffer_id in buffers_to_delete {
            self.opened_buffers.remove(&buffer_id);
        }

        for (buffer, old_file) in renamed_buffers {
            self.unregister_buffer_from_language_servers(&buffer, &old_file, cx);
            self.detect_language_for_buffer(&buffer, cx);
            self.register_buffer_with_language_servers(&buffer, cx);
        }
    }

    fn update_local_worktree_language_servers(
        &mut self,
        worktree_handle: &ModelHandle<Worktree>,
        changes: &HashMap<Arc<Path>, PathChange>,
        cx: &mut ModelContext<Self>,
    ) {
        let worktree_id = worktree_handle.read(cx).id();
        let abs_path = worktree_handle.read(cx).abs_path();
        for ((server_worktree_id, _), server_id) in &self.language_server_ids {
            if *server_worktree_id == worktree_id {
                if let Some(server) = self.language_servers.get(server_id) {
                    if let LanguageServerState::Running {
                        server,
                        watched_paths,
                        ..
                    } = server
                    {
                        let params = lsp::DidChangeWatchedFilesParams {
                            changes: changes
                                .iter()
                                .filter_map(|(path, change)| {
                                    let path = abs_path.join(path);
                                    if watched_paths.matches(&path) {
                                        Some(lsp::FileEvent {
                                            uri: lsp::Url::from_file_path(path).unwrap(),
                                            typ: match change {
                                                PathChange::Added => lsp::FileChangeType::CREATED,
                                                PathChange::Removed => lsp::FileChangeType::DELETED,
                                                PathChange::Updated
                                                | PathChange::AddedOrUpdated => {
                                                    lsp::FileChangeType::CHANGED
                                                }
                                            },
                                        })
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        };

                        if !params.changes.is_empty() {
                            server
                                .notify::<lsp::notification::DidChangeWatchedFiles>(params)
                                .log_err();
                        }
                    }
                }
            }
        }
    }

    fn update_local_worktree_buffers_git_repos(
        &mut self,
        worktree: ModelHandle<Worktree>,
        repos: &[GitRepositoryEntry],
        cx: &mut ModelContext<Self>,
    ) {
        for (_, buffer) in &self.opened_buffers {
            if let Some(buffer) = buffer.upgrade(cx) {
                let file = match File::from_dyn(buffer.read(cx).file()) {
                    Some(file) => file,
                    None => continue,
                };
                if file.worktree != worktree {
                    continue;
                }

                let path = file.path().clone();

                let repo = match repos.iter().find(|repo| repo.manages(&path)) {
                    Some(repo) => repo.clone(),
                    None => return,
                };

                let relative_repo = match path.strip_prefix(repo.content_path) {
                    Ok(relative_repo) => relative_repo.to_owned(),
                    Err(_) => return,
                };

                let remote_id = self.remote_id();
                let client = self.client.clone();

                cx.spawn(|_, mut cx| async move {
                    let diff_base = cx
                        .background()
                        .spawn(async move { repo.repo.lock().load_index_text(&relative_repo) })
                        .await;

                    let buffer_id = buffer.update(&mut cx, |buffer, cx| {
                        buffer.set_diff_base(diff_base.clone(), cx);
                        buffer.remote_id()
                    });

                    if let Some(project_id) = remote_id {
                        client
                            .send(proto::UpdateDiffBase {
                                project_id,
                                buffer_id: buffer_id as u64,
                                diff_base,
                            })
                            .log_err();
                    }
                })
                .detach();
            }
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

    pub fn language_servers_running_disk_based_diagnostics(
        &self,
    ) -> impl Iterator<Item = LanguageServerId> + '_ {
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
        for (_, _, path_summary) in self.diagnostic_summaries(cx) {
            summary.error_count += path_summary.error_count;
            summary.warning_count += path_summary.warning_count;
        }
        summary
    }

    pub fn diagnostic_summaries<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = (ProjectPath, LanguageServerId, DiagnosticSummary)> + 'a {
        self.visible_worktrees(cx).flat_map(move |worktree| {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            worktree
                .diagnostic_summaries()
                .map(move |(path, server_id, summary)| {
                    (ProjectPath { worktree_id, path }, server_id, summary)
                })
        })
    }

    pub fn disk_based_diagnostics_started(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) {
        cx.emit(Event::DiskBasedDiagnosticsStarted { language_server_id });
    }

    pub fn disk_based_diagnostics_finished(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) {
        cx.emit(Event::DiskBasedDiagnosticsFinished { language_server_id });
    }

    pub fn active_entry(&self) -> Option<ProjectEntryId> {
        self.active_entry
    }

    pub fn entry_for_path(&self, path: &ProjectPath, cx: &AppContext) -> Option<Entry> {
        self.worktree_for_id(path.worktree_id, cx)?
            .read(cx)
            .entry_for_path(&path.path)
            .cloned()
    }

    pub fn path_for_entry(&self, entry_id: ProjectEntryId, cx: &AppContext) -> Option<ProjectPath> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        let worktree = worktree.read(cx);
        let worktree_id = worktree.id();
        let path = worktree.entry_for_id(entry_id)?.path.clone();
        Some(ProjectPath { worktree_id, path })
    }

    // RPC message handlers

    async fn handle_unshare_project(
        this: ModelHandle<Self>,
        _: TypedEnvelope<proto::UnshareProject>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            if this.is_local() {
                this.unshare(cx)?;
            } else {
                this.disconnected_from_host(cx);
            }
            Ok(())
        })
    }

    async fn handle_add_collaborator(
        this: ModelHandle<Self>,
        mut envelope: TypedEnvelope<proto::AddProjectCollaborator>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let collaborator = envelope
            .payload
            .collaborator
            .take()
            .ok_or_else(|| anyhow!("empty collaborator"))?;

        let collaborator = Collaborator::from_proto(collaborator)?;
        this.update(&mut cx, |this, cx| {
            this.shared_buffers.remove(&collaborator.peer_id);
            this.collaborators
                .insert(collaborator.peer_id, collaborator);
            cx.notify();
        });

        Ok(())
    }

    async fn handle_update_project_collaborator(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateProjectCollaborator>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let old_peer_id = envelope
            .payload
            .old_peer_id
            .ok_or_else(|| anyhow!("missing old peer id"))?;
        let new_peer_id = envelope
            .payload
            .new_peer_id
            .ok_or_else(|| anyhow!("missing new peer id"))?;
        this.update(&mut cx, |this, cx| {
            let collaborator = this
                .collaborators
                .remove(&old_peer_id)
                .ok_or_else(|| anyhow!("received UpdateProjectCollaborator for unknown peer"))?;
            let is_host = collaborator.replica_id == 0;
            this.collaborators.insert(new_peer_id, collaborator);

            let buffers = this.shared_buffers.remove(&old_peer_id);
            log::info!(
                "peer {} became {}. moving buffers {:?}",
                old_peer_id,
                new_peer_id,
                &buffers
            );
            if let Some(buffers) = buffers {
                this.shared_buffers.insert(new_peer_id, buffers);
            }

            if is_host {
                this.opened_buffers
                    .retain(|_, buffer| !matches!(buffer, OpenBuffer::Operations(_)));
                this.buffer_ordered_messages_tx
                    .unbounded_send(BufferOrderedMessage::Resync)
                    .unwrap();
            }

            cx.emit(Event::CollaboratorUpdated {
                old_peer_id,
                new_peer_id,
            });
            cx.notify();
            Ok(())
        })
    }

    async fn handle_remove_collaborator(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::RemoveProjectCollaborator>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let peer_id = envelope
                .payload
                .peer_id
                .ok_or_else(|| anyhow!("invalid peer id"))?;
            let replica_id = this
                .collaborators
                .remove(&peer_id)
                .ok_or_else(|| anyhow!("unknown peer {:?}", peer_id))?
                .replica_id;
            for buffer in this.opened_buffers.values() {
                if let Some(buffer) = buffer.upgrade(cx) {
                    buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
                }
            }
            this.shared_buffers.remove(&peer_id);

            cx.emit(Event::CollaboratorLeft(peer_id));
            cx.notify();
            Ok(())
        })
    }

    async fn handle_update_project(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateProject>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            // Don't handle messages that were sent before the response to us joining the project
            if envelope.message_id > this.join_project_response_message_id {
                this.set_worktrees_from_proto(envelope.payload.worktrees, cx)?;
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
                let path = PathBuf::from(envelope.payload.path);
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
                let new_path = PathBuf::from(envelope.payload.new_path);
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
                let new_path = PathBuf::from(envelope.payload.new_path);
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
                        language_server_id: LanguageServerId(summary.language_server_id as usize),
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
                LanguageServerId(server.id as usize),
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
        this.update(&mut cx, |this, cx| {
            let language_server_id = LanguageServerId(envelope.payload.language_server_id as usize);

            match envelope
                .payload
                .variant
                .ok_or_else(|| anyhow!("invalid variant"))?
            {
                proto::update_language_server::Variant::WorkStart(payload) => {
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
                }

                proto::update_language_server::Variant::WorkProgress(payload) => {
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
                }

                proto::update_language_server::Variant::WorkEnd(payload) => {
                    this.on_lsp_work_end(language_server_id, payload.token, cx);
                }

                proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(_) => {
                    this.disk_based_diagnostics_started(language_server_id, cx);
                }

                proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(_) => {
                    this.disk_based_diagnostics_finished(language_server_id, cx)
                }
            }

            Ok(())
        })
    }

    async fn handle_update_buffer(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |this, cx| {
            let payload = envelope.payload.clone();
            let buffer_id = payload.buffer_id;
            let ops = payload
                .operations
                .into_iter()
                .map(language::proto::deserialize_operation)
                .collect::<Result<Vec<_>, _>>()?;
            let is_remote = this.is_remote();
            match this.opened_buffers.entry(buffer_id) {
                hash_map::Entry::Occupied(mut e) => match e.get_mut() {
                    OpenBuffer::Strong(buffer) => {
                        buffer.update(cx, |buffer, cx| buffer.apply_ops(ops, cx))?;
                    }
                    OpenBuffer::Operations(operations) => operations.extend_from_slice(&ops),
                    OpenBuffer::Weak(_) => {}
                },
                hash_map::Entry::Vacant(e) => {
                    assert!(
                        is_remote,
                        "received buffer update from {:?}",
                        envelope.original_sender_id
                    );
                    e.insert(OpenBuffer::Operations(ops));
                }
            }
            Ok(proto::Ack {})
        })
    }

    async fn handle_create_buffer_for_peer(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::CreateBufferForPeer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            match envelope
                .payload
                .variant
                .ok_or_else(|| anyhow!("missing variant"))?
            {
                proto::create_buffer_for_peer::Variant::State(mut state) => {
                    let mut buffer_file = None;
                    if let Some(file) = state.file.take() {
                        let worktree_id = WorktreeId::from_proto(file.worktree_id);
                        let worktree = this.worktree_for_id(worktree_id, cx).ok_or_else(|| {
                            anyhow!("no worktree found for id {}", file.worktree_id)
                        })?;
                        buffer_file = Some(Arc::new(File::from_proto(file, worktree.clone(), cx)?)
                            as Arc<dyn language::File>);
                    }

                    let buffer_id = state.id;
                    let buffer = cx.add_model(|_| {
                        Buffer::from_proto(this.replica_id(), state, buffer_file).unwrap()
                    });
                    this.incomplete_remote_buffers
                        .insert(buffer_id, Some(buffer));
                }
                proto::create_buffer_for_peer::Variant::Chunk(chunk) => {
                    let buffer = this
                        .incomplete_remote_buffers
                        .get(&chunk.buffer_id)
                        .cloned()
                        .flatten()
                        .ok_or_else(|| {
                            anyhow!(
                                "received chunk for buffer {} without initial state",
                                chunk.buffer_id
                            )
                        })?;
                    let operations = chunk
                        .operations
                        .into_iter()
                        .map(language::proto::deserialize_operation)
                        .collect::<Result<Vec<_>>>()?;
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(operations, cx))?;

                    if chunk.is_last {
                        this.incomplete_remote_buffers.remove(&chunk.buffer_id);
                        this.register_buffer(&buffer, cx)?;
                    }
                }
            }

            Ok(())
        })
    }

    async fn handle_update_diff_base(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::UpdateDiffBase>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let buffer_id = envelope.payload.buffer_id;
            let diff_base = envelope.payload.diff_base;
            if let Some(buffer) = this
                .opened_buffers
                .get_mut(&buffer_id)
                .and_then(|b| b.upgrade(cx))
                .or_else(|| {
                    this.incomplete_remote_buffers
                        .get(&buffer_id)
                        .cloned()
                        .flatten()
                })
            {
                buffer.update(cx, |buffer, cx| buffer.set_diff_base(diff_base, cx));
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
        let buffer_id = envelope.payload.buffer_id;

        this.update(&mut cx, |this, cx| {
            let payload = envelope.payload.clone();
            if let Some(buffer) = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|b| b.upgrade(cx))
                .or_else(|| {
                    this.incomplete_remote_buffers
                        .get(&buffer_id)
                        .cloned()
                        .flatten()
                })
            {
                let file = payload.file.ok_or_else(|| anyhow!("invalid file"))?;
                let worktree = this
                    .worktree_for_id(WorktreeId::from_proto(file.worktree_id), cx)
                    .ok_or_else(|| anyhow!("no such worktree"))?;
                let file = File::from_proto(file, worktree, cx)?;
                buffer.update(cx, |buffer, cx| {
                    buffer.file_updated(Arc::new(file), cx).detach();
                });
                this.detect_language_for_buffer(&buffer, cx);
            }
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
        let (project_id, buffer) = this.update(&mut cx, |this, cx| {
            let project_id = this.remote_id().ok_or_else(|| anyhow!("not connected"))?;
            let buffer = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade(cx))
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?;
            anyhow::Ok((project_id, buffer))
        })?;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&envelope.payload.version))
            })
            .await?;
        let buffer_id = buffer.read_with(&cx, |buffer, _| buffer.remote_id());

        let (saved_version, fingerprint, mtime) = this
            .update(&mut cx, |this, cx| this.save_buffer(buffer, cx))
            .await?;
        Ok(proto::BufferSaved {
            project_id,
            buffer_id,
            version: serialize_version(&saved_version),
            mtime: Some(mtime.into()),
            fingerprint: language::proto::serialize_fingerprint(fingerprint),
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

    async fn handle_synchronize_buffers(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::SynchronizeBuffers>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::SynchronizeBuffersResponse> {
        let project_id = envelope.payload.project_id;
        let mut response = proto::SynchronizeBuffersResponse {
            buffers: Default::default(),
        };

        this.update(&mut cx, |this, cx| {
            let Some(guest_id) = envelope.original_sender_id else {
                log::error!("missing original_sender_id on SynchronizeBuffers request");
                return;
            };

            this.shared_buffers.entry(guest_id).or_default().clear();
            for buffer in envelope.payload.buffers {
                let buffer_id = buffer.id;
                let remote_version = language::proto::deserialize_version(&buffer.version);
                if let Some(buffer) = this.buffer_for_id(buffer_id, cx) {
                    this.shared_buffers
                        .entry(guest_id)
                        .or_default()
                        .insert(buffer_id);

                    let buffer = buffer.read(cx);
                    response.buffers.push(proto::BufferVersion {
                        id: buffer_id,
                        version: language::proto::serialize_version(&buffer.version),
                    });

                    let operations = buffer.serialize_ops(Some(remote_version), cx);
                    let client = this.client.clone();
                    if let Some(file) = buffer.file() {
                        client
                            .send(proto::UpdateBufferFile {
                                project_id,
                                buffer_id: buffer_id as u64,
                                file: Some(file.to_proto()),
                            })
                            .log_err();
                    }

                    client
                        .send(proto::UpdateDiffBase {
                            project_id,
                            buffer_id: buffer_id as u64,
                            diff_base: buffer.diff_base().map(Into::into),
                        })
                        .log_err();

                    client
                        .send(proto::BufferReloaded {
                            project_id,
                            buffer_id,
                            version: language::proto::serialize_version(buffer.saved_version()),
                            mtime: Some(buffer.saved_mtime().into()),
                            fingerprint: language::proto::serialize_fingerprint(
                                buffer.saved_version_fingerprint(),
                            ),
                            line_ending: language::proto::serialize_line_ending(
                                buffer.line_ending(),
                            ) as i32,
                        })
                        .log_err();

                    cx.background()
                        .spawn(
                            async move {
                                let operations = operations.await;
                                for chunk in split_operations(operations) {
                                    client
                                        .request(proto::UpdateBuffer {
                                            project_id,
                                            buffer_id,
                                            operations: chunk,
                                        })
                                        .await?;
                                }
                                anyhow::Ok(())
                            }
                            .log_err(),
                        )
                        .detach();
                }
            }
        });

        Ok(response)
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
            let trigger = FormatTrigger::from_proto(envelope.payload.trigger);
            Ok::<_, anyhow::Error>(this.format(buffers, false, trigger, cx))
        })?;

        let project_transaction = format.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::FormatBuffersResponse {
            transaction: Some(project_transaction),
        })
    }

    async fn handle_apply_additional_edits_for_completion(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::ApplyCompletionAdditionalEdits>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ApplyCompletionAdditionalEditsResponse> {
        let (buffer, completion) = this.update(&mut cx, |this, cx| {
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
                language.cloned(),
            );
            Ok::<_, anyhow::Error>((buffer, completion))
        })?;

        let completion = completion.await?;

        let apply_additional_edits = this.update(&mut cx, |this, cx| {
            this.apply_additional_edits_for_completion(buffer, completion, false, cx)
        });

        Ok(proto::ApplyCompletionAdditionalEditsResponse {
            transaction: apply_additional_edits
                .await?
                .as_ref()
                .map(language::proto::serialize_transaction),
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
                    let buffer_id = this.create_buffer_for_peer(&buffer, peer_id, cx);
                    locations.push(proto::Location {
                        buffer_id,
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
        let symbol = this
            .read_with(&cx, |this, _| this.deserialize_symbol(symbol))
            .await?;
        let symbol = this.read_with(&cx, |this, _| {
            let signature = this.symbol_signature(&symbol.path);
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
            buffer_id: this.update(&mut cx, |this, cx| {
                this.create_buffer_for_peer(&buffer, peer_id, cx)
            }),
        })
    }

    fn symbol_signature(&self, project_path: &ProjectPath) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(project_path.worktree_id.to_proto().to_be_bytes());
        hasher.update(project_path.path.to_string_lossy().as_bytes());
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
                buffer_id: this.create_buffer_for_peer(&buffer, peer_id, cx),
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
                buffer_id: this.create_buffer_for_peer(&buffer, peer_id, cx),
            })
        })
    }

    fn serialize_project_transaction_for_peer(
        &mut self,
        project_transaction: ProjectTransaction,
        peer_id: proto::PeerId,
        cx: &mut AppContext,
    ) -> proto::ProjectTransaction {
        let mut serialized_transaction = proto::ProjectTransaction {
            buffer_ids: Default::default(),
            transactions: Default::default(),
        };
        for (buffer, transaction) in project_transaction.0 {
            serialized_transaction
                .buffer_ids
                .push(self.create_buffer_for_peer(&buffer, peer_id, cx));
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
            for (buffer_id, transaction) in message.buffer_ids.into_iter().zip(message.transactions)
            {
                let buffer = this
                    .update(&mut cx, |this, cx| {
                        this.wait_for_remote_buffer(buffer_id, cx)
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
                    .await?;

                if push_to_history {
                    buffer.update(&mut cx, |buffer, _| {
                        buffer.push_transaction(transaction.clone(), Instant::now());
                    });
                }
            }

            Ok(project_transaction)
        })
    }

    fn create_buffer_for_peer(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut AppContext,
    ) -> u64 {
        let buffer_id = buffer.read(cx).remote_id();
        if let Some(ProjectClientState::Local { updates_tx, .. }) = &self.client_state {
            updates_tx
                .unbounded_send(LocalProjectUpdate::CreateBufferForPeer { peer_id, buffer_id })
                .ok();
        }
        buffer_id
    }

    fn wait_for_remote_buffer(
        &mut self,
        id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<Buffer>>> {
        let mut opened_buffer_rx = self.opened_buffer.1.clone();

        cx.spawn_weak(|this, mut cx| async move {
            let buffer = loop {
                let Some(this) = this.upgrade(&cx) else {
                    return Err(anyhow!("project dropped"));
                };
                let buffer = this.read_with(&cx, |this, cx| {
                    this.opened_buffers
                        .get(&id)
                        .and_then(|buffer| buffer.upgrade(cx))
                });
                if let Some(buffer) = buffer {
                    break buffer;
                } else if this.read_with(&cx, |this, _| this.is_read_only()) {
                    return Err(anyhow!("disconnected before buffer {} could be opened", id));
                }

                this.update(&mut cx, |this, _| {
                    this.incomplete_remote_buffers.entry(id).or_default();
                });
                drop(this);
                opened_buffer_rx
                    .next()
                    .await
                    .ok_or_else(|| anyhow!("project dropped while waiting for buffer"))?;
            };
            buffer.update(&mut cx, |buffer, cx| buffer.git_diff_recalc(cx));
            Ok(buffer)
        })
    }

    fn synchronize_remote_buffers(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let project_id = match self.client_state.as_ref() {
            Some(ProjectClientState::Remote {
                sharing_has_stopped,
                remote_id,
                ..
            }) => {
                if *sharing_has_stopped {
                    return Task::ready(Err(anyhow!(
                        "can't synchronize remote buffers on a readonly project"
                    )));
                } else {
                    *remote_id
                }
            }
            Some(ProjectClientState::Local { .. }) | None => {
                return Task::ready(Err(anyhow!(
                    "can't synchronize remote buffers on a local project"
                )))
            }
        };

        let client = self.client.clone();
        cx.spawn(|this, cx| async move {
            let (buffers, incomplete_buffer_ids) = this.read_with(&cx, |this, cx| {
                let buffers = this
                    .opened_buffers
                    .iter()
                    .filter_map(|(id, buffer)| {
                        let buffer = buffer.upgrade(cx)?;
                        Some(proto::BufferVersion {
                            id: *id,
                            version: language::proto::serialize_version(&buffer.read(cx).version),
                        })
                    })
                    .collect();
                let incomplete_buffer_ids = this
                    .incomplete_remote_buffers
                    .keys()
                    .copied()
                    .collect::<Vec<_>>();

                (buffers, incomplete_buffer_ids)
            });
            let response = client
                .request(proto::SynchronizeBuffers {
                    project_id,
                    buffers,
                })
                .await?;

            let send_updates_for_buffers = response.buffers.into_iter().map(|buffer| {
                let client = client.clone();
                let buffer_id = buffer.id;
                let remote_version = language::proto::deserialize_version(&buffer.version);
                this.read_with(&cx, |this, cx| {
                    if let Some(buffer) = this.buffer_for_id(buffer_id, cx) {
                        let operations = buffer.read(cx).serialize_ops(Some(remote_version), cx);
                        cx.background().spawn(async move {
                            let operations = operations.await;
                            for chunk in split_operations(operations) {
                                client
                                    .request(proto::UpdateBuffer {
                                        project_id,
                                        buffer_id,
                                        operations: chunk,
                                    })
                                    .await?;
                            }
                            anyhow::Ok(())
                        })
                    } else {
                        Task::ready(Ok(()))
                    }
                })
            });

            // Any incomplete buffers have open requests waiting. Request that the host sends
            // creates these buffers for us again to unblock any waiting futures.
            for id in incomplete_buffer_ids {
                cx.background()
                    .spawn(client.request(proto::OpenBufferById { project_id, id }))
                    .detach();
            }

            futures::future::join_all(send_updates_for_buffers)
                .await
                .into_iter()
                .collect()
        })
    }

    pub fn worktree_metadata_protos(&self, cx: &AppContext) -> Vec<proto::WorktreeMetadata> {
        self.worktrees(cx)
            .map(|worktree| {
                let worktree = worktree.read(cx);
                proto::WorktreeMetadata {
                    id: worktree.id().to_proto(),
                    root_name: worktree.root_name().into(),
                    visible: worktree.is_visible(),
                    abs_path: worktree.abs_path().to_string_lossy().into(),
                }
            })
            .collect()
    }

    fn set_worktrees_from_proto(
        &mut self,
        worktrees: Vec<proto::WorktreeMetadata>,
        cx: &mut ModelContext<Project>,
    ) -> Result<()> {
        let replica_id = self.replica_id();
        let remote_id = self.remote_id().ok_or_else(|| anyhow!("invalid project"))?;

        let mut old_worktrees_by_id = self
            .worktrees
            .drain(..)
            .filter_map(|worktree| {
                let worktree = worktree.upgrade(cx)?;
                Some((worktree.read(cx).id(), worktree))
            })
            .collect::<HashMap<_, _>>();

        for worktree in worktrees {
            if let Some(old_worktree) =
                old_worktrees_by_id.remove(&WorktreeId::from_proto(worktree.id))
            {
                self.worktrees.push(WorktreeHandle::Strong(old_worktree));
            } else {
                let worktree =
                    Worktree::remote(remote_id, replica_id, worktree, self.client.clone(), cx);
                let _ = self.add_worktree(&worktree, cx);
            }
        }

        self.metadata_changed(cx);
        for (id, _) in old_worktrees_by_id {
            cx.emit(Event::WorktreeRemoved(id));
        }

        Ok(())
    }

    fn set_collaborators_from_proto(
        &mut self,
        messages: Vec<proto::Collaborator>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let mut collaborators = HashMap::default();
        for message in messages {
            let collaborator = Collaborator::from_proto(message)?;
            collaborators.insert(collaborator.peer_id, collaborator);
        }
        for old_peer_id in self.collaborators.keys() {
            if !collaborators.contains_key(old_peer_id) {
                cx.emit(Event::CollaboratorLeft(*old_peer_id));
            }
        }
        self.collaborators = collaborators;
        Ok(())
    }

    fn deserialize_symbol(
        &self,
        serialized_symbol: proto::Symbol,
    ) -> impl Future<Output = Result<Symbol>> {
        let languages = self.languages.clone();
        async move {
            let source_worktree_id = WorktreeId::from_proto(serialized_symbol.source_worktree_id);
            let worktree_id = WorktreeId::from_proto(serialized_symbol.worktree_id);
            let start = serialized_symbol
                .start
                .ok_or_else(|| anyhow!("invalid start"))?;
            let end = serialized_symbol
                .end
                .ok_or_else(|| anyhow!("invalid end"))?;
            let kind = unsafe { mem::transmute(serialized_symbol.kind) };
            let path = ProjectPath {
                worktree_id,
                path: PathBuf::from(serialized_symbol.path).into(),
            };
            let language = languages
                .language_for_file(&path.path, None)
                .await
                .log_err();
            Ok(Symbol {
                language_server_name: LanguageServerName(
                    serialized_symbol.language_server_name.into(),
                ),
                source_worktree_id,
                path,
                label: {
                    match language {
                        Some(language) => {
                            language
                                .label_for_symbol(&serialized_symbol.name, kind)
                                .await
                        }
                        None => None,
                    }
                    .unwrap_or_else(|| CodeLabel::plain(serialized_symbol.name.clone(), None))
                },

                name: serialized_symbol.name,
                range: Unclipped(PointUtf16::new(start.row, start.column))
                    ..Unclipped(PointUtf16::new(end.row, end.column)),
                kind,
                signature: serialized_symbol
                    .signature
                    .try_into()
                    .map_err(|_| anyhow!("invalid signature"))?,
            })
        }
    }

    async fn handle_buffer_saved(
        this: ModelHandle<Self>,
        envelope: TypedEnvelope<proto::BufferSaved>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let fingerprint = deserialize_fingerprint(&envelope.payload.fingerprint)?;
        let version = deserialize_version(&envelope.payload.version);
        let mtime = envelope
            .payload
            .mtime
            .ok_or_else(|| anyhow!("missing mtime"))?
            .into();

        this.update(&mut cx, |this, cx| {
            let buffer = this
                .opened_buffers
                .get(&envelope.payload.buffer_id)
                .and_then(|buffer| buffer.upgrade(cx))
                .or_else(|| {
                    this.incomplete_remote_buffers
                        .get(&envelope.payload.buffer_id)
                        .and_then(|b| b.clone())
                });
            if let Some(buffer) = buffer {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_save(version, fingerprint, mtime, cx);
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
        let version = deserialize_version(&payload.version);
        let fingerprint = deserialize_fingerprint(&payload.fingerprint)?;
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
                .and_then(|buffer| buffer.upgrade(cx))
                .or_else(|| {
                    this.incomplete_remote_buffers
                        .get(&payload.buffer_id)
                        .cloned()
                        .flatten()
                });
            if let Some(buffer) = buffer {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_reload(version, fingerprint, line_ending, mtime, cx);
                });
            }
            Ok(())
        })
    }

    #[allow(clippy::type_complexity)]
    fn edits_from_lsp(
        &mut self,
        buffer: &ModelHandle<Buffer>,
        lsp_edits: impl 'static + Send + IntoIterator<Item = lsp::TextEdit>,
        server_id: LanguageServerId,
        version: Option<i32>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<(Range<Anchor>, String)>>> {
        let snapshot = self.buffer_snapshot_for_lsp_version(buffer, server_id, version, cx);
        cx.background().spawn(async move {
            let snapshot = snapshot?;
            let mut lsp_edits = lsp_edits
                .into_iter()
                .map(|edit| (range_from_lsp(edit.range), edit.new_text))
                .collect::<Vec<_>>();
            lsp_edits.sort_by_key(|(range, _)| range.start);

            let mut lsp_edits = lsp_edits.into_iter().peekable();
            let mut edits = Vec::new();
            while let Some((range, mut new_text)) = lsp_edits.next() {
                // Clip invalid ranges provided by the language server.
                let mut range = snapshot.clip_point_utf16(range.start, Bias::Left)
                    ..snapshot.clip_point_utf16(range.end, Bias::Left);

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
                    if next_range.start.0 > range.end {
                        if next_range.start.0.row > range.end.row + 1
                            || next_range.start.0.column > 0
                            || snapshot.clip_point_utf16(
                                Unclipped(PointUtf16::new(range.end.row, u32::MAX)),
                                Bias::Left,
                            ) > range.end
                        {
                            break;
                        }
                        new_text.push('\n');
                    }
                    range.end = snapshot.clip_point_utf16(next_range.end, Bias::Left);
                    new_text.push_str(next_text);
                    lsp_edits.next();
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
                                    edits.push((anchor..anchor, value.to_string()));
                                } else {
                                    edits.last_mut().unwrap().1.push_str(value);
                                }
                                moved_since_edit = false;
                            }
                        }
                    }
                } else if range.end == range.start {
                    let anchor = snapshot.anchor_after(range.start);
                    edits.push((anchor..anchor, new_text));
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
        server_id: LanguageServerId,
        version: Option<i32>,
        cx: &AppContext,
    ) -> Result<TextBufferSnapshot> {
        const OLD_VERSIONS_TO_RETAIN: i32 = 10;

        if let Some(version) = version {
            let buffer_id = buffer.read(cx).remote_id();
            let snapshots = self
                .buffer_snapshots
                .get_mut(&buffer_id)
                .and_then(|m| m.get_mut(&server_id))
                .ok_or_else(|| {
                    anyhow!("no snapshots found for buffer {buffer_id} and server {server_id}")
                })?;

            let found_snapshot = snapshots
                .binary_search_by_key(&version, |e| e.version)
                .map(|ix| snapshots[ix].snapshot.clone())
                .map_err(|_| {
                    anyhow!("snapshot not found for buffer {buffer_id} server {server_id} at version {version}")
                })?;

            snapshots.retain(|snapshot| snapshot.version + OLD_VERSIONS_TO_RETAIN >= version);
            Ok(found_snapshot)
        } else {
            Ok((buffer.read(cx)).text_snapshot())
        }
    }

    pub fn language_servers_iter_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> impl Iterator<Item = (&Arc<CachedLspAdapter>, &Arc<LanguageServer>)> {
        self.language_server_ids_for_buffer(buffer, cx)
            .into_iter()
            .filter_map(|server_id| {
                let server = self.language_servers.get(&server_id)?;
                if let LanguageServerState::Running {
                    adapter, server, ..
                } = server
                {
                    Some((adapter, server))
                } else {
                    None
                }
            })
    }

    fn language_servers_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Vec<(&Arc<CachedLspAdapter>, &Arc<LanguageServer>)> {
        self.language_servers_iter_for_buffer(buffer, cx).collect()
    }

    fn primary_language_servers_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Option<(&Arc<CachedLspAdapter>, &Arc<LanguageServer>)> {
        self.language_servers_iter_for_buffer(buffer, cx).next()
    }

    fn language_server_for_buffer(
        &self,
        buffer: &Buffer,
        server_id: LanguageServerId,
        cx: &AppContext,
    ) -> Option<(&Arc<CachedLspAdapter>, &Arc<LanguageServer>)> {
        self.language_servers_iter_for_buffer(buffer, cx)
            .find(|(_, s)| s.server_id() == server_id)
    }

    fn language_server_ids_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Vec<LanguageServerId> {
        if let Some((file, language)) = File::from_dyn(buffer.file()).zip(buffer.language()) {
            let worktree_id = file.worktree_id(cx);
            language
                .lsp_adapters()
                .iter()
                .flat_map(|adapter| {
                    let key = (worktree_id, adapter.name.clone());
                    self.language_server_ids.get(&key).copied()
                })
                .collect()
        } else {
            Vec::new()
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
            OpenBuffer::Operations(_) => None,
        }
    }
}

pub struct PathMatchCandidateSet {
    pub snapshot: Snapshot,
    pub include_ignored: bool,
    pub include_root_name: bool,
}

impl<'a> fuzzy::PathMatchCandidateSet<'a> for PathMatchCandidateSet {
    type Candidates = PathMatchCandidateSetIter<'a>;

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
        PathMatchCandidateSetIter {
            traversal: self.snapshot.files(self.include_ignored, start),
        }
    }
}

pub struct PathMatchCandidateSetIter<'a> {
    traversal: Traversal<'a>,
}

impl<'a> Iterator for PathMatchCandidateSetIter<'a> {
    type Item = fuzzy::PathMatchCandidate<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.traversal.next().map(|entry| {
            if let EntryKind::File(char_bag) = entry.kind {
                fuzzy::PathMatchCandidate {
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

    fn release(&mut self, cx: &mut gpui::AppContext) {
        match &self.client_state {
            Some(ProjectClientState::Local { .. }) => {
                let _ = self.unshare_internal(cx);
            }
            Some(ProjectClientState::Remote { remote_id, .. }) => {
                let _ = self.client.send(proto::LeaveProject {
                    project_id: *remote_id,
                });
                self.disconnected_from_host_internal(cx);
            }
            _ => {}
        }
    }

    fn app_will_quit(
        &mut self,
        _: &mut AppContext,
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
    fn from_proto(message: proto::Collaborator) -> Result<Self> {
        Ok(Self {
            peer_id: message.peer_id.ok_or_else(|| anyhow!("invalid peer id"))?,
            replica_id: message.replica_id as ReplicaId,
        })
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

fn split_operations(
    mut operations: Vec<proto::Operation>,
) -> impl Iterator<Item = Vec<proto::Operation>> {
    #[cfg(any(test, feature = "test-support"))]
    const CHUNK_SIZE: usize = 5;

    #[cfg(not(any(test, feature = "test-support")))]
    const CHUNK_SIZE: usize = 100;

    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            return None;
        }

        let operations = operations
            .drain(..cmp::min(CHUNK_SIZE, operations.len()))
            .collect::<Vec<_>>();
        if operations.is_empty() {
            done = true;
        }
        Some(operations)
    })
}

fn serialize_symbol(symbol: &Symbol) -> proto::Symbol {
    proto::Symbol {
        language_server_name: symbol.language_server_name.0.to_string(),
        source_worktree_id: symbol.source_worktree_id.to_proto(),
        worktree_id: symbol.path.worktree_id.to_proto(),
        path: symbol.path.path.to_string_lossy().to_string(),
        name: symbol.name.clone(),
        kind: unsafe { mem::transmute(symbol.kind) },
        start: Some(proto::PointUtf16 {
            row: symbol.range.start.0.row,
            column: symbol.range.start.0.column,
        }),
        end: Some(proto::PointUtf16 {
            row: symbol.range.end.0.row,
            column: symbol.range.end.0.column,
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

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        File::from_dyn(self.file()).map(|file| ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }
}
