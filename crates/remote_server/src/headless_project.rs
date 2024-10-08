use anyhow::{anyhow, Result};
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, Context, Model, ModelContext};
use language::{proto::serialize_operation, Buffer, BufferEvent, LanguageRegistry};
use node_runtime::NodeRuntime;
use project::{
    buffer_store::{BufferStore, BufferStoreEvent},
    project_settings::SettingsObserver,
    search::SearchQuery,
    worktree_store::WorktreeStore,
    LspStore, LspStoreEvent, PrettierStore, ProjectPath, WorktreeId,
};
use remote::ssh_session::ChannelClient;
use rpc::{
    proto::{self, SSH_PEER_ID, SSH_PROJECT_ID},
    AnyProtoClient, TypedEnvelope,
};
use smol::stream::StreamExt;
use std::{
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, Arc},
};
use util::ResultExt;
use worktree::Worktree;

pub struct HeadlessProject {
    pub fs: Arc<dyn Fs>,
    pub session: AnyProtoClient,
    pub worktree_store: Model<WorktreeStore>,
    pub buffer_store: Model<BufferStore>,
    pub lsp_store: Model<LspStore>,
    pub settings_observer: Model<SettingsObserver>,
    pub next_entry_id: Arc<AtomicUsize>,
    pub languages: Arc<LanguageRegistry>,
}

impl HeadlessProject {
    pub fn init(cx: &mut AppContext) {
        settings::init(cx);
        language::init(cx);
        project::Project::init_settings(cx);
    }

