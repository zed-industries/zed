use std::{
    fs,
    io::IsTerminal,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use clap::Args;
use cloud_llm_client::udiff::DiffLine;
use collections::HashSet;
use gpui::AsyncApp;

use crate::{
    example::{Example, Excerpt, NamedExample},
    headless::ZetaCliAppState,
    predict::{PredictionDetails, zeta2_predict},
};

#[derive(Debug, Args)]
pub struct EvaluateArguments {
    example_paths: Vec<PathBuf>,
    #[clap(long)]
    re_run: bool,
}

pub async fn run_evaluate(
    args: EvaluateArguments,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) {
    let example_len = args.example_paths.len();
    let all_tasks = args.example_paths.into_iter().map(|path| {
        let app_state = app_state.clone();
        cx.spawn(async move |cx| run_evaluate_one(&path, args.re_run, app_state.clone(), cx).await)
    });
    let all_results = futures::future::try_join_all(all_tasks).await.unwrap();

    let aggregated_result = EvaluationResult {
        context: Scores::aggregate(all_results.iter().map(|r| &r.context)),
        edit_prediction: Scores::aggregate(all_results.iter().map(|r| &r.edit_prediction)),
        edit_sites_coverage: all_results
            .iter()
            .map(|r| r.edit_sites_coverage)
            .sum::<f64>()
            / all_results.len() as f64,
    };

    if example_len > 1 {
        println!("\n{}", "-".repeat(80));
        println!("# TOTAL SCORES:");
        println!("{}", aggregated_result.to_markdown());
    }
}

pub async fn run_evaluate_one(
    example_path: &Path,
    re_run: bool,
    app_state: Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<EvaluationResult> {
    let cache_dir = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default())
        .join("../../target/zeta-prediction-cache");
    let example = NamedExample::load(&example_path).unwrap();
    let example_cache_path = cache_dir.join(&example_path.file_name().unwrap());

    let predictions = if !re_run && example_cache_path.exists() {
        let file_contents = fs::read_to_string(&example_cache_path)?;
        let as_json = serde_json::from_str::<PredictionDetails>(&file_contents)?;
        log::debug!(
            "Loaded predictions from cache: {}",
            example_cache_path.display()
        );
        as_json
    } else {
        zeta2_predict(example.clone(), &app_state, cx)
            .await
            .unwrap()
    };

    if !example_cache_path.exists() {
        fs::create_dir_all(&cache_dir).unwrap();
        fs::write(
            example_cache_path,
            serde_json::to_string(&predictions).unwrap(),
        )
        .unwrap();
    }

    let evaluation_result = evaluate(&example.example, &predictions);

    println!("# {}\n", example.name);
    println!(
        "## Expected Context: \n\n```\n{}\n```\n\n",
        compare_context(&example.example, &predictions)
    );
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
    pub context: Scores,

    /// Ratio of edited lines that we expect to edit (as indicated in the
    /// expected patch) AND were included into the context
    /// num_correctly_retrieved_lines / num_expected_lines
    pub edit_sites_coverage: f64,

    pub edit_prediction: Scores,
}

#[derive(Default, Debug)]
pub struct Scores {
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub true_positives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
}

impl Scores {
    pub fn to_markdown(&self) -> String {
        format!(
            "
Precision          : {:.4}
Recall             : {:.4}
F1 Score           : {:.4}
True Positives     : {}
False Positives    : {}
False Negatives    : {}",
            self.precision,
            self.recall,
            self.f1_score,
            self.true_positives,
            self.false_positives,
            self.false_negatives
        )
    }
}

impl Scores {
    pub fn aggregate<'a>(scores: impl Iterator<Item = &'a Scores>) -> Scores {
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for score in scores {
            true_positives += score.true_positives;
            false_positives += score.false_positives;
            false_negatives += score.false_negatives;
        }

        let precision = true_positives as f64 / (true_positives + false_positives) as f64;
        let recall = true_positives as f64 / (true_positives + false_negatives) as f64;
        let mut f1_score = 2.0 * precision * recall / (precision + recall);
        if f1_score.is_nan() {
            f1_score = 0.0;
        }

        Scores {
            precision,
            recall,
            f1_score,
            true_positives,
            false_positives,
            false_negatives,
        }
    }
}

#[derive(Debug, Clone)]
struct EditSitesScores {
    num_edit_sites: u32,
    num_correctly_retrieved: u32,
}

impl EvaluationResult {
    pub fn to_markdown(&self) -> String {
        format!(
            r#"
### Context Scores
{}
Edit sites coverage: {}

### Edit Prediction Scores
{}
"#,
            self.context.to_markdown(),
            self.edit_sites_coverage,
            self.edit_prediction.to_markdown()
        )
    }
}

