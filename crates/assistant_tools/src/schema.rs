use language_model::LanguageModelToolSchemaFormat;
use schemars::{
    schema::{RootSchema, Schema, SchemaObject},
    JsonSchema,
};

pub fn schema_for<T: JsonSchema>(format: LanguageModelToolSchemaFormat) -> RootSchema {
    let mut generator = match format {
        LanguageModelToolSchemaFormat::JsonSchema => schemars::SchemaGenerator::default(),
        LanguageModelToolSchemaFormat::JsonSchemaSubset => {
            schemars::r#gen::SchemaSettings::default()
                .with(|settings| {
                    settings.meta_schema = None;
                    settings.inline_subschemas = true;
                    settings.visitors.push(Box::new(SchemaVisitor));
                })
                .into_generator()
        }
    };
    generator.root_schema_for::<T>()
}

#[derive(Debug, Clone)]
struct SchemaVisitor;

impl schemars::visit::Visitor for SchemaVisitor {
    fn visit_root_schema(&mut self, root: &mut RootSchema) {
        schemars::visit::visit_root_schema(self, root)
    }

    fn visit_schema(&mut self, schema: &mut Schema) {
        schemars::visit::visit_schema(self, schema)
    }

    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        if let Some(subschema) = schema.subschemas.as_mut() {
            if let Some(one_of) = subschema.one_of.take() {
                subschema.any_of = Some(one_of);
            }
        }

        schemars::visit::visit_schema_object(self, schema)
    }
}
