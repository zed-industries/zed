//! Dithering for audio quantization and requantization.
//!
//! Dithering is a technique in digital audio processing that eliminates quantization
//! artifacts during various stages of audio processing. This module provides tools for
//! adding appropriate dither noise to maintain audio quality during quantization
//! operations.
//!
//! ## Example
//!
//! ```rust
//! use rodio::source::{DitherAlgorithm, SineWave};
//! use rodio::{BitDepth, Source};
//!
//! let source = SineWave::new(440.0);
//! let dithered = source.dither(BitDepth::new(16).unwrap(), DitherAlgorithm::TPDF);
//! ```
//!
//! ## Guidelines
//!
//! - **Apply dithering before volume changes** for optimal results
//! - **Dither once** - Apply only at the final output stage to avoid noise accumulation
//! - **Choose TPDF** for most professional audio applications (it's the default)
//! - **Use target output bit depth** - Not the source bit depth!
//!
//! When you later change volume (e.g., with `Player::set_volume()`), both the signal
//! and dither noise scale together, maintaining proper dithering behavior.

use rand::{rngs::SmallRng, Rng};
use std::time::Duration;

use crate::{
    source::noise::{Blue, WhiteGaussian, WhiteTriangular, WhiteUniform},
    BitDepth, ChannelCount, Float, Sample, SampleRate, Source,
};

/// Dither algorithm selection for runtime choice
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    /// GPDF (Gaussian PDF) - normal/bell curve distribution.
    ///
    /// Uses Gaussian white noise which more closely mimics natural processes and
    /// analog circuits. Higher noise floor than TPDF.
    GPDF,

    /// High-pass dithering - reduces low-frequency artifacts.
    ///
    /// Uses blue noise (high-pass filtered white noise) to push dither energy
    /// toward higher frequencies. Particularly effective for reducing audible
    /// low-frequency modulation artifacts.
    HighPass,

    /// RPDF (Rectangular PDF) - uniform distribution.
    ///
    /// Uses uniform white noise for basic decorrelation. Simpler than TPDF but
    /// allows some correlation between signal and quantization error at low levels.
    /// Slightly lower noise floor than TPDF.
    RPDF,

    /// TPDF (Triangular PDF) - triangular distribution.
    ///
    /// The gold standard for audio dithering. Provides mathematically optimal
    /// decorrelation by completely eliminating correlation between the original
    /// signal and quantization error.
    #[default]
    TPDF,
}

#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
enum NoiseGenerator<R: Rng = SmallRng> {
    TPDF(WhiteTriangular<R>),
    RPDF(WhiteUniform<R>),
    GPDF(WhiteGaussian<R>),
    HighPass(Vec<Blue<R>>),
}

impl NoiseGenerator {
    fn new(algorithm: Algorithm, sample_rate: SampleRate, channels: ChannelCount) -> Self {
        match algorithm {
            Algorithm::TPDF => Self::TPDF(WhiteTriangular::new(sample_rate)),
            Algorithm::RPDF => Self::RPDF(WhiteUniform::new(sample_rate)),
            Algorithm::GPDF => Self::GPDF(WhiteGaussian::new(sample_rate)),
            Algorithm::HighPass => {
                // Create per-channel generators for HighPass to prevent prev_white state from
                // crossing channel boundaries in interleaved audio. Each channel must have an
                // independent RNG to avoid correlation. Use this iterator instead of the `vec!`
                // macro to avoid cloning the RNG.
                Self::HighPass(
                    (0..channels.get())
                        .map(|_| Blue::new(sample_rate))
                        .collect(),
                )
            }
        }
    }

    #[inline]
    fn next(&mut self, channel: usize) -> Option<Sample> {
        match self {
            Self::TPDF(gen) => gen.next(),
            Self::RPDF(gen) => gen.next(),
            Self::GPDF(gen) => gen.next(),
            Self::HighPass(gens) => gens[channel].next(),
        }
    }

