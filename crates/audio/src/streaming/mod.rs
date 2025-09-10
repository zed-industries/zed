use std::{
    sync::{Arc, Weak},
    time::Duration,
};

use anyhow::Result;
use futures::{
    FutureExt, SinkExt, StreamExt,
    channel::mpsc::{UnboundedReceiver, UnboundedSender, unbounded},
};
use rodio::{ChannelCount, SampleRate, cpal};

use self::{capture::AudioCapture, device::list_devices, playback::AudioPlayback};
pub use self::{
    capture::AudioSink,
    device::{AudioConfig, Devices, Direction},
    playback::AudioSource,
};

mod processor;
use processor::AudioProcessor;

#[cfg(all(feature = "aec-processing", not(feature = "webrtc-processing")))]
pub use processor::AECAudioProcessor as DefaultAudioProcessor;
#[cfg(all(feature = "webrtc-processing", not(feature = "aec-processing")))]
pub use processor::WebrtcAudioProcessor as DefaultAudioProcessor;
#[cfg(all(not(feature = "webrtc-processing"), not(feature = "aec-processing")))]
#[derive(Debug, Clone)]
pub struct DefaultAudioProcessor;
#[cfg(all(not(feature = "webrtc-processing"), not(feature = "aec-processing")))]
impl processor::AudioProcessor for DefaultAudioProcessor {
    fn new(_enabled: bool) -> anyhow::Result<Self> {
        Ok(Self)
    }
    fn is_enabled(&self) -> bool {
        false
    }
    fn set_enabled(&self, _enabled: bool) {}
    fn init_capture(&self, _channels: usize) -> anyhow::Result<()> {
        Ok(())
    }
    fn init_playback(&self, _channels: usize) -> anyhow::Result<()> {
        Ok(())
    }
    fn process_capture_frame(&self, _frame: &mut [f32]) -> Result<(), ()> {
        Ok(())
    }
    fn process_render_frame(&self, _frame: &mut [f32]) -> Result<(), ()> {
        Ok(())
    }
}

mod capture;
mod device;
mod playback;

// Currently cannot be altered due to assumptions about sample rate / channel count elsewhere--would need to be addressed
pub const SAMPLE_RATE: SampleRate = SampleRate::new(48_000).unwrap();
pub const CHANNEL_COUNT: ChannelCount = ChannelCount::new(2).unwrap();
pub const ENGINE_FORMAT: AudioFormat = AudioFormat::new(SAMPLE_RATE, CHANNEL_COUNT);

pub const DURATION_10MS: Duration = Duration::from_millis(10);
pub const DURATION_20MS: Duration = Duration::from_millis(20);

#[derive(Debug, Clone)]
pub struct AudioContext {
    pub playback: AudioPlayback,
    pub capture: AudioCapture,
}

impl AudioContext {
    pub async fn list_devices() -> Result<Devices> {
        tokio::task::spawn_blocking(list_devices).await?
    }

    pub fn list_devices_sync() -> Result<Devices> {
        list_devices()
    }

    /// Create a new [`AudioContext`].
    pub async fn new(config: AudioConfig) -> Result<Self> {
        let host = cpal::default_host();

        let processor = DefaultAudioProcessor::new(config.processing_enabled)?;

        let capture =
            AudioCapture::build(&host, config.input_device.as_deref(), processor.clone()).await?;
        let playback =
            AudioPlayback::build(&host, config.output_device.as_deref(), processor.clone()).await?;
        Ok(Self { playback, capture })
    }

    pub async fn feedback_raw(&self) -> Result<()> {
        let buffer_size = ENGINE_FORMAT.sample_count(DURATION_20MS * 16);
        let (sink, source) = ringbuf_pipe(buffer_size);
        self.capture.add_sink(sink).await?;
        self.playback.add_source(source).await?;
        Ok(())
    }
}

use std::ops::ControlFlow;

use ringbuf::{
    HeapCons as Consumer, HeapProd as Producer,
    traits::{Consumer as _, Producer as _, Split},
};

pub struct RingbufSink {
    producer: Producer<f32>,
    peer_alive: Weak<()>,
    _self_alive: Arc<()>,
}

pub struct RingbufSource {
    consumer: Consumer<f32>,
    tx: UnboundedSender<AudioSourceEvent>,
    rx: UnboundedReceiver<AudioSourceEvent>,
    peer_alive: Weak<()>,
    _self_alive: Arc<()>,
}

pub fn ringbuf_pipe(buffer_size: usize) -> (RingbufSink, RingbufSource) {
    let sink_alive = Arc::new(());
    let source_alive = Arc::new(());

    let (producer, consumer) = ringbuf::HeapRb::<f32>::new(buffer_size).split();
    (
        RingbufSink::new(producer, Arc::downgrade(&source_alive), sink_alive.clone()),
        RingbufSource::new(consumer, Arc::downgrade(&sink_alive), source_alive.clone()),
    )
}

impl AudioSink for RingbufSink {
    fn tick(&mut self, buf: &[f32]) -> Result<ControlFlow<(), ()>> {
        if self.peer_alive.upgrade().is_none() {
            return Ok(ControlFlow::Break(()));
        }
        let _ = self.producer.push_slice(buf);
        Ok(ControlFlow::Continue(()))
    }
}

impl RingbufSink {
    fn new(producer: Producer<f32>, peer_alive: Weak<()>, self_alive: Arc<()>) -> Self {
        Self {
            producer,
            peer_alive,
            _self_alive: self_alive,
        }
    }
}

impl AudioSource for RingbufSource {
    fn tick(&mut self, buf: &mut [f32]) -> Result<ControlFlow<(), usize>> {
        if self.peer_alive.upgrade().is_none() {
            return Ok(ControlFlow::Break(()));
        }
        if let Some(ev) = self.rx.next().now_or_never().flatten() {
            match ev {
                AudioSourceEvent::Clear => {
                    self.consumer.clear();
                }
            }
        }
        let len = self.consumer.pop_slice(buf);
        Ok(ControlFlow::Continue(len))
    }
}

impl RingbufSource {
    fn new(consumer: Consumer<f32>, peer_alive: Weak<()>, self_alive: Arc<()>) -> Self {
        let (tx, rx) = unbounded();
        Self {
            consumer,
            tx,
            rx,
            peer_alive,
            _self_alive: self_alive,
        }
    }

    pub fn controller(&self) -> AudioSourceController {
        AudioSourceController {
            tx: self.tx.clone(),
        }
    }
}

pub enum AudioSourceEvent {
    Clear,
}

#[derive(Clone)]
pub struct AudioSourceController {
    tx: UnboundedSender<AudioSourceEvent>,
}

impl AudioSourceController {
    pub async fn clear(&mut self) -> Result<()> {
        self.tx.send(AudioSourceEvent::Clear).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioFormat {
    pub sample_rate: SampleRate,
    pub channel_count: ChannelCount,
}

impl AudioFormat {
    pub const fn new(sample_rate: SampleRate, channel_count: ChannelCount) -> Self {
        Self {
            sample_rate,
            channel_count,
        }
    }

    pub fn duration_from_sample_count(&self, sample_count: usize) -> Duration {
        Duration::from_secs_f32(
            (sample_count as f32 / self.channel_count.get() as f32) / self.sample_rate.get() as f32,
        )
    }

    pub const fn block_count(&self, duration: Duration) -> usize {
        (self.sample_rate.get() as usize / 1000) * duration.as_millis() as usize
    }

    pub const fn sample_count(&self, duration: Duration) -> usize {
        self.block_count(duration) * self.channel_count.get() as usize
    }
}
