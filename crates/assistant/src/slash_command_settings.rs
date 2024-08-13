use anyhow::Result;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct SlashCommandSettings {
    #[serde(default)]
    pub docs: DocsCommandSettings,
    #[serde(default)]
    pub project: ProjectCommandSettings,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct DocsCommandSettings {
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct ProjectCommandSettings {
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
