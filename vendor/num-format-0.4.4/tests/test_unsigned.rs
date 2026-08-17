mod common;

use num_format::{Buffer, CustomFormat};
#[cfg(feature = "std")]
use num_format::{ToFormattedString, WriteFormatted};

use crate::common::POLICIES;

#[test]
fn test_u8() {
    let test_cases: &[(&str, u8, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("255", std::u8::MAX, &POLICIES[0]),
        ("255", std::u8::MAX, &POLICIES[1]),
        ("255", std::u8::MAX, &POLICIES[2]),
        ("255", std::u8::MAX, &POLICIES[3]),
        ("255", std::u8::MAX, &POLICIES[4]),
    ];

    for (expected, input, format) in test_cases {
        // Buffer
        let mut buf = Buffer::default();
        buf.write_formatted(input, *format);
        assert_eq!(*expected, buf.as_str());

        #[cfg(feature = "std")]
        {
            // ToFormattedString
            assert_eq!(expected.to_string(), input.to_formatted_string(*format));

            // WriteFormatted
            let mut s = String::new();
            s.write_formatted(input, *format).unwrap();
            assert_eq!(expected.to_string(), s);
        }
    }
}

#[test]
fn test_u16() {
    let test_cases: &[(&str, u16, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("65,535", std::u16::MAX, &POLICIES[0]),
        ("65𠜱535", std::u16::MAX, &POLICIES[1]),
        ("65𠜱535", std::u16::MAX, &POLICIES[2]),
        ("65535", std::u16::MAX, &POLICIES[3]),
        ("65535", std::u16::MAX, &POLICIES[4]),
    ];

    for (expected, input, format) in test_cases {
        // Buffer
        let mut buf = Buffer::default();
        buf.write_formatted(input, *format);
        assert_eq!(*expected, buf.as_str());

        #[cfg(feature = "std")]
        {
            // ToFormattedString
            assert_eq!(expected.to_string(), input.to_formatted_string(*format));

            // WriteFormatted
            let mut s = String::new();
            s.write_formatted(input, *format).unwrap();
            assert_eq!(expected.to_string(), s);
        }
    }
}

#[test]
fn test_u32() {
    let test_cases: &[(&str, u32, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("4,294,967,295", std::u32::MAX, &POLICIES[0]),
        ("4𠜱294𠜱967𠜱295", std::u32::MAX, &POLICIES[1]),
        ("4𠜱29𠜱49𠜱67𠜱295", std::u32::MAX, &POLICIES[2]),
        ("4294967295", std::u32::MAX, &POLICIES[3]),
        ("4294967295", std::u32::MAX, &POLICIES[4]),
    ];

    for (expected, input, format) in test_cases {
        // Buffer
        let mut buf = Buffer::default();
        buf.write_formatted(input, *format);
        assert_eq!(*expected, buf.as_str());

        #[cfg(feature = "std")]
        {
            // ToFormattedString
            assert_eq!(expected.to_string(), input.to_formatted_string(*format));

            // WriteFormatted
            let mut s = String::new();
            s.write_formatted(input, *format).unwrap();
            assert_eq!(expected.to_string(), s);
        }
    }
}

#[test]
fn test_usize() {
    let test_cases: &[(&str, usize, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("18,446,744,073,709,551,615", std::usize::MAX, &POLICIES[0]),
        (
            "18𠜱446𠜱744𠜱073𠜱709𠜱551𠜱615",
            std::usize::MAX,
            &POLICIES[1],
        ),
        (
            "1𠜱84𠜱46𠜱74𠜱40𠜱73𠜱70𠜱95𠜱51𠜱615",
            std::usize::MAX,
            &POLICIES[2],
        ),
        ("18446744073709551615", std::usize::MAX, &POLICIES[3]),
        ("18446744073709551615", std::usize::MAX, &POLICIES[4]),
    ];

    for (expected, input, format) in test_cases {
        // Buffer
        let mut buf = Buffer::default();
        buf.write_formatted(input, *format);
        assert_eq!(*expected, buf.as_str());

        #[cfg(feature = "std")]
        {
            // ToFormattedString
            assert_eq!(expected.to_string(), input.to_formatted_string(*format));

            // WriteFormatted
            let mut s = String::new();
            s.write_formatted(input, *format).unwrap();
            assert_eq!(expected.to_string(), s);
        }
    }
}

#[test]
fn test_u64() {
    let test_cases: &[(&str, u64, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("18,446,744,073,709,551,615", std::u64::MAX, &POLICIES[0]),
        (
            "18𠜱446𠜱744𠜱073𠜱709𠜱551𠜱615",
            std::u64::MAX,
            &POLICIES[1],
        ),
        (
            "1𠜱84𠜱46𠜱74𠜱40𠜱73𠜱70𠜱95𠜱51𠜱615",
            std::u64::MAX,
            &POLICIES[2],
        ),
        ("18446744073709551615", std::u64::MAX, &POLICIES[3]),
        ("18446744073709551615", std::u64::MAX, &POLICIES[4]),
    ];

    for (expected, input, format) in test_cases {
        // Buffer
        let mut buf = Buffer::default();
        buf.write_formatted(input, *format);
        assert_eq!(*expected, buf.as_str());

        #[cfg(feature = "std")]
        {
            // ToFormattedString
            assert_eq!(expected.to_string(), input.to_formatted_string(*format));

            // WriteFormatted
            let mut s = String::new();
            s.write_formatted(input, *format).unwrap();
            assert_eq!(expected.to_string(), s);
        }
    }
}

#[test]
fn test_u128() {
    let test_cases: &[(&str, u128, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        (
            "340,282,366,920,938,463,463,374,607,431,768,211,455",
            std::u128::MAX,
            &POLICIES[0],
        ),
        (
            "340𠜱282𠜱366𠜱920𠜱938𠜱463𠜱463𠜱374𠜱607𠜱431𠜱768𠜱211𠜱455",
            std::u128::MAX,
            &POLICIES[1],
        ),
        (
            "34𠜱02𠜱82𠜱36𠜱69𠜱20𠜱93𠜱84𠜱63𠜱46𠜱33𠜱74𠜱60𠜱74𠜱31𠜱76𠜱82𠜱11𠜱455",
            std::u128::MAX,
            &POLICIES[2],
        ),
        (
            "340282366920938463463374607431768211455",
            std::u128::MAX,
            &POLICIES[3],
        ),
        (
            "340282366920938463463374607431768211455",
            std::u128::MAX,
            &POLICIES[4],
        ),
    ];

    for (expected, input, format) in test_cases {
        // Buffer
        let mut buf = Buffer::default();
        buf.write_formatted(input, *format);
        assert_eq!(*expected, buf.as_str());

        #[cfg(feature = "std")]
        {
            // ToFormattedString
            assert_eq!(expected.to_string(), input.to_formatted_string(*format));

            // WriteFormatted
            let mut s = String::new();
            s.write_formatted(input, *format).unwrap();
            assert_eq!(expected.to_string(), s);
        }
    }
}
