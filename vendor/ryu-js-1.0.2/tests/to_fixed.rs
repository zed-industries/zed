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

fn pretty_to_fixed(f: f64, exp: u8) -> String {
    ryu_js::Buffer::new().format_to_fixed(f, exp).to_owned()
}

fn pretty_to_string(f: f64) -> String {
    ryu_js::Buffer::new().format(f).to_owned()
}

#[test]
fn range_over_100() {
    let expected = "0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    assert_eq!(pretty_to_fixed(0.0, 101), expected);
}

#[test]
fn nan() {
    for fraction_digits in 0..=100u8 {
        assert_eq!(pretty_to_fixed(f64::NAN, fraction_digits), "NaN");
    }
}

#[test]
fn infinity() {
    for fraction_digits in 0..=100u8 {
        assert_eq!(pretty_to_fixed(f64::INFINITY, fraction_digits), "Infinity");
    }
    for fraction_digits in 0..=100u8 {
        assert_eq!(
            pretty_to_fixed(f64::NEG_INFINITY, fraction_digits),
            "-Infinity"
        );
    }
}

#[test]
fn positive_zero() {
    assert_eq!(pretty_to_fixed(0.0, 0), "0");
    for fraction_digits in 1..=100u8 {
        let expected = "0".repeat(usize::from(fraction_digits));
        assert_eq!(
            pretty_to_fixed(0.0, fraction_digits),
            format!("0.{expected}")
        );
    }
}

#[test]
fn negative_zero() {
    assert_eq!(pretty_to_fixed(-0.0, 0), "0");
    for fraction_digits in 1..=100u8 {
        let expected = "0".repeat(usize::from(fraction_digits));
        assert_eq!(
            pretty_to_fixed(-0.0, fraction_digits),
            format!("0.{expected}")
        );
    }
}

const WHOLE_NUMBERS: &[f64] = &[
    1.0,
    10.0,
    100.0,
    123.0,
    1234567890.0,
    i32::MAX as f64,
    12_345_678_910_111_213.0,
    9_007_199_254_740_992.0,
];

#[track_caller]
fn check_whole_number(test_case: usize, number: f64) {
    for fraction_digits in 0..=100u8 {
        let mut fraction = "0".repeat(usize::from(fraction_digits));
        if fraction_digits != 0 {
            fraction = format!(".{fraction}");
        }
        let expected = format!("{number}{fraction}");

        assert_eq!(
            pretty_to_fixed(number, fraction_digits),
            expected,
            "Test case {test_case}. expected {number} with fraction_digits {fraction_digits} to equal {expected}"
        );
    }
}

#[test]
fn test_positive_whole_numbers() {
    for (test_case, number) in WHOLE_NUMBERS.iter().copied().enumerate() {
        check_whole_number(test_case, number);
    }
}

#[test]
fn test_negative_whole_numbers() {
    for (test_case, number) in WHOLE_NUMBERS.iter().copied().map(|x| -x).enumerate() {
        check_whole_number(test_case, number);
    }
}

// https://github.com/boa-dev/boa/issues/2609
#[test]
fn boa_issue_2609() {
    assert_eq!(pretty_to_fixed(1.25, 1), "1.3");
    assert_eq!(pretty_to_fixed(1.35, 1), "1.4");
}

#[test]
fn test262() {
    // test262 commit: be0abd93cd799a758714b5707fa87c9048fc38ce

    // test/built-ins/Number/prototype/toFixed/S15.7.4.5_A1.1_T02.js
    assert_eq!(pretty_to_fixed(1.0, 0), "1");
    assert_eq!(pretty_to_fixed(1.0, 1), "1.0");

    // test/built-ins/Number/prototype/toFixed/S15.7.4.5_A1.4_T01.js
    assert_eq!(pretty_to_fixed(1e21, 1), pretty_to_string(1e21));

    // test/built-ins/Number/prototype/toFixed/exactness.js
    assert_eq!(
        pretty_to_fixed(1000000000000000128.0, 0),
        "1000000000000000128"
    );
}

