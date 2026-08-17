use crate::common::{ChannelCount, SampleRate};
use crate::math::nz;
use crate::source::{Function, SignalGenerator};
use crate::{Sample, Source};
use std::time::Duration;

use super::SeekError;

/// An infinite source that produces a square wave.
///
/// Always has a sample rate of 48kHz and one channel.
///
/// This source is a thin interface on top of `SignalGenerator` provided for
/// your convenience.
#[derive(Clone, Debug)]
pub struct SquareWave {
    test_square: SignalGenerator,
}

impl SquareWave {
    const SAMPLE_RATE: SampleRate = nz!(48000);

    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> SquareWave {
        SquareWave {
            test_square: SignalGenerator::new(Self::SAMPLE_RATE, freq, Function::Square),
        }
    }
}

impl Iterator for SquareWave {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Sample> {
        self.test_square.next()
    }
}

impl Source for SquareWave {
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
        self.test_square.try_seek(duration)
    }
}
