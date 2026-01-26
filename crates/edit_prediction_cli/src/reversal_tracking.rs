use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result};
use edit_prediction::udiff::apply_diff_to_string;
use language::text_diff;

use crate::example::{Example, ExamplePrediction, ExamplePromptInputs};

/// Reverse a unified diff by swapping + and - lines.
/// This transforms a diff that goes A→B into one that goes B→A.
pub fn reverse_diff(diff: &str) -> String {
    diff.lines()
        .map(|line| {
            if line.starts_with("--- ") {
                line.replacen("--- ", "+++ ", 1)
            } else if line.starts_with("+++ ") {
                line.replacen("+++ ", "--- ", 1)
            } else if line.starts_with('+') && !line.starts_with("+++") {
                format!("-{}", &line[1..])
            } else if line.starts_with('-') && !line.starts_with("---") {
                format!("+{}", &line[1..])
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Represents a single granular edit operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GranularEdit {
    pub range: Range<usize>,
    pub old_text: String,
    pub new_text: String,
}

/// Compute granular word-level edits between two strings.
pub fn compute_granular_edits(old_text: &str, new_text: &str) -> Vec<GranularEdit> {
    text_diff(old_text, new_text)
        .into_iter()
        .map(|(range, new_text)| GranularEdit {
            old_text: old_text[range.clone()].to_string(),
            range,
            new_text: new_text.to_string(),
        })
        .collect()
}

/// Tracks byte ranges in the "current" content that were added by history edits.
/// These ranges represent text that exists in current_content but didn't exist in original_content.
#[derive(Debug, Clone)]
pub struct HistoryAdditionRange {
    pub range_in_current: Range<usize>,
    pub added_text: String,
}

/// Tracks byte ranges in the "original" content that were deleted by history edits.
/// These ranges represent text that existed in original_content but doesn't exist in current_content.
#[derive(Debug, Clone)]
pub struct HistoryDeletionRange {
    pub range_in_original: Range<usize>,
    pub deleted_text: String,
}

/// Compute ranges of text that history added (now present in current_content).
/// Takes the granular edits from original→current and computes where added text ends up.
pub fn compute_history_addition_ranges(
    history_edits: &[GranularEdit],
) -> Vec<HistoryAdditionRange> {
    let mut result = Vec::new();
    let mut offset_delta: isize = 0;

    for edit in history_edits {
        if !edit.new_text.is_empty() {
            let new_start = (edit.range.start as isize + offset_delta) as usize;
            let new_end = new_start + edit.new_text.len();
            result.push(HistoryAdditionRange {
                range_in_current: new_start..new_end,
                added_text: edit.new_text.clone(),
            });
        }

        offset_delta += edit.new_text.len() as isize - edit.old_text.len() as isize;
    }

    result
}

/// Compute ranges of text that history deleted (was present in original_content).
pub fn compute_history_deletion_ranges(
    history_edits: &[GranularEdit],
) -> Vec<HistoryDeletionRange> {
    history_edits
        .iter()
        .filter(|edit| !edit.old_text.is_empty())
        .map(|edit| HistoryDeletionRange {
            range_in_original: edit.range.clone(),
            deleted_text: edit.old_text.clone(),
        })
        .collect()
}

/// Measures overlap between a prediction and user edit history.
///
/// This struct tracks how much of a prediction's changes undo recent user edits:
/// - `chars_reversing_user_edits`: Characters in the prediction that reverse user actions
///   (either deleting text the user added, or re-adding text the user deleted)
/// - `total_chars_in_prediction`: Total characters changed by the prediction (added + deleted)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReversalOverlap {
    pub chars_reversing_user_edits: usize,
    pub total_chars_in_prediction: usize,
}

impl ReversalOverlap {
    pub fn ratio(&self) -> f32 {
        if self.total_chars_in_prediction == 0 {
            0.0
        } else {
            self.chars_reversing_user_edits as f32 / self.total_chars_in_prediction as f32
        }
    }
}

/// Compute how much of a prediction reverses recent user edits.
///
/// Takes three content snapshots:
/// - `original_content`: The content before the user made their edits
/// - `current_content`: The content after user edits (before prediction)
/// - `predicted_content`: The content the model is predicting
///
/// Returns a `ReversalOverlap` measuring how much of the prediction undoes user work.
pub fn compute_reversal_overlap(
    original_content: &str,
    current_content: &str,
    predicted_content: &str,
) -> ReversalOverlap {
    let history_edits = compute_granular_edits(original_content, current_content);
    let prediction_edits = compute_granular_edits(current_content, predicted_content);

    let history_addition_ranges = compute_history_addition_ranges(&history_edits);
    let history_deletion_ranges = compute_history_deletion_ranges(&history_edits);

    let reversed_additions =
        compute_reversed_additions(&history_addition_ranges, &prediction_edits, current_content);
    let restored_deletions =
        compute_restored_deletions(&history_deletion_ranges, &prediction_edits);

    let prediction_added_chars: usize = prediction_edits.iter().map(|e| e.new_text.len()).sum();
    let prediction_deleted_chars: usize = prediction_edits.iter().map(|e| e.old_text.len()).sum();

    ReversalOverlap {
        chars_reversing_user_edits: reversed_additions + restored_deletions,
        total_chars_in_prediction: prediction_added_chars + prediction_deleted_chars,
    }
}

/// Metrics for how much of a prediction reverses edit history.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ReversalMetrics {
    pub history_added_chars: usize,
    pub history_deleted_chars: usize,
    pub prediction_added_chars: usize,
    pub prediction_deleted_chars: usize,
    /// Characters deleted by prediction that were added by history (undoing additions)
    pub reversed_addition_chars: usize,
    /// Characters added by prediction that match text deleted by history (restoring deletions)
    pub restored_deletion_chars: usize,
    pub reversal_ratio: f64,
}

/// Compute how many characters the prediction deletes that were added by history.
/// This measures "undoing additions" - the prediction removing text that history added.
pub fn compute_reversed_additions(
    history_addition_ranges: &[HistoryAdditionRange],
    prediction_edits: &[GranularEdit],
    _current_content: &str,
) -> usize {
    let mut reversed_chars = 0;

    for pred_edit in prediction_edits {
        for history_addition in history_addition_ranges {
            let overlap_start = pred_edit
                .range
                .start
                .max(history_addition.range_in_current.start);
            let overlap_end = pred_edit
                .range
                .end
                .min(history_addition.range_in_current.end);

            if overlap_start < overlap_end {
                reversed_chars += overlap_end - overlap_start;
            }
        }
    }

    reversed_chars
}

/// Compute how many characters the prediction adds that were deleted by history.
/// This measures "restoring deletions" - the prediction adding back text that history removed.
/// Uses character-level matching since positions don't correspond directly.
pub fn compute_restored_deletions(
    history_deletion_ranges: &[HistoryDeletionRange],
    prediction_edits: &[GranularEdit],
) -> usize {
    let history_deleted_text: String = history_deletion_ranges
        .iter()
        .map(|r| r.deleted_text.as_str())
        .collect();

    let prediction_added_text: String = prediction_edits
        .iter()
        .map(|e| e.new_text.as_str())
        .collect();

    compute_char_overlap(&history_deleted_text, &prediction_added_text)
}

/// Compute character-level overlap between two strings (bag of characters).
fn compute_char_overlap(a: &str, b: &str) -> usize {
    use collections::HashMap;

    let mut a_chars: HashMap<char, usize> = HashMap::default();
    for c in a.chars() {
        *a_chars.entry(c).or_default() += 1;
    }

    let mut overlap = 0;
    for c in b.chars() {
        if let Some(count) = a_chars.get_mut(&c) {
            if *count > 0 {
                *count -= 1;
                overlap += 1;
            }
        }
    }
    overlap
}

/// Filter edit history events to only include changes to a specific file path.
pub fn filter_edit_history_by_path<'a>(
    edit_history: &'a [Arc<zeta_prompt::Event>],
    cursor_path: &std::path::Path,
) -> Vec<&'a zeta_prompt::Event> {
    edit_history
        .iter()
        .filter(|event| match event.as_ref() {
            zeta_prompt::Event::BufferChange { path, .. } => {
                let event_path = path.as_ref();
                event_path == cursor_path || event_path.ends_with(cursor_path)
            }
        })
        .map(|arc| arc.as_ref())
        .collect()
}

