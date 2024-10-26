use crate::{
    buffer_store::{BufferStore, BufferStoreEvent},
    deserialize_code_actions,
    environment::ProjectEnvironment,
    lsp_command::{self, *},
    lsp_ext_command,
    prettier_store::{self, PrettierStore, PrettierStoreEvent},
    project_settings::{LspSettings, ProjectSettings},
    relativize_path, resolve_path,
    toolchain_store::{EmptyToolchainStore, ToolchainStoreEvent},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    yarn::YarnPathStore,
    CodeAction, Completion, CoreCompletion, Hover, InlayHint, Item as _, ProjectPath,
    ProjectTransaction, ResolveState, Symbol, ToolchainStore,
};
use anyhow::{anyhow, Context as _, Result};
use async_trait::async_trait;
use client::{proto, TypedEnvelope};
use collections::{btree_map, BTreeMap, HashMap, HashSet};
use futures::{
    future::{join_all, Shared},
    select,
    stream::FuturesUnordered,
    AsyncWriteExt, Future, FutureExt, StreamExt,
};
use globset::{Glob, GlobSet, GlobSetBuilder};
use gpui::{
    AppContext, AsyncAppContext, Context, Entity, EventEmitter, Model, ModelContext, PromptLevel,
    Task, WeakModel,
};
use http_client::HttpClient;
use language::{
    language_settings::{
        language_settings, FormatOnSave, Formatter, LanguageSettings, SelectedFormatter,
    },
    markdown, point_to_lsp, prepare_completion_documentation,
    proto::{deserialize_anchor, deserialize_version, serialize_anchor, serialize_version},
    range_from_lsp, Bias, Buffer, BufferSnapshot, CachedLspAdapter, CodeLabel, Diagnostic,
    DiagnosticEntry, DiagnosticSet, Diff, Documentation, File as _, Language, LanguageName,
    LanguageRegistry, LanguageServerBinaryStatus, LanguageServerName, LanguageToolchainStore,
    LocalFile, LspAdapter, LspAdapterDelegate, Patch, PointUtf16, TextBufferSnapshot, ToOffset,
    ToPointUtf16, Transaction, Unclipped,
};
use lsp::{
    CodeActionKind, CompletionContext, DiagnosticSeverity, DiagnosticTag,
    DidChangeWatchedFilesRegistrationOptions, Edit, FileSystemWatcher, InsertTextFormat,
    LanguageServer, LanguageServerBinary, LanguageServerBinaryOptions, LanguageServerId,
    LspRequestFuture, MessageActionItem, MessageType, OneOf, ServerHealthStatus, ServerStatus,
    SymbolKind, TextEdit, Url, WorkDoneProgressCancelParams, WorkspaceFolder,
};
use node_runtime::read_package_installed_version;
use parking_lot::{Mutex, RwLock};
use postage::watch;
use rand::prelude::*;

use rpc::AnyProtoClient;
use serde::Serialize;
use settings::{Settings, SettingsLocation, SettingsStore};
use sha2::{Digest, Sha256};
use similar::{ChangeTag, TextDiff};
use smol::channel::Sender;
use snippet::Snippet;
use std::{
    any::Any,
    cmp::Ordering,
    convert::TryInto,
    ffi::OsStr,
    iter, mem,
    ops::{ControlFlow, Range},
    path::{self, Path, PathBuf},
    str,
    sync::Arc,
    time::{Duration, Instant},
};
use text::{Anchor, BufferId, LineEnding, Point, Selection};
use util::{
    debug_panic, defer, maybe, merge_json_value_into, post_inc, ResultExt, TryFutureExt as _,
};

pub use fs::*;
pub use language::Location;
#[cfg(any(test, feature = "test-support"))]
pub use prettier::FORMAT_SUFFIX as TEST_PRETTIER_FORMAT_SUFFIX;
pub use worktree::{
    Entry, EntryKind, File, LocalWorktree, PathChange, ProjectEntryId, RepositoryEntry,
    UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree, WorktreeId, WorktreeSettings,
    FS_WATCH_LATENCY,
};

const SERVER_LAUNCHING_BEFORE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);
pub const SERVER_PROGRESS_THROTTLE_TIMEOUT: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatTrigger {
    Save,
    Manual,
}

pub enum FormatTarget {
    Buffer,
    Ranges(Vec<Selection<Point>>),
}

impl FormatTarget {
    pub fn as_selections(&self) -> Option<&[Selection<Point>]> {
        match self {
            FormatTarget::Buffer => None,
            FormatTarget::Ranges(selections) => Some(selections.as_slice()),
        }
    }
}

// Currently, formatting operations are represented differently depending on
// whether they come from a language server or an external command.
#[derive(Debug)]
pub enum FormatOperation {
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

pub struct LocalLspStore {
    http_client: Arc<dyn HttpClient>,
    environment: Model<ProjectEnvironment>,
    fs: Arc<dyn Fs>,
    yarn: Model<YarnPathStore>,
    pub language_servers: HashMap<LanguageServerId, LanguageServerState>,
    buffers_being_formatted: HashSet<BufferId>,
    last_workspace_edits_by_language_server: HashMap<LanguageServerId, ProjectTransaction>,
    language_server_watched_paths: HashMap<LanguageServerId, Model<LanguageServerWatchedPaths>>,
    language_server_watcher_registrations:
        HashMap<LanguageServerId, HashMap<String, Vec<FileSystemWatcher>>>,
    supplementary_language_servers:
        HashMap<LanguageServerId, (LanguageServerName, Arc<LanguageServer>)>,
    prettier_store: Model<PrettierStore>,
    current_lsp_settings: HashMap<LanguageServerName, LspSettings>,
    last_formatting_failure: Option<String>,
    _subscription: gpui::Subscription,
}

impl LocalLspStore {
    fn shutdown_language_servers(
        &mut self,
        _cx: &mut ModelContext<LspStore>,
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

    async fn format_locally(
        lsp_store: WeakModel<LspStore>,
        mut buffers: Vec<FormattableBuffer>,
        push_to_history: bool,
        trigger: FormatTrigger,
        target: FormatTarget,
        mut cx: AsyncAppContext,
    ) -> anyhow::Result<ProjectTransaction> {
        // Do not allow multiple concurrent formatting requests for the
        // same buffer.
        lsp_store.update(&mut cx, |this, cx| {
            let this = this.as_local_mut().unwrap();
            buffers.retain(|buffer| {
                this.buffers_being_formatted
                    .insert(buffer.handle.read(cx).remote_id())
            });
        })?;

        let _cleanup = defer({
            let this = lsp_store.clone();
            let mut cx = cx.clone();
            let buffers = &buffers;
            move || {
                this.update(&mut cx, |this, cx| {
                    let this = this.as_local_mut().unwrap();
                    for buffer in buffers {
                        this.buffers_being_formatted
                            .remove(&buffer.handle.read(cx).remote_id());
                    }
                })
                .ok();
            }
        });

        let mut project_transaction = ProjectTransaction::default();
        for buffer in &buffers {
            let (primary_adapter_and_server, adapters_and_servers) =
                lsp_store.update(&mut cx, |lsp_store, cx| {
                    let buffer = buffer.handle.read(cx);

                    let adapters_and_servers = lsp_store
                        .language_servers_for_buffer(buffer, cx)
                        .map(|(adapter, lsp)| (adapter.clone(), lsp.clone()))
                        .collect::<Vec<_>>();

                    let primary_adapter = lsp_store
                        .primary_language_server_for_buffer(buffer, cx)
                        .map(|(adapter, lsp)| (adapter.clone(), lsp.clone()));

                    (primary_adapter, adapters_and_servers)
                })?;

            let settings = buffer.handle.update(&mut cx, |buffer, cx| {
                language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx)
                    .into_owned()
            })?;

            let remove_trailing_whitespace = settings.remove_trailing_whitespace_on_save;
            let ensure_final_newline = settings.ensure_final_newline_on_save;

            // First, format buffer's whitespace according to the settings.
            let trailing_whitespace_diff = if remove_trailing_whitespace {
                Some(
                    buffer
                        .handle
                        .update(&mut cx, |b, cx| b.remove_trailing_whitespace(cx))?
                        .await,
                )
            } else {
                None
            };
            let whitespace_transaction_id = buffer.handle.update(&mut cx, |buffer, cx| {
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
                LspStore::execute_code_actions_on_servers(
                    &lsp_store,
                    &adapters_and_servers,
                    code_actions,
                    &buffer.handle,
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
                .zip(buffer.abs_path.as_ref());

            let prettier_settings = buffer.handle.read_with(&cx, |buffer, cx| {
                language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx)
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
                                                    &target,
                                                    server_and_buffer,
                                                    lsp_store.clone(),
                                                    buffer,
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
                                                    &target,
                                                    server_and_buffer,
                                                    lsp_store.clone(),
                                                    buffer,
                                                    &settings,
                                                    &adapters_and_servers,
                                                    push_to_history,
                                                    &mut project_transaction,
                                                    &mut cx,
                                                )
                                                .await
                                            }
                                        }?;

                                        if let Some(op) = diff {
                                            format_operations.push(op);
                                        }
                                    }
                                    SelectedFormatter::List(formatters) => {
                                        for formatter in formatters.as_ref() {
                                            let diff = Self::perform_format(
                                                formatter,
                                                &target,
                                                server_and_buffer,
                                                lsp_store.clone(),
                                                buffer,
                                                &settings,
                                                &adapters_and_servers,
                                                push_to_history,
                                                &mut project_transaction,
                                                &mut cx,
                                            )
                                            .await?;
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
                                        formatter,
                                        &target,
                                        server_and_buffer,
                                        lsp_store.clone(),
                                        buffer,
                                        &settings,
                                        &adapters_and_servers,
                                        push_to_history,
                                        &mut project_transaction,
                                        &mut cx,
                                    )
                                    .await?;
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
                                            &target,
                                            server_and_buffer,
                                            lsp_store.clone(),
                                            buffer,
                                            &settings,
                                            &adapters_and_servers,
                                            push_to_history,
                                            &mut project_transaction,
                                            &mut cx,
                                        )
                                        .await
                                    } else {
                                        let formatter = Formatter::LanguageServer {
                                            name: primary_language_server
                                                .as_ref()
                                                .map(|server| server.name().to_string()),
                                        };
                                        Self::perform_format(
                                            &formatter,
                                            &target,
                                            server_and_buffer,
                                            lsp_store.clone(),
                                            buffer,
                                            &settings,
                                            &adapters_and_servers,
                                            push_to_history,
                                            &mut project_transaction,
                                            &mut cx,
                                        )
                                        .await
                                    }
                                }?;

                                if let Some(op) = diff {
                                    format_operations.push(op)
                                }
                            }
                            SelectedFormatter::List(formatters) => {
                                for formatter in formatters.as_ref() {
                                    // format with formatter
                                    let diff = Self::perform_format(
                                        formatter,
                                        &target,
                                        server_and_buffer,
                                        lsp_store.clone(),
                                        buffer,
                                        &settings,
                                        &adapters_and_servers,
                                        push_to_history,
                                        &mut project_transaction,
                                        &mut cx,
                                    )
                                    .await?;
                                    if let Some(op) = diff {
                                        format_operations.push(op);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            buffer.handle.update(&mut cx, |b, cx| {
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
                    } else if let Some(transaction) = project_transaction.0.get(&buffer.handle) {
                        b.group_until_transaction(transaction.id)
                    }
                }

                if let Some(transaction) = b.finalize_last_transaction().cloned() {
                    if !push_to_history {
                        b.forget_transaction(transaction.id);
                    }
                    project_transaction
                        .0
                        .insert(buffer.handle.clone(), transaction);
                }
            })?;
        }

        Ok(project_transaction)
    }

    #[allow(clippy::too_many_arguments)]
    async fn perform_format(
        formatter: &Formatter,
        format_target: &FormatTarget,
        primary_server_and_buffer: Option<(&Arc<LanguageServer>, &PathBuf)>,
        lsp_store: WeakModel<LspStore>,
        buffer: &FormattableBuffer,
        settings: &LanguageSettings,
        adapters_and_servers: &[(Arc<CachedLspAdapter>, Arc<LanguageServer>)],
        push_to_history: bool,
        transaction: &mut ProjectTransaction,
        cx: &mut AsyncAppContext,
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
                            .unwrap_or(language_server)
                    } else {
                        language_server
                    };

                    match format_target {
                        FormatTarget::Buffer => Some(FormatOperation::Lsp(
                            LspStore::format_via_lsp(
                                &lsp_store,
                                &buffer.handle,
                                buffer_abs_path,
                                language_server,
                                settings,
                                cx,
                            )
                            .await
                            .context("failed to format via language server")?,
                        )),
                        FormatTarget::Ranges(selections) => Some(FormatOperation::Lsp(
                            LspStore::format_range_via_lsp(
                                &lsp_store,
                                &buffer.handle,
                                selections.as_slice(),
                                buffer_abs_path,
                                language_server,
                                settings,
                                cx,
                            )
                            .await
                            .context("failed to format ranges via language server")?,
                        )),
                    }
                } else {
                    None
                }
            }
            Formatter::Prettier => {
                let prettier = lsp_store.update(cx, |lsp_store, _cx| {
                    lsp_store.prettier_store().unwrap().downgrade()
                })?;
                prettier_store::format_with_prettier(&prettier, &buffer.handle, cx)
                    .await
                    .transpose()
                    .ok()
                    .flatten()
            }
            Formatter::External { command, arguments } => {
                Self::format_via_external_command(buffer, command, arguments.as_deref(), cx)
                    .await
                    .context(format!(
                        "failed to format via external command {:?}",
                        command
                    ))?
                    .map(FormatOperation::External)
            }
            Formatter::CodeActions(code_actions) => {
                let code_actions = deserialize_code_actions(code_actions);
                if !code_actions.is_empty() {
                    LspStore::execute_code_actions_on_servers(
                        &lsp_store,
                        adapters_and_servers,
                        code_actions,
                        &buffer.handle,
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

    async fn format_via_external_command(
        buffer: &FormattableBuffer,
        command: &str,
        arguments: Option<&[String]>,
        cx: &mut AsyncAppContext,
    ) -> Result<Option<Diff>> {
        let working_dir_path = buffer.handle.update(cx, |buffer, cx| {
            let file = File::from_dyn(buffer.file())?;
            let worktree = file.worktree.read(cx);
            let mut worktree_path = worktree.abs_path().to_path_buf();
            if worktree.root_entry()?.is_file() {
                worktree_path.pop();
            }
            Some(worktree_path)
        })?;

        let mut child = smol::process::Command::new(command);
        #[cfg(target_os = "windows")]
        {
            use smol::process::windows::CommandExt;
            child.creation_flags(windows::Win32::System::Threading::CREATE_NO_WINDOW.0);
        }

        if let Some(buffer_env) = buffer.env.as_ref() {
            child.envs(buffer_env);
        }

        if let Some(working_dir_path) = working_dir_path {
            child.current_dir(working_dir_path);
        }

        if let Some(arguments) = arguments {
            child.args(arguments.iter().map(|arg| {
                if let Some(buffer_abs_path) = buffer.abs_path.as_ref() {
                    arg.replace("{buffer_path}", &buffer_abs_path.to_string_lossy())
                } else {
                    arg.replace("{buffer_path}", "Untitled")
                }
            }));
        }

        let mut child = child
            .stdin(smol::process::Stdio::piped())
            .stdout(smol::process::Stdio::piped())
            .stderr(smol::process::Stdio::piped())
            .spawn()?;

        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("failed to acquire stdin"))?;
        let text = buffer
            .handle
            .update(cx, |buffer, _| buffer.as_rope().clone())?;
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
                .handle
                .update(cx, |buffer, cx| buffer.diff(stdout, cx))?
                .await,
        ))
    }
}

