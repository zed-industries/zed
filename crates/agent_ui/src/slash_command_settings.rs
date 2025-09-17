use gpui::App;
use settings::Settings;
use util::MergeFrom;

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
    fn from_defaults(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        Self {
            cargo_workspace: CargoWorkspaceCommandSettings {
                enabled: content
                    .project
                    .slash_commands
                    .clone()
                    .unwrap()
                    .cargo_workspace
                    .unwrap()
                    .enabled
                    .unwrap(),
            },
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, _cx: &mut App) {
        let Some(slash_command) = content.project.slash_commands.as_ref() else {
            return;
        };
        let Some(cargo_workspace) = slash_command.cargo_workspace.as_ref() else {
            return;
        };
        self.cargo_workspace
            .enabled
            .merge_from(&cargo_workspace.enabled);
    }
}
