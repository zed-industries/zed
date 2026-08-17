use super::{FloatLit, FloatType};
use crate::{
    test_util::{assert_parse_ok_eq, assert_roundtrip},
    Literal, ParseError,
};


// ===== Utility functions =======================================================================

/// Helper macro to check parsing a float.
///
/// This macro contains quite a bit of logic itself (which can be buggy of
/// course), so we have a few test functions below to test a bunch of cases
/// manually.
macro_rules! check {
    ($intpart:literal $fracpart:literal $exppart:literal $suffix:tt) => {
        let input = concat!($intpart, $fracpart, $exppart, check!(@stringify_suffix $suffix));
        let expected_float = FloatLit {
            raw: input,
            end_integer_part: $intpart.len(),
            end_fractional_part: $intpart.len() + $fracpart.len(),
            end_number_part: $intpart.len() + $fracpart.len() + $exppart.len(),
        };

        assert_parse_ok_eq(
            input, FloatLit::parse(input), expected_float.clone(), "FloatLit::parse");
        assert_parse_ok_eq(
            input, Literal::parse(input), Literal::Float(expected_float), "Literal::parse");
        assert_eq!(FloatLit::parse(input).unwrap().suffix(), check!(@ty $suffix));
        assert_roundtrip(expected_float.to_owned(), input);
    };
    (@ty f32) => { "f32" };
    (@ty f64) => { "f64" };
    (@ty -) => { "" };
    (@stringify_suffix -) => { "" };
    (@stringify_suffix $suffix:ident) => { stringify!($suffix) };
}


// ===== Actual tests ===========================================================================

#[test]
fn manual_without_suffix() -> Result<(), ParseError> {
    let f = FloatLit::parse("3.14")?;
    assert_eq!(f.number_part(), "3.14");
    assert_eq!(f.integer_part(), "3");
    assert_eq!(f.fractional_part(), Some("14"));
    assert_eq!(f.exponent_part(), "");
    assert_eq!(f.suffix(), "");

    let f = FloatLit::parse("9.")?;
    assert_eq!(f.number_part(), "9.");
    assert_eq!(f.integer_part(), "9");
    assert_eq!(f.fractional_part(), Some(""));
    assert_eq!(f.exponent_part(), "");
    assert_eq!(f.suffix(), "");

    let f = FloatLit::parse("8e1")?;
    assert_eq!(f.number_part(), "8e1");
    assert_eq!(f.integer_part(), "8");
    assert_eq!(f.fractional_part(), None);
    assert_eq!(f.exponent_part(), "e1");
    assert_eq!(f.suffix(), "");

    let f = FloatLit::parse("8E3")?;
    assert_eq!(f.number_part(), "8E3");
    assert_eq!(f.integer_part(), "8");
    assert_eq!(f.fractional_part(), None);
    assert_eq!(f.exponent_part(), "E3");
    assert_eq!(f.suffix(), "");

    let f = FloatLit::parse("8_7_6.1_23e15")?;
    assert_eq!(f.number_part(), "8_7_6.1_23e15");
    assert_eq!(f.integer_part(), "8_7_6");
    assert_eq!(f.fractional_part(), Some("1_23"));
    assert_eq!(f.exponent_part(), "e15");
    assert_eq!(f.suffix(), "");

    let f = FloatLit::parse("8.2e-_04_9")?;
    assert_eq!(f.number_part(), "8.2e-_04_9");
    assert_eq!(f.integer_part(), "8");
    assert_eq!(f.fractional_part(), Some("2"));
    assert_eq!(f.exponent_part(), "e-_04_9");
    assert_eq!(f.suffix(), "");

    Ok(())
}

