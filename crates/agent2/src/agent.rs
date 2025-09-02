use crate::{
    ContextServerRegistry, Thread, ThreadEvent, ThreadsDatabase, ToolCallAuthorization,
    UserMessageContent, templates::Templates,
};
use crate::{HistoryStore, TerminalHandle, ThreadEnvironment, TitleUpdated, TokenUsageUpdated};
use acp_thread::{AcpThread, AgentModelSelector};
use action_log::ActionLog;
use agent_client_protocol as acp;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, anyhow};
use collections::{HashSet, IndexMap};
use fs::Fs;
use futures::channel::{mpsc, oneshot};
use futures::future::Shared;
use futures::{StreamExt, future};
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, SharedString, Subscription, Task, WeakEntity,
};
use language_model::{LanguageModel, LanguageModelProvider, LanguageModelRegistry};
use project::{Project, ProjectItem, ProjectPath, Worktree};
use prompt_store::{
    ProjectContext, PromptId, PromptStore, RulesFileContext, UserRulesContext, WorktreeContext,
};
use settings::update_settings_file;
use std::any::Any;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use util::ResultExt;

const RULES_FILE_NAMES: [&str; 9] = [
    ".rules",
    ".cursorrules",
    ".windsurfrules",
    ".clinerules",
    ".github/copilot-instructions.md",
    "CLAUDE.md",
    "AGENT.md",
    "AGENTS.md",
    "GEMINI.md",
];

pub struct RulesLoadingError {
    pub message: SharedString,
}

/// Holds both the internal Thread and the AcpThread for a session
struct Session {
    /// The internal thread that processes messages
    thread: Entity<Thread>,
    /// The ACP thread that handles protocol communication
    acp_thread: WeakEntity<acp_thread::AcpThread>,
    pending_save: Task<()>,
    _subscriptions: Vec<Subscription>,
}

pub struct LanguageModels {
    /// Access language model by ID
    models: HashMap<acp_thread::AgentModelId, Arc<dyn LanguageModel>>,
    /// Cached list for returning language model information
    model_list: acp_thread::AgentModelList,
    refresh_models_rx: watch::Receiver<()>,
    refresh_models_tx: watch::Sender<()>,
    _authenticate_all_providers_task: Task<()>,
}

impl LanguageModels {
    fn new(cx: &mut App) -> Self {
        let (refresh_models_tx, refresh_models_rx) = watch::channel(());

        let mut this = Self {
            models: HashMap::default(),
            model_list: acp_thread::AgentModelList::Grouped(IndexMap::default()),
            refresh_models_rx,
            refresh_models_tx,
            _authenticate_all_providers_task: Self::authenticate_all_language_model_providers(cx),
        };
        this.refresh_list(cx);
        this
    }

    fn refresh_list(&mut self, cx: &App) {
        let providers = LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .into_iter()
            .filter(|provider| provider.is_authenticated(cx))
            .collect::<Vec<_>>();

        let mut language_model_list = IndexMap::default();
        let mut recommended_models = HashSet::default();

        let mut recommended = Vec::new();
        for provider in &providers {
            for model in provider.recommended_models(cx) {
                recommended_models.insert((model.provider_id(), model.id()));
                recommended.push(Self::map_language_model_to_info(&model, provider));
            }
        }
        if !recommended.is_empty() {
            language_model_list.insert(
                acp_thread::AgentModelGroupName("Recommended".into()),
                recommended,
            );
        }

        let mut models = HashMap::default();
        for provider in providers {
            let mut provider_models = Vec::new();
            for model in provider.provided_models(cx) {
                let model_info = Self::map_language_model_to_info(&model, &provider);
                let model_id = model_info.id.clone();
                if !recommended_models.contains(&(model.provider_id(), model.id())) {
                    provider_models.push(model_info);
                }
                models.insert(model_id, model);
            }
            if !provider_models.is_empty() {
                language_model_list.insert(
                    acp_thread::AgentModelGroupName(provider.name().0.clone()),
                    provider_models,
                );
            }
        }

        self.models = models;
        self.model_list = acp_thread::AgentModelList::Grouped(language_model_list);
        self.refresh_models_tx.send(()).ok();
    }

    fn watch(&self) -> watch::Receiver<()> {
        self.refresh_models_rx.clone()
    }

    pub fn model_from_id(
        &self,
        model_id: &acp_thread::AgentModelId,
    ) -> Option<Arc<dyn LanguageModel>> {
        self.models.get(model_id).cloned()
    }

    fn map_language_model_to_info(
        model: &Arc<dyn LanguageModel>,
        provider: &Arc<dyn LanguageModelProvider>,
    ) -> acp_thread::AgentModelInfo {
        acp_thread::AgentModelInfo {
            id: Self::model_id(model),
            name: model.name().0,
            icon: Some(provider.icon()),
        }
    }

    fn model_id(model: &Arc<dyn LanguageModel>) -> acp_thread::AgentModelId {
        acp_thread::AgentModelId(format!("{}/{}", model.provider_id().0, model.id().0).into())
    }

