use crate::{
    PredictArgs, PredictionProvider,
    example::Example,
    format_prompt::TeacherPrompt,
    headless::EpAppState,
    parse_output::parse_prediction_output,
    predict::run_prediction,
    progress::{ExampleProgress, Step},
};
use anyhow::Context as _;
use edit_prediction_context::limit_retrieved_context_to_bytes;
use edit_prediction_metrics::{
    ActualPredictionCursor, Excerpt, PredictionReversalContext, PredictionScoringInput,
};
use gpui::{AppContext as _, AsyncApp};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;
use zeta_prompt::{ContextSource, RelatedFile};

pub const EVAL_RELATED_CONTEXT_TOKENS_LIMIT: usize = 4000;

pub async fn run_scoring(
    example: &mut Example,
    args: &PredictArgs,
    app_state: Arc<EpAppState>,
    example_progress: &ExampleProgress,
    cx: AsyncApp,
    allow_missing_predictions: bool,
    retrieved_context_byte_limit: Option<usize>,
    context_source_filter: Option<Vec<ContextSource>>,
) -> anyhow::Result<()> {
    if !(allow_missing_predictions && args.provider.is_none() && example.predictions.is_empty()) {
        run_prediction(example, args, app_state, example_progress, cx.clone()).await?;
    }

    let progress = example_progress.start(Step::Score);

    progress.set_substatus("computing metrics");
    let example_for_scoring = example.clone();
    example.score = cx
        .background_spawn(async move {
            let prompt_inputs = example_for_scoring
                .prompt_inputs
                .as_ref()
                .context("prompt_inputs is required for scoring - run prediction first or ensure JSON includes prompt_inputs")?;
            let original_text: &str = prompt_inputs.cursor_excerpt.as_ref();
            let expected_patches_with_cursors = example_for_scoring
                .spec
                .expected_patches_with_cursor_positions();

            let old_editable_region = if let Some(p) = example_for_scoring.prompt.as_ref() {
                if matches!(
                    p.provider,
                    PredictionProvider::Teacher(_, _) | PredictionProvider::TeacherNonBatching(_, _)
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

            let prepared_expected_patches = edit_prediction_metrics::prepare_expected_patches(
                &expected_patches_with_cursors,
                original_text,
                old_editable_region.as_deref(),
            )
            .with_context(|| {
                format!(
                    "Expected patch did not apply for {}",
                    example_for_scoring.spec.name
                )
            })?;

            let cursor_path = example_for_scoring.spec.cursor_path.as_ref();
            let context = context_excerpts(
                &example_for_scoring,
                prompt_inputs,
                retrieved_context_byte_limit,
                context_source_filter.as_deref(),
            );

            let mut scores = vec![];
            if allow_missing_predictions && example_for_scoring.predictions.is_empty() {
                scores.push(edit_prediction_metrics::score_prediction(
                    PredictionScoringInput {
                        original_text,
                        expected_patches: &prepared_expected_patches,
                        actual_patch: None,
                        actual_cursor: None,
                        reversal_context: Some(PredictionReversalContext {
                            edit_history: &prompt_inputs.events,
                            excerpt_start_row: prompt_inputs.excerpt_start_row,
                            cursor_path,
                        }),
                        cumulative_logprob: None,
                        avg_logprob: None,
                        context: Some(&context),
                    },
                ));
            }

            for prediction in &example_for_scoring.predictions {
                let actual_patch = prediction.actual_patch.clone().or_else(|| {
                    parse_prediction_output(
                        &example_for_scoring,
                        &prediction.actual_output,
                        prediction.provider,
                    )
                    .ok()
                    .map(|(patch, _)| patch)
                });

                let actual_cursor = prediction.actual_cursor.as_ref().map(|cursor| {
                    ActualPredictionCursor {
                        row: cursor.row,
                        editable_region_offset: cursor.editable_region_offset,
                    }
                });

                scores.push(edit_prediction_metrics::score_prediction(
                    PredictionScoringInput {
                        original_text,
                        expected_patches: &prepared_expected_patches,
                        actual_patch: actual_patch.as_deref(),
                        actual_cursor,
                        reversal_context: Some(PredictionReversalContext {
                            edit_history: &prompt_inputs.events,
                            excerpt_start_row: prompt_inputs.excerpt_start_row,
                            cursor_path,
                        }),
                        cumulative_logprob: prediction.cumulative_logprob,
                        avg_logprob: prediction.avg_logprob,
                        context: Some(&context),
                    },
                ));
            }

            anyhow::Ok(scores)
        })
        .await?;
    Ok(())
}

pub fn run_context_coverage_scoring(
    example: &mut Example,
    example_progress: &ExampleProgress,
    retrieved_context_byte_limit: Option<usize>,
    context_source_filter: Option<&[ContextSource]>,
) -> anyhow::Result<()> {
    let progress = example_progress.start(Step::Score);

    progress.set_substatus("computing context coverage");
    let prompt_inputs = example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs is required for context coverage scoring")?;
    let context = context_excerpts(
        example,
        prompt_inputs,
        retrieved_context_byte_limit,
        context_source_filter,
    );

    let editable_context_coverage = example
        .spec
        .expected_patches_with_cursor_positions()
        .iter()
        .map(|(expected_patch, _)| {
            edit_prediction_metrics::editable_context_coverage(expected_patch, &context)
        })
        .max_by(|left, right| {
            left.lines_f1
                .total_cmp(&right.lines_f1)
                .then_with(|| left.files_f1.total_cmp(&right.files_f1))
        });

    let mut score = edit_prediction_metrics::PredictionScore::zero();
    score.editable_context_coverage = editable_context_coverage;
    example.score = vec![score];

    Ok(())
}

fn context_excerpts(
    _example: &Example,
    prompt_inputs: &zeta_prompt::ZetaPromptInput,
    retrieved_context_byte_limit: Option<usize>,
    context_source_filter: Option<&[ContextSource]>,
) -> Vec<Excerpt> {
    let mut context = Vec::new();

    if let Some(excerpt_start_row) = prompt_inputs.excerpt_start_row {
        let row_count = prompt_inputs.cursor_excerpt.lines().count() as u32;

        context.push(Excerpt {
            path: prompt_inputs.cursor_path.to_string_lossy().to_string(),
            row_range: excerpt_start_row..excerpt_start_row.saturating_add(row_count),
            content: prompt_inputs.cursor_excerpt.to_string(),
        });
    }

    if let Some(related_files) = &prompt_inputs.related_files {
        let related_files = filtered_related_files(related_files, context_source_filter);
        let related_files = if let Some(max_bytes) = retrieved_context_byte_limit {
            limit_retrieved_context_to_bytes(&related_files, max_bytes)
        } else {
            related_files
        };
        for related_file in &related_files {
            for excerpt in &related_file.excerpts {
                // First component is a project name which is not present in expected patch, skip it
                let path = related_file
                    .path
                    .iter()
                    .skip(1)
                    .collect::<std::path::PathBuf>()
                    .to_string_lossy()
                    .to_string();
                context.push(Excerpt {
                    path,
                    row_range: excerpt.row_range.clone(),
                    content: excerpt.text.to_string(),
                });
            }
        }
    }

    context
}

fn filtered_related_files(
    related_files: &[RelatedFile],
    context_source_filter: Option<&[ContextSource]>,
) -> Vec<RelatedFile> {
    let Some(context_source_filter) = context_source_filter else {
        return related_files.to_vec();
    };

    related_files
        .iter()
        .filter_map(|related_file| {
            let excerpts = related_file
                .excerpts
                .iter()
                .filter(|excerpt| context_source_filter.contains(&excerpt.context_source))
                .cloned()
                .collect::<Vec<_>>();
            if excerpts.is_empty() {
                None
            } else {
                Some(RelatedFile {
                    path: related_file.path.clone(),
                    max_row: related_file.max_row,
                    excerpts,
                    in_open_source_repo: related_file.in_open_source_repo,
                })
            }
        })
        .collect()
}

fn retrieved_context_bytes(
    example: &Example,
    retrieved_context_byte_limit: Option<usize>,
    context_source_filter: Option<&[ContextSource]>,
) -> Option<usize> {
    let related_files = example.prompt_inputs.as_ref()?.related_files.as_ref()?;
    let related_files = filtered_related_files(related_files, context_source_filter);
    let related_files = if let Some(max_bytes) = retrieved_context_byte_limit {
        limit_retrieved_context_to_bytes(&related_files, max_bytes)
    } else {
        related_files
    };
    Some(
        related_files
            .iter()
            .flat_map(|file| file.excerpts.iter())
            .map(|excerpt| excerpt.text.len())
            .sum::<usize>(),
    )
}

pub fn print_report(
    examples: &[Example],
    verbose: bool,
    context_only: bool,
    retrieved_context_byte_limit: Option<usize>,
    context_source_filter: Option<&[ContextSource]>,
) {
    const MAX_EXAMPLES_DEFAULT: usize = 20;
    use crate::metrics::ClassificationMetrics;

    const LINE_WIDTH: usize = 101;

    if context_only {
        print_context_coverage_report(
            examples,
            verbose,
            retrieved_context_byte_limit,
            context_source_filter,
        );
        return;
    }

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
    let mut total_delta_chr_f = ClassificationMetrics::default();
    let mut total_delta_chr_f_precision = 0.0;
    let mut total_delta_chr_f_recall = 0.0;
    let mut delta_chr_f_beta = 0.0;
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
    let mut wrong_editable_region_count: usize = 0;
    let mut wrong_editable_region_total: usize = 0;
    let mut isolated_whitespace_count: usize = 0;
    let mut kept_rate_sum: f64 = 0.0;
    let mut kept_rate_count: usize = 0;
    let mut kept_chars_total: usize = 0;
    let mut correctly_deleted_chars_total: usize = 0;
    let mut discarded_chars_total: usize = 0;
    let mut recall_rate_sum: f64 = 0.0;
    let mut recall_rate_count: usize = 0;
    let mut editable_context_coverage_count: usize = 0;
    let mut editable_context_lines_precision_sum = 0.0;
    let mut editable_context_lines_recall_sum = 0.0;
    let mut editable_context_lines_f1_sum = 0.0;
    let mut editable_context_files_precision_sum = 0.0;
    let mut editable_context_files_recall_sum = 0.0;
    let mut editable_context_files_f1_sum = 0.0;
    let mut total_editable_context_lines = ClassificationMetrics::default();
    let mut total_editable_context_files = ClassificationMetrics::default();
    let mut patch_inserted_tokens: Vec<usize> = Vec::new();
    let mut patch_deleted_tokens: Vec<usize> = Vec::new();
    let mut predictions_with_patch: usize = 0;
    let mut retrieved_context_bytes_sum = 0.0;
    let mut retrieved_context_bytes_count = 0;

    let mut printed_lines: usize = 0;
    let mut skipped_lines: usize = 0;

    for example in examples {
        if let Some(bytes) =
            retrieved_context_bytes(example, retrieved_context_byte_limit, context_source_filter)
        {
            retrieved_context_bytes_sum += bytes as f64;
            retrieved_context_bytes_count += 1;
        }

        for (score_idx, score) in example.score.iter().enumerate() {
            let exact_lines = score.exact_lines_counts();

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

            if verbose || printed_lines < MAX_EXAMPLES_DEFAULT {
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
                printed_lines += 1;
            } else {
                skipped_lines += 1;
            }

            all_delta_chr_f_scores.push(score.delta_chr_f);
            all_reversal_ratios.push(score.reversal_ratio);
            total_scores += 1;
            braces_disbalance_sum += score.braces_disbalance;
            total_delta_chr_f.accumulate(&score.delta_chr_f_counts());
            total_delta_chr_f_precision += score.delta_chr_f_precision;
            total_delta_chr_f_recall += score.delta_chr_f_recall;
            delta_chr_f_beta = score.delta_chr_f_beta;
            total_exact_lines.accumulate(&score.exact_lines_counts());

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

            // Accumulate kept and recall rate metrics
            if let Some(kr) = score.kept_rate {
                kept_rate_sum += kr;
                kept_rate_count += 1;
            }
            if let Some(kept_chars) = score.kept_chars {
                kept_chars_total += kept_chars;
            }
            if let Some(correctly_deleted_chars) = score.correctly_deleted_chars {
                correctly_deleted_chars_total += correctly_deleted_chars;
            }
            if let Some(discarded_chars) = score.discarded_chars {
                discarded_chars_total += discarded_chars;
            }
            if let Some(rr) = score.recall_rate {
                recall_rate_sum += rr;
                recall_rate_count += 1;
            }
            if let Some(coverage) = &score.editable_context_coverage {
                editable_context_coverage_count += 1;
                editable_context_lines_precision_sum += coverage.lines_precision;
                editable_context_lines_recall_sum += coverage.lines_recall;
                editable_context_lines_f1_sum += coverage.lines_f1;
                editable_context_files_precision_sum += coverage.files_precision;
                editable_context_files_recall_sum += coverage.files_recall;
                editable_context_files_f1_sum += coverage.files_f1;
                total_editable_context_lines.accumulate(&ClassificationMetrics {
                    true_positives: coverage.lines_tp,
                    false_positives: coverage.lines_fp,
                    false_negatives: coverage.lines_fn,
                });
                total_editable_context_files.accumulate(&ClassificationMetrics {
                    true_positives: coverage.files_tp,
                    false_positives: coverage.files_fp,
                    false_negatives: coverage.files_fn,
                });
            }

            // Accumulate token change metrics (only for predictions that produced a patch)
            let has_patch = example
                .predictions
                .get(score_idx)
                .and_then(|p| p.actual_patch.as_ref())
                .is_some_and(|p| !p.is_empty());
            if has_patch {
                predictions_with_patch += 1;
                patch_inserted_tokens.push(score.inserted_tokens);
                patch_deleted_tokens.push(score.deleted_tokens);
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

    if skipped_lines > 0 {
        println!(
            "{:<40} (use --verbose to see all {} examples)",
            format!("... and {} more", skipped_lines),
            printed_lines + skipped_lines
        );
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
        println!(
            "Delta chrF (β={:.1}): TP={}, FP={}, FN={}, P={:.1}%, R={:.1}%",
            delta_chr_f_beta,
            total_delta_chr_f.true_positives,
            total_delta_chr_f.false_positives,
            total_delta_chr_f.false_negatives,
            total_delta_chr_f_precision / total_scores as f64 * 100.0,
            total_delta_chr_f_recall / total_scores as f64 * 100.0
        );

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

        // Print isolated whitespace metrics
        if total_scores > 0 {
            println!("Isolated whitespace changes: {}", isolated_ws_str);
        }

        // Print kept and recall rate metrics
        if kept_rate_count > 0 {
            let avg_kept_rate = kept_rate_sum / kept_rate_count as f64;
            println!(
                "Kept rate: {:.1}% avg ({} evaluated, kept chars: {}, correctly deleted chars: {}, discarded chars: {})",
                avg_kept_rate * 100.0,
                kept_rate_count,
                kept_chars_total,
                correctly_deleted_chars_total,
                discarded_chars_total
            );
        }
        if recall_rate_count > 0 {
            let avg_recall_rate = recall_rate_sum / recall_rate_count as f64;
            println!(
                "Recall rate: {:.1}% avg ({} evaluated)",
                avg_recall_rate * 100.0,
                recall_rate_count
            );
        }
        if retrieved_context_bytes_count > 0 {
            println!(
                "Retrieved context size: {:.0} bytes avg ({} examples)",
                retrieved_context_bytes_sum / retrieved_context_bytes_count as f64,
                retrieved_context_bytes_count
            );
        }
        if editable_context_coverage_count > 0 {
            let count = editable_context_coverage_count as f64;
            println!(
                "Editable context lines: P={:.1}%, R={:.1}%, F1={:.1}% avg ({} evaluated, TP={}, FP={}, FN={})",
                editable_context_lines_precision_sum / count * 100.0,
                editable_context_lines_recall_sum / count * 100.0,
                editable_context_lines_f1_sum / count * 100.0,
                editable_context_coverage_count,
                total_editable_context_lines.true_positives,
                total_editable_context_lines.false_positives,
                total_editable_context_lines.false_negatives
            );
            println!(
                "Editable context files: P={:.1}%, R={:.1}%, F1={:.1}% avg ({} evaluated, TP={}, FP={}, FN={})",
                editable_context_files_precision_sum / count * 100.0,
                editable_context_files_recall_sum / count * 100.0,
                editable_context_files_f1_sum / count * 100.0,
                editable_context_coverage_count,
                total_editable_context_files.true_positives,
                total_editable_context_files.false_positives,
                total_editable_context_files.false_negatives
            );
        }

        // Print token change percentile summary (only for predictions with a patch)
        if !patch_inserted_tokens.is_empty() {
            patch_inserted_tokens.sort_unstable();
            patch_deleted_tokens.sort_unstable();
            let mut patch_total_tokens: Vec<usize> = patch_inserted_tokens
                .iter()
                .zip(patch_deleted_tokens.iter())
                .map(|(i, d)| i + d)
                .collect();
            patch_total_tokens.sort_unstable();

            let patch_rate = predictions_with_patch as f32 / total_scores as f32 * 100.0;
            println!();
            println!(
                "Token changes ({}/{} predictions produced a patch, {:.1}% — table includes only those)",
                predictions_with_patch, total_scores, patch_rate
            );
            println!(
                "{:<20} {:>8} {:>8} {:>8} {:>8} {:>8}",
                "", "p25", "p50", "p75", "p90", "p99"
            );
            println!("{}", "─".repeat(LINE_WIDTH));
            println!(
                "{:<20} {:>8} {:>8} {:>8} {:>8} {:>8}",
                "Inserted tokens",
                percentile(&patch_inserted_tokens, 25),
                percentile(&patch_inserted_tokens, 50),
                percentile(&patch_inserted_tokens, 75),
                percentile(&patch_inserted_tokens, 90),
                percentile(&patch_inserted_tokens, 99),
            );
            println!(
                "{:<20} {:>8} {:>8} {:>8} {:>8} {:>8}",
                "Deleted tokens",
                percentile(&patch_deleted_tokens, 25),
                percentile(&patch_deleted_tokens, 50),
                percentile(&patch_deleted_tokens, 75),
                percentile(&patch_deleted_tokens, 90),
                percentile(&patch_deleted_tokens, 99),
            );
            println!(
                "{:<20} {:>8} {:>8} {:>8} {:>8} {:>8}",
                "Total tokens",
                percentile(&patch_total_tokens, 25),
                percentile(&patch_total_tokens, 50),
                percentile(&patch_total_tokens, 75),
                percentile(&patch_total_tokens, 90),
                percentile(&patch_total_tokens, 99),
            );
        }
    }

    println!("\n");
}

fn print_context_coverage_report(
    examples: &[Example],
    verbose: bool,
    retrieved_context_byte_limit: Option<usize>,
    context_source_filter: Option<&[ContextSource]>,
) {
    const MAX_EXAMPLES_DEFAULT: usize = 20;
    const LINE_WIDTH: usize = 120;

    use crate::metrics::ClassificationMetrics;

    let separator = "─".repeat(LINE_WIDTH);
    println!("{}", separator);
    println!(
        "{:<40} {:>6} {:>6} {:>6} {:>5} {:>5} {:>5} {:>6} {:>6} {:>6} {:>5} {:>5} {:>5}",
        "Example",
        "LineP",
        "LineR",
        "LineF1",
        "LTP",
        "LFP",
        "LFN",
        "FileP",
        "FileR",
        "FileF1",
        "FTP",
        "FFP",
        "FFN"
    );
    println!("{}", separator);

    let mut total_lines = ClassificationMetrics::default();
    let mut total_files = ClassificationMetrics::default();
    let mut line_precision_sum = 0.0;
    let mut line_recall_sum = 0.0;
    let mut line_f1_sum = 0.0;
    let mut file_precision_sum = 0.0;
    let mut file_recall_sum = 0.0;
    let mut file_f1_sum = 0.0;
    let mut total_scores = 0;
    let mut retrieved_context_bytes_sum = 0.0;
    let mut retrieved_context_bytes_count = 0;
    let mut printed_lines = 0;
    let mut skipped_lines = 0;

    for example in examples {
        if let Some(bytes) =
            retrieved_context_bytes(example, retrieved_context_byte_limit, context_source_filter)
        {
            retrieved_context_bytes_sum += bytes as f64;
            retrieved_context_bytes_count += 1;
        }

        for score in &example.score {
            let Some(coverage) = &score.editable_context_coverage else {
                continue;
            };

            if verbose || printed_lines < MAX_EXAMPLES_DEFAULT {
                println!(
                    "{:<40} {:>5.1}% {:>5.1}% {:>5.1}% {:>5} {:>5} {:>5} {:>5.1}% {:>5.1}% {:>5.1}% {:>5} {:>5} {:>5}",
                    truncate_name(&example.spec.name, 40),
                    coverage.lines_precision * 100.0,
                    coverage.lines_recall * 100.0,
                    coverage.lines_f1 * 100.0,
                    coverage.lines_tp,
                    coverage.lines_fp,
                    coverage.lines_fn,
                    coverage.files_precision * 100.0,
                    coverage.files_recall * 100.0,
                    coverage.files_f1 * 100.0,
                    coverage.files_tp,
                    coverage.files_fp,
                    coverage.files_fn
                );
                printed_lines += 1;
            } else {
                skipped_lines += 1;
            }

            total_scores += 1;
            line_precision_sum += coverage.lines_precision;
            line_recall_sum += coverage.lines_recall;
            line_f1_sum += coverage.lines_f1;
            file_precision_sum += coverage.files_precision;
            file_recall_sum += coverage.files_recall;
            file_f1_sum += coverage.files_f1;
            total_lines.accumulate(&ClassificationMetrics {
                true_positives: coverage.lines_tp,
                false_positives: coverage.lines_fp,
                false_negatives: coverage.lines_fn,
            });
            total_files.accumulate(&ClassificationMetrics {
                true_positives: coverage.files_tp,
                false_positives: coverage.files_fp,
                false_negatives: coverage.files_fn,
            });
        }
    }

    if skipped_lines > 0 {
        println!(
            "{:<40} (use --verbose to see all {} examples)",
            format!("... and {} more", skipped_lines),
            printed_lines + skipped_lines
        );
    }

    println!("{}", separator);

    if total_scores > 0 {
        let count = total_scores as f64;
        println!(
            "{:<40} {:>5.1}% {:>5.1}% {:>5.1}% {:>5} {:>5} {:>5} {:>5.1}% {:>5.1}% {:>5.1}% {:>5} {:>5} {:>5}",
            "TOTAL / AVERAGE",
            line_precision_sum / count * 100.0,
            line_recall_sum / count * 100.0,
            line_f1_sum / count * 100.0,
            total_lines.true_positives,
            total_lines.false_positives,
            total_lines.false_negatives,
            file_precision_sum / count * 100.0,
            file_recall_sum / count * 100.0,
            file_f1_sum / count * 100.0,
            total_files.true_positives,
            total_files.false_positives,
            total_files.false_negatives
        );
        println!("{}", separator);
        println!(
            "Evaluated editable context coverage for {} examples",
            total_scores
        );
        if retrieved_context_bytes_count > 0 {
            println!(
                "Retrieved context size: {:.0} bytes avg ({} examples)",
                retrieved_context_bytes_sum / retrieved_context_bytes_count as f64,
                retrieved_context_bytes_count
            );
        }
    }

    println!("\n");
}

fn percentile(sorted_values: &[usize], p: usize) -> usize {
    if sorted_values.is_empty() {
        return 0;
    }
    let idx = (p as f64 / 100.0 * (sorted_values.len() as f64 - 1.0)).round() as usize;
    sorted_values[idx.min(sorted_values.len() - 1)]
}

fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[..max_len - 3])
    }
}

pub type SummaryJson = edit_prediction_metrics::SummaryJson;

pub fn compute_summary(
    examples: &[Example],
    retrieved_context_byte_limit: Option<usize>,
    context_source_filter: Option<&[ContextSource]>,
) -> SummaryJson {
    edit_prediction_metrics::compute_summary(examples.iter().flat_map(|example| {
        let retrieved_context_bytes =
            retrieved_context_bytes(example, retrieved_context_byte_limit, context_source_filter);
        example
            .score
            .iter()
            .enumerate()
            .map(move |(score_idx, score)| {
                let qa = example
                    .qa
                    .get(score_idx)
                    .and_then(|qa| qa.as_ref())
                    .map(|qa| edit_prediction_metrics::QaSummaryData {
                        reverts_edits: qa.reverts_edits,
                        confidence: qa.confidence,
                    });
                let retrieved_context_bytes = (score_idx == 0)
                    .then_some(retrieved_context_bytes)
                    .flatten();

                edit_prediction_metrics::PredictionSummaryInput {
                    score,
                    qa,
                    retrieved_context_bytes,
                }
            })
    }))
}

pub fn write_summary_json(
    examples: &[Example],
    path: &Path,
    retrieved_context_byte_limit: Option<usize>,
    context_source_filter: Option<&[ContextSource]>,
) -> anyhow::Result<()> {
    let summary = compute_summary(
        examples,
        retrieved_context_byte_limit,
        context_source_filter,
    );
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
    use edit_prediction::example_spec::ExampleSpec;
    use edit_prediction_metrics::PredictionScore;
    use std::path::Path;
    use zeta_prompt::{ExcerptRanges, RelatedExcerpt, ZetaPromptInput};

    #[test]
    fn summary_includes_limited_filtered_retrieved_context_bytes_once_per_example() {
        let examples = vec![
            example_with_related_files(
                Some(vec![RelatedFile {
                    path: Path::new("project/src/lib.rs").into(),
                    max_row: 10,
                    excerpts: vec![
                        related_excerpt("abcd", 0..1, 0, ContextSource::CurrentFile),
                        related_excerpt("ignored by source filter", 1..2, 1, ContextSource::Lsp),
                        related_excerpt("efghij", 2..3, 2, ContextSource::CurrentFile),
                    ],
                    in_open_source_repo: false,
                }]),
                2,
            ),
            example_with_related_files(None, 1),
        ];

        let summary = compute_summary(&examples, Some(10), Some(&[ContextSource::CurrentFile]));

        assert_eq!(summary.total_examples, 3);
        assert_eq!(summary.avg_retrieved_context_bytes, Some(10.0));
        assert_eq!(summary.total_retrieved_context_bytes, Some(10));
        assert_eq!(summary.retrieved_context_examples, Some(1));
    }

    fn example_with_related_files(
        related_files: Option<Vec<RelatedFile>>,
        score_count: usize,
    ) -> Example {
        Example {
            spec: ExampleSpec {
                name: "example".to_string(),
                repository_url: "https://github.com/zed-industries/zed.git".to_string(),
                revision: "revision".to_string(),
                tags: Vec::new(),
                reasoning: None,
                uncommitted_diff: String::new(),
                recently_opened_files: Vec::new(),
                recently_viewed_files: Vec::new(),
                uncommitted_diff_contains_edit_history: false,
                cursor_path: Path::new("project/src/main.rs").into(),
                cursor_position: String::new(),
                edit_history: String::new(),
                expected_patches: Vec::new(),
                rejected_patch: None,
                telemetry: None,
                human_feedback: Vec::new(),
                rating: None,
            },
            prompt_inputs: Some(ZetaPromptInput {
                cursor_path: Path::new("project/src/main.rs").into(),
                cursor_excerpt: "".into(),
                cursor_offset_in_excerpt: 0,
                excerpt_start_row: None,
                events: Vec::new(),
                related_files,
                active_buffer_diagnostics: Vec::new(),
                excerpt_ranges: ExcerptRanges::default(),
                syntax_ranges: None,
                in_open_source_repo: false,
                can_collect_data: false,
                repo_url: None,
            }),
            prompt: None,
            predictions: Vec::new(),
            score: vec![PredictionScore::zero(); score_count],
            qa: Vec::new(),
            zed_version: None,
            state: None,
        }
    }

    fn related_excerpt(
        text: &str,
        row_range: std::ops::Range<u32>,
        order: usize,
        context_source: ContextSource,
    ) -> RelatedExcerpt {
        RelatedExcerpt {
            row_range,
            text: text.into(),
            order,
            context_source,
        }
    }
}
