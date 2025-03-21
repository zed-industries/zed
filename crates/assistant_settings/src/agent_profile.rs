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

impl AgentProfile {
    pub fn read_only() -> Self {
        Self {
            name: "Read-only".into(),
            tools: HashMap::from_iter([
                ("diagnostics".into(), true),
                ("fetch".into(), true),
                ("list-directory".into(), true),
                ("now".into(), true),
                ("path-search".into(), true),
                ("read-file".into(), true),
                ("regex-search".into(), true),
                ("thinking".into(), true),
            ]),
            context_servers: HashMap::default(),
        }
    }

    pub fn code_writer() -> Self {
        Self {
            name: "Code Writer".into(),
            tools: HashMap::from_iter([
                ("bash".into(), true),
                ("delete-path".into(), true),
                ("diagnostics".into(), true),
                ("edit-files".into(), true),
                ("fetch".into(), true),
                ("list-directory".into(), true),
                ("now".into(), true),
                ("path-search".into(), true),
                ("read-file".into(), true),
                ("regex-search".into(), true),
                ("thinking".into(), true),
            ]),
            context_servers: HashMap::default(),
        }
    }
}
