use std::{any::Any, path::Path, rc::Rc, sync::Arc};

use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use fs::Fs;
use gpui::{App, Entity, SharedString, Task};
use prompt_store::PromptStore;

use crate::{HistoryStore, NativeAgent, NativeAgentConnection, templates::Templates};

#[derive(Clone)]
pub struct NativeAgentServer {
    fs: Arc<dyn Fs>,
    history: Entity<HistoryStore>,
}

impl NativeAgentServer {
    pub fn new(fs: Arc<dyn Fs>, history: Entity<HistoryStore>) -> Self {
        Self { fs, history }
    }
}

impl AgentServer for NativeAgentServer {
    fn telemetry_id(&self) -> &'static str {
        "zed"
    }

    fn name(&self) -> SharedString {
        "Zed Agent".into()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::ZedAgent
    }

    fn connect(
        &self,
        _root_dir: &Path,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn acp_thread::AgentConnection>>> {
        log::debug!(
            "NativeAgentServer::connect called for path: {:?}",
            _root_dir
        );
        let project = delegate.project().clone();
        let fs = self.fs.clone();
        let history = self.history.clone();
        let prompt_store = PromptStore::global(cx);
        cx.spawn(async move |cx| {
            log::debug!("Creating templates for native agent");
            let templates = Templates::new();
            let prompt_store = prompt_store.await?;

            log::debug!("Creating native agent entity");
            let agent =
                NativeAgent::new(project, history, templates, Some(prompt_store), fs, cx).await?;

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

    use assistant_context::ContextStore;
    use gpui::AppContext;

    agent_servers::e2e_tests::common_e2e_tests!(
        async |fs, project, cx| {
            let auth = cx.update(|cx| {
                prompt_store::init(cx);
                terminal::init(cx);

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

            let history = cx.update(|cx| {
                let context_store = cx.new(move |cx| ContextStore::fake(project.clone(), cx));
                cx.new(move |cx| HistoryStore::new(context_store, cx))
            });

            NativeAgentServer::new(fs.clone(), history)
        },
        allow_option_id = "allow"
    );
}
