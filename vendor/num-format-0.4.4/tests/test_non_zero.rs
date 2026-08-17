mod common;

use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

use num_format::{Buffer, CustomFormat};
#[cfg(feature = "std")]
use num_format::{ToFormattedString, WriteFormatted};

use crate::common::POLICIES;

#[test]
fn test_non_zero_u8() {
    let test_cases: &[(&str, NonZeroU8, &CustomFormat)] = &[
        ("1", NonZeroU8::new(1).unwrap(), &POLICIES[0]),
        ("1", NonZeroU8::new(1).unwrap(), &POLICIES[1]),
        ("1", NonZeroU8::new(1).unwrap(), &POLICIES[2]),
        ("1", NonZeroU8::new(1).unwrap(), &POLICIES[3]),
        ("1", NonZeroU8::new(1).unwrap(), &POLICIES[4]),
        ("255", NonZeroU8::new(std::u8::MAX).unwrap(), &POLICIES[0]),
        ("255", NonZeroU8::new(std::u8::MAX).unwrap(), &POLICIES[1]),
        ("255", NonZeroU8::new(std::u8::MAX).unwrap(), &POLICIES[2]),
        ("255", NonZeroU8::new(std::u8::MAX).unwrap(), &POLICIES[3]),
        ("255", NonZeroU8::new(std::u8::MAX).unwrap(), &POLICIES[4]),
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
fn test_non_zero_u16() {
    let test_cases: &[(&str, NonZeroU16, &CustomFormat)] = &[
        ("1", NonZeroU16::new(1).unwrap(), &POLICIES[0]),
        ("1", NonZeroU16::new(1).unwrap(), &POLICIES[1]),
        ("1", NonZeroU16::new(1).unwrap(), &POLICIES[2]),
        ("1", NonZeroU16::new(1).unwrap(), &POLICIES[3]),
        ("1", NonZeroU16::new(1).unwrap(), &POLICIES[4]),
        (
            "65,535",
            NonZeroU16::new(std::u16::MAX).unwrap(),
            &POLICIES[0],
        ),
        (
            "65𠜱535",
            NonZeroU16::new(std::u16::MAX).unwrap(),
            &POLICIES[1],
        ),
        (
            "65𠜱535",
            NonZeroU16::new(std::u16::MAX).unwrap(),
            &POLICIES[2],
        ),
        (
            "65535",
            NonZeroU16::new(std::u16::MAX).unwrap(),
            &POLICIES[3],
        ),
        (
            "65535",
            NonZeroU16::new(std::u16::MAX).unwrap(),
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

#[test]
fn test_non_zero_u32() {
    let test_cases: &[(&str, NonZeroU32, &CustomFormat)] = &[
        ("1", NonZeroU32::new(1).unwrap(), &POLICIES[0]),
        ("1", NonZeroU32::new(1).unwrap(), &POLICIES[1]),
        ("1", NonZeroU32::new(1).unwrap(), &POLICIES[2]),
        ("1", NonZeroU32::new(1).unwrap(), &POLICIES[3]),
        ("1", NonZeroU32::new(1).unwrap(), &POLICIES[4]),
        (
            "4,294,967,295",
            NonZeroU32::new(std::u32::MAX).unwrap(),
            &POLICIES[0],
        ),
        (
            "4𠜱294𠜱967𠜱295",
            NonZeroU32::new(std::u32::MAX).unwrap(),
            &POLICIES[1],
        ),
        (
            "4𠜱29𠜱49𠜱67𠜱295",
            NonZeroU32::new(std::u32::MAX).unwrap(),
            &POLICIES[2],
        ),
        (
            "4294967295",
            NonZeroU32::new(std::u32::MAX).unwrap(),
            &POLICIES[3],
        ),
        (
            "4294967295",
            NonZeroU32::new(std::u32::MAX).unwrap(),
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

#[test]
fn test_non_zero_usize() {
    let test_cases: &[(&str, NonZeroUsize, &CustomFormat)] = &[
        ("1", NonZeroUsize::new(1).unwrap(), &POLICIES[0]),
        ("1", NonZeroUsize::new(1).unwrap(), &POLICIES[1]),
        ("1", NonZeroUsize::new(1).unwrap(), &POLICIES[2]),
        ("1", NonZeroUsize::new(1).unwrap(), &POLICIES[3]),
        ("1", NonZeroUsize::new(1).unwrap(), &POLICIES[4]),
        (
            "18,446,744,073,709,551,615",
            NonZeroUsize::new(std::usize::MAX).unwrap(),
            &POLICIES[0],
        ),
        (
            "18𠜱446𠜱744𠜱073𠜱709𠜱551𠜱615",
            NonZeroUsize::new(std::usize::MAX).unwrap(),
            &POLICIES[1],
        ),
        (
            "1𠜱84𠜱46𠜱74𠜱40𠜱73𠜱70𠜱95𠜱51𠜱615",
            NonZeroUsize::new(std::usize::MAX).unwrap(),
            &POLICIES[2],
        ),
        (
            "18446744073709551615",
            NonZeroUsize::new(std::usize::MAX).unwrap(),
            &POLICIES[3],
        ),
        (
            "18446744073709551615",
            NonZeroUsize::new(std::usize::MAX).unwrap(),
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

#[test]
fn test_non_zero_u64() {
    let test_cases: &[(&str, NonZeroU64, &CustomFormat)] = &[
        ("1", NonZeroU64::new(1).unwrap(), &POLICIES[0]),
        ("1", NonZeroU64::new(1).unwrap(), &POLICIES[1]),
        ("1", NonZeroU64::new(1).unwrap(), &POLICIES[2]),
        ("1", NonZeroU64::new(1).unwrap(), &POLICIES[3]),
        ("1", NonZeroU64::new(1).unwrap(), &POLICIES[4]),
        (
            "18,446,744,073,709,551,615",
            NonZeroU64::new(std::u64::MAX).unwrap(),
            &POLICIES[0],
        ),
        (
            "18𠜱446𠜱744𠜱073𠜱709𠜱551𠜱615",
            NonZeroU64::new(std::u64::MAX).unwrap(),
            &POLICIES[1],
        ),
        (
            "1𠜱84𠜱46𠜱74𠜱40𠜱73𠜱70𠜱95𠜱51𠜱615",
            NonZeroU64::new(std::u64::MAX).unwrap(),
            &POLICIES[2],
        ),
        (
            "18446744073709551615",
            NonZeroU64::new(std::u64::MAX).unwrap(),
            &POLICIES[3],
        ),
        (
            "18446744073709551615",
            NonZeroU64::new(std::u64::MAX).unwrap(),
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

#[test]
fn test_non_zero_u128() {
    let test_cases: &[(&str, NonZeroU128, &CustomFormat)] = &[
        ("1", NonZeroU128::new(1).unwrap(), &POLICIES[0]),
        ("1", NonZeroU128::new(1).unwrap(), &POLICIES[1]),
        ("1", NonZeroU128::new(1).unwrap(), &POLICIES[2]),
        ("1", NonZeroU128::new(1).unwrap(), &POLICIES[3]),
        ("1", NonZeroU128::new(1).unwrap(), &POLICIES[4]),
        (
            "340,282,366,920,938,463,463,374,607,431,768,211,455",
            NonZeroU128::new(std::u128::MAX).unwrap(),
            &POLICIES[0],
        ),
        (
            "340𠜱282𠜱366𠜱920𠜱938𠜱463𠜱463𠜱374𠜱607𠜱431𠜱768𠜱211𠜱455",
            NonZeroU128::new(std::u128::MAX).unwrap(),
            &POLICIES[1],
        ),
        (
            "34𠜱02𠜱82𠜱36𠜱69𠜱20𠜱93𠜱84𠜱63𠜱46𠜱33𠜱74𠜱60𠜱74𠜱31𠜱76𠜱82𠜱11𠜱455",
            NonZeroU128::new(std::u128::MAX).unwrap(),
            &POLICIES[2],
        ),
        (
            "340282366920938463463374607431768211455",
            NonZeroU128::new(std::u128::MAX).unwrap(),
            &POLICIES[3],
        ),
        (
            "340282366920938463463374607431768211455",
            NonZeroU128::new(std::u128::MAX).unwrap(),
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
