use collections::{HashMap, HashSet};
use zeta::udiff::DiffLine;

type Counts = HashMap<String, usize>;
type CountsDelta = HashMap<String, isize>;

#[derive(Default, Debug, Clone)]
pub struct Scores {
    pub true_positives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
}

impl Scores {
    pub fn from_sets(expected: &HashSet<String>, actual: &HashSet<String>) -> Scores {
        let true_positives = expected.intersection(actual).count();
        let false_positives = actual.difference(expected).count();
        let false_negatives = expected.difference(actual).count();

        Scores {
            true_positives,
            false_positives,
            false_negatives,
        }
    }

    pub fn from_counts(expected: &Counts, actual: &Counts) -> Scores {
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

pub fn line_match_score(expected_patch: &[DiffLine], actual_patch: &[DiffLine]) -> Scores {
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

    Scores::from_sets(&expected_change_lines, &actual_change_lines)
}

pub fn chr_f(expected: &str, actual: &str) -> f64 {
    const CHAR_ORDER: usize = 6;
    const BETA: f64 = 2.0;

    // Special case: empty strings
    if expected.is_empty() && actual.is_empty() {
        return 100.0;
    }

    let expected_ngrams = chr_f_ngram_counts(expected);
    let actual_ngrams = chr_f_ngram_counts(actual);

    // Compute precision and recall for each n-gram order, then average
    let mut total_precision = 0.0;
    let mut total_recall = 0.0;

    for order in 0..CHAR_ORDER {
        let score = Scores::from_counts(&expected_ngrams[order], &actual_ngrams[order]);

        total_precision += score.precision();
        total_recall += score.recall();
    }

    // Compute chrF
    let prec = total_precision / CHAR_ORDER as f64;
    let recall = total_recall / CHAR_ORDER as f64;
    let f_score = if prec + recall == 0.0 {
        0.0
    } else {
        (1.0 + BETA * BETA) * prec * recall / (BETA * BETA * prec + recall)
    };

    f_score * 100.0
}

/// Compute character n-gram counts to be used in chrF computation
pub fn chr_f_ngram_counts(text: &str) -> Vec<Counts> {
    const CHAR_ORDER: usize = 6;
    const IGNORE_WHITESPACE: bool = true;

    // Ignore whitespace. The original chrF implementation skips all
    // whitespace. We should consider compressing multiple consecutive
    // spaces into one -- this may reflect our task more closely.
    let text = if IGNORE_WHITESPACE {
        text.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    } else {
        text.to_string()
    };

    (1..=CHAR_ORDER)
        .map(|order| get_ngram_counts(&text, order))
        .collect()
}

/// Computes a diff-aware chrF score for comparing predicted edits
/// against expected edits.
///
/// Standard chrF is usually used for comparing single code completions
/// (old-school code autocompletion), but Zeta edits can modify existing
/// code across multiple locations. Naively comparing entire files after
/// applying edits would result in a score dominated by unchanged context
/// lines.
///
/// This metric addresses that by:
/// 1. Extracting only the changed lines from each patch
/// 2. Computing chrF separately for deletions and insertions
/// 3. Combining them via harmonic mean.
///
/// The harmonic mean ensures both deletion and insertion accuracy
/// contribute to the final score. A patch that perfectly matches
/// deletions but misses insertions (or vice versa) will be penalized
/// appropriately.
pub fn patch_chr_f(expected: &[DiffLine], actual: &[DiffLine]) -> f64 {
    let mut expected_ins = String::default();
    let mut expected_del = String::default();
    let mut actual_ins = String::default();
    let mut actual_del = String::default();

    for line in expected {
        match line {
            DiffLine::Deletion(s) => expected_del.push_str(s),
            DiffLine::Addition(s) => expected_ins.push_str(s),
            _ => (),
        };
    }

    for line in actual {
        match line {
            DiffLine::Deletion(s) => actual_del.push_str(s),
            DiffLine::Addition(s) => actual_ins.push_str(s),
            _ => (),
        };
    }

    let score_del = chr_f(&expected_del, &actual_del);
    let score_ins = chr_f(&expected_ins, &actual_ins);

    let score = 2.0 * score_del * score_ins / (score_del + score_ins + 0.00001);

    score
}

/// Computes a delta-based chrF score that compares the *changes* made by edits
/// rather than the resulting text.
///
/// This metric works by:
/// 1. Reconstructing original, golden (expected result), and actual texts from diffs
/// 2. Computing n-gram count differences (deltas) between original→golden and original→actual
/// 3. Comparing these deltas to measure how well actual edits match expected edits
///
/// This approach ensures that unchanged context doesn't dominate the score - only
/// the actual changes (additions and removals) contribute to precision/recall.
pub fn delta_chr_f(expected: &[DiffLine], actual: &[DiffLine]) -> f64 {
    const CHAR_ORDER: usize = 6;
    const BETA: f64 = 2.0;

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

    // Edge case: no changes expected and no changes made
    if original_text == golden_text && golden_text == actual_text {
        return 100.0;
    }

    // Compute n-gram counts for all three texts
    let original_ngrams = chr_f_ngram_counts(&original_text);
    let golden_ngrams = chr_f_ngram_counts(&golden_text);
    let actual_ngrams = chr_f_ngram_counts(&actual_text);

    let mut total_precision = 0.0;
    let mut total_recall = 0.0;

    for order in 0..CHAR_ORDER {
        let expected_delta = compute_ngram_delta(&golden_ngrams[order], &original_ngrams[order]);
        let actual_delta = compute_ngram_delta(&actual_ngrams[order], &original_ngrams[order]);

        if expected_delta.is_empty() && actual_delta.is_empty() {
            total_precision += 1.0;
            total_recall += 1.0;
            continue;
        }

        let expected_counts = ngram_delta_to_counts(&expected_delta);
        let actual_counts = ngram_delta_to_counts(&actual_delta);

        let score = Scores::from_counts(&expected_counts, &actual_counts);
        total_precision += score.precision();
        total_recall += score.recall();
    }

    let prec = total_precision / CHAR_ORDER as f64;
    let recall = total_recall / CHAR_ORDER as f64;
    let f_score = if prec + recall == 0.0 {
        0.0
    } else {
        (1.0 + BETA * BETA) * prec * recall / (BETA * BETA * prec + recall)
    };

    f_score * 100.0
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
            counts.insert(format!("¬{ngram}"), delta.abs() as usize);
        }
    }

