//! Noise sources for audio synthesis and testing.
//!
//! ## Available Noise Types
//!
//! | **Noise Type** | **Best For** | **Sound Character** | **Technical Notes* |
//! |----------------|--------------|---------------------|--------------------|
//! | **White noise** | Testing equipment linearly, masking sounds | Harsh, static-like, evenly bright | RPDF (uniform), equal power all frequencies |
//! | **Gaussian white** | Scientific modeling, natural processes | Similar to white but more natural | GPDF (bell curve), better statistical properties |
//! | **Triangular white** | High-quality audio dithering | Similar to white noise | TPDF, eliminates quantization correlation |
//! | **Pink noise** | Speaker testing, calibration, background sounds | Warm, pleasant, like rainfall | 1/f spectrum, matches human hearing |
//! | **Blue noise** | High-passed dithering, reducing low-frequency artifacts | Bright but smoother than white | High-pass filtered white, less harsh |
//! | **Violet noise** | Testing high frequencies, harsh effects | Very bright, sharp, can be piercing | Heavy high-frequency emphasis |
//! | **Brownian noise** | Scientific modeling of Brownian motion | Very deep, muffled, lacks highs | True stochastic process, Gaussian increments |
//! | **Red noise** | Practical deep/muffled effects, distant rumbles | Very deep, muffled, lacks highs | 1/f² spectrum, uniform input |
//! | **Velvet noise** | Artificial reverb, room simulation | Sparse random impulses | Computationally efficient, decorrelated |
//!
//! ## Basic Usage
//!
//! ```rust
//! use std::num::NonZero;
//! use rodio::source::noise::{WhiteUniform, Pink, WhiteTriangular, Blue, Red};
//! use rodio::SampleRate;
//!
//! let sample_rate = NonZero::new(44100).unwrap();
//!
//! // Simple usage - creates generators with `SmallRng`
//!
//! // For testing equipment linearly
//! let white = WhiteUniform::new(sample_rate);
//! // For pleasant background sound
//! let pink = Pink::new(sample_rate);
//! // For TPDF dithering
//! let triangular = WhiteTriangular::new(sample_rate);
//! // For high-passed dithering applications
//! let blue = Blue::new(sample_rate);
//! // For practical deep/muffled effects
//! let red = Red::new(sample_rate);
//!
//! // Advanced usage - specify your own RNG type
//! use rand::{rngs::StdRng, SeedableRng};
//! let white_custom = WhiteUniform::<StdRng>::new_with_rng(sample_rate, StdRng::seed_from_u64(12345));
//! ```

use std::{num::NonZero, time::Duration};

use rand::{
    distr::{Distribution, Uniform},
    rngs::SmallRng,
    Rng, SeedableRng,
};
use rand_distr::{Normal, Triangular};

use crate::math::{nz, PI};
use crate::{ChannelCount, Float, Sample, SampleRate, Source};

/// Convenience function to create a new `WhiteUniform` noise source.
#[deprecated(since = "0.21.0", note = "use WhiteUniform::new() instead")]
pub fn white(sample_rate: SampleRate) -> WhiteUniform<SmallRng> {
    WhiteUniform::new(sample_rate)
}

/// Convenience function to create a new `Pink` noise source.
#[deprecated(since = "0.21.0", note = "use Pink::new() instead")]
pub fn pink(sample_rate: SampleRate) -> Pink<SmallRng> {
    Pink::new(sample_rate)
}

/// Macro to implement the basic `Source` trait for mono noise generators with stateless seeking
/// support.
macro_rules! impl_noise_source {
    ($type:ty) => {
        impl<R: Rng> Source for $type {
            fn current_span_len(&self) -> Option<usize> {
                None
            }

            fn channels(&self) -> ChannelCount {
                nz!(1)
            }

            fn sample_rate(&self) -> SampleRate {
                self.sample_rate
            }

            fn total_duration(&self) -> Option<Duration> {
                None
            }

            fn try_seek(&mut self, _pos: Duration) -> Result<(), crate::source::SeekError> {
                // Stateless noise generators can seek to any position since all positions
                // are equally random and don't depend on previous state
                Ok(())
            }
        }
    };
}

/// Common sampling utilities for noise generators.
/// Provides a generic interface for sampling from any distribution.
#[derive(Clone, Debug)]
struct NoiseSampler<R: Rng, D: Distribution<Sample> + Clone> {
    rng: R,
    distribution: D,
}

impl<R: Rng, D: Distribution<Sample> + Clone> NoiseSampler<R, D> {
    /// Create a new sampler with the given distribution.
    fn new(rng: R, distribution: D) -> Self {
        Self { rng, distribution }
    }

    /// Generate a sample from the configured distribution.
    #[inline]
    fn sample(&mut self) -> Sample {
        self.rng.sample(&self.distribution)
    }
}