#[test]
fn rounding() {
    assert_eq!(pretty_to_fixed(1.5, 0), "2");
    assert_eq!(pretty_to_fixed(2.9, 0), "3");

    assert_eq!(pretty_to_fixed(2.55, 1), "2.5");
    assert_eq!(pretty_to_fixed(2.449999999999999999, 1), "2.5");

    assert_eq!(pretty_to_fixed(1010.954526123, 9), "1010.954526123");
    assert_eq!(pretty_to_fixed(1010.954526123, 8), "1010.95452612");
    assert_eq!(pretty_to_fixed(1010.954526123, 7), "1010.9545261");
    assert_eq!(pretty_to_fixed(1010.954526123, 6), "1010.954526");
    assert_eq!(pretty_to_fixed(1010.954526123, 5), "1010.95453");
    assert_eq!(pretty_to_fixed(1010.954526123, 4), "1010.9545");
    assert_eq!(pretty_to_fixed(1010.954526123, 3), "1010.955");
    assert_eq!(pretty_to_fixed(1010.954526123, 2), "1010.95");
    assert_eq!(pretty_to_fixed(1010.954526123, 1), "1011.0");
    assert_eq!(pretty_to_fixed(1010.954526123, 0), "1011");
}

#[test]
fn test_to_fixed_fraction_digits_50() {
    assert_eq!(
        pretty_to_fixed(0.3, 50),
        "0.29999999999999998889776975374843459576368331909180"
    );
}

#[test]
fn test_to_fixed_fraction_digits_100() {
    assert_eq!(pretty_to_fixed(1.0, 100), "1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(pretty_to_fixed(1.256, 100), "1.2560000000000000053290705182007513940334320068359375000000000000000000000000000000000000000000000000");
    assert_eq!(pretty_to_fixed(1.12345678910111213, 100), "1.1234567891011122409139488809159956872463226318359375000000000000000000000000000000000000000000000000");
}

#[test]
#[rustfmt::skip]
fn test_exponential_notation() {
    assert_eq!(pretty_to_fixed(1.23, 2),    "1.23");
    assert_eq!(pretty_to_fixed(1.23e0, 2),  "1.23");
    assert_eq!(pretty_to_fixed(1.23e1, 2),  "12.30");
    assert_eq!(pretty_to_fixed(1.23e2, 2),  "123.00");
    assert_eq!(pretty_to_fixed(1.23e3, 2),  "1230.00");
    assert_eq!(pretty_to_fixed(1.23e4, 2),  "12300.00");
    assert_eq!(pretty_to_fixed(1.23e5, 2),  "123000.00");
    assert_eq!(pretty_to_fixed(1.23e6, 2),  "1230000.00");
    assert_eq!(pretty_to_fixed(1.23e7, 2),  "12300000.00");
    assert_eq!(pretty_to_fixed(1.23e8, 2),  "123000000.00");
    assert_eq!(pretty_to_fixed(1.23e9, 2),  "1230000000.00");
    assert_eq!(pretty_to_fixed(1.23e10, 2), "12300000000.00");
    assert_eq!(pretty_to_fixed(1.23e11, 2), "123000000000.00");
    assert_eq!(pretty_to_fixed(1.23e12, 2), "1230000000000.00");
    assert_eq!(pretty_to_fixed(1.23e13, 2), "12300000000000.00");
    assert_eq!(pretty_to_fixed(1.23e14, 2), "123000000000000.00");
    assert_eq!(pretty_to_fixed(1.23e15, 2), "1230000000000000.00");
    assert_eq!(pretty_to_fixed(1.23e16, 2), "12300000000000000.00");
    assert_eq!(pretty_to_fixed(1.23e17, 2), "123000000000000000.00");
    assert_eq!(pretty_to_fixed(1.23e18, 2), "1230000000000000000.00");
    assert_eq!(pretty_to_fixed(1.23e19, 2), "12300000000000000000.00");
    assert_eq!(pretty_to_fixed(1.23e20, 2), "123000000000000000000.00");

    // fallback to exponential notation
    assert_eq!(pretty_to_fixed(1.23e21, 2), "1.23e+21");
    assert_eq!(pretty_to_fixed(1.23e22, 2), "1.23e+22");
    assert_eq!(pretty_to_fixed(1.23e23, 2), "1.23e+23");
    assert_eq!(pretty_to_fixed(1.23e24, 2), "1.23e+24");
    assert_eq!(pretty_to_fixed(1.23e25, 2), "1.23e+25");
}

const DOUBLE_MANTISSA_BITS: u8 = 52;
const DOUBLE_BIAS: i32 = 1023;

fn f64_and_e2_from_parts(sign: bool, exponent: u16, mantissa: u64) -> (f64, i32) {
    assert!(exponent <= 0b111_1111_1111, "Invalid f64 exponent");

    let mut bits: u64 = 0;

    bits |= mantissa;
    bits |= (u64::from(exponent)) << 52;
    bits |= u64::from(sign) << (52 + 11);

    let e2 = if exponent == 0 {
        1 - DOUBLE_BIAS - i32::from(DOUBLE_MANTISSA_BITS)
    } else {
        i32::from(exponent) - DOUBLE_BIAS - i32::from(DOUBLE_MANTISSA_BITS)
    };

    (f64::from_bits(bits), e2)
}

fn f64_from_parts(sign: bool, exponent: u16, mantissa: u64) -> f64 {
    f64_and_e2_from_parts(sign, exponent, mantissa).0
}

#[test]
fn test_f64_from_parts() {
    assert_eq!(pretty_to_fixed(f64_from_parts(false, 0, 0), 2), "0.00");
    assert_eq!(pretty_to_fixed(f64_from_parts(true, 0, 0), 2), "0.00");
}

#[test]
fn test_max_exponent_boundry_zero_mantissa() {
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0000_0000, 0), 2),
        "2.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0011_1111, 0), 2),
        "18446744073709551616.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0000, 0), 2),
        "36893488147419103232.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0001, 0), 2),
        "73786976294838206464.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0010, 0), 2),
        "147573952589676412928.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0011, 0), 2),
        "295147905179352825856.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0100, 0), 2),
        "590295810358705651712.00"
    );

    // ToString fallback
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0101, 0), 2),
        "1.1805916207174113e+21"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0110, 0), 2),
        "2.3611832414348226e+21"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0111, 0), 2),
        "4.722366482869645e+21"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0111_1111, 0), 2),
        "3.402823669209385e+38"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_1111_1111, 0), 2),
        "1.157920892373162e+77"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b101_1111_1111, 0), 2),
        "1.3407807929942597e+154"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b111_1111_1111, 0), 2),
        "Infinity"
    );
}

