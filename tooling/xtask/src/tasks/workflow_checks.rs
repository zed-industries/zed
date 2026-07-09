mod check_permissions;
mod check_run_patterns;

use std::{
    fs,
    path::{Path, PathBuf},
};

use annotate_snippets::{Group, Renderer};
use anyhow::{Result, anyhow};
use clap::Parser;
use itertools::{Either, Itertools};
use serde_yaml::Value;
use strum::IntoEnumIterator;

use crate::tasks::workflows::WorkflowType;

use check_permissions::PermissionsError;
use check_run_patterns::RunValidationError;

pub use check_run_patterns::validate_run_command;

#[derive(Default, Parser)]
pub struct WorkflowValidationArgs {}

pub fn validate(_: WorkflowValidationArgs) -> Result<()> {
    let (parsing_errors, file_errors): (Vec<_>, Vec<_>) = get_all_workflow_files()
        .map(check_workflow)
        .flat_map(Result::err)
        .partition_map(|error| match error {
            WorkflowError::ParseError(error) => Either::Left(error),
            WorkflowError::ValidationError(error) => Either::Right(error),
        });

    if !parsing_errors.is_empty() {
        Err(anyhow!(
            "Failed to read or parse some workflow files: {}",
            parsing_errors.into_iter().join("\n")
        ))
    } else if !file_errors.is_empty() {
        let groups: Vec<_> = file_errors
            .iter()
            .flat_map(|error| error.annotation_groups())
            .collect();

        let renderer =
            Renderer::styled().decor_style(annotate_snippets::renderer::DecorStyle::Ascii);
        println!("{}", renderer.render(groups.as_slice()));

        Err(anyhow!("Workflow checks failed!"))
    } else {
        Ok(())
    }
}

struct WorkflowFile {
    raw_content: String,
    parsed_content: Value,
}

impl WorkflowFile {
    fn load(workflow_file_path: &Path) -> Result<Self> {
        fs::read_to_string(workflow_file_path)
            .map_err(|_| {
                anyhow!(
                    "Could not read workflow file at {}",
                    workflow_file_path.display()
                )
            })
            .and_then(|file_content| {
                serde_yaml::from_str(&file_content)
                    .map(|parsed_content| Self {
                        raw_content: file_content,
                        parsed_content,
                    })
                    .map_err(|e| anyhow!("Failed to parse workflow file: {e:?}"))
            })
    }
}

/// A single kind of validation failure found within a workflow file.
enum ValidationError {
    RunInjection(RunValidationError),
    Permissions(PermissionsError),
}

impl ValidationError {
    fn annotation_group<'a>(&self, file_path: &Path, raw_content: &'a str) -> Group<'a> {
        match self {
            ValidationError::RunInjection(error) => error.annotation_group(file_path, raw_content),
            ValidationError::Permissions(error) => error.annotation_group(file_path, raw_content),
        }
    }
}

struct WorkflowValidationError {
    file_path: PathBuf,
    contents: WorkflowFile,
    errors: Vec<ValidationError>,
}

impl WorkflowValidationError {
    fn annotation_groups(&self) -> Vec<Group<'_>> {
        self.errors
            .iter()
            .map(|error| error.annotation_group(&self.file_path, &self.contents.raw_content))
            .collect()
    }
}

enum WorkflowError {
    ParseError(anyhow::Error),
    ValidationError(Box<WorkflowValidationError>),
}

fn get_all_workflow_files() -> impl Iterator<Item = PathBuf> {
    WorkflowType::iter()
        .map(|workflow_type| workflow_type.folder_path())
        .flat_map(|folder_path| {
            fs::read_dir(folder_path).into_iter().flat_map(|entries| {
                entries
                    .flat_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| {
                        path.extension()
                            .is_some_and(|ext| ext == "yaml" || ext == "yml")
                    })
            })
        })
}

fn check_workflow(workflow_file_path: PathBuf) -> Result<(), WorkflowError> {
    let file_content =
        WorkflowFile::load(&workflow_file_path).map_err(WorkflowError::ParseError)?;

    let mut errors = Vec::new();

    if let Err(error) = check_permissions::validate_permissions(&file_content.parsed_content) {
        errors.push(ValidationError::Permissions(error));
    }

    errors.extend(
        collect_run_injection_errors(&Value::Null, &file_content.parsed_content)
            .into_iter()
            .map(ValidationError::RunInjection),
    );

    if errors.is_empty() {
        Ok(())
    } else {
        Err(WorkflowError::ValidationError(Box::new(
            WorkflowValidationError {
                file_path: workflow_file_path,
                contents: file_content,
                errors,
            },
        )))
    }
}

fn collect_run_injection_errors(key: &Value, value: &Value) -> Vec<RunValidationError> {
    match value {
        Value::Mapping(mapping) => mapping
            .iter()
            .flat_map(|(key, value)| collect_run_injection_errors(key, value))
            .collect(),
        Value::Sequence(sequence) => sequence
            .iter()
            .flat_map(|value| collect_run_injection_errors(key, value))
            .collect(),
        Value::String(string) => check_string(key, string).err().into_iter().collect(),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::Tagged(_) => Vec::new(),
    }
}

fn check_string(key: &Value, value: &str) -> Result<(), RunValidationError> {
    match key {
        Value::String(key) if key == "run" => validate_run_command(value),
        _ => Ok(()),
    }
}
