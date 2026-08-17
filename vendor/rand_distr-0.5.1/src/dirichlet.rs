// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The dirichlet distribution `Dirichlet(α₁, α₂, ..., αₙ)`.

#![cfg(feature = "alloc")]
use crate::{Beta, Distribution, Exp1, Gamma, Open01, StandardNormal};
use core::fmt;
use num_traits::{Float, NumCast};
use rand::Rng;
#[cfg(feature = "serde")]
use serde_with::serde_as;

use alloc::{boxed::Box, vec, vec::Vec};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", serde_as)]
struct DirichletFromGamma<F, const N: usize>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    samplers: [Gamma<F>; N],
}

/// Error type returned from [`DirchletFromGamma::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirichletFromGammaError {
    /// Gamma::new(a, 1) failed.
    GammmaNewFailed,

    /// gamma_dists.try_into() failed (in theory, this should not happen).
    GammaArrayCreationFailed,
}

impl<F, const N: usize> DirichletFromGamma<F, N>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Construct a new `DirichletFromGamma` with the given parameters `alpha`.
    ///
    /// This function is part of a private implementation detail.
    /// It assumes that the input is correct, so no validation of alpha is done.
    #[inline]
    fn new(alpha: [F; N]) -> Result<DirichletFromGamma<F, N>, DirichletFromGammaError> {
        let mut gamma_dists = Vec::new();
        for a in alpha {
            let dist =
                Gamma::new(a, F::one()).map_err(|_| DirichletFromGammaError::GammmaNewFailed)?;
            gamma_dists.push(dist);
        }
        Ok(DirichletFromGamma {
            samplers: gamma_dists
                .try_into()
                .map_err(|_| DirichletFromGammaError::GammaArrayCreationFailed)?,
        })
    }
}

impl<F, const N: usize> Distribution<[F; N]> for DirichletFromGamma<F, N>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [F; N] {
        let mut samples = [F::zero(); N];
        let mut sum = F::zero();

        for (s, g) in samples.iter_mut().zip(self.samplers.iter()) {
            *s = g.sample(rng);
            sum = sum + *s;
        }
        let invacc = F::one() / sum;
        for s in samples.iter_mut() {
            *s = *s * invacc;
        }
        samples
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct DirichletFromBeta<F, const N: usize>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    samplers: Box<[Beta<F>]>,
}

/// Error type returned from [`DirchletFromBeta::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirichletFromBetaError {
    /// Beta::new(a, b) failed.
    BetaNewFailed,
}

impl<F, const N: usize> DirichletFromBeta<F, N>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Construct a new `DirichletFromBeta` with the given parameters `alpha`.
    ///
    /// This function is part of a private implementation detail.
    /// It assumes that the input is correct, so no validation of alpha is done.
    #[inline]
    fn new(alpha: [F; N]) -> Result<DirichletFromBeta<F, N>, DirichletFromBetaError> {
        // `alpha_rev_csum` is the reverse of the cumulative sum of the
        // reverse of `alpha[1..]`.  E.g. if `alpha = [a0, a1, a2, a3]`, then
        // `alpha_rev_csum` is `[a1 + a2 + a3, a2 + a3, a3]`.
        // Note that instances of DirichletFromBeta will always have N >= 2,
        // so the subtractions of 1, 2 and 3 from N in the following are safe.
        let mut alpha_rev_csum = vec![alpha[N - 1]; N - 1];
        for k in 0..(N - 2) {
            alpha_rev_csum[N - 3 - k] = alpha_rev_csum[N - 2 - k] + alpha[N - 2 - k];
        }

        // Zip `alpha[..(N-1)]` and `alpha_rev_csum`; for the example
        // `alpha = [a0, a1, a2, a3]`, the zip result holds the tuples
        // `[(a0, a1+a2+a3), (a1, a2+a3), (a2, a3)]`.
        // Then pass each tuple to `Beta::new()` to create the `Beta`
        // instances.
        let mut beta_dists = Vec::new();
        for (&a, &b) in alpha[..(N - 1)].iter().zip(alpha_rev_csum.iter()) {
            let dist = Beta::new(a, b).map_err(|_| DirichletFromBetaError::BetaNewFailed)?;
            beta_dists.push(dist);
        }
        Ok(DirichletFromBeta {
            samplers: beta_dists.into_boxed_slice(),
        })
    }
}

impl<F, const N: usize> Distribution<[F; N]> for DirichletFromBeta<F, N>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [F; N] {
        let mut samples = [F::zero(); N];
        let mut acc = F::one();

