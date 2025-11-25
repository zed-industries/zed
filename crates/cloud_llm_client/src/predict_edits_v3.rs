use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Write as _},
    ops::{Add, Range, Sub},
    path::Path,
    sync::Arc,
};
use strum::EnumIter;
use uuid::Uuid;

use crate::PredictEditsGitInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanContextRetrievalRequest {
    pub excerpt: String,
    pub excerpt_path: Arc<Path>,
    pub excerpt_line_range: Range<Line>,
    pub cursor_file_max_row: Line,
    pub events: Vec<Arc<Event>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictEditsRequest {
    pub excerpt: String,
    pub excerpt_path: Arc<Path>,
    /// Within file
    pub excerpt_range: Range<usize>,
    pub excerpt_line_range: Range<Line>,
    pub cursor_point: Point,
    /// Within `signatures`
    pub excerpt_parent: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub included_files: Vec<IncludedFile>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub signatures: Vec<Signature>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub referenced_declarations: Vec<ReferencedDeclaration>,
    pub events: Vec<Arc<Event>>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncludedFile {
    pub path: Arc<Path>,
    pub max_row: Line,
    pub excerpts: Vec<Excerpt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Excerpt {
    pub start_line: Line,
    pub text: Arc<str>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum PromptFormat {
    MarkedExcerpt,
    LabeledSections,
    NumLinesUniDiff,
    OldTextNewText,
    /// Prompt format intended for use via zeta_cli
    OnlySnippets,
    /// One-sentence instructions used in fine-tuned models
    Minimal,
    /// One-sentence instructions + FIM-like template
    MinimalQwen,
    /// No instructions, Qwen chat + Seed-Coder 1120 FIM-like template
    SeedCoder1120,
}

impl PromptFormat {
    pub const DEFAULT: PromptFormat = PromptFormat::NumLinesUniDiff;
}

impl Default for PromptFormat {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl PromptFormat {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}

impl std::fmt::Display for PromptFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptFormat::MarkedExcerpt => write!(f, "Marked Excerpt"),
            PromptFormat::LabeledSections => write!(f, "Labeled Sections"),
            PromptFormat::OnlySnippets => write!(f, "Only Snippets"),
            PromptFormat::NumLinesUniDiff => write!(f, "Numbered Lines / Unified Diff"),
            PromptFormat::OldTextNewText => write!(f, "Old Text / New Text"),
            PromptFormat::Minimal => write!(f, "Minimal"),
            PromptFormat::MinimalQwen => write!(f, "Minimal + Qwen FIM"),
            PromptFormat::SeedCoder1120 => write!(f, "Seed-Coder 1120"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "test-support"), derive(PartialEq))]
#[serde(tag = "event")]
pub enum Event {
    BufferChange {
        path: Arc<Path>,
        old_path: Arc<Path>,
        diff: String,
        predicted: bool,
        in_open_source_repo: bool,
    },
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::BufferChange {
                path,
                old_path,
                diff,
                predicted,
                ..
            } => {
                if *predicted {
                    write!(
                        f,
                        "// User accepted prediction:\n--- a/{}\n+++ b/{}\n{diff}",
                        DiffPathFmt(old_path),
                        DiffPathFmt(path)
                    )
                } else {
                    write!(
                        f,
                        "--- a/{}\n+++ b/{}\n{diff}",
                        DiffPathFmt(old_path),
                        DiffPathFmt(path)
                    )
                }
            }
        }
    }
}

/// always format the Path as a unix path with `/` as the path sep in Diffs
pub struct DiffPathFmt<'a>(pub &'a Path);

