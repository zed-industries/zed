use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::Source;

/// Builds a `Pausable` object.
pub fn pausable<I>(source: I, paused: bool) -> Pausable<I>
where
    I: Source,
{
    let paused_channels = if paused {
        Some(source.channels())
    } else {
        None
    };
    Pausable {
        input: source,
        paused_channels,
        remaining_paused_samples: 0,
    }
}

/// Wraps a source and makes it pausable by calling [`Pausable::set_paused`] on
/// this object. When the source is paused it returns zero value samples.
///
/// You can usually still use this from another source wrapping this one by
/// calling `inner_mut` on it. Similarly this provides [`Pausable::inner`] and
/// mutable/destructing variants for accessing the underlying source.
#[derive(Clone, Debug)]
pub struct Pausable<I> {
    input: I,
    paused_channels: Option<ChannelCount>,
    remaining_paused_samples: u16,
}

impl<I> Pausable<I>
where
    I: Source,
{
    /// Sets whether the filter applies.
    ///
    /// If set to true, the inner sound stops playing and no samples are processed from it.
    #[inline]
    pub fn set_paused(&mut self, paused: bool) {
        match (self.paused_channels, paused) {
            (None, true) => self.paused_channels = Some(self.input.channels()),
            (Some(_), false) => self.paused_channels = None,
            _ => (),
        }
    }

    /// Indicates if the data source is in a paused state.
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.paused_channels.is_some()
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

impl<I> Iterator for Pausable<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.remaining_paused_samples > 0 {
            self.remaining_paused_samples -= 1;
            return Some(0.0);
        }

        if let Some(paused_channels) = self.paused_channels {
            self.remaining_paused_samples = paused_channels.get() - 1;
            return Some(0.0);
        }

        self.input.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> Source for Pausable<I>
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
