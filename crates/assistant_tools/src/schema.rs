use anyhow::Result;
use language_model::LanguageModelToolSchemaFormat;
use schemars::{
    JsonSchema,
    schema::{RootSchema, Schema, SchemaObject},
};

pub fn json_schema_for<T: JsonSchema>(
    format: LanguageModelToolSchemaFormat,
) -> Result<serde_json::Value> {
    let schema = root_schema_for::<T>(format);
    schema_to_json(&schema, format)
}

fn schema_to_json(
    schema: &RootSchema,
    format: LanguageModelToolSchemaFormat,
) -> Result<serde_json::Value> {
    let mut value = serde_json::to_value(schema)?;
    assistant_tool::adapt_schema_to_format(&mut value, format)?;
    Ok(value)
}

pub fn root_schema_for<T: JsonSchema>(format: LanguageModelToolSchemaFormat) -> RootSchema {
    let mut generator = match format {
        LanguageModelToolSchemaFormat::JsonSchema => schemars::SchemaGenerator::default(),
        LanguageModelToolSchemaFormat::JsonSchemaSubset => {
            schemars::r#gen::SchemaSettings::default()
                .with(|settings| {
                    settings.meta_schema = None;
                    settings.inline_subschemas = true;
                    settings
                        .visitors
                        .push(Box::new(TransformToJsonSchemaSubsetVisitor));
                })
                .into_generator()
        }
    };
    generator.root_schema_for::<T>()
}

#[derive(Debug, Clone)]
struct TransformToJsonSchemaSubsetVisitor;

impl schemars::visit::Visitor for TransformToJsonSchemaSubsetVisitor {
    fn visit_root_schema(&mut self, root: &mut RootSchema) {
        schemars::visit::visit_root_schema(self, root)
    }

    fn visit_schema(&mut self, schema: &mut Schema) {
        schemars::visit::visit_schema(self, schema)
    }

    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        // Ensure that the type field is not an array, this happens when we use
        // Option<T>, the type will be [T, "null"].
        if let Some(instance_type) = schema.instance_type.take() {
            schema.instance_type = match instance_type {
                schemars::schema::SingleOrVec::Single(t) => {
                    Some(schemars::schema::SingleOrVec::Single(t))
                }
                schemars::schema::SingleOrVec::Vec(items) => items
                    .into_iter()
                    .next()
                    .map(schemars::schema::SingleOrVec::from),
            };
        }

        // One of is not supported, use anyOf instead.
        if let Some(subschema) = schema.subschemas.as_mut() {
            if let Some(one_of) = subschema.one_of.take() {
                subschema.any_of = Some(one_of);
            }
        }

        schemars::visit::visit_schema_object(self, schema)
    }
}
