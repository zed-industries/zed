use anyhow::{Context as _, Result, anyhow};
use client::ProjectId;
use collections::HashMap;
use collections::HashSet;
use language::{BufferId, File};
use lsp::LanguageServerId;

use extension::ExtensionHostProxy;
use extension_host::headless_host::HeadlessExtensionStore;
use fs::Fs;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, PromptLevel};
use http_client::HttpClient;
use language::{
    Buffer, BufferEvent, LanguageRegistry,
    proto::{serialize_operation, split_operations},
};
use node_runtime::NodeRuntime;
use project::{
    AgentRegistryStore, LspStore, LspStoreEvent, ManifestTree, PrettierStore, ProjectEnvironment,
    ProjectPath, ProjectTransaction, ToolchainStore, WorktreeId,
    agent_server_store::AgentServerStore,
    buffer_store::{BufferStore, BufferStoreEvent, PeerBufferAccess, SharedBuffer},
    context_server_store::ContextServerStore,
    debugger::{
        breakpoint_store::{BreakpointStore, BreakpointStoreEvent, BreakpointUpdatedReason},
        dap_store::{DapStore, DapStoreEvent},
    },
    git_store::{GitStore, GitStoreEvent, RepositoryId, RepositorySnapshot},
    image_store::ImageId,
    lsp_store::log_store::{self, GlobalLogStore, LanguageServerKind, LogKind},
    project_settings::{self, SettingsObserver, SettingsObserverEvent},
    search::SearchQuery,
    task_store::TaskStore,
    trusted_worktrees::{PathTrust, RemoteHostLocation, TrustedWorktrees},
    worktree_store::{WorktreeIdCounter, WorktreeStore, WorktreeStoreEvent},
};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, REMOTE_SERVER_PEER_ID, REMOTE_SERVER_PROJECT_ID},
};
use smol::process::Child;

use settings::initial_server_settings_content;
use std::{
    num::NonZeroU64,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
    time::Instant,
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind};
use util::{ResultExt, debug_panic, paths::PathStyle, rel_path::RelPath};
use worktree::Worktree;

pub struct HeadlessProject {
    pub fs: Arc<dyn Fs>,
    pub session: AnyProtoClient,
    pub worktree_store: Entity<WorktreeStore>,
    pub buffer_store: Entity<BufferStore>,
    pub lsp_store: Entity<LspStore>,
    pub task_store: Entity<TaskStore>,
    pub dap_store: Entity<DapStore>,
    pub breakpoint_store: Entity<BreakpointStore>,
    pub agent_server_store: Entity<AgentServerStore>,
    pub context_server_store: Entity<ContextServerStore>,
    pub settings_observer: Entity<SettingsObserver>,
    pub next_entry_id: Arc<AtomicUsize>,
    pub languages: Arc<LanguageRegistry>,
    pub extensions: Entity<HeadlessExtensionStore>,
    pub git_store: Entity<GitStore>,
    pub environment: Entity<ProjectEnvironment>,
    pub profiling_collector: gpui::ProfilingCollector,
    // Used mostly to keep alive the toolchain store for RPC handlers.
    // Local variant is used within LSP store, but that's a separate entity.
    pub _toolchain_store: Entity<ToolchainStore>,
    pub kernels: HashMap<String, Child>,
    /// Strong handles for every worktree the headless server tracks. The
    /// host-side `WorktreeStore` only holds weak references; this list is
    /// what keeps them alive while the connected client cares about them.
    /// Headless always retains all worktrees (no visibility distinction).
    worktrees: Vec<Entity<Worktree>>,
    /// Last `RepositorySnapshot` we forwarded to the connected zed client
    /// for each repository, used to compute incremental
    /// `proto::UpdateRepository` payloads. Mirrors `Project::
    /// git_repository_snapshots_for_peer`.
    git_repository_snapshots_for_peer: HashMap<RepositoryId, RepositorySnapshot>,
    /// Per-peer per-buffer state for the connected zed client. Mirrors
    /// `Project::shared_buffers` for the headless side; created here in
    /// BufferStore Phase 1 because the headless server can't reuse the
    /// host-side `BufferStore::shared_buffers` (which moved up to
    /// `Project`).
    shared_buffers: HashMap<proto::PeerId, HashMap<BufferId, SharedBuffer>>,
}

pub struct HeadlessAppState {
    pub session: AnyProtoClient,
    pub fs: Arc<dyn Fs>,
    pub http_client: Arc<dyn HttpClient>,
    pub node_runtime: NodeRuntime,
    pub languages: Arc<LanguageRegistry>,
    pub extension_host_proxy: Arc<ExtensionHostProxy>,
    pub startup_time: Instant,
}

impl HeadlessProject {
    pub fn init(cx: &mut App) {
        settings::init(cx);
        log_store::init(true, cx);
    }

    pub fn new(
        HeadlessAppState {
            session,
            fs,
            http_client,
            node_runtime,
            languages,
            extension_host_proxy: proxy,
            startup_time,
        }: HeadlessAppState,
        init_worktree_trust: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        debug_adapter_extension::init(proxy.clone(), cx);
        languages::init(languages.clone(), fs.clone(), node_runtime.clone(), cx);

        let worktree_store = cx.new(|cx| {
            let mut store = WorktreeStore::local(fs.clone(), WorktreeIdCounter::get(cx));
            store.set_id_allocator(session.clone());
            store
        });
        cx.subscribe(&worktree_store, Self::on_worktree_store_event)
            .detach();

        if init_worktree_trust {
            project::trusted_worktrees::track_worktree_trust(
                worktree_store.clone(),
                None::<RemoteHostLocation>,
                Some((session.clone(), ProjectId(REMOTE_SERVER_PROJECT_ID))),
                None,
                cx,
            );
        }

        let environment =
            cx.new(|cx| ProjectEnvironment::new(None, worktree_store.downgrade(), None, true, cx));
        let manifest_tree = ManifestTree::new(worktree_store.clone(), cx);
        let toolchain_store = cx.new(|cx| {
            ToolchainStore::local(
                languages.clone(),
                worktree_store.clone(),
                environment.clone(),
                manifest_tree.clone(),
                cx,
            )
        });

        let buffer_store = cx.new(|cx| BufferStore::local(worktree_store.clone(), cx));

        let breakpoint_store =
            cx.new(|_| BreakpointStore::local(worktree_store.clone(), buffer_store.clone()));
        cx.subscribe(&breakpoint_store, Self::on_breakpoint_store_event)
            .detach();

        let dap_store = cx.new(|cx| {
            DapStore::new_local(
                http_client.clone(),
                node_runtime.clone(),
                fs.clone(),
                environment.clone(),
                toolchain_store.read(cx).as_language_toolchain_store(),
                worktree_store.clone(),
                breakpoint_store.clone(),
                true,
                cx,
            )
        });
        cx.subscribe(&dap_store, Self::on_dap_store_event).detach();

        let git_store = cx.new(|cx| {
            GitStore::local(
                &worktree_store,
                buffer_store.clone(),
                environment.clone(),
                fs.clone(),
                cx,
            )
        });
        cx.subscribe(&git_store, Self::on_git_store_event).detach();

        let prettier_store = cx.new(|cx| {
            PrettierStore::new(
                node_runtime.clone(),
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
                git_store.clone(),
                cx,
            )
        });
        let settings_observer = cx.new(|cx| {
            SettingsObserver::new_local(
                fs.clone(),
                worktree_store.clone(),
                task_store.clone(),
                true,
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
                toolchain_store
                    .read(cx)
                    .as_local_store()
                    .expect("Toolchain store to be local")
                    .clone(),
                environment.clone(),
                manifest_tree,
                languages.clone(),
                http_client.clone(),
                fs.clone(),
                cx,
            )
        });

        AgentRegistryStore::init_global(cx, fs.clone(), http_client.clone());

        let agent_server_store = cx.new(|cx| {
            let mut agent_server_store = AgentServerStore::local(
                node_runtime.clone(),
                fs.clone(),
                environment.clone(),
                http_client.clone(),
                cx,
            );
            agent_server_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            agent_server_store
        });

        let context_server_store = cx.new(|cx| {
            let mut context_server_store =
                ContextServerStore::local(worktree_store.clone(), true, cx);
            context_server_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone());
            context_server_store
        });

        cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();
        language_extension::init(
            language_extension::LspAccess::ViaLspStore(lsp_store.clone()),
            proxy.clone(),
            languages.clone(),
        );

        cx.subscribe(&buffer_store, Self::on_buffer_store_event)
            .detach();

        let extensions = HeadlessExtensionStore::new(
            fs.clone(),
            http_client.clone(),
            paths::remote_extensions_dir().to_path_buf(),
            proxy,
            node_runtime,
            cx,
        );

        // local_machine -> ssh handlers
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &worktree_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &buffer_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &cx.entity());
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &lsp_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &task_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &toolchain_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &dap_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &breakpoint_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &settings_observer);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &git_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &agent_server_store);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &context_server_store);

        session.add_request_handler(cx.weak_entity(), Self::handle_list_remote_directory);
        session.add_request_handler(cx.weak_entity(), Self::handle_get_path_metadata);
        session.add_request_handler(cx.weak_entity(), Self::handle_shutdown_remote_server);
        session.add_request_handler(cx.weak_entity(), Self::handle_ping);
        session.add_request_handler(cx.weak_entity(), Self::handle_get_processes);
        session.add_request_handler(cx.weak_entity(), Self::handle_get_remote_profiling_data);

        session.add_entity_request_handler(Self::handle_add_worktree);
        session.add_entity_request_handler(Self::handle_lsp_query);
        session.add_entity_request_handler(Self::handle_fetch);
        session.add_entity_request_handler(Self::handle_push);
        session.add_entity_request_handler(Self::handle_pull);
        session.add_entity_request_handler(Self::handle_commit);
        session.add_entity_request_handler(Self::handle_apply_code_action);
        session.add_entity_request_handler(Self::handle_apply_code_action_kind);
        session.add_entity_request_handler(Self::handle_format_buffers);
        session.add_entity_request_handler(Self::handle_open_buffer_for_symbol);
        session.add_entity_request_handler(Self::handle_register_buffer_with_language_servers);
        session.add_entity_request_handler(Self::handle_open_commit_message_buffer);
        session.add_entity_request_handler(Self::handle_rename_project_entry);
        session.add_entity_request_handler(
            Self::handle_lsp_command_with_project::<project::lsp_command::PerformRename>,
        );
        session.add_entity_request_handler(
            Self::handle_lsp_command_with_project::<
                project::lsp_store::lsp_ext_command::GoToParentModule,
            >,
        );
        session.add_entity_request_handler(
            Self::handle_lsp_command_with_project::<
                project::lsp_store::lsp_ext_command::GetLspRunnables,
            >,
        );
        session.add_request_handler(cx.weak_entity(), Self::handle_remove_worktree);

        session.add_entity_request_handler(Self::handle_open_buffer_by_path);
        session.add_entity_request_handler(Self::handle_open_new_buffer);
        session.add_entity_request_handler(Self::handle_find_search_candidates);
        session.add_entity_request_handler(Self::handle_open_server_settings);
        session.add_entity_request_handler(Self::handle_get_directory_environment);
        session.add_entity_message_handler(Self::handle_toggle_lsp_logs);
        session.add_entity_request_handler(Self::handle_open_image_by_path);
        session.add_entity_request_handler(Self::handle_trust_worktrees);
        session.add_entity_request_handler(Self::handle_restrict_worktrees);
        session.add_entity_request_handler(Self::handle_download_file_by_path);

        session.add_entity_message_handler(Self::handle_find_search_candidates_cancel);
        session.add_entity_request_handler(BufferStore::handle_update_buffer);
        // handle_close_buffer / handle_reload_buffers moved to Project /
        // HeadlessProject in BufferStore Phase 1.
        session.add_entity_message_handler(Self::handle_close_buffer);
        session.add_entity_request_handler(Self::handle_reload_buffers);

        session.add_request_handler(
            extensions.downgrade(),
            HeadlessExtensionStore::handle_sync_extensions,
        );
        session.add_request_handler(
            extensions.downgrade(),
            HeadlessExtensionStore::handle_install_extension,
        );

        session.add_request_handler(cx.weak_entity(), Self::handle_spawn_kernel);
        session.add_request_handler(cx.weak_entity(), Self::handle_kill_kernel);

        BufferStore::init(&session);
        WorktreeStore::init(&session);
        SettingsObserver::init(&session);
        LspStore::init(&session);
        TaskStore::init(Some(&session));
        ToolchainStore::init(&session);
        DapStore::init(&session, cx);
        // todo(debugger): Re init breakpoint store when we set it up for collab
        BreakpointStore::init(&session);
        GitStore::init(&session);
        AgentServerStore::init_headless(&session);
        ContextServerStore::init_headless(&session);

        HeadlessProject {
            next_entry_id: Default::default(),
            session,
            settings_observer,
            fs,
            worktree_store,
            buffer_store,
            lsp_store,
            task_store,
            dap_store,
            breakpoint_store,
            agent_server_store,
            context_server_store,
            languages,
            extensions,
            git_store,
            environment,
            profiling_collector: gpui::ProfilingCollector::new(startup_time),
            _toolchain_store: toolchain_store,
            kernels: Default::default(),
            worktrees: Vec::new(),
            git_repository_snapshots_for_peer: HashMap::default(),
            shared_buffers: HashMap::default(),
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            BufferEvent::Operation {
                operation,
                is_local: true,
            } => cx
                .background_spawn(self.session.request(proto::UpdateBuffer {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                    buffer_id: buffer.read(cx).remote_id().to_proto(),
                    operations: vec![serialize_operation(operation)],
                }))
                .detach(),
            BufferEvent::ReloadNeeded => {
                // The server's worktree scanner observed a content
                // change for this buffer; reload from disk. Mirrors
                // `Project::on_buffer_event`'s ReloadNeeded branch.
                // Without this the buffer's contents stay frozen at
                // the load-time text even though `file_updated` ran
                // and emitted `ReloadNeeded` — manifesting as
                // `test_remote_reload` seeing the old buffer text.
                self.buffer_store.update(cx, |buffer_store, cx| {
                    let mut buffers = HashSet::default();
                    buffers.insert(buffer);
                    buffer_store
                        .reload_buffers(buffers, true, cx)
                        .detach_and_log_err(cx);
                });
            }
            _ => {}
        }
    }

    fn on_worktree_store_event(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(worktree) => {
                // Pin the worktree on our side; the host registry only holds
                // a weak reference. Headless always retains.
                self.worktrees.push(worktree.clone());
                // Set up an observer that streams worktree updates to the
                // connected zed client.
                let session = self.session.clone();
                worktree.update(cx, move |worktree, cx| {
                    worktree.observe_updates(REMOTE_SERVER_PROJECT_ID, cx, move |update| {
                        let session = session.clone();
                        async move { session.send(update).log_err().is_some() }
                    });
                });
            }
            WorktreeStoreEvent::WorktreeRemoved(entity_id, _) => {
                self.worktrees.retain(|w| w.entity_id() != *entity_id);
            }
            WorktreeStoreEvent::WorktreeMetadataChanged => {
                let metadata = worktree_store.read(cx).worktree_metadata_protos(cx);
                self.session
                    .send(rpc::proto::UpdateProject {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        worktrees: metadata,
                    })
                    .log_err();
            }
            WorktreeStoreEvent::WorktreeUpdateSent(worktree) => {
                if let Some(summaries) = self.lsp_store.read(cx).diagnostic_summaries_for_worktree(
                    worktree.read(cx).id(),
                    REMOTE_SERVER_PROJECT_ID,
                ) {
                    self.session.send(summaries).log_err();
                }
            }
            _ => {}
        }
    }

    fn on_settings_observer_event(
        &mut self,
        _: Entity<SettingsObserver>,
        event: &SettingsObserverEvent,
        _cx: &mut Context<Self>,
    ) {
        if let SettingsObserverEvent::LocalSettingsApplied {
            worktree_id,
            path,
            kind,
            content,
        } = event
        {
            self.session
                .send(proto::UpdateWorktreeSettings {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                    worktree_id: worktree_id.to_proto(),
                    path: path.to_proto(),
                    content: content.clone(),
                    kind: Some(project_settings::local_settings_kind_to_proto(*kind).into()),
                    outside_worktree: Some(path.is_outside_worktree()),
                })
                .log_err();
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
                cx.subscribe(buffer, Self::on_buffer_event).detach();
            }
            BufferStoreEvent::LocalBufferReloaded(buffer) => {
                let buffer = buffer.read(cx);
                self.session
                    .send(rpc::proto::BufferReloaded {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        buffer_id: buffer.remote_id().to_proto(),
                        version: language::proto::serialize_version(&buffer.version()),
                        mtime: buffer.saved_mtime().map(|t| t.into()),
                        line_ending: language::proto::serialize_line_ending(buffer.line_ending())
                            as i32,
                    })
                    .log_err();
            }
            BufferStoreEvent::UpdateBufferFileForwarded { buffer_id, file } => {
                self.session
                    .send(rpc::proto::UpdateBufferFile {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        buffer_id: buffer_id.to_proto(),
                        file: file.clone(),
                    })
                    .log_err();
            }
            BufferStoreEvent::BufferSavedForwarded {
                buffer_id,
                version,
                mtime,
            } => {
                self.session
                    .send(rpc::proto::BufferSaved {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        buffer_id: buffer_id.to_proto(),
                        version: version.clone(),
                        mtime: mtime.clone(),
                    })
                    .log_err();
            }
            BufferStoreEvent::BufferReloadedForwarded {
                buffer_id,
                version,
                mtime,
                line_ending,
            } => {
                self.session
                    .send(rpc::proto::BufferReloaded {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        buffer_id: buffer_id.to_proto(),
                        version: version.clone(),
                        mtime: mtime.clone(),
                        line_ending: *line_ending,
                    })
                    .log_err();
            }
            _ => {}
        }
    }

    fn on_dap_store_event(
        &mut self,
        _: Entity<DapStore>,
        event: &DapStoreEvent,
        _cx: &mut Context<Self>,
    ) {
        if let DapStoreEvent::LogToDebugConsole {
            session_id,
            message,
        } = event
        {
            self.session
                .send(proto::LogToDebugConsole {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                    session_id: *session_id,
                    message: message.clone(),
                })
                .log_err();
        }
    }

    fn on_git_store_event(
        &mut self,
        _: Entity<GitStore>,
        event: &GitStoreEvent,
        _cx: &mut Context<Self>,
    ) {
        match event {
            GitStoreEvent::RepositorySnapshotForDownstream(snapshot) => {
                let update =
                    if let Some(old) = self.git_repository_snapshots_for_peer.get(&snapshot.id) {
                        snapshot.build_update(old, REMOTE_SERVER_PROJECT_ID)
                    } else {
                        snapshot.initial_update(REMOTE_SERVER_PROJECT_ID)
                    };
                for chunk in proto::split_repository_update(update) {
                    self.session.send(chunk).log_err();
                }
                self.git_repository_snapshots_for_peer
                    .insert(snapshot.id, snapshot.clone());
            }
            GitStoreEvent::RepositorySnapshotRemovedForDownstream(id) => {
                self.session
                    .send(proto::RemoveRepository {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        id: id.to_proto(),
                    })
                    .log_err();
                self.git_repository_snapshots_for_peer.remove(id);
            }
            GitStoreEvent::ForwardRepositoryUpdate(update) => {
                let mut update = update.clone();
                update.project_id = REMOTE_SERVER_PROJECT_ID;
                self.session.send(update).log_err();
            }
            GitStoreEvent::ForwardRepositoryRemove(update) => {
                let mut update = update.clone();
                update.project_id = REMOTE_SERVER_PROJECT_ID;
                self.session.send(update).log_err();
            }
            GitStoreEvent::DiffBasesUpdatedForDownstream(update) => {
                let mut update = update.clone();
                update.project_id = REMOTE_SERVER_PROJECT_ID;
                self.session.send(update).log_err();
            }
            _ => {}
        }
    }

    fn on_breakpoint_store_event(
        &mut self,
        breakpoint_store: Entity<BreakpointStore>,
        event: &BreakpointStoreEvent,
        cx: &mut Context<Self>,
    ) {
        if let BreakpointStoreEvent::BreakpointsUpdated(path, BreakpointUpdatedReason::Toggled) =
            event
        {
            let proto = breakpoint_store
                .read(cx)
                .breakpoints_for_file_proto(path, REMOTE_SERVER_PROJECT_ID);
            let _ = self.session.send(proto);
        }
    }

    fn on_lsp_store_event(
        &mut self,
        lsp_store: Entity<LspStore>,
        event: &LspStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            LspStoreEvent::LanguageServerAdded(id, name, worktree_id) => {
                let log_store = cx
                    .try_global::<GlobalLogStore>()
                    .map(|lsp_logs| lsp_logs.0.clone());
                if let Some(log_store) = log_store {
                    log_store.update(cx, |log_store, cx| {
                        log_store.add_language_server(
                            LanguageServerKind::LocalSsh {
                                session: self.session.clone(),
                                project_id: REMOTE_SERVER_PROJECT_ID,
                            },
                            *id,
                            Some(name.clone()),
                            *worktree_id,
                            lsp_store.read(cx).language_server_for_id(*id),
                            cx,
                        );
                    });
                }
                let lsp_store = lsp_store.read(cx);
                if let Some(capabilities) = lsp_store.lsp_server_capabilities.get(id) {
                    self.session
                        .send(proto::StartLanguageServer {
                            project_id: REMOTE_SERVER_PROJECT_ID,
                            server: Some(proto::LanguageServer {
                                id: id.to_proto(),
                                name: name.to_string(),
                                worktree_id: worktree_id.map(|id| id.to_proto()),
                            }),
                            capabilities: serde_json::to_string(capabilities)
                                .expect("serializing server LSP capabilities"),
                        })
                        .log_err();
                }
            }
            LspStoreEvent::LanguageServerRemoved(id) => {
                let log_store = cx
                    .try_global::<GlobalLogStore>()
                    .map(|lsp_logs| lsp_logs.0.clone());
                if let Some(log_store) = log_store {
                    log_store.update(cx, |log_store, cx| {
                        log_store.remove_language_server(*id, cx);
                    });
                }
            }
            LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message,
            } => {
                self.session
                    .send(proto::UpdateLanguageServer {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        server_name: name.as_ref().map(|name| name.to_string()),
                        language_server_id: language_server_id.to_proto(),
                        variant: Some(message.clone()),
                    })
                    .log_err();
            }
            LspStoreEvent::Notification(message) => {
                self.session
                    .send(proto::Toast {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        notification_id: "lsp".to_string(),
                        message: message.clone(),
                    })
                    .log_err();
            }
            LspStoreEvent::RefreshInlayHints {
                server_id,
                request_id,
            } => {
                self.session
                    .send(proto::RefreshInlayHints {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        server_id: server_id.to_proto(),
                        request_id: request_id.map(|id| id as u64),
                    })
                    .log_err();
            }
            LspStoreEvent::RefreshSemanticTokens {
                server_id,
                request_id,
            } => {
                self.session
                    .send(proto::RefreshSemanticTokens {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        server_id: server_id.to_proto(),
                        request_id: request_id.map(|id| id as u64),
                    })
                    .log_err();
            }
            LspStoreEvent::RefreshCodeLens => {
                self.session
                    .send(proto::RefreshCodeLens {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                    })
                    .log_err();
            }
            LspStoreEvent::PullWorkspaceDiagnosticsRequested { server_id } => {
                self.session
                    .send(proto::PullWorkspaceDiagnostics {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        server_id: server_id.to_proto(),
                    })
                    .log_err();
            }
            LspStoreEvent::DiagnosticsSummariesUpdated {
                worktree_id,
                summary,
                more_summaries,
            } => {
                self.session
                    .send(proto::UpdateDiagnosticSummary {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        worktree_id: worktree_id.to_proto(),
                        summary: Some(summary.clone()),
                        more_summaries: more_summaries.clone(),
                    })
                    .log_err();
            }
            LspStoreEvent::LanguageServerPrompt(prompt) => {
                let request = self.session.request(proto::LanguageServerPromptRequest {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                    actions: prompt
                        .actions
                        .iter()
                        .map(|action| action.title.to_string())
                        .collect(),
                    level: Some(prompt_to_proto(prompt)),
                    lsp_name: prompt.lsp_name.clone(),
                    message: prompt.message.clone(),
                });
                let prompt = prompt.clone();
                cx.background_spawn(async move {
                    let response = request.await?;
                    if let Some(action_response) = response.action_response {
                        prompt.respond(action_response as usize).await;
                    }
                    anyhow::Ok(())
                })
                .detach();
            }
            LspStoreEvent::ApplyWorkspaceEditRequested {
                server_id,
                params,
                response,
            } => {
                // HeadlessProject has no UI, hence no active entry; pass
                // `None` so all snippet edits are baked in as plain text.
                let lsp_store = lsp_store.downgrade();
                let server_id = *server_id;
                let params = params.clone();
                let response = response.clone();
                cx.spawn(async move |_, cx| {
                    let result = project::lsp_store::LocalLspStore::on_lsp_workspace_edit(
                        lsp_store, params, server_id, None, cx,
                    )
                    .await;
                    response.send(result).await.ok();
                })
                .detach();
            }
            _ => {}
        }
    }

    /// Streams a buffer's initial state and pending operations to the
    /// connected zed client. Mirrors `Project::create_buffer_for_peer` for
    /// the headless side; the field used to live on `BufferStore` before
    /// Phase 1.
    pub fn create_buffer_for_peer(
        &mut self,
        buffer: &Entity<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> gpui::Task<Result<()>> {
        let buffer_id = buffer.read(cx).remote_id();
        let shared_buffers = self.shared_buffers.entry(peer_id).or_default();
        if shared_buffers.contains_key(&buffer_id) {
            return gpui::Task::ready(Ok(()));
        }
        shared_buffers.insert(
            buffer_id,
            SharedBuffer {
                buffer: buffer.clone(),
                lsp_handle: None,
            },
        );

        let project_id = REMOTE_SERVER_PROJECT_ID;
        let client = self.session.clone();
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

    pub fn forget_shared_buffers_for(&mut self, peer_id: &proto::PeerId) {
        self.shared_buffers.remove(peer_id);
    }

    pub fn has_shared_buffers(&self) -> bool {
        !self.shared_buffers.is_empty()
    }

    pub fn register_shared_lsp_handle(
        &mut self,
        peer_id: proto::PeerId,
        buffer_id: BufferId,
        handle: project::lsp_store::OpenLspBufferHandle,
    ) {
        if let Some(shared_buffers) = self.shared_buffers.get_mut(&peer_id)
            && let Some(buffer) = shared_buffers.get_mut(&buffer_id)
        {
            buffer.lsp_handle = Some(handle);
            return;
        }
        debug_panic!("tried to register shared lsp handle, but buffer was not shared")
    }

    /// Mirrors `Project::handle_reload_buffers` for the headless side.
    /// The handler was moved off `BufferStore` in BufferStore Phase 1,
    /// but the headless half of that move was missed — the remote
    /// server had no `ReloadBuffers` handler registered, so explicit
    /// `project.reload_buffers(...)` calls from the client failed with
    /// "no handler registered for ReloadBuffers" (and any consumer that
    /// went through this RPC path silently broke after the move).
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
                buffers.insert(this.buffer_store.read(cx).get_existing(buffer_id)?);
            }
            anyhow::Ok(this.buffer_store.update(cx, |buffer_store, cx| {
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

    async fn handle_close_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CloseBuffer>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let peer_id = envelope.sender_id;
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        this.update(&mut cx, |this, _| {
            if let Some(shared) = this.shared_buffers.get_mut(&peer_id)
                && shared.remove(&buffer_id).is_some()
            {
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

    /// Forwards `proto::LspQuery` rpc to the LSP store with the headless
    /// session as the downstream peer. Mirrors `Project::handle_lsp_query`
    /// for the headless side.
    async fn handle_lsp_query(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::LspQuery>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let (lsp_store, session) = this.read_with(&cx, |this, _| {
            (this.lsp_store.clone(), this.session.clone())
        });
        project::LspStore::process_lsp_query::<Self>(
            lsp_store,
            this.downgrade(),
            session,
            REMOTE_SERVER_PROJECT_ID,
            envelope,
            cx,
        )
        .await
    }

    /// Forwards `proto::Fetch` rpc to the git store with the headless
    /// session as the downstream peer for askpass.
    async fn handle_fetch(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Fetch>,
        cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let (git_store, session) = this.read_with(&cx, |this, _| {
            (this.git_store.clone(), this.session.clone())
        });
        GitStore::process_fetch(git_store, session, envelope, cx).await
    }

    /// Forwards `proto::Push` rpc. See [`Self::handle_fetch`].
    async fn handle_push(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Push>,
        cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let (git_store, session) = this.read_with(&cx, |this, _| {
            (this.git_store.clone(), this.session.clone())
        });
        GitStore::process_push(git_store, session, envelope, cx).await
    }

    /// Forwards `proto::Pull` rpc. See [`Self::handle_fetch`].
    async fn handle_pull(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Pull>,
        cx: AsyncApp,
    ) -> Result<proto::RemoteMessageResponse> {
        let (git_store, session) = this.read_with(&cx, |this, _| {
            (this.git_store.clone(), this.session.clone())
        });
        GitStore::process_pull(git_store, session, envelope, cx).await
    }

    /// Forwards `proto::Commit` rpc. See [`Self::handle_fetch`].
    async fn handle_commit(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Commit>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let (git_store, session) = this.read_with(&cx, |this, _| {
            (this.git_store.clone(), this.session.clone())
        });
        GitStore::process_commit(git_store, session, envelope, cx).await
    }

    async fn handle_apply_code_action(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeAction>,
        mut cx: AsyncApp,
    ) -> Result<proto::ApplyCodeActionResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let lsp_store = this.read_with(&cx, |this, _| this.lsp_store.clone());
        // HeadlessProject has no UI, hence no active entry; pass `None`.
        let project_transaction =
            LspStore::process_apply_code_action(lsp_store, envelope, None, cx.clone()).await?;
        let serialized = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::ApplyCodeActionResponse {
            transaction: Some(serialized),
        })
    }

    async fn handle_apply_code_action_kind(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ApplyCodeActionKind>,
        mut cx: AsyncApp,
    ) -> Result<proto::ApplyCodeActionKindResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let lsp_store = this.read_with(&cx, |this, _| this.lsp_store.clone());
        // HeadlessProject has no UI, hence no active entry; pass `None`.
        let project_transaction =
            LspStore::process_apply_code_action_kind(lsp_store, envelope, None, cx.clone()).await?;
        let serialized = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::ApplyCodeActionKindResponse {
            transaction: Some(serialized),
        })
    }

    async fn handle_format_buffers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FormatBuffers>,
        mut cx: AsyncApp,
    ) -> Result<proto::FormatBuffersResponse> {
        let sender_id = envelope.original_sender_id().unwrap_or_default();
        let lsp_store = this.read_with(&cx, |this, _| this.lsp_store.clone());
        let project_transaction =
            LspStore::process_format_buffers(lsp_store, envelope, cx.clone()).await?;
        let serialized = this.update(&mut cx, |this, cx| {
            this.serialize_project_transaction_for_peer(project_transaction, sender_id, cx)
        });
        Ok(proto::FormatBuffersResponse {
            transaction: Some(serialized),
        })
    }

    async fn handle_open_buffer_for_symbol(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenBufferForSymbol>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferForSymbolResponse> {
        let peer_id = envelope.original_sender_id().unwrap_or_default();
        let lsp_store = this.read_with(&cx, |this, _| this.lsp_store.clone());
        let buffer =
            LspStore::process_open_buffer_for_symbol(lsp_store, envelope, cx.clone()).await?;
        this.update(&mut cx, |this, cx| {
            let is_private = buffer
                .read(cx)
                .file()
                .map(|f| f.is_private())
                .unwrap_or_default();
            if is_private {
                Err(anyhow!(rpc::ErrorCode::UnsharedItem))
            } else {
                this.create_buffer_for_peer(&buffer, peer_id, cx)
                    .detach_and_log_err(cx);
                let buffer_id = buffer.read(cx).remote_id().to_proto();
                Ok(proto::OpenBufferForSymbolResponse { buffer_id })
            }
        })
    }

    /// Forwards `proto::RenameProjectEntry`. Mirrors
    /// `Project::handle_rename_project_entry`. HeadlessProject has no UI
    /// and therefore no active entry; pass `None`.
    async fn handle_rename_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RenameProjectEntry>,
        cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let lsp_store = this.read_with(&cx, |this, _| this.lsp_store.clone());
        LspStore::process_rename_project_entry(lsp_store, envelope, None, cx).await
    }

    async fn handle_register_buffer_with_language_servers(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RegisterBufferWithLanguageServers>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let peer_id = envelope.original_sender_id.unwrap_or(envelope.sender_id);
        let lsp_store = this.read_with(&cx, |this, _| this.lsp_store.clone());
        let registered =
            LspStore::process_register_buffer_with_language_servers(lsp_store, envelope, &mut cx)?;
        if let Some((buffer_id, handle)) = registered {
            this.update(&mut cx, |this, _| {
                this.register_shared_lsp_handle(peer_id, buffer_id, handle);
            });
        }
        Ok(proto::Ack {})
    }

    /// Generic wrapper for LSP commands that need `HeadlessProject` access
    /// for `T::response_to_proto_project` (per-peer buffer sharing).
    /// Mirrors `Project::handle_lsp_command_with_project` for the headless
    /// side.
    async fn handle_lsp_command_with_project<T: project::lsp_command::LspCommand>(
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
        let lsp_store = this.read_with(&cx, |this, _| this.lsp_store.clone());
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
        // HeadlessProject has no UI, hence no active entry; pass `None`
        // (no-op for commands other than `PerformRename`).
        request.set_active_entry(None);
        let response = lsp_store
            .update(&mut cx, |lsp_store, cx| {
                lsp_store.request_lsp(
                    buffer_handle.clone(),
                    project::LanguageServerToQuery::FirstCapable,
                    request,
                    cx,
                )
            })
            .await?;
        this.update(&mut cx, |this, cx| {
            Ok(T::response_to_proto_project(
                response,
                lsp_store.clone(),
                this,
                sender_id,
                &buffer_handle.read(cx).version(),
                cx,
            ))
        })
    }

    async fn handle_open_commit_message_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenCommitMessageBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let peer_id = envelope.original_sender_id.unwrap_or(envelope.sender_id);
        let git_store = this.read_with(&cx, |this, _| this.git_store.clone());
        let buffer =
            GitStore::process_open_commit_message_buffer(git_store, envelope, cx.clone()).await?;
        let buffer_id = buffer.read_with(&cx, |buffer, _| buffer.remote_id());
        this.update(&mut cx, |this, cx| {
            this.create_buffer_for_peer(&buffer, peer_id, cx)
                .detach_and_log_err(cx);
        });
        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    pub async fn handle_add_worktree(
        this: Entity<Self>,
        message: TypedEnvelope<proto::AddWorktree>,
        mut cx: AsyncApp,
    ) -> Result<proto::AddWorktreeResponse> {
        use client::ErrorCodeExt;
        let fs = this.read_with(&cx, |this, _| this.fs.clone());
        let path = PathBuf::from(shellexpand::tilde(&message.payload.path).to_string());

        let canonicalized = match fs.canonicalize(&path).await {
            Ok(path) => path,
            Err(e) => {
                let mut parent = path
                    .parent()
                    .ok_or(e)
                    .with_context(|| format!("{path:?} does not exist"))?;
                if parent == Path::new("") {
                    parent = util::paths::home_dir();
                }
                let parent = fs.canonicalize(parent).await.map_err(|_| {
                    anyhow!(
                        proto::ErrorCode::DevServerProjectPathDoesNotExist
                            .with_tag("path", path.to_string_lossy().as_ref())
                    )
                })?;
                if let Some(file_name) = path.file_name() {
                    parent.join(file_name)
                } else {
                    parent
                }
            }
        };
        let next_worktree_id = this
            .update(&mut cx, |this, cx| {
                this.worktree_store
                    .update(cx, |worktree_store, _| worktree_store.next_worktree_id())
            })
            .await?;
        let worktree = this
            .read_with(&cx.clone(), |this, _| {
                Worktree::local(
                    Arc::from(canonicalized.as_path()),
                    message.payload.visible,
                    this.fs.clone(),
                    this.next_entry_id.clone(),
                    true,
                    next_worktree_id,
                    &mut cx,
                )
            })
            .await?;

        let response = this.read_with(&cx, |_, cx| {
            let worktree = worktree.read(cx);
            proto::AddWorktreeResponse {
                worktree_id: worktree.id().to_proto(),
                canonicalized_path: canonicalized.to_string_lossy().into_owned(),
                root_repo_common_dir: worktree
                    .root_repo_common_dir()
                    .map(|p| p.to_string_lossy().into_owned()),
            }
        });

        // We spawn this asynchronously, so that we can send the response back
        // *before* `worktree_store.add()` can send out UpdateProject requests
        // to the client about the new worktree.
        //
        // That lets the client manage the reference/handles of the newly-added
        // worktree, before getting interrupted by an UpdateProject request.
        //
        // This fixes the problem of the client sending the AddWorktree request,
        // headless project sending out a project update, client receiving it
        // and immediately dropping the reference of the new client, causing it
        // to be dropped on the headless project, and the client only then
        // receiving a response to AddWorktree.
        cx.spawn(async move |cx| {
            this.update(cx, |this, cx| {
                this.worktree_store.update(cx, |worktree_store, cx| {
                    worktree_store.add(&worktree, cx);
                });
            });
        })
        .detach();

        Ok(response)
    }

    pub async fn handle_remove_worktree(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RemoveWorktree>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        this.update(&mut cx, |this, cx| {
            this.worktree_store.update(cx, |worktree_store, cx| {
                worktree_store.remove_worktree(worktree_id, cx);
            });
        });
        Ok(proto::Ack {})
    }

    pub async fn handle_open_buffer_by_path(
        this: Entity<Self>,
        message: TypedEnvelope<proto::OpenBufferByPath>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(message.payload.worktree_id);
        let path = RelPath::from_proto(&message.payload.path)?;
        let buffer = this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(ProjectPath { worktree_id, path }, cx)
            })
        });

        let buffer = buffer.await?;
        let buffer_id = buffer.read_with(&cx, |b, _| b.remote_id());
        this.update(&mut cx, |this, cx| {
            this.create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                .detach_and_log_err(cx);
        });

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    pub async fn handle_open_image_by_path(
        this: Entity<Self>,
        message: TypedEnvelope<proto::OpenImageByPath>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenImageResponse> {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        let worktree_id = WorktreeId::from_proto(message.payload.worktree_id);
        let path = RelPath::from_proto(&message.payload.path)?;
        let project_id = message.payload.project_id;
        use proto::create_image_for_peer::Variant;

        let (worktree_store, session) = this.read_with(&cx, |this, _| {
            (this.worktree_store.clone(), this.session.clone())
        });

        let worktree = worktree_store
            .read_with(&cx, |store, cx| store.worktree_for_id(worktree_id, cx))
            .context("worktree not found")?;

        let load_task = worktree.update(&mut cx, |worktree, cx| {
            worktree.load_binary_file(path.as_ref(), cx)
        });

        let loaded_file = load_task.await?;
        let content = loaded_file.content;
        let file = loaded_file.file;

        let proto_file = worktree.read_with(&cx, |_worktree, cx| file.to_proto(cx));
        let image_id =
            ImageId::from(NonZeroU64::new(NEXT_ID.fetch_add(1, Ordering::Relaxed)).unwrap());

        let format = image::guess_format(&content)
            .map(|f| format!("{:?}", f).to_lowercase())
            .unwrap_or_else(|_| "unknown".to_string());

        let state = proto::ImageState {
            id: image_id.to_proto(),
            file: Some(proto_file),
            content_size: content.len() as u64,
            format,
        };

        session.send(proto::CreateImageForPeer {
            project_id,
            peer_id: Some(REMOTE_SERVER_PEER_ID),
            variant: Some(Variant::State(state)),
        })?;

        const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks
        for chunk in content.chunks(CHUNK_SIZE) {
            session.send(proto::CreateImageForPeer {
                project_id,
                peer_id: Some(REMOTE_SERVER_PEER_ID),
                variant: Some(Variant::Chunk(proto::ImageChunk {
                    image_id: image_id.to_proto(),
                    data: chunk.to_vec(),
                })),
            })?;
        }

        Ok(proto::OpenImageResponse {
            image_id: image_id.to_proto(),
        })
    }

    pub async fn handle_trust_worktrees(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::TrustWorktrees>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let trusted_worktrees = cx
            .update(|cx| TrustedWorktrees::try_get_global(cx))
            .context("missing trusted worktrees")?;
        let worktree_store = this.read_with(&cx, |project, _| project.worktree_store.clone());
        trusted_worktrees.update(&mut cx, |trusted_worktrees, cx| {
            trusted_worktrees.trust(
                &worktree_store,
                envelope
                    .payload
                    .trusted_paths
                    .into_iter()
                    .filter_map(PathTrust::from_proto)
                    .collect(),
                cx,
            );
        });
        Ok(proto::Ack {})
    }

    pub async fn handle_restrict_worktrees(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::RestrictWorktrees>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let trusted_worktrees = cx
            .update(|cx| TrustedWorktrees::try_get_global(cx))
            .context("missing trusted worktrees")?;
        let worktree_store = this.read_with(&cx, |project, _| project.worktree_store.downgrade());
        trusted_worktrees.update(&mut cx, |trusted_worktrees, cx| {
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

    pub async fn handle_download_file_by_path(
        this: Entity<Self>,
        message: TypedEnvelope<proto::DownloadFileByPath>,
        mut cx: AsyncApp,
    ) -> Result<proto::DownloadFileResponse> {
        log::debug!(
            "handle_download_file_by_path: received request: {:?}",
            message.payload
        );

        let worktree_id = WorktreeId::from_proto(message.payload.worktree_id);
        let path = RelPath::from_proto(&message.payload.path)?;
        let project_id = message.payload.project_id;
        let file_id = message.payload.file_id;
        log::debug!(
            "handle_download_file_by_path: worktree_id={:?}, path={:?}, file_id={}",
            worktree_id,
            path,
            file_id
        );
        use proto::create_file_for_peer::Variant;

        let (worktree_store, session): (Entity<WorktreeStore>, AnyProtoClient) = this
            .read_with(&cx, |this, _| {
                (this.worktree_store.clone(), this.session.clone())
            });

        let worktree = worktree_store
            .read_with(&cx, |store, cx| store.worktree_for_id(worktree_id, cx))
            .context("worktree not found")?;

        let download_task = worktree.update(&mut cx, |worktree: &mut Worktree, cx| {
            worktree.load_binary_file(path.as_ref(), cx)
        });

        let downloaded_file = download_task.await?;
        let content = downloaded_file.content;
        let file = downloaded_file.file;
        log::debug!(
            "handle_download_file_by_path: file loaded, content_size={}",
            content.len()
        );

        let proto_file = worktree.read_with(&cx, |_worktree: &Worktree, cx| file.to_proto(cx));
        log::debug!(
            "handle_download_file_by_path: using client-provided file_id={}",
            file_id
        );

        let state = proto::FileState {
            id: file_id,
            file: Some(proto_file),
            content_size: content.len() as u64,
        };

        log::debug!("handle_download_file_by_path: sending State message");
        session.send(proto::CreateFileForPeer {
            project_id,
            peer_id: Some(REMOTE_SERVER_PEER_ID),
            variant: Some(Variant::State(state)),
        })?;

        const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks
        let num_chunks = content.len().div_ceil(CHUNK_SIZE);
        log::debug!(
            "handle_download_file_by_path: sending {} chunks",
            num_chunks
        );
        for (i, chunk) in content.chunks(CHUNK_SIZE).enumerate() {
            log::trace!(
                "handle_download_file_by_path: sending chunk {}/{}, size={}",
                i + 1,
                num_chunks,
                chunk.len()
            );
            session.send(proto::CreateFileForPeer {
                project_id,
                peer_id: Some(REMOTE_SERVER_PEER_ID),
                variant: Some(Variant::Chunk(proto::FileChunk {
                    file_id,
                    data: chunk.to_vec(),
                })),
            })?;
        }

        log::debug!(
            "handle_download_file_by_path: returning file_id={}",
            file_id
        );
        Ok(proto::DownloadFileResponse { file_id })
    }

    pub async fn handle_open_new_buffer(
        this: Entity<Self>,
        _message: TypedEnvelope<proto::OpenNewBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let buffer = this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.create_buffer(None, true, cx)
            })
        });

        let buffer = buffer.await?;
        let buffer_id = buffer.read_with(&cx, |b, _| b.remote_id());
        this.update(&mut cx, |this, cx| {
            this.create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                .detach_and_log_err(cx);
        });

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    async fn handle_toggle_lsp_logs(
        _: Entity<Self>,
        envelope: TypedEnvelope<proto::ToggleLspLogs>,
        cx: AsyncApp,
    ) -> Result<()> {
        let server_id = LanguageServerId::from_proto(envelope.payload.server_id);
        cx.update(|cx| {
            let log_store = cx
                .try_global::<GlobalLogStore>()
                .map(|global_log_store| global_log_store.0.clone())
                .context("lsp logs store is missing")?;
            let toggled_log_kind =
                match proto::toggle_lsp_logs::LogType::from_i32(envelope.payload.log_type)
                    .context("invalid log type")?
                {
                    proto::toggle_lsp_logs::LogType::Log => LogKind::Logs,
                    proto::toggle_lsp_logs::LogType::Trace => LogKind::Trace,
                    proto::toggle_lsp_logs::LogType::Rpc => LogKind::Rpc,
                };
            log_store.update(cx, |log_store, _| {
                log_store.toggle_lsp_logs(server_id, envelope.payload.enabled, toggled_log_kind);
            });
            anyhow::Ok(())
        })?;

        Ok(())
    }

    async fn handle_open_server_settings(
        this: Entity<Self>,
        _: TypedEnvelope<proto::OpenServerSettings>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let settings_path = paths::settings_file();
        let (worktree, path) = this
            .update(&mut cx, |this, cx| {
                this.worktree_store.update(cx, |worktree_store, cx| {
                    worktree_store.find_or_create_worktree(settings_path, false, cx)
                })
            })
            .await?;

        let buffer = this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(
                    ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path,
                    },
                    cx,
                )
            })
        });

        let buffer = buffer.await?;

        let buffer_id = this.update(&mut cx, |this, cx| {
            if buffer.read(cx).is_empty() {
                buffer.update(cx, |buffer, cx| {
                    buffer.edit([(0..0, initial_server_settings_content())], None, cx)
                });
            }

            let buffer_id = buffer.read(cx).remote_id();
            this.create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                .detach_and_log_err(cx);
            buffer_id
        });

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    async fn handle_spawn_kernel(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SpawnKernel>,
        cx: AsyncApp,
    ) -> Result<proto::SpawnKernelResponse> {
        let fs = this.update(&mut cx.clone(), |this, _| this.fs.clone());

        let mut ports = Vec::new();
        for _ in 0..5 {
            let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
            let port = listener.local_addr()?.port();
            ports.push(port);
        }

        let connection_info = serde_json::json!({
            "shell_port": ports[0],
            "iopub_port": ports[1],
            "stdin_port": ports[2],
            "control_port": ports[3],
            "hb_port": ports[4],
            "ip": "127.0.0.1",
            "key": uuid::Uuid::new_v4().to_string(),
            "transport": "tcp",
            "signature_scheme": "hmac-sha256",
            "kernel_name": envelope.payload.kernel_name,
        });

        let connection_file_content = serde_json::to_string_pretty(&connection_info)?;
        let kernel_id = uuid::Uuid::new_v4().to_string();

        let connection_file_path = std::env::temp_dir().join(format!("kernel-{}.json", kernel_id));
        fs.save(
            &connection_file_path,
            &connection_file_content.as_str().into(),
            language::LineEnding::Unix,
        )
        .await?;

        let working_directory = if envelope.payload.working_directory.is_empty() {
            std::env::current_dir()
                .ok()
                .map(|p| p.to_string_lossy().into_owned())
        } else {
            Some(envelope.payload.working_directory)
        };

        // Spawn kernel (Assuming python for now, or we'd need to parse kernelspec logic here or pass the command)

        // Spawn kernel
        let spawn_kernel = |binary: &str, args: &[String]| {
            let mut command = smol::process::Command::new(binary);

            if !args.is_empty() {
                for arg in args {
                    if arg == "{connection_file}" {
                        command.arg(&connection_file_path);
                    } else {
                        command.arg(arg);
                    }
                }
            } else {
                command
                    .arg("-m")
                    .arg("ipykernel_launcher")
                    .arg("-f")
                    .arg(&connection_file_path);
            }

            // This ensures subprocesses spawned from the kernel use the correct Python environment
            let python_bin_dir = std::path::Path::new(binary).parent();
            if let Some(bin_dir) = python_bin_dir {
                if let Some(path_var) = std::env::var_os("PATH") {
                    let mut paths = std::env::split_paths(&path_var).collect::<Vec<_>>();
                    paths.insert(0, bin_dir.to_path_buf());
                    if let Ok(new_path) = std::env::join_paths(paths) {
                        command.env("PATH", new_path);
                    }
                }

                if let Some(venv_root) = bin_dir.parent() {
                    command.env("VIRTUAL_ENV", venv_root.to_string_lossy().to_string());
                }
            }

            if let Some(wd) = &working_directory {
                command.current_dir(wd);
            }
            command.spawn()
        };

        // We need to manage the child process lifecycle
        let child = if !envelope.payload.command.is_empty() {
            spawn_kernel(&envelope.payload.command, &envelope.payload.args).context(format!(
                "failed to spawn kernel process (command: {})",
                envelope.payload.command
            ))?
        } else if let Some(venv_python) = working_directory
            .as_ref()
            .and_then(|wd| find_venv_python(wd))
        {
            let path_str = venv_python.to_string_lossy().to_string();
            spawn_kernel(&path_str, &[]).context(format!(
                "failed to spawn kernel process (venv: {})",
                path_str
            ))?
        } else {
            spawn_kernel("python3", &[])
                .or_else(|_| spawn_kernel("python", &[]))
                .context("failed to spawn kernel process (tried python3 and python)")?
        };

        this.update(&mut cx.clone(), |this, _cx| {
            this.kernels.insert(kernel_id.clone(), child);
        });

        Ok(proto::SpawnKernelResponse {
            kernel_id,
            connection_file: connection_file_content,
        })
    }

    async fn handle_kill_kernel(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::KillKernel>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let kernel_id = envelope.payload.kernel_id;
        let child = this.update(&mut cx, |this, _| this.kernels.remove(&kernel_id));
        if let Some(mut child) = child {
            child.kill().log_err();
        }
        Ok(proto::Ack {})
    }

    async fn handle_find_search_candidates(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidates>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        use futures::stream::StreamExt as _;

        let peer_id = envelope.original_sender_id.unwrap_or(envelope.sender_id);
        let message = envelope.payload;
        let query = SearchQuery::from_proto(
            message.query.context("missing query field")?,
            PathStyle::local(),
        )?;

        let project_id = message.project_id;
        let handle = message.handle;
        let buffer_store = this.read_with(&cx, |this, _| this.buffer_store.clone());
        let client = this.read_with(&cx, |this, _| this.session.clone());
        let task = cx.spawn(async move |cx| {
            let results = this.update(cx, |this, cx| {
                project::Search::local(
                    this.fs.clone(),
                    this.buffer_store.clone(),
                    this.worktree_store.clone(),
                    message.limit as _,
                    cx,
                )
                .into_handle(query, cx)
                .matching_buffers(cx)
            });
            let (batcher, batches) =
                project::project_search::AdaptiveBatcher::new(cx.background_executor());
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
                    let buffer_id = buffer.read(cx).remote_id().to_proto();
                    this.create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                        .detach_and_log_err(cx);
                    buffer_id
                });
                batcher.push(buffer_id).await;
            }
            batcher.flush().await;

            sender_task.await?;

            client
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

    // Goes from client to host.
    async fn handle_find_search_candidates_cancel(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidatesCancelled>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        let buffer_store = this.read_with(&mut cx, |this, _| this.buffer_store.clone());
        BufferStore::handle_find_search_candidates_cancel(buffer_store, envelope, cx).await
    }

    async fn handle_list_remote_directory(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ListRemoteDirectory>,
        cx: AsyncApp,
    ) -> Result<proto::ListRemoteDirectoryResponse> {
        use smol::stream::StreamExt;
        let fs = cx.read_entity(&this, |this, _| this.fs.clone());
        let expanded = PathBuf::from(shellexpand::tilde(&envelope.payload.path).to_string());
        let check_info = envelope
            .payload
            .config
            .as_ref()
            .is_some_and(|config| config.is_dir);

        let mut entries = Vec::new();
        let mut entry_info = Vec::new();
        let mut response = fs.read_dir(&expanded).await?;
        while let Some(path) = response.next().await {
            let path = path?;
            if let Some(file_name) = path.file_name() {
                entries.push(file_name.to_string_lossy().into_owned());
                if check_info {
                    let is_dir = fs.is_dir(&path).await;
                    entry_info.push(proto::EntryInfo { is_dir });
                }
            }
        }
        Ok(proto::ListRemoteDirectoryResponse {
            entries,
            entry_info,
        })
    }

    async fn handle_get_path_metadata(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetPathMetadata>,
        cx: AsyncApp,
    ) -> Result<proto::GetPathMetadataResponse> {
        let fs = cx.read_entity(&this, |this, _| this.fs.clone());
        let expanded = PathBuf::from(shellexpand::tilde(&envelope.payload.path).to_string());

        let metadata = fs.metadata(&expanded).await?;
        let is_dir = metadata.map(|metadata| metadata.is_dir).unwrap_or(false);

        Ok(proto::GetPathMetadataResponse {
            exists: metadata.is_some(),
            is_dir,
            path: expanded.to_string_lossy().into_owned(),
        })
    }

    async fn handle_shutdown_remote_server(
        _this: Entity<Self>,
        _envelope: TypedEnvelope<proto::ShutdownRemoteServer>,
        cx: AsyncApp,
    ) -> Result<proto::Ack> {
        cx.spawn(async move |cx| {
            cx.update(|cx| {
                // TODO: This is a hack, because in a headless project, shutdown isn't executed
                // when calling quit, but it should be.
                cx.shutdown();
                cx.quit();
            })
        })
        .detach();

        Ok(proto::Ack {})
    }

    pub async fn handle_ping(
        _this: Entity<Self>,
        _envelope: TypedEnvelope<proto::Ping>,
        _cx: AsyncApp,
    ) -> Result<proto::Ack> {
        log::debug!("Received ping from client");
        Ok(proto::Ack {})
    }

    async fn handle_get_processes(
        _this: Entity<Self>,
        _envelope: TypedEnvelope<proto::GetProcesses>,
        _cx: AsyncApp,
    ) -> Result<proto::GetProcessesResponse> {
        let mut processes = Vec::new();
        let refresh_kind = RefreshKind::nothing().with_processes(
            ProcessRefreshKind::nothing()
                .without_tasks()
                .with_cmd(UpdateKind::Always),
        );

        for process in System::new_with_specifics(refresh_kind)
            .processes()
            .values()
        {
            let name = process.name().to_string_lossy().into_owned();
            let command = process
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().into_owned())
                .collect::<Vec<_>>();

            processes.push(proto::ProcessInfo {
                pid: process.pid().as_u32(),
                name,
                command,
            });
        }

        processes.sort_by_key(|p| p.name.clone());

        Ok(proto::GetProcessesResponse { processes })
    }

    async fn handle_get_remote_profiling_data(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetRemoteProfilingData>,
        cx: AsyncApp,
    ) -> Result<proto::GetRemoteProfilingDataResponse> {
        let foreground_only = envelope.payload.foreground_only;

        let (deltas, now_nanos) = cx.update(|cx| {
            let dispatcher = cx.foreground_executor().dispatcher();
            let timings = if foreground_only {
                vec![dispatcher.get_current_thread_timings()]
            } else {
                dispatcher.get_all_timings()
            };
            this.update(cx, |this, _cx| {
                let deltas = this.profiling_collector.collect_unseen(timings);
                let now_nanos = Instant::now()
                    .duration_since(this.profiling_collector.startup_time())
                    .as_nanos() as u64;
                (deltas, now_nanos)
            })
        });

        let threads = deltas
            .into_iter()
            .map(|delta| proto::RemoteProfilingThread {
                thread_name: delta.thread_name,
                thread_id: delta.thread_id,
                timings: delta
                    .new_timings
                    .into_iter()
                    .map(|t| proto::RemoteProfilingTiming {
                        location: Some(proto::RemoteProfilingLocation {
                            file: t.location.file.to_string(),
                            line: t.location.line,
                            column: t.location.column,
                        }),
                        start_nanos: t.start as u64,
                        duration_nanos: t.duration as u64,
                    })
                    .collect(),
            })
            .collect();

        Ok(proto::GetRemoteProfilingDataResponse { threads, now_nanos })
    }

    async fn handle_get_directory_environment(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetDirectoryEnvironment>,
        mut cx: AsyncApp,
    ) -> Result<proto::DirectoryEnvironment> {
        let shell = task::shell_from_proto(envelope.payload.shell.context("missing shell")?)?;
        let directory = PathBuf::from(envelope.payload.directory);
        let environment = this
            .update(&mut cx, |this, cx| {
                this.environment.update(cx, |environment, cx| {
                    environment.local_directory_environment(&shell, directory.into(), cx)
                })
            })
            .await
            .context("failed to get directory environment")?
            .into_iter()
            .collect();
        Ok(proto::DirectoryEnvironment { environment })
    }
}

