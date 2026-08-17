/*
 * // Copyright (c) Radzivon Bartoshyk 6/2025. All rights reserved.
 * //
 * // Redistribution and use in source and binary forms, with or without modification,
 * // are permitted provided that the following conditions are met:
 * //
 * // 1.  Redistributions of source code must retain the above copyright notice, this
 * // list of conditions and the following disclaimer.
 * //
 * // 2.  Redistributions in binary form must reproduce the above copyright notice,
 * // this list of conditions and the following disclaimer in the documentation
 * // and/or other materials provided with the distribution.
 * //
 * // 3.  Neither the name of the copyright holder nor the names of its
 * // contributors may be used to endorse or promote products derived from
 * // this software without specific prior written permission.
 * //
 * // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * // AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * // IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * // FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * // DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * // CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * // OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use crate::common::f_fmla;
use crate::logs::LOG_RANGE_REDUCTION;
use crate::polyeval::{f_estrin_polyeval7, f_polyeval6};

// Generated in SageMath:
// print("[")
// for i in range(128):
//     R = RealField(200)
//     r = R(2)**(-8) * ( R(2)**8 * (R(1) - R(2)**(-8)) / (R(1) + R(i)*R(2)**-7) ).ceil()
//
//     if i == 0 or i == 127:
//         print(double_to_hex(0), ",")
//     else:
//         print(double_to_hex(-r.log2()), ",")
// print("];")
static LOG2_D: [u64; 128] = [
    0x0000000000000000,
    0x3f872c7ba20f7327,
    0x3f9743ee861f3556,
    0x3fa184b8e4c56af8,
    0x3fa77394c9d958d5,
    0x3fad6ebd1f1febfe,
    0x3fb1bb32a600549d,
    0x3fb4c560fe68af88,
    0x3fb7d60496cfbb4c,
    0x3fb960caf9abb7ca,
    0x3fbc7b528b70f1c5,
    0x3fbf9c95dc1d1165,
    0x3fc097e38ce60649,
    0x3fc22dadc2ab3497,
    0x3fc3c6fb650cde51,
    0x3fc494f863b8df35,
    0x3fc633a8bf437ce1,
    0x3fc7046031c79f85,
    0x3fc8a8980abfbd32,
    0x3fc97c1cb13c7ec1,
    0x3fcb2602497d5346,
    0x3fcbfc67a7fff4cc,
    0x3fcdac22d3e441d3,
    0x3fce857d3d361368,
    0x3fd01d9bbcfa61d4,
    0x3fd08bce0d95fa38,
    0x3fd169c05363f158,
    0x3fd1d982c9d52708,
    0x3fd249cd2b13cd6c,
    0x3fd32bfee370ee68,
    0x3fd39de8e1559f6f,
    0x3fd4106017c3eca3,
    0x3fd4f6fbb2cec598,
    0x3fd56b22e6b578e5,
    0x3fd5dfdcf1eeae0e,
    0x3fd6552b49986277,
    0x3fd6cb0f6865c8ea,
    0x3fd7b89f02cf2aad,
    0x3fd8304d90c11fd3,
    0x3fd8a8980abfbd32,
    0x3fd921800924dd3b,
    0x3fd99b072a96c6b2,
    0x3fda8ff971810a5e,
    0x3fdb0b67f4f46810,
    0x3fdb877c57b1b070,
    0x3fdc043859e2fdb3,
    0x3fdc819dc2d45fe4,
    0x3fdcffae611ad12b,
    0x3fdd7e6c0abc3579,
    0x3fddfdd89d586e2b,
    0x3fde7df5fe538ab3,
    0x3fdefec61b011f85,
    0x3fdf804ae8d0cd02,
    0x3fe0014332be0033,
    0x3fe042bd4b9a7c99,
    0x3fe08494c66b8ef0,
    0x3fe0c6caaf0c5597,
    0x3fe1096015dee4da,
    0x3fe14c560fe68af9,
    0x3fe18fadb6e2d3c2,
    0x3fe1d368296b5255,
    0x3fe217868b0c37e8,
    0x3fe25c0a0463beb0,
    0x3fe2a0f3c340705c,
    0x3fe2e644fac04fd8,
    0x3fe2e644fac04fd8,
    0x3fe32bfee370ee68,
    0x3fe37222bb70747c,
    0x3fe3b8b1c68fa6ed,
    0x3fe3ffad4e74f1d6,
    0x3fe44716a2c08262,
    0x3fe44716a2c08262,
    0x3fe48eef19317991,
    0x3fe4d7380dcc422d,
    0x3fe51ff2e30214bc,
    0x3fe5692101d9b4a6,
    0x3fe5b2c3da19723b,
    0x3fe5b2c3da19723b,
    0x3fe5fcdce2727ddb,
    0x3fe6476d98ad990a,
    0x3fe6927781d932a8,
    0x3fe6927781d932a8,
    0x3fe6ddfc2a78fc63,
    0x3fe729fd26b707c8,
    0x3fe7767c12967a45,
    0x3fe7767c12967a45,
    0x3fe7c37a9227e7fb,
    0x3fe810fa51bf65fd,
    0x3fe810fa51bf65fd,
    0x3fe85efd062c656d,
    0x3fe8ad846cf369a4,
    0x3fe8ad846cf369a4,
    0x3fe8fc924c89ac84,
    0x3fe94c287492c4db,
    0x3fe94c287492c4db,
    0x3fe99c48be2063c8,
    0x3fe9ecf50bf43f13,
    0x3fe9ecf50bf43f13,
    0x3fea3e2f4ac43f60,
    0x3fea8ff971810a5e,
    0x3fea8ff971810a5e,
    0x3feae255819f022d,
    0x3feb35458761d479,
    0x3feb35458761d479,
    0x3feb88cb9a2ab521,
    0x3feb88cb9a2ab521,
    0x3febdce9dcc96187,
    0x3fec31a27dd00b4a,
    0x3fec31a27dd00b4a,
    0x3fec86f7b7ea4a89,
    0x3fec86f7b7ea4a89,
    0x3fecdcebd2373995,
    0x3fed338120a6dd9d,
    0x3fed338120a6dd9d,
    0x3fed8aba045b01c8,
    0x3fed8aba045b01c8,
    0x3fede298ec0bac0d,
    0x3fede298ec0bac0d,
    0x3fee3b20546f554a,
    0x3fee3b20546f554a,
    0x3fee9452c8a71028,
    0x3fee9452c8a71028,
    0x3feeee32e2aeccbf,
    0x3feeee32e2aeccbf,
    0x3fef48c34bd1e96f,
    0x3fef48c34bd1e96f,
    0x3fefa406bd2443df,
    0x0000000000000000,
];

#[inline]
pub(crate) fn core_log2f(x: f64) -> f64 {
    let x_u = x.to_bits();

    const E_BIAS: u64 = (1u64 << (11 - 1u64)) - 1u64;

    let mut x_e: i32 = -(E_BIAS as i32);

    // log2(x) = log2(2^x_e * x_m)
    //         = x_e + log2(x_m)
    // Range reduction for log2(x_m):
    // For each x_m, we would like to find r such that:
    //   -2^-8 <= r * x_m - 1 < 2^-7
    let shifted = (x_u >> 45) as i32;
    let index = shifted & 0x7F;
    let r = f64::from_bits(LOG_RANGE_REDUCTION[index as usize]);

    // Add unbiased exponent. Add an extra 1 if the 8 leading fractional bits are
    // all 1's.
    x_e = x_e.wrapping_add(x_u.wrapping_add(1u64 << 45).wrapping_shr(52) as i32);
    let e_x = x_e as f64;

    let log_r_dd = LOG2_D[index as usize];

    // hi is exact
    let hi = e_x + f64::from_bits(log_r_dd);

    // Set m = 1.mantissa.
    let x_m = (x_u & 0x000F_FFFF_FFFF_FFFFu64) | 0x3FF0_0000_0000_0000u64;
    let m = f64::from_bits(x_m);

    let u;
    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    ))]
    {
        u = f_fmla(r, m, -1.0); // exact
    }
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "fma"
        ),
        target_arch = "aarch64"
    )))]
    {
        use crate::logs::LOG_CD;
        let c_m = x_m & 0x3FFF_E000_0000_0000u64;
        let c = f64::from_bits(c_m);
        u = f_fmla(r, m - c, f64::from_bits(LOG_CD[index as usize])); // exact
    }

    // Polynomial for log(1+x)/x generated in Sollya:
    // d = [-2^-8, 2^-7];
    // f_log2p1 = log2(1 + x)/x;
    // Q = fpminimax(f_log2p1, 6, [|D...|], d);
    // See ./notes/log10pf_core.sollya
    let p = f_polyeval6(
        u,
        f64::from_bits(0x3ff71547652b82fe),
        f64::from_bits(0xbfe71547652b82e7),
        f64::from_bits(0x3fdec709dc3b1fd5),
        f64::from_bits(0xbfd7154766124214),
        f64::from_bits(0x3fd2776bd902599e),
        f64::from_bits(0xbfcec586c6f55d08),
    );
    f_fmla(p, u, hi)
}

/// Computes log2(x+1)
///
/// Max ULP 0.5
#[inline]
pub fn f_log2p1f(x: f32) -> f32 {
    let z = x as f64;
    let ux = x.to_bits().wrapping_shl(1);
    if ux >= 0xffu32 << 24 || ux == 0 {
        // |x| == 0, |x| == inf, x == NaN
        if ux == 0 {
            return x;
        }
        if x.is_infinite() {
            return if x.is_sign_positive() {
                f32::INFINITY
            } else {
                f32::NAN
            };
        }
        return x + f32::NAN;
    }

    let ax = x.to_bits() & 0x7fff_ffffu32;

    // Use log2p1(x) = log10(1 + x) for |x| > 2^-6;
    if ax > 0x3c80_0000u32 {
        if x == -1. {
            return f32::NEG_INFINITY;
        }
        let x1p = z + 1.;
        if x1p <= 0. {
            if x1p == 0. {
                return f32::NEG_INFINITY;
            }
            return f32::NAN;
        }
        return core_log2f(x1p) as f32;
    }

    // log2p1 is expected to be used near zero:
    // Polynomial generated by Sollya:
    // d = [-2^-6; 2^-6];
    // f_log2pf = log2(1+x)/x;
    // Q = fpminimax(f_log2pf, 6, [|D...|], d);
    const C: [u64; 7] = [
        0x3ff71547652b82fe,
        0xbfe71547652b8d18,
        0x3fdec709dc3a501c,
        0xbfd715475b117c95,
        0x3fd2776c3fd833bd,
        0xbfcec9905627faa6,
        0x3fca64536a0ad148,
    ];
    let p = f_estrin_polyeval7(
        z,
        f64::from_bits(C[0]),
        f64::from_bits(C[1]),
        f64::from_bits(C[2]),
        f64::from_bits(C[3]),
        f64::from_bits(C[4]),
        f64::from_bits(C[5]),
        f64::from_bits(C[6]),
    );
    (p * z) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log2p1f() {
        assert_eq!(f_log2p1f(0.0), 0.0);
        assert_eq!(f_log2p1f(1.0), 1.0);
        assert_eq!(f_log2p1f(-0.0432432), -0.063775845);
        assert_eq!(f_log2p1f(-0.009874634), -0.01431689);
        assert_eq!(f_log2p1f(1.2443), 1.1662655);
        assert_eq!(f_log2p1f(f32::INFINITY), f32::INFINITY);
        assert!(f_log2p1f(f32::NEG_INFINITY).is_nan());
        assert!(f_log2p1f(-1.0432432).is_nan());
    }
}
