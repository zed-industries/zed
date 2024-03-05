pub mod debounced_delay;
pub mod lsp_command;
pub mod lsp_ext_command;
mod prettier_support;
pub mod search;
mod task_inventory;
pub mod terminals;

#[cfg(test)]
mod project_tests;

use anyhow::{anyhow, bail, Context as _, Result};
use async_trait::async_trait;
use client::{proto, Client, Collaborator, HostedProjectId, TypedEnvelope, UserStore};
use clock::ReplicaId;
use collections::{hash_map, BTreeMap, HashMap, HashSet, VecDeque};
use copilot::Copilot;
use debounced_delay::DebouncedDelay;
use fs::repository::GitRepository;
use futures::{
    channel::mpsc::{self, UnboundedReceiver},
    future::{try_join_all, Shared},
    select,
    stream::FuturesUnordered,
    AsyncWriteExt, Future, FutureExt, StreamExt, TryFutureExt,
};
use globset::{Glob, GlobSet, GlobSetBuilder};
use gpui::{
    AnyModel, AppContext, AsyncAppContext, BackgroundExecutor, Context, Entity, EventEmitter,
    Model, ModelContext, PromptLevel, Task, WeakModel,
};
use itertools::Itertools;
use language::{
    language_settings::{language_settings, FormatOnSave, Formatter, InlayHintKind},
    markdown, point_to_lsp,
    proto::{
        deserialize_anchor, deserialize_fingerprint, deserialize_line_ending, deserialize_version,
        serialize_anchor, serialize_version, split_operations,
    },
    range_from_lsp, range_to_lsp, Bias, Buffer, BufferSnapshot, CachedLspAdapter, Capability,
    CodeAction, CodeLabel, Completion, Diagnostic, DiagnosticEntry, DiagnosticSet, Diff,
    Documentation, Event as BufferEvent, File as _, Language, LanguageRegistry, LanguageServerName,
    LocalFile, LspAdapterDelegate, OffsetRangeExt, Operation, Patch, PendingLanguageServer,
    PointUtf16, TextBufferSnapshot, ToOffset, ToPointUtf16, Transaction, Unclipped,
};
use log::error;
use lsp::{
    DiagnosticSeverity, DiagnosticTag, DidChangeWatchedFilesRegistrationOptions,
    DocumentHighlightKind, LanguageServer, LanguageServerBinary, LanguageServerId,
    MessageActionItem, OneOf, ServerHealthStatus, ServerStatus,
};
use lsp_command::*;
use node_runtime::NodeRuntime;
use parking_lot::{Mutex, RwLock};
use postage::watch;
use prettier_support::{DefaultPrettier, PrettierInstance};
use project_core::project_settings::{LspSettings, ProjectSettings};
pub use project_core::{DiagnosticSummary, ProjectEntryId};
use rand::prelude::*;

use rpc::{ErrorCode, ErrorExt as _};
use search::SearchQuery;
use serde::Serialize;
use settings::{watch_config_file, Settings, SettingsStore};
use sha2::{Digest, Sha256};
use similar::{ChangeTag, TextDiff};
use smol::channel::{Receiver, Sender};
use smol::lock::Semaphore;
use std::{
    cmp::{self, Ordering},
    convert::TryInto,
    env,
    ffi::OsString,
    hash::Hash,
    mem,
    num::NonZeroU32,
    ops::Range,
    path::{self, Component, Path, PathBuf},
    process::Stdio,
    str,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
    time::{Duration, Instant},
};
use task::static_source::StaticSource;
use terminals::Terminals;
use text::{Anchor, BufferId};
use util::{
    debug_panic, defer,
    http::HttpClient,
    merge_json_value_into,
    paths::{LOCAL_SETTINGS_RELATIVE_PATH, LOCAL_TASKS_RELATIVE_PATH},
    post_inc, ResultExt, TryFutureExt as _,
};

pub use fs::*;
pub use language::Location;
#[cfg(any(test, feature = "test-support"))]
pub use prettier::FORMAT_SUFFIX as TEST_PRETTIER_FORMAT_SUFFIX;
pub use project_core::project_settings;
pub use project_core::worktree::{self, *};
#[cfg(feature = "test-support")]
pub use task_inventory::test_inventory::*;
pub use task_inventory::{Inventory, TaskSourceKind};

const MAX_SERVER_REINSTALL_ATTEMPT_COUNT: u64 = 4;
const SERVER_REINSTALL_DEBOUNCE_TIMEOUT: Duration = Duration::from_secs(1);
const SERVER_LAUNCHING_BEFORE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

pub trait Item {
    fn entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId>;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
}

pub struct Project {
    worktrees: Vec<WorktreeHandle>,
    active_entry: Option<ProjectEntryId>,
    buffer_ordered_messages_tx: mpsc::UnboundedSender<BufferOrderedMessage>,
    languages: Arc<LanguageRegistry>,
    supplementary_language_servers:
        HashMap<LanguageServerId, (LanguageServerName, Arc<LanguageServer>)>,
    language_servers: HashMap<LanguageServerId, LanguageServerState>,
    language_server_ids: HashMap<(WorktreeId, LanguageServerName), LanguageServerId>,
    language_server_statuses: BTreeMap<LanguageServerId, LanguageServerStatus>,
    last_workspace_edits_by_language_server: HashMap<LanguageServerId, ProjectTransaction>,
    client: Arc<client::Client>,
    next_entry_id: Arc<AtomicUsize>,
    join_project_response_message_id: u32,
    next_diagnostic_group_id: usize,
    user_store: Model<UserStore>,
    fs: Arc<dyn Fs>,
    client_state: ProjectClientState,
    collaborators: HashMap<proto::PeerId, Collaborator>,
    client_subscriptions: Vec<client::Subscription>,
    _subscriptions: Vec<gpui::Subscription>,
    next_buffer_id: BufferId,
    opened_buffer: (watch::Sender<()>, watch::Receiver<()>),
    shared_buffers: HashMap<proto::PeerId, HashSet<BufferId>>,
    #[allow(clippy::type_complexity)]
    loading_buffers_by_path: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<Model<Buffer>, Arc<anyhow::Error>>>>,
    >,
    #[allow(clippy::type_complexity)]
    loading_local_worktrees:
        HashMap<Arc<Path>, Shared<Task<Result<Model<Worktree>, Arc<anyhow::Error>>>>>,
    opened_buffers: HashMap<BufferId, OpenBuffer>,
    local_buffer_ids_by_path: HashMap<ProjectPath, BufferId>,
    local_buffer_ids_by_entry_id: HashMap<ProjectEntryId, BufferId>,
    /// A mapping from a buffer ID to None means that we've started waiting for an ID but haven't finished loading it.
    /// Used for re-issuing buffer requests when peers temporarily disconnect
    incomplete_remote_buffers: HashMap<BufferId, Option<Model<Buffer>>>,
    buffer_snapshots: HashMap<BufferId, HashMap<LanguageServerId, Vec<LspBufferSnapshot>>>, // buffer_id -> server_id -> vec of snapshots
    buffers_being_formatted: HashSet<BufferId>,
    buffers_needing_diff: HashSet<WeakModel<Buffer>>,
    git_diff_debouncer: DebouncedDelay,
    nonce: u128,
    _maintain_buffer_languages: Task<()>,
    _maintain_workspace_config: Task<Result<()>>,
    terminals: Terminals,
    copilot_lsp_subscription: Option<gpui::Subscription>,
    copilot_log_subscription: Option<lsp::Subscription>,
    current_lsp_settings: HashMap<Arc<str>, LspSettings>,
    node: Option<Arc<dyn NodeRuntime>>,
    default_prettier: DefaultPrettier,
    prettiers_per_worktree: HashMap<WorktreeId, HashSet<Option<PathBuf>>>,
    prettier_instances: HashMap<PathBuf, PrettierInstance>,
    tasks: Model<Inventory>,
    hosted_project_id: Option<HostedProjectId>,
}

pub enum LanguageServerToQuery {
    Primary,
    Other(LanguageServerId),
}

struct LspBufferSnapshot {
    version: i32,
    snapshot: TextBufferSnapshot,
}

/// Message ordered with respect to buffer operations
enum BufferOrderedMessage {
    Operation {
        buffer_id: BufferId,
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
        buffer_id: BufferId,
    },
}

enum OpenBuffer {
    Strong(Model<Buffer>),
    Weak(WeakModel<Buffer>),
    Operations(Vec<Operation>),
}

#[derive(Clone)]
enum WorktreeHandle {
    Strong(Model<Worktree>),
    Weak(WeakModel<Worktree>),
}

#[derive(Debug)]
enum ProjectClientState {
    Local,
    Shared {
        remote_id: u64,
        updates_tx: mpsc::UnboundedSender<LocalProjectUpdate>,
        _send_updates: Task<Result<()>>,
    },
    Remote {
        sharing_has_stopped: bool,
        capability: Capability,
        remote_id: u64,
        replica_id: ReplicaId,
    },
}

/// A prompt requested by LSP server.
#[derive(Clone, Debug)]
pub struct LanguageServerPromptRequest {
    pub level: PromptLevel,
    pub message: String,
    pub actions: Vec<MessageActionItem>,
    pub lsp_name: String,
    response_channel: Sender<MessageActionItem>,
}

impl LanguageServerPromptRequest {
    pub async fn respond(self, index: usize) -> Option<()> {
        if let Some(response) = self.actions.into_iter().nth(index) {
            self.response_channel.send(response).await.ok()
        } else {
            None
        }
    }
}
impl PartialEq for LanguageServerPromptRequest {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message && self.actions == other.actions
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    LanguageServerAdded(LanguageServerId),
    LanguageServerRemoved(LanguageServerId),
    LanguageServerLog(LanguageServerId, String),
    Notification(String),
    LanguageServerPrompt(LanguageServerPromptRequest),
    ActiveEntryChanged(Option<ProjectEntryId>),
    ActivateProjectPanel,
    WorktreeAdded,
    WorktreeRemoved(WorktreeId),
    WorktreeUpdatedEntries(WorktreeId, UpdatedEntriesSet),
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
    DeletedEntry(ProjectEntryId),
    CollaboratorUpdated {
        old_peer_id: proto::PeerId,
        new_peer_id: proto::PeerId,
    },
    CollaboratorJoined(proto::PeerId),
    CollaboratorLeft(proto::PeerId),
    RefreshInlayHints,
    RevealInProjectPanel(ProjectEntryId),
}

pub enum LanguageServerState {
    Starting(Task<Option<Arc<LanguageServer>>>),

