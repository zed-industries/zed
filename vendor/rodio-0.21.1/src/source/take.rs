use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::math::NANOS_PER_SEC;
use crate::{Float, Sample, Source};

/// Internal function that builds a `TakeDuration` object.
pub fn take_duration<I>(input: I, duration: Duration) -> TakeDuration<I>
where
    I: Source,
{
    TakeDuration {
        current_span_len: input.current_span_len(),
        duration_per_sample: TakeDuration::get_duration_per_sample(&input),
        input,
        remaining_duration: duration,
        requested_duration: duration,
        filter: None,
    }
}

/// A filter that can be applied to a `TakeDuration`.
#[derive(Clone, Debug)]
enum DurationFilter {
    FadeOut,
}
impl DurationFilter {
    fn apply<I: Iterator>(&self, sample: Sample, parent: &TakeDuration<I>) -> Sample {
        match self {
            DurationFilter::FadeOut => {
                let remaining = parent.remaining_duration.as_millis() as Float;
                let total = parent.requested_duration.as_millis() as Float;
                sample * remaining / total
            }
        }
    }
}

/// A source that truncates the given source to a certain duration.
#[derive(Clone, Debug)]
pub struct TakeDuration<I> {
    input: I,
    remaining_duration: Duration,
    requested_duration: Duration,
    filter: Option<DurationFilter>,
    // Remaining samples in current span.
    current_span_len: Option<usize>,
    // Only updated when the current span len is exhausted.
    duration_per_sample: Duration,
}

impl<I> TakeDuration<I>
where
    I: Source,
{
    /// Returns the duration elapsed for each sample extracted.
    #[inline]
    fn get_duration_per_sample(input: &I) -> Duration {
        let ns = NANOS_PER_SEC / (input.sample_rate().get() as u64 * input.channels().get() as u64);
        // \|/ the maximum value of `ns` is one billion, so this can't fail
        Duration::new(0, ns as u32)
    }

    /// Returns a reference to the inner source.
    #[inline]
    pub fn inner(&self) -> &I {
        &self.input
    }

    /// Returns a mutable reference to the inner source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }

    /// Returns the inner source.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }

    /// Make the truncated source end with a FadeOut. The fadeout covers the
    /// entire length of the take source.
    pub fn set_filter_fadeout(&mut self) {
        self.filter = Some(DurationFilter::FadeOut);
    }

    /// Remove any filter set.
    pub fn clear_filter(&mut self) {
        self.filter = None;
    }
}

impl<I> Iterator for TakeDuration<I>
where
    I: Source,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if let Some(span_len) = self.current_span_len.take() {
            if span_len > 0 {
                self.current_span_len = Some(span_len - 1);
            } else {
                self.current_span_len = self.input.current_span_len();
                // Sample rate might have changed
                self.duration_per_sample = Self::get_duration_per_sample(&self.input);
            }
        }

        if self.remaining_duration <= self.duration_per_sample {
            None
        } else if let Some(sample) = self.input.next() {
            let sample = match &self.filter {
                Some(filter) => filter.apply(sample, self),
                None => sample,
            };

            self.remaining_duration -= self.duration_per_sample;

            Some(sample)
        } else {
            None
        }
    }

    // TODO: size_hint
}

impl<I> Source for TakeDuration<I>
where
    I: Iterator + Source,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        let remaining_nanos = self.remaining_duration.as_secs() * NANOS_PER_SEC
            + self.remaining_duration.subsec_nanos() as u64;
        let nanos_per_sample = self.duration_per_sample.as_secs() * NANOS_PER_SEC
            + self.duration_per_sample.subsec_nanos() as u64;
        let remaining_samples = (remaining_nanos / nanos_per_sample) as usize;

        self.input
            .current_span_len()
            .filter(|value| *value < remaining_samples)
            .or(Some(remaining_samples))
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.input.channels()
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.input.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        if let Some(duration) = self.input.total_duration() {
            if duration < self.requested_duration {
                Some(duration)
            } else {
                Some(self.requested_duration)
            }
        } else {
            None
        }
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.input.try_seek(pos)
    }
}
