use std::sync::Arc;

use anyhow::{Result, bail};
use collections::IndexMap;
use convert_case::{Case, Casing as _};
use fs::Fs;
use gpui::{App, SharedString};
use settings::{
    AgentProfileContent, ContextServerPresetContent, LanguageModelSelection, Settings as _,
    SettingsContent, update_settings_file,
};
use util::ResultExt as _;

use crate::{AgentProfileId, AgentSettings};

pub mod builtin_profiles {
    use super::AgentProfileId;

    pub const WRITE: &str = "write";
    pub const ASK: &str = "ask";
    pub const MINIMAL: &str = "minimal";

    pub fn is_builtin(profile_id: &AgentProfileId) -> bool {
        profile_id.as_str() == WRITE || profile_id.as_str() == ASK || profile_id.as_str() == MINIMAL
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentProfile {
    id: AgentProfileId,
}

pub type AvailableProfiles = IndexMap<AgentProfileId, SharedString>;

impl AgentProfile {
    pub fn new(id: AgentProfileId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &AgentProfileId {
        &self.id
    }

    /// Saves a new profile to the settings.
    pub fn create(
        name: String,
        base_profile_id: Option<AgentProfileId>,
        fs: Arc<dyn Fs>,
        cx: &App,
    ) -> AgentProfileId {
        let id = AgentProfileId(name.to_case(Case::Kebab).into());

        let base_profile =
            base_profile_id.and_then(|id| AgentSettings::get_global(cx).profiles.get(&id).cloned());

        // Copy toggles from the base profile so the new profile starts with familiar defaults.
        let tools = base_profile
            .as_ref()
            .map(|profile| profile.tools.clone())
            .unwrap_or_default();
        let enable_all_context_servers = base_profile
            .as_ref()
            .map(|profile| profile.enable_all_context_servers)
            .unwrap_or_default();
        let context_servers = base_profile
            .as_ref()
            .map(|profile| profile.context_servers.clone())
            .unwrap_or_default();
        // Preserve the base profile's model preference when cloning into a new profile.
        let default_model = base_profile
            .as_ref()
            .and_then(|profile| profile.default_model.clone());

        let profile_settings = AgentProfileSettings {
            name: name.into(),
            tools,
            enable_all_context_servers,
            context_servers,
            default_model,
        };

        update_settings_file(fs, cx, {
            let id = id.clone();
            move |settings, _cx| {
                profile_settings.save_to_settings(id, settings).log_err();
            }
        });

        id
    }

    /// Returns a map of AgentProfileIds to their names
    pub fn available_profiles(cx: &App) -> AvailableProfiles {
        let mut profiles = AvailableProfiles::default();
        for (id, profile) in AgentSettings::get_global(cx).profiles.iter() {
            profiles.insert(id.clone(), profile.name.clone());
        }
        profiles
    }
}

/// A profile for the Zed Agent that controls its behavior.
#[derive(Debug, Clone)]
pub struct AgentProfileSettings {
    /// The name of the profile.
    pub name: SharedString,
    pub tools: IndexMap<Arc<str>, bool>,
    pub enable_all_context_servers: bool,
    pub context_servers: IndexMap<Arc<str>, ContextServerPreset>,
    /// Default language model to apply when this profile becomes active.
    pub default_model: Option<LanguageModelSelection>,
}

impl AgentProfileSettings {
    pub fn is_tool_enabled(&self, tool_name: &str) -> bool {
        self.tools.get(tool_name) == Some(&true)
    }

    pub fn is_context_server_tool_enabled(&self, server_id: &str, tool_name: &str) -> bool {
        self.context_servers
            .get(server_id)
            .and_then(|preset| preset.tools.get(tool_name).copied())
            .unwrap_or(self.enable_all_context_servers)
    }

    pub fn save_to_settings(
        &self,
        profile_id: AgentProfileId,
        content: &mut SettingsContent,
    ) -> Result<()> {
        let profiles = content
            .agent
            .get_or_insert_default()
            .profiles
            .get_or_insert_default();
        if profiles.contains_key(&profile_id.0) {
            bail!("profile with ID '{profile_id}' already exists");
        }

        profiles.insert(
            profile_id.0,
            AgentProfileContent {
                name: self.name.clone().into(),
                tools: self.tools.clone(),
                enable_all_context_servers: Some(self.enable_all_context_servers),
                context_servers: self
                    .context_servers
                    .clone()
                    .into_iter()
                    .map(|(server_id, preset)| {
                        (
                            server_id,
                            ContextServerPresetContent {
                                tools: preset.tools,
                            },
                        )
                    })
                    .collect(),
                default_model: self.default_model.clone(),
            },
        );

        Ok(())
    }
}

impl From<AgentProfileContent> for AgentProfileSettings {
    fn from(content: AgentProfileContent) -> Self {
        let AgentProfileContent {
            name,
            tools,
            enable_all_context_servers,
            context_servers,
            default_model,
        } = content;

        Self {
            name: name.into(),
            tools,
            enable_all_context_servers: enable_all_context_servers.unwrap_or_default(),
            context_servers: context_servers
                .into_iter()
                .map(|(server_id, preset)| (server_id, preset.into()))
                .collect(),
            default_model,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ContextServerPreset {
    pub tools: IndexMap<Arc<str>, bool>,
}

impl From<settings::ContextServerPresetContent> for ContextServerPreset {
    fn from(content: settings::ContextServerPresetContent) -> Self {
        Self {
            tools: content.tools,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile(
        enable_all_context_servers: bool,
        context_servers: IndexMap<Arc<str>, ContextServerPreset>,
    ) -> AgentProfileSettings {
        AgentProfileSettings {
            name: "test".into(),
            tools: IndexMap::default(),
            enable_all_context_servers,
            context_servers,
            default_model: None,
        }
    }

    fn preset(tools: &[(&str, bool)]) -> ContextServerPreset {
        ContextServerPreset {
            tools: tools
                .iter()
                .map(|(name, enabled)| (Arc::from(*name), *enabled))
                .collect(),
        }
    }

    #[test]
    fn explicit_false_disables_tool_when_enable_all_is_true() {
        let mut servers = IndexMap::default();
        servers.insert(Arc::from("server"), preset(&[("disabled_tool", false)]));
        let profile = profile(true, servers);

        assert!(!profile.is_context_server_tool_enabled("server", "disabled_tool"));
        assert!(profile.is_context_server_tool_enabled("server", "other_tool"));
        assert!(profile.is_context_server_tool_enabled("other_server", "any_tool"));
    }

    #[test]
    fn explicit_true_enables_tool_when_enable_all_is_false() {
        let mut servers = IndexMap::default();
        servers.insert(Arc::from("server"), preset(&[("enabled_tool", true)]));
        let profile = profile(false, servers);

        assert!(profile.is_context_server_tool_enabled("server", "enabled_tool"));
        assert!(!profile.is_context_server_tool_enabled("server", "other_tool"));
        assert!(!profile.is_context_server_tool_enabled("other_server", "any_tool"));
    }
}
