use schemars::schema::{RootSchema, Schema};

type PropertyName<'a> = &'a str;
type ReferencePath<'a> = &'a str;

/// Modifies the provided [`RootSchema`] by adding references to all of the specified properties.
///
/// # Examples
///
/// ```
/// # let root_schema = RootSchema::default();
/// add_references_to_properties(&mut root_schema, &[
///     ("property_a", "#/definitions/DefinitionA"),
///     ("property_b", "#/definitions/DefinitionB"),
/// ])
/// ```
pub fn add_references_to_properties(
    root_schema: &mut RootSchema,
    properties_with_references: &[(PropertyName, ReferencePath)],
) {
    for (property, definition) in properties_with_references {
        let Some(schema) = root_schema.schema.object().properties.get_mut(*property) else {
            log::warn!("property '{property}' not found in JSON schema");
            continue;
        };

        match schema {
            Schema::Object(schema) => {
                schema.reference = Some(definition.to_string());
            }
            Schema::Bool(_) => {
                // Boolean schemas can't have references.
            }
        }
    }
}
