// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! The PERT distribution.

use crate::{Beta, Distribution, Exp1, Open01, StandardNormal};
use core::fmt;
use num_traits::Float;
use rand::Rng;

/// The [PERT distribution](https://en.wikipedia.org/wiki/PERT_distribution) `PERT(min, max, mode, shape)`.
///
/// Similar to the [`Triangular`] distribution, the PERT distribution is
/// parameterised by a range and a mode within that range. Unlike the
/// [`Triangular`] distribution, the probability density function of the PERT
/// distribution is smooth, with a configurable weighting around the mode.
///
/// # Plot
///
/// The following plot shows the PERT distribution with `min = -1`, `max = 1`,
/// and various values of `mode` and `shape`.
///
/// ![PERT distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/pert.svg)
///
/// # Example
///
/// ```rust
/// use rand_distr::{Pert, Distribution};
///
/// let d = Pert::new(0., 5.).with_mode(2.5).unwrap();
/// let v = d.sample(&mut rand::rng());
/// println!("{} is from a PERT distribution", v);
/// ```
///
/// [`Triangular`]: crate::Triangular
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pert<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    min: F,
    range: F,
    beta: Beta<F>,
}

/// Error type returned from [`Pert`] constructors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PertError {
    /// `max < min` or `min` or `max` is NaN.
    RangeTooSmall,
    /// `mode < min` or `mode > max` or `mode` is NaN.
    ModeRange,
    /// `shape < 0` or `shape` is NaN
    ShapeTooSmall,
}

impl fmt::Display for PertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PertError::RangeTooSmall => "requirement min < max is not met in PERT distribution",
            PertError::ModeRange => "mode is outside [min, max] in PERT distribution",
            PertError::ShapeTooSmall => "shape < 0 or is NaN in PERT distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PertError {}

impl<F> Pert<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Construct a PERT distribution with defined `min`, `max`
    ///
    /// # Example
    ///
    /// ```
    /// use rand_distr::Pert;
    /// let pert_dist = Pert::new(0.0, 10.0)
    ///     .with_shape(3.5)
    ///     .with_mean(3.0)
    ///     .unwrap();
    /// # let _unused: Pert<f64> = pert_dist;
    /// ```
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub fn new(min: F, max: F) -> PertBuilder<F> {
        let shape = F::from(4.0).unwrap();
        PertBuilder { min, max, shape }
    }
}

/// Struct used to build a [`Pert`]
#[derive(Debug)]
pub struct PertBuilder<F> {
    min: F,
    max: F,
    shape: F,
}

impl<F> PertBuilder<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Set the shape parameter
    ///
    /// If not specified, this defaults to 4.
    #[inline]
    pub fn with_shape(mut self, shape: F) -> PertBuilder<F> {
        self.shape = shape;
        self
    }

    /// Specify the mean
    #[inline]
    pub fn with_mean(self, mean: F) -> Result<Pert<F>, PertError> {
        let two = F::from(2.0).unwrap();
        let mode = ((self.shape + two) * mean - self.min - self.max) / self.shape;
        self.with_mode(mode)
    }

    /// Specify the mode
    #[inline]
    pub fn with_mode(self, mode: F) -> Result<Pert<F>, PertError> {
        if !(self.max > self.min) {
            return Err(PertError::RangeTooSmall);
        }
        if !(mode >= self.min && self.max >= mode) {
            return Err(PertError::ModeRange);
        }
        if !(self.shape >= F::from(0.).unwrap()) {
            return Err(PertError::ShapeTooSmall);
        }

        let (min, max, shape) = (self.min, self.max, self.shape);
        let range = max - min;
        let v = F::from(1.0).unwrap() + shape * (mode - min) / range;
        let w = F::from(1.0).unwrap() + shape * (max - mode) / range;
        let beta = Beta::new(v, w).map_err(|_| PertError::RangeTooSmall)?;
        Ok(Pert { min, range, beta })
    }
}

impl<F> Distribution<F> for Pert<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        self.beta.sample(rng) * self.range + self.min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pert() {
        for &(min, max, mode) in &[(-1., 1., 0.), (1., 2., 1.), (5., 25., 25.)] {
            let _distr = Pert::new(min, max).with_mode(mode).unwrap();
            // TODO: test correctness
        }

        for &(min, max, mode) in &[(-1., 1., 2.), (-1., 1., -2.), (2., 1., 1.)] {
            assert!(Pert::new(min, max).with_mode(mode).is_err());
        }
    }

    #[test]
    fn distributions_can_be_compared() {
        let (min, mode, max, shape) = (1.0, 2.0, 3.0, 4.0);
        let p1 = Pert::new(min, max).with_mode(mode).unwrap();
        let mean = (min + shape * mode + max) / (shape + 2.0);
        let p2 = Pert::new(min, max).with_mean(mean).unwrap();
        assert_eq!(p1, p2);
    }

    #[test]
    fn mode_almost_half_range() {
        assert!(Pert::new(0.0f32, 0.48258883).with_mode(0.24129441).is_ok());
    }

    #[test]
    fn almost_symmetric_about_zero() {
        let distr = Pert::new(-10f32, 10f32).with_mode(f32::EPSILON);
        assert!(distr.is_ok());
    }

    #[test]
    fn almost_symmetric() {
        let distr = Pert::new(0f32, 2f32).with_mode(1f32 + f32::EPSILON);
        assert!(distr.is_ok());
    }
}
