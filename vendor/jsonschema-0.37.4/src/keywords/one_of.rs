use crate::{
    compiler,
    error::ValidationError,
    evaluation::ErrorDescription,
    keywords::CompilationResult,
    node::SchemaNode,
    paths::{LazyLocation, Location},
    types::JsonType,
    validator::{EvaluationResult, Validate, ValidationContext},
};
use serde_json::{Map, Value};

pub(crate) struct OneOfValidator {
    schemas: Vec<SchemaNode>,
    location: Location,
}

impl OneOfValidator {
    #[inline]
    pub(crate) fn compile<'a>(ctx: &compiler::Context, schema: &'a Value) -> CompilationResult<'a> {
        if let Value::Array(items) = schema {
            let ctx = ctx.new_at_location("oneOf");
            let mut schemas = Vec::with_capacity(items.len());
            for (idx, item) in items.iter().enumerate() {
                let ctx = ctx.new_at_location(idx);
                let node = compiler::compile(&ctx, ctx.as_resource_ref(item))?;
                schemas.push(node);
            }
            Ok(Box::new(OneOfValidator {
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

    fn get_first_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> Option<usize> {
        let mut first_valid_idx = None;
        for (idx, node) in self.schemas.iter().enumerate() {
            if node.is_valid(instance, ctx) {
                first_valid_idx = Some(idx);
                break;
            }
        }
        first_valid_idx
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn are_others_valid(&self, instance: &Value, idx: usize, ctx: &mut ValidationContext) -> bool {
        self.schemas
            .iter()
            .skip(idx + 1)
            .any(|n| n.is_valid(instance, ctx))
    }
}

impl Validate for OneOfValidator {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        let first_valid_idx = self.get_first_valid(instance, ctx);
        first_valid_idx.is_some_and(|idx| !self.are_others_valid(instance, idx, ctx))
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        let first_valid_idx = self.get_first_valid(instance, ctx);
        if let Some(idx) = first_valid_idx {
            if self.are_others_valid(instance, idx, ctx) {
                return Err(ValidationError::one_of_multiple_valid(
                    self.location.clone(),
                    location.into(),
                    instance,
                    self.schemas
                        .iter()
                        .map(|schema| schema.iter_errors(instance, location, ctx).collect())
                        .collect(),
                ));
            }
            Ok(())
        } else {
            Err(ValidationError::one_of_not_valid(
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
            let child = node.evaluate_instance(instance, location, ctx);
            if child.valid {
                let mut successes = Vec::with_capacity(total.saturating_sub(failures.len()));
                successes.push(child);
                for node in iter {
                    let next = node.evaluate_instance(instance, location, ctx);
                    if next.valid {
                        successes.push(next);
                    }
                }
                return match successes.len() {
                    1 => EvaluationResult::from(successes.remove(0)),
                    _ => EvaluationResult::Invalid {
                        errors: vec![ErrorDescription::new(
                            "oneOf",
                            "more than one subschema succeeded".to_string(),
                        )],
                        children: successes,
                        annotations: None,
                    },
                };
            }
            failures.push(child);
        }
        EvaluationResult::Invalid {
            errors: Vec::new(),
            children: failures,
            annotations: None,
        }
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    Some(OneOfValidator::compile(ctx, schema))
}

#[cfg(test)]
mod tests {
    use crate::tests_util;
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!({"oneOf": [{"type": "string"}]}), &json!(0), "/oneOf")]
    #[test_case(&json!({"oneOf": [{"type": "string"}, {"maxLength": 3}]}), &json!(""), "/oneOf")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }
}
