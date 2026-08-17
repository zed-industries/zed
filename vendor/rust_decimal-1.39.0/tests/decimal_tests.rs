mod macros;

use core::{cmp::Ordering::*, str::FromStr};
use num_traits::{Inv, Signed, ToPrimitive};
use rust_decimal::{Decimal, Error, RoundingStrategy};

#[test]
#[cfg(feature = "c-repr")]
fn layout_is_correct() {
    assert_eq!(std::mem::size_of::<Decimal>(), std::mem::size_of::<u128>());
}

#[test]
#[cfg(feature = "align16")]
fn align_is_correct() {
    assert_eq!(align_of::<Decimal>(), 16);
    assert_eq!(align_of::<Decimal>(), align_of::<u128>());
    assert_eq!(align_of::<Decimal>(), align_of::<i128>());
}

#[test]
fn it_can_extract_the_mantissa() {
    let tests = [
        ("1", 1i128, 0),
        ("1.123456", 1123456i128, 6),
        ("-0.123456", -123456i128, 6),
    ];
    for &(input, mantissa, scale) in &tests {
        let num = Decimal::from_str(input).unwrap();
        assert_eq!(num.mantissa(), mantissa, "Mantissa for {input}");
        assert_eq!(num.scale(), scale, "Scale for {input}");
    }
}

// Parsing

#[test]
fn it_creates_a_new_negative_decimal() {
    let a = Decimal::new(-100, 2);
    assert!(a.is_sign_negative());
    assert_eq!(a.scale(), 2);
    assert_eq!("-1.00", a.to_string());
}

#[test]
fn it_creates_a_new_decimal_using_numeric_boundaries() {
    let a = Decimal::new(i64::MAX, 2);
    assert!(!a.is_sign_negative());
    assert_eq!(a.scale(), 2);
    assert_eq!("92233720368547758.07", a.to_string());

    let b = Decimal::new(i64::MIN, 2);
    assert!(b.is_sign_negative());
    assert_eq!(b.scale(), 2);
    assert_eq!("-92233720368547758.08", b.to_string());
}

#[test]
fn it_parses_empty_string() {
    assert!(Decimal::from_str("").is_err());
    assert!(Decimal::from_str(" ").is_err());
}

#[test]
fn it_parses_positive_int_string() {
    let a = Decimal::from_str("233").unwrap();
    assert!(!a.is_sign_negative());
    assert_eq!(a.scale(), 0);
    assert_eq!("233", a.to_string());
}

#[test]
fn it_parses_negative_int_string() {
    let a = Decimal::from_str("-233").unwrap();
    assert!(a.is_sign_negative());
    assert_eq!(a.scale(), 0);
    assert_eq!("-233", a.to_string());
}

#[test]
fn it_parses_positive_float_string() {
    let a = Decimal::from_str("233.323223").unwrap();
    assert!(!a.is_sign_negative());
    assert_eq!(a.scale(), 6);
    assert_eq!("233.323223", a.to_string());
}

#[test]
fn it_parses_negative_float_string() {
    let a = Decimal::from_str("-233.43343").unwrap();
    assert!(a.is_sign_negative());
    assert_eq!(a.scale(), 5);
    assert_eq!("-233.43343", a.to_string());
}

#[test]
fn it_parses_positive_tiny_float_string() {
    let a = Decimal::from_str(".000001").unwrap();
    assert!(!a.is_sign_negative());
    assert_eq!(a.scale(), 6);
    assert_eq!("0.000001", a.to_string());
}

#[test]
fn it_parses_negative_tiny_float_string() {
    let a = Decimal::from_str("-0.000001").unwrap();
    assert!(a.is_sign_negative());
    assert_eq!(a.scale(), 6);
    assert_eq!("-0.000001", a.to_string());
}

#[test]
fn it_parses_big_integer_string() {
    let a = Decimal::from_str("79228162514264337593543950330").unwrap();
    assert_eq!("79228162514264337593543950330", a.to_string());
}

#[test]
fn it_parses_big_float_string() {
    let a = Decimal::from_str("79.228162514264337593543950330").unwrap();
    assert_eq!("79.228162514264337593543950330", a.to_string());
}

#[test]
fn it_can_serialize_deserialize() {
    let tests = [
        "12.3456789",
        "5233.9008808150288439427720175",
        "-5233.9008808150288439427720175",
    ];
    for test in &tests {
        let a = Decimal::from_str(test).unwrap();
        let bytes = a.serialize();
        let b = Decimal::deserialize(bytes);
        assert_eq!(test.to_string(), b.to_string());
    }
}

#[cfg(feature = "borsh")]
mod borsh_tests {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn it_can_serialize_deserialize_borsh() {
        let tests = [
            "12.3456789",
            "5233.9008808150288439427720175",
            "-5233.9008808150288439427720175",
        ];
        for test in &tests {
            let a = Decimal::from_str(test).unwrap();
            let mut bytes: Vec<u8> = Vec::new();
            borsh::BorshSerialize::serialize(&a, &mut bytes).unwrap();
            let b: Decimal = borsh::BorshDeserialize::deserialize(&mut bytes.as_slice()).unwrap();
            assert_eq!(test.to_string(), b.to_string());
            let bytes = borsh::try_to_vec_with_schema(&a);
            assert!(bytes.is_ok(), "try_to_vec_with_schema.is_ok()");
            let bytes = bytes.unwrap();
            let result = borsh::try_from_slice_with_schema(&bytes);
            assert!(result.is_ok(), "try_from_slice_with_schema.is_ok()");
            let b: Decimal = result.unwrap();
            assert_eq!(test.to_string(), b.to_string());
        }
    }
}

#[cfg(feature = "ndarray")]
mod ndarray_tests {
    use rust_decimal::Decimal;

    #[test]
    fn it_can_do_scalar_ops_in_ndarray() {
        use ndarray::Array1;
        use num_traits::FromPrimitive;

        let array_a = Array1::from(vec![
            Decimal::from_f32(1.0).unwrap(),
            Decimal::from_f32(2.0).unwrap(),
            Decimal::from_f32(3.0).unwrap(),
        ]);

        // Add
        let output = array_a.clone() + Decimal::from_f32(5.0).unwrap();
        let expectation = Array1::from(vec![
            Decimal::from_f32(6.0).unwrap(),
            Decimal::from_f32(7.0).unwrap(),
            Decimal::from_f32(8.0).unwrap(),
        ]);
        assert_eq!(output, expectation);

        // Sub
        let output = array_a.clone() - Decimal::from_f32(5.0).unwrap();
        let expectation = Array1::from(vec![
            Decimal::from_f32(-4.0).unwrap(),
            Decimal::from_f32(-3.0).unwrap(),
            Decimal::from_f32(-2.0).unwrap(),
        ]);
        assert_eq!(output, expectation);

        // Mul
        let output = array_a.clone() * Decimal::from_f32(5.0).unwrap();
        let expectation = Array1::from(vec![
            Decimal::from_f32(5.0).unwrap(),
            Decimal::from_f32(10.0).unwrap(),
            Decimal::from_f32(15.0).unwrap(),
        ]);
        assert_eq!(output, expectation);

        // Div
        let output = array_a / Decimal::from_f32(5.0).unwrap();
        let expectation = Array1::from(vec![
            Decimal::from_f32(0.2).unwrap(),
            Decimal::from_f32(0.4).unwrap(),
            Decimal::from_f32(0.6).unwrap(),
        ]);
        assert_eq!(output, expectation);
    }
}

#[cfg(feature = "rkyv")]
mod rkyv_tests {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn it_can_serialize_deserialize_rkyv() {
        use rkyv::Deserialize;
        let tests = [
            "12.3456789",
            "5233.9008808150288439427720175",
            "-5233.9008808150288439427720175",
        ];
        for test in &tests {
            let a = Decimal::from_str(test).unwrap();
            let bytes = rkyv::to_bytes::<_, 256>(&a).unwrap();

            #[cfg(feature = "rkyv-safe")]
            {
                let archived = rkyv::check_archived_root::<Decimal>(&bytes[..]).unwrap();
                assert_eq!(archived, &a);
            }

            let archived = unsafe { rkyv::archived_root::<Decimal>(&bytes[..]) };
            assert_eq!(archived, &a);

            let deserialized: Decimal = archived.deserialize(&mut rkyv::Infallible).unwrap();
            assert_eq!(deserialized, a);
        }
    }
}

#[test]
fn it_can_deserialize_unbounded_values() {
    // Mantissa for these: 19393111376951473493673267553
    let tests = [
        (
            [1u8, 0, 28, 206, 97, 81, 216, 182, 20, 30, 165, 78, 18, 155, 169, 62],
            // Scale 28: -1.9393111376951473493673267553
            "-1.9393111376951473493673267553",
        ),
        (
            [1u8, 0, 29, 206, 97, 81, 216, 182, 20, 30, 165, 78, 18, 155, 169, 62],
            // Scale 29: -0.19393111376951473493673267553
            "-0.1939311137695147349367326755",
        ),
        (
            [1u8, 0, 30, 206, 97, 81, 216, 182, 20, 30, 165, 78, 18, 155, 169, 62],
            // Scale 30: -0.019393111376951473493673267553
            "-0.0193931113769514734936732676",
        ),
        (
            [1u8, 0, 31, 206, 97, 81, 216, 182, 20, 30, 165, 78, 18, 155, 169, 62],
            // Scale 31: -0.0019393111376951473493673267553
            "-0.0019393111376951473493673268",
        ),
    ];
    for &(bytes, expected) in &tests {
        let dec = Decimal::deserialize(bytes);
        let string = format!("{dec:.9999}");
        let dec2 = Decimal::from_str(&string).unwrap();
        assert_eq!(dec, dec2);
        assert_eq!(dec.to_string(), expected, "dec.to_string()");
        assert_eq!(dec2.to_string(), expected, "dec2.to_string()");
    }
}

// Formatting

#[test]
fn it_formats() {
    let a = Decimal::from_str("233.323223").unwrap();
    assert_eq!(format!("{a}"), "233.323223");
    assert_eq!(format!("{a:.9}"), "233.323223000");
    assert_eq!(format!("{a:.0}"), "233");
    assert_eq!(format!("{a:.2}"), "233.32");
    assert_eq!(format!("{a:010.2}"), "0000233.32");
    assert_eq!(format!("{a:0<10.2}"), "233.320000");
}
#[test]
fn it_formats_neg() {
    let a = Decimal::from_str("-233.323223").unwrap();
    assert_eq!(format!("{a}"), "-233.323223");
    assert_eq!(format!("{a:.9}"), "-233.323223000");
    assert_eq!(format!("{a:.0}"), "-233");
    assert_eq!(format!("{a:.2}"), "-233.32");
    assert_eq!(format!("{a:010.2}"), "-000233.32");
    assert_eq!(format!("{a:0<10.2}"), "-233.32000");
}
#[test]
fn it_formats_small() {
    let a = Decimal::from_str("0.2223").unwrap();
    assert_eq!(format!("{a}"), "0.2223");
    assert_eq!(format!("{a:.9}"), "0.222300000");
    assert_eq!(format!("{a:.0}"), "0");
    assert_eq!(format!("{a:.2}"), "0.22");
    assert_eq!(format!("{a:010.2}"), "0000000.22");
    assert_eq!(format!("{a:0<10.2}"), "0.22000000");
}
#[test]
fn it_formats_small_leading_zeros() {
    let a = Decimal::from_str("0.0023554701772169").unwrap();
    assert_eq!(format!("{a}"), "0.0023554701772169");
    assert_eq!(format!("{a:.9}"), "0.002355470");
    assert_eq!(format!("{a:.0}"), "0");
    assert_eq!(format!("{a:.2}"), "0.00");
    assert_eq!(format!("{a:010.2}"), "0000000.00");
    assert_eq!(format!("{a:0<10.2}"), "0.00000000");
}
#[test]
fn it_formats_small_neg() {
    let a = Decimal::from_str("-0.2223").unwrap();
    assert_eq!(format!("{a}"), "-0.2223");
    assert_eq!(format!("{a:.9}"), "-0.222300000");
    assert_eq!(format!("{a:.0}"), "-0");
    assert_eq!(format!("{a:.2}"), "-0.22");
    assert_eq!(format!("{a:010.2}"), "-000000.22");
    assert_eq!(format!("{a:0<10.2}"), "-0.2200000");
}

#[test]
fn it_formats_zero() {
    let a = Decimal::from_str("0").unwrap();
    assert_eq!(format!("{a}"), "0");
    assert_eq!(format!("{a:.9}"), "0.000000000");
    assert_eq!(format!("{a:.0}"), "0");
    assert_eq!(format!("{a:.2}"), "0.00");
    assert_eq!(format!("{a:010.2}"), "0000000.00");
    assert_eq!(format!("{a:0<10.2}"), "0.00000000");
}

#[test]
fn it_formats_int() {
    let a = Decimal::from_str("5").unwrap();
    assert_eq!(format!("{a}"), "5");
    assert_eq!(format!("{a:.9}"), "5.000000000");
    assert_eq!(format!("{a:.0}"), "5");
    assert_eq!(format!("{a:.2}"), "5.00");
    assert_eq!(format!("{a:010.2}"), "0000005.00");
    assert_eq!(format!("{a:0<10.2}"), "5.00000000");
}

#[test]
fn it_formats_lower_exp() {
    let tests = [
        ("0.00001", "1e-5"),
        ("-0.00001", "-1e-5"),
        ("42.123", "4.2123e1"),
        ("-42.123", "-4.2123e1"),
        ("100", "1e2"),
    ];
    for (value, expected) in &tests {
        let a = Decimal::from_str(value).unwrap();
        assert_eq!(&format!("{a:e}"), *expected, "format!(\"{{:e}}\", {a})");
    }
}

#[test]
fn it_formats_lower_exp_padding() {
    let tests = [
        ("0.00001", "01e-5"),
        ("-0.00001", "-1e-5"),
        ("42.123", "4.2123e1"),
        ("-42.123", "-4.2123e1"),
        ("100", "001e2"),
    ];
    for (value, expected) in &tests {
        let a = Decimal::from_str(value).unwrap();
        assert_eq!(&format!("{a:05e}"), *expected, "format!(\"{{:05e}}\", {a})");
    }
}

#[test]
fn it_formats_scientific_precision() {
    for (num, scale, expected_no_precision, expected_precision) in [
        (
            123456,
            10,
            "1.23456e-5",
            [
                "1e-5",
                "1.2e-5",
                "1.23e-5",
                "1.234e-5",
                "1.2345e-5",
                "1.23456e-5",
                "1.234560e-5",
                "1.2345600e-5",
            ],
        ),
        (
            123456,
            0,
            "1.23456e5",
            [
                "1e5",
                "1.2e5",
                "1.23e5",
                "1.234e5",
                "1.2345e5",
                "1.23456e5",
                "1.234560e5",
                "1.2345600e5",
            ],
        ),
        (
            1,
            0,
            "1e0",
            [
                "1e0",
                "1.0e0",
                "1.00e0",
                "1.000e0",
                "1.0000e0",
                "1.00000e0",
                "1.000000e0",
                "1.0000000e0",
            ],
        ),
        (
            -123456,
            10,
            "-1.23456e-5",
            [
                "-1e-5",
                "-1.2e-5",
                "-1.23e-5",
                "-1.234e-5",
                "-1.2345e-5",
                "-1.23456e-5",
                "-1.234560e-5",
                "-1.2345600e-5",
            ],
        ),
        (
            -100000,
            10,
            "-1e-5",
            [
                "-1e-5",
                "-1.0e-5",
                "-1.00e-5",
                "-1.000e-5",
                "-1.0000e-5",
                "-1.00000e-5",
                "-1.000000e-5",
                "-1.0000000e-5",
            ],
        ),
    ] {
        assert_eq!(format!("{:e}", Decimal::new(num, scale)), expected_no_precision);
        for (i, precision) in expected_precision.iter().enumerate() {
            assert_eq!(&format!("{:.prec$e}", Decimal::new(num, scale), prec = i), precision);
        }
    }
}

// Negation
#[test]
fn it_negates_decimals() {
    fn neg(a: &str, b: &str) {
        let a = Decimal::from_str(a).unwrap();
        let result = -a;
        assert_eq!(b, result.to_string(), "- {a}");
    }

    let tests = &[
        ("1", "-1"),
        ("2", "-2"),
        ("2454495034", "-2454495034"),
        ("0.1", "-0.1"),
        ("11.815126050420168067226890757", "-11.815126050420168067226890757"),
    ];

    for &(a, b) in tests {
        neg(a, b);
        neg(b, a);
    }
}

// Addition

#[test]
fn it_can_add_simple() {
    // This is the most basic test for addition, intended largely for micro-optimization.
    let two = Decimal::ONE + Decimal::ONE;
    assert_eq!(two.to_u32(), Some(2));
}

#[test]
fn it_adds_decimals() {
    fn add(a: &str, b: &str, c: &str) {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        let result = a + b;
        assert_eq!(c, result.to_string(), "{a} + {b}");
        let result = b + a;
        assert_eq!(c, result.to_string(), "{b} + {a}");
    }

    let tests = &[
        ("0", "0", "0"),
        ("0", "-0", "0"),
        ("-0", "0", "0"),
        ("-0", "-0", "0"),
        ("2", "3", "5"),
        ("2454495034", "3451204593", "5905699627"),
        ("24544.95034", ".3451204593", "24545.2954604593"),
        (".1", ".1", "0.2"),
        (".10", ".1", "0.20"),
        (".1", "-.1", "0.0"),
        ("0", "1.001", "1.001"),
        ("2", "-3", "-1"),
        ("-2", "3", "1"),
        ("-2", "-3", "-5"),
        ("3", "-2", "1"),
        ("-3", "2", "-1"),
        ("1.234", "2.4567", "3.6907"),
        (
            "11.815126050420168067226890757",
            "0.6386554621848739495798319328",
            "12.453781512605042016806722690",
        ),
        (
            "-11.815126050420168067226890757",
            "0.6386554621848739495798319328",
            "-11.176470588235294117647058824",
        ),
        (
            "11.815126050420168067226890757",
            "-0.6386554621848739495798319328",
            "11.176470588235294117647058824",
        ),
        (
            "-11.815126050420168067226890757",
            "-0.6386554621848739495798319328",
            "-12.453781512605042016806722690",
        ),
        (
            "11815126050420168067226890757",
            "0.4386554621848739495798319328",
            "11815126050420168067226890757",
        ),
        (
            "-11815126050420168067226890757",
            "0.4386554621848739495798319328",
            "-11815126050420168067226890757",
        ),
        (
            "11815126050420168067226890757",
            "-0.4386554621848739495798319328",
            "11815126050420168067226890757",
        ),
        (
            "-11815126050420168067226890757",
            "-0.4386554621848739495798319328",
            "-11815126050420168067226890757",
        ),
        (
            "0.0872727272727272727272727272",
            "843.65000000",
            "843.7372727272727272727272727",
        ),
        (
            "7314.6229858868828353570724702",
            "1000",
            // Overflow causes this to round
            "8314.622985886882835357072470",
        ),
        (
            "108053.27500000000000000000000",
            "0.00000000000000000000000",
            "108053.27500000000000000000000",
        ),
        (
            "108053.27500000000000000000000",
            // This zero value has too high precision and will be trimmed
            "0.000000000000000000000000",
            "108053.27500000000000000000000",
        ),
        (
            "108053.27500000000000000000000",
            // This value has too high precision and will be rounded
            "0.000000000000000000000001",
            "108053.27500000000000000000000",
        ),
        (
            "108053.27500000000000000000000",
            // This value has too high precision and will be rounded
            "0.000000000000000000000005",
            either!("108053.27500000000000000000000", "108053.27500000000000000000001"),
        ),
        (
            "8097370036018690744.2590371109596744091",
            "3807285637671831400.15346897797550749555",
            "11904655673690522144.412506089",
        ),
    ];
    for &(a, b, c) in tests {
        add(a, b, c);
    }
}

#[test]
fn it_can_addassign() {
    let mut a = Decimal::from_str("1.01").unwrap();
    let b = Decimal::from_str("0.99").unwrap();
    a += b;
    assert_eq!("2.00", a.to_string());

    a += &b;
    assert_eq!("2.99", a.to_string());

    let mut c = &mut a;
    c += b;
    assert_eq!("3.98", a.to_string());

    let mut c = &mut a;
    c += &b;
    assert_eq!("4.97", a.to_string());
}

// Subtraction

#[test]
fn it_subtracts_decimals() {
    fn sub(a: &str, b: &str, c: &str) {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        let result = a - b;
        assert_eq!(c, result.to_string(), "{a} - {b}");
    }

    let tests = &[
        ("0", "0", "0"),
        ("0", "-0", "0"),
        ("-0", "0", "0"),
        ("-0", "-0", "0"),
        ("2", "3", "-1"),
        ("3451204593", "2323322332", "1127882261"),
        ("24544.95034", ".3451204593", "24544.6052195407"),
        (".1", ".1", "0.0"),
        (".1", "-.1", "0.2"),
        ("1.001", "0", "1.001"),
        ("2", "-3", "5"),
        ("-2", "3", "-5"),
        ("-2", "-3", "1"),
        ("3", "-2", "5"),
        ("-3", "2", "-5"),
        ("1.234", "2.4567", "-1.2227"),
        ("844.13000000", "843.65000000", "0.48000000"),
        ("79228162514264337593543950335", "79228162514264337593543950335", "0"), // 0xFFFF_FFFF_FFFF_FFFF_FFF_FFFF - 0xFFFF_FFFF_FFFF_FFFF_FFF_FFFF
        ("79228162514264337593543950335", "0", "79228162514264337593543950335"),
        ("79228162514264337593543950335", "79228162514264337593543950333", "2"),
        ("4951760157141521099596496896", "1", "4951760157141521099596496895"), // 0x1000_0000_0000_0000_0000_0000 - 1 = 0x0FFF_FFFF_FFFF_FFFF_FFF_FFFF
        ("79228162514264337593543950334", "79228162514264337593543950335", "-1"),
        ("1", "4951760157141521099596496895", "-4951760157141521099596496894"),
        ("18446744073709551615", "-18446744073709551615", "36893488147419103230"), // 0xFFFF_FFFF_FFFF_FFFF - -0xFFFF_FFFF_FFFF_FFFF
    ];
    for &(a, b, c) in tests {
        sub(a, b, c);
    }
}

#[test]
fn it_can_subassign() {
    let mut a = Decimal::from_str("1.01").unwrap();
    let b = Decimal::from_str("0.51").unwrap();
    a -= b;
    assert_eq!("0.50", a.to_string());

    a -= &b;
    assert_eq!("-0.01", a.to_string());

    let mut c = &mut a;
    c -= b;
    assert_eq!("-0.52", a.to_string());

    let mut c = &mut a;
    c -= &b;
    assert_eq!("-1.03", a.to_string());
}

// Multiplication

