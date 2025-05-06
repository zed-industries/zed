use std::sync::Arc;

use gpui::SharedString;
use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod builtin_profiles {
    use super::AgentProfileId;

    pub const WRITE: &str = "write";
    pub const ASK: &str = "ask";
    pub const MINIMAL: &str = "minimal";

    pub fn is_builtin(profile_id: &AgentProfileId) -> bool {
        profile_id.as_str() == WRITE || profile_id.as_str() == ASK || profile_id.as_str() == MINIMAL
    }
}

#[derive(Default)]
pub struct GroupedAgentProfiles {
    pub builtin: IndexMap<AgentProfileId, AgentProfile>,
    pub custom: IndexMap<AgentProfileId, AgentProfile>,
}

impl GroupedAgentProfiles {
    pub fn from_settings(settings: &crate::AgentSettings) -> Self {
        let mut builtin = IndexMap::default();
        let mut custom = IndexMap::default();

        for (profile_id, profile) in settings.profiles.clone() {
            if builtin_profiles::is_builtin(&profile_id) {
                builtin.insert(profile_id, profile);
            } else {
                custom.insert(profile_id, profile);
            }
        }

        Self { builtin, custom }
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

/// A profile for the Zed Agent that controls its behavior.
#[derive(Debug, Clone)]
pub struct AgentProfile {
    /// The name of the profile.
    pub name: SharedString,
    pub tools: IndexMap<Arc<str>, bool>,
    pub enable_all_context_servers: bool,
    pub context_servers: IndexMap<Arc<str>, ContextServerPreset>,
}

#[derive(Debug, Clone, Default)]
pub struct ContextServerPreset {
    pub tools: IndexMap<Arc<str>, bool>,
}
