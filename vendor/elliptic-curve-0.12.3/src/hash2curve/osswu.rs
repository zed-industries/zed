//! Optimized simplified Shallue-van de Woestijne-Ulas methods.
//!
//! <https://eprint.iacr.org/2009/340.pdf>

use ff::Field;
use subtle::Choice;

/// The Optimized Simplified Shallue-van de Woestijne-Ulas parameters
pub struct OsswuMapParams<F>
where
    F: Field,
{
    /// The first constant term
    pub c1: [u64; 4],
    /// The second constant term
    pub c2: F,
    /// The ISO A variable or Curve A variable
    pub map_a: F,
    /// The ISO A variable or Curve A variable
    pub map_b: F,
    /// The Z parameter
    pub z: F,
}

/// Trait for determining the parity of the field
pub trait Sgn0 {
    /// Return the parity of the field
    /// 1 == negative
    /// 0 == non-negative
    fn sgn0(&self) -> Choice;
}

/// The optimized simplified Shallue-van de Woestijne-Ulas method
/// for mapping elliptic curve scalars to affine points.
pub trait OsswuMap: Field + Sgn0 {
    /// The OSSWU parameters for mapping the field to affine points.
    /// For Weierstrass curves having A==0 or B==0, the parameters
    /// should be for isogeny where A≠0 and B≠0.
    const PARAMS: OsswuMapParams<Self>;

    /// Convert this field element into an affine point on the elliptic curve
    /// returning (X, Y). For Weierstrass curves having A==0 or B==0
    /// the result is a point on an isogeny.
    fn osswu(&self) -> (Self, Self) {
        let tv1 = self.square(); // u^2
        let tv3 = Self::PARAMS.z * tv1; // Z * u^2
        let mut tv2 = tv3.square(); // tv3^2
        let mut xd = tv2 + tv3; // tv3^2 + tv3
        let x1n = Self::PARAMS.map_b * (xd + Self::one()); // B * (xd + 1)
        xd *= -Self::PARAMS.map_a; // -A * xd

        let tv = Self::PARAMS.z * Self::PARAMS.map_a;
        xd.conditional_assign(&tv, xd.is_zero());

        tv2 = xd.square(); //xd^2
        let gxd = tv2 * xd; // xd^3
        tv2 *= Self::PARAMS.map_a; // A * tv2

        let mut gx1 = x1n * (tv2 + x1n.square()); //x1n *(tv2 + x1n^2)
        tv2 = gxd * Self::PARAMS.map_b; // B * gxd
        gx1 += tv2; // gx1 + tv2

        let mut tv4 = gxd.square(); // gxd^2
        tv2 = gx1 * gxd; // gx1 * gxd
        tv4 *= tv2;

        let y1 = tv4.pow_vartime(&Self::PARAMS.c1) * tv2; // tv4^C1 * tv2
        let x2n = tv3 * x1n; // tv3 * x1n

        let y2 = y1 * Self::PARAMS.c2 * tv1 * self; // y1 * c2 * tv1 * u

        tv2 = y1.square() * gxd; //y1^2 * gxd

        let e2 = tv2.ct_eq(&gx1);

        // if e2 , x = x1, else x = x2
        let mut x = Self::conditional_select(&x2n, &x1n, e2);
        // xn / xd
        x *= xd.invert().unwrap();

        // if e2, y = y1, else y = y2
        let mut y = Self::conditional_select(&y2, &y1, e2);

        y.conditional_assign(&-y, self.sgn0() ^ y.sgn0());
        (x, y)
    }
}