    fn authenticate_all_language_model_providers(cx: &mut App) -> Task<()> {
        let authenticate_all_providers = LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .iter()
            .map(|provider| (provider.id(), provider.name(), provider.authenticate(cx)))
            .collect::<Vec<_>>();

        cx.background_spawn(async move {
            for (provider_id, provider_name, authenticate_task) in authenticate_all_providers {
                if let Err(err) = authenticate_task.await {
                    if matches!(err, language_model::AuthenticateError::CredentialsNotFound) {
                        // Since we're authenticating these providers in the
                        // background for the purposes of populating the
                        // language selector, we don't care about providers
                        // where the credentials are not found.
                    } else {
                        // Some providers have noisy failure states that we
                        // don't want to spam the logs with every time the
                        // language model selector is initialized.
                        //
                        // Ideally these should have more clear failure modes
                        // that we know are safe to ignore here, like what we do
                        // with `CredentialsNotFound` above.
                        match provider_id.0.as_ref() {
                            "lmstudio" | "ollama" => {
                                // LM Studio and Ollama both make fetch requests to the local APIs to determine if they are "authenticated".
                                //
                                // These fail noisily, so we don't log them.
                            }
                            "copilot_chat" => {
                                // Copilot Chat returns an error if Copilot is not enabled, so we don't log those errors.
                            }
                            _ => {
                                log::error!(
                                    "Failed to authenticate provider: {}: {err}",
                                    provider_name.0
                                );
                            }
                        }
                    }
                }
            }
        })
    }
}

pub struct NativeAgent {
    /// Session ID -> Session mapping
    sessions: HashMap<acp::SessionId, Session>,
    history: Entity<HistoryStore>,
    /// Shared project context for all threads
    project_context: Entity<ProjectContext>,
    project_context_needs_refresh: watch::Sender<()>,
    _maintain_project_context: Task<Result<()>>,
    context_server_registry: Entity<ContextServerRegistry>,
    /// Shared templates for all threads
    templates: Arc<Templates>,
    /// Cached model information
    models: LanguageModels,
    project: Entity<Project>,
    prompt_store: Option<Entity<PromptStore>>,
    fs: Arc<dyn Fs>,
    _subscriptions: Vec<Subscription>,
}

