use std::sync::Arc;

use collections::IndexMap;
use convert_case::{Case, Casing as _};
use fs::Fs;
use gpui::{App, SharedString};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings as _, update_settings_file};
use util::ResultExt as _;

use crate::AgentSettings;

pub mod builtin_profiles {
    use super::AgentProfileId;

    pub const WRITE: &str = "write";
    pub const ASK: &str = "ask";
    pub const MINIMAL: &str = "minimal";

    pub fn is_builtin(profile_id: &AgentProfileId) -> bool {
        profile_id.as_str() == WRITE || profile_id.as_str() == ASK || profile_id.as_str() == MINIMAL
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentProfileId(pub Arc<str>);

impl AgentProfileId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AgentProfileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for AgentProfileId {
    fn default() -> Self {
        Self("write".into())
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

        let profile_settings = AgentProfileSettings {
            name: name.into(),
            tools: base_profile
                .as_ref()
                .map(|profile| profile.tools.clone())
                .unwrap_or_default(),
            enable_all_context_servers: base_profile
                .as_ref()
                .map(|profile| profile.enable_all_context_servers)
                .unwrap_or_default(),
            context_servers: base_profile
                .map(|profile| profile.context_servers)
                .unwrap_or_default(),
        };

        update_settings_file::<AgentSettings>(fs, cx, {
            let id = id.clone();
            move |settings, _cx| {
                settings.create_profile(id, profile_settings).log_err();
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
}

impl AgentProfileSettings {
    pub fn is_tool_enabled(&self, tool_name: &str) -> bool {
        self.tools.get(tool_name) == Some(&true)
    }

    pub fn is_context_server_tool_enabled(&self, server_id: &str, tool_name: &str) -> bool {
        self.enable_all_context_servers
            || self
                .context_servers
                .get(server_id)
                .is_some_and(|preset| preset.tools.get(tool_name) == Some(&true))
    }
}

#[derive(Debug, Clone, Default)]
pub struct ContextServerPreset {
    pub tools: IndexMap<Arc<str>, bool>,
}
