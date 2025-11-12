use std::{
    io::{IsTerminal, Write},
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use clap::Args;
use collections::HashSet;
use gpui::{AsyncApp, Entity};
use project::Project;
use util::ResultExt as _;
use zeta2::{Zeta, udiff::DiffLine};

use crate::{
    PromptFormat,
    example::{Example, NamedExample},
    headless::ZetaCliAppState,
    paths::print_run_data_dir,
    predict::{CacheMode, PredictionDetails, zeta2_predict},
};

#[derive(Debug, Args)]
pub struct EvaluateArguments {
    example_paths: Vec<PathBuf>,
    #[arg(long, value_enum, default_value_t = PromptFormat::default())]
    prompt_format: PromptFormat,
    #[arg(long)]
    use_expected_context: bool,
    #[clap(long, value_enum, default_value_t = CacheMode::default())]
    cache: CacheMode,
    #[clap(short, long, default_value_t = 1, alias = "repeat")]
    repetitions: u16,
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
        let app_state = app_state.clone();
        let example = NamedExample::load(&path).unwrap();

        cx.spawn(async move |cx| {
            let (project, zetas, _edited_buffers) = example
                .setup_project(&app_state, args.repetitions, cx)
                .await
                .unwrap();

            let tasks = zetas.into_iter().enumerate().map(|(repetition_ix, zeta)| {
                let repetition_ix = (args.repetitions > 1).then(|| repetition_ix as u16);

                let example = example.clone();
                let project = project.clone();

                cx.spawn(async move |cx| {
                    let name = example.name.clone();
                    run_evaluate_one(
                        example,
                        repetition_ix,
                        project,
                        zeta,
                        args.prompt_format,
                        args.use_expected_context,
                        args.cache,
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
    print_run_data_dir(args.repetitions == 1);
}

fn write_aggregated_scores(
    w: &mut impl std::io::Write,
    all_results: &Vec<Vec<Result<EvaluationResult, (anyhow::Error, String, Option<u16>)>>>,
) -> Result<()> {
    let mut successful = Vec::new();
    let mut failed_count = 0;
    writeln!(w, "## Errors\n")?;
    for result in all_results.iter().flatten() {
        match result {
            Ok(eval_result) => successful.push(eval_result),
            Err((err, name, repetition_ix)) => {
                failed_count += 1;
                let err = err
                    .to_string()
                    .replace("<edits", "```xml\n<edits")
                    .replace("</edits>", "</edits>\n```");
                writeln!(
                    w,
                    "### ERROR {name}{}\n\n{err}\n",
                    repetition_ix
                        .map(|ix| format!(" [RUN {ix:03}]"))
                        .unwrap_or_default()
                )?;
            }
        }
    }
    let aggregated_result = EvaluationResult {
        context: Scores::aggregate(successful.iter().map(|r| &r.context)),
        edit_prediction: Scores::aggregate(successful.iter().map(|r| &r.edit_prediction)),
    };

    writeln!(w, "\n{}", "-".repeat(80))?;
    writeln!(w, "\n## TOTAL SCORES")?;
    writeln!(w, "\n### Success Rate")?;
    writeln!(
        w,
        "\nCongratulations! {}/{} ({:.2}%) of runs weren't outright failures 🎉",
        successful.len(),
        successful.len() + failed_count,
        (successful.len() as f64 / (successful.len() + failed_count) as f64) * 100.0
    )?;
    writeln!(w, "{}", aggregated_result)?;

    Ok(())
}

pub async fn run_evaluate_one(
    example: NamedExample,
    repetition_ix: Option<u16>,
    project: Entity<Project>,
    zeta: Entity<Zeta>,
    prompt_format: PromptFormat,
    use_expected_context: bool,
    cache_mode: CacheMode,
    cx: &mut AsyncApp,
) -> Result<EvaluationResult> {
    let predict_result = zeta2_predict(
        example.clone(),
        project,
        zeta,
        repetition_ix,
        prompt_format,
        use_expected_context,
        cache_mode,
        cx,
    )
    .await?;

    let evaluation_result = evaluate(&example.example, &predict_result);

    if repetition_ix.is_none() {
        write_eval_result(
            &example,
            &predict_result,
            &evaluation_result,
            &mut std::io::stdout(),
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
        )
        .log_err();
    }

    anyhow::Ok(evaluation_result)
}

fn write_eval_result(
    example: &NamedExample,
    predictions: &PredictionDetails,
    evaluation_result: &EvaluationResult,
    out: &mut impl Write,
) -> Result<()> {
    writeln!(
        out,
        "## Expected edit prediction:\n\n```diff\n{}\n```\n",
        compare_diffs(&example.example.expected_patch, &predictions.diff)
    )?;
    writeln!(
        out,
        "## Actual edit prediction:\n\n```diff\n{}\n```\n",
        compare_diffs(&predictions.diff, &example.example.expected_patch)
    )?;
    writeln!(out, "{}", evaluation_result)?;

    anyhow::Ok(())
}

#[derive(Debug, Default)]
pub struct EvaluationResult {
    pub edit_prediction: Scores,
    pub context: Scores,
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
        write!(
            f,
            r#"
### Context Scores
{}

### Edit Prediction Scores
{}
"#,
            self.context.to_markdown(),
            self.edit_prediction.to_markdown()
        )
    }
}

pub fn evaluate(example: &Example, preds: &PredictionDetails) -> EvaluationResult {
    let mut eval_result = EvaluationResult::default();

    let actual_context_lines: HashSet<_> = preds
        .excerpts
        .iter()
        .flat_map(|excerpt| {
            excerpt
                .text
                .lines()
                .map(|line| format!("{}: {line}", excerpt.path.display()))
        })
        .collect();

    let mut false_positive_lines = actual_context_lines.clone();

    for entry in &example.expected_context {
        let mut best_alternative_score = Scores::default();

        for alternative in &entry.alternatives {
            let expected: HashSet<_> = alternative
                .excerpts
                .iter()
                .flat_map(|excerpt| {
                    excerpt
                        .text
                        .lines()
                        .map(|line| format!("{}: {line}", excerpt.path.display()))
                })
                .collect();

            let scores = Scores::new(&expected, &actual_context_lines);

            false_positive_lines.retain(|line| !actual_context_lines.contains(line));

            if scores.recall() > best_alternative_score.recall() {
                best_alternative_score = scores;
            }
        }

        eval_result.context.false_negatives += best_alternative_score.false_negatives;
        eval_result.context.true_positives += best_alternative_score.true_positives;
    }

    eval_result.context.false_positives = false_positive_lines.len();

    // todo: alternatives for patches
    let expected_patch_lines = example
        .expected_patch
        .lines()
        .map(DiffLine::parse)
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

    eval_result.edit_prediction = Scores::new(&expected_patch_lines, &actual_patch_lines);
    eval_result
}

/// Return annotated `patch_a` so that:
/// Additions and deletions that are not present in `patch_b` will be highlighted in red.
/// Additions and deletions that are present in `patch_b` will be highlighted in green.
pub fn compare_diffs(patch_a: &str, patch_b: &str) -> String {
    let use_color = std::io::stdout().is_terminal();
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
