// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! The triangular distribution.

use crate::{Distribution, StandardUniform};
use core::fmt;
use num_traits::Float;
use rand::Rng;

/// The [triangular distribution](https://en.wikipedia.org/wiki/Triangular_distribution) `Triangular(min, max, mode)`.
///
/// A continuous probability distribution parameterised by a range, and a mode
/// (most likely value) within that range.
///
/// The probability density function is triangular. For a similar distribution
/// with a smooth PDF, see the [`Pert`] distribution.
///
/// # Plot
///
/// The following plot shows the triangular distribution with various values of
/// `min`, `max`, and `mode`.
///
/// ![Triangular distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/triangular.svg)
///
/// # Example
///
/// ```rust
/// use rand_distr::{Triangular, Distribution};
///
/// let d = Triangular::new(0., 5., 2.5).unwrap();
/// let v = d.sample(&mut rand::rng());
/// println!("{} is from a triangular distribution", v);
/// ```
///
/// [`Pert`]: crate::Pert
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Triangular<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
{
    min: F,
    max: F,
    mode: F,
}

/// Error type returned from [`Triangular::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriangularError {
    /// `max < min` or `min` or `max` is NaN.
    RangeTooSmall,
    /// `mode < min` or `mode > max` or `mode` is NaN.
    ModeRange,
}

impl fmt::Display for TriangularError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            TriangularError::RangeTooSmall => {
                "requirement min <= max is not met in triangular distribution"
            }
            TriangularError::ModeRange => "mode is outside [min, max] in triangular distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TriangularError {}

impl<F> Triangular<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
{
    /// Set up the Triangular distribution with defined `min`, `max` and `mode`.
    #[inline]
    pub fn new(min: F, max: F, mode: F) -> Result<Triangular<F>, TriangularError> {
        if !(max >= min) {
            return Err(TriangularError::RangeTooSmall);
        }
        if !(mode >= min && max >= mode) {
            return Err(TriangularError::ModeRange);
        }
        Ok(Triangular { min, max, mode })
    }
}

impl<F> Distribution<F> for Triangular<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let f: F = rng.sample(StandardUniform);
        let diff_mode_min = self.mode - self.min;
        let range = self.max - self.min;
        let f_range = f * range;
        if f_range < diff_mode_min {
            self.min + (f_range * diff_mode_min).sqrt()
        } else {
            self.max - ((range - f_range) * (self.max - self.mode)).sqrt()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::mock, Rng};

    #[test]
    fn test_triangular() {
        let mut half_rng = mock::StepRng::new(0x8000_0000_0000_0000, 0);
        assert_eq!(half_rng.random::<f64>(), 0.5);
        for &(min, max, mode, median) in &[
            (-1., 1., 0., 0.),
            (1., 2., 1., 2. - 0.5f64.sqrt()),
            (5., 25., 25., 5. + 200f64.sqrt()),
            (1e-5, 1e5, 1e-3, 1e5 - 4999999949.5f64.sqrt()),
            (0., 1., 0.9, 0.45f64.sqrt()),
            (-4., -0.5, -2., -4.0 + 3.5f64.sqrt()),
        ] {
            #[cfg(feature = "std")]
            std::println!("{} {} {} {}", min, max, mode, median);
            let distr = Triangular::new(min, max, mode).unwrap();
            // Test correct value at median:
            assert_eq!(distr.sample(&mut half_rng), median);
        }

        for &(min, max, mode) in &[(-1., 1., 2.), (-1., 1., -2.), (2., 1., 1.)] {
            assert!(Triangular::new(min, max, mode).is_err());
        }
    }

    #[test]
    fn triangular_distributions_can_be_compared() {
        assert_eq!(
            Triangular::new(1.0, 3.0, 2.0),
            Triangular::new(1.0, 3.0, 2.0)
        );
    }
}
