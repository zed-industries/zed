use serde::{Deserialize, Serialize};
use std::ops::Range;

use crate::PredictEditsGitInfo;

// TODO: snippet ordering within file / relative to excerpt

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictEditsRequest {
    pub excerpt: String,
    /// Within `signatures`
    pub excerpt_parent: Option<usize>,
    pub signatures: Vec<Signature>,
    pub referenced_declarations: Vec<ReferencedDeclaration>,
    pub events: Vec<Event>,
    #[serde(default)]
    pub can_collect_data: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub diagnostic_groups: Vec<DiagnosticGroup>,
    /// Info about the git repository state, only present when can_collect_data is true.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub git_info: Option<PredictEditsGitInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub text: String,
    pub text_is_truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent_index: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencedDeclaration {
    pub text: String,
    pub text_is_truncated: bool,
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
pub struct DiagnosticGroup {
    pub language_server: String,
    pub diagnostic_group: serde_json::Value,
}

/*
#[derive(Debug, Clone)]
pub struct SerializedJson<T> {
    raw: Box<RawValue>,
    _phantom: PhantomData<T>,
}

impl<T> SerializedJson<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(value: &T) -> Result<Self, serde_json::Error> {
        Ok(SerializedJson {
            raw: serde_json::value::to_raw_value(value)?,
            _phantom: PhantomData,
        })
    }

    pub fn deserialize(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(self.raw.get())
    }

    pub fn as_raw(&self) -> &RawValue {
        &self.raw
    }

    pub fn into_raw(self) -> Box<RawValue> {
        self.raw
    }
}

impl<T> Serialize for SerializedJson<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.raw.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for SerializedJson<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        Ok(SerializedJson {
            raw,
            _phantom: PhantomData,
        })
    }
}
*/