#[test]
fn it_multiplies_decimals() {
    fn mul(a: &str, b: &str, c: &str) {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        let result = a * b;
        assert_eq!(c, result.to_string(), "{a} * {b}");
        let result = b * a;
        assert_eq!(c, result.to_string(), "{b} * {a}");
    }

    let tests = &[
        ("2", "3", "6"),
        ("2454495034", "3451204593", "8470964534836491162"),
        ("24544.95034", ".3451204593", "8470.964534836491162"),
        (".1", ".1", "0.01"),
        ("0", "1.001", "0"),
        ("2", "-3", "-6"),
        ("-2", "3", "-6"),
        ("-2", "-3", "6"),
        ("1", "2.01", "2.01"),
        ("1.0", "2.01", "2.010"), // Scale is always additive
        (
            "0.00000000000000001",
            "0.00000000000000001",
            "0.0000000000000000000000000000",
        ),
        ("0.0000000000000000000000000001", "0.0000000000000000000000000001", "0"),
        (
            "0.6386554621848739495798319328",
            "11.815126050420168067226890757",
            "7.5457947885036367488171739292",
        ),
        (
            "2123456789012345678901234567.8",
            "11.815126050420168067226890757",
            "25088909624801327937270048761",
        ),
        (
            "2123456789012345678901234567.8",
            "-11.815126050420168067226890757",
            "-25088909624801327937270048761",
        ),
        (
            "2.1234567890123456789012345678",
            "2.1234567890123456789012345678",
            "4.5090687348026215523554336227",
        ),
        (
            "0.48000000",
            "0.1818181818181818181818181818",
            either!("0.0872727272727272727272727273", "0.0872727272727272727272727272"),
        ),
    ];
    for &(a, b, c) in tests {
        mul(a, b, c);
    }
}

#[test]
#[should_panic(expected = "Multiplication overflowed")]
fn it_panics_when_multiply_with_overflow() {
    let a = Decimal::from_str("2000000000000000000001").unwrap();
    let b = Decimal::from_str("3000000000000000000001").unwrap();
    let _ = a * b;
}

#[test]
fn it_can_mulassign() {
    let mut a = Decimal::from_str("1.25").unwrap();
    let b = Decimal::from_str("0.01").unwrap();

    a *= b;
    assert_eq!("0.0125", a.to_string());

    a *= &b;
    assert_eq!("0.000125", a.to_string());

    let mut c = &mut a;
    c *= b;
    assert_eq!("0.00000125", a.to_string());

    let mut c = &mut a;
    c *= &b;
    assert_eq!("0.0000000125", a.to_string());
}

// Division

#[test]
fn it_divides_decimals() {
    fn div(a: &str, b: &str, c: &str) {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        let result = a / b;
        assert_eq!(c, result.to_string(), "{a} / {b}");
    }

    let tests = &[
        ("6", "3", "2"),
        ("10", "2", "5"),
        ("2.2", "1.1", "2"),
        ("-2.2", "-1.1", "2"),
        ("12.88", "5.6", "2.3"),
        (
            "1023427554493",
            "43432632",
            either!("23563.562864276795382789603909", "23563.562864276795382789603908"),
        ),
        ("10000", "3", "3333.3333333333333333333333333"),
        ("2", "3", "0.6666666666666666666666666667"),
        ("1", "3", "0.3333333333333333333333333333"),
        ("-2", "3", "-0.6666666666666666666666666667"),
        ("2", "-3", "-0.6666666666666666666666666667"),
        ("-2", "-3", "0.6666666666666666666666666667"),
        (
            "1234.5678",
            "0.1234567890123456",
            either!("9999.99926999999982999953127", "9999.999269999999829999531269"),
        ),
        ("1234.567890123456789012345678", "1.234567890123456789012345678", "1000"),
        (
            "32.91625929004387114334488",
            "3.27629537",
            either!("10.046792359274942644546996384", "10.046792359274942644546996383"),
        ),
        ("5000", "1000.26957490549", "4.9986524887277738570721416846"),
        ("6142.6941216127122745222131114", "2", "3071.3470608063561372611065557"),
        (
            "3071.3470608063561372611065557",
            "1228.87000756",
            either!("2.4993262443638869285360708423", "2.4993262443638869285360708422"),
        ),
        (
            "590.3274854004009467754255123",
            "53.68997202826239",
            "10.995116277759516850521689988",
        ),
    ];
    for &(a, b, c) in tests {
        div(a, b, c);
    }
}

#[test]
#[should_panic(expected = "Division by zero")]
fn it_can_divide_by_zero() {
    let a = Decimal::from_str("2").unwrap();
    let _ = a / Decimal::ZERO;
}

#[test]
fn it_can_divassign() {
    let mut a = Decimal::from_str("1.25").unwrap();
    let b = Decimal::from_str("0.01").unwrap();

    a /= b;
    assert_eq!("125", a.to_string());

    a /= &b;
    assert_eq!("12500", a.to_string());

    let mut c = &mut a;
    c /= b;
    assert_eq!("1250000", a.to_string());

    let mut c = &mut a;
    c /= &b;
    assert_eq!("125000000", a.to_string());
}

// Modulus and Remainder are not the same thing!
// https://math.stackexchange.com/q/801962/82277

#[test]
fn it_rems_decimals() {
    fn rem(a: &str, b: &str, c: &str) {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        // a = qb + r
        let result = a % b;
        assert_eq!(c, result.to_string(), "{a} % {b}");
    }

    let tests = &[
        ("2", "3", "2"),
        ("-2", "3", "-2"),
        ("2", "-3", "2"),
        ("-2", "-3", "-2"),
        ("6", "3", "0"),
        ("42.2", "11.9", "6.5"),
        ("2.1", "3", "2.1"),
        ("2", "3.1", "2"),
        ("2.0", "3.1", "2.0"),
        ("4", "3.1", "0.9"),
        ("2", "2", "0"),
        ("2", "-2", "0"),
        // Legacy keeps sign from lhs operand
        ("-2", "2", "0"),
        ("-2", "-2", "0"),
    ];
    for &(a, b, c) in tests {
        rem(a, b, c);
    }
}

#[test]
fn it_can_remassign() {
    let mut a = Decimal::from_str("5").unwrap();
    let b = Decimal::from_str("2").unwrap();

    a %= b;
    assert_eq!("1", a.to_string());

    a %= &b;
    assert_eq!("1", a.to_string());

    let mut c = &mut a;
    c %= b;
    assert_eq!("1", a.to_string());

    let mut c = &mut a;
    c %= &b;
    assert_eq!("1", a.to_string());
}

#[test]
fn it_eqs_decimals() {
    fn eq(a: &str, b: &str, c: bool) {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        assert_eq!(c, a.eq(&b), "{a} == {b}");
        assert_eq!(c, b.eq(&a), "{b} == {a}");
    }

    let tests = &[
        ("1", "1", true),
        ("1", "-1", false),
        ("1", "1.00", true),
        ("1.2345000000000", "1.2345", true),
        ("1.0000000000000000000000000000", "1.0000000000000000000000000000", true),
        (
            "1.0000000000000000000000000001",
            "1.0000000000000000000000000000",
            false,
        ),
    ];
    for &(a, b, c) in tests {
        eq(a, b, c);
    }
}

