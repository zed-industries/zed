//! Routines for parsing values into BigDecimals

use super::{BigDecimal, ParseBigDecimalError};
use stdlib::num::FpCategory;

use stdlib::cmp::{self, Ordering};

use num_bigint::{BigInt, BigUint, Sign};
use num_traits::Zero;


/// Try creating bigdecimal from f32
///
/// Non "normal" values will return Error case
///
pub(crate) fn try_parse_from_f32(n: f32) -> Result<BigDecimal, ParseBigDecimalError> {
    use stdlib::num::FpCategory::*;
    match n.classify() {
        Nan => Err(ParseBigDecimalError::Other("NAN".into())),
        Infinite => Err(ParseBigDecimalError::Other("Infinite".into())),
        Subnormal => Ok(parse_from_f32_subnormal(n)),
        Normal | Zero => Ok(parse_from_f32(n)),
    }
}


/// Return mantissa, exponent, and sign of given floating point number
///
/// ```math
/// f = frac * 2^pow
/// ```
///
fn split_f32_into_parts(f: f32) -> (u32, i64, Sign) {
    let bits = f.to_bits();
    let frac = (bits & ((1 << 23) - 1)) + (1 << 23);
    let exp = (bits >> 23) & 0xFF;

    let pow = exp as i64 - 127 - 23;

    let sign_bit = bits & (1 << 31);
    let sign = if sign_bit == 0 {
        Sign::Plus
    } else {
        Sign::Minus
    };

    (frac, pow, sign)
}


/// Create bigdecimal from f32
///
pub(crate) fn parse_from_f32(n: f32) -> BigDecimal {
    if n.classify() == FpCategory::Subnormal {
        return parse_from_f32_subnormal(n);
    }
    let bits = n.to_bits();

    if (bits << 1) == 0 {
        return Zero::zero();
    }

    // n = <sign> frac * 2^pow
    let (frac, pow, sign) = split_f32_into_parts(n);

    let result;
    let scale;
    match pow.cmp(&0) {
        Ordering::Equal => {
            result = BigUint::from(frac);
            scale = 0;
        }
        Ordering::Less => {
            let trailing_zeros = cmp::min(frac.trailing_zeros(), -pow as u32);

            let reduced_frac = frac >> trailing_zeros;
            let reduced_pow = pow + trailing_zeros as i64;
            debug_assert!(reduced_pow <= 0);

            let shift = BigUint::from(5u8).pow(-reduced_pow as u32);

            result = reduced_frac * shift;
            scale = -reduced_pow;
        }
        Ordering::Greater => {
            let shift = BigUint::from(2u8).pow(pow.abs() as u32);

            result = frac * shift;
            scale = 0;
        }
    }

    BigDecimal {
        int_val: BigInt::from_biguint(sign, result),
        scale: scale,
    }
}

