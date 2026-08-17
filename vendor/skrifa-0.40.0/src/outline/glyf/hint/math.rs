//! Fixed point math helpers that are specific to TrueType hinting.
//!
//! These are implemented in terms of font-types types when possible. It
//! likely makes sense to use more strongly typed fixed point values
//! in the future.

use read_fonts::types::{Fixed, Point};

pub fn floor(x: i32) -> i32 {
    x & !63
}

pub fn round(x: i32) -> i32 {
    floor(x + 32)
}

pub fn ceil(x: i32) -> i32 {
    floor(x + 63)
}

fn floor_pad(x: i32, n: i32) -> i32 {
    x & !(n - 1)
}

pub fn round_pad(x: i32, n: i32) -> i32 {
    floor_pad(x + n / 2, n)
}

#[inline(always)]
pub fn mul(a: i32, b: i32) -> i32 {
    (Fixed::from_bits(a) * Fixed::from_bits(b)).to_bits()
}

pub fn div(a: i32, b: i32) -> i32 {
    (Fixed::from_bits(a) / Fixed::from_bits(b)).to_bits()
}

/// Fixed point multiply and divide: a * b / c
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    Fixed::from_bits(a)
        .mul_div(Fixed::from_bits(b), Fixed::from_bits(c))
        .to_bits()
}

/// Fixed point multiply and divide without rounding: a * b / c
///
/// Based on <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/base/ftcalc.c#L200>
pub fn mul_div_no_round(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    let mut s = 1;
    if a < 0 {
        a = -a;
        s = -1;
    }
    if b < 0 {
        b = -b;
        s = -s;
    }
    if c < 0 {
        c = -c;
        s = -s;
    }
    let d = if c > 0 {
        ((a as i64) * (b as i64)) / c as i64
    } else {
        0x7FFFFFFF
    };
    if s < 0 {
        -(d as i32)
    } else {
        d as i32
    }
}

/// Multiplication for 2.14 fixed point.
///
/// Based on <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L1234>
pub fn mul14(a: i32, b: i32) -> i32 {
    let mut v = a as i64 * b as i64;
    v += 0x2000 + (v >> 63);
    (v >> 14) as i32
}

/// Normalize a vector in 2.14 fixed point.
///
/// Direct port of <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/base/ftcalc.c#L800>
pub fn normalize14(x: i32, y: i32) -> Point<i32> {
    use core::num::Wrapping;
    let (mut sx, mut sy) = (Wrapping(1i32), Wrapping(1i32));
    let mut ux = Wrapping(x as u32);
    let mut uy = Wrapping(y as u32);
    const ZERO: Wrapping<u32> = Wrapping(0);
    let mut result = Point::default();
    if x < 0 {
        ux = ZERO - ux;
        sx = -sx;
    }
    if y < 0 {
        uy = ZERO - uy;
        sy = -sy;
    }
    if ux == ZERO {
        result.x = x / 4;
        if uy.0 > 0 {
            result.y = (sy * Wrapping(0x10000) / Wrapping(4)).0;
        }
        return result;
    }
    if uy == ZERO {
        result.y = y / 4;
        if ux.0 > 0 {
            result.x = (sx * Wrapping(0x10000) / Wrapping(4)).0;
        }
        return result;
    }
    let mut len = if ux > uy {
        ux + (uy >> 1)
    } else {
        uy + (ux >> 1)
    };
    let mut shift = Wrapping(len.0.leading_zeros() as i32);
    shift -= Wrapping(15)
        + if len >= (Wrapping(0xAAAAAAAAu32) >> shift.0 as usize) {
            Wrapping(1)
        } else {
            Wrapping(0)
        };
    if shift.0 > 0 {
        let s = shift.0 as usize;
        ux <<= s;
        uy <<= s;
        len = if ux > uy {
            ux + (uy >> 1)
        } else {
            uy + (ux >> 1)
        };
    } else {
        let s = -shift.0 as usize;
        ux >>= s;
        uy >>= s;
        len >>= s;
    }
    let mut b = Wrapping(0x10000) - Wrapping(len.0 as i32);
    let x = Wrapping(ux.0 as i32);
    let y = Wrapping(uy.0 as i32);
    let mut z;
    let mut u;
    let mut v;
    loop {
        u = Wrapping((x + ((x * b) >> 16)).0 as u32);
        v = Wrapping((y + ((y * b) >> 16)).0 as u32);
        z = Wrapping(-((u * u + v * v).0 as i32)) / Wrapping(0x200);
        z = z * ((Wrapping(0x10000) + b) >> 8) / Wrapping(0x10000);
        b += z;
        if z <= Wrapping(0) {
            break;
        }
    }
    Point::new(
        (Wrapping(u.0 as i32) * sx / Wrapping(4)).0,
        (Wrapping(v.0 as i32) * sy / Wrapping(4)).0,
    )
}

