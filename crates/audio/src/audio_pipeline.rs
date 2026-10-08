use anyhow::{Context as _, Result};
use collections::HashMap;
use cpal::{
    DeviceDescription, DeviceId, Error, ErrorKind, default_host,
    traits::{DeviceTrait, HostTrait},
};
use futures::channel::oneshot;
use gpui::{App, AsyncApp, BorrowAppContext, Global, Task};

pub(super) use cpal::Sample;

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Source, mixer::Mixer, source::Buffered};
use settings::Settings;
use std::io::Cursor;
use util::ResultExt;

mod echo_canceller;
use echo_canceller::EchoCanceller;
mod rodio_ext;
pub use crate::audio_settings::AudioSettings;
pub use rodio_ext::RodioExt;

use crate::Sound;

use super::{CHANNEL_COUNT, SAMPLE_RATE};
pub const BUFFER_SIZE: usize = // echo canceller and livekit want 10ms of audio
    (SAMPLE_RATE.get() as usize / 100) * CHANNEL_COUNT.get() as usize;

pub fn init(_cx: &mut App) {}

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
    output_error_task: Option<Task<()>>,
    pub echo_canceller: EchoCanceller,
    source_cache: HashMap<Sound, Buffered<Decoder<Cursor<Vec<u8>>>>>,
}

impl Global for Audio {}

impl Audio {
    fn ensure_output_exists(
        &mut self,
        output_audio_device: Option<DeviceId>,
        cx: &mut App,
    ) -> Result<&Mixer> {
        #[cfg(debug_assertions)]
        log::warn!(
            "Audio does not sound correct without optimizations. Use a release build to debug audio issues"
        );

        if self.output.is_none() {
            let (error_callback, error_task) = output_error_handler(cx);
            let output = open_output_stream(
                output_audio_device,
                self.echo_canceller.clone(),
                error_callback,
            )?;
            self.output = Some(output);
            self.output_error_task = Some(error_task);
        }

        Ok(self
            .output
            .as_ref()
            .map(|(_, mixer)| mixer)
            .expect("we only get here if opening the outputstream succeeded"))
    }

    pub fn play_sound(sound: Sound, cx: &mut App) {
        let output_audio_device = AudioSettings::get_global(cx).output_audio_device.clone();
        cx.update_default_global(|this: &mut Self, cx| {
            let source = this.sound_source(sound, cx).log_err()?;
            let output_mixer = this
                .ensure_output_exists(output_audio_device, cx)
                .context("Could not get output mixer")
                .log_err()?;

            output_mixer.add(source);
            Some(())
        });
    }

    pub fn end_call(cx: &mut App) {
        cx.update_default_global(|audio: &mut Self, _cx| {
            audio.output_error_task.take();
            audio.output.take();
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
        .with_buffer_size(cpal::BufferSize::Default)
        .open_stream()
        .context("Could not open output stream")
}

pub fn open_output_stream(
    device_id: Option<DeviceId>,
    mut echo_canceller: EchoCanceller,
    error_callback: impl FnMut(Error) + Send + 'static,
) -> anyhow::Result<(MixerDeviceSink, Mixer)> {
    let device = resolve_device(device_id.as_ref(), false)?;
    let mut output_handle = DeviceSinkBuilder::from_device(device)?
        .with_buffer_size(cpal::BufferSize::Default)
        .with_error_callback(error_callback)
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

fn output_error_handler(cx: &mut App) -> (impl FnMut(Error) + Send + 'static + use<>, Task<()>) {
    let (sender, receiver) = oneshot::channel();
    let mut sender = Some(sender);
    let callback = move |error: Error| {
        if matches!(
            error.kind(),
            ErrorKind::Xrun | ErrorKind::DeviceChanged | ErrorKind::RealtimeDenied
        ) {
            return;
        }
        if let Some(sender) = sender.take() {
            sender.send(error).ok();
        }
    };
    let task = cx.spawn(async move |cx| {
        if let Ok(error) = receiver.await {
            log::error!("Audio output stream failed: {error}");
            cx.update(Audio::end_call);
        }
    });
    (callback, task)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn test_output_error_handler(cx: &mut TestAppContext) {
        for error_kind in [
            ErrorKind::DeviceNotAvailable,
            ErrorKind::StreamInvalidated,
            ErrorKind::BackendError,
        ] {
            let mut callback = cx.update(install_output_error_handler);
            for recoverable_kind in [
                ErrorKind::Xrun,
                ErrorKind::DeviceChanged,
                ErrorKind::RealtimeDenied,
            ] {
                callback(Error::new(recoverable_kind));
                cx.run_until_parked();
                cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_some()));
            }

            callback(Error::new(error_kind));
            callback(Error::new(error_kind));
            cx.run_until_parked();
            cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_none()));
        }
    }

    #[gpui::test]
    fn test_output_error_handler_cancellation(cx: &mut TestAppContext) {
        for error_before_close in [false, true] {
            let mut old_callback = cx.update(install_output_error_handler);
            if error_before_close {
                old_callback(Error::new(ErrorKind::DeviceNotAvailable));
            }
            cx.update(Audio::end_call);

            let mut callback = cx.update(install_output_error_handler);
            old_callback(Error::new(ErrorKind::DeviceNotAvailable));
            cx.run_until_parked();
            cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_some()));

            callback(Error::new(ErrorKind::StreamInvalidated));
            cx.run_until_parked();
            cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_none()));
        }

        let callback = cx.update(install_output_error_handler);
        drop(callback);
        cx.run_until_parked();
        cx.update(|cx| assert!(cx.global::<Audio>().output_error_task.is_some()));
        cx.update(Audio::end_call);
    }

    fn install_output_error_handler(cx: &mut App) -> impl FnMut(Error) + Send + 'static + use<> {
        let (callback, task) = output_error_handler(cx);
        cx.update_default_global(|audio: &mut Audio, _cx| {
            audio.output_error_task = Some(task);
        });
        callback
    }
}
