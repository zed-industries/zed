use crate::{
    compiler, node::SchemaNode, paths::Location, thread::ThreadBound, validator::Validate as _,
};
use ahash::AHashMap;
use serde_json::{Map, Value};

use crate::ValidationError;

pub(crate) type FancyRegexValidators = Vec<(fancy_regex::Regex, SchemaNode)>;
pub(crate) type RegexValidators = Vec<(regex::Regex, SchemaNode)>;

/// A value that can look up property validators by name.
pub(crate) trait PropertiesValidatorsMap: ThreadBound {
    fn get_validator(&self, property: &str) -> Option<&SchemaNode>;
    fn get_key_validator(&self, property: &str) -> Option<(&String, &SchemaNode)>;
}

// We're defining two different property validator map implementations, one for small map sizes and
// one for large map sizes, to optimize the performance depending on the number of properties
// present.
//
// Implementors should use `compile_dynamic_prop_map_validator!` for building their validator maps
// at runtime, as it wraps up all of the logic to choose the right map size and then build and
// compile the validator.
pub(crate) type SmallValidatorsMap = Vec<(String, SchemaNode)>;
pub(crate) type BigValidatorsMap = AHashMap<String, SchemaNode>;

impl PropertiesValidatorsMap for SmallValidatorsMap {
    #[inline]
    fn get_validator(&self, property: &str) -> Option<&SchemaNode> {
        for (prop, node) in self {
            if prop == property {
                return Some(node);
            }
        }
        None
    }
    #[inline]
    fn get_key_validator(&self, property: &str) -> Option<(&String, &SchemaNode)> {
        for (prop, node) in self {
            if prop == property {
                return Some((prop, node));
            }
        }
        None
    }
}

impl PropertiesValidatorsMap for BigValidatorsMap {
    #[inline]
    fn get_validator(&self, property: &str) -> Option<&SchemaNode> {
        self.get(property)
    }

    #[inline]
    fn get_key_validator(&self, property: &str) -> Option<(&String, &SchemaNode)> {
        self.get_key_value(property)
    }
}

pub(crate) fn compile_small_map<'a>(
    ctx: &compiler::Context,
    map: &'a Map<String, Value>,
) -> Result<SmallValidatorsMap, ValidationError<'a>> {
    let mut properties = Vec::with_capacity(map.len());
    let kctx = ctx.new_at_location("properties");
    for (key, subschema) in map {
        let pctx = kctx.new_at_location(key.as_str());
        properties.push((
            key.clone(),
            compiler::compile(&pctx, pctx.as_resource_ref(subschema))?,
        ));
    }
    Ok(properties)
}

pub(crate) fn compile_big_map<'a>(
    ctx: &compiler::Context,
    map: &'a Map<String, Value>,
) -> Result<BigValidatorsMap, ValidationError<'a>> {
    let mut properties = AHashMap::with_capacity(map.len());
    let kctx = ctx.new_at_location("properties");
    for (key, subschema) in map {
        let pctx = kctx.new_at_location(key.as_str());
        properties.insert(
            key.clone(),
            compiler::compile(&pctx, pctx.as_resource_ref(subschema))?,
        );
    }
    Ok(properties)
}

pub(crate) fn are_properties_valid<M, F>(
    prop_map: &M,
    props: &Map<String, Value>,
    ctx: &mut crate::validator::ValidationContext,
    check: F,
) -> bool
where
    M: PropertiesValidatorsMap,
    F: Fn(&Value, &mut crate::validator::ValidationContext) -> bool,
{
    for (property, instance) in props {
        if let Some(validator) = prop_map.get_validator(property) {
            if !validator.is_valid(instance, ctx) {
                return false;
            }
        } else if !check(instance, ctx) {
            return false;
        }
    }
    true
}

/// Create a vector of pattern-validators pairs.
#[inline]
pub(crate) fn compile_fancy_regex_patterns<'a>(
    ctx: &compiler::Context,
    obj: &'a Map<String, Value>,
) -> Result<FancyRegexValidators, ValidationError<'a>> {
    let kctx = ctx.new_at_location("patternProperties");
    let mut compiled_patterns = Vec::with_capacity(obj.len());
    for (pattern, subschema) in obj {
        let pctx = kctx.new_at_location(pattern.as_str());
        let compiled_pattern = ctx.get_or_compile_regex(pattern).map_err(|()| {
            ValidationError::format(Location::new(), kctx.location().clone(), subschema, "regex")
        })?;
        let node = compiler::compile(&pctx, pctx.as_resource_ref(subschema))?;
        compiled_patterns.push(((*compiled_pattern).clone(), node));
    }
    Ok(compiled_patterns)
}

#[inline]
pub(crate) fn compile_regex_patterns<'a>(
    ctx: &compiler::Context,
    obj: &'a Map<String, Value>,
) -> Result<RegexValidators, ValidationError<'a>> {
    let kctx = ctx.new_at_location("patternProperties");
    let mut compiled_patterns = Vec::with_capacity(obj.len());
    for (pattern, subschema) in obj {
        let pctx = kctx.new_at_location(pattern.as_str());
        let compiled_pattern = ctx.get_or_compile_standard_regex(pattern).map_err(|()| {
            ValidationError::format(Location::new(), kctx.location().clone(), subschema, "regex")
        })?;
        let node = compiler::compile(&pctx, pctx.as_resource_ref(subschema))?;
        compiled_patterns.push(((*compiled_pattern).clone(), node));
    }
    Ok(compiled_patterns)
}

macro_rules! compile_dynamic_prop_map_validator {
    ($validator:tt, $properties:ident, $( $arg:expr ),* $(,)*) => {{
        if let Value::Object(map) = $properties {
            if map.len() < 40 {
                Some($validator::<SmallValidatorsMap>::compile(
                    map, $($arg, )*
                ))
            } else {
                Some($validator::<BigValidatorsMap>::compile(
                    map, $($arg, )*
                ))
            }
        } else {
            Some(Err(ValidationError::custom(
                Location::new(),
                Location::new(),
                $properties,
                "Unexpected type",
            )))
        }
    }};
}

pub(crate) use compile_dynamic_prop_map_validator;
