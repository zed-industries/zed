use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use schemars::{schema::RootSchema, visit::Visitor};
use serde_json::Value;

#[derive(Parser)]
pub struct SchemaValidationArgs {
    path: PathBuf,
}

#[derive(Default)]
struct SettingsSchemaValidator {
    count: usize,
    default: usize,
}

impl Visitor for SettingsSchemaValidator {
    fn visit_schema_object(&mut self, schema: &mut schemars::schema::SchemaObject) {
        self.count += 1;
        if schema.metadata.as_ref().map_or(false, |meta| {
            meta.default.as_ref().is_some_and(|d| d != &Value::Null)
        }) {
            dbg!(&schema.metadata);
            self.default += 1;
        }
        schemars::visit::visit_schema_object(self, schema)
    }
}

pub fn run_validate_schema(args: SchemaValidationArgs) -> Result<()> {
    let contents = std::fs::read_to_string(args.path)?;
    let mut schema: RootSchema = serde_json::from_str(&contents)?;
    //schemars::visit::
    let mut validator = SettingsSchemaValidator::default();
    validator.visit_root_schema(&mut schema);
    println!(
        "Found {} schemas, {} with defaults",
        validator.count, validator.default
    );
    Ok(())
}
