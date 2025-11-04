use std::io::IsTerminal;

use cloud_llm_client::udiff::DiffLine;
use collections::HashSet;

use crate::{PredictionDetails, example::Example};

#[derive(Debug, Default)]
pub struct EvaluationResult {
    pub context: Scores,
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
Precision       : {:.4}
Recall          : {:.4}
F1 Score        : {:.4}
True Positives  : {}
False Positives : {}
False Negatives : {}",
            self.precision,
            self.recall,
            self.f1_score,
            self.true_positives,
            self.false_positives,
            self.false_negatives
        )
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

    result
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
