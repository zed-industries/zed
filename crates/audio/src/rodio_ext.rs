use std::{num::NonZero, time::Duration};

use denoise::DenoiserError;
use log::warn;
use rodio::{
    ChannelCount, Sample, SampleRate, Source, buffer::SamplesBuffer,
    conversions::ChannelCountConverter, nz,
};

use crate::rodio_ext::resample::FixedResampler;
pub use replayable::{Replay, ReplayDurationTooShort, Replayable};

mod replayable;
mod resample;
mod resampling_denoise;

const MAX_CHANNELS: usize = 8;

// These all require constant sources (so the span is infinitely long)
// this is not guaranteed by rodio however we know it to be true in all our
// applications. Rodio desperately needs a constant source concept.
pub trait RodioExt: Source + Sized {
    fn process_buffer<const N: usize, F>(self, callback: F) -> ProcessBuffer<N, Self, F>
    where
        F: FnMut(&mut [Sample; N]);
    fn inspect_buffer<const N: usize, F>(self, callback: F) -> InspectBuffer<N, Self, F>
    where
        F: FnMut(&[Sample; N]);
    fn replayable(
        self,
        duration: Duration,
    ) -> Result<(Replay, Replayable<Self>), ReplayDurationTooShort>;
    fn take_samples(self, n: usize) -> TakeSamples<Self>;
    fn denoise(self) -> Result<resampling_denoise::ResamplingDenoiser<Self>, DenoiserError>;
    fn constant_params(
        self,
        channel_count: ChannelCount,
        sample_rate: SampleRate,
    ) -> ConstantChannelCount<FixedResampler<Self>>;
    fn constant_samplerate(self, sample_rate: SampleRate) -> FixedResampler<Self>;
    fn possibly_disconnected_channels_to_mono(self) -> ToMono<Self>;
    fn into_samples_buffer(self) -> SamplesBuffer;
}

impl<S: Source> RodioExt for S {
    fn process_buffer<const N: usize, F>(self, callback: F) -> ProcessBuffer<N, Self, F>
    where
        F: FnMut(&mut [Sample; N]),
    {
        ProcessBuffer {
            inner: self,
            callback,
            buffer: [0.0; N],
            next: N,
        }
    }
    fn inspect_buffer<const N: usize, F>(self, callback: F) -> InspectBuffer<N, Self, F>
    where
        F: FnMut(&[Sample; N]),
    {
        InspectBuffer {
            inner: self,
            callback,
            buffer: [0.0; N],
            free: 0,
        }
    }
    /// Maintains a live replay with a history of at least `duration` seconds.
    ///
    /// Note:
    /// History can be 100ms longer if the source drops before or while the
    /// replay is being read
    ///
    /// # Errors
    /// If duration is smaller than 100ms
    fn replayable(
        self,
        duration: Duration,
    ) -> Result<(Replay, Replayable<Self>), ReplayDurationTooShort> {
        replayable::replayable(self, duration)
    }
    fn take_samples(self, n: usize) -> TakeSamples<S> {
        TakeSamples {
            inner: self,
            left_to_take: n,
        }
    }
    fn denoise(self) -> Result<resampling_denoise::ResamplingDenoiser<Self>, DenoiserError> {
        resampling_denoise::ResamplingDenoiser::new(self)
    }
    fn constant_params(
        self,
        channel_count: ChannelCount,
        sample_rate: SampleRate,
    ) -> ConstantChannelCount<FixedResampler<Self>> {
        ConstantChannelCount::new(self.constant_samplerate(sample_rate), channel_count)
    }
    fn constant_samplerate(self, sample_rate: SampleRate) -> FixedResampler<Self> {
        FixedResampler::new(self, sample_rate)
    }
    fn possibly_disconnected_channels_to_mono(self) -> ToMono<Self> {
        ToMono::new(self)
    }
    fn into_samples_buffer(mut self) -> SamplesBuffer {
        let samples: Vec<_> = self.by_ref().collect();
        SamplesBuffer::new(self.channels(), self.sample_rate(), samples)
    }
}

pub struct ConstantChannelCount<S: Source> {
    inner: ChannelCountConverter<S>,
    channels: ChannelCount,
    sample_rate: SampleRate,
}

impl<S: Source> ConstantChannelCount<S> {
    pub fn new(source: S, target_channels: ChannelCount) -> Self {
        let input_channels = source.channels();
        let sample_rate = source.sample_rate();
        let inner = ChannelCountConverter::new(source, input_channels, target_channels);
        Self {
            sample_rate,
            inner,
            channels: target_channels,
        }
    }
}

impl<S: Source> Iterator for ConstantChannelCount<S> {
    type Item = rodio::Sample;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<S: Source> Source for ConstantChannelCount<S> {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        self.channels
    }

    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None // not supported (not used by us)
    }
}

const TYPICAL_NOISE_FLOOR: Sample = 1e-3;

/// constant source, only works on a single span
pub struct ToMono<S> {
    inner: S,
    input_channel_count: ChannelCount,
    connected_channels: ChannelCount,
    /// running mean of second channel 'volume'
    means: [f32; MAX_CHANNELS],
}
impl<S: Source> ToMono<S> {
    fn new(input: S) -> Self {
        let channels = input
            .channels()
            .min(const { NonZero::<u16>::new(MAX_CHANNELS as u16).unwrap() });
        if channels < input.channels() {
            warn!("Ignoring input channels {}..", channels.get());
        }

        Self {
            connected_channels: channels,
            input_channel_count: channels,
            inner: input,
            means: [TYPICAL_NOISE_FLOOR; MAX_CHANNELS],
        }
    }
}

