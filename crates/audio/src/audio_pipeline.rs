use anyhow::{Context as _, Result};
use collections::HashMap;
use cpal::{
    DeviceDescription, DeviceId, default_host,
    traits::{DeviceTrait, HostTrait},
};
use gpui::{App, AsyncApp, BackgroundExecutor, BorrowAppContext, Global};

pub(super) use cpal::Sample;
pub(super) use rodio::source::LimitSettings;

use rodio::{
    Decoder, DeviceSinkBuilder, MixerDeviceSink, Source,
    mixer::Mixer,
    source::{AutomaticGainControlSettings, Buffered},
};
use settings::Settings;
use std::{io::Cursor, path::PathBuf, sync::atomic::Ordering, time::Duration};
use util::ResultExt;

mod echo_canceller;
use echo_canceller::EchoCanceller;
mod replays;
mod rodio_ext;
pub use crate::audio_settings::AudioSettings;
pub use rodio_ext::RodioExt;

use crate::audio_settings::LIVE_SETTINGS;

use crate::Sound;

use super::{CHANNEL_COUNT, SAMPLE_RATE};
pub const BUFFER_SIZE: usize = // echo canceller and livekit want 10ms of audio
    (SAMPLE_RATE.get() as usize / 100) * CHANNEL_COUNT.get() as usize;

pub fn init(cx: &mut App) {
    LIVE_SETTINGS.initialize(cx);
}

// TODO(jk): this is currently cached only once - we should observe and react instead
pub fn ensure_devices_initialized(cx: &mut App) {
    if cx.has_global::<AvailableAudioDevices>() {
        return;
    }
    cx.default_global::<AvailableAudioDevices>();
    let task = cx
        .background_executor()
        .spawn(async move { get_available_audio_devices() });
    cx.spawn(async move |cx: &mut AsyncApp| {
        let devices = task.await;
        cx.update(|cx| cx.set_global(AvailableAudioDevices(devices)));
        cx.refresh();
    })
    .detach();
}

#[derive(Default)]
pub struct Audio {
    output: Option<(MixerDeviceSink, Mixer)>,
    pub echo_canceller: EchoCanceller,
    source_cache: HashMap<Sound, Buffered<Decoder<Cursor<Vec<u8>>>>>,
    replays: replays::Replays,
}

impl Global for Audio {}

impl Audio {
    fn ensure_output_exists(&mut self, output_audio_device: Option<DeviceId>) -> Result<&Mixer> {
        #[cfg(debug_assertions)]
        log::warn!(
            "Audio does not sound correct without optimizations. Use a release build to debug audio issues"
        );

        if self.output.is_none() {
            let (output_handle, output_mixer) =
                open_output_stream(output_audio_device, self.echo_canceller.clone())?;
            self.output = Some((output_handle, output_mixer));
        }

        Ok(self
            .output
            .as_ref()
            .map(|(_, mixer)| mixer)
            .expect("we only get here if opening the outputstream succeeded"))
    }

    pub fn save_replays(
        &self,
        executor: BackgroundExecutor,
    ) -> gpui::Task<anyhow::Result<(PathBuf, Duration)>> {
        self.replays.replays_to_tar(executor)
    }