/// Generates an infinite stream of uniformly distributed white noise samples in [-1.0, 1.0].
/// White noise generator - sounds like radio static (RPDF).
///
/// Generates uniformly distributed random samples with equal power at all frequencies. This is the
/// most basic noise type and serves as a building block for other noise generators. Uses RPDF
/// (Rectangular Probability Density Function) - uniform distribution.
///
/// **When to use:** Audio equipment testing, sound masking, or as a base for other effects.
///
/// **Sound:** Harsh, bright, evenly distributed across all frequencies.
#[derive(Clone, Debug)]
pub struct WhiteUniform<R: Rng = SmallRng> {
    sample_rate: SampleRate,
    sampler: NoiseSampler<R, Uniform<Sample>>,
}

impl WhiteUniform<SmallRng> {
    /// Create a new white noise generator with `SmallRng` seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> WhiteUniform<R> {
    /// Create a new white noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        let distribution =
            Uniform::new_inclusive(-1.0, 1.0).expect("Failed to create uniform distribution");

        Self {
            sample_rate,
            sampler: NoiseSampler::new(rng, distribution),
        }
    }

    /// Get the standard deviation of the uniform distribution.
    ///
    /// For uniform distribution [-1.0, 1.0], std_dev = √(1/3) ≈ 0.5774.
    pub fn std_dev(&self) -> Sample {
        UNIFORM_VARIANCE.sqrt()
    }
}

impl<R: Rng> Iterator for WhiteUniform<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.sampler.sample())
    }
}

impl_noise_source!(WhiteUniform<R>);

/// Triangular white noise generator - ideal for TPDF dithering.
///
/// Generates triangular-distributed white noise by summing two uniform random samples. This
/// creates TPDF (Triangular Probability Density Function) which is superior to RPDF for audio
/// dithering because it completely eliminates correlation between the original signal and
/// quantization error.
///
/// **When to use:** High-quality audio dithering when reducing bit depth.
///
/// **Sound:** Similar to white noise but with better statistical properties.
///
/// **Distribution**: TPDF - triangular distribution from sum of two uniform samples.
#[derive(Clone, Debug)]
pub struct WhiteTriangular<R: Rng = SmallRng> {
    sample_rate: SampleRate,
    sampler: NoiseSampler<R, Triangular<Sample>>,
}

impl WhiteTriangular {
    /// Create a new triangular white noise generator with SmallRng seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> WhiteTriangular<R> {
    /// Create a new triangular white noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        let distribution = Triangular::new(-1.0, 1.0, 0.0).expect("Valid triangular distribution");

        Self {
            sample_rate,
            sampler: NoiseSampler::new(rng, distribution),
        }
    }

    /// Get the standard deviation of the triangular distribution.
    ///
    /// For triangular distribution [-1.0, 1.0] with mode 0.0, std_dev = 2/√6 ≈ 0.8165.
    pub fn std_dev(&self) -> Sample {
        2.0 / Sample::sqrt(6.0)
    }
}

impl<R: Rng> Iterator for WhiteTriangular<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.sampler.sample())
    }
}

impl_noise_source!(WhiteTriangular<R>);

/// Velvet noise generator - creates sparse random impulses, not continuous noise.
/// Also known as sparse noise or decorrelated noise.
///
/// Unlike other noise types, velvet noise produces random impulses separated by periods of
/// silence. Divides time into regular intervals and places one impulse randomly within each
/// interval.
///
/// **When to use:** Building reverb effects, room simulation, decorrelating audio channels.
///
/// **Sound:** Random impulses with silence between - smoother than continuous noise.
///
/// **Default:** 2000 impulses per second.
///
/// **Efficiency:** Very computationally efficient - mostly outputs zeros, only occasional
/// computation.
#[derive(Clone, Debug)]
pub struct Velvet<R: Rng = SmallRng> {
    sample_rate: SampleRate,
    rng: R,
    grid_size: usize,   // samples per grid cell
    grid_pos: usize,    // current position in grid cell
    impulse_pos: usize, // where impulse occurs in current grid
}

impl Velvet {
    /// Create a new velvet noise generator with SmallRng seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> Velvet<R> {
    /// Create a new velvet noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        Self::new_with_density(sample_rate, VELVET_DEFAULT_DENSITY, rng)
    }

    /// Create a new velvet noise generator with custom density (impulses per second) and RNG.
    ///
    /// **Density guidelines:**
    /// - 500-1000 Hz: Sparse, distant reverb effects
    /// - 1000-2000 Hz: Balanced reverb simulation (default: 2000 Hz)
    /// - 2000-4000 Hz: Dense, close reverb effects
    /// - >4000 Hz: Very dense, approaching continuous noise
    pub fn new_with_density(sample_rate: SampleRate, density: NonZero<usize>, mut rng: R) -> Self {
        let grid_size = (sample_rate.get() as f32 / density.get() as f32).ceil() as usize;
        let impulse_pos = if grid_size > 0 {
            rng.random_range(0..grid_size)
        } else {
            0
        };

        Self {
            sample_rate,
            rng,
            grid_size,
            grid_pos: 0,
            impulse_pos,
        }
    }
}

