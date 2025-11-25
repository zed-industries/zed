use std::{
    collections::HashMap,
    io::{IsTerminal, Write},
    sync::Arc,
};

use anyhow::Result;
use collections::HashSet;
use gpui::{AsyncApp, Entity};
use project::Project;
use util::ResultExt as _;
use zeta::{Zeta, udiff::DiffLine};

use crate::{
    EvaluateArguments, PredictionOptions,
    example::{Example, NamedExample},
    headless::ZetaCliAppState,
    paths::print_run_data_dir,
    predict::{PredictionDetails, perform_predict, setup_zeta},
};

#[derive(Debug)]
pub(crate) struct ExecutionData {
    execution_id: String,
    diff: String,
    reasoning: String,
}

pub async fn run_evaluate(
    args: EvaluateArguments,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) {
    if args.example_paths.is_empty() {
        eprintln!("No examples provided");
        return;
    }

    let all_tasks = args.example_paths.into_iter().map(|path| {
        let options = args.options.clone();
        let app_state = app_state.clone();
        let example = NamedExample::load(&path).expect("Failed to load example");

        cx.spawn(async move |cx| {
            let project = example.setup_project(&app_state, cx).await.unwrap();

            let providers = (0..args.repetitions)
                .map(|_| setup_zeta(args.options.provider, &project, &app_state, cx).unwrap())
                .collect::<Vec<_>>();

            let _edited_buffers = example.apply_edit_history(&project, cx).await.unwrap();

            let tasks = providers
                .into_iter()
                .enumerate()
                .map(move |(repetition_ix, zeta)| {
                    let repetition_ix = (args.repetitions > 1).then(|| repetition_ix as u16);
                    let example = example.clone();
                    let project = project.clone();
                    let options = options.clone();

                    cx.spawn(async move |cx| {
                        let name = example.name.clone();
                        run_evaluate_one(
                            example,
                            repetition_ix,
                            project,
                            zeta,
                            options,
                            !args.skip_prediction,
                            cx,
                        )
                        .await
                        .map_err(|err| (err, name, repetition_ix))
                    })
                });
            futures::future::join_all(tasks).await
        })
    });
    let all_results = futures::future::join_all(all_tasks).await;

    write_aggregated_scores(&mut std::io::stdout(), &all_results).unwrap();
    if let Some(mut output_file) =
        std::fs::File::create(crate::paths::RUN_DIR.join("aggregated_results.md")).log_err()
    {
        write_aggregated_scores(&mut output_file, &all_results).log_err();
    };

    if args.repetitions > 1 {
        if let Err(e) = write_bucketed_analysis(&all_results) {
            eprintln!("Failed to write bucketed analysis: {:?}", e);
        }
    }

    print_run_data_dir(args.repetitions == 1, std::io::stdout().is_terminal());
}

fn write_aggregated_scores(
    w: &mut impl std::io::Write,
    all_results: &Vec<
        Vec<Result<(EvaluationResult, ExecutionData), (anyhow::Error, String, Option<u16>)>>,
    >,
) -> Result<()> {
    let mut successful = Vec::new();
    let mut failed_count = 0;

    for result in all_results.iter().flatten() {
        match result {
            Ok((eval_result, _execution_data)) => successful.push(eval_result),
            Err((err, name, repetition_ix)) => {
                if failed_count == 0 {
                    writeln!(w, "## Errors\n")?;
                }

                failed_count += 1;
                writeln!(w, "{}", fmt_evaluation_error(err, name, repetition_ix))?;
            }
        }
    }

    if successful.len() > 1 {
        let mut edit_predictions = successful
            .iter()
            .filter_map(|r| r.edit_prediction.as_ref())
            .peekable();
        let has_edit_predictions = edit_predictions.peek().is_some();
        let aggregated_result = EvaluationResult {
            edit_prediction: has_edit_predictions.then(|| Scores::aggregate(edit_predictions)),
            prompt_len: successful.iter().map(|r| r.prompt_len).sum::<usize>() / successful.len(),
            generated_len: successful.iter().map(|r| r.generated_len).sum::<usize>()
                / successful.len(),
        };

        writeln!(w, "\n{}", "-".repeat(80))?;
        writeln!(w, "\n## TOTAL SCORES")?;
        writeln!(w, "{:#}", aggregated_result)?;
    }

    if successful.len() + failed_count > 1 {
        writeln!(
            w,
            "\nCongratulations! {}/{} ({:.2}%) of runs weren't outright failures 🎉",
            successful.len(),
            successful.len() + failed_count,
            (successful.len() as f64 / (successful.len() + failed_count) as f64) * 100.0
        )?;
    }

    Ok(())
}

