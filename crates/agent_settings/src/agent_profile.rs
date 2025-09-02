use std::sync::Arc;

use collections::IndexMap;
use gpui::SharedString;
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