impl<'a> std::fmt::Display for DiffPathFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        for component in self.0.components() {
            if !is_first {
                f.write_char('/')?;
            } else {
                is_first = false;
            }
            write!(f, "{}", component.as_os_str().display())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub text: String,
    pub text_is_truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent_index: Option<usize>,
    /// Range of `text` within the file, possibly truncated according to `text_is_truncated`. The
    /// file is implicitly the file that contains the descendant declaration or excerpt.
    pub range: Range<Line>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencedDeclaration {
    pub path: Arc<Path>,
    pub text: String,
    pub text_is_truncated: bool,
    /// Range of `text` within file, possibly truncated according to `text_is_truncated`
    pub range: Range<Line>,
    /// Range within `text`
    pub signature_range: Range<usize>,
    /// Index within `signatures`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent_index: Option<usize>,
    pub score_components: DeclarationScoreComponents,
    pub signature_score: f32,
    pub declaration_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclarationScoreComponents {
    pub is_same_file: bool,
    pub is_referenced_nearby: bool,
    pub is_referenced_in_breadcrumb: bool,
    pub reference_count: usize,
    pub same_file_declaration_count: usize,
    pub declaration_count: usize,
    pub reference_line_distance: u32,
    pub declaration_line_distance: u32,
    pub excerpt_vs_item_jaccard: f32,
    pub excerpt_vs_signature_jaccard: f32,
    pub adjacent_vs_item_jaccard: f32,
    pub adjacent_vs_signature_jaccard: f32,
    pub excerpt_vs_item_weighted_overlap: f32,
    pub excerpt_vs_signature_weighted_overlap: f32,
    pub adjacent_vs_item_weighted_overlap: f32,
    pub adjacent_vs_signature_weighted_overlap: f32,
    pub path_import_match_count: usize,
    pub wildcard_path_import_match_count: usize,
    pub import_similarity: f32,
    pub max_import_similarity: f32,
    pub normalized_import_similarity: f32,
    pub wildcard_import_similarity: f32,
    pub normalized_wildcard_import_similarity: f32,
    pub included_by_others: usize,
    pub includes_others: usize,
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
    pub range: Range<Line>,
    pub content: String,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point {
    pub line: Line,
    pub column: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
#[serde(transparent)]
pub struct Line(pub u32);

impl Add for Line {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Line {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_event_display() {
        let ev = Event::BufferChange {
            path: Path::new("untitled").into(),
            old_path: Path::new("untitled").into(),
            diff: "@@ -1,2 +1,2 @@\n-a\n-b\n".into(),
            predicted: false,
            in_open_source_repo: true,
        };
        assert_eq!(
            ev.to_string(),
            indoc! {"
                --- a/untitled
                +++ b/untitled
                @@ -1,2 +1,2 @@
                -a
                -b
            "}
        );

        let ev = Event::BufferChange {
            path: Path::new("foo/bar.txt").into(),
            old_path: Path::new("foo/bar.txt").into(),
            diff: "@@ -1,2 +1,2 @@\n-a\n-b\n".into(),
            predicted: false,
            in_open_source_repo: true,
        };
        assert_eq!(
            ev.to_string(),
            indoc! {"
                --- a/foo/bar.txt
                +++ b/foo/bar.txt
                @@ -1,2 +1,2 @@
                -a
                -b
            "}
        );

        let ev = Event::BufferChange {
            path: Path::new("abc.txt").into(),
            old_path: Path::new("123.txt").into(),
            diff: "@@ -1,2 +1,2 @@\n-a\n-b\n".into(),
            predicted: false,
            in_open_source_repo: true,
        };
        assert_eq!(
            ev.to_string(),
            indoc! {"
                --- a/123.txt
                +++ b/abc.txt
                @@ -1,2 +1,2 @@
                -a
                -b
            "}
        );

        let ev = Event::BufferChange {
            path: Path::new("abc.txt").into(),
            old_path: Path::new("123.txt").into(),
            diff: "@@ -1,2 +1,2 @@\n-a\n-b\n".into(),
            predicted: true,
            in_open_source_repo: true,
        };
        assert_eq!(
            ev.to_string(),
            indoc! {"
                // User accepted prediction:
                --- a/123.txt
                +++ b/abc.txt
                @@ -1,2 +1,2 @@
                -a
                -b
            "}
        );
    }
}
