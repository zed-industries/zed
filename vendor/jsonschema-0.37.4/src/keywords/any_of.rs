use crate::{
    compiler,
    error::{error, no_error, ErrorIterator, ValidationError},
    node::SchemaNode,
    paths::{LazyLocation, Location},
    types::JsonType,
    validator::{EvaluationResult, Validate, ValidationContext},
};
use serde_json::{Map, Value};

use super::CompilationResult;

pub(crate) struct AnyOfValidator {
    schemas: Vec<SchemaNode>,
    location: Location,
}

impl AnyOfValidator {
    #[inline]
    pub(crate) fn compile<'a>(ctx: &compiler::Context, schema: &'a Value) -> CompilationResult<'a> {
        if let Value::Array(items) = schema {
            let ctx = ctx.new_at_location("anyOf");
            let mut schemas = Vec::with_capacity(items.len());
            for (idx, item) in items.iter().enumerate() {
                let ctx = ctx.new_at_location(idx);
                let node = compiler::compile(&ctx, ctx.as_resource_ref(item))?;
                schemas.push(node);
            }
            Ok(Box::new(AnyOfValidator {
                schemas,
                location: ctx.location().clone(),
            }))
        } else {
            Err(ValidationError::single_type_error(
                Location::new(),
                ctx.location().clone(),
                schema,
                JsonType::Array,
            ))
        }
    }
}

impl Validate for AnyOfValidator {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        self.schemas.iter().any(|s| s.is_valid(instance, ctx))
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if self.is_valid(instance, ctx) {
            Ok(())
        } else {
            Err(ValidationError::any_of(
                self.location.clone(),
                location.into(),
                instance,
                self.schemas
                    .iter()
                    .map(|schema| schema.iter_errors(instance, location, ctx).collect())
                    .collect(),
            ))
        }
    }

    fn iter_errors<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> ErrorIterator<'i> {
        if self.is_valid(instance, ctx) {
            no_error()
        } else {
            error(ValidationError::any_of(
                self.location.clone(),
                location.into(),
                instance,
                self.schemas
                    .iter()
                    .map(|schema| schema.iter_errors(instance, location, ctx).collect())
                    .collect(),
            ))
        }
    }

    fn evaluate(
        &self,
        instance: &Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> EvaluationResult {
        let total = self.schemas.len();
        let mut failures = Vec::with_capacity(total);
        let mut iter = self.schemas.iter();
        while let Some(node) = iter.next() {
            let result = node.evaluate_instance(instance, location, ctx);
            if result.valid {
                let remaining = total.saturating_sub(failures.len() + 1);
                let mut successes = Vec::with_capacity(remaining + 1);
                successes.push(result);
                for node in iter {
                    let tail = node.evaluate_instance(instance, location, ctx);
                    if tail.valid {
                        successes.push(tail);
                    }
                }
                return EvaluationResult::from_children(successes);
            }
            failures.push(result);
        }
        EvaluationResult::from_children(failures)
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    Some(AnyOfValidator::compile(ctx, schema))
}

#[cfg(test)]
mod tests {
    use crate::tests_util;
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!({"anyOf": [{"type": "string"}]}), &json!(1), "/anyOf")]
    #[test_case(&json!({"anyOf": [{"type": "integer"}, {"type": "string"}]}), &json!({}), "/anyOf")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }
}
