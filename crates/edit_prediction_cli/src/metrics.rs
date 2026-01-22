use collections::HashMap;

pub type Counts = HashMap<String, usize>;
type CountsDelta = HashMap<String, isize>;

/// Context characters needed on each side of a change to capture all affected n-grams
const CONTEXT_CHARS: usize = CHR_F_CHAR_ORDER - 1;

#[derive(Default, Debug, Clone)]
pub struct ClassificationMetrics {
    pub true_positives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
}

impl ClassificationMetrics {
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

    pub fn f1(&self) -> f64 {
        let precision = self.precision();
        let recall = self.recall();
        if precision + recall == 0.0 {
            0.0
        } else {
            2.0 * precision * recall / (precision + recall)
        }
    }
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
/// 1. Computing n-gram count differences (deltas) between original→expected and original→actual
/// 2. Comparing these deltas to measure how well actual edits match expected edits
///
/// Returns a score from 0.0 to 100.0, where 100.0 means the actual edits perfectly match
/// the expected edits.
pub fn delta_chr_f(original: &str, expected: &str, actual: &str) -> f64 {
    // Edge case: if all texts are identical, the edits match perfectly
    if original == expected && expected == actual {
        return 100.0;
    }

    // Pre-filter whitespace once for all texts
    let orig_chars: Vec<char> = filter_whitespace_chars(original);
    let exp_chars: Vec<char> = filter_whitespace_chars(expected);
    let act_chars: Vec<char> = filter_whitespace_chars(actual);

    // Find the changed regions between original→expected and original→actual
    // We only need to compute n-grams on these regions (plus context for boundary n-grams)
    let (orig_for_exp, exp_region) = extract_changed_regions(&orig_chars, &exp_chars);
    let (orig_for_act, act_region) = extract_changed_regions(&orig_chars, &act_chars);

    let mut total_precision = 0.0;
    let mut total_recall = 0.0;

    for order in 1..=CHR_F_CHAR_ORDER {
        // Compute n-grams only on the affected regions
        let orig_ngrams_for_exp = count_ngrams_from_chars(&orig_for_exp, order);
        let exp_ngrams = count_ngrams_from_chars(&exp_region, order);
        let expected_delta = compute_ngram_delta(&exp_ngrams, &orig_ngrams_for_exp);

        let orig_ngrams_for_act = count_ngrams_from_chars(&orig_for_act, order);
        let act_ngrams = count_ngrams_from_chars(&act_region, order);
        let actual_delta = compute_ngram_delta(&act_ngrams, &orig_ngrams_for_act);

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

/// Reference implementation of delta_chr_f (original, non-optimized version).
/// Used for testing that the optimized version produces identical results.
#[cfg(test)]
fn delta_chr_f_reference(original: &str, expected: &str, actual: &str) -> f64 {
    if original == expected && expected == actual {
        return 100.0;
    }

    let original_ngrams = chr_f_ngram_counts(original);
    let expected_ngrams = chr_f_ngram_counts(expected);
    let actual_ngrams = chr_f_ngram_counts(actual);

    let mut total_precision = 0.0;
    let mut total_recall = 0.0;

    for order in 0..CHR_F_CHAR_ORDER {
        let expected_delta = compute_ngram_delta(&expected_ngrams[order], &original_ngrams[order]);
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

/// Filter whitespace from a string and return as Vec<char>
fn filter_whitespace_chars(text: &str) -> Vec<char> {
    match CHR_F_WHITESPACE {
        ChrfWhitespace::Unchanged => text.chars().collect(),
        ChrfWhitespace::Ignore => text.chars().filter(|c| !c.is_whitespace()).collect(),
    }
}

/// Extract only the changed regions between two texts, with context for n-gram boundaries.
///
/// Returns (original_affected_region, modified_affected_region) as Vec<char>.
///
/// The key insight: when computing n-gram delta between two nearly-identical texts,
/// n-grams from unchanged regions cancel out. We only need to process:
/// 1. The changed content itself
/// 2. CONTEXT_CHARS (n-1) characters before and after, to capture boundary-crossing n-grams
fn extract_changed_regions(original: &[char], modified: &[char]) -> (Vec<char>, Vec<char>) {
    // Find longest common prefix
    let prefix_len = original
        .iter()
        .zip(modified.iter())
        .take_while(|(a, b)| a == b)
        .count();

    // Find longest common suffix (that doesn't overlap with prefix)
    let orig_remaining = original.len().saturating_sub(prefix_len);
    let mod_remaining = modified.len().saturating_sub(prefix_len);
    let max_suffix = orig_remaining.min(mod_remaining);

    let suffix_len = original
        .iter()
        .rev()
        .zip(modified.iter().rev())
        .take(max_suffix)
        .take_while(|(a, b)| a == b)
        .count();

    // Calculate the changed region boundaries
    let orig_change_start = prefix_len;
    let orig_change_end = original.len().saturating_sub(suffix_len);
    let mod_change_start = prefix_len;
    let mod_change_end = modified.len().saturating_sub(suffix_len);

    // If there's no actual change, return empty regions
    if orig_change_start >= orig_change_end && mod_change_start >= mod_change_end {
        return (Vec::new(), Vec::new());
    }

    // Expand to include context for n-gram boundaries
    let orig_context_start = orig_change_start.saturating_sub(CONTEXT_CHARS);
    let orig_context_end = (orig_change_end + CONTEXT_CHARS).min(original.len());
    let mod_context_start = mod_change_start.saturating_sub(CONTEXT_CHARS);
    let mod_context_end = (mod_change_end + CONTEXT_CHARS).min(modified.len());

    let orig_region: Vec<char> = original[orig_context_start..orig_context_end].to_vec();
    let mod_region: Vec<char> = modified[mod_context_start..mod_context_end].to_vec();

    (orig_region, mod_region)
}

/// Count n-grams directly from a char slice (avoids String allocation for the full text)
fn count_ngrams_from_chars(chars: &[char], n: usize) -> Counts {
    let mut counts = Counts::default();

    if chars.len() < n {
        return counts;
    }

    for window in chars.windows(n) {
        let ngram: String = window.iter().collect();
        *counts.entry(ngram).or_insert(0) += 1;
    }

    counts
}

#[allow(dead_code)]
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
        } else if delta < 0 {
            counts.insert(format!("¬{ngram}"), delta.unsigned_abs());
        }
    }

    counts
}

#[allow(dead_code)]
fn count_ngrams(text: &str, n: usize) -> Counts {
    let chars: Vec<char> = text.chars().collect();
    let mut counts = Counts::default();

    for window in chars.windows(n) {
        let ngram: String = window.iter().collect();
        *counts.entry(ngram).or_insert(0) += 1;
    }

    counts
}

pub fn braces_disbalance(text: &str) -> usize {
    let mut disbalance = 0isize;

    let a = text.chars().filter(|&c| c == '{').count() as isize;
    let b = text.chars().filter(|&c| c == '}').count() as isize;
    disbalance += (a - b).abs();

    let a = text.chars().filter(|&c| c == '(').count() as isize;
    let b = text.chars().filter(|&c| c == ')').count() as isize;
    disbalance += (a - b).abs();

    let a = text.chars().filter(|&c| c == '[').count() as isize;
    let b = text.chars().filter(|&c| c == ']').count() as isize;
    disbalance += (a - b).abs();

    disbalance as usize
}

/// Extracts changed lines from a unified diff string.
/// Returns a bag (multiset) of lines that were added (+) or removed (-).
/// The +/- prefix is included in the line to distinguish additions from deletions.
pub fn extract_changed_lines_from_diff(diff: &str) -> Counts {
    let mut counts = Counts::default();

    for line in diff.lines() {
        // Skip file headers (--- and +++)
        if line.starts_with("---") || line.starts_with("+++") {
            continue;
        }
        // Skip hunk headers (@@)
        if line.starts_with("@@") {
            continue;
        }
        // Skip diff header lines (diff --git, index, etc.)
        if line.starts_with("diff ") || line.starts_with("index ") {
            continue;
        }
        // Include added and removed lines (with their prefix)
        if line.starts_with('+') || line.starts_with('-') {
            *counts.entry(line.to_string()).or_insert(0) += 1;
        }
    }

    counts
}

/// Computes exact lines match metrics between expected and actual patches.
/// Treats changed lines as a bag (multiset) - order is discarded but count matters.
/// Returns ClassificationMetrics with TP/FP/FN counts.
pub fn exact_lines_match(expected_patch: &str, actual_patch: &str) -> ClassificationMetrics {
    let expected_lines = extract_changed_lines_from_diff(expected_patch);
    let actual_lines = extract_changed_lines_from_diff(actual_patch);
    ClassificationMetrics::from_counts(&expected_lines, &actual_lines)
}

#[cfg(test)]
mod test_optimization {
    use super::*;

    #[test]
    fn test_extract_changed_regions_simple() {
        let original: Vec<char> = "hello world".chars().collect();
        let modified: Vec<char> = "hello there".chars().collect();

        let (orig_region, mod_region) = extract_changed_regions(&original, &modified);

        // "world" vs "there" - with 5 chars context, we get "ello world" vs "ello there"
        // (or less if not enough chars available)
        assert!(orig_region.len() < original.len());
        assert!(mod_region.len() < modified.len());
    }

    #[test]
    fn test_extract_changed_regions_insertion() {
        let original: Vec<char> = "abcdef".chars().collect();
        let modified: Vec<char> = "abcXYZdef".chars().collect();

        let (orig_region, mod_region) = extract_changed_regions(&original, &modified);

        // The insertion is between c and d, so we need context around that point
        assert!(orig_region.len() <= original.len());
        assert!(mod_region.iter().collect::<String>().contains("XYZ"));
    }

    #[test]
    fn test_extract_changed_regions_identical() {
        let text: Vec<char> = "identical text".chars().collect();

        let (orig_region, mod_region) = extract_changed_regions(&text, &text);

        // When texts are identical, regions should be empty
        assert!(orig_region.is_empty());
        assert!(mod_region.is_empty());
    }

    #[test]
    fn test_optimized_matches_original_score() {
        // Test that our optimized version produces the same results
        let test_cases = vec![
            ("hello world", "hello there", "hello world"),
            (
                "fn main() {}",
                "fn main() { println!(); }",
                "fn main() { print!(); }",
            ),
            ("abcdefghij", "abcXXXghij", "abcYYghij"),
            ("unchanged", "unchanged", "unchanged"),
            (
                "prefix middle suffix",
                "prefix CHANGED suffix",
                "prefix middle suffix",
            ),
        ];

        for (original, expected, actual) in test_cases {
            let score = delta_chr_f(original, expected, actual);
            // Just verify it produces a reasonable score (0-100)
            assert!(
                score >= 0.0 && score <= 100.0,
                "Score {} out of range for ({}, {}, {})",
                score,
                original,
                expected,
                actual
            );
        }
    }

    #[test]
    fn test_optimized_equals_reference() {
        // Comprehensive test that optimized version matches reference implementation exactly
        let test_cases = vec![
            // Basic cases
            ("hello world", "hello there", "hello world"),
            ("hello world", "hello there", "hello there"),
            ("unchanged", "unchanged", "unchanged"),
            // Code-like cases
            (
                "fn main() { println!(\"Hello\"); }",
                "fn main() { println!(\"Hello, World!\"); }",
                "fn main() { println!(\"Hello, World!\"); }",
            ),
            (
                "fn main() { println!(\"Hello\"); }",
                "fn main() { println!(\"Hello, World!\"); }",
                "fn main() { println!(\"Goodbye\"); }",
            ),
            // Insertion
            ("abcdef", "abcXYZdef", "abcdef"),
            ("abcdef", "abcXYZdef", "abcXYZdef"),
            ("abcdef", "abcXYZdef", "abcABCdef"),
            // Deletion
            ("abcXYZdef", "abcdef", "abcXYZdef"),
            ("abcXYZdef", "abcdef", "abcdef"),
            // Multiple changes (simulated by different expected/actual)
            ("one two three four", "one THREE four", "one two FOUR"),
            // Edge cases
            ("a", "b", "c"),
            ("", "abc", ""),
            ("abc", "", "abc"),
            // Longer text with small change
            (
                "This is a longer piece of text that contains many words and characters to process",
                "This is a longer piece of TEXT that contains many words and characters to process",
                "This is a longer piece of text that contains many words and characters to process",
            ),
            // Change at the beginning
            (
                "ORIGINAL start of text",
                "NEW start of text",
                "DIFFERENT start of text",
            ),
            // Change at the end
            (
                "text ending ORIGINAL",
                "text ending NEW",
                "text ending DIFFERENT",
            ),
            // Whitespace (should be ignored)
            ("hello   world", "hello   there", "hello   world"),
            ("a b c d", "a X c d", "a Y c d"),
        ];

        for (original, expected, actual) in test_cases {
            let optimized_score = delta_chr_f(original, expected, actual);
            let reference_score = delta_chr_f_reference(original, expected, actual);

            assert!(
                (optimized_score - reference_score).abs() < 1e-10,
                "Mismatch for ({:?}, {:?}, {:?}):\n  optimized: {}\n  reference: {}",
                original,
                expected,
                actual,
                optimized_score,
                reference_score
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_delta_chr_f_perfect_match() {
        let original = "fn main() {    println!(\"Hello\");}";
        let expected = "fn main() {    println!(\"Hello, World!\");}";

        let score = delta_chr_f(original, expected, expected);
        assert!((score - 100.0).abs() < 1e-2);
    }

    #[test]
    fn test_delta_chr_f_wrong_edit() {
        // When the edit is wrong
        let original = "one two three";
        let expected = "one three"; // deleted "two "
        let actual = "one two four"; // deleted "three", added "four"

        // Then the score should be low
        let score = delta_chr_f(original, expected, actual);
        assert!(score > 20.0 && score < 40.0);
    }

    #[test]
    fn test_delta_chr_f_partial_match() {
        let original = "let x = 42;";
        let expected = "let x = 100;";
        let actual = "let x = 99;";

        // We got the edit location right, but the replacement text is wrong.
        // Deleted ngrams will match, bringing the score somewhere in the middle.
        let score = delta_chr_f(original, expected, actual);
        assert!(score > 40.0 && score < 60.0);
    }

    #[test]
    fn test_delta_chr_f_missed_edit() {
        // When predictions makes no changes
        let original = "prefix old suffix";
        let expected = "prefix new suffix";
        let actual = "prefix old suffix"; // no change

        // Then the score should be low (all expected changes are false negatives)
        let score = delta_chr_f(original, expected, actual);
        assert!(score < 20.0);
    }

    #[test]
    fn test_delta_chr_f_extra_edit() {
        // When adding unexpected content
        let original = "helloworld";
        let expected = "helloworld"; // no change expected
        let actual = "helloextraworld"; // added "extra"

        // Then the score should be low (all actual changes are false positives)
        let score = delta_chr_f(original, expected, actual);
        assert!(score < 20.0);
    }

    #[test]
    fn test_delta_chr_f_no_changes() {
        let text = "unchanged text";
        let score = delta_chr_f(text, text, text);
        assert!((score - 100.0).abs() < 1e-2);
    }

    #[test]
    fn test_braces_disbalance() {
        let text = "let x = { 1 + 2 };";
        assert_eq!(braces_disbalance(text), 0);

        let text = "let x = { 1 + 2";
        assert_eq!(braces_disbalance(text), 1);

        let text = "let x = { 1 + 2 )";
        assert_eq!(braces_disbalance(text), 2);
    }

    #[test]
    fn test_extract_changed_lines_from_diff() {
        let diff = r#"--- a/file.rs
+++ b/file.rs
@@ -1,3 +1,3 @@
 fn main() {
-    println!("hello");
+    println!("world");
 }"#;

        let counts = extract_changed_lines_from_diff(diff);
        assert_eq!(counts.get("-    println!(\"hello\");"), Some(&1));
        assert_eq!(counts.get("+    println!(\"world\");"), Some(&1));
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn test_extract_changed_lines_skips_headers() {
        let diff = r#"diff --git a/file.rs b/file.rs
index abc123..def456 100644
--- a/file.rs
+++ b/file.rs
@@ -1,2 +1,2 @@
-old line
+new line"#;

        let counts = extract_changed_lines_from_diff(diff);
        assert_eq!(counts.get("-old line"), Some(&1));
        assert_eq!(counts.get("+new line"), Some(&1));
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn test_exact_lines_match_perfect() {
        let expected = r#"--- a/file.rs
+++ b/file.rs
@@ -1,3 +1,3 @@
-old line 1
-old line 2
+new line 1
+new line 2"#;

        let actual = r#"--- a/file.rs
+++ b/file.rs
@@ -1,3 +1,3 @@
-old line 1
-old line 2
+new line 1
+new line 2"#;

        let metrics = exact_lines_match(expected, actual);
        assert_eq!(metrics.true_positives, 4);
        assert_eq!(metrics.false_positives, 0);
        assert_eq!(metrics.false_negatives, 0);
        assert!((metrics.precision() - 1.0).abs() < 1e-6);
        assert!((metrics.recall() - 1.0).abs() < 1e-6);
        assert!((metrics.f1() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_exact_lines_match_partial() {
        let expected = r#"-old line 1
-old line 2
+new line 1
+new line 2"#;

        let actual = r#"-old line 1
+new line 1
+extra line"#;

        let metrics = exact_lines_match(expected, actual);
        // TP: "-old line 1" and "+new line 1" (2)
        // FP: "+extra line" (1)
        // FN: "-old line 2" and "+new line 2" (2)
        assert_eq!(metrics.true_positives, 2);
        assert_eq!(metrics.false_positives, 1);
        assert_eq!(metrics.false_negatives, 2);
    }

    #[test]
    fn test_exact_lines_match_no_overlap() {
        let expected = r#"-line a
+line b"#;

        let actual = r#"-line x
+line y"#;

        let metrics = exact_lines_match(expected, actual);
        assert_eq!(metrics.true_positives, 0);
        assert_eq!(metrics.false_positives, 2);
        assert_eq!(metrics.false_negatives, 2);
        assert!((metrics.precision()).abs() < 1e-6);
        assert!((metrics.recall()).abs() < 1e-6);
    }

    #[test]
    fn test_exact_lines_match_duplicate_lines() {
        let expected = r#"+line a
+line a
+line a"#;

        let actual = r#"+line a
+line a"#;

        let metrics = exact_lines_match(expected, actual);
        // Expected has 3 "+line a", actual has 2
        // TP: 2, FN: 1, FP: 0
        assert_eq!(metrics.true_positives, 2);
        assert_eq!(metrics.false_positives, 0);
        assert_eq!(metrics.false_negatives, 1);
    }

    #[test]
    fn test_exact_lines_match_empty_patches() {
        let metrics = exact_lines_match("", "");
        assert_eq!(metrics.true_positives, 0);
        assert_eq!(metrics.false_positives, 0);
        assert_eq!(metrics.false_negatives, 0);
    }
}