pub async fn run_evaluate_one(
    example: NamedExample,
    repetition_ix: Option<u16>,
    project: Entity<Project>,
    zeta: Entity<Zeta>,
    prediction_options: PredictionOptions,
    predict: bool,
    cx: &mut AsyncApp,
) -> Result<(EvaluationResult, ExecutionData)> {
    let predict_result = perform_predict(
        example.clone(),
        project,
        zeta,
        repetition_ix,
        prediction_options,
        cx,
    )
    .await?;

    let evaluation_result = evaluate(&example.example, &predict_result, predict);

    if repetition_ix.is_none() {
        write_eval_result(
            &example,
            &predict_result,
            &evaluation_result,
            &mut std::io::stdout(),
            std::io::stdout().is_terminal(),
            predict,
        )?;
    }

    if let Some(mut results_file) =
        std::fs::File::create(predict_result.run_example_dir.join("results.md")).log_err()
    {
        write_eval_result(
            &example,
            &predict_result,
            &evaluation_result,
            &mut results_file,
            false,
            predict,
        )
        .log_err();
    }

    let execution_data = ExecutionData {
        execution_id: if let Some(rep_ix) = repetition_ix {
            format!("{:03}", rep_ix)
        } else {
            example.name.clone()
        },
        diff: predict_result.diff.clone(),
        reasoning: std::fs::read_to_string(
            predict_result
                .run_example_dir
                .join("prediction_response.md"),
        )
        .unwrap_or_default(),
    };

    anyhow::Ok((evaluation_result, execution_data))
}

fn write_eval_result(
    example: &NamedExample,
    predictions: &PredictionDetails,
    evaluation_result: &EvaluationResult,
    out: &mut impl Write,
    use_color: bool,
    predict: bool,
) -> Result<()> {
    if predict {
        writeln!(
            out,
            "## Expected edit prediction:\n\n```diff\n{}\n```\n",
            compare_diffs(
                &example.example.expected_patch,
                &predictions.diff,
                use_color
            )
        )?;
        writeln!(
            out,
            "## Actual edit prediction:\n\n```diff\n{}\n```\n",
            compare_diffs(
                &predictions.diff,
                &example.example.expected_patch,
                use_color
            )
        )?;
    }

    writeln!(out, "{:#}", evaluation_result)?;

    anyhow::Ok(())
}

#[derive(Debug, Default)]
pub struct EvaluationResult {
    pub edit_prediction: Option<Scores>,
    pub prompt_len: usize,
    pub generated_len: usize,
}

#[derive(Default, Debug)]
pub struct Scores {
    pub true_positives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
}

impl Scores {
    pub fn new(expected: &HashSet<String>, actual: &HashSet<String>) -> Scores {
        let true_positives = expected.intersection(actual).count();
        let false_positives = actual.difference(expected).count();
        let false_negatives = expected.difference(actual).count();

        Scores {
            true_positives,
            false_positives,
            false_negatives,
        }
    }

    pub fn to_markdown(&self) -> String {
        format!(
            "
Precision       : {:.4}
Recall          : {:.4}
F1 Score        : {:.4}
True Positives  : {}
False Positives : {}
False Negatives : {}",
            self.precision(),
            self.recall(),
            self.f1_score(),
            self.true_positives,
            self.false_positives,
            self.false_negatives
        )
    }

    pub fn aggregate<'a>(scores: impl Iterator<Item = &'a Scores>) -> Scores {
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for score in scores {
            true_positives += score.true_positives;
            false_positives += score.false_positives;
            false_negatives += score.false_negatives;
        }

        Scores {
            true_positives,
            false_positives,
            false_negatives,
        }
    }

    pub fn precision(&self) -> f64 {
        if self.true_positives + self.false_positives == 0 {
            0.0
        } else {
            self.true_positives as f64 / (self.true_positives + self.false_positives) as f64
        }
    }

    pub fn recall(&self) -> f64 {
        if self.true_positives + self.false_negatives == 0 {
            0.0
        } else {
            self.true_positives as f64 / (self.true_positives + self.false_negatives) as f64
        }
    }

    pub fn f1_score(&self) -> f64 {
        let recall = self.recall();
        let precision = self.precision();
        if precision + recall == 0.0 {
            0.0
        } else {
            2.0 * precision * recall / (precision + recall)
        }
    }
}

impl std::fmt::Display for EvaluationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            self.fmt_table(f)
        } else {
            self.fmt_markdown(f)
        }
    }
}

