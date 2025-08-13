use crate::AgentServerCommand;
use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

pub fn init(cx: &mut App) {
    AllAgentServersSettings::register(cx);
}

#[derive(Default, Deserialize, Serialize, Clone, JsonSchema, Debug)]
pub struct AllAgentServersSettings {
    pub gemini: Option<AgentServerSettings>,
    pub claude: Option<AgentServerSettings>,
}

#[derive(Deserialize, Serialize, Clone, JsonSchema, Debug)]
pub struct AgentServerSettings {
    #[serde(flatten)]
    pub command: AgentServerCommand,
}

impl settings::Settings for AllAgentServersSettings {
    const KEY: Option<&'static str> = Some("agent_servers");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        let mut settings = AllAgentServersSettings::default();

        for AllAgentServersSettings { gemini, claude } in sources.defaults_and_customizations() {
            if gemini.is_some() {
                settings.gemini = gemini.clone();
            }
            if claude.is_some() {
                settings.claude = claude.clone();
            }
        }

        Ok(settings)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