/// Create bigdecimal from subnormal f32
pub(crate) fn parse_from_f32_subnormal(n: f32) -> BigDecimal {
    debug_assert_eq!(n.classify(), FpCategory::Subnormal);
    let bits = n.to_bits();

    let sign_bit = bits >> 31;
    debug_assert_eq!(bits >> 24, sign_bit << 7);

    let frac = bits - (sign_bit << 31);

    // 5^149 = 5^126 + 5^23  (f32-bit-bias=126, fraction-bits=23)
    let five_to_149 = BigUint::from_slice(&[
        1466336501, 2126633373, 2856417274, 1232167559, 2512314040, 1644054862,
        3843013918, 3873995871, 858643596, 3706384338, 65604258
    ]);

    let sign = if sign_bit == 0 { Sign::Plus } else { Sign::Minus };
    let magnitude = BigUint::from(frac) * five_to_149;
    let scale = 149;
    let result = BigDecimal::new(BigInt::from_biguint(sign, magnitude), scale);
    return result;
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test_parse_from_f32 {
    use super::*;

    include!("parsing.tests.parse_from_f32.rs");
}


/// Try creating bigdecimal from f64
///
/// Non "normal" values will return Error case
///
pub(crate) fn try_parse_from_f64(n: f64) -> Result<BigDecimal, ParseBigDecimalError> {
    use stdlib::num::FpCategory::*;
    match n.classify() {
        Nan => Err(ParseBigDecimalError::Other("NAN".into())),
        Infinite => Err(ParseBigDecimalError::Other("Infinite".into())),
        Subnormal => Ok(parse_from_f64_subnormal(n)),
        Normal | Zero => Ok(parse_from_f64(n)),
    }
}


/// Return mantissa, exponent, and sign of given floating point number
///
/// ```math
/// f = frac * 2^pow
/// ```
///
fn split_f64_into_parts(f: f64) -> (u64, i64, Sign) {
    let bits = f.to_bits();
    let frac = (bits & ((1 << 52) - 1)) + (1 << 52);
    let exp = (bits >> 52) & 0x7FF;

    let pow = exp as i64 - 1023 - 52;

    let sign_bit = bits & (1 << 63);
    let sign = if sign_bit == 0 {
        Sign::Plus
    } else {
        Sign::Minus
    };

    (frac, pow, sign)
}

/// Create bigdecimal from subnormal f64
pub(crate) fn parse_from_f64_subnormal(n: f64) -> BigDecimal {
    debug_assert_eq!(n.classify(), FpCategory::Subnormal);
    let bits = n.to_bits();

    let sign_bit = bits >> 63;
    debug_assert_eq!(bits >> 52, sign_bit << 11);

    // 5^1074 = 5^1022 + 5^52  (f64-bit-bias=1022, fraction-bits=52)
    let five_to_1074 = BigUint::from_slice(&[
        2993937753, 2678407619, 3969251600, 2340035423,  635686544, 3544357150, 2618749834,
        3195461310, 2593678749, 4014721034, 2512738537, 1379014958, 2606506302, 1209795638,
        3422246832, 2235398534, 2765471138, 3453720203, 3699786234, 1752628667, 3832472493,
        2479745915, 4210941784, 2088904316, 4137646701, 3840319652, 3815898978, 2202136831,
        1022273801, 1470939580, 2032173740, 4063736241, 2069243191, 4077145663, 4033014231,
        1920904652, 4195885152, 3551517817, 4246423481, 2447790869, 1797774111,   11284306,
         195273359, 3811183395, 4065514955, 3382133286, 1078447835, 2100087074, 3915378083,
        1127077286, 1409634978, 2331452623, 1301118814, 3692061923, 2506161869, 4270519152,
        1066095370,  212429084, 3729063602, 3175008277, 2075072468, 2136773221, 4247151843,
        2395660055,  449096848, 2439918400, 1564416362, 3638689409, 3054795416, 1803373736,
        1506581328, 2791252870, 3391180271, 1768177410, 3891987426, 3655546435, 3881223940,
         903390128
    ]);

    let frac = bits - (sign_bit << 63);

    let sign = if sign_bit == 0 { Sign::Plus } else { Sign::Minus };
    let magnitude = BigUint::from(frac) * five_to_1074;
    let scale = 1074;

    return BigDecimal::new(BigInt::from_biguint(sign, magnitude), scale);
}

/// Create bigdecimal from f64
///
/// Non "normal" values is undefined behavior
///
pub(crate) fn parse_from_f64(n: f64) -> BigDecimal {
    if n.classify() == FpCategory::Subnormal {
        return parse_from_f64_subnormal(n);
    }

    let bits = n.to_bits();

    // shift right by 1 bit to handle -0.0
    if (bits << 1) == 0 {
        return Zero::zero();
    }

    // n = <sign> frac * 2^pow
    let (frac, pow, sign) = split_f64_into_parts(n);
    debug_assert!(frac > 0);

    let result;
    let scale;
    match pow.cmp(&0) {
        Ordering::Equal => {
            result = BigUint::from(frac);
            scale = 0;
        }
        Ordering::Less => {
            let trailing_zeros = cmp::min(frac.trailing_zeros(), -pow as u32);

            let reduced_frac = frac >> trailing_zeros;
            let reduced_pow = pow + trailing_zeros as i64;
            debug_assert!(reduced_pow <= 0);

            let shift = BigUint::from(5u8).pow(-reduced_pow as u32);

            result = reduced_frac * shift;
            scale = -reduced_pow;
        }
        Ordering::Greater => {
            let shift = BigUint::from(2u8).pow(pow as u32);
            result = frac * shift;
            scale = 0;
        }
    }

    BigDecimal {
        int_val: BigInt::from_biguint(sign, result),
        scale: scale,
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_parse_from_f64 {
    use super::*;

    include!("parsing.tests.parse_from_f64.rs");
}
