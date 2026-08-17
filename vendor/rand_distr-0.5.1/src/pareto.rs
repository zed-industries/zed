// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Pareto distribution `Pareto(xₘ, α)`.

use crate::{Distribution, OpenClosed01};
use core::fmt;
use num_traits::Float;
use rand::Rng;

/// The [Pareto distribution](https://en.wikipedia.org/wiki/Pareto_distribution) `Pareto(xₘ, α)`.
///
/// The Pareto distribution is a continuous probability distribution with
/// scale parameter `xₘ` ( or `k`) and shape parameter `α`.
///
/// # Plot
///
/// The following plot shows the Pareto distribution with various values of
/// `xₘ` and `α`.
/// Note how the shape parameter `α` corresponds to the height of the jump
/// in density at `x = xₘ`, and to the rate of decay in the tail.
///
/// ![Pareto distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/pareto.svg)
///
/// # Example
/// ```
/// use rand::prelude::*;
/// use rand_distr::Pareto;
///
/// let val: f64 = rand::rng().sample(Pareto::new(1., 2.).unwrap());
/// println!("{}", val);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pareto<F>
where
    F: Float,
    OpenClosed01: Distribution<F>,
{
    scale: F,
    inv_neg_shape: F,
}

/// Error type returned from [`Pareto::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `scale <= 0` or `nan`.
    ScaleTooSmall,
    /// `shape <= 0` or `nan`.
    ShapeTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::ScaleTooSmall => "scale is not positive in Pareto distribution",
            Error::ShapeTooSmall => "shape is not positive in Pareto distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl<F> Pareto<F>
where
    F: Float,
    OpenClosed01: Distribution<F>,
{
    /// Construct a new Pareto distribution with given `scale` and `shape`.
    ///
    /// In the literature, `scale` is commonly written as x<sub>m</sub> or k and
    /// `shape` is often written as α.
    pub fn new(scale: F, shape: F) -> Result<Pareto<F>, Error> {
        let zero = F::zero();

        if !(scale > zero) {
            return Err(Error::ScaleTooSmall);
        }
        if !(shape > zero) {
            return Err(Error::ShapeTooSmall);
        }
        Ok(Pareto {
            scale,
            inv_neg_shape: F::from(-1.0).unwrap() / shape,
        })
    }
}

impl<F> Distribution<F> for Pareto<F>
where
    F: Float,
    OpenClosed01: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        let u: F = OpenClosed01.sample(rng);
        self.scale * u.powf(self.inv_neg_shape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::{Debug, Display, LowerExp};

    #[test]
    #[should_panic]
    fn invalid() {
        Pareto::new(0., 0.).unwrap();
    }

    #[test]
    fn sample() {
        let scale = 1.0;
        let shape = 2.0;
        let d = Pareto::new(scale, shape).unwrap();
        let mut rng = crate::test::rng(1);
        for _ in 0..1000 {
            let r = d.sample(&mut rng);
            assert!(r >= scale);
        }
    }

    #[test]
    fn value_stability() {
        fn test_samples<F: Float + Debug + Display + LowerExp, D: Distribution<F>>(
            distr: D,
            thresh: F,
            expected: &[F],
        ) {
            let mut rng = crate::test::rng(213);
            for v in expected {
                let x = rng.sample(&distr);
                assert_almost_eq!(x, *v, thresh);
            }
        }

        test_samples(
            Pareto::new(1f32, 1.0).unwrap(),
            1e-6,
            &[1.0423688, 2.1235929, 4.132709, 1.4679428],
        );
        test_samples(
            Pareto::new(2.0, 0.5).unwrap(),
            1e-14,
            &[
                9.019295276219136,
                4.3097126018270595,
                6.837815045397157,
                105.8826669383772,
            ],
        );
    }

    #[test]
    fn pareto_distributions_can_be_compared() {
        assert_eq!(Pareto::new(1.0, 2.0), Pareto::new(1.0, 2.0));
    }
}
