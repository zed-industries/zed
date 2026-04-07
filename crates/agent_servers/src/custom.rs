use crate::{AgentServer, AgentServerDelegate, load_proxy_env};
use acp_thread::AgentConnection;
use agent_client_protocol as acp;
use anyhow::{Context as _, Result};
use collections::HashSet;
use fs::Fs;
use gpui::{App, AppContext as _, Entity, Task};
use language_model::{ApiKey, EnvVar};
use project::{
    Project,
    agent_server_store::{AgentId, AllAgentServersSettings},
};
use settings::{SettingsStore, update_settings_file};
use std::{rc::Rc, sync::Arc};
use ui::IconName;

pub const GEMINI_ID: &str = "gemini";
pub const CLAUDE_AGENT_ID: &str = "claude-acp";
pub const CODEX_ID: &str = "codex-acp";

/// A generic agent server implementation for custom user-defined agents
pub struct CustomAgentServer {
    agent_id: AgentId,
}

impl CustomAgentServer {
    pub fn new(agent_id: AgentId) -> Self {
        Self { agent_id }
    }
}

impl AgentServer for CustomAgentServer {
    fn agent_id(&self) -> AgentId {
        self.agent_id.clone()
    }

    fn logo(&self) -> IconName {
        IconName::Terminal
    }

    fn default_mode(&self, cx: &App) -> Option<acp::SessionModeId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .get(self.agent_id().0.as_ref())
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
                .get(self.agent_id().0.as_ref())
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
        let agent_id = self.agent_id();
        let config_id = config_id.to_string();
        let value_id = value_id.to_string();

