//! Generator sources for various periodic test waveforms.
//!
//! This module provides several periodic, deterministic waveforms for testing other sources and
//! for simple additive sound synthesis. Every source is monoaural and in the codomain [-1.0, 1.0].
//!
//! # Example
//!
//! ```
//! use rodio::source::{SignalGenerator,Function};
//! use core::num::NonZero;
//!
//! let tone = SignalGenerator::new(NonZero::new(48000).unwrap(), 440.0, Function::Sine);
//! ```
use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::math::{duration_to_float, nz, TAU};
use crate::{Float, Sample, Source};
use std::time::Duration;

/// Generator function.
///
/// A generator function is the core of a signal generator, the `SignalGenerator` type uses these
/// function to create periodic waveforms.
///
/// # Arguments
///  *  A `Float` representing a time in the signal to generate. The scale of this variable is
///     normalized to the period of the signal, such that "0.0" is time zero, "1.0" is one period of
///     the signal, "2.0" is two periods and so on. This function should be written to accept any
///     float in the range (`Float::MIN`, `Float::MAX`) but `SignalGenerator` will only pass values in
///     (0.0, 1.0) to mitigate floating point error.
///
/// # Returns
///
/// A `Sample` (Float) representing the signal level at the passed time. This value should be normalized
/// in the range [-1.0,1.0].
pub type GeneratorFunction = fn(Float) -> Sample;

/// Waveform functions.
#[derive(Clone, Debug)]
pub enum Function {
    /// A sinusoidal waveform.
    Sine,
    /// A triangle waveform.
    Triangle,
    /// A square wave, rising edge at t=0.
    Square,
    /// A rising sawtooth wave.
    Sawtooth,
}

fn sine_signal(phase: Float) -> Sample {
    (TAU * phase).sin()
}

fn triangle_signal(phase: Float) -> Sample {
    4.0 * (phase - (phase + 0.5).floor()).abs() - 1.0
}

fn square_signal(phase: Float) -> Sample {
    if phase % 1.0 < 0.5 {
        1.0
    } else {
        -1.0
    }
}

fn sawtooth_signal(phase: Float) -> Sample {
    2.0 * (phase - (phase + 0.5).floor())
}

/// An infinite source that produces one of a selection of test waveforms.
#[derive(Clone, Debug)]
pub struct SignalGenerator {
    sample_rate: SampleRate,
    function: GeneratorFunction,
    phase_step: Float,
    phase: Float,
    period: Float,
}

impl SignalGenerator {
    /// Create a new `SignalGenerator` object that generates an endless waveform
    /// `f`.
    ///
    /// # Panics
    ///
    /// Will panic if `frequency` is equal to zero.
    #[inline]
    pub fn new(sample_rate: SampleRate, frequency: f32, f: Function) -> Self {
        let function: GeneratorFunction = match f {
            Function::Sine => sine_signal,
            Function::Triangle => triangle_signal,
            Function::Square => square_signal,
            Function::Sawtooth => sawtooth_signal,
        };

        Self::with_function(sample_rate, frequency, function)
    }

    /// Create a new `SignalGenerator` object that generates an endless waveform
    /// from the [generator function](crate::source::signal_generator::GeneratorFunction) `generator_function`.
    ///
    /// # Panics
    ///
    /// Will panic if `frequency` is equal to zero.
    #[inline]
    pub fn with_function(
        sample_rate: SampleRate,
        frequency: f32,
        generator_function: GeneratorFunction,
    ) -> Self {
        assert!(frequency > 0.0, "frequency must be greater than zero");
        let period = sample_rate.get() as Float / frequency as Float;
        let phase_step = 1.0 / period;

        SignalGenerator {
            sample_rate,
            function: generator_function,
            phase_step,
            phase: 0.0,
            period,
        }
    }
}

impl Iterator for SignalGenerator {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Sample> {
        let f = self.function;
        let val = Some(f(self.phase));
        self.phase = (self.phase + self.phase_step).rem_euclid(1.0);
        val
    }
}

impl Source for SignalGenerator {
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
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }

    #[inline]
    fn try_seek(&mut self, duration: Duration) -> Result<(), SeekError> {
        let seek = duration_to_float(duration) * (self.sample_rate.get() as Float) / self.period;
        self.phase = seek.rem_euclid(1.0);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::nz;
    use crate::source::{Function, SignalGenerator};
    use crate::Sample;
    use approx::assert_abs_diff_eq;

    const TEST_EPSILON: Sample = 0.0001;

    #[test]
    fn square() {
        let mut wf = SignalGenerator::new(nz!(2000), 500.0, Function::Square);
        assert_eq!(wf.next(), Some(1.0));
        assert_eq!(wf.next(), Some(1.0));
        assert_eq!(wf.next(), Some(-1.0));
        assert_eq!(wf.next(), Some(-1.0));
        assert_eq!(wf.next(), Some(1.0));
        assert_eq!(wf.next(), Some(1.0));
        assert_eq!(wf.next(), Some(-1.0));
        assert_eq!(wf.next(), Some(-1.0));
    }

    #[test]
    fn triangle() {
        let mut wf = SignalGenerator::new(nz!(8000), 1000.0, Function::Triangle);
        assert_eq!(wf.next(), Some(-1.0));
        assert_eq!(wf.next(), Some(-0.5));
        assert_eq!(wf.next(), Some(0.0));
        assert_eq!(wf.next(), Some(0.5));
        assert_eq!(wf.next(), Some(1.0));
        assert_eq!(wf.next(), Some(0.5));
        assert_eq!(wf.next(), Some(0.0));
        assert_eq!(wf.next(), Some(-0.5));
        assert_eq!(wf.next(), Some(-1.0));
        assert_eq!(wf.next(), Some(-0.5));
        assert_eq!(wf.next(), Some(0.0));
        assert_eq!(wf.next(), Some(0.5));
        assert_eq!(wf.next(), Some(1.0));
        assert_eq!(wf.next(), Some(0.5));
        assert_eq!(wf.next(), Some(0.0));
        assert_eq!(wf.next(), Some(-0.5));
    }

    #[test]
    fn saw() {
        let mut wf = SignalGenerator::new(nz!(200), 50.0, Function::Sawtooth);
        assert_eq!(wf.next(), Some(0.0));
        assert_eq!(wf.next(), Some(0.5));
        assert_eq!(wf.next(), Some(-1.0));
        assert_eq!(wf.next(), Some(-0.5));
        assert_eq!(wf.next(), Some(0.0));
        assert_eq!(wf.next(), Some(0.5));
        assert_eq!(wf.next(), Some(-1.0));
    }

    #[test]
    fn sine() {
        let mut wf = SignalGenerator::new(nz!(1000), 100f32, Function::Sine);

        assert_abs_diff_eq!(wf.next().unwrap(), 0.0, epsilon = TEST_EPSILON);
        assert_abs_diff_eq!(wf.next().unwrap(), 0.58778525, epsilon = TEST_EPSILON);
        assert_abs_diff_eq!(wf.next().unwrap(), 0.95105652, epsilon = TEST_EPSILON);
        assert_abs_diff_eq!(wf.next().unwrap(), 0.95105652, epsilon = TEST_EPSILON);
        assert_abs_diff_eq!(wf.next().unwrap(), 0.58778525, epsilon = TEST_EPSILON);
        assert_abs_diff_eq!(wf.next().unwrap(), 0.0, epsilon = TEST_EPSILON);
        assert_abs_diff_eq!(wf.next().unwrap(), -0.58778554, epsilon = TEST_EPSILON);
    }
}