    counts
}

fn get_ngram_counts(text: &str, n: usize) -> Counts {
    let chars: Vec<char> = text.chars().collect();
    let mut counts = Counts::default();

    for window in chars.windows(n) {
        let ngram: String = window.iter().collect();
        *counts.entry(ngram).or_insert(0) += 1;
    }

    counts
}

#[test]
fn test_chr_f_normal() {
    let reference = "let s = \"Привіт!\";";
    let hypothesis = "let mut s = \"Hello!\";";
    let score = chr_f(reference, hypothesis);
    assert!((score - 24.16).abs() < 1e-2);
}

#[test]
fn test_chr_f_empty() {
    let reference = "";
    let hypothesis = "";
    let score = chr_f(reference, hypothesis);
    assert!((score - 100.00).abs() < 1e-2);
}

#[test]
fn test_patch_chr_f_perfect_match() {
    use zeta::udiff::DiffLine;

    let diff = vec![
        DiffLine::Context("fn main() {"),
        DiffLine::Deletion("    println!(\"Hello\");"),
        DiffLine::Addition("    println!(\"Hello, World!\");"),
        DiffLine::Context("}"),
    ];

    let score = patch_chr_f(&diff, &diff);
    assert!((score - 100.0).abs() < 1e-2);
}

#[test]
fn test_patch_chr_f_partial_match() {
    use zeta::udiff::DiffLine;

    let expected = vec![
        DiffLine::Deletion("let x = 42;"),
        DiffLine::Addition("let x = 100;"),
    ];

    let actual = vec![
        DiffLine::Deletion("let x = 42;"),
        DiffLine::Addition("let x = 99;"),
    ];

    let score = patch_chr_f(&expected, &actual);
    // Deletions match perfectly, insertions differ slightly
    // Score should be high but not perfect
    assert!(score > 50.0 && score < 100.0);
}

#[test]
fn test_patch_chr_f_empty_diffs() {
    use zeta::udiff::DiffLine;

    // Both diffs have only context lines (no actual changes)
    let expected = vec![
        DiffLine::Context("fn foo() {}"),
        DiffLine::Context("fn bar() {}"),
    ];

    let actual = vec![
        DiffLine::Context("fn foo() {}"),
        DiffLine::Context("fn bar() {}"),
    ];

    let score = patch_chr_f(&expected, &actual);
    // Empty strings compared via chr_f return 100.0, harmonic mean ~100.0
    assert!((score - 100.0).abs() < 1e-2);
}

#[test]
fn test_delta_chr_f_perfect_match() {
    use zeta::udiff::DiffLine;

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
    use zeta::udiff::DiffLine;

    // Expected: remove "two"
    let expected = vec![
        DiffLine::Context("one "),
        DiffLine::Deletion("two "),
        DiffLine::Context("three"),
    ];

    // Actual: remove "three" and add "four"
    let actual = vec![
        DiffLine::Context("one "),
        DiffLine::Context("two "),
        DiffLine::Deletion("three"),
        DiffLine::Addition("four"),
    ];

    // Score should be low
    let score = delta_chr_f(&expected, &actual);
    assert!(score > 20.0 && score < 40.0);
}

#[test]
fn test_delta_chr_f_partial_match() {
    use zeta::udiff::DiffLine;

    let expected = vec![
        DiffLine::Deletion("let x = 42;"),
        DiffLine::Addition("let x = 100;"),
    ];

    let actual = vec![
        DiffLine::Deletion("let x = 42;"),
        DiffLine::Addition("let x = 99;"),
    ];

    // We got the edit location right, but new text is wrong.
    // Deleted ngrams will match, while inserted won't
    let score = delta_chr_f(&expected, &actual);
    assert!(score > 40.0 && score < 60.0);
}

#[test]
fn test_delta_chr_f_missed_edit() {
    use zeta::udiff::DiffLine;

    // Expected: remove "old" and add "new"
    // Actual: no changes (just context)
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

    let score = delta_chr_f(&expected, &actual);
    // Should be low - all expected changes are false negatives
    assert!(score < 20.0);
}

#[test]
fn test_delta_chr_f_extra_edit() {
    use zeta::udiff::DiffLine;

    // Expected: no changes
    // Actual: added unexpected content
    let expected = vec![DiffLine::Context("hello"), DiffLine::Context("world")];

    let actual = vec![
        DiffLine::Context("hello"),
        DiffLine::Addition("extra"),
        DiffLine::Context("world"),
    ];

    let score = delta_chr_f(&expected, &actual);
    // Should be low - all actual changes are false positives
    assert!(score < 20.0);
}
