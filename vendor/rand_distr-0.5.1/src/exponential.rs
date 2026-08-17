// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The exponential distribution `Exp(λ)`.

use crate::utils::ziggurat;
use crate::{ziggurat_tables, Distribution};
use core::fmt;
use num_traits::Float;
use rand::Rng;

/// The standard exponential distribution `Exp(1)`.
///
/// This is equivalent to `Exp::new(1.0)` or sampling with
/// `-rng.gen::<f64>().ln()`, but faster.
///
/// See [`Exp`](crate::Exp) for the general exponential distribution.
///
/// # Plot
///
/// The following plot illustrates the exponential distribution with `λ = 1`.
///
/// ![Exponential distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/exponential_exp1.svg)
///
/// # Example
///
/// ```
/// use rand::prelude::*;
/// use rand_distr::Exp1;
///
/// let val: f64 = rand::rng().sample(Exp1);
/// println!("{}", val);
/// ```
///
/// # Notes
///
/// Implemented via the ZIGNOR variant[^1] of the Ziggurat method. The exact
/// description in the paper was adjusted to use tables for the exponential
/// distribution rather than normal.
///
/// [^1]: Jurgen A. Doornik (2005). [*An Improved Ziggurat Method to
///       Generate Normal Random Samples*](
///       https://www.doornik.com/research/ziggurat.pdf).
///       Nuffield College, Oxford
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Exp1;

impl Distribution<f32> for Exp1 {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f32 {
        // TODO: use optimal 32-bit implementation
        let x: f64 = self.sample(rng);
        x as f32
    }
}

// This could be done via `-rng.gen::<f64>().ln()` but that is slower.
impl Distribution<f64> for Exp1 {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        #[inline]
        fn pdf(x: f64) -> f64 {
            (-x).exp()
        }
        #[inline]
        fn zero_case<R: Rng + ?Sized>(rng: &mut R, _u: f64) -> f64 {
            ziggurat_tables::ZIG_EXP_R - rng.random::<f64>().ln()
        }

        ziggurat(
            rng,
            false,
            &ziggurat_tables::ZIG_EXP_X,
            &ziggurat_tables::ZIG_EXP_F,
            pdf,
            zero_case,
        )
    }
}

/// The [exponential distribution](https://en.wikipedia.org/wiki/Exponential_distribution) `Exp(λ)`.
///
/// The exponential distribution is a continuous probability distribution
/// with rate parameter `λ` (`lambda`). It describes the time between events
/// in a [`Poisson`](crate::Poisson) process, i.e. a process in which
/// events occur continuously and independently at a constant average rate.
///
/// See [`Exp1`](crate::Exp1) for an optimised implementation for `λ = 1`.
///
/// # Density function
///
/// `f(x) = λ * exp(-λ * x)` for `x > 0`, when `λ > 0`.
///
/// For `λ = 0`, all samples yield infinity (because a Poisson process
/// with rate 0 has no events).
///
/// # Plot
///
/// The following plot illustrates the exponential distribution with
/// various values of `λ`.
/// The `λ` parameter controls the rate of decay as `x` approaches infinity,
/// and the mean of the distribution is `1/λ`.
///
/// ![Exponential distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/exponential.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{Exp, Distribution};
///
/// let exp = Exp::new(2.0).unwrap();
/// let v = exp.sample(&mut rand::rng());
/// println!("{} is from a Exp(2) distribution", v);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Exp<F>
where
    F: Float,
    Exp1: Distribution<F>,
{
    /// `lambda` stored as `1/lambda`, since this is what we scale by.
    lambda_inverse: F,
}

/// Error type returned from [`Exp::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `lambda < 0` or `nan`.
    LambdaTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::LambdaTooSmall => "lambda is negative or NaN in exponential distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F: Float> Exp<F>
where
    F: Float,
    Exp1: Distribution<F>,
{
    /// Construct a new `Exp` with the given shape parameter
    /// `lambda`.
    ///
    /// # Remarks
    ///
    /// For custom types `N` implementing the [`Float`] trait,
    /// the case `lambda = 0` is handled as follows: each sample corresponds
    /// to a sample from an `Exp1` multiplied by `1 / 0`. Primitive types
    /// yield infinity, since `1 / 0 = infinity`.
    #[inline]
    pub fn new(lambda: F) -> Result<Exp<F>, Error> {
        if !(lambda >= F::zero()) {
            return Err(Error::LambdaTooSmall);
        }
        Ok(Exp {
            lambda_inverse: F::one() / lambda,
        })
    }
}

impl<F> Distribution<F> for Exp<F>
where
    F: Float,
    Exp1: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        rng.sample(Exp1) * self.lambda_inverse
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exp() {
        let exp = Exp::new(10.0).unwrap();
        let mut rng = crate::test::rng(221);
        for _ in 0..1000 {
            assert!(exp.sample(&mut rng) >= 0.0);
        }
    }
    #[test]
    fn test_zero() {
        let d = Exp::new(0.0).unwrap();
        assert_eq!(d.sample(&mut crate::test::rng(21)), f64::infinity());
    }
    #[test]
    #[should_panic]
    fn test_exp_invalid_lambda_neg() {
        Exp::new(-10.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_exp_invalid_lambda_nan() {
        Exp::new(f64::nan()).unwrap();
    }

    #[test]
    fn exponential_distributions_can_be_compared() {
        assert_eq!(Exp::new(1.0), Exp::new(1.0));
    }
}
