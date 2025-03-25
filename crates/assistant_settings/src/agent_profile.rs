use std::sync::Arc;

use gpui::SharedString;
use indexmap::IndexMap;

/// A profile for the Zed Agent that controls its behavior.
#[derive(Debug, Clone)]
pub struct AgentProfile {
    /// The name of the profile.
    pub name: SharedString,
    pub tools: IndexMap<Arc<str>, bool>,
    #[allow(dead_code)]
    pub context_servers: IndexMap<Arc<str>, ContextServerPreset>,
}

#[derive(Debug, Clone)]
pub struct ContextServerPreset {
    #[allow(dead_code)]
    pub tools: IndexMap<Arc<str>, bool>,
}
