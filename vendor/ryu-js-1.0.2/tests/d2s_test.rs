// Translated from C to Rust. The original C code can be found at
// https://github.com/ulfjack/ryu and carries the following license:
//
// Copyright 2018 Ulf Adams
//
// The contents of this file may be used under the terms of the Apache License,
// Version 2.0.
//
//    (See accompanying file LICENSE-Apache or copy at
//     http://www.apache.org/licenses/LICENSE-2.0)
//
// Alternatively, the contents of this file may be used under the terms of
// the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE-Boost or copy at
//     https://www.boost.org/LICENSE_1_0.txt)
//
// Unless required by applicable law or agreed to in writing, this software
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.

#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::cast_lossless,
    clippy::float_cmp,
    clippy::int_plus_one,
    clippy::non_ascii_literal,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix
)]

#[macro_use]
mod macros;

use std::f64;

fn pretty(f: f64) -> String {
    ryu_js::Buffer::new().format(f).to_owned()
}

#[allow(clippy::int_plus_one)]
fn ieee_parts_to_double(sign: bool, ieee_exponent: u32, ieee_mantissa: u64) -> f64 {
    assert!(ieee_exponent <= 2047);
    assert!(ieee_mantissa <= (1u64 << 53) - 1);
    f64::from_bits(((sign as u64) << 63) | ((ieee_exponent as u64) << 52) | ieee_mantissa)
}

#[test]
fn test_ryu() {
    check!(0.3);
    assert_eq!(pretty(1234000000000000.0), "1234000000000000");
    assert_eq!(pretty(1.234e16), "12340000000000000");
    check!(2.71828);
    assert_eq!(pretty(1.1e128), "1.1e+128");
    check!(1.1e-64);
    check!(2.718281828459045);
    check!(5e-324);
    assert_eq!(pretty(1.7976931348623157e308), "1.7976931348623157e+308");
}

#[test]
fn test_random() {
    let n = if cfg!(miri) { 100 } else { 1000000 };
    let mut buffer = ryu_js::Buffer::new();
    for _ in 0..n {
        let f: f64 = rand::random();
        assert_eq!(f, buffer.format_finite(f).parse().unwrap());
    }
}

#[test]
#[cfg_attr(miri, ignore = "too slow for miri")]
fn test_non_finite() {
    for i in 0u64..1 << 23 {
        let f = f64::from_bits((((1 << 11) - 1) << 52) + (i << 29));
        assert!(!f.is_finite(), "f={}", f);
        ryu_js::Buffer::new().format_finite(f);
    }
}

#[test]
fn test_basic() {
    assert_eq!(pretty(0.0), "0");
    assert_eq!(pretty(-0.0), "0");
    assert_eq!(pretty(1.0), "1");
    assert_eq!(pretty(-1.0), "-1");
    assert_eq!(pretty(f64::NAN.copysign(1.0)), "NaN");
    assert_eq!(pretty(f64::NAN.copysign(-1.0)), "NaN");
    assert_eq!(pretty(f64::INFINITY), "Infinity");
    assert_eq!(pretty(f64::NEG_INFINITY), "-Infinity");
}

#[test]
fn test_switch_to_subnormal() {
    check!(2.2250738585072014e-308);
}

#[test]
fn test_min_and_max() {
    assert_eq!(f64::from_bits(0x7fefffffffffffff), 1.7976931348623157e308);
    check!(1.7976931348623157e+308);
    assert_eq!(f64::from_bits(1), 5e-324);
    check!(5e-324);
}

#[test]
fn test_lots_of_trailing_zeros() {
    check!(2.9802322387695312e-8);
}

#[test]
fn test_regression() {
    assert_eq!(pretty(-21098088986959630.0), "-21098088986959630");
    check!(4.940656e-318);
    check!(1.18575755e-316);
    check!(2.989102097996e-312);
    assert_eq!(pretty(4.708356024711512e+18), "4708356024711512000");
    assert_eq!(pretty(9.409340012568248e+18), "9409340012568248000");
    check!(1.2345678);
}