impl<R: Rng> Velvet<R> {}

impl<R: Rng> Iterator for Velvet<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let output = if self.grid_pos == self.impulse_pos {
            // Generate impulse with random polarity
            if self.rng.random::<bool>() {
                1.0
            } else {
                -1.0
            }
        } else {
            0.0
        };

        self.grid_pos = self.grid_pos.wrapping_add(1);

        // Start new grid cell when we reach the end
        if self.grid_pos >= self.grid_size {
            self.grid_pos = 0;
            self.impulse_pos = if self.grid_size > 0 {
                self.rng.random_range(0..self.grid_size)
            } else {
                0
            };
        }

        Some(output)
    }
}

impl_noise_source!(Velvet<R>);

/// Gaussian white noise generator - statistically perfect white noise (GPDF).
/// Also known as normal noise or bell curve noise.
///
/// Like regular white noise but with normal distribution (bell curve) instead of uniform. More
/// closely mimics analog circuits and natural processes, which typically follow bell curves. Uses
/// GPDF (Gaussian Probability Density Function) - 99.7% of samples within [-1.0, 1.0].
///
/// **When to use:** Modeling analog circuits, natural random processes, or when you need more
/// realistic noise that mimics how natural systems behave (most follow bell curves).
///
/// **Sound character**: Very similar to regular white noise, but with more analog-like character.
///
/// **vs White Noise:** Gaussian mimics natural/analog systems better, uniform white is faster and
/// simpler.
///
/// **Clipping Warning:** Can exceed [-1.0, 1.0] bounds. Consider attenuation or limiting if
/// clipping is critical.
#[derive(Clone, Debug)]
pub struct WhiteGaussian<R: Rng = SmallRng> {
    sample_rate: SampleRate,
    sampler: NoiseSampler<R, Normal<Sample>>,
}

impl<R: Rng> WhiteGaussian<R> {
    /// Get the mean (average) value of the noise distribution.
    pub fn mean(&self) -> Sample {
        self.sampler.distribution.mean()
    }

    /// Get the standard deviation of the noise distribution.
    pub fn std_dev(&self) -> Sample {
        self.sampler.distribution.std_dev()
    }
}

impl WhiteGaussian {
    /// Create a new Gaussian white noise generator with `SmallRng` seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> WhiteGaussian<R> {
    /// Create a new Gaussian white noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        // For Gaussian to achieve equivalent decorrelation to triangular dithering, it needs
        // 3-4 dB higher amplitude than TPDF's optimal 0.408 LSB. If optimizing:
        // - minimum correlation: σ ≈ 0.58
        // - perceptual equivalence: σ ≈ 0.65
        // - worst-case performance: σ ≈ 0.70
        //
        // σ = 0.6 LSB is a reasonable compromise that balances mathematical theory with
        // empirical performance across various signal types.
        let distribution = Normal::new(0.0, 0.6)
            .expect("Normal distribution with mean=0, std=0.6 should be valid");

        Self {
            sample_rate,
            sampler: NoiseSampler::new(rng, distribution),
        }
    }
}

impl<R: Rng> Iterator for WhiteGaussian<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.sampler.sample())
    }
}

impl_noise_source!(WhiteGaussian<R>);

/// Number of generators used in PinkNoise for frequency coverage.
///
/// The pink noise implementation uses the Voss-McCartney algorithm with 16 independent generators
/// to achieve proper 1/f frequency distribution. Each generator covers approximately one octave of
/// the frequency spectrum, providing smooth pink noise characteristics across the entire audio
/// range. 16 generators gives excellent frequency coverage for sample rates from 8kHz to 192kHz+
/// while maintaining computational efficiency.
const PINK_NOISE_GENERATORS: usize = 16;

/// Default impulse density for Velvet noise in impulses per second.
///
/// This provides a good balance between realistic reverb characteristics and computational
/// efficiency. Lower values create sparser, more distant reverb effects, while higher values
/// create denser, closer reverb simulation.
const VELVET_DEFAULT_DENSITY: NonZero<usize> = nz!(2000);

/// Variance of uniform distribution [-1.0, 1.0].
///
/// For uniform distribution U(-1, 1), the variance is (b-a)²/12 = 4/12 = 1/3.
const UNIFORM_VARIANCE: Sample = 1.0 / 3.0;

/// Pink noise generator - sounds much more natural than white noise.
///
/// Pink noise emphasizes lower frequencies, making it sound warmer and more pleasant than harsh
/// white noise. Often described as sounding like gentle rainfall or wind. Uses the
/// industry-standard Voss-McCartney algorithm with 16 generators.
///
/// **When to use:** Audio testing (matches human hearing better), pleasant background sounds,
/// speaker testing, or any time you want "natural" sounding noise.
///
/// **Sound:** Warmer, more pleasant than white noise - like distant rainfall.
///
/// **vs White Noise:** Pink sounds much more natural and less harsh to human ears.
///
/// Technical: 1/f frequency spectrum (power decreases 3dB per octave).
/// Works correctly at all sample rates from 8kHz to 192kHz+.
#[derive(Clone, Debug)]
pub struct Pink<R: Rng = SmallRng> {
    sample_rate: SampleRate,
    white_noise: WhiteUniform<R>,
    values: [Sample; PINK_NOISE_GENERATORS],
    counters: [u32; PINK_NOISE_GENERATORS],
    max_counts: [u32; PINK_NOISE_GENERATORS],
}

