// Copyright 2018-2019 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{uniform::SampleUniform, Distribution, Uniform};
use num_traits::Float;
use rand::Rng;

/// Samples uniformly from the surface of the unit sphere in three dimensions.
///
/// Implemented via a method by Marsaglia[^1].
///
/// For a distribution that also samples from the interior of the sphere,
/// see [`UnitBall`](crate::UnitBall).
///
/// For a similar distribution in two dimensions, see [`UnitCircle`](crate::UnitCircle).
///
/// # Plot
///
/// The following plot shows the unit sphere as a wireframe.
/// The wireframe is meant to illustrate that this distribution samples
/// from the surface of the sphere only, not from the interior.
///
/// ![Unit sphere](https://raw.githubusercontent.com/rust-random/charts/main/charts/unit_sphere.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{UnitSphere, Distribution};
///
/// let v: [f64; 3] = UnitSphere.sample(&mut rand::rng());
/// println!("{:?} is from the unit sphere surface.", v)
/// ```
///
/// [^1]: Marsaglia, George (1972). [*Choosing a Point from the Surface of a
///       Sphere.*](https://doi.org/10.1214/aoms/1177692644)
///       Ann. Math. Statist. 43, no. 2, 645--646.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitSphere;

impl<F: Float + SampleUniform> Distribution<[F; 3]> for UnitSphere {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [F; 3] {
        let uniform = Uniform::new(F::from(-1.).unwrap(), F::from(1.).unwrap()).unwrap();
        loop {
            let (x1, x2) = (uniform.sample(rng), uniform.sample(rng));
            let sum = x1 * x1 + x2 * x2;
            if sum >= F::from(1.).unwrap() {
                continue;
            }
            let factor = F::from(2.).unwrap() * (F::one() - sum).sqrt();
            return [
                x1 * factor,
                x2 * factor,
                F::from(1.).unwrap() - F::from(2.).unwrap() * sum,
            ];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UnitSphere;
    use crate::Distribution;

    #[test]
    fn norm() {
        let mut rng = crate::test::rng(1);
        for _ in 0..1000 {
            let x: [f64; 3] = UnitSphere.sample(&mut rng);
            assert_almost_eq!(x[0] * x[0] + x[1] * x[1] + x[2] * x[2], 1., 1e-15);
        }
    }
}