impl EvaluationResult {
    fn fmt_markdown(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prediction) = &self.edit_prediction {
            write!(
                f,
                r#"
                ### Edit Prediction Scores
                {}"#,
                prediction.to_markdown()
            )?;
        }
        Ok(())
    }

    fn fmt_table(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "### Scores\n")?;
        writeln!(
            f,
            "                   Prompt  Generated  TP     FP     FN     Precision   Recall      F1"
        )?;
        writeln!(
            f,
            "───────────────────────────────────────────────────────────────────────────────────────────────"
        )?;
        if let Some(edit_prediction) = &self.edit_prediction {
            writeln!(
                f,
                "Edit Prediction    {:<7} {:<9}  {:<6} {:<6} {:<6} {:>9.2} {:>8.2} {:>7.2}",
                self.prompt_len,
                self.generated_len,
                edit_prediction.true_positives,
                edit_prediction.false_positives,
                edit_prediction.false_negatives,
                edit_prediction.precision() * 100.0,
                edit_prediction.recall() * 100.0,
                edit_prediction.f1_score() * 100.0
            )?;
        }
        Ok(())
    }
}

fn evaluate(example: &Example, preds: &PredictionDetails, predict: bool) -> EvaluationResult {
    let mut eval_result = EvaluationResult {
        prompt_len: preds.prompt_len,
        generated_len: preds.generated_len,
        ..Default::default()
    };

    if predict {
        // todo: alternatives for patches
        let expected_patch = example
            .expected_patch
            .lines()
            .map(DiffLine::parse)
            .collect::<Vec<_>>();
        let expected_patch_lines = expected_patch
            .iter()
            .filter(|line| matches!(line, DiffLine::Addition(_) | DiffLine::Deletion(_)))
            .map(|line| line.to_string())
            .collect();

        let actual_patch_lines = preds
            .diff
            .lines()
            .map(DiffLine::parse)
            .filter(|line| matches!(line, DiffLine::Addition(_) | DiffLine::Deletion(_)))
            .map(|line| line.to_string())
            .collect();

        eval_result.edit_prediction = Some(Scores::new(&expected_patch_lines, &actual_patch_lines));
    }

    eval_result
}

/// Return annotated `patch_a` so that:
/// Additions and deletions that are not present in `patch_b` will be highlighted in red.
/// Additions and deletions that are present in `patch_b` will be highlighted in green.
pub fn compare_diffs(patch_a: &str, patch_b: &str, use_color: bool) -> String {
    let green = if use_color { "\x1b[32m✓ " } else { "" };
    let red = if use_color { "\x1b[31m✗ " } else { "" };
    let neutral = if use_color { "  " } else { "" };
    let reset = if use_color { "\x1b[0m" } else { "" };
    let lines_a = patch_a.lines().map(DiffLine::parse);
    let lines_b: Vec<_> = patch_b.lines().map(DiffLine::parse).collect();

    let annotated = lines_a
        .map(|line| match line {
            DiffLine::Addition(_) | DiffLine::Deletion(_) => {
                if lines_b.contains(&line) {
                    format!("{green}{line}{reset}")
                } else {
                    format!("{red}{line}{reset}")
                }
            }
            _ => format!("{neutral}{line}{reset}"),
        })
        .collect::<Vec<String>>();

    annotated.join("\n")
}

