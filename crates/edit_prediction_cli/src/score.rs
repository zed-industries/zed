use crate::{
    PredictArgs,
    example::{Example, ExampleScore},
    headless::EpAppState,
    metrics::{self, ClassificationMetrics},
    predict::run_prediction,
    progress::{Progress, Step},
};
use edit_prediction::udiff::DiffLine;
use gpui::AsyncApp;
use std::sync::Arc;

pub async fn run_scoring(
    example: &mut Example,
    args: &PredictArgs,
    app_state: Arc<EpAppState>,
    cx: AsyncApp,
) -> anyhow::Result<()> {
    run_prediction(
        example,
        Some(args.provider),
        args.repetitions,
        app_state,
        cx,
    )
    .await?;

    let _progress = Progress::global().start(Step::Score, &example.spec.name);

    let expected_patch = parse_patch(&example.spec.expected_patch);

    let mut scores = vec![];

    for pred in &example.predictions {
        let actual_patch = parse_patch(&pred.actual_patch);
        let line_match = metrics::line_match_score(&expected_patch, &actual_patch);
        let delta_chr_f = metrics::delta_chr_f(&expected_patch, &actual_patch) as f32;

        scores.push(ExampleScore {
            delta_chr_f,
            line_match,
        });
    }

    example.score = scores;
    Ok(())
}

fn parse_patch(patch: &str) -> Vec<DiffLine<'_>> {
    patch.lines().map(DiffLine::parse).collect()
}

pub fn print_report(examples: &[Example]) {
    eprintln!(
        "──────────────────────────────────────────────────────────────────────────────────────"
    );
    eprintln!(
        "{:<30} {:>4} {:>4} {:>4} {:>10} {:>8} {:>8} {:>10}",
        "Example name", "TP", "FP", "FN", "Precision", "Recall", "F1", "DeltaChrF"
    );
    eprintln!(
        "──────────────────────────────────────────────────────────────────────────────────────"
    );

    let mut all_line_match_scores = Vec::new();
    let mut all_delta_chr_f_scores = Vec::new();

    for example in examples {
        for score in example.score.iter() {
            let line_match = &score.line_match;

            eprintln!(
                "{:<30} {:>4} {:>4} {:>4} {:>9.2}% {:>7.2}% {:>7.2}% {:>9.2}",
                truncate_name(&example.spec.name, 30),
                line_match.true_positives,
                line_match.false_positives,
                line_match.false_negatives,
                line_match.precision() * 100.0,
                line_match.recall() * 100.0,
                line_match.f1_score() * 100.0,
                score.delta_chr_f
            );

            all_line_match_scores.push(line_match.clone());
            all_delta_chr_f_scores.push(score.delta_chr_f);
        }
    }

    eprintln!(
        "──────────────────────────────────────────────────────────────────────────────────────"
    );

    if !all_line_match_scores.is_empty() {
        let total_line_match = ClassificationMetrics::aggregate(all_line_match_scores.iter());
        let avg_delta_chr_f: f32 =
            all_delta_chr_f_scores.iter().sum::<f32>() / all_delta_chr_f_scores.len() as f32;

        eprintln!(
            "{:<30} {:>4} {:>4} {:>4} {:>9.2}% {:>7.2}% {:>7.2}% {:>9.2}",
            "TOTAL",
            total_line_match.true_positives,
            total_line_match.false_positives,
            total_line_match.false_negatives,
            total_line_match.precision() * 100.0,
            total_line_match.recall() * 100.0,
            total_line_match.f1_score() * 100.0,
            avg_delta_chr_f
        );
        eprintln!(
            "──────────────────────────────────────────────────────────────────────────────────────"
        );
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
