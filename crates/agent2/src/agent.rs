use acp_thread::ModelSelector;
use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, AppContext, AsyncApp, Entity, Task};
use language_model::{LanguageModel, LanguageModelRegistry};
use project::Project;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use crate::{templates::Templates, Thread};

pub struct Agent {
    /// Session ID -> Thread entity mapping
    sessions: HashMap<acp::SessionId, Entity<Thread>>,
    /// Shared templates for all threads
    templates: Arc<Templates>,
}

impl Agent {
    pub fn new(templates: Arc<Templates>) -> Self {
        Self {
            sessions: HashMap::new(),
            templates,
        }
    }
}

/// Wrapper struct that implements the AgentConnection trait
#[derive(Clone)]
pub struct AgentConnection(pub Entity<Agent>);

impl ModelSelector for AgentConnection {
    fn list_models(&self, cx: &mut AsyncApp) -> Task<Result<Vec<Arc<dyn LanguageModel>>>> {
        let result = cx.update(|cx| {
            let registry = LanguageModelRegistry::read_global(cx);
            let models = registry.available_models(cx).collect::<Vec<_>>();
            if models.is_empty() {
                Err(anyhow::anyhow!("No models available"))
            } else {
                Ok(models)
            }
        });
        Task::ready(result.unwrap_or_else(|e| Err(anyhow::anyhow!("Failed to update: {}", e))))
    }

    fn select_model(
        &self,
        session_id: &acp::SessionId,
        model: Arc<dyn LanguageModel>,
        cx: &mut AsyncApp,
    ) -> Task<Result<()>> {
        let agent = self.0.clone();
        let result = agent.update(cx, |agent, cx| {
            if let Some(thread) = agent.sessions.get(session_id) {
                thread.update(cx, |thread, _| {
                    thread.selected_model = model;
                });
                Ok(())
            } else {
                Err(anyhow::anyhow!("Session not found"))
            }
        });
        Task::ready(result.unwrap_or_else(|e| Err(anyhow::anyhow!("Failed to update: {}", e))))
    }

    fn selected_model(
        &self,
        session_id: &acp::SessionId,
        cx: &mut AsyncApp,
    ) -> Task<Result<Arc<dyn LanguageModel>>> {
        let agent = self.0.clone();
        let thread_result = agent
            .read_with(cx, |agent, _| agent.sessions.get(session_id).cloned())
            .ok()
            .flatten()
            .ok_or_else(|| anyhow::anyhow!("Session not found"));

        match thread_result {
            Ok(thread) => {
                let selected = thread
                    .read_with(cx, |thread, _| thread.selected_model.clone())
                    .unwrap_or_else(|e| panic!("Failed to read thread: {}", e));
                Task::ready(Ok(selected))
            }
            Err(e) => Task::ready(Err(e)),
        }
    }
}

impl acp_thread::AgentConnection for AgentConnection {
    fn new_thread(
        self: Rc<Self>,
        project: Entity<Project>,
        cwd: &Path,
        cx: &mut AsyncApp,
    ) -> Task<Result<Entity<acp_thread::AcpThread>>> {
        let _cwd = cwd.to_owned();
        let agent = self.0.clone();

        cx.spawn(async move |cx| {
            // Create Thread and store in Agent
            let (session_id, _thread) =
                agent.update(cx, |agent, cx: &mut gpui::Context<Agent>| {
                    // Fetch default model
                    let default_model = LanguageModelRegistry::read_global(cx)
                        .available_models(cx)
                        .next()
                        .unwrap_or_else(|| panic!("No default model available"));

                    let thread = cx.new(|_| Thread::new(agent.templates.clone(), default_model));
                    let session_id = acp::SessionId(uuid::Uuid::new_v4().to_string().into());
                    agent.sessions.insert(session_id.clone(), thread.clone());
                    (session_id, thread)
                })?;

            // Create AcpThread
            let acp_thread = cx.update(|cx| {
                cx.new(|cx| {
                    acp_thread::AcpThread::new("agent2", self.clone(), project, session_id, cx)
                })
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

        cx.spawn(async move |cx| {
            // Get thread
            let thread: Entity<Thread> = agent
                .read_with(cx, |agent, _| agent.sessions.get(&session_id).cloned())?
                .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

            // Convert prompt to message
            let message = convert_prompt_to_message(params.prompt);

            // Get model using the ModelSelector capability (always available for agent2)
            // Get the selected model from the thread directly
            let model = thread.read_with(cx, |thread, _| thread.selected_model.clone())?;

            // Send to thread
            thread.update(cx, |thread, cx| thread.send(model, message, cx))?;

            Ok(())
        })
    }

    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App) {
        self.0.update(cx, |agent, _cx| {
            agent.sessions.remove(session_id);
        });
    }
}

/// Convert ACP content blocks to a message string
fn convert_prompt_to_message(blocks: Vec<acp::ContentBlock>) -> String {
    let mut message = String::new();

    for block in blocks {
        match block {
            acp::ContentBlock::Text(text) => {
                message.push_str(&text.text);
            }
            acp::ContentBlock::ResourceLink(link) => {
                message.push_str(&format!(" @{} ", link.uri));
            }
            acp::ContentBlock::Image(_) => {
                message.push_str(" [image] ");
            }
            acp::ContentBlock::Audio(_) => {
                message.push_str(" [audio] ");
            }
            acp::ContentBlock::Resource(resource) => {
                message.push_str(&format!(" [resource: {:?}] ", resource.resource));
            }
        }
    }

    message
}