    #[inline]
    fn algorithm(&self) -> Algorithm {
        match self {
            Self::TPDF(_) => Algorithm::TPDF,
            Self::RPDF(_) => Algorithm::RPDF,
            Self::GPDF(_) => Algorithm::GPDF,
            Self::HighPass(_) => Algorithm::HighPass,
        }
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        match self {
            Self::TPDF(gen) => gen.sample_rate(),
            Self::RPDF(gen) => gen.sample_rate(),
            Self::GPDF(gen) => gen.sample_rate(),
            Self::HighPass(gens) => gens
                .first()
                .map(|g| g.sample_rate())
                .expect("HighPass should have at least one generator"),
        }
    }

    #[inline]
    fn update_parameters(&mut self, sample_rate: SampleRate, channels: ChannelCount) {
        if self.sample_rate() != sample_rate {
            // The noise generators that we use are currently not dependent on sample rate,
            // but we recreate them anyway in case that changes in the future.
            *self = Self::new(self.algorithm(), sample_rate, channels);
        } else if let Self::HighPass(gens) = self {
            // Sample rate unchanged - only adjust channel count for stateful algorithms
            // resize_with is a no-op if the size hasn't changed
            gens.resize_with(channels.get() as usize, || Blue::new(sample_rate));
        }
    }
}

/// A dithered audio source that applies quantization noise to reduce artifacts.
///
/// This struct wraps any audio source and applies dithering noise according to the
/// selected algorithm. Dithering is essential for digital audio playback and when
/// converting audio to different bit depths to prevent audible distortion.
///
/// # Example
///
/// ```rust
/// use rodio::source::{DitherAlgorithm, SineWave};
/// use rodio::{BitDepth, Source};
///
/// let source = SineWave::new(440.0);
/// let dithered = source.dither(BitDepth::new(16).unwrap(), DitherAlgorithm::TPDF);
/// ```
#[derive(Clone, Debug)]
pub struct Dither<I> {
    input: I,
    noise: NoiseGenerator,
    current_channel: usize,
    remaining_in_span: Option<usize>,
    lsb_amplitude: Float,
}

impl<I> Dither<I>
where
    I: Source,
{
    /// Creates a new dithered source with the specified algorithm
    pub fn new(input: I, target_bits: BitDepth, algorithm: Algorithm) -> Self {
        // LSB amplitude for signed audio: 1.0 / (2^(bits-1))
        // Using f64 intermediate prevents precision loss and u64 handles all bit depths without
        // overflow (64-bit being the theoretical maximum for audio samples). Values stay well
        // above f32 denormal threshold, avoiding denormal arithmetic performance penalty.
        let lsb_amplitude = (1.0 / (1_u64 << (target_bits.get() - 1)) as f64) as Float;

        let sample_rate = input.sample_rate();
        let channels = input.channels();
        let active_span_len = input.current_span_len();

        Self {
            input,
            noise: NoiseGenerator::new(algorithm, sample_rate, channels),
            current_channel: 0,
            remaining_in_span: active_span_len,
            lsb_amplitude,
        }
    }

    /// Change the dithering algorithm at runtime
    pub fn set_algorithm(&mut self, algorithm: Algorithm) {
        if self.noise.algorithm() != algorithm {
            self.noise =
                NoiseGenerator::new(algorithm, self.input.sample_rate(), self.input.channels());
        }
    }

    /// Get the current dithering algorithm
    #[inline]
    pub fn algorithm(&self) -> Algorithm {
        self.noise.algorithm()
    }
}

impl<I> Iterator for Dither<I>
where
    I: Source,
{
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remaining) = self.remaining_in_span {
            *remaining = remaining.saturating_sub(1);
        }

        // Consume next input sample *after* decrementing span position and *before* checking for
        // span boundary crossing. This ensures that the source has its parameters updated
        // correctly before we generate noise for the next sample.
        let input_sample = self.input.next()?;
        let num_channels = self.input.channels();

        if self.remaining_in_span == Some(0) {
            self.noise
                .update_parameters(self.input.sample_rate(), num_channels);
            self.current_channel = 0;
            self.remaining_in_span = self.input.current_span_len();
        }

        let noise_sample = self
            .noise
            .next(self.current_channel)
            .expect("Noise generator should always produce samples");

        // Advance to next channel (wrapping around)
        self.current_channel = (self.current_channel + 1) % num_channels.get() as usize;

        // Apply subtractive dithering at the target quantization level
        Some(input_sample - noise_sample * self.lsb_amplitude)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for Dither<I> where I: Source + ExactSizeIterator {}

