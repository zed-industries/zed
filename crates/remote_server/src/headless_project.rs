use ::proto::{FromProto, ToProto};
use anyhow::{Context as _, Result, anyhow};

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
    buffer_store::{BufferStore, BufferStoreEvent},
    debugger::{breakpoint_store::BreakpointStore, dap_store::DapStore},
    git_store::GitStore,
    project_settings::SettingsObserver,
    search::SearchQuery,
    task_store::TaskStore,
    worktree_store::WorktreeStore,
};
use rpc::{
    AnyProtoClient, TypedEnvelope,
    proto::{self, REMOTE_SERVER_PEER_ID, REMOTE_SERVER_PROJECT_ID},
};

use settings::initial_server_settings_content;
use smol::stream::StreamExt;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use util::ResultExt;
use worktree::Worktree;

pub struct HeadlessProject {
    pub fs: Arc<dyn Fs>,
    pub session: AnyProtoClient,
    pub worktree_store: Entity<WorktreeStore>,
    pub buffer_store: Entity<BufferStore>,
    pub lsp_store: Entity<LspStore>,
    pub task_store: Entity<TaskStore>,
    pub dap_store: Entity<DapStore>,
    pub settings_observer: Entity<SettingsObserver>,
    pub next_entry_id: Arc<AtomicUsize>,
    pub languages: Arc<LanguageRegistry>,
    pub extensions: Entity<HeadlessExtensionStore>,
    pub git_store: Entity<GitStore>,
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
        language::init(cx);
        project::Project::init_settings(cx);
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
        cx: &mut Context<Self>,
    ) -> Self {
        debug_adapter_extension::init(proxy.clone(), cx);
        languages::init(languages.clone(), node_runtime.clone(), cx);

        let worktree_store = cx.new(|cx| {
            let mut store = WorktreeStore::local(true, fs.clone());
            store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            store
        });

        let environment = cx.new(|_| ProjectEnvironment::new(None));
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

        let buffer_store = cx.new(|cx| {
            let mut buffer_store = BufferStore::local(worktree_store.clone(), cx);
            buffer_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            buffer_store
        });

        let breakpoint_store =
            cx.new(|_| BreakpointStore::local(worktree_store.clone(), buffer_store.clone()));

        let dap_store = cx.new(|cx| {
            let mut dap_store = DapStore::new_local(
                http_client.clone(),
                node_runtime.clone(),
                fs.clone(),
                environment.clone(),
                toolchain_store.read(cx).as_language_toolchain_store(),
                worktree_store.clone(),
                breakpoint_store.clone(),
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
                fs.clone(),
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
                environment,
                manifest_tree,
                languages.clone(),
                http_client.clone(),
                fs.clone(),
                cx,
            );
            lsp_store.shared(REMOTE_SERVER_PROJECT_ID, session.clone(), cx);
            lsp_store
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
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &settings_observer);
        session.subscribe_to_entity(REMOTE_SERVER_PROJECT_ID, &git_store);

        session.add_request_handler(cx.weak_entity(), Self::handle_list_remote_directory);
        session.add_request_handler(cx.weak_entity(), Self::handle_get_path_metadata);
        session.add_request_handler(cx.weak_entity(), Self::handle_shutdown_remote_server);
        session.add_request_handler(cx.weak_entity(), Self::handle_ping);

        session.add_entity_request_handler(Self::handle_add_worktree);
        session.add_request_handler(cx.weak_entity(), Self::handle_remove_worktree);

        session.add_entity_request_handler(Self::handle_open_buffer_by_path);
        session.add_entity_request_handler(Self::handle_open_new_buffer);
        session.add_entity_request_handler(Self::handle_find_search_candidates);
        session.add_entity_request_handler(Self::handle_open_server_settings);

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
        // BreakpointStore::init(&client);
        GitStore::init(&session);

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
            languages,
            extensions,
            git_store,
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
        _lsp_store: Entity<LspStore>,
        event: &LspStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
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
            LspStoreEvent::LanguageServerLog(language_server_id, log_type, message) => {
                self.session
                    .send(proto::LanguageServerLog {
                        project_id: REMOTE_SERVER_PROJECT_ID,
                        language_server_id: language_server_id.to_proto(),
                        message: message.clone(),
                        log_type: Some(log_type.to_proto()),
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
        let fs = this.read_with(&cx, |this, _| this.fs.clone())?;
        let path = PathBuf::from_proto(shellexpand::tilde(&message.payload.path).to_string());

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
                parent.join(path.file_name().unwrap())
            }
        };

        let worktree = this
            .read_with(&cx.clone(), |this, _| {
                Worktree::local(
                    Arc::from(canonicalized.as_path()),
                    message.payload.visible,
                    this.fs.clone(),
                    this.next_entry_id.clone(),
                    &mut cx,
                )
            })?
            .await?;

        let response = this.read_with(&cx, |_, cx| {
            let worktree = worktree.read(cx);
            proto::AddWorktreeResponse {
                worktree_id: worktree.id().to_proto(),
                canonicalized_path: canonicalized.to_proto(),
            }
        })?;

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
            })
            .log_err();
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
        })?;
        Ok(proto::Ack {})
    }

    pub async fn handle_open_buffer_by_path(
        this: Entity<Self>,
        message: TypedEnvelope<proto::OpenBufferByPath>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(message.payload.worktree_id);
        let (buffer_store, buffer) = this.update(&mut cx, |this, cx| {
            let buffer_store = this.buffer_store.clone();
            let buffer = this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(
                    ProjectPath {
                        worktree_id,
                        path: Arc::<Path>::from_proto(message.payload.path),
                    },
                    cx,
                )
            });
            anyhow::Ok((buffer_store, buffer))
        })??;

        let buffer = buffer.await?;
        let buffer_id = buffer.read_with(&cx, |b, _| b.remote_id())?;
        buffer_store.update(&mut cx, |buffer_store, cx| {
            buffer_store
                .create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                .detach_and_log_err(cx);
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    pub async fn handle_open_new_buffer(
        this: Entity<Self>,
        _message: TypedEnvelope<proto::OpenNewBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let (buffer_store, buffer) = this.update(&mut cx, |this, cx| {
            let buffer_store = this.buffer_store.clone();
            let buffer = this
                .buffer_store
                .update(cx, |buffer_store, cx| buffer_store.create_buffer(cx));
            anyhow::Ok((buffer_store, buffer))
        })??;

        let buffer = buffer.await?;
        let buffer_id = buffer.read_with(&cx, |b, _| b.remote_id())?;
        buffer_store.update(&mut cx, |buffer_store, cx| {
            buffer_store
                .create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                .detach_and_log_err(cx);
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    pub async fn handle_open_server_settings(
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
            })?
            .await?;

        let (buffer, buffer_store) = this.update(&mut cx, |this, cx| {
            let buffer = this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(
                    ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: path.into(),
                    },
                    cx,
                )
            });

            (buffer, this.buffer_store.clone())
        })?;

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
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    pub async fn handle_find_search_candidates(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidates>,
        mut cx: AsyncApp,
    ) -> Result<proto::FindSearchCandidatesResponse> {
        let message = envelope.payload;
        let query = SearchQuery::from_proto(message.query.context("missing query field")?)?;
        let results = this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.find_search_candidates(&query, message.limit as _, this.fs.clone(), cx)
            })
        })?;

        let mut response = proto::FindSearchCandidatesResponse {
            buffer_ids: Vec::new(),
        };

        let buffer_store = this.read_with(&cx, |this, _| this.buffer_store.clone())?;

        while let Ok(buffer) = results.recv().await {
            let buffer_id = buffer.read_with(&cx, |this, _| this.remote_id())?;
            response.buffer_ids.push(buffer_id.to_proto());
            buffer_store
                .update(&mut cx, |buffer_store, cx| {
                    buffer_store.create_buffer_for_peer(&buffer, REMOTE_SERVER_PEER_ID, cx)
                })?
                .await?;
        }

        Ok(response)
    }

    pub async fn handle_list_remote_directory(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ListRemoteDirectory>,
        cx: AsyncApp,
    ) -> Result<proto::ListRemoteDirectoryResponse> {
        let fs = cx.read_entity(&this, |this, _| this.fs.clone())?;
        let expanded = PathBuf::from_proto(shellexpand::tilde(&envelope.payload.path).to_string());
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
                entries.push(file_name.to_string_lossy().to_string());
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

    pub async fn handle_get_path_metadata(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetPathMetadata>,
        cx: AsyncApp,
    ) -> Result<proto::GetPathMetadataResponse> {
        let fs = cx.read_entity(&this, |this, _| this.fs.clone())?;
        let expanded = PathBuf::from_proto(shellexpand::tilde(&envelope.payload.path).to_string());

        let metadata = fs.metadata(&expanded).await?;
        let is_dir = metadata.map(|metadata| metadata.is_dir).unwrap_or(false);

        Ok(proto::GetPathMetadataResponse {
            exists: metadata.is_some(),
            is_dir,
            path: expanded.to_proto(),
        })
    }

    pub async fn handle_shutdown_remote_server(
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
