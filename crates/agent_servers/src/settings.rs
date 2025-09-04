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
pub struct AllAgentServersSettingsContent {
    gemini: Option<GeminiSettingsContent>,
    claude: Option<AgentServerCommand>,
    /// Custom agent servers configured by the user
    #[serde(flatten)]
    pub custom: HashMap<SharedString, AgentServerCommand>,
}

#[derive(Clone, Debug, Default)]
pub struct AllAgentServersSettings {
    pub commands: HashMap<SharedString, AgentServerCommand>,
    pub gemini_is_system: bool,
}

impl AllAgentServersSettings {
    pub fn is_system(this: &Self, name: &str) -> bool {
        if name == "gemini" {
            this.gemini_is_system
        } else {
            false
        }
    }
}

impl std::ops::Deref for AllAgentServersSettings {
    type Target = HashMap<SharedString, AgentServerCommand>;
    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}

impl std::ops::DerefMut for AllAgentServersSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub struct GeminiSettingsContent {
    ignore_system_version: Option<bool>,
    #[serde(flatten)]
    inner: Option<AgentServerCommand>,
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
pub struct AgentServerSettings {
    #[serde(flatten)]
    pub command: AgentServerCommand,
}

impl settings::Settings for AllAgentServersSettings {
    type FileContent = AllAgentServersSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let mut settings = AllAgentServersSettings::default();

        for AllAgentServersSettingsContent {
            gemini,
            claude,
            custom,
        } in sources.defaults_and_customizations()
        {
            if let Some(gemini) = gemini {
                if let Some(ignore) = gemini.ignore_system_version {
                    settings.gemini_is_system = !ignore;
                }
                if let Some(gemini) = gemini.inner.as_ref() {
                    settings.insert("gemini".into(), gemini.clone());
                }
            }
            if let Some(claude) = claude.clone() {
                settings.insert("claude".into(), claude);
            }

            // Merge custom agents
            for (name, command) in custom {
                // Skip built-in agent names to avoid conflicts
                if name != "gemini" && name != "claude" {
                    settings.commands.insert(name.clone(), command.clone());
                }
            }
        }

        Ok(settings)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{AgentServerCommand, GeminiSettingsContent};

    #[test]
    fn test_deserialization() {
        let value = json!({
            "command": "foo",
            "args": ["bar"],
            "ignore_system_version": false
        });
        let settings = serde_json::from_value::<GeminiSettingsContent>(value).unwrap();
        assert_eq!(
            settings,
            GeminiSettingsContent {
                ignore_system_version: Some(false),
                inner: Some(AgentServerCommand {
                    path: "foo".into(),
                    args: vec!["bar".into()],
                    env: Default::default(),
                })
            }
        )
    }
}
