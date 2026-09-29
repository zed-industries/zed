use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use zeta_prompt::udiff::{apply_diff_to_string, apply_diff_to_string_with_hunk_offset};

use crate::reversal::compute_prediction_reversal_ratio_from_history;
use crate::{
    jumps::{
        EditableContextCoverage, Excerpt, PatchLocationMatch, editable_context_coverage,
        patch_location_match,
    },
    patch::{Hunk, Patch, PatchLine},
    patch_metrics::{
        ClassificationMetrics, DeltaChrFMetrics, braces_disbalance, count_patch_token_changes,
        delta_chr_f, delta_chr_f_beta, exact_lines_match, has_isolated_whitespace_changes,
        is_editable_region_correct,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PredictionScore {
    pub delta_chr_f: f32,
    #[serde(default)]
    pub delta_chr_f_true_positives: usize,
    #[serde(default)]
    pub delta_chr_f_false_positives: usize,
    #[serde(default)]
    pub delta_chr_f_false_negatives: usize,
    #[serde(default)]
    pub delta_chr_f_precision: f64,
    #[serde(default)]
    pub delta_chr_f_recall: f64,
    #[serde(default)]
    pub delta_chr_f_beta: f64,
    pub braces_disbalance: usize,
    #[serde(default)]
    pub exact_lines_tp: usize,
    #[serde(default)]
    pub exact_lines_fp: usize,
    #[serde(default)]
    pub exact_lines_fn: usize,
    #[serde(default)]
    pub reversal_ratio: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor_distance: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor_exact_match: Option<bool>,
    pub wrong_editable_region: Option<bool>,
    #[serde(default)]
    pub has_isolated_whitespace_changes: bool,
    #[serde(default)]
    pub inserted_tokens: usize,
    #[serde(default)]
    pub deleted_tokens: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept_rate: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recall_rate: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept_chars: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correctly_deleted_chars: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discarded_chars: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cumulative_logprob: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_logprob: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable_context_coverage: Option<EditableContextCoverage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump_location: Option<PatchLocationMatch>,
}

impl PredictionScore {
    pub fn zero() -> Self {
        Self {
            delta_chr_f: 0.0,
            delta_chr_f_true_positives: 0,
            delta_chr_f_false_positives: 0,
            delta_chr_f_false_negatives: 0,
            delta_chr_f_precision: 0.0,
            delta_chr_f_recall: 0.0,
            delta_chr_f_beta: delta_chr_f_beta(),
            braces_disbalance: 0,
            exact_lines_tp: 0,
            exact_lines_fp: 0,
            exact_lines_fn: 0,
            reversal_ratio: 0.0,
            cursor_distance: None,
            cursor_exact_match: None,
            wrong_editable_region: None,
            has_isolated_whitespace_changes: false,
            inserted_tokens: 0,
            deleted_tokens: 0,
            kept_rate: None,
            recall_rate: None,
            kept_chars: None,
            correctly_deleted_chars: None,
            discarded_chars: None,
            cumulative_logprob: None,
            avg_logprob: None,
            editable_context_coverage: None,
            jump_location: None,
        }
    }

    pub fn delta_chr_f_counts(&self) -> ClassificationMetrics {
        ClassificationMetrics {
            true_positives: self.delta_chr_f_true_positives,
            false_positives: self.delta_chr_f_false_positives,
            false_negatives: self.delta_chr_f_false_negatives,
        }
    }

    pub fn exact_lines_counts(&self) -> ClassificationMetrics {
        ClassificationMetrics {
            true_positives: self.exact_lines_tp,
            false_positives: self.exact_lines_fp,
            false_negatives: self.exact_lines_fn,
        }
    }
}

impl Default for PredictionScore {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Clone, Debug)]
pub struct PreparedExpectedPatch {
    pub patch: String,
    pub text: String,
    pub cursor_editable_region_offset: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct PrepareExpectedPatchError {
    message: String,
}

impl fmt::Display for PrepareExpectedPatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

impl Error for PrepareExpectedPatchError {}

pub fn prepare_expected_patches(
    expected_patches_with_cursors: &[(String, Option<usize>)],
    original_text: &str,
    old_editable_region: Option<&str>,
) -> Result<Vec<PreparedExpectedPatch>, PrepareExpectedPatchError> {
    expected_patches_with_cursors
        .iter()
        .map(|(patch, cursor_in_patch)| {
            let text = apply_diff_to_string(patch, original_text).map_err(|error| {
                PrepareExpectedPatchError {
                    message: error.to_string(),
                }
            })?;
            let cursor_editable_region_offset =
                if let (Some(editable_region), Some(cursor_in_patch)) =
                    (old_editable_region, *cursor_in_patch)
                {
                    match apply_diff_to_string_with_hunk_offset(patch, editable_region) {
                        Ok((_, hunk_offset)) => Some(hunk_offset.unwrap_or(0) + cursor_in_patch),
                        Err(_) => None,
                    }
                } else {
                    *cursor_in_patch
                };

            Ok(PreparedExpectedPatch {
                patch: patch.clone(),
                text,
                cursor_editable_region_offset,
            })
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
pub struct ActualPredictionCursor {
    pub row: u32,
    pub editable_region_offset: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct PredictionReversalContext<'a> {
    pub edit_history: &'a [Arc<zeta_prompt::Event>],
    pub excerpt_start_row: Option<u32>,
    pub cursor_path: &'a Path,
}

#[derive(Clone, Copy, Debug)]
pub struct PredictionScoringInput<'a> {
    pub original_text: &'a str,
    pub expected_patches: &'a [PreparedExpectedPatch],
    pub actual_patch: Option<&'a str>,
    pub actual_cursor: Option<ActualPredictionCursor>,
    pub reversal_context: Option<PredictionReversalContext<'a>>,
    pub cumulative_logprob: Option<f64>,
    pub avg_logprob: Option<f64>,
    pub context: Option<&'a [Excerpt]>,
}

pub fn score_prediction(input: PredictionScoringInput<'_>) -> PredictionScore {
    let editable_context_coverage = input.context.and_then(|context| {
        input
            .expected_patches
            .iter()
            .map(|expected| editable_context_coverage(&expected.patch, context))
            .max_by(|left, right| {
                left.lines_f1
                    .total_cmp(&right.lines_f1)
                    .then_with(|| left.files_f1.total_cmp(&right.files_f1))
            })
    });

    let actual_patch = input.actual_patch.unwrap_or("");
    let token_changes = count_patch_token_changes(actual_patch);

    let mut best = input
        .expected_patches
        .iter()
        .map(|expected| score_against_expected_patch(input, expected, actual_patch))
        .max_by(|left, right| {
            left.delta_chr_f_metrics
                .score
                .total_cmp(&right.delta_chr_f_metrics.score)
                .then_with(|| left.exact_lines.f1().total_cmp(&right.exact_lines.f1()))
                .then_with(|| {
                    left.jump_location
                        .lines_f1
                        .total_cmp(&right.jump_location.lines_f1)
                })
        })
        .unwrap_or_else(|| score_against_no_expected_patch(input, actual_patch));

    let (cursor_distance, cursor_exact_match) =
        compute_cursor_metrics(best.expected_cursor, input.actual_cursor);

    let wrong_editable_region = input
        .actual_patch
        .map(|actual_patch| !is_editable_region_correct(actual_patch));
    let has_isolated_whitespace_changes = input.actual_patch.is_some_and(|actual_patch| {
        has_isolated_whitespace_changes(actual_patch, input.actual_cursor.map(|cursor| cursor.row))
    });

    best.score.cumulative_logprob = input.cumulative_logprob;
    best.score.avg_logprob = input.avg_logprob;
    best.score.editable_context_coverage = editable_context_coverage;
    best.score.inserted_tokens = token_changes.inserted_tokens;
    best.score.deleted_tokens = token_changes.deleted_tokens;
    best.score.cursor_distance = cursor_distance;
    best.score.cursor_exact_match = cursor_exact_match;
    best.score.wrong_editable_region = wrong_editable_region;
    best.score.has_isolated_whitespace_changes = has_isolated_whitespace_changes;
    best.score
}

struct ExpectedPatchScore {
    score: PredictionScore,
    delta_chr_f_metrics: DeltaChrFMetrics,
    exact_lines: ClassificationMetrics,
    jump_location: PatchLocationMatch,
    expected_cursor: Option<usize>,
}

struct ContentScore {
    delta_chr_f_metrics: DeltaChrFMetrics,
    braces_disbalance: usize,
    reversal_ratio: f32,
    kept_rate: Option<f64>,
    recall_rate: Option<f64>,
    kept_chars: Option<usize>,
    correctly_deleted_chars: Option<usize>,
    discarded_chars: Option<usize>,
}

fn score_against_expected_patch(
    input: PredictionScoringInput<'_>,
    expected: &PreparedExpectedPatch,
    actual_patch: &str,
) -> ExpectedPatchScore {
    let exact_lines = exact_lines_match(&expected.patch, actual_patch);
    let jump_location = patch_location_match(&expected.patch, actual_patch);
    let content_score = if let Some(context) = input.context {
        score_content_on_context(input, expected, actual_patch, context).unwrap_or_else(|| {
            if expected.patch.trim().is_empty() && actual_patch.trim().is_empty() {
                score_content_on_cursor_excerpt(input, expected, actual_patch)
            } else {
                zero_content_score()
            }
        })
    } else {
        score_content_on_cursor_excerpt(input, expected, actual_patch)
    };
    let delta_chr_f_metrics = content_score.delta_chr_f_metrics.clone();

    let score = PredictionScore {
        delta_chr_f: delta_chr_f_metrics.score as f32,
        delta_chr_f_true_positives: delta_chr_f_metrics.counts.true_positives,
        delta_chr_f_false_positives: delta_chr_f_metrics.counts.false_positives,
        delta_chr_f_false_negatives: delta_chr_f_metrics.counts.false_negatives,
        delta_chr_f_precision: delta_chr_f_metrics.precision,
        delta_chr_f_recall: delta_chr_f_metrics.recall,
        delta_chr_f_beta: delta_chr_f_metrics.beta,
        braces_disbalance: content_score.braces_disbalance,
        exact_lines_tp: exact_lines.true_positives,
        exact_lines_fp: exact_lines.false_positives,
        exact_lines_fn: exact_lines.false_negatives,
        reversal_ratio: content_score.reversal_ratio,
        cursor_distance: None,
        cursor_exact_match: None,
        wrong_editable_region: None,
        has_isolated_whitespace_changes: false,
        inserted_tokens: 0,
        deleted_tokens: 0,
        kept_rate: content_score.kept_rate,
        recall_rate: content_score.recall_rate,
        kept_chars: content_score.kept_chars,
        correctly_deleted_chars: content_score.correctly_deleted_chars,
        discarded_chars: content_score.discarded_chars,
        cumulative_logprob: None,
        avg_logprob: None,
        editable_context_coverage: None,
        jump_location: Some(jump_location.clone()),
    };

    ExpectedPatchScore {
        score,
        delta_chr_f_metrics,
        exact_lines,
        jump_location,
        expected_cursor: expected.cursor_editable_region_offset,
    }
}

fn score_against_no_expected_patch(
    input: PredictionScoringInput<'_>,
    actual_patch: &str,
) -> ExpectedPatchScore {
    let expected = PreparedExpectedPatch {
        patch: String::new(),
        text: input.original_text.to_string(),
        cursor_editable_region_offset: None,
    };
    score_against_expected_patch(input, &expected, actual_patch)
}

fn zero_content_score() -> ContentScore {
    ContentScore {
        delta_chr_f_metrics: DeltaChrFMetrics {
            beta: delta_chr_f_beta(),
            ..DeltaChrFMetrics::default()
        },
        braces_disbalance: 0,
        reversal_ratio: 0.0,
        kept_rate: None,
        recall_rate: None,
        kept_chars: None,
        correctly_deleted_chars: None,
        discarded_chars: None,
    }
}

fn score_content_on_cursor_excerpt(
    input: PredictionScoringInput<'_>,
    expected: &PreparedExpectedPatch,
    actual_patch: &str,
) -> ContentScore {
    let actual_text = apply_diff_to_string(actual_patch, input.original_text)
        .unwrap_or_else(|_| input.original_text.to_string());
    let delta_chr_f_metrics = delta_chr_f(input.original_text, &expected.text, &actual_text);
    let braces_disbalance =
        braces_disbalance(&actual_text).saturating_sub(braces_disbalance(input.original_text));
    let reversal_ratio = input
        .reversal_context
        .map(|context| {
            compute_prediction_reversal_ratio_from_history(
                input.original_text,
                context.edit_history,
                context.excerpt_start_row,
                &actual_text,
                context.cursor_path,
            )
        })
        .unwrap_or(0.0);
    let kept_rate =
        crate::kept_rate::compute_kept_rate(input.original_text, &actual_text, &expected.text);

    ContentScore {
        delta_chr_f_metrics,
        braces_disbalance,
        reversal_ratio,
        kept_rate: Some(kept_rate.kept_rate),
        recall_rate: Some(kept_rate.recall_rate),
        kept_chars: Some(kept_rate.kept_chars),
        correctly_deleted_chars: Some(kept_rate.correctly_deleted_chars),
        discarded_chars: Some(kept_rate.discarded_chars),
    }
}

fn score_content_on_context(
    input: PredictionScoringInput<'_>,
    expected: &PreparedExpectedPatch,
    actual_patch: &str,
    context: &[Excerpt],
) -> Option<ContentScore> {
    let expected_documents = apply_patch_to_documents(&expected.patch, context);
    let actual_documents = apply_patch_to_documents(actual_patch, context);
    let document_indices = expected_documents
        .keys()
        .chain(actual_documents.keys())
        .copied()
        .collect::<BTreeSet<_>>();

    if document_indices.is_empty() {
        return None;
    }

    let mut original_text = String::new();
    let mut expected_text = String::new();
    let mut actual_text = String::new();
    let mut braces_disbalance_before = 0;
    let mut braces_disbalance_after = 0;
    let mut cursor_actual_text = None;

    for document_ix in document_indices {
        let document = context.get(document_ix)?;
        let expected_document_text = expected_documents
            .get(&document_ix)
            .map(String::as_str)
            .unwrap_or(document.content.as_str());
        let actual_document_text = actual_documents
            .get(&document_ix)
            .map(String::as_str)
            .unwrap_or(document.content.as_str());

        if !original_text.is_empty() {
            original_text.push('\n');
            expected_text.push('\n');
            actual_text.push('\n');
        }
        original_text.push_str(&document.content);
        expected_text.push_str(expected_document_text);
        actual_text.push_str(actual_document_text);

        braces_disbalance_before += braces_disbalance(&document.content);
        braces_disbalance_after += braces_disbalance(actual_document_text);

        if input.reversal_context.is_some_and(|reversal_context| {
            path_matches(
                &reversal_context.cursor_path.to_string_lossy(),
                &document.path,
            )
        }) {
            cursor_actual_text = Some(actual_document_text.to_string());
        }
    }

    let delta_chr_f_metrics = delta_chr_f(&original_text, &expected_text, &actual_text);
    let kept_rate =
        crate::kept_rate::compute_kept_rate(&original_text, &actual_text, &expected_text);
    let reversal_ratio = if let (Some(reversal_context), Some(cursor_actual_text)) =
        (input.reversal_context, cursor_actual_text.as_deref())
    {
        compute_prediction_reversal_ratio_from_history(
            input.original_text,
            reversal_context.edit_history,
            reversal_context.excerpt_start_row,
            cursor_actual_text,
            reversal_context.cursor_path,
        )
    } else {
        0.0
    };

    Some(ContentScore {
        delta_chr_f_metrics,
        braces_disbalance: braces_disbalance_after.saturating_sub(braces_disbalance_before),
        reversal_ratio,
        kept_rate: Some(kept_rate.kept_rate),
        recall_rate: Some(kept_rate.recall_rate),
        kept_chars: Some(kept_rate.kept_chars),
        correctly_deleted_chars: Some(kept_rate.correctly_deleted_chars),
        discarded_chars: Some(kept_rate.discarded_chars),
    })
}

fn apply_patch_to_documents(patch: &str, context: &[Excerpt]) -> BTreeMap<usize, String> {
    let patch = Patch::parse_unified_diff(patch);
    let mut hunks_by_document: BTreeMap<usize, Vec<Hunk>> = BTreeMap::new();

    for hunk in patch.hunks.into_iter().filter(hunk_has_change) {
        if let Some(document_ix) = find_hunk_document(&hunk, context) {
            hunks_by_document.entry(document_ix).or_default().push(hunk);
        }
    }

    hunks_by_document
        .into_iter()
        .filter_map(|(document_ix, hunks)| {
            let document = context.get(document_ix)?;
            let document_patch = diff_for_document_hunks(document, &hunks);
            let text = apply_diff_to_string(&document_patch, &document.content).ok()?;
            Some((document_ix, text))
        })
        .collect()
}

fn find_hunk_document(hunk: &Hunk, context: &[Excerpt]) -> Option<usize> {
    context
        .iter()
        .enumerate()
        .find_map(|(document_ix, document)| {
            if !path_matches(&hunk.filename, &document.path) {
                return None;
            }

            let document_patch = diff_for_document_hunks(document, std::slice::from_ref(hunk));
            apply_diff_to_string(&document_patch, &document.content)
                .is_ok()
                .then_some(document_ix)
        })
}

fn diff_for_document_hunks(document: &Excerpt, hunks: &[Hunk]) -> String {
    let mut diff = String::new();
    diff.push_str(&format!("--- a/{}\n", document.path));
    diff.push_str(&format!("+++ b/{}\n", document.path));

    for hunk in hunks {
        let old_start = adjust_hunk_start(hunk.old_start, &document.row_range);
        let new_start = adjust_hunk_start(hunk.new_start, &document.row_range);
        let old_count = hunk
            .lines
            .iter()
            .filter(|line| matches!(line, PatchLine::Context(_) | PatchLine::Deletion(_)))
            .count();
        let new_count = hunk
            .lines
            .iter()
            .filter(|line| matches!(line, PatchLine::Context(_) | PatchLine::Addition(_)))
            .count();
        diff.push_str(&format!(
            "@@ -{},{} +{},{} @@\n",
            old_start, old_count, new_start, new_count
        ));
        for line in &hunk.lines {
            match line {
                PatchLine::Context(text) => {
                    diff.push(' ');
                    diff.push_str(text);
                    diff.push('\n');
                }
                PatchLine::Addition(text) => {
                    diff.push('+');
                    diff.push_str(text);
                    diff.push('\n');
                }
                PatchLine::Deletion(text) => {
                    diff.push('-');
                    diff.push_str(text);
                    diff.push('\n');
                }
                PatchLine::Garbage(text) => {
                    diff.push_str(text);
                    diff.push('\n');
                }
            }
        }
    }

    diff
}

fn adjust_hunk_start(start: isize, row_range: &std::ops::Range<u32>) -> isize {
    let Ok(start_row) = u32::try_from(start.saturating_sub(1)) else {
        return start;
    };

    if row_range.start <= start_row && start_row <= row_range.end {
        start.saturating_sub(row_range.start as isize)
    } else {
        start
    }
}

fn hunk_has_change(hunk: &Hunk) -> bool {
    hunk.lines
        .iter()
        .any(|line| matches!(line, PatchLine::Addition(_) | PatchLine::Deletion(_)))
}

fn path_matches(patch_path: &str, document_path: &str) -> bool {
    patch_path == document_path
        || strip_first_path_component(patch_path).is_some_and(|stripped| stripped == document_path)
}

fn strip_first_path_component(path: &str) -> Option<&str> {
    path.split_once('/')
        .map(|(_, rest)| rest)
        .filter(|rest| !rest.is_empty())
}

fn compute_cursor_metrics(
    expected_cursor_editable_region_offset: Option<usize>,
    actual_cursor: Option<ActualPredictionCursor>,
) -> (Option<usize>, Option<bool>) {
    match (expected_cursor_editable_region_offset, actual_cursor) {
        (Some(expected), Some(actual)) => {
            let distance = expected.abs_diff(actual.editable_region_offset.unwrap_or_default());
            let exact_match = distance == 0;
            (Some(distance), Some(exact_match))
        }
        (None, None) => (None, None),
        (Some(_), None) | (None, Some(_)) => (None, Some(false)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kept_rate_is_computed_when_best_delta_chr_f_score_is_zero() {
        let original_text = "";
        let actual_patch = "--- a/file.txt\n+++ b/file.txt\n@@ -0,0 +1 @@\n+bbbbbb\n";
        let expected_patch = "--- a/file.txt\n+++ b/file.txt\n@@ -0,0 +1 @@\n+cccccc\n";
        let expected_patches = [PreparedExpectedPatch {
            patch: expected_patch.to_string(),
            text: "cccccc".to_string(),
            cursor_editable_region_offset: None,
        }];

        let score = score_prediction(PredictionScoringInput {
            original_text,
            expected_patches: &expected_patches,
            actual_patch: Some(actual_patch),
            actual_cursor: None,
            reversal_context: None,
            cumulative_logprob: None,
            avg_logprob: None,
            context: None,
        });

        assert_eq!(score.delta_chr_f, 0.0);
        assert_eq!(score.kept_rate, Some(0.0));
    }

    #[test]
    fn test_scores_related_file_patch_against_context_document() {
        let original_text = "fn main() {}\n";
        let expected_patch = "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -11,3 +11,3 @@\n fn value() {\n-    1\n+    2\n }\n";
        let actual_patch = "--- a/project/src/lib.rs\n+++ b/project/src/lib.rs\n@@ -11,3 +11,3 @@\n fn value() {\n-    1\n+    2\n }\n";
        let expected_patches = [PreparedExpectedPatch {
            patch: expected_patch.to_string(),
            text: original_text.to_string(),
            cursor_editable_region_offset: None,
        }];
        let context = [
            Excerpt {
                path: "src/main.rs".to_string(),
                row_range: 0..1,
                content: original_text.to_string(),
            },
            Excerpt {
                path: "src/lib.rs".to_string(),
                row_range: 10..13,
                content: "fn value() {\n    1\n}\n".to_string(),
            },
        ];

        let score = score_prediction(PredictionScoringInput {
            original_text,
            expected_patches: &expected_patches,
            actual_patch: Some(actual_patch),
            actual_cursor: None,
            reversal_context: None,
            cumulative_logprob: None,
            avg_logprob: None,
            context: Some(&context),
        });

        assert_eq!(score.delta_chr_f, 100.0);
        assert_eq!(score.exact_lines_tp, 2);
        assert_eq!(score.jump_location.unwrap().files_f1, 1.0);
    }

    #[test]
    fn test_missing_related_file_prediction_counts_as_false_negative() {
        let original_text = "fn main() {}\n";
        let expected_patch = "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -11,3 +11,3 @@\n fn value() {\n-    1\n+    2\n }\n";
        let expected_patches = [PreparedExpectedPatch {
            patch: expected_patch.to_string(),
            text: original_text.to_string(),
            cursor_editable_region_offset: None,
        }];
        let context = [Excerpt {
            path: "src/lib.rs".to_string(),
            row_range: 10..13,
            content: "fn value() {\n    1\n}\n".to_string(),
        }];

        let score = score_prediction(PredictionScoringInput {
            original_text,
            expected_patches: &expected_patches,
            actual_patch: None,
            actual_cursor: None,
            reversal_context: None,
            cumulative_logprob: None,
            avg_logprob: None,
            context: Some(&context),
        });

        assert!(score.delta_chr_f < 100.0);
        assert_eq!(score.exact_lines_fn, 2);
        let location = score.jump_location.unwrap();
        assert_eq!(location.files_fn, 1);
        assert_eq!(location.lines_fn, 1);
    }
}
