use agent_client_protocol as acp;
use std::path::PathBuf;

use crate::AgentServerCommand;
use anyhow::Result;
use collections::HashMap;
use gpui::{App, SharedString};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

pub fn init(cx: &mut App) {
    AllAgentServersSettings::register(cx);
}

#[derive(Default, Deserialize, Serialize, Clone, JsonSchema, Debug, SettingsUi, SettingsKey)]
#[settings_key(key = "agent_servers")]
pub struct AllAgentServersSettings {
    pub gemini: Option<BuiltinAgentServerSettings>,
    pub claude: Option<BuiltinAgentServerSettings>,

    /// Custom agent servers configured by the user
    #[serde(flatten)]
    pub custom: HashMap<SharedString, CustomAgentServerSettings>,
}

#[derive(Default, Deserialize, Serialize, Clone, JsonSchema, Debug, PartialEq)]
pub struct BuiltinAgentServerSettings {
    /// Absolute path to a binary to be used when launching this agent.
    ///
    /// This can be used to run a specific binary without automatic downloads or searching `$PATH`.
    #[serde(rename = "command")]
    pub path: Option<PathBuf>,
    /// If a binary is specified in `command`, it will be passed these arguments.
    pub args: Option<Vec<String>>,
    /// If a binary is specified in `command`, it will be passed these environment variables.
    pub env: Option<HashMap<String, String>>,
    /// Whether to skip searching `$PATH` for an agent server binary when
    /// launching this agent.
    ///
    /// This has no effect if a `command` is specified. Otherwise, when this is
    /// `false`, Zed will search `$PATH` for an agent server binary and, if one
    /// is found, use it for threads with this agent. If no agent binary is
    /// found on `$PATH`, Zed will automatically install and use its own binary.
    /// When this is `true`, Zed will not search `$PATH`, and will always use
    /// its own binary.
    ///
    /// Default: true
    pub ignore_system_version: Option<bool>,
    /// The default mode for new threads.
    ///
    /// Note: Not all agents support modes.
    ///
    /// Default: None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<acp::SessionModeId>,
}

impl BuiltinAgentServerSettings {
    pub(crate) fn custom_command(self) -> Option<AgentServerCommand> {
        self.path.map(|path| AgentServerCommand {
            path,
            args: self.args.unwrap_or_default(),
            env: self.env,
        })
    }
}

impl From<AgentServerCommand> for BuiltinAgentServerSettings {
    fn from(value: AgentServerCommand) -> Self {
        BuiltinAgentServerSettings {
            path: Some(value.path),
            args: Some(value.args),
            env: value.env,
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema, Debug, PartialEq)]
pub struct CustomAgentServerSettings {
    #[serde(flatten)]
    pub command: AgentServerCommand,
    /// The default mode for new threads.
    ///
    /// Note: Not all agents support modes.
    ///
    /// Default: None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<acp::SessionModeId>,
}

impl settings::Settings for AllAgentServersSettings {
    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let mut settings = AllAgentServersSettings::default();

        for AllAgentServersSettings {
            gemini,
            claude,
            custom,
        } in sources.defaults_and_customizations()
        {
            if gemini.is_some() {
                settings.gemini = gemini.clone();
            }
            if claude.is_some() {
                settings.claude = claude.clone();
            }

            // Merge custom agents
            for (name, config) in custom {
                // Skip built-in agent names to avoid conflicts
                if name != "gemini" && name != "claude" {
                    settings.custom.insert(name.clone(), config.clone());
                }
            }
        }

        Ok(settings)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
