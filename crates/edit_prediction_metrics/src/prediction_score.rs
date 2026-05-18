use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use zeta_prompt::udiff::{apply_diff_to_string, apply_diff_to_string_with_hunk_offset};

use crate::patch_metrics::{
    ClassificationMetrics, DeltaChrFMetrics, braces_disbalance, count_patch_token_changes,
    delta_chr_f, delta_chr_f_beta, exact_lines_match, has_isolated_whitespace_changes,
    is_editable_region_correct,
};
use crate::reversal::compute_prediction_reversal_ratio_from_history;

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
}

pub fn score_prediction(input: PredictionScoringInput<'_>) -> PredictionScore {
    let Some(actual_patch) = input.actual_patch else {
        return PredictionScore::zero();
    };

    let token_changes = count_patch_token_changes(actual_patch);

    let actual_text = match apply_diff_to_string(actual_patch, input.original_text) {
        Ok(text) => text,
        Err(_) => {
            let mut score = PredictionScore::zero();
            score.inserted_tokens = token_changes.inserted_tokens;
            score.deleted_tokens = token_changes.deleted_tokens;
            return score;
        }
    };

    let mut best_delta_chr_f_metrics = DeltaChrFMetrics::default();
    let mut best_expected_cursor = None;
    let mut best_expected_text = None;

    for expected in input.expected_patches {
        let delta_chr_f_metrics = delta_chr_f(input.original_text, &expected.text, &actual_text);
        if best_expected_text.is_none()
            || delta_chr_f_metrics.score > best_delta_chr_f_metrics.score
        {
            best_delta_chr_f_metrics = delta_chr_f_metrics;
            best_expected_cursor = expected.cursor_editable_region_offset;
            best_expected_text = Some(expected.text.as_str());
        }
    }

    let disbalance_before = braces_disbalance(input.original_text);
    let disbalance_after = braces_disbalance(&actual_text);
    let braces_disbalance = disbalance_after.saturating_sub(disbalance_before);

    let best_exact_lines = input
        .expected_patches
        .iter()
        .map(|expected| exact_lines_match(&expected.patch, actual_patch))
        .max_by_key(|metrics| metrics.true_positives)
        .unwrap_or_default();

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

    let (cursor_distance, cursor_exact_match) =
        compute_cursor_metrics(best_expected_cursor, input.actual_cursor);

    let wrong_editable_region = Some(!is_editable_region_correct(actual_patch));
    let has_isolated_whitespace_changes =
        has_isolated_whitespace_changes(actual_patch, input.actual_cursor.map(|cursor| cursor.row));

    let (kept_rate, recall_rate, kept_chars, correctly_deleted_chars, discarded_chars) =
        best_expected_text
            .map(|reference_text| {
                let result = crate::kept_rate::compute_kept_rate(
                    input.original_text,
                    &actual_text,
                    reference_text,
                );
                (
                    Some(result.kept_rate),
                    Some(result.recall_rate),
                    Some(result.kept_chars),
                    Some(result.correctly_deleted_chars),
                    Some(result.discarded_chars),
                )
            })
            .unwrap_or((None, None, None, None, None));

    PredictionScore {
        delta_chr_f: best_delta_chr_f_metrics.score as f32,
        delta_chr_f_true_positives: best_delta_chr_f_metrics.counts.true_positives,
        delta_chr_f_false_positives: best_delta_chr_f_metrics.counts.false_positives,
        delta_chr_f_false_negatives: best_delta_chr_f_metrics.counts.false_negatives,
        delta_chr_f_precision: best_delta_chr_f_metrics.precision,
        delta_chr_f_recall: best_delta_chr_f_metrics.recall,
        delta_chr_f_beta: best_delta_chr_f_metrics.beta,
        braces_disbalance,
        exact_lines_tp: best_exact_lines.true_positives,
        exact_lines_fp: best_exact_lines.false_positives,
        exact_lines_fn: best_exact_lines.false_negatives,
        reversal_ratio,
        cursor_distance,
        cursor_exact_match,
        wrong_editable_region,
        has_isolated_whitespace_changes,
        inserted_tokens: token_changes.inserted_tokens,
        deleted_tokens: token_changes.deleted_tokens,
        kept_rate,
        recall_rate,
        kept_chars,
        correctly_deleted_chars,
        discarded_chars,
        cumulative_logprob: input.cumulative_logprob,
        avg_logprob: input.avg_logprob,
    }
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
        });

        assert_eq!(score.delta_chr_f, 0.0);
        assert_eq!(score.kept_rate, Some(0.0));
    }
}
