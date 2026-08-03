use std::{any::Any, rc::Rc, sync::Arc};

use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use fs::Fs;
use gpui::{App, Entity, Task};
use project::{AgentId, Project};

use crate::{NativeAgent, NativeAgentConnection, ThreadStore, templates::Templates};

#[derive(Clone)]
pub struct NativeAgentServer {
    fs: Arc<dyn Fs>,
    thread_store: Entity<ThreadStore>,
}

impl NativeAgentServer {
    pub fn new(fs: Arc<dyn Fs>, thread_store: Entity<ThreadStore>) -> Self {
        Self { fs, thread_store }
    }
}

impl AgentServer for NativeAgentServer {
    fn agent_id(&self) -> AgentId {
        crate::ZED_AGENT_ID.clone()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::ZedAgent
    }

    fn connect(
        &self,
        _delegate: AgentServerDelegate,
        _project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn acp_thread::AgentConnection>>> {
        log::debug!("NativeAgentServer::connect");
        let fs = self.fs.clone();
        let thread_store = self.thread_store.clone();
        cx.spawn(async move |cx| {
            log::debug!("Creating templates for native agent");
            let templates = Templates::new();

            log::debug!("Creating native agent entity");
            let agent = cx.update(|cx| NativeAgent::new(thread_store, templates, fs, cx));

            // Create the connection wrapper
            let connection = NativeAgentConnection(agent);
            log::debug!("NativeAgentServer connection established successfully");

            Ok(Rc::new(connection) as Rc<dyn acp_thread::AgentConnection>)
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use gpui::AppContext;

    agent_servers::e2e_tests::common_e2e_tests!(
        async |fs, cx| {
            let auth = cx.update(|cx| {
                prompt_store::init(cx);
                let registry = language_model::LanguageModelRegistry::read_global(cx);
                let auth = registry
                    .provider(&language_model::ANTHROPIC_PROVIDER_ID)
                    .unwrap()
                    .authenticate(cx);

                cx.spawn(async move |_| auth.await)
            });

            auth.await.unwrap();

            cx.update(|cx| {
                let registry = language_model::LanguageModelRegistry::global(cx);

                registry.update(cx, |registry, cx| {
                    registry.select_default_model(
                        Some(&language_model::SelectedModel {
                            provider: language_model::ANTHROPIC_PROVIDER_ID,
                            model: language_model::LanguageModelId("claude-sonnet-4-latest".into()),
                        }),
                        cx,
                    );
                });
            });

            let thread_store = cx.update(|cx| cx.new(|cx| ThreadStore::new(cx)));

            NativeAgentServer::new(fs.clone(), thread_store)
        },
        allow_option_id = "allow"
    );
}
