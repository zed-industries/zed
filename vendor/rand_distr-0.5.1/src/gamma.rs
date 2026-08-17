// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Gamma distribution.

use self::GammaRepr::*;

use crate::{Distribution, Exp, Exp1, Open01, StandardNormal};
use core::fmt;
use num_traits::Float;
use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The [Gamma distribution](https://en.wikipedia.org/wiki/Gamma_distribution) `Gamma(k, θ)`.
///
/// The Gamma distribution is a continuous probability distribution
/// with shape parameter `k > 0` (number of events) and
/// scale parameter `θ > 0` (mean waiting time between events).
/// It describes the time until `k` events occur in a Poisson
/// process with rate `1/θ`. It is the generalization of the
/// [`Exponential`](crate::Exp) distribution.
///
/// # Density function
///
/// `f(x) =  x^(k - 1) * exp(-x / θ) / (Γ(k) * θ^k)` for `x > 0`,
/// where `Γ` is the [gamma function](https://en.wikipedia.org/wiki/Gamma_function).
///
/// # Plot
///
/// The following plot illustrates the Gamma distribution with
/// various values of `k` and `θ`.
/// Curves with `θ = 1` are more saturated, while corresponding
/// curves with `θ = 2` have a lighter color.
///
/// ![Gamma distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/gamma.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{Distribution, Gamma};
///
/// let gamma = Gamma::new(2.0, 5.0).unwrap();
/// let v = gamma.sample(&mut rand::rng());
/// println!("{} is from a Gamma(2, 5) distribution", v);
/// ```
///
/// # Notes
///
/// The algorithm used is that described by Marsaglia & Tsang 2000[^1],
/// falling back to directly sampling from an Exponential for `shape
/// == 1`, and using the boosting technique described in that paper for
/// `shape < 1`.
///
/// [^1]: George Marsaglia and Wai Wan Tsang. 2000. "A Simple Method for
///       Generating Gamma Variables" *ACM Trans. Math. Softw.* 26, 3
///       (September 2000), 363-372.
///       DOI:[10.1145/358407.358414](https://doi.acm.org/10.1145/358407.358414)
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Gamma<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    repr: GammaRepr<F>,
}

/// Error type returned from [`Gamma::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `shape <= 0` or `nan`.
    ShapeTooSmall,
    /// `scale <= 0` or `nan`.
    ScaleTooSmall,
    /// `1 / scale == 0`.
    ScaleTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::ShapeTooSmall => "shape is not positive in gamma distribution",
            Error::ScaleTooSmall => "scale is not positive in gamma distribution",
            Error::ScaleTooLarge => "scale is infinity in gamma distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum GammaRepr<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    Large(GammaLargeShape<F>),
    One(Exp<F>),
    Small(GammaSmallShape<F>),
}

// These two helpers could be made public, but saving the
// match-on-Gamma-enum branch from using them directly (e.g. if one
// knows that the shape is always > 1) doesn't appear to be much
// faster.

/// Gamma distribution where the shape parameter is less than 1.
///
/// Note, samples from this require a compulsory floating-point `pow`
/// call, which makes it significantly slower than sampling from a
/// gamma distribution where the shape parameter is greater than or
/// equal to 1.
///
/// See `Gamma` for sampling from a Gamma distribution with general
/// shape parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct GammaSmallShape<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Open01: Distribution<F>,
{
    inv_shape: F,
    large_shape: GammaLargeShape<F>,
}

/// Gamma distribution where the shape parameter is larger than 1.
///
/// See `Gamma` for sampling from a Gamma distribution with general
/// shape parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct GammaLargeShape<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Open01: Distribution<F>,
{
    scale: F,
    c: F,
    d: F,
}

impl<F> Gamma<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    /// Construct an object representing the `Gamma(shape, scale)`
    /// distribution.
    #[inline]
    pub fn new(shape: F, scale: F) -> Result<Gamma<F>, Error> {
        if !(shape > F::zero()) {
            return Err(Error::ShapeTooSmall);
        }
        if !(scale > F::zero()) {
            return Err(Error::ScaleTooSmall);
        }

        let repr = if shape == F::one() {
            One(Exp::new(F::one() / scale).map_err(|_| Error::ScaleTooLarge)?)
        } else if shape < F::one() {
            Small(GammaSmallShape::new_raw(shape, scale))
        } else {
            Large(GammaLargeShape::new_raw(shape, scale))
        };
        Ok(Gamma { repr })
    }
}

impl<F> GammaSmallShape<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Open01: Distribution<F>,
{
    fn new_raw(shape: F, scale: F) -> GammaSmallShape<F> {
        GammaSmallShape {
            inv_shape: F::one() / shape,
            large_shape: GammaLargeShape::new_raw(shape + F::one(), scale),
        }
    }
}

impl<F> GammaLargeShape<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Open01: Distribution<F>,
{
    fn new_raw(shape: F, scale: F) -> GammaLargeShape<F> {
        let d = shape - F::from(1. / 3.).unwrap();
        GammaLargeShape {
            scale,
            c: F::one() / (F::from(9.).unwrap() * d).sqrt(),
            d,
        }
    }
}

impl<F> Distribution<F> for Gamma<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Exp1: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        match self.repr {
            Small(ref g) => g.sample(rng),
            One(ref g) => g.sample(rng),
            Large(ref g) => g.sample(rng),
        }
    }
}
impl<F> Distribution<F> for GammaSmallShape<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let u: F = rng.sample(Open01);

        self.large_shape.sample(rng) * u.powf(self.inv_shape)
    }
}
impl<F> Distribution<F> for GammaLargeShape<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    Open01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        // Marsaglia & Tsang method, 2000
        loop {
            let x: F = rng.sample(StandardNormal);
            let v_cbrt = F::one() + self.c * x;
            if v_cbrt <= F::zero() {
                // a^3 <= 0 iff a <= 0
                continue;
            }

            let v = v_cbrt * v_cbrt * v_cbrt;
            let u: F = rng.sample(Open01);

            let x_sqr = x * x;
            if u < F::one() - F::from(0.0331).unwrap() * x_sqr * x_sqr
                || u.ln() < F::from(0.5).unwrap() * x_sqr + self.d * (F::one() - v + v.ln())
            {
                return self.d * v * self.scale;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gamma_distributions_can_be_compared() {
        assert_eq!(Gamma::new(1.0, 2.0), Gamma::new(1.0, 2.0));
    }
}
