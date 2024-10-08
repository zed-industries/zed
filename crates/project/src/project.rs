pub mod buffer_store;
mod color_extractor;
pub mod connection_manager;
pub mod debounced_delay;
pub mod lsp_command;
pub mod lsp_ext_command;
pub mod lsp_store;
pub mod prettier_store;
pub mod project_settings;
pub mod search;
mod task_inventory;
pub mod terminals;
pub mod worktree_store;

#[cfg(test)]
mod project_tests;

mod direnv;
mod environment;
pub use environment::EnvironmentErrorMessage;
pub mod search_history;
mod yarn;

use anyhow::{anyhow, Context as _, Result};
use buffer_store::{BufferStore, BufferStoreEvent};
use client::{
    proto, Client, Collaborator, DevServerProjectId, PendingEntitySubscription, ProjectId,
    TypedEnvelope, UserStore,
};
use clock::ReplicaId;
use collections::{BTreeSet, HashMap, HashSet};
use debounced_delay::DebouncedDelay;
pub use environment::ProjectEnvironment;
use futures::{
    channel::mpsc::{self, UnboundedReceiver},
    future::try_join_all,
    StreamExt,
};

use git::{blame::Blame, repository::GitRepository};
use gpui::{
    AnyModel, AppContext, AsyncAppContext, BorrowAppContext, Context, EventEmitter, Hsla, Model,
    ModelContext, SharedString, Task, WeakModel, WindowContext,
};
use itertools::Itertools;
use language::{
    language_settings::InlayHintKind,
    proto::{deserialize_anchor, serialize_anchor, split_operations},
    Buffer, BufferEvent, CachedLspAdapter, Capability, CodeLabel, ContextProvider, DiagnosticEntry,
    Documentation, File as _, Language, LanguageRegistry, LanguageServerName, PointUtf16, ToOffset,
    ToPointUtf16, Transaction, Unclipped,
};
use lsp::{
    CompletionContext, CompletionItemKind, DocumentHighlightKind, LanguageServer, LanguageServerId,
};
use lsp_command::*;
use node_runtime::NodeRuntime;
use parking_lot::{Mutex, RwLock};
use paths::{local_tasks_file_relative_path, local_vscode_tasks_file_relative_path};
pub use prettier_store::PrettierStore;
use project_settings::{ProjectSettings, SettingsObserver, SettingsObserverEvent};
use remote::SshRemoteClient;
use rpc::{proto::SSH_PROJECT_ID, AnyProtoClient, ErrorCode};
use search::{SearchInputKind, SearchQuery, SearchResult};
use search_history::SearchHistory;
use settings::{
    watch_config_file, InvalidSettingsError, Settings, SettingsLocation, SettingsStore,
};
use smol::channel::Receiver;
use snippet::Snippet;
use snippet_provider::SnippetProvider;
use std::{
    borrow::Cow,
    ops::Range,
    path::{Component, Path, PathBuf},
    str,
    sync::Arc,
    time::Duration,
};
use task::{
    static_source::{StaticSource, TrackedFile},
    HideStrategy, RevealStrategy, Shell, TaskContext, TaskTemplate, TaskVariables, VariableName,
};
use terminals::Terminals;
use text::{Anchor, BufferId};
use util::{paths::compare_paths, ResultExt as _};
use worktree::{CreatedEntry, Snapshot, Traversal};
use worktree_store::{WorktreeStore, WorktreeStoreEvent};

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

pub use buffer_store::ProjectTransaction;
pub use lsp_store::{
    DiagnosticSummary, LanguageServerLogType, LanguageServerProgress, LanguageServerPromptRequest,
    LanguageServerStatus, LanguageServerToQuery, LspStore, LspStoreEvent,
    SERVER_PROGRESS_THROTTLE_TIMEOUT,
};

const MAX_PROJECT_SEARCH_HISTORY_SIZE: usize = 500;
const MAX_SEARCH_RESULT_FILES: usize = 5_000;
const MAX_SEARCH_RESULT_RANGES: usize = 10_000;

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
    client: Arc<client::Client>,
    join_project_response_message_id: u32,
    user_store: Model<UserStore>,
    fs: Arc<dyn Fs>,
    ssh_client: Option<Model<SshRemoteClient>>,
    client_state: ProjectClientState,
    collaborators: HashMap<proto::PeerId, Collaborator>,
    client_subscriptions: Vec<client::Subscription>,
    worktree_store: Model<WorktreeStore>,
    buffer_store: Model<BufferStore>,
    lsp_store: Model<LspStore>,
    _subscriptions: Vec<gpui::Subscription>,
    buffers_needing_diff: HashSet<WeakModel<Buffer>>,
    git_diff_debouncer: DebouncedDelay<Self>,
    remotely_created_models: Arc<Mutex<RemotelyCreatedModels>>,
    terminals: Terminals,
    node: Option<NodeRuntime>,
    tasks: Model<Inventory>,
    hosted_project_id: Option<ProjectId>,
    dev_server_project_id: Option<client::DevServerProjectId>,
    search_history: SearchHistory,
    search_included_history: SearchHistory,
    search_excluded_history: SearchHistory,
    snippets: Model<SnippetProvider>,
    environment: Model<ProjectEnvironment>,
    settings_observer: Model<SettingsObserver>,
}

