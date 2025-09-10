use anyhow::{Context as _, Result, anyhow};
use collections::HashMap;
use gpui::{App, BorrowAppContext, Global};
use rodio::{
    ChannelCount, Decoder, OutputStream, OutputStreamBuilder, SampleRate, Source,
    buffer::SamplesBuffer,
    conversions::{ChannelCountConverter, SampleRateConverter, SampleTypeConverter},
    source::Buffered,
};
use settings::Settings;
use std::{io::Cursor, sync::Arc, time::Duration};
use tokio::sync::OnceCell;
use util::ResultExt;

mod audio_settings;
pub use audio_settings::AudioSettings;

mod streaming;
pub use streaming::*;

pub fn init(cx: &mut App) {
    AudioSettings::register(cx);
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Sound {
    Joined,
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
            Self::Leave => "leave_call",
            Self::Mute => "mute",
            Self::Unmute => "unmute",
            Self::StartScreenshare => "start_screenshare",
            Self::StopScreenshare => "stop_screenshare",
            Self::AgentDone => "agent_done",
        }
    }
}

#[derive(Default)]
pub struct Audio {
    output_handle: Option<OutputStream>,
    source_cache: HashMap<Sound, Buffered<Decoder<Cursor<Vec<u8>>>>>,
}

impl Global for Audio {}

impl Audio {
    fn ensure_output_exists(&mut self) -> Option<&OutputStream> {
        if self.output_handle.is_none() {
            self.output_handle = OutputStreamBuilder::open_default_stream().log_err();
        }

        self.output_handle.as_ref()
    }

    pub fn play_source(
        source: impl rodio::Source + Send + 'static,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        cx.update_default_global(|this: &mut Self, _cx| {
            let output_handle = this
                .ensure_output_exists()
                .ok_or_else(|| anyhow!("Could not open audio output"))?;
            output_handle.mixer().add(source);
            Ok(())
        })
    }

    pub fn play_sound(sound: Sound, cx: &mut App) {
        cx.update_default_global(|this: &mut Self, cx| {
            let source = this.sound_source(sound, cx).log_err()?;
            let output_handle = this.ensure_output_exists()?;
            output_handle.mixer().add(source);
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

static AUDIO_CONTEXT: OnceCell<Arc<AudioContext>> = OnceCell::const_new();

pub async fn async_init() -> anyhow::Result<Arc<AudioContext>> {
    AUDIO_CONTEXT
        .get_or_try_init(async || {
            let ctx = AudioContext::new(AudioConfig {
                input_device: None,
                output_device: None,
                processing_enabled: true,
            })
            .await?;
            Ok(Arc::new(ctx))
        })
        .await
        .cloned()
}

pub trait SourceExt: Source {
    fn frames_for_duration(&self, d: Duration) -> u64 {
        frames_for_duration(self.sample_rate(), d)
    }

    fn samples_for_duration(&self, d: Duration) -> usize {
        samples_for_duration(self.sample_rate(), self.channels(), d)
    }

    fn convert_channels(&mut self, channels: ChannelCount) -> SamplesBuffer {
        let from_channels = self.channels();
        let sample_rate = self.sample_rate();

        let converter = ChannelCountConverter::new(self, from_channels, channels);
        SamplesBuffer::new(channels, sample_rate, converter.collect::<Vec<f32>>())
    }

    fn convert_sample_rate(&mut self, sample_rate: SampleRate) -> SamplesBuffer {
        let from_sample_rate = self.sample_rate();
        let channels = self.channels();

        let samples = SamplesBuffer::new(
            self.channels(),
            from_sample_rate,
            self.collect::<Vec<f32>>(),
        );

        let converter = SampleRateConverter::new(samples, from_sample_rate, sample_rate, channels);
        SamplesBuffer::new(channels, sample_rate, converter.collect::<Vec<f32>>())
    }
}

fn frames_for_duration(sample_rate: SampleRate, d: Duration) -> u64 {
    let sr = sample_rate.get() as u128;
    let micros = d.as_micros();
    ((micros * sr + 999_999) / 1_000_000) as u64
}

pub fn samples_for_duration(sample_rate: SampleRate, channels: ChannelCount, d: Duration) -> usize {
    let frames = frames_for_duration(sample_rate, d) as u128;
    let ch = channels.get() as u128;
    usize::try_from(frames * ch).unwrap_or(usize::MAX)
}

pub trait SamplesBufferExt: Sized {
    fn convert_sample_type<T>(self) -> SampleTypeConverter<Self, T>;

    fn from_sample_type(
        i16s: Vec<i16>,
        sample_rate: SampleRate,
        channel_count: ChannelCount,
    ) -> Self;
}

impl SamplesBufferExt for SamplesBuffer {
    fn convert_sample_type<T>(self) -> SampleTypeConverter<Self, T> {
        SampleTypeConverter::<_, T>::new(self)
    }

    fn from_sample_type(
        i16s: Vec<i16>,
        sample_rate: SampleRate,
        channel_count: ChannelCount,
    ) -> Self {
        let samples: Vec<f32> = i16s.into_iter().map(|s| (s as f32) / 32768.0).collect();
        let samples = SamplesBuffer::new(channel_count, sample_rate, samples);
        samples
    }
}

impl<T: Source + ?Sized> SourceExt for T {}
