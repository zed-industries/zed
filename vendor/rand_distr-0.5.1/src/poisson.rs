// Copyright 2018 Developers of the Rand project.
// Copyright 2016-2017 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Poisson distribution `Poisson(λ)`.

use crate::{Distribution, Exp1, Normal, StandardNormal, StandardUniform};
use core::fmt;
use num_traits::{Float, FloatConst};
use rand::Rng;

/// The [Poisson distribution](https://en.wikipedia.org/wiki/Poisson_distribution) `Poisson(λ)`.
///
/// The Poisson distribution is a discrete probability distribution with
/// rate parameter `λ` (`lambda`). It models the number of events occurring in a fixed
/// interval of time or space.
///
/// This distribution has density function:
/// `f(k) = λ^k * exp(-λ) / k!` for `k >= 0`.
///
/// # Plot
///
/// The following plot shows the Poisson distribution with various values of `λ`.
/// Note how the expected number of events increases with `λ`.
///
/// ![Poisson distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/poisson.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{Poisson, Distribution};
///
/// let poi = Poisson::new(2.0).unwrap();
/// let v: f64 = poi.sample(&mut rand::rng());
/// println!("{} is from a Poisson(2) distribution", v);
/// ```
///
/// # Integer vs FP return type
///
/// This implementation uses floating-point (FP) logic internally.
///
/// Due to the parameter limit <code>λ < [Self::MAX_LAMBDA]</code>, it
/// statistically impossible to sample a value larger [`u64::MAX`]. As such, it
/// is reasonable to cast generated samples to `u64` using `as`:
/// `distr.sample(&mut rng) as u64` (and memory safe since Rust 1.45).
/// Similarly, when `λ < 4.2e9` it can be safely assumed that samples are less
/// than `u32::MAX`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Poisson<F>(Method<F>)
where
    F: Float + FloatConst,
    StandardUniform: Distribution<F>;

/// Error type returned from [`Poisson::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `lambda <= 0`
    ShapeTooSmall,
    /// `lambda = ∞` or `lambda = nan`
    NonFinite,
    /// `lambda` is too large, see [Poisson::MAX_LAMBDA]
    ShapeTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::ShapeTooSmall => "lambda is not positive in Poisson distribution",
            Error::NonFinite => "lambda is infinite or nan in Poisson distribution",
            Error::ShapeTooLarge => {
                "lambda is too large in Poisson distribution, see Poisson::MAX_LAMBDA"
            }
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct KnuthMethod<F> {
    exp_lambda: F,
}