#[cfg(test)]
mod tests {
    use raw::types::{F2Dot14, Fixed};

    /// Tolerance value for floating point sanity checks.
    /// Tests with large sets of values show this is the best we can
    /// expect from the fixed point implementations.
    const FLOAT_TOLERANCE: f32 = 1e-4;

    #[test]
    fn mul_div_no_round() {
        let cases = [
            // computed with FT_MulDiv_NoRound():
            // ((a, b, c), result) where result = a * b / c
            ((-326, -11474, 9942), 376),
            ((-6781, 13948, 11973), -7899),
            ((3517, 15622, 8075), 6804),
            ((-6127, 15026, 2276), -40450),
            ((11257, 14828, 2542), 65664),
            ((-12797, -16280, -9086), -22929),
            ((-7994, -3340, 9583), 2786),
            ((-16101, -13780, -1427), -155481),
            ((10304, -16331, 15480), -10870),
            ((-15879, 11912, -4650), 40677),
            ((-5015, 6382, -15977), 2003),
            ((2080, -11930, -15457), 1605),
            ((-11071, 13350, 16138), -9158),
            ((16084, -13564, -770), 283329),
            ((14304, -10377, -21), 7068219),
            ((-14056, -8853, -5488), -22674),
            ((-10319, 14797, 8554), -17850),
            ((-7820, 6826, 10555), -5057),
            ((7257, 15928, 8159), 14167),
            ((14929, 11579, -13204), -13091),
            ((2808, 12070, -14697), -2306),
            ((-13818, 8544, -1649), 71595),
            ((3265, 7325, -1373), -17418),
            ((14832, 10586, -6440), -24380),
            ((4123, 8274, -2022), -16871),
            ((4645, -4149, -7242), 2661),
            ((-3891, 8366, 5771), -5640),
            ((-15447, -3428, -9335), -5672),
            ((13670, -14311, -11122), 17589),
            ((12590, -6592, 13159), -6306),
            ((-8369, -10193, 5051), 16888),
            ((-9539, 5167, 2595), -18993),
        ];
        for ((a, b, c), expected_result) in cases {
            let result = super::mul_div_no_round(a, b, c);
            assert_eq!(result, expected_result);
            let fa = Fixed::from_bits(a as _).to_f32();
            let fb = Fixed::from_bits(b as _).to_f32();
            let fc = Fixed::from_bits(c as _).to_f32();
            let fresult = fa * fb / fc;
            let fexpected_result = Fixed::from_bits(expected_result as _).to_f32();
            assert!((fresult - fexpected_result).abs() < FLOAT_TOLERANCE);
        }
    }

