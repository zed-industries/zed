use crate::common::{ChannelCount, SampleRate};
use crate::math::nz;
use crate::source::{Function, SignalGenerator};
use crate::{Sample, Source};
use std::time::Duration;

use super::SeekError;

/// An infinite source that produces a triangle wave.
///
/// Always has a sample rate of 48kHz and one channel.
///
/// This source is a thin interface on top of `SignalGenerator` provided for
/// your convenience.
#[derive(Clone, Debug)]
pub struct TriangleWave {
    test_tri: SignalGenerator,
}

impl TriangleWave {
    const SAMPLE_RATE: SampleRate = nz!(48000);

    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> TriangleWave {
        TriangleWave {
            test_tri: SignalGenerator::new(Self::SAMPLE_RATE, freq, Function::Triangle),
        }
    }
}

impl Iterator for TriangleWave {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Sample> {
        self.test_tri.next()
    }
}

impl Source for TriangleWave {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        nz!(1)
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        Self::SAMPLE_RATE
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }

    #[inline]
    fn try_seek(&mut self, duration: Duration) -> Result<(), SeekError> {
        self.test_tri.try_seek(duration)
    }
}
