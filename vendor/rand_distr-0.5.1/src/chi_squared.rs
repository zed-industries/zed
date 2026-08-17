// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Chi-squared distribution.

use self::ChiSquaredRepr::*;

use crate::{Distribution, Exp1, Gamma, Open01, StandardNormal};
use core::fmt;
use num_traits::Float;
use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The [chi-squared distribution](https://en.wikipedia.org/wiki/Chi-squared_distribution) `χ²(k)`.
///
/// The chi-squared distribution is a continuous probability
/// distribution with parameter `k > 0` degrees of freedom.
///
/// For `k > 0` integral, this distribution is the sum of the squares
/// of `k` independent standard normal random variables. For other
/// `k`, this uses the equivalent characterisation
/// `χ²(k) = Gamma(k/2, 2)`.
///
/// # Plot
///
/// The plot shows the chi-squared distribution with various degrees
/// of freedom.
///
/// ![Chi-squared distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/chi_squared.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{ChiSquared, Distribution};
///
/// let chi = ChiSquared::new(11.0).unwrap();
/// let v = chi.sample(&mut rand::rng());
/// println!("{} is from a χ²(11) distribution", v)
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChiSquared<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    repr: ChiSquaredRepr<F>,
}

/// Error type returned from [`ChiSquared::new`] and [`StudentT::new`](crate::StudentT::new).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Error {
    /// `0.5 * k <= 0` or `nan`.
    DoFTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::DoFTooSmall => {
                "degrees-of-freedom k is not positive in chi-squared distribution"
            }
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum ChiSquaredRepr<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    // k == 1, Gamma(alpha, ..) is particularly slow for alpha < 1,
    // e.g. when alpha = 1/2 as it would be for this case, so special-
    // casing and using the definition of N(0,1)^2 is faster.
    DoFExactlyOne,
    DoFAnythingElse(Gamma<F>),
}

impl<F> ChiSquared<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Create a new chi-squared distribution with degrees-of-freedom
    /// `k`.
    pub fn new(k: F) -> Result<ChiSquared<F>, Error> {
        let repr = if k == F::one() {
            DoFExactlyOne
        } else {
            if !(F::from(0.5).unwrap() * k > F::zero()) {
                return Err(Error::DoFTooSmall);
            }
            DoFAnythingElse(Gamma::new(F::from(0.5).unwrap() * k, F::from(2.0).unwrap()).unwrap())
        };
        Ok(ChiSquared { repr })
    }
}
impl<F> Distribution<F> for ChiSquared<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        match self.repr {
            DoFExactlyOne => {
                // k == 1 => N(0,1)^2
                let norm: F = rng.sample(StandardNormal);
                norm * norm
            }
            DoFAnythingElse(ref g) => g.sample(rng),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chi_squared_one() {
        let chi = ChiSquared::new(1.0).unwrap();
        let mut rng = crate::test::rng(201);
        for _ in 0..1000 {
            chi.sample(&mut rng);
        }
    }
    #[test]
    fn test_chi_squared_small() {
        let chi = ChiSquared::new(0.5).unwrap();
        let mut rng = crate::test::rng(202);
        for _ in 0..1000 {
            chi.sample(&mut rng);
        }
    }
    #[test]
    fn test_chi_squared_large() {
        let chi = ChiSquared::new(30.0).unwrap();
        let mut rng = crate::test::rng(203);
        for _ in 0..1000 {
            chi.sample(&mut rng);
        }
    }
    #[test]
    #[should_panic]
    fn test_chi_squared_invalid_dof() {
        ChiSquared::new(-1.0).unwrap();
    }

    #[test]
    fn gamma_distributions_can_be_compared() {
        assert_eq!(Gamma::new(1.0, 2.0), Gamma::new(1.0, 2.0));
    }

    #[test]
    fn chi_squared_distributions_can_be_compared() {
        assert_eq!(ChiSquared::new(1.0), ChiSquared::new(1.0));
    }
}
