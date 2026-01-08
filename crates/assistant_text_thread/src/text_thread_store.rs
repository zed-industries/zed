use crate::{
    SavedTextThread, SavedTextThreadMetadata, TextThread, TextThreadEvent, TextThreadId,
    TextThreadOperation, TextThreadVersion,
};
use anyhow::{Context as _, Result};
use assistant_slash_command::{SlashCommandId, SlashCommandWorkingSet};
use client::{Client, TypedEnvelope, proto};
use clock::ReplicaId;
use collections::HashMap;
use context_server::ContextServerId;
use fs::{Fs, RemoveOptions};
use futures::StreamExt;
use fuzzy::StringMatchCandidate;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, Task, WeakEntity};
use language::LanguageRegistry;
use paths::text_threads_dir;
use project::{
    Project,
    context_server_store::{ContextServerStatus, ContextServerStore},
};
use prompt_store::PromptBuilder;
use regex::Regex;
use rpc::AnyProtoClient;
use std::sync::LazyLock;
use std::{cmp::Reverse, ffi::OsStr, mem, path::Path, sync::Arc, time::Duration};
use util::{ResultExt, TryFutureExt};
use zed_env_vars::ZED_STATELESS;

pub(crate) fn init(client: &AnyProtoClient) {
    client.add_entity_message_handler(TextThreadStore::handle_advertise_contexts);
    client.add_entity_request_handler(TextThreadStore::handle_open_context);
    client.add_entity_request_handler(TextThreadStore::handle_create_context);
    client.add_entity_message_handler(TextThreadStore::handle_update_context);
    client.add_entity_request_handler(TextThreadStore::handle_synchronize_contexts);
}

#[derive(Clone)]
pub struct RemoteTextThreadMetadata {
    pub id: TextThreadId,
    pub summary: Option<String>,
}

pub struct TextThreadStore {
    text_threads: Vec<TextThreadHandle>,
    text_threads_metadata: Vec<SavedTextThreadMetadata>,
    context_server_slash_command_ids: HashMap<ContextServerId, Vec<SlashCommandId>>,
    host_text_threads: Vec<RemoteTextThreadMetadata>,
    fs: Arc<dyn Fs>,
    languages: Arc<LanguageRegistry>,
    slash_commands: Arc<SlashCommandWorkingSet>,
    _watch_updates: Task<Option<()>>,
    client: Arc<Client>,
    project: WeakEntity<Project>,
    project_is_shared: bool,
    client_subscription: Option<client::Subscription>,
    _project_subscriptions: Vec<gpui::Subscription>,
    prompt_builder: Arc<PromptBuilder>,
}

enum TextThreadHandle {
    Weak(WeakEntity<TextThread>),
    Strong(Entity<TextThread>),
}

impl TextThreadHandle {
    fn upgrade(&self) -> Option<Entity<TextThread>> {
        match self {
            TextThreadHandle::Weak(weak) => weak.upgrade(),
            TextThreadHandle::Strong(strong) => Some(strong.clone()),
        }
    }

    fn downgrade(&self) -> WeakEntity<TextThread> {
        match self {
            TextThreadHandle::Weak(weak) => weak.clone(),
            TextThreadHandle::Strong(strong) => strong.downgrade(),
        }
    }
}

