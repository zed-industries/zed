use num_cmp::NumCmp;
use serde_json::{Map, Value};

macro_rules! num_cmp {
    ($left:expr, $right:expr) => {
        if let Some(b) = $right.as_u64() {
            NumCmp::num_eq($left, b)
        } else if let Some(b) = $right.as_i64() {
            NumCmp::num_eq($left, b)
        } else {
            #[cfg(feature = "arbitrary-precision")]
            {
                use crate::ext::numeric::bignum;
                use fraction::BigFraction;

                let left_frac = BigFraction::from($left);

                // Check BigInt/BigFraction BEFORE f64 to avoid precision loss
                if let Some(right_bigint) = bignum::try_parse_bigint($right) {
                    let right_frac = BigFraction::from(right_bigint);
                    left_frac == right_frac
                } else if let Some(right_frac) = bignum::try_parse_bigfraction($right) {
                    left_frac == right_frac
                } else if let Some(b) = $right.as_f64() {
                    // Fallback to f64 for scientific notation or other cases
                    left_frac == BigFraction::from(b)
                } else {
                    // Can't parse right - not equal
                    false
                }
            }
            #[cfg(not(feature = "arbitrary-precision"))]
            {
                if let Some(b) = $right.as_f64() {
                    NumCmp::num_eq($left, b)
                } else {
                    unreachable!("Numbers always fit in u64/i64/f64 without arbitrary-precision")
                }
            }
        }
    };
}

/// Compare two JSON numbers for equality with arbitrary precision support
#[inline]
pub(crate) fn equal_numbers(left: &serde_json::Number, right: &serde_json::Number) -> bool {
    #[cfg(feature = "arbitrary-precision")]
    {
        use crate::ext::numeric::bignum;
        use fraction::BigFraction;

        // Check BigInt/BigFraction first to avoid precision loss from f64 conversion
        if let Some(left_bigint) = bignum::try_parse_bigint(left) {
            if let Some(right_bigint) = bignum::try_parse_bigint(right) {
                left_bigint == right_bigint
            } else if let Some(b) = right.as_u64() {
                left_bigint == num_bigint::BigInt::from(b)
            } else if let Some(b) = right.as_i64() {
                left_bigint == num_bigint::BigInt::from(b)
            } else if let Some(right_frac) = bignum::try_parse_bigfraction(right) {
                BigFraction::from(left_bigint) == right_frac
            } else if let Some(b) = right.as_f64() {
                BigFraction::from(left_bigint) == BigFraction::from(b)
            } else {
                unreachable!("Right is not parseable as any numeric type - should not happen for valid JSON numbers")
            }
        } else if let Some(left_frac) = bignum::try_parse_bigfraction(left) {
            if let Some(right_frac) = bignum::try_parse_bigfraction(right) {
                left_frac == right_frac
            } else if let Some(right_bigint) = bignum::try_parse_bigint(right) {
                left_frac == BigFraction::from(right_bigint)
            } else if let Some(b) = right.as_u64() {
                left_frac == BigFraction::from(b)
            } else if let Some(b) = right.as_i64() {
                left_frac == BigFraction::from(b)
            } else if let Some(b) = right.as_f64() {
                left_frac == BigFraction::from(b)
            } else {
                unreachable!("Right is not parseable as any numeric type - should not happen for valid JSON numbers")
            }
        } else if let Some(a) = left.as_u64() {
            num_cmp!(a, right)
        } else if let Some(a) = left.as_i64() {
            num_cmp!(a, right)
        } else if let Some(a) = left.as_f64() {
            num_cmp!(a, right)
        } else {
            // Left is a number in scientific notation that doesn't fit in f64
            // (e.g., 1e309, 1e400). With arbitrary-precision, these are stored as
            // strings but can't be converted to any numeric type we support.
            // Return false as we can't reliably compare them.
            false
        }
    }
    #[cfg(not(feature = "arbitrary-precision"))]
    {
        if let Some(a) = left.as_u64() {
            num_cmp!(a, right)
        } else if let Some(a) = left.as_i64() {
            num_cmp!(a, right)
        } else if let Some(a) = left.as_f64() {
            num_cmp!(a, right)
        } else {
            unreachable!("Numbers always fit in u64/i64/f64 without arbitrary-precision")
        }
    }
}