#[test]
fn it_cmps_decimals() {
    fn cmp(a: &str, b: &str, c: core::cmp::Ordering) {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        assert_eq!(
            c,
            a.cmp(&b),
            "{} {} {}",
            a,
            match c {
                Less => "<",
                Equal => "==",
                Greater => ">",
            },
            b
        );
    }

    let tests = &[
        ("1", "1", Equal),
        ("1", "-1", Greater),
        ("1", "1.00", Equal),
        ("1.2345000000000", "1.2345", Equal),
        (
            "1.0000000000000000000000000001",
            "1.0000000000000000000000000000",
            Greater,
        ),
        ("1.0000000000000000000000000000", "1.0000000000000000000000000001", Less),
        ("-1", "100", Less),
        ("-100", "1", Less),
        ("0", "0.5", Less),
        ("0.5", "0", Greater),
        ("100", "0.0098", Greater),
        ("1000000000000000", "999000000000000.0001", Greater),
        ("2.0001", "2.0001", Equal),
        (
            "11.815126050420168067226890757",
            "0.6386554621848739495798319328",
            Greater,
        ),
        ("0.6386554621848739495798319328", "11.815126050420168067226890757", Less),
        ("-0.5", "-0.01", Less),
        ("-0.5", "-0.1", Less),
        ("-0.01", "-0.5", Greater),
        ("-0.1", "-0.5", Greater),
        // 000 equality
        ("0.00000000", "0.00000000", Equal),
        // 000 000 same scale
        ("0.00000000", "0.00000000", Equal),
        ("-0.00000000", "0.00000000", Equal),
        ("0.00000000", "-0.00000000", Equal),
        ("-0.00000000", "-0.00000000", Equal),
        // 000 000 different scale
        ("0.000000000", "0.00000000000000000000000", Equal),
        ("-0.000000000", "0.00000000000000000000000", Equal),
        ("0.000000000", "-0.00000000000000000000000", Equal),
        ("-0.000000000", "-0.00000000000000000000000", Equal),
        // 000 100 same scale
        ("0.00000000", "6.56792910", Less),
        ("-0.00000000", "6.56792910", Less),
        ("0.00000000", "-6.56792910", Greater),
        ("-0.00000000", "-6.56792910", Greater),
        // 000 100 different scale
        ("0.0000000000000000000", "0.00000000000000001916236746", Less),
        ("-0.0000000000000000000", "0.00000000000000001916236746", Less),
        ("0.0000000000000000000", "-0.00000000000000001916236746", Greater),
        ("-0.0000000000000000000", "-0.00000000000000001916236746", Greater),
        // 000 010 same scale
        ("0.00000000", "49037796231.72571136", Less),
        ("-0.00000000", "49037796231.72571136", Less),
        ("0.00000000", "-49037796231.72571136", Greater),
        ("-0.00000000", "-49037796231.72571136", Greater),
        // 000 010 different scale
        ("0", "14459264155.12895488", Less),
        ("-0", "14459264155.12895488", Less),
        ("0", "-14459264155.12895488", Greater),
        ("-0", "-14459264155.12895488", Greater),
        // 000 110 same scale
        ("0.00000000", "38675108055.09052783", Less),
        ("-0.00000000", "38675108055.09052783", Less),
        ("0.00000000", "-38675108055.09052783", Greater),
        ("-0.00000000", "-38675108055.09052783", Greater),
        // 000 110 different scale
        ("0.00", "1495767034080324868", Less),
        ("-0.00", "1495767034080324868", Less),
        ("0.00", "-1495767034080324868", Greater),
        ("-0.00", "-1495767034080324868", Greater),
        // 000 001 same scale
        ("0.00000000", "359299289270893106016.81305600", Less),
        ("-0.00000000", "359299289270893106016.81305600", Less),
        ("0.00000000", "-359299289270893106016.81305600", Greater),
        ("-0.00000000", "-359299289270893106016.81305600", Greater),
        // 000 001 different scale
        ("0.00000000000000000000000000", "261631091689.9518486763536384", Less),
        ("-0.00000000000000000000000000", "261631091689.9518486763536384", Less),
        (
            "0.00000000000000000000000000",
            "-261631091689.9518486763536384",
            Greater,
        ),
        (
            "-0.00000000000000000000000000",
            "-261631091689.9518486763536384",
            Greater,
        ),
        // 000 101 same scale
        ("0.00000000", "184137107696737410476.63166815", Less),
        ("-0.00000000", "184137107696737410476.63166815", Less),
        ("0.00000000", "-184137107696737410476.63166815", Greater),
        ("-0.00000000", "-184137107696737410476.63166815", Greater),
        // 000 101 different scale
        ("0.000000000", "2286857871088.7514840434334478", Less),
        ("-0.000000000", "2286857871088.7514840434334478", Less),
        ("0.000000000", "-2286857871088.7514840434334478", Greater),
        ("-0.000000000", "-2286857871088.7514840434334478", Greater),
        // 000 011 same scale
        ("0.00000000", "169194696640288819908.07715840", Less),
        ("-0.00000000", "169194696640288819908.07715840", Less),
        ("0.00000000", "-169194696640288819908.07715840", Greater),
        ("-0.00000000", "-169194696640288819908.07715840", Greater),
        // 000 011 different scale
        ("0.00000000", "2757550691.7650076909569048576", Less),
        ("-0.00000000", "2757550691.7650076909569048576", Less),
        ("0.00000000", "-2757550691.7650076909569048576", Greater),
        ("-0.00000000", "-2757550691.7650076909569048576", Greater),
        // 000 111 same scale
        ("0.00000000", "133610725292915899001.10059212", Less),
        ("-0.00000000", "133610725292915899001.10059212", Less),
        ("0.00000000", "-133610725292915899001.10059212", Greater),
        ("-0.00000000", "-133610725292915899001.10059212", Greater),
        // 000 111 different scale
        ("0.00000000000000000000", "86.25432767926620368822165265", Less),
        ("-0.00000000000000000000", "86.25432767926620368822165265", Less),
        ("0.00000000000000000000", "-86.25432767926620368822165265", Greater),
        ("-0.00000000000000000000", "-86.25432767926620368822165265", Greater),
        // 100 equality
        ("0.0000000000598992228", "0.0000000000598992228", Equal),
        // 100 000 same scale
        ("0.0000000000598992228", "0.0000000000000000000", Greater),
        ("-0.0000000000598992228", "0.0000000000000000000", Less),
        ("0.0000000000598992228", "-0.0000000000000000000", Greater),
        ("-0.0000000000598992228", "-0.0000000000000000000", Less),
        // 100 000 different scale
        ("0.1797407597", "0.0000000000000000000", Greater),
        ("-0.1797407597", "0.0000000000000000000", Less),
        ("0.1797407597", "-0.0000000000000000000", Greater),
        ("-0.1797407597", "-0.0000000000000000000", Less),
        // 100 100 same scale
        ("0.0000000000598992228", "0.0000000000064510789", Greater),
        ("-0.0000000000598992228", "0.0000000000064510789", Less),
        ("0.0000000000598992228", "-0.0000000000064510789", Greater),
        ("-0.0000000000598992228", "-0.0000000000064510789", Less),
        // 100 100 different scale
        ("0.000000000000011217354", "0.0000000000217735186", Less),
        ("-0.000000000000011217354", "0.0000000000217735186", Less),
        ("0.000000000000011217354", "-0.0000000000217735186", Greater),
        ("-0.000000000000011217354", "-0.0000000000217735186", Greater),
        // 100 010 same scale
        ("0.0000000000598992228", "0.0659116848159129600", Less),
        ("-0.0000000000598992228", "0.0659116848159129600", Less),
        ("0.0000000000598992228", "-0.0659116848159129600", Greater),
        ("-0.0000000000598992228", "-0.0659116848159129600", Greater),
        // 100 010 different scale
        ("0.00042035421", "0.004709460588143575040", Less),
        ("-0.00042035421", "0.004709460588143575040", Less),
        ("0.00042035421", "-0.004709460588143575040", Greater),
        ("-0.00042035421", "-0.004709460588143575040", Greater),
        // 100 110 same scale
        ("0.0000000000598992228", "0.0755686585127091375", Less),
        ("-0.0000000000598992228", "0.0755686585127091375", Less),
        ("0.0000000000598992228", "-0.0755686585127091375", Greater),
        ("-0.0000000000598992228", "-0.0755686585127091375", Greater),
        // 100 110 different scale
        ("14872.94465", "1284707831905.854085", Less),
        ("-14872.94465", "1284707831905.854085", Less),
        ("14872.94465", "-1284707831905.854085", Greater),
        ("-14872.94465", "-1284707831905.854085", Greater),
        // 100 001 same scale
        ("0.0000000000598992228", "888767595.6145376468836286464", Less),
        ("-0.0000000000598992228", "888767595.6145376468836286464", Less),
        ("0.0000000000598992228", "-888767595.6145376468836286464", Greater),
        ("-0.0000000000598992228", "-888767595.6145376468836286464", Greater),
        // 100 001 different scale
        ("0.0002108155975", "36.07555527243968476014968832", Less),
        ("-0.0002108155975", "36.07555527243968476014968832", Less),
        ("0.0002108155975", "-36.07555527243968476014968832", Greater),
        ("-0.0002108155975", "-36.07555527243968476014968832", Greater),
        // 100 101 same scale
        ("0.0000000000598992228", "125730345.7412344676309569911", Less),
        ("-0.0000000000598992228", "125730345.7412344676309569911", Less),
        ("0.0000000000598992228", "-125730345.7412344676309569911", Greater),
        ("-0.0000000000598992228", "-125730345.7412344676309569911", Greater),
        // 100 101 different scale
        ("0.0000000000871576741", "1.7283925558865766140690239662", Less),
        ("-0.0000000000871576741", "1.7283925558865766140690239662", Less),
        ("0.0000000000871576741", "-1.7283925558865766140690239662", Greater),
        ("-0.0000000000871576741", "-1.7283925558865766140690239662", Greater),
        // 100 011 same scale
        ("0.0000000000598992228", "645513262.9193254090737451008", Less),
        ("-0.0000000000598992228", "645513262.9193254090737451008", Less),
        ("0.0000000000598992228", "-645513262.9193254090737451008", Greater),
        ("-0.0000000000598992228", "-645513262.9193254090737451008", Greater),
        // 100 011 different scale
        ("0.000000000000000760885021", "3718370638.2004059326675681280", Less),
        ("-0.000000000000000760885021", "3718370638.2004059326675681280", Less),
        ("0.000000000000000760885021", "-3718370638.2004059326675681280", Greater),
        (
            "-0.000000000000000760885021",
            "-3718370638.2004059326675681280",
            Greater,
        ),
        // 100 111 same scale
        ("0.0000000000598992228", "422482675.5515775479939306437", Less),
        ("-0.0000000000598992228", "422482675.5515775479939306437", Less),
        ("0.0000000000598992228", "-422482675.5515775479939306437", Greater),
        ("-0.0000000000598992228", "-422482675.5515775479939306437", Greater),
        // 100 111 different scale
        ("0.000000044182898", "25.452953262109919998605674045", Less),
        ("-0.000000044182898", "25.452953262109919998605674045", Less),
        ("0.000000044182898", "-25.452953262109919998605674045", Greater),
        ("-0.000000044182898", "-25.452953262109919998605674045", Greater),
        // 010 equality
        ("423.1744746042687488", "423.1744746042687488", Equal),
        // 010 000 same scale
        ("423.1744746042687488", "0.0000000000000000", Greater),
        ("-423.1744746042687488", "0.0000000000000000", Less),
        ("423.1744746042687488", "-0.0000000000000000", Greater),
        ("-423.1744746042687488", "-0.0000000000000000", Less),
        // 010 000 different scale
        ("9002354991.192604672", "0.00000000000000000000000000", Greater),
        ("-9002354991.192604672", "0.00000000000000000000000000", Less),
        ("9002354991.192604672", "-0.00000000000000000000000000", Greater),
        ("-9002354991.192604672", "-0.00000000000000000000000000", Less),
        // 010 100 same scale
        ("423.1744746042687488", "0.0000000981820809", Greater),
        ("-423.1744746042687488", "0.0000000981820809", Less),
        ("423.1744746042687488", "-0.0000000981820809", Greater),
        ("-423.1744746042687488", "-0.0000000981820809", Less),
        // 010 100 different scale
        ("4327019125101559.808", "0.00484846050", Greater),
        ("-4327019125101559.808", "0.00484846050", Less),
        ("4327019125101559.808", "-0.00484846050", Greater),
        ("-4327019125101559.808", "-0.00484846050", Less),
        // 010 010 same scale
        ("423.1744746042687488", "786.9082854590775296", Less),
        ("-423.1744746042687488", "786.9082854590775296", Less),
        ("423.1744746042687488", "-786.9082854590775296", Greater),
        ("-423.1744746042687488", "-786.9082854590775296", Greater),
        // 010 010 different scale
        ("793.0067125291450368", "0.0001587297248335626240", Greater),
        ("-793.0067125291450368", "0.0001587297248335626240", Less),
        ("793.0067125291450368", "-0.0001587297248335626240", Greater),
        ("-793.0067125291450368", "-0.0001587297248335626240", Less),
        // 010 110 same scale
        ("423.1744746042687488", "300.0541360230572049", Greater),
        ("-423.1744746042687488", "300.0541360230572049", Less),
        ("423.1744746042687488", "-300.0541360230572049", Greater),
        ("-423.1744746042687488", "-300.0541360230572049", Less),
        // 010 110 different scale
        ("90627.2042582540288", "4472414566654924.741", Less),
        ("-90627.2042582540288", "4472414566654924.741", Less),
        ("90627.2042582540288", "-4472414566654924.741", Greater),
        ("-90627.2042582540288", "-4472414566654924.741", Greater),
        // 010 001 same scale
        ("423.1744746042687488", "3960577151543.5796707636412416", Less),
        ("-423.1744746042687488", "3960577151543.5796707636412416", Less),
        ("423.1744746042687488", "-3960577151543.5796707636412416", Greater),
        ("-423.1744746042687488", "-3960577151543.5796707636412416", Greater),
        // 010 001 different scale
        ("0.000008867461591822499840", "185286.04378228713249986052096", Less),
        ("-0.000008867461591822499840", "185286.04378228713249986052096", Less),
        ("0.000008867461591822499840", "-185286.04378228713249986052096", Greater),
        (
            "-0.000008867461591822499840",
            "-185286.04378228713249986052096",
            Greater,
        ),
        // 010 101 same scale
        ("423.1744746042687488", "2825958416017.6213507229869501", Less),
        ("-423.1744746042687488", "2825958416017.6213507229869501", Less),
        ("423.1744746042687488", "-2825958416017.6213507229869501", Greater),
        ("-423.1744746042687488", "-2825958416017.6213507229869501", Greater),
        // 010 101 different scale
        ("0.01901870767742648320", "37662082383021542232.529651212", Less),
        ("-0.01901870767742648320", "37662082383021542232.529651212", Less),
        ("0.01901870767742648320", "-37662082383021542232.529651212", Greater),
        ("-0.01901870767742648320", "-37662082383021542232.529651212", Greater),
        // 010 011 same scale
        ("423.1744746042687488", "3628063966991.6759417059016704", Less),
        ("-423.1744746042687488", "3628063966991.6759417059016704", Less),
        ("423.1744746042687488", "-3628063966991.6759417059016704", Greater),
        ("-423.1744746042687488", "-3628063966991.6759417059016704", Greater),
        // 010 011 different scale
        ("45359904.32470925312", "2.4203452488052342918570049536", Greater),
        ("-45359904.32470925312", "2.4203452488052342918570049536", Less),
        ("45359904.32470925312", "-2.4203452488052342918570049536", Greater),
        ("-45359904.32470925312", "-2.4203452488052342918570049536", Less),
        // 010 111 same scale
        ("423.1744746042687488", "2629665172331.9610693820109120", Less),
        ("-423.1744746042687488", "2629665172331.9610693820109120", Less),
        ("423.1744746042687488", "-2629665172331.9610693820109120", Greater),
        ("-423.1744746042687488", "-2629665172331.9610693820109120", Greater),
        // 010 111 different scale
        ("0.0006420803252266205184", "8172.032417576265900588945489", Less),
        ("-0.0006420803252266205184", "8172.032417576265900588945489", Less),
        ("0.0006420803252266205184", "-8172.032417576265900588945489", Greater),
        ("-0.0006420803252266205184", "-8172.032417576265900588945489", Greater),
        // 110 equality
        ("844.5530620517286511", "844.5530620517286511", Equal),
        // 110 000 same scale
        ("844.5530620517286511", "0.0000000000000000", Greater),
        ("-844.5530620517286511", "0.0000000000000000", Less),
        ("844.5530620517286511", "-0.0000000000000000", Greater),
        ("-844.5530620517286511", "-0.0000000000000000", Less),
        // 110 000 different scale
        ("3285.530033386074797", "0.00000000000000000000", Greater),
        ("-3285.530033386074797", "0.00000000000000000000", Less),
        ("3285.530033386074797", "-0.00000000000000000000", Greater),
        ("-3285.530033386074797", "-0.00000000000000000000", Less),
        // 110 100 same scale
        ("844.5530620517286511", "0.0000001953470063", Greater),
        ("-844.5530620517286511", "0.0000001953470063", Less),
        ("844.5530620517286511", "-0.0000001953470063", Greater),
        ("-844.5530620517286511", "-0.0000001953470063", Less),
        // 110 100 different scale
        ("371284.0210972493371", "0.000000000000001307794657", Greater),
        ("-371284.0210972493371", "0.000000000000001307794657", Less),
        ("371284.0210972493371", "-0.000000000000001307794657", Greater),
        ("-371284.0210972493371", "-0.000000000000001307794657", Less),
        // 110 010 same scale
        ("844.5530620517286511", "612.1542773033140224", Greater),
        ("-844.5530620517286511", "612.1542773033140224", Less),
        ("844.5530620517286511", "-612.1542773033140224", Greater),
        ("-844.5530620517286511", "-612.1542773033140224", Less),
        // 110 010 different scale
        ("0.00004869219159821525572", "23341676485159.15776", Less),
        ("-0.00004869219159821525572", "23341676485159.15776", Less),
        ("0.00004869219159821525572", "-23341676485159.15776", Greater),
        ("-0.00004869219159821525572", "-23341676485159.15776", Greater),
        // 110 110 same scale
        ("844.5530620517286511", "326.6132818317622015", Greater),
        ("-844.5530620517286511", "326.6132818317622015", Less),
        ("844.5530620517286511", "-326.6132818317622015", Greater),
        ("-844.5530620517286511", "-326.6132818317622015", Less),
        // 110 110 different scale
        ("4752139369958.820619", "1330851022882027.972", Less),
        ("-4752139369958.820619", "1330851022882027.972", Less),
        ("4752139369958.820619", "-1330851022882027.972", Greater),
        ("-4752139369958.820619", "-1330851022882027.972", Greater),
        // 110 001 same scale
        ("844.5530620517286511", "3585610241942.1922435648192512", Less),
        ("-844.5530620517286511", "3585610241942.1922435648192512", Less),
        ("844.5530620517286511", "-3585610241942.1922435648192512", Greater),
        ("-844.5530620517286511", "-3585610241942.1922435648192512", Greater),
        // 110 001 different scale
        ("539313715923.4424678", "12410950080603997079634706432", Less),
        ("-539313715923.4424678", "12410950080603997079634706432", Less),
        ("539313715923.4424678", "-12410950080603997079634706432", Greater),
        ("-539313715923.4424678", "-12410950080603997079634706432", Greater),
        // 110 101 same scale
        ("844.5530620517286511", "1947825396031.6933708908343230", Less),
        ("-844.5530620517286511", "1947825396031.6933708908343230", Less),
        ("844.5530620517286511", "-1947825396031.6933708908343230", Greater),
        ("-844.5530620517286511", "-1947825396031.6933708908343230", Greater),
        // 110 101 different scale
        ("0.2301405445512525317", "245433629587.71206426704897154", Less),
        ("-0.2301405445512525317", "245433629587.71206426704897154", Less),
        ("0.2301405445512525317", "-245433629587.71206426704897154", Greater),
        ("-0.2301405445512525317", "-245433629587.71206426704897154", Greater),
        // 110 011 same scale
        ("844.5530620517286511", "3637850451015.0843464291450880", Less),
        ("-844.5530620517286511", "3637850451015.0843464291450880", Less),
        ("844.5530620517286511", "-3637850451015.0843464291450880", Greater),
        ("-844.5530620517286511", "-3637850451015.0843464291450880", Greater),
        // 110 011 different scale
        ("0.00000000717944802566514691", "18143443615.480512395717115904", Less),
        ("-0.00000000717944802566514691", "18143443615.480512395717115904", Less),
        (
            "0.00000000717944802566514691",
            "-18143443615.480512395717115904",
            Greater,
        ),
        (
            "-0.00000000717944802566514691",
            "-18143443615.480512395717115904",
            Greater,
        ),
        // 110 111 same scale
        ("844.5530620517286511", "2738424264600.4917875777303163", Less),
        ("-844.5530620517286511", "2738424264600.4917875777303163", Less),
        ("844.5530620517286511", "-2738424264600.4917875777303163", Greater),
        ("-844.5530620517286511", "-2738424264600.4917875777303163", Greater),
        // 110 111 different scale
        ("0.0000000007762706076409491335", "2489879185787497651.6458518595", Less),
        (
            "-0.0000000007762706076409491335",
            "2489879185787497651.6458518595",
            Less,
        ),
        (
            "0.0000000007762706076409491335",
            "-2489879185787497651.6458518595",
            Greater,
        ),
        (
            "-0.0000000007762706076409491335",
            "-2489879185787497651.6458518595",
            Greater,
        ),
        // 001 equality
        ("316007568232.9263258873102336", "316007568232.9263258873102336", Equal),
        // 001 000 same scale
        ("316007568232.9263258873102336", "0.0000000000000000", Greater),
        ("-316007568232.9263258873102336", "0.0000000000000000", Less),
        ("316007568232.9263258873102336", "-0.0000000000000000", Greater),
        ("-316007568232.9263258873102336", "-0.0000000000000000", Less),
        // 001 000 different scale
        ("3522055990024364385815547084.8", "0.000000000000", Greater),
        ("-3522055990024364385815547084.8", "0.000000000000", Less),
        ("3522055990024364385815547084.8", "-0.000000000000", Greater),
        ("-3522055990024364385815547084.8", "-0.000000000000", Less),
        // 001 100 same scale
        ("316007568232.9263258873102336", "0.0000001073412971", Greater),
        ("-316007568232.9263258873102336", "0.0000001073412971", Less),
        ("316007568232.9263258873102336", "-0.0000001073412971", Greater),
        ("-316007568232.9263258873102336", "-0.0000001073412971", Less),
        // 001 100 different scale
        ("1319006.0491408208640440532992", "0.00000000000000611866432", Greater),
        ("-1319006.0491408208640440532992", "0.00000000000000611866432", Less),
        ("1319006.0491408208640440532992", "-0.00000000000000611866432", Greater),
        ("-1319006.0491408208640440532992", "-0.00000000000000611866432", Less),
        // 001 010 same scale
        ("316007568232.9263258873102336", "159.1054215143227392", Greater),
        ("-316007568232.9263258873102336", "159.1054215143227392", Less),
        ("316007568232.9263258873102336", "-159.1054215143227392", Greater),
        ("-316007568232.9263258873102336", "-159.1054215143227392", Less),
        // 001 010 different scale
        ("2470144.7146711063666704252928", "211.0186916505714688", Greater),
        ("-2470144.7146711063666704252928", "211.0186916505714688", Less),
        ("2470144.7146711063666704252928", "-211.0186916505714688", Greater),
        ("-2470144.7146711063666704252928", "-211.0186916505714688", Less),
        // 001 110 same scale
        ("316007568232.9263258873102336", "15.1186658969096112", Greater),
        ("-316007568232.9263258873102336", "15.1186658969096112", Less),
        ("316007568232.9263258873102336", "-15.1186658969096112", Greater),
        ("-316007568232.9263258873102336", "-15.1186658969096112", Less),
        // 001 110 different scale
        ("3840504199004148630832.3360768", "7581138850996748864", Greater),
        ("-3840504199004148630832.3360768", "7581138850996748864", Less),
        ("3840504199004148630832.3360768", "-7581138850996748864", Greater),
        ("-3840504199004148630832.3360768", "-7581138850996748864", Less),
        // 001 001 same scale
        ("316007568232.9263258873102336", "810157633226.6053390856880128", Less),
        ("-316007568232.9263258873102336", "810157633226.6053390856880128", Less),
        (
            "316007568232.9263258873102336",
            "-810157633226.6053390856880128",
            Greater,
        ),
        (
            "-316007568232.9263258873102336",
            "-810157633226.6053390856880128",
            Greater,
        ),
        // 001 001 different scale
        ("1951046382014.4037956952260608", "3626102772868740412010083.1232", Less),
        (
            "-1951046382014.4037956952260608",
            "3626102772868740412010083.1232",
            Less,
        ),
        (
            "1951046382014.4037956952260608",
            "-3626102772868740412010083.1232",
            Greater,
        ),
        (
            "-1951046382014.4037956952260608",
            "-3626102772868740412010083.1232",
            Greater,
        ),
        // 001 101 same scale
        ("316007568232.9263258873102336", "3258394380359.1965879291312453", Less),
        ("-316007568232.9263258873102336", "3258394380359.1965879291312453", Less),
        (
            "316007568232.9263258873102336",
            "-3258394380359.1965879291312453",
            Greater,
        ),
        (
            "-316007568232.9263258873102336",
            "-3258394380359.1965879291312453",
            Greater,
        ),
        // 001 101 different scale
        (
            "17580513970289834.943527780352",
            "3.7977957031395371036126086595",
            Greater,
        ),
        (
            "-17580513970289834.943527780352",
            "3.7977957031395371036126086595",
            Less,
        ),
        (
            "17580513970289834.943527780352",
            "-3.7977957031395371036126086595",
            Greater,
        ),
        (
            "-17580513970289834.943527780352",
            "-3.7977957031395371036126086595",
            Less,
        ),
        // 001 011 same scale
        ("316007568232.9263258873102336", "1154574080460.9867510617997312", Less),
        ("-316007568232.9263258873102336", "1154574080460.9867510617997312", Less),
        (
            "316007568232.9263258873102336",
            "-1154574080460.9867510617997312",
            Greater,
        ),
        (
            "-316007568232.9263258873102336",
            "-1154574080460.9867510617997312",
            Greater,
        ),
        // 001 011 different scale
        ("2008379587.5525351789031325696", "32824109460554.341800487157760", Less),
        (
            "-2008379587.5525351789031325696",
            "32824109460554.341800487157760",
            Less,
        ),
        (
            "2008379587.5525351789031325696",
            "-32824109460554.341800487157760",
            Greater,
        ),
        (
            "-2008379587.5525351789031325696",
            "-32824109460554.341800487157760",
            Greater,
        ),
        // 001 111 same scale
        ("316007568232.9263258873102336", "2816795479724.6069787794805311", Less),
        ("-316007568232.9263258873102336", "2816795479724.6069787794805311", Less),
        (
            "316007568232.9263258873102336",
            "-2816795479724.6069787794805311",
            Greater,
        ),
        (
            "-316007568232.9263258873102336",
            "-2816795479724.6069787794805311",
            Greater,
        ),
        // 001 111 different scale
        (
            "3536806574745420013541890.4576",
            "9793146.81730411145833529126",
            Greater,
        ),
        ("-3536806574745420013541890.4576", "9793146.81730411145833529126", Less),
        (
            "3536806574745420013541890.4576",
            "-9793146.81730411145833529126",
            Greater,
        ),
        ("-3536806574745420013541890.4576", "-9793146.81730411145833529126", Less),
        // 101 equality
        (
            "254208186622762823842.71629992",
            "254208186622762823842.71629992",
            Equal,
        ),
        // 101 000 same scale
        ("254208186622762823842.71629992", "0.00000000", Greater),
        ("-254208186622762823842.71629992", "0.00000000", Less),
        ("254208186622762823842.71629992", "-0.00000000", Greater),
        ("-254208186622762823842.71629992", "-0.00000000", Less),
        // 101 000 different scale
        ("975421950664039.3614304091804", "0.0000000000000000", Greater),
        ("-975421950664039.3614304091804", "0.0000000000000000", Less),
        ("975421950664039.3614304091804", "-0.0000000000000000", Greater),
        ("-975421950664039.3614304091804", "-0.0000000000000000", Less),
        // 101 100 same scale
        ("254208186622762823842.71629992", "10.74141379", Greater),
        ("-254208186622762823842.71629992", "10.74141379", Less),
        ("254208186622762823842.71629992", "-10.74141379", Greater),
        ("-254208186622762823842.71629992", "-10.74141379", Less),
        // 101 100 different scale
        ("2552221405032275.8358630506229", "0.000000000000000592656995", Greater),
        ("-2552221405032275.8358630506229", "0.000000000000000592656995", Less),
        ("2552221405032275.8358630506229", "-0.000000000000000592656995", Greater),
        ("-2552221405032275.8358630506229", "-0.000000000000000592656995", Less),
        // 101 010 same scale
        ("254208186622762823842.71629992", "62767493748.48499712", Greater),
        ("-254208186622762823842.71629992", "62767493748.48499712", Less),
        ("254208186622762823842.71629992", "-62767493748.48499712", Greater),
        ("-254208186622762823842.71629992", "-62767493748.48499712", Less),
        // 101 010 different scale
        ("197346074219.25327589999174264", "0.7623493575178715136", Greater),
        ("-197346074219.25327589999174264", "0.7623493575178715136", Less),
        ("197346074219.25327589999174264", "-0.7623493575178715136", Greater),
        ("-197346074219.25327589999174264", "-0.7623493575178715136", Less),
        // 101 110 same scale
        ("254208186622762823842.71629992", "76597126194.51389094", Greater),
        ("-254208186622762823842.71629992", "76597126194.51389094", Less),
        ("254208186622762823842.71629992", "-76597126194.51389094", Greater),
        ("-254208186622762823842.71629992", "-76597126194.51389094", Less),
        // 101 110 different scale
        ("25899773651648.380071130043467", "61306258142804903.38", Less),
        ("-25899773651648.380071130043467", "61306258142804903.38", Less),
        ("25899773651648.380071130043467", "-61306258142804903.38", Greater),
        ("-25899773651648.380071130043467", "-61306258142804903.38", Greater),
        // 101 001 same scale
        (
            "254208186622762823842.71629992",
            "83547191565151621967.28086528",
            Greater,
        ),
        ("-254208186622762823842.71629992", "83547191565151621967.28086528", Less),
        (
            "254208186622762823842.71629992",
            "-83547191565151621967.28086528",
            Greater,
        ),
        (
            "-254208186622762823842.71629992",
            "-83547191565151621967.28086528",
            Less,
        ),
        // 101 001 different scale
        ("244762.90302171293318719286219", "9964072.600255221193011888128", Less),
        ("-244762.90302171293318719286219", "9964072.600255221193011888128", Less),
        (
            "244762.90302171293318719286219",
            "-9964072.600255221193011888128",
            Greater,
        ),
        (
            "-244762.90302171293318719286219",
            "-9964072.600255221193011888128",
            Greater,
        ),
        // 101 101 same scale
        (
            "254208186622762823842.71629992",
            "106541875981662806348.63716235",
            Greater,
        ),
        (
            "-254208186622762823842.71629992",
            "106541875981662806348.63716235",
            Less,
        ),
        (
            "254208186622762823842.71629992",
            "-106541875981662806348.63716235",
            Greater,
        ),
        (
            "-254208186622762823842.71629992",
            "-106541875981662806348.63716235",
            Less,
        ),
        // 101 101 different scale
        ("362319.18250030256385507568342", "3619454249020577423546109236", Less),
        ("-362319.18250030256385507568342", "3619454249020577423546109236", Less),
        (
            "362319.18250030256385507568342",
            "-3619454249020577423546109236",
            Greater,
        ),
        (
            "-362319.18250030256385507568342",
            "-3619454249020577423546109236",
            Greater,
        ),
        // 101 011 same scale
        (
            "254208186622762823842.71629992",
            "156781478378762688050.06557184",
            Greater,
        ),
        (
            "-254208186622762823842.71629992",
            "156781478378762688050.06557184",
            Less,
        ),
        (
            "254208186622762823842.71629992",
            "-156781478378762688050.06557184",
            Greater,
        ),
        (
            "-254208186622762823842.71629992",
            "-156781478378762688050.06557184",
            Less,
        ),
        // 101 011 different scale
        ("2486073465266.0337130589931876", "153874906950888902858.19691008", Less),
        (
            "-2486073465266.0337130589931876",
            "153874906950888902858.19691008",
            Less,
        ),
        (
            "2486073465266.0337130589931876",
            "-153874906950888902858.19691008",
            Greater,
        ),
        (
            "-2486073465266.0337130589931876",
            "-153874906950888902858.19691008",
            Greater,
        ),
        // 101 111 same scale
        (
            "254208186622762823842.71629992",
            "10101645723744656462.08148676",
            Greater,
        ),
        ("-254208186622762823842.71629992", "10101645723744656462.08148676", Less),
        (
            "254208186622762823842.71629992",
            "-10101645723744656462.08148676",
            Greater,
        ),
        (
            "-254208186622762823842.71629992",
            "-10101645723744656462.08148676",
            Less,
        ),
        // 101 111 different scale
        ("14107601965.909635434653634526", "111238758546502973973477.99454", Less),
        (
            "-14107601965.909635434653634526",
            "111238758546502973973477.99454",
            Less,
        ),
        (
            "14107601965.909635434653634526",
            "-111238758546502973973477.99454",
            Greater,
        ),
        (
            "-14107601965.909635434653634526",
            "-111238758546502973973477.99454",
            Greater,
        ),
        // 011 equality
        (
            "272322.48219624218537039495168",
            "272322.48219624218537039495168",
            Equal,
        ),
        // 011 000 same scale
        ("272322.48219624218537039495168", "0.00000000000000000000000", Greater),
        ("-272322.48219624218537039495168", "0.00000000000000000000000", Less),
        ("272322.48219624218537039495168", "-0.00000000000000000000000", Greater),
        ("-272322.48219624218537039495168", "-0.00000000000000000000000", Less),
        // 011 000 different scale
        ("3214885516.0787854158246969344", "0.00000000000000000", Greater),
        ("-3214885516.0787854158246969344", "0.00000000000000000", Less),
        ("3214885516.0787854158246969344", "-0.00000000000000000", Greater),
        ("-3214885516.0787854158246969344", "-0.00000000000000000", Less),
        // 011 100 same scale
        ("272322.48219624218537039495168", "0.00000000000000379487994", Greater),
        ("-272322.48219624218537039495168", "0.00000000000000379487994", Less),
        ("272322.48219624218537039495168", "-0.00000000000000379487994", Greater),
        ("-272322.48219624218537039495168", "-0.00000000000000379487994", Less),
        // 011 100 different scale
        ("388166715906.19371912596291584", "0.000000000000001996700736", Greater),
        ("-388166715906.19371912596291584", "0.000000000000001996700736", Less),
        ("388166715906.19371912596291584", "-0.000000000000001996700736", Greater),
        ("-388166715906.19371912596291584", "-0.000000000000001996700736", Less),
        // 011 010 same scale
        ("272322.48219624218537039495168", "0.00000175873997328613376", Greater),
        ("-272322.48219624218537039495168", "0.00000175873997328613376", Less),
        ("272322.48219624218537039495168", "-0.00000175873997328613376", Greater),
        ("-272322.48219624218537039495168", "-0.00000175873997328613376", Less),
        // 011 010 different scale
        (
            "17963864.0946434527121637376",
            "0.0000000001112699367808040960",
            Greater,
        ),
        ("-17963864.0946434527121637376", "0.0000000001112699367808040960", Less),
        (
            "17963864.0946434527121637376",
            "-0.0000000001112699367808040960",
            Greater,
        ),
        ("-17963864.0946434527121637376", "-0.0000000001112699367808040960", Less),
        // 011 110 same scale
        ("272322.48219624218537039495168", "0.00009168252278596474115", Greater),
        ("-272322.48219624218537039495168", "0.00009168252278596474115", Less),
        ("272322.48219624218537039495168", "-0.00009168252278596474115", Greater),
        ("-272322.48219624218537039495168", "-0.00009168252278596474115", Less),
        // 011 110 different scale
        ("3.1530040332824324172729548800", "0.05636139104712652784", Greater),
        ("-3.1530040332824324172729548800", "0.05636139104712652784", Less),
        ("3.1530040332824324172729548800", "-0.05636139104712652784", Greater),
        ("-3.1530040332824324172729548800", "-0.05636139104712652784", Less),
        // 011 001 same scale
        (
            "272322.48219624218537039495168",
            "78956.15667671475190631497728",
            Greater,
        ),
        ("-272322.48219624218537039495168", "78956.15667671475190631497728", Less),
        (
            "272322.48219624218537039495168",
            "-78956.15667671475190631497728",
            Greater,
        ),
        (
            "-272322.48219624218537039495168",
            "-78956.15667671475190631497728",
            Less,
        ),
        // 011 001 different scale
        (
            "247159638929.90774677497446400",
            "18573048.37697991870462820352",
            Greater,
        ),
        ("-247159638929.90774677497446400", "18573048.37697991870462820352", Less),
        (
            "247159638929.90774677497446400",
            "-18573048.37697991870462820352",
            Greater,
        ),
        (
            "-247159638929.90774677497446400",
            "-18573048.37697991870462820352",
            Less,
        ),
        // 011 101 same scale
        ("272322.48219624218537039495168", "311953.28803357654172947915942", Less),
        (
            "-272322.48219624218537039495168",
            "311953.28803357654172947915942",
            Less,
        ),
        (
            "272322.48219624218537039495168",
            "-311953.28803357654172947915942",
            Greater,
        ),
        (
            "-272322.48219624218537039495168",
            "-311953.28803357654172947915942",
            Greater,
        ),
        // 011 101 different scale
        ("2922696937234119470.0273745920", "22441101906503785827606686629", Less),
        ("-2922696937234119470.0273745920", "22441101906503785827606686629", Less),
        (
            "2922696937234119470.0273745920",
            "-22441101906503785827606686629",
            Greater,
        ),
        (
            "-2922696937234119470.0273745920",
            "-22441101906503785827606686629",
            Greater,
        ),
        // 011 011 same scale
        ("272322.48219624218537039495168", "348316.28306497394164006649856", Less),
        (
            "-272322.48219624218537039495168",
            "348316.28306497394164006649856",
            Less,
        ),
        (
            "272322.48219624218537039495168",
            "-348316.28306497394164006649856",
            Greater,
        ),
        (
            "-272322.48219624218537039495168",
            "-348316.28306497394164006649856",
            Greater,
        ),
        // 011 011 different scale
        ("178190346.76624086261395619840", "62208030746.22038852927225856", Less),
        ("-178190346.76624086261395619840", "62208030746.22038852927225856", Less),
        (
            "178190346.76624086261395619840",
            "-62208030746.22038852927225856",
            Greater,
        ),
        (
            "-178190346.76624086261395619840",
            "-62208030746.22038852927225856",
            Greater,
        ),
        // 011 111 same scale
        (
            "272322.48219624218537039495168",
            "41534.52021391898039335355927",
            Greater,
        ),
        ("-272322.48219624218537039495168", "41534.52021391898039335355927", Less),
        (
            "272322.48219624218537039495168",
            "-41534.52021391898039335355927",
            Greater,
        ),
        (
            "-272322.48219624218537039495168",
            "-41534.52021391898039335355927",
            Less,
        ),
        // 011 111 different scale
        ("11.959910518519677083499626496", "2844684364802261541879551.2259", Less),
        (
            "-11.959910518519677083499626496",
            "2844684364802261541879551.2259",
            Less,
        ),
        (
            "11.959910518519677083499626496",
            "-2844684364802261541879551.2259",
            Greater,
        ),
        (
            "-11.959910518519677083499626496",
            "-2844684364802261541879551.2259",
            Greater,
        ),
        // 111 equality
        (
            "3836286746260530032892706.6174",
            "3836286746260530032892706.6174",
            Equal,
        ),
        // 111 000 same scale
        ("3836286746260530032892706.6174", "0.0000", Greater),
        ("-3836286746260530032892706.6174", "0.0000", Less),
        ("3836286746260530032892706.6174", "-0.0000", Greater),
        ("-3836286746260530032892706.6174", "-0.0000", Less),
        // 111 000 different scale
        ("4401861854803552.033657814547", "0.0000000", Greater),
        ("-4401861854803552.033657814547", "0.0000000", Less),
        ("4401861854803552.033657814547", "-0.0000000", Greater),
        ("-4401861854803552.033657814547", "-0.0000000", Less),
        // 111 100 same scale
        ("3836286746260530032892706.6174", "68758.6561", Greater),
        ("-3836286746260530032892706.6174", "68758.6561", Less),
        ("3836286746260530032892706.6174", "-68758.6561", Greater),
        ("-3836286746260530032892706.6174", "-68758.6561", Less),
        // 111 100 different scale
        (
            "18794337354296131695536777.153",
            "0.000000000000000001563875977",
            Greater,
        ),
        ("-18794337354296131695536777.153", "0.000000000000000001563875977", Less),
        (
            "18794337354296131695536777.153",
            "-0.000000000000000001563875977",
            Greater,
        ),
        (
            "-18794337354296131695536777.153",
            "-0.000000000000000001563875977",
            Less,
        ),
        // 111 010 same scale
        ("3836286746260530032892706.6174", "439097665563236.7616", Greater),
        ("-3836286746260530032892706.6174", "439097665563236.7616", Less),
        ("3836286746260530032892706.6174", "-439097665563236.7616", Greater),
        ("-3836286746260530032892706.6174", "-439097665563236.7616", Less),
        // 111 010 different scale
        ("219364497389.57405761662363679", "0.6569644274462228480", Greater),
        ("-219364497389.57405761662363679", "0.6569644274462228480", Less),
        ("219364497389.57405761662363679", "-0.6569644274462228480", Greater),
        ("-219364497389.57405761662363679", "-0.6569644274462228480", Less),
        // 111 110 same scale
        ("3836286746260530032892706.6174", "100013274294974.8269", Greater),
        ("-3836286746260530032892706.6174", "100013274294974.8269", Less),
        ("3836286746260530032892706.6174", "-100013274294974.8269", Greater),
        ("-3836286746260530032892706.6174", "-100013274294974.8269", Less),
        // 111 110 different scale
        ("76072704083682.85472479207171", "50.52437989651117182", Greater),
        ("-76072704083682.85472479207171", "50.52437989651117182", Less),
        ("76072704083682.85472479207171", "-50.52437989651117182", Greater),
        ("-76072704083682.85472479207171", "-50.52437989651117182", Less),
        // 111 001 same scale
        (
            "3836286746260530032892706.6174",
            "2766133872545894272402142.0032",
            Greater,
        ),
        (
            "-3836286746260530032892706.6174",
            "2766133872545894272402142.0032",
            Less,
        ),
        (
            "3836286746260530032892706.6174",
            "-2766133872545894272402142.0032",
            Greater,
        ),
        (
            "-3836286746260530032892706.6174",
            "-2766133872545894272402142.0032",
            Less,
        ),
        // 111 001 different scale
        (
            "38199979438250010.80610984395",
            "31104752430710.408848162684928",
            Greater,
        ),
        ("-38199979438250010.80610984395", "31104752430710.408848162684928", Less),
        (
            "38199979438250010.80610984395",
            "-31104752430710.408848162684928",
            Greater,
        ),
        (
            "-38199979438250010.80610984395",
            "-31104752430710.408848162684928",
            Less,
        ),
        // 111 101 same scale
        (
            "3836286746260530032892706.6174",
            "441847458119168110406908.6115",
            Greater,
        ),
        ("-3836286746260530032892706.6174", "441847458119168110406908.6115", Less),
        (
            "3836286746260530032892706.6174",
            "-441847458119168110406908.6115",
            Greater,
        ),
        (
            "-3836286746260530032892706.6174",
            "-441847458119168110406908.6115",
            Less,
        ),
        // 111 101 different scale
        ("255945012905633524.15746865235", "1005021647855597114428453997.8", Less),
        (
            "-255945012905633524.15746865235",
            "1005021647855597114428453997.8",
            Less,
        ),
        (
            "255945012905633524.15746865235",
            "-1005021647855597114428453997.8",
            Greater,
        ),
        (
            "-255945012905633524.15746865235",
            "-1005021647855597114428453997.8",
            Greater,
        ),
        // 111 011 same scale
        (
            "3836286746260530032892706.6174",
            "1111481055212557787730018.3040",
            Greater,
        ),
        (
            "-3836286746260530032892706.6174",
            "1111481055212557787730018.3040",
            Less,
        ),
        (
            "3836286746260530032892706.6174",
            "-1111481055212557787730018.3040",
            Greater,
        ),
        (
            "-3836286746260530032892706.6174",
            "-1111481055212557787730018.3040",
            Less,
        ),
        // 111 011 different scale
        (
            "79710135995301690627798250.27",
            "4613684285077.479267304996864",
            Greater,
        ),
        ("-79710135995301690627798250.27", "4613684285077.479267304996864", Less),
        (
            "79710135995301690627798250.27",
            "-4613684285077.479267304996864",
            Greater,
        ),
        ("-79710135995301690627798250.27", "-4613684285077.479267304996864", Less),
        // 111 111 same scale
        (
            "3836286746260530032892706.6174",
            "1881105048659612897896770.8539",
            Greater,
        ),
        (
            "-3836286746260530032892706.6174",
            "1881105048659612897896770.8539",
            Less,
        ),
        (
            "3836286746260530032892706.6174",
            "-1881105048659612897896770.8539",
            Greater,
        ),
        (
            "-3836286746260530032892706.6174",
            "-1881105048659612897896770.8539",
            Less,
        ),
        // 111 111 different scale
        (
            "3879592276836332218003.2886500",
            "35612499407667292.686490959658",
            Greater,
        ),
        (
            "-3879592276836332218003.2886500",
            "35612499407667292.686490959658",
            Less,
        ),
        (
            "3879592276836332218003.2886500",
            "-35612499407667292.686490959658",
            Greater,
        ),
        (
            "-3879592276836332218003.2886500",
            "-35612499407667292.686490959658",
            Less,
        ),
    ];
    for &(a, b, c) in tests {
        cmp(a, b, c);
    }
}

