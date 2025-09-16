use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

/// Settings for slash commands.
#[derive(Debug, Default, Clone)]
pub struct SlashCommandSettings {
    /// Settings for the `/cargo-workspace` slash command.
    pub cargo_workspace: CargoWorkspaceCommandSettings,
}

/// Settings for the `/cargo-workspace` slash command.
#[derive(Debug, Default, Clone)]
pub struct CargoWorkspaceCommandSettings {
    /// Whether `/cargo-workspace` is enabled.
    pub enabled: bool,
}

impl Settings for SlashCommandSettings {
    fn from_defaults(content: &settings::SettingsContent, cx: &mut App) -> Self {
        Self {
            cargo_workspace: CargoWorkspaceCommandSettings {
                enabled: content.project.slash_commands.unwrap(),
            },
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, cx: &mut App) {
        todo!()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