#[test]
fn test_looks_like_pow5() {
    // These numbers have a mantissa that is a multiple of the largest power of
    // 5 that fits, and an exponent that causes the computation for q to result
    // in 22, which is a corner case for Ryū.
    assert_eq!(f64::from_bits(0x4830F0CF064DD592), 5.764607523034235e39);
    check!(5.764607523034235e+39);
    assert_eq!(f64::from_bits(0x4840F0CF064DD592), 1.152921504606847e40);
    check!(1.152921504606847e+40);
    assert_eq!(f64::from_bits(0x4850F0CF064DD592), 2.305843009213694e+40);
    check!(2.305843009213694e+40);
}

#[test]
fn test_output_length() {
    check!(1.2);
    check!(1.23);
    check!(1.234);
    check!(1.2345);
    check!(1.23456);
    check!(1.234567);
    check!(1.2345678); // already tested in Regression
    check!(1.23456789);
    check!(1.234567895); // 1.234567890 would be trimmed
    check!(1.2345678901);
    check!(1.23456789012);
    check!(1.234567890123);
    check!(1.2345678901234);
    check!(1.23456789012345);
    check!(1.234567890123456);
    check!(1.2345678901234567);

    // Test 32-bit chunking
    check!(4.294967294); // 2^32 - 2
    check!(4.294967295); // 2^32 - 1
    check!(4.294967296); // 2^32
    check!(4.294967297); // 2^32 + 1
    check!(4.294967298); // 2^32 + 2
}

// Test min, max shift values in shiftright128
#[test]
fn test_min_max_shift() {
    let max_mantissa = (1u64 << 53) - 1;

    // 32-bit opt-size=0:  49 <= dist <= 50
    // 32-bit opt-size=1:  30 <= dist <= 50
    // 64-bit opt-size=0:  50 <= dist <= 50
    // 64-bit opt-size=1:  30 <= dist <= 50
    assert_eq!(1.7800590868057611E-307, ieee_parts_to_double(false, 4, 0));
    check!(1.7800590868057611e-307);
    // 32-bit opt-size=0:  49 <= dist <= 49
    // 32-bit opt-size=1:  28 <= dist <= 49
    // 64-bit opt-size=0:  50 <= dist <= 50
    // 64-bit opt-size=1:  28 <= dist <= 50
    assert_eq!(
        2.8480945388892175E-306,
        ieee_parts_to_double(false, 6, max_mantissa)
    );
    check!(2.8480945388892175e-306);
    // 32-bit opt-size=0:  52 <= dist <= 53
    // 32-bit opt-size=1:   2 <= dist <= 53
    // 64-bit opt-size=0:  53 <= dist <= 53
    // 64-bit opt-size=1:   2 <= dist <= 53
    assert_eq!(2.446494580089078E-296, ieee_parts_to_double(false, 41, 0));
    check!(2.446494580089078e-296);
    // 32-bit opt-size=0:  52 <= dist <= 52
    // 32-bit opt-size=1:   2 <= dist <= 52
    // 64-bit opt-size=0:  53 <= dist <= 53
    // 64-bit opt-size=1:   2 <= dist <= 53
    assert_eq!(
        4.8929891601781557E-296,
        ieee_parts_to_double(false, 40, max_mantissa)
    );
    check!(4.8929891601781557e-296);

    // 32-bit opt-size=0:  57 <= dist <= 58
    // 32-bit opt-size=1:  57 <= dist <= 58
    // 64-bit opt-size=0:  58 <= dist <= 58
    // 64-bit opt-size=1:  58 <= dist <= 58
    assert_eq!(1.8014398509481984E16, ieee_parts_to_double(false, 1077, 0));
    assert_eq!(pretty(1.8014398509481984e16), "18014398509481984");
    // 32-bit opt-size=0:  57 <= dist <= 57
    // 32-bit opt-size=1:  57 <= dist <= 57
    // 64-bit opt-size=0:  58 <= dist <= 58
    // 64-bit opt-size=1:  58 <= dist <= 58
    assert_eq!(
        3.6028797018963964E16,
        ieee_parts_to_double(false, 1076, max_mantissa)
    );
    assert_eq!(pretty(3.6028797018963964e+16), "36028797018963964");
    // 32-bit opt-size=0:  51 <= dist <= 52
    // 32-bit opt-size=1:  51 <= dist <= 59
    // 64-bit opt-size=0:  52 <= dist <= 52
    // 64-bit opt-size=1:  52 <= dist <= 59
    assert_eq!(2.900835519859558E-216, ieee_parts_to_double(false, 307, 0));
    check!(2.900835519859558e-216);
    // 32-bit opt-size=0:  51 <= dist <= 51
    // 32-bit opt-size=1:  51 <= dist <= 59
    // 64-bit opt-size=0:  52 <= dist <= 52
    // 64-bit opt-size=1:  52 <= dist <= 59
    assert_eq!(
        5.801671039719115E-216,
        ieee_parts_to_double(false, 306, max_mantissa)
    );
    check!(5.801671039719115e-216);

    // https://github.com/ulfjack/ryu/commit/19e44d16d80236f5de25800f56d82606d1be00b9#commitcomment-30146483
    // 32-bit opt-size=0:  49 <= dist <= 49
    // 32-bit opt-size=1:  44 <= dist <= 49
    // 64-bit opt-size=0:  50 <= dist <= 50
    // 64-bit opt-size=1:  44 <= dist <= 50
    assert_eq!(
        3.196104012172126E-27,
        ieee_parts_to_double(false, 934, 0x000FA7161A4D6E0C)
    );
    check!(3.196104012172126e-27);
}

