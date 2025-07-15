mod claude;
mod gemini;
mod settings;
mod stdio_agent_server;

pub use claude::*;
pub use gemini::*;
pub use settings::*;
pub use stdio_agent_server::*;

use gpui::App;

pub fn init(cx: &mut App) {
    settings::init(cx);
}

use acp_thread::AcpThread;
use anyhow::Result;
use collections::HashMap;
use gpui::{AsyncApp, Entity, SharedString};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct AgentServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

pub struct AgentServerVersion {
    pub current_version: SharedString,
    pub supported: bool,
}

pub trait AgentServer: Send {
    fn new_thread(
        self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<Entity<AcpThread>>>;
}

impl std::fmt::Debug for AgentServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filtered_env = self.env.as_ref().map(|env| {
            env.iter()
                .map(|(k, v)| {
                    (
                        k,
                        if util::redact::should_redact(k) {
                            "[REDACTED]"
                        } else {
                            v
                        },
                    )
                })
                .collect::<Vec<_>>()
        });

        f.debug_struct("AgentServerCommand")
            .field("path", &self.path)
            .field("args", &self.args)
            .field("env", &filtered_env)
            .finish()
    }
}
