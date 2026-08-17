// Copyright 2021 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Zipf distribution.

use crate::{Distribution, StandardUniform};
use core::fmt;
use num_traits::Float;
use rand::Rng;

/// The Zipf (Zipfian) distribution `Zipf(n, s)`.
///
/// The samples follow [Zipf's law](https://en.wikipedia.org/wiki/Zipf%27s_law):
/// The frequency of each sample from a finite set of size `n` is inversely
/// proportional to a power of its frequency rank (with exponent `s`).
///
/// For large `n`, this converges to the [`Zeta`](crate::Zeta) distribution.
///
/// For `s = 0`, this becomes a [`uniform`](crate::Uniform) distribution.
///
/// # Plot
///
/// The following plot illustrates the Zipf distribution for `n = 10` and
/// various values of `s`.
///
/// ![Zipf distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/zipf.svg)
///
/// # Example
/// ```
/// use rand::prelude::*;
/// use rand_distr::Zipf;
///
/// let val: f64 = rand::rng().sample(Zipf::new(10.0, 1.5).unwrap());
/// println!("{}", val);
/// ```
///
/// # Integer vs FP return type
///
/// This implementation uses floating-point (FP) logic internally. It may be
/// expected that the samples are no greater than `n`, thus it is reasonable to
/// cast generated samples to any integer type which can also represent `n`
/// (e.g. `distr.sample(&mut rng) as u64`).
///
/// # Implementation details
///
/// Implemented via [rejection sampling](https://en.wikipedia.org/wiki/Rejection_sampling),
/// due to Jason Crease[1].
///
/// [1]: https://jasoncrease.medium.com/rejection-sampling-the-zipf-distribution-6b359792cffa
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zipf<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
{
    s: F,
    t: F,
    q: F,
}

/// Error type returned from [`Zipf::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `s < 0` or `nan`.
    STooSmall,
    /// `n < 1`.
    NTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::STooSmall => "s < 0 or is NaN in Zipf distribution",
            Error::NTooSmall => "n < 1 in Zipf distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F> Zipf<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
{
    /// Construct a new `Zipf` distribution for a set with `n` elements and a
    /// frequency rank exponent `s`.
    ///
    /// The parameter `n` is typically integral, however we use type
    /// <pre><code>F: [Float]</code></pre> in order to permit very large values
    /// and since our implementation requires a floating-point type.
    #[inline]
    pub fn new(n: F, s: F) -> Result<Zipf<F>, Error> {
        if !(s >= F::zero()) {
            return Err(Error::STooSmall);
        }
        if n < F::one() {
            return Err(Error::NTooSmall);
        }
        let q = if s != F::one() {
            // Make sure to calculate the division only once.
            F::one() / (F::one() - s)
        } else {
            // This value is never used.
            F::zero()
        };
        let t = if s != F::one() {
            (n.powf(F::one() - s) - s) * q
        } else {
            F::one() + n.ln()
        };
        debug_assert!(t > F::zero());
        Ok(Zipf { s, t, q })
    }

    /// Inverse cumulative density function
    #[inline]
    fn inv_cdf(&self, p: F) -> F {
        let one = F::one();
        let pt = p * self.t;
        if pt <= one {
            pt
        } else if self.s != one {
            (pt * (one - self.s) + self.s).powf(self.q)
        } else {
            (pt - one).exp()
        }
    }
}

impl<F> Distribution<F> for Zipf<F>
where
    F: Float,
    StandardUniform: Distribution<F>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let one = F::one();
        loop {
            let inv_b = self.inv_cdf(rng.sample(StandardUniform));
            let x = (inv_b + one).floor();
            let mut ratio = x.powf(-self.s);
            if x > one {
                ratio = ratio * inv_b.powf(self.s)
            };

            let y = rng.sample(StandardUniform);
            if y < ratio {
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
    fn zipf_s_too_small() {
        Zipf::new(10., -1.).unwrap();
    }

    #[test]
    #[should_panic]
    fn zipf_n_too_small() {
        Zipf::new(0., 1.).unwrap();
    }

    #[test]
    #[should_panic]
    fn zipf_nan() {
        Zipf::new(10., f64::NAN).unwrap();
    }

    #[test]
    fn zipf_sample() {
        let d = Zipf::new(10., 0.5).unwrap();
        let mut rng = crate::test::rng(2);
        for _ in 0..1000 {
            let r = d.sample(&mut rng);
            assert!(r >= 1.);
        }
    }

    #[test]
    fn zipf_sample_s_1() {
        let d = Zipf::new(10., 1.).unwrap();
        let mut rng = crate::test::rng(2);
        for _ in 0..1000 {
            let r = d.sample(&mut rng);
            assert!(r >= 1.);
        }
    }

    #[test]
    fn zipf_sample_s_0() {
        let d = Zipf::new(10., 0.).unwrap();
        let mut rng = crate::test::rng(2);
        for _ in 0..1000 {
            let r = d.sample(&mut rng);
            assert!(r >= 1.);
        }
        // TODO: verify that this is a uniform distribution
    }

    #[test]
    fn zipf_sample_large_n() {
        let d = Zipf::new(f64::MAX, 1.5).unwrap();
        let mut rng = crate::test::rng(2);
        for _ in 0..1000 {
            let r = d.sample(&mut rng);
            assert!(r >= 1.);
        }
        // TODO: verify that this is a zeta distribution
    }

    #[test]
    fn zipf_value_stability() {
        test_samples(Zipf::new(10., 0.5).unwrap(), 0f32, &[10.0, 2.0, 6.0, 7.0]);
        test_samples(Zipf::new(10., 2.0).unwrap(), 0f64, &[1.0, 2.0, 3.0, 2.0]);
    }

    #[test]
    fn zipf_distributions_can_be_compared() {
        assert_eq!(Zipf::new(1.0, 2.0), Zipf::new(1.0, 2.0));
    }
}
