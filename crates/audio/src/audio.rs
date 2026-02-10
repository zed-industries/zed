use anyhow::{Context as _, Result};
use collections::HashMap;
use cpal::{
    DeviceId, default_host,
    traits::{DeviceTrait, HostTrait},
};
use gpui::{App, BackgroundExecutor, BorrowAppContext, Global};

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
mod non_windows_and_freebsd_deps {
    pub(super) use cpal::Sample;
    pub(super) use gpui::AsyncApp;
    pub(super) use libwebrtc::native::apm;
    pub(super) use parking_lot::Mutex;
    pub(super) use rodio::source::LimitSettings;
    pub(super) use std::sync::Arc;
}

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
use non_windows_and_freebsd_deps::*;

use rodio::{
    Decoder, DeviceSinkBuilder, MixerDeviceSink, Source,
    mixer::Mixer,
    nz,
    source::{AutomaticGainControlSettings, Buffered},
};
use settings::Settings;
use std::{
    io::Cursor, num::NonZero, path::PathBuf, str::FromStr, sync::atomic::Ordering, time::Duration,
};
use util::ResultExt;

mod audio_settings;
mod replays;
mod rodio_ext;
pub use audio_settings::AudioSettings;
pub use rodio_ext::RodioExt;

use crate::audio_settings::LIVE_SETTINGS;

// We are migrating to 16kHz sample rate from 48kHz. In the future
// once we are reasonably sure most users have upgraded we will
// remove the LEGACY parameters.
//
// We migrate to 16kHz because it is sufficient for speech and required
// by the denoiser and future Speech to Text layers.
pub const SAMPLE_RATE: NonZero<u32> = nz!(16000);
pub const CHANNEL_COUNT: NonZero<u16> = nz!(1);
pub const BUFFER_SIZE: usize = // echo canceller and livekit want 10ms of audio
    (SAMPLE_RATE.get() as usize / 100) * CHANNEL_COUNT.get() as usize;

pub const LEGACY_SAMPLE_RATE: NonZero<u32> = nz!(48000);
pub const LEGACY_CHANNEL_COUNT: NonZero<u16> = nz!(2);

pub const REPLAY_DURATION: Duration = Duration::from_secs(30);

pub fn init(cx: &mut App) {
    LIVE_SETTINGS.initialize(cx);
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Sound {
    Joined,
    GuestJoined,
    Leave,
    Mute,
    Unmute,
    StartScreenshare,
    StopScreenshare,
    AgentDone,
}

impl Sound {
    fn file(&self) -> &'static str {
        match self {
            Self::Joined => "joined_call",
            Self::GuestJoined => "guest_joined_call",
            Self::Leave => "leave_call",
            Self::Mute => "mute",
            Self::Unmute => "unmute",
            Self::StartScreenshare => "start_screenshare",
            Self::StopScreenshare => "stop_screenshare",
            Self::AgentDone => "agent_done",
        }
    }
}

pub struct Audio {
    output_handle: Option<MixerDeviceSink>,
    #[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
    pub echo_canceller: Arc<Mutex<apm::AudioProcessingModule>>,
    source_cache: HashMap<Sound, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    replays: replays::Replays,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            output_handle: Default::default(),
            #[cfg(not(any(
                all(target_os = "windows", target_env = "gnu"),
                target_os = "freebsd"
            )))]
            echo_canceller: Arc::new(Mutex::new(apm::AudioProcessingModule::new(
                true, false, false, false,
            ))),
            source_cache: Default::default(),
            replays: Default::default(),
        }
    }
}

impl Global for Audio {}