#[derive(Default)]
struct RemotelyCreatedModels {
    worktrees: Vec<Model<Worktree>>,
    buffers: Vec<Model<Buffer>>,
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
    Local,
    Shared {
        remote_id: u64,
    },
    Remote {
        sharing_has_stopped: bool,
        capability: Capability,
        remote_id: u64,
        replica_id: ReplicaId,
        in_room: bool,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    LanguageServerAdded(LanguageServerId),
    LanguageServerRemoved(LanguageServerId),
    LanguageServerLog(LanguageServerId, LanguageServerLogType, String),
    Notification(String),
    LocalSettingsUpdated(Result<(), InvalidSettingsError>),
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
pub(crate) struct CoreCompletion {
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
    WorktreeStore(PendingEntitySubscription<WorktreeStore>),
    LspStore(PendingEntitySubscription<LspStore>),
    SettingsObserver(PendingEntitySubscription<SettingsObserver>),
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

    pub fn resolve_tilde<'a>(&self, path: &'a String, cx: &AppContext) -> Cow<'a, str> {
        if self.is_local(cx) {
            shellexpand::tilde(path)
        } else {
            Cow::from(path)
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

    pub fn list_directory(&self, path: String, cx: &mut AppContext) -> Task<Result<Vec<PathBuf>>> {
        match self {
            DirectoryLister::Project(project) => {
                project.update(cx, |project, cx| project.list_directory(path, cx))
            }
            DirectoryLister::Local(fs) => {
                let fs = fs.clone();
                cx.background_executor().spawn(async move {
                    let mut results = vec![];
                    let expanded = shellexpand::tilde(&path);
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

        let client: AnyProtoClient = client.clone().into();
        client.add_model_message_handler(Self::handle_add_collaborator);
        client.add_model_message_handler(Self::handle_update_project_collaborator);
        client.add_model_message_handler(Self::handle_remove_collaborator);
        client.add_model_message_handler(Self::handle_update_project);
        client.add_model_message_handler(Self::handle_unshare_project);
        client.add_model_request_handler(Self::handle_update_buffer);
        client.add_model_message_handler(Self::handle_update_worktree);
        client.add_model_request_handler(Self::handle_synchronize_buffers);

        client.add_model_request_handler(Self::handle_search_candidate_buffers);
        client.add_model_request_handler(Self::handle_open_buffer_by_id);
        client.add_model_request_handler(Self::handle_open_buffer_by_path);
        client.add_model_request_handler(Self::handle_open_new_buffer);
        client.add_model_request_handler(Self::handle_task_context_for_location);
        client.add_model_request_handler(Self::handle_task_templates);
        client.add_model_message_handler(Self::handle_create_buffer_for_peer);

        WorktreeStore::init(&client);
        BufferStore::init(&client);
        LspStore::init(&client);
        SettingsObserver::init(&client);
    }

    pub fn local(
        client: Arc<Client>,
        node: NodeRuntime,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        env: Option<HashMap<String, String>>,
        cx: &mut AppContext,
    ) -> Model<Self> {
        cx.new_model(|cx: &mut ModelContext<Self>| {
            let (tx, rx) = mpsc::unbounded();
            cx.spawn(move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx))
                .detach();
            let tasks = Inventory::new(cx);
            let snippets = SnippetProvider::new(fs.clone(), BTreeSet::from_iter([]), cx);
            let worktree_store = cx.new_model(|_| WorktreeStore::local(false, fs.clone()));
            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();

            let buffer_store = cx.new_model(|cx| BufferStore::local(worktree_store.clone(), cx));
            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();

            let prettier_store = cx.new_model(|cx| {
                PrettierStore::new(
                    node.clone(),
                    fs.clone(),
                    languages.clone(),
                    worktree_store.clone(),
                    cx,
                )
            });

            let settings_observer = cx.new_model(|cx| {
                SettingsObserver::new_local(fs.clone(), worktree_store.clone(), cx)
            });
            cx.subscribe(&settings_observer, Self::on_settings_observer_event)
                .detach();

            let environment = ProjectEnvironment::new(&worktree_store, env, cx);
            let lsp_store = cx.new_model(|cx| {
                LspStore::new_local(
                    buffer_store.clone(),
                    worktree_store.clone(),
                    prettier_store.clone(),
                    environment.clone(),
                    languages.clone(),
                    Some(client.http_client()),
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
                lsp_store,
                join_project_response_message_id: 0,
                client_state: ProjectClientState::Local,
                client_subscriptions: Vec::new(),
                _subscriptions: vec![cx.on_release(Self::release)],
                active_entry: None,
                snippets,
                languages,
                client,
                user_store,
                settings_observer,
                fs,
                ssh_client: None,
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                node: Some(node),
                tasks,
                hosted_project_id: None,
                dev_server_project_id: None,
                search_history: Self::new_search_history(),
                environment,
                remotely_created_models: Default::default(),

                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),
            }
        })
    }

    pub fn ssh(
        ssh: Model<SshRemoteClient>,
        client: Arc<Client>,
        node: NodeRuntime,
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

            let ssh_proto = ssh.read(cx).to_proto_client();
            let worktree_store =
                cx.new_model(|_| WorktreeStore::remote(false, ssh_proto.clone(), 0, None));
            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();

            let buffer_store = cx.new_model(|cx| {
                BufferStore::remote(
                    worktree_store.clone(),
                    ssh.read(cx).to_proto_client(),
                    SSH_PROJECT_ID,
                    cx,
                )
            });
            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();

            let settings_observer = cx.new_model(|cx| {
                SettingsObserver::new_ssh(ssh_proto.clone(), worktree_store.clone(), cx)
            });
            cx.subscribe(&settings_observer, Self::on_settings_observer_event)
                .detach();

            let environment = ProjectEnvironment::new(&worktree_store, None, cx);
            let lsp_store = cx.new_model(|cx| {
                LspStore::new_remote(
                    buffer_store.clone(),
                    worktree_store.clone(),
                    languages.clone(),
                    ssh_proto.clone(),
                    SSH_PROJECT_ID,
                    cx,
                )
            });
            cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();

            cx.on_release(|this, cx| {
                if let Some(ssh_client) = this.ssh_client.as_ref() {
                    ssh_client
                        .read(cx)
                        .to_proto_client()
                        .send(proto::ShutdownRemoteServer {})
                        .log_err();
                }
            })
            .detach();

            let this = Self {
                buffer_ordered_messages_tx: tx,
                collaborators: Default::default(),
                worktree_store,
                buffer_store,
                lsp_store,
                join_project_response_message_id: 0,
                client_state: ProjectClientState::Local,
                client_subscriptions: Vec::new(),
                _subscriptions: vec![cx.on_release(Self::release)],
                active_entry: None,
                snippets,
                languages,
                client,
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
                tasks,
                hosted_project_id: None,
                dev_server_project_id: None,
                search_history: Self::new_search_history(),
                environment,
                remotely_created_models: Default::default(),

                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),
            };

            let ssh = ssh.read(cx);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &cx.handle());
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.buffer_store);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.worktree_store);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.lsp_store);
            ssh.subscribe_to_entity(SSH_PROJECT_ID, &this.settings_observer);

            ssh_proto.add_model_message_handler(Self::handle_create_buffer_for_peer);
            ssh_proto.add_model_message_handler(Self::handle_update_worktree);
            ssh_proto.add_model_message_handler(Self::handle_update_project);
            ssh_proto.add_model_request_handler(BufferStore::handle_update_buffer);
            BufferStore::init(&ssh_proto);
            LspStore::init(&ssh_proto);
            SettingsObserver::init(&ssh_proto);

            this
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

        let subscriptions = [
            EntitySubscription::Project(client.subscribe_to_entity::<Self>(remote_id)?),
            EntitySubscription::BufferStore(client.subscribe_to_entity::<BufferStore>(remote_id)?),
            EntitySubscription::WorktreeStore(
                client.subscribe_to_entity::<WorktreeStore>(remote_id)?,
            ),
            EntitySubscription::LspStore(client.subscribe_to_entity::<LspStore>(remote_id)?),
            EntitySubscription::SettingsObserver(
                client.subscribe_to_entity::<SettingsObserver>(remote_id)?,
            ),
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
            user_store,
            languages,
            fs,
            cx,
        )
        .await
    }

    async fn from_join_project_response(
        response: TypedEnvelope<proto::JoinProjectResponse>,
        subscriptions: [EntitySubscription; 5],
        client: Arc<Client>,
        user_store: Model<UserStore>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        mut cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        let remote_id = response.payload.project_id;
        let role = response.payload.role();

        let worktree_store = cx.new_model(|_| {
            WorktreeStore::remote(
                true,
                client.clone().into(),
                response.payload.project_id,
                response
                    .payload
                    .dev_server_project_id
                    .map(DevServerProjectId),
            )
        })?;
        let buffer_store = cx.new_model(|cx| {
            BufferStore::remote(worktree_store.clone(), client.clone().into(), remote_id, cx)
        })?;

        let lsp_store = cx.new_model(|cx| {
            let mut lsp_store = LspStore::new_remote(
                buffer_store.clone(),
                worktree_store.clone(),
                languages.clone(),
                client.clone().into(),
                remote_id,
                cx,
            );
            lsp_store.set_language_server_statuses_from_proto(response.payload.language_servers);
            lsp_store
        })?;

        let settings_observer =
            cx.new_model(|cx| SettingsObserver::new_remote(worktree_store.clone(), cx))?;

        let this = cx.new_model(|cx| {
            let replica_id = response.payload.replica_id as ReplicaId;
            let tasks = Inventory::new(cx);

            let snippets = SnippetProvider::new(fs.clone(), BTreeSet::from_iter([]), cx);

            let mut worktrees = Vec::new();
            for worktree in response.payload.worktrees {
                let worktree =
                    Worktree::remote(remote_id, replica_id, worktree, client.clone().into(), cx);
                worktrees.push(worktree);
            }

            let (tx, rx) = mpsc::unbounded();
            cx.spawn(move |this, cx| Self::send_buffer_ordered_messages(this, rx, cx))
                .detach();

            cx.subscribe(&worktree_store, Self::on_worktree_store_event)
                .detach();

            cx.subscribe(&buffer_store, Self::on_buffer_store_event)
                .detach();
            cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();
            cx.subscribe(&settings_observer, Self::on_settings_observer_event)
                .detach();

            let mut this = Self {
                buffer_ordered_messages_tx: tx,
                buffer_store: buffer_store.clone(),
                worktree_store: worktree_store.clone(),
                lsp_store: lsp_store.clone(),
                active_entry: None,
                collaborators: Default::default(),
                join_project_response_message_id: response.message_id,
                languages,
                user_store: user_store.clone(),
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
                    in_room: response.payload.dev_server_project_id.is_none(),
                },
                buffers_needing_diff: Default::default(),
                git_diff_debouncer: DebouncedDelay::new(),
                terminals: Terminals {
                    local_handles: Vec::new(),
                },
                node: None,
                tasks,
                hosted_project_id: None,
                dev_server_project_id: response
                    .payload
                    .dev_server_project_id
                    .map(DevServerProjectId),
                search_history: Self::new_search_history(),
                search_included_history: Self::new_search_history(),
                search_excluded_history: Self::new_search_history(),
                environment: ProjectEnvironment::new(&worktree_store, None, cx),
                remotely_created_models: Arc::new(Mutex::new(RemotelyCreatedModels::default())),
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
                    subscription.set_model(&buffer_store, &mut cx)
                }
                EntitySubscription::WorktreeStore(subscription) => {
                    subscription.set_model(&worktree_store, &mut cx)
                }
                EntitySubscription::SettingsObserver(subscription) => {
                    subscription.set_model(&settings_observer, &mut cx)
                }
                EntitySubscription::Project(subscription) => subscription.set_model(&this, &mut cx),
                EntitySubscription::LspStore(subscription) => {
                    subscription.set_model(&lsp_store, &mut cx)
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

    pub async fn hosted(
        remote_id: ProjectId,
        user_store: Model<UserStore>,
        client: Arc<Client>,
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        client.authenticate_and_connect(true, &cx).await?;

        let subscriptions = [
            EntitySubscription::Project(client.subscribe_to_entity::<Self>(remote_id.0)?),
            EntitySubscription::BufferStore(
                client.subscribe_to_entity::<BufferStore>(remote_id.0)?,
            ),
            EntitySubscription::WorktreeStore(
                client.subscribe_to_entity::<WorktreeStore>(remote_id.0)?,
            ),
            EntitySubscription::LspStore(client.subscribe_to_entity::<LspStore>(remote_id.0)?),
            EntitySubscription::SettingsObserver(
                client.subscribe_to_entity::<SettingsObserver>(remote_id.0)?,
            ),
        ];
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

            project.update(cx, |project, cx| {
                let tree_id = tree.read(cx).id();
                project.environment.update(cx, |environment, _| {
                    environment.set_cached(&[(tree_id, HashMap::default())])
                });
            });

            tree.update(cx, |tree, _| tree.as_local().unwrap().scan_complete())
                .await;
        }
        project
    }

    pub fn lsp_store(&self) -> Model<LspStore> {
        self.lsp_store.clone()
    }

    pub fn worktree_store(&self) -> Model<WorktreeStore> {
        self.worktree_store.clone()
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

    pub fn node_runtime(&self) -> Option<&NodeRuntime> {
        self.node.as_ref()
    }

    pub fn opened_buffers(&self, cx: &AppContext) -> Vec<Model<Buffer>> {
        self.buffer_store.read(cx).buffers().collect()
    }

    pub fn cli_environment(&self, cx: &AppContext) -> Option<HashMap<String, String>> {
        self.environment.read(cx).get_cli_environment()
    }

    pub fn shell_environment_errors<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = (&'a WorktreeId, &'a EnvironmentErrorMessage)> {
        self.environment.read(cx).environment_errors()
    }

    pub fn remove_environment_error(
        &mut self,
        cx: &mut ModelContext<Self>,
        worktree_id: WorktreeId,
    ) {
        self.environment.update(cx, |environment, _| {
            environment.remove_environment_error(worktree_id);
        });
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

    pub fn supports_terminal(&self, cx: &AppContext) -> bool {
        if self.is_local() {
            return true;
        }
        if self.is_via_ssh() {
            return true;
        }
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

    pub fn ssh_connection_string(&self, cx: &AppContext) -> Option<SharedString> {
        if let Some(ssh_state) = &self.ssh_client {
            return Some(ssh_state.read(cx).connection_string().into());
        }
        let dev_server_id = self.dev_server_project_id()?;
        dev_server_projects::Store::global(cx)
            .read(cx)
            .dev_server_for_project(dev_server_id)?
            .ssh_connection_string
            .clone()
    }

    pub fn ssh_connection_state(&self, cx: &AppContext) -> Option<remote::ConnectionState> {
        self.ssh_client
            .as_ref()
            .map(|ssh| ssh.read(cx).connection_state())
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

    pub fn task_inventory(&self) -> &Model<Inventory> {
        &self.tasks
    }

    pub fn snippets(&self) -> &Model<SnippetProvider> {
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
        relative_worktree_source_path: Option<PathBuf>,
        new_path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Entry>>> {
        let Some(worktree) = self.worktree_for_entry(entry_id, cx) else {
            return Task::ready(Ok(None));
        };
        worktree.update(cx, |worktree, cx| {
            worktree.copy_entry(entry_id, relative_worktree_source_path, new_path, cx)
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
            self.client
                .subscribe_to_entity(project_id)?
                .set_model(&self.lsp_store, &mut cx.to_async()),
            self.client
                .subscribe_to_entity(project_id)?
                .set_model(&self.settings_observer, &mut cx.to_async()),
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
        self.settings_observer.update(cx, |settings_observer, cx| {
            settings_observer.shared(project_id, self.client.clone().into(), cx)
        });

        self.client_state = ProjectClientState::Shared {
            remote_id: project_id,
        };

        cx.emit(Event::RemoteIdChanged(Some(project_id)));
        cx.notify();
        Ok(())
    }

    pub fn reshared(
        &mut self,
        message: proto::ResharedProject,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.buffer_store
            .update(cx, |buffer_store, _| buffer_store.forget_shared_buffers());
        self.set_collaborators_from_proto(message.collaborators, cx)?;

        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.send_project_updates(cx);
        });
        cx.notify();
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
        cx.notify();
        Ok(())
    }

    pub fn unshare(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        self.unshare_internal(cx)?;
        cx.notify();
        Ok(())
    }

    fn unshare_internal(&mut self, cx: &mut AppContext) -> Result<()> {
        if self.is_via_collab() {
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
            self.client_subscriptions.clear();
            self.worktree_store.update(cx, |store, cx| {
                store.unshared(cx);
            });
            self.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.forget_shared_buffers();
                buffer_store.unshared(cx)
            });
            self.settings_observer.update(cx, |settings_observer, cx| {
                settings_observer.unshared(cx);
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
            self.lsp_store
                .update(cx, |lsp_store, _cx| lsp_store.disconnected_from_host());
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

    pub fn create_buffer(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<Model<Buffer>>> {
        self.buffer_store
            .update(cx, |buffer_store, cx| buffer_store.create_buffer(cx))
    }

    pub fn create_local_buffer(
        &mut self,
        text: &str,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Model<Buffer> {
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
        if self.is_via_collab() && self.is_disconnected() {
            return Task::ready(Err(anyhow!(ErrorCode::Disconnected)));
        }

        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.open_buffer(path.into(), cx)
        })
    }

    pub fn open_buffer_by_id(
        &mut self,
        id: BufferId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        if let Some(buffer) = self.buffer_for_id(id, cx) {
            Task::ready(Ok(buffer))
        } else if self.is_local() || self.is_via_ssh() {
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
            BufferStoreEvent::BufferChangedFilePath { .. } => {}
            BufferStoreEvent::BufferDropped(buffer_id) => {
                if let Some(ref ssh_client) = self.ssh_client {
                    ssh_client
                        .read(cx)
                        .to_proto_client()
                        .send(proto::CloseBuffer {
                            project_id: 0,
                            buffer_id: buffer_id.to_proto(),
                        })
                        .log_err();
                }
            }
        }
    }

    fn on_lsp_store_event(
        &mut self,
        _: Model<LspStore>,
        event: &LspStoreEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            LspStoreEvent::DiagnosticsUpdated {
                language_server_id,
                path,
            } => cx.emit(Event::DiagnosticsUpdated {
                path: path.clone(),
                language_server_id: *language_server_id,
            }),
            LspStoreEvent::LanguageServerAdded(language_server_id) => {
                cx.emit(Event::LanguageServerAdded(*language_server_id))
            }
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
            LspStoreEvent::Notification(message) => cx.emit(Event::Notification(message.clone())),
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

    fn on_settings_observer_event(
        &mut self,
        _: Model<SettingsObserver>,
        event: &SettingsObserverEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            SettingsObserverEvent::LocalSettingsUpdated(error) => {
                cx.emit(Event::LocalSettingsUpdated(error.clone()))
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
            WorktreeStoreEvent::WorktreeAdded(worktree) => {
                self.on_worktree_added(worktree, cx);
                cx.emit(Event::WorktreeAdded);
            }
            WorktreeStoreEvent::WorktreeRemoved(_, id) => {
                self.on_worktree_removed(*id, cx);
                cx.emit(Event::WorktreeRemoved(*id));
            }
            WorktreeStoreEvent::WorktreeOrderChanged => cx.emit(Event::WorktreeOrderChanged),
            WorktreeStoreEvent::WorktreeUpdateSent(_) => {}
        }
    }

    fn on_worktree_added(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
        {
            let mut remotely_created_models = self.remotely_created_models.lock();
            if remotely_created_models.retain_count > 0 {
                remotely_created_models.worktrees.push(worktree.clone())
            }
        }
        cx.observe(worktree, |_, _, cx| cx.notify()).detach();
        cx.subscribe(worktree, |this, worktree, event, cx| {
            let is_local = worktree.read(cx).is_local();
            match event {
                worktree::Event::UpdatedEntries(changes) => {
                    if is_local {
                        this.update_local_worktree_settings(&worktree, changes, cx);
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
        cx.notify();
    }

    fn on_worktree_removed(&mut self, id_to_remove: WorktreeId, cx: &mut ModelContext<Self>) {
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
            if !paths.is_empty() {
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

        self.task_inventory().update(cx, |inventory, _| {
            inventory.remove_worktree_sources(id_to_remove);
        });

        cx.notify();
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
            BufferEvent::Operation {
                operation,
                is_local: true,
            } => {
                let operation = language::proto::serialize_operation(operation);

                if let Some(ssh) = &self.ssh_client {
                    ssh.read(cx)
                        .to_proto_client()
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
                        .update(&mut cx, |buffer, cx| buffer.recalculate_diff(cx))
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

    pub fn set_language_for_buffer(
        &mut self,
        buffer: &Model<Buffer>,
        new_language: Arc<Language>,
        cx: &mut ModelContext<Self>,
    ) {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.set_language_for_buffer(buffer, new_language, cx)
        })
    }

    pub fn restart_language_servers_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Model<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.restart_language_servers_for_buffers(buffers, cx)
        })
    }

    pub fn cancel_language_server_work_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Model<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.cancel_language_server_work_for_buffers(buffers, cx)
        })
    }

    pub fn cancel_language_server_work(
        &mut self,
        server_id: LanguageServerId,
        token_to_cancel: Option<String>,
        cx: &mut ModelContext<Self>,
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

    pub fn language_server_statuses<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl DoubleEndedIterator<Item = (LanguageServerId, &'a LanguageServerStatus)> {
        self.lsp_store.read(cx).language_server_statuses()
    }

    pub fn last_formatting_failure<'a>(&self, cx: &'a AppContext) -> Option<&'a str> {
        self.lsp_store.read(cx).last_formatting_failure()
    }

    pub fn update_diagnostics(
        &mut self,
        language_server_id: LanguageServerId,
        params: lsp::PublishDiagnosticsParams,
        disk_based_sources: &[String],
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.update_diagnostics(language_server_id, params, disk_based_sources, cx)
        })
    }

    pub fn update_diagnostic_entries(
        &mut self,
        server_id: LanguageServerId,
        abs_path: PathBuf,
        version: Option<i32>,
        diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        cx: &mut ModelContext<Project>,
    ) -> Result<(), anyhow::Error> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.update_diagnostic_entries(server_id, abs_path, version, diagnostics, cx)
        })
    }

    pub fn reload_buffers(
        &self,
        buffers: HashSet<Model<Buffer>>,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.reload_buffers(buffers, push_to_history, cx)
        })
    }

    pub fn format(
        &mut self,
        buffers: HashSet<Model<Buffer>>,
        push_to_history: bool,
        trigger: lsp_store::FormatTrigger,
        cx: &mut ModelContext<Project>,
    ) -> Task<anyhow::Result<ProjectTransaction>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.format(buffers, push_to_history, trigger, cx)
        })
    }

    #[inline(never)]
    fn definition_impl(
        &mut self,
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
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.definition_impl(buffer, position, cx)
    }

    fn declaration_impl(
        &mut self,
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
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.declaration_impl(buffer, position, cx)
    }

    fn type_definition_impl(
        &mut self,
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
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.type_definition_impl(buffer, position, cx)
    }

    pub fn implementation<T: ToPointUtf16>(
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetImplementation { position },
            cx,
        )
    }

