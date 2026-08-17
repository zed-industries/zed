use crate::{
    compiler,
    error::ValidationError,
    ext::numeric,
    keywords::CompilationResult,
    paths::{LazyLocation, Location},
    thread::ThreadBound,
    types::JsonType,
    validator::{Validate, ValidationContext},
};
use num_cmp::NumCmp;
use serde_json::{Map, Value};

macro_rules! define_numeric_keywords {
    ($($struct_name:ident => $fn_name:path => $error_fn_name:ident),* $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub(crate) struct $struct_name<T> {
                pub(super) limit: T,
                limit_val: Value,
                location: Location,
            }

            impl<T> From<(T, Value, Location)> for $struct_name<T> {
                fn from((limit, limit_val, location): (T, Value, Location)) -> Self {
                    Self { limit, limit_val, location }
                }
            }

            impl<T> Validate for $struct_name<T>
            where
                T: Copy + ThreadBound + num_traits::ToPrimitive,
                u64: NumCmp<T>,
                i64: NumCmp<T>,
                f64: NumCmp<T>,
            {
                fn validate<'i>(
                    &self,
                    instance: &'i Value,
                    location: &LazyLocation,
                    ctx: &mut ValidationContext,
                ) -> Result<(), ValidationError<'i>> {
                    if self.is_valid(instance, ctx) {
                        Ok(())
                    } else {
                        Err(ValidationError::$error_fn_name(
                            self.location.clone(),
                            location.into(),
                            instance,
                            self.limit_val.clone(),
                        ))
                    }
                }

                fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
                    if let Value::Number(item) = instance {
                        $fn_name(item, self.limit)
                    } else {
                        true
                    }
                }
            }
        )*
    };
}

define_numeric_keywords!(
    Minimum => numeric::ge => minimum,
    Maximum => numeric::le => maximum,
    ExclusiveMinimum => numeric::gt => exclusive_minimum,
    ExclusiveMaximum => numeric::lt => exclusive_maximum,
);

#[cfg(feature = "arbitrary-precision")]
pub(crate) mod bigint_validators {
    use super::{
        numeric, LazyLocation, Location, Validate, ValidationContext, ValidationError, Value,
    };
    use crate::ext::numeric::bignum::{
        f64_ge_bigfrac, f64_ge_bigint, f64_gt_bigfrac, f64_gt_bigint, f64_le_bigfrac,
        f64_le_bigint, f64_lt_bigfrac, f64_lt_bigint, i64_ge_bigfrac, i64_ge_bigint,
        i64_gt_bigfrac, i64_gt_bigint, i64_le_bigfrac, i64_le_bigint, i64_lt_bigfrac,
        i64_lt_bigint, try_parse_bigfraction, u64_ge_bigfrac, u64_ge_bigint, u64_gt_bigfrac,
        u64_gt_bigint, u64_le_bigfrac, u64_le_bigint, u64_lt_bigfrac, u64_lt_bigint,
    };
    use num_bigint::BigInt;

