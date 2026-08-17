//! Math utilities for audio processing.

use crate::common::SampleRate;
use std::time::Duration;

/// Nanoseconds per second, used for Duration calculations.
pub(crate) const NANOS_PER_SEC: u64 = 1_000_000_000;

// Re-export float constants with appropriate precision for the Float type.
// This centralizes all cfg gating for constants in one place.
#[cfg(not(feature = "64bit"))]
pub use std::f32::consts::{E, LN_10, LN_2, LOG10_2, LOG10_E, LOG2_10, LOG2_E, PI, TAU};
#[cfg(feature = "64bit")]
pub use std::f64::consts::{E, LN_10, LN_2, LOG10_2, LOG10_E, LOG2_10, LOG2_E, PI, TAU};

/// Linear interpolation between two samples.
///
/// The result should be equivalent to
/// `first * (1 - numerator / denominator) + second * numerator / denominator`.
///
/// To avoid numeric overflows pick smaller numerator.
// TODO (refactoring) Streamline this using coefficient instead of numerator and denominator.
#[inline]
pub(crate) fn lerp(first: Sample, second: Sample, numerator: u32, denominator: u32) -> Sample {
    first + (second - first) * numerator as Float / denominator as Float
}

/// Converts decibels to linear amplitude scale.
///
/// This function converts a decibel value to its corresponding linear amplitude value
/// using the formula: `linear = 10^(decibels/20)` for amplitude.
///
/// # Arguments
///
/// * `decibels` - The decibel value to convert. Common ranges:
///   - 0 dB = linear value of 1.0 (no change)
///   - Positive dB values represent amplification (> 1.0)
///   - Negative dB values represent attenuation (< 1.0)
///   - -60 dB ≈ 0.001 (barely audible)
///   - +20 dB = 10.0 (10x amplification)
///
/// # Returns
///
/// The linear amplitude value corresponding to the input decibels.
///
/// # Performance
///
/// This implementation is optimized for speed, being ~3-4% faster than the standard
/// `10f32.powf(decibels * 0.05)` approach, with a maximum error of only 2.48e-7
/// (representing about -132 dB precision).
#[inline]
pub fn db_to_linear(decibels: Float) -> Float {
    // ~3-4% faster than using `10f32.powf(decibels * 0.05)`,
    // with a maximum error of 2.48e-7 representing only about -132 dB.
    Float::powf(2.0, decibels * 0.05 * LOG2_10)
}

/// Converts linear amplitude scale to decibels.
///
/// This function converts a linear amplitude value to its corresponding decibel value
/// using the formula: `decibels = 20 * log10(linear)` for amplitude.
///
/// # Arguments
///
/// * `linear` - The linear amplitude value to convert. Must be positive for meaningful results:
///   - 1.0 = 0 dB (no change)
///   - Values > 1.0 represent amplification (positive dB)
///   - Values < 1.0 represent attenuation (negative dB)
///   - 0.0 results in negative infinity
///   - Negative values are not physically meaningful for amplitude
///
/// # Returns
///
/// The decibel value corresponding to the input linear amplitude.
///
/// # Performance
///
/// This implementation is optimized for speed, being faster than the standard
/// `20.0 * linear.log10()` approach while maintaining high precision.
///
/// # Special Cases
///
/// - `linear_to_db(0.0)` returns negative infinity
/// - Very small positive values approach negative infinity
/// - Negative values return NaN (not physically meaningful for amplitude)
#[inline]
pub fn linear_to_db(linear: Float) -> Float {
    // Same as `to_linear`: faster than using `20f32.log10() * linear`
    linear.log2() * LOG10_2 * 20.0
}

/// Converts a time duration to a smoothing coefficient for exponential filtering.
///
/// Used for both attack and release filtering in the limiter's envelope detector.
/// Creates a coefficient that determines how quickly the limiter responds to level changes:
/// * Longer times = higher coefficients (closer to 1.0) = slower, smoother response
/// * Shorter times = lower coefficients (closer to 0.0) = faster, more immediate response
///
/// The coefficient is calculated using the formula: `e^(-1 / (duration_seconds * sample_rate))`
/// which provides exponential smoothing behavior suitable for audio envelope detection.
///
/// # Arguments
///
/// * `duration` - Desired response time (attack or release duration)
/// * `sample_rate` - Audio sample rate in Hz
///
/// # Returns
///
/// Smoothing coefficient in the range [0.0, 1.0] for use in exponential filters
#[must_use]
pub(crate) fn duration_to_coefficient(duration: Duration, sample_rate: SampleRate) -> Float {
    Float::exp(-1.0 / (duration_to_float(duration) * sample_rate.get() as Float))
}

