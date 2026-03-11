use std::sync::Arc;

use futures::StreamExt;
use libwebrtc::{audio_stream::native::NativeAudioStream, prelude::AudioFrame};
use livekit::track::RemoteAudioTrack;
use parking_lot::Mutex;
use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rodio::{
    ChannelCount, Sample, SampleRate, Source, buffer::SamplesBuffer,
    conversions::SampleTypeConverter,
};

use audio::{CHANNEL_COUNT, LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE, SAMPLE_RATE};

// 10ms frames; 10 frames = 100ms max buffered before we start dropping old frames.
const RING_BUFFER_CAPACITY: usize = 10;

fn frame_to_samplesbuffer(frame: AudioFrame) -> SamplesBuffer {
    let samples = frame.data.iter().copied();
    let samples = SampleTypeConverter::<_, _>::new(samples);
    let samples: Vec<f32> = samples.collect();
    SamplesBuffer::new(
        std::num::NonZero::new(frame.num_channels as u16).expect("zero channels is nonsense"),
        std::num::NonZero::new(frame.sample_rate).expect("samplerate zero is nonsense"),
        samples,
    )
}

pub struct LiveKitStream {
    buffer: Arc<Mutex<ConstGenericRingBuffer<SamplesBuffer, RING_BUFFER_CAPACITY>>>,
    current: Option<SamplesBuffer>,
    _receiver_task: gpui::Task<()>,
    channel_count: ChannelCount,
    sample_rate: SampleRate,
}

impl LiveKitStream {
    pub fn new(
        executor: &gpui::BackgroundExecutor,
        track: &RemoteAudioTrack,
        legacy: bool,
    ) -> Self {
        let (channel_count, sample_rate) = if legacy {
            (LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE)
        } else {
            (CHANNEL_COUNT, SAMPLE_RATE)
        };

        let mut stream = NativeAudioStream::new(
            track.rtc_track(),
            sample_rate.get() as i32,
            channel_count.get().into(),
        );

        let buffer: Arc<Mutex<ConstGenericRingBuffer<SamplesBuffer, RING_BUFFER_CAPACITY>>> =
            Arc::default();

        let receiver_task = executor.spawn_with_priority(gpui::Priority::RealtimeAudio, {
            let buffer = Arc::clone(&buffer);
            async move {
                while let Some(frame) = stream.next().await {
                    let samples = frame_to_samplesbuffer(frame);
                    buffer.lock().enqueue(samples);
                }
            }
        });

        LiveKitStream {
            buffer,
            current: None,
            _receiver_task: receiver_task,
            sample_rate,
            channel_count,
        }
    }
}

impl Iterator for LiveKitStream {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = &mut self.current {
                if let Some(sample) = current.next() {
                    return Some(sample);
                }
            }
            self.current = self.buffer.lock().dequeue();
            if self.current.is_none() {
                // Underrun: emit silence rather than ending the stream.
                return Some(0.0);
            }
        }
    }
}

impl Source for LiveKitStream {
    fn current_span_len(&self) -> Option<usize> {
        self.current.as_ref().and_then(|s| s.current_span_len())
    }

    fn channels(&self) -> ChannelCount {
        self.channel_count
    }

    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
