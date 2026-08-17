// Copyright 2021 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Skew Normal distribution `SN(ξ, ω, α)`.

use crate::{Distribution, StandardNormal};
use core::fmt;
use num_traits::Float;
use rand::Rng;

/// The [skew normal distribution](https://en.wikipedia.org/wiki/Skew_normal_distribution) `SN(ξ, ω, α)`.
///
/// The skew normal distribution is a generalization of the
/// [`Normal`](crate::Normal) distribution to allow for non-zero skewness.
/// It has location parameter `ξ` (`xi`), scale parameter `ω` (`omega`),
/// and shape parameter `α` (`alpha`).
///
/// The `ξ` and `ω` parameters correspond to the mean `μ` and standard
/// deviation `σ` of the normal distribution, respectively.
/// The `α` parameter controls the skewness.
///
/// # Density function
///
/// It has the density function, for `scale > 0`,
/// `f(x) = 2 / scale * phi((x - location) / scale) * Phi(alpha * (x - location) / scale)`
/// where `phi` and `Phi` are the density and distribution of a standard normal variable.
///
/// # Plot
///
/// The following plot shows the skew normal distribution with `location = 0`, `scale = 1`
/// (corresponding to the [`standard normal distribution`](crate::StandardNormal)), and
/// various values of `shape`.
///
/// ![Skew normal distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/skew_normal.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{SkewNormal, Distribution};
///
/// // location 2, scale 3, shape 1
/// let skew_normal = SkewNormal::new(2.0, 3.0, 1.0).unwrap();
/// let v = skew_normal.sample(&mut rand::rng());
/// println!("{} is from a SN(2, 3, 1) distribution", v)
/// ```
///
/// # Implementation details
///
/// We are using the algorithm from [A Method to Simulate the Skew Normal Distribution].
///
/// [skew normal distribution]: https://en.wikipedia.org/wiki/Skew_normal_distribution
/// [`Normal`]: struct.Normal.html
/// [A Method to Simulate the Skew Normal Distribution]: https://dx.doi.org/10.4236/am.2014.513201
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkewNormal<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
{
    location: F,
    scale: F,
    shape: F,
}

/// Error type returned from [`SkewNormal::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// The scale parameter is not finite or it is less or equal to zero.
    ScaleTooSmall,
    /// The shape parameter is not finite.
    BadShape,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::ScaleTooSmall => {
                "scale parameter is either non-finite or it is less or equal to zero in skew normal distribution"
            }
            Error::BadShape => "shape parameter is non-finite in skew normal distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F> SkewNormal<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
{
    /// Construct, from location, scale and shape.
    ///
    /// Parameters:
    ///
    /// -   location (unrestricted)
    /// -   scale (must be finite and larger than zero)
    /// -   shape (must be finite)
    #[inline]
    pub fn new(location: F, scale: F, shape: F) -> Result<SkewNormal<F>, Error> {
        if !scale.is_finite() || !(scale > F::zero()) {
            return Err(Error::ScaleTooSmall);
        }
        if !shape.is_finite() {
            return Err(Error::BadShape);
        }
        Ok(SkewNormal {
            location,
            scale,
            shape,
        })
    }

    /// Returns the location of the distribution.
    pub fn location(&self) -> F {
        self.location
    }

    /// Returns the scale of the distribution.
    pub fn scale(&self) -> F {
        self.scale
    }

    /// Returns the shape of the distribution.
    pub fn shape(&self) -> F {
        self.shape
    }
}

impl<F> Distribution<F> for SkewNormal<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let linear_map = |x: F| -> F { x * self.scale + self.location };
        let u_1: F = rng.sample(StandardNormal);
        if self.shape == F::zero() {
            linear_map(u_1)
        } else {
            let u_2 = rng.sample(StandardNormal);
            let (u, v) = (u_1.max(u_2), u_1.min(u_2));
            if self.shape == -F::one() {
                linear_map(v)
            } else if self.shape == F::one() {
                linear_map(u)
            } else {
                let normalized = ((F::one() + self.shape) * u + (F::one() - self.shape) * v)
                    / ((F::one() + self.shape * self.shape).sqrt()
                        * F::from(core::f64::consts::SQRT_2).unwrap());
                linear_map(normalized)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_samples<F: Float + fmt::Debug, D: Distribution<F>>(distr: D, zero: F, expected: &[F]) {
        let mut rng = crate::test::rng(213);
        let mut buf = [zero; 4];
        for x in &mut buf {
            *x = rng.sample(&distr);
        }
        assert_eq!(buf, expected);
    }

    #[test]
    #[should_panic]
    fn invalid_scale_nan() {
        SkewNormal::new(0.0, f64::NAN, 0.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_scale_zero() {
        SkewNormal::new(0.0, 0.0, 0.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_scale_negative() {
        SkewNormal::new(0.0, -1.0, 0.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_scale_infinite() {
        SkewNormal::new(0.0, f64::INFINITY, 0.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_shape_nan() {
        SkewNormal::new(0.0, 1.0, f64::NAN).unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_shape_infinite() {
        SkewNormal::new(0.0, 1.0, f64::INFINITY).unwrap();
    }

    #[test]
    fn valid_location_nan() {
        SkewNormal::new(f64::NAN, 1.0, 0.0).unwrap();
    }

    #[test]
    fn skew_normal_value_stability() {
        test_samples(
            SkewNormal::new(0.0, 1.0, 0.0).unwrap(),
            0f32,
            &[-0.11844189, 0.781378, 0.06563994, -1.1932899],
        );
        test_samples(
            SkewNormal::new(0.0, 1.0, 0.0).unwrap(),
            0f64,
            &[
                -0.11844188827977231,
                0.7813779637772346,
                0.06563993969580051,
                -1.1932899004186373,
            ],
        );
        test_samples(
            SkewNormal::new(f64::INFINITY, 1.0, 0.0).unwrap(),
            0f64,
            &[f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY],
        );
        test_samples(
            SkewNormal::new(f64::NEG_INFINITY, 1.0, 0.0).unwrap(),
            0f64,
            &[
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
            ],
        );
    }

    #[test]
    fn skew_normal_value_location_nan() {
        let skew_normal = SkewNormal::new(f64::NAN, 1.0, 0.0).unwrap();
        let mut rng = crate::test::rng(213);
        let mut buf = [0.0; 4];
        for x in &mut buf {
            *x = rng.sample(skew_normal);
        }
        for value in buf.iter() {
            assert!(value.is_nan());
        }
    }

    #[test]
    fn skew_normal_distributions_can_be_compared() {
        assert_eq!(
            SkewNormal::new(1.0, 2.0, 3.0),
            SkewNormal::new(1.0, 2.0, 3.0)
        );
    }
}
