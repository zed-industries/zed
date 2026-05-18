pub mod agent_registry_store;
pub mod agent_server_store;
pub mod bookmark_store;
pub mod buffer_store;
pub mod color_extractor;
pub mod connection_manager;
pub mod context_server_store;
pub mod debounced_delay;
pub mod debugger;
pub mod git_store;
pub mod host;
pub mod image_store;
pub mod lsp_command;
pub mod lsp_store;
pub mod manifest_tree;
pub mod prettier_store;
pub mod project_search;
pub mod project_settings;
pub mod search;
pub mod task_inventory;
pub mod task_store;
pub mod telemetry_snapshot;
pub mod terminals;
pub mod toolchain_store;
pub mod trusted_worktrees;
pub mod worktree_store;

mod environment;
use buffer_diff::BufferDiff;
use context_server_store::{
    ContextServerStore, ContextServersChanged, registry::ContextServerDescriptorRegistry,
};
pub use environment::ProjectEnvironmentEvent;
use git::repository::get_git_committer;
use git_store::{GitStoreEvent, Repository, RepositoryId, RepositorySnapshot};
pub mod search_history;
pub mod yarn;

use dap::inline_value::{InlineValueLocation, VariableLookupKind, VariableScope};
use itertools::{Either, Itertools};

use crate::{
    bookmark_store::{BookmarkStore, SerializedBookmark},
    git_store::GitStore,
    lsp_store::{SymbolLocation, log_store::LogKind},
    project_search::SearchResultsHandle,
    trusted_worktrees::{PathTrust, RemoteHostLocation, TrustedWorktrees},
    worktree_store::WorktreeHandle,
};
pub use agent_registry_store::{AgentRegistryStore, RegistryAgent};
pub use agent_server_store::{AgentId, AgentServerStore, AgentServersUpdated, ExternalAgentSource};
pub use git_store::{
    ConflictRegion, ConflictSet, ConflictSetSnapshot, ConflictSetUpdate,
    git_traversal::{ChildEntriesGitIter, GitEntry, GitEntryRef, GitTraversal},
    linked_worktree_short_name, repo_identity_path, worktrees_directory_for_repo,
};
pub use manifest_tree::ManifestTree;
pub use project_search::{Search, SearchResults};
pub use worktree_store::WorktreePaths;

use anyhow::{Context as _, Result, anyhow};
use buffer_store::{BufferStore, BufferStoreEvent, PeerBufferAccess, SharedBuffer};
use client::{
    Client, Collaborator, PendingEntitySubscription, ProjectId, TypedEnvelope, UserStore, proto,
};
use clock::ReplicaId;

use dap::client::{DebugAdapterClient, SessionId};

use collections::{HashMap, HashSet, IndexSet, VecDeque};
use debounced_delay::DebouncedDelay;
pub use debugger::breakpoint_store::BreakpointWithPosition;
use debugger::{
    breakpoint_store::{
        ActiveStackFrame, BreakpointStore, BreakpointStoreEvent, BreakpointUpdatedReason,
        SourceBreakpoint,
    },
    dap_store::{DapStore, DapStoreEvent},
    session::Session,
};

pub use environment::ProjectEnvironment;

use futures::{
    StreamExt,
    channel::mpsc::{self, UnboundedReceiver},
    future::try_join_all,
};
pub use image_store::{ImageItem, ImageStore};
use image_store::{ImageItemEvent, ImageStoreEvent};

use ::git::{blame::Blame, status::FileStatus};
use gpui::{
    App, AppContext, AsyncApp, BorrowAppContext, Context, Entity, EventEmitter, Hsla, SharedString,
    Task, TaskExt, WeakEntity, Window,
};
use language::{
    Buffer, BufferEvent, Capability, CodeLabel, CursorShape, DiskState, Language, LanguageName,
    LanguageRegistry, PointUtf16, ToOffset, ToPointUtf16, Toolchain, ToolchainMetadata,
    ToolchainScope, Transaction, Unclipped, language_settings::InlayHintKind,
    proto::split_operations,
};
use lsp::{
    CodeActionKind, CompletionContext, CompletionItemKind, DocumentHighlightKind, InsertTextMode,
    LanguageServerBinary, LanguageServerId, LanguageServerName, LanguageServerSelector,
    MessageActionItem,
};
use lsp_command::*;
use lsp_store::{CompletionDocumentation, LspFormatTarget, OpenLspBufferHandle};
pub use manifest_tree::ManifestProvidersStore;
use node_runtime::NodeRuntime;
use parking_lot::Mutex;
pub use prettier_store::PrettierStore;
use project_settings::{ProjectSettings, SettingsObserver, SettingsObserverEvent};
#[cfg(target_os = "windows")]
use remote::wsl_path_to_windows_path;
use remote::{RemoteClient, RemoteConnectionOptions, same_remote_connection_identity};
use rpc::{
    AnyProtoClient, ErrorCode,
    proto::{LanguageServerPromptResponse, REMOTE_SERVER_PROJECT_ID},
};
use search::{SearchInputKind, SearchQuery, SearchResult};
use search_history::SearchHistory;
use settings::{InvalidSettingsError, RegisterSetting, Settings, SettingsLocation, SettingsStore};
use snippet::Snippet;
pub use snippet_provider;
use snippet_provider::SnippetProvider;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    ffi::OsString,
    future::Future,
    ops::{Not as _, Range},
    path::{Path, PathBuf},
    pin::pin,
    str::{self, FromStr},
    sync::Arc,
    time::Duration,
};

use task_store::TaskStore;
use terminals::Terminals;
use text::{Anchor, BufferId, Point, Rope};

use util::{
    ResultExt as _, TryFutureExt, debug_panic, maybe,
    path_list::PathList,
    paths::{PathStyle, SanitizedPath, is_absolute},
    rel_path::RelPath,
};
use worktree::{CreatedEntry, Snapshot, Traversal};
pub use worktree::{
    Entry, EntryKind, FS_WATCH_LATENCY, File, LocalWorktree, PathChange, ProjectEntryId,
    UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree, WorktreeId, WorktreeSettings,
    discover_root_repo_common_dir,
};
use worktree_store::{WorktreeStore, WorktreeStoreEvent};

pub use fs::*;
pub use language::Location;
#[cfg(any(test, feature = "test-support"))]
pub use prettier::FORMAT_SUFFIX as TEST_PRETTIER_FORMAT_SUFFIX;
#[cfg(any(test, feature = "test-support"))]
pub use prettier::RANGE_FORMAT_SUFFIX as TEST_PRETTIER_RANGE_FORMAT_SUFFIX;
use task::{DebugScenario, ResolvedTask, SharedTaskContext, TaskId};
pub use task_inventory::{
    BasicContextProvider, ContextProviderWithTasks, DebugScenarioContext, GIT_COMMAND_TASK_TAG,
    Inventory, InventoryEvent, TaskContexts, TaskSourceKind, TaskTemplateReload,
};

pub use buffer_store::ProjectTransaction;
pub use lsp_store::{
    DiagnosticSummary, InvalidationStrategy, LanguageServerLogType, LanguageServerProgress,
    LanguageServerPromptRequest, LanguageServerStatus, LanguageServerToQuery, LspStore,
    LspStoreEvent, ProgressToken, SERVER_PROGRESS_THROTTLE_TIMEOUT,
};
pub use toolchain_store::{ToolchainStore, Toolchains};
const MAX_PROJECT_SEARCH_HISTORY_SIZE: usize = 500;

#[derive(Clone, Copy, Debug)]
pub struct LocalProjectFlags {
    pub init_worktree_trust: bool,
    pub watch_global_configs: bool,
}

impl Default for LocalProjectFlags {
    fn default() -> Self {
        Self {
            init_worktree_trust: true,
            watch_global_configs: true,
        }
    }
}

pub trait ProjectItem: 'static {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>>
    where
        Self: Sized;
    fn entry_id(&self, cx: &App) -> Option<ProjectEntryId>;
    fn project_path(&self, cx: &App) -> Option<ProjectPath>;
    fn is_dirty(&self) -> bool;
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
    /// Machine-bound services and host-shaped stores. See [`Host`] for
    /// the rationale and Phase 1/2 split. Public host-shaped accessors
    /// (`project.lsp_store(cx)`, `project.fs(cx)`, etc.) all take
    /// `cx: &App` and forward to `self.host.read(cx).X.clone()`.
    /// `languages` and `collab_client` below are `Arc` duplicates kept
    /// directly on `Project` because they're cheap clones and are
    /// referenced often by internal code without `cx` already in scope;
    /// they will be migrated alongside Phase 2's host registry work.
    host: Entity<host::Host>,
    active_entry: Option<ProjectEntryId>,
    buffer_ordered_messages_tx: mpsc::UnboundedSender<BufferOrderedMessage>,
    languages: Arc<LanguageRegistry>,
    collab_client: Arc<client::Client>,
    join_project_response_message_id: u32,
    // todo lw explain the client_state x remote_client matrix, its super confusing
    client_state: ProjectClientState,
    collaborators: HashMap<proto::PeerId, Collaborator>,
    client_subscriptions: Vec<client::Subscription>,
    _subscriptions: Vec<gpui::Subscription>,
    buffers_needing_diff: HashSet<WeakEntity<Buffer>>,
    git_diff_debouncer: DebouncedDelay<Self>,
    remotely_created_models: Arc<Mutex<RemotelyCreatedModels>>,
    terminals: Terminals,
    search_history: SearchHistory,
    search_included_history: SearchHistory,
    search_excluded_history: SearchHistory,
    agent_location: Option<AgentLocation>,
    downloading_files: Arc<Mutex<HashMap<(WorktreeId, String), DownloadingFile>>>,
    last_worktree_paths: WorktreePaths,
    /// Per-project worktree retention. The host's `WorktreeStore` only holds
    /// weak references; this list keeps the worktrees alive for as long as
    /// the project needs them. Visible worktrees are always strong; invisible
    /// worktrees are strong only while the project is collab-shared.
    worktrees: Vec<WorktreeHandle>,
    /// While true, all worktrees (visible or not) are retained as strong
    /// handles. Toggled on `Project::shared` and off `Project::unshare`.
    retain_worktrees: bool,
    /// Last `RepositorySnapshot` that we sent downstream for each repository,
    /// used to compute incremental `proto::UpdateRepository` payloads when
    /// the host's `GitStore` reports a snapshot change. Populated on share,
    /// updated on each forwarded change, cleared on unshare.
    git_repository_snapshots_for_peer:
        HashMap<git_store::RepositoryId, git_store::RepositorySnapshot>,
    /// Per-peer per-buffer state tracking which buffers have been streamed
    /// to which collaborator. Used by `Project::create_buffer_for_peer` to
    /// avoid double-sending and by language-server-related buffer
    /// notifications. Populated by `create_buffer_for_peer`, mutated by
    /// `handle_synchronize_buffers` / `handle_close_buffer` /
    /// `register_shared_lsp_handle`. Moved here from `BufferStore` so that
    /// the host store has no per-project state.
    shared_buffers: HashMap<proto::PeerId, HashMap<BufferId, SharedBuffer>>,
    /// Per-project view of which buffers (by id) this Project considers
    /// "its own" out of the (potentially shared) host `BufferStore`.
    /// Populated idempotently by `on_buffer_store_event::BufferAdded`,
    /// pruned by `BufferDropped`. Every Project accessor that walks
    /// buffers (`opened_buffers`, `buffer_for_id`, `dirty_buffers`,
    /// `has_open_buffer`, `get_open_buffer`) filters through this set so
    /// that, in Phase 2 sharing, sibling-Project events for buffers we
    /// don't own become no-ops without changing the host store.
    buffers: HashSet<BufferId>,
    /// Per-project view of which git repositories (by id) this Project
    /// considers "its own" out of the (potentially shared) host
    /// `GitStore`. A repository is claimed when the host store fires
    /// `RepositoryAdded` and at least one of the repository's worktrees
    /// is in `self.worktrees`. Pruned by `RepositoryRemoved`. Phase 2
    /// will use this set to filter `Project::repositories(cx)` and the
    /// downstream-broadcast paths in `on_git_store_event`.
    repositories: HashSet<RepositoryId>,
    /// Per-project active repository. `GitStore` is host-shared, so
    /// active-repository selection must live on the tenant `Project`.
    active_repository_id: Option<RepositoryId>,
    /// Per-project view of which language servers (by id) this Project
    /// considers "its own" out of the (potentially shared) host
    /// `LspStore`. A server is claimed when the host store fires
    /// `LanguageServerAdded` and either (a) it has no associated worktree
    /// (a "global" server we always claim) or (b) its worktree is in
    /// `self.worktrees`. Pruned by `LanguageServerRemoved`. Phase 2 will
    /// use this set to gate the proto-broadcast paths in
    /// `on_lsp_store_event` so a shared LspStore doesn't produce duplicate
    /// `StartLanguageServer`/`RefreshInlayHints`/etc. broadcasts from
    /// each tenant Project.
    language_servers: HashSet<LanguageServerId>,
    /// Paths this Project has requested via `find_or_create_worktree`
    /// or `create_worktree` whose worktree-creation is still in flight.
    /// Consulted by `on_worktree_store_event::WorktreeAdded` to claim a
    /// just-added visible worktree synchronously — critical because
    /// downstream events fired by other host stores (notably
    /// `GitStore::RepositoryAdded`) race against the
    /// `find_or_create_worktree` continuation that would otherwise be
    /// the only place visible worktrees get claimed. Pruned by the
    /// continuation; the event handler doesn't prune (so a duplicate
    /// add for the same path remains claimable).
    pending_worktree_paths: HashSet<Arc<Path>>,
    /// Drives the context server maintain loop on this Project's behalf.
    /// Set when a refresh is in flight; cleared when the spawned task
    /// completes. Lives on `Project` (not `ContextServerStore`) so that
    /// the per-project `active_project_directory` can be threaded into
    /// `ContextServerStore::maintain_servers` as `root_path_override`.
    context_server_update_task: Option<Task<()>>,
    /// Set when an `available_context_servers_changed` call lands while
    /// `context_server_update_task` is already in flight; the in-flight
    /// task re-triggers itself on completion if this is `true`.
    context_server_needs_update: bool,
    /// Per-project LRU of recently-scheduled tasks. Used for the
    /// "recently used" section of the tasks picker and the
    /// `task::Rerun` action's "last task" lookup. Lives on `Project`
    /// (not the host-shared `Inventory`) so workspace A's task
    /// scheduling doesn't appear in workspace B's picker.
    last_scheduled_tasks: VecDeque<(TaskSourceKind, ResolvedTask)>,
    /// Per-project LRU of recently-scheduled debug scenarios. Same
    /// per-Project rationale as `last_scheduled_tasks`. Used by the
    /// debug rerun action and the debug-scenario picker.
    last_scheduled_scenarios: VecDeque<(DebugScenario, DebugScenarioContext)>,
    /// Per-project view of which DAP sessions (by id) this Project
    /// considers "its own" out of the shared host `DapStore`. A
    /// session is claimed when the Project initiates it via
    /// `Project::new_dap_session` and pruned on
    /// `DapStoreEvent::DebugClientShutdown`. Phase 2 uses this set
    /// to gate `on_dap_store_event` (collab broadcasts of session
    /// lifecycle / log messages / notifications) and to filter the
    /// `Project::dap_sessions` / `Project::dap_session_by_id`
    /// accessors so a sibling Project's debug sessions don't bleed
    /// into our UI.
    dap_sessions: HashSet<SessionId>,
}

struct DownloadingFile {
    destination_path: PathBuf,
    chunks: Vec<u8>,
    total_size: u64,
    file_id: Option<u64>, // Set when we receive the State message
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentLocation {
    pub buffer: WeakEntity<Buffer>,
    pub position: Anchor,
}

#[derive(Default)]
struct RemotelyCreatedModels {
    worktrees: Vec<Entity<Worktree>>,
    buffers: Vec<Entity<Buffer>>,
    retain_count: usize,
}

struct RemotelyCreatedModelGuard {
    remote_models: std::sync::Weak<Mutex<RemotelyCreatedModels>>,
}

impl Drop for RemotelyCreatedModelGuard {
    fn drop(&mut self) {
        if let Some(remote_models) = self.remote_models.upgrade() {
            let mut remote_models = remote_models.lock();
            assert!(
                remote_models.retain_count > 0,
                "RemotelyCreatedModelGuard dropped too many times"
            );
            remote_models.retain_count -= 1;
            if remote_models.retain_count == 0 {
                remote_models.buffers.clear();
                remote_models.worktrees.clear();
            }
        }
    }
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
        name: Option<LanguageServerName>,
    },
    Resync,
}

#[derive(Debug)]
enum ProjectClientState {
    /// Single-player mode.
    Local,
    /// Multi-player mode but still a local project.
    Shared { remote_id: u64 },
    /// Multi-player mode but working on a remote project.
    Collab {
        sharing_has_stopped: bool,
        capability: Capability,
        remote_id: u64,
        replica_id: ReplicaId,
    },
}

/// A link to display in a toast notification, useful to point to documentation.
#[derive(PartialEq, Debug, Clone)]
pub struct ToastLink {
    pub label: &'static str,
    pub url: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    LanguageServerAdded(LanguageServerId, LanguageServerName, Option<WorktreeId>),
    LanguageServerRemoved(LanguageServerId),
    LanguageServerLog(LanguageServerId, LanguageServerLogType, String),
    // [`lsp::notification::DidOpenTextDocument`] was sent to this server using the buffer data.
    // Zed's buffer-related data is updated accordingly.
    LanguageServerBufferRegistered {
        server_id: LanguageServerId,
        buffer_id: BufferId,
        buffer_abs_path: PathBuf,
        name: Option<LanguageServerName>,
    },
    ToggleLspLogs {
        server_id: LanguageServerId,
        enabled: bool,
        toggled_log_kind: LogKind,
    },
    Toast {
        notification_id: SharedString,
        message: String,
        /// Optional link to display as a button in the toast.
        link: Option<ToastLink>,
    },
    HideToast {
        notification_id: SharedString,
    },
    LanguageServerPrompt(LanguageServerPromptRequest),
    LanguageNotFound(Entity<Buffer>),
    ActiveEntryChanged(Option<ProjectEntryId>),
    ActivateProjectPanel,
    WorktreeAdded(WorktreeId),
    WorktreeOrderChanged,
    WorktreeRemoved(WorktreeId),
    ActiveRepositoryChanged(Option<RepositoryId>),
    WorktreeUpdatedEntries(WorktreeId, UpdatedEntriesSet),
    WorktreeUpdatedRootRepoCommonDir(WorktreeId),
    WorktreePathsChanged {
        old_worktree_paths: WorktreePaths,
    },
    DiskBasedDiagnosticsStarted {
        language_server_id: LanguageServerId,
    },
    DiskBasedDiagnosticsFinished {
        language_server_id: LanguageServerId,
    },
    DiagnosticsUpdated {
        paths: Vec<ProjectPath>,
        language_server_id: LanguageServerId,
    },
    RemoteIdChanged(Option<u64>),
    DisconnectedFromHost,
    DisconnectedFromRemote {
        server_not_running: bool,
    },
    Closed,
    DeletedEntry(WorktreeId, ProjectEntryId),
    CollaboratorUpdated {
        old_peer_id: proto::PeerId,
        new_peer_id: proto::PeerId,
    },
    CollaboratorJoined(proto::PeerId),
    CollaboratorLeft(proto::PeerId),
    HostReshared,
    Reshared,
    Rejoined,
    RefreshInlayHints {
        server_id: LanguageServerId,
        request_id: Option<usize>,
    },
    RefreshSemanticTokens {
        server_id: LanguageServerId,
        request_id: Option<usize>,
    },
    RefreshCodeLens,
    RevealInProjectPanel(ProjectEntryId),
    SnippetEdit(BufferId, Vec<(lsp::Range, Snippet)>),
    ExpandedAllForEntry(WorktreeId, ProjectEntryId),
    EntryRenamed(ProjectTransaction, ProjectPath, PathBuf),
    WorkspaceEditApplied(ProjectTransaction),
    AgentLocationChanged,
    BufferEdited,
}

pub struct AgentLocationChanged;

pub enum DebugAdapterClientState {
    Starting(Task<Option<Arc<DebugAdapterClient>>>),
    Running(Arc<DebugAdapterClient>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ProjectPath {
    pub worktree_id: WorktreeId,
    pub path: Arc<RelPath>,
}

impl ProjectPath {
    pub fn from_file(value: &dyn language::File, cx: &App) -> Self {
        ProjectPath {
            worktree_id: value.worktree_id(cx),
            path: value.path().clone(),
        }
    }

    pub fn from_proto(p: proto::ProjectPath) -> Option<Self> {
        Some(Self {
            worktree_id: WorktreeId::from_proto(p.worktree_id),
            path: RelPath::from_proto(&p.path).log_err()?,
        })
    }

    pub fn to_proto(&self) -> proto::ProjectPath {
        proto::ProjectPath {
            worktree_id: self.worktree_id.to_proto(),
            path: self.path.as_ref().to_proto(),
        }
    }

    pub fn root_path(worktree_id: WorktreeId) -> Self {
        Self {
            worktree_id,
            path: RelPath::empty().into(),
        }
    }

    pub fn starts_with(&self, other: &ProjectPath) -> bool {
        self.worktree_id == other.worktree_id && self.path.starts_with(&other.path)
    }
}

#[derive(Debug, Default)]
pub enum PrepareRenameResponse {
    Success(Range<Anchor>),
    OnlyUnpreparedRenameSupported,
    #[default]
    InvalidPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlayId {
    EditPrediction(usize),
    DebuggerValue(usize),
    // LSP
    Hint(usize),
    Color(usize),
    ReplResult(usize),
}

impl InlayId {
    pub fn id(&self) -> usize {
        match self {
            Self::EditPrediction(id) => *id,
            Self::DebuggerValue(id) => *id,
            Self::Hint(id) => *id,
            Self::Color(id) => *id,
            Self::ReplResult(id) => *id,
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

/// The user's intent behind a given completion confirmation.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum CompletionIntent {
    /// The user intends to 'commit' this result, if possible.
    /// Completion confirmations should run side effects.
    ///
    /// For LSP completions, will respect the setting `completions.lsp_insert_mode`.
    Complete,
    /// Similar to [Self::Complete], but behaves like `lsp_insert_mode` is set to `insert`.
    CompleteWithInsert,
    /// Similar to [Self::Complete], but behaves like `lsp_insert_mode` is set to `replace`.
    CompleteWithReplace,
    /// The user intends to continue 'composing' this completion.
    /// Completion confirmations should not run side effects and
    /// let the user continue composing their action.
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

/// Similar to `CoreCompletion`, but with extra metadata attached.
#[derive(Clone)]
pub struct Completion {
    /// The range of text that will be replaced by this completion.
    pub replace_range: Range<Anchor>,
    /// The new text that will be inserted.
    pub new_text: String,
    /// A label for this completion that is shown in the menu.
    pub label: CodeLabel,
    /// The documentation for this completion.
    pub documentation: Option<CompletionDocumentation>,
    /// Completion data source which it was constructed from.
    pub source: CompletionSource,
    /// A path to an icon for this completion that is shown in the menu.
    pub icon_path: Option<SharedString>,
    /// Text starting here and ending at the cursor will be used as the query for filtering this completion.
    ///
    /// If None, the start of the surrounding word is used.
    pub match_start: Option<text::Anchor>,
    /// Key used for de-duplicating snippets. If None, always considered unique.
    pub snippet_deduplication_key: Option<(usize, usize)>,
    /// Whether to adjust indentation (the default) or not.
    pub insert_text_mode: Option<InsertTextMode>,
    /// An optional callback to invoke when this completion is confirmed.
    /// Returns whether new completions should be retriggered after the current one.
    /// If `true` is returned, the editor will show a new completion menu after this completion is confirmed.
    /// if no confirmation is provided or `false` is returned, the completion will be committed.
    pub confirm: Option<Arc<dyn Send + Sync + Fn(CompletionIntent, &mut Window, &mut App) -> bool>>,
}

#[derive(Debug, Clone)]
pub enum CompletionSource {
    Lsp {
        /// The alternate `insert` range, if provided by the LSP server.
        insert_range: Option<Range<Anchor>>,
        /// The id of the language server that produced this completion.
        server_id: LanguageServerId,
        /// The raw completion provided by the language server.
        lsp_completion: Box<lsp::CompletionItem>,
        /// A set of defaults for this completion item.
        lsp_defaults: Option<Arc<lsp::CompletionListItemDefaults>>,
        /// Whether this completion has been resolved, to ensure it happens once per completion.
        resolved: bool,
    },
    Dap {
        /// The sort text for this completion.
        sort_text: String,
    },
    Custom,
    BufferWord {
        word_range: Range<Anchor>,
        resolved: bool,
    },
}

impl CompletionSource {
    pub fn server_id(&self) -> Option<LanguageServerId> {
        if let CompletionSource::Lsp { server_id, .. } = self {
            Some(*server_id)
        } else {
            None
        }
    }

    pub fn lsp_completion(&self, apply_defaults: bool) -> Option<Cow<'_, lsp::CompletionItem>> {
        if let Self::Lsp {
            lsp_completion,
            lsp_defaults,
            ..
        } = self
        {
            if apply_defaults && let Some(lsp_defaults) = lsp_defaults {
                let mut completion_with_defaults = *lsp_completion.clone();
                let default_commit_characters = lsp_defaults.commit_characters.as_ref();
                let default_edit_range = lsp_defaults.edit_range.as_ref();
                let default_insert_text_format = lsp_defaults.insert_text_format.as_ref();
                let default_insert_text_mode = lsp_defaults.insert_text_mode.as_ref();

                if default_commit_characters.is_some()
                    || default_edit_range.is_some()
                    || default_insert_text_format.is_some()
                    || default_insert_text_mode.is_some()
                {
                    if completion_with_defaults.commit_characters.is_none()
                        && default_commit_characters.is_some()
                    {
                        completion_with_defaults.commit_characters =
                            default_commit_characters.cloned()
                    }
                    if completion_with_defaults.text_edit.is_none() {
                        match default_edit_range {
                            Some(lsp::CompletionListItemDefaultsEditRange::Range(range)) => {
                                completion_with_defaults.text_edit =
                                    Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                        range: *range,
                                        new_text: completion_with_defaults.label.clone(),
                                    }))
                            }
                            Some(lsp::CompletionListItemDefaultsEditRange::InsertAndReplace {
                                insert,
                                replace,
                            }) => {
                                completion_with_defaults.text_edit =
                                    Some(lsp::CompletionTextEdit::InsertAndReplace(
                                        lsp::InsertReplaceEdit {
                                            new_text: completion_with_defaults.label.clone(),
                                            insert: *insert,
                                            replace: *replace,
                                        },
                                    ))
                            }
                            None => {}
                        }
                    }
                    if completion_with_defaults.insert_text_format.is_none()
                        && default_insert_text_format.is_some()
                    {
                        completion_with_defaults.insert_text_format =
                            default_insert_text_format.cloned()
                    }
                    if completion_with_defaults.insert_text_mode.is_none()
                        && default_insert_text_mode.is_some()
                    {
                        completion_with_defaults.insert_text_mode =
                            default_insert_text_mode.cloned()
                    }
                }
                return Some(Cow::Owned(completion_with_defaults));
            }
            Some(Cow::Borrowed(lsp_completion))
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Completion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Completion")
            .field("replace_range", &self.replace_range)
            .field("new_text", &self.new_text)
            .field("label", &self.label)
            .field("documentation", &self.documentation)
            .field("source", &self.source)
            .finish()
    }
}

/// Response from a source of completions.
pub struct CompletionResponse {
    pub completions: Vec<Completion>,
    pub display_options: CompletionDisplayOptions,
    /// When false, indicates that the list is complete and does not need to be re-queried if it
    /// can be filtered instead.
    pub is_incomplete: bool,
}

#[derive(Default)]
pub struct CompletionDisplayOptions {
    pub dynamic_width: bool,
}

impl CompletionDisplayOptions {
    pub fn merge(&mut self, other: &CompletionDisplayOptions) {
        self.dynamic_width = self.dynamic_width && other.dynamic_width;
    }
}

/// Response from language server completion request.
#[derive(Clone, Debug, Default)]
pub(crate) struct CoreCompletionResponse {
    pub completions: Vec<CoreCompletion>,
    /// When false, indicates that the list is complete and does not need to be re-queried if it
    /// can be filtered instead.
    pub is_incomplete: bool,
}

/// A generic completion that can come from different sources.
#[derive(Clone, Debug)]
pub(crate) struct CoreCompletion {
    replace_range: Range<Anchor>,
    new_text: String,
    source: CompletionSource,
}

/// A code action provided by a language server.
#[derive(Clone, Debug, PartialEq)]
pub struct CodeAction {
    /// The id of the language server that produced this code action.
    pub server_id: LanguageServerId,
    /// The range of the buffer where this code action is applicable.
    pub range: Range<Anchor>,
    /// The raw code action provided by the language server.
    /// Can be either an action or a command.
    pub lsp_action: LspAction,
    /// Whether the action needs to be resolved using the language server.
    pub resolved: bool,
}

/// An action sent back by a language server.
#[derive(Clone, Debug, PartialEq)]
pub enum LspAction {
    /// An action with the full data, may have a command or may not.
    /// May require resolving.
    Action(Box<lsp::CodeAction>),
    /// A command data to run as an action.
    Command(lsp::Command),
    /// A code lens data to run as an action.
    CodeLens(lsp::CodeLens),
}

impl LspAction {
    pub fn title(&self) -> &str {
        match self {
            Self::Action(action) => &action.title,
            Self::Command(command) => &command.title,
            Self::CodeLens(lens) => lens
                .command
                .as_ref()
                .map(|command| command.title.as_str())
                .unwrap_or("Unknown command"),
        }
    }

    pub fn action_kind(&self) -> Option<lsp::CodeActionKind> {
        match self {
            Self::Action(action) => action.kind.clone(),
            Self::Command(_) => Some(lsp::CodeActionKind::new("command")),
            Self::CodeLens(_) => Some(lsp::CodeActionKind::new("code lens")),
        }
    }

    pub fn edit(&self) -> Option<&lsp::WorkspaceEdit> {
        match self {
            Self::Action(action) => action.edit.as_ref(),
            Self::Command(_) => None,
            Self::CodeLens(_) => None,
        }
    }

    pub fn command(&self) -> Option<&lsp::Command> {
        match self {
            Self::Action(action) => action.command.as_ref(),
            Self::Command(command) => Some(command),
            Self::CodeLens(lens) => lens.command.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveState {
    Resolved,
    CanResolve(LanguageServerId, Option<lsp::LSPAny>),
    Resolving,
}
impl InlayHint {
    pub fn text(&self) -> Rope {
        match &self.label {
            InlayHintLabel::String(s) => Rope::from(s),
            InlayHintLabel::LabelParts(parts) => parts.iter().map(|part| &*part.value).collect(),
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

#[derive(Debug, Clone, PartialEq)]
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
    pub source_language_server_id: LanguageServerId,
    pub path: SymbolLocation,
    pub label: CodeLabel,
    pub name: String,
    pub kind: lsp::SymbolKind,
    pub range: Range<Unclipped<PointUtf16>>,
    pub container_name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DocumentSymbol {
    pub name: String,
    pub kind: lsp::SymbolKind,
    pub range: Range<Unclipped<PointUtf16>>,
    pub selection_range: Range<Unclipped<PointUtf16>>,
    pub children: Vec<DocumentSymbol>,
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

enum EntitySubscription {
    Project(PendingEntitySubscription<Project>),
    BufferStore(PendingEntitySubscription<BufferStore>),
    GitStore(PendingEntitySubscription<GitStore>),
    WorktreeStore(PendingEntitySubscription<WorktreeStore>),
    LspStore(PendingEntitySubscription<LspStore>),
    SettingsObserver(PendingEntitySubscription<SettingsObserver>),
    DapStore(PendingEntitySubscription<DapStore>),
    BreakpointStore(PendingEntitySubscription<BreakpointStore>),
}

#[derive(Debug, Clone)]
pub struct DirectoryItem {
    pub path: PathBuf,
    pub is_dir: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocumentColor {
    pub lsp_range: lsp::Range,
    pub color: lsp::Color,
    pub resolved: bool,
    pub color_presentations: Vec<ColorPresentation>,
}

impl Eq for DocumentColor {}

impl std::hash::Hash for DocumentColor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lsp_range.hash(state);
        self.color.red.to_bits().hash(state);
        self.color.green.to_bits().hash(state);
        self.color.blue.to_bits().hash(state);
        self.color.alpha.to_bits().hash(state);
        self.resolved.hash(state);
        self.color_presentations.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorPresentation {
    pub label: SharedString,
    pub text_edit: Option<lsp::TextEdit>,
    pub additional_text_edits: Vec<lsp::TextEdit>,
}

impl std::hash::Hash for ColorPresentation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label.hash(state);
        if let Some(ref edit) = self.text_edit {
            edit.range.hash(state);
            edit.new_text.hash(state);
        }
        self.additional_text_edits.len().hash(state);
        for edit in &self.additional_text_edits {
            edit.range.hash(state);
            edit.new_text.hash(state);
        }
    }
}

#[derive(Clone)]
pub enum DirectoryLister {
    Project(Entity<Project>),
    Local(Entity<Project>, Arc<dyn Fs>),
}

impl std::fmt::Debug for DirectoryLister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectoryLister::Project(project) => {
                write!(f, "DirectoryLister::Project({project:?})")
            }
            DirectoryLister::Local(project, _) => {
                write!(f, "DirectoryLister::Local({project:?})")
            }
        }
    }
}

impl DirectoryLister {
    pub fn is_local(&self, cx: &App) -> bool {
        match self {
            DirectoryLister::Local(..) => true,
            DirectoryLister::Project(project) => project.read(cx).is_local(cx),
        }
    }

    pub fn resolve_tilde<'a>(&self, path: &'a String, cx: &App) -> Cow<'a, str> {
        if self.is_local(cx) {
            shellexpand::tilde(path)
        } else {
            Cow::from(path)
        }
    }

    pub fn default_query(&self, cx: &mut App) -> String {
        let project = match self {
            DirectoryLister::Project(project) => project,
            DirectoryLister::Local(project, _) => project,
        }
        .read(cx);
        let path_style = project.path_style(cx);
        project
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().into_owned())
            .or_else(|| std::env::home_dir().map(|dir| dir.to_string_lossy().into_owned()))
            .map(|mut s| {
                s.push_str(path_style.primary_separator());
                s
            })
            .unwrap_or_else(|| {
                if path_style.is_windows() {
                    "C:\\"
                } else {
                    "~/"
                }
                .to_string()
            })
    }

    pub fn list_directory(&self, path: String, cx: &mut App) -> Task<Result<Vec<DirectoryItem>>> {
        match self {
            DirectoryLister::Project(project) => {
                project.update(cx, |project, cx| project.list_directory(path, cx))
            }
            DirectoryLister::Local(_, fs) => {
                let fs = fs.clone();
                cx.background_spawn(async move {
                    let mut results = vec![];
                    let expanded = shellexpand::tilde(&path);
                    let query = Path::new(expanded.as_ref());
                    let mut response = fs.read_dir(query).await?;
                    while let Some(path) = response.next().await {
                        let path = path?;
                        if let Some(file_name) = path.file_name() {
                            results.push(DirectoryItem {
                                path: PathBuf::from(file_name.to_os_string()),
                                is_dir: fs.is_dir(&path).await,
                            });
                        }
                    }
                    Ok(results)
                })
            }
        }
    }

    pub fn path_style(&self, cx: &App) -> PathStyle {
        match self {
            Self::Local(project, ..) | Self::Project(project, ..) => {
                project.read(cx).path_style(cx)
            }
        }
    }
}

pub const CURRENT_PROJECT_FEATURES: &[&str] = &["new-style-anchors"];

#[cfg(feature = "test-support")]
pub const DEFAULT_COMPLETION_CONTEXT: CompletionContext = CompletionContext {
    trigger_kind: lsp::CompletionTriggerKind::INVOKED,
    trigger_character: None,
};

/// An LSP diagnostics associated with a certain language server.
#[derive(Clone, Debug, Default)]
pub enum LspPullDiagnostics {
    #[default]
    Default,
    Response {
        /// The id of the language server that produced diagnostics.
        server_id: LanguageServerId,
        /// URI of the resource,
        uri: lsp::Uri,
        /// The ID provided by the dynamic registration that produced diagnostics.
        registration_id: Option<SharedString>,
        /// The diagnostics produced by this language server.
        diagnostics: PulledDiagnostics,
    },
}

#[derive(Clone, Debug)]
pub enum PulledDiagnostics {
    Unchanged {
        /// An ID the current pulled batch for this file.
        /// If given, can be used to query workspace diagnostics partially.
        result_id: SharedString,
    },
    Changed {
        result_id: Option<SharedString>,
        diagnostics: Vec<lsp::Diagnostic>,
    },
}

/// Whether to disable all AI features in Zed.
///
/// Default: false
#[derive(Copy, Clone, Debug, RegisterSetting)]
pub struct DisableAiSettings {
    pub disable_ai: bool,
}

impl settings::Settings for DisableAiSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            disable_ai: content.project.disable_ai.unwrap().0,
        }
    }
}