/// Extract the diff string from a BufferChange event.
pub fn extract_diff_from_event(event: &zeta_prompt::Event) -> &str {
    match event {
        zeta_prompt::Event::BufferChange { diff, .. } => diff.as_str(),
    }
}

/// Reconstruct original content by applying reversed edit history diffs.
fn reconstruct_original_content(
    current_content: &str,
    edit_history: &[Arc<zeta_prompt::Event>],
    cursor_path: &Path,
) -> Result<String> {
    let relevant_events = filter_edit_history_by_path(edit_history, cursor_path);

    let mut content = current_content.to_string();
    for event in relevant_events.into_iter().rev() {
        let diff = extract_diff_from_event(event);
        if diff.is_empty() {
            continue;
        }
        let reversed = reverse_diff(diff);
        let with_headers = format!("--- a/file\n+++ b/file\n{}", reversed);
        content = apply_diff_to_string(&with_headers, &content)
            .with_context(|| format!("Failed to apply reversed diff: {}", diff))?;
    }
    Ok(content)
}

/// Compute reversal overlap for an example's prediction.
fn compute_example_reversal(
    prompt_inputs: &ExamplePromptInputs,
    prediction: &ExamplePrediction,
    cursor_path: &Path,
) -> Result<Option<ReversalOverlap>> {
    let current_content = &prompt_inputs.content;

    let actual_patch = match &prediction.actual_patch {
        Some(patch) if !patch.is_empty() => patch,
        _ => anyhow::bail!("No Actual patch. Run parse-output"),
    };

    let predicted_content = apply_diff_to_string(actual_patch, current_content)
        .with_context(|| "Failed to apply prediction patch")?;

    let original_content =
        reconstruct_original_content(current_content, &prompt_inputs.edit_history, cursor_path)?;

    Ok(Some(compute_reversal_overlap(
        &original_content,
        current_content,
        &predicted_content,
    )))
}