impl Pink {
    /// Create a new pink noise generator with `SmallRng` seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> Pink<R> {
    /// Create a new pink noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        let mut max_counts = [1u32; PINK_NOISE_GENERATORS];
        // Each generator updates at half the rate of the previous one: 1, 2, 4, 8, 16, ...
        for i in 1..PINK_NOISE_GENERATORS {
            max_counts[i] = max_counts[i - 1] * 2;
        }

        Self {
            sample_rate,
            white_noise: WhiteUniform::new_with_rng(sample_rate, rng),
            values: [0.0; PINK_NOISE_GENERATORS],
            counters: [0; PINK_NOISE_GENERATORS],
            max_counts,
        }
    }
}

impl<R: Rng> Iterator for Pink<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = 0.0;

        // Update each generator when its counter reaches the update interval
        for i in 0..PINK_NOISE_GENERATORS {
            if self.counters[i] >= self.max_counts[i] {
                // Time to update this generator with a new white noise sample
                self.values[i] = self
                    .white_noise
                    .next()
                    .expect("WhiteNoise should never return None");
                self.counters[i] = 0;
            }
            sum += self.values[i];
            self.counters[i] += 1;
        }

        // Normalize by number of generators to keep output in reasonable range
        Some(sum / PINK_NOISE_GENERATORS as Sample)
    }
}

impl_noise_source!(Pink<R>);

/// Blue noise generator - sounds brighter than white noise but smoother.
/// Also known as azure noise.
///
/// Blue noise emphasizes higher frequencies while distributing energy more evenly than white
/// noise. It's "brighter" sounding but less harsh and fatiguing. Generated by differentiating pink
/// noise.
///
/// **When to use:** High-passed audio dithering (preferred over violet), digital signal processing,
/// or when you want bright sound without the harshness of white noise.
///
/// **Sound:** Brighter than white noise but smoother and less fatiguing.
///
/// **vs White Noise:** Blue has better frequency distribution and less clustering.
///
/// **vs Violet Noise:** Blue is better for dithering - violet pushes too much energy to very high
/// frequencies.
///
/// **Clipping Warning:** Can exceed [-1.0, 1.0] bounds due to differentiation. Consider
/// attenuation or limiting if clipping is critical.
///
/// Technical: f frequency spectrum (power increases 3dB per octave).
#[derive(Clone, Debug)]
pub struct Blue<R: Rng = SmallRng> {
    sample_rate: SampleRate,
    white_noise: WhiteUniform<R>,
    prev_white: Sample,
}

impl Blue {
    /// Create a new blue noise generator with `SmallRng` seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> Blue<R> {
    /// Create a new blue noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        Self {
            sample_rate,
            white_noise: WhiteUniform::new_with_rng(sample_rate, rng),
            prev_white: 0.0,
        }
    }
}

impl<R: Rng> Iterator for Blue<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let white = self
            .white_noise
            .next()
            .expect("White noise should never return None");
        let blue = white - self.prev_white;
        self.prev_white = white;
        Some(blue)
    }
}

impl_noise_source!(Blue<R>);

/// Violet noise generator - very bright and sharp sounding.
/// Also known as purple noise.
///
/// Violet noise (also called purple noise) heavily emphasizes high frequencies, creating a very
/// bright, sharp, sometimes harsh sound. It's the opposite of brownian noise in terms of frequency
/// emphasis.
///
/// **When to use:** Testing high-frequency equipment response, creating bright/sharp sound
/// effects, or when you need to emphasize treble frequencies.
///
/// **Sound:** Very bright, sharp, can be harsh - use sparingly in audio applications.
///
/// **vs Blue Noise:** Violet is much brighter and more aggressive than blue noise.
///
/// **Not ideal for dithering:** Too much energy at very high frequencies can cause aliasing.
///
/// **Clipping Warning:** Can exceed [-1.0, 1.0] bounds due to differentiation. Consider
/// attenuation or limiting if clipping is critical.
///
/// Technical: f² frequency spectrum (power increases 6dB per octave).
/// Generated by differentiating uniform random samples.
#[derive(Clone, Debug)]
pub struct Violet<R: Rng = SmallRng> {
    sample_rate: SampleRate,
    blue_noise: Blue<R>,
    prev: Sample,
}

impl Violet {
    /// Create a new violet noise generator with `SmallRng` seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> Violet<R> {
    /// Create a new violet noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        Self {
            sample_rate,
            blue_noise: Blue::new_with_rng(sample_rate, rng),
            prev: 0.0,
        }
    }
}