#[test]
fn manual_with_suffix() -> Result<(), ParseError> {
    let f = FloatLit::parse("3.14f32")?;
    assert_eq!(f.number_part(), "3.14");
    assert_eq!(f.integer_part(), "3");
    assert_eq!(f.fractional_part(), Some("14"));
    assert_eq!(f.exponent_part(), "");
    assert_eq!(FloatType::from_suffix(f.suffix()), Some(FloatType::F32));

    let f = FloatLit::parse("8e1f64")?;
    assert_eq!(f.number_part(), "8e1");
    assert_eq!(f.integer_part(), "8");
    assert_eq!(f.fractional_part(), None);
    assert_eq!(f.exponent_part(), "e1");
    assert_eq!(FloatType::from_suffix(f.suffix()), Some(FloatType::F64));

    let f = FloatLit::parse("8_7_6.1_23e15f32")?;
    assert_eq!(f.number_part(), "8_7_6.1_23e15");
    assert_eq!(f.integer_part(), "8_7_6");
    assert_eq!(f.fractional_part(), Some("1_23"));
    assert_eq!(f.exponent_part(), "e15");
    assert_eq!(FloatType::from_suffix(f.suffix()), Some(FloatType::F32));

    let f = FloatLit::parse("8.2e-_04_9f64")?;
    assert_eq!(f.number_part(), "8.2e-_04_9");
    assert_eq!(f.integer_part(), "8");
    assert_eq!(f.fractional_part(), Some("2"));
    assert_eq!(f.exponent_part(), "e-_04_9");
    assert_eq!(FloatType::from_suffix(f.suffix()), Some(FloatType::F64));

    Ok(())
}

#[test]
fn simple() {
    check!("3" ".14" "" -);
    check!("3" ".14" "" f32);
    check!("3" ".14" "" f64);

    check!("3" "" "e987654321" -);
    check!("3" "" "e987654321" f64);

    check!("42_888" ".05" "" -);
    check!("42_888" ".05" "E5___" f32);
    check!("123456789" "" "e_1" f64);
    check!("123456789" ".99" "e_1" f64);
    check!("123456789" ".99" "" f64);
    check!("123456789" ".99" "" -);

    check!("147" ".3_33" "" -);
    check!("147" ".3_33__" "E3" f64);
    check!("147" ".3_33__" "" f32);

    check!("147" ".333" "e-10" -);
    check!("147" ".333" "e-_7" f32);
    check!("147" ".333" "e+10" -);
    check!("147" ".333" "e+_7" f32);

    check!("86" "." "" -);
    check!("0" "." "" -);
    check!("0_" "." "" -);
    check!("0" ".0000001" "" -);
    check!("0" ".000_0001" "" -);

    check!("0" ".0" "e+0" -);
    check!("0" "" "E+0" -);
    check!("34" "" "e+0" -);
    check!("0" ".9182" "E+0" f32);
}

#[test]
fn non_standard_suffixes() {
    #[track_caller]
    fn check_suffix(
        input: &str,
        integer_part: &str,
        fractional_part: Option<&str>,
        exponent_part: &str,
        suffix: &str,
    ) {
        let lit = FloatLit::parse(input)
            .unwrap_or_else(|e| panic!("expected to parse '{}' but got {}", input, e));
        assert_eq!(lit.integer_part(), integer_part);
        assert_eq!(lit.fractional_part(), fractional_part);
        assert_eq!(lit.exponent_part(), exponent_part);
        assert_eq!(lit.suffix(), suffix);

        let lit = match Literal::parse(input) {
            Ok(Literal::Float(f)) => f,
            other => panic!("Expected float literal, but got {other:?} for '{input}'"),
        };
        assert_eq!(lit.integer_part(), integer_part);
        assert_eq!(lit.fractional_part(), fractional_part);
        assert_eq!(lit.exponent_part(), exponent_part);
        assert_eq!(lit.suffix(), suffix);
    }

    check_suffix("7.1f23", "7", Some("1"), "", "f23");
    check_suffix("7.1f320", "7", Some("1"), "", "f320");
    check_suffix("7.1f64_", "7", Some("1"), "", "f64_");
    check_suffix("8.1f649", "8", Some("1"), "", "f649");
    check_suffix("8.1f64f32", "8", Some("1"), "", "f64f32");
    check_suffix("23e2_banana", "23", None, "e2_", "banana");
    check_suffix("23.2_banana", "23", Some("2_"), "", "banana");
    check_suffix("23e2pe55ter", "23", None, "e2", "pe55ter");
    check_suffix("23e2p_e55ter", "23", None, "e2", "p_e55ter");
    check_suffix("3.15Jürgen", "3", Some("15"), "", "Jürgen");
    check_suffix("3e2e5", "3", None, "e2", "e5");
    check_suffix("3e2e5f", "3", None, "e2", "e5f");
}

