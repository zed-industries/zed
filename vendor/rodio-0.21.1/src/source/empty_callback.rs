use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::math::nz;
use crate::{Sample, Source};

/// An empty source that executes a callback function
pub struct EmptyCallback {
    callback: Box<dyn Send + Fn()>,
}

impl EmptyCallback {
    #[inline]
    /// Create an empty source that executes a callback function.
    /// Example use-case:
    ///
    /// Detect and do something when the source before this one has ended.
    pub fn new(callback: Box<dyn Send + Fn()>) -> EmptyCallback {
        EmptyCallback { callback }
    }
}

impl Iterator for EmptyCallback {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.callback)();
        None
    }
}

impl Source for EmptyCallback {
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
        nz!(48000)
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::new(0, 0))
    }

    #[inline]
    fn try_seek(&mut self, _: Duration) -> Result<(), SeekError> {
        Err(SeekError::NotSupported {
            underlying_source: std::any::type_name::<Self>(),
        })
    }
}
