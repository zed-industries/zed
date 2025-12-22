use collections::{HashMap, HashSet};
use edit_prediction::udiff::DiffLine;
use serde::{Deserialize, Serialize};

type Counts = HashMap<String, usize>;
type CountsDelta = HashMap<String, isize>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationMetrics {
    pub true_positives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
}

impl ClassificationMetrics {
    pub fn from_sets(
        expected: &HashSet<String>,
        actual: &HashSet<String>,
    ) -> ClassificationMetrics {
        let true_positives = expected.intersection(actual).count();
        let false_positives = actual.difference(expected).count();
        let false_negatives = expected.difference(actual).count();

        ClassificationMetrics {
            true_positives,
            false_positives,
            false_negatives,
        }
    }

    pub fn from_counts(expected: &Counts, actual: &Counts) -> ClassificationMetrics {
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for (ngram, &expected_count) in expected {
            let actual_count = *actual.get(ngram).unwrap_or(&0);
            if actual_count > expected_count {
                false_positives += actual_count - expected_count;
            } else {
                false_negatives += expected_count - actual_count;
            }
            true_positives += expected_count.min(actual_count);
        }

        for (ngram, &actual_count) in actual {
            if !expected.contains_key(ngram) {
                false_positives += actual_count;
            }
        }

        ClassificationMetrics {
            true_positives,
            false_positives,
            false_negatives,
        }
    }

