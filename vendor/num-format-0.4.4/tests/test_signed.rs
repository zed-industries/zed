mod common;

use num_format::{Buffer, CustomFormat};
#[cfg(feature = "std")]
use num_format::{ToFormattedString, WriteFormatted};

use crate::common::POLICIES;

#[test]
fn test_i8() {
    let test_cases: &[(&str, i8, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("127", std::i8::MAX, &POLICIES[0]),
        ("127", std::i8::MAX, &POLICIES[1]),
        ("127", std::i8::MAX, &POLICIES[2]),
        ("127", std::i8::MAX, &POLICIES[3]),
        ("127", std::i8::MAX, &POLICIES[4]),
        ("-128", std::i8::MIN, &POLICIES[0]),
        ("\u{200e}-\u{200e}128", std::i8::MIN, &POLICIES[1]),
        ("\u{200e}-\u{200e}128", std::i8::MIN, &POLICIES[2]),
        ("\u{200e}-\u{200e}128", std::i8::MIN, &POLICIES[3]),
        ("\u{200e}-\u{200e}128", std::i8::MIN, &POLICIES[4]),
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
fn test_i16() {
    let test_cases: &[(&str, i16, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("32,767", std::i16::MAX, &POLICIES[0]),
        ("32𠜱767", std::i16::MAX, &POLICIES[1]),
        ("32𠜱767", std::i16::MAX, &POLICIES[2]),
        ("32767", std::i16::MAX, &POLICIES[3]),
        ("32767", std::i16::MAX, &POLICIES[4]),
        ("-32,768", std::i16::MIN, &POLICIES[0]),
        ("\u{200e}-\u{200e}32𠜱768", std::i16::MIN, &POLICIES[1]),
        ("\u{200e}-\u{200e}32𠜱768", std::i16::MIN, &POLICIES[2]),
        ("\u{200e}-\u{200e}32768", std::i16::MIN, &POLICIES[3]),
        ("\u{200e}-\u{200e}32768", std::i16::MIN, &POLICIES[4]),
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
fn test_i32() {
    let test_cases: &[(&str, i32, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("2,147,483,647", std::i32::MAX, &POLICIES[0]),
        ("2𠜱147𠜱483𠜱647", std::i32::MAX, &POLICIES[1]),
        ("2𠜱14𠜱74𠜱83𠜱647", std::i32::MAX, &POLICIES[2]),
        ("2147483647", std::i32::MAX, &POLICIES[3]),
        ("2147483647", std::i32::MAX, &POLICIES[4]),
        ("-2,147,483,648", std::i32::MIN, &POLICIES[0]),
        (
            "\u{200e}-\u{200e}2𠜱147𠜱483𠜱648",
            std::i32::MIN,
            &POLICIES[1],
        ),
        (
            "\u{200e}-\u{200e}2𠜱14𠜱74𠜱83𠜱648",
            std::i32::MIN,
            &POLICIES[2],
        ),
        ("\u{200e}-\u{200e}2147483648", std::i32::MIN, &POLICIES[3]),
        ("\u{200e}-\u{200e}2147483648", std::i32::MIN, &POLICIES[4]),
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
fn test_isize() {
    let test_cases: &[(&str, isize, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("9,223,372,036,854,775,807", std::isize::MAX, &POLICIES[0]),
        (
            "9𠜱223𠜱372𠜱036𠜱854𠜱775𠜱807",
            std::isize::MAX,
            &POLICIES[1],
        ),
        (
            "92𠜱23𠜱37𠜱20𠜱36𠜱85𠜱47𠜱75𠜱807",
            std::isize::MAX,
            &POLICIES[2],
        ),
        ("9223372036854775807", std::isize::MAX, &POLICIES[3]),
        ("9223372036854775807", std::isize::MAX, &POLICIES[4]),
        ("-9,223,372,036,854,775,808", std::isize::MIN, &POLICIES[0]),
        (
            "\u{200e}-\u{200e}9𠜱223𠜱372𠜱036𠜱854𠜱775𠜱808",
            std::isize::MIN,
            &POLICIES[1],
        ),
        (
            "\u{200e}-\u{200e}92𠜱23𠜱37𠜱20𠜱36𠜱85𠜱47𠜱75𠜱808",
            std::isize::MIN,
            &POLICIES[2],
        ),
        (
            "\u{200e}-\u{200e}9223372036854775808",
            std::isize::MIN,
            &POLICIES[3],
        ),
        (
            "\u{200e}-\u{200e}9223372036854775808",
            std::isize::MIN,
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
fn test_i64() {
    let test_cases: &[(&str, i64, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        ("9,223,372,036,854,775,807", std::i64::MAX, &POLICIES[0]),
        (
            "9𠜱223𠜱372𠜱036𠜱854𠜱775𠜱807",
            std::i64::MAX,
            &POLICIES[1],
        ),
        (
            "92𠜱23𠜱37𠜱20𠜱36𠜱85𠜱47𠜱75𠜱807",
            std::i64::MAX,
            &POLICIES[2],
        ),
        ("9223372036854775807", std::i64::MAX, &POLICIES[3]),
        ("9223372036854775807", std::i64::MAX, &POLICIES[4]),
        ("-9,223,372,036,854,775,808", std::i64::MIN, &POLICIES[0]),
        (
            "\u{200e}-\u{200e}9𠜱223𠜱372𠜱036𠜱854𠜱775𠜱808",
            std::i64::MIN,
            &POLICIES[1],
        ),
        (
            "\u{200e}-\u{200e}92𠜱23𠜱37𠜱20𠜱36𠜱85𠜱47𠜱75𠜱808",
            std::i64::MIN,
            &POLICIES[2],
        ),
        (
            "\u{200e}-\u{200e}9223372036854775808",
            std::i64::MIN,
            &POLICIES[3],
        ),
        (
            "\u{200e}-\u{200e}9223372036854775808",
            std::i64::MIN,
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
fn test_i128() {
    let test_cases: &[(&str, i128, &CustomFormat)] = &[
        ("0", 0, &POLICIES[0]),
        ("0", 0, &POLICIES[1]),
        ("0", 0, &POLICIES[2]),
        ("0", 0, &POLICIES[3]),
        ("0", 0, &POLICIES[4]),
        (
            "170,141,183,460,469,231,731,687,303,715,884,105,727",
            std::i128::MAX,
            &POLICIES[0],
        ),
        (
            "170𠜱141𠜱183𠜱460𠜱469𠜱231𠜱731𠜱687𠜱303𠜱715𠜱884𠜱105𠜱727",
            std::i128::MAX,
            &POLICIES[1],
        ),
        (
            "17𠜱01𠜱41𠜱18𠜱34𠜱60𠜱46𠜱92𠜱31𠜱73𠜱16𠜱87𠜱30𠜱37𠜱15𠜱88𠜱41𠜱05𠜱727",
            std::i128::MAX,
            &POLICIES[2],
        ),
        (
            "170141183460469231731687303715884105727",
            std::i128::MAX,
            &POLICIES[3],
        ),
        (
            "170141183460469231731687303715884105727",
            std::i128::MAX,
            &POLICIES[4],
        ),
        (
            "-170,141,183,460,469,231,731,687,303,715,884,105,728",
            std::i128::MIN,
            &POLICIES[0],
        ),
        (
            "\u{200e}-\u{200e}170𠜱141𠜱183𠜱460𠜱469𠜱231𠜱731𠜱687𠜱303𠜱715𠜱884𠜱105𠜱728",
            std::i128::MIN,
            &POLICIES[1],
        ),
        (
            "\u{200e}-\u{200e}17𠜱01𠜱41𠜱18𠜱34𠜱60𠜱46𠜱92𠜱31𠜱73𠜱16𠜱87𠜱30𠜱37𠜱15𠜱88𠜱41𠜱05𠜱728",
            std::i128::MIN,
            &POLICIES[2],
        ),
        (
            "\u{200e}-\u{200e}170141183460469231731687303715884105728",
            std::i128::MIN,
            &POLICIES[3],
        ),
        (
            "\u{200e}-\u{200e}170141183460469231731687303715884105728",
            std::i128::MIN,
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
