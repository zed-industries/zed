use anyhow::{anyhow, Context as _, Result};
use extension::ExtensionHostProxy;
use extension_host::headless_host::HeadlessExtensionStore;
use fs::{CreateOptions, Fs};
use git::{repository::RepoPath, COMMIT_MESSAGE};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, PromptLevel, SharedString};
use http_client::HttpClient;
use language::{proto::serialize_operation, Buffer, BufferEvent, LanguageRegistry};
use node_runtime::NodeRuntime;
use project::{
    buffer_store::{BufferStore, BufferStoreEvent},
    git::{GitRepo, GitState, Repository},
    project_settings::SettingsObserver,
    search::SearchQuery,
    task_store::TaskStore,
    worktree_store::WorktreeStore,
    LspStore, LspStoreEvent, PrettierStore, ProjectEntryId, ProjectPath, ToolchainStore,
    WorktreeId,
};
use remote::ssh_session::ChannelClient;
use rpc::{
    proto::{self, SSH_PEER_ID, SSH_PROJECT_ID},
    AnyProtoClient, TypedEnvelope,
};

use settings::initial_server_settings_content;
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
    pub worktree_store: Entity<WorktreeStore>,
    pub buffer_store: Entity<BufferStore>,
    pub lsp_store: Entity<LspStore>,
    pub task_store: Entity<TaskStore>,
    pub settings_observer: Entity<SettingsObserver>,
    pub next_entry_id: Arc<AtomicUsize>,
    pub languages: Arc<LanguageRegistry>,
    pub extensions: Entity<HeadlessExtensionStore>,
    pub git_state: Entity<GitState>,
}

pub struct HeadlessAppState {
    pub session: Arc<ChannelClient>,
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
        language_extension::init(proxy.clone(), languages.clone());
        languages::init(languages.clone(), node_runtime.clone(), cx);

        let worktree_store = cx.new(|cx| {
            let mut store = WorktreeStore::local(true, fs.clone());
            store.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            store
        });

        let git_state = cx.new(|cx| GitState::new(&worktree_store, None, None, cx));