pub fn evaluate(example: &Example, preds: &PredictionDetails) -> EvaluationResult {
    let mut result = EvaluationResult::default();

    let expected_context_lines = example
        .expected_excerpts
        .iter()
        .flat_map(|excerpt| {
            excerpt
                .text
                .lines()
                .map(|line| format!("{}: {line}", excerpt.path.display()))
        })
        .collect();
    let actual_context_lines = preds
        .excerpts
        .iter()
        .flat_map(|excerpt| {
            excerpt
                .text
                .lines()
                .map(|line| format!("{}: {line}", excerpt.path.display()))
        })
        .collect();

    result.context = precision_recall(&expected_context_lines, &actual_context_lines);

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

    result.edit_prediction = precision_recall(&expected_patch_lines, &actual_patch_lines);

    result.edit_sites_coverage =
        calculate_edit_sites_coverage(&example.expected_patch, &preds.excerpts);

    result
}

/// Compute the ratio of lines that we expect to edit (are in the expected patch) that
/// were included in the retrieved context
/// `num_correctly_retrieved_lines / num_edited_lines_in_expected_patch`
///
/// In order to make an edit in some line, the model has to have an access to this line.
/// If we don't include the line in the retrieved context, there's no chance to make an edit.
///
/// This metric reflects that, where 1.0 -- we retrieved all lines to be
/// edited, and 0.0 -- we retrieved none of them.
///
/// Example:
fn calculate_edit_sites_coverage(patch: &str, excerpts: &[Excerpt]) -> EditSitesScores {
    // todo:
    let expected_patch_lines = patch
        .lines()
        .map(DiffLine::parse)
        .filter_map(|line| match line {
            DiffLine::Deletion(text) => Some(text.trim().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();

    let correct_cases = expected_patch_lines
        .iter()
        .filter(|line| {
            excerpts.iter().any(|excerpt| {
                excerpt
                    .text
                    .lines()
                    .any(|excerpt_line| excerpt_line == *line)
            })
        })
        .count();
    let total_cases = expected_patch_lines.len();

    if total_cases == 0 {
        0.0
    } else {
        correct_cases as f64 / total_cases as f64
    }
}

fn precision_recall(expected: &HashSet<String>, actual: &HashSet<String>) -> Scores {
    let true_positives = expected.intersection(actual).count();
    let false_positives = actual.difference(expected).count();
    let false_negatives = expected.difference(actual).count();

    let precision = if true_positives + false_positives == 0 {
        0.0
    } else {
        true_positives as f64 / (true_positives + false_positives) as f64
    };
    let recall = if true_positives + false_negatives == 0 {
        0.0
    } else {
        true_positives as f64 / (true_positives + false_negatives) as f64
    };
    let f1_score = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };

    Scores {
        precision,
        recall,
        f1_score,
        true_positives,
        false_positives,
        false_negatives,
    }
}

/// Compare actual and expected context.
///
/// Return expected context annotated with these markers:
///
/// `✓ context line`  -- line was correctly predicted
/// `✗ context line`  -- line is missing from predictions
pub fn compare_context(example: &Example, preds: &PredictionDetails) -> String {
    let use_color = std::io::stdout().is_terminal();
    let green = if use_color { "\x1b[32m" } else { "" };
    let red = if use_color { "\x1b[31m" } else { "" };
    let reset = if use_color { "\x1b[0m" } else { "" };
    let expected: Vec<_> = example
        .expected_excerpts
        .iter()
        .flat_map(|excerpt| {
            excerpt
                .text
                .lines()
                .map(|line| (excerpt.path.clone(), line))
        })
        .collect();
    let actual: HashSet<_> = preds
        .excerpts
        .iter()
        .flat_map(|excerpt| {
            excerpt
                .text
                .lines()
                .map(|line| (excerpt.path.clone(), line))
        })
        .collect();

    let annotated = expected
        .iter()
        .map(|(path, line)| {
            if actual.contains(&(path.to_path_buf(), line)) {
                format!("{green}✓ {line}{reset}")
            } else {
                format!("{red}✗ {line}{reset}")
            }
        })
        .collect::<Vec<String>>();

    annotated.join("\n")
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

#[cfg(test)]
mod tests {
    use super::calculate_edit_sites_coverage;
    use crate::example::Excerpt;

    #[test]
    fn test_evaluate_expected_edit_places() {
        let patch = indoc::indoc! {"
            --- a/test.txt
            +++ b/test.txt
            @@ -1,4 +1,4 @@
             apple
            -banana
            +BANANA
             cherry
            -date
            +DATE
            "};

        let one_correct_excerpt = vec![Excerpt {
            path: "test.txt".into(),
            text: "apple\nbanana\n".to_string(),
        }];

        assert_eq!(
            calculate_edit_sites_coverage(&patch, &one_correct_excerpt),
            0.5,
        );

        let both_correct_excerpts = vec![
            Excerpt {
                path: "test.txt".into(),
                text: "apple\nbanana\n".to_string(),
            },
            Excerpt {
                path: "test.txt".into(),
                text: "cherry\ndate\n".to_string(),
            },
        ];

        assert_eq!(
            calculate_edit_sites_coverage(&patch, &both_correct_excerpts),
            1.0,
        );

        let incorrect_excerpts = vec![Excerpt {
            path: "test.txt".into(),
            text: "apple\n".into(),
        }];
        assert_eq!(
            calculate_edit_sites_coverage(&patch, &incorrect_excerpts),
            0.0,
        );
    }
}
