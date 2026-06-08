mod check_run_patterns;

use std::{fs, path::PathBuf};

use annotate_snippets::Renderer;
use anyhow::{Result, anyhow};
use clap::Parser;
use itertools::{Either, Itertools};
use serde_yaml::Value;
use strum::IntoEnumIterator;

use crate::tasks::{
    workflow_checks::check_run_patterns::{
        ValidationError, WorkflowFile, WorkflowValidationError, validate_uses_command,
    },
    workflows::WorkflowType,
};

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
        let errors: Vec<_> = file_errors
            .iter()
            .flat_map(|error| error.annotation_groups())
            .collect();

        let renderer =
            Renderer::styled().decor_style(annotate_snippets::renderer::DecorStyle::Ascii);
        println!("{}", renderer.render(errors.as_slice()));

        Err(anyhow!("Workflow checks failed!"))
    } else {
        Ok(())
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
    fn collect_errors<T>(iter: impl Iterator<Item = Result<(), Vec<T>>>) -> Result<(), Vec<T>> {
        Some(iter.flat_map(Result::err).flatten().collect::<Vec<_>>())
            .filter(|errors| !errors.is_empty())
            .map_or(Ok(()), Err)
    }

    fn check_recursive(key: &Value, value: &Value) -> Result<(), Vec<ValidationError>> {
        match value {
            Value::Mapping(mapping) => collect_errors(
                mapping
                    .into_iter()
                    .map(|(key, value)| check_recursive(key, value)),
            ),
            Value::Sequence(sequence) => collect_errors(
                sequence
                    .into_iter()
                    .map(|value| check_recursive(key, value)),
            ),
            Value::String(string) => check_string(key, string).map_err(|error| vec![error]),
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::Tagged(_) => Ok(()),
        }
    }

    let file_content =
        WorkflowFile::load(&workflow_file_path).map_err(WorkflowError::ParseError)?;

    check_recursive(&Value::Null, &file_content.parsed_content).map_err(|errors| {
        WorkflowError::ValidationError(Box::new(WorkflowValidationError::new(
            errors,
            file_content,
            workflow_file_path,
        )))
    })
}

fn check_string(key: &Value, value: &str) -> Result<(), ValidationError> {
    match key {
        Value::String(key) if key == "run" => {
            validate_run_command(value).map_err(ValidationError::Run)
        }
        Value::String(key) if key == "uses" => {
            validate_uses_command(value).map_err(ValidationError::Uses)
        }
        _ => Ok(()),
    }
}