    macro_rules! define_bigint_validator {
        ($struct_name:ident, $u64_cmp:path, $i64_cmp:path, $f64_cmp:path, $bigint_op:tt, $error_fn:ident, $infinity_fn:expr) => {
            #[derive(Debug, Clone, PartialEq)]
            pub(crate) struct $struct_name {
                pub(super) limit: BigInt,
                pub(super) limit_val: Value,
                pub(super) location: Location,
            }

            impl $struct_name {
                pub(crate) fn new(limit: BigInt, limit_val: Value, location: Location) -> Self {
                    Self { limit, limit_val, location }
                }
            }

            impl Validate for $struct_name {
                fn validate<'i>(
                    &self,
                    instance: &'i Value,
                    location: &LazyLocation,
                    ctx: &mut ValidationContext,
                ) -> Result<(), ValidationError<'i>> {
                    if self.is_valid(instance, ctx) {
                        Ok(())
                    } else {
                        Err(ValidationError::$error_fn(
                            self.location.clone(),
                            location.into(),
                            instance,
                            self.limit_val.clone(),
                        ))
                    }
                }

                fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
                    use fraction::BigFraction;
                    if let Value::Number(item) = instance {
                        // Try to parse instance as BigInt first
                        if let Some(instance_bigint) = numeric::bignum::try_parse_bigint(item) {
                            // Both are BigInt - direct comparison
                            instance_bigint $bigint_op self.limit
                        } else if let Some(v) = item.as_u64() {
                            $u64_cmp(v, &self.limit)
                        } else if let Some(v) = item.as_i64() {
                            $i64_cmp(v, &self.limit)
                        } else if let Some(v) = item.as_f64() {
                            $f64_cmp(v, &self.limit)
                        } else {
                            // Number doesn't fit in f64 (e.g., 1e1000)
                            // Since limit is BigInt, we need to compare with BigFraction
                            if let Some(instance_bigfrac) = numeric::bignum::try_parse_bigfraction(item) {
                                // Convert BigInt limit to BigFraction for comparison
                                // Use clone to avoid truncation through i128
                                let limit_frac = BigFraction::from(self.limit.clone());
                                instance_bigfrac $bigint_op limit_frac
                            } else {
                                // Can't parse as BigFraction - extremely large scientific notation
                                // (e.g., "1e10000" which exceeds BigFraction's parsing capability)
                                // These behave like positive/negative infinity for comparison purposes
                                let is_negative = item.as_str().starts_with('-');
                                $infinity_fn(is_negative)
                            }
                        }
                    } else {
                        true
                    }
                }
            }
        };
    }

    define_bigint_validator!(
        BigIntMinimum,
        u64_ge_bigint,
        i64_ge_bigint,
        f64_ge_bigint,
        >=,
        minimum,
        |is_negative: bool| !is_negative  // For >= and >, positive infinity passes
    );
    define_bigint_validator!(
        BigIntMaximum,
        u64_le_bigint,
        i64_le_bigint,
        f64_le_bigint,
        <=,
        maximum,
        |is_negative: bool| is_negative  // For <= and <, negative infinity passes
    );
    define_bigint_validator!(
        BigIntExclusiveMinimum,
        u64_gt_bigint,
        i64_gt_bigint,
        f64_gt_bigint,
        >,
        exclusive_minimum,
        |is_negative: bool| !is_negative  // For >= and >, positive infinity passes
    );
    define_bigint_validator!(
        BigIntExclusiveMaximum,
        u64_lt_bigint,
        i64_lt_bigint,
        f64_lt_bigint,
        <,
        exclusive_maximum,
        |is_negative: bool| is_negative  // For <= and <, negative infinity passes
    );

    // BigFraction validators for arbitrary precision floats
    use fraction::BigFraction;

    macro_rules! define_bigfrac_validator {
        ($struct_name:ident, $u64_cmp:path, $i64_cmp:path, $f64_cmp:path, $bigfrac_op:tt, $error_fn:ident) => {
            #[derive(Debug, Clone, PartialEq)]
            pub(crate) struct $struct_name {
                pub(super) limit: BigFraction,
                pub(super) limit_val: Value,
                pub(super) location: Location,
            }

            impl $struct_name {
                pub(crate) fn new(limit: BigFraction, limit_val: Value, location: Location) -> Self {
                    Self { limit, limit_val, location }
                }
            }

            impl Validate for $struct_name {
                fn validate<'i>(
                    &self,
                    instance: &'i Value,
                    location: &LazyLocation,
                    ctx: &mut ValidationContext,
                ) -> Result<(), ValidationError<'i>> {
                    if self.is_valid(instance, ctx) {
                        Ok(())
                    } else {
                        Err(ValidationError::$error_fn(
                            self.location.clone(),
                            location.into(),
                            instance,
                            self.limit_val.clone(),
                        ))
                    }
                }

                fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
                    if let Value::Number(item) = instance {
                        // Try to parse instance as BigFraction for exact precision
                        if let Some(instance_bigfrac) = try_parse_bigfraction(item) {
                            // Both are BigFraction - direct comparison
                            instance_bigfrac $bigfrac_op self.limit
                        } else if let Some(v) = item.as_u64() {
                            $u64_cmp(v, &self.limit)
                        } else if let Some(v) = item.as_i64() {
                            $i64_cmp(v, &self.limit)
                        } else if let Some(v) = item.as_f64() {
                            // Scientific notation or other f64-representable numbers
                            $f64_cmp(v, &self.limit)
                        } else {
                            // Extreme scientific notation beyond f64 range (e.g., 1e309, 1e400)
                            // These are not supported for comparison - treat as always valid
                            // since we can't reliably compare them
                            true
                        }
                    } else {
                        true
                    }
                }
            }
        };
    }

    define_bigfrac_validator!(
        BigFracMinimum,
        u64_ge_bigfrac,
        i64_ge_bigfrac,
        f64_ge_bigfrac,
        >=,
        minimum
    );
    define_bigfrac_validator!(
        BigFracMaximum,
        u64_le_bigfrac,
        i64_le_bigfrac,
        f64_le_bigfrac,
        <=,
        maximum
    );
    define_bigfrac_validator!(
        BigFracExclusiveMinimum,
        u64_gt_bigfrac,
        i64_gt_bigfrac,
        f64_gt_bigfrac,
        >,
        exclusive_minimum
    );
    define_bigfrac_validator!(
        BigFracExclusiveMaximum,
        u64_lt_bigfrac,
        i64_lt_bigfrac,
        f64_lt_bigfrac,
        <,
        exclusive_maximum
    );
}