#[test]
fn test_ecma262_compliance() {
    assert_eq!(pretty(f64::NAN), "NaN");
    assert_eq!(pretty(f64::INFINITY), "Infinity");
    assert_eq!(pretty(f64::NEG_INFINITY), "-Infinity");
    assert_eq!(pretty(0.0), "0");
    assert_eq!(pretty(9.0), "9");
    assert_eq!(pretty(90.0), "90");
    assert_eq!(pretty(90.12), "90.12");

    assert_eq!(pretty(0.000001), "0.000001");
    assert_eq!(pretty(0.0000001), "1e-7");
    assert_eq!(pretty(3e50), "3e+50");

    assert_eq!(pretty(90.12), "90.12");

    assert_eq!(pretty(111111111111111111111.0), "111111111111111110000");
    assert_eq!(pretty(1111111111111111111111.0), "1.1111111111111111e+21");
    assert_eq!(pretty(11111111111111111111111.0), "1.1111111111111111e+22");

    assert_eq!(pretty(0.1), "0.1");
    assert_eq!(pretty(0.01), "0.01");
    assert_eq!(pretty(0.001), "0.001");
    assert_eq!(pretty(0.0001), "0.0001");
    assert_eq!(pretty(0.00001), "0.00001");
    assert_eq!(pretty(0.000001), "0.000001");
    assert_eq!(pretty(0.0000001), "1e-7");
    assert_eq!(pretty(0.00000012), "1.2e-7");
    assert_eq!(pretty(0.000000123), "1.23e-7");
    assert_eq!(pretty(0.00000001), "1e-8");

    assert_eq!(pretty(-0.0), "0");
    assert_eq!(pretty(-9.0), "-9");
    assert_eq!(pretty(-90.12), "-90.12");
    assert_eq!(pretty(-0.0000000123), "-1.23e-8");
    assert_eq!(pretty(-111111111111111111111.0), "-111111111111111110000");
    assert_eq!(pretty(-1111111111111111111111.0), "-1.1111111111111111e+21");
    assert_eq!(pretty(-0.000000123), "-1.23e-7");

    assert_eq!(
        pretty(123456789010111213141516171819.0),
        "1.234567890101112e+29"
    );
}

#[test]
fn max_size_double_to_string() {
    // See: https://viewer.scuttlebot.io/%25LQo5KOMeR%2Baj%2BEj0JVg3qLRqr%2BwiKo74nS8Uz7o0LDM%3D.sha256
    assert_eq!(
        pretty(-0.0000015809161985788154),
        "-0.0000015809161985788154"
    );
}
