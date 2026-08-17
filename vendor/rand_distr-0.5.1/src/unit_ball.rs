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

/// Samples uniformly from the volume of the unit ball in three dimensions.
///
/// Implemented via rejection sampling.
///
/// For a distribution that samples only from the surface of the unit ball,
/// see [`UnitSphere`](crate::UnitSphere).
///
/// For a similar distribution in two dimensions, see [`UnitDisc`](crate::UnitDisc).
///
/// # Plot
///
/// The following plot shows the unit ball in three dimensions.
/// This distribution samples individual points from the entire volume
/// of the ball.
///
/// ![Unit ball](https://raw.githubusercontent.com/rust-random/charts/main/charts/unit_ball.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{UnitBall, Distribution};
///
/// let v: [f64; 3] = UnitBall.sample(&mut rand::rng());
/// println!("{:?} is from the unit ball.", v)
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitBall;

impl<F: Float + SampleUniform> Distribution<[F; 3]> for UnitBall {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [F; 3] {
        let uniform = Uniform::new(F::from(-1.).unwrap(), F::from(1.).unwrap()).unwrap();
        let mut x1;
        let mut x2;
        let mut x3;
        loop {
            x1 = uniform.sample(rng);
            x2 = uniform.sample(rng);
            x3 = uniform.sample(rng);
            if x1 * x1 + x2 * x2 + x3 * x3 <= F::from(1.).unwrap() {
                break;
            }
        }
        [x1, x2, x3]
    }
}
