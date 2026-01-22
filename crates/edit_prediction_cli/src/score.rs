use crate::{
    PredictArgs,
    example::{Example, ExampleScore},
    headless::EpAppState,
    metrics,
    parse_output::parse_prediction_output,
    predict::run_prediction,
    progress::{ExampleProgress, Step},
};
use anyhow::Context as _;
use edit_prediction::udiff::apply_diff_to_string;
use gpui::AsyncApp;
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
    let expected_texts: Vec<String> = example
        .spec
        .expected_patches
        .iter()
        .map(|patch| {
            apply_diff_to_string(patch, original_text)
                .with_context(|| format!("Expected patch did not apply for {}", example.spec.name))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let zero_scores = ExampleScore {
        delta_chr_f: 0.0,
        braces_disbalance: 0,
        exact_lines_tp: 0,
        exact_lines_fp: 0,
        exact_lines_fn: 0,
    };

    progress.set_substatus("computing metrics");
    let mut scores = vec![];
    for prediction in &example.predictions {
        let actual_patch = prediction.actual_patch.clone().or_else(|| {
            parse_prediction_output(example, &prediction.actual_output, prediction.provider).ok()
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
        let best_delta_chr_f = expected_texts
            .iter()
            .map(|expected| metrics::delta_chr_f(original_text, expected, &actual_text) as f32)
            .fold(0.0, f32::max);

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
        let best_exact_lines = example
            .spec
            .expected_patches
            .iter()
            .map(|expected_patch| metrics::exact_lines_match(expected_patch, &actual_patch))
            .max_by_key(|m| m.true_positives)
            .unwrap_or_default();

        scores.push(ExampleScore {
            delta_chr_f: best_delta_chr_f,
            braces_disbalance,
            exact_lines_tp: best_exact_lines.true_positives,
            exact_lines_fp: best_exact_lines.false_positives,
            exact_lines_fn: best_exact_lines.false_negatives,
        });
    }

    example.score = scores;
    Ok(())
}

pub fn print_report(examples: &[Example]) {
    use crate::metrics::ClassificationMetrics;

    const LINE_WIDTH: usize = 100;
    let separator = "─".repeat(LINE_WIDTH);

    eprintln!("{}", separator);
    eprintln!(
        "{:<40} {:>8} {:>5} {:>4} {:>4} {:>4} {:>7} {:>7} {:>7}",
        "Example", "DeltaChrF", "Brace", "TP", "FP", "FN", "Prec", "Rec", "F1"
    );
    eprintln!("{}", separator);

    let mut all_delta_chr_f_scores = Vec::new();
    let mut braces_disbalance_sum: usize = 0;
    let mut total_exact_lines = ClassificationMetrics::default();
    let mut total_scores: usize = 0;

    for example in examples {
        for score in example.score.iter() {
            let exact_lines = ClassificationMetrics {
                true_positives: score.exact_lines_tp,
                false_positives: score.exact_lines_fp,
                false_negatives: score.exact_lines_fn,
            };

            eprintln!(
                "{:<40} {:>8.2} {:>5} {:>4} {:>4} {:>4} {:>6.1}% {:>6.1}% {:>6.1}%",
                truncate_name(&example.spec.name, 40),
                score.delta_chr_f,
                score.braces_disbalance,
                score.exact_lines_tp,
                score.exact_lines_fp,
                score.exact_lines_fn,
                exact_lines.precision() * 100.0,
                exact_lines.recall() * 100.0,
                exact_lines.f1() * 100.0
            );

            all_delta_chr_f_scores.push(score.delta_chr_f);
            total_scores += 1;
            braces_disbalance_sum += score.braces_disbalance;
            total_exact_lines.true_positives += score.exact_lines_tp;
            total_exact_lines.false_positives += score.exact_lines_fp;
            total_exact_lines.false_negatives += score.exact_lines_fn;
        }
    }

    eprintln!("{}", separator);

    if !all_delta_chr_f_scores.is_empty() {
        let avg_delta_chr_f: f32 =
            all_delta_chr_f_scores.iter().sum::<f32>() / all_delta_chr_f_scores.len() as f32;
        let braces_disbalance_avg: f32 = braces_disbalance_sum as f32 / total_scores as f32;

        eprintln!(
            "{:<40} {:>8.2} {:>5.1} {:>4} {:>4} {:>4} {:>6.1}% {:>6.1}% {:>6.1}%",
            "TOTAL / AVERAGE",
            avg_delta_chr_f,
            braces_disbalance_avg,
            total_exact_lines.true_positives,
            total_exact_lines.false_positives,
            total_exact_lines.false_negatives,
            total_exact_lines.precision() * 100.0,
            total_exact_lines.recall() * 100.0,
            total_exact_lines.f1() * 100.0
        );
        eprintln!("{}", separator);
    }

    eprintln!("\n");
}

fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[..max_len - 3])
    }
}
