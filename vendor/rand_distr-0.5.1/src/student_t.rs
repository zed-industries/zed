// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Student's t-distribution.

use crate::{ChiSquared, ChiSquaredError};
use crate::{Distribution, Exp1, Open01, StandardNormal};
use num_traits::Float;
use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The [Student t-distribution](https://en.wikipedia.org/wiki/Student%27s_t-distribution) `t(ν)`.
///
/// The t-distribution is a continuous probability distribution
/// parameterized by degrees of freedom `ν` (`nu`), which
/// arises when estimating the mean of a normally-distributed
/// population in situations where the sample size is small and
/// the population's standard deviation is unknown.
/// It is widely used in hypothesis testing.
///
/// For `ν = 1`, this is equivalent to the standard
/// [`Cauchy`](crate::Cauchy) distribution,
/// and as `ν` diverges to infinity, `t(ν)` converges to
/// [`StandardNormal`](crate::StandardNormal).
///
/// # Plot
///
/// The plot shows the t-distribution with various degrees of freedom.
///
/// ![T-distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/student_t.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{StudentT, Distribution};
///
/// let t = StudentT::new(11.0).unwrap();
/// let v = t.sample(&mut rand::rng());
/// println!("{} is from a t(11) distribution", v)
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StudentT<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    chi: ChiSquared<F>,
    dof: F,
}

impl<F> StudentT<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Create a new Student t-distribution with `ν` (nu)
    /// degrees of freedom.
    pub fn new(nu: F) -> Result<StudentT<F>, ChiSquaredError> {
        Ok(StudentT {
            chi: ChiSquared::new(nu)?,
            dof: nu,
        })
    }
}
impl<F> Distribution<F> for StudentT<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let norm: F = rng.sample(StandardNormal);
        norm * (self.dof / self.chi.sample(rng)).sqrt()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_t() {
        let t = StudentT::new(11.0).unwrap();
        let mut rng = crate::test::rng(205);
        for _ in 0..1000 {
            t.sample(&mut rng);
        }
    }

    #[test]
    fn student_t_distributions_can_be_compared() {
        assert_eq!(StudentT::new(1.0), StudentT::new(1.0));
    }
}
