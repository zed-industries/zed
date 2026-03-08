use settings::{RegisterSetting, Settings};

#[derive(Debug, RegisterSetting)]
pub struct CallSettings {
    pub mute_on_join: bool,
    pub share_on_join: bool,
    pub play_incoming_call_ring: bool,
}

impl Settings for CallSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let call = content.calls.clone().unwrap();
        CallSettings {
            mute_on_join: call.mute_on_join.unwrap(),
            share_on_join: call.share_on_join.unwrap(),
            play_incoming_call_ring: call.play_incoming_call_ring.unwrap(),
        }
    }
}
