use crate::{
    compiler,
    error::ValidationError,
    ext::cmp,
    keywords::CompilationResult,
    paths::{LazyLocation, Location},
    types::{JsonType, JsonTypeSet},
    validator::{Validate, ValidationContext},
};
use serde_json::{Map, Value};

#[derive(Debug)]
pub(crate) struct EnumValidator {
    options: Value,
    // Types that occur in items
    types: JsonTypeSet,
    items: Vec<Value>,
    location: Location,
}

impl EnumValidator {
    #[inline]
    pub(crate) fn compile<'a>(
        schema: &'a Value,
        items: &'a [Value],
        location: Location,
    ) -> CompilationResult<'a> {
        let mut types = JsonTypeSet::empty();
        for item in items {
            types = types.insert(JsonType::from(item));
        }
        Ok(Box::new(EnumValidator {
            options: schema.clone(),
            items: items.to_vec(),
            types,
            location,
        }))
    }
}

impl Validate for EnumValidator {
    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if self.is_valid(instance, ctx) {
            Ok(())
        } else {
            Err(ValidationError::enumeration(
                self.location.clone(),
                location.into(),
                instance,
                &self.options,
            ))
        }
    }

    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        // If the input value type is not in the types present among the enum options, then there
        // is no reason to compare it against all items - we know that
        // there are no items with such type at all
        if self.types.contains_value_type(instance) {
            self.items.iter().any(|item| cmp::equal(instance, item))
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub(crate) struct SingleValueEnumValidator {
    value: Value,
    options: Value,
    location: Location,
}

impl SingleValueEnumValidator {
    #[inline]
    pub(crate) fn compile<'a>(
        schema: &'a Value,
        value: &'a Value,
        location: Location,
    ) -> CompilationResult<'a> {
        Ok(Box::new(SingleValueEnumValidator {
            options: schema.clone(),
            value: value.clone(),
            location,
        }))
    }
}

impl Validate for SingleValueEnumValidator {
    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if self.is_valid(instance, ctx) {
            Ok(())
        } else {
            Err(ValidationError::enumeration(
                self.location.clone(),
                location.into(),
                instance,
                &self.options,
            ))
        }
    }

    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        cmp::equal(&self.value, instance)
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    if let Value::Array(items) = schema {
        let location = ctx.location().join("enum");
        if items.len() == 1 {
            let value = items.iter().next().expect("Vec is not empty");
            Some(SingleValueEnumValidator::compile(schema, value, location))
        } else {
            Some(EnumValidator::compile(schema, items, location))
        }
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

    #[test_case(&json!({"enum": [1]}), &json!(2), "/enum")]
    #[test_case(&json!({"enum": [1, 3]}), &json!(2), "/enum")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }
}
