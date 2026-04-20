use std::{any::Any, rc::Rc, sync::Arc};

use agent_client_protocol as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use agent_settings::{AgentSettings, language_model_to_selection};
use anyhow::Result;
use collections::HashSet;
use fs::Fs;
use gpui::{App, Entity, Task};
use language_model::{LanguageModelId, LanguageModelProviderId, LanguageModelRegistry};
use project::{AgentId, Project};
use prompt_store::PromptStore;
use settings::{LanguageModelSelection, Settings as _, update_settings_file};

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
        let prompt_store = PromptStore::global(cx);
        cx.spawn(async move |cx| {
            log::debug!("Creating templates for native agent");
            let templates = Templates::new();
            let prompt_store = prompt_store.await?;

            log::debug!("Creating native agent entity");
            let agent = cx
                .update(|cx| NativeAgent::new(thread_store, templates, Some(prompt_store), fs, cx));

            // Create the connection wrapper
            let connection = NativeAgentConnection(agent);
            log::debug!("NativeAgentServer connection established successfully");

            Ok(Rc::new(connection) as Rc<dyn acp_thread::AgentConnection>)
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

    fn favorite_model_ids(&self, cx: &mut App) -> HashSet<acp::ModelId> {
        AgentSettings::get_global(cx).favorite_model_ids()
    }

    fn toggle_favorite_model(
        &self,
        model_id: acp::ModelId,
        should_be_favorite: bool,
        fs: Arc<dyn Fs>,
        cx: &App,
    ) {
        let selection = model_id_to_selection(&model_id, cx);
        update_settings_file(fs, cx, move |settings, _| {
            let agent = settings.agent.get_or_insert_default();
            if should_be_favorite {
                agent.add_favorite_model(selection.clone());
            } else {
                agent.remove_favorite_model(&selection);
            }
        });
    }
}

/// Convert a ModelId (e.g. "anthropic/claude-3-5-sonnet") to a LanguageModelSelection.
fn model_id_to_selection(model_id: &acp::ModelId, cx: &App) -> LanguageModelSelection {
    let id = model_id.0.as_ref();
    let (provider, model) = id.split_once('/').unwrap_or(("", id));

    let provider_id = LanguageModelProviderId(provider.to_string().into());
    let model_id_typed = LanguageModelId(model.to_string().into());
    let resolved = LanguageModelRegistry::global(cx)
        .read(cx)
        .provider(&provider_id)
        .and_then(|p| {
            p.provided_models(cx)
                .into_iter()
                .find(|m| m.id() == model_id_typed)
        });

    let Some(resolved) = resolved else {
        return LanguageModelSelection {
            provider: provider.to_owned().into(),
            model: model.to_owned(),
            enable_thinking: false,
            effort: None,
            speed: None,
        };
    };

    let current_user_selection = AgentSettings::get_global(cx)
        .default_model
        .as_ref()
        .filter(|selection| {
            selection.provider.0 == resolved.provider_id().0.as_ref()
                && selection.model == resolved.id().0.as_ref()
        })
        .cloned();

    language_model_to_selection(&resolved, current_user_selection.as_ref())
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
