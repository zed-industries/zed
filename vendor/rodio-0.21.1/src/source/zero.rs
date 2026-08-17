use std::time::Duration;

use dasp_sample::Sample as DaspSample;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::{Sample, Source};

/// An source that produces samples with value zero (silence). Depending on if
/// it where created with [`Zero::new`] or [`Zero::new_samples`] it can be never
/// ending or finite.
#[derive(Copy, Clone, Debug)]
pub struct Zero {
    channels: ChannelCount,
    sample_rate: SampleRate,
    total_samples: Option<usize>,
    position: usize,
}

impl Zero {
    /// Create a new source that never ends and produces total silence.
    #[inline]
    pub fn new(channels: ChannelCount, sample_rate: SampleRate) -> Self {
        Self {
            channels,
            sample_rate,
            total_samples: None,
            position: 0,
        }
    }

    /// Create a new source that never ends and produces total silence.
    #[inline]
    pub fn new_samples(
        channels: ChannelCount,
        sample_rate: SampleRate,
        num_samples: usize,
    ) -> Self {
        Self {
            channels,
            sample_rate,
            total_samples: Some(num_samples),
            position: 0,
        }
    }
}

impl Iterator for Zero {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(total_samples) = self.total_samples {
            if self.position < total_samples {
                self.position += 1;
            } else {
                return None;
            }
        }

        Some(Sample::EQUILIBRIUM)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.total_samples {
            Some(total_samples) => {
                let remaining = total_samples - self.position;
                (remaining, Some(remaining))
            }
            None => (usize::MAX, None),
        }
    }
}

impl Source for Zero {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.total_samples
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.channels
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        self.total_samples.map(|total| {
            let sample_rate = self.sample_rate.get() as u64;
            let frames = total / self.channels.get() as usize;
            let secs = frames as u64 / sample_rate;
            let nanos = ((frames as u64 % sample_rate) * 1_000_000_000) / sample_rate;
            Duration::new(secs, nanos as u32)
        })
    }

    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        if let (Some(total_samples), Some(total_duration)) =
            (self.total_samples, self.total_duration())
        {
            let mut target = pos;
            if target > total_duration {
                target = total_duration;
            }

            let target_samples = (target.as_secs_f32()
                * self.sample_rate.get() as f32
                * self.channels.get() as f32) as usize;
            let target_samples = target_samples.min(total_samples);

            self.position = target_samples;
        }

        Ok(())
    }
}
