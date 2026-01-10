use std::rc::Rc;
use std::sync::Arc;
use std::{any::Any, path::Path};

use acp_thread::AgentConnection;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use collections::HashSet;
use fs::Fs;
use gpui::{App, AppContext as _, SharedString, Task};
use project::agent_server_store::{AllAgentServersSettings, CODEX_NAME};
use settings::{SettingsStore, update_settings_file};

use crate::{AgentServer, AgentServerDelegate, load_proxy_env};

#[derive(Clone)]
pub struct Codex;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    crate::common_e2e_tests!(async |_, _| Codex, allow_option_id = "proceed_once");
}

impl AgentServer for Codex {
    fn name(&self) -> SharedString {
        "Codex".into()
    }

    fn logo(&self) -> ui::IconName {
        ui::IconName::AiOpenAi
    }

    fn default_mode(&self, cx: &App) -> Option<acp::SessionModeId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).codex.clone()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_mode.clone().map(acp::SessionModeId::new))
    }

    fn set_default_mode(&self, mode_id: Option<acp::SessionModeId>, fs: Arc<dyn Fs>, cx: &mut App) {
        update_settings_file(fs, cx, |settings, _| {
            settings
                .agent_servers
                .get_or_insert_default()
                .codex
                .get_or_insert_default()
                .default_mode = mode_id.map(|m| m.to_string())
        });
    }

    fn default_model(&self, cx: &App) -> Option<acp::ModelId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).codex.clone()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_model.clone().map(acp::ModelId::new))
    }

    fn set_default_model(&self, model_id: Option<acp::ModelId>, fs: Arc<dyn Fs>, cx: &mut App) {
        update_settings_file(fs, cx, |settings, _| {
            settings
                .agent_servers
                .get_or_insert_default()
                .codex
                .get_or_insert_default()
                .default_model = model_id.map(|m| m.to_string())
        });
    }

    fn favorite_model_ids(&self, cx: &mut App) -> HashSet<acp::ModelId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).codex.clone()
        });

        settings
            .as_ref()
            .map(|s| {
                s.favorite_models
                    .iter()
                    .map(|id| acp::ModelId::new(id.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn toggle_favorite_model(
        &self,
        model_id: acp::ModelId,
        should_be_favorite: bool,
        fs: Arc<dyn Fs>,
        cx: &App,
    ) {
        update_settings_file(fs, cx, move |settings, _| {
            let favorite_models = &mut settings
                .agent_servers
                .get_or_insert_default()
                .codex
                .get_or_insert_default()
                .favorite_models;

            let model_id_str = model_id.to_string();
            if should_be_favorite {
                if !favorite_models.contains(&model_id_str) {
                    favorite_models.push(model_id_str);
                }
            } else {
                favorite_models.retain(|id| id != &model_id_str);
            }
        });
    }

    fn default_config_option(&self, config_id: &str, cx: &App) -> Option<String> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).codex.clone()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_config_options.get(config_id).cloned())
    }

    fn set_default_config_option(
        &self,
        config_id: &str,
        value_id: Option<&str>,
        fs: Arc<dyn Fs>,
        cx: &mut App,
    ) {
        let config_id = config_id.to_string();
        let value_id = value_id.map(|s| s.to_string());
        update_settings_file(fs, cx, move |settings, _| {
            let config_options = &mut settings
                .agent_servers
                .get_or_insert_default()
                .codex
                .get_or_insert_default()
                .default_config_options;

            if let Some(value) = value_id.clone() {
                config_options.insert(config_id.clone(), value);
            } else {
                config_options.remove(&config_id);
            }
        });
    }

    fn favorite_config_option_value_ids(
        &self,
        config_id: &acp::SessionConfigId,
        cx: &mut App,
    ) -> HashSet<acp::SessionConfigValueId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings.get::<AllAgentServersSettings>(None).codex.clone()
        });

        settings
            .as_ref()
            .and_then(|s| s.favorite_config_option_values.get(config_id.0.as_ref()))
            .map(|values| {
                values
                    .iter()
                    .cloned()
                    .map(acp::SessionConfigValueId::new)
                    .collect()
            })
            .unwrap_or_default()
    }

    fn toggle_favorite_config_option_value(
        &self,
        config_id: acp::SessionConfigId,
        value_id: acp::SessionConfigValueId,
        should_be_favorite: bool,
        fs: Arc<dyn Fs>,
        cx: &App,
    ) {
        let config_id = config_id.to_string();
        let value_id = value_id.to_string();

        update_settings_file(fs, cx, move |settings, _| {
            let favorites = &mut settings
                .agent_servers
                .get_or_insert_default()
                .codex
                .get_or_insert_default()
                .favorite_config_option_values;

            let entry = favorites.entry(config_id.clone()).or_insert_with(Vec::new);

            if should_be_favorite {
                if !entry.iter().any(|v| v == &value_id) {
                    entry.push(value_id.clone());
                }
            } else {
                entry.retain(|v| v != &value_id);
                if entry.is_empty() {
                    favorites.remove(&config_id);
                }
            }
        });
    }

    fn connect(
        &self,
        root_dir: Option<&Path>,
        delegate: AgentServerDelegate,
        cx: &mut App,
    ) -> Task<Result<(Rc<dyn AgentConnection>, Option<task::SpawnInTerminal>)>> {
        let name = self.name();
        let root_dir = root_dir.map(|root_dir| root_dir.to_string_lossy().into_owned());
        let is_remote = delegate.project.read(cx).is_via_remote_server();
        let store = delegate.store.downgrade();
        let extra_env = load_proxy_env(cx);
        let default_mode = self.default_mode(cx);
        let default_model = self.default_model(cx);
        let default_config_options = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .codex
                .as_ref()
                .map(|s| s.default_config_options.clone())
                .unwrap_or_default()
        });

        cx.spawn(async move |cx| {
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&CODEX_NAME.into())
                        .context("Codex is not registered")?;
                    anyhow::Ok(agent.get_command(
                        root_dir.as_deref(),
                        extra_env,
                        delegate.status_tx,
                        delegate.new_version_available,
                        &mut cx.to_async(),
                    ))
                })??
                .await?;

            let connection = crate::acp::connect(
                name,
                command,
                root_dir.as_ref(),
                default_mode,
                default_model,
                default_config_options,
                is_remote,
                cx,
            )
            .await?;
            Ok((connection, login))
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
