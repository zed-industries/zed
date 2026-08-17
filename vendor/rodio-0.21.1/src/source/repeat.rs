use std::time::Duration;

use crate::source::buffered::Buffered;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::Source;

/// Internal function that builds a `Repeat` object.
pub fn repeat<I>(input: I) -> Repeat<I>
where
    I: Source,
{
    let input = input.buffered();
    Repeat {
        inner: input.clone(),
        next: input,
    }
}

/// A source that repeats the given source.
pub struct Repeat<I>
where
    I: Source,
{
    inner: Buffered<I>,
    next: Buffered<I>,
}

impl<I> Iterator for Repeat<I>
where
    I: Source,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if let Some(value) = self.inner.next() {
            return Some(value);
        }

        self.inner = self.next.clone();
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // infinite
        (0, None)
    }
}

impl<I> Source for Repeat<I>
where
    I: Iterator + Source,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        if self.inner.is_exhausted() {
            self.next.current_span_len()
        } else {
            self.inner.current_span_len()
        }
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        if self.inner.is_exhausted() {
            self.next.channels()
        } else {
            self.inner.channels()
        }
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        if self.inner.is_exhausted() {
            self.next.sample_rate()
        } else {
            self.inner.sample_rate()
        }
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.inner.try_seek(pos)
    }
}

impl<I> Clone for Repeat<I>
where
    I: Source,
{
    #[inline]
    fn clone(&self) -> Repeat<I> {
        Repeat {
            inner: self.inner.clone(),
            next: self.next.clone(),
        }
    }
}