impl NativeAgent {
    pub async fn new(
        project: Entity<Project>,
        history: Entity<HistoryStore>,
        templates: Arc<Templates>,
        prompt_store: Option<Entity<PromptStore>>,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<NativeAgent>> {
        log::debug!("Creating new NativeAgent");

        let project_context = cx
            .update(|cx| Self::build_project_context(&project, prompt_store.as_ref(), cx))?
            .await;

        cx.new(|cx| {
            let mut subscriptions = vec![
                cx.subscribe(&project, Self::handle_project_event),
                cx.subscribe(
                    &LanguageModelRegistry::global(cx),
                    Self::handle_models_updated_event,
                ),
            ];
            if let Some(prompt_store) = prompt_store.as_ref() {
                subscriptions.push(cx.subscribe(prompt_store, Self::handle_prompts_updated_event))
            }

            let (project_context_needs_refresh_tx, project_context_needs_refresh_rx) =
                watch::channel(());
            Self {
                sessions: HashMap::new(),
                history,
                project_context: cx.new(|_| project_context),
                project_context_needs_refresh: project_context_needs_refresh_tx,
                _maintain_project_context: cx.spawn(async move |this, cx| {
                    Self::maintain_project_context(this, project_context_needs_refresh_rx, cx).await
                }),
                context_server_registry: cx.new(|cx| {
                    ContextServerRegistry::new(project.read(cx).context_server_store(), cx)
                }),
                templates,
                models: LanguageModels::new(cx),
                project,
                prompt_store,
                fs,
                _subscriptions: subscriptions,
            }
        })
    }

    fn register_session(
        &mut self,
        thread_handle: Entity<Thread>,
        cx: &mut Context<Self>,
    ) -> Entity<AcpThread> {
        let connection = Rc::new(NativeAgentConnection(cx.entity()));

        let thread = thread_handle.read(cx);
        let session_id = thread.id().clone();
        let title = thread.title();
        let project = thread.project.clone();
        let action_log = thread.action_log.clone();
        let prompt_capabilities_rx = thread.prompt_capabilities_rx.clone();
        let acp_thread = cx.new(|cx| {
            acp_thread::AcpThread::new(
                title,
                connection,
                project.clone(),
                action_log.clone(),
                session_id.clone(),
                prompt_capabilities_rx,
                vec![],
                cx,
            )
        });

        let registry = LanguageModelRegistry::read_global(cx);
        let summarization_model = registry.thread_summary_model().map(|c| c.model);

        thread_handle.update(cx, |thread, cx| {
            thread.set_summarization_model(summarization_model, cx);
            thread.add_default_tools(
                Rc::new(AcpThreadEnvironment {
                    acp_thread: acp_thread.downgrade(),
                }) as _,
                cx,
            )
        });

        let subscriptions = vec![
            cx.observe_release(&acp_thread, |this, acp_thread, _cx| {
                this.sessions.remove(acp_thread.session_id());
            }),
            cx.subscribe(&thread_handle, Self::handle_thread_title_updated),
            cx.subscribe(&thread_handle, Self::handle_thread_token_usage_updated),
            cx.observe(&thread_handle, move |this, thread, cx| {
                this.save_thread(thread, cx)
            }),
        ];

        self.sessions.insert(
            session_id,
            Session {
                thread: thread_handle,
                acp_thread: acp_thread.downgrade(),
                _subscriptions: subscriptions,
                pending_save: Task::ready(()),
            },
        );
        acp_thread
    }

    pub fn models(&self) -> &LanguageModels {
        &self.models
    }

    async fn maintain_project_context(
        this: WeakEntity<Self>,
        mut needs_refresh: watch::Receiver<()>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        while needs_refresh.changed().await.is_ok() {
            let project_context = this
                .update(cx, |this, cx| {
                    Self::build_project_context(&this.project, this.prompt_store.as_ref(), cx)
                })?
                .await;
            this.update(cx, |this, cx| {
                this.project_context = cx.new(|_| project_context);
            })?;
        }

        Ok(())
    }

    fn build_project_context(
        project: &Entity<Project>,
        prompt_store: Option<&Entity<PromptStore>>,
        cx: &mut App,
    ) -> Task<ProjectContext> {
        let worktrees = project.read(cx).visible_worktrees(cx).collect::<Vec<_>>();
        let worktree_tasks = worktrees
            .into_iter()
            .map(|worktree| {
                Self::load_worktree_info_for_system_prompt(worktree, project.clone(), cx)
            })
            .collect::<Vec<_>>();
        let default_user_rules_task = if let Some(prompt_store) = prompt_store.as_ref() {
            prompt_store.read_with(cx, |prompt_store, cx| {
                let prompts = prompt_store.default_prompt_metadata();
                let load_tasks = prompts.into_iter().map(|prompt_metadata| {
                    let contents = prompt_store.load(prompt_metadata.id, cx);
                    async move { (contents.await, prompt_metadata) }
                });
                cx.background_spawn(future::join_all(load_tasks))
            })
        } else {
            Task::ready(vec![])
        };

        cx.spawn(async move |_cx| {
            let (worktrees, default_user_rules) =
                future::join(future::join_all(worktree_tasks), default_user_rules_task).await;

            let worktrees = worktrees
                .into_iter()
                .map(|(worktree, _rules_error)| {
                    // TODO: show error message
                    // if let Some(rules_error) = rules_error {
                    //     this.update(cx, |_, cx| cx.emit(rules_error)).ok();
                    // }
                    worktree
                })
                .collect::<Vec<_>>();

            let default_user_rules = default_user_rules
                .into_iter()
                .flat_map(|(contents, prompt_metadata)| match contents {
                    Ok(contents) => Some(UserRulesContext {
                        uuid: match prompt_metadata.id {
                            PromptId::User { uuid } => uuid,
                            PromptId::EditWorkflow => return None,
                        },
                        title: prompt_metadata.title.map(|title| title.to_string()),
                        contents,
                    }),
                    Err(_err) => {
                        // TODO: show error message
                        // this.update(cx, |_, cx| {
                        //     cx.emit(RulesLoadingError {
                        //         message: format!("{err:?}").into(),
                        //     });
                        // })
                        // .ok();
                        None
                    }
                })
                .collect::<Vec<_>>();

            ProjectContext::new(worktrees, default_user_rules)
        })
    }

    fn load_worktree_info_for_system_prompt(
        worktree: Entity<Worktree>,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<(WorktreeContext, Option<RulesLoadingError>)> {
        let tree = worktree.read(cx);
        let root_name = tree.root_name().into();
        let abs_path = tree.abs_path();

        let mut context = WorktreeContext {
            root_name,
            abs_path,
            rules_file: None,
        };

        let rules_task = Self::load_worktree_rules_file(worktree, project, cx);
        let Some(rules_task) = rules_task else {
            return Task::ready((context, None));
        };

        cx.spawn(async move |_| {
            let (rules_file, rules_file_error) = match rules_task.await {
                Ok(rules_file) => (Some(rules_file), None),
                Err(err) => (
                    None,
                    Some(RulesLoadingError {
                        message: format!("{err}").into(),
                    }),
                ),
            };
            context.rules_file = rules_file;
            (context, rules_file_error)
        })
    }

    fn load_worktree_rules_file(
        worktree: Entity<Worktree>,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Option<Task<Result<RulesFileContext>>> {
        let worktree = worktree.read(cx);
        let worktree_id = worktree.id();
        let selected_rules_file = RULES_FILE_NAMES
            .into_iter()
            .filter_map(|name| {
                worktree
                    .entry_for_path(name)
                    .filter(|entry| entry.is_file())
                    .map(|entry| entry.path.clone())
            })
            .next();

        // Note that Cline supports `.clinerules` being a directory, but that is not currently
        // supported. This doesn't seem to occur often in GitHub repositories.
        selected_rules_file.map(|path_in_worktree| {
            let project_path = ProjectPath {
                worktree_id,
                path: path_in_worktree.clone(),
            };
            let buffer_task =
                project.update(cx, |project, cx| project.open_buffer(project_path, cx));
            let rope_task = cx.spawn(async move |cx| {
                buffer_task.await?.read_with(cx, |buffer, cx| {
                    let project_entry_id = buffer.entry_id(cx).context("buffer has no file")?;
                    anyhow::Ok((project_entry_id, buffer.as_rope().clone()))
                })?
            });
            // Build a string from the rope on a background thread.
            cx.background_spawn(async move {
                let (project_entry_id, rope) = rope_task.await?;
                anyhow::Ok(RulesFileContext {
                    path_in_worktree,
                    text: rope.to_string().trim().to_string(),
                    project_entry_id: project_entry_id.to_usize(),
                })
            })
        })
    }

    fn handle_thread_title_updated(
        &mut self,
        thread: Entity<Thread>,
        _: &TitleUpdated,
        cx: &mut Context<Self>,
    ) {
        let session_id = thread.read(cx).id();
        let Some(session) = self.sessions.get(session_id) else {
            return;
        };
        let thread = thread.downgrade();
        let acp_thread = session.acp_thread.clone();
        cx.spawn(async move |_, cx| {
            let title = thread.read_with(cx, |thread, _| thread.title())?;
            let task = acp_thread.update(cx, |acp_thread, cx| acp_thread.set_title(title, cx))?;
            task.await
        })
        .detach_and_log_err(cx);
    }

    fn handle_thread_token_usage_updated(
        &mut self,
        thread: Entity<Thread>,
        usage: &TokenUsageUpdated,
        cx: &mut Context<Self>,
    ) {
        let Some(session) = self.sessions.get(thread.read(cx).id()) else {
            return;
        };
        session
            .acp_thread
            .update(cx, |acp_thread, cx| {
                acp_thread.update_token_usage(usage.0.clone(), cx);
            })
            .ok();
    }

    fn handle_project_event(
        &mut self,
        _project: Entity<Project>,
        event: &project::Event,
        _cx: &mut Context<Self>,
    ) {
        match event {
            project::Event::WorktreeAdded(_) | project::Event::WorktreeRemoved(_) => {
                self.project_context_needs_refresh.send(()).ok();
            }
            project::Event::WorktreeUpdatedEntries(_, items) => {
                if items.iter().any(|(path, _, _)| {
                    RULES_FILE_NAMES
                        .iter()
                        .any(|name| path.as_ref() == Path::new(name))
                }) {
                    self.project_context_needs_refresh.send(()).ok();
                }
            }
            _ => {}
        }
    }

    fn handle_prompts_updated_event(
        &mut self,
        _prompt_store: Entity<PromptStore>,
        _event: &prompt_store::PromptsUpdatedEvent,
        _cx: &mut Context<Self>,
    ) {
        self.project_context_needs_refresh.send(()).ok();
    }

    fn handle_models_updated_event(
        &mut self,
        _registry: Entity<LanguageModelRegistry>,
        _event: &language_model::Event,
        cx: &mut Context<Self>,
    ) {
        self.models.refresh_list(cx);

        let registry = LanguageModelRegistry::read_global(cx);
        let default_model = registry.default_model().map(|m| m.model);
        let summarization_model = registry.thread_summary_model().map(|m| m.model);

        for session in self.sessions.values_mut() {
            session.thread.update(cx, |thread, cx| {
                if thread.model().is_none()
                    && let Some(model) = default_model.clone()
                {
                    thread.set_model(model, cx);
                    cx.notify();
                }
                thread.set_summarization_model(summarization_model.clone(), cx);
            });
        }
    }

    pub fn open_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<AcpThread>>> {
        let database_future = ThreadsDatabase::connect(cx);
        cx.spawn(async move |this, cx| {
            let database = database_future.await.map_err(|err| anyhow!(err))?;
            let db_thread = database
                .load_thread(id.clone())
                .await?
                .with_context(|| format!("no thread found with ID: {id:?}"))?;

            let thread = this.update(cx, |this, cx| {
                let action_log = cx.new(|_cx| ActionLog::new(this.project.clone()));
                cx.new(|cx| {
                    Thread::from_db(
                        id.clone(),
                        db_thread,
                        this.project.clone(),
                        this.project_context.clone(),
                        this.context_server_registry.clone(),
                        action_log.clone(),
                        this.templates.clone(),
                        cx,
                    )
                })
            })?;
            let acp_thread =
                this.update(cx, |this, cx| this.register_session(thread.clone(), cx))?;
            let events = thread.update(cx, |thread, cx| thread.replay(cx))?;
            cx.update(|cx| {
                NativeAgentConnection::handle_thread_events(events, acp_thread.downgrade(), cx)
            })?
            .await?;
            Ok(acp_thread)
        })
    }

    pub fn thread_summary(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<SharedString>> {
        let thread = self.open_thread(id.clone(), cx);
        cx.spawn(async move |this, cx| {
            let acp_thread = thread.await?;
            let result = this
                .update(cx, |this, cx| {
                    this.sessions
                        .get(&id)
                        .unwrap()
                        .thread
                        .update(cx, |thread, cx| thread.summary(cx))
                })?
                .await?;
            drop(acp_thread);
            Ok(result)
        })
    }

    fn save_thread(&mut self, thread: Entity<Thread>, cx: &mut Context<Self>) {
        if thread.read(cx).is_empty() {
            return;
        }

        let database_future = ThreadsDatabase::connect(cx);
        let (id, db_thread) =
            thread.update(cx, |thread, cx| (thread.id().clone(), thread.to_db(cx)));
        let Some(session) = self.sessions.get_mut(&id) else {
            return;
        };
        let history = self.history.clone();
        session.pending_save = cx.spawn(async move |_, cx| {
            let Some(database) = database_future.await.map_err(|err| anyhow!(err)).log_err() else {
                return;
            };
            let db_thread = db_thread.await;
            database.save_thread(id, db_thread).await.log_err();
            history.update(cx, |history, cx| history.reload(cx)).ok();
        });
    }
}

/// Wrapper struct that implements the AgentConnection trait
#[derive(Clone)]
pub struct NativeAgentConnection(pub Entity<NativeAgent>);

impl NativeAgentConnection {
    pub fn thread(&self, session_id: &acp::SessionId, cx: &App) -> Option<Entity<Thread>> {
        self.0
            .read(cx)
            .sessions
            .get(session_id)
            .map(|session| session.thread.clone())
    }

    fn run_turn(
        &self,
        session_id: acp::SessionId,
        cx: &mut App,
        f: impl 'static
        + FnOnce(Entity<Thread>, &mut App) -> Result<mpsc::UnboundedReceiver<Result<ThreadEvent>>>,
    ) -> Task<Result<acp::PromptResponse>> {
        let Some((thread, acp_thread)) = self.0.update(cx, |agent, _cx| {
            agent
                .sessions
                .get_mut(&session_id)
                .map(|s| (s.thread.clone(), s.acp_thread.clone()))
        }) else {
            return Task::ready(Err(anyhow!("Session not found")));
        };
        log::debug!("Found session for: {}", session_id);

        let response_stream = match f(thread, cx) {
            Ok(stream) => stream,
            Err(err) => return Task::ready(Err(err)),
        };
        Self::handle_thread_events(response_stream, acp_thread, cx)
    }

    fn handle_thread_events(
        mut events: mpsc::UnboundedReceiver<Result<ThreadEvent>>,
        acp_thread: WeakEntity<AcpThread>,
        cx: &App,
    ) -> Task<Result<acp::PromptResponse>> {
        cx.spawn(async move |cx| {
            // Handle response stream and forward to session.acp_thread
            while let Some(result) = events.next().await {
                match result {
                    Ok(event) => {
                        log::trace!("Received completion event: {:?}", event);

                        match event {
                            ThreadEvent::UserMessage(message) => {
                                acp_thread.update(cx, |thread, cx| {
                                    for content in message.content {
                                        thread.push_user_content_block(
                                            Some(message.id.clone()),
                                            content.into(),
                                            cx,
                                        );
                                    }
                                })?;
                            }
                            ThreadEvent::AgentText(text) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.push_assistant_content_block(
                                        acp::ContentBlock::Text(acp::TextContent {
                                            text,
                                            annotations: None,
                                        }),
                                        false,
                                        cx,
                                    )
                                })?;
                            }
                            ThreadEvent::AgentThinking(text) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.push_assistant_content_block(
                                        acp::ContentBlock::Text(acp::TextContent {
                                            text,
                                            annotations: None,
                                        }),
                                        true,
                                        cx,
                                    )
                                })?;
                            }
                            ThreadEvent::ToolCallAuthorization(ToolCallAuthorization {
                                tool_call,
                                options,
                                response,
                            }) => {
                                let outcome_task = acp_thread.update(cx, |thread, cx| {
                                    thread.request_tool_call_authorization(tool_call, options, cx)
                                })??;
                                cx.background_spawn(async move {
                                    if let acp::RequestPermissionOutcome::Selected { option_id } =
                                        outcome_task.await
                                    {
                                        response
                                            .send(option_id)
                                            .map(|_| anyhow!("authorization receiver was dropped"))
                                            .log_err();
                                    }
                                })
                                .detach();
                            }
                            ThreadEvent::ToolCall(tool_call) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.upsert_tool_call(tool_call, cx)
                                })??;
                            }
                            ThreadEvent::ToolCallUpdate(update) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.update_tool_call(update, cx)
                                })??;
                            }
                            ThreadEvent::Retry(status) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.update_retry_status(status, cx)
                                })?;
                            }
                            ThreadEvent::Stop(stop_reason) => {
                                log::debug!("Assistant message complete: {:?}", stop_reason);
                                return Ok(acp::PromptResponse { stop_reason });
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error in model response stream: {:?}", e);
                        return Err(e);
                    }
                }
            }

            log::debug!("Response stream completed");
            anyhow::Ok(acp::PromptResponse {
                stop_reason: acp::StopReason::EndTurn,
            })
        })
    }
}