impl DisableAiSettings {
    /// Returns whether AI is disabled for the given file.
    ///
    /// This checks the project-level settings for the file's worktree,
    /// allowing `disable_ai` to be configured per-project in `.zed/settings.json`.
    pub fn is_ai_disabled_for_buffer(buffer: Option<&Entity<Buffer>>, cx: &App) -> bool {
        Self::is_ai_disabled_for_file(buffer.and_then(|buffer| buffer.read(cx).file()), cx)
    }

    pub fn is_ai_disabled_for_file(file: Option<&Arc<dyn language::File>>, cx: &App) -> bool {
        let location = file.map(|f| settings::SettingsLocation {
            worktree_id: f.worktree_id(cx),
            path: f.path().as_ref(),
        });
        Self::get(location, cx).disable_ai
    }
}

impl Project {
    pub fn init(client: &Arc<Client>, cx: &mut App) {
        connection_manager::init(client.clone(), cx);
        host::init(cx);

        let client: AnyProtoClient = client.clone().into();
        client.add_entity_message_handler(Self::handle_add_collaborator);
        client.add_entity_message_handler(Self::handle_update_project_collaborator);
        client.add_entity_message_handler(Self::handle_remove_collaborator);
        client.add_entity_message_handler(Self::handle_update_project);
        client.add_entity_message_handler(Self::handle_unshare_project);
        client.add_entity_request_handler(Self::handle_update_buffer);
        client.add_entity_message_handler(Self::handle_update_worktree);
        client.add_entity_request_handler(Self::handle_synchronize_buffers);
        client.add_entity_message_handler(Self::handle_close_buffer);
        client.add_entity_request_handler(Self::handle_reload_buffers);
        client.add_entity_request_handler(Self::handle_lsp_query);
        client.add_entity_request_handler(Self::handle_fetch);
        client.add_entity_request_handler(Self::handle_push);
        client.add_entity_request_handler(Self::handle_pull);
        client.add_entity_request_handler(Self::handle_commit);
        client.add_entity_request_handler(Self::handle_apply_code_action);
        client.add_entity_request_handler(Self::handle_apply_code_action_kind);
        client.add_entity_request_handler(Self::handle_format_buffers);
        client.add_entity_request_handler(Self::handle_open_buffer_for_symbol);
        client.add_entity_request_handler(Self::handle_register_buffer_with_language_servers);
        client.add_entity_request_handler(Self::handle_open_commit_message_buffer);
        client.add_entity_request_handler(Self::handle_rename_project_entry);
        client.add_entity_request_handler(Self::handle_lsp_command_with_project::<PerformRename>);
        client.add_entity_request_handler(
            Self::handle_lsp_command_with_project::<lsp_store::lsp_ext_command::GoToParentModule>,
        );
        client.add_entity_request_handler(
            Self::handle_lsp_command_with_project::<lsp_store::lsp_ext_command::GetLspRunnables>,
        );

        client.add_entity_request_handler(Self::handle_search_candidate_buffers);
        client.add_entity_request_handler(Self::handle_open_buffer_by_id);
        client.add_entity_request_handler(Self::handle_open_buffer_by_path);
        client.add_entity_request_handler(Self::handle_open_new_buffer);
        client.add_entity_message_handler(Self::handle_create_buffer_for_peer);
        client.add_entity_message_handler(Self::handle_toggle_lsp_logs);
        client.add_entity_message_handler(Self::handle_create_image_for_peer);
        client.add_entity_request_handler(Self::handle_find_search_candidates_chunk);
        client.add_entity_message_handler(Self::handle_find_search_candidates_cancel);
        client.add_entity_message_handler(Self::handle_create_file_for_peer);

        WorktreeStore::init(&client);
        BufferStore::init(&client);
        LspStore::init(&client);
        GitStore::init(&client);
        SettingsObserver::init(&client);
        TaskStore::init(Some(&client));
        ToolchainStore::init(&client);
        DapStore::init(&client, cx);
        BreakpointStore::init(&client);
        context_server_store::init(cx);
    }

