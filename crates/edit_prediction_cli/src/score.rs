use crate::{
    PredictArgs,
    example::{Example, ExampleScore},
    headless::EpAppState,
    metrics,
    parse_output::parse_prediction_output,
    predict::run_prediction,
    progress::{ExampleProgress, Step},
    reversal_tracking,
};
use anyhow::Context as _;
use edit_prediction::udiff::apply_diff_to_string_with_hunk_offset;
use gpui::AsyncApp;
use language::text_diff;
use serde::Serialize;

use std::fs::File;
use std::io::BufWriter;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

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

    run_scoring_impl(example)
}

pub fn run_scoring_impl(example: &mut Example) -> anyhow::Result<()> {
    let original_text = &example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs is required for scoring - run prediction first or ensure JSON includes prompt_inputs")?
        .content;

    let expected_patches_with_cursors = example.spec.expected_patches_with_selections();
    let expected_texts_and_selections: Vec<(String, Vec<Range<usize>>)> =
        expected_patches_with_cursors
            .iter()
            .map(|(patch, ranges)| {
                let (result, hunk_offset) =
                    apply_diff_to_string_with_hunk_offset(patch, original_text).with_context(
                        || format!("Expected patch did not apply for {}", example.spec.name),
                    )?;
                Ok((
                    result,
                    ranges
                        .iter()
                        .map(|range| hunk_offset + range.start..hunk_offset + range.end)
                        .collect(),
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

    let zero_scores = ExampleScore {
        delta_chr_f: 0.0,
        braces_disbalance: 0,
        exact_lines_tp: 0,
        exact_lines_fp: 0,
        exact_lines_fn: 0,
        reversal_ratio: 0.0,
        edit_distance: None,
        edit_distance_percentage: None,
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

        let hunk_selections: Vec<Range<usize>> = prediction.actual_selections.clone();
        let Some(actual_patch) = actual_patch else {
            scores.push(zero_scores.clone());
            continue;
        };

        let (actual_text, actual_hunk_offset) =
            match apply_diff_to_string_with_hunk_offset(&actual_patch, original_text) {
                Ok(result) => result,
                Err(_) => {
                    scores.push(zero_scores.clone());
                    continue;
                }
            };

        // Convert hunk-relative selections to full-text coordinates
        let actual_selections: Vec<Range<usize>> = hunk_selections
            .iter()
            .map(|s| (actual_hunk_offset + s.start)..(actual_hunk_offset + s.end))
            .collect();

        let mut best_edit_distance: Option<usize> = None;
        let mut best_original_to_expected_distance: Option<usize> = None;
        let mut best_delta_chr_f = 0.0f32;
        let mut best_selection_metrics = SelectionMetrics {
            cursor_distance: None,
            cursor_exact_match: None,
            selection_start_distance: None,
            selection_exact_match: None,
        };

        for (expected_text, expected_selections) in &expected_texts_and_selections {
            let actual_to_expected_diff = text_diff(&actual_text, expected_text);
            let original_to_expected_diff = text_diff(original_text, expected_text);
            let mapped_selections: Vec<Range<usize>> = actual_selections
                .iter()
                .map(|s| map_selection_through_diff(&actual_to_expected_diff, s.clone()))
                .collect();

            // Compute cursor metrics by comparing mapped selections to expected selections
            let selection_metrics =
                compute_cursor_metrics_from_mapped(expected_selections, &mapped_selections);

            // Filter to only correctly-predicted selections (where mapped selection matches expected)
            let correct_selections: Vec<Range<usize>> = actual_selections
                .iter()
                .zip(mapped_selections.iter())
                .filter(|(_, mapped)| expected_selections.iter().any(|exp| exp == *mapped))
                .map(|(actual, _)| actual.clone())
                .collect();

            // Compute edit distance ignoring whitespace differences and edits within correct selections
            let edit_distance = metrics::whitespace_normalized_edit_distance(
                &actual_to_expected_diff,
                &actual_text,
                &correct_selections,
            );

            // Compute original to expected distance (discounting expected selections as uncertain regions)
            let original_to_expected_distance = metrics::whitespace_normalized_edit_distance(
                &original_to_expected_diff,
                original_text,
                expected_selections,
            );

            let is_better = best_edit_distance
                .map(|best| edit_distance < best)
                .unwrap_or(true);

            if is_better {
                best_edit_distance = Some(edit_distance);
                best_original_to_expected_distance = Some(original_to_expected_distance);
                best_selection_metrics = selection_metrics;
                // Compute delta_chr_f for this best match
                best_delta_chr_f =
                    metrics::delta_chr_f(original_text, expected_text, &actual_text) as f32;
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

        // Compute approximation of editable region correctness
        let wrong_editable_region = Some(!metrics::is_editable_region_correct(&actual_patch));

        // Check for isolated whitespace changes.
        // We need the new editable region text to compute line numbers from offsets.
        // For now, we use the actual_text and assume the editable region starts at hunk_offset.
        let new_editable_region = &actual_text[actual_hunk_offset..];
        let editable_region_start_line = original_text[..actual_hunk_offset].matches('\n').count();
        let has_isolated_whitespace_changes = metrics::has_isolated_whitespace_changes(
            &actual_patch,
            &hunk_selections,
            new_editable_region,
            editable_region_start_line,
        );

        // Compute edit distance percentage: 100 * (original_to_expected - actual_to_expected) / original_to_expected
        let edit_distance_percentage =
            match (best_edit_distance, best_original_to_expected_distance) {
                (Some(actual_to_expected), Some(original_to_expected))
                    if original_to_expected > 0 =>
                {
                    Some(
                        100.0 * (original_to_expected as f32 - actual_to_expected as f32)
                            / original_to_expected as f32,
                    )
                }
                (Some(0), Some(0)) => Some(100.0), // Both zero means perfect match (no edit needed)
                _ => None,
            };

        scores.push(ExampleScore {
            delta_chr_f: best_delta_chr_f,
            braces_disbalance,
            exact_lines_tp: best_exact_lines.true_positives,
            exact_lines_fp: best_exact_lines.false_positives,
            exact_lines_fn: best_exact_lines.false_negatives,
            reversal_ratio,
            edit_distance: best_edit_distance,
            edit_distance_percentage,
            cursor_distance: best_selection_metrics.cursor_distance,
            cursor_exact_match: best_selection_metrics.cursor_exact_match,
            selection_start_distance: best_selection_metrics.selection_start_distance,
            selection_exact_match: best_selection_metrics.selection_exact_match,
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
fn map_selection_through_diff<S: AsRef<str>>(
    edits: &[(Range<usize>, S)],
    actual_selection: Range<usize>,
) -> Range<usize> {
    if edits.is_empty() {
        return actual_selection;
    }

    let map_position = |actual_pos: usize| -> usize {
        let mut actual_cursor = 0usize;
        let mut expected_cursor = 0usize;

        for (old_range, new_text) in edits {
            let new_text = new_text.as_ref();
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

/// Compute cursor metrics from already-mapped selections.
/// This is used when selections have already been mapped through a diff.
fn compute_cursor_metrics_from_mapped(
    expected_selections: &[Range<usize>],
    mapped_actual_selections: &[Range<usize>],
) -> SelectionMetrics {
    if expected_selections.is_empty() && mapped_actual_selections.is_empty() {
        return SelectionMetrics {
            cursor_distance: None,
            cursor_exact_match: None,
            selection_start_distance: None,
            selection_exact_match: None,
        };
    }

    if expected_selections.len() != mapped_actual_selections.len() {
        return SelectionMetrics {
            cursor_distance: None,
            cursor_exact_match: Some(false),
            selection_start_distance: None,
            selection_exact_match: Some(false),
        };
    }

    let mut total_cursor_distance = 0usize;
    let mut total_start_distance = 0usize;

    for (expected, actual) in expected_selections.iter().zip(mapped_actual_selections) {
        total_cursor_distance += expected.end.abs_diff(actual.end);
        total_start_distance += expected.start.abs_diff(actual.start);
    }

    let cursor_exact_match = total_cursor_distance == 0;
    let selection_exact_match = cursor_exact_match && total_start_distance == 0;

    SelectionMetrics {
        cursor_distance: Some(total_cursor_distance),
        cursor_exact_match: Some(cursor_exact_match),
        selection_start_distance: Some(total_start_distance),
        selection_exact_match: Some(selection_exact_match),
    }
}

pub fn print_report(examples: &[Example]) {
    use crate::metrics::ClassificationMetrics;

    const LINE_WIDTH: usize = 119;
    let separator = "─".repeat(LINE_WIDTH);

    println!("{}", separator);
    println!(
        "{:<50} {:>7} {:>8} {:>5} {:>7} {:>7} {:>7} {:>7} {:>6} {:>5}",
        "Example",
        "EditPct",
        "DeltaChrF",
        "Brace",
        "F1",
        "Revert",
        "QaRev",
        "QaConf",
        "Cursor",
        "WrgER"
    );
    println!("{}", separator);

    let mut all_edit_distance_pcts = Vec::new();
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

            // Format edit distance percentage
            let edit_pct_str = match score.edit_distance_percentage {
                Some(pct) => format!("{:.0}%", pct),
                None => "-".to_string(),
            };

            println!(
                "{:<50} {:>7} {:>8.2} {:>5} {:>6.1}% {:>6.1}% {:>7} {:>7} {:>6} {:>5}",
                truncate_name(&example.spec.name, 50),
                edit_pct_str,
                score.delta_chr_f,
                score.braces_disbalance,
                exact_lines.f1() * 100.0,
                score.reversal_ratio * 100.0,
                qa_reverts_str,
                qa_conf_str,
                cursor_str,
                wrong_er_str
            );

            if let Some(pct) = score.edit_distance_percentage {
                all_edit_distance_pcts.push(pct);
            }
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
        let avg_edit_distance_pct: f32 = if !all_edit_distance_pcts.is_empty() {
            all_edit_distance_pcts.iter().sum::<f32>() / all_edit_distance_pcts.len() as f32
        } else {
            0.0
        };
        let avg_edit_pct_str = if !all_edit_distance_pcts.is_empty() {
            format!("{:.0}%", avg_edit_distance_pct)
        } else {
            "-".to_string()
        };
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
            "{:<50} {:>7} {:>8.2} {:>5.1} {:>6.1}% {:>6.1}% {:>7} {:>7} {:>6} {:>5}",
            "TOTAL / AVERAGE",
            avg_edit_pct_str,
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
    use crate::PredictionProvider;
    use crate::example::{Example, ExamplePrediction, ExamplePromptInputs};
    use edit_prediction::example_spec::ExampleSpec;
    use indoc::indoc;
    use std::path::Path;

    fn make_example(
        original_text: &str,
        expected_patch: &str,
        actual_patch: &str,
        actual_selections: Vec<Range<usize>>,
    ) -> Example {
        Example {
            spec: ExampleSpec {
                name: "test".to_string(),
                repository_url: "test".to_string(),
                revision: "test".to_string(),
                tags: vec![],
                reasoning: None,
                uncommitted_diff: String::new(),
                cursor_path: Arc::from(Path::new("test.py")),
                cursor_position: String::new(),
                edit_history: String::new(),
                expected_patches: vec![expected_patch.to_string()],
                rejected_patch: None,
                captured_prompt_input: None,
                telemetry: None,
                human_feedback: vec![],
                rating: None,
            },
            prompt_inputs: Some(ExamplePromptInputs {
                content: original_text.to_string(),
                cursor_row: 0,
                cursor_column: 0,
                cursor_offset: 0,
                selection_start_offset: None,
                excerpt_start_row: None,
                edit_history: vec![],
                related_files: None,
            }),
            prompt: None,
            predictions: vec![ExamplePrediction {
                actual_patch: Some(actual_patch.to_string()),
                actual_output: String::new(),
                actual_selections,
                error: None,
                provider: PredictionProvider::default(),
            }],
            score: vec![],
            qa: vec![],
            state: None,
        }
    }

    /// Tests that selection scoring uses diff normalization to handle different placeholder names.
    ///
    /// When a model predicts a different placeholder name (e.g., "module_name" instead of "module"),
    /// but correctly selects that placeholder, the scoring should recognize this as a perfect
    /// selection match by mapping positions through the diff between actual and expected text.
    #[test]
    fn test_selection_scoring_normalizes_through_diff() {
        // Simple single-line test without context lines
        let original_text = indoc! {"
            import zero
            import one
            import two
            import old_module
            import four
        "};

        // Expected: replace old_module with "module" and select it
        // Marker must be immediately after the line it references
        let expected_patch = indoc! {"
            --- a/test.py
            +++ b/test.py
            @@ -1 +1 @@
             import one
             import two
            -import old_module
            +import module
            #       ------^[SELECTION]
             import four
        "};

        // Actual: replace old_module with "module_name" and select it (7..18)
        let actual_patch = indoc! {"
            --- a/test.py
            +++ b/test.py
            @@ -1 +1 @@
             import one
             import two
            -import old_module
            +import module_name
             import four
        "};

        // In the hunk's new content (context + additions):
        // "import one\n" = 0..11
        // "import two\n" = 11..22
        // "import module_name\n" = 22..41, so "module_name" is at 29..40
        let actual_selection = 29..40;

        let mut example = make_example(
            original_text,
            expected_patch,
            actual_patch,
            vec![actual_selection],
        );

        run_scoring_impl(&mut example).unwrap();

        let score = &example.score[0];
        assert_eq!(
            score.selection_exact_match,
            Some(true),
            "Selection should match after diff normalization maps 'module_name' to 'module'"
        );
        assert_eq!(score.cursor_exact_match, Some(true));
        assert_eq!(score.cursor_distance, Some(0));
        assert_eq!(score.selection_start_distance, Some(0));
    }

    #[test]
    fn test_single_selection_exact_match() {
        let original_text = "let x = old;\n";

        // Expected: select "new" at positions 8..11
        let expected_patch = indoc! {"
            --- a/test.rs
            +++ b/test.rs
            @@ -1 +1 @@
            -let x = old;
            +let x = new;
            #        ---^[SELECTION]
        "};

        let actual_patch = indoc! {"
            --- a/test.rs
            +++ b/test.rs
            @@ -1 +1 @@
            -let x = old;
            +let x = new;
        "};

        // "let x = new;\n" - "new" is at positions 8..11
        let actual_selection = 8..11;

        let mut example = make_example(
            original_text,
            expected_patch,
            actual_patch,
            vec![actual_selection],
        );
        run_scoring_impl(&mut example).unwrap();

        let score = &example.score[0];
        assert_eq!(score.cursor_exact_match, Some(true));
        assert_eq!(score.selection_exact_match, Some(true));
        assert_eq!(score.cursor_distance, Some(0));
        assert_eq!(score.selection_start_distance, Some(0));
    }

    #[test]
    fn test_single_selection_partial_mismatch() {
        let original_text = "let x = old;\n";

        // Expected: select "new" at positions 8..11
        let expected_patch = indoc! {"
            --- a/test.rs
            +++ b/test.rs
            @@ -1 +1 @@
            -let x = old;
            +let x = new;
            #        ---^[SELECTION]
        "};

        let actual_patch = indoc! {"
            --- a/test.rs
            +++ b/test.rs
            @@ -1 +1 @@
            -let x = old;
            +let x = new;
        "};

        // Actual selection is off by 1 on both start and end
        let actual_selection = 9..12;

        let mut example = make_example(
            original_text,
            expected_patch,
            actual_patch,
            vec![actual_selection],
        );
        run_scoring_impl(&mut example).unwrap();

        let score = &example.score[0];
        assert_eq!(score.cursor_exact_match, Some(false));
        assert_eq!(score.selection_exact_match, Some(false));
        assert_eq!(score.cursor_distance, Some(1));
        assert_eq!(score.selection_start_distance, Some(1));
    }

    #[test]
    fn test_selection_count_mismatch_is_miss() {
        let original_text = "let x = old;\n";

        // Expected: one selection
        let expected_patch = indoc! {"
            --- a/test.rs
            +++ b/test.rs
            @@ -1 +1 @@
            -let x = old;
            +let x = new;
            #        ---^[SELECTION]
        "};

        let actual_patch = indoc! {"
            --- a/test.rs
            +++ b/test.rs
            @@ -1 +1 @@
            -let x = old;
            +let x = new;
        "};

        // No actual selections - count mismatch
        let actual_selections = vec![];

        let mut example = make_example(
            original_text,
            expected_patch,
            actual_patch,
            actual_selections,
        );
        run_scoring_impl(&mut example).unwrap();

        let score = &example.score[0];
        assert_eq!(score.cursor_exact_match, Some(false));
        assert_eq!(score.selection_exact_match, Some(false));
        assert_eq!(score.cursor_distance, None);
        assert_eq!(score.selection_start_distance, None);
    }

    #[test]
    fn test_both_empty_skips_scoring() {
        let original_text = "let x = old;\n";

        // No selection markers in expected patch
        let expected_patch = indoc! {"
            --- a/test.rs
            +++ b/test.rs
            @@ -1 +1 @@
            -let x = old;
            +let x = new;
        "};

        let actual_patch = indoc! {"
            --- a/test.rs
            +++ b/test.rs
            @@ -1 +1 @@
            -let x = old;
            +let x = new;
        "};

        // No actual selections
        let actual_selections = vec![];

        let mut example = make_example(
            original_text,
            expected_patch,
            actual_patch,
            actual_selections,
        );
        run_scoring_impl(&mut example).unwrap();

        let score = &example.score[0];
        assert_eq!(score.cursor_exact_match, None);
        assert_eq!(score.selection_exact_match, None);
    }
}