        update_settings_file(fs, cx, move |settings, cx| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .entry(agent_id.0.to_string())
                .or_insert_with(|| default_settings_for_agent(agent_id, cx));

            match settings {
                settings::CustomAgentServerSettings::Custom {
                    favorite_config_option_values,
                    ..
                }
                | settings::CustomAgentServerSettings::Extension {
                    favorite_config_option_values,
                    ..
                }
                | settings::CustomAgentServerSettings::Registry {
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
        let agent_id = self.agent_id();
        update_settings_file(fs, cx, move |settings, cx| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .entry(agent_id.0.to_string())
                .or_insert_with(|| default_settings_for_agent(agent_id, cx));

            match settings {
                settings::CustomAgentServerSettings::Custom { default_mode, .. }
                | settings::CustomAgentServerSettings::Extension { default_mode, .. }
                | settings::CustomAgentServerSettings::Registry { default_mode, .. } => {
                    *default_mode = mode_id.map(|m| m.to_string());
                }
            }
        });
    }

    fn default_model(&self, cx: &App) -> Option<acp::ModelId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .get(self.agent_id().as_ref())
                .cloned()
        });

        settings
            .as_ref()
            .and_then(|s| s.default_model().map(acp::ModelId::new))
    }

    fn set_default_model(&self, model_id: Option<acp::ModelId>, fs: Arc<dyn Fs>, cx: &mut App) {
        let agent_id = self.agent_id();
        update_settings_file(fs, cx, move |settings, cx| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .entry(agent_id.0.to_string())
                .or_insert_with(|| default_settings_for_agent(agent_id, cx));

            match settings {
                settings::CustomAgentServerSettings::Custom { default_model, .. }
                | settings::CustomAgentServerSettings::Extension { default_model, .. }
                | settings::CustomAgentServerSettings::Registry { default_model, .. } => {
                    *default_model = model_id.map(|m| m.to_string());
                }
            }
        });
    }

    fn favorite_model_ids(&self, cx: &mut App) -> HashSet<acp::ModelId> {
        let settings = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .get(self.agent_id().as_ref())
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
        let agent_id = self.agent_id();
        update_settings_file(fs, cx, move |settings, cx| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .entry(agent_id.0.to_string())
                .or_insert_with(|| default_settings_for_agent(agent_id, cx));

            let favorite_models = match settings {
                settings::CustomAgentServerSettings::Custom {
                    favorite_models, ..
                }
                | settings::CustomAgentServerSettings::Extension {
                    favorite_models, ..
                }
                | settings::CustomAgentServerSettings::Registry {
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
                .get(self.agent_id().as_ref())
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
        let agent_id = self.agent_id();
        let config_id = config_id.to_string();
        let value_id = value_id.map(|s| s.to_string());
        update_settings_file(fs, cx, move |settings, cx| {
            let settings = settings
                .agent_servers
                .get_or_insert_default()
                .entry(agent_id.0.to_string())
                .or_insert_with(|| default_settings_for_agent(agent_id, cx));

            match settings {
                settings::CustomAgentServerSettings::Custom {
                    default_config_options,
                    ..
                }
                | settings::CustomAgentServerSettings::Extension {
                    default_config_options,
                    ..
                }
                | settings::CustomAgentServerSettings::Registry {
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
        delegate: AgentServerDelegate,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Rc<dyn AgentConnection>>> {
        let agent_id = self.agent_id();
        let default_mode = self.default_mode(cx);
        let default_model = self.default_model(cx);
        let is_registry_agent = is_registry_agent(agent_id.clone(), cx);
        let default_config_options = cx.read_global(|settings: &SettingsStore, _| {
            settings
                .get::<AllAgentServersSettings>(None)
                .get(self.agent_id().as_ref())
                .map(|s| match s {
                    project::agent_server_store::CustomAgentServerSettings::Custom {
                        default_config_options,
                        ..
                    }
                    | project::agent_server_store::CustomAgentServerSettings::Extension {
                        default_config_options,
                        ..
                    }
                    | project::agent_server_store::CustomAgentServerSettings::Registry {
                        default_config_options,
                        ..
                    } => default_config_options.clone(),
                })
                .unwrap_or_default()
        });

        if is_registry_agent {
            if let Some(registry_store) = project::AgentRegistryStore::try_global(cx) {
                registry_store.update(cx, |store, cx| store.refresh_if_stale(cx));
            }
        }

        let mut extra_env = load_proxy_env(cx);
        if delegate.store.read(cx).no_browser() {
            extra_env.insert("NO_BROWSER".to_owned(), "1".to_owned());
        }
        if is_registry_agent {
            match agent_id.as_ref() {
                CLAUDE_AGENT_ID => {
                    extra_env.insert("ANTHROPIC_API_KEY".into(), "".into());
                }
                CODEX_ID => {
                    if let Ok(api_key) = std::env::var("CODEX_API_KEY") {
                        extra_env.insert("CODEX_API_KEY".into(), api_key);
                    }
                    if let Ok(api_key) = std::env::var("OPEN_AI_API_KEY") {
                        extra_env.insert("OPEN_AI_API_KEY".into(), api_key);
                    }
                }
                GEMINI_ID => {
                    extra_env.insert("SURFACE".to_owned(), "zed".to_owned());
                }
                _ => {}
            }
        }
        let store = delegate.store.downgrade();
        cx.spawn(async move |cx| {
            if is_registry_agent && agent_id.as_ref() == GEMINI_ID {
                if let Some(api_key) = cx.update(api_key_for_gemini_cli).await.ok() {
                    extra_env.insert("GEMINI_API_KEY".into(), api_key);
                }
            }
            let command = store
                .update(cx, |store, cx| {
                    let agent = store.get_external_agent(&agent_id).with_context(|| {
                        format!("Custom agent server `{}` is not registered", agent_id)
                    })?;
                    anyhow::Ok(agent.get_command(
                        extra_env,
                        delegate.new_version_available,
                        &mut cx.to_async(),
                    ))
                })??
                .await?;
            let connection = crate::acp::connect(
                agent_id,
                project,
                command,
                default_mode,
                default_model,
                default_config_options,
                cx,
            )
            .await?;
            Ok(connection)
        })
    }

    fn into_any(self: Rc<Self>) -> Rc<dyn std::any::Any> {
        self
    }
}

fn api_key_for_gemini_cli(cx: &mut App) -> Task<Result<String>> {
    let env_var = EnvVar::new("GEMINI_API_KEY".into()).or(EnvVar::new("GOOGLE_AI_API_KEY".into()));
    if let Some(key) = env_var.value {
        return Task::ready(Ok(key));
    }
    let credentials_provider = zed_credentials_provider::global(cx);
    let api_url = google_ai::API_URL.to_string();
    cx.spawn(async move |cx| {
        Ok(
            ApiKey::load_from_system_keychain(&api_url, credentials_provider.as_ref(), cx)
                .await?
                .key()
                .to_string(),
        )
    })
}

fn is_registry_agent(agent_id: impl Into<AgentId>, cx: &App) -> bool {
    let agent_id = agent_id.into();
    let is_in_registry = project::AgentRegistryStore::try_global(cx)
        .map(|store| store.read(cx).agent(&agent_id).is_some())
        .unwrap_or(false);
    let is_settings_registry = cx.read_global(|settings: &SettingsStore, _| {
        settings
            .get::<AllAgentServersSettings>(None)
            .get(agent_id.as_ref())
            .is_some_and(|s| {
                matches!(
                    s,
                    project::agent_server_store::CustomAgentServerSettings::Registry { .. }
                )
            })
    });
    is_in_registry || is_settings_registry
}

fn default_settings_for_agent(
    agent_id: impl Into<AgentId>,
    cx: &App,
) -> settings::CustomAgentServerSettings {
    if is_registry_agent(agent_id, cx) {
        settings::CustomAgentServerSettings::Registry {
            default_model: None,
            default_mode: None,
            env: Default::default(),
            favorite_models: Vec::new(),
            default_config_options: Default::default(),
            favorite_config_option_values: Default::default(),
        }
    } else {
        settings::CustomAgentServerSettings::Extension {
            default_model: None,
            default_mode: None,
            env: Default::default(),
            favorite_models: Vec::new(),
            default_config_options: Default::default(),
            favorite_config_option_values: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collections::HashMap;
    use gpui::TestAppContext;
    use project::agent_registry_store::{
        AgentRegistryStore, RegistryAgent, RegistryAgentMetadata, RegistryNpxAgent,
    };
    use settings::Settings as _;
    use ui::SharedString;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    fn init_registry_with_agents(cx: &mut TestAppContext, agent_ids: &[&str]) {
        let agents: Vec<RegistryAgent> = agent_ids
            .iter()
            .map(|id| {
                let id = SharedString::from(id.to_string());
                RegistryAgent::Npx(RegistryNpxAgent {
                    metadata: RegistryAgentMetadata {
                        id: AgentId::new(id.clone()),
                        name: id.clone(),
                        description: SharedString::from(""),
                        version: SharedString::from("1.0.0"),
                        repository: None,
                        website: None,
                        icon_path: None,
                    },
                    package: id,
                    args: Vec::new(),
                    env: HashMap::default(),
                })
            })
            .collect();
        cx.update(|cx| {
            AgentRegistryStore::init_test_global(cx, agents);
        });
    }

    fn set_agent_server_settings(
        cx: &mut TestAppContext,
        entries: Vec<(&str, settings::CustomAgentServerSettings)>,
    ) {
        cx.update(|cx| {
            AllAgentServersSettings::override_global(
                project::agent_server_store::AllAgentServersSettings(
                    entries
                        .into_iter()
                        .map(|(name, settings)| (name.to_string(), settings.into()))
                        .collect(),
                ),
                cx,
            );
        });
    }

    #[gpui::test]
    fn test_unknown_agent_is_not_registry(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            assert!(!is_registry_agent("my-custom-agent", cx));
        });
    }

    #[gpui::test]
    fn test_agent_in_registry_store_is_registry(cx: &mut TestAppContext) {
        init_test(cx);
        init_registry_with_agents(cx, &["some-new-registry-agent"]);
        cx.update(|cx| {
            assert!(is_registry_agent("some-new-registry-agent", cx));
            assert!(!is_registry_agent("not-in-registry", cx));
        });
    }

    #[gpui::test]
    fn test_agent_with_registry_settings_type_is_registry(cx: &mut TestAppContext) {
        init_test(cx);
        set_agent_server_settings(
            cx,
            vec![(
                "agent-from-settings",
                settings::CustomAgentServerSettings::Registry {
                    env: HashMap::default(),
                    default_mode: None,
                    default_model: None,
                    favorite_models: Vec::new(),
                    default_config_options: HashMap::default(),
                    favorite_config_option_values: HashMap::default(),
                },
            )],
        );
        cx.update(|cx| {
            assert!(is_registry_agent("agent-from-settings", cx));
        });
    }

    #[gpui::test]
    fn test_agent_with_extension_settings_type_is_not_registry(cx: &mut TestAppContext) {
        init_test(cx);
        set_agent_server_settings(
            cx,
            vec![(
                "my-extension-agent",
                settings::CustomAgentServerSettings::Extension {
                    env: HashMap::default(),
                    default_mode: None,
                    default_model: None,
                    favorite_models: Vec::new(),
                    default_config_options: HashMap::default(),
                    favorite_config_option_values: HashMap::default(),
                },
            )],
        );
        cx.update(|cx| {
            assert!(!is_registry_agent("my-extension-agent", cx));
        });
    }

    #[gpui::test]
    fn test_default_settings_for_extension_agent(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            assert!(matches!(
                default_settings_for_agent("some-extension-agent", cx),
                settings::CustomAgentServerSettings::Extension { .. }
            ));
        });
    }

    #[gpui::test]
    fn test_default_settings_for_agent_in_registry(cx: &mut TestAppContext) {
        init_test(cx);
        init_registry_with_agents(cx, &["new-registry-agent"]);
        cx.update(|cx| {
            assert!(matches!(
                default_settings_for_agent("new-registry-agent", cx),
                settings::CustomAgentServerSettings::Registry { .. }
            ));
            assert!(matches!(
                default_settings_for_agent("not-in-registry", cx),
                settings::CustomAgentServerSettings::Extension { .. }
            ));
        });
    }
}
