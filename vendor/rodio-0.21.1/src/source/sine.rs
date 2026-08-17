use crate::common::{ChannelCount, SampleRate};
use crate::math::nz;
use crate::source::{Function, SignalGenerator};
use crate::{Sample, Source};
use std::time::Duration;

use super::SeekError;

/// An infinite source that produces a sine.
///
/// Always has a sample rate of 48kHz and one channel.
///
/// This source is a thin interface on top of `SignalGenerator` provided for
/// your convenience.
#[derive(Clone, Debug)]
pub struct SineWave {
    test_sine: SignalGenerator,
}

impl SineWave {
    const SAMPLE_RATE: SampleRate = nz!(48000);

    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> SineWave {
        SineWave {
            test_sine: SignalGenerator::new(Self::SAMPLE_RATE, freq, Function::Sine),
        }
    }
}

impl Iterator for SineWave {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Sample> {
        self.test_sine.next()
    }
}

impl Source for SineWave {
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
        self.test_sine.try_seek(duration)
    }
}
