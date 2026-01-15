use anyhow::{Context as _, Result, anyhow};
use client::ProjectId;
use collections::HashSet;
use language::File;
use lsp::LanguageServerId;

use extension::ExtensionHostProxy;
use extension_host::headless_host::HeadlessExtensionStore;
use fs::Fs;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, PromptLevel};
use http_client::HttpClient;
use language::{Buffer, BufferEvent, LanguageRegistry, proto::serialize_operation};
use node_runtime::NodeRuntime;
use project::{
    LspStore, LspStoreEvent, ManifestTree, PrettierStore, ProjectEnvironment, ProjectPath,
    ToolchainStore, WorktreeId,
    agent_server_store::AgentServerStore,
    buffer_store::{BufferStore, BufferStoreEvent},
    context_server_store::ContextServerStore,
    debugger::{breakpoint_store::BreakpointStore, dap_store::DapStore},
    git_store::GitStore,
    image_store::ImageId,
    lsp_store::log_store::{self, GlobalLogStore, LanguageServerKind, LogKind},
    project_settings::SettingsObserver,
    search::SearchQuery,
    task_store::TaskStore,
    trusted_worktrees::{PathTrust, RemoteHostLocation, TrustedWorktrees},
    worktree_store::WorktreeStore,
};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, REMOTE_SERVER_PEER_ID, REMOTE_SERVER_PROJECT_ID},
};

use settings::initial_server_settings_content;
use std::{
    num::NonZeroU64,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind};
use util::{ResultExt, paths::PathStyle, rel_path::RelPath};
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
    // Used mostly to keep alive the toolchain store for RPC handlers.
    // Local variant is used within LSP store, but that's a separate entity.
    pub _toolchain_store: Entity<ToolchainStore>,
}

