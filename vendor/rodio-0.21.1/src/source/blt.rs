use crate::common::{ChannelCount, Float, SampleRate};
use crate::math::PI;
use crate::{Sample, Source};
use std::time::Duration;

use super::SeekError;

// Implemented following http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt

/// Internal function that builds a `BltFilter` object.
pub fn low_pass<I>(input: I, freq: u32) -> BltFilter<I>
where
    I: Source<Item = Sample>,
{
    low_pass_with_q(input, freq, 0.5)
}

pub fn high_pass<I>(input: I, freq: u32) -> BltFilter<I>
where
    I: Source<Item = Sample>,
{
    high_pass_with_q(input, freq, 0.5)
}

/// Same as low_pass but allows the q value (bandwidth) to be changed
pub fn low_pass_with_q<I>(input: I, freq: u32, q: Float) -> BltFilter<I>
where
    I: Source<Item = Sample>,
{
    BltFilter {
        input,
        formula: BltFormula::LowPass { freq, q },
        applier: None,
        x_n1: 0.0,
        x_n2: 0.0,
        y_n1: 0.0,
        y_n2: 0.0,
    }
}

/// Same as high_pass but allows the q value (bandwidth) to be changed
pub fn high_pass_with_q<I>(input: I, freq: u32, q: Float) -> BltFilter<I>
where
    I: Source<Item = Sample>,
{
    BltFilter {
        input,
        formula: BltFormula::HighPass { freq, q },
        applier: None,
        x_n1: 0.0,
        x_n2: 0.0,
        y_n1: 0.0,
        y_n2: 0.0,
    }
}

/// This applies an audio filter, it can be a high or low pass filter.
#[derive(Clone, Debug)]
pub struct BltFilter<I> {
    input: I,
    formula: BltFormula,
    applier: Option<BltApplier>,
    x_n1: Float,
    x_n2: Float,
    y_n1: Float,
    y_n2: Float,
}

impl<I> BltFilter<I> {
    /// Modifies this filter so that it becomes a low-pass filter.
    pub fn to_low_pass(&mut self, freq: u32) {
        self.to_low_pass_with_q(freq, 0.5);
    }

    /// Modifies this filter so that it becomes a high-pass filter
    pub fn to_high_pass(&mut self, freq: u32) {
        self.to_high_pass_with_q(freq, 0.5);
    }

    /// Same as to_low_pass but allows the q value (bandwidth) to be changed
    pub fn to_low_pass_with_q(&mut self, freq: u32, q: Float) {
        self.formula = BltFormula::LowPass { freq, q };
        self.applier = None;
    }

    /// Same as to_high_pass but allows the q value (bandwidth) to be changed
    pub fn to_high_pass_with_q(&mut self, freq: u32, q: Float) {
        self.formula = BltFormula::HighPass { freq, q };
        self.applier = None;
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

impl<I> Iterator for BltFilter<I>
where
    I: Source<Item = Sample>,
{
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Sample> {
        let last_in_span = self.input.current_span_len() == Some(1);

        if self.applier.is_none() {
            self.applier = Some(self.formula.to_applier(self.input.sample_rate().get()));
        }

        let sample = self.input.next()?;
        let result = self
            .applier
            .as_ref()
            .unwrap()
            .apply(sample, self.x_n1, self.x_n2, self.y_n1, self.y_n2);

        self.y_n2 = self.y_n1;
        self.x_n2 = self.x_n1;
        self.y_n1 = result;
        self.x_n1 = sample;

        if last_in_span {
            self.applier = None;
        }

        Some(result)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for BltFilter<I> where I: Source<Item = Sample> + ExactSizeIterator {}

impl<I> Source for BltFilter<I>
where
    I: Source<Item = Sample>,
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

#[derive(Clone, Debug)]
enum BltFormula {
    LowPass { freq: u32, q: Float },
    HighPass { freq: u32, q: Float },
}

impl BltFormula {
    fn to_applier(&self, sampling_frequency: u32) -> BltApplier {
        match *self {
            BltFormula::LowPass { freq, q } => {
                let w0 = 2.0 * PI * freq as Float / sampling_frequency as Float;

                let alpha = w0.sin() / (2.0 * q);
                let b1 = 1.0 - w0.cos();
                let b0 = b1 / 2.0;
                let b2 = b0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * w0.cos();
                let a2 = 1.0 - alpha;

                BltApplier {
                    b0: b0 / a0,
                    b1: b1 / a0,
                    b2: b2 / a0,
                    a1: a1 / a0,
                    a2: a2 / a0,
                }
            }
            BltFormula::HighPass { freq, q } => {
                let w0 = 2.0 * PI * freq as Float / sampling_frequency as Float;
                let cos_w0 = w0.cos();
                let alpha = w0.sin() / (2.0 * q);

                let b0 = (1.0 + cos_w0) / 2.0;
                let b1 = -1.0 - cos_w0;
                let b2 = b0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w0;
                let a2 = 1.0 - alpha;

                BltApplier {
                    b0: b0 / a0,
                    b1: b1 / a0,
                    b2: b2 / a0,
                    a1: a1 / a0,
                    a2: a2 / a0,
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct BltApplier {
    b0: Float,
    b1: Float,
    b2: Float,
    a1: Float,
    a2: Float,
}

impl BltApplier {
    #[inline]
    fn apply(&self, x_n: Float, x_n1: Float, x_n2: Float, y_n1: Float, y_n2: Float) -> Float {
        self.b0 * x_n + self.b1 * x_n1 + self.b2 * x_n2 - self.a1 * y_n1 - self.a2 * y_n2
    }
}
