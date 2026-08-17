//! Complete projective formulas for prime order elliptic curves as described
//! in [Renes-Costello-Batina 2015].
//!
//! [Renes-Costello-Batina 2015]: https://eprint.iacr.org/2015/1060

#![allow(clippy::op_ref)]

use ff::Field;

/// Affine point whose coordinates are represented by the given field element.
pub type AffinePoint<Fe> = (Fe, Fe);

/// Projective point whose coordinates are represented by the given field element.
pub type ProjectivePoint<Fe> = (Fe, Fe, Fe);

/// Implements the complete addition formula from [Renes-Costello-Batina 2015]
/// (Algorithm 4).
///
/// [Renes-Costello-Batina 2015]: https://eprint.iacr.org/2015/1060
#[inline(always)]
pub fn add<Fe>(
    (ax, ay, az): ProjectivePoint<Fe>,
    (bx, by, bz): ProjectivePoint<Fe>,
    curve_equation_b: Fe,
) -> ProjectivePoint<Fe>
where
    Fe: Field,
{
    // The comments after each line indicate which algorithm steps are being
    // performed.
    let xx = ax * bx; // 1
    let yy = ay * by; // 2
    let zz = az * bz; // 3
    let xy_pairs = ((ax + ay) * &(bx + by)) - &(xx + &yy); // 4, 5, 6, 7, 8
    let yz_pairs = ((ay + az) * &(by + bz)) - &(yy + &zz); // 9, 10, 11, 12, 13
    let xz_pairs = ((ax + az) * &(bx + bz)) - &(xx + &zz); // 14, 15, 16, 17, 18

    let bzz_part = xz_pairs - &(curve_equation_b * &zz); // 19, 20
    let bzz3_part = bzz_part.double() + &bzz_part; // 21, 22
    let yy_m_bzz3 = yy - &bzz3_part; // 23
    let yy_p_bzz3 = yy + &bzz3_part; // 24

    let zz3 = zz.double() + &zz; // 26, 27
    let bxz_part = (curve_equation_b * &xz_pairs) - &(zz3 + &xx); // 25, 28, 29
    let bxz3_part = bxz_part.double() + &bxz_part; // 30, 31
    let xx3_m_zz3 = xx.double() + &xx - &zz3; // 32, 33, 34

    (
        (yy_p_bzz3 * &xy_pairs) - &(yz_pairs * &bxz3_part), // 35, 39, 40
        (yy_p_bzz3 * &yy_m_bzz3) + &(xx3_m_zz3 * &bxz3_part), // 36, 37, 38
        (yy_m_bzz3 * &yz_pairs) + &(xy_pairs * &xx3_m_zz3), // 41, 42, 43
    )
}

/// Implements the complete mixed addition formula from
/// [Renes-Costello-Batina 2015] (Algorithm 5).
///
/// [Renes-Costello-Batina 2015]: https://eprint.iacr.org/2015/1060
#[inline(always)]
pub fn add_mixed<Fe>(
    (ax, ay, az): ProjectivePoint<Fe>,
    (bx, by): AffinePoint<Fe>,
    curve_equation_b: Fe,
) -> ProjectivePoint<Fe>
where
    Fe: Field,
{
    // The comments after each line indicate which algorithm steps are being
    // performed.
    let xx = ax * &bx; // 1
    let yy = ay * &by; // 2
    let xy_pairs = ((ax + &ay) * &(bx + &by)) - &(xx + &yy); // 3, 4, 5, 6, 7
    let yz_pairs = (by * &az) + &ay; // 8, 9 (t4)
    let xz_pairs = (bx * &az) + &ax; // 10, 11 (y3)

    let bz_part = xz_pairs - &(curve_equation_b * &az); // 12, 13
    let bz3_part = bz_part.double() + &bz_part; // 14, 15
    let yy_m_bzz3 = yy - &bz3_part; // 16
    let yy_p_bzz3 = yy + &bz3_part; // 17

    let z3 = az.double() + &az; // 19, 20
    let bxz_part = (curve_equation_b * &xz_pairs) - &(z3 + &xx); // 18, 21, 22
    let bxz3_part = bxz_part.double() + &bxz_part; // 23, 24
    let xx3_m_zz3 = xx.double() + &xx - &z3; // 25, 26, 27

    (
        (yy_p_bzz3 * &xy_pairs) - &(yz_pairs * &bxz3_part), // 28, 32, 33
        (yy_p_bzz3 * &yy_m_bzz3) + &(xx3_m_zz3 * &bxz3_part), // 29, 30, 31
        (yy_m_bzz3 * &yz_pairs) + &(xy_pairs * &xx3_m_zz3), // 34, 35, 36
    )
}

/// Implements the exception-free point doubling formula from
/// [Renes-Costello-Batina 2015] (Algorithm 6).
///
/// [Renes-Costello-Batina 2015]: https://eprint.iacr.org/2015/1060
#[inline(always)]
pub fn double<Fe>((x, y, z): ProjectivePoint<Fe>, curve_equation_b: Fe) -> ProjectivePoint<Fe>
where
    Fe: Field,
{
    // The comments after each line indicate which algorithm steps are being
    // performed.
    let xx = x.square(); // 1
    let yy = y.square(); // 2
    let zz = z.square(); // 3
    let xy2 = (x * &y).double(); // 4, 5
    let xz2 = (x * &z).double(); // 6, 7

    let bzz_part = (curve_equation_b * &zz) - &xz2; // 8, 9
    let bzz3_part = bzz_part.double() + &bzz_part; // 10, 11
    let yy_m_bzz3 = yy - &bzz3_part; // 12
    let yy_p_bzz3 = yy + &bzz3_part; // 13
    let y_frag = yy_p_bzz3 * &yy_m_bzz3; // 14
    let x_frag = yy_m_bzz3 * &xy2; // 15

    let zz3 = zz.double() + &zz; // 16, 17
    let bxz2_part = (curve_equation_b * &xz2) - &(zz3 + &xx); // 18, 19, 20
    let bxz6_part = bxz2_part.double() + &bxz2_part; // 21, 22
    let xx3_m_zz3 = xx.double() + &xx - &zz3; // 23, 24, 25

    let dy = y_frag + &(xx3_m_zz3 * &bxz6_part); // 26, 27
    let yz2 = (y * &z).double(); // 28, 29
    let dx = x_frag - &(bxz6_part * &yz2); // 30, 31
    let dz = (yz2 * &yy).double().double(); // 32, 33, 34

    (dx, dy, dz)
}
