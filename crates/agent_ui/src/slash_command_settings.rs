use gpui::App;
use settings::Settings;

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

// todo!() I think this setting is bogus... default.json has "slash_commands": {"project"}
impl Settings for SlashCommandSettings {
    fn from_defaults(_content: &settings::SettingsContent, _cx: &mut App) -> Self {
        Self {
            cargo_workspace: CargoWorkspaceCommandSettings { enabled: false },
        }
    }

    fn refine(&mut self, _content: &settings::SettingsContent, _cx: &mut App) {}
}
