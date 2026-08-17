use crate::{
    compiler,
    error::ValidationError,
    ext::numeric,
    keywords::CompilationResult,
    paths::{LazyLocation, Location},
    types::JsonType,
    validator::{Validate, ValidationContext},
};
use serde_json::{Map, Value};

pub(crate) struct MultipleOfFloatValidator {
    multiple_of: f64,
    #[cfg(feature = "arbitrary-precision")]
    original_value: serde_json::Number,
    location: Location,
}

impl MultipleOfFloatValidator {
    #[inline]
    pub(crate) fn compile<'a>(
        multiple_of: f64,
        #[cfg(feature = "arbitrary-precision")] original_value: &serde_json::Number,
        location: Location,
    ) -> CompilationResult<'a> {
        Ok(Box::new(MultipleOfFloatValidator {
            multiple_of,
            #[cfg(feature = "arbitrary-precision")]
            original_value: original_value.clone(),
            location,
        }))
    }
}

impl Validate for MultipleOfFloatValidator {
    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        if let Value::Number(item) = instance {
            numeric::is_multiple_of_float(item, self.multiple_of)
        } else {
            true
        }
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if !self.is_valid(instance, ctx) {
            #[cfg(feature = "arbitrary-precision")]
            {
                return Err(ValidationError::multiple_of(
                    self.location.clone(),
                    location.into(),
                    instance,
                    Value::Number(self.original_value.clone()),
                ));
            }
            #[cfg(not(feature = "arbitrary-precision"))]
            {
                return Err(ValidationError::multiple_of(
                    self.location.clone(),
                    location.into(),
                    instance,
                    self.multiple_of,
                ));
            }
        }
        Ok(())
    }
}

pub(crate) struct MultipleOfIntegerValidator {
    multiple_of: f64,
    #[cfg(feature = "arbitrary-precision")]
    original_value: serde_json::Number,
    location: Location,
}

impl MultipleOfIntegerValidator {
    #[inline]
    pub(crate) fn compile<'a>(
        multiple_of: f64,
        #[cfg(feature = "arbitrary-precision")] original_value: &serde_json::Number,
        location: Location,
    ) -> CompilationResult<'a> {
        Ok(Box::new(MultipleOfIntegerValidator {
            multiple_of,
            #[cfg(feature = "arbitrary-precision")]
            original_value: original_value.clone(),
            location,
        }))
    }
}

impl Validate for MultipleOfIntegerValidator {
    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        if let Value::Number(item) = instance {
            numeric::is_multiple_of_integer(item, self.multiple_of)
        } else {
            true
        }
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if !self.is_valid(instance, ctx) {
            #[cfg(feature = "arbitrary-precision")]
            {
                return Err(ValidationError::multiple_of(
                    self.location.clone(),
                    location.into(),
                    instance,
                    Value::Number(self.original_value.clone()),
                ));
            }
            #[cfg(not(feature = "arbitrary-precision"))]
            {
                return Err(ValidationError::multiple_of(
                    self.location.clone(),
                    location.into(),
                    instance,
                    self.multiple_of,
                ));
            }
        }
        Ok(())
    }
}

#[cfg(feature = "arbitrary-precision")]
pub(crate) struct MultipleOfBigIntValidator {
    multiple_of: num_bigint::BigInt,
    original_value: serde_json::Number,
    location: Location,
}

#[cfg(feature = "arbitrary-precision")]
impl MultipleOfBigIntValidator {
    #[inline]
    pub(crate) fn compile<'a>(
        multiple_of: num_bigint::BigInt,
        original_value: &serde_json::Number,
        location: Location,
    ) -> CompilationResult<'a> {
        Ok(Box::new(MultipleOfBigIntValidator {
            multiple_of,
            original_value: original_value.clone(),
            location,
        }))
    }
}