#[test]
fn it_floors_decimals() {
    let tests = &[
        ("1", "1"),
        ("1.00", "1"),
        ("1.2345", "1"),
        ("-1", "-1"),
        ("-1.00", "-1"),
        ("-1.2345", "-2"),
    ];
    for &(a, expected) in tests {
        let a = Decimal::from_str(a).unwrap();
        assert_eq!(expected, a.floor().to_string(), "Failed flooring {a}");
    }
}

#[test]
fn it_ceils_decimals() {
    let tests = &[
        ("1", "1"),
        ("1.00", "1"),
        ("1.2345", "2"),
        ("-1", "-1"),
        ("-1.00", "-1"),
        ("-1.2345", "-1"),
    ];
    for &(a, expected) in tests {
        let a = Decimal::from_str(a).unwrap();
        assert_eq!(expected, a.ceil().to_string(), "Failed ceiling {a}");
    }
}

#[test]
fn it_finds_max_of_two() {
    let tests = &[("1", "1", "1"), ("2", "1", "2"), ("1", "2", "2")];
    for &(a, b, expected) in tests {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        assert_eq!(expected, a.max(b).to_string());
    }
}

#[test]
fn it_finds_min_of_two() {
    let tests = &[("1", "1", "1"), ("2", "1", "1"), ("1", "2", "1")];
    for &(a, b, expected) in tests {
        let a = Decimal::from_str(a).unwrap();
        let b = Decimal::from_str(b).unwrap();
        assert_eq!(expected, a.min(b).to_string());
    }
}

#[test]
fn test_max_compares() {
    let x = "225.33543601344182".parse::<Decimal>().unwrap();
    let y = Decimal::MAX;
    assert!(x < y);
    assert!(y > x);
    assert_ne!(y, x);
}

#[test]
fn test_min_compares() {
    let x = "225.33543601344182".parse::<Decimal>().unwrap();
    let y = Decimal::MIN;
    assert!(x > y);
    assert!(y < x);
    assert_ne!(y, x);
}

#[test]
fn it_can_parse_from_i32() {
    use num_traits::FromPrimitive;

    let tests = &[
        (0i32, "0"),
        (1i32, "1"),
        (-1i32, "-1"),
        (i32::MAX, "2147483647"),
        (i32::MIN, "-2147483648"),
    ];
    for &(input, expected) in tests {
        let parsed = Decimal::from_i32(input).unwrap();
        assert_eq!(
            expected,
            parsed.to_string(),
            "expected {expected} does not match parsed {parsed}"
        );
        assert_eq!(
            input.to_string(),
            parsed.to_string(),
            "i32 to_string {input} does not match parsed {parsed}"
        );
    }
}

#[test]
fn it_can_parse_from_i64() {
    use num_traits::FromPrimitive;

    let tests = &[
        (0i64, "0"),
        (1i64, "1"),
        (-1i64, "-1"),
        (i64::MAX, "9223372036854775807"),
        (i64::MIN, "-9223372036854775808"),
    ];
    for &(input, expected) in tests {
        let parsed = Decimal::from_i64(input).unwrap();
        assert_eq!(
            expected,
            parsed.to_string(),
            "expected {expected} does not match parsed {parsed}"
        );
        assert_eq!(
            input.to_string(),
            parsed.to_string(),
            "i64 to_string {input} does not match parsed {parsed}"
        );
    }
}

#[test]
fn it_can_round_to_2dp() {
    let a = Decimal::from_str("6.12345").unwrap();
    let b = (Decimal::from_str("100").unwrap() * a).round() / Decimal::from_str("100").unwrap();
    assert_eq!("6.12", b.to_string());
}

#[test]
fn it_can_round_using_basic_midpoint_rules() {
    let tests = &[
        ("3.5", RoundingStrategy::MidpointAwayFromZero, "4"),
        ("2.8", RoundingStrategy::MidpointAwayFromZero, "3"),
        ("2.5", RoundingStrategy::MidpointAwayFromZero, "3"),
        ("2.1", RoundingStrategy::MidpointAwayFromZero, "2"),
        ("-2.1", RoundingStrategy::MidpointAwayFromZero, "-2"),
        ("-2.5", RoundingStrategy::MidpointAwayFromZero, "-3"),
        ("-2.8", RoundingStrategy::MidpointAwayFromZero, "-3"),
        ("-3.5", RoundingStrategy::MidpointAwayFromZero, "-4"),
        ("3.5", RoundingStrategy::MidpointNearestEven, "4"),
        ("2.8", RoundingStrategy::MidpointNearestEven, "3"),
        ("2.5", RoundingStrategy::MidpointNearestEven, "2"),
        ("2.1", RoundingStrategy::MidpointNearestEven, "2"),
        ("-2.1", RoundingStrategy::MidpointNearestEven, "-2"),
        ("-2.5", RoundingStrategy::MidpointNearestEven, "-2"),
        ("-2.8", RoundingStrategy::MidpointNearestEven, "-3"),
        ("-3.5", RoundingStrategy::MidpointNearestEven, "-4"),
        ("3.5", RoundingStrategy::MidpointTowardZero, "3"),
        ("2.8", RoundingStrategy::MidpointTowardZero, "3"),
        ("2.5", RoundingStrategy::MidpointTowardZero, "2"),
        ("2.1", RoundingStrategy::MidpointTowardZero, "2"),
        ("-2.1", RoundingStrategy::MidpointTowardZero, "-2"),
        ("-2.5", RoundingStrategy::MidpointTowardZero, "-2"),
        ("-2.8", RoundingStrategy::MidpointTowardZero, "-3"),
        ("-3.5", RoundingStrategy::MidpointTowardZero, "-3"),
        ("2.8", RoundingStrategy::ToNegativeInfinity, "2"),
        ("2.5", RoundingStrategy::ToNegativeInfinity, "2"),
        ("2.1", RoundingStrategy::ToNegativeInfinity, "2"),
        ("-2.1", RoundingStrategy::ToNegativeInfinity, "-3"),
        ("-2.5", RoundingStrategy::ToNegativeInfinity, "-3"),
        ("-2.8", RoundingStrategy::ToNegativeInfinity, "-3"),
        ("2.8", RoundingStrategy::ToPositiveInfinity, "3"),
        ("2.5", RoundingStrategy::ToPositiveInfinity, "3"),
        ("2.1", RoundingStrategy::ToPositiveInfinity, "3"),
        ("-2.1", RoundingStrategy::ToPositiveInfinity, "-2"),
        ("-2.5", RoundingStrategy::ToPositiveInfinity, "-2"),
        ("-2.8", RoundingStrategy::ToPositiveInfinity, "-2"),
        ("2.8", RoundingStrategy::ToZero, "2"),
        ("2.5", RoundingStrategy::ToZero, "2"),
        ("2.1", RoundingStrategy::ToZero, "2"),
        ("-2.1", RoundingStrategy::ToZero, "-2"),
        ("-2.5", RoundingStrategy::ToZero, "-2"),
        ("-2.8", RoundingStrategy::ToZero, "-2"),
        ("2.8", RoundingStrategy::AwayFromZero, "3"),
        ("2.5", RoundingStrategy::AwayFromZero, "3"),
        ("2.1", RoundingStrategy::AwayFromZero, "3"),
        ("-2.1", RoundingStrategy::AwayFromZero, "-3"),
        ("-2.5", RoundingStrategy::AwayFromZero, "-3"),
        ("-2.8", RoundingStrategy::AwayFromZero, "-3"),
    ];

    for &(input, strategy, expected) in tests {
        let a = Decimal::from_str(input).unwrap();
        let b = a.round_dp_with_strategy(0, strategy);
        assert_eq!(expected, b.to_string(), "{input} > {expected} for {strategy:?}");
    }
}

#[test]
fn it_can_round_using_bankers_rounding() {
    let tests = &[
        ("6.12345", 2, "6.12"),
        ("6.126", 2, "6.13"),
        ("-6.126", 2, "-6.13"),
        ("6.5", 0, "6"),
        ("7.5", 0, "8"),
        ("1.2250", 2, "1.22"),
        ("1.2252", 2, "1.23"),
        ("1.2249", 2, "1.22"),
        ("6.1", 2, "6.1"),
        ("0.0000", 2, "0.00"),
        ("0.6666666666666666666666666666", 2, "0.67"),
        ("1.40", 0, "1"),
        ("2.60", 0, "3"),
        ("2.1234567890123456789012345678", 27, "2.123456789012345678901234568"),
    ];
    for &(input, dp, expected) in tests {
        let a = Decimal::from_str(input).unwrap();
        #[allow(deprecated)]
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::BankersRounding);
        assert_eq!(expected, b.to_string(), "BankersRounding");

        // Recommended replacement
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::MidpointNearestEven);
        assert_eq!(expected, b.to_string(), "MidpointNearestEven");
    }
}

#[test]
fn it_can_round_complex_numbers_using_bankers_rounding() {
    // Issue #71
    let rate = Decimal::new(19, 2); // 0.19
    let one = Decimal::new(1, 0); // 1
    let part = rate / (rate + one); // 0.19 / (0.19 + 1) = 0.1596638655462184873949579832

    #[allow(deprecated)]
    let part = part.round_dp_with_strategy(2, RoundingStrategy::BankersRounding); // 0.16
    assert_eq!("0.16", part.to_string(), "BankersRounding");

    // Recommended replacement
    let part = part.round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven); // 0.16
    assert_eq!("0.16", part.to_string(), "MidpointNearestEven");
}

#[test]
fn it_can_round_using_round_half_up() {
    let tests = &[
        ("0", 0, "0"),
        ("1.234", 3, "1.234"),
        ("1.12", 5, "1.12"),
        ("6.34567", 2, "6.35"),
        ("6.5", 0, "7"),
        ("12.49", 0, "12"),
        ("0.6666666666666666666666666666", 2, "0.67"),
        ("1.40", 0, "1"),
        ("2.60", 0, "3"),
        ("2.1234567890123456789012345678", 27, "2.123456789012345678901234568"),
    ];
    for &(input, dp, expected) in tests {
        let a = Decimal::from_str(input).unwrap();
        #[allow(deprecated)]
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::RoundHalfUp);
        assert_eq!(expected, b.to_string(), "RoundHalfUp");

        // Recommended replacement
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::MidpointAwayFromZero);
        assert_eq!(expected, b.to_string(), "MidpointAwayFromZero");
    }
}

#[test]
fn it_can_round_complex_numbers_using_round_half_up() {
    // Issue #71
    let rate = Decimal::new(19, 2); // 0.19
    let one = Decimal::new(1, 0); // 1
    let part = rate / (rate + one); // 0.19 / (0.19 + 1) = 0.1596638655462184873949579832
    #[allow(deprecated)]
    let part = part.round_dp_with_strategy(2, RoundingStrategy::RoundHalfUp); // 0.16
    assert_eq!("0.16", part.to_string(), "RoundHalfUp");

    // Recommended replacement
    let part = part.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero); // 0.16
    assert_eq!("0.16", part.to_string(), "MidpointAwayFromZero");
}

#[test]
fn it_can_round_using_round_half_down() {
    let tests = &[
        ("0", 0, "0"),
        ("1.234", 3, "1.234"),
        ("1.12", 5, "1.12"),
        ("6.34567", 2, "6.35"),
        ("6.51", 0, "7"),
        ("12.5", 0, "12"),
        ("0.6666666666666666666666666666", 2, "0.67"),
        ("1.40", 0, "1"),
        ("2.60", 0, "3"),
        ("2.1234567890123456789012345678", 27, "2.123456789012345678901234568"),
    ];
    for &(input, dp, expected) in tests {
        let a = Decimal::from_str(input).unwrap();
        #[allow(deprecated)]
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::RoundHalfDown);
        assert_eq!(expected, b.to_string(), "RoundHalfDown");

        // Recommended replacement
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::MidpointTowardZero);
        assert_eq!(expected, b.to_string(), "MidpointTowardZero");
    }
}

#[test]
fn it_can_round_complex_numbers_using_round_half_down() {
    // Issue #71
    let rate = Decimal::new(19, 2); // 0.19
    let one = Decimal::new(1, 0); // 1
    let part = rate / (rate + one); // 0.19 / (0.19 + 1) = 0.1596638655462184873949579832

    #[allow(deprecated)]
    let part = part.round_dp_with_strategy(2, RoundingStrategy::RoundHalfDown); // 0.16
    assert_eq!("0.16", part.to_string(), "RoundHalfDown");

    // Recommended replacement
    let part = part.round_dp_with_strategy(2, RoundingStrategy::MidpointTowardZero); // 0.16
    assert_eq!("0.16", part.to_string(), "RoundHalfDown");
}

#[test]
fn it_can_round_to_2dp_using_explicit_function() {
    let a = Decimal::from_str("6.12345").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("6.12", b.to_string());
}

#[test]
fn it_can_round_up_to_2dp_using_explicit_function() {
    let a = Decimal::from_str("6.126").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("6.13", b.to_string());
}

#[test]
fn it_can_round_down_to_2dp_using_explicit_function() {
    let a = Decimal::from_str("-6.126").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("-6.13", b.to_string());
}

#[test]
fn it_can_round_down_using_bankers_rounding() {
    let a = Decimal::from_str("6.5").unwrap();
    let b = a.round_dp(0u32);
    assert_eq!("6", b.to_string());
}

#[test]
fn it_can_round_up_using_bankers_rounding() {
    let a = Decimal::from_str("7.5").unwrap();
    let b = a.round_dp(0u32);
    assert_eq!("8", b.to_string());
}

#[test]
fn it_can_round_correctly_using_bankers_rounding_1() {
    let a = Decimal::from_str("1.2250").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("1.22", b.to_string());
}

#[test]
fn it_can_round_correctly_using_bankers_rounding_2() {
    let a = Decimal::from_str("1.2251").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("1.23", b.to_string());
}

#[test]
fn it_can_round_down_when_required() {
    let a = Decimal::from_str("1.2249").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("1.22", b.to_string());
}

#[test]
fn it_can_round_to_2dp_using_explicit_function_without_changing_value() {
    let a = Decimal::from_str("6.1").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("6.1", b.to_string());
}

#[test]
fn it_can_round_zero() {
    let a = Decimal::from_str("0.0000").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("0.00", b.to_string());
}

#[test]
fn it_can_round_large_decimals() {
    let a = Decimal::from_str("0.6666666666666666666666666666").unwrap();
    let b = a.round_dp(2u32);
    assert_eq!("0.67", b.to_string());
}

#[test]
fn it_can_round_simple_numbers_down() {
    let a = Decimal::from_str("1.40").unwrap();
    let b = a.round_dp(0u32);
    assert_eq!("1", b.to_string());
}

#[test]
fn it_can_round_simple_numbers_up() {
    let a = Decimal::from_str("2.60").unwrap();
    let b = a.round_dp(0u32);
    assert_eq!("3", b.to_string());
}

#[test]
fn it_can_round_simple_numbers_with_high_precision() {
    let a = Decimal::from_str("2.1234567890123456789012345678").unwrap();
    let b = a.round_dp(27u32);
    assert_eq!("2.123456789012345678901234568", b.to_string());
}

#[test]
fn it_can_round_complex_numbers() {
    // Issue #71
    let rate = Decimal::new(19, 2); // 0.19
    let one = Decimal::new(1, 0); // 1
    let part = rate / (rate + one); // 0.19 / (0.19 + 1) = 0.1596638655462184873949579832
    let part = part.round_dp(2); // 0.16
    assert_eq!("0.16", part.to_string());
}

#[test]
fn it_does_not_round_decimals_to_too_many_dp() {
    // Issue 574
    let zero = Decimal::new(0, 28);
    let rounded = zero.round_dp(32);
    assert_eq!(rounded.scale(), 28); // If dp > old_scale, we retain the old scale.
    rounded.to_string();
}

#[test]
fn it_can_round_down() {
    let tests = &[
        ("0.470", 1, "0.4"),
        ("-0.470", 1, "-0.4"), // Toward zero
        ("0.400", 1, "0.4"),
        ("-0.400", 1, "-0.4"),
    ];
    for &(input, dp, expected) in tests {
        let a = Decimal::from_str(input).unwrap();
        #[allow(deprecated)]
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::RoundDown);
        assert_eq!(expected, b.to_string(), "RoundDown");

        // Recommended replacement
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::ToZero);
        assert_eq!(expected, b.to_string(), "ToZero");
    }
}

#[test]
fn it_can_round_up() {
    let tests = &[
        ("2.8", 0, "3"),
        ("2.5", 0, "3"),
        ("2.1", 0, "3"),
        ("-2.1", 0, "-3"),
        ("-2.5", 0, "-3"),
        ("-2.8", 0, "-3"),
        ("0.320", 1, "0.4"),
        ("-0.320", 1, "-0.4"),
        ("0.300", 1, "0.3"),
        ("-0.300", 1, "-0.3"),
    ];

    for &(input, dp, expected) in tests {
        let a = Decimal::from_str(input).unwrap();
        #[allow(deprecated)]
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::RoundUp);
        assert_eq!(expected, b.to_string(), "RoundUp");

        // Recommended replacement
        let b = a.round_dp_with_strategy(dp, RoundingStrategy::AwayFromZero);
        assert_eq!(expected, b.to_string(), "AwayFromZero");
    }
}

#[test]
fn it_can_round_significant_figures() {
    let tests = &[
        ("305.459", 0u32, Some("0")),
        ("305.459", 1, Some("300")),
        ("305.459", 2, Some("310")),
        ("305.459", 3, Some("305")),
        ("305.459", 4, Some("305.5")),
        ("305.459", 5, Some("305.46")),
        ("305.459", 6, Some("305.459")),
        ("305.459", 7, Some("305.4590")),
        ("305.459", 10, Some("305.4590000")),
        ("-305.459", 3, Some("-305")),
        ("-305.459", 2, Some("-310")), // We ignore the negative
        ("-305.459", 5, Some("-305.46")),
        (
            "79228162514264337593543950335",
            29,
            Some("79228162514264337593543950335"),
        ),
        ("79228162514264337593543950335", 1, None),
        (
            "79228162514264337593543950335",
            2,
            Some("79000000000000000000000000000"),
        ),
        (
            "79228162514264337593543950335",
            30,
            Some("79228162514264337593543950335"),
        ),
        (
            "79228162514264337593543950335",
            u32::MAX,
            Some("79228162514264337593543950335"),
        ),
    ];
    for &(input, sf, expected) in tests {
        let input = Decimal::from_str(input).unwrap();
        let result = input.round_sf(sf);
        if let Some(expected) = expected {
            assert!(result.is_some(), "Expected result for {input}.round_sf({sf})");
            assert_eq!(expected, result.unwrap().to_string(), "{input}.round_sf({sf})");
        } else {
            assert!(result.is_none(), "Unexpected result for {input}.round_sf({sf})");
        }
    }
}