    pub fn aggregate<'a>(
        scores: impl Iterator<Item = &'a ClassificationMetrics>,
    ) -> ClassificationMetrics {
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for score in scores {
            true_positives += score.true_positives;
            false_positives += score.false_positives;
            false_negatives += score.false_negatives;
        }

        ClassificationMetrics {
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

pub fn line_match_score(
    expected_patch: &[DiffLine],
    actual_patch: &[DiffLine],
) -> ClassificationMetrics {
    let expected_change_lines = expected_patch
        .iter()
        .filter(|line| matches!(line, DiffLine::Addition(_) | DiffLine::Deletion(_)))
        .map(|line| line.to_string())
        .collect();

    let actual_change_lines = actual_patch
        .iter()
        .filter(|line| matches!(line, DiffLine::Addition(_) | DiffLine::Deletion(_)))
        .map(|line| line.to_string())
        .collect();

    ClassificationMetrics::from_sets(&expected_change_lines, &actual_change_lines)
}

enum ChrfWhitespace {
    #[allow(unused)]
    Unchanged,
    Ignore,
}

const CHR_F_CHAR_ORDER: usize = 6;
const CHR_F_BETA: f64 = 2.0;
const CHR_F_WHITESPACE: ChrfWhitespace = ChrfWhitespace::Ignore;

/// Computes a delta-chrF score that compares two sets of edits.
///
/// This metric works by:
/// 1. Reconstructing original, golden (expected result), and actual texts from diffs
/// 2. Computing n-gram count differences (deltas) between original→golden and original→actual
/// 3. Comparing these deltas to measure how well actual edits match expected edits
pub fn delta_chr_f(expected: &[DiffLine], actual: &[DiffLine]) -> f64 {
    // Reconstruct texts from diffs
    let mut original_text = String::new(); // state of the text before any edits
    let mut golden_text = String::new(); // text after applying golden edits
    let mut actual_text = String::new(); // text after applying actual edits

    for line in expected {
        match line {
            DiffLine::Context(s) => {
                original_text.push_str(s);
                golden_text.push_str(s);
            }
            DiffLine::Deletion(s) => {
                original_text.push_str(s);
            }
            DiffLine::Addition(s) => {
                golden_text.push_str(s);
            }
            _ => {}
        }
    }

    for line in actual {
        match line {
            DiffLine::Context(s) | DiffLine::Addition(s) => {
                actual_text.push_str(s);
            }
            _ => {}
        }
    }

    // Edge case
    if original_text == golden_text && golden_text == actual_text {
        return 100.0;
    }

    // Compute the metric
    let original_ngrams = chr_f_ngram_counts(&original_text);
    let golden_ngrams = chr_f_ngram_counts(&golden_text);
    let actual_ngrams = chr_f_ngram_counts(&actual_text);

    let mut total_precision = 0.0;
    let mut total_recall = 0.0;

    for order in 0..CHR_F_CHAR_ORDER {
        let expected_delta = compute_ngram_delta(&golden_ngrams[order], &original_ngrams[order]);
        let actual_delta = compute_ngram_delta(&actual_ngrams[order], &original_ngrams[order]);

        if expected_delta.is_empty() && actual_delta.is_empty() {
            total_precision += 1.0;
            total_recall += 1.0;
            continue;
        }

        let expected_counts = ngram_delta_to_counts(&expected_delta);
        let actual_counts = ngram_delta_to_counts(&actual_delta);

        let score = ClassificationMetrics::from_counts(&expected_counts, &actual_counts);
        total_precision += score.precision();
        total_recall += score.recall();
    }

    let prec = total_precision / CHR_F_CHAR_ORDER as f64;
    let recall = total_recall / CHR_F_CHAR_ORDER as f64;
    let f_score = if prec + recall == 0.0 {
        0.0
    } else {
        (1.0 + CHR_F_BETA * CHR_F_BETA) * prec * recall / (CHR_F_BETA * CHR_F_BETA * prec + recall)
    };

    f_score * 100.0
}

fn chr_f_ngram_counts(text: &str) -> Vec<Counts> {
    // Ignore whitespace. The original chrF implementation skips all
    // whitespace. We should consider compressing multiple consecutive
    // spaces into one -- this may reflect our task more closely.
    let text = match CHR_F_WHITESPACE {
        ChrfWhitespace::Unchanged => text.to_string(),
        ChrfWhitespace::Ignore => text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
    };

    (1..=CHR_F_CHAR_ORDER)
        .map(|order| count_ngrams(&text, order))
        .collect()
}

fn compute_ngram_delta(after: &Counts, before: &Counts) -> CountsDelta {
    let mut delta = CountsDelta::default();

    for (ngram, &before_count) in before {
        let after_count = *after.get(ngram).unwrap_or(&0);
        delta.insert(ngram.clone(), after_count as isize - before_count as isize);
    }

    for (ngram, &after_count) in after {
        if !before.contains_key(ngram) {
            delta.insert(ngram.clone(), after_count as isize);
        }
    }

    delta
}

/// Convert negative counts to special deletion tokens.
/// For example, if expected delta is {"foo": -1} and actual delta is {"bar": -1},
/// we convert it to {"¬foo": +1} and {"¬bar": +1}. This way _not_ deleting "foo"
/// will result in a false negative, and mistakenly deleting "bar" will result in a false positive.
fn ngram_delta_to_counts(delta: &CountsDelta) -> Counts {
    let mut counts = Counts::default();

    for (ngram, &delta) in delta {
        if delta > 0 {
            counts.insert(ngram.clone(), delta as usize);
        } else {
            counts.insert(format!("¬{ngram}"), delta.unsigned_abs());
        }
    }

    counts
}

fn count_ngrams(text: &str, n: usize) -> Counts {
    let chars: Vec<char> = text.chars().collect();
    let mut counts = Counts::default();

    for window in chars.windows(n) {
        let ngram: String = window.iter().collect();
        *counts.entry(ngram).or_insert(0) += 1;
    }

    counts
}

#[cfg(test)]
mod test {
    use super::*;
    use edit_prediction::udiff::DiffLine;

    #[test]
    fn test_delta_chr_f_perfect_match() {
        let diff = vec![
            DiffLine::Context("fn main() {"),
            DiffLine::Deletion("    println!(\"Hello\");"),
            DiffLine::Addition("    println!(\"Hello, World!\");"),
            DiffLine::Context("}"),
        ];

        let score = delta_chr_f(&diff, &diff);
        assert!((score - 100.0).abs() < 1e-2);
    }

    #[test]
    fn test_delta_chr_f_wrong_edit() {
        // When the edit is wrong
        let expected = vec![
            DiffLine::Context("one "),
            DiffLine::Deletion("two "),
            DiffLine::Context("three"),
        ];

        let actual = vec![
            DiffLine::Context("one "),
            DiffLine::Context("two "),
            DiffLine::Deletion("three"),
            DiffLine::Addition("four"),
        ];

        // Then the score should be low
        let score = delta_chr_f(&expected, &actual);
        assert!(score > 20.0 && score < 40.0);
    }

    #[test]
    fn test_delta_chr_f_partial_match() {
        let expected = vec![
            DiffLine::Deletion("let x = 42;"),
            DiffLine::Addition("let x = 100;"),
        ];

        let actual = vec![
            DiffLine::Deletion("let x = 42;"),
            DiffLine::Addition("let x = 99;"),
        ];

        // We got the edit location right, but the replacement text is wrong.
        // Deleted ngrams will match, bringing the score somewhere in the middle.
        let score = delta_chr_f(&expected, &actual);
        assert!(score > 40.0 && score < 60.0);
    }

    #[test]
    fn test_delta_chr_f_missed_edit() {
        // When predictions makes no changes
        let expected = vec![
            DiffLine::Context("prefix "),
            DiffLine::Deletion("old"),
            DiffLine::Addition("new"),
            DiffLine::Context(" suffix"),
        ];

        let actual = vec![
            DiffLine::Context("prefix "),
            DiffLine::Context("old"),
            DiffLine::Context(" suffix"),
        ];

        // Then the score should be low (all expected changes are false negatives)
        let score = delta_chr_f(&expected, &actual);
        assert!(score < 20.0);
    }

    #[test]
    fn test_delta_chr_f_extra_edit() {
        // When adding unexpected content
        let expected = vec![DiffLine::Context("hello"), DiffLine::Context("world")];

        let actual = vec![
            DiffLine::Context("hello"),
            DiffLine::Addition("extra"),
            DiffLine::Context("world"),
        ];

        // Then the score should be low (all actual changes are false positives)
        let score = delta_chr_f(&expected, &actual);
        assert!(score < 20.0);
    }
}