pub struct HeadlessAppState {
    pub session: AnyProtoClient,
    pub fs: Arc<dyn Fs>,
    pub http_client: Arc<dyn HttpClient>,
    pub node_runtime: NodeRuntime,
    pub languages: Arc<LanguageRegistry>,
    pub extension_host_proxy: Arc<ExtensionHostProxy>,
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
        }: HeadlessAppState,
        init_worktree_trust: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        debug_adapter_extension::init(proxy.clone(), cx);
        languages::init(languages.clone(), fs.clone(), node_runtime.clone(), cx);

        let worktree_store = cx.new(|cx| {
            let mut store = WorktreeStore::local(true, fs.clone());
            store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            store
        });

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
                fs.clone(),
                cx,
            )
        });

        let buffer_store = cx.new(|cx| {
            let mut buffer_store = BufferStore::local(worktree_store.clone(), cx);
            buffer_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            buffer_store
        });

        let breakpoint_store = cx.new(|_| {
            let mut breakpoint_store =
                BreakpointStore::local(worktree_store.clone(), buffer_store.clone());
            breakpoint_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone());

            breakpoint_store
        });

        let dap_store = cx.new(|cx| {
            let mut dap_store = DapStore::new_local(
                http_client.clone(),
                node_runtime.clone(),
                fs.clone(),
                environment.clone(),
                toolchain_store.read(cx).as_language_toolchain_store(),
                worktree_store.clone(),
                breakpoint_store.clone(),
                true,
                cx,
            );
            dap_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            dap_store
        });

        let git_store = cx.new(|cx| {
            let mut store = GitStore::local(
                &worktree_store,
                buffer_store.clone(),
                environment.clone(),
                fs.clone(),
                cx,
            );
            store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            store
        });

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
            let mut task_store = TaskStore::local(
                buffer_store.downgrade(),
                worktree_store.clone(),
                toolchain_store.read(cx).as_language_toolchain_store(),
                environment.clone(),
                cx,
            );
            task_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            task_store
        });
        let settings_observer = cx.new(|cx| {
            let mut observer = SettingsObserver::new_local(
                fs.clone(),
                worktree_store.clone(),
                task_store.clone(),
                cx,
            );
            observer.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            observer
        });

        let lsp_store = cx.new(|cx| {
            let mut lsp_store = LspStore::new_local(
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
            );
            lsp_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            lsp_store
        });

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
                ContextServerStore::local(worktree_store.clone(), None, true, cx);
            context_server_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone());
            context_server_store
        });

        cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();
        language_extension::init(
            language_extension::LspAccess::ViaLspStore(lsp_store.clone()),
            proxy.clone(),
            languages.clone(),
        );

        cx.subscribe(&buffer_store, |_this, _buffer_store, event, cx| {
            if let BufferStoreEvent::BufferAdded(buffer) = event {
                cx.subscribe(buffer, Self::on_buffer_event).detach();
            }
        })
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

        session.add_entity_request_handler(Self::handle_add_worktree);
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

        session.add_entity_message_handler(Self::handle_find_search_candidates_cancel);
        session.add_entity_request_handler(BufferStore::handle_update_buffer);
        session.add_entity_message_handler(BufferStore::handle_close_buffer);

        session.add_request_handler(
            extensions.downgrade(),
            HeadlessExtensionStore::handle_sync_extensions,
        );
        session.add_request_handler(
            extensions.downgrade(),
            HeadlessExtensionStore::handle_install_extension,
        );

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
            _toolchain_store: toolchain_store,
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) {
        if let BufferEvent::Operation {
            operation,
            is_local: true,
        } = event
        {
            cx.background_spawn(self.session.request(proto::UpdateBuffer {
                project_id: REMOTE_SERVER_PROJECT_ID,
                buffer_id: buffer.read(cx).remote_id().to_proto(),
                operations: vec![serialize_operation(operation)],
            }))
            .detach()
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
                                lsp_store: self.lsp_store.downgrade(),
                            },
                            *id,
                            Some(name.clone()),
                            *worktree_id,
                            lsp_store.read(cx).language_server_for_id(*id),
                            cx,
                        );
                    });
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
            _ => {}
        }
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

        let worktree = this
            .read_with(&cx.clone(), |this, _| {
                Worktree::local(
                    Arc::from(canonicalized.as_path()),
                    message.payload.visible,
                    this.fs.clone(),
                    this.next_entry_id.clone(),
                    true,
                    &mut cx,
                )
            })
            .await?;

        let response = this.read_with(&cx, |_, cx| {
            let worktree = worktree.read(cx);
            proto::AddWorktreeResponse {
                worktree_id: worktree.id().to_proto(),
                canonicalized_path: canonicalized.to_string_lossy().into_owned(),
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
        let (buffer_store, buffer) = this.update(&mut cx, |this, cx| {
            let buffer_store = this.buffer_store.clone();
            let buffer = this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(ProjectPath { worktree_id, path }, cx)
            });
            (buffer_store, buffer)
        });

        let buffer = buffer.await?;
        let buffer_id = buffer.read_with(&cx, |b, _| b.remote_id());
        buffer_store.update(&mut cx, |buffer_store, cx| {
            buffer_store
                .create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
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

    pub async fn handle_open_new_buffer(
        this: Entity<Self>,
        _message: TypedEnvelope<proto::OpenNewBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let (buffer_store, buffer) = this.update(&mut cx, |this, cx| {
            let buffer_store = this.buffer_store.clone();
            let buffer = this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.create_buffer(None, true, cx)
            });
            (buffer_store, buffer)
        });

        let buffer = buffer.await?;
        let buffer_id = buffer.read_with(&cx, |b, _| b.remote_id());
        buffer_store.update(&mut cx, |buffer_store, cx| {
            buffer_store
                .create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
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

        let (buffer, buffer_store) = this.update(&mut cx, |this, cx| {
            let buffer = this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(
                    ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: path,
                    },
                    cx,
                )
            });

            (buffer, this.buffer_store.clone())
        });

        let buffer = buffer.await?;

        let buffer_id = cx.update(|cx| {
            if buffer.read(cx).is_empty() {
                buffer.update(cx, |buffer, cx| {
                    buffer.edit([(0..0, initial_server_settings_content())], None, cx)
                });
            }

            let buffer_id = buffer.read(cx).remote_id();

            buffer_store.update(cx, |buffer_store, cx| {
                buffer_store
                    .create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                    .detach_and_log_err(cx);
            });

            buffer_id
        });

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
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
        let buffer_store = this.read_with(&cx, |this, _| this.buffer_store.clone());
        let handle = message.handle;
        let _buffer_store = buffer_store.clone();
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
                let _ = buffer_store
                    .update(cx, |this, cx| {
                        this.create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                    })
                    .await;
                let buffer_id = buffer.read_with(cx, |this, _| this.remote_id().to_proto());
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
        _buffer_store.update(&mut cx, |this, _| {
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