/// Tests for two JSON values to be equal using the JSON Schema semantic.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::String(left), Value::String(right)) => left == right,
        (Value::Bool(left), Value::Bool(right)) => left == right,
        (Value::Null, Value::Null) => true,
        (Value::Number(left), Value::Number(right)) => equal_numbers(left, right),
        (Value::Array(left), Value::Array(right)) => equal_arrays(left, right),
        (Value::Object(left), Value::Object(right)) => equal_objects(left, right),
        (_, _) => false,
    }
}

#[inline]
pub(crate) fn equal_arrays(left: &[Value], right: &[Value]) -> bool {
    left.len() == right.len() && {
        let mut idx = 0_usize;
        while idx < left.len() {
            if !equal(&left[idx], &right[idx]) {
                return false;
            }
            idx += 1;
        }
        true
    }
}

#[inline]
pub(crate) fn equal_objects(left: &Map<String, Value>, right: &Map<String, Value>) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|((ka, va), (kb, vb))| ka == kb && equal(va, vb))
}

#[cfg(test)]
mod tests {
    use super::equal;
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!(1), &json!(1.0))]
    #[test_case(&json!([2]), &json!([2.0]))]
    #[test_case(&json!([-3]), &json!([-3.0]))]
    #[test_case(&json!({"a": 1}), &json!({"a": 1.0}))]
    fn are_equal(left: &Value, right: &Value) {
        assert!(equal(left, right));
    }

    #[test_case(&json!(1), &json!(2.0))]
    #[test_case(&json!([]), &json!(["foo"]))]
    #[test_case(&json!([-3]), &json!([-4.0]))]
    #[test_case(&json!({"a": 1}), &json!({"a": 1.0, "b": 2}))]
    fn are_not_equal(left: &Value, right: &Value) {
        assert!(!equal(left, right));
    }

    #[cfg(feature = "arbitrary-precision")]
    mod arbitrary_precision {
        use super::equal;
        use serde_json::Value;
        use test_case::test_case;

        fn parse_json(s: &str) -> Value {
            serde_json::from_str(s).unwrap()
        }
        #[test_case("0.1", "0.1", true; "exact decimal match")]
        #[test_case("0.1", "0.10", true; "decimal with trailing zero")]
        #[test_case("0.1", "0.100000", true; "decimal with many trailing zeros")]
        #[test_case("0.1", "0.2", false; "different decimals")]
        #[test_case("0.3", "0.30", true; "another trailing zero case")]
        #[test_case("1.0", "1", true; "decimal vs integer")]
        #[test_case("1.00", "1.0", true; "decimals with different trailing zeros")]
        #[test_case(
            "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.5",
            "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.5",
            true;
            "huge decimal self equality"
        )]
        #[test_case(
            "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.5",
            "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.6",
            false;
            "huge decimals different"
        )]
        #[test_case(
            "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.5",
            "100",
            false;
            "huge decimal vs small integer"
        )]
        #[test_case("18446744073709551616", "18446744073709551616", true; "large integer self equality")]
        #[test_case("18446744073709551616", "18446744073709551617", false; "large integers different")]
        #[test_case("99999999999999999999999999999999999999", "99999999999999999999999999999999999999", true; "very large integer equality")]
        #[test_case("99999999999999999999999999999999999999", "100", false; "very large vs small integer")]
        #[test_case("100", "100.0", true; "small integer vs decimal")]
        #[test_case("0.1", "1", false; "small decimal vs integer")]
        #[test_case("1.0", "1", true; "decimal one vs integer one")]
        #[test_case("18446744073709551616", "100.5", false; "large int vs decimal")]
        #[test_case("18446744073709551616.0", "18446744073709551616", true; "large int as decimal vs large int")]
        #[test_case("-0.1", "-0.1", true; "negative decimal equality")]
        #[test_case("-0.1", "-0.10", true; "negative decimal trailing zero")]
        #[test_case("-18446744073709551616", "-18446744073709551616", true; "negative large int")]
        #[test_case("-18446744073709551616", "18446744073709551616", false; "negative vs positive large")]
        #[test_case("-100.5", "-100.5", true; "negative decimal match")]
        #[test_case("-100.5", "100.5", false; "negative vs positive decimal")]
        #[test_case("0", "0.0", true; "zero integer vs decimal")]
        #[test_case("0.0", "0.00", true; "zero decimals with different precision")]
        #[test_case("-0.0", "0.0", true; "negative zero vs positive zero")]
        #[test_case("1e10", "10000000000", true; "scientific notation vs integer")]
        #[test_case("1e19", "10000000000000000000", true; "scientific integer beyond i64")]
        #[test_case("1e19", "10000000000000000001", false; "scientific integer mismatch")]
        #[test_case("1.5e2", "150", true; "decimal scientific vs integer")]
        #[test_case("1.5e2", "150.0", true; "decimal scientific vs decimal")]
        #[test_case(r"[0.1, 0.2, 0.3]", r"[0.1, 0.2, 0.3]", true; "array exact match")]
        #[test_case(r"[0.1, 0.2]", r"[0.10, 0.20]", true; "array with trailing zeros")]
        #[test_case(r"[18446744073709551616]", r"[18446744073709551616]", true; "array with large integer")]
        #[test_case(r"[0.1, 0.2]", r"[0.1, 0.3]", false; "array different values")]
        #[test_case(r#"{"value": 0.1}"#, r#"{"value": 0.1}"#, true; "object exact match")]
        #[test_case(r#"{"value": 0.1}"#, r#"{"value": 0.10}"#, true; "object with trailing zero")]
        #[test_case(r#"{"id": 18446744073709551616}"#, r#"{"id": 18446744073709551616}"#, true; "object with large integer")]
        #[test_case(r#"{"value": 0.1}"#, r#"{"value": 0.2}"#, false; "object different values")]
        #[test_case("18446744073709551616", "-1", false; "large positive bigint vs negative i64")]
        #[test_case("18446744073709551616", "-100", false; "large positive bigint vs negative i64 small")]
        #[test_case("-18446744073709551616", "-1", false; "large negative bigint vs small negative i64")]
        #[test_case("18446744073709551616", "1e10", false; "large bigint vs scientific notation f64")]
        #[test_case("10000000000", "1e10", true; "bigint vs scientific notation equal")]
        #[test_case("-18446744073709551616", "-1.5e3", false; "negative bigint vs scientific notation")]
        #[test_case("0.5", "5e-1", true; "bigfraction vs scientific notation equal")]
        #[test_case("0.3", "3e-1", true; "bigfraction vs scientific equal exact")]
        #[test_case("123.456", "1.23456e2", true; "bigfraction vs scientific notation")]
        #[test_case("0.1", "1e-2", false; "bigfraction vs scientific not equal")]
        #[test_case("1e309", "1e309", true; "huge scientific notation now handled")]
        #[test_case("1e400", "1e400", true; "extreme scientific notation now handled")]
        #[test_case("1e-400", "1e-400", true; "extreme small scientific notation self equality")]
        #[test_case("1e309", "1", false; "huge scientific notation vs integer")]
        fn arbitrary_precision_equality(left_str: &str, right_str: &str, should_equal: bool) {
            let left = parse_json(left_str);
            let right = parse_json(right_str);
            assert_eq!(equal(&left, &right), should_equal);
        }
    }
}
