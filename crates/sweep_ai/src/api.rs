use std::{path::Path, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct AutocompleteRequest {
    pub debug_info: String,
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