fn write_bucketed_analysis(
    all_results: &Vec<
        Vec<Result<(EvaluationResult, ExecutionData), (anyhow::Error, String, Option<u16>)>>,
    >,
) -> Result<()> {
    #[derive(Debug)]
    struct EditBucket {
        diff: String,
        is_correct: bool,
        execution_indices: Vec<String>,
        reasoning_samples: Vec<String>,
    }

    let mut total_executions = 0;
    let mut empty_predictions = Vec::new();
    let mut errors = Vec::new();

    let mut buckets: HashMap<String, EditBucket> = HashMap::new();

    for result in all_results.iter().flatten() {
        total_executions += 1;

        let (evaluation_result, execution_data) = match result {
            Ok((eval_result, execution_data)) => {
                if execution_data.diff.is_empty() {
                    empty_predictions.push(execution_data);
                    continue;
                }
                (eval_result, execution_data)
            }
            Err(err) => {
                errors.push(err);
                continue;
            }
        };

        buckets
            .entry(execution_data.diff.clone())
            .and_modify(|bucket| {
                bucket
                    .execution_indices
                    .push(execution_data.execution_id.clone());
                bucket
                    .reasoning_samples
                    .push(execution_data.reasoning.clone());
            })
            .or_insert_with(|| EditBucket {
                diff: execution_data.diff.clone(),
                is_correct: {
                    evaluation_result
                        .edit_prediction
                        .as_ref()
                        .map_or(false, |edit_prediction| {
                            edit_prediction.false_positives == 0
                                && edit_prediction.false_negatives == 0
                                && edit_prediction.true_positives > 0
                        })
                },
                execution_indices: vec![execution_data.execution_id.clone()],
                reasoning_samples: vec![execution_data.reasoning.clone()],
            });
    }

    let mut sorted_buckets = buckets.into_values().collect::<Vec<_>>();
    sorted_buckets.sort_by(|a, b| match (a.is_correct, b.is_correct) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => b.execution_indices.len().cmp(&a.execution_indices.len()),
    });

    let output_path = crate::paths::RUN_DIR.join("bucketed_analysis.md");
    let mut output = std::fs::File::create(&output_path)?;

    writeln!(output, "# Bucketed Edit Analysis\n")?;

    writeln!(output, "## Summary\n")?;
    writeln!(output, "- **Total executions**: {}", total_executions)?;

    let correct_count: usize = sorted_buckets
        .iter()
        .filter(|b| b.is_correct)
        .map(|b| b.execution_indices.len())
        .sum();

    let incorrect_count: usize = sorted_buckets
        .iter()
        .filter(|b| !b.is_correct)
        .map(|b| b.execution_indices.len())
        .sum();

    writeln!(
        output,
        "- **Correct predictions**: {} ({:.1}%)",
        correct_count,
        (correct_count as f64 / total_executions as f64) * 100.0
    )?;

    writeln!(
        output,
        "- **Incorrect predictions**: {} ({:.1}%)",
        incorrect_count,
        (incorrect_count as f64 / total_executions as f64) * 100.0
    )?;

    writeln!(
        output,
        "- **No Predictions**: {} ({:.1}%)",
        empty_predictions.len(),
        (empty_predictions.len() as f64 / total_executions as f64) * 100.0
    )?;

    let unique_incorrect = sorted_buckets.iter().filter(|b| !b.is_correct).count();
    writeln!(
        output,
        "- **Unique incorrect edit patterns**: {}\n",
        unique_incorrect
    )?;

    writeln!(output, "---\n")?;

    for (idx, bucket) in sorted_buckets.iter().filter(|b| b.is_correct).enumerate() {
        if idx == 0 {
            writeln!(
                output,
                "## Correct Predictions ({} occurrences)\n",
                bucket.execution_indices.len()
            )?;
        }

        writeln!(output, "**Predicted Edit:**\n")?;
        writeln!(output, "```diff")?;
        writeln!(output, "{}", bucket.diff)?;
        writeln!(output, "```\n")?;

        writeln!(
            output,
            "**Executions:** {}\n",
            bucket.execution_indices.join(", ")
        )?;
        writeln!(output, "---\n")?;
    }

    for (idx, bucket) in sorted_buckets.iter().filter(|b| !b.is_correct).enumerate() {
        writeln!(
            output,
            "## Incorrect Prediction #{} ({} occurrences)\n",
            idx + 1,
            bucket.execution_indices.len()
        )?;

        writeln!(output, "**Predicted Edit:**\n")?;
        writeln!(output, "```diff")?;
        writeln!(output, "{}", bucket.diff)?;
        writeln!(output, "```\n")?;

        writeln!(
            output,
            "**Executions:** {}\n",
            bucket.execution_indices.join(", ")
        )?;

        for (exec_id, reasoning) in bucket
            .execution_indices
            .iter()
            .zip(bucket.reasoning_samples.iter())
        {
            writeln!(output, "{}", fmt_execution(exec_id, reasoning))?;
        }

        writeln!(output, "\n---\n")?;
    }

    if !empty_predictions.is_empty() {
        writeln!(
            output,
            "## No Predictions ({} occurrences)\n",
            empty_predictions.len()
        )?;

        for execution_data in &empty_predictions {
            writeln!(
                output,
                "{}",
                fmt_execution(&execution_data.execution_id, &execution_data.reasoning)
            )?;
        }
        writeln!(output, "\n---\n")?;
    }

    if !errors.is_empty() {
        writeln!(output, "## Errors ({} occurrences)\n", errors.len())?;

        for (err, name, repetition_ix) in &errors {
            writeln!(output, "{}", fmt_evaluation_error(err, name, repetition_ix))?;
        }
        writeln!(output, "\n---\n")?;
    }

    fn fmt_execution(exec_id: &str, reasoning: &str) -> String {
        let exec_content = format!(
            "\n### Execution {} `{}/{}/prediction_response.md`{}",
            exec_id,
            crate::paths::RUN_DIR.display(),
            exec_id,
            indent_text(&format!("\n\n```\n{}\n```\n", reasoning,), 2)
        );
        indent_text(&exec_content, 2)
    }

    fn indent_text(text: &str, spaces: usize) -> String {
        let indent = " ".repeat(spaces);
        text.lines()
            .collect::<Vec<_>>()
            .join(&format!("\n{}", indent))
    }

    Ok(())
}

fn fmt_evaluation_error(err: &anyhow::Error, name: &str, repetition_ix: &Option<u16>) -> String {
    let err = format!("{err:?}")
        .replace("<edits", "```xml\n<edits")
        .replace("</edits>", "</edits>\n```");
    format!(
        "### ERROR {name}{}\n\n{err}\n",
        repetition_ix
            .map(|ix| format!(" [RUN {ix:03}]"))
            .unwrap_or_default()
    )
}
