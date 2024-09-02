use crate::{
    prompts::PromptBuilder, Context, ContextEvent, ContextId, ContextOperation, ContextVersion,
    SavedContext, SavedContextMetadata,
};
use ::proto::AnyProtoClient;
use anyhow::{anyhow, Context as _, Result};
use client::{proto, telemetry::Telemetry, Client, TypedEnvelope};
use clock::ReplicaId;
use fs::Fs;
use futures::StreamExt;
use fuzzy::StringMatchCandidate;
use gpui::{
    AppContext, AsyncAppContext, Context as _, EventEmitter, Model, ModelContext, Task, WeakModel,
};
use language::LanguageRegistry;
use paths::contexts_dir;
use project::Project;
use regex::Regex;
use std::{
    cmp::Reverse,
    ffi::OsStr,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use util::{ResultExt, TryFutureExt};

pub fn init(client: &AnyProtoClient) {
    client.add_model_message_handler(ContextStore::handle_advertise_contexts);
    client.add_model_request_handler(ContextStore::handle_open_context);
    client.add_model_request_handler(ContextStore::handle_create_context);
    client.add_model_message_handler(ContextStore::handle_update_context);
    client.add_model_request_handler(ContextStore::handle_synchronize_contexts);
}

#[derive(Clone)]
pub struct RemoteContextMetadata {
    pub id: ContextId,
    pub summary: Option<String>,
}

pub struct ContextStore {
    contexts: Vec<ContextHandle>,
    contexts_metadata: Vec<SavedContextMetadata>,
    host_contexts: Vec<RemoteContextMetadata>,
    fs: Arc<dyn Fs>,
    languages: Arc<LanguageRegistry>,
    telemetry: Arc<Telemetry>,
    _watch_updates: Task<Option<()>>,
    client: Arc<Client>,
    project: Model<Project>,
    project_is_shared: bool,
    client_subscription: Option<client::Subscription>,
    _project_subscriptions: Vec<gpui::Subscription>,
    prompt_builder: Arc<PromptBuilder>,
}

pub enum ContextStoreEvent {
    ContextCreated(ContextId),
}

impl EventEmitter<ContextStoreEvent> for ContextStore {}

enum ContextHandle {
    Weak(WeakModel<Context>),
    Strong(Model<Context>),
}

impl ContextHandle {
    fn upgrade(&self) -> Option<Model<Context>> {
        match self {
            ContextHandle::Weak(weak) => weak.upgrade(),
            ContextHandle::Strong(strong) => Some(strong.clone()),
        }
    }

    fn downgrade(&self) -> WeakModel<Context> {
        match self {
            ContextHandle::Weak(weak) => weak.clone(),
            ContextHandle::Strong(strong) => strong.downgrade(),
        }
    }
}

impl ContextStore {
    pub fn new(
        project: Model<Project>,
        prompt_builder: Arc<PromptBuilder>,
        cx: &mut AppContext,
    ) -> Task<Result<Model<Self>>> {
        let fs = project.read(cx).fs().clone();
        let languages = project.read(cx).languages().clone();
        let telemetry = project.read(cx).client().telemetry().clone();
        cx.spawn(|mut cx| async move {
            const CONTEXT_WATCH_DURATION: Duration = Duration::from_millis(100);
            let (mut events, _) = fs.watch(contexts_dir(), CONTEXT_WATCH_DURATION).await;

            let this = cx.new_model(|cx: &mut ModelContext<Self>| {
                let mut this = Self {
                    contexts: Vec::new(),
                    contexts_metadata: Vec::new(),
                    host_contexts: Vec::new(),
                    fs,
                    languages,
                    telemetry,
                    _watch_updates: cx.spawn(|this, mut cx| {
                        async move {
                            while events.next().await.is_some() {
                                this.update(&mut cx, |this, cx| this.reload(cx))?
                                    .await
                                    .log_err();
                            }
                            anyhow::Ok(())
                        }
                        .log_err()
                    }),
                    client_subscription: None,
                    _project_subscriptions: vec![
                        cx.observe(&project, Self::handle_project_changed),
                        cx.subscribe(&project, Self::handle_project_event),
                    ],
                    project_is_shared: false,
                    client: project.read(cx).client(),
                    project: project.clone(),
                    prompt_builder,
                };
                this.handle_project_changed(project, cx);
                this.synchronize_contexts(cx);
                this
            })?;
            this.update(&mut cx, |this, cx| this.reload(cx))?
                .await
                .log_err();
            Ok(this)
        })
    }

    async fn handle_advertise_contexts(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::AdvertiseContexts>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.host_contexts = envelope
                .payload
                .contexts
                .into_iter()
                .map(|context| RemoteContextMetadata {
                    id: ContextId::from_proto(context.context_id),
                    summary: context.summary,
                })
                .collect();
            cx.notify();
        })
    }

    async fn handle_open_context(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::OpenContext>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::OpenContextResponse> {
        let context_id = ContextId::from_proto(envelope.payload.context_id);
        let operations = this.update(&mut cx, |this, cx| {
            if this.project.read(cx).is_via_collab() {
                return Err(anyhow!("only the host contexts can be opened"));
            }

            let context = this
                .loaded_context_for_id(&context_id, cx)
                .context("context not found")?;
            if context.read(cx).replica_id() != ReplicaId::default() {
                return Err(anyhow!("context must be opened via the host"));
            }

            anyhow::Ok(
                context
                    .read(cx)
                    .serialize_ops(&ContextVersion::default(), cx),
            )
        })??;
        let operations = operations.await;
        Ok(proto::OpenContextResponse {
            context: Some(proto::Context { operations }),
        })
    }

    async fn handle_create_context(
        this: Model<Self>,
        _: TypedEnvelope<proto::CreateContext>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::CreateContextResponse> {
        let (context_id, operations) = this.update(&mut cx, |this, cx| {
            if this.project.read(cx).is_via_collab() {
                return Err(anyhow!("can only create contexts as the host"));
            }

            let context = this.create(cx);
            let context_id = context.read(cx).id().clone();
            cx.emit(ContextStoreEvent::ContextCreated(context_id.clone()));

            anyhow::Ok((
                context_id,
                context
                    .read(cx)
                    .serialize_ops(&ContextVersion::default(), cx),
            ))
        })??;
        let operations = operations.await;
        Ok(proto::CreateContextResponse {
            context_id: context_id.to_proto(),
            context: Some(proto::Context { operations }),
        })
    }

    async fn handle_update_context(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::UpdateContext>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            let context_id = ContextId::from_proto(envelope.payload.context_id);
            if let Some(context) = this.loaded_context_for_id(&context_id, cx) {
                let operation_proto = envelope.payload.operation.context("invalid operation")?;
                let operation = ContextOperation::from_proto(operation_proto)?;
                context.update(cx, |context, cx| context.apply_ops([operation], cx))?;
            }
            Ok(())
        })?
    }

    async fn handle_synchronize_contexts(
        this: Model<Self>,
        envelope: TypedEnvelope<proto::SynchronizeContexts>,
        mut cx: AsyncAppContext,
    ) -> Result<proto::SynchronizeContextsResponse> {
        this.update(&mut cx, |this, cx| {
            if this.project.read(cx).is_via_collab() {
                return Err(anyhow!("only the host can synchronize contexts"));
            }

            let mut local_versions = Vec::new();
            for remote_version_proto in envelope.payload.contexts {
                let remote_version = ContextVersion::from_proto(&remote_version_proto);
                let context_id = ContextId::from_proto(remote_version_proto.context_id);
                if let Some(context) = this.loaded_context_for_id(&context_id, cx) {
                    let context = context.read(cx);
                    let operations = context.serialize_ops(&remote_version, cx);
                    local_versions.push(context.version(cx).to_proto(context_id.clone()));
                    let client = this.client.clone();
                    let project_id = envelope.payload.project_id;
                    cx.background_executor()
                        .spawn(async move {
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
        })?
    }

    fn handle_project_changed(&mut self, _: Model<Project>, cx: &mut ModelContext<Self>) {
        let is_shared = self.project.read(cx).is_shared();
        let was_shared = mem::replace(&mut self.project_is_shared, is_shared);
        if is_shared == was_shared {
            return;
        }

        if is_shared {
            self.contexts.retain_mut(|context| {
                if let Some(strong_context) = context.upgrade() {
                    *context = ContextHandle::Strong(strong_context);
                    true
                } else {
                    false
                }
            });
            let remote_id = self.project.read(cx).remote_id().unwrap();
            self.client_subscription = self
                .client
                .subscribe_to_entity(remote_id)
                .log_err()
                .map(|subscription| subscription.set_model(&cx.handle(), &mut cx.to_async()));
            self.advertise_contexts(cx);
        } else {
            self.client_subscription = None;
        }
    }

    fn handle_project_event(
        &mut self,
        _: Model<Project>,
        event: &project::Event,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            project::Event::Reshared => {
                self.advertise_contexts(cx);
            }
            project::Event::HostReshared | project::Event::Rejoined => {
                self.synchronize_contexts(cx);
            }
            project::Event::DisconnectedFromHost => {
                self.contexts.retain_mut(|context| {
                    if let Some(strong_context) = context.upgrade() {
                        *context = ContextHandle::Weak(context.downgrade());
                        strong_context.update(cx, |context, cx| {
                            if context.replica_id() != ReplicaId::default() {
                                context.set_capability(language::Capability::ReadOnly, cx);
                            }
                        });
                        true
                    } else {
                        false
                    }
                });
                self.host_contexts.clear();
                cx.notify();
            }
            _ => {}
        }
    }

    pub fn create(&mut self, cx: &mut ModelContext<Self>) -> Model<Context> {
        let context = cx.new_model(|cx| {
            Context::local(
                self.languages.clone(),
                Some(self.project.clone()),
                Some(self.telemetry.clone()),
                self.prompt_builder.clone(),
                cx,
            )
        });
        self.register_context(&context, cx);
        context
    }

    pub fn create_remote_context(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Context>>> {
        let project = self.project.read(cx);
        let Some(project_id) = project.remote_id() else {
            return Task::ready(Err(anyhow!("project was not remote")));
        };
        if project.is_local_or_ssh() {
            return Task::ready(Err(anyhow!("cannot create remote contexts as the host")));
        }

        let replica_id = project.replica_id();
        let capability = project.capability();
        let language_registry = self.languages.clone();
        let project = self.project.clone();
        let telemetry = self.telemetry.clone();
        let prompt_builder = self.prompt_builder.clone();
        let request = self.client.request(proto::CreateContext { project_id });
        cx.spawn(|this, mut cx| async move {
            let response = request.await?;
            let context_id = ContextId::from_proto(response.context_id);
            let context_proto = response.context.context("invalid context")?;
            let context = cx.new_model(|cx| {
                Context::new(
                    context_id.clone(),
                    replica_id,
                    capability,
                    language_registry,
                    prompt_builder,
                    Some(project),
                    Some(telemetry),
                    cx,
                )
            })?;
            let operations = cx
                .background_executor()
                .spawn(async move {
                    context_proto
                        .operations
                        .into_iter()
                        .map(|op| ContextOperation::from_proto(op))
                        .collect::<Result<Vec<_>>>()
                })
                .await?;
            context.update(&mut cx, |context, cx| context.apply_ops(operations, cx))??;
            this.update(&mut cx, |this, cx| {
                if let Some(existing_context) = this.loaded_context_for_id(&context_id, cx) {
                    existing_context
                } else {
                    this.register_context(&context, cx);
                    this.synchronize_contexts(cx);
                    context
                }
            })
        })
    }

    pub fn open_local_context(
        &mut self,
        path: PathBuf,
        cx: &ModelContext<Self>,
    ) -> Task<Result<Model<Context>>> {
        if let Some(existing_context) = self.loaded_context_for_path(&path, cx) {
            return Task::ready(Ok(existing_context));
        }

        let fs = self.fs.clone();
        let languages = self.languages.clone();
        let project = self.project.clone();
        let telemetry = self.telemetry.clone();
        let load = cx.background_executor().spawn({
            let path = path.clone();
            async move {
                let saved_context = fs.load(&path).await?;
                SavedContext::from_json(&saved_context)
            }
        });
        let prompt_builder = self.prompt_builder.clone();

        cx.spawn(|this, mut cx| async move {
            let saved_context = load.await?;
            let context = cx.new_model(|cx| {
                Context::deserialize(
                    saved_context,
                    path.clone(),
                    languages,
                    prompt_builder,
                    Some(project),
                    Some(telemetry),
                    cx,
                )
            })?;
            this.update(&mut cx, |this, cx| {
                if let Some(existing_context) = this.loaded_context_for_path(&path, cx) {
                    existing_context
                } else {
                    this.register_context(&context, cx);
                    context
                }
            })
        })
    }

    fn loaded_context_for_path(&self, path: &Path, cx: &AppContext) -> Option<Model<Context>> {
        self.contexts.iter().find_map(|context| {
            let context = context.upgrade()?;
            if context.read(cx).path() == Some(path) {
                Some(context)
            } else {
                None
            }
        })
    }

    pub(super) fn loaded_context_for_id(
        &self,
        id: &ContextId,
        cx: &AppContext,
    ) -> Option<Model<Context>> {
        self.contexts.iter().find_map(|context| {
            let context = context.upgrade()?;
            if context.read(cx).id() == id {
                Some(context)
            } else {
                None
            }
        })
    }

    pub fn open_remote_context(
        &mut self,
        context_id: ContextId,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Model<Context>>> {
        let project = self.project.read(cx);
        let Some(project_id) = project.remote_id() else {
            return Task::ready(Err(anyhow!("project was not remote")));
        };
        if project.is_local_or_ssh() {
            return Task::ready(Err(anyhow!("cannot open remote contexts as the host")));
        }

        if let Some(context) = self.loaded_context_for_id(&context_id, cx) {
            return Task::ready(Ok(context));
        }

        let replica_id = project.replica_id();
        let capability = project.capability();
        let language_registry = self.languages.clone();
        let project = self.project.clone();
        let telemetry = self.telemetry.clone();
        let request = self.client.request(proto::OpenContext {
            project_id,
            context_id: context_id.to_proto(),
        });
        let prompt_builder = self.prompt_builder.clone();
        cx.spawn(|this, mut cx| async move {
            let response = request.await?;
            let context_proto = response.context.context("invalid context")?;
            let context = cx.new_model(|cx| {
                Context::new(
                    context_id.clone(),
                    replica_id,
                    capability,
                    language_registry,
                    prompt_builder,
                    Some(project),
                    Some(telemetry),
                    cx,
                )
            })?;
            let operations = cx
                .background_executor()
                .spawn(async move {
                    context_proto
                        .operations
                        .into_iter()
                        .map(|op| ContextOperation::from_proto(op))
                        .collect::<Result<Vec<_>>>()
                })
                .await?;
            context.update(&mut cx, |context, cx| context.apply_ops(operations, cx))??;
            this.update(&mut cx, |this, cx| {
                if let Some(existing_context) = this.loaded_context_for_id(&context_id, cx) {
                    existing_context
                } else {
                    this.register_context(&context, cx);
                    this.synchronize_contexts(cx);
                    context
                }
            })
        })
    }

    fn register_context(&mut self, context: &Model<Context>, cx: &mut ModelContext<Self>) {
        let handle = if self.project_is_shared {
            ContextHandle::Strong(context.clone())
        } else {
            ContextHandle::Weak(context.downgrade())
        };
        self.contexts.push(handle);
        self.advertise_contexts(cx);
        cx.subscribe(context, Self::handle_context_event).detach();
    }

    fn handle_context_event(
        &mut self,
        context: Model<Context>,
        event: &ContextEvent,
        cx: &mut ModelContext<Self>,
    ) {
        let Some(project_id) = self.project.read(cx).remote_id() else {
            return;
        };

        match event {
            ContextEvent::SummaryChanged => {
                self.advertise_contexts(cx);
            }
            ContextEvent::Operation(operation) => {
                let context_id = context.read(cx).id().to_proto();
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

    fn advertise_contexts(&self, cx: &AppContext) {
        let Some(project_id) = self.project.read(cx).remote_id() else {
            return;
        };

        // For now, only the host can advertise their open contexts.
        if self.project.read(cx).is_via_collab() {
            return;
        }

        let contexts = self
            .contexts
            .iter()
            .rev()
            .filter_map(|context| {
                let context = context.upgrade()?.read(cx);
                if context.replica_id() == ReplicaId::default() {
                    Some(proto::ContextMetadata {
                        context_id: context.id().to_proto(),
                        summary: context.summary().map(|summary| summary.text.clone()),
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

    fn synchronize_contexts(&mut self, cx: &mut ModelContext<Self>) {
        let Some(project_id) = self.project.read(cx).remote_id() else {
            return;
        };

        let contexts = self
            .contexts
            .iter()
            .filter_map(|context| {
                let context = context.upgrade()?.read(cx);
                if context.replica_id() != ReplicaId::default() {
                    Some(context.version(cx).to_proto(context.id().clone()))
                } else {
                    None
                }
            })
            .collect();

        let client = self.client.clone();
        let request = self.client.request(proto::SynchronizeContexts {
            project_id,
            contexts,
        });
        cx.spawn(|this, cx| async move {
            let response = request.await?;

            let mut context_ids = Vec::new();
            let mut operations = Vec::new();
            this.read_with(&cx, |this, cx| {
                for context_version_proto in response.contexts {
                    let context_version = ContextVersion::from_proto(&context_version_proto);
                    let context_id = ContextId::from_proto(context_version_proto.context_id);
                    if let Some(context) = this.loaded_context_for_id(&context_id, cx) {
                        context_ids.push(context_id);
                        operations.push(context.read(cx).serialize_ops(&context_version, cx));
                    }
                }
            })?;

            let operations = futures::future::join_all(operations).await;
            for (context_id, operations) in context_ids.into_iter().zip(operations) {
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

    pub fn search(&self, query: String, cx: &AppContext) -> Task<Vec<SavedContextMetadata>> {
        let metadata = self.contexts_metadata.clone();
        let executor = cx.background_executor().clone();
        cx.background_executor().spawn(async move {
            if query.is_empty() {
                metadata
            } else {
                let candidates = metadata
                    .iter()
                    .enumerate()
                    .map(|(id, metadata)| StringMatchCandidate::new(id, metadata.title.clone()))
                    .collect::<Vec<_>>();
                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
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

    pub fn host_contexts(&self) -> &[RemoteContextMetadata] {
        &self.host_contexts
    }

    fn reload(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            fs.create_dir(contexts_dir()).await?;

            let mut paths = fs.read_dir(contexts_dir()).await?;
            let mut contexts = Vec::<SavedContextMetadata>::new();
            while let Some(path) = paths.next().await {
                let path = path?;
                if path.extension() != Some(OsStr::new("json")) {
                    continue;
                }

                let pattern = r" - \d+.zed.json$";
                let re = Regex::new(pattern).unwrap();

                let metadata = fs.metadata(&path).await?;
                if let Some((file_name, metadata)) = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .zip(metadata)
                {
                    // This is used to filter out contexts saved by the new assistant.
                    if !re.is_match(file_name) {
                        continue;
                    }

                    if let Some(title) = re.replace(file_name, "").lines().next() {
                        contexts.push(SavedContextMetadata {
                            title: title.to_string(),
                            path,
                            mtime: metadata.mtime.into(),
                        });
                    }
                }
            }
            contexts.sort_unstable_by_key(|context| Reverse(context.mtime));

            this.update(&mut cx, |this, cx| {
                this.contexts_metadata = contexts;
                cx.notify();
            })
        })
    }
}