    #[test]
    fn mul14() {
        let cases = [
            // computed with TT_MulFix14():
            // ((a, b), result) where result = a * b
            ((6236, -10078), -3836),
            ((-6803, -5405), 2244),
            ((-10006, -12852), 7849),
            ((-15434, -4102), 3864),
            ((-8681, 9269), -4911),
            ((9449, -9130), -5265),
            ((12643, 2161), 1668),
            ((-6115, 9284), -3465),
            ((316, 3390), 65),
            ((15077, -12901), -11872),
            ((-12182, 11613), -8635),
            ((-7213, 8246), -3630),
            ((13482, 8096), 6662),
            ((5690, 15016), 5215),
            ((-5991, 12613), -4612),
            ((13112, -8404), -6726),
            ((13524, 6786), 5601),
            ((7156, 3291), 1437),
            ((-2978, 353), -64),
            ((-1755, 14626), -1567),
            ((14402, 7886), 6932),
            ((7124, 15730), 6840),
            ((-12679, 14830), -11476),
            ((-9374, -12999), 7437),
            ((12301, -4685), -3517),
            ((5324, 2066), 671),
            ((6783, -4946), -2048),
            ((12078, -968), -714),
            ((-10137, 14116), -8734),
            ((-13946, 11585), -9861),
            ((-678, -2205), 91),
            ((-2629, -3319), 533),
        ];
        for ((a, b), expected_result) in cases {
            let result = super::mul14(a, b);
            assert_eq!(result, expected_result);
            let fa = F2Dot14::from_bits(a as _).to_f32();
            let fb = F2Dot14::from_bits(b as _).to_f32();
            let fresult = fa * fb;
            let fexpected_result = F2Dot14::from_bits(expected_result as _).to_f32();
            assert!((fresult - fexpected_result).abs() < FLOAT_TOLERANCE);
        }
    }

    #[test]
    fn normalize14() {
        let cases = [
            // computed with FT_Vector_NormLen():
            // (input vector, expected normalized vector)
            ((-13660, 11807), (-12395, 10713)),
            ((-10763, 9293), (-12401, 10707)),
            ((-3673, 673), (-16115, 2952)),
            ((15886, -2964), (16106, -3005)),
            ((15442, -2871), (16108, -2994)),
            ((-6308, 5744), (-12114, 11031)),
            ((9410, -10415), (10983, -12156)),
            ((-10620, -14856), (-9528, -13328)),
            ((-9372, 12029), (-10069, 12924)),
            ((-1272, -1261), (-11635, -11534)),
            ((-7076, -5517), (-12920, -10074)),
            ((-10297, 179), (-16381, 284)),
            ((9256, -13235), (9389, -13426)),
            ((5315, -12449), (6433, -15068)),
            ((8064, 15213), (7673, 14476)),
            ((-8665, 41), (-16383, 77)),
            ((-3455, -4720), (-9677, -13220)),
            ((13449, -5152), (15299, -5861)),
            ((-15605, 8230), (-14492, 7643)),
            ((4716, -13690), (5336, -15490)),
            ((12904, -11422), (12268, -10859)),
            ((2825, -6396), (6619, -14987)),
            ((4654, 15245), (4783, 15670)),
            ((-14769, 15133), (-11443, 11725)),
            ((-8090, -9057), (-10914, -12219)),
            ((-472, 1953), (-3848, 15925)),
            ((-12563, 1040), (-16328, 1351)),
            ((-7938, 15587), (-7435, 14599)),
            ((-9701, 5356), (-14343, 7919)),
            ((-642, -14484), (-725, -16367)),
            ((12963, -9690), (13123, -9809)),
            ((7067, 5361), (13053, 9902)),
            ((0x4000, 0), (0x4000, 0)),
            ((0, 0x4000), (0, 0x4000)),
            ((-0x4000, 0), (-0x4000, 0)),
            ((0, -0x4000), (0, -0x4000)),
        ];
        for ((x, y), expected) in cases {
            let n = super::normalize14(x, y);
            assert_eq!((n.x, n.y), expected);
            // Ensure the length of the vector is nearly 1.0
            let fx = F2Dot14::from_bits(n.x as _).to_f32();
            let fy = F2Dot14::from_bits(n.y as _).to_f32();
            let flen = (fx * fx + fy * fy).sqrt();
            assert!((flen - 1.0).abs() <= FLOAT_TOLERANCE);
        }
    }
}
