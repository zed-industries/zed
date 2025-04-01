use anyhow::Result;
use language_model::LanguageModelToolSchemaFormat;
use schemars::{
    JsonSchema,
    schema::{RootSchema, Schema, SchemaObject},
};

pub fn json_schema_for<T: JsonSchema>(format: LanguageModelToolSchemaFormat) -> serde_json::Value {
    let schema = root_schema_for::<T>(format);
    schema_to_json(&schema, format).expect("Failed to convert tool calling schema to JSON")
}

pub fn schema_to_json(
    schema: &RootSchema,
    format: LanguageModelToolSchemaFormat,
) -> Result<serde_json::Value> {
    let mut value = serde_json::to_value(schema)?;
    match format {
        LanguageModelToolSchemaFormat::JsonSchema => Ok(value),
        LanguageModelToolSchemaFormat::JsonSchemaSubset => {
            transform_fields_to_json_schema_subset(&mut value);
            Ok(value)
        }
    }
}

fn root_schema_for<T: JsonSchema>(format: LanguageModelToolSchemaFormat) -> RootSchema {
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

fn transform_fields_to_json_schema_subset(json: &mut serde_json::Value) {
    if let serde_json::Value::Object(obj) = json {
        if let Some(default) = obj.get("default") {
            let is_null = default.is_null();
            //Default is not supported, so we need to remove it.
            obj.remove("default");
            if is_null {
                obj.insert("nullable".to_string(), serde_json::Value::Bool(true));
            }
        }

        // If a type is not specified for an input parameter we need to add it.
        if obj.contains_key("description")
            && !obj.contains_key("type")
            && !(obj.contains_key("anyOf")
                || obj.contains_key("oneOf")
                || obj.contains_key("allOf"))
        {
            obj.insert(
                "type".to_string(),
                serde_json::Value::String("string".to_string()),
            );
        }

        //Format field is only partially supported (e.g. not uint compatibility)
        obj.remove("format");

        for (_, value) in obj.iter_mut() {
            if let serde_json::Value::Object(_) | serde_json::Value::Array(_) = value {
                transform_fields_to_json_schema_subset(value);
            }
        }
    } else if let serde_json::Value::Array(arr) = json {
        for item in arr.iter_mut() {
            transform_fields_to_json_schema_subset(item);
        }
    }
}
