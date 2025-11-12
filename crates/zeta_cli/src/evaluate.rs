use std::{io::IsTerminal, path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::Args;
use collections::HashSet;
use gpui::AsyncApp;
use zeta2::udiff::DiffLine;

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
    #[clap(long, default_value_t = 1)]
    repeat: u16,
}

pub async fn run_evaluate(
    args: EvaluateArguments,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) {
    let example_len = args.example_paths.len();

    let all_tasks = args.example_paths.into_iter().map(|path| {
        let app_state = app_state.clone();
        let example = NamedExample::load(&path).unwrap();

        cx.spawn(async move |cx| {
            let worktree_path = example.setup_worktree().await.unwrap();

            let tasks = (0..args.repeat).map(|repetition_ix| {
                let repetition_ix = (args.repeat > 1).then(|| repetition_ix);

                let example = example.clone();
                let worktree_path = worktree_path.clone();
                let app_state = app_state.clone();

                cx.spawn(async move |cx| {
                    let name = example.name.clone();
                    run_evaluate_one(
                        example,
                        repetition_ix,
                        worktree_path,
                        args.prompt_format,
                        args.use_expected_context,
                        args.cache,
                        app_state,
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
    let mut successful = Vec::new();
    let mut failed_count = 0;
    for result in all_results.iter().flatten() {
        match result {
            Ok(eval_result) => successful.push(eval_result),
            Err((err, name, repetition_ix)) => {
                failed_count += 1;
                eprintln!(
                    "{name}{}: {err}",
                    repetition_ix
                        .map(|ix| format!(" [{ix:03}]"))
                        .unwrap_or_default()
                );
            }
        }
    }

    let aggregated_result = EvaluationResult {
        context: Scores::aggregate(successful.iter().map(|r| &r.context)),
        edit_prediction: Scores::aggregate(successful.iter().map(|r| &r.edit_prediction)),
    };

    if example_len > 1 {
        println!("\n{}", "-".repeat(80));
        println!("\n## TOTAL SCORES");
        println!("{}", aggregated_result.to_markdown());
        println!("\n{}", "-".repeat(80));
        println!("{} failed examples", failed_count);
    }

    print_run_data_dir();
}

pub async fn run_evaluate_one(
    example: NamedExample,
    repetition_ix: Option<u16>,
    worktree_path: PathBuf,
    prompt_format: PromptFormat,
    use_expected_context: bool,
    cache_mode: CacheMode,
    app_state: Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<EvaluationResult> {
    let predictions = zeta2_predict(
        example.clone(),
        &worktree_path,
        repetition_ix,
        prompt_format,
        use_expected_context,
        cache_mode,
        &app_state,
        cx,
    )
    .await?;

    let evaluation_result = evaluate(&example.example, &predictions);

    println!(
        "## Expected edit prediction:\n\n```diff\n{}\n```\n",
        compare_diffs(&example.example.expected_patch, &predictions.diff)
    );
    println!(
        "## Actual edit prediction:\n\n```diff\n{}\n```\n",
        compare_diffs(&predictions.diff, &example.example.expected_patch)
    );

    println!("{}", evaluation_result.to_markdown());

    anyhow::Ok(evaluation_result)
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

impl EvaluationResult {
    pub fn to_markdown(&self) -> String {
        format!(
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