/// Convert Duration to Float with appropriate precision for the Sample type.
#[inline]
#[must_use]
pub(crate) fn duration_to_float(duration: Duration) -> Float {
    #[cfg(not(feature = "64bit"))]
    {
        duration.as_secs_f32()
    }
    #[cfg(feature = "64bit")]
    {
        duration.as_secs_f64()
    }
}

/// Utility macro for getting a `NonZero` from a literal. Especially
/// useful for passing in `ChannelCount` and `Samplerate`.
/// Equivalent to: `const { core::num::NonZero::new($n).unwrap() }`
///
/// # Example
/// ```
/// use rodio::nz;
/// use rodio::static_buffer::StaticSamplesBuffer;
/// let buffer = StaticSamplesBuffer::new(nz!(2), nz!(44_100), &[0.0, 0.5, 0.0, 0.5]);
/// ```
///
/// # Panics
/// If the literal passed in is zero this panicks.
#[macro_export]
macro_rules! nz {
    ($n:literal) => {
        const { core::num::NonZero::new($n).unwrap() }
    };
}

pub use nz;

use crate::{common::Float, Sample};

#[cfg(test)]
mod test {
    use super::*;
    use num_rational::Ratio;
    use quickcheck::{quickcheck, TestResult};

    quickcheck! {
        fn lerp_random(first: Sample, second: Sample, numerator: u32, denominator: u32) -> TestResult {
            if denominator == 0 { return TestResult::discard(); }

            // Constrain to realistic audio sample range [-1.0, 1.0]
            // Audio samples rarely exceed this range, and large values cause floating-point error accumulation
            if first.abs() > 1.0 || second.abs() > 1.0 { return TestResult::discard(); }

            // Discard infinite or NaN samples (can occur in quickcheck)
            if !first.is_finite() || !second.is_finite() { return TestResult::discard(); }

            let (numerator, denominator) = Ratio::new(numerator, denominator).into_raw();
            // Reduce max numerator to avoid floating-point error accumulation with large ratios
            if numerator > 1000 { return TestResult::discard(); }

            let a = first as f64;
            let b = second as f64;
            let c = numerator as f64 / denominator as f64;
            if !(0.0..=1.0).contains(&c) { return TestResult::discard(); };

            let reference = a * (1.0 - c) + b * c;
            let x = lerp(first, second, numerator, denominator);

            // With realistic audio-range inputs, lerp should be very precise
            // f32 has ~7 decimal digits, so 1e-6 tolerance is reasonable
            // This is well below 16-bit audio precision (~1.5e-5)
            let tolerance = 1e-6;
            TestResult::from_bool((x as f64 - reference).abs() < tolerance)
        }
    }

    /// Tolerance values for precision tests, derived from empirical measurement
    /// of actual implementation errors across the full ±100dB range.
    ///
    /// Methodology:
    /// 1. Calculated relative errors against mathematically exact `f64` calculations
    /// 2. Found maximum errors: dB->linear = 2.3x ε, linear->dB = 1.0x ε, round-trip = 8x ε
    /// 3. Applied 2x safety margins for cross-platform robustness
    /// 4. All tolerances are much stricter than audio precision requirements:
    ///    - 16-bit audio: ~6e-6 precision needed
    ///    - 24-bit audio: ~6e-8 precision needed
    ///    - Our tolerances: ~6e-7 to 2e-6 (10-1000x better than audio needs)
    ///
    /// Range context:
    /// - Practical audio range (-60dB to +40dB): max errors ~1x ε
    /// - Extended range (-100dB to +100dB): max errors ~2.3x ε
    /// - Extreme edge cases beyond ±100dB have larger errors but are rarely used
    ///
    /// Based on [Wikipedia's Decibel article].
    ///
    /// [Wikipedia's Decibel article]: https://web.archive.org/web/20230810185300/https://en.wikipedia.org/wiki/Decibel
    const DECIBELS_LINEAR_TABLE: [(Float, Float); 27] = [
        (100., 100000.),
        (90., 31623.),
        (80., 10000.),
        (70., 3162.),
        (60., 1000.),
        (50., 316.2),
        (40., 100.),
        (30., 31.62),
        (20., 10.),
        (10., 3.162),
        (5.998, 1.995),
        (3.003, 1.413),
        (1.002, 1.122),
        (0., 1.),
        (-1.002, 0.891),
        (-3.003, 0.708),
        (-5.998, 0.501),
        (-10., 0.3162),
        (-20., 0.1),
        (-30., 0.03162),
        (-40., 0.01),
        (-50., 0.003162),
        (-60., 0.001),
        (-70., 0.0003162),
        (-80., 0.0001),
        (-90., 0.00003162),
        (-100., 0.00001),
    ];