impl TextThreadStore {
    pub fn new(
        project: Entity<Project>,
        prompt_builder: Arc<PromptBuilder>,
        slash_commands: Arc<SlashCommandWorkingSet>,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let fs = project.read(cx).fs().clone();
        let languages = project.read(cx).languages().clone();
        cx.spawn(async move |cx| {
            const CONTEXT_WATCH_DURATION: Duration = Duration::from_millis(100);
            let (mut events, _) = fs.watch(text_threads_dir(), CONTEXT_WATCH_DURATION).await;

            let this = cx.new(|cx: &mut Context<Self>| {
                let mut this = Self {
                    text_threads: Vec::new(),
                    text_threads_metadata: Vec::new(),
                    context_server_slash_command_ids: HashMap::default(),
                    host_text_threads: Vec::new(),
                    fs,
                    languages,
                    slash_commands,
                    _watch_updates: cx.spawn(async move |this, cx| {
                        async move {
                            while events.next().await.is_some() {
                                this.update(cx, |this, cx| this.reload(cx))?.await.log_err();
                            }
                            anyhow::Ok(())
                        }
                        .log_err()
                        .await
                    }),
                    client_subscription: None,
                    _project_subscriptions: vec![
                        cx.subscribe(&project, Self::handle_project_event),
                    ],
                    project_is_shared: false,
                    client: project.read(cx).client(),
                    project: project.downgrade(),
                    prompt_builder,
                };
                this.handle_project_shared(cx);
                this.synchronize_contexts(cx);
                this.register_context_server_handlers(cx);
                this.reload(cx).detach_and_log_err(cx);
                this
            });

            Ok(this)
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        Self {
            text_threads: Default::default(),
            text_threads_metadata: Default::default(),
            context_server_slash_command_ids: Default::default(),
            host_text_threads: Default::default(),
            fs: project.read(cx).fs().clone(),
            languages: project.read(cx).languages().clone(),
            slash_commands: Arc::default(),
            _watch_updates: Task::ready(None),
            client: project.read(cx).client(),
            project: project.downgrade(),
            project_is_shared: false,
            client_subscription: None,
            _project_subscriptions: Default::default(),
            prompt_builder: Arc::new(PromptBuilder::new(None).unwrap()),
        }
    }

    async fn handle_advertise_contexts(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::AdvertiseContexts>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.host_text_threads = envelope
                .payload
                .contexts
                .into_iter()
                .map(|text_thread| RemoteTextThreadMetadata {
                    id: TextThreadId::from_proto(text_thread.context_id),
                    summary: text_thread.summary,
                })
                .collect();
            cx.notify();
        });
        Ok(())
    }

