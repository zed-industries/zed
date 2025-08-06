use acp_thread::ModelSelector;
use agent_client_protocol as acp;
use anyhow::{anyhow, Result};
use futures::StreamExt;
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use language_model::{LanguageModel, LanguageModelRegistry};
use project::Project;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use crate::{templates::Templates, AgentResponseEvent, Thread};

/// Holds both the internal Thread and the AcpThread for a session
struct Session {
    /// The internal thread that processes messages
    thread: Entity<Thread>,
    /// The ACP thread that handles protocol communication
    acp_thread: Entity<acp_thread::AcpThread>,
}

pub struct NativeAgent {
    /// Session ID -> Session mapping
    sessions: HashMap<acp::SessionId, Session>,
    /// Shared templates for all threads
    templates: Arc<Templates>,
}

impl NativeAgent {
    pub fn new(templates: Arc<Templates>) -> Self {
        log::info!("Creating new NativeAgent");
        Self {
            sessions: HashMap::new(),
            templates,
        }
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
            // Create Thread
            let (session_id, thread) = agent.update(
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

                    let thread = cx.new(|_| Thread::new(project.clone(), agent.templates.clone(), default_model));

                    // Generate session ID
                    let session_id = acp::SessionId(uuid::Uuid::new_v4().to_string().into());
                    log::info!("Created session with ID: {}", session_id);
                    Ok((session_id, thread))
                },
            )??;

            // Create AcpThread
            let acp_thread = cx.update(|cx| {
                cx.new(|cx| {
                    acp_thread::AcpThread::new("agent2", self.clone(), project, session_id.clone(), cx)
                })
            })?;

            // Store the session
            agent.update(cx, |agent, _cx| {
                agent.sessions.insert(
                    session_id,
                    Session {
                        thread,
                        acp_thread: acp_thread.clone(),
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
            let mut response_stream =
                thread.update(cx, |thread, cx| thread.send(model, message, cx))?;

            // Handle response stream and forward to session.acp_thread
            while let Some(result) = response_stream.next().await {
                match result {
                    Ok(event) => {
                        log::trace!("Received completion event: {:?}", event);

                        match event {
                            AgentResponseEvent::Text(text) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.handle_session_update(
                                        acp::SessionUpdate::AgentMessageChunk {
                                            content: acp::ContentBlock::Text(acp::TextContent {
                                                text,
                                                annotations: None,
                                            }),
                                        },
                                        cx,
                                    )
                                })??;
                            }
                            AgentResponseEvent::Thinking(text) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.handle_session_update(
                                        acp::SessionUpdate::AgentThoughtChunk {
                                            content: acp::ContentBlock::Text(acp::TextContent {
                                                text,
                                                annotations: None,
                                            }),
                                        },
                                        cx,
                                    )
                                })??;
                            }
                            AgentResponseEvent::ToolCall(tool_call) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.handle_session_update(
                                        acp::SessionUpdate::ToolCall(tool_call),
                                        cx,
                                    )
                                })??;
                            }
                            AgentResponseEvent::ToolCallUpdate(tool_call_update) => {
                                acp_thread.update(cx, |thread, cx| {
                                    thread.handle_session_update(
                                        acp::SessionUpdate::ToolCallUpdate(tool_call_update),
                                        cx,
                                    )
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
