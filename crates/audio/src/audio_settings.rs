use std::str::FromStr;

use cpal::DeviceId;
use settings::{RegisterSetting, Settings};

#[derive(Clone, Debug, RegisterSetting)]
pub struct AudioSettings {
    /// Select specific output audio device.
    pub output_audio_device: Option<DeviceId>,
    /// Select specific input audio device.
    pub input_audio_device: Option<DeviceId>,
}

/// Configuration of audio in Zed
impl Settings for AudioSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let audio = &content.audio.as_ref().unwrap();
        AudioSettings {
            output_audio_device: audio
                .output_audio_device
                .as_ref()
                .and_then(|x| x.0.as_ref().and_then(|id| DeviceId::from_str(&id).ok())),
            input_audio_device: audio
                .input_audio_device
                .as_ref()
                .and_then(|x| x.0.as_ref().and_then(|id| DeviceId::from_str(&id).ok())),
        }
    }
}