impl<R: Rng> Iterator for Violet<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let blue = self
            .blue_noise
            .next()
            .expect("Blue noise should never return None");
        let violet = blue - self.prev; // Difference can exceed [-1.0, 1.0] - this is mathematically correct
        self.prev = blue;
        Some(violet)
    }
}

impl_noise_source!(Violet<R>);

/// Private shared implementation for 1/f² integrated noise generators.
///
/// This provides the common leaky integration algorithm used by both Brownian and Red noise
/// generators, avoiding code duplication while maintaining distinct public APIs.
#[derive(Clone, Debug)]
struct IntegratedNoise<W> {
    sample_rate: SampleRate,
    white_noise: W,
    accumulator: Sample,
    leak_factor: Float,
    scale: Float,
}

impl<W> IntegratedNoise<W>
where
    W: Iterator<Item = Sample>,
{
    /// Create a new integrated noise generator with the given white noise source and its standard deviation.
    fn new_with_stddev(sample_rate: SampleRate, white_noise: W, white_noise_stddev: Float) -> Self {
        // Leak factor prevents DC buildup while maintaining 1/f² characteristics.
        // Center frequency is set to 5Hz, which provides good behavior
        // while preventing excessive low-frequency buildup across common sample rates.
        let center_freq_hz = 5.0;
        let leak_factor = 1.0 - ((2.0 * PI * center_freq_hz) / sample_rate.get() as Float);

        // Calculate the scaling factor to normalize output based on leak factor.
        // This ensures consistent output level regardless of the leak factor value.
        let variance =
            (white_noise_stddev * white_noise_stddev) / (1.0 - leak_factor * leak_factor);
        let scale = 1.0 / variance.sqrt();

        Self {
            sample_rate,
            white_noise,
            accumulator: 0.0,
            leak_factor,
            scale,
        }
    }
}

impl<W: Iterator<Item = Sample>> Iterator for IntegratedNoise<W> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let white = self.white_noise.next()?;
        // Leaky integration: prevents DC buildup while maintaining 1/f² characteristics
        self.accumulator = self.accumulator * self.leak_factor + white;
        Some(self.accumulator * self.scale)
    }
}

/// Brownian noise generator - true stochastic Brownian motion with Gaussian increments.
///
/// Brownian noise is the mathematically precise implementation of Brownian motion using Gaussian
/// white noise increments. This creates the theoretically correct stochastic process with proper
/// statistical properties. Generated by integrating Gaussian white noise with a 5Hz center
/// frequency leak factor to prevent DC buildup.
///
/// **When to use:** Scientific modeling, research applications, or when mathematical precision
/// of Brownian motion is required.
///
/// **Sound:** Very muffled, deep, lacks high frequencies - sounds "distant".
///
/// **vs Red Noise:** Brownian noise is a specific stochastic process with Gaussian properties.
/// For general 1/f² spectrum without Gaussian requirements, use Red noise instead.
///
/// **Technical:** Uses Gaussian white noise as mathematically required for true Brownian motion.
///
/// **Clipping Warning:** Can exceed [-1.0, 1.0] bounds due to integration. Consider attenuation or
/// limiting if clipping is critical.
#[derive(Clone, Debug)]
pub struct Brownian<R: Rng = SmallRng> {
    inner: IntegratedNoise<WhiteGaussian<R>>,
}

impl Brownian {
    /// Create a new brownian noise generator with `SmallRng` seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> Brownian<R> {
    /// Create a new brownian noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        let white_noise = WhiteGaussian::new_with_rng(sample_rate, rng);
        let stddev = white_noise.std_dev();
        let inner = IntegratedNoise::new_with_stddev(sample_rate, white_noise, stddev);

        Self { inner }
    }
}

impl<R: Rng> Iterator for Brownian<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<R: Rng> Source for Brownian<R> {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        nz!(1)
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }

    fn try_seek(&mut self, _pos: Duration) -> Result<(), crate::source::SeekError> {
        // Stateless noise generators can seek to any position since all positions
        // are equally random and don't depend on previous state
        Ok(())
    }
}

/// Red noise generator - practical 1/f² spectrum with bounded output.
///
/// Red noise provides the same 1/f² power spectral density as Brownian noise but uses uniform
/// white noise input for better practical behavior. This avoids the clipping issues of Gaussian
/// input while maintaining the characteristic deep, muffled sound with heavy low-frequency
/// emphasis.
///
/// **When to use:** Audio applications where you want Brownian-like sound characteristics but
/// need predictable bounded output, background rumbles, or muffled distant effects.
///
/// **Sound:** Very muffled, deep, lacks high frequencies - sounds "distant", similar to Brownian.
///
/// **vs Brownian Noise:** Red noise uses uniform input (less clipping) while Brownian
/// noise uses Gaussian input (more clipping). Both have 1/f² spectrum and can exceed bounds.
///
/// **Technical:** Uses uniform white noise input with variance-adjusted scaling for proper
/// normalization.
///
/// **Clipping Warning:** Can exceed [-1.0, 1.0] bounds due to integration, though less
/// frequently than Brownian noise due to uniform input. Consider attenuation or limiting if
/// clipping is critical.
#[derive(Clone, Debug)]
pub struct Red<R: Rng = SmallRng> {
    inner: IntegratedNoise<WhiteUniform<R>>,
}