        let buffer_store = cx.new(|cx| {
            let mut buffer_store = BufferStore::local(worktree_store.clone(), cx);
            buffer_store.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            buffer_store
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
        let environment = project::ProjectEnvironment::new(&worktree_store, None, cx);
        let toolchain_store = cx.new(|cx| {
            ToolchainStore::local(
                languages.clone(),
                worktree_store.clone(),
                environment.clone(),
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
            task_store.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            task_store
        });
        let settings_observer = cx.new(|cx| {
            let mut observer = SettingsObserver::new_local(
                fs.clone(),
                worktree_store.clone(),
                task_store.clone(),
                cx,
            );
            observer.shared(SSH_PROJECT_ID, session.clone().into(), cx);
            observer
        });

        let lsp_store = cx.new(|cx| {
            let mut lsp_store = LspStore::new_local(
                buffer_store.clone(),
                worktree_store.clone(),
                prettier_store.clone(),
                toolchain_store.clone(),
                environment,
                languages.clone(),
                http_client.clone(),
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

        let extensions = HeadlessExtensionStore::new(
            fs.clone(),
            http_client.clone(),
            paths::remote_extensions_dir().to_path_buf(),
            proxy,
            node_runtime,
            cx,
        );

        let client: AnyProtoClient = session.clone().into();

        // local_machine -> ssh handlers
        session.subscribe_to_entity(SSH_PROJECT_ID, &worktree_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &buffer_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &cx.entity());
        session.subscribe_to_entity(SSH_PROJECT_ID, &lsp_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &task_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &toolchain_store);
        session.subscribe_to_entity(SSH_PROJECT_ID, &settings_observer);

        client.add_request_handler(cx.weak_entity(), Self::handle_list_remote_directory);
        client.add_request_handler(cx.weak_entity(), Self::handle_get_path_metadata);
        client.add_request_handler(cx.weak_entity(), Self::handle_shutdown_remote_server);
        client.add_request_handler(cx.weak_entity(), Self::handle_ping);

        client.add_entity_request_handler(Self::handle_add_worktree);
        client.add_request_handler(cx.weak_entity(), Self::handle_remove_worktree);

        client.add_entity_request_handler(Self::handle_open_buffer_by_path);
        client.add_entity_request_handler(Self::handle_open_new_buffer);
        client.add_entity_request_handler(Self::handle_find_search_candidates);
        client.add_entity_request_handler(Self::handle_open_server_settings);

        client.add_entity_request_handler(BufferStore::handle_update_buffer);
        client.add_entity_message_handler(BufferStore::handle_close_buffer);

        client.add_entity_request_handler(Self::handle_stage);
        client.add_entity_request_handler(Self::handle_unstage);
        client.add_entity_request_handler(Self::handle_commit);
        client.add_entity_request_handler(Self::handle_open_commit_message_buffer);

        client.add_request_handler(
            extensions.clone().downgrade(),
            HeadlessExtensionStore::handle_sync_extensions,
        );
        client.add_request_handler(
            extensions.clone().downgrade(),
            HeadlessExtensionStore::handle_install_extension,
        );

        BufferStore::init(&client);
        WorktreeStore::init(&client);
        SettingsObserver::init(&client);
        LspStore::init(&client);
        TaskStore::init(Some(&client));
        ToolchainStore::init(&client);

        HeadlessProject {
            session: client,
            settings_observer,
            fs,
            worktree_store,
            buffer_store,
            lsp_store,
            task_store,
            next_entry_id: Default::default(),
            languages,
            extensions,
            git_state,
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
        _lsp_store: Entity<LspStore>,
        event: &LspStoreEvent,
        cx: &mut Context<Self>,
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
            LspStoreEvent::Notification(message) => {
                self.session
                    .send(proto::Toast {
                        project_id: SSH_PROJECT_ID,
                        notification_id: "lsp".to_string(),
                        message: message.clone(),
                    })
                    .log_err();
            }
            LspStoreEvent::LanguageServerLog(language_server_id, log_type, message) => {
                self.session
                    .send(proto::LanguageServerLog {
                        project_id: SSH_PROJECT_ID,
                        language_server_id: language_server_id.to_proto(),
                        message: message.clone(),
                        log_type: Some(log_type.to_proto()),
                    })
                    .log_err();
            }
            LspStoreEvent::LanguageServerPrompt(prompt) => {
                let request = self.session.request(proto::LanguageServerPromptRequest {
                    project_id: SSH_PROJECT_ID,
                    actions: prompt
                        .actions
                        .iter()
                        .map(|action| action.title.to_string())
                        .collect(),
                    level: Some(prompt_to_proto(&prompt)),
                    lsp_name: prompt.lsp_name.clone(),
                    message: prompt.message.clone(),
                });
                let prompt = prompt.clone();
                cx.background_executor()
                    .spawn(async move {
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
                    Arc::from(canonicalized.as_path()),
                    message.payload.visible,
                    this.fs.clone(),
                    this.next_entry_id.clone(),
                    &mut cx,
                )
            })?
            .await?;

        let response = this.update(&mut cx, |_, cx| {
            worktree.update(cx, |worktree, _| proto::AddWorktreeResponse {
                worktree_id: worktree.id().to_proto(),
                canonicalized_path: canonicalized.to_string_lossy().to_string(),
            })
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
        cx.spawn(|mut cx| async move {
            this.update(&mut cx, |this, cx| {
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
                .create_buffer_for_peer(&buffer, SSH_PEER_ID, cx)
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

            let buffer_id = buffer.read_with(cx, |b, _| b.remote_id());

            buffer_store.update(cx, |buffer_store, cx| {
                buffer_store
                    .create_buffer_for_peer(&buffer, SSH_PEER_ID, cx)
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
        let query = SearchQuery::from_proto(
            message
                .query
                .ok_or_else(|| anyhow!("missing query field"))?,
        )?;
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
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ListRemoteDirectory>,
        cx: AsyncApp,
    ) -> Result<proto::ListRemoteDirectoryResponse> {
        let expanded = shellexpand::tilde(&envelope.payload.path).to_string();
        let fs = cx.read_entity(&this, |this, _| this.fs.clone())?;

        let mut entries = Vec::new();
        let mut response = fs.read_dir(Path::new(&expanded)).await?;
        while let Some(path) = response.next().await {
            if let Some(file_name) = path?.file_name() {
                entries.push(file_name.to_string_lossy().to_string());
            }
        }
        Ok(proto::ListRemoteDirectoryResponse { entries })
    }

    pub async fn handle_get_path_metadata(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GetPathMetadata>,
        cx: AsyncApp,
    ) -> Result<proto::GetPathMetadataResponse> {
        let fs = cx.read_entity(&this, |this, _| this.fs.clone())?;
        let expanded = shellexpand::tilde(&envelope.payload.path).to_string();

        let metadata = fs.metadata(&PathBuf::from(expanded.clone())).await?;
        let is_dir = metadata.map(|metadata| metadata.is_dir).unwrap_or(false);

        Ok(proto::GetPathMetadataResponse {
            exists: metadata.is_some(),
            is_dir,
            path: expanded,
        })
    }

    pub async fn handle_shutdown_remote_server(
        _this: Entity<Self>,
        _envelope: TypedEnvelope<proto::ShutdownRemoteServer>,
        cx: AsyncApp,
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
        _this: Entity<Self>,
        _envelope: TypedEnvelope<proto::Ping>,
        _cx: AsyncApp,
    ) -> Result<proto::Ack> {
        log::debug!("Received ping from client");
        Ok(proto::Ack {})
    }

    async fn handle_stage(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Stage>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let entries = envelope
            .payload
            .paths
            .into_iter()
            .map(PathBuf::from)
            .map(RepoPath::new)
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.stage_entries(entries)
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_unstage(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Unstage>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let entries = envelope
            .payload
            .paths
            .into_iter()
            .map(PathBuf::from)
            .map(RepoPath::new)
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.unstage_entries(entries)
            })?
            .await??;

        Ok(proto::Ack {})
    }

    async fn handle_commit(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Commit>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let name = envelope.payload.name.map(SharedString::from);
        let email = envelope.payload.email.map(SharedString::from);

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.commit(name.zip(email))
            })?
            .await??;

        Ok(proto::Ack {})
    }

    async fn handle_open_commit_message_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenCommitMessageBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let git_repository = match repository_handle.update(&mut cx, |repository_handle, _| {
            repository_handle.git_repo.clone()
        })? {
            GitRepo::Local(git_repository) => git_repository,
            GitRepo::Remote { .. } => {
                anyhow::bail!("Cannot handle open commit message buffer for remote git repo")
            }
        };
        let commit_message_file = git_repository.dot_git_dir().join(*COMMIT_MESSAGE);
        let fs = this.update(&mut cx, |headless_project, _| headless_project.fs.clone())?;
        fs.create_file(
            &commit_message_file,
            CreateOptions {
                overwrite: false,
                ignore_if_exists: true,
            },
        )
        .await
        .with_context(|| format!("creating commit message file {commit_message_file:?}"))?;

        let (worktree, relative_path) = this
            .update(&mut cx, |headless_project, cx| {
                headless_project
                    .worktree_store
                    .update(cx, |worktree_store, cx| {
                        worktree_store.find_or_create_worktree(&commit_message_file, false, cx)
                    })
            })?
            .await
            .with_context(|| {
                format!("deriving worktree for commit message file {commit_message_file:?}")
            })?;

        let buffer = this
            .update(&mut cx, |headless_project, cx| {
                headless_project
                    .buffer_store
                    .update(cx, |buffer_store, cx| {
                        buffer_store.open_buffer(
                            ProjectPath {
                                worktree_id: worktree.read(cx).id(),
                                path: Arc::from(relative_path),
                            },
                            cx,
                        )
                    })
            })
            .with_context(|| {
                format!("opening buffer for commit message file {commit_message_file:?}")
            })?
            .await?;

        let buffer_id = buffer.read_with(&cx, |buffer, _| buffer.remote_id())?;
        this.update(&mut cx, |headless_project, cx| {
            headless_project
                .buffer_store
                .update(cx, |buffer_store, cx| {
                    buffer_store
                        .create_buffer_for_peer(&buffer, SSH_PEER_ID, cx)
                        .detach_and_log_err(cx);
                })
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    fn repository_for_request(
        this: &Entity<Self>,
        worktree_id: WorktreeId,
        work_directory_id: ProjectEntryId,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Repository>> {
        this.update(cx, |project, cx| {
            let repository_handle = project
                .git_state
                .read(cx)
                .all_repositories()
                .into_iter()
                .find(|repository_handle| {
                    repository_handle.read(cx).worktree_id == worktree_id
                        && repository_handle
                            .read(cx)
                            .repository_entry
                            .work_directory_id()
                            == work_directory_id
                })
                .context("missing repository handle")?;
            anyhow::Ok(repository_handle)
        })?
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
