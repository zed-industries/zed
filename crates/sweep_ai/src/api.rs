use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct AutocompleteRequest {
    pub debug_info: String,
    pub device_id: String,
    pub repo_name: String,
    pub branch: Option<String>,
    pub file_path: PathBuf,
    pub file_contents: String,
    pub recent_changes: String,
    pub cursor_position: usize,
    pub original_file_contents: String,
    pub file_chunks: Vec<FileChunk>,
    pub retrieval_chunks: Vec<RetrievalChunk>,
    pub recent_user_actions: Vec<UserAction>,
    pub multiple_suggestions: bool,
    pub privacy_mode_enabled: bool,
    pub client_ip: Option<String>,
    pub recent_changes_high_res: String,
    pub changes_above_cursor: bool,
    pub ping: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileChunk {
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub timestamp: u64,
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
    pub start_index: usize,
    pub end_index: usize,
    pub completion: String,
    pub confidence: f64,
    pub autocomplete_id: String,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: Option<String>,
    pub elapsed_time_ms: u64,
    // pub completions: Vec<Completion>,
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct Completion {
//     pub start_index: usize,
//     pub end_index: usize,
//     pub completion: String,
//     pub confidence: f64,
//     pub autocomplete_id: String,
//     pub logprobs: Option<serde_json::Value>,
//     pub finish_reason: Option<String>,
// }