#[inline]
fn create_validator<T, V>(
    ctx: &compiler::Context,
    keyword: &str,
    limit: T,
    schema: &Value,
) -> CompilationResult<'static>
where
    V: From<(T, Value, Location)> + Validate + 'static,
{
    let location = ctx.location().join(keyword);
    Ok(Box::new(V::from((limit, schema.clone(), location))))
}

fn number_type_error<'a>(ctx: &compiler::Context, schema: &'a Value) -> CompilationResult<'a> {
    Err(ValidationError::single_type_error(
        Location::new(),
        ctx.location().clone(),
        schema,
        JsonType::Number,
    ))
}

macro_rules! create_numeric_validator {
    ($validator_type:ident, $ctx:expr, $keyword:expr, $limit:expr, $schema:expr) => {
        if let Some(limit) = $limit.as_u64() {
            Some(create_validator::<_, $validator_type<u64>>(
                $ctx, $keyword, limit, $schema,
            ))
        } else if let Some(limit) = $limit.as_i64() {
            Some(create_validator::<_, $validator_type<i64>>(
                $ctx, $keyword, limit, $schema,
            ))
        } else {
            #[cfg(feature = "arbitrary-precision")]
            {
                if let Some(result) = create_bigint_validator($ctx, $keyword, $limit, $schema) {
                    return Some(result);
                }
            }
            // Handle numbers that don't fit in f64 (e.g., 1e10000)
            // These are extremely large scientific notation numbers
            if let Some(limit_f64) = $limit.as_f64() {
                Some(create_validator::<_, $validator_type<f64>>(
                    $ctx, $keyword, limit_f64, $schema,
                ))
            } else {
                #[cfg(feature = "arbitrary-precision")]
                {
                    // Number is too large for f64 and couldn't be parsed as BigInt/BigFraction
                    // (e.g., "1e10000" with huge exponent exceeding f64 range)
                    // Use actual infinity as the comparison limit based on sign
                    let is_negative = $limit.as_str().starts_with('-');
                    let infinity_limit = if is_negative {
                        f64::NEG_INFINITY
                    } else {
                        f64::INFINITY
                    };
                    Some(create_validator::<_, $validator_type<f64>>(
                        $ctx,
                        $keyword,
                        infinity_limit,
                        $schema,
                    ))
                }
                #[cfg(not(feature = "arbitrary-precision"))]
                {
                    unreachable!("as_f64() always returns Some without arbitrary-precision")
                }
            }
        }
    };
}

