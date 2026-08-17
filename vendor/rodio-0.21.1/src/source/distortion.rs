use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::{Float, Source};

/// Internal function that builds a `Distortion` object.
pub(crate) fn distortion<I>(input: I, gain: Float, threshold: Float) -> Distortion<I>
where
    I: Source,
{
    Distortion {
        input,
        gain,
        threshold,
    }
}

/// Filter that applies a distortion effect to the source.
#[derive(Clone, Debug)]
pub struct Distortion<I> {
    input: I,
    gain: Float,
    threshold: Float,
}

impl<I> Distortion<I> {
    /// Modifies the distortion gain.
    #[inline]
    pub fn set_gain(&mut self, gain: Float) {
        self.gain = gain;
    }

    /// Modifies the distortion threshold.
    #[inline]
    pub fn set_threshold(&mut self, threshold: Float) {
        self.threshold = threshold;
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
}

impl<I> Iterator for Distortion<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|value| {
            let v = value * self.gain;
            let t = self.threshold;
            v.clamp(-t, t)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for Distortion<I> where I: Source + ExactSizeIterator {}

impl<I> Source for Distortion<I>
where
    I: Source,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.input.current_span_len()
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
        self.input.total_duration()
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.input.try_seek(pos)
    }
}
