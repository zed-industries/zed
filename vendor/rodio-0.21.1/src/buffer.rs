//! A simple source of samples coming from a buffer.
//!
//! The `SamplesBuffer` struct can be used to treat a list of values as a `Source`.
//!
//! # Example
//!
//! ```
//! use rodio::buffer::SamplesBuffer;
//! use core::num::NonZero;
//! let _ = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(44100).unwrap(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
//! ```
//!

use crate::common::{ChannelCount, SampleRate};
use crate::math::{duration_to_float, NANOS_PER_SEC};
use crate::source::{SeekError, UniformSourceIterator};
use crate::{Float, Sample, Source};
use std::sync::Arc;
use std::time::Duration;

/// A buffer of samples treated as a source.
#[derive(Debug, Clone)]
pub struct SamplesBuffer {
    data: Arc<[Sample]>,
    pos: usize,
    channels: ChannelCount,
    sample_rate: SampleRate,
    duration: Duration,
}

impl SamplesBuffer {
    /// Builds a new `SamplesBuffer`.
    ///
    /// # Panics
    ///
    /// - Panics if the samples rate is zero.
    /// - Panics if the length of the buffer is larger than approximately 16 billion elements.
    ///   This is because the calculation of the duration would overflow.
    ///
    pub fn new<D>(channels: ChannelCount, sample_rate: SampleRate, data: D) -> Self
    where
        D: Into<Vec<Sample>>,
    {
        let data: Arc<[Sample]> = data.into().into();
        let duration_ns = NANOS_PER_SEC.checked_mul(data.len() as u64).unwrap()
            / sample_rate.get() as u64
            / channels.get() as u64;
        let duration = Duration::new(
            duration_ns / NANOS_PER_SEC,
            (duration_ns % NANOS_PER_SEC) as u32,
        );

        Self {
            data,
            pos: 0,
            channels,
            sample_rate,
            duration,
        }
    }

    pub(crate) fn record_source(source: impl Source) -> Self {
        let channel_count = source.channels();
        let sample_rate = source.sample_rate();
        let source = UniformSourceIterator::new(source, channel_count, sample_rate);
        Self::new(
            source.channels(),
            source.sample_rate(),
            source.into_iter().collect::<Vec<_>>(),
        )
    }
}

impl Source for SamplesBuffer {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        if self.pos >= self.data.len() {
            Some(0)
        } else {
            Some(self.data.len())
        }
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

    /// This jumps in memory till the sample for `pos`.
    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        // This is fast because all the samples are in memory already
        // and due to the constant sample_rate we can jump to the right
        // sample directly.

        let curr_channel = self.pos % self.channels().get() as usize;
        let new_pos = duration_to_float(pos)
            * self.sample_rate().get() as Float
            * self.channels().get() as Float;
        // saturate pos at the end of the source
        let new_pos = new_pos as usize;
        let new_pos = new_pos.min(self.data.len());

        // make sure the next sample is for the right channel
        let new_pos = new_pos.next_multiple_of(self.channels().get() as usize);
        let new_pos = new_pos - curr_channel;

        self.pos = new_pos;
        Ok(())
    }
}

impl Iterator for SamplesBuffer {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.data.get(self.pos)?;
        self.pos += 1;
        Some(*sample)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len() - self.pos;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for SamplesBuffer {}

#[cfg(test)]
mod tests {
    use crate::buffer::SamplesBuffer;
    use crate::math::nz;
    use crate::source::Source;

    #[test]
    fn basic() {
        let _ = SamplesBuffer::new(nz!(1), nz!(44100), vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn duration_basic() {
        let buf = SamplesBuffer::new(nz!(2), nz!(2), vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let dur = buf.total_duration().unwrap();
        assert_eq!(dur.as_secs(), 1);
        assert_eq!(dur.subsec_nanos(), 500_000_000);
    }

    #[test]
    fn iteration() {
        let mut buf = SamplesBuffer::new(nz!(1), nz!(44100), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(buf.next(), Some(1.0));
        assert_eq!(buf.next(), Some(2.0));
        assert_eq!(buf.next(), Some(3.0));
        assert_eq!(buf.next(), Some(4.0));
        assert_eq!(buf.next(), Some(5.0));
        assert_eq!(buf.next(), Some(6.0));
        assert_eq!(buf.next(), None);
    }

    #[cfg(test)]
    mod try_seek {
        use super::*;
        use crate::common::{ChannelCount, Float, SampleRate};
        use crate::Sample;
        use std::time::Duration;

        #[test]
        fn channel_order_stays_correct() {
            const SAMPLE_RATE: SampleRate = nz!(100);
            const CHANNELS: ChannelCount = nz!(2);
            let mut buf = SamplesBuffer::new(
                CHANNELS,
                SAMPLE_RATE,
                (0..2000i16).map(|s| s as Sample).collect::<Vec<_>>(),
            );
            buf.try_seek(Duration::from_secs(5)).unwrap();
            assert_eq!(
                buf.next(),
                Some(5.0 * SAMPLE_RATE.get() as Float * CHANNELS.get() as Float)
            );

            assert!(buf.next().is_some_and(|s| s.trunc() as i32 % 2 == 1));
            assert!(buf.next().is_some_and(|s| s.trunc() as i32 % 2 == 0));

            buf.try_seek(Duration::from_secs(6)).unwrap();
            assert!(buf.next().is_some_and(|s| s.trunc() as i32 % 2 == 1),);
        }
    }
}
