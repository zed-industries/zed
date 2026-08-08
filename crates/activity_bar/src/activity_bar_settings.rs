use settings::{RegisterSetting, Settings};

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct ActivityBarSettings {
    pub show: bool,
}

impl Settings for ActivityBarSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let activity_bar = content.activity_bar.clone().unwrap();
        Self {
            show: activity_bar.show.unwrap(),
        }
    }
}