    pub fn local(
        client: Arc<Client>,
        node: NodeRuntime,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        env: Option<HashMap<String, String>>,
        flags: LocalProjectFlags,
        cx: &mut App,
    ) -> Entity<Self> {
        let host = host::Host::local(
            client.clone(),
            node,
            user_store,
            languages.clone(),
            fs,
            env,
            flags.watch_global_configs,
            cx,
        );
        let project = cx.new(|cx: &mut Context<Self>| {
            let (tx, rx) = mpsc::unbounded();
            cx.spawn(async move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx).await)
                .detach();

            // Pull entity refs out of `Host` so we can subscribe and run
            // Project-level wiring without holding a borrow of `cx`.
            let (
                worktree_store,
                buffer_store,
                breakpoint_store,
                dap_store,
                image_store,
                git_store,
                settings_observer,
                lsp_store,
                context_server_store,
            ) = {
                let host = host.read(cx);
                (
                    host.worktree_store.clone(),
                    host.buffer_store.clone(),
                    host.breakpoint_store.clone(),
                    host.dap_store.clone(),
                    host.image_store.clone(),
                    host.git_store.clone(),
                    host.settings_observer.clone(),
                    host.lsp_store.clone(),
                    host.context_server_store.clone(),
                )
            };

            if flags.init_worktree_trust {
                trusted_worktrees::track_worktree_trust(
                    worktree_store.clone(),
                    None,
                    None,
                    None,
                    cx,
                );
            }

            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();
            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();
            cx.subscribe(&breakpoint_store, Self::on_breakpoint_store_event)
                .detach();
            cx.subscribe(&dap_store, Self::on_dap_store_event).detach();
            cx.subscribe(&image_store, Self::on_image_store_event)
                .detach();
            cx.subscribe(&git_store, Self::on_git_store_event).detach();
            cx.subscribe(&settings_observer, Self::on_settings_observer_event)
                .detach();
            cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();
            Self::wire_context_server_triggers(&context_server_store, cx);

            Self {
                host,
                buffer_ordered_messages_tx: tx,
                collaborators: Default::default(),
                join_project_response_message_id: 0,
                client_state: ProjectClientState::Local,
                client_subscriptions: Vec::new(),
                _subscriptions: vec![cx.on_release(Self::release)],
                active_entry: None,
                languages,
                collab_client: client,

                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                search_history: Self::new_search_history(),
                remotely_created_models: Default::default(),

                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),

                agent_location: None,
                downloading_files: Default::default(),
                last_worktree_paths: WorktreePaths::default(),
                worktrees: Vec::new(),
                retain_worktrees: false,
                git_repository_snapshots_for_peer: HashMap::default(),
                shared_buffers: HashMap::default(),
                buffers: HashSet::default(),
                repositories: HashSet::default(),
                active_repository_id: None,
                language_servers: HashSet::default(),
                pending_worktree_paths: HashSet::default(),
                context_server_update_task: None,
                context_server_needs_update: false,
                last_scheduled_tasks: VecDeque::default(),
                last_scheduled_scenarios: VecDeque::default(),
                dap_sessions: HashSet::default(),
            }
        });
        Self::subscribe_to_inventory_events(&project, cx);
        Self::trigger_initial_context_server_refresh(&project, cx);
        project
    }

    pub fn remote(
        remote: Entity<RemoteClient>,
        client: Arc<Client>,
        node: NodeRuntime,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        init_worktree_trust: bool,
        cx: &mut App,
    ) -> Entity<Self> {
        let (remote_proto, connection_options) = remote.read_with(cx, |remote, _| {
            (remote.proto_client(), remote.connection_options())
        });
        let host = host::Host::remote(
            remote.clone(),
            client.clone(),
            node,
            user_store,
            languages.clone(),
            fs,
            cx,
        );
        let project = cx.new(|cx: &mut Context<Self>| {
            let (tx, rx) = mpsc::unbounded();
            cx.spawn(async move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx).await)
                .detach();

            // Project::remote subscribes to a different subset of host
            // stores than Project::local (no dap/image_store events,
            // for example).
            let (
                worktree_store,
                buffer_store,
                breakpoint_store,
                git_store,
                settings_observer,
                lsp_store,
                context_server_store,
            ) = {
                let host = host.read(cx);
                (
                    host.worktree_store.clone(),
                    host.buffer_store.clone(),
                    host.breakpoint_store.clone(),
                    host.git_store.clone(),
                    host.settings_observer.clone(),
                    host.lsp_store.clone(),
                    host.context_server_store.clone(),
                )
            };

            if init_worktree_trust {
                trusted_worktrees::track_worktree_trust(
                    worktree_store.clone(),
                    Some(RemoteHostLocation::from(connection_options)),
                    None,
                    Some((remote_proto.clone(), ProjectId(REMOTE_SERVER_PROJECT_ID))),
                    cx,
                );
            }

            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();
            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();
            cx.subscribe(&breakpoint_store, Self::on_breakpoint_store_event)
                .detach();
            cx.subscribe(&git_store, Self::on_git_store_event).detach();
            cx.subscribe(&settings_observer, Self::on_settings_observer_event)
                .detach();
            cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();
            cx.subscribe(&remote, Self::on_remote_client_event).detach();
            Self::wire_context_server_triggers(&context_server_store, cx);

            let this = Self {
                host,
                buffer_ordered_messages_tx: tx,
                collaborators: Default::default(),
                join_project_response_message_id: 0,
                client_state: ProjectClientState::Local,
                client_subscriptions: Vec::new(),
                // SSH-process shutdown is registered on `Host` itself
                // (both `on_release` and `on_app_quit`) — see
                // `host::Host::remote`. Project only needs to clean up
                // its own collab/shared state on release.
                _subscriptions: vec![cx.on_release(Self::release)],
                active_entry: None,
                languages,
                collab_client: client,
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                search_history: Self::new_search_history(),
                remotely_created_models: Default::default(),

                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),

                agent_location: None,
                downloading_files: Default::default(),
                last_worktree_paths: WorktreePaths::default(),
                worktrees: Vec::new(),
                retain_worktrees: false,
                git_repository_snapshots_for_peer: HashMap::default(),
                shared_buffers: HashMap::default(),
                buffers: HashSet::default(),
                repositories: HashSet::default(),
                active_repository_id: None,
                language_servers: HashSet::default(),
                pending_worktree_paths: HashSet::default(),
                context_server_update_task: None,
                context_server_needs_update: false,
                last_scheduled_tasks: VecDeque::default(),
                last_scheduled_scenarios: VecDeque::default(),
                dap_sessions: HashSet::default(),
            };

            // remote server -> local machine handlers
            remote_proto.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &cx.entity());
            remote_proto.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &this.buffer_store(cx));
            remote_proto.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &this.worktree_store(cx));
            remote_proto.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &this.lsp_store(cx));
            remote_proto.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &this.dap_store(cx));
            remote_proto.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &this.breakpoint_store(cx));
            remote_proto.subscribe_to_entity(
                REMOTE_SERVER_PROJECT_ID,
                &this.host.read(cx).settings_observer,
            );
            remote_proto.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &this.git_store(cx));
            remote_proto.subscribe_to_entity(
                REMOTE_SERVER_PROJECT_ID,
                &this.host.read(cx).agent_server_store,
            );

            remote_proto.add_entity_message_handler(Self::handle_create_buffer_for_peer);
            remote_proto.add_entity_message_handler(Self::handle_create_image_for_peer);
            remote_proto.add_entity_message_handler(Self::handle_create_file_for_peer);
            remote_proto.add_entity_message_handler(Self::handle_update_worktree);
            remote_proto.add_entity_message_handler(Self::handle_update_project);
            remote_proto.add_entity_message_handler(Self::handle_toast);
            remote_proto.add_entity_request_handler(Self::handle_language_server_prompt_request);
            remote_proto.add_entity_message_handler(Self::handle_hide_toast);
            remote_proto.add_entity_request_handler(Self::handle_update_buffer_from_remote_server);
            remote_proto.add_entity_request_handler(Self::handle_trust_worktrees);
            remote_proto.add_entity_request_handler(Self::handle_restrict_worktrees);
            remote_proto.add_entity_request_handler(Self::handle_find_search_candidates_chunk);

            remote_proto.add_entity_message_handler(Self::handle_find_search_candidates_cancel);
            remote_proto.add_entity_request_handler(Self::handle_lsp_query);
            remote_proto.add_entity_request_handler(Self::handle_fetch);
            remote_proto.add_entity_request_handler(Self::handle_push);
            remote_proto.add_entity_request_handler(Self::handle_pull);
            remote_proto.add_entity_request_handler(Self::handle_commit);
            remote_proto.add_entity_request_handler(Self::handle_apply_code_action);
            remote_proto.add_entity_request_handler(Self::handle_apply_code_action_kind);
            remote_proto.add_entity_request_handler(Self::handle_format_buffers);
            remote_proto.add_entity_request_handler(Self::handle_open_buffer_for_symbol);
            remote_proto
                .add_entity_request_handler(Self::handle_register_buffer_with_language_servers);
            remote_proto.add_entity_request_handler(Self::handle_open_commit_message_buffer);
            remote_proto.add_entity_request_handler(Self::handle_rename_project_entry);
            remote_proto
                .add_entity_request_handler(Self::handle_lsp_command_with_project::<PerformRename>);
            remote_proto.add_entity_request_handler(
                Self::handle_lsp_command_with_project::<
                    lsp_store::lsp_ext_command::GoToParentModule,
                >,
            );
            remote_proto.add_entity_request_handler(
                Self::handle_lsp_command_with_project::<
                    lsp_store::lsp_ext_command::GetLspRunnables,
                >,
            );
            BufferStore::init(&remote_proto);
            WorktreeStore::init_remote(&remote_proto);
            LspStore::init(&remote_proto);
            SettingsObserver::init(&remote_proto);
            TaskStore::init(Some(&remote_proto));
            ToolchainStore::init(&remote_proto);
            DapStore::init(&remote_proto, cx);
            BreakpointStore::init(&remote_proto);
            GitStore::init(&remote_proto);
            AgentServerStore::init_remote(&remote_proto);

            this
        });
        Self::subscribe_to_inventory_events(&project, cx);
        Self::trigger_initial_context_server_refresh(&project, cx);
        project
    }

    pub async fn in_room(
        remote_id: u64,
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncApp,
    ) -> Result<Entity<Self>> {
        client.connect(true, &cx).await.into_response()?;

        let subscriptions = [
            EntitySubscription::Project(client.subscribe_to_entity::<Self>(remote_id)?),
            EntitySubscription::BufferStore(client.subscribe_to_entity::<BufferStore>(remote_id)?),
            EntitySubscription::GitStore(client.subscribe_to_entity::<GitStore>(remote_id)?),
            EntitySubscription::WorktreeStore(
                client.subscribe_to_entity::<WorktreeStore>(remote_id)?,
            ),
            EntitySubscription::LspStore(client.subscribe_to_entity::<LspStore>(remote_id)?),
            EntitySubscription::SettingsObserver(
                client.subscribe_to_entity::<SettingsObserver>(remote_id)?,
            ),
            EntitySubscription::DapStore(client.subscribe_to_entity::<DapStore>(remote_id)?),
            EntitySubscription::BreakpointStore(
                client.subscribe_to_entity::<BreakpointStore>(remote_id)?,
            ),
        ];
        let committer = get_git_committer(&cx).await;
        let response = client
            .request_envelope(proto::JoinProject {
                project_id: remote_id,
                committer_email: committer.email,
                committer_name: committer.name,
                features: CURRENT_PROJECT_FEATURES
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            })
            .await?;
        Self::from_join_project_response(
            response,
            subscriptions,
            client,
            false,
            user_store,
            languages,
            fs,
            cx,
        )
        .await
    }

    async fn from_join_project_response(
        response: TypedEnvelope<proto::JoinProjectResponse>,
        subscriptions: [EntitySubscription; 8],
        client: Arc<Client>,
        run_tasks: bool,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        mut cx: AsyncApp,
    ) -> Result<Entity<Self>> {
        let remote_id = response.payload.project_id;
        let role = response.payload.role();
        let replica_id = ReplicaId::new(response.payload.replica_id as u16);

        let path_style = if response.payload.windows_paths {
            PathStyle::Windows
        } else {
            PathStyle::Posix
        };

        let user_store_for_host = user_store.clone();
        let host = host::Host::collab(
            remote_id,
            path_style,
            client.clone(),
            run_tasks,
            user_store_for_host,
            languages.clone(),
            fs,
            &mut cx,
        );

        // Pull entity refs out of `Host` once so we can use them both
        // inside Project's `cx.new` closure and after for the EntitySubscription
        // bindings / `lsp_store.update(...)` call below.
        let (
            worktree_store,
            buffer_store,
            breakpoint_store,
            dap_store,
            git_store,
            settings_observer,
            lsp_store,
            context_server_store,
        ) = host.read_with(&cx, |host, _| {
            (
                host.worktree_store.clone(),
                host.buffer_store.clone(),
                host.breakpoint_store.clone(),
                host.dap_store.clone(),
                host.git_store.clone(),
                host.settings_observer.clone(),
                host.lsp_store.clone(),
                host.context_server_store.clone(),
            )
        });

        let project = cx.new(|cx: &mut Context<Self>| {
            let mut worktrees = Vec::new();
            for worktree in &response.payload.worktrees {
                let worktree = Worktree::remote(
                    remote_id,
                    replica_id,
                    worktree.clone(),
                    client.clone().into(),
                    path_style,
                    cx,
                );
                worktrees.push(worktree);
            }

            let (tx, rx) = mpsc::unbounded();
            cx.spawn(async move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx).await)
                .detach();

            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();
            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();
            cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();
            cx.subscribe(&settings_observer, Self::on_settings_observer_event)
                .detach();
            cx.subscribe(&dap_store, Self::on_dap_store_event).detach();
            cx.subscribe(&breakpoint_store, Self::on_breakpoint_store_event)
                .detach();
            cx.subscribe(&git_store, Self::on_git_store_event).detach();
            Self::wire_context_server_triggers(&context_server_store, cx);

            let mut project = Self {
                host,
                buffer_ordered_messages_tx: tx,
                active_entry: None,
                collaborators: Default::default(),
                join_project_response_message_id: response.message_id,
                languages,
                client_subscriptions: Default::default(),
                _subscriptions: vec![cx.on_release(Self::release)],
                collab_client: client.clone(),
                client_state: ProjectClientState::Collab {
                    sharing_has_stopped: false,
                    capability: Capability::ReadWrite,
                    remote_id,
                    replica_id,
                },
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                search_history: Self::new_search_history(),
                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),
                remotely_created_models: Arc::new(Mutex::new(RemotelyCreatedModels::default())),
                agent_location: None,
                downloading_files: Default::default(),
                last_worktree_paths: WorktreePaths::default(),
                worktrees: Vec::new(),
                // Joined collab projects retain all worktrees because the
                // host's view (which we mirror) keeps them all alive.
                retain_worktrees: true,
                git_repository_snapshots_for_peer: HashMap::default(),
                shared_buffers: HashMap::default(),
                buffers: HashSet::default(),
                repositories: HashSet::default(),
                active_repository_id: None,
                language_servers: HashSet::default(),
                pending_worktree_paths: HashSet::default(),
                context_server_update_task: None,
                context_server_needs_update: false,
                last_scheduled_tasks: VecDeque::default(),
                last_scheduled_scenarios: VecDeque::default(),
                dap_sessions: HashSet::default(),
            };
            project.set_role(role, cx);
            for worktree in worktrees {
                project.add_worktree(&worktree, cx);
            }
            project
        });
        cx.update(|cx| Self::subscribe_to_inventory_events(&project, cx));

        let weak_project = project.downgrade();
        lsp_store.update(&mut cx, |lsp_store, cx| {
            lsp_store.set_language_server_statuses_from_proto(
                weak_project,
                response.payload.language_servers,
                response.payload.language_server_capabilities,
                cx,
            );
        });

        let subscriptions = subscriptions
            .into_iter()
            .map(|s| match s {
                EntitySubscription::BufferStore(subscription) => {
                    subscription.set_entity(&buffer_store, &cx)
                }
                EntitySubscription::WorktreeStore(subscription) => {
                    subscription.set_entity(&worktree_store, &cx)
                }
                EntitySubscription::GitStore(subscription) => {
                    subscription.set_entity(&git_store, &cx)
                }
                EntitySubscription::SettingsObserver(subscription) => {
                    subscription.set_entity(&settings_observer, &cx)
                }
                EntitySubscription::Project(subscription) => subscription.set_entity(&project, &cx),
                EntitySubscription::LspStore(subscription) => {
                    subscription.set_entity(&lsp_store, &cx)
                }
                EntitySubscription::DapStore(subscription) => {
                    subscription.set_entity(&dap_store, &cx)
                }
                EntitySubscription::BreakpointStore(subscription) => {
                    subscription.set_entity(&breakpoint_store, &cx)
                }
            })
            .collect::<Vec<_>>();

        let user_ids = response
            .payload
            .collaborators
            .iter()
            .map(|peer| peer.user_id)
            .collect();
        user_store
            .update(&mut cx, |user_store, cx| user_store.get_users(user_ids, cx))
            .await?;

        project.update(&mut cx, |this, cx| {
            this.set_collaborators_from_proto(response.payload.collaborators, cx)?;
            this.client_subscriptions.extend(subscriptions);
            anyhow::Ok(())
        })?;
        cx.update(|cx| Self::trigger_initial_context_server_refresh(&project, cx));

        Ok(project)
    }

    fn new_search_history() -> SearchHistory {
        SearchHistory::new(
            Some(MAX_PROJECT_SEARCH_HISTORY_SIZE),
            search_history::QueryInsertionBehavior::AlwaysInsert,
        )
    }

    fn release(&mut self, cx: &mut App) {
        // Remote-client shutdown lives on `Host` (see
        // `host::Host::remote`); when the last `Project` drops its
        // `Entity<Host>`, Host's own `on_release` runs and triggers
        // `shutdown_processes`. Project only handles the collab /
        // shared lifecycle here.
        match &self.client_state {
            ProjectClientState::Local => {}
            ProjectClientState::Shared { .. } => {
                let _ = self.unshare_internal(cx);
            }
            ProjectClientState::Collab { remote_id, .. } => {
                let _ = self.collab_client.send(proto::LeaveProject {
                    project_id: *remote_id,
                });
                self.disconnected_from_host_internal(cx);
            }
        }
    }

    #[cfg(feature = "test-support")]
    pub fn client_subscriptions(&self) -> &Vec<client::Subscription> {
        &self.client_subscriptions
    }

    #[cfg(feature = "test-support")]
    pub async fn example(
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut AsyncApp,
    ) -> Entity<Project> {
        use clock::FakeSystemClock;

        let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
        let languages = LanguageRegistry::test(cx.background_executor().clone());
        let clock = Arc::new(FakeSystemClock::new());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let client = cx.update(|cx| client::Client::new(clock, http_client.clone(), cx));
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let project = cx.update(|cx| {
            Project::local(
                client,
                node_runtime::NodeRuntime::unavailable(),
                user_store,
                Arc::new(languages),
                fs,
                None,
                LocalProjectFlags {
                    init_worktree_trust: false,
                    ..Default::default()
                },
                cx,
            )
        });
        for path in root_paths {
            let (tree, _): (Entity<Worktree>, _) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(path, true, cx)
                })
                .await
                .unwrap();
            tree.read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .await;
        }
        project
    }

    #[cfg(feature = "test-support")]
    pub async fn test(
        fs: Arc<dyn Fs>,
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut gpui::TestAppContext,
    ) -> Entity<Project> {
        Self::test_project(fs, root_paths, false, cx).await
    }

    #[cfg(feature = "test-support")]
    pub async fn test_with_worktree_trust(
        fs: Arc<dyn Fs>,
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut gpui::TestAppContext,
    ) -> Entity<Project> {
        Self::test_project(fs, root_paths, true, cx).await
    }

    #[cfg(feature = "test-support")]
    async fn test_project(
        fs: Arc<dyn Fs>,
        root_paths: impl IntoIterator<Item = &Path>,
        init_worktree_trust: bool,
        cx: &mut gpui::TestAppContext,
    ) -> Entity<Project> {
        use clock::FakeSystemClock;

        let languages = LanguageRegistry::test(cx.executor());
        let clock = Arc::new(FakeSystemClock::new());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let client = cx.update(|cx| client::Client::new(clock, http_client.clone(), cx));
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let project = cx.update(|cx| {
            Project::local(
                client,
                node_runtime::NodeRuntime::unavailable(),
                user_store,
                Arc::new(languages),
                fs,
                None,
                LocalProjectFlags {
                    init_worktree_trust,
                    ..Default::default()
                },
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

            tree.read_with(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .await;
        }
        project
    }

    /// Transitions a local test project into the `Collab` client state so that
    /// `is_via_collab()` returns `true`. Use only in tests.
    #[cfg(any(test, feature = "test-support"))]
    pub fn mark_as_collab_for_testing(&mut self) {
        self.client_state = ProjectClientState::Collab {
            sharing_has_stopped: false,
            capability: Capability::ReadWrite,
            remote_id: 0,
            replica_id: clock::ReplicaId::new(1),
        };
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn add_test_remote_worktree(
        &mut self,
        abs_path: &str,
        cx: &mut Context<Self>,
    ) -> Entity<Worktree> {
        use rpc::NoopProtoClient;
        use util::paths::PathStyle;

        let root_name = std::path::Path::new(abs_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let client = AnyProtoClient::new(NoopProtoClient::new());
        let worktree = Worktree::remote(
            0,
            ReplicaId::new(1),
            proto::WorktreeMetadata {
                id: 100 + self.visible_worktrees(cx).count() as u64,
                root_name,
                visible: true,
                abs_path: abs_path.to_string(),
                root_repo_common_dir: None,
            },
            client,
            PathStyle::Posix,
            cx,
        );
        self.worktree_store(cx)
            .update(cx, |store, cx| store.add(&worktree, cx));
        // Phase 2 sharing: event handler no longer auto-claims; claim
        // explicitly so `self.worktrees` reflects the new worktree.
        self.claim_found_worktree(&worktree, cx);
        worktree
    }

    #[inline]
    pub fn dap_store(&self, cx: &App) -> Entity<DapStore> {
        self.host.read(cx).dap_store.clone()
    }

    #[inline]
    pub fn bookmark_store(&self, cx: &App) -> Entity<BookmarkStore> {
        self.host.read(cx).bookmark_store.clone()
    }

    #[inline]
    pub fn breakpoint_store(&self, cx: &App) -> Entity<BreakpointStore> {
        self.host.read(cx).breakpoint_store.clone()
    }

    pub fn active_debug_session(&self, cx: &App) -> Option<(Entity<Session>, ActiveStackFrame)> {
        let active_position = self.breakpoint_store(cx).read(cx).active_position()?;
        // Phase 2 multi-tenant: the host `BreakpointStore` holds a
        // single host-wide active stack frame; filter to ours so an
        // "active debug line" set by a sibling Project's session
        // doesn't show up here.
        if !self.owns_dap_session(active_position.session_id) {
            return None;
        }
        let session = self
            .dap_store(cx)
            .read(cx)
            .session_by_id(active_position.session_id)?;
        Some((session, active_position.clone()))
    }

    /// Returns `true` when `session_id` was launched by this Project.
    /// Used to scope the host-shared `DapStore` (sessions, log
    /// messages, lifecycle events, notifications) to a single
    /// Project's view in Phase 2.
    #[inline]
    pub fn owns_dap_session(&self, session_id: SessionId) -> bool {
        self.dap_sessions.contains(&session_id)
    }

    /// Records `session_id` as belonging to this Project. Callers
    /// should invoke this immediately after
    /// `DapStore::new_session(...)` so subsequent
    /// `DebugClientStarted` / `LogToDebugConsole` / `Notification`
    /// events get routed to this Project (and not its siblings on the
    /// shared host store).
    pub fn claim_dap_session(&mut self, session_id: SessionId) {
        self.dap_sessions.insert(session_id);
    }

    /// Filtered view of `DapStore::session_by_id` that only resolves
    /// sessions launched by this Project.
    pub fn dap_session_by_id(&self, session_id: SessionId, cx: &App) -> Option<Entity<Session>> {
        if !self.owns_dap_session(session_id) {
            return None;
        }
        self.dap_store(cx).read(cx).session_by_id(session_id)
    }

    /// Filtered view of `DapStore::sessions()` that only includes
    /// sessions launched by this Project. UI surfaces that iterate
    /// debug sessions (debug panel, inline values, breakpoint
    /// indicators) should go through this rather than the host store
    /// so sibling Projects' sessions don't leak in.
    pub fn dap_sessions(&self, cx: &App) -> Vec<Entity<Session>> {
        let dap_store = self.dap_store(cx);
        let dap_store = dap_store.read(cx);
        self.dap_sessions
            .iter()
            .filter_map(|id| dap_store.session_by_id(*id))
            .collect()
    }

    #[inline]
    pub fn lsp_store(&self, cx: &App) -> Entity<LspStore> {
        self.host.read(cx).lsp_store.clone()
    }

    #[inline]
    pub fn image_store(&self, cx: &App) -> Entity<ImageStore> {
        self.host.read(cx).image_store.clone()
    }

    #[inline]
    pub fn worktree_store(&self, cx: &App) -> Entity<WorktreeStore> {
        self.host.read(cx).worktree_store.clone()
    }

    /// Returns a future that resolves when all visible worktrees have completed
    /// their initial scan.
    pub fn wait_for_initial_scan(&self, cx: &App) -> impl Future<Output = ()> + use<> {
        self.worktree_store(cx).read(cx).wait_for_initial_scan()
    }

    #[inline]
    pub fn context_server_store(&self, cx: &App) -> Entity<ContextServerStore> {
        self.host.read(cx).context_server_store.clone()
    }

    /// Subscribe this Project to the triggers that should re-run the
    /// context server maintain loop: `ContextServersChanged` events from
    /// the (possibly shared) `ContextServerStore` (settings updates, AI
    /// re-enabled, OAuth logout) and changes to the global
    /// `ContextServerDescriptorRegistry` (extension installs).
    fn wire_context_server_triggers(
        context_server_store: &Entity<ContextServerStore>,
        cx: &mut Context<Self>,
    ) {
        cx.subscribe(
            context_server_store,
            |this, _, _: &ContextServersChanged, cx| {
                if !DisableAiSettings::get_global(cx).disable_ai {
                    this.available_context_servers_changed(cx);
                }
            },
        )
        .detach();
        let registry = ContextServerDescriptorRegistry::default_global(cx);
        cx.observe(&registry, |this, _registry, cx| {
            if !DisableAiSettings::get_global(cx).disable_ai {
                this.available_context_servers_changed(cx);
            }
        })
        .detach();
    }

    /// Subscribe to the host-shared `Inventory`'s settings-reload
    /// events so this Project can prune its per-Project task / scenario
    /// LRU when the underlying template definitions change.
    fn subscribe_to_inventory_events(project: &Entity<Self>, cx: &mut App) {
        let Some(inventory) = project
            .read(cx)
            .task_store(cx)
            .read(cx)
            .task_inventory()
            .cloned()
        else {
            return;
        };
        project.update(cx, |_, cx| {
            cx.subscribe(&inventory, Self::on_inventory_event).detach();
        });
    }

    fn on_inventory_event(
        &mut self,
        _: Entity<Inventory>,
        event: &InventoryEvent,
        _cx: &mut Context<Self>,
    ) {
        match event {
            InventoryEvent::TaskTemplatesReloaded { reload } => match reload {
                TaskTemplateReload::Global { abs_path } => {
                    self.last_scheduled_tasks.retain(|(kind, _)| {
                        if let TaskSourceKind::AbsPath {
                            abs_path: kind_path,
                            ..
                        } = kind
                        {
                            kind_path != abs_path
                        } else {
                            true
                        }
                    });
                }
                TaskTemplateReload::Worktree {
                    worktree_id,
                    directory,
                } => {
                    self.last_scheduled_tasks.retain(|(kind, _)| {
                        if let TaskSourceKind::Worktree {
                            id,
                            directory_in_worktree,
                            ..
                        } = kind
                        {
                            id != worktree_id || directory_in_worktree != directory
                        } else {
                            true
                        }
                    });
                }
            },
            InventoryEvent::DebugScenariosReloaded {
                new_definitions,
                previously_existing,
            } => {
                self.last_scheduled_scenarios.retain_mut(|(scenario, _)| {
                    if !previously_existing.contains(&scenario.label) {
                        return true;
                    }
                    if let Some(new_definition) = new_definitions.get(&scenario.label) {
                        *scenario = new_definition.clone();
                        true
                    } else {
                        false
                    }
                });
            }
        }
    }

    /// Records a task as just-scheduled in this Project's recent-tasks
    /// LRU. Used by `Workspace::schedule_resolved_task`. The LRU is
    /// capped at 5000 entries.
    pub fn task_scheduled(
        &mut self,
        task_source_kind: TaskSourceKind,
        resolved_task: ResolvedTask,
    ) {
        self.last_scheduled_tasks
            .push_back((task_source_kind, resolved_task));
        if self.last_scheduled_tasks.len() > 5_000 {
            self.last_scheduled_tasks.pop_front();
        }
    }

    /// Records a debug scenario as just-scheduled in this Project's
    /// recent-scenarios LRU. Deduplicates by label — a later schedule
    /// of the same label drops the earlier entry. Capped at 5000.
    pub fn scenario_scheduled(
        &mut self,
        scenario: DebugScenario,
        task_context: SharedTaskContext,
        worktree_id: Option<WorktreeId>,
        active_buffer: Option<WeakEntity<Buffer>>,
    ) {
        self.last_scheduled_scenarios
            .retain(|(s, _)| s.label != scenario.label);
        self.last_scheduled_scenarios.push_front((
            scenario,
            DebugScenarioContext {
                task_context,
                worktree_id,
                active_buffer,
            },
        ));
        if self.last_scheduled_scenarios.len() > 5_000 {
            self.last_scheduled_scenarios.pop_front();
        }
    }

    /// Returns the last scheduled task by `task_id` if provided.
    /// Otherwise returns the most recently scheduled task overall.
    pub fn last_scheduled_task(
        &self,
        task_id: Option<&TaskId>,
    ) -> Option<(TaskSourceKind, ResolvedTask)> {
        if let Some(task_id) = task_id {
            self.last_scheduled_tasks
                .iter()
                .find(|(_, task)| &task.id == task_id)
                .cloned()
        } else {
            self.last_scheduled_tasks.back().cloned()
        }
    }

    /// Returns the most recently scheduled debug scenario for this
    /// Project.
    pub fn last_scheduled_scenario(&self) -> Option<(DebugScenario, DebugScenarioContext)> {
        self.last_scheduled_scenarios.back().cloned()
    }

    /// Removes a task from this Project's recent-tasks LRU by its
    /// resolved id. A similar task may resurface in
    /// `Inventory::used_and_current_resolved_tasks` when its
    /// [`TaskTemplate`](task::TaskTemplate) is resolved again.
    pub fn delete_previously_used_task(&mut self, id: &TaskId) {
        self.last_scheduled_tasks.retain(|(_, task)| &task.id != id);
    }

    /// Snapshot of this Project's recent-tasks LRU. Pass to
    /// `Inventory::used_and_current_resolved_tasks` so the LRU sort
    /// reflects only this Project's history.
    pub fn last_scheduled_tasks(&self) -> VecDeque<(TaskSourceKind, ResolvedTask)> {
        self.last_scheduled_tasks.clone()
    }

    /// Snapshot of this Project's recent-scenarios LRU. Pass to
    /// `Inventory::list_debug_scenarios`.
    pub fn last_scheduled_scenarios(&self) -> VecDeque<(DebugScenario, DebugScenarioContext)> {
        self.last_scheduled_scenarios.clone()
    }

    /// Kick off the first maintain pass. The store no longer auto-starts
    /// the loop on construction (it can't, since it doesn't know which
    /// Project's `active_project_directory` to use), so each Project
    /// initiates its own initial refresh after construction.
    fn trigger_initial_context_server_refresh(project: &Entity<Self>, cx: &mut App) {
        if DisableAiSettings::get_global(cx).disable_ai {
            return;
        }
        project.update(cx, |this, cx| {
            this.available_context_servers_changed(cx);
        });
    }

    /// Spawn (or queue) a context server maintain pass for this Project,
    /// computing the preferred `root_path` from this Project's view
    /// (`active_project_directory`) and threading it through to
    /// `ContextServerStore::maintain_servers` as the root-path override.
    ///
    /// Equivalent to the previous in-store `available_context_servers_changed`
    /// debouncer, but driven from `Project` so that each Project can
    /// supply its own per-project `root_path` to the (potentially shared)
    /// `ContextServerStore`.
    pub fn available_context_servers_changed(&mut self, cx: &mut Context<Self>) {
        if self.context_server_update_task.is_some() {
            self.context_server_needs_update = true;
            return;
        }
        self.context_server_needs_update = false;
        let store = self.context_server_store(cx);
        let root_path = self.active_project_directory(cx);
        let store_weak = store.downgrade();
        self.context_server_update_task = Some(cx.spawn(async move |this, cx| {
            if let Err(err) =
                ContextServerStore::maintain_servers(store_weak.clone(), root_path, cx).await
            {
                log::error!("Error maintaining context servers: {}", err);
            }

            store_weak
                .update(cx, |store, cx| {
                    store.populate_server_ids(cx);
                    cx.notify();
                })
                .log_err();

            this.update(cx, |this, cx| {
                this.context_server_update_task.take();
                if this.context_server_needs_update {
                    this.available_context_servers_changed(cx);
                }
            })
            .log_err();
        }));
    }

    #[inline]
    pub fn buffer_for_id(&self, remote_id: BufferId, cx: &App) -> Option<Entity<Buffer>> {
        // Filter through this Project's owned set so that Phase 2
        // sibling-Project buffers don't bleed through.
        if !self.buffers.contains(&remote_id) {
            return None;
        }
        self.buffer_store(cx).read(cx).get(remote_id)
    }

    #[inline]
    pub fn languages(&self) -> &Arc<LanguageRegistry> {
        &self.languages
    }

    #[inline]
    pub fn client(&self) -> Arc<Client> {
        self.collab_client.clone()
    }

    #[inline]
    pub fn remote_client(&self, cx: &App) -> Option<Entity<RemoteClient>> {
        self.host.read(cx).remote_client.clone()
    }

    #[inline]
    pub fn user_store(&self, cx: &App) -> Entity<UserStore> {
        self.host.read(cx).user_store.clone()
    }

    #[inline]
    pub fn node_runtime(&self, cx: &App) -> Option<NodeRuntime> {
        self.host.read(cx).node.clone()
    }

    #[inline]
    pub fn opened_buffers(&self, cx: &App) -> Vec<Entity<Buffer>> {
        // Iterate this Project's owned ids and resolve through the host
        // BufferStore. The store may hold buffers owned by sibling
        // Projects in Phase 2; we surface only ours.
        let buffer_store = self.buffer_store(cx);
        let buffer_store = buffer_store.read(cx);
        self.buffers
            .iter()
            .filter_map(|id| buffer_store.get(*id))
            .collect()
    }

    #[inline]
    pub fn environment(&self, cx: &App) -> Entity<ProjectEnvironment> {
        self.host.read(cx).environment.clone()
    }

    #[inline]
    pub fn cli_environment(&self, cx: &App) -> Option<HashMap<String, String>> {
        self.host
            .read(cx)
            .environment
            .read(cx)
            .get_cli_environment()
    }

    #[inline]
    pub fn peek_environment_error<'a>(&'a self, cx: &'a App) -> Option<&'a String> {
        self.host
            .read(cx)
            .environment
            .read(cx)
            .peek_environment_error()
    }

    #[inline]
    pub fn pop_environment_error(&mut self, cx: &mut Context<Self>) {
        let environment = self.host.read(cx).environment.clone();
        environment.update(cx, |environment, _| {
            environment.pop_environment_error();
        });
    }

    #[cfg(feature = "test-support")]
    #[inline]
    pub fn has_open_buffer(&self, path: impl Into<ProjectPath>, cx: &App) -> bool {
        let buffer_store = self.buffer_store(cx);
        let buffer_store = buffer_store.read(cx);
        buffer_store
            .buffer_id_for_project_path(&path.into())
            .map(|id| self.buffers.contains(id))
            .unwrap_or(false)
    }

    #[inline]
    pub fn fs(&self, cx: &App) -> Arc<dyn Fs> {
        self.host.read(cx).fs.clone()
    }

    #[inline]
    pub fn remote_id(&self) -> Option<u64> {
        match self.client_state {
            ProjectClientState::Local => None,
            ProjectClientState::Shared { remote_id, .. }
            | ProjectClientState::Collab { remote_id, .. } => Some(remote_id),
        }
    }

    #[inline]
    pub fn supports_terminal(&self, cx: &App) -> bool {
        self.is_local(cx) || self.is_via_remote_server(cx)
    }

    #[inline]
    pub fn remote_connection_state(&self, cx: &App) -> Option<remote::ConnectionState> {
        self.host
            .read(cx)
            .remote_client
            .as_ref()
            .map(|remote| remote.read(cx).connection_state())
    }

    pub fn remote_connection_options(&self, cx: &App) -> Option<RemoteConnectionOptions> {
        self.host
            .read(cx)
            .remote_client
            .as_ref()
            .map(|remote| remote.read(cx).connection_options())
    }

    /// Reveals the given path in the system file manager.
    ///
    /// On Windows with a WSL remote connection, this converts the POSIX path
    /// to a Windows UNC path before revealing.
    pub fn reveal_path(&self, path: &Path, cx: &mut Context<Self>) {
        #[cfg(target_os = "windows")]
        if let Some(RemoteConnectionOptions::Wsl(wsl_options)) = self.remote_connection_options(cx)
        {
            let path = path.to_path_buf();
            cx.spawn(async move |_, cx| {
                wsl_path_to_windows_path(&wsl_options, &path)
                    .await
                    .map(|windows_path| cx.update(|cx| cx.reveal_path(&windows_path)))
            })
            .detach_and_log_err(cx);
            return;
        }

        cx.reveal_path(path);
    }

    #[inline]
    pub fn replica_id(&self, cx: &App) -> ReplicaId {
        match self.client_state {
            ProjectClientState::Collab { replica_id, .. } => replica_id,
            _ => {
                if self.host.read(cx).remote_client.is_some() {
                    ReplicaId::REMOTE_SERVER
                } else {
                    ReplicaId::LOCAL
                }
            }
        }
    }

    #[inline]
    pub fn task_store(&self, cx: &App) -> Entity<TaskStore> {
        self.host.read(cx).task_store.clone()
    }

    #[inline]
    pub fn settings_observer(&self, cx: &App) -> Entity<SettingsObserver> {
        self.host.read(cx).settings_observer.clone()
    }

    #[inline]
    pub fn snippets(&self, cx: &App) -> Entity<SnippetProvider> {
        self.host.read(cx).snippets.clone()
    }

    #[inline]
    pub fn search_history(&self, kind: SearchInputKind) -> &SearchHistory {
        match kind {
            SearchInputKind::Query => &self.search_history,
            SearchInputKind::Include => &self.search_included_history,
            SearchInputKind::Exclude => &self.search_excluded_history,
        }
    }

    #[inline]
    pub fn search_history_mut(&mut self, kind: SearchInputKind) -> &mut SearchHistory {
        match kind {
            SearchInputKind::Query => &mut self.search_history,
            SearchInputKind::Include => &mut self.search_included_history,
            SearchInputKind::Exclude => &mut self.search_excluded_history,
        }
    }

    #[inline]
    pub fn collaborators(&self) -> &HashMap<proto::PeerId, Collaborator> {
        &self.collaborators
    }

    #[inline]
    pub fn host(&self) -> Option<&Collaborator> {
        self.collaborators.values().find(|c| c.is_host)
    }

    /// Collect all worktrees this `Project` owns, including ones that
    /// don't appear in the project panel.
    ///
    /// This iterates the `Project`'s own `worktrees` list (a per-project
    /// view) rather than the host's `WorktreeStore`. With Phase 2 host
    /// sharing the host's store can hold worktrees from sibling
    /// `Project`s targeting the same machine; this filter narrows to
    /// just the ones owned by this project.
    #[inline]
    pub fn worktrees<'a>(
        &'a self,
        _cx: &App,
    ) -> impl 'a + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktrees.iter().filter_map(|handle| handle.upgrade())
    }

    /// Collect all user-visible worktrees, the ones that appear in the project panel.
    #[inline]
    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktrees(cx)
            .filter(move |worktree| worktree.read(cx).is_visible())
    }

    pub(crate) fn default_visible_worktree_paths(
        worktree_store: &WorktreeStore,
        cx: &App,
    ) -> Vec<PathBuf> {
        worktree_store
            .visible_worktrees(cx)
            .sorted_by(|left, right| {
                left.read(cx)
                    .is_single_file()
                    .cmp(&right.read(cx).is_single_file())
            })
            .filter_map(|worktree| {
                let worktree = worktree.read(cx);
                let path = worktree.abs_path();
                if worktree.is_single_file() {
                    Some(path.parent()?.to_path_buf())
                } else {
                    Some(path.to_path_buf())
                }
            })
            .collect()
    }

    pub fn default_path_list(&self, cx: &App) -> PathList {
        // Use this Project's visible worktrees, not the host store's
        // full set: in Phase 2 sharing the host may contain sibling
        // Projects' worktrees that aren't part of our path list.
        let worktree_roots: Vec<PathBuf> = self
            .visible_worktrees(cx)
            .sorted_by(|left, right| {
                left.read(cx)
                    .is_single_file()
                    .cmp(&right.read(cx).is_single_file())
            })
            .filter_map(|worktree| {
                let worktree = worktree.read(cx);
                let path = worktree.abs_path();
                if worktree.is_single_file() {
                    Some(path.parent()?.to_path_buf())
                } else {
                    Some(path.to_path_buf())
                }
            })
            .collect();

        if worktree_roots.is_empty() {
            PathList::new(&[paths::home_dir().as_path()])
        } else {
            PathList::new(&worktree_roots)
        }
    }

    #[inline]
    pub fn worktree_for_root_name(&self, root_name: &str, cx: &App) -> Option<Entity<Worktree>> {
        self.visible_worktrees(cx)
            .find(|tree| tree.read(cx).root_name() == root_name)
    }

    fn emit_group_key_changed_if_needed(&mut self, cx: &mut Context<Self>) {
        let new_worktree_paths = self.worktree_paths(cx);
        if new_worktree_paths != self.last_worktree_paths {
            let old_worktree_paths =
                std::mem::replace(&mut self.last_worktree_paths, new_worktree_paths);
            cx.emit(Event::WorktreePathsChanged { old_worktree_paths });
        }
    }

    #[inline]
    pub fn worktree_root_names<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = &'a str> {
        self.visible_worktrees(cx)
            .map(|tree| tree.read(cx).root_name().as_unix_str())
    }

    #[inline]
    pub fn worktree_for_id(&self, id: WorktreeId, cx: &App) -> Option<Entity<Worktree>> {
        self.worktrees(cx)
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &App,
    ) -> Option<Entity<Worktree>> {
        self.worktree_store(cx)
            .read(cx)
            .worktree_for_entry(entry_id, cx)
    }

    #[inline]
    pub fn worktree_id_for_entry(&self, entry_id: ProjectEntryId, cx: &App) -> Option<WorktreeId> {
        self.worktree_for_entry(entry_id, cx)
            .map(|worktree| worktree.read(cx).id())
    }

    /// Checks if the entry is the root of a worktree.
    #[inline]
    pub fn entry_is_worktree_root(&self, entry_id: ProjectEntryId, cx: &App) -> bool {
        self.worktree_for_entry(entry_id, cx)
            .map(|worktree| {
                worktree
                    .read(cx)
                    .root_entry()
                    .is_some_and(|e| e.id == entry_id)
            })
            .unwrap_or(false)
    }

    #[inline]
    pub fn project_path_git_status(
        &self,
        project_path: &ProjectPath,
        cx: &App,
    ) -> Option<FileStatus> {
        self.git_store(cx)
            .read(cx)
            .project_path_git_status(project_path, cx)
    }

    #[inline]
    pub fn visibility_for_paths(
        &self,
        paths: &[PathBuf],
        exclude_sub_dirs: bool,
        cx: &App,
    ) -> Option<bool> {
        paths
            .iter()
            .map(|path| self.visibility_for_path(path, exclude_sub_dirs, cx))
            .max()
            .flatten()
    }

    pub fn visibility_for_path(
        &self,
        path: &Path,
        exclude_sub_dirs: bool,
        cx: &App,
    ) -> Option<bool> {
        let path = SanitizedPath::new(path).as_path();
        let path_style = self.path_style(cx);
        self.worktrees(cx)
            .filter_map(|worktree| {
                let worktree = worktree.read(cx);
                let abs_path = worktree.abs_path();
                let relative_path = path_style.strip_prefix(path, abs_path.as_ref());
                let is_dir = relative_path
                    .as_ref()
                    .and_then(|p| worktree.entry_for_path(p))
                    .is_some_and(|e| e.is_dir());
                // Don't exclude the worktree root itself, only actual subdirectories
                let is_subdir = relative_path
                    .as_ref()
                    .is_some_and(|p| !p.as_ref().as_unix_str().is_empty());
                let contains =
                    relative_path.is_some() && (!exclude_sub_dirs || !is_dir || !is_subdir);
                contains.then(|| worktree.is_visible())
            })
            .max()
    }

    pub fn create_entry(
        &mut self,
        project_path: impl Into<ProjectPath>,
        is_directory: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<CreatedEntry>> {
        let project_path = project_path.into();
        let Some(worktree) = self.worktree_for_id(project_path.worktree_id, cx) else {
            return Task::ready(Err(anyhow!(format!(
                "No worktree for path {project_path:?}"
            ))));
        };
        worktree.update(cx, |worktree, cx| {
            worktree.create_entry(project_path.path, is_directory, None, cx)
        })
    }

    #[inline]
    pub fn copy_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Entry>>> {
        self.worktree_store(cx).update(cx, |worktree_store, cx| {
            worktree_store.copy_entry(entry_id, new_project_path, cx)
        })
    }

    /// Renames the project entry with given `entry_id`.
    ///
    /// `new_path` is a relative path to worktree root.
    /// If root entry is renamed then its new root name is used instead.
    pub fn rename_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<CreatedEntry>> {
        let worktree_store = self.worktree_store(cx);
        let Some((worktree, old_path, is_dir)) = worktree_store
            .read(cx)
            .worktree_and_entry_for_id(entry_id, cx)
            .map(|(worktree, entry)| (worktree, entry.path.clone(), entry.is_dir()))
        else {
            return Task::ready(Err(anyhow!(format!("No worktree for entry {entry_id:?}"))));
        };

        let worktree_id = worktree.read(cx).id();
        let is_root_entry = self.entry_is_worktree_root(entry_id, cx);

        let lsp_store = self.lsp_store(cx).downgrade();
        let active_entry = self.active_entry;
        cx.spawn(async move |project, cx| {
            let (old_abs_path, new_abs_path) = {
                let root_path = worktree.read_with(cx, |this, _| this.abs_path());
                let new_abs_path = if is_root_entry {
                    root_path
                        .parent()
                        .unwrap()
                        .join(new_path.path.as_std_path())
                } else {
                    root_path.join(&new_path.path.as_std_path())
                };
                (root_path.join(old_path.as_std_path()), new_abs_path)
            };
            let transaction = LspStore::will_rename_entry(
                lsp_store.clone(),
                worktree_id,
                &old_abs_path,
                &new_abs_path,
                is_dir,
                active_entry,
                cx.clone(),
            )
            .await;

            let entry = worktree_store
                .update(cx, |worktree_store, cx| {
                    worktree_store.rename_entry(entry_id, new_path.clone(), cx)
                })
                .await?;

            project
                .update(cx, |_, cx| {
                    cx.emit(Event::EntryRenamed(
                        transaction,
                        new_path.clone(),
                        new_abs_path.clone(),
                    ));
                })
                .ok();

            lsp_store
                .read_with(cx, |this, _| {
                    this.did_rename_entry(worktree_id, &old_abs_path, &new_abs_path, is_dir);
                })
                .ok();
            Ok(entry)
        })
    }

    #[inline]
    pub fn delete_file(
        &mut self,
        path: ProjectPath,
        trash: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<Option<TrashedEntry>>>> {
        let entry = self.entry_for_path(&path, cx)?;
        self.delete_entry(entry.id, trash, cx)
    }

    #[inline]
    pub fn delete_entry(
        &mut self,
        entry_id: ProjectEntryId,
        trash: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<Option<TrashedEntry>>>> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        cx.emit(Event::DeletedEntry(worktree.read(cx).id(), entry_id));
        worktree.update(cx, |worktree, cx| {
            worktree.delete_entry(entry_id, trash, cx)
        })
    }

    #[inline]
    pub fn restore_entry(
        &self,
        worktree_id: WorktreeId,
        trash_entry: TrashedEntry,
        cx: &mut Context<'_, Self>,
    ) -> Task<Result<ProjectPath>> {
        let Some(worktree) = self.worktree_for_id(worktree_id, cx) else {
            return Task::ready(Err(anyhow!("No worktree for id {worktree_id:?}")));
        };

        cx.spawn(async move |_, cx| {
            Worktree::restore_entry(trash_entry, worktree, cx)
                .await
                .map(|rel_path_buf| ProjectPath {
                    worktree_id: worktree_id,
                    path: Arc::from(rel_path_buf.as_rel_path()),
                })
        })
    }

    #[inline]
    pub fn expand_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree_for_id(worktree_id, cx)?;
        worktree.update(cx, |worktree, cx| worktree.expand_entry(entry_id, cx))
    }

    pub fn expand_all_for_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree_for_id(worktree_id, cx)?;
        let task = worktree.update(cx, |worktree, cx| {
            worktree.expand_all_for_entry(entry_id, cx)
        });
        Some(cx.spawn(async move |this, cx| {
            task.context("no task")?.await?;
            this.update(cx, |_, cx| {
                cx.emit(Event::ExpandedAllForEntry(worktree_id, entry_id));
            })?;
            Ok(())
        }))
    }

    /// Pin every loaded worktree as a strong handle on this project. Called
    /// when entering a state where every worktree must outlive its sole
    /// external user (e.g. while shared via collab). Mirrors the previous
    /// `WorktreeStore::retain_all_worktrees` behavior, now per-project.
    fn retain_all_worktrees(&mut self) {
        self.retain_worktrees = true;
        for handle in self.worktrees.iter_mut() {
            if let WorktreeHandle::Weak(weak) = handle
                && let Some(worktree) = weak.upgrade()
            {
                *handle = WorktreeHandle::Strong(worktree);
            }
        }
    }

    /// Reverts `retain_all_worktrees`: invisible worktrees become weak again.
    /// If they have no other holder they will be released, and the host's
    /// `WorktreeStore` will clean up its registry entry.
    fn release_invisible_worktrees(&mut self, cx: &mut App) {
        self.retain_worktrees = false;
        for handle in self.worktrees.iter_mut() {
            if let WorktreeHandle::Strong(worktree) = handle {
                let is_visible = worktree.read(cx).is_visible();
                if !is_visible {
                    *handle = WorktreeHandle::Weak(worktree.downgrade());
                }
            }
        }
        self.worktree_store(cx).update(cx, |store, _| {
            store.cleanup_released_worktrees();
        });
    }

    /// Send the initial worktree metadata downstream and (re)set up
    /// per-worktree observers for streaming updates. Used when entering or
    /// re-entering a collab-shared state.
    fn send_worktree_project_updates(&mut self, project_id: u64, cx: &mut Context<Self>) {
        let downstream_client: AnyProtoClient = self.collab_client.clone().into();
        // Use this Project's filtered metadata, not the host's full set:
        // in Phase 2, the shared `WorktreeStore` may contain worktrees
        // owned by sibling Projects on the same host, and announcing
        // them under our `project_id` would silently leak those
        // worktrees into our collab share.
        let metadata = self.worktree_metadata_protos(cx);
        let update = proto::UpdateProject {
            project_id,
            worktrees: metadata,
        };

        // collab has bad concurrency guarantees, so we send requests in serial.
        let update_project = if downstream_client.is_via_collab() {
            Some(downstream_client.request(update))
        } else {
            downstream_client.send(update).log_err();
            None
        };

        cx.spawn(async move |this, cx| {
            if let Some(update_project) = update_project {
                update_project.await?;
            }
            this.update(cx, |this, cx| {
                let worktrees: Vec<_> = this.worktrees(cx).collect();
                this.worktree_store(cx).update(cx, |store, cx| {
                    store.observe_worktrees_for_downstream(
                        worktrees,
                        downstream_client,
                        project_id,
                        cx,
                    );
                });
                anyhow::Ok(())
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn shared(&mut self, project_id: u64, cx: &mut Context<Self>) -> Result<()> {
        anyhow::ensure!(
            matches!(self.client_state, ProjectClientState::Local),
            "project was already shared"
        );

        self.client_subscriptions.extend([
            self.collab_client
                .subscribe_to_entity(project_id)?
                .set_entity(&cx.entity(), &cx.to_async()),
            self.collab_client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.worktree_store(cx), &cx.to_async()),
            self.collab_client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.buffer_store(cx), &cx.to_async()),
            self.collab_client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.lsp_store(cx), &cx.to_async()),
            self.collab_client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.host.read(cx).settings_observer, &cx.to_async()),
            self.collab_client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.dap_store(cx), &cx.to_async()),
            self.collab_client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.breakpoint_store(cx), &cx.to_async()),
            self.collab_client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.git_store(cx), &cx.to_async()),
        ]);

        self.retain_all_worktrees();
        self.send_worktree_project_updates(project_id, cx);
        // Announce all language servers that belong to this project. After
        // Phase 2 the worktree filter actually narrows the list; in Phase 0
        // every server in this project's lsp_store is owned by this project,
        // so the filter is a no-op (but encodes the right invariant).
        let my_worktree_ids: HashSet<WorktreeId> =
            self.worktrees(cx).map(|w| w.read(cx).id()).collect();
        let lsp_store = self.lsp_store(cx).read(cx);
        let server_announcements: Vec<_> = lsp_store
            .language_server_statuses
            .iter()
            .filter(|(_, status)| {
                status
                    .worktree
                    .map_or(true, |id| my_worktree_ids.contains(&id))
            })
            .filter_map(|(server_id, status)| {
                let capabilities = lsp_store.lsp_server_capabilities.get(server_id)?;
                Some(proto::StartLanguageServer {
                    project_id,
                    server: Some(proto::LanguageServer {
                        id: server_id.to_proto(),
                        name: status.name.to_string(),
                        worktree_id: status.worktree.map(|id| id.to_proto()),
                    }),
                    capabilities: serde_json::to_string(capabilities)
                        .expect("serializing server LSP capabilities"),
                })
            })
            .collect();
        for announcement in server_announcements {
            self.collab_client.send(announcement).log_err();
        }
        let initial_settings_protos = self
            .host
            .read(cx)
            .settings_observer
            .read(cx)
            .initial_worktree_settings_protos(project_id, cx);
        for proto in initial_settings_protos {
            self.collab_client.send(proto).log_err();
        }
        // Announce all current repositories to the downstream peer. The
        // diff/send pipeline used to live inside `GitStore::shared`; in
        // Phase 0 it moves here, with `git_repository_snapshots_for_peer`
        // tracking what we last sent so subsequent
        // `RepositorySnapshotForDownstream` events can build incremental
        // updates.
        let initial_snapshots: Vec<RepositorySnapshot> = self
            .git_store(cx)
            .read(cx)
            .repositories()
            .values()
            .map(|repo| repo.read(cx).snapshot())
            .collect();
        for snapshot in initial_snapshots {
            for chunk in proto::split_repository_update(snapshot.initial_update(project_id)) {
                self.collab_client.send(chunk).log_err();
            }
            self.git_repository_snapshots_for_peer
                .insert(snapshot.id, snapshot);
        }

        self.client_state = ProjectClientState::Shared {
            remote_id: project_id,
        };

        cx.emit(Event::RemoteIdChanged(Some(project_id)));
        Ok(())
    }

    pub fn reshared(
        &mut self,
        message: proto::ResharedProject,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.forget_shared_buffers();
        self.set_collaborators_from_proto(message.collaborators, cx)?;

        if let Some(remote_id) = self.remote_id() {
            self.send_worktree_project_updates(remote_id, cx);
        }
        if let Some(remote_id) = self.remote_id() {
            // Re-announce all repositories to the new peer. Previous
            // snapshots are stale (the new peer has no state yet), so clear
            // and re-send full initial updates.
            self.git_repository_snapshots_for_peer.clear();
            let initial_snapshots: Vec<RepositorySnapshot> = self
                .git_store(cx)
                .read(cx)
                .repositories()
                .values()
                .map(|repo| repo.read(cx).snapshot())
                .collect();
            for snapshot in initial_snapshots {
                for chunk in proto::split_repository_update(snapshot.initial_update(remote_id)) {
                    self.collab_client.send(chunk).log_err();
                }
                self.git_repository_snapshots_for_peer
                    .insert(snapshot.id, snapshot);
            }
        }
        cx.emit(Event::Reshared);
        Ok(())
    }

    pub fn rejoined(
        &mut self,
        message: proto::RejoinedProject,
        message_id: u32,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            for worktree_metadata in &message.worktrees {
                store
                    .clear_local_settings(WorktreeId::from_proto(worktree_metadata.id), cx)
                    .log_err();
            }
        });

        self.join_project_response_message_id = message_id;
        self.set_worktrees_from_proto(message.worktrees, cx)?;
        self.set_collaborators_from_proto(message.collaborators, cx)?;

        let project = cx.weak_entity();
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.set_language_server_statuses_from_proto(
                project,
                message.language_servers,
                message.language_server_capabilities,
                cx,
            )
        });
        self.enqueue_buffer_ordered_message(BufferOrderedMessage::Resync)
            .unwrap();
        cx.emit(Event::Rejoined);
        Ok(())
    }

    #[inline]
    pub fn unshare(&mut self, cx: &mut Context<Self>) -> Result<()> {
        self.unshare_internal(cx)?;
        cx.emit(Event::RemoteIdChanged(None));
        Ok(())
    }

    fn unshare_internal(&mut self, cx: &mut App) -> Result<()> {
        anyhow::ensure!(
            !self.is_via_collab(),
            "attempted to unshare a remote project"
        );

        if let ProjectClientState::Shared { remote_id, .. } = self.client_state {
            self.client_state = ProjectClientState::Local;
            self.collaborators.clear();
            self.client_subscriptions.clear();
            // Stop observing only THIS project's worktrees: in Phase 2
            // sharing, the host's `WorktreeStore` may also hold sibling
            // Projects' worktrees that are still actively shared.
            let our_worktrees: Vec<_> = self.worktrees(cx).collect();
            self.worktree_store(cx).update(cx, |store, cx| {
                store.stop_observing_worktrees(our_worktrees, cx);
            });
            self.release_invisible_worktrees(cx);
            self.forget_shared_buffers();
            self.git_store(cx).update(cx, |git_store, _| {
                git_store.forget_all_shared_diffs();
            });
            self.git_repository_snapshots_for_peer.clear();

            self.collab_client
                .send(proto::UnshareProject {
                    project_id: remote_id,
                })
                .ok();
            Ok(())
        } else {
            anyhow::bail!("attempted to unshare an unshared project");
        }
    }

    pub fn disconnected_from_host(&mut self, cx: &mut Context<Self>) {
        if self.is_disconnected(cx) {
            return;
        }
        self.disconnected_from_host_internal(cx);
        cx.emit(Event::DisconnectedFromHost);
    }

    pub fn set_role(&mut self, role: proto::ChannelRole, cx: &mut Context<Self>) {
        let new_capability =
            if role == proto::ChannelRole::Member || role == proto::ChannelRole::Admin {
                Capability::ReadWrite
            } else {
                Capability::ReadOnly
            };
        if let ProjectClientState::Collab { capability, .. } = &mut self.client_state {
            if *capability == new_capability {
                return;
            }

            *capability = new_capability;
            for buffer in self.opened_buffers(cx) {
                buffer.update(cx, |buffer, cx| buffer.set_capability(new_capability, cx));
            }
        }
    }

    fn disconnected_from_host_internal(&mut self, cx: &mut App) {
        if let ProjectClientState::Collab {
            sharing_has_stopped,
            ..
        } = &mut self.client_state
        {
            *sharing_has_stopped = true;
            self.client_subscriptions.clear();
            self.collaborators.clear();
            self.worktree_store(cx).update(cx, |store, cx| {
                store.disconnected_from_host(cx);
            });
            self.buffer_store(cx).update(cx, |buffer_store, cx| {
                buffer_store.disconnected_from_host(cx)
            });
        }
    }

    #[inline]
    pub fn close(&mut self, cx: &mut Context<Self>) {
        cx.emit(Event::Closed);
    }

    #[inline]
    pub fn is_disconnected(&self, cx: &App) -> bool {
        match &self.client_state {
            ProjectClientState::Collab {
                sharing_has_stopped,
                ..
            } => *sharing_has_stopped,
            ProjectClientState::Local if self.is_via_remote_server(cx) => {
                self.remote_client_is_disconnected(cx)
            }
            _ => false,
        }
    }

    #[inline]
    fn remote_client_is_disconnected(&self, cx: &App) -> bool {
        self.host
            .read(cx)
            .remote_client
            .as_ref()
            .map(|remote| remote.read(cx).is_disconnected())
            .unwrap_or(false)
    }

    #[inline]
    pub fn capability(&self) -> Capability {
        match &self.client_state {
            ProjectClientState::Collab { capability, .. } => *capability,
            ProjectClientState::Shared { .. } | ProjectClientState::Local => Capability::ReadWrite,
        }
    }

    #[inline]
    pub fn is_read_only(&self, cx: &App) -> bool {
        self.is_disconnected(cx) || !self.capability().editable()
    }

    #[inline]
    pub fn is_local(&self, cx: &App) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => {
                self.host.read(cx).remote_client.is_none()
            }
            ProjectClientState::Collab { .. } => false,
        }
    }

    /// Whether this project is a remote server (not counting collab).
    #[inline]
    pub fn is_via_remote_server(&self, cx: &App) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => {
                self.host.read(cx).remote_client.is_some()
            }
            ProjectClientState::Collab { .. } => false,
        }
    }

    /// Whether this project is from collab (not counting remote servers).
    #[inline]
    pub fn is_via_collab(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => false,
            ProjectClientState::Collab { .. } => true,
        }
    }

    /// `!self.is_local(cx)`
    #[inline]
    pub fn is_remote(&self, cx: &App) -> bool {
        debug_assert_eq!(
            !self.is_local(cx),
            self.is_via_collab() || self.is_via_remote_server(cx)
        );
        !self.is_local(cx)
    }

    #[inline]
    pub fn is_via_wsl_with_host_interop(&self, cx: &App) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => {
                matches!(
                    &self.host.read(cx).remote_client, Some(remote_client)
                    if remote_client.read(cx).has_wsl_interop()
                )
            }
            _ => false,
        }
    }

    pub fn disable_worktree_scanner(&mut self, cx: &mut Context<Self>) {
        self.worktree_store(cx).update(cx, |worktree_store, _cx| {
            worktree_store.disable_scanner();
        });
    }

    pub fn create_buffer(
        &mut self,
        language: Option<Arc<Language>>,
        project_searchable: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        let task = self.buffer_store(cx).update(cx, |buffer_store, cx| {
            buffer_store.create_buffer(language, project_searchable, cx)
        });
        // Pathless buffers don't match the worktree-membership rule the
        // `BufferAdded` handler uses to claim, so the initiating Project
        // must claim them explicitly once the load completes.
        cx.spawn(async move |this, cx| {
            let buffer = task.await?;
            this.update(cx, |this, cx| this.claim_buffer(&buffer, cx))?;
            Ok(buffer)
        })
    }

    pub fn create_local_buffer(
        &mut self,
        text: &str,
        language: Option<Arc<Language>>,
        project_searchable: bool,
        cx: &mut Context<Self>,
    ) -> Entity<Buffer> {
        if self.is_remote(cx) {
            panic!("called create_local_buffer on a remote project")
        }
        let buffer = self.buffer_store(cx).update(cx, |buffer_store, cx| {
            buffer_store.create_local_buffer(text, language, project_searchable, cx)
        });
        // Pre-claim before the queued `BufferAdded` event reaches our
        // subscriber so the path-based handler sees the buffer as
        // already-owned and skips re-registering its subscription.
        self.claim_buffer(&buffer, cx);
        buffer
    }

    pub fn open_path(
        &mut self,
        path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Option<ProjectEntryId>, Entity<Buffer>)>> {
        let task = self.open_buffer(path, cx);
        cx.spawn(async move |_project, cx| {
            let buffer = task.await?;
            let project_entry_id = buffer.read_with(cx, |buffer, _cx| {
                File::from_dyn(buffer.file()).and_then(|file| file.project_entry_id())
            });

            Ok((project_entry_id, buffer))
        })
    }

    pub fn open_local_buffer(
        &mut self,
        abs_path: impl AsRef<Path>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        let worktree_task = self.find_or_create_worktree(abs_path.as_ref(), false, cx);
        cx.spawn(async move |this, cx| {
            let (worktree, relative_path) = worktree_task.await?;
            this.update(cx, |this, cx| {
                this.open_buffer((worktree.read(cx).id(), relative_path), cx)
            })?
            .await
        })
    }

    #[cfg(feature = "test-support")]
    pub fn open_local_buffer_with_lsp(
        &mut self,
        abs_path: impl AsRef<Path>,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Entity<Buffer>, lsp_store::OpenLspBufferHandle)>> {
        if let Some((worktree, relative_path)) = self.find_worktree(abs_path.as_ref(), cx) {
            self.open_buffer_with_lsp((worktree.read(cx).id(), relative_path), cx)
        } else {
            Task::ready(Err(anyhow!("no such path")))
        }
    }

    pub fn download_file(
        &mut self,
        worktree_id: WorktreeId,
        path: Arc<RelPath>,
        destination_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        log::debug!(
            "download_file called: worktree_id={:?}, path={:?}, destination={:?}",
            worktree_id,
            path,
            destination_path
        );

        let Some(remote_client) = &self.host.read(cx).remote_client else {
            log::error!("download_file: not a remote project");
            return Task::ready(Err(anyhow!("not a remote project")));
        };

        let proto_client = remote_client.read(cx).proto_client();
        // For SSH remote projects, use REMOTE_SERVER_PROJECT_ID instead of remote_id()
        // because SSH projects have client_state: Local but still need to communicate with remote server
        let project_id = self.remote_id().unwrap_or(REMOTE_SERVER_PROJECT_ID);
        let downloading_files = self.downloading_files.clone();
        let path_str = path.to_proto();

        static NEXT_FILE_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let file_id = NEXT_FILE_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        // Register BEFORE sending request to avoid race condition
        let key = (worktree_id, path_str.clone());
        log::debug!(
            "download_file: pre-registering download with key={:?}, file_id={}",
            key,
            file_id
        );
        downloading_files.lock().insert(
            key,
            DownloadingFile {
                destination_path: destination_path,
                chunks: Vec::new(),
                total_size: 0,
                file_id: Some(file_id),
            },
        );
        log::debug!(
            "download_file: sending DownloadFileByPath request, path_str={}",
            path_str
        );

        cx.spawn(async move |_this, _cx| {
            log::debug!("download_file: sending request with file_id={}...", file_id);
            let response = proto_client
                .request(proto::DownloadFileByPath {
                    project_id,
                    worktree_id: worktree_id.to_proto(),
                    path: path_str.clone(),
                    file_id,
                })
                .await?;

            log::debug!("download_file: got response, file_id={}", response.file_id);
            // The file_id is set from the State message, we just confirm the request succeeded
            Ok(())
        })
    }

    #[ztracing::instrument(skip_all)]
    pub fn open_buffer(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut App,
    ) -> Task<Result<Entity<Buffer>>> {
        if self.is_disconnected(cx) {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }

        self.buffer_store(cx).update(cx, |buffer_store, cx| {
            buffer_store.open_buffer(path.into(), cx)
        })
    }

    #[cfg(feature = "test-support")]
    pub fn open_buffer_with_lsp(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Entity<Buffer>, lsp_store::OpenLspBufferHandle)>> {
        let buffer = self.open_buffer(path, cx);
        cx.spawn(async move |this, cx| {
            let buffer = buffer.await?;
            let handle = this.update(cx, |project, cx| {
                project.register_buffer_with_language_servers(&buffer, cx)
            })?;
            Ok((buffer, handle))
        })
    }

    pub fn register_buffer_with_language_servers(
        &self,
        buffer: &Entity<Buffer>,
        cx: &mut App,
    ) -> OpenLspBufferHandle {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.register_buffer_with_language_servers(buffer, HashSet::default(), false, cx)
        })
    }

    pub fn open_unstaged_diff(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferDiff>>> {
        if self.is_disconnected(cx) {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }
        self.git_store(cx)
            .update(cx, |git_store, cx| git_store.open_unstaged_diff(buffer, cx))
    }

    #[ztracing::instrument(skip_all)]
    pub fn open_uncommitted_diff(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferDiff>>> {
        if self.is_disconnected(cx) {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }
        self.git_store(cx).update(cx, |git_store, cx| {
            git_store.open_uncommitted_diff(buffer, cx)
        })
    }

    pub fn open_buffer_by_id(
        &mut self,
        id: BufferId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        if let Some(buffer) = self.buffer_for_id(id, cx) {
            Task::ready(Ok(buffer))
        } else if self.is_local(cx) || self.is_via_remote_server(cx) {
            Task::ready(Err(anyhow!("buffer {id} does not exist")))
        } else if let Some(project_id) = self.remote_id() {
            let request = self.collab_client.request(proto::OpenBufferById {
                project_id,
                id: id.into(),
            });
            cx.spawn(async move |project, cx| {
                let buffer_id = BufferId::new(request.await?.buffer_id)?;
                project
                    .update(cx, |project, cx| {
                        project.buffer_store(cx).update(cx, |buffer_store, cx| {
                            buffer_store.wait_for_remote_buffer(buffer_id, cx)
                        })
                    })?
                    .await
            })
        } else {
            Task::ready(Err(anyhow!("cannot open buffer while disconnected")))
        }
    }

    pub fn save_buffers(
        &self,
        buffers: HashSet<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        cx.spawn(async move |this, cx| {
            let save_tasks = buffers.into_iter().filter_map(|buffer| {
                this.update(cx, |this, cx| this.save_buffer(buffer, cx))
                    .ok()
            });
            try_join_all(save_tasks).await?;
            Ok(())
        })
    }

    pub fn save_buffer(&self, buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.buffer_store(cx)
            .update(cx, |buffer_store, cx| buffer_store.save_buffer(buffer, cx))
    }

    pub fn save_buffer_as(
        &mut self,
        buffer: Entity<Buffer>,
        path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.buffer_store(cx).update(cx, |buffer_store, cx| {
            buffer_store.save_buffer_as(buffer.clone(), path, cx)
        })
    }

    pub fn get_open_buffer(&self, path: &ProjectPath, cx: &App) -> Option<Entity<Buffer>> {
        let buffer = self.buffer_store(cx).read(cx).get_by_path(path)?;
        if !self.buffers.contains(&buffer.read(cx).remote_id()) {
            return None;
        }
        Some(buffer)
    }

    fn register_buffer(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) -> Result<()> {
        {
            let mut remotely_created_models = self.remotely_created_models.lock();
            if remotely_created_models.retain_count > 0 {
                remotely_created_models.buffers.push(buffer.clone())
            }
        }

        self.request_buffer_diff_recalculation(buffer, cx);

        cx.subscribe(buffer, |this, buffer, event, cx| {
            this.on_buffer_event(buffer, event, cx);
        })
        .detach();

        Ok(())
    }

    /// Idempotently mark `buffer` as owned by this Project and run the
    /// one-time `register_buffer` setup (buffer-event subscription, diff
    /// recalculation, etc). Used by:
    ///
    /// - `on_buffer_store_event`'s path-based claim for file-backed
    ///   buffers in our worktrees.
    /// - Pre-claim call sites (`create_local_buffer`, `create_buffer`,
    ///   `handle_create_buffer_for_peer`) for buffers whose ownership
    ///   isn't expressible via the worktree-membership rule.
    ///
    /// Calling twice for the same buffer is safe — the second call's
    /// `HashSet::insert` returns `false` and `register_buffer` is
    /// skipped, avoiding a double subscription.
    fn claim_buffer(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        let buffer_id = buffer.read(cx).remote_id();
        if self.buffers.insert(buffer_id) {
            self.register_buffer(buffer, cx).log_err();
        }
    }

    pub fn open_image(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<ImageItem>>> {
        if self.is_disconnected(cx) {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }

        let image_store = self.host.read(cx).image_store.clone();
        let open_image_task = image_store.update(cx, |image_store, cx| {
            image_store.open_image(path.into(), cx)
        });

        let weak_project = cx.entity().downgrade();
        cx.spawn(async move |_, cx| {
            let image_item = open_image_task.await?;

            // Check if metadata already exists (e.g., for remote images)
            let needs_metadata =
                cx.read_entity(&image_item, |item, _| item.image_metadata.is_none());

            if needs_metadata {
                let project = weak_project.upgrade().context("Project dropped")?;
                let metadata =
                    ImageItem::load_image_metadata(image_item.clone(), project, cx).await?;
                image_item.update(cx, |image_item, cx| {
                    image_item.image_metadata = Some(metadata);
                    cx.emit(ImageItemEvent::MetadataUpdated);
                });
            }

            Ok(image_item)
        })
    }

    async fn send_buffer_ordered_messages(
        project: WeakEntity<Self>,
        rx: UnboundedReceiver<BufferOrderedMessage>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        const MAX_BATCH_SIZE: usize = 128;

        let mut operations_by_buffer_id = HashMap::default();
        async fn flush_operations(
            this: &WeakEntity<Project>,
            operations_by_buffer_id: &mut HashMap<BufferId, Vec<proto::Operation>>,
            needs_resync_with_host: &mut bool,
            is_local: bool,
            cx: &mut AsyncApp,
        ) -> Result<()> {
            for (buffer_id, operations) in operations_by_buffer_id.drain() {
                let request = this.read_with(cx, |this, _| {
                    let project_id = this.remote_id()?;
                    Some(this.collab_client.request(proto::UpdateBuffer {
                        buffer_id: buffer_id.into(),
                        project_id,
                        operations,
                    }))
                })?;
                if let Some(request) = request
                    && request.await.is_err()
                    && !is_local
                {
                    *needs_resync_with_host = true;
                    break;
                }
            }
            Ok(())
        }

        let mut needs_resync_with_host = false;
        let mut changes = rx.ready_chunks(MAX_BATCH_SIZE);

        while let Some(changes) = changes.next().await {
            let is_local = project.read_with(cx, |this, cx| this.is_local(cx))?;

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
                        if project
                            .update(cx, |this, cx| this.synchronize_remote_buffers(cx))?
                            .await
                            .is_ok()
                        {
                            needs_resync_with_host = false;
                        }
                    }

                    BufferOrderedMessage::LanguageServerUpdate {
                        language_server_id,
                        message,
                        name,
                    } => {
                        flush_operations(
                            &project,
                            &mut operations_by_buffer_id,
                            &mut needs_resync_with_host,
                            is_local,
                            cx,
                        )
                        .await?;

                        project.read_with(cx, |project, _| {
                            if let Some(project_id) = project.remote_id() {
                                project
                                    .collab_client
                                    .send(proto::UpdateLanguageServer {
                                        project_id,
                                        server_name: name.map(|name| String::from(name.0)),
                                        language_server_id: language_server_id.to_proto(),
                                        variant: Some(message),
                                    })
                                    .log_err();
                            }
                        })?;
                    }
                }
            }

            flush_operations(
                &project,
                &mut operations_by_buffer_id,
                &mut needs_resync_with_host,
                is_local,
                cx,
            )
            .await?;
        }

        Ok(())
    }

    fn on_buffer_store_event(
        &mut self,
        _: Entity<BufferStore>,
        event: &BufferStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            BufferStoreEvent::BufferAdded(buffer) => {
                // Path-based ownership: claim only buffers whose file
                // lives in one of *this* Project's worktrees. This is
                // the safe default when several Projects share a host
                // BufferStore in Phase 2 — each Project picks up only
                // its own file-backed buffers from the broadcast event.
                //
                // Pathless buffers (scratch/`create_local_buffer`,
                // `create_buffer`) and peer-streamed buffers are claimed
                // explicitly by their initiating call site via
                // `claim_buffer`. `claim_buffer` is idempotent, so the
                // pre-claim path and the path-based handler can both run
                // for the same buffer without double-registering its
                // event subscription.
                let owned_by_path = if let Some(file) = File::from_dyn(buffer.read(cx).file()) {
                    let worktree_id = file.worktree_id(cx);
                    self.worktrees(cx).any(|w| w.read(cx).id() == worktree_id)
                } else {
                    false
                };
                if owned_by_path {
                    self.claim_buffer(buffer, cx);
                }
            }
            BufferStoreEvent::BufferDropped(buffer_id) => {
                // Only act if this Project actually owns the buffer.
                // Skipping the proto send for non-owned buffers prevents
                // duplicate `CloseBuffer` broadcasts in Phase 2 sharing.
                if !self.buffers.remove(buffer_id) {
                    return;
                }
                if let Some(ref remote_client) = self.host.read(cx).remote_client {
                    remote_client
                        .read(cx)
                        .proto_client()
                        .send(proto::CloseBuffer {
                            project_id: 0,
                            buffer_id: buffer_id.to_proto(),
                        })
                        .log_err();
                }
            }
            BufferStoreEvent::LocalBufferReloaded(buffer) => {
                // Only the Project that owns this buffer should rebroadcast,
                // to avoid duplicate sends when multiple Projects share a host
                // BufferStore in Phase 2.
                if !self.buffers.contains(&buffer.read(cx).remote_id()) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    let buffer = buffer.read(cx);
                    self.collab_client
                        .send(proto::BufferReloaded {
                            project_id: *remote_id,
                            buffer_id: buffer.remote_id().to_proto(),
                            version: language::proto::serialize_version(&buffer.version()),
                            mtime: buffer.saved_mtime().map(|t| t.into()),
                            line_ending: language::proto::serialize_line_ending(
                                buffer.line_ending(),
                            ) as i32,
                        })
                        .log_err();
                }
            }
            BufferStoreEvent::UpdateBufferFileForwarded { buffer_id, file } => {
                if !self.buffers.contains(buffer_id) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::UpdateBufferFile {
                            project_id: *remote_id,
                            buffer_id: buffer_id.to_proto(),
                            file: file.clone(),
                        })
                        .log_err();
                }
            }
            BufferStoreEvent::BufferSavedForwarded {
                buffer_id,
                version,
                mtime,
            } => {
                if !self.buffers.contains(buffer_id) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::BufferSaved {
                            project_id: *remote_id,
                            buffer_id: buffer_id.to_proto(),
                            version: version.clone(),
                            mtime: mtime.clone(),
                        })
                        .log_err();
                }
            }
            BufferStoreEvent::BufferReloadedForwarded {
                buffer_id,
                version,
                mtime,
                line_ending,
            } => {
                if !self.buffers.contains(buffer_id) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::BufferReloaded {
                            project_id: *remote_id,
                            buffer_id: buffer_id.to_proto(),
                            version: version.clone(),
                            mtime: mtime.clone(),
                            line_ending: *line_ending,
                        })
                        .log_err();
                }
            }
            _ => {}
        }
    }

    fn on_image_store_event(
        &mut self,
        _: Entity<ImageStore>,
        event: &ImageStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            ImageStoreEvent::ImageAdded(image) => {
                cx.subscribe(image, |this, image, event, cx| {
                    this.on_image_event(image, event, cx);
                })
                .detach();
            }
        }
    }

    fn on_dap_store_event(
        &mut self,
        _: Entity<DapStore>,
        event: &DapStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            DapStoreEvent::Notification {
                session_id,
                message,
            } => {
                // Phase 2 multi-tenant: when the notification names a
                // specific session, only the Project that launched
                // that session should toast. A `None` session means
                // host-wide (every Project surfaces it).
                if let Some(session_id) = session_id
                    && !self.owns_dap_session(*session_id)
                {
                    return;
                }
                cx.emit(Event::Toast {
                    notification_id: "dap".into(),
                    message: message.clone(),
                    link: None,
                });
            }
            DapStoreEvent::LogToDebugConsole {
                session_id,
                message,
            } => {
                // Phase 2 multi-tenant: the shared `DapStore` emits
                // log messages for every session on the host. Only
                // forward to our collab peer when the session
                // belongs to this Project, otherwise we leak sibling
                // Projects' debug output into our share.
                if !self.owns_dap_session(SessionId::from_proto(*session_id)) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::LogToDebugConsole {
                            project_id: *remote_id,
                            session_id: *session_id,
                            message: message.clone(),
                        })
                        .log_err();
                }
            }
            DapStoreEvent::DebugClientShutdown(session_id) => {
                // Drop the session from our per-project set; the host
                // store removes it from its own map separately. Doing
                // this on the shutdown event (rather than e.g. an
                // `observe_release`) gives us deterministic cleanup
                // synchronous with the broadcast.
                self.dap_sessions.remove(session_id);
            }
            _ => {}
        }
    }

    fn on_git_store_event(
        &mut self,
        git_store: Entity<GitStore>,
        event: &GitStoreEvent,
        cx: &mut Context<Self>,
    ) {
        // First, ownership tracking. These run regardless of whether
        // we're collab-shared.
        match event {
            GitStoreEvent::RepositoryAdded(id) => {
                if self.repository_belongs_to_us(*id, &git_store, cx) {
                    let inserted = self.repositories.insert(*id);
                    if inserted && self.active_repository_id.is_none() {
                        self.set_active_repository_id(Some(*id), cx);
                    }
                }
            }
            GitStoreEvent::RepositoryRemoved(id) => {
                self.repositories.remove(id);
                if self.active_repository_id == Some(*id) {
                    let fallback = self.next_active_repository_id(cx);
                    self.set_active_repository_id(fallback, cx);
                }
            }
            _ => {}
        }

        // Downstream-broadcast paths only run when we're host-sharing.
        let ProjectClientState::Shared { remote_id } = self.client_state else {
            return;
        };
        match event {
            GitStoreEvent::RepositorySnapshotForDownstream(snapshot) => {
                if !self.repositories.contains(&snapshot.id) {
                    return;
                }
                let update =
                    if let Some(old) = self.git_repository_snapshots_for_peer.get(&snapshot.id) {
                        snapshot.build_update(old, remote_id)
                    } else {
                        snapshot.initial_update(remote_id)
                    };
                for chunk in proto::split_repository_update(update) {
                    self.collab_client.send(chunk).log_err();
                }
                self.git_repository_snapshots_for_peer
                    .insert(snapshot.id, snapshot.clone());
            }
            GitStoreEvent::RepositorySnapshotRemovedForDownstream(id) => {
                if !self.git_repository_snapshots_for_peer.contains_key(id) {
                    // Either we never owned this repo or already cleaned
                    // up; in either case skip the downstream send to
                    // avoid duplicate broadcasts in Phase 2 sharing.
                    return;
                }
                self.collab_client
                    .send(proto::RemoveRepository {
                        project_id: remote_id,
                        id: id.to_proto(),
                    })
                    .log_err();
                self.git_repository_snapshots_for_peer.remove(id);
            }
            GitStoreEvent::ForwardRepositoryUpdate(update) => {
                let id = RepositoryId::from_proto(update.id);
                if !self.repositories.contains(&id) {
                    return;
                }
                let mut update = update.clone();
                update.project_id = remote_id;
                self.collab_client.send(update).log_err();
            }
            GitStoreEvent::ForwardRepositoryRemove(update) => {
                let id = RepositoryId::from_proto(update.id);
                if !self.repositories.contains(&id) {
                    return;
                }
                let mut update = update.clone();
                update.project_id = remote_id;
                self.collab_client.send(update).log_err();
            }
            GitStoreEvent::DiffBasesUpdatedForDownstream(update) => {
                // Diff-base updates are buffer-keyed; gate on this
                // Project owning the buffer.
                if let Ok(buffer_id) = BufferId::new(update.buffer_id) {
                    if !self.buffers.contains(&buffer_id) {
                        return;
                    }
                }
                let mut update = update.clone();
                update.project_id = remote_id;
                self.collab_client.send(update).log_err();
            }
            _ => {}
        }
    }

    /// Returns true if a language server with the given (optional)
    /// worktree association belongs to this Project. Servers without
    /// a worktree are claimed unconditionally (they're "global" within
    /// the host LspStore); worktree-scoped servers are claimed only if
    /// their worktree is in `self.worktrees`. Mirrors the filter
    /// already used in `Project::shared` when announcing servers to a
    /// joining peer.
    fn language_server_belongs_to_us(&self, worktree_id: Option<WorktreeId>, cx: &App) -> bool {
        match worktree_id {
            None => true,
            Some(id) => self.worktrees(cx).any(|w| w.read(cx).id() == id),
        }
    }

    /// Returns true if any of the given repository's worktrees are owned
    /// by this Project. Used by `on_git_store_event` to decide whether
    /// to claim a repository when the host store fires `RepositoryAdded`.
    ///
    /// When the host store has no worktree associations recorded for
    /// the repository (currently the case for remote/peer-driven
    /// repositories — `handle_update_repository` doesn't populate
    /// `worktree_ids`), defaults to claiming. Remote projects are
    /// effectively single-tenant (one Project per remote endpoint), so
    /// this preserves Phase 1 behavior. Phase 2 sharing on remote
    /// endpoints would need to refine this rule.
    fn repository_belongs_to_us(
        &self,
        repository_id: RepositoryId,
        git_store: &Entity<GitStore>,
        cx: &App,
    ) -> bool {
        let Some(worktree_ids) = git_store
            .read(cx)
            .worktree_ids_for_repository(repository_id)
        else {
            return true;
        };
        let our_worktree_ids: HashSet<WorktreeId> =
            self.worktrees(cx).map(|w| w.read(cx).id()).collect();
        worktree_ids.iter().any(|id| our_worktree_ids.contains(id))
    }

    fn on_breakpoint_store_event(
        &mut self,
        breakpoint_store: Entity<BreakpointStore>,
        event: &BreakpointStoreEvent,
        cx: &mut Context<Self>,
    ) {
        // Mirror the previous in-store broadcast: only broadcast on Toggled,
        // and only when this project is the host of a collab share running
        // a local breakpoint store (not via remote-server).
        if let BreakpointStoreEvent::BreakpointsUpdated(path, BreakpointUpdatedReason::Toggled) =
            event
        {
            if let ProjectClientState::Shared { remote_id } = &self.client_state {
                if self.host.read(cx).remote_client.is_none() {
                    let proto = breakpoint_store
                        .read(cx)
                        .breakpoints_for_file_proto(path, *remote_id);
                    let _ = self.collab_client.send(proto);
                }
            }
        }
    }

    fn on_lsp_store_event(
        &mut self,
        _: Entity<LspStore>,
        event: &LspStoreEvent,
        cx: &mut Context<Self>,
    ) {
        // Ownership tracking. Run regardless of share state.
        match event {
            LspStoreEvent::LanguageServerAdded(server_id, _, worktree_id) => {
                if self.language_server_belongs_to_us(*worktree_id, cx) {
                    self.language_servers.insert(*server_id);
                }
            }
            LspStoreEvent::LanguageServerRemoved(server_id) => {
                self.language_servers.remove(server_id);
            }
            _ => {}
        }

        match event {
            LspStoreEvent::DiagnosticsUpdated { server_id, paths } => {
                cx.emit(Event::DiagnosticsUpdated {
                    paths: paths.clone(),
                    language_server_id: *server_id,
                })
            }
            LspStoreEvent::LanguageServerAdded(server_id, name, worktree_id) => {
                cx.emit(Event::LanguageServerAdded(
                    *server_id,
                    name.clone(),
                    *worktree_id,
                ));
                if !self.language_servers.contains(server_id) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    let lsp_store = self.lsp_store(cx).read(cx);
                    if let Some(capabilities) = lsp_store.lsp_server_capabilities.get(server_id) {
                        self.collab_client
                            .send(proto::StartLanguageServer {
                                project_id: *remote_id,
                                server: Some(proto::LanguageServer {
                                    id: server_id.to_proto(),
                                    name: name.to_string(),
                                    worktree_id: worktree_id.map(|id| id.to_proto()),
                                }),
                                capabilities: serde_json::to_string(capabilities)
                                    .expect("serializing server LSP capabilities"),
                            })
                            .log_err();
                    }
                }
            }
            LspStoreEvent::LanguageServerRemoved(server_id) => {
                cx.emit(Event::LanguageServerRemoved(*server_id))
            }
            LspStoreEvent::LanguageServerLog(server_id, log_type, string) => cx.emit(
                Event::LanguageServerLog(*server_id, log_type.clone(), string.clone()),
            ),
            LspStoreEvent::LanguageDetected {
                buffer,
                new_language,
            } => {
                let Some(_) = new_language else {
                    cx.emit(Event::LanguageNotFound(buffer.clone()));
                    return;
                };
            }
            LspStoreEvent::RefreshInlayHints {
                server_id,
                request_id,
            } => {
                cx.emit(Event::RefreshInlayHints {
                    server_id: *server_id,
                    request_id: *request_id,
                });
                if !self.language_servers.contains(server_id) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::RefreshInlayHints {
                            project_id: *remote_id,
                            server_id: server_id.to_proto(),
                            request_id: request_id.map(|id| id as u64),
                        })
                        .log_err();
                }
            }
            LspStoreEvent::RefreshSemanticTokens {
                server_id,
                request_id,
            } => {
                cx.emit(Event::RefreshSemanticTokens {
                    server_id: *server_id,
                    request_id: *request_id,
                });
                if !self.language_servers.contains(server_id) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::RefreshSemanticTokens {
                            project_id: *remote_id,
                            server_id: server_id.to_proto(),
                            request_id: request_id.map(|id| id as u64),
                        })
                        .log_err();
                }
            }
            LspStoreEvent::RefreshCodeLens => {
                cx.emit(Event::RefreshCodeLens);
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::RefreshCodeLens {
                            project_id: *remote_id,
                        })
                        .log_err();
                }
            }
            LspStoreEvent::PullWorkspaceDiagnosticsRequested { server_id } => {
                if !self.language_servers.contains(server_id) {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::PullWorkspaceDiagnostics {
                            project_id: *remote_id,
                            server_id: server_id.to_proto(),
                        })
                        .log_err();
                }
            }
            LspStoreEvent::DiagnosticsSummariesUpdated {
                worktree_id,
                summary,
                more_summaries,
            } => {
                // Gate on this Project owning the worktree the
                // diagnostics belong to. In Phase 2 sharing, sibling
                // Projects on the same host LspStore would otherwise
                // each fire the same downstream broadcast.
                let owns_worktree = self.worktrees(cx).any(|w| w.read(cx).id() == *worktree_id);
                if !owns_worktree {
                    return;
                }
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::UpdateDiagnosticSummary {
                            project_id: *remote_id,
                            worktree_id: worktree_id.to_proto(),
                            summary: Some(summary.clone()),
                            more_summaries: more_summaries.clone(),
                        })
                        .log_err();
                }
            }
            LspStoreEvent::LanguageServerPrompt(prompt) => {
                cx.emit(Event::LanguageServerPrompt(prompt.clone()))
            }
            LspStoreEvent::DiskBasedDiagnosticsStarted { language_server_id } => {
                cx.emit(Event::DiskBasedDiagnosticsStarted {
                    language_server_id: *language_server_id,
                });
            }
            LspStoreEvent::DiskBasedDiagnosticsFinished { language_server_id } => {
                cx.emit(Event::DiskBasedDiagnosticsFinished {
                    language_server_id: *language_server_id,
                });
            }
            LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message,
            } => {
                if self.is_local(cx) && self.language_servers.contains(language_server_id) {
                    self.enqueue_buffer_ordered_message(
                        BufferOrderedMessage::LanguageServerUpdate {
                            language_server_id: *language_server_id,
                            message: message.clone(),
                            name: name.clone(),
                        },
                    )
                    .ok();
                }

                match message {
                    proto::update_language_server::Variant::MetadataUpdated(update) => {
                        self.lsp_store(cx).update(cx, |lsp_store, _| {
                            if let Some(capabilities) = update
                                .capabilities
                                .as_ref()
                                .and_then(|capabilities| serde_json::from_str(capabilities).ok())
                            {
                                lsp_store
                                    .lsp_server_capabilities
                                    .insert(*language_server_id, capabilities);
                            }

                            if let Some(language_server_status) = lsp_store
                                .language_server_statuses
                                .get_mut(language_server_id)
                            {
                                if let Some(binary) = &update.binary {
                                    language_server_status.binary = Some(LanguageServerBinary {
                                        path: PathBuf::from(&binary.path),
                                        arguments: binary
                                            .arguments
                                            .iter()
                                            .map(OsString::from)
                                            .collect(),
                                        env: None,
                                    });
                                }

                                language_server_status.configuration = update
                                    .configuration
                                    .as_ref()
                                    .and_then(|config_str| serde_json::from_str(config_str).ok());

                                language_server_status.workspace_folders = update
                                    .workspace_folders
                                    .iter()
                                    .filter_map(|uri_str| lsp::Uri::from_str(uri_str).ok())
                                    .collect();
                            }
                        });
                    }
                    proto::update_language_server::Variant::RegisteredForBuffer(update) => {
                        if let Some(buffer_id) = BufferId::new(update.buffer_id).ok() {
                            cx.emit(Event::LanguageServerBufferRegistered {
                                buffer_id,
                                server_id: *language_server_id,
                                buffer_abs_path: PathBuf::from(&update.buffer_abs_path),
                                name: name.clone(),
                            });
                        }
                    }
                    _ => (),
                }
            }
            LspStoreEvent::Notification(message) => cx.emit(Event::Toast {
                notification_id: "lsp".into(),
                message: message.clone(),
                link: None,
            }),
            LspStoreEvent::SnippetEdit {
                buffer_id,
                edits,
                most_recent_edit,
            } => {
                if most_recent_edit.replica_id == self.replica_id(cx) {
                    cx.emit(Event::SnippetEdit(*buffer_id, edits.clone()))
                }
            }
            LspStoreEvent::WorkspaceEditApplied(transaction) => {
                cx.emit(Event::WorkspaceEditApplied(transaction.clone()))
            }
            LspStoreEvent::ApplyWorkspaceEditRequested {
                server_id,
                params,
                response,
            } => {
                let lsp_store = self.lsp_store(cx).downgrade();
                let active_entry = self.active_entry;
                let server_id = *server_id;
                let params = params.clone();
                let response = response.clone();
                cx.spawn(async move |_, cx| {
                    let result = lsp_store::LocalLspStore::on_lsp_workspace_edit(
                        lsp_store,
                        params,
                        server_id,
                        active_entry,
                        cx,
                    )
                    .await;
                    response.send(result).await.ok();
                })
                .detach();
            }
        }
    }

    fn on_remote_client_event(
        &mut self,
        _: Entity<RemoteClient>,
        event: &remote::RemoteClientEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            &remote::RemoteClientEvent::Disconnected { server_not_running } => {
                self.worktree_store(cx).update(cx, |store, cx| {
                    store.disconnected_from_host(cx);
                });
                self.buffer_store(cx).update(cx, |buffer_store, cx| {
                    buffer_store.disconnected_from_host(cx)
                });
                self.lsp_store(cx).update(cx, |lsp_store, _cx| {
                    lsp_store.disconnected_from_ssh_remote()
                });
                cx.emit(Event::DisconnectedFromRemote { server_not_running });
            }
        }
    }

    fn on_settings_observer_event(
        &mut self,
        _: Entity<SettingsObserver>,
        event: &SettingsObserverEvent,
        cx: &mut Context<Self>,
    ) {
        if let SettingsObserverEvent::LocalSettingsApplied {
            worktree_id,
            path,
            kind,
            content,
        } = event
        {
            if self.owns_worktree_id(*worktree_id, cx) {
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.collab_client
                        .send(proto::UpdateWorktreeSettings {
                            project_id: *remote_id,
                            worktree_id: worktree_id.to_proto(),
                            path: path.to_proto(),
                            content: content.clone(),
                            kind: Some(
                                project_settings::local_settings_kind_to_proto(*kind).into(),
                            ),
                            outside_worktree: Some(path.is_outside_worktree()),
                        })
                        .log_err();
                }
            }
        }
        match event {
            SettingsObserverEvent::LocalSettingsUpdated(result) => match result {
                Err(InvalidSettingsError::LocalSettings {
                    worktree_id,
                    message,
                    path,
                }) => {
                    // Phase 2 multi-tenant: scope the toast to the
                    // Project that owns the failing worktree so a
                    // sibling workspace's parse error doesn't pop a
                    // notification in ours.
                    if !self.owns_worktree_id(*worktree_id, cx) {
                        return;
                    }
                    let message = format!("Failed to set local settings in {path:?}:\n{message}");
                    cx.emit(Event::Toast {
                        notification_id: format!("local-settings-{path:?}").into(),
                        link: None,
                        message,
                    });
                }
                Ok(path) => {
                    if !self.toast_path_visible_to_us(path, cx) {
                        return;
                    }
                    cx.emit(Event::HideToast {
                        notification_id: format!("local-settings-{path:?}").into(),
                    });
                }
                Err(_) => {}
            },
            SettingsObserverEvent::LocalTasksUpdated(result) => match result {
                Err(InvalidSettingsError::Tasks { message, path }) => {
                    if !self.toast_path_visible_to_us(path, cx) {
                        return;
                    }
                    let message = format!("Failed to set local tasks in {path:?}:\n{message}");
                    cx.emit(Event::Toast {
                        notification_id: format!("local-tasks-{path:?}").into(),
                        link: Some(ToastLink {
                            label: "Open Tasks Documentation",
                            url: "https://zed.dev/docs/tasks",
                        }),
                        message,
                    });
                }
                Ok(path) => {
                    if !self.toast_path_visible_to_us(path, cx) {
                        return;
                    }
                    cx.emit(Event::HideToast {
                        notification_id: format!("local-tasks-{path:?}").into(),
                    });
                }
                Err(_) => {}
            },
            SettingsObserverEvent::LocalDebugScenariosUpdated(result) => match result {
                Err(InvalidSettingsError::Debug { message, path }) => {
                    if !self.toast_path_visible_to_us(path, cx) {
                        return;
                    }
                    let message =
                        format!("Failed to set local debug scenarios in {path:?}:\n{message}");
                    cx.emit(Event::Toast {
                        notification_id: format!("local-debug-scenarios-{path:?}").into(),
                        link: None,
                        message,
                    });
                }
                Ok(path) => {
                    if !self.toast_path_visible_to_us(path, cx) {
                        return;
                    }
                    cx.emit(Event::HideToast {
                        notification_id: format!("local-debug-scenarios-{path:?}").into(),
                    });
                }
                Err(_) => {}
            },
            SettingsObserverEvent::LocalSettingsApplied { .. } => {}
        }
    }

    fn toast_path_visible_to_us(&self, path: &Path, cx: &App) -> bool {
        if self.owns_abs_path(path, cx) {
            return true;
        }
        let worktree_store = self.worktree_store(cx);
        worktree_store.read(cx).find_worktree(path, cx).is_none()
    }

    fn on_worktree_store_event(
        &mut self,
        _: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(worktree) => {
                // In Phase 2 sharing the same `WorktreeStore` serves
                // multiple `Project`s. Ownership rules:
                //
                // - VISIBLE worktrees correspond to user-opened
                //   workspaces. Each is owned by exactly one Project
                //   (the one whose `find_or_create_worktree` /
                //   `create_worktree` / `add_worktree` initiated it).
                //   The initiating call site claims explicitly; this
                //   handler does NOT auto-claim visible worktrees, so a
                //   sibling Project opening its own visible worktree
                //   doesn't leak into ours.
                //
                // - INVISIBLE worktrees are typically created by host
                //   stores (e.g. `LspStore::open_local_buffer_via_lsp`
                //   when a Go-to-Definition crosses into an external
                //   file). These don't have a clean "initiator Project"
                //   because the host store doesn't know which Project's
                //   request triggered them. Auto-claim them so they
                //   appear in the Project's view; in shared-Host
                //   multi-tenant scenarios every Project will claim,
                //   which matches the "visible to all" semantics that
                //   already applied in Phase 1.
                let already_owned = self.worktrees.iter().any(|handle| {
                    handle
                        .upgrade()
                        .is_some_and(|w| w.entity_id() == worktree.entity_id())
                });
                if !already_owned {
                    let snapshot = worktree.read(cx);
                    let is_visible = snapshot.is_visible();
                    let abs_path = snapshot.abs_path();
                    let was_pending = self.pending_worktree_paths.contains(&abs_path);
                    if is_visible && !was_pending {
                        // A sibling Project on a shared `WorktreeStore`
                        // added its own visible worktree; don't claim.
                        return;
                    }
                    let push_strong = self.retain_worktrees || is_visible;
                    let handle = if push_strong {
                        WorktreeHandle::Strong(worktree.clone())
                    } else {
                        WorktreeHandle::Weak(worktree.downgrade())
                    };
                    self.worktrees.push(handle);
                }
                self.on_worktree_added(worktree, cx);
                cx.emit(Event::WorktreeAdded(worktree.read(cx).id()));
                self.emit_group_key_changed_if_needed(cx);
            }
            WorktreeStoreEvent::WorktreeMetadataChanged => {
                // When shared, broadcast the new project metadata downstream
                // and re-set up observers (which also covers any newly added
                // worktree). Mirrors the previous behavior: if the metadata
                // send fails (e.g. transient disconnect), observers are not
                // set up, matching the old `send_project_updates` flow.
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    self.send_worktree_project_updates(*remote_id, cx);
                }
            }
            WorktreeStoreEvent::WorktreeRemoved(entity_id, id) => {
                self.worktrees.retain(|handle| match handle {
                    WorktreeHandle::Strong(w) => w.entity_id() != *entity_id,
                    WorktreeHandle::Weak(w) => {
                        w.upgrade().is_some_and(|w| w.entity_id() != *entity_id)
                    }
                });
                cx.emit(Event::WorktreeRemoved(*id));
                self.emit_group_key_changed_if_needed(cx);
            }
            WorktreeStoreEvent::WorktreeReleased(_, id) => {
                self.on_worktree_released(*id, cx);
            }

            WorktreeStoreEvent::WorktreeUpdateSent(worktree) => {
                if let ProjectClientState::Shared { remote_id } = &self.client_state {
                    let summaries = self
                        .lsp_store(cx)
                        .read(cx)
                        .diagnostic_summaries_for_worktree(worktree.read(cx).id(), *remote_id);
                    if let Some(summaries) = summaries {
                        self.collab_client.send(summaries).log_err();
                    }
                }
            }
            WorktreeStoreEvent::WorktreeUpdatedEntries(worktree_id, changes) => {
                self.client()
                    .telemetry()
                    .report_discovered_project_type_events(*worktree_id, changes);
                cx.emit(Event::WorktreeUpdatedEntries(*worktree_id, changes.clone()))
            }
            WorktreeStoreEvent::WorktreeDeletedEntry(worktree_id, id) => {
                cx.emit(Event::DeletedEntry(*worktree_id, *id))
            }
            // Listen to the GitStore instead.
            WorktreeStoreEvent::WorktreeUpdatedGitRepositories(_, _) => {}
            WorktreeStoreEvent::WorktreeUpdatedRootRepoCommonDir(worktree_id) => {
                cx.emit(Event::WorktreeUpdatedRootRepoCommonDir(*worktree_id));
                self.emit_group_key_changed_if_needed(cx);
            }
        }
    }

    fn on_worktree_added(&mut self, worktree: &Entity<Worktree>, _: &mut Context<Self>) {
        let mut remotely_created_models = self.remotely_created_models.lock();
        if remotely_created_models.retain_count > 0 {
            remotely_created_models.worktrees.push(worktree.clone())
        }
    }

    fn on_worktree_released(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
        if let Some(remote) = &self.host.read(cx).remote_client {
            remote
                .read(cx)
                .proto_client()
                .send(proto::RemoveWorktree {
                    worktree_id: id_to_remove.to_proto(),
                })
                .log_err();
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        if matches!(event, BufferEvent::Edited { .. } | BufferEvent::Reloaded) {
            self.request_buffer_diff_recalculation(&buffer, cx);
        }

        if matches!(event, BufferEvent::Edited { .. }) {
            cx.emit(Event::BufferEdited);
        }

        let buffer_id = buffer.read(cx).remote_id();
        match event {
            BufferEvent::ReloadNeeded => {
                if !self.is_via_collab() {
                    self.reload_buffers([buffer.clone()].into_iter().collect(), true, cx)
                        .detach_and_log_err(cx);
                }
            }
            BufferEvent::Operation {
                operation,
                is_local: true,
            } => {
                let operation = language::proto::serialize_operation(operation);

                if let Some(remote) = &self.host.read(cx).remote_client {
                    remote
                        .read(cx)
                        .proto_client()
                        .send(proto::UpdateBuffer {
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

            _ => {}
        }

        None
    }

    fn on_image_event(
        &mut self,
        image: Entity<ImageItem>,
        event: &ImageItemEvent,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        // TODO: handle image events from remote
        if let ImageItemEvent::ReloadNeeded = event
            && !self.is_via_collab()
        {
            self.reload_images([image].into_iter().collect(), cx)
                .detach_and_log_err(cx);
        }

        None
    }

    fn request_buffer_diff_recalculation(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        self.buffers_needing_diff.insert(buffer.downgrade());
        let first_insertion = self.buffers_needing_diff.len() == 1;
        let settings = ProjectSettings::get_global(cx);
        let delay = settings.git.gutter_debounce;

        if delay == 0 {
            if first_insertion {
                let this = cx.weak_entity();
                cx.defer(move |cx| {
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            this.recalculate_buffer_diffs(cx).detach();
                        });
                    }
                });
            }
            return;
        }

        const MIN_DELAY: u64 = 50;
        let delay = delay.max(MIN_DELAY);
        let duration = Duration::from_millis(delay);

        self.git_diff_debouncer
            .fire_new(duration, cx, move |this, cx| {
                this.recalculate_buffer_diffs(cx)
            });
    }

    fn recalculate_buffer_diffs(&mut self, cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async move |this, cx| {
            loop {
                let task = this
                    .update(cx, |this, cx| {
                        let buffers = this
                            .buffers_needing_diff
                            .drain()
                            .filter_map(|buffer| buffer.upgrade())
                            .collect::<Vec<_>>();
                        if buffers.is_empty() {
                            None
                        } else {
                            Some(this.git_store(cx).update(cx, |git_store, cx| {
                                git_store.recalculate_buffer_diffs(buffers, cx)
                            }))
                        }
                    })
                    .ok()
                    .flatten();

                if let Some(task) = task {
                    task.await;
                } else {
                    break;
                }
            }
        })
    }

    pub fn set_language_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        new_language: Arc<Language>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.set_language_for_buffer(buffer, new_language, cx)
        })
    }

    pub fn restart_language_servers_for_buffers(
        &mut self,
        buffers: Vec<Entity<Buffer>>,
        only_restart_servers: HashSet<LanguageServerSelector>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.restart_language_servers_for_buffers(buffers, only_restart_servers, cx)
        })
    }

    pub fn stop_language_servers_for_buffers(
        &mut self,
        buffers: Vec<Entity<Buffer>>,
        also_restart_servers: HashSet<LanguageServerSelector>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store(cx)
            .update(cx, |lsp_store, cx| {
                lsp_store.stop_language_servers_for_buffers(buffers, also_restart_servers, cx)
            })
            .detach_and_log_err(cx);
    }

    pub fn cancel_language_server_work_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.cancel_language_server_work_for_buffers(buffers, cx)
        })
    }

    pub fn cancel_language_server_work(
        &mut self,
        server_id: LanguageServerId,
        token_to_cancel: Option<ProgressToken>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.cancel_language_server_work(server_id, token_to_cancel, cx)
        })
    }

    fn enqueue_buffer_ordered_message(&mut self, message: BufferOrderedMessage) -> Result<()> {
        self.buffer_ordered_messages_tx
            .unbounded_send(message)
            .map_err(|e| anyhow!(e))
    }

    pub fn available_toolchains(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<Toolchains>> {
        if let Some(toolchain_store) = self
            .host
            .read(cx)
            .toolchain_store
            .as_ref()
            .map(Entity::downgrade)
        {
            cx.spawn(async move |cx| {
                toolchain_store
                    .update(cx, |this, cx| this.list_toolchains(path, language_name, cx))
                    .ok()?
                    .await
            })
        } else {
            Task::ready(None)
        }
    }

    pub async fn toolchain_metadata(
        languages: Arc<LanguageRegistry>,
        language_name: LanguageName,
    ) -> Option<ToolchainMetadata> {
        languages
            .language_for_name(language_name.as_ref())
            .await
            .ok()?
            .toolchain_lister()
            .map(|lister| lister.meta())
    }

    pub fn add_toolchain(
        &self,
        toolchain: Toolchain,
        scope: ToolchainScope,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let toolchain_store = self.host.read(cx).toolchain_store.clone()?;
            toolchain_store.update(cx, |this, cx| {
                this.add_toolchain(toolchain, scope, cx);
            });
            Some(())
        });
    }

    pub fn remove_toolchain(
        &self,
        toolchain: Toolchain,
        scope: ToolchainScope,
        cx: &mut Context<Self>,
    ) {
        maybe!({
            let toolchain_store = self.host.read(cx).toolchain_store.clone()?;
            toolchain_store.update(cx, |this, cx| {
                this.remove_toolchain(toolchain, scope, cx);
            });
            Some(())
        });
    }

    pub fn user_toolchains(
        &self,
        cx: &App,
    ) -> Option<BTreeMap<ToolchainScope, IndexSet<Toolchain>>> {
        Some(
            self.host
                .read(cx)
                .toolchain_store
                .as_ref()?
                .read(cx)
                .user_toolchains(),
        )
    }

    pub fn resolve_toolchain(
        &self,
        path: PathBuf,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Result<Toolchain>> {
        if let Some(toolchain_store) = self
            .host
            .read(cx)
            .toolchain_store
            .as_ref()
            .map(Entity::downgrade)
        {
            cx.spawn(async move |cx| {
                toolchain_store
                    .update(cx, |this, cx| {
                        this.resolve_toolchain(path, language_name, cx)
                    })?
                    .await
            })
        } else {
            Task::ready(Err(anyhow!("This project does not support toolchains")))
        }
    }

    pub fn toolchain_store(&self, cx: &App) -> Option<Entity<ToolchainStore>> {
        self.host.read(cx).toolchain_store.clone()
    }
    pub fn activate_toolchain(
        &self,
        path: ProjectPath,
        toolchain: Toolchain,
        cx: &mut App,
    ) -> Task<Option<()>> {
        let Some(toolchain_store) = self.host.read(cx).toolchain_store.clone() else {
            return Task::ready(None);
        };
        toolchain_store.update(cx, |this, cx| this.activate_toolchain(path, toolchain, cx))
    }
    pub fn active_toolchain(
        &self,
        path: ProjectPath,
        language_name: LanguageName,
        cx: &App,
    ) -> Task<Option<Toolchain>> {
        let Some(toolchain_store) = self.host.read(cx).toolchain_store.clone() else {
            return Task::ready(None);
        };
        toolchain_store
            .read(cx)
            .active_toolchain(path, language_name, cx)
    }
    pub fn language_server_statuses<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl DoubleEndedIterator<Item = (LanguageServerId, &'a LanguageServerStatus)> {
        self.lsp_store(cx).read(cx).language_server_statuses()
    }

    pub fn last_formatting_failure<'a>(&self, cx: &'a App) -> Option<&'a str> {
        self.lsp_store(cx).read(cx).last_formatting_failure()
    }

    pub fn reset_last_formatting_failure(&self, cx: &mut App) {
        self.lsp_store(cx)
            .update(cx, |store, _| store.reset_last_formatting_failure());
    }

    pub fn reload_buffers(
        &self,
        buffers: HashSet<Entity<Buffer>>,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        self.buffer_store(cx).update(cx, |buffer_store, cx| {
            buffer_store.reload_buffers(buffers, push_to_history, cx)
        })
    }

    pub fn reload_images(
        &self,
        images: HashSet<Entity<ImageItem>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let image_store = self.host.read(cx).image_store.clone();
        image_store.update(cx, |image_store, cx| image_store.reload_images(images, cx))
    }

    pub fn owns_abs_path(&self, abs_path: &Path, cx: &App) -> bool {
        self.worktrees(cx)
            .any(|worktree| abs_path.starts_with(worktree.read(cx).abs_path().as_ref()))
    }

    pub fn owns_worktree_id(&self, worktree_id: WorktreeId, cx: &App) -> bool {
        self.worktrees(cx)
            .any(|worktree| worktree.read(cx).id() == worktree_id)
    }

    pub fn images(&self, cx: &App) -> Vec<Entity<ImageItem>> {
        self.image_store(cx)
            .read(cx)
            .images()
            .filter(|image| self.owns_worktree_id(image.read(cx).file.worktree_id(cx), cx))
            .collect()
    }

    pub async fn all_bookmark_locations(
        project: Entity<Self>,
        cx: &mut AsyncApp,
    ) -> Result<std::collections::HashMap<Entity<Buffer>, Vec<Range<Point>>>> {
        let bookmark_store = project.read_with(cx, |project, cx| project.bookmark_store(cx));
        let locations =
            bookmark_store::BookmarkStore::all_bookmark_locations(bookmark_store, cx).await?;
        let filtered = project.read_with(cx, |project, cx| {
            locations
                .into_iter()
                .filter(|(buffer, _)| match buffer.read(cx).file() {
                    Some(file) => project.owns_worktree_id(file.worktree_id(cx), cx),
                    None => false,
                })
                .collect()
        });
        Ok(filtered)
    }

    pub fn serialized_bookmarks(&self, cx: &App) -> BTreeMap<Arc<Path>, Vec<SerializedBookmark>> {
        self.bookmark_store(cx)
            .read(cx)
            .all_serialized_bookmarks(cx)
            .into_iter()
            .filter(|(path, _)| self.owns_abs_path(path, cx))
            .collect()
    }

    pub fn restore_serialized_bookmarks(
        &self,
        bookmarks: BTreeMap<Arc<Path>, Vec<SerializedBookmark>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let bookmark_store = self.bookmark_store(cx);
        let owned_paths: Vec<Arc<Path>> = bookmark_store
            .read(cx)
            .all_serialized_bookmarks(cx)
            .into_keys()
            .filter(|path| self.owns_abs_path(path, cx))
            .collect();
        bookmark_store.update(cx, |store, cx| {
            store.clear_bookmarks_for_paths(&owned_paths, cx);
            store.load_serialized_bookmarks(bookmarks, cx)
        })
    }

    pub fn serialized_breakpoints(&self, cx: &App) -> BTreeMap<Arc<Path>, Vec<SourceBreakpoint>> {
        self.breakpoint_store(cx)
            .read(cx)
            .all_source_breakpoints(cx)
            .into_iter()
            .filter(|(path, _)| self.owns_abs_path(path, cx))
            .collect()
    }

    pub fn restore_serialized_breakpoints(
        &self,
        breakpoints: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let breakpoint_store = self.breakpoint_store(cx);
        let owned_paths: Vec<Arc<Path>> = breakpoint_store
            .read(cx)
            .all_source_breakpoints(cx)
            .into_keys()
            .filter(|path| self.owns_abs_path(path, cx))
            .collect();
        breakpoint_store.update(cx, |store, cx| {
            store.clear_breakpoints_for_paths(&owned_paths, cx);
        });
        breakpoint_store.update(cx, |store, cx| {
            store.with_serialized_breakpoints(breakpoints, cx)
        })
    }

    pub fn clear_breakpoints(&self, cx: &mut Context<Self>) {
        let breakpoint_store = self.breakpoint_store(cx);
        let owned_paths: Vec<Arc<Path>> = breakpoint_store
            .read(cx)
            .all_source_breakpoints(cx)
            .into_keys()
            .filter(|path| self.owns_abs_path(path, cx))
            .collect();
        breakpoint_store.update(cx, |store, cx| {
            store.clear_breakpoints_for_paths(&owned_paths, cx);
        });
    }

    pub fn format(
        &mut self,
        buffers: HashSet<Entity<Buffer>>,
        target: LspFormatTarget,
        push_to_history: bool,
        trigger: lsp_store::FormatTrigger,
        cx: &mut Context<Project>,
    ) -> Task<anyhow::Result<ProjectTransaction>> {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.format(buffers, target, push_to_history, trigger, cx)
        })
    }

    pub fn supports_range_formatting(&self, buffer: &Entity<Buffer>, cx: &App) -> bool {
        self.lsp_store(cx)
            .read(cx)
            .supports_range_formatting(buffer, cx)
    }

    pub fn definitions<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<LocationLink>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        let guard = self.retain_remotely_created_models(cx);
        let task = self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.definitions(buffer, position, cx)
        });
        cx.background_spawn(async move {
            let result = task.await;
            drop(guard);
            result
        })
    }

    pub fn declarations<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<LocationLink>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        let guard = self.retain_remotely_created_models(cx);
        let task = self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.declarations(buffer, position, cx)
        });
        cx.background_spawn(async move {
            let result = task.await;
            drop(guard);
            result
        })
    }

    pub fn type_definitions<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<LocationLink>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        let guard = self.retain_remotely_created_models(cx);
        let task = self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.type_definitions(buffer, position, cx)
        });
        cx.background_spawn(async move {
            let result = task.await;
            drop(guard);
            result
        })
    }

    pub fn implementations<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<LocationLink>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        let guard = self.retain_remotely_created_models(cx);
        let task = self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.implementations(buffer, position, cx)
        });
        cx.background_spawn(async move {
            let result = task.await;
            drop(guard);
            result
        })
    }

    pub fn references<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<Location>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        let guard = self.retain_remotely_created_models(cx);
        let task = self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.references(buffer, position, cx)
        });
        cx.background_spawn(async move {
            let result = task.await;
            drop(guard);
            result
        })
    }

    pub fn document_highlights<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<DocumentHighlight>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::FirstCapable,
            GetDocumentHighlights { position },
            cx,
        )
    }

    pub fn document_symbols(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<DocumentSymbol>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::FirstCapable,
            GetDocumentSymbols,
            cx,
        )
    }

    pub fn symbols(&self, query: &str, cx: &mut Context<Self>) -> Task<Result<Vec<Symbol>>> {
        self.lsp_store(cx)
            .update(cx, |lsp_store, cx| lsp_store.symbols(query, cx))
    }

    pub fn open_buffer_for_symbol(
        &mut self,
        symbol: &Symbol,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.open_buffer_for_symbol(symbol, cx)
        })
    }

    pub fn open_server_settings(&mut self, cx: &mut Context<Self>) -> Task<Result<Entity<Buffer>>> {
        let guard = self.retain_remotely_created_models(cx);
        let Some(remote) = self.host.read(cx).remote_client.as_ref() else {
            return Task::ready(Err(anyhow!("not an ssh project")));
        };

        let proto_client = remote.read(cx).proto_client();

        cx.spawn(async move |project, cx| {
            let buffer = proto_client
                .request(proto::OpenServerSettings {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                })
                .await?;

            let buffer = project
                .update(cx, |project, cx| {
                    project.buffer_store(cx).update(cx, |buffer_store, cx| {
                        anyhow::Ok(
                            buffer_store
                                .wait_for_remote_buffer(BufferId::new(buffer.buffer_id)?, cx),
                        )
                    })
                })??
                .await;

            drop(guard);
            buffer
        })
    }

    pub fn open_local_buffer_via_lsp(
        &mut self,
        abs_path: lsp::Uri,
        language_server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.open_local_buffer_via_lsp(abs_path, language_server_id, cx)
        })
    }

    pub fn hover<T: ToPointUtf16>(
        &self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Option<Vec<Hover>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.lsp_store(cx)
            .update(cx, |lsp_store, cx| lsp_store.hover(buffer, position, cx))
    }

    pub fn linked_edits(
        &self,
        buffer: &Entity<Buffer>,
        position: Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<Range<Anchor>>>> {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.linked_edits(buffer, position, cx)
        })
    }

    pub fn completions<T: ToOffset + ToPointUtf16>(
        &self,
        buffer: &Entity<Buffer>,
        position: T,
        context: CompletionContext,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.completions(buffer, position, context, cx)
        })
    }

    pub fn code_actions<T: Clone + ToOffset>(
        &mut self,
        buffer_handle: &Entity<Buffer>,
        range: Range<T>,
        kinds: Option<Vec<CodeActionKind>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<CodeAction>>>> {
        let buffer = buffer_handle.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.code_actions(buffer_handle, range, kinds, cx)
        })
    }

    pub fn apply_code_action(
        &self,
        buffer_handle: Entity<Buffer>,
        action: CodeAction,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        let active_entry = self.active_entry;
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.apply_code_action(buffer_handle, action, push_to_history, active_entry, cx)
        })
    }

    pub fn apply_code_action_kind(
        &self,
        buffers: HashSet<Entity<Buffer>>,
        kind: CodeActionKind,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        let active_entry = self.active_entry;
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.apply_code_action_kind(buffers, kind, push_to_history, active_entry, cx)
        })
    }

    pub fn prepare_rename<T: ToPointUtf16>(
        &mut self,
        buffer: Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<PrepareRenameResponse>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(
            buffer,
            LanguageServerToQuery::FirstCapable,
            PrepareRename { position },
            cx,
        )
    }

    pub fn perform_rename<T: ToPointUtf16>(
        &mut self,
        buffer: Entity<Buffer>,
        position: T,
        new_name: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        let push_to_history = true;
        let position = position.to_point_utf16(buffer.read(cx));
        let active_entry = self.active_entry;
        self.request_lsp(
            buffer,
            LanguageServerToQuery::FirstCapable,
            PerformRename {
                position,
                new_name,
                push_to_history,
                active_entry,
            },
            cx,
        )
    }

    pub fn on_type_format<T: ToPointUtf16>(
        &mut self,
        buffer: Entity<Buffer>,
        position: T,
        trigger: String,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.on_type_format(buffer, position, trigger, push_to_history, cx)
        })
    }

    pub fn inline_values(
        &mut self,
        session: Entity<Session>,
        active_stack_frame: ActiveStackFrame,
        buffer_handle: Entity<Buffer>,
        range: Range<text::Anchor>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Vec<InlayHint>>> {
        let snapshot = buffer_handle.read(cx).snapshot();

        let captures =
            snapshot.debug_variables_query(Anchor::min_for_buffer(snapshot.remote_id())..range.end);

        let row = snapshot
            .summary_for_anchor::<text::PointUtf16>(&range.end)
            .row as usize;

        let inline_value_locations = provide_inline_values(captures, &snapshot, row);

        let stack_frame_id = active_stack_frame.stack_frame_id;
        cx.spawn(async move |this, cx| {
            this.update(cx, |project, cx| {
                project.dap_store(cx).update(cx, |dap_store, cx| {
                    dap_store.resolve_inline_value_locations(
                        session,
                        stack_frame_id,
                        buffer_handle,
                        inline_value_locations,
                        cx,
                    )
                })
            })?
            .await
        })
    }

    fn search_impl(&mut self, query: SearchQuery, cx: &mut Context<Self>) -> SearchResultsHandle {
        let client: Option<(AnyProtoClient, _)> =
            if let Some(ssh_client) = &self.host.read(cx).remote_client {
                Some((ssh_client.read(cx).proto_client(), 0))
            } else if let Some(remote_id) = self.remote_id() {
                self.is_local(cx)
                    .not()
                    .then(|| (self.collab_client.clone().into(), remote_id))
            } else {
                None
            };
        let searcher = if query.is_opened_only() {
            project_search::Search::open_buffers_only(
                self.buffer_store(cx),
                self.worktree_store(cx),
                project_search::Search::MAX_SEARCH_RESULT_FILES + 1,
            )
        } else {
            match client {
                Some((client, remote_id)) => project_search::Search::remote(
                    self.buffer_store(cx),
                    self.worktree_store(cx),
                    project_search::Search::MAX_SEARCH_RESULT_FILES + 1,
                    (client, remote_id, cx.weak_entity()),
                ),
                None => project_search::Search::local(
                    self.fs(cx),
                    self.buffer_store(cx),
                    self.worktree_store(cx),
                    project_search::Search::MAX_SEARCH_RESULT_FILES + 1,
                    cx,
                ),
            }
        };
        searcher.into_handle(query, cx)
    }

    pub fn search(
        &mut self,
        query: SearchQuery,
        cx: &mut Context<Self>,
    ) -> SearchResults<SearchResult> {
        self.search_impl(query, cx).results(cx)
    }

    pub fn request_lsp<R: LspCommand>(
        &mut self,
        buffer_handle: Entity<Buffer>,
        server: LanguageServerToQuery,
        request: R,
        cx: &mut Context<Self>,
    ) -> Task<Result<R::Response>>
    where
        <R::LspRequest as lsp::request::Request>::Result: Send,
        <R::LspRequest as lsp::request::Request>::Params: Send,
    {
        let guard = self.retain_remotely_created_models(cx);
        let task = self.lsp_store(cx).update(cx, |lsp_store, cx| {
            lsp_store.request_lsp(buffer_handle, server, request, cx)
        });
        cx.background_spawn(async move {
            let result = task.await;
            drop(guard);
            result
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
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if source == destination {
            return Ok(());
        }

        let mut source_index = None;
        let mut destination_index = None;
        for (i, handle) in self.worktrees.iter().enumerate() {
            if let Some(worktree) = handle.upgrade() {
                let id = worktree.read(cx).id();
                if id == source {
                    source_index = Some(i);
                    if destination_index.is_some() {
                        break;
                    }
                } else if id == destination {
                    destination_index = Some(i);
                    if source_index.is_some() {
                        break;
                    }
                }
            }
        }

        let source_index =
            source_index.with_context(|| format!("Missing worktree for id {source}"))?;
        let destination_index =
            destination_index.with_context(|| format!("Missing worktree for id {destination}"))?;

        if source_index == destination_index {
            return Ok(());
        }

        let handle = self.worktrees.remove(source_index);
        self.worktrees.insert(destination_index, handle);
        cx.emit(Event::WorktreeOrderChanged);
        cx.notify();
        Ok(())
    }

    /// Attempts to convert the input path to a WSL path if this is a wsl remote project and the input path is a host windows path.
    pub fn try_windows_path_to_wsl(
        &self,
        abs_path: &Path,
        cx: &App,
    ) -> impl Future<Output = Result<PathBuf>> + use<> {
        let fut = if cfg!(windows)
            && let (
                ProjectClientState::Local | ProjectClientState::Shared { .. },
                Some(remote_client),
            ) = (&self.client_state, &self.host.read(cx).remote_client)
            && let RemoteConnectionOptions::Wsl(wsl) = remote_client.read(cx).connection_options()
        {
            Either::Left(wsl.abs_windows_path_to_wsl_path(abs_path))
        } else {
            Either::Right(abs_path.to_owned())
        };
        async move {
            match fut {
                Either::Left(fut) => fut.await.map(Into::into),
                Either::Right(path) => Ok(path),
            }
        }
    }

    pub fn find_or_create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Entity<Worktree>, Arc<RelPath>)>> {
        let path_arc: Arc<Path> = Arc::from(abs_path.as_ref());
        // Mark the path as "this Project's pending worktree creation"
        // so the `WorktreeAdded` event handler can claim synchronously
        // when the worktree-creation case fires `WorktreeAdded`.
        self.pending_worktree_paths.insert(path_arc.clone());
        // If the store can satisfy the request immediately from an
        // existing worktree (no `WorktreeAdded` event will fire for
        // us), claim it here. In Phase 2 sharing this is how we pick
        // up a sibling Project's worktree at the same path.
        if let Some((worktree, _)) = self.find_worktree(abs_path.as_ref(), cx) {
            self.pending_worktree_paths.remove(&path_arc);
            self.claim_found_worktree(&worktree, cx);
        }
        self.worktree_store(cx).update(cx, |worktree_store, cx| {
            worktree_store.find_or_create_worktree(abs_path, visible, cx)
        })
    }

    /// Idempotent counterpart to the `WorktreeAdded` event handler:
    /// adds `worktree` to `self.worktrees` and runs the same side
    /// effects (`on_worktree_added`, `Event::WorktreeAdded` emit,
    /// `emit_group_key_changed_if_needed`) iff this Project didn't
    /// already own it. Used by `find_or_create_worktree` when the host
    /// store returns an existing worktree (no `WorktreeAdded` fires in
    /// that case).
    ///
    /// Also back-claims any repositories already registered for the
    /// worktree on the shared `GitStore`. In multi-tenant Phase 2, a
    /// sibling Project may have already triggered repository
    /// registration before this Project claimed the worktree, so the
    /// `RepositoryAdded` event would have fired before our
    /// `self.worktrees` contained the new worktree — we'd otherwise
    /// miss the repo.
    fn claim_found_worktree(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        let already_owned = self.worktrees.iter().any(|handle| {
            handle
                .upgrade()
                .is_some_and(|w| w.entity_id() == worktree.entity_id())
        });
        if already_owned {
            return;
        }
        let push_strong = self.retain_worktrees || worktree.read(cx).is_visible();
        let handle = if push_strong {
            WorktreeHandle::Strong(worktree.clone())
        } else {
            WorktreeHandle::Weak(worktree.downgrade())
        };
        self.worktrees.push(handle);
        self.on_worktree_added(worktree, cx);
        cx.emit(Event::WorktreeAdded(worktree.read(cx).id()));
        self.emit_group_key_changed_if_needed(cx);

        let worktree_id = worktree.read(cx).id();
        let git_store = self.git_store(cx);
        let pre_existing_repos: Vec<RepositoryId> = git_store
            .read(cx)
            .repositories()
            .keys()
            .copied()
            .filter(|repo_id| {
                git_store
                    .read(cx)
                    .worktree_ids_for_repository(*repo_id)
                    .is_some_and(|ids| ids.contains(&worktree_id))
            })
            .collect();
        for repo_id in pre_existing_repos {
            self.repositories.insert(repo_id);
        }
    }

    pub fn find_worktree(
        &self,
        abs_path: &Path,
        cx: &App,
    ) -> Option<(Entity<Worktree>, Arc<RelPath>)> {
        self.worktree_store(cx).read(cx).find_worktree(abs_path, cx)
    }

    /// Streams a buffer's initial state and pending operations to a
    /// collaborator. Tracks the buffer in `shared_buffers[peer_id]` so we
    /// don't double-send if the same buffer is referenced multiple times.
    /// Returns immediately when the project is not collab-shared.
    ///
    /// Moved from `BufferStore` in Phase 1; the per-peer state is
    /// per-project, not per-host-store. Takes `&mut App` so it can be
    /// invoked through the [`PeerBufferAccess`] trait alongside
    /// `HeadlessProject`'s mirror method.
    pub fn create_buffer_for_peer(
        &mut self,
        buffer: &Entity<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let buffer_id = buffer.read(cx).remote_id();
        let shared_buffers = self.shared_buffers.entry(peer_id).or_default();
        if shared_buffers.contains_key(&buffer_id) {
            return Task::ready(Ok(()));
        }
        shared_buffers.insert(
            buffer_id,
            SharedBuffer {
                buffer: buffer.clone(),
                lsp_handle: None,
            },
        );

        let ProjectClientState::Shared { remote_id } = self.client_state else {
            return Task::ready(Ok(()));
        };
        let project_id = remote_id;
        let client: AnyProtoClient = self.collab_client.clone().into();
        let buffer = buffer.clone();

        cx.spawn(async move |cx| {
            let operations = buffer.update(cx, |b, cx| b.serialize_ops(None, cx));
            let operations = operations.await;
            let state = buffer.update(cx, |buffer, cx| buffer.to_proto(cx));

            let initial_state = proto::CreateBufferForPeer {
                project_id,
                peer_id: Some(peer_id),
                variant: Some(proto::create_buffer_for_peer::Variant::State(state)),
            };

            if client.send(initial_state).log_err().is_some() {
                let client = client.clone();
                cx.background_spawn(async move {
                    let mut chunks = split_operations(operations).peekable();
                    while let Some(chunk) = chunks.next() {
                        let is_last = chunks.peek().is_none();
                        client.send(proto::CreateBufferForPeer {
                            project_id,
                            peer_id: Some(peer_id),
                            variant: Some(proto::create_buffer_for_peer::Variant::Chunk(
                                proto::BufferChunk {
                                    buffer_id: buffer_id.into(),
                                    operations: chunk,
                                    is_last,
                                },
                            )),
                        })?;
                    }
                    anyhow::Ok(())
                })
                .await
                .log_err();
            }
            Ok(())
        })
    }

    /// Serializes a `ProjectTransaction` for sending to a collaborator,
    /// also creating the buffers on that peer via `create_buffer_for_peer`.
    /// Moved from `BufferStore` alongside `create_buffer_for_peer`. Takes
    /// `&mut App` so it can be invoked through the [`PeerBufferAccess`]
    /// trait.
    pub fn serialize_project_transaction_for_peer(
        &mut self,
        project_transaction: ProjectTransaction,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> proto::ProjectTransaction {
        let mut serialized_transaction = proto::ProjectTransaction {
            buffer_ids: Default::default(),
            transactions: Default::default(),
        };
        for (buffer, transaction) in project_transaction.0 {
            self.create_buffer_for_peer(&buffer, peer_id, cx)
                .detach_and_log_err(cx);
            serialized_transaction
                .buffer_ids
                .push(buffer.read(cx).remote_id().into());
            serialized_transaction
                .transactions
                .push(language::proto::serialize_transaction(&transaction));
        }
        serialized_transaction
    }

    /// Drops the per-peer shared-buffer registry. Called from
    /// `Project::reshared` (collaborators get re-introduced) and from
    /// `Project::unshare_internal`.
    pub fn forget_shared_buffers(&mut self) {
        self.shared_buffers.clear();
    }

    pub fn forget_shared_buffers_for(&mut self, peer_id: &proto::PeerId) {
        self.shared_buffers.remove(peer_id);
    }

    pub fn update_shared_buffer_peer_id(
        &mut self,
        old_peer_id: &proto::PeerId,
        new_peer_id: proto::PeerId,
    ) {
        if let Some(buffers) = self.shared_buffers.remove(old_peer_id) {
            self.shared_buffers.insert(new_peer_id, buffers);
        }
    }

    pub fn has_shared_buffers(&self) -> bool {
        !self.shared_buffers.is_empty()
    }

    /// Records the language-server handle that's keeping a buffer alive on
    /// behalf of a peer. Mirrors what was on `BufferStore` before Phase 1.
    pub fn register_shared_lsp_handle(
        &mut self,
        peer_id: proto::PeerId,
        buffer_id: BufferId,
        handle: OpenLspBufferHandle,
    ) {
        if let Some(shared_buffers) = self.shared_buffers.get_mut(&peer_id)
            && let Some(buffer) = shared_buffers.get_mut(&buffer_id)
        {
            buffer.lsp_handle = Some(handle);
            return;
        }
        debug_panic!("tried to register shared lsp handle, but buffer was not shared")
    }

    /// Forwards a language-server log entry from the global `LogStore` to the
    /// downstream peer when this project is collab-shared. Replaces the path
    /// where `LogStore::emit_event` used to read `LspStore::downstream_client`
    /// directly.
    pub fn forward_language_server_log_to_peer(
        &self,
        server_id: LanguageServerId,
        kind: LanguageServerLogType,
        message: String,
    ) {
        if let ProjectClientState::Shared { remote_id } = &self.client_state {
            self.collab_client
                .send(proto::LanguageServerLog {
                    project_id: *remote_id,
                    language_server_id: server_id.to_proto(),
                    message,
                    log_type: Some(kind.to_proto()),
                })
                .ok();
        }
    }

    pub fn is_shared(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Shared { .. } => true,
            ProjectClientState::Local => false,
            ProjectClientState::Collab { .. } => true,
        }
    }

    /// Returns the resolved version of `path`, that was found in `buffer`, if it exists.
    pub fn resolve_path_in_buffer(
        &self,
        path: &str,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Option<ResolvedPath>> {
        if util::paths::is_absolute(path, self.path_style(cx)) || path.starts_with("~") {
            self.resolve_abs_path(path, cx)
        } else {
            self.resolve_path_in_worktrees(path, buffer, cx)
        }
    }

    pub fn resolve_abs_file_path(
        &self,
        path: &str,
        cx: &mut Context<Self>,
    ) -> Task<Option<ResolvedPath>> {
        let resolve_task = self.resolve_abs_path(path, cx);
        cx.background_spawn(async move {
            let resolved_path = resolve_task.await;
            resolved_path.filter(|path| path.is_file())
        })
    }

    pub fn resolve_abs_path(&self, path: &str, cx: &App) -> Task<Option<ResolvedPath>> {
        if self.is_local(cx) {
            let expanded = PathBuf::from(shellexpand::tilde(&path).into_owned());
            let fs = self.fs(cx);
            cx.background_spawn(async move {
                let metadata = fs.metadata(&expanded).await.ok().flatten();

                metadata.map(|metadata| ResolvedPath::AbsPath {
                    path: expanded.to_string_lossy().into_owned(),
                    is_dir: metadata.is_dir,
                })
            })
        } else if let Some(ssh_client) = self.host.read(cx).remote_client.as_ref() {
            let request = ssh_client
                .read(cx)
                .proto_client()
                .request(proto::GetPathMetadata {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                    path: path.into(),
                });
            cx.background_spawn(async move {
                let response = request.await.log_err()?;
                if response.exists {
                    Some(ResolvedPath::AbsPath {
                        path: response.path,
                        is_dir: response.is_dir,
                    })
                } else {
                    None
                }
            })
        } else {
            Task::ready(None)
        }
    }

    fn resolve_path_in_worktrees(
        &self,
        path: &str,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Option<ResolvedPath>> {
        let mut candidates = vec![];
        let path_style = self.path_style(cx);
        if let Ok(path) = RelPath::new(path.as_ref(), path_style) {
            candidates.push(path.into_arc());
        }

        if let Some(file) = buffer.read(cx).file()
            && let Some(dir) = file.path().parent()
        {
            if let Some(joined) = path_style.join(&*dir.display(path_style), path)
                && let Some(joined) = RelPath::new(joined.as_ref(), path_style).ok()
            {
                candidates.push(joined.into_arc());
            }
        }

        let buffer_worktree_id = buffer.read(cx).file().map(|file| file.worktree_id(cx));
        let worktrees_with_ids: Vec<_> = self
            .worktrees(cx)
            .map(|worktree| {
                let id = worktree.read(cx).id();
                (worktree, id)
            })
            .collect();

        cx.spawn(async move |_, cx| {
            if let Some(buffer_worktree_id) = buffer_worktree_id
                && let Some((worktree, _)) = worktrees_with_ids
                    .iter()
                    .find(|(_, id)| *id == buffer_worktree_id)
            {
                for candidate in candidates.iter() {
                    if let Some(path) = Self::resolve_path_in_worktree(worktree, candidate, cx) {
                        return Some(path);
                    }
                }
            }
            for (worktree, id) in worktrees_with_ids {
                if Some(id) == buffer_worktree_id {
                    continue;
                }
                for candidate in candidates.iter() {
                    if let Some(path) = Self::resolve_path_in_worktree(&worktree, candidate, cx) {
                        return Some(path);
                    }
                }
            }
            None
        })
    }

    fn resolve_path_in_worktree(
        worktree: &Entity<Worktree>,
        path: &RelPath,
        cx: &mut AsyncApp,
    ) -> Option<ResolvedPath> {
        worktree.read_with(cx, |worktree, _| {
            worktree.entry_for_path(path).map(|entry| {
                let project_path = ProjectPath {
                    worktree_id: worktree.id(),
                    path: entry.path.clone(),
                };
                ResolvedPath::ProjectPath {
                    project_path,
                    is_dir: entry.is_dir(),
                }
            })
        })
    }

    pub fn list_directory(
        &self,
        query: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<DirectoryItem>>> {
        if self.is_local(cx) {
            DirectoryLister::Local(cx.entity(), self.fs(cx)).list_directory(query, cx)
        } else if let Some(session) = self.host.read(cx).remote_client.as_ref() {
            let request = proto::ListRemoteDirectory {
                dev_server_id: REMOTE_SERVER_PROJECT_ID,
                path: query,
                config: Some(proto::ListRemoteDirectoryConfig { is_dir: true }),
            };

            let response = session.read(cx).proto_client().request(request);
            cx.background_spawn(async move {
                let proto::ListRemoteDirectoryResponse {
                    entries,
                    entry_info,
                } = response.await?;
                Ok(entries
                    .into_iter()
                    .zip(entry_info)
                    .map(|(entry, info)| DirectoryItem {
                        path: PathBuf::from(entry),
                        is_dir: info.is_dir,
                    })
                    .collect())
            })
        } else {
            Task::ready(Err(anyhow!("cannot list directory in remote project")))
        }
    }

    pub fn create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Worktree>>> {
        let path_arc: Arc<Path> = Arc::from(abs_path.as_ref());
        // `create_worktree` always forces a new worktree (no
        // find-existing path). The `WorktreeAdded` event handler will
        // claim synchronously via `pending_worktree_paths`.
        self.pending_worktree_paths.insert(path_arc);
        self.worktree_store(cx).update(cx, |worktree_store, cx| {
            worktree_store.create_worktree(abs_path, visible, cx)
        })
    }

    /// Returns a task that resolves when the given worktree's `Entity` is
    /// fully dropped (all strong references released), not merely when
    /// `remove_worktree` is called. `remove_worktree` drops the store's
    /// reference and emits `WorktreeRemoved`, but other code may still
    /// hold a strong handle — the worktree isn't safe to delete from
    /// disk until every handle is gone.
    ///
    /// We use `observe_release` on the specific entity rather than
    /// listening for `WorktreeReleased` events because it's simpler at
    /// the call site (one awaitable task, no subscription / channel /
    /// ID filtering).
    pub fn wait_for_worktree_release(
        &mut self,
        worktree_id: WorktreeId,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(worktree) = self.worktree_for_id(worktree_id, cx) else {
            return Task::ready(Ok(()));
        };

        let (released_tx, released_rx) = futures::channel::oneshot::channel();
        let released_tx = std::sync::Arc::new(Mutex::new(Some(released_tx)));
        let release_subscription =
            cx.observe_release(&worktree, move |_project, _released_worktree, _cx| {
                if let Some(released_tx) = released_tx.lock().take() {
                    let _ = released_tx.send(());
                }
            });

        cx.spawn(async move |_project, _cx| {
            let _release_subscription = release_subscription;
            released_rx
                .await
                .map_err(|_| anyhow!("worktree release observer dropped before release"))?;
            Ok(())
        })
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
        self.worktree_store(cx).update(cx, |worktree_store, cx| {
            worktree_store.remove_worktree(id_to_remove, cx);
        });
    }

    pub fn remove_worktree_for_main_worktree_path(
        &mut self,
        path: impl AsRef<Path>,
        cx: &mut Context<Self>,
    ) {
        let path = path.as_ref();
        self.worktree_store(cx).update(cx, |worktree_store, cx| {
            if let Some(worktree) = worktree_store.worktree_for_main_worktree_path(path, cx) {
                worktree_store.remove_worktree(worktree.read(cx).id(), cx);
            }
        });
    }

    fn add_worktree(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        self.worktree_store(cx).update(cx, |worktree_store, cx| {
            worktree_store.add(worktree, cx);
        });
        // Claim explicitly: the `WorktreeAdded` event no longer
        // auto-claims in Phase 2 sharing. Used by
        // `Project::from_join_project_response` to register the
        // worktrees that arrived in the join response.
        self.claim_found_worktree(worktree, cx);
    }

    pub fn set_active_path(&mut self, entry: Option<ProjectPath>, cx: &mut Context<Self>) {
        let new_active_entry = entry.and_then(|project_path| {
            let worktree = self.worktree_for_id(project_path.worktree_id, cx)?;
            let entry = worktree.read(cx).entry_for_path(&project_path.path)?;
            Some(entry.id)
        });
        if new_active_entry != self.active_entry {
            self.active_entry = new_active_entry;
            cx.emit(Event::ActiveEntryChanged(new_active_entry));
        }
    }

    pub fn language_servers_running_disk_based_diagnostics<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl Iterator<Item = LanguageServerId> + 'a {
        self.lsp_store(cx)
            .read(cx)
            .language_servers_running_disk_based_diagnostics()
    }

    pub fn diagnostic_summary(&self, include_ignored: bool, cx: &App) -> DiagnosticSummary {
        self.lsp_store(cx)
            .read(cx)
            .diagnostic_summary(include_ignored, cx)
    }

    /// Returns a summary of the diagnostics for the provided project path only.
    pub fn diagnostic_summary_for_path(&self, path: &ProjectPath, cx: &App) -> DiagnosticSummary {
        self.lsp_store(cx)
            .read(cx)
            .diagnostic_summary_for_path(path, cx)
    }

    pub fn diagnostic_summaries<'a>(
        &'a self,
        include_ignored: bool,
        cx: &'a App,
    ) -> impl Iterator<Item = (ProjectPath, LanguageServerId, DiagnosticSummary)> + 'a {
        self.lsp_store(cx)
            .read(cx)
            .diagnostic_summaries(include_ignored, cx)
    }

    pub fn active_entry(&self) -> Option<ProjectEntryId> {
        self.active_entry
    }

    pub fn entry_for_path<'a>(&'a self, path: &ProjectPath, cx: &'a App) -> Option<&'a Entry> {
        self.worktree_store(cx).read(cx).entry_for_path(path, cx)
    }

    pub fn path_for_entry(&self, entry_id: ProjectEntryId, cx: &App) -> Option<ProjectPath> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        let worktree = worktree.read(cx);
        let worktree_id = worktree.id();
        let path = worktree.entry_for_id(entry_id)?.path.clone();
        Some(ProjectPath { worktree_id, path })
    }

    pub fn absolute_path(&self, project_path: &ProjectPath, cx: &App) -> Option<PathBuf> {
        Some(
            self.worktree_for_id(project_path.worktree_id, cx)?
                .read(cx)
                .absolutize(&project_path.path),
        )
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
    /// * `path` - An absolute path, or a full path that starts with a worktree root name, or a
    ///   relative path within a visible worktree.
    /// * `cx` - A reference to the `AppContext`.
    ///
    /// # Returns
    ///
    /// Returns `Some(ProjectPath)` if a matching worktree is found, otherwise `None`.
    pub fn find_project_path(&self, path: impl AsRef<Path>, cx: &App) -> Option<ProjectPath> {
        let path_style = self.path_style(cx);
        let path = path.as_ref();
        let worktree_store = self.worktree_store(cx).read(cx);

        if is_absolute(&path.to_string_lossy(), path_style) {
            for worktree in worktree_store.visible_worktrees(cx) {
                let worktree_abs_path = worktree.read(cx).abs_path();

                if let Ok(relative_path) = path.strip_prefix(worktree_abs_path)
                    && let Ok(path) = RelPath::new(relative_path, path_style)
                {
                    return Some(ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: path.into_arc(),
                    });
                }
            }
        } else {
            // First pass: for each worktree, try two interpretations of the path and
            // return whichever finds an existing entry first:
            //   (a) Strip the worktree root name as a prefix.
            //   (b) Treat the path as a literal worktree-relative path.
            for worktree in worktree_store.visible_worktrees(cx) {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name().as_std_path())
                    && let Ok(rel_path) = RelPath::new(relative_path, path_style)
                    && let Some(entry) = worktree.entry_for_path(&rel_path)
                {
                    return Some(ProjectPath {
                        worktree_id: worktree.id(),
                        path: entry.path.clone(),
                    });
                }
                if let Ok(rel_path) = RelPath::new(path, path_style)
                    && let Some(entry) = worktree.entry_for_path(&rel_path)
                {
                    return Some(ProjectPath {
                        worktree_id: worktree.id(),
                        path: entry.path.clone(),
                    });
                }
            }

            // Second pass: strip the worktree root name prefix without requiring the
            // entry to exist, to allow resolving paths that don't exist yet.
            for worktree in worktree_store.visible_worktrees(cx) {
                let worktree_root_name = worktree.read(cx).root_name();
                if let Ok(relative_path) = path.strip_prefix(worktree_root_name.as_std_path())
                    && let Ok(path) = RelPath::new(relative_path, path_style)
                {
                    return Some(ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: path.into_arc(),
                    });
                }
            }
        }

        None
    }

    /// If there's only one visible worktree, returns the given worktree-relative path with no prefix.
    ///
    /// Otherwise, returns the full path for the project path (obtained by prefixing the worktree-relative path with the name of the worktree).
    pub fn short_full_path_for_project_path(
        &self,
        project_path: &ProjectPath,
        cx: &App,
    ) -> Option<String> {
        let path_style = self.path_style(cx);
        if self.visible_worktrees(cx).take(2).count() < 2 {
            return Some(project_path.path.display(path_style).to_string());
        }
        self.worktree_for_id(project_path.worktree_id, cx)
            .map(|worktree| {
                let worktree_name = worktree.read(cx).root_name();
                worktree_name
                    .join(&project_path.path)
                    .display(path_style)
                    .to_string()
            })
    }

    pub fn project_path_for_absolute_path(&self, abs_path: &Path, cx: &App) -> Option<ProjectPath> {
        self.worktree_store(cx)
            .read(cx)
            .project_path_for_absolute_path(abs_path, cx)
    }

    pub fn get_workspace_root(&self, project_path: &ProjectPath, cx: &App) -> Option<PathBuf> {
        Some(
            self.worktree_for_id(project_path.worktree_id, cx)?
                .read(cx)
                .abs_path()
                .to_path_buf(),
        )
    }

    pub fn blame_buffer(
        &self,
        buffer: &Entity<Buffer>,
        version: Option<clock::Global>,
        cx: &mut App,
    ) -> Task<Result<Option<Blame>>> {
        self.git_store(cx).update(cx, |git_store, cx| {
            git_store.blame_buffer(buffer, version, cx)
        })
    }

    pub fn get_permalink_to_line(
        &self,
        buffer: &Entity<Buffer>,
        selection: Range<u32>,
        cx: &mut App,
    ) -> Task<Result<url::Url>> {
        self.git_store(cx).update(cx, |git_store, cx| {
            git_store.get_permalink_to_line(buffer, selection, cx)
        })
    }

    // RPC message handlers

    /// Forwards `proto::LspQuery` rpc to the LSP store while supplying the
    /// downstream peer info. Lives on `Project` because the `LspStore` no
    /// longer holds the downstream client. Silently acks when the project
    /// is not shared, mirroring the previous LspStore-side behavior of
    /// no-op'ing when `downstream_client` was `None`.
    async fn handle_lsp_query(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::LspQuery>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let downstream = this.read_with(&cx, |project, cx| {
            project.remote_id().map(|project_id| {
                (
                    project.lsp_store(cx),
                    AnyProtoClient::from(project.collab_client.clone()),
                    project_id,
                )
            })
        });
        let Some((lsp_store, downstream_client, downstream_project_id)) = downstream else {
            return Ok(proto::Ack {});
        };
        LspStore::process_lsp_query::<Self>(
            lsp_store,
            this.downgrade(),
            downstream_client,
            downstream_project_id,
            envelope,
            cx,
        )
        .await
    }

    /// Forwards `proto::Fetch` rpc to the git store with the downstream
    /// client used by the askpass round-trip. Lives on `Project` because
    /// `GitStore` no longer holds a downstream client.
    async fn handle_fetch(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Fetch>,
        cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let (git_store, downstream_client) = this.read_with(&cx, |project, cx| {
            Self::git_downstream_for_handler(project, cx)
        });
        GitStore::process_fetch(git_store, downstream_client, envelope, cx).await
    }

    /// Forwards `proto::Push` rpc. See [`Self::handle_fetch`].
    async fn handle_push(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Push>,
        cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let (git_store, downstream_client) = this.read_with(&cx, |project, cx| {
            Self::git_downstream_for_handler(project, cx)
        });
        GitStore::process_push(git_store, downstream_client, envelope, cx).await
    }

    /// Forwards `proto::Pull` rpc. See [`Self::handle_fetch`].
    async fn handle_pull(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Pull>,
        cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let (git_store, downstream_client) = this.read_with(&cx, |project, cx| {
            Self::git_downstream_for_handler(project, cx)
        });
        GitStore::process_pull(git_store, downstream_client, envelope, cx).await
    }

    /// Forwards `proto::Commit` rpc. See [`Self::handle_fetch`].
    async fn handle_commit(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Commit>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let (git_store, downstream_client) = this.read_with(&cx, |project, cx| {
            Self::git_downstream_for_handler(project, cx)
        });
        GitStore::process_commit(git_store, downstream_client, envelope, cx).await
    }

    /// Helper for the four `handle_fetch`/`handle_push`/`handle_pull`/
    /// `handle_commit` handlers. The downstream client is whichever client
    /// received the rpc envelope (collab in the host case, remote_proto in
    /// the SSH-host case); we read it from `Project::collab_client`. The
    /// project id passed to `make_remote_delegate` comes from the envelope,
    /// not from us.
    fn git_downstream_for_handler(project: &Self, cx: &App) -> (Entity<GitStore>, AnyProtoClient) {
        (
            project.git_store(cx),
            AnyProtoClient::from(project.collab_client.clone()),
        )
    }

    /// Forwards `proto::ApplyCodeAction`. Calls `LspStore::process_apply_code_action`
    /// for the LSP work, then serializes the resulting transaction with
    /// `Project::serialize_project_transaction_for_peer`. Lives on `Project`
    /// because the serialize step depends on `Project::shared_buffers`.
    async fn handle_apply_code_action(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeAction>,
        mut cx: AsyncApp,
    ) -> Result<proto::ApplyCodeActionResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let (lsp_store, active_entry) = this.read_with(&cx, |project, cx| {
            (project.lsp_store(cx), project.active_entry)
        });
        let project_transaction =
            LspStore::process_apply_code_action(lsp_store, envelope, active_entry, cx.clone())
                .await?;
        let serialized = this.update(&mut cx, |project, cx| {
            project.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::ApplyCodeActionResponse {
            transaction: Some(serialized),
        })
    }

    /// Forwards `proto::ApplyCodeActionKind`. See [`Self::handle_apply_code_action`].
    async fn handle_apply_code_action_kind(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeActionKind>,
        mut cx: AsyncApp,
    ) -> Result<proto::ApplyCodeActionKindResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let (lsp_store, active_entry) = this.read_with(&cx, |project, cx| {
            (project.lsp_store(cx), project.active_entry)
        });
        let project_transaction =
            LspStore::process_apply_code_action_kind(lsp_store, envelope, active_entry, cx.clone())
                .await?;
        let serialized = this.update(&mut cx, |project, cx| {
            project.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::ApplyCodeActionKindResponse {
            transaction: Some(serialized),
        })
    }

    /// Forwards `proto::FormatBuffers`. See [`Self::handle_apply_code_action`].
    async fn handle_format_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FormatBuffers>,
        mut cx: AsyncApp,
    ) -> Result<proto::FormatBuffersResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let lsp_store = this.read_with(&cx, |project, cx| project.lsp_store(cx));
        let project_transaction =
            LspStore::process_format_buffers(lsp_store, envelope, cx.clone()).await?;
        let serialized = this.update(&mut cx, |project, cx| {
            project.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::FormatBuffersResponse {
            transaction: Some(serialized),
        })
    }

    /// Forwards `proto::OpenBufferForSymbol`. Looks up the buffer via the
    /// LSP store, then uses `Project::create_buffer_for_peer` to share it.
    async fn handle_open_buffer_for_symbol(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenBufferForSymbol>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferForSymbolResponse> {
        let peer_id = envelope.original_sender_id().unwrap_or_default();
        let lsp_store = this.read_with(&cx, |project, cx| project.lsp_store(cx));
        let buffer =
            LspStore::process_open_buffer_for_symbol(lsp_store, envelope, cx.clone()).await?;
        this.update(&mut cx, |project, cx| {
            let is_private = buffer
                .read(cx)
                .file()
                .map(|f| f.is_private())
                .unwrap_or_default();
            if is_private {
                Err(anyhow!(rpc::ErrorCode::UnsharedItem))
            } else {
                project
                    .create_buffer_for_peer(&buffer, peer_id, cx)
                    .detach_and_log_err(cx);
                let buffer_id = buffer.read(cx).remote_id().to_proto();
                Ok(proto::OpenBufferForSymbolResponse { buffer_id })
            }
        })
    }

    /// Forwards `proto::RenameProjectEntry`. Calls
    /// `LspStore::process_rename_project_entry` with the host's
    /// `active_entry` so the snippet-vs-edit gating in
    /// `LocalLspStore::deserialize_workspace_edit` has the project state
    /// it needs.
    async fn handle_rename_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RenameProjectEntry>,
        cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let (lsp_store, active_entry) = this.read_with(&cx, |project, cx| {
            (project.lsp_store(cx), project.active_entry)
        });
        LspStore::process_rename_project_entry(lsp_store, envelope, active_entry, cx).await
    }

    /// Forwards `proto::RegisterBufferWithLanguageServers`. Calls into the
    /// LSP store and (when not just forwarding upstream) records the LSP
    /// handle in `Project::shared_buffers`.
    async fn handle_register_buffer_with_language_servers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RegisterBufferWithLanguageServers>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let peer_id = envelope.original_sender_id.unwrap_or(envelope.sender_id);
        let lsp_store = this.read_with(&cx, |project, cx| project.lsp_store(cx));
        let registered =
            LspStore::process_register_buffer_with_language_servers(lsp_store, envelope, &mut cx)?;
        if let Some((buffer_id, handle)) = registered {
            this.update(&mut cx, |project, _| {
                project.register_shared_lsp_handle(peer_id, buffer_id, handle);
            });
        }
        Ok(proto::Ack {})
    }

    /// Generic wrapper for LSP commands that need `Project` access for
    /// `T::response_to_proto_project` (i.e., commands that serialize
    /// per-peer buffer state). Registered on `Project` for `PerformRename`,
    /// `lsp_ext_command::GoToParentModule`, and `lsp_ext_command::GetLspRunnables`.
    /// Other `LspCommand`s' rpc registrations stay on `LspStore` because
    /// their `response_to_proto` is self-contained.
    async fn handle_lsp_command_with_project<T: LspCommand>(
        this: Entity<Self>,
        envelope: TypedEnvelope<T::ProtoRequest>,
        mut cx: AsyncApp,
    ) -> Result<<T::ProtoRequest as proto::RequestMessage>::Response>
    where
        <T::LspRequest as lsp::request::Request>::Params: Send,
        <T::LspRequest as lsp::request::Request>::Result: Send,
    {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let buffer_id = T::buffer_id_from_proto(&envelope.payload)?;
        let (lsp_store, active_entry) = this.read_with(&cx, |project, cx| {
            (project.lsp_store(cx), project.active_entry)
        });
        let buffer_handle = lsp_store.update(&mut cx, |lsp_store, cx| {
            lsp_store.buffer_store().read(cx).get_existing(buffer_id)
        })?;
        let mut request = T::from_proto(
            envelope.payload,
            lsp_store.clone(),
            buffer_handle.clone(),
            cx.clone(),
        )
        .await?;
        // For `PerformRename`, inject the host's `active_entry` so
        // `LocalLspStore::deserialize_workspace_edit` can gate snippet
        // emission. Default no-op for other commands.
        request.set_active_entry(active_entry);
        let response = lsp_store
            .update(&mut cx, |lsp_store, cx| {
                lsp_store.request_lsp(
                    buffer_handle.clone(),
                    LanguageServerToQuery::FirstCapable,
                    request,
                    cx,
                )
            })
            .await?;
        this.update(&mut cx, |project, cx| {
            Ok(T::response_to_proto_project(
                response,
                lsp_store.clone(),
                project,
                sender_id,
                &buffer_handle.read(cx).version(),
                cx,
            ))
        })
    }

    /// Forwards `proto::OpenCommitMessageBuffer`. The git store opens the
    /// commit message buffer; this wrapper shares it with the requesting peer
    /// via `Project::create_buffer_for_peer`.
    async fn handle_open_commit_message_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenCommitMessageBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let peer_id = envelope.original_sender_id.unwrap_or(envelope.sender_id);
        let git_store = this.read_with(&cx, |project, cx| project.git_store(cx));
        let buffer =
            GitStore::process_open_commit_message_buffer(git_store, envelope, cx.clone()).await?;
        let buffer_id = buffer.read_with(&cx, |buffer, _| buffer.remote_id());
        this.update(&mut cx, |project, cx| {
            project
                .create_buffer_for_peer(&buffer, peer_id, cx)
                .detach_and_log_err(cx);
        });
        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    async fn handle_unshare_project(
        this: Entity<Self>,
        _: TypedEnvelope<proto::UnshareProject>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            if this.is_local(cx) || this.is_via_remote_server(cx) {
                this.unshare(cx)?;
            } else {
                this.disconnected_from_host(cx);
            }
            Ok(())
        })
    }

    async fn handle_add_collaborator(
        this: Entity<Self>,
        mut envelope: TypedEnvelope<proto::AddProjectCollaborator>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let collaborator = envelope
            .payload
            .collaborator
            .take()
            .context("empty collaborator")?;

        let collaborator = Collaborator::from_proto(collaborator)?;
        this.update(&mut cx, |this, cx| {
            this.forget_shared_buffers_for(&collaborator.peer_id);
            if let ProjectClientState::Shared { remote_id } = &this.client_state {
                this.breakpoint_store(cx)
                    .read(cx)
                    .broadcast(&this.collab_client.clone().into(), *remote_id);
            }
            cx.emit(Event::CollaboratorJoined(collaborator.peer_id));
            this.collaborators
                .insert(collaborator.peer_id, collaborator);
        });

        Ok(())
    }

    async fn handle_update_project_collaborator(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateProjectCollaborator>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let old_peer_id = envelope
            .payload
            .old_peer_id
            .context("missing old peer id")?;
        let new_peer_id = envelope
            .payload
            .new_peer_id
            .context("missing new peer id")?;
        this.update(&mut cx, |this, cx| {
            let collaborator = this
                .collaborators
                .remove(&old_peer_id)
                .context("received UpdateProjectCollaborator for unknown peer")?;
            let is_host = collaborator.is_host;
            this.collaborators.insert(new_peer_id, collaborator);

            log::info!("peer {} became {}", old_peer_id, new_peer_id,);
            this.update_shared_buffer_peer_id(&old_peer_id, new_peer_id);

            if is_host {
                this.buffer_store(cx)
                    .update(cx, |buffer_store, _| buffer_store.discard_incomplete());
                this.enqueue_buffer_ordered_message(BufferOrderedMessage::Resync)
                    .unwrap();
                cx.emit(Event::HostReshared);
            }

            cx.emit(Event::CollaboratorUpdated {
                old_peer_id,
                new_peer_id,
            });
            Ok(())
        })
    }

    async fn handle_remove_collaborator(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RemoveProjectCollaborator>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let peer_id = envelope.payload.peer_id.context("invalid peer id")?;
            let replica_id = this
                .collaborators
                .remove(&peer_id)
                .with_context(|| format!("unknown peer {peer_id:?}"))?
                .replica_id;
            this.forget_shared_buffers_for(&peer_id);
            // Iterate this Project's owned buffers, not the host
            // BufferStore's full set: in Phase 2 sharing, a peer leaving
            // *this* Project shouldn't strip its replica from buffers
            // owned by a sibling Project on the same host.
            let buffer_store = this.buffer_store(cx);
            let buffers: Vec<Entity<Buffer>> = this
                .buffers
                .iter()
                .filter_map(|id| buffer_store.read(cx).get(*id))
                .collect();
            for buffer in buffers {
                buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
            }
            this.git_store(cx).update(cx, |git_store, _| {
                git_store.forget_shared_diffs_for(&peer_id);
            });

            cx.emit(Event::CollaboratorLeft(peer_id));
            Ok(())
        })
    }

    async fn handle_update_project(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateProject>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            // Don't handle messages that were sent before the response to us joining the project
            if envelope.message_id > this.join_project_response_message_id {
                cx.update_global::<SettingsStore, _>(|store, cx| {
                    for worktree_metadata in &envelope.payload.worktrees {
                        store
                            .clear_local_settings(WorktreeId::from_proto(worktree_metadata.id), cx)
                            .log_err();
                    }
                });

                this.set_worktrees_from_proto(envelope.payload.worktrees, cx)?;
            }
            Ok(())
        })
    }

    async fn handle_toast(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Toast>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |_, cx| {
            cx.emit(Event::Toast {
                notification_id: envelope.payload.notification_id.into(),
                message: envelope.payload.message,
                link: None,
            });
            Ok(())
        })
    }

    async fn handle_language_server_prompt_request(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::LanguageServerPromptRequest>,
        mut cx: AsyncApp,
    ) -> Result<proto::LanguageServerPromptResponse> {
        let (tx, rx) = async_channel::bounded(1);
        let actions: Vec<_> = envelope
            .payload
            .actions
            .into_iter()
            .map(|action| MessageActionItem {
                title: action,
                properties: Default::default(),
            })
            .collect();
        this.update(&mut cx, |_, cx| {
            cx.emit(Event::LanguageServerPrompt(
                LanguageServerPromptRequest::new(
                    proto_to_prompt(envelope.payload.level.context("Invalid prompt level")?),
                    envelope.payload.message,
                    actions.clone(),
                    envelope.payload.lsp_name,
                    tx,
                ),
            ));

            anyhow::Ok(())
        })?;

        // We drop `this` to avoid holding a reference in this future for too
        // long.
        // If we keep the reference, we might not drop the `Project` early
        // enough when closing a window and it will only get releases on the
        // next `flush_effects()` call.
        drop(this);

        let mut rx = pin!(rx);
        let answer = rx.next().await;

        Ok(LanguageServerPromptResponse {
            action_response: answer.and_then(|answer| {
                actions
                    .iter()
                    .position(|action| *action == answer)
                    .map(|index| index as u64)
            }),
        })
    }

    async fn handle_hide_toast(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::HideToast>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |_, cx| {
            cx.emit(Event::HideToast {
                notification_id: envelope.payload.notification_id.into(),
            });
            Ok(())
        })
    }

    // Collab sends UpdateWorktree protos as messages
    async fn handle_update_worktree(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateWorktree>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |project, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            if let Some(worktree) = project.worktree_for_id(worktree_id, cx) {
                worktree.update(cx, |worktree, _| {
                    let worktree = worktree.as_remote_mut().unwrap();
                    worktree.update_from_remote(envelope.payload);
                });
            }
            Ok(())
        })
    }

    async fn handle_update_buffer_from_remote_server(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let buffer_store = this.read_with(&cx, |this, cx| {
            if let Some(remote_id) = this.remote_id() {
                let mut payload = envelope.payload.clone();
                payload.project_id = remote_id;
                cx.background_spawn(this.collab_client.request(payload))
                    .detach_and_log_err(cx);
            }
            this.buffer_store(cx)
        });
        BufferStore::handle_update_buffer(buffer_store, envelope, cx).await
    }

    async fn handle_trust_worktrees(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::TrustWorktrees>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        if this.read_with(&cx, |project, _cx| project.is_via_collab()) {
            return Ok(proto::Ack {});
        }

        let trusted_worktrees = cx
            .update(|cx| TrustedWorktrees::try_get_global(cx))
            .context("missing trusted worktrees")?;
        trusted_worktrees.update(&mut cx, |trusted_worktrees, cx| {
            trusted_worktrees.trust(
                &this.read(cx).worktree_store(cx),
                envelope
                    .payload
                    .trusted_paths
                    .into_iter()
                    .filter_map(|proto_path| PathTrust::from_proto(proto_path))
                    .collect(),
                cx,
            );
        });
        Ok(proto::Ack {})
    }

    async fn handle_restrict_worktrees(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RestrictWorktrees>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        if this.read_with(&cx, |project, _cx| project.is_via_collab()) {
            return Ok(proto::Ack {});
        }

        let trusted_worktrees = cx
            .update(|cx| TrustedWorktrees::try_get_global(cx))
            .context("missing trusted worktrees")?;
        trusted_worktrees.update(&mut cx, |trusted_worktrees, cx| {
            let worktree_store = this.read(cx).worktree_store(cx).downgrade();
            let restricted_paths = envelope
                .payload
                .worktree_ids
                .into_iter()
                .map(WorktreeId::from_proto)
                .map(PathTrust::Worktree)
                .collect::<HashSet<_>>();
            trusted_worktrees.restrict(worktree_store, restricted_paths, cx);
        });
        Ok(proto::Ack {})
    }

    // Goes from host to client.
    async fn handle_find_search_candidates_chunk(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidatesChunk>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let buffer_store = this.read_with(&mut cx, |this, cx| this.buffer_store(cx));
        BufferStore::handle_find_search_candidates_chunk(buffer_store, envelope, cx).await
    }

    // Goes from client to host.
    async fn handle_find_search_candidates_cancel(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidatesCancelled>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let buffer_store = this.read_with(&mut cx, |this, cx| this.buffer_store(cx));
        BufferStore::handle_find_search_candidates_cancel(buffer_store, envelope, cx).await
    }

    async fn handle_update_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let buffer_store = this.read_with(&cx, |this, cx| {
            if let Some(ssh) = &this.host.read(cx).remote_client {
                let mut payload = envelope.payload.clone();
                payload.project_id = REMOTE_SERVER_PROJECT_ID;
                cx.background_spawn(ssh.read(cx).proto_client().request(payload))
                    .detach_and_log_err(cx);
            }
            this.buffer_store(cx)
        });
        BufferStore::handle_update_buffer(buffer_store, envelope, cx).await
    }

    fn retain_remotely_created_models(
        &mut self,
        cx: &mut Context<Self>,
    ) -> RemotelyCreatedModelGuard {
        // The retained snapshot exists to keep buffers/worktrees alive
        // for the duration of an in-flight remote operation. Capture
        // *this* Project's view (its owned buffer ids and worktree
        // handles) so a sibling Project's models on the same shared
        // host store aren't kept alive by our retention.
        let buffer_store = self.buffer_store(cx);
        let buffers: Vec<Entity<Buffer>> = self
            .buffers
            .iter()
            .filter_map(|id| buffer_store.read(cx).get(*id))
            .collect();
        let worktrees: Vec<Entity<Worktree>> = self.worktrees(cx).collect();
        {
            let mut remotely_create_models = self.remotely_created_models.lock();
            if remotely_create_models.retain_count == 0 {
                remotely_create_models.buffers = buffers;
                remotely_create_models.worktrees = worktrees;
            }
            remotely_create_models.retain_count += 1;
        }
        RemotelyCreatedModelGuard {
            remote_models: Arc::downgrade(&self.remotely_created_models),
        }
    }

    async fn handle_create_buffer_for_peer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CreateBufferForPeer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let added = this.buffer_store(cx).update(cx, |buffer_store, cx| {
                buffer_store.handle_create_buffer_for_peer(
                    envelope,
                    this.replica_id(cx),
                    this.capability(),
                    cx,
                )
            })?;
            // Peer-streamed buffers belong to whichever Project this RPC
            // routes to (RPC handlers are per-Project entities). Claim
            // explicitly so pathless peer buffers and buffers in
            // worktrees we don't yet have aren't dropped on the floor by
            // the path-based `BufferAdded` handler.
            if let Some(buffer) = added {
                this.claim_buffer(&buffer, cx);
            }
            anyhow::Ok(())
        })
    }

    async fn handle_toggle_lsp_logs(
        project: Entity<Self>,
        envelope: TypedEnvelope<proto::ToggleLspLogs>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let toggled_log_kind =
            match proto::toggle_lsp_logs::LogType::from_i32(envelope.payload.log_type)
                .context("invalid log type")?
            {
                proto::toggle_lsp_logs::LogType::Log => LogKind::Logs,
                proto::toggle_lsp_logs::LogType::Trace => LogKind::Trace,
                proto::toggle_lsp_logs::LogType::Rpc => LogKind::Rpc,
            };
        project.update(&mut cx, |_, cx| {
            cx.emit(Event::ToggleLspLogs {
                server_id: LanguageServerId::from_proto(envelope.payload.server_id),
                enabled: envelope.payload.enabled,
                toggled_log_kind,
            })
        });
        Ok(())
    }

    async fn handle_synchronize_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SynchronizeBuffers>,
        mut cx: AsyncApp,
    ) -> Result<proto::SynchronizeBuffersResponse> {
        let response = this.update(&mut cx, |this, cx| {
            let project_id = envelope.payload.project_id;
            let mut response = proto::SynchronizeBuffersResponse {
                buffers: Default::default(),
            };
            let Some(guest_id) = envelope.original_sender_id else {
                anyhow::bail!("missing original_sender_id on SynchronizeBuffers request");
            };

            this.shared_buffers.entry(guest_id).or_default().clear();
            for buffer in envelope.payload.buffers {
                let buffer_id = BufferId::new(buffer.id)?;
                let remote_version = language::proto::deserialize_version(&buffer.version);
                if let Some(buffer) = this.buffer_store(cx).read(cx).get(buffer_id) {
                    this.shared_buffers
                        .entry(guest_id)
                        .or_default()
                        .entry(buffer_id)
                        .or_insert_with(|| SharedBuffer {
                            buffer: buffer.clone(),
                            lsp_handle: None,
                        });

                    let buffer_ref = buffer.read(cx);
                    response.buffers.push(proto::BufferVersion {
                        id: buffer_id.into(),
                        version: language::proto::serialize_version(&buffer_ref.version),
                    });

                    let operations = buffer_ref.serialize_ops(Some(remote_version), cx);
                    let client = this.collab_client.clone();
                    if let Some(file) = buffer_ref.file() {
                        client
                            .send(proto::UpdateBufferFile {
                                project_id,
                                buffer_id: buffer_id.into(),
                                file: Some(file.to_proto(cx)),
                            })
                            .log_err();
                    }

                    client
                        .send(proto::BufferReloaded {
                            project_id,
                            buffer_id: buffer_id.into(),
                            version: language::proto::serialize_version(buffer_ref.saved_version()),
                            mtime: buffer_ref.saved_mtime().map(|time| time.into()),
                            line_ending: language::proto::serialize_line_ending(
                                buffer_ref.line_ending(),
                            ) as i32,
                        })
                        .log_err();

                    cx.background_spawn(
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
            anyhow::Ok(response)
        })?;
        Ok(response)
    }

    async fn handle_close_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let peer_id = envelope.sender_id;
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        this.update(&mut cx, |this, cx| {
            let buffer_store = this.buffer_store(cx);
            if let Some(shared) = this.shared_buffers.get_mut(&peer_id)
                && shared.remove(&buffer_id).is_some()
            {
                // Emit on the buffer-store entity so existing subscribers
                // (notably `GitStore::on_buffer_store_event`) keep working
                // even though the per-peer state has moved up to `Project`.
                buffer_store.update(cx, |_, cx| {
                    cx.emit(BufferStoreEvent::SharedBufferClosed(peer_id, buffer_id));
                });
                if shared.is_empty() {
                    this.shared_buffers.remove(&peer_id);
                }
                return;
            }
            debug_panic!(
                "peer_id {} closed buffer_id {} which was either not open or already closed",
                peer_id,
                buffer_id
            )
        });
        Ok(())
    }

    async fn handle_reload_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ReloadBuffers>,
        mut cx: AsyncApp,
    ) -> Result<proto::ReloadBuffersResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let reload = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(this.buffer_store(cx).read(cx).get_existing(buffer_id)?);
            }
            anyhow::Ok(this.buffer_store(cx).update(cx, |buffer_store, cx| {
                buffer_store.reload_buffers(buffers, false, cx)
            }))
        })?;

        let project_transaction = reload.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::ReloadBuffersResponse {
            transaction: Some(project_transaction),
        })
    }

    // Goes from client to host.
    async fn handle_search_candidate_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidates>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let peer_id = envelope.original_sender_id.unwrap_or(envelope.sender_id);
        let message = envelope.payload;
        let project_id = message.project_id;
        let path_style = this.read_with(&cx, |this, cx| this.path_style(cx));
        let query =
            SearchQuery::from_proto(message.query.context("missing query field")?, path_style)?;

        let handle = message.handle;
        let buffer_store = this.read_with(&cx, |this, cx| this.buffer_store(cx));
        let client = this.read_with(&cx, |this, _cx| this.client());
        let task = cx.spawn(async move |cx| {
            let results = this.update(cx, |this, cx| {
                this.search_impl(query, cx).matching_buffers(cx)
            });
            let (batcher, batches) = project_search::AdaptiveBatcher::new(cx.background_executor());
            let mut new_matches = Box::pin(results.rx);

            let sender_task = cx.background_executor().spawn({
                let client = client.clone();
                async move {
                    let mut batches = std::pin::pin!(batches);
                    while let Some(buffer_ids) = batches.next().await {
                        client
                            .request(proto::FindSearchCandidatesChunk {
                                handle,
                                peer_id: Some(peer_id),
                                project_id,
                                variant: Some(
                                    proto::find_search_candidates_chunk::Variant::Matches(
                                        proto::FindSearchCandidatesMatches { buffer_ids },
                                    ),
                                ),
                            })
                            .await?;
                    }
                    anyhow::Ok(())
                }
            });

            while let Some(buffer) = new_matches.next().await {
                let buffer_id = this.update(cx, |this, cx| {
                    let buffer_id = buffer.read(cx).remote_id();
                    this.create_buffer_for_peer(&buffer, peer_id, cx)
                        .detach_and_log_err(cx);
                    buffer_id.to_proto()
                });
                batcher.push(buffer_id).await;
            }
            batcher.flush().await;

            sender_task.await?;

            let _ = client
                .request(proto::FindSearchCandidatesChunk {
                    handle,
                    peer_id: Some(peer_id),
                    project_id,
                    variant: Some(proto::find_search_candidates_chunk::Variant::Done(
                        proto::FindSearchCandidatesDone {},
                    )),
                })
                .await?;
            anyhow::Ok(())
        });
        buffer_store.update(&mut cx, |this, _| {
            this.register_ongoing_project_search((peer_id, handle), task);
        });

        Ok(proto::Ack {})
    }

    async fn handle_open_buffer_by_id(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenBufferById>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let peer_id = envelope.original_sender_id()?;
        let buffer_id = BufferId::new(envelope.payload.id)?;
        let buffer = this
            .update(&mut cx, |this, cx| this.open_buffer_by_id(buffer_id, cx))
            .await?;
        Project::respond_to_open_buffer_request(this, buffer, peer_id, &mut cx)
    }

    async fn handle_open_buffer_by_path(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenBufferByPath>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let peer_id = envelope.original_sender_id()?;
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let path = RelPath::from_proto(&envelope.payload.path)?;
        let open_buffer = this
            .update(&mut cx, |this, cx| {
                this.open_buffer(ProjectPath { worktree_id, path }, cx)
            })
            .await?;
        Project::respond_to_open_buffer_request(this, open_buffer, peer_id, &mut cx)
    }

    async fn handle_open_new_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenNewBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let buffer = this
            .update(&mut cx, |this, cx| this.create_buffer(None, true, cx))
            .await?;
        let peer_id = envelope.original_sender_id()?;

        Project::respond_to_open_buffer_request(this, buffer, peer_id, &mut cx)
    }

    fn respond_to_open_buffer_request(
        this: Entity<Self>,
        buffer: Entity<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        this.update(cx, |this, cx| {
            let is_private = buffer
                .read(cx)
                .file()
                .map(|f| f.is_private())
                .unwrap_or_default();
            anyhow::ensure!(!is_private, ErrorCode::UnsharedItem);
            let buffer_id = buffer.read(cx).remote_id();
            this.create_buffer_for_peer(&buffer, peer_id, cx)
                .detach_and_log_err(cx);
            Ok(proto::OpenBufferResponse {
                buffer_id: buffer_id.into(),
            })
        })
    }

    async fn handle_create_image_for_peer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CreateImageForPeer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.host
                .read(cx)
                .image_store
                .clone()
                .update(cx, |image_store, cx| {
                    image_store.handle_create_image_for_peer(envelope, cx)
                })
        })
    }

    async fn handle_create_file_for_peer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CreateFileForPeer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        use proto::create_file_for_peer::Variant;
        log::debug!("handle_create_file_for_peer: received message");

        let downloading_files: Arc<Mutex<HashMap<(WorktreeId, String), DownloadingFile>>> =
            this.update(&mut cx, |this, _| this.downloading_files.clone());

        match &envelope.payload.variant {
            Some(Variant::State(state)) => {
                log::debug!(
                    "handle_create_file_for_peer: got State: id={}, content_size={}",
                    state.id,
                    state.content_size
                );

                // Extract worktree_id and path from the File field
                if let Some(ref file) = state.file {
                    let worktree_id = WorktreeId::from_proto(file.worktree_id);
                    let path = file.path.clone();
                    let key = (worktree_id, path);
                    log::debug!("handle_create_file_for_peer: looking up key={:?}", key);

                    let empty_file_destination: Option<PathBuf> = {
                        let mut files = downloading_files.lock();
                        log::trace!(
                            "handle_create_file_for_peer: current downloading_files keys: {:?}",
                            files.keys().collect::<Vec<_>>()
                        );

                        if let Some(file_entry) = files.get_mut(&key) {
                            file_entry.total_size = state.content_size;
                            file_entry.file_id = Some(state.id);
                            log::debug!(
                                "handle_create_file_for_peer: updated file entry: total_size={}, file_id={}",
                                state.content_size,
                                state.id
                            );
                        } else {
                            log::warn!(
                                "handle_create_file_for_peer: key={:?} not found in downloading_files",
                                key
                            );
                        }

                        if state.content_size == 0 {
                            // No chunks will arrive for an empty file; write it now.
                            files.remove(&key).map(|entry| entry.destination_path)
                        } else {
                            None
                        }
                    };

                    if let Some(destination) = empty_file_destination {
                        log::debug!(
                            "handle_create_file_for_peer: writing empty file to {:?}",
                            destination
                        );
                        match smol::fs::write(&destination, &[] as &[u8]).await {
                            Ok(_) => log::info!(
                                "handle_create_file_for_peer: successfully wrote file to {:?}",
                                destination
                            ),
                            Err(e) => log::error!(
                                "handle_create_file_for_peer: failed to write empty file: {:?}",
                                e
                            ),
                        }
                    }
                } else {
                    log::warn!("handle_create_file_for_peer: State has no file field");
                }
            }
            Some(Variant::Chunk(chunk)) => {
                log::debug!(
                    "handle_create_file_for_peer: got Chunk: file_id={}, data_len={}",
                    chunk.file_id,
                    chunk.data.len()
                );

                // Extract data while holding the lock, then release it before await
                let (key_to_remove, write_info): (
                    Option<(WorktreeId, String)>,
                    Option<(PathBuf, Vec<u8>)>,
                ) = {
                    let mut files = downloading_files.lock();
                    let mut found_key: Option<(WorktreeId, String)> = None;
                    let mut write_data: Option<(PathBuf, Vec<u8>)> = None;

                    for (key, file_entry) in files.iter_mut() {
                        if file_entry.file_id == Some(chunk.file_id) {
                            file_entry.chunks.extend_from_slice(&chunk.data);
                            log::debug!(
                                "handle_create_file_for_peer: accumulated {} bytes, total_size={}",
                                file_entry.chunks.len(),
                                file_entry.total_size
                            );

                            if file_entry.chunks.len() as u64 >= file_entry.total_size
                                && file_entry.total_size > 0
                            {
                                let destination = file_entry.destination_path.clone();
                                let content = std::mem::take(&mut file_entry.chunks);
                                found_key = Some(key.clone());
                                write_data = Some((destination, content));
                            }
                            break;
                        }
                    }
                    (found_key, write_data)
                }; // MutexGuard is dropped here

                // Perform the async write outside the lock
                if let Some((destination, content)) = write_info {
                    log::debug!(
                        "handle_create_file_for_peer: writing {} bytes to {:?}",
                        content.len(),
                        destination
                    );
                    match smol::fs::write(&destination, &content).await {
                        Ok(_) => log::info!(
                            "handle_create_file_for_peer: successfully wrote file to {:?}",
                            destination
                        ),
                        Err(e) => log::error!(
                            "handle_create_file_for_peer: failed to write file: {:?}",
                            e
                        ),
                    }
                }

                // Remove the completed entry
                if let Some(key) = key_to_remove {
                    downloading_files.lock().remove(&key);
                    log::debug!("handle_create_file_for_peer: removed completed download entry");
                }
            }
            None => {
                log::warn!("handle_create_file_for_peer: got None variant");
            }
        }

        Ok(())
    }

    fn synchronize_remote_buffers(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let project_id = match self.client_state {
            ProjectClientState::Collab {
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
                )));
            }
        };

        let client = self.collab_client.clone();
        cx.spawn(async move |this, cx| {
            let (buffers, incomplete_buffer_ids) = this.update(cx, |this, cx| {
                this.buffer_store(cx).read(cx).buffer_version_info(cx)
            })?;
            let response = client
                .request(proto::SynchronizeBuffers {
                    project_id,
                    buffers,
                })
                .await?;

            let send_updates_for_buffers = this.update(cx, |this, cx| {
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
                            cx.background_spawn(async move {
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
                cx.background_spawn(client.request(proto::OpenBufferById {
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

    pub fn worktree_metadata_protos(&self, cx: &App) -> Vec<proto::WorktreeMetadata> {
        // Use this Project's worktree order, not the host's. The host's
        // `WorktreeStore` is unordered (a multi-tenant set); the
        // metadata sent downstream represents *this* project's view.
        self.worktrees(cx)
            .map(|worktree| {
                let worktree = worktree.read(cx);
                proto::WorktreeMetadata {
                    id: worktree.id().to_proto(),
                    root_name: worktree.root_name_str().to_owned(),
                    visible: worktree.is_visible(),
                    abs_path: worktree.abs_path().to_string_lossy().into_owned(),
                    root_repo_common_dir: worktree
                        .root_repo_common_dir()
                        .map(|p| p.to_string_lossy().into_owned()),
                }
            })
            .collect()
    }

    /// Iterator of all open buffers that have unsaved changes
    pub fn dirty_buffers<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = ProjectPath> + 'a {
        let buffer_store = self.buffer_store(cx);
        self.buffers.iter().filter_map(move |id| {
            let buffer = buffer_store.read(cx).get(*id)?;
            let buf = buffer.read(cx);
            if buf.is_dirty() {
                buf.project_path(cx)
            } else {
                None
            }
        })
    }

    fn set_worktrees_from_proto(
        &mut self,
        worktrees: Vec<proto::WorktreeMetadata>,
        cx: &mut Context<Project>,
    ) -> Result<()> {
        let replica_id = self.replica_id(cx);
        self.worktree_store(cx).update(cx, |worktree_store, cx| {
            worktree_store.set_worktrees_from_proto(worktrees.clone(), replica_id, cx)
        })?;

        // Rebuild `self.worktrees` to match the proto's order, dropping
        // entries that no longer exist on the host. The shared
        // WorktreeStore may still hold worktrees from sibling Projects;
        // we only update *this* project's view.
        let kept_ids: HashSet<WorktreeId> = worktrees
            .iter()
            .map(|w| WorktreeId::from_proto(w.id))
            .collect();
        let mut existing_by_id: HashMap<WorktreeId, WorktreeHandle> = self
            .worktrees
            .drain(..)
            .filter_map(|handle| {
                let id = handle.upgrade()?.read(cx).id();
                if kept_ids.contains(&id) {
                    Some((id, handle))
                } else {
                    None
                }
            })
            .collect();
        let worktree_store = self.worktree_store(cx);
        for proto_worktree in &worktrees {
            let id = WorktreeId::from_proto(proto_worktree.id);
            if let Some(handle) = existing_by_id.remove(&id) {
                self.worktrees.push(handle);
            } else if let Some(worktree) = worktree_store.read(cx).worktree_for_id(id, cx) {
                // Newly added: pin it on the project side. Visible
                // worktrees are always strong; invisible follow
                // `retain_worktrees`.
                let push_strong = self.retain_worktrees || worktree.read(cx).is_visible();
                self.worktrees.push(if push_strong {
                    WorktreeHandle::Strong(worktree)
                } else {
                    WorktreeHandle::Weak(worktree.downgrade())
                });
            }
        }
        Ok(())
    }

    fn set_collaborators_from_proto(
        &mut self,
        messages: Vec<proto::Collaborator>,
        cx: &mut Context<Self>,
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

    pub fn supplementary_language_servers<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + Iterator<Item = (LanguageServerId, LanguageServerName)> {
        self.lsp_store(cx).read(cx).supplementary_language_servers()
    }

    pub fn any_language_server_supports_inlay_hints(&self, buffer: &Buffer, cx: &mut App) -> bool {
        let Some(language) = buffer.language().cloned() else {
            return false;
        };
        self.lsp_store(cx).update(cx, |lsp_store, _| {
            let relevant_language_servers = lsp_store
                .languages
                .lsp_adapters(&language.name())
                .into_iter()
                .map(|lsp_adapter| lsp_adapter.name())
                .collect::<HashSet<_>>();
            lsp_store
                .language_server_statuses()
                .filter_map(|(server_id, server_status)| {
                    relevant_language_servers
                        .contains(&server_status.name)
                        .then_some(server_id)
                })
                .filter_map(|server_id| lsp_store.lsp_server_capabilities.get(&server_id))
                .any(InlayHints::check_capabilities)
        })
    }

    pub fn any_language_server_supports_semantic_tokens(
        &self,
        buffer: &Buffer,
        cx: &mut App,
    ) -> bool {
        let Some(language) = buffer.language().cloned() else {
            return false;
        };
        let lsp_store = self.lsp_store(cx).read(cx);
        let relevant_language_servers = lsp_store
            .languages
            .lsp_adapters(&language.name())
            .into_iter()
            .map(|lsp_adapter| lsp_adapter.name())
            .collect::<HashSet<_>>();
        lsp_store
            .language_server_statuses()
            .filter_map(|(server_id, server_status)| {
                relevant_language_servers
                    .contains(&server_status.name)
                    .then_some(server_id)
            })
            .filter_map(|server_id| lsp_store.lsp_server_capabilities.get(&server_id))
            .any(|capabilities| capabilities.semantic_tokens_provider.is_some())
    }

    pub fn language_server_id_for_name(
        &self,
        buffer: &Buffer,
        name: &LanguageServerName,
        cx: &App,
    ) -> Option<LanguageServerId> {
        let language = buffer.language()?;
        let relevant_language_servers = self
            .languages
            .lsp_adapters(&language.name())
            .into_iter()
            .map(|lsp_adapter| lsp_adapter.name())
            .collect::<HashSet<_>>();
        if !relevant_language_servers.contains(name) {
            return None;
        }
        self.language_server_statuses(cx)
            .filter(|(_, server_status)| relevant_language_servers.contains(&server_status.name))
            .find_map(|(server_id, server_status)| {
                if &server_status.name == name {
                    Some(server_id)
                } else {
                    None
                }
            })
    }

    #[cfg(feature = "test-support")]
    pub fn has_language_servers_for(&self, buffer: &Buffer, cx: &mut App) -> bool {
        self.lsp_store(cx).update(cx, |this, cx| {
            this.running_language_servers_for_local_buffer(buffer, cx)
                .next()
                .is_some()
        })
    }

    pub fn git_init(
        &self,
        path: Arc<Path>,
        fallback_branch_name: String,
        cx: &App,
    ) -> Task<Result<()>> {
        self.git_store(cx)
            .read(cx)
            .git_init(path, fallback_branch_name, cx)
    }

    pub fn git_config(&self, path: Arc<Path>, args: Vec<String>, cx: &App) -> Task<Result<String>> {
        self.git_store(cx).read(cx).git_config(path, args, cx)
    }

    // todo! this should have multi tenet tests
    pub fn buffer_store(&self, cx: &App) -> Entity<BufferStore> {
        self.host.read(cx).buffer_store.clone()
    }

    // todo! this should have multi tenet tests
    pub fn git_store(&self, cx: &App) -> Entity<GitStore> {
        self.host.read(cx).git_store.clone()
    }

    // todo! this should have multi tenet tests
    pub fn agent_server_store(&self, cx: &App) -> Entity<AgentServerStore> {
        self.host.read(cx).agent_server_store.clone()
    }

    #[cfg(feature = "test-support")]
    pub fn git_scans_complete(&self, cx: &Context<Self>) -> Task<()> {
        use futures::future::join_all;
        cx.spawn(async move |this, cx| {
            let scans_complete = this
                .read_with(cx, |this, cx| {
                    this.worktrees(cx)
                        .filter_map(|worktree| Some(worktree.read(cx).as_local()?.scan_complete()))
                        .collect::<Vec<_>>()
                })
                .unwrap();
            join_all(scans_complete).await;
            let barriers = this
                .update(cx, |this, cx| {
                    let repos = this.repositories(cx).values().cloned().collect::<Vec<_>>();
                    repos
                        .into_iter()
                        .map(|repo| repo.update(cx, |repo, _| repo.barrier()))
                        .collect::<Vec<_>>()
                })
                .unwrap();
            join_all(barriers).await;
        })
    }

    pub fn active_repository_id(&self) -> Option<RepositoryId> {
        self.active_repository_id
    }

    pub fn active_repository(&self, cx: &App) -> Option<Entity<Repository>> {
        let active_repository_id = self.active_repository_id?;
        if !self.repositories.contains(&active_repository_id) {
            return None;
        }
        self.git_store(cx)
            .read(cx)
            .repositories()
            .get(&active_repository_id)
            .cloned()
    }

    pub fn set_active_repository_id(
        &mut self,
        repository_id: Option<RepositoryId>,
        cx: &mut Context<Self>,
    ) {
        let repository_id = repository_id.filter(|id| self.repositories.contains(id));
        if self.active_repository_id != repository_id {
            self.active_repository_id = repository_id;
            cx.emit(Event::ActiveRepositoryChanged(repository_id));
            cx.notify();
        }
    }

    pub fn set_active_repository(
        &mut self,
        repository: &Entity<Repository>,
        cx: &mut Context<Self>,
    ) {
        let repository_id = repository.read(cx).id;
        self.set_active_repository_id(Some(repository_id), cx);
    }

    pub fn set_active_repository_for_path(
        &mut self,
        project_path: &ProjectPath,
        cx: &mut Context<Self>,
    ) {
        let repository_id = self
            .git_store(cx)
            .read(cx)
            .repository_and_path_for_project_path(project_path, cx)
            .map(|(repository, _)| repository.read(cx).id);
        self.set_active_repository_id(repository_id, cx);
    }

    pub fn set_active_repository_for_worktree(
        &mut self,
        worktree_id: WorktreeId,
        cx: &mut Context<Self>,
    ) {
        let Some(worktree) = self.worktree_for_id(worktree_id, cx) else {
            return;
        };
        let worktree_abs_path = worktree.read(cx).abs_path();
        let repository_id = self
            .repositories(cx)
            .values()
            .filter(|repo| {
                let repo_path = &repo.read(cx).work_directory_abs_path;
                *repo_path == worktree_abs_path || worktree_abs_path.starts_with(repo_path.as_ref())
            })
            .max_by_key(|repo| repo.read(cx).work_directory_abs_path.as_os_str().len())
            .map(|repo| repo.read(cx).id);
        self.set_active_repository_id(repository_id, cx);
    }

    fn next_active_repository_id(&self, cx: &App) -> Option<RepositoryId> {
        self.repositories(cx)
            .into_iter()
            .sorted_by(|(_, left), (_, right)| {
                left.read(cx)
                    .work_directory_abs_path
                    .cmp(&right.read(cx).work_directory_abs_path)
            })
            .map(|(repository_id, _)| repository_id)
            .next()
    }

    pub fn repositories(&self, cx: &App) -> HashMap<RepositoryId, Entity<Repository>> {
        // Filter the host store's repository map through `self.repositories`
        // so a shared GitStore in Phase 2 doesn't leak sibling-Project
        // repositories through this accessor.
        let host = self.git_store(cx);
        let host = host.read(cx);
        let host_repos = host.repositories();
        self.repositories
            .iter()
            .filter_map(|id| host_repos.get(id).map(|repo| (*id, repo.clone())))
            .collect()
    }

    pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus> {
        self.git_store(cx)
            .read(cx)
            .status_for_buffer_id(buffer_id, cx)
    }

    pub fn set_agent_location(
        &mut self,
        new_location: Option<AgentLocation>,
        cx: &mut Context<Self>,
    ) {
        if let Some(old_location) = self.agent_location.as_ref() {
            old_location
                .buffer
                .update(cx, |buffer, cx| buffer.remove_agent_selections(cx))
                .ok();
        }

        if let Some(location) = new_location.as_ref() {
            location
                .buffer
                .update(cx, |buffer, cx| {
                    buffer.set_agent_selections(
                        Arc::from([language::Selection {
                            id: 0,
                            start: location.position,
                            end: location.position,
                            reversed: false,
                            goal: language::SelectionGoal::None,
                        }]),
                        false,
                        CursorShape::Hollow,
                        cx,
                    )
                })
                .ok();
        }

        self.agent_location = new_location;
        cx.emit(Event::AgentLocationChanged);
    }

    pub fn agent_location(&self) -> Option<AgentLocation> {
        self.agent_location.clone()
    }

    pub fn path_style(&self, cx: &App) -> PathStyle {
        self.worktree_store(cx).read(cx).path_style()
    }

    pub fn contains_local_settings_file(
        &self,
        worktree_id: WorktreeId,
        rel_path: &RelPath,
        cx: &App,
    ) -> bool {
        self.worktree_for_id(worktree_id, cx)
            .map_or(false, |worktree| {
                worktree.read(cx).entry_for_path(rel_path).is_some()
            })
    }

    pub fn worktree_paths(&self, cx: &App) -> WorktreePaths {
        // Compute paths from *this* Project's visible worktrees, not
        // the host's full set: in Phase 2 sharing the host `WorktreeStore`
        // may contain sibling Projects' worktrees, and we'd otherwise
        // surface their roots through every key (e.g.
        // `project_group_key`) that flows through `worktree_paths`.
        let (mains, folders): (Vec<PathBuf>, Vec<PathBuf>) = self
            .visible_worktrees(cx)
            .map(|worktree| {
                let snapshot = worktree.read(cx).snapshot();
                let folder_path = snapshot.abs_path().to_path_buf();
                let main_path = snapshot
                    .root_repo_common_dir()
                    .map(|dir| git_store::repo_identity_path(dir).to_path_buf())
                    .unwrap_or_else(|| folder_path.clone());
                (main_path, folder_path)
            })
            .unzip();

        WorktreePaths::from_path_lists(PathList::new(&mains), PathList::new(&folders))
            .expect("main and folder path lists are built from the same iteration")
    }

    pub fn project_group_key(&self, cx: &App) -> ProjectGroupKey {
        ProjectGroupKey::from_project(self, cx)
    }
}

/// Identifies a project group by a set of paths the workspaces in this group
/// have.
///
/// Paths are mapped to their main worktree path first so we can group
/// workspaces by main repos.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct ProjectGroupKey {
    /// The paths of the main worktrees for this project group.
    paths: PathList,
    host: Option<RemoteConnectionOptions>,
}

impl ProjectGroupKey {
    /// Creates a new `ProjectGroupKey` with the given path list.
    ///
    /// The path list should point to the git main worktree paths for a project.
    pub fn new(host: Option<RemoteConnectionOptions>, paths: PathList) -> Self {
        Self { paths, host }
    }

    pub fn from_project(project: &Project, cx: &App) -> Self {
        let paths = project.worktree_paths(cx);
        let host = project.remote_connection_options(cx);
        Self {
            paths: paths.main_worktree_path_list().clone(),
            host,
        }
    }

    pub fn from_worktree_paths(
        paths: &WorktreePaths,
        host: Option<RemoteConnectionOptions>,
    ) -> Self {
        Self {
            paths: paths.main_worktree_path_list().clone(),
            host,
        }
    }

    pub fn path_list(&self) -> &PathList {
        &self.paths
    }

    pub fn display_name(
        &self,
        path_detail_map: &std::collections::HashMap<PathBuf, usize>,
    ) -> SharedString {
        let mut names = Vec::with_capacity(self.paths.paths().len());
        for abs_path in self.paths.ordered_paths() {
            let detail = path_detail_map.get(abs_path).copied().unwrap_or(0);
            // Strip a `.git` extension for display (bare clones like `foo.git`
            // should display as `foo`, matching the titlebar).
            let display_path = if abs_path.extension() == Some(std::ffi::OsStr::new("git")) {
                std::borrow::Cow::Owned(abs_path.with_extension(""))
            } else {
                std::borrow::Cow::Borrowed(abs_path.as_path())
            };
            let suffix = path_suffix(&display_path, detail);
            if !suffix.is_empty() {
                names.push(suffix);
            }
        }
        if names.is_empty() {
            "Empty Workspace".into()
        } else {
            names.join(", ").into()
        }
    }

    pub fn host(&self) -> Option<RemoteConnectionOptions> {
        self.host.clone()
    }

    pub fn matches(&self, other: &ProjectGroupKey) -> bool {
        self.paths == other.paths
            && same_remote_connection_identity(self.host.as_ref(), other.host.as_ref())
    }
}

pub fn path_suffix(path: &Path, detail: usize) -> String {
    let mut components: Vec<_> = path
        .components()
        .rev()
        .filter_map(|component| match component {
            std::path::Component::Normal(s) => Some(s.to_string_lossy()),
            _ => None,
        })
        .take(detail + 1)
        .collect();
    components.reverse();
    components.join("/")
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

    fn prefix(&self) -> Arc<RelPath> {
        if self.snapshot.root_entry().is_some_and(|e| e.is_file()) || self.include_root_name {
            self.snapshot.root_name().into()
        } else {
            RelPath::empty().into()
        }
    }

    fn root_is_file(&self) -> bool {
        self.snapshot.root_entry().is_some_and(|f| f.is_file())
    }

    fn path_style(&self) -> PathStyle {
        self.snapshot.path_style()
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

impl<'a> fuzzy_nucleo::PathMatchCandidateSet<'a> for PathMatchCandidateSet {
    type Candidates = PathMatchCandidateSetNucleoIter<'a>;
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
    fn prefix(&self) -> Arc<RelPath> {
        if self.snapshot.root_entry().is_some_and(|e| e.is_file()) || self.include_root_name {
            self.snapshot.root_name().into()
        } else {
            RelPath::empty().into()
        }
    }
    fn root_is_file(&self) -> bool {
        self.snapshot.root_entry().is_some_and(|f| f.is_file())
    }
    fn path_style(&self) -> PathStyle {
        self.snapshot.path_style()
    }
    fn candidates(&'a self, start: usize) -> Self::Candidates {
        PathMatchCandidateSetNucleoIter {
            traversal: match self.candidates {
                Candidates::Directories => self.snapshot.directories(self.include_ignored, start),
                Candidates::Files => self.snapshot.files(self.include_ignored, start),
                Candidates::Entries => self.snapshot.entries(self.include_ignored, start),
            },
        }
    }
}

pub struct PathMatchCandidateSetNucleoIter<'a> {
    traversal: Traversal<'a>,
}

impl<'a> Iterator for PathMatchCandidateSetNucleoIter<'a> {
    type Item = fuzzy_nucleo::PathMatchCandidate<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.traversal
            .next()
            .map(|entry| fuzzy_nucleo::PathMatchCandidate {
                is_dir: entry.kind.is_dir(),
                path: &entry.path,
                char_bag: entry.char_bag,
            })
    }
}

impl EventEmitter<Event> for Project {}

impl PeerBufferAccess for Project {
    fn create_buffer_for_peer(
        &mut self,
        buffer: &Entity<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> Task<Result<()>> {
        Project::create_buffer_for_peer(self, buffer, peer_id, cx)
    }

    fn serialize_project_transaction_for_peer(
        &mut self,
        project_transaction: ProjectTransaction,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> proto::ProjectTransaction {
        Project::serialize_project_transaction_for_peer(self, project_transaction, peer_id, cx)
    }
}

impl<'a> From<&'a ProjectPath> for SettingsLocation<'a> {
    fn from(val: &'a ProjectPath) -> Self {
        SettingsLocation {
            worktree_id: val.worktree_id,
            path: val.path.as_ref(),
        }
    }
}

impl<P: Into<Arc<RelPath>>> From<(WorktreeId, P)> for ProjectPath {
    fn from((worktree_id, path): (WorktreeId, P)) -> Self {
        Self {
            worktree_id,
            path: path.into(),
        }
    }
}

/// ResolvedPath is a path that has been resolved to either a ProjectPath
/// or an AbsPath and that *exists*.
#[derive(Debug, Clone)]
pub enum ResolvedPath {
    ProjectPath {
        project_path: ProjectPath,
        is_dir: bool,
    },
    AbsPath {
        path: String,
        is_dir: bool,
    },
}

impl ResolvedPath {
    pub fn abs_path(&self) -> Option<&str> {
        match self {
            Self::AbsPath { path, .. } => Some(path),
            _ => None,
        }
    }

    pub fn into_abs_path(self) -> Option<String> {
        match self {
            Self::AbsPath { path, .. } => Some(path),
            _ => None,
        }
    }

    pub fn project_path(&self) -> Option<&ProjectPath> {
        match self {
            Self::ProjectPath { project_path, .. } => Some(project_path),
            _ => None,
        }
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub fn is_dir(&self) -> bool {
        match self {
            Self::ProjectPath { is_dir, .. } => *is_dir,
            Self::AbsPath { is_dir, .. } => *is_dir,
        }
    }
}

impl ProjectItem for Buffer {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        Some(project.update(cx, |project, cx| project.open_buffer(path.clone(), cx)))
    }

    fn entry_id(&self, _cx: &App) -> Option<ProjectEntryId> {
        File::from_dyn(self.file()).and_then(|file| file.project_entry_id())
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        let file = self.file()?;

        (!matches!(file.disk_state(), DiskState::Historic { .. })).then(|| ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty()
    }
}

impl Completion {
    pub fn kind(&self) -> Option<CompletionItemKind> {
        self.source
            // `lsp::CompletionListItemDefaults` has no `kind` field
            .lsp_completion(false)
            .and_then(|lsp_completion| lsp_completion.kind)
    }

    pub fn label(&self) -> Option<String> {
        self.source
            .lsp_completion(false)
            .map(|lsp_completion| lsp_completion.label.clone())
    }

    /// A key that can be used to sort completions when displaying
    /// them to the user.
    pub fn sort_key(&self) -> (usize, &str) {
        const DEFAULT_KIND_KEY: usize = 4;
        let kind_key = self
            .kind()
            .and_then(|lsp_completion_kind| match lsp_completion_kind {
                lsp::CompletionItemKind::KEYWORD => Some(0),
                lsp::CompletionItemKind::VARIABLE => Some(1),
                lsp::CompletionItemKind::CONSTANT => Some(2),
                lsp::CompletionItemKind::PROPERTY => Some(3),
                _ => None,
            })
            .unwrap_or(DEFAULT_KIND_KEY);
        (kind_key, self.label.filter_text())
    }

    /// Whether this completion is a snippet.
    pub fn is_snippet_kind(&self) -> bool {
        matches!(
            &self.source,
            CompletionSource::Lsp { lsp_completion, .. }
            if lsp_completion.kind == Some(CompletionItemKind::SNIPPET)
        )
    }

    /// Whether this completion is a snippet or snippet-style LSP completion.
    pub fn is_snippet(&self) -> bool {
        self.source
            // `lsp::CompletionListItemDefaults` has `insert_text_format` field
            .lsp_completion(true)
            .is_some_and(|lsp_completion| {
                lsp_completion.insert_text_format == Some(lsp::InsertTextFormat::SNIPPET)
            })
    }

    /// Returns the corresponding color for this completion.
    ///
    /// Will return `None` if this completion's kind is not [`CompletionItemKind::COLOR`].
    pub fn color(&self) -> Option<Hsla> {
        // `lsp::CompletionListItemDefaults` has no `kind` field
        let lsp_completion = self.source.lsp_completion(false)?;
        if lsp_completion.kind? == CompletionItemKind::COLOR {
            return color_extractor::extract_color(&lsp_completion);
        }
        None
    }
}

fn proto_to_prompt(level: proto::language_server_prompt_request::Level) -> gpui::PromptLevel {
    match level {
        proto::language_server_prompt_request::Level::Info(_) => gpui::PromptLevel::Info,
        proto::language_server_prompt_request::Level::Warning(_) => gpui::PromptLevel::Warning,
        proto::language_server_prompt_request::Level::Critical(_) => gpui::PromptLevel::Critical,
    }
}

fn provide_inline_values(
    captures: impl Iterator<Item = (Range<usize>, language::DebuggerTextObject)>,
    snapshot: &language::BufferSnapshot,
    max_row: usize,
) -> Vec<InlineValueLocation> {
    let mut variables = Vec::new();
    let mut variable_position = HashSet::default();
    let mut scopes = Vec::new();

    let active_debug_line_offset = snapshot.point_to_offset(Point::new(max_row as u32, 0));

    for (capture_range, capture_kind) in captures {
        match capture_kind {
            language::DebuggerTextObject::Variable => {
                let variable_name = snapshot
                    .text_for_range(capture_range.clone())
                    .collect::<String>();
                let point = snapshot.offset_to_point(capture_range.end);

                while scopes
                    .last()
                    .is_some_and(|scope: &Range<_>| !scope.contains(&capture_range.start))
                {
                    scopes.pop();
                }

                if point.row as usize > max_row {
                    break;
                }

                let scope = if scopes
                    .last()
                    .is_none_or(|scope| !scope.contains(&active_debug_line_offset))
                {
                    VariableScope::Global
                } else {
                    VariableScope::Local
                };

                if variable_position.insert(capture_range.end) {
                    variables.push(InlineValueLocation {
                        variable_name,
                        scope,
                        lookup: VariableLookupKind::Variable,
                        row: point.row as usize,
                        column: point.column as usize,
                    });
                }
            }
            language::DebuggerTextObject::Scope => {
                while scopes.last().map_or_else(
                    || false,
                    |scope: &Range<usize>| {
                        !(scope.contains(&capture_range.start)
                            && scope.contains(&capture_range.end))
                    },
                ) {
                    scopes.pop();
                }
                scopes.push(capture_range);
            }
        }
    }

    variables
}
