use gpui::App;
use settings::Settings;

#[derive(Debug)]
pub struct CallSettings {
    pub mute_on_join: bool,
    pub share_on_join: bool,
}

impl Settings for CallSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        let call = content.calls.clone().unwrap();
        CallSettings {
            mute_on_join: call.mute_on_join.unwrap(),
            share_on_join: call.share_on_join.unwrap(),
        }
    }

    fn import_from_vscode(
        _vscode: &settings::VsCodeSettings,
        _current: &mut settings::SettingsContent,
    ) {
    }
}
