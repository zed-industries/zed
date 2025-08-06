pub mod clangd_ext;
pub mod json_language_server_ext;
pub mod lsp_ext_command;
pub mod rust_analyzer_ext;

use crate::{
    CodeAction, ColorPresentation, Completion, CompletionResponse, CompletionSource,
    CoreCompletion, DocumentColor, Hover, InlayHint, LocationLink, LspAction, LspPullDiagnostics,
    ProjectItem, ProjectPath, ProjectTransaction, PulledDiagnostics, ResolveState, Symbol,
    ToolchainStore,
    buffer_store::{BufferStore, BufferStoreEvent},
    environment::ProjectEnvironment,
    lsp_command::{self, *},
    lsp_store,
    manifest_tree::{
        AdapterQuery, LanguageServerTree, LanguageServerTreeNode, LaunchDisposition,
        ManifestQueryDelegate, ManifestTree,
    },
    prettier_store::{self, PrettierStore, PrettierStoreEvent},
    project_settings::{LspSettings, ProjectSettings},
    relativize_path, resolve_path,
    toolchain_store::{EmptyToolchainStore, ToolchainStoreEvent},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
    yarn::YarnPathStore,
};
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use client::{TypedEnvelope, proto};
use clock::Global;
use collections::{BTreeMap, BTreeSet, HashMap, HashSet, btree_map};
use futures::{
    AsyncWriteExt, Future, FutureExt, StreamExt,
    future::{Either, Shared, join_all, pending, select},
    select, select_biased,
    stream::FuturesUnordered,
};
use globset::{Glob, GlobBuilder, GlobMatcher, GlobSet, GlobSetBuilder};
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, PromptLevel, SharedString, Task,
    WeakEntity,
};
use http_client::HttpClient;
use itertools::Itertools as _;
use language::{
    Bias, BinaryStatus, Buffer, BufferSnapshot, CachedLspAdapter, CodeLabel, Diagnostic,
    DiagnosticEntry, DiagnosticSet, DiagnosticSourceKind, Diff, File as _, Language, LanguageName,
    LanguageRegistry, LanguageToolchainStore, LocalFile, LspAdapter, LspAdapterDelegate, Patch,
    PointUtf16, TextBufferSnapshot, ToOffset, ToPointUtf16, Transaction, Unclipped,
    WorkspaceFoldersContent,
    language_settings::{
        FormatOnSave, Formatter, LanguageSettings, SelectedFormatter, language_settings,
    },
    point_to_lsp,
    proto::{
        deserialize_anchor, deserialize_lsp_edit, deserialize_version, serialize_anchor,
        serialize_lsp_edit, serialize_version,
    },
    range_from_lsp, range_to_lsp,
};
use lsp::{
    AdapterServerCapabilities, CodeActionKind, CompletionContext, DiagnosticSeverity,
    DiagnosticTag, DidChangeWatchedFilesRegistrationOptions, Edit, FileOperationFilter,
    FileOperationPatternKind, FileOperationRegistrationOptions, FileRename, FileSystemWatcher,
    LanguageServer, LanguageServerBinary, LanguageServerBinaryOptions, LanguageServerId,
    LanguageServerName, LanguageServerSelector, LspRequestFuture, MessageActionItem, MessageType,
    OneOf, RenameFilesParams, SymbolKind, TextEdit, WillRenameFiles, WorkDoneProgressCancelParams,
    WorkspaceFolder, notification::DidRenameFiles,
};
use node_runtime::read_package_installed_version;
use parking_lot::Mutex;
use postage::{mpsc, sink::Sink, stream::Stream, watch};
use rand::prelude::*;

use rpc::{
    AnyProtoClient,
    proto::{FromProto, ToProto},
};
use serde::Serialize;
use settings::{Settings, SettingsLocation, SettingsStore};
use sha2::{Digest, Sha256};
use smol::channel::Sender;
use snippet::Snippet;
use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    cmp::{Ordering, Reverse},
    convert::TryInto,
    ffi::OsStr,
    future::ready,
    iter, mem,
    ops::{ControlFlow, Range},
    path::{self, Path, PathBuf},
    pin::pin,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};
use sum_tree::Dimensions;
use text::{Anchor, BufferId, LineEnding, OffsetRangeExt};
use url::Url;
use util::{
    ConnectionResult, ResultExt as _, debug_panic, defer, maybe, merge_json_value_into,
    paths::{PathExt, SanitizedPath},
    post_inc,
};

pub use fs::*;
pub use language::Location;
#[cfg(any(test, feature = "test-support"))]
pub use prettier::FORMAT_SUFFIX as TEST_PRETTIER_FORMAT_SUFFIX;
pub use worktree::{
    Entry, EntryKind, FS_WATCH_LATENCY, File, LocalWorktree, PathChange, ProjectEntryId,
    UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree, WorktreeId, WorktreeSettings,
};

const SERVER_LAUNCHING_BEFORE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);
pub const SERVER_PROGRESS_THROTTLE_TIMEOUT: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatTrigger {
    Save,
    Manual,
}

pub enum LspFormatTarget {
    Buffers,
    Ranges(BTreeMap<BufferId, Vec<Range<Anchor>>>),
}

pub type OpenLspBufferHandle = Entity<Entity<Buffer>>;

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
    weak: WeakEntity<LspStore>,
    worktree_store: Entity<WorktreeStore>,
    toolchain_store: Entity<ToolchainStore>,
    http_client: Arc<dyn HttpClient>,
    environment: Entity<ProjectEnvironment>,
    fs: Arc<dyn Fs>,
    languages: Arc<LanguageRegistry>,
    language_server_ids: HashMap<(WorktreeId, LanguageServerName), BTreeSet<LanguageServerId>>,
    yarn: Entity<YarnPathStore>,
    pub language_servers: HashMap<LanguageServerId, LanguageServerState>,
    buffers_being_formatted: HashSet<BufferId>,
    last_workspace_edits_by_language_server: HashMap<LanguageServerId, ProjectTransaction>,
    language_server_watched_paths: HashMap<LanguageServerId, LanguageServerWatchedPaths>,
    language_server_paths_watched_for_rename:
        HashMap<LanguageServerId, RenamePathsWatchedForServer>,
    language_server_watcher_registrations:
        HashMap<LanguageServerId, HashMap<String, Vec<FileSystemWatcher>>>,
    supplementary_language_servers:
        HashMap<LanguageServerId, (LanguageServerName, Arc<LanguageServer>)>,
    prettier_store: Entity<PrettierStore>,
    next_diagnostic_group_id: usize,
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
    buffer_snapshots: HashMap<BufferId, HashMap<LanguageServerId, Vec<LspBufferSnapshot>>>, // buffer_id -> server_id -> vec of snapshots
    _subscription: gpui::Subscription,
    lsp_tree: Entity<LanguageServerTree>,
    registered_buffers: HashMap<BufferId, usize>,
    buffers_opened_in_servers: HashMap<BufferId, HashSet<LanguageServerId>>,
    buffer_pull_diagnostics_result_ids: HashMap<LanguageServerId, HashMap<PathBuf, Option<String>>>,
}

impl LocalLspStore {
    /// Returns the running language server for the given ID. Note if the language server is starting, it will not be returned.
    pub fn running_language_server_for_id(
        &self,
        id: LanguageServerId,
    ) -> Option<&Arc<LanguageServer>> {
        let language_server_state = self.language_servers.get(&id)?;

        match language_server_state {
            LanguageServerState::Running { server, .. } => Some(server),
            LanguageServerState::Starting { .. } => None,
        }
    }

