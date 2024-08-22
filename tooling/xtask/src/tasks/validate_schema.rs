use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use schemars::{
    schema::{RootSchema, SchemaObject},
    visit::Visitor,
};
use serde_json::Value;

#[derive(Parser)]
pub struct SchemaValidationArgs {
    path: PathBuf,
}

enum Violation {
    MissingDefault,
    MissingExample,
    MissingMetadata,
}
struct Violations(SchemaObject, Vec<Violation>);

#[derive(Default)]
struct SettingsSchemaValidator {
    objects: Vec<Violations>,
}

struct ViolationsBuilder<'a> {
    target: &'a mut SettingsSchemaValidator,
    violation: Violation,
}

impl Drop for ViolationsBuilder<'_> {
    fn drop(&mut self) {
        if !self.violation.1.is_empty() {
            self.target.objects.push(self.violation);
        }
    }
}
impl Visitor for SettingsSchemaValidator {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        let mut violation = Violations(schema.clone(), vec![]);

        let Some(metadata) = schema.metadata.as_ref() else {};

        if schema
            .metadata
            .as_ref()
            .map_or(false, |meta| meta.default.as_ref().is_none())
        {
            self.objects.entry()
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

    Ok(())
}
