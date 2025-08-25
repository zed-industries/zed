use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

/// Settings for slash commands.
#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct SlashCommandSettings {
    /// Settings for the `/cargo-workspace` slash command.
    #[serde(default)]
    pub cargo_workspace: CargoWorkspaceCommandSettings,
}

/// Settings for the `/cargo-workspace` slash command.
#[derive(Deserialize, Serialize, Debug, Default, Clone, JsonSchema)]
pub struct CargoWorkspaceCommandSettings {
    /// Whether `/cargo-workspace` is enabled.
    #[serde(default)]
    pub enabled: bool,
}

impl Settings for SlashCommandSettings {
    const KEY: Option<&'static str> = Some("slash_commands");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut App) -> Result<Self> {
        SettingsSources::<Self::FileContent>::json_merge_with(
            [sources.default]
                .into_iter()
                .chain(sources.user)
                .chain(sources.server),
        )
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
