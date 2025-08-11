use crate::{AgentResponseEvent, Thread, templates::Templates};
use crate::{EditFileTool, FindPathTool, ReadFileTool, ThinkingTool, ToolCallAuthorization};
use acp_thread::ModelSelector;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result, anyhow};
use futures::{StreamExt, future};
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, SharedString, Subscription, Task, WeakEntity,
};
use language_model::{LanguageModel, LanguageModelRegistry};
use project::{Project, ProjectItem, ProjectPath, Worktree};
use prompt_store::{
    ProjectContext, PromptId, PromptStore, RulesFileContext, UserRulesContext, WorktreeContext,
};
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

pub struct NativeAgent {
    /// Session ID -> Session mapping
    sessions: HashMap<acp::SessionId, Session>,
    /// Shared project context for all threads
    project_context: Rc<RefCell<ProjectContext>>,
    project_context_needs_refresh: watch::Sender<()>,
    _maintain_project_context: Task<Result<()>>,
    /// Shared templates for all threads
    templates: Arc<Templates>,
    project: Entity<Project>,
    prompt_store: Option<Entity<PromptStore>>,
    _subscriptions: Vec<Subscription>,
}

impl NativeAgent {
    pub async fn new(
        project: Entity<Project>,
        templates: Arc<Templates>,
        prompt_store: Option<Entity<PromptStore>>,
        cx: &mut AsyncApp,
    ) -> Result<Entity<NativeAgent>> {
        log::info!("Creating new NativeAgent");

        let project_context = cx
            .update(|cx| Self::build_project_context(&project, prompt_store.as_ref(), cx))?
            .await;

        cx.new(|cx| {
            let mut subscriptions = vec![cx.subscribe(&project, Self::handle_project_event)];
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
                templates,
                project,
                prompt_store,
                _subscriptions: subscriptions,
            }
        })
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
}

/// Wrapper struct that implements the AgentConnection trait
#[derive(Clone)]
pub struct NativeAgentConnection(pub Entity<NativeAgent>);

impl ModelSelector for NativeAgentConnection {
    fn list_models(&self, cx: &mut AsyncApp) -> Task<Result<Vec<Arc<dyn LanguageModel>>>> {
        log::debug!("NativeAgentConnection::list_models called");
        cx.spawn(async move |cx| {
            cx.update(|cx| {
                let registry = LanguageModelRegistry::read_global(cx);
                let models = registry.available_models(cx).collect::<Vec<_>>();
                log::info!("Found {} available models", models.len());
                if models.is_empty() {
                    Err(anyhow::anyhow!("No models available"))
                } else {
                    Ok(models)
                }
            })?
        })
    }

    fn select_model(
        &self,
        session_id: acp::SessionId,
        model: Arc<dyn LanguageModel>,
        cx: &mut AsyncApp,
    ) -> Task<Result<()>> {
        log::info!(
            "Setting model for session {}: {:?}",
            session_id,
            model.name()
        );
        let agent = self.0.clone();

        cx.spawn(async move |cx| {
            agent.update(cx, |agent, cx| {
                if let Some(session) = agent.sessions.get(&session_id) {
                    session.thread.update(cx, |thread, _cx| {
                        thread.selected_model = model;
                    });
                    Ok(())
                } else {
                    Err(anyhow!("Session not found"))
                }
            })?
        })
    }

    fn selected_model(
        &self,
        session_id: &acp::SessionId,
        cx: &mut AsyncApp,
    ) -> Task<Result<Arc<dyn LanguageModel>>> {
        let agent = self.0.clone();
        let session_id = session_id.clone();
        cx.spawn(async move |cx| {
            let thread = agent
                .read_with(cx, |agent, _| {
                    agent
                        .sessions
                        .get(&session_id)
                        .map(|session| session.thread.clone())
                })?
                .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
            let selected = thread.read_with(cx, |thread, _| thread.selected_model.clone())?;
            Ok(selected)
        })
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
                    acp_thread::AcpThread::new("agent2", self.clone(), project.clone(), session_id.clone(), cx)
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
                        .map(|configured| {
                            log::info!(
                                "Using configured default model: {:?} from provider: {:?}",
                                configured.model.name(),
                                configured.provider.name()
                            );
                            configured.model
                        })
                        .ok_or_else(|| {
                            log::warn!("No default model configured in settings");
                            anyhow!("No default model configured. Please configure a default model in settings.")
                        })?;

                    let thread = cx.new(|cx| {
                        let mut thread = Thread::new(project.clone(), agent.project_context.clone(), action_log.clone(), agent.templates.clone(), default_model);
                        thread.add_tool(ThinkingTool);
                        thread.add_tool(FindPathTool::new(project.clone()));
                        thread.add_tool(ReadFileTool::new(project.clone(), action_log));
                        thread.add_tool(EditFileTool::new(cx.entity()));
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
                        })
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

    fn model_selector(&self) -> Option<Rc<dyn ModelSelector>> {
        Some(Rc::new(self.clone()) as Rc<dyn ModelSelector>)
    }

    fn prompt(
        &self,
        params: acp::PromptRequest,
        cx: &mut App,
    ) -> Task<Result<acp::PromptResponse>> {
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

            // Convert prompt to message
            let message = convert_prompt_to_message(params.prompt);
            log::info!("Converted prompt to message: {} chars", message.len());
            log::debug!("Message content: {}", message);

            // Get model using the ModelSelector capability (always available for agent2)
            // Get the selected model from the thread directly
            let model = thread.read_with(cx, |thread, _| thread.selected_model.clone())?;

            // Send to thread
            log::info!("Sending message to thread with model: {:?}", model.name());
            let mut response_stream = thread.update(cx, |thread, cx| thread.send(message, cx))?;

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
}

/// Convert ACP content blocks to a message string
fn convert_prompt_to_message(blocks: Vec<acp::ContentBlock>) -> String {
    log::debug!("Converting {} content blocks to message", blocks.len());
    let mut message = String::new();

    for block in blocks {
        match block {
            acp::ContentBlock::Text(text) => {
                log::trace!("Processing text block: {} chars", text.text.len());
                message.push_str(&text.text);
            }
            acp::ContentBlock::ResourceLink(link) => {
                log::trace!("Processing resource link: {}", link.uri);
                message.push_str(&format!(" @{} ", link.uri));
            }
            acp::ContentBlock::Image(_) => {
                log::trace!("Processing image block");
                message.push_str(" [image] ");
            }
            acp::ContentBlock::Audio(_) => {
                log::trace!("Processing audio block");
                message.push_str(" [audio] ");
            }
            acp::ContentBlock::Resource(resource) => {
                log::trace!("Processing resource block: {:?}", resource.resource);
                message.push_str(&format!(" [resource: {:?}] ", resource.resource));
            }
        }
    }

    message
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let agent = NativeAgent::new(project.clone(), Templates::new(), None, &mut cx.to_async())
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

    fn init_test(cx: &mut TestAppContext) {
        env_logger::try_init().ok();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            language::init(cx);
        });
    }
}
