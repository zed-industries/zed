use crate::source::{FadeIn, Mix, TakeDuration};
use crate::Source;
use std::time::Duration;

/// Mixes one sound fading out with another sound fading in for the given
/// duration.
///
/// Only the crossfaded portion (beginning of fadeout, beginning of fadein) is
/// returned.
pub fn crossfade<I1, I2>(
    input_fadeout: I1,
    input_fadein: I2,
    duration: Duration,
) -> Crossfade<I1, I2>
where
    I1: Source,
    I2: Source,
{
    let mut input_fadeout = input_fadeout.take_duration(duration);
    input_fadeout.set_filter_fadeout();
    let input_fadein = input_fadein.take_duration(duration).fade_in(duration);
    input_fadeout.mix(input_fadein)
}

/// Mixes one sound fading out with another sound fading in for the given
/// duration.
///
/// Only the crossfaded portion (beginning of fadeout, beginning of fadein) is
/// covered.
pub type Crossfade<I1, I2> = Mix<TakeDuration<I1>, FadeIn<TakeDuration<I2>>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::SamplesBuffer;
    use crate::math::nz;
    use crate::source::Zero;
    use crate::Sample;

    fn dummy_source(length: u8) -> SamplesBuffer {
        let data: Vec<Sample> = (1..=length).map(Sample::from).collect();
        SamplesBuffer::new(nz!(1), nz!(1), data)
    }

    #[test]
    fn test_crossfade_with_self() {
        let source1 = dummy_source(10);
        let source2 = dummy_source(10);
        let mut mixed = crossfade(
            source1,
            source2,
            Duration::from_secs(5) + Duration::from_nanos(1),
        );

        // Use approximate equality for floating-point comparisons
        let eps = 1e-6;
        assert!((mixed.next().unwrap() - 1.0).abs() < eps);
        assert!((mixed.next().unwrap() - 2.0).abs() < eps);
        assert!((mixed.next().unwrap() - 3.0).abs() < eps);
        assert!((mixed.next().unwrap() - 4.0).abs() < eps);
        assert!((mixed.next().unwrap() - 5.0).abs() < eps);
        assert_eq!(mixed.next(), None);
    }

    #[test]
    fn test_crossfade() {
        let source1 = dummy_source(10);
        let source2 = Zero::new(nz!(1), nz!(1));
        let mixed = crossfade(
            source1,
            source2,
            Duration::from_secs(5) + Duration::from_nanos(1),
        );
        let result = mixed.collect::<Vec<_>>();
        assert_eq!(result.len(), 5);
        assert!(result
            .iter()
            .zip(vec![1.0, 2.0 * 0.8, 3.0 * 0.6, 4.0 * 0.4, 5.0 * 0.2])
            .all(|(a, b)| (a - b).abs() < 1e-6));
    }
}
