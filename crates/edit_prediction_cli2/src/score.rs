use edit_prediction::udiff::DiffLine;

use crate::{
    ScoreArgs,
    example::{Example, ExampleScore},
    metrics,
    predict::run_prediction,
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
