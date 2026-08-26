use schemars::{JsonSchema, transform::transform_subschemas};

const DEFS_PATH: &str = "#/$defs/";

/// Replaces the JSON schema definition for some type if it is in use (in the definitions list), and
/// returns a reference to it.
///
/// This asserts that JsonSchema::schema_name() + "2" does not exist because this indicates that
/// there are multiple types that use this name, and unfortunately schemars APIs do not support
/// resolving this ambiguity - see <https://github.com/GREsau/schemars/issues/449>
///
/// This takes a closure for `schema` because some settings types are not available on the remote
/// server, and so will crash when attempting to access e.g. GlobalThemeRegistry.
pub fn replace_subschema<T: JsonSchema>(
    generator: &mut schemars::SchemaGenerator,
    schema: impl Fn() -> schemars::Schema,
) -> schemars::Schema {
    let schema_name = T::schema_name();
    let definitions = generator.definitions_mut();
    assert!(!definitions.contains_key(&format!("{schema_name}2")));
    assert!(definitions.contains_key(schema_name.as_ref()));
    definitions.insert(schema_name.to_string(), schema().to_value());
    schemars::Schema::new_ref(format!("{DEFS_PATH}{schema_name}"))
}

/// Adds a new JSON schema definition and returns a reference to it. **Panics** if the name is
/// already in use.
pub fn add_new_subschema(
    generator: &mut schemars::SchemaGenerator,
    name: &str,
    schema: serde_json::Value,
) -> schemars::Schema {
    let old_definition = generator.definitions_mut().insert(name.to_string(), schema);
    assert_eq!(old_definition, None);
    schemars::Schema::new_ref(format!("{DEFS_PATH}{name}"))
}

/// Defaults `additionalProperties` to `true`, as if `#[schemars(deny_unknown_fields)]` was on every
/// struct. Skips structs that have `additionalProperties` set (such as if #[serde(flatten)] is used
/// on a map).
#[derive(Clone)]
pub struct DefaultDenyUnknownFields;

impl schemars::transform::Transform for DefaultDenyUnknownFields {
    fn transform(&mut self, schema: &mut schemars::Schema) {
        if let Some(object) = schema.as_object_mut()
            && object.contains_key("properties")
            && !object.contains_key("additionalProperties")
            && !object.contains_key("unevaluatedProperties")
        {
            object.insert("additionalProperties".to_string(), false.into());
        }
        transform_subschemas(self, schema);
    }
}

/// Defaults `allowTrailingCommas` to `true`, for use with `json-language-server`.
/// This can be applied to any schema that will be treated as `jsonc`.
///
/// Note that this is non-recursive and only applied to the root schema.
#[derive(Clone)]
pub struct AllowTrailingCommas;

impl schemars::transform::Transform for AllowTrailingCommas {
    fn transform(&mut self, schema: &mut schemars::Schema) {
        if let Some(object) = schema.as_object_mut()
            && !object.contains_key("allowTrailingCommas")
        {
            object.insert("allowTrailingCommas".to_string(), true.into());
        }
    }
}
