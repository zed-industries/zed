// Copyright 2019 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://rust-random.github.io/rand/"
)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(
    clippy::excessive_precision,
    clippy::float_cmp,
    clippy::unreadable_literal
)]
#![allow(clippy::neg_cmp_op_on_partial_ord)] // suggested fix too verbose
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Generating random samples from probability distributions.
//!
//! ## Re-exports
//!
//! This crate is a super-set of the [`rand::distr`] module. See the
//! [`rand::distr`] module documentation for an overview of the core
//! [`Distribution`] trait and implementations.
//!
//! The following are re-exported:
//!
//! - The [`Distribution`] trait and [`Iter`] helper type
//! - The [`StandardUniform`], [`Alphanumeric`], [`Uniform`], [`OpenClosed01`],
//!   [`Open01`], [`Bernoulli`] distributions
//! - The [`weighted`] module
//!
//! ## Distributions
//!
//! This crate provides the following probability distributions:
//!
//! - Related to real-valued quantities that grow linearly
//!   (e.g. errors, offsets):
//!   - [`Normal`] distribution, and [`StandardNormal`] as a primitive
//!   - [`SkewNormal`] distribution
//!   - [`Cauchy`] distribution
//! - Related to Bernoulli trials (yes/no events, with a given probability):
//!   - [`Binomial`] distribution
//!   - [`Geometric`] distribution
//!   - [`Hypergeometric`] distribution
//! - Related to positive real-valued quantities that grow exponentially
//!   (e.g. prices, incomes, populations):
//!   - [`LogNormal`] distribution
//! - Related to the occurrence of independent events at a given rate:
//!   - [`Pareto`] distribution
//!   - [`Poisson`] distribution
//!   - [`Exp`]onential distribution, and [`Exp1`] as a primitive
//!   - [`Weibull`] distribution
//!   - [`Gumbel`] distribution
//!   - [`Frechet`] distribution
//!   - [`Zeta`] distribution
//!   - [`Zipf`] distribution
//! - Gamma and derived distributions:
//!   - [`Gamma`] distribution
//!   - [`ChiSquared`] distribution
//!   - [`StudentT`] distribution
//!   - [`FisherF`] distribution
//! - Triangular distribution:
//!   - [`Beta`] distribution
//!   - [`Triangular`] distribution
//! - Multivariate probability distributions
//!   - [`Dirichlet`] distribution
//!   - [`UnitSphere`] distribution
//!   - [`UnitBall`] distribution
//!   - [`UnitCircle`] distribution
//!   - [`UnitDisc`] distribution
//! - Misc. distributions
//!   - [`InverseGaussian`] distribution
//!   - [`NormalInverseGaussian`] distribution

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// This is used for doc links:
#[allow(unused)]
use rand::Rng;

pub use rand::distr::{
    uniform, Alphanumeric, Bernoulli, BernoulliError, Distribution, Iter, Open01, OpenClosed01,
    StandardUniform, Uniform,
};

pub use self::beta::{Beta, Error as BetaError};
pub use self::binomial::{Binomial, Error as BinomialError};
pub use self::cauchy::{Cauchy, Error as CauchyError};
pub use self::chi_squared::{ChiSquared, Error as ChiSquaredError};
#[cfg(feature = "alloc")]
pub use self::dirichlet::{Dirichlet, Error as DirichletError};
pub use self::exponential::{Error as ExpError, Exp, Exp1};
pub use self::fisher_f::{Error as FisherFError, FisherF};
pub use self::frechet::{Error as FrechetError, Frechet};
pub use self::gamma::{Error as GammaError, Gamma};
pub use self::geometric::{Error as GeoError, Geometric, StandardGeometric};
pub use self::gumbel::{Error as GumbelError, Gumbel};
pub use self::hypergeometric::{Error as HyperGeoError, Hypergeometric};
pub use self::inverse_gaussian::{Error as InverseGaussianError, InverseGaussian};
pub use self::normal::{Error as NormalError, LogNormal, Normal, StandardNormal};
pub use self::normal_inverse_gaussian::{
    Error as NormalInverseGaussianError, NormalInverseGaussian,
};
pub use self::pareto::{Error as ParetoError, Pareto};
pub use self::pert::{Pert, PertBuilder, PertError};
pub use self::poisson::{Error as PoissonError, Poisson};
pub use self::skew_normal::{Error as SkewNormalError, SkewNormal};
pub use self::triangular::{Triangular, TriangularError};
pub use self::unit_ball::UnitBall;
pub use self::unit_circle::UnitCircle;
pub use self::unit_disc::UnitDisc;
pub use self::unit_sphere::UnitSphere;
pub use self::weibull::{Error as WeibullError, Weibull};
pub use self::zeta::{Error as ZetaError, Zeta};
pub use self::zipf::{Error as ZipfError, Zipf};
pub use student_t::StudentT;

pub use num_traits;

#[cfg(feature = "alloc")]
pub mod weighted;

#[cfg(test)]
#[macro_use]
mod test {
    // Notes on testing
    //
    // Testing random number distributions correctly is hard. The following
    // testing is desired:
    //
    // - Construction: test initialisation with a few valid parameter sets.
    // - Erroneous usage: test that incorrect usage generates an error.
    // - Vector: test that usage with fixed inputs (including RNG) generates a
    //   fixed output sequence on all platforms.
    // - Correctness at fixed points (optional): using a specific mock RNG,
    //   check that specific values are sampled (e.g. end-points and median of
    //   distribution).
    // - Correctness of PDF (extra): generate a histogram of samples within a
    //   certain range, and check this approximates the PDF. These tests are
    //   expected to be expensive, and should be behind a feature-gate.
    //
    // TODO: Vector and correctness tests are largely absent so far.
    // NOTE: Some distributions have tests checking only that samples can be
    // generated. This is redundant with vector and correctness tests.

    /// Construct a deterministic RNG with the given seed
    pub fn rng(seed: u64) -> impl rand::RngCore {
        // For tests, we want a statistically good, fast, reproducible RNG.
        // PCG32 will do fine, and will be easy to embed if we ever need to.
        const INC: u64 = 11634580027462260723;
        rand_pcg::Pcg32::new(seed, INC)
    }

    /// Assert that two numbers are almost equal to each other.
    ///
    /// On panic, this macro will print the values of the expressions with their
    /// debug representations.
    macro_rules! assert_almost_eq {
        ($a:expr, $b:expr, $prec:expr) => {
            let diff = ($a - $b).abs();
            assert!(
                diff <= $prec,
                "assertion failed: `abs(left - right) = {:.1e} < {:e}`, \
                    (left: `{}`, right: `{}`)",
                diff,
                $prec,
                $a,
                $b
            );
        };
    }
}

mod beta;
mod binomial;
mod cauchy;
mod chi_squared;
mod dirichlet;
mod exponential;
mod fisher_f;
mod frechet;
mod gamma;
mod geometric;
mod gumbel;
mod hypergeometric;
mod inverse_gaussian;
mod normal;
mod normal_inverse_gaussian;
mod pareto;
mod pert;
pub(crate) mod poisson;
mod skew_normal;
mod student_t;
mod triangular;
mod unit_ball;
mod unit_circle;
mod unit_disc;
mod unit_sphere;
mod utils;
mod weibull;
mod zeta;
mod ziggurat_tables;
mod zipf;