/// Filter JSONL examples by reversal ratio threshold, or compute statistics.
///
/// When `stats` is false: reads examples from input, computes reversal ratio for each prediction,
/// and writes examples where any prediction has reversal ratio >= threshold.
///
/// When `stats` is true: computes and prints statistics about reversal ratios in the input.
pub fn run_filter_reversals(
    threshold: f32,
    stats: bool,
    inputs: &[PathBuf],
    output: Option<&PathBuf>,
) -> Result<()> {
    let input_path: Option<&Path> = match inputs.first().map(|p| p.as_path()) {
        Some(p) if p.as_os_str() == "-" => None,
        Some(p) => Some(p),
        None => None,
    };

    let reader: Box<dyn BufRead> = match input_path {
        Some(path) => {
            let file =
                File::open(path).with_context(|| format!("failed to open '{}'", path.display()))?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(std::io::stdin())),
    };

    let mut writer: Box<dyn Write> = if stats {
        Box::new(std::io::sink())
    } else {
        match output {
            Some(path) => {
                if let Some(parent) = path.parent() {
                    if !parent.as_os_str().is_empty() {
                        std::fs::create_dir_all(parent)?;
                    }
                }
                let file = File::create(path)
                    .with_context(|| format!("failed to create '{}'", path.display()))?;
                Box::new(BufWriter::new(file))
            }
            None => Box::new(BufWriter::new(std::io::stdout())),
        }
    };

    let mut total_count = 0usize;
    let mut kept_count = 0usize;
    let mut no_prompt_inputs = 0usize;
    let mut no_predictions = 0usize;
    let mut failed_reversal = 0usize;
    let mut all_ratios: Vec<f32> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.context("failed to read line")?;
        if line.trim().is_empty() {
            continue;
        }

        total_count += 1;

        let example: Example = match serde_json::from_str(&line) {
            Ok(e) => e,
            Err(_) => continue,
        };

        let prompt_inputs = match &example.prompt_inputs {
            Some(inputs) => inputs,
            None => {
                no_prompt_inputs += 1;
                continue;
            }
        };

        if example.predictions.is_empty() {
            no_predictions += 1;
            continue;
        }

        let cursor_path = example.spec.cursor_path.as_ref();
        let mut max_reversal_ratio = 0.0f32;

        for prediction in &example.predictions {
            match compute_example_reversal(prompt_inputs, prediction, cursor_path) {
                Ok(Some(overlap)) => {
                    let ratio = overlap.ratio();
                    if ratio > max_reversal_ratio {
                        max_reversal_ratio = ratio;
                    }
                }
                Ok(None) => {}
                Err(_) => {
                    failed_reversal += 1;
                }
            }
        }

        all_ratios.push(max_reversal_ratio);

        if max_reversal_ratio >= threshold {
            kept_count += 1;
            if !stats {
                writeln!(writer, "{}", line)?;
            }
        }
    }

    writer.flush()?;

    if stats {
        print_reversal_stats(
            &all_ratios,
            total_count,
            no_prompt_inputs,
            no_predictions,
            failed_reversal,
        );
    } else {
        eprintln!(
            "Filtered {} examples to {} with reversal ratio >= {:.0}%",
            total_count,
            kept_count,
            threshold * 100.0
        );
        eprintln!(
            "  Skipped: {} no prompt_inputs, {} no predictions, {} failed reversal computation",
            no_prompt_inputs, no_predictions, failed_reversal
        );
    }

    Ok(())
}