    fn start_language_server(
        &mut self,
        worktree_handle: &Entity<Worktree>,
        delegate: Arc<LocalLspAdapterDelegate>,
        adapter: Arc<CachedLspAdapter>,
        settings: Arc<LspSettings>,
        cx: &mut App,
    ) -> LanguageServerId {
        let worktree = worktree_handle.read(cx);
        let worktree_id = worktree.id();
        let root_path = worktree.abs_path();
        let key = (worktree_id, adapter.name.clone());

        let override_options = settings.initialization_options.clone();

        let stderr_capture = Arc::new(Mutex::new(Some(String::new())));

        let server_id = self.languages.next_language_server_id();
        log::info!(
            "attempting to start language server {:?}, path: {root_path:?}, id: {server_id}",
            adapter.name.0
        );

        let binary = self.get_language_server_binary(adapter.clone(), delegate.clone(), true, cx);
        let pending_workspace_folders: Arc<Mutex<BTreeSet<Url>>> = Default::default();

        let pending_server = cx.spawn({
            let adapter = adapter.clone();
            let server_name = adapter.name.clone();
            let stderr_capture = stderr_capture.clone();
            #[cfg(any(test, feature = "test-support"))]
            let lsp_store = self.weak.clone();
            let pending_workspace_folders = pending_workspace_folders.clone();
            async move |cx| {
                let binary = binary.await?;
                #[cfg(any(test, feature = "test-support"))]
                if let Some(server) = lsp_store
                    .update(&mut cx.clone(), |this, cx| {
                        this.languages.create_fake_language_server(
                            server_id,
                            &server_name,
                            binary.clone(),
                            &mut cx.to_async(),
                        )
                    })
                    .ok()
                    .flatten()
                {
                    return Ok(server);
                }

                let code_action_kinds = adapter.code_action_kinds();
                lsp::LanguageServer::new(
                    stderr_capture,
                    server_id,
                    server_name,
                    binary,
                    &root_path,
                    code_action_kinds,
                    Some(pending_workspace_folders).filter(|_| {
                        adapter.adapter.workspace_folders_content()
                            == WorkspaceFoldersContent::SubprojectRoots
                    }),
                    cx,
                )
            }
        });

        let startup = {
            let server_name = adapter.name.0.clone();
            let delegate = delegate as Arc<dyn LspAdapterDelegate>;
            let key = key.clone();
            let adapter = adapter.clone();
            let lsp_store = self.weak.clone();
            let pending_workspace_folders = pending_workspace_folders.clone();
            let fs = self.fs.clone();
            let pull_diagnostics = ProjectSettings::get_global(cx)
                .diagnostics
                .lsp_pull_diagnostics
                .enabled;
            cx.spawn(async move |cx| {
                let result = async {
                    let toolchains =
                        lsp_store.update(cx, |lsp_store, cx| lsp_store.toolchain_store(cx))?;
                    let language_server = pending_server.await?;

                    let workspace_config = Self::workspace_configuration_for_adapter(
                        adapter.adapter.clone(),
                        fs.as_ref(),
                        &delegate,
                        toolchains.clone(),
                        cx,
                    )
                    .await?;

                    let mut initialization_options = Self::initialization_options_for_adapter(
                        adapter.adapter.clone(),
                        fs.as_ref(),
                        &delegate,
                    )
                    .await?;

                    match (&mut initialization_options, override_options) {
                        (Some(initialization_options), Some(override_options)) => {
                            merge_json_value_into(override_options, initialization_options);
                        }
                        (None, override_options) => initialization_options = override_options,
                        _ => {}
                    }

                    let initialization_params = cx.update(|cx| {
                        let mut params =
                            language_server.default_initialize_params(pull_diagnostics, cx);
                        params.initialization_options = initialization_options;
                        adapter.adapter.prepare_initialize_params(params, cx)
                    })??;

                    Self::setup_lsp_messages(
                        lsp_store.clone(),
                        fs,
                        &language_server,
                        delegate.clone(),
                        adapter.clone(),
                    );

                    let did_change_configuration_params =
                        Arc::new(lsp::DidChangeConfigurationParams {
                            settings: workspace_config,
                        });
                    let language_server = cx
                        .update(|cx| {
                            language_server.initialize(
                                initialization_params,
                                did_change_configuration_params.clone(),
                                cx,
                            )
                        })?
                        .await
                        .inspect_err(|_| {
                            if let Some(lsp_store) = lsp_store.upgrade() {
                                lsp_store
                                    .update(cx, |lsp_store, cx| {
                                        lsp_store.cleanup_lsp_data(server_id);
                                        cx.emit(LspStoreEvent::LanguageServerRemoved(server_id))
                                    })
                                    .ok();
                            }
                        })?;

                    language_server
                        .notify::<lsp::notification::DidChangeConfiguration>(
                            &did_change_configuration_params,
                        )
                        .ok();

                    anyhow::Ok(language_server)
                }
                .await;

                match result {
                    Ok(server) => {
                        lsp_store
                            .update(cx, |lsp_store, mut cx| {
                                lsp_store.insert_newly_running_language_server(
                                    adapter,
                                    server.clone(),
                                    server_id,
                                    key,
                                    pending_workspace_folders,
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
                            BinaryStatus::Failed {
                                error: format!("{err}\n-- stderr--\n{log}"),
                            },
                        );
                        let message =
                            format!("Failed to start language server {server_name:?}: {err:#?}");
                        log::error!("{message}");
                        log::error!("server stderr: {log}");
                        None
                    }
                }
            })
        };
        let state = LanguageServerState::Starting {
            startup,
            pending_workspace_folders,
        };

        self.languages
            .update_lsp_binary_status(adapter.name(), BinaryStatus::Starting);

        self.language_servers.insert(server_id, state);
        self.language_server_ids
            .entry(key)
            .or_default()
            .insert(server_id);
        server_id
    }

    fn get_language_server_binary(
        &self,
        adapter: Arc<CachedLspAdapter>,
        delegate: Arc<dyn LspAdapterDelegate>,
        allow_binary_download: bool,
        cx: &mut App,
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

            return cx.background_spawn(async move {
                let mut env = delegate.shell_env().await;
                env.extend(settings.env.unwrap_or_default());

                Ok(LanguageServerBinary {
                    path: PathBuf::from(&settings.path.unwrap()),
                    env: Some(env),
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
        let toolchains = self.toolchain_store.read(cx).as_language_toolchain_store();
        cx.spawn(async move |cx| {
            let binary_result = adapter
                .clone()
                .get_language_server_command(delegate.clone(), toolchains, lsp_binary_options, cx)
                .await;

            delegate.update_status(adapter.name.clone(), BinaryStatus::None);

            let mut binary = binary_result?;
            let mut shell_env = delegate.shell_env().await;

            shell_env.extend(binary.env.unwrap_or_default());

            if let Some(settings) = settings {
                if let Some(arguments) = settings.arguments {
                    binary.arguments = arguments.into_iter().map(Into::into).collect();
                }
                if let Some(env) = settings.env {
                    shell_env.extend(env);
                }
            }

            binary.env = Some(shell_env);
            Ok(binary)
        })
    }

    fn setup_lsp_messages(
        this: WeakEntity<LspStore>,
        fs: Arc<dyn Fs>,
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
                move |mut params, cx| {
                    let adapter = adapter.clone();
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            {
                                let buffer = params
                                    .uri
                                    .to_file_path()
                                    .map(|file_path| this.get_buffer(&file_path, cx))
                                    .ok()
                                    .flatten();
                                adapter.process_diagnostics(&mut params, server_id, buffer);
                            }

                            this.merge_diagnostics(
                                server_id,
                                params,
                                None,
                                DiagnosticSourceKind::Pushed,
                                &adapter.disk_based_diagnostic_sources,
                                |_, diagnostic, cx| match diagnostic.source_kind {
                                    DiagnosticSourceKind::Other | DiagnosticSourceKind::Pushed => {
                                        adapter.retain_old_diagnostic(diagnostic, cx)
                                    }
                                    DiagnosticSourceKind::Pulled => true,
                                },
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
                let fs = fs.clone();
                move |params, cx| {
                    let adapter = adapter.clone();
                    let delegate = delegate.clone();
                    let this = this.clone();
                    let fs = fs.clone();
                    let mut cx = cx.clone();
                    async move {
                        let toolchains =
                            this.update(&mut cx, |this, cx| this.toolchain_store(cx))?;

                        let workspace_config = Self::workspace_configuration_for_adapter(
                            adapter.clone(),
                            fs.as_ref(),
                            &delegate,
                            toolchains.clone(),
                            &mut cx,
                        )
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
                move |_, cx| {
                    let this = this.clone();
                    let mut cx = cx.clone();
                    async move {
                        let Some(server) = this
                            .read_with(&mut cx, |this, _| this.language_server_for_id(server_id))?
                        else {
                            return Ok(None);
                        };
                        let root = server.workspace_folders();
                        Ok(Some(
                            root.into_iter()
                                .map(|uri| WorkspaceFolder {
                                    uri,
                                    name: Default::default(),
                                })
                                .collect(),
                        ))
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
                move |params, cx| {
                    let this = this.clone();
                    let mut cx = cx.clone();
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
                move |params, cx| {
                    let lsp_store = this.clone();
                    let mut cx = cx.clone();
                    async move {
                        for reg in params.registrations {
                            match reg.method.as_str() {
                                "workspace/didChangeWatchedFiles" => {
                                    if let Some(options) = reg.register_options {
                                        let options = serde_json::from_value(options)?;
                                        lsp_store.update(&mut cx, |this, cx| {
                                            this.as_local_mut()?.on_lsp_did_change_watched_files(
                                                server_id, &reg.id, options, cx,
                                            );
                                            Some(())
                                        })?;
                                    }
                                }
                                "textDocument/rangeFormatting" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        if let Some(server) =
                                            lsp_store.language_server_for_id(server_id)
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
                                            });
                                            notify_server_capabilities_updated(&server, cx);
                                        }
                                        anyhow::Ok(())
                                    })??;
                                }
                                "textDocument/onTypeFormatting" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        if let Some(server) =
                                            lsp_store.language_server_for_id(server_id)
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
                                                });
                                                notify_server_capabilities_updated(&server, cx);
                                            }
                                        }
                                        anyhow::Ok(())
                                    })??;
                                }
                                "textDocument/formatting" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        if let Some(server) =
                                            lsp_store.language_server_for_id(server_id)
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
                                            });
                                            notify_server_capabilities_updated(&server, cx);
                                        }
                                        anyhow::Ok(())
                                    })??;
                                }
                                "workspace/didChangeConfiguration" => {
                                    // Ignore payload since we notify clients of setting changes unconditionally, relying on them pulling the latest settings.
                                }
                                "textDocument/rename" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        if let Some(server) =
                                            lsp_store.language_server_for_id(server_id)
                                        {
                                            let options = reg
                                                .register_options
                                                .map(|options| {
                                                    serde_json::from_value::<lsp::RenameOptions>(
                                                        options,
                                                    )
                                                })
                                                .transpose()?;
                                            let options = match options {
                                                None => OneOf::Left(true),
                                                Some(options) => OneOf::Right(options),
                                            };

                                            server.update_capabilities(|capabilities| {
                                                capabilities.rename_provider = Some(options);
                                            });
                                            notify_server_capabilities_updated(&server, cx);
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
                move |params, cx| {
                    let lsp_store = this.clone();
                    let mut cx = cx.clone();
                    async move {
                        for unreg in params.unregisterations.iter() {
                            match unreg.method.as_str() {
                                "workspace/didChangeWatchedFiles" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        lsp_store
                                            .as_local_mut()?
                                            .on_lsp_unregister_did_change_watched_files(
                                                server_id, &unreg.id, cx,
                                            );
                                        Some(())
                                    })?;
                                }
                                "workspace/didChangeConfiguration" => {
                                    // Ignore payload since we notify clients of setting changes unconditionally, relying on them pulling the latest settings.
                                }
                                "textDocument/rename" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        if let Some(server) =
                                            lsp_store.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.rename_provider = None
                                            });
                                            notify_server_capabilities_updated(&server, cx);
                                        }
                                    })?;
                                }
                                "textDocument/rangeFormatting" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        if let Some(server) =
                                            lsp_store.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_range_formatting_provider =
                                                    None
                                            });
                                            notify_server_capabilities_updated(&server, cx);
                                        }
                                    })?;
                                }
                                "textDocument/onTypeFormatting" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        if let Some(server) =
                                            lsp_store.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_on_type_formatting_provider =
                                                    None;
                                            });
                                            notify_server_capabilities_updated(&server, cx);
                                        }
                                    })?;
                                }
                                "textDocument/formatting" => {
                                    lsp_store.update(&mut cx, |lsp_store, cx| {
                                        if let Some(server) =
                                            lsp_store.language_server_for_id(server_id)
                                        {
                                            server.update_capabilities(|capabilities| {
                                                capabilities.document_formatting_provider = None;
                                            });
                                            notify_server_capabilities_updated(&server, cx);
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
                    let mut cx = cx.clone();
                    let this = this.clone();
                    let adapter = adapter.clone();
                    async move {
                        LocalLspStore::on_lsp_workspace_edit(
                            this.clone(),
                            params,
                            server_id,
                            adapter.clone(),
                            &mut cx,
                        )
                        .await
                    }
                }
            })
            .detach();

        language_server
            .on_request::<lsp::request::InlayHintRefreshRequest, _, _>({
                let this = this.clone();
                move |(), cx| {
                    let this = this.clone();
                    let mut cx = cx.clone();
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
            .on_request::<lsp::request::CodeLensRefresh, _, _>({
                let this = this.clone();
                move |(), cx| {
                    let this = this.clone();
                    let mut cx = cx.clone();
                    async move {
                        this.update(&mut cx, |this, cx| {
                            cx.emit(LspStoreEvent::RefreshCodeLens);
                            this.downstream_client.as_ref().map(|(client, project_id)| {
                                client.send(proto::RefreshCodeLens {
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
            .on_request::<lsp::request::WorkspaceDiagnosticRefresh, _, _>({
                let this = this.clone();
                move |(), cx| {
                    let this = this.clone();
                    let mut cx = cx.clone();
                    async move {
                        this.update(&mut cx, |lsp_store, _| {
                            lsp_store.pull_workspace_diagnostics(server_id);
                            lsp_store
                                .downstream_client
                                .as_ref()
                                .map(|(client, project_id)| {
                                    client.send(proto::PullWorkspaceDiagnostics {
                                        project_id: *project_id,
                                        server_id: server_id.to_proto(),
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
                move |params, cx| {
                    let this = this.clone();
                    let name = name.to_string();
                    let mut cx = cx.clone();
                    async move {
                        let actions = params.actions.unwrap_or_default();
                        let (tx, rx) = smol::channel::bounded(1);
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
                            let response = rx.recv().await.ok();
                            Ok(response)
                        } else {
                            Ok(None)
                        }
                    }
                }
            })
            .detach();
        language_server
            .on_notification::<lsp::notification::ShowMessage, _>({
                let this = this.clone();
                let name = name.to_string();
                move |params, cx| {
                    let this = this.clone();
                    let name = name.to_string();
                    let mut cx = cx.clone();

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
                move |params, cx| {
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
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
                move |params, cx| {
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |_, cx| {
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
                move |params, cx| {
                    let mut cx = cx.clone();
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

        json_language_server_ext::register_requests(this.clone(), language_server);
        rust_analyzer_ext::register_notifications(this.clone(), language_server);
        clangd_ext::register_notifications(this, language_server, adapter);
    }

    fn shutdown_language_servers_on_quit(
        &mut self,
        _: &mut Context<LspStore>,
    ) -> impl Future<Output = ()> + use<> {
        let shutdown_futures = self
            .language_servers
            .drain()
            .map(|(_, server_state)| Self::shutdown_server(server_state))
            .collect::<Vec<_>>();

        async move {
            join_all(shutdown_futures).await;
        }
    }

    async fn shutdown_server(server_state: LanguageServerState) -> anyhow::Result<()> {
        match server_state {
            LanguageServerState::Running { server, .. } => {
                if let Some(shutdown) = server.shutdown() {
                    shutdown.await;
                }
            }
            LanguageServerState::Starting { startup, .. } => {
                if let Some(server) = startup.await {
                    if let Some(shutdown) = server.shutdown() {
                        shutdown.await;
                    }
                }
            }
        }
        Ok(())
    }

    fn language_servers_for_worktree(
        &self,
        worktree_id: WorktreeId,
    ) -> impl Iterator<Item = &Arc<LanguageServer>> {
        self.language_server_ids
            .iter()
            .flat_map(move |((language_server_path, _), ids)| {
                ids.iter().filter_map(move |id| {
                    if *language_server_path != worktree_id {
                        return None;
                    }
                    if let Some(LanguageServerState::Running { server, .. }) =
                        self.language_servers.get(id)
                    {
                        return Some(server);
                    } else {
                        None
                    }
                })
            })
    }

    fn language_server_ids_for_project_path(
        &self,
        project_path: ProjectPath,
        language: &Language,
        cx: &mut App,
    ) -> Vec<LanguageServerId> {
        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Vec::new();
        };
        let delegate = Arc::new(ManifestQueryDelegate::new(worktree.read(cx).snapshot()));
        let root = self.lsp_tree.update(cx, |this, cx| {
            this.get(
                project_path,
                AdapterQuery::Language(&language.name()),
                delegate,
                cx,
            )
            .filter_map(|node| node.server_id())
            .collect::<Vec<_>>()
        });

        root
    }

    fn language_server_ids_for_buffer(
        &self,
        buffer: &Buffer,
        cx: &mut App,
    ) -> Vec<LanguageServerId> {
        if let Some((file, language)) = File::from_dyn(buffer.file()).zip(buffer.language()) {
            let worktree_id = file.worktree_id(cx);

            let path: Arc<Path> = file
                .path()
                .parent()
                .map(Arc::from)
                .unwrap_or_else(|| file.path().clone());
            let worktree_path = ProjectPath { worktree_id, path };
            self.language_server_ids_for_project_path(worktree_path, language, cx)
        } else {
            Vec::new()
        }
    }

    fn language_servers_for_buffer<'a>(
        &'a self,
        buffer: &'a Buffer,
        cx: &'a mut App,
    ) -> impl Iterator<Item = (&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        self.language_server_ids_for_buffer(buffer, cx)
            .into_iter()
            .filter_map(|server_id| match self.language_servers.get(&server_id)? {
                LanguageServerState::Running {
                    adapter, server, ..
                } => Some((adapter, server)),
                _ => None,
            })
    }

    async fn execute_code_action_kind_locally(
        lsp_store: WeakEntity<LspStore>,
        mut buffers: Vec<Entity<Buffer>>,
        kind: CodeActionKind,
        push_to_history: bool,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<ProjectTransaction> {
        // Do not allow multiple concurrent code actions requests for the
        // same buffer.
        lsp_store.update(cx, |this, cx| {
            let this = this.as_local_mut().unwrap();
            buffers.retain(|buffer| {
                this.buffers_being_formatted
                    .insert(buffer.read(cx).remote_id())
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
                            .remove(&buffer.read(cx).remote_id());
                    }
                })
                .ok();
            }
        });
        let mut project_transaction = ProjectTransaction::default();

        for buffer in &buffers {
            let adapters_and_servers = lsp_store.update(cx, |lsp_store, cx| {
                buffer.update(cx, |buffer, cx| {
                    lsp_store
                        .as_local()
                        .unwrap()
                        .language_servers_for_buffer(buffer, cx)
                        .map(|(adapter, lsp)| (adapter.clone(), lsp.clone()))
                        .collect::<Vec<_>>()
                })
            })?;
            for (lsp_adapter, language_server) in adapters_and_servers.iter() {
                let actions = Self::get_server_code_actions_from_action_kinds(
                    &lsp_store,
                    language_server.server_id(),
                    vec![kind.clone()],
                    buffer,
                    cx,
                )
                .await?;
                Self::execute_code_actions_on_server(
                    &lsp_store,
                    language_server,
                    lsp_adapter,
                    actions,
                    push_to_history,
                    &mut project_transaction,
                    cx,
                )
                .await?;
            }
        }
        Ok(project_transaction)
    }

    async fn format_locally(
        lsp_store: WeakEntity<LspStore>,
        mut buffers: Vec<FormattableBuffer>,
        push_to_history: bool,
        trigger: FormatTrigger,
        logger: zlog::Logger,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<ProjectTransaction> {
        // Do not allow multiple concurrent formatting requests for the
        // same buffer.
        lsp_store.update(cx, |this, cx| {
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
            zlog::debug!(
                logger =>
                "formatting buffer '{:?}'",
                buffer.abs_path.as_ref().unwrap_or(&PathBuf::from("unknown")).display()
            );
            // Create an empty transaction to hold all of the formatting edits.
            let formatting_transaction_id = buffer.handle.update(cx, |buffer, cx| {
                // ensure no transactions created while formatting are
                // grouped with the previous transaction in the history
                // based on the transaction group interval
                buffer.finalize_last_transaction();
                buffer
                    .start_transaction()
                    .context("transaction already open")?;
                buffer.end_transaction(cx);
                let transaction_id = buffer.push_empty_transaction(cx.background_executor().now());
                buffer.finalize_last_transaction();
                anyhow::Ok(transaction_id)
            })??;

            let result = Self::format_buffer_locally(
                lsp_store.clone(),
                buffer,
                formatting_transaction_id,
                trigger,
                logger,
                cx,
            )
            .await;

            buffer.handle.update(cx, |buffer, cx| {
                let Some(formatting_transaction) =
                    buffer.get_transaction(formatting_transaction_id).cloned()
                else {
                    zlog::warn!(logger => "no formatting transaction");
                    return;
                };
                if formatting_transaction.edit_ids.is_empty() {
                    zlog::debug!(logger => "no changes made while formatting");
                    buffer.forget_transaction(formatting_transaction_id);
                    return;
                }
                if !push_to_history {
                    zlog::trace!(logger => "forgetting format transaction");
                    buffer.forget_transaction(formatting_transaction.id);
                }
                project_transaction
                    .0
                    .insert(cx.entity(), formatting_transaction);
            })?;

            result?;
        }

        Ok(project_transaction)
    }

    async fn format_buffer_locally(
        lsp_store: WeakEntity<LspStore>,
        buffer: &FormattableBuffer,
        formatting_transaction_id: clock::Lamport,
        trigger: FormatTrigger,
        logger: zlog::Logger,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let (adapters_and_servers, settings) = lsp_store.update(cx, |lsp_store, cx| {
            buffer.handle.update(cx, |buffer, cx| {
                let adapters_and_servers = lsp_store
                    .as_local()
                    .unwrap()
                    .language_servers_for_buffer(buffer, cx)
                    .map(|(adapter, lsp)| (adapter.clone(), lsp.clone()))
                    .collect::<Vec<_>>();
                let settings =
                    language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx)
                        .into_owned();
                (adapters_and_servers, settings)
            })
        })?;

        /// Apply edits to the buffer that will become part of the formatting transaction.
        /// Fails if the buffer has been edited since the start of that transaction.
        fn extend_formatting_transaction(
            buffer: &FormattableBuffer,
            formatting_transaction_id: text::TransactionId,
            cx: &mut AsyncApp,
            operation: impl FnOnce(&mut Buffer, &mut Context<Buffer>),
        ) -> anyhow::Result<()> {
            buffer.handle.update(cx, |buffer, cx| {
                let last_transaction_id = buffer.peek_undo_stack().map(|t| t.transaction_id());
                if last_transaction_id != Some(formatting_transaction_id) {
                    anyhow::bail!("Buffer edited while formatting. Aborting")
                }
                buffer.start_transaction();
                operation(buffer, cx);
                if let Some(transaction_id) = buffer.end_transaction(cx) {
                    buffer.merge_transactions(transaction_id, formatting_transaction_id);
                }
                Ok(())
            })?
        }

        // handle whitespace formatting
        if settings.remove_trailing_whitespace_on_save {
            zlog::trace!(logger => "removing trailing whitespace");
            let diff = buffer
                .handle
                .read_with(cx, |buffer, cx| buffer.remove_trailing_whitespace(cx))?
                .await;
            extend_formatting_transaction(buffer, formatting_transaction_id, cx, |buffer, cx| {
                buffer.apply_diff(diff, cx);
            })?;
        }

        if settings.ensure_final_newline_on_save {
            zlog::trace!(logger => "ensuring final newline");
            extend_formatting_transaction(buffer, formatting_transaction_id, cx, |buffer, cx| {
                buffer.ensure_final_newline(cx);
            })?;
        }

        // Formatter for `code_actions_on_format` that runs before
        // the rest of the formatters
        let mut code_actions_on_format_formatter = None;
        let should_run_code_actions_on_format = !matches!(
            (trigger, &settings.format_on_save),
            (FormatTrigger::Save, &FormatOnSave::Off)
        );
        if should_run_code_actions_on_format {
            let have_code_actions_to_run_on_format = settings
                .code_actions_on_format
                .values()
                .any(|enabled| *enabled);
            if have_code_actions_to_run_on_format {
                zlog::trace!(logger => "going to run code actions on format");
                code_actions_on_format_formatter = Some(Formatter::CodeActions(
                    settings.code_actions_on_format.clone(),
                ));
            }
        }

        let formatters = match (trigger, &settings.format_on_save) {
            (FormatTrigger::Save, FormatOnSave::Off) => &[],
            (FormatTrigger::Save, FormatOnSave::List(formatters)) => formatters.as_ref(),
            (FormatTrigger::Manual, _) | (FormatTrigger::Save, FormatOnSave::On) => {
                match &settings.formatter {
                    SelectedFormatter::Auto => {
                        if settings.prettier.allowed {
                            zlog::trace!(logger => "Formatter set to auto: defaulting to prettier");
                            std::slice::from_ref(&Formatter::Prettier)
                        } else {
                            zlog::trace!(logger => "Formatter set to auto: defaulting to primary language server");
                            std::slice::from_ref(&Formatter::LanguageServer { name: None })
                        }
                    }
                    SelectedFormatter::List(formatter_list) => formatter_list.as_ref(),
                }
            }
        };

        let formatters = code_actions_on_format_formatter.iter().chain(formatters);

        for formatter in formatters {
            match formatter {
                Formatter::Prettier => {
                    let logger = zlog::scoped!(logger => "prettier");
                    zlog::trace!(logger => "formatting");
                    let _timer = zlog::time!(logger => "Formatting buffer via prettier");

                    let prettier = lsp_store.read_with(cx, |lsp_store, _cx| {
                        lsp_store.prettier_store().unwrap().downgrade()
                    })?;
                    let diff = prettier_store::format_with_prettier(&prettier, &buffer.handle, cx)
                        .await
                        .transpose()?;
                    let Some(diff) = diff else {
                        zlog::trace!(logger => "No changes");
                        continue;
                    };

                    extend_formatting_transaction(
                        buffer,
                        formatting_transaction_id,
                        cx,
                        |buffer, cx| {
                            buffer.apply_diff(diff, cx);
                        },
                    )?;
                }
                Formatter::External { command, arguments } => {
                    let logger = zlog::scoped!(logger => "command");
                    zlog::trace!(logger => "formatting");
                    let _timer = zlog::time!(logger => "Formatting buffer via external command");

                    let diff = Self::format_via_external_command(
                        buffer,
                        command.as_ref(),
                        arguments.as_deref(),
                        cx,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to format buffer via external command: {}", command)
                    })?;
                    let Some(diff) = diff else {
                        zlog::trace!(logger => "No changes");
                        continue;
                    };

                    extend_formatting_transaction(
                        buffer,
                        formatting_transaction_id,
                        cx,
                        |buffer, cx| {
                            buffer.apply_diff(diff, cx);
                        },
                    )?;
                }
                Formatter::LanguageServer { name } => {
                    let logger = zlog::scoped!(logger => "language-server");
                    zlog::trace!(logger => "formatting");
                    let _timer = zlog::time!(logger => "Formatting buffer using language server");

                    let Some(buffer_path_abs) = buffer.abs_path.as_ref() else {
                        zlog::warn!(logger => "Cannot format buffer that is not backed by a file on disk using language servers. Skipping");
                        continue;
                    };

                    let language_server = if let Some(name) = name.as_deref() {
                        adapters_and_servers.iter().find_map(|(adapter, server)| {
                            if adapter.name.0.as_ref() == name {
                                Some(server.clone())
                            } else {
                                None
                            }
                        })
                    } else {
                        adapters_and_servers.first().map(|e| e.1.clone())
                    };

                    let Some(language_server) = language_server else {
                        log::debug!(
                            "No language server found to format buffer '{:?}'. Skipping",
                            buffer_path_abs.as_path().to_string_lossy()
                        );
                        continue;
                    };

                    zlog::trace!(
                        logger =>
                        "Formatting buffer '{:?}' using language server '{:?}'",
                        buffer_path_abs.as_path().to_string_lossy(),
                        language_server.name()
                    );

                    let edits = if let Some(ranges) = buffer.ranges.as_ref() {
                        zlog::trace!(logger => "formatting ranges");
                        Self::format_ranges_via_lsp(
                            &lsp_store,
                            &buffer.handle,
                            ranges,
                            buffer_path_abs,
                            &language_server,
                            &settings,
                            cx,
                        )
                        .await
                        .context("Failed to format ranges via language server")?
                    } else {
                        zlog::trace!(logger => "formatting full");
                        Self::format_via_lsp(
                            &lsp_store,
                            &buffer.handle,
                            buffer_path_abs,
                            &language_server,
                            &settings,
                            cx,
                        )
                        .await
                        .context("failed to format via language server")?
                    };

                    if edits.is_empty() {
                        zlog::trace!(logger => "No changes");
                        continue;
                    }
                    extend_formatting_transaction(
                        buffer,
                        formatting_transaction_id,
                        cx,
                        |buffer, cx| {
                            buffer.edit(edits, None, cx);
                        },
                    )?;
                }
                Formatter::CodeActions(code_actions) => {
                    let logger = zlog::scoped!(logger => "code-actions");
                    zlog::trace!(logger => "formatting");
                    let _timer = zlog::time!(logger => "Formatting buffer using code actions");

                    let Some(buffer_path_abs) = buffer.abs_path.as_ref() else {
                        zlog::warn!(logger => "Cannot format buffer that is not backed by a file on disk using code actions. Skipping");
                        continue;
                    };
                    let code_action_kinds = code_actions
                        .iter()
                        .filter_map(|(action_kind, enabled)| {
                            enabled.then_some(action_kind.clone().into())
                        })
                        .collect::<Vec<_>>();
                    if code_action_kinds.is_empty() {
                        zlog::trace!(logger => "No code action kinds enabled, skipping");
                        continue;
                    }
                    zlog::trace!(logger => "Attempting to resolve code actions {:?}", &code_action_kinds);

                    let mut actions_and_servers = Vec::new();

                    for (index, (_, language_server)) in adapters_and_servers.iter().enumerate() {
                        let actions_result = Self::get_server_code_actions_from_action_kinds(
                            &lsp_store,
                            language_server.server_id(),
                            code_action_kinds.clone(),
                            &buffer.handle,
                            cx,
                        )
                        .await
                        .with_context(
                            || format!("Failed to resolve code actions with kinds {:?} for language server {}",
                                code_action_kinds.iter().map(|kind| kind.as_str()).join(", "),
                                language_server.name())
                        );
                        let Ok(actions) = actions_result else {
                            // note: it may be better to set result to the error and break formatters here
                            // but for now we try to execute the actions that we can resolve and skip the rest
                            zlog::error!(
                                logger =>
                                "Failed to resolve code actions with kinds {:?} with language server {}",
                                code_action_kinds.iter().map(|kind| kind.as_str()).join(", "),
                                language_server.name()
                            );
                            continue;
                        };
                        for action in actions {
                            actions_and_servers.push((action, index));
                        }
                    }

                    if actions_and_servers.is_empty() {
                        zlog::warn!(logger => "No code actions were resolved, continuing");
                        continue;
                    }

                    'actions: for (mut action, server_index) in actions_and_servers {
                        let server = &adapters_and_servers[server_index].1;

                        let describe_code_action = |action: &CodeAction| {
                            format!(
                                "code action '{}' with title \"{}\" on server {}",
                                action
                                    .lsp_action
                                    .action_kind()
                                    .unwrap_or("unknown".into())
                                    .as_str(),
                                action.lsp_action.title(),
                                server.name(),
                            )
                        };

                        zlog::trace!(logger => "Executing {}", describe_code_action(&action));

                        if let Err(err) = Self::try_resolve_code_action(server, &mut action).await {
                            zlog::error!(
                                logger =>
                                "Failed to resolve {}. Error: {}",
                                describe_code_action(&action),
                                err
                            );
                            continue;
                        }

                        if let Some(edit) = action.lsp_action.edit().cloned() {
                            // NOTE: code below duplicated from `Self::deserialize_workspace_edit`
                            // but filters out and logs warnings for code actions that cause unreasonably
                            // difficult handling on our part, such as:
                            // - applying edits that call commands
                            //   which can result in arbitrary workspace edits being sent from the server that
                            //   have no way of being tied back to the command that initiated them (i.e. we
                            //   can't know which edits are part of the format request, or if the server is done sending
                            //   actions in response to the command)
                            // - actions that create/delete/modify/rename files other than the one we are formatting
                            //   as we then would need to handle such changes correctly in the local history as well
                            //   as the remote history through the ProjectTransaction
                            // - actions with snippet edits, as these simply don't make sense in the context of a format request
                            // Supporting these actions is not impossible, but not supported as of yet.
                            if edit.changes.is_none() && edit.document_changes.is_none() {
                                zlog::trace!(
                                    logger =>
                                    "No changes for code action. Skipping {}",
                                    describe_code_action(&action),
                                );
                                continue;
                            }

                            let mut operations = Vec::new();
                            if let Some(document_changes) = edit.document_changes {
                                match document_changes {
                                    lsp::DocumentChanges::Edits(edits) => operations.extend(
                                        edits.into_iter().map(lsp::DocumentChangeOperation::Edit),
                                    ),
                                    lsp::DocumentChanges::Operations(ops) => operations = ops,
                                }
                            } else if let Some(changes) = edit.changes {
                                operations.extend(changes.into_iter().map(|(uri, edits)| {
                                    lsp::DocumentChangeOperation::Edit(lsp::TextDocumentEdit {
                                        text_document:
                                            lsp::OptionalVersionedTextDocumentIdentifier {
                                                uri,
                                                version: None,
                                            },
                                        edits: edits.into_iter().map(Edit::Plain).collect(),
                                    })
                                }));
                            }

                            let mut edits = Vec::with_capacity(operations.len());

                            if operations.is_empty() {
                                zlog::trace!(
                                    logger =>
                                    "No changes for code action. Skipping {}",
                                    describe_code_action(&action),
                                );
                                continue;
                            }
                            for operation in operations {
                                let op = match operation {
                                    lsp::DocumentChangeOperation::Edit(op) => op,
                                    lsp::DocumentChangeOperation::Op(_) => {
                                        zlog::warn!(
                                            logger =>
                                            "Code actions which create, delete, or rename files are not supported on format. Skipping {}",
                                            describe_code_action(&action),
                                        );
                                        continue 'actions;
                                    }
                                };
                                let Ok(file_path) = op.text_document.uri.to_file_path() else {
                                    zlog::warn!(
                                        logger =>
                                        "Failed to convert URI '{:?}' to file path. Skipping {}",
                                        &op.text_document.uri,
                                        describe_code_action(&action),
                                    );
                                    continue 'actions;
                                };
                                if &file_path != buffer_path_abs {
                                    zlog::warn!(
                                        logger =>
                                        "File path '{:?}' does not match buffer path '{:?}'. Skipping {}",
                                        file_path,
                                        buffer_path_abs,
                                        describe_code_action(&action),
                                    );
                                    continue 'actions;
                                }

                                let mut lsp_edits = Vec::new();
                                for edit in op.edits {
                                    match edit {
                                        Edit::Plain(edit) => {
                                            if !lsp_edits.contains(&edit) {
                                                lsp_edits.push(edit);
                                            }
                                        }
                                        Edit::Annotated(edit) => {
                                            if !lsp_edits.contains(&edit.text_edit) {
                                                lsp_edits.push(edit.text_edit);
                                            }
                                        }
                                        Edit::Snippet(_) => {
                                            zlog::warn!(
                                                logger =>
                                                "Code actions which produce snippet edits are not supported during formatting. Skipping {}",
                                                describe_code_action(&action),
                                            );
                                            continue 'actions;
                                        }
                                    }
                                }
                                let edits_result = lsp_store
                                    .update(cx, |lsp_store, cx| {
                                        lsp_store.as_local_mut().unwrap().edits_from_lsp(
                                            &buffer.handle,
                                            lsp_edits,
                                            server.server_id(),
                                            op.text_document.version,
                                            cx,
                                        )
                                    })?
                                    .await;
                                let Ok(resolved_edits) = edits_result else {
                                    zlog::warn!(
                                        logger =>
                                        "Failed to resolve edits from LSP for buffer {:?} while handling {}",
                                        buffer_path_abs.as_path(),
                                        describe_code_action(&action),
                                    );
                                    continue 'actions;
                                };
                                edits.extend(resolved_edits);
                            }

                            if edits.is_empty() {
                                zlog::warn!(logger => "No edits resolved from LSP");
                                continue;
                            }

                            extend_formatting_transaction(
                                buffer,
                                formatting_transaction_id,
                                cx,
                                |buffer, cx| {
                                    buffer.edit(edits, None, cx);
                                },
                            )?;
                        }

                        if let Some(command) = action.lsp_action.command() {
                            zlog::warn!(
                                logger =>
                                "Executing code action command '{}'. This may cause formatting to abort unnecessarily as well as splitting formatting into two entries in the undo history",
                                &command.command,
                            );

                            // bail early if command is invalid
                            let server_capabilities = server.capabilities();
                            let available_commands = server_capabilities
                                .execute_command_provider
                                .as_ref()
                                .map(|options| options.commands.as_slice())
                                .unwrap_or_default();
                            if !available_commands.contains(&command.command) {
                                zlog::warn!(
                                    logger =>
                                    "Cannot execute a command {} not listed in the language server capabilities of server {}",
                                    command.command,
                                    server.name(),
                                );
                                continue;
                            }

                            // noop so we just ensure buffer hasn't been edited since resolving code actions
                            extend_formatting_transaction(
                                buffer,
                                formatting_transaction_id,
                                cx,
                                |_, _| {},
                            )?;
                            zlog::info!(logger => "Executing command {}", &command.command);

                            lsp_store.update(cx, |this, _| {
                                this.as_local_mut()
                                    .unwrap()
                                    .last_workspace_edits_by_language_server
                                    .remove(&server.server_id());
                            })?;

                            let execute_command_result = server
                                .request::<lsp::request::ExecuteCommand>(
                                    lsp::ExecuteCommandParams {
                                        command: command.command.clone(),
                                        arguments: command.arguments.clone().unwrap_or_default(),
                                        ..Default::default()
                                    },
                                )
                                .await
                                .into_response();

                            if execute_command_result.is_err() {
                                zlog::error!(
                                    logger =>
                                    "Failed to execute command '{}' as part of {}",
                                    &command.command,
                                    describe_code_action(&action),
                                );
                                continue 'actions;
                            }

                            let mut project_transaction_command =
                                lsp_store.update(cx, |this, _| {
                                    this.as_local_mut()
                                        .unwrap()
                                        .last_workspace_edits_by_language_server
                                        .remove(&server.server_id())
                                        .unwrap_or_default()
                                })?;

                            if let Some(transaction) =
                                project_transaction_command.0.remove(&buffer.handle)
                            {
                                zlog::trace!(
                                    logger =>
                                    "Successfully captured {} edits that resulted from command {}",
                                    transaction.edit_ids.len(),
                                    &command.command,
                                );
                                let transaction_id_project_transaction = transaction.id;
                                buffer.handle.update(cx, |buffer, _| {
                                    // it may have been removed from history if push_to_history was
                                    // false in deserialize_workspace_edit. If so push it so we
                                    // can merge it with the format transaction
                                    // and pop the combined transaction off the history stack
                                    // later if push_to_history is false
                                    if buffer.get_transaction(transaction.id).is_none() {
                                        buffer.push_transaction(transaction, Instant::now());
                                    }
                                    buffer.merge_transactions(
                                        transaction_id_project_transaction,
                                        formatting_transaction_id,
                                    );
                                })?;
                            }

                            if !project_transaction_command.0.is_empty() {
                                let extra_buffers = project_transaction_command
                                    .0
                                    .keys()
                                    .filter_map(|buffer_handle| {
                                        buffer_handle
                                            .read_with(cx, |b, cx| b.project_path(cx))
                                            .ok()
                                            .flatten()
                                    })
                                    .map(|p| p.path.to_sanitized_string())
                                    .join(", ");
                                zlog::warn!(
                                    logger =>
                                    "Unexpected edits to buffers other than the buffer actively being formatted due to command {}. Impacted buffers: [{}].",
                                    &command.command,
                                    extra_buffers,
                                );
                                // NOTE: if this case is hit, the proper thing to do is to for each buffer, merge the extra transaction
                                // into the existing transaction in project_transaction if there is one, and if there isn't one in project_transaction,
                                // add it so it's included, and merge it into the format transaction when its created later
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn format_ranges_via_lsp(
        this: &WeakEntity<LspStore>,
        buffer_handle: &Entity<Buffer>,
        ranges: &[Range<Anchor>],
        abs_path: &Path,
        language_server: &Arc<LanguageServer>,
        settings: &LanguageSettings,
        cx: &mut AsyncApp,
    ) -> Result<Vec<(Range<Anchor>, Arc<str>)>> {
        let capabilities = &language_server.capabilities();
        let range_formatting_provider = capabilities.document_range_formatting_provider.as_ref();
        if range_formatting_provider.map_or(false, |provider| provider == &OneOf::Left(false)) {
            anyhow::bail!(
                "{} language server does not support range formatting",
                language_server.name()
            );
        }

        let uri = file_path_to_lsp_url(abs_path)?;
        let text_document = lsp::TextDocumentIdentifier::new(uri);

        let lsp_edits = {
            let mut lsp_ranges = Vec::new();
            this.update(cx, |_this, cx| {
                // TODO(#22930): In the case of formatting multibuffer selections, this buffer may
                // not have been sent to the language server. This seems like a fairly systemic
                // issue, though, the resolution probably is not specific to formatting.
                //
                // TODO: Instead of using current snapshot, should use the latest snapshot sent to
                // LSP.
                let snapshot = buffer_handle.read(cx).snapshot();
                for range in ranges {
                    lsp_ranges.push(range_to_lsp(range.to_point_utf16(&snapshot))?);
                }
                anyhow::Ok(())
            })??;

            let mut edits = None;
            for range in lsp_ranges {
                if let Some(mut edit) = language_server
                    .request::<lsp::request::RangeFormatting>(lsp::DocumentRangeFormattingParams {
                        text_document: text_document.clone(),
                        range,
                        options: lsp_command::lsp_formatting_options(settings),
                        work_done_progress_params: Default::default(),
                    })
                    .await
                    .into_response()?
                {
                    edits.get_or_insert_with(Vec::new).append(&mut edit);
                }
            }
            edits
        };

        if let Some(lsp_edits) = lsp_edits {
            this.update(cx, |this, cx| {
                this.as_local_mut().unwrap().edits_from_lsp(
                    &buffer_handle,
                    lsp_edits,
                    language_server.server_id(),
                    None,
                    cx,
                )
            })?
            .await
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    async fn format_via_lsp(
        this: &WeakEntity<LspStore>,
        buffer: &Entity<Buffer>,
        abs_path: &Path,
        language_server: &Arc<LanguageServer>,
        settings: &LanguageSettings,
        cx: &mut AsyncApp,
    ) -> Result<Vec<(Range<Anchor>, Arc<str>)>> {
        let logger = zlog::scoped!("lsp_format");
        zlog::info!(logger => "Formatting via LSP");

        let uri = file_path_to_lsp_url(abs_path)?;
        let text_document = lsp::TextDocumentIdentifier::new(uri);
        let capabilities = &language_server.capabilities();

        let formatting_provider = capabilities.document_formatting_provider.as_ref();
        let range_formatting_provider = capabilities.document_range_formatting_provider.as_ref();

        let lsp_edits = if matches!(formatting_provider, Some(p) if *p != OneOf::Left(false)) {
            let _timer = zlog::time!(logger => "format-full");
            language_server
                .request::<lsp::request::Formatting>(lsp::DocumentFormattingParams {
                    text_document,
                    options: lsp_command::lsp_formatting_options(settings),
                    work_done_progress_params: Default::default(),
                })
                .await
                .into_response()?
        } else if matches!(range_formatting_provider, Some(p) if *p != OneOf::Left(false)) {
            let _timer = zlog::time!(logger => "format-range");
            let buffer_start = lsp::Position::new(0, 0);
            let buffer_end = buffer.read_with(cx, |b, _| point_to_lsp(b.max_point_utf16()))?;
            language_server
                .request::<lsp::request::RangeFormatting>(lsp::DocumentRangeFormattingParams {
                    text_document: text_document.clone(),
                    range: lsp::Range::new(buffer_start, buffer_end),
                    options: lsp_command::lsp_formatting_options(settings),
                    work_done_progress_params: Default::default(),
                })
                .await
                .into_response()?
        } else {
            None
        };

        if let Some(lsp_edits) = lsp_edits {
            this.update(cx, |this, cx| {
                this.as_local_mut().unwrap().edits_from_lsp(
                    buffer,
                    lsp_edits,
                    language_server.server_id(),
                    None,
                    cx,
                )
            })?
            .await
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    async fn format_via_external_command(
        buffer: &FormattableBuffer,
        command: &str,
        arguments: Option<&[String]>,
        cx: &mut AsyncApp,
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

        let mut child = util::command::new_smol_command(command);

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

        let stdin = child.stdin.as_mut().context("failed to acquire stdin")?;
        let text = buffer
            .handle
            .read_with(cx, |buffer, _| buffer.as_rope().clone())?;
        for chunk in text.chunks() {
            stdin.write_all(chunk.as_bytes()).await?;
        }
        stdin.flush().await?;

        let output = child.output().await?;
        anyhow::ensure!(
            output.status.success(),
            "command failed with exit code {:?}:\nstdout: {}\nstderr: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );

        let stdout = String::from_utf8(output.stdout)?;
        Ok(Some(
            buffer
                .handle
                .update(cx, |buffer, cx| buffer.diff(stdout, cx))?
                .await,
        ))
    }

    async fn try_resolve_code_action(
        lang_server: &LanguageServer,
        action: &mut CodeAction,
    ) -> anyhow::Result<()> {
        match &mut action.lsp_action {
            LspAction::Action(lsp_action) => {
                if !action.resolved
                    && GetCodeActions::can_resolve_actions(&lang_server.capabilities())
                    && lsp_action.data.is_some()
                    && (lsp_action.command.is_none() || lsp_action.edit.is_none())
                {
                    *lsp_action = Box::new(
                        lang_server
                            .request::<lsp::request::CodeActionResolveRequest>(*lsp_action.clone())
                            .await
                            .into_response()?,
                    );
                }
            }
            LspAction::CodeLens(lens) => {
                if !action.resolved && GetCodeLens::can_resolve_lens(&lang_server.capabilities()) {
                    *lens = lang_server
                        .request::<lsp::request::CodeLensResolve>(lens.clone())
                        .await
                        .into_response()?;
                }
            }
            LspAction::Command(_) => {}
        }

        action.resolved = true;
        anyhow::Ok(())
    }

    fn initialize_buffer(&mut self, buffer_handle: &Entity<Buffer>, cx: &mut Context<LspStore>) {
        let buffer = buffer_handle.read(cx);

        let file = buffer.file().cloned();
        let Some(file) = File::from_dyn(file.as_ref()) else {
            return;
        };
        if !file.is_local() {
            return;
        }

        let worktree_id = file.worktree_id(cx);
        let language = buffer.language().cloned();

        if let Some(diagnostics) = self.diagnostics.get(&worktree_id) {
            for (server_id, diagnostics) in
                diagnostics.get(file.path()).cloned().unwrap_or_default()
            {
                self.update_buffer_diagnostics(
                    buffer_handle,
                    server_id,
                    None,
                    None,
                    diagnostics,
                    Vec::new(),
                    cx,
                )
                .log_err();
            }
        }
        let Some(language) = language else {
            return;
        };
        for adapter in self.languages.lsp_adapters(&language.name()) {
            let servers = self
                .language_server_ids
                .get(&(worktree_id, adapter.name.clone()));
            if let Some(server_ids) = servers {
                for server_id in server_ids {
                    let server = self
                        .language_servers
                        .get(server_id)
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

                    buffer_handle.update(cx, |buffer, cx| {
                        buffer.set_completion_triggers(
                            server.server_id(),
                            server
                                .capabilities()
                                .completion_provider
                                .as_ref()
                                .and_then(|provider| {
                                    provider
                                        .trigger_characters
                                        .as_ref()
                                        .map(|characters| characters.iter().cloned().collect())
                                })
                                .unwrap_or_default(),
                            cx,
                        );
                    });
                }
            }
        }
    }

    pub(crate) fn reset_buffer(&mut self, buffer: &Entity<Buffer>, old_file: &File, cx: &mut App) {
        buffer.update(cx, |buffer, cx| {
            let Some(language) = buffer.language() else {
                return;
            };
            let path = ProjectPath {
                worktree_id: old_file.worktree_id(cx),
                path: old_file.path.clone(),
            };
            for server_id in self.language_server_ids_for_project_path(path, language, cx) {
                buffer.update_diagnostics(server_id, DiagnosticSet::new([], buffer), cx);
                buffer.set_completion_triggers(server_id, Default::default(), cx);
            }
        });
    }

    fn update_buffer_diagnostics(
        &mut self,
        buffer: &Entity<Buffer>,
        server_id: LanguageServerId,
        result_id: Option<String>,
        version: Option<i32>,
        new_diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        reused_diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        cx: &mut Context<LspStore>,
    ) -> Result<()> {
        fn compare_diagnostics(a: &Diagnostic, b: &Diagnostic) -> Ordering {
            Ordering::Equal
                .then_with(|| b.is_primary.cmp(&a.is_primary))
                .then_with(|| a.is_disk_based.cmp(&b.is_disk_based))
                .then_with(|| a.severity.cmp(&b.severity))
                .then_with(|| a.message.cmp(&b.message))
        }

        let mut diagnostics = Vec::with_capacity(new_diagnostics.len() + reused_diagnostics.len());
        diagnostics.extend(new_diagnostics.into_iter().map(|d| (true, d)));
        diagnostics.extend(reused_diagnostics.into_iter().map(|d| (false, d)));

        diagnostics.sort_unstable_by(|(_, a), (_, b)| {
            Ordering::Equal
                .then_with(|| a.range.start.cmp(&b.range.start))
                .then_with(|| b.range.end.cmp(&a.range.end))
                .then_with(|| compare_diagnostics(&a.diagnostic, &b.diagnostic))
        });

        let snapshot = self.buffer_snapshot_for_lsp_version(buffer, server_id, version, cx)?;

        let edits_since_save = std::cell::LazyCell::new(|| {
            let saved_version = buffer.read(cx).saved_version();
            Patch::new(snapshot.edits_since::<PointUtf16>(saved_version).collect())
        });

        let mut sanitized_diagnostics = Vec::with_capacity(diagnostics.len());

        for (new_diagnostic, entry) in diagnostics {
            let start;
            let end;
            if new_diagnostic && entry.diagnostic.is_disk_based {
                // Some diagnostics are based on files on disk instead of buffers'
                // current contents. Adjust these diagnostics' ranges to reflect
                // any unsaved edits.
                // Do not alter the reused ones though, as their coordinates were stored as anchors
                // and were properly adjusted on reuse.
                start = Unclipped((*edits_since_save).old_to_new(entry.range.start.0));
                end = Unclipped((*edits_since_save).old_to_new(entry.range.end.0));
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
            if let Some(abs_path) = File::from_dyn(buffer.file()).map(|f| f.abs_path(cx)) {
                self.buffer_pull_diagnostics_result_ids
                    .entry(server_id)
                    .or_default()
                    .insert(abs_path, result_id);
            }

            buffer.update_diagnostics(server_id, set, cx)
        });

        Ok(())
    }

    fn register_buffer_with_language_servers(
        &mut self,
        buffer_handle: &Entity<Buffer>,
        only_register_servers: HashSet<LanguageServerSelector>,
        cx: &mut Context<LspStore>,
    ) {
        let buffer = buffer_handle.read(cx);
        let buffer_id = buffer.remote_id();

        let Some(file) = File::from_dyn(buffer.file()) else {
            return;
        };
        if !file.is_local() {
            return;
        }

        let abs_path = file.abs_path(cx);
        let Some(uri) = file_path_to_lsp_url(&abs_path).log_err() else {
            return;
        };
        let initial_snapshot = buffer.text_snapshot();
        let worktree_id = file.worktree_id(cx);

        let Some(language) = buffer.language().cloned() else {
            return;
        };
        let path: Arc<Path> = file
            .path()
            .parent()
            .map(Arc::from)
            .unwrap_or_else(|| file.path().clone());
        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(worktree_id, cx)
        else {
            return;
        };
        let language_name = language.name();
        let (reused, delegate, servers) = self
            .lsp_tree
            .update(cx, |lsp_tree, cx| {
                self.reuse_existing_language_server(lsp_tree, &worktree, &language_name, cx)
            })
            .map(|(delegate, servers)| (true, delegate, servers))
            .unwrap_or_else(|| {
                let lsp_delegate = LocalLspAdapterDelegate::from_local_lsp(self, &worktree, cx);
                let delegate = Arc::new(ManifestQueryDelegate::new(worktree.read(cx).snapshot()));
                let servers = self
                    .lsp_tree
                    .clone()
                    .update(cx, |language_server_tree, cx| {
                        language_server_tree
                            .get(
                                ProjectPath { worktree_id, path },
                                AdapterQuery::Language(&language.name()),
                                delegate.clone(),
                                cx,
                            )
                            .collect::<Vec<_>>()
                    });
                (false, lsp_delegate, servers)
            });
        let servers_and_adapters = servers
            .into_iter()
            .filter_map(|server_node| {
                if reused && server_node.server_id().is_none() {
                    return None;
                }
                if !only_register_servers.is_empty() {
                    if let Some(server_id) = server_node.server_id() {
                        if !only_register_servers.contains(&LanguageServerSelector::Id(server_id)) {
                            return None;
                        }
                    }
                    if let Some(name) = server_node.name() {
                        if !only_register_servers.contains(&LanguageServerSelector::Name(name)) {
                            return None;
                        }
                    }
                }

                let server_id = server_node.server_id_or_init(
                    |LaunchDisposition {
                         server_name,
                         path,
                         settings,
                     }| {
                        let server_id =
                           {
                               let uri = Url::from_file_path(
                                   worktree.read(cx).abs_path().join(&path.path),
                               );
                               let key = (worktree_id, server_name.clone());
                               if !self.language_server_ids.contains_key(&key) {
                                   let language_name = language.name();
                                   let adapter = self.languages
                                       .lsp_adapters(&language_name)
                                       .into_iter()
                                       .find(|adapter| &adapter.name() == server_name)
                                       .expect("To find LSP adapter");
                                   self.start_language_server(
                                       &worktree,
                                       delegate.clone(),
                                       adapter,
                                       settings,
                                       cx,
                                   );
                               }
                               if let Some(server_ids) = self
                                   .language_server_ids
                                   .get(&key)
                               {
                                   debug_assert_eq!(server_ids.len(), 1);
                                   let server_id = server_ids.iter().cloned().next().unwrap();
                                   if let Some(state) = self.language_servers.get(&server_id) {
                                       if let Ok(uri) = uri {
                                           state.add_workspace_folder(uri);
                                       };
                                   }
                                   server_id
                               } else {
                                   unreachable!("Language server ID should be available, as it's registered on demand")
                               }

                        };
                        server_id
                    },
                )?;
                let server_state = self.language_servers.get(&server_id)?;
                if let LanguageServerState::Running { server, adapter, .. } = server_state {
                    Some((server.clone(), adapter.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (server, adapter) in servers_and_adapters {
            buffer_handle.update(cx, |buffer, cx| {
                buffer.set_completion_triggers(
                    server.server_id(),
                    server
                        .capabilities()
                        .completion_provider
                        .as_ref()
                        .and_then(|provider| {
                            provider
                                .trigger_characters
                                .as_ref()
                                .map(|characters| characters.iter().cloned().collect())
                        })
                        .unwrap_or_default(),
                    cx,
                );
            });

            let snapshot = LspBufferSnapshot {
                version: 0,
                snapshot: initial_snapshot.clone(),
            };

            let mut registered = false;
            self.buffer_snapshots
                .entry(buffer_id)
                .or_default()
                .entry(server.server_id())
                .or_insert_with(|| {
                    registered = true;
                    server.register_buffer(
                        uri.clone(),
                        adapter.language_id(&language.name()),
                        0,
                        initial_snapshot.text(),
                    );

                    vec![snapshot]
                });

            self.buffers_opened_in_servers
                .entry(buffer_id)
                .or_default()
                .insert(server.server_id());
            if registered {
                cx.emit(LspStoreEvent::LanguageServerUpdate {
                    language_server_id: server.server_id(),
                    name: None,
                    message: proto::update_language_server::Variant::RegisteredForBuffer(
                        proto::RegisteredForBuffer {
                            buffer_abs_path: abs_path.to_string_lossy().to_string(),
                            buffer_id: buffer_id.to_proto(),
                        },
                    ),
                });
            }
        }
    }

    fn reuse_existing_language_server(
        &self,
        server_tree: &mut LanguageServerTree,
        worktree: &Entity<Worktree>,
        language_name: &LanguageName,
        cx: &mut App,
    ) -> Option<(Arc<LocalLspAdapterDelegate>, Vec<LanguageServerTreeNode>)> {
        if worktree.read(cx).is_visible() {
            return None;
        }

        let worktree_store = self.worktree_store.read(cx);
        let servers = server_tree
            .instances
            .iter()
            .filter(|(worktree_id, _)| {
                worktree_store
                    .worktree_for_id(**worktree_id, cx)
                    .is_some_and(|worktree| worktree.read(cx).is_visible())
            })
            .flat_map(|(worktree_id, servers)| {
                servers
                    .roots
                    .iter()
                    .flat_map(|(_, language_servers)| language_servers)
                    .map(move |(_, (server_node, server_languages))| {
                        (worktree_id, server_node, server_languages)
                    })
                    .filter(|(_, _, server_languages)| server_languages.contains(language_name))
                    .map(|(worktree_id, server_node, _)| {
                        (
                            *worktree_id,
                            LanguageServerTreeNode::from(Arc::downgrade(server_node)),
                        )
                    })
            })
            .fold(HashMap::default(), |mut acc, (worktree_id, server_node)| {
                acc.entry(worktree_id)
                    .or_insert_with(Vec::new)
                    .push(server_node);
                acc
            })
            .into_values()
            .max_by_key(|servers| servers.len())?;

        for server_node in &servers {
            server_tree.register_reused(
                worktree.read(cx).id(),
                language_name.clone(),
                server_node.clone(),
            );
        }

        let delegate = LocalLspAdapterDelegate::from_local_lsp(self, worktree, cx);
        Some((delegate, servers))
    }

    pub(crate) fn unregister_old_buffer_from_language_servers(
        &mut self,
        buffer: &Entity<Buffer>,
        old_file: &File,
        cx: &mut App,
    ) {
        let old_path = match old_file.as_local() {
            Some(local) => local.abs_path(cx),
            None => return,
        };

        let Ok(file_url) = lsp::Url::from_file_path(old_path.as_path()) else {
            debug_panic!(
                "`{}` is not parseable as an URI",
                old_path.to_string_lossy()
            );
            return;
        };
        self.unregister_buffer_from_language_servers(buffer, &file_url, cx);
    }

    pub(crate) fn unregister_buffer_from_language_servers(
        &mut self,
        buffer: &Entity<Buffer>,
        file_url: &lsp::Url,
        cx: &mut App,
    ) {
        buffer.update(cx, |buffer, cx| {
            let _ = self.buffer_snapshots.remove(&buffer.remote_id());

            for (_, language_server) in self.language_servers_for_buffer(buffer, cx) {
                language_server.unregister_buffer(file_url.clone());
            }
        });
    }

    fn buffer_snapshot_for_lsp_version(
        &mut self,
        buffer: &Entity<Buffer>,
        server_id: LanguageServerId,
        version: Option<i32>,
        cx: &App,
    ) -> Result<TextBufferSnapshot> {
        const OLD_VERSIONS_TO_RETAIN: i32 = 10;

        if let Some(version) = version {
            let buffer_id = buffer.read(cx).remote_id();
            let snapshots = if let Some(snapshots) = self
                .buffer_snapshots
                .get_mut(&buffer_id)
                .and_then(|m| m.get_mut(&server_id))
            {
                snapshots
            } else if version == 0 {
                // Some language servers report version 0 even if the buffer hasn't been opened yet.
                // We detect this case and treat it as if the version was `None`.
                return Ok(buffer.read(cx).text_snapshot());
            } else {
                anyhow::bail!("no snapshots found for buffer {buffer_id} and server {server_id}");
            };

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

    async fn get_server_code_actions_from_action_kinds(
        lsp_store: &WeakEntity<LspStore>,
        language_server_id: LanguageServerId,
        code_action_kinds: Vec<lsp::CodeActionKind>,
        buffer: &Entity<Buffer>,
        cx: &mut AsyncApp,
    ) -> Result<Vec<CodeAction>> {
        let actions = lsp_store
            .update(cx, move |this, cx| {
                let request = GetCodeActions {
                    range: text::Anchor::MIN..text::Anchor::MAX,
                    kinds: Some(code_action_kinds),
                };
                let server = LanguageServerToQuery::Other(language_server_id);
                this.request_lsp(buffer.clone(), server, request, cx)
            })?
            .await?;
        return Ok(actions);
    }

    pub async fn execute_code_actions_on_server(
        lsp_store: &WeakEntity<LspStore>,
        language_server: &Arc<LanguageServer>,
        lsp_adapter: &Arc<CachedLspAdapter>,
        actions: Vec<CodeAction>,
        push_to_history: bool,
        project_transaction: &mut ProjectTransaction,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        for mut action in actions {
            Self::try_resolve_code_action(language_server, &mut action)
                .await
                .context("resolving a formatting code action")?;

            if let Some(edit) = action.lsp_action.edit() {
                if edit.changes.is_none() && edit.document_changes.is_none() {
                    continue;
                }

                let new = Self::deserialize_workspace_edit(
                    lsp_store.upgrade().context("project dropped")?,
                    edit.clone(),
                    push_to_history,
                    lsp_adapter.clone(),
                    language_server.clone(),
                    cx,
                )
                .await?;
                project_transaction.0.extend(new.0);
            }

            if let Some(command) = action.lsp_action.command() {
                let server_capabilities = language_server.capabilities();
                let available_commands = server_capabilities
                    .execute_command_provider
                    .as_ref()
                    .map(|options| options.commands.as_slice())
                    .unwrap_or_default();
                if available_commands.contains(&command.command) {
                    lsp_store.update(cx, |lsp_store, _| {
                        if let LspStoreMode::Local(mode) = &mut lsp_store.mode {
                            mode.last_workspace_edits_by_language_server
                                .remove(&language_server.server_id());
                        }
                    })?;

                    language_server
                        .request::<lsp::request::ExecuteCommand>(lsp::ExecuteCommandParams {
                            command: command.command.clone(),
                            arguments: command.arguments.clone().unwrap_or_default(),
                            ..Default::default()
                        })
                        .await
                        .into_response()
                        .context("execute command")?;

                    lsp_store.update(cx, |this, _| {
                        if let LspStoreMode::Local(mode) = &mut this.mode {
                            project_transaction.0.extend(
                                mode.last_workspace_edits_by_language_server
                                    .remove(&language_server.server_id())
                                    .unwrap_or_default()
                                    .0,
                            )
                        }
                    })?;
                } else {
                    log::warn!(
                        "Cannot execute a command {} not listed in the language server capabilities",
                        command.command
                    )
                }
            }
        }
        return Ok(());
    }

    pub async fn deserialize_text_edits(
        this: Entity<LspStore>,
        buffer_to_edit: Entity<Buffer>,
        edits: Vec<lsp::TextEdit>,
        push_to_history: bool,
        _: Arc<CachedLspAdapter>,
        language_server: Arc<LanguageServer>,
        cx: &mut AsyncApp,
    ) -> Result<Option<Transaction>> {
        let edits = this
            .update(cx, |this, cx| {
                this.as_local_mut().unwrap().edits_from_lsp(
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

    #[allow(clippy::type_complexity)]
    pub(crate) fn edits_from_lsp(
        &mut self,
        buffer: &Entity<Buffer>,
        lsp_edits: impl 'static + Send + IntoIterator<Item = lsp::TextEdit>,
        server_id: LanguageServerId,
        version: Option<i32>,
        cx: &mut Context<LspStore>,
    ) -> Task<Result<Vec<(Range<Anchor>, Arc<str>)>>> {
        let snapshot = self.buffer_snapshot_for_lsp_version(buffer, server_id, version, cx);
        cx.background_spawn(async move {
            let snapshot = snapshot?;
            let mut lsp_edits = lsp_edits
                .into_iter()
                .map(|edit| (range_from_lsp(edit.range), edit.new_text))
                .collect::<Vec<_>>();

            lsp_edits.sort_by_key(|(range, _)| (range.start, range.end));

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
                    let offset = range.start.to_offset(&snapshot);
                    let old_text = snapshot.text_for_range(range).collect::<String>();
                    let range_edits = language::text_diff(old_text.as_str(), &new_text);
                    edits.extend(range_edits.into_iter().map(|(range, replacement)| {
                        (
                            snapshot.anchor_after(offset + range.start)
                                ..snapshot.anchor_before(offset + range.end),
                            replacement,
                        )
                    }));
                } else if range.end == range.start {
                    let anchor = snapshot.anchor_after(range.start);
                    edits.push((anchor..anchor, new_text.into()));
                } else {
                    let edit_start = snapshot.anchor_after(range.start);
                    let edit_end = snapshot.anchor_before(range.end);
                    edits.push((edit_start..edit_end, new_text.into()));
                }
            }

            Ok(edits)
        })
    }

    pub(crate) async fn deserialize_workspace_edit(
        this: Entity<LspStore>,
        edit: lsp::WorkspaceEdit,
        push_to_history: bool,
        lsp_adapter: Arc<CachedLspAdapter>,
        language_server: Arc<LanguageServer>,
        cx: &mut AsyncApp,
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
                        .map_err(|()| anyhow!("can't convert URI to path"))?;

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
                        .map_err(|()| anyhow!("can't convert URI to path"))?;
                    let target_abs_path = op
                        .new_uri
                        .to_file_path()
                        .map_err(|()| anyhow!("can't convert URI to path"))?;
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
                        .map_err(|()| anyhow!("can't convert URI to path"))?;
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
                            let local = this.as_local_mut().unwrap();

                            let (mut edits, mut snippet_edits) = (vec![], vec![]);
                            for edit in op.edits {
                                match edit {
                                    Edit::Plain(edit) => {
                                        if !edits.contains(&edit) {
                                            edits.push(edit)
                                        }
                                    }
                                    Edit::Annotated(edit) => {
                                        if !edits.contains(&edit.text_edit) {
                                            edits.push(edit.text_edit)
                                        }
                                    }
                                    Edit::Snippet(edit) => {
                                        let Ok(snippet) = Snippet::parse(&edit.snippet.value)
                                        else {
                                            continue;
                                        };

                                        if is_active_entry {
                                            snippet_edits.push((edit.range, snippet));
                                        } else {
                                            // Since this buffer is not focused, apply a normal edit.
                                            let new_edit = TextEdit {
                                                range: edit.range,
                                                new_text: snippet.text,
                                            };
                                            if !edits.contains(&new_edit) {
                                                edits.push(new_edit);
                                            }
                                        }
                                    }
                                }
                            }
                            if !snippet_edits.is_empty() {
                                let buffer_id = buffer_to_edit.read(cx).remote_id();
                                let version = if let Some(buffer_version) = op.text_document.version
                                {
                                    local
                                        .buffer_snapshot_for_lsp_version(
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

                            local.edits_from_lsp(
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

                        let transaction = buffer.end_transaction(cx).and_then(|transaction_id| {
                            if push_to_history {
                                buffer.finalize_last_transaction();
                                buffer.get_transaction(transaction_id).cloned()
                            } else {
                                buffer.forget_transaction(transaction_id)
                            }
                        });

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

    async fn on_lsp_workspace_edit(
        this: WeakEntity<LspStore>,
        params: lsp::ApplyWorkspaceEditParams,
        server_id: LanguageServerId,
        adapter: Arc<CachedLspAdapter>,
        cx: &mut AsyncApp,
    ) -> Result<lsp::ApplyWorkspaceEditResponse> {
        let this = this.upgrade().context("project project closed")?;
        let language_server = this
            .read_with(cx, |this, _| this.language_server_for_id(server_id))?
            .context("language server not found")?;
        let transaction = Self::deserialize_workspace_edit(
            this.clone(),
            params.edit,
            true,
            adapter.clone(),
            language_server.clone(),
            cx,
        )
        .await
        .log_err();
        this.update(cx, |this, _| {
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

    fn remove_worktree(
        &mut self,
        id_to_remove: WorktreeId,
        cx: &mut Context<LspStore>,
    ) -> Vec<LanguageServerId> {
        self.diagnostics.remove(&id_to_remove);
        self.prettier_store.update(cx, |prettier_store, cx| {
            prettier_store.remove_worktree(id_to_remove, cx);
        });

        let mut servers_to_remove = BTreeMap::default();
        let mut servers_to_preserve = HashSet::default();
        for ((path, server_name), ref server_ids) in &self.language_server_ids {
            if *path == id_to_remove {
                servers_to_remove.extend(server_ids.iter().map(|id| (*id, server_name.clone())));
            } else {
                servers_to_preserve.extend(server_ids.iter().cloned());
            }
        }
        servers_to_remove.retain(|server_id, _| !servers_to_preserve.contains(server_id));

        for (server_id_to_remove, _) in &servers_to_remove {
            self.language_server_ids
                .values_mut()
                .for_each(|server_ids| {
                    server_ids.remove(server_id_to_remove);
                });
            self.language_server_watched_paths
                .remove(server_id_to_remove);
            self.language_server_paths_watched_for_rename
                .remove(server_id_to_remove);
            self.last_workspace_edits_by_language_server
                .remove(server_id_to_remove);
            self.language_servers.remove(server_id_to_remove);
            self.buffer_pull_diagnostics_result_ids
                .remove(server_id_to_remove);
            for buffer_servers in self.buffers_opened_in_servers.values_mut() {
                buffer_servers.remove(server_id_to_remove);
            }
            cx.emit(LspStoreEvent::LanguageServerRemoved(*server_id_to_remove));
        }
        servers_to_remove.into_keys().collect()
    }

    fn rebuild_watched_paths_inner<'a>(
        &'a self,
        language_server_id: LanguageServerId,
        watchers: impl Iterator<Item = &'a FileSystemWatcher>,
        cx: &mut Context<LspStore>,
    ) -> LanguageServerWatchedPathsBuilder {
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

        let mut worktree_globs = HashMap::default();
        let mut abs_globs = HashMap::default();
        log::trace!(
            "Processing new watcher paths for language server with id {}",
            language_server_id
        );

        for watcher in watchers {
            if let Some((worktree, literal_prefix, pattern)) =
                self.worktree_and_path_for_file_watcher(&worktrees, &watcher, cx)
            {
                worktree.update(cx, |worktree, _| {
                    if let Some((tree, glob)) =
                        worktree.as_local_mut().zip(Glob::new(&pattern).log_err())
                    {
                        tree.add_path_prefix_to_scan(literal_prefix.into());
                        worktree_globs
                            .entry(tree.id())
                            .or_insert_with(GlobSetBuilder::new)
                            .add(glob);
                    }
                });
            } else {
                let (path, pattern) = match &watcher.glob_pattern {
                    lsp::GlobPattern::String(s) => {
                        let watcher_path = SanitizedPath::from(s);
                        let path = glob_literal_prefix(watcher_path.as_path());
                        let pattern = watcher_path
                            .as_path()
                            .strip_prefix(&path)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|e| {
                                debug_panic!(
                                    "Failed to strip prefix for string pattern: {}, with prefix: {}, with error: {}",
                                    s,
                                    path.display(),
                                    e
                                );
                                watcher_path.as_path().to_string_lossy().to_string()
                            });
                        (path, pattern)
                    }
                    lsp::GlobPattern::Relative(rp) => {
                        let Ok(mut base_uri) = match &rp.base_uri {
                            lsp::OneOf::Left(workspace_folder) => &workspace_folder.uri,
                            lsp::OneOf::Right(base_uri) => base_uri,
                        }
                        .to_file_path() else {
                            continue;
                        };

                        let path = glob_literal_prefix(Path::new(&rp.pattern));
                        let pattern = Path::new(&rp.pattern)
                            .strip_prefix(&path)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|e| {
                                debug_panic!(
                                    "Failed to strip prefix for relative pattern: {}, with prefix: {}, with error: {}",
                                    rp.pattern,
                                    path.display(),
                                    e
                                );
                                rp.pattern.clone()
                            });
                        base_uri.push(path);
                        (base_uri, pattern)
                    }
                };

                if let Some(glob) = Glob::new(&pattern).log_err() {
                    if !path
                        .components()
                        .any(|c| matches!(c, path::Component::Normal(_)))
                    {
                        // For an unrooted glob like `**/Cargo.toml`, watch it within each worktree,
                        // rather than adding a new watcher for `/`.
                        for worktree in &worktrees {
                            worktree_globs
                                .entry(worktree.read(cx).id())
                                .or_insert_with(GlobSetBuilder::new)
                                .add(glob.clone());
                        }
                    } else {
                        abs_globs
                            .entry(path.into())
                            .or_insert_with(GlobSetBuilder::new)
                            .add(glob);
                    }
                }
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
        watch_builder
    }

    fn worktree_and_path_for_file_watcher(
        &self,
        worktrees: &[Entity<Worktree>],
        watcher: &FileSystemWatcher,
        cx: &App,
    ) -> Option<(Entity<Worktree>, PathBuf, String)> {
        worktrees.iter().find_map(|worktree| {
            let tree = worktree.read(cx);
            let worktree_root_path = tree.abs_path();
            match &watcher.glob_pattern {
                lsp::GlobPattern::String(s) => {
                    let watcher_path = SanitizedPath::from(s);
                    let relative = watcher_path
                        .as_path()
                        .strip_prefix(&worktree_root_path)
                        .ok()?;
                    let literal_prefix = glob_literal_prefix(relative);
                    Some((
                        worktree.clone(),
                        literal_prefix,
                        relative.to_string_lossy().to_string(),
                    ))
                }
                lsp::GlobPattern::Relative(rp) => {
                    let base_uri = match &rp.base_uri {
                        lsp::OneOf::Left(workspace_folder) => &workspace_folder.uri,
                        lsp::OneOf::Right(base_uri) => base_uri,
                    }
                    .to_file_path()
                    .ok()?;
                    let relative = base_uri.strip_prefix(&worktree_root_path).ok()?;
                    let mut literal_prefix = relative.to_owned();
                    literal_prefix.push(glob_literal_prefix(Path::new(&rp.pattern)));
                    Some((worktree.clone(), literal_prefix, rp.pattern.clone()))
                }
            }
        })
    }

    fn rebuild_watched_paths(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut Context<LspStore>,
    ) {
        let Some(watchers) = self
            .language_server_watcher_registrations
            .get(&language_server_id)
        else {
            return;
        };

        let watch_builder =
            self.rebuild_watched_paths_inner(language_server_id, watchers.values().flatten(), cx);
        let watcher = watch_builder.build(self.fs.clone(), language_server_id, cx);
        self.language_server_watched_paths
            .insert(language_server_id, watcher);

        cx.notify();
    }

    fn on_lsp_did_change_watched_files(
        &mut self,
        language_server_id: LanguageServerId,
        registration_id: &str,
        params: DidChangeWatchedFilesRegistrationOptions,
        cx: &mut Context<LspStore>,
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
        cx: &mut Context<LspStore>,
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

    async fn initialization_options_for_adapter(
        adapter: Arc<dyn LspAdapter>,
        fs: &dyn Fs,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let Some(mut initialization_config) =
            adapter.clone().initialization_options(fs, delegate).await?
        else {
            return Ok(None);
        };

        for other_adapter in delegate.registered_lsp_adapters() {
            if other_adapter.name() == adapter.name() {
                continue;
            }
            if let Ok(Some(target_config)) = other_adapter
                .clone()
                .additional_initialization_options(adapter.name(), fs, delegate)
                .await
            {
                merge_json_value_into(target_config.clone(), &mut initialization_config);
            }
        }

        Ok(Some(initialization_config))
    }

    async fn workspace_configuration_for_adapter(
        adapter: Arc<dyn LspAdapter>,
        fs: &dyn Fs,
        delegate: &Arc<dyn LspAdapterDelegate>,
        toolchains: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncApp,
    ) -> Result<serde_json::Value> {
        let mut workspace_config = adapter
            .clone()
            .workspace_configuration(fs, delegate, toolchains.clone(), cx)
            .await?;

        for other_adapter in delegate.registered_lsp_adapters() {
            if other_adapter.name() == adapter.name() {
                continue;
            }
            if let Ok(Some(target_config)) = other_adapter
                .clone()
                .additional_workspace_configuration(
                    adapter.name(),
                    fs,
                    delegate,
                    toolchains.clone(),
                    cx,
                )
                .await
            {
                merge_json_value_into(target_config.clone(), &mut workspace_config);
            }
        }

        Ok(workspace_config)
    }
}

fn notify_server_capabilities_updated(server: &LanguageServer, cx: &mut Context<LspStore>) {
    if let Some(capabilities) = serde_json::to_string(&server.capabilities()).ok() {
        cx.emit(LspStoreEvent::LanguageServerUpdate {
            language_server_id: server.server_id(),
            name: Some(server.name()),
            message: proto::update_language_server::Variant::MetadataUpdated(
                proto::ServerMetadataUpdated {
                    capabilities: Some(capabilities),
                },
            ),
        });
    }
}

#[derive(Debug)]
pub struct FormattableBuffer {
    handle: Entity<Buffer>,
    abs_path: Option<PathBuf>,
    env: Option<HashMap<String, String>>,
    ranges: Option<Vec<Range<Anchor>>>,
}

pub struct RemoteLspStore {
    upstream_client: Option<AnyProtoClient>,
    upstream_project_id: u64,
}

pub(crate) enum LspStoreMode {
    Local(LocalLspStore),   // ssh host and collab host
    Remote(RemoteLspStore), // collab guest
}

impl LspStoreMode {
    fn is_local(&self) -> bool {
        matches!(self, LspStoreMode::Local(_))
    }
}

pub struct LspStore {
    mode: LspStoreMode,
    last_formatting_failure: Option<String>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    nonce: u128,
    buffer_store: Entity<BufferStore>,
    worktree_store: Entity<WorktreeStore>,
    toolchain_store: Option<Entity<ToolchainStore>>,
    pub languages: Arc<LanguageRegistry>,
    language_server_statuses: BTreeMap<LanguageServerId, LanguageServerStatus>,
    active_entry: Option<ProjectEntryId>,
    _maintain_workspace_config: (Task<Result<()>>, watch::Sender<()>),
    _maintain_buffer_languages: Task<()>,
    diagnostic_summaries:
        HashMap<WorktreeId, HashMap<Arc<Path>, HashMap<LanguageServerId, DiagnosticSummary>>>,
    pub(super) lsp_server_capabilities: HashMap<LanguageServerId, lsp::ServerCapabilities>,
    lsp_document_colors: HashMap<BufferId, DocumentColorData>,
    lsp_code_lens: HashMap<BufferId, CodeLensData>,
}

#[derive(Debug, Default, Clone)]
pub struct DocumentColors {
    pub colors: HashSet<DocumentColor>,
    pub cache_version: Option<usize>,
}

type DocumentColorTask = Shared<Task<std::result::Result<DocumentColors, Arc<anyhow::Error>>>>;
type CodeLensTask = Shared<Task<std::result::Result<Vec<CodeAction>, Arc<anyhow::Error>>>>;

#[derive(Debug, Default)]
struct DocumentColorData {
    colors_for_version: Global,
    colors: HashMap<LanguageServerId, HashSet<DocumentColor>>,
    cache_version: usize,
    colors_update: Option<(Global, DocumentColorTask)>,
}

#[derive(Debug, Default)]
struct CodeLensData {
    lens_for_version: Global,
    lens: HashMap<LanguageServerId, Vec<CodeAction>>,
    update: Option<(Global, CodeLensTask)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LspFetchStrategy {
    IgnoreCache,
    UseCache { known_cache_version: Option<usize> },
}

#[derive(Debug)]
pub enum LspStoreEvent {
    LanguageServerAdded(LanguageServerId, LanguageServerName, Option<WorktreeId>),
    LanguageServerRemoved(LanguageServerId),
    LanguageServerUpdate {
        language_server_id: LanguageServerId,
        name: Option<LanguageServerName>,
        message: proto::update_language_server::Variant,
    },
    LanguageServerLog(LanguageServerId, LanguageServerLogType, String),
    LanguageServerPrompt(LanguageServerPromptRequest),
    LanguageDetected {
        buffer: Entity<Buffer>,
        new_language: Option<Arc<Language>>,
    },
    Notification(String),
    RefreshInlayHints,
    RefreshCodeLens,
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
    pub name: LanguageServerName,
    pub pending_work: BTreeMap<String, LanguageServerProgress>,
    pub has_pending_diagnostic_updates: bool,
    progress_tokens: HashSet<String>,
}

#[derive(Clone, Debug)]
struct CoreSymbol {
    pub language_server_name: LanguageServerName,
    pub source_worktree_id: WorktreeId,
    pub source_language_server_id: LanguageServerId,
    pub path: ProjectPath,
    pub name: String,
    pub kind: lsp::SymbolKind,
    pub range: Range<Unclipped<PointUtf16>>,
    pub signature: [u8; 32],
}

impl LspStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_multi_lsp_query);
        client.add_entity_request_handler(Self::handle_restart_language_servers);
        client.add_entity_request_handler(Self::handle_stop_language_servers);
        client.add_entity_request_handler(Self::handle_cancel_language_server_work);
        client.add_entity_message_handler(Self::handle_start_language_server);
        client.add_entity_message_handler(Self::handle_update_language_server);
        client.add_entity_message_handler(Self::handle_language_server_log);
        client.add_entity_message_handler(Self::handle_update_diagnostic_summary);
        client.add_entity_request_handler(Self::handle_format_buffers);
        client.add_entity_request_handler(Self::handle_apply_code_action_kind);
        client.add_entity_request_handler(Self::handle_resolve_completion_documentation);
        client.add_entity_request_handler(Self::handle_apply_code_action);
        client.add_entity_request_handler(Self::handle_inlay_hints);
        client.add_entity_request_handler(Self::handle_get_project_symbols);
        client.add_entity_request_handler(Self::handle_resolve_inlay_hint);
        client.add_entity_request_handler(Self::handle_get_color_presentation);
        client.add_entity_request_handler(Self::handle_open_buffer_for_symbol);
        client.add_entity_request_handler(Self::handle_refresh_inlay_hints);
        client.add_entity_request_handler(Self::handle_refresh_code_lens);
        client.add_entity_request_handler(Self::handle_on_type_formatting);
        client.add_entity_request_handler(Self::handle_apply_additional_edits_for_completion);
        client.add_entity_request_handler(Self::handle_register_buffer_with_language_servers);
        client.add_entity_request_handler(Self::handle_rename_project_entry);
        client.add_entity_request_handler(Self::handle_pull_workspace_diagnostics);
        client.add_entity_request_handler(Self::handle_lsp_command::<GetCodeActions>);
        client.add_entity_request_handler(Self::handle_lsp_command::<GetCompletions>);
        client.add_entity_request_handler(Self::handle_lsp_command::<GetHover>);
        client.add_entity_request_handler(Self::handle_lsp_command::<GetDocumentHighlights>);
        client.add_entity_request_handler(Self::handle_lsp_command::<GetDocumentSymbols>);
        client.add_entity_request_handler(Self::handle_lsp_command::<PrepareRename>);
        client.add_entity_request_handler(Self::handle_lsp_command::<PerformRename>);
        client.add_entity_request_handler(Self::handle_lsp_command::<LinkedEditingRange>);

        client.add_entity_request_handler(Self::handle_lsp_ext_cancel_flycheck);
        client.add_entity_request_handler(Self::handle_lsp_ext_run_flycheck);
        client.add_entity_request_handler(Self::handle_lsp_ext_clear_flycheck);
        client.add_entity_request_handler(Self::handle_lsp_command::<lsp_ext_command::ExpandMacro>);
        client.add_entity_request_handler(Self::handle_lsp_command::<lsp_ext_command::OpenDocs>);
        client.add_entity_request_handler(
            Self::handle_lsp_command::<lsp_ext_command::GoToParentModule>,
        );
        client.add_entity_request_handler(
            Self::handle_lsp_command::<lsp_ext_command::GetLspRunnables>,
        );
        client.add_entity_request_handler(
            Self::handle_lsp_command::<lsp_ext_command::SwitchSourceHeader>,
        );
        client.add_entity_request_handler(Self::handle_lsp_command::<GetDocumentDiagnostics>);
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

    pub fn new_local(
        buffer_store: Entity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        prettier_store: Entity<PrettierStore>,
        toolchain_store: Entity<ToolchainStore>,
        environment: Entity<ProjectEnvironment>,
        manifest_tree: Entity<ManifestTree>,
        languages: Arc<LanguageRegistry>,
        http_client: Arc<dyn HttpClient>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
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
        if let Some(extension_events) = extension::ExtensionEvents::try_global(cx).as_ref() {
            cx.subscribe(
                extension_events,
                Self::reload_zed_json_schemas_on_extensions_changed,
            )
            .detach();
        } else {
            log::debug!("No extension events global found. Skipping JSON schema auto-reload setup");
        }
        cx.observe_global::<SettingsStore>(Self::on_settings_changed)
            .detach();
        subscribe_to_binary_statuses(&languages, cx).detach();

        let _maintain_workspace_config = {
            let (sender, receiver) = watch::channel();
            (
                Self::maintain_workspace_config(fs.clone(), receiver, cx),
                sender,
            )
        };

        Self {
            mode: LspStoreMode::Local(LocalLspStore {
                weak: cx.weak_entity(),
                worktree_store: worktree_store.clone(),
                toolchain_store: toolchain_store.clone(),
                supplementary_language_servers: Default::default(),
                languages: languages.clone(),
                language_server_ids: Default::default(),
                language_servers: Default::default(),
                last_workspace_edits_by_language_server: Default::default(),
                language_server_watched_paths: Default::default(),
                language_server_paths_watched_for_rename: Default::default(),
                language_server_watcher_registrations: Default::default(),
                buffers_being_formatted: Default::default(),
                buffer_snapshots: Default::default(),
                prettier_store,
                environment,
                http_client,
                fs,
                yarn,
                next_diagnostic_group_id: Default::default(),
                diagnostics: Default::default(),
                _subscription: cx.on_app_quit(|this, cx| {
                    this.as_local_mut()
                        .unwrap()
                        .shutdown_language_servers_on_quit(cx)
                }),
                lsp_tree: LanguageServerTree::new(manifest_tree, languages.clone(), cx),
                registered_buffers: HashMap::default(),
                buffers_opened_in_servers: HashMap::default(),
                buffer_pull_diagnostics_result_ids: HashMap::default(),
            }),
            last_formatting_failure: None,
            downstream_client: None,
            buffer_store,
            worktree_store,
            toolchain_store: Some(toolchain_store),
            languages: languages.clone(),
            language_server_statuses: Default::default(),
            nonce: StdRng::from_entropy().r#gen(),
            diagnostic_summaries: HashMap::default(),
            lsp_server_capabilities: HashMap::default(),
            lsp_document_colors: HashMap::default(),
            lsp_code_lens: HashMap::default(),
            active_entry: None,
            _maintain_workspace_config,
            _maintain_buffer_languages: Self::maintain_buffer_languages(languages, cx),
        }
    }

    fn send_lsp_proto_request<R: LspCommand>(
        &self,
        buffer: Entity<Buffer>,
        client: AnyProtoClient,
        upstream_project_id: u64,
        request: R,
        cx: &mut Context<LspStore>,
    ) -> Task<anyhow::Result<<R as LspCommand>::Response>> {
        if !self.is_capable_for_proto_request(&buffer, &request, cx) {
            return Task::ready(Ok(R::Response::default()));
        }
        let message = request.to_proto(upstream_project_id, buffer.read(cx));
        cx.spawn(async move |this, cx| {
            let response = client.request(message).await?;
            let this = this.upgrade().context("project dropped")?;
            request
                .response_from_proto(response, this, buffer, cx.clone())
                .await
        })
    }

    pub(super) fn new_remote(
        buffer_store: Entity<BufferStore>,
        worktree_store: Entity<WorktreeStore>,
        toolchain_store: Option<Entity<ToolchainStore>>,
        languages: Arc<LanguageRegistry>,
        upstream_client: AnyProtoClient,
        project_id: u64,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&buffer_store, Self::on_buffer_store_event)
            .detach();
        cx.subscribe(&worktree_store, Self::on_worktree_store_event)
            .detach();
        subscribe_to_binary_statuses(&languages, cx).detach();
        let _maintain_workspace_config = {
            let (sender, receiver) = watch::channel();
            (Self::maintain_workspace_config(fs, receiver, cx), sender)
        };
        Self {
            mode: LspStoreMode::Remote(RemoteLspStore {
                upstream_client: Some(upstream_client),
                upstream_project_id: project_id,
            }),
            downstream_client: None,
            last_formatting_failure: None,
            buffer_store,
            worktree_store,
            languages: languages.clone(),
            language_server_statuses: Default::default(),
            nonce: StdRng::from_entropy().r#gen(),
            diagnostic_summaries: HashMap::default(),
            lsp_server_capabilities: HashMap::default(),
            lsp_document_colors: HashMap::default(),
            lsp_code_lens: HashMap::default(),
            active_entry: None,
            toolchain_store,
            _maintain_workspace_config,
            _maintain_buffer_languages: Self::maintain_buffer_languages(languages.clone(), cx),
        }
    }

    fn on_buffer_store_event(
        &mut self,
        _: Entity<BufferStore>,
        event: &BufferStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            BufferStoreEvent::BufferAdded(buffer) => {
                self.on_buffer_added(buffer, cx).log_err();
            }
            BufferStoreEvent::BufferChangedFilePath { buffer, old_file } => {
                let buffer_id = buffer.read(cx).remote_id();
                if let Some(local) = self.as_local_mut() {
                    if let Some(old_file) = File::from_dyn(old_file.as_ref()) {
                        local.reset_buffer(buffer, old_file, cx);

                        if local.registered_buffers.contains_key(&buffer_id) {
                            local.unregister_old_buffer_from_language_servers(buffer, old_file, cx);
                        }
                    }
                }

                self.detect_language_for_buffer(buffer, cx);
                if let Some(local) = self.as_local_mut() {
                    local.initialize_buffer(buffer, cx);
                    if local.registered_buffers.contains_key(&buffer_id) {
                        local.register_buffer_with_language_servers(buffer, HashSet::default(), cx);
                    }
                }
            }
            _ => {}
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
            WorktreeStoreEvent::WorktreeRemoved(_, id) => self.remove_worktree(*id, cx),
            WorktreeStoreEvent::WorktreeUpdateSent(worktree) => {
                worktree.update(cx, |worktree, _cx| self.send_diagnostic_summaries(worktree));
            }
            WorktreeStoreEvent::WorktreeReleased(..)
            | WorktreeStoreEvent::WorktreeOrderChanged
            | WorktreeStoreEvent::WorktreeUpdatedEntries(..)
            | WorktreeStoreEvent::WorktreeUpdatedGitRepositories(..)
            | WorktreeStoreEvent::WorktreeDeletedEntry(..) => {}
        }
    }

    fn on_prettier_store_event(
        &mut self,
        _: Entity<PrettierStore>,
        event: &PrettierStoreEvent,
        cx: &mut Context<Self>,
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
        _: Entity<ToolchainStore>,
        event: &ToolchainStoreEvent,
        _: &mut Context<Self>,
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

    pub fn prettier_store(&self) -> Option<Entity<PrettierStore>> {
        self.as_local().map(|local| local.prettier_store.clone())
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &language::BufferEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            language::BufferEvent::Edited => {
                self.on_buffer_edited(buffer, cx);
            }

            language::BufferEvent::Saved => {
                self.on_buffer_saved(buffer, cx);
            }

            _ => {}
        }
    }

    fn on_buffer_added(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) -> Result<()> {
        buffer
            .read(cx)
            .set_language_registry(self.languages.clone());

        cx.subscribe(buffer, |this, buffer, event, cx| {
            this.on_buffer_event(buffer, event, cx);
        })
        .detach();

        self.detect_language_for_buffer(buffer, cx);
        if let Some(local) = self.as_local_mut() {
            local.initialize_buffer(buffer, cx);
        }

        Ok(())
    }

    pub fn reload_zed_json_schemas_on_extensions_changed(
        &mut self,
        _: Entity<extension::ExtensionEvents>,
        evt: &extension::Event,
        cx: &mut Context<Self>,
    ) {
        match evt {
            extension::Event::ExtensionInstalled(_)
            | extension::Event::ExtensionUninstalled(_)
            | extension::Event::ConfigureExtensionRequested(_) => return,
            extension::Event::ExtensionsInstalledChanged => {}
        }
        if self.as_local().is_none() {
            return;
        }
        cx.spawn(async move |this, cx| {
            let weak_ref = this.clone();

            let servers = this
                .update(cx, |this, cx| {
                    let local = this.as_local()?;

                    let mut servers = Vec::new();
                    for ((worktree_id, _), server_ids) in &local.language_server_ids {
                        for server_id in server_ids {
                            let Some(states) = local.language_servers.get(server_id) else {
                                continue;
                            };
                            let (json_adapter, json_server) = match states {
                                LanguageServerState::Running {
                                    adapter, server, ..
                                } if adapter.adapter.is_primary_zed_json_schema_adapter() => {
                                    (adapter.adapter.clone(), server.clone())
                                }
                                _ => continue,
                            };

                            let Some(worktree) = this
                                .worktree_store
                                .read(cx)
                                .worktree_for_id(*worktree_id, cx)
                            else {
                                continue;
                            };
                            let json_delegate: Arc<dyn LspAdapterDelegate> =
                                LocalLspAdapterDelegate::new(
                                    local.languages.clone(),
                                    &local.environment,
                                    weak_ref.clone(),
                                    &worktree,
                                    local.http_client.clone(),
                                    local.fs.clone(),
                                    cx,
                                );

                            servers.push((json_adapter, json_server, json_delegate));
                        }
                    }
                    return Some(servers);
                })
                .ok()
                .flatten();

            let Some(servers) = servers else {
                return;
            };

            let Ok(Some((fs, toolchain_store))) = this.read_with(cx, |this, cx| {
                let local = this.as_local()?;
                let toolchain_store = this.toolchain_store(cx);
                return Some((local.fs.clone(), toolchain_store));
            }) else {
                return;
            };
            for (adapter, server, delegate) in servers {
                adapter.clear_zed_json_schema_cache().await;

                let Some(json_workspace_config) = LocalLspStore::workspace_configuration_for_adapter(
                        adapter,
                        fs.as_ref(),
                        &delegate,
                        toolchain_store.clone(),
                        cx,
                    )
                    .await
                    .context("generate new workspace configuration for JSON language server while trying to refresh JSON Schemas")
                    .ok()
                else {
                    continue;
                };
                server
                    .notify::<lsp::notification::DidChangeConfiguration>(
                        &lsp::DidChangeConfigurationParams {
                            settings: json_workspace_config,
                        },
                    )
                    .ok();
            }
        })
        .detach();
    }

    pub(crate) fn register_buffer_with_language_servers(
        &mut self,
        buffer: &Entity<Buffer>,
        only_register_servers: HashSet<LanguageServerSelector>,
        ignore_refcounts: bool,
        cx: &mut Context<Self>,
    ) -> OpenLspBufferHandle {
        let buffer_id = buffer.read(cx).remote_id();
        let handle = cx.new(|_| buffer.clone());
        if let Some(local) = self.as_local_mut() {
            let refcount = local.registered_buffers.entry(buffer_id).or_insert(0);
            if !ignore_refcounts {
                *refcount += 1;
            }

            // We run early exits on non-existing buffers AFTER we mark the buffer as registered in order to handle buffer saving.
            // When a new unnamed buffer is created and saved, we will start loading it's language. Once the language is loaded, we go over all "language-less" buffers and try to fit that new language
            // with them. However, we do that only for the buffers that we think are open in at least one editor; thus, we need to keep tab of unnamed buffers as well, even though they're not actually registered with any language
            // servers in practice (we don't support non-file URI schemes in our LSP impl).
            let Some(file) = File::from_dyn(buffer.read(cx).file()) else {
                return handle;
            };
            if !file.is_local() {
                return handle;
            }

            if ignore_refcounts || *refcount == 1 {
                local.register_buffer_with_language_servers(buffer, only_register_servers, cx);
            }
            if !ignore_refcounts {
                cx.observe_release(&handle, move |lsp_store, buffer, cx| {
                    let refcount = {
                        let local = lsp_store.as_local_mut().unwrap();
                        let Some(refcount) = local.registered_buffers.get_mut(&buffer_id) else {
                            debug_panic!("bad refcounting");
                            return;
                        };

                        *refcount -= 1;
                        *refcount
                    };
                    if refcount == 0 {
                        lsp_store.lsp_document_colors.remove(&buffer_id);
                        lsp_store.lsp_code_lens.remove(&buffer_id);
                        let local = lsp_store.as_local_mut().unwrap();
                        local.registered_buffers.remove(&buffer_id);
                        local.buffers_opened_in_servers.remove(&buffer_id);
                        if let Some(file) = File::from_dyn(buffer.read(cx).file()).cloned() {
                            local.unregister_old_buffer_from_language_servers(&buffer, &file, cx);
                        }
                    }
                })
                .detach();
            }
        } else if let Some((upstream_client, upstream_project_id)) = self.upstream_client() {
            let buffer_id = buffer.read(cx).remote_id().to_proto();
            cx.background_spawn(async move {
                upstream_client
                    .request(proto::RegisterBufferWithLanguageServers {
                        project_id: upstream_project_id,
                        buffer_id,
                        only_servers: only_register_servers
                            .into_iter()
                            .map(|selector| {
                                let selector = match selector {
                                    LanguageServerSelector::Id(language_server_id) => {
                                        proto::language_server_selector::Selector::ServerId(
                                            language_server_id.to_proto(),
                                        )
                                    }
                                    LanguageServerSelector::Name(language_server_name) => {
                                        proto::language_server_selector::Selector::Name(
                                            language_server_name.to_string(),
                                        )
                                    }
                                };
                                proto::LanguageServerSelector {
                                    selector: Some(selector),
                                }
                            })
                            .collect(),
                    })
                    .await
            })
            .detach();
        } else {
            panic!("oops!");
        }
        handle
    }

    fn maintain_buffer_languages(
        languages: Arc<LanguageRegistry>,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let mut subscription = languages.subscribe();
        let mut prev_reload_count = languages.reload_count();
        cx.spawn(async move |this, cx| {
            while let Some(()) = subscription.next().await {
                if let Some(this) = this.upgrade() {
                    // If the language registry has been reloaded, then remove and
                    // re-assign the languages on all open buffers.
                    let reload_count = languages.reload_count();
                    if reload_count > prev_reload_count {
                        prev_reload_count = reload_count;
                        this.update(cx, |this, cx| {
                            this.buffer_store.clone().update(cx, |buffer_store, cx| {
                                for buffer in buffer_store.buffers() {
                                    if let Some(f) = File::from_dyn(buffer.read(cx).file()).cloned()
                                    {
                                        buffer
                                            .update(cx, |buffer, cx| buffer.set_language(None, cx));
                                        if let Some(local) = this.as_local_mut() {
                                            local.reset_buffer(&buffer, &f, cx);

                                            if local
                                                .registered_buffers
                                                .contains_key(&buffer.read(cx).remote_id())
                                            {
                                                if let Some(file_url) =
                                                    file_path_to_lsp_url(&f.abs_path(cx)).log_err()
                                                {
                                                    local.unregister_buffer_from_language_servers(
                                                        &buffer, &file_url, cx,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                        })
                        .ok();
                    }

                    this.update(cx, |this, cx| {
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

                        // Deprioritize the invisible worktrees so main worktrees' language servers can be started first,
                        // and reused later in the invisible worktrees.
                        plain_text_buffers.sort_by_key(|buffer| {
                            Reverse(
                                File::from_dyn(buffer.read(cx).file())
                                    .map(|file| file.worktree.read(cx).is_visible()),
                            )
                        });

                        for buffer in plain_text_buffers {
                            this.detect_language_for_buffer(&buffer, cx);
                            if let Some(local) = this.as_local_mut() {
                                local.initialize_buffer(&buffer, cx);
                                if local
                                    .registered_buffers
                                    .contains_key(&buffer.read(cx).remote_id())
                                {
                                    local.register_buffer_with_language_servers(
                                        &buffer,
                                        HashSet::default(),
                                        cx,
                                    );
                                }
                            }
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
        buffer_handle: &Entity<Buffer>,
        cx: &mut Context<Self>,
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

    pub(crate) fn set_language_for_buffer(
        &mut self,
        buffer_entity: &Entity<Buffer>,
        new_language: Arc<Language>,
        cx: &mut Context<Self>,
    ) {
        let buffer = buffer_entity.read(cx);
        let buffer_file = buffer.file().cloned();
        let buffer_id = buffer.remote_id();
        if let Some(local_store) = self.as_local_mut() {
            if local_store.registered_buffers.contains_key(&buffer_id) {
                if let Some(abs_path) =
                    File::from_dyn(buffer_file.as_ref()).map(|file| file.abs_path(cx))
                {
                    if let Some(file_url) = file_path_to_lsp_url(&abs_path).log_err() {
                        local_store.unregister_buffer_from_language_servers(
                            buffer_entity,
                            &file_url,
                            cx,
                        );
                    }
                }
            }
        }
        buffer_entity.update(cx, |buffer, cx| {
            if buffer.language().map_or(true, |old_language| {
                !Arc::ptr_eq(old_language, &new_language)
            }) {
                buffer.set_language(Some(new_language.clone()), cx);
            }
        });

        let settings =
            language_settings(Some(new_language.name()), buffer_file.as_ref(), cx).into_owned();
        let buffer_file = File::from_dyn(buffer_file.as_ref());

        let worktree_id = if let Some(file) = buffer_file {
            let worktree = file.worktree.clone();

            if let Some(local) = self.as_local_mut() {
                if local.registered_buffers.contains_key(&buffer_id) {
                    local.register_buffer_with_language_servers(
                        buffer_entity,
                        HashSet::default(),
                        cx,
                    );
                }
            }
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
            buffer: buffer_entity.clone(),
            new_language: Some(new_language),
        })
    }

    pub fn buffer_store(&self) -> Entity<BufferStore> {
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

    // TODO: remove MultiLspQuery: instead, the proto handler should pick appropriate server(s)
    // Then, use `send_lsp_proto_request` or analogue for most of the LSP proto requests and inline this check inside
    fn is_capable_for_proto_request<R>(
        &self,
        buffer: &Entity<Buffer>,
        request: &R,
        cx: &Context<Self>,
    ) -> bool
    where
        R: LspCommand,
    {
        self.check_if_capable_for_proto_request(
            buffer,
            |capabilities| {
                request.check_capabilities(AdapterServerCapabilities {
                    server_capabilities: capabilities.clone(),
                    code_action_kinds: None,
                })
            },
            cx,
        )
    }

    fn check_if_capable_for_proto_request<F>(
        &self,
        buffer: &Entity<Buffer>,
        check: F,
        cx: &Context<Self>,
    ) -> bool
    where
        F: Fn(&lsp::ServerCapabilities) -> bool,
    {
        let Some(language) = buffer.read(cx).language().cloned() else {
            return false;
        };
        let relevant_language_servers = self
            .languages
            .lsp_adapters(&language.name())
            .into_iter()
            .map(|lsp_adapter| lsp_adapter.name())
            .collect::<HashSet<_>>();
        self.language_server_statuses
            .iter()
            .filter_map(|(server_id, server_status)| {
                relevant_language_servers
                    .contains(&server_status.name)
                    .then_some(server_id)
            })
            .filter_map(|server_id| self.lsp_server_capabilities.get(&server_id))
            .any(check)
    }

    pub fn request_lsp<R>(
        &mut self,
        buffer: Entity<Buffer>,
        server: LanguageServerToQuery,
        request: R,
        cx: &mut Context<Self>,
    ) -> Task<Result<R::Response>>
    where
        R: LspCommand,
        <R::LspRequest as lsp::request::Request>::Result: Send,
        <R::LspRequest as lsp::request::Request>::Params: Send,
    {
        if let Some((upstream_client, upstream_project_id)) = self.upstream_client() {
            return self.send_lsp_proto_request(
                buffer,
                upstream_client,
                upstream_project_id,
                request,
                cx,
            );
        }

        let Some(language_server) = buffer.update(cx, |buffer, cx| match server {
            LanguageServerToQuery::FirstCapable => self.as_local().and_then(|local| {
                local
                    .language_servers_for_buffer(buffer, cx)
                    .find(|(_, server)| {
                        request.check_capabilities(server.adapter_server_capabilities())
                    })
                    .map(|(_, server)| server.clone())
            }),
            LanguageServerToQuery::Other(id) => self
                .language_server_for_local_buffer(buffer, id, cx)
                .and_then(|(_, server)| {
                    request
                        .check_capabilities(server.adapter_server_capabilities())
                        .then(|| Arc::clone(server))
                }),
        }) else {
            return Task::ready(Ok(Default::default()));
        };

        let file = File::from_dyn(buffer.read(cx).file()).and_then(File::as_local);

        let Some(file) = file else {
            return Task::ready(Ok(Default::default()));
        };

        let lsp_params = match request.to_lsp_params_or_response(
            &file.abs_path(cx),
            buffer.read(cx),
            &language_server,
            cx,
        ) {
            Ok(LspParamsOrResponse::Params(lsp_params)) => lsp_params,
            Ok(LspParamsOrResponse::Response(response)) => return Task::ready(Ok(response)),

            Err(err) => {
                let message = format!(
                    "{} via {} failed: {}",
                    request.display_name(),
                    language_server.name(),
                    err
                );
                log::warn!("{message}");
                return Task::ready(Err(anyhow!(message)));
            }
        };

        let status = request.status();
        if !request.check_capabilities(language_server.adapter_server_capabilities()) {
            return Task::ready(Ok(Default::default()));
        }
        return cx.spawn(async move |this, cx| {
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
                            this.on_lsp_work_end(language_server.server_id(), id.to_string(), cx);
                        })
                    })
                    .log_err();
                }))
            } else {
                None
            };

            let result = lsp_request.await.into_response();

            let response = result.map_err(|err| {
                let message = format!(
                    "{} via {} failed: {}",
                    request.display_name(),
                    language_server.name(),
                    err
                );
                log::warn!("{message}");
                anyhow::anyhow!(message)
            })?;

            let response = request
                .response_from_lsp(
                    response,
                    this.upgrade().context("no app context")?,
                    buffer,
                    language_server.server_id(),
                    cx.clone(),
                )
                .await;
            response
        });
    }

    fn on_settings_changed(&mut self, cx: &mut Context<Self>) {
        let mut language_formatters_to_check = Vec::new();
        for buffer in self.buffer_store.read(cx).buffers() {
            let buffer = buffer.read(cx);
            let buffer_file = File::from_dyn(buffer.file());
            let buffer_language = buffer.language();
            let settings = language_settings(buffer_language.map(|l| l.name()), buffer.file(), cx);
            if buffer_language.is_some() {
                language_formatters_to_check.push((
                    buffer_file.map(|f| f.worktree_id(cx)),
                    settings.into_owned(),
                ));
            }
        }

        self.refresh_server_tree(cx);

        if let Some(prettier_store) = self.as_local().map(|s| s.prettier_store.clone()) {
            prettier_store.update(cx, |prettier_store, cx| {
                prettier_store.on_settings_changed(language_formatters_to_check, cx)
            })
        }

        cx.notify();
    }

    fn refresh_server_tree(&mut self, cx: &mut Context<Self>) {
        let buffer_store = self.buffer_store.clone();
        if let Some(local) = self.as_local_mut() {
            let mut adapters = BTreeMap::default();
            let get_adapter = {
                let languages = local.languages.clone();
                let environment = local.environment.clone();
                let weak = local.weak.clone();
                let worktree_store = local.worktree_store.clone();
                let http_client = local.http_client.clone();
                let fs = local.fs.clone();
                move |worktree_id, cx: &mut App| {
                    let worktree = worktree_store.read(cx).worktree_for_id(worktree_id, cx)?;
                    Some(LocalLspAdapterDelegate::new(
                        languages.clone(),
                        &environment,
                        weak.clone(),
                        &worktree,
                        http_client.clone(),
                        fs.clone(),
                        cx,
                    ))
                }
            };

            let mut messages_to_report = Vec::new();
            let to_stop = local.lsp_tree.clone().update(cx, |lsp_tree, cx| {
                let mut rebase = lsp_tree.rebase();
                for buffer_handle in buffer_store.read(cx).buffers().sorted_by_key(|buffer| {
                    Reverse(
                        File::from_dyn(buffer.read(cx).file())
                            .map(|file| file.worktree.read(cx).is_visible()),
                    )
                }) {
                    let buffer = buffer_handle.read(cx);
                     let buffer_id = buffer.remote_id();
                    if !local.registered_buffers.contains_key(&buffer_id) {
                        continue;
                    }
                    if let Some((file, language)) = File::from_dyn(buffer.file())
                        .cloned()
                        .zip(buffer.language().map(|l| l.name()))
                    {
                        let worktree_id = file.worktree_id(cx);
                        let Some(worktree) = local
                            .worktree_store
                            .read(cx)
                            .worktree_for_id(worktree_id, cx)
                        else {
                            continue;
                        };

                        let Some((reused, delegate, nodes)) = local
                            .reuse_existing_language_server(
                                rebase.server_tree(),
                                &worktree,
                                &language,
                                cx,
                            )
                            .map(|(delegate, servers)| (true, delegate, servers))
                            .or_else(|| {
                                let lsp_delegate = adapters
                                    .entry(worktree_id)
                                    .or_insert_with(|| get_adapter(worktree_id, cx))
                                    .clone()?;
                                let delegate = Arc::new(ManifestQueryDelegate::new(
                                    worktree.read(cx).snapshot(),
                                ));
                                let path = file
                                    .path()
                                    .parent()
                                    .map(Arc::from)
                                    .unwrap_or_else(|| file.path().clone());
                                let worktree_path = ProjectPath { worktree_id, path };

                                let nodes = rebase.get(
                                    worktree_path,
                                    AdapterQuery::Language(&language),
                                    delegate.clone(),
                                    cx,
                                );

                                Some((false, lsp_delegate, nodes.collect()))
                            })
                        else {
                            continue;
                        };

                        let abs_path = file.abs_path(cx);
                        for node in nodes {
                            if !reused {
                                let server_id = node.server_id_or_init(
                                    |LaunchDisposition {
                                         server_name,

                                         path,
                                         settings,
                                     }|
                                         {
                                            let uri = Url::from_file_path(
                                                worktree.read(cx).abs_path().join(&path.path),
                                            );
                                            let key = (worktree_id, server_name.clone());
                                            local.language_server_ids.remove(&key);

                                            let adapter = local
                                                .languages
                                                .lsp_adapters(&language)
                                                .into_iter()
                                                .find(|adapter| &adapter.name() == server_name)
                                                .expect("To find LSP adapter");
                                            let server_id = local.start_language_server(
                                                &worktree,
                                                delegate.clone(),
                                                adapter,
                                                settings,
                                                cx,
                                            );
                                            if let Some(state) =
                                                local.language_servers.get(&server_id)
                                            {
                                                if let Ok(uri) = uri {
                                                    state.add_workspace_folder(uri);
                                                };
                                            }
                                            server_id
                                        }
                                );

                                if let Some(language_server_id) = server_id {
                                    messages_to_report.push(LspStoreEvent::LanguageServerUpdate {
                                        language_server_id,
                                        name: node.name(),
                                        message:
                                            proto::update_language_server::Variant::RegisteredForBuffer(
                                                proto::RegisteredForBuffer {
                                                    buffer_abs_path: abs_path.to_string_lossy().to_string(),
                                                    buffer_id: buffer_id.to_proto(),
                                                },
                                            ),
                                    });
                                }
                            }
                        }
                    }
                }
                rebase.finish()
            });
            for message in messages_to_report {
                cx.emit(message);
            }
            for (id, _) in to_stop {
                self.stop_local_language_server(id, cx).detach();
            }
        }
    }

    pub fn apply_code_action(
        &self,
        buffer_handle: Entity<Buffer>,
        mut action: CodeAction,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<ProjectTransaction>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = proto::ApplyCodeAction {
                project_id,
                buffer_id: buffer_handle.read(cx).remote_id().into(),
                action: Some(Self::serialize_code_action(&action)),
            };
            let buffer_store = self.buffer_store();
            cx.spawn(async move |_, cx| {
                let response = upstream_client
                    .request(request)
                    .await?
                    .transaction
                    .context("missing transaction")?;

                buffer_store
                    .update(cx, |buffer_store, cx| {
                        buffer_store.deserialize_project_transaction(response, push_to_history, cx)
                    })?
                    .await
            })
        } else if self.mode.is_local() {
            let Some((lsp_adapter, lang_server)) = buffer_handle.update(cx, |buffer, cx| {
                self.language_server_for_local_buffer(buffer, action.server_id, cx)
                    .map(|(adapter, server)| (adapter.clone(), server.clone()))
            }) else {
                return Task::ready(Ok(ProjectTransaction::default()));
            };
            cx.spawn(async move |this,  cx| {
                LocalLspStore::try_resolve_code_action(&lang_server, &mut action)
                    .await
                    .context("resolving a code action")?;
                if let Some(edit) = action.lsp_action.edit() {
                    if edit.changes.is_some() || edit.document_changes.is_some() {
                        return LocalLspStore::deserialize_workspace_edit(
                            this.upgrade().context("no app present")?,
                            edit.clone(),
                            push_to_history,
                            lsp_adapter.clone(),
                            lang_server.clone(),
                            cx,
                        )
                        .await;
                    }
                }

                if let Some(command) = action.lsp_action.command() {
                    let server_capabilities = lang_server.capabilities();
                    let available_commands = server_capabilities
                        .execute_command_provider
                        .as_ref()
                        .map(|options| options.commands.as_slice())
                        .unwrap_or_default();
                    if available_commands.contains(&command.command) {
                        this.update(cx, |this, _| {
                            this.as_local_mut()
                                .unwrap()
                                .last_workspace_edits_by_language_server
                                .remove(&lang_server.server_id());
                        })?;

                        let _result = lang_server
                            .request::<lsp::request::ExecuteCommand>(lsp::ExecuteCommandParams {
                                command: command.command.clone(),
                                arguments: command.arguments.clone().unwrap_or_default(),
                                ..lsp::ExecuteCommandParams::default()
                            })
                            .await.into_response()
                            .context("execute command")?;

                        return this.update(cx, |this, _| {
                            this.as_local_mut()
                                .unwrap()
                                .last_workspace_edits_by_language_server
                                .remove(&lang_server.server_id())
                                .unwrap_or_default()
                        });
                    } else {
                        log::warn!("Cannot execute a command {} not listed in the language server capabilities", command.command);
                    }
                }

                Ok(ProjectTransaction::default())
            })
        } else {
            Task::ready(Err(anyhow!("no upstream client and not local")))
        }
    }

    pub fn apply_code_action_kind(
        &mut self,
        buffers: HashSet<Entity<Buffer>>,
        kind: CodeActionKind,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<ProjectTransaction>> {
        if let Some(_) = self.as_local() {
            cx.spawn(async move |lsp_store, cx| {
                let buffers = buffers.into_iter().collect::<Vec<_>>();
                let result = LocalLspStore::execute_code_action_kind_locally(
                    lsp_store.clone(),
                    buffers,
                    kind,
                    push_to_history,
                    cx,
                )
                .await;
                lsp_store.update(cx, |lsp_store, _| {
                    lsp_store.update_last_formatting_failure(&result);
                })?;
                result
            })
        } else if let Some((client, project_id)) = self.upstream_client() {
            let buffer_store = self.buffer_store();
            cx.spawn(async move |lsp_store, cx| {
                let result = client
                    .request(proto::ApplyCodeActionKind {
                        project_id,
                        kind: kind.as_str().to_owned(),
                        buffer_ids: buffers
                            .iter()
                            .map(|buffer| {
                                buffer.read_with(cx, |buffer, _| buffer.remote_id().into())
                            })
                            .collect::<Result<_>>()?,
                    })
                    .await
                    .and_then(|result| result.transaction.context("missing transaction"));
                lsp_store.update(cx, |lsp_store, _| {
                    lsp_store.update_last_formatting_failure(&result);
                })?;

                let transaction_response = result?;
                buffer_store
                    .update(cx, |buffer_store, cx| {
                        buffer_store.deserialize_project_transaction(
                            transaction_response,
                            push_to_history,
                            cx,
                        )
                    })?
                    .await
            })
        } else {
            Task::ready(Ok(ProjectTransaction::default()))
        }
    }

    pub fn resolve_inlay_hint(
        &self,
        mut hint: InlayHint,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<InlayHint>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            if !self.check_if_capable_for_proto_request(&buffer, InlayHints::can_resolve_inlays, cx)
            {
                hint.resolve_state = ResolveState::Resolved;
                return Task::ready(Ok(hint));
            }
            let request = proto::ResolveInlayHint {
                project_id,
                buffer_id: buffer.read(cx).remote_id().into(),
                language_server_id: server_id.0 as u64,
                hint: Some(InlayHints::project_to_proto_hint(hint.clone())),
            };
            cx.background_spawn(async move {
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
            let Some(lang_server) = buffer.update(cx, |buffer, cx| {
                self.language_server_for_local_buffer(buffer, server_id, cx)
                    .map(|(_, server)| server.clone())
            }) else {
                return Task::ready(Ok(hint));
            };
            if !InlayHints::can_resolve_inlays(&lang_server.capabilities()) {
                return Task::ready(Ok(hint));
            }
            let buffer_snapshot = buffer.read(cx).snapshot();
            cx.spawn(async move |_, cx| {
                let resolve_task = lang_server.request::<lsp::request::InlayHintResolveRequest>(
                    InlayHints::project_to_lsp_hint(hint, &buffer_snapshot),
                );
                let resolved_hint = resolve_task
                    .await
                    .into_response()
                    .context("inlay hint resolve LSP request")?;
                let resolved_hint = InlayHints::lsp_to_project_hint(
                    resolved_hint,
                    &buffer,
                    server_id,
                    ResolveState::Resolved,
                    false,
                    cx,
                )
                .await?;
                Ok(resolved_hint)
            })
        }
    }

    pub fn resolve_color_presentation(
        &mut self,
        mut color: DocumentColor,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) -> Task<Result<DocumentColor>> {
        if color.resolved {
            return Task::ready(Ok(color));
        }

        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let start = color.lsp_range.start;
            let end = color.lsp_range.end;
            let request = proto::GetColorPresentation {
                project_id,
                server_id: server_id.to_proto(),
                buffer_id: buffer.read(cx).remote_id().into(),
                color: Some(proto::ColorInformation {
                    red: color.color.red,
                    green: color.color.green,
                    blue: color.color.blue,
                    alpha: color.color.alpha,
                    lsp_range_start: Some(proto::PointUtf16 {
                        row: start.line,
                        column: start.character,
                    }),
                    lsp_range_end: Some(proto::PointUtf16 {
                        row: end.line,
                        column: end.character,
                    }),
                }),
            };
            cx.background_spawn(async move {
                let response = upstream_client
                    .request(request)
                    .await
                    .context("color presentation proto request")?;
                color.resolved = true;
                color.color_presentations = response
                    .presentations
                    .into_iter()
                    .map(|presentation| ColorPresentation {
                        label: SharedString::from(presentation.label),
                        text_edit: presentation.text_edit.and_then(deserialize_lsp_edit),
                        additional_text_edits: presentation
                            .additional_text_edits
                            .into_iter()
                            .filter_map(deserialize_lsp_edit)
                            .collect(),
                    })
                    .collect();
                Ok(color)
            })
        } else {
            let path = match buffer
                .update(cx, |buffer, cx| {
                    Some(File::from_dyn(buffer.file())?.abs_path(cx))
                })
                .context("buffer with the missing path")
            {
                Ok(path) => path,
                Err(e) => return Task::ready(Err(e)),
            };
            let Some(lang_server) = buffer.update(cx, |buffer, cx| {
                self.language_server_for_local_buffer(buffer, server_id, cx)
                    .map(|(_, server)| server.clone())
            }) else {
                return Task::ready(Ok(color));
            };
            cx.background_spawn(async move {
                let resolve_task = lang_server.request::<lsp::request::ColorPresentationRequest>(
                    lsp::ColorPresentationParams {
                        text_document: make_text_document_identifier(&path)?,
                        color: color.color,
                        range: color.lsp_range,
                        work_done_progress_params: Default::default(),
                        partial_result_params: Default::default(),
                    },
                );
                color.color_presentations = resolve_task
                    .await
                    .into_response()
                    .context("color presentation resolve LSP request")?
                    .into_iter()
                    .map(|presentation| ColorPresentation {
                        label: SharedString::from(presentation.label),
                        text_edit: presentation.text_edit,
                        additional_text_edits: presentation
                            .additional_text_edits
                            .unwrap_or_default(),
                    })
                    .collect();
                color.resolved = true;
                Ok(color)
            })
        }
    }

    pub(crate) fn linked_edits(
        &mut self,
        buffer: &Entity<Buffer>,
        position: Anchor,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<Range<Anchor>>>> {
        let snapshot = buffer.read(cx).snapshot();
        let scope = snapshot.language_scope_at(position);
        let Some(server_id) = self
            .as_local()
            .and_then(|local| {
                buffer.update(cx, |buffer, cx| {
                    local
                        .language_servers_for_buffer(buffer, cx)
                        .filter(|(_, server)| {
                            LinkedEditingRange::check_server_capabilities(server.capabilities())
                        })
                        .filter(|(adapter, _)| {
                            scope
                                .as_ref()
                                .map(|scope| scope.language_allowed(&adapter.name))
                                .unwrap_or(true)
                        })
                        .map(|(_, server)| LanguageServerToQuery::Other(server.server_id()))
                        .next()
                })
            })
            .or_else(|| {
                self.upstream_client()
                    .is_some()
                    .then_some(LanguageServerToQuery::FirstCapable)
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
            return Task::ready(Ok(Vec::new()));
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
        buffer: Entity<Buffer>,
        position: Anchor,
        trigger: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            if !self.check_if_capable_for_proto_request(
                &buffer,
                |capabilities| {
                    OnTypeFormatting::supports_on_type_formatting(&trigger, capabilities)
                },
                cx,
            ) {
                return Task::ready(Ok(None));
            }
            let request = proto::OnTypeFormatting {
                project_id,
                buffer_id: buffer.read(cx).remote_id().into(),
                position: Some(serialize_anchor(&position)),
                trigger,
                version: serialize_version(&buffer.read(cx).version()),
            };
            cx.background_spawn(async move {
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
            cx.spawn(async move |this, cx| {
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
                    .update(cx, |buffer, _| {
                        buffer.wait_for_edits(Some(position.timestamp))
                    })?
                    .await?;
                this.update(cx, |this, cx| {
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
        buffer: Entity<Buffer>,
        position: T,
        trigger: String,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        let position = position.to_point_utf16(buffer.read(cx));
        self.on_type_format_impl(buffer, position, trigger, push_to_history, cx)
    }

    fn on_type_format_impl(
        &mut self,
        buffer: Entity<Buffer>,
        position: PointUtf16,
        trigger: String,
        push_to_history: bool,
        cx: &mut Context<Self>,
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

        cx.spawn(async move |this, cx| {
            if let Some(waiter) =
                buffer.update(cx, |buffer, _| buffer.wait_for_autoindent_applied())?
            {
                waiter.await?;
            }
            cx.update(|cx| {
                this.update(cx, |this, cx| {
                    this.request_lsp(
                        buffer.clone(),
                        LanguageServerToQuery::FirstCapable,
                        OnTypeFormatting {
                            position,
                            trigger,
                            options,
                            push_to_history,
                        },
                        cx,
                    )
                })
            })??
            .await
        })
    }

    pub fn definitions(
        &mut self,
        buffer_handle: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetDefinitions { position };
            if !self.is_capable_for_proto_request(buffer_handle, &request, cx) {
                return Task::ready(Ok(Vec::new()));
            }
            let request_task = upstream_client.request(proto::MultiLspQuery {
                buffer_id: buffer_handle.read(cx).remote_id().into(),
                version: serialize_version(&buffer_handle.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetDefinition(
                    request.to_proto(project_id, buffer_handle.read(cx)),
                )),
            });
            let buffer = buffer_handle.clone();
            cx.spawn(async move |weak_project, cx| {
                let Some(project) = weak_project.upgrade() else {
                    return Ok(Vec::new());
                };
                let responses = request_task.await?.responses;
                let actions = join_all(
                    responses
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetDefinitionResponse(response) => {
                                Some(response)
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|definitions_response| {
                            GetDefinitions { position }.response_from_proto(
                                definitions_response,
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
                    .dedup()
                    .collect())
            })
        } else {
            let definitions_task = self.request_multiple_lsp_locally(
                buffer_handle,
                Some(position),
                GetDefinitions { position },
                cx,
            );
            cx.background_spawn(async move {
                Ok(definitions_task
                    .await
                    .into_iter()
                    .flat_map(|(_, definitions)| definitions)
                    .dedup()
                    .collect())
            })
        }
    }

    pub fn declarations(
        &mut self,
        buffer_handle: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetDeclarations { position };
            if !self.is_capable_for_proto_request(buffer_handle, &request, cx) {
                return Task::ready(Ok(Vec::new()));
            }
            let request_task = upstream_client.request(proto::MultiLspQuery {
                buffer_id: buffer_handle.read(cx).remote_id().into(),
                version: serialize_version(&buffer_handle.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetDeclaration(
                    request.to_proto(project_id, buffer_handle.read(cx)),
                )),
            });
            let buffer = buffer_handle.clone();
            cx.spawn(async move |weak_project, cx| {
                let Some(project) = weak_project.upgrade() else {
                    return Ok(Vec::new());
                };
                let responses = request_task.await?.responses;
                let actions = join_all(
                    responses
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetDeclarationResponse(response) => {
                                Some(response)
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|declarations_response| {
                            GetDeclarations { position }.response_from_proto(
                                declarations_response,
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
                    .dedup()
                    .collect())
            })
        } else {
            let declarations_task = self.request_multiple_lsp_locally(
                buffer_handle,
                Some(position),
                GetDeclarations { position },
                cx,
            );
            cx.background_spawn(async move {
                Ok(declarations_task
                    .await
                    .into_iter()
                    .flat_map(|(_, declarations)| declarations)
                    .dedup()
                    .collect())
            })
        }
    }

    pub fn type_definitions(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetTypeDefinitions { position };
            if !self.is_capable_for_proto_request(&buffer, &request, cx) {
                return Task::ready(Ok(Vec::new()));
            }
            let request_task = upstream_client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetTypeDefinition(
                    request.to_proto(project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(async move |weak_project, cx| {
                let Some(project) = weak_project.upgrade() else {
                    return Ok(Vec::new());
                };
                let responses = request_task.await?.responses;
                let actions = join_all(
                    responses
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetTypeDefinitionResponse(response) => {
                                Some(response)
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|type_definitions_response| {
                            GetTypeDefinitions { position }.response_from_proto(
                                type_definitions_response,
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
                    .dedup()
                    .collect())
            })
        } else {
            let type_definitions_task = self.request_multiple_lsp_locally(
                buffer,
                Some(position),
                GetTypeDefinitions { position },
                cx,
            );
            cx.background_spawn(async move {
                Ok(type_definitions_task
                    .await
                    .into_iter()
                    .flat_map(|(_, type_definitions)| type_definitions)
                    .dedup()
                    .collect())
            })
        }
    }

    pub fn implementations(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LocationLink>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetImplementations { position };
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(Vec::new()));
            }
            let request_task = upstream_client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetImplementation(
                    request.to_proto(project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(async move |weak_project, cx| {
                let Some(project) = weak_project.upgrade() else {
                    return Ok(Vec::new());
                };
                let responses = request_task.await?.responses;
                let actions = join_all(
                    responses
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetImplementationResponse(response) => {
                                Some(response)
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|implementations_response| {
                            GetImplementations { position }.response_from_proto(
                                implementations_response,
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
                    .dedup()
                    .collect())
            })
        } else {
            let implementations_task = self.request_multiple_lsp_locally(
                buffer,
                Some(position),
                GetImplementations { position },
                cx,
            );
            cx.background_spawn(async move {
                Ok(implementations_task
                    .await
                    .into_iter()
                    .flat_map(|(_, implementations)| implementations)
                    .dedup()
                    .collect())
            })
        }
    }

    pub fn references(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<Location>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetReferences { position };
            if !self.is_capable_for_proto_request(&buffer, &request, cx) {
                return Task::ready(Ok(Vec::new()));
            }
            let request_task = upstream_client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetReferences(
                    request.to_proto(project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(async move |weak_project, cx| {
                let Some(project) = weak_project.upgrade() else {
                    return Ok(Vec::new());
                };
                let responses = request_task.await?.responses;
                let actions = join_all(
                    responses
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetReferencesResponse(response) => {
                                Some(response)
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|references_response| {
                            GetReferences { position }.response_from_proto(
                                references_response,
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
                    .dedup()
                    .collect())
            })
        } else {
            let references_task = self.request_multiple_lsp_locally(
                buffer,
                Some(position),
                GetReferences { position },
                cx,
            );
            cx.background_spawn(async move {
                Ok(references_task
                    .await
                    .into_iter()
                    .flat_map(|(_, references)| references)
                    .dedup()
                    .collect())
            })
        }
    }

    pub fn code_actions(
        &mut self,
        buffer: &Entity<Buffer>,
        range: Range<Anchor>,
        kinds: Option<Vec<CodeActionKind>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<CodeAction>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetCodeActions {
                range: range.clone(),
                kinds: kinds.clone(),
            };
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(Vec::new()));
            }
            let request_task = upstream_client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetCodeActions(
                    request.to_proto(project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(async move |weak_project, cx| {
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
                                kinds: kinds.clone(),
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
                buffer,
                Some(range.start),
                GetCodeActions {
                    range: range.clone(),
                    kinds: kinds.clone(),
                },
                cx,
            );
            cx.background_spawn(async move {
                Ok(all_actions_task
                    .await
                    .into_iter()
                    .flat_map(|(_, actions)| actions)
                    .collect())
            })
        }
    }

    pub fn code_lens_actions(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> CodeLensTask {
        let version_queried_for = buffer.read(cx).version();
        let buffer_id = buffer.read(cx).remote_id();

        if let Some(cached_data) = self.lsp_code_lens.get(&buffer_id) {
            if !version_queried_for.changed_since(&cached_data.lens_for_version) {
                let has_different_servers = self.as_local().is_some_and(|local| {
                    local
                        .buffers_opened_in_servers
                        .get(&buffer_id)
                        .cloned()
                        .unwrap_or_default()
                        != cached_data.lens.keys().copied().collect()
                });
                if !has_different_servers {
                    return Task::ready(Ok(cached_data.lens.values().flatten().cloned().collect()))
                        .shared();
                }
            }
        }

        let lsp_data = self.lsp_code_lens.entry(buffer_id).or_default();
        if let Some((updating_for, running_update)) = &lsp_data.update {
            if !version_queried_for.changed_since(&updating_for) {
                return running_update.clone();
            }
        }
        let buffer = buffer.clone();
        let query_version_queried_for = version_queried_for.clone();
        let new_task = cx
            .spawn(async move |lsp_store, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;
                let fetched_lens = lsp_store
                    .update(cx, |lsp_store, cx| lsp_store.fetch_code_lens(&buffer, cx))
                    .map_err(Arc::new)?
                    .await
                    .context("fetching code lens")
                    .map_err(Arc::new);
                let fetched_lens = match fetched_lens {
                    Ok(fetched_lens) => fetched_lens,
                    Err(e) => {
                        lsp_store
                            .update(cx, |lsp_store, _| {
                                lsp_store.lsp_code_lens.entry(buffer_id).or_default().update = None;
                            })
                            .ok();
                        return Err(e);
                    }
                };

                lsp_store
                    .update(cx, |lsp_store, _| {
                        let lsp_data = lsp_store.lsp_code_lens.entry(buffer_id).or_default();
                        if lsp_data.lens_for_version == query_version_queried_for {
                            lsp_data.lens.extend(fetched_lens.clone());
                        } else if !lsp_data
                            .lens_for_version
                            .changed_since(&query_version_queried_for)
                        {
                            lsp_data.lens_for_version = query_version_queried_for;
                            lsp_data.lens = fetched_lens.clone();
                        }
                        lsp_data.update = None;
                        lsp_data.lens.values().flatten().cloned().collect()
                    })
                    .map_err(Arc::new)
            })
            .shared();
        lsp_data.update = Some((version_queried_for, new_task.clone()));
        new_task
    }

    fn fetch_code_lens(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<HashMap<LanguageServerId, Vec<CodeAction>>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetCodeLens;
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(HashMap::default()));
            }
            let request_task = upstream_client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetCodeLens(
                    request.to_proto(project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(async move |weak_lsp_store, cx| {
                let Some(lsp_store) = weak_lsp_store.upgrade() else {
                    return Ok(HashMap::default());
                };
                let responses = request_task.await?.responses;
                let code_lens_actions = join_all(
                    responses
                        .into_iter()
                        .filter_map(|lsp_response| {
                            let response = match lsp_response.response? {
                                proto::lsp_response::Response::GetCodeLensResponse(response) => {
                                    Some(response)
                                }
                                unexpected => {
                                    debug_panic!("Unexpected response: {unexpected:?}");
                                    None
                                }
                            }?;
                            let server_id = LanguageServerId::from_proto(lsp_response.server_id);
                            Some((server_id, response))
                        })
                        .map(|(server_id, code_lens_response)| {
                            let lsp_store = lsp_store.clone();
                            let buffer = buffer.clone();
                            let cx = cx.clone();
                            async move {
                                (
                                    server_id,
                                    GetCodeLens
                                        .response_from_proto(
                                            code_lens_response,
                                            lsp_store,
                                            buffer,
                                            cx,
                                        )
                                        .await,
                                )
                            }
                        }),
                )
                .await;

                let mut has_errors = false;
                let code_lens_actions = code_lens_actions
                    .into_iter()
                    .filter_map(|(server_id, code_lens)| match code_lens {
                        Ok(code_lens) => Some((server_id, code_lens)),
                        Err(e) => {
                            has_errors = true;
                            log::error!("{e:#}");
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>();
                anyhow::ensure!(
                    !has_errors || !code_lens_actions.is_empty(),
                    "Failed to fetch code lens"
                );
                Ok(code_lens_actions)
            })
        } else {
            let code_lens_actions_task =
                self.request_multiple_lsp_locally(buffer, None::<usize>, GetCodeLens, cx);
            cx.background_spawn(
                async move { Ok(code_lens_actions_task.await.into_iter().collect()) },
            )
        }
    }

    #[inline(never)]
    pub fn completions(
        &self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        context: CompletionContext,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let language_registry = self.languages.clone();

        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetCompletions { position, context };
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(Vec::new()));
            }
            let task = self.send_lsp_proto_request(
                buffer.clone(),
                upstream_client,
                project_id,
                request,
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
                let completion_response = task.await?;
                let completions = populate_labels_for_completions(
                    completion_response.completions,
                    language,
                    lsp_adapter,
                )
                .await;
                Ok(vec![CompletionResponse {
                    completions,
                    is_incomplete: completion_response.is_incomplete,
                }])
            })
        } else if let Some(local) = self.as_local() {
            let snapshot = buffer.read(cx).snapshot();
            let offset = position.to_offset(&snapshot);
            let scope = snapshot.language_scope_at(offset);
            let language = snapshot.language().cloned();
            let completion_settings = language_settings(
                language.as_ref().map(|language| language.name()),
                buffer.read(cx).file(),
                cx,
            )
            .completions;
            if !completion_settings.lsp {
                return Task::ready(Ok(Vec::new()));
            }

            let server_ids: Vec<_> = buffer.update(cx, |buffer, cx| {
                local
                    .language_servers_for_buffer(buffer, cx)
                    .filter(|(_, server)| server.capabilities().completion_provider.is_some())
                    .filter(|(adapter, _)| {
                        scope
                            .as_ref()
                            .map(|scope| scope.language_allowed(&adapter.name))
                            .unwrap_or(true)
                    })
                    .map(|(_, server)| server.server_id())
                    .collect()
            });

            let buffer = buffer.clone();
            let lsp_timeout = completion_settings.lsp_fetch_timeout_ms;
            let lsp_timeout = if lsp_timeout > 0 {
                Some(Duration::from_millis(lsp_timeout))
            } else {
                None
            };
            cx.spawn(async move |this,  cx| {
                let mut tasks = Vec::with_capacity(server_ids.len());
                this.update(cx, |lsp_store, cx| {
                    for server_id in server_ids {
                        let lsp_adapter = lsp_store.language_server_adapter_for_id(server_id);
                        let lsp_timeout = lsp_timeout
                            .map(|lsp_timeout| cx.background_executor().timer(lsp_timeout));
                        let mut timeout = cx.background_spawn(async move {
                            match lsp_timeout {
                                Some(lsp_timeout) => {
                                    lsp_timeout.await;
                                    true
                                },
                                None => false,
                            }
                        }).fuse();
                        let mut lsp_request = lsp_store.request_lsp(
                            buffer.clone(),
                            LanguageServerToQuery::Other(server_id),
                            GetCompletions {
                                position,
                                context: context.clone(),
                            },
                            cx,
                        ).fuse();
                        let new_task = cx.background_spawn(async move {
                            select_biased! {
                                response = lsp_request => anyhow::Ok(Some(response?)),
                                timeout_happened = timeout => {
                                    if timeout_happened {
                                        log::warn!("Fetching completions from server {server_id} timed out, timeout ms: {}", completion_settings.lsp_fetch_timeout_ms);
                                        Ok(None)
                                    } else {
                                        let completions = lsp_request.await?;
                                        Ok(Some(completions))
                                    }
                                },
                            }
                        });
                        tasks.push((lsp_adapter, new_task));
                    }
                })?;

                let futures = tasks.into_iter().map(async |(lsp_adapter, task)| {
                    let completion_response = task.await.ok()??;
                    let completions = populate_labels_for_completions(
                            completion_response.completions,
                            language.clone(),
                            lsp_adapter,
                        )
                        .await;
                    Some(CompletionResponse {
                        completions,
                        is_incomplete: completion_response.is_incomplete,
                    })
                });

                let responses: Vec<Option<CompletionResponse>> = join_all(futures).await;

                Ok(responses.into_iter().flatten().collect())
            })
        } else {
            Task::ready(Err(anyhow!("No upstream client or local language server")))
        }
    }

    pub fn resolve_completions(
        &self,
        buffer: Entity<Buffer>,
        completion_indices: Vec<usize>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        let client = self.upstream_client();
        let buffer_id = buffer.read(cx).remote_id();
        let buffer_snapshot = buffer.read(cx).snapshot();

        if !self.check_if_capable_for_proto_request(
            &buffer,
            GetCompletions::can_resolve_completions,
            cx,
        ) {
            return Task::ready(Ok(false));
        }
        cx.spawn(async move |lsp_store, cx| {
            let mut did_resolve = false;
            if let Some((client, project_id)) = client {
                for completion_index in completion_indices {
                    let server_id = {
                        let completion = &completions.borrow()[completion_index];
                        completion.source.server_id()
                    };
                    if let Some(server_id) = server_id {
                        if Self::resolve_completion_remote(
                            project_id,
                            server_id,
                            buffer_id,
                            completions.clone(),
                            completion_index,
                            client.clone(),
                        )
                        .await
                        .log_err()
                        .is_some()
                        {
                            did_resolve = true;
                        }
                    } else {
                        resolve_word_completion(
                            &buffer_snapshot,
                            &mut completions.borrow_mut()[completion_index],
                        );
                    }
                }
            } else {
                for completion_index in completion_indices {
                    let server_id = {
                        let completion = &completions.borrow()[completion_index];
                        completion.source.server_id()
                    };
                    if let Some(server_id) = server_id {
                        let server_and_adapter = lsp_store
                            .read_with(cx, |lsp_store, _| {
                                let server = lsp_store.language_server_for_id(server_id)?;
                                let adapter =
                                    lsp_store.language_server_adapter_for_id(server.server_id())?;
                                Some((server, adapter))
                            })
                            .ok()
                            .flatten();
                        let Some((server, adapter)) = server_and_adapter else {
                            continue;
                        };

                        let resolved = Self::resolve_completion_local(
                            server,
                            completions.clone(),
                            completion_index,
                        )
                        .await
                        .log_err()
                        .is_some();
                        if resolved {
                            Self::regenerate_completion_labels(
                                adapter,
                                &buffer_snapshot,
                                completions.clone(),
                                completion_index,
                            )
                            .await
                            .log_err();
                            did_resolve = true;
                        }
                    } else {
                        resolve_word_completion(
                            &buffer_snapshot,
                            &mut completions.borrow_mut()[completion_index],
                        );
                    }
                }
            }

            Ok(did_resolve)
        })
    }

    async fn resolve_completion_local(
        server: Arc<lsp::LanguageServer>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        completion_index: usize,
    ) -> Result<()> {
        let server_id = server.server_id();
        if !GetCompletions::can_resolve_completions(&server.capabilities()) {
            return Ok(());
        }

        let request = {
            let completion = &completions.borrow()[completion_index];
            match &completion.source {
                CompletionSource::Lsp {
                    lsp_completion,
                    resolved,
                    server_id: completion_server_id,
                    ..
                } => {
                    if *resolved {
                        return Ok(());
                    }
                    anyhow::ensure!(
                        server_id == *completion_server_id,
                        "server_id mismatch, querying completion resolve for {server_id} but completion server id is {completion_server_id}"
                    );
                    server.request::<lsp::request::ResolveCompletionItem>(*lsp_completion.clone())
                }
                CompletionSource::BufferWord { .. }
                | CompletionSource::Dap { .. }
                | CompletionSource::Custom => {
                    return Ok(());
                }
            }
        };
        let resolved_completion = request
            .await
            .into_response()
            .context("resolve completion")?;

        // We must not use any data such as sortText, filterText, insertText and textEdit to edit `Completion` since they are not suppose change during resolve.
        // Refer: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_completion

        let mut completions = completions.borrow_mut();
        let completion = &mut completions[completion_index];
        if let CompletionSource::Lsp {
            lsp_completion,
            resolved,
            server_id: completion_server_id,
            ..
        } = &mut completion.source
        {
            if *resolved {
                return Ok(());
            }
            anyhow::ensure!(
                server_id == *completion_server_id,
                "server_id mismatch, applying completion resolve for {server_id} but completion server id is {completion_server_id}"
            );
            *lsp_completion = Box::new(resolved_completion);
            *resolved = true;
        }
        Ok(())
    }

    async fn regenerate_completion_labels(
        adapter: Arc<CachedLspAdapter>,
        snapshot: &BufferSnapshot,
        completions: Rc<RefCell<Box<[Completion]>>>,
        completion_index: usize,
    ) -> Result<()> {
        let completion_item = completions.borrow()[completion_index]
            .source
            .lsp_completion(true)
            .map(Cow::into_owned);
        if let Some(lsp_documentation) = completion_item
            .as_ref()
            .and_then(|completion_item| completion_item.documentation.clone())
        {
            let mut completions = completions.borrow_mut();
            let completion = &mut completions[completion_index];
            completion.documentation = Some(lsp_documentation.into());
        } else {
            let mut completions = completions.borrow_mut();
            let completion = &mut completions[completion_index];
            completion.documentation = Some(CompletionDocumentation::Undocumented);
        }

        let mut new_label = match completion_item {
            Some(completion_item) => {
                // NB: Zed does not have `details` inside the completion resolve capabilities, but certain language servers violate the spec and do not return `details` immediately, e.g. https://github.com/yioneko/vtsls/issues/213
                // So we have to update the label here anyway...
                let language = snapshot.language();
                match language {
                    Some(language) => {
                        adapter
                            .labels_for_completions(
                                std::slice::from_ref(&completion_item),
                                language,
                            )
                            .await?
                    }
                    None => Vec::new(),
                }
                .pop()
                .flatten()
                .unwrap_or_else(|| {
                    CodeLabel::fallback_for_completion(
                        &completion_item,
                        language.map(|language| language.as_ref()),
                    )
                })
            }
            None => CodeLabel::plain(
                completions.borrow()[completion_index].new_text.clone(),
                None,
            ),
        };
        ensure_uniform_list_compatible_label(&mut new_label);

        let mut completions = completions.borrow_mut();
        let completion = &mut completions[completion_index];
        if completion.label.filter_text() == new_label.filter_text() {
            completion.label = new_label;
        } else {
            log::error!(
                "Resolved completion changed display label from {} to {}. \
                 Refusing to apply this because it changes the fuzzy match text from {} to {}",
                completion.label.text(),
                new_label.text(),
                completion.label.filter_text(),
                new_label.filter_text()
            );
        }

        Ok(())
    }

    async fn resolve_completion_remote(
        project_id: u64,
        server_id: LanguageServerId,
        buffer_id: BufferId,
        completions: Rc<RefCell<Box<[Completion]>>>,
        completion_index: usize,
        client: AnyProtoClient,
    ) -> Result<()> {
        let lsp_completion = {
            let completion = &completions.borrow()[completion_index];
            match &completion.source {
                CompletionSource::Lsp {
                    lsp_completion,
                    resolved,
                    server_id: completion_server_id,
                    ..
                } => {
                    anyhow::ensure!(
                        server_id == *completion_server_id,
                        "remote server_id mismatch, querying completion resolve for {server_id} but completion server id is {completion_server_id}"
                    );
                    if *resolved {
                        return Ok(());
                    }
                    serde_json::to_string(lsp_completion).unwrap().into_bytes()
                }
                CompletionSource::Custom
                | CompletionSource::Dap { .. }
                | CompletionSource::BufferWord { .. } => {
                    return Ok(());
                }
            }
        };
        let request = proto::ResolveCompletionDocumentation {
            project_id,
            language_server_id: server_id.0 as u64,
            lsp_completion,
            buffer_id: buffer_id.into(),
        };

        let response = client
            .request(request)
            .await
            .context("completion documentation resolve proto request")?;
        let resolved_lsp_completion = serde_json::from_slice(&response.lsp_completion)?;

        let documentation = if response.documentation.is_empty() {
            CompletionDocumentation::Undocumented
        } else if response.documentation_is_markdown {
            CompletionDocumentation::MultiLineMarkdown(response.documentation.into())
        } else if response.documentation.lines().count() <= 1 {
            CompletionDocumentation::SingleLine(response.documentation.into())
        } else {
            CompletionDocumentation::MultiLinePlainText(response.documentation.into())
        };

        let mut completions = completions.borrow_mut();
        let completion = &mut completions[completion_index];
        completion.documentation = Some(documentation);
        if let CompletionSource::Lsp {
            insert_range,
            lsp_completion,
            resolved,
            server_id: completion_server_id,
            lsp_defaults: _,
        } = &mut completion.source
        {
            let completion_insert_range = response
                .old_insert_start
                .and_then(deserialize_anchor)
                .zip(response.old_insert_end.and_then(deserialize_anchor));
            *insert_range = completion_insert_range.map(|(start, end)| start..end);

            if *resolved {
                return Ok(());
            }
            anyhow::ensure!(
                server_id == *completion_server_id,
                "remote server_id mismatch, applying completion resolve for {server_id} but completion server id is {completion_server_id}"
            );
            *lsp_completion = Box::new(resolved_lsp_completion);
            *resolved = true;
        }

        let replace_range = response
            .old_replace_start
            .and_then(deserialize_anchor)
            .zip(response.old_replace_end.and_then(deserialize_anchor));
        if let Some((old_replace_start, old_replace_end)) = replace_range {
            if !response.new_text.is_empty() {
                completion.new_text = response.new_text;
                completion.replace_range = old_replace_start..old_replace_end;
            }
        }

        Ok(())
    }

    pub fn apply_additional_edits_for_completion(
        &self,
        buffer_handle: Entity<Buffer>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        completion_index: usize,
        push_to_history: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Transaction>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let buffer = buffer_handle.read(cx);
            let buffer_id = buffer.remote_id();
            cx.spawn(async move |_, cx| {
                let request = {
                    let completion = completions.borrow()[completion_index].clone();
                    proto::ApplyCompletionAdditionalEdits {
                        project_id,
                        buffer_id: buffer_id.into(),
                        completion: Some(Self::serialize_completion(&CoreCompletion {
                            replace_range: completion.replace_range,
                            new_text: completion.new_text,
                            source: completion.source,
                        })),
                    }
                };

                if let Some(transaction) = client.request(request).await?.transaction {
                    let transaction = language::proto::deserialize_transaction(transaction)?;
                    buffer_handle
                        .update(cx, |buffer, _| {
                            buffer.wait_for_edits(transaction.edit_ids.iter().copied())
                        })?
                        .await?;
                    if push_to_history {
                        buffer_handle.update(cx, |buffer, _| {
                            buffer.push_transaction(transaction.clone(), Instant::now());
                            buffer.finalize_last_transaction();
                        })?;
                    }
                    Ok(Some(transaction))
                } else {
                    Ok(None)
                }
            })
        } else {
            let Some(server) = buffer_handle.update(cx, |buffer, cx| {
                let completion = &completions.borrow()[completion_index];
                let server_id = completion.source.server_id()?;
                Some(
                    self.language_server_for_local_buffer(buffer, server_id, cx)?
                        .1
                        .clone(),
                )
            }) else {
                return Task::ready(Ok(None));
            };

            cx.spawn(async move |this, cx| {
                Self::resolve_completion_local(
                    server.clone(),
                    completions.clone(),
                    completion_index,
                )
                .await
                .context("resolving completion")?;
                let completion = completions.borrow()[completion_index].clone();
                let additional_text_edits = completion
                    .source
                    .lsp_completion(true)
                    .as_ref()
                    .and_then(|lsp_completion| lsp_completion.additional_text_edits.clone());
                if let Some(edits) = additional_text_edits {
                    let edits = this
                        .update(cx, |this, cx| {
                            this.as_local_mut().unwrap().edits_from_lsp(
                                &buffer_handle,
                                edits,
                                server.server_id(),
                                None,
                                cx,
                            )
                        })?
                        .await?;

                    buffer_handle.update(cx, |buffer, cx| {
                        buffer.finalize_last_transaction();
                        buffer.start_transaction();

                        for (range, text) in edits {
                            let primary = &completion.replace_range;
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

    pub fn pull_diagnostics(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<LspPullDiagnostics>>> {
        let buffer_id = buffer.read(cx).remote_id();

        if let Some((client, upstream_project_id)) = self.upstream_client() {
            if !self.is_capable_for_proto_request(
                &buffer,
                &GetDocumentDiagnostics {
                    previous_result_id: None,
                },
                cx,
            ) {
                return Task::ready(Ok(Vec::new()));
            }
            let request_task = client.request(proto::MultiLspQuery {
                buffer_id: buffer_id.to_proto(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id: upstream_project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetDocumentDiagnostics(
                    proto::GetDocumentDiagnostics {
                        project_id: upstream_project_id,
                        buffer_id: buffer_id.to_proto(),
                        version: serialize_version(&buffer.read(cx).version()),
                    },
                )),
            });
            cx.background_spawn(async move {
                Ok(request_task
                    .await?
                    .responses
                    .into_iter()
                    .filter_map(|lsp_response| match lsp_response.response? {
                        proto::lsp_response::Response::GetDocumentDiagnosticsResponse(response) => {
                            Some(response)
                        }
                        unexpected => {
                            debug_panic!("Unexpected response: {unexpected:?}");
                            None
                        }
                    })
                    .flat_map(GetDocumentDiagnostics::diagnostics_from_proto)
                    .collect())
            })
        } else {
            let server_ids = buffer.update(cx, |buffer, cx| {
                self.language_servers_for_local_buffer(buffer, cx)
                    .map(|(_, server)| server.server_id())
                    .collect::<Vec<_>>()
            });
            let pull_diagnostics = server_ids
                .into_iter()
                .map(|server_id| {
                    let result_id = self.result_id(server_id, buffer_id, cx);
                    self.request_lsp(
                        buffer.clone(),
                        LanguageServerToQuery::Other(server_id),
                        GetDocumentDiagnostics {
                            previous_result_id: result_id,
                        },
                        cx,
                    )
                })
                .collect::<Vec<_>>();

            cx.background_spawn(async move {
                let mut responses = Vec::new();
                for diagnostics in join_all(pull_diagnostics).await {
                    responses.extend(diagnostics?);
                }
                Ok(responses)
            })
        }
    }

    pub fn inlay_hints(
        &mut self,
        buffer: Entity<Buffer>,
        range: Range<Anchor>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Vec<InlayHint>>> {
        let range_start = range.start;
        let range_end = range.end;
        let buffer_id = buffer.read(cx).remote_id().into();
        let request = InlayHints { range };

        if let Some((client, project_id)) = self.upstream_client() {
            if !self.is_capable_for_proto_request(&buffer, &request, cx) {
                return Task::ready(Ok(Vec::new()));
            }
            let proto_request = proto::InlayHints {
                project_id,
                buffer_id,
                start: Some(serialize_anchor(&range_start)),
                end: Some(serialize_anchor(&range_end)),
                version: serialize_version(&buffer.read(cx).version()),
            };
            cx.spawn(async move |project, cx| {
                let response = client
                    .request(proto_request)
                    .await
                    .context("inlay hints proto request")?;
                LspCommand::response_from_proto(
                    request,
                    response,
                    project.upgrade().context("No project")?,
                    buffer.clone(),
                    cx.clone(),
                )
                .await
                .context("inlay hints proto response conversion")
            })
        } else {
            let lsp_request_task = self.request_lsp(
                buffer.clone(),
                LanguageServerToQuery::FirstCapable,
                request,
                cx,
            );
            cx.spawn(async move |_, cx| {
                buffer
                    .update(cx, |buffer, _| {
                        buffer.wait_for_edits(vec![range_start.timestamp, range_end.timestamp])
                    })?
                    .await
                    .context("waiting for inlay hint request range edits")?;
                lsp_request_task.await.context("inlay hints LSP request")
            })
        }
    }

    pub fn pull_diagnostics_for_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        let buffer_id = buffer.read(cx).remote_id();
        let diagnostics = self.pull_diagnostics(buffer, cx);
        cx.spawn(async move |lsp_store, cx| {
            let diagnostics = diagnostics.await.context("pulling diagnostics")?;
            lsp_store.update(cx, |lsp_store, cx| {
                if lsp_store.as_local().is_none() {
                    return;
                }

                for diagnostics_set in diagnostics {
                    let LspPullDiagnostics::Response {
                        server_id,
                        uri,
                        diagnostics,
                    } = diagnostics_set
                    else {
                        continue;
                    };

                    let adapter = lsp_store.language_server_adapter_for_id(server_id);
                    let disk_based_sources = adapter
                        .as_ref()
                        .map(|adapter| adapter.disk_based_diagnostic_sources.as_slice())
                        .unwrap_or(&[]);
                    match diagnostics {
                        PulledDiagnostics::Unchanged { result_id } => {
                            lsp_store
                                .merge_diagnostics(
                                    server_id,
                                    lsp::PublishDiagnosticsParams {
                                        uri: uri.clone(),
                                        diagnostics: Vec::new(),
                                        version: None,
                                    },
                                    Some(result_id),
                                    DiagnosticSourceKind::Pulled,
                                    disk_based_sources,
                                    |_, _, _| true,
                                    cx,
                                )
                                .log_err();
                        }
                        PulledDiagnostics::Changed {
                            diagnostics,
                            result_id,
                        } => {
                            lsp_store
                                .merge_diagnostics(
                                    server_id,
                                    lsp::PublishDiagnosticsParams {
                                        uri: uri.clone(),
                                        diagnostics,
                                        version: None,
                                    },
                                    result_id,
                                    DiagnosticSourceKind::Pulled,
                                    disk_based_sources,
                                    |buffer, old_diagnostic, _| match old_diagnostic.source_kind {
                                        DiagnosticSourceKind::Pulled => {
                                            buffer.remote_id() != buffer_id
                                        }
                                        DiagnosticSourceKind::Other
                                        | DiagnosticSourceKind::Pushed => true,
                                    },
                                    cx,
                                )
                                .log_err();
                        }
                    }
                }
            })
        })
    }

    pub fn document_colors(
        &mut self,
        fetch_strategy: LspFetchStrategy,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Option<DocumentColorTask> {
        let version_queried_for = buffer.read(cx).version();
        let buffer_id = buffer.read(cx).remote_id();

        match fetch_strategy {
            LspFetchStrategy::IgnoreCache => {}
            LspFetchStrategy::UseCache {
                known_cache_version,
            } => {
                if let Some(cached_data) = self.lsp_document_colors.get(&buffer_id) {
                    if !version_queried_for.changed_since(&cached_data.colors_for_version) {
                        let has_different_servers = self.as_local().is_some_and(|local| {
                            local
                                .buffers_opened_in_servers
                                .get(&buffer_id)
                                .cloned()
                                .unwrap_or_default()
                                != cached_data.colors.keys().copied().collect()
                        });
                        if !has_different_servers {
                            if Some(cached_data.cache_version) == known_cache_version {
                                return None;
                            } else {
                                return Some(
                                    Task::ready(Ok(DocumentColors {
                                        colors: cached_data
                                            .colors
                                            .values()
                                            .flatten()
                                            .cloned()
                                            .collect(),
                                        cache_version: Some(cached_data.cache_version),
                                    }))
                                    .shared(),
                                );
                            }
                        }
                    }
                }
            }
        }

        let lsp_data = self.lsp_document_colors.entry(buffer_id).or_default();
        if let Some((updating_for, running_update)) = &lsp_data.colors_update {
            if !version_queried_for.changed_since(&updating_for) {
                return Some(running_update.clone());
            }
        }
        let query_version_queried_for = version_queried_for.clone();
        let new_task = cx
            .spawn(async move |lsp_store, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;
                let fetched_colors = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.fetch_document_colors_for_buffer(&buffer, cx)
                    })?
                    .await
                    .context("fetching document colors")
                    .map_err(Arc::new);
                let fetched_colors = match fetched_colors {
                    Ok(fetched_colors) => {
                        if fetch_strategy != LspFetchStrategy::IgnoreCache
                            && Some(true)
                                == buffer
                                    .update(cx, |buffer, _| {
                                        buffer.version() != query_version_queried_for
                                    })
                                    .ok()
                        {
                            return Ok(DocumentColors::default());
                        }
                        fetched_colors
                    }
                    Err(e) => {
                        lsp_store
                            .update(cx, |lsp_store, _| {
                                lsp_store
                                    .lsp_document_colors
                                    .entry(buffer_id)
                                    .or_default()
                                    .colors_update = None;
                            })
                            .ok();
                        return Err(e);
                    }
                };

                lsp_store
                    .update(cx, |lsp_store, _| {
                        let lsp_data = lsp_store.lsp_document_colors.entry(buffer_id).or_default();

                        if lsp_data.colors_for_version == query_version_queried_for {
                            lsp_data.colors.extend(fetched_colors.clone());
                            lsp_data.cache_version += 1;
                        } else if !lsp_data
                            .colors_for_version
                            .changed_since(&query_version_queried_for)
                        {
                            lsp_data.colors_for_version = query_version_queried_for;
                            lsp_data.colors = fetched_colors.clone();
                            lsp_data.cache_version += 1;
                        }
                        lsp_data.colors_update = None;
                        let colors = lsp_data
                            .colors
                            .values()
                            .flatten()
                            .cloned()
                            .collect::<HashSet<_>>();
                        DocumentColors {
                            colors,
                            cache_version: Some(lsp_data.cache_version),
                        }
                    })
                    .map_err(Arc::new)
            })
            .shared();
        lsp_data.colors_update = Some((version_queried_for, new_task.clone()));
        Some(new_task)
    }

    fn fetch_document_colors_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<HashMap<LanguageServerId, HashSet<DocumentColor>>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = GetDocumentColor {};
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(HashMap::default()));
            }

            let request_task = client.request(proto::MultiLspQuery {
                project_id,
                buffer_id: buffer.read(cx).remote_id().to_proto(),
                version: serialize_version(&buffer.read(cx).version()),
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetDocumentColor(
                    request.to_proto(project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(async move |project, cx| {
                let Some(project) = project.upgrade() else {
                    return Ok(HashMap::default());
                };
                let colors = join_all(
                    request_task
                        .await
                        .log_err()
                        .map(|response| response.responses)
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|lsp_response| match lsp_response.response? {
                            proto::lsp_response::Response::GetDocumentColorResponse(response) => {
                                Some((
                                    LanguageServerId::from_proto(lsp_response.server_id),
                                    response,
                                ))
                            }
                            unexpected => {
                                debug_panic!("Unexpected response: {unexpected:?}");
                                None
                            }
                        })
                        .map(|(server_id, color_response)| {
                            let response = request.response_from_proto(
                                color_response,
                                project.clone(),
                                buffer.clone(),
                                cx.clone(),
                            );
                            async move { (server_id, response.await.log_err().unwrap_or_default()) }
                        }),
                )
                .await
                .into_iter()
                .fold(HashMap::default(), |mut acc, (server_id, colors)| {
                    acc.entry(server_id)
                        .or_insert_with(HashSet::default)
                        .extend(colors);
                    acc
                });
                Ok(colors)
            })
        } else {
            let document_colors_task =
                self.request_multiple_lsp_locally(buffer, None::<usize>, GetDocumentColor, cx);
            cx.background_spawn(async move {
                Ok(document_colors_task
                    .await
                    .into_iter()
                    .fold(HashMap::default(), |mut acc, (server_id, colors)| {
                        acc.entry(server_id)
                            .or_insert_with(HashSet::default)
                            .extend(colors);
                        acc
                    })
                    .into_iter()
                    .collect())
            })
        }
    }

    pub fn signature_help<T: ToPointUtf16>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: T,
        cx: &mut Context<Self>,
    ) -> Task<Vec<SignatureHelp>> {
        let position = position.to_point_utf16(buffer.read(cx));

        if let Some((client, upstream_project_id)) = self.upstream_client() {
            let request = GetSignatureHelp { position };
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Vec::new());
            }
            let request_task = client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id: upstream_project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetSignatureHelp(
                    request.to_proto(upstream_project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(async move |weak_project, cx| {
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
            cx.background_spawn(async move {
                all_actions_task
                    .await
                    .into_iter()
                    .flat_map(|(_, actions)| actions)
                    .collect::<Vec<_>>()
            })
        }
    }

    pub fn hover(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Hover>> {
        if let Some((client, upstream_project_id)) = self.upstream_client() {
            let request = GetHover { position };
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Vec::new());
            }
            let request_task = client.request(proto::MultiLspQuery {
                buffer_id: buffer.read(cx).remote_id().into(),
                version: serialize_version(&buffer.read(cx).version()),
                project_id: upstream_project_id,
                strategy: Some(proto::multi_lsp_query::Strategy::All(
                    proto::AllLanguageServers {},
                )),
                request: Some(proto::multi_lsp_query::Request::GetHover(
                    request.to_proto(upstream_project_id, buffer.read(cx)),
                )),
            });
            let buffer = buffer.clone();
            cx.spawn(async move |weak_project, cx| {
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
            cx.background_spawn(async move {
                all_actions_task
                    .await
                    .into_iter()
                    .filter_map(|(_, hover)| remove_empty_hover_blocks(hover?))
                    .collect::<Vec<Hover>>()
            })
        }
    }

    pub fn symbols(&self, query: &str, cx: &mut Context<Self>) -> Task<Result<Vec<Symbol>>> {
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
                populate_labels_for_symbols(core_symbols, &language_registry, None, &mut symbols)
                    .await;
                Ok(symbols)
            })
        } else if let Some(local) = self.as_local() {
            struct WorkspaceSymbolsResult {
                server_id: LanguageServerId,
                lsp_adapter: Arc<CachedLspAdapter>,
                worktree: WeakEntity<Worktree>,
                worktree_abs_path: Arc<Path>,
                lsp_symbols: Vec<(String, SymbolKind, lsp::Location)>,
            }

            let mut requests = Vec::new();
            let mut requested_servers = BTreeSet::new();
            'next_server: for ((worktree_id, _), server_ids) in local.language_server_ids.iter() {
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

                let mut servers_to_query = server_ids
                    .difference(&requested_servers)
                    .cloned()
                    .collect::<BTreeSet<_>>();
                for server_id in &servers_to_query {
                    let (lsp_adapter, server) = match local.language_servers.get(server_id) {
                        Some(LanguageServerState::Running {
                            adapter, server, ..
                        }) => (adapter.clone(), server),

                        _ => continue 'next_server,
                    };
                    let supports_workspace_symbol_request =
                        match server.capabilities().workspace_symbol_provider {
                            Some(OneOf::Left(supported)) => supported,
                            Some(OneOf::Right(_)) => true,
                            None => false,
                        };
                    if !supports_workspace_symbol_request {
                        continue 'next_server;
                    }
                    let worktree_abs_path = worktree.abs_path().clone();
                    let worktree_handle = worktree_handle.clone();
                    let server_id = server.server_id();
                    requests.push(
                        server
                            .request::<lsp::request::WorkspaceSymbolRequest>(
                                lsp::WorkspaceSymbolParams {
                                    query: query.to_string(),
                                    ..Default::default()
                                },
                            )
                            .map(move |response| {
                                let lsp_symbols = response.into_response()
                                    .context("workspace symbols request")
                                    .log_err()
                                    .flatten()
                                    .map(|symbol_response| match symbol_response {
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
                                    server_id,
                                    lsp_adapter,
                                    worktree: worktree_handle.downgrade(),
                                    worktree_abs_path,
                                    lsp_symbols,
                                }
                            }),
                    );
                }
                requested_servers.append(&mut servers_to_query);
            }

            cx.spawn(async move |this, cx| {
                let responses = futures::future::join_all(requests).await;
                let this = match this.upgrade() {
                    Some(this) => this,
                    None => return Ok(Vec::new()),
                };

                let mut symbols = Vec::new();
                for result in responses {
                    let core_symbols = this.update(cx, |this, cx| {
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
                                    source_language_server_id: result.server_id,
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
                        Some(result.lsp_adapter),
                        &mut symbols,
                    )
                    .await;
                }

                Ok(symbols)
            })
        } else {
            Task::ready(Err(anyhow!("No upstream client or local language server")))
        }
    }

    pub fn diagnostic_summary(&self, include_ignored: bool, cx: &App) -> DiagnosticSummary {
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
        cx: &'a App,
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

    pub fn on_buffer_edited(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let language_servers: Vec<_> = buffer.update(cx, |buffer, cx| {
            Some(
                self.as_local()?
                    .language_servers_for_buffer(buffer, cx)
                    .map(|i| i.1.clone())
                    .collect(),
            )
        })?;

        let buffer = buffer.read(cx);
        let file = File::from_dyn(buffer.file())?;
        let abs_path = file.as_local()?.abs_path(cx);
        let uri = lsp::Url::from_file_path(abs_path).unwrap();
        let next_snapshot = buffer.text_snapshot();
        for language_server in language_servers {
            let language_server = language_server.clone();

            let buffer_snapshots = self
                .as_local_mut()
                .unwrap()
                .buffer_snapshots
                .get_mut(&buffer.remote_id())
                .and_then(|m| m.get_mut(&language_server.server_id()))?;
            let previous_snapshot = buffer_snapshots.last()?;

            let build_incremental_change = || {
                buffer
                    .edits_since::<Dimensions<PointUtf16, usize>>(
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
                    &lsp::DidChangeTextDocumentParams {
                        text_document: lsp::VersionedTextDocumentIdentifier::new(
                            uri.clone(),
                            next_version,
                        ),
                        content_changes,
                    },
                )
                .ok();
            self.pull_workspace_diagnostics(language_server.server_id());
        }

        None
    }

    pub fn on_buffer_saved(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let file = File::from_dyn(buffer.read(cx).file())?;
        let worktree_id = file.worktree_id(cx);
        let abs_path = file.as_local()?.abs_path(cx);
        let text_document = lsp::TextDocumentIdentifier {
            uri: file_path_to_lsp_url(&abs_path).log_err()?,
        };
        let local = self.as_local()?;

        for server in local.language_servers_for_worktree(worktree_id) {
            if let Some(include_text) = include_text(server.as_ref()) {
                let text = if include_text {
                    Some(buffer.read(cx).text())
                } else {
                    None
                };
                server
                    .notify::<lsp::notification::DidSaveTextDocument>(
                        &lsp::DidSaveTextDocumentParams {
                            text_document: text_document.clone(),
                            text,
                        },
                    )
                    .ok();
            }
        }

        let language_servers = buffer.update(cx, |buffer, cx| {
            local.language_server_ids_for_buffer(buffer, cx)
        });
        for language_server_id in language_servers {
            self.simulate_disk_based_diagnostics_events_if_needed(language_server_id, cx);
        }

        None
    }

    pub(crate) async fn refresh_workspace_configurations(
        lsp_store: &WeakEntity<Self>,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncApp,
    ) {
        maybe!(async move {
            let mut refreshed_servers = HashSet::default();
            let servers = lsp_store
                .update(cx, |lsp_store, cx| {
                    let toolchain_store = lsp_store.toolchain_store(cx);
                    let Some(local) = lsp_store.as_local() else {
                        return Vec::default();
                    };
                    local
                        .language_server_ids
                        .iter()
                        .flat_map(|((worktree_id, _), server_ids)| {
                            let worktree = lsp_store
                                .worktree_store
                                .read(cx)
                                .worktree_for_id(*worktree_id, cx);
                            let delegate = worktree.map(|worktree| {
                                LocalLspAdapterDelegate::new(
                                    local.languages.clone(),
                                    &local.environment,
                                    cx.weak_entity(),
                                    &worktree,
                                    local.http_client.clone(),
                                    local.fs.clone(),
                                    cx,
                                )
                            });

                            let fs = fs.clone();
                            let toolchain_store = toolchain_store.clone();
                            server_ids.iter().filter_map(|server_id| {
                                let delegate = delegate.clone()? as Arc<dyn LspAdapterDelegate>;
                                let states = local.language_servers.get(server_id)?;

                                match states {
                                    LanguageServerState::Starting { .. } => None,
                                    LanguageServerState::Running {
                                        adapter, server, ..
                                    } => {
                                        let fs = fs.clone();
                                        let toolchain_store = toolchain_store.clone();
                                        let adapter = adapter.clone();
                                        let server = server.clone();
                                        refreshed_servers.insert(server.name());
                                        Some(cx.spawn(async move |_, cx| {
                                            let settings =
                                                LocalLspStore::workspace_configuration_for_adapter(
                                                    adapter.adapter.clone(),
                                                    fs.as_ref(),
                                                    &delegate,
                                                    toolchain_store,
                                                    cx,
                                                )
                                                .await
                                                .ok()?;
                                            server
                                                .notify::<lsp::notification::DidChangeConfiguration>(
                                                    &lsp::DidChangeConfigurationParams { settings },
                                                )
                                                .ok()?;
                                            Some(())
                                        }))
                                    }
                                }
                            }).collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .ok()?;

            log::info!("Refreshing workspace configurations for servers {refreshed_servers:?}");
            // TODO this asynchronous job runs concurrently with extension (de)registration and may take enough time for a certain extension
            // to stop and unregister its language server wrapper.
            // This is racy : an extension might have already removed all `local.language_servers` state, but here we `.clone()` and hold onto it anyway.
            // This now causes errors in the logs, we should find a way to remove such servers from the processing everywhere.
            let _: Vec<Option<()>> = join_all(servers).await;
            Some(())
        })
        .await;
    }

    fn toolchain_store(&self, cx: &App) -> Arc<dyn LanguageToolchainStore> {
        if let Some(toolchain_store) = self.toolchain_store.as_ref() {
            toolchain_store.read(cx).as_language_toolchain_store()
        } else {
            Arc::new(EmptyToolchainStore)
        }
    }
    fn maintain_workspace_config(
        fs: Arc<dyn Fs>,
        external_refresh_requests: watch::Receiver<()>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let (mut settings_changed_tx, mut settings_changed_rx) = watch::channel();
        let _ = postage::stream::Stream::try_recv(&mut settings_changed_rx);

        let settings_observation = cx.observe_global::<SettingsStore>(move |_, _| {
            *settings_changed_tx.borrow_mut() = ();
        });

        let mut joint_future =
            futures::stream::select(settings_changed_rx, external_refresh_requests);
        cx.spawn(async move |this, cx| {
            while let Some(()) = joint_future.next().await {
                Self::refresh_workspace_configurations(&this, fs.clone(), cx).await;
            }

            drop(settings_observation);
            anyhow::Ok(())
        })
    }

    pub fn language_servers_for_local_buffer<'a>(
        &'a self,
        buffer: &Buffer,
        cx: &mut App,
    ) -> impl Iterator<Item = (&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        let local = self.as_local();
        let language_server_ids = local
            .map(|local| local.language_server_ids_for_buffer(buffer, cx))
            .unwrap_or_default();

        language_server_ids
            .into_iter()
            .filter_map(
                move |server_id| match local?.language_servers.get(&server_id)? {
                    LanguageServerState::Running {
                        adapter, server, ..
                    } => Some((adapter, server)),
                    _ => None,
                },
            )
    }

    pub fn language_server_for_local_buffer<'a>(
        &'a self,
        buffer: &'a Buffer,
        server_id: LanguageServerId,
        cx: &'a mut App,
    ) -> Option<(&'a Arc<CachedLspAdapter>, &'a Arc<LanguageServer>)> {
        self.as_local()?
            .language_servers_for_buffer(buffer, cx)
            .find(|(_, s)| s.server_id() == server_id)
    }

    fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
        self.diagnostic_summaries.remove(&id_to_remove);
        if let Some(local) = self.as_local_mut() {
            let to_remove = local.remove_worktree(id_to_remove, cx);
            for server in to_remove {
                self.language_server_statuses.remove(&server);
            }
        }
    }

    pub fn shared(
        &mut self,
        project_id: u64,
        downstream_client: AnyProtoClient,
        _: &mut Context<Self>,
    ) {
        self.downstream_client = Some((downstream_client.clone(), project_id));

        for (server_id, status) in &self.language_server_statuses {
            if let Some(server) = self.language_server_for_id(*server_id) {
                downstream_client
                    .send(proto::StartLanguageServer {
                        project_id,
                        server: Some(proto::LanguageServer {
                            id: server_id.to_proto(),
                            name: status.name.to_string(),
                            worktree_id: None,
                        }),
                        capabilities: serde_json::to_string(&server.capabilities())
                            .expect("serializing server LSP capabilities"),
                    })
                    .log_err();
            }
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
                        name: LanguageServerName::from_proto(server.name),
                        pending_work: Default::default(),
                        has_pending_diagnostic_updates: false,
                        progress_tokens: Default::default(),
                    },
                )
            })
            .collect();
    }

    fn register_local_language_server(
        &mut self,
        worktree: Entity<Worktree>,
        language_server_name: LanguageServerName,
        language_server_id: LanguageServerId,
        cx: &mut App,
    ) {
        let Some(local) = self.as_local_mut() else {
            return;
        };

        let worktree_id = worktree.read(cx).id();
        if worktree.read(cx).is_visible() {
            let path = ProjectPath {
                worktree_id,
                path: Arc::from("".as_ref()),
            };
            let delegate = Arc::new(ManifestQueryDelegate::new(worktree.read(cx).snapshot()));
            local.lsp_tree.update(cx, |language_server_tree, cx| {
                for node in language_server_tree.get(
                    path,
                    AdapterQuery::Adapter(&language_server_name),
                    delegate,
                    cx,
                ) {
                    node.server_id_or_init(|disposition| {
                        assert_eq!(disposition.server_name, &language_server_name);

                        language_server_id
                    });
                }
            });
        }

        local
            .language_server_ids
            .entry((worktree_id, language_server_name))
            .or_default()
            .insert(language_server_id);
    }

    #[cfg(test)]
    pub fn update_diagnostic_entries(
        &mut self,
        server_id: LanguageServerId,
        abs_path: PathBuf,
        result_id: Option<String>,
        version: Option<i32>,
        diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<()> {
        self.merge_diagnostic_entries(
            server_id,
            abs_path,
            result_id,
            version,
            diagnostics,
            |_, _, _| false,
            cx,
        )?;
        Ok(())
    }

    pub fn merge_diagnostic_entries(
        &mut self,
        server_id: LanguageServerId,
        abs_path: PathBuf,
        result_id: Option<String>,
        version: Option<i32>,
        mut diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        filter: impl Fn(&Buffer, &Diagnostic, &App) -> bool + Clone,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<()> {
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

        if let Some(buffer_handle) = self.buffer_store.read(cx).get_by_path(&project_path) {
            let snapshot = buffer_handle.read(cx).snapshot();
            let buffer = buffer_handle.read(cx);
            let reused_diagnostics = buffer
                .get_diagnostics(server_id)
                .into_iter()
                .flat_map(|diag| {
                    diag.iter()
                        .filter(|v| filter(buffer, &v.diagnostic, cx))
                        .map(|v| {
                            let start = Unclipped(v.range.start.to_point_utf16(&snapshot));
                            let end = Unclipped(v.range.end.to_point_utf16(&snapshot));
                            DiagnosticEntry {
                                range: start..end,
                                diagnostic: v.diagnostic.clone(),
                            }
                        })
                })
                .collect::<Vec<_>>();

            self.as_local_mut()
                .context("cannot merge diagnostics on a remote LspStore")?
                .update_buffer_diagnostics(
                    &buffer_handle,
                    server_id,
                    result_id,
                    version,
                    diagnostics.clone(),
                    reused_diagnostics.clone(),
                    cx,
                )?;

            diagnostics.extend(reused_diagnostics);
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
        _: &mut Context<Worktree>,
    ) -> Result<bool> {
        let local = match &mut self.mode {
            LspStoreMode::Local(local_lsp_store) => local_lsp_store,
            _ => anyhow::bail!("update_worktree_diagnostics called on remote"),
        };

        let summaries_for_tree = self.diagnostic_summaries.entry(worktree_id).or_default();
        let diagnostics_for_tree = local.diagnostics.entry(worktree_id).or_default();
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
                            path: worktree_path.to_proto(),
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
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = client.request(proto::OpenBufferForSymbol {
                project_id,
                symbol: Some(Self::serialize_symbol(symbol)),
            });
            cx.spawn(async move |this, cx| {
                let response = request.await?;
                let buffer_id = BufferId::new(response.buffer_id)?;
                this.update(cx, |this, cx| this.wait_for_remote_buffer(buffer_id, cx))?
                    .await
            })
        } else if let Some(local) = self.as_local() {
            let Some(language_server_id) = local
                .language_server_ids
                .get(&(
                    symbol.source_worktree_id,
                    symbol.language_server_name.clone(),
                ))
                .and_then(|ids| {
                    ids.contains(&symbol.source_language_server_id)
                        .then_some(symbol.source_language_server_id)
                })
            else {
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
        } else {
            Task::ready(Err(anyhow!("no upstream client or local store")))
        }
    }

    pub fn open_local_buffer_via_lsp(
        &mut self,
        mut abs_path: lsp::Url,
        language_server_id: LanguageServerId,
        language_server_name: LanguageServerName,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        cx.spawn(async move |lsp_store, cx| {
            // Escape percent-encoded string.
            let current_scheme = abs_path.scheme().to_owned();
            let _ = abs_path.set_scheme("file");

            let abs_path = abs_path
                .to_file_path()
                .map_err(|()| anyhow!("can't convert URI to path"))?;
            let p = abs_path.clone();
            let yarn_worktree = lsp_store
                .update(cx, move |lsp_store, cx| match lsp_store.as_local() {
                    Some(local_lsp_store) => local_lsp_store.yarn.update(cx, |_, cx| {
                        cx.spawn(async move |this, cx| {
                            let t = this
                                .update(cx, |this, cx| this.process_path(&p, &current_scheme, cx))
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
                lsp_store.update(cx, |lsp_store, cx| {
                    lsp_store.worktree_store.update(cx, |worktree_store, cx| {
                        worktree_store.find_worktree(&worktree_root_target, cx)
                    })
                })? {
                let relative_path =
                    known_relative_path.unwrap_or_else(|| Arc::<Path>::from(result.1));
                (result.0, relative_path)
            } else {
                let worktree = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.worktree_store.update(cx, |worktree_store, cx| {
                            worktree_store.create_worktree(&worktree_root_target, false, cx)
                        })
                    })?
                    .await?;
                if worktree.read_with(cx, |worktree, _| worktree.is_local())? {
                    lsp_store
                        .update(cx, |lsp_store, cx| {
                            lsp_store.register_local_language_server(
                                worktree.clone(),
                                language_server_name,
                                language_server_id,
                                cx,
                            )
                        })
                        .ok();
                }
                let worktree_root = worktree.read_with(cx, |worktree, _| worktree.abs_path())?;
                let relative_path = if let Some(known_path) = known_relative_path {
                    known_path
                } else {
                    abs_path.strip_prefix(worktree_root)?.into()
                };
                (worktree, relative_path)
            };
            let project_path = ProjectPath {
                worktree_id: worktree.read_with(cx, |worktree, _| worktree.id())?,
                path: relative_path,
            };
            lsp_store
                .update(cx, |lsp_store, cx| {
                    lsp_store.buffer_store().update(cx, |buffer_store, cx| {
                        buffer_store.open_buffer(project_path, cx)
                    })
                })?
                .await
        })
    }

    fn request_multiple_lsp_locally<P, R>(
        &mut self,
        buffer: &Entity<Buffer>,
        position: Option<P>,
        request: R,
        cx: &mut Context<Self>,
    ) -> Task<Vec<(LanguageServerId, R::Response)>>
    where
        P: ToOffset,
        R: LspCommand + Clone,
        <R::LspRequest as lsp::request::Request>::Result: Send,
        <R::LspRequest as lsp::request::Request>::Params: Send,
    {
        let Some(local) = self.as_local() else {
            return Task::ready(Vec::new());
        };

        let snapshot = buffer.read(cx).snapshot();
        let scope = position.and_then(|position| snapshot.language_scope_at(position));

        let server_ids = buffer.update(cx, |buffer, cx| {
            local
                .language_servers_for_buffer(buffer, cx)
                .filter(|(adapter, _)| {
                    scope
                        .as_ref()
                        .map(|scope| scope.language_allowed(&adapter.name))
                        .unwrap_or(true)
                })
                .map(|(_, server)| server.server_id())
                .filter(|server_id| {
                    self.as_local().is_none_or(|local| {
                        local
                            .buffers_opened_in_servers
                            .get(&snapshot.remote_id())
                            .is_some_and(|servers| servers.contains(server_id))
                    })
                })
                .collect::<Vec<_>>()
        });

        let mut response_results = server_ids
            .into_iter()
            .map(|server_id| {
                let task = self.request_lsp(
                    buffer.clone(),
                    LanguageServerToQuery::Other(server_id),
                    request.clone(),
                    cx,
                );
                async move { (server_id, task.await) }
            })
            .collect::<FuturesUnordered<_>>();

        cx.background_spawn(async move {
            let mut responses = Vec::with_capacity(response_results.len());
            while let Some((server_id, response_result)) = response_results.next().await {
                if let Some(response) = response_result.log_err() {
                    responses.push((server_id, response));
                }
            }
            responses
        })
    }

    async fn handle_lsp_command<T: LspCommand>(
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
                    LanguageServerToQuery::FirstCapable,
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
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::MultiLspQuery>,
        mut cx: AsyncApp,
    ) -> Result<proto::MultiLspQueryResponse> {
        let response_from_ssh = lsp_store.read_with(&mut cx, |this, _| {
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
        let buffer = lsp_store.update(&mut cx, |this, cx| {
            this.buffer_store.read(cx).get_existing(buffer_id)
        })??;
        buffer
            .update(&mut cx, |buffer, _| {
                buffer.wait_for_version(version.clone())
            })?
            .await?;
        let buffer_version = buffer.read_with(&mut cx, |buffer, _| buffer.version())?;
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
            Some(proto::multi_lsp_query::Request::GetHover(message)) => {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(deserialize_version(&message.version))
                    })?
                    .await?;
                let get_hover =
                    GetHover::from_proto(message, lsp_store.clone(), buffer.clone(), cx.clone())
                        .await?;
                let all_hovers = lsp_store
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
                    .filter_map(|(server_id, hover)| {
                        Some((server_id, remove_empty_hover_blocks(hover?)?))
                    });
                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: all_hovers
                        .map(|(server_id, hover)| proto::LspResponse {
                            server_id: server_id.to_proto(),
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
            Some(proto::multi_lsp_query::Request::GetCodeActions(message)) => {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(deserialize_version(&message.version))
                    })?
                    .await?;
                let get_code_actions = GetCodeActions::from_proto(
                    message,
                    lsp_store.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let all_actions = lsp_store
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

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: all_actions
                        .map(|(server_id, code_actions)| proto::LspResponse {
                            server_id: server_id.to_proto(),
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
            Some(proto::multi_lsp_query::Request::GetSignatureHelp(message)) => {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(deserialize_version(&message.version))
                    })?
                    .await?;
                let get_signature_help = GetSignatureHelp::from_proto(
                    message,
                    lsp_store.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let all_signatures = lsp_store
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

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: all_signatures
                        .map(|(server_id, signature_help)| proto::LspResponse {
                            server_id: server_id.to_proto(),
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
            Some(proto::multi_lsp_query::Request::GetCodeLens(message)) => {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(deserialize_version(&message.version))
                    })?
                    .await?;
                let get_code_lens =
                    GetCodeLens::from_proto(message, lsp_store.clone(), buffer.clone(), cx.clone())
                        .await?;

                let code_lens_actions = lsp_store
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            None::<usize>,
                            get_code_lens,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: code_lens_actions
                        .map(|(server_id, actions)| proto::LspResponse {
                            server_id: server_id.to_proto(),
                            response: Some(proto::lsp_response::Response::GetCodeLensResponse(
                                GetCodeLens::response_to_proto(
                                    actions,
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
            Some(proto::multi_lsp_query::Request::GetDocumentDiagnostics(message)) => {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(deserialize_version(&message.version))
                    })?
                    .await?;
                lsp_store
                    .update(&mut cx, |lsp_store, cx| {
                        lsp_store.pull_diagnostics_for_buffer(buffer, cx)
                    })?
                    .await?;
                // `pull_diagnostics_for_buffer` will merge in the new diagnostics and send them to the client.
                // The client cannot merge anything into its non-local LspStore, so we do not need to return anything.
                Ok(proto::MultiLspQueryResponse {
                    responses: Vec::new(),
                })
            }
            Some(proto::multi_lsp_query::Request::GetDocumentColor(message)) => {
                buffer
                    .update(&mut cx, |buffer, _| {
                        buffer.wait_for_version(deserialize_version(&message.version))
                    })?
                    .await?;
                let get_document_color = GetDocumentColor::from_proto(
                    message,
                    lsp_store.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let all_colors = lsp_store
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            None::<usize>,
                            get_document_color,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: all_colors
                        .map(|(server_id, colors)| proto::LspResponse {
                            server_id: server_id.to_proto(),
                            response: Some(
                                proto::lsp_response::Response::GetDocumentColorResponse(
                                    GetDocumentColor::response_to_proto(
                                        colors,
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
            Some(proto::multi_lsp_query::Request::GetDefinition(message)) => {
                let get_definitions = GetDefinitions::from_proto(
                    message,
                    lsp_store.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let definitions = lsp_store
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_definitions.position),
                            get_definitions,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: definitions
                        .map(|(server_id, definitions)| proto::LspResponse {
                            server_id: server_id.to_proto(),
                            response: Some(proto::lsp_response::Response::GetDefinitionResponse(
                                GetDefinitions::response_to_proto(
                                    definitions,
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
            Some(proto::multi_lsp_query::Request::GetDeclaration(message)) => {
                let get_declarations = GetDeclarations::from_proto(
                    message,
                    lsp_store.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let declarations = lsp_store
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_declarations.position),
                            get_declarations,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: declarations
                        .map(|(server_id, declarations)| proto::LspResponse {
                            server_id: server_id.to_proto(),
                            response: Some(proto::lsp_response::Response::GetDeclarationResponse(
                                GetDeclarations::response_to_proto(
                                    declarations,
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
            Some(proto::multi_lsp_query::Request::GetTypeDefinition(message)) => {
                let get_type_definitions = GetTypeDefinitions::from_proto(
                    message,
                    lsp_store.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let type_definitions = lsp_store
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_type_definitions.position),
                            get_type_definitions,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: type_definitions
                        .map(|(server_id, type_definitions)| proto::LspResponse {
                            server_id: server_id.to_proto(),
                            response: Some(
                                proto::lsp_response::Response::GetTypeDefinitionResponse(
                                    GetTypeDefinitions::response_to_proto(
                                        type_definitions,
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
            Some(proto::multi_lsp_query::Request::GetImplementation(message)) => {
                let get_implementations = GetImplementations::from_proto(
                    message,
                    lsp_store.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let implementations = lsp_store
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_implementations.position),
                            get_implementations,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: implementations
                        .map(|(server_id, implementations)| proto::LspResponse {
                            server_id: server_id.to_proto(),
                            response: Some(
                                proto::lsp_response::Response::GetImplementationResponse(
                                    GetImplementations::response_to_proto(
                                        implementations,
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
            Some(proto::multi_lsp_query::Request::GetReferences(message)) => {
                let get_references = GetReferences::from_proto(
                    message,
                    lsp_store.clone(),
                    buffer.clone(),
                    cx.clone(),
                )
                .await?;

                let references = lsp_store
                    .update(&mut cx, |project, cx| {
                        project.request_multiple_lsp_locally(
                            &buffer,
                            Some(get_references.position),
                            get_references,
                            cx,
                        )
                    })?
                    .await
                    .into_iter();

                lsp_store.update(&mut cx, |project, cx| proto::MultiLspQueryResponse {
                    responses: references
                        .map(|(server_id, references)| proto::LspResponse {
                            server_id: server_id.to_proto(),
                            response: Some(proto::lsp_response::Response::GetReferencesResponse(
                                GetReferences::response_to_proto(
                                    references,
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
            None => anyhow::bail!("empty multi lsp query request"),
        }
    }

    async fn handle_apply_code_action(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeAction>,
        mut cx: AsyncApp,
    ) -> Result<proto::ApplyCodeActionResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let action =
            Self::deserialize_code_action(envelope.payload.action.context("invalid action")?)?;
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

    async fn handle_register_buffer_with_language_servers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RegisterBufferWithLanguageServers>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let peer_id = envelope.original_sender_id.unwrap_or(envelope.sender_id);
        this.update(&mut cx, |this, cx| {
            if let Some((upstream_client, upstream_project_id)) = this.upstream_client() {
                return upstream_client.send(proto::RegisterBufferWithLanguageServers {
                    project_id: upstream_project_id,
                    buffer_id: buffer_id.to_proto(),
                    only_servers: envelope.payload.only_servers,
                });
            }

            let Some(buffer) = this.buffer_store().read(cx).get(buffer_id) else {
                anyhow::bail!("buffer is not open");
            };

            let handle = this.register_buffer_with_language_servers(
                &buffer,
                envelope
                    .payload
                    .only_servers
                    .into_iter()
                    .filter_map(|selector| {
                        Some(match selector.selector? {
                            proto::language_server_selector::Selector::ServerId(server_id) => {
                                LanguageServerSelector::Id(LanguageServerId::from_proto(server_id))
                            }
                            proto::language_server_selector::Selector::Name(name) => {
                                LanguageServerSelector::Name(LanguageServerName(
                                    SharedString::from(name),
                                ))
                            }
                        })
                    })
                    .collect(),
                false,
                cx,
            );
            this.buffer_store().update(cx, |buffer_store, _| {
                buffer_store.register_shared_lsp_handle(peer_id, buffer_id, handle);
            });

            Ok(())
        })??;
        Ok(proto::Ack {})
    }

    async fn handle_rename_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RenameProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let (worktree_id, worktree, old_path, is_dir) = this
            .update(&mut cx, |this, cx| {
                this.worktree_store
                    .read(cx)
                    .worktree_and_entry_for_id(entry_id, cx)
                    .map(|(worktree, entry)| {
                        (
                            worktree.read(cx).id(),
                            worktree,
                            entry.path.clone(),
                            entry.is_dir(),
                        )
                    })
            })?
            .context("worktree not found")?;
        let (old_abs_path, new_abs_path) = {
            let root_path = worktree.read_with(&mut cx, |this, _| this.abs_path())?;
            let new_path = PathBuf::from_proto(envelope.payload.new_path.clone());
            (root_path.join(&old_path), root_path.join(&new_path))
        };

        Self::will_rename_entry(
            this.downgrade(),
            worktree_id,
            &old_abs_path,
            &new_abs_path,
            is_dir,
            cx.clone(),
        )
        .await;
        let response = Worktree::handle_rename_entry(worktree, envelope.payload, cx.clone()).await;
        this.read_with(&mut cx, |this, _| {
            this.did_rename_entry(worktree_id, &old_abs_path, &new_abs_path, is_dir);
        })
        .ok();
        response
    }

    async fn handle_update_diagnostic_summary(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateDiagnosticSummary>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            if let Some(message) = envelope.payload.summary {
                let project_path = ProjectPath {
                    worktree_id,
                    path: Arc::<Path>::from_proto(message.path),
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
                if let Some((downstream_client, project_id)) = &this.downstream_client {
                    downstream_client
                        .send(proto::UpdateDiagnosticSummary {
                            project_id: *project_id,
                            worktree_id: worktree_id.to_proto(),
                            summary: Some(proto::DiagnosticSummary {
                                path: project_path.path.as_ref().to_proto(),
                                language_server_id: server_id.0 as u64,
                                error_count: summary.error_count as u32,
                                warning_count: summary.warning_count as u32,
                            }),
                        })
                        .log_err();
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
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::StartLanguageServer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let server = envelope.payload.server.context("invalid server")?;
        let server_capabilities =
            serde_json::from_str::<lsp::ServerCapabilities>(&envelope.payload.capabilities)
                .with_context(|| {
                    format!(
                        "incorrect server capabilities {}",
                        envelope.payload.capabilities
                    )
                })?;
        lsp_store.update(&mut cx, |lsp_store, cx| {
            let server_id = LanguageServerId(server.id as usize);
            let server_name = LanguageServerName::from_proto(server.name.clone());
            lsp_store
                .lsp_server_capabilities
                .insert(server_id, server_capabilities);
            lsp_store.language_server_statuses.insert(
                server_id,
                LanguageServerStatus {
                    name: server_name.clone(),
                    pending_work: Default::default(),
                    has_pending_diagnostic_updates: false,
                    progress_tokens: Default::default(),
                },
            );
            cx.emit(LspStoreEvent::LanguageServerAdded(
                server_id,
                server_name,
                server.worktree_id.map(WorktreeId::from_proto),
            ));
            cx.notify();
        })?;
        Ok(())
    }

    async fn handle_update_language_server(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateLanguageServer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        lsp_store.update(&mut cx, |lsp_store, cx| {
            let language_server_id = LanguageServerId(envelope.payload.language_server_id as usize);

            match envelope.payload.variant.context("invalid variant")? {
                proto::update_language_server::Variant::WorkStart(payload) => {
                    lsp_store.on_lsp_work_start(
                        language_server_id,
                        payload.token,
                        LanguageServerProgress {
                            title: payload.title,
                            is_disk_based_diagnostics_progress: false,
                            is_cancellable: payload.is_cancellable.unwrap_or(false),
                            message: payload.message,
                            percentage: payload.percentage.map(|p| p as usize),
                            last_update_at: cx.background_executor().now(),
                        },
                        cx,
                    );
                }
                proto::update_language_server::Variant::WorkProgress(payload) => {
                    lsp_store.on_lsp_work_progress(
                        language_server_id,
                        payload.token,
                        LanguageServerProgress {
                            title: None,
                            is_disk_based_diagnostics_progress: false,
                            is_cancellable: payload.is_cancellable.unwrap_or(false),
                            message: payload.message,
                            percentage: payload.percentage.map(|p| p as usize),
                            last_update_at: cx.background_executor().now(),
                        },
                        cx,
                    );
                }

                proto::update_language_server::Variant::WorkEnd(payload) => {
                    lsp_store.on_lsp_work_end(language_server_id, payload.token, cx);
                }

                proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(_) => {
                    lsp_store.disk_based_diagnostics_started(language_server_id, cx);
                }

                proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(_) => {
                    lsp_store.disk_based_diagnostics_finished(language_server_id, cx)
                }

                non_lsp @ proto::update_language_server::Variant::StatusUpdate(_)
                | non_lsp @ proto::update_language_server::Variant::RegisteredForBuffer(_)
                | non_lsp @ proto::update_language_server::Variant::MetadataUpdated(_) => {
                    cx.emit(LspStoreEvent::LanguageServerUpdate {
                        language_server_id,
                        name: envelope
                            .payload
                            .server_name
                            .map(SharedString::new)
                            .map(LanguageServerName),
                        message: non_lsp,
                    });
                }
            }

            Ok(())
        })?
    }

    async fn handle_language_server_log(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::LanguageServerLog>,
        mut cx: AsyncApp,
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

    async fn handle_lsp_ext_cancel_flycheck(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::LspExtCancelFlycheck>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let server_id = LanguageServerId(envelope.payload.language_server_id as usize);
        lsp_store.read_with(&mut cx, |lsp_store, _| {
            if let Some(server) = lsp_store.language_server_for_id(server_id) {
                server
                    .notify::<lsp_store::lsp_ext_command::LspExtCancelFlycheck>(&())
                    .context("handling lsp ext cancel flycheck")
            } else {
                anyhow::Ok(())
            }
        })??;

        Ok(proto::Ack {})
    }

    async fn handle_lsp_ext_run_flycheck(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::LspExtRunFlycheck>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let server_id = LanguageServerId(envelope.payload.language_server_id as usize);
        lsp_store.update(&mut cx, |lsp_store, cx| {
            if let Some(server) = lsp_store.language_server_for_id(server_id) {
                let text_document = if envelope.payload.current_file_only {
                    let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
                    lsp_store
                        .buffer_store()
                        .read(cx)
                        .get(buffer_id)
                        .and_then(|buffer| Some(buffer.read(cx).file()?.as_local()?.abs_path(cx)))
                        .map(|path| make_text_document_identifier(&path))
                        .transpose()?
                } else {
                    None
                };
                server
                    .notify::<lsp_store::lsp_ext_command::LspExtRunFlycheck>(
                        &lsp_store::lsp_ext_command::RunFlycheckParams { text_document },
                    )
                    .context("handling lsp ext run flycheck")
            } else {
                anyhow::Ok(())
            }
        })??;

        Ok(proto::Ack {})
    }

    async fn handle_lsp_ext_clear_flycheck(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::LspExtClearFlycheck>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let server_id = LanguageServerId(envelope.payload.language_server_id as usize);
        lsp_store.read_with(&mut cx, |lsp_store, _| {
            if let Some(server) = lsp_store.language_server_for_id(server_id) {
                server
                    .notify::<lsp_store::lsp_ext_command::LspExtClearFlycheck>(&())
                    .context("handling lsp ext clear flycheck")
            } else {
                anyhow::Ok(())
            }
        })??;

        Ok(proto::Ack {})
    }

    pub fn disk_based_diagnostics_started(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) {
        if let Some(language_server_status) =
            self.language_server_statuses.get_mut(&language_server_id)
        {
            language_server_status.has_pending_diagnostic_updates = true;
        }

        cx.emit(LspStoreEvent::DiskBasedDiagnosticsStarted { language_server_id });
        cx.emit(LspStoreEvent::LanguageServerUpdate {
            language_server_id,
            name: self
                .language_server_adapter_for_id(language_server_id)
                .map(|adapter| adapter.name()),
            message: proto::update_language_server::Variant::DiskBasedDiagnosticsUpdating(
                Default::default(),
            ),
        })
    }

    pub fn disk_based_diagnostics_finished(
        &mut self,
        language_server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) {
        if let Some(language_server_status) =
            self.language_server_statuses.get_mut(&language_server_id)
        {
            language_server_status.has_pending_diagnostic_updates = false;
        }

        cx.emit(LspStoreEvent::DiskBasedDiagnosticsFinished { language_server_id });
        cx.emit(LspStoreEvent::LanguageServerUpdate {
            language_server_id,
            name: self
                .language_server_adapter_for_id(language_server_id)
                .map(|adapter| adapter.name()),
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
        cx: &mut Context<Self>,
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

        let prev_task =
            simulate_disk_based_diagnostics_completion.replace(cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(DISK_BASED_DIAGNOSTICS_DEBOUNCE)
                    .await;

                this.update(cx, |this, cx| {
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
            }));

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

    pub(super) fn did_rename_entry(
        &self,
        worktree_id: WorktreeId,
        old_path: &Path,
        new_path: &Path,
        is_dir: bool,
    ) {
        maybe!({
            let local_store = self.as_local()?;

            let old_uri = lsp::Url::from_file_path(old_path).ok().map(String::from)?;
            let new_uri = lsp::Url::from_file_path(new_path).ok().map(String::from)?;

            for language_server in local_store.language_servers_for_worktree(worktree_id) {
                let Some(filter) = local_store
                    .language_server_paths_watched_for_rename
                    .get(&language_server.server_id())
                else {
                    continue;
                };

                if filter.should_send_did_rename(&old_uri, is_dir) {
                    language_server
                        .notify::<DidRenameFiles>(&RenameFilesParams {
                            files: vec![FileRename {
                                old_uri: old_uri.clone(),
                                new_uri: new_uri.clone(),
                            }],
                        })
                        .ok();
                }
            }
            Some(())
        });
    }

    pub(super) fn will_rename_entry(
        this: WeakEntity<Self>,
        worktree_id: WorktreeId,
        old_path: &Path,
        new_path: &Path,
        is_dir: bool,
        cx: AsyncApp,
    ) -> Task<()> {
        let old_uri = lsp::Url::from_file_path(old_path).ok().map(String::from);
        let new_uri = lsp::Url::from_file_path(new_path).ok().map(String::from);
        cx.spawn(async move |cx| {
            let mut tasks = vec![];
            this.update(cx, |this, cx| {
                let local_store = this.as_local()?;
                let old_uri = old_uri?;
                let new_uri = new_uri?;
                for language_server in local_store.language_servers_for_worktree(worktree_id) {
                    let Some(filter) = local_store
                        .language_server_paths_watched_for_rename
                        .get(&language_server.server_id())
                    else {
                        continue;
                    };
                    let Some(adapter) =
                        this.language_server_adapter_for_id(language_server.server_id())
                    else {
                        continue;
                    };
                    if filter.should_send_will_rename(&old_uri, is_dir) {
                        let apply_edit = cx.spawn({
                            let old_uri = old_uri.clone();
                            let new_uri = new_uri.clone();
                            let language_server = language_server.clone();
                            async move |this, cx| {
                                let edit = language_server
                                    .request::<WillRenameFiles>(RenameFilesParams {
                                        files: vec![FileRename { old_uri, new_uri }],
                                    })
                                    .await
                                    .into_response()
                                    .context("will rename files")
                                    .log_err()
                                    .flatten()?;

                                LocalLspStore::deserialize_workspace_edit(
                                    this.upgrade()?,
                                    edit,
                                    false,
                                    adapter.clone(),
                                    language_server.clone(),
                                    cx,
                                )
                                .await
                                .ok();
                                Some(())
                            }
                        });
                        tasks.push(apply_edit);
                    }
                }
                Some(())
            })
            .ok()
            .flatten();
            for task in tasks {
                // Await on tasks sequentially so that the order of application of edits is deterministic
                // (at least with regards to the order of registration of language servers)
                task.await;
            }
        })
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
                        uri: file_path_to_lsp_url(&event.path).log_err()?,
                        typ,
                    })
                })
                .collect::<Vec<_>>();
            if !changes.is_empty() {
                server
                    .notify::<lsp::notification::DidChangeWatchedFiles>(
                        &lsp::DidChangeWatchedFilesParams { changes },
                    )
                    .ok();
            }
            Some(())
        });
    }

    pub fn language_server_for_id(&self, id: LanguageServerId) -> Option<Arc<LanguageServer>> {
        let local_lsp_store = self.as_local()?;
        if let Some(LanguageServerState::Running { server, .. }) =
            local_lsp_store.language_servers.get(&id)
        {
            Some(server.clone())
        } else if let Some((_, server)) = local_lsp_store.supplementary_language_servers.get(&id) {
            Some(Arc::clone(server))
        } else {
            None
        }
    }

    fn on_lsp_progress(
        &mut self,
        progress: lsp::ProgressParams,
        language_server_id: LanguageServerId,
        disk_based_diagnostics_progress_token: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let token = match progress.token {
            lsp::NumberOrString::String(token) => token,
            lsp::NumberOrString::Number(token) => {
                log::info!("skipping numeric progress token {}", token);
                return;
            }
        };

        match progress.value {
            lsp::ProgressParamsValue::WorkDone(progress) => {
                self.handle_work_done_progress(
                    progress,
                    language_server_id,
                    disk_based_diagnostics_progress_token,
                    token,
                    cx,
                );
            }
            lsp::ProgressParamsValue::WorkspaceDiagnostic(report) => {
                if let Some(LanguageServerState::Running {
                    workspace_refresh_task: Some(workspace_refresh_task),
                    ..
                }) = self
                    .as_local_mut()
                    .and_then(|local| local.language_servers.get_mut(&language_server_id))
                {
                    workspace_refresh_task.progress_tx.try_send(()).ok();
                    self.apply_workspace_diagnostic_report(language_server_id, report, cx)
                }
            }
        }
    }

    fn handle_work_done_progress(
        &mut self,
        progress: lsp::WorkDoneProgress,
        language_server_id: LanguageServerId,
        disk_based_diagnostics_progress_token: Option<String>,
        token: String,
        cx: &mut Context<Self>,
    ) {
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
            lsp::WorkDoneProgress::Report(report) => self.on_lsp_work_progress(
                language_server_id,
                token,
                LanguageServerProgress {
                    title: None,
                    is_disk_based_diagnostics_progress,
                    is_cancellable: report.cancellable.unwrap_or(false),
                    message: report.message,
                    percentage: report.percentage.map(|p| p as usize),
                    last_update_at: cx.background_executor().now(),
                },
                cx,
            ),
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
        cx: &mut Context<Self>,
    ) {
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            status.pending_work.insert(token.clone(), progress.clone());
            cx.notify();
        }
        cx.emit(LspStoreEvent::LanguageServerUpdate {
            language_server_id,
            name: self
                .language_server_adapter_for_id(language_server_id)
                .map(|adapter| adapter.name()),
            message: proto::update_language_server::Variant::WorkStart(proto::LspWorkStart {
                token,
                title: progress.title,
                message: progress.message,
                percentage: progress.percentage.map(|p| p as u32),
                is_cancellable: Some(progress.is_cancellable),
            }),
        })
    }

    fn on_lsp_work_progress(
        &mut self,
        language_server_id: LanguageServerId,
        token: String,
        progress: LanguageServerProgress,
        cx: &mut Context<Self>,
    ) {
        let mut did_update = false;
        if let Some(status) = self.language_server_statuses.get_mut(&language_server_id) {
            match status.pending_work.entry(token.clone()) {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(progress.clone());
                    did_update = true;
                }
                btree_map::Entry::Occupied(mut entry) => {
                    let entry = entry.get_mut();
                    if (progress.last_update_at - entry.last_update_at)
                        >= SERVER_PROGRESS_THROTTLE_TIMEOUT
                    {
                        entry.last_update_at = progress.last_update_at;
                        if progress.message.is_some() {
                            entry.message = progress.message.clone();
                        }
                        if progress.percentage.is_some() {
                            entry.percentage = progress.percentage;
                        }
                        if progress.is_cancellable != entry.is_cancellable {
                            entry.is_cancellable = progress.is_cancellable;
                        }
                        did_update = true;
                    }
                }
            }
        }

        if did_update {
            cx.emit(LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name: self
                    .language_server_adapter_for_id(language_server_id)
                    .map(|adapter| adapter.name()),
                message: proto::update_language_server::Variant::WorkProgress(
                    proto::LspWorkProgress {
                        token,
                        message: progress.message,
                        percentage: progress.percentage.map(|p| p as u32),
                        is_cancellable: Some(progress.is_cancellable),
                    },
                ),
            })
        }
    }

    fn on_lsp_work_end(
        &mut self,
        language_server_id: LanguageServerId,
        token: String,
        cx: &mut Context<Self>,
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
            name: self
                .language_server_adapter_for_id(language_server_id)
                .map(|adapter| adapter.name()),
            message: proto::update_language_server::Variant::WorkEnd(proto::LspWorkEnd { token }),
        })
    }

    pub async fn handle_resolve_completion_documentation(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ResolveCompletionDocumentation>,
        mut cx: AsyncApp,
    ) -> Result<proto::ResolveCompletionDocumentationResponse> {
        let lsp_completion = serde_json::from_slice(&envelope.payload.lsp_completion)?;

        let completion = this
            .read_with(&cx, |this, cx| {
                let id = LanguageServerId(envelope.payload.language_server_id as usize);
                let server = this
                    .language_server_for_id(id)
                    .with_context(|| format!("No language server {id}"))?;

                anyhow::Ok(cx.background_spawn(async move {
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
                            .into_response()
                            .context("resolve completion item")
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
        let mut old_replace_start = None;
        let mut old_replace_end = None;
        let mut old_insert_start = None;
        let mut old_insert_end = None;
        let mut new_text = String::default();
        if let Ok(buffer_id) = BufferId::new(envelope.payload.buffer_id) {
            let buffer_snapshot = this.update(&mut cx, |this, cx| {
                let buffer = this.buffer_store.read(cx).get_existing(buffer_id)?;
                anyhow::Ok(buffer.read(cx).snapshot())
            })??;

            if let Some(text_edit) = completion.text_edit.as_ref() {
                let edit = parse_completion_text_edit(text_edit, &buffer_snapshot);

                if let Some(mut edit) = edit {
                    LineEnding::normalize(&mut edit.new_text);

                    new_text = edit.new_text;
                    old_replace_start = Some(serialize_anchor(&edit.replace_range.start));
                    old_replace_end = Some(serialize_anchor(&edit.replace_range.end));
                    if let Some(insert_range) = edit.insert_range {
                        old_insert_start = Some(serialize_anchor(&insert_range.start));
                        old_insert_end = Some(serialize_anchor(&insert_range.end));
                    }
                }
            }
        }

        Ok(proto::ResolveCompletionDocumentationResponse {
            documentation,
            documentation_is_markdown,
            old_replace_start,
            old_replace_end,
            new_text,
            lsp_completion,
            old_insert_start,
            old_insert_end,
        })
    }

    async fn handle_on_type_formatting(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OnTypeFormatting>,
        mut cx: AsyncApp,
    ) -> Result<proto::OnTypeFormattingResponse> {
        let on_type_formatting = this.update(&mut cx, |this, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            let buffer = this.buffer_store.read(cx).get_existing(buffer_id)?;
            let position = envelope
                .payload
                .position
                .and_then(deserialize_anchor)
                .context("invalid position")?;
            anyhow::Ok(this.apply_on_type_formatting(
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
        this: Entity<Self>,
        _: TypedEnvelope<proto::RefreshInlayHints>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |_, cx| {
            cx.emit(LspStoreEvent::RefreshInlayHints);
        })?;
        Ok(proto::Ack {})
    }

    async fn handle_pull_workspace_diagnostics(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::PullWorkspaceDiagnostics>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let server_id = LanguageServerId::from_proto(envelope.payload.server_id);
        lsp_store.update(&mut cx, |lsp_store, _| {
            lsp_store.pull_workspace_diagnostics(server_id);
        })?;
        Ok(proto::Ack {})
    }

    async fn handle_inlay_hints(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::InlayHints>,
        mut cx: AsyncApp,
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

    async fn handle_get_color_presentation(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::GetColorPresentation>,
        mut cx: AsyncApp,
    ) -> Result<proto::GetColorPresentationResponse> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let buffer = lsp_store.update(&mut cx, |lsp_store, cx| {
            lsp_store.buffer_store.read(cx).get_existing(buffer_id)
        })??;

        let color = envelope
            .payload
            .color
            .context("invalid color resolve request")?;
        let start = color
            .lsp_range_start
            .context("invalid color resolve request")?;
        let end = color
            .lsp_range_end
            .context("invalid color resolve request")?;

        let color = DocumentColor {
            lsp_range: lsp::Range {
                start: point_to_lsp(PointUtf16::new(start.row, start.column)),
                end: point_to_lsp(PointUtf16::new(end.row, end.column)),
            },
            color: lsp::Color {
                red: color.red,
                green: color.green,
                blue: color.blue,
                alpha: color.alpha,
            },
            resolved: false,
            color_presentations: Vec::new(),
        };
        let resolved_color = lsp_store
            .update(&mut cx, |lsp_store, cx| {
                lsp_store.resolve_color_presentation(
                    color,
                    buffer.clone(),
                    LanguageServerId(envelope.payload.server_id as usize),
                    cx,
                )
            })?
            .await
            .context("resolving color presentation")?;

        Ok(proto::GetColorPresentationResponse {
            presentations: resolved_color
                .color_presentations
                .into_iter()
                .map(|presentation| proto::ColorPresentation {
                    label: presentation.label.to_string(),
                    text_edit: presentation.text_edit.map(serialize_lsp_edit),
                    additional_text_edits: presentation
                        .additional_text_edits
                        .into_iter()
                        .map(serialize_lsp_edit)
                        .collect(),
                })
                .collect(),
        })
    }

    async fn handle_resolve_inlay_hint(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ResolveInlayHint>,
        mut cx: AsyncApp,
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

    async fn handle_refresh_code_lens(
        this: Entity<Self>,
        _: TypedEnvelope<proto::RefreshCodeLens>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |_, cx| {
            cx.emit(LspStoreEvent::RefreshCodeLens);
        })?;
        Ok(proto::Ack {})
    }

    async fn handle_open_buffer_for_symbol(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenBufferForSymbol>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferForSymbolResponse> {
        let peer_id = envelope.original_sender_id().unwrap_or_default();
        let symbol = envelope.payload.symbol.context("invalid symbol")?;
        let symbol = Self::deserialize_symbol(symbol)?;
        let symbol = this.read_with(&mut cx, |this, _| {
            let signature = this.symbol_signature(&symbol.path);
            anyhow::ensure!(signature == symbol.signature, "invalid symbol signature");
            Ok(symbol)
        })??;
        let buffer = this
            .update(&mut cx, |this, cx| {
                this.open_buffer_for_symbol(
                    &Symbol {
                        language_server_name: symbol.language_server_name,
                        source_worktree_id: symbol.source_worktree_id,
                        source_language_server_id: symbol.source_language_server_id,
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
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetProjectSymbols>,
        mut cx: AsyncApp,
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
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RestartLanguageServers>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |lsp_store, cx| {
            let buffers =
                lsp_store.buffer_ids_to_buffers(envelope.payload.buffer_ids.into_iter(), cx);
            lsp_store.restart_language_servers_for_buffers(
                buffers,
                envelope
                    .payload
                    .only_servers
                    .into_iter()
                    .filter_map(|selector| {
                        Some(match selector.selector? {
                            proto::language_server_selector::Selector::ServerId(server_id) => {
                                LanguageServerSelector::Id(LanguageServerId::from_proto(server_id))
                            }
                            proto::language_server_selector::Selector::Name(name) => {
                                LanguageServerSelector::Name(LanguageServerName(
                                    SharedString::from(name),
                                ))
                            }
                        })
                    })
                    .collect(),
                cx,
            );
        })?;

        Ok(proto::Ack {})
    }

    pub async fn handle_stop_language_servers(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::StopLanguageServers>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        lsp_store.update(&mut cx, |lsp_store, cx| {
            if envelope.payload.all
                && envelope.payload.also_servers.is_empty()
                && envelope.payload.buffer_ids.is_empty()
            {
                lsp_store.stop_all_language_servers(cx);
            } else {
                let buffers =
                    lsp_store.buffer_ids_to_buffers(envelope.payload.buffer_ids.into_iter(), cx);
                lsp_store
                    .stop_language_servers_for_buffers(
                        buffers,
                        envelope
                            .payload
                            .also_servers
                            .into_iter()
                            .filter_map(|selector| {
                                Some(match selector.selector? {
                                    proto::language_server_selector::Selector::ServerId(
                                        server_id,
                                    ) => LanguageServerSelector::Id(LanguageServerId::from_proto(
                                        server_id,
                                    )),
                                    proto::language_server_selector::Selector::Name(name) => {
                                        LanguageServerSelector::Name(LanguageServerName(
                                            SharedString::from(name),
                                        ))
                                    }
                                })
                            })
                            .collect(),
                        cx,
                    )
                    .detach_and_log_err(cx);
            }
        })?;

        Ok(proto::Ack {})
    }

    pub async fn handle_cancel_language_server_work(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CancelLanguageServerWork>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |this, cx| {
            if let Some(work) = envelope.payload.work {
                match work {
                    proto::cancel_language_server_work::Work::Buffers(buffers) => {
                        let buffers =
                            this.buffer_ids_to_buffers(buffers.buffer_ids.into_iter(), cx);
                        this.cancel_language_server_work_for_buffers(buffers, cx);
                    }
                    proto::cancel_language_server_work::Work::LanguageServerWork(work) => {
                        let server_id = LanguageServerId::from_proto(work.language_server_id);
                        this.cancel_language_server_work(server_id, work.token, cx);
                    }
                }
            }
        })?;

        Ok(proto::Ack {})
    }

    fn buffer_ids_to_buffers(
        &mut self,
        buffer_ids: impl Iterator<Item = u64>,
        cx: &mut Context<Self>,
    ) -> Vec<Entity<Buffer>> {
        buffer_ids
            .into_iter()
            .flat_map(|buffer_id| {
                self.buffer_store
                    .read(cx)
                    .get(BufferId::new(buffer_id).log_err()?)
            })
            .collect::<Vec<_>>()
    }

    async fn handle_apply_additional_edits_for_completion(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ApplyCompletionAdditionalEdits>,
        mut cx: AsyncApp,
    ) -> Result<proto::ApplyCompletionAdditionalEditsResponse> {
        let (buffer, completion) = this.update(&mut cx, |this, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            let buffer = this.buffer_store.read(cx).get_existing(buffer_id)?;
            let completion = Self::deserialize_completion(
                envelope.payload.completion.context("invalid completion")?,
            )?;
            anyhow::Ok((buffer, completion))
        })??;

        let apply_additional_edits = this.update(&mut cx, |this, cx| {
            this.apply_additional_edits_for_completion(
                buffer,
                Rc::new(RefCell::new(Box::new([Completion {
                    replace_range: completion.replace_range,
                    new_text: completion.new_text,
                    source: completion.source,
                    documentation: None,
                    label: CodeLabel {
                        text: Default::default(),
                        runs: Default::default(),
                        filter_range: Default::default(),
                    },
                    insert_text_mode: None,
                    icon_path: None,
                    confirm: None,
                }]))),
                0,
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
        self.last_formatting_failure.as_deref()
    }

    pub fn reset_last_formatting_failure(&mut self) {
        self.last_formatting_failure = None;
    }

    pub fn environment_for_buffer(
        &self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Shared<Task<Option<HashMap<String, String>>>> {
        if let Some(environment) = &self.as_local().map(|local| local.environment.clone()) {
            environment.update(cx, |env, cx| {
                env.get_buffer_environment(&buffer, &self.worktree_store, cx)
            })
        } else {
            Task::ready(None).shared()
        }
    }

    pub fn format(
        &mut self,
        buffers: HashSet<Entity<Buffer>>,
        target: LspFormatTarget,
        push_to_history: bool,
        trigger: FormatTrigger,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<ProjectTransaction>> {
        let logger = zlog::scoped!("format");
        if let Some(_) = self.as_local() {
            zlog::trace!(logger => "Formatting locally");
            let logger = zlog::scoped!(logger => "local");
            let buffers = buffers
                .into_iter()
                .map(|buffer_handle| {
                    let buffer = buffer_handle.read(cx);
                    let buffer_abs_path = File::from_dyn(buffer.file())
                        .and_then(|file| file.as_local().map(|f| f.abs_path(cx)));

                    (buffer_handle, buffer_abs_path, buffer.remote_id())
                })
                .collect::<Vec<_>>();

            cx.spawn(async move |lsp_store, cx| {
                let mut formattable_buffers = Vec::with_capacity(buffers.len());

                for (handle, abs_path, id) in buffers {
                    let env = lsp_store
                        .update(cx, |lsp_store, cx| {
                            lsp_store.environment_for_buffer(&handle, cx)
                        })?
                        .await;

                    let ranges = match &target {
                        LspFormatTarget::Buffers => None,
                        LspFormatTarget::Ranges(ranges) => {
                            Some(ranges.get(&id).context("No format ranges provided for buffer")?.clone())
                        }
                    };

                    formattable_buffers.push(FormattableBuffer {
                        handle,
                        abs_path,
                        env,
                        ranges,
                    });
                }
                zlog::trace!(logger => "Formatting {:?} buffers", formattable_buffers.len());

                let format_timer = zlog::time!(logger => "Formatting buffers");
                let result = LocalLspStore::format_locally(
                    lsp_store.clone(),
                    formattable_buffers,
                    push_to_history,
                    trigger,
                    logger,
                    cx,
                )
                .await;
                format_timer.end();

                zlog::trace!(logger => "Formatting completed with result {:?}", result.as_ref().map(|_| "<project-transaction>"));

                lsp_store.update(cx, |lsp_store, _| {
                    lsp_store.update_last_formatting_failure(&result);
                })?;

                result
            })
        } else if let Some((client, project_id)) = self.upstream_client() {
            zlog::trace!(logger => "Formatting remotely");
            let logger = zlog::scoped!(logger => "remote");
            // Don't support formatting ranges via remote
            match target {
                LspFormatTarget::Buffers => {}
                LspFormatTarget::Ranges(_) => {
                    zlog::trace!(logger => "Ignoring unsupported remote range formatting request");
                    return Task::ready(Ok(ProjectTransaction::default()));
                }
            }

            let buffer_store = self.buffer_store();
            cx.spawn(async move |lsp_store, cx| {
                zlog::trace!(logger => "Sending remote format request");
                let request_timer = zlog::time!(logger => "remote format request");
                let result = client
                    .request(proto::FormatBuffers {
                        project_id,
                        trigger: trigger as i32,
                        buffer_ids: buffers
                            .iter()
                            .map(|buffer| buffer.read_with(cx, |buffer, _| buffer.remote_id().into()))
                            .collect::<Result<_>>()?,
                    })
                    .await
                    .and_then(|result| result.transaction.context("missing transaction"));
                request_timer.end();

                zlog::trace!(logger => "Remote format request resolved to {:?}", result.as_ref().map(|_| "<project_transaction>"));

                lsp_store.update(cx, |lsp_store, _| {
                    lsp_store.update_last_formatting_failure(&result);
                })?;

                let transaction_response = result?;
                let _timer = zlog::time!(logger => "deserializing project transaction");
                buffer_store
                    .update(cx, |buffer_store, cx| {
                        buffer_store.deserialize_project_transaction(
                            transaction_response,
                            push_to_history,
                            cx,
                        )
                    })?
                    .await
            })
        } else {
            zlog::trace!(logger => "Not formatting");
            Task::ready(Ok(ProjectTransaction::default()))
        }
    }

    async fn handle_format_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FormatBuffers>,
        mut cx: AsyncApp,
    ) -> Result<proto::FormatBuffersResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let format = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(this.buffer_store.read(cx).get_existing(buffer_id)?);
            }
            let trigger = FormatTrigger::from_proto(envelope.payload.trigger);
            anyhow::Ok(this.format(buffers, LspFormatTarget::Buffers, false, trigger, cx))
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

    async fn handle_apply_code_action_kind(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeActionKind>,
        mut cx: AsyncApp,
    ) -> Result<proto::ApplyCodeActionKindResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let format = this.update(&mut cx, |this, cx| {
            let mut buffers = HashSet::default();
            for buffer_id in &envelope.payload.buffer_ids {
                let buffer_id = BufferId::new(*buffer_id)?;
                buffers.insert(this.buffer_store.read(cx).get_existing(buffer_id)?);
            }
            let kind = match envelope.payload.kind.as_str() {
                "" => CodeActionKind::EMPTY,
                "quickfix" => CodeActionKind::QUICKFIX,
                "refactor" => CodeActionKind::REFACTOR,
                "refactor.extract" => CodeActionKind::REFACTOR_EXTRACT,
                "refactor.inline" => CodeActionKind::REFACTOR_INLINE,
                "refactor.rewrite" => CodeActionKind::REFACTOR_REWRITE,
                "source" => CodeActionKind::SOURCE,
                "source.organizeImports" => CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
                "source.fixAll" => CodeActionKind::SOURCE_FIX_ALL,
                _ => anyhow::bail!(
                    "Invalid code action kind {}",
                    envelope.payload.kind.as_str()
                ),
            };
            anyhow::Ok(this.apply_code_action_kind(buffers, kind, false, cx))
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
        Ok(proto::ApplyCodeActionKindResponse {
            transaction: Some(project_transaction),
        })
    }

    async fn shutdown_language_server(
        server_state: Option<LanguageServerState>,
        name: LanguageServerName,
        cx: &mut AsyncApp,
    ) {
        let server = match server_state {
            Some(LanguageServerState::Starting { startup, .. }) => {
                let mut timer = cx
                    .background_executor()
                    .timer(SERVER_LAUNCHING_BEFORE_SHUTDOWN_TIMEOUT)
                    .fuse();

                select! {
                    server = startup.fuse() => server,
                    () = timer => {
                        log::info!("timeout waiting for language server {name} to finish launching before stopping");
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
        server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) -> Task<Vec<WorktreeId>> {
        let local = match &mut self.mode {
            LspStoreMode::Local(local) => local,
            _ => {
                return Task::ready(Vec::new());
            }
        };

        let mut orphaned_worktrees = Vec::new();
        // Remove this server ID from all entries in the given worktree.
        local.language_server_ids.retain(|(worktree, _), ids| {
            if !ids.remove(&server_id) {
                return true;
            }

            if ids.is_empty() {
                orphaned_worktrees.push(*worktree);
                false
            } else {
                true
            }
        });
        self.buffer_store.update(cx, |buffer_store, cx| {
            for buffer in buffer_store.buffers() {
                buffer.update(cx, |buffer, cx| {
                    buffer.update_diagnostics(server_id, DiagnosticSet::new([], buffer), cx);
                    buffer.set_completion_triggers(server_id, Default::default(), cx);
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
                                    path: path.as_ref().to_proto(),
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

        let local = self.as_local_mut().unwrap();
        for diagnostics in local.diagnostics.values_mut() {
            diagnostics.retain(|_, diagnostics_by_server_id| {
                if let Ok(ix) = diagnostics_by_server_id.binary_search_by_key(&server_id, |e| e.0) {
                    diagnostics_by_server_id.remove(ix);
                    !diagnostics_by_server_id.is_empty()
                } else {
                    true
                }
            });
        }
        local.language_server_watched_paths.remove(&server_id);

        let server_state = local.language_servers.remove(&server_id);
        self.cleanup_lsp_data(server_id);
        let name = self
            .language_server_statuses
            .remove(&server_id)
            .map(|status| status.name.clone())
            .or_else(|| {
                if let Some(LanguageServerState::Running { adapter, .. }) = server_state.as_ref() {
                    Some(adapter.name())
                } else {
                    None
                }
            });

        if let Some(name) = name {
            log::info!("stopping language server {name}");
            self.languages
                .update_lsp_binary_status(name.clone(), BinaryStatus::Stopping);
            cx.notify();

            return cx.spawn(async move |lsp_store, cx| {
                Self::shutdown_language_server(server_state, name.clone(), cx).await;
                lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store
                            .languages
                            .update_lsp_binary_status(name, BinaryStatus::Stopped);
                        cx.emit(LspStoreEvent::LanguageServerRemoved(server_id));
                        cx.notify();
                    })
                    .ok();
                orphaned_worktrees
            });
        }

        if server_state.is_some() {
            cx.emit(LspStoreEvent::LanguageServerRemoved(server_id));
        }
        Task::ready(orphaned_worktrees)
    }

    pub fn stop_all_language_servers(&mut self, cx: &mut Context<Self>) {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = client.request(proto::StopLanguageServers {
                project_id,
                buffer_ids: Vec::new(),
                also_servers: Vec::new(),
                all: true,
            });
            cx.background_spawn(request).detach_and_log_err(cx);
        } else {
            let Some(local) = self.as_local_mut() else {
                return;
            };
            let language_servers_to_stop = local
                .language_server_ids
                .values()
                .flatten()
                .copied()
                .collect();
            local.lsp_tree.update(cx, |this, _| {
                this.remove_nodes(&language_servers_to_stop);
            });
            let tasks = language_servers_to_stop
                .into_iter()
                .map(|server| self.stop_local_language_server(server, cx))
                .collect::<Vec<_>>();
            cx.background_spawn(async move {
                futures::future::join_all(tasks).await;
            })
            .detach();
        }
    }

    pub fn restart_language_servers_for_buffers(
        &mut self,
        buffers: Vec<Entity<Buffer>>,
        only_restart_servers: HashSet<LanguageServerSelector>,
        cx: &mut Context<Self>,
    ) {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = client.request(proto::RestartLanguageServers {
                project_id,
                buffer_ids: buffers
                    .into_iter()
                    .map(|b| b.read(cx).remote_id().to_proto())
                    .collect(),
                only_servers: only_restart_servers
                    .into_iter()
                    .map(|selector| {
                        let selector = match selector {
                            LanguageServerSelector::Id(language_server_id) => {
                                proto::language_server_selector::Selector::ServerId(
                                    language_server_id.to_proto(),
                                )
                            }
                            LanguageServerSelector::Name(language_server_name) => {
                                proto::language_server_selector::Selector::Name(
                                    language_server_name.to_string(),
                                )
                            }
                        };
                        proto::LanguageServerSelector {
                            selector: Some(selector),
                        }
                    })
                    .collect(),
                all: false,
            });
            cx.background_spawn(request).detach_and_log_err(cx);
        } else {
            let stop_task = if only_restart_servers.is_empty() {
                self.stop_local_language_servers_for_buffers(&buffers, HashSet::default(), cx)
            } else {
                self.stop_local_language_servers_for_buffers(&[], only_restart_servers.clone(), cx)
            };
            cx.spawn(async move |lsp_store, cx| {
                stop_task.await;
                lsp_store
                    .update(cx, |lsp_store, cx| {
                        for buffer in buffers {
                            lsp_store.register_buffer_with_language_servers(
                                &buffer,
                                only_restart_servers.clone(),
                                true,
                                cx,
                            );
                        }
                    })
                    .ok()
            })
            .detach();
        }
    }

    pub fn stop_language_servers_for_buffers(
        &mut self,
        buffers: Vec<Entity<Buffer>>,
        also_stop_servers: HashSet<LanguageServerSelector>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = client.request(proto::StopLanguageServers {
                project_id,
                buffer_ids: buffers
                    .into_iter()
                    .map(|b| b.read(cx).remote_id().to_proto())
                    .collect(),
                also_servers: also_stop_servers
                    .into_iter()
                    .map(|selector| {
                        let selector = match selector {
                            LanguageServerSelector::Id(language_server_id) => {
                                proto::language_server_selector::Selector::ServerId(
                                    language_server_id.to_proto(),
                                )
                            }
                            LanguageServerSelector::Name(language_server_name) => {
                                proto::language_server_selector::Selector::Name(
                                    language_server_name.to_string(),
                                )
                            }
                        };
                        proto::LanguageServerSelector {
                            selector: Some(selector),
                        }
                    })
                    .collect(),
                all: false,
            });
            cx.background_spawn(async move {
                let _ = request.await?;
                Ok(())
            })
        } else {
            let task =
                self.stop_local_language_servers_for_buffers(&buffers, also_stop_servers, cx);
            cx.background_spawn(async move {
                task.await;
                Ok(())
            })
        }
    }

    fn stop_local_language_servers_for_buffers(
        &mut self,
        buffers: &[Entity<Buffer>],
        also_stop_servers: HashSet<LanguageServerSelector>,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let Some(local) = self.as_local_mut() else {
            return Task::ready(());
        };
        let mut language_server_names_to_stop = BTreeSet::default();
        let mut language_servers_to_stop = also_stop_servers
            .into_iter()
            .flat_map(|selector| match selector {
                LanguageServerSelector::Id(id) => Some(id),
                LanguageServerSelector::Name(name) => {
                    language_server_names_to_stop.insert(name);
                    None
                }
            })
            .collect::<BTreeSet<_>>();

        let mut covered_worktrees = HashSet::default();
        for buffer in buffers {
            buffer.update(cx, |buffer, cx| {
                language_servers_to_stop.extend(local.language_server_ids_for_buffer(buffer, cx));
                if let Some(worktree_id) = buffer.file().map(|f| f.worktree_id(cx)) {
                    if covered_worktrees.insert(worktree_id) {
                        language_server_names_to_stop.retain(|name| {
                            match local.language_server_ids.get(&(worktree_id, name.clone())) {
                                Some(server_ids) => {
                                    language_servers_to_stop
                                        .extend(server_ids.into_iter().copied());
                                    false
                                }
                                None => true,
                            }
                        });
                    }
                }
            });
        }
        for name in language_server_names_to_stop {
            if let Some(server_ids) = local
                .language_server_ids
                .iter()
                .filter(|((_, server_name), _)| server_name == &name)
                .map(|((_, _), server_ids)| server_ids)
                .max_by_key(|server_ids| server_ids.len())
            {
                language_servers_to_stop.extend(server_ids.into_iter().copied());
            }
        }

        local.lsp_tree.update(cx, |this, _| {
            this.remove_nodes(&language_servers_to_stop);
        });
        let tasks = language_servers_to_stop
            .into_iter()
            .map(|server| self.stop_local_language_server(server, cx))
            .collect::<Vec<_>>();

        cx.background_spawn(futures::future::join_all(tasks).map(|_| ()))
    }

    fn get_buffer<'a>(&self, abs_path: &Path, cx: &'a App) -> Option<&'a Buffer> {
        let (worktree, relative_path) =
            self.worktree_store.read(cx).find_worktree(&abs_path, cx)?;

        let project_path = ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: relative_path.into(),
        };

        Some(
            self.buffer_store()
                .read(cx)
                .get_by_path(&project_path)?
                .read(cx),
        )
    }

    pub fn update_diagnostics(
        &mut self,
        language_server_id: LanguageServerId,
        params: lsp::PublishDiagnosticsParams,
        result_id: Option<String>,
        source_kind: DiagnosticSourceKind,
        disk_based_sources: &[String],
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.merge_diagnostics(
            language_server_id,
            params,
            result_id,
            source_kind,
            disk_based_sources,
            |_, _, _| false,
            cx,
        )
    }

    pub fn merge_diagnostics(
        &mut self,
        language_server_id: LanguageServerId,
        mut params: lsp::PublishDiagnosticsParams,
        result_id: Option<String>,
        source_kind: DiagnosticSourceKind,
        disk_based_sources: &[String],
        filter: impl Fn(&Buffer, &Diagnostic, &App) -> bool + Clone,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        anyhow::ensure!(self.mode.is_local(), "called update_diagnostics on remote");
        let abs_path = params
            .uri
            .to_file_path()
            .map_err(|()| anyhow!("URI is not a file"))?;
        let mut diagnostics = Vec::default();
        let mut primary_diagnostic_group_ids = HashMap::default();
        let mut sources_by_group_id = HashMap::default();
        let mut supporting_diagnostics = HashMap::default();

        let adapter = self.language_server_adapter_for_id(language_server_id);

        // Ensure that primary diagnostics are always the most severe
        params.diagnostics.sort_by_key(|item| item.severity);

        for diagnostic in &params.diagnostics {
            let source = diagnostic.source.as_ref();
            let range = range_from_lsp(diagnostic.range);
            let is_supporting = diagnostic
                .related_information
                .as_ref()
                .map_or(false, |infos| {
                    infos.iter().any(|info| {
                        primary_diagnostic_group_ids.contains_key(&(
                            source,
                            diagnostic.code.clone(),
                            range_from_lsp(info.location.range),
                        ))
                    })
                });

            let is_unnecessary = diagnostic
                .tags
                .as_ref()
                .map_or(false, |tags| tags.contains(&DiagnosticTag::UNNECESSARY));

            let underline = self
                .language_server_adapter_for_id(language_server_id)
                .map_or(true, |adapter| adapter.underline_diagnostic(diagnostic));

            if is_supporting {
                supporting_diagnostics.insert(
                    (source, diagnostic.code.clone(), range),
                    (diagnostic.severity, is_unnecessary),
                );
            } else {
                let group_id = post_inc(&mut self.as_local_mut().unwrap().next_diagnostic_group_id);
                let is_disk_based =
                    source.map_or(false, |source| disk_based_sources.contains(source));

                sources_by_group_id.insert(group_id, source);
                primary_diagnostic_group_ids
                    .insert((source, diagnostic.code.clone(), range.clone()), group_id);

                diagnostics.push(DiagnosticEntry {
                    range,
                    diagnostic: Diagnostic {
                        source: diagnostic.source.clone(),
                        source_kind,
                        code: diagnostic.code.clone(),
                        code_description: diagnostic
                            .code_description
                            .as_ref()
                            .and_then(|d| d.href.clone()),
                        severity: diagnostic.severity.unwrap_or(DiagnosticSeverity::ERROR),
                        markdown: adapter.as_ref().and_then(|adapter| {
                            adapter.diagnostic_message_to_markdown(&diagnostic.message)
                        }),
                        message: diagnostic.message.trim().to_string(),
                        group_id,
                        is_primary: true,
                        is_disk_based,
                        is_unnecessary,
                        underline,
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
                                    source_kind,
                                    code: diagnostic.code.clone(),
                                    code_description: diagnostic
                                        .code_description
                                        .as_ref()
                                        .and_then(|d| d.href.clone()),
                                    severity: DiagnosticSeverity::INFORMATION,
                                    markdown: adapter.as_ref().and_then(|adapter| {
                                        adapter.diagnostic_message_to_markdown(&info.message)
                                    }),
                                    message: info.message.trim().to_string(),
                                    group_id,
                                    is_primary: false,
                                    is_disk_based,
                                    is_unnecessary: false,
                                    underline,
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

        self.merge_diagnostic_entries(
            language_server_id,
            abs_path,
            result_id,
            params.version,
            diagnostics,
            filter,
            cx,
        )?;
        Ok(())
    }

    fn insert_newly_running_language_server(
        &mut self,
        adapter: Arc<CachedLspAdapter>,
        language_server: Arc<LanguageServer>,
        server_id: LanguageServerId,
        key: (WorktreeId, LanguageServerName),
        workspace_folders: Arc<Mutex<BTreeSet<Url>>>,
        cx: &mut Context<Self>,
    ) {
        let Some(local) = self.as_local_mut() else {
            return;
        };
        // If the language server for this key doesn't match the server id, don't store the
        // server. Which will cause it to be dropped, killing the process
        if local
            .language_server_ids
            .get(&key)
            .map(|ids| !ids.contains(&server_id))
            .unwrap_or(false)
        {
            return;
        }

        // Update language_servers collection with Running variant of LanguageServerState
        // indicating that the server is up and running and ready
        let workspace_folders = workspace_folders.lock().clone();
        language_server.set_workspace_folders(workspace_folders);

        local.language_servers.insert(
            server_id,
            LanguageServerState::Running {
                workspace_refresh_task: lsp_workspace_diagnostics_refresh(
                    language_server.clone(),
                    cx,
                ),
                adapter: adapter.clone(),
                server: language_server.clone(),
                simulate_disk_based_diagnostics_completion: None,
            },
        );
        local
            .languages
            .update_lsp_binary_status(adapter.name(), BinaryStatus::None);
        if let Some(file_ops_caps) = language_server
            .capabilities()
            .workspace
            .as_ref()
            .and_then(|ws| ws.file_operations.as_ref())
        {
            let did_rename_caps = file_ops_caps.did_rename.as_ref();
            let will_rename_caps = file_ops_caps.will_rename.as_ref();
            if did_rename_caps.or(will_rename_caps).is_some() {
                let watcher = RenamePathsWatchedForServer::default()
                    .with_did_rename_patterns(did_rename_caps)
                    .with_will_rename_patterns(will_rename_caps);
                local
                    .language_server_paths_watched_for_rename
                    .insert(server_id, watcher);
            }
        }

        self.language_server_statuses.insert(
            server_id,
            LanguageServerStatus {
                name: language_server.name(),
                pending_work: Default::default(),
                has_pending_diagnostic_updates: false,
                progress_tokens: Default::default(),
            },
        );

        cx.emit(LspStoreEvent::LanguageServerAdded(
            server_id,
            language_server.name(),
            Some(key.0),
        ));
        cx.emit(LspStoreEvent::RefreshInlayHints);

        let server_capabilities = language_server.capabilities();
        if let Some((downstream_client, project_id)) = self.downstream_client.as_ref() {
            downstream_client
                .send(proto::StartLanguageServer {
                    project_id: *project_id,
                    server: Some(proto::LanguageServer {
                        id: server_id.to_proto(),
                        name: language_server.name().to_string(),
                        worktree_id: Some(key.0.to_proto()),
                    }),
                    capabilities: serde_json::to_string(&server_capabilities)
                        .expect("serializing server LSP capabilities"),
                })
                .log_err();
        }
        self.lsp_server_capabilities
            .insert(server_id, server_capabilities);

        // Tell the language server about every open buffer in the worktree that matches the language.
        // Also check for buffers in worktrees that reused this server
        let mut worktrees_using_server = vec![key.0];
        if let Some(local) = self.as_local() {
            // Find all worktrees that have this server in their language server tree
            for (worktree_id, servers) in &local.lsp_tree.read(cx).instances {
                if *worktree_id != key.0 {
                    for (_, server_map) in &servers.roots {
                        if server_map.contains_key(&key.1) {
                            worktrees_using_server.push(*worktree_id);
                        }
                    }
                }
            }
        }

        let mut buffer_paths_registered = Vec::new();
        self.buffer_store.clone().update(cx, |buffer_store, cx| {
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

                if !worktrees_using_server.contains(&file.worktree.read(cx).id())
                    || !self
                        .languages
                        .lsp_adapters(&language.name())
                        .iter()
                        .any(|a| a.name == key.1)
                {
                    continue;
                }
                // didOpen
                let file = match file.as_local() {
                    Some(file) => file,
                    None => continue,
                };

                let local = self.as_local_mut().unwrap();

                let buffer_id = buffer.remote_id();
                if local.registered_buffers.contains_key(&buffer_id) {
                    let versions = local
                        .buffer_snapshots
                        .entry(buffer_id)
                        .or_default()
                        .entry(server_id)
                        .and_modify(|_| {
                            assert!(
                            false,
                            "There should not be an existing snapshot for a newly inserted buffer"
                        )
                        })
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
                    language_server.register_buffer(
                        uri,
                        adapter.language_id(&language.name()),
                        version,
                        initial_snapshot.text(),
                    );
                    buffer_paths_registered.push((buffer_id, file.abs_path(cx)));
                    local
                        .buffers_opened_in_servers
                        .entry(buffer_id)
                        .or_default()
                        .insert(server_id);
                }
                buffer_handle.update(cx, |buffer, cx| {
                    buffer.set_completion_triggers(
                        server_id,
                        language_server
                            .capabilities()
                            .completion_provider
                            .as_ref()
                            .and_then(|provider| {
                                provider
                                    .trigger_characters
                                    .as_ref()
                                    .map(|characters| characters.iter().cloned().collect())
                            })
                            .unwrap_or_default(),
                        cx,
                    )
                });
            }
        });

        for (buffer_id, abs_path) in buffer_paths_registered {
            cx.emit(LspStoreEvent::LanguageServerUpdate {
                language_server_id: server_id,
                name: Some(adapter.name()),
                message: proto::update_language_server::Variant::RegisteredForBuffer(
                    proto::RegisteredForBuffer {
                        buffer_abs_path: abs_path.to_string_lossy().to_string(),
                        buffer_id: buffer_id.to_proto(),
                    },
                ),
            });
        }

        cx.notify();
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

    pub(crate) fn cancel_language_server_work_for_buffers(
        &mut self,
        buffers: impl IntoIterator<Item = Entity<Buffer>>,
        cx: &mut Context<Self>,
    ) {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = client.request(proto::CancelLanguageServerWork {
                project_id,
                work: Some(proto::cancel_language_server_work::Work::Buffers(
                    proto::cancel_language_server_work::Buffers {
                        buffer_ids: buffers
                            .into_iter()
                            .map(|b| b.read(cx).remote_id().to_proto())
                            .collect(),
                    },
                )),
            });
            cx.background_spawn(request).detach_and_log_err(cx);
        } else if let Some(local) = self.as_local() {
            let servers = buffers
                .into_iter()
                .flat_map(|buffer| {
                    buffer.update(cx, |buffer, cx| {
                        local.language_server_ids_for_buffer(buffer, cx).into_iter()
                    })
                })
                .collect::<HashSet<_>>();
            for server_id in servers {
                self.cancel_language_server_work(server_id, None, cx);
            }
        }
    }

    pub(crate) fn cancel_language_server_work(
        &mut self,
        server_id: LanguageServerId,
        token_to_cancel: Option<String>,
        cx: &mut Context<Self>,
    ) {
        if let Some(local) = self.as_local() {
            let status = self.language_server_statuses.get(&server_id);
            let server = local.language_servers.get(&server_id);
            if let Some((LanguageServerState::Running { server, .. }, status)) = server.zip(status)
            {
                for (token, progress) in &status.pending_work {
                    if let Some(token_to_cancel) = token_to_cancel.as_ref() {
                        if token != token_to_cancel {
                            continue;
                        }
                    }
                    if progress.is_cancellable {
                        server
                            .notify::<lsp::notification::WorkDoneProgressCancel>(
                                &WorkDoneProgressCancelParams {
                                    token: lsp::NumberOrString::String(token.clone()),
                                },
                            )
                            .ok();
                    }
                }
            }
        } else if let Some((client, project_id)) = self.upstream_client() {
            let request = client.request(proto::CancelLanguageServerWork {
                project_id,
                work: Some(
                    proto::cancel_language_server_work::Work::LanguageServerWork(
                        proto::cancel_language_server_work::LanguageServerWork {
                            language_server_id: server_id.to_proto(),
                            token: token_to_cancel,
                        },
                    ),
                ),
            });
            cx.background_spawn(request).detach_and_log_err(cx);
        }
    }

    fn register_supplementary_language_server(
        &mut self,
        id: LanguageServerId,
        name: LanguageServerName,
        server: Arc<LanguageServer>,
        cx: &mut Context<Self>,
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
        cx: &mut Context<Self>,
    ) {
        if let Some(local) = self.as_local_mut() {
            local.supplementary_language_servers.remove(&id);
            cx.emit(LspStoreEvent::LanguageServerRemoved(id));
        }
    }

    pub(crate) fn supplementary_language_servers(
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
        worktree_handle: &Entity<Worktree>,
        changes: &[(Arc<Path>, ProjectEntryId, PathChange)],
        cx: &mut Context<Self>,
    ) {
        if changes.is_empty() {
            return;
        }

        let Some(local) = self.as_local() else { return };

        local.prettier_store.update(cx, |prettier_store, cx| {
            prettier_store.update_prettier_settings(&worktree_handle, changes, cx)
        });

        let worktree_id = worktree_handle.read(cx).id();
        let mut language_server_ids = local
            .language_server_ids
            .iter()
            .flat_map(|((server_worktree, _), server_ids)| {
                server_ids
                    .iter()
                    .filter_map(|server_id| server_worktree.eq(&worktree_id).then(|| *server_id))
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
                    .and_then(|paths| paths.worktree_paths.get(&worktree_id))
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
                            .notify::<lsp::notification::DidChangeWatchedFiles>(&params)
                            .ok();
                    }
                }
            }
        }
    }

    pub fn wait_for_remote_buffer(
        &mut self,
        id: BufferId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        self.buffer_store.update(cx, |buffer_store, cx| {
            buffer_store.wait_for_remote_buffer(id, cx)
        })
    }

    fn serialize_symbol(symbol: &Symbol) -> proto::Symbol {
        proto::Symbol {
            language_server_name: symbol.language_server_name.0.to_string(),
            source_worktree_id: symbol.source_worktree_id.to_proto(),
            language_server_id: symbol.source_language_server_id.to_proto(),
            worktree_id: symbol.path.worktree_id.to_proto(),
            path: symbol.path.path.as_ref().to_proto(),
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
            path: Arc::<Path>::from_proto(serialized_symbol.path),
        };

        let start = serialized_symbol.start.context("invalid start")?;
        let end = serialized_symbol.end.context("invalid end")?;
        Ok(CoreSymbol {
            language_server_name: LanguageServerName(serialized_symbol.language_server_name.into()),
            source_worktree_id,
            source_language_server_id: LanguageServerId::from_proto(
                serialized_symbol.language_server_id,
            ),
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
        let mut serialized_completion = proto::Completion {
            old_replace_start: Some(serialize_anchor(&completion.replace_range.start)),
            old_replace_end: Some(serialize_anchor(&completion.replace_range.end)),
            new_text: completion.new_text.clone(),
            ..proto::Completion::default()
        };
        match &completion.source {
            CompletionSource::Lsp {
                insert_range,
                server_id,
                lsp_completion,
                lsp_defaults,
                resolved,
            } => {
                let (old_insert_start, old_insert_end) = insert_range
                    .as_ref()
                    .map(|range| (serialize_anchor(&range.start), serialize_anchor(&range.end)))
                    .unzip();

                serialized_completion.old_insert_start = old_insert_start;
                serialized_completion.old_insert_end = old_insert_end;
                serialized_completion.source = proto::completion::Source::Lsp as i32;
                serialized_completion.server_id = server_id.0 as u64;
                serialized_completion.lsp_completion = serde_json::to_vec(lsp_completion).unwrap();
                serialized_completion.lsp_defaults = lsp_defaults
                    .as_deref()
                    .map(|lsp_defaults| serde_json::to_vec(lsp_defaults).unwrap());
                serialized_completion.resolved = *resolved;
            }
            CompletionSource::BufferWord {
                word_range,
                resolved,
            } => {
                serialized_completion.source = proto::completion::Source::BufferWord as i32;
                serialized_completion.buffer_word_start = Some(serialize_anchor(&word_range.start));
                serialized_completion.buffer_word_end = Some(serialize_anchor(&word_range.end));
                serialized_completion.resolved = *resolved;
            }
            CompletionSource::Custom => {
                serialized_completion.source = proto::completion::Source::Custom as i32;
                serialized_completion.resolved = true;
            }
            CompletionSource::Dap { sort_text } => {
                serialized_completion.source = proto::completion::Source::Dap as i32;
                serialized_completion.sort_text = Some(sort_text.clone());
            }
        }

        serialized_completion
    }

    pub(crate) fn deserialize_completion(completion: proto::Completion) -> Result<CoreCompletion> {
        let old_replace_start = completion
            .old_replace_start
            .and_then(deserialize_anchor)
            .context("invalid old start")?;
        let old_replace_end = completion
            .old_replace_end
            .and_then(deserialize_anchor)
            .context("invalid old end")?;
        let insert_range = {
            match completion.old_insert_start.zip(completion.old_insert_end) {
                Some((start, end)) => {
                    let start = deserialize_anchor(start).context("invalid insert old start")?;
                    let end = deserialize_anchor(end).context("invalid insert old end")?;
                    Some(start..end)
                }
                None => None,
            }
        };
        Ok(CoreCompletion {
            replace_range: old_replace_start..old_replace_end,
            new_text: completion.new_text,
            source: match proto::completion::Source::from_i32(completion.source) {
                Some(proto::completion::Source::Custom) => CompletionSource::Custom,
                Some(proto::completion::Source::Lsp) => CompletionSource::Lsp {
                    insert_range,
                    server_id: LanguageServerId::from_proto(completion.server_id),
                    lsp_completion: serde_json::from_slice(&completion.lsp_completion)?,
                    lsp_defaults: completion
                        .lsp_defaults
                        .as_deref()
                        .map(serde_json::from_slice)
                        .transpose()?,
                    resolved: completion.resolved,
                },
                Some(proto::completion::Source::BufferWord) => {
                    let word_range = completion
                        .buffer_word_start
                        .and_then(deserialize_anchor)
                        .context("invalid buffer word start")?
                        ..completion
                            .buffer_word_end
                            .and_then(deserialize_anchor)
                            .context("invalid buffer word end")?;
                    CompletionSource::BufferWord {
                        word_range,
                        resolved: completion.resolved,
                    }
                }
                Some(proto::completion::Source::Dap) => CompletionSource::Dap {
                    sort_text: completion
                        .sort_text
                        .context("expected sort text to exist")?,
                },
                _ => anyhow::bail!("Unexpected completion source {}", completion.source),
            },
        })
    }

    pub(crate) fn serialize_code_action(action: &CodeAction) -> proto::CodeAction {
        let (kind, lsp_action) = match &action.lsp_action {
            LspAction::Action(code_action) => (
                proto::code_action::Kind::Action as i32,
                serde_json::to_vec(code_action).unwrap(),
            ),
            LspAction::Command(command) => (
                proto::code_action::Kind::Command as i32,
                serde_json::to_vec(command).unwrap(),
            ),
            LspAction::CodeLens(code_lens) => (
                proto::code_action::Kind::CodeLens as i32,
                serde_json::to_vec(code_lens).unwrap(),
            ),
        };

        proto::CodeAction {
            server_id: action.server_id.0 as u64,
            start: Some(serialize_anchor(&action.range.start)),
            end: Some(serialize_anchor(&action.range.end)),
            lsp_action,
            kind,
            resolved: action.resolved,
        }
    }

    pub(crate) fn deserialize_code_action(action: proto::CodeAction) -> Result<CodeAction> {
        let start = action
            .start
            .and_then(deserialize_anchor)
            .context("invalid start")?;
        let end = action
            .end
            .and_then(deserialize_anchor)
            .context("invalid end")?;
        let lsp_action = match proto::code_action::Kind::from_i32(action.kind) {
            Some(proto::code_action::Kind::Action) => {
                LspAction::Action(serde_json::from_slice(&action.lsp_action)?)
            }
            Some(proto::code_action::Kind::Command) => {
                LspAction::Command(serde_json::from_slice(&action.lsp_action)?)
            }
            Some(proto::code_action::Kind::CodeLens) => {
                LspAction::CodeLens(serde_json::from_slice(&action.lsp_action)?)
            }
            None => anyhow::bail!("Unknown action kind {}", action.kind),
        };
        Ok(CodeAction {
            server_id: LanguageServerId(action.server_id as usize),
            range: start..end,
            resolved: action.resolved,
            lsp_action,
        })
    }

    fn update_last_formatting_failure<T>(&mut self, formatting_result: &anyhow::Result<T>) {
        match &formatting_result {
            Ok(_) => self.last_formatting_failure = None,
            Err(error) => {
                let error_string = format!("{error:#}");
                log::error!("Formatting failed: {error_string}");
                self.last_formatting_failure
                    .replace(error_string.lines().join(" "));
            }
        }
    }

    fn cleanup_lsp_data(&mut self, for_server: LanguageServerId) {
        self.lsp_server_capabilities.remove(&for_server);
        for buffer_colors in self.lsp_document_colors.values_mut() {
            buffer_colors.colors.remove(&for_server);
            buffer_colors.cache_version += 1;
        }
        for buffer_lens in self.lsp_code_lens.values_mut() {
            buffer_lens.lens.remove(&for_server);
        }
        if let Some(local) = self.as_local_mut() {
            local.buffer_pull_diagnostics_result_ids.remove(&for_server);
            for buffer_servers in local.buffers_opened_in_servers.values_mut() {
                buffer_servers.remove(&for_server);
            }
        }
    }

    pub fn result_id(
        &self,
        server_id: LanguageServerId,
        buffer_id: BufferId,
        cx: &App,
    ) -> Option<String> {
        let abs_path = self
            .buffer_store
            .read(cx)
            .get(buffer_id)
            .and_then(|b| File::from_dyn(b.read(cx).file()))
            .map(|f| f.abs_path(cx))?;
        self.as_local()?
            .buffer_pull_diagnostics_result_ids
            .get(&server_id)?
            .get(&abs_path)?
            .clone()
    }

    pub fn all_result_ids(&self, server_id: LanguageServerId) -> HashMap<PathBuf, String> {
        let Some(local) = self.as_local() else {
            return HashMap::default();
        };
        local
            .buffer_pull_diagnostics_result_ids
            .get(&server_id)
            .into_iter()
            .flatten()
            .filter_map(|(abs_path, result_id)| Some((abs_path.clone(), result_id.clone()?)))
            .collect()
    }

    pub fn pull_workspace_diagnostics(&mut self, server_id: LanguageServerId) {
        if let Some(LanguageServerState::Running {
            workspace_refresh_task: Some(workspace_refresh_task),
            ..
        }) = self
            .as_local_mut()
            .and_then(|local| local.language_servers.get_mut(&server_id))
        {
            workspace_refresh_task.refresh_tx.try_send(()).ok();
        }
    }

    pub fn pull_workspace_diagnostics_for_buffer(&mut self, buffer_id: BufferId, cx: &mut App) {
        let Some(buffer) = self.buffer_store().read(cx).get_existing(buffer_id).ok() else {
            return;
        };
        let Some(local) = self.as_local_mut() else {
            return;
        };

        for server_id in buffer.update(cx, |buffer, cx| {
            local.language_server_ids_for_buffer(buffer, cx)
        }) {
            if let Some(LanguageServerState::Running {
                workspace_refresh_task: Some(workspace_refresh_task),
                ..
            }) = local.language_servers.get_mut(&server_id)
            {
                workspace_refresh_task.refresh_tx.try_send(()).ok();
            }
        }
    }

    fn apply_workspace_diagnostic_report(
        &mut self,
        server_id: LanguageServerId,
        report: lsp::WorkspaceDiagnosticReportResult,
        cx: &mut Context<Self>,
    ) {
        let workspace_diagnostics =
            GetDocumentDiagnostics::deserialize_workspace_diagnostics_report(report, server_id);
        for workspace_diagnostics in workspace_diagnostics {
            let LspPullDiagnostics::Response {
                server_id,
                uri,
                diagnostics,
            } = workspace_diagnostics.diagnostics
            else {
                continue;
            };

            let adapter = self.language_server_adapter_for_id(server_id);
            let disk_based_sources = adapter
                .as_ref()
                .map(|adapter| adapter.disk_based_diagnostic_sources.as_slice())
                .unwrap_or(&[]);

            match diagnostics {
                PulledDiagnostics::Unchanged { result_id } => {
                    self.merge_diagnostics(
                        server_id,
                        lsp::PublishDiagnosticsParams {
                            uri: uri.clone(),
                            diagnostics: Vec::new(),
                            version: None,
                        },
                        Some(result_id),
                        DiagnosticSourceKind::Pulled,
                        disk_based_sources,
                        |_, _, _| true,
                        cx,
                    )
                    .log_err();
                }
                PulledDiagnostics::Changed {
                    diagnostics,
                    result_id,
                } => {
                    self.merge_diagnostics(
                        server_id,
                        lsp::PublishDiagnosticsParams {
                            uri: uri.clone(),
                            diagnostics,
                            version: workspace_diagnostics.version,
                        },
                        result_id,
                        DiagnosticSourceKind::Pulled,
                        disk_based_sources,
                        |buffer, old_diagnostic, cx| match old_diagnostic.source_kind {
                            DiagnosticSourceKind::Pulled => {
                                let buffer_url = File::from_dyn(buffer.file())
                                    .map(|f| f.abs_path(cx))
                                    .and_then(|abs_path| file_path_to_lsp_url(&abs_path).ok());
                                buffer_url.is_none_or(|buffer_url| buffer_url != uri)
                            }
                            DiagnosticSourceKind::Other | DiagnosticSourceKind::Pushed => true,
                        },
                        cx,
                    )
                    .log_err();
                }
            }
        }
    }
}

fn subscribe_to_binary_statuses(
    languages: &Arc<LanguageRegistry>,
    cx: &mut Context<'_, LspStore>,
) -> Task<()> {
    let mut server_statuses = languages.language_server_binary_statuses();
    cx.spawn(async move |lsp_store, cx| {
        while let Some((server_name, binary_status)) = server_statuses.next().await {
            if lsp_store
                .update(cx, |_, cx| {
                    let mut message = None;
                    let binary_status = match binary_status {
                        BinaryStatus::None => proto::ServerBinaryStatus::None,
                        BinaryStatus::CheckingForUpdate => {
                            proto::ServerBinaryStatus::CheckingForUpdate
                        }
                        BinaryStatus::Downloading => proto::ServerBinaryStatus::Downloading,
                        BinaryStatus::Starting => proto::ServerBinaryStatus::Starting,
                        BinaryStatus::Stopping => proto::ServerBinaryStatus::Stopping,
                        BinaryStatus::Stopped => proto::ServerBinaryStatus::Stopped,
                        BinaryStatus::Failed { error } => {
                            message = Some(error);
                            proto::ServerBinaryStatus::Failed
                        }
                    };
                    cx.emit(LspStoreEvent::LanguageServerUpdate {
                        // Binary updates are about the binary that might not have any language server id at that point.
                        // Reuse `LanguageServerUpdate` for them and provide a fake id that won't be used on the receiver side.
                        language_server_id: LanguageServerId(0),
                        name: Some(server_name),
                        message: proto::update_language_server::Variant::StatusUpdate(
                            proto::StatusUpdate {
                                message,
                                status: Some(proto::status_update::Status::Binary(
                                    binary_status as i32,
                                )),
                            },
                        ),
                    });
                })
                .is_err()
            {
                break;
            }
        }
    })
}

fn lsp_workspace_diagnostics_refresh(
    server: Arc<LanguageServer>,
    cx: &mut Context<'_, LspStore>,
) -> Option<WorkspaceRefreshTask> {
    let identifier = match server.capabilities().diagnostic_provider? {
        lsp::DiagnosticServerCapabilities::Options(diagnostic_options) => {
            if !diagnostic_options.workspace_diagnostics {
                return None;
            }
            diagnostic_options.identifier
        }
        lsp::DiagnosticServerCapabilities::RegistrationOptions(registration_options) => {
            let diagnostic_options = registration_options.diagnostic_options;
            if !diagnostic_options.workspace_diagnostics {
                return None;
            }
            diagnostic_options.identifier
        }
    };

    let (progress_tx, mut progress_rx) = mpsc::channel(1);
    let (mut refresh_tx, mut refresh_rx) = mpsc::channel(1);
    refresh_tx.try_send(()).ok();

    let workspace_query_language_server = cx.spawn(async move |lsp_store, cx| {
        let mut attempts = 0;
        let max_attempts = 50;
        let mut requests = 0;

        loop {
            let Some(()) = refresh_rx.recv().await else {
                return;
            };

            'request: loop {
                requests += 1;
                if attempts > max_attempts {
                    log::error!(
                        "Failed to pull workspace diagnostics {max_attempts} times, aborting"
                    );
                    return;
                }
                let backoff_millis = (50 * (1 << attempts)).clamp(30, 1000);
                cx.background_executor()
                    .timer(Duration::from_millis(backoff_millis))
                    .await;
                attempts += 1;

                let Ok(previous_result_ids) = lsp_store.update(cx, |lsp_store, _| {
                    lsp_store
                        .all_result_ids(server.server_id())
                        .into_iter()
                        .filter_map(|(abs_path, result_id)| {
                            let uri = file_path_to_lsp_url(&abs_path).ok()?;
                            Some(lsp::PreviousResultId {
                                uri,
                                value: result_id,
                            })
                        })
                        .collect()
                }) else {
                    return;
                };

                let token = format!("workspace/diagnostic-{}-{}", server.server_id(), requests);

                progress_rx.try_recv().ok();
                let timer =
                    LanguageServer::default_request_timer(cx.background_executor().clone()).fuse();
                let progress = pin!(progress_rx.recv().fuse());
                let response_result = server
                    .request_with_timer::<lsp::WorkspaceDiagnosticRequest, _>(
                        lsp::WorkspaceDiagnosticParams {
                            previous_result_ids,
                            identifier: identifier.clone(),
                            work_done_progress_params: Default::default(),
                            partial_result_params: lsp::PartialResultParams {
                                partial_result_token: Some(lsp::ProgressToken::String(token)),
                            },
                        },
                        select(timer, progress).then(|either| match either {
                            Either::Left((message, ..)) => ready(message).left_future(),
                            Either::Right(..) => pending::<String>().right_future(),
                        }),
                    )
                    .await;

                // https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnostic_refresh
                // >  If a server closes a workspace diagnostic pull request the client should re-trigger the request.
                match response_result {
                    ConnectionResult::Timeout => {
                        log::error!("Timeout during workspace diagnostics pull");
                        continue 'request;
                    }
                    ConnectionResult::ConnectionReset => {
                        log::error!("Server closed a workspace diagnostics pull request");
                        continue 'request;
                    }
                    ConnectionResult::Result(Err(e)) => {
                        log::error!("Error during workspace diagnostics pull: {e:#}");
                        break 'request;
                    }
                    ConnectionResult::Result(Ok(pulled_diagnostics)) => {
                        attempts = 0;
                        if lsp_store
                            .update(cx, |lsp_store, cx| {
                                lsp_store.apply_workspace_diagnostic_report(
                                    server.server_id(),
                                    pulled_diagnostics,
                                    cx,
                                )
                            })
                            .is_err()
                        {
                            return;
                        }
                        break 'request;
                    }
                }
            }
        }
    });

    Some(WorkspaceRefreshTask {
        refresh_tx,
        progress_tx,
        task: workspace_query_language_server,
    })
}

fn resolve_word_completion(snapshot: &BufferSnapshot, completion: &mut Completion) {
    let CompletionSource::BufferWord {
        word_range,
        resolved,
    } = &mut completion.source
    else {
        return;
    };
    if *resolved {
        return;
    }

    if completion.new_text
        != snapshot
            .text_for_range(word_range.clone())
            .collect::<String>()
    {
        return;
    }

    let mut offset = 0;
    for chunk in snapshot.chunks(word_range.clone(), true) {
        let end_offset = offset + chunk.text.len();
        if let Some(highlight_id) = chunk.syntax_highlight_id {
            completion
                .label
                .runs
                .push((offset..end_offset, highlight_id));
        }
        offset = end_offset;
    }
    *resolved = true;
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
    new_completions: Vec<CoreCompletion>,
    language: Option<Arc<Language>>,
    lsp_adapter: Option<Arc<CachedLspAdapter>>,
) -> Vec<Completion> {
    let lsp_completions = new_completions
        .iter()
        .filter_map(|new_completion| {
            if let Some(lsp_completion) = new_completion.source.lsp_completion(true) {
                Some(lsp_completion.into_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut labels = if let Some((language, lsp_adapter)) = language.as_ref().zip(lsp_adapter) {
        lsp_adapter
            .labels_for_completions(&lsp_completions, language)
            .await
            .log_err()
            .unwrap_or_default()
    } else {
        Vec::new()
    }
    .into_iter()
    .fuse();

    let mut completions = Vec::new();
    for completion in new_completions {
        match completion.source.lsp_completion(true) {
            Some(lsp_completion) => {
                let documentation = if let Some(docs) = lsp_completion.documentation.clone() {
                    Some(docs.into())
                } else {
                    None
                };

                let mut label = labels.next().flatten().unwrap_or_else(|| {
                    CodeLabel::fallback_for_completion(&lsp_completion, language.as_deref())
                });
                ensure_uniform_list_compatible_label(&mut label);
                completions.push(Completion {
                    label,
                    documentation,
                    replace_range: completion.replace_range,
                    new_text: completion.new_text,
                    insert_text_mode: lsp_completion.insert_text_mode,
                    source: completion.source,
                    icon_path: None,
                    confirm: None,
                });
            }
            None => {
                let mut label = CodeLabel::plain(completion.new_text.clone(), None);
                ensure_uniform_list_compatible_label(&mut label);
                completions.push(Completion {
                    label,
                    documentation: None,
                    replace_range: completion.replace_range,
                    new_text: completion.new_text,
                    source: completion.source,
                    insert_text_mode: None,
                    icon_path: None,
                    confirm: None,
                });
            }
        }
    }
    completions
}

#[derive(Debug)]
pub enum LanguageServerToQuery {
    /// Query language servers in order of users preference, up until one capable of handling the request is found.
    FirstCapable,
    /// Query a specific language server.
    Other(LanguageServerId),
}

#[derive(Default)]
struct RenamePathsWatchedForServer {
    did_rename: Vec<RenameActionPredicate>,
    will_rename: Vec<RenameActionPredicate>,
}

impl RenamePathsWatchedForServer {
    fn with_did_rename_patterns(
        mut self,
        did_rename: Option<&FileOperationRegistrationOptions>,
    ) -> Self {
        if let Some(did_rename) = did_rename {
            self.did_rename = did_rename
                .filters
                .iter()
                .filter_map(|filter| filter.try_into().log_err())
                .collect();
        }
        self
    }
    fn with_will_rename_patterns(
        mut self,
        will_rename: Option<&FileOperationRegistrationOptions>,
    ) -> Self {
        if let Some(will_rename) = will_rename {
            self.will_rename = will_rename
                .filters
                .iter()
                .filter_map(|filter| filter.try_into().log_err())
                .collect();
        }
        self
    }

    fn should_send_did_rename(&self, path: &str, is_dir: bool) -> bool {
        self.did_rename.iter().any(|pred| pred.eval(path, is_dir))
    }
    fn should_send_will_rename(&self, path: &str, is_dir: bool) -> bool {
        self.will_rename.iter().any(|pred| pred.eval(path, is_dir))
    }
}

impl TryFrom<&FileOperationFilter> for RenameActionPredicate {
    type Error = globset::Error;
    fn try_from(ops: &FileOperationFilter) -> Result<Self, globset::Error> {
        Ok(Self {
            kind: ops.pattern.matches.clone(),
            glob: GlobBuilder::new(&ops.pattern.glob)
                .case_insensitive(
                    ops.pattern
                        .options
                        .as_ref()
                        .map_or(false, |ops| ops.ignore_case.unwrap_or(false)),
                )
                .build()?
                .compile_matcher(),
        })
    }
}
struct RenameActionPredicate {
    glob: GlobMatcher,
    kind: Option<FileOperationPatternKind>,
}

impl RenameActionPredicate {
    // Returns true if language server should be notified
    fn eval(&self, path: &str, is_dir: bool) -> bool {
        self.kind.as_ref().map_or(true, |kind| {
            let expected_kind = if is_dir {
                FileOperationPatternKind::Folder
            } else {
                FileOperationPatternKind::File
            };
            kind == &expected_kind
        }) && self.glob.is_match(path)
    }
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
        cx: &mut Context<LspStore>,
    ) -> LanguageServerWatchedPaths {
        let project = cx.weak_entity();

        const LSP_ABS_PATH_OBSERVE: Duration = Duration::from_millis(100);
        let abs_paths = self
            .abs_paths
            .into_iter()
            .map(|(abs_path, globset)| {
                let task = cx.spawn({
                    let abs_path = abs_path.clone();
                    let fs = fs.clone();

                    let lsp_store = project.clone();
                    async move |_, cx| {
                        maybe!(async move {
                            let mut push_updates = fs.watch(&abs_path, LSP_ABS_PATH_OBSERVE).await;
                            while let Some(update) = push_updates.0.next().await {
                                let action = lsp_store
                                    .update(cx, |this, _| {
                                        let Some(local) = this.as_local() else {
                                            return ControlFlow::Break(());
                                        };
                                        let Some(watcher) = local
                                            .language_server_watched_paths
                                            .get(&language_server_id)
                                        else {
                                            return ControlFlow::Break(());
                                        };
                                        let (globs, _) = watcher.abs_paths.get(&abs_path).expect(
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

pub struct WorkspaceRefreshTask {
    refresh_tx: mpsc::Sender<()>,
    progress_tx: mpsc::Sender<()>,
    #[allow(dead_code)]
    task: Task<()>,
}

pub enum LanguageServerState {
    Starting {
        startup: Task<Option<Arc<LanguageServer>>>,
        /// List of language servers that will be added to the workspace once it's initialization completes.
        pending_workspace_folders: Arc<Mutex<BTreeSet<Url>>>,
    },

    Running {
        adapter: Arc<CachedLspAdapter>,
        server: Arc<LanguageServer>,
        simulate_disk_based_diagnostics_completion: Option<Task<()>>,
        workspace_refresh_task: Option<WorkspaceRefreshTask>,
    },
}

impl LanguageServerState {
    fn add_workspace_folder(&self, uri: Url) {
        match self {
            LanguageServerState::Starting {
                pending_workspace_folders,
                ..
            } => {
                pending_workspace_folders.lock().insert(uri);
            }
            LanguageServerState::Running { server, .. } => {
                server.add_workspace_folder(uri);
            }
        }
    }
    fn _remove_workspace_folder(&self, uri: Url) {
        match self {
            LanguageServerState::Starting {
                pending_workspace_folders,
                ..
            } => {
                pending_workspace_folders.lock().remove(&uri);
            }
            LanguageServerState::Running { server, .. } => server.remove_workspace_folder(uri),
        }
    }
}

impl std::fmt::Debug for LanguageServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageServerState::Starting { .. } => {
                f.debug_struct("LanguageServerState::Starting").finish()
            }
            LanguageServerState::Running { .. } => {
                f.debug_struct("LanguageServerState::Running").finish()
            }
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
            path: path.to_proto(),
            language_server_id: language_server_id.0 as u64,
            error_count: self.error_count as u32,
            warning_count: self.warning_count as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CompletionDocumentation {
    /// There is no documentation for this completion.
    Undocumented,
    /// A single line of documentation.
    SingleLine(SharedString),
    /// Multiple lines of plain text documentation.
    MultiLinePlainText(SharedString),
    /// Markdown documentation.
    MultiLineMarkdown(SharedString),
    /// Both single line and multiple lines of plain text documentation.
    SingleLineAndMultiLinePlainText {
        single_line: SharedString,
        plain_text: Option<SharedString>,
    },
}

impl From<lsp::Documentation> for CompletionDocumentation {
    fn from(docs: lsp::Documentation) -> Self {
        match docs {
            lsp::Documentation::String(text) => {
                if text.lines().count() <= 1 {
                    CompletionDocumentation::SingleLine(text.into())
                } else {
                    CompletionDocumentation::MultiLinePlainText(text.into())
                }
            }

            lsp::Documentation::MarkupContent(lsp::MarkupContent { kind, value }) => match kind {
                lsp::MarkupKind::PlainText => {
                    if value.lines().count() <= 1 {
                        CompletionDocumentation::SingleLine(value.into())
                    } else {
                        CompletionDocumentation::MultiLinePlainText(value.into())
                    }
                }

                lsp::MarkupKind::Markdown => {
                    CompletionDocumentation::MultiLineMarkdown(value.into())
                }
            },
        }
    }
}

fn glob_literal_prefix(glob: &Path) -> PathBuf {
    glob.components()
        .take_while(|component| match component {
            path::Component::Normal(part) => !part.to_string_lossy().contains(['*', '?', '{', '}']),
            _ => true,
        })
        .collect()
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
        _: &dyn Fs,
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
        _: Arc<dyn LanguageToolchainStore>,
        _: &AsyncApp,
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

pub fn language_server_settings<'a>(
    delegate: &'a dyn LspAdapterDelegate,
    language: &LanguageServerName,
    cx: &'a App,
) -> Option<&'a LspSettings> {
    language_server_settings_for(
        SettingsLocation {
            worktree_id: delegate.worktree_id(),
            path: delegate.worktree_root_path(),
        },
        language,
        cx,
    )
}

pub(crate) fn language_server_settings_for<'a>(
    location: SettingsLocation<'a>,
    language: &LanguageServerName,
    cx: &'a App,
) -> Option<&'a LspSettings> {
    ProjectSettings::get(Some(location), cx).lsp.get(language)
}

pub struct LocalLspAdapterDelegate {
    lsp_store: WeakEntity<LspStore>,
    worktree: worktree::Snapshot,
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    language_registry: Arc<LanguageRegistry>,
    load_shell_env_task: Shared<Task<Option<HashMap<String, String>>>>,
}

impl LocalLspAdapterDelegate {
    pub fn new(
        language_registry: Arc<LanguageRegistry>,
        environment: &Entity<ProjectEnvironment>,
        lsp_store: WeakEntity<LspStore>,
        worktree: &Entity<Worktree>,
        http_client: Arc<dyn HttpClient>,
        fs: Arc<dyn Fs>,
        cx: &mut App,
    ) -> Arc<Self> {
        let load_shell_env_task = environment.update(cx, |env, cx| {
            env.get_worktree_environment(worktree.clone(), cx)
        });

        Arc::new(Self {
            lsp_store,
            worktree: worktree.read(cx).snapshot(),
            fs,
            http_client,
            language_registry,
            load_shell_env_task,
        })
    }

    fn from_local_lsp(
        local: &LocalLspStore,
        worktree: &Entity<Worktree>,
        cx: &mut App,
    ) -> Arc<Self> {
        Self::new(
            local.languages.clone(),
            &local.environment,
            local.weak.clone(),
            worktree,
            local.http_client.clone(),
            local.fs.clone(),
            cx,
        )
    }
}

#[async_trait]
impl LspAdapterDelegate for LocalLspAdapterDelegate {
    fn show_notification(&self, message: &str, cx: &mut App) {
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
        let output = util::command::new_smol_command(&npm)
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
        let output = util::command::new_smol_command(&command.path)
            .args(command.arguments)
            .envs(command.env.clone().unwrap_or_default())
            .current_dir(working_dir)
            .output()
            .await?;

        anyhow::ensure!(
            output.status.success(),
            "{}, stdout: {:?}, stderr: {:?}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    fn update_status(&self, server_name: LanguageServerName, status: language::BinaryStatus) {
        self.language_registry
            .update_lsp_binary_status(server_name, status);
    }

    fn registered_lsp_adapters(&self) -> Vec<Arc<dyn LspAdapter>> {
        self.language_registry
            .all_lsp_adapters()
            .into_iter()
            .map(|adapter| adapter.adapter.clone() as Arc<dyn LspAdapter>)
            .collect()
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
    lsp_adapter: Option<Arc<CachedLspAdapter>>,
    output: &mut Vec<Symbol>,
) {
    #[allow(clippy::mutable_key_type)]
    let mut symbols_by_language = HashMap::<Option<Arc<Language>>, Vec<CoreSymbol>>::default();

    let mut unknown_paths = BTreeSet::new();
    for symbol in symbols {
        let language = language_registry
            .language_for_file_path(&symbol.path.path)
            .await
            .ok()
            .or_else(|| {
                unknown_paths.insert(symbol.path.path.clone());
                None
            });
        symbols_by_language
            .entry(language)
            .or_default()
            .push(symbol);
    }

    for unknown_path in unknown_paths {
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
                source_language_server_id: symbol.source_language_server_id,
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

/// Completion items are displayed in a `UniformList`.
/// Usually, those items are single-line strings, but in LSP responses,
/// completion items `label`, `detail` and `label_details.description` may contain newlines or long spaces.
/// Many language plugins construct these items by joining these parts together, and we may use `CodeLabel::fallback_for_completion` that uses `label` at least.
/// All that may lead to a newline being inserted into resulting `CodeLabel.text`, which will force `UniformList` to bloat each entry to occupy more space,
/// breaking the completions menu presentation.
///
/// Sanitize the text to ensure there are no newlines, or, if there are some, remove them and also remove long space sequences if there were newlines.
fn ensure_uniform_list_compatible_label(label: &mut CodeLabel) {
    let mut new_text = String::with_capacity(label.text.len());
    let mut offset_map = vec![0; label.text.len() + 1];
    let mut last_char_was_space = false;
    let mut new_idx = 0;
    let mut chars = label.text.char_indices().fuse();
    let mut newlines_removed = false;

    while let Some((idx, c)) = chars.next() {
        offset_map[idx] = new_idx;

        match c {
            '\n' if last_char_was_space => {
                newlines_removed = true;
            }
            '\t' | ' ' if last_char_was_space => {}
            '\n' if !last_char_was_space => {
                new_text.push(' ');
                new_idx += 1;
                last_char_was_space = true;
                newlines_removed = true;
            }
            ' ' | '\t' => {
                new_text.push(' ');
                new_idx += 1;
                last_char_was_space = true;
            }
            _ => {
                new_text.push(c);
                new_idx += c.len_utf8();
                last_char_was_space = false;
            }
        }
    }
    offset_map[label.text.len()] = new_idx;

    // Only modify the label if newlines were removed.
    if !newlines_removed {
        return;
    }

    let last_index = new_idx;
    let mut run_ranges_errors = Vec::new();
    label.runs.retain_mut(|(range, _)| {
        match offset_map.get(range.start) {
            Some(&start) => range.start = start,
            None => {
                run_ranges_errors.push(range.clone());
                return false;
            }
        }

        match offset_map.get(range.end) {
            Some(&end) => range.end = end,
            None => {
                run_ranges_errors.push(range.clone());
                range.end = last_index;
            }
        }
        true
    });
    if !run_ranges_errors.is_empty() {
        log::error!(
            "Completion label has errors in its run ranges: {run_ranges_errors:?}, label text: {}",
            label.text
        );
    }

    let mut wrong_filter_range = None;
    if label.filter_range == (0..label.text.len()) {
        label.filter_range = 0..new_text.len();
    } else {
        let mut original_filter_range = Some(label.filter_range.clone());
        match offset_map.get(label.filter_range.start) {
            Some(&start) => label.filter_range.start = start,
            None => {
                wrong_filter_range = original_filter_range.take();
                label.filter_range.start = last_index;
            }
        }

        match offset_map.get(label.filter_range.end) {
            Some(&end) => label.filter_range.end = end,
            None => {
                wrong_filter_range = original_filter_range.take();
                label.filter_range.end = last_index;
            }
        }
    }
    if let Some(wrong_filter_range) = wrong_filter_range {
        log::error!(
            "Completion label has an invalid filter range: {wrong_filter_range:?}, label text: {}",
            label.text
        );
    }

    label.text = new_text;
}

#[cfg(test)]
mod tests {
    use language::HighlightId;

    use super::*;

    #[test]
    fn test_glob_literal_prefix() {
        assert_eq!(glob_literal_prefix(Path::new("**/*.js")), Path::new(""));
        assert_eq!(
            glob_literal_prefix(Path::new("node_modules/**/*.js")),
            Path::new("node_modules")
        );
        assert_eq!(
            glob_literal_prefix(Path::new("foo/{bar,baz}.js")),
            Path::new("foo")
        );
        assert_eq!(
            glob_literal_prefix(Path::new("foo/bar/baz.js")),
            Path::new("foo/bar/baz.js")
        );

        #[cfg(target_os = "windows")]
        {
            assert_eq!(glob_literal_prefix(Path::new("**\\*.js")), Path::new(""));
            assert_eq!(
                glob_literal_prefix(Path::new("node_modules\\**/*.js")),
                Path::new("node_modules")
            );
            assert_eq!(
                glob_literal_prefix(Path::new("foo/{bar,baz}.js")),
                Path::new("foo")
            );
            assert_eq!(
                glob_literal_prefix(Path::new("foo\\bar\\baz.js")),
                Path::new("foo/bar/baz.js")
            );
        }
    }

    #[test]
    fn test_multi_len_chars_normalization() {
        let mut label = CodeLabel {
            text: "myElˇ (parameter) myElˇ: {\n    foo: string;\n}".to_string(),
            runs: vec![(0..6, HighlightId(1))],
            filter_range: 0..6,
        };
        ensure_uniform_list_compatible_label(&mut label);
        assert_eq!(
            label,
            CodeLabel {
                text: "myElˇ (parameter) myElˇ: { foo: string; }".to_string(),
                runs: vec![(0..6, HighlightId(1))],
                filter_range: 0..6,
            }
        );
    }
}
