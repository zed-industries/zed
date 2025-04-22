pub mod buffer_store;
mod color_extractor;
pub mod connection_manager;
pub mod debounced_delay;
pub mod debugger;
pub mod git_store;
pub mod image_store;
pub mod lsp_command;
pub mod lsp_store;
mod manifest_tree;
pub mod prettier_store;
pub mod project_settings;
pub mod search;
mod task_inventory;
pub mod task_store;
pub mod terminals;
pub mod toolchain_store;
pub mod worktree_store;

#[cfg(test)]
mod project_tests;

mod direnv;
mod environment;
use buffer_diff::BufferDiff;
pub use environment::{EnvironmentErrorMessage, ProjectEnvironmentEvent};
use git_store::{Repository, RepositoryId};
use task::DebugTaskDefinition;
pub mod search_history;
mod yarn;

use crate::git_store::GitStore;
pub use git_store::git_traversal::{ChildEntriesGitIter, GitEntry, GitEntryRef, GitTraversal};

use anyhow::{Context as _, Result, anyhow};
use buffer_store::{BufferStore, BufferStoreEvent};
use client::{
    Client, Collaborator, PendingEntitySubscription, ProjectId, TypedEnvelope, UserStore, proto,
};
use clock::ReplicaId;

use dap::{
    adapters::{DebugAdapterBinary, TcpArguments},
    client::DebugAdapterClient,
};

use collections::{BTreeSet, HashMap, HashSet};
use debounced_delay::DebouncedDelay;
use debugger::{
    breakpoint_store::BreakpointStore,
    dap_store::{DapStore, DapStoreEvent},
    session::Session,
};
pub use environment::ProjectEnvironment;
#[cfg(test)]
use futures::future::join_all;
use futures::{
    StreamExt,
    channel::mpsc::{self, UnboundedReceiver},
    future::try_join_all,
};
pub use image_store::{ImageItem, ImageStore};
use image_store::{ImageItemEvent, ImageStoreEvent};

use ::git::{blame::Blame, status::FileStatus};
use gpui::{
    AnyEntity, App, AppContext, AsyncApp, BorrowAppContext, Context, Entity, EventEmitter, Hsla,
    SharedString, Task, WeakEntity, Window,
};
use itertools::Itertools;
use language::{
    Buffer, BufferEvent, Capability, CodeLabel, Language, LanguageName, LanguageRegistry,
    PointUtf16, ToOffset, ToPointUtf16, Toolchain, ToolchainList, Transaction, Unclipped,
    language_settings::InlayHintKind, proto::split_operations,
};
use lsp::{
    CodeActionKind, CompletionContext, CompletionItemKind, DocumentHighlightKind, InsertTextMode,
    LanguageServerId, LanguageServerName, MessageActionItem,
};
use lsp_command::*;
use lsp_store::{CompletionDocumentation, LspFormatTarget, OpenLspBufferHandle};
pub use manifest_tree::ManifestProviders;
use node_runtime::NodeRuntime;
use parking_lot::Mutex;
pub use prettier_store::PrettierStore;
use project_settings::{ProjectSettings, SettingsObserver, SettingsObserverEvent};
use remote::{SshConnectionOptions, SshRemoteClient};
use rpc::{
    AnyProtoClient, ErrorCode,
    proto::{FromProto, LanguageServerPromptResponse, SSH_PROJECT_ID, ToProto},
};
use search::{SearchInputKind, SearchQuery, SearchResult};
use search_history::SearchHistory;
use settings::{InvalidSettingsError, Settings, SettingsLocation, SettingsStore};
use smol::channel::Receiver;
use snippet::Snippet;
use snippet_provider::SnippetProvider;
use std::{
    borrow::Cow,
    net::Ipv4Addr,
    ops::Range,
    path::{Component, Path, PathBuf},
    pin::pin,
    str,
    sync::Arc,
    time::Duration,
};

use task_store::TaskStore;
use terminals::{SshCommand, Terminals, wrap_for_ssh};
use text::{Anchor, BufferId};
use toolchain_store::EmptyToolchainStore;
use util::{
    ResultExt as _,
    paths::{SanitizedPath, compare_paths},
};
use worktree::{CreatedEntry, Snapshot, Traversal};
pub use worktree::{
    Entry, EntryKind, FS_WATCH_LATENCY, File, LocalWorktree, PathChange, ProjectEntryId,
    UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree, WorktreeId, WorktreeSettings,
};
use worktree_store::{WorktreeStore, WorktreeStoreEvent};

pub use fs::*;
pub use language::Location;
#[cfg(any(test, feature = "test-support"))]
pub use prettier::FORMAT_SUFFIX as TEST_PRETTIER_FORMAT_SUFFIX;
pub use task_inventory::{
    BasicContextProvider, ContextProviderWithTasks, Inventory, TaskContexts, TaskSourceKind,
};

pub use buffer_store::ProjectTransaction;
pub use lsp_store::{
    DiagnosticSummary, LanguageServerLogType, LanguageServerProgress, LanguageServerPromptRequest,
    LanguageServerStatus, LanguageServerToQuery, LspStore, LspStoreEvent,
    SERVER_PROGRESS_THROTTLE_TIMEOUT,
};
pub use toolchain_store::ToolchainStore;
const MAX_PROJECT_SEARCH_HISTORY_SIZE: usize = 500;
const MAX_SEARCH_RESULT_FILES: usize = 5_000;
const MAX_SEARCH_RESULT_RANGES: usize = 10_000;

pub trait ProjectItem {
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
    active_entry: Option<ProjectEntryId>,
    buffer_ordered_messages_tx: mpsc::UnboundedSender<BufferOrderedMessage>,
    languages: Arc<LanguageRegistry>,
    dap_store: Entity<DapStore>,