impl AgentModelSelector for NativeAgentConnection {
    fn list_models(&self, cx: &mut App) -> Task<Result<acp_thread::AgentModelList>> {
        log::debug!("NativeAgentConnection::list_models called");
        let list = self.0.read(cx).models.model_list.clone();
        Task::ready(if list.is_empty() {
            Err(anyhow::anyhow!("No models available"))
        } else {
            Ok(list)
        })
    }

    fn select_model(
        &self,
        session_id: acp::SessionId,
        model_id: acp_thread::AgentModelId,
        cx: &mut App,
    ) -> Task<Result<()>> {
        log::debug!("Setting model for session {}: {}", session_id, model_id);
        let Some(thread) = self
            .0
            .read(cx)
            .sessions
            .get(&session_id)
            .map(|session| session.thread.clone())
        else {
            return Task::ready(Err(anyhow!("Session not found")));
        };

        let Some(model) = self.0.read(cx).models.model_from_id(&model_id) else {
            return Task::ready(Err(anyhow!("Invalid model ID {}", model_id)));
        };

        thread.update(cx, |thread, cx| {
            thread.set_model(model.clone(), cx);
        });

        update_settings_file::<AgentSettings>(
            self.0.read(cx).fs.clone(),
            cx,
            move |settings, _cx| {
                settings.set_model(model);
            },
        );

        Task::ready(Ok(()))
    }

