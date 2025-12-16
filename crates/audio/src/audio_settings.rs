use std::sync::atomic::{AtomicBool, Ordering};

use gpui::App;
use settings::{RegisterSetting, Settings, SettingsStore};

#[derive(Clone, Debug, RegisterSetting)]
pub struct AudioSettings {
    /// Opt into the new audio system.
    ///
    /// You need to rejoin a call for this setting to apply
    pub rodio_audio: bool, // default is false
    /// Requires 'rodio_audio: true'
    ///
    /// Automatically increase or decrease you microphone's volume. This affects how
    /// loud you sound to others.
    ///
    /// Recommended: off (default)
    /// Microphones are too quite in zed, until everyone is on experimental
    /// audio and has auto speaker volume on this will make you very loud
    /// compared to other speakers.
    pub auto_microphone_volume: bool,
    /// Requires 'rodio_audio: true'
    ///
    /// Automatically increate or decrease the volume of other call members.
    /// This only affects how things sound for you.
    pub auto_speaker_volume: bool,
    /// Requires 'rodio_audio: true'
    ///
    /// Remove background noises. Works great for typing, cars, dogs, AC. Does
    /// not work well on music.
    pub denoise: bool,
    /// Requires 'rodio_audio: true'
    ///
    /// Use audio parameters compatible with the previous versions of
    /// experimental audio and non-experimental audio. When this is false you
    /// will sound strange to anyone not on the latest experimental audio. In
    /// the future we will migrate by setting this to false
    ///
    /// You need to rejoin a call for this setting to apply
    pub legacy_audio_compatible: bool,
}

/// Configuration of audio in Zed
impl Settings for AudioSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let audio = &content.audio.as_ref().unwrap();
        AudioSettings {
            rodio_audio: audio.rodio_audio.unwrap(),
            auto_microphone_volume: audio.auto_microphone_volume.unwrap(),
            auto_speaker_volume: audio.auto_speaker_volume.unwrap(),
            denoise: audio.denoise.unwrap(),
            legacy_audio_compatible: audio.legacy_audio_compatible.unwrap(),
        }
    }
}

/// See docs on [LIVE_SETTINGS]
pub(crate) struct LiveSettings {
    pub(crate) auto_microphone_volume: AtomicBool,
    pub(crate) auto_speaker_volume: AtomicBool,
    pub(crate) denoise: AtomicBool,
}

impl LiveSettings {
    pub(crate) fn initialize(&self, cx: &mut App) {
        cx.observe_global::<SettingsStore>(move |cx| {
            LIVE_SETTINGS.auto_microphone_volume.store(
                AudioSettings::get_global(cx).auto_microphone_volume,
                Ordering::Relaxed,
            );
            LIVE_SETTINGS.auto_speaker_volume.store(
                AudioSettings::get_global(cx).auto_speaker_volume,
                Ordering::Relaxed,
            );

            let denoise_enabled = AudioSettings::get_global(cx).denoise;
            #[cfg(debug_assertions)]
            {
                static DENOISE_WARNING_SEND: AtomicBool = AtomicBool::new(false);
                if denoise_enabled && !DENOISE_WARNING_SEND.load(Ordering::Relaxed) {
                    DENOISE_WARNING_SEND.store(true, Ordering::Relaxed);
                    log::warn!("Denoise does not work on debug builds, not enabling")
                }
            }
            #[cfg(not(debug_assertions))]
            LIVE_SETTINGS
                .denoise
                .store(denoise_enabled, Ordering::Relaxed);
        })
        .detach();

        let init_settings = AudioSettings::get_global(cx);
        LIVE_SETTINGS
            .auto_microphone_volume
            .store(init_settings.auto_microphone_volume, Ordering::Relaxed);
        LIVE_SETTINGS
            .auto_speaker_volume
            .store(init_settings.auto_speaker_volume, Ordering::Relaxed);
        let denoise_enabled = AudioSettings::get_global(cx).denoise;
        #[cfg(debug_assertions)]
        if denoise_enabled {
            log::warn!("Denoise does not work on debug builds, not enabling")
        }
        #[cfg(not(debug_assertions))]
        LIVE_SETTINGS
            .denoise
            .store(denoise_enabled, Ordering::Relaxed);
    }
}

/// Allows access to settings from the audio thread. Updated by
/// observer of SettingsStore. Needed because audio playback and recording are
/// real time and must each run in a dedicated OS thread, therefore we can not
/// use the background executor.
pub(crate) static LIVE_SETTINGS: LiveSettings = LiveSettings {
    auto_microphone_volume: AtomicBool::new(true),
    auto_speaker_volume: AtomicBool::new(true),
    denoise: AtomicBool::new(true),
};
