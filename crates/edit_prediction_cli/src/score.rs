use crate::{
    PredictArgs, PredictionProvider,
    example::{ActualCursor, Example, ExampleScore},
    format_prompt::TeacherPrompt,
    headless::EpAppState,
    metrics,
    parse_output::parse_prediction_output,
    predict::run_prediction,
    progress::{ExampleProgress, Step},
    reversal_tracking,
};
use anyhow::Context as _;
use edit_prediction::udiff::{apply_diff_to_string, apply_diff_to_string_with_hunk_offset};
use gpui::AsyncApp;
use serde::Serialize;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

use std::ops::Range;

pub async fn run_scoring(
    example: &mut Example,
    args: &PredictArgs,
    app_state: Arc<EpAppState>,
    example_progress: &ExampleProgress,
    cx: AsyncApp,
) -> anyhow::Result<()> {
    run_prediction(example, args, app_state, example_progress, cx).await?;

    let progress = example_progress.start(Step::Score);
    progress.set_substatus("computing metrics");

    run_scoring_impl(example).await
}

pub async fn run_scoring_impl(example: &mut Example) -> anyhow::Result<()> {
    let original_text = &example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs is required for scoring - run prediction first or ensure JSON includes prompt_inputs")?
        .content;
    let expected_patches_with_cursors = example.spec.expected_patches_with_selections();

    let expected_texts: Vec<String> = expected_patches_with_cursors
        .iter()
        .map(|(patch, _)| {
            apply_diff_to_string(patch, original_text)
                .with_context(|| format!("Expected patch did not apply for {}", example.spec.name))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // For Teacher prompts, we need to extract the editable region to properly compute cursor offsets.
    // The actual_cursor_offset from Teacher is relative to the editable region, while the expected
    // cursor from the patch is relative to the hunk. We need to apply the patch to the editable
    // region to find where the hunk matched, then compute the expected cursor position.
    let old_editable_region = if let Some(p) = example.prompt.as_ref() {
        if matches!(
            p.provider,
            PredictionProvider::Teacher(_) | PredictionProvider::TeacherNonBatching(_)
        ) {
            Some(
                TeacherPrompt::extract_editable_region(&p.input)?
                    .replace(TeacherPrompt::USER_CURSOR_MARKER, ""),
            )
        } else {
            None
        }
    } else {
        None
    };

    let zero_scores = ExampleScore {
        delta_chr_f: 0.0,
        braces_disbalance: 0,
        exact_lines_tp: 0,
        exact_lines_fp: 0,
        exact_lines_fn: 0,
        reversal_ratio: 0.0,
        cursor_distance: None,
        cursor_exact_match: None,
        selection_start_distance: None,
        selection_exact_match: None,
        wrong_editable_region: None,
        has_isolated_whitespace_changes: false,
    };

    let prompt_inputs = example.prompt_inputs.as_ref().unwrap();
    let cursor_path = example.spec.cursor_path.as_ref();

    let mut scores = vec![];
    for prediction in &example.predictions {
        let actual_patch = prediction.actual_patch.clone().or_else(|| {
            parse_prediction_output(example, &prediction.actual_output, prediction.provider)
                .ok()
                .map(|(patch, _)| patch)
        });

        let Some(actual_patch) = actual_patch else {
            scores.push(zero_scores.clone());
            continue;
        };

        let (actual_text, _actual_hunk_offset) =
            match apply_diff_to_string_with_hunk_offset(&actual_patch, original_text) {
                Ok(result) => result,
                Err(_) => {
                    scores.push(zero_scores.clone());
                    continue;
                }
            };

        let mut best_delta_chr_f = 0.0f32;
        let mut best_expected_selection: Option<Range<usize>> = None;
        let mut best_expected_new_editable_region: Option<String> = None;
        let mut best_patch_idx: Option<usize> = None;

        for (idx, expected) in expected_texts.iter().enumerate() {
            let delta_chr_f = metrics::delta_chr_f(original_text, expected, &actual_text) as f32;
            if delta_chr_f > best_delta_chr_f {
                best_delta_chr_f = delta_chr_f;
                best_patch_idx = Some(idx);
            }
        }

        if let Some(idx) = best_patch_idx {
            let (patch, _) = &expected_patches_with_cursors[idx];

            // Get the selection range from the expected patch (relative to hunk new text).
            let expected_selection_in_patch = expected_patches_with_cursors
                .get(idx)
                .and_then(|(_, selection)| selection.clone());

            // For Teacher prompts, we need to apply the patch to the editable region
            // to find where the hunk matched, then compute the expected selection position
            // and the new editable region text (for diff normalization).
            if let Some(editable_region) = &old_editable_region {
                if let Ok((new_editable_region, hunk_offset)) =
                    apply_diff_to_string_with_hunk_offset(patch, editable_region)
                {
                    best_expected_new_editable_region = Some(new_editable_region);

                    if let Some(selection_in_patch) = expected_selection_in_patch.clone() {
                        let hunk_start = hunk_offset.unwrap_or(0);
                        // Shift the selection range by hunk offset
                        best_expected_selection = Some(
                            (hunk_start + selection_in_patch.start)
                                ..(hunk_start + selection_in_patch.end),
                        );
                    }
                }
            } else if let Some(selection) = expected_selection_in_patch {
                // For non-Teacher prompts or if we can't compute, use raw selection
                best_expected_selection = Some(selection);
            }
        }

        let disbalance_before = metrics::braces_disbalance(&original_text);
        let disbalance_after = metrics::braces_disbalance(&actual_text);
        let braces_disbalance = disbalance_after.saturating_sub(disbalance_before);

        // Compute exact lines match against best matching expected patch
        let best_exact_lines = expected_patches_with_cursors
            .iter()
            .map(|(expected_patch, _)| metrics::exact_lines_match(expected_patch, &actual_patch))
            .max_by_key(|m| m.true_positives)
            .unwrap_or_default();

        // Compute reversal ratio
        let reversal_ratio = reversal_tracking::compute_prediction_reversal_ratio(
            prompt_inputs,
            &actual_text,
            cursor_path,
        );

        // Compute actual new editable region for diff-normalized selection comparison.
        // We use the hunk offset from the patch application to determine where the edit
        // occurred, then extract the corresponding portion of the new text.
        let actual_new_editable_region = old_editable_region.as_ref().map(|old_region| {
            let old_region_len = old_region.len();
            let old_text_len = original_text.len();
            let new_text_len = actual_text.len();
            let length_delta = new_text_len as isize - old_text_len as isize;
            let new_region_len = (old_region_len as isize + length_delta).max(0) as usize;
            let new_region_len = new_region_len.min(actual_text.len());
            actual_text[..new_region_len].to_string()
        });

        // Compute cursor/selection position metrics
        let selection_metrics = compute_cursor_metrics(
            best_expected_selection.clone(),
            prediction.actual_cursor.as_ref(),
            actual_new_editable_region.as_deref(),
            best_expected_new_editable_region.as_deref(),
        );

        // Compute approximation of editable region correctness
        let wrong_editable_region = Some(!metrics::is_editable_region_correct(&actual_patch));

        // Check for isolated whitespace changes.
        let has_isolated_whitespace_changes = metrics::has_isolated_whitespace_changes(
            &actual_patch,
            prediction.actual_cursor.as_ref(),
        );

        scores.push(ExampleScore {
            delta_chr_f: best_delta_chr_f,
            braces_disbalance,
            exact_lines_tp: best_exact_lines.true_positives,
            exact_lines_fp: best_exact_lines.false_positives,
            exact_lines_fn: best_exact_lines.false_negatives,
            reversal_ratio,
            cursor_distance: selection_metrics.cursor_distance,
            cursor_exact_match: selection_metrics.cursor_exact_match,
            selection_start_distance: selection_metrics.selection_start_distance,
            selection_exact_match: selection_metrics.selection_exact_match,
            wrong_editable_region,
            has_isolated_whitespace_changes,
        });
    }

    example.score = scores;
    Ok(())
}

/// Result of comparing expected and actual selection ranges.
struct SelectionMetrics {
    /// Distance between expected and actual cursor (end of selection) positions.
    cursor_distance: Option<usize>,
    /// Whether the cursor (end of selection) positions match exactly.
    cursor_exact_match: Option<bool>,
    /// Distance between expected and actual selection start positions.
    selection_start_distance: Option<usize>,
    /// Whether the full selection (both start and end) matches exactly.
    selection_exact_match: Option<bool>,
}

/// Maps a selection range from the actual text's coordinate space to the expected text's
/// coordinate space using a word-level diff.
///
/// This allows comparing selections even when the actual and expected texts differ slightly
/// (e.g., different placeholder variable names). If the selection covers a region that was
/// changed, it maps to the corresponding changed region in the expected text.
fn map_selection_through_diff(
    actual_text: &str,
    expected_text: &str,
    actual_selection: Range<usize>,
) -> Range<usize> {
    if actual_text == expected_text {
        return actual_selection;
    }

    let edits = language::text_diff(actual_text, expected_text);

    let map_position = |actual_pos: usize| -> usize {
        let mut actual_cursor = 0usize;
        let mut expected_cursor = 0usize;

        for (old_range, new_text) in &edits {
            // Check if position is in the unchanged region before this edit.
            if actual_pos < old_range.start {
                return expected_cursor + (actual_pos - actual_cursor);
            }

            // Advance expected_cursor past the unchanged region.
            expected_cursor += old_range.start - actual_cursor;

            // Check if position is within the changed region.
            if actual_pos <= old_range.end {
                if actual_pos == old_range.start {
                    return expected_cursor;
                } else if actual_pos == old_range.end {
                    return expected_cursor + new_text.len();
                } else if old_range.len() > 0 {
                    // Position is in the middle - map proportionally.
                    let ratio = (actual_pos - old_range.start) as f64 / old_range.len() as f64;
                    return expected_cursor + (ratio * new_text.len() as f64) as usize;
                } else {
                    return expected_cursor;
                }
            }

            expected_cursor += new_text.len();
            actual_cursor = old_range.end;
        }

        // Position is after all edits.
        expected_cursor + (actual_pos - actual_cursor)
    };

    map_position(actual_selection.start)..map_position(actual_selection.end)
}

fn compute_cursor_metrics(
    expected_selection: Option<Range<usize>>,
    actual_cursor: Option<&ActualCursor>,
    actual_text: Option<&str>,
    expected_text: Option<&str>,
) -> SelectionMetrics {
    let actual_selection = actual_cursor.and_then(|c| c.editable_region_selection());

    match (&expected_selection, &actual_selection) {
        (Some(expected), Some(actual)) => {
            // If we have both texts, map the actual selection through the diff
            // to normalize for minor text differences (e.g., different placeholder names).
            let normalized_actual = match (actual_text, expected_text) {
                (Some(actual_txt), Some(expected_txt)) => {
                    map_selection_through_diff(actual_txt, expected_txt, actual.clone())
                }
                _ => actual.clone(),
            };

            let cursor_distance = expected.end.abs_diff(normalized_actual.end);
            let start_distance = expected.start.abs_diff(normalized_actual.start);
            let cursor_exact_match = cursor_distance == 0;
            let selection_exact_match = cursor_exact_match && start_distance == 0;

            SelectionMetrics {
                cursor_distance: Some(cursor_distance),
                cursor_exact_match: Some(cursor_exact_match),
                selection_start_distance: Some(start_distance),
                selection_exact_match: Some(selection_exact_match),
            }
        }
        (None, None) => {
            // Neither has selection - skip scoring
            SelectionMetrics {
                cursor_distance: None,
                cursor_exact_match: None,
                selection_start_distance: None,
                selection_exact_match: None,
            }
        }
        (Some(_), None) | (None, Some(_)) => {
            // Only one has selection - count as miss
            SelectionMetrics {
                cursor_distance: None,
                cursor_exact_match: Some(false),
                selection_start_distance: None,
                selection_exact_match: Some(false),
            }
        }
    }
}

pub fn print_report(examples: &[Example]) {
    use crate::metrics::ClassificationMetrics;

    const LINE_WIDTH: usize = 101;
    let separator = "─".repeat(LINE_WIDTH);

    println!("{}", separator);
    println!(
        "{:<40} {:>8} {:>5} {:>7} {:>7} {:>7} {:>7} {:>6} {:>5}",
        "Example", "DeltaChrF", "Brace", "F1", "Revert", "QaRev", "QaConf", "Cursor", "WrgER"
    );
    println!("{}", separator);

    let mut all_delta_chr_f_scores = Vec::new();
    let mut all_reversal_ratios = Vec::new();
    let mut braces_disbalance_sum: usize = 0;
    let mut total_exact_lines = ClassificationMetrics::default();
    let mut total_scores: usize = 0;
    let mut qa_reverts_count: usize = 0;
    let mut qa_reverts_total: usize = 0;
    let mut qa_confidence_sum: u64 = 0;
    let mut qa_confidence_count: usize = 0;
    let mut cursor_exact_matches: usize = 0;
    let mut cursor_total: usize = 0;
    let mut cursor_distance_sum: usize = 0;
    let mut cursor_distance_count: usize = 0;
    let mut selection_exact_matches: usize = 0;
    let mut selection_total: usize = 0;
    let mut selection_start_distance_sum: usize = 0;
    let mut selection_start_distance_count: usize = 0;
    let mut wrong_editable_region_count: usize = 0;
    let mut wrong_editable_region_total: usize = 0;
    let mut isolated_whitespace_count: usize = 0;

    for example in examples {
        for (score_idx, score) in example.score.iter().enumerate() {
            let exact_lines = ClassificationMetrics {
                true_positives: score.exact_lines_tp,
                false_positives: score.exact_lines_fp,
                false_negatives: score.exact_lines_fn,
            };

            // Get QA results for this prediction if available
            let qa_result = example.qa.get(score_idx).and_then(|q| q.as_ref());
            let qa_reverts_str = qa_result
                .and_then(|q| q.reverts_edits)
                .map(|v| if v { "yes" } else { "no" })
                .unwrap_or("-");
            let qa_conf_str = qa_result
                .and_then(|q| q.confidence)
                .map(|v| format!("{}", v))
                .unwrap_or("-".to_string());

            // Format wrong editable region metric
            let wrong_er_str = match score.wrong_editable_region {
                Some(true) => "✗",
                Some(false) => "",
                None => "",
            };

            // Format cursor metric
            let cursor_str = match (score.cursor_exact_match, score.cursor_distance) {
                (Some(true), _) => "✓".to_string(),
                (Some(false), Some(dist)) => format!("±{}", dist),
                (Some(false), None) => "✗".to_string(),
                (None, _) => "-".to_string(),
            };

            println!(
                "{:<40} {:>8.2} {:>5} {:>6.1}% {:>6.1}% {:>7} {:>7} {:>6} {:>5}",
                truncate_name(&example.spec.name, 40),
                score.delta_chr_f,
                score.braces_disbalance,
                exact_lines.f1() * 100.0,
                score.reversal_ratio * 100.0,
                qa_reverts_str,
                qa_conf_str,
                cursor_str,
                wrong_er_str
            );

            all_delta_chr_f_scores.push(score.delta_chr_f);
            all_reversal_ratios.push(score.reversal_ratio);
            total_scores += 1;
            braces_disbalance_sum += score.braces_disbalance;
            total_exact_lines.true_positives += score.exact_lines_tp;
            total_exact_lines.false_positives += score.exact_lines_fp;
            total_exact_lines.false_negatives += score.exact_lines_fn;

            // Accumulate QA metrics
            if let Some(qa) = qa_result {
                if let Some(reverts) = qa.reverts_edits {
                    qa_reverts_total += 1;
                    if reverts {
                        qa_reverts_count += 1;
                    }
                }
                if let Some(conf) = qa.confidence {
                    qa_confidence_sum += conf as u64;
                    qa_confidence_count += 1;
                }
            }

            // Accumulate wrong editable region metrics
            if let Some(wrong) = score.wrong_editable_region {
                wrong_editable_region_total += 1;
                if wrong {
                    wrong_editable_region_count += 1;
                }
            }

            // Accumulate isolated whitespace metrics
            if score.has_isolated_whitespace_changes {
                isolated_whitespace_count += 1;
            }

            // Accumulate cursor metrics
            if let Some(exact_match) = score.cursor_exact_match {
                cursor_total += 1;
                if exact_match {
                    cursor_exact_matches += 1;
                }
            }
            if let Some(dist) = score.cursor_distance {
                cursor_distance_sum += dist;
                cursor_distance_count += 1;
            }

            // Accumulate selection metrics
            if let Some(exact_match) = score.selection_exact_match {
                selection_total += 1;
                if exact_match {
                    selection_exact_matches += 1;
                }
            }
            if let Some(dist) = score.selection_start_distance {
                selection_start_distance_sum += dist;
                selection_start_distance_count += 1;
            }
        }
    }

    println!("{}", separator);

    if !all_delta_chr_f_scores.is_empty() {
        let avg_delta_chr_f: f32 =
            all_delta_chr_f_scores.iter().sum::<f32>() / all_delta_chr_f_scores.len() as f32;
        let avg_reversal_ratio: f32 =
            all_reversal_ratios.iter().sum::<f32>() / all_reversal_ratios.len() as f32;
        let braces_disbalance_avg: f32 = braces_disbalance_sum as f32 / total_scores as f32;

        let qa_reverts_str = if qa_reverts_total > 0 {
            format!(
                "{:.1}%",
                qa_reverts_count as f32 / qa_reverts_total as f32 * 100.0
            )
        } else {
            "-".to_string()
        };
        let qa_conf_str = if qa_confidence_count > 0 {
            format!(
                "{:.1}",
                qa_confidence_sum as f32 / qa_confidence_count as f32
            )
        } else {
            "-".to_string()
        };
        let cursor_str = if cursor_total > 0 {
            format!(
                "{:.0}%",
                cursor_exact_matches as f32 / cursor_total as f32 * 100.0
            )
        } else {
            "-".to_string()
        };
        let wrong_er_str = if wrong_editable_region_total > 0 {
            format!(
                "{:.2}%",
                wrong_editable_region_count as f32 / wrong_editable_region_total as f32 * 100.0
            )
        } else {
            "-".to_string()
        };
        let isolated_ws_str = if total_scores > 0 {
            format!(
                "{}/{} ({:.1}%)",
                isolated_whitespace_count,
                total_scores,
                isolated_whitespace_count as f32 / total_scores as f32 * 100.0
            )
        } else {
            "-".to_string()
        };
        let avg_cursor_distance = if cursor_distance_count > 0 {
            Some(cursor_distance_sum as f32 / cursor_distance_count as f32)
        } else {
            None
        };
        let avg_selection_start_distance = if selection_start_distance_count > 0 {
            Some(selection_start_distance_sum as f32 / selection_start_distance_count as f32)
        } else {
            None
        };

        println!(
            "{:<40} {:>8.2} {:>5.1} {:>6.1}% {:>6.1}% {:>7} {:>7} {:>6} {:>5}",
            "TOTAL / AVERAGE",
            avg_delta_chr_f,
            braces_disbalance_avg,
            total_exact_lines.f1() * 100.0,
            avg_reversal_ratio * 100.0,
            qa_reverts_str,
            qa_conf_str,
            cursor_str,
            wrong_er_str
        );
        println!("{}", separator);

        // Print additional cursor metrics if available
        if let Some(avg_dist) = avg_cursor_distance {
            println!(
                "Cursor: {}/{} exact matches ({:.0}%), avg distance: {:.1} bytes",
                cursor_exact_matches,
                cursor_total,
                cursor_exact_matches as f32 / cursor_total as f32 * 100.0,
                avg_dist
            );
        }

        // Print selection metrics if available
        if selection_total > 0 {
            let selection_exact_match_rate =
                selection_exact_matches as f32 / selection_total as f32 * 100.0;
            let avg_start_dist_str = avg_selection_start_distance
                .map(|d| format!("{:.1}", d))
                .unwrap_or("-".to_string());
            println!(
                "Selection: {}/{} exact matches ({:.0}%), avg start distance: {} bytes",
                selection_exact_matches,
                selection_total,
                selection_exact_match_rate,
                avg_start_dist_str
            );
        }

        // Print isolated whitespace metrics
        if total_scores > 0 {
            println!("Isolated whitespace changes: {}", isolated_ws_str);
        }
    }

    println!("\n");
}

fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[..max_len - 3])
    }
}

