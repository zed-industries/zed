use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::math::nz;
use crate::{Sample, Source};

/// An empty source.
#[derive(Debug, Default, Copy, Clone)]
pub struct Empty;

impl Empty {
    /// An empty source that immediately ends without ever returning a sample to
    /// play
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Iterator for Empty {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl ExactSizeIterator for Empty {}

impl Source for Empty {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        Some(0)
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        nz!(1)
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        nz!(48000)
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::ZERO)
    }

    #[inline]
    fn try_seek(&mut self, _: Duration) -> Result<(), SeekError> {
        Err(SeekError::NotSupported {
            underlying_source: std::any::type_name::<Self>(),
        })
    }
}