    Running {
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        server: Arc<LanguageServer>,
        watched_paths: HashMap<WorktreeId, GlobSet>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlayHint {
    pub position: language::Anchor,
    pub label: InlayHintLabel,
    pub kind: Option<InlayHintKind>,
    pub padding_left: bool,
    pub padding_right: bool,
    pub tooltip: Option<InlayHintTooltip>,
    pub resolve_state: ResolveState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveState {
    Resolved,
    CanResolve(LanguageServerId, Option<lsp::LSPAny>),
    Resolving,
}

impl InlayHint {
    pub fn text(&self) -> String {
        match &self.label {
            InlayHintLabel::String(s) => s.to_owned(),
            InlayHintLabel::LabelParts(parts) => parts.iter().map(|part| &part.value).join(""),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlayHintLabel {
    String(String),
    LabelParts(Vec<InlayHintLabelPart>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlayHintLabelPart {
    pub value: String,
    pub tooltip: Option<InlayHintLabelPartTooltip>,
    pub location: Option<(LanguageServerId, lsp::Location)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlayHintTooltip {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlayHintLabelPartTooltip {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkupContent {
    pub kind: HoverBlockKind,
    pub value: String,
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
    pub kind: HoverBlockKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoverBlockKind {
    PlainText,
    Markdown,
    Code { language: String },
}

#[derive(Debug)]
pub struct Hover {
    pub contents: Vec<HoverBlock>,
    pub range: Option<Range<language::Anchor>>,
    pub language: Option<Arc<Language>>,
}

impl Hover {
    pub fn is_empty(&self) -> bool {
        self.contents.iter().all(|block| block.text.is_empty())
    }
}

#[derive(Default)]
pub struct ProjectTransaction(pub HashMap<Model<Buffer>, language::Transaction>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatTrigger {
    Save,
    Manual,
}

// Currently, formatting operations are represented differently depending on
// whether they come from a language server or an external command.
enum FormatOperation {
    Lsp(Vec<(Range<Anchor>, String)>),
    External(Diff),
    Prettier(Diff),
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
#[derive(Clone, Debug, PartialEq)]
enum SearchMatchCandidate {
    OpenBuffer {
        buffer: Model<Buffer>,
        // This might be an unnamed file without representation on filesystem
        path: Option<Arc<Path>>,
    },
    Path {
        worktree_id: WorktreeId,
        is_ignored: bool,
        path: Arc<Path>,
    },
}

type SearchMatchCandidateIndex = usize;
impl SearchMatchCandidate {
    fn path(&self) -> Option<Arc<Path>> {
        match self {
            SearchMatchCandidate::OpenBuffer { path, .. } => path.clone(),
            SearchMatchCandidate::Path { path, .. } => Some(path.clone()),
        }
    }
}

impl Project {
    pub fn init_settings(cx: &mut AppContext) {
        ProjectSettings::register(cx);
    }

    pub fn init(client: &Arc<Client>, cx: &mut AppContext) {
        Self::init_settings(cx);

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
        client.add_model_message_handler(Self::handle_update_worktree_settings);
        client.add_model_request_handler(Self::handle_create_project_entry);
        client.add_model_request_handler(Self::handle_rename_project_entry);
        client.add_model_request_handler(Self::handle_copy_project_entry);
        client.add_model_request_handler(Self::handle_delete_project_entry);
        client.add_model_request_handler(Self::handle_expand_project_entry);
        client.add_model_request_handler(Self::handle_apply_additional_edits_for_completion);
        client.add_model_request_handler(Self::handle_resolve_completion_documentation);
        client.add_model_request_handler(Self::handle_apply_code_action);
        client.add_model_request_handler(Self::handle_on_type_formatting);
        client.add_model_request_handler(Self::handle_inlay_hints);
        client.add_model_request_handler(Self::handle_resolve_inlay_hint);
        client.add_model_request_handler(Self::handle_refresh_inlay_hints);
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
        client.add_model_request_handler(Self::handle_lsp_command::<lsp_ext_command::ExpandMacro>);
    }

    pub fn local(
        client: Arc<Client>,
        node: Arc<dyn NodeRuntime>,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx: &mut ModelContext<Self>| {
            let (tx, rx) = mpsc::unbounded();
            cx.spawn(move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx))
                .detach();
            let copilot_lsp_subscription =
                Copilot::global(cx).map(|copilot| subscribe_for_copilot_events(&copilot, cx));
            let tasks = Inventory::new(cx);

            Self {
                worktrees: Vec::new(),
                buffer_ordered_messages_tx: tx,
                collaborators: Default::default(),
                next_buffer_id: BufferId::new(1).unwrap(),
                opened_buffers: Default::default(),
                shared_buffers: Default::default(),
                incomplete_remote_buffers: Default::default(),
                loading_buffers_by_path: Default::default(),
                loading_local_worktrees: Default::default(),
                local_buffer_ids_by_path: Default::default(),
                local_buffer_ids_by_entry_id: Default::default(),
                buffer_snapshots: Default::default(),
                join_project_response_message_id: 0,
                client_state: ProjectClientState::Local,
                opened_buffer: watch::channel(),
                client_subscriptions: Vec::new(),
                _subscriptions: vec![
                    cx.observe_global::<SettingsStore>(Self::on_settings_changed),
                    cx.on_release(Self::release),
                    cx.on_app_quit(Self::shutdown_language_servers),
                ],
                _maintain_buffer_languages: Self::maintain_buffer_languages(languages.clone(), cx),
                _maintain_workspace_config: Self::maintain_workspace_config(cx),
                active_entry: None,
                languages,
                client,
                user_store,
                fs,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                supplementary_language_servers: HashMap::default(),
                language_servers: Default::default(),
                language_server_ids: HashMap::default(),
                language_server_statuses: Default::default(),
                last_workspace_edits_by_language_server: Default::default(),
                buffers_being_formatted: Default::default(),
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                nonce: StdRng::from_entropy().gen(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                copilot_lsp_subscription,
                copilot_log_subscription: None,
                current_lsp_settings: ProjectSettings::get_global(cx).lsp.clone(),
                node: Some(node),
                default_prettier: DefaultPrettier::default(),
                prettiers_per_worktree: HashMap::default(),
                prettier_instances: HashMap::default(),
                tasks,
                hosted_project_id: None,
            }
        })
    }

    pub async fn remote(
        remote_id: u64,
        client: Arc<Client>,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        client.authenticate_and_connect(true, &cx).await?;

        let response = client
            .request_envelope(proto::JoinProject {
                project_id: remote_id,
            })
            .await?;
        Self::from_join_project_response(response, None, client, user_store, languages, fs, cx)
            .await
    }
    async fn from_join_project_response(
        response: TypedEnvelope<proto::JoinProjectResponse>,
        hosted_project_id: Option<HostedProjectId>,
        client: Arc<Client>,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        mut cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        let remote_id = response.payload.project_id;
        let role = response.payload.role();
        let subscription = client.subscribe_to_entity(remote_id)?;
        let this = cx.new_model(|cx| {
            let replica_id = response.payload.replica_id as ReplicaId;
            let tasks = Inventory::new(cx);
            // BIG CAUTION NOTE: The order in which we initialize fields here matters and it should match what's done in Self::local.
            // Otherwise, you might run into issues where worktree id on remote is different than what's on local host.
            // That's because Worktree's identifier is entity id, which should probably be changed.
            let mut worktrees = Vec::new();
            for worktree in response.payload.worktrees {
                let worktree =
                    Worktree::remote(remote_id, replica_id, worktree, client.clone(), cx);
                worktrees.push(worktree);
            }

            let (tx, rx) = mpsc::unbounded();
            cx.spawn(move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx))
                .detach();
            let copilot_lsp_subscription =
                Copilot::global(cx).map(|copilot| subscribe_for_copilot_events(&copilot, cx));
            let mut this = Self {
                worktrees: Vec::new(),
                buffer_ordered_messages_tx: tx,
                loading_buffers_by_path: Default::default(),
                next_buffer_id: BufferId::new(1).unwrap(),
                opened_buffer: watch::channel(),
                shared_buffers: Default::default(),
                incomplete_remote_buffers: Default::default(),
                loading_local_worktrees: Default::default(),
                local_buffer_ids_by_path: Default::default(),
                local_buffer_ids_by_entry_id: Default::default(),
                active_entry: None,
                collaborators: Default::default(),
                join_project_response_message_id: response.message_id,
                _maintain_buffer_languages: Self::maintain_buffer_languages(languages.clone(), cx),
                _maintain_workspace_config: Self::maintain_workspace_config(cx),
                languages,
                user_store: user_store.clone(),
                fs,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                client_subscriptions: Default::default(),
                _subscriptions: vec![
                    cx.on_release(Self::release),
                    cx.on_app_quit(Self::shutdown_language_servers),
                ],
                client: client.clone(),
                client_state: ProjectClientState::Remote {
                    sharing_has_stopped: false,
                    capability: Capability::ReadWrite,
                    remote_id,
                    replica_id,
                },
                supplementary_language_servers: HashMap::default(),
                language_servers: Default::default(),
                language_server_ids: HashMap::default(),
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
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                buffer_snapshots: Default::default(),
                nonce: StdRng::from_entropy().gen(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                copilot_lsp_subscription,
                copilot_log_subscription: None,
                current_lsp_settings: ProjectSettings::get_global(cx).lsp.clone(),
                node: None,
                default_prettier: DefaultPrettier::default(),
                prettiers_per_worktree: HashMap::default(),
                prettier_instances: HashMap::default(),
                tasks,
                hosted_project_id,
            };
            this.set_role(role, cx);
            for worktree in worktrees {
                let _ = this.add_worktree(&worktree, cx);
            }
            this
        })?;
        let subscription = subscription.set_model(&this, &mut cx);

        let user_ids = response
            .payload
            .collaborators
            .iter()
            .map(|peer| peer.user_id)
            .collect();
        user_store
            .update(&mut cx, |user_store, cx| user_store.get_users(user_ids, cx))?
            .await?;

        this.update(&mut cx, |this, cx| {
            this.set_collaborators_from_proto(response.payload.collaborators, cx)?;
            this.client_subscriptions.push(subscription);
            anyhow::Ok(())
        })??;

        Ok(this)
    }

    pub async fn hosted(
        hosted_project_id: HostedProjectId,
        user_store: Model<UserStore>,
        client: Arc<Client>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        let response = client
            .request_envelope(proto::JoinHostedProject {
                id: hosted_project_id.0,
            })
            .await?;
        Self::from_join_project_response(
            response,
            Some(hosted_project_id),
            client,
            user_store,
            languages,
            fs,
            cx,
        )
        .await
    }

    fn release(&mut self, cx: &mut AppContext) {
        match &self.client_state {
            ProjectClientState::Local => {}
            ProjectClientState::Shared { .. } => {
                let _ = self.unshare_internal(cx);
            }
            ProjectClientState::Remote { remote_id, .. } => {
                let _ = self.client.send(proto::LeaveProject {
                    project_id: *remote_id,
                });
                self.disconnected_from_host_internal(cx);
            }
        }
    }

    fn shutdown_language_servers(
        &mut self,
        _cx: &mut ModelContext<Self>,
    ) -> impl Future<Output = ()> {
        let shutdown_futures = self
            .language_servers
            .drain()
            .map(|(_, server_state)| async {
                use LanguageServerState::*;
                match server_state {
                    Running { server, .. } => server.shutdown()?.await,
                    Starting(task) => task.await?.shutdown()?.await,
                }
            })
            .collect::<Vec<_>>();

        async move {
            futures::future::join_all(shutdown_futures).await;
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn test(
        fs: Arc<dyn Fs>,
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut gpui::TestAppContext,
    ) -> Model<Project> {
        use clock::FakeSystemClock;

        let mut languages = LanguageRegistry::test();
        languages.set_executor(cx.executor());
        let clock = Arc::new(FakeSystemClock::default());
        let http_client = util::http::FakeHttpClient::with_404_response();
        let client = cx.update(|cx| client::Client::new(clock, http_client.clone(), cx));
        let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
        let project = cx.update(|cx| {
            Project::local(
                client,
                node_runtime::FakeNodeRuntime::new(),
                user_store,
                Arc::new(languages),
                fs,
                cx,
            )
        });
        for path in root_paths {
            let (tree, _) = project
                .update(cx, |project, cx| {
                    project.find_or_create_local_worktree(path, true, cx)
                })
                .await
                .unwrap();
            tree.update(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .await;
        }
        project
    }

    fn on_settings_changed(&mut self, cx: &mut ModelContext<Self>) {
        let mut language_servers_to_start = Vec::new();
        let mut language_formatters_to_check = Vec::new();
        for buffer in self.opened_buffers.values() {
            if let Some(buffer) = buffer.upgrade() {
                let buffer = buffer.read(cx);
                let buffer_file = File::from_dyn(buffer.file());
                let buffer_language = buffer.language();
                let settings = language_settings(buffer_language, buffer.file(), cx);
                if let Some(language) = buffer_language {
                    if settings.enable_language_server {
                        if let Some(file) = buffer_file {
                            language_servers_to_start
                                .push((file.worktree.clone(), Arc::clone(language)));
                        }
                    }
                    language_formatters_to_check.push((
                        buffer_file.map(|f| f.worktree_id(cx)),
                        Arc::clone(language),
                        settings.clone(),
                    ));
                }
            }
        }

        let mut language_servers_to_stop = Vec::new();
        let mut language_servers_to_restart = Vec::new();
        let languages = self.languages.to_vec();

        let new_lsp_settings = ProjectSettings::get_global(cx).lsp.clone();
        let current_lsp_settings = &self.current_lsp_settings;
        for (worktree_id, started_lsp_name) in self.language_server_ids.keys() {
            let language = languages.iter().find_map(|l| {
                let adapter = self
                    .languages
                    .lsp_adapters(l)
                    .iter()
                    .find(|adapter| &adapter.name == started_lsp_name)?
                    .clone();
                Some((l, adapter))
            });
            if let Some((language, adapter)) = language {
                let worktree = self.worktree_for_id(*worktree_id, cx);
                let file = worktree.as_ref().and_then(|tree| {
                    tree.update(cx, |tree, cx| tree.root_file(cx).map(|f| f as _))
                });
                if !language_settings(Some(language), file.as_ref(), cx).enable_language_server {
                    language_servers_to_stop.push((*worktree_id, started_lsp_name.clone()));
                } else if let Some(worktree) = worktree {
                    let server_name = &adapter.name.0;
                    match (
                        current_lsp_settings.get(server_name),
                        new_lsp_settings.get(server_name),
                    ) {
                        (None, None) => {}
                        (Some(_), None) | (None, Some(_)) => {
                            language_servers_to_restart.push((worktree, Arc::clone(language)));
                        }
                        (Some(current_lsp_settings), Some(new_lsp_settings)) => {
                            if current_lsp_settings != new_lsp_settings {
                                language_servers_to_restart.push((worktree, Arc::clone(language)));
                            }
                        }
                    }
                }
            }
        }
        self.current_lsp_settings = new_lsp_settings;

        // Stop all newly-disabled language servers.
        for (worktree_id, adapter_name) in language_servers_to_stop {
            self.stop_language_server(worktree_id, adapter_name, cx)
                .detach();
        }

        let mut prettier_plugins_by_worktree = HashMap::default();
        for (worktree, language, settings) in language_formatters_to_check {
            if let Some(plugins) = prettier_support::prettier_plugins_for_language(
                &self.languages,
                &language,
                &settings,
            ) {
                prettier_plugins_by_worktree
                    .entry(worktree)
                    .or_insert_with(|| HashSet::default())
                    .extend(plugins);
            }
        }
        for (worktree, prettier_plugins) in prettier_plugins_by_worktree {
            self.install_default_prettier(worktree, prettier_plugins, cx);
        }

        // Start all the newly-enabled language servers.
        for (worktree, language) in language_servers_to_start {
            self.start_language_servers(&worktree, language, cx);
        }

        // Restart all language servers with changed initialization options.
        for (worktree, language) in language_servers_to_restart {
            self.restart_language_servers(worktree, language, cx);
        }

        if self.copilot_lsp_subscription.is_none() {
            if let Some(copilot) = Copilot::global(cx) {
                for buffer in self.opened_buffers.values() {
                    if let Some(buffer) = buffer.upgrade() {
                        self.register_buffer_with_copilot(&buffer, cx);
                    }
                }
                self.copilot_lsp_subscription = Some(subscribe_for_copilot_events(&copilot, cx));
            }
        }

        cx.notify();
    }

    pub fn buffer_for_id(&self, remote_id: BufferId) -> Option<Model<Buffer>> {
        self.opened_buffers
            .get(&remote_id)
            .and_then(|buffer| buffer.upgrade())
    }

    pub fn languages(&self) -> &Arc<LanguageRegistry> {
        &self.languages
    }

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn user_store(&self) -> Model<UserStore> {
        self.user_store.clone()
    }

    pub fn opened_buffers(&self) -> Vec<Model<Buffer>> {
        self.opened_buffers
            .values()
            .filter_map(|b| b.upgrade())
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn has_open_buffer(&self, path: impl Into<ProjectPath>, cx: &AppContext) -> bool {
        let path = path.into();
        if let Some(worktree) = self.worktree_for_id(path.worktree_id, cx) {
            self.opened_buffers.iter().any(|(_, buffer)| {
                if let Some(buffer) = buffer.upgrade() {
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
        match self.client_state {
            ProjectClientState::Local => None,
            ProjectClientState::Shared { remote_id, .. }
            | ProjectClientState::Remote { remote_id, .. } => Some(remote_id),
        }
    }

    pub fn hosted_project_id(&self) -> Option<HostedProjectId> {
        self.hosted_project_id
    }

    pub fn replica_id(&self) -> ReplicaId {
        match self.client_state {
            ProjectClientState::Remote { replica_id, .. } => replica_id,
            _ => 0,
        }
    }

    fn metadata_changed(&mut self, cx: &mut ModelContext<Self>) {
        if let ProjectClientState::Shared { updates_tx, .. } = &mut self.client_state {
            updates_tx
                .unbounded_send(LocalProjectUpdate::WorktreesChanged)
                .ok();
        }
        cx.notify();
    }

    pub fn task_inventory(&self) -> &Model<Inventory> {
        &self.tasks
    }

    pub fn collaborators(&self) -> &HashMap<proto::PeerId, Collaborator> {
        &self.collaborators
    }

    pub fn host(&self) -> Option<&Collaborator> {
        self.collaborators.values().find(|c| c.replica_id == 0)
    }

    /// Collect all worktrees, including ones that don't appear in the project panel
    pub fn worktrees(&self) -> impl '_ + DoubleEndedIterator<Item = Model<Worktree>> {
        self.worktrees
            .iter()
            .filter_map(move |worktree| worktree.upgrade())
    }

    /// Collect all user-visible worktrees, the ones that appear in the project panel
    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + DoubleEndedIterator<Item = Model<Worktree>> {
        self.worktrees.iter().filter_map(|worktree| {
            worktree.upgrade().and_then(|worktree| {
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

    pub fn worktree_for_id(&self, id: WorktreeId, cx: &AppContext) -> Option<Model<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<Model<Worktree>> {
        self.worktrees()
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
        for worktree in self.worktrees() {
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
    ) -> Task<Result<Option<Entry>>> {
        let project_path = project_path.into();
        let Some(worktree) = self.worktree_for_id(project_path.worktree_id, cx) else {
            return Task::ready(Ok(None));
        };
        if self.is_local() {
            worktree.update(cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .create_entry(project_path.path, is_directory, cx)
            })
        } else {
            let client = self.client.clone();
            let project_id = self.remote_id().unwrap();
            cx.spawn(move |_, mut cx| async move {
                let response = client
                    .request(proto::CreateProjectEntry {
                        worktree_id: project_path.worktree_id.to_proto(),
                        project_id,
                        path: project_path.path.to_string_lossy().into(),
                        is_directory,
                    })
                    .await?;
                match response.entry {
                    Some(entry) => worktree
                        .update(&mut cx, |worktree, cx| {
                            worktree.as_remote_mut().unwrap().insert_entry(
                                entry,
                                response.worktree_scan_id as usize,
                                cx,
                            )
                        })?
                        .await
                        .map(Some),
                    None => Ok(None),
                }
            })
        }
    }

    pub fn copy_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Entry>>> {
        let Some(worktree) = self.worktree_for_entry(entry_id, cx) else {
            return Task::ready(Ok(None));
        };
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

            cx.spawn(move |_, mut cx| async move {
                let response = client
                    .request(proto::CopyProjectEntry {
                        project_id,
                        entry_id: entry_id.to_proto(),
                        new_path: new_path.to_string_lossy().into(),
                    })
                    .await?;
                match response.entry {
                    Some(entry) => worktree
                        .update(&mut cx, |worktree, cx| {
                            worktree.as_remote_mut().unwrap().insert_entry(
                                entry,
                                response.worktree_scan_id as usize,
                                cx,
                            )
                        })?
                        .await
                        .map(Some),
                    None => Ok(None),
                }
            })
        }
    }

    pub fn rename_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Entry>>> {
        let Some(worktree) = self.worktree_for_entry(entry_id, cx) else {
            return Task::ready(Ok(None));
        };
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

            cx.spawn(move |_, mut cx| async move {
                let response = client
                    .request(proto::RenameProjectEntry {
                        project_id,
                        entry_id: entry_id.to_proto(),
                        new_path: new_path.to_string_lossy().into(),
                    })
                    .await?;
                match response.entry {
                    Some(entry) => worktree
                        .update(&mut cx, |worktree, cx| {
                            worktree.as_remote_mut().unwrap().insert_entry(
                                entry,
                                response.worktree_scan_id as usize,
                                cx,
                            )
                        })?
                        .await
                        .map(Some),
                    None => Ok(None),
                }
            })
        }
    }

    pub fn delete_entry(
        &mut self,
        entry_id: ProjectEntryId,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;

        cx.emit(Event::DeletedEntry(entry_id));

        if self.is_local() {
            worktree.update(cx, |worktree, cx| {
                worktree.as_local_mut().unwrap().delete_entry(entry_id, cx)
            })
        } else {
            let client = self.client.clone();
            let project_id = self.remote_id().unwrap();
            Some(cx.spawn(move |_, mut cx| async move {
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
                    })?
                    .await
            }))
        }
    }

    pub fn expand_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree_for_id(worktree_id, cx)?;
        if self.is_local() {
            worktree.update(cx, |worktree, cx| {
                worktree.as_local_mut().unwrap().expand_entry(entry_id, cx)
            })
        } else {
            let worktree = worktree.downgrade();
            let request = self.client.request(proto::ExpandProjectEntry {
                project_id: self.remote_id().unwrap(),
                entry_id: entry_id.to_proto(),
            });
            Some(cx.spawn(move |_, mut cx| async move {
                let response = request.await?;
                if let Some(worktree) = worktree.upgrade() {
                    worktree
                        .update(&mut cx, |worktree, _| {
                            worktree
                                .as_remote_mut()
                                .unwrap()
                                .wait_for_snapshot(response.worktree_scan_id as usize)
                        })?
                        .await?;
                }
                Ok(())
            }))
        }
    }

    pub fn shared(&mut self, project_id: u64, cx: &mut ModelContext<Self>) -> Result<()> {
        if !matches!(self.client_state, ProjectClientState::Local) {
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
                    if let Some(buffer) = buffer.upgrade() {
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
                    if let Some(worktree) = worktree.upgrade() {
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

        let store = cx.global::<SettingsStore>();
        for worktree in self.worktrees() {
            let worktree_id = worktree.read(cx).id().to_proto();
            for (path, content) in store.local_settings(worktree.entity_id().as_u64() as usize) {
                self.client
                    .send(proto::UpdateWorktreeSettings {
                        project_id,
                        worktree_id,
                        path: path.to_string_lossy().into(),
                        content: Some(content),
                    })
                    .log_err();
            }
        }

        let (updates_tx, mut updates_rx) = mpsc::unbounded();
        let client = self.client.clone();
        self.client_state = ProjectClientState::Shared {
            remote_id: project_id,
            updates_tx,
            _send_updates: cx.spawn(move |this, mut cx| async move {
                while let Some(update) = updates_rx.next().await {
                    match update {
                        LocalProjectUpdate::WorktreesChanged => {
                            let worktrees = this.update(&mut cx, |this, _cx| {
                                this.worktrees().collect::<Vec<_>>()
                            })?;
                            let update_project = this
                                .update(&mut cx, |this, cx| {
                                    this.client.request(proto::UpdateProject {
                                        project_id,
                                        worktrees: this.worktree_metadata_protos(cx),
                                    })
                                })?
                                .await;
                            if update_project.is_ok() {
                                for worktree in worktrees {
                                    worktree.update(&mut cx, |worktree, cx| {
                                        let worktree = worktree.as_local_mut().unwrap();
                                        worktree.share(project_id, cx).detach_and_log_err(cx)
                                    })?;
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
                            })?;

                            let Some(buffer) = buffer else { continue };
                            let operations =
                                buffer.update(&mut cx, |b, cx| b.serialize_ops(None, cx))?;
                            let operations = operations.await;
                            let state = buffer.update(&mut cx, |buffer, _| buffer.to_proto())?;

                            let initial_state = proto::CreateBufferForPeer {
                                project_id,
                                peer_id: Some(peer_id),
                                variant: Some(proto::create_buffer_for_peer::Variant::State(state)),
                            };
                            if client.send(initial_state).log_err().is_some() {
                                let client = client.clone();
                                cx.background_executor()
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
                                                            buffer_id: buffer_id.into(),
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
                Ok(())
            }),
        };

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
        cx.update_global::<SettingsStore, _>(|store, cx| {
            for worktree in &self.worktrees {
                store
                    .clear_local_settings(worktree.handle_id(), cx)
                    .log_err();
            }
        });

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

        if let ProjectClientState::Shared { remote_id, .. } = self.client_state {
            self.client_state = ProjectClientState::Local;
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
                if let Some(buffer) = open_buffer.upgrade() {
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

    pub fn set_role(&mut self, role: proto::ChannelRole, cx: &mut ModelContext<Self>) {
        let new_capability =
            if role == proto::ChannelRole::Member || role == proto::ChannelRole::Admin {
                Capability::ReadWrite
            } else {
                Capability::ReadOnly
            };
        if let ProjectClientState::Remote { capability, .. } = &mut self.client_state {
            if *capability == new_capability {
                return;
            }

            *capability = new_capability;
            for buffer in self.opened_buffers() {
                buffer.update(cx, |buffer, cx| buffer.set_capability(new_capability, cx));
            }
        }
    }

    fn disconnected_from_host_internal(&mut self, cx: &mut AppContext) {
        if let ProjectClientState::Remote {
            sharing_has_stopped,
            ..
        } = &mut self.client_state
        {
            *sharing_has_stopped = true;

            self.collaborators.clear();

            for worktree in &self.worktrees {
                if let Some(worktree) = worktree.upgrade() {
                    worktree.update(cx, |worktree, _| {
                        if let Some(worktree) = worktree.as_remote_mut() {
                            worktree.disconnected_from_host();
                        }
                    });
                }
            }

            for open_buffer in self.opened_buffers.values_mut() {
                // Wake up any tasks waiting for peers' edits to this buffer.
                if let Some(buffer) = open_buffer.upgrade() {
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

    pub fn is_disconnected(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Remote {
                sharing_has_stopped,
                ..
            } => *sharing_has_stopped,
            _ => false,
        }
    }

    pub fn capability(&self) -> Capability {
        match &self.client_state {
            ProjectClientState::Remote { capability, .. } => *capability,
            ProjectClientState::Shared { .. } | ProjectClientState::Local => Capability::ReadWrite,
        }
    }

    pub fn is_read_only(&self) -> bool {
        self.is_disconnected() || self.capability() == Capability::ReadOnly
    }

    pub fn is_local(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => true,
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
    ) -> Result<Model<Buffer>> {
        if self.is_remote() {
            return Err(anyhow!("creating buffers as a guest is not supported yet"));
        }
        let id = self.next_buffer_id.next();
        let buffer = cx.new_model(|cx| {
            Buffer::new(self.replica_id(), id, text)
                .with_language(language.unwrap_or_else(|| language::PLAIN_TEXT.clone()), cx)
        });
        self.register_buffer(&buffer, cx)?;
        Ok(buffer)
    }

    pub fn open_path(
        &mut self,
        path: ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(Option<ProjectEntryId>, AnyModel)>> {
        let task = self.open_buffer(path.clone(), cx);
        cx.spawn(move |_, cx| async move {
            let buffer = task.await?;
            let project_entry_id = buffer.read_with(&cx, |buffer, cx| {
                File::from_dyn(buffer.file()).and_then(|file| file.project_entry_id(cx))
            })?;

            let buffer: &AnyModel = &buffer;
            Ok((project_entry_id, buffer.clone()))
        })
    }

    pub fn open_local_buffer(
        &mut self,
        abs_path: impl AsRef<Path>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
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
    ) -> Task<Result<Model<Buffer>>> {
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

        let loading_watch = match self.loading_buffers_by_path.entry(project_path.clone()) {
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

                let project_path = project_path.clone();
                cx.spawn(move |this, mut cx| async move {
                    let load_result = load_buffer.await;
                    *tx.borrow_mut() = Some(this.update(&mut cx, |this, _| {
                        // Record the fact that the buffer is no longer loading.
                        this.loading_buffers_by_path.remove(&project_path);
                        let buffer = load_result.map_err(Arc::new)?;
                        Ok(buffer)
                    })?);
                    anyhow::Ok(())
                })
                .detach();
                rx
            }
        };

        cx.background_executor().spawn(async move {
            wait_for_loading_buffer(loading_watch)
                .await
                .map_err(|e| e.cloned())
        })
    }

    fn open_local_buffer_internal(
        &mut self,
        path: &Arc<Path>,
        worktree: &Model<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        let buffer_id = self.next_buffer_id.next();
        let load_buffer = worktree.update(cx, |worktree, cx| {
            let worktree = worktree.as_local_mut().unwrap();
            worktree.load_buffer(buffer_id, path, cx)
        });
        cx.spawn(move |this, mut cx| async move {
            let buffer = load_buffer.await?;
            this.update(&mut cx, |this, cx| this.register_buffer(&buffer, cx))??;
            Ok(buffer)
        })
    }

    fn open_remote_buffer_internal(
        &mut self,
        path: &Arc<Path>,
        worktree: &Model<Worktree>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        let rpc = self.client.clone();
        let project_id = self.remote_id().unwrap();
        let remote_worktree_id = worktree.read(cx).id();
        let path = path.clone();
        let path_string = path.to_string_lossy().to_string();
        cx.spawn(move |this, mut cx| async move {
            let response = rpc
                .request(proto::OpenBufferByPath {
                    project_id,
                    worktree_id: remote_worktree_id.to_proto(),
                    path: path_string,
                })
                .await?;
            let buffer_id = BufferId::new(response.buffer_id)?;
            this.update(&mut cx, |this, cx| {
                this.wait_for_remote_buffer(buffer_id, cx)
            })?
            .await
        })
    }

    /// LanguageServerName is owned, because it is inserted into a map
    pub fn open_local_buffer_via_lsp(
        &mut self,
        abs_path: lsp::Url,
        language_server_id: LanguageServerId,
        language_server_name: LanguageServerName,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        cx.spawn(move |this, mut cx| async move {
            let abs_path = abs_path
                .to_file_path()
                .map_err(|_| anyhow!("can't convert URI to path"))?;
            let (worktree, relative_path) = if let Some(result) =
                this.update(&mut cx, |this, cx| this.find_local_worktree(&abs_path, cx))?
            {
                result
            } else {
                let worktree = this
                    .update(&mut cx, |this, cx| {
                        this.create_local_worktree(&abs_path, false, cx)
                    })?
                    .await?;
                this.update(&mut cx, |this, cx| {
                    this.language_server_ids.insert(
                        (worktree.read(cx).id(), language_server_name),
                        language_server_id,
                    );
                })
                .ok();
                (worktree, PathBuf::new())
            };

            let project_path = ProjectPath {
                worktree_id: worktree.update(&mut cx, |worktree, _| worktree.id())?,
                path: relative_path.into(),
            };
            this.update(&mut cx, |this, cx| this.open_buffer(project_path, cx))?
                .await
        })
    }

    pub fn open_buffer_by_id(
        &mut self,
        id: BufferId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        if let Some(buffer) = self.buffer_for_id(id) {
            Task::ready(Ok(buffer))
        } else if self.is_local() {
            Task::ready(Err(anyhow!("buffer {} does not exist", id)))
        } else if let Some(project_id) = self.remote_id() {
            let request = self.client.request(proto::OpenBufferById {
                project_id,
                id: id.into(),
            });
            cx.spawn(move |this, mut cx| async move {
                let buffer_id = BufferId::new(request.await?.buffer_id)?;
                this.update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(buffer_id, cx)
                })?
                .await
            })
        } else {
            Task::ready(Err(anyhow!("cannot open buffer while disconnected")))
        }
    }

    pub fn save_buffers(
        &self,
        buffers: HashSet<Model<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        cx.spawn(move |this, mut cx| async move {
            let save_tasks = buffers.into_iter().filter_map(|buffer| {
                this.update(&mut cx, |this, cx| this.save_buffer(buffer, cx))
                    .ok()
            });
            try_join_all(save_tasks).await?;
            Ok(())
        })
    }

    pub fn save_buffer(
        &self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
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
        buffer: Model<Buffer>,
        abs_path: PathBuf,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let worktree_task = self.find_or_create_local_worktree(&abs_path, true, cx);
        let old_file = File::from_dyn(buffer.read(cx).file())
            .filter(|f| f.is_local())
            .cloned();
        cx.spawn(move |this, mut cx| async move {
            if let Some(old_file) = &old_file {
                this.update(&mut cx, |this, cx| {
                    this.unregister_buffer_from_language_servers(&buffer, old_file, cx);
                })?;
            }
            let (worktree, path) = worktree_task.await?;
            worktree
                .update(&mut cx, |worktree, cx| match worktree {
                    Worktree::Local(worktree) => {
                        worktree.save_buffer(buffer.clone(), path.into(), true, cx)
                    }
                    Worktree::Remote(_) => panic!("cannot remote buffers as new files"),
                })?
                .await?;

            this.update(&mut cx, |this, cx| {
                this.detect_language_for_buffer(&buffer, cx);
                this.register_buffer_with_language_servers(&buffer, cx);
            })?;
            Ok(())
        })
    }

    pub fn get_open_buffer(
        &mut self,
        path: &ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Option<Model<Buffer>> {
        let worktree = self.worktree_for_id(path.worktree_id, cx)?;
        self.opened_buffers.values().find_map(|buffer| {
            let buffer = buffer.upgrade()?;
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
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.request_buffer_diff_recalculation(buffer, cx);
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
                } else if entry.get().upgrade().is_some() {
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

        if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
            if file.is_local {
                self.local_buffer_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path.clone(),
                    },
                    remote_id,
                );

                if let Some(entry_id) = file.entry_id {
                    self.local_buffer_ids_by_entry_id
                        .insert(entry_id, remote_id);
                }
            }
        }

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
        buffer_handle: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id();

        if let Some(file) = File::from_dyn(buffer.file()) {
            if !file.is_local() {
                return;
            }

            let abs_path = file.abs_path(cx);
            let uri = lsp::Url::from_file_path(&abs_path)
                .unwrap_or_else(|()| panic!("Failed to register file {abs_path:?}"));
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
                for adapter in self.languages.lsp_adapters(&language) {
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
        buffer: &Model<Buffer>,
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

            if let Some(language) = buffer.language().cloned() {
                for adapter in self.languages.lsp_adapters(&language) {
                    if let Some(server_id) = ids.get(&(worktree_id, adapter.name.clone())) {
                        buffer.update_diagnostics(*server_id, Default::default(), cx);
                    }
                }
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
        buffer_handle: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(copilot) = Copilot::global(cx) {
            copilot.update(cx, |copilot, cx| copilot.register_buffer(buffer_handle, cx));
        }
    }

    async fn send_buffer_ordered_messages(
        this: WeakModel<Self>,
        rx: UnboundedReceiver<BufferOrderedMessage>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        const MAX_BATCH_SIZE: usize = 128;

        let mut operations_by_buffer_id = HashMap::default();
        async fn flush_operations(
            this: &WeakModel<Project>,
            operations_by_buffer_id: &mut HashMap<BufferId, Vec<proto::Operation>>,
            needs_resync_with_host: &mut bool,
            is_local: bool,
            cx: &mut AsyncAppContext,
        ) -> Result<()> {
            for (buffer_id, operations) in operations_by_buffer_id.drain() {
                let request = this.update(cx, |this, _| {
                    let project_id = this.remote_id()?;
                    Some(this.client.request(proto::UpdateBuffer {
                        buffer_id: buffer_id.into(),
                        project_id,
                        operations,
                    }))
                })?;
                if let Some(request) = request {
                    if request.await.is_err() && !is_local {
                        *needs_resync_with_host = true;
                        break;
                    }
                }
            }
            Ok(())
        }

        let mut needs_resync_with_host = false;
        let mut changes = rx.ready_chunks(MAX_BATCH_SIZE);

        while let Some(changes) = changes.next().await {
            let is_local = this.update(&mut cx, |this, _| this.is_local())?;

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
                            .update(&mut cx, |this, cx| this.synchronize_remote_buffers(cx))?
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
                            &mut cx,
                        )
                        .await?;

                        this.update(&mut cx, |this, _| {
                            if let Some(project_id) = this.remote_id() {
                                this.client
                                    .send(proto::UpdateLanguageServer {
                                        project_id,
                                        language_server_id: language_server_id.0 as u64,
                                        variant: Some(message),
                                    })
                                    .log_err();
                            }
                        })?;
                    }
                }
            }

            flush_operations(
                &this,
                &mut operations_by_buffer_id,
                &mut needs_resync_with_host,
                is_local,
                &mut cx,
            )
            .await?;
        }

        Ok(())
    }

    fn on_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &BufferEvent,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        if matches!(
            event,
            BufferEvent::Edited { .. } | BufferEvent::Reloaded | BufferEvent::DiffBaseChanged
        ) {
            self.request_buffer_diff_recalculation(&buffer, cx);
        }

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
                    .language_servers_for_buffer(buffer, cx)
                    .map(|i| i.1.clone())
                    .collect();

                for language_server in language_servers {
                    let language_server = language_server.clone();

                    let buffer_snapshots = self
                        .buffer_snapshots
                        .get_mut(&buffer.remote_id())
                        .and_then(|m| m.get_mut(&language_server.server_id()))?;
                    let previous_snapshot = buffer_snapshots.last()?;

                    let build_incremental_change = || {
                        buffer
                            .edits_since::<(PointUtf16, usize)>(
                                previous_snapshot.snapshot.version(),
                            )
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
                            .collect()
                    };

                    let document_sync_kind = language_server
                        .capabilities()
                        .text_document_sync
                        .as_ref()
                        .and_then(|sync| match sync {
                            lsp::TextDocumentSyncCapability::Kind(kind) => Some(*kind),
                            lsp::TextDocumentSyncCapability::Options(options) => options.change,
                        });

                    let content_changes: Vec<_> = match document_sync_kind {
                        Some(lsp::TextDocumentSyncKind::FULL) => {
                            vec![lsp::TextDocumentContentChangeEvent {
                                range: None,
                                range_length: None,
                                text: next_snapshot.text(),
                            }]
                        }
                        Some(lsp::TextDocumentSyncKind::INCREMENTAL) => build_incremental_change(),
                        _ => {
                            #[cfg(any(test, feature = "test-support"))]
                            {
                                build_incremental_change()
                            }

                            #[cfg(not(any(test, feature = "test-support")))]
                            {
                                continue;
                            }
                        }
                    };

                    let next_version = previous_snapshot.version + 1;

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
                    let text = include_text(server.as_ref()).then(|| buffer.read(cx).text());

                    server
                        .notify::<lsp::notification::DidSaveTextDocument>(
                            lsp::DidSaveTextDocumentParams {
                                text_document: text_document.clone(),
                                text,
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

                            let task = cx.spawn(move |this, mut cx| async move {
                                cx.background_executor().timer(DISK_BASED_DIAGNOSTICS_DEBOUNCE).await;
                                if let Some(this) = this.upgrade() {
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
                                    }).ok();
                                }
                            });
                            *simulate_disk_based_diagnostics_completion = Some(task);
                        }
                    }
                }
            }
            BufferEvent::FileHandleChanged => {
                let Some(file) = File::from_dyn(buffer.read(cx).file()) else {
                    return None;
                };

                let remote_id = buffer.read(cx).remote_id();
                if let Some(entry_id) = file.entry_id {
                    match self.local_buffer_ids_by_entry_id.get(&entry_id) {
                        Some(_) => {
                            return None;
                        }
                        None => {
                            self.local_buffer_ids_by_entry_id
                                .insert(entry_id, remote_id);
                        }
                    }
                };
                self.local_buffer_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path.clone(),
                    },
                    remote_id,
                );
            }
            _ => {}
        }

        None
    }

    fn request_buffer_diff_recalculation(
        &mut self,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        self.buffers_needing_diff.insert(buffer.downgrade());
        let first_insertion = self.buffers_needing_diff.len() == 1;

        let settings = ProjectSettings::get_global(cx);
        let delay = if let Some(delay) = settings.git.gutter_debounce {
            delay
        } else {
            if first_insertion {
                let this = cx.weak_model();
                cx.defer(move |cx| {
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            this.recalculate_buffer_diffs(cx).detach();
                        });
                    }
                });
            }
            return;
        };

        const MIN_DELAY: u64 = 50;
        let delay = delay.max(MIN_DELAY);
        let duration = Duration::from_millis(delay);

        self.git_diff_debouncer
            .fire_new(duration, cx, move |this, cx| {
                this.recalculate_buffer_diffs(cx)
            });
    }

    fn recalculate_buffer_diffs(&mut self, cx: &mut ModelContext<Self>) -> Task<()> {
        let buffers = self.buffers_needing_diff.drain().collect::<Vec<_>>();
        cx.spawn(move |this, mut cx| async move {
            let tasks: Vec<_> = buffers
                .iter()
                .filter_map(|buffer| {
                    let buffer = buffer.upgrade()?;
                    buffer
                        .update(&mut cx, |buffer, cx| buffer.git_diff_recalc(cx))
                        .ok()
                        .flatten()
                })
                .collect();

            futures::future::join_all(tasks).await;

            this.update(&mut cx, |this, cx| {
                if !this.buffers_needing_diff.is_empty() {
                    this.recalculate_buffer_diffs(cx).detach();
                } else {
                    // TODO: Would a `ModelContext<Project>.notify()` suffice here?
                    for buffer in buffers {
                        if let Some(buffer) = buffer.upgrade() {
                            buffer.update(cx, |_, cx| cx.notify());
                        }
                    }
                }
            })
            .ok();
        })
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
        languages: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Project>,
    ) -> Task<()> {
        let mut subscription = languages.subscribe();
        let mut prev_reload_count = languages.reload_count();
        cx.spawn(move |project, mut cx| async move {
            while let Some(()) = subscription.next().await {
                if let Some(project) = project.upgrade() {
                    // If the language registry has been reloaded, then remove and
                    // re-assign the languages on all open buffers.
                    let reload_count = languages.reload_count();
                    if reload_count > prev_reload_count {
                        prev_reload_count = reload_count;
                        project
                            .update(&mut cx, |this, cx| {
                                let buffers = this
                                    .opened_buffers
                                    .values()
                                    .filter_map(|b| b.upgrade())
                                    .collect::<Vec<_>>();
                                for buffer in buffers {
                                    if let Some(f) = File::from_dyn(buffer.read(cx).file()).cloned()
                                    {
                                        this.unregister_buffer_from_language_servers(
                                            &buffer, &f, cx,
                                        );
                                        buffer
                                            .update(cx, |buffer, cx| buffer.set_language(None, cx));
                                    }
                                }
                            })
                            .ok();
                    }

                    project
                        .update(&mut cx, |project, cx| {
                            let mut plain_text_buffers = Vec::new();
                            let mut buffers_with_unknown_injections = Vec::new();
                            for buffer in project.opened_buffers.values() {
                                if let Some(handle) = buffer.upgrade() {
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
                        })
                        .ok();
                }
            }
        })
    }

    fn maintain_workspace_config(cx: &mut ModelContext<Project>) -> Task<Result<()>> {
        let (mut settings_changed_tx, mut settings_changed_rx) = watch::channel();
        let _ = postage::stream::Stream::try_recv(&mut settings_changed_rx);

        let settings_observation = cx.observe_global::<SettingsStore>(move |_, _| {
            *settings_changed_tx.borrow_mut() = ();
        });

        cx.spawn(move |this, mut cx| async move {
            while let Some(()) = settings_changed_rx.next().await {
                let servers: Vec<_> = this.update(&mut cx, |this, _| {
                    this.language_servers
                        .values()
                        .filter_map(|state| match state {
                            LanguageServerState::Starting(_) => None,
                            LanguageServerState::Running {
                                adapter, server, ..
                            } => Some((adapter.clone(), server.clone())),
                        })
                        .collect()
                })?;

                for (adapter, server) in servers {
                    let settings =
                        cx.update(|cx| adapter.workspace_configuration(server.root_path(), cx))?;

                    server
                        .notify::<lsp::notification::DidChangeConfiguration>(
                            lsp::DidChangeConfigurationParams { settings },
                        )
                        .ok();
                }
            }

            drop(settings_observation);
            anyhow::Ok(())
        })
    }

    fn detect_language_for_buffer(
        &mut self,
        buffer_handle: &Model<Buffer>,
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
        buffer: &Model<Buffer>,
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

        let buffer_file = buffer.read(cx).file().cloned();
        let settings = language_settings(Some(&new_language), buffer_file.as_ref(), cx).clone();
        let buffer_file = File::from_dyn(buffer_file.as_ref());
        let worktree = buffer_file.as_ref().map(|f| f.worktree_id(cx));
        if let Some(prettier_plugins) = prettier_support::prettier_plugins_for_language(
            &self.languages,
            &new_language,
            &settings,
        ) {
            self.install_default_prettier(worktree, prettier_plugins, cx);
        };
        if let Some(file) = buffer_file {
            let worktree = file.worktree.clone();
            if worktree.read(cx).is_local() {
                self.start_language_servers(&worktree, new_language, cx);
            }
        }
    }

    fn start_language_servers(
        &mut self,
        worktree: &Model<Worktree>,
        language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        let root_file = worktree.update(cx, |tree, cx| tree.root_file(cx));
        let settings = language_settings(Some(&language), root_file.map(|f| f as _).as_ref(), cx);
        if !settings.enable_language_server {
            return;
        }

        for adapter in self.languages.clone().lsp_adapters(&language) {
            self.start_language_server(worktree, adapter.clone(), language.clone(), cx);
        }
    }

    fn start_language_server(
        &mut self,
        worktree_handle: &Model<Worktree>,
        adapter: Arc<CachedLspAdapter>,
        language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        if adapter.reinstall_attempt_count.load(SeqCst) > MAX_SERVER_REINSTALL_ATTEMPT_COUNT {
            return;
        }

        let worktree = worktree_handle.read(cx);
        let worktree_id = worktree.id();
        let worktree_path = worktree.abs_path();
        let key = (worktree_id, adapter.name.clone());
        if self.language_server_ids.contains_key(&key) {
            return;
        }

        let stderr_capture = Arc::new(Mutex::new(Some(String::new())));
        let pending_server = match self.languages.create_pending_language_server(
            stderr_capture.clone(),
            language.clone(),
            adapter.clone(),
            Arc::clone(&worktree_path),
            ProjectLspAdapterDelegate::new(self, worktree_handle, cx),
            cx,
        ) {
            Some(pending_server) => pending_server,
            None => return,
        };

        let project_settings =
            ProjectSettings::get(Some((worktree_id.to_proto() as usize, Path::new(""))), cx);
        let lsp = project_settings.lsp.get(&adapter.name.0);
        let override_options = lsp.and_then(|s| s.initialization_options.clone());

        let server_id = pending_server.server_id;
        let container_dir = pending_server.container_dir.clone();
        let state = LanguageServerState::Starting({
            let adapter = adapter.clone();
            let server_name = adapter.name.0.clone();
            let language = language.clone();
            let key = key.clone();

            cx.spawn(move |this, mut cx| async move {
                let result = Self::setup_and_insert_language_server(
                    this.clone(),
                    &worktree_path,
                    override_options,
                    pending_server,
                    adapter.clone(),
                    language.clone(),
                    server_id,
                    key,
                    &mut cx,
                )
                .await;

                match result {
                    Ok(server) => {
                        stderr_capture.lock().take();
                        server
                    }

                    Err(err) => {
                        log::error!("failed to start language server {server_name:?}: {err}");
                        log::error!("server stderr: {:?}", stderr_capture.lock().take());

                        let this = this.upgrade()?;
                        let container_dir = container_dir?;

                        let attempt_count = adapter.reinstall_attempt_count.fetch_add(1, SeqCst);
                        if attempt_count >= MAX_SERVER_REINSTALL_ATTEMPT_COUNT {
                            let max = MAX_SERVER_REINSTALL_ATTEMPT_COUNT;
                            log::error!("Hit {max} reinstallation attempts for {server_name:?}");
                            return None;
                        }

                        log::info!(
                            "retrying installation of language server {server_name:?} in {}s",
                            SERVER_REINSTALL_DEBOUNCE_TIMEOUT.as_secs()
                        );
                        cx.background_executor()
                            .timer(SERVER_REINSTALL_DEBOUNCE_TIMEOUT)
                            .await;

                        let installation_test_binary = adapter
                            .installation_test_binary(container_dir.to_path_buf())
                            .await;

                        this.update(&mut cx, |_, cx| {
                            Self::check_errored_server(
                                language,
                                adapter,
                                server_id,
                                installation_test_binary,
                                cx,
                            )
                        })
                        .ok();

                        None
                    }
                }
            })
        });

        self.language_servers.insert(server_id, state);
        self.language_server_ids.insert(key, server_id);
    }

    fn reinstall_language_server(
        &mut self,
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<()>> {
        log::info!("beginning to reinstall server");

        let existing_server = match self.language_servers.remove(&server_id) {
            Some(LanguageServerState::Running { server, .. }) => Some(server),
            _ => None,
        };

        for worktree in &self.worktrees {
            if let Some(worktree) = worktree.upgrade() {
                let key = (worktree.read(cx).id(), adapter.name.clone());
                self.language_server_ids.remove(&key);
            }
        }

        Some(cx.spawn(move |this, mut cx| async move {
            if let Some(task) = existing_server.and_then(|server| server.shutdown()) {
                log::info!("shutting down existing server");
                task.await;
            }

            // TODO: This is race-safe with regards to preventing new instances from
            // starting while deleting, but existing instances in other projects are going
            // to be very confused and messed up
            let Some(task) = this
                .update(&mut cx, |this, cx| {
                    this.languages.delete_server_container(adapter.clone(), cx)
                })
                .log_err()
            else {
                return;
            };
            task.await;

            this.update(&mut cx, |this, cx| {
                let worktrees = this.worktrees.clone();
                for worktree in worktrees {
                    if let Some(worktree) = worktree.upgrade() {
                        this.start_language_server(
                            &worktree,
                            adapter.clone(),
                            language.clone(),
                            cx,
                        );
                    }
                }
            })
            .ok();
        }))
    }

    #[allow(clippy::too_many_arguments)]
    async fn setup_and_insert_language_server(
        this: WeakModel<Self>,
        worktree_path: &Path,
        override_initialization_options: Option<serde_json::Value>,
        pending_server: PendingLanguageServer,
        adapter: Arc<CachedLspAdapter>,
        language: Arc<Language>,
        server_id: LanguageServerId,
        key: (WorktreeId, LanguageServerName),
        cx: &mut AsyncAppContext,
    ) -> Result<Option<Arc<LanguageServer>>> {
        let language_server = Self::setup_pending_language_server(
            this.clone(),
            override_initialization_options,
            pending_server,
            worktree_path,
            adapter.clone(),
            server_id,
            cx,
        )
        .await?;

        let this = match this.upgrade() {
            Some(this) => this,
            None => return Err(anyhow!("failed to upgrade project handle")),
        };

        this.update(cx, |this, cx| {
            this.insert_newly_running_language_server(
                language,
                adapter,
                language_server.clone(),
                server_id,
                key,
                cx,
            )
        })??;

        Ok(Some(language_server))
    }

    async fn setup_pending_language_server(
        this: WeakModel<Self>,
        override_options: Option<serde_json::Value>,
        pending_server: PendingLanguageServer,
        worktree_path: &Path,
        adapter: Arc<CachedLspAdapter>,
        server_id: LanguageServerId,
        cx: &mut AsyncAppContext,
    ) -> Result<Arc<LanguageServer>> {
        let workspace_config =
            cx.update(|cx| adapter.workspace_configuration(worktree_path, cx))?;
        let language_server = pending_server.task.await?;

        let name = language_server.name();
        language_server
            .on_notification::<lsp::notification::PublishDiagnostics, _>({
                let adapter = adapter.clone();
                let this = this.clone();
                move |mut params, mut cx| {
                    let adapter = adapter.clone();
                    if let Some(this) = this.upgrade() {
                        adapter.process_diagnostics(&mut params);
                        this.update(&mut cx, |this, cx| {
                            this.update_diagnostics(
                                server_id,
                                params,
                                &adapter.disk_based_diagnostic_sources,
                                cx,
                            )
                            .log_err();
                        })
                        .ok();
                    }
                }
            })
            .detach();

        language_server
            .on_request::<lsp::request::WorkspaceConfiguration, _, _>({
                let adapter = adapter.clone();
                let worktree_path = worktree_path.to_path_buf();
                move |params, cx| {
                    let adapter = adapter.clone();
                    let worktree_path = worktree_path.clone();
                    async move {
                        let workspace_config =
                            cx.update(|cx| adapter.workspace_configuration(&worktree_path, cx))?;
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
                let this = this.clone();
                move |params, mut cx| {
                    let this = this.clone();
                    async move {
                        this.update(&mut cx, |this, _| {
                            if let Some(status) = this.language_server_statuses.get_mut(&server_id)
                            {
                                if let lsp::NumberOrString::String(token) = params.token {
                                    status.progress_tokens.insert(token);
                                }
                            }
                        })?;

                        Ok(())
                    }
                }
            })
            .detach();

        language_server
            .on_request::<lsp::request::RegisterCapability, _, _>({
                let this = this.clone();
                move |params, mut cx| {
                    let this = this.clone();
                    async move {
                        for reg in params.registrations {
                            if reg.method == "workspace/didChangeWatchedFiles" {
                                if let Some(options) = reg.register_options {
                                    let options = serde_json::from_value(options)?;
                                    this.update(&mut cx, |this, cx| {
                                        this.on_lsp_did_change_watched_files(
                                            server_id, options, cx,
                                        );
                                    })?;
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
            .detach();

        language_server
            .on_request::<lsp::request::ApplyWorkspaceEdit, _, _>({
                let adapter = adapter.clone();
                let this = this.clone();
                move |params, cx| {
                    Self::on_lsp_workspace_edit(
                        this.clone(),
                        params,
                        server_id,
                        adapter.clone(),
                        cx,
                    )
                }
            })
            .detach();

        language_server
            .on_request::<lsp::request::InlayHintRefreshRequest, _, _>({
                let this = this.clone();
                move |(), mut cx| {
                    let this = this.clone();
                    async move {
                        this.update(&mut cx, |project, cx| {
                            cx.emit(Event::RefreshInlayHints);
                            project.remote_id().map(|project_id| {
                                project.client.send(proto::RefreshInlayHints { project_id })
                            })
                        })?
                        .transpose()?;
                        Ok(())
                    }
                }
            })
            .detach();

        language_server
            .on_request::<lsp::request::ShowMessageRequest, _, _>({
                let this = this.clone();
                let name = name.to_string();
                move |params, mut cx| {
                    let this = this.clone();
                    let name = name.to_string();
                    async move {
                        if let Some(actions) = params.actions {
                            let (tx, mut rx) = smol::channel::bounded(1);
                            let request = LanguageServerPromptRequest {
                                level: match params.typ {
                                    lsp::MessageType::ERROR => PromptLevel::Critical,
                                    lsp::MessageType::WARNING => PromptLevel::Warning,
                                    _ => PromptLevel::Info,
                                },
                                message: params.message,
                                actions,
                                response_channel: tx,
                                lsp_name: name.clone(),
                            };

                            if let Ok(_) = this.update(&mut cx, |_, cx| {
                                cx.emit(Event::LanguageServerPrompt(request));
                            }) {
                                let response = rx.next().await;

                                Ok(response)
                            } else {
                                Ok(None)
                            }
                        } else {
                            Ok(None)
                        }
                    }
                }
            })
            .detach();

        let disk_based_diagnostics_progress_token =
            adapter.disk_based_diagnostics_progress_token.clone();

        language_server
            .on_notification::<ServerStatus, _>({
                let this = this.clone();
                let name = name.to_string();
                move |params, mut cx| {
                    let this = this.clone();
                    let name = name.to_string();
                    if let Some(ref message) = params.message {
                        let message = message.trim();
                        if !message.is_empty() {
                            let formatted_message = format!(
                                "Language server {name} (id {server_id}) status update: {message}"
                            );
                            match params.health {
                                ServerHealthStatus::Ok => log::info!("{}", formatted_message),
                                ServerHealthStatus::Warning => log::warn!("{}", formatted_message),
                                ServerHealthStatus::Error => {
                                    log::error!("{}", formatted_message);
                                    let (tx, _rx) = smol::channel::bounded(1);
                                    let request = LanguageServerPromptRequest {
                                        level: PromptLevel::Critical,
                                        message: params.message.unwrap_or_default(),
                                        actions: Vec::new(),
                                        response_channel: tx,
                                        lsp_name: name.clone(),
                                    };
                                    let _ = this
                                        .update(&mut cx, |_, cx| {
                                            cx.emit(Event::LanguageServerPrompt(request));
                                        })
                                        .ok();
                                }
                                ServerHealthStatus::Other(status) => {
                                    log::info!(
                                        "Unknown server health: {status}\n{formatted_message}"
                                    )
                                }
                            }
                        }
                    }
                }
            })
            .detach();

        language_server
            .on_notification::<lsp::notification::Progress, _>(move |params, mut cx| {
                if let Some(this) = this.upgrade() {
                    this.update(&mut cx, |this, cx| {
                        this.on_lsp_progress(
                            params,
                            server_id,
                            disk_based_diagnostics_progress_token.clone(),
                            cx,
                        );
                    })
                    .ok();
                }
            })
            .detach();

        let mut initialization_options = adapter.adapter.initialization_options();
        match (&mut initialization_options, override_options) {
            (Some(initialization_options), Some(override_options)) => {
                merge_json_value_into(override_options, initialization_options);
            }
            (None, override_options) => initialization_options = override_options,
            _ => {}
        }
        let language_server = cx
            .update(|cx| language_server.initialize(initialization_options, cx))?
            .await?;

        language_server
            .notify::<lsp::notification::DidChangeConfiguration>(
                lsp::DidChangeConfigurationParams {
                    settings: workspace_config,
                },
            )
            .ok();

        Ok(language_server)
    }

    fn insert_newly_running_language_server(
        &mut self,
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        language_server: Arc<LanguageServer>,
        server_id: LanguageServerId,
        key: (WorktreeId, LanguageServerName),
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        // If the language server for this key doesn't match the server id, don't store the
        // server. Which will cause it to be dropped, killing the process
        if self
            .language_server_ids
            .get(&key)
            .map(|id| id != &server_id)
            .unwrap_or(false)
        {
            return Ok(());
        }

        // Update language_servers collection with Running variant of LanguageServerState
        // indicating that the server is up and running and ready
        self.language_servers.insert(
            server_id,
            LanguageServerState::Running {
                adapter: adapter.clone(),
                language: language.clone(),
                watched_paths: Default::default(),
                server: language_server.clone(),
                simulate_disk_based_diagnostics_completion: None,
            },
        );

        self.language_server_statuses.insert(
            server_id,
            LanguageServerStatus {
                name: language_server.name().to_string(),
                pending_work: Default::default(),
                has_pending_diagnostic_updates: false,
                progress_tokens: Default::default(),
            },
        );

        cx.emit(Event::LanguageServerAdded(server_id));

        if let Some(project_id) = self.remote_id() {
            self.client.send(proto::StartLanguageServer {
                project_id,
                server: Some(proto::LanguageServer {
                    id: server_id.0 as u64,
                    name: language_server.name().to_string(),
                }),
            })?;
        }

        // Tell the language server about every open buffer in the worktree that matches the language.
        for buffer in self.opened_buffers.values() {
            if let Some(buffer_handle) = buffer.upgrade() {
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
                    || !self
                        .languages
                        .lsp_adapters(&language)
                        .iter()
                        .any(|a| a.name == key.1)
                {
                    continue;
                }

                let file = match file.as_local() {
                    Some(file) => file,
                    None => continue,
                };

                let versions = self
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
                language_server.notify::<lsp::notification::DidOpenTextDocument>(
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
                )?;

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
        Ok(())
    }

    // Returns a list of all of the worktrees which no longer have a language server and the root path
    // for the stopped server
    fn stop_language_server(
        &mut self,
        worktree_id: WorktreeId,
        adapter_name: LanguageServerName,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<WorktreeId>> {
        let key = (worktree_id, adapter_name);
        if let Some(server_id) = self.language_server_ids.remove(&key) {
            let name = key.1 .0;
            log::info!("stopping language server {name}");

            // Remove other entries for this language server as well
            let mut orphaned_worktrees = vec![worktree_id];
            let other_keys = self.language_server_ids.keys().cloned().collect::<Vec<_>>();
            for other_key in other_keys {
                if self.language_server_ids.get(&other_key) == Some(&server_id) {
                    self.language_server_ids.remove(&other_key);
                    orphaned_worktrees.push(other_key.0);
                }
            }

            for buffer in self.opened_buffers.values() {
                if let Some(buffer) = buffer.upgrade() {
                    buffer.update(cx, |buffer, cx| {
                        buffer.update_diagnostics(server_id, Default::default(), cx);
                    });
                }
            }
            for worktree in &self.worktrees {
                if let Some(worktree) = worktree.upgrade() {
                    worktree.update(cx, |worktree, cx| {
                        if let Some(worktree) = worktree.as_local_mut() {
                            worktree.clear_diagnostics_for_language_server(server_id, cx);
                        }
                    });
                }
            }

            self.language_server_statuses.remove(&server_id);
            cx.notify();

            let server_state = self.language_servers.remove(&server_id);
            cx.emit(Event::LanguageServerRemoved(server_id));
            cx.spawn(move |this, cx| async move {
                Self::shutdown_language_server(this, server_state, name, server_id, cx).await;
                orphaned_worktrees
            })
        } else {
            Task::ready(Vec::new())
        }
    }

    async fn shutdown_language_server(
        this: WeakModel<Project>,
        server_state: Option<LanguageServerState>,
        name: Arc<str>,
        server_id: LanguageServerId,
        mut cx: AsyncAppContext,
    ) {
        let server = match server_state {
            Some(LanguageServerState::Starting(task)) => {
                let mut timer = cx
                    .background_executor()
                    .timer(SERVER_LAUNCHING_BEFORE_SHUTDOWN_TIMEOUT)
                    .fuse();

                select! {
                    server = task.fuse() => server,
                    _ = timer => {
                        log::info!(
                            "timeout waiting for language server {} to finish launching before stopping",
                            name
                        );
                        None
                    },
                }
            }

            Some(LanguageServerState::Running { server, .. }) => Some(server),

            None => None,
        };

        if let Some(server) = server {
            if let Some(shutdown) = server.shutdown() {
                shutdown.await;
            }
        }

        if let Some(this) = this.upgrade() {
            this.update(&mut cx, |this, cx| {
                this.language_server_statuses.remove(&server_id);
                cx.notify();
            })
            .ok();
        }
    }

    pub fn restart_language_servers_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Model<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let language_server_lookup_info: HashSet<(Model<Worktree>, Arc<Language>)> = buffers
            .into_iter()
            .filter_map(|buffer| {
                let buffer = buffer.read(cx);
                let file = File::from_dyn(buffer.file())?;
                let full_path = file.full_path(cx);
                let language = self
                    .languages
                    .language_for_file(&full_path, Some(buffer.as_rope()))
                    .now_or_never()?
                    .ok()?;
                Some((file.worktree.clone(), language))
            })
            .collect();
        for (worktree, language) in language_server_lookup_info {
            self.restart_language_servers(worktree, language, cx);
        }

        None
    }

    fn restart_language_servers(
        &mut self,
        worktree: Model<Worktree>,
        language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        let worktree_id = worktree.read(cx).id();

        let stop_tasks = self
            .languages
            .clone()
            .lsp_adapters(&language)
            .iter()
            .map(|adapter| {
                let stop_task = self.stop_language_server(worktree_id, adapter.name.clone(), cx);
                (stop_task, adapter.name.clone())
            })
            .collect::<Vec<_>>();
        if stop_tasks.is_empty() {
            return;
        }

        cx.spawn(move |this, mut cx| async move {
            // For each stopped language server, record all of the worktrees with which
            // it was associated.
            let mut affected_worktrees = Vec::new();
            for (stop_task, language_server_name) in stop_tasks {
                for affected_worktree_id in stop_task.await {
                    affected_worktrees.push((affected_worktree_id, language_server_name.clone()));
                }
            }

            this.update(&mut cx, |this, cx| {
                // Restart the language server for the given worktree.
                this.start_language_servers(&worktree, language.clone(), cx);

                // Lookup new server ids and set them for each of the orphaned worktrees
                for (affected_worktree_id, language_server_name) in affected_worktrees {
                    if let Some(new_server_id) = this
                        .language_server_ids
                        .get(&(worktree_id, language_server_name.clone()))
                        .cloned()
                    {
                        this.language_server_ids
                            .insert((affected_worktree_id, language_server_name), new_server_id);
                    }
                }
            })
            .ok();
        })
        .detach();
    }

    fn check_errored_server(
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        server_id: LanguageServerId,
        installation_test_binary: Option<LanguageServerBinary>,
        cx: &mut ModelContext<Self>,
    ) {
        if !adapter.can_be_reinstalled() {
            log::info!(
                "Validation check requested for {:?} but it cannot be reinstalled",
                adapter.name.0
            );
            return;
        }

        cx.spawn(move |this, mut cx| async move {
            log::info!("About to spawn test binary");

            // A lack of test binary counts as a failure
            let process = installation_test_binary.and_then(|binary| {
                smol::process::Command::new(&binary.path)
                    .current_dir(&binary.path)
                    .args(binary.arguments)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .kill_on_drop(true)
                    .spawn()
                    .ok()
            });

            const PROCESS_TIMEOUT: Duration = Duration::from_secs(5);
            let mut timeout = cx.background_executor().timer(PROCESS_TIMEOUT).fuse();

            let mut errored = false;
            if let Some(mut process) = process {
                futures::select! {
                    status = process.status().fuse() => match status {
                        Ok(status) => errored = !status.success(),
                        Err(_) => errored = true,
                    },

                    _ = timeout => {
                        log::info!("test binary time-ed out, this counts as a success");
                        _ = process.kill();
                    }
                }
            } else {
                log::warn!("test binary failed to launch");
                errored = true;
            }

            if errored {
                log::warn!("test binary check failed");
                let task = this
                    .update(&mut cx, move |this, cx| {
                        this.reinstall_language_server(language, adapter, server_id, cx)
                    })
                    .ok()
                    .flatten();

                if let Some(task) = task {
                    task.await;
                }
            }
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
                                    percentage: report.percentage,
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
                                    percentage: report.percentage,
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
            cx.emit(Event::RefreshInlayHints);
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
            let mut builders = HashMap::default();
            for watcher in params.watchers {
                for worktree in &self.worktrees {
                    if let Some(worktree) = worktree.upgrade() {
                        let glob_is_inside_worktree = worktree.update(cx, |tree, _| {
                            if let Some(abs_path) = tree.abs_path().to_str() {
                                let relative_glob_pattern = match &watcher.glob_pattern {
                                    lsp::GlobPattern::String(s) => s
                                        .strip_prefix(abs_path)
                                        .and_then(|s| s.strip_prefix(std::path::MAIN_SEPARATOR)),
                                    lsp::GlobPattern::Relative(rp) => {
                                        let base_uri = match &rp.base_uri {
                                            lsp::OneOf::Left(workspace_folder) => {
                                                &workspace_folder.uri
                                            }
                                            lsp::OneOf::Right(base_uri) => base_uri,
                                        };
                                        base_uri.to_file_path().ok().and_then(|file_path| {
                                            (file_path.to_str() == Some(abs_path))
                                                .then_some(rp.pattern.as_str())
                                        })
                                    }
                                };
                                if let Some(relative_glob_pattern) = relative_glob_pattern {
                                    let literal_prefix = glob_literal_prefix(relative_glob_pattern);
                                    tree.as_local_mut()
                                        .unwrap()
                                        .add_path_prefix_to_scan(Path::new(literal_prefix).into());
                                    if let Some(glob) = Glob::new(relative_glob_pattern).log_err() {
                                        builders
                                            .entry(tree.id())
                                            .or_insert_with(|| GlobSetBuilder::new())
                                            .add(glob);
                                    }
                                    return true;
                                }
                            }
                            false
                        });
                        if glob_is_inside_worktree {
                            break;
                        }
                    }
                }
            }

            watched_paths.clear();
            for (worktree_id, builder) in builders {
                if let Ok(globset) = builder.build() {
                    watched_paths.insert(worktree_id, globset);
                }
            }

            cx.notify();
        }
    }

    async fn on_lsp_workspace_edit(
        this: WeakModel<Self>,
        params: lsp::ApplyWorkspaceEditParams,
        server_id: LanguageServerId,
        adapter: Arc<CachedLspAdapter>,
        mut cx: AsyncAppContext,
    ) -> Result<lsp::ApplyWorkspaceEditResponse> {
        let this = this
            .upgrade()
            .ok_or_else(|| anyhow!("project project closed"))?;
        let language_server = this
            .update(&mut cx, |this, _| this.language_server_for_id(server_id))?
            .ok_or_else(|| anyhow!("language server not found"))?;
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
        })?;
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
                        source: diagnostic.source.clone(),
                        code: code.clone(),
                        severity: diagnostic.severity.unwrap_or(DiagnosticSeverity::ERROR),
                        message: diagnostic.message.trim().to_string(),
                        group_id,
                        is_primary: true,
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
                                    source: diagnostic.source.clone(),
                                    code: code.clone(),
                                    severity: DiagnosticSeverity::INFORMATION,
                                    message: info.message.trim().to_string(),
                                    group_id,
                                    is_primary: false,
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
            .ok_or_else(|| anyhow!("no worktree found for diagnostics path {abs_path:?}"))?;

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
        buffer: &Model<Buffer>,
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
                    range.start = snapshot.clip_point_utf16(Unclipped(range.start), Bias::Left);
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
        buffers: HashSet<Model<Buffer>>,
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

        cx.spawn(move |this, mut cx| async move {
            let mut project_transaction = ProjectTransaction::default();

            if let Some((project_id, remote_buffers)) = remote_buffers {
                let response = client
                    .request(proto::ReloadBuffers {
                        project_id,
                        buffer_ids: remote_buffers
                            .iter()
                            .filter_map(|buffer| {
                                buffer
                                    .update(&mut cx, |buffer, _| buffer.remote_id().into())
                                    .ok()
                            })
                            .collect(),
                    })
                    .await?
                    .transaction
                    .ok_or_else(|| anyhow!("missing transaction"))?;
                project_transaction = this
                    .update(&mut cx, |this, cx| {
                        this.deserialize_project_transaction(response, push_to_history, cx)
                    })?
                    .await?;
            }

            for buffer in local_buffers {
                let transaction = buffer
                    .update(&mut cx, |buffer, cx| buffer.reload(cx))?
                    .await?;
                buffer.update(&mut cx, |buffer, cx| {
                    if let Some(transaction) = transaction {
                        if !push_to_history {
                            buffer.forget_transaction(transaction.id);
                        }
                        project_transaction.0.insert(cx.handle(), transaction);
                    }
                })?;
            }

            Ok(project_transaction)
        })
    }

    pub fn format(
        &mut self,
        buffers: HashSet<Model<Buffer>>,
        push_to_history: bool,
        trigger: FormatTrigger,
        cx: &mut ModelContext<Project>,
    ) -> Task<anyhow::Result<ProjectTransaction>> {
        if self.is_local() {
            let mut buffers_with_paths = buffers
                .into_iter()
                .filter_map(|buffer_handle| {
                    let buffer = buffer_handle.read(cx);
                    let file = File::from_dyn(buffer.file())?;
                    let buffer_abs_path = file.as_local().map(|f| f.abs_path(cx));
                    Some((buffer_handle, buffer_abs_path))
                })
                .collect::<Vec<_>>();

            cx.spawn(move |project, mut cx| async move {
                // Do not allow multiple concurrent formatting requests for the
                // same buffer.
                project.update(&mut cx, |this, cx| {
                    buffers_with_paths.retain(|(buffer, _)| {
                        this.buffers_being_formatted
                            .insert(buffer.read(cx).remote_id())
                    });
                })?;

                let _cleanup = defer({
                    let this = project.clone();
                    let mut cx = cx.clone();
                    let buffers = &buffers_with_paths;
                    move || {
                        this.update(&mut cx, |this, cx| {
                            for (buffer, _) in buffers {
                                this.buffers_being_formatted
                                    .remove(&buffer.read(cx).remote_id());
                            }
                        })
                        .ok();
                    }
                });

                let mut project_transaction = ProjectTransaction::default();
                for (buffer, buffer_abs_path) in &buffers_with_paths {
                    let adapters_and_servers: Vec<_> = project.update(&mut cx, |project, cx| {
                        project
                            .language_servers_for_buffer(&buffer.read(cx), cx)
                            .map(|(adapter, lsp)| (adapter.clone(), lsp.clone()))
                            .collect()
                    })?;

                    let settings = buffer.update(&mut cx, |buffer, cx| {
                        language_settings(buffer.language(), buffer.file(), cx).clone()
                    })?;

                    let remove_trailing_whitespace = settings.remove_trailing_whitespace_on_save;
                    let ensure_final_newline = settings.ensure_final_newline_on_save;
                    let tab_size = settings.tab_size;

                    // First, format buffer's whitespace according to the settings.
                    let trailing_whitespace_diff = if remove_trailing_whitespace {
                        Some(
                            buffer
                                .update(&mut cx, |b, cx| b.remove_trailing_whitespace(cx))?
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
                    })?;

                    for (lsp_adapter, language_server) in adapters_and_servers.iter() {
                        // Apply the code actions on
                        let code_actions: Vec<lsp::CodeActionKind> = settings
                            .code_actions_on_format
                            .iter()
                            .flat_map(|(kind, enabled)| {
                                if *enabled {
                                    Some(kind.clone().into())
                                } else {
                                    None
                                }
                            })
                            .collect();

                        #[allow(clippy::nonminimal_bool)]
                        if !code_actions.is_empty()
                            && !(trigger == FormatTrigger::Save
                                && settings.format_on_save == FormatOnSave::Off)
                        {
                            let actions = project
                                .update(&mut cx, |this, cx| {
                                    this.request_lsp(
                                        buffer.clone(),
                                        LanguageServerToQuery::Other(language_server.server_id()),
                                        GetCodeActions {
                                            range: text::Anchor::MIN..text::Anchor::MAX,
                                            kinds: Some(code_actions),
                                        },
                                        cx,
                                    )
                                })?
                                .await?;

                            for action in actions {
                                if let Some(edit) = action.lsp_action.edit {
                                    if edit.changes.is_none() && edit.document_changes.is_none() {
                                        continue;
                                    }

                                    let new = Self::deserialize_workspace_edit(
                                        project
                                            .upgrade()
                                            .ok_or_else(|| anyhow!("project dropped"))?,
                                        edit,
                                        push_to_history,
                                        lsp_adapter.clone(),
                                        language_server.clone(),
                                        &mut cx,
                                    )
                                    .await?;
                                    project_transaction.0.extend(new.0);
                                }

                                if let Some(command) = action.lsp_action.command {
                                    project.update(&mut cx, |this, _| {
                                        this.last_workspace_edits_by_language_server
                                            .remove(&language_server.server_id());
                                    })?;

                                    language_server
                                        .request::<lsp::request::ExecuteCommand>(
                                            lsp::ExecuteCommandParams {
                                                command: command.command,
                                                arguments: command.arguments.unwrap_or_default(),
                                                ..Default::default()
                                            },
                                        )
                                        .await?;

                                    project.update(&mut cx, |this, _| {
                                        project_transaction.0.extend(
                                            this.last_workspace_edits_by_language_server
                                                .remove(&language_server.server_id())
                                                .unwrap_or_default()
                                                .0,
                                        )
                                    })?;
                                }
                            }
                        }
                    }

                    // Apply language-specific formatting using either the primary language server
                    // or external command.
                    let primary_language_server = adapters_and_servers
                        .first()
                        .cloned()
                        .map(|(_, lsp)| lsp.clone());
                    let server_and_buffer = primary_language_server
                        .as_ref()
                        .zip(buffer_abs_path.as_ref());

                    let mut format_operation = None;
                    match (&settings.formatter, &settings.format_on_save) {
                        (_, FormatOnSave::Off) if trigger == FormatTrigger::Save => {}

                        (Formatter::LanguageServer, FormatOnSave::On | FormatOnSave::Off)
                        | (_, FormatOnSave::LanguageServer) => {
                            if let Some((language_server, buffer_abs_path)) = server_and_buffer {
                                format_operation = Some(FormatOperation::Lsp(
                                    Self::format_via_lsp(
                                        &project,
                                        buffer,
                                        buffer_abs_path,
                                        language_server,
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
                                    buffer,
                                    buffer_abs_path,
                                    command,
                                    arguments,
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
                        (Formatter::Auto, FormatOnSave::On | FormatOnSave::Off) => {
                            if let Some(new_operation) =
                                prettier_support::format_with_prettier(&project, buffer, &mut cx)
                                    .await
                            {
                                format_operation = Some(new_operation);
                            } else if let Some((language_server, buffer_abs_path)) =
                                server_and_buffer
                            {
                                format_operation = Some(FormatOperation::Lsp(
                                    Self::format_via_lsp(
                                        &project,
                                        buffer,
                                        buffer_abs_path,
                                        language_server,
                                        tab_size,
                                        &mut cx,
                                    )
                                    .await
                                    .context("failed to format via language server")?,
                                ));
                            }
                        }
                        (Formatter::Prettier, FormatOnSave::On | FormatOnSave::Off) => {
                            if let Some(new_operation) =
                                prettier_support::format_with_prettier(&project, buffer, &mut cx)
                                    .await
                            {
                                format_operation = Some(new_operation);
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
                                FormatOperation::Prettier(diff) => {
                                    b.apply_diff(diff, cx);
                                }
                            }

                            if let Some(transaction_id) = whitespace_transaction_id {
                                b.group_until_transaction(transaction_id);
                            } else if let Some(transaction) = project_transaction.0.get(buffer) {
                                b.group_until_transaction(transaction.id)
                            }
                        }

                        if let Some(transaction) = b.finalize_last_transaction().cloned() {
                            if !push_to_history {
                                b.forget_transaction(transaction.id);
                            }
                            project_transaction.0.insert(buffer.clone(), transaction);
                        }
                    })?;
                }

                Ok(project_transaction)
            })
        } else {
            let remote_id = self.remote_id();
            let client = self.client.clone();
            cx.spawn(move |this, mut cx| async move {
                let mut project_transaction = ProjectTransaction::default();
                if let Some(project_id) = remote_id {
                    let response = client
                        .request(proto::FormatBuffers {
                            project_id,
                            trigger: trigger as i32,
                            buffer_ids: buffers
                                .iter()
                                .map(|buffer| {
                                    buffer.update(&mut cx, |buffer, _| buffer.remote_id().into())
                                })
                                .collect::<Result<_>>()?,
                        })
                        .await?
                        .transaction
                        .ok_or_else(|| anyhow!("missing transaction"))?;
                    project_transaction = this
                        .update(&mut cx, |this, cx| {
                            this.deserialize_project_transaction(response, push_to_history, cx)
                        })?
                        .await?;
                }
                Ok(project_transaction)
            })
        }
    }

    async fn format_via_lsp(
        this: &WeakModel<Self>,
        buffer: &Model<Buffer>,
        abs_path: &Path,
        language_server: &Arc<LanguageServer>,
        tab_size: NonZeroU32,
        cx: &mut AsyncAppContext,
    ) -> Result<Vec<(Range<Anchor>, String)>> {
        let uri = lsp::Url::from_file_path(abs_path)
            .map_err(|_| anyhow!("failed to convert abs path to uri"))?;
        let text_document = lsp::TextDocumentIdentifier::new(uri);
        let capabilities = &language_server.capabilities();

        let formatting_provider = capabilities.document_formatting_provider.as_ref();
        let range_formatting_provider = capabilities.document_range_formatting_provider.as_ref();

        let lsp_edits = if matches!(formatting_provider, Some(p) if *p != OneOf::Left(false)) {
            language_server
                .request::<lsp::request::Formatting>(lsp::DocumentFormattingParams {
                    text_document,
                    options: lsp_command::lsp_formatting_options(tab_size.get()),
                    work_done_progress_params: Default::default(),
                })
                .await?
        } else if matches!(range_formatting_provider, Some(p) if *p != OneOf::Left(false)) {
            let buffer_start = lsp::Position::new(0, 0);
            let buffer_end = buffer.update(cx, |b, _| point_to_lsp(b.max_point_utf16()))?;

            language_server
                .request::<lsp::request::RangeFormatting>(lsp::DocumentRangeFormattingParams {
                    text_document,
                    range: lsp::Range::new(buffer_start, buffer_end),
                    options: lsp_command::lsp_formatting_options(tab_size.get()),
                    work_done_progress_params: Default::default(),
                })
                .await?
        } else {
            None
        };

        if let Some(lsp_edits) = lsp_edits {
            this.update(cx, |this, cx| {
                this.edits_from_lsp(buffer, lsp_edits, language_server.server_id(), None, cx)
            })?
            .await
        } else {
            Ok(Vec::new())
        }
    }

    async fn format_via_external_command(
        buffer: &Model<Buffer>,
        buffer_abs_path: &Path,
        command: &str,
        arguments: &[String],
        cx: &mut AsyncAppContext,
    ) -> Result<Option<Diff>> {
        let working_dir_path = buffer.update(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file())?;
            let worktree = file.worktree.read(cx).as_local()?;
            let mut worktree_path = worktree.abs_path().to_path_buf();
            if worktree.root_entry()?.is_file() {
                worktree_path.pop();
            }
            Some(worktree_path)
        })?;

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
            let text = buffer.update(cx, |buffer, _| buffer.as_rope().clone())?;
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
                    .update(cx, |buffer, cx| buffer.diff(stdout, cx))?
                    .await,
            ))
        } else {
            Ok(None)
        }
    }

    #[inline(never)]
    fn definition_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetDefinition { position },
            cx,
        )
    }
    pub fn definition<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.definition_impl(buffer, position, cx)
    }

    fn type_definition_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetTypeDefinition { position },
            cx,
        )
    }

    pub fn type_definition<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.type_definition_impl(buffer, position, cx)
    }

    fn implementation_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetImplementation { position },
            cx,
        )
    }

    pub fn implementation<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.implementation_impl(buffer, position, cx)
    }

    fn references_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Location>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetReferences { position },
            cx,
        )
    }
    pub fn references<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Location>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.references_impl(buffer, position, cx)
    }

    fn document_highlights_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<DocumentHighlight>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetDocumentHighlights { position },
            cx,
        )
    }

    pub fn document_highlights<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<DocumentHighlight>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.document_highlights_impl(buffer, position, cx)
    }

    pub fn symbols(&self, query: &str, cx: &mut ModelContext<Self>) -> Task<Result<Vec<Symbol>>> {
        if self.is_local() {
            let mut requests = Vec::new();
            for ((worktree_id, _), server_id) in self.language_server_ids.iter() {
                let worktree_id = *worktree_id;
                let worktree_handle = self.worktree_for_id(worktree_id, cx);
                let worktree = match worktree_handle.and_then(|tree| tree.read(cx).as_local()) {
                    Some(worktree) => worktree,
                    None => continue,
                };
                let worktree_abs_path = worktree.abs_path().clone();

                let (adapter, language, server) = match self.language_servers.get(server_id) {
                    Some(LanguageServerState::Running {
                        adapter,
                        language,
                        server,
                        ..
                    }) => (adapter.clone(), language.clone(), server),

                    _ => continue,
                };

                requests.push(
                    server
                        .request::<lsp::request::WorkspaceSymbolRequest>(
                            lsp::WorkspaceSymbolParams {
                                query: query.to_string(),
                                ..Default::default()
                            },
                        )
                        .log_err()
                        .map(move |response| {
                            let lsp_symbols = response.flatten().map(|symbol_response| match symbol_response {
                                lsp::WorkspaceSymbolResponse::Flat(flat_responses) => {
                                    flat_responses.into_iter().map(|lsp_symbol| {
                                        (lsp_symbol.name, lsp_symbol.kind, lsp_symbol.location)
                                    }).collect::<Vec<_>>()
                                }
                                lsp::WorkspaceSymbolResponse::Nested(nested_responses) => {
                                    nested_responses.into_iter().filter_map(|lsp_symbol| {
                                        let location = match lsp_symbol.location {
                                            OneOf::Left(location) => location,
                                            OneOf::Right(_) => {
                                                error!("Unexpected: client capabilities forbid symbol resolutions in workspace.symbol.resolveSupport");
                                                return None
                                            }
                                        };
                                        Some((lsp_symbol.name, lsp_symbol.kind, location))
                                    }).collect::<Vec<_>>()
                                }
                            }).unwrap_or_default();

                            (
                                adapter,
                                language,
                                worktree_id,
                                worktree_abs_path,
                                lsp_symbols,
                            )
                        }),
                );
            }

            cx.spawn(move |this, mut cx| async move {
                let responses = futures::future::join_all(requests).await;
                let this = match this.upgrade() {
                    Some(this) => this,
                    None => return Ok(Vec::new()),
                };

                let symbols = this.update(&mut cx, |this, cx| {
                    let mut symbols = Vec::new();
                    for (
                        adapter,
                        adapter_language,
                        source_worktree_id,
                        worktree_abs_path,
                        lsp_symbols,
                    ) in responses
                    {
                        symbols.extend(lsp_symbols.into_iter().filter_map(
                            |(symbol_name, symbol_kind, symbol_location)| {
                                let abs_path = symbol_location.uri.to_file_path().ok()?;
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
                                let adapter = adapter.clone();
                                Some(async move {
                                    let language = language.await;
                                    let label = adapter
                                        .label_for_symbol(&symbol_name, symbol_kind, &language)
                                        .await;

                                    Symbol {
                                        language_server_name: adapter.name.clone(),
                                        source_worktree_id,
                                        path: project_path,
                                        label: label.unwrap_or_else(|| {
                                            CodeLabel::plain(symbol_name.clone(), None)
                                        }),
                                        kind: symbol_kind,
                                        name: symbol_name,
                                        range: range_from_lsp(symbol_location.range),
                                        signature,
                                    }
                                })
                            },
                        ));
                    }

                    symbols
                })?;

                Ok(futures::future::join_all(symbols).await)
            })
        } else if let Some(project_id) = self.remote_id() {
            let request = self.client.request(proto::GetProjectSymbols {
                project_id,
                query: query.to_string(),
            });
            cx.spawn(move |this, mut cx| async move {
                let response = request.await?;
                let mut symbols = Vec::new();
                if let Some(this) = this.upgrade() {
                    let new_symbols = this.update(&mut cx, |this, _| {
                        response
                            .symbols
                            .into_iter()
                            .map(|symbol| this.deserialize_symbol(symbol))
                            .collect::<Vec<_>>()
                    })?;
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
    ) -> Task<Result<Model<Buffer>>> {
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

            let symbol_abs_path = resolve_path(worktree_abs_path, &symbol.path.path);
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
            cx.spawn(move |this, mut cx| async move {
                let response = request.await?;
                let buffer_id = BufferId::new(response.buffer_id)?;
                this.update(&mut cx, |this, cx| {
                    this.wait_for_remote_buffer(buffer_id, cx)
                })?
                .await
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    fn hover_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Hover>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetHover { position },
            cx,
        )
    }
    pub fn hover<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Hover>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.hover_impl(buffer, position, cx)
    }

    #[inline(never)]
    fn completions_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        if self.is_local() {
            let snapshot = buffer.read(cx).snapshot();
            let offset = position.to_offset(&snapshot);
            let scope = snapshot.language_scope_at(offset);

            let server_ids: Vec<_> = self
                .language_servers_for_buffer(buffer.read(cx), cx)
                .filter(|(_, server)| server.capabilities().completion_provider.is_some())
                .filter(|(adapter, _)| {
                    scope
                        .as_ref()
                        .map(|scope| scope.language_allowed(&adapter.name))
                        .unwrap_or(true)
                })
                .map(|(_, server)| server.server_id())
                .collect();

            let buffer = buffer.clone();
            cx.spawn(move |this, mut cx| async move {
                let mut tasks = Vec::with_capacity(server_ids.len());
                this.update(&mut cx, |this, cx| {
                    for server_id in server_ids {
                        tasks.push(this.request_lsp(
                            buffer.clone(),
                            LanguageServerToQuery::Other(server_id),
                            GetCompletions { position },
                            cx,
                        ));
                    }
                })?;

                let mut completions = Vec::new();
                for task in tasks {
                    if let Ok(new_completions) = task.await {
                        completions.extend_from_slice(&new_completions);
                    }
                }

                Ok(completions)
            })
        } else if let Some(project_id) = self.remote_id() {
            self.send_lsp_proto_request(buffer.clone(), project_id, GetCompletions { position }, cx)
        } else {
            Task::ready(Ok(Default::default()))
        }
    }
    pub fn completions<T: ToOffset + ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.completions_impl(buffer, position, cx)
    }

    pub fn resolve_completions(
        &self,
        completion_indices: Vec<usize>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<bool>> {
        let client = self.client();
        let language_registry = self.languages().clone();

        let is_remote = self.is_remote();
        let project_id = self.remote_id();

        cx.spawn(move |this, mut cx| async move {
            let mut did_resolve = false;
            if is_remote {
                let project_id =
                    project_id.ok_or_else(|| anyhow!("Remote project without remote_id"))?;

                for completion_index in completion_indices {
                    let completions_guard = completions.read();
                    let completion = &completions_guard[completion_index];
                    if completion.documentation.is_some() {
                        continue;
                    }

                    did_resolve = true;
                    let server_id = completion.server_id;
                    let completion = completion.lsp_completion.clone();
                    drop(completions_guard);

                    Self::resolve_completion_documentation_remote(
                        project_id,
                        server_id,
                        completions.clone(),
                        completion_index,
                        completion,
                        client.clone(),
                        language_registry.clone(),
                    )
                    .await;
                }
            } else {
                for completion_index in completion_indices {
                    let completions_guard = completions.read();
                    let completion = &completions_guard[completion_index];
                    if completion.documentation.is_some() {
                        continue;
                    }

                    let server_id = completion.server_id;
                    let completion = completion.lsp_completion.clone();
                    drop(completions_guard);

                    let server = this
                        .read_with(&mut cx, |project, _| {
                            project.language_server_for_id(server_id)
                        })
                        .ok()
                        .flatten();
                    let Some(server) = server else {
                        continue;
                    };

                    did_resolve = true;
                    Self::resolve_completion_documentation_local(
                        server,
                        completions.clone(),
                        completion_index,
                        completion,
                        language_registry.clone(),
                    )
                    .await;
                }
            }

            Ok(did_resolve)
        })
    }

    async fn resolve_completion_documentation_local(
        server: Arc<lsp::LanguageServer>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        completion_index: usize,
        completion: lsp::CompletionItem,
        language_registry: Arc<LanguageRegistry>,
    ) {
        let can_resolve = server
            .capabilities()
            .completion_provider
            .as_ref()
            .and_then(|options| options.resolve_provider)
            .unwrap_or(false);
        if !can_resolve {
            return;
        }

        let request = server.request::<lsp::request::ResolveCompletionItem>(completion);
        let Some(completion_item) = request.await.log_err() else {
            return;
        };

        if let Some(lsp_documentation) = completion_item.documentation {
            let documentation = language::prepare_completion_documentation(
                &lsp_documentation,
                &language_registry,
                None, // TODO: Try to reasonably work out which language the completion is for
            )
            .await;

            let mut completions = completions.write();
            let completion = &mut completions[completion_index];
            completion.documentation = Some(documentation);
        } else {
            let mut completions = completions.write();
            let completion = &mut completions[completion_index];
            completion.documentation = Some(Documentation::Undocumented);
        }
    }

    async fn resolve_completion_documentation_remote(
        project_id: u64,
        server_id: LanguageServerId,
        completions: Arc<RwLock<Box<[Completion]>>>,
        completion_index: usize,
        completion: lsp::CompletionItem,
        client: Arc<Client>,
        language_registry: Arc<LanguageRegistry>,
    ) {
        let request = proto::ResolveCompletionDocumentation {
            project_id,
            language_server_id: server_id.0 as u64,
            lsp_completion: serde_json::to_string(&completion).unwrap().into_bytes(),
        };

        let Some(response) = client
            .request(request)
            .await
            .context("completion documentation resolve proto request")
            .log_err()
        else {
            return;
        };

        if response.text.is_empty() {
            let mut completions = completions.write();
            let completion = &mut completions[completion_index];
            completion.documentation = Some(Documentation::Undocumented);
        }

        let documentation = if response.is_markdown {
            Documentation::MultiLineMarkdown(
                markdown::parse_markdown(&response.text, &language_registry, None).await,
            )
        } else if response.text.lines().count() <= 1 {
            Documentation::SingleLine(response.text)
        } else {
            Documentation::MultiLinePlainText(response.text)
        };

        let mut completions = completions.write();
        let completion = &mut completions[completion_index];
        completion.documentation = Some(documentation);
    }

    pub fn apply_additional_edits_for_completion(
        &self,
        buffer_handle: Model<Buffer>,
        completion: Completion,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id();

        if self.is_local() {
            let server_id = completion.server_id;
            let lang_server = match self.language_server_for_buffer(buffer, server_id, cx) {
                Some((_, server)) => server.clone(),
                _ => return Task::ready(Ok(Default::default())),
            };

            cx.spawn(move |this, mut cx| async move {
                let can_resolve = lang_server
                    .capabilities()
                    .completion_provider
                    .as_ref()
                    .and_then(|options| options.resolve_provider)
                    .unwrap_or(false);
                let additional_text_edits = if can_resolve {
                    lang_server
                        .request::<lsp::request::ResolveCompletionItem>(completion.lsp_completion)
                        .await?
                        .additional_text_edits
                } else {
                    completion.lsp_completion.additional_text_edits
                };
                if let Some(edits) = additional_text_edits {
                    let edits = this
                        .update(&mut cx, |this, cx| {
                            this.edits_from_lsp(
                                &buffer_handle,
                                edits,
                                lang_server.server_id(),
                                None,
                                cx,
                            )
                        })?
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

                            //Skip additional edits which overlap with the primary completion edit
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
                    })?
                } else {
                    Ok(None)
                }
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
            cx.spawn(move |_, mut cx| async move {
                let response = client
                    .request(proto::ApplyCompletionAdditionalEdits {
                        project_id,
                        buffer_id: buffer_id.into(),
                        completion: Some(language::proto::serialize_completion(&completion)),
                    })
                    .await?;

                if let Some(transaction) = response.transaction {
                    let transaction = language::proto::deserialize_transaction(transaction)?;
                    buffer_handle
                        .update(&mut cx, |buffer, _| {
                            buffer.wait_for_edits(transaction.edit_ids.iter().copied())
                        })?
                        .await?;
                    if push_to_history {
                        buffer_handle.update(&mut cx, |buffer, _| {
                            buffer.push_transaction(transaction.clone(), Instant::now());
                        })?;
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

    fn code_actions_impl(
        &self,
        buffer_handle: &Model<Buffer>,
        range: Range<Anchor>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        self.request_lsp(
            buffer_handle.clone(),
            LanguageServerToQuery::Primary,
            GetCodeActions { range, kinds: None },
            cx,
        )
    }

    pub fn code_actions<T: Clone + ToOffset>(
        &self,
        buffer_handle: &Model<Buffer>,
        range: Range<T>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        let buffer = buffer_handle.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);
        self.code_actions_impl(buffer_handle, range, cx)
    }

    pub fn apply_code_action(
        &self,
        buffer_handle: Model<Buffer>,
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

            cx.spawn(move |this, mut cx| async move {
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
                        })?
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
                            this.upgrade().ok_or_else(|| anyhow!("no app present"))?,
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
                    })?;

                    let result = lang_server
                        .request::<lsp::request::ExecuteCommand>(lsp::ExecuteCommandParams {
                            command: command.command,
                            arguments: command.arguments.unwrap_or_default(),
                            ..Default::default()
                        })
                        .await;

                    if let Err(err) = result {
                        // TODO: LSP ERROR
                        return Err(err);
                    }

                    return this.update(&mut cx, |this, _| {
                        this.last_workspace_edits_by_language_server
                            .remove(&lang_server.server_id())
                            .unwrap_or_default()
                    });
                }

                Ok(ProjectTransaction::default())
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
            let request = proto::ApplyCodeAction {
                project_id,
                buffer_id: buffer_handle.read(cx).remote_id().into(),
                action: Some(language::proto::serialize_code_action(&action)),
            };
            cx.spawn(move |this, mut cx| async move {
                let response = client
                    .request(request)
                    .await?
                    .transaction
                    .ok_or_else(|| anyhow!("missing transaction"))?;
                this.update(&mut cx, |this, cx| {
                    this.deserialize_project_transaction(response, push_to_history, cx)
                })?
                .await
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    fn apply_on_type_formatting(
        &self,
        buffer: Model<Buffer>,
        position: Anchor,
        trigger: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        if self.is_local() {
            cx.spawn(move |this, mut cx| async move {
                // Do not allow multiple concurrent formatting requests for the
                // same buffer.
                this.update(&mut cx, |this, cx| {
                    this.buffers_being_formatted
                        .insert(buffer.read(cx).remote_id())
                })?;

                let _cleanup = defer({
                    let this = this.clone();
                    let mut cx = cx.clone();
                    let closure_buffer = buffer.clone();
                    move || {
                        this.update(&mut cx, |this, cx| {
                            this.buffers_being_formatted
                                .remove(&closure_buffer.read(cx).remote_id());
                        })
                        .ok();
                    }
                });

                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_edits(Some(position.timestamp))
                    })?
                    .await?;
                this.update(&mut cx, |this, cx| {
                    let position = position.to_point_utf16(buffer.read(cx));
                    this.on_type_format(buffer, position, trigger, false, cx)
                })?
                .await
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
            let request = proto::OnTypeFormatting {
                project_id,
                buffer_id: buffer.read(cx).remote_id().into(),
                position: Some(serialize_anchor(&position)),
                trigger,
                version: serialize_version(&buffer.read(cx).version()),
            };
            cx.spawn(move |_, _| async move {
                client
                    .request(request)
                    .await?
                    .transaction
                    .map(language::proto::deserialize_transaction)
                    .transpose()
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    async fn deserialize_edits(
        this: Model<Self>,
        buffer_to_edit: Model<Buffer>,
        edits: Vec<lsp::TextEdit>,
        push_to_history: bool,
        _: Arc<CachedLspAdapter>,
        language_server: Arc<LanguageServer>,
        cx: &mut AsyncAppContext,
    ) -> Result<Option<Transaction>> {
        let edits = this
            .update(cx, |this, cx| {
                this.edits_from_lsp(
                    &buffer_to_edit,
                    edits,
                    language_server.server_id(),
                    None,
                    cx,
                )
            })?
            .await?;

        let transaction = buffer_to_edit.update(cx, |buffer, cx| {
            buffer.finalize_last_transaction();
            buffer.start_transaction();
            for (range, text) in edits {
                buffer.edit([(range, text)], None, cx);
            }

            if buffer.end_transaction(cx).is_some() {
                let transaction = buffer.finalize_last_transaction().unwrap().clone();
                if !push_to_history {
                    buffer.forget_transaction(transaction.id);
                }
                Some(transaction)
            } else {
                None
            }
        })?;

        Ok(transaction)
    }

    async fn deserialize_workspace_edit(
        this: Model<Self>,
        edit: lsp::WorkspaceEdit,
        push_to_history: bool,
        lsp_adapter: Arc<CachedLspAdapter>,
        language_server: Arc<LanguageServer>,
        cx: &mut AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        let fs = this.update(cx, |this, _| this.fs.clone())?;
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
                    edits: edits.into_iter().map(OneOf::Left).collect(),
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
                        fs.create_file(
                            &abs_path,
                            op.options
                                .map(|options| fs::CreateOptions {
                                    overwrite: options.overwrite.unwrap_or(false),
                                    ignore_if_exists: options.ignore_if_exists.unwrap_or(false),
                                })
                                .unwrap_or_default(),
                        )
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
                        op.options
                            .map(|options| fs::RenameOptions {
                                overwrite: options.overwrite.unwrap_or(false),
                                ignore_if_exists: options.ignore_if_exists.unwrap_or(false),
                            })
                            .unwrap_or_default(),
                    )
                    .await?;
                }

                lsp::DocumentChangeOperation::Op(lsp::ResourceOp::Delete(op)) => {
                    let abs_path = op
                        .uri
                        .to_file_path()
                        .map_err(|_| anyhow!("can't convert URI to path"))?;
                    let options = op
                        .options
                        .map(|options| fs::RemoveOptions {
                            recursive: options.recursive.unwrap_or(false),
                            ignore_if_not_exists: options.ignore_if_not_exists.unwrap_or(false),
                        })
                        .unwrap_or_default();
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
                        })?
                        .await?;

                    let edits = this
                        .update(cx, |this, cx| {
                            let edits = op.edits.into_iter().map(|edit| match edit {
                                OneOf::Left(edit) => edit,
                                OneOf::Right(edit) => edit.text_edit,
                            });
                            this.edits_from_lsp(
                                &buffer_to_edit,
                                edits,
                                language_server.server_id(),
                                op.text_document.version,
                                cx,
                            )
                        })?
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
                    })?;
                    if let Some(transaction) = transaction {
                        project_transaction.0.insert(buffer_to_edit, transaction);
                    }
                }
            }
        }

        Ok(project_transaction)
    }

    fn prepare_rename_impl(
        &self,
        buffer: Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Range<Anchor>>>> {
        self.request_lsp(
            buffer,
            LanguageServerToQuery::Primary,
            PrepareRename { position },
            cx,
        )
    }
    pub fn prepare_rename<T: ToPointUtf16>(
        &self,
        buffer: Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Range<Anchor>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.prepare_rename_impl(buffer, position, cx)
    }

    fn perform_rename_impl(
        &self,
        buffer: Model<Buffer>,
        position: PointUtf16,
        new_name: String,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(
            buffer,
            LanguageServerToQuery::Primary,
            PerformRename {
                position,
                new_name,
                push_to_history,
            },
            cx,
        )
    }
    pub fn perform_rename<T: ToPointUtf16>(
        &self,
        buffer: Model<Buffer>,
        position: T,
        new_name: String,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.perform_rename_impl(buffer, position, new_name, push_to_history, cx)
    }

    pub fn on_type_format_impl(
        &self,
        buffer: Model<Buffer>,
        position: PointUtf16,
        trigger: String,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        let tab_size = buffer.update(cx, |buffer, cx| {
            language_settings(buffer.language_at(position).as_ref(), buffer.file(), cx).tab_size
        });
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            OnTypeFormatting {
                position,
                trigger,
                options: lsp_command::lsp_formatting_options(tab_size.get()).into(),
                push_to_history,
            },
            cx,
        )
    }

    pub fn on_type_format<T: ToPointUtf16>(
        &self,
        buffer: Model<Buffer>,
        position: T,
        trigger: String,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.on_type_format_impl(buffer, position, trigger, push_to_history, cx)
    }

    pub fn inlay_hints<T: ToOffset>(
        &self,
        buffer_handle: Model<Buffer>,
        range: Range<T>,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<Vec<InlayHint>>> {
        let buffer = buffer_handle.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);
        self.inlay_hints_impl(buffer_handle, range, cx)
    }
    fn inlay_hints_impl(
        &self,
        buffer_handle: Model<Buffer>,
        range: Range<Anchor>,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<Vec<InlayHint>>> {
        let buffer = buffer_handle.read(cx);
        let range_start = range.start;
        let range_end = range.end;
        let buffer_id = buffer.remote_id().into();
        let lsp_request = InlayHints { range };

        if self.is_local() {
            let lsp_request_task = self.request_lsp(
                buffer_handle.clone(),
                LanguageServerToQuery::Primary,
                lsp_request,
                cx,
            );
            cx.spawn(move |_, mut cx| async move {
                buffer_handle
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_edits(vec![range_start.timestamp, range_end.timestamp])
                    })?
                    .await
                    .context("waiting for inlay hint request range edits")?;
                lsp_request_task.await.context("inlay hints LSP request")
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
            let request = proto::InlayHints {
                project_id,
                buffer_id,
                start: Some(serialize_anchor(&range_start)),
                end: Some(serialize_anchor(&range_end)),
                version: serialize_version(&buffer_handle.read(cx).version()),
            };
            cx.spawn(move |project, cx| async move {
                let response = client
                    .request(request)
                    .await
                    .context("inlay hints proto request")?;
                LspCommand::response_from_proto(
                    lsp_request,
                    response,
                    project.upgrade().ok_or_else(|| anyhow!("No project"))?,
                    buffer_handle.clone(),
                    cx.clone(),
                )
                .await
                .context("inlay hints proto response conversion")
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    pub fn resolve_inlay_hint(
        &self,
        hint: InlayHint,
        buffer_handle: Model<Buffer>,
        server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<InlayHint>> {
        if self.is_local() {
            let buffer = buffer_handle.read(cx);
            let (_, lang_server) = if let Some((adapter, server)) =
                self.language_server_for_buffer(buffer, server_id, cx)
            {
                (adapter.clone(), server.clone())
            } else {
                return Task::ready(Ok(hint));
            };
            if !InlayHints::can_resolve_inlays(lang_server.capabilities()) {
                return Task::ready(Ok(hint));
            }

            let buffer_snapshot = buffer.snapshot();
            cx.spawn(move |_, mut cx| async move {
                let resolve_task = lang_server.request::<lsp::request::InlayHintResolveRequest>(
                    InlayHints::project_to_lsp_hint(hint, &buffer_snapshot),
                );
                let resolved_hint = resolve_task
                    .await
                    .context("inlay hint resolve LSP request")?;
                let resolved_hint = InlayHints::lsp_to_project_hint(
                    resolved_hint,
                    &buffer_handle,
                    server_id,
                    ResolveState::Resolved,
                    false,
                    &mut cx,
                )
                .await?;
                Ok(resolved_hint)
            })
        } else if let Some(project_id) = self.remote_id() {
            let client = self.client.clone();
            let request = proto::ResolveInlayHint {
                project_id,
                buffer_id: buffer_handle.read(cx).remote_id().into(),
                language_server_id: server_id.0 as u64,
                hint: Some(InlayHints::project_to_proto_hint(hint.clone())),
            };
            cx.spawn(move |_, _| async move {
                let response = client
                    .request(request)
                    .await
                    .context("inlay hints proto request")?;
                match response.hint {
                    Some(resolved_hint) => InlayHints::proto_to_project_hint(resolved_hint)
                        .context("inlay hints proto resolve response conversion"),
                    None => Ok(hint),
                }
            })
        } else {
            Task::ready(Err(anyhow!("project does not have a remote id")))
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn search(
        &self,
        query: SearchQuery,
        cx: &mut ModelContext<Self>,
    ) -> Receiver<(Model<Buffer>, Vec<Range<Anchor>>)> {
        if self.is_local() {
            self.search_local(query, cx)
        } else if let Some(project_id) = self.remote_id() {
            let (tx, rx) = smol::channel::unbounded();
            let request = self.client.request(query.to_proto(project_id));
            cx.spawn(move |this, mut cx| async move {
                let response = request.await?;
                let mut result = HashMap::default();
                for location in response.locations {
                    let buffer_id = BufferId::new(location.buffer_id)?;
                    let target_buffer = this
                        .update(&mut cx, |this, cx| {
                            this.wait_for_remote_buffer(buffer_id, cx)
                        })?
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
                for (buffer, ranges) in result {
                    let _ = tx.send((buffer, ranges)).await;
                }
                Result::<(), anyhow::Error>::Ok(())
            })
            .detach_and_log_err(cx);
            rx
        } else {
            unimplemented!();
        }
    }

    pub fn search_local(
        &self,
        query: SearchQuery,
        cx: &mut ModelContext<Self>,
    ) -> Receiver<(Model<Buffer>, Vec<Range<Anchor>>)> {
        // Local search is split into several phases.
        // TL;DR is that we do 2 passes; initial pass to pick files which contain at least one match
        // and the second phase that finds positions of all the matches found in the candidate files.
        // The Receiver obtained from this function returns matches sorted by buffer path. Files without a buffer path are reported first.
        //
        // It gets a bit hairy though, because we must account for files that do not have a persistent representation
        // on FS. Namely, if you have an untitled buffer or unsaved changes in a buffer, we want to scan that too.
        //
        // 1. We initialize a queue of match candidates and feed all opened buffers into it (== unsaved files / untitled buffers).
        //    Then, we go through a worktree and check for files that do match a predicate. If the file had an opened version, we skip the scan
        //    of FS version for that file altogether - after all, what we have in memory is more up-to-date than what's in FS.
        // 2. At this point, we have a list of all potentially matching buffers/files.
        //    We sort that list by buffer path - this list is retained for later use.
        //    We ensure that all buffers are now opened and available in project.
        // 3. We run a scan over all the candidate buffers on multiple background threads.
        //    We cannot assume that there will even be a match - while at least one match
        //    is guaranteed for files obtained from FS, the buffers we got from memory (unsaved files/unnamed buffers) might not have a match at all.
        //    There is also an auxiliary background thread responsible for result gathering.
        //    This is where the sorted list of buffers comes into play to maintain sorted order; Whenever this background thread receives a notification (buffer has/doesn't have matches),
        //    it keeps it around. It reports matches in sorted order, though it accepts them in unsorted order as well.
        //    As soon as the match info on next position in sorted order becomes available, it reports it (if it's a match) or skips to the next
        //    entry - which might already be available thanks to out-of-order processing.
        //
        // We could also report matches fully out-of-order, without maintaining a sorted list of matching paths.
        // This however would mean that project search (that is the main user of this function) would have to do the sorting itself, on the go.
        // This isn't as straightforward as running an insertion sort sadly, and would also mean that it would have to care about maintaining match index
        // in face of constantly updating list of sorted matches.
        // Meanwhile, this implementation offers index stability, since the matches are already reported in a sorted order.
        let snapshots = self
            .visible_worktrees(cx)
            .filter_map(|tree| {
                let tree = tree.read(cx).as_local()?;
                Some(tree.snapshot())
            })
            .collect::<Vec<_>>();

        let background = cx.background_executor().clone();
        let path_count: usize = snapshots
            .iter()
            .map(|s| {
                if query.include_ignored() {
                    s.file_count()
                } else {
                    s.visible_file_count()
                }
            })
            .sum();
        if path_count == 0 {
            let (_, rx) = smol::channel::bounded(1024);
            return rx;
        }
        let workers = background.num_cpus().min(path_count);
        let (matching_paths_tx, matching_paths_rx) = smol::channel::bounded(1024);
        let mut unnamed_files = vec![];
        let opened_buffers = self
            .opened_buffers
            .iter()
            .filter_map(|(_, b)| {
                let buffer = b.upgrade()?;
                let (is_ignored, snapshot) = buffer.update(cx, |buffer, cx| {
                    let is_ignored = buffer
                        .project_path(cx)
                        .and_then(|path| self.entry_for_path(&path, cx))
                        .map_or(false, |entry| entry.is_ignored);
                    (is_ignored, buffer.snapshot())
                });
                if is_ignored && !query.include_ignored() {
                    return None;
                } else if let Some(path) = snapshot.file().map(|file| file.path()) {
                    Some((path.clone(), (buffer, snapshot)))
                } else {
                    unnamed_files.push(buffer);
                    None
                }
            })
            .collect();
        cx.background_executor()
            .spawn(Self::background_search(
                unnamed_files,
                opened_buffers,
                cx.background_executor().clone(),
                self.fs.clone(),
                workers,
                query.clone(),
                path_count,
                snapshots,
                matching_paths_tx,
            ))
            .detach();

        let (buffers, buffers_rx) = Self::sort_candidates_and_open_buffers(matching_paths_rx, cx);
        let background = cx.background_executor().clone();
        let (result_tx, result_rx) = smol::channel::bounded(1024);
        cx.background_executor()
            .spawn(async move {
                let Ok(buffers) = buffers.await else {
                    return;
                };

                let buffers_len = buffers.len();
                if buffers_len == 0 {
                    return;
                }
                let query = &query;
                let (finished_tx, mut finished_rx) = smol::channel::unbounded();
                background
                    .scoped(|scope| {
                        #[derive(Clone)]
                        struct FinishedStatus {
                            entry: Option<(Model<Buffer>, Vec<Range<Anchor>>)>,
                            buffer_index: SearchMatchCandidateIndex,
                        }

                        for _ in 0..workers {
                            let finished_tx = finished_tx.clone();
                            let mut buffers_rx = buffers_rx.clone();
                            scope.spawn(async move {
                                while let Some((entry, buffer_index)) = buffers_rx.next().await {
                                    let buffer_matches = if let Some((_, snapshot)) = entry.as_ref()
                                    {
                                        if query.file_matches(
                                            snapshot.file().map(|file| file.path().as_ref()),
                                        ) {
                                            query
                                                .search(snapshot, None)
                                                .await
                                                .iter()
                                                .map(|range| {
                                                    snapshot.anchor_before(range.start)
                                                        ..snapshot.anchor_after(range.end)
                                                })
                                                .collect()
                                        } else {
                                            Vec::new()
                                        }
                                    } else {
                                        Vec::new()
                                    };

                                    let status = if !buffer_matches.is_empty() {
                                        let entry = if let Some((buffer, _)) = entry.as_ref() {
                                            Some((buffer.clone(), buffer_matches))
                                        } else {
                                            None
                                        };
                                        FinishedStatus {
                                            entry,
                                            buffer_index,
                                        }
                                    } else {
                                        FinishedStatus {
                                            entry: None,
                                            buffer_index,
                                        }
                                    };
                                    if finished_tx.send(status).await.is_err() {
                                        break;
                                    }
                                }
                            });
                        }
                        // Report sorted matches
                        scope.spawn(async move {
                            let mut current_index = 0;
                            let mut scratch = vec![None; buffers_len];
                            while let Some(status) = finished_rx.next().await {
                                debug_assert!(
                                    scratch[status.buffer_index].is_none(),
                                    "Got match status of position {} twice",
                                    status.buffer_index
                                );
                                let index = status.buffer_index;
                                scratch[index] = Some(status);
                                while current_index < buffers_len {
                                    let Some(current_entry) = scratch[current_index].take() else {
                                        // We intentionally **do not** increment `current_index` here. When next element arrives
                                        // from `finished_rx`, we will inspect the same position again, hoping for it to be Some(_)
                                        // this time.
                                        break;
                                    };
                                    if let Some(entry) = current_entry.entry {
                                        result_tx.send(entry).await.log_err();
                                    }
                                    current_index += 1;
                                }
                                if current_index == buffers_len {
                                    break;
                                }
                            }
                        });
                    })
                    .await;
            })
            .detach();
        result_rx
    }

    /// Pick paths that might potentially contain a match of a given search query.
    #[allow(clippy::too_many_arguments)]
    async fn background_search(
        unnamed_buffers: Vec<Model<Buffer>>,
        opened_buffers: HashMap<Arc<Path>, (Model<Buffer>, BufferSnapshot)>,
        executor: BackgroundExecutor,
        fs: Arc<dyn Fs>,
        workers: usize,
        query: SearchQuery,
        path_count: usize,
        snapshots: Vec<LocalSnapshot>,
        matching_paths_tx: Sender<SearchMatchCandidate>,
    ) {
        let fs = &fs;
        let query = &query;
        let matching_paths_tx = &matching_paths_tx;
        let snapshots = &snapshots;
        let paths_per_worker = (path_count + workers - 1) / workers;
        for buffer in unnamed_buffers {
            matching_paths_tx
                .send(SearchMatchCandidate::OpenBuffer {
                    buffer: buffer.clone(),
                    path: None,
                })
                .await
                .log_err();
        }
        for (path, (buffer, _)) in opened_buffers.iter() {
            matching_paths_tx
                .send(SearchMatchCandidate::OpenBuffer {
                    buffer: buffer.clone(),
                    path: Some(path.clone()),
                })
                .await
                .log_err();
        }
        executor
            .scoped(|scope| {
                let max_concurrent_workers = Arc::new(Semaphore::new(workers));

                for worker_ix in 0..workers {
                    let worker_start_ix = worker_ix * paths_per_worker;
                    let worker_end_ix = worker_start_ix + paths_per_worker;
                    let unnamed_buffers = opened_buffers.clone();
                    let limiter = Arc::clone(&max_concurrent_workers);
                    scope.spawn(async move {
                        let _guard = limiter.acquire().await;
                        let mut snapshot_start_ix = 0;
                        let mut abs_path = PathBuf::new();
                        for snapshot in snapshots {
                            let snapshot_end_ix = snapshot_start_ix
                                + if query.include_ignored() {
                                    snapshot.file_count()
                                } else {
                                    snapshot.visible_file_count()
                                };
                            if worker_end_ix <= snapshot_start_ix {
                                break;
                            } else if worker_start_ix > snapshot_end_ix {
                                snapshot_start_ix = snapshot_end_ix;
                                continue;
                            } else {
                                let start_in_snapshot =
                                    worker_start_ix.saturating_sub(snapshot_start_ix);
                                let end_in_snapshot =
                                    cmp::min(worker_end_ix, snapshot_end_ix) - snapshot_start_ix;

                                for entry in snapshot
                                    .files(query.include_ignored(), start_in_snapshot)
                                    .take(end_in_snapshot - start_in_snapshot)
                                {
                                    if matching_paths_tx.is_closed() {
                                        break;
                                    }
                                    if unnamed_buffers.contains_key(&entry.path) {
                                        continue;
                                    }
                                    let matches = if query.file_matches(Some(&entry.path)) {
                                        abs_path.clear();
                                        abs_path.push(&snapshot.abs_path());
                                        abs_path.push(&entry.path);
                                        if let Some(file) = fs.open_sync(&abs_path).await.log_err()
                                        {
                                            query.detect(file).unwrap_or(false)
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    };

                                    if matches {
                                        let project_path = SearchMatchCandidate::Path {
                                            worktree_id: snapshot.id(),
                                            path: entry.path.clone(),
                                            is_ignored: entry.is_ignored,
                                        };
                                        if matching_paths_tx.send(project_path).await.is_err() {
                                            break;
                                        }
                                    }
                                }

                                snapshot_start_ix = snapshot_end_ix;
                            }
                        }
                    });
                }

                if query.include_ignored() {
                    for snapshot in snapshots {
                        for ignored_entry in snapshot
                            .entries(query.include_ignored())
                            .filter(|e| e.is_ignored)
                        {
                            let limiter = Arc::clone(&max_concurrent_workers);
                            scope.spawn(async move {
                                let _guard = limiter.acquire().await;
                                let mut ignored_paths_to_process =
                                    VecDeque::from([snapshot.abs_path().join(&ignored_entry.path)]);
                                while let Some(ignored_abs_path) =
                                    ignored_paths_to_process.pop_front()
                                {
                                    if let Some(fs_metadata) = fs
                                        .metadata(&ignored_abs_path)
                                        .await
                                        .with_context(|| {
                                            format!("fetching fs metadata for {ignored_abs_path:?}")
                                        })
                                        .log_err()
                                        .flatten()
                                    {
                                        if fs_metadata.is_dir {
                                            if let Some(mut subfiles) = fs
                                                .read_dir(&ignored_abs_path)
                                                .await
                                                .with_context(|| {
                                                    format!(
                                                        "listing ignored path {ignored_abs_path:?}"
                                                    )
                                                })
                                                .log_err()
                                            {
                                                while let Some(subfile) = subfiles.next().await {
                                                    if let Some(subfile) = subfile.log_err() {
                                                        ignored_paths_to_process.push_back(subfile);
                                                    }
                                                }
                                            }
                                        } else if !fs_metadata.is_symlink {
                                            if !query.file_matches(Some(&ignored_abs_path))
                                                || snapshot.is_path_excluded(
                                                    ignored_entry.path.to_path_buf(),
                                                )
                                            {
                                                continue;
                                            }
                                            let matches = if let Some(file) = fs
                                                .open_sync(&ignored_abs_path)
                                                .await
                                                .with_context(|| {
                                                    format!(
                                                        "Opening ignored path {ignored_abs_path:?}"
                                                    )
                                                })
                                                .log_err()
                                            {
                                                query.detect(file).unwrap_or(false)
                                            } else {
                                                false
                                            };
                                            if matches {
                                                let project_path = SearchMatchCandidate::Path {
                                                    worktree_id: snapshot.id(),
                                                    path: Arc::from(
                                                        ignored_abs_path
                                                            .strip_prefix(snapshot.abs_path())
                                                            .expect(
                                                                "scanning worktree-related files",
                                                            ),
                                                    ),
                                                    is_ignored: true,
                                                };
                                                if matching_paths_tx
                                                    .send(project_path)
                                                    .await
                                                    .is_err()
                                                {
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            })
            .await;
    }

    pub fn request_lsp<R: LspCommand>(
        &self,
        buffer_handle: Model<Buffer>,
        server: LanguageServerToQuery,
        request: R,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<R::Response>>
    where
        <R::LspRequest as lsp::request::Request>::Result: Send,
        <R::LspRequest as lsp::request::Request>::Params: Send,
    {
        let buffer = buffer_handle.read(cx);
        if self.is_local() {
            let language_server = match server {
                LanguageServerToQuery::Primary => {
                    match self.primary_language_server_for_buffer(buffer, cx) {
                        Some((_, server)) => Some(Arc::clone(server)),
                        None => return Task::ready(Ok(Default::default())),
                    }
                }
                LanguageServerToQuery::Other(id) => self
                    .language_server_for_buffer(buffer, id, cx)
                    .map(|(_, server)| Arc::clone(server)),
            };
            let file = File::from_dyn(buffer.file()).and_then(File::as_local);
            if let (Some(file), Some(language_server)) = (file, language_server) {
                let lsp_params = request.to_lsp(&file.abs_path(cx), buffer, &language_server, cx);
                return cx.spawn(move |this, cx| async move {
                    if !request.check_capabilities(language_server.capabilities()) {
                        return Ok(Default::default());
                    }

                    let result = language_server.request::<R::LspRequest>(lsp_params).await;
                    let response = match result {
                        Ok(response) => response,

                        Err(err) => {
                            log::warn!(
                                "Generic lsp request to {} failed: {}",
                                language_server.name(),
                                err
                            );
                            return Err(err);
                        }
                    };

                    request
                        .response_from_lsp(
                            response,
                            this.upgrade().ok_or_else(|| anyhow!("no app context"))?,
                            buffer_handle,
                            language_server.server_id(),
                            cx,
                        )
                        .await
                });
            }
        } else if let Some(project_id) = self.remote_id() {
            return self.send_lsp_proto_request(buffer_handle, project_id, request, cx);
        }

        Task::ready(Ok(Default::default()))
    }

    fn send_lsp_proto_request<R: LspCommand>(
        &self,
        buffer: Model<Buffer>,
        project_id: u64,
        request: R,
        cx: &mut ModelContext<'_, Project>,
    ) -> Task<anyhow::Result<<R as LspCommand>::Response>> {
        let rpc = self.client.clone();
        let message = request.to_proto(project_id, buffer.read(cx));
        cx.spawn(move |this, mut cx| async move {
            // Ensure the project is still alive by the time the task
            // is scheduled.
            this.upgrade().context("project dropped")?;
            let response = rpc.request(message).await?;
            let this = this.upgrade().context("project dropped")?;
            if this.update(&mut cx, |this, _| this.is_disconnected())? {
                Err(anyhow!("disconnected before completing request"))
            } else {
                request
                    .response_from_proto(response, this, buffer, cx)
                    .await
            }
        })
    }

    fn sort_candidates_and_open_buffers(
        mut matching_paths_rx: Receiver<SearchMatchCandidate>,
        cx: &mut ModelContext<Self>,
    ) -> (
        futures::channel::oneshot::Receiver<Vec<SearchMatchCandidate>>,
        Receiver<(
            Option<(Model<Buffer>, BufferSnapshot)>,
            SearchMatchCandidateIndex,
        )>,
    ) {
        let (buffers_tx, buffers_rx) = smol::channel::bounded(1024);
        let (sorted_buffers_tx, sorted_buffers_rx) = futures::channel::oneshot::channel();
        cx.spawn(move |this, cx| async move {
            let mut buffers = Vec::new();
            let mut ignored_buffers = Vec::new();
            while let Some(entry) = matching_paths_rx.next().await {
                if matches!(
                    entry,
                    SearchMatchCandidate::Path {
                        is_ignored: true,
                        ..
                    }
                ) {
                    ignored_buffers.push(entry);
                } else {
                    buffers.push(entry);
                }
            }
            buffers.sort_by_key(|candidate| candidate.path());
            ignored_buffers.sort_by_key(|candidate| candidate.path());
            buffers.extend(ignored_buffers);
            let matching_paths = buffers.clone();
            let _ = sorted_buffers_tx.send(buffers);
            for (index, candidate) in matching_paths.into_iter().enumerate() {
                if buffers_tx.is_closed() {
                    break;
                }
                let this = this.clone();
                let buffers_tx = buffers_tx.clone();
                cx.spawn(move |mut cx| async move {
                    let buffer = match candidate {
                        SearchMatchCandidate::OpenBuffer { buffer, .. } => Some(buffer),
                        SearchMatchCandidate::Path {
                            worktree_id, path, ..
                        } => this
                            .update(&mut cx, |this, cx| {
                                this.open_buffer((worktree_id, path), cx)
                            })?
                            .await
                            .log_err(),
                    };
                    if let Some(buffer) = buffer {
                        let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
                        buffers_tx
                            .send((Some((buffer, snapshot)), index))
                            .await
                            .log_err();
                    } else {
                        buffers_tx.send((None, index)).await.log_err();
                    }

                    Ok::<_, anyhow::Error>(())
                })
                .detach();
            }
        })
        .detach();
        (sorted_buffers_rx, buffers_rx)
    }

    pub fn find_or_create_local_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(Model<Worktree>, PathBuf)>> {
        let abs_path = abs_path.as_ref();
        if let Some((tree, relative_path)) = self.find_local_worktree(abs_path, cx) {
            Task::ready(Ok((tree, relative_path)))
        } else {
            let worktree = self.create_local_worktree(abs_path, visible, cx);
            cx.background_executor()
                .spawn(async move { Ok((worktree.await?, PathBuf::new())) })
        }
    }

    pub fn find_local_worktree(
        &self,
        abs_path: &Path,
        cx: &AppContext,
    ) -> Option<(Model<Worktree>, PathBuf)> {
        for tree in &self.worktrees {
            if let Some(tree) = tree.upgrade() {
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
            ProjectClientState::Shared { .. } => true,
            ProjectClientState::Local | ProjectClientState::Remote { .. } => false,
        }
    }

    fn create_local_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>>> {
        let fs = self.fs.clone();
        let client = self.client.clone();
        let next_entry_id = self.next_entry_id.clone();
        let path: Arc<Path> = abs_path.as_ref().into();
        let task = self
            .loading_local_worktrees
            .entry(path.clone())
            .or_insert_with(|| {
                cx.spawn(move |project, mut cx| {
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
                        })?;

                        let worktree = worktree?;
                        project
                            .update(&mut cx, |project, cx| project.add_worktree(&worktree, cx))?;
                        Ok(worktree)
                    }
                    .map_err(Arc::new)
                })
                .shared()
            })
            .clone();
        cx.background_executor().spawn(async move {
            match task.await {
                Ok(worktree) => Ok(worktree),
                Err(err) => Err(anyhow!("{}", err)),
            }
        })
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut ModelContext<Self>) {
        let mut servers_to_remove = HashMap::default();
        let mut servers_to_preserve = HashSet::default();
        for ((worktree_id, server_name), &server_id) in &self.language_server_ids {
            if worktree_id == &id_to_remove {
                servers_to_remove.insert(server_id, server_name.clone());
            } else {
                servers_to_preserve.insert(server_id);
            }
        }
        servers_to_remove.retain(|server_id, _| !servers_to_preserve.contains(server_id));
        for (server_id_to_remove, server_name) in servers_to_remove {
            self.language_server_ids
                .remove(&(id_to_remove, server_name));
            self.language_server_statuses.remove(&server_id_to_remove);
            self.last_workspace_edits_by_language_server
                .remove(&server_id_to_remove);
            self.language_servers.remove(&server_id_to_remove);
            cx.emit(Event::LanguageServerRemoved(server_id_to_remove));
        }

        let mut prettier_instances_to_clean = FuturesUnordered::new();
        if let Some(prettier_paths) = self.prettiers_per_worktree.remove(&id_to_remove) {
            for path in prettier_paths.iter().flatten() {
                if let Some(prettier_instance) = self.prettier_instances.remove(path) {
                    prettier_instances_to_clean.push(async move {
                        prettier_instance
                            .server()
                            .await
                            .map(|server| server.server_id())
                    });
                }
            }
        }
        cx.spawn(|project, mut cx| async move {
            while let Some(prettier_server_id) = prettier_instances_to_clean.next().await {
                if let Some(prettier_server_id) = prettier_server_id {
                    project
                        .update(&mut cx, |project, cx| {
                            project
                                .supplementary_language_servers
                                .remove(&prettier_server_id);
                            cx.emit(Event::LanguageServerRemoved(prettier_server_id));
                        })
                        .ok();
                }
            }
        })
        .detach();

        self.task_inventory().update(cx, |inventory, _| {
            inventory.remove_worktree_sources(id_to_remove);
        });

        self.worktrees.retain(|worktree| {
            if let Some(worktree) = worktree.upgrade() {
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

    fn add_worktree(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
        cx.observe(worktree, |_, _, cx| cx.notify()).detach();
        if worktree.read(cx).is_local() {
            cx.subscribe(worktree, |this, worktree, event, cx| match event {
                worktree::Event::UpdatedEntries(changes) => {
                    this.update_local_worktree_buffers(&worktree, changes, cx);
                    this.update_local_worktree_language_servers(&worktree, changes, cx);
                    this.update_local_worktree_settings(&worktree, changes, cx);
                    this.update_prettier_settings(&worktree, changes, cx);
                    cx.emit(Event::WorktreeUpdatedEntries(
                        worktree.read(cx).id(),
                        changes.clone(),
                    ));
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

        let handle_id = worktree.entity_id();
        cx.observe_release(worktree, move |this, worktree, cx| {
            let _ = this.remove_worktree(worktree.id(), cx);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store
                    .clear_local_settings(handle_id.as_u64() as usize, cx)
                    .log_err()
            });
        })
        .detach();

        cx.emit(Event::WorktreeAdded);
        self.metadata_changed(cx);
    }

    fn update_local_worktree_buffers(
        &mut self,
        worktree_handle: &Model<Worktree>,
        changes: &[(Arc<Path>, ProjectEntryId, PathChange)],
        cx: &mut ModelContext<Self>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();

        let mut renamed_buffers = Vec::new();
        for (path, entry_id, _) in changes {
            let worktree_id = worktree_handle.read(cx).id();
            let project_path = ProjectPath {
                worktree_id,
                path: path.clone(),
            };

            let buffer_id = match self.local_buffer_ids_by_entry_id.get(entry_id) {
                Some(&buffer_id) => buffer_id,
                None => match self.local_buffer_ids_by_path.get(&project_path) {
                    Some(&buffer_id) => buffer_id,
                    None => {
                        continue;
                    }
                },
            };

            let open_buffer = self.opened_buffers.get(&buffer_id);
            let buffer = if let Some(buffer) = open_buffer.and_then(|buffer| buffer.upgrade()) {
                buffer
            } else {
                self.opened_buffers.remove(&buffer_id);
                self.local_buffer_ids_by_path.remove(&project_path);
                self.local_buffer_ids_by_entry_id.remove(entry_id);
                continue;
            };

            buffer.update(cx, |buffer, cx| {
                if let Some(old_file) = File::from_dyn(buffer.file()) {
                    if old_file.worktree != *worktree_handle {
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
                            is_deleted: false,
                            is_private: entry.is_private,
                        }
                    } else if let Some(entry) = snapshot.entry_for_path(old_file.path().as_ref()) {
                        File {
                            is_local: true,
                            entry_id: Some(entry.id),
                            mtime: entry.mtime,
                            path: entry.path.clone(),
                            worktree: worktree_handle.clone(),
                            is_deleted: false,
                            is_private: entry.is_private,
                        }
                    } else {
                        File {
                            is_local: true,
                            entry_id: old_file.entry_id,
                            path: old_file.path().clone(),
                            mtime: old_file.mtime(),
                            worktree: worktree_handle.clone(),
                            is_deleted: true,
                            is_private: old_file.is_private,
                        }
                    };

                    let old_path = old_file.abs_path(cx);
                    if new_file.abs_path(cx) != old_path {
                        renamed_buffers.push((cx.handle(), old_file.clone()));
                        self.local_buffer_ids_by_path.remove(&project_path);
                        self.local_buffer_ids_by_path.insert(
                            ProjectPath {
                                worktree_id,
                                path: path.clone(),
                            },
                            buffer_id,
                        );
                    }

                    if new_file.entry_id != Some(*entry_id) {
                        self.local_buffer_ids_by_entry_id.remove(entry_id);
                        if let Some(entry_id) = new_file.entry_id {
                            self.local_buffer_ids_by_entry_id
                                .insert(entry_id, buffer_id);
                        }
                    }

                    if new_file != *old_file {
                        if let Some(project_id) = self.remote_id() {
                            self.client
                                .send(proto::UpdateBufferFile {
                                    project_id,
                                    buffer_id: buffer_id.into(),
                                    file: Some(new_file.to_proto()),
                                })
                                .log_err();
                        }

                        buffer.file_updated(Arc::new(new_file), cx);
                    }
                }
            });
        }

        for (buffer, old_file) in renamed_buffers {
            self.unregister_buffer_from_language_servers(&buffer, &old_file, cx);
            self.detect_language_for_buffer(&buffer, cx);
            self.register_buffer_with_language_servers(&buffer, cx);
        }
    }

    fn update_local_worktree_language_servers(
        &mut self,
        worktree_handle: &Model<Worktree>,
        changes: &[(Arc<Path>, ProjectEntryId, PathChange)],
        cx: &mut ModelContext<Self>,
    ) {
        if changes.is_empty() {
            return;
        }

        let worktree_id = worktree_handle.read(cx).id();
        let mut language_server_ids = self
            .language_server_ids
            .iter()
            .filter_map(|((server_worktree_id, _), server_id)| {
                (*server_worktree_id == worktree_id).then_some(*server_id)
            })
            .collect::<Vec<_>>();
        language_server_ids.sort();
        language_server_ids.dedup();

        let abs_path = worktree_handle.read(cx).abs_path();
        for server_id in &language_server_ids {
            if let Some(LanguageServerState::Running {
                server,
                watched_paths,
                ..
            }) = self.language_servers.get(server_id)
            {
                if let Some(watched_paths) = watched_paths.get(&worktree_id) {
                    let params = lsp::DidChangeWatchedFilesParams {
                        changes: changes
                            .iter()
                            .filter_map(|(path, _, change)| {
                                if !watched_paths.is_match(&path) {
                                    return None;
                                }
                                let typ = match change {
                                    PathChange::Loaded => return None,
                                    PathChange::Added => lsp::FileChangeType::CREATED,
                                    PathChange::Removed => lsp::FileChangeType::DELETED,
                                    PathChange::Updated => lsp::FileChangeType::CHANGED,
                                    PathChange::AddedOrUpdated => lsp::FileChangeType::CHANGED,
                                };
                                Some(lsp::FileEvent {
                                    uri: lsp::Url::from_file_path(abs_path.join(path)).unwrap(),
                                    typ,
                                })
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

    fn update_local_worktree_buffers_git_repos(
        &mut self,
        worktree_handle: Model<Worktree>,
        changed_repos: &UpdatedGitRepositoriesSet,
        cx: &mut ModelContext<Self>,
    ) {
        debug_assert!(worktree_handle.read(cx).is_local());

        // Identify the loading buffers whose containing repository that has changed.
        let future_buffers = self
            .loading_buffers_by_path
            .iter()
            .filter_map(|(project_path, receiver)| {
                if project_path.worktree_id != worktree_handle.read(cx).id() {
                    return None;
                }
                let path = &project_path.path;
                changed_repos
                    .iter()
                    .find(|(work_dir, _)| path.starts_with(work_dir))?;
                let receiver = receiver.clone();
                let path = path.clone();
                Some(async move {
                    wait_for_loading_buffer(receiver)
                        .await
                        .ok()
                        .map(|buffer| (buffer, path))
                })
            })
            .collect::<FuturesUnordered<_>>();

        // Identify the current buffers whose containing repository has changed.
        let current_buffers = self
            .opened_buffers
            .values()
            .filter_map(|buffer| {
                let buffer = buffer.upgrade()?;
                let file = File::from_dyn(buffer.read(cx).file())?;
                if file.worktree != worktree_handle {
                    return None;
                }
                let path = file.path();
                changed_repos
                    .iter()
                    .find(|(work_dir, _)| path.starts_with(work_dir))?;
                Some((buffer, path.clone()))
            })
            .collect::<Vec<_>>();

        if future_buffers.len() + current_buffers.len() == 0 {
            return;
        }

        let remote_id = self.remote_id();
        let client = self.client.clone();
        cx.spawn(move |_, mut cx| async move {
            // Wait for all of the buffers to load.
            let future_buffers = future_buffers.collect::<Vec<_>>().await;

            // Reload the diff base for every buffer whose containing git repository has changed.
            let snapshot =
                worktree_handle.update(&mut cx, |tree, _| tree.as_local().unwrap().snapshot())?;
            let diff_bases_by_buffer = cx
                .background_executor()
                .spawn(async move {
                    future_buffers
                        .into_iter()
                        .flatten()
                        .chain(current_buffers)
                        .filter_map(|(buffer, path)| {
                            let (work_directory, repo) =
                                snapshot.repository_and_work_directory_for_path(&path)?;
                            let repo = snapshot.get_local_repo(&repo)?;
                            let relative_path = path.strip_prefix(&work_directory).ok()?;
                            let base_text = repo.load_index_text(relative_path);
                            Some((buffer, base_text))
                        })
                        .collect::<Vec<_>>()
                })
                .await;

            // Assign the new diff bases on all of the buffers.
            for (buffer, diff_base) in diff_bases_by_buffer {
                let buffer_id = buffer.update(&mut cx, |buffer, cx| {
                    buffer.set_diff_base(diff_base.clone(), cx);
                    buffer.remote_id().into()
                })?;
                if let Some(project_id) = remote_id {
                    client
                        .send(proto::UpdateDiffBase {
                            project_id,
                            buffer_id,
                            diff_base,
                        })
                        .log_err();
                }
            }

            anyhow::Ok(())
        })
        .detach();
    }

    fn update_local_worktree_settings(
        &mut self,
        worktree: &Model<Worktree>,
        changes: &UpdatedEntriesSet,
        cx: &mut ModelContext<Self>,
    ) {
        if worktree.read(cx).as_local().is_none() {
            return;
        }
        let project_id = self.remote_id();
        let worktree_id = worktree.entity_id();
        let remote_worktree_id = worktree.read(cx).id();

        let mut settings_contents = Vec::new();
        for (path, _, change) in changes.iter() {
            let removed = change == &PathChange::Removed;
            let abs_path = match worktree.read(cx).absolutize(path) {
                Ok(abs_path) => abs_path,
                Err(e) => {
                    log::warn!("Cannot absolutize {path:?} received as {change:?} FS change: {e}");
                    continue;
                }
            };

            if abs_path.ends_with(&*LOCAL_SETTINGS_RELATIVE_PATH) {
                let settings_dir = Arc::from(
                    path.ancestors()
                        .nth(LOCAL_SETTINGS_RELATIVE_PATH.components().count())
                        .unwrap(),
                );
                let fs = self.fs.clone();
                settings_contents.push(async move {
                    (
                        settings_dir,
                        if removed {
                            None
                        } else {
                            Some(async move { fs.load(&abs_path).await }.await)
                        },
                    )
                });
            } else if abs_path.ends_with(&*LOCAL_TASKS_RELATIVE_PATH) {
                self.task_inventory().update(cx, |task_inventory, cx| {
                    if removed {
                        task_inventory.remove_local_static_source(&abs_path);
                    } else {
                        let fs = self.fs.clone();
                        let task_abs_path = abs_path.clone();
                        task_inventory.add_source(
                            TaskSourceKind::Worktree {
                                id: remote_worktree_id,
                                abs_path,
                            },
                            |cx| {
                                let tasks_file_rx =
                                    watch_config_file(&cx.background_executor(), fs, task_abs_path);
                                StaticSource::new(
                                    format!("local_tasks_for_workspace_{remote_worktree_id}"),
                                    tasks_file_rx,
                                    cx,
                                )
                            },
                            cx,
                        );
                    }
                })
            }
        }

        if settings_contents.is_empty() {
            return;
        }

        let client = self.client.clone();
        cx.spawn(move |_, cx| async move {
            let settings_contents: Vec<(Arc<Path>, _)> =
                futures::future::join_all(settings_contents).await;
            cx.update(|cx| {
                cx.update_global::<SettingsStore, _>(|store, cx| {
                    for (directory, file_content) in settings_contents {
                        let file_content = file_content.and_then(|content| content.log_err());
                        store
                            .set_local_settings(
                                worktree_id.as_u64() as usize,
                                directory.clone(),
                                file_content.as_deref(),
                                cx,
                            )
                            .log_err();
                        if let Some(remote_id) = project_id {
                            client
                                .send(proto::UpdateWorktreeSettings {
                                    project_id: remote_id,
                                    worktree_id: remote_worktree_id.to_proto(),
                                    path: directory.to_string_lossy().into_owned(),
                                    content: file_content,
                                })
                                .log_err();
                        }
                    }
                });
            })
            .ok();
        })
        .detach();
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

    pub fn diagnostic_summary(&self, include_ignored: bool, cx: &AppContext) -> DiagnosticSummary {
        let mut summary = DiagnosticSummary::default();
        for (_, _, path_summary) in
            self.diagnostic_summaries(include_ignored, cx)
                .filter(|(path, _, _)| {
                    let worktree = self.entry_for_path(path, cx).map(|entry| entry.is_ignored);
                    include_ignored || worktree == Some(false)
                })
        {
            summary.error_count += path_summary.error_count;
            summary.warning_count += path_summary.warning_count;
        }
        summary
    }

    pub fn diagnostic_summaries<'a>(
        &'a self,
        include_ignored: bool,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = (ProjectPath, LanguageServerId, DiagnosticSummary)> + 'a {
        self.visible_worktrees(cx)
            .flat_map(move |worktree| {
                let worktree = worktree.read(cx);
                let worktree_id = worktree.id();
                worktree
                    .diagnostic_summaries()
                    .map(move |(path, server_id, summary)| {
                        (ProjectPath { worktree_id, path }, server_id, summary)
                    })
            })
            .filter(move |(path, _, _)| {
                let worktree = self.entry_for_path(path, cx).map(|entry| entry.is_ignored);
                include_ignored || worktree == Some(false)
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

    pub fn absolute_path(&self, project_path: &ProjectPath, cx: &AppContext) -> Option<PathBuf> {
        let workspace_root = self
            .worktree_for_id(project_path.worktree_id, cx)?
            .read(cx)
            .abs_path();
        let project_path = project_path.path.as_ref();

        Some(if project_path == Path::new("") {
            workspace_root.to_path_buf()
        } else {
            workspace_root.join(project_path)
        })
    }

    pub fn get_repo(
        &self,
        project_path: &ProjectPath,
        cx: &AppContext,
    ) -> Option<Arc<Mutex<dyn GitRepository>>> {
        self.worktree_for_id(project_path.worktree_id, cx)?
            .read(cx)
            .as_local()?
            .snapshot()
            .local_git_repo(&project_path.path)
    }

    // RPC message handlers

    async fn handle_unshare_project(
        this: Model<Self>,
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
        })?
    }

    async fn handle_add_collaborator(
        this: Model<Self>,
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
            cx.emit(Event::CollaboratorJoined(collaborator.peer_id));
            this.collaborators
                .insert(collaborator.peer_id, collaborator);
            cx.notify();
        })?;

        Ok(())
    }

    async fn handle_update_project_collaborator(
        this: Model<Self>,
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
        })?
    }

    async fn handle_remove_collaborator(
        this: Model<Self>,
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
                if let Some(buffer) = buffer.upgrade() {
                    buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
                }
            }
            this.shared_buffers.remove(&peer_id);

            cx.emit(Event::CollaboratorLeft(peer_id));
            cx.notify();
            Ok(())
        })?
    }

    async fn handle_update_project(
        this: Model<Self>,
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
        })?
    }

    async fn handle_update_worktree(
        this: Model<Self>,
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
        })?
    }

    async fn handle_update_worktree_settings(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateWorktreeSettings>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            if let Some(worktree) = this.worktree_for_id(worktree_id, cx) {
                cx.update_global::<SettingsStore, _>(|store, cx| {
                    store
                        .set_local_settings(
                            worktree.entity_id().as_u64() as usize,
                            PathBuf::from(&envelope.payload.path).into(),
                            envelope.payload.content.as_deref(),
                            cx,
                        )
                        .log_err();
                });
            }
            Ok(())
        })?
    }

    async fn handle_create_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CreateProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let worktree = this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            this.worktree_for_id(worktree_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        let worktree_scan_id = worktree.update(&mut cx, |worktree, _| worktree.scan_id())?;
        let entry = worktree
            .update(&mut cx, |worktree, cx| {
                let worktree = worktree.as_local_mut().unwrap();
                let path = PathBuf::from(envelope.payload.path);
                worktree.create_entry(path, envelope.payload.is_directory, cx)
            })?
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: entry.as_ref().map(|e| e.into()),
            worktree_scan_id: worktree_scan_id as u64,
        })
    }

    async fn handle_rename_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::RenameProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        let worktree_scan_id = worktree.update(&mut cx, |worktree, _| worktree.scan_id())?;
        let entry = worktree
            .update(&mut cx, |worktree, cx| {
                let new_path = PathBuf::from(envelope.payload.new_path);
                worktree
                    .as_local_mut()
                    .unwrap()
                    .rename_entry(entry_id, new_path, cx)
            })?
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: entry.as_ref().map(|e| e.into()),
            worktree_scan_id: worktree_scan_id as u64,
        })
    }

    async fn handle_copy_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CopyProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        let worktree_scan_id = worktree.update(&mut cx, |worktree, _| worktree.scan_id())?;
        let entry = worktree
            .update(&mut cx, |worktree, cx| {
                let new_path = PathBuf::from(envelope.payload.new_path);
                worktree
                    .as_local_mut()
                    .unwrap()
                    .copy_entry(entry_id, new_path, cx)
            })?
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: entry.as_ref().map(|e| e.into()),
            worktree_scan_id: worktree_scan_id as u64,
        })
    }

    async fn handle_delete_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::DeleteProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);

        this.update(&mut cx, |_, cx| cx.emit(Event::DeletedEntry(entry_id)))?;

        let worktree = this.update(&mut cx, |this, cx| {
            this.worktree_for_entry(entry_id, cx)
                .ok_or_else(|| anyhow!("worktree not found"))
        })??;
        let worktree_scan_id = worktree.update(&mut cx, |worktree, _| worktree.scan_id())?;
        worktree
            .update(&mut cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .delete_entry(entry_id, cx)
                    .ok_or_else(|| anyhow!("invalid entry"))
            })??
            .await?;
        Ok(proto::ProjectEntryResponse {
            entry: None,
            worktree_scan_id: worktree_scan_id as u64,
        })
    }

    async fn handle_expand_project_entry(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ExpandProjectEntry>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ExpandProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this
            .update(&mut cx, |this, cx| this.worktree_for_entry(entry_id, cx))?
            .ok_or_else(|| anyhow!("invalid request"))?;
        worktree
            .update(&mut cx, |worktree, cx| {
                worktree
                    .as_local_mut()
                    .unwrap()
                    .expand_entry(entry_id, cx)
                    .ok_or_else(|| anyhow!("invalid entry"))
            })??
            .await?;
        let worktree_scan_id = worktree.update(&mut cx, |worktree, _| worktree.scan_id())? as u64;
        Ok(proto::ExpandProjectEntryResponse { worktree_scan_id })
    }

    async fn handle_update_diagnostic_summary(
        this: Model<Self>,
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
        })?
    }

    async fn handle_start_language_server(
        this: Model<Self>,
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
        })?;
        Ok(())
    }

    async fn handle_update_language_server(
        this: Model<Self>,
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
        })?
    }

    async fn handle_update_buffer(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |this, cx| {
            let payload = envelope.payload.clone();
            let buffer_id = BufferId::new(payload.buffer_id)?;
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
        })?
    }

    async fn handle_create_buffer_for_peer(
        this: Model<Self>,
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

                    let buffer_id = BufferId::new(state.id)?;
                    let buffer = cx.new_model(|_| {
                        Buffer::from_proto(this.replica_id(), this.capability(), state, buffer_file)
                            .unwrap()
                    });
                    this.incomplete_remote_buffers
                        .insert(buffer_id, Some(buffer));
                }
                proto::create_buffer_for_peer::Variant::Chunk(chunk) => {
                    let buffer_id = BufferId::new(chunk.buffer_id)?;
                    let buffer = this
                        .incomplete_remote_buffers
                        .get(&buffer_id)
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
                        this.incomplete_remote_buffers.remove(&buffer_id);
                        this.register_buffer(&buffer, cx)?;
                    }
                }
            }

            Ok(())
        })?
    }

    async fn handle_update_diff_base(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateDiffBase>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let buffer_id = envelope.payload.buffer_id;
            let buffer_id = BufferId::new(buffer_id)?;
            let diff_base = envelope.payload.diff_base;
            if let Some(buffer) = this
                .opened_buffers
                .get_mut(&buffer_id)
                .and_then(|b| b.upgrade())
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
        })?
    }

    async fn handle_update_buffer_file(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateBufferFile>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let buffer_id = envelope.payload.buffer_id;
        let buffer_id = BufferId::new(buffer_id)?;

        this.update(&mut cx, |this, cx| {
            let payload = envelope.payload.clone();
            if let Some(buffer) = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|b| b.upgrade())
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
                    buffer.file_updated(Arc::new(file), cx);
                });
                this.detect_language_for_buffer(&buffer, cx);
            }
            Ok(())
        })?
    }

    async fn handle_save_buffer(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SaveBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::BufferSaved> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let (project_id, buffer) = this.update(&mut cx, |this, _cx| {
            let project_id = this.remote_id().ok_or_else(|| anyhow!("not connected"))?;
            let buffer = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?;
            anyhow::Ok((project_id, buffer))
        })??;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&envelope.payload.version))
            })?
            .await?;
        let buffer_id = buffer.update(&mut cx, |buffer, _| buffer.remote_id())?;

        this.update(&mut cx, |this, cx| this.save_buffer(buffer.clone(), cx))?
            .await?;
        buffer.update(&mut cx, |buffer, _| proto::BufferSaved {
            project_id,
            buffer_id: buffer_id.into(),
            version: serialize_version(buffer.saved_version()),
            mtime: Some(buffer.saved_mtime().into()),
            fingerprint: language::proto::serialize_fingerprint(buffer.saved_version_fingerprint()),
        })
    }

    async fn handle_reload_buffers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ReloadBuffers>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ReloadBuffersResponse> {
        let sender_id = envelope.original_sender_id()?;
        let reload = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(
                    this.opened_buffers
                        .get(&buffer_id)
                        .and_then(|buffer| buffer.upgrade())
                        .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?,
                );
            }
            Ok::<_, anyhow::Error>(this.reload_buffers(buffers, false, cx))
        })??;

        let project_transaction = reload.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        })?;
        Ok(proto::ReloadBuffersResponse {
            transaction: Some(project_transaction),
        })
    }

    async fn handle_synchronize_buffers(
        this: Model<Self>,
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
                error!("missing original_sender_id on SynchronizeBuffers request");
                bail!("missing original_sender_id on SynchronizeBuffers request");
            };

            this.shared_buffers.entry(guest_id).or_default().clear();
            for buffer in envelope.payload.buffers {
                let buffer_id = BufferId::new(buffer.id)?;
                let remote_version = language::proto::deserialize_version(&buffer.version);
                if let Some(buffer) = this.buffer_for_id(buffer_id) {
                    this.shared_buffers
                        .entry(guest_id)
                        .or_default()
                        .insert(buffer_id);

                    let buffer = buffer.read(cx);
                    response.buffers.push(proto::BufferVersion {
                        id: buffer_id.into(),
                        version: language::proto::serialize_version(&buffer.version),
                    });

                    let operations = buffer.serialize_ops(Some(remote_version), cx);
                    let client = this.client.clone();
                    if let Some(file) = buffer.file() {
                        client
                            .send(proto::UpdateBufferFile {
                                project_id,
                                buffer_id: buffer_id.into(),
                                file: Some(file.to_proto()),
                            })
                            .log_err();
                    }

                    client
                        .send(proto::UpdateDiffBase {
                            project_id,
                            buffer_id: buffer_id.into(),
                            diff_base: buffer.diff_base().map(Into::into),
                        })
                        .log_err();

                    client
                        .send(proto::BufferReloaded {
                            project_id,
                            buffer_id: buffer_id.into(),
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

                    cx.background_executor()
                        .spawn(
                            async move {
                                let operations = operations.await;
                                for chunk in split_operations(operations) {
                                    client
                                        .request(proto::UpdateBuffer {
                                            project_id,
                                            buffer_id: buffer_id.into(),
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
            Ok(())
        })??;

        Ok(response)
    }

    async fn handle_format_buffers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::FormatBuffers>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FormatBuffersResponse> {
        let sender_id = envelope.original_sender_id()?;
        let format = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(
                    this.opened_buffers
                        .get(&buffer_id)
                        .and_then(|buffer| buffer.upgrade())
                        .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?,
                );
            }
            let trigger = FormatTrigger::from_proto(envelope.payload.trigger);
            Ok::<_, anyhow::Error>(this.format(buffers, false, trigger, cx))
        })??;

        let project_transaction = format.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        })?;
        Ok(proto::FormatBuffersResponse {
            transaction: Some(project_transaction),
        })
    }

    async fn handle_apply_additional_edits_for_completion(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ApplyCompletionAdditionalEdits>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ApplyCompletionAdditionalEditsResponse> {
        let languages = this.update(&mut cx, |this, _| this.languages.clone())?;
        let (buffer, completion) = this.update(&mut cx, |this, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            let buffer = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?;
            let language = buffer.read(cx).language();
            let completion = language::proto::deserialize_completion(
                envelope
                    .payload
                    .completion
                    .ok_or_else(|| anyhow!("invalid completion"))?,
                language.cloned(),
                &languages,
            );
            Ok::<_, anyhow::Error>((buffer, completion))
        })??;

        let completion = completion.await?;

        let apply_additional_edits = this.update(&mut cx, |this, cx| {
            this.apply_additional_edits_for_completion(buffer, completion, false, cx)
        })?;

        Ok(proto::ApplyCompletionAdditionalEditsResponse {
            transaction: apply_additional_edits
                .await?
                .as_ref()
                .map(language::proto::serialize_transaction),
        })
    }

    async fn handle_resolve_completion_documentation(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ResolveCompletionDocumentation>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ResolveCompletionDocumentationResponse> {
        let lsp_completion = serde_json::from_slice(&envelope.payload.lsp_completion)?;

        let completion = this
            .read_with(&mut cx, |this, _| {
                let id = LanguageServerId(envelope.payload.language_server_id as usize);
                let Some(server) = this.language_server_for_id(id) else {
                    return Err(anyhow!("No language server {id}"));
                };

                Ok(server.request::<lsp::request::ResolveCompletionItem>(lsp_completion))
            })??
            .await?;

        let mut is_markdown = false;
        let text = match completion.documentation {
            Some(lsp::Documentation::String(text)) => text,

            Some(lsp::Documentation::MarkupContent(lsp::MarkupContent { kind, value })) => {
                is_markdown = kind == lsp::MarkupKind::Markdown;
                value
            }

            _ => String::new(),
        };

        Ok(proto::ResolveCompletionDocumentationResponse { text, is_markdown })
    }

    async fn handle_apply_code_action(
        this: Model<Self>,
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
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            let buffer = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))?;
            Ok::<_, anyhow::Error>(this.apply_code_action(buffer, action, false, cx))
        })??;

        let project_transaction = apply_code_action.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        })?;
        Ok(proto::ApplyCodeActionResponse {
            transaction: Some(project_transaction),
        })
    }

    async fn handle_on_type_formatting(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::OnTypeFormatting>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OnTypeFormattingResponse> {
        let on_type_formatting = this.update(&mut cx, |this, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            let buffer = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))?;
            let position = envelope
                .payload
                .position
                .and_then(deserialize_anchor)
                .ok_or_else(|| anyhow!("invalid position"))?;
            Ok::<_, anyhow::Error>(this.apply_on_type_formatting(
                buffer,
                position,
                envelope.payload.trigger.clone(),
                cx,
            ))
        })??;

        let transaction = on_type_formatting
            .await?
            .as_ref()
            .map(language::proto::serialize_transaction);
        Ok(proto::OnTypeFormattingResponse { transaction })
    }

    async fn handle_inlay_hints(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::InlayHints>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::InlayHintsResponse> {
        let sender_id = envelope.original_sender_id()?;
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let buffer = this.update(&mut cx, |this, _| {
            this.opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .ok_or_else(|| anyhow!("unknown buffer id {}", envelope.payload.buffer_id))
        })??;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(deserialize_version(&envelope.payload.version))
            })?
            .await
            .with_context(|| format!("waiting for version for buffer {}", buffer.entity_id()))?;

        let start = envelope
            .payload
            .start
            .and_then(deserialize_anchor)
            .context("missing range start")?;
        let end = envelope
            .payload
            .end
            .and_then(deserialize_anchor)
            .context("missing range end")?;
        let buffer_hints = this
            .update(&mut cx, |project, cx| {
                project.inlay_hints(buffer.clone(), start..end, cx)
            })?
            .await
            .context("inlay hints fetch")?;

        this.update(&mut cx, |project, cx| {
            InlayHints::response_to_proto(
                buffer_hints,
                project,
                sender_id,
                &buffer.read(cx).version(),
                cx,
            )
        })
    }

    async fn handle_resolve_inlay_hint(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ResolveInlayHint>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ResolveInlayHintResponse> {
        let proto_hint = envelope
            .payload
            .hint
            .expect("incorrect protobuf resolve inlay hint message: missing the inlay hint");
        let hint = InlayHints::proto_to_project_hint(proto_hint)
            .context("resolved proto inlay hint conversion")?;
        let buffer = this.update(&mut cx, |this, _cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            this.opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))
        })??;
        let response_hint = this
            .update(&mut cx, |project, cx| {
                project.resolve_inlay_hint(
                    hint,
                    buffer,
                    LanguageServerId(envelope.payload.language_server_id as usize),
                    cx,
                )
            })?
            .await
            .context("inlay hints fetch")?;
        Ok(proto::ResolveInlayHintResponse {
            hint: Some(InlayHints::project_to_proto_hint(response_hint)),
        })
    }

    async fn handle_refresh_inlay_hints(
        this: Model<Self>,
        _: TypedEnvelope<proto::RefreshInlayHints>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |_, cx| {
            cx.emit(Event::RefreshInlayHints);
        })?;
        Ok(proto::Ack {})
    }

    async fn handle_lsp_command<T: LspCommand>(
        this: Model<Self>,
        envelope: TypedEnvelope<T::ProtoRequest>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<<T::ProtoRequest as proto::RequestMessage>::Response>
    where
        <T::LspRequest as lsp::request::Request>::Params: Send,
        <T::LspRequest as lsp::request::Request>::Result: Send,
    {
        let sender_id = envelope.original_sender_id()?;
        let buffer_id = T::buffer_id_from_proto(&envelope.payload)?;
        let buffer_handle = this.update(&mut cx, |this, _cx| {
            this.opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .ok_or_else(|| anyhow!("unknown buffer id {}", buffer_id))
        })??;
        let request = T::from_proto(
            envelope.payload,
            this.clone(),
            buffer_handle.clone(),
            cx.clone(),
        )
        .await?;
        let response = this
            .update(&mut cx, |this, cx| {
                this.request_lsp(
                    buffer_handle.clone(),
                    LanguageServerToQuery::Primary,
                    request,
                    cx,
                )
            })?
            .await?;
        this.update(&mut cx, |this, cx| {
            Ok(T::response_to_proto(
                response,
                this,
                sender_id,
                &buffer_handle.read(cx).version(),
                cx,
            ))
        })?
    }

    async fn handle_get_project_symbols(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::GetProjectSymbols>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::GetProjectSymbolsResponse> {
        let symbols = this
            .update(&mut cx, |this, cx| {
                this.symbols(&envelope.payload.query, cx)
            })?
            .await?;

        Ok(proto::GetProjectSymbolsResponse {
            symbols: symbols.iter().map(serialize_symbol).collect(),
        })
    }

    async fn handle_search_project(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SearchProject>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::SearchProjectResponse> {
        let peer_id = envelope.original_sender_id()?;
        let query = SearchQuery::from_proto(envelope.payload)?;
        let mut result = this.update(&mut cx, |this, cx| this.search(query, cx))?;

        cx.spawn(move |mut cx| async move {
            let mut locations = Vec::new();
            while let Some((buffer, ranges)) = result.next().await {
                for range in ranges {
                    let start = serialize_anchor(&range.start);
                    let end = serialize_anchor(&range.end);
                    let buffer_id = this.update(&mut cx, |this, cx| {
                        this.create_buffer_for_peer(&buffer, peer_id, cx).into()
                    })?;
                    locations.push(proto::Location {
                        buffer_id,
                        start: Some(start),
                        end: Some(end),
                    });
                }
            }
            Ok(proto::SearchProjectResponse { locations })
        })
        .await
    }

    async fn handle_open_buffer_for_symbol(
        this: Model<Self>,
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
            .update(&mut cx, |this, _| this.deserialize_symbol(symbol))?
            .await?;
        let symbol = this.update(&mut cx, |this, _| {
            let signature = this.symbol_signature(&symbol.path);
            if signature == symbol.signature {
                Ok(symbol)
            } else {
                Err(anyhow!("invalid symbol signature"))
            }
        })??;
        let buffer = this
            .update(&mut cx, |this, cx| this.open_buffer_for_symbol(&symbol, cx))?
            .await?;

        this.update(&mut cx, |this, cx| {
            let is_private = buffer
                .read(cx)
                .file()
                .map(|f| f.is_private())
                .unwrap_or_default();
            if is_private {
                Err(anyhow!(ErrorCode::UnsharedItem))
            } else {
                Ok(proto::OpenBufferForSymbolResponse {
                    buffer_id: this.create_buffer_for_peer(&buffer, peer_id, cx).into(),
                })
            }
        })?
    }

    fn symbol_signature(&self, project_path: &ProjectPath) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(project_path.worktree_id.to_proto().to_be_bytes());
        hasher.update(project_path.path.to_string_lossy().as_bytes());
        hasher.update(self.nonce.to_be_bytes());
        hasher.finalize().as_slice().try_into().unwrap()
    }

    async fn handle_open_buffer_by_id(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::OpenBufferById>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferResponse> {
        let peer_id = envelope.original_sender_id()?;
        let buffer_id = BufferId::new(envelope.payload.id)?;
        let buffer = this
            .update(&mut cx, |this, cx| this.open_buffer_by_id(buffer_id, cx))?
            .await?;
        Project::respond_to_open_buffer_request(this, buffer, peer_id, &mut cx)
    }

    async fn handle_open_buffer_by_path(
        this: Model<Self>,
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
        })?;

        let buffer = open_buffer.await?;
        Project::respond_to_open_buffer_request(this, buffer, peer_id, &mut cx)
    }

    fn respond_to_open_buffer_request(
        this: Model<Self>,
        buffer: Model<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut AsyncAppContext,
    ) -> Result<proto::OpenBufferResponse> {
        this.update(cx, |this, cx| {
            let is_private = buffer
                .read(cx)
                .file()
                .map(|f| f.is_private())
                .unwrap_or_default();
            if is_private {
                Err(anyhow!(ErrorCode::UnsharedItem))
            } else {
                Ok(proto::OpenBufferResponse {
                    buffer_id: this.create_buffer_for_peer(&buffer, peer_id, cx).into(),
                })
            }
        })?
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
                .push(self.create_buffer_for_peer(&buffer, peer_id, cx).into());
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
        cx.spawn(move |this, mut cx| async move {
            let mut project_transaction = ProjectTransaction::default();
            for (buffer_id, transaction) in message.buffer_ids.into_iter().zip(message.transactions)
            {
                let buffer_id = BufferId::new(buffer_id)?;
                let buffer = this
                    .update(&mut cx, |this, cx| {
                        this.wait_for_remote_buffer(buffer_id, cx)
                    })?
                    .await?;
                let transaction = language::proto::deserialize_transaction(transaction)?;
                project_transaction.0.insert(buffer, transaction);
            }

            for (buffer, transaction) in &project_transaction.0 {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_edits(transaction.edit_ids.iter().copied())
                    })?
                    .await?;

                if push_to_history {
                    buffer.update(&mut cx, |buffer, _| {
                        buffer.push_transaction(transaction.clone(), Instant::now());
                    })?;
                }
            }

            Ok(project_transaction)
        })
    }

    fn create_buffer_for_peer(
        &mut self,
        buffer: &Model<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut AppContext,
    ) -> BufferId {
        let buffer_id = buffer.read(cx).remote_id();
        if let ProjectClientState::Shared { updates_tx, .. } = &self.client_state {
            updates_tx
                .unbounded_send(LocalProjectUpdate::CreateBufferForPeer { peer_id, buffer_id })
                .ok();
        }
        buffer_id
    }

    fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        let mut opened_buffer_rx = self.opened_buffer.1.clone();

        cx.spawn(move |this, mut cx| async move {
            let buffer = loop {
                let Some(this) = this.upgrade() else {
                    return Err(anyhow!("project dropped"));
                };

                let buffer = this.update(&mut cx, |this, _cx| {
                    this.opened_buffers
                        .get(&id)
                        .and_then(|buffer| buffer.upgrade())
                })?;

                if let Some(buffer) = buffer {
                    break buffer;
                } else if this.update(&mut cx, |this, _| this.is_disconnected())? {
                    return Err(anyhow!("disconnected before buffer {} could be opened", id));
                }

                this.update(&mut cx, |this, _| {
                    this.incomplete_remote_buffers.entry(id).or_default();
                })?;
                drop(this);

                opened_buffer_rx
                    .next()
                    .await
                    .ok_or_else(|| anyhow!("project dropped while waiting for buffer"))?;
            };

            Ok(buffer)
        })
    }

    fn synchronize_remote_buffers(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let project_id = match self.client_state {
            ProjectClientState::Remote {
                sharing_has_stopped,
                remote_id,
                ..
            } => {
                if sharing_has_stopped {
                    return Task::ready(Err(anyhow!(
                        "can't synchronize remote buffers on a readonly project"
                    )));
                } else {
                    remote_id
                }
            }
            ProjectClientState::Shared { .. } | ProjectClientState::Local => {
                return Task::ready(Err(anyhow!(
                    "can't synchronize remote buffers on a local project"
                )))
            }
        };

        let client = self.client.clone();
        cx.spawn(move |this, mut cx| async move {
            let (buffers, incomplete_buffer_ids) = this.update(&mut cx, |this, cx| {
                let buffers = this
                    .opened_buffers
                    .iter()
                    .filter_map(|(id, buffer)| {
                        let buffer = buffer.upgrade()?;
                        Some(proto::BufferVersion {
                            id: (*id).into(),
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
            })?;
            let response = client
                .request(proto::SynchronizeBuffers {
                    project_id,
                    buffers,
                })
                .await?;

            let send_updates_for_buffers = this.update(&mut cx, |this, cx| {
                response
                    .buffers
                    .into_iter()
                    .map(|buffer| {
                        let client = client.clone();
                        let buffer_id = match BufferId::new(buffer.id) {
                            Ok(id) => id,
                            Err(e) => {
                                return Task::ready(Err(e));
                            }
                        };
                        let remote_version = language::proto::deserialize_version(&buffer.version);
                        if let Some(buffer) = this.buffer_for_id(buffer_id) {
                            let operations =
                                buffer.read(cx).serialize_ops(Some(remote_version), cx);
                            cx.background_executor().spawn(async move {
                                let operations = operations.await;
                                for chunk in split_operations(operations) {
                                    client
                                        .request(proto::UpdateBuffer {
                                            project_id,
                                            buffer_id: buffer_id.into(),
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
                    .collect::<Vec<_>>()
            })?;

            // Any incomplete buffers have open requests waiting. Request that the host sends
            // creates these buffers for us again to unblock any waiting futures.
            for id in incomplete_buffer_ids {
                cx.background_executor()
                    .spawn(client.request(proto::OpenBufferById {
                        project_id,
                        id: id.into(),
                    }))
                    .detach();
            }

            futures::future::join_all(send_updates_for_buffers)
                .await
                .into_iter()
                .collect()
        })
    }

    pub fn worktree_metadata_protos(&self, cx: &AppContext) -> Vec<proto::WorktreeMetadata> {
        self.worktrees()
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
                let worktree = worktree.upgrade()?;
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
        for id in old_worktrees_by_id.keys() {
            cx.emit(Event::WorktreeRemoved(*id));
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
            let adapter = language
                .as_ref()
                .and_then(|language| languages.lsp_adapters(language).first().cloned());
            Ok(Symbol {
                language_server_name: LanguageServerName(
                    serialized_symbol.language_server_name.into(),
                ),
                source_worktree_id,
                path,
                label: {
                    match language.as_ref().zip(adapter.as_ref()) {
                        Some((language, adapter)) => {
                            adapter
                                .label_for_symbol(&serialized_symbol.name, kind, language)
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
        this: Model<Self>,
        envelope: TypedEnvelope<proto::BufferSaved>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let fingerprint = deserialize_fingerprint(&envelope.payload.fingerprint)?;
        let version = deserialize_version(&envelope.payload.version);
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let mtime = envelope
            .payload
            .mtime
            .ok_or_else(|| anyhow!("missing mtime"))?
            .into();

        this.update(&mut cx, |this, cx| {
            let buffer = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .or_else(|| {
                    this.incomplete_remote_buffers
                        .get(&buffer_id)
                        .and_then(|b| b.clone())
                });
            if let Some(buffer) = buffer {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_save(version, fingerprint, mtime, cx);
                });
            }
            Ok(())
        })?
    }

    async fn handle_buffer_reloaded(
        this: Model<Self>,
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
        let buffer_id = BufferId::new(payload.buffer_id)?;
        this.update(&mut cx, |this, cx| {
            let buffer = this
                .opened_buffers
                .get(&buffer_id)
                .and_then(|buffer| buffer.upgrade())
                .or_else(|| {
                    this.incomplete_remote_buffers
                        .get(&buffer_id)
                        .cloned()
                        .flatten()
                });
            if let Some(buffer) = buffer {
                buffer.update(cx, |buffer, cx| {
                    buffer.did_reload(version, fingerprint, line_ending, mtime, cx);
                });
            }
            Ok(())
        })?
    }

    #[allow(clippy::type_complexity)]
    fn edits_from_lsp(
        &mut self,
        buffer: &Model<Buffer>,
        lsp_edits: impl 'static + Send + IntoIterator<Item = lsp::TextEdit>,
        server_id: LanguageServerId,
        version: Option<i32>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<(Range<Anchor>, String)>>> {
        let snapshot = self.buffer_snapshot_for_lsp_version(buffer, server_id, version, cx);
        cx.background_executor().spawn(async move {
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
        buffer: &Model<Buffer>,
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

    pub fn language_servers(
        &self,
    ) -> impl '_ + Iterator<Item = (LanguageServerId, LanguageServerName, WorktreeId)> {
        self.language_server_ids
            .iter()
            .map(|((worktree_id, server_name), server_id)| {
                (*server_id, server_name.clone(), *worktree_id)
            })
    }

    pub fn supplementary_language_servers(
        &self,
    ) -> impl '_
           + Iterator<
        Item = (
            &LanguageServerId,
            &(LanguageServerName, Arc<LanguageServer>),
        ),
    > {
        self.supplementary_language_servers.iter()
    }

    pub fn language_server_adapter_for_id(
        &self,
        id: LanguageServerId,
    ) -> Option<Arc<CachedLspAdapter>> {
        if let Some(LanguageServerState::Running { adapter, .. }) = self.language_servers.get(&id) {
            Some(adapter.clone())
        } else {
            None
        }
    }

    pub fn language_server_for_id(&self, id: LanguageServerId) -> Option<Arc<LanguageServer>> {
        if let Some(LanguageServerState::Running { server, .. }) = self.language_servers.get(&id) {
            Some(server.clone())
        } else if let Some((_, server)) = self.supplementary_language_servers.get(&id) {
            Some(Arc::clone(server))
        } else {
            None
        }
    }

    pub fn language_servers_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> impl Iterator<Item = (&Arc<CachedLspAdapter>, &Arc<LanguageServer>)> {
        self.language_server_ids_for_buffer(buffer, cx)
            .into_iter()
            .filter_map(|server_id| match self.language_servers.get(&server_id)? {
                LanguageServerState::Running {
                    adapter, server, ..
                } => Some((adapter, server)),
                _ => None,
            })
    }

    fn primary_language_server_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Option<(&Arc<CachedLspAdapter>, &Arc<LanguageServer>)> {
        self.language_servers_for_buffer(buffer, cx).next()
    }

    pub fn language_server_for_buffer(
        &self,
        buffer: &Buffer,
        server_id: LanguageServerId,
        cx: &AppContext,
    ) -> Option<(&Arc<CachedLspAdapter>, &Arc<LanguageServer>)> {
        self.language_servers_for_buffer(buffer, cx)
            .find(|(_, s)| s.server_id() == server_id)
    }

    fn language_server_ids_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Vec<LanguageServerId> {
        if let Some((file, language)) = File::from_dyn(buffer.file()).zip(buffer.language()) {
            let worktree_id = file.worktree_id(cx);
            self.languages
                .lsp_adapters(&language)
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

fn subscribe_for_copilot_events(
    copilot: &Model<Copilot>,
    cx: &mut ModelContext<'_, Project>,
) -> gpui::Subscription {
    cx.subscribe(
        copilot,
        |project, copilot, copilot_event, cx| match copilot_event {
            copilot::Event::CopilotLanguageServerStarted => {
                match copilot.read(cx).language_server() {
                    Some((name, copilot_server)) => {
                        // Another event wants to re-add the server that was already added and subscribed to, avoid doing it again.
                        if !copilot_server.has_notification_handler::<copilot::request::LogMessage>() {
                            let new_server_id = copilot_server.server_id();
                            let weak_project = cx.weak_model();
                            let copilot_log_subscription = copilot_server
                                .on_notification::<copilot::request::LogMessage, _>(
                                    move |params, mut cx| {
                                        weak_project.update(&mut cx, |_, cx| {
                                            cx.emit(Event::LanguageServerLog(
                                                new_server_id,
                                                params.message,
                                            ));
                                        }).ok();
                                    },
                                );
                            project.supplementary_language_servers.insert(new_server_id, (name.clone(), Arc::clone(copilot_server)));
                            project.copilot_log_subscription = Some(copilot_log_subscription);
                            cx.emit(Event::LanguageServerAdded(new_server_id));
                        }
                    }
                    None => debug_panic!("Received Copilot language server started event, but no language server is running"),
                }
            }
        },
    )
}

fn glob_literal_prefix(glob: &str) -> &str {
    let mut literal_end = 0;
    for (i, part) in glob.split(path::MAIN_SEPARATOR).enumerate() {
        if part.contains(&['*', '?', '{', '}']) {
            break;
        } else {
            if i > 0 {
                // Account for separator prior to this part
                literal_end += path::MAIN_SEPARATOR.len_utf8();
            }
            literal_end += part.len();
        }
    }
    &glob[..literal_end]
}

impl WorktreeHandle {
    pub fn upgrade(&self) -> Option<Model<Worktree>> {
        match self {
            WorktreeHandle::Strong(handle) => Some(handle.clone()),
            WorktreeHandle::Weak(handle) => handle.upgrade(),
        }
    }

    pub fn handle_id(&self) -> usize {
        match self {
            WorktreeHandle::Strong(handle) => handle.entity_id().as_u64() as usize,
            WorktreeHandle::Weak(handle) => handle.entity_id().as_u64() as usize,
        }
    }
}

impl OpenBuffer {
    pub fn upgrade(&self) -> Option<Model<Buffer>> {
        match self {
            OpenBuffer::Strong(handle) => Some(handle.clone()),
            OpenBuffer::Weak(handle) => handle.upgrade(),
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

impl EventEmitter<Event> for Project {}

impl<P: AsRef<Path>> From<(WorktreeId, P)> for ProjectPath {
    fn from((worktree_id, path): (WorktreeId, P)) -> Self {
        Self {
            worktree_id,
            path: path.as_ref().into(),
        }
    }
}

struct ProjectLspAdapterDelegate {
    project: Model<Project>,
    worktree: worktree::Snapshot,
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
}

impl ProjectLspAdapterDelegate {
    fn new(project: &Project, worktree: &Model<Worktree>, cx: &ModelContext<Project>) -> Arc<Self> {
        Arc::new(Self {
            project: cx.handle(),
            worktree: worktree.read(cx).snapshot(),
            fs: project.fs.clone(),
            http_client: project.client.http_client(),
            language_registry: project.languages.clone(),
        })
    }
}

#[async_trait]
impl LspAdapterDelegate for ProjectLspAdapterDelegate {
    fn show_notification(&self, message: &str, cx: &mut AppContext) {
        self.project
            .update(cx, |_, cx| cx.emit(Event::Notification(message.to_owned())));
    }

    fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    async fn which_command(&self, command: OsString) -> Option<(PathBuf, HashMap<String, String>)> {
        let worktree_abs_path = self.worktree.abs_path();

        let shell_env = load_shell_environment(&worktree_abs_path)
            .await
            .with_context(|| {
                format!("failed to determine load login shell environment in {worktree_abs_path:?}")
            })
            .log_err();

        if let Some(shell_env) = shell_env {
            let shell_path = shell_env.get("PATH");
            match which::which_in(&command, shell_path, &worktree_abs_path) {
                Ok(command_path) => Some((command_path, shell_env)),
                Err(error) => {
                    log::warn!(
                        "failed to determine path for command {:?} in shell PATH {:?}: {error}",
                        command.to_string_lossy(),
                        shell_path.map(String::as_str).unwrap_or("")
                    );
                    None
                }
            }
        } else {
            None
        }
    }

    fn update_status(
        &self,
        server_name: LanguageServerName,
        status: language::LanguageServerBinaryStatus,
    ) {
        self.language_registry
            .update_lsp_status(server_name, status);
    }

    async fn read_text_file(&self, path: PathBuf) -> Result<String> {
        if self.worktree.entry_for_path(&path).is_none() {
            return Err(anyhow!("no such path {path:?}"));
        }
        let path = self.worktree.absolutize(path.as_ref())?;
        let content = self.fs.load(&path).await?;
        Ok(content)
    }
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
            (Some(a), Some(Component::CurDir)) => components.push(a),
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

fn resolve_path(base: &Path, path: &Path) -> PathBuf {
    let mut result = base.to_path_buf();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => (),
            _ => result.push(component),
        }
    }
    result
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

async fn wait_for_loading_buffer(
    mut receiver: postage::watch::Receiver<Option<Result<Model<Buffer>, Arc<anyhow::Error>>>>,
) -> Result<Model<Buffer>, Arc<anyhow::Error>> {
    loop {
        if let Some(result) = receiver.borrow().as_ref() {
            match result {
                Ok(buffer) => return Ok(buffer.to_owned()),
                Err(e) => return Err(e.to_owned()),
            }
        }
        receiver.next().await;
    }
}

fn include_text(server: &lsp::LanguageServer) -> bool {
    server
        .capabilities()
        .text_document_sync
        .as_ref()
        .and_then(|sync| match sync {
            lsp::TextDocumentSyncCapability::Kind(_) => None,
            lsp::TextDocumentSyncCapability::Options(options) => options.save.as_ref(),
        })
        .and_then(|save_options| match save_options {
            lsp::TextDocumentSyncSaveOptions::Supported(_) => None,
            lsp::TextDocumentSyncSaveOptions::SaveOptions(options) => options.include_text,
        })
        .unwrap_or(false)
}

async fn load_shell_environment(dir: &Path) -> Result<HashMap<String, String>> {
    let marker = "ZED_SHELL_START";
    let shell = env::var("SHELL").context(
        "SHELL environment variable is not assigned so we can't source login environment variables",
    )?;

    // What we're doing here is to spawn a shell and then `cd` into
    // the project directory to get the env in there as if the user
    // `cd`'d into it. We do that because tools like direnv, asdf, ...
    // hook into `cd` and only set up the env after that.
    //
    // In certain shells we need to execute additional_command in order to
    // trigger the behavior of direnv, etc.
    //
    //
    // The `exit 0` is the result of hours of debugging, trying to find out
    // why running this command here, without `exit 0`, would mess
    // up signal process for our process so that `ctrl-c` doesn't work
    // anymore.
    //
    // We still don't know why `$SHELL -l -i -c '/usr/bin/env -0'`  would
    // do that, but it does, and `exit 0` helps.
    let additional_command = PathBuf::from(&shell)
        .file_name()
        .and_then(|f| f.to_str())
        .and_then(|shell| match shell {
            "fish" => Some("emit fish_prompt;"),
            _ => None,
        });

    let command = format!(
        "cd {dir:?};{} echo {marker}; /usr/bin/env -0; exit 0;",
        additional_command.unwrap_or("")
    );

    let output = smol::process::Command::new(&shell)
        .args(["-i", "-c", &command])
        .output()
        .await
        .context("failed to spawn login shell to source login environment variables")?;

    anyhow::ensure!(
        output.status.success(),
        "login shell exited with error {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let env_output_start = stdout.find(marker).ok_or_else(|| {
        anyhow!(
            "failed to parse output of `env` command in login shell: {}",
            stdout
        )
    })?;

    let mut parsed_env = HashMap::default();
    let env_output = &stdout[env_output_start + marker.len()..];
    for line in env_output.split_terminator('\0') {
        if let Some(separator_index) = line.find('=') {
            let key = line[..separator_index].to_string();
            let value = line[separator_index + 1..].to_string();
            parsed_env.insert(key, value);
        }
    }
    Ok(parsed_env)
}
