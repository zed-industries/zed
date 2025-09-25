use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

use crate::PredictEditsGitInfo;

// TODO: snippet ordering within file / relative to excerpt

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictEditsRequest {
    pub excerpt: String,
    pub excerpt_path: Arc<Path>,
    /// Within file
    pub excerpt_range: Range<usize>,
    /// Within `excerpt`
    pub cursor_offset: usize,
    /// Within `signatures`
    pub excerpt_parent: Option<usize>,
    pub signatures: Vec<Signature>,
    pub referenced_declarations: Vec<ReferencedDeclaration>,
    pub events: Vec<Event>,
    #[serde(default)]
    pub can_collect_data: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub diagnostic_groups: Vec<DiagnosticGroup>,
    #[serde(skip_serializing_if = "is_default", default)]
    pub diagnostic_groups_truncated: bool,
    /// Info about the git repository state, only present when can_collect_data is true.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub git_info: Option<PredictEditsGitInfo>,
    // Only available to staff
    #[serde(default)]
    pub debug_info: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub prompt_max_bytes: Option<usize>,
    #[serde(default)]
    pub prompt_format: PromptFormat,
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PromptFormat {
    #[default]
    MarkedExcerpt,
    LabeledSections,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Event {
    BufferChange {
        path: Option<PathBuf>,
        old_path: Option<PathBuf>,
        diff: String,
        predicted: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub text: String,
    pub text_is_truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent_index: Option<usize>,
    /// Range of `text` within the file, possibly truncated according to `text_is_truncated`. The
    /// file is implicitly the file that contains the descendant declaration or excerpt.
    pub range: Range<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencedDeclaration {
    pub path: Arc<Path>,
    pub text: String,
    pub text_is_truncated: bool,
    /// Range of `text` within file, possibly truncated according to `text_is_truncated`
    pub range: Range<usize>,
    /// Range within `text`
    pub signature_range: Range<usize>,
    /// Index within `signatures`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent_index: Option<usize>,
    pub score_components: ScoreComponents,
    pub signature_score: f32,
    pub declaration_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreComponents {
    pub is_same_file: bool,
    pub is_referenced_nearby: bool,
    pub is_referenced_in_breadcrumb: bool,
    pub reference_count: usize,
    pub same_file_declaration_count: usize,
    pub declaration_count: usize,
    pub reference_line_distance: u32,
    pub declaration_line_distance: u32,
    pub declaration_line_distance_rank: usize,
    pub containing_range_vs_item_jaccard: f32,
    pub containing_range_vs_signature_jaccard: f32,
    pub adjacent_vs_item_jaccard: f32,
    pub adjacent_vs_signature_jaccard: f32,
    pub containing_range_vs_item_weighted_overlap: f32,
    pub containing_range_vs_signature_weighted_overlap: f32,
    pub adjacent_vs_item_weighted_overlap: f32,
    pub adjacent_vs_signature_weighted_overlap: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiagnosticGroup(pub Box<serde_json::value::RawValue>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictEditsResponse {
    pub request_id: Uuid,
    pub edits: Vec<Edit>,
    pub debug_info: Option<DebugInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfo {
    pub prompt: String,
    pub prompt_planning_time: Duration,
    pub model_response: String,
    pub inference_time: Duration,
    pub parsing_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edit {
    pub path: Arc<Path>,
    pub range: Range<usize>,
    pub content: String,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