impl Audio {
    fn ensure_output_exists(&mut self, output_audio_device: Option<DeviceId>) -> Result<&Mixer> {
        #[cfg(debug_assertions)]
        log::warn!(
            "Audio does not sound correct without optimizations. Use a release build to debug audio issues"
        );

        if self.output_handle.is_none() {
            let output_handle = open_output_stream(output_audio_device)?;

            // The webrtc apm is not yet compiling for windows & freebsd
            #[cfg(not(any(
                any(all(target_os = "windows", target_env = "gnu")),
                target_os = "freebsd"
            )))]
            let echo_canceller = Arc::clone(&self.echo_canceller);

            #[cfg(not(any(
                any(all(target_os = "windows", target_env = "gnu")),
                target_os = "freebsd"
            )))]
            {
                let source = rodio::source::Zero::new(CHANNEL_COUNT, SAMPLE_RATE)
                    .inspect_buffer::<BUFFER_SIZE, _>(move |buffer| {
                        let mut buf: [i16; _] = buffer.map(|s| s.to_sample());
                        echo_canceller
                            .lock()
                            .process_reverse_stream(
                                &mut buf,
                                SAMPLE_RATE.get() as i32,
                                CHANNEL_COUNT.get().into(),
                            )
                            .expect("Audio input and output threads should not panic");
                    });
                output_handle.mixer().add(source);
            }

            #[cfg(any(
                any(all(target_os = "windows", target_env = "gnu")),
                target_os = "freebsd"
            ))]
            {
                let source = rodio::source::Zero::<f32>::new(CHANNEL_COUNT, SAMPLE_RATE);
                output_handle.mixer().add(source);
            }

            self.output_handle = Some(output_handle);
        }

        Ok(self
            .output_handle
            .as_ref()
            .map(|h| h.mixer())
            .expect("we only get here if opening the outputstream succeeded"))
    }

    pub fn save_replays(
        &self,
        executor: BackgroundExecutor,
    ) -> gpui::Task<anyhow::Result<(PathBuf, Duration)>> {
        self.replays.replays_to_tar(executor)
    }

    #[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
    pub fn open_microphone(voip_parts: VoipParts) -> anyhow::Result<impl Source> {
        let stream = open_input_stream(voip_parts.input_audio_device)?;
        let stream = stream
            .possibly_disconnected_channels_to_mono()
            .constant_samplerate(SAMPLE_RATE)
            .limit(LimitSettings::live_performance())
            .process_buffer::<BUFFER_SIZE, _>(move |buffer| {
                let mut int_buffer: [i16; _] = buffer.map(|s| s.to_sample());
                if voip_parts
                    .echo_canceller
                    .lock()
                    .process_stream(
                        &mut int_buffer,
                        SAMPLE_RATE.get() as i32,
                        CHANNEL_COUNT.get() as i32,
                    )
                    .context("livekit audio processor error")
                    .log_err()
                    .is_some()
                {
                    for (sample, processed) in buffer.iter_mut().zip(&int_buffer) {
                        *sample = (*processed).to_sample();
                    }
                }
            })
            .denoise()
            .context("Could not set up denoiser")?
            .automatic_gain_control(AutomaticGainControlSettings {
                target_level: 0.90,
                attack_time: Duration::from_secs(1),
                release_time: Duration::from_secs(0),
                absolute_max_gain: 5.0,
            })
            .periodic_access(Duration::from_millis(100), move |agc_source| {
                agc_source
                    .set_enabled(LIVE_SETTINGS.auto_microphone_volume.load(Ordering::Relaxed));
                let denoise = agc_source.inner_mut();
                denoise.set_enabled(LIVE_SETTINGS.denoise.load(Ordering::Relaxed));
            });

        let stream = if voip_parts.legacy_audio_compatible {
            stream.constant_params(LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE)
        } else {
            stream.constant_params(CHANNEL_COUNT, SAMPLE_RATE)
        };

        let (replay, stream) = stream.replayable(REPLAY_DURATION)?;
        voip_parts
            .replays
            .add_voip_stream("local microphone".to_string(), replay);

        Ok(stream)
    }

    pub fn play_voip_stream(
        source: impl rodio::Source + Send + 'static,
        speaker_name: String,
        is_staff: bool,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        let (replay_source, source) = source
            .constant_params(CHANNEL_COUNT, SAMPLE_RATE)
            .automatic_gain_control(AutomaticGainControlSettings {
                target_level: 0.90,
                attack_time: Duration::from_secs(1),
                release_time: Duration::from_secs(0),
                absolute_max_gain: 5.0,
            })
            .periodic_access(Duration::from_millis(100), move |agc_source| {
                agc_source.set_enabled(LIVE_SETTINGS.auto_speaker_volume.load(Ordering::Relaxed));
            })
            .replayable(REPLAY_DURATION)
            .expect("REPLAY_DURATION is longer than 100ms");
        let output_audio_device = AudioSettings::get_global(cx)
            .output_audio_device
            .as_ref()
            .and_then(|id| DeviceId::from_str(id).ok());

        cx.update_default_global(|this: &mut Self, _cx| {
            let output_mixer = this
                .ensure_output_exists(output_audio_device)
                .context("Could not get output mixer")?;
            output_mixer.add(source);
            if is_staff {
                this.replays.add_voip_stream(speaker_name, replay_source);
            }
            Ok(())
        })
    }

    pub fn play_sound(sound: Sound, cx: &mut App) {
        let output_audio_device = AudioSettings::get_global(cx)
            .output_audio_device
            .as_ref()
            .and_then(|id| DeviceId::from_str(id).ok());
        cx.update_default_global(|this: &mut Self, cx| {
            let source = this.sound_source(sound, cx).log_err()?;
            let output_mixer = this
                .ensure_output_exists(output_audio_device)
                .context("Could not get output mixer")
                .log_err()?;

            output_mixer.add(source);
            Some(())
        });
    }

    pub fn end_call(cx: &mut App) {
        cx.update_default_global(|this: &mut Self, _cx| {
            this.output_handle.take();
        });
    }

    fn sound_source(&mut self, sound: Sound, cx: &App) -> Result<impl Source + use<>> {
        if let Some(wav) = self.source_cache.get(&sound) {
            return Ok(wav.clone());
        }

        let path = format!("sounds/{}.wav", sound.file());
        let bytes = cx
            .asset_source()
            .load(&path)?
            .map(anyhow::Ok)
            .with_context(|| format!("No asset available for path {path}"))??
            .into_owned();
        let cursor = Cursor::new(bytes);
        let source = Decoder::new(cursor)?.buffered();

        self.source_cache.insert(sound, source.clone());

        Ok(source)
    }
}

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
pub struct VoipParts {
    echo_canceller: Arc<Mutex<apm::AudioProcessingModule>>,
    replays: replays::Replays,
    legacy_audio_compatible: bool,
    input_audio_device: Option<DeviceId>,
}

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
impl VoipParts {
    pub fn new(cx: &AsyncApp) -> anyhow::Result<Self> {
        let (apm, replays) = cx.read_default_global::<Audio, _>(|audio, _| {
            (Arc::clone(&audio.echo_canceller), audio.replays.clone())
        });
        let legacy_audio_compatible =
            AudioSettings::try_read_global(cx, |settings| settings.legacy_audio_compatible)
                .unwrap_or(true);
        let input_audio_device =
            AudioSettings::try_read_global(cx, |settings| settings.input_audio_device.clone())
                .flatten()
                .and_then(|id| DeviceId::from_str(&id).ok());

        Ok(Self {
            legacy_audio_compatible,
            echo_canceller: apm,
            replays,
            input_audio_device,
        })
    }
}

