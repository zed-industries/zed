//! Traits for mapping an isogeny to another curve
//!
//! <https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve>

use core::ops::{AddAssign, Mul};
use ff::Field;
use generic_array::{typenum::Unsigned, ArrayLength, GenericArray};

/// The coefficients for mapping from one isogenous curve to another
pub struct IsogenyCoefficients<F: Field + AddAssign + Mul<Output = F>> {
    /// The coefficients for the x numerator
    pub xnum: &'static [F],
    /// The coefficients for the x denominator
    pub xden: &'static [F],
    /// The coefficients for the y numerator
    pub ynum: &'static [F],
    /// The coefficients for the x denominator
    pub yden: &'static [F],
}

/// The [`Isogeny`] methods to map to another curve.
pub trait Isogeny: Field + AddAssign + Mul<Output = Self> {
    /// The maximum number of coefficients
    type Degree: ArrayLength<Self>;
    /// The isogeny coefficients
    const COEFFICIENTS: IsogenyCoefficients<Self>;

    /// Map from the isogeny points to the main curve
    fn isogeny(x: Self, y: Self) -> (Self, Self) {
        let mut xs = GenericArray::<Self, Self::Degree>::default();
        xs[0] = Self::one();
        xs[1] = x;
        xs[2] = x.square();
        for i in 3..Self::Degree::to_usize() {
            xs[i] = xs[i - 1] * x;
        }
        let x_num = Self::compute_iso(&xs, Self::COEFFICIENTS.xnum);
        let x_den = Self::compute_iso(&xs, Self::COEFFICIENTS.xden)
            .invert()
            .unwrap();
        let y_num = Self::compute_iso(&xs, Self::COEFFICIENTS.ynum) * y;
        let y_den = Self::compute_iso(&xs, Self::COEFFICIENTS.yden)
            .invert()
            .unwrap();

        (x_num * x_den, y_num * y_den)
    }

    /// Compute the ISO transform
    fn compute_iso(xxs: &[Self], k: &[Self]) -> Self {
        let mut xx = Self::zero();
        for (xi, ki) in xxs.iter().zip(k.iter()) {
            xx += *xi * ki;
        }
        xx
    }
}