#[test]
fn test_max_exponent_boundry_and_full_mantissa() {
    let m = !(u64::MAX << u64::from(DOUBLE_MANTISSA_BITS));

    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0000_0000, m), 2),
        "4.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0011_1111, m), 2),
        "36893488147419099136.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0000, m), 2),
        "73786976294838198272.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0001, m), 2),
        "147573952589676396544.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0010, m), 2),
        "295147905179352793088.00"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0011, m), 2),
        "590295810358705586176.00"
    );

    // ToString fallback
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0100, m), 2),
        "1.1805916207174112e+21"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b100_0100_0101, m), 2),
        "2.3611832414348223e+21"
    );
    assert_eq!(
        pretty_to_fixed(f64_from_parts(false, 0b111_1111_1111, m), 2),
        "NaN"
    );
}

const MIN_EXPONENT: u16 = 0b010_1001_0011;

#[test]
fn test_min_exponent_boundry_zero_mantissa() {
    let m = 0;

    let expected = "0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    for exponent in 0..=MIN_EXPONENT {
        assert_eq!(
            pretty_to_fixed(f64_from_parts(false, exponent, m), 100),
            expected
        );
    }

    assert_ne!(
        pretty_to_fixed(f64_from_parts(false, MIN_EXPONENT + 1, m), 100),
        expected
    );
}

#[test]
fn test_min_exponent_boundry_full_mantissa() {
    let m = !(u64::MAX << u64::from(DOUBLE_MANTISSA_BITS));

    let expected = "0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    for exponent in 0..=MIN_EXPONENT {
        assert_eq!(
            pretty_to_fixed(f64_from_parts(false, exponent, m), 100),
            expected
        );
    }

    assert_ne!(
        pretty_to_fixed(f64_from_parts(false, MIN_EXPONENT + 1, m), 100),
        expected
    );
}