    fn selected_model(
        &self,
        session_id: &acp::SessionId,
        cx: &mut App,
    ) -> Task<Result<acp_thread::AgentModelInfo>> {
        let session_id = session_id.clone();

        let Some(thread) = self
            .0
            .read(cx)
            .sessions
            .get(&session_id)
            .map(|session| session.thread.clone())
        else {
            return Task::ready(Err(anyhow!("Session not found")));
        };
        let Some(model) = thread.read(cx).model() else {
            return Task::ready(Err(anyhow!("Model not found")));
        };
        let Some(provider) = LanguageModelRegistry::read_global(cx).provider(&model.provider_id())
        else {
            return Task::ready(Err(anyhow!("Provider not found")));
        };
        Task::ready(Ok(LanguageModels::map_language_model_to_info(
            model, &provider,
        )))
    }

    fn watch(&self, cx: &mut App) -> watch::Receiver<()> {
        self.0.read(cx).models.watch()
    }
}

impl acp_thread::AgentConnection for NativeAgentConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut App,
    ) -> Task<Result<Entity<acp_thread::AcpThread>>> {
        let agent = self.0.clone();
        log::debug!("Creating new thread for project at: {:?}", cwd);

        cx.spawn(async move |cx| {
            log::debug!("Starting thread creation in async context");

            // Create Thread
            let thread = agent.update(
                cx,
                |agent, cx: &mut gpui::Context<NativeAgent>| -> Result<_> {
                    // Fetch default model from registry settings
                    let registry = LanguageModelRegistry::read_global(cx);
                    // Log available models for debugging
                    let available_count = registry.available_models(cx).count();
                    log::debug!("Total available models: {}", available_count);

                    let default_model = registry.default_model().and_then(|default_model| {
                        agent
                            .models
                            .model_from_id(&LanguageModels::model_id(&default_model.model))
                    });
                    Ok(cx.new(|cx| {
                        Thread::new(
                            project.clone(),
                            agent.project_context.clone(),
                            agent.context_server_registry.clone(),
                            agent.templates.clone(),
                            default_model,
                            cx,
                        )
                    }))
                },
            )??;
            agent.update(cx, |agent, cx| agent.register_session(thread, cx))
        })
    }

    fn auth_methods(&self) -> &[acp::AuthMethod] {
        &[] // No auth for in-process
    }

    fn authenticate(&self, _method: acp::AuthMethodId, _cx: &mut App) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn model_selector(&self) -> Option<Rc<dyn AgentModelSelector>> {
        Some(Rc::new(self.clone()) as Rc<dyn AgentModelSelector>)
    }

    fn prompt(
        &self,
        id: Option<acp_thread::UserMessageId>,
        params: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
        let id = id.expect("UserMessageId is required");
        let session_id = params.session_id.clone();
        log::info!("Received prompt request for session: {}", session_id);
        log::debug!("Prompt blocks count: {}", params.prompt.len());

        self.run_turn(session_id, cx, |thread, cx| {
            let content: Vec<UserMessageContent> = params
                .prompt
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>();
            log::debug!("Converted prompt to message: {} chars", content.len());
            log::debug!("Message id: {:?}", id);
            log::debug!("Message content: {:?}", content);

            thread.update(cx, |thread, cx| thread.send(id, content, cx))
        })
    }

    fn resume(
        &self,
        session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn acp_thread::AgentSessionResume>> {
        Some(Rc::new(NativeAgentSessionResume {
            connection: self.clone(),
            session_id: session_id.clone(),
        }) as _)
    }

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
        log::info!("Cancelling on session: {}", session_id);
        self.0.update(cx, |agent, cx| {
            if let Some(agent) = agent.sessions.get(session_id) {
                agent.thread.update(cx, |thread, cx| thread.cancel(cx));
            }
        });
    }

    fn truncate(
        &self,
        session_id: &agent_client_protocol::SessionId,
        cx: &App,
    ) -> Option<Rc<dyn acp_thread::AgentSessionTruncate>> {
        self.0.read_with(cx, |agent, _cx| {
            agent.sessions.get(session_id).map(|session| {
                Rc::new(NativeAgentSessionTruncate {
                    thread: session.thread.clone(),
                    acp_thread: session.acp_thread.clone(),
                }) as _
            })
        })
    }

    fn set_title(
        &self,
        session_id: &acp::SessionId,
        _cx: &App,
    ) -> Option<Rc<dyn acp_thread::AgentSessionSetTitle>> {
        Some(Rc::new(NativeAgentSessionSetTitle {
            connection: self.clone(),
            session_id: session_id.clone(),
        }) as _)
    }

    fn telemetry(&self) -> Option<Rc<dyn acp_thread::AgentTelemetry>> {
        Some(Rc::new(self.clone()) as Rc<dyn acp_thread::AgentTelemetry>)
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

impl acp_thread::AgentTelemetry for NativeAgentConnection {
    fn agent_name(&self) -> String {
        "Zed".into()
    }

    fn thread_data(
        &self,
        session_id: &acp::SessionId,
        cx: &mut App,
    ) -> Task<Result<serde_json::Value>> {
        let Some(session) = self.0.read(cx).sessions.get(session_id) else {
            return Task::ready(Err(anyhow!("Session not found")));
        };

        let task = session.thread.read(cx).to_db(cx);
        cx.background_spawn(async move {
            serde_json::to_value(task.await).context("Failed to serialize thread")
        })
    }
}

struct NativeAgentSessionTruncate {
    thread: Entity<Thread>,
    acp_thread: WeakEntity<AcpThread>,
}

impl acp_thread::AgentSessionTruncate for NativeAgentSessionTruncate {
    fn run(&self, message_id: acp_thread::UserMessageId, cx: &mut App) -> Task<Result<()>> {
        match self.thread.update(cx, |thread, cx| {
            thread.truncate(message_id.clone(), cx)?;
            Ok(thread.latest_token_usage())
        }) {
            Ok(usage) => {
                self.acp_thread
                    .update(cx, |thread, cx| {
                        thread.update_token_usage(usage, cx);
                    })
                    .ok();
                Task::ready(Ok(()))
            }
            Err(error) => Task::ready(Err(error)),
        }
    }
}

struct NativeAgentSessionResume {
    connection: NativeAgentConnection,
    session_id: acp::SessionId,
}

impl acp_thread::AgentSessionResume for NativeAgentSessionResume {
    fn run(&self, cx: &mut App) -> Task<Result<acp::PromptResponse>> {
        self.connection
            .run_turn(self.session_id.clone(), cx, |thread, cx| {
                thread.update(cx, |thread, cx| thread.resume(cx))
            })
    }
}

struct NativeAgentSessionSetTitle {
    connection: NativeAgentConnection,
    session_id: acp::SessionId,
}

impl acp_thread::AgentSessionSetTitle for NativeAgentSessionSetTitle {
    fn run(&self, title: SharedString, cx: &mut App) -> Task<Result<()>> {
        let Some(session) = self.connection.0.read(cx).sessions.get(&self.session_id) else {
            return Task::ready(Err(anyhow!("session not found")));
        };
        let thread = session.thread.clone();
        thread.update(cx, |thread, cx| thread.set_title(title, cx));
        Task::ready(Ok(()))
    }
}

pub struct AcpThreadEnvironment {
    acp_thread: WeakEntity<AcpThread>,
}

impl ThreadEnvironment for AcpThreadEnvironment {
    fn create_terminal(
        &self,
        command: String,
        cwd: Option<PathBuf>,
        output_byte_limit: Option<u64>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Rc<dyn TerminalHandle>>> {
        let task = self.acp_thread.update(cx, |thread, cx| {
            thread.create_terminal(command, vec![], vec![], cwd, output_byte_limit, cx)
        });

        let acp_thread = self.acp_thread.clone();
        cx.spawn(async move |cx| {
            let terminal = task?.await?;

            let (drop_tx, drop_rx) = oneshot::channel();
            let terminal_id = terminal.read_with(cx, |terminal, _cx| terminal.id().clone())?;

            cx.spawn(async move |cx| {
                drop_rx.await.ok();
                acp_thread.update(cx, |thread, cx| thread.release_terminal(terminal_id, cx))
            })
            .detach();

            let handle = AcpTerminalHandle {
                terminal,
                _drop_tx: Some(drop_tx),
            };

            Ok(Rc::new(handle) as _)
        })
    }
}

pub struct AcpTerminalHandle {
    terminal: Entity<acp_thread::Terminal>,
    _drop_tx: Option<oneshot::Sender<()>>,
}

impl TerminalHandle for AcpTerminalHandle {
    fn id(&self, cx: &AsyncApp) -> Result<acp::TerminalId> {
        self.terminal.read_with(cx, |term, _cx| term.id().clone())
    }

    fn wait_for_exit(&self, cx: &AsyncApp) -> Result<Shared<Task<acp::TerminalExitStatus>>> {
        self.terminal
            .read_with(cx, |term, _cx| term.wait_for_exit())
    }

    fn current_output(&self, cx: &AsyncApp) -> Result<acp::TerminalOutputResponse> {
        self.terminal
            .read_with(cx, |term, cx| term.current_output(cx))
    }
}

#[cfg(test)]
mod tests {
    use crate::HistoryEntryId;

    use super::*;
    use acp_thread::{
        AgentConnection, AgentModelGroupName, AgentModelId, AgentModelInfo, MentionUri,
    };
    use fs::FakeFs;
    use gpui::TestAppContext;
    use indoc::indoc;
    use language_model::fake_provider::FakeLanguageModel;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_maintaining_project_context(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/",
            json!({
                "a": {}
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [], cx).await;
        let context_store = cx.new(|cx| assistant_context::ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));
        let agent = NativeAgent::new(
            project.clone(),
            history_store,
            Templates::new(),
            None,
            fs.clone(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        agent.read_with(cx, |agent, cx| {
            assert_eq!(agent.project_context.read(cx).worktrees, vec![])
        });

        let worktree = project
            .update(cx, |project, cx| project.create_worktree("/a", true, cx))
            .await
            .unwrap();
        cx.run_until_parked();
        agent.read_with(cx, |agent, cx| {
            assert_eq!(
                agent.project_context.read(cx).worktrees,
                vec![WorktreeContext {
                    root_name: "a".into(),
                    abs_path: Path::new("/a").into(),
                    rules_file: None
                }]
            )
        });

        // Creating `/a/.rules` updates the project context.
        fs.insert_file("/a/.rules", Vec::new()).await;
        cx.run_until_parked();
        agent.read_with(cx, |agent, cx| {
            let rules_entry = worktree.read(cx).entry_for_path(".rules").unwrap();
            assert_eq!(
                agent.project_context.read(cx).worktrees,
                vec![WorktreeContext {
                    root_name: "a".into(),
                    abs_path: Path::new("/a").into(),
                    rules_file: Some(RulesFileContext {
                        path_in_worktree: Path::new(".rules").into(),
                        text: "".into(),
                        project_entry_id: rules_entry.id.to_usize()
                    })
                }]
            )
        });
    }

    #[gpui::test]
    async fn test_listing_models(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/", json!({ "a": {}  })).await;
        let project = Project::test(fs.clone(), [], cx).await;
        let context_store = cx.new(|cx| assistant_context::ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));
        let connection = NativeAgentConnection(
            NativeAgent::new(
                project.clone(),
                history_store,
                Templates::new(),
                None,
                fs.clone(),
                &mut cx.to_async(),
            )
            .await
            .unwrap(),
        );

        let models = cx.update(|cx| connection.list_models(cx)).await.unwrap();

        let acp_thread::AgentModelList::Grouped(models) = models else {
            panic!("Unexpected model group");
        };
        assert_eq!(
            models,
            IndexMap::from_iter([(
                AgentModelGroupName("Fake".into()),
                vec![AgentModelInfo {
                    id: AgentModelId("fake/fake".into()),
                    name: "Fake".into(),
                    icon: Some(ui::IconName::ZedAssistant),
                }]
            )])
        );
    }

    #[gpui::test]
    async fn test_model_selection_persists_to_settings(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(paths::settings_file().parent().unwrap())
            .await
            .unwrap();
        fs.insert_file(
            paths::settings_file(),
            json!({
                "agent": {
                    "default_model": {
                        "provider": "foo",
                        "model": "bar"
                    }
                }
            })
            .to_string()
            .into_bytes(),
        )
        .await;
        let project = Project::test(fs.clone(), [], cx).await;

        let context_store = cx.new(|cx| assistant_context::ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));

        // Create the agent and connection
        let agent = NativeAgent::new(
            project.clone(),
            history_store,
            Templates::new(),
            None,
            fs.clone(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        let connection = NativeAgentConnection(agent.clone());

        // Create a thread/session
        let acp_thread = cx
            .update(|cx| {
                Rc::new(connection.clone()).new_thread(project.clone(), Path::new("/a"), cx)
            })
            .await
            .unwrap();

        let session_id = cx.update(|cx| acp_thread.read(cx).session_id().clone());

        // Select a model
        let model_id = AgentModelId("fake/fake".into());
        cx.update(|cx| connection.select_model(session_id.clone(), model_id.clone(), cx))
            .await
            .unwrap();

        // Verify the thread has the selected model
        agent.read_with(cx, |agent, _| {
            let session = agent.sessions.get(&session_id).unwrap();
            session.thread.read_with(cx, |thread, _| {
                assert_eq!(thread.model().unwrap().id().0, "fake");
            });
        });

        cx.run_until_parked();

        // Verify settings file was updated
        let settings_content = fs.load(paths::settings_file()).await.unwrap();
        let settings_json: serde_json::Value = serde_json::from_str(&settings_content).unwrap();

        // Check that the agent settings contain the selected model
        assert_eq!(
            settings_json["agent"]["default_model"]["model"],
            json!("fake")
        );
        assert_eq!(
            settings_json["agent"]["default_model"]["provider"],
            json!("fake")
        );
    }

    #[gpui::test]
    #[cfg_attr(target_os = "windows", ignore)] // TODO: Fix this test on Windows
    async fn test_save_load_thread(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/",
            json!({
                "a": {
                    "b.md": "Lorem"
                }
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/a").as_ref()], cx).await;
        let context_store = cx.new(|cx| assistant_context::ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));
        let agent = NativeAgent::new(
            project.clone(),
            history_store.clone(),
            Templates::new(),
            None,
            fs.clone(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        let connection = Rc::new(NativeAgentConnection(agent.clone()));

        let acp_thread = cx
            .update(|cx| {
                connection
                    .clone()
                    .new_thread(project.clone(), Path::new(""), cx)
            })
            .await
            .unwrap();
        let session_id = acp_thread.read_with(cx, |thread, _| thread.session_id().clone());
        let thread = agent.read_with(cx, |agent, _| {
            agent.sessions.get(&session_id).unwrap().thread.clone()
        });

        // Ensure empty threads are not saved, even if they get mutated.
        let model = Arc::new(FakeLanguageModel::default());
        let summary_model = Arc::new(FakeLanguageModel::default());
        thread.update(cx, |thread, cx| {
            thread.set_model(model.clone(), cx);
            thread.set_summarization_model(Some(summary_model.clone()), cx);
        });
        cx.run_until_parked();
        assert_eq!(history_entries(&history_store, cx), vec![]);

        let send = acp_thread.update(cx, |thread, cx| {
            thread.send(
                vec![
                    "What does ".into(),
                    acp::ContentBlock::ResourceLink(acp::ResourceLink {
                        name: "b.md".into(),
                        uri: MentionUri::File {
                            abs_path: path!("/a/b.md").into(),
                        }
                        .to_uri()
                        .to_string(),
                        annotations: None,
                        description: None,
                        mime_type: None,
                        size: None,
                        title: None,
                    }),
                    " mean?".into(),
                ],
                cx,
            )
        });
        let send = cx.foreground_executor().spawn(send);
        cx.run_until_parked();

        model.send_last_completion_stream_text_chunk("Lorem.");
        model.end_last_completion_stream();
        cx.run_until_parked();
        summary_model.send_last_completion_stream_text_chunk("Explaining /a/b.md");
        summary_model.end_last_completion_stream();

        send.await.unwrap();
        acp_thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User

                    What does [@b.md](file:///a/b.md) mean?

                    ## Assistant

                    Lorem.

                "}
            )
        });

        cx.run_until_parked();

        // Drop the ACP thread, which should cause the session to be dropped as well.
        cx.update(|_| {
            drop(thread);
            drop(acp_thread);
        });
        agent.read_with(cx, |agent, _| {
            assert_eq!(agent.sessions.keys().cloned().collect::<Vec<_>>(), []);
        });

        // Ensure the thread can be reloaded from disk.
        assert_eq!(
            history_entries(&history_store, cx),
            vec![(
                HistoryEntryId::AcpThread(session_id.clone()),
                "Explaining /a/b.md".into()
            )]
        );
        let acp_thread = agent
            .update(cx, |agent, cx| agent.open_thread(session_id.clone(), cx))
            .await
            .unwrap();
        acp_thread.read_with(cx, |thread, cx| {
            assert_eq!(
                thread.to_markdown(cx),
                indoc! {"
                    ## User

                    What does [@b.md](file:///a/b.md) mean?

                    ## Assistant

                    Lorem.

                "}
            )
        });
    }

    fn history_entries(
        history: &Entity<HistoryStore>,
        cx: &mut TestAppContext,
    ) -> Vec<(HistoryEntryId, String)> {
        history.read_with(cx, |history, _| {
            history
                .entries()
                .map(|e| (e.id(), e.title().to_string()))
                .collect::<Vec<_>>()
        })
    }

    fn init_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            agent_settings::init(cx);
            language::init(cx);
            LanguageModelRegistry::test(cx);
        });
    }
}
