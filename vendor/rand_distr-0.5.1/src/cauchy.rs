// Copyright 2018 Developers of the Rand project.
// Copyright 2016-2017 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Cauchy distribution `Cauchy(x₀, γ)`.

use crate::{Distribution, StandardUniform};
use core::fmt;
use num_traits::{Float, FloatConst};
use rand::Rng;

/// The [Cauchy distribution](https://en.wikipedia.org/wiki/Cauchy_distribution) `Cauchy(x₀, γ)`.
///
/// The Cauchy distribution is a continuous probability distribution with
/// parameters `x₀` (median) and `γ` (scale).
/// It describes the distribution of the ratio of two independent
/// normally distributed random variables with means `x₀` and scales `γ`.
/// In other words, if `X` and `Y` are independent normally distributed
/// random variables with means `x₀` and scales `γ`, respectively, then
/// `X / Y` is `Cauchy(x₀, γ)` distributed.
///
/// # Density function
///
/// `f(x) = 1 / (π * γ * (1 + ((x - x₀) / γ)²))`
///
/// # Plot
///
/// The plot illustrates the Cauchy distribution with various values of `x₀` and `γ`.
/// Note how the median parameter `x₀` shifts the distribution along the x-axis,
/// and how the scale `γ` changes the density around the median.
///
/// The standard Cauchy distribution is the special case with `x₀ = 0` and `γ = 1`,
/// which corresponds to the ratio of two [`StandardNormal`](crate::StandardNormal) distributions.
///
/// ![Cauchy distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/cauchy.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{Cauchy, Distribution};
///
/// let cau = Cauchy::new(2.0, 5.0).unwrap();
/// let v = cau.sample(&mut rand::rng());
/// println!("{} is from a Cauchy(2, 5) distribution", v);
/// ```
///
/// # Notes
///
/// Note that at least for `f32`, results are not fully portable due to minor
/// differences in the target system's *tan* implementation, `tanf`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cauchy<F>
where
    F: Float + FloatConst,
    StandardUniform: Distribution<F>,
{
    median: F,
    scale: F,
}

/// Error type returned from [`Cauchy::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `scale <= 0` or `nan`.
    ScaleTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::ScaleTooSmall => "scale is not positive in Cauchy distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F> Cauchy<F>
where
    F: Float + FloatConst,
    StandardUniform: Distribution<F>,
{
    /// Construct a new `Cauchy` with the given shape parameters
    /// `median` the peak location and `scale` the scale factor.
    pub fn new(median: F, scale: F) -> Result<Cauchy<F>, Error> {
        if !(scale > F::zero()) {
            return Err(Error::ScaleTooSmall);
        }
        Ok(Cauchy { median, scale })
    }
}

impl<F> Distribution<F> for Cauchy<F>
where
    F: Float + FloatConst,
    StandardUniform: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        // sample from [0, 1)
        let x = StandardUniform.sample(rng);
        // get standard cauchy random number
        // note that π/2 is not exactly representable, even if x=0.5 the result is finite
        let comp_dev = (F::PI() * x).tan();
        // shift and scale according to parameters
        self.median + self.scale * comp_dev
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn median(numbers: &mut [f64]) -> f64 {
        sort(numbers);
        let mid = numbers.len() / 2;
        numbers[mid]
    }

    fn sort(numbers: &mut [f64]) {
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    #[test]
    fn test_cauchy_averages() {
        // NOTE: given that the variance and mean are undefined,
        // this test does not have any rigorous statistical meaning.
        let cauchy = Cauchy::new(10.0, 5.0).unwrap();
        let mut rng = crate::test::rng(123);
        let mut numbers: [f64; 1000] = [0.0; 1000];
        let mut sum = 0.0;
        for number in &mut numbers[..] {
            *number = cauchy.sample(&mut rng);
            sum += *number;
        }
        let median = median(&mut numbers);
        #[cfg(feature = "std")]
        std::println!("Cauchy median: {}", median);
        assert!((median - 10.0).abs() < 0.4); // not 100% certain, but probable enough
        let mean = sum / 1000.0;
        #[cfg(feature = "std")]
        std::println!("Cauchy mean: {}", mean);
        // for a Cauchy distribution the mean should not converge
        assert!((mean - 10.0).abs() > 0.4); // not 100% certain, but probable enough
    }

    #[test]
    #[should_panic]
    fn test_cauchy_invalid_scale_zero() {
        Cauchy::new(0.0, 0.0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_cauchy_invalid_scale_neg() {
        Cauchy::new(0.0, -10.0).unwrap();
    }

    #[test]
    fn value_stability() {
        fn gen_samples<F: Float + FloatConst + fmt::Debug>(m: F, s: F, buf: &mut [F])
        where
            StandardUniform: Distribution<F>,
        {
            let distr = Cauchy::new(m, s).unwrap();
            let mut rng = crate::test::rng(353);
            for x in buf {
                *x = rng.sample(distr);
            }
        }

        let mut buf = [0.0; 4];
        gen_samples(100f64, 10.0, &mut buf);
        assert_eq!(
            &buf,
            &[
                77.93369152808678,
                90.1606912098641,
                125.31516221323625,
                86.10217834773925
            ]
        );

        // Unfortunately this test is not fully portable due to reliance on the
        // system's implementation of tanf (see doc on Cauchy struct).
        let mut buf = [0.0; 4];
        gen_samples(10f32, 7.0, &mut buf);
        let expected = [15.023088, -5.446413, 3.7092876, 3.112482];
        for (a, b) in buf.iter().zip(expected.iter()) {
            assert_almost_eq!(*a, *b, 1e-5);
        }
    }

    #[test]
    fn cauchy_distributions_can_be_compared() {
        assert_eq!(Cauchy::new(1.0, 2.0), Cauchy::new(1.0, 2.0));
    }
}
