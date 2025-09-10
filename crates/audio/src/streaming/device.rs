use anyhow::{Context, Result};
use log::{debug, info};
use rodio::{
    ChannelCount, SampleRate,
    cpal::{
        self, BufferSize, Device, SampleFormat, StreamConfig,
        SupportedBufferSize::{Range, Unknown},
        SupportedStreamConfig, SupportedStreamConfigRange,
        traits::{DeviceTrait, HostTrait},
    },
};

use super::AudioFormat;
use super::DURATION_20MS;

#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// The input device to use.
    pub input_device: Option<String>,
    /// The output device to use.
    pub output_device: Option<String>,
    /// If true, audio processing with echo cancellation is enabled.
    pub processing_enabled: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let input_device = std::env::var("CALLME_INPUT_DEVICE").ok();
        #[cfg(target_arch = "wasm32")]
        let input_device = None;

        #[cfg(not(target_arch = "wasm32"))]
        let output_device = std::env::var("CALLME_OUTPUT_DEVICE").ok();
        #[cfg(target_arch = "wasm32")]
        let output_device = None;

        Self {
            input_device,
            output_device,
            processing_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Capture,
    Playback,
}

pub fn list_devices() -> Result<Devices> {
    let host = cpal::default_host();
    let input = host
        .input_devices()?
        .filter_map(|x| x.name().ok())
        .collect();
    let output = host
        .output_devices()?
        .filter_map(|x| x.name().ok())
        .collect();
    Ok(Devices { input, output })
}

#[derive(Debug, Default)]
pub struct Devices {
    pub input: Vec<String>,
    pub output: Vec<String>,
}

pub fn find_device(host: &cpal::Host, direction: Direction, name: Option<&str>) -> Result<Device> {
    let iter = || match direction {
        Direction::Capture => host.input_devices(),
        Direction::Playback => host.output_devices(),
    };
    let default = || {
        // On linux, prefer the `pipewire` device, if available.
        #[cfg(target_os = "linux")]
        if let Some(device) = iter()?.find(|x| x.name().ok().as_deref() == Some("pipewire")) {
            return anyhow::Ok(Some(device));
        };

        let default_device = match direction {
            Direction::Capture => host.default_input_device(),
            Direction::Playback => host.default_output_device(),
        };

        let default_device = match default_device {
            Some(device) => Some(device),
            None => iter()?.next(),
        };
        anyhow::Ok(default_device)
    };

    let device = match &name {
        Some(device) => iter()?.find(|x| x.name().map(|y| &y == device).unwrap_or(false)),
        None => default()?,
    };
    device.with_context(|| {
        format!(
            "could not find input audio device `{}`",
            name.unwrap_or("default")
        )
    })
}

#[derive(Debug)]
pub struct StreamConfigWithFormat {
    pub sample_format: SampleFormat,
    pub config: StreamConfig,
}

impl StreamConfigWithFormat {
    fn new(config: SupportedStreamConfig, ideal_buffer_size: u32) -> Self {
        let sample_format = config.sample_format();
        let buffer_size = match config.buffer_size() {
            Range { min, max } => BufferSize::Fixed(ideal_buffer_size.clamp(*min, *max)),
            Unknown => BufferSize::Default,
        };
        let config = StreamConfig {
            channels: config.channels(),
            sample_rate: config.sample_rate(),
            buffer_size,
        };
        Self {
            sample_format,
            config,
        }
    }

    pub fn audio_format(&self) -> AudioFormat {
        AudioFormat {
            sample_rate: SampleRate::new(self.config.sample_rate.0).unwrap(),
            channel_count: ChannelCount::new(self.config.channels).unwrap(),
        }
    }
}

pub fn find_input_stream_config(
    device: &Device,
    format: &AudioFormat,
) -> Result<StreamConfigWithFormat> {
    let d = device.name().unwrap();
    debug!("find capture stream config for device {d} and format {format:?}");
    let mut supported_configs: Vec<_> = device
        .supported_input_configs()
        .with_context(|| format!("failed to get supported stream configs for audio device `{d}`"))?
        .collect();

    let config = if !supported_configs.is_empty() {
        supported_configs.sort_by(|a, b| cmp_stream_format(format, a, b).reverse());
        let config_range = supported_configs[0];
        debug!("selected capture stream config range: {config_range:?}");
        config_range
            .try_with_sample_rate(cpal::SampleRate(format.sample_rate.get()))
            .unwrap_or_else(|| config_range.with_max_sample_rate())
    } else {
        info!("no supported configs available, use default input config");
        device.default_input_config().with_context(|| {
            format!("failed to get default stream config for audio device `{d}`")
        })?
    };

    let ideal_buffer_size = format.sample_count(DURATION_20MS) as u32;
    info!("selected capture stream config: {config:?}");
    Ok(StreamConfigWithFormat::new(config, ideal_buffer_size))
}

