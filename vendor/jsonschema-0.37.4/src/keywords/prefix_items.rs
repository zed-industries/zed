use crate::{
    compiler,
    error::{no_error, ErrorIterator, ValidationError},
    evaluation::Annotations,
    node::SchemaNode,
    paths::{LazyLocation, Location},
    types::JsonType,
    validator::{EvaluationResult, Validate, ValidationContext},
};
use serde_json::{Map, Value};

use super::CompilationResult;

pub(crate) struct PrefixItemsValidator {
    schemas: Vec<SchemaNode>,
}

impl PrefixItemsValidator {
    #[inline]
    pub(crate) fn compile<'a>(
        ctx: &compiler::Context,
        items: &'a [Value],
    ) -> CompilationResult<'a> {
        let ctx = ctx.new_at_location("prefixItems");
        let mut schemas = Vec::with_capacity(items.len());
        for (idx, item) in items.iter().enumerate() {
            let ctx = ctx.new_at_location(idx);
            let validators = compiler::compile(&ctx, ctx.as_resource_ref(item))?;
            schemas.push(validators);
        }
        Ok(Box::new(PrefixItemsValidator { schemas }))
    }
}

impl Validate for PrefixItemsValidator {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        if let Value::Array(items) = instance {
            for (schema, item) in self.schemas.iter().zip(items.iter()) {
                if !schema.is_valid(item, ctx) {
                    return false;
                }
            }
            true
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
        if let Value::Array(items) = instance {
            for (idx, (schema, item)) in self.schemas.iter().zip(items.iter()).enumerate() {
                schema.validate(item, &location.push(idx), ctx)?;
            }
        }
        Ok(())
    }

    fn iter_errors<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> ErrorIterator<'i> {
        if let Value::Array(items) = instance {
            let mut errors = Vec::new();
            for (idx, (schema, item)) in self.schemas.iter().zip(items.iter()).enumerate() {
                errors.extend(schema.iter_errors(item, &location.push(idx), ctx));
            }
            ErrorIterator::from_iterator(errors.into_iter())
        } else {
            no_error()
        }
    }

    fn evaluate(
        &self,
        instance: &Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> EvaluationResult {
        if let Value::Array(items) = instance {
            if !items.is_empty() {
                let mut children = Vec::with_capacity(self.schemas.len().min(items.len()));
                let mut max_index_applied = 0usize;
                for (idx, (schema_node, item)) in self.schemas.iter().zip(items.iter()).enumerate()
                {
                    children.push(schema_node.evaluate_instance(item, &location.push(idx), ctx));
                    max_index_applied = idx;
                }
                let annotation = if children.len() == items.len() {
                    Value::Bool(true)
                } else {
                    Value::from(max_index_applied)
                };
                let mut result = EvaluationResult::from_children(children);
                result.annotate(Annotations::new(annotation));
                return result;
            }
        }
        EvaluationResult::valid_empty()
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    if let Value::Array(items) = schema {
        Some(PrefixItemsValidator::compile(ctx, items))
    } else {
        Some(Err(ValidationError::single_type_error(
            Location::new(),
            ctx.location().clone(),
            schema,
            JsonType::Array,
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_util;
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!({"$schema": "https://json-schema.org/draft/2020-12/schema", "prefixItems": [{"type": "integer"}, {"maximum": 5}]}), &json!(["string"]), "/prefixItems/0/type")]
    #[test_case(&json!({"$schema": "https://json-schema.org/draft/2020-12/schema", "prefixItems": [{"type": "integer"}, {"maximum": 5}]}), &json!([42, 42]), "/prefixItems/1/maximum")]
    #[test_case(&json!({"$schema": "https://json-schema.org/draft/2020-12/schema", "prefixItems": [{"type": "integer"}, {"maximum": 5}], "items": {"type": "boolean"}}), &json!([42, 1, 42]), "/items/type")]
    #[test_case(&json!({"$schema": "https://json-schema.org/draft/2020-12/schema", "prefixItems": [{"type": "integer"}, {"maximum": 5}], "items": {"type": "boolean"}}), &json!([42, 42, true]), "/prefixItems/1/maximum")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }

    #[test]
    fn evaluation_outputs_cover_prefix_items() {
        let schema = json!({
            "type": "object",
            "properties": {"name": {"type": "string"}, "age": {"type": "number", "minimum": 0}},
            "required": ["name"]
        });
        let validator = crate::validator_for(&schema).expect("schema compiles");
        let evaluation = validator.evaluate(&json!({"name": "Alice", "age": 1}));

        assert_eq!(
            serde_json::to_value(evaluation.list()).unwrap(),
            json!({
                "valid": true,
                "details": [
                    {"evaluationPath": "", "instanceLocation": "", "schemaLocation": "", "valid": true},
                    {
                        "valid": true,
                        "evaluationPath": "/properties",
                        "instanceLocation": "",
                        "schemaLocation": "/properties",
                        "annotations": ["age", "name"]
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/properties/age",
                        "instanceLocation": "/age",
                        "schemaLocation": "/properties/age"
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/properties/age/minimum",
                        "instanceLocation": "/age",
                        "schemaLocation": "/properties/age/minimum"
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/properties/age/type",
                        "instanceLocation": "/age",
                        "schemaLocation": "/properties/age/type"
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/properties/name",
                        "instanceLocation": "/name",
                        "schemaLocation": "/properties/name"
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/properties/name/type",
                        "instanceLocation": "/name",
                        "schemaLocation": "/properties/name/type"
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/required",
                        "instanceLocation": "",
                        "schemaLocation": "/required"
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/type",
                        "instanceLocation": "",
                        "schemaLocation": "/type"
                    }
                ]
            })
        );

        assert_eq!(
            serde_json::to_value(evaluation.hierarchical()).unwrap(),
            json!({
                "valid": true,
                "evaluationPath": "",
                "instanceLocation": "",
                "schemaLocation": "",
                "details": [
                    {
                        "valid": true,
                        "evaluationPath": "/properties",
                        "instanceLocation": "",
                        "schemaLocation": "/properties",
                        "annotations": ["age", "name"],
                        "details": [
                            {
                                "valid": true,
                                "evaluationPath": "/properties/age",
                                "instanceLocation": "/age",
                                "schemaLocation": "/properties/age",
                                "details": [
                                    {
                                        "valid": true,
                                        "evaluationPath": "/properties/age/minimum",
                                        "instanceLocation": "/age",
                                        "schemaLocation": "/properties/age/minimum"
                                    },
                                    {
                                        "valid": true,
                                        "evaluationPath": "/properties/age/type",
                                        "instanceLocation": "/age",
                                        "schemaLocation": "/properties/age/type"
                                    }
                                ]
                            },
                            {
                                "valid": true,
                                "evaluationPath": "/properties/name",
                                "instanceLocation": "/name",
                                "schemaLocation": "/properties/name",
                                "details": [
                                    {
                                        "valid": true,
                                        "evaluationPath": "/properties/name/type",
                                        "instanceLocation": "/name",
                                        "schemaLocation": "/properties/name/type"
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/required",
                        "instanceLocation": "",
                        "schemaLocation": "/required"
                    },
                    {
                        "valid": true,
                        "evaluationPath": "/type",
                        "instanceLocation": "",
                        "schemaLocation": "/type"
                    }
                ]
            })
        );
    }
}
