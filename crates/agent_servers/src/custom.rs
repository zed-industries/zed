use crate::{AgentServer, AgentServerDelegate, load_proxy_env};
use acp_thread::AgentConnection;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use collections::HashSet;
use fs::Fs;
use gpui::{App, AppContext as _, SharedString, Task};
use project::agent_server_store::{AllAgentServersSettings, ExternalAgentServerName};
use settings::{SettingsStore, update_settings_file};
use std::{path::Path, rc::Rc, sync::Arc};
use ui::IconName;

/// A generic agent server implementation for custom user-defined agents
pub struct CustomAgentServer {
    name: SharedString,
}

impl CustomAgentServer {
    pub fn new(name: SharedString) -> Self {
        Self { name }
    }
}

impl AgentServer for CustomAgentServer {
    fn name(&self) -> SharedString {
        self.name.clone()
    }

    fn logo(&self) -> IconName {
        IconName::Terminal
    }

    fn default_mode(&self, cx: &App) -> Option<acp::SessionModeId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .custom
                .get(&self.name())
                .cloned()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_mode().map(acp::SessionModeId::new))
    }

    fn favorite_config_option_value_ids(
        &self,
        config_id: &acp::SessionConfigId,
        cx: &mut App,
    ) -> HashSet<acp::SessionConfigValueId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .custom
                .get(&self.name())
                .cloned()
        });

        settings
            .as_ref()
            .and_then(|s| s.favorite_config_option_values(config_id.0.as_ref()))
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
        let name = self.name();
        let config_id = config_id.to_string();
        let value_id = value_id.to_string();

        update_settings_file(fs, cx, move |settings, _| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .custom
                .entry(name.clone())
                .or_insert_with(|| settings::CustomAgentServerSettings::Extension {
                    default_model: None,
                    default_mode: None,
                    favorite_models: Vec::new(),
                    default_config_options: Default::default(),
                    favorite_config_option_values: Default::default(),
                });

            match settings {
                settings::CustomAgentServerSettings::Custom {
                    favorite_config_option_values,
                    ..
                }
                | settings::CustomAgentServerSettings::Extension {
                    favorite_config_option_values,
                    ..
                } => {
                    let entry = favorite_config_option_values
                        .entry(config_id.clone())
                        .or_insert_with(Vec::new);

                    if should_be_favorite {
                        if !entry.iter().any(|v| v == &value_id) {
                            entry.push(value_id.clone());
                        }
                    } else {
                        entry.retain(|v| v != &value_id);
                        if entry.is_empty() {
                            favorite_config_option_values.remove(&config_id);
                        }
                    }
                }
            }
        });
    }

    fn set_default_mode(&self, mode_id: Option<acp::SessionModeId>, fs: Arc<dyn Fs>, cx: &mut App) {
        let name = self.name();
        update_settings_file(fs, cx, move |settings, _| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .custom
                .entry(name.clone())
                .or_insert_with(|| settings::CustomAgentServerSettings::Extension {
                    default_model: None,
                    default_mode: None,
                    favorite_models: Vec::new(),
                    default_config_options: Default::default(),
                    favorite_config_option_values: Default::default(),
                });

            match settings {
                settings::CustomAgentServerSettings::Custom { default_mode, .. }
                | settings::CustomAgentServerSettings::Extension { default_mode, .. } => {
                    *default_mode = mode_id.map(|m| m.to_string());
                }
            }
        });
    }

    fn default_model(&self, cx: &App) -> Option<acp::ModelId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .custom
                .get(&self.name())
                .cloned()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_model().map(acp::ModelId::new))
    }

    fn set_default_model(&self, model_id: Option<acp::ModelId>, fs: Arc<dyn Fs>, cx: &mut App) {
        let name = self.name();
        update_settings_file(fs, cx, move |settings, _| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .custom
                .entry(name.clone())
                .or_insert_with(|| settings::CustomAgentServerSettings::Extension {
                    default_model: None,
                    default_mode: None,
                    favorite_models: Vec::new(),
                    default_config_options: Default::default(),
                    favorite_config_option_values: Default::default(),
                });

            match settings {
                settings::CustomAgentServerSettings::Custom { default_model, .. }
                | settings::CustomAgentServerSettings::Extension { default_model, .. } => {
                    *default_model = model_id.map(|m| m.to_string());
                }
            }
        });
    }

    fn favorite_model_ids(&self, cx: &mut App) -> HashSet<acp::ModelId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .custom
                .get(&self.name())
                .cloned()
        });

        settings
            .as_ref()
            .map(|s| {
                s.favorite_models()
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
        let name = self.name();
        update_settings_file(fs, cx, move |settings, _| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .custom
                .entry(name.clone())
                .or_insert_with(|| settings::CustomAgentServerSettings::Extension {
                    default_model: None,
                    default_mode: None,
                    favorite_models: Vec::new(),
                    default_config_options: Default::default(),
                    favorite_config_option_values: Default::default(),
                });

            let favorite_models = match settings {
                settings::CustomAgentServerSettings::Custom {
                    favorite_models, ..
                }
                | settings::CustomAgentServerSettings::Extension {
                    favorite_models, ..
                } => favorite_models,
            };

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
            settings
                .get::<AllAgentServersSettings>(None)
                .custom
                .get(&self.name())
                .cloned()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_config_option(config_id).map(|s| s.to_string()))
    }

    fn set_default_config_option(
        &self,
        config_id: &str,
        value_id: Option<&str>,
        fs: Arc<dyn Fs>,
        cx: &mut App,
    ) {
        let name = self.name();
        let config_id = config_id.to_string();
        let value_id = value_id.map(|s| s.to_string());
        update_settings_file(fs, cx, move |settings, _| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .custom
                .entry(name.clone())
                .or_insert_with(|| settings::CustomAgentServerSettings::Extension {
                    default_model: None,
                    default_mode: None,
                    favorite_models: Vec::new(),
                    default_config_options: Default::default(),
                    favorite_config_option_values: Default::default(),
                });

            match settings {
                settings::CustomAgentServerSettings::Custom {
                    default_config_options,
                    ..
                }
                | settings::CustomAgentServerSettings::Extension {
                    default_config_options,
                    ..
                } => {
                    if let Some(value) = value_id.clone() {
                        default_config_options.insert(config_id.clone(), value);
                    } else {
                        default_config_options.remove(&config_id);
                    }
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
        let default_mode = self.default_mode(cx);
        let default_model = self.default_model(cx);
        let default_config_options = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .custom
                .get(&self.name())
                .map(|s| match s {
                    project::agent_server_store::CustomAgentServerSettings::Custom {
                        default_config_options,
                        ..
                    }
                    | project::agent_server_store::CustomAgentServerSettings::Extension {
                        default_config_options,
                        ..
                    } => default_config_options.clone(),
                })
                .unwrap_or_default()
        });
        let store = delegate.store.downgrade();
        let extra_env = load_proxy_env(cx);
        cx.spawn(async move |cx| {
            let (command, root_dir, login) = store
                .update(cx, |store, cx| {
                    let agent = store
                        .get_external_agent(&ExternalAgentServerName(name.clone()))
                        .with_context(|| {
                            format!("Custom agent server `{}` is not registered", name)
                        })?;
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

    fn into_any(self: Rc<Self>) -> Rc<dyn std::any::Any> {
        self
    }
}
