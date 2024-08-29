use anyhow;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct TabSwitcherSettings {
    pub show_icons: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct TabSwitcherSettingsContent {
    /// Whether to show file icons in the tab switcher.
    ///
    /// default: true
    pub show_icons: Option<bool>,
}

impl Settings for TabSwitcherSettings {
    const KEY: Option<&'static str> = Some("tab_switcher");

    type FileContent = TabSwitcherSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
