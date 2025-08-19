use anyhow::Result;
use language_model::LanguageModelToolSchemaFormat;
use schemars::{
    JsonSchema, Schema,
    generate::SchemaSettings,
    transform::{Transform, transform_subschemas},
};

pub fn json_schema_for<T: JsonSchema>(
    format: LanguageModelToolSchemaFormat,
) -> Result<serde_json::Value> {
    let schema = root_schema_for::<T>(format);
    schema_to_json(&schema, format)
}

fn schema_to_json(
    schema: &Schema,
    format: LanguageModelToolSchemaFormat,
) -> Result<serde_json::Value> {
    let mut value = serde_json::to_value(schema)?;
    assistant_tool::adapt_schema_to_format(&mut value, format)?;
    Ok(value)
}

fn root_schema_for<T: JsonSchema>(format: LanguageModelToolSchemaFormat) -> Schema {
    let mut generator = match format {
        LanguageModelToolSchemaFormat::JsonSchema => SchemaSettings::draft07().into_generator(),
        LanguageModelToolSchemaFormat::JsonSchemaSubset => SchemaSettings::openapi3()
            .with(|settings| {
                settings.meta_schema = None;
                settings.inline_subschemas = true;
            })
            .with_transform(ToJsonSchemaSubsetTransform)
            .into_generator(),
    };
    generator.root_schema_for::<T>()
}

#[derive(Debug, Clone)]
struct ToJsonSchemaSubsetTransform;

impl Transform for ToJsonSchemaSubsetTransform {
    fn transform(&mut self, schema: &mut Schema) {
        // Ensure that the type field is not an array, this happens when we use
        // Option<T>, the type will be [T, "null"].
        if let Some(type_field) = schema.get_mut("type")
            && let Some(types) = type_field.as_array()
            && let Some(first_type) = types.first()
        {
            *type_field = first_type.clone();
        }

        // oneOf is not supported, use anyOf instead
        if let Some(one_of) = schema.remove("oneOf") {
            schema.insert("anyOf".to_string(), one_of);
        }

        transform_subschemas(self, schema);
    }
}