pub struct FormattableBuffer {
    handle: Model<Buffer>,
    abs_path: Option<PathBuf>,
    env: Option<HashMap<String, String>>,
}

pub struct RemoteLspStore {
    upstream_client: Option<AnyProtoClient>,
    upstream_project_id: u64,
}

#[allow(clippy::large_enum_variant)]
pub enum LspStoreMode {
    Local(LocalLspStore),   // ssh host and collab host
    Remote(RemoteLspStore), // collab guest
}

impl LspStoreMode {
    fn is_local(&self) -> bool {
        matches!(self, LspStoreMode::Local(_))
    }

    fn is_remote(&self) -> bool {
        matches!(self, LspStoreMode::Remote(_))
    }
}

pub struct LspStore {
    mode: LspStoreMode,
    downstream_client: Option<(AnyProtoClient, u64)>,
    nonce: u128,
    buffer_store: Model<BufferStore>,
    worktree_store: Model<WorktreeStore>,
    toolchain_store: Option<Model<ToolchainStore>>,
    buffer_snapshots: HashMap<BufferId, HashMap<LanguageServerId, Vec<LspBufferSnapshot>>>, // buffer_id -> server_id -> vec of snapshots
    pub languages: Arc<LanguageRegistry>,
    language_server_ids: HashMap<(WorktreeId, LanguageServerName), LanguageServerId>,
    pub language_server_statuses: BTreeMap<LanguageServerId, LanguageServerStatus>,
    active_entry: Option<ProjectEntryId>,
    _maintain_workspace_config: (Task<Result<()>>, watch::Sender<()>),
    _maintain_buffer_languages: Task<()>,
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
}