#[cfg(feature = "arbitrary-precision")]
fn create_bigint_validator(
    ctx: &compiler::Context,
    keyword: &str,
    limit: &serde_json::Number,
    schema: &Value,
) -> Option<CompilationResult<'static>> {
    use bigint_validators::{
        BigFracExclusiveMaximum, BigFracExclusiveMinimum, BigFracMaximum, BigFracMinimum,
        BigIntExclusiveMaximum, BigIntExclusiveMinimum, BigIntMaximum, BigIntMinimum,
    };

    // Try BigInt first for large integers
    if let Some(bigint_limit) = numeric::bignum::try_parse_bigint(limit) {
        let location = ctx.location().join(keyword);
        let validator: Box<dyn Validate> = match keyword {
            "minimum" => Box::new(BigIntMinimum::new(bigint_limit, schema.clone(), location)),
            "maximum" => Box::new(BigIntMaximum::new(bigint_limit, schema.clone(), location)),
            "exclusiveMinimum" => Box::new(BigIntExclusiveMinimum::new(
                bigint_limit,
                schema.clone(),
                location,
            )),
            "exclusiveMaximum" => Box::new(BigIntExclusiveMaximum::new(
                bigint_limit,
                schema.clone(),
                location,
            )),
            _ => return None,
        };
        return Some(Ok(validator));
    }

    // If not a BigInt, try BigFraction for exact decimal precision
    if let Some(bigfrac_limit) = numeric::bignum::try_parse_bigfraction(limit) {
        let location = ctx.location().join(keyword);
        let validator: Box<dyn Validate> = match keyword {
            "minimum" => Box::new(BigFracMinimum::new(bigfrac_limit, schema.clone(), location)),
            "maximum" => Box::new(BigFracMaximum::new(bigfrac_limit, schema.clone(), location)),
            "exclusiveMinimum" => Box::new(BigFracExclusiveMinimum::new(
                bigfrac_limit,
                schema.clone(),
                location,
            )),
            "exclusiveMaximum" => Box::new(BigFracExclusiveMaximum::new(
                bigfrac_limit,
                schema.clone(),
                location,
            )),
            _ => return None,
        };
        return Some(Ok(validator));
    }

    None
}

#[inline]
pub(crate) fn compile_minimum<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    match schema {
        Value::Number(limit) => create_numeric_validator!(Minimum, ctx, "minimum", limit, schema),
        _ => Some(number_type_error(ctx, schema)),
    }
}

#[inline]
pub(crate) fn compile_maximum<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    match schema {
        Value::Number(limit) => create_numeric_validator!(Maximum, ctx, "maximum", limit, schema),
        _ => Some(number_type_error(ctx, schema)),
    }
}

#[inline]
pub(crate) fn compile_exclusive_minimum<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    match schema {
        Value::Number(limit) => {
            create_numeric_validator!(ExclusiveMinimum, ctx, "exclusiveMinimum", limit, schema)
        }
        _ => Some(number_type_error(ctx, schema)),
    }
}