pub fn open_input_stream(
    device_id: Option<DeviceId>,
) -> anyhow::Result<rodio::microphone::Microphone> {
    let builder = rodio::microphone::MicrophoneBuilder::new();
    let builder = if let Some(id) = device_id {
        // TODO(jk): upstream patch
        // if let Some(input_device) = default_host().device_by_id(id) {
        //     builder.device(input_device);
        // }
        let mut found = None;
        for input in rodio::microphone::available_inputs()? {
            if input.clone().into_inner().id()? == id {
                found = Some(builder.device(input));
                break;
            }
        }
        found.unwrap_or_else(|| builder.default_device())?
    } else {
        builder.default_device()?
    };
    let stream = builder
        .default_config()?
        .prefer_sample_rates([
            SAMPLE_RATE,
            SAMPLE_RATE.saturating_mul(rodio::nz!(2)),
            SAMPLE_RATE.saturating_mul(rodio::nz!(3)),
            SAMPLE_RATE.saturating_mul(rodio::nz!(4)),
        ])
        .prefer_channel_counts([rodio::nz!(1), rodio::nz!(2), rodio::nz!(3), rodio::nz!(4)])
        .prefer_buffer_sizes(512..)
        .open_stream()?;
    log::info!("Opened microphone: {:?}", stream.config());
    Ok(stream)
}

pub fn open_output_stream(device_id: Option<DeviceId>) -> anyhow::Result<MixerDeviceSink> {
    let output_handle = if let Some(id) = device_id {
        if let Some(device) = default_host().device_by_id(&id) {
            DeviceSinkBuilder::from_device(device)?.open_stream()
        } else {
            DeviceSinkBuilder::open_default_sink()
        }
    } else {
        DeviceSinkBuilder::open_default_sink()
    };
    let mut output_handle = output_handle.context("Could not open output stream")?;
    output_handle.log_on_drop(false);
    log::info!("Output stream: {:?}", output_handle);
    Ok(output_handle)
}
