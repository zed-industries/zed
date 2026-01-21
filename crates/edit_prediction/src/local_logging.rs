use crate::StoredEvent;
use crate::example_spec::ExampleSpec;
use crate::prediction::EditPrediction;
use anyhow::Result;
use gpui::{App, Entity};
use language::ToPoint as _;
use project::Project;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use zeta_prompt::{RelatedFile, ZetaVersion, format_zeta_prompt};

const LOGGING_SUBDIR: &str = "edit_prediction_logs";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggedExample {
    #[serde(flatten)]
    pub spec: ExampleSpec,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_inputs: Option<LoggedPromptInputs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<LoggedPrompt>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub predictions: Vec<LoggedPrediction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggedPromptInputs {
    pub content: String,
    pub cursor_row: u32,
    pub cursor_column: u32,
    pub cursor_offset: usize,
    pub edit_history: Vec<Arc<zeta_prompt::Event>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_files: Option<Vec<RelatedFile>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggedPrompt {
    pub input: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub expected_output: String,
    pub provider: LoggedPredictionProvider,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggedPrediction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_patch: Option<String>,
    pub actual_output: String,
    pub provider: LoggedPredictionProvider,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoggedPredictionProvider {
    Zeta1,
    Zeta2(ZetaVersion),
}

impl std::fmt::Display for LoggedPredictionProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoggedPredictionProvider::Zeta1 => write!(f, "zeta1"),
            LoggedPredictionProvider::Zeta2(version) => write!(f, "zeta2:{version}"),
        }
    }
}

impl Serialize for LoggedPredictionProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for LoggedPredictionProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "zeta1" {
            Ok(LoggedPredictionProvider::Zeta1)
        } else if let Some(version_str) = s.strip_prefix("zeta2:") {
            let version = ZetaVersion::parse(version_str).map_err(serde::de::Error::custom)?;
            Ok(LoggedPredictionProvider::Zeta2(version))
        } else if s == "zeta2" {
            Ok(LoggedPredictionProvider::Zeta2(ZetaVersion::default()))
        } else {
            Err(serde::de::Error::custom(format!("unknown provider: {s}")))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PredictionOutcome {
    Accepted,
    Rejected,
}

fn log_dir() -> PathBuf {
    paths::data_dir().join(LOGGING_SUBDIR)
}

fn log_file_path(project_name: &str) -> PathBuf {
    let sanitized_name: String = project_name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            c => c,
        })
        .collect();
    log_dir().join(format!("{sanitized_name}.jsonl"))
}

fn get_project_name(project: &Entity<Project>, cx: &App) -> String {
    let names: Vec<_> = project.read(cx).worktree_root_names(cx).collect();
    if names.is_empty() {
        "unknown_project".to_string()
    } else if names.len() == 1 {
        names[0].to_string()
    } else {
        names.join("_")
    }
}

#[allow(dead_code)]
pub fn should_compute_uncommitted_diff() -> bool {
    rand::random::<u16>().is_multiple_of(10)
}

fn write_example_to_file(example: &LoggedExample, file_path: &Path) -> Result<()> {
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let json = serde_json::to_string(example)?;
    writeln!(file, "{json}")?;

    Ok(())
}

fn generate_timestamp_name() -> String {
    let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]");
    match format {
        Ok(format) => {
            let now = time::OffsetDateTime::now_local()
                .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
            now.format(&format)
                .unwrap_or_else(|_| "unknown-time".to_string())
        }
        Err(_) => "unknown-time".to_string(),
    }
}

pub fn log_prediction_outcome(
    project: &Entity<Project>,
    prediction: &EditPrediction,
    outcome: PredictionOutcome,
    events: Vec<StoredEvent>,
    provider: LoggedPredictionProvider,
    cx: &App,
) {
    let project_name = get_project_name(project, cx);
    let file_path = log_file_path(&project_name);

    let snapshot = &prediction.snapshot;
    let file = match snapshot.file() {
        Some(f) => f,
        None => {
            log::debug!("Skipping prediction logging: no file associated with buffer");
            return;
        }
    };

    let cursor_path: Arc<Path> = file.path().as_std_path().into();

    let inputs = &prediction.inputs;
    let cursor_offset_in_excerpt = inputs.cursor_offset_in_excerpt;

    let excerpt_start_offset = {
        let full_content = snapshot.text();
        let excerpt = &inputs.cursor_excerpt;
        full_content.find(excerpt.as_ref()).unwrap_or(0)
    };
    let full_cursor_offset = excerpt_start_offset + cursor_offset_in_excerpt;
    let cursor_point = full_cursor_offset.to_point(snapshot);

    let edit_history_string = build_edit_history_string(&events);

    let unified_diff = prediction
        .edit_preview
        .as_unified_diff(snapshot.file(), &prediction.edits);

    let mut spec = ExampleSpec {
        name: generate_timestamp_name(),
        repository_url: String::new(),
        revision: String::new(),
        tags: Vec::new(),
        reasoning: None,
        uncommitted_diff: String::new(),
        cursor_path,
        cursor_position: String::new(),
        edit_history: edit_history_string,
        expected_patches: Vec::new(),
        rejected_patch: None,
    };

    let line_comment_prefix = snapshot
        .language()
        .and_then(|lang| lang.config().line_comments.first())
        .map(|s| s.to_string())
        .unwrap_or_default();

    spec.set_cursor_excerpt(
        &inputs.cursor_excerpt,
        inputs.cursor_offset_in_excerpt,
        &line_comment_prefix,
    );

    match outcome {
        PredictionOutcome::Accepted => {
            if let Some(diff) = unified_diff.clone() {
                spec.expected_patches.push(diff);
            }
        }
        PredictionOutcome::Rejected => {
            spec.rejected_patch = unified_diff.clone();
        }
    }

    let prompt_inputs = LoggedPromptInputs {
        content: snapshot.text(),
        cursor_row: cursor_point.row,
        cursor_column: cursor_point.column,
        cursor_offset: full_cursor_offset,
        edit_history: inputs.events.clone(),
        related_files: if inputs.related_files.is_empty() {
            None
        } else {
            Some(inputs.related_files.clone())
        },
    };

    let prompt = match provider {
        LoggedPredictionProvider::Zeta2(version) => {
            let prompt_text = format_zeta_prompt(inputs, version);
            Some(LoggedPrompt {
                input: prompt_text,
                expected_output: String::new(),
                provider,
            })
        }
        LoggedPredictionProvider::Zeta1 => None,
    };

    let predictions = unified_diff
        .map(|diff| {
            vec![LoggedPrediction {
                actual_patch: Some(diff),
                actual_output: inputs.cursor_excerpt.to_string(),
                provider,
            }]
        })
        .unwrap_or_default();

    let example = LoggedExample {
        spec,
        prompt_inputs: Some(prompt_inputs),
        prompt,
        predictions,
    };

    if let Err(err) = write_example_to_file(&example, &file_path) {
        log::error!("Failed to write edit prediction log: {err}");
    }
}

fn build_edit_history_string(events: &[StoredEvent]) -> String {
    let mut edit_history = String::new();
    for stored_event in events {
        let zeta_prompt::Event::BufferChange {
            path,
            old_path,
            diff,
            ..
        } = stored_event.event.as_ref();

        edit_history.push_str("--- a");
        edit_history.push_str(&path.to_string_lossy());
        edit_history.push_str("\n+++ b");
        edit_history.push_str(&old_path.to_string_lossy());
        edit_history.push('\n');
        edit_history.push_str(diff);
        if !edit_history.ends_with('\n') {
            edit_history.push('\n');
        }
    }
    edit_history
}
