// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{uniform::SampleUniform, Distribution, Uniform};
use num_traits::Float;
use rand::Rng;

/// Samples uniformly from the circumference of the unit circle in two dimensions.
///
/// Implemented via a method by von Neumann[^1].
///
/// For a distribution that also samples from the interior of the unit circle,
/// see [`UnitDisc`](crate::UnitDisc).
///
/// For a similar distribution in three dimensions, see [`UnitSphere`](crate::UnitSphere).
///
/// # Plot
///
/// The following plot shows the unit circle.
///
/// ![Unit circle](https://raw.githubusercontent.com/rust-random/charts/main/charts/unit_circle.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{UnitCircle, Distribution};
///
/// let v: [f64; 2] = UnitCircle.sample(&mut rand::rng());
/// println!("{:?} is from the unit circle.", v)
/// ```
///
/// [^1]: von Neumann, J. (1951) [*Various Techniques Used in Connection with
///       Random Digits.*](https://mcnp.lanl.gov/pdf_files/nbs_vonneumann.pdf)
///       NBS Appl. Math. Ser., No. 12. Washington, DC: U.S. Government Printing
///       Office, pp. 36-38.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitCircle;

impl<F: Float + SampleUniform> Distribution<[F; 2]> for UnitCircle {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [F; 2] {
        let uniform = Uniform::new(F::from(-1.).unwrap(), F::from(1.).unwrap()).unwrap();
        let mut x1;
        let mut x2;
        let mut sum;
        loop {
            x1 = uniform.sample(rng);
            x2 = uniform.sample(rng);
            sum = x1 * x1 + x2 * x2;
            if sum < F::from(1.).unwrap() {
                break;
            }
        }
        let diff = x1 * x1 - x2 * x2;
        [diff / sum, F::from(2.).unwrap() * x1 * x2 / sum]
    }
}

#[cfg(test)]
mod tests {
    use super::UnitCircle;
    use crate::Distribution;

    #[test]
    fn norm() {
        let mut rng = crate::test::rng(1);
        for _ in 0..1000 {
            let x: [f64; 2] = UnitCircle.sample(&mut rng);
            assert_almost_eq!(x[0] * x[0] + x[1] * x[1], 1., 1e-15);
        }
    }
}