#[test]
fn it_can_round_significant_figures_with_strategy() {
    let tests = &[
        ("12301", 3u32, RoundingStrategy::AwayFromZero, Some("12400")),
        ("123.01", 3u32, RoundingStrategy::AwayFromZero, Some("124")),
        ("1.2301", 3u32, RoundingStrategy::AwayFromZero, Some("1.24")),
        ("0.12301", 3u32, RoundingStrategy::AwayFromZero, Some("0.124")),
        ("0.012301", 3u32, RoundingStrategy::AwayFromZero, Some("0.0124")),
        ("0.0000012301", 3u32, RoundingStrategy::AwayFromZero, Some("0.00000124")),
        ("1.012301", 3u32, RoundingStrategy::AwayFromZero, Some("1.02")),
    ];
    for &(input, sf, strategy, expected) in tests {
        let input = Decimal::from_str(input).unwrap();
        let result = input.round_sf_with_strategy(sf, strategy);
        if let Some(expected) = expected {
            assert!(
                result.is_some(),
                "Expected result for {input}.round_sf_with_strategy({sf}, {strategy:?})"
            );
            assert_eq!(
                expected,
                result.unwrap().to_string(),
                "{input}.round_sf_with_strategy({sf}, {strategy:?})"
            );
        } else {
            assert!(
                result.is_none(),
                "Unexpected result for {input}.round_sf_with_strategy({sf}, {strategy:?})"
            );
        }
    }
}

#[test]
fn it_can_trunc() {
    let tests = &[
        ("1.00000000000000000000", "1"),
        ("1.000000000000000000000001", "1"),
        ("1.123456789", "1"),
        ("1.9", "1"),
        ("1", "1"),
        // Also the inflection
        ("-1.00000000000000000000", "-1"),
        ("-1.000000000000000000000001", "-1"),
        ("-1.123456789", "-1"),
        ("-1.9", "-1"),
        ("-1", "-1"),
    ];

    for &(value, expected) in tests {
        let value = Decimal::from_str(value).unwrap();
        let expected = Decimal::from_str(expected).unwrap();
        let trunc = value.trunc();
        assert_eq!(expected.to_string(), trunc.to_string());
    }
}

#[test]
fn it_can_trunc_with_scale() {
    let cmp = Decimal::from_str("1.2345").unwrap();
    let tests = [
        "1.23450",
        "1.234500001",
        "1.23451",
        "1.23454",
        "1.23455",
        "1.23456",
        "1.23459",
        "1.234599999",
    ];
    for test in tests {
        assert_eq!(
            Decimal::from_str(test).unwrap().trunc_with_scale(4),
            cmp,
            "Original: {}",
            test
        );
    }

    let cmp = Decimal::from_str("-1.2345").unwrap();
    let tests = [
        "-1.23450",
        "-1.234500001",
        "-1.23451",
        "-1.23454",
        "-1.23455",
        "-1.23456",
        "-1.23459",
        "-1.234599999",
    ];
    for test in tests {
        assert_eq!(
            Decimal::from_str(test).unwrap().trunc_with_scale(4),
            cmp,
            "Original: {}",
            test
        );
    }

    // Complex cases
    let cmp = Decimal::from_str("0.5156").unwrap();
    let tests = [
        "0.51560089",
        "0.515600893",
        "0.5156008936",
        "0.51560089369",
        "0.515600893691",
        "0.5156008936910",
        "0.51560089369101",
        "0.515600893691016",
        "0.5156008936910161",
        "0.51560089369101613",
        "0.515600893691016134",
        "0.5156008936910161349",
        "0.51560089369101613494",
        "0.515600893691016134941",
        "0.5156008936910161349411",
        "0.51560089369101613494115",
        "0.515600893691016134941151",
        "0.5156008936910161349411515",
        "0.51560089369101613494115158",
        "0.515600893691016134941151581",
        "0.5156008936910161349411515818",
    ];
    for test in tests {
        assert_eq!(
            Decimal::from_str(test).unwrap().trunc_with_scale(4),
            cmp,
            "Original: {}",
            test
        );
    }
}

#[test]
fn it_can_fract() {
    let tests = &[
        ("1.00000000000000000000", "0.00000000000000000000"),
        ("1.000000000000000000000001", "0.000000000000000000000001"),
    ];

    for &(value, expected) in tests {
        let value = Decimal::from_str(value).unwrap();
        let expected = Decimal::from_str(expected).unwrap();
        let fract = value.fract();
        assert_eq!(expected.to_string(), fract.to_string());
    }
}

#[test]
fn it_can_normalize() {
    let tests = &[
        ("1.00000000000000000000", "1"),
        ("1.10000000000000000000000", "1.1"),
        ("1.00010000000000000000000", "1.0001"),
        ("1", "1"),
        ("1.1", "1.1"),
        ("1.0001", "1.0001"),
        ("-0", "0"),
        ("-0.0", "0"),
        ("-0.010", "-0.01"),
        ("0.0", "0"),
    ];

    for &(value, expected) in tests {
        let value = Decimal::from_str(value).unwrap();
        let expected = Decimal::from_str(expected).unwrap();
        let normalized = value.normalize();
        assert_eq!(expected.to_string(), normalized.to_string());
    }
}

#[test]
fn it_can_return_the_max_value() {
    assert_eq!("79228162514264337593543950335", Decimal::MAX.to_string());
}

#[test]
fn it_can_return_the_min_value() {
    assert_eq!("-79228162514264337593543950335", Decimal::MIN.to_string());
}

#[test]
fn it_can_go_from_and_into() {
    let d = Decimal::from_str("5").unwrap();
    let di8: Decimal = 5u8.into();
    let di32: Decimal = 5i32.into();
    let disize: Decimal = 5isize.into();
    let di64: Decimal = 5i64.into();
    let du8: Decimal = 5u8.into();
    let du32: Decimal = 5u32.into();
    let dusize: Decimal = 5usize.into();
    let du64: Decimal = 5u64.into();

    assert_eq!(d, di8);
    assert_eq!(di8, di32);
    assert_eq!(di32, disize);
    assert_eq!(disize, di64);
    assert_eq!(di64, du8);
    assert_eq!(du8, du32);
    assert_eq!(du32, dusize);
    assert_eq!(dusize, du64);
}

#[test]
fn it_converts_to_f64() {
    let tests = &[
        ("5", Some(5f64)),
        ("-5", Some(-5f64)),
        ("0.1", Some(0.1f64)),
        ("0.0", Some(0f64)),
        ("-0.0", Some(0f64)),
        ("0.0000000000025", Some(0.25e-11f64)),
        ("1000000.0000000000025", Some(1e6f64)),
        ("0.000000000000000000000000025", Some(0.25e-25_f64)),
        (
            "2.1234567890123456789012345678",
            Some(2.1234567890123456789012345678_f64),
        ),
        ("21234567890123456789012345678", Some(21234567890123458000000000000_f64)),
        (
            "-21234567890123456789012345678",
            Some(-21234567890123458000000000000_f64),
        ),
        ("1.59283191", Some(1.59283191_f64)),
        ("2.2238", Some(2.2238_f64)),
        ("2.2238123", Some(2.2238123_f64)),
        ("22238", Some(22238_f64)),
        ("1000000", Some(1000000_f64)),
        ("1000000.000000000000000000", Some(1000000_f64)),
        ("10000", Some(10000_f64)),
        ("10000.000000000000000000", Some(10000_f64)),
        ("100000", Some(100000_f64)),
        ("100000.000000000000000000", Some(100000_f64)),
    ];
    for &(value, expected) in tests {
        let value = Decimal::from_str(value).unwrap().to_f64();
        assert_eq!(expected, value);
    }
}

#[test]
fn it_converts_to_f64_try() {
    let tests = &[
        ("5", Some(5f64)),
        ("-5", Some(-5f64)),
        ("0.1", Some(0.1f64)),
        ("0.0", Some(0f64)),
        ("-0.0", Some(0f64)),
        ("0.0000000000025", Some(0.25e-11f64)),
        ("1000000.0000000000025", Some(1e6f64)),
        ("0.000000000000000000000000025", Some(0.25e-25_f64)),
        (
            "2.1234567890123456789012345678",
            Some(2.1234567890123456789012345678_f64),
        ),
        ("21234567890123456789012345678", Some(21234567890123458000000000000_f64)),
        (
            "-21234567890123456789012345678",
            Some(-21234567890123458000000000000_f64),
        ),
        ("1.59283191", Some(1.59283191_f64)),
    ];
    for &(value, expected) in tests {
        let value = Decimal::from_str(value).unwrap().try_into().ok();
        assert_eq!(expected, value);
    }
}

#[test]
fn it_converts_to_i64() {
    let tests = [
        ("5", Some(5_i64)),
        ("-5", Some(-5_i64)),
        ("5.12345", Some(5_i64)),
        ("-5.12345", Some(-5_i64)),
        ("-9223372036854775808", Some(-9223372036854775808_i64)),
        ("-9223372036854775808", Some(i64::MIN)),
        ("9223372036854775807", Some(9223372036854775807_i64)),
        ("9223372036854775807", Some(i64::MAX)),
        ("-9223372036854775809", None), // i64::MIN - 1
        ("9223372036854775808", None),  // i64::MAX + 1
        // Clear overflows in hi bit
        ("-92233720368547758089", None),
        ("92233720368547758088", None),
    ];
    for (input, expected) in tests {
        let input = Decimal::from_str(input).unwrap();
        let actual = input.to_i64();
        assert_eq!(expected, actual, "Input: {input}");
    }
}

#[test]
fn it_converts_to_u64() {
    assert_eq!(5u64, Decimal::from_str("5").unwrap().to_u64().unwrap());
    assert_eq!(None, Decimal::from_str("-5").unwrap().to_u64());
    assert_eq!(5u64, Decimal::from_str("5.12345").unwrap().to_u64().unwrap());
    assert_eq!(
        0xFFFF_FFFF_FFFF_FFFF,
        Decimal::from_str("18446744073709551615").unwrap().to_u64().unwrap()
    );
    assert_eq!(None, Decimal::from_str("18446744073709551616").unwrap().to_u64());
}

#[test]
fn it_converts_to_i128() {
    let tests = &[
        ("5", Some(5i128)),
        ("-5", Some(-5i128)),
        ("5.12345", Some(5i128)),
        ("-5.12345", Some(-5i128)),
        ("9223372036854775807", Some(0x7FFF_FFFF_FFFF_FFFF)),
        ("92233720368547758089", Some(92233720368547758089i128)),
    ];
    for (dec, expected) in tests {
        assert_eq!(Decimal::from_str(dec).unwrap().to_i128(), *expected);
    }

    assert_eq!(
        79_228_162_514_264_337_593_543_950_335_i128,
        Decimal::MAX.to_i128().unwrap()
    );
}

#[test]
fn it_converts_to_u128() {
    let tests = &[
        ("5", Some(5u128)),
        ("-5", None),
        ("5.12345", Some(5u128)),
        ("-5.12345", None),
        ("18446744073709551615", Some(0xFFFF_FFFF_FFFF_FFFF)),
        ("18446744073709551616", Some(18446744073709551616u128)),
    ];
    for (dec, expected) in tests {
        assert_eq!(Decimal::from_str(dec).unwrap().to_u128(), *expected);
    }
    assert_eq!(
        79_228_162_514_264_337_593_543_950_335_u128,
        Decimal::MAX.to_u128().unwrap()
    );
}

#[test]
fn it_converts_from_i128() {
    let tests: &[(i128, Option<&str>)] = &[
        (5, Some("5")),
        (-5, Some("-5")),
        (0x7FFF_FFFF_FFFF_FFFF, Some("9223372036854775807")),
        (92233720368547758089, Some("92233720368547758089")),
        (0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF, Some("79228162514264337593543950335")),
        (0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, None),
        (i128::MIN, None),
        (i128::MAX, None),
    ];
    for (value, expected) in tests {
        let from_i128 = num_traits::FromPrimitive::from_i128(*value);

        match expected {
            Some(expected_value) => {
                let decimal = Decimal::from_str(expected_value).unwrap();
                assert_eq!(from_i128, Some(decimal));
            }
            None => assert!(from_i128.is_none()),
        }
    }
}

#[test]
fn it_converts_from_u128() {
    let tests: &[(u128, Option<&str>)] = &[
        (5, Some("5")),
        (0xFFFF_FFFF_FFFF_FFFF, Some("18446744073709551615")),
        (0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF, Some("79228162514264337593543950335")),
        (0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, None),
        (u128::MAX, None),
    ];
    for (value, expected) in tests {
        let from_u128 = num_traits::FromPrimitive::from_u128(*value);

        match expected {
            Some(expected_value) => {
                let decimal = Decimal::from_str(expected_value).unwrap();
                assert_eq!(from_u128, Some(decimal));
            }
            None => assert!(from_u128.is_none()),
        }
    }
}

#[test]
fn it_converts_from_str() {
    assert_eq!(Decimal::try_from("1").unwrap(), Decimal::ONE);
    assert_eq!(Decimal::try_from("10").unwrap(), Decimal::TEN);
}

#[test]
fn it_converts_from_f32() {
    use num_traits::FromPrimitive;

    let tests = [
        (0.1_f32, "0.1"),
        (1_f32, "1"),
        (0_f32, "0"),
        (0.12345_f32, "0.12345"),
        (0.1234567800123456789012345678_f32, "0.12345678"),
        (0.12345678901234567890123456789_f32, "0.12345679"),
        (0.00000000000000000000000000001_f32, "0"),
        (5.1_f32, "5.1"),
    ];

    for &(input, expected) in &tests {
        assert_eq!(
            expected,
            Decimal::from_f32(input).unwrap().to_string(),
            "from_f32({input})"
        );
        assert_eq!(
            expected,
            Decimal::try_from(input).unwrap().to_string(),
            "try_from({input})"
        );
    }
}

#[test]
fn it_converts_from_f32_limits() {
    use num_traits::FromPrimitive;

    assert!(Decimal::from_f32(f32::NAN).is_none(), "from_f32(f32::NAN)");
    assert!(Decimal::from_f32(f32::INFINITY).is_none(), "from_f32(f32::INFINITY)");
    assert!(Decimal::try_from(f32::NAN).is_err(), "try_from(f32::NAN)");
    assert!(Decimal::try_from(f32::INFINITY).is_err(), "try_from(f32::INFINITY)");

    // These overflow
    assert!(Decimal::from_f32(f32::MAX).is_none(), "from_f32(f32::MAX)");
    assert!(Decimal::from_f32(f32::MIN).is_none(), "from_f32(f32::MIN)");
    assert!(Decimal::try_from(f32::MAX).is_err(), "try_from(f32::MAX)");
    assert!(Decimal::try_from(f32::MIN).is_err(), "try_from(f32::MIN)");
}

#[test]
fn it_converts_from_f32_retaining_bits() {
    let tests = [
        (0.1_f32, "0.100000001490116119384765625"),
        (2_f32, "2"),
        (4.000_f32, "4"),
        (5.1_f32, "5.099999904632568359375"),
    ];

    for &(input, expected) in &tests {
        assert_eq!(
            expected,
            Decimal::from_f32_retain(input).unwrap().to_string(),
            "from_f32_retain({input})"
        );
    }
}

#[test]
fn it_converts_from_f64() {
    use num_traits::FromPrimitive;

    let tests = [
        (0.1_f64, "0.1"),
        (1_f64, "1"),
        (0_f64, "0"),
        (0.12345_f64, "0.12345"),
        (0.1234567890123456089012345678_f64, "0.1234567890123456"),
        (0.12345678901234567890123456789_f64, "0.1234567890123457"),
        (0.00000000000000000000000000001_f64, "0"),
        (0.6927_f64, "0.6927"),
        (0.00006927_f64, "0.00006927"),
        (0.000000006927_f64, "0.000000006927"),
        (5.1_f64, "5.1"),
    ];

    for &(input, expected) in &tests {
        assert_eq!(
            expected,
            Decimal::from_f64(input).unwrap().to_string(),
            "from_f64({input})"
        );
        assert_eq!(
            expected,
            Decimal::try_from(input).unwrap().to_string(),
            "try_from({input})"
        );
    }
}

#[test]
fn it_converts_from_f64_limits() {
    use num_traits::FromPrimitive;

    assert!(Decimal::from_f64(f64::NAN).is_none(), "from_f64(f64::NAN)");
    assert!(Decimal::from_f64(f64::INFINITY).is_none(), "from_f64(f64::INFINITY)");
    assert!(Decimal::try_from(f64::NAN).is_err(), "try_from(f64::NAN)");
    assert!(Decimal::try_from(f64::INFINITY).is_err(), "try_from(f64::INFINITY)");

    // These overflow
    assert!(Decimal::from_f64(f64::MAX).is_none(), "from_f64(f64::MAX)");
    assert!(Decimal::from_f64(f64::MIN).is_none(), "from_f64(f64::MIN)");
    assert!(Decimal::try_from(f64::MAX).is_err(), "try_from(f64::MIN)");
    assert!(Decimal::try_from(f64::MIN).is_err(), "try_from(f64::MAX)");
}

#[test]
fn it_converts_from_f64_dec_limits() {
    use num_traits::FromPrimitive;

    // Note Decimal MAX is: 79_228_162_514_264_337_593_543_950_335
    let over_max = 79_228_162_514_264_355_185_729_994_752_f64;
    let max_plus_one = 79_228_162_514_264_337_593_543_950_336_f64;
    let under_max = 79_228_162_514_264_328_797_450_928_128_f64;

    assert!(
        Decimal::from_f64(over_max).is_none(),
        "from_f64(79_228_162_514_264_355_185_729_994_752_f64) -> none (too large)"
    );
    assert!(
        Decimal::from_f64(max_plus_one).is_none(),
        "from_f64(79_228_162_514_264_337_593_543_950_336_f64) -> none (too large)"
    );
    assert_eq!(
        "79228162514264328797450928128",
        Decimal::from_f64(under_max).unwrap().to_string(),
        "from_f64(79_228_162_514_264_328_797_450_928_128_f64) -> some (inside limits)"
    );
}

#[test]
fn it_converts_from_f64_retaining_bits() {
    let tests = [
        (0.1_f64, "0.1000000000000000055511151231"),
        (2_f64, "2"),
        (4.000_f64, "4"),
        (5.1_f64, "5.0999999999999996447286321175"),
    ];

    for &(input, expected) in &tests {
        assert_eq!(
            expected,
            Decimal::from_f64_retain(input).unwrap().to_string(),
            "from_f64_retain({input})"
        );
    }
}

#[test]
fn it_converts_to_integers() {
    assert_eq!(i64::try_from(Decimal::ONE), Ok(1));
    assert_eq!(i64::try_from(Decimal::MAX), Err(Error::ConversionTo("i64".to_string())));
    assert_eq!(u128::try_from(Decimal::ONE_HUNDRED), Ok(100));
}

#[test]
fn it_handles_simple_underflow() {
    // Issue #71
    let rate = Decimal::new(19, 2); // 0.19
    let one = Decimal::new(1, 0); // 1
    let part = rate / (rate + one); // 0.19 / (0.19 + 1) = 0.1596638655462184873949579832
    let result = one * part;
    assert_eq!("0.1596638655462184873949579832", result.to_string());

    // 169 * 0.1596638655462184873949579832 = 26.983193277310924
    let result = part * Decimal::new(169, 0);
    assert_eq!("26.983193277310924369747899161", result.to_string());
    let result = Decimal::new(169, 0) * part;
    assert_eq!("26.983193277310924369747899161", result.to_string());
}

#[test]
fn it_can_parse_highly_significant_numbers() {
    let tests = &[
        ("11.111111111111111111111111111", "11.111111111111111111111111111"),
        ("11.11111111111111111111111111111", "11.111111111111111111111111111"),
        ("11.1111111111111111111111111115", "11.111111111111111111111111112"),
        ("115.111111111111111111111111111", "115.11111111111111111111111111"),
        ("1115.11111111111111111111111111", "1115.1111111111111111111111111"),
        ("11.1111111111111111111111111195", "11.111111111111111111111111120"),
        ("99.9999999999999999999999999995", "100.00000000000000000000000000"),
        ("-11.1111111111111111111111111195", "-11.111111111111111111111111120"),
        ("-99.9999999999999999999999999995", "-100.00000000000000000000000000"),
        ("3.1415926535897932384626433832", "3.1415926535897932384626433832"),
        (
            "8808257419827262908.5944405087133154018",
            "8808257419827262908.594440509",
        ),
        (
            "8097370036018690744.2590371109596744091",
            "8097370036018690744.259037111",
        ),
        (
            "8097370036018690744.2590371149596744091",
            "8097370036018690744.259037115",
        ),
        (
            "8097370036018690744.2590371159596744091",
            "8097370036018690744.259037116",
        ),
        ("1.234567890123456789012345678949999", "1.2345678901234567890123456789"),
        (".00000000000000000000000000001", "0.0000000000000000000000000000"),
        (".10000000000000000000000000000", "0.1000000000000000000000000000"),
    ];
    for &(value, expected) in tests {
        assert_eq!(expected, Decimal::from_str(value).unwrap().to_string());
    }
}

#[test]
fn it_can_parse_exact_highly_significant_numbers() {
    use rust_decimal::Error;

    let tests = &[
        (
            "11.111111111111111111111111111",
            Ok("11.111111111111111111111111111".to_string()),
        ),
        ("11.11111111111111111111111111111", Err(Error::Underflow)),
        ("11.1111111111111111111111111115", Err(Error::Underflow)),
        ("115.111111111111111111111111111", Err(Error::Underflow)),
        ("1115.11111111111111111111111111", Err(Error::Underflow)),
        ("11.1111111111111111111111111195", Err(Error::Underflow)),
        ("99.9999999999999999999999999995", Err(Error::Underflow)),
        ("-11.1111111111111111111111111195", Err(Error::Underflow)),
        ("-99.9999999999999999999999999995", Err(Error::Underflow)),
        (
            "3.1415926535897932384626433832",
            Ok("3.1415926535897932384626433832".to_string()),
        ),
        ("8808257419827262908.5944405087133154018", Err(Error::Underflow)),
        ("8097370036018690744.2590371109596744091", Err(Error::Underflow)),
        ("8097370036018690744.2590371149596744091", Err(Error::Underflow)),
        ("8097370036018690744.2590371159596744091", Err(Error::Underflow)),
        ("1.234567890123456789012345678949999", Err(Error::Underflow)),
        (".00000000000000000000000000001", Err(Error::Underflow)),
        (".10000000000000000000000000000", Err(Error::Underflow)),
    ];
    for &(value, ref expected) in tests.iter() {
        let actual = Decimal::from_str_exact(value).map(|d| d.to_string());
        assert_eq!(*expected, actual);
    }
}

#[test]
fn it_can_parse_alternative_formats() {
    let tests = &[
        ("1_000", "1000"),
        ("1_000_000", "1000000"),
        ("10_000_000", "10000000"),
        ("100_000", "100000"),
        // At the moment, we'll accept this
        ("1_____________0", "10"),
    ];
    for &(value, expected) in tests {
        assert_eq!(expected, Decimal::from_str(value).unwrap().to_string());
    }
}