pub enum LspStoreEvent {
    LanguageServerAdded(LanguageServerId, LanguageServerName, Option<WorktreeId>),
    LanguageServerRemoved(LanguageServerId),
    LanguageServerUpdate {
        language_server_id: LanguageServerId,
        message: proto::update_language_server::Variant,
    },
    LanguageServerLog(LanguageServerId, LanguageServerLogType, String),
    LanguageServerPrompt(LanguageServerPromptRequest),
    LanguageDetected {
        buffer: Model<Buffer>,
        new_language: Option<Arc<Language>>,
    },
    Notification(String),
    RefreshInlayHints,
    DiagnosticsUpdated {
        language_server_id: LanguageServerId,
        path: ProjectPath,
    },
    DiskBasedDiagnosticsStarted {
        language_server_id: LanguageServerId,
    },
    DiskBasedDiagnosticsFinished {
        language_server_id: LanguageServerId,
    },
    SnippetEdit {
        buffer_id: BufferId,
        edits: Vec<(lsp::Range, Snippet)>,
        most_recent_edit: clock::Lamport,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct LanguageServerStatus {
    pub name: String,
    pub pending_work: BTreeMap<String, LanguageServerProgress>,
    pub has_pending_diagnostic_updates: bool,
    progress_tokens: HashSet<String>,
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

impl LspStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_model_request_handler(Self::handle_multi_lsp_query);
        client.add_model_request_handler(Self::handle_restart_language_servers);
        client.add_model_message_handler(Self::handle_start_language_server);
        client.add_model_message_handler(Self::handle_update_language_server);
        client.add_model_message_handler(Self::handle_language_server_log);
        client.add_model_message_handler(Self::handle_update_diagnostic_summary);
        client.add_model_request_handler(Self::handle_format_buffers);
        client.add_model_request_handler(Self::handle_resolve_completion_documentation);
        client.add_model_request_handler(Self::handle_apply_code_action);
        client.add_model_request_handler(Self::handle_inlay_hints);
        client.add_model_request_handler(Self::handle_get_project_symbols);
        client.add_model_request_handler(Self::handle_resolve_inlay_hint);
        client.add_model_request_handler(Self::handle_open_buffer_for_symbol);
        client.add_model_request_handler(Self::handle_refresh_inlay_hints);
        client.add_model_request_handler(Self::handle_on_type_formatting);
        client.add_model_request_handler(Self::handle_apply_additional_edits_for_completion);
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
        client.add_model_request_handler(Self::handle_lsp_command::<lsp_ext_command::ExpandMacro>);
        client.add_model_request_handler(Self::handle_lsp_command::<LinkedEditingRange>);
    }

    pub fn as_remote(&self) -> Option<&RemoteLspStore> {
        match &self.mode {
            LspStoreMode::Remote(remote_lsp_store) => Some(remote_lsp_store),
            _ => None,
        }
    }

    pub fn as_local(&self) -> Option<&LocalLspStore> {
        match &self.mode {
            LspStoreMode::Local(local_lsp_store) => Some(local_lsp_store),
            _ => None,
        }
    }

    pub fn as_local_mut(&mut self) -> Option<&mut LocalLspStore> {
        match &mut self.mode {
            LspStoreMode::Local(local_lsp_store) => Some(local_lsp_store),
            _ => None,
        }
    }

    pub fn upstream_client(&self) -> Option<(AnyProtoClient, u64)> {
        match &self.mode {
            LspStoreMode::Remote(RemoteLspStore {
                upstream_client: Some(upstream_client),
                upstream_project_id,
                ..
            }) => Some((upstream_client.clone(), *upstream_project_id)),

            LspStoreMode::Remote(RemoteLspStore {
                upstream_client: None,
                ..
            }) => None,
            LspStoreMode::Local(_) => None,
        }
    }

    pub fn swap_current_lsp_settings(
        &mut self,
        new_settings: HashMap<LanguageServerName, LspSettings>,
    ) -> Option<HashMap<LanguageServerName, LspSettings>> {
        match &mut self.mode {
            LspStoreMode::Local(LocalLspStore {
                current_lsp_settings,
                ..
            }) => {
                let ret = mem::take(current_lsp_settings);
                *current_lsp_settings = new_settings;
                Some(ret)
            }
            LspStoreMode::Remote(_) => None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_local(
        buffer_store: Model<BufferStore>,
        worktree_store: Model<WorktreeStore>,
        prettier_store: Model<PrettierStore>,
        toolchain_store: Model<ToolchainStore>,
        environment: Model<ProjectEnvironment>,
        languages: Arc<LanguageRegistry>,
        http_client: Arc<dyn HttpClient>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let yarn = YarnPathStore::new(fs.clone(), cx);
        cx.subscribe(&buffer_store, Self::on_buffer_store_event)
            .detach();
        cx.subscribe(&worktree_store, Self::on_worktree_store_event)
            .detach();
        cx.subscribe(&prettier_store, Self::on_prettier_store_event)
            .detach();
        cx.subscribe(&toolchain_store, Self::on_toolchain_store_event)
            .detach();
        cx.observe_global::<SettingsStore>(Self::on_settings_changed)
            .detach();

        let _maintain_workspace_config = {
            let (sender, receiver) = watch::channel();
            (Self::maintain_workspace_config(receiver, cx), sender)
        };
        Self {
            mode: LspStoreMode::Local(LocalLspStore {
                supplementary_language_servers: Default::default(),
                language_servers: Default::default(),
                last_workspace_edits_by_language_server: Default::default(),
                language_server_watched_paths: Default::default(),
                language_server_watcher_registrations: Default::default(),
                current_lsp_settings: ProjectSettings::get_global(cx).lsp.clone(),
                buffers_being_formatted: Default::default(),
                last_formatting_failure: None,
                prettier_store,
                environment,
                http_client,
                fs,
                yarn,
                _subscription: cx.on_app_quit(|this, cx| {
                    this.as_local_mut().unwrap().shutdown_language_servers(cx)
                }),
            }),
            downstream_client: None,
            buffer_store,
            worktree_store,
            toolchain_store: Some(toolchain_store),
            languages: languages.clone(),
            language_server_ids: Default::default(),
            language_server_statuses: Default::default(),
            nonce: StdRng::from_entropy().gen(),
            buffer_snapshots: Default::default(),
            next_diagnostic_group_id: Default::default(),
            diagnostic_summaries: Default::default(),
            diagnostics: Default::default(),
            active_entry: None,

            _maintain_workspace_config,
            _maintain_buffer_languages: Self::maintain_buffer_languages(languages.clone(), cx),
        }
    }

    fn send_lsp_proto_request<R: LspCommand>(
        &self,
        buffer: Model<Buffer>,
        client: AnyProtoClient,
        upstream_project_id: u64,
        request: R,
        cx: &mut ModelContext<'_, LspStore>,
    ) -> Task<anyhow::Result<<R as LspCommand>::Response>> {
        let message = request.to_proto(upstream_project_id, buffer.read(cx));
        cx.spawn(move |this, cx| async move {
            let response = client.request(message).await?;
            let this = this.upgrade().context("project dropped")?;
            request
                .response_from_proto(response, this, buffer, cx)
                .await
        })
    }

    pub(super) fn new_remote(
        buffer_store: Model<BufferStore>,
        worktree_store: Model<WorktreeStore>,
        toolchain_store: Option<Model<ToolchainStore>>,
        languages: Arc<LanguageRegistry>,
        upstream_client: AnyProtoClient,
        project_id: u64,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        cx.subscribe(&buffer_store, Self::on_buffer_store_event)
            .detach();
        cx.subscribe(&worktree_store, Self::on_worktree_store_event)
            .detach();
        let _maintain_workspace_config = {
            let (sender, receiver) = watch::channel();
            (Self::maintain_workspace_config(receiver, cx), sender)
        };
        Self {
            mode: LspStoreMode::Remote(RemoteLspStore {
                upstream_client: Some(upstream_client),
                upstream_project_id: project_id,
            }),
            downstream_client: None,
            buffer_store,
            worktree_store,
            languages: languages.clone(),
            language_server_ids: Default::default(),
            language_server_statuses: Default::default(),
            nonce: StdRng::from_entropy().gen(),
            buffer_snapshots: Default::default(),
            next_diagnostic_group_id: Default::default(),
            diagnostic_summaries: Default::default(),
            diagnostics: Default::default(),
            active_entry: None,
            toolchain_store,
            _maintain_workspace_config,
            _maintain_buffer_languages: Self::maintain_buffer_languages(languages.clone(), cx),
        }
    }

    fn worktree_for_id(
        &self,
        worktree_id: WorktreeId,
        cx: &ModelContext<Self>,
    ) -> Result<Model<Worktree>> {
        self.worktree_store
            .read(cx)
            .worktree_for_id(worktree_id, cx)
            .ok_or_else(|| anyhow!("worktree not found"))
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
                    self.unregister_buffer_from_language_servers(buffer, old_file, cx);
                }

                self.register_buffer_with_language_servers(buffer, cx);
            }
            BufferStoreEvent::BufferDropped(_) => {}
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
                if !worktree.read(cx).is_local() {
                    return;
                }
                cx.subscribe(worktree, |this, worktree, event, cx| match event {
                    worktree::Event::UpdatedEntries(changes) => {
                        this.update_local_worktree_language_servers(&worktree, changes, cx);
                    }
                    worktree::Event::UpdatedGitRepositories(_)
                    | worktree::Event::DeletedEntry(_) => {}
                })
                .detach()
            }
            WorktreeStoreEvent::WorktreeReleased(..) => {}
            WorktreeStoreEvent::WorktreeRemoved(_, id) => self.remove_worktree(*id, cx),
            WorktreeStoreEvent::WorktreeOrderChanged => {}
            WorktreeStoreEvent::WorktreeUpdateSent(worktree) => {
                worktree.update(cx, |worktree, _cx| self.send_diagnostic_summaries(worktree));
            }
        }
    }

    fn on_prettier_store_event(
        &mut self,
        _: Model<PrettierStore>,
        event: &PrettierStoreEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            PrettierStoreEvent::LanguageServerRemoved(prettier_server_id) => {
                self.unregister_supplementary_language_server(*prettier_server_id, cx);
            }
            PrettierStoreEvent::LanguageServerAdded {
                new_server_id,
                name,
                prettier_server,
            } => {
                self.register_supplementary_language_server(
                    *new_server_id,
                    name.clone(),
                    prettier_server.clone(),
                    cx,
                );
            }
        }
    }

    fn on_toolchain_store_event(
        &mut self,
        _: Model<ToolchainStore>,
        event: &ToolchainStoreEvent,
        _: &mut ModelContext<Self>,
    ) {
        match event {
            ToolchainStoreEvent::ToolchainActivated { .. } => {
                self.request_workspace_config_refresh()
            }
        }
    }

    fn request_workspace_config_refresh(&mut self) {
        *self._maintain_workspace_config.1.borrow_mut() = ();
    }
    // todo!
    pub fn prettier_store(&self) -> Option<Model<PrettierStore>> {
        self.as_local().map(|local| local.prettier_store.clone())
    }

    fn on_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &language::BufferEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            language::BufferEvent::Edited { .. } => {
                self.on_buffer_edited(buffer, cx);
            }

            language::BufferEvent::Saved => {
                self.on_buffer_saved(buffer, cx);
            }

            _ => {}
        }
    }

    fn register_buffer(
        &mut self,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        buffer.update(cx, |buffer, _| {
            buffer.set_language_registry(self.languages.clone())
        });

        cx.subscribe(buffer, |this, buffer, event, cx| {
            this.on_buffer_event(buffer, event, cx);
        })
        .detach();

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

    fn maintain_buffer_languages(
        languages: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Task<()> {
        let mut subscription = languages.subscribe();
        let mut prev_reload_count = languages.reload_count();
        cx.spawn(move |this, mut cx| async move {
            while let Some(()) = subscription.next().await {
                if let Some(this) = this.upgrade() {
                    // If the language registry has been reloaded, then remove and
                    // re-assign the languages on all open buffers.
                    let reload_count = languages.reload_count();
                    if reload_count > prev_reload_count {
                        prev_reload_count = reload_count;
                        this.update(&mut cx, |this, cx| {
                            this.buffer_store.clone().update(cx, |buffer_store, cx| {
                                for buffer in buffer_store.buffers() {
                                    if let Some(f) = File::from_dyn(buffer.read(cx).file()).cloned()
                                    {
                                        this.unregister_buffer_from_language_servers(
                                            &buffer, &f, cx,
                                        );
                                        buffer
                                            .update(cx, |buffer, cx| buffer.set_language(None, cx));
                                    }
                                }
                            });
                        })
                        .ok();
                    }

                    this.update(&mut cx, |this, cx| {
                        let mut plain_text_buffers = Vec::new();
                        let mut buffers_with_unknown_injections = Vec::new();
                        for handle in this.buffer_store.read(cx).buffers() {
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
                            this.register_buffer_with_language_servers(&buffer, cx);
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

    fn detect_language_for_buffer(
        &mut self,
        buffer_handle: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Option<language::AvailableLanguage> {
        // If the buffer has a language, set it and start the language server if we haven't already.
        let buffer = buffer_handle.read(cx);
        let file = buffer.file()?;

        let content = buffer.as_rope();
        let available_language = self.languages.language_for_file(file, Some(content), cx);
        if let Some(available_language) = &available_language {
            if let Some(Ok(Ok(new_language))) = self
                .languages
                .load_language(available_language)
                .now_or_never()
            {
                self.set_language_for_buffer(buffer_handle, new_language, cx);
            }
        } else {
            cx.emit(LspStoreEvent::LanguageDetected {
                buffer: buffer_handle.clone(),
                new_language: None,
            });
        }

        available_language
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
        let settings =
            language_settings(Some(new_language.name()), buffer_file.as_ref(), cx).into_owned();
        let buffer_file = File::from_dyn(buffer_file.as_ref());

        let worktree_id = if let Some(file) = buffer_file {
            let worktree = file.worktree.clone();
            self.start_language_servers(&worktree, new_language.name(), cx);

            Some(worktree.read(cx).id())
        } else {
            None
        };

        if settings.prettier.allowed {
            if let Some(prettier_plugins) = prettier_store::prettier_plugins_for_language(&settings)
            {
                let prettier_store = self.as_local().map(|s| s.prettier_store.clone());
                if let Some(prettier_store) = prettier_store {
                    prettier_store.update(cx, |prettier_store, cx| {
                        prettier_store.install_default_prettier(
                            worktree_id,
                            prettier_plugins.iter().map(|s| Arc::from(s.as_str())),
                            cx,
                        )
                    })
                }
            }
        }

        cx.emit(LspStoreEvent::LanguageDetected {
            buffer: buffer.clone(),
            new_language: Some(new_language),
        })
    }

    pub fn buffer_store(&self) -> Model<BufferStore> {
        self.buffer_store.clone()
    }

    pub fn set_active_entry(&mut self, active_entry: Option<ProjectEntryId>) {
        self.active_entry = active_entry;
    }

    pub(crate) fn send_diagnostic_summaries(&self, worktree: &mut Worktree) {
        if let Some((client, downstream_project_id)) = self.downstream_client.clone() {
            if let Some(summaries) = self.diagnostic_summaries.get(&worktree.id()) {
                for (path, summaries) in summaries {
                    for (&server_id, summary) in summaries {
                        client
                            .send(proto::UpdateDiagnosticSummary {
                                project_id: downstream_project_id,
                                worktree_id: worktree.id().to_proto(),
                                summary: Some(summary.to_proto(server_id, path)),
                            })
                            .log_err();
                    }
                }
            }
        }
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

        if let Some((upstream_client, upstream_project_id)) = self.upstream_client() {
            return self.send_lsp_proto_request(
                buffer_handle,
                upstream_client,
                upstream_project_id,
                request,
                cx,
            );
        }

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

        Task::ready(Ok(Default::default()))
    }

    fn on_settings_changed(&mut self, cx: &mut ModelContext<Self>) {
        let mut language_servers_to_start = Vec::new();
        let mut language_formatters_to_check = Vec::new();
        for buffer in self.buffer_store.read(cx).buffers() {
            let buffer = buffer.read(cx);
            let buffer_file = File::from_dyn(buffer.file());
            let buffer_language = buffer.language();
            let settings = language_settings(buffer_language.map(|l| l.name()), buffer.file(), cx);
            if let Some(language) = buffer_language {
                if settings.enable_language_server {
                    if let Some(file) = buffer_file {
                        language_servers_to_start.push((file.worktree.clone(), language.name()));
                    }
                }
                language_formatters_to_check.push((
                    buffer_file.map(|f| f.worktree_id(cx)),
                    settings.into_owned(),
                ));
            }
        }

        let mut language_servers_to_stop = Vec::new();
        let mut language_servers_to_restart = Vec::new();
        let languages = self.languages.to_vec();

        let new_lsp_settings = ProjectSettings::get_global(cx).lsp.clone();
        let Some(current_lsp_settings) = self.swap_current_lsp_settings(new_lsp_settings.clone())
        else {
            return;
        };
        for (worktree_id, started_lsp_name) in self.started_language_servers() {
            let language = languages.iter().find_map(|l| {
                let adapter = self
                    .languages
                    .lsp_adapters(&l.name())
                    .iter()
                    .find(|adapter| adapter.name == started_lsp_name)?
                    .clone();
                Some((l, adapter))
            });
            if let Some((language, adapter)) = language {
                let worktree = self.worktree_for_id(worktree_id, cx).ok();
                let root_file = worktree.as_ref().and_then(|worktree| {
                    worktree
                        .update(cx, |tree, cx| tree.root_file(cx))
                        .map(|f| f as _)
                });
                let settings = language_settings(Some(language.name()), root_file.as_ref(), cx);
                if !settings.enable_language_server {
                    language_servers_to_stop.push((worktree_id, started_lsp_name.clone()));
                } else if let Some(worktree) = worktree {
                    let server_name = &adapter.name;
                    match (
                        current_lsp_settings.get(server_name),
                        new_lsp_settings.get(server_name),
                    ) {
                        (None, None) => {}
                        (Some(_), None) | (None, Some(_)) => {
                            language_servers_to_restart.push((worktree, language.name()));
                        }
                        (Some(current_lsp_settings), Some(new_lsp_settings)) => {
                            if current_lsp_settings != new_lsp_settings {
                                language_servers_to_restart.push((worktree, language.name()));
                            }
                        }
                    }
                }
            }
        }

        for (worktree_id, adapter_name) in language_servers_to_stop {
            self.stop_local_language_server(worktree_id, adapter_name, cx)
                .detach();
        }

        if let Some(prettier_store) = self.as_local().map(|s| s.prettier_store.clone()) {
            prettier_store.update(cx, |prettier_store, cx| {
                prettier_store.on_settings_changed(language_formatters_to_check, cx)
            })
        }

        // Start all the newly-enabled language servers.
        for (worktree, language) in language_servers_to_start {
            self.start_language_servers(&worktree, language, cx);
        }

        // Restart all language servers with changed initialization options.
        for (worktree, language) in language_servers_to_restart {
            self.restart_local_language_servers(worktree, language, cx);
        }

        cx.notify();
    }

    pub async fn execute_code_actions_on_servers(
        this: &WeakModel<LspStore>,
        adapters_and_servers: &[(Arc<CachedLspAdapter>, Arc<LanguageServer>)],
        code_actions: Vec<lsp::CodeActionKind>,
        buffer: &Model<Buffer>,
        push_to_history: bool,
        project_transaction: &mut ProjectTransaction,
        cx: &mut AsyncAppContext,
    ) -> Result<(), anyhow::Error> {
        for (lsp_adapter, language_server) in adapters_and_servers.iter() {
            let code_actions = code_actions.clone();

            let actions = this
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
                Self::try_resolve_code_action(language_server, &mut action)
                    .await
                    .context("resolving a formatting code action")?;

                if let Some(edit) = action.lsp_action.edit {
                    if edit.changes.is_none() && edit.document_changes.is_none() {
                        continue;
                    }

                    let new = Self::deserialize_workspace_edit(
                        this.upgrade().ok_or_else(|| anyhow!("project dropped"))?,
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
                    this.update(cx, |this, _| {
                        if let LspStoreMode::Local(mode) = &mut this.mode {
                            mode.last_workspace_edits_by_language_server
                                .remove(&language_server.server_id());
                        }
                    })?;

                    language_server
                        .request::<lsp::request::ExecuteCommand>(lsp::ExecuteCommandParams {
                            command: command.command,
                            arguments: command.arguments.unwrap_or_default(),
                            ..Default::default()
                        })
                        .await?;

                    this.update(cx, |this, _| {
                        if let LspStoreMode::Local(mode) = &mut this.mode {
                            project_transaction.0.extend(
                                mode.last_workspace_edits_by_language_server
                                    .remove(&language_server.server_id())
                                    .unwrap_or_default()
                                    .0,
                            )
                        }
                    })?;
                }
            }
        }

        Ok(())
    }

    async fn try_resolve_code_action(
        lang_server: &LanguageServer,
        action: &mut CodeAction,
    ) -> anyhow::Result<()> {
        if GetCodeActions::can_resolve_actions(&lang_server.capabilities())
            && action.lsp_action.data.is_some()
            && (action.lsp_action.command.is_none() || action.lsp_action.edit.is_none())
        {
            action.lsp_action = lang_server
                .request::<lsp::request::CodeActionResolveRequest>(action.lsp_action.clone())
                .await?;
        }

        anyhow::Ok(())
    }

    pub fn apply_code_action(
        &self,
        buffer_handle: Model<Buffer>,
        mut action: CodeAction,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = proto::ApplyCodeAction {
                project_id,
                buffer_id: buffer_handle.read(cx).remote_id().into(),
                action: Some(Self::serialize_code_action(&action)),
            };
            let buffer_store = self.buffer_store();
            cx.spawn(move |_, mut cx| async move {
                let response = upstream_client
                    .request(request)
                    .await?
                    .transaction
                    .ok_or_else(|| anyhow!("missing transaction"))?;

                buffer_store
                    .update(&mut cx, |buffer_store, cx| {
                        buffer_store.deserialize_project_transaction(response, push_to_history, cx)
                    })?
                    .await
            })
        } else {
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
                        this.as_local_mut()
                            .unwrap()
                            .last_workspace_edits_by_language_server
                            .remove(&lang_server.server_id());
                    })?;

                    let result = lang_server
                        .request::<lsp::request::ExecuteCommand>(lsp::ExecuteCommandParams {
                            command: command.command,
                            arguments: command.arguments.unwrap_or_default(),
                            ..Default::default()
                        })
                        .await;

                    result?;

                    return this.update(&mut cx, |this, _| {
                        this.as_local_mut()
                            .unwrap()
                            .last_workspace_edits_by_language_server
                            .remove(&lang_server.server_id())
                            .unwrap_or_default()
                    });
                }

                Ok(ProjectTransaction::default())
            })
        }
    }

    pub fn resolve_inlay_hint(
        &self,
        hint: InlayHint,
        buffer_handle: Model<Buffer>,
        server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<InlayHint>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = proto::ResolveInlayHint {
                project_id,
                buffer_id: buffer_handle.read(cx).remote_id().into(),
                language_server_id: server_id.0 as u64,
                hint: Some(InlayHints::project_to_proto_hint(hint.clone())),
            };
            cx.spawn(move |_, _| async move {
                let response = upstream_client
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
        }
    }

    pub(crate) fn linked_edit(
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
            .or_else(|| {
                self.upstream_client()
                    .is_some()
                    .then_some(LanguageServerToQuery::Primary)
            })
            .filter(|_| {
                maybe!({
                    let language = buffer.read(cx).language_at(position)?;
                    Some(
                        language_settings(Some(language.name()), buffer.read(cx).file(), cx)
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

    fn apply_on_type_formatting(
        &mut self,
        buffer: Model<Buffer>,
        position: Anchor,
        trigger: String,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        if let Some((client, project_id)) = self.upstream_client() {
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
        } else if let Some(local) = self.as_local_mut() {
            let buffer_id = buffer.read(cx).remote_id();
            local.buffers_being_formatted.insert(buffer_id);
            cx.spawn(move |this, mut cx| async move {
                let _cleanup = defer({
                    let this = this.clone();
                    let mut cx = cx.clone();
                    move || {
                        this.update(&mut cx, |this, _| {
                            if let Some(local) = this.as_local_mut() {
                                local.buffers_being_formatted.remove(&buffer_id);
                            }
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
        } else {
            Task::ready(Err(anyhow!("No upstream client or local language server")))
        }
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

    fn on_type_format_impl(
        &mut self,
        buffer: Model<Buffer>,
        position: PointUtf16,
        trigger: String,
        push_to_history: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        let options = buffer.update(cx, |buffer, cx| {
            lsp_command::lsp_formatting_options(
                language_settings(
                    buffer.language_at(position).map(|l| l.name()),
                    buffer.file(),
                    cx,
                )
                .as_ref(),
            )
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

    pub async fn format_via_lsp(
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
                    text_document: text_document.clone(),
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
            Ok(Vec::with_capacity(0))
        }
    }
    pub async fn format_range_via_lsp(
        this: &WeakModel<Self>,
        buffer: &Model<Buffer>,
        selections: &[Selection<Point>],
        abs_path: &Path,
        language_server: &Arc<LanguageServer>,
        settings: &LanguageSettings,
        cx: &mut AsyncAppContext,
    ) -> Result<Vec<(Range<Anchor>, String)>> {
        let capabilities = &language_server.capabilities();
        let range_formatting_provider = capabilities.document_range_formatting_provider.as_ref();
        if range_formatting_provider.map_or(false, |provider| provider == &OneOf::Left(false)) {
            return Err(anyhow!(
                "{} language server does not support range formatting",
                language_server.name()
            ));
        }

        let uri = lsp::Url::from_file_path(abs_path)
            .map_err(|_| anyhow!("failed to convert abs path to uri"))?;
        let text_document = lsp::TextDocumentIdentifier::new(uri);

        let lsp_edits = {
            let ranges = selections.into_iter().map(|s| {
                let start = lsp::Position::new(s.start.row, s.start.column);
                let end = lsp::Position::new(s.end.row, s.end.column);
                lsp::Range::new(start, end)
            });

            let mut edits = None;
            for range in ranges {
                if let Some(mut edit) = language_server
                    .request::<lsp::request::RangeFormatting>(lsp::DocumentRangeFormattingParams {
                        text_document: text_document.clone(),
                        range,
                        options: lsp_command::lsp_formatting_options(settings),
                        work_done_progress_params: Default::default(),
                    })
                    .await?
                {
                    edits.get_or_insert_with(Vec::new).append(&mut edit);
                }
            }
            edits
        };

        if let Some(lsp_edits) = lsp_edits {
            this.update(cx, |this, cx| {
                this.edits_from_lsp(buffer, lsp_edits, language_server.server_id(), None, cx)
            })?
            .await
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    pub fn code_actions(
        &mut self,
        buffer_handle: &Model<Buffer>,
        range: Range<Anchor>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request_task = upstream_client.request(proto::MultiLspQuery {
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
                    return Ok(Vec::new());
                };
                let responses = request_task.await?.responses;
                let actions = join_all(
                    responses
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
                            GetCodeActions {
                                range: range.clone(),
                                kinds: None,
                            }
                            .response_from_proto(
                                code_actions_response,
                                project.clone(),
                                buffer.clone(),
                                cx.clone(),
                            )
                        }),
                )
                .await;

                Ok(actions
                    .into_iter()
                    .collect::<Result<Vec<Vec<_>>>>()?
                    .into_iter()
                    .flatten()
                    .collect())
            })
        } else {
            let all_actions_task = self.request_multiple_lsp_locally(
                buffer_handle,
                Some(range.start),
                GetCodeActions {
                    range: range.clone(),
                    kinds: None,
                },
                cx,
            );
            cx.spawn(
                |_, _| async move { Ok(all_actions_task.await.into_iter().flatten().collect()) },
            )
        }
    }

    #[inline(never)]
    pub fn completions(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        context: CompletionContext,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Completion>>> {
        let language_registry = self.languages.clone();

        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let task = self.send_lsp_proto_request(
                buffer.clone(),
                upstream_client,
                project_id,
                GetCompletions { position, context },
                cx,
            );
            let language = buffer.read(cx).language().cloned();

            // In the future, we should provide project guests with the names of LSP adapters,
            // so that they can use the correct LSP adapter when computing labels. For now,
            // guests just use the first LSP adapter associated with the buffer's language.
            let lsp_adapter = language.as_ref().and_then(|language| {
                language_registry
                    .lsp_adapters(&language.name())
                    .first()
                    .cloned()
            });

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
        }
    }

    pub fn resolve_completions(
        &self,
        buffer: Model<Buffer>,
        completion_indices: Vec<usize>,
        completions: Arc<RwLock<Box<[Completion]>>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<bool>> {
        let client = self.upstream_client();
        let language_registry = self.languages.clone();

        let buffer_id = buffer.read(cx).remote_id();
        let buffer_snapshot = buffer.read(cx).snapshot();

        cx.spawn(move |this, cx| async move {
            let mut did_resolve = false;
            if let Some((client, project_id)) = client {
                for completion_index in completion_indices {
                    let (server_id, completion) = {
                        let completions_guard = completions.read();
                        let completion = &completions_guard[completion_index];
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
                        let server_id = completion.server_id;
                        let completion = completion.lsp_completion.clone();

                        (server_id, completion)
                    };

                    let server = this
                        .read_with(&cx, |this, _| this.language_server_for_id(server_id))
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

        let mut completions = completions.write();
        let completion = &mut completions[completion_index];
        completion.lsp_completion = completion_item;
    }

    #[allow(clippy::too_many_arguments)]
    async fn resolve_completion_remote(
        project_id: u64,
        server_id: LanguageServerId,
        buffer_id: BufferId,
        completions: Arc<RwLock<Box<[Completion]>>>,
        completion_index: usize,
        completion: lsp::CompletionItem,
        client: AnyProtoClient,
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
        let Some(lsp_completion) = serde_json::from_slice(&response.lsp_completion).log_err()
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
        completion.lsp_completion = lsp_completion;

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

        if let Some((client, project_id)) = self.upstream_client() {
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
        }
    }

    pub fn inlay_hints(
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

        if let Some((client, project_id)) = self.upstream_client() {
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
        }
    }

    pub fn signature_help<T: ToPointUtf16>(
        &self,
        buffer: &Model<Buffer>,
        position: T,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<SignatureHelp>> {
        let position = position.to_point_utf16(buffer.read(cx));

        if let Some((client, upstream_project_id)) = self.upstream_client() {
            let request_task = client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id: upstream_project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetSignatureHelp(
                    GetSignatureHelp { position }.to_proto(upstream_project_id, buffer.read(cx)),
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
        }
    }

    pub fn hover(
        &self,
        buffer: &Model<Buffer>,
        position: PointUtf16,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<Hover>> {
        if let Some((client, upstream_project_id)) = self.upstream_client() {
            let request_task = client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id: upstream_project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetHover(
                    GetHover { position }.to_proto(upstream_project_id, buffer.read(cx)),
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
            let all_actions_task = self.request_multiple_lsp_locally(
                buffer,
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
        }
    }

    pub fn symbols(&self, query: &str, cx: &mut ModelContext<Self>) -> Task<Result<Vec<Symbol>>> {
        let language_registry = self.languages.clone();

        if let Some((upstream_client, project_id)) = self.upstream_client().as_ref() {
            let request = upstream_client.request(proto::GetProjectSymbols {
                project_id: *project_id,
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
            struct WorkspaceSymbolsResult {
                lsp_adapter: Arc<CachedLspAdapter>,
                language: LanguageName,
                worktree: WeakModel<Worktree>,
                worktree_abs_path: Arc<Path>,
                lsp_symbols: Vec<(String, SymbolKind, lsp::Location)>,
            }

            let mut requests = Vec::new();
            for ((worktree_id, _), server_id) in self.language_server_ids.iter() {
                let Some(worktree_handle) = self
                    .worktree_store
                    .read(cx)
                    .worktree_for_id(*worktree_id, cx)
                else {
                    continue;
                };
                let worktree = worktree_handle.read(cx);
                if !worktree.is_visible() {
                    continue;
                }
                let worktree_abs_path = worktree.abs_path().clone();

                let (lsp_adapter, language, server) =
                    match self.as_local().unwrap().language_servers.get(server_id) {
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
                                                    log::error!("Unexpected: client capabilities forbid symbol resolutions in workspace.symbol.resolveSupport");
                                                    return None
                                                }
                                            };
                                            Some((lsp_symbol.name, lsp_symbol.kind, location))
                                        }).collect::<Vec<_>>()
                                    }
                                }).unwrap_or_default();

                                WorkspaceSymbolsResult {
                                    lsp_adapter,
                                    language,
                                    worktree: worktree_handle.downgrade(),
                                    worktree_abs_path,
                                    lsp_symbols,
                                }
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
                for result in responses {
                    let core_symbols = this.update(&mut cx, |this, cx| {
                        result
                            .lsp_symbols
                            .into_iter()
                            .filter_map(|(symbol_name, symbol_kind, symbol_location)| {
                                let abs_path = symbol_location.uri.to_file_path().ok()?;
                                let source_worktree = result.worktree.upgrade()?;
                                let source_worktree_id = source_worktree.read(cx).id();

                                let path;
                                let worktree;
                                if let Some((tree, rel_path)) =
                                    this.worktree_store.read(cx).find_worktree(&abs_path, cx)
                                {
                                    worktree = tree;
                                    path = rel_path;
                                } else {
                                    worktree = source_worktree.clone();
                                    path = relativize_path(&result.worktree_abs_path, &abs_path);
                                }

                                let worktree_id = worktree.read(cx).id();
                                let project_path = ProjectPath {
                                    worktree_id,
                                    path: path.into(),
                                };
                                let signature = this.symbol_signature(&project_path);
                                Some(CoreSymbol {
                                    language_server_name: result.lsp_adapter.name.clone(),
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
                        Some(result.language),
                        Some(result.lsp_adapter),
                        &mut symbols,
                    )
                    .await;
                }

                Ok(symbols)
            })
        }
    }

    pub fn diagnostic_summaries<'a>(
        &'a self,
        include_ignored: bool,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = (ProjectPath, LanguageServerId, DiagnosticSummary)> + 'a {
        self.worktree_store
            .read(cx)
            .visible_worktrees(cx)
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

    pub fn started_language_servers(&self) -> Vec<(WorktreeId, LanguageServerName)> {
        self.language_server_ids.keys().cloned().collect()
    }

    pub fn on_buffer_edited(
        &mut self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
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

        None
    }

    pub fn on_buffer_saved(
        &mut self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let file = File::from_dyn(buffer.read(cx).file())?;
        let worktree_id = file.worktree_id(cx);
        let abs_path = file.as_local()?.abs_path(cx);
        let text_document = lsp::TextDocumentIdentifier {
            uri: lsp::Url::from_file_path(abs_path).log_err()?,
        };

        for server in self.language_servers_for_worktree(worktree_id) {
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

        None
    }

    pub(crate) async fn refresh_workspace_configurations(
        this: &WeakModel<Self>,
        mut cx: AsyncAppContext,
    ) {
        maybe!(async move {
            let servers = this
                .update(&mut cx, |this, cx| {
                    this.language_server_ids
                        .iter()
                        .filter_map(|((worktree_id, _), server_id)| {
                            let worktree = this
                                .worktree_store
                                .read(cx)
                                .worktree_for_id(*worktree_id, cx)?;
                            let state = this.as_local()?.language_servers.get(server_id)?;
                            let delegate = LocalLspAdapterDelegate::for_local(this, &worktree, cx);
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
                })
                .ok()?;

            let toolchain_store = this
                .update(&mut cx, |this, cx| this.toolchain_store(cx))
                .ok()?;
            for (adapter, server, delegate) in servers {
                let settings = adapter
                    .workspace_configuration(&delegate, toolchain_store.clone(), &mut cx)
                    .await
                    .ok()?;

                server
                    .notify::<lsp::notification::DidChangeConfiguration>(
                        lsp::DidChangeConfigurationParams { settings },
                    )
                    .ok();
            }
            Some(())
        })
        .await;
    }

    fn toolchain_store(&self, cx: &AppContext) -> Arc<dyn LanguageToolchainStore> {
        if let Some(toolchain_store) = self.toolchain_store.as_ref() {
            toolchain_store.read(cx).as_language_toolchain_store()
        } else {
            Arc::new(EmptyToolchainStore)
        }
    }
    fn maintain_workspace_config(
        external_refresh_requests: watch::Receiver<()>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let (mut settings_changed_tx, mut settings_changed_rx) = watch::channel();
        let _ = postage::stream::Stream::try_recv(&mut settings_changed_rx);

        let settings_observation = cx.observe_global::<SettingsStore>(move |_, _| {
            *settings_changed_tx.borrow_mut() = ();
        });

        let mut joint_future =
            futures::stream::select(settings_changed_rx, external_refresh_requests);
        cx.spawn(move |this, cx| async move {
            while let Some(()) = joint_future.next().await {
                Self::refresh_workspace_configurations(&this, cx.clone()).await;
            }

            drop(settings_observation);
            anyhow::Ok(())
        })
    }

    fn primary_language_server_for_buffer<'a>(
        &'a self,
        buffer: &'a Buffer,
        cx: &'a AppContext,
    ) -> Option<(&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        // The list of language servers is ordered based on the `language_servers` setting
        // for each language, thus we can consider the first one in the list to be the
        // primary one.
        self.language_servers_for_buffer(buffer, cx).next()
    }

    pub fn language_server_for_buffer<'a>(
        &'a self,
        buffer: &'a Buffer,
        server_id: LanguageServerId,
        cx: &'a AppContext,
    ) -> Option<(&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        self.language_servers_for_buffer(buffer, cx)
            .find(|(_, s)| s.server_id() == server_id)
    }

    fn language_servers_for_worktree(
        &self,
        worktree_id: WorktreeId,
    ) -> impl Iterator<Item = &Arc<LanguageServer>> {
        self.language_server_ids
            .iter()
            .filter_map(move |((language_server_worktree_id, _), id)| {
                if *language_server_worktree_id == worktree_id {
                    if let Some(LanguageServerState::Running { server, .. }) =
                        self.as_local()?.language_servers.get(id)
                    {
                        return Some(server);
                    }
                }
                None
            })
    }

    fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut ModelContext<Self>) {
        self.diagnostics.remove(&id_to_remove);
        self.diagnostic_summaries.remove(&id_to_remove);

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
            if let Some(local_lsp_store) = self.as_local_mut() {
                local_lsp_store
                    .language_server_watched_paths
                    .remove(&server_id_to_remove);
                local_lsp_store
                    .last_workspace_edits_by_language_server
                    .remove(&server_id_to_remove);
                local_lsp_store
                    .language_servers
                    .remove(&server_id_to_remove);
            }
            cx.emit(LspStoreEvent::LanguageServerRemoved(server_id_to_remove));
        }

        if let Some(local) = self.as_local() {
            local.prettier_store.update(cx, |prettier_store, cx| {
                prettier_store.remove_worktree(id_to_remove, cx);
            })
        }
    }

    pub fn shared(
        &mut self,
        project_id: u64,
        downstream_client: AnyProtoClient,
        _: &mut ModelContext<Self>,
    ) {
        self.downstream_client = Some((downstream_client.clone(), project_id));

        for (server_id, status) in &self.language_server_statuses {
            downstream_client
                .send(proto::StartLanguageServer {
                    project_id,
                    server: Some(proto::LanguageServer {
                        id: server_id.0 as u64,
                        name: status.name.clone(),
                        worktree_id: None,
                    }),
                })
                .log_err();
        }
    }

    pub fn disconnected_from_host(&mut self) {
        self.downstream_client.take();
    }

    pub fn disconnected_from_ssh_remote(&mut self) {
        if let LspStoreMode::Remote(RemoteLspStore {
            upstream_client, ..
        }) = &mut self.mode
        {
            upstream_client.take();
        }
    }

    pub(crate) fn set_language_server_statuses_from_proto(
        &mut self,
        language_servers: Vec<proto::LanguageServer>,
    ) {
        self.language_server_statuses = language_servers
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
    }

    pub(crate) fn register_language_server(
        &mut self,
        worktree_id: WorktreeId,
        language_server_name: LanguageServerName,
        language_server_id: LanguageServerId,
    ) {
        self.language_server_ids
            .insert((worktree_id, language_server_name), language_server_id);
    }

    #[track_caller]
    pub(crate) fn register_buffer_with_language_servers(
        &mut self,
        buffer_handle: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) {
        let available_language = self.detect_language_for_buffer(buffer_handle, cx);

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
            let worktree_id = file.worktree_id(cx);

            if let Some(diagnostics) = self.diagnostics.get(&worktree_id) {
                for (server_id, diagnostics) in
                    diagnostics.get(file.path()).cloned().unwrap_or_default()
                {
                    self.update_buffer_diagnostics(buffer_handle, server_id, None, diagnostics, cx)
                        .log_err();
                }
            }

            if let Some(language) = available_language {
                for adapter in self.languages.lsp_adapters(&language.name()) {
                    let server = self
                        .language_server_ids
                        .get(&(worktree_id, adapter.name.clone()))
                        .and_then(|id| self.as_local()?.language_servers.get(id))
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
                                    adapter.language_id(&language.name()),
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

    pub(crate) fn unregister_buffer_from_language_servers(
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
                for adapter in self.languages.lsp_adapters(&language.name()) {
                    if let Some(server_id) = ids.get(&(worktree_id, adapter.name.clone())) {
                        buffer.update_diagnostics(*server_id, DiagnosticSet::new([], buffer), cx);
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

    pub fn update_diagnostic_entries(
        &mut self,
        server_id: LanguageServerId,
        abs_path: PathBuf,
        version: Option<i32>,
        diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        cx: &mut ModelContext<Self>,
    ) -> Result<(), anyhow::Error> {
        let Some((worktree, relative_path)) =
            self.worktree_store.read(cx).find_worktree(&abs_path, cx)
        else {
            log::warn!("skipping diagnostics update, no worktree found for path {abs_path:?}");
            return Ok(());
        };

        let project_path = ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: relative_path.into(),
        };

        if let Some(buffer) = self.buffer_store.read(cx).get_by_path(&project_path, cx) {
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
            cx.emit(LspStoreEvent::DiagnosticsUpdated {
                language_server_id: server_id,
                path: project_path,
            })
        }
        Ok(())
    }

    fn update_worktree_diagnostics(
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
            if let Some((downstream_client, project_id)) = &self.downstream_client {
                downstream_client
                    .send(proto::UpdateDiagnosticSummary {
                        project_id: *project_id,
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

    pub fn open_buffer_for_symbol(
        &mut self,
        symbol: &Symbol,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = client.request(proto::OpenBufferForSymbol {
                project_id,
                symbol: Some(Self::serialize_symbol(symbol)),
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
            let Some(&language_server_id) = self.language_server_ids.get(&(
                symbol.source_worktree_id,
                symbol.language_server_name.clone(),
            )) else {
                return Task::ready(Err(anyhow!(
                    "language server for worktree and language not found"
                )));
            };

            let worktree_abs_path = if let Some(worktree_abs_path) = self
                .worktree_store
                .read(cx)
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
        }
    }

    pub fn open_local_buffer_via_lsp(
        &mut self,
        mut abs_path: lsp::Url,
        language_server_id: LanguageServerId,
        language_server_name: LanguageServerName,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        cx.spawn(move |lsp_store, mut cx| async move {
            // Escape percent-encoded string.
            let current_scheme = abs_path.scheme().to_owned();
            let _ = abs_path.set_scheme("file");

            let abs_path = abs_path
                .to_file_path()
                .map_err(|_| anyhow!("can't convert URI to path"))?;
            let p = abs_path.clone();
            let yarn_worktree = lsp_store
                .update(&mut cx, move |lsp_store, cx| match lsp_store.as_local() {
                    Some(local_lsp_store) => local_lsp_store.yarn.update(cx, |_, cx| {
                        cx.spawn(|this, mut cx| async move {
                            let t = this
                                .update(&mut cx, |this, cx| {
                                    this.process_path(&p, &current_scheme, cx)
                                })
                                .ok()?;
                            t.await
                        })
                    }),
                    None => Task::ready(None),
                })?
                .await;
            let (worktree_root_target, known_relative_path) =
                if let Some((zip_root, relative_path)) = yarn_worktree {
                    (zip_root, Some(relative_path))
                } else {
                    (Arc::<Path>::from(abs_path.as_path()), None)
                };
            let (worktree, relative_path) = if let Some(result) =
                lsp_store.update(&mut cx, |lsp_store, cx| {
                    lsp_store.worktree_store.update(cx, |worktree_store, cx| {
                        worktree_store.find_worktree(&worktree_root_target, cx)
                    })
                })? {
                let relative_path =
                    known_relative_path.unwrap_or_else(|| Arc::<Path>::from(result.1));
                (result.0, relative_path)
            } else {
                let worktree = lsp_store
                    .update(&mut cx, |lsp_store, cx| {
                        lsp_store.worktree_store.update(cx, |worktree_store, cx| {
                            worktree_store.create_worktree(&worktree_root_target, false, cx)
                        })
                    })?
                    .await?;
                if worktree.update(&mut cx, |worktree, _| worktree.is_local())? {
                    lsp_store
                        .update(&mut cx, |lsp_store, cx| {
                            lsp_store.register_language_server(
                                worktree.read(cx).id(),
                                language_server_name,
                                language_server_id,
                            )
                        })
                        .ok();
                }
                let worktree_root = worktree.update(&mut cx, |worktree, _| worktree.abs_path())?;
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
            lsp_store
                .update(&mut cx, |lsp_store, cx| {
                    lsp_store.buffer_store().update(cx, |buffer_store, cx| {
                        buffer_store.open_buffer(project_path, cx)
                    })
                })?
                .await
        })
    }

    pub(crate) fn update_buffer_diagnostics(
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
        debug_assert!(self.upstream_client().is_none());

        let snapshot = buffer.read(cx).snapshot();
        let scope = position.and_then(|position| snapshot.language_scope_at(position));
        let server_ids = self
            .language_servers_for_buffer(buffer.read(cx), cx)
            .filter(|(adapter, _)| {
                scope
                    .as_ref()
                    .map(|scope| scope.language_allowed(&adapter.name))
                    .unwrap_or(true)
            })
            .map(|(_, server)| server.server_id())
            .collect::<Vec<_>>();

        let mut response_results = server_ids
            .into_iter()
            .map(|server_id| {
                self.request_lsp(
                    buffer.clone(),
                    LanguageServerToQuery::Other(server_id),
                    request.clone(),
                    cx,
                )
            })
            .collect::<FuturesUnordered<_>>();

        cx.spawn(|_, _| async move {
            let mut responses = Vec::with_capacity(response_results.len());
            while let Some(response_result) = response_results.next().await {
                if let Some(response) = response_result.log_err() {
                    responses.push(response);
                }
            }
            responses
        })
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
        let sender_id = envelope.original_sender_id().unwrap_or_default();
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

    async fn handle_multi_lsp_query(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::MultiLspQuery>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::MultiLspQueryResponse> {
        let response_from_ssh = this.update(&mut cx, |this, _| {
            let (upstream_client, project_id) = this.upstream_client()?;
            let mut payload = envelope.payload.clone();
            payload.project_id = project_id;

            Some(upstream_client.request(payload))
        })?;
        if let Some(response_from_ssh) = response_from_ssh {
            return response_from_ssh.await;
        }

        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let version = deserialize_version(&envelope.payload.version);
        let buffer = this.update(&mut cx, |this, cx| {
            this.buffer_store.read(cx).get_existing(buffer_id)
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
                    GetHover::from_proto(get_hover, this.clone(), buffer.clone(), cx.clone())
                        .await?;
                let all_hovers = this
                    .update(&mut cx, |this, cx| {
                        this.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_hover.position),
                            get_hover,
                            cx,
                        )
                    })?
                    .await
                    .into_iter()
                    .filter_map(|hover| remove_empty_hover_blocks(hover?));
                this.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
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
                    this.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let all_actions = this
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

                this.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
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
                    this.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let all_signatures = this
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

                this.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
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

    async fn handle_apply_code_action(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeAction>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ApplyCodeActionResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
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
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.serialize_project_transaction_for_peer(
                    project_transaction,
                    sender_id,
                    cx,
                )
            })
        })?;
        Ok(proto::ApplyCodeActionResponse {
            transaction: Some(project_transaction),
        })
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
                cx.emit(LspStoreEvent::DiagnosticsUpdated {
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
            let server_id = LanguageServerId(server.id as usize);
            this.language_server_statuses.insert(
                server_id,
                LanguageServerStatus {
                    name: server.name.clone(),
                    pending_work: Default::default(),
                    has_pending_diagnostic_updates: false,
                    progress_tokens: Default::default(),
                },
            );
            cx.emit(LspStoreEvent::LanguageServerAdded(
                server_id,
                LanguageServerName(server.name.into()),
                server.worktree_id.map(WorktreeId::from_proto),
            ));
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

    async fn handle_language_server_log(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::LanguageServerLog>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let language_server_id = LanguageServerId(envelope.payload.language_server_id as usize);
        let log_type = envelope
            .payload
            .log_type
            .map(LanguageServerLogType::from_proto)
            .context("invalid language server log type")?;

        let message = envelope.payload.message;

        this.update(&mut cx, |_, cx| {
            cx.emit(LspStoreEvent::LanguageServerLog(
                language_server_id,
                log_type,
                message,
            ));
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

        cx.emit(LspStoreEvent::DiskBasedDiagnosticsStarted { language_server_id });
        cx.emit(LspStoreEvent::LanguageServerUpdate {
            language_server_id,
            message: proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(
                Default::default(),
            ),
        })
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

        cx.emit(LspStoreEvent::DiskBasedDiagnosticsFinished { language_server_id });
        cx.emit(LspStoreEvent::LanguageServerUpdate {
            language_server_id,
            message: proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                Default::default(),
            ),
        })
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
        }) = self
            .as_local_mut()
            .and_then(|local_store| local_store.language_servers.get_mut(&language_server_id))
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
                    }) = this.as_local_mut().and_then(|local_store| {
                        local_store.language_servers.get_mut(&language_server_id)
                    }) {
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

    pub fn language_server_statuses(
        &self,
    ) -> impl DoubleEndedIterator<Item = (LanguageServerId, &LanguageServerStatus)> {
        self.language_server_statuses
            .iter()
            .map(|(key, value)| (*key, value))
    }

    fn lsp_notify_abs_paths_changed(
        &mut self,
        server_id: LanguageServerId,
        changes: Vec<PathEvent>,
    ) {
        maybe!({
            let server = self.language_server_for_id(server_id)?;
            let changes = changes
                .into_iter()
                .filter_map(|event| {
                    let typ = match event.kind? {
                        PathEventKind::Created => lsp::FileChangeType::CREATED,
                        PathEventKind::Removed => lsp::FileChangeType::DELETED,
                        PathEventKind::Changed => lsp::FileChangeType::CHANGED,
                    };
                    Some(lsp::FileEvent {
                        uri: lsp::Url::from_file_path(&event.path).ok()?,
                        typ,
                    })
                })
                .collect::<Vec<_>>();
            if !changes.is_empty() {
                server
                    .notify::<lsp::notification::DidChangeWatchedFiles>(
                        lsp::DidChangeWatchedFilesParams { changes },
                    )
                    .log_err();
            }
            Some(())
        });
    }

    fn rebuild_watched_paths(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) {
        let worktrees = self
            .worktree_store
            .read(cx)
            .worktrees()
            .filter_map(|worktree| {
                self.language_servers_for_worktree(worktree.read(cx).id())
                    .find(|server| server.server_id() == language_server_id)
                    .map(|_| worktree)
            })
            .collect::<Vec<_>>();

        let local_lsp_store = self.as_local_mut().unwrap();

        let Some(watchers) = local_lsp_store
            .language_server_watcher_registrations
            .get(&language_server_id)
        else {
            return;
        };

        let mut worktree_globs = HashMap::default();
        let mut abs_globs = HashMap::default();
        log::trace!(
            "Processing new watcher paths for language server with id {}",
            language_server_id
        );

        enum PathToWatch {
            Worktree {
                literal_prefix: Arc<Path>,
                pattern: String,
            },
            Absolute {
                path: Arc<Path>,
                pattern: String,
            },
        }
        for watcher in watchers.values().flatten() {
            let mut found_host = false;
            for worktree in &worktrees {
                let glob_is_inside_worktree = worktree.update(cx, |tree, _| {
                    if let Some(worktree_root_path) = tree.abs_path().to_str() {
                        let path_to_watch = match &watcher.glob_pattern {
                            lsp::GlobPattern::String(s) => {
                                match s.strip_prefix(worktree_root_path) {
                                    Some(relative) => {
                                        let pattern = relative
                                            .strip_prefix(std::path::MAIN_SEPARATOR)
                                            .unwrap_or(relative)
                                            .to_owned();
                                        let literal_prefix = glob_literal_prefix(&pattern);

                                        let literal_prefix = Arc::from(PathBuf::from(
                                            literal_prefix
                                                .strip_prefix(std::path::MAIN_SEPARATOR)
                                                .unwrap_or(literal_prefix),
                                        ));
                                        PathToWatch::Worktree {
                                            literal_prefix,
                                            pattern,
                                        }
                                    }
                                    None => {
                                        let path = glob_literal_prefix(s);
                                        let glob = &s[path.len()..];
                                        let pattern = glob
                                            .strip_prefix(std::path::MAIN_SEPARATOR)
                                            .unwrap_or(glob)
                                            .to_owned();
                                        let path = if Path::new(path).components().next().is_none()
                                        {
                                            Arc::from(Path::new(worktree_root_path))
                                        } else {
                                            PathBuf::from(path).into()
                                        };

                                        PathToWatch::Absolute { path, pattern }
                                    }
                                }
                            }
                            lsp::GlobPattern::Relative(rp) => {
                                let Ok(mut base_uri) = match &rp.base_uri {
                                    lsp::OneOf::Left(workspace_folder) => &workspace_folder.uri,
                                    lsp::OneOf::Right(base_uri) => base_uri,
                                }
                                .to_file_path() else {
                                    return false;
                                };

                                match base_uri.strip_prefix(worktree_root_path) {
                                    Ok(relative) => {
                                        let mut literal_prefix = relative.to_owned();
                                        literal_prefix.push(glob_literal_prefix(&rp.pattern));

                                        PathToWatch::Worktree {
                                            literal_prefix: literal_prefix.into(),
                                            pattern: rp.pattern.clone(),
                                        }
                                    }
                                    Err(_) => {
                                        let path = glob_literal_prefix(&rp.pattern);
                                        let glob = &rp.pattern[path.len()..];
                                        let pattern = glob
                                            .strip_prefix(std::path::MAIN_SEPARATOR)
                                            .unwrap_or(glob)
                                            .to_owned();
                                        base_uri.push(path);

                                        let path = if base_uri.components().next().is_none() {
                                            Arc::from(Path::new("/"))
                                        } else {
                                            base_uri.into()
                                        };
                                        PathToWatch::Absolute { path, pattern }
                                    }
                                }
                            }
                        };
                        match path_to_watch {
                            PathToWatch::Worktree {
                                literal_prefix,
                                pattern,
                            } => {
                                if let Some((tree, glob)) =
                                    tree.as_local_mut().zip(Glob::new(&pattern).log_err())
                                {
                                    tree.add_path_prefix_to_scan(literal_prefix);
                                    worktree_globs
                                        .entry(tree.id())
                                        .or_insert_with(GlobSetBuilder::new)
                                        .add(glob);
                                } else {
                                    return false;
                                }
                            }
                            PathToWatch::Absolute { path, pattern } => {
                                if let Some(glob) = Glob::new(&pattern).log_err() {
                                    abs_globs
                                        .entry(path)
                                        .or_insert_with(GlobSetBuilder::new)
                                        .add(glob);
                                }
                            }
                        }
                        return true;
                    }
                    false
                });
                if glob_is_inside_worktree {
                    log::trace!(
                        "Watcher pattern `{}` has been attached to the worktree at `{}`",
                        serde_json::to_string(&watcher.glob_pattern).unwrap(),
                        worktree.read(cx).abs_path().display()
                    );
                    found_host = true;
                }
            }
            if !found_host {
                log::error!(
                    "Watcher pattern `{}` has not been attached to any worktree or absolute path",
                    serde_json::to_string(&watcher.glob_pattern).unwrap()
                )
            }
        }

        let mut watch_builder = LanguageServerWatchedPathsBuilder::default();
        for (worktree_id, builder) in worktree_globs {
            if let Ok(globset) = builder.build() {
                watch_builder.watch_worktree(worktree_id, globset);
            }
        }
        for (abs_path, builder) in abs_globs {
            if let Ok(globset) = builder.build() {
                watch_builder.watch_abs_path(abs_path, globset);
            }
        }
        let watcher = watch_builder.build(local_lsp_store.fs.clone(), language_server_id, cx);
        local_lsp_store
            .language_server_watched_paths
            .insert(language_server_id, watcher);

        cx.notify();
    }

    pub fn language_server_for_id(&self, id: LanguageServerId) -> Option<Arc<LanguageServer>> {
        if let Some(local_lsp_store) = self.as_local() {
            if let Some(LanguageServerState::Running { server, .. }) =
                local_lsp_store.language_servers.get(&id)
            {
                Some(server.clone())
            } else if let Some((_, server)) =
                local_lsp_store.supplementary_language_servers.get(&id)
            {
                Some(Arc::clone(server))
            } else {
                None
            }
        } else {
            None
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
                this.as_local_mut()
                    .unwrap()
                    .last_workspace_edits_by_language_server
                    .insert(server_id, transaction);
            }
        })?;
        Ok(lsp::ApplyWorkspaceEditResponse {
            applied: true,
            failed_change: None,
            failure_reason: None,
        })
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
                    cx.emit(LspStoreEvent::LanguageServerUpdate {
                        language_server_id,
                        message: proto::update_language_server::Variant::WorkProgress(
                            proto::LspWorkProgress {
                                token,
                                message: report.message,
                                percentage: report.percentage,
                            },
                        ),
                    })
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
        cx.emit(LspStoreEvent::LanguageServerUpdate {
            language_server_id,
            message: proto::update_language_server::Variant::WorkStart(proto::LspWorkStart {
                token,
                title: progress.title,
                message: progress.message,
                percentage: progress.percentage.map(|p| p as u32),
            }),
        })
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
                    cx.emit(LspStoreEvent::RefreshInlayHints);
                }
            }
            cx.notify();
        }

        cx.emit(LspStoreEvent::LanguageServerUpdate {
            language_server_id,
            message: proto::update_language_server::Variant::WorkEnd(proto::LspWorkEnd { token }),
        })
    }

    fn on_lsp_did_change_watched_files(
        &mut self,
        language_server_id: LanguageServerId,
        registration_id: &str,
        params: DidChangeWatchedFilesRegistrationOptions,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(local) = self.as_local_mut() {
            let registrations = local
                .language_server_watcher_registrations
                .entry(language_server_id)
                .or_default();

            registrations.insert(registration_id.to_string(), params.watchers);

            self.rebuild_watched_paths(language_server_id, cx);
        }
    }

    fn on_lsp_unregister_did_change_watched_files(
        &mut self,
        language_server_id: LanguageServerId,
        registration_id: &str,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(local) = self.as_local_mut() {
            let registrations = local
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
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn edits_from_lsp(
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

    pub async fn handle_resolve_completion_documentation(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ResolveCompletionDocumentation>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::ResolveCompletionDocumentationResponse> {
        let lsp_completion = serde_json::from_slice(&envelope.payload.lsp_completion)?;

        let completion = this
            .read_with(&cx, |this, cx| {
                let id = LanguageServerId(envelope.payload.language_server_id as usize);
                let Some(server) = this.language_server_for_id(id) else {
                    return Err(anyhow!("No language server {id}"));
                };

                Ok(cx.background_executor().spawn(async move {
                    let can_resolve = server
                        .capabilities()
                        .completion_provider
                        .as_ref()
                        .and_then(|options| options.resolve_provider)
                        .unwrap_or(false);
                    if can_resolve {
                        server
                            .request::<lsp::request::ResolveCompletionItem>(lsp_completion)
                            .await
                    } else {
                        anyhow::Ok(lsp_completion)
                    }
                }))
            })??
            .await?;

        let mut documentation_is_markdown = false;
        let lsp_completion = serde_json::to_string(&completion)?.into_bytes();
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
            lsp_completion,
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

    async fn handle_refresh_inlay_hints(
        this: Model<Self>,
        _: TypedEnvelope<proto::RefreshInlayHints>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |_, cx| {
            cx.emit(LspStoreEvent::RefreshInlayHints);
        })?;
        Ok(proto::Ack {})
    }

    async fn handle_inlay_hints(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::InlayHints>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::InlayHintsResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
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
            .update(&mut cx, |lsp_store, cx| {
                lsp_store.inlay_hints(buffer.clone(), start..end, cx)
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
            .update(&mut cx, |this, cx| {
                this.resolve_inlay_hint(
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

    async fn handle_open_buffer_for_symbol(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::OpenBufferForSymbol>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferForSymbolResponse> {
        let peer_id = envelope.original_sender_id().unwrap_or_default();
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
                Err(anyhow!(rpc::ErrorCode::UnsharedItem))
            } else {
                this.buffer_store
                    .update(cx, |buffer_store, cx| {
                        buffer_store.create_buffer_for_peer(&buffer, peer_id, cx)
                    })
                    .detach_and_log_err(cx);
                let buffer_id = buffer.read(cx).remote_id().to_proto();
                Ok(proto::OpenBufferForSymbolResponse { buffer_id })
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

    pub async fn handle_get_project_symbols(
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
            symbols: symbols.iter().map(Self::serialize_symbol).collect(),
        })
    }

    pub async fn handle_restart_language_servers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::RestartLanguageServers>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |this, cx| {
            let buffers: Vec<_> = envelope
                .payload
                .buffer_ids
                .into_iter()
                .flat_map(|buffer_id| {
                    this.buffer_store
                        .read(cx)
                        .get(BufferId::new(buffer_id).log_err()?)
                })
                .collect();
            this.restart_language_servers_for_buffers(buffers, cx)
        })?;

        Ok(proto::Ack {})
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
    pub fn last_formatting_failure(&self) -> Option<&str> {
        self.as_local()
            .and_then(|local| local.last_formatting_failure.as_deref())
    }

    pub fn environment_for_buffer(
        &self,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        let worktree_id = buffer.read(cx).file().map(|file| file.worktree_id(cx));
        let worktree_abs_path = worktree_id.and_then(|worktree_id| {
            self.worktree_store
                .read(cx)
                .worktree_for_id(worktree_id, cx)
                .map(|entry| entry.read(cx).abs_path().clone())
        });

        if let Some(environment) = &self.as_local().map(|local| local.environment.clone()) {
            environment.update(cx, |env, cx| {
                env.get_environment(worktree_id, worktree_abs_path, cx)
            })
        } else {
            Task::ready(None).shared()
        }
    }

    pub fn format(
        &mut self,
        buffers: HashSet<Model<Buffer>>,
        push_to_history: bool,
        trigger: FormatTrigger,
        target: FormatTarget,
        cx: &mut ModelContext<Self>,
    ) -> Task<anyhow::Result<ProjectTransaction>> {
        if let Some(_) = self.as_local() {
            let buffers_with_paths = buffers
                .into_iter()
                .map(|buffer_handle| {
                    let buffer = buffer_handle.read(cx);
                    let buffer_abs_path = File::from_dyn(buffer.file())
                        .and_then(|file| file.as_local().map(|f| f.abs_path(cx)));

                    (buffer_handle, buffer_abs_path)
                })
                .collect::<Vec<_>>();

            cx.spawn(move |lsp_store, mut cx| async move {
                let mut formattable_buffers = Vec::with_capacity(buffers_with_paths.len());

                for (handle, abs_path) in buffers_with_paths {
                    let env = lsp_store
                        .update(&mut cx, |lsp_store, cx| {
                            lsp_store.environment_for_buffer(&handle, cx)
                        })?
                        .await;

                    formattable_buffers.push(FormattableBuffer {
                        handle,
                        abs_path,
                        env,
                    });
                }

                let result = LocalLspStore::format_locally(
                    lsp_store.clone(),
                    formattable_buffers,
                    push_to_history,
                    trigger,
                    target,
                    cx.clone(),
                )
                .await;

                lsp_store.update(&mut cx, |lsp_store, _| {
                    let local = lsp_store.as_local_mut().unwrap();
                    match &result {
                        Ok(_) => local.last_formatting_failure = None,
                        Err(error) => {
                            local.last_formatting_failure.replace(error.to_string());
                        }
                    }
                })?;

                result
            })
        } else if let Some((client, project_id)) = self.upstream_client() {
            let buffer_store = self.buffer_store();
            cx.spawn(move |_, mut cx| async move {
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

                buffer_store
                    .update(&mut cx, |buffer_store, cx| {
                        buffer_store.deserialize_project_transaction(response, push_to_history, cx)
                    })?
                    .await
            })
        } else {
            Task::ready(Ok(ProjectTransaction::default()))
        }
    }

    async fn handle_format_buffers(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::FormatBuffers>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FormatBuffersResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let format = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(this.buffer_store.read(cx).get_existing(buffer_id)?);
            }
            let trigger = FormatTrigger::from_proto(envelope.payload.trigger);
            Ok::<_, anyhow::Error>(this.format(buffers, false, trigger, FormatTarget::Buffer, cx))
        })??;

        let project_transaction = format.await?;
        let project_transaction = this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.serialize_project_transaction_for_peer(
                    project_transaction,
                    sender_id,
                    cx,
                )
            })
        })?;
        Ok(proto::FormatBuffersResponse {
            transaction: Some(project_transaction),
        })
    }

    pub fn start_language_servers(
        &mut self,
        worktree: &Model<Worktree>,
        language: LanguageName,
        cx: &mut ModelContext<Self>,
    ) {
        let root_file = worktree
            .update(cx, |tree, cx| tree.root_file(cx))
            .map(|f| f as _);
        let settings = language_settings(Some(language.clone()), root_file.as_ref(), cx);
        if !settings.enable_language_server || self.mode.is_remote() {
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
                self.languages
                    .register_lsp_adapter(language.clone(), adapter.adapter.clone());
                enabled_lsp_adapters.push(adapter);
                continue;
            }

            log::warn!(
                "no language server found matching '{}'",
                desired_language_server.0
            );
        }

        for adapter in &enabled_lsp_adapters {
            self.start_language_server(worktree, adapter.clone(), language.clone(), cx);
        }

        // After starting all the language servers, reorder them to reflect the desired order
        // based on the settings.
        //
        // This is done, in part, to ensure that language servers loaded at different points
        // (e.g., native vs extension) still end up in the right order at the end, rather than
        // it being based on which language server happened to be loaded in first.
        self.languages
            .reorder_language_servers(&language, enabled_lsp_adapters);
    }

    fn get_language_server_binary(
        &self,
        adapter: Arc<CachedLspAdapter>,
        delegate: Arc<dyn LspAdapterDelegate>,
        allow_binary_download: bool,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<LanguageServerBinary>> {
        let settings = ProjectSettings::get(
            Some(SettingsLocation {
                worktree_id: delegate.worktree_id(),
                path: Path::new(""),
            }),
            cx,
        )
        .lsp
        .get(&adapter.name)
        .and_then(|s| s.binary.clone());

        if settings.as_ref().is_some_and(|b| b.path.is_some()) {
            let settings = settings.unwrap();
            return cx.spawn(|_, _| async move {
                Ok(LanguageServerBinary {
                    path: PathBuf::from(&settings.path.unwrap()),
                    env: Some(delegate.shell_env().await),
                    arguments: settings
                        .arguments
                        .unwrap_or_default()
                        .iter()
                        .map(Into::into)
                        .collect(),
                })
            });
        }
        let lsp_binary_options = LanguageServerBinaryOptions {
            allow_path_lookup: !settings
                .as_ref()
                .and_then(|b| b.ignore_system_version)
                .unwrap_or_default(),
            allow_binary_download,
        };
        cx.spawn(|_, mut cx| async move {
            let binary_result = adapter
                .clone()
                .get_language_server_command(delegate.clone(), lsp_binary_options, &mut cx)
                .await;

            delegate.update_status(adapter.name.clone(), LanguageServerBinaryStatus::None);

            let mut binary = binary_result?;
            if let Some(arguments) = settings.and_then(|b| b.arguments) {
                binary.arguments = arguments.into_iter().map(Into::into).collect();
            }

            // If we do have a project environment (either by spawning a shell in in the project directory
            // or by getting it from the CLI) and the language server command itself
            // doesn't have an environment, then we use the project environment.
            if binary.env.is_none() {
                log::info!(
                    "using project environment for language server {:?}",
                    adapter.name()
                );
                binary.env = Some(delegate.shell_env().await);
            }
            Ok(binary)
        })
    }

    fn start_language_server(
        &mut self,
        worktree_handle: &Model<Worktree>,
        adapter: Arc<CachedLspAdapter>,
        language: LanguageName,
        cx: &mut ModelContext<Self>,
    ) {
        if self.mode.is_remote() {
            return;
        }

        let worktree = worktree_handle.read(cx);
        let worktree_id = worktree.id();
        let worktree_path = worktree.abs_path();
        let key = (worktree_id, adapter.name.clone());

        if self.language_server_ids.contains_key(&key) {
            return;
        }

        let project_settings = ProjectSettings::get(
            Some(SettingsLocation {
                worktree_id,
                path: Path::new(""),
            }),
            cx,
        );
        let lsp = project_settings.lsp.get(&adapter.name);
        let override_options = lsp.and_then(|s| s.initialization_options.clone());

        let stderr_capture = Arc::new(Mutex::new(Some(String::new())));
        let delegate = LocalLspAdapterDelegate::for_local(self, worktree_handle, cx)
            as Arc<dyn LspAdapterDelegate>;

        let server_id = self.languages.next_language_server_id();
        let root_path = worktree_path.clone();
        log::info!(
            "attempting to start language server {:?}, path: {root_path:?}, id: {server_id}",
            adapter.name.0
        );

        let binary = self.get_language_server_binary(adapter.clone(), delegate.clone(), true, cx);

        let pending_server = cx.spawn({
            let adapter = adapter.clone();
            let stderr_capture = stderr_capture.clone();

            move |_lsp_store, cx| async move {
                let binary = binary.await?;

                #[cfg(any(test, feature = "test-support"))]
                if let Some(server) = _lsp_store
                    .update(&mut cx.clone(), |this, cx| {
                        this.languages.create_fake_language_server(
                            server_id,
                            &adapter.name,
                            binary.clone(),
                            cx.to_async(),
                        )
                    })
                    .ok()
                    .flatten()
                {
                    return Ok(server);
                }

                lsp::LanguageServer::new(
                    stderr_capture,
                    server_id,
                    binary,
                    &root_path,
                    adapter.code_action_kinds(),
                    cx,
                )
            }
        });

        let state = LanguageServerState::Starting({
            let server_name = adapter.name.0.clone();
            let delegate = delegate as Arc<dyn LspAdapterDelegate>;
            let language = language.clone();
            let key = key.clone();
            let adapter = adapter.clone();

            cx.spawn(move |this, mut cx| async move {
                let result = {
                    let delegate = delegate.clone();
                    let adapter = adapter.clone();
                    let this = this.clone();
                    let toolchains = this
                        .update(&mut cx, |this, cx| this.toolchain_store(cx))
                        .ok()?;
                    let mut cx = cx.clone();
                    async move {
                        let language_server = pending_server.await?;

                        let workspace_config = adapter
                            .adapter
                            .clone()
                            .workspace_configuration(&delegate, toolchains.clone(), &mut cx)
                            .await?;

                        let mut initialization_options = adapter
                            .adapter
                            .clone()
                            .initialization_options(&(delegate))
                            .await?;

                        Self::setup_lsp_messages(this.clone(), &language_server, delegate, adapter);

                        match (&mut initialization_options, override_options) {
                            (Some(initialization_options), Some(override_options)) => {
                                merge_json_value_into(override_options, initialization_options);
                            }
                            (None, override_options) => initialization_options = override_options,
                            _ => {}
                        }

                        let language_server = cx
                            .update(|cx| language_server.initialize(initialization_options, cx))?
                            .await
                            .inspect_err(|_| {
                                if let Some(this) = this.upgrade() {
                                    this.update(&mut cx, |_, cx| {
                                        cx.emit(LspStoreEvent::LanguageServerRemoved(server_id))
                                    })
                                    .ok();
                                }
                            })?;

                        language_server
                            .notify::<lsp::notification::DidChangeConfiguration>(
                                lsp::DidChangeConfigurationParams {
                                    settings: workspace_config,
                                },
                            )
                            .ok();

                        anyhow::Ok(language_server)
                    }
                }
                .await;

                match result {
                    Ok(server) => {
                        this.update(&mut cx, |this, mut cx| {
                            this.insert_newly_running_language_server(
                                language,
                                adapter,
                                server.clone(),
                                server_id,
                                key,
                                &mut cx,
                            );
                        })
                        .ok();
                        stderr_capture.lock().take();
                        Some(server)
                    }

                    Err(err) => {
                        let log = stderr_capture.lock().take().unwrap_or_default();
                        delegate.update_status(
                            adapter.name(),
                            LanguageServerBinaryStatus::Failed {
                                error: format!("{err}\n-- stderr--\n{}", log),
                            },
                        );
                        log::error!("Failed to start language server {server_name:?}: {err}");
                        log::error!("server stderr: {:?}", log);
                        None
                    }
                }
            })
        });

        self.as_local_mut()
            .unwrap()
            .language_servers
            .insert(server_id, state);
        self.language_server_ids.insert(key, server_id);
    }

    async fn shutdown_language_server(
        server_state: Option<LanguageServerState>,
        name: LanguageServerName,
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

    // Returns a list of all of the worktrees which no longer have a language server and the root path
    // for the stopped server
    fn stop_local_language_server(
        &mut self,
        worktree_id: WorktreeId,
        adapter_name: LanguageServerName,
        cx: &mut ModelContext<Self>,
    ) -> Task<Vec<WorktreeId>> {
        let key = (worktree_id, adapter_name);
        if self.mode.is_local() {
            if let Some(server_id) = self.language_server_ids.remove(&key) {
                let name = key.1;
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
                            buffer.update_diagnostics(
                                server_id,
                                DiagnosticSet::new([], buffer),
                                cx,
                            );
                        });
                    }
                });

                for (worktree_id, summaries) in self.diagnostic_summaries.iter_mut() {
                    summaries.retain(|path, summaries_by_server_id| {
                        if summaries_by_server_id.remove(&server_id).is_some() {
                            if let Some((client, project_id)) = self.downstream_client.clone() {
                                client
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

                self.as_local_mut()
                    .unwrap()
                    .language_server_watched_paths
                    .remove(&server_id);
                self.language_server_statuses.remove(&server_id);
                cx.notify();

                let server_state = self
                    .as_local_mut()
                    .unwrap()
                    .language_servers
                    .remove(&server_id);
                cx.emit(LspStoreEvent::LanguageServerRemoved(server_id));
                cx.spawn(move |_, cx| async move {
                    Self::shutdown_language_server(server_state, name, cx).await;
                    orphaned_worktrees
                })
            } else {
                Task::ready(Vec::new())
            }
        } else {
            Task::ready(Vec::new())
        }
    }

    pub fn restart_language_servers_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Model<Buffer>>,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = client.request(proto::RestartLanguageServers {
                project_id,
                buffer_ids: buffers
                    .into_iter()
                    .map(|b| b.read(cx).remote_id().to_proto())
                    .collect(),
            });
            cx.background_executor()
                .spawn(request)
                .detach_and_log_err(cx);
        } else {
            let language_server_lookup_info: HashSet<(Model<Worktree>, LanguageName)> = buffers
                .into_iter()
                .filter_map(|buffer| {
                    let buffer = buffer.read(cx);
                    let file = buffer.file()?;
                    let worktree = File::from_dyn(Some(file))?.worktree.clone();
                    let language =
                        self.languages
                            .language_for_file(file, Some(buffer.as_rope()), cx)?;

                    Some((worktree, language.name()))
                })
                .collect();

            for (worktree, language) in language_server_lookup_info {
                self.restart_local_language_servers(worktree, language, cx);
            }
        }
    }

    fn restart_local_language_servers(
        &mut self,
        worktree: Model<Worktree>,
        language: LanguageName,
        cx: &mut ModelContext<Self>,
    ) {
        let worktree_id = worktree.read(cx).id();

        let stop_tasks = self
            .languages
            .clone()
            .lsp_adapters(&language)
            .iter()
            .map(|adapter| {
                let stop_task =
                    self.stop_local_language_server(worktree_id, adapter.name.clone(), cx);
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

    fn setup_lsp_messages(
        this: WeakModel<Self>,
        language_server: &LanguageServer,
        delegate: Arc<dyn LspAdapterDelegate>,
        adapter: Arc<CachedLspAdapter>,
    ) {
        let name = language_server.name();
        let server_id = language_server.server_id();
        language_server
            .on_notification::<lsp::notification::PublishDiagnostics, _>({
                let adapter = adapter.clone();
                let this = this.clone();
                move |mut params, mut cx| {
                    let adapter = adapter.clone();
                    if let Some(this) = this.upgrade() {
                        adapter.process_diagnostics(&mut params);
                        // Everything else has to be on the server, Can we make it on the client?
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
                let this = this.clone();
                move |params, mut cx| {
                    let adapter = adapter.clone();
                    let delegate = delegate.clone();
                    let this = this.clone();
                    async move {
                        let toolchains =
                            this.update(&mut cx, |this, cx| this.toolchain_store(cx))?;
                        let workspace_config = adapter
                            .workspace_configuration(&delegate, toolchains, &mut cx)
                            .await?;
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

        language_server
            .on_request::<lsp::request::WorkspaceFoldersRequest, _, _>({
                let this = this.clone();
                move |_, mut cx| {
                    let this = this.clone();
                    async move {
                        let Some(server) =
                            this.update(&mut cx, |this, _| this.language_server_for_id(server_id))?
                        else {
                            return Ok(None);
                        };
                        let root = server.root_path();
                        let Ok(uri) = Url::from_file_path(&root) else {
                            return Ok(None);
                        };
                        Ok(Some(vec![WorkspaceFolder {
                            uri,
                            name: Default::default(),
                        }]))
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
                            match reg.method.as_str() {
                                "workspace/didChangeWatchedFiles" => {
                                    if let Some(options) = reg.register_options {
                                        let options = serde_json::from_value(options)?;
                                        this.update(&mut cx, |this, cx| {
                                            this.on_lsp_did_change_watched_files(
                                                server_id, &reg.id, options, cx,
                                            );
                                        })?;
                                    }
                                }
                                "textDocument/rangeFormatting" => {
                                    this.update(&mut cx, |this, _| {
                                        if let Some(server) = this.language_server_for_id(server_id)
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
                                    this.update(&mut cx, |this, _| {
                                        if let Some(server) = this.language_server_for_id(server_id)
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
                                    this.update(&mut cx, |this, _| {
                                        if let Some(server) = this.language_server_for_id(server_id)
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
                let this = this.clone();
                move |params, mut cx| {
                    let this = this.clone();
                    async move {
                        for unreg in params.unregisterations.iter() {
                            match unreg.method.as_str() {
                                "workspace/didChangeWatchedFiles" => {
                                    this.update(&mut cx, |this, cx| {
                                        this.on_lsp_unregister_did_change_watched_files(
                                            server_id, &unreg.id, cx,
                                        );
                                    })?;
                                }
                                "textDocument/rename" => {
                                    this.update(&mut cx, |this, _| {
                                        if let Some(server) = this.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.rename_provider = None
                                            })
                                        }
                                    })?;
                                }
                                "textDocument/rangeFormatting" => {
                                    this.update(&mut cx, |this, _| {
                                        if let Some(server) = this.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_range_formatting_provider =
                                                    None
                                            })
                                        }
                                    })?;
                                }
                                "textDocument/onTypeFormatting" => {
                                    this.update(&mut cx, |this, _| {
                                        if let Some(server) = this.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_on_type_formatting_provider =
                                                    None;
                                            })
                                        }
                                    })?;
                                }
                                "textDocument/formatting" => {
                                    this.update(&mut cx, |this, _| {
                                        if let Some(server) = this.language_server_for_id(server_id)
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
                        this.update(&mut cx, |this, cx| {
                            cx.emit(LspStoreEvent::RefreshInlayHints);
                            this.downstream_client.as_ref().map(|(client, project_id)| {
                                client.send(proto::RefreshInlayHints {
                                    project_id: *project_id,
                                })
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

                        let did_update = this
                            .update(&mut cx, |_, cx| {
                                cx.emit(LspStoreEvent::LanguageServerPrompt(request));
                            })
                            .is_ok();
                        if did_update {
                            let response = rx.next().await;

                            Ok(response)
                        } else {
                            Ok(None)
                        }
                    }
                }
            })
            .detach();

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
                                            cx.emit(LspStoreEvent::LanguageServerPrompt(request));
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
                let this = this.clone();
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
                        cx.emit(LspStoreEvent::LanguageServerPrompt(request));
                    });
                }
            })
            .detach();

        let disk_based_diagnostics_progress_token =
            adapter.disk_based_diagnostics_progress_token.clone();

        language_server
            .on_notification::<lsp::notification::Progress, _>({
                let this = this.clone();
                move |params, mut cx| {
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
                }
            })
            .detach();

        language_server
            .on_notification::<lsp::notification::LogMessage, _>({
                let this = this.clone();
                move |params, mut cx| {
                    if let Some(this) = this.upgrade() {
                        this.update(&mut cx, |_, cx| {
                            cx.emit(LspStoreEvent::LanguageServerLog(
                                server_id,
                                LanguageServerLogType::Log(params.typ),
                                params.message,
                            ));
                        })
                        .ok();
                    }
                }
            })
            .detach();

        language_server
            .on_notification::<lsp::notification::LogTrace, _>({
                let this = this.clone();
                move |params, mut cx| {
                    if let Some(this) = this.upgrade() {
                        this.update(&mut cx, |_, cx| {
                            cx.emit(LspStoreEvent::LanguageServerLog(
                                server_id,
                                LanguageServerLogType::Trace(params.verbose),
                                params.message,
                            ));
                        })
                        .ok();
                    }
                }
            })
            .detach();
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

    fn insert_newly_running_language_server(
        &mut self,
        language: LanguageName,
        adapter: Arc<CachedLspAdapter>,
        language_server: Arc<LanguageServer>,
        server_id: LanguageServerId,
        key: (WorktreeId, LanguageServerName),
        cx: &mut ModelContext<Self>,
    ) {
        // If the language server for this key doesn't match the server id, don't store the
        // server. Which will cause it to be dropped, killing the process
        if self
            .language_server_ids
            .get(&key)
            .map(|id| id != &server_id)
            .unwrap_or(false)
        {
            return;
        }

        // Update language_servers collection with Running variant of LanguageServerState
        // indicating that the server is up and running and ready
        if let Some(local) = self.as_local_mut() {
            local.language_servers.insert(
                server_id,
                LanguageServerState::Running {
                    adapter: adapter.clone(),
                    language: language.clone(),
                    server: language_server.clone(),
                    simulate_disk_based_diagnostics_completion: None,
                },
            );
        }

        self.language_server_statuses.insert(
            server_id,
            LanguageServerStatus {
                name: language_server.name().to_string(),
                pending_work: Default::default(),
                has_pending_diagnostic_updates: false,
                progress_tokens: Default::default(),
            },
        );

        cx.emit(LspStoreEvent::LanguageServerAdded(
            server_id,
            language_server.name().into(),
            Some(key.0),
        ));

        if let Some((downstream_client, project_id)) = self.downstream_client.as_ref() {
            downstream_client
                .send(proto::StartLanguageServer {
                    project_id: *project_id,
                    server: Some(proto::LanguageServer {
                        id: server_id.0 as u64,
                        name: language_server.name().to_string(),
                        worktree_id: Some(key.0.to_proto()),
                    }),
                })
                .log_err();
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
                        .lsp_adapters(&language.name())
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
                language_server
                    .notify::<lsp::notification::DidOpenTextDocument>(
                        lsp::DidOpenTextDocumentParams {
                            text_document: lsp::TextDocumentItem::new(
                                uri,
                                adapter.language_id(&language.name()),
                                version,
                                initial_snapshot.text(),
                            ),
                        },
                    )
                    .log_err();

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
        });

        cx.notify();
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

    pub(crate) fn language_servers_for_buffer<'a>(
        &'a self,
        buffer: &'a Buffer,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = (&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        self.language_server_ids_for_buffer(buffer, cx)
            .into_iter()
            .filter_map(
                |server_id| match self.as_local()?.language_servers.get(&server_id)? {
                    LanguageServerState::Running {
                        adapter, server, ..
                    } => Some((adapter, server)),
                    _ => None,
                },
            )
    }

    pub(crate) fn cancel_language_server_work_for_buffers(
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

    pub fn language_servers(
        &self,
    ) -> impl '_ + Iterator<Item = (LanguageServerId, LanguageServerName, WorktreeId)> {
        self.language_server_ids
            .iter()
            .map(|((worktree_id, server_name), server_id)| {
                (*server_id, server_name.clone(), *worktree_id)
            })
    }

    fn register_supplementary_language_server(
        &mut self,
        id: LanguageServerId,
        name: LanguageServerName,
        server: Arc<LanguageServer>,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(local) = self.as_local_mut() {
            local
                .supplementary_language_servers
                .insert(id, (name.clone(), server));
            cx.emit(LspStoreEvent::LanguageServerAdded(id, name, None));
        }
    }

    fn unregister_supplementary_language_server(
        &mut self,
        id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(local) = self.as_local_mut() {
            local.supplementary_language_servers.remove(&id);
            cx.emit(LspStoreEvent::LanguageServerRemoved(id));
        }
    }

    pub fn supplementary_language_servers(
        &self,
    ) -> impl '_ + Iterator<Item = (LanguageServerId, LanguageServerName)> {
        self.as_local().into_iter().flat_map(|local| {
            local
                .supplementary_language_servers
                .iter()
                .map(|(id, (name, _))| (*id, name.clone()))
        })
    }

    pub fn language_server_adapter_for_id(
        &self,
        id: LanguageServerId,
    ) -> Option<Arc<CachedLspAdapter>> {
        self.as_local()
            .and_then(|local| local.language_servers.get(&id))
            .and_then(|language_server_state| match language_server_state {
                LanguageServerState::Running { adapter, .. } => Some(adapter.clone()),
                _ => None,
            })
    }

    pub(super) fn update_local_worktree_language_servers(
        &mut self,
        worktree_handle: &Model<Worktree>,
        changes: &[(Arc<Path>, ProjectEntryId, PathChange)],
        cx: &mut ModelContext<Self>,
    ) {
        if changes.is_empty() {
            return;
        }

        let Some(local) = self.as_local() else { return };

        local.prettier_store.update(cx, |prettier_store, cx| {
            prettier_store.update_prettier_settings(&worktree_handle, changes, cx)
        });

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
                local.language_servers.get(server_id)
            {
                if let Some(watched_paths) = local
                    .language_server_watched_paths
                    .get(server_id)
                    .and_then(|paths| paths.read(cx).worktree_paths.get(&worktree_id))
                {
                    let params = lsp::DidChangeWatchedFilesParams {
                        changes: changes
                            .iter()
                            .filter_map(|(path, _, change)| {
                                if !watched_paths.is_match(path) {
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

    pub(crate) fn cancel_language_server_work(
        &mut self,
        server_id: LanguageServerId,
        token_to_cancel: Option<String>,
        _cx: &mut ModelContext<Self>,
    ) {
        let Some(local) = self.as_local() else {
            return;
        };
        let status = self.language_server_statuses.get(&server_id);
        let server = local.language_servers.get(&server_id);
        if let Some((LanguageServerState::Running { server, .. }, status)) = server.zip(status) {
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

    pub fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Buffer>>> {
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.wait_for_remote_buffer(id, cx)
        })
    }

    pub(crate) fn language_server_ids_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &AppContext,
    ) -> Vec<LanguageServerId> {
        if let Some((file, language)) = File::from_dyn(buffer.file()).zip(buffer.language()) {
            let worktree_id = file.worktree_id(cx);
            self.languages
                .lsp_adapters(&language.name())
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

    pub async fn deserialize_text_edits(
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

    pub async fn deserialize_workspace_edit(
        this: Model<Self>,
        edit: lsp::WorkspaceEdit,
        push_to_history: bool,
        lsp_adapter: Arc<CachedLspAdapter>,
        language_server: Arc<LanguageServer>,
        cx: &mut AsyncAppContext,
    ) -> Result<ProjectTransaction> {
        let fs = this.read_with(cx, |this, _| this.as_local().unwrap().fs.clone())?;

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
                                this.worktree_store
                                    .read(cx)
                                    .entry_for_path(&project_path, cx)
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
                                let buffer_id = buffer_to_edit.read(cx).remote_id();
                                let version = if let Some(buffer_version) = op.text_document.version
                                {
                                    this.buffer_snapshot_for_lsp_version(
                                        &buffer_to_edit,
                                        language_server.server_id(),
                                        Some(buffer_version),
                                        cx,
                                    )
                                    .ok()
                                    .map(|snapshot| snapshot.version)
                                } else {
                                    Some(buffer_to_edit.read(cx).saved_version().clone())
                                };

                                let most_recent_edit = version.and_then(|version| {
                                    version.iter().max_by_key(|timestamp| timestamp.value)
                                });
                                // Check if the edit that triggered that edit has been made by this participant.

                                if let Some(most_recent_edit) = most_recent_edit {
                                    cx.emit(LspStoreEvent::SnippetEdit {
                                        buffer_id,
                                        edits: snippet_edits,
                                        most_recent_edit,
                                    });
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

    pub(crate) fn serialize_completion(completion: &CoreCompletion) -> proto::Completion {
        proto::Completion {
            old_start: Some(serialize_anchor(&completion.old_range.start)),
            old_end: Some(serialize_anchor(&completion.old_range.end)),
            new_text: completion.new_text.clone(),
            server_id: completion.server_id.0 as u64,
            lsp_completion: serde_json::to_vec(&completion.lsp_completion).unwrap(),
        }
    }

    pub(crate) fn deserialize_completion(completion: proto::Completion) -> Result<CoreCompletion> {
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

    pub(crate) fn serialize_code_action(action: &CodeAction) -> proto::CodeAction {
        proto::CodeAction {
            server_id: action.server_id.0 as u64,
            start: Some(serialize_anchor(&action.range.start)),
            end: Some(serialize_anchor(&action.range.end)),
            lsp_action: serde_json::to_vec(&action.lsp_action).unwrap(),
        }
    }

    pub(crate) fn deserialize_code_action(action: proto::CodeAction) -> Result<CodeAction> {
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
}

impl EventEmitter<LspStoreEvent> for LspStore {}

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
            Some(prepare_completion_documentation(docs, language_registry, language.clone()).await)
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

#[derive(Debug)]
pub enum LanguageServerToQuery {
    Primary,
    Other(LanguageServerId),
}

#[derive(Default)]
struct LanguageServerWatchedPaths {
    worktree_paths: HashMap<WorktreeId, GlobSet>,
    abs_paths: HashMap<Arc<Path>, (GlobSet, Task<()>)>,
}

#[derive(Default)]
struct LanguageServerWatchedPathsBuilder {
    worktree_paths: HashMap<WorktreeId, GlobSet>,
    abs_paths: HashMap<Arc<Path>, GlobSet>,
}

impl LanguageServerWatchedPathsBuilder {
    fn watch_worktree(&mut self, worktree_id: WorktreeId, glob_set: GlobSet) {
        self.worktree_paths.insert(worktree_id, glob_set);
    }
    fn watch_abs_path(&mut self, path: Arc<Path>, glob_set: GlobSet) {
        self.abs_paths.insert(path, glob_set);
    }
    fn build(
        self,
        fs: Arc<dyn Fs>,
        language_server_id: LanguageServerId,
        cx: &mut ModelContext<LspStore>,
    ) -> Model<LanguageServerWatchedPaths> {
        let project = cx.weak_model();

        cx.new_model(|cx| {
                let this_id = cx.entity_id();
                const LSP_ABS_PATH_OBSERVE: Duration = Duration::from_millis(100);
                let abs_paths = self
                    .abs_paths
                    .into_iter()
                    .map(|(abs_path, globset)| {
                        let task = cx.spawn({
                            let abs_path = abs_path.clone();
                            let fs = fs.clone();

                            let lsp_store = project.clone();
                            |_, mut cx| async move {
                                maybe!(async move {
                                    let mut push_updates =
                                        fs.watch(&abs_path, LSP_ABS_PATH_OBSERVE).await;
                                    while let Some(update) = push_updates.0.next().await {
                                        let action = lsp_store
                                            .update(&mut cx, |this, cx| {
                                                let Some(local) = this.as_local() else {
                                                    return ControlFlow::Break(());
                                                };
                                                let Some(watcher) = local
                                                    .language_server_watched_paths
                                                    .get(&language_server_id)
                                                else {
                                                    return ControlFlow::Break(());
                                                };
                                                if watcher.entity_id() != this_id {
                                                    // This watcher is no longer registered on the project, which means that we should
                                                    // cease operations.
                                                    return ControlFlow::Break(());
                                                }
                                                let (globs, _) = watcher
                                                    .read(cx)
                                                    .abs_paths
                                                    .get(&abs_path)
                                                    .expect(
                                                    "Watched abs path is not registered with a watcher",
                                                );
                                                let matching_entries = update
                                                    .into_iter()
                                                    .filter(|event| globs.is_match(&event.path))
                                                    .collect::<Vec<_>>();
                                                this.lsp_notify_abs_paths_changed(
                                                    language_server_id,
                                                    matching_entries,
                                                );
                                                ControlFlow::Continue(())
                                            })
                                            .ok()?;

                                        if action.is_break() {
                                            break;
                                        }
                                    }
                                    Some(())
                                })
                                    .await;
                            }
                        });
                        (abs_path, (globset, task))
                    })
                    .collect();
                LanguageServerWatchedPaths {
                    worktree_paths: self.worktree_paths,
                    abs_paths,
                }
            })
    }
}

struct LspBufferSnapshot {
    version: i32,
    snapshot: TextBufferSnapshot,
}

/// A prompt requested by LSP server.
#[derive(Clone, Debug)]
pub struct LanguageServerPromptRequest {
    pub level: PromptLevel,
    pub message: String,
    pub actions: Vec<MessageActionItem>,
    pub lsp_name: String,
    pub(crate) response_channel: Sender<MessageActionItem>,
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
pub enum LanguageServerLogType {
    Log(MessageType),
    Trace(Option<String>),
}

impl LanguageServerLogType {
    pub fn to_proto(&self) -> proto::language_server_log::LogType {
        match self {
            Self::Log(log_type) => {
                let message_type = match *log_type {
                    MessageType::ERROR => 1,
                    MessageType::WARNING => 2,
                    MessageType::INFO => 3,
                    MessageType::LOG => 4,
                    other => {
                        log::warn!("Unknown lsp log message type: {:?}", other);
                        4
                    }
                };
                proto::language_server_log::LogType::LogMessageType(message_type)
            }
            Self::Trace(message) => {
                proto::language_server_log::LogType::LogTrace(proto::LspLogTrace {
                    message: message.clone(),
                })
            }
        }
    }

    pub fn from_proto(log_type: proto::language_server_log::LogType) -> Self {
        match log_type {
            proto::language_server_log::LogType::LogMessageType(message_type) => {
                Self::Log(match message_type {
                    1 => MessageType::ERROR,
                    2 => MessageType::WARNING,
                    3 => MessageType::INFO,
                    4 => MessageType::LOG,
                    _ => MessageType::LOG,
                })
            }
            proto::language_server_log::LogType::LogTrace(trace) => Self::Trace(trace.message),
        }
    }
}

pub enum LanguageServerState {
    Starting(Task<Option<Arc<LanguageServer>>>),

    Running {
        language: LanguageName,
        adapter: Arc<CachedLspAdapter>,
        server: Arc<LanguageServer>,
        simulate_disk_based_diagnostics_completion: Option<Task<()>>,
    },
}

impl std::fmt::Debug for LanguageServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageServerState::Starting(_) => {
                f.debug_struct("LanguageServerState::Starting").finish()
            }
            LanguageServerState::Running { language, .. } => f
                .debug_struct("LanguageServerState::Running")
                .field("language", &language)
                .finish(),
        }
    }
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

fn glob_literal_prefix(glob: &str) -> &str {
    let is_absolute = glob.starts_with(path::MAIN_SEPARATOR);

    let mut literal_end = is_absolute as usize;
    for (i, part) in glob.split(path::MAIN_SEPARATOR).enumerate() {
        if part.contains(['*', '?', '{', '}']) {
            break;
        } else {
            if i > 0 {
                // Account for separator prior to this part
                literal_end += path::MAIN_SEPARATOR.len_utf8();
            }
            literal_end += part.len();
        }
    }
    let literal_end = literal_end.min(glob.len());
    &glob[..literal_end]
}

pub struct SshLspAdapter {
    name: LanguageServerName,
    binary: LanguageServerBinary,
    initialization_options: Option<String>,
    code_action_kinds: Option<Vec<CodeActionKind>>,
}

impl SshLspAdapter {
    pub fn new(
        name: LanguageServerName,
        binary: LanguageServerBinary,
        initialization_options: Option<String>,
        code_action_kinds: Option<String>,
    ) -> Self {
        Self {
            name,
            binary,
            initialization_options,
            code_action_kinds: code_action_kinds
                .as_ref()
                .and_then(|c| serde_json::from_str(c).ok()),
        }
    }
}

#[async_trait(?Send)]
impl LspAdapter for SshLspAdapter {
    fn name(&self) -> LanguageServerName {
        self.name.clone()
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let Some(options) = &self.initialization_options else {
            return Ok(None);
        };
        let result = serde_json::from_str(options)?;
        Ok(result)
    }

    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        self.code_action_kinds.clone()
    }

    async fn check_if_user_installed(
        &self,
        _: &dyn LspAdapterDelegate,
        _: &AsyncAppContext,
    ) -> Option<LanguageServerBinary> {
        Some(self.binary.clone())
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        None
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        anyhow::bail!("SshLspAdapter does not support fetch_latest_server_version")
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        anyhow::bail!("SshLspAdapter does not support fetch_server_binary")
    }
}

pub fn language_server_settings<'a, 'b: 'a>(
    delegate: &'a dyn LspAdapterDelegate,
    language: &LanguageServerName,
    cx: &'b AppContext,
) -> Option<&'a LspSettings> {
    ProjectSettings::get(
        Some(SettingsLocation {
            worktree_id: delegate.worktree_id(),
            path: delegate.worktree_root_path(),
        }),
        cx,
    )
    .lsp
    .get(language)
}

pub struct LocalLspAdapterDelegate {
    lsp_store: WeakModel<LspStore>,
    worktree: worktree::Snapshot,
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
}

impl LocalLspAdapterDelegate {
    fn for_local(
        lsp_store: &LspStore,
        worktree: &Model<Worktree>,
        cx: &mut ModelContext<LspStore>,
    ) -> Arc<Self> {
        let local = lsp_store
            .as_local()
            .expect("LocalLspAdapterDelegate cannot be constructed on a remote");

        let http_client = local.http_client.clone();

        Self::new(lsp_store, worktree, http_client, local.fs.clone(), cx)
    }

    pub fn new(
        lsp_store: &LspStore,
        worktree: &Model<Worktree>,
        http_client: Arc<dyn HttpClient>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<LspStore>,
    ) -> Arc<Self> {
        let worktree_id = worktree.read(cx).id();
        let worktree_abs_path = worktree.read(cx).abs_path();
        let load_shell_env_task = if let Some(environment) =
            &lsp_store.as_local().map(|local| local.environment.clone())
        {
            environment.update(cx, |env, cx| {
                env.get_environment(Some(worktree_id), Some(worktree_abs_path), cx)
            })
        } else {
            Task::ready(None).shared()
        };

        Arc::new(Self {
            lsp_store: cx.weak_model(),
            worktree: worktree.read(cx).snapshot(),
            fs,
            http_client,
            language_registry: lsp_store.languages.clone(),
            load_shell_env_task,
        })
    }
}

#[async_trait]
impl LspAdapterDelegate for LocalLspAdapterDelegate {
    fn show_notification(&self, message: &str, cx: &mut AppContext) {
        self.lsp_store
            .update(cx, |_, cx| {
                cx.emit(LspStoreEvent::Notification(message.to_owned()))
            })
            .ok();
    }

    fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    fn worktree_id(&self) -> WorktreeId {
        self.worktree.id()
    }

    fn worktree_root_path(&self) -> &Path {
        self.worktree.abs_path().as_ref()
    }

    async fn shell_env(&self) -> HashMap<String, String> {
        let task = self.load_shell_env_task.clone();
        task.await.unwrap_or_default()
    }

    async fn npm_package_installed_version(
        &self,
        package_name: &str,
    ) -> Result<Option<(PathBuf, String)>> {
        let local_package_directory = self.worktree_root_path();
        let node_modules_directory = local_package_directory.join("node_modules");

        if let Some(version) =
            read_package_installed_version(node_modules_directory.clone(), package_name).await?
        {
            return Ok(Some((node_modules_directory, version)));
        }
        let Some(npm) = self.which("npm".as_ref()).await else {
            log::warn!(
                "Failed to find npm executable for {:?}",
                local_package_directory
            );
            return Ok(None);
        };

        let env = self.shell_env().await;
        let output = smol::process::Command::new(&npm)
            .args(["root", "-g"])
            .envs(env)
            .current_dir(local_package_directory)
            .output()
            .await?;
        let global_node_modules =
            PathBuf::from(String::from_utf8_lossy(&output.stdout).to_string());

        if let Some(version) =
            read_package_installed_version(global_node_modules.clone(), package_name).await?
        {
            return Ok(Some((global_node_modules, version)));
        }
        return Ok(None);
    }

    #[cfg(not(target_os = "windows"))]
    async fn which(&self, command: &OsStr) -> Option<PathBuf> {
        let worktree_abs_path = self.worktree.abs_path();
        let shell_path = self.shell_env().await.get("PATH").cloned();
        which::which_in(command, shell_path.as_ref(), worktree_abs_path).ok()
    }

    #[cfg(target_os = "windows")]
    async fn which(&self, command: &OsStr) -> Option<PathBuf> {
        // todo(windows) Getting the shell env variables in a current directory on Windows is more complicated than other platforms
        //               there isn't a 'default shell' necessarily. The closest would be the default profile on the windows terminal
        //               SEE: https://learn.microsoft.com/en-us/windows/terminal/customize-settings/startup
        which::which(command).ok()
    }

    async fn try_exec(&self, command: LanguageServerBinary) -> Result<()> {
        let working_dir = self.worktree_root_path();
        let output = smol::process::Command::new(&command.path)
            .args(command.arguments)
            .envs(command.env.clone().unwrap_or_default())
            .current_dir(working_dir)
            .output()
            .await?;

        if output.status.success() {
            return Ok(());
        }
        Err(anyhow!(
            "{}, stdout: {:?}, stderr: {:?}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }

    fn update_status(
        &self,
        server_name: LanguageServerName,
        status: language::LanguageServerBinaryStatus,
    ) {
        self.language_registry
            .update_lsp_status(server_name, status);
    }

    async fn language_server_download_dir(&self, name: &LanguageServerName) -> Option<Arc<Path>> {
        let dir = self.language_registry.language_server_download_dir(name)?;

        if !dir.exists() {
            smol::fs::create_dir_all(&dir)
                .await
                .context("failed to create container directory")
                .log_err()?;
        }

        Some(dir)
    }

    async fn read_text_file(&self, path: PathBuf) -> Result<String> {
        let entry = self
            .worktree
            .entry_for_path(&path)
            .with_context(|| format!("no worktree entry for path {path:?}"))?;
        let abs_path = self
            .worktree
            .absolutize(&entry.path)
            .with_context(|| format!("cannot absolutize path {path:?}"))?;

        self.fs.load(&abs_path).await
    }
}

async fn populate_labels_for_symbols(
    symbols: Vec<CoreSymbol>,
    language_registry: &Arc<LanguageRegistry>,
    default_language: Option<LanguageName>,
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
                default_language.as_ref().and_then(|name| {
                    language_registry
                        .language_for_name(&name.0)
                        .now_or_never()?
                        .ok()
                })
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
            let lsp_adapter = lsp_adapter.clone().or_else(|| {
                language_registry
                    .lsp_adapters(&language.name())
                    .first()
                    .cloned()
            });
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

fn include_text(server: &lsp::LanguageServer) -> Option<bool> {
    match server.capabilities().text_document_sync.as_ref()? {
        lsp::TextDocumentSyncCapability::Kind(kind) => match *kind {
            lsp::TextDocumentSyncKind::NONE => None,
            lsp::TextDocumentSyncKind::FULL => Some(true),
            lsp::TextDocumentSyncKind::INCREMENTAL => Some(false),
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

#[cfg(test)]
#[test]
fn test_glob_literal_prefix() {
    assert_eq!(glob_literal_prefix("**/*.js"), "");
    assert_eq!(glob_literal_prefix("node_modules/**/*.js"), "node_modules");
    assert_eq!(glob_literal_prefix("foo/{bar,baz}.js"), "foo");
    assert_eq!(glob_literal_prefix("foo/bar/baz.js"), "foo/bar/baz.js");
}