impl<S: Source> Source for ToMono<S> {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        rodio::nz!(1)
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
}

fn update_mean(mean: &mut f32, sample: Sample) {
    const HISTORY: f32 = 500.0;
    *mean *= (HISTORY - 1.0) / HISTORY;
    *mean += sample.abs() / HISTORY;
}

impl<S: Source> Iterator for ToMono<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let mut mono_sample = 0f32;
        let mut active_channels = 0;
        for channel in 0..self.input_channel_count.get() as usize {
            let sample = self.inner.next()?;
            mono_sample += sample;

            update_mean(&mut self.means[channel], sample);
            if self.means[channel] > TYPICAL_NOISE_FLOOR / 10.0 {
                active_channels += 1;
            }
        }
        mono_sample /= self.connected_channels.get() as f32;
        self.connected_channels = NonZero::new(active_channels).unwrap_or(nz!(1));

        Some(mono_sample)
    }
}

/// constant source, only works on a single span
pub struct TakeSamples<S> {
    inner: S,
    left_to_take: usize,
}

impl<S: Clone> Clone for TakeSamples<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            left_to_take: self.left_to_take,
        }
    }
}

impl<S: Source> Iterator for TakeSamples<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left_to_take == 0 {
            None
        } else {
            self.left_to_take -= 1;
            self.inner.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.left_to_take))
    }
}

impl<S: Source> Source for TakeSamples<S> {
    fn current_span_len(&self) -> Option<usize> {
        None // does not support spans
    }

    fn channels(&self) -> ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            self.left_to_take as f64
                / self.sample_rate().get() as f64
                / self.channels().get() as f64,
        ))
    }
}

/// constant source, only works on a single span
pub struct ProcessBuffer<const N: usize, S, F>
where
    S: Source + Sized,
    F: FnMut(&mut [Sample; N]),
{
    inner: S,
    callback: F,
    /// Buffer used for both input and output.
    buffer: [Sample; N],
    /// Next already processed sample is at this index
    /// in buffer.
    ///
    /// If this is equal to the length of the buffer we have no more samples and
    /// we must get new ones and process them
    next: usize,
}

impl<const N: usize, S, F> Iterator for ProcessBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&mut [Sample; N]),
{
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        self.next += 1;
        if self.next < self.buffer.len() {
            let sample = self.buffer[self.next];
            return Some(sample);
        }

        for sample in &mut self.buffer {
            *sample = self.inner.next()?
        }
        (self.callback)(&mut self.buffer);

        self.next = 0;
        Some(self.buffer[0])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<const N: usize, S, F> Source for ProcessBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&mut [Sample; N]),
{
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}

/// constant source, only works on a single span
pub struct InspectBuffer<const N: usize, S, F>
where
    S: Source + Sized,
    F: FnMut(&[Sample; N]),
{
    inner: S,
    callback: F,
    /// Stores already emitted samples, once its full we call the callback.
    buffer: [Sample; N],
    /// Next free element in buffer. If this is equal to the buffer length
    /// we have no more free lements.
    free: usize,
}

impl<const N: usize, S, F> Iterator for InspectBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&[Sample; N]),
{
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(sample) = self.inner.next() else {
            return None;
        };

        self.buffer[self.free] = sample;
        self.free += 1;

        if self.free == self.buffer.len() {
            (self.callback)(&self.buffer);
            self.free = 0
        }

        Some(sample)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<const N: usize, S, F> Source for InspectBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&[Sample; N]),
{
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}

#[cfg(test)]
mod tests {
    use rodio::{nz, static_buffer::StaticSamplesBuffer};

    use super::*;

    pub const SAMPLES: [Sample; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];

    pub fn test_source() -> StaticSamplesBuffer {
        StaticSamplesBuffer::new(nz!(1), nz!(1), &SAMPLES)
    }

    mod process_buffer {
        use super::*;

        #[test]
        fn callback_gets_all_samples() {
            let input = test_source();

            let _ = input
                .process_buffer::<{ SAMPLES.len() }, _>(|buffer| assert_eq!(*buffer, SAMPLES))
                .count();
        }
        #[test]
        fn callback_modifies_yielded() {
            let input = test_source();

            let yielded: Vec<_> = input
                .process_buffer::<{ SAMPLES.len() }, _>(|buffer| {
                    for sample in buffer {
                        *sample += 1.0;
                    }
                })
                .collect();
            assert_eq!(
                yielded,
                SAMPLES.into_iter().map(|s| s + 1.0).collect::<Vec<_>>()
            )
        }
        #[test]
        fn source_truncates_to_whole_buffers() {
            let input = test_source();

            let yielded = input
                .process_buffer::<3, _>(|buffer| assert_eq!(buffer, &SAMPLES[..3]))
                .count();
            assert_eq!(yielded, 3)
        }
    }

    mod inspect_buffer {
        use super::*;

        #[test]
        fn callback_gets_all_samples() {
            let input = test_source();

            let _ = input
                .inspect_buffer::<{ SAMPLES.len() }, _>(|buffer| assert_eq!(*buffer, SAMPLES))
                .count();
        }
        #[test]
        fn source_does_not_truncate() {
            let input = test_source();

            let yielded = input
                .inspect_buffer::<3, _>(|buffer| assert_eq!(buffer, &SAMPLES[..3]))
                .count();
            assert_eq!(yielded, SAMPLES.len())
        }
    }
}
