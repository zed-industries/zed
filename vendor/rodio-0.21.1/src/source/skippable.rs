use crate::common::{ChannelCount, SampleRate};
use crate::Source;
use std::time::Duration;

use super::SeekError;

/// Wrap the source in a skippable. It allows ending the current source early by
/// calling [`Skippable::skip`]. If this source is in a queue such as the Player
/// ending the source early is equal to skipping the source.
pub fn skippable<I>(source: I) -> Skippable<I> {
    Skippable {
        input: source,
        do_skip: false,
    }
}

/// Wrap the source in a skippable. It allows ending the current source early by
/// calling [`Skippable::skip`]. If this source is in a queue such as the Player
/// ending the source early is equal to skipping the source.
#[derive(Clone, Debug)]
pub struct Skippable<I> {
    input: I,
    do_skip: bool,
}

impl<I> Skippable<I> {
    /// Skips the current source
    #[inline]
    pub fn skip(&mut self) {
        self.do_skip = true;
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

impl<I> Iterator for Skippable<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.do_skip {
            None
        } else {
            self.input.next()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> Source for Skippable<I>
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
