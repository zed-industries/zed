// Copyright 2021 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Zeta distribution.

use crate::{Distribution, StandardUniform};
use core::fmt;
use num_traits::Float;
use rand::{distr::OpenClosed01, Rng};

/// The [Zeta distribution](https://en.wikipedia.org/wiki/Zeta_distribution) `Zeta(s)`.
///
/// The [Zeta distribution](https://en.wikipedia.org/wiki/Zeta_distribution)
/// is a discrete probability distribution with parameter `s`.
/// It is a special case of the [`Zipf`](crate::Zipf) distribution with `n = ∞`.
/// It is also known as the discrete Pareto, Riemann-Zeta, Zipf, or Zipf–Estoup distribution.
///
/// # Density function
///
/// `f(k) = k^(-s) / ζ(s)` for `k >= 1`, where `ζ` is the
/// [Riemann zeta function](https://en.wikipedia.org/wiki/Riemann_zeta_function).
///
/// # Plot
///
/// The following plot illustrates the zeta distribution for various values of `s`.
///
/// ![Zeta distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/zeta.svg)
///
/// # Example
/// ```
/// use rand::prelude::*;
/// use rand_distr::Zeta;
///
/// let val: f64 = rand::rng().sample(Zeta::new(1.5).unwrap());
/// println!("{}", val);
/// ```
///
/// # Integer vs FP return type
///
/// This implementation uses floating-point (FP) logic internally, which can
/// potentially generate very large samples (exceeding e.g. `u64::MAX`).
///
/// It is *safe* to cast such results to an integer type using `as`
/// (e.g. `distr.sample(&mut rng) as u64`), since such casts are saturating
/// (e.g. `2f64.powi(64) as u64 == u64::MAX`). It is up to the user to
/// determine whether this potential loss of accuracy is acceptable
/// (this determination may depend on the distribution's parameters).
///
/// # Notes
///
/// The zeta distribution has no upper limit. Sampled values may be infinite.
/// In particular, a value of infinity might be returned for the following
/// reasons:
/// 1. it is the best representation in the type `F` of the actual sample.
/// 2. to prevent infinite loops for very small `s`.
///
/// # Implementation details
///
/// We are using the algorithm from
/// [Non-Uniform Random Variate Generation](https://doi.org/10.1007/978-1-4613-8643-8),
/// Section 6.1, page 551.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zeta<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
    OpenClosed01: Distribution<F>,
{
    s_minus_1: F,
    b: F,
}

/// Error type returned from [`Zeta::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `s <= 1` or `nan`.
    STooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::STooSmall => "s <= 1 or is NaN in Zeta distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F> Zeta<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
    OpenClosed01: Distribution<F>,
{
    /// Construct a new `Zeta` distribution with given `s` parameter.
    #[inline]
    pub fn new(s: F) -> Result<Zeta<F>, Error> {
        if !(s > F::one()) {
            return Err(Error::STooSmall);
        }
        let s_minus_1 = s - F::one();
        let two = F::one() + F::one();
        Ok(Zeta {
            s_minus_1,
            b: two.powf(s_minus_1),
        })
    }
}

impl<F> Distribution<F> for Zeta<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
    OpenClosed01: Distribution<F>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        loop {
            let u = rng.sample(OpenClosed01);
            let x = u.powf(-F::one() / self.s_minus_1).floor();
            debug_assert!(x >= F::one());
            if x.is_infinite() {
                // For sufficiently small `s`, `x` will always be infinite,
                // which is rejected, resulting in an infinite loop. We avoid
                // this by always returning infinity instead.
                return x;
            }

            let t = (F::one() + F::one() / x).powf(self.s_minus_1);

            let v = rng.sample(StandardUniform);
            if v * x * (t - F::one()) * self.b <= t * (self.b - F::one()) {
                return x;
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
    fn zeta_invalid() {
        Zeta::new(1.).unwrap();
    }

    #[test]
    #[should_panic]
    fn zeta_nan() {
        Zeta::new(f64::NAN).unwrap();
    }

    #[test]
    fn zeta_sample() {
        let a = 2.0;
        let d = Zeta::new(a).unwrap();
        let mut rng = crate::test::rng(1);
        for _ in 0..1000 {
            let r = d.sample(&mut rng);
            assert!(r >= 1.);
        }
    }

    #[test]
    fn zeta_small_a() {
        let a = 1. + 1e-15;
        let d = Zeta::new(a).unwrap();
        let mut rng = crate::test::rng(2);
        for _ in 0..1000 {
            let r = d.sample(&mut rng);
            assert!(r >= 1.);
        }
    }

    #[test]
    fn zeta_value_stability() {
        test_samples(Zeta::new(1.5).unwrap(), 0f32, &[1.0, 2.0, 1.0, 1.0]);
        test_samples(Zeta::new(2.0).unwrap(), 0f64, &[2.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn zeta_distributions_can_be_compared() {
        assert_eq!(Zeta::new(1.0), Zeta::new(1.0));
    }
}
