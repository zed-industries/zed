use crate::{
    PredictArgs,
    example::{Example, ExampleScore},
    headless::EpAppState,
    metrics,
    predict::run_prediction,
    progress::{Progress, Step},
};
use anyhow::Context as _;
use edit_prediction::udiff::apply_diff_to_string;
use gpui::AsyncApp;
use std::sync::Arc;

pub async fn run_scoring(
    example: &mut Example,
    args: &PredictArgs,
    app_state: Arc<EpAppState>,
    cx: AsyncApp,
) -> anyhow::Result<()> {
    run_prediction(example, args, app_state, cx).await?;

    let progress = Progress::global().start(Step::Score, &example.spec.name);

    progress.set_substatus("applying patches");
    let original_text = &example.prompt_inputs.as_ref().unwrap().content;
    let expected_texts: Vec<String> = example
        .spec
        .expected_patches
        .iter()
        .map(|patch| {
            apply_diff_to_string(patch, original_text)
                .with_context(|| format!("Expected patch did not apply for {}", example.spec.name))
        })
        .collect::<Result<Vec<_>, _>>()?;

    progress.set_substatus("computing metrics");
    let mut scores = vec![];
    for prediction in &example.predictions {
        let actual_text = match apply_diff_to_string(&prediction.actual_patch, original_text) {
            Ok(text) => text,
            Err(_) => {
                scores.push(ExampleScore { delta_chr_f: 0.0 });
                continue;
            }
        };
        let best_delta_chr_f = expected_texts
            .iter()
            .map(|expected| metrics::delta_chr_f(original_text, expected, &actual_text) as f32)
            .fold(0.0, f32::max);
        scores.push(ExampleScore {
            delta_chr_f: best_delta_chr_f,
        });
    }

    example.score = scores;
    Ok(())
}

pub fn print_report(examples: &[Example]) {
    eprintln!(
        "──────────────────────────────────────────────────────────────────────────────────────"
    );
    eprintln!("{:<50} {:>10}", "Example name", "DeltaChrF");
    eprintln!(
        "──────────────────────────────────────────────────────────────────────────────────────"
    );

    let mut all_delta_chr_f_scores = Vec::new();

    for example in examples {
        for score in example.score.iter() {
            eprintln!(
                "{:<50} {:>9.2}",
                truncate_name(&example.spec.name, 50),
                score.delta_chr_f
            );

            all_delta_chr_f_scores.push(score.delta_chr_f);
        }
    }

    eprintln!(
        "──────────────────────────────────────────────────────────────────────────────────────"
    );

    if !all_delta_chr_f_scores.is_empty() {
        let avg_delta_chr_f: f32 =
            all_delta_chr_f_scores.iter().sum::<f32>() / all_delta_chr_f_scores.len() as f32;

        eprintln!("{:<50} {:>9.2}", "AVERAGE", avg_delta_chr_f);
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