        for (s, beta) in samples.iter_mut().zip(self.samplers.iter()) {
            let beta_sample = beta.sample(rng);
            *s = acc * beta_sample;
            acc = acc * (F::one() - beta_sample);
        }
        samples[N - 1] = acc;
        samples
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", serde_as)]
enum DirichletRepr<F, const N: usize>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Dirichlet distribution that generates samples using the Gamma distribution.
    FromGamma(DirichletFromGamma<F, N>),

    /// Dirichlet distribution that generates samples using the Beta distribution.
    FromBeta(DirichletFromBeta<F, N>),
}

/// The [Dirichlet distribution](https://en.wikipedia.org/wiki/Dirichlet_distribution) `Dirichlet(α₁, α₂, ..., αₖ)`.
///
/// The Dirichlet distribution is a family of continuous multivariate
/// probability distributions parameterized by a vector of positive
/// real numbers `α₁, α₂, ..., αₖ`, where `k` is the number of dimensions
/// of the distribution. The distribution is supported on the `k-1`-dimensional
/// simplex, which is the set of points `x = [x₁, x₂, ..., xₖ]` such that
/// `0 ≤ xᵢ ≤ 1` and `∑ xᵢ = 1`.
/// It is a multivariate generalization of the [`Beta`](crate::Beta) distribution.
/// The distribution is symmetric when all `αᵢ` are equal.
///
/// # Plot
///
/// The following plot illustrates the 2-dimensional simplices for various
/// 3-dimensional Dirichlet distributions.
///
/// ![Dirichlet distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/dirichlet.png)
///
/// # Example
///
/// ```
/// use rand::prelude::*;
/// use rand_distr::Dirichlet;
///
/// let dirichlet = Dirichlet::new([1.0, 2.0, 3.0]).unwrap();
/// let samples = dirichlet.sample(&mut rand::rng());
/// println!("{:?} is from a Dirichlet([1.0, 2.0, 3.0]) distribution", samples);
/// ```
#[cfg_attr(feature = "serde", serde_as)]
#[derive(Clone, Debug, PartialEq)]
pub struct Dirichlet<F, const N: usize>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    repr: DirichletRepr<F, N>,
}

/// Error type returned from [`Dirichlet::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `alpha.len() < 2`.
    AlphaTooShort,
    /// `alpha <= 0.0` or `nan`.
    AlphaTooSmall,
    /// `alpha` is subnormal.
    /// Variate generation methods are not reliable with subnormal inputs.
    AlphaSubnormal,
    /// `alpha` is infinite.
    AlphaInfinite,
    /// Failed to create required Gamma distribution(s).
    FailedToCreateGamma,
    /// Failed to create required Beta distribition(s).
    FailedToCreateBeta,
    /// `size < 2`.
    SizeTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::AlphaTooShort | Error::SizeTooSmall => {
                "less than 2 dimensions in Dirichlet distribution"
            }
            Error::AlphaTooSmall => "alpha is not positive in Dirichlet distribution",
            Error::AlphaSubnormal => "alpha contains a subnormal value in Dirichlet distribution",
            Error::AlphaInfinite => "alpha contains an infinite value in Dirichlet distribution",
            Error::FailedToCreateGamma => {
                "failed to create required Gamma distribution for Dirichlet distribution"
            }
            Error::FailedToCreateBeta => {
                "failed to create required Beta distribition for Dirichlet distribution"
            }
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F, const N: usize> Dirichlet<F, N>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Construct a new `Dirichlet` with the given alpha parameter `alpha`.
    ///
    /// Requires `alpha.len() >= 2`, and each value in `alpha` must be positive,
    /// finite and not subnormal.
    #[inline]
    pub fn new(alpha: [F; N]) -> Result<Dirichlet<F, N>, Error> {
        if N < 2 {
            return Err(Error::AlphaTooShort);
        }
        for &ai in alpha.iter() {
            if !(ai > F::zero()) {
                // This also catches nan.
                return Err(Error::AlphaTooSmall);
            }
            if ai.is_infinite() {
                return Err(Error::AlphaInfinite);
            }
            if !ai.is_normal() {
                return Err(Error::AlphaSubnormal);
            }
        }

        if alpha.iter().all(|&x| x <= NumCast::from(0.1).unwrap()) {
            // Use the Beta method when all the alphas are less than 0.1  This
            // threshold provides a reasonable compromise between using the faster
            // Gamma method for as wide a range as possible while ensuring that
            // the probability of generating nans is negligibly small.
            let dist = DirichletFromBeta::new(alpha).map_err(|_| Error::FailedToCreateBeta)?;
            Ok(Dirichlet {
                repr: DirichletRepr::FromBeta(dist),
            })
        } else {
            let dist = DirichletFromGamma::new(alpha).map_err(|_| Error::FailedToCreateGamma)?;
            Ok(Dirichlet {
                repr: DirichletRepr::FromGamma(dist),
            })
        }
    }
}

