use crate::buffer_store::{BufferStore, BufferStoreEvent};
use crate::debounced_delay::DebouncedDelay;
use anyhow::{anyhow, Context as _, Result};
use async_trait::async_trait;
use client::{
    proto, Client, Collaborator, DevServerProjectId, PendingEntitySubscription, ProjectId,
    TypedEnvelope, UserStore,
};
use clock::ReplicaId;
use collections::{btree_map, BTreeMap, BTreeSet, HashMap, HashSet};
use futures::{
    channel::mpsc::{self, UnboundedReceiver},
    future::{join_all, try_join_all, Shared},
    select,
    stream::FuturesUnordered,
    AsyncWriteExt, Future, FutureExt, StreamExt,
};

use crate::prettier_support::{DefaultPrettier, PrettierInstance};
use crate::project_settings::{DirenvSettings, LspSettings, ProjectSettings};
use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use crate::yarn::YarnPathStore;
use crate::{lsp_command::*, ProjectPath, ProjectTransaction};
use git::{blame::Blame, repository::GitRepository};
use globset::{Glob, GlobSet, GlobSetBuilder};
use gpui::{
    AnyModel, AppContext, AsyncAppContext, BorrowAppContext, Context, Entity, EventEmitter, Model,
    ModelContext, PromptLevel, SharedString, Task, WeakModel, WindowContext,
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
    LanguageServerBinary, LanguageServerId, LspRequestFuture, MessageActionItem, MessageType,
    OneOf, ServerHealthStatus, ServerStatus, TextEdit, WorkDoneProgressCancelParams,
};
use node_runtime::NodeRuntime;
use parking_lot::{Mutex, RwLock};
use paths::{
    local_settings_file_relative_path, local_tasks_file_relative_path,
    local_vscode_tasks_file_relative_path,
};
use postage::watch;
use rand::prelude::*;
use remote::SshSession;
use rpc::{
    proto::{AddWorktree, AnyProtoClient},
    ErrorCode,
};
use serde::Serialize;
use settings::{watch_config_file, Settings, SettingsLocation, SettingsStore};
use sha2::{Digest, Sha256};
use similar::{ChangeTag, TextDiff};
use smol::channel::{Receiver, Sender};
use snippet::Snippet;
use snippet_provider::SnippetProvider;
use std::{
    borrow::Cow,
    cell::RefCell,
    cmp::Ordering,
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
use text::{Anchor, BufferId, LineEnding};
use util::{
    debug_panic, defer, maybe, merge_json_value_into, parse_env_output, paths::compare_paths,
    post_inc, ResultExt, TryFutureExt as _,
};
use worktree::{CreatedEntry, Snapshot, Traversal};

pub use fs::*;
pub use language::Location;
#[cfg(any(test, feature = "test-support"))]
pub use prettier::FORMAT_SUFFIX as TEST_PRETTIER_FORMAT_SUFFIX;
pub use worktree::{
    Entry, EntryKind, File, LocalWorktree, PathChange, ProjectEntryId, RepositoryEntry,
    UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree, WorktreeId, WorktreeSettings,
    FS_WATCH_LATENCY,
};

pub struct LspStore {
    client: AnyProtoClient,
    remote_id: Option<u64>,
    buffer_store: Model<BufferStore>,
    worktree_store: Model<WorktreeStore>,
    buffer_snapshots: HashMap<BufferId, HashMap<LanguageServerId, Vec<LspBufferSnapshot>>>, // buffer_id -> server_id -> vec of snapshots
    supplementary_language_servers:
        HashMap<LanguageServerId, (LanguageServerName, Arc<LanguageServer>)>,
    languages: Arc<LanguageRegistry>,
    language_servers: HashMap<LanguageServerId, LanguageServerState>,
    language_server_ids: HashMap<(WorktreeId, LanguageServerName), LanguageServerId>,
    language_server_statuses: BTreeMap<LanguageServerId, LanguageServerStatus>,
    last_formatting_failure: Option<String>,
    last_workspace_edits_by_language_server: HashMap<LanguageServerId, ProjectTransaction>,
    language_server_watched_paths: HashMap<LanguageServerId, HashMap<WorktreeId, GlobSet>>,
    language_server_watcher_registrations:
        HashMap<LanguageServerId, HashMap<String, Vec<FileSystemWatcher>>>,

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
    buffers_being_formatted: HashSet<BufferId>,
}

impl LspStore {
    pub fn new(
        buffer_store: Model<BufferStore>,
        worktree_store: Model<WorktreeStore>,
        languages: Arc<LanguageRegistry>,
        client: AnyProtoClient,
        remote_id: Option<u64>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        cx.on_app_quit(Self::shutdown_language_servers);
        Self {
            client,
            remote_id,
            buffer_store,
            worktree_store,
            languages,
            buffer_snapshots: Default::default(),
            supplementary_language_servers: Default::default(),
            language_servers: Default::default(),
            language_server_ids: Default::default(),
            language_server_statuses: Default::default(),
            last_formatting_failure: Default::default(),
            last_workspace_edits_by_language_server: Default::default(),
            language_server_watched_paths: Default::default(),
            language_server_watcher_registrations: Default::default(),

            next_diagnostic_group_id: Default::default(),
            diagnostic_summaries: Default::default(),
            diagnostics: Default::default(),
            buffers_being_formatted: Default::default(),
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

    pub(crate) fn send_diagnostic_summaries(&self, cx: &mut ModelContext<Self>) -> Result<()> {
        let Some(project_id) = self.remote_id else {
            return Ok(());
        };
        let worktrees = self.worktree_store.read(cx).worktrees().collect::<Vec<_>>();
        for worktree in worktrees {
            worktree.update(cx, |worktree, cx| {
                if let Some(summaries) = self.diagnostic_summaries.get(&worktree.id()) {
                    for (path, summaries) in summaries {
                        for (&server_id, summary) in summaries {
                            self.client.send(proto::UpdateDiagnosticSummary {
                                project_id,
                                worktree_id: worktree.id().to_proto(),
                                summary: Some(summary.to_proto(server_id, path)),
                            })?;
                        }
                    }
                }

                worktree.observe_updates(project_id, cx, {
                    let client = self.client.clone();
                    move |update| client.request(update).map(|result| result.is_ok())
                });

                anyhow::Ok(())
            })?;
        }
        anyhow::Ok(())
    }

    pub(crate) fn started_language_servers(&self) -> Vec<(WorktreeId, LanguageServerName)> {
        self.language_server_ids.keys().cloned().collect()
    }

    pub(crate) fn shared(&mut self, project_id: u64, cx: &mut ModelContext<Self>) {
        self.remote_id = Some(project_id);

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
    }

    pub(crate) fn rejoined(&mut self, language_servers: Vec<proto::LanguageServer>) {
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

    pub(crate) fn register_buffer_with_language_servers(
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

    pub fn update_diagnostic_entries(
        &mut self,
        server_id: LanguageServerId,
        abs_path: PathBuf,
        version: Option<i32>,
        diagnostics: Vec<DiagnosticEntry<Unclipped<PointUtf16>>>,
        cx: &mut ModelContext<Self>,
    ) -> Result<Option<ProjectPath>, anyhow::Error> {
        let (worktree, relative_path) =
            self.worktree_store
                .read(cx)
                .find_worktree(&abs_path, cx)
                .ok_or_else(|| anyhow!("no worktree found for diagnostics path {abs_path:?}"))?;

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
        Ok(if updated { Some(project_path) } else { None })
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
            if let Some(project_id) = self.remote_id {
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

    pub(crate) fn language_servers_for_buffer(
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

    pub(crate) fn cancel_language_server_work(
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

    pub(crate) fn language_server_ids_for_buffer(
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

#[derive(Debug)]
pub enum LanguageServerToQuery {
    Primary,
    Other(LanguageServerId),
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
pub enum LanguageServerLogType {
    Log(MessageType),
    Trace(Option<String>),
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
