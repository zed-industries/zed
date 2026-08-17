// Copyright 2019 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{uniform::SampleUniform, Distribution, Uniform};
use num_traits::Float;
use rand::Rng;

/// Samples uniformly from the unit disc in two dimensions.
///
/// Implemented via rejection sampling.
///
/// For a distribution that samples only from the circumference of the unit disc,
/// see [`UnitCircle`](crate::UnitCircle).
///
/// For a similar distribution in three dimensions, see [`UnitBall`](crate::UnitBall).
///
/// # Plot
///
/// The following plot shows the unit disc.
/// This distribution samples individual points from the entire area of the disc.
///
/// ![Unit disc](https://raw.githubusercontent.com/rust-random/charts/main/charts/unit_disc.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{UnitDisc, Distribution};
///
/// let v: [f64; 2] = UnitDisc.sample(&mut rand::rng());
/// println!("{:?} is from the unit Disc.", v)
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitDisc;

impl<F: Float + SampleUniform> Distribution<[F; 2]> for UnitDisc {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [F; 2] {
        let uniform = Uniform::new(F::from(-1.).unwrap(), F::from(1.).unwrap()).unwrap();
        let mut x1;
        let mut x2;
        loop {
            x1 = uniform.sample(rng);
            x2 = uniform.sample(rng);
            if x1 * x1 + x2 * x2 <= F::from(1.).unwrap() {
                break;
            }
        }
        [x1, x2]
    }
}
