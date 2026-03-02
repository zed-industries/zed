mod check_run_patterns;

use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    ops::Range,
    path::{Path, PathBuf},
};

use annotate_snippets::Renderer;
use anyhow::{Result, anyhow};
use clap::Parser;
use itertools::{Either, Itertools};
use serde_yaml::Value;
use strum::IntoEnumIterator;

use crate::tasks::{
    workflow_validation::check_run_patterns::{
        InvalidPatternsErrror, annotations_for_indices, validate_run_command,
    },
    workflows::WorkflowType,
};

#[derive(Parser)]
pub struct WorkflowValidationArgs {}

pub fn validate(_: WorkflowValidationArgs) -> Result<()> {
    let (parsing_errors, file_errors): (Vec<_>, Vec<_>) = WorkflowType::iter()
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
        struct Error {
            raw_content: String,
            file_path: PathBuf,
            ranges: Vec<Range<usize>>,
        }

        let file_errors = file_errors
            .into_iter()
            .map(|file_error| {
                let raw_content = &file_error.contents.raw_content;
                let mut identical_lines = HashMap::new();

                let ranges = file_error
                    .errors
                    .into_iter()
                    .flat_map(|error| error.patterns.into_iter())
                    .map(|(line, pattern_range)| {
                        let initial_offset = identical_lines
                            .get(&(Cow::Borrowed(line.as_str()), pattern_range.start))
                            .copied()
                            .unwrap_or_default();

                        let line_start = raw_content[initial_offset..]
                            .find(&line)
                            .map(|offset| offset + initial_offset)
                            .unwrap_or_default();

                        let pattern_start = line_start + pattern_range.start;
                        let pattern_end = pattern_start + pattern_range.len();

                        identical_lines
                            .insert((Cow::Owned(line), pattern_range.start), pattern_end);

                        pattern_start..pattern_end
                    });

                Error {
                    file_path: file_error.file_path,
                    ranges: ranges.collect(),
                    raw_content: file_error.contents.raw_content,
                }
            })
            .collect::<Vec<_>>();

        let errors: Vec<_> = file_errors
            .iter()
            .map(|error| {
                annotations_for_indices(
                    error.ranges.iter().cloned(),
                    &error.raw_content,
                    &error.file_path,
                )
            })
            .collect();

        let renderer =
            Renderer::styled().decor_style(annotate_snippets::renderer::DecorStyle::Unicode);
        anstream::println!("{}", renderer.render(errors.as_slice()));

        Err(anyhow!("Validation failed"))
    } else {
        Ok(())
    }
}

struct WorkflowFile {
    raw_content: String,
    parsed_content: Value,
}

enum WorkflowError {
    ParseError(anyhow::Error),
    ValidationError(WorkflowValidationError),
}

struct WorkflowValidationError {
    file_path: PathBuf,
    contents: WorkflowFile,
    errors: Vec<InvalidPatternsErrror>,
}

fn check_workflow(workflow_file_path: PathBuf) -> Result<(), WorkflowError> {
    fn check_recursive(key: &Value, value: &Value) -> Result<(), Vec<InvalidPatternsErrror>> {
        match value {
            Value::Mapping(mapping) => mapping
                .iter()
                .map(|(key, value)| check_recursive(key, value))
                .fold(Ok(()), fold_errors),
            Value::Sequence(sequence) => sequence
                .iter()
                .map(|value| check_recursive(key, value))
                .fold(Ok(()), fold_errors),
            Value::String(string) => check_string(key, string).map_err(|error| vec![error]),
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::Tagged(_) => Ok(()),
        }
    }

    fn fold_errors(
        acc: Result<(), Vec<InvalidPatternsErrror>>,
        result: Result<(), Vec<InvalidPatternsErrror>>,
    ) -> Result<(), Vec<InvalidPatternsErrror>> {
        match result {
            Ok(()) => acc,
            Err(mut errors) => match acc {
                Ok(_) => Err(errors),
                Err(mut existing_errors) => {
                    existing_errors.append(&mut errors);
                    Err(existing_errors)
                }
            },
        }
    }

    let workflow_file =
        load_workflow_file(&workflow_file_path).map_err(WorkflowError::ParseError)?;

    check_recursive(&Value::Null, &workflow_file.parsed_content).map_err(|errors| {
        WorkflowError::ValidationError(WorkflowValidationError {
            file_path: workflow_file_path,
            contents: workflow_file,
            errors,
        })
    })
}

fn check_string(key: &Value, value: &str) -> Result<(), InvalidPatternsErrror> {
    match key {
        Value::String(key) if key == "run" => validate_run_command(value),
        _ => Ok(()),
    }
}

fn load_workflow_file(workflow_file_path: &Path) -> Result<WorkflowFile> {
    fs::read_to_string(workflow_file_path)
        .map_err(|_| {
            anyhow!(
                "Could not read workflow file at {}",
                workflow_file_path.display()
            )
        })
        .and_then(|file_content| {
            serde_yaml::from_str(&file_content)
                .map(|parsed_file| WorkflowFile {
                    raw_content: file_content,
                    parsed_content: parsed_file,
                })
                .map_err(|e| anyhow!("Failed to parse workflow file: {e:?}"))
        })
}
