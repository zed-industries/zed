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

#[derive(Debug)]
enum Violation {
    MissingDefault,
    MissingExample,
    MissingMetadata,
    MissingDescription,
}
struct Violations {
    schema: SchemaObject,
    violations: Vec<(Vec<String>, Violation)>,
}

#[derive(Default)]
struct SettingsSchemaValidator {
    objects: Vec<Violations>,
}

impl SettingsSchemaValidator {
    fn validate_schema_object(&mut self, builder: &mut ViolationsBuilder, schema: &SchemaObject) {
        if schema.metadata.is_none() {
            builder.add(Violation::MissingMetadata);
        } else if let Some(metadata) = &schema.metadata {
            if metadata.description.is_none() {
                builder.add(Violation::MissingDescription);
            }
            if metadata.default.is_none() {
                builder.add(Violation::MissingDefault);
            }
            if metadata.examples.is_empty() && !Self::is_boolean_schema(schema) {
                builder.add(Violation::MissingExample);
            }
        }

        if let Some(object) = &schema.object {
            for (property, property_schema) in &object.properties {
                builder.push(property.clone());
                self.validate_schema_object(builder, &property_schema.clone().into_object());
                builder.pop();
            }
        }
    }

    fn is_boolean_schema(schema: &SchemaObject) -> bool {
        schema.instance_type == Some(schemars::schema::InstanceType::Boolean.into())
    }
}

struct ViolationsBuilder<'a> {
    violations: &'a mut Violations,
    current_path: Vec<String>,
}

impl<'a> ViolationsBuilder<'a> {
    fn new(violations: &'a mut Violations) -> Self {
        Self {
            violations,
            current_path: Vec::new(),
        }
    }

    fn add(&mut self, violation: Violation) {
        self.violations
            .violations
            .push((self.current_path.clone(), violation));
    }

    fn push(&mut self, segment: String) {
        self.current_path.push(segment);
    }

    fn pop(&mut self) {
        self.current_path.pop();
    }
}

impl Visitor for SettingsSchemaValidator {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        let mut violations = Violations {
            schema: schema.clone(),
            violations: Vec::new(),
        };
        let mut builder = ViolationsBuilder::new(&mut violations);

        self.validate_schema_object(&mut builder, schema);

        if !violations.violations.is_empty() {
            self.objects.push(violations);
        }

        schemars::visit::visit_schema_object(self, schema)
    }
}

pub fn run_validate_schema(args: SchemaValidationArgs) -> Result<()> {
    let contents = std::fs::read_to_string(args.path)?;
    let mut schema: RootSchema = serde_json::from_str(&contents)?;
    let mut validator = SettingsSchemaValidator::default();
    validator.visit_root_schema(&mut schema);

    let total_violations: usize = validator.objects.iter().map(|v| v.violations.len()).sum();
    println!("Total violations found: {}", total_violations);

    for violation in validator.objects.iter().flat_map(|v| &v.violations) {
        let path_str = if violation.0.is_empty() {
            "<root>".to_string()
        } else {
            violation.0.join(".")
        };
        println!("{}: {:?}", path_str, violation.1);
    }

    Ok(())
}