impl PeerBufferAccess for HeadlessProject {
    fn create_buffer_for_peer(
        &mut self,
        buffer: &Entity<Buffer>,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> gpui::Task<Result<()>> {
        HeadlessProject::create_buffer_for_peer(self, buffer, peer_id, cx)
    }

    fn serialize_project_transaction_for_peer(
        &mut self,
        project_transaction: ProjectTransaction,
        peer_id: proto::PeerId,
        cx: &mut App,
    ) -> proto::ProjectTransaction {
        HeadlessProject::serialize_project_transaction_for_peer(
            self,
            project_transaction,
            peer_id,
            cx,
        )
    }
}

fn prompt_to_proto(
    prompt: &project::LanguageServerPromptRequest,
) -> proto::language_server_prompt_request::Level {
    match prompt.level {
        PromptLevel::Info => proto::language_server_prompt_request::Level::Info(
            proto::language_server_prompt_request::Info {},
        ),
        PromptLevel::Warning => proto::language_server_prompt_request::Level::Warning(
            proto::language_server_prompt_request::Warning {},
        ),
        PromptLevel::Critical => proto::language_server_prompt_request::Level::Critical(
            proto::language_server_prompt_request::Critical {},
        ),
    }
}

fn find_venv_python(working_directory: &str) -> Option<std::path::PathBuf> {
    let wd = std::path::Path::new(working_directory);
    for dir_name in &[".venv", "venv", ".env", "env"] {
        let venv_dir = wd.join(dir_name);
        let has_pyvenv_cfg = venv_dir.join("pyvenv.cfg").is_file();
        let has_activate = venv_dir.join("bin").join("activate").is_file();
        if has_pyvenv_cfg || has_activate {
            let python = venv_dir.join("bin").join("python");
            if python.is_file() {
                return Some(python);
            }
            let python3 = venv_dir.join("bin").join("python3");
            if python3.is_file() {
                return Some(python3);
            }
        }
    }
    None
}
