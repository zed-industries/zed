use std::{
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
};

use cpal::DeviceId;
use gpui::App;
use settings::{RegisterSetting, Settings, SettingsStore};

#[derive(Clone, Debug, RegisterSetting)]
pub struct AudioSettings {
    /// Automatically increase or decrease you microphone's volume. This affects how
    /// loud you sound to others.
    ///
    /// Recommended: off (default)
    /// Microphones are too quite in zed, until everyone is on experimental
    /// audio and has auto speaker volume on this will make you very loud
    /// compared to other speakers.
    pub auto_microphone_volume: bool,
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
            auto_microphone_volume: audio.auto_microphone_volume.unwrap(),
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

/// See docs on [LIVE_SETTINGS]
pub struct LiveSettings {
    pub auto_microphone_volume: AtomicBool,
}

impl LiveSettings {
    pub(crate) fn initialize(&self, cx: &mut App) {
        cx.observe_global::<SettingsStore>(move |cx| {
            LIVE_SETTINGS.auto_microphone_volume.store(
                AudioSettings::get_global(cx).auto_microphone_volume,
                Ordering::Relaxed,
            );
        })
        .detach();

        let init_settings = AudioSettings::get_global(cx);
        LIVE_SETTINGS
            .auto_microphone_volume
            .store(init_settings.auto_microphone_volume, Ordering::Relaxed);
    }
}

/// Allows access to settings from the audio thread. Updated by
/// observer of SettingsStore. Needed because audio playback and recording are
/// real time and must each run in a dedicated OS thread, therefore we can not
/// use the background executor.
pub static LIVE_SETTINGS: LiveSettings = LiveSettings {
    auto_microphone_volume: AtomicBool::new(true),
};
