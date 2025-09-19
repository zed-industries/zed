use std::num::NonZero;

use futures::StreamExt;
use libwebrtc::{audio_stream::native::NativeAudioStream, prelude::AudioFrame};
use livekit::track::RemoteAudioTrack;
use rodio::{Source, buffer::SamplesBuffer, conversions::SampleTypeConverter, nz};

use audio::{CHANNEL_COUNT, SAMPLE_RATE};

fn frame_to_samplesbuffer(frame: AudioFrame) -> SamplesBuffer {
    let samples = frame.data.iter().copied();
    let samples = SampleTypeConverter::<_, _>::new(samples);
    let samples: Vec<f32> = samples.collect();
    SamplesBuffer::new(
        nz!(2), // frame always has two channels
        NonZero::new(frame.sample_rate).expect("audio frame sample rate is nonzero"),
        samples,
    )
}

pub struct LiveKitStream {
    // shared_buffer: SharedBuffer,
    inner: rodio::queue::SourcesQueueOutput,
    _receiver_task: gpui::Task<()>,
}

impl LiveKitStream {
    pub fn new(executor: &gpui::BackgroundExecutor, track: &RemoteAudioTrack) -> Self {
        let mut stream = NativeAudioStream::new(
            track.rtc_track(),
            SAMPLE_RATE.get() as i32,
            CHANNEL_COUNT.get().into(),
        );
        let (queue_input, queue_output) = rodio::queue::queue(true);
        // spawn rtc stream
        let receiver_task = executor.spawn({
            async move {
                while let Some(frame) = stream.next().await {
                    let samples = frame_to_samplesbuffer(frame);
                    queue_input.append(samples);
                }
            }
        });

        LiveKitStream {
            _receiver_task: receiver_task,
            inner: queue_output,
        }
    }
}

impl Iterator for LiveKitStream {
    type Item = rodio::Sample;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl Source for LiveKitStream {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }

    fn channels(&self) -> rodio::ChannelCount {
        // This must be hardcoded because the playback source assumes constant
        // sample rate and channel count. The queue upon which this is build
        // will however report different counts and rates. Even though we put in
        // only items with our (constant) CHANNEL_COUNT & SAMPLE_RATE this will
        // play silence on one channel and at 44100 which is not what our
        // constants are.
        CHANNEL_COUNT
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        SAMPLE_RATE // see comment on channels
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}