    breakpoint_store: Entity<BreakpointStore>,
    client: Arc<client::Client>,
    join_project_response_message_id: u32,
    task_store: Entity<TaskStore>,
    user_store: Entity<UserStore>,
    fs: Arc<dyn Fs>,
    ssh_client: Option<Entity<SshRemoteClient>>,
    client_state: ProjectClientState,
    git_store: Entity<GitStore>,
    collaborators: HashMap<proto::PeerId, Collaborator>,
    client_subscriptions: Vec<client::Subscription>,
    worktree_store: Entity<WorktreeStore>,
    buffer_store: Entity<BufferStore>,
    image_store: Entity<ImageStore>,
    lsp_store: Entity<LspStore>,
    _subscriptions: Vec<gpui::Subscription>,
    buffers_needing_diff: HashSet<WeakEntity<Buffer>>,
    git_diff_debouncer: DebouncedDelay<Self>,
    remotely_created_models: Arc<Mutex<RemotelyCreatedModels>>,
    terminals: Terminals,
    node: Option<NodeRuntime>,
    search_history: SearchHistory,
    search_included_history: SearchHistory,
    search_excluded_history: SearchHistory,
    snippets: Entity<SnippetProvider>,
    environment: Entity<ProjectEnvironment>,
    settings_observer: Entity<SettingsObserver>,
    toolchain_store: Option<Entity<ToolchainStore>>,
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
    Remote {
        sharing_has_stopped: bool,
        capability: Capability,
        remote_id: u64,
        replica_id: ReplicaId,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    LanguageServerAdded(LanguageServerId, LanguageServerName, Option<WorktreeId>),
    LanguageServerRemoved(LanguageServerId),
    LanguageServerLog(LanguageServerId, LanguageServerLogType, String),
    Toast {
        notification_id: SharedString,
        message: String,
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
    DisconnectedFromSshRemote,
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
    RefreshInlayHints,
    RefreshCodeLens,
    RevealInProjectPanel(ProjectEntryId),
    SnippetEdit(BufferId, Vec<(lsp::Range, Snippet)>),
    ExpandedAllForEntry(WorktreeId, ProjectEntryId),
}

pub enum DebugAdapterClientState {
    Starting(Task<Option<Arc<DebugAdapterClient>>>),
    Running(Arc<DebugAdapterClient>),
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
            path: Arc::<Path>::from_proto(p.path),
        }
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
            path: Path::new("").into(),
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
    /// completion confirmations should run side effects.
    ///
    /// For LSP completions, will respect the setting `completions.lsp_insert_mode`.
    Complete,
    /// Similar to [Self::Complete], but behaves like `lsp_insert_mode` is set to `insert`.
    CompleteWithInsert,
    /// Similar to [Self::Complete], but behaves like `lsp_insert_mode` is set to `replace`.
    CompleteWithReplace,
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
    /// Whether to adjust indentation (the default) or not.
    pub insert_text_mode: Option<InsertTextMode>,
    /// An optional callback to invoke when this completion is confirmed.
    /// Returns, whether new completions should be retriggered after the current one.
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

    pub fn lsp_completion(&self, apply_defaults: bool) -> Option<Cow<lsp::CompletionItem>> {
        if let Self::Lsp {
            lsp_completion,
            lsp_defaults,
            ..
        } = self
        {
            if apply_defaults {
                if let Some(lsp_defaults) = lsp_defaults {
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
                                Some(
                                    lsp::CompletionListItemDefaultsEditRange::InsertAndReplace {
                                        insert,
                                        replace,
                                    },
                                ) => {
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

/// A generic completion that can come from different sources.
#[derive(Clone, Debug)]
pub(crate) struct CoreCompletion {
    replace_range: Range<Anchor>,
    new_text: String,
    source: CompletionSource,
}

/// A code action provided by a language server.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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

    fn action_kind(&self) -> Option<lsp::CodeActionKind> {
        match self {
            Self::Action(action) => action.kind.clone(),
            Self::Command(_) => Some(lsp::CodeActionKind::new("command")),
            Self::CodeLens(_) => Some(lsp::CodeActionKind::new("code lens")),
        }
    }

    fn edit(&self) -> Option<&lsp::WorkspaceEdit> {
        match self {
            Self::Action(action) => action.edit.as_ref(),
            Self::Command(_) => None,
            Self::CodeLens(_) => None,
        }
    }

    fn command(&self) -> Option<&lsp::Command> {
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
    pub source_language_server_id: LanguageServerId,
    pub path: ProjectPath,
    pub label: CodeLabel,
    pub name: String,
    pub kind: lsp::SymbolKind,
    pub range: Range<Unclipped<PointUtf16>>,
    pub signature: [u8; 32],
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
}

#[derive(Debug, Clone)]
pub struct DirectoryItem {
    pub path: PathBuf,
    pub is_dir: bool,
}

#[derive(Clone)]
pub enum DirectoryLister {
    Project(Entity<Project>),
    Local(Arc<dyn Fs>),
}

impl DirectoryLister {
    pub fn is_local(&self, cx: &App) -> bool {
        match self {
            DirectoryLister::Local(_) => true,
            DirectoryLister::Project(project) => project.read(cx).is_local(),
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
        if let DirectoryLister::Project(project) = self {
            if let Some(worktree) = project.read(cx).visible_worktrees(cx).next() {
                return worktree.read(cx).abs_path().to_string_lossy().to_string();
            }
        };
        format!("~{}", std::path::MAIN_SEPARATOR_STR)
    }

    pub fn list_directory(&self, path: String, cx: &mut App) -> Task<Result<Vec<DirectoryItem>>> {
        match self {
            DirectoryLister::Project(project) => {
                project.update(cx, |project, cx| project.list_directory(path, cx))
            }
            DirectoryLister::Local(fs) => {
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
}

#[cfg(any(test, feature = "test-support"))]
pub const DEFAULT_COMPLETION_CONTEXT: CompletionContext = CompletionContext {
    trigger_kind: lsp::CompletionTriggerKind::INVOKED,
    trigger_character: None,
};

impl Project {
    pub fn init_settings(cx: &mut App) {
        WorktreeSettings::register(cx);
        ProjectSettings::register(cx);
    }

    pub fn init(client: &Arc<Client>, cx: &mut App) {
        connection_manager::init(client.clone(), cx);
        Self::init_settings(cx);

        let client: AnyProtoClient = client.clone().into();
        client.add_entity_message_handler(Self::handle_add_collaborator);
        client.add_entity_message_handler(Self::handle_update_project_collaborator);
        client.add_entity_message_handler(Self::handle_remove_collaborator);
        client.add_entity_message_handler(Self::handle_update_project);
        client.add_entity_message_handler(Self::handle_unshare_project);
        client.add_entity_request_handler(Self::handle_update_buffer);
        client.add_entity_message_handler(Self::handle_update_worktree);
        client.add_entity_request_handler(Self::handle_synchronize_buffers);

        client.add_entity_request_handler(Self::handle_search_candidate_buffers);
        client.add_entity_request_handler(Self::handle_open_buffer_by_id);
        client.add_entity_request_handler(Self::handle_open_buffer_by_path);
        client.add_entity_request_handler(Self::handle_open_new_buffer);
        client.add_entity_message_handler(Self::handle_create_buffer_for_peer);

        WorktreeStore::init(&client);
        BufferStore::init(&client);
        LspStore::init(&client);
        GitStore::init(&client);
        SettingsObserver::init(&client);
        TaskStore::init(Some(&client));
        ToolchainStore::init(&client);
        DapStore::init(&client);
        BreakpointStore::init(&client);
    }

    pub fn local(
        client: Arc<Client>,
        node: NodeRuntime,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        env: Option<HashMap<String, String>>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            let (tx, rx) = mpsc::unbounded();
            cx.spawn(async move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx).await)
                .detach();
            let snippets = SnippetProvider::new(fs.clone(), BTreeSet::from_iter([]), cx);
            let worktree_store = cx.new(|_| WorktreeStore::local(false, fs.clone()));
            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();

            let environment = cx.new(|_| ProjectEnvironment::new(env));
            let toolchain_store = cx.new(|cx| {
                ToolchainStore::local(
                    languages.clone(),
                    worktree_store.clone(),
                    environment.clone(),
                    cx,
                )
            });

            let buffer_store = cx.new(|cx| BufferStore::local(worktree_store.clone(), cx));
            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();

            let breakpoint_store =
                cx.new(|_| BreakpointStore::local(worktree_store.clone(), buffer_store.clone()));

            let dap_store = cx.new(|cx| {
                DapStore::new_local(
                    client.http_client(),
                    node.clone(),
                    fs.clone(),
                    languages.clone(),
                    environment.clone(),
                    toolchain_store.read(cx).as_language_toolchain_store(),
                    worktree_store.clone(),
                    breakpoint_store.clone(),
                    cx,
                )
            });
            cx.subscribe(&dap_store, Self::on_dap_store_event).detach();

            let image_store = cx.new(|cx| ImageStore::local(worktree_store.clone(), cx));
            cx.subscribe(&image_store, Self::on_image_store_event)
                .detach();

            let prettier_store = cx.new(|cx| {
                PrettierStore::new(
                    node.clone(),
                    fs.clone(),
                    languages.clone(),
                    worktree_store.clone(),
                    cx,
                )
            });

            let task_store = cx.new(|cx| {
                TaskStore::local(
                    buffer_store.downgrade(),
                    worktree_store.clone(),
                    toolchain_store.read(cx).as_language_toolchain_store(),
                    environment.clone(),
                    cx,
                )
            });

            let settings_observer = cx.new(|cx| {
                SettingsObserver::new_local(
                    fs.clone(),
                    worktree_store.clone(),
                    task_store.clone(),
                    cx,
                )
            });
            cx.subscribe(&settings_observer, Self::on_settings_observer_event)
                .detach();

            let lsp_store = cx.new(|cx| {
                LspStore::new_local(
                    buffer_store.clone(),
                    worktree_store.clone(),
                    prettier_store.clone(),
                    toolchain_store.clone(),
                    environment.clone(),
                    languages.clone(),
                    client.http_client(),
                    fs.clone(),
                    cx,
                )
            });

            let git_store = cx.new(|cx| {
                GitStore::local(
                    &worktree_store,
                    buffer_store.clone(),
                    environment.clone(),
                    fs.clone(),
                    cx,
                )
            });

            cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();

            Self {
                buffer_ordered_messages_tx: tx,
                collaborators: Default::default(),
                worktree_store,
                buffer_store,
                image_store,
                lsp_store,
                join_project_response_message_id: 0,
                client_state: ProjectClientState::Local,
                git_store,
                client_subscriptions: Vec::new(),
                _subscriptions: vec![cx.on_release(Self::release)],
                active_entry: None,
                snippets,
                languages,
                client,
                task_store,
                user_store,
                settings_observer,
                fs,
                ssh_client: None,
                breakpoint_store,
                dap_store,

                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                node: Some(node),
                search_history: Self::new_search_history(),
                environment,
                remotely_created_models: Default::default(),

                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),

                toolchain_store: Some(toolchain_store),
            }
        })
    }

    pub fn ssh(
        ssh: Entity<SshRemoteClient>,
        client: Arc<Client>,
        node: NodeRuntime,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx: &mut Context<Self>| {
            let (tx, rx) = mpsc::unbounded();
            cx.spawn(async move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx).await)
                .detach();
            let global_snippets_dir = paths::config_dir().join("snippets");
            let snippets =
                SnippetProvider::new(fs.clone(), BTreeSet::from_iter([global_snippets_dir]), cx);

            let ssh_proto = ssh.read(cx).proto_client();
            let worktree_store =
                cx.new(|_| WorktreeStore::remote(false, ssh_proto.clone(), SSH_PROJECT_ID));
            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();

            let buffer_store = cx.new(|cx| {
                BufferStore::remote(
                    worktree_store.clone(),
                    ssh.read(cx).proto_client(),
                    SSH_PROJECT_ID,
                    cx,
                )
            });
            let image_store = cx.new(|cx| {
                ImageStore::remote(
                    worktree_store.clone(),
                    ssh.read(cx).proto_client(),
                    SSH_PROJECT_ID,
                    cx,
                )
            });
            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();
            let toolchain_store = cx
                .new(|cx| ToolchainStore::remote(SSH_PROJECT_ID, ssh.read(cx).proto_client(), cx));
            let task_store = cx.new(|cx| {
                TaskStore::remote(
                    buffer_store.downgrade(),
                    worktree_store.clone(),
                    toolchain_store.read(cx).as_language_toolchain_store(),
                    ssh.read(cx).proto_client(),
                    SSH_PROJECT_ID,
                    cx,
                )
            });

            let settings_observer = cx.new(|cx| {
                SettingsObserver::new_remote(
                    fs.clone(),
                    worktree_store.clone(),
                    task_store.clone(),
                    cx,
                )
            });
            cx.subscribe(&settings_observer, Self::on_settings_observer_event)
                .detach();

            let environment = cx.new(|_| ProjectEnvironment::new(None));

            let lsp_store = cx.new(|cx| {
                LspStore::new_remote(
                    buffer_store.clone(),
                    worktree_store.clone(),
                    Some(toolchain_store.clone()),
                    languages.clone(),
                    ssh_proto.clone(),
                    SSH_PROJECT_ID,
                    fs.clone(),
                    cx,
                )
            });
            cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();

            let breakpoint_store =
                cx.new(|_| BreakpointStore::remote(SSH_PROJECT_ID, ssh_proto.clone()));

            let dap_store = cx.new(|cx| {
                DapStore::new_ssh(
                    SSH_PROJECT_ID,
                    ssh_proto.clone(),
                    breakpoint_store.clone(),
                    cx,
                )
            });

            let git_store = cx.new(|cx| {
                GitStore::ssh(&worktree_store, buffer_store.clone(), ssh_proto.clone(), cx)
            });

            cx.subscribe(&ssh, Self::on_ssh_event).detach();

            let this = Self {
                buffer_ordered_messages_tx: tx,
                collaborators: Default::default(),
                worktree_store,
                buffer_store,
                image_store,
                lsp_store,
                breakpoint_store,
                dap_store,
                join_project_response_message_id: 0,
                client_state: ProjectClientState::Local,
                git_store,
                client_subscriptions: Vec::new(),
                _subscriptions: vec![
                    cx.on_release(Self::release),
                    cx.on_app_quit(|this, cx| {
                        let shutdown = this.ssh_client.take().and_then(|client| {
                            client
                                .read(cx)
                                .shutdown_processes(Some(proto::ShutdownRemoteServer {}))
                        });

                        cx.background_executor().spawn(async move {
                            if let Some(shutdown) = shutdown {
                                shutdown.await;
                            }
                        })
                    }),
                ],
                active_entry: None,
                snippets,
                languages,
                client,
                task_store,
                user_store,
                settings_observer,
                fs,
                ssh_client: Some(ssh.clone()),
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                node: Some(node),
                search_history: Self::new_search_history(),
                environment,
                remotely_created_models: Default::default(),

                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),

                toolchain_store: Some(toolchain_store),
            };

            // ssh -> local machine handlers
            let ssh = ssh.read(cx);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &cx.entity());
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.buffer_store);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.worktree_store);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.lsp_store);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.dap_store);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.settings_observer);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.git_store);

            ssh_proto.add_entity_message_handler(Self::handle_create_buffer_for_peer);
            ssh_proto.add_entity_message_handler(Self::handle_update_worktree);
            ssh_proto.add_entity_message_handler(Self::handle_update_project);
            ssh_proto.add_entity_message_handler(Self::handle_toast);
            ssh_proto.add_entity_request_handler(Self::handle_language_server_prompt_request);
            ssh_proto.add_entity_message_handler(Self::handle_hide_toast);
            ssh_proto.add_entity_request_handler(Self::handle_update_buffer_from_ssh);
            BufferStore::init(&ssh_proto);
            LspStore::init(&ssh_proto);
            SettingsObserver::init(&ssh_proto);
            TaskStore::init(Some(&ssh_proto));
            ToolchainStore::init(&ssh_proto);
            DapStore::init(&ssh_proto);
            GitStore::init(&ssh_proto);

