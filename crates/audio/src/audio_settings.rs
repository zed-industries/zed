use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsStore, SettingsUi};

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug, SettingsUi)]
pub struct AudioSettings {
    /// Opt into the new audio system.
    #[serde(rename = "experimental.rodio_audio", default)]
    pub rodio_audio: bool, // default is false
    /// Requires 'rodio_audio: true'
    ///
    /// Use the new audio systems automatic gain control for your microphone.
    /// This affects how loud you sound to others.
    #[serde(rename = "experimental.control_input_volume", default)]
    pub control_input_volume: bool,
    /// Requires 'rodio_audio: true'
    ///
    /// Use the new audio systems automatic gain control on everyone in the
    /// call. This makes call members who are too quite louder and those who are
    /// too loud quieter. This only affects how things sound for you.
    #[serde(rename = "experimental.control_output_volume", default)]
    pub control_output_volume: bool,
}

/// Configuration of audio in Zed.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug, SettingsUi, SettingsKey)]
#[serde(default)]
#[settings_key(key = "audio")]
pub struct AudioSettingsContent {
    /// Opt into the new audio system.
    #[serde(rename = "experimental.rodio_audio", default)]
    pub rodio_audio: bool, // default is false
    /// Requires 'rodio_audio: true'
    ///
    /// Use the new audio systems automatic gain control for your microphone.
    /// This affects how loud you sound to others.
    #[serde(rename = "experimental.control_input_volume", default)]
    pub control_input_volume: bool,
    /// Requires 'rodio_audio: true'
    ///
    /// Use the new audio systems automatic gain control on everyone in the
    /// call. This makes call members who are too quite louder and those who are
    /// too loud quieter. This only affects how things sound for you.
    #[serde(rename = "experimental.control_output_volume", default)]
    pub control_output_volume: bool,
}

impl Settings for AudioSettings {
    type FileContent = AudioSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _cx: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
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