impl<F: Float> KnuthMethod<F> {
    pub(crate) fn new(lambda: F) -> Self {
        KnuthMethod {
            exp_lambda: (-lambda).exp(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct RejectionMethod<F> {
    lambda: F,
    s: F,
    d: F,
    l: F,
    c: F,
    c0: F,
    c1: F,
    c2: F,
    c3: F,
    omega: F,
}

impl<F: Float + FloatConst> RejectionMethod<F> {
    pub(crate) fn new(lambda: F) -> Self {
        let b1 = F::from(1.0 / 24.0).unwrap() / lambda;
        let b2 = F::from(0.3).unwrap() * b1 * b1;
        let c3 = F::from(1.0 / 7.0).unwrap() * b1 * b2;
        let c2 = b2 - F::from(15).unwrap() * c3;
        let c1 = b1 - F::from(6).unwrap() * b2 + F::from(45).unwrap() * c3;
        let c0 = F::one() - b1 + F::from(3).unwrap() * b2 - F::from(15).unwrap() * c3;

        RejectionMethod {
            lambda,
            s: lambda.sqrt(),
            d: F::from(6.0).unwrap() * lambda.powi(2),
            l: (lambda - F::from(1.1484).unwrap()).floor(),
            c: F::from(0.1069).unwrap() / lambda,
            c0,
            c1,
            c2,
            c3,
            omega: F::one() / (F::from(2).unwrap() * F::PI()).sqrt() / lambda.sqrt(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum Method<F> {
    Knuth(KnuthMethod<F>),
    Rejection(RejectionMethod<F>),
}

impl<F> Poisson<F>
where
    F: Float + FloatConst,
    StandardUniform: Distribution<F>,
{
    /// Construct a new `Poisson` with the given shape parameter
    /// `lambda`.
    ///
    /// The maximum allowed lambda is [MAX_LAMBDA](Self::MAX_LAMBDA).
    pub fn new(lambda: F) -> Result<Poisson<F>, Error> {
        if !lambda.is_finite() {
            return Err(Error::NonFinite);
        }
        if !(lambda > F::zero()) {
            return Err(Error::ShapeTooSmall);
        }

        // Use the Knuth method only for low expected values
        let method = if lambda < F::from(12.0).unwrap() {
            Method::Knuth(KnuthMethod::new(lambda))
        } else {
            if lambda > F::from(Self::MAX_LAMBDA).unwrap() {
                return Err(Error::ShapeTooLarge);
            }
            Method::Rejection(RejectionMethod::new(lambda))
        };

        Ok(Poisson(method))
    }

    /// The maximum supported value of `lambda`
    ///
    /// This value was selected such that
    /// `MAX_LAMBDA + 1e6 * sqrt(MAX_LAMBDA) < 2^64 - 1`,
    /// thus ensuring that the probability of sampling a value larger than
    /// `u64::MAX` is less than 1e-1000.
    ///
    /// Applying this limit also solves
    /// [#1312](https://github.com/rust-random/rand/issues/1312).
    pub const MAX_LAMBDA: f64 = 1.844e19;
}

impl<F> Distribution<F> for KnuthMethod<F>
where
    F: Float + FloatConst,
    StandardUniform: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let mut result = F::one();
        let mut p = rng.random::<F>();
        while p > self.exp_lambda {
            p = p * rng.random::<F>();
            result = result + F::one();
        }
        result - F::one()
    }
}

impl<F> Distribution<F> for RejectionMethod<F>
where
    F: Float + FloatConst,
    StandardUniform: Distribution<F>,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        // The algorithm is based on:
        // J. H. Ahrens and U. Dieter. 1982.
        // Computer Generation of Poisson Deviates from Modified Normal Distributions.
        // ACM Trans. Math. Softw. 8, 2 (June 1982), 163–179. https://doi.org/10.1145/355993.355997

        // Step F
        let f = |k: F| {
            const FACT: [f64; 10] = [
                1.0, 1.0, 2.0, 6.0, 24.0, 120.0, 720.0, 5040.0, 40320.0, 362880.0,
            ]; // factorial of 0..10
            const A: [f64; 10] = [
                -0.5000000002,
                0.3333333343,
                -0.2499998565,
                0.1999997049,
                -0.1666848753,
                0.1428833286,
                -0.1241963125,
                0.1101687109,
                -0.1142650302,
                0.1055093006,
            ]; // coefficients from Table 1
            let (px, py) = if k < F::from(10.0).unwrap() {
                let px = -self.lambda;
                let py = self.lambda.powf(k) / F::from(FACT[k.to_usize().unwrap()]).unwrap();

                (px, py)
            } else {
                let delta = (F::from(12.0).unwrap() * k).recip();
                let delta = delta - F::from(4.8).unwrap() * delta.powi(3);
                let v = (self.lambda - k) / k;

                let px = if v.abs() <= F::from(0.25).unwrap() {
                    k * v.powi(2)
                        * A.iter()
                            .rev()
                            .fold(F::zero(), |acc, &a| {
                                acc * v + F::from(a).unwrap()
                            }) // Σ a_i * v^i
                        - delta
                } else {
                    k * (F::one() + v).ln() - (self.lambda - k) - delta
                };

                let py = F::one() / (F::from(2.0).unwrap() * F::PI()).sqrt() / k.sqrt();

                (px, py)
            };

            let x = (k - self.lambda + F::from(0.5).unwrap()) / self.s;
            let fx = -F::from(0.5).unwrap() * x * x;
            let fy =
                self.omega * (((self.c3 * x * x + self.c2) * x * x + self.c1) * x * x + self.c0);

            (px, py, fx, fy)
        };

        // Step N
        let normal = Normal::new(self.lambda, self.s).unwrap();
        let g = normal.sample(rng);
        if g >= F::zero() {
            let k1 = g.floor();

            // Step I
            if k1 >= self.l {
                return k1;
            }

            // Step S
            let u: F = rng.random();
            if self.d * u >= (self.lambda - k1).powi(3) {
                return k1;
            }

            let (px, py, fx, fy) = f(k1);

            if fy * (F::one() - u) <= py * (px - fx).exp() {
                return k1;
            }
        }

        loop {
            // Step E
            let e = Exp1.sample(rng);
            let u: F = rng.random() * F::from(2.0).unwrap() - F::one();
            let t = F::from(1.8).unwrap() + e * u.signum();
            if t > F::from(-0.6744).unwrap() {
                let k2 = (self.lambda + self.s * t).floor();
                let (px, py, fx, fy) = f(k2);
                // Step H
                if self.c * u.abs() <= py * (px + e).exp() - fy * (fx + e).exp() {
                    return k2;
                }
            }
        }
    }
}

impl<F> Distribution<F> for Poisson<F>
where
    F: Float + FloatConst,
    StandardUniform: Distribution<F>,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        match &self.0 {
            Method::Knuth(method) => method.sample(rng),
            Method::Rejection(method) => method.sample(rng),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_poisson_invalid_lambda_zero() {
        Poisson::new(0.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_poisson_invalid_lambda_infinity() {
        Poisson::new(f64::INFINITY).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_poisson_invalid_lambda_neg() {
        Poisson::new(-10.0).unwrap();
    }

    #[test]
    fn poisson_distributions_can_be_compared() {
        assert_eq!(Poisson::new(1.0), Poisson::new(1.0));
    }
}
