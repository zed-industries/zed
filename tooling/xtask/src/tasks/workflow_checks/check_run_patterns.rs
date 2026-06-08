use annotate_snippets::{AnnotationKind, Group, Level, Snippet};
use anyhow::{Result, anyhow};
use indexmap::IndexMap;
use regex::Regex;
use serde_yaml::Value;
use std::{
    collections::HashMap,
    fs,
    ops::Range,
    path::{Path, PathBuf},
    sync::LazyLock,
};

static GITHUB_INPUT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\$\{\{[[:blank:]]*([[:alnum:]]|[[:punct:]])+?[[:blank:]]*\}\}"#)
        .expect("Should compile")
});

pub struct WorkflowFile {
    raw_content: String,
    pub parsed_content: Value,
}

impl WorkflowFile {
    pub fn load(workflow_file_path: &Path) -> Result<Self> {
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

pub struct WorkflowValidationError {
    file_path: PathBuf,
    contents: WorkflowFile,
    errors: Vec<ValidationError>,
}

impl WorkflowValidationError {
    pub fn new(errors: Vec<ValidationError>, contents: WorkflowFile, file_path: PathBuf) -> Self {
        Self {
            file_path,
            contents,
            errors,
        }
    }

    pub fn annotation_groups(&self) -> Vec<Group<'_>> {
        let raw_content = &self.contents.raw_content;
        let path = self.file_path.display().to_string();

        let mut spans_by_kind: IndexMap<ValidationErrorKind, Vec<AnnotationSpan>> = IndexMap::new();
        for error in &self.errors {
            spans_by_kind
                .entry(error.kind())
                .or_default()
                .extend(error.annotation_spans());
        }

        spans_by_kind
            .into_iter()
            .map(|(kind, spans)| {
                let ranges = resolve_ranges(raw_content, &spans);
                Level::ERROR.primary_title(kind.group_title()).element(
                    Snippet::source(raw_content).path(path.clone()).annotations(
                        ranges.into_iter().map(move |range| {
                            AnnotationKind::Primary
                                .span(range)
                                .label(kind.annotation_label())
                        }),
                    ),
                )
            })
            .collect()
    }
}

/// Resolves each [`AnnotationSpan`] (a substring plus an offset within it) into an
/// absolute byte range inside `raw_content`.
///
/// The same line may legitimately appear multiple times in a workflow file, so we
/// remember where each `(line, offset)` match ended and resume searching from there
/// to annotate the next occurrence rather than repeatedly highlighting the first one.
fn resolve_ranges(raw_content: &str, spans: &[AnnotationSpan]) -> Vec<Range<usize>> {
    let mut identical_lines: HashMap<(&str, usize), usize> = HashMap::new();

    spans
        .iter()
        .map(|span| {
            let initial_offset = identical_lines
                .get(&(span.line.as_str(), span.range.start))
                .copied()
                .unwrap_or_default();

            let line_start = raw_content[initial_offset..]
                .find(span.line.as_str())
                .map(|offset| offset + initial_offset)
                .unwrap_or_default();

            let span_start = line_start + span.range.start;
            let span_end = span_start + span.range.len();

            identical_lines.insert((span.line.as_str(), span.range.start), span_end);

            span_start..span_end
        })
        .collect()
}

/// The category a [`ValidationError`] belongs to. All errors sharing a kind are
/// rendered together in a single annotation group, using the kind's title and label.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ValidationErrorKind {
    Injection,
    UnpinnedAction,
}

impl ValidationErrorKind {
    fn group_title(self) -> &'static str {
        match self {
            Self::Injection => "Found GitHub input injection in run command",
            Self::UnpinnedAction => "Found GitHub action not pinned to a commit SHA",
        }
    }

    fn annotation_label(self) -> &'static str {
        match self {
            Self::Injection => "This should be passed via an environment variable",
            Self::UnpinnedAction => "This action should be pinned to a full-length commit SHA",
        }
    }
}

/// A substring of a workflow file to highlight. `line` is searched for verbatim in the
/// file contents and `range` selects the portion of that match to underline.
struct AnnotationSpan {
    line: String,
    range: Range<usize>,
}

pub struct UsesValidationError {
    unpinned_action: String,
}

pub struct RunValidationError {
    found_injection_patterns: Vec<(String, Range<usize>)>,
}

pub enum ValidationError {
    Run(RunValidationError),
    Uses(UsesValidationError),
}

impl ValidationError {
    /// The category this error belongs to, used to group errors of the same kind into a
    /// single annotation group.
    fn kind(&self) -> ValidationErrorKind {
        match self {
            ValidationError::Run(_) => ValidationErrorKind::Injection,
            ValidationError::Uses(_) => ValidationErrorKind::UnpinnedAction,
        }
    }

    /// The substrings of the workflow file this error wants to highlight.
    fn annotation_spans(&self) -> Vec<AnnotationSpan> {
        match self {
            ValidationError::Run(error) => error
                .found_injection_patterns
                .iter()
                .map(|(line, range)| AnnotationSpan {
                    line: line.clone(),
                    range: range.clone(),
                })
                .collect(),
            ValidationError::Uses(error) => vec![AnnotationSpan {
                range: 0..error.unpinned_action.len(),
                line: error.unpinned_action.clone(),
            }],
        }
    }
}

pub fn validate_run_command(command: &str) -> Result<(), RunValidationError> {
    let patterns: Vec<_> = command
        .lines()
        .flat_map(move |line| {
            GITHUB_INPUT_PATTERN
                .find_iter(line)
                .map(|m| (line.to_owned(), m.range()))
        })
        .collect();

    if patterns.is_empty() {
        Ok(())
    } else {
        Err(RunValidationError {
            found_injection_patterns: patterns,
        })
    }
}

const COMMIT_SHA_LENGTH: usize = 40;

pub fn validate_uses_command(command: &str) -> Result<(), UsesValidationError> {
    // Local actions (`./path` or `../path`) live in this repository and have no ref
    if command.starts_with("./") || command.starts_with("../") {
        return Ok(());
    }

    let is_pinned_to_commit_sha = command
        .split_once('@')
        .map(|(_action, git_ref)| git_ref)
        .is_some_and(|git_ref| {
            git_ref.len() == COMMIT_SHA_LENGTH
                && git_ref.chars().all(|char| char.to_digit(16).is_some())
        });

    if is_pinned_to_commit_sha {
        Ok(())
    } else {
        Err(UsesValidationError {
            unpinned_action: command.to_owned(),
        })
    }
}