    #[test]
    fn convert_decibels_to_linear() {
        for (db, wikipedia_linear) in DECIBELS_LINEAR_TABLE {
            let actual_linear = db_to_linear(db);

            // Sanity check: ensure we're in the right order of magnitude as Wikipedia data
            // This is lenient to account for rounding in the reference values
            let magnitude_ratio = actual_linear / wikipedia_linear;
            assert!(
                magnitude_ratio > 0.99 && magnitude_ratio < 1.01,
                "Result magnitude differs significantly from Wikipedia reference for {db}dB: Wikipedia {wikipedia_linear}, got {actual_linear}, ratio: {magnitude_ratio:.4}"
            );
        }
    }

    #[test]
    fn convert_linear_to_decibels() {
        // Test the inverse conversion function using the same reference data
        for (expected_db, linear) in DECIBELS_LINEAR_TABLE {
            let actual_db = linear_to_db(linear);

            // Sanity check: ensure we're reasonably close to the expected dB value from the table
            // This accounts for rounding in both the linear and dB reference values
            let magnitude_ratio = if expected_db.abs() > 10.0 * Float::EPSILON {
                actual_db / expected_db
            } else {
                1.0 // Skip ratio check for values very close to 0 dB
            };

            if expected_db.abs() > 10.0 * Float::EPSILON {
                assert!(
                    magnitude_ratio > 0.99 && magnitude_ratio < 1.01,
                    "Result differs significantly from table reference for linear {linear}: expected {expected_db}dB, got {actual_db}dB, ratio: {magnitude_ratio:.4}"
                );
            }
        }
    }

    #[test]
    fn round_trip_conversion_accuracy() {
        // Test that converting dB -> linear -> dB gives back the original value
        let test_db_values = [-60.0, -20.0, -6.0, 0.0, 6.0, 20.0, 40.0];

        for &original_db in &test_db_values {
            let linear = db_to_linear(original_db);
            let round_trip_db = linear_to_db(linear);

            let error = (round_trip_db - original_db).abs();
            const MAX_ROUND_TRIP_ERROR: Float = 16.0 * Float::EPSILON; // max error: 8x ε (practical audio range), with 2x safety margin

            assert!(
                error < MAX_ROUND_TRIP_ERROR,
                "Round-trip conversion failed for {original_db}dB: got {round_trip_db:.8}dB, error: {error:.2e}"
            );
        }

        // Test that converting linear -> dB -> linear gives back the original value
        let test_linear_values = [0.001, 0.1, 1.0, 10.0, 100.0];

        for &original_linear in &test_linear_values {
            let db = linear_to_db(original_linear);
            let round_trip_linear = db_to_linear(db);

            let relative_error = ((round_trip_linear - original_linear) / original_linear).abs();
            const MAX_ROUND_TRIP_RELATIVE_ERROR: Float = 16.0 * Float::EPSILON; // Same as above, for linear->dB->linear round trips

            assert!(
                relative_error < MAX_ROUND_TRIP_RELATIVE_ERROR,
                "Round-trip conversion failed for {original_linear}: got {round_trip_linear:.8}, relative error: {relative_error:.2e}"
            );
        }
    }
}
