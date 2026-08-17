//! Chirp/sweep source.

use std::time::Duration;

use crate::{
    common::{ChannelCount, SampleRate},
    math::{nz, TAU},
    source::SeekError,
    Float, Sample, Source,
};

/// Convenience function to create a new `Chirp` source.
#[inline]
pub fn chirp(
    sample_rate: SampleRate,
    start_frequency: Float,
    end_frequency: Float,
    duration: Duration,
) -> Chirp {
    Chirp::new(sample_rate, start_frequency, end_frequency, duration)
}

/// Generate a sine wave with an instantaneous frequency that changes/sweeps linearly over time.
/// At the end of the chirp, once the `end_frequency` is reached, the source is exhausted.
#[derive(Clone, Debug)]
pub struct Chirp {
    start_frequency: Float,
    end_frequency: Float,
    sample_rate: SampleRate,
    total_samples: u64,
    elapsed_samples: u64,
}

impl Chirp {
    fn new(
        sample_rate: SampleRate,
        start_frequency: Float,
        end_frequency: Float,
        duration: Duration,
    ) -> Self {
        Self {
            sample_rate,
            start_frequency,
            end_frequency,
            total_samples: (duration.as_secs_f64() * sample_rate.get() as f64) as u64,
            elapsed_samples: 0,
        }
    }

    #[allow(dead_code)]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        let mut target = (pos.as_secs_f64() * self.sample_rate.get() as f64) as u64;
        if target >= self.total_samples {
            target = self.total_samples;
        }

        self.elapsed_samples = target;
        Ok(())
    }
}

impl Iterator for Chirp {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.elapsed_samples;
        if i >= self.total_samples {
            return None; // Exhausted
        }

        let ratio = (i as f64 / self.total_samples as f64) as Float;
        let freq = self.start_frequency * (1.0 - ratio) + self.end_frequency * ratio;
        let t = (i as f64 / self.sample_rate().get() as f64) as Float * TAU * freq;

        self.elapsed_samples += 1;
        Some(t.sin())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_samples - self.elapsed_samples;
        (remaining as usize, Some(remaining as usize))
    }
}

impl ExactSizeIterator for Chirp {}

impl Source for Chirp {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        nz!(1)
    }

    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        let secs = self.total_samples as f64 / self.sample_rate.get() as f64;
        Some(Duration::from_secs_f64(secs))
    }
}
