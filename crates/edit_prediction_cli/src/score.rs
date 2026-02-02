use crate::{
    PredictArgs, PredictionProvider,
    example::{Example, ExampleScore},
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

pub async fn run_scoring(
    example: &mut Example,
    args: &PredictArgs,
    app_state: Arc<EpAppState>,
    example_progress: &ExampleProgress,
    cx: AsyncApp,
) -> anyhow::Result<()> {
    run_prediction(example, args, app_state, example_progress, cx).await?;

    let progress = example_progress.start(Step::Score);

    progress.set_substatus("applying patches");
    let original_text = &example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs is required for scoring - run prediction first or ensure JSON includes prompt_inputs")?
        .content;
    let expected_patches_with_cursors = example.spec.expected_patches_with_cursor_positions();

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
    };

    let prompt_inputs = example.prompt_inputs.as_ref().unwrap();
    let cursor_path = example.spec.cursor_path.as_ref();

    progress.set_substatus("computing metrics");
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

        let actual_text = match apply_diff_to_string(&actual_patch, original_text) {
            Ok(text) => text,
            Err(_) => {
                scores.push(zero_scores.clone());
                continue;
            }
        };

        let mut best_delta_chr_f = 0.0f32;
        let mut best_expected_cursor: Option<usize> = None;
        let mut best_patch_idx: Option<usize> = None;

        for (idx, expected) in expected_texts.iter().enumerate() {
            let delta_chr_f = metrics::delta_chr_f(original_text, expected, &actual_text) as f32;
            if delta_chr_f > best_delta_chr_f {
                best_delta_chr_f = delta_chr_f;
                best_patch_idx = Some(idx);
            }
        }

        if let Some(idx) = best_patch_idx {
            // Get the raw cursor offset from the expected patch (relative to hunk new text)
            let expected_cursor_in_patch = expected_patches_with_cursors
                .get(idx)
                .and_then(|(_, cursor)| *cursor);

            // For Teacher prompts, we need to apply the patch to the editable region
            // to find where the hunk matched, then compute the actual cursor position
            if let (Some(editable_region), Some(cursor_in_patch)) =
                (&old_editable_region, expected_cursor_in_patch)
            {
                let (patch, _) = &expected_patches_with_cursors[idx];
                if let Ok((_, hunk_offset)) =
                    apply_diff_to_string_with_hunk_offset(patch, editable_region)
                {
                    let hunk_start = hunk_offset.unwrap_or(0);
                    best_expected_cursor = Some(hunk_start + cursor_in_patch);
                }
            } else {
                // For non-Teacher prompts or if we can't compute, use raw offset
                best_expected_cursor = expected_cursor_in_patch;
            }
        }

        let disbalance_before = metrics::braces_disbalance(&original_text);
        let disbalance_after = metrics::braces_disbalance(&actual_text);
        let braces_disbalance = disbalance_after.saturating_sub(disbalance_before);
        if braces_disbalance > 0 {
            std::fs::write(
                "/tmp/unbalanced-count.before",
                disbalance_before.to_string(),
            )
            .ok();
            std::fs::write("/tmp/unbalanced-count.after", disbalance_after.to_string()).ok();
            std::fs::write("/tmp/unbalanced-text.before", &original_text).ok();
            std::fs::write("/tmp/unbalanced-text.after", &actual_text).ok();
        }

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

        // Compute cursor position metrics
        let (cursor_distance, cursor_exact_match) =
            compute_cursor_metrics(best_expected_cursor, prediction.actual_cursor_offset);

        scores.push(ExampleScore {
            delta_chr_f: best_delta_chr_f,
            braces_disbalance,
            exact_lines_tp: best_exact_lines.true_positives,
            exact_lines_fp: best_exact_lines.false_positives,
            exact_lines_fn: best_exact_lines.false_negatives,
            reversal_ratio,
            cursor_distance,
            cursor_exact_match,
        });
    }

    example.score = scores;
    Ok(())
}

fn compute_cursor_metrics(
    expected_cursor: Option<usize>,
    actual_cursor: Option<usize>,
) -> (Option<usize>, Option<bool>) {
    match (expected_cursor, actual_cursor) {
        (Some(expected), Some(actual)) => {
            let distance = expected.abs_diff(actual);
            let exact_match = expected == actual;
            (Some(distance), Some(exact_match))
        }
        (None, None) => {
            // Neither has cursor position - skip cursor scoring
            (None, None)
        }
        (Some(_), None) | (None, Some(_)) => {
            // Only one has cursor position - count as miss
            (None, Some(false))
        }
    }
}

pub fn print_report(examples: &[Example]) {
    use crate::metrics::ClassificationMetrics;

    const LINE_WIDTH: usize = 94;
    let separator = "─".repeat(LINE_WIDTH);

    println!("{}", separator);
    println!(
        "{:<40} {:>8} {:>5} {:>7} {:>7} {:>7} {:>7} {:>6}",
        "Example", "DeltaChrF", "Brace", "F1", "Revert", "QaRev", "QaConf", "Cursor"
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

            // Format cursor metric
            let cursor_str = match (score.cursor_exact_match, score.cursor_distance) {
                (Some(true), _) => "✓".to_string(),
                (Some(false), Some(dist)) => format!("±{}", dist),
                (Some(false), None) => "✗".to_string(),
                (None, _) => "-".to_string(),
            };

            println!(
                "{:<40} {:>8.2} {:>5} {:>6.1}% {:>6.1}% {:>7} {:>7} {:>6}",
                truncate_name(&example.spec.name, 40),
                score.delta_chr_f,
                score.braces_disbalance,
                exact_lines.f1() * 100.0,
                score.reversal_ratio * 100.0,
                qa_reverts_str,
                qa_conf_str,
                cursor_str
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
        let avg_cursor_distance = if cursor_distance_count > 0 {
            Some(cursor_distance_sum as f32 / cursor_distance_count as f32)
        } else {
            None
        };

        println!(
            "{:<40} {:>8.2} {:>5.1} {:>6.1}% {:>6.1}% {:>7} {:>7} {:>6}",
            "TOTAL / AVERAGE",
            avg_delta_chr_f,
            braces_disbalance_avg,
            total_exact_lines.f1() * 100.0,
            avg_reversal_ratio * 100.0,
            qa_reverts_str,
            qa_conf_str,
            cursor_str
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
