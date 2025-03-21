use std::sync::Arc;

use collections::HashMap;
use gpui::SharedString;

/// A profile for the Zed Agent that controls its behavior.
#[derive(Debug, Clone)]
pub struct AgentProfile {
    /// The name of the profile.
    pub name: SharedString,
    pub tools: HashMap<Arc<str>, bool>,
    #[allow(dead_code)]
    pub context_servers: HashMap<Arc<str>, ContextServerPreset>,
}

#[derive(Debug, Clone)]
pub struct ContextServerPreset {
    #[allow(dead_code)]
    pub tools: HashMap<Arc<str>, bool>,
}