#[cfg(feature = "arbitrary-precision")]
impl Validate for MultipleOfBigIntValidator {
    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        use num_bigint::BigInt;
        use num_traits::One;
        if let Value::Number(item) = instance {
            if let Some(instance_bigint) = numeric::bignum::try_parse_bigint(item) {
                numeric::bignum::is_multiple_of_bigint(&instance_bigint, &self.multiple_of)
            } else if let Some(v) = item.as_u64() {
                let v_bigint = BigInt::from(v);
                numeric::bignum::is_multiple_of_bigint(&v_bigint, &self.multiple_of)
            } else if let Some(v) = item.as_i64() {
                let v_bigint = BigInt::from(v);
                numeric::bignum::is_multiple_of_bigint(&v_bigint, &self.multiple_of)
            } else if let Some(v) = item.as_f64() {
                if v.fract() != 0.0 {
                    return false;
                }
                // JSON numbers that originate as f64 are only exactly representable if their
                // absolute value is below 2^53. Within that "safe integer" window we can round-trip
                // to i64 without losing bits, so convert and reuse the bigint path.
                #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
                if v.is_finite() && v.abs() < (1u64 << 53) as f64 {
                    let v_bigint = BigInt::from(v as i64);
                    numeric::bignum::is_multiple_of_bigint(&v_bigint, &self.multiple_of)
                } else {
                    // 2^53 and above can no longer be recovered exactly from the binary64 mantissa,
                    // so we bail out instead of pretending we know whether the bigint divisor divides
                    // such an approximation.
                    false
                }
            } else {
                // as_f64() returning None means the literal overflowed binary64 (e.g. 1e10000) or was
                // written in plain decimal form with more precision than f64 can carry. We attempt to
                // parse it as an exact BigFraction and only accept it when the denominator is 1 so we
                // can safely convert it to BigInt before running the modulo check below.
                if let Some(instance_bigfrac) = numeric::bignum::try_parse_bigfraction(item) {
                    if instance_bigfrac.denom().is_none_or(One::is_one) {
                        if let Some(numer) = instance_bigfrac.numer() {
                            let instance_bigint = BigInt::from(numer.clone());
                            return numeric::bignum::is_multiple_of_bigint(
                                &instance_bigint,
                                &self.multiple_of,
                            );
                        }
                    }
                }
                // If we made it here we encountered scientific notation that can’t be normalized, or
                // a decimal with a fractional part. Either way it cannot be an integer multiple.
                false
            }
        } else {
            true
        }
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if !self.is_valid(instance, ctx) {
            return Err(ValidationError::multiple_of(
                self.location.clone(),
                location.into(),
                instance,
                Value::Number(self.original_value.clone()),
            ));
        }
        Ok(())
    }
}

#[cfg(feature = "arbitrary-precision")]
pub(crate) struct MultipleOfBigFracValidator {
    multiple_of: fraction::BigFraction,
    original_value: serde_json::Number,
    location: Location,
}

#[cfg(feature = "arbitrary-precision")]
impl MultipleOfBigFracValidator {
    #[inline]
    pub(crate) fn compile<'a>(
        multiple_of: fraction::BigFraction,
        original_value: &serde_json::Number,
        location: Location,
    ) -> CompilationResult<'a> {
        Ok(Box::new(MultipleOfBigFracValidator {
            multiple_of,
            original_value: original_value.clone(),
            location,
        }))
    }
}