#[test]
fn it_can_parse_fractional_numbers_with_underscore_separators() {
    let a = Decimal::from_str("0.1_23_456").unwrap();
    assert!(!a.is_sign_negative());
    assert_eq!(a.scale(), 6);
    assert_eq!("0.123456", a.to_string());
}

#[test]
fn it_can_parse_numbers_with_underscore_separators_before_decimal_point() {
    let a = Decimal::from_str("1_234.56").unwrap();
    assert!(!a.is_sign_negative());
    assert_eq!(a.scale(), 2);
    assert_eq!("1234.56", a.to_string());
}

#[test]
fn it_can_parse_numbers_and_round_correctly_with_underscore_separators_before_decimal_point() {
    let tests = &[
        (
            "8_097_370_036_018_690_744.2590371159596744091",
            "8097370036018690744.259037116",
        ),
        (
            "8097370036018690744.259_037_115_959_674_409_1",
            "8097370036018690744.259037116",
        ),
        (
            "8_097_370_036_018_690_744.259_037_115_959_674_409_1",
            "8097370036018690744.259037116",
        ),
    ];
    for &(value, expected) in tests {
        assert_eq!(expected, Decimal::from_str(value).unwrap().to_string());
    }
}

#[test]
fn it_can_reject_invalid_formats() {
    let tests = &["_1", "1.0.0", "10_00.0_00.0"];
    for &value in tests {
        assert!(
            Decimal::from_str(value).is_err(),
            "This succeeded unexpectedly: {value}"
        );
    }
}

#[test]
fn it_can_reject_large_numbers_with_panic() {
    let tests = &[
        // The maximum number supported is 79,228,162,514,264,337,593,543,950,335
        "79228162514264337593543950336",
        "79228162514264337593543950337",
        "79228162514264337593543950338",
        "79228162514264337593543950339",
        "79228162514264337593543950340",
    ];
    for &value in tests {
        if let Ok(out) = Decimal::from_str(value) {
            panic!("Unexpectedly parsed {value} into {out}")
        }
    }
}

#[test]
fn it_can_parse_individual_parts() {
    let pi = Decimal::from_parts(1102470952, 185874565, 1703060790, false, 28);
    assert_eq!(pi.to_string(), "3.1415926535897932384626433832");
}

#[test]
fn it_can_parse_scientific_notation() {
    let tests = &[
        ("9.7e-7", Ok("0.00000097".to_string())),
        ("9e-7", Ok("0.0000009".to_string())),
        ("1.2e10", Ok("12000000000".to_string())),
        ("1.2e+10", Ok("12000000000".to_string())),
        ("12e10", Ok("120000000000".to_string())),
        ("9.7E-7", Ok("0.00000097".to_string())),
        ("1.2345E-24", Ok("0.0000000000000000000000012345".to_string())),
        ("12345E-28", Ok("0.0000000000000000000000012345".to_string())),
        ("1.2345E0", Ok("1.2345".to_string())),
        ("1E28", Ok("10000000000000000000000000000".to_string())),
        (
            "-20165.4676_e-+4294967292",
            Err(Error::ScaleExceedsMaximumPrecision(4294967292)),
        ),
    ];

    for &(value, ref expected) in tests {
        let actual = Decimal::from_scientific(value).map(|d| d.to_string());
        assert_eq!(*expected, actual);
    }
}

#[test]
fn it_errors_parsing_large_scientific_notation() {
    let result = Decimal::from_scientific("1.2345E-28");
    assert!(result.is_err());
    assert_eq!(
        result.err(),
        Some(Error::ScaleExceedsMaximumPrecision(32)) // 4 + 28
    );

    let result = Decimal::from_scientific("12345E29");
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::ScaleExceedsMaximumPrecision(29)));

    let result = Decimal::from_scientific("12345E28");
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::ExceedsMaximumPossibleValue));
}

#[test]
fn it_can_parse_scientific_notation_rounded() {
    let tests = &[
        ("9.7e-7", Ok("0.00000097".to_string())),
        ("9e-7", Ok("0.0000009".to_string())),
        ("1.2e10", Ok("12000000000".to_string())),
        ("1.2e+10", Ok("12000000000".to_string())),
        ("12e10", Ok("120000000000".to_string())),
        ("9.7E-7", Ok("0.00000097".to_string())),
        ("1.2345E-24", Ok("0.0000000000000000000000012345".to_string())),
        ("12345E-28", Ok("0.0000000000000000000000012345".to_string())),
        ("1.2345E0", Ok("1.2345".to_string())),
        ("1E28", Ok("10000000000000000000000000000".to_string())),
        ("1.2345E-28", Ok("0.0000000000000000000000000001".to_string())),
        ("8.7654E-28", Ok("0.0000000000000000000000000009".to_string())),
        (
            "-20165.4676_e-+4294967292",
            Err(Error::ScaleExceedsMaximumPrecision(4294967292)),
        ),
    ];

    for &(value, ref expected) in tests {
        let actual = Decimal::from_scientific_lossy(value).map(|d| d.to_string());
        assert_eq!(*expected, actual);
    }
}

#[test]
fn it_errors_parsing_large_scientific_notation_rounded() {
    let result = Decimal::from_scientific_lossy("1.2345E-29");
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::ScaleExceedsMaximumPrecision(29)));

    let result = Decimal::from_scientific_lossy("12345E29");
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::ScaleExceedsMaximumPrecision(29)));

    let result = Decimal::from_scientific_lossy("12345E28");
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::ExceedsMaximumPossibleValue));
}

#[test]
fn it_can_parse_different_radix() {
    let tests = &[
        // Input, Radix, Success, to_string()
        ("123", 10, true, "123"),
        ("123", 8, true, "83"),
        ("123", 16, true, "291"),
        ("abc", 10, false, ""),
        ("abc", 16, true, "2748"),
        ("78", 10, true, "78"),
        ("78", 8, false, ""),
        ("101", 2, true, "5"),
        // Parse base 2
        ("1111_1111_1111_1111_1111_1111_1111_1111", 2, true, "4294967295"),
        // Max supported value
        (
            "1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_\
          1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111",
            2,
            true,
            &Decimal::MAX.to_string(),
        ),
        // We limit to 28 dp
        (
            "843.6500000000000000000000000000",
            10,
            true,
            "843.6500000000000000000000000",
        ),
    ];

    for &(input, radix, success, expected) in tests {
        let result = Decimal::from_str_radix(input, radix);
        assert_eq!(
            success,
            result.is_ok(),
            "Failed to parse: {} radix {}: {:?}",
            input,
            radix,
            result.err()
        );
        if result.is_ok() {
            assert_eq!(
                expected,
                result.unwrap().to_string(),
                "Original input: {input} radix {radix}"
            );
        }
    }
}

#[test]
fn it_can_calculate_signum() {
    let tests = &[("123", 1), ("-123", -1), ("0", 0)];

    for &(input, expected) in tests {
        let input = Decimal::from_str(input).unwrap();
        assert_eq!(expected, input.signum().to_i32().unwrap(), "Input: {input}");
    }
}

#[test]
fn it_can_calculate_abs_sub() {
    let tests = &[
        ("123", "124", 0),
        ("123", "123", 0),
        ("123", "122", 1),
        ("-123", "-124", 1),
        ("-123", "-123", 0),
        ("-123", "-122", 0),
    ];

    for &(input1, input2, expected) in tests {
        let input1 = Decimal::from_str(input1).unwrap();
        let input2 = Decimal::from_str(input2).unwrap();
        assert_eq!(
            expected,
            input1.abs_sub(&input2).to_i32().unwrap(),
            "Input: {input1} {input2}"
        );
    }
}

#[test]
#[should_panic(expected = "Scale exceeds the maximum precision allowed: 29 > 28")]
fn it_panics_when_scale_too_large() {
    let _ = Decimal::new(1, 29);
}

#[test]
fn test_zero_eq_negative_zero() {
    let zero: Decimal = 0.into();

    assert_eq!(zero, zero);
    assert_eq!(-zero, zero);
    assert_eq!(zero, -zero);
}

#[test]
fn declarative_dec_product() {
    let vs = (1..5).map(|i| i.into()).collect::<Vec<Decimal>>();
    let product: Decimal = vs.into_iter().product();
    assert_eq!(product, Decimal::from(24))
}

#[test]
fn declarative_ref_dec_product() {
    let vs = (1..5).map(|i| i.into()).collect::<Vec<Decimal>>();
    let product: Decimal = vs.iter().product();
    assert_eq!(product, Decimal::from(24))
}

#[test]
fn declarative_dec_sum() {
    let vs = (0..10).map(|i| i.into()).collect::<Vec<Decimal>>();
    let sum: Decimal = vs.into_iter().sum();
    assert_eq!(sum, Decimal::from(45))
}

#[test]
fn declarative_ref_dec_sum() {
    let vs = (0..10).map(|i| i.into()).collect::<Vec<Decimal>>();
    let sum: Decimal = vs.iter().sum();
    assert_eq!(sum, Decimal::from(45))
}

#[cfg(feature = "db-postgres")]
#[test]
fn postgres_to_from_sql() {
    use bytes::BytesMut;
    use postgres::types::{FromSql, Kind, ToSql, Type};

    let tests = &[
        "3950.123456",
        "3950",
        "0.1",
        "0.01",
        "0.001",
        "0.0001",
        "0.00001",
        "0.000001",
        "1",
        "-100",
        "-123.456",
        "119996.25",
        "1000000",
        "9999999.99999",
        "12340.56789",
        "79228162514264337593543950335", // 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF (96 bit)
        "4951760157141521099596496895",  // 0x0FFF_FFFF_FFFF_FFFF_FFFF_FFFF (95 bit)
        "4951760157141521099596496896",  // 0x1000_0000_0000_0000_0000_0000
        "18446744073709551615",
        "-18446744073709551615",
    ];

    let t = Type::new("".into(), 0, Kind::Simple, "".into());

    for test in tests {
        let input = Decimal::from_str(test).unwrap();
        let mut bytes = BytesMut::new();
        input.to_sql(&t, &mut bytes).unwrap();
        let output = Decimal::from_sql(&t, &bytes).unwrap();

        assert_eq!(input, output);
    }
}

#[cfg(feature = "db-postgres")]
#[test]
fn postgres_from_sql_special_numeric() {
    use postgres::types::{FromSql, Kind, Type};

    // The numbers below are the big-endian equivalent of the NUMERIC_* masks for NAN, PINF, NINF
    let tests = &[
        ("NaN", &[0, 0, 0, 0, 192, 0, 0, 0]),
        ("Infinity", &[0, 0, 0, 0, 208, 0, 0, 0]),
        ("-Infinity", &[0, 0, 0, 0, 240, 0, 0, 0]),
    ];

    let t = Type::new("".into(), 0, Kind::Simple, "".into());

    for (name, bytes) in tests {
        let res = Decimal::from_sql(&t, *bytes);
        match &res {
            Ok(_) => panic!("Expected error, got Ok"),
            Err(e) => {
                let error_message = e.to_string();
                assert!(
                    error_message.contains(name),
                    "Error message does not contain the expected value: {}",
                    name
                );
            }
        }
    }
}

fn hash_it(d: Decimal) -> u64 {
    use core::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let mut h = DefaultHasher::new();
    d.hash(&mut h);
    h.finish()
}

#[test]
fn it_computes_equal_hashes_for_equal_values() {
    // From the Rust Hash docs:
    //
    // "When implementing both Hash and Eq, it is important that the following property holds:
    //
    //     k1 == k2 -> hash(k1) == hash(k2)"

    let k1 = Decimal::from_str("1").unwrap();
    let k2 = Decimal::from_str("1.0").unwrap();
    let k3 = Decimal::from_str("1.00").unwrap();
    let k4 = Decimal::from_str("1.01").unwrap();

    assert_eq!(k1, k2);
    assert_eq!(k1, k3);
    assert_ne!(k1, k4);

    let h1 = hash_it(k1);
    let h2 = hash_it(k2);
    let h3 = hash_it(k3);
    let h4 = hash_it(k4);

    assert_eq!(h1, h2);
    assert_eq!(h1, h3);
    assert_ne!(h1, h4);

    // Test the application of Hash calculation to a HashMap.

    let mut map = std::collections::HashMap::new();

    map.insert(k1, k1.to_string());
    // map[k2] should overwrite map[k1] because k1 == k2.
    map.insert(k2, k2.to_string());

    assert_eq!("1.0", map.get(&k3).expect("could not get k3"));
    assert_eq!(1, map.len());

    // map[k3] should overwrite map[k2] because k3 == k2.
    map.insert(k3, k3.to_string());
    // map[k4] should not overwrite map[k3] because k4 != k3.
    map.insert(k4, k4.to_string());

    assert_eq!(2, map.len());
    assert_eq!("1.00", map.get(&k1).expect("could not get k1"));
}

#[test]
fn it_computes_equal_hashes_for_positive_and_negative_zero() {
    // Verify 0 and -0 have the same hash
    let k1 = Decimal::from_str("0").unwrap();
    let k2 = Decimal::from_str("-0").unwrap();
    assert_eq!(k1, k2);
    let h1 = hash_it(k1);
    let h2 = hash_it(k2);
    assert_eq!(h1, h2);

    // Verify 0 and -0.0 have the same hash
    let k1 = Decimal::from_str("0").unwrap();
    let k2 = Decimal::from_str("-0.0").unwrap();
    assert_eq!(k1, k2);
    let h1 = hash_it(k1);
    let h2 = hash_it(k2);
    assert_eq!(h1, h2);
}

#[test]
#[should_panic(expected = "Number less than minimum value that can be represented.")]
fn it_handles_i128_min() {
    let _ = Decimal::from_i128_with_scale(i128::MIN, 0);
}

#[test]
fn it_handles_i128_min_safely() {
    let result = Decimal::try_from_i128_with_scale(i128::MIN, 0);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Error::LessThanMinimumPossibleValue);
}

#[test]
fn it_can_rescale() {
    let tests = &[
        ("0", 6, "0.000000", 6),
        ("0.000000", 2, "0.00", 2),
        ("0.12345600000", 6, "0.123456", 6),
        ("0.123456", 12, "0.123456000000", 12),
        ("0.123456", 0, "0", 0),
        ("0.000001", 4, "0.0000", 4),
        ("1233456", 4, "1233456.0000", 4),
        // Cap to 28
        ("1.2", 30, "1.2000000000000000000000000000", 28),
        ("79228162514264337593543950335", 0, "79228162514264337593543950335", 0),
        ("4951760157141521099596496895", 1, "4951760157141521099596496895.0", 1),
        ("4951760157141521099596496896", 1, "4951760157141521099596496896.0", 1),
        ("18446744073709551615", 6, "18446744073709551615.000000", 6),
        ("-18446744073709551615", 6, "-18446744073709551615.000000", 6),
        // 27 since we can't fit a scale of 28 for this number
        ("11.76470588235294", 28, "11.764705882352940000000000000", 27),
    ];

    for &(value_raw, new_scale, expected_value, expected_scale) in tests {
        let mut value = Decimal::from_str(value_raw).unwrap();
        value.rescale(new_scale);
        assert_eq!(expected_value, value.to_string());
        assert_eq!(expected_scale, value.scale());
    }
}

#[test]
fn test_constants() {
    assert_eq!("0", Decimal::ZERO.to_string());
    assert_eq!("1", Decimal::ONE.to_string());
    assert_eq!("-1", Decimal::NEGATIVE_ONE.to_string());
    assert_eq!("10", Decimal::TEN.to_string());
    assert_eq!("100", Decimal::ONE_HUNDRED.to_string());
    assert_eq!("1000", Decimal::ONE_THOUSAND.to_string());
    assert_eq!("2", Decimal::TWO.to_string());
}

#[test]
fn test_inv() {
    assert_eq!("0.01", Decimal::ONE_HUNDRED.inv().to_string());
}

#[test]
fn test_is_integer() {
    let tests = &[
        ("0", true),
        ("1", true),
        ("79_228_162_514_264_337_593_543_950_335", true),
        ("1.0", true),
        ("1.1", false),
        ("3.1415926535897932384626433833", false),
        ("3.0000000000000000000000000000", true),
        ("0.400000000", false),
        ("0.4000000000", false),
        ("0.4000000000000000000", false),
        ("0.4000000000000000001", false),
    ];
    for &(raw, integer) in tests {
        let value = Decimal::from_str(raw).unwrap();
        assert_eq!(value.is_integer(), integer, "value: {raw}")
    }
}

// Mathematical features
#[cfg(feature = "maths")]
mod maths {
    use super::*;
    use rust_decimal::MathematicalOps;

    use num_traits::One;

    #[test]
    fn test_constants() {
        assert_eq!("3.1415926535897932384626433833", Decimal::PI.to_string());
        assert_eq!("6.2831853071795864769252867666", Decimal::TWO_PI.to_string());
        assert_eq!("1.5707963267948966192313216916", Decimal::HALF_PI.to_string());
        assert_eq!("2.7182818284590452353602874714", Decimal::E.to_string());
        assert_eq!("0.3678794411714423215955237702", Decimal::E_INVERSE.to_string());
    }

    #[test]
    fn test_powu() {
        let test_cases = &[
            // x, y, expected x ^ y
            ("4", 3_u64, "64"),
            ("3.222", 5_u64, "347.238347228449632"),
            ("0.1", 0_u64, "1"),
            ("342.4", 1_u64, "342.4"),
            ("2.0", 16_u64, "65536"),
            ("0.99999999999999", 1477289400_u64, "0.9999852272151186611602884841"),
            ("0.99999999999999", 0x8000_8000_0000_0000, "0"),
        ];
        for &(x, y, expected) in test_cases {
            let x = Decimal::from_str(x).unwrap();
            let pow = x.powu(y);
            assert_eq!(pow.to_string(), expected, "{} ^ {}", x, y);
        }
    }

    #[test]
    #[should_panic(expected = "Pow overflowed")]
    fn test_powu_panic() {
        let two = Decimal::new(2, 0);
        let _ = two.powu(128);
    }

    #[test]
    fn test_checked_powu() {
        let test_cases = &[
            (Decimal::new(4, 0), 3_u64, Some(Decimal::new(64, 0))),
            (
                Decimal::from_str("3.222").unwrap(),
                5_u64,
                Some(Decimal::from_str("347.238347228449632").unwrap()),
            ),
            (
                Decimal::from_str("0.1").unwrap(),
                0_u64,
                Some(Decimal::from_str("1").unwrap()),
            ),
            (
                Decimal::from_str("342.4").unwrap(),
                1_u64,
                Some(Decimal::from_str("342.4").unwrap()),
            ),
            (
                Decimal::from_str("2.0").unwrap(),
                16_u64,
                Some(Decimal::from_str("65536").unwrap()),
            ),
            (Decimal::from_str("2.0").unwrap(), 128_u64, None),
        ];
        for case in test_cases {
            assert_eq!(case.2, case.0.checked_powu(case.1));
        }
    }

    #[test]
    fn test_powi() {
        let test_cases = &[
            // x, y, expected x ^ y
            ("0", 0, "1"),
            ("1", 0, "1"),
            ("0", 1, "0"),
            ("2", 3, "8"),
            ("-2", 3, "-8"),
            ("2", -3, "0.125"),
            ("-2", -3, "-0.125"),
            (
                "3",
                -3,
                either!("0.037037037037037037037037037", "0.0370370370370370370370370370"),
            ),
            ("6", 3, "216"),
            ("0.5", 2, "0.25"),
        ];
        for &(x, y, expected) in test_cases {
            let x = Decimal::from_str(x).unwrap();
            let pow = x.powi(y);
            assert_eq!(pow.to_string(), expected, "{} ^ {}", x, y);
        }
    }

    #[test]
    fn test_powd() {
        let test_cases = &[
            // x, y, expected x ^ y
            ("0", "0", "1"),
            ("1", "0", "1"),
            ("0", "1", "0"),
            ("2", "3", "8"),
            ("-2", "3", "-8"),
            ("2", "-3", "0.125"),
            ("-2", "-3", "-0.125"),
            ("2.0", "3.0", "8"),
            ("-2.0", "3.0", "-8"),
            ("2.0", "-3.0", "0.125"),
            ("-2.0", "-3.0", "-0.125"),
            ("2.00", "3.00", "8"),
            ("-2.00", "3.00", "-8"),
            ("2.00", "-3.00", "0.125"),
            ("-2.00", "-3.00", "-0.125"),
            (
                "3",
                "-3",
                either!("0.037037037037037037037037037", "0.0370370370370370370370370370"),
            ),
            ("6", "3", "216"),
            ("0.5", "2", "0.25"),
            ("6", "13", "13060694016"),
            // Exact result: 1 / 6^7
            ("6", "-7", "0.0000035722450845907636031093"),
            ("0.16", "12.5", "0.0000000001125899906842624007"),
            // ~= 0.8408964152537145
            (
                "0.5",
                "0.25",
                either!("0.8408964159265360661551317741", "0.8408964159265360661551317743"),
            ),
            // ~= 0.999999999999999999999999999790814
            (
                "0.1234567890123456789012345678",
                "0.0000000000000000000000000001",
                "0.9999999999999999999999999998",
            ),
            // ~= 611.0451043224257
            (
                "1234.5678",
                "0.9012",
                either!("611.04510428561357675577986292", "611.04510428561357675577986284"),
            ),
            (
                "-2",
                "0.5",
                either!("-1.4142135570048917090885260834", "-1.4142135570048917090885260830"),
            ),
            // ~= -1.1193003023312942
            (
                "-2.5",
                "0.123",
                either!("-1.1193002994383985239135362087", "-1.1193002994383985239135362083"),
            ),
            // ~= 0.0003493091
            (
                "0.0000000000000000000000000001",
                "0.1234567890123456789012345678",
                either!("0.0003533642785253753219626202", "0.0003305188582777444871658669"),
            ),
            ("0.99999999999999", "1477289400", "0.9999852272151186611602884841"),
        ];
        for &(x, y, expected) in test_cases {
            let x = Decimal::from_str(x).unwrap();
            let y = Decimal::from_str(y).unwrap();
            let pow = x.powd(y);
            assert_eq!(pow.to_string(), expected, "{} ^ {}", x, y);
        }
    }

    #[test]
    fn test_sqrt() {
        let test_cases = &[
            ("4", "2"),
            ("3.222", "1.7949930361981909371487724124"),
            ("199.45", "14.122676800097069416754994263"),
            ("342.4", "18.504053609952604112132102540"),
            ("2", "1.414213562373095048801688724209698078569671875376948073176"),
            ("0.0000000000000000000000000001", "0.0000000000000100000000000000"),
        ];
        for case in test_cases {
            let a = Decimal::from_str(case.0).unwrap();
            let expected = Decimal::from_str(case.1).unwrap();
            assert_eq!(expected, a.sqrt().unwrap());
        }

        assert_eq!(Decimal::new(-2, 0).sqrt(), None);
    }

