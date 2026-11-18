use settings::Settings;

#[derive(Clone, Debug, PartialEq)]
pub struct CommandPaletteSettings {
    pub use_command_history: bool,
}

impl Settings for CommandPaletteSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            use_command_history: content
                .command_palette
                .as_ref()
                .and_then(|settings| settings.use_command_history)
                .expect("missing default for command_palette.use_command_history"),
        }
    }
}
