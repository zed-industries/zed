use acp_thread::ModelSelector;
use agent_client_protocol as acp;
use anyhow::{anyhow, Result};
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use language_model::{LanguageModel, LanguageModelRegistry};
use project::Project;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use crate::{templates::Templates, Thread};

/// Holds both the internal Thread and the AcpThread for a session
#[derive(Clone)]
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
            let session = agent
                .read_with(cx, |agent, _| agent.sessions.get(&session_id).cloned())?
                .ok_or_else(|| anyhow::anyhow!("Session not found"))?;
            let selected = session
                .thread
                .read_with(cx, |thread, _| thread.selected_model.clone())?;
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

                    let thread = cx.new(|_| Thread::new(agent.templates.clone(), default_model));

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

    fn prompt(&self, params: acp::PromptRequest, cx: &mut App) -> Task<Result<()>> {
        let session_id = params.session_id.clone();
        let agent = self.0.clone();
        log::info!("Received prompt request for session: {}", session_id);
        log::debug!("Prompt blocks count: {}", params.prompt.len());

        cx.spawn(async move |cx| {
            // Get session
            let session = agent
                .read_with(cx, |agent, _| {
                    agent.sessions.get(&session_id).map(|s| Session {
                        thread: s.thread.clone(),
                        acp_thread: s.acp_thread.clone(),
                    })
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
            let model = session
                .thread
                .read_with(cx, |thread, _| thread.selected_model.clone())?;

            // Send to thread
            log::info!("Sending message to thread with model: {:?}", model.name());
            let response_stream = session
                .thread
                .update(cx, |thread, cx| thread.send(model, message, cx))?;

            // Handle response stream and forward to session.acp_thread
            let acp_thread = session.acp_thread.clone();
            cx.spawn(async move |cx| {
                use futures::StreamExt;
                use language_model::LanguageModelCompletionEvent;

                let mut response_stream = response_stream;

                while let Some(result) = response_stream.next().await {
                    match result {
                        Ok(event) => {
                            log::trace!("Received completion event: {:?}", event);

                            match event {
                                LanguageModelCompletionEvent::Text(text) => {
                                    // Send text chunk as agent message
                                    acp_thread.update(cx, |thread, cx| {
                                        thread.handle_session_update(
                                            acp::SessionUpdate::AgentMessageChunk {
                                                content: acp::ContentBlock::Text(
                                                    acp::TextContent {
                                                        text: text.into(),
                                                        annotations: None,
                                                    },
                                                ),
                                            },
                                            cx,
                                        )
                                    })??;
                                }
                                LanguageModelCompletionEvent::ToolUse(tool_use) => {
                                    // Convert LanguageModelToolUse to ACP ToolCall
                                    acp_thread.update(cx, |thread, cx| {
                                        thread.handle_session_update(
                                            acp::SessionUpdate::ToolCall(acp::ToolCall {
                                                id: acp::ToolCallId(tool_use.id.to_string().into()),
                                                label: tool_use.name.to_string(),
                                                kind: acp::ToolKind::Other,
                                                status: acp::ToolCallStatus::Pending,
                                                content: vec![],
                                                locations: vec![],
                                                raw_input: Some(tool_use.input),
                                            }),
                                            cx,
                                        )
                                    })??;
                                }
                                LanguageModelCompletionEvent::StartMessage { .. } => {
                                    log::debug!("Started new assistant message");
                                }
                                LanguageModelCompletionEvent::UsageUpdate(usage) => {
                                    log::debug!("Token usage update: {:?}", usage);
                                }
                                LanguageModelCompletionEvent::Thinking { text, .. } => {
                                    // Send thinking text as agent thought chunk
                                    acp_thread.update(cx, |thread, cx| {
                                        thread.handle_session_update(
                                            acp::SessionUpdate::AgentThoughtChunk {
                                                content: acp::ContentBlock::Text(
                                                    acp::TextContent {
                                                        text: text.into(),
                                                        annotations: None,
                                                    },
                                                ),
                                            },
                                            cx,
                                        )
                                    })??;
                                }
                                LanguageModelCompletionEvent::StatusUpdate(status) => {
                                    log::trace!("Status update: {:?}", status);
                                }
                                LanguageModelCompletionEvent::Stop(stop_reason) => {
                                    log::debug!("Assistant message complete: {:?}", stop_reason);
                                }
                                LanguageModelCompletionEvent::RedactedThinking { .. } => {
                                    log::trace!("Redacted thinking event");
                                }
                                LanguageModelCompletionEvent::ToolUseJsonParseError {
                                    id,
                                    tool_name,
                                    raw_input,
                                    json_parse_error,
                                } => {
                                    log::error!(
                                        "Tool use JSON parse error for tool '{}' (id: {}): {} - input: {}",
                                        tool_name,
                                        id,
                                        json_parse_error,
                                        raw_input
                                    );
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
                anyhow::Ok(())
            })
            .detach();

            log::info!("Successfully sent prompt to thread and started response handler");
            Ok(())
        })
    }

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
        log::info!("Cancelling session: {}", session_id);
        self.0.update(cx, |agent, _cx| {
            agent.sessions.remove(session_id);
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
