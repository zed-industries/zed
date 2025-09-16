use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};
use util::MergeFrom;

#[derive(Deserialize, Debug)]
pub struct CallSettings {
    pub mute_on_join: bool,
    pub share_on_join: bool,
}

impl Settings for CallSettings {
    fn from_defaults(content: &settings::SettingsContent, cx: &mut App) -> Self {
        let call = content.call.unwrap();
        CallSettings {
            mute_on_join: call.mute_on_join.unwrap(),
            share_on_join: call.share_on_join.unwrap(),
        }
    }

    fn refine(&mut self, content: &settings::SettingsContent, cx: &mut App) {
        if let Some(call) = content.call.clone() {
            self.mute_on_join.merge_from(call.mute_on_join);
            self.share_on_join.merge_from(call.share_on_join);
        }
    }

    fn import_from_vscode(
        _vscode: &settings::VsCodeSettings,
        _current: &settings::SettingsContent,
    ) {
    }
}
