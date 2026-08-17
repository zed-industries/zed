use std::time::Duration;

use super::SeekError;
use crate::{
    common::{ChannelCount, Float, SampleRate},
    math, Source,
};

/// Internal function that builds a `Amplify` object.
pub fn amplify<I>(input: I, factor: Float) -> Amplify<I>
where
    I: Source,
{
    Amplify { input, factor }
}

/// Filter that modifies each sample by a given value.
#[derive(Clone, Debug)]
pub struct Amplify<I> {
    input: I,
    factor: Float,
}

impl<I> Amplify<I> {
    /// Modifies the amplification factor.
    #[inline]
    pub fn set_factor(&mut self, factor: Float) {
        self.factor = factor;
    }

    /// Modifies the amplification factor logarithmically.
    #[inline]
    pub fn set_log_factor(&mut self, factor: Float) {
        self.factor = math::db_to_linear(factor);
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

impl<I> Iterator for Amplify<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|value| value * self.factor)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for Amplify<I> where I: Source + ExactSizeIterator {}

impl<I> Source for Amplify<I>
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