#[inline]
pub(crate) fn compile_exclusive_maximum<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    match schema {
        Value::Number(limit) => {
            create_numeric_validator!(ExclusiveMaximum, ctx, "exclusiveMaximum", limit, schema)
        }
        _ => Some(number_type_error(ctx, schema)),
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_util;
    use serde_json::{json, Value};
    use test_case::test_case;

    #[cfg(feature = "arbitrary-precision")]
    fn parse_json(json: &str) -> Value {
        serde_json::from_str(json).unwrap()
    }

    #[test_case(&json!({"minimum": 1_u64 << 54}), &json!((1_u64 << 54) - 1))]
    #[test_case(&json!({"minimum": 1_i64 << 54}), &json!((1_i64 << 54) - 1))]
    #[test_case(&json!({"maximum": 1_u64 << 54}), &json!((1_u64 << 54) + 1))]
    #[test_case(&json!({"maximum": 1_i64 << 54}), &json!((1_i64 << 54) + 1))]
    #[test_case(&json!({"exclusiveMinimum": 1_u64 << 54}), &json!(1_u64 << 54))]
    #[test_case(&json!({"exclusiveMinimum": 1_i64 << 54}), &json!(1_i64 << 54))]
    #[test_case(&json!({"exclusiveMinimum": 1_u64 << 54}), &json!((1_u64 << 54) - 1))]
    #[test_case(&json!({"exclusiveMinimum": 1_i64 << 54}), &json!((1_i64 << 54) - 1))]
    #[test_case(&json!({"exclusiveMaximum": 1_u64 << 54}), &json!(1_u64 << 54))]
    #[test_case(&json!({"exclusiveMaximum": 1_i64 << 54}), &json!(1_i64 << 54))]
    #[test_case(&json!({"exclusiveMaximum": 1_u64 << 54}), &json!((1_u64 << 54) + 1))]
    #[test_case(&json!({"exclusiveMaximum": 1_i64 << 54}), &json!((1_i64 << 54) + 1))]
    fn is_not_valid(schema: &Value, instance: &Value) {
        tests_util::is_not_valid(schema, instance);
    }

    #[test_case(&json!({"minimum": 5}), &json!(1), "/minimum")]
    #[test_case(&json!({"minimum": 6}), &json!(1), "/minimum")]
    #[test_case(&json!({"minimum": 7}), &json!(1), "/minimum")]
    #[test_case(&json!({"maximum": 5}), &json!(10), "/maximum")]
    #[test_case(&json!({"maximum": 6}), &json!(10), "/maximum")]
    #[test_case(&json!({"maximum": 7}), &json!(10), "/maximum")]
    #[test_case(&json!({"exclusiveMinimum": 5}), &json!(1), "/exclusiveMinimum")]
    #[test_case(&json!({"exclusiveMinimum": 6}), &json!(1), "/exclusiveMinimum")]
    #[test_case(&json!({"exclusiveMinimum": 7}), &json!(1), "/exclusiveMinimum")]
    #[test_case(&json!({"exclusiveMaximum": 5}), &json!(7), "/exclusiveMaximum")]
    #[test_case(&json!({"exclusiveMaximum": -1}), &json!(7), "/exclusiveMaximum")]
    #[test_case(&json!({"exclusiveMaximum": -1.0}), &json!(7), "/exclusiveMaximum")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }

    // Tests for arbitrary-precision feature - valid cases
    #[cfg(feature = "arbitrary-precision")]
    #[test_case(r#"{"minimum": 18446744073709551616}"#, r"18446744073709551617"; "minimum valid above u64_max")]
    #[test_case(r#"{"minimum": -9223372036854775809}"#, r"-9223372036854775808"; "minimum valid below i64_min")]
    #[test_case(r#"{"maximum": 18446744073709551616}"#, r"18446744073709551615"; "maximum valid below limit")]
    #[test_case(r#"{"exclusiveMinimum": 18446744073709551616}"#, r"18446744073709551617"; "exclusive_minimum valid")]
    #[test_case(r#"{"exclusiveMaximum": 18446744073709551616}"#, r"18446744073709551615"; "exclusive_maximum valid")]
    #[test_case(r#"{"minimum": 18446744073709551616}"#, "1e20"; "scientific notation above bigint minimum")]
    #[test_case(r#"{"maximum": 18446744073709551616}"#, "1e15"; "scientific notation below bigint maximum")]
    #[test_case(r#"{"minimum": 0}"#, "1e1000"; "minimum with huge positive number")]
    #[test_case(r#"{"minimum": -1e100}"#, "1e1000"; "minimum with huge positive above limit")]
    #[test_case(r#"{"exclusiveMinimum": 0}"#, "1e1000"; "exclusive minimum with huge positive")]
    #[test_case(r#"{"minimum": 18446744073709551616}"#, "1e20"; "bigint minimum with huge instance valid")]
    #[test_case(r#"{"maximum": 18446744073709551616}"#, "1e15"; "bigint maximum with huge instance valid")]
    #[test_case(r#"{"exclusiveMinimum": 18446744073709551616}"#, "1e25"; "bigint exclusive minimum with huge instance valid")]
    #[test_case(r#"{"minimum": 0}"#, "1e10000"; "infinity positive above minimum")]
    #[test_case(r#"{"maximum": 0}"#, "-1e10000"; "infinity negative below maximum")]
    #[test_case(r#"{"minimum": -100}"#, "1e10000"; "infinity positive above negative minimum")]
    #[test_case(r#"{"maximum": 100}"#, "-1e10000"; "infinity negative below positive maximum")]
    #[test_case(r#"{"minimum": 18446744073709551616}"#, "1e10000"; "infinity above bigint minimum")]
    #[test_case(r#"{"maximum": 18446744073709551616}"#, "-1e10000"; "infinity below bigint maximum")]
    fn is_valid_arbitrary_precision(schema_json: &str, instance_json: &str) {
        let schema = parse_json(schema_json);
        let instance = parse_json(instance_json);
        tests_util::is_valid(&schema, &instance);
    }

    // Tests for arbitrary-precision feature - invalid cases
    #[cfg(feature = "arbitrary-precision")]
    #[test_case(r#"{"minimum": 18446744073709551616}"#, r"18446744073709551615"; "minimum invalid at u64_max")]
    #[test_case(r#"{"maximum": 18446744073709551616}"#, r"18446744073709551617"; "maximum invalid above limit")]
    #[test_case(r#"{"exclusiveMinimum": 18446744073709551616}"#, r"18446744073709551616"; "exclusive_minimum invalid at boundary")]
    #[test_case(r#"{"exclusiveMaximum": 18446744073709551616}"#, r"18446744073709551617"; "exclusive_maximum invalid above limit")]
    #[test_case(r#"{"minimum": 18446744073709551616}"#, "1e10"; "scientific notation below bigint minimum")]
    #[test_case(r#"{"maximum": 18446744073709551616}"#, "1e25"; "scientific notation above bigint maximum")]
    #[test_case(r#"{"maximum": 1e100}"#, "1e1000"; "maximum with huge number above limit")]
    #[test_case(r#"{"exclusiveMaximum": 1e100}"#, "1e1000"; "exclusive maximum with huge number")]
    #[test_case(r#"{"minimum": 1e100}"#, "-1e1000"; "minimum with huge negative below limit")]
    #[test_case(r#"{"minimum": 18446744073709551616}"#, "1e10"; "bigint minimum with huge instance invalid")]
    #[test_case(r#"{"maximum": 18446744073709551616}"#, "1e25"; "bigint maximum with huge instance invalid")]
    #[test_case(r#"{"exclusiveMaximum": 18446744073709551616}"#, "1e25"; "bigint exclusive maximum with huge instance invalid")]
    #[test_case(r#"{"maximum": 0}"#, "1e10000"; "infinity positive above maximum")]
    #[test_case(r#"{"minimum": 0}"#, "-1e10000"; "infinity negative below minimum")]
    #[test_case(r#"{"exclusiveMaximum": 100}"#, "1e10000"; "infinity positive above exclusive maximum")]
    #[test_case(r#"{"exclusiveMinimum": -100}"#, "-1e10000"; "infinity negative below exclusive minimum")]
    #[test_case(r#"{"maximum": 18446744073709551616}"#, "1e10000"; "infinity above bigint maximum")]
    #[test_case(r#"{"minimum": 18446744073709551616}"#, "-1e10000"; "infinity below bigint minimum")]
    fn is_not_valid_arbitrary_precision(schema_json: &str, instance_json: &str) {
        let schema = parse_json(schema_json);
        let instance = parse_json(instance_json);
        tests_util::is_not_valid(&schema, &instance);
    }

    // Regression test for BigInt limits exceeding i128 range with fractional instances
    #[cfg(feature = "arbitrary-precision")]
    #[test]
    fn bigint_limit_exceeds_i128_with_fraction() {
        // 10^50 is way beyond i128::MAX (~1.7e38)
        let limit = "1".to_string() + &"0".repeat(50);
        let schema_min = parse_json(&format!(r#"{{"minimum": {limit}}}"#));

        // Instance slightly below limit (10^50 - 1), should be INVALID
        let instance_below = "9".to_string() + &"9".repeat(49);
        let instance = parse_json(&instance_below);
        tests_util::is_not_valid(&schema_min, &instance);

        // Instance equal to limit, should be VALID
        let instance = parse_json(&limit);
        tests_util::is_valid(&schema_min, &instance);

        // Test maximum as well
        let schema_max = parse_json(&format!(r#"{{"maximum": {limit}}}"#));

        // Instance slightly above limit (10^50 + 1), should be INVALID
        let instance_above = "1".to_string() + &"0".repeat(50) + "1";
        let instance = parse_json(&instance_above);
        tests_util::is_not_valid(&schema_max, &instance);
    }

    // Test that schema compilation doesn't panic with extremely large scientific notation limits
    // These numbers (e.g., 1e10000) exceed f64 range and are treated as infinity for comparison
    #[cfg(feature = "arbitrary-precision")]
    #[test_case(r#"{"minimum": 1e10000}"#, "999999999999"; "huge positive scientific notation minimum")]
    #[test_case(r#"{"maximum": 1e10000}"#, "999999999999"; "huge positive scientific notation maximum")]
    #[test_case(r#"{"minimum": -1e10000}"#, "999999999999"; "huge negative scientific notation minimum")]
    #[test_case(r#"{"maximum": -1e10000}"#, "999999999999"; "huge negative scientific notation maximum")]
    #[test_case(r#"{"exclusiveMinimum": 1e10000}"#, "999999999999"; "huge positive scientific notation exclusive minimum")]
    #[test_case(r#"{"exclusiveMaximum": 1e10000}"#, "999999999999"; "huge positive scientific notation exclusive maximum")]
    #[test_case(r#"{"exclusiveMinimum": -1e10000}"#, "999999999999"; "huge negative scientific notation exclusive minimum")]
    #[test_case(r#"{"exclusiveMaximum": -1e10000}"#, "999999999999"; "huge negative scientific notation exclusive maximum")]
    fn extreme_scientific_notation_schema_limits_no_panic(schema_json: &str, instance_json: &str) {
        // This test ensures compilation doesn't panic when schema limits exceed f64 range
        let schema = parse_json(schema_json);
        let instance = parse_json(instance_json);
        // The actual validation result doesn't matter - we're just ensuring no panic
        let compiled = crate::validator_for(&schema);
        assert!(compiled.is_ok(), "Schema compilation should not panic");

        // Also verify validation works
        let validator = compiled.unwrap();
        let _ = validator.is_valid(&instance);
    }

    // Test validation behavior with extremely large scientific notation schema limits
    #[cfg(feature = "arbitrary-precision")]
    #[test_case(r#"{"minimum": 1e10000}"#, "999999999999", false; "normal number vs huge positive minimum")]
    #[test_case(r#"{"maximum": 1e10000}"#, "999999999999", true; "normal number vs huge positive maximum")]
    #[test_case(r#"{"minimum": -1e10000}"#, "999999999999", true; "normal number vs huge negative minimum")]
    #[test_case(r#"{"maximum": -1e10000}"#, "999999999999", false; "normal number vs huge negative maximum")]
    fn extreme_scientific_notation_schema_limits_validation(
        schema_json: &str,
        instance_json: &str,
        expected_valid: bool,
    ) {
        let schema = parse_json(schema_json);
        let instance = parse_json(instance_json);
        if expected_valid {
            tests_util::is_valid(&schema, &instance);
        } else {
            tests_util::is_not_valid(&schema, &instance);
        }
    }
}
