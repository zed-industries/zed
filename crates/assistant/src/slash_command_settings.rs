use anyhow::Result;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

/// Settings for slash commands.
#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct SlashCommandSettings {
    /// Settings for the `/docs` slash command.
    #[serde(default)]
    pub docs: DocsCommandSettings,
    /// Settings for the `/project` slash command.
    #[serde(default)]
    pub project: ProjectCommandSettings,
}

/// Settings for the `/docs` slash command.
#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct DocsCommandSettings {
    /// Whether `/docs` is enabled.
    #[serde(default)]
    pub enabled: bool,
}

/// Settings for the `/project` slash command.
#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct ProjectCommandSettings {
    /// Whether `/project` is enabled.
    #[serde(default)]
    pub enabled: bool,
}

impl Settings for SlashCommandSettings {
    const KEY: Option<&'static str> = Some("slash_commands");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut AppContext) -> Result<Self> {
        SettingsSources::<Self::FileContent>::json_merge_with(
            [sources.default].into_iter().chain(sources.user),
        )
    }
}
