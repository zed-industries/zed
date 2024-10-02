use schemars::schema::{ArrayValidation, InstanceType, RootSchema, Schema, SchemaObject};
use serde_json::Value;

pub struct SettingsJsonSchemaParams<'a> {
    pub staff_mode: bool,
    pub language_names: &'a [String],
    pub font_names: &'a [String],
}

impl<'a> SettingsJsonSchemaParams<'a> {
    pub fn font_family_schema(&self) -> Schema {
        let available_fonts: Vec<_> = self.font_names.iter().cloned().map(Value::String).collect();

        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(available_fonts),
            ..Default::default()
        }
        .into()
    }

    pub fn font_fallback_schema(&self) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(schemars::schema::SingleOrVec::Single(Box::new(
                    self.font_family_schema(),
                ))),
                unique_items: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

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