#[derive(Serialize)]
pub struct SummaryJson {
    pub total_examples: usize,
    pub avg_delta_chr_f: f32,
    pub avg_braces_disbalance: f32,
    pub exact_lines_true_positives: usize,
    pub exact_lines_false_positives: usize,
    pub exact_lines_false_negatives: usize,
    pub exact_lines_precision: f64,
    pub exact_lines_recall: f64,
    pub exact_lines_f1: f64,
    pub avg_reversal_ratio: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qa_avg_reverts_edits: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qa_avg_confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_exact_match_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_avg_distance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_total_evaluated: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_exact_match_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_start_avg_distance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_editable_region_rate: Option<f32>,
    pub isolated_whitespace_rate: Option<f32>,
}

pub fn compute_summary(examples: &[Example]) -> SummaryJson {
    use crate::metrics::ClassificationMetrics;

    let mut all_delta_chr_f_scores = Vec::new();
    let mut all_reversal_ratios = Vec::new();
    let mut braces_disbalance_sum: usize = 0;
    let mut total_exact_lines = ClassificationMetrics::default();
    let mut total_scores: usize = 0;
    let mut qa_reverts_count: usize = 0;
    let mut qa_reverts_total: usize = 0;
    let mut qa_confidence_sum: u64 = 0;
    let mut qa_confidence_count: usize = 0;
    let mut cursor_exact_matches: usize = 0;
    let mut cursor_total: usize = 0;
    let mut cursor_distance_sum: usize = 0;
    let mut cursor_distance_count: usize = 0;
    let mut selection_exact_matches: usize = 0;
    let mut selection_total: usize = 0;
    let mut selection_start_distance_sum: usize = 0;
    let mut selection_start_distance_count: usize = 0;
    let mut wrong_editable_region_count: usize = 0;
    let mut wrong_editable_region_total: usize = 0;
    let mut isolated_whitespace_count: usize = 0;

    for example in examples {
        for (score_idx, score) in example.score.iter().enumerate() {
            all_delta_chr_f_scores.push(score.delta_chr_f);
            all_reversal_ratios.push(score.reversal_ratio);
            total_scores += 1;
            braces_disbalance_sum += score.braces_disbalance;
            total_exact_lines.true_positives += score.exact_lines_tp;
            total_exact_lines.false_positives += score.exact_lines_fp;
            total_exact_lines.false_negatives += score.exact_lines_fn;

            // Accumulate QA metrics
            if let Some(Some(qa)) = example.qa.get(score_idx) {
                if let Some(reverts) = qa.reverts_edits {
                    qa_reverts_total += 1;
                    if reverts {
                        qa_reverts_count += 1;
                    }
                }
                if let Some(conf) = qa.confidence {
                    qa_confidence_sum += conf as u64;
                    qa_confidence_count += 1;
                }
            }

            // Accumulate wrong editable region metrics
            if let Some(wrong) = score.wrong_editable_region {
                wrong_editable_region_total += 1;
                if wrong {
                    wrong_editable_region_count += 1;
                }
            }

            // Accumulate isolated whitespace metrics
            if score.has_isolated_whitespace_changes {
                isolated_whitespace_count += 1;
            }

            // Accumulate cursor metrics
            if let Some(exact_match) = score.cursor_exact_match {
                cursor_total += 1;
                if exact_match {
                    cursor_exact_matches += 1;
                }
            }
            if let Some(dist) = score.cursor_distance {
                cursor_distance_sum += dist;
                cursor_distance_count += 1;
            }

            // Accumulate selection metrics
            if let Some(exact_match) = score.selection_exact_match {
                selection_total += 1;
                if exact_match {
                    selection_exact_matches += 1;
                }
            }
            if let Some(dist) = score.selection_start_distance {
                selection_start_distance_sum += dist;
                selection_start_distance_count += 1;
            }
        }
    }

    let avg_delta_chr_f = if all_delta_chr_f_scores.is_empty() {
        0.0
    } else {
        all_delta_chr_f_scores.iter().sum::<f32>() / all_delta_chr_f_scores.len() as f32
    };

    let avg_reversal_ratio = if all_reversal_ratios.is_empty() {
        0.0
    } else {
        all_reversal_ratios.iter().sum::<f32>() / all_reversal_ratios.len() as f32
    };

    let avg_braces_disbalance = if total_scores == 0 {
        0.0
    } else {
        braces_disbalance_sum as f32 / total_scores as f32
    };

    let qa_avg_reverts_edits = if qa_reverts_total > 0 {
        Some(qa_reverts_count as f32 / qa_reverts_total as f32)
    } else {
        None
    };

    let qa_avg_confidence = if qa_confidence_count > 0 {
        Some(qa_confidence_sum as f32 / qa_confidence_count as f32)
    } else {
        None
    };

    let cursor_exact_match_rate = if cursor_total > 0 {
        Some(cursor_exact_matches as f32 / cursor_total as f32)
    } else {
        None
    };

    let cursor_avg_distance = if cursor_distance_count > 0 {
        Some(cursor_distance_sum as f32 / cursor_distance_count as f32)
    } else {
        None
    };

    let cursor_total_evaluated = if cursor_total > 0 {
        Some(cursor_total)
    } else {
        None
    };

    let selection_exact_match_rate = if selection_total > 0 {
        Some(selection_exact_matches as f32 / selection_total as f32)
    } else {
        None
    };

    let selection_start_avg_distance = if selection_start_distance_count > 0 {
        Some(selection_start_distance_sum as f32 / selection_start_distance_count as f32)
    } else {
        None
    };

    let wrong_editable_region_rate = if wrong_editable_region_total > 0 {
        Some(wrong_editable_region_count as f32 / wrong_editable_region_total as f32)
    } else {
        None
    };

    let isolated_whitespace_rate = if total_scores > 0 {
        Some(isolated_whitespace_count as f32 / total_scores as f32)
    } else {
        None
    };

    SummaryJson {
        total_examples: total_scores,
        avg_delta_chr_f,
        avg_braces_disbalance,
        exact_lines_true_positives: total_exact_lines.true_positives,
        exact_lines_false_positives: total_exact_lines.false_positives,
        exact_lines_false_negatives: total_exact_lines.false_negatives,
        exact_lines_precision: total_exact_lines.precision(),
        exact_lines_recall: total_exact_lines.recall(),
        exact_lines_f1: total_exact_lines.f1(),
        avg_reversal_ratio,
        qa_avg_reverts_edits,
        qa_avg_confidence,
        cursor_exact_match_rate,
        cursor_avg_distance,
        cursor_total_evaluated,
        selection_exact_match_rate,
        selection_start_avg_distance,
        wrong_editable_region_rate,
        isolated_whitespace_rate,
    }
}