impl<F, const N: usize> Distribution<[F; N]> for Dirichlet<F, N>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [F; N] {
        match &self.repr {
            DirichletRepr::FromGamma(dirichlet) => dirichlet.sample(rng),
            DirichletRepr::FromBeta(dirichlet) => dirichlet.sample(rng),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dirichlet() {
        let d = Dirichlet::new([1.0, 2.0, 3.0]).unwrap();
        let mut rng = crate::test::rng(221);
        let samples = d.sample(&mut rng);
        assert!(samples.into_iter().all(|x: f64| x > 0.0));
    }

    #[test]
    #[should_panic]
    fn test_dirichlet_invalid_length() {
        Dirichlet::new([0.5]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_dirichlet_alpha_zero() {
        Dirichlet::new([0.1, 0.0, 0.3]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_dirichlet_alpha_negative() {
        Dirichlet::new([0.1, -1.5, 0.3]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_dirichlet_alpha_nan() {
        Dirichlet::new([0.5, f64::NAN, 0.25]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_dirichlet_alpha_subnormal() {
        Dirichlet::new([0.5, 1.5e-321, 0.25]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_dirichlet_alpha_inf() {
        Dirichlet::new([0.5, f64::INFINITY, 0.25]).unwrap();
    }

    #[test]
    fn dirichlet_distributions_can_be_compared() {
        assert_eq!(Dirichlet::new([1.0, 2.0]), Dirichlet::new([1.0, 2.0]));
    }

    /// Check that the means of the components of n samples from
    /// the Dirichlet distribution agree with the expected means
    /// with a relative tolerance of rtol.
    ///
    /// This is a crude statistical test, but it will catch egregious
    /// mistakes.  It will also also fail if any samples contain nan.
    fn check_dirichlet_means<const N: usize>(alpha: [f64; N], n: i32, rtol: f64, seed: u64) {
        let d = Dirichlet::new(alpha).unwrap();
        let mut rng = crate::test::rng(seed);
        let mut sums = [0.0; N];
        for _ in 0..n {
            let samples = d.sample(&mut rng);
            for i in 0..N {
                sums[i] += samples[i];
            }
        }
        let sample_mean = sums.map(|x| x / n as f64);
        let alpha_sum: f64 = alpha.iter().sum();
        let expected_mean = alpha.map(|x| x / alpha_sum);
        for i in 0..N {
            assert_almost_eq!(sample_mean[i], expected_mean[i], rtol);
        }
    }

    #[test]
    fn test_dirichlet_means() {
        // Check the means of 20000 samples for several different alphas.
        let n = 20000;
        let rtol = 2e-2;
        let seed = 1317624576693539401;
        check_dirichlet_means([0.5, 0.25], n, rtol, seed);
        check_dirichlet_means([123.0, 75.0], n, rtol, seed);
        check_dirichlet_means([2.0, 2.5, 5.0, 7.0], n, rtol, seed);
        check_dirichlet_means([0.1, 8.0, 1.0, 2.0, 2.0, 0.85, 0.05, 12.5], n, rtol, seed);
    }

    #[test]
    fn test_dirichlet_means_very_small_alpha() {
        // With values of alpha that are all 0.001, check that the means of the
        // components of 10000 samples are within 1% of the expected means.
        // With the sampling method based on gamma variates, this test would
        // fail, with about 10% of the samples containing nan.
        let alpha = [0.001; 3];
        let n = 10000;
        let rtol = 1e-2;
        let seed = 1317624576693539401;
        check_dirichlet_means(alpha, n, rtol, seed);
    }

    #[test]
    fn test_dirichlet_means_small_alpha() {
        // With values of alpha that are all less than 0.1, check that the
        // means of the components of 150000 samples are within 0.1% of the
        // expected means.
        let alpha = [0.05, 0.025, 0.075, 0.05];
        let n = 150000;
        let rtol = 1e-3;
        let seed = 1317624576693539401;
        check_dirichlet_means(alpha, n, rtol, seed);
    }
}
