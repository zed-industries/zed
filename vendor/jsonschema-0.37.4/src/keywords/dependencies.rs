use crate::{
    compiler,
    error::{no_error, ErrorIterator, ValidationError},
    keywords::{required, unique_items, CompilationResult},
    node::SchemaNode,
    paths::{LazyLocation, Location},
    types::JsonType,
    validator::{EvaluationResult, Validate, ValidationContext},
};
use serde_json::{Map, Value};

pub(crate) struct DependenciesValidator {
    dependencies: Vec<(String, SchemaNode)>,
}

impl DependenciesValidator {
    #[inline]
    pub(crate) fn compile<'a>(ctx: &compiler::Context, schema: &'a Value) -> CompilationResult<'a> {
        if let Value::Object(map) = schema {
            let kctx = ctx.new_at_location("dependencies");
            let mut dependencies = Vec::with_capacity(map.len());
            for (key, subschema) in map {
                let ctx = kctx.new_at_location(key.as_str());
                let s =
                    match subschema {
                        Value::Array(_) => {
                            let validators = vec![required::compile_with_path(
                                subschema,
                                kctx.location().clone(),
                            )
                            .expect("The required validator compilation does not return None")?];
                            SchemaNode::from_array(&kctx, validators)
                        }
                        _ => compiler::compile(&ctx, ctx.as_resource_ref(subschema))?,
                    };
                dependencies.push((key.clone(), s));
            }
            Ok(Box::new(DependenciesValidator { dependencies }))
        } else {
            Err(ValidationError::single_type_error(
                Location::new(),
                ctx.location().clone(),
                schema,
                JsonType::Object,
            ))
        }
    }
}

impl Validate for DependenciesValidator {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        if let Value::Object(item) = instance {
            for (property, node) in &self.dependencies {
                if item.contains_key(property) && !node.is_valid(instance, ctx) {
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
            for (property, dependency) in &self.dependencies {
                if item.contains_key(property) {
                    dependency.validate(instance, location, ctx)?;
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
            for (property, node) in &self.dependencies {
                if item.contains_key(property) {
                    errors.extend(node.iter_errors(instance, location, ctx));
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
            let mut children = Vec::new();
            for (property, dependency) in &self.dependencies {
                if item.contains_key(property) {
                    children.push(dependency.evaluate_instance(instance, location, ctx));
                }
            }
            EvaluationResult::from_children(children)
        } else {
            EvaluationResult::valid_empty()
        }
    }
}

pub(crate) struct DependentRequiredValidator {
    dependencies: Vec<(String, SchemaNode)>,
}

impl DependentRequiredValidator {
    #[inline]
    pub(crate) fn compile<'a>(ctx: &compiler::Context, schema: &'a Value) -> CompilationResult<'a> {
        if let Value::Object(map) = schema {
            let kctx = ctx.new_at_location("dependentRequired");
            let mut dependencies = Vec::with_capacity(map.len());
            for (key, subschema) in map {
                let ictx = kctx.new_at_location(key.as_str());
                if let Value::Array(dependency_array) = subschema {
                    if !unique_items::is_unique(dependency_array) {
                        return Err(ValidationError::unique_items(
                            Location::new(),
                            ictx.location().clone(),
                            subschema,
                        ));
                    }
                    let validators =
                        vec![
                            required::compile_with_path(subschema, kctx.location().clone())
                                .expect(
                                    "The required validator compilation does not return None",
                                )?,
                        ];
                    dependencies.push((key.clone(), SchemaNode::from_array(&kctx, validators)));
                } else {
                    return Err(ValidationError::single_type_error(
                        Location::new(),
                        ictx.location().clone(),
                        subschema,
                        JsonType::Array,
                    ));
                }
            }
            Ok(Box::new(DependentRequiredValidator { dependencies }))
        } else {
            Err(ValidationError::single_type_error(
                Location::new(),
                ctx.location().clone(),
                schema,
                JsonType::Object,
            ))
        }
    }
}
impl Validate for DependentRequiredValidator {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        if let Value::Object(item) = instance {
            for (property, node) in &self.dependencies {
                if item.contains_key(property) && !node.is_valid(instance, ctx) {
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
            for (property, dependency) in &self.dependencies {
                if item.contains_key(property) {
                    dependency.validate(instance, location, ctx)?;
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
            for (property, node) in &self.dependencies {
                if item.contains_key(property) {
                    errors.extend(node.iter_errors(instance, location, ctx));
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
            let mut children = Vec::new();
            for (property, dependency) in &self.dependencies {
                if item.contains_key(property) {
                    children.push(dependency.evaluate_instance(instance, location, ctx));
                }
            }
            EvaluationResult::from_children(children)
        } else {
            EvaluationResult::valid_empty()
        }
    }
}

pub(crate) struct DependentSchemasValidator {
    dependencies: Vec<(String, SchemaNode)>,
}
impl DependentSchemasValidator {
    #[inline]
    pub(crate) fn compile<'a>(ctx: &compiler::Context, schema: &'a Value) -> CompilationResult<'a> {
        if let Value::Object(map) = schema {
            let ctx = ctx.new_at_location("dependentSchemas");
            let mut dependencies = Vec::with_capacity(map.len());
            for (key, subschema) in map {
                let ctx = ctx.new_at_location(key.as_str());
                let schema_nodes = compiler::compile(&ctx, ctx.as_resource_ref(subschema))?;
                dependencies.push((key.clone(), schema_nodes));
            }
            Ok(Box::new(DependentSchemasValidator { dependencies }))
        } else {
            Err(ValidationError::single_type_error(
                Location::new(),
                ctx.location().clone(),
                schema,
                JsonType::Object,
            ))
        }
    }
}
impl Validate for DependentSchemasValidator {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        if let Value::Object(item) = instance {
            for (property, node) in &self.dependencies {
                if item.contains_key(property) && !node.is_valid(instance, ctx) {
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
            for (property, dependency) in &self.dependencies {
                if item.contains_key(property) {
                    dependency.validate(instance, location, ctx)?;
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
            for (property, node) in &self.dependencies {
                if item.contains_key(property) {
                    errors.extend(node.iter_errors(instance, location, ctx));
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
            let mut children = Vec::new();
            for (property, dependency) in &self.dependencies {
                if item.contains_key(property) {
                    children.push(dependency.evaluate_instance(instance, location, ctx));
                }
            }
            EvaluationResult::from_children(children)
        } else {
            EvaluationResult::valid_empty()
        }
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    Some(DependenciesValidator::compile(ctx, schema))
}
#[inline]
pub(crate) fn compile_dependent_required<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    Some(DependentRequiredValidator::compile(ctx, schema))
}
#[inline]
pub(crate) fn compile_dependent_schemas<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    Some(DependentSchemasValidator::compile(ctx, schema))
}
#[cfg(test)]
mod tests {
    use crate::tests_util;
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!({"dependencies": {"bar": ["foo"]}}), &json!({"bar": 1}), "/dependencies")]
    #[test_case(&json!({"dependencies": {"bar": {"type": "string"}}}), &json!({"bar": 1}), "/dependencies/bar/type")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }
}