    pub fn references<T: ToPointUtf16>(
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Location>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.request_lsp(
            buffer.clone(),
            LanguageServerToQuery::Primary,
            GetReferences { position },
            cx,
        )
    }

    fn document_highlights_impl(
        &mut self,
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
        &mut self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<DocumentHighlight>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.document_highlights_impl(buffer, position, cx)
    }

    pub fn symbols(&self, query: &str, cx: &mut ModelContext<Self>) -> Task<Result<Vec<Symbol>>> {
        self.lsp_store
            .update(cx, |lsp_store, cx| lsp_store.symbols(query, cx))
    }

    pub fn open_buffer_for_symbol(
        &mut self,
        symbol: &Symbol,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.open_buffer_for_symbol(symbol, cx)
        })
    }

    pub fn open_local_buffer_via_lsp(
        &mut self,
        abs_path: lsp::Url,
        language_server_id: LanguageServerId,
        language_server_name: LanguageServerName,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
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
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<SignatureHelp>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.signature_help(buffer, position, cx)
        })
    }

    pub fn hover<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<Hover>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.lsp_store
            .update(cx, |lsp_store, cx| lsp_store.hover(buffer, position, cx))
    }

    pub fn linked_edit(
        &self,
        buffer: &Model<Buffer>,
        position: Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Range<Anchor>>>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.linked_edit(buffer, position, cx)
        })
    }

    pub fn completions<T: ToOffset + ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        context: CompletionContext,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.completions(buffer, position, context, cx)
        })
    }

    pub fn resolve_completions(
        &self,
        buffer: Model<Buffer>,
        completion_indices: Vec<usize>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<bool>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.resolve_completions(buffer, completion_indices, completions, cx)
        })
    }

    pub fn apply_additional_edits_for_completion(
        &self,
        buffer_handle: Model<Buffer>,
        completion: Completion,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.apply_additional_edits_for_completion(
                buffer_handle,
                completion,
                push_to_history,
                cx,
            )
        })
    }

    pub fn code_actions<T: Clone + ToOffset>(
        &mut self,
        buffer_handle: &Model<Buffer>,
        range: Range<T>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        let buffer = buffer_handle.read(cx);
        let range = buffer.anchor_before(range.start)..buffer.anchor_before(range.end);
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.code_actions(buffer_handle, range, cx)
        })
    }

    pub fn apply_code_action(
        &self,
        buffer_handle: Model<Buffer>,
        action: CodeAction,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.apply_code_action(buffer_handle, action, push_to_history, cx)
        })
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

    pub fn on_type_format<T: ToPointUtf16>(
        &mut self,
        buffer: Model<Buffer>,
        position: T,
        trigger: String,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.on_type_format(buffer, position, trigger, push_to_history, cx)
        })
    }

    pub fn inlay_hints<T: ToOffset>(
        &mut self,
        buffer_handle: Model<Buffer>,
        range: Range<T>,
        cx: &mut ModelContext<Self>,
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
        buffer_handle: Model<Buffer>,
        server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<InlayHint>> {
        self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.resolve_inlay_hint(hint, buffer_handle, server_id, cx)
        })
    }

    pub fn search(
        &mut self,
        query: SearchQuery,
        cx: &mut ModelContext<Self>,
    ) -> Receiver<SearchResult> {
        let (result_tx, result_rx) = smol::channel::unbounded();

        let matching_buffers_rx = if query.is_opened_only() {
            self.sort_search_candidates(&query, cx)
        } else {
            self.find_search_candidate_buffers(&query, MAX_SEARCH_RESULT_FILES + 1, cx)
        };

        cx.spawn(|_, cx| async move {
            let mut range_count = 0;
            let mut buffer_count = 0;
            let mut limit_reached = false;
            let query = Arc::new(query);
            let mut chunks = matching_buffers_rx.ready_chunks(64);

            // Now that we know what paths match the query, we will load at most
            // 64 buffers at a time to avoid overwhelming the main thread. For each
            // opened buffer, we will spawn a background task that retrieves all the
            // ranges in the buffer matched by the query.
            'outer: while let Some(matching_buffer_chunk) = chunks.next().await {
                let mut chunk_results = Vec::new();
                for buffer in matching_buffer_chunk {
                    let buffer = buffer.clone();
                    let query = query.clone();
                    let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot())?;
                    chunk_results.push(cx.background_executor().spawn(async move {
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
        cx: &mut ModelContext<Project>,
    ) -> Receiver<Model<Buffer>> {
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
        cx: &mut ModelContext<Project>,
    ) -> Receiver<Model<Buffer>> {
        let worktree_store = self.worktree_store.read(cx);
        let mut buffers = search_query
            .buffers()
            .into_iter()
            .flatten()
            .filter(|buffer| {
                let b = buffer.read(cx);
                if let Some(file) = b.file() {
                    if !search_query.file_matches(file.path()) {
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
        cx: &mut ModelContext<Project>,
    ) -> Receiver<Model<Buffer>> {
        let (tx, rx) = smol::channel::unbounded();

        let (client, remote_id): (AnyProtoClient, _) = if let Some(ssh_client) = &self.ssh_client {
            (ssh_client.read(cx).to_proto_client(), 0)
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

        cx.spawn(move |this, mut cx| async move {
            let response = request.await?;
            for buffer_id in response.buffer_ids {
                let buffer_id = BufferId::new(buffer_id)?;
                let buffer = this
                    .update(&mut cx, |this, cx| {
                        this.wait_for_remote_buffer(buffer_id, cx)
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
        buffer_handle: Model<Buffer>,
        server: LanguageServerToQuery,
        request: R,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<R::Response>>
    where
        <R::LspRequest as lsp::request::Request>::Result: Send,
        <R::LspRequest as lsp::request::Request>::Params: Send,
    {
        let guard = self.retain_remotely_created_models(cx);
        let task = self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.request_lsp(buffer_handle, server, request, cx)
        });
        cx.spawn(|_, _| async move {
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
            worktree_store.find_worktree(abs_path, cx)
        })
    }

    pub fn is_shared(&self) -> bool {
        match &self.client_state {
            ProjectClientState::Shared { .. } => true,
            ProjectClientState::Local => false,
            ProjectClientState::Remote { in_room, .. } => *in_room,
        }
    }

    // Returns the resolved version of `path`, that was found in `buffer`, if it exists.
    pub fn resolve_existing_file_path(
        &self,
        path: &str,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Option<ResolvedPath>> {
        let path_buf = PathBuf::from(path);
        if path_buf.is_absolute() || path.starts_with("~") {
            if self.is_local() {
                let expanded = PathBuf::from(shellexpand::tilde(&path).into_owned());

                let fs = self.fs.clone();
                cx.background_executor().spawn(async move {
                    let path = expanded.as_path();
                    let exists = fs.is_file(path).await;

                    exists.then(|| ResolvedPath::AbsPath(expanded))
                })
            } else if let Some(ssh_client) = self.ssh_client.as_ref() {
                let request =
                    ssh_client
                        .read(cx)
                        .to_proto_client()
                        .request(proto::CheckFileExists {
                            project_id: SSH_PROJECT_ID,
                            path: path.to_string(),
                        });
                cx.background_executor().spawn(async move {
                    let response = request.await.log_err()?;
                    if response.exists {
                        Some(ResolvedPath::AbsPath(PathBuf::from(response.path)))
                    } else {
                        None
                    }
                })
            } else {
                return Task::ready(None);
            }
        } else {
            self.resolve_path_in_worktrees(path_buf, buffer, cx)
        }
    }

    fn resolve_path_in_worktrees(
        &self,
        path: PathBuf,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Option<ResolvedPath>> {
        let mut candidates = vec![path.clone()];

        if let Some(file) = buffer.read(cx).file() {
            if let Some(dir) = file.path().parent() {
                let joined = dir.to_path_buf().join(path);
                candidates.push(joined);
            }
        }

        let worktrees = self.worktrees(cx).collect::<Vec<_>>();
        cx.spawn(|_, mut cx| async move {
            for worktree in worktrees {
                for candidate in candidates.iter() {
                    let path = worktree
                        .update(&mut cx, |worktree, _| {
                            let root_entry_path = &worktree.root_entry()?.path;

                            let resolved = resolve_path(root_entry_path, candidate);

                            let stripped =
                                resolved.strip_prefix(root_entry_path).unwrap_or(&resolved);

                            worktree.entry_for_path(stripped).map(|entry| {
                                ResolvedPath::ProjectPath(ProjectPath {
                                    worktree_id: worktree.id(),
                                    path: entry.path.clone(),
                                })
                            })
                        })
                        .ok()?;

                    if path.is_some() {
                        return path;
                    }
                }
            }
            None
        })
    }

    pub fn list_directory(
        &self,
        query: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<PathBuf>>> {
        if self.is_local() {
            DirectoryLister::Local(self.fs.clone()).list_directory(query, cx)
        } else if let Some(session) = self.ssh_client.as_ref() {
            let request = proto::ListRemoteDirectory {
                dev_server_id: SSH_PROJECT_ID,
                path: query,
            };

            let response = session.read(cx).to_proto_client().request(request);
            cx.background_executor().spawn(async move {
                let response = response.await?;
                Ok(response.entries.into_iter().map(PathBuf::from).collect())
            })
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
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.create_worktree(abs_path, visible, cx)
        })
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut ModelContext<Self>) {
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.remove_worktree(id_to_remove, cx);
        });
    }

    fn add_worktree(&mut self, worktree: &Model<Worktree>, cx: &mut ModelContext<Self>) {
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.add(worktree, cx);
        });
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
        let remote_worktree_id = worktree.read(cx).id();

        for (path, _, change) in changes.iter() {
            let removed = change == &PathChange::Removed;
            let abs_path = match worktree.read(cx).absolutize(path) {
                Ok(abs_path) => abs_path,
                Err(e) => {
                    log::warn!("Cannot absolutize {path:?} received as {change:?} FS change: {e}");
                    continue;
                }
            };

            if path.ends_with(local_tasks_file_relative_path()) {
                self.task_inventory().update(cx, |task_inventory, cx| {
                    if removed {
                        task_inventory.remove_local_static_source(&abs_path);
                    } else {
                        let fs = self.fs.clone();
                        let task_abs_path = abs_path.clone();
                        let tasks_file_rx =
                            watch_config_file(cx.background_executor(), fs, task_abs_path);
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
                            watch_config_file(cx.background_executor(), fs, task_abs_path);
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
    }

    pub fn set_active_path(&mut self, entry: Option<ProjectPath>, cx: &mut ModelContext<Self>) {
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
        cx: &'a AppContext,
    ) -> impl Iterator<Item = LanguageServerId> + 'a {
        self.lsp_store
            .read(cx)
            .language_servers_running_disk_based_diagnostics()
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
        self.lsp_store
            .read(cx)
            .diagnostic_summaries(include_ignored, cx)
    }

    pub fn active_entry(&self) -> Option<ProjectEntryId> {
        self.active_entry
    }

    pub fn entry_for_path(&self, path: &ProjectPath, cx: &AppContext) -> Option<Entry> {
        self.worktree_store.read(cx).entry_for_path(path, cx)
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

    async fn handle_unshare_project(
        this: Model<Self>,
        _: TypedEnvelope<proto::UnshareProject>,
        mut cx: AsyncAppContext,
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
            this.buffer_store.update(cx, |buffer_store, _| {
                buffer_store.forget_shared_buffers_for(&collaborator.peer_id);
            });
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
                buffer_store.forget_shared_buffers_for(&peer_id);
                for buffer in buffer_store.buffers() {
                    buffer.update(cx, |buffer, cx| buffer.remove_peer(replica_id, cx));
                }
            });

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

    // Collab sends UpdateWorktree protos as messages
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

    async fn handle_update_buffer(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateBuffer>,
        cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        let buffer_store = this.read_with(&cx, |this, cx| {
            if let Some(ssh) = &this.ssh_client {
                let mut payload = envelope.payload.clone();
                payload.project_id = 0;
                cx.background_executor()
                    .spawn(ssh.read(cx).to_proto_client().request(payload))
                    .detach_and_log_err(cx);
            }
            this.buffer_store.clone()
        })?;
        BufferStore::handle_update_buffer(buffer_store, envelope, cx).await
    }

    fn retain_remotely_created_models(
        &mut self,
        cx: &mut ModelContext<Self>,
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

    async fn handle_synchronize_buffers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SynchronizeBuffers>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::SynchronizeBuffersResponse> {
        let response = this.update(&mut cx, |this, cx| {
            let client = this.client.clone();
            this.buffer_store.update(cx, |this, cx| {
                this.handle_synchronize_buffers(envelope, cx, client)
            })
        })??;

        Ok(response)
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

    async fn handle_search_candidate_buffers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidates>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FindSearchCandidatesResponse> {
        let peer_id = envelope.original_sender_id()?;
        let message = envelope.payload;
        let query = SearchQuery::from_proto(
            message
                .query
                .ok_or_else(|| anyhow!("missing query field"))?,
        )?;
        let mut results = this.update(&mut cx, |this, cx| {
            this.find_search_candidate_buffers(&query, message.limit as _, cx)
        })?;

        let mut response = proto::FindSearchCandidatesResponse {
            buffer_ids: Vec::new(),
        };

        while let Some(buffer) = results.next().await {
            this.update(&mut cx, |this, cx| {
                let buffer_id = this.create_buffer_for_peer(&buffer, peer_id, cx);
                response.buffer_ids.push(buffer_id.to_proto());
            })?;
        }

        Ok(response)
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
        let buffer = this
            .update(&mut cx, |this, cx| this.create_buffer(cx))?
            .await?;
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

    fn create_buffer_for_peer(
        &mut self,
        buffer: &Model<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut AppContext,
    ) -> BufferId {
        self.buffer_store
            .update(cx, |buffer_store, cx| {
                buffer_store.create_buffer_for_peer(buffer, peer_id, cx)
            })
            .detach_and_log_err(cx);
        buffer.read(cx).remote_id()
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
        self.worktree_store.read(cx).worktree_metadata_protos(cx)
    }

    fn set_worktrees_from_proto(
        &mut self,
        worktrees: Vec<proto::WorktreeMetadata>,
        cx: &mut ModelContext<Project>,
    ) -> Result<()> {
        cx.notify();
        self.worktree_store.update(cx, |worktree_store, cx| {
            worktree_store.set_worktrees_from_proto(worktrees, self.replica_id(), cx)
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

    pub fn language_servers<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = (LanguageServerId, LanguageServerName, WorktreeId)> {
        self.lsp_store.read(cx).language_servers()
    }

    pub fn supplementary_language_servers<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = (LanguageServerId, LanguageServerName)> {
        self.lsp_store.read(cx).supplementary_language_servers()
    }

    pub fn language_server_for_id(
        &self,
        id: LanguageServerId,
        cx: &AppContext,
    ) -> Option<Arc<LanguageServer>> {
        self.lsp_store.read(cx).language_server_for_id(id)
    }

    pub fn language_servers_for_buffer<'a>(
        &'a self,
        buffer: &'a Buffer,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = (&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        self.lsp_store
            .read(cx)
            .language_servers_for_buffer(buffer, cx)
    }

    pub fn language_server_for_buffer<'a>(
        &'a self,
        buffer: &'a Buffer,
        server_id: LanguageServerId,
        cx: &'a AppContext,
    ) -> Option<(&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        self.lsp_store
            .read(cx)
            .language_server_for_buffer(buffer, server_id, cx)
    }

    pub fn task_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut ModelContext<'_, Project>,
    ) -> Task<Option<TaskContext>> {
        if self.is_local() {
            let (worktree_id, worktree_abs_path) = if let Some(worktree) = self.task_worktree(cx) {
                (
                    Some(worktree.read(cx).id()),
                    Some(worktree.read(cx).abs_path()),
                )
            } else {
                (None, None)
            };

            cx.spawn(|project, mut cx| async move {
                let project_env = project
                    .update(&mut cx, |project, cx| {
                        let worktree_abs_path = worktree_abs_path.clone();
                        project.environment.update(cx, |environment, cx| {
                            environment.get_environment(worktree_id, worktree_abs_path, cx)
                        })
                    })
                    .ok()?
                    .await;

                let mut task_variables = cx
                    .update(|cx| {
                        combine_task_variables(
                            captured_variables,
                            location,
                            project_env.as_ref(),
                            BasicContextProvider::new(project.upgrade()?),
                            cx,
                        )
                        .log_err()
                    })
                    .ok()
                    .flatten()?;
                // Remove all custom entries starting with _, as they're not intended for use by the end user.
                task_variables.sweep();

                Some(TaskContext {
                    project_env: project_env.unwrap_or_default(),
                    cwd: worktree_abs_path.map(|p| p.to_path_buf()),
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
}

fn combine_task_variables(
    mut captured_variables: TaskVariables,
    location: Location,
    project_env: Option<&HashMap<String, String>>,
    baseline: BasicContextProvider,
    cx: &mut AppContext,
) -> anyhow::Result<TaskVariables> {
    let language_context_provider = location
        .buffer
        .read(cx)
        .language()
        .and_then(|language| language.context_provider());
    let baseline = baseline
        .build_context(&captured_variables, &location, project_env, cx)
        .context("building basic default context")?;
    captured_variables.extend(baseline);
    if let Some(provider) = language_context_provider {
        captured_variables.extend(
            provider
                .build_context(&captured_variables, &location, project_env, cx)
                .context("building provider context")?,
        );
    }
    Ok(captured_variables)
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
    ProjectPath(ProjectPath),
    AbsPath(PathBuf),
}

impl ResolvedPath {
    pub fn abs_path(&self) -> Option<&Path> {
        match self {
            Self::AbsPath(path) => Some(path.as_path()),
            _ => None,
        }
    }

    pub fn project_path(&self) -> Option<&ProjectPath> {
        match self {
            Self::ProjectPath(path) => Some(&path),
            _ => None,
        }
    }
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

    /// Returns the corresponding color for this completion.
    ///
    /// Will return `None` if this completion's kind is not [`CompletionItemKind::COLOR`].
    pub fn color(&self) -> Option<Hsla> {
        match self.lsp_completion.kind {
            Some(CompletionItemKind::COLOR) => color_extractor::extract_color(&self.lsp_completion),
            _ => None,
        }
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

pub fn sort_worktree_entries(entries: &mut [Entry]) {
    entries.sort_by(|entry_a, entry_b| {
        compare_paths(
            (&entry_a.path, entry_a.is_file()),
            (&entry_b.path, entry_b.is_file()),
        )
    });
}