    async fn handle_open_context(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenContext>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenContextResponse> {
        let context_id = TextThreadId::from_proto(envelope.payload.context_id);
        let operations = this.update(&mut cx, |this, cx| {
            let project = this.project.upgrade().context("project not found")?;

            anyhow::ensure!(
                !project.read(cx).is_via_collab(),
                "only the host contexts can be opened"
            );

            let text_thread = this
                .loaded_text_thread_for_id(&context_id, cx)
                .context("context not found")?;
            anyhow::ensure!(
                text_thread.read(cx).replica_id() == ReplicaId::default(),
                "context must be opened via the host"
            );

            anyhow::Ok(
                text_thread
                    .read(cx)
                    .serialize_ops(&TextThreadVersion::default(), cx),
            )
        })?;
        let operations = operations.await;
        Ok(proto::OpenContextResponse {
            context: Some(proto::Context { operations }),
        })
    }

    async fn handle_create_context(
        this: Entity<Self>,
        _: TypedEnvelope<proto::CreateContext>,
        mut cx: AsyncApp,
    ) -> Result<proto::CreateContextResponse> {
        let (context_id, operations) = this.update(&mut cx, |this, cx| {
            let project = this.project.upgrade().context("project not found")?;
            anyhow::ensure!(
                !project.read(cx).is_via_collab(),
                "can only create contexts as the host"
            );

            let text_thread = this.create(cx);
            let context_id = text_thread.read(cx).id().clone();

            anyhow::Ok((
                context_id,
                text_thread
                    .read(cx)
                    .serialize_ops(&TextThreadVersion::default(), cx),
            ))
        })?;
        let operations = operations.await;
        Ok(proto::CreateContextResponse {
            context_id: context_id.to_proto(),
            context: Some(proto::Context { operations }),
        })
    }

    async fn handle_update_context(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::UpdateContext>,
        mut cx: AsyncApp,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let context_id = TextThreadId::from_proto(envelope.payload.context_id);
            if let Some(text_thread) = this.loaded_text_thread_for_id(&context_id, cx) {
                let operation_proto = envelope.payload.operation.context("invalid operation")?;
                let operation = TextThreadOperation::from_proto(operation_proto)?;
                text_thread.update(cx, |text_thread, cx| text_thread.apply_ops([operation], cx));
            }
            Ok(())
        })
    }

    async fn handle_synchronize_contexts(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SynchronizeContexts>,
        mut cx: AsyncApp,
    ) -> Result<proto::SynchronizeContextsResponse> {
        this.update(&mut cx, |this, cx| {
            let project = this.project.upgrade().context("project not found")?;
            anyhow::ensure!(
                !project.read(cx).is_via_collab(),
                "only the host can synchronize contexts"
            );

            let mut local_versions = Vec::new();
            for remote_version_proto in envelope.payload.contexts {
                let remote_version = TextThreadVersion::from_proto(&remote_version_proto);
                let context_id = TextThreadId::from_proto(remote_version_proto.context_id);
                if let Some(text_thread) = this.loaded_text_thread_for_id(&context_id, cx) {
                    let text_thread = text_thread.read(cx);
                    let operations = text_thread.serialize_ops(&remote_version, cx);
                    local_versions.push(text_thread.version(cx).to_proto(context_id.clone()));
                    let client = this.client.clone();
                    let project_id = envelope.payload.project_id;
                    cx.background_spawn(async move {
                        let operations = operations.await;
                        for operation in operations {
                            client.send(proto::UpdateContext {
                                project_id,
                                context_id: context_id.to_proto(),
                                operation: Some(operation),
                            })?;
                        }
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                }
            }

            this.advertise_contexts(cx);

            anyhow::Ok(proto::SynchronizeContextsResponse {
                contexts: local_versions,
            })
        })
    }

    fn handle_project_shared(&mut self, cx: &mut Context<Self>) {
        let Some(project) = self.project.upgrade() else {
            return;
        };

        let is_shared = project.read(cx).is_shared();
        let was_shared = mem::replace(&mut self.project_is_shared, is_shared);
        if is_shared == was_shared {
            return;
        }

        if is_shared {
            self.text_threads.retain_mut(|text_thread| {
                if let Some(strong_context) = text_thread.upgrade() {
                    *text_thread = TextThreadHandle::Strong(strong_context);
                    true
                } else {
                    false
                }
            });
            let remote_id = project.read(cx).remote_id().unwrap();
            self.client_subscription = self
                .client
                .subscribe_to_entity(remote_id)
                .log_err()
                .map(|subscription| subscription.set_entity(&cx.entity(), &cx.to_async()));
            self.advertise_contexts(cx);
        } else {
            self.client_subscription = None;
        }
    }

    fn handle_project_event(
        &mut self,
        _project: Entity<Project>,
        event: &project::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            project::Event::RemoteIdChanged(_) => {
                self.handle_project_shared(cx);
            }
            project::Event::Reshared => {
                self.advertise_contexts(cx);
            }
            project::Event::HostReshared | project::Event::Rejoined => {
                self.synchronize_contexts(cx);
            }
            project::Event::DisconnectedFromHost => {
                self.text_threads.retain_mut(|text_thread| {
                    if let Some(strong_context) = text_thread.upgrade() {
                        *text_thread = TextThreadHandle::Weak(text_thread.downgrade());
                        strong_context.update(cx, |text_thread, cx| {
                            if text_thread.replica_id() != ReplicaId::default() {
                                text_thread.set_capability(language::Capability::ReadOnly, cx);
                            }
                        });
                        true
                    } else {
                        false
                    }
                });
                self.host_text_threads.clear();
                cx.notify();
            }
            _ => {}
        }
    }

    pub fn unordered_text_threads(&self) -> impl Iterator<Item = &SavedTextThreadMetadata> {
        self.text_threads_metadata.iter()
    }

    pub fn host_text_threads(&self) -> impl Iterator<Item = &RemoteTextThreadMetadata> {
        self.host_text_threads.iter()
    }

    pub fn create(&mut self, cx: &mut Context<Self>) -> Entity<TextThread> {
        let context = cx.new(|cx| {
            TextThread::local(
                self.languages.clone(),
                Some(self.project.clone()),
                self.prompt_builder.clone(),
                self.slash_commands.clone(),
                cx,
            )
        });
        self.register_text_thread(&context, cx);
        context
    }

    pub fn create_remote(&mut self, cx: &mut Context<Self>) -> Task<Result<Entity<TextThread>>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("project was dropped")));
        };
        let project = project.read(cx);
        let Some(project_id) = project.remote_id() else {
            return Task::ready(Err(anyhow::anyhow!("project was not remote")));
        };

        let replica_id = project.replica_id();
        let capability = project.capability();
        let language_registry = self.languages.clone();
        let project = self.project.clone();

        let prompt_builder = self.prompt_builder.clone();
        let slash_commands = self.slash_commands.clone();
        let request = self.client.request(proto::CreateContext { project_id });
        cx.spawn(async move |this, cx| {
            let response = request.await?;
            let context_id = TextThreadId::from_proto(response.context_id);
            let context_proto = response.context.context("invalid context")?;
            let text_thread = cx.new(|cx| {
                TextThread::new(
                    context_id.clone(),
                    replica_id,
                    capability,
                    language_registry,
                    prompt_builder,
                    slash_commands,
                    Some(project),
                    cx,
                )
            });
            let operations = cx
                .background_spawn(async move {
                    context_proto
                        .operations
                        .into_iter()
                        .map(TextThreadOperation::from_proto)
                        .collect::<Result<Vec<_>>>()
                })
                .await?;
            text_thread.update(cx, |context, cx| context.apply_ops(operations, cx));
            this.update(cx, |this, cx| {
                if let Some(existing_context) = this.loaded_text_thread_for_id(&context_id, cx) {
                    existing_context
                } else {
                    this.register_text_thread(&text_thread, cx);
                    this.synchronize_contexts(cx);
                    text_thread
                }
            })
        })
    }

    pub fn open_local(
        &mut self,
        path: Arc<Path>,
        cx: &Context<Self>,
    ) -> Task<Result<Entity<TextThread>>> {
        if let Some(existing_context) = self.loaded_text_thread_for_path(&path, cx) {
            return Task::ready(Ok(existing_context));
        }

        let fs = self.fs.clone();
        let languages = self.languages.clone();
        let project = self.project.clone();
        let load = cx.background_spawn({
            let path = path.clone();
            async move {
                let saved_context = fs.load(&path).await?;
                SavedTextThread::from_json(&saved_context)
            }
        });
        let prompt_builder = self.prompt_builder.clone();
        let slash_commands = self.slash_commands.clone();

        cx.spawn(async move |this, cx| {
            let saved_context = load.await?;
            let context = cx.new(|cx| {
                TextThread::deserialize(
                    saved_context,
                    path.clone(),
                    languages,
                    prompt_builder,
                    slash_commands,
                    Some(project),
                    cx,
                )
            });
            this.update(cx, |this, cx| {
                if let Some(existing_context) = this.loaded_text_thread_for_path(&path, cx) {
                    existing_context
                } else {
                    this.register_text_thread(&context, cx);
                    context
                }
            })
        })
    }

    pub fn delete_local(&mut self, path: Arc<Path>, cx: &mut Context<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();

        cx.spawn(async move |this, cx| {
            fs.remove_file(
                &path,
                RemoveOptions {
                    recursive: false,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

            this.update(cx, |this, cx| {
                this.text_threads.retain(|text_thread| {
                    text_thread
                        .upgrade()
                        .and_then(|text_thread| text_thread.read(cx).path())
                        != Some(&path)
                });
                this.text_threads_metadata
                    .retain(|text_thread| text_thread.path.as_ref() != path.as_ref());
            })?;

            Ok(())
        })
    }

    fn loaded_text_thread_for_path(&self, path: &Path, cx: &App) -> Option<Entity<TextThread>> {
        self.text_threads.iter().find_map(|text_thread| {
            let text_thread = text_thread.upgrade()?;
            if text_thread.read(cx).path().map(Arc::as_ref) == Some(path) {
                Some(text_thread)
            } else {
                None
            }
        })
    }

    pub fn loaded_text_thread_for_id(
        &self,
        id: &TextThreadId,
        cx: &App,
    ) -> Option<Entity<TextThread>> {
        self.text_threads.iter().find_map(|text_thread| {
            let text_thread = text_thread.upgrade()?;
            if text_thread.read(cx).id() == id {
                Some(text_thread)
            } else {
                None
            }
        })
    }

    pub fn open_remote(
        &mut self,
        text_thread_id: TextThreadId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<TextThread>>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("project was dropped")));
        };
        let project = project.read(cx);
        let Some(project_id) = project.remote_id() else {
            return Task::ready(Err(anyhow::anyhow!("project was not remote")));
        };

        if let Some(context) = self.loaded_text_thread_for_id(&text_thread_id, cx) {
            return Task::ready(Ok(context));
        }

        let replica_id = project.replica_id();
        let capability = project.capability();
        let language_registry = self.languages.clone();
        let project = self.project.clone();
        let request = self.client.request(proto::OpenContext {
            project_id,
            context_id: text_thread_id.to_proto(),
        });
        let prompt_builder = self.prompt_builder.clone();
        let slash_commands = self.slash_commands.clone();
        cx.spawn(async move |this, cx| {
            let response = request.await?;
            let context_proto = response.context.context("invalid context")?;
            let text_thread = cx.new(|cx| {
                TextThread::new(
                    text_thread_id.clone(),
                    replica_id,
                    capability,
                    language_registry,
                    prompt_builder,
                    slash_commands,
                    Some(project),
                    cx,
                )
            });
            let operations = cx
                .background_spawn(async move {
                    context_proto
                        .operations
                        .into_iter()
                        .map(TextThreadOperation::from_proto)
                        .collect::<Result<Vec<_>>>()
                })
                .await?;
            text_thread.update(cx, |context, cx| context.apply_ops(operations, cx));
            this.update(cx, |this, cx| {
                if let Some(existing_context) = this.loaded_text_thread_for_id(&text_thread_id, cx)
                {
                    existing_context
                } else {
                    this.register_text_thread(&text_thread, cx);
                    this.synchronize_contexts(cx);
                    text_thread
                }
            })
        })
    }

    fn register_text_thread(&mut self, text_thread: &Entity<TextThread>, cx: &mut Context<Self>) {
        let handle = if self.project_is_shared {
            TextThreadHandle::Strong(text_thread.clone())
        } else {
            TextThreadHandle::Weak(text_thread.downgrade())
        };
        self.text_threads.push(handle);
        self.advertise_contexts(cx);
        cx.subscribe(text_thread, Self::handle_context_event)
            .detach();
    }

    fn handle_context_event(
        &mut self,
        text_thread: Entity<TextThread>,
        event: &TextThreadEvent,
        cx: &mut Context<Self>,
    ) {
        let Some(project) = self.project.upgrade() else {
            return;
        };
        let Some(project_id) = project.read(cx).remote_id() else {
            return;
        };

        match event {
            TextThreadEvent::SummaryChanged => {
                self.advertise_contexts(cx);
            }
            TextThreadEvent::PathChanged { old_path, new_path } => {
                if let Some(old_path) = old_path.as_ref() {
                    for metadata in &mut self.text_threads_metadata {
                        if &metadata.path == old_path {
                            metadata.path = new_path.clone();
                            break;
                        }
                    }
                }
            }
            TextThreadEvent::Operation(operation) => {
                let context_id = text_thread.read(cx).id().to_proto();
                let operation = operation.to_proto();
                self.client
                    .send(proto::UpdateContext {
                        project_id,
                        context_id,
                        operation: Some(operation),
                    })
                    .log_err();
            }
            _ => {}
        }
    }

    fn advertise_contexts(&self, cx: &App) {
        let Some(project) = self.project.upgrade() else {
            return;
        };
        let Some(project_id) = project.read(cx).remote_id() else {
            return;
        };
        // For now, only the host can advertise their open contexts.
        if project.read(cx).is_via_collab() {
            return;
        }

        let contexts = self
            .text_threads
            .iter()
            .rev()
            .filter_map(|text_thread| {
                let text_thread = text_thread.upgrade()?.read(cx);
                if text_thread.replica_id() == ReplicaId::default() {
                    Some(proto::ContextMetadata {
                        context_id: text_thread.id().to_proto(),
                        summary: text_thread
                            .summary()
                            .content()
                            .map(|summary| summary.text.clone()),
                    })
                } else {
                    None
                }
            })
            .collect();
        self.client
            .send(proto::AdvertiseContexts {
                project_id,
                contexts,
            })
            .ok();
    }

    fn synchronize_contexts(&mut self, cx: &mut Context<Self>) {
        let Some(project) = self.project.upgrade() else {
            return;
        };
        let Some(project_id) = project.read(cx).remote_id() else {
            return;
        };

        let text_threads = self
            .text_threads
            .iter()
            .filter_map(|text_thread| {
                let text_thread = text_thread.upgrade()?.read(cx);
                if text_thread.replica_id() != ReplicaId::default() {
                    Some(text_thread.version(cx).to_proto(text_thread.id().clone()))
                } else {
                    None
                }
            })
            .collect();

        let client = self.client.clone();
        let request = self.client.request(proto::SynchronizeContexts {
            project_id,
            contexts: text_threads,
        });
        cx.spawn(async move |this, cx| {
            let response = request.await?;

            let mut text_thread_ids = Vec::new();
            let mut operations = Vec::new();
            this.read_with(cx, |this, cx| {
                for context_version_proto in response.contexts {
                    let text_thread_version = TextThreadVersion::from_proto(&context_version_proto);
                    let text_thread_id = TextThreadId::from_proto(context_version_proto.context_id);
                    if let Some(text_thread) = this.loaded_text_thread_for_id(&text_thread_id, cx) {
                        text_thread_ids.push(text_thread_id);
                        operations
                            .push(text_thread.read(cx).serialize_ops(&text_thread_version, cx));
                    }
                }
            })?;

            let operations = futures::future::join_all(operations).await;
            for (context_id, operations) in text_thread_ids.into_iter().zip(operations) {
                for operation in operations {
                    client.send(proto::UpdateContext {
                        project_id,
                        context_id: context_id.to_proto(),
                        operation: Some(operation),
                    })?;
                }
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    pub fn search(&self, query: String, cx: &App) -> Task<Vec<SavedTextThreadMetadata>> {
        let metadata = self.text_threads_metadata.clone();
        let executor = cx.background_executor().clone();
        cx.background_spawn(async move {
            if query.is_empty() {
                metadata
            } else {
                let candidates = metadata
                    .iter()
                    .enumerate()
                    .map(|(id, metadata)| StringMatchCandidate::new(id, &metadata.title))
                    .collect::<Vec<_>>();
                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    executor,
                )
                .await;

                matches
                    .into_iter()
                    .map(|mat| metadata[mat.candidate_id].clone())
                    .collect()
            }
        })
    }

    fn reload(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        cx.spawn(async move |this, cx| {
            if *ZED_STATELESS {
                return Ok(());
            }
            fs.create_dir(text_threads_dir()).await?;

            let mut paths = fs.read_dir(text_threads_dir()).await?;
            let mut contexts = Vec::<SavedTextThreadMetadata>::new();
            while let Some(path) = paths.next().await {
                let path = path?;
                if path.extension() != Some(OsStr::new("json")) {
                    continue;
                }

                static ASSISTANT_CONTEXT_REGEX: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r" - \d+.zed.json$").unwrap());

                let metadata = fs.metadata(&path).await?;
                if let Some((file_name, metadata)) = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .zip(metadata)
                {
                    // This is used to filter out contexts saved by the new assistant.
                    if !ASSISTANT_CONTEXT_REGEX.is_match(file_name) {
                        continue;
                    }

                    if let Some(title) = ASSISTANT_CONTEXT_REGEX
                        .replace(file_name, "")
                        .lines()
                        .next()
                    {
                        contexts.push(SavedTextThreadMetadata {
                            title: title.to_string().into(),
                            path: path.into(),
                            mtime: metadata.mtime.timestamp_for_user().into(),
                        });
                    }
                }
            }
            contexts.sort_unstable_by_key(|text_thread| Reverse(text_thread.mtime));

            this.update(cx, |this, cx| {
                this.text_threads_metadata = contexts;
                cx.notify();
            })
        })
    }

    fn register_context_server_handlers(&self, cx: &mut Context<Self>) {
        let Some(project) = self.project.upgrade() else {
            return;
        };
        let context_server_store = project.read(cx).context_server_store();
        cx.subscribe(&context_server_store, Self::handle_context_server_event)
            .detach();

        // Check for any servers that were already running before the handler was registered
        for server in context_server_store.read(cx).running_servers() {
            self.load_context_server_slash_commands(server.id(), context_server_store.clone(), cx);
        }
    }

    fn handle_context_server_event(
        &mut self,
        context_server_store: Entity<ContextServerStore>,
        event: &project::context_server_store::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            project::context_server_store::Event::ServerStatusChanged { server_id, status } => {
                match status {
                    ContextServerStatus::Running => {
                        self.load_context_server_slash_commands(
                            server_id.clone(),
                            context_server_store,
                            cx,
                        );
                    }
                    ContextServerStatus::Stopped | ContextServerStatus::Error(_) => {
                        if let Some(slash_command_ids) =
                            self.context_server_slash_command_ids.remove(server_id)
                        {
                            self.slash_commands.remove(&slash_command_ids);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn load_context_server_slash_commands(
        &self,
        server_id: ContextServerId,
        context_server_store: Entity<ContextServerStore>,
        cx: &mut Context<Self>,
    ) {
        let Some(server) = context_server_store.read(cx).get_running_server(&server_id) else {
            return;
        };
        let slash_command_working_set = self.slash_commands.clone();
        cx.spawn(async move |this, cx| {
            let Some(protocol) = server.client() else {
                return;
            };

            if protocol.capable(context_server::protocol::ServerCapability::Prompts)
                && let Some(response) = protocol
                    .request::<context_server::types::requests::PromptsList>(())
                    .await
                    .log_err()
            {
                let slash_command_ids = response
                    .prompts
                    .into_iter()
                    .filter(assistant_slash_commands::acceptable_prompt)
                    .map(|prompt| {
                        log::info!("registering context server command: {:?}", prompt.name);
                        slash_command_working_set.insert(Arc::new(
                            assistant_slash_commands::ContextServerSlashCommand::new(
                                context_server_store.clone(),
                                server.id(),
                                prompt,
                            ),
                        ))
                    })
                    .collect::<Vec<_>>();

                this.update(cx, |this, _cx| {
                    this.context_server_slash_command_ids
                        .insert(server_id.clone(), slash_command_ids);
                })
                .log_err();
            }
        })
        .detach();
    }
}