impl<I> Source for Dither<I>
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
    fn try_seek(&mut self, pos: Duration) -> Result<(), crate::source::SeekError> {
        self.input.try_seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{SineWave, Source};
    use crate::{nz, BitDepth, SampleRate};

    const TEST_SAMPLE_RATE: SampleRate = nz!(44100);
    const TEST_BIT_DEPTH: BitDepth = nz!(16);

    #[test]
    fn test_dither_adds_noise() {
        let source = SineWave::new(440.0).take_duration(std::time::Duration::from_millis(10));
        let mut dithered = Dither::new(source.clone(), TEST_BIT_DEPTH, Algorithm::TPDF);
        let mut undithered = source;

        // Collect samples from both sources
        let dithered_samples: Vec<Sample> = (0..10).filter_map(|_| dithered.next()).collect();
        let undithered_samples: Vec<Sample> = (0..10).filter_map(|_| undithered.next()).collect();

        let lsb = 1.0 / (1_i64 << (TEST_BIT_DEPTH.get() - 1)) as Float;

        // Verify dithered samples differ from undithered and are reasonable
        for (i, (&dithered_sample, &undithered_sample)) in dithered_samples
            .iter()
            .zip(undithered_samples.iter())
            .enumerate()
        {
            // Should be finite
            assert!(
                dithered_sample.is_finite(),
                "Dithered sample {} should be finite",
                i
            );

            // The difference should be small (just dither noise)
            let diff = (dithered_sample - undithered_sample).abs();
            let max_expected_diff = lsb * 2.0; // Max triangular dither amplitude
            assert!(
                diff <= max_expected_diff,
                "Dither noise too large: sample {}, diff {}, max expected {}",
                i,
                diff,
                max_expected_diff
            );
        }
    }

    #[test]
    fn test_highpass_dither_multichannel_independence() {
        use crate::source::Zero;

        // Create a stereo source that outputs zeros
        // This makes it easy to extract just the dither noise
        let constant_source = Zero::new(nz!(2), TEST_SAMPLE_RATE);

        // Apply HighPass dithering to stereo
        let mut dithered = Dither::new(constant_source, TEST_BIT_DEPTH, Algorithm::HighPass);

        // Collect interleaved samples (L, R, L, R, ...)
        let samples: Vec<Sample> = dithered.by_ref().take(1000).collect();

        // De-interleave into left and right channels
        let left: Vec<Sample> = samples.iter().step_by(2).copied().collect();
        let right: Vec<Sample> = samples.iter().skip(1).step_by(2).copied().collect();

        assert_eq!(left.len(), 500);
        assert_eq!(right.len(), 500);

        // Calculate autocorrelation at lag 1 for each channel
        // Blue noise (high-pass) should have negative correlation at lag 1
        let left_autocorr: Float =
            left.windows(2).map(|w| w[0] * w[1]).sum::<Float>() / (left.len() - 1) as Float;

        let right_autocorr: Float =
            right.windows(2).map(|w| w[0] * w[1]).sum::<Float>() / (right.len() - 1) as Float;

        // Blue noise should have negative autocorrelation (high-pass characteristic)
        // If channels were cross-contaminated, this property would be broken
        assert!(
            left_autocorr < 0.0,
            "Left channel should have negative autocorr (high-pass), got {}",
            left_autocorr
        );
        assert!(
            right_autocorr < 0.0,
            "Right channel should have negative autocorr (high-pass), got {}",
            right_autocorr
        );

        // Channels should be independent - cross-correlation between L and R should be near zero
        let cross_corr: Float = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| l * r)
            .sum::<Float>()
            / left.len() as Float;

        assert!(
            cross_corr.abs() < 0.1,
            "Channels should be independent, cross-correlation should be near 0, got {}",
            cross_corr
        );
    }
}
