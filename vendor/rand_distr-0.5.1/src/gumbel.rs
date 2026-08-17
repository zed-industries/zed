// Copyright 2021 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Gumbel distribution `Gumbel(μ, β)`.

use crate::{Distribution, OpenClosed01};
use core::fmt;
use num_traits::Float;
use rand::Rng;

/// The [Gumbel distribution](https://en.wikipedia.org/wiki/Gumbel_distribution) `Gumbel(μ, β)`.
///
/// The Gumbel distribution is a continuous probability distribution
/// with location parameter `μ` (`mu`) and scale parameter `β` (`beta`).
/// It is used to model the distribution of the maximum (or minimum)
/// of a number of samples of various distributions.
///
/// # Density function
///
/// `f(x) = exp(-(z + exp(-z))) / β`, where `z = (x - μ) / β`.
///
/// # Plot
///
/// The following plot illustrates the Gumbel distribution with various values of `μ` and `β`.
/// Note how the location parameter `μ` shifts the distribution along the x-axis,
/// and the scale parameter `β` changes the density around `μ`.
/// Note also the asymptotic behavior of the distribution towards the right.
///
/// ![Gumbel distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/gumbel.svg)
///
/// # Example
/// ```
/// use rand::prelude::*;
/// use rand_distr::Gumbel;
///
/// let val: f64 = rand::rng().sample(Gumbel::new(0.0, 1.0).unwrap());
/// println!("{}", val);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gumbel<F>
where
    F: Float,
    OpenClosed01: Distribution<F>,
{
    location: F,
    scale: F,
}

/// Error type returned from [`Gumbel::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// location is infinite or NaN
    LocationNotFinite,
    /// scale is not finite positive number
    ScaleNotPositive,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::ScaleNotPositive => "scale is not positive and finite in Gumbel distribution",
            Error::LocationNotFinite => "location is not finite in Gumbel distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F> Gumbel<F>
where
    F: Float,
    OpenClosed01: Distribution<F>,
{
    /// Construct a new `Gumbel` distribution with given `location` and `scale`.
    pub fn new(location: F, scale: F) -> Result<Gumbel<F>, Error> {
        if scale <= F::zero() || scale.is_infinite() || scale.is_nan() {
            return Err(Error::ScaleNotPositive);
        }
        if location.is_infinite() || location.is_nan() {
            return Err(Error::LocationNotFinite);
        }
        Ok(Gumbel { location, scale })
    }
}

impl<F> Distribution<F> for Gumbel<F>
where
    F: Float,
    OpenClosed01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let x: F = rng.sample(OpenClosed01);
        self.location - self.scale * (-x.ln()).ln()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_zero_scale() {
        Gumbel::new(0.0, 0.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_infinite_scale() {
        Gumbel::new(0.0, f64::INFINITY).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_nan_scale() {
        Gumbel::new(0.0, f64::NAN).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_infinite_location() {
        Gumbel::new(f64::INFINITY, 1.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_nan_location() {
        Gumbel::new(f64::NAN, 1.0).unwrap();
    }

    #[test]
    fn test_sample_against_cdf() {
        fn neg_log_log(x: f64) -> f64 {
            -(-x.ln()).ln()
        }
        let location = 0.0;
        let scale = 1.0;
        let iterations = 100_000;
        let increment = 1.0 / iterations as f64;
        let probabilities = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let mut quantiles = [0.0; 9];
        for (i, p) in probabilities.iter().enumerate() {
            quantiles[i] = neg_log_log(*p);
        }
        let mut proportions = [0.0; 9];
        let d = Gumbel::new(location, scale).unwrap();
        let mut rng = crate::test::rng(1);
        for _ in 0..iterations {
            let replicate = d.sample(&mut rng);
            for (i, q) in quantiles.iter().enumerate() {
                if replicate < *q {
                    proportions[i] += increment;
                }
            }
        }
        assert!(proportions
            .iter()
            .zip(&probabilities)
            .all(|(p_hat, p)| (p_hat - p).abs() < 0.003))
    }

    #[test]
    fn gumbel_distributions_can_be_compared() {
        assert_eq!(Gumbel::new(1.0, 2.0), Gumbel::new(1.0, 2.0));
    }
}
