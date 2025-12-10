use edit_prediction::udiff::DiffLine;

use crate::{
    ScoreArgs,
    example::{Example, ExampleScore},
    metrics::{self, ClassificationMetrics},
};

pub async fn run_scoring(example: &mut Example, _score_args: &ScoreArgs) {
    let expected_patch = parse_patch(&example.expected_patch);

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
}

fn parse_patch(patch: &str) -> Vec<DiffLine<'_>> {
    patch.lines().map(DiffLine::parse).collect()
}

pub fn print_report(examples: &[Example]) {
    println!("\n");
    println!(
        "────────────────────────────────────────────────────────────────────────────────────────────────────────"
    );
    println!(
        "{:<30} {:<18} {:>4} {:>4} {:>4} {:>10} {:>8} {:>8} {:>10}",
        "Example name", "Provider", "TP", "FP", "FN", "Precision", "Recall", "F1", "DeltaChrF"
    );
    println!(
        "────────────────────────────────────────────────────────────────────────────────────────────────────────"
    );

    let mut all_line_match_scores = Vec::new();
    let mut all_delta_chr_f_scores = Vec::new();

    for example in examples {
        for (idx, score) in example.score.iter().enumerate() {
            let provider_name = if let Some(pred) = example.predictions.get(idx) {
                format!("{:?}", pred.provider)
            } else {
                "Unknown".to_string()
            };

            let line_match = &score.line_match;

            println!(
                "{:<30} {:<18} {:>4} {:>4} {:>4} {:>9.2}% {:>7.2}% {:>7.2}% {:>9.2}",
                truncate_name(&example.name, 30),
                truncate_name(&provider_name, 18),
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

    println!(
        "────────────────────────────────────────────────────────────────────────────────────────────────────────"
    );

    if !all_line_match_scores.is_empty() {
        let total_line_match = ClassificationMetrics::aggregate(all_line_match_scores.iter());
        let avg_delta_chr_f: f32 =
            all_delta_chr_f_scores.iter().sum::<f32>() / all_delta_chr_f_scores.len() as f32;

        println!(
            "{:<30} {:<18} {:>4} {:>4} {:>4} {:>9.2}% {:>7.2}% {:>7.2}% {:>9.2}",
            "TOTAL",
            "",
            total_line_match.true_positives,
            total_line_match.false_positives,
            total_line_match.false_negatives,
            total_line_match.precision() * 100.0,
            total_line_match.recall() * 100.0,
            total_line_match.f1_score() * 100.0,
            avg_delta_chr_f
        );
        println!(
            "────────────────────────────────────────────────────────────────────────────────────────────────────────"
        );
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
