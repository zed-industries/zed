use std::sync::atomic::{AtomicBool, Ordering};

use gpui::App;
use settings::{Settings, SettingsStore};

#[derive(Clone, Debug)]
pub struct AudioSettings {
    /// Opt into the new audio system.
    pub rodio_audio: bool, // default is false
    /// Requires 'rodio_audio: true'
    ///
    /// Use the new audio systems automatic gain control for your microphone.
    /// This affects how loud you sound to others.
    pub control_input_volume: bool,
    /// Requires 'rodio_audio: true'
    ///
    /// Use the new audio systems automatic gain control on everyone in the
    /// call. This makes call members who are too quite louder and those who are
    /// too loud quieter. This only affects how things sound for you.
    pub control_output_volume: bool,
}

/// Configuration of audio in Zed
impl Settings for AudioSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        let audio = &content.audio.as_ref().unwrap();
        AudioSettings {
            control_input_volume: audio.control_input_volume.unwrap(),
            control_output_volume: audio.control_output_volume.unwrap(),
            rodio_audio: audio.rodio_audio.unwrap(),
        }
    }

    fn import_from_vscode(
        _vscode: &settings::VsCodeSettings,
        _current: &mut settings::SettingsContent,
    ) {
    }
}

/// See docs on [LIVE_SETTINGS]
pub(crate) struct LiveSettings {
    pub(crate) control_input_volume: AtomicBool,
    pub(crate) control_output_volume: AtomicBool,
}

impl LiveSettings {
    pub(crate) fn initialize(&self, cx: &mut App) {
        cx.observe_global::<SettingsStore>(move |cx| {
            LIVE_SETTINGS.control_input_volume.store(
                AudioSettings::get_global(cx).control_input_volume,
                Ordering::Relaxed,
            );
            LIVE_SETTINGS.control_output_volume.store(
                AudioSettings::get_global(cx).control_output_volume,
                Ordering::Relaxed,
            );
        })
        .detach();

        let init_settings = AudioSettings::get_global(cx);
        LIVE_SETTINGS
            .control_input_volume
            .store(init_settings.control_input_volume, Ordering::Relaxed);
        LIVE_SETTINGS
            .control_output_volume
            .store(init_settings.control_output_volume, Ordering::Relaxed);
    }
}

/// Allows access to settings from the audio thread. Updated by
/// observer of SettingsStore. Needed because audio playback and recording are
/// real time and must each run in a dedicated OS thread, therefore we can not
/// use the background executor.
pub(crate) static LIVE_SETTINGS: LiveSettings = LiveSettings {
    control_input_volume: AtomicBool::new(true),
    control_output_volume: AtomicBool::new(true),
};
