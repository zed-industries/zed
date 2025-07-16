mod claude;
mod gemini;
mod settings;
mod stdio_agent_server;

pub use claude::*;
pub use gemini::*;
pub use settings::*;
pub use stdio_agent_server::*;

use acp_thread::AcpThread;
use anyhow::Result;
use collections::HashMap;
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub fn init(cx: &mut App) {
    settings::init(cx);
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct AgentServerCommand {
    #[serde(rename = "command")]
    pub path: PathBuf,
    #[serde(default)]
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

pub enum AgentServerVersion {
    Supported,
    Unsupported {
        error_message: SharedString,
        upgrade_message: SharedString,
        upgrade_command: String,
    },
}

pub trait AgentServer: Send {
    fn logo(&self) -> ui::IconName;
    fn name(&self) -> &'static str;
    fn empty_state_headline(&self) -> &'static str;
    fn empty_state_message(&self) -> &'static str;
    fn supports_always_allow(&self) -> bool;

    fn new_thread(
        &self,
        root_dir: &Path,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Entity<AcpThread>>>;
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