#[test]
fn parse_err() {
    assert_err!(FloatLit, "", Empty, None);
    assert_err_single!(FloatLit::parse("."), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse("+"), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse("-"), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse("e"), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse("e8"), DoesNotStartWithDigit, 0);
    assert_err!(FloatLit, "0e", NoExponentDigits, 1..2);
    assert_err_single!(FloatLit::parse("f32"), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse("foo"), DoesNotStartWithDigit, 0);

    assert_err_single!(FloatLit::parse("inf"), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse("nan"), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse("NaN"), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse("NAN"), DoesNotStartWithDigit, 0);

    assert_err_single!(FloatLit::parse("_2.7"), DoesNotStartWithDigit, 0);
    assert_err_single!(FloatLit::parse(".5"), DoesNotStartWithDigit, 0);
    assert_err!(FloatLit, "1e", NoExponentDigits, 1..2);
    assert_err!(FloatLit, "1.e4", UnexpectedChar, 2);
    assert_err!(FloatLit, "3._4", UnexpectedChar, 2);
    assert_err!(FloatLit, "3.f32", UnexpectedChar, 2);
    assert_err!(FloatLit, "3.e5", UnexpectedChar, 2);
    assert_err!(FloatLit, "12345._987", UnexpectedChar, 6);
    assert_err!(FloatLit, "46._", UnexpectedChar, 3);
    assert_err!(FloatLit, "46.f32", UnexpectedChar, 3);
    assert_err!(FloatLit, "46.e3", UnexpectedChar, 3);
    assert_err!(FloatLit, "46._e3", UnexpectedChar, 3);
    assert_err!(FloatLit, "46.e3f64", UnexpectedChar, 3);
    assert_err!(FloatLit, "23.4e_", NoExponentDigits, 4..6);
    assert_err!(FloatLit, "23E___f32", NoExponentDigits, 2..6);
    assert_err!(FloatLit, "55e3.1", UnexpectedChar, 4..6);

    assert_err!(FloatLit, "3.7+", UnexpectedChar, 3..4);
    assert_err!(FloatLit, "3.7+2", UnexpectedChar, 3..5);
    assert_err!(FloatLit, "3.7-", UnexpectedChar, 3..4);
    assert_err!(FloatLit, "3.7-2", UnexpectedChar, 3..5);
    assert_err!(FloatLit, "3.7e+", NoExponentDigits, 3..5);
    assert_err!(FloatLit, "3.7e-", NoExponentDigits, 3..5);
    assert_err!(FloatLit, "3.7e-+3", NoExponentDigits, 3..5); // suboptimal error
    assert_err!(FloatLit, "3.7e+-3", NoExponentDigits, 3..5); // suboptimal error
    assert_err_single!(FloatLit::parse("0x44.5"), InvalidSuffix, 1..6);

    assert_err_single!(FloatLit::parse("3"), UnexpectedIntegerLit, None);
    assert_err_single!(FloatLit::parse("35_389"), UnexpectedIntegerLit, None);
    assert_err_single!(FloatLit::parse("9_8_7f32"), UnexpectedIntegerLit, None);
    assert_err_single!(FloatLit::parse("9_8_7banana"), UnexpectedIntegerLit, None);
    assert_err_single!(FloatLit::parse("7f23"), UnexpectedIntegerLit, None);
    assert_err_single!(FloatLit::parse("7f320"), UnexpectedIntegerLit, None);
    assert_err_single!(FloatLit::parse("7f64_"), UnexpectedIntegerLit, None);
    assert_err_single!(FloatLit::parse("8f649"), UnexpectedIntegerLit, None);
    assert_err_single!(FloatLit::parse("8f64f32"), UnexpectedIntegerLit, None);
}
