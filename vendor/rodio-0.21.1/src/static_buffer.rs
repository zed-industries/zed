//! A simple source of samples coming from a static buffer.
//!
//! The `StaticSamplesBuffer` struct can be used to treat a list of values as a `Source`.
//!
//! # Example
//!
//! ```
//! use rodio::static_buffer::StaticSamplesBuffer;
//! use core::num::NonZero;
//! let _ = StaticSamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(44100).unwrap(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
//! ```
//!

use core::fmt;
use std::slice::Iter as SliceIter;
use std::time::Duration;

use crate::common::{ChannelCount, SampleRate};
use crate::math::NANOS_PER_SEC;
use crate::source::SeekError;
use crate::{Sample, Source};

/// A buffer of samples treated as a source.
#[derive(Clone)]
pub struct StaticSamplesBuffer {
    data: SliceIter<'static, Sample>,
    channels: ChannelCount,
    sample_rate: SampleRate,
    duration: Duration,
}

impl fmt::Debug for StaticSamplesBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticSamplesBuffer")
            .field("channels", &self.channels)
            .field("sample_rate", &self.sample_rate)
            .field("duration", &self.duration)
            .finish()
    }
}

impl StaticSamplesBuffer {
    /// Builds a new `StaticSamplesBuffer`.
    ///
    /// # Panic
    ///
    /// - Panics if the number of channels is zero.
    /// - Panics if the samples rate is zero.
    /// - Panics if the length of the buffer is larger than approximately 16 billion elements.
    ///   This is because the calculation of the duration would overflow.
    ///
    pub fn new(
        channels: ChannelCount,
        sample_rate: SampleRate,
        data: &'static [Sample],
    ) -> StaticSamplesBuffer {
        let duration_ns = NANOS_PER_SEC.checked_mul(data.len() as u64).unwrap()
            / sample_rate.get() as u64
            / channels.get() as u64;
        let duration = Duration::new(
            duration_ns / NANOS_PER_SEC,
            (duration_ns % NANOS_PER_SEC) as u32,
        );

        StaticSamplesBuffer {
            data: data.iter(),
            channels,
            sample_rate,
            duration,
        }
    }
}

impl Source for StaticSamplesBuffer {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.channels
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        Some(self.duration)
    }

    #[inline]
    fn try_seek(&mut self, _: Duration) -> Result<(), SeekError> {
        Err(SeekError::NotSupported {
            underlying_source: std::any::type_name::<Self>(),
        })
    }
}

impl Iterator for StaticSamplesBuffer {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().cloned()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.data.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::nz;
    use crate::source::Source;
    use crate::static_buffer::StaticSamplesBuffer;

    #[test]
    fn basic() {
        let _ = StaticSamplesBuffer::new(nz!(1), nz!(44100), &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn duration_basic() {
        let buf = StaticSamplesBuffer::new(nz!(2), nz!(2), &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let dur = buf.total_duration().unwrap();
        assert_eq!(dur.as_secs(), 1);
        assert_eq!(dur.subsec_nanos(), 500_000_000);
    }

    #[test]
    fn iteration() {
        let mut buf = StaticSamplesBuffer::new(nz!(1), nz!(44100), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(buf.next(), Some(1.0));
        assert_eq!(buf.next(), Some(2.0));
        assert_eq!(buf.next(), Some(3.0));
        assert_eq!(buf.next(), Some(4.0));
        assert_eq!(buf.next(), Some(5.0));
        assert_eq!(buf.next(), Some(6.0));
        assert_eq!(buf.next(), None);
    }
}