fn print_reversal_stats(
    ratios: &[f32],
    total_count: usize,
    no_prompt_inputs: usize,
    no_predictions: usize,
    failed_reversal: usize,
) {
    let valid_count = ratios.len();

    println!("Reversal Ratio Statistics");
    println!("==========================");
    println!("Total examples:      {}", total_count);
    println!("  With valid ratios: {}", valid_count);
    println!("  No prompt_inputs:  {}", no_prompt_inputs);
    println!("  No predictions:    {}", no_predictions);
    println!("  Failed to compute: {}", failed_reversal);
    println!();

    if ratios.is_empty() {
        println!("No valid ratios to analyze.");
        return;
    }

    let sum: f32 = ratios.iter().sum();
    let mean = sum / valid_count as f32;

    let mut sorted = ratios.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = if valid_count % 2 == 0 {
        (sorted[valid_count / 2 - 1] + sorted[valid_count / 2]) / 2.0
    } else {
        sorted[valid_count / 2]
    };

    let min = sorted.first().copied().unwrap_or(0.0);
    let max = sorted.last().copied().unwrap_or(0.0);
    let p25 = sorted[valid_count / 4];
    let p75 = sorted[valid_count * 3 / 4];
    let p90 = sorted[valid_count * 9 / 10];
    let p95 = sorted[valid_count * 95 / 100];

    println!("Mean:   {:.1}%", mean * 100.0);
    println!("Median: {:.1}%", median * 100.0);
    println!("Min:    {:.1}%", min * 100.0);
    println!("Max:    {:.1}%", max * 100.0);
    println!("P25:    {:.1}%", p25 * 100.0);
    println!("P75:    {:.1}%", p75 * 100.0);
    println!("P90:    {:.1}%", p90 * 100.0);
    println!("P95:    {:.1}%", p95 * 100.0);
    println!();

    println!("Distribution:");
    let buckets = [
        (0.0, 0.1, "0-10%"),
        (0.1, 0.2, "10-20%"),
        (0.2, 0.3, "20-30%"),
        (0.3, 0.4, "30-40%"),
        (0.4, 0.5, "40-50%"),
        (0.5, 0.6, "50-60%"),
        (0.6, 0.7, "60-70%"),
        (0.7, 0.8, "70-80%"),
        (0.8, 0.9, "80-90%"),
        (0.9, 1.01, "90-100%"),
    ];

    for (low, high, label) in buckets {
        let count = ratios.iter().filter(|&&r| r >= low && r < high).count();
        let pct = (count as f32 / valid_count as f32) * 100.0;
        let bar_len = (pct / 2.0) as usize;
        let bar: String = "█".repeat(bar_len);
        println!("  {:>7}: {:>5} ({:>5.1}%) {}", label, count, pct, bar);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edit_prediction::udiff::apply_diff_to_string;
    use indoc::indoc;

    struct ReversalTestCase {
        name: &'static str,
        original: &'static str,
        current: &'static str,
        predicted: &'static str,
        expected_reversal_chars: usize,
        expected_total_chars: usize,
        explanation: &'static str,
    }

    #[test]
    fn test_reversal_overlap_table() {
        let cases = [
            // === Complete reversals ===
            ReversalTestCase {
                name: "user_adds_line_prediction_removes_it",
                original: indoc! {"
                    a
                    b
                    c"},
                current: indoc! {"
                    a
                    new line
                    b
                    c"},
                predicted: indoc! {"
                    a
                    b
                    c"},
                expected_reversal_chars: 9,
                expected_total_chars: 9,
                explanation: "User added 'new line\n' (9 chars). Prediction deletes exactly those 9 chars.",
            },
            ReversalTestCase {
                name: "user_deletes_line_prediction_restores_it",
                original: indoc! {"
                    a
                    deleted
                    b"},
                current: indoc! {"
                    a
                    b"},
                predicted: indoc! {"
                    a
                    deleted
                    b"},
                expected_reversal_chars: 8,
                expected_total_chars: 8,
                explanation: "User deleted 'deleted\n' (8 chars). Prediction adds back exactly those 8 chars.",
            },
            // === Partial reversals ===
            ReversalTestCase {
                name: "user_adds_line_prediction_removes_partial",
                original: "fn main() {}",
                current: indoc! {"
                    fn main() {
                        let x = 42;
                    }"},
                predicted: indoc! {"
                    fn main() {
                    }"},
                expected_reversal_chars: 16,
                expected_total_chars: 16,
                explanation: "User added '\n    let x = 42;' (16 chars). Prediction removes all 16 added chars.",
            },
            ReversalTestCase {
                name: "user_deletes_function_prediction_restores_partial",
                original: indoc! {r#"
                    fn helper() {
                        println!("help");
                    }"#},
                current: "",
                predicted: indoc! {"
                    fn helper() {
                    }"},
                expected_reversal_chars: 15,
                expected_total_chars: 15,
                explanation: indoc! {"
                    User deleted entire function (38 chars). Prediction adds 'fn helper() {\n}'
                    (15 chars). Character overlap: f,n, ,h,e,l,p,e,r,(,),{,\n,} = 15 chars."},
            },
            // === Similar but different content ===
            ReversalTestCase {
                name: "user_deletes_hello_world_prediction_adds_hello_sailor",
                original: r#"println!("hello world");"#,
                current: "",
                predicted: r#"println!("hello sailor");"#,
                expected_reversal_chars: 22,
                expected_total_chars: 25,
                explanation: indoc! {r#"
                    User deleted 'println!("hello world");' (24 chars). Prediction adds
                    'println!("hello sailor");' (25 chars). Since current is empty, prediction
                    only adds (no deletes), so total = 25. Char overlap between deleted text
                    and added text: p,r,i,n,t,l,n,!,(,",h,e,l,l,o, ,o,l,r,",),; = 22."#},
            },
            ReversalTestCase {
                name: "user_deletes_foo_prediction_adds_bar",
                original: "foo",
                current: "",
                predicted: "bar",
                expected_reversal_chars: 0,
                expected_total_chars: 3,
                explanation: "User deleted 'foo'. Prediction adds 'bar'. No character overlap.",
            },
            // === No reversal cases ===
            ReversalTestCase {
                name: "independent_edits_different_locations",
                original: indoc! {"
                    line1
                    line2
                    line3"},
                current: indoc! {"
                    LINE1
                    line2
                    line3"},
                predicted: indoc! {"
                    LINE1
                    line2
                    LINE3"},
                expected_reversal_chars: 0,
                expected_total_chars: 10,
                explanation: indoc! {"
                    User changed line1→LINE1. Prediction changes line3→LINE3. No overlap
                    since they edit different regions. Total: delete 'line3' (5) + add 'LINE3' (5) = 10."},
            },
            ReversalTestCase {
                name: "no_history_edits",
                original: "same",
                current: "same",
                predicted: "different",
                expected_reversal_chars: 0,
                expected_total_chars: 13,
                explanation: indoc! {"
                    No user edits (original==current). Prediction changes 'same'→'different'.
                    Delete 4 + add 9 = 13 total chars. No reversal possible."},
            },
            // === Mixed operations ===
            ReversalTestCase {
                name: "user_adds_and_deletes_prediction_reverses_both",
                original: indoc! {"
                    keep
                    delete_me
                    keep2"},
                current: indoc! {"
                    keep
                    added
                    keep2"},
                predicted: indoc! {"
                    keep
                    delete_me
                    keep2"},
                expected_reversal_chars: 14,
                expected_total_chars: 14,
                explanation: indoc! {"
                    User changed 'delete_me' to 'added'. Prediction changes it back.
                    Prediction deletes 'added' (5 chars, all user-added = 5 reversal).
                    Prediction adds 'delete_me' (9 chars). Char overlap = 9.
                    Total reversal = 5 + 9 = 14. Total prediction = 5 deleted + 9 added = 14."},
            },
            ReversalTestCase {
                name: "user_modifies_word_prediction_modifies_differently",
                original: "the quick brown fox",
                current: "the slow brown fox",
                predicted: "the fast brown fox",
                expected_reversal_chars: 4,
                expected_total_chars: 8,
                explanation: indoc! {"
                    User changed 'quick'→'slow'. Prediction changes 'slow'→'fast'.
                    Prediction deletes 'slow' (4 chars, all user-added = 4 reversal).
                    Prediction adds 'fast' (4 chars). Char overlap between user-deleted 'quick'
                    and prediction-added 'fast' = 0. Total reversal = 4, total prediction = 8."},
            },
        ];

        for case in &cases {
            let overlap = compute_reversal_overlap(case.original, case.current, case.predicted);

            assert_eq!(
                overlap.chars_reversing_user_edits,
                case.expected_reversal_chars,
                "Test '{}' failed on reversal chars.\n\
                 Explanation: {}\n\
                 Got: {} reversal chars, expected: {}",
                case.name,
                case.explanation,
                overlap.chars_reversing_user_edits,
                case.expected_reversal_chars
            );

            assert_eq!(
                overlap.total_chars_in_prediction,
                case.expected_total_chars,
                "Test '{}' failed on total chars.\n\
                 Explanation: {}\n\
                 Got: {} total chars, expected: {}",
                case.name,
                case.explanation,
                overlap.total_chars_in_prediction,
                case.expected_total_chars
            );
        }
    }

    #[test]
    fn test_reversal_detection() {
        // Scenario: User writes some code, then the model predicts undoing part of it.
        // Original: fn main() { println!("hello"); }
        // History adds: let x = 42; and modifies print
        // Prediction removes the added line (reversal)

        let original_content = indoc! {r#"
            fn main() {
                println!("hello");
            }"#};

        let current_content = indoc! {r#"
            fn main() {
                let x = 42;
                println!("hello, x = {}", x);
            }"#};

        let predicted_content = indoc! {r#"
            fn main() {
                println!("hello, x = {}", x);
            }"#};

        // Step 1: Verify we can compute edits from original to current (what history did)
        let history_edits = compute_granular_edits(original_content, current_content);

        assert!(!history_edits.is_empty(), "History should have edits");

        let history_added_chars: usize = history_edits.iter().map(|e| e.new_text.len()).sum();
        let history_deleted_chars: usize = history_edits.iter().map(|e| e.old_text.len()).sum();

        assert!(
            history_added_chars > 0,
            "History should have added characters"
        );

        // Step 2: Verify we can compute edits from current to predicted (what prediction does)
        let prediction_edits = compute_granular_edits(current_content, predicted_content);

        assert!(!prediction_edits.is_empty(), "Prediction should have edits");

        let prediction_added_chars: usize = prediction_edits.iter().map(|e| e.new_text.len()).sum();
        let prediction_deleted_chars: usize =
            prediction_edits.iter().map(|e| e.old_text.len()).sum();

        // The prediction should delete more than it adds (removing the let x = 42 line)
        assert!(
            prediction_deleted_chars > prediction_added_chars,
            "Prediction should delete more than it adds"
        );

        // Step 3: Compute where history additions ended up in current_content
        let history_addition_ranges = compute_history_addition_ranges(&history_edits);

        assert!(
            !history_addition_ranges.is_empty(),
            "Should have history addition ranges"
        );

        // Verify the addition ranges point to the added text
        for range in &history_addition_ranges {
            let text_at_range = &current_content[range.range_in_current.clone()];
            assert_eq!(
                text_at_range, range.added_text,
                "Addition range should match the added text"
            );
        }

        // Step 4: Compute how much of the prediction deletes history additions
        let reversed_addition_chars = compute_reversed_additions(
            &history_addition_ranges,
            &prediction_edits,
            current_content,
        );

        // The "let x = 42;\n    " part was added by history, and prediction deletes it
        assert!(
            reversed_addition_chars > 0,
            "Should have reversed some additions"
        );

        // Step 5: Compute history deletions and restored deletions
        let history_deletion_ranges = compute_history_deletion_ranges(&history_edits);

        let restored_deletion_chars =
            compute_restored_deletions(&history_deletion_ranges, &prediction_edits);

        // Step 6: Calculate reversal ratio
        let total_prediction_change = prediction_added_chars + prediction_deleted_chars;
        let total_reversal = reversed_addition_chars + restored_deletion_chars;

        let reversal_ratio = if total_prediction_change > 0 {
            total_reversal as f64 / total_prediction_change as f64
        } else {
            0.0
        };

        // The majority of the prediction should be reversal since it's mostly removing
        // the line that was added
        assert!(
            reversal_ratio > 0.5,
            "Reversal ratio should be > 50%, got {:.2}%",
            reversal_ratio * 100.0
        );

        // Step 7: Verify the net change from original to predicted
        let net_edits = compute_granular_edits(original_content, predicted_content);
        let net_added_chars: usize = net_edits.iter().map(|e| e.new_text.len()).sum();
        let net_deleted_chars: usize = net_edits.iter().map(|e| e.old_text.len()).sum();

        // The net change should be smaller than history + prediction combined
        // because some changes cancel out
        let combined_change = history_added_chars
            + history_deleted_chars
            + prediction_added_chars
            + prediction_deleted_chars;
        let net_change = net_added_chars + net_deleted_chars;

        assert!(
            net_change < combined_change,
            "Net change ({}) should be less than combined change ({})",
            net_change,
            combined_change
        );

        println!("=== Reversal Detection Test Results ===");
        println!(
            "History: added {} chars, deleted {} chars",
            history_added_chars, history_deleted_chars
        );
        println!(
            "Prediction: added {} chars, deleted {} chars",
            prediction_added_chars, prediction_deleted_chars
        );
        println!("Reversed additions: {} chars", reversed_addition_chars);
        println!("Restored deletions: {} chars", restored_deletion_chars);
        println!("Reversal ratio: {:.2}%", reversal_ratio * 100.0);
        println!(
            "Net change: {} chars (vs combined {})",
            net_change, combined_change
        );
    }

    #[test]
    fn test_reverse_diff() {
        let forward_diff = indoc! {r#"
            --- a/file.rs
            +++ b/file.rs
            @@ -1,3 +1,4 @@
             fn main() {
            +    let x = 42;
                 println!("hello");
            }"#};

        let reversed = reverse_diff(forward_diff);

        assert!(
            reversed.contains("+++ a/file.rs"),
            "Should have +++ for old path"
        );
        assert!(
            reversed.contains("--- b/file.rs"),
            "Should have --- for new path"
        );
        assert!(
            reversed.contains("-    let x = 42;"),
            "Added line should become deletion"
        );
        assert!(
            reversed.contains(" fn main()"),
            "Context lines should be unchanged"
        );
    }

    #[test]
    fn test_reverse_diff_roundtrip() {
        // Applying a diff and then its reverse should get back to original
        let original = "first line\nhello world\nlast line\n";
        let modified = "first line\nhello beautiful world\nlast line\n";

        // unified_diff doesn't include file headers, but apply_diff_to_string needs them
        let diff_body = language::unified_diff(original, modified);
        let forward_diff = format!("--- a/file\n+++ b/file\n{}", diff_body);
        let reversed_diff = reverse_diff(&forward_diff);

        // Apply forward diff to original
        let after_forward = apply_diff_to_string(&forward_diff, original).unwrap();
        assert_eq!(after_forward, modified);

        // Apply reversed diff to modified
        let after_reverse = apply_diff_to_string(&reversed_diff, &after_forward).unwrap();
        assert_eq!(after_reverse, original);
    }

    #[test]
    fn test_history_addition_ranges_offset_tracking() {
        // Test that offset tracking works correctly across multiple edits
        let original = "abc";
        let current = "aXXbYYc";

        let edits = compute_granular_edits(original, current);

        // Should have two insertions: XX after a, YY after b
        let addition_ranges = compute_history_addition_ranges(&edits);

        // Verify each range points to the correct text in current
        for range in &addition_ranges {
            let actual_text = &current[range.range_in_current.clone()];
            assert_eq!(
                actual_text, range.added_text,
                "Range {:?} should contain '{}' but got '{}'",
                range.range_in_current, range.added_text, actual_text
            );
        }
    }

    #[test]
    fn test_restored_deletions() {
        // Test detecting when prediction adds back text that was deleted
        let original = "hello beautiful world";
        let current = "hello world"; // "beautiful " was deleted

        let history_edits = compute_granular_edits(original, current);
        let history_deletions = compute_history_deletion_ranges(&history_edits);

        // Verify we captured the deletion
        assert!(!history_deletions.is_empty());
        let deleted_text: String = history_deletions
            .iter()
            .map(|d| d.deleted_text.as_str())
            .collect();
        assert!(
            deleted_text.contains("beautiful"),
            "Should have captured 'beautiful' as deleted"
        );

        // Now if prediction adds "beautiful" back
        let prediction_edits = vec![GranularEdit {
            range: 6..6,
            old_text: String::new(),
            new_text: "beautiful ".to_string(),
        }];

        let restored = compute_restored_deletions(&history_deletions, &prediction_edits);

        // Should detect the restoration
        assert!(restored > 0, "Should detect restored deletion");
        assert!(
            restored >= "beautiful".len(),
            "Should restore at least 'beautiful'"
        );
    }

    #[test]
    fn test_no_reversal_when_edits_are_independent() {
        // When prediction edits different parts than history, there should be no reversal
        let original = "line1\nline2\nline3";
        let current = "LINE1\nline2\nline3"; // History changed line1

        let history_edits = compute_granular_edits(original, current);
        let history_addition_ranges = compute_history_addition_ranges(&history_edits);
        let history_deletion_ranges = compute_history_deletion_ranges(&history_edits);

        // Prediction changes line3 (independent of history)
        let predicted = "LINE1\nline2\nLINE3";
        let prediction_edits = compute_granular_edits(current, predicted);

        let reversed_additions =
            compute_reversed_additions(&history_addition_ranges, &prediction_edits, current);
        let restored_deletions =
            compute_restored_deletions(&history_deletion_ranges, &prediction_edits);

        assert_eq!(reversed_additions, 0, "Should not have reversed additions");
        assert_eq!(restored_deletions, 0, "Should not have restored deletions");
    }

    #[test]
    fn test_reversal_restoring_deleted_code() {
        // Scenario: User deletes a function, then the model predicts adding it back.

        let original_content = indoc! {r#"
            fn main() {
                helper();
            }

            fn helper() {
                println!("helping");
            }
            "#};

        let current_content = indoc! {r#"
            fn main() {
                helper();
            }
            "#};

        let predicted_content = indoc! {r#"
            fn main() {
                helper();
            }

            fn helper() {
            }
            "#};

        // Compute what history did (deleted the helper function body)
        let history_edits = compute_granular_edits(original_content, current_content);

        let history_added_chars: usize = history_edits.iter().map(|e| e.new_text.len()).sum();
        let history_deleted_chars: usize = history_edits.iter().map(|e| e.old_text.len()).sum();

        // History should have deleted text (the helper function)
        assert!(
            history_deleted_chars > 0,
            "History should have deleted characters"
        );
        assert_eq!(
            history_added_chars, 0,
            "History should not have added characters"
        );

        // Compute what prediction does
        let prediction_edits = compute_granular_edits(current_content, predicted_content);

        let prediction_added_chars: usize = prediction_edits.iter().map(|e| e.new_text.len()).sum();
        let prediction_deleted_chars: usize =
            prediction_edits.iter().map(|e| e.old_text.len()).sum();

        // Prediction should add text (restoring part of the function)
        assert!(
            prediction_added_chars > 0,
            "Prediction should have added characters"
        );

        // Compute reversal metrics
        let history_addition_ranges = compute_history_addition_ranges(&history_edits);
        let history_deletion_ranges = compute_history_deletion_ranges(&history_edits);

        let reversed_additions = compute_reversed_additions(
            &history_addition_ranges,
            &prediction_edits,
            current_content,
        );
        let restored_deletions =
            compute_restored_deletions(&history_deletion_ranges, &prediction_edits);

        // Since history only deleted (no additions), reversed_additions should be 0
        assert_eq!(
            reversed_additions, 0,
            "No reversed additions since history only deleted"
        );

        // Prediction added text that was previously deleted - should detect restored deletions
        assert!(restored_deletions > 0, "Should detect restored deletions");

        // The restored text should include at least "fn helper()" and "{" and "}"
        // Using character overlap, we should see significant restoration
        let total_reversal = reversed_additions + restored_deletions;
        let total_prediction_change = prediction_added_chars + prediction_deleted_chars;
        let reversal_ratio = total_reversal as f64 / total_prediction_change as f64;

        assert!(
            reversal_ratio > 0.3,
            "Reversal ratio should be significant, got {:.2}%",
            reversal_ratio * 100.0
        );

        println!("=== Restore Deletion Test Results ===");
        println!(
            "History: added {} chars, deleted {} chars",
            history_added_chars, history_deleted_chars
        );
        println!(
            "Prediction: added {} chars, deleted {} chars",
            prediction_added_chars, prediction_deleted_chars
        );
        println!("Reversed additions: {} chars", reversed_additions);
        println!("Restored deletions: {} chars", restored_deletions);
        println!("Reversal ratio: {:.2}%", reversal_ratio * 100.0);
    }

    #[test]
    fn test_filter_edit_history_by_path_with_prefix() {
        // Test that filter_edit_history_by_path correctly matches paths when
        // the edit history has paths with a repo prefix (e.g., "repo/file.md")
        // but the cursor_path is just the file name (e.g., "file.md")
        let events = vec![
            Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("myrepo/src/file.rs")),
                old_path: Arc::from(Path::new("myrepo/src/file.rs")),
                diff: "@@ -1 +1 @@\n-old\n+new".into(),
                predicted: false,
                in_open_source_repo: true,
            }),
            Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("myrepo/other.rs")),
                old_path: Arc::from(Path::new("myrepo/other.rs")),
                diff: "@@ -1 +1 @@\n-a\n+b".into(),
                predicted: false,
                in_open_source_repo: true,
            }),
            Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("src/file.rs")),
                old_path: Arc::from(Path::new("src/file.rs")),
                diff: "@@ -1 +1 @@\n-x\n+y".into(),
                predicted: false,
                in_open_source_repo: true,
            }),
        ];

        // When cursor_path is just "src/file.rs", it should match both
        // "myrepo/src/file.rs" (ends_with) and "src/file.rs" (exact)
        let cursor_path = Path::new("src/file.rs");
        let filtered = filter_edit_history_by_path(&events, cursor_path);
        assert_eq!(filtered.len(), 2, "Should match both paths ending with src/file.rs");

        // When cursor_path is "file.rs", it should match paths ending with "file.rs"
        let cursor_path = Path::new("file.rs");
        let filtered = filter_edit_history_by_path(&events, cursor_path);
        assert_eq!(filtered.len(), 2, "Should match myrepo/src/file.rs and src/file.rs");

        // When cursor_path is "other.rs", it should only match "myrepo/other.rs"
        let cursor_path = Path::new("other.rs");
        let filtered = filter_edit_history_by_path(&events, cursor_path);
        assert_eq!(filtered.len(), 1, "Should match only myrepo/other.rs");
    }
}
