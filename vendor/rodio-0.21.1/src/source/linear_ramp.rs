use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::math::{duration_to_float, NANOS_PER_SEC};
use crate::{Float, Source};

/// Internal function that builds a `LinearRamp` object.
pub fn linear_gain_ramp<I>(
    input: I,
    duration: Duration,
    start_gain: Float,
    end_gain: Float,
    clamp_end: bool,
) -> LinearGainRamp<I>
where
    I: Source,
{
    assert!(!duration.is_zero(), "duration must be greater than zero");

    LinearGainRamp {
        input,
        elapsed: Duration::ZERO,
        total: duration,
        start_gain,
        end_gain,
        clamp_end,
        sample_idx: 0u64,
    }
}

/// Filter that adds a linear gain ramp to the source over a given time range.
#[derive(Clone, Debug)]
pub struct LinearGainRamp<I> {
    input: I,
    elapsed: Duration,
    total: Duration,
    start_gain: Float,
    end_gain: Float,
    clamp_end: bool,
    sample_idx: u64,
}

impl<I> LinearGainRamp<I>
where
    I: Source,
{
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

impl<I> Iterator for LinearGainRamp<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let factor: Float;

        if self.elapsed >= self.total {
            if self.clamp_end {
                factor = self.end_gain;
            } else {
                factor = 1.0;
            }
        } else {
            self.sample_idx += 1;

            // Calculate progress (0.0 to 1.0) using appropriate precision for Float type
            let p = duration_to_float(self.elapsed) / duration_to_float(self.total);

            factor = self.start_gain * (1.0 - p) + self.end_gain * p;
        }

        if self.sample_idx.is_multiple_of(self.channels().get() as u64) {
            let sample_duration =
                Duration::from_nanos(NANOS_PER_SEC / self.input.sample_rate().get() as u64);
            self.elapsed += sample_duration;
        }

        self.input.next().map(|value| value * factor)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for LinearGainRamp<I> where I: Source + ExactSizeIterator {}

impl<I> Source for LinearGainRamp<I>
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
        self.elapsed = pos;
        self.input.try_seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::buffer::SamplesBuffer;
    use crate::math::nz;
    use crate::Sample;

    /// Create a SamplesBuffer of identical samples with value `value`.
    /// Returned buffer is one channel and has a sample rate of 1 hz.
    fn const_source(length: u8, value: Sample) -> SamplesBuffer {
        let data: Vec<Sample> = (1..=length).map(|_| value).collect();
        SamplesBuffer::new(nz!(1), nz!(1), data)
    }

    /// Create a SamplesBuffer of repeating sample values from `values`.
    fn cycle_source(length: u8, values: Vec<Sample>) -> SamplesBuffer {
        let data: Vec<Sample> = (1..=length)
            .enumerate()
            .map(|(i, _)| values[i % values.len()])
            .collect();

        SamplesBuffer::new(nz!(1), nz!(1), data)
    }

    #[test]
    fn test_linear_ramp() {
        let source1 = const_source(10, 1.0);
        let mut faded = linear_gain_ramp(source1, Duration::from_secs(4), 0.0, 1.0, true);

        assert_eq!(faded.next(), Some(0.0));
        assert_eq!(faded.next(), Some(0.25));
        assert_eq!(faded.next(), Some(0.5));
        assert_eq!(faded.next(), Some(0.75));
        assert_eq!(faded.next(), Some(1.0));
        assert_eq!(faded.next(), Some(1.0));
        assert_eq!(faded.next(), Some(1.0));
        assert_eq!(faded.next(), Some(1.0));
        assert_eq!(faded.next(), Some(1.0));
        assert_eq!(faded.next(), Some(1.0));
        assert_eq!(faded.next(), None);
    }

    #[test]
    fn test_linear_ramp_clamped() {
        let source1 = const_source(10, 1.0);
        let mut faded = linear_gain_ramp(source1, Duration::from_secs(4), 0.0, 0.5, true);

        assert_eq!(faded.next(), Some(0.0)); // fading in...
        assert_eq!(faded.next(), Some(0.125));
        assert_eq!(faded.next(), Some(0.25));
        assert_eq!(faded.next(), Some(0.375));
        assert_eq!(faded.next(), Some(0.5)); // fade is done
        assert_eq!(faded.next(), Some(0.5));
        assert_eq!(faded.next(), Some(0.5));
        assert_eq!(faded.next(), Some(0.5));
        assert_eq!(faded.next(), Some(0.5));
        assert_eq!(faded.next(), Some(0.5));
        assert_eq!(faded.next(), None);
    }

    #[test]
    fn test_linear_ramp_seek() {
        let source1 = cycle_source(20, vec![0.0, 0.4, 0.8]);
        let mut faded = linear_gain_ramp(source1, Duration::from_secs(10), 0.0, 1.0, true);

        assert_abs_diff_eq!(faded.next().unwrap(), 0.0); // source value 0
        assert_abs_diff_eq!(faded.next().unwrap(), 0.04); // source value 0.4, ramp gain 0.1
        assert_abs_diff_eq!(faded.next().unwrap(), 0.16); // source value 0.8, ramp gain 0.2

        if let Ok(_result) = faded.try_seek(Duration::from_secs(5)) {
            assert_abs_diff_eq!(faded.next().unwrap(), 0.40); // source value 0.8, ramp gain 0.5
            assert_abs_diff_eq!(faded.next().unwrap(), 0.0); // source value 0, ramp gain 0.6
            assert_abs_diff_eq!(faded.next().unwrap(), 0.28); // source value 0.4. ramp gain 0.7
        } else {
            panic!("try_seek() failed!");
        }

        if let Ok(_result) = faded.try_seek(Duration::from_secs(0)) {
            assert_abs_diff_eq!(faded.next().unwrap(), 0.0); // source value 0, ramp gain 0.0
            assert_abs_diff_eq!(faded.next().unwrap(), 0.04); // source value 0.4, ramp gain 0.1
            assert_abs_diff_eq!(faded.next().unwrap(), 0.16); // source value 0.8. ramp gain 0.2
        } else {
            panic!("try_seek() failed!");
        }

        if let Ok(_result) = faded.try_seek(Duration::from_secs(10)) {
            assert_abs_diff_eq!(faded.next().unwrap(), 0.4); // source value 0.4, ramp gain 1.0
            assert_abs_diff_eq!(faded.next().unwrap(), 0.8); // source value 0.8, ramp gain 1.0
            assert_abs_diff_eq!(faded.next().unwrap(), 0.0); // source value 0. ramp gain 1.0
        } else {
            panic!("try_seek() failed!");
        }
    }
}
