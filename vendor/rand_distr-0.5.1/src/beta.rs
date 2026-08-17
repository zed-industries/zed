// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Beta distribution.

use crate::{Distribution, Open01};
use core::fmt;
use num_traits::Float;
use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The algorithm used for sampling the Beta distribution.
///
/// Reference:
///
/// R. C. H. Cheng (1978).
/// Generating beta variates with nonintegral shape parameters.
/// Communications of the ACM 21, 317-322.
/// https://doi.org/10.1145/359460.359482
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum BetaAlgorithm<N> {
    BB(BB<N>),
    BC(BC<N>),
}

/// Algorithm BB for `min(alpha, beta) > 1`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct BB<N> {
    alpha: N,
    beta: N,
    gamma: N,
}

/// Algorithm BC for `min(alpha, beta) <= 1`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct BC<N> {
    alpha: N,
    beta: N,
    kappa1: N,
    kappa2: N,
}

/// The [Beta distribution](https://en.wikipedia.org/wiki/Beta_distribution) `Beta(α, β)`.
///
/// The Beta distribution is a continuous probability distribution
/// defined on the interval `[0, 1]`. It is the conjugate prior for the
/// parameter `p` of the [`Binomial`][crate::Binomial] distribution.
///
/// It has two shape parameters `α` (alpha) and `β` (beta) which control
/// the shape of the distribution. Both `a` and `β` must be greater than zero.
/// The distribution is symmetric when `α = β`.
///
/// # Plot
///
/// The plot shows the Beta distribution with various combinations
/// of `α` and `β`.
///
/// ![Beta distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/beta.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{Distribution, Beta};
///
/// let beta = Beta::new(2.0, 5.0).unwrap();
/// let v = beta.sample(&mut rand::rng());
/// println!("{} is from a Beta(2, 5) distribution", v);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Beta<F>
where
    F: Float,
    Open01: Distribution<F>,
{
    a: F,
    b: F,
    switched_params: bool,
    algorithm: BetaAlgorithm<F>,
}

/// Error type returned from [`Beta::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Error {
    /// `alpha <= 0` or `nan`.
    AlphaTooSmall,
    /// `beta <= 0` or `nan`.
    BetaTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::AlphaTooSmall => "alpha is not positive in beta distribution",
            Error::BetaTooSmall => "beta is not positive in beta distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F> Beta<F>
where
    F: Float,
    Open01: Distribution<F>,
{
    /// Construct an object representing the `Beta(alpha, beta)`
    /// distribution.
    pub fn new(alpha: F, beta: F) -> Result<Beta<F>, Error> {
        if !(alpha > F::zero()) {
            return Err(Error::AlphaTooSmall);
        }
        if !(beta > F::zero()) {
            return Err(Error::BetaTooSmall);
        }
        // From now on, we use the notation from the reference,
        // i.e. `alpha` and `beta` are renamed to `a0` and `b0`.
        let (a0, b0) = (alpha, beta);
        let (a, b, switched_params) = if a0 < b0 {
            (a0, b0, false)
        } else {
            (b0, a0, true)
        };
        if a > F::one() {
            // Algorithm BB
            let alpha = a + b;

            let two = F::from(2.).unwrap();
            let beta_numer = alpha - two;
            let beta_denom = two * a * b - alpha;
            let beta = (beta_numer / beta_denom).sqrt();

            let gamma = a + F::one() / beta;

            Ok(Beta {
                a,
                b,
                switched_params,
                algorithm: BetaAlgorithm::BB(BB { alpha, beta, gamma }),
            })
        } else {
            // Algorithm BC
            //
            // Here `a` is the maximum instead of the minimum.
            let (a, b, switched_params) = (b, a, !switched_params);
            let alpha = a + b;
            let beta = F::one() / b;
            let delta = F::one() + a - b;
            let kappa1 = delta
                * (F::from(1. / 18. / 4.).unwrap() + F::from(3. / 18. / 4.).unwrap() * b)
                / (a * beta - F::from(14. / 18.).unwrap());
            let kappa2 = F::from(0.25).unwrap()
                + (F::from(0.5).unwrap() + F::from(0.25).unwrap() / delta) * b;

            Ok(Beta {
                a,
                b,
                switched_params,
                algorithm: BetaAlgorithm::BC(BC {
                    alpha,
                    beta,
                    kappa1,
                    kappa2,
                }),
            })
        }
    }
}

impl<F> Distribution<F> for Beta<F>
where
    F: Float,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let mut w;
        match self.algorithm {
            BetaAlgorithm::BB(algo) => {
                loop {
                    // 1.
                    let u1 = rng.sample(Open01);
                    let u2 = rng.sample(Open01);
                    let v = algo.beta * (u1 / (F::one() - u1)).ln();
                    w = self.a * v.exp();
                    let z = u1 * u1 * u2;
                    let r = algo.gamma * v - F::from(4.).unwrap().ln();
                    let s = self.a + r - w;
                    // 2.
                    if s + F::one() + F::from(5.).unwrap().ln() >= F::from(5.).unwrap() * z {
                        break;
                    }
                    // 3.
                    let t = z.ln();
                    if s >= t {
                        break;
                    }
                    // 4.
                    if !(r + algo.alpha * (algo.alpha / (self.b + w)).ln() < t) {
                        break;
                    }
                }
            }
            BetaAlgorithm::BC(algo) => {
                loop {
                    let z;
                    // 1.
                    let u1 = rng.sample(Open01);
                    let u2 = rng.sample(Open01);
                    if u1 < F::from(0.5).unwrap() {
                        // 2.
                        let y = u1 * u2;
                        z = u1 * y;
                        if F::from(0.25).unwrap() * u2 + z - y >= algo.kappa1 {
                            continue;
                        }
                    } else {
                        // 3.
                        z = u1 * u1 * u2;
                        if z <= F::from(0.25).unwrap() {
                            let v = algo.beta * (u1 / (F::one() - u1)).ln();
                            w = self.a * v.exp();
                            break;
                        }
                        // 4.
                        if z >= algo.kappa2 {
                            continue;
                        }
                    }
                    // 5.
                    let v = algo.beta * (u1 / (F::one() - u1)).ln();
                    w = self.a * v.exp();
                    if !(algo.alpha * ((algo.alpha / (self.b + w)).ln() + v)
                        - F::from(4.).unwrap().ln()
                        < z.ln())
                    {
                        break;
                    };
                }
            }
        };
        // 5. for BB, 6. for BC
        if !self.switched_params {
            if w == F::infinity() {
                // Assuming `b` is finite, for large `w`:
                return F::one();
            }
            w / (self.b + w)
        } else {
            self.b / (self.b + w)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_beta() {
        let beta = Beta::new(1.0, 2.0).unwrap();
        let mut rng = crate::test::rng(201);
        for _ in 0..1000 {
            beta.sample(&mut rng);
        }
    }

    #[test]
    #[should_panic]
    fn test_beta_invalid_dof() {
        Beta::new(0., 0.).unwrap();
    }

    #[test]
    fn test_beta_small_param() {
        let beta = Beta::<f64>::new(1e-3, 1e-3).unwrap();
        let mut rng = crate::test::rng(206);
        for i in 0..1000 {
            assert!(!beta.sample(&mut rng).is_nan(), "failed at i={}", i);
        }
    }

    #[test]
    fn beta_distributions_can_be_compared() {
        assert_eq!(Beta::new(1.0, 2.0), Beta::new(1.0, 2.0));
    }
}