    #[cfg(not(feature = "legacy-ops"))]
    #[test]
    fn test_exp() {
        // These are approximations
        let test_cases = &[
            // e^20 ~= 485165195.4097902
            ("20", "485165195.40979022078611628964"),
            // e^10 ~= 22026.465794806703
            ("10", "22026.465794741520789276707505"),
            // e^11 ~= 59874.14171519778
            ("11", "59874.141715145577936788584621"),
            // e^3 ~= 20.085536923187664
            ("3", "20.085536911963143539758560764"),
            // e^8 ~= 2980.957987041727
            ("8", "2980.9579870195167711895866166"),
            // e^0.1 ~= 1.1051709180756477
            ("0.1", "1.1051709166666666666666666667"),
            // e^2.0 ~= 7.3890560989306495
            ("2.0", "7.3890560703259115957528655948"),
            // e^-2 ~= 0.1353352832366127
            ("-2", "0.1353352837605267572029589224"),
            // e^-1 ~= 0.36787944117144233
            ("-1", "0.3678794414773748171422559335"),
            // e^0.123456789 ~= 1.131401115
            ("0.123456789", "1.1314011144241455834073838005"),
            // e^0.123456789123456789123456789 ~= 1.131401114651912752617990081
            ("0.123456789123456789123456789", "1.1314011145638247316063947841"),
        ];
        for &(x, expected) in test_cases {
            let x = Decimal::from_str(x).unwrap();
            let expected = Decimal::from_str(expected).unwrap();
            assert_eq!(expected, x.exp());
            assert_eq!(Some(expected), x.checked_exp());
        }
    }

    #[cfg(not(feature = "legacy-ops"))]
    #[test]
    fn test_exp_with_tolerance() {
        let test_cases = &[
            // e^0 = 1
            ("0", "0.0002", "1"),
            // e^1 ~= 2.7182539682539682539682539683
            ("1", "0.0002", "2.7182539682539682539682539683"),
            // e^10 ~= 22026.4657948067165169579006453
            ("10", "0.0002", "22026.465747586389870727226329"),
            // e^11 ~= 59874.1417151978184553264857923
            ("11", "0.0002", "59874.141680567932663075014271"),
            // e^11.7578 ~= 127746.03548949540892948423052
            ("11.7578", "0.0002", "127746.10261578321551805466515"),
            // e^3 ~= 20.085536923187664
            ("3", "0.00002", "20.085534430970814899386327956"),
            // e^8 ~= 2980.957987041727
            ("8", "0.0002", "2980.9579632726949323840240320"),
            // e^0.1 ~= 1.1051709180756477
            ("0.1", "0.0002", "1.1051666666666666666666666667"),
            // e^2.0 ~= 7.3890560989306495
            ("2.0", "0.0002", "7.3890460157126823793490460164"),
            // e^66.542 ~= 7.92179e28
            ("66.542", "0.0002", "79217916301129338946322758261"),
            // e^66.543 overflows
            ("66.543", "0.0002", ""),
            // e^123 overflows
            ("123", "0.0002", ""),
            // e^-8 ~= 0.000335462627902511838821389125781
            ("-8", "0.0002", "0.0003354626305773641802399811"),
            // e^-9 ~= 0.000123409804086679549497636690730
            ("-9", "0.0002", "0.0001234098050655108657269324"),
            // e^-11 ~= 0.0000167017007902456593126355173606
            ("-11", "0.0002", "0.0000167017007999055554659406"),
            // e^66.542 ~= 1.26234*10^-29
            ("-66.542", "0.0002", "0"),
            // e^66.543 underflows
            ("-66.543", "0.0002", ""),
            // e^-1024 underflows
            ("-1024", "0.0002", ""),
        ];
        for &(x, tolerance, expected) in test_cases {
            let x = Decimal::from_str(x).unwrap();
            let tolerance = Decimal::from_str(tolerance).unwrap();
            let expected = if expected.is_empty() {
                None
            } else {
                Some(Decimal::from_str(expected).unwrap())
            };

            if let Some(expected) = expected {
                assert_eq!(expected, x.exp_with_tolerance(tolerance));
                assert_eq!(Some(expected), x.checked_exp_with_tolerance(tolerance));
            } else {
                assert_eq!(None, x.checked_exp_with_tolerance(tolerance));
            }
        }
    }

    #[test]
    #[should_panic(expected = "Exp overflowed")]
    fn test_exp_expected_panic_from_overflow() {
        let d = Decimal::from_str("1024").unwrap();
        let _ = d.exp();
    }

    #[test]
    #[should_panic(expected = "Exp underflowed")]
    fn test_exp_expected_panic_from_underflow() {
        let d = Decimal::from_str("-1024").unwrap();
        let _ = d.exp();
    }

    #[test]
    #[cfg(not(feature = "legacy-ops"))]
    fn test_norm_cdf() {
        let test_cases = &[
            (
                Decimal::from_str("-0.4").unwrap(),
                Decimal::from_str("0.3445781286821245037094401704").unwrap(),
            ),
            (
                Decimal::from_str("-0.1").unwrap(),
                Decimal::from_str("0.4601722899186706579921922696").unwrap(),
            ),
            (
                Decimal::from_str("0.1").unwrap(),
                Decimal::from_str("0.5398277100813293420078077304").unwrap(),
            ),
            (
                Decimal::from_str("0.4").unwrap(),
                Decimal::from_str("0.6554218713178754962905598296").unwrap(),
            ),
            (
                Decimal::from_str("2.0").unwrap(),
                Decimal::from_str("0.9772497381095865280953380673").unwrap(),
            ),
        ];
        for case in test_cases {
            assert_eq!(case.1, case.0.norm_cdf());
        }
    }

    #[test]
    fn test_norm_pdf() {
        let test_cases = &[
            (
                Decimal::from_str("-2.0").unwrap(),
                Decimal::from_str("0.0539909667221995238993056051").unwrap(),
            ),
            (
                Decimal::from_str("-0.4").unwrap(),
                Decimal::from_str("0.3682701404285264134348468378").unwrap(),
            ),
            (
                Decimal::from_str("-0.1").unwrap(),
                Decimal::from_str("0.3969525474873078082322691394").unwrap(),
            ),
            (
                Decimal::from_str("0.1").unwrap(),
                Decimal::from_str("0.3969525474873078082322691394").unwrap(),
            ),
            (
                Decimal::from_str("0.4").unwrap(),
                Decimal::from_str("0.3682701404285264134348468378").unwrap(),
            ),
            (
                Decimal::from_str("2.0").unwrap(),
                Decimal::from_str("0.0539909667221995238993056051").unwrap(),
            ),
        ];
        for case in test_cases {
            assert_eq!(case.1, case.0.norm_pdf());
        }
    }

    #[test]
    fn test_ln() {
        let test_cases = [
            ("1", "0"),
            // Wolfram Alpha gives -1.46968
            (
                "0.23",
                either!("-1.4696759700589416772292300779", "-1.4696759700589416772292300777"),
            ),
            // Wolfram Alpha gives 0.693147180559945309417232121458176568075500134360255254120
            ("2", "0.6931471805599453094172321218"),
            // Wolfram Alpha gives 3.218875824868200749201518666452375279051202708537035443825
            (
                "25",
                either!("3.2188758248682007492015186670", "3.2188758248682007492015186674"),
            ),
            // Wolfram Alpha gives 0.210721022
            (
                "1.234567890",
                either!("0.2107210222156525610500017104", "0.2107210222156525610500017106"),
            ),
        ];

        for (input, expected) in test_cases {
            let input = Decimal::from_str(input).unwrap();
            let expected = Decimal::from_str(expected).unwrap();
            assert_eq!(expected, input.ln(), "Failed to calculate ln({})", input);
        }
    }

    #[test]
    #[cfg(feature = "maths-nopanic")]
    fn test_invalid_ln_nopanic() {
        let test_cases = ["0", "-2.0"];

        for input in test_cases {
            let input = Decimal::from_str(input).unwrap();
            assert_eq!("0", input.ln().to_string(), "Failed to calculate ln({})", input);
        }
    }

    #[test]
    #[should_panic(expected = "Unable to calculate ln for zero")]
    #[cfg(not(feature = "maths-nopanic"))]
    fn test_invalid_ln_zero_panic() {
        let _ = Decimal::ZERO.ln();
    }

    #[test]
    #[should_panic(expected = "Unable to calculate ln for negative numbers")]
    #[cfg(not(feature = "maths-nopanic"))]
    fn test_invalid_ln_negative_panic() {
        let _ = Decimal::NEGATIVE_ONE.ln();
    }

    #[test]
    fn test_log10() {
        let test_cases = [
            ("1", "0"),
            // Wolfram Alpha: 0.3010299956639811952137388947
            (
                "2",
                either!("0.3010299956639811952137388949", "0.3010299956639811952137388948"),
            ),
            // Wolfram Alpha: 0.0915149772
            (
                "1.234567890",
                either!("0.0915149771692704475183336230", "0.0915149771692704475183336231"),
            ),
            ("10", "1"),
            ("100", "2"),
            ("1000", "3"),
            ("1000.00000000", "3"),
            ("1000.000000000000000000000", "3"),
            ("10.000000000000000000000000000", "1"),
            ("100000000000000.0000000000", "14"),
            ("0.10", "-1"),
            ("0.1", "-1"),
            ("0.01", "-2"),
            ("0.010", "-2"),
            ("0.0100000000000000000", "-2"),
            ("0.000001", "-6"),
            ("0.000001000000000", "-6"),
        ];

        for (input, expected) in test_cases {
            let input = Decimal::from_str(input).unwrap();
            let expected = Decimal::from_str(expected).unwrap();
            assert_eq!(expected, input.log10(), "Failed to calculate log10({})", input);
        }
    }

    #[test]
    #[cfg(feature = "maths-nopanic")]
    fn test_invalid_log10_nopanic() {
        let test_cases = ["0", "-2.0"];

        for input in test_cases {
            let input = Decimal::from_str(input).unwrap();
            assert_eq!("0", input.log10().to_string(), "Failed to calculate ln({})", input);
        }
    }

    #[test]
    #[should_panic(expected = "Unable to calculate log10 for zero")]
    #[cfg(not(feature = "maths-nopanic"))]
    fn test_invalid_log10_zero_panic() {
        let _ = Decimal::ZERO.log10();
    }

    #[test]
    #[should_panic(expected = "Unable to calculate log10 for negative numbers")]
    #[cfg(not(feature = "maths-nopanic"))]
    fn test_invalid_log10_negative_panic() {
        let _ = Decimal::NEGATIVE_ONE.log10();
    }

    #[test]
    fn test_erf() {
        let test_cases = &[
            (
                Decimal::from_str("-2.0").unwrap(),
                // Wolfram give -0.9953222650189527
                Decimal::from_str("-0.9953225170750043399400930073").unwrap(),
            ),
            (
                Decimal::from_str("-0.4").unwrap(),
                Decimal::from_str("-0.4283924127205154977961931420").unwrap(),
            ),
            (
                Decimal::from_str("0.4").unwrap(),
                Decimal::from_str("0.4283924127205154977961931420").unwrap(),
            ),
            (
                Decimal::one(),
                Decimal::from_str("0.8427010463338918630217928957").unwrap(),
            ),
            (
                Decimal::from_str("2").unwrap(),
                Decimal::from_str("0.9953225170750043399400930073").unwrap(),
            ),
        ];
        for case in test_cases {
            assert_eq!(case.1, case.0.erf());
        }
    }

    #[test]
    fn test_checked_sin() {
        const ACCEPTED_PRECISION: u32 = 10;
        let test_cases = &[
            // Sin(0)
            ("0", Some("0")),
            // Sin(PI/2)
            ("1.5707963267948966192313216916", Some("1")),
            // Sin(PI)
            ("3.1415926535897932384626433833", Some("0")),
            // Sin(3PI/2)
            ("4.7123889803846898576939650749", Some("-1")),
            // Sin(2PI)
            ("6.2831853071795864769252867666", Some("0")),
            // Sin(1) ~= 0.8414709848078965066525023216302989996225630607983710656727517099
            ("1", Some("0.8414709848078965066525023216")),
            // Sin(2) ~= 0.9092974268256816953960198659117448427022549714478902683789730115
            ("2", Some("0.9092974268256816953960198659")),
            // Sin(4) ~= -0.756802495307928251372639094511829094135912887336472571485416773
            ("4", Some("-0.7568024953079282513726390945")),
            // Sin(6) ~= -0.279415498198925872811555446611894759627994864318204318483351369
            ("6", Some("-0.2794154981989258728115554466")),
            // WA estimate: -0.893653245236708. Legacy ops is closer to f64 accuracy.
            (
                "-79228162514264.337593543950335",
                Some(either!("-0.893653245236708", "-0.8963358176")),
            ),
            ("0.7853981633974483096156608458", Some("0.7071067811865475244008443621")),
        ];
        for (input, result) in test_cases {
            let radians = Decimal::from_str(input).unwrap();
            let sin = radians.checked_sin();
            if let Some(result) = result {
                assert!(sin.is_some(), "Expected result for sin({})", input);
                let result = Decimal::from_str(result).unwrap();
                assert_approx_eq!(sin.unwrap(), result, ACCEPTED_PRECISION, "sin({})", input);
            } else {
                assert!(sin.is_none(), "Unexpected result for sin({})", input);
            }
        }
    }

    #[test]
    fn test_checked_cos() {
        const ACCEPTED_PRECISION: u32 = 10;
        let test_cases = &[
            // Cos(0)
            ("0", Some("1")),
            // Cos(PI/2)
            ("1.5707963267948966192313216916", Some("0")),
            // Cos(PI)
            ("3.1415926535897932384626433833", Some("-1")),
            // Cos(3PI/2)
            ("4.7123889803846898576939650749", Some("0")),
            // Cos(2PI)
            ("6.2831853071795864769252867666", Some("1")),
            // Cos(1) ~= 0.5403023058681397174009366074429766037323104206179222276700972553
            ("1", Some("0.5403023058681397174009366074")),
            // Cos(2) ~= -0.416146836547142386997568229500762189766000771075544890755149973
            ("2", Some("-0.4161468365471423869975682295")),
            // Cos(4) ~= -0.653643620863611914639168183097750381424133596646218247007010283
            ("4", Some("-0.6536436208636119146391681831")),
            // Cos(6) ~= 0.9601702866503660205456522979229244054519376792110126981292864260
            ("6", Some("0.9601702866503660205456522979")),
            // WA estimate: 0.448758150096352. Legacy ops is closer to f64 accuracy.
            (
                "-79228162514264.337593543950335",
                Some(either!("0.448758150096352", "0.443375802326")),
            ),
            ("0.7853981633974483096156608458", Some("0.7071067810719247405681474639")),
            ("8.639379797371931405772269304", Some("-0.7071067811796194351866715184")),
        ];
        for (input, result) in test_cases {
            let radians = Decimal::from_str(input).unwrap();
            let cos = radians.checked_cos();
            if let Some(result) = result {
                assert!(cos.is_some(), "Expected result for cos({})", input);
                let result = Decimal::from_str(result).unwrap();
                assert_approx_eq!(cos.unwrap(), result, ACCEPTED_PRECISION, "cos({})", input);
            } else {
                assert!(cos.is_none(), "Unexpected result for cos({})", input);
            }
        }
    }

    #[test]
    fn test_checked_tan() {
        const ACCEPTED_PRECISION: u32 = 8;
        let test_cases = &[
            // Tan(0)
            ("0", Some("0")),
            // Tan(PI/2)
            ("1.5707963267948966192313216916", None),
            // Tan(PI)
            ("3.1415926535897932384626433833", Some("0")),
            // Tan(3PI/2)
            ("4.7123889803846898576939650749", None),
            // Tan(2PI)
            ("6.2831853071795864769252867666", Some("0")),
            // Tan(1) ~= 1.5574077246549022305069748074583601730872507723815200383839466056
            ("1", Some("1.5574077246549022305069748075")),
            // Tan(2) ~= -2.185039863261518991643306102313682543432017746227663164562955869
            ("2", Some("-2.1850398632615189916433061023")),
            // Tan(4) ~= 1.1578212823495775831373424182673239231197627673671421300848571893
            ("4", Some("1.1578212823495775831373424183")),
            // Tan(6) ~= -0.291006191384749157053699588868175542831155570912339131608827193
            ("6", Some("-0.2910061913847491570536995889")),
            // WA estimate: -1.99139167733184. Legacy ops is closer to f64 accuracy.
            (
                "-79228162514264.337593543950335",
                Some(either!("-1.99139167733184", "-2.021616454709")),
            ),
        ];
        for (input, result) in test_cases {
            let radians = Decimal::from_str(input).unwrap();
            let tan = radians.checked_tan();
            if let Some(result) = result {
                assert!(tan.is_some(), "Expected result for tan({})", input);
                let result = Decimal::from_str(result).unwrap();
                assert_approx_eq!(tan.unwrap(), result, ACCEPTED_PRECISION, "tan({})", input);
            } else {
                assert!(tan.is_none(), "Unexpected result for tan({})", input);
            }
        }
    }
}

// Generated tests
#[cfg(not(feature = "legacy-ops"))]
mod generated {
    use rust_decimal::prelude::*;

    macro_rules! gen_test {
        ($name:ident, $csv:expr, $method:tt) => {
            #[test]
            fn $name() {
                let path = std::env::current_dir().unwrap();
                let mut rdr = csv::Reader::from_reader(
                    std::fs::File::open(format!("{}/tests/generated/{}", path.display(), $csv)).unwrap(),
                );
                let mut row = 0;
                for result in rdr.records() {
                    let record = result.unwrap();
                    row += 1;

                    // Extract the data
                    let d1 = record.get(0).unwrap();
                    let d2 = record.get(1).unwrap();
                    let result = record.get(2).unwrap();
                    let error = record.get(3).unwrap();

                    // Do the calc
                    let d1 = Decimal::from_str(&d1).unwrap();
                    let d2 = Decimal::from_str(&d2).unwrap();
                    let expected = Decimal::from_str(&result).unwrap();
                    match d1.$method(d2) {
                        Some(v) => assert_eq!(expected, v, "Row {}", row),
                        None => assert!(!error.is_empty()),
                    }
                }
            }
        };
    }

    gen_test!(test_add_000_001, "Add_000_001.csv", checked_add);
    gen_test!(test_add_000_010, "Add_000_010.csv", checked_add);
    gen_test!(test_add_000_011, "Add_000_011.csv", checked_add);
    gen_test!(test_add_000_100, "Add_000_100.csv", checked_add);
    gen_test!(test_add_000_101, "Add_000_101.csv", checked_add);
    gen_test!(test_add_000_110, "Add_000_110.csv", checked_add);
    gen_test!(test_add_000_111, "Add_000_111.csv", checked_add);
    gen_test!(test_add_001_000, "Add_001_000.csv", checked_add);
    gen_test!(test_add_001_001, "Add_001_001.csv", checked_add);
    gen_test!(test_add_001_010, "Add_001_010.csv", checked_add);
    gen_test!(test_add_001_011, "Add_001_011.csv", checked_add);
    gen_test!(test_add_001_100, "Add_001_100.csv", checked_add);
    gen_test!(test_add_001_101, "Add_001_101.csv", checked_add);
    gen_test!(test_add_001_110, "Add_001_110.csv", checked_add);
    gen_test!(test_add_001_111, "Add_001_111.csv", checked_add);
    gen_test!(test_add_010_000, "Add_010_000.csv", checked_add);
    gen_test!(test_add_010_001, "Add_010_001.csv", checked_add);
    gen_test!(test_add_010_010, "Add_010_010.csv", checked_add);
    gen_test!(test_add_010_011, "Add_010_011.csv", checked_add);
    gen_test!(test_add_010_100, "Add_010_100.csv", checked_add);
    gen_test!(test_add_010_101, "Add_010_101.csv", checked_add);
    gen_test!(test_add_010_110, "Add_010_110.csv", checked_add);
    gen_test!(test_add_010_111, "Add_010_111.csv", checked_add);
    gen_test!(test_add_011_000, "Add_011_000.csv", checked_add);
    gen_test!(test_add_011_001, "Add_011_001.csv", checked_add);
    gen_test!(test_add_011_010, "Add_011_010.csv", checked_add);
    gen_test!(test_add_011_011, "Add_011_011.csv", checked_add);
    gen_test!(test_add_011_100, "Add_011_100.csv", checked_add);
    gen_test!(test_add_011_101, "Add_011_101.csv", checked_add);
    gen_test!(test_add_011_110, "Add_011_110.csv", checked_add);
    gen_test!(test_add_011_111, "Add_011_111.csv", checked_add);
    gen_test!(test_add_100_000, "Add_100_000.csv", checked_add);
    gen_test!(test_add_100_001, "Add_100_001.csv", checked_add);
    gen_test!(test_add_100_010, "Add_100_010.csv", checked_add);
    gen_test!(test_add_100_011, "Add_100_011.csv", checked_add);
    gen_test!(test_add_100_100, "Add_100_100.csv", checked_add);
    gen_test!(test_add_100_101, "Add_100_101.csv", checked_add);
    gen_test!(test_add_100_110, "Add_100_110.csv", checked_add);
    gen_test!(test_add_100_111, "Add_100_111.csv", checked_add);
    gen_test!(test_add_101_000, "Add_101_000.csv", checked_add);
    gen_test!(test_add_101_001, "Add_101_001.csv", checked_add);
    gen_test!(test_add_101_010, "Add_101_010.csv", checked_add);
    gen_test!(test_add_101_011, "Add_101_011.csv", checked_add);
    gen_test!(test_add_101_100, "Add_101_100.csv", checked_add);
    gen_test!(test_add_101_101, "Add_101_101.csv", checked_add);
    gen_test!(test_add_101_110, "Add_101_110.csv", checked_add);
    gen_test!(test_add_101_111, "Add_101_111.csv", checked_add);
    gen_test!(test_add_110_000, "Add_110_000.csv", checked_add);
    gen_test!(test_add_110_001, "Add_110_001.csv", checked_add);
    gen_test!(test_add_110_010, "Add_110_010.csv", checked_add);
    gen_test!(test_add_110_011, "Add_110_011.csv", checked_add);
    gen_test!(test_add_110_100, "Add_110_100.csv", checked_add);
    gen_test!(test_add_110_101, "Add_110_101.csv", checked_add);
    gen_test!(test_add_110_110, "Add_110_110.csv", checked_add);
    gen_test!(test_add_110_111, "Add_110_111.csv", checked_add);
    gen_test!(test_add_111_000, "Add_111_000.csv", checked_add);
    gen_test!(test_add_111_001, "Add_111_001.csv", checked_add);
    gen_test!(test_add_111_010, "Add_111_010.csv", checked_add);
    gen_test!(test_add_111_011, "Add_111_011.csv", checked_add);
    gen_test!(test_add_111_100, "Add_111_100.csv", checked_add);
    gen_test!(test_add_111_101, "Add_111_101.csv", checked_add);
    gen_test!(test_add_111_110, "Add_111_110.csv", checked_add);
    gen_test!(test_add_111_111, "Add_111_111.csv", checked_add);

