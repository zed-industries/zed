// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Fisher F-distribution.

use crate::{ChiSquared, Distribution, Exp1, Open01, StandardNormal};
use core::fmt;
use num_traits::Float;
use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The [Fisher F-distribution](https://en.wikipedia.org/wiki/F-distribution) `F(m, n)`.
///
/// This distribution is equivalent to the ratio of two normalised
/// chi-squared distributions, that is, `F(m,n) = (χ²(m)/m) /
/// (χ²(n)/n)`.
///
/// # Plot
///
/// The plot shows the F-distribution with various values of `m` and `n`.
///
/// ![F-distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/fisher_f.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{FisherF, Distribution};
///
/// let f = FisherF::new(2.0, 32.0).unwrap();
/// let v = f.sample(&mut rand::rng());
/// println!("{} is from an F(2, 32) distribution", v)
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FisherF<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    numer: ChiSquared<F>,
    denom: ChiSquared<F>,
    // denom_dof / numer_dof so that this can just be a straight
    // multiplication, rather than a division.
    dof_ratio: F,
}

/// Error type returned from [`FisherF::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Error {
    /// `m <= 0` or `nan`.
    MTooSmall,
    /// `n <= 0` or `nan`.
    NTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::MTooSmall => "m is not positive in Fisher F distribution",
            Error::NTooSmall => "n is not positive in Fisher F distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F> FisherF<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Create a new `FisherF` distribution, with the given parameter.
    pub fn new(m: F, n: F) -> Result<FisherF<F>, Error> {
        let zero = F::zero();
        if !(m > zero) {
            return Err(Error::MTooSmall);
        }
        if !(n > zero) {
            return Err(Error::NTooSmall);
        }

        Ok(FisherF {
            numer: ChiSquared::new(m).unwrap(),
            denom: ChiSquared::new(n).unwrap(),
            dof_ratio: n / m,
        })
    }
}
impl<F> Distribution<F> for FisherF<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        self.numer.sample(rng) / self.denom.sample(rng) * self.dof_ratio
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f() {
        let f = FisherF::new(2.0, 32.0).unwrap();
        let mut rng = crate::test::rng(204);
        for _ in 0..1000 {
            f.sample(&mut rng);
        }
    }

    #[test]
    fn fisher_f_distributions_can_be_compared() {
        assert_eq!(FisherF::new(1.0, 2.0), FisherF::new(1.0, 2.0));
    }
}
