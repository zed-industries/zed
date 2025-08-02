use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, AppContext, AsyncApp, Entity, Task};
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
pub struct AgentConnection(pub Entity<Agent>);

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
                    let thread = cx.new(|_| Thread::new(agent.templates.clone()));
                    let session_id = acp::SessionId(uuid::Uuid::new_v4().to_string().into());
                    agent.sessions.insert(session_id.clone(), thread.clone());
                    (session_id, thread)
                })?;

            // Create AcpThread
            let acp_thread = cx.update(|cx| {
                cx.new(|cx| acp_thread::AcpThread::new("agent2", self, project, session_id, cx))
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

    fn prompt(&self, params: acp::PromptRequest, cx: &mut App) -> Task<Result<()>> {
        let session_id = params.session_id.clone();
        let agent = self.0.clone();

        cx.spawn(|cx| async move {
            // Get thread
            let thread: Entity<Thread> = agent
                .read_with(cx, |agent, _| agent.sessions.get(&session_id).cloned())?
                .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

            // Convert prompt to message
            let message = convert_prompt_to_message(params.prompt);

            // TODO: Get model from somewhere - for now use a placeholder
            log::warn!("Model selection not implemented - need to get from UI context");

            // Send to thread
            // thread.update(&mut cx, |thread, cx| {
            //     thread.send(model, message, cx)
            // })?;

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