    gen_test!(test_div_000_001, "Div_000_001.csv", checked_div);
    gen_test!(test_div_000_010, "Div_000_010.csv", checked_div);
    gen_test!(test_div_000_011, "Div_000_011.csv", checked_div);
    gen_test!(test_div_000_100, "Div_000_100.csv", checked_div);
    gen_test!(test_div_000_101, "Div_000_101.csv", checked_div);
    gen_test!(test_div_000_110, "Div_000_110.csv", checked_div);
    gen_test!(test_div_000_111, "Div_000_111.csv", checked_div);
    gen_test!(test_div_001_000, "Div_001_000.csv", checked_div);
    gen_test!(test_div_001_001, "Div_001_001.csv", checked_div);
    gen_test!(test_div_001_010, "Div_001_010.csv", checked_div);
    gen_test!(test_div_001_011, "Div_001_011.csv", checked_div);
    gen_test!(test_div_001_100, "Div_001_100.csv", checked_div);
    gen_test!(test_div_001_101, "Div_001_101.csv", checked_div);
    gen_test!(test_div_001_110, "Div_001_110.csv", checked_div);
    gen_test!(test_div_001_111, "Div_001_111.csv", checked_div);
    gen_test!(test_div_010_000, "Div_010_000.csv", checked_div);
    gen_test!(test_div_010_001, "Div_010_001.csv", checked_div);
    gen_test!(test_div_010_010, "Div_010_010.csv", checked_div);
    gen_test!(test_div_010_011, "Div_010_011.csv", checked_div);
    gen_test!(test_div_010_100, "Div_010_100.csv", checked_div);
    gen_test!(test_div_010_101, "Div_010_101.csv", checked_div);
    gen_test!(test_div_010_110, "Div_010_110.csv", checked_div);
    gen_test!(test_div_010_111, "Div_010_111.csv", checked_div);
    gen_test!(test_div_011_000, "Div_011_000.csv", checked_div);
    gen_test!(test_div_011_001, "Div_011_001.csv", checked_div);
    gen_test!(test_div_011_010, "Div_011_010.csv", checked_div);
    gen_test!(test_div_011_011, "Div_011_011.csv", checked_div);
    gen_test!(test_div_011_100, "Div_011_100.csv", checked_div);
    gen_test!(test_div_011_101, "Div_011_101.csv", checked_div);
    gen_test!(test_div_011_110, "Div_011_110.csv", checked_div);
    gen_test!(test_div_011_111, "Div_011_111.csv", checked_div);
    gen_test!(test_div_100_000, "Div_100_000.csv", checked_div);
    gen_test!(test_div_100_001, "Div_100_001.csv", checked_div);
    gen_test!(test_div_100_010, "Div_100_010.csv", checked_div);
    gen_test!(test_div_100_011, "Div_100_011.csv", checked_div);
    gen_test!(test_div_100_100, "Div_100_100.csv", checked_div);
    gen_test!(test_div_100_101, "Div_100_101.csv", checked_div);
    gen_test!(test_div_100_110, "Div_100_110.csv", checked_div);
    gen_test!(test_div_100_111, "Div_100_111.csv", checked_div);
    gen_test!(test_div_101_000, "Div_101_000.csv", checked_div);
    gen_test!(test_div_101_001, "Div_101_001.csv", checked_div);
    gen_test!(test_div_101_010, "Div_101_010.csv", checked_div);
    gen_test!(test_div_101_011, "Div_101_011.csv", checked_div);
    gen_test!(test_div_101_100, "Div_101_100.csv", checked_div);
    gen_test!(test_div_101_101, "Div_101_101.csv", checked_div);
    gen_test!(test_div_101_110, "Div_101_110.csv", checked_div);
    gen_test!(test_div_101_111, "Div_101_111.csv", checked_div);
    gen_test!(test_div_110_000, "Div_110_000.csv", checked_div);
    gen_test!(test_div_110_001, "Div_110_001.csv", checked_div);
    gen_test!(test_div_110_010, "Div_110_010.csv", checked_div);
    gen_test!(test_div_110_011, "Div_110_011.csv", checked_div);
    gen_test!(test_div_110_100, "Div_110_100.csv", checked_div);
    gen_test!(test_div_110_101, "Div_110_101.csv", checked_div);
    gen_test!(test_div_110_110, "Div_110_110.csv", checked_div);
    gen_test!(test_div_110_111, "Div_110_111.csv", checked_div);
    gen_test!(test_div_111_000, "Div_111_000.csv", checked_div);
    gen_test!(test_div_111_001, "Div_111_001.csv", checked_div);
    gen_test!(test_div_111_010, "Div_111_010.csv", checked_div);
    gen_test!(test_div_111_011, "Div_111_011.csv", checked_div);
    gen_test!(test_div_111_100, "Div_111_100.csv", checked_div);
    gen_test!(test_div_111_101, "Div_111_101.csv", checked_div);
    gen_test!(test_div_111_110, "Div_111_110.csv", checked_div);
    gen_test!(test_div_111_111, "Div_111_111.csv", checked_div);

    gen_test!(test_mul_000_001, "Mul_000_001.csv", checked_mul);
    gen_test!(test_mul_000_010, "Mul_000_010.csv", checked_mul);
    gen_test!(test_mul_000_011, "Mul_000_011.csv", checked_mul);
    gen_test!(test_mul_000_100, "Mul_000_100.csv", checked_mul);
    gen_test!(test_mul_000_101, "Mul_000_101.csv", checked_mul);
    gen_test!(test_mul_000_110, "Mul_000_110.csv", checked_mul);
    gen_test!(test_mul_000_111, "Mul_000_111.csv", checked_mul);
    gen_test!(test_mul_001_000, "Mul_001_000.csv", checked_mul);
    gen_test!(test_mul_001_001, "Mul_001_001.csv", checked_mul);
    gen_test!(test_mul_001_010, "Mul_001_010.csv", checked_mul);
    gen_test!(test_mul_001_011, "Mul_001_011.csv", checked_mul);
    gen_test!(test_mul_001_100, "Mul_001_100.csv", checked_mul);
    gen_test!(test_mul_001_101, "Mul_001_101.csv", checked_mul);
    gen_test!(test_mul_001_110, "Mul_001_110.csv", checked_mul);
    gen_test!(test_mul_001_111, "Mul_001_111.csv", checked_mul);
    gen_test!(test_mul_010_000, "Mul_010_000.csv", checked_mul);
    gen_test!(test_mul_010_001, "Mul_010_001.csv", checked_mul);
    gen_test!(test_mul_010_010, "Mul_010_010.csv", checked_mul);
    gen_test!(test_mul_010_011, "Mul_010_011.csv", checked_mul);
    gen_test!(test_mul_010_100, "Mul_010_100.csv", checked_mul);
    gen_test!(test_mul_010_101, "Mul_010_101.csv", checked_mul);
    gen_test!(test_mul_010_110, "Mul_010_110.csv", checked_mul);
    gen_test!(test_mul_010_111, "Mul_010_111.csv", checked_mul);
    gen_test!(test_mul_011_000, "Mul_011_000.csv", checked_mul);
    gen_test!(test_mul_011_001, "Mul_011_001.csv", checked_mul);
    gen_test!(test_mul_011_010, "Mul_011_010.csv", checked_mul);
    gen_test!(test_mul_011_011, "Mul_011_011.csv", checked_mul);
    gen_test!(test_mul_011_100, "Mul_011_100.csv", checked_mul);
    gen_test!(test_mul_011_101, "Mul_011_101.csv", checked_mul);
    gen_test!(test_mul_011_110, "Mul_011_110.csv", checked_mul);
    gen_test!(test_mul_011_111, "Mul_011_111.csv", checked_mul);
    gen_test!(test_mul_100_000, "Mul_100_000.csv", checked_mul);
    gen_test!(test_mul_100_001, "Mul_100_001.csv", checked_mul);
    gen_test!(test_mul_100_010, "Mul_100_010.csv", checked_mul);
    gen_test!(test_mul_100_011, "Mul_100_011.csv", checked_mul);
    gen_test!(test_mul_100_100, "Mul_100_100.csv", checked_mul);
    gen_test!(test_mul_100_101, "Mul_100_101.csv", checked_mul);
    gen_test!(test_mul_100_110, "Mul_100_110.csv", checked_mul);
    gen_test!(test_mul_100_111, "Mul_100_111.csv", checked_mul);
    gen_test!(test_mul_101_000, "Mul_101_000.csv", checked_mul);
    gen_test!(test_mul_101_001, "Mul_101_001.csv", checked_mul);
    gen_test!(test_mul_101_010, "Mul_101_010.csv", checked_mul);
    gen_test!(test_mul_101_011, "Mul_101_011.csv", checked_mul);
    gen_test!(test_mul_101_100, "Mul_101_100.csv", checked_mul);
    gen_test!(test_mul_101_101, "Mul_101_101.csv", checked_mul);
    gen_test!(test_mul_101_110, "Mul_101_110.csv", checked_mul);
    gen_test!(test_mul_101_111, "Mul_101_111.csv", checked_mul);
    gen_test!(test_mul_110_000, "Mul_110_000.csv", checked_mul);
    gen_test!(test_mul_110_001, "Mul_110_001.csv", checked_mul);
    gen_test!(test_mul_110_010, "Mul_110_010.csv", checked_mul);
    gen_test!(test_mul_110_011, "Mul_110_011.csv", checked_mul);
    gen_test!(test_mul_110_100, "Mul_110_100.csv", checked_mul);
    gen_test!(test_mul_110_101, "Mul_110_101.csv", checked_mul);
    gen_test!(test_mul_110_110, "Mul_110_110.csv", checked_mul);
    gen_test!(test_mul_110_111, "Mul_110_111.csv", checked_mul);
    gen_test!(test_mul_111_000, "Mul_111_000.csv", checked_mul);
    gen_test!(test_mul_111_001, "Mul_111_001.csv", checked_mul);
    gen_test!(test_mul_111_010, "Mul_111_010.csv", checked_mul);
    gen_test!(test_mul_111_011, "Mul_111_011.csv", checked_mul);
    gen_test!(test_mul_111_100, "Mul_111_100.csv", checked_mul);
    gen_test!(test_mul_111_101, "Mul_111_101.csv", checked_mul);
    gen_test!(test_mul_111_110, "Mul_111_110.csv", checked_mul);
    gen_test!(test_mul_111_111, "Mul_111_111.csv", checked_mul);

    gen_test!(test_rem_000_001, "Rem_000_001.csv", checked_rem);
    gen_test!(test_rem_000_010, "Rem_000_010.csv", checked_rem);
    gen_test!(test_rem_000_011, "Rem_000_011.csv", checked_rem);
    gen_test!(test_rem_000_100, "Rem_000_100.csv", checked_rem);
    gen_test!(test_rem_000_101, "Rem_000_101.csv", checked_rem);
    gen_test!(test_rem_000_110, "Rem_000_110.csv", checked_rem);
    gen_test!(test_rem_000_111, "Rem_000_111.csv", checked_rem);
    gen_test!(test_rem_001_000, "Rem_001_000.csv", checked_rem);
    gen_test!(test_rem_001_001, "Rem_001_001.csv", checked_rem);
    gen_test!(test_rem_001_010, "Rem_001_010.csv", checked_rem);
    gen_test!(test_rem_001_011, "Rem_001_011.csv", checked_rem);
    gen_test!(test_rem_001_100, "Rem_001_100.csv", checked_rem);
    gen_test!(test_rem_001_101, "Rem_001_101.csv", checked_rem);
    gen_test!(test_rem_001_110, "Rem_001_110.csv", checked_rem);
    gen_test!(test_rem_001_111, "Rem_001_111.csv", checked_rem);
    gen_test!(test_rem_010_000, "Rem_010_000.csv", checked_rem);
    gen_test!(test_rem_010_001, "Rem_010_001.csv", checked_rem);
    gen_test!(test_rem_010_010, "Rem_010_010.csv", checked_rem);
    gen_test!(test_rem_010_011, "Rem_010_011.csv", checked_rem);
    gen_test!(test_rem_010_100, "Rem_010_100.csv", checked_rem);
    gen_test!(test_rem_010_101, "Rem_010_101.csv", checked_rem);
    gen_test!(test_rem_010_110, "Rem_010_110.csv", checked_rem);
    gen_test!(test_rem_010_111, "Rem_010_111.csv", checked_rem);
    gen_test!(test_rem_011_000, "Rem_011_000.csv", checked_rem);
    gen_test!(test_rem_011_001, "Rem_011_001.csv", checked_rem);
    gen_test!(test_rem_011_010, "Rem_011_010.csv", checked_rem);
    gen_test!(test_rem_011_011, "Rem_011_011.csv", checked_rem);
    gen_test!(test_rem_011_100, "Rem_011_100.csv", checked_rem);
    gen_test!(test_rem_011_101, "Rem_011_101.csv", checked_rem);
    gen_test!(test_rem_011_110, "Rem_011_110.csv", checked_rem);
    gen_test!(test_rem_011_111, "Rem_011_111.csv", checked_rem);
    gen_test!(test_rem_100_000, "Rem_100_000.csv", checked_rem);
    gen_test!(test_rem_100_001, "Rem_100_001.csv", checked_rem);
    gen_test!(test_rem_100_010, "Rem_100_010.csv", checked_rem);
    gen_test!(test_rem_100_011, "Rem_100_011.csv", checked_rem);
    gen_test!(test_rem_100_100, "Rem_100_100.csv", checked_rem);
    gen_test!(test_rem_100_101, "Rem_100_101.csv", checked_rem);
    gen_test!(test_rem_100_110, "Rem_100_110.csv", checked_rem);
    gen_test!(test_rem_100_111, "Rem_100_111.csv", checked_rem);
    gen_test!(test_rem_101_000, "Rem_101_000.csv", checked_rem);
    gen_test!(test_rem_101_001, "Rem_101_001.csv", checked_rem);
    gen_test!(test_rem_101_010, "Rem_101_010.csv", checked_rem);
    gen_test!(test_rem_101_011, "Rem_101_011.csv", checked_rem);
    gen_test!(test_rem_101_100, "Rem_101_100.csv", checked_rem);
    gen_test!(test_rem_101_101, "Rem_101_101.csv", checked_rem);
    gen_test!(test_rem_101_110, "Rem_101_110.csv", checked_rem);
    gen_test!(test_rem_101_111, "Rem_101_111.csv", checked_rem);
    gen_test!(test_rem_110_000, "Rem_110_000.csv", checked_rem);
    gen_test!(test_rem_110_001, "Rem_110_001.csv", checked_rem);
    gen_test!(test_rem_110_010, "Rem_110_010.csv", checked_rem);
    gen_test!(test_rem_110_011, "Rem_110_011.csv", checked_rem);
    gen_test!(test_rem_110_100, "Rem_110_100.csv", checked_rem);
    gen_test!(test_rem_110_101, "Rem_110_101.csv", checked_rem);
    gen_test!(test_rem_110_110, "Rem_110_110.csv", checked_rem);
    gen_test!(test_rem_110_111, "Rem_110_111.csv", checked_rem);
    gen_test!(test_rem_111_000, "Rem_111_000.csv", checked_rem);
    gen_test!(test_rem_111_001, "Rem_111_001.csv", checked_rem);
    gen_test!(test_rem_111_010, "Rem_111_010.csv", checked_rem);
    gen_test!(test_rem_111_011, "Rem_111_011.csv", checked_rem);
    gen_test!(test_rem_111_100, "Rem_111_100.csv", checked_rem);
    gen_test!(test_rem_111_101, "Rem_111_101.csv", checked_rem);
    gen_test!(test_rem_111_110, "Rem_111_110.csv", checked_rem);
    gen_test!(test_rem_111_111, "Rem_111_111.csv", checked_rem);

    gen_test!(test_sub_000_001, "Sub_000_001.csv", checked_sub);
    gen_test!(test_sub_000_010, "Sub_000_010.csv", checked_sub);
    gen_test!(test_sub_000_011, "Sub_000_011.csv", checked_sub);
    gen_test!(test_sub_000_100, "Sub_000_100.csv", checked_sub);
    gen_test!(test_sub_000_101, "Sub_000_101.csv", checked_sub);
    gen_test!(test_sub_000_110, "Sub_000_110.csv", checked_sub);
    gen_test!(test_sub_000_111, "Sub_000_111.csv", checked_sub);
    gen_test!(test_sub_001_000, "Sub_001_000.csv", checked_sub);
    gen_test!(test_sub_001_001, "Sub_001_001.csv", checked_sub);
    gen_test!(test_sub_001_010, "Sub_001_010.csv", checked_sub);
    gen_test!(test_sub_001_011, "Sub_001_011.csv", checked_sub);
    gen_test!(test_sub_001_100, "Sub_001_100.csv", checked_sub);
    gen_test!(test_sub_001_101, "Sub_001_101.csv", checked_sub);
    gen_test!(test_sub_001_110, "Sub_001_110.csv", checked_sub);
    gen_test!(test_sub_001_111, "Sub_001_111.csv", checked_sub);
    gen_test!(test_sub_010_000, "Sub_010_000.csv", checked_sub);
    gen_test!(test_sub_010_001, "Sub_010_001.csv", checked_sub);
    gen_test!(test_sub_010_010, "Sub_010_010.csv", checked_sub);
    gen_test!(test_sub_010_011, "Sub_010_011.csv", checked_sub);
    gen_test!(test_sub_010_100, "Sub_010_100.csv", checked_sub);
    gen_test!(test_sub_010_101, "Sub_010_101.csv", checked_sub);
    gen_test!(test_sub_010_110, "Sub_010_110.csv", checked_sub);
    gen_test!(test_sub_010_111, "Sub_010_111.csv", checked_sub);
    gen_test!(test_sub_011_000, "Sub_011_000.csv", checked_sub);
    gen_test!(test_sub_011_001, "Sub_011_001.csv", checked_sub);
    gen_test!(test_sub_011_010, "Sub_011_010.csv", checked_sub);
    gen_test!(test_sub_011_011, "Sub_011_011.csv", checked_sub);
    gen_test!(test_sub_011_100, "Sub_011_100.csv", checked_sub);
    gen_test!(test_sub_011_101, "Sub_011_101.csv", checked_sub);
    gen_test!(test_sub_011_110, "Sub_011_110.csv", checked_sub);
    gen_test!(test_sub_011_111, "Sub_011_111.csv", checked_sub);
    gen_test!(test_sub_100_000, "Sub_100_000.csv", checked_sub);
    gen_test!(test_sub_100_001, "Sub_100_001.csv", checked_sub);
    gen_test!(test_sub_100_010, "Sub_100_010.csv", checked_sub);
    gen_test!(test_sub_100_011, "Sub_100_011.csv", checked_sub);
    gen_test!(test_sub_100_100, "Sub_100_100.csv", checked_sub);
    gen_test!(test_sub_100_101, "Sub_100_101.csv", checked_sub);
    gen_test!(test_sub_100_110, "Sub_100_110.csv", checked_sub);
    gen_test!(test_sub_100_111, "Sub_100_111.csv", checked_sub);
    gen_test!(test_sub_101_000, "Sub_101_000.csv", checked_sub);
    gen_test!(test_sub_101_001, "Sub_101_001.csv", checked_sub);
    gen_test!(test_sub_101_010, "Sub_101_010.csv", checked_sub);
    gen_test!(test_sub_101_011, "Sub_101_011.csv", checked_sub);
    gen_test!(test_sub_101_100, "Sub_101_100.csv", checked_sub);
    gen_test!(test_sub_101_101, "Sub_101_101.csv", checked_sub);
    gen_test!(test_sub_101_110, "Sub_101_110.csv", checked_sub);
    gen_test!(test_sub_101_111, "Sub_101_111.csv", checked_sub);
    gen_test!(test_sub_110_000, "Sub_110_000.csv", checked_sub);
    gen_test!(test_sub_110_001, "Sub_110_001.csv", checked_sub);
    gen_test!(test_sub_110_010, "Sub_110_010.csv", checked_sub);
    gen_test!(test_sub_110_011, "Sub_110_011.csv", checked_sub);
    gen_test!(test_sub_110_100, "Sub_110_100.csv", checked_sub);
    gen_test!(test_sub_110_101, "Sub_110_101.csv", checked_sub);
    gen_test!(test_sub_110_110, "Sub_110_110.csv", checked_sub);
    gen_test!(test_sub_110_111, "Sub_110_111.csv", checked_sub);
    gen_test!(test_sub_111_000, "Sub_111_000.csv", checked_sub);
    gen_test!(test_sub_111_001, "Sub_111_001.csv", checked_sub);
    gen_test!(test_sub_111_010, "Sub_111_010.csv", checked_sub);
    gen_test!(test_sub_111_011, "Sub_111_011.csv", checked_sub);
    gen_test!(test_sub_111_100, "Sub_111_100.csv", checked_sub);
    gen_test!(test_sub_111_101, "Sub_111_101.csv", checked_sub);
    gen_test!(test_sub_111_110, "Sub_111_110.csv", checked_sub);
    gen_test!(test_sub_111_111, "Sub_111_111.csv", checked_sub);
}

#[cfg(feature = "proptest")]
mod proptest_tests {
    use super::Decimal;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_proptest_validate_arbitrary_decimals(num in any::<Decimal>()) {
            assert!(num.is_zero() || !num.is_zero());
        }
    }
}

#[cfg(feature = "rocket-traits")]
#[allow(clippy::disallowed_names)]
mod rocket_tests {
    use crate::Decimal;
    use rocket::form::{Form, FromForm};
    use std::str::FromStr;

    #[derive(FromForm)]
    struct Example {
        foo: Decimal,
        bar: Decimal,
    }

    #[test]
    fn it_can_parse_form() {
        let parsed: Example = Form::parse("bar=0.12345678901234567890123456789&foo=-123456.78").unwrap();
        assert_eq!(parsed.foo, Decimal::from_str("-123456.78").unwrap());
        assert_eq!(
            parsed.bar,
            Decimal::from_str("0.12345678901234567890123456789").unwrap()
        );
    }
}

#[cfg(feature = "rust-fuzz")]
mod rust_fuzz_tests {
    use arbitrary::{Arbitrary, Unstructured};

    use super::*;

    #[test]
    fn it_can_generate_arbitrary_decimals() {
        let mut u = Unstructured::new(b"it_can_generate_arbitrary_decimals");
        let d = Decimal::arbitrary(&mut u);
        assert!(d.is_ok());
    }
}

mod issues {
    use rust_decimal::prelude::*;

    #[test]
    fn issue_384_neg_overflow_during_subtract_carry() {
        // 288230376151711744
        let a = Decimal::from_parts(0, 67108864, 0, false, 0);
        // 714606955844629274884780.85120
        let b = Decimal::from_parts(0, 0, 3873892070, false, 5);
        let c = a.checked_sub(b);
        assert!(c.is_some());

        //         288230376151711744.
        // - 714606955844629274884780.85120
        // =
        // - 714606667614253123173036.85120
        assert_eq!("-714606667614253123173036.85120", c.unwrap().to_string());
    }

    #[test]
    fn issue_392_overflow_during_remainder() {
        let a = Decimal::from_str("-79228157791897.854723898738431").unwrap();
        let b = Decimal::from_str("184512476.73336922111").unwrap();
        let c = a.checked_div(b);
        assert!(c.is_some());
        assert_eq!("-429391.87200000000002327170816", c.unwrap().to_string())
    }

    #[test]
    fn issue_624_to_f64_precision() {
        let tests = [
            ("1000000.000000000000000000", 1000000.0f64),
            ("10000.000000000000000000", 10000.0f64),
            ("100000.000000000000000000", 100000.0f64), // Problematic value
        ];
        for (index, (test, expected)) in tests.iter().enumerate() {
            let decimal = Decimal::from_str_exact(test).unwrap();
            assert_eq!(
                f64::try_from(decimal).unwrap(),
                *expected,
                "Test index {} failed",
                index
            );
        }
    }

    #[test]
    #[cfg(not(feature = "legacy-ops"))] // I will deprecate this feature/behavior in an upcoming release
    fn issue_618_rescaling_overflow() {
        fn assert_result(scale: u32, v1: Decimal, v2: Decimal) {
            assert_eq!(scale, v1.scale(), "initial scale: {scale}");
            let result1 = v1 + -v2;
            assert_eq!(
                result1.to_string(),
                "-0.0999999999999999999999999999",
                "a + -b : {scale}"
            );
            assert_eq!(28, result1.scale(), "a + -b : {scale}");
            let result2 = v1 - v2;
            assert_eq!(
                result2.to_string(),
                "-0.0999999999999999999999999999",
                "a - b : {scale}"
            );
            assert_eq!(28, result2.scale(), "a - b : {scale}");
        }

        let mut a = Decimal::from_str("0.0000000000000000000000000001").unwrap();
        let b = Decimal::from_str("0.1").unwrap();
        assert_result(28, a, b);

        // Try at a new scale (this works)
        a.rescale(30);
        assert_result(30, a, b);

        // And finally the scale causing an issue
        a.rescale(29);
        assert_result(29, a, b);
    }
}
