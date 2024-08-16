pub mod buffer_store;
pub mod connection_manager;
pub mod debounced_delay;
pub mod lsp_command;
pub mod lsp_ext_command;
mod prettier_support;
pub mod project_settings;
pub mod search;
mod task_inventory;
pub mod terminals;
pub mod worktree_store;

#[cfg(test)]
mod project_tests;

pub mod search_history;
mod yarn;

use anyhow::{anyhow, bail, Context as _, Result};
use async_trait::async_trait;
use buffer_store::{BufferStore, BufferStoreEvent};
use client::{
    proto, Client, Collaborator, DevServerProjectId, PendingEntitySubscription, ProjectId,
    TypedEnvelope, UserStore,
};
use clock::ReplicaId;
use collections::{btree_map, BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use debounced_delay::DebouncedDelay;
use futures::{
    channel::mpsc::{self, UnboundedReceiver},
    future::{join_all, try_join_all, Shared},
    select,
    stream::FuturesUnordered,
    AsyncWriteExt, Future, FutureExt, StreamExt,
};

use git::{blame::Blame, repository::GitRepository};
use globset::{Glob, GlobSet, GlobSetBuilder};
use gpui::{
    AnyModel, AppContext, AsyncAppContext, BackgroundExecutor, BorrowAppContext, Context, Entity,
    EventEmitter, Model, ModelContext, PromptLevel, SharedString, Task, WeakModel, WindowContext,
};
use http_client::HttpClient;
use itertools::Itertools;
use language::{
    language_settings::{
        language_settings, AllLanguageSettings, FormatOnSave, Formatter, InlayHintKind,
        LanguageSettings, SelectedFormatter,
    },
    markdown, point_to_lsp, prepare_completion_documentation,
    proto::{
        deserialize_anchor, deserialize_version, serialize_anchor, serialize_line_ending,
        serialize_version, split_operations,
    },
    range_from_lsp, Bias, Buffer, BufferSnapshot, CachedLspAdapter, Capability, CodeLabel,
    ContextProvider, Diagnostic, DiagnosticEntry, DiagnosticSet, Diff, Documentation,
    Event as BufferEvent, File as _, Language, LanguageRegistry, LanguageServerName, LocalFile,
    LspAdapterDelegate, Patch, PendingLanguageServer, PointUtf16, TextBufferSnapshot, ToOffset,
    ToPointUtf16, Transaction, Unclipped,
};
use log::error;
use lsp::{
    CompletionContext, DiagnosticSeverity, DiagnosticTag, DidChangeWatchedFilesRegistrationOptions,
    DocumentHighlightKind, Edit, FileSystemWatcher, InsertTextFormat, LanguageServer,
    LanguageServerBinary, LanguageServerId, LspRequestFuture, MessageActionItem, OneOf,
    ServerHealthStatus, ServerStatus, TextEdit, WorkDoneProgressCancelParams,
};
use lsp_command::*;
use node_runtime::NodeRuntime;
use parking_lot::{Mutex, RwLock};
use paths::{
    local_settings_file_relative_path, local_tasks_file_relative_path,
    local_vscode_tasks_file_relative_path,
};
use postage::watch;
use prettier_support::{DefaultPrettier, PrettierInstance};
use project_settings::{DirenvSettings, LspSettings, ProjectSettings};
use rand::prelude::*;
use remote::SshSession;
use rpc::{proto::AddWorktree, ErrorCode};
use search::SearchQuery;
use search_history::SearchHistory;
use serde::Serialize;
use settings::{watch_config_file, Settings, SettingsLocation, SettingsStore};
use sha2::{Digest, Sha256};
use similar::{ChangeTag, TextDiff};
use smol::{
    channel::{Receiver, Sender},
    lock::Semaphore,
};
use snippet::Snippet;
use snippet_provider::SnippetProvider;
use std::{
    borrow::Cow,
    cell::RefCell,
    cmp::{self, Ordering},
    convert::TryInto,
    env,
    ffi::OsStr,
    hash::Hash,
    iter, mem,
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
use task::{
    static_source::{StaticSource, TrackedFile},
    HideStrategy, RevealStrategy, Shell, TaskContext, TaskTemplate, TaskVariables, VariableName,
};
use terminals::Terminals;
use text::{Anchor, BufferId, LineEnding};
use util::{
    debug_panic, defer, maybe, merge_json_value_into, parse_env_output, paths::compare_paths,
    post_inc, ResultExt, TryFutureExt as _,
};
use worktree::{CreatedEntry, Snapshot, Traversal};
use worktree_store::{WorktreeStore, WorktreeStoreEvent};
use yarn::YarnPathStore;

pub use fs::*;
pub use language::Location;
#[cfg(any(test, feature = "test-support"))]
pub use prettier::FORMAT_SUFFIX as TEST_PRETTIER_FORMAT_SUFFIX;
pub use task_inventory::{
    BasicContextProvider, ContextProviderWithTasks, Inventory, TaskSourceKind,
};
pub use worktree::{
    Entry, EntryKind, File, LocalWorktree, PathChange, ProjectEntryId, RepositoryEntry,
    UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree, WorktreeId, WorktreeSettings,
    FS_WATCH_LATENCY,
};

const MAX_SERVER_REINSTALL_ATTEMPT_COUNT: u64 = 4;
const SERVER_REINSTALL_DEBOUNCE_TIMEOUT: Duration = Duration::from_secs(1);
const SERVER_LAUNCHING_BEFORE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);
pub const SERVER_PROGRESS_THROTTLE_TIMEOUT: Duration = Duration::from_millis(100);

const MAX_PROJECT_SEARCH_HISTORY_SIZE: usize = 500;

pub trait Item {
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<Result<Model<Self>>>>
    where
        Self: Sized;
    fn entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId>;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
}

#[derive(Clone)]
pub enum OpenedBufferEvent {
    Disconnected,
    Ok(BufferId),
    Err(BufferId, Arc<anyhow::Error>),
}

/// Semantics-aware entity that is relevant to one or more [`Worktree`] with the files.
/// `Project` is responsible for tasks, LSP and collab queries, synchronizing worktree states accordingly.
/// Maps [`Worktree`] entries with its own logic using [`ProjectEntryId`] and [`ProjectPath`] structs.
///
/// Can be either local (for the project opened on the same host) or remote.(for collab projects, browsed by multiple remote users).
pub struct Project {
    active_entry: Option<ProjectEntryId>,
    buffer_ordered_messages_tx: mpsc::UnboundedSender<BufferOrderedMessage>,
    languages: Arc<LanguageRegistry>,
    supplementary_language_servers:
        HashMap<LanguageServerId, (LanguageServerName, Arc<LanguageServer>)>,
    language_servers: HashMap<LanguageServerId, LanguageServerState>,
    language_server_ids: HashMap<(WorktreeId, LanguageServerName), LanguageServerId>,
    language_server_statuses: BTreeMap<LanguageServerId, LanguageServerStatus>,
    last_formatting_failure: Option<String>,
    last_workspace_edits_by_language_server: HashMap<LanguageServerId, ProjectTransaction>,
    language_server_watched_paths: HashMap<LanguageServerId, HashMap<WorktreeId, GlobSet>>,
    language_server_watcher_registrations:
        HashMap<LanguageServerId, HashMap<String, Vec<FileSystemWatcher>>>,
    client: Arc<client::Client>,
    next_entry_id: Arc<AtomicUsize>,
    join_project_response_message_id: u32,
    next_diagnostic_group_id: usize,
    diagnostic_summaries:
        HashMap<WorktreeId, HashMap<Arc<Path>, HashMap<LanguageServerId, DiagnosticSummary>>>,
    diagnostics: HashMap<
        WorktreeId,
        HashMap<
            Arc<Path>,
            Vec<(
                LanguageServerId,
                Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
            )>,
        >,
    >,
    user_store: Model<UserStore>,
    fs: Arc<dyn Fs>,
    ssh_session: Option<Arc<SshSession>>,
    client_state: ProjectClientState,
    collaborators: HashMap<proto::PeerId, Collaborator>,
    client_subscriptions: Vec<client::Subscription>,
    worktree_store: Model<WorktreeStore>,
    buffer_store: Model<BufferStore>,
    _subscriptions: Vec<gpui::Subscription>,
    shared_buffers: HashMap<proto::PeerId, HashSet<BufferId>>,
    #[allow(clippy::type_complexity)]
    loading_worktrees:
        HashMap<Arc<Path>, Shared<Task<Result<Model<Worktree>, Arc<anyhow::Error>>>>>,
    buffer_snapshots: HashMap<BufferId, HashMap<LanguageServerId, Vec<LspBufferSnapshot>>>, // buffer_id -> server_id -> vec of snapshots
    buffers_being_formatted: HashSet<BufferId>,
    buffers_needing_diff: HashSet<WeakModel<Buffer>>,
    git_diff_debouncer: DebouncedDelay<Self>,
    nonce: u128,
    _maintain_buffer_languages: Task<()>,
    _maintain_workspace_config: Task<Result<()>>,
    terminals: Terminals,
    current_lsp_settings: HashMap<Arc<str>, LspSettings>,
    node: Option<Arc<dyn NodeRuntime>>,
    default_prettier: DefaultPrettier,
    prettiers_per_worktree: HashMap<WorktreeId, HashSet<Option<PathBuf>>>,
    prettier_instances: HashMap<PathBuf, PrettierInstance>,
    tasks: Model<Inventory>,
    hosted_project_id: Option<ProjectId>,
    dev_server_project_id: Option<client::DevServerProjectId>,
    search_history: SearchHistory,
    snippets: Model<SnippetProvider>,
    yarn: Model<YarnPathStore>,
    cached_shell_environments: HashMap<WorktreeId, HashMap<String, String>>,
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
#[derive(Debug)]
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

#[derive(Debug)]
enum LocalProjectUpdate {
    WorktreesChanged,
    CreateBufferForPeer {
        peer_id: proto::PeerId,
        buffer_id: BufferId,
    },
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
        in_room: bool,
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
    LanguageNotFound(Model<Buffer>),
    ActiveEntryChanged(Option<ProjectEntryId>),
    ActivateProjectPanel,
    WorktreeAdded,
    WorktreeOrderChanged,
    WorktreeRemoved(WorktreeId),
    WorktreeUpdatedEntries(WorktreeId, UpdatedEntriesSet),
    WorktreeUpdatedGitRepositories,
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
    HostReshared,
    Reshared,
    Rejoined,
    RefreshInlayHints,
    RevealInProjectPanel(ProjectEntryId),
    SnippetEdit(BufferId, Vec<(lsp::Range, Snippet)>),
}

pub enum LanguageServerState {
    Starting(Task<Option<Arc<LanguageServer>>>),

    Running {
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        server: Arc<LanguageServer>,
        simulate_disk_based_diagnostics_completion: Option<Task<()>>,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct LanguageServerStatus {
    pub name: String,
    pub pending_work: BTreeMap<String, LanguageServerProgress>,
    pub has_pending_diagnostic_updates: bool,
    progress_tokens: HashSet<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LanguageServerProgress {
    pub is_disk_based_diagnostics_progress: bool,
    pub is_cancellable: bool,
    pub title: Option<String>,
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

impl ProjectPath {
    pub fn from_proto(p: proto::ProjectPath) -> Self {
        Self {
            worktree_id: WorktreeId::from_proto(p.worktree_id),
            path: Arc::from(PathBuf::from(p.path)),
        }
    }

    pub fn to_proto(&self) -> proto::ProjectPath {
        proto::ProjectPath {
            worktree_id: self.worktree_id.to_proto(),
            path: self.path.to_string_lossy().to_string(),
        }
    }
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

/// The user's intent behind a given completion confirmation
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum CompletionIntent {
    /// The user intends to 'commit' this result, if possible
    /// completion confirmations should run side effects
    Complete,
    /// The user intends to continue 'composing' this completion
    /// completion confirmations should not run side effects and
    /// let the user continue composing their action
    Compose,
}

impl CompletionIntent {
    pub fn is_complete(&self) -> bool {
        self == &Self::Complete
    }

    pub fn is_compose(&self) -> bool {
        self == &Self::Compose
    }
}

/// A completion provided by a language server
#[derive(Clone)]
pub struct Completion {
    /// The range of the buffer that will be replaced.
    pub old_range: Range<Anchor>,
    /// The new text that will be inserted.
    pub new_text: String,
    /// A label for this completion that is shown in the menu.
    pub label: CodeLabel,
    /// The id of the language server that produced this completion.
    pub server_id: LanguageServerId,
    /// The documentation for this completion.
    pub documentation: Option<Documentation>,
    /// The raw completion provided by the language server.
    pub lsp_completion: lsp::CompletionItem,
    /// An optional callback to invoke when this completion is confirmed.
    /// Returns, whether new completions should be retriggered after the current one.
    /// If `true` is returned, the editor will show a new completion menu after this completion is confirmed.
    /// if no confirmation is provided or `false` is returned, the completion will be committed.
    pub confirm: Option<Arc<dyn Send + Sync + Fn(CompletionIntent, &mut WindowContext) -> bool>>,
}

impl std::fmt::Debug for Completion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Completion")
            .field("old_range", &self.old_range)
            .field("new_text", &self.new_text)
            .field("label", &self.label)
            .field("server_id", &self.server_id)
            .field("documentation", &self.documentation)
            .field("lsp_completion", &self.lsp_completion)
            .finish()
    }
}

/// A completion provided by a language server
#[derive(Clone, Debug)]
struct CoreCompletion {
    old_range: Range<Anchor>,
    new_text: String,
    server_id: LanguageServerId,
    lsp_completion: lsp::CompletionItem,
}

/// A code action provided by a language server.
#[derive(Clone, Debug)]
pub struct CodeAction {
    /// The id of the language server that produced this code action.
    pub server_id: LanguageServerId,
    /// The range of the buffer where this code action is applicable.
    pub range: Range<Anchor>,
    /// The raw code action provided by the language server.
    pub lsp_action: lsp::CodeAction,
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

#[derive(Clone, Debug)]
struct CoreSymbol {
    pub language_server_name: LanguageServerName,
    pub source_worktree_id: WorktreeId,
    pub path: ProjectPath,
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

#[derive(Debug, Clone)]
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
#[derive(Debug)]
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

#[derive(Clone)]
pub enum DirectoryLister {
    Project(Model<Project>),
    Local(Arc<dyn Fs>),
}

impl DirectoryLister {
    pub fn is_local(&self, cx: &AppContext) -> bool {
        match self {
            DirectoryLister::Local(_) => true,
            DirectoryLister::Project(project) => project.read(cx).is_local(),
        }
    }

    pub fn default_query(&self, cx: &mut AppContext) -> String {
        if let DirectoryLister::Project(project) = self {
            if let Some(worktree) = project.read(cx).visible_worktrees(cx).next() {
                return worktree.read(cx).abs_path().to_string_lossy().to_string();
            }
        };
        "~/".to_string()
    }
    pub fn list_directory(&self, query: String, cx: &mut AppContext) -> Task<Result<Vec<PathBuf>>> {
        match self {
            DirectoryLister::Project(project) => {
                project.update(cx, |project, cx| project.list_directory(query, cx))
            }
            DirectoryLister::Local(fs) => {
                let fs = fs.clone();
                cx.background_executor().spawn(async move {
                    let mut results = vec![];
                    let expanded = shellexpand::tilde(&query);
                    let query = Path::new(expanded.as_ref());
                    let mut response = fs.read_dir(query).await?;
                    while let Some(path) = response.next().await {
                        if let Some(file_name) = path?.file_name() {
                            results.push(PathBuf::from(file_name.to_os_string()));
                        }
                    }
                    Ok(results)
                })
            }
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
        is_file: bool,
        path: Arc<Path>,
    },
}

pub enum SearchResult {
    Buffer {
        buffer: Model<Buffer>,
        ranges: Vec<Range<Anchor>>,
    },
    LimitReached,
}

#[cfg(any(test, feature = "test-support"))]
pub const DEFAULT_COMPLETION_CONTEXT: CompletionContext = CompletionContext {
    trigger_kind: lsp::CompletionTriggerKind::INVOKED,
    trigger_character: None,
};

impl Project {
    pub fn init_settings(cx: &mut AppContext) {
        WorktreeSettings::register(cx);
        ProjectSettings::register(cx);
    }

    pub fn init(client: &Arc<Client>, cx: &mut AppContext) {
        connection_manager::init(client.clone(), cx);
        Self::init_settings(cx);

        client.add_model_message_handler(Self::handle_add_collaborator);
        client.add_model_message_handler(Self::handle_update_project_collaborator);
        client.add_model_message_handler(Self::handle_remove_collaborator);
        client.add_model_message_handler(Self::handle_start_language_server);
        client.add_model_message_handler(Self::handle_update_language_server);
        client.add_model_message_handler(Self::handle_update_project);
        client.add_model_message_handler(Self::handle_unshare_project);
        client.add_model_message_handler(Self::handle_create_buffer_for_peer);
        client.add_model_request_handler(Self::handle_update_buffer);
        client.add_model_message_handler(Self::handle_update_diagnostic_summary);
        client.add_model_message_handler(Self::handle_update_worktree);
        client.add_model_message_handler(Self::handle_update_worktree_settings);
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
        client.add_model_request_handler(Self::handle_lsp_command::<GetDeclaration>);
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
        client.add_model_request_handler(Self::handle_open_new_buffer);
        client.add_model_request_handler(Self::handle_lsp_command::<lsp_ext_command::ExpandMacro>);
        client.add_model_request_handler(Self::handle_multi_lsp_query);
        client.add_model_request_handler(Self::handle_restart_language_servers);
        client.add_model_request_handler(Self::handle_task_context_for_location);
        client.add_model_request_handler(Self::handle_task_templates);
        client.add_model_request_handler(Self::handle_lsp_command::<LinkedEditingRange>);

        client.add_model_request_handler(WorktreeStore::handle_create_project_entry);
        client.add_model_request_handler(WorktreeStore::handle_rename_project_entry);
        client.add_model_request_handler(WorktreeStore::handle_copy_project_entry);
        client.add_model_request_handler(WorktreeStore::handle_delete_project_entry);
        client.add_model_request_handler(WorktreeStore::handle_expand_project_entry);

        client.add_model_message_handler(BufferStore::handle_buffer_reloaded);
        client.add_model_message_handler(BufferStore::handle_buffer_saved);
        client.add_model_message_handler(BufferStore::handle_update_buffer_file);
        client.add_model_message_handler(BufferStore::handle_update_diff_base);
        client.add_model_request_handler(BufferStore::handle_save_buffer);
        client.add_model_request_handler(BufferStore::handle_blame_buffer);
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
            let tasks = Inventory::new(cx);
            let global_snippets_dir = paths::config_dir().join("snippets");
            let snippets =
                SnippetProvider::new(fs.clone(), BTreeSet::from_iter([global_snippets_dir]), cx);

            let worktree_store = cx.new_model(|_| WorktreeStore::new(false));
            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();

            let buffer_store =
                cx.new_model(|cx| BufferStore::new(worktree_store.clone(), None, cx));
            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();

            let yarn = YarnPathStore::new(fs.clone(), cx);

            Self {
                buffer_ordered_messages_tx: tx,
                collaborators: Default::default(),
                worktree_store,
                buffer_store,
                shared_buffers: Default::default(),
                loading_worktrees: Default::default(),
                buffer_snapshots: Default::default(),
                join_project_response_message_id: 0,
                client_state: ProjectClientState::Local,
                client_subscriptions: Vec::new(),
                _subscriptions: vec![
                    cx.observe_global::<SettingsStore>(Self::on_settings_changed),
                    cx.on_release(Self::release),
                    cx.on_app_quit(Self::shutdown_language_servers),
                ],
                _maintain_buffer_languages: Self::maintain_buffer_languages(languages.clone(), cx),
                _maintain_workspace_config: Self::maintain_workspace_config(cx),
                active_entry: None,
                yarn,
                snippets,
                languages,
                client,
                user_store,
                fs,
                ssh_session: None,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                diagnostics: Default::default(),
                diagnostic_summaries: Default::default(),
                supplementary_language_servers: HashMap::default(),
                language_servers: Default::default(),
                language_server_ids: HashMap::default(),
                language_server_statuses: Default::default(),
                last_formatting_failure: None,
                last_workspace_edits_by_language_server: Default::default(),
                language_server_watched_paths: HashMap::default(),
                language_server_watcher_registrations: HashMap::default(),
                buffers_being_formatted: Default::default(),
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                nonce: StdRng::from_entropy().gen(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                current_lsp_settings: ProjectSettings::get_global(cx).lsp.clone(),
                node: Some(node),
                default_prettier: DefaultPrettier::default(),
                prettiers_per_worktree: HashMap::default(),
                prettier_instances: HashMap::default(),
                tasks,
                hosted_project_id: None,
                dev_server_project_id: None,
                search_history: Self::new_search_history(),
                cached_shell_environments: HashMap::default(),
            }
        })
    }

    pub fn ssh(
        ssh: Arc<SshSession>,
        client: Arc<Client>,
        node: Arc<dyn NodeRuntime>,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        let this = Self::local(client, node, user_store, languages, fs, cx);
        this.update(cx, |this, cx| {
            let buffer_store = this.buffer_store.downgrade();

            ssh.add_message_handler(cx.weak_model(), Self::handle_update_worktree);
            ssh.add_message_handler(cx.weak_model(), Self::handle_create_buffer_for_peer);
            ssh.add_message_handler(buffer_store.clone(), BufferStore::handle_update_buffer_file);
            ssh.add_message_handler(buffer_store.clone(), BufferStore::handle_update_diff_base);

            this.ssh_session = Some(ssh);
        });
        this
    }

    pub async fn remote(
        remote_id: u64,
        client: Arc<Client>,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        let project =
            Self::in_room(remote_id, client, user_store, languages, fs, cx.clone()).await?;
        cx.update(|cx| {
            connection_manager::Manager::global(cx).update(cx, |manager, cx| {
                manager.maintain_project_connection(&project, cx)
            })
        })?;
        Ok(project)
    }

    pub async fn in_room(
        remote_id: u64,
        client: Arc<Client>,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        client.authenticate_and_connect(true, &cx).await?;

        let subscriptions = (
            client.subscribe_to_entity::<Self>(remote_id)?,
            client.subscribe_to_entity::<BufferStore>(remote_id)?,
        );
        let response = client
            .request_envelope(proto::JoinProject {
                project_id: remote_id,
            })
            .await?;
        Self::from_join_project_response(
            response,
            subscriptions,
            client,
            user_store,
            languages,
            fs,
            cx,
        )
        .await
    }

    async fn from_join_project_response(
        response: TypedEnvelope<proto::JoinProjectResponse>,
        subscription: (
            PendingEntitySubscription<Project>,
            PendingEntitySubscription<BufferStore>,
        ),
        client: Arc<Client>,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        mut cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        let remote_id = response.payload.project_id;
        let role = response.payload.role();

        let worktree_store = cx.new_model(|_| WorktreeStore::new(true))?;
        let buffer_store =
            cx.new_model(|cx| BufferStore::new(worktree_store.clone(), Some(remote_id), cx))?;

        let this = cx.new_model(|cx| {
            let replica_id = response.payload.replica_id as ReplicaId;
            let tasks = Inventory::new(cx);
            let global_snippets_dir = paths::config_dir().join("snippets");
            let snippets =
                SnippetProvider::new(fs.clone(), BTreeSet::from_iter([global_snippets_dir]), cx);
            let yarn = YarnPathStore::new(fs.clone(), cx);

            let mut worktrees = Vec::new();
            for worktree in response.payload.worktrees {
                let worktree =
                    Worktree::remote(remote_id, replica_id, worktree, client.clone().into(), cx);
                worktrees.push(worktree);
            }

            let (tx, rx) = mpsc::unbounded();
            cx.spawn(move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx))
                .detach();

            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();

            let mut this = Self {
                buffer_ordered_messages_tx: tx,
                buffer_store: buffer_store.clone(),
                worktree_store,
                shared_buffers: Default::default(),
                loading_worktrees: Default::default(),
                active_entry: None,
                collaborators: Default::default(),
                join_project_response_message_id: response.message_id,
                _maintain_buffer_languages: Self::maintain_buffer_languages(languages.clone(), cx),
                _maintain_workspace_config: Self::maintain_workspace_config(cx),
                languages,
                user_store: user_store.clone(),
                snippets,
                yarn,
                fs,
                ssh_session: None,
                next_entry_id: Default::default(),
                next_diagnostic_group_id: Default::default(),
                diagnostic_summaries: Default::default(),
                diagnostics: Default::default(),
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
                    in_room: response.payload.dev_server_project_id.is_none(),
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
                last_formatting_failure: None,
                last_workspace_edits_by_language_server: Default::default(),
                language_server_watched_paths: HashMap::default(),
                language_server_watcher_registrations: HashMap::default(),
                buffers_being_formatted: Default::default(),
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                buffer_snapshots: Default::default(),
                nonce: StdRng::from_entropy().gen(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                current_lsp_settings: ProjectSettings::get_global(cx).lsp.clone(),
                node: None,
                default_prettier: DefaultPrettier::default(),
                prettiers_per_worktree: HashMap::default(),
                prettier_instances: HashMap::default(),
                tasks,
                hosted_project_id: None,
                dev_server_project_id: response
                    .payload
                    .dev_server_project_id
                    .map(|dev_server_project_id| DevServerProjectId(dev_server_project_id)),
                search_history: Self::new_search_history(),
                cached_shell_environments: HashMap::default(),
            };
            this.set_role(role, cx);
            for worktree in worktrees {
                let _ = this.add_worktree(&worktree, cx);
            }
            this
        })?;

        let subscriptions = [
            subscription.0.set_model(&this, &mut cx),
            subscription.1.set_model(&buffer_store, &mut cx),
        ];

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
            this.client_subscriptions.extend(subscriptions);
            anyhow::Ok(())
        })??;

        Ok(this)
    }

    pub async fn hosted(
        remote_id: ProjectId,
        user_store: Model<UserStore>,
        client: Arc<Client>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        client.authenticate_and_connect(true, &cx).await?;

        let subscriptions = (
            client.subscribe_to_entity::<Self>(remote_id.0)?,
            client.subscribe_to_entity::<BufferStore>(remote_id.0)?,
        );
        let response = client
            .request_envelope(proto::JoinHostedProject {
                project_id: remote_id.0,
            })
            .await?;
        Self::from_join_project_response(
            response,
            subscriptions,
            client,
            user_store,
            languages,
            fs,
            cx,
        )
        .await
    }

    fn new_search_history() -> SearchHistory {
        SearchHistory::new(
            Some(MAX_PROJECT_SEARCH_HISTORY_SIZE),
            search_history::QueryInsertionBehavior::AlwaysInsert,
        )
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
    pub async fn example(
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut AsyncAppContext,
    ) -> Model<Project> {
        use clock::FakeSystemClock;

        let fs = Arc::new(RealFs::default());
        let languages = LanguageRegistry::test(cx.background_executor().clone());
        let clock = Arc::new(FakeSystemClock::default());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let client = cx
            .update(|cx| client::Client::new(clock, http_client.clone(), cx))
            .unwrap();
        let user_store = cx
            .new_model(|cx| UserStore::new(client.clone(), cx))
            .unwrap();
        let project = cx
            .update(|cx| {
                Project::local(
                    client,
                    node_runtime::FakeNodeRuntime::new(),
                    user_store,
                    Arc::new(languages),
                    fs,
                    cx,
                )
            })
            .unwrap();
        for path in root_paths {
            let (tree, _) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(path, true, cx)
                })
                .unwrap()
                .await
                .unwrap();
            tree.update(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .unwrap()
                .await;
        }
        project
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn test(
        fs: Arc<dyn Fs>,
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut gpui::TestAppContext,
    ) -> Model<Project> {
        use clock::FakeSystemClock;

        let languages = LanguageRegistry::test(cx.executor());
        let clock = Arc::new(FakeSystemClock::default());
        let http_client = http_client::FakeHttpClient::with_404_response();
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
                    project.find_or_create_worktree(path, true, cx)
                })
                .await
                .unwrap();

            project.update(cx, |project, cx| {
                let tree_id = tree.read(cx).id();
                // In tests we always populate the environment to be empty so we don't run the shell
                project
                    .cached_shell_environments
                    .insert(tree_id, HashMap::default());
            });

            tree.update(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .await;
        }
        project
    }

    fn on_settings_changed(&mut self, cx: &mut ModelContext<Self>) {
        let mut language_servers_to_start = Vec::new();
        let mut language_formatters_to_check = Vec::new();
        for buffer in self.buffer_store.read(cx).buffers() {
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
                language_formatters_to_check
                    .push((buffer_file.map(|f| f.worktree_id(cx)), settings.clone()));
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
        for (worktree, language_settings) in language_formatters_to_check {
            if let Some(plugins) =
                prettier_support::prettier_plugins_for_language(&language_settings)
            {
                prettier_plugins_by_worktree
                    .entry(worktree)
                    .or_insert_with(|| HashSet::default())
                    .extend(plugins.iter().cloned());
            }
        }
        for (worktree, prettier_plugins) in prettier_plugins_by_worktree {
            self.install_default_prettier(
                worktree,
                prettier_plugins.into_iter().map(Arc::from),
                cx,
            );
        }

        // Start all the newly-enabled language servers.
        for (worktree, language) in language_servers_to_start {
            self.start_language_servers(&worktree, language, cx);
        }

        // Restart all language servers with changed initialization options.
        for (worktree, language) in language_servers_to_restart {
            self.restart_language_servers(worktree, language, cx);
        }

        cx.notify();
    }

    pub fn buffer_for_id(&self, remote_id: BufferId, cx: &AppContext) -> Option<Model<Buffer>> {
        self.buffer_store.read(cx).get(remote_id)
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

    pub fn node_runtime(&self) -> Option<&Arc<dyn NodeRuntime>> {
        self.node.as_ref()
    }

    pub fn opened_buffers(&self, cx: &AppContext) -> Vec<Model<Buffer>> {
        self.buffer_store.read(cx).buffers().collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn has_open_buffer(&self, path: impl Into<ProjectPath>, cx: &AppContext) -> bool {
        self.buffer_store
            .read(cx)
            .get_by_path(&path.into(), cx)
            .is_some()
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

    pub fn hosted_project_id(&self) -> Option<ProjectId> {
        self.hosted_project_id
    }

    pub fn dev_server_project_id(&self) -> Option<DevServerProjectId> {
        self.dev_server_project_id
    }

    pub fn supports_remote_terminal(&self, cx: &AppContext) -> bool {
        let Some(id) = self.dev_server_project_id else {
            return false;
        };
        let Some(server) = dev_server_projects::Store::global(cx)
            .read(cx)
            .dev_server_for_project(id)
        else {
            return false;
        };
        server.ssh_connection_string.is_some()
    }

    pub fn ssh_connection_string(&self, cx: &ModelContext<Self>) -> Option<SharedString> {
        if self.is_local() {
            return None;
        }

        let dev_server_id = self.dev_server_project_id()?;
        dev_server_projects::Store::global(cx)
            .read(cx)
            .dev_server_for_project(dev_server_id)?
            .ssh_connection_string
            .clone()
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

    pub fn snippets(&self) -> &Model<SnippetProvider> {
        &self.snippets
    }

    pub fn search_history(&self) -> &SearchHistory {
        &self.search_history
    }

    pub fn search_history_mut(&mut self) -> &mut SearchHistory {
        &mut self.search_history
    }

    pub fn collaborators(&self) -> &HashMap<proto::PeerId, Collaborator> {
        &self.collaborators
    }

    pub fn host(&self) -> Option<&Collaborator> {
        self.collaborators.values().find(|c| c.replica_id == 0)
    }

    pub fn set_worktrees_reordered(&mut self, worktrees_reordered: bool, cx: &mut AppContext) {
        self.worktree_store.update(cx, |store, _| {
            store.set_worktrees_reordered(worktrees_reordered);
        });
    }

    /// Collect all worktrees, including ones that don't appear in the project panel
    pub fn worktrees<'a>(
        &self,
        cx: &'a AppContext,
    ) -> impl 'a + DoubleEndedIterator<Item = Model<Worktree>> {
        self.worktree_store.read(cx).worktrees()
    }

    /// Collect all user-visible worktrees, the ones that appear in the project panel.
    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + DoubleEndedIterator<Item = Model<Worktree>> {
        self.worktree_store.read(cx).visible_worktrees(cx)
    }

    pub fn worktree_root_names<'a>(&'a self, cx: &'a AppContext) -> impl Iterator<Item = &'a str> {
        self.visible_worktrees(cx)
            .map(|tree| tree.read(cx).root_name())
    }

    pub fn worktree_for_id(&self, id: WorktreeId, cx: &AppContext) -> Option<Model<Worktree>> {
        self.worktree_store.read(cx).worktree_for_id(id, cx)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<Model<Worktree>> {
        self.worktree_store
            .read(cx)
            .worktree_for_entry(entry_id, cx)
    }

    pub fn worktree_id_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &AppContext,
    ) -> Option<WorktreeId> {
        self.worktree_for_entry(entry_id, cx)
            .map(|worktree| worktree.read(cx).id())
    }

    /// Checks if the entry is the root of a worktree.
    pub fn entry_is_worktree_root(&self, entry_id: ProjectEntryId, cx: &AppContext) -> bool {
        self.worktree_for_entry(entry_id, cx)
            .map(|worktree| {
                worktree
                    .read(cx)
                    .root_entry()
                    .is_some_and(|e| e.id == entry_id)
            })
            .unwrap_or(false)
    }

    pub fn visibility_for_paths(&self, paths: &[PathBuf], cx: &AppContext) -> Option<bool> {
        paths
            .iter()
            .map(|path| self.visibility_for_path(path, cx))
            .max()
            .flatten()
    }

    pub fn visibility_for_path(&self, path: &Path, cx: &AppContext) -> Option<bool> {
        self.worktrees(cx)
            .filter_map(|worktree| {
                let worktree = worktree.read(cx);
                worktree
                    .as_local()?
                    .contains_abs_path(path)
                    .then(|| worktree.is_visible())
            })
            .max()
    }

    pub fn create_entry(
        &mut self,
        project_path: impl Into<ProjectPath>,
        is_directory: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<CreatedEntry>> {
        let project_path = project_path.into();
        let Some(worktree) = self.worktree_for_id(project_path.worktree_id, cx) else {
            return Task::ready(Err(anyhow!(format!(
                "No worktree for path {project_path:?}"
            ))));
        };
        worktree.update(cx, |worktree, cx| {
            worktree.create_entry(project_path.path, is_directory, cx)
        })
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
        worktree.update(cx, |worktree, cx| {
            worktree.copy_entry(entry_id, new_path, cx)
        })
    }

    pub fn rename_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<CreatedEntry>> {
        let Some(worktree) = self.worktree_for_entry(entry_id, cx) else {
            return Task::ready(Err(anyhow!(format!("No worktree for entry {entry_id:?}"))));
        };
        worktree.update(cx, |worktree, cx| {
            worktree.rename_entry(entry_id, new_path, cx)
        })
    }

    pub fn delete_entry(
        &mut self,
        entry_id: ProjectEntryId,
        trash: bool,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        worktree.update(cx, |worktree, cx| {
            worktree.delete_entry(entry_id, trash, cx)
        })
    }

    pub fn expand_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut ModelContext<Self>,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree_for_id(worktree_id, cx)?;
        worktree.update(cx, |worktree, cx| worktree.expand_entry(entry_id, cx))
    }

    pub fn shared(&mut self, project_id: u64, cx: &mut ModelContext<Self>) -> Result<()> {
        if !matches!(self.client_state, ProjectClientState::Local) {
            if let ProjectClientState::Remote { in_room, .. } = &mut self.client_state {
                if *in_room || self.dev_server_project_id.is_none() {
                    return Err(anyhow!("project was already shared"));
                } else {
                    *in_room = true;
                    return Ok(());
                }
            } else {
                return Err(anyhow!("project was already shared"));
            }
        }
        self.client_subscriptions.extend([
            self.client
                .subscribe_to_entity(project_id)?
                .set_model(&cx.handle(), &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_model(&self.worktree_store, &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_model(&self.buffer_store, &mut cx.to_async()),
        ]);

        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.set_remote_id(Some(project_id), cx)
        });
        self.worktree_store.update(cx, |store, cx| {
            store.set_shared(true, cx);
        });

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
        for worktree in self.worktrees(cx) {
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
                            let worktrees = this.update(&mut cx, |this, cx| {
                                this.worktrees(cx).collect::<Vec<_>>()
                            })?;

                            let update_project = this
                                .update(&mut cx, |this, cx| {
                                    this.client.request(proto::UpdateProject {
                                        project_id,
                                        worktrees: this.worktree_metadata_protos(cx),
                                    })
                                })?
                                .await;
                            if update_project.log_err().is_none() {
                                continue;
                            }

                            this.update(&mut cx, |this, cx| {
                                for worktree in worktrees {
                                    worktree.update(cx, |worktree, cx| {
                                        if let Some(summaries) =
                                            this.diagnostic_summaries.get(&worktree.id())
                                        {
                                            for (path, summaries) in summaries {
                                                for (&server_id, summary) in summaries {
                                                    this.client.send(
                                                        proto::UpdateDiagnosticSummary {
                                                            project_id,
                                                            worktree_id: worktree.id().to_proto(),
                                                            summary: Some(
                                                                summary.to_proto(server_id, path),
                                                            ),
                                                        },
                                                    )?;
                                                }
                                            }
                                        }

                                        worktree.observe_updates(project_id, cx, {
                                            let client = client.clone();
                                            move |update| {
                                                client.request(update).map(|result| result.is_ok())
                                            }
                                        });

                                        anyhow::Ok(())
                                    })?;
                                }
                                anyhow::Ok(())
                            })??;
                        }
                        LocalProjectUpdate::CreateBufferForPeer { peer_id, buffer_id } => {
                            let Some(buffer_store) = this.update(&mut cx, |this, _| {
                                if this
                                    .shared_buffers
                                    .entry(peer_id)
                                    .or_default()
                                    .insert(buffer_id)
                                {
                                    Some(this.buffer_store.clone())
                                } else {
                                    None
                                }
                            })?
                            else {
                                continue;
                            };
                            BufferStore::create_buffer_for_peer(
                                buffer_store,
                                peer_id,
                                buffer_id,
                                project_id,
                                client.clone().into(),
                                &mut cx,
                            )
                            .await?;
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
        cx.emit(Event::Reshared);
        Ok(())
    }

    pub fn rejoined(
        &mut self,
        message: proto::RejoinedProject,
        message_id: u32,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            self.worktree_store.update(cx, |worktree_store, cx| {
                for worktree in worktree_store.worktrees() {
                    store
                        .clear_local_settings(worktree.entity_id().as_u64() as usize, cx)
                        .log_err();
                }
            });
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
        self.enqueue_buffer_ordered_message(BufferOrderedMessage::Resync)
            .unwrap();
        cx.emit(Event::Rejoined);
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
            if self.dev_server_project_id().is_some() {
                if let ProjectClientState::Remote { in_room, .. } = &mut self.client_state {
                    *in_room = false
                }
                return Ok(());
            } else {
                return Err(anyhow!("attempted to unshare a remote project"));
            }
        }

        if let ProjectClientState::Shared { remote_id, .. } = self.client_state {
            self.client_state = ProjectClientState::Local;
            self.collaborators.clear();
            self.shared_buffers.clear();
            self.client_subscriptions.clear();
            self.worktree_store.update(cx, |store, cx| {
                store.set_shared(false, cx);
            });
            self.buffer_store
                .update(cx, |buffer_store, cx| buffer_store.set_remote_id(None, cx));
            self.client
                .send(proto::UnshareProject {
                    project_id: remote_id,
                })
                .ok();
            Ok(())
        } else {
            Err(anyhow!("attempted to unshare an unshared project"))
        }
    }

    pub fn disconnected_from_host(&mut self, cx: &mut ModelContext<Self>) {
        if self.is_disconnected() {
            return;
        }
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
            for buffer in self.opened_buffers(cx) {
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
            self.worktree_store.update(cx, |store, cx| {
                store.disconnected_from_host(cx);
            });
            self.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.disconnected_from_host(cx)
            });
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

    pub fn is_ssh(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => true,
            ProjectClientState::Remote { .. } => false,
        }
    }

    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }

    pub fn create_buffer(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<Model<Buffer>>> {
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.create_buffer(
                if self.is_remote() {
                    Some((self.client.clone().into(), self.remote_id().unwrap()))
                } else {
                    None
                },
                cx,
            )
        })
    }

    pub fn create_local_buffer(
        &mut self,
        text: &str,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Model<Buffer> {
        if self.is_remote() {
            panic!("called create_local_buffer on a remote project")
        }
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.create_local_buffer(text, language, cx)
        })
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
        if let Some((worktree, relative_path)) = self.find_worktree(abs_path.as_ref(), cx) {
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
        if self.is_remote() && self.is_disconnected() {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }

        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.open_buffer(path.into(), cx)
        })
    }

    pub fn open_local_buffer_via_lsp(
        &mut self,
        mut abs_path: lsp::Url,
        language_server_id: LanguageServerId,
        language_server_name: LanguageServerName,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        cx.spawn(move |this, mut cx| async move {
            // Escape percent-encoded string.
            let current_scheme = abs_path.scheme().to_owned();
            let _ = abs_path.set_scheme("file");

            let abs_path = abs_path
                .to_file_path()
                .map_err(|_| anyhow!("can't convert URI to path"))?;
            let p = abs_path.clone();
            let yarn_worktree = this
                .update(&mut cx, move |this, cx| {
                    this.yarn.update(cx, |_, cx| {
                        cx.spawn(|this, mut cx| async move {
                            let t = this
                                .update(&mut cx, |this, cx| {
                                    this.process_path(&p, &current_scheme, cx)
                                })
                                .ok()?;
                            t.await
                        })
                    })
                })?
                .await;
            let (worktree_root_target, known_relative_path) =
                if let Some((zip_root, relative_path)) = yarn_worktree {
                    (zip_root, Some(relative_path))
                } else {
                    (Arc::<Path>::from(abs_path.as_path()), None)
                };
            let (worktree, relative_path) = if let Some(result) = this
                .update(&mut cx, |this, cx| {
                    this.find_worktree(&worktree_root_target, cx)
                })? {
                let relative_path =
                    known_relative_path.unwrap_or_else(|| Arc::<Path>::from(result.1));
                (result.0, relative_path)
            } else {
                let worktree = this
                    .update(&mut cx, |this, cx| {
                        this.create_worktree(&worktree_root_target, false, cx)
                    })?
                    .await?;
                this.update(&mut cx, |this, cx| {
                    this.language_server_ids.insert(
                        (worktree.read(cx).id(), language_server_name),
                        language_server_id,
                    );
                })
                .ok();
                let worktree_root = worktree.update(&mut cx, |this, _| this.abs_path())?;
                let relative_path = if let Some(known_path) = known_relative_path {
                    known_path
                } else {
                    abs_path.strip_prefix(worktree_root)?.into()
                };
                (worktree, relative_path)
            };
            let project_path = ProjectPath {
                worktree_id: worktree.update(&mut cx, |worktree, _| worktree.id())?,
                path: relative_path,
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
        if let Some(buffer) = self.buffer_for_id(id, cx) {
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
        self.buffer_store
            .update(cx, |buffer_store, cx| buffer_store.save_buffer(buffer, cx))
    }

    pub fn save_buffer_as(
        &mut self,
        buffer: Model<Buffer>,
        path: ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.save_buffer_as(buffer.clone(), path, cx)
        })
    }

    pub fn get_open_buffer(
        &mut self,
        path: &ProjectPath,
        cx: &mut ModelContext<Self>,
    ) -> Option<Model<Buffer>> {
        self.buffer_store.read(cx).get_by_path(path, cx)
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

        cx.subscribe(buffer, |this, buffer, event, cx| {
            this.on_buffer_event(buffer, event, cx);
        })
        .detach();

        self.detect_language_for_buffer(buffer, cx);
        self.register_buffer_with_language_servers(buffer, cx);
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
            let Some(uri) = lsp::Url::from_file_path(&abs_path).log_err() else {
                return;
            };
            let initial_snapshot = buffer.text_snapshot();
            let language = buffer.language().cloned();
            let worktree_id = file.worktree_id(cx);

            if let Some(diagnostics) = self.diagnostics.get(&worktree_id) {
                for (server_id, diagnostics) in
                    diagnostics.get(file.path()).cloned().unwrap_or_default()
                {
                    self.update_buffer_diagnostics(buffer_handle, server_id, None, diagnostics, cx)
                        .log_err();
                }
            }

            if let Some(language) = language {
                for adapter in self.languages.lsp_adapters(&language) {
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
                                    adapter.language_id(&language),
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
        cx: &mut AppContext,
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

    fn on_buffer_store_event(
        &mut self,
        _: Model<BufferStore>,
        event: &BufferStoreEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            BufferStoreEvent::BufferAdded(buffer) => {
                self.register_buffer(buffer, cx).log_err();
            }
            BufferStoreEvent::BufferChangedFilePath { buffer, old_file } => {
                if let Some(old_file) = File::from_dyn(old_file.as_ref()) {
                    self.unregister_buffer_from_language_servers(&buffer, old_file, cx);
                }

                self.detect_language_for_buffer(&buffer, cx);
                self.register_buffer_with_language_servers(&buffer, cx);
            }
            BufferStoreEvent::MessageToReplicas(message) => {
                self.client.send_dynamic(message.as_ref().clone()).log_err();
            }
        }
    }

    fn on_worktree_store_event(
        &mut self,
        _: Model<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(_) => cx.emit(Event::WorktreeAdded),
            WorktreeStoreEvent::WorktreeRemoved(_, id) => cx.emit(Event::WorktreeRemoved(*id)),
            WorktreeStoreEvent::WorktreeOrderChanged => cx.emit(Event::WorktreeOrderChanged),
        }
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

        let buffer_id = buffer.read(cx).remote_id();
        match event {
            BufferEvent::Operation(operation) => {
                let operation = language::proto::serialize_operation(operation);

                if let Some(ssh) = &self.ssh_session {
                    ssh.send(proto::UpdateBuffer {
                        project_id: 0,
                        buffer_id: buffer_id.to_proto(),
                        operations: vec![operation.clone()],
                    })
                    .ok();
                }

                self.enqueue_buffer_ordered_message(BufferOrderedMessage::Operation {
                    buffer_id,
                    operation,
                })
                .ok();
            }

            BufferEvent::Reloaded => {
                if self.is_local() {
                    if let Some(project_id) = self.remote_id() {
                        let buffer = buffer.read(cx);
                        self.client
                            .send(proto::BufferReloaded {
                                project_id,
                                buffer_id: buffer.remote_id().to_proto(),
                                version: serialize_version(&buffer.version()),
                                mtime: buffer.saved_mtime().map(|t| t.into()),
                                line_ending: serialize_line_ending(buffer.line_ending()) as i32,
                            })
                            .log_err();
                    }
                }
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
                    if let Some(include_text) = include_text(server.as_ref()) {
                        let text = if include_text {
                            Some(buffer.read(cx).text())
                        } else {
                            None
                        };
                        server
                            .notify::<lsp::notification::DidSaveTextDocument>(
                                lsp::DidSaveTextDocumentParams {
                                    text_document: text_document.clone(),
                                    text,
                                },
                            )
                            .log_err();
                    }
                }

                for language_server_id in self.language_server_ids_for_buffer(buffer.read(cx), cx) {
                    self.simulate_disk_based_diagnostics_events_if_needed(language_server_id, cx);
                }
            }

            _ => {}
        }

        None
    }

    // After saving a buffer using a language server that doesn't provide a disk-based progress token,
    // kick off a timer that will reset every time the buffer is saved. If the timer eventually fires,
    // simulate disk-based diagnostics being finished so that other pieces of UI (e.g., project
    // diagnostics view, diagnostic status bar) can update. We don't emit an event right away because
    // the language server might take some time to publish diagnostics.
    fn simulate_disk_based_diagnostics_events_if_needed(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) {
        const DISK_BASED_DIAGNOSTICS_DEBOUNCE: Duration = Duration::from_secs(1);

        let Some(LanguageServerState::Running {
            simulate_disk_based_diagnostics_completion,
            adapter,
            ..
        }) = self.language_servers.get_mut(&language_server_id)
        else {
            return;
        };

        if adapter.disk_based_diagnostics_progress_token.is_some() {
            return;
        }

        let prev_task = simulate_disk_based_diagnostics_completion.replace(cx.spawn(
            move |this, mut cx| async move {
                cx.background_executor()
                    .timer(DISK_BASED_DIAGNOSTICS_DEBOUNCE)
                    .await;

                this.update(&mut cx, |this, cx| {
                    this.disk_based_diagnostics_finished(language_server_id, cx);

                    if let Some(LanguageServerState::Running {
                        simulate_disk_based_diagnostics_completion,
                        ..
                    }) = this.language_servers.get_mut(&language_server_id)
                    {
                        *simulate_disk_based_diagnostics_completion = None;
                    }
                })
                .ok();
            },
        ));

        if prev_task.is_none() {
            self.disk_based_diagnostics_started(language_server_id, cx);
        }
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
                if this.buffers_needing_diff.is_empty() {
                    // TODO: Would a `ModelContext<Project>.notify()` suffice here?
                    for buffer in buffers {
                        if let Some(buffer) = buffer.upgrade() {
                            buffer.update(cx, |_, cx| cx.notify());
                        }
                    }
                } else {
                    this.recalculate_buffer_diffs(cx).detach();
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
                                this.buffer_store.clone().update(cx, |buffer_store, cx| {
                                    for buffer in buffer_store.buffers() {
                                        if let Some(f) =
                                            File::from_dyn(buffer.read(cx).file()).cloned()
                                        {
                                            this.unregister_buffer_from_language_servers(
                                                &buffer, &f, cx,
                                            );
                                            buffer.update(cx, |buffer, cx| {
                                                buffer.set_language(None, cx)
                                            });
                                        }
                                    }
                                });
                            })
                            .ok();
                    }

                    project
                        .update(&mut cx, |project, cx| {
                            let mut plain_text_buffers = Vec::new();
                            let mut buffers_with_unknown_injections = Vec::new();
                            for handle in project.buffer_store.read(cx).buffers() {
                                let buffer = handle.read(cx);
                                if buffer.language().is_none()
                                    || buffer.language() == Some(&*language::PLAIN_TEXT)
                                {
                                    plain_text_buffers.push(handle);
                                } else if buffer.contains_unknown_injections() {
                                    buffers_with_unknown_injections.push(handle);
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
                let servers = this.update(&mut cx, |this, cx| {
                    this.language_server_ids
                        .iter()
                        .filter_map(|((worktree_id, _), server_id)| {
                            let worktree = this.worktree_for_id(*worktree_id, cx)?;
                            let state = this.language_servers.get(server_id)?;
                            let delegate = ProjectLspAdapterDelegate::new(this, &worktree, cx);
                            match state {
                                LanguageServerState::Starting(_) => None,
                                LanguageServerState::Running {
                                    adapter, server, ..
                                } => Some((
                                    adapter.adapter.clone(),
                                    server.clone(),
                                    delegate as Arc<dyn LspAdapterDelegate>,
                                )),
                            }
                        })
                        .collect::<Vec<_>>()
                })?;

                for (adapter, server, delegate) in servers {
                    let settings = adapter.workspace_configuration(&delegate, &mut cx).await?;

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
    ) {
        // If the buffer has a language, set it and start the language server if we haven't already.
        let buffer = buffer_handle.read(cx);
        let Some(file) = buffer.file() else {
            return;
        };
        let content = buffer.as_rope();
        let Some(new_language_result) = self
            .languages
            .language_for_file(file, Some(content), cx)
            .now_or_never()
        else {
            return;
        };

        match new_language_result {
            Err(e) => {
                if e.is::<language::LanguageNotFound>() {
                    cx.emit(Event::LanguageNotFound(buffer_handle.clone()))
                }
            }
            Ok(new_language) => {
                self.set_language_for_buffer(buffer_handle, new_language, cx);
            }
        };
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
        if let Some(prettier_plugins) = prettier_support::prettier_plugins_for_language(&settings) {
            self.install_default_prettier(
                worktree,
                prettier_plugins.iter().map(|s| Arc::from(s.as_str())),
                cx,
            );
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
        let (root_file, is_local) =
            worktree.update(cx, |tree, cx| (tree.root_file(cx), tree.is_local()));
        let settings = language_settings(Some(&language), root_file.map(|f| f as _).as_ref(), cx);
        if !settings.enable_language_server || !is_local {
            return;
        }

        let available_lsp_adapters = self.languages.clone().lsp_adapters(&language);
        let available_language_servers = available_lsp_adapters
            .iter()
            .map(|lsp_adapter| lsp_adapter.name.clone())
            .collect::<Vec<_>>();

        let desired_language_servers =
            settings.customized_language_servers(&available_language_servers);

        let mut enabled_lsp_adapters: Vec<Arc<CachedLspAdapter>> = Vec::new();
        for desired_language_server in desired_language_servers {
            if let Some(adapter) = available_lsp_adapters
                .iter()
                .find(|adapter| adapter.name == desired_language_server)
            {
                enabled_lsp_adapters.push(adapter.clone());
                continue;
            }

            if let Some(adapter) = self
                .languages
                .load_available_lsp_adapter(&desired_language_server)
            {
                self.languages()
                    .register_lsp_adapter(language.name(), adapter.adapter.clone());
                enabled_lsp_adapters.push(adapter);
                continue;
            }

            log::warn!(
                "no language server found matching '{}'",
                desired_language_server.0
            );
        }

        log::info!(
            "starting language servers for {language}: {adapters}",
            language = language.name(),
            adapters = enabled_lsp_adapters
                .iter()
                .map(|adapter| adapter.name.0.as_ref())
                .join(", ")
        );

        for adapter in &enabled_lsp_adapters {
            self.start_language_server(worktree, adapter.clone(), language.clone(), cx);
        }

        // After starting all the language servers, reorder them to reflect the desired order
        // based on the settings.
        //
        // This is done, in part, to ensure that language servers loaded at different points
        // (e.g., native vs extension) still end up in the right order at the end, rather than
        // it being based on which language server happened to be loaded in first.
        self.languages()
            .reorder_language_servers(&language, enabled_lsp_adapters);
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
        let lsp_adapter_delegate = ProjectLspAdapterDelegate::new(self, worktree_handle, cx);
        let pending_server = match self.languages.create_pending_language_server(
            stderr_capture.clone(),
            language.clone(),
            adapter.clone(),
            Arc::clone(&worktree_path),
            lsp_adapter_delegate.clone(),
            cx,
        ) {
            Some(pending_server) => pending_server,
            None => return,
        };

        let project_settings = ProjectSettings::get(
            Some(SettingsLocation {
                worktree_id: worktree_id.to_proto() as usize,
                path: Path::new(""),
            }),
            cx,
        );
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
                    lsp_adapter_delegate,
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

        self.worktree_store.update(cx, |store, cx| {
            for worktree in store.worktrees() {
                let key = (worktree.read(cx).id(), adapter.name.clone());
                self.language_server_ids.remove(&key);
            }
        });

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
                for worktree in this.worktree_store.read(cx).worktrees().collect::<Vec<_>>() {
                    this.start_language_server(&worktree, adapter.clone(), language.clone(), cx);
                }
            })
            .ok();
        }))
    }

    #[allow(clippy::too_many_arguments)]
    async fn setup_and_insert_language_server(
        this: WeakModel<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
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
            delegate,
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
        project: WeakModel<Self>,
        override_options: Option<serde_json::Value>,
        pending_server: PendingLanguageServer,
        delegate: Arc<dyn LspAdapterDelegate>,
        adapter: Arc<CachedLspAdapter>,
        server_id: LanguageServerId,
        cx: &mut AsyncAppContext,
    ) -> Result<Arc<LanguageServer>> {
        let workspace_config = adapter
            .adapter
            .clone()
            .workspace_configuration(&delegate, cx)
            .await?;
        let (language_server, mut initialization_options) = pending_server.task.await?;

        let name = language_server.name();
        language_server
            .on_notification::<lsp::notification::PublishDiagnostics, _>({
                let adapter = adapter.clone();
                let this = project.clone();
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
                let adapter = adapter.adapter.clone();
                let delegate = delegate.clone();
                move |params, mut cx| {
                    let adapter = adapter.clone();
                    let delegate = delegate.clone();
                    async move {
                        let workspace_config =
                            adapter.workspace_configuration(&delegate, &mut cx).await?;
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
                let this = project.clone();
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
                let project = project.clone();
                move |params, mut cx| {
                    let project = project.clone();
                    async move {
                        for reg in params.registrations {
                            match reg.method.as_str() {
                                "workspace/didChangeWatchedFiles" => {
                                    if let Some(options) = reg.register_options {
                                        let options = serde_json::from_value(options)?;
                                        project.update(&mut cx, |project, cx| {
                                            project.on_lsp_did_change_watched_files(
                                                server_id, &reg.id, options, cx,
                                            );
                                        })?;
                                    }
                                }
                                "textDocument/rangeFormatting" => {
                                    project.update(&mut cx, |project, _| {
                                        if let Some(server) =
                                            project.language_server_for_id(server_id)
                                        {
                                            let options = reg
                                                .register_options
                                                .map(|options| {
                                                    serde_json::from_value::<
                                                        lsp::DocumentRangeFormattingOptions,
                                                    >(
                                                        options
                                                    )
                                                })
                                                .transpose()?;
                                            let provider = match options {
                                                None => OneOf::Left(true),
                                                Some(options) => OneOf::Right(options),
                                            };
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_range_formatting_provider =
                                                    Some(provider);
                                            })
                                        }
                                        anyhow::Ok(())
                                    })??;
                                }
                                "textDocument/onTypeFormatting" => {
                                    project.update(&mut cx, |project, _| {
                                        if let Some(server) =
                                            project.language_server_for_id(server_id)
                                        {
                                            let options = reg
                                                .register_options
                                                .map(|options| {
                                                    serde_json::from_value::<
                                                        lsp::DocumentOnTypeFormattingOptions,
                                                    >(
                                                        options
                                                    )
                                                })
                                                .transpose()?;
                                            if let Some(options) = options {
                                                server.update_capabilities(|capabilities| {
                                                    capabilities
                                                        .document_on_type_formatting_provider =
                                                        Some(options);
                                                })
                                            }
                                        }
                                        anyhow::Ok(())
                                    })??;
                                }
                                "textDocument/formatting" => {
                                    project.update(&mut cx, |project, _| {
                                        if let Some(server) =
                                            project.language_server_for_id(server_id)
                                        {
                                            let options = reg
                                                .register_options
                                                .map(|options| {
                                                    serde_json::from_value::<
                                                        lsp::DocumentFormattingOptions,
                                                    >(
                                                        options
                                                    )
                                                })
                                                .transpose()?;
                                            let provider = match options {
                                                None => OneOf::Left(true),
                                                Some(options) => OneOf::Right(options),
                                            };
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_formatting_provider =
                                                    Some(provider);
                                            })
                                        }
                                        anyhow::Ok(())
                                    })??;
                                }
                                _ => log::warn!("unhandled capability registration: {reg:?}"),
                            }
                        }
                        Ok(())
                    }
                }
            })
            .detach();

        language_server
            .on_request::<lsp::request::UnregisterCapability, _, _>({
                let this = project.clone();
                move |params, mut cx| {
                    let project = this.clone();
                    async move {
                        for unreg in params.unregisterations.iter() {
                            match unreg.method.as_str() {
                                "workspace/didChangeWatchedFiles" => {
                                    project.update(&mut cx, |project, cx| {
                                        project.on_lsp_unregister_did_change_watched_files(
                                            server_id, &unreg.id, cx,
                                        );
                                    })?;
                                }
                                "textDocument/rangeFormatting" => {
                                    project.update(&mut cx, |project, _| {
                                        if let Some(server) =
                                            project.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_range_formatting_provider =
                                                    None
                                            })
                                        }
                                    })?;
                                }
                                "textDocument/onTypeFormatting" => {
                                    project.update(&mut cx, |project, _| {
                                        if let Some(server) =
                                            project.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_on_type_formatting_provider =
                                                    None;
                                            })
                                        }
                                    })?;
                                }
                                "textDocument/formatting" => {
                                    project.update(&mut cx, |project, _| {
                                        if let Some(server) =
                                            project.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_formatting_provider = None;
                                            })
                                        }
                                    })?;
                                }
                                _ => log::warn!("unhandled capability unregistration: {unreg:?}"),
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
                let this = project.clone();
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
                let this = project.clone();
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
                let this = project.clone();
                let name = name.to_string();
                move |params, mut cx| {
                    let this = this.clone();
                    let name = name.to_string();
                    async move {
                        let actions = params.actions.unwrap_or_default();
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
                    }
                }
            })
            .detach();

        let disk_based_diagnostics_progress_token =
            adapter.disk_based_diagnostics_progress_token.clone();

        language_server
            .on_notification::<ServerStatus, _>({
                let this = project.clone();
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
            .on_notification::<lsp::notification::ShowMessage, _>({
                let this = project.clone();
                let name = name.to_string();
                move |params, mut cx| {
                    let this = this.clone();
                    let name = name.to_string();

                    let (tx, _) = smol::channel::bounded(1);
                    let request = LanguageServerPromptRequest {
                        level: match params.typ {
                            lsp::MessageType::ERROR => PromptLevel::Critical,
                            lsp::MessageType::WARNING => PromptLevel::Warning,
                            _ => PromptLevel::Info,
                        },
                        message: params.message,
                        actions: vec![],
                        response_channel: tx,
                        lsp_name: name.clone(),
                    };

                    let _ = this.update(&mut cx, |_, cx| {
                        cx.emit(Event::LanguageServerPrompt(request));
                    });
                }
            })
            .detach();
        language_server
            .on_notification::<lsp::notification::Progress, _>(move |params, mut cx| {
                if let Some(this) = project.upgrade() {
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
        self.buffer_store.update(cx, |buffer_store, cx| {
            for buffer_handle in buffer_store.buffers() {
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
                            adapter.language_id(&language),
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
            anyhow::Ok(())
        })?;

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

            self.buffer_store.update(cx, |buffer_store, cx| {
                for buffer in buffer_store.buffers() {
                    buffer.update(cx, |buffer, cx| {
                        buffer.update_diagnostics(server_id, Default::default(), cx);
                    });
                }
            });

            let project_id = self.remote_id();
            for (worktree_id, summaries) in self.diagnostic_summaries.iter_mut() {
                summaries.retain(|path, summaries_by_server_id| {
                    if summaries_by_server_id.remove(&server_id).is_some() {
                        if let Some(project_id) = project_id {
                            self.client
                                .send(proto::UpdateDiagnosticSummary {
                                    project_id,
                                    worktree_id: worktree_id.to_proto(),
                                    summary: Some(proto::DiagnosticSummary {
                                        path: path.to_string_lossy().to_string(),
                                        language_server_id: server_id.0 as u64,
                                        error_count: 0,
                                        warning_count: 0,
                                    }),
                                })
                                .log_err();
                        }
                        !summaries_by_server_id.is_empty()
                    } else {
                        true
                    }
                });
            }

            for diagnostics in self.diagnostics.values_mut() {
                diagnostics.retain(|_, diagnostics_by_server_id| {
                    if let Ok(ix) =
                        diagnostics_by_server_id.binary_search_by_key(&server_id, |e| e.0)
                    {
                        diagnostics_by_server_id.remove(ix);
                        !diagnostics_by_server_id.is_empty()
                    } else {
                        true
                    }
                });
            }

            self.language_server_watched_paths.remove(&server_id);
            self.language_server_statuses.remove(&server_id);
            cx.notify();

            let server_state = self.language_servers.remove(&server_id);
            cx.emit(Event::LanguageServerRemoved(server_id));
            cx.spawn(move |_, cx| async move {
                Self::shutdown_language_server(server_state, name, cx).await;
                orphaned_worktrees
            })
        } else {
            Task::ready(Vec::new())
        }
    }

    async fn shutdown_language_server(
        server_state: Option<LanguageServerState>,
        name: Arc<str>,
        cx: AsyncAppContext,
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
    }

    async fn handle_restart_language_servers(
        project: Model<Self>,
        envelope: TypedEnvelope<proto::RestartLanguageServers>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        project.update(&mut cx, |project, cx| {
            let buffers: Vec<_> = envelope
                .payload
                .buffer_ids
                .into_iter()
                .flat_map(|buffer_id| {
                    project.buffer_for_id(BufferId::new(buffer_id).log_err()?, cx)
                })
                .collect();
            project.restart_language_servers_for_buffers(buffers, cx)
        })?;

        Ok(proto::Ack {})
    }

    pub fn restart_language_servers_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Model<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) {
        if self.is_remote() {
            let request = self.client.request(proto::RestartLanguageServers {
                project_id: self.remote_id().unwrap(),
                buffer_ids: buffers
                    .into_iter()
                    .map(|b| b.read(cx).remote_id().to_proto())
                    .collect(),
            });
            cx.background_executor()
                .spawn(request)
                .detach_and_log_err(cx);
            return;
        }

        #[allow(clippy::mutable_key_type)]
        let language_server_lookup_info: HashSet<(Model<Worktree>, Arc<Language>)> = buffers
            .into_iter()
            .filter_map(|buffer| {
                let buffer = buffer.read(cx);
                let file = buffer.file()?;
                let worktree = File::from_dyn(Some(file))?.worktree.clone();
                let language = self
                    .languages
                    .language_for_file(file, Some(buffer.as_rope()), cx)
                    .now_or_never()?
                    .ok()?;
                Some((worktree, language))
            })
            .collect();
        for (worktree, language) in language_server_lookup_info {
            self.restart_language_servers(worktree, language, cx);
        }
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

    pub fn cancel_language_server_work_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Model<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) {
        let servers = buffers
            .into_iter()
            .flat_map(|buffer| {
                self.language_server_ids_for_buffer(buffer.read(cx), cx)
                    .into_iter()
            })
            .collect::<HashSet<_>>();

        for server_id in servers {
            self.cancel_language_server_work(server_id, None, cx);
        }
    }

    pub fn cancel_language_server_work(
        &mut self,
        server_id: LanguageServerId,
        token_to_cancel: Option<String>,
        _cx: &mut ModelContext<Self>,
    ) {
        let status = self.language_server_statuses.get(&server_id);
        let server = self.language_servers.get(&server_id);
        if let Some((server, status)) = server.zip(status) {
            if let LanguageServerState::Running { server, .. } = server {
                for (token, progress) in &status.pending_work {
                    if let Some(token_to_cancel) = token_to_cancel.as_ref() {
                        if token != token_to_cancel {
                            continue;
                        }
                    }
                    if progress.is_cancellable {
                        server
                            .notify::<lsp::notification::WorkDoneProgressCancel>(
                                WorkDoneProgressCancelParams {
                                    token: lsp::NumberOrString::String(token.clone()),
                                },
                            )
                            .ok();
                    }
                }
            }
        }
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

    fn enqueue_buffer_ordered_message(&mut self, message: BufferOrderedMessage) -> Result<()> {
        self.buffer_ordered_messages_tx
            .unbounded_send(message)
            .map_err(|e| anyhow!(e))
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
                    self.disk_based_diagnostics_started(language_server_id, cx);
                }
                self.on_lsp_work_start(
                    language_server_id,
                    token.clone(),
                    LanguageServerProgress {
                        title: Some(report.title),
                        is_disk_based_diagnostics_progress,
                        is_cancellable: report.cancellable.unwrap_or(false),
                        message: report.message.clone(),
                        percentage: report.percentage.map(|p| p as usize),
                        last_update_at: cx.background_executor().now(),
                    },
                    cx,
                );
            }
            lsp::WorkDoneProgress::Report(report) => {
                if self.on_lsp_work_progress(
                    language_server_id,
                    token.clone(),
                    LanguageServerProgress {
                        title: None,
                        is_disk_based_diagnostics_progress,
                        is_cancellable: report.cancellable.unwrap_or(false),
                        message: report.message.clone(),
                        percentage: report.percentage.map(|p| p as usize),
                        last_update_at: cx.background_executor().now(),
                    },
                    cx,
                ) {
                    self.enqueue_buffer_ordered_message(
                        BufferOrderedMessage::LanguageServerUpdate {
                            language_server_id,
                            message: proto::update_language_server::Variant::WorkProgress(
                                proto::LspWorkProgress {
                                    token,
                                    message: report.message,
                                    percentage: report.percentage,
                                },
                            ),
                        },
                    )
                    .ok();
                }
            }
            lsp::WorkDoneProgress::End(_) => {
                language_server_status.progress_tokens.remove(&token);
                self.on_lsp_work_end(language_server_id, token.clone(), cx);
                if is_disk_based_diagnostics_progress {
                    self.disk_based_diagnostics_finished(language_server_id, cx);
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
            status.pending_work.insert(token.clone(), progress.clone());
            cx.notify();
        }

        if self.is_local() {
            self.enqueue_buffer_ordered_message(BufferOrderedMessage::LanguageServerUpdate {
                language_server_id,
                message: proto::update_language_server::Variant::WorkStart(proto::LspWorkStart {
                    token,
                    title: progress.title,
                    message: progress.message,
                    percentage: progress.percentage.map(|p| p as u32),
                }),
            })
            .ok();
        }
    }

    fn on_lsp_work_progress(
        &mut self,
        language_server_id: LanguageServerId,
        token: String,
        progress: LanguageServerProgress,
        cx: &mut ModelContext<Self>,
    ) -> bool {
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            match status.pending_work.entry(token) {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(progress);
                    cx.notify();
                    return true;
                }
                btree_map::Entry::Occupied(mut entry) => {
                    let entry = entry.get_mut();
                    if (progress.last_update_at - entry.last_update_at)
                        >= SERVER_PROGRESS_THROTTLE_TIMEOUT
                    {
                        entry.last_update_at = progress.last_update_at;
                        if progress.message.is_some() {
                            entry.message = progress.message;
                        }
                        if progress.percentage.is_some() {
                            entry.percentage = progress.percentage;
                        }
                        cx.notify();
                        return true;
                    }
                }
            }
        }

        false
    }

    fn on_lsp_work_end(
        &mut self,
        language_server_id: LanguageServerId,
        token: String,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            if let Some(work) = status.pending_work.remove(&token) {
                if !work.is_disk_based_diagnostics_progress {
                    cx.emit(Event::RefreshInlayHints);
                }
            }
            cx.notify();
        }

        if self.is_local() {
            self.enqueue_buffer_ordered_message(BufferOrderedMessage::LanguageServerUpdate {
                language_server_id,
                message: proto::update_language_server::Variant::WorkEnd(proto::LspWorkEnd {
                    token,
                }),
            })
            .ok();
        }
    }

    fn on_lsp_did_change_watched_files(
        &mut self,
        language_server_id: LanguageServerId,
        registration_id: &str,
        params: DidChangeWatchedFilesRegistrationOptions,
        cx: &mut ModelContext<Self>,
    ) {
        let registrations = self
            .language_server_watcher_registrations
            .entry(language_server_id)
            .or_default();

        registrations.insert(registration_id.to_string(), params.watchers);

        self.rebuild_watched_paths(language_server_id, cx);
    }

    fn on_lsp_unregister_did_change_watched_files(
        &mut self,
        language_server_id: LanguageServerId,
        registration_id: &str,
        cx: &mut ModelContext<Self>,
    ) {
        let registrations = self
            .language_server_watcher_registrations
            .entry(language_server_id)
            .or_default();

        if registrations.remove(registration_id).is_some() {
            log::info!(
                "language server {}: unregistered workspace/DidChangeWatchedFiles capability with id {}",
                language_server_id,
                registration_id
            );
        } else {
            log::warn!(
                "language server {}: failed to unregister workspace/DidChangeWatchedFiles capability with id {}. not registered.",
                language_server_id,
                registration_id
            );
        }

        self.rebuild_watched_paths(language_server_id, cx);
    }

    fn rebuild_watched_paths(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(watchers) = self
            .language_server_watcher_registrations
            .get(&language_server_id)
        else {
            return;
        };

        let watched_paths = self
            .language_server_watched_paths
            .entry(language_server_id)
            .or_default();

        let mut builders = HashMap::default();
        for watcher in watchers.values().flatten() {
            for worktree in self.worktree_store.read(cx).worktrees().collect::<Vec<_>>() {
                let glob_is_inside_worktree = worktree.update(cx, |tree, _| {
                    if let Some(abs_path) = tree.abs_path().to_str() {
                        let relative_glob_pattern = match &watcher.glob_pattern {
                            lsp::GlobPattern::String(s) => Some(
                                s.strip_prefix(abs_path)
                                    .unwrap_or(s)
                                    .strip_prefix(std::path::MAIN_SEPARATOR)
                                    .unwrap_or(s),
                            ),
                            lsp::GlobPattern::Relative(rp) => {
                                let base_uri = match &rp.base_uri {
                                    lsp::OneOf::Left(workspace_folder) => &workspace_folder.uri,
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

        watched_paths.clear();
        for (worktree_id, builder) in builders {
            if let Ok(globset) = builder.build() {
                watched_paths.insert(worktree_id, globset);
            }
        }

        cx.notify();
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
    ) -> impl DoubleEndedIterator<Item = (LanguageServerId, &LanguageServerStatus)> {
        self.language_server_statuses
            .iter()
            .map(|(key, value)| (*key, value))
    }

    pub fn last_formatting_failure(&self) -> Option<&str> {
        self.last_formatting_failure.as_deref()
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
                        data: diagnostic.data.clone(),
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
                                    data: diagnostic.data.clone(),
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
            .find_worktree(&abs_path, cx)
            .ok_or_else(|| anyhow!("no worktree found for diagnostics path {abs_path:?}"))?;

        let project_path = ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: relative_path.into(),
        };

        if let Some(buffer) = self.get_open_buffer(&project_path, cx) {
            self.update_buffer_diagnostics(&buffer, server_id, version, diagnostics.clone(), cx)?;
        }

        let updated = worktree.update(cx, |worktree, cx| {
            self.update_worktree_diagnostics(
                worktree.id(),
                server_id,
                project_path.path.clone(),
                diagnostics,
                cx,
            )
        })?;
        if updated {
            cx.emit(Event::DiagnosticsUpdated {
                language_server_id: server_id,
                path: project_path,
            });
        }
        Ok(())
    }

    pub fn update_worktree_diagnostics(
        &mut self,
        worktree_id: WorktreeId,
        server_id: LanguageServerId,
        worktree_path: Arc<Path>,
        diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        _: &mut ModelContext<Worktree>,
    ) -> Result<bool> {
        let summaries_for_tree = self.diagnostic_summaries.entry(worktree_id).or_default();
        let diagnostics_for_tree = self.diagnostics.entry(worktree_id).or_default();
        let summaries_by_server_id = summaries_for_tree.entry(worktree_path.clone()).or_default();

        let old_summary = summaries_by_server_id
            .remove(&server_id)
            .unwrap_or_default();

        let new_summary = DiagnosticSummary::new(&diagnostics);
        if new_summary.is_empty() {
            if let Some(diagnostics_by_server_id) = diagnostics_for_tree.get_mut(&worktree_path) {
                if let Ok(ix) = diagnostics_by_server_id.binary_search_by_key(&server_id, |e| e.0) {
                    diagnostics_by_server_id.remove(ix);
                }
                if diagnostics_by_server_id.is_empty() {
                    diagnostics_for_tree.remove(&worktree_path);
                }
            }
        } else {
            summaries_by_server_id.insert(server_id, new_summary);
            let diagnostics_by_server_id = diagnostics_for_tree
                .entry(worktree_path.clone())
                .or_default();
            match diagnostics_by_server_id.binary_search_by_key(&server_id, |e| e.0) {
                Ok(ix) => {
                    diagnostics_by_server_id[ix] = (server_id, diagnostics);
                }
                Err(ix) => {
                    diagnostics_by_server_id.insert(ix, (server_id, diagnostics));
                }
            }
        }

        if !old_summary.is_empty() || !new_summary.is_empty() {
            if let Some(project_id) = self.remote_id() {
                self.client
                    .send(proto::UpdateDiagnosticSummary {
                        project_id,
                        worktree_id: worktree_id.to_proto(),
                        summary: Some(proto::DiagnosticSummary {
                            path: worktree_path.to_string_lossy().to_string(),
                            language_server_id: server_id.0 as u64,
                            error_count: new_summary.error_count as u32,
                            warning_count: new_summary.warning_count as u32,
                        }),
                    })
                    .log_err();
            }
        }

        Ok(!old_summary.is_empty() || !new_summary.is_empty())
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
                Self::deserialize_project_transaction(this, response, push_to_history, cx.clone())
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
            let buffers_with_paths = buffers
                .into_iter()
                .map(|buffer_handle| {
                    let buffer = buffer_handle.read(cx);
                    let buffer_abs_path = File::from_dyn(buffer.file())
                        .and_then(|file| file.as_local().map(|f| f.abs_path(cx)));
                    (buffer_handle, buffer_abs_path)
                })
                .collect::<Vec<_>>();

            cx.spawn(move |project, mut cx| async move {
                let result = Self::format_locally(
                    project.clone(),
                    buffers_with_paths,
                    push_to_history,
                    trigger,
                    cx.clone(),
                )
                .await;

                project.update(&mut cx, |project, _| match &result {
                    Ok(_) => project.last_formatting_failure = None,
                    Err(error) => {
                        project.last_formatting_failure.replace(error.to_string());
                    }
                })?;

                result
            })
        } else {
            let remote_id = self.remote_id();
            let client = self.client.clone();
            cx.spawn(move |this, mut cx| async move {
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
                    Self::deserialize_project_transaction(this, response, push_to_history, cx).await
                } else {
                    Ok(ProjectTransaction::default())
                }
            })
        }
    }

    async fn format_locally(
        project: WeakModel<Project>,
        mut buffers_with_paths: Vec<(Model<Buffer>, Option<PathBuf>)>,
        push_to_history: bool,
        trigger: FormatTrigger,
        mut cx: AsyncAppContext,
    ) -> anyhow::Result<ProjectTransaction> {
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
            let (primary_adapter_and_server, adapters_and_servers) =
                project.update(&mut cx, |project, cx| {
                    let buffer = buffer.read(cx);

                    let adapters_and_servers = project
                        .language_servers_for_buffer(buffer, cx)
                        .map(|(adapter, lsp)| (adapter.clone(), lsp.clone()))
                        .collect::<Vec<_>>();

                    let primary_adapter = project
                        .primary_language_server_for_buffer(buffer, cx)
                        .map(|(adapter, lsp)| (adapter.clone(), lsp.clone()));

                    (primary_adapter, adapters_and_servers)
                })?;

            let settings = buffer.update(&mut cx, |buffer, cx| {
                language_settings(buffer.language(), buffer.file(), cx).clone()
            })?;

            let remove_trailing_whitespace = settings.remove_trailing_whitespace_on_save;
            let ensure_final_newline = settings.ensure_final_newline_on_save;

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

            // Apply the `code_actions_on_format` before we run the formatter.
            let code_actions = deserialize_code_actions(&settings.code_actions_on_format);
            #[allow(clippy::nonminimal_bool)]
            if !code_actions.is_empty()
                && !(trigger == FormatTrigger::Save && settings.format_on_save == FormatOnSave::Off)
            {
                Self::execute_code_actions_on_servers(
                    &project,
                    &adapters_and_servers,
                    code_actions,
                    buffer,
                    push_to_history,
                    &mut project_transaction,
                    &mut cx,
                )
                .await?;
            }

            // Apply language-specific formatting using either the primary language server
            // or external command.
            // Except for code actions, which are applied with all connected language servers.
            let primary_language_server =
                primary_adapter_and_server.map(|(_adapter, server)| server.clone());
            let server_and_buffer = primary_language_server
                .as_ref()
                .zip(buffer_abs_path.as_ref());

            let prettier_settings = buffer.read_with(&mut cx, |buffer, cx| {
                language_settings(buffer.language(), buffer.file(), cx)
                    .prettier
                    .clone()
            })?;

            let mut format_operations: Vec<FormatOperation> = vec![];
            {
                match trigger {
                    FormatTrigger::Save => {
                        match &settings.format_on_save {
                            FormatOnSave::Off => {
                                // nothing
                            }
                            FormatOnSave::On => {
                                match &settings.formatter {
                                    SelectedFormatter::Auto => {
                                        // do the auto-format: prefer prettier, fallback to primary language server
                                        let diff = {
                                            if prettier_settings.allowed {
                                                Self::perform_format(
                                                    &Formatter::Prettier,
                                                    server_and_buffer,
                                                    project.clone(),
                                                    buffer,
                                                    buffer_abs_path,
                                                    &settings,
                                                    &adapters_and_servers,
                                                    push_to_history,
                                                    &mut project_transaction,
                                                    &mut cx,
                                                )
                                                .await
                                            } else {
                                                Self::perform_format(
                                                    &Formatter::LanguageServer { name: None },
                                                    server_and_buffer,
                                                    project.clone(),
                                                    buffer,
                                                    buffer_abs_path,
                                                    &settings,
                                                    &adapters_and_servers,
                                                    push_to_history,
                                                    &mut project_transaction,
                                                    &mut cx,
                                                )
                                                .await
                                            }
                                        }
                                        .log_err()
                                        .flatten();
                                        if let Some(op) = diff {
                                            format_operations.push(op);
                                        }
                                    }
                                    SelectedFormatter::List(formatters) => {
                                        for formatter in formatters.as_ref() {
                                            let diff = Self::perform_format(
                                                formatter,
                                                server_and_buffer,
                                                project.clone(),
                                                buffer,
                                                buffer_abs_path,
                                                &settings,
                                                &adapters_and_servers,
                                                push_to_history,
                                                &mut project_transaction,
                                                &mut cx,
                                            )
                                            .await
                                            .log_err()
                                            .flatten();
                                            if let Some(op) = diff {
                                                format_operations.push(op);
                                            }

                                            // format with formatter
                                        }
                                    }
                                }
                            }
                            FormatOnSave::List(formatters) => {
                                for formatter in formatters.as_ref() {
                                    let diff = Self::perform_format(
                                        &formatter,
                                        server_and_buffer,
                                        project.clone(),
                                        buffer,
                                        buffer_abs_path,
                                        &settings,
                                        &adapters_and_servers,
                                        push_to_history,
                                        &mut project_transaction,
                                        &mut cx,
                                    )
                                    .await
                                    .log_err()
                                    .flatten();
                                    if let Some(op) = diff {
                                        format_operations.push(op);
                                    }
                                }
                            }
                        }
                    }
                    FormatTrigger::Manual => {
                        match &settings.formatter {
                            SelectedFormatter::Auto => {
                                // do the auto-format: prefer prettier, fallback to primary language server
                                let diff = {
                                    if prettier_settings.allowed {
                                        Self::perform_format(
                                            &Formatter::Prettier,
                                            server_and_buffer,
                                            project.clone(),
                                            buffer,
                                            buffer_abs_path,
                                            &settings,
                                            &adapters_and_servers,
                                            push_to_history,
                                            &mut project_transaction,
                                            &mut cx,
                                        )
                                        .await
                                    } else {
                                        Self::perform_format(
                                            &Formatter::LanguageServer { name: None },
                                            server_and_buffer,
                                            project.clone(),
                                            buffer,
                                            buffer_abs_path,
                                            &settings,
                                            &adapters_and_servers,
                                            push_to_history,
                                            &mut project_transaction,
                                            &mut cx,
                                        )
                                        .await
                                    }
                                }
                                .log_err()
                                .flatten();

                                if let Some(op) = diff {
                                    format_operations.push(op)
                                }
                            }
                            SelectedFormatter::List(formatters) => {
                                for formatter in formatters.as_ref() {
                                    // format with formatter
                                    let diff = Self::perform_format(
                                        formatter,
                                        server_and_buffer,
                                        project.clone(),
                                        buffer,
                                        buffer_abs_path,
                                        &settings,
                                        &adapters_and_servers,
                                        push_to_history,
                                        &mut project_transaction,
                                        &mut cx,
                                    )
                                    .await
                                    .log_err()
                                    .flatten();
                                    if let Some(op) = diff {
                                        format_operations.push(op);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            buffer.update(&mut cx, |b, cx| {
                // If the buffer had its whitespace formatted and was edited while the language-specific
                // formatting was being computed, avoid applying the language-specific formatting, because
                // it can't be grouped with the whitespace formatting in the undo history.
                if let Some(transaction_id) = whitespace_transaction_id {
                    if b.peek_undo_stack()
                        .map_or(true, |e| e.transaction_id() != transaction_id)
                    {
                        format_operations.clear();
                    }
                }

                // Apply any language-specific formatting, and group the two formatting operations
                // in the buffer's undo history.
                for operation in format_operations {
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
    }

    #[allow(clippy::too_many_arguments)]
    async fn perform_format(
        formatter: &Formatter,
        primary_server_and_buffer: Option<(&Arc<LanguageServer>, &PathBuf)>,
        project: WeakModel<Project>,
        buffer: &Model<Buffer>,
        buffer_abs_path: &Option<PathBuf>,
        settings: &LanguageSettings,
        adapters_and_servers: &Vec<(Arc<CachedLspAdapter>, Arc<LanguageServer>)>,
        push_to_history: bool,
        transaction: &mut ProjectTransaction,
        mut cx: &mut AsyncAppContext,
    ) -> Result<Option<FormatOperation>, anyhow::Error> {
        let result = match formatter {
            Formatter::LanguageServer { name } => {
                if let Some((language_server, buffer_abs_path)) = primary_server_and_buffer {
                    let language_server = if let Some(name) = name {
                        adapters_and_servers
                            .iter()
                            .find_map(|(adapter, server)| {
                                adapter.name.0.as_ref().eq(name.as_str()).then_some(server)
                            })
                            .unwrap_or_else(|| language_server)
                    } else {
                        language_server
                    };
                    Some(FormatOperation::Lsp(
                        Self::format_via_lsp(
                            &project,
                            buffer,
                            buffer_abs_path,
                            language_server,
                            settings,
                            cx,
                        )
                        .await
                        .context("failed to format via language server")?,
                    ))
                } else {
                    None
                }
            }
            Formatter::Prettier => {
                prettier_support::format_with_prettier(&project, buffer, &mut cx)
                    .await
                    .transpose()
                    .ok()
                    .flatten()
            }
            Formatter::External { command, arguments } => {
                let buffer_abs_path = buffer_abs_path.as_ref().map(|path| path.as_path());
                Self::format_via_external_command(
                    buffer,
                    buffer_abs_path,
                    &command,
                    &arguments,
                    &mut cx,
                )
                .await
                .context(format!(
                    "failed to format via external command {:?}",
                    command
                ))?
                .map(FormatOperation::External)
            }
            Formatter::CodeActions(code_actions) => {
                let code_actions = deserialize_code_actions(&code_actions);
                if !code_actions.is_empty() {
                    Self::execute_code_actions_on_servers(
                        &project,
                        &adapters_and_servers,
                        code_actions,
                        buffer,
                        push_to_history,
                        transaction,
                        cx,
                    )
                    .await?;
                }
                None
            }
        };
        anyhow::Ok(result)
    }

    async fn format_via_lsp(
        this: &WeakModel<Self>,
        buffer: &Model<Buffer>,
        abs_path: &Path,
        language_server: &Arc<LanguageServer>,
        settings: &LanguageSettings,
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
                    options: lsp_command::lsp_formatting_options(settings),
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
                    options: lsp_command::lsp_formatting_options(settings),
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
        buffer_abs_path: Option<&Path>,
        command: &str,
        arguments: &[String],
        cx: &mut AsyncAppContext,
    ) -> Result<Option<Diff>> {
        let working_dir_path = buffer.update(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file())?;
            let worktree = file.worktree.read(cx);
            let mut worktree_path = worktree.abs_path().to_path_buf();
            if worktree.root_entry()?.is_file() {
                worktree_path.pop();
            }
            Some(worktree_path)
        })?;

        let mut child = smol::process::Command::new(command);

        if let Some(working_dir_path) = working_dir_path {
            child.current_dir(working_dir_path);
        }

        let mut child = child
            .args(arguments.iter().map(|arg| {
                if let Some(buffer_abs_path) = buffer_abs_path {
                    arg.replace("{buffer_path}", &buffer_abs_path.to_string_lossy())
                } else {
                    arg.replace("{buffer_path}", "Untitled")
                }
            }))
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

    fn declaration_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetDeclaration { position },
            cx,
        )
    }

    pub fn declaration<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.declaration_impl(buffer, position, cx)
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
        let language_registry = self.languages.clone();

        if self.is_local() {
            let mut requests = Vec::new();
            for ((worktree_id, _), server_id) in self.language_server_ids.iter() {
                let Some(worktree_handle) = self.worktree_for_id(*worktree_id, cx) else {
                    continue;
                };
                let worktree = worktree_handle.read(cx);
                if !worktree.is_visible() {
                    continue;
                }
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
                                worktree_handle.downgrade(),
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

                let mut symbols = Vec::new();
                for (adapter, adapter_language, source_worktree, worktree_abs_path, lsp_symbols) in
                    responses
                {
                    let core_symbols = this.update(&mut cx, |this, cx| {
                        lsp_symbols
                            .into_iter()
                            .filter_map(|(symbol_name, symbol_kind, symbol_location)| {
                                let abs_path = symbol_location.uri.to_file_path().ok()?;
                                let source_worktree = source_worktree.upgrade()?;
                                let source_worktree_id = source_worktree.read(cx).id();

                                let path;
                                let worktree;
                                if let Some((tree, rel_path)) = this.find_worktree(&abs_path, cx) {
                                    worktree = tree;
                                    path = rel_path;
                                } else {
                                    worktree = source_worktree.clone();
                                    path = relativize_path(&worktree_abs_path, &abs_path);
                                }

                                let worktree_id = worktree.read(cx).id();
                                let project_path = ProjectPath {
                                    worktree_id,
                                    path: path.into(),
                                };
                                let signature = this.symbol_signature(&project_path);
                                Some(CoreSymbol {
                                    language_server_name: adapter.name.clone(),
                                    source_worktree_id,
                                    path: project_path,
                                    kind: symbol_kind,
                                    name: symbol_name,
                                    range: range_from_lsp(symbol_location.range),
                                    signature,
                                })
                            })
                            .collect()
                    })?;

                    populate_labels_for_symbols(
                        core_symbols,
                        &language_registry,
                        Some(adapter_language),
                        Some(adapter),
                        &mut symbols,
                    )
                    .await;
                }

                Ok(symbols)
            })
        } else if let Some(project_id) = self.remote_id() {
            let request = self.client.request(proto::GetProjectSymbols {
                project_id,
                query: query.to_string(),
            });
            cx.foreground_executor().spawn(async move {
                let response = request.await?;
                let mut symbols = Vec::new();
                let core_symbols = response
                    .symbols
                    .into_iter()
                    .filter_map(|symbol| Self::deserialize_symbol(symbol).log_err())
                    .collect::<Vec<_>>();
                populate_labels_for_symbols(
                    core_symbols,
                    &language_registry,
                    None,
                    None,
                    &mut symbols,
                )
                .await;
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
                .map(|worktree| worktree.read(cx).abs_path())
            {
                worktree_abs_path
            } else {
                return Task::ready(Err(anyhow!("worktree not found for symbol")));
            };

            let symbol_abs_path = resolve_path(&worktree_abs_path, &symbol.path.path);
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

    pub fn signature_help<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<SignatureHelp>> {
        let position = position.to_point_utf16(buffer.read(cx));
        if self.is_local() {
            let all_actions_task = self.request_multiple_lsp_locally(
                buffer,
                Some(position),
                GetSignatureHelp { position },
                cx,
            );
            cx.spawn(|_, _| async move {
                all_actions_task
                    .await
                    .into_iter()
                    .flatten()
                    .filter(|help| !help.markdown.is_empty())
                    .collect::<Vec<_>>()
            })
        } else if let Some(project_id) = self.remote_id() {
            let request_task = self.client().request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetSignatureHelp(
                    GetSignatureHelp { position }.to_proto(project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(|weak_project, cx| async move {
                let Some(project) = weak_project.upgrade() else {
                    return Vec::new();
                };
                join_all(
                    request_task
                        .await
                        .log_err()
                        .map(|response| response.responses)
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetSignatureHelpResponse(response) => {
                                Some(response)
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|signature_response| {
                            let response = GetSignatureHelp { position }.response_from_proto(
                                signature_response,
                                project.clone(),
                                buffer.clone(),
                                cx.clone(),
                            );
                            async move { response.await.log_err().flatten() }
                        }),
                )
                .await
                .into_iter()
                .flatten()
                .collect()
            })
        } else {
            Task::ready(Vec::new())
        }
    }

    fn hover_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<Hover>> {
        if self.is_local() {
            let all_actions_task = self.request_multiple_lsp_locally(
                &buffer,
                Some(position),
                GetHover { position },
                cx,
            );
            cx.spawn(|_, _| async move {
                all_actions_task
                    .await
                    .into_iter()
                    .filter_map(|hover| remove_empty_hover_blocks(hover?))
                    .collect::<Vec<Hover>>()
            })
        } else if let Some(project_id) = self.remote_id() {
            let request_task = self.client().request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetHover(
                    GetHover { position }.to_proto(project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(|weak_project, cx| async move {
                let Some(project) = weak_project.upgrade() else {
                    return Vec::new();
                };
                join_all(
                    request_task
                        .await
                        .log_err()
                        .map(|response| response.responses)
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetHoverResponse(response) => {
                                Some(response)
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|hover_response| {
                            let response = GetHover { position }.response_from_proto(
                                hover_response,
                                project.clone(),
                                buffer.clone(),
                                cx.clone(),
                            );
                            async move {
                                response
                                    .await
                                    .log_err()
                                    .flatten()
                                    .and_then(remove_empty_hover_blocks)
                            }
                        }),
                )
                .await
                .into_iter()
                .flatten()
                .collect()
            })
        } else {
            log::error!("cannot show hovers: project does not have a remote id");
            Task::ready(Vec::new())
        }
    }

    pub fn hover<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<Hover>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.hover_impl(buffer, position, cx)
    }

    fn linked_edit_impl(
        &self,
        buffer: &Model<Buffer>,
        position: Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Range<Anchor>>>> {
        let snapshot = buffer.read(cx).snapshot();
        let scope = snapshot.language_scope_at(position);
        let Some(server_id) = self
            .language_servers_for_buffer(buffer.read(cx), cx)
            .filter(|(_, server)| {
                server
                    .capabilities()
                    .linked_editing_range_provider
                    .is_some()
            })
            .filter(|(adapter, _)| {
                scope
                    .as_ref()
                    .map(|scope| scope.language_allowed(&adapter.name))
                    .unwrap_or(true)
            })
            .map(|(_, server)| LanguageServerToQuery::Other(server.server_id()))
            .next()
            .or_else(|| self.is_remote().then_some(LanguageServerToQuery::Primary))
            .filter(|_| {
                maybe!({
                    let language_name = buffer.read(cx).language_at(position)?.name();
                    Some(
                        AllLanguageSettings::get_global(cx)
                            .language(Some(&language_name))
                            .linked_edits,
                    )
                }) == Some(true)
            })
        else {
            return Task::ready(Ok(vec![]));
        };

        self.request_lsp(
            buffer.clone(),
            server_id,
            LinkedEditingRange { position },
            cx,
        )
    }

    pub fn linked_edit(
        &self,
        buffer: &Model<Buffer>,
        position: Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Range<Anchor>>>> {
        self.linked_edit_impl(buffer, position, cx)
    }

    #[inline(never)]
    fn completions_impl(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        context: CompletionContext,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        let language_registry = self.languages.clone();

        if self.is_local() {
            let snapshot = buffer.read(cx).snapshot();
            let offset = position.to_offset(&snapshot);
            let scope = snapshot.language_scope_at(offset);
            let language = snapshot.language().cloned();

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
                        let lsp_adapter = this.language_server_adapter_for_id(server_id);
                        tasks.push((
                            lsp_adapter,
                            this.request_lsp(
                                buffer.clone(),
                                LanguageServerToQuery::Other(server_id),
                                GetCompletions {
                                    position,
                                    context: context.clone(),
                                },
                                cx,
                            ),
                        ));
                    }
                })?;

                let mut completions = Vec::new();
                for (lsp_adapter, task) in tasks {
                    if let Ok(new_completions) = task.await {
                        populate_labels_for_completions(
                            new_completions,
                            &language_registry,
                            language.clone(),
                            lsp_adapter,
                            &mut completions,
                        )
                        .await;
                    }
                }

                Ok(completions)
            })
        } else if let Some(project_id) = self.remote_id() {
            let task = self.send_lsp_proto_request(
                buffer.clone(),
                project_id,
                GetCompletions { position, context },
                cx,
            );
            let language = buffer.read(cx).language().cloned();

            // In the future, we should provide project guests with the names of LSP adapters,
            // so that they can use the correct LSP adapter when computing labels. For now,
            // guests just use the first LSP adapter associated with the buffer's language.
            let lsp_adapter = language
                .as_ref()
                .and_then(|language| language_registry.lsp_adapters(language).first().cloned());

            cx.foreground_executor().spawn(async move {
                let completions = task.await?;
                let mut result = Vec::new();
                populate_labels_for_completions(
                    completions,
                    &language_registry,
                    language,
                    lsp_adapter,
                    &mut result,
                )
                .await;
                Ok(result)
            })
        } else {
            Task::ready(Ok(Default::default()))
        }
    }

    pub fn completions<T: ToOffset + ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        context: CompletionContext,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.completions_impl(buffer, position, context, cx)
    }

    pub fn resolve_completions(
        &self,
        buffer: Model<Buffer>,
        completion_indices: Vec<usize>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<bool>> {
        let client = self.client();
        let language_registry = self.languages().clone();

        let is_remote = self.is_remote();
        let project_id = self.remote_id();

        let buffer_id = buffer.read(cx).remote_id();
        let buffer_snapshot = buffer.read(cx).snapshot();

        cx.spawn(move |this, mut cx| async move {
            let mut did_resolve = false;
            if is_remote {
                let project_id =
                    project_id.ok_or_else(|| anyhow!("Remote project without remote_id"))?;

                for completion_index in completion_indices {
                    let (server_id, completion) = {
                        let completions_guard = completions.read();
                        let completion = &completions_guard[completion_index];
                        if completion.documentation.is_some() {
                            continue;
                        }

                        did_resolve = true;
                        let server_id = completion.server_id;
                        let completion = completion.lsp_completion.clone();

                        (server_id, completion)
                    };

                    Self::resolve_completion_remote(
                        project_id,
                        server_id,
                        buffer_id,
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
                    let (server_id, completion) = {
                        let completions_guard = completions.read();
                        let completion = &completions_guard[completion_index];
                        if completion.documentation.is_some() {
                            continue;
                        }

                        let server_id = completion.server_id;
                        let completion = completion.lsp_completion.clone();

                        (server_id, completion)
                    };

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
                    Self::resolve_completion_local(
                        server,
                        &buffer_snapshot,
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

    async fn resolve_completion_local(
        server: Arc<lsp::LanguageServer>,
        snapshot: &BufferSnapshot,
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

        if let Some(lsp_documentation) = completion_item.documentation.as_ref() {
            let documentation = language::prepare_completion_documentation(
                lsp_documentation,
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

        if let Some(text_edit) = completion_item.text_edit.as_ref() {
            // Technically we don't have to parse the whole `text_edit`, since the only
            // language server we currently use that does update `text_edit` in `completionItem/resolve`
            // is `typescript-language-server` and they only update `text_edit.new_text`.
            // But we should not rely on that.
            let edit = parse_completion_text_edit(text_edit, snapshot);

            if let Some((old_range, mut new_text)) = edit {
                LineEnding::normalize(&mut new_text);

                let mut completions = completions.write();
                let completion = &mut completions[completion_index];

                completion.new_text = new_text;
                completion.old_range = old_range;
            }
        }
        if completion_item.insert_text_format == Some(InsertTextFormat::SNIPPET) {
            // vtsls might change the type of completion after resolution.
            let mut completions = completions.write();
            let completion = &mut completions[completion_index];
            if completion_item.insert_text_format != completion.lsp_completion.insert_text_format {
                completion.lsp_completion.insert_text_format = completion_item.insert_text_format;
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn resolve_completion_remote(
        project_id: u64,
        server_id: LanguageServerId,
        buffer_id: BufferId,
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
            buffer_id: buffer_id.into(),
        };

        let Some(response) = client
            .request(request)
            .await
            .context("completion documentation resolve proto request")
            .log_err()
        else {
            return;
        };

        let documentation = if response.documentation.is_empty() {
            Documentation::Undocumented
        } else if response.documentation_is_markdown {
            Documentation::MultiLineMarkdown(
                markdown::parse_markdown(&response.documentation, &language_registry, None).await,
            )
        } else if response.documentation.lines().count() <= 1 {
            Documentation::SingleLine(response.documentation)
        } else {
            Documentation::MultiLinePlainText(response.documentation)
        };

        let mut completions = completions.write();
        let completion = &mut completions[completion_index];
        completion.documentation = Some(documentation);

        let old_range = response
            .old_start
            .and_then(deserialize_anchor)
            .zip(response.old_end.and_then(deserialize_anchor));
        if let Some((old_start, old_end)) = old_range {
            if !response.new_text.is_empty() {
                completion.new_text = response.new_text;
                completion.old_range = old_start..old_end;
            }
        }
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
                        completion: Some(Self::serialize_completion(&CoreCompletion {
                            old_range: completion.old_range,
                            new_text: completion.new_text,
                            server_id: completion.server_id,
                            lsp_completion: completion.lsp_completion,
                        })),
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
        &mut self,
        buffer_handle: &Model<Buffer>,
        range: Range<Anchor>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<CodeAction>> {
        if self.is_local() {
            let all_actions_task = self.request_multiple_lsp_locally(
                &buffer_handle,
                Some(range.start),
                GetCodeActions {
                    range: range.clone(),
                    kinds: None,
                },
                cx,
            );
            cx.spawn(|_, _| async move { all_actions_task.await.into_iter().flatten().collect() })
        } else if let Some(project_id) = self.remote_id() {
            let request_task = self.client().request(proto::MultiLspQuery {
                buffer_id: buffer_handle.read(cx).remote_id().into(),
                version: serialize_version(&buffer_handle.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetCodeActions(
                    GetCodeActions {
                        range: range.clone(),
                        kinds: None,
                    }
                    .to_proto(project_id, buffer_handle.read(cx)),
                )),
            });
            let buffer = buffer_handle.clone();
            cx.spawn(|weak_project, cx| async move {
                let Some(project) = weak_project.upgrade() else {
                    return Vec::new();
                };
                join_all(
                    request_task
                        .await
                        .log_err()
                        .map(|response| response.responses)
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetCodeActionsResponse(response) => {
                                Some(response)
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|code_actions_response| {
                            let response = GetCodeActions {
                                range: range.clone(),
                                kinds: None,
                            }
                            .response_from_proto(
                                code_actions_response,
                                project.clone(),
                                buffer.clone(),
                                cx.clone(),
                            );
                            async move { response.await.log_err().unwrap_or_default() }
                        }),
                )
                .await
                .into_iter()
                .flatten()
                .collect()
            })
        } else {
            log::error!("cannot fetch actions: project does not have a remote id");
            Task::ready(Vec::new())
        }
    }

    pub fn code_actions<T: Clone + ToOffset>(
        &mut self,
        buffer_handle: &Model<Buffer>,
        range: Range<T>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<CodeAction>> {
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
            cx.spawn(move |this, mut cx| async move {
                Self::try_resolve_code_action(&lang_server, &mut action)
                    .await
                    .context("resolving a code action")?;
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
                action: Some(Self::serialize_code_action(&action)),
            };
            cx.spawn(move |this, cx| async move {
                let response = client
                    .request(request)
                    .await?
                    .transaction
                    .ok_or_else(|| anyhow!("missing transaction"))?;
                Self::deserialize_project_transaction(this, response, push_to_history, cx).await
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
                    edits: edits.into_iter().map(Edit::Plain).collect(),
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
                                op.text_document.uri.clone(),
                                language_server.server_id(),
                                lsp_adapter.name.clone(),
                                cx,
                            )
                        })?
                        .await?;

                    let edits = this
                        .update(cx, |this, cx| {
                            let path = buffer_to_edit.read(cx).project_path(cx);
                            let active_entry = this.active_entry;
                            let is_active_entry = path.clone().map_or(false, |project_path| {
                                this.entry_for_path(&project_path, cx)
                                    .map_or(false, |entry| Some(entry.id) == active_entry)
                            });

                            let (mut edits, mut snippet_edits) = (vec![], vec![]);
                            for edit in op.edits {
                                match edit {
                                    Edit::Plain(edit) => edits.push(edit),
                                    Edit::Annotated(edit) => edits.push(edit.text_edit),
                                    Edit::Snippet(edit) => {
                                        let Ok(snippet) = Snippet::parse(&edit.snippet.value)
                                        else {
                                            continue;
                                        };

                                        if is_active_entry {
                                            snippet_edits.push((edit.range, snippet));
                                        } else {
                                            // Since this buffer is not focused, apply a normal edit.
                                            edits.push(TextEdit {
                                                range: edit.range,
                                                new_text: snippet.text,
                                            });
                                        }
                                    }
                                }
                            }
                            if !snippet_edits.is_empty() {
                                if let Some(buffer_version) = op.text_document.version {
                                    let buffer_id = buffer_to_edit.read(cx).remote_id();
                                    // Check if the edit that triggered that edit has been made by this participant.
                                    let should_apply_edit = this
                                        .buffer_snapshots
                                        .get(&buffer_id)
                                        .and_then(|server_to_snapshots| {
                                            let all_snapshots = server_to_snapshots
                                                .get(&language_server.server_id())?;
                                            all_snapshots
                                                .binary_search_by_key(&buffer_version, |snapshot| {
                                                    snapshot.version
                                                })
                                                .ok()
                                                .and_then(|index| all_snapshots.get(index))
                                        })
                                        .map_or(false, |lsp_snapshot| {
                                            let version = lsp_snapshot.snapshot.version();
                                            let most_recent_edit = version
                                                .iter()
                                                .max_by_key(|timestamp| timestamp.value);
                                            most_recent_edit.map_or(false, |edit| {
                                                edit.replica_id == this.replica_id()
                                            })
                                        });
                                    if should_apply_edit {
                                        cx.emit(Event::SnippetEdit(buffer_id, snippet_edits));
                                    }
                                }
                            }

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
        &mut self,
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
        &mut self,
        buffer: Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Range<Anchor>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.prepare_rename_impl(buffer, position, cx)
    }

    fn perform_rename_impl(
        &mut self,
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
        &mut self,
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
        &mut self,
        buffer: Model<Buffer>,
        position: PointUtf16,
        trigger: String,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        let options = buffer.update(cx, |buffer, cx| {
            lsp_command::lsp_formatting_options(language_settings(
                buffer.language_at(position).as_ref(),
                buffer.file(),
                cx,
            ))
        });
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            OnTypeFormatting {
                position,
                trigger,
                options,
                push_to_history,
            },
            cx,
        )
    }

    pub fn on_type_format<T: ToPointUtf16>(
        &mut self,
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
        &mut self,
        buffer_handle: Model<Buffer>,
        range: Range<T>,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<Vec<InlayHint>>> {
        let buffer = buffer_handle.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);
        self.inlay_hints_impl(buffer_handle, range, cx)
    }
    fn inlay_hints_impl(
        &mut self,
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
            if !InlayHints::can_resolve_inlays(&lang_server.capabilities()) {
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
    ) -> Receiver<SearchResult> {
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
                    let _ = tx.send(SearchResult::Buffer { buffer, ranges }).await;
                }

                if response.limit_reached {
                    let _ = tx.send(SearchResult::LimitReached).await;
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
    ) -> Receiver<SearchResult> {
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
                let tree = tree.read(cx);
                Some((tree.snapshot(), tree.as_local()?.settings()))
            })
            .collect::<Vec<_>>();
        let include_root = snapshots.len() > 1;

        let background = cx.background_executor().clone();
        let path_count: usize = snapshots
            .iter()
            .map(|(snapshot, _)| {
                if query.include_ignored() {
                    snapshot.file_count()
                } else {
                    snapshot.visible_file_count()
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
        let opened_buffers = self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store
                .buffers()
                .filter_map(|buffer| {
                    let (is_ignored, snapshot) = buffer.update(cx, |buffer, cx| {
                        let is_ignored = buffer
                            .project_path(cx)
                            .and_then(|path| self.entry_for_path(&path, cx))
                            .map_or(false, |entry| entry.is_ignored);
                        (is_ignored, buffer.snapshot())
                    });
                    if is_ignored && !query.include_ignored() {
                        return None;
                    } else if let Some(file) = snapshot.file() {
                        let matched_path = if include_root {
                            query.file_matches(Some(&file.full_path(cx)))
                        } else {
                            query.file_matches(Some(file.path()))
                        };

                        if matched_path {
                            Some((file.path().clone(), (buffer, snapshot)))
                        } else {
                            None
                        }
                    } else {
                        unnamed_files.push(buffer);
                        None
                    }
                })
                .collect()
        });
        cx.background_executor()
            .spawn(Self::background_search(
                unnamed_files,
                opened_buffers,
                cx.background_executor().clone(),
                self.fs.clone(),
                workers,
                query.clone(),
                include_root,
                path_count,
                snapshots,
                matching_paths_tx,
            ))
            .detach();

        let (result_tx, result_rx) = smol::channel::bounded(1024);

        cx.spawn(|this, mut cx| async move {
            const MAX_SEARCH_RESULT_FILES: usize = 5_000;
            const MAX_SEARCH_RESULT_RANGES: usize = 10_000;

            let mut matching_paths = matching_paths_rx
                .take(MAX_SEARCH_RESULT_FILES + 1)
                .collect::<Vec<_>>()
                .await;
            let mut limit_reached = if matching_paths.len() > MAX_SEARCH_RESULT_FILES {
                matching_paths.pop();
                true
            } else {
                false
            };
            cx.update(|cx| {
                sort_search_matches(&mut matching_paths, cx);
            })?;

            let mut range_count = 0;
            let query = Arc::new(query);

            // Now that we know what paths match the query, we will load at most
            // 64 buffers at a time to avoid overwhelming the main thread. For each
            // opened buffer, we will spawn a background task that retrieves all the
            // ranges in the buffer matched by the query.
            'outer: for matching_paths_chunk in matching_paths.chunks(64) {
                let mut chunk_results = Vec::new();
                for matching_path in matching_paths_chunk {
                    let query = query.clone();
                    let buffer = match matching_path {
                        SearchMatchCandidate::OpenBuffer { buffer, .. } => {
                            Task::ready(Ok(buffer.clone()))
                        }
                        SearchMatchCandidate::Path {
                            worktree_id, path, ..
                        } => this.update(&mut cx, |this, cx| {
                            this.open_buffer((*worktree_id, path.clone()), cx)
                        })?,
                    };

                    chunk_results.push(cx.spawn(|cx| async move {
                        let buffer = buffer.await?;
                        let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot())?;
                        let ranges = cx
                            .background_executor()
                            .spawn(async move {
                                query
                                    .search(&snapshot, None)
                                    .await
                                    .iter()
                                    .map(|range| {
                                        snapshot.anchor_before(range.start)
                                            ..snapshot.anchor_after(range.end)
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .await;
                        anyhow::Ok((buffer, ranges))
                    }));
                }

                let chunk_results = futures::future::join_all(chunk_results).await;
                for result in chunk_results {
                    if let Some((buffer, ranges)) = result.log_err() {
                        range_count += ranges.len();
                        result_tx
                            .send(SearchResult::Buffer { buffer, ranges })
                            .await?;
                        if range_count > MAX_SEARCH_RESULT_RANGES {
                            limit_reached = true;
                            break 'outer;
                        }
                    }
                }
            }

            if limit_reached {
                result_tx.send(SearchResult::LimitReached).await?;
            }

            anyhow::Ok(())
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
        include_root: bool,
        path_count: usize,
        snapshots: Vec<(Snapshot, WorktreeSettings)>,
        matching_paths_tx: Sender<SearchMatchCandidate>,
    ) {
        let fs = &fs;
        let query = &query;
        let matching_paths_tx = &matching_paths_tx;
        let snapshots = &snapshots;
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

        let paths_per_worker = (path_count + workers - 1) / workers;

        executor
            .scoped(|scope| {
                let max_concurrent_workers = Arc::new(Semaphore::new(workers));

                for worker_ix in 0..workers {
                    let worker_start_ix = worker_ix * paths_per_worker;
                    let worker_end_ix = worker_start_ix + paths_per_worker;
                    let opened_buffers = opened_buffers.clone();
                    let limiter = Arc::clone(&max_concurrent_workers);
                    scope.spawn({
                        async move {
                            let _guard = limiter.acquire().await;
                            search_snapshots(
                                snapshots,
                                worker_start_ix,
                                worker_end_ix,
                                query,
                                matching_paths_tx,
                                &opened_buffers,
                                include_root,
                                fs,
                            )
                            .await;
                        }
                    });
                }

                if query.include_ignored() {
                    for (snapshot, settings) in snapshots {
                        for ignored_entry in snapshot.entries(true, 0).filter(|e| e.is_ignored) {
                            let limiter = Arc::clone(&max_concurrent_workers);
                            scope.spawn(async move {
                                let _guard = limiter.acquire().await;
                                search_ignored_entry(
                                    snapshot,
                                    settings,
                                    ignored_entry,
                                    fs,
                                    query,
                                    matching_paths_tx,
                                )
                                .await;
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
                let status = request.status();
                return cx.spawn(move |this, cx| async move {
                    if !request.check_capabilities(language_server.adapter_server_capabilities()) {
                        return Ok(Default::default());
                    }

                    let lsp_request = language_server.request::<R::LspRequest>(lsp_params);

                    let id = lsp_request.id();
                    let _cleanup = if status.is_some() {
                        cx.update(|cx| {
                            this.update(cx, |this, cx| {
                                this.on_lsp_work_start(
                                    language_server.server_id(),
                                    id.to_string(),
                                    LanguageServerProgress {
                                        is_disk_based_diagnostics_progress: false,
                                        is_cancellable: false,
                                        title: None,
                                        message: status.clone(),
                                        percentage: None,
                                        last_update_at: cx.background_executor().now(),
                                    },
                                    cx,
                                );
                            })
                        })
                        .log_err();

                        Some(defer(|| {
                            cx.update(|cx| {
                                this.update(cx, |this, cx| {
                                    this.on_lsp_work_end(
                                        language_server.server_id(),
                                        id.to_string(),
                                        cx,
                                    );
                                })
                            })
                            .log_err();
                        }))
                    } else {
                        None
                    };

                    let result = lsp_request.await;

                    let response = result.map_err(|err| {
                        log::warn!(
                            "Generic lsp request to {} failed: {}",
                            language_server.name(),
                            err
                        );
                        err
                    })?;

                    request
                        .response_from_lsp(
                            response,
                            this.upgrade().ok_or_else(|| anyhow!("no app context"))?,
                            buffer_handle,
                            language_server.server_id(),
                            cx.clone(),
                        )
                        .await
                });
            }
        } else if let Some(project_id) = self.remote_id() {
            return self.send_lsp_proto_request(buffer_handle, project_id, request, cx);
        }

        Task::ready(Ok(Default::default()))
    }

    fn request_multiple_lsp_locally<P, R>(
        &self,
        buffer: &Model<Buffer>,
        position: Option<P>,
        request: R,
        cx: &mut ModelContext<'_, Self>,
    ) -> Task<Vec<R::Response>>
    where
        P: ToOffset,
        R: LspCommand + Clone,
        <R::LspRequest as lsp::request::Request>::Result: Send,
        <R::LspRequest as lsp::request::Request>::Params: Send,
    {
        if !self.is_local() {
            debug_panic!("Should not request multiple lsp commands in non-local project");
            return Task::ready(Vec::new());
        }
        let snapshot = buffer.read(cx).snapshot();
        let scope = position.and_then(|position| snapshot.language_scope_at(position));
        let mut response_results = self
            .language_servers_for_buffer(buffer.read(cx), cx)
            .filter(|(adapter, _)| {
                scope
                    .as_ref()
                    .map(|scope| scope.language_allowed(&adapter.name))
                    .unwrap_or(true)
            })
            .map(|(_, server)| server.server_id())
            .map(|server_id| {
                self.request_lsp(
                    buffer.clone(),
                    LanguageServerToQuery::Other(server_id),
                    request.clone(),
                    cx,
                )
            })
            .collect::<FuturesUnordered<_>>();

        return cx.spawn(|_, _| async move {
            let mut responses = Vec::with_capacity(response_results.len());
            while let Some(response_result) = response_results.next().await {
                if let Some(response) = response_result.log_err() {
                    responses.push(response);
                }
            }
            responses
        });
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

    /// Move a worktree to a new position in the worktree order.
    ///
    /// The worktree will moved to the opposite side of the destination worktree.
    ///
    /// # Example
    ///
    /// Given the worktree order `[11, 22, 33]` and a call to move worktree `22` to `33`,
    /// worktree_order will be updated to produce the indexes `[11, 33, 22]`.
    ///
    /// Given the worktree order `[11, 22, 33]` and a call to move worktree `22` to `11`,
    /// worktree_order will be updated to produce the indexes `[22, 11, 33]`.
    ///
    /// # Errors
    ///
    /// An error will be returned if the worktree or destination worktree are not found.
    pub fn move_worktree(
        &mut self,
        source: WorktreeId,
        destination: WorktreeId,
        cx: &mut ModelContext<'_, Self>,
    ) -> Result<()> {
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.move_worktree(source, destination, cx)
        })
    }

    pub fn find_or_create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(Model<Worktree>, PathBuf)>> {
        let abs_path = abs_path.as_ref();
        if let Some((tree, relative_path)) = self.find_worktree(abs_path, cx) {
            Task::ready(Ok((tree, relative_path)))
        } else {
            let worktree = self.create_worktree(abs_path, visible, cx);
            cx.background_executor()
                .spawn(async move { Ok((worktree.await?, PathBuf::new())) })
        }
    }

    pub fn find_worktree(
        &self,
        abs_path: &Path,
        cx: &AppContext,
    ) -> Option<(Model<Worktree>, PathBuf)> {
        self.worktree_store.read_with(cx, |worktree_store, cx| {
            for tree in worktree_store.worktrees() {
                if let Ok(relative_path) = abs_path.strip_prefix(tree.read(cx).abs_path()) {
                    return Some((tree.clone(), relative_path.into()));
                }
            }
            None
        })
    }

    pub fn is_shared(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Shared { .. } => true,
            ProjectClientState::Local => false,
            ProjectClientState::Remote { in_room, .. } => *in_room,
        }
    }

    pub fn list_directory(
        &self,
        query: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<PathBuf>>> {
        if self.is_local() {
            DirectoryLister::Local(self.fs.clone()).list_directory(query, cx)
        } else if let Some(dev_server) = self.dev_server_project_id().and_then(|id| {
            dev_server_projects::Store::global(cx)
                .read(cx)
                .dev_server_for_project(id)
        }) {
            let request = proto::ListRemoteDirectory {
                dev_server_id: dev_server.id.0,
                path: query,
            };
            let response = self.client.request(request);
            cx.background_executor().spawn(async move {
                let response = response.await?;
                Ok(response.entries.into_iter().map(PathBuf::from).collect())
            })
        } else {
            Task::ready(Err(anyhow!("cannot list directory in remote project")))
        }
    }

    fn create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>>> {
        let path: Arc<Path> = abs_path.as_ref().into();
        if !self.loading_worktrees.contains_key(&path) {
            let task = if self.ssh_session.is_some() {
                self.create_ssh_worktree(abs_path, visible, cx)
            } else if self.is_local() {
                self.create_local_worktree(abs_path, visible, cx)
            } else if self.dev_server_project_id.is_some() {
                self.create_dev_server_worktree(abs_path, cx)
            } else {
                return Task::ready(Err(anyhow!("not a local project")));
            };
            self.loading_worktrees.insert(path.clone(), task.shared());
        }
        let task = self.loading_worktrees.get(&path).unwrap().clone();
        cx.background_executor().spawn(async move {
            let result = match task.await {
                Ok(worktree) => Ok(worktree),
                Err(err) => Err(anyhow!("{}", err)),
            };
            result
        })
    }

    fn create_ssh_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>, Arc<anyhow::Error>>> {
        let ssh = self.ssh_session.clone().unwrap();
        let abs_path = abs_path.as_ref();
        let root_name = abs_path.file_name().unwrap().to_string_lossy().to_string();
        let path = abs_path.to_string_lossy().to_string();
        cx.spawn(|this, mut cx| async move {
            let response = ssh.request(AddWorktree { path: path.clone() }).await?;
            let worktree = cx.update(|cx| {
                Worktree::remote(
                    0,
                    0,
                    proto::WorktreeMetadata {
                        id: response.worktree_id,
                        root_name,
                        visible,
                        abs_path: path,
                    },
                    ssh.clone().into(),
                    cx,
                )
            })?;

            this.update(&mut cx, |this, cx| this.add_worktree(&worktree, cx))?;

            Ok(worktree)
        })
    }

    fn create_local_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>, Arc<anyhow::Error>>> {
        let fs = self.fs.clone();
        let next_entry_id = self.next_entry_id.clone();
        let path: Arc<Path> = abs_path.as_ref().into();

        cx.spawn(move |project, mut cx| async move {
            let worktree = Worktree::local(path.clone(), visible, fs, next_entry_id, &mut cx).await;

            project.update(&mut cx, |project, _| {
                project.loading_worktrees.remove(&path);
            })?;

            let worktree = worktree?;
            project.update(&mut cx, |project, cx| project.add_worktree(&worktree, cx))?;

            if visible {
                cx.update(|cx| {
                    cx.add_recent_document(&path);
                })
                .log_err();
            }

            Ok(worktree)
        })
    }

    fn create_dev_server_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Worktree>, Arc<anyhow::Error>>> {
        let client = self.client.clone();
        let path: Arc<Path> = abs_path.as_ref().into();
        let mut paths: Vec<String> = self
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().to_string())
            .collect();
        paths.push(path.to_string_lossy().to_string());
        let request = client.request(proto::UpdateDevServerProject {
            dev_server_project_id: self.dev_server_project_id.unwrap().0,
            paths,
        });

        let abs_path = abs_path.as_ref().to_path_buf();
        cx.spawn(move |project, mut cx| async move {
            let (tx, rx) = futures::channel::oneshot::channel();
            let tx = RefCell::new(Some(tx));
            let Some(project) = project.upgrade() else {
                return Err(anyhow!("project dropped"))?;
            };
            let observer = cx.update(|cx| {
                cx.observe(&project, move |project, cx| {
                    let abs_path = abs_path.clone();
                    project.update(cx, |project, cx| {
                        if let Some((worktree, _)) = project.find_worktree(&abs_path, cx) {
                            if let Some(tx) = tx.borrow_mut().take() {
                                tx.send(worktree).ok();
                            }
                        }
                    })
                })
            })?;

            request.await?;
            let worktree = rx.await.map_err(|e| anyhow!(e))?;
            drop(observer);
            project.update(&mut cx, |project, _| {
                project.loading_worktrees.remove(&path);
            })?;
            Ok(worktree)
        })
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut ModelContext<Self>) {
        if let Some(dev_server_project_id) = self.dev_server_project_id {
            let paths: Vec<String> = self
                .visible_worktrees(cx)
                .filter_map(|worktree| {
                    if worktree.read(cx).id() == id_to_remove {
                        None
                    } else {
                        Some(worktree.read(cx).abs_path().to_string_lossy().to_string())
                    }
                })
                .collect();
            if paths.len() > 0 {
                let request = self.client.request(proto::UpdateDevServerProject {
                    dev_server_project_id: dev_server_project_id.0,
                    paths,
                });
                cx.background_executor()
                    .spawn(request)
                    .detach_and_log_err(cx);
            }
            return;
        }
        self.diagnostics.remove(&id_to_remove);
        self.diagnostic_summaries.remove(&id_to_remove);
        self.cached_shell_environments.remove(&id_to_remove);

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
            self.language_server_watched_paths
                .remove(&server_id_to_remove);
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

        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.remove_worktree(id_to_remove, cx);
        });

        self.metadata_changed(cx);
    }

    fn add_worktree(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
        cx.observe(worktree, |_, _, cx| cx.notify()).detach();
        cx.subscribe(worktree, |this, worktree, event, cx| {
            let is_local = worktree.read(cx).is_local();
            match event {
                worktree::Event::UpdatedEntries(changes) => {
                    if is_local {
                        this.update_local_worktree_language_servers(&worktree, changes, cx);
                        this.update_local_worktree_settings(&worktree, changes, cx);
                        this.update_prettier_settings(&worktree, changes, cx);
                    }

                    cx.emit(Event::WorktreeUpdatedEntries(
                        worktree.read(cx).id(),
                        changes.clone(),
                    ));

                    let worktree_id = worktree.update(cx, |worktree, _| worktree.id());
                    this.client()
                        .telemetry()
                        .report_discovered_project_events(worktree_id, changes);
                }
                worktree::Event::UpdatedGitRepositories(_) => {
                    cx.emit(Event::WorktreeUpdatedGitRepositories);
                }
                worktree::Event::DeletedEntry(id) => cx.emit(Event::DeletedEntry(*id)),
            }
        })
        .detach();

        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.add(worktree, cx);
        });
        self.metadata_changed(cx);
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
            if let Some(LanguageServerState::Running { server, .. }) =
                self.language_servers.get(server_id)
            {
                if let Some(watched_paths) = self
                    .language_server_watched_paths
                    .get(&server_id)
                    .and_then(|paths| paths.get(&worktree_id))
                {
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

    fn update_local_worktree_settings(
        &mut self,
        worktree: &Model<Worktree>,
        changes: &UpdatedEntriesSet,
        cx: &mut ModelContext<Self>,
    ) {
        if worktree.read(cx).is_remote() {
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

            if path.ends_with(local_settings_file_relative_path()) {
                let settings_dir = Arc::from(
                    path.ancestors()
                        .nth(local_settings_file_relative_path().components().count())
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
            } else if path.ends_with(local_tasks_file_relative_path()) {
                self.task_inventory().update(cx, |task_inventory, cx| {
                    if removed {
                        task_inventory.remove_local_static_source(&abs_path);
                    } else {
                        let fs = self.fs.clone();
                        let task_abs_path = abs_path.clone();
                        let tasks_file_rx =
                            watch_config_file(&cx.background_executor(), fs, task_abs_path);
                        task_inventory.add_source(
                            TaskSourceKind::Worktree {
                                id: remote_worktree_id,
                                abs_path,
                                id_base: "local_tasks_for_worktree".into(),
                            },
                            |tx, cx| StaticSource::new(TrackedFile::new(tasks_file_rx, tx, cx)),
                            cx,
                        );
                    }
                })
            } else if path.ends_with(local_vscode_tasks_file_relative_path()) {
                self.task_inventory().update(cx, |task_inventory, cx| {
                    if removed {
                        task_inventory.remove_local_static_source(&abs_path);
                    } else {
                        let fs = self.fs.clone();
                        let task_abs_path = abs_path.clone();
                        let tasks_file_rx =
                            watch_config_file(&cx.background_executor(), fs, task_abs_path);
                        task_inventory.add_source(
                            TaskSourceKind::Worktree {
                                id: remote_worktree_id,
                                abs_path,
                                id_base: "local_vscode_tasks_for_worktree".into(),
                            },
                            |tx, cx| {
                                StaticSource::new(TrackedFile::new_convertible::<
                                    task::VsCodeTaskFile,
                                >(
                                    tasks_file_rx, tx, cx
                                ))
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
        for (_, _, path_summary) in self.diagnostic_summaries(include_ignored, cx) {
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
            .filter_map(|worktree| {
                let worktree = worktree.read(cx);
                Some((worktree, self.diagnostic_summaries.get(&worktree.id())?))
            })
            .flat_map(move |(worktree, summaries)| {
                let worktree_id = worktree.id();
                summaries
                    .iter()
                    .filter(move |(path, _)| {
                        include_ignored
                            || worktree
                                .entry_for_path(path.as_ref())
                                .map_or(false, |entry| !entry.is_ignored)
                    })
                    .flat_map(move |(path, summaries)| {
                        summaries.iter().map(move |(server_id, summary)| {
                            (
                                ProjectPath {
                                    worktree_id,
                                    path: path.clone(),
                                },
                                *server_id,
                                *summary,
                            )
                        })
                    })
            })
    }

    pub fn disk_based_diagnostics_started(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(language_server_status) =
            self.language_server_statuses.get_mut(&language_server_id)
        {
            language_server_status.has_pending_diagnostic_updates = true;
        }

        cx.emit(Event::DiskBasedDiagnosticsStarted { language_server_id });
        if self.is_local() {
            self.enqueue_buffer_ordered_message(BufferOrderedMessage::LanguageServerUpdate {
                language_server_id,
                message: proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(
                    Default::default(),
                ),
            })
            .ok();
        }
    }

    pub fn disk_based_diagnostics_finished(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(language_server_status) =
            self.language_server_statuses.get_mut(&language_server_id)
        {
            language_server_status.has_pending_diagnostic_updates = false;
        }

        cx.emit(Event::DiskBasedDiagnosticsFinished { language_server_id });

        if self.is_local() {
            self.enqueue_buffer_ordered_message(BufferOrderedMessage::LanguageServerUpdate {
                language_server_id,
                message: proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                    Default::default(),
                ),
            })
            .ok();
        }
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

    /// Attempts to find a `ProjectPath` corresponding to the given path. If the path
    /// is a *full path*, meaning it starts with the root name of a worktree, we'll locate
    /// it in that worktree. Otherwise, we'll attempt to find it as a relative path in
    /// the first visible worktree that has an entry for that relative path.
    ///
    /// We use this to resolve edit steps, when there's a chance an LLM may omit the workree
    /// root name from paths.
    ///
    /// # Arguments
    ///
    /// * `path` - A full path that starts with a worktree root name, or alternatively a
    ///            relative path within a visible worktree.
    /// * `cx` - A reference to the `AppContext`.
    ///
    /// # Returns
    ///
    /// Returns `Some(ProjectPath)` if a matching worktree is found, otherwise `None`.
    pub fn find_project_path(&self, path: &Path, cx: &AppContext) -> Option<ProjectPath> {
        let worktree_store = self.worktree_store.read(cx);

        for worktree in worktree_store.visible_worktrees(cx) {
            let worktree_root_name = worktree.read(cx).root_name();
            if let Ok(relative_path) = path.strip_prefix(worktree_root_name) {
                return Some(ProjectPath {
                    worktree_id: worktree.read(cx).id(),
                    path: relative_path.into(),
                });
            }
        }

        for worktree in worktree_store.visible_worktrees(cx) {
            let worktree = worktree.read(cx);
            if let Some(entry) = worktree.entry_for_path(path) {
                return Some(ProjectPath {
                    worktree_id: worktree.id(),
                    path: entry.path.clone(),
                });
            }
        }

        None
    }

    pub fn get_workspace_root(
        &self,
        project_path: &ProjectPath,
        cx: &AppContext,
    ) -> Option<PathBuf> {
        Some(
            self.worktree_for_id(project_path.worktree_id, cx)?
                .read(cx)
                .abs_path()
                .to_path_buf(),
        )
    }

    pub fn get_repo(
        &self,
        project_path: &ProjectPath,
        cx: &AppContext,
    ) -> Option<Arc<dyn GitRepository>> {
        self.worktree_for_id(project_path.worktree_id, cx)?
            .read(cx)
            .as_local()?
            .local_git_repo(&project_path.path)
    }

    pub fn get_first_worktree_root_repo(&self, cx: &AppContext) -> Option<Arc<dyn GitRepository>> {
        let worktree = self.visible_worktrees(cx).next()?.read(cx).as_local()?;
        let root_entry = worktree.root_git_entry()?;
        worktree.get_local_repo(&root_entry)?.repo().clone().into()
    }

    pub fn blame_buffer(
        &self,
        buffer: &Model<Buffer>,
        version: Option<clock::Global>,
        cx: &AppContext,
    ) -> Task<Result<Blame>> {
        self.buffer_store.read(cx).blame_buffer(buffer, version, cx)
    }

    // RPC message handlers

    async fn handle_multi_lsp_query(
        project: Model<Self>,
        envelope: TypedEnvelope<proto::MultiLspQuery>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::MultiLspQueryResponse> {
        let sender_id = envelope.original_sender_id()?;
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let version = deserialize_version(&envelope.payload.version);
        let buffer = project.update(&mut cx, |project, cx| {
            project.buffer_store.read(cx).get_existing(buffer_id)
        })??;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(version.clone())
            })?
            .await?;
        let buffer_version = buffer.update(&mut cx, |buffer, _| buffer.version())?;
        match envelope
            .payload
            .strategy
            .context("invalid request without the strategy")?
        {
            proto::multi_lsp_query::Strategy::All(_) => {
                // currently, there's only one multiple language servers query strategy,
                // so just ensure it's specified correctly
            }
        }
        match envelope.payload.request {
            Some(proto::multi_lsp_query::Request::GetHover(get_hover)) => {
                let get_hover =
                    GetHover::from_proto(get_hover, project.clone(), buffer.clone(), cx.clone())
                        .await?;
                let all_hovers = project
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_hover.position),
                            get_hover,
                            cx,
                        )
                    })?
                    .await
                    .into_iter()
                    .filter_map(|hover| remove_empty_hover_blocks(hover?));
                project.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: all_hovers
                        .map(|hover| proto::LspResponse {
                            response: Some(proto::lsp_response::Response::GetHoverResponse(
                                GetHover::response_to_proto(
                                    Some(hover),
                                    project,
                                    sender_id,
                                    &buffer_version,
                                    cx,
                                ),
                            )),
                        })
                        .collect(),
                })
            }
            Some(proto::multi_lsp_query::Request::GetCodeActions(get_code_actions)) => {
                let get_code_actions = GetCodeActions::from_proto(
                    get_code_actions,
                    project.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let all_actions = project
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_code_actions.range.start),
                            get_code_actions,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                project.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: all_actions
                        .map(|code_actions| proto::LspResponse {
                            response: Some(proto::lsp_response::Response::GetCodeActionsResponse(
                                GetCodeActions::response_to_proto(
                                    code_actions,
                                    project,
                                    sender_id,
                                    &buffer_version,
                                    cx,
                                ),
                            )),
                        })
                        .collect(),
                })
            }
            Some(proto::multi_lsp_query::Request::GetSignatureHelp(get_signature_help)) => {
                let get_signature_help = GetSignatureHelp::from_proto(
                    get_signature_help,
                    project.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let all_signatures = project
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_signature_help.position),
                            get_signature_help,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                project.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: all_signatures
                        .map(|signature_help| proto::LspResponse {
                            response: Some(
                                proto::lsp_response::Response::GetSignatureHelpResponse(
                                    GetSignatureHelp::response_to_proto(
                                        signature_help,
                                        project,
                                        sender_id,
                                        &buffer_version,
                                        cx,
                                    ),
                                ),
                            ),
                        })
                        .collect(),
                })
            }
            None => anyhow::bail!("empty multi lsp query request"),
        }
    }

    async fn handle_unshare_project(
        this: Model<Self>,
        _: TypedEnvelope<proto::UnshareProject>,
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
                this.buffer_store
                    .update(cx, |buffer_store, _| buffer_store.discard_incomplete());
                this.enqueue_buffer_ordered_message(BufferOrderedMessage::Resync)
                    .unwrap();
                cx.emit(Event::HostReshared);
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
            this.buffer_store.update(cx, |buffer_store, cx| {
                for buffer in buffer_store.buffers() {
                    buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
                }
            });
            this.shared_buffers.remove(&peer_id);

            cx.emit(Event::CollaboratorLeft(peer_id));
            cx.notify();
            Ok(())
        })?
    }

    async fn handle_update_project(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateProject>,
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

    async fn handle_update_diagnostic_summary(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateDiagnosticSummary>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            if let Some(message) = envelope.payload.summary {
                let project_path = ProjectPath {
                    worktree_id,
                    path: Path::new(&message.path).into(),
                };
                let path = project_path.path.clone();
                let server_id = LanguageServerId(message.language_server_id as usize);
                let summary = DiagnosticSummary {
                    error_count: message.error_count as usize,
                    warning_count: message.warning_count as usize,
                };

                if summary.is_empty() {
                    if let Some(worktree_summaries) =
                        this.diagnostic_summaries.get_mut(&worktree_id)
                    {
                        if let Some(summaries) = worktree_summaries.get_mut(&path) {
                            summaries.remove(&server_id);
                            if summaries.is_empty() {
                                worktree_summaries.remove(&path);
                            }
                        }
                    }
                } else {
                    this.diagnostic_summaries
                        .entry(worktree_id)
                        .or_default()
                        .entry(path)
                        .or_default()
                        .insert(server_id, summary);
                }
                cx.emit(Event::DiagnosticsUpdated {
                    language_server_id: LanguageServerId(message.language_server_id as usize),
                    path: project_path,
                });
            }
            Ok(())
        })?
    }

    async fn handle_start_language_server(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::StartLanguageServer>,
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
                            title: payload.title,
                            is_disk_based_diagnostics_progress: false,
                            is_cancellable: false,
                            message: payload.message,
                            percentage: payload.percentage.map(|p| p as usize),
                            last_update_at: cx.background_executor().now(),
                        },
                        cx,
                    );
                }

                proto::update_language_server::Variant::WorkProgress(payload) => {
                    this.on_lsp_work_progress(
                        language_server_id,
                        payload.token,
                        LanguageServerProgress {
                            title: None,
                            is_disk_based_diagnostics_progress: false,
                            is_cancellable: false,
                            message: payload.message,
                            percentage: payload.percentage.map(|p| p as usize),
                            last_update_at: cx.background_executor().now(),
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
        cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        let buffer_store = this.read_with(&cx, |this, cx| {
            if let Some(ssh) = &this.ssh_session {
                let mut payload = envelope.payload.clone();
                payload.project_id = 0;
                cx.background_executor()
                    .spawn(ssh.request(payload))
                    .detach_and_log_err(cx);
            }
            this.buffer_store.clone()
        })?;
        BufferStore::handle_update_buffer(buffer_store, envelope, cx).await
    }

    async fn handle_create_buffer_for_peer(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CreateBufferForPeer>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.handle_create_buffer_for_peer(
                    envelope,
                    this.replica_id(),
                    this.capability(),
                    cx,
                )
            })
        })?
    }

    async fn handle_reload_buffers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ReloadBuffers>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ReloadBuffersResponse> {
        let sender_id = envelope.original_sender_id()?;
        let reload = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(this.buffer_store.read(cx).get_existing(buffer_id)?);
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
                if let Some(buffer) = this.buffer_for_id(buffer_id, cx) {
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
                                file: Some(file.to_proto(cx)),
                            })
                            .log_err();
                    }

                    client
                        .send(proto::UpdateDiffBase {
                            project_id,
                            buffer_id: buffer_id.into(),
                            diff_base: buffer.diff_base().map(ToString::to_string),
                        })
                        .log_err();

                    client
                        .send(proto::BufferReloaded {
                            project_id,
                            buffer_id: buffer_id.into(),
                            version: language::proto::serialize_version(buffer.saved_version()),
                            mtime: buffer.saved_mtime().map(|time| time.into()),
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
        mut cx: AsyncAppContext,
    ) -> Result<proto::FormatBuffersResponse> {
        let sender_id = envelope.original_sender_id()?;
        let format = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(this.buffer_store.read(cx).get_existing(buffer_id)?);
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
        mut cx: AsyncAppContext,
    ) -> Result<proto::ApplyCompletionAdditionalEditsResponse> {
        let (buffer, completion) = this.update(&mut cx, |this, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            let buffer = this.buffer_store.read(cx).get_existing(buffer_id)?;
            let completion = Self::deserialize_completion(
                envelope
                    .payload
                    .completion
                    .ok_or_else(|| anyhow!("invalid completion"))?,
            )?;
            anyhow::Ok((buffer, completion))
        })??;

        let apply_additional_edits = this.update(&mut cx, |this, cx| {
            this.apply_additional_edits_for_completion(
                buffer,
                Completion {
                    old_range: completion.old_range,
                    new_text: completion.new_text,
                    lsp_completion: completion.lsp_completion,
                    server_id: completion.server_id,
                    documentation: None,
                    label: CodeLabel {
                        text: Default::default(),
                        runs: Default::default(),
                        filter_range: Default::default(),
                    },
                    confirm: None,
                },
                false,
                cx,
            )
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

        let mut documentation_is_markdown = false;
        let documentation = match completion.documentation {
            Some(lsp::Documentation::String(text)) => text,

            Some(lsp::Documentation::MarkupContent(lsp::MarkupContent { kind, value })) => {
                documentation_is_markdown = kind == lsp::MarkupKind::Markdown;
                value
            }

            _ => String::new(),
        };

        // If we have a new buffer_id, that means we're talking to a new client
        // and want to check for new text_edits in the completion too.
        let mut old_start = None;
        let mut old_end = None;
        let mut new_text = String::default();
        if let Ok(buffer_id) = BufferId::new(envelope.payload.buffer_id) {
            let buffer_snapshot = this.update(&mut cx, |this, cx| {
                let buffer = this.buffer_store.read(cx).get_existing(buffer_id)?;
                anyhow::Ok(buffer.read(cx).snapshot())
            })??;

            if let Some(text_edit) = completion.text_edit.as_ref() {
                let edit = parse_completion_text_edit(text_edit, &buffer_snapshot);

                if let Some((old_range, mut text_edit_new_text)) = edit {
                    LineEnding::normalize(&mut text_edit_new_text);

                    new_text = text_edit_new_text;
                    old_start = Some(serialize_anchor(&old_range.start));
                    old_end = Some(serialize_anchor(&old_range.end));
                }
            }
        }

        Ok(proto::ResolveCompletionDocumentationResponse {
            documentation,
            documentation_is_markdown,
            old_start,
            old_end,
            new_text,
        })
    }

    async fn handle_apply_code_action(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeAction>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ApplyCodeActionResponse> {
        let sender_id = envelope.original_sender_id()?;
        let action = Self::deserialize_code_action(
            envelope
                .payload
                .action
                .ok_or_else(|| anyhow!("invalid action"))?,
        )?;
        let apply_code_action = this.update(&mut cx, |this, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            let buffer = this.buffer_store.read(cx).get_existing(buffer_id)?;
            anyhow::Ok(this.apply_code_action(buffer, action, false, cx))
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
        mut cx: AsyncAppContext,
    ) -> Result<proto::OnTypeFormattingResponse> {
        let on_type_formatting = this.update(&mut cx, |this, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            let buffer = this.buffer_store.read(cx).get_existing(buffer_id)?;
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
        mut cx: AsyncAppContext,
    ) -> Result<proto::InlayHintsResponse> {
        let sender_id = envelope.original_sender_id()?;
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let buffer = this.update(&mut cx, |this, cx| {
            this.buffer_store.read(cx).get_existing(buffer_id)
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
        mut cx: AsyncAppContext,
    ) -> Result<proto::ResolveInlayHintResponse> {
        let proto_hint = envelope
            .payload
            .hint
            .expect("incorrect protobuf resolve inlay hint message: missing the inlay hint");
        let hint = InlayHints::proto_to_project_hint(proto_hint)
            .context("resolved proto inlay hint conversion")?;
        let buffer = this.update(&mut cx, |this, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            this.buffer_store.read(cx).get_existing(buffer_id)
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

    async fn handle_task_context_for_location(
        project: Model<Self>,
        envelope: TypedEnvelope<proto::TaskContextForLocation>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::TaskContext> {
        let location = envelope
            .payload
            .location
            .context("no location given for task context handling")?;
        let location = cx
            .update(|cx| deserialize_location(&project, location, cx))?
            .await?;
        let context_task = project.update(&mut cx, |project, cx| {
            let captured_variables = {
                let mut variables = TaskVariables::default();
                for range in location
                    .buffer
                    .read(cx)
                    .snapshot()
                    .runnable_ranges(location.range.clone())
                {
                    for (capture_name, value) in range.extra_captures {
                        variables.insert(VariableName::Custom(capture_name.into()), value);
                    }
                }
                variables
            };
            project.task_context_for_location(captured_variables, location, cx)
        })?;
        let task_context = context_task.await.unwrap_or_default();
        Ok(proto::TaskContext {
            project_env: task_context.project_env.into_iter().collect(),
            cwd: task_context
                .cwd
                .map(|cwd| cwd.to_string_lossy().to_string()),
            task_variables: task_context
                .task_variables
                .into_iter()
                .map(|(variable_name, variable_value)| (variable_name.to_string(), variable_value))
                .collect(),
        })
    }

    async fn handle_task_templates(
        project: Model<Self>,
        envelope: TypedEnvelope<proto::TaskTemplates>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::TaskTemplatesResponse> {
        let worktree = envelope.payload.worktree_id.map(WorktreeId::from_proto);
        let location = match envelope.payload.location {
            Some(location) => Some(
                cx.update(|cx| deserialize_location(&project, location, cx))?
                    .await
                    .context("task templates request location deserializing")?,
            ),
            None => None,
        };

        let templates = project
            .update(&mut cx, |project, cx| {
                project.task_templates(worktree, location, cx)
            })?
            .await
            .context("receiving task templates")?
            .into_iter()
            .map(|(kind, template)| {
                let kind = Some(match kind {
                    TaskSourceKind::UserInput => proto::task_source_kind::Kind::UserInput(
                        proto::task_source_kind::UserInput {},
                    ),
                    TaskSourceKind::Worktree {
                        id,
                        abs_path,
                        id_base,
                    } => {
                        proto::task_source_kind::Kind::Worktree(proto::task_source_kind::Worktree {
                            id: id.to_proto(),
                            abs_path: abs_path.to_string_lossy().to_string(),
                            id_base: id_base.to_string(),
                        })
                    }
                    TaskSourceKind::AbsPath { id_base, abs_path } => {
                        proto::task_source_kind::Kind::AbsPath(proto::task_source_kind::AbsPath {
                            abs_path: abs_path.to_string_lossy().to_string(),
                            id_base: id_base.to_string(),
                        })
                    }
                    TaskSourceKind::Language { name } => {
                        proto::task_source_kind::Kind::Language(proto::task_source_kind::Language {
                            name: name.to_string(),
                        })
                    }
                });
                let kind = Some(proto::TaskSourceKind { kind });
                let template = Some(proto::TaskTemplate {
                    label: template.label,
                    command: template.command,
                    args: template.args,
                    env: template.env.into_iter().collect(),
                    cwd: template.cwd,
                    use_new_terminal: template.use_new_terminal,
                    allow_concurrent_runs: template.allow_concurrent_runs,
                    reveal: match template.reveal {
                        RevealStrategy::Always => proto::RevealStrategy::RevealAlways as i32,
                        RevealStrategy::Never => proto::RevealStrategy::RevealNever as i32,
                    },
                    hide: match template.hide {
                        HideStrategy::Always => proto::HideStrategy::HideAlways as i32,
                        HideStrategy::Never => proto::HideStrategy::HideNever as i32,
                        HideStrategy::OnSuccess => proto::HideStrategy::HideOnSuccess as i32,
                    },
                    shell: Some(proto::Shell {
                        shell_type: Some(match template.shell {
                            Shell::System => proto::shell::ShellType::System(proto::System {}),
                            Shell::Program(program) => proto::shell::ShellType::Program(program),
                            Shell::WithArguments { program, args } => {
                                proto::shell::ShellType::WithArguments(
                                    proto::shell::WithArguments { program, args },
                                )
                            }
                        }),
                    }),
                    tags: template.tags,
                });
                proto::TemplatePair { kind, template }
            })
            .collect();

        Ok(proto::TaskTemplatesResponse { templates })
    }

    async fn try_resolve_code_action(
        lang_server: &LanguageServer,
        action: &mut CodeAction,
    ) -> anyhow::Result<()> {
        if GetCodeActions::can_resolve_actions(&lang_server.capabilities()) {
            if action.lsp_action.data.is_some()
                && (action.lsp_action.command.is_none() || action.lsp_action.edit.is_none())
            {
                action.lsp_action = lang_server
                    .request::<lsp::request::CodeActionResolveRequest>(action.lsp_action.clone())
                    .await?;
            }
        }

        anyhow::Ok(())
    }

    async fn execute_code_actions_on_servers(
        project: &WeakModel<Project>,
        adapters_and_servers: &Vec<(Arc<CachedLspAdapter>, Arc<LanguageServer>)>,
        code_actions: Vec<lsp::CodeActionKind>,
        buffer: &Model<Buffer>,
        push_to_history: bool,
        project_transaction: &mut ProjectTransaction,
        cx: &mut AsyncAppContext,
    ) -> Result<(), anyhow::Error> {
        for (lsp_adapter, language_server) in adapters_and_servers.iter() {
            let code_actions = code_actions.clone();

            let actions = project
                .update(cx, move |this, cx| {
                    let request = GetCodeActions {
                        range: text::Anchor::MIN..text::Anchor::MAX,
                        kinds: Some(code_actions),
                    };
                    let server = LanguageServerToQuery::Other(language_server.server_id());
                    this.request_lsp(buffer.clone(), server, request, cx)
                })?
                .await?;

            for mut action in actions {
                Self::try_resolve_code_action(&language_server, &mut action)
                    .await
                    .context("resolving a formatting code action")?;

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
                        cx,
                    )
                    .await?;
                    project_transaction.0.extend(new.0);
                }

                if let Some(command) = action.lsp_action.command {
                    project.update(cx, |this, _| {
                        this.last_workspace_edits_by_language_server
                            .remove(&language_server.server_id());
                    })?;

                    language_server
                        .request::<lsp::request::ExecuteCommand>(lsp::ExecuteCommandParams {
                            command: command.command,
                            arguments: command.arguments.unwrap_or_default(),
                            ..Default::default()
                        })
                        .await?;

                    project.update(cx, |this, _| {
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

        Ok(())
    }

    async fn handle_refresh_inlay_hints(
        this: Model<Self>,
        _: TypedEnvelope<proto::RefreshInlayHints>,
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
        mut cx: AsyncAppContext,
    ) -> Result<<T::ProtoRequest as proto::RequestMessage>::Response>
    where
        <T::LspRequest as lsp::request::Request>::Params: Send,
        <T::LspRequest as lsp::request::Request>::Result: Send,
    {
        let sender_id = envelope.original_sender_id()?;
        let buffer_id = T::buffer_id_from_proto(&envelope.payload)?;
        let buffer_handle = this.update(&mut cx, |this, cx| {
            this.buffer_store.read(cx).get_existing(buffer_id)
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
        mut cx: AsyncAppContext,
    ) -> Result<proto::SearchProjectResponse> {
        let peer_id = envelope.original_sender_id()?;
        let query = SearchQuery::from_proto(envelope.payload)?;
        let mut result = this.update(&mut cx, |this, cx| this.search(query, cx))?;

        cx.spawn(move |mut cx| async move {
            let mut locations = Vec::new();
            let mut limit_reached = false;
            while let Some(result) = result.next().await {
                match result {
                    SearchResult::Buffer { buffer, ranges } => {
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
                    SearchResult::LimitReached => limit_reached = true,
                }
            }
            Ok(proto::SearchProjectResponse {
                locations,
                limit_reached,
            })
        })
        .await
    }

    async fn handle_open_buffer_for_symbol(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::OpenBufferForSymbol>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferForSymbolResponse> {
        let peer_id = envelope.original_sender_id()?;
        let symbol = envelope
            .payload
            .symbol
            .ok_or_else(|| anyhow!("invalid symbol"))?;
        let symbol = Self::deserialize_symbol(symbol)?;
        let symbol = this.update(&mut cx, |this, _| {
            let signature = this.symbol_signature(&symbol.path);
            if signature == symbol.signature {
                Ok(symbol)
            } else {
                Err(anyhow!("invalid symbol signature"))
            }
        })??;
        let buffer = this
            .update(&mut cx, |this, cx| {
                this.open_buffer_for_symbol(
                    &Symbol {
                        language_server_name: symbol.language_server_name,
                        source_worktree_id: symbol.source_worktree_id,
                        path: symbol.path,
                        name: symbol.name,
                        kind: symbol.kind,
                        range: symbol.range,
                        signature: symbol.signature,
                        label: CodeLabel {
                            text: Default::default(),
                            runs: Default::default(),
                            filter_range: Default::default(),
                        },
                    },
                    cx,
                )
            })?
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

    async fn handle_open_new_buffer(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::OpenNewBuffer>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferResponse> {
        let buffer = this.update(&mut cx, |this, cx| this.create_local_buffer("", None, cx))?;
        let peer_id = envelope.original_sender_id()?;

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

    async fn deserialize_project_transaction(
        this: WeakModel<Self>,
        message: proto::ProjectTransaction,
        push_to_history: bool,
        mut cx: AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        let mut project_transaction = ProjectTransaction::default();
        for (buffer_id, transaction) in message.buffer_ids.into_iter().zip(message.transactions) {
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
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.wait_for_remote_buffer(id, cx)
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
                this.buffer_store.read(cx).buffer_version_info(cx)
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
                        if let Some(buffer) = this.buffer_for_id(buffer_id, cx) {
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
        self.metadata_changed(cx);
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.set_worktrees_from_proto(
                worktrees,
                self.replica_id(),
                self.remote_id().ok_or_else(|| anyhow!("invalid project"))?,
                self.client.clone().into(),
                cx,
            )
        })
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

    fn deserialize_symbol(serialized_symbol: proto::Symbol) -> Result<CoreSymbol> {
        let source_worktree_id = WorktreeId::from_proto(serialized_symbol.source_worktree_id);
        let worktree_id = WorktreeId::from_proto(serialized_symbol.worktree_id);
        let kind = unsafe { mem::transmute::<i32, lsp::SymbolKind>(serialized_symbol.kind) };
        let path = ProjectPath {
            worktree_id,
            path: PathBuf::from(serialized_symbol.path).into(),
        };

        let start = serialized_symbol
            .start
            .ok_or_else(|| anyhow!("invalid start"))?;
        let end = serialized_symbol
            .end
            .ok_or_else(|| anyhow!("invalid end"))?;
        Ok(CoreSymbol {
            language_server_name: LanguageServerName(serialized_symbol.language_server_name.into()),
            source_worktree_id,
            path,
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

    fn serialize_completion(completion: &CoreCompletion) -> proto::Completion {
        proto::Completion {
            old_start: Some(serialize_anchor(&completion.old_range.start)),
            old_end: Some(serialize_anchor(&completion.old_range.end)),
            new_text: completion.new_text.clone(),
            server_id: completion.server_id.0 as u64,
            lsp_completion: serde_json::to_vec(&completion.lsp_completion).unwrap(),
        }
    }

    fn deserialize_completion(completion: proto::Completion) -> Result<CoreCompletion> {
        let old_start = completion
            .old_start
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid old start"))?;
        let old_end = completion
            .old_end
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid old end"))?;
        let lsp_completion = serde_json::from_slice(&completion.lsp_completion)?;

        Ok(CoreCompletion {
            old_range: old_start..old_end,
            new_text: completion.new_text,
            server_id: LanguageServerId(completion.server_id as usize),
            lsp_completion,
        })
    }

    fn serialize_code_action(action: &CodeAction) -> proto::CodeAction {
        proto::CodeAction {
            server_id: action.server_id.0 as u64,
            start: Some(serialize_anchor(&action.range.start)),
            end: Some(serialize_anchor(&action.range.end)),
            lsp_action: serde_json::to_vec(&action.lsp_action).unwrap(),
        }
    }

    fn deserialize_code_action(action: proto::CodeAction) -> Result<CodeAction> {
        let start = action
            .start
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid start"))?;
        let end = action
            .end
            .and_then(deserialize_anchor)
            .ok_or_else(|| anyhow!("invalid end"))?;
        let lsp_action = serde_json::from_slice(&action.lsp_action)?;
        Ok(CodeAction {
            server_id: LanguageServerId(action.server_id as usize),
            range: start..end,
            lsp_action,
        })
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
    ) -> impl '_ + Iterator<Item = (&LanguageServerId, &LanguageServerName)> {
        self.supplementary_language_servers
            .iter()
            .map(|(id, (name, _))| (id, name))
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
        // The list of language servers is ordered based on the `language_servers` setting
        // for each language, thus we can consider the first one in the list to be the
        // primary one.
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

    pub fn task_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut ModelContext<'_, Project>,
    ) -> Task<Option<TaskContext>> {
        if self.is_local() {
            let (worktree_id, cwd) = if let Some(worktree) = self.task_worktree(cx) {
                (
                    Some(worktree.read(cx).id()),
                    Some(self.task_cwd(worktree, cx)),
                )
            } else {
                (None, None)
            };

            cx.spawn(|project, cx| async move {
                let mut task_variables = cx
                    .update(|cx| {
                        combine_task_variables(
                            captured_variables,
                            location,
                            BasicContextProvider::new(project.upgrade()?),
                            cx,
                        )
                        .log_err()
                    })
                    .ok()
                    .flatten()?;
                // Remove all custom entries starting with _, as they're not intended for use by the end user.
                task_variables.sweep();

                let mut project_env = None;
                if let Some((worktree_id, cwd)) = worktree_id.zip(cwd.as_ref()) {
                    let env = Self::get_worktree_shell_env(project, worktree_id, cwd, cx).await;
                    if let Some(env) = env {
                        project_env.replace(env);
                    }
                };

                Some(TaskContext {
                    project_env: project_env.unwrap_or_default(),
                    cwd,
                    task_variables,
                })
            })
        } else if let Some(project_id) = self
            .remote_id()
            .filter(|_| self.ssh_connection_string(cx).is_some())
        {
            let task_context = self.client().request(proto::TaskContextForLocation {
                project_id,
                location: Some(proto::Location {
                    buffer_id: location.buffer.read(cx).remote_id().into(),
                    start: Some(serialize_anchor(&location.range.start)),
                    end: Some(serialize_anchor(&location.range.end)),
                }),
            });
            cx.background_executor().spawn(async move {
                let task_context = task_context.await.log_err()?;
                Some(TaskContext {
                    project_env: task_context.project_env.into_iter().collect(),
                    cwd: task_context.cwd.map(PathBuf::from),
                    task_variables: task_context
                        .task_variables
                        .into_iter()
                        .filter_map(
                            |(variable_name, variable_value)| match variable_name.parse() {
                                Ok(variable_name) => Some((variable_name, variable_value)),
                                Err(()) => {
                                    log::error!("Unknown variable name: {variable_name}");
                                    None
                                }
                            },
                        )
                        .collect(),
                })
            })
        } else {
            Task::ready(None)
        }
    }

    async fn get_worktree_shell_env(
        this: WeakModel<Self>,
        worktree_id: WorktreeId,
        cwd: &PathBuf,
        mut cx: AsyncAppContext,
    ) -> Option<HashMap<String, String>> {
        let cached_env = this
            .update(&mut cx, |project, _| {
                project.cached_shell_environments.get(&worktree_id).cloned()
            })
            .ok()?;

        if let Some(env) = cached_env {
            Some(env)
        } else {
            let load_direnv = this
                .update(&mut cx, |_, cx| {
                    ProjectSettings::get_global(cx).load_direnv.clone()
                })
                .ok()?;

            let shell_env = cx
                .background_executor()
                .spawn({
                    let cwd = cwd.clone();
                    async move {
                        load_shell_environment(&cwd, &load_direnv)
                            .await
                            .unwrap_or_default()
                    }
                })
                .await;

            this.update(&mut cx, |project, _| {
                project
                    .cached_shell_environments
                    .insert(worktree_id, shell_env.clone());
            })
            .ok()?;

            Some(shell_env)
        }
    }

    pub fn task_templates(
        &self,
        worktree: Option<WorktreeId>,
        location: Option<Location>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<(TaskSourceKind, TaskTemplate)>>> {
        if self.is_local() {
            let (file, language) = location
                .map(|location| {
                    let buffer = location.buffer.read(cx);
                    (
                        buffer.file().cloned(),
                        buffer.language_at(location.range.start),
                    )
                })
                .unwrap_or_default();
            Task::ready(Ok(self
                .task_inventory()
                .read(cx)
                .list_tasks(file, language, worktree, cx)))
        } else if let Some(project_id) = self
            .remote_id()
            .filter(|_| self.ssh_connection_string(cx).is_some())
        {
            let remote_templates =
                self.query_remote_task_templates(project_id, worktree, location.as_ref(), cx);
            cx.background_executor().spawn(remote_templates)
        } else {
            Task::ready(Ok(Vec::new()))
        }
    }

    pub fn query_remote_task_templates(
        &self,
        project_id: u64,
        worktree: Option<WorktreeId>,
        location: Option<&Location>,
        cx: &AppContext,
    ) -> Task<Result<Vec<(TaskSourceKind, TaskTemplate)>>> {
        let client = self.client();
        let location = location.map(|location| serialize_location(location, cx));
        cx.spawn(|_| async move {
            let response = client
                .request(proto::TaskTemplates {
                    project_id,
                    worktree_id: worktree.map(|id| id.to_proto()),
                    location,
                })
                .await?;

            Ok(response
                .templates
                .into_iter()
                .filter_map(|template_pair| {
                    let task_source_kind = match template_pair.kind?.kind? {
                        proto::task_source_kind::Kind::UserInput(_) => TaskSourceKind::UserInput,
                        proto::task_source_kind::Kind::Worktree(worktree) => {
                            TaskSourceKind::Worktree {
                                id: WorktreeId::from_proto(worktree.id),
                                abs_path: PathBuf::from(worktree.abs_path),
                                id_base: Cow::Owned(worktree.id_base),
                            }
                        }
                        proto::task_source_kind::Kind::AbsPath(abs_path) => {
                            TaskSourceKind::AbsPath {
                                id_base: Cow::Owned(abs_path.id_base),
                                abs_path: PathBuf::from(abs_path.abs_path),
                            }
                        }
                        proto::task_source_kind::Kind::Language(language) => {
                            TaskSourceKind::Language {
                                name: language.name.into(),
                            }
                        }
                    };

                    let proto_template = template_pair.template?;
                    let reveal = match proto::RevealStrategy::from_i32(proto_template.reveal)
                        .unwrap_or(proto::RevealStrategy::RevealAlways)
                    {
                        proto::RevealStrategy::RevealAlways => RevealStrategy::Always,
                        proto::RevealStrategy::RevealNever => RevealStrategy::Never,
                    };
                    let hide = match proto::HideStrategy::from_i32(proto_template.hide)
                        .unwrap_or(proto::HideStrategy::HideNever)
                    {
                        proto::HideStrategy::HideAlways => HideStrategy::Always,
                        proto::HideStrategy::HideNever => HideStrategy::Never,
                        proto::HideStrategy::HideOnSuccess => HideStrategy::OnSuccess,
                    };
                    let shell = match proto_template
                        .shell
                        .and_then(|shell| shell.shell_type)
                        .unwrap_or(proto::shell::ShellType::System(proto::System {}))
                    {
                        proto::shell::ShellType::System(_) => Shell::System,
                        proto::shell::ShellType::Program(program) => Shell::Program(program),
                        proto::shell::ShellType::WithArguments(with_arguments) => {
                            Shell::WithArguments {
                                program: with_arguments.program,
                                args: with_arguments.args,
                            }
                        }
                    };
                    let task_template = TaskTemplate {
                        label: proto_template.label,
                        command: proto_template.command,
                        args: proto_template.args,
                        env: proto_template.env.into_iter().collect(),
                        cwd: proto_template.cwd,
                        use_new_terminal: proto_template.use_new_terminal,
                        allow_concurrent_runs: proto_template.allow_concurrent_runs,
                        reveal,
                        hide,
                        shell,
                        tags: proto_template.tags,
                    };
                    Some((task_source_kind, task_template))
                })
                .collect())
        })
    }

    fn task_worktree(&self, cx: &AppContext) -> Option<Model<Worktree>> {
        let available_worktrees = self
            .worktrees(cx)
            .filter(|worktree| {
                let worktree = worktree.read(cx);
                worktree.is_visible()
                    && worktree.is_local()
                    && worktree.root_entry().map_or(false, |e| e.is_dir())
            })
            .collect::<Vec<_>>();

        match available_worktrees.len() {
            0 => None,
            1 => Some(available_worktrees[0].clone()),
            _ => self.active_entry().and_then(|entry_id| {
                available_worktrees.into_iter().find_map(|worktree| {
                    if worktree.read(cx).contains_entry(entry_id) {
                        Some(worktree)
                    } else {
                        None
                    }
                })
            }),
        }
    }

    fn task_cwd(&self, worktree: Model<Worktree>, cx: &AppContext) -> PathBuf {
        worktree.read(cx).abs_path().to_path_buf()
    }
}

fn combine_task_variables(
    mut captured_variables: TaskVariables,
    location: Location,
    baseline: BasicContextProvider,
    cx: &mut AppContext,
) -> anyhow::Result<TaskVariables> {
    let language_context_provider = location
        .buffer
        .read(cx)
        .language()
        .and_then(|language| language.context_provider());
    let baseline = baseline
        .build_context(&captured_variables, &location, cx)
        .context("building basic default context")?;
    captured_variables.extend(baseline);
    if let Some(provider) = language_context_provider {
        captured_variables.extend(
            provider
                .build_context(&captured_variables, &location, cx)
                .context("building provider context")?,
        );
    }
    Ok(captured_variables)
}

async fn populate_labels_for_symbols(
    symbols: Vec<CoreSymbol>,
    language_registry: &Arc<LanguageRegistry>,
    default_language: Option<Arc<Language>>,
    lsp_adapter: Option<Arc<CachedLspAdapter>>,
    output: &mut Vec<Symbol>,
) {
    #[allow(clippy::mutable_key_type)]
    let mut symbols_by_language = HashMap::<Option<Arc<Language>>, Vec<CoreSymbol>>::default();

    let mut unknown_path = None;
    for symbol in symbols {
        let language = language_registry
            .language_for_file_path(&symbol.path.path)
            .await
            .ok()
            .or_else(|| {
                unknown_path.get_or_insert(symbol.path.path.clone());
                default_language.clone()
            });
        symbols_by_language
            .entry(language)
            .or_default()
            .push(symbol);
    }

    if let Some(unknown_path) = unknown_path {
        log::info!(
            "no language found for symbol path {}",
            unknown_path.display()
        );
    }

    let mut label_params = Vec::new();
    for (language, mut symbols) in symbols_by_language {
        label_params.clear();
        label_params.extend(
            symbols
                .iter_mut()
                .map(|symbol| (mem::take(&mut symbol.name), symbol.kind)),
        );

        let mut labels = Vec::new();
        if let Some(language) = language {
            let lsp_adapter = lsp_adapter
                .clone()
                .or_else(|| language_registry.lsp_adapters(&language).first().cloned());
            if let Some(lsp_adapter) = lsp_adapter {
                labels = lsp_adapter
                    .labels_for_symbols(&label_params, &language)
                    .await
                    .log_err()
                    .unwrap_or_default();
            }
        }

        for ((symbol, (name, _)), label) in symbols
            .into_iter()
            .zip(label_params.drain(..))
            .zip(labels.into_iter().chain(iter::repeat(None)))
        {
            output.push(Symbol {
                language_server_name: symbol.language_server_name,
                source_worktree_id: symbol.source_worktree_id,
                path: symbol.path,
                label: label.unwrap_or_else(|| CodeLabel::plain(name.clone(), None)),
                name,
                kind: symbol.kind,
                range: symbol.range,
                signature: symbol.signature,
            });
        }
    }
}

async fn populate_labels_for_completions(
    mut new_completions: Vec<CoreCompletion>,
    language_registry: &Arc<LanguageRegistry>,
    language: Option<Arc<Language>>,
    lsp_adapter: Option<Arc<CachedLspAdapter>>,
    completions: &mut Vec<Completion>,
) {
    let lsp_completions = new_completions
        .iter_mut()
        .map(|completion| mem::take(&mut completion.lsp_completion))
        .collect::<Vec<_>>();

    let labels = if let Some((language, lsp_adapter)) = language.as_ref().zip(lsp_adapter) {
        lsp_adapter
            .labels_for_completions(&lsp_completions, language)
            .await
            .log_err()
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    for ((completion, lsp_completion), label) in new_completions
        .into_iter()
        .zip(lsp_completions)
        .zip(labels.into_iter().chain(iter::repeat(None)))
    {
        let documentation = if let Some(docs) = &lsp_completion.documentation {
            Some(prepare_completion_documentation(docs, &language_registry, language.clone()).await)
        } else {
            None
        };

        completions.push(Completion {
            old_range: completion.old_range,
            new_text: completion.new_text,
            label: label.unwrap_or_else(|| {
                CodeLabel::plain(
                    lsp_completion.label.clone(),
                    lsp_completion.filter_text.as_deref(),
                )
            }),
            server_id: completion.server_id,
            documentation,
            lsp_completion,
            confirm: None,
        })
    }
}

fn deserialize_code_actions(code_actions: &HashMap<String, bool>) -> Vec<lsp::CodeActionKind> {
    code_actions
        .iter()
        .flat_map(|(kind, enabled)| {
            if *enabled {
                Some(kind.clone().into())
            } else {
                None
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
async fn search_snapshots(
    snapshots: &Vec<(Snapshot, WorktreeSettings)>,
    worker_start_ix: usize,
    worker_end_ix: usize,
    query: &SearchQuery,
    results_tx: &Sender<SearchMatchCandidate>,
    opened_buffers: &HashMap<Arc<Path>, (Model<Buffer>, BufferSnapshot)>,
    include_root: bool,
    fs: &Arc<dyn Fs>,
) {
    let mut snapshot_start_ix = 0;
    let mut abs_path = PathBuf::new();

    for (snapshot, _) in snapshots {
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
            let start_in_snapshot = worker_start_ix.saturating_sub(snapshot_start_ix);
            let end_in_snapshot = cmp::min(worker_end_ix, snapshot_end_ix) - snapshot_start_ix;

            for entry in snapshot
                .files(false, start_in_snapshot)
                .take(end_in_snapshot - start_in_snapshot)
            {
                if results_tx.is_closed() {
                    break;
                }
                if opened_buffers.contains_key(&entry.path) {
                    continue;
                }

                let matched_path = if include_root {
                    let mut full_path = PathBuf::from(snapshot.root_name());
                    full_path.push(&entry.path);
                    query.file_matches(Some(&full_path))
                } else {
                    query.file_matches(Some(&entry.path))
                };

                let matches = if matched_path {
                    abs_path.clear();
                    abs_path.push(&snapshot.abs_path());
                    abs_path.push(&entry.path);
                    if let Some(file) = fs.open_sync(&abs_path).await.log_err() {
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
                        is_file: entry.is_file(),
                    };
                    if results_tx.send(project_path).await.is_err() {
                        return;
                    }
                }
            }

            snapshot_start_ix = snapshot_end_ix;
        }
    }
}

async fn search_ignored_entry(
    snapshot: &Snapshot,
    settings: &WorktreeSettings,
    ignored_entry: &Entry,
    fs: &Arc<dyn Fs>,
    query: &SearchQuery,
    counter_tx: &Sender<SearchMatchCandidate>,
) {
    let mut ignored_paths_to_process =
        VecDeque::from([snapshot.abs_path().join(&ignored_entry.path)]);

    while let Some(ignored_abs_path) = ignored_paths_to_process.pop_front() {
        let metadata = fs
            .metadata(&ignored_abs_path)
            .await
            .with_context(|| format!("fetching fs metadata for {ignored_abs_path:?}"))
            .log_err()
            .flatten();

        if let Some(fs_metadata) = metadata {
            if fs_metadata.is_dir {
                let files = fs
                    .read_dir(&ignored_abs_path)
                    .await
                    .with_context(|| format!("listing ignored path {ignored_abs_path:?}"))
                    .log_err();

                if let Some(mut subfiles) = files {
                    while let Some(subfile) = subfiles.next().await {
                        if let Some(subfile) = subfile.log_err() {
                            ignored_paths_to_process.push_back(subfile);
                        }
                    }
                }
            } else if !fs_metadata.is_symlink {
                if !query.file_matches(Some(&ignored_abs_path))
                    || settings.is_path_excluded(&ignored_entry.path)
                {
                    continue;
                }
                let matches = if let Some(file) = fs
                    .open_sync(&ignored_abs_path)
                    .await
                    .with_context(|| format!("Opening ignored path {ignored_abs_path:?}"))
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
                                .expect("scanning worktree-related files"),
                        ),
                        is_ignored: true,
                        is_file: ignored_entry.is_file(),
                    };
                    if counter_tx.send(project_path).await.is_err() {
                        return;
                    }
                }
            }
        }
    }
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

pub struct PathMatchCandidateSet {
    pub snapshot: Snapshot,
    pub include_ignored: bool,
    pub include_root_name: bool,
    pub candidates: Candidates,
}

pub enum Candidates {
    /// Only consider directories.
    Directories,
    /// Only consider files.
    Files,
    /// Consider directories and files.
    Entries,
}

impl<'a> fuzzy::PathMatchCandidateSet<'a> for PathMatchCandidateSet {
    type Candidates = PathMatchCandidateSetIter<'a>;

    fn id(&self) -> usize {
        self.snapshot.id().to_usize()
    }

    fn len(&self) -> usize {
        match self.candidates {
            Candidates::Files => {
                if self.include_ignored {
                    self.snapshot.file_count()
                } else {
                    self.snapshot.visible_file_count()
                }
            }

            Candidates::Directories => {
                if self.include_ignored {
                    self.snapshot.dir_count()
                } else {
                    self.snapshot.visible_dir_count()
                }
            }

            Candidates::Entries => {
                if self.include_ignored {
                    self.snapshot.entry_count()
                } else {
                    self.snapshot.visible_entry_count()
                }
            }
        }
    }

    fn prefix(&self) -> Arc<str> {
        if self.snapshot.root_entry().map_or(false, |e| e.is_file()) {
            self.snapshot.root_name().into()
        } else if self.include_root_name {
            format!("{}/", self.snapshot.root_name()).into()
        } else {
            Arc::default()
        }
    }

    fn candidates(&'a self, start: usize) -> Self::Candidates {
        PathMatchCandidateSetIter {
            traversal: match self.candidates {
                Candidates::Directories => self.snapshot.directories(self.include_ignored, start),
                Candidates::Files => self.snapshot.files(self.include_ignored, start),
                Candidates::Entries => self.snapshot.entries(self.include_ignored, start),
            },
        }
    }
}

pub struct PathMatchCandidateSetIter<'a> {
    traversal: Traversal<'a>,
}

impl<'a> Iterator for PathMatchCandidateSetIter<'a> {
    type Item = fuzzy::PathMatchCandidate<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.traversal
            .next()
            .map(|entry| fuzzy::PathMatchCandidate {
                is_dir: entry.kind.is_dir(),
                path: &entry.path,
                char_bag: entry.char_bag,
            })
    }
}

impl EventEmitter<Event> for Project {}

impl<'a> Into<SettingsLocation<'a>> for &'a ProjectPath {
    fn into(self) -> SettingsLocation<'a> {
        SettingsLocation {
            worktree_id: self.worktree_id.to_usize(),
            path: self.path.as_ref(),
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

pub struct ProjectLspAdapterDelegate {
    project: WeakModel<Project>,
    worktree: worktree::Snapshot,
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    shell_env: Mutex<Option<HashMap<String, String>>>,
    load_direnv: DirenvSettings,
}

impl ProjectLspAdapterDelegate {
    pub fn new(
        project: &Project,
        worktree: &Model<Worktree>,
        cx: &ModelContext<Project>,
    ) -> Arc<Self> {
        let load_direnv = ProjectSettings::get_global(cx).load_direnv.clone();
        Arc::new(Self {
            project: cx.weak_model(),
            worktree: worktree.read(cx).snapshot(),
            fs: project.fs.clone(),
            http_client: project.client.http_client(),
            language_registry: project.languages.clone(),
            shell_env: Default::default(),
            load_direnv,
        })
    }

    async fn load_shell_env(&self) {
        let worktree_abs_path = self.worktree.abs_path();
        let shell_env = load_shell_environment(&worktree_abs_path, &self.load_direnv)
            .await
            .with_context(|| {
                format!("failed to determine load login shell environment in {worktree_abs_path:?}")
            })
            .log_err()
            .unwrap_or_default();
        *self.shell_env.lock() = Some(shell_env);
    }
}

#[async_trait]
impl LspAdapterDelegate for ProjectLspAdapterDelegate {
    fn show_notification(&self, message: &str, cx: &mut AppContext) {
        self.project
            .update(cx, |_, cx| cx.emit(Event::Notification(message.to_owned())))
            .ok();
    }

    fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    fn worktree_id(&self) -> u64 {
        self.worktree.id().to_proto()
    }

    fn worktree_root_path(&self) -> &Path {
        self.worktree.abs_path().as_ref()
    }

    async fn shell_env(&self) -> HashMap<String, String> {
        self.load_shell_env().await;
        self.shell_env.lock().as_ref().cloned().unwrap_or_default()
    }

    #[cfg(not(target_os = "windows"))]
    async fn which(&self, command: &OsStr) -> Option<PathBuf> {
        let worktree_abs_path = self.worktree.abs_path();
        self.load_shell_env().await;
        let shell_path = self
            .shell_env
            .lock()
            .as_ref()
            .and_then(|shell_env| shell_env.get("PATH").cloned());
        which::which_in(command, shell_path.as_ref(), &worktree_abs_path).ok()
    }

    #[cfg(target_os = "windows")]
    async fn which(&self, command: &OsStr) -> Option<PathBuf> {
        // todo(windows) Getting the shell env variables in a current directory on Windows is more complicated than other platforms
        //               there isn't a 'default shell' necessarily. The closest would be the default profile on the windows terminal
        //               SEE: https://learn.microsoft.com/en-us/windows/terminal/customize-settings/startup
        which::which(command).ok()
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
        kind: unsafe { mem::transmute::<lsp::SymbolKind, i32>(symbol.kind) },
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
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<Result<Model<Self>>>> {
        Some(project.update(cx, |project, cx| project.open_buffer(path.clone(), cx)))
    }

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

impl Completion {
    /// A key that can be used to sort completions when displaying
    /// them to the user.
    pub fn sort_key(&self) -> (usize, &str) {
        let kind_key = match self.lsp_completion.kind {
            Some(lsp::CompletionItemKind::KEYWORD) => 0,
            Some(lsp::CompletionItemKind::VARIABLE) => 1,
            _ => 2,
        };
        (kind_key, &self.label.text[self.label.filter_range.clone()])
    }

    /// Whether this completion is a snippet.
    pub fn is_snippet(&self) -> bool {
        self.lsp_completion.insert_text_format == Some(lsp::InsertTextFormat::SNIPPET)
    }
}

fn include_text(server: &lsp::LanguageServer) -> Option<bool> {
    match server.capabilities().text_document_sync.as_ref()? {
        lsp::TextDocumentSyncCapability::Kind(kind) => match kind {
            &lsp::TextDocumentSyncKind::NONE => None,
            &lsp::TextDocumentSyncKind::FULL => Some(true),
            &lsp::TextDocumentSyncKind::INCREMENTAL => Some(false),
            _ => None,
        },
        lsp::TextDocumentSyncCapability::Options(options) => match options.save.as_ref()? {
            lsp::TextDocumentSyncSaveOptions::Supported(supported) => {
                if *supported {
                    Some(true)
                } else {
                    None
                }
            }
            lsp::TextDocumentSyncSaveOptions::SaveOptions(save_options) => {
                Some(save_options.include_text.unwrap_or(false))
            }
        },
    }
}

async fn load_direnv_environment(dir: &Path) -> Result<Option<HashMap<String, String>>> {
    let Ok(direnv_path) = which::which("direnv") else {
        return Ok(None);
    };

    let direnv_output = smol::process::Command::new(direnv_path)
        .args(["export", "json"])
        .current_dir(dir)
        .output()
        .await
        .context("failed to spawn direnv to get local environment variables")?;

    anyhow::ensure!(
        direnv_output.status.success(),
        "direnv exited with error {:?}",
        direnv_output.status
    );

    let output = String::from_utf8_lossy(&direnv_output.stdout);
    if output.is_empty() {
        return Ok(None);
    }

    Ok(Some(
        serde_json::from_str(&output).context("failed to parse direnv output")?,
    ))
}

async fn load_shell_environment(
    dir: &Path,
    load_direnv: &DirenvSettings,
) -> Result<HashMap<String, String>> {
    let direnv_environment = match load_direnv {
        DirenvSettings::ShellHook => None,
        DirenvSettings::Direct => load_direnv_environment(dir).await?,
    }
    .unwrap_or(HashMap::default());

    let marker = "ZED_SHELL_START";
    let shell = env::var("SHELL").context(
        "SHELL environment variable is not assigned so we can't source login environment variables",
    )?;

    // What we're doing here is to spawn a shell and then `cd` into
    // the project directory to get the env in there as if the user
    // `cd`'d into it. We do that because tools like direnv, asdf, ...
    // hook into `cd` and only set up the env after that.
    //
    // If the user selects `Direct` for direnv, it would set an environment
    // variable that later uses to know that it should not run the hook.
    // We would include in `.envs` call so it is okay to run the hook
    // even if direnv direct mode is enabled.
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
        "cd '{}';{} printf '%s' {marker}; /usr/bin/env; exit 0;",
        dir.display(),
        additional_command.unwrap_or("")
    );

    let output = smol::process::Command::new(&shell)
        .args(["-i", "-c", &command])
        .envs(direnv_environment)
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

    parse_env_output(env_output, |key, value| {
        parsed_env.insert(key, value);
    });

    Ok(parsed_env)
}

fn remove_empty_hover_blocks(mut hover: Hover) -> Option<Hover> {
    hover
        .contents
        .retain(|hover_block| !hover_block.text.trim().is_empty());
    if hover.contents.is_empty() {
        None
    } else {
        Some(hover)
    }
}

#[derive(Debug)]
pub struct NoRepositoryError {}

impl std::fmt::Display for NoRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no git repository for worktree found")
    }
}

impl std::error::Error for NoRepositoryError {}

fn serialize_location(location: &Location, cx: &AppContext) -> proto::Location {
    proto::Location {
        buffer_id: location.buffer.read(cx).remote_id().into(),
        start: Some(serialize_anchor(&location.range.start)),
        end: Some(serialize_anchor(&location.range.end)),
    }
}

fn deserialize_location(
    project: &Model<Project>,
    location: proto::Location,
    cx: &mut AppContext,
) -> Task<Result<Location>> {
    let buffer_id = match BufferId::new(location.buffer_id) {
        Ok(id) => id,
        Err(e) => return Task::ready(Err(e)),
    };
    let buffer_task = project.update(cx, |project, cx| {
        project.wait_for_remote_buffer(buffer_id, cx)
    });
    cx.spawn(|_| async move {
        let buffer = buffer_task.await?;
        let start = location
            .start
            .and_then(deserialize_anchor)
            .context("missing task context location start")?;
        let end = location
            .end
            .and_then(deserialize_anchor)
            .context("missing task context location end")?;
        Ok(Location {
            buffer,
            range: start..end,
        })
    })
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize)]
pub struct DiagnosticSummary {
    pub error_count: usize,
    pub warning_count: usize,
}

impl DiagnosticSummary {
    pub fn new<'a, T: 'a>(diagnostics: impl IntoIterator<Item = &'a DiagnosticEntry<T>>) -> Self {
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

pub fn sort_worktree_entries(entries: &mut Vec<Entry>) {
    entries.sort_by(|entry_a, entry_b| {
        compare_paths(
            (&entry_a.path, entry_a.is_file()),
            (&entry_b.path, entry_b.is_file()),
        )
    });
}

fn sort_search_matches(search_matches: &mut Vec<SearchMatchCandidate>, cx: &AppContext) {
    search_matches.sort_by(|entry_a, entry_b| match (entry_a, entry_b) {
        (
            SearchMatchCandidate::OpenBuffer {
                buffer: buffer_a,
                path: None,
            },
            SearchMatchCandidate::OpenBuffer {
                buffer: buffer_b,
                path: None,
            },
        ) => buffer_a
            .read(cx)
            .remote_id()
            .cmp(&buffer_b.read(cx).remote_id()),
        (
            SearchMatchCandidate::OpenBuffer { path: None, .. },
            SearchMatchCandidate::Path { .. }
            | SearchMatchCandidate::OpenBuffer { path: Some(_), .. },
        ) => Ordering::Less,
        (
            SearchMatchCandidate::OpenBuffer { path: Some(_), .. }
            | SearchMatchCandidate::Path { .. },
            SearchMatchCandidate::OpenBuffer { path: None, .. },
        ) => Ordering::Greater,
        (
            SearchMatchCandidate::OpenBuffer {
                path: Some(path_a), ..
            },
            SearchMatchCandidate::Path {
                is_file: is_file_b,
                path: path_b,
                ..
            },
        ) => compare_paths((path_a.as_ref(), true), (path_b.as_ref(), *is_file_b)),
        (
            SearchMatchCandidate::Path {
                is_file: is_file_a,
                path: path_a,
                ..
            },
            SearchMatchCandidate::OpenBuffer {
                path: Some(path_b), ..
            },
        ) => compare_paths((path_a.as_ref(), *is_file_a), (path_b.as_ref(), true)),
        (
            SearchMatchCandidate::OpenBuffer {
                path: Some(path_a), ..
            },
            SearchMatchCandidate::OpenBuffer {
                path: Some(path_b), ..
            },
        ) => compare_paths((path_a.as_ref(), true), (path_b.as_ref(), true)),
        (
            SearchMatchCandidate::Path {
                worktree_id: worktree_id_a,
                is_file: is_file_a,
                path: path_a,
                ..
            },
            SearchMatchCandidate::Path {
                worktree_id: worktree_id_b,
                is_file: is_file_b,
                path: path_b,
                ..
            },
        ) => worktree_id_a.cmp(&worktree_id_b).then_with(|| {
            compare_paths((path_a.as_ref(), *is_file_a), (path_b.as_ref(), *is_file_b))
        }),
    });
}