pub fn find_output_stream_config(
    device: &Device,
    format: &AudioFormat,
) -> Result<StreamConfigWithFormat> {
    let d = device.name().unwrap();
    debug!("find playback stream config for device {d} and format {format:?}");
    let mut supported_configs: Vec<_> = device
        .supported_output_configs()
        .with_context(|| format!("failed to get supported stream configs for audio device `{d}`"))?
        .collect();

    let config = if !supported_configs.is_empty() {
        supported_configs.sort_by(|a, b| cmp_stream_format(format, a, b).reverse());
        let config_range = supported_configs[0];
        debug!("selected playback stream config range: {config_range:?}");
        config_range
            .try_with_sample_rate(cpal::SampleRate(format.sample_rate.get()))
            .unwrap_or_else(|| config_range.with_max_sample_rate())
    } else {
        info!("no supported configs available, use default output config");
        device.default_output_config().with_context(|| {
            format!("failed to get default stream config for audio device `{d}`")
        })?
    };
    info!("selected playback stream config: {config:?}");

    let ideal_buffer_size = format.sample_count(DURATION_20MS) as u32;
    Ok(StreamConfigWithFormat::new(config, ideal_buffer_size))
}

fn cmp_stream_format(
    format: &AudioFormat,
    a: &SupportedStreamConfigRange,
    b: &SupportedStreamConfigRange,
) -> std::cmp::Ordering {
    use cpal::SupportedBufferSize::{Range, Unknown};
    use std::cmp::Ordering::{Equal, Greater, Less};

    let is_perfect = |x: &SupportedStreamConfigRange| {
        x.channels() == format.channel_count.get()
            && x.sample_format() == SampleFormat::F32
            && x.try_with_sample_rate(cpal::SampleRate(format.sample_rate.get()))
                .is_some()
    };
    // check if one of the configs is our desired config.
    let a_is_perfect = is_perfect(a);
    let b_is_perfect = is_perfect(b);
    let cmp = a_is_perfect.cmp(&b_is_perfect);
    // if only one supports the desired config, use that.
    if cmp != Equal {
        return cmp;
    }
    // if both support the desired config, use the one with the smaller buffer size.
    if a_is_perfect {
        return match (a.buffer_size(), b.buffer_size()) {
            (Range { min: a, .. }, Range { min: b, .. }) => a.cmp(b).reverse(),
            (Range { .. }, _) => Greater,
            (Unknown, Range { .. }) => Less,
            (Unknown, Unknown) => Equal,
        };
    }

    // if none, support the desired config, first look for the correct channel count, then for the
    // desired sample format, then for the desired sample rate.

    // first: get a config with the correct number of channels.
    let cmp_channel_count = (a.channels() == format.channel_count.get())
        .cmp(&(b.channels() == format.channel_count.get()));
    if cmp_channel_count != Equal {
        return cmp_channel_count;
    }

    // second: get the desired sample format, or one of the "good ones"
    let cmp_sample_format =
        (a.sample_format() == SampleFormat::F32).cmp(&(b.sample_format() == SampleFormat::F32));
    if cmp_sample_format != Equal {
        return cmp_sample_format;
    }
    let cmp_sample_format =
        (a.sample_format() == SampleFormat::I16).cmp(&(b.sample_format() == SampleFormat::I16));
    if cmp_sample_format != Equal {
        return cmp_sample_format;
    }
    let cmp_sample_format =
        (a.sample_format() == SampleFormat::U16).cmp(&(b.sample_format() == SampleFormat::U16));
    if cmp_sample_format != Equal {
        return cmp_sample_format;
    }

    // third: get the desired sample rate
    let cmp_sample_rate = (a
        .try_with_sample_rate(cpal::SampleRate(format.sample_rate.get()))
        .is_some())
    .cmp(
        &(b.try_with_sample_rate(cpal::SampleRate(format.sample_rate.get()))
            .is_some()),
    );
    if cmp_sample_rate != Equal {
        return cmp_sample_rate;
    }

    // forth: support the smaller buffer size
    match (a.buffer_size(), b.buffer_size()) {
        (Range { min: a, .. }, Range { min: b, .. }) => a.cmp(b).reverse(),
        (Range { .. }, _) => Greater,
        (Unknown, Range { .. }) => Less,
        (Unknown, Unknown) => Equal,
    }
}
