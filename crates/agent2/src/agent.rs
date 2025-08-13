use crate::{AgentResponseEvent, Thread, templates::Templates};
use crate::{
    ContextServerRegistry, CopyPathTool, CreateDirectoryTool, DiagnosticsTool, EditFileTool,
    FetchTool, FindPathTool, GrepTool, ListDirectoryTool, MovePathTool, NowTool, OpenTool,
    ReadFileTool, TerminalTool, ThinkingTool, ToolCallAuthorization, UserMessageContent,
    WebSearchTool,
};
use acp_thread::AgentModelSelector;
use agent_client_protocol as acp;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, anyhow};
use collections::{HashSet, IndexMap};
use fs::Fs;
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
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use util::ResultExt;

const RULES_FILE_NAMES: [&'static str; 9] = [
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
    _subscription: Subscription,
}

pub struct LanguageModels {
    /// Access language model by ID
    models: HashMap<acp_thread::AgentModelId, Arc<dyn LanguageModel>>,
    /// Cached list for returning language model information
    model_list: acp_thread::AgentModelList,
    refresh_models_rx: watch::Receiver<()>,
    refresh_models_tx: watch::Sender<()>,
}

impl LanguageModels {
    fn new(cx: &App) -> Self {
        let (refresh_models_tx, refresh_models_rx) = watch::channel(());
        let mut this = Self {
            models: HashMap::default(),
            model_list: acp_thread::AgentModelList::Grouped(IndexMap::default()),
            refresh_models_rx,
            refresh_models_tx,
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
                recommended_models.insert(model.id());
                recommended.push(Self::map_language_model_to_info(&model, &provider));
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
                if !recommended_models.contains(&model.id()) {
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
}

pub struct NativeAgent {
    /// Session ID -> Session mapping
    sessions: HashMap<acp::SessionId, Session>,
    /// Shared project context for all threads
    project_context: Rc<RefCell<ProjectContext>>,
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
        templates: Arc<Templates>,
        prompt_store: Option<Entity<PromptStore>>,
        fs: Arc<dyn Fs>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<NativeAgent>> {
        log::info!("Creating new NativeAgent");

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
                project_context: Rc::new(RefCell::new(project_context)),
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
            this.update(cx, |this, _| this.project_context.replace(project_context))?;
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
        for session in self.sessions.values_mut() {
            session.thread.update(cx, |thread, _| {
                let model_id = LanguageModels::model_id(&thread.selected_model);
                if let Some(model) = self.models.model_from_id(&model_id) {
                    thread.selected_model = model.clone();
                }
            });
        }
    }
}

/// Wrapper struct that implements the AgentConnection trait
#[derive(Clone)]
pub struct NativeAgentConnection(pub Entity<NativeAgent>);

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
        log::info!("Setting model for session {}: {}", session_id, model_id);
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

        thread.update(cx, |thread, _cx| {
            thread.selected_model = model.clone();
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
        let model = thread.read(cx).selected_model.clone();
        let Some(provider) = LanguageModelRegistry::read_global(cx).provider(&model.provider_id())
        else {
            return Task::ready(Err(anyhow!("Provider not found")));
        };
        Task::ready(Ok(LanguageModels::map_language_model_to_info(
            &model, &provider,
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
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<acp_thread::AcpThread>>> {
        let agent = self.0.clone();
        log::info!("Creating new thread for project at: {:?}", cwd);

        cx.spawn(async move |cx| {
            log::debug!("Starting thread creation in async context");

            // Generate session ID
            let session_id = acp::SessionId(uuid::Uuid::new_v4().to_string().into());
            log::info!("Created session with ID: {}", session_id);

            // Create AcpThread
            let acp_thread = cx.update(|cx| {
                cx.new(|cx| {
                    acp_thread::AcpThread::new(
                        "agent2",
                        self.clone(),
                        project.clone(),
                        session_id.clone(),
                        cx,
                    )
                })
            })?;
            let action_log = cx.update(|cx| acp_thread.read(cx).action_log().clone())?;

            // Create Thread
            let thread = agent.update(
                cx,
                |agent, cx: &mut gpui::Context<NativeAgent>| -> Result<_> {
                    // Fetch default model from registry settings
                    let registry = LanguageModelRegistry::read_global(cx);

                    // Log available models for debugging
                    let available_count = registry.available_models(cx).count();
                    log::debug!("Total available models: {}", available_count);

                    let default_model = registry
                        .default_model()
                        .and_then(|default_model| {
                            agent
                                .models
                                .model_from_id(&LanguageModels::model_id(&default_model.model))
                        })
                        .ok_or_else(|| {
                            log::warn!("No default model configured in settings");
                            anyhow!(
                                "No default model. Please configure a default model in settings."
                            )
                        })?;

                    let thread = cx.new(|cx| {
                        let mut thread = Thread::new(
                            project.clone(),
                            agent.project_context.clone(),
                            agent.context_server_registry.clone(),
                            action_log.clone(),
                            agent.templates.clone(),
                            default_model,
                            cx,
                        );
                        thread.add_tool(CreateDirectoryTool::new(project.clone()));
                        thread.add_tool(CopyPathTool::new(project.clone()));
                        thread.add_tool(DiagnosticsTool::new(project.clone()));
                        thread.add_tool(MovePathTool::new(project.clone()));
                        thread.add_tool(ListDirectoryTool::new(project.clone()));
                        thread.add_tool(OpenTool::new(project.clone()));
                        thread.add_tool(ThinkingTool);
                        thread.add_tool(FindPathTool::new(project.clone()));
                        thread.add_tool(FetchTool::new(project.read(cx).client().http_client()));
                        thread.add_tool(GrepTool::new(project.clone()));
                        thread.add_tool(ReadFileTool::new(project.clone(), action_log));
                        thread.add_tool(EditFileTool::new(cx.entity()));
                        thread.add_tool(NowTool);
                        thread.add_tool(TerminalTool::new(project.clone(), cx));
                        // TODO: Needs to be conditional based on zed model or not
                        thread.add_tool(WebSearchTool);
                        thread
                    });

                    Ok(thread)
                },
            )??;

            // Store the session
            agent.update(cx, |agent, cx| {
                agent.sessions.insert(
                    session_id,
                    Session {
                        thread,
                        acp_thread: acp_thread.downgrade(),
                        _subscription: cx.observe_release(&acp_thread, |this, acp_thread, _cx| {
                            this.sessions.remove(acp_thread.session_id());
                        }),
                    },
                );
            })?;

            Ok(acp_thread)
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
        let agent = self.0.clone();
        log::info!("Received prompt request for session: {}", session_id);
        log::debug!("Prompt blocks count: {}", params.prompt.len());

        cx.spawn(async move |cx| {
            // Get session
            let (thread, acp_thread) = agent
                .update(cx, |agent, _| {
                    agent
                        .sessions
                        .get_mut(&session_id)
                        .map(|s| (s.thread.clone(), s.acp_thread.clone()))
                })?
                .ok_or_else(|| {
                    log::error!("Session not found: {}", session_id);
                    anyhow::anyhow!("Session not found")
                })?;
            log::debug!("Found session for: {}", session_id);

            let content: Vec<UserMessageContent> = params
                .prompt
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>();
            log::info!("Converted prompt to message: {} chars", content.len());
            log::debug!("Message id: {:?}", id);
            log::debug!("Message content: {:?}", content);

            // Get model using the ModelSelector capability (always available for agent2)
            // Get the selected model from the thread directly
            let model = thread.read_with(cx, |thread, _| thread.selected_model.clone())?;

            // Send to thread
            log::info!("Sending message to thread with model: {:?}", model.name());
            let mut response_stream =
                thread.update(cx, |thread, cx| thread.send(id, content, cx))?;

            // Handle response stream and forward to session.acp_thread
            while let Some(result) = response_stream.next().await {
                match result {
                    Ok(event) => {
                        log::trace!("Received completion event: {:?}", event);

                        match event {
                            AgentResponseEvent::Text(text) => {
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
                            AgentResponseEvent::Thinking(text) => {
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
                            AgentResponseEvent::ToolCallAuthorization(ToolCallAuthorization {
                                tool_call,
                                options,
                                response,
                            }) => {
                                let recv = acp_thread.update(cx, |thread, cx| {
                                    thread.request_tool_call_authorization(tool_call, options, cx)
                                })?;
                                cx.background_spawn(async move {
                                    if let Some(option) = recv
                                        .await
                                        .context("authorization sender was dropped")
                                        .log_err()
                                    {
                                        response
                                            .send(option)
                                            .map(|_| anyhow!("authorization receiver was dropped"))
                                            .log_err();
                                    }
                                })
                                .detach();
                            }
                            AgentResponseEvent::ToolCall(tool_call) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.upsert_tool_call(tool_call, cx)
                                })?;
                            }
                            AgentResponseEvent::ToolCallUpdate(update) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.update_tool_call(update, cx)
                                })??;
                            }
                            AgentResponseEvent::Stop(stop_reason) => {
                                log::debug!("Assistant message complete: {:?}", stop_reason);
                                return Ok(acp::PromptResponse { stop_reason });
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error in model response stream: {:?}", e);
                        // TODO: Consider sending an error message to the UI
                        break;
                    }
                }
            }

            log::info!("Response stream completed");
            anyhow::Ok(acp::PromptResponse {
                stop_reason: acp::StopReason::EndTurn,
            })
        })
    }

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
        log::info!("Cancelling on session: {}", session_id);
        self.0.update(cx, |agent, cx| {
            if let Some(agent) = agent.sessions.get(session_id) {
                agent.thread.update(cx, |thread, _cx| thread.cancel());
            }
        });
    }

    fn session_editor(
        &self,
        session_id: &agent_client_protocol::SessionId,
        cx: &mut App,
    ) -> Option<Rc<dyn acp_thread::AgentSessionEditor>> {
        self.0.update(cx, |agent, _cx| {
            agent
                .sessions
                .get(session_id)
                .map(|session| Rc::new(NativeAgentSessionEditor(session.thread.clone())) as _)
        })
    }
}

struct NativeAgentSessionEditor(Entity<Thread>);

impl acp_thread::AgentSessionEditor for NativeAgentSessionEditor {
    fn truncate(&self, message_id: acp_thread::UserMessageId, cx: &mut App) -> Task<Result<()>> {
        Task::ready(self.0.update(cx, |thread, _cx| thread.truncate(message_id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::{AgentConnection, AgentModelGroupName, AgentModelId, AgentModelInfo};
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::SettingsStore;

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
        let agent = NativeAgent::new(
            project.clone(),
            Templates::new(),
            None,
            fs.clone(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        agent.read_with(cx, |agent, _| {
            assert_eq!(agent.project_context.borrow().worktrees, vec![])
        });

        let worktree = project
            .update(cx, |project, cx| project.create_worktree("/a", true, cx))
            .await
            .unwrap();
        cx.run_until_parked();
        agent.read_with(cx, |agent, _| {
            assert_eq!(
                agent.project_context.borrow().worktrees,
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
                agent.project_context.borrow().worktrees,
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
        let connection = NativeAgentConnection(
            NativeAgent::new(
                project.clone(),
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

        // Create the agent and connection
        let agent = NativeAgent::new(
            project.clone(),
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
                Rc::new(connection.clone()).new_thread(
                    project.clone(),
                    Path::new("/a"),
                    &mut cx.to_async(),
                )
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
                assert_eq!(thread.selected_model.id().0, "fake");
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