impl Red {
    /// Create a new red noise generator with `SmallRng` seeded from system entropy.
    pub fn new(sample_rate: SampleRate) -> Self {
        Self::new_with_rng(sample_rate, SmallRng::from_os_rng())
    }
}

impl<R: Rng> Red<R> {
    /// Create a new red noise generator with a custom RNG.
    pub fn new_with_rng(sample_rate: SampleRate, rng: R) -> Self {
        let white_noise = WhiteUniform::new_with_rng(sample_rate, rng);
        let stddev = white_noise.std_dev() as Float;
        let inner = IntegratedNoise::new_with_stddev(sample_rate, white_noise, stddev);

        Self { inner }
    }
}

impl<R: Rng> Iterator for Red<R> {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<R: Rng> Source for Red<R> {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        nz!(1)
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }

    fn try_seek(&mut self, _pos: Duration) -> Result<(), crate::source::SeekError> {
        // Stateless noise generators can seek to any position since all positions
        // are equally random and don't depend on previous state
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    // Test constants
    const TEST_SAMPLE_RATE: SampleRate = nz!(44100);
    const TEST_SAMPLES_SMALL: usize = 100;
    const TEST_SAMPLES_MEDIUM: usize = 1000;

    /// Calculate correlation between consecutive samples.
    fn calculate_correlation(samples: &[Sample]) -> Sample {
        let mut correlation_sum = 0.0;
        for i in 0..samples.len() - 1 {
            correlation_sum += samples[i] * samples[i + 1];
        }
        correlation_sum / (samples.len() - 1) as Sample
    }

    /// Test properties common to 1/f² integrated noise generators (Brownian and Red).
    fn test_integrated_noise_properties<T: Iterator<Item = Sample>>(mut generator: T, name: &str) {
        // Test that integrated noise doesn't accumulate DC indefinitely
        let samples: Vec<Sample> = (0..TEST_SAMPLE_RATE.get() * 10)
            .map(|_| generator.next().unwrap())
            .collect(); // 10 seconds

        let average = samples.iter().sum::<Sample>() / samples.len() as Sample;
        // Average should be close to zero due to leak factor
        assert!(
            average.abs() < 0.5,
            "{name} noise average too far from zero: {average}"
        );

        // Integrated noise should have strong positive correlation between consecutive samples
        let avg_correlation = calculate_correlation(&samples);
        assert!(
            avg_correlation > 0.1,
            "{name} noise should have strong positive correlation: {avg_correlation}"
        );
    }

    // Helper function to create iterator from generator name
    fn create_generator_iterator(name: &str) -> Box<dyn Iterator<Item = Sample>> {
        match name {
            "WhiteUniform" => Box::new(WhiteUniform::new(TEST_SAMPLE_RATE)),
            "WhiteTriangular" => Box::new(WhiteTriangular::new(TEST_SAMPLE_RATE)),
            "WhiteGaussian" => Box::new(WhiteGaussian::new(TEST_SAMPLE_RATE)),
            "Pink" => Box::new(Pink::new(TEST_SAMPLE_RATE)),
            "Blue" => Box::new(Blue::new(TEST_SAMPLE_RATE)),
            "Violet" => Box::new(Violet::new(TEST_SAMPLE_RATE)),
            "Brownian" => Box::new(Brownian::new(TEST_SAMPLE_RATE)),
            "Red" => Box::new(Red::new(TEST_SAMPLE_RATE)),
            "Velvet" => Box::new(Velvet::new(TEST_SAMPLE_RATE)),
            _ => panic!("Unknown generator: {name}"),
        }
    }

    // Helper function to create source from generator name
    fn create_generator_source(name: &str) -> Box<dyn Source> {
        match name {
            "WhiteUniform" => Box::new(WhiteUniform::new(TEST_SAMPLE_RATE)),
            "WhiteTriangular" => Box::new(WhiteTriangular::new(TEST_SAMPLE_RATE)),
            "WhiteGaussian" => Box::new(WhiteGaussian::new(TEST_SAMPLE_RATE)),
            "Pink" => Box::new(Pink::new(TEST_SAMPLE_RATE)),
            "Blue" => Box::new(Blue::new(TEST_SAMPLE_RATE)),
            "Violet" => Box::new(Violet::new(TEST_SAMPLE_RATE)),
            "Brownian" => Box::new(Brownian::new(TEST_SAMPLE_RATE)),
            "Red" => Box::new(Red::new(TEST_SAMPLE_RATE)),
            "Velvet" => Box::new(Velvet::new(TEST_SAMPLE_RATE)),
            _ => panic!("Unknown generator: {name}"),
        }
    }

    // Templates for different generator groups
    #[template]
    #[rstest]
    #[case("WhiteUniform")]
    #[case("WhiteTriangular")]
    #[case("WhiteGaussian")]
    #[case("Pink")]
    #[case("Blue")]
    #[case("Violet")]
    #[case("Brownian")]
    #[case("Red")]
    #[case("Velvet")]
    fn all_generators(#[case] generator_name: &str) {}

    // Generators that are mathematically bounded to [-1.0, 1.0]
    #[template]
    #[rstest]
    #[case("WhiteUniform")]
    #[case("WhiteTriangular")]
    #[case("Pink")]
    #[case("Velvet")]
    fn bounded_generators(#[case] generator_name: &str) {}

    // Generators that can mathematically exceed [-1.0, 1.0] (differentiators and integrators)
    #[template]
    #[rstest]
    #[case("WhiteGaussian")] // Gaussian can exceed bounds (3-sigma rule, ~0.3% chance)
    #[case("Blue")] // Difference of bounded values can exceed bounds
    #[case("Violet")] // Difference of bounded values can exceed bounds
    #[case("Brownian")] // Integration can exceed bounds despite scaling
    #[case("Red")] // Integration can exceed bounds despite scaling and uniform input
    fn unbounded_generators(#[case] generator_name: &str) {}

    // Test that mathematically bounded generators stay within [-1.0, 1.0]
    #[apply(bounded_generators)]
    #[trace]
    fn test_bounded_generators_range(generator_name: &str) {
        let mut generator = create_generator_iterator(generator_name);
        for i in 0..TEST_SAMPLES_MEDIUM {
            let sample = generator.next().unwrap();
            assert!(
                (-1.0..=1.0).contains(&sample),
                "{generator_name} sample {i} out of range [-1.0, 1.0]: {sample}"
            );
        }
    }

    // Test that unbounded generators produce finite samples (no bounds check)
    #[apply(unbounded_generators)]
    #[trace]
    fn test_unbounded_generators_finite(generator_name: &str) {
        let mut generator = create_generator_iterator(generator_name);
        for i in 0..TEST_SAMPLES_MEDIUM {
            let sample = generator.next().unwrap();
            assert!(
                sample.is_finite(),
                "{generator_name} produced non-finite sample at index {i}: {sample}"
            );
        }
    }

    // Test that generators can seek without errors
    #[apply(all_generators)]
    #[trace]
    fn test_generators_seek(generator_name: &str) {
        let mut generator = create_generator_source(generator_name);
        let seek_result = generator.try_seek(std::time::Duration::from_secs(1));
        assert!(
            seek_result.is_ok(),
            "{generator_name} should support seeking but returned error: {seek_result:?}"
        );
    }

    // Test common Source trait properties for all generators
    #[apply(all_generators)]
    #[trace]
    fn test_source_trait_properties(generator_name: &str) {
        let source = create_generator_source(generator_name);

        // All noise generators should be mono (1 channel)
        assert_eq!(source.channels(), nz!(1), "{generator_name} should be mono");

        // All should have the expected sample rate
        assert_eq!(
            source.sample_rate(),
            TEST_SAMPLE_RATE,
            "{generator_name} should have correct sample rate"
        );

        // All should have infinite duration
        assert_eq!(
            source.total_duration(),
            None,
            "{generator_name} should have infinite duration"
        );

        // All should return None for current_span_len (infinite streams)
        assert_eq!(
            source.current_span_len(),
            None,
            "{generator_name} should have no span length limit"
        );
    }

    #[test]
    fn test_white_uniform_distribution() {
        let mut generator = WhiteUniform::new(TEST_SAMPLE_RATE);
        let mut min = Sample::INFINITY;
        let mut max = Sample::NEG_INFINITY;

        for _ in 0..TEST_SAMPLES_MEDIUM {
            let sample = generator.next().unwrap();
            min = min.min(sample);
            max = max.max(sample);
        }

        // Should use the full range approximately
        assert!(min < -0.9, "Min sample should be close to -1.0: {min}");
        assert!(max > 0.9, "Max sample should be close to 1.0: {max}");
    }

    #[test]
    fn test_triangular_distribution() {
        let mut generator = WhiteTriangular::new(TEST_SAMPLE_RATE);

        // Triangular distribution should have most values near 0
        let mut near_zero_count = 0;
        let total_samples = TEST_SAMPLES_MEDIUM;

        for _ in 0..total_samples {
            let sample = generator.next().unwrap();
            if sample.abs() < 0.5 {
                near_zero_count += 1;
            }
        }

        // Triangular distribution should have more samples near zero than uniform
        assert!(
            near_zero_count > total_samples / 2,
            "Triangular distribution should favor values near zero"
        );
    }

    #[test]
    fn test_gaussian_noise_properties() {
        let generator = WhiteGaussian::new(TEST_SAMPLE_RATE);
        assert_eq!(generator.std_dev(), 0.6);
        assert_eq!(generator.mean(), 0.0);

        // Test that most samples fall within 3 standard deviations
        // (should be ~85%; theoretical: ~90.5%)
        let mut generator = WhiteGaussian::new(TEST_SAMPLE_RATE);
        let samples: Vec<Sample> = (0..TEST_SAMPLES_MEDIUM)
            .map(|_| generator.next().unwrap())
            .collect();
        let out_of_bounds = samples.iter().filter(|&&s| s.abs() > 1.0).count();
        let within_bounds_percentage =
            ((samples.len() - out_of_bounds) as f64 / samples.len() as f64) * 100.0;

        assert!(
 within_bounds_percentage > 85.0,
 "Expected >85% of Gaussian samples within [-1.0, 1.0], got {within_bounds_percentage:.1}%"
 );
    }

    #[test]
    fn test_pink_noise_properties() {
        let mut generator = Pink::new(TEST_SAMPLE_RATE);
        let samples: Vec<Sample> = (0..TEST_SAMPLES_MEDIUM)
            .map(|_| generator.next().unwrap())
            .collect();

        // Pink noise should have more correlation between consecutive samples than white noise
        let avg_correlation = calculate_correlation(&samples);

        // Pink noise should have some positive correlation (though not as strong as Brownian)
        assert!(
            avg_correlation > -0.1,
            "Pink noise should have low positive correlation, got: {avg_correlation}"
        );
    }

    #[test]
    fn test_blue_noise_properties() {
        let mut generator = Blue::new(TEST_SAMPLE_RATE);
        let samples: Vec<Sample> = (0..TEST_SAMPLES_MEDIUM)
            .map(|_| generator.next().unwrap())
            .collect();

        // Blue noise should have less correlation than pink noise
        let avg_correlation = calculate_correlation(&samples);

        // Blue noise should have near-zero or negative correlation
        assert!(
            avg_correlation < 0.1,
            "Blue noise should have low correlation, got: {avg_correlation}"
        );
    }

    #[test]
    fn test_violet_noise_properties() {
        let mut generator = Violet::new(TEST_SAMPLE_RATE);
        let samples: Vec<Sample> = (0..TEST_SAMPLES_MEDIUM)
            .map(|_| generator.next().unwrap())
            .collect();

        // Violet noise should have high-frequency characteristics
        // Check that consecutive differences have higher variance than the original signal
        let mut diff_variance = 0.0;
        let mut signal_variance = 0.0;
        let mean = samples.iter().sum::<Sample>() / samples.len() as Sample;

        for i in 0..samples.len() - 1 {
            let diff = samples[i + 1] - samples[i];
            diff_variance += diff * diff;
            let centered = samples[i] - mean;
            signal_variance += centered * centered;
        }

        diff_variance /= (samples.len() - 1) as Sample;
        signal_variance /= samples.len() as Sample;

        // For violet noise (high-pass), differences should have comparable or higher variance
        assert!(
 diff_variance > signal_variance * 0.1,
 "Violet noise should have high-frequency characteristics, diff_var: {diff_variance}, signal_var: {signal_variance}"
 );
    }

    #[test]
    fn test_brownian_noise_properties() {
        let generator = Brownian::new(TEST_SAMPLE_RATE);
        test_integrated_noise_properties(generator, "Brownian");
    }

    #[test]
    fn test_red_noise_properties() {
        let generator = Red::new(TEST_SAMPLE_RATE);
        test_integrated_noise_properties(generator, "Red");
    }

    #[test]
    fn test_velvet_noise_properties() {
        let mut generator = Velvet::new(TEST_SAMPLE_RATE);
        let mut impulse_count = 0;

        for _ in 0..TEST_SAMPLE_RATE.get() {
            let sample = generator.next().unwrap();
            if sample != 0.0 {
                impulse_count += 1;
                // Velvet impulses should be exactly +1.0 or -1.0
                assert!(sample == 1.0 || sample == -1.0);
            }
        }

        assert!(
            impulse_count > (VELVET_DEFAULT_DENSITY.get() as f32 * 0.75) as usize
                && impulse_count < (VELVET_DEFAULT_DENSITY.get() as f32 * 1.25) as usize,
            "Impulse count out of range: expected ~{VELVET_DEFAULT_DENSITY}, got {impulse_count}"
        );
    }

    #[test]
    fn test_velvet_custom_density() {
        let density = nz!(1000); // impulses per second for testing
        let mut generator =
            Velvet::new_with_density(TEST_SAMPLE_RATE, density, SmallRng::from_os_rng());

        let mut impulse_count = 0;
        for _ in 0..TEST_SAMPLE_RATE.get() {
            if generator.next().unwrap() != 0.0 {
                impulse_count += 1;
            }
        }

        // Should be approximately the requested density
        assert!(
            density.get() - impulse_count < 200,
            "Custom density not achieved: expected ~{density}, got {impulse_count}"
        );
    }
}
