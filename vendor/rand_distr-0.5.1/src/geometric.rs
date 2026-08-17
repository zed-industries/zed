//! The geometric distribution `Geometric(p)`.

use crate::Distribution;
use core::fmt;
#[allow(unused_imports)]
use num_traits::Float;
use rand::Rng;

/// The [geometric distribution](https://en.wikipedia.org/wiki/Geometric_distribution) `Geometric(p)`.
///
/// This is the probability distribution of the number of failures
/// (bounded to `[0, u64::MAX]`) before the first success in a
/// series of [`Bernoulli`](crate::Bernoulli) trials, where the
/// probability of success on each trial is `p`.
///
/// This is the discrete analogue of the [exponential distribution](crate::Exp).
///
/// See [`StandardGeometric`](crate::StandardGeometric) for an optimised
/// implementation for `p = 0.5`.
///
/// # Density function
///
/// `f(k) = (1 - p)^k p` for `k >= 0`.
///
/// # Plot
///
/// The following plot illustrates the geometric distribution for various
/// values of `p`. Note how higher `p` values shift the distribution to
/// the left, and the mean of the distribution is `1/p`.
///
/// ![Geometric distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/geometric.svg)
///
/// # Example
/// ```
/// use rand_distr::{Geometric, Distribution};
///
/// let geo = Geometric::new(0.25).unwrap();
/// let v = geo.sample(&mut rand::rng());
/// println!("{} is from a Geometric(0.25) distribution", v);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Geometric {
    p: f64,
    pi: f64,
    k: u64,
}

/// Error type returned from [`Geometric::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `p < 0 || p > 1` or `nan`
    InvalidProbability,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::InvalidProbability => {
                "p is NaN or outside the interval [0, 1] in geometric distribution"
            }
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl Geometric {
    /// Construct a new `Geometric` with the given shape parameter `p`
    /// (probability of success on each trial).
    pub fn new(p: f64) -> Result<Self, Error> {
        if !p.is_finite() || !(0.0..=1.0).contains(&p) {
            Err(Error::InvalidProbability)
        } else if p == 0.0 || p >= 2.0 / 3.0 {
            Ok(Geometric { p, pi: p, k: 0 })
        } else {
            let (pi, k) = {
                // choose smallest k such that pi = (1 - p)^(2^k) <= 0.5
                let mut k = 1;
                let mut pi = (1.0 - p).powi(2);
                while pi > 0.5 {
                    k += 1;
                    pi = pi * pi;
                }
                (pi, k)
            };

            Ok(Geometric { p, pi, k })
        }
    }
}

impl Distribution<u64> for Geometric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        if self.p >= 2.0 / 3.0 {
            // use the trivial algorithm:
            let mut failures = 0;
            loop {
                let u = rng.random::<f64>();
                if u <= self.p {
                    break;
                }
                failures += 1;
            }
            return failures;
        }

        if self.p == 0.0 {
            return u64::MAX;
        }

        let Geometric { p, pi, k } = *self;

        // Based on the algorithm presented in section 3 of
        // Karl Bringmann and Tobias Friedrich (July 2013) - Exact and Efficient
        // Generation of Geometric Random Variates and Random Graphs, published
        // in International Colloquium on Automata, Languages and Programming
        // (pp.267-278)
        // https://people.mpi-inf.mpg.de/~kbringma/paper/2013ICALP-1.pdf

        // Use the trivial algorithm to sample D from Geo(pi) = Geo(p) / 2^k:
        let d = {
            let mut failures = 0;
            while rng.random::<f64>() < pi {
                failures += 1;
            }
            failures
        };

        // Use rejection sampling for the remainder M from Geo(p) % 2^k:
        // choose M uniformly from [0, 2^k), but reject with probability (1 - p)^M
        // NOTE: The paper suggests using bitwise sampling here, which is
        // currently unsupported, but should improve performance by requiring
        // fewer iterations on average.                 ~ October 28, 2020
        let m = loop {
            let m = rng.random::<u64>() & ((1 << k) - 1);
            let p_reject = if m <= i32::MAX as u64 {
                (1.0 - p).powi(m as i32)
            } else {
                (1.0 - p).powf(m as f64)
            };

            let u = rng.random::<f64>();
            if u < p_reject {
                break m;
            }
        };

        (d << k) + m
    }
}

/// The standard geometric distribution `Geometric(0.5)`.
///
/// This is equivalent to `Geometric::new(0.5)`, but faster.
///
/// See [`Geometric`](crate::Geometric) for the general geometric distribution.
///
/// # Plot
///
/// The following plot illustrates the standard geometric distribution.
///
/// ![Standard Geometric distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/standard_geometric.svg)
///
/// # Example
/// ```
/// use rand::prelude::*;
/// use rand_distr::StandardGeometric;
///
/// let v = StandardGeometric.sample(&mut rand::rng());
/// println!("{} is from a Geometric(0.5) distribution", v);
/// ```
///
/// # Notes
/// Implemented via iterated
/// [`Rng::gen::<u64>().leading_zeros()`](Rng::gen::<u64>().leading_zeros()).
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StandardGeometric;

impl Distribution<u64> for StandardGeometric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        let mut result = 0;
        loop {
            let x = rng.random::<u64>().leading_zeros() as u64;
            result += x;
            if x < 64 {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_geo_invalid_p() {
        assert!(Geometric::new(f64::NAN).is_err());
        assert!(Geometric::new(f64::INFINITY).is_err());
        assert!(Geometric::new(f64::NEG_INFINITY).is_err());

        assert!(Geometric::new(-0.5).is_err());
        assert!(Geometric::new(0.0).is_ok());
        assert!(Geometric::new(1.0).is_ok());
        assert!(Geometric::new(2.0).is_err());
    }

    fn test_geo_mean_and_variance<R: Rng>(p: f64, rng: &mut R) {
        let distr = Geometric::new(p).unwrap();

        let expected_mean = (1.0 - p) / p;
        let expected_variance = (1.0 - p) / (p * p);

        let mut results = [0.0; 10000];
        for i in results.iter_mut() {
            *i = distr.sample(rng) as f64;
        }

        let mean = results.iter().sum::<f64>() / results.len() as f64;
        assert!((mean - expected_mean).abs() < expected_mean / 40.0);

        let variance =
            results.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / results.len() as f64;
        assert!((variance - expected_variance).abs() < expected_variance / 10.0);
    }

    #[test]
    fn test_geometric() {
        let mut rng = crate::test::rng(12345);

        test_geo_mean_and_variance(0.10, &mut rng);
        test_geo_mean_and_variance(0.25, &mut rng);
        test_geo_mean_and_variance(0.50, &mut rng);
        test_geo_mean_and_variance(0.75, &mut rng);
        test_geo_mean_and_variance(0.90, &mut rng);
    }

    #[test]
    fn test_standard_geometric() {
        let mut rng = crate::test::rng(654321);

        let distr = StandardGeometric;
        let expected_mean = 1.0;
        let expected_variance = 2.0;

        let mut results = [0.0; 1000];
        for i in results.iter_mut() {
            *i = distr.sample(&mut rng) as f64;
        }

        let mean = results.iter().sum::<f64>() / results.len() as f64;
        assert!((mean - expected_mean).abs() < expected_mean / 50.0);

        let variance =
            results.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / results.len() as f64;
        assert!((variance - expected_variance).abs() < expected_variance / 10.0);
    }

    #[test]
    fn geometric_distributions_can_be_compared() {
        assert_eq!(Geometric::new(1.0), Geometric::new(1.0));
    }
}
