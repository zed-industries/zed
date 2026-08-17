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
    clippy::float_cmp,
    clippy::non_ascii_literal,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix
)]

#[macro_use]
mod macros;

use std::f32;

fn pretty(f: f32) -> String {
    ryu_js::Buffer::new().format(f).to_owned()
}

#[test]
fn test_ryu() {
    assert_eq!(pretty(1234000000000.0), "1234000000000");
    assert_eq!(pretty(1.234e13), "12340000000000");
    assert_eq!(pretty(2.71828), "2.71828");
    assert_eq!(pretty(1.1e32), "1.1e+32");
    assert_eq!(pretty(1.1e-32), "1.1e-32");
    assert_eq!(pretty(2.7182817), "2.7182817");
    assert_eq!(pretty(1e-45), "1e-45");
    assert_eq!(pretty(3.4028235e38), "3.4028235e+38");
    assert_eq!(pretty(-0.001234), "-0.001234");
}

#[test]
fn test_random() {
    let n = if cfg!(miri) { 100 } else { 1000000 };
    let mut buffer = ryu_js::Buffer::new();
    for _ in 0..n {
        let f: f32 = rand::random();
        assert_eq!(f, buffer.format_finite(f).parse().unwrap());
    }
}

#[test]
#[cfg_attr(miri, ignore = "too slow for miri")]
fn test_non_finite() {
    for i in 0u32..1 << 23 {
        let f = f32::from_bits((((1 << 8) - 1) << 23) + i);
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
    assert_eq!(pretty(f32::NAN.copysign(1.0)), "NaN");
    assert_eq!(pretty(f32::NAN.copysign(-1.0)), "NaN");
    assert_eq!(pretty(f32::INFINITY), "Infinity");
    assert_eq!(pretty(f32::NEG_INFINITY), "-Infinity");
}

#[test]
fn test_switch_to_subnormal() {
    check!(1.1754944e-38);
}

#[test]
fn test_min_and_max() {
    assert_eq!(f32::from_bits(0x7f7fffff), 3.4028235e38);
    assert_eq!(pretty(3.4028235e38), "3.4028235e+38");
    assert_eq!(f32::from_bits(1), 1e-45);
    assert_eq!(pretty(1e-45), "1e-45");
}

// Check that we return the exact boundary if it is the shortest
// representation, but only if the original floating point number is even.
#[test]
fn test_boundary_round_even() {
    assert_eq!(pretty(33554450.0), "33554450");
    assert_eq!(pretty(9000000000.0), "9000000000");
    assert_eq!(pretty(34366720000.0), "34366720000");
}

// If the exact value is exactly halfway between two shortest representations,
// then we round to even. It seems like this only makes a difference if the
// last two digits are ...2|5 or ...7|5, and we cut off the 5.
#[test]
fn test_exact_value_round_even() {
    check!(305404.12);
    check!(8099.0312);
}

#[test]
fn test_lots_of_trailing_zeros() {
    // Pattern for the first test: 00111001100000000000000000000000
    check!(0.00024414062);
    check!(0.0024414062);
    check!(0.0043945312);
    check!(0.0063476562);
}

#[test]
fn test_regression() {
    assert_eq!(pretty(4.7223665e21), "4.7223665e+21");
    assert_eq!(pretty(8388608.0), "8388608");
    assert_eq!(pretty(16777216.0), "16777216");
    assert_eq!(pretty(33554436.0), "33554436");
    assert_eq!(pretty(67131496.0), "67131496");
    assert_eq!(pretty(1.9310392e-38), "1.9310392e-38");
    assert_eq!(pretty(-2.47e-43), "-2.47e-43");
    assert_eq!(pretty(1.993244e-38), "1.993244e-38");
    assert_eq!(pretty(4103.9004), "4103.9004");
    assert_eq!(pretty(5339999700.0), "5339999700");
    assert_eq!(pretty(6.0898e-39), "6.0898e-39");
    assert_eq!(pretty(0.0010310042), "0.0010310042");
    assert_eq!(pretty(2.882326e17), "288232600000000000");
    assert_eq!(pretty(7.038531e-26), "7.038531e-26");
    assert_eq!(pretty(9.223404e17), "922340400000000000");
    assert_eq!(pretty(67108870.0), "67108870");
    assert_eq!(pretty(1e-44), "1e-44");
    assert_eq!(pretty(2.816025e14), "281602500000000");
    assert_eq!(pretty(9.223372e18), "9223372000000000000");
    assert_eq!(pretty(1.5846086e29), "1.5846086e+29");
    assert_eq!(pretty(1.1811161e19), "11811161000000000000");
    assert_eq!(pretty(5.368709e18), "5368709000000000000");
    assert_eq!(pretty(4.6143166e18), "4614316600000000000");
    assert_eq!(pretty(0.007812537), "0.007812537");
    assert_eq!(pretty(1e-45), "1e-45");
    assert_eq!(pretty(1.18697725e20), "118697725000000000000");
    assert_eq!(pretty(1.00014165e-36), "1.00014165e-36");
    assert_eq!(pretty(200.0), "200");
    assert_eq!(pretty(33554432.0), "33554432");
}

#[test]
fn test_looks_like_pow5() {
    // These numbers have a mantissa that is the largest power of 5 that fits,
    // and an exponent that causes the computation for q to result in 10, which
    // is a corner case for Ryū.
    assert_eq!(f32::from_bits(0x5D1502F9), 6.7108864e17);
    assert_eq!(pretty(6.7108864e17), "671088640000000000");

    assert_eq!(f32::from_bits(0x5D9502F9), 1.3421773e18);
    assert_eq!(pretty(1.3421773e18), "1342177300000000000");

    assert_eq!(f32::from_bits(0x5E1502F9), 2.6843546e18);
    assert_eq!(pretty(2.6843546e18), "2684354600000000000");
}

#[test]
fn test_output_length() {
    assert_eq!(pretty(1.0), "1"); // already tested in test_basic
    assert_eq!(pretty(1.2), "1.2");
    assert_eq!(pretty(1.23), "1.23");
    assert_eq!(pretty(1.234), "1.234");
    assert_eq!(pretty(1.2345), "1.2345");
    assert_eq!(pretty(1.23456), "1.23456");
    assert_eq!(pretty(1.234567), "1.234567");
    assert_eq!(pretty(1.2345678), "1.2345678");
    assert_eq!(pretty(1.23456735e-36), "1.23456735e-36");
}
