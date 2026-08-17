use std::sync::Arc;

use crate::{
    compiler,
    error::{no_error, ErrorIterator, ValidationError},
    evaluation::Annotations,
    keywords::CompilationResult,
    node::SchemaNode,
    options::PatternEngineOptions,
    paths::{LazyLocation, Location},
    regex::RegexEngine,
    types::JsonType,
    validator::{EvaluationResult, Validate, ValidationContext},
};
use serde_json::{Map, Value};

pub(crate) struct PatternPropertiesValidator<R> {
    patterns: Vec<(Arc<R>, SchemaNode)>,
}

impl<R: RegexEngine> Validate for PatternPropertiesValidator<R> {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        if let Value::Object(item) = instance {
            for (re, node) in &self.patterns {
                for (key, value) in item {
                    if re.is_match(key).unwrap_or(false) && !node.is_valid(value, ctx) {
                        return false;
                    }
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
        if let Value::Object(item) = instance {
            for (key, value) in item {
                for (re, node) in &self.patterns {
                    if re.is_match(key).unwrap_or(false) {
                        node.validate(value, &location.push(key), ctx)?;
                    }
                }
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
        if let Value::Object(item) = instance {
            let mut errors = Vec::new();
            for (re, node) in &self.patterns {
                for (key, value) in item {
                    if re.is_match(key).unwrap_or(false) {
                        errors.extend(node.iter_errors(value, &location.push(key.as_str()), ctx));
                    }
                }
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
        if let Value::Object(item) = instance {
            let mut matched_propnames = Vec::with_capacity(item.len());
            let mut children = Vec::new();
            for (pattern, node) in &self.patterns {
                for (key, value) in item {
                    if pattern.is_match(key).unwrap_or(false) {
                        matched_propnames.push(key.clone());
                        children.push(node.evaluate_instance(
                            value,
                            &location.push(key.as_str()),
                            ctx,
                        ));
                    }
                }
            }
            let mut result = EvaluationResult::from_children(children);
            result.annotate(Annotations::new(Value::from(matched_propnames)));
            result
        } else {
            EvaluationResult::valid_empty()
        }
    }
}

pub(crate) struct SingleValuePatternPropertiesValidator<R> {
    regex: Arc<R>,
    node: SchemaNode,
}

impl<R: RegexEngine> Validate for SingleValuePatternPropertiesValidator<R> {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        if let Value::Object(item) = instance {
            for (key, value) in item {
                if self.regex.is_match(key).unwrap_or(false) && !self.node.is_valid(value, ctx) {
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
        if let Value::Object(item) = instance {
            for (key, value) in item {
                if self.regex.is_match(key).unwrap_or(false) {
                    self.node.validate(value, &location.push(key), ctx)?;
                }
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
        if let Value::Object(item) = instance {
            let mut errors = Vec::new();
            for (key, value) in item {
                if self.regex.is_match(key).unwrap_or(false) {
                    errors.extend(
                        self.node
                            .iter_errors(value, &location.push(key.as_str()), ctx),
                    );
                }
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
        if let Value::Object(item) = instance {
            let mut matched_propnames = Vec::with_capacity(item.len());
            let mut children = Vec::new();
            for (key, value) in item {
                if self.regex.is_match(key).unwrap_or(false) {
                    matched_propnames.push(key.clone());
                    children.push(self.node.evaluate_instance(
                        value,
                        &location.push(key.as_str()),
                        ctx,
                    ));
                }
            }
            let mut result = EvaluationResult::from_children(children);
            result.annotate(Annotations::new(Value::from(matched_propnames)));
            result
        } else {
            EvaluationResult::valid_empty()
        }
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    parent: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    if matches!(
        parent.get("additionalProperties"),
        Some(Value::Bool(false) | Value::Object(_))
    ) {
        // This type of `additionalProperties` validator handles `patternProperties` logic
        return None;
    }

    let Value::Object(map) = schema else {
        return Some(Err(ValidationError::single_type_error(
            Location::new(),
            ctx.location().clone(),
            schema,
            JsonType::Object,
        )));
    };
    let ctx = ctx.new_at_location("patternProperties");
    let result = match ctx.config().pattern_options() {
        PatternEngineOptions::FancyRegex { .. } => {
            compile_pattern_entries(&ctx, map, |pctx, pattern, subschema| {
                pctx.get_or_compile_regex(pattern)
                    .map_err(|()| invalid_regex(pctx, subschema))
            })
            .map(|patterns| {
                build_validator_from_entries(patterns, |regex, node| {
                    Box::new(SingleValuePatternPropertiesValidator { regex, node })
                        as Box<dyn Validate>
                })
            })
        }
        PatternEngineOptions::Regex { .. } => {
            compile_pattern_entries(&ctx, map, |pctx, pattern, subschema| {
                pctx.get_or_compile_standard_regex(pattern)
                    .map_err(|()| invalid_regex(pctx, subschema))
            })
            .map(|patterns| {
                build_validator_from_entries(patterns, |regex, node| {
                    Box::new(SingleValuePatternPropertiesValidator { regex, node })
                        as Box<dyn Validate>
                })
            })
        }
    };
    Some(result)
}

fn invalid_regex<'a>(ctx: &compiler::Context, schema: &'a Value) -> ValidationError<'a> {
    ValidationError::format(Location::new(), ctx.location().clone(), schema, "regex")
}

/// Compile every `(pattern, subschema)` pair into `(regex, node)` tuples.
fn compile_pattern_entries<'a, R, F>(
    ctx: &compiler::Context,
    map: &'a Map<String, Value>,
    mut compile_regex: F,
) -> Result<Vec<(Arc<R>, SchemaNode)>, ValidationError<'a>>
where
    F: FnMut(&compiler::Context, &str, &'a Value) -> Result<Arc<R>, ValidationError<'a>>,
{
    let mut patterns = Vec::with_capacity(map.len());
    for (pattern, subschema) in map {
        let pctx = ctx.new_at_location(pattern.as_str());
        let regex = compile_regex(&pctx, pattern, subschema)?;
        let node = compiler::compile(&pctx, pctx.as_resource_ref(subschema))?;
        patterns.push((regex, node));
    }
    Ok(patterns)
}

/// Pick the optimal validator representation for the compiled pattern entries.
fn build_validator_from_entries<R>(
    mut entries: Vec<(Arc<R>, SchemaNode)>,
    single_factory: impl FnOnce(Arc<R>, SchemaNode) -> Box<dyn Validate>,
) -> Box<dyn Validate>
where
    R: RegexEngine + 'static,
{
    if entries.len() == 1 {
        let (regex, node) = entries.pop().expect("len checked");
        single_factory(regex, node)
    } else {
        Box::new(PatternPropertiesValidator { patterns: entries })
    }
}

#[cfg(test)]
mod tests {
    use crate::tests_util;
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!({"patternProperties": {"^f": {"type": "string"}}}), &json!({"f": 42}), "/patternProperties/^f/type")]
    #[test_case(&json!({"patternProperties": {"^f": {"type": "string"}, "^x": {"type": "string"}}}), &json!({"f": 42}), "/patternProperties/^f/type")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }
}