pub fn write_summary_json(examples: &[Example], path: &Path) -> anyhow::Result<()> {
    let summary = compute_summary(examples);
    let file = File::create(path)
        .with_context(|| format!("Failed to create summary JSON file: {}", path.display()))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &summary)
        .with_context(|| format!("Failed to write summary JSON to: {}", path.display()))?;
    eprintln!("Wrote summary JSON to: {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that selection scoring uses diff normalization to handle different placeholder names.
    ///
    /// When a model predicts a different placeholder name (e.g., "module_name" instead of "module"),
    /// but correctly selects that placeholder, the scoring should recognize this as a perfect
    /// selection match by mapping positions through the diff between actual and expected text.
    #[test]
    fn test_selection_scoring_normalizes_through_diff() {
        // Scenario: expected text has "module", actual text has "module_name"
        // The selection covers the placeholder in both cases.
        let expected_text = "import module\nfrom werkzeug.local import LocalProxy\n";
        let actual_text = "import module_name\nfrom werkzeug.local import LocalProxy\n";

        // Expected selection: "module" at positions 7..13
        let expected_selection = Some(7..13);

        // Actual selection: "module_name" at positions 7..18
        let actual_cursor = ActualCursor {
            path: "test.py".to_string(),
            row: 0,
            column: 18,
            offset: 18,
            editable_region_offset: Some(18),
            selection_start_offset: Some(7),
            selection_start_editable_region_offset: Some(7),
        };

        // Call compute_cursor_metrics with diff normalization enabled
        let metrics = compute_cursor_metrics(
            expected_selection,
            Some(&actual_cursor),
            Some(actual_text),
            Some(expected_text),
        );

        // With diff normalization, the actual selection (7..18 for "module_name")
        // should map to the expected selection (7..13 for "module")
        assert_eq!(
            metrics.selection_exact_match,
            Some(true),
            "Selection should match after diff normalization maps 'module_name' to 'module'"
        );
        assert_eq!(metrics.cursor_exact_match, Some(true));
        assert_eq!(metrics.cursor_distance, Some(0));
        assert_eq!(metrics.selection_start_distance, Some(0));
    }
}
