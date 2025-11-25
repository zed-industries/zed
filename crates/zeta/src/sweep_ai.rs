use std::fmt;
use std::{path::Path, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct AutocompleteRequest {
    pub debug_info: Arc<str>,
    pub repo_name: String,
    pub branch: Option<String>,
    pub file_path: Arc<Path>,
    pub file_contents: String,
    pub recent_changes: String,
    pub cursor_position: usize,
    pub original_file_contents: String,
    pub file_chunks: Vec<FileChunk>,
    pub retrieval_chunks: Vec<RetrievalChunk>,
    pub recent_user_actions: Vec<UserAction>,
    pub multiple_suggestions: bool,
    pub privacy_mode_enabled: bool,
    pub changes_above_cursor: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileChunk {
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RetrievalChunk {
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserAction {
    pub action_type: ActionType,
    pub line_number: usize,
    pub offset: usize,
    pub file_path: String,
    pub timestamp: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionType {
    CursorMovement,
    InsertChar,
    DeleteChar,
    InsertSelection,
    DeleteSelection,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AutocompleteResponse {
    pub autocomplete_id: String,
    pub start_index: usize,
    pub end_index: usize,
    pub completion: String,
    #[allow(dead_code)]
    pub confidence: f64,
    #[allow(dead_code)]
    pub logprobs: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub finish_reason: Option<String>,
    #[allow(dead_code)]
    pub elapsed_time_ms: u64,
    #[allow(dead_code)]
    #[serde(default, rename = "completions")]
    pub additional_completions: Vec<AdditionalCompletion>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct AdditionalCompletion {
    pub start_index: usize,
    pub end_index: usize,
    pub completion: String,
    pub confidence: f64,
    pub autocomplete_id: String,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: Option<String>,
}

pub(crate) fn write_event(
    event: &cloud_llm_client::predict_edits_v3::Event,
    f: &mut impl fmt::Write,
) -> fmt::Result {
    match event {
        cloud_llm_client::predict_edits_v3::Event::BufferChange {
            old_path,
            path,
            diff,
            ..
        } => {
            if old_path != path {
                // TODO confirm how to do this for sweep
                // writeln!(f, "User renamed {:?} to {:?}\n", old_path, new_path)?;
            }

            if !diff.is_empty() {
                write!(f, "File: {}:\n{}\n", path.display(), diff)?
            }

            fmt::Result::Ok(())
        }
    }
}

pub(crate) fn debug_info(cx: &gpui::App) -> Arc<str> {
    format!(
        "Zed v{version} ({sha}) - OS: {os} - Zed v{version}",
        version = release_channel::AppVersion::global(cx),
        sha = release_channel::AppCommitSha::try_global(cx)
            .map_or("unknown".to_string(), |sha| sha.full()),
        os = client::telemetry::os_name(),
    )
    .into()
}