    pub fn open_microphone(mut voip_parts: VoipParts) -> anyhow::Result<impl Source> {
        let stream = open_input_stream(voip_parts.input_audio_device)?;
        let stream = stream
            .possibly_disconnected_channels_to_mono()
            .constant_params(CHANNEL_COUNT, SAMPLE_RATE)
            .process_buffer::<BUFFER_SIZE, _>(move |buffer| {
                let mut int_buffer: [i16; _] = buffer.map(|s| s.to_sample());
                if voip_parts
                    .echo_canceller
                    .process_stream(&mut int_buffer)
                    .log_err()
                    .is_some()
                {
                    for (sample, processed) in buffer.iter_mut().zip(&int_buffer) {
                        *sample = (*processed).to_sample();
                    }
                }
            })
            .limit(LimitSettings::live_performance())
            .automatic_gain_control(AutomaticGainControlSettings {
                target_level: 0.90,
                attack_time: Duration::from_secs(1),
                release_time: Duration::from_secs(0),
                absolute_max_gain: 5.0,
            })
            .periodic_access(Duration::from_millis(100), move |agc_source| {
                agc_source
                    .set_enabled(LIVE_SETTINGS.auto_microphone_volume.load(Ordering::Relaxed));
                let _ = LIVE_SETTINGS.denoise; // TODO(audio: re-introduce de-noising
            });

        let (replay, stream) = stream.replayable(crate::REPLAY_DURATION)?;
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
            .automatic_gain_control(AutomaticGainControlSettings {
                target_level: 0.90,
                attack_time: Duration::from_secs(1),
                release_time: Duration::from_secs(0),
                absolute_max_gain: 5.0,
            })
            .periodic_access(Duration::from_millis(100), move |agc_source| {
                agc_source.set_enabled(LIVE_SETTINGS.auto_speaker_volume.load(Ordering::Relaxed));
            })
            .replayable(crate::REPLAY_DURATION)
            .expect("REPLAY_DURATION is longer than 100ms");
        let output_audio_device = AudioSettings::get_global(cx).output_audio_device.clone();

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
        let output_audio_device = AudioSettings::get_global(cx).output_audio_device.clone();
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
            this.output.take();
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

pub struct VoipParts {
    echo_canceller: EchoCanceller,
    replays: replays::Replays,
    input_audio_device: Option<DeviceId>,
}

impl VoipParts {
    pub fn new(cx: &AsyncApp) -> anyhow::Result<Self> {
        let (apm, replays) = cx.read_default_global::<Audio, _>(|audio, _| {
            (audio.echo_canceller.clone(), audio.replays.clone())
        });
        let input_audio_device =
            AudioSettings::try_read_global(cx, |settings| settings.input_audio_device.clone())
                .flatten();

        Ok(Self {
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

pub fn resolve_device(device_id: Option<&DeviceId>, input: bool) -> anyhow::Result<cpal::Device> {
    if let Some(id) = device_id {
        if let Some(device) = default_host().device_by_id(id) {
            return Ok(device);
        }
        log::warn!("Selected audio device not found, falling back to default");
    }
    if input {
        default_host()
            .default_input_device()
            .context("no audio input device available")
    } else {
        default_host()
            .default_output_device()
            .context("no audio output device available")
    }
}

pub fn open_test_output(device_id: Option<DeviceId>) -> anyhow::Result<MixerDeviceSink> {
    let device = resolve_device(device_id.as_ref(), false)?;
    DeviceSinkBuilder::from_device(device)?
        .open_stream()
        .context("Could not open output stream")
}

pub fn open_output_stream(
    device_id: Option<DeviceId>,
    mut echo_canceller: EchoCanceller,
) -> anyhow::Result<(MixerDeviceSink, Mixer)> {
    let device = resolve_device(device_id.as_ref(), false)?;
    let mut output_handle = DeviceSinkBuilder::from_device(device)?
        .open_stream()
        .context("Could not open output stream")?;
    output_handle.log_on_drop(false);
    log::info!("Output stream: {:?}", output_handle);

    let (output_mixer, source) = rodio::mixer::mixer(CHANNEL_COUNT, SAMPLE_RATE);
    // otherwise the mixer ends as it's empty
    output_mixer.add(rodio::source::Zero::new(CHANNEL_COUNT, SAMPLE_RATE));
    let echo_cancelling_source = source // apply echo cancellation just before output
        .inspect_buffer::<BUFFER_SIZE, _>(move |buffer| {
            let mut buf: [i16; _] = buffer.map(|s| s.to_sample());
            echo_canceller.process_reverse_stream(&mut buf)
        });
    output_handle.mixer().add(echo_cancelling_source);

    Ok((output_handle, output_mixer))
}

#[derive(Clone, Debug)]
pub struct AudioDeviceInfo {
    pub id: DeviceId,
    pub desc: DeviceDescription,
}

impl AudioDeviceInfo {
    pub fn matches_input(&self, is_input: bool) -> bool {
        if is_input {
            self.desc.supports_input()
        } else {
            self.desc.supports_output()
        }
    }

    pub fn matches(&self, id: &DeviceId, is_input: bool) -> bool {
        &self.id == id && self.matches_input(is_input)
    }
}

impl std::fmt::Display for AudioDeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.desc.name(), self.id)
    }
}

fn get_available_audio_devices() -> Vec<AudioDeviceInfo> {
    let Some(devices) = default_host().devices().ok() else {
        return Vec::new();
    };
    devices
        .filter_map(|device| {
            let id = device.id().ok()?;
            let desc = device.description().ok()?;
            Some(AudioDeviceInfo { id, desc })
        })
        .collect()
}

#[derive(Default, Clone, Debug)]
pub struct AvailableAudioDevices(pub Vec<AudioDeviceInfo>);

impl Global for AvailableAudioDevices {}
