#![cfg(feature = "with-num-bigint")]

mod common;

use num_bigint::{BigInt, BigUint, Sign};
use num_format::{CustomFormat, ToFormattedString, WriteFormatted};

use crate::common::POLICIES;

#[test]
fn test_num_big_int() {
    let test_cases: &[(&str, BigInt, &CustomFormat)] = &[
        ("\u{200e}-\u{200e}1𠜱000", BigInt::new(Sign::Minus, vec![1000]), &POLICIES[2]),
        ("\u{200e}-\u{200e}1𠜱00𠜱000", BigInt::new(Sign::Minus, vec![100000]), &POLICIES[2]),

        ("1", BigInt::new(Sign::Plus, vec![1]), &POLICIES[0]),
        ("1", BigInt::new(Sign::Plus, vec![1]), &POLICIES[1]),
        ("1", BigInt::new(Sign::Plus, vec![1]), &POLICIES[2]),
        ("1", BigInt::new(Sign::Plus, vec![1]), &POLICIES[3]),
        ("1", BigInt::new(Sign::Plus, vec![1]), &POLICIES[4]),
        (
            "340,282,366,920,938,463,463,374,607,431,768,211,455",
            BigInt::new(
                Sign::Plus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[0],
        ),
        (
            "340𠜱282𠜱366𠜱920𠜱938𠜱463𠜱463𠜱374𠜱607𠜱431𠜱768𠜱211𠜱455",
            BigInt::new(
                Sign::Plus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[1],
        ),
        (
            "34𠜱02𠜱82𠜱36𠜱69𠜱20𠜱93𠜱84𠜱63𠜱46𠜱33𠜱74𠜱60𠜱74𠜱31𠜱76𠜱82𠜱11𠜱455",
            BigInt::new(
                Sign::Plus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[2],
        ),
        (
            "340282366920938463463374607431768211455",
            BigInt::new(
                Sign::Plus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[3],
        ),
        (
            "340282366920938463463374607431768211455",
            BigInt::new(
                Sign::Plus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[4],
        ),
        (
            "-340,282,366,920,938,463,463,374,607,431,768,211,455",
            BigInt::new(
                Sign::Minus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[0],
        ),
        (
            "\u{200e}-\u{200e}340𠜱282𠜱366𠜱920𠜱938𠜱463𠜱463𠜱374𠜱607𠜱431𠜱768𠜱211𠜱455",
            BigInt::new(
                Sign::Minus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[1],
        ),
        (
            "\u{200e}-\u{200e}34𠜱02𠜱82𠜱36𠜱69𠜱20𠜱93𠜱84𠜱63𠜱46𠜱33𠜱74𠜱60𠜱74𠜱31𠜱76𠜱82𠜱11𠜱455",
            BigInt::new(
                Sign::Minus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[2],
        ),
        (
            "\u{200e}-\u{200e}340282366920938463463374607431768211455",
            BigInt::new(
                Sign::Minus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[3],
        ),
        (
            "\u{200e}-\u{200e}340282366920938463463374607431768211455",
            BigInt::new(
                Sign::Minus,
                vec![std::u32::MAX, std::u32::MAX, std::u32::MAX, std::u32::MAX],
            ),
            &POLICIES[4],
        ),
    ];

    for (expected, input, format) in test_cases {
        // ToFormattedString
        assert_eq!(expected.to_string(), input.to_formatted_string(*format));

        // WriteFormatted (io::Write)
        let mut v = Vec::new();
        v.write_formatted(input, *format).unwrap();
        let s = String::from_utf8(v).unwrap();
        assert_eq!(expected.to_string(), s);

        // WriteFormatted (fmt::Write)
        let mut s = String::new();
        s.write_formatted(input, *format).unwrap();
        assert_eq!(expected.to_string(), s);
    }
}

#[test]
fn test_num_big_uint() {
    let test_cases: &[(&str, BigUint, &CustomFormat)] = &[
        ("1", BigUint::new(vec![1]), &POLICIES[0]),
        ("1", BigUint::new(vec![1]), &POLICIES[1]),
        ("1", BigUint::new(vec![1]), &POLICIES[2]),
        ("1", BigUint::new(vec![1]), &POLICIES[3]),
        ("1", BigUint::new(vec![1]), &POLICIES[4]),
        (
            "340,282,366,920,938,463,463,374,607,431,768,211,455",
            BigUint::new(vec![
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
            ]),
            &POLICIES[0],
        ),
        (
            "340𠜱282𠜱366𠜱920𠜱938𠜱463𠜱463𠜱374𠜱607𠜱431𠜱768𠜱211𠜱455",
            BigUint::new(vec![
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
            ]),
            &POLICIES[1],
        ),
        (
            "34𠜱02𠜱82𠜱36𠜱69𠜱20𠜱93𠜱84𠜱63𠜱46𠜱33𠜱74𠜱60𠜱74𠜱31𠜱76𠜱82𠜱11𠜱455",
            BigUint::new(vec![
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
            ]),
            &POLICIES[2],
        ),
        (
            "340282366920938463463374607431768211455",
            BigUint::new(vec![
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
            ]),
            &POLICIES[3],
        ),
        (
            "340282366920938463463374607431768211455",
            BigUint::new(vec![
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
                std::u32::MAX,
            ]),
            &POLICIES[4],
        ),
    ];

    for (expected, input, format) in test_cases {
        // ToFormattedString
        assert_eq!(expected.to_string(), input.to_formatted_string(*format));

        // WriteFormatted (io::Write)
        let mut v = Vec::new();
        v.write_formatted(input, *format).unwrap();
        let s = String::from_utf8(v).unwrap();
        assert_eq!(expected.to_string(), s);

        // WriteFormatted (fmt::Write)
        let mut s = String::new();
        s.write_formatted(input, *format).unwrap();
        assert_eq!(expected.to_string(), s);
    }
}