            this
        })
    }

    pub async fn remote(
        remote_id: u64,
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncApp,
    ) -> Result<Entity<Self>> {
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
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncApp,
    ) -> Result<Entity<Self>> {
        client.authenticate_and_connect(true, &cx).await?;

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
        ];
        let response = client
            .request_envelope(proto::JoinProject {
                project_id: remote_id,
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
        subscriptions: [EntitySubscription; 7],
        client: Arc<Client>,
        run_tasks: bool,
        user_store: Entity<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        mut cx: AsyncApp,
    ) -> Result<Entity<Self>> {
        let remote_id = response.payload.project_id;
        let role = response.payload.role();

        let worktree_store = cx.new(|_| {
            WorktreeStore::remote(true, client.clone().into(), response.payload.project_id)
        })?;
        let buffer_store = cx.new(|cx| {
            BufferStore::remote(worktree_store.clone(), client.clone().into(), remote_id, cx)
        })?;
        let image_store = cx.new(|cx| {
            ImageStore::remote(worktree_store.clone(), client.clone().into(), remote_id, cx)
        })?;

        let environment = cx.new(|_| ProjectEnvironment::new(None))?;

        let breakpoint_store =
            cx.new(|_| BreakpointStore::remote(remote_id, client.clone().into()))?;
        let dap_store = cx.new(|cx| {
            DapStore::new_collab(
                remote_id,
                client.clone().into(),
                breakpoint_store.clone(),
                cx,
            )
        })?;

        let lsp_store = cx.new(|cx| {
            let mut lsp_store = LspStore::new_remote(
                buffer_store.clone(),
                worktree_store.clone(),
                None,
                languages.clone(),
                client.clone().into(),
                remote_id,
                fs.clone(),
                cx,
            );
            lsp_store.set_language_server_statuses_from_proto(response.payload.language_servers);
            lsp_store
        })?;

        let task_store = cx.new(|cx| {
            if run_tasks {
                TaskStore::remote(
                    buffer_store.downgrade(),
                    worktree_store.clone(),
                    Arc::new(EmptyToolchainStore),
                    client.clone().into(),
                    remote_id,
                    cx,
                )
            } else {
                TaskStore::Noop
            }
        })?;

        let settings_observer = cx.new(|cx| {
            SettingsObserver::new_remote(fs.clone(), worktree_store.clone(), task_store.clone(), cx)
        })?;

        let git_store = cx.new(|cx| {
            GitStore::remote(
                // In this remote case we pass None for the environment
                &worktree_store,
                buffer_store.clone(),
                client.clone().into(),
                ProjectId(remote_id),
                cx,
            )
        })?;

        let this = cx.new(|cx| {
            let replica_id = response.payload.replica_id as ReplicaId;

            let snippets = SnippetProvider::new(fs.clone(), BTreeSet::from_iter([]), cx);

            let mut worktrees = Vec::new();
            for worktree in response.payload.worktrees {
                let worktree =
                    Worktree::remote(remote_id, replica_id, worktree, client.clone().into(), cx);
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

            let mut this = Self {
                buffer_ordered_messages_tx: tx,
                buffer_store: buffer_store.clone(),
                image_store,
                worktree_store: worktree_store.clone(),
                lsp_store: lsp_store.clone(),
                active_entry: None,
                collaborators: Default::default(),
                join_project_response_message_id: response.message_id,
                languages,
                user_store: user_store.clone(),
                task_store,
                snippets,
                fs,
                ssh_client: None,
                settings_observer: settings_observer.clone(),
                client_subscriptions: Default::default(),
                _subscriptions: vec![cx.on_release(Self::release)],
                client: client.clone(),
                client_state: ProjectClientState::Remote {
                    sharing_has_stopped: false,
                    capability: Capability::ReadWrite,
                    remote_id,
                    replica_id,
                },
                breakpoint_store,
                dap_store: dap_store.clone(),
                git_store: git_store.clone(),
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                node: None,
                search_history: Self::new_search_history(),
                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),
                environment,
                remotely_created_models: Arc::new(Mutex::new(RemotelyCreatedModels::default())),
                toolchain_store: None,
            };
            this.set_role(role, cx);
            for worktree in worktrees {
                this.add_worktree(&worktree, cx);
            }
            this
        })?;

        let subscriptions = subscriptions
            .into_iter()
            .map(|s| match s {
                EntitySubscription::BufferStore(subscription) => {
                    subscription.set_entity(&buffer_store, &mut cx)
                }
                EntitySubscription::WorktreeStore(subscription) => {
                    subscription.set_entity(&worktree_store, &mut cx)
                }
                EntitySubscription::GitStore(subscription) => {
                    subscription.set_entity(&git_store, &mut cx)
                }
                EntitySubscription::SettingsObserver(subscription) => {
                    subscription.set_entity(&settings_observer, &mut cx)
                }
                EntitySubscription::Project(subscription) => {
                    subscription.set_entity(&this, &mut cx)
                }
                EntitySubscription::LspStore(subscription) => {
                    subscription.set_entity(&lsp_store, &mut cx)
                }
                EntitySubscription::DapStore(subscription) => {
                    subscription.set_entity(&dap_store, &mut cx)
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
            .update(&mut cx, |user_store, cx| user_store.get_users(user_ids, cx))?
            .await?;

        this.update(&mut cx, |this, cx| {
            this.set_collaborators_from_proto(response.payload.collaborators, cx)?;
            this.client_subscriptions.extend(subscriptions);
            anyhow::Ok(())
        })??;

        Ok(this)
    }

    fn new_search_history() -> SearchHistory {
        SearchHistory::new(
            Some(MAX_PROJECT_SEARCH_HISTORY_SIZE),
            search_history::QueryInsertionBehavior::AlwaysInsert,
        )
    }

    fn release(&mut self, cx: &mut App) {
        if let Some(client) = self.ssh_client.take() {
            let shutdown = client
                .read(cx)
                .shutdown_processes(Some(proto::ShutdownRemoteServer {}));

            cx.background_spawn(async move {
                if let Some(shutdown) = shutdown {
                    shutdown.await;
                }
            })
            .detach()
        }

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

    #[cfg(any(test, feature = "test-support"))]
    pub async fn example(
        root_paths: impl IntoIterator<Item = &Path>,
        cx: &mut AsyncApp,
    ) -> Entity<Project> {
        use clock::FakeSystemClock;

        let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
        let languages = LanguageRegistry::test(cx.background_executor().clone());
        let clock = Arc::new(FakeSystemClock::new());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let client = cx
            .update(|cx| client::Client::new(clock, http_client.clone(), cx))
            .unwrap();
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx)).unwrap();
        let project = cx
            .update(|cx| {
                Project::local(
                    client,
                    node_runtime::NodeRuntime::unavailable(),
                    user_store,
                    Arc::new(languages),
                    fs,
                    None,
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

            tree.update(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .await;
        }
        project
    }

    pub fn dap_store(&self) -> Entity<DapStore> {
        self.dap_store.clone()
    }

    pub fn breakpoint_store(&self) -> Entity<BreakpointStore> {
        self.breakpoint_store.clone()
    }

    pub fn lsp_store(&self) -> Entity<LspStore> {
        self.lsp_store.clone()
    }

    pub fn worktree_store(&self) -> Entity<WorktreeStore> {
        self.worktree_store.clone()
    }

    pub fn buffer_for_id(&self, remote_id: BufferId, cx: &App) -> Option<Entity<Buffer>> {
        self.buffer_store.read(cx).get(remote_id)
    }

    pub fn languages(&self) -> &Arc<LanguageRegistry> {
        &self.languages
    }

    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    pub fn ssh_client(&self) -> Option<Entity<SshRemoteClient>> {
        self.ssh_client.clone()
    }

    pub fn user_store(&self) -> Entity<UserStore> {
        self.user_store.clone()
    }

    pub fn node_runtime(&self) -> Option<&NodeRuntime> {
        self.node.as_ref()
    }

    pub fn opened_buffers(&self, cx: &App) -> Vec<Entity<Buffer>> {
        self.buffer_store.read(cx).buffers().collect()
    }

    pub fn environment(&self) -> &Entity<ProjectEnvironment> {
        &self.environment
    }

    pub fn cli_environment(&self, cx: &App) -> Option<HashMap<String, String>> {
        self.environment.read(cx).get_cli_environment()
    }

    pub fn shell_environment_errors<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl Iterator<Item = (&'a Arc<Path>, &'a EnvironmentErrorMessage)> {
        self.environment.read(cx).environment_errors()
    }

    pub fn remove_environment_error(&mut self, abs_path: &Path, cx: &mut Context<Self>) {
        self.environment.update(cx, |environment, cx| {
            environment.remove_environment_error(abs_path, cx);
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn has_open_buffer(&self, path: impl Into<ProjectPath>, cx: &App) -> bool {
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

    pub fn supports_terminal(&self, _cx: &App) -> bool {
        if self.is_local() {
            return true;
        }
        if self.is_via_ssh() {
            return true;
        }

        return false;
    }

    pub fn ssh_connection_string(&self, cx: &App) -> Option<SharedString> {
        if let Some(ssh_state) = &self.ssh_client {
            return Some(ssh_state.read(cx).connection_string().into());
        }

        return None;
    }

    pub fn ssh_connection_state(&self, cx: &App) -> Option<remote::ConnectionState> {
        self.ssh_client
            .as_ref()
            .map(|ssh| ssh.read(cx).connection_state())
    }

    pub fn ssh_connection_options(&self, cx: &App) -> Option<SshConnectionOptions> {
        self.ssh_client
            .as_ref()
            .map(|ssh| ssh.read(cx).connection_options())
    }

    pub fn replica_id(&self) -> ReplicaId {
        match self.client_state {
            ProjectClientState::Remote { replica_id, .. } => replica_id,
            _ => {
                if self.ssh_client.is_some() {
                    1
                } else {
                    0
                }
            }
        }
    }

    pub fn task_store(&self) -> &Entity<TaskStore> {
        &self.task_store
    }

    pub fn snippets(&self) -> &Entity<SnippetProvider> {
        &self.snippets
    }

    pub fn search_history(&self, kind: SearchInputKind) -> &SearchHistory {
        match kind {
            SearchInputKind::Query => &self.search_history,
            SearchInputKind::Include => &self.search_included_history,
            SearchInputKind::Exclude => &self.search_excluded_history,
        }
    }

    pub fn search_history_mut(&mut self, kind: SearchInputKind) -> &mut SearchHistory {
        match kind {
            SearchInputKind::Query => &mut self.search_history,
            SearchInputKind::Include => &mut self.search_included_history,
            SearchInputKind::Exclude => &mut self.search_excluded_history,
        }
    }

    pub fn collaborators(&self) -> &HashMap<proto::PeerId, Collaborator> {
        &self.collaborators
    }

    pub fn host(&self) -> Option<&Collaborator> {
        self.collaborators.values().find(|c| c.is_host)
    }

    pub fn set_worktrees_reordered(&mut self, worktrees_reordered: bool, cx: &mut App) {
        self.worktree_store.update(cx, |store, _| {
            store.set_worktrees_reordered(worktrees_reordered);
        });
    }

    /// Collect all worktrees, including ones that don't appear in the project panel
    pub fn worktrees<'a>(
        &self,
        cx: &'a App,
    ) -> impl 'a + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktree_store.read(cx).worktrees()
    }

    /// Collect all user-visible worktrees, the ones that appear in the project panel.
    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktree_store.read(cx).visible_worktrees(cx)
    }

    pub fn worktree_for_root_name(&self, root_name: &str, cx: &App) -> Option<Entity<Worktree>> {
        self.visible_worktrees(cx)
            .find(|tree| tree.read(cx).root_name() == root_name)
    }

    pub fn worktree_root_names<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = &'a str> {
        self.visible_worktrees(cx)
            .map(|tree| tree.read(cx).root_name())
    }

    pub fn worktree_for_id(&self, id: WorktreeId, cx: &App) -> Option<Entity<Worktree>> {
        self.worktree_store.read(cx).worktree_for_id(id, cx)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &App,
    ) -> Option<Entity<Worktree>> {
        self.worktree_store
            .read(cx)
            .worktree_for_entry(entry_id, cx)
    }

    pub fn worktree_id_for_entry(&self, entry_id: ProjectEntryId, cx: &App) -> Option<WorktreeId> {
        self.worktree_for_entry(entry_id, cx)
            .map(|worktree| worktree.read(cx).id())
    }

    /// Checks if the entry is the root of a worktree.
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

    pub fn project_path_git_status(
        &self,
        project_path: &ProjectPath,
        cx: &App,
    ) -> Option<FileStatus> {
        self.git_store
            .read(cx)
            .project_path_git_status(project_path, cx)
    }

    pub fn visibility_for_paths(
        &self,
        paths: &[PathBuf],
        metadatas: &[Metadata],
        exclude_sub_dirs: bool,
        cx: &App,
    ) -> Option<bool> {
        paths
            .iter()
            .zip(metadatas)
            .map(|(path, metadata)| self.visibility_for_path(path, metadata, exclude_sub_dirs, cx))
            .max()
            .flatten()
    }

    pub fn visibility_for_path(
        &self,
        path: &Path,
        metadata: &Metadata,
        exclude_sub_dirs: bool,
        cx: &App,
    ) -> Option<bool> {
        let sanitized_path = SanitizedPath::from(path);
        let path = sanitized_path.as_path();
        self.worktrees(cx)
            .filter_map(|worktree| {
                let worktree = worktree.read(cx);
                let abs_path = worktree.as_local()?.abs_path();
                let contains = path == abs_path
                    || (path.starts_with(abs_path) && (!exclude_sub_dirs || !metadata.is_dir));
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

    pub fn copy_entry(
        &mut self,
        entry_id: ProjectEntryId,
        relative_worktree_source_path: Option<PathBuf>,
        new_path: impl Into<Arc<Path>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Entry>>> {
        let Some(worktree) = self.worktree_for_entry(entry_id, cx) else {
            return Task::ready(Ok(None));
        };
        worktree.update(cx, |worktree, cx| {
            worktree.copy_entry(entry_id, relative_worktree_source_path, new_path, cx)
        })
    }

    /// Renames the project entry with given `entry_id`.
    ///
    /// `new_path` is a relative path to worktree root.
    /// If root entry is renamed then its new root name is used instead.
    pub fn rename_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<CreatedEntry>> {
        let worktree_store = self.worktree_store.read(cx);
        let new_path = new_path.into();
        let Some((worktree, old_path, is_dir)) = worktree_store
            .worktree_and_entry_for_id(entry_id, cx)
            .map(|(worktree, entry)| (worktree, entry.path.clone(), entry.is_dir()))
        else {
            return Task::ready(Err(anyhow!(format!("No worktree for entry {entry_id:?}"))));
        };

        let worktree_id = worktree.read(cx).id();
        let is_root_entry = self.entry_is_worktree_root(entry_id, cx);

        let lsp_store = self.lsp_store().downgrade();
        cx.spawn(async move |_, cx| {
            let (old_abs_path, new_abs_path) = {
                let root_path = worktree.update(cx, |this, _| this.abs_path())?;
                let new_abs_path = if is_root_entry {
                    root_path.parent().unwrap().join(&new_path)
                } else {
                    root_path.join(&new_path)
                };
                (root_path.join(&old_path), new_abs_path)
            };
            LspStore::will_rename_entry(
                lsp_store.clone(),
                worktree_id,
                &old_abs_path,
                &new_abs_path,
                is_dir,
                cx.clone(),
            )
            .await;

            let entry = worktree
                .update(cx, |worktree, cx| {
                    worktree.rename_entry(entry_id, new_path.clone(), cx)
                })?
                .await?;

            lsp_store
                .update(cx, |this, _| {
                    this.did_rename_entry(worktree_id, &old_abs_path, &new_abs_path, is_dir);
                })
                .ok();
            Ok(entry)
        })
    }

    pub fn delete_file(
        &mut self,
        path: ProjectPath,
        trash: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let entry = self.entry_for_path(&path, cx)?;
        self.delete_entry(entry.id, trash, cx)
    }

    pub fn delete_entry(
        &mut self,
        entry_id: ProjectEntryId,
        trash: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        cx.emit(Event::DeletedEntry(worktree.read(cx).id(), entry_id));
        worktree.update(cx, |worktree, cx| {
            worktree.delete_entry(entry_id, trash, cx)
        })
    }

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
            task.ok_or_else(|| anyhow!("no task"))?.await?;
            this.update(cx, |_, cx| {
                cx.emit(Event::ExpandedAllForEntry(worktree_id, entry_id));
            })?;
            Ok(())
        }))
    }

    pub fn shared(&mut self, project_id: u64, cx: &mut Context<Self>) -> Result<()> {
        if !matches!(self.client_state, ProjectClientState::Local) {
            return Err(anyhow!("project was already shared"));
        }

        self.client_subscriptions.extend([
            self.client
                .subscribe_to_entity(project_id)?
                .set_entity(&cx.entity(), &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.worktree_store, &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.buffer_store, &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.lsp_store, &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.settings_observer, &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.dap_store, &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.breakpoint_store, &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_entity(&self.git_store, &mut cx.to_async()),
        ]);

        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.shared(project_id, self.client.clone().into(), cx)
        });
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.shared(project_id, self.client.clone().into(), cx);
        });
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.shared(project_id, self.client.clone().into(), cx)
        });
        self.breakpoint_store.update(cx, |breakpoint_store, _| {
            breakpoint_store.shared(project_id, self.client.clone().into())
        });
        self.dap_store.update(cx, |dap_store, cx| {
            dap_store.shared(project_id, self.client.clone().into(), cx);
        });
        self.task_store.update(cx, |task_store, cx| {
            task_store.shared(project_id, self.client.clone().into(), cx);
        });
        self.settings_observer.update(cx, |settings_observer, cx| {
            settings_observer.shared(project_id, self.client.clone().into(), cx)
        });
        self.git_store.update(cx, |git_store, cx| {
            git_store.shared(project_id, self.client.clone().into(), cx)
        });

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
        self.buffer_store
            .update(cx, |buffer_store, _| buffer_store.forget_shared_buffers());
        self.set_collaborators_from_proto(message.collaborators, cx)?;

        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.send_project_updates(cx);
        });
        if let Some(remote_id) = self.remote_id() {
            self.git_store.update(cx, |git_store, cx| {
                git_store.shared(remote_id, self.client.clone().into(), cx)
            });
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
            self.worktree_store.update(cx, |worktree_store, cx| {
                for worktree in worktree_store.worktrees() {
                    store
                        .clear_local_settings(worktree.read(cx).id(), cx)
                        .log_err();
                }
            });
        });

        self.join_project_response_message_id = message_id;
        self.set_worktrees_from_proto(message.worktrees, cx)?;
        self.set_collaborators_from_proto(message.collaborators, cx)?;
        self.lsp_store.update(cx, |lsp_store, _| {
            lsp_store.set_language_server_statuses_from_proto(message.language_servers)
        });
        self.enqueue_buffer_ordered_message(BufferOrderedMessage::Resync)
            .unwrap();
        cx.emit(Event::Rejoined);
        Ok(())
    }

    pub fn unshare(&mut self, cx: &mut Context<Self>) -> Result<()> {
        self.unshare_internal(cx)?;
        cx.emit(Event::RemoteIdChanged(None));
        Ok(())
    }

    fn unshare_internal(&mut self, cx: &mut App) -> Result<()> {
        if self.is_via_collab() {
            return Err(anyhow!("attempted to unshare a remote project"));
        }

        if let ProjectClientState::Shared { remote_id, .. } = self.client_state {
            self.client_state = ProjectClientState::Local;
            self.collaborators.clear();
            self.client_subscriptions.clear();
            self.worktree_store.update(cx, |store, cx| {
                store.unshared(cx);
            });
            self.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.forget_shared_buffers();
                buffer_store.unshared(cx)
            });
            self.task_store.update(cx, |task_store, cx| {
                task_store.unshared(cx);
            });
            self.breakpoint_store.update(cx, |breakpoint_store, cx| {
                breakpoint_store.unshared(cx);
            });
            self.dap_store.update(cx, |dap_store, cx| {
                dap_store.unshared(cx);
            });
            self.settings_observer.update(cx, |settings_observer, cx| {
                settings_observer.unshared(cx);
            });
            self.git_store.update(cx, |git_store, cx| {
                git_store.unshared(cx);
            });

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

    fn disconnected_from_host_internal(&mut self, cx: &mut App) {
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
            self.lsp_store
                .update(cx, |lsp_store, _cx| lsp_store.disconnected_from_host());
        }
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        cx.emit(Event::Closed);
    }

    pub fn is_disconnected(&self, cx: &App) -> bool {
        match &self.client_state {
            ProjectClientState::Remote {
                sharing_has_stopped,
                ..
            } => *sharing_has_stopped,
            ProjectClientState::Local if self.is_via_ssh() => self.ssh_is_disconnected(cx),
            _ => false,
        }
    }

    fn ssh_is_disconnected(&self, cx: &App) -> bool {
        self.ssh_client
            .as_ref()
            .map(|ssh| ssh.read(cx).is_disconnected())
            .unwrap_or(false)
    }

    pub fn capability(&self) -> Capability {
        match &self.client_state {
            ProjectClientState::Remote { capability, .. } => *capability,
            ProjectClientState::Shared { .. } | ProjectClientState::Local => Capability::ReadWrite,
        }
    }

    pub fn is_read_only(&self, cx: &App) -> bool {
        self.is_disconnected(cx) || self.capability() == Capability::ReadOnly
    }

    pub fn is_local(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => {
                self.ssh_client.is_none()
            }
            ProjectClientState::Remote { .. } => false,
        }
    }

    pub fn is_via_ssh(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => {
                self.ssh_client.is_some()
            }
            ProjectClientState::Remote { .. } => false,
        }
    }

    pub fn is_via_collab(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Local | ProjectClientState::Shared { .. } => false,
            ProjectClientState::Remote { .. } => true,
        }
    }

    pub fn create_buffer(&mut self, cx: &mut Context<Self>) -> Task<Result<Entity<Buffer>>> {
        self.buffer_store
            .update(cx, |buffer_store, cx| buffer_store.create_buffer(cx))
    }

    pub fn create_local_buffer(
        &mut self,
        text: &str,
        language: Option<Arc<Language>>,
        cx: &mut Context<Self>,
    ) -> Entity<Buffer> {
        if self.is_via_collab() || self.is_via_ssh() {
            panic!("called create_local_buffer on a remote project")
        }
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.create_local_buffer(text, language, cx)
        })
    }

    pub fn open_path(
        &mut self,
        path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Option<ProjectEntryId>, AnyEntity)>> {
        let task = self.open_buffer(path.clone(), cx);
        cx.spawn(async move |_project, cx| {
            let buffer = task.await?;
            let project_entry_id = buffer.read_with(cx, |buffer, cx| {
                File::from_dyn(buffer.file()).and_then(|file| file.project_entry_id(cx))
            })?;

            let buffer: &AnyEntity = &buffer;
            Ok((project_entry_id, buffer.clone()))
        })
    }

    pub fn open_local_buffer(
        &mut self,
        abs_path: impl AsRef<Path>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        if let Some((worktree, relative_path)) = self.find_worktree(abs_path.as_ref(), cx) {
            self.open_buffer((worktree.read(cx).id(), relative_path), cx)
        } else {
            Task::ready(Err(anyhow!("no such path")))
        }
    }

    #[cfg(any(test, feature = "test-support"))]
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

    pub fn open_buffer(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut App,
    ) -> Task<Result<Entity<Buffer>>> {
        if self.is_disconnected(cx) {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }

        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.open_buffer(path.into(), cx)
        })
    }

    #[cfg(any(test, feature = "test-support"))]
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
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.register_buffer_with_language_servers(&buffer, false, cx)
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
        self.git_store
            .update(cx, |git_store, cx| git_store.open_unstaged_diff(buffer, cx))
    }

    pub fn open_uncommitted_diff(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<BufferDiff>>> {
        if self.is_disconnected(cx) {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }
        self.git_store.update(cx, |git_store, cx| {
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
        } else if self.is_local() || self.is_via_ssh() {
            Task::ready(Err(anyhow!("buffer {} does not exist", id)))
        } else if let Some(project_id) = self.remote_id() {
            let request = self.client.request(proto::OpenBufferById {
                project_id,
                id: id.into(),
            });
            cx.spawn(async move |project, cx| {
                let buffer_id = BufferId::new(request.await?.buffer_id)?;
                project
                    .update(cx, |project, cx| {
                        project.buffer_store.update(cx, |buffer_store, cx| {
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
        self.buffer_store
            .update(cx, |buffer_store, cx| buffer_store.save_buffer(buffer, cx))
    }

    pub fn save_buffer_as(
        &mut self,
        buffer: Entity<Buffer>,
        path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.save_buffer_as(buffer.clone(), path, cx)
        })
    }

    pub fn get_open_buffer(&self, path: &ProjectPath, cx: &App) -> Option<Entity<Buffer>> {
        self.buffer_store.read(cx).get_by_path(path, cx)
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

    pub fn open_image(
        &mut self,
        path: impl Into<ProjectPath>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<ImageItem>>> {
        if self.is_disconnected(cx) {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }

        let open_image_task = self.image_store.update(cx, |image_store, cx| {
            image_store.open_image(path.into(), cx)
        });

        let weak_project = cx.entity().downgrade();
        cx.spawn(async move |_, cx| {
            let image_item = open_image_task.await?;
            let project = weak_project
                .upgrade()
                .ok_or_else(|| anyhow!("Project dropped"))?;

            let metadata = ImageItem::load_image_metadata(image_item.clone(), project, cx).await?;
            image_item.update(cx, |image_item, cx| {
                image_item.image_metadata = Some(metadata);
                cx.emit(ImageItemEvent::MetadataUpdated);
            })?;

            Ok(image_item)
        })
    }

    async fn send_buffer_ordered_messages(
        this: WeakEntity<Self>,
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
            let is_local = this.update(cx, |this, _| this.is_local())?;

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
                    } => {
                        flush_operations(
                            &this,
                            &mut operations_by_buffer_id,
                            &mut needs_resync_with_host,
                            is_local,
                            cx,
                        )
                        .await?;

                        this.update(cx, |this, _| {
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
                self.register_buffer(buffer, cx).log_err();
            }
            BufferStoreEvent::BufferDropped(buffer_id) => {
                if let Some(ref ssh_client) = self.ssh_client {
                    ssh_client
                        .read(cx)
                        .proto_client()
                        .send(proto::CloseBuffer {
                            project_id: 0,
                            buffer_id: buffer_id.to_proto(),
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
            DapStoreEvent::Notification(message) => {
                cx.emit(Event::Toast {
                    notification_id: "dap".into(),
                    message: message.clone(),
                });
            }
            _ => {}
        }
    }

    fn on_lsp_store_event(
        &mut self,
        _: Entity<LspStore>,
        event: &LspStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            LspStoreEvent::DiagnosticsUpdated {
                language_server_id,
                path,
            } => cx.emit(Event::DiagnosticsUpdated {
                path: path.clone(),
                language_server_id: *language_server_id,
            }),
            LspStoreEvent::LanguageServerAdded(language_server_id, name, worktree_id) => cx.emit(
                Event::LanguageServerAdded(*language_server_id, name.clone(), *worktree_id),
            ),
            LspStoreEvent::LanguageServerRemoved(language_server_id) => {
                cx.emit(Event::LanguageServerRemoved(*language_server_id))
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
            LspStoreEvent::RefreshInlayHints => cx.emit(Event::RefreshInlayHints),
            LspStoreEvent::RefreshCodeLens => cx.emit(Event::RefreshCodeLens),
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
                message,
            } => {
                if self.is_local() {
                    self.enqueue_buffer_ordered_message(
                        BufferOrderedMessage::LanguageServerUpdate {
                            language_server_id: *language_server_id,
                            message: message.clone(),
                        },
                    )
                    .ok();
                }
            }
            LspStoreEvent::Notification(message) => cx.emit(Event::Toast {
                notification_id: "lsp".into(),
                message: message.clone(),
            }),
            LspStoreEvent::SnippetEdit {
                buffer_id,
                edits,
                most_recent_edit,
            } => {
                if most_recent_edit.replica_id == self.replica_id() {
                    cx.emit(Event::SnippetEdit(*buffer_id, edits.clone()))
                }
            }
        }
    }

    fn on_ssh_event(
        &mut self,
        _: Entity<SshRemoteClient>,
        event: &remote::SshRemoteEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            remote::SshRemoteEvent::Disconnected => {
                // if self.is_via_ssh() {
                // self.collaborators.clear();
                self.worktree_store.update(cx, |store, cx| {
                    store.disconnected_from_host(cx);
                });
                self.buffer_store.update(cx, |buffer_store, cx| {
                    buffer_store.disconnected_from_host(cx)
                });
                self.lsp_store.update(cx, |lsp_store, _cx| {
                    lsp_store.disconnected_from_ssh_remote()
                });
                cx.emit(Event::DisconnectedFromSshRemote);
            }
        }
    }

    fn on_settings_observer_event(
        &mut self,
        _: Entity<SettingsObserver>,
        event: &SettingsObserverEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            SettingsObserverEvent::LocalSettingsUpdated(result) => match result {
                Err(InvalidSettingsError::LocalSettings { message, path }) => {
                    let message = format!("Failed to set local settings in {path:?}:\n{message}");
                    cx.emit(Event::Toast {
                        notification_id: format!("local-settings-{path:?}").into(),
                        message,
                    });
                }
                Ok(path) => cx.emit(Event::HideToast {
                    notification_id: format!("local-settings-{path:?}").into(),
                }),
                Err(_) => {}
            },
            SettingsObserverEvent::LocalTasksUpdated(result) => match result {
                Err(InvalidSettingsError::Tasks { message, path }) => {
                    let message = format!("Failed to set local tasks in {path:?}:\n{message}");
                    cx.emit(Event::Toast {
                        notification_id: format!("local-tasks-{path:?}").into(),
                        message,
                    });
                }
                Ok(path) => cx.emit(Event::HideToast {
                    notification_id: format!("local-tasks-{path:?}").into(),
                }),
                Err(_) => {}
            },
        }
    }

    fn on_worktree_store_event(
        &mut self,
        _: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(worktree) => {
                self.on_worktree_added(worktree, cx);
                cx.emit(Event::WorktreeAdded(worktree.read(cx).id()));
            }
            WorktreeStoreEvent::WorktreeRemoved(_, id) => {
                cx.emit(Event::WorktreeRemoved(*id));
            }
            WorktreeStoreEvent::WorktreeReleased(_, id) => {
                self.on_worktree_released(*id, cx);
            }
            WorktreeStoreEvent::WorktreeOrderChanged => cx.emit(Event::WorktreeOrderChanged),
            WorktreeStoreEvent::WorktreeUpdateSent(_) => {}
            WorktreeStoreEvent::WorktreeUpdatedEntries(worktree_id, changes) => {
                self.client()
                    .telemetry()
                    .report_discovered_project_events(*worktree_id, changes);
                cx.emit(Event::WorktreeUpdatedEntries(*worktree_id, changes.clone()))
            }
            WorktreeStoreEvent::WorktreeDeletedEntry(worktree_id, id) => {
                cx.emit(Event::DeletedEntry(*worktree_id, *id))
            }
            // Listen to the GitStore instead.
            WorktreeStoreEvent::WorktreeUpdatedGitRepositories(_, _) => {}
        }
    }

    fn on_worktree_added(&mut self, worktree: &Entity<Worktree>, _: &mut Context<Self>) {
        let mut remotely_created_models = self.remotely_created_models.lock();
        if remotely_created_models.retain_count > 0 {
            remotely_created_models.worktrees.push(worktree.clone())
        }
    }

    fn on_worktree_released(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
        if let Some(ssh) = &self.ssh_client {
            ssh.read(cx)
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

                if let Some(ssh) = &self.ssh_client {
                    ssh.read(cx)
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
        match event {
            ImageItemEvent::ReloadNeeded => {
                if !self.is_via_collab() {
                    self.reload_images([image.clone()].into_iter().collect(), cx)
                        .detach_and_log_err(cx);
                }
            }
            _ => {}
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
        let delay = if let Some(delay) = settings.git.gutter_debounce {
            delay
        } else {
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
        };

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
                            Some(this.git_store.update(cx, |git_store, cx| {
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
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.set_language_for_buffer(buffer, new_language, cx)
        })
    }

    pub fn restart_language_servers_for_buffers(
        &mut self,
        buffers: Vec<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.restart_language_servers_for_buffers(buffers, cx)
        })
    }

    pub fn stop_language_servers_for_buffers(
        &mut self,
        buffers: Vec<Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.stop_language_servers_for_buffers(buffers, cx)
        })
    }

    pub fn cancel_language_server_work_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.cancel_language_server_work_for_buffers(buffers, cx)
        })
    }

    pub fn cancel_language_server_work(
        &mut self,
        server_id: LanguageServerId,
        token_to_cancel: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.lsp_store.update(cx, |lsp_store, cx| {
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
    ) -> Task<Option<ToolchainList>> {
        if let Some(toolchain_store) = self.toolchain_store.clone() {
            cx.spawn(async move |cx| {
                cx.update(|cx| {
                    toolchain_store
                        .read(cx)
                        .list_toolchains(path, language_name, cx)
                })
                .ok()?
                .await
            })
        } else {
            Task::ready(None)
        }
    }

    pub async fn toolchain_term(
        languages: Arc<LanguageRegistry>,
        language_name: LanguageName,
    ) -> Option<SharedString> {
        languages
            .language_for_name(language_name.as_ref())
            .await
            .ok()?
            .toolchain_lister()
            .map(|lister| lister.term())
    }

    pub fn toolchain_store(&self) -> Option<Entity<ToolchainStore>> {
        self.toolchain_store.clone()
    }
    pub fn activate_toolchain(
        &self,
        path: ProjectPath,
        toolchain: Toolchain,
        cx: &mut App,
    ) -> Task<Option<()>> {
        let Some(toolchain_store) = self.toolchain_store.clone() else {
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
        let Some(toolchain_store) = self.toolchain_store.clone() else {
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
        self.lsp_store.read(cx).language_server_statuses()
    }

    pub fn last_formatting_failure<'a>(&self, cx: &'a App) -> Option<&'a str> {
        self.lsp_store.read(cx).last_formatting_failure()
    }

    pub fn reset_last_formatting_failure(&self, cx: &mut App) {
        self.lsp_store
            .update(cx, |store, _| store.reset_last_formatting_failure());
    }

    pub fn reload_buffers(
        &self,
        buffers: HashSet<Entity<Buffer>>,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.reload_buffers(buffers, push_to_history, cx)
        })
    }

    pub fn reload_images(
        &self,
        images: HashSet<Entity<ImageItem>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.image_store
            .update(cx, |image_store, cx| image_store.reload_images(images, cx))
    }

    pub fn format(
        &mut self,
        buffers: HashSet<Entity<Buffer>>,
        target: LspFormatTarget,
        push_to_history: bool,
        trigger: lsp_store::FormatTrigger,
        cx: &mut Context<Project>,
    ) -> Task<anyhow::Result<ProjectTransaction>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.format(buffers, target, push_to_history, trigger, cx)
        })
    }

    #[inline(never)]
    fn definition_impl(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::FirstCapable,
            GetDefinition { position },
            cx,
        )
    }
    pub fn definition<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.definition_impl(buffer, position, cx)
    }

    fn declaration_impl(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::FirstCapable,
            GetDeclaration { position },
            cx,
        )
    }

    pub fn declaration<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.declaration_impl(buffer, position, cx)
    }

    fn type_definition_impl(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::FirstCapable,
            GetTypeDefinition { position },
            cx,
        )
    }

    pub fn type_definition<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.type_definition_impl(buffer, position, cx)
    }

    pub fn implementation<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::FirstCapable,
            GetImplementation { position },
            cx,
        )
    }

    pub fn references<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<Location>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::FirstCapable,
            GetReferences { position },
            cx,
        )
    }

    fn document_highlights_impl(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<DocumentHighlight>>> {
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::FirstCapable,
            GetDocumentHighlights { position },
            cx,
        )
    }

    pub fn document_highlights<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<DocumentHighlight>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.document_highlights_impl(buffer, position, cx)
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
        self.lsp_store
            .update(cx, |lsp_store, cx| lsp_store.symbols(query, cx))
    }

    pub fn open_buffer_for_symbol(
        &mut self,
        symbol: &Symbol,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.open_buffer_for_symbol(symbol, cx)
        })
    }

    pub fn open_server_settings(&mut self, cx: &mut Context<Self>) -> Task<Result<Entity<Buffer>>> {
        let guard = self.retain_remotely_created_models(cx);
        let Some(ssh_client) = self.ssh_client.as_ref() else {
            return Task::ready(Err(anyhow!("not an ssh project")));
        };

        let proto_client = ssh_client.read(cx).proto_client();

        cx.spawn(async move |project, cx| {
            let buffer = proto_client
                .request(proto::OpenServerSettings {
                    project_id: SSH_PROJECT_ID,
                })
                .await?;

            let buffer = project
                .update(cx, |project, cx| {
                    project.buffer_store.update(cx, |buffer_store, cx| {
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
        abs_path: lsp::Url,
        language_server_id: LanguageServerId,
        language_server_name: LanguageServerName,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.open_local_buffer_via_lsp(
                abs_path,
                language_server_id,
                language_server_name,
                cx,
            )
        })
    }

    pub fn signature_help<T: ToPointUtf16>(
        &self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Vec<SignatureHelp>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.signature_help(buffer, position, cx)
        })
    }

    pub fn hover<T: ToPointUtf16>(
        &self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Hover>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.lsp_store
            .update(cx, |lsp_store, cx| lsp_store.hover(buffer, position, cx))
    }

    pub fn linked_edit(
        &self,
        buffer: &Entity<Buffer>,
        position: Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<Range<Anchor>>>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.linked_edit(buffer, position, cx)
        })
    }

    pub fn completions<T: ToOffset + ToPointUtf16>(
        &self,
        buffer: &Entity<Buffer>,
        position: T,
        context: CompletionContext,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<Completion>>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.completions(buffer, position, context, cx)
        })
    }

    pub fn code_actions<T: Clone + ToOffset>(
        &mut self,
        buffer_handle: &Entity<Buffer>,
        range: Range<T>,
        kinds: Option<Vec<CodeActionKind>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        let buffer = buffer_handle.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.code_actions(buffer_handle, range, kinds, cx)
        })
    }

    pub fn code_lens<T: Clone + ToOffset>(
        &mut self,
        buffer_handle: &Entity<Buffer>,
        range: Range<T>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        let snapshot = buffer_handle.read(cx).snapshot();
        let range = snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end);
        let code_lens_actions = self
            .lsp_store
            .update(cx, |lsp_store, cx| lsp_store.code_lens(buffer_handle, cx));

        cx.background_spawn(async move {
            let mut code_lens_actions = code_lens_actions.await?;
            code_lens_actions.retain(|code_lens_action| {
                range
                    .start
                    .cmp(&code_lens_action.range.start, &snapshot)
                    .is_ge()
                    && range
                        .end
                        .cmp(&code_lens_action.range.end, &snapshot)
                        .is_le()
            });
            Ok(code_lens_actions)
        })
    }

    pub fn apply_code_action(
        &self,
        buffer_handle: Entity<Buffer>,
        action: CodeAction,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.apply_code_action(buffer_handle, action, push_to_history, cx)
        })
    }

    pub fn apply_code_action_kind(
        &self,
        buffers: HashSet<Entity<Buffer>>,
        kind: CodeActionKind,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.apply_code_action_kind(buffers, kind, push_to_history, cx)
        })
    }

    fn prepare_rename_impl(
        &mut self,
        buffer: Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<PrepareRenameResponse>> {
        self.request_lsp(
            buffer,
            LanguageServerToQuery::FirstCapable,
            PrepareRename { position },
            cx,
        )
    }
    pub fn prepare_rename<T: ToPointUtf16>(
        &mut self,
        buffer: Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Result<PrepareRenameResponse>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.prepare_rename_impl(buffer, position, cx)
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
        self.request_lsp(
            buffer,
            LanguageServerToQuery::FirstCapable,
            PerformRename {
                position,
                new_name,
                push_to_history,
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
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.on_type_format(buffer, position, trigger, push_to_history, cx)
        })
    }

    pub fn inlay_hints<T: ToOffset>(
        &mut self,
        buffer_handle: Entity<Buffer>,
        range: Range<T>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Vec<InlayHint>>> {
        let buffer = buffer_handle.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.inlay_hints(buffer_handle, range, cx)
        })
    }

    pub fn resolve_inlay_hint(
        &self,
        hint: InlayHint,
        buffer_handle: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<InlayHint>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.resolve_inlay_hint(hint, buffer_handle, server_id, cx)
        })
    }

    pub fn search(&mut self, query: SearchQuery, cx: &mut Context<Self>) -> Receiver<SearchResult> {
        let (result_tx, result_rx) = smol::channel::unbounded();

        let matching_buffers_rx = if query.is_opened_only() {
            self.sort_search_candidates(&query, cx)
        } else {
            self.find_search_candidate_buffers(&query, MAX_SEARCH_RESULT_FILES + 1, cx)
        };

        cx.spawn(async move |_, cx| {
            let mut range_count = 0;
            let mut buffer_count = 0;
            let mut limit_reached = false;
            let query = Arc::new(query);
            let mut chunks = matching_buffers_rx.ready_chunks(64);

            // Now that we know what paths match the query, we will load at most
            // 64 buffers at a time to avoid overwhelming the main thread. For each
            // opened buffer, we will spawn a background task that retrieves all the
            // ranges in the buffer matched by the query.
            let mut chunks = pin!(chunks);
            'outer: while let Some(matching_buffer_chunk) = chunks.next().await {
                let mut chunk_results = Vec::new();
                for buffer in matching_buffer_chunk {
                    let buffer = buffer.clone();
                    let query = query.clone();
                    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
                    chunk_results.push(cx.background_spawn(async move {
                        let ranges = query
                            .search(&snapshot, None)
                            .await
                            .iter()
                            .map(|range| {
                                snapshot.anchor_before(range.start)
                                    ..snapshot.anchor_after(range.end)
                            })
                            .collect::<Vec<_>>();
                        anyhow::Ok((buffer, ranges))
                    }));
                }

                let chunk_results = futures::future::join_all(chunk_results).await;
                for result in chunk_results {
                    if let Some((buffer, ranges)) = result.log_err() {
                        range_count += ranges.len();
                        buffer_count += 1;
                        result_tx
                            .send(SearchResult::Buffer { buffer, ranges })
                            .await?;
                        if buffer_count > MAX_SEARCH_RESULT_FILES
                            || range_count > MAX_SEARCH_RESULT_RANGES
                        {
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

    fn find_search_candidate_buffers(
        &mut self,
        query: &SearchQuery,
        limit: usize,
        cx: &mut Context<Project>,
    ) -> Receiver<Entity<Buffer>> {
        if self.is_local() {
            let fs = self.fs.clone();
            self.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.find_search_candidates(query, limit, fs, cx)
            })
        } else {
            self.find_search_candidates_remote(query, limit, cx)
        }
    }

    fn sort_search_candidates(
        &mut self,
        search_query: &SearchQuery,
        cx: &mut Context<Project>,
    ) -> Receiver<Entity<Buffer>> {
        let worktree_store = self.worktree_store.read(cx);
        let mut buffers = search_query
            .buffers()
            .into_iter()
            .flatten()
            .filter(|buffer| {
                let b = buffer.read(cx);
                if let Some(file) = b.file() {
                    if !search_query.match_path(file.path()) {
                        return false;
                    }
                    if let Some(entry) = b
                        .entry_id(cx)
                        .and_then(|entry_id| worktree_store.entry_for_id(entry_id, cx))
                    {
                        if entry.is_ignored && !search_query.include_ignored() {
                            return false;
                        }
                    }
                }
                true
            })
            .collect::<Vec<_>>();
        let (tx, rx) = smol::channel::unbounded();
        buffers.sort_by(|a, b| match (a.read(cx).file(), b.read(cx).file()) {
            (None, None) => a.read(cx).remote_id().cmp(&b.read(cx).remote_id()),
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(a), Some(b)) => compare_paths((a.path(), true), (b.path(), true)),
        });
        for buffer in buffers {
            tx.send_blocking(buffer.clone()).unwrap()
        }

        rx
    }

    fn find_search_candidates_remote(
        &mut self,
        query: &SearchQuery,
        limit: usize,
        cx: &mut Context<Project>,
    ) -> Receiver<Entity<Buffer>> {
        let (tx, rx) = smol::channel::unbounded();

        let (client, remote_id): (AnyProtoClient, _) = if let Some(ssh_client) = &self.ssh_client {
            (ssh_client.read(cx).proto_client(), 0)
        } else if let Some(remote_id) = self.remote_id() {
            (self.client.clone().into(), remote_id)
        } else {
            return rx;
        };

        let request = client.request(proto::FindSearchCandidates {
            project_id: remote_id,
            query: Some(query.to_proto()),
            limit: limit as _,
        });
        let guard = self.retain_remotely_created_models(cx);

        cx.spawn(async move |project, cx| {
            let response = request.await?;
            for buffer_id in response.buffer_ids {
                let buffer_id = BufferId::new(buffer_id)?;
                let buffer = project
                    .update(cx, |project, cx| {
                        project.buffer_store.update(cx, |buffer_store, cx| {
                            buffer_store.wait_for_remote_buffer(buffer_id, cx)
                        })
                    })?
                    .await?;
                let _ = tx.send(buffer).await;
            }

            drop(guard);
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
        rx
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
        let task = self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.request_lsp(buffer_handle, server, request, cx)
        });
        cx.spawn(async move |_, _| {
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
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.move_worktree(source, destination, cx)
        })
    }

    pub fn find_or_create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Entity<Worktree>, PathBuf)>> {
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.find_or_create_worktree(abs_path, visible, cx)
        })
    }

    pub fn find_worktree(&self, abs_path: &Path, cx: &App) -> Option<(Entity<Worktree>, PathBuf)> {
        self.worktree_store.read_with(cx, |worktree_store, cx| {
            worktree_store.find_worktree(abs_path, cx)
        })
    }

    pub fn is_shared(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Shared { .. } => true,
            ProjectClientState::Local => false,
            ProjectClientState::Remote { .. } => true,
        }
    }

    /// Returns the resolved version of `path`, that was found in `buffer`, if it exists.
    pub fn resolve_path_in_buffer(
        &self,
        path: &str,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Option<ResolvedPath>> {
        let path_buf = PathBuf::from(path);
        if path_buf.is_absolute() || path.starts_with("~") {
            self.resolve_abs_path(path, cx)
        } else {
            self.resolve_path_in_worktrees(path_buf, buffer, cx)
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

    pub fn resolve_abs_path(
        &self,
        path: &str,
        cx: &mut Context<Self>,
    ) -> Task<Option<ResolvedPath>> {
        if self.is_local() {
            let expanded = PathBuf::from(shellexpand::tilde(&path).into_owned());
            let fs = self.fs.clone();
            cx.background_spawn(async move {
                let path = expanded.as_path();
                let metadata = fs.metadata(path).await.ok().flatten();

                metadata.map(|metadata| ResolvedPath::AbsPath {
                    path: expanded,
                    is_dir: metadata.is_dir,
                })
            })
        } else if let Some(ssh_client) = self.ssh_client.as_ref() {
            let request_path = Path::new(path);
            let request = ssh_client
                .read(cx)
                .proto_client()
                .request(proto::GetPathMetadata {
                    project_id: SSH_PROJECT_ID,
                    path: request_path.to_proto(),
                });
            cx.background_spawn(async move {
                let response = request.await.log_err()?;
                if response.exists {
                    Some(ResolvedPath::AbsPath {
                        path: PathBuf::from_proto(response.path),
                        is_dir: response.is_dir,
                    })
                } else {
                    None
                }
            })
        } else {
            return Task::ready(None);
        }
    }

    fn resolve_path_in_worktrees(
        &self,
        path: PathBuf,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Option<ResolvedPath>> {
        let mut candidates = vec![path.clone()];

        if let Some(file) = buffer.read(cx).file() {
            if let Some(dir) = file.path().parent() {
                let joined = dir.to_path_buf().join(path);
                candidates.push(joined);
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

        cx.spawn(async move |_, mut cx| {
            if let Some(buffer_worktree_id) = buffer_worktree_id {
                if let Some((worktree, _)) = worktrees_with_ids
                    .iter()
                    .find(|(_, id)| *id == buffer_worktree_id)
                {
                    for candidate in candidates.iter() {
                        if let Some(path) =
                            Self::resolve_path_in_worktree(&worktree, candidate, &mut cx)
                        {
                            return Some(path);
                        }
                    }
                }
            }
            for (worktree, id) in worktrees_with_ids {
                if Some(id) == buffer_worktree_id {
                    continue;
                }
                for candidate in candidates.iter() {
                    if let Some(path) =
                        Self::resolve_path_in_worktree(&worktree, candidate, &mut cx)
                    {
                        return Some(path);
                    }
                }
            }
            None
        })
    }

    fn resolve_path_in_worktree(
        worktree: &Entity<Worktree>,
        path: &PathBuf,
        cx: &mut AsyncApp,
    ) -> Option<ResolvedPath> {
        worktree
            .update(cx, |worktree, _| {
                let root_entry_path = &worktree.root_entry()?.path;
                let resolved = resolve_path(root_entry_path, path);
                let stripped = resolved.strip_prefix(root_entry_path).unwrap_or(&resolved);
                worktree.entry_for_path(stripped).map(|entry| {
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
            .ok()?
    }

    pub fn list_directory(
        &self,
        query: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<DirectoryItem>>> {
        if self.is_local() {
            DirectoryLister::Local(self.fs.clone()).list_directory(query, cx)
        } else if let Some(session) = self.ssh_client.as_ref() {
            let path_buf = PathBuf::from(query);
            let request = proto::ListRemoteDirectory {
                dev_server_id: SSH_PROJECT_ID,
                path: path_buf.to_proto(),
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
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.create_worktree(abs_path, visible, cx)
        })
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.remove_worktree(id_to_remove, cx);
        });
    }

    fn add_worktree(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.add(worktree, cx);
        });
    }

    pub fn set_active_path(&mut self, entry: Option<ProjectPath>, cx: &mut Context<Self>) {
        let new_active_entry = entry.and_then(|project_path| {
            let worktree = self.worktree_for_id(project_path.worktree_id, cx)?;
            let entry = worktree.read(cx).entry_for_path(project_path.path)?;
            Some(entry.id)
        });
        if new_active_entry != self.active_entry {
            self.active_entry = new_active_entry;
            self.lsp_store.update(cx, |lsp_store, _| {
                lsp_store.set_active_entry(new_active_entry);
            });
            cx.emit(Event::ActiveEntryChanged(new_active_entry));
        }
    }

    pub fn language_servers_running_disk_based_diagnostics<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl Iterator<Item = LanguageServerId> + 'a {
        self.lsp_store
            .read(cx)
            .language_servers_running_disk_based_diagnostics()
    }

    pub fn diagnostic_summary(&self, include_ignored: bool, cx: &App) -> DiagnosticSummary {
        self.lsp_store
            .read(cx)
            .diagnostic_summary(include_ignored, cx)
    }

    pub fn diagnostic_summaries<'a>(
        &'a self,
        include_ignored: bool,
        cx: &'a App,
    ) -> impl Iterator<Item = (ProjectPath, LanguageServerId, DiagnosticSummary)> + 'a {
        self.lsp_store
            .read(cx)
            .diagnostic_summaries(include_ignored, cx)
    }

    pub fn active_entry(&self) -> Option<ProjectEntryId> {
        self.active_entry
    }

    pub fn entry_for_path(&self, path: &ProjectPath, cx: &App) -> Option<Entry> {
        self.worktree_store.read(cx).entry_for_path(path, cx)
    }

    pub fn path_for_entry(&self, entry_id: ProjectEntryId, cx: &App) -> Option<ProjectPath> {
        let worktree = self.worktree_for_entry(entry_id, cx)?;
        let worktree = worktree.read(cx);
        let worktree_id = worktree.id();
        let path = worktree.entry_for_id(entry_id)?.path.clone();
        Some(ProjectPath { worktree_id, path })
    }

    pub fn absolute_path(&self, project_path: &ProjectPath, cx: &App) -> Option<PathBuf> {
        self.worktree_for_id(project_path.worktree_id, cx)?
            .read(cx)
            .absolutize(&project_path.path)
            .ok()
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
    pub fn find_project_path(&self, path: impl AsRef<Path>, cx: &App) -> Option<ProjectPath> {
        let path = path.as_ref();
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

    pub fn project_path_for_absolute_path(&self, abs_path: &Path, cx: &App) -> Option<ProjectPath> {
        self.find_worktree(abs_path, cx)
            .map(|(worktree, relative_path)| ProjectPath {
                worktree_id: worktree.read(cx).id(),
                path: relative_path.into(),
            })
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
        self.git_store.update(cx, |git_store, cx| {
            git_store.blame_buffer(buffer, version, cx)
        })
    }

    pub fn get_permalink_to_line(
        &self,
        buffer: &Entity<Buffer>,
        selection: Range<u32>,
        cx: &mut App,
    ) -> Task<Result<url::Url>> {
        self.git_store.update(cx, |git_store, cx| {
            git_store.get_permalink_to_line(buffer, selection, cx)
        })
    }

    // RPC message handlers

    async fn handle_unshare_project(
        this: Entity<Self>,
        _: TypedEnvelope<proto::UnshareProject>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            if this.is_local() || this.is_via_ssh() {
                this.unshare(cx)?;
            } else {
                this.disconnected_from_host(cx);
            }
            Ok(())
        })?
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
            .ok_or_else(|| anyhow!("empty collaborator"))?;

        let collaborator = Collaborator::from_proto(collaborator)?;
        this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, _| {
                buffer_store.forget_shared_buffers_for(&collaborator.peer_id);
            });
            this.breakpoint_store.read(cx).broadcast();
            cx.emit(Event::CollaboratorJoined(collaborator.peer_id));
            this.collaborators
                .insert(collaborator.peer_id, collaborator);
        })?;

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
            let is_host = collaborator.is_host;
            this.collaborators.insert(new_peer_id, collaborator);

            log::info!("peer {} became {}", old_peer_id, new_peer_id,);
            this.buffer_store.update(cx, |buffer_store, _| {
                buffer_store.update_peer_id(&old_peer_id, new_peer_id)
            });

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
            Ok(())
        })?
    }

    async fn handle_remove_collaborator(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RemoveProjectCollaborator>,
        mut cx: AsyncApp,
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
                buffer_store.forget_shared_buffers_for(&peer_id);
                for buffer in buffer_store.buffers() {
                    buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
                }
            });
            this.git_store.update(cx, |git_store, _| {
                git_store.forget_shared_diffs_for(&peer_id);
            });

            cx.emit(Event::CollaboratorLeft(peer_id));
            Ok(())
        })?
    }

    async fn handle_update_project(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateProject>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            // Don't handle messages that were sent before the response to us joining the project
            if envelope.message_id > this.join_project_response_message_id {
                this.set_worktrees_from_proto(envelope.payload.worktrees, cx)?;
            }
            Ok(())
        })?
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
            });
            Ok(())
        })?
    }

    async fn handle_language_server_prompt_request(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::LanguageServerPromptRequest>,
        mut cx: AsyncApp,
    ) -> Result<proto::LanguageServerPromptResponse> {
        let (tx, mut rx) = smol::channel::bounded(1);
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
            cx.emit(Event::LanguageServerPrompt(LanguageServerPromptRequest {
                level: proto_to_prompt(envelope.payload.level.context("Invalid prompt level")?),
                message: envelope.payload.message,
                actions: actions.clone(),
                lsp_name: envelope.payload.lsp_name,
                response_channel: tx,
            }));

            anyhow::Ok(())
        })??;

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
        })?
    }

    // Collab sends UpdateWorktree protos as messages
    async fn handle_update_worktree(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateWorktree>,
        mut cx: AsyncApp,
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

    async fn handle_update_buffer_from_ssh(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let buffer_store = this.read_with(&cx, |this, cx| {
            if let Some(remote_id) = this.remote_id() {
                let mut payload = envelope.payload.clone();
                payload.project_id = remote_id;
                cx.background_spawn(this.client.request(payload))
                    .detach_and_log_err(cx);
            }
            this.buffer_store.clone()
        })?;
        BufferStore::handle_update_buffer(buffer_store, envelope, cx).await
    }

    async fn handle_update_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let buffer_store = this.read_with(&cx, |this, cx| {
            if let Some(ssh) = &this.ssh_client {
                let mut payload = envelope.payload.clone();
                payload.project_id = SSH_PROJECT_ID;
                cx.background_spawn(ssh.read(cx).proto_client().request(payload))
                    .detach_and_log_err(cx);
            }
            this.buffer_store.clone()
        })?;
        BufferStore::handle_update_buffer(buffer_store, envelope, cx).await
    }

    fn retain_remotely_created_models(
        &mut self,
        cx: &mut Context<Self>,
    ) -> RemotelyCreatedModelGuard {
        {
            let mut remotely_create_models = self.remotely_created_models.lock();
            if remotely_create_models.retain_count == 0 {
                remotely_create_models.buffers = self.buffer_store.read(cx).buffers().collect();
                remotely_create_models.worktrees =
                    self.worktree_store.read(cx).worktrees().collect();
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

    async fn handle_synchronize_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SynchronizeBuffers>,
        mut cx: AsyncApp,
    ) -> Result<proto::SynchronizeBuffersResponse> {
        let response = this.update(&mut cx, |this, cx| {
            let client = this.client.clone();
            this.buffer_store.update(cx, |this, cx| {
                this.handle_synchronize_buffers(envelope, cx, client)
            })
        })??;

        Ok(response)
    }

    async fn handle_search_candidate_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidates>,
        mut cx: AsyncApp,
    ) -> Result<proto::FindSearchCandidatesResponse> {
        let peer_id = envelope.original_sender_id()?;
        let message = envelope.payload;
        let query = SearchQuery::from_proto(
            message
                .query
                .ok_or_else(|| anyhow!("missing query field"))?,
        )?;
        let results = this.update(&mut cx, |this, cx| {
            this.find_search_candidate_buffers(&query, message.limit as _, cx)
        })?;

        let mut response = proto::FindSearchCandidatesResponse {
            buffer_ids: Vec::new(),
        };

        while let Ok(buffer) = results.recv().await {
            this.update(&mut cx, |this, cx| {
                let buffer_id = this.create_buffer_for_peer(&buffer, peer_id, cx);
                response.buffer_ids.push(buffer_id.to_proto());
            })?;
        }

        Ok(response)
    }

    async fn handle_open_buffer_by_id(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenBufferById>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let peer_id = envelope.original_sender_id()?;
        let buffer_id = BufferId::new(envelope.payload.id)?;
        let buffer = this
            .update(&mut cx, |this, cx| this.open_buffer_by_id(buffer_id, cx))?
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
        let open_buffer = this.update(&mut cx, |this, cx| {
            this.open_buffer(
                ProjectPath {
                    worktree_id,
                    path: Arc::<Path>::from_proto(envelope.payload.path),
                },
                cx,
            )
        })?;

        let buffer = open_buffer.await?;
        Project::respond_to_open_buffer_request(this, buffer, peer_id, &mut cx)
    }

    async fn handle_open_new_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenNewBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let buffer = this
            .update(&mut cx, |this, cx| this.create_buffer(cx))?
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
            if is_private {
                Err(anyhow!(ErrorCode::UnsharedItem))
            } else {
                Ok(proto::OpenBufferResponse {
                    buffer_id: this.create_buffer_for_peer(&buffer, peer_id, cx).into(),
                })
            }
        })?
    }

    fn create_buffer_for_peer(
        &mut self,
        buffer: &Entity<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> BufferId {
        self.buffer_store
            .update(cx, |buffer_store, cx| {
                buffer_store.create_buffer_for_peer(buffer, peer_id, cx)
            })
            .detach_and_log_err(cx);
        buffer.read(cx).remote_id()
    }

    fn synchronize_remote_buffers(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
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
                )));
            }
        };

        let client = self.client.clone();
        cx.spawn(async move |this, cx| {
            let (buffers, incomplete_buffer_ids) = this.update(cx, |this, cx| {
                this.buffer_store.read(cx).buffer_version_info(cx)
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
        self.worktree_store.read(cx).worktree_metadata_protos(cx)
    }

    /// Iterator of all open buffers that have unsaved changes
    pub fn dirty_buffers<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = ProjectPath> + 'a {
        self.buffer_store.read(cx).buffers().filter_map(|buf| {
            let buf = buf.read(cx);
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
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.set_worktrees_from_proto(worktrees, self.replica_id(), cx)
        })
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
        self.lsp_store.read(cx).supplementary_language_servers()
    }

    pub fn any_language_server_supports_inlay_hints(&self, buffer: &Buffer, cx: &mut App) -> bool {
        self.lsp_store.update(cx, |this, cx| {
            this.language_servers_for_local_buffer(buffer, cx)
                .any(
                    |(_, server)| match server.capabilities().inlay_hint_provider {
                        Some(lsp::OneOf::Left(enabled)) => enabled,
                        Some(lsp::OneOf::Right(_)) => true,
                        None => false,
                    },
                )
        })
    }

    pub fn language_server_id_for_name(
        &self,
        buffer: &Buffer,
        name: &str,
        cx: &mut App,
    ) -> Task<Option<LanguageServerId>> {
        if self.is_local() {
            Task::ready(self.lsp_store.update(cx, |lsp_store, cx| {
                lsp_store
                    .language_servers_for_local_buffer(buffer, cx)
                    .find_map(|(adapter, server)| {
                        if adapter.name.0 == name {
                            Some(server.server_id())
                        } else {
                            None
                        }
                    })
            }))
        } else if let Some(project_id) = self.remote_id() {
            let request = self.client.request(proto::LanguageServerIdForName {
                project_id,
                buffer_id: buffer.remote_id().to_proto(),
                name: name.to_string(),
            });
            cx.background_spawn(async move {
                let response = request.await.log_err()?;
                response.server_id.map(LanguageServerId::from_proto)
            })
        } else if let Some(ssh_client) = self.ssh_client.as_ref() {
            let request =
                ssh_client
                    .read(cx)
                    .proto_client()
                    .request(proto::LanguageServerIdForName {
                        project_id: SSH_PROJECT_ID,
                        buffer_id: buffer.remote_id().to_proto(),
                        name: name.to_string(),
                    });
            cx.background_spawn(async move {
                let response = request.await.log_err()?;
                response.server_id.map(LanguageServerId::from_proto)
            })
        } else {
            Task::ready(None)
        }
    }

    pub fn has_language_servers_for(&self, buffer: &Buffer, cx: &mut App) -> bool {
        self.lsp_store.update(cx, |this, cx| {
            this.language_servers_for_local_buffer(buffer, cx)
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
        self.git_store
            .read(cx)
            .git_init(path, fallback_branch_name, cx)
    }

    pub fn buffer_store(&self) -> &Entity<BufferStore> {
        &self.buffer_store
    }

    pub fn git_store(&self) -> &Entity<GitStore> {
        &self.git_store
    }

    #[cfg(test)]
    fn git_scans_complete(&self, cx: &Context<Self>) -> Task<()> {
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

    pub fn active_repository(&self, cx: &App) -> Option<Entity<Repository>> {
        self.git_store.read(cx).active_repository()
    }

    pub fn repositories<'a>(&self, cx: &'a App) -> &'a HashMap<RepositoryId, Entity<Repository>> {
        self.git_store.read(cx).repositories()
    }

    pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus> {
        self.git_store.read(cx).status_for_buffer_id(buffer_id, cx)
    }
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
            format!("{}{}", self.snapshot.root_name(), std::path::MAIN_SEPARATOR).into()
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

impl<'a> From<&'a ProjectPath> for SettingsLocation<'a> {
    fn from(val: &'a ProjectPath) -> Self {
        SettingsLocation {
            worktree_id: val.worktree_id,
            path: val.path.as_ref(),
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

pub fn relativize_path(base: &Path, path: &Path) -> PathBuf {
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

/// ResolvedPath is a path that has been resolved to either a ProjectPath
/// or an AbsPath and that *exists*.
#[derive(Debug, Clone)]
pub enum ResolvedPath {
    ProjectPath {
        project_path: ProjectPath,
        is_dir: bool,
    },
    AbsPath {
        path: PathBuf,
        is_dir: bool,
    },
}

impl ResolvedPath {
    pub fn abs_path(&self) -> Option<&Path> {
        match self {
            Self::AbsPath { path, .. } => Some(path.as_path()),
            _ => None,
        }
    }

    pub fn project_path(&self) -> Option<&ProjectPath> {
        match self {
            Self::ProjectPath { project_path, .. } => Some(&project_path),
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

    fn entry_id(&self, cx: &App) -> Option<ProjectEntryId> {
        File::from_dyn(self.file()).and_then(|file| file.project_entry_id(cx))
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        self.file().map(|file| ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty()
    }
}

impl Completion {
    /// A key that can be used to sort completions when displaying
    /// them to the user.
    pub fn sort_key(&self) -> (usize, &str) {
        const DEFAULT_KIND_KEY: usize = 2;
        let kind_key = self
            .source
            // `lsp::CompletionListItemDefaults` has no `kind` field
            .lsp_completion(false)
            .and_then(|lsp_completion| lsp_completion.kind)
            .and_then(|lsp_completion_kind| match lsp_completion_kind {
                lsp::CompletionItemKind::KEYWORD => Some(0),
                lsp::CompletionItemKind::VARIABLE => Some(1),
                _ => None,
            })
            .unwrap_or(DEFAULT_KIND_KEY);
        (kind_key, &self.label.text[self.label.filter_range.clone()])
    }

    /// Whether this completion is a snippet.
    pub fn is_snippet(&self) -> bool {
        self.source
            // `lsp::CompletionListItemDefaults` has `insert_text_format` field
            .lsp_completion(true)
            .map_or(false, |lsp_completion| {
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

pub fn sort_worktree_entries(entries: &mut [impl AsRef<Entry>]) {
    entries.sort_by(|entry_a, entry_b| {
        let entry_a = entry_a.as_ref();
        let entry_b = entry_b.as_ref();
        compare_paths(
            (&entry_a.path, entry_a.is_file()),
            (&entry_b.path, entry_b.is_file()),
        )
    });
}

fn proto_to_prompt(level: proto::language_server_prompt_request::Level) -> gpui::PromptLevel {
    match level {
        proto::language_server_prompt_request::Level::Info(_) => gpui::PromptLevel::Info,
        proto::language_server_prompt_request::Level::Warning(_) => gpui::PromptLevel::Warning,
        proto::language_server_prompt_request::Level::Critical(_) => gpui::PromptLevel::Critical,
    }
}