    pub fn new(session: Arc<ChannelClient>, fs: Arc<dyn Fs>, cx: &mut ModelContext<Self>) -> Self {
        let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));

        let node_runtime = NodeRuntime::unavailable();

        languages::init(languages.clone(), node_runtime.clone(), cx);

        let worktree_store = cx.new_model(|cx| {
            let mut store = WorktreeStore::local(true, fs.clone());
            store.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            store
        });
        let buffer_store = cx.new_model(|cx| {
            let mut buffer_store = BufferStore::local(worktree_store.clone(), cx);
            buffer_store.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            buffer_store
        });
        let prettier_store = cx.new_model(|cx| {
            PrettierStore::new(
                node_runtime,
                fs.clone(),
                languages.clone(),
                worktree_store.clone(),
                cx,
            )
        });

        let settings_observer = cx.new_model(|cx| {
            let mut observer = SettingsObserver::new_local(fs.clone(), worktree_store.clone(), cx);
            observer.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            observer
        });
        let environment = project::ProjectEnvironment::new(&worktree_store, None, cx);
        let lsp_store = cx.new_model(|cx| {
            let mut lsp_store = LspStore::new_local(
                buffer_store.clone(),
                worktree_store.clone(),
                prettier_store.clone(),
                environment,
                languages.clone(),
                None,
                fs.clone(),
                cx,
            );
            lsp_store.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            lsp_store
        });

        cx.subscribe(&lsp_store, Self::on_lsp_store_event).detach();

        cx.subscribe(
            &buffer_store,
            |_this, _buffer_store, event, cx| match event {
                BufferStoreEvent::BufferAdded(buffer) => {
                    cx.subscribe(buffer, Self::on_buffer_event).detach();
                }
                _ => {}
            },
        )
        .detach();

        let client: AnyProtoClient = session.clone().into();

        session.subscribe_to_entity(SSH_PROJECT_ID, &worktree_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &buffer_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &cx.handle());
        session.subscribe_to_entity(SSH_PROJECT_ID, &lsp_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &settings_observer);

        client.add_request_handler(cx.weak_model(), Self::handle_list_remote_directory);
        client.add_request_handler(cx.weak_model(), Self::handle_check_file_exists);
        client.add_request_handler(cx.weak_model(), Self::handle_shutdown_remote_server);
        client.add_request_handler(cx.weak_model(), Self::handle_ping);

        client.add_model_request_handler(Self::handle_add_worktree);
        client.add_model_request_handler(Self::handle_open_buffer_by_path);
        client.add_model_request_handler(Self::handle_find_search_candidates);

        client.add_model_request_handler(BufferStore::handle_update_buffer);
        client.add_model_message_handler(BufferStore::handle_close_buffer);

        BufferStore::init(&client);
        WorktreeStore::init(&client);
        SettingsObserver::init(&client);
        LspStore::init(&client);

        HeadlessProject {
            session: client,
            settings_observer,
            fs,
            worktree_store,
            buffer_store,
            lsp_store,
            next_entry_id: Default::default(),
            languages,
        }
    }

    fn on_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &BufferEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            BufferEvent::Operation {
                operation,
                is_local: true,
            } => cx
                .background_executor()
                .spawn(self.session.request(proto::UpdateBuffer {
                    project_id: SSH_PROJECT_ID,
                    buffer_id: buffer.read(cx).remote_id().to_proto(),
                    operations: vec![serialize_operation(operation)],
                }))
                .detach(),
            _ => {}
        }
    }

    fn on_lsp_store_event(
        &mut self,
        _lsp_store: Model<LspStore>,
        event: &LspStoreEvent,
        _cx: &mut ModelContext<Self>,
    ) {
        match event {
            LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                message,
            } => {
                self.session
                    .send(proto::UpdateLanguageServer {
                        project_id: SSH_PROJECT_ID,
                        language_server_id: language_server_id.to_proto(),
                        variant: Some(message.clone()),
                    })
                    .log_err();
            }
            _ => {}
        }
    }

    pub async fn handle_add_worktree(
        this: Model<Self>,
        message: TypedEnvelope<proto::AddWorktree>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::AddWorktreeResponse> {
        use client::ErrorCodeExt;
        let path = shellexpand::tilde(&message.payload.path).to_string();

        let fs = this.read_with(&mut cx, |this, _| this.fs.clone())?;
        let path = PathBuf::from(path);

        let canonicalized = match fs.canonicalize(&path).await {
            Ok(path) => path,
            Err(e) => {
                let mut parent = path
                    .parent()
                    .ok_or(e)
                    .map_err(|_| anyhow!("{:?} does not exist", path))?;
                if parent == Path::new("") {
                    parent = util::paths::home_dir();
                }
                let parent = fs.canonicalize(parent).await.map_err(|_| {
                    anyhow!(proto::ErrorCode::DevServerProjectPathDoesNotExist
                        .with_tag("path", &path.to_string_lossy().as_ref()))
                })?;
                parent.join(path.file_name().unwrap())
            }
        };

        let worktree = this
            .update(&mut cx.clone(), |this, _| {
                Worktree::local(
                    Arc::from(canonicalized),
                    true,
                    this.fs.clone(),
                    this.next_entry_id.clone(),
                    &mut cx,
                )
            })?
            .await?;

        this.update(&mut cx, |this, cx| {
            this.worktree_store.update(cx, |worktree_store, cx| {
                worktree_store.add(&worktree, cx);
            });
            worktree.update(cx, |worktree, _| proto::AddWorktreeResponse {
                worktree_id: worktree.id().to_proto(),
            })
        })
    }

    pub async fn handle_open_buffer_by_path(
        this: Model<Self>,
        message: TypedEnvelope<proto::OpenBufferByPath>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(message.payload.worktree_id);
        let (buffer_store, buffer) = this.update(&mut cx, |this, cx| {
            let buffer_store = this.buffer_store.clone();
            let buffer = this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(
                    ProjectPath {
                        worktree_id,
                        path: PathBuf::from(message.payload.path).into(),
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
                .create_buffer_for_peer(&buffer, SSH_PEER_ID, cx)
                .detach_and_log_err(cx);
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    pub async fn handle_find_search_candidates(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::FindSearchCandidates>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::FindSearchCandidatesResponse> {
        let message = envelope.payload;
        let query = SearchQuery::from_proto(
            message
                .query
                .ok_or_else(|| anyhow!("missing query field"))?,
        )?;
        let mut results = this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store.find_search_candidates(&query, message.limit as _, this.fs.clone(), cx)
            })
        })?;

        let mut response = proto::FindSearchCandidatesResponse {
            buffer_ids: Vec::new(),
        };

        let buffer_store = this.read_with(&cx, |this, _| this.buffer_store.clone())?;

        while let Some(buffer) = results.next().await {
            let buffer_id = buffer.update(&mut cx, |this, _| this.remote_id())?;
            response.buffer_ids.push(buffer_id.to_proto());
            buffer_store
                .update(&mut cx, |buffer_store, cx| {
                    buffer_store.create_buffer_for_peer(&buffer, SSH_PEER_ID, cx)
                })?
                .await?;
        }

        Ok(response)
    }

    pub async fn handle_list_remote_directory(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::ListRemoteDirectory>,
        cx: AsyncAppContext,
    ) -> Result<proto::ListRemoteDirectoryResponse> {
        let expanded = shellexpand::tilde(&envelope.payload.path).to_string();
        let fs = cx.read_model(&this, |this, _| this.fs.clone())?;

        let mut entries = Vec::new();
        let mut response = fs.read_dir(Path::new(&expanded)).await?;
        while let Some(path) = response.next().await {
            if let Some(file_name) = path?.file_name() {
                entries.push(file_name.to_string_lossy().to_string());
            }
        }
        Ok(proto::ListRemoteDirectoryResponse { entries })
    }

    pub async fn handle_check_file_exists(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::CheckFileExists>,
        cx: AsyncAppContext,
    ) -> Result<proto::CheckFileExistsResponse> {
        let fs = cx.read_model(&this, |this, _| this.fs.clone())?;
        let expanded = shellexpand::tilde(&envelope.payload.path).to_string();

        let exists = fs.is_file(&PathBuf::from(expanded.clone())).await;

        Ok(proto::CheckFileExistsResponse {
            exists,
            path: expanded,
        })
    }

    pub async fn handle_shutdown_remote_server(
        _this: Model<Self>,
        _envelope: TypedEnvelope<proto::ShutdownRemoteServer>,
        cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        cx.spawn(|cx| async move {
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
        _this: Model<Self>,
        _envelope: TypedEnvelope<proto::Ping>,
        _cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        log::debug!("Received ping from client");
        Ok(proto::Ack {})
    }
}