#[cfg(feature = "arbitrary-precision")]
impl Validate for MultipleOfBigFracValidator {
    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        use num_traits::ToPrimitive;
        if let Value::Number(item) = instance {
            // Try to parse instance as BigFraction for exact precision
            if let Some(instance_bigfrac) = numeric::bignum::try_parse_bigfraction(item) {
                numeric::bignum::is_multiple_of_bigfrac(&instance_bigfrac, &self.multiple_of)
            } else if let Some(instance_bigint) = numeric::bignum::try_parse_bigint(item) {
                let value_frac = fraction::BigFraction::from(instance_bigint);
                numeric::bignum::is_multiple_of_bigfrac(&value_frac, &self.multiple_of)
            } else if let Some(v) = item.as_u64() {
                let v_frac = fraction::BigFraction::from(v);
                numeric::bignum::is_multiple_of_bigfrac(&v_frac, &self.multiple_of)
            } else if let Some(v) = item.as_i64() {
                let v_frac = fraction::BigFraction::from(v);
                numeric::bignum::is_multiple_of_bigfrac(&v_frac, &self.multiple_of)
            } else {
                // Fall back to the regular float-based validation since precision is already lost
                let multiple_f64 = self.multiple_of.to_f64().unwrap_or(f64::INFINITY);
                numeric::is_multiple_of_float(item, multiple_f64)
            }
        } else {
            true
        }
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if !self.is_valid(instance, ctx) {
            return Err(ValidationError::multiple_of(
                self.location.clone(),
                location.into(),
                instance,
                Value::Number(self.original_value.clone()),
            ));
        }
        Ok(())
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    if let Value::Number(multiple_of) = schema {
        let location = ctx.location().join("multipleOf");

        #[cfg(feature = "arbitrary-precision")]
        {
            // Try BigInt first for large integers
            if let Some(bigint_multiple) = numeric::bignum::try_parse_bigint(multiple_of) {
                return Some(MultipleOfBigIntValidator::compile(
                    bigint_multiple,
                    multiple_of,
                    location,
                ));
            }
            // Then try BigFraction for exact decimal precision
            if let Some(bigfrac_multiple) = numeric::bignum::try_parse_bigfraction(multiple_of) {
                return Some(MultipleOfBigFracValidator::compile(
                    bigfrac_multiple,
                    multiple_of,
                    location,
                ));
            }
        }

        // If not BigInt or BigFraction, try to represent as f64
        // Note: Extreme scientific notation (e.g., 1e309, 1e400) cannot fit in f64
        if let Some(f64_value) = multiple_of.as_f64() {
            #[cfg(feature = "arbitrary-precision")]
            {
                if f64_value.fract() == 0. {
                    Some(MultipleOfIntegerValidator::compile(
                        f64_value,
                        multiple_of,
                        location,
                    ))
                } else {
                    Some(MultipleOfFloatValidator::compile(
                        f64_value,
                        multiple_of,
                        location,
                    ))
                }
            }
            #[cfg(not(feature = "arbitrary-precision"))]
            {
                if f64_value.fract() == 0. {
                    Some(MultipleOfIntegerValidator::compile(f64_value, location))
                } else {
                    Some(MultipleOfFloatValidator::compile(f64_value, location))
                }
            }
        } else {
            // Extreme numbers beyond f64 range (e.g., 1e309) - not supported
            // Skip this keyword - all instances will be considered valid
            None
        }
    } else {
        Some(Err(ValidationError::single_type_error(
            Location::new(),
            ctx.location().clone(),
            schema,
            JsonType::Number,
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_util;
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!({"multipleOf": 2}), &json!(4))]
    #[test_case(&json!({"multipleOf": 1.0}), &json!(4.0))]
    #[test_case(&json!({"multipleOf": 1.5}), &json!(3.0))]
    #[test_case(&json!({"multipleOf": 1.5}), &json!(4.5))]
    #[test_case(&json!({"multipleOf": 0.1}), &json!(12.2))]
    #[test_case(&json!({"multipleOf": 0.0001}), &json!(3.1254))]
    #[test_case(&json!({"multipleOf": 0.0001}), &json!(47.498))]
    #[test_case(&json!({"multipleOf": 0.1}), &json!(1.1))]
    #[test_case(&json!({"multipleOf": 0.1}), &json!(1.2))]
    #[test_case(&json!({"multipleOf": 0.1}), &json!(1.3))]
    #[test_case(&json!({"multipleOf": 0.02}), &json!(1.02))]
    #[test_case(&json!({"multipleOf": 1e-16}), &json!(1e-15))]
    fn multiple_of_is_valid(schema: &Value, instance: &Value) {
        tests_util::is_valid(schema, instance);
    }

    #[test_case(&json!({"multipleOf": 1.0}), &json!(4.5))]
    #[test_case(&json!({"multipleOf": 0.1}), &json!(4.55))]
    #[test_case(&json!({"multipleOf": 0.2}), &json!(4.5))]
    #[test_case(&json!({"multipleOf": 0.02}), &json!(1.01))]
    #[test_case(&json!({"multipleOf": 1.3}), &json!(1.3e-16))]
    fn multiple_of_is_not_valid(schema: &Value, instance: &Value) {
        tests_util::is_not_valid(schema, instance);
    }

    #[test_case(&json!({"multipleOf": 2}), &json!(3), "/multipleOf")]
    #[test_case(&json!({"multipleOf": 1.5}), &json!(5), "/multipleOf")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }

    #[cfg(feature = "arbitrary-precision")]
    mod arbitrary_precision {
        use crate::tests_util;
        use serde_json::Value;
        use test_case::test_case;

        fn parse_json(s: &str) -> Value {
            serde_json::from_str(s).unwrap()
        }

        #[test_case(r#"{"multipleOf": 0.1}"#, "0.3", true; "bigfrac 0.3 is multiple of 0.1")]
        #[test_case(r#"{"multipleOf": 0.1}"#, "0.35", false; "bigfrac 0.35 not multiple of 0.1")]
        #[test_case(r#"{"multipleOf": 0.01}"#, "0.03", true; "bigfrac 0.03 is multiple of 0.01")]
        #[test_case(r#"{"multipleOf": 0.1}"#, "10", true; "bigfrac validator with u64 instance")]
        #[test_case(r#"{"multipleOf": 0.5}"#, "-2", true; "bigfrac validator with i64 instance")]
        #[test_case(r#"{"multipleOf": 0.3}"#, "0.0", true; "bigfrac zero is multiple")]
        #[test_case(r#"{"multipleOf": 0.1}"#, "10.05", false; "bigfrac 10.05 not multiple of 0.1")]
        #[test_case(r#"{"multipleOf": 0.5}"#, "-1.3", false; "bigfrac -1.3 not multiple of 0.5")]
        #[test_case(r#"{"minimum": 0.1}"#, "0.3", true; "bigfrac minimum 0.3 >= 0.1")]
        #[test_case(r#"{"minimum": 0.1}"#, "0.05", false; "bigfrac minimum 0.05 < 0.1")]
        #[test_case(r#"{"maximum": 0.5}"#, "0.3", true; "bigfrac maximum 0.3 <= 0.5")]
        #[test_case(r#"{"maximum": 0.5}"#, "0.7", false; "bigfrac maximum 0.7 > 0.5")]
        #[test_case(r#"{"multipleOf": 2}"#, "1e1000", true; "huge scientific integer is even")]
        #[test_case(r#"{"multipleOf": 3}"#, "1e1000", false; "10^1000 not multiple of 3")]
        #[test_case(r#"{"multipleOf": 0.5}"#, "1e1000", true; "huge scientific integer multiple of 0.5")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "36893488147419103232", true; "large bigint multiple")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "18446744073709551617", false; "large bigint non-multiple")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "100", false; "small int not multiple of large")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "-100", false; "negative small int not multiple")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "0", true; "zero is multiple of large bigint")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "1.5", false; "decimal not multiple of bigint")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "1e20", false; "scientific notation not multiple")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "9223372036854775808", false; "2^63 not multiple of 2^64")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "1e308", true; "huge float multiple of 2^64")]
        #[test_case(r#"{"multipleOf": 1e19}"#, "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", true; "1e19 scientific notation divisor with huge instance that is exact multiple")]
        #[test_case(r#"{"multipleOf": 2e19}"#, "200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", true; "2e19 scientific notation divisor with huge instance that is exact multiple")]
        #[test_case(r#"{"multipleOf": 1000000000000000000}"#, "1e18", true; "1e18 is 10^18")]
        #[test_case(r#"{"multipleOf": 1000000000000000000}"#, "2e18", true; "2e18 is 2*10^18")]
        #[test_case(r#"{"multipleOf": 1000000000000000000}"#, "1.5e18", false; "1.5e18 not integer multiple")]
        #[test_case(r#"{"multipleOf": 3}"#, "1e20", false; "1e20 not multiple of 3")]
        #[test_case(r#"{"minimum": 18446744073709551616}"#, "1e100", true; "bigint min with huge number")]
        #[test_case(r#"{"maximum": 18446744073709551616}"#, "1e100", false; "bigint max with huge number")]
        #[test_case(r#"{"multipleOf": 1e309}"#, "1e309", true; "extreme schema value beyond f64 - schema compilation should handle")]
        #[test_case(r#"{"multipleOf": 1e400}"#, "2e400", true; "extreme schema values - both unsupported")]
        #[test_case(r#"{"minimum": 1e309}"#, "1e310", true; "extreme minimum - instance validation")]
        #[test_case(r#"{"maximum": 1e309}"#, "1e308", true; "extreme maximum - instance validation")]
        #[test_case(r#"{"exclusiveMinimum": 18446744073709551616}"#, "1e100", true; "bigint exclusive min")]
        #[test_case(r#"{"exclusiveMaximum": 18446744073709551616}"#, "1e10", true; "bigint exclusive max")]
        #[test_case(r#"{"minimum": 18446744073709551616}"#, "100", false; "small number below bigint min")]
        #[test_case(r#"{"maximum": 18446744073709551616}"#, "99999999999999999999", false; "large number above bigint max")]
        fn arbitrary_precision_validation(schema_json: &str, instance_json: &str, expected: bool) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "4.0", false; "safe_float_not_multiple")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "0.0", true; "safe_float_zero_multiple")]
        #[test_case(
            r#"{"multipleOf": 18446744073709551616}"#,
            "18014398509481984.0",
            false;
            "float_beyond_safe_range_rejected"
        )]
        fn bigint_validator_float_instances(
            schema_json: &str,
            instance_json: &str,
            expected: bool,
        ) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        #[test]
        fn bigint_validator_accepts_bigfraction_integer_instances() {
            let schema = parse_json(r#"{"multipleOf": 18446744073709551616}"#);
            let mut huge = "18446744073709551616".to_owned();
            huge.push_str(&"0".repeat(400));
            huge.push_str(".0");
            let instance = parse_json(&huge);
            tests_util::is_valid(&schema, &instance);
        }

        // Test cases for dynamically generated huge numbers
        #[test_case("2", 100, 2, true; "2 followed by 100 zeros is multiple of 2")]
        #[test_case("2", 200, 3, false; "2 followed by 200 zeros not multiple of 3")]
        #[test_case("1", 100, 3, false; "1 followed by 100 zeros not multiple of 3")]
        #[test_case("3", 400, 7, false; "3 followed by 400 zeros not multiple of 7")]
        #[test_case("2", 100, 2, true; "huge bigint is multiple")]
        fn generated_huge_numbers(prefix: &str, zeros: usize, divisor: u64, expected: bool) {
            let schema_json = format!(r#"{{"multipleOf": {divisor}}}"#);
            let schema = parse_json(&schema_json);
            let huge = prefix.to_string() + &"0".repeat(zeros);
            let instance = parse_json(&huge);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        // Test non-number instances with MultipleOfFloatValidator
        // Use serde_json::json! macro to avoid arbitrary-precision parsing
        #[test_case(&serde_json::json!({"multipleOf": 2.5}), &serde_json::json!("string"); "float string")]
        #[test_case(&serde_json::json!({"multipleOf": 2.5}), &serde_json::json!(true); "float bool")]
        #[test_case(&serde_json::json!({"multipleOf": 2.5}), &serde_json::json!([]); "float array")]
        #[test_case(&serde_json::json!({"multipleOf": 2.5}), &serde_json::json!({}); "float object")]
        fn non_number_with_float_validator(schema: &Value, instance: &Value) {
            tests_util::is_valid(schema, instance);
        }

        // Test non-number instances with BigInt/BigFrac validators (via JSON parsing)
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, r#""string""#; "bigint string")]
        #[test_case(r#"{"multipleOf": 0.1}"#, r"true"; "bigfrac bool")]
        #[test_case(r#"{"minimum": 18446744073709551616}"#, r"[]"; "bigint min array")]
        fn non_number_with_bignum_validators(schema_json: &str, instance_json: &str) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            tests_util::is_valid(&schema, &instance);
        }

        // Test validate() error paths
        #[test_case(r#"{"multipleOf": 2.5}"#, "5.1"; "float validator error")]
        #[test_case(r#"{"multipleOf": 3}"#, "7"; "integer validator error")]
        #[test_case(r#"{"multipleOf": 0.3}"#, "0.7"; "bigfrac validator error")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "123"; "bigint validator error")]
        #[test_case(r#"{"minimum": 18446744073709551616}"#, "100"; "bigint minimum error")]
        #[test_case(r#"{"maximum": 18446744073709551616}"#, "99999999999999999999"; "bigint maximum error")]
        fn validator_error_paths(schema_json: &str, instance_json: &str) {
            let schema = parse_json(schema_json);
            let validator = crate::validator_for(&schema).unwrap();
            let instance = parse_json(instance_json);
            assert!(validator.validate(&instance).is_err());
        }

        #[test]
        fn invalid_multiple_of_schema() {
            let schema = parse_json(r#"{"multipleOf": "not a number"}"#);
            assert!(crate::validator_for(&schema).is_err());
        }

        #[test]
        fn huge_bigfraction_beyond_f64() {
            // Test BigFraction parsing for numbers way beyond f64::MAX
            let schema = parse_json(r#"{"multipleOf": 7}"#);
            // Number not divisible by 7 (sum of digits = 3, not divisible by 7)
            let huge = "3".to_string() + &"0".repeat(400);
            let instance = parse_json(&huge);
            tests_util::is_not_valid(&schema, &instance);
        }

        #[test]
        fn large_u64_divisor_beyond_i64_max() {
            // Test for the bug where u64 divisors beyond i64::MAX were incorrectly handled
            // 10^19 = 10000000000000000000 is beyond i64::MAX but within u64::MAX
            // This should now use BigIntValidator to preserve exact value
            let schema = parse_json(r#"{"multipleOf": 10000000000000000000}"#);

            // 10^320 is a valid multiple of 10^19 (it's 10^19 * 10^301)
            let huge_multiple = "1".to_string() + &"0".repeat(320);
            let instance_valid = parse_json(&huge_multiple);
            tests_util::is_valid(&schema, &instance_valid);

            // 10^320 + 1 is NOT a multiple of 10^19
            let mut huge_non_multiple = "1".to_string() + &"0".repeat(320);
            huge_non_multiple = huge_non_multiple[..huge_non_multiple.len() - 1].to_string() + "1";
            let instance_invalid = parse_json(&huge_non_multiple);
            tests_util::is_not_valid(&schema, &instance_invalid);
        }

        #[test]
        fn u64_max_divisor() {
            // Test with exactly u64::MAX as divisor
            let schema = parse_json(r#"{"multipleOf": 18446744073709551615}"#);

            // u64::MAX * 2 (expressed as a large number)
            let double = "36893488147419103230";
            let instance = parse_json(double);
            tests_util::is_valid(&schema, &instance);
        }

        #[test]
        fn huge_decimal_beyond_f64_but_parseable() {
            // This number is so large it overflows f64 to infinity (as_f64() returns None)
            // but can still be parsed as BigFraction - hits MultipleOfBigIntValidator else branch
            let divisor = "18446744073709551616"; // 2^64
            let schema_json = format!(r#"{{"multipleOf": {divisor}}}"#);
            let schema = parse_json(&schema_json);

            // Valid: 2^64 * 10^400 - parseable as BigFraction
            let huge = divisor.to_string() + &"0".repeat(400) + ".0";
            let instance = parse_json(&huge);
            tests_util::is_valid(&schema, &instance);

            // Invalid: all 1s - parseable as BigFraction but not a multiple
            let huge_non_multiple = "1".repeat(400) + ".0";
            let instance = parse_json(&huge_non_multiple);
            tests_util::is_not_valid(&schema, &instance);
        }

        #[test]
        fn huge_scientific_notation_else_branch_unparsable() {
            // This hits the else branch but CANNOT be parsed as BigFraction (scientific notation)
            // as_f64() returns None (infinity), try_parse_bigfraction returns None (has exponent)
            let divisor = "18446744073709551616"; // 2^64
            let schema_json = format!(r#"{{"multipleOf": {divisor}}}"#);
            let schema = parse_json(&schema_json);

            // Scientific notation that overflows to infinity - cannot be parsed as BigFraction
            let infinity_positive = parse_json("1e1000001");
            tests_util::is_not_valid(&schema, &infinity_positive);

            let infinity_negative = parse_json("-1e1000001");
            tests_util::is_not_valid(&schema, &infinity_negative);

            // Huge scientific notation decimal (not just integer exponent)
            let huge_scientific = parse_json("1.5e1000002");
            tests_util::is_not_valid(&schema, &huge_scientific);
        }

        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "1.0", false; "bigint divisor small float")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "2.0", false; "bigint divisor 2.0")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "1000.0", false; "bigint divisor 1000.0")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "18446744073709551616", true; "bigint schema u64 instance")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "-18446744073709551616", true; "bigint schema neg i64")]
        #[test_case(r#"{"minimum": 18446744073709551616}"#, "18446744073709551616", true; "bigint min u64 exact")]
        #[test_case(r#"{"maximum": 18446744073709551616}"#, "0", true; "bigint max small i64")]
        #[test_case(r#"{"exclusiveMinimum": 9223372036854775807}"#, "9223372036854775808", true; "i64_max exclusive u64")]
        #[test_case(r#"{"minimum": 0.1}"#, "18446744073709551616", true; "bigfrac schema large int")]
        #[test_case(r#"{"multipleOf": 0.5}"#, "36893488147419103232", true; "bigfrac schema huge int")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "9223372036854775808", false; "bigint half value")]
        #[test_case(r#"{"minimum": 18446744073709551616}"#, "9223372036854775807", false; "bigint min i64_max")]
        #[test_case(r#"{"maximum": 9223372036854775807}"#, "18446744073709551616", false; "i64_max max u64")]
        #[test_case(r#"{"exclusiveMaximum": 18446744073709551616}"#, "18446744073709551616", false; "bigint exmax boundary")]
        #[test_case(r#"{"multipleOf": 0.3}"#, "18446744073709551616", false; "bigfrac 0.3 large int")]
        fn mixed_type_comparisons(schema_json: &str, instance_json: &str, expected: bool) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        #[test_case(r#"{"multipleOf": 0.1}"#, "0.1", true; "0.1 multiple 0.1")]
        #[test_case(r#"{"multipleOf": 0.1}"#, "0.2", true; "0.2 multiple 0.1")]
        #[test_case(r#"{"multipleOf": 0.1}"#, "0.7", true; "0.7 multiple 0.1")]
        #[test_case(r#"{"multipleOf": 0.1}"#, "0.11", false; "0.11 not multiple 0.1")]
        #[test_case(r#"{"multipleOf": 0.01}"#, "0.99", true; "0.99 multiple 0.01")]
        #[test_case(r#"{"multipleOf": 0.01}"#, "0.999", false; "0.999 not multiple 0.01")]
        #[test_case(r#"{"multipleOf": 0.001}"#, "1.234", true; "1.234 multiple 0.001")]
        #[test_case(r#"{"multipleOf": 0.001}"#, "1.2345", false; "1.2345 not multiple 0.001")]
        #[test_case(r#"{"minimum": 0.1}"#, "0.1", true; "min 0.1 equal")]
        #[test_case(r#"{"maximum": 0.3}"#, "0.3", true; "max 0.3 equal")]
        #[test_case(r#"{"minimum": 0.1}"#, "0.09999999999999999", false; "below 0.1")]
        #[test_case(r#"{"maximum": 0.1}"#, "0.10000000000000001", false; "above 0.1")]
        #[test_case(r#"{"const": 0.3}"#, "0.3", true; "const 0.3")]
        #[test_case(r#"{"const": 0.1}"#, "0.10000000000000002", false; "const close not equal")]
        #[test_case(r#"{"minimum": 1.0}"#, "1", true; "decimal min int equal")]
        #[test_case(r#"{"minimum": 1.5}"#, "1", false; "decimal min int below")]
        #[test_case(r#"{"minimum": 1.5}"#, "2", true; "decimal min int above")]
        #[test_case(r#"{"maximum": 2.5}"#, "2", true; "decimal max int below")]
        #[test_case(r#"{"maximum": 2.5}"#, "3", false; "decimal max int above")]
        #[test_case(r#"{"multipleOf": 0.5}"#, "1", true; "0.5 multiple int 1")]
        #[test_case(r#"{"multipleOf": 0.5}"#, "2", true; "0.5 multiple int 2")]
        #[test_case(r#"{"multipleOf": 0.5}"#, "3", true; "0.5 multiple int 3")]
        #[test_case(r#"{"multipleOf": 0.3}"#, "1", false; "0.3 not multiple int 1")]
        fn decimal_precision_tests(schema_json: &str, instance_json: &str, expected: bool) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "-36893488147419103232", true; "pos bigint neg instance")]
        #[test_case(r#"{"minimum": -18446744073709551616}"#, "-18446744073709551615", true; "neg bigint minimum")]
        #[test_case(r#"{"maximum": -18446744073709551616}"#, "-18446744073709551617", true; "neg bigint maximum")]
        #[test_case(r#"{"minimum": -99999999999999999999}"#, "-99999999999999999998", true; "huge neg minimum")]
        #[test_case(r#"{"maximum": -99999999999999999999}"#, "-100000000000000000000", true; "huge neg maximum")]
        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "0", true; "zero multiple large")]
        #[test_case(r#"{"minimum": -18446744073709551616}"#, "0", true; "zero above neg min")]
        #[test_case(r#"{"maximum": 18446744073709551616}"#, "0", true; "zero below large max")]
        fn negative_large_numbers(schema_json: &str, instance_json: &str, expected: bool) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        #[test_case(r#"{"minimum": 9223372036854775807}"#, "9223372036854775807", true; "i64_max min equal")]
        #[test_case(r#"{"minimum": 9223372036854775807}"#, "9223372036854775808", true; "i64_max min plus")]
        #[test_case(r#"{"maximum": 18446744073709551615}"#, "18446744073709551615", true; "u64_max equal")]
        #[test_case(r#"{"maximum": 18446744073709551615}"#, "18446744073709551616", false; "u64_max plus")]
        #[test_case(r#"{"maximum": -9223372036854775808}"#, "-9223372036854775808", true; "i64_min equal")]
        #[test_case(r#"{"maximum": -9223372036854775808}"#, "-9223372036854775809", true; "i64_min minus")]
        fn boundary_conditions(schema_json: &str, instance_json: &str, expected: bool) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        #[test_case(r#"{"minimum": 99999999999999999999}"#, "99999999999999999999", true; "min equal")]
        #[test_case(r#"{"minimum": 99999999999999999999}"#, "100000000000000000000", true; "min greater")]
        #[test_case(r#"{"minimum": 99999999999999999999}"#, "99999999999999999998", false; "min less")]
        #[test_case(r#"{"maximum": 99999999999999999999}"#, "99999999999999999999", true; "max equal")]
        #[test_case(r#"{"maximum": 99999999999999999999}"#, "100000000000000000000", false; "max greater")]
        #[test_case(r#"{"maximum": 99999999999999999999}"#, "99999999999999999998", true; "max less")]
        #[test_case(r#"{"exclusiveMinimum": 99999999999999999999}"#, "99999999999999999999", false; "exmin equal")]
        #[test_case(r#"{"exclusiveMinimum": 99999999999999999999}"#, "100000000000000000000", true; "exmin greater")]
        #[test_case(r#"{"exclusiveMinimum": 99999999999999999999}"#, "99999999999999999998", false; "exmin less")]
        #[test_case(r#"{"exclusiveMaximum": 99999999999999999999}"#, "99999999999999999999", false; "exmax equal")]
        #[test_case(r#"{"exclusiveMaximum": 99999999999999999999}"#, "100000000000000000000", false; "exmax greater")]
        #[test_case(r#"{"exclusiveMaximum": 99999999999999999999}"#, "99999999999999999998", true; "exmax less")]
        #[test_case(r#"{"minimum": 0.5}"#, "0.5", true; "frac min equal")]
        #[test_case(r#"{"minimum": 0.5}"#, "0.6", true; "frac min greater")]
        #[test_case(r#"{"minimum": 0.5}"#, "0.4", false; "frac min less")]
        #[test_case(r#"{"maximum": 0.5}"#, "0.5", true; "frac max equal")]
        #[test_case(r#"{"maximum": 0.5}"#, "0.6", false; "frac max greater")]
        #[test_case(r#"{"maximum": 0.5}"#, "0.4", true; "frac max less")]
        fn comparison_operators(schema_json: &str, instance_json: &str, expected: bool) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        #[test_case(r#"{"const": 18446744073709551616}"#, "18446744073709551616", true; "bigint match")]
        #[test_case(r#"{"const": 18446744073709551616}"#, "18446744073709551617", false; "bigint mismatch")]
        #[test_case(r#"{"const": 0.1}"#, "0.1", true; "decimal match")]
        #[test_case(r#"{"const": 0.1}"#, "0.10", true; "decimal trailing zero")]
        #[test_case(r#"{"const": 0.1}"#, "0.2", false; "decimal mismatch")]
        #[test_case(r#"{"const": -99999999999999999999}"#, "-99999999999999999999", true; "neg bigint")]
        fn const_large_numbers(schema_json: &str, instance_json: &str, expected: bool) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            if expected {
                tests_util::is_valid(&schema, &instance);
            } else {
                tests_util::is_not_valid(&schema, &instance);
            }
        }

        #[test_case(r#"{"multipleOf": "not a number"}"#; "string")]
        #[test_case(r#"{"multipleOf": null}"#; "null")]
        #[test_case(r#"{"multipleOf": []}"#; "array")]
        #[test_case(r#"{"multipleOf": {}}"#; "object")]
        #[test_case(r#"{"multipleOf": true}"#; "boolean")]
        fn malformed_schema_errors(schema_json: &str) {
            let schema = parse_json(schema_json);
            assert!(crate::validator_for(&schema).is_err());
        }

        #[test_case(r#"{"multipleOf": 18446744073709551616}"#, "18446744073709551617"; "multiple_of bigint")]
        #[test_case(r#"{"minimum": 99999999999999999999}"#, "99999999999999999998"; "minimum bigint")]
        #[test_case(r#"{"multipleOf": 0.1}"#, "0.15"; "multiple_of decimal")]
        fn error_messages_preserve_precision(schema_json: &str, instance_json: &str) {
            let schema = parse_json(schema_json);
            let instance = parse_json(instance_json);
            let validator = crate::validator_for(&schema).unwrap();
            assert!(validator.validate(&instance).is_err());
        }

        #[test_case(r#"{"multipleOf": 0.0000000000000001}"#; "tiny decimal")]
        #[test_case(r#"{"multipleOf": 999999999999999999999999999999}"#; "huge integer")]
        #[test_case(r#"{"minimum": -999999999999999999999999999999}"#; "huge negative")]
        #[test_case(r#"{"maximum": 999999999999999999999999999999}"#; "huge positive")]
        #[test_case(r#"{"multipleOf": 1e-100}"#; "tiny scientific")]
        #[test_case(r#"{"minimum": 1e308}"#; "near f64_max")]
        #[test_case(r#"{"maximum": -1e308}"#; "near f64_min")]
        fn schema_compilation_edge_cases(schema_json: &str) {
            let schema = parse_json(schema_json);
            assert!(crate::validator_for(&schema).is_ok());
        }
    }
}
