use std::collections::HashMap;

use crate::tokenize::tokenize;
use serde::Serialize;
use similar::{DiffTag, TextDiff};

pub type Counts = HashMap<String, usize>;
type CountsDelta = HashMap<String, isize>;

/// Context characters needed on each side of a change to capture all affected n-grams
const CONTEXT_CHARS: usize = CHR_F_CHAR_ORDER - 1;

#[derive(Default, Debug, Clone, Serialize)]
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

    pub fn accumulate(&mut self, other: &ClassificationMetrics) {
        self.true_positives += other.true_positives;
        self.false_positives += other.false_positives;
        self.false_negatives += other.false_negatives;
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
    /// Preserve whitespace as-is
    #[allow(unused)]
    Unchanged,

    /// Ignore all whitespace differences
    #[allow(unused)]
    Ignore,

    /// Collapse whitespace into single spaces
    Collapse,
}

const CHR_F_CHAR_ORDER: usize = 6;
const CHR_F_BETA: f64 = 0.5;
const CHR_F_WHITESPACE: ChrfWhitespace = ChrfWhitespace::Collapse;

pub fn delta_chr_f_beta() -> f64 {
    CHR_F_BETA
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeltaChrFMetrics {
    pub score: f64,
    pub beta: f64,
    pub counts: ClassificationMetrics,
    pub precision: f64,
    pub recall: f64,
}

/// Computes delta-chrF metrics that compare two sets of edits.
///
/// This metric works by:
/// 1. Computing n-gram count differences (deltas) between original→expected and original→actual
/// 2. Comparing these deltas to measure how well actual edits match expected edits
///
/// Returns a score from 0.0 to 100.0, where 100.0 means the actual edits perfectly match
/// the expected edits.
pub fn delta_chr_f(original: &str, expected: &str, actual: &str) -> DeltaChrFMetrics {
    if original == expected && expected == actual {
        return DeltaChrFMetrics {
            score: 100.0,
            beta: CHR_F_BETA,
            precision: 1.0,
            recall: 1.0,
            ..DeltaChrFMetrics::default()
        };
    }

    let orig_chars: Vec<char> = filter_whitespace_chars(original);
    let exp_chars: Vec<char> = filter_whitespace_chars(expected);
    let act_chars: Vec<char> = filter_whitespace_chars(actual);

    // Find the changed regions between original→expected and original→actual
    // We only need to compute n-grams on these regions (plus context for boundary n-grams)
    let (orig_for_exp, exp_region) = extract_changed_regions(&orig_chars, &exp_chars);
    let (orig_for_act, act_region) = extract_changed_regions(&orig_chars, &act_chars);

    let mut total_precision = 0.0;
    let mut total_recall = 0.0;
    let mut total_counts = ClassificationMetrics::default();

    for order in 1..=CHR_F_CHAR_ORDER {
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

        let counts = ClassificationMetrics::from_counts(&expected_counts, &actual_counts);
        total_precision += counts.precision();
        total_recall += counts.recall();
        total_counts.accumulate(&counts);
    }

    let average_precision = total_precision / CHR_F_CHAR_ORDER as f64;
    let average_recall = total_recall / CHR_F_CHAR_ORDER as f64;
    let score = if average_precision + average_recall == 0.0 {
        0.0
    } else {
        (1.0 + CHR_F_BETA * CHR_F_BETA) * average_precision * average_recall
            / (CHR_F_BETA * CHR_F_BETA * average_precision + average_recall)
            * 100.0
    };

    DeltaChrFMetrics {
        score,
        beta: CHR_F_BETA,
        counts: total_counts,
        precision: average_precision,
        recall: average_recall,
    }
}

/// Reference implementation of delta-chrF metrics (original, non-optimized version).
/// Used for testing that the optimized version produces identical results.
#[cfg(test)]
fn delta_chr_f_reference(original: &str, expected: &str, actual: &str) -> DeltaChrFMetrics {
    if original == expected && expected == actual {
        return DeltaChrFMetrics {
            score: 100.0,
            beta: CHR_F_BETA,
            precision: 1.0,
            recall: 1.0,
            ..DeltaChrFMetrics::default()
        };
    }

    let original_ngrams = chr_f_ngram_counts(original);
    let expected_ngrams = chr_f_ngram_counts(expected);
    let actual_ngrams = chr_f_ngram_counts(actual);

    let mut total_precision = 0.0;
    let mut total_recall = 0.0;
    let mut total_counts = ClassificationMetrics::default();

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

        let counts = ClassificationMetrics::from_counts(&expected_counts, &actual_counts);
        total_precision += counts.precision();
        total_recall += counts.recall();
        total_counts.accumulate(&counts);
    }

    let average_precision = total_precision / CHR_F_CHAR_ORDER as f64;
    let average_recall = total_recall / CHR_F_CHAR_ORDER as f64;
    let score = if average_precision + average_recall == 0.0 {
        0.0
    } else {
        (1.0 + CHR_F_BETA * CHR_F_BETA) * average_precision * average_recall
            / (CHR_F_BETA * CHR_F_BETA * average_precision + average_recall)
            * 100.0
    };

    DeltaChrFMetrics {
        score,
        beta: CHR_F_BETA,
        counts: total_counts,
        precision: average_precision,
        recall: average_recall,
    }
}

/// Filter whitespace from a string and return as Vec<char>
fn filter_whitespace_chars(text: &str) -> Vec<char> {
    match CHR_F_WHITESPACE {
        ChrfWhitespace::Unchanged => text.chars().collect(),
        ChrfWhitespace::Ignore => text.chars().filter(|c| !c.is_whitespace()).collect(),
        ChrfWhitespace::Collapse => collapse_whitespace(text.chars()),
    }
}

/// Collapse whitespace into single spaces.
/// Newlines and spaces are collapsed separately.
fn collapse_whitespace(chars: impl Iterator<Item = char>) -> Vec<char> {
    let mut result = Vec::new();
    let mut last_whitespace = None;
    for c in chars {
        if c.is_whitespace() && c != '\n' {
            if last_whitespace != Some(' ') {
                result.push(' ');
                last_whitespace = Some(' ');
            }
        } else if c == '\n' {
            if last_whitespace != Some('\n') {
                result.push(c);
                last_whitespace = Some('\n');
            }
        } else {
            result.push(c);
            last_whitespace = None;
        }
    }
    result
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
    let text = match CHR_F_WHITESPACE {
        ChrfWhitespace::Unchanged => text.to_string(),
        ChrfWhitespace::Ignore => text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        ChrfWhitespace::Collapse => collapse_whitespace(text.chars())
            .into_iter()
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

/// Returns whether the patch contains any isolated whitespace-only changes.
///
/// A whitespace-only change is an added or deleted line whose content is empty or
/// contains only whitespace. It is "isolated" when it is not adjacent to any
/// substantive (non-whitespace) change within the same contiguous change group.
pub fn has_isolated_whitespace_changes(patch_str: &str, cursor_row: Option<u32>) -> bool {
    let patch = Patch::parse_unified_diff(patch_str);

    let cursor_new_file_line = cursor_row.map(|row| (row + 1) as usize);

    for hunk in &patch.hunks {
        let lines = &hunk.lines;
        let mut new_text_line = hunk.new_start as usize;

        for (i, line) in lines.iter().enumerate() {
            let content = match line {
                PatchLine::Addition(s) => {
                    let addition_line = new_text_line;
                    new_text_line += 1;
                    if s.trim().is_empty() && cursor_new_file_line == Some(addition_line) {
                        continue;
                    }
                    s.as_str()
                }
                PatchLine::Deletion(s) => s.as_str(),
                PatchLine::Context(_) => {
                    new_text_line += 1;
                    continue;
                }
                _ => continue,
            };

            if !content.trim().is_empty() {
                continue;
            }

            if is_whitespace_change_isolated(lines, i) {
                return true;
            }
        }
    }

    false
}

fn is_whitespace_change_isolated(lines: &[PatchLine], index: usize) -> bool {
    // Look backward for a non-whitespace change before hitting a context line
    for line in lines[..index].iter().rev() {
        match line {
            PatchLine::Addition(s) | PatchLine::Deletion(s) => {
                if !s.trim().is_empty() {
                    return false;
                }
            }
            _ => break,
        }
    }

    // Look forward for a non-whitespace change before hitting a context line
    for line in &lines[index + 1..] {
        match line {
            PatchLine::Addition(s) | PatchLine::Deletion(s) => {
                if !s.trim().is_empty() {
                    return false;
                }
            }
            _ => break,
        }
    }

    true
}

/// A simple proxy for whether the prediction respects editable region.
pub fn is_editable_region_correct(actual_patch: &str) -> bool {
    // A typical sign of a wrong editable region: a bunch of lines deletion
    // at the beginning or end of the patch.
    let patch = Patch::parse_unified_diff(actual_patch);
    if patch.hunks.is_empty() {
        return true;
    }

    let hunk = &patch.hunks[0];
    let mut deletions_at_start = 0;

    for line in hunk.lines.iter() {
        match line {
            PatchLine::Deletion(_) => deletions_at_start += 1,
            _ => break,
        }
    }

    if deletions_at_start >= 3 {
        return false;
    }

    true
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct TokenChangeCounts {
    pub inserted_tokens: usize,
    pub deleted_tokens: usize,
}

/// Counts the number of inserted and deleted tokens in a unified diff patch.
///
/// Tokens are words and whitespace sequences (as defined by `word_diff::tokenize`).
/// Within each hunk, the old (`-`) and new (`+`) lines are compared at the token level
/// using an LCS-based diff, so modified lines only count the actually changed tokens
/// rather than the entire line.
pub fn count_patch_token_changes(patch: &str) -> TokenChangeCounts {
    let mut counts = TokenChangeCounts::default();
    let mut old_lines: Vec<&str> = Vec::new();
    let mut new_lines: Vec<&str> = Vec::new();

    let flush =
        |old_lines: &mut Vec<&str>, new_lines: &mut Vec<&str>, counts: &mut TokenChangeCounts| {
            if old_lines.is_empty() && new_lines.is_empty() {
                return;
            }

            let old_text: String = old_lines
                .iter()
                .map(|line| if line.len() > 1 { &line[1..] } else { "" })
                .collect::<Vec<_>>()
                .join("\n");

            let new_text: String = new_lines
                .iter()
                .map(|line| if line.len() > 1 { &line[1..] } else { "" })
                .collect::<Vec<_>>()
                .join("\n");

            let old_tokens = tokenize(&old_text);
            let new_tokens = tokenize(&new_text);
            let ops = diff_tokens(&old_tokens, &new_tokens);

            for op in ops {
                match op {
                    DiffOp::Equal(..) => {}
                    DiffOp::Delete(start, end) => {
                        counts.deleted_tokens += end - start;
                    }
                    DiffOp::Insert(start, end) => {
                        counts.inserted_tokens += end - start;
                    }
                    DiffOp::Replace {
                        old_start,
                        old_end,
                        new_start,
                        new_end,
                    } => {
                        counts.deleted_tokens += old_end - old_start;
                        counts.inserted_tokens += new_end - new_start;
                    }
                }
            }

            old_lines.clear();
            new_lines.clear();
        };

    for line in patch.lines() {
        if line.starts_with("---")
            || line.starts_with("+++")
            || line.starts_with("@@")
            || line.starts_with("diff ")
            || line.starts_with("index ")
        {
            flush(&mut old_lines, &mut new_lines, &mut counts);
        } else if line.starts_with('-') {
            old_lines.push(line);
        } else if line.starts_with('+') {
            new_lines.push(line);
        } else {
            flush(&mut old_lines, &mut new_lines, &mut counts);
        }
    }

    flush(&mut old_lines, &mut new_lines, &mut counts);
    counts
}

#[allow(dead_code)]
#[derive(Debug)]
enum DiffOp {
    Equal(usize, usize),
    Delete(usize, usize),
    Insert(usize, usize),
    Replace {
        old_start: usize,
        old_end: usize,
        new_start: usize,
        new_end: usize,
    },
}

fn diff_tokens<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<DiffOp> {
    let diff = TextDiff::from_slices(old, new);
    diff.ops()
        .iter()
        .map(|op| {
            let tag = op.tag();
            let old_range = op.old_range();
            let new_range = op.new_range();
            match tag {
                DiffTag::Equal => DiffOp::Equal(old_range.start, old_range.end),
                DiffTag::Delete => DiffOp::Delete(old_range.start, old_range.end),
                DiffTag::Insert => DiffOp::Insert(new_range.start, new_range.end),
                DiffTag::Replace => DiffOp::Replace {
                    old_start: old_range.start,
                    old_end: old_range.end,
                    new_start: new_range.start,
                    new_end: new_range.end,
                },
            }
        })
        .collect()
}

/// Reconstruct old and new text from a unified diff.
///
/// Context and deletion lines form the old text; context and addition
/// lines form the new text. Returns `(old_text, new_text)`.
pub fn reconstruct_texts_from_diff(patch_str: &str) -> (String, String) {
    let patch = Patch::parse_unified_diff(patch_str);
    let mut old_lines: Vec<&str> = Vec::new();
    let mut new_lines: Vec<&str> = Vec::new();

    for hunk in &patch.hunks {
        for line in &hunk.lines {
            match line {
                PatchLine::Context(content) => {
                    old_lines.push(content);
                    new_lines.push(content);
                }
                PatchLine::Deletion(content) => {
                    old_lines.push(content);
                }
                PatchLine::Addition(content) => {
                    new_lines.push(content);
                }
                PatchLine::Garbage(_) => {}
            }
        }
    }

    (old_lines.join("\n"), new_lines.join("\n"))
}
#[derive(Debug, Default, Clone)]
struct Patch {
    hunks: Vec<Hunk>,
}

impl Patch {
    fn parse_unified_diff(unified_diff: &str) -> Patch {
        let mut current_file = String::new();
        let mut is_filename_inherited = false;
        let mut hunk = Hunk::default();
        let mut patch = Patch::default();
        let mut in_header = true;

        for line in unified_diff.lines() {
            if line.starts_with("--- ") || line.starts_with("+++") || line.starts_with("@@") {
                in_header = false;
            }

            if in_header {
                continue;
            }

            if line.starts_with("@@") {
                if !hunk.lines.is_empty() {
                    patch.hunks.push(hunk);
                }
                hunk = Hunk::from_header(line, &current_file, is_filename_inherited);
                is_filename_inherited = true;
            } else if let Some(path) = line.strip_prefix("--- ") {
                is_filename_inherited = false;
                let path = path.trim().strip_prefix("a/").unwrap_or(path);
                if path != "/dev/null" {
                    current_file = path.into();
                }
            } else if let Some(path) = line.strip_prefix("+++ ") {
                is_filename_inherited = false;
                let path = path.trim().strip_prefix("b/").unwrap_or(path);
                if path != "/dev/null" {
                    current_file = path.into();
                }
            } else if let Some(line) = line.strip_prefix('+') {
                hunk.lines.push(PatchLine::Addition(line.to_string()));
            } else if let Some(line) = line.strip_prefix('-') {
                hunk.lines.push(PatchLine::Deletion(line.to_string()));
            } else if let Some(line) = line.strip_prefix(' ') {
                hunk.lines.push(PatchLine::Context(line.to_string()));
            } else {
                hunk.lines.push(PatchLine::Garbage(line.to_string()));
            }
        }

        if !hunk.lines.is_empty() {
            patch.hunks.push(hunk);
        }

        patch
    }
}

#[derive(Debug, Default, Clone)]
struct Hunk {
    new_start: isize,
    lines: Vec<PatchLine>,
}

impl Hunk {
    fn from_header(header: &str, _filename: &str, _is_filename_inherited: bool) -> Self {
        let (_, _, new_start, _, _) = Self::parse_hunk_header(header);
        Self {
            new_start,
            lines: Vec::new(),
        }
    }

    fn parse_hunk_header(line: &str) -> (isize, isize, isize, isize, String) {
        let header_part = line.trim_start_matches("@@").trim();
        let parts: Vec<&str> = header_part.split_whitespace().collect();

        if parts.len() < 2 {
            return (0, 0, 0, 0, String::new());
        }

        let old_part = parts[0].trim_start_matches('-');
        let new_part = parts[1].trim_start_matches('+');

        let (old_start, old_count) = Hunk::parse_hunk_header_range(old_part);
        let (new_start, new_count) = Hunk::parse_hunk_header_range(new_part);

        let comment = if parts.len() > 2 {
            parts[2..]
                .join(" ")
                .trim_start_matches("@@")
                .trim()
                .to_string()
        } else {
            String::new()
        };

        (
            old_start as isize,
            old_count as isize,
            new_start as isize,
            new_count as isize,
            comment,
        )
    }

    fn parse_hunk_header_range(part: &str) -> (usize, usize) {
        if let Some((start, count)) = part.split_once(',') {
            (start.parse().unwrap_or(0), count.parse().unwrap_or(0))
        } else {
            (part.parse().unwrap_or(0), 1)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PatchLine {
    Context(String),
    Addition(String),
    Deletion(String),
    Garbage(String),
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
            let score = delta_chr_f(original, expected, actual).score;
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
            let optimized_metrics = delta_chr_f(original, expected, actual);
            let reference_metrics = delta_chr_f_reference(original, expected, actual);

            assert!(
                (optimized_metrics.score - reference_metrics.score).abs() < 1e-10,
                "Score mismatch for ({:?}, {:?}, {:?}):\n  optimized: {}\n  reference: {}",
                original,
                expected,
                actual,
                optimized_metrics.score,
                reference_metrics.score
            );
            assert_eq!(
                optimized_metrics.counts.true_positives,
                reference_metrics.counts.true_positives
            );
            assert_eq!(
                optimized_metrics.counts.false_positives,
                reference_metrics.counts.false_positives
            );
            assert_eq!(
                optimized_metrics.counts.false_negatives,
                reference_metrics.counts.false_negatives
            );
            assert!((optimized_metrics.precision - reference_metrics.precision).abs() < 1e-10);
            assert!((optimized_metrics.recall - reference_metrics.recall).abs() < 1e-10);
        }
    }

    #[test]
    fn test_delta_chr_f_metrics_include_counts_and_rates() {
        let original = "one two three";
        let expected = "one three";
        let actual = "one two four";

        let metrics = delta_chr_f(original, expected, actual);

        assert!(metrics.score > 20.0 && metrics.score < 40.0);
        assert!(metrics.counts.true_positives > 0);
        assert!(metrics.counts.false_positives > 0);
        assert!(metrics.counts.false_negatives > 0);
        assert!(metrics.precision > 0.0 && metrics.precision < 1.0);
        assert!(metrics.recall > 0.0 && metrics.recall < 1.0);
        assert_eq!(metrics.beta, CHR_F_BETA);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    fn cursor_on_line(one_based_line: u32) -> u32 {
        one_based_line - 1
    }

    #[test]
    fn test_delta_chr_f_perfect_match() {
        let original = "fn main() {    println!(\"Hello\");}";
        let expected = "fn main() {    println!(\"Hello, World!\");}";

        let score = delta_chr_f(original, expected, expected).score;
        assert!((score - 100.0).abs() < 1e-2);
    }

    #[test]
    fn test_delta_chr_f_wrong_edit() {
        // When the edit is wrong
        let original = "one two three";
        let expected = "one three"; // deleted "two "
        let actual = "one two four"; // deleted "three", added "four"

        // Then the score should be low
        let score = delta_chr_f(original, expected, actual).score;
        assert!(score > 20.0 && score < 40.0);
    }

    #[test]
    fn test_delta_chr_f_partial_match() {
        let original = "let x = 42;";
        let expected = "let x = 100;";
        let actual = "let x = 99;";

        // We got the edit location right, but the replacement text is wrong.
        // Deleted ngrams will match, bringing the score somewhere in the middle.
        let score = delta_chr_f(original, expected, actual).score;
        assert!(score > 40.0 && score < 60.0);
    }

    #[test]
    fn test_delta_chr_f_missed_edit() {
        // When predictions makes no changes
        let original = "prefix old suffix";
        let expected = "prefix new suffix";
        let actual = "prefix old suffix"; // no change

        // Then the score should be low (all expected changes are false negatives)
        let score = delta_chr_f(original, expected, actual).score;
        assert!(score < 20.0);
    }

    #[test]
    fn test_delta_chr_f_extra_edit() {
        // When adding unexpected content
        let original = "helloworld";
        let expected = "helloworld"; // no change expected
        let actual = "helloextraworld"; // added "extra"

        // Then the score should be low (all actual changes are false positives)
        let score = delta_chr_f(original, expected, actual).score;
        assert!(score < 20.0);
    }

    #[test]
    fn test_delta_chr_f_no_changes() {
        let text = "unchanged text";
        let score = delta_chr_f(text, text, text).score;
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

    #[test]
    fn test_is_editable_region_correct() {
        let patch = indoc! {"
            @@ -1,1 +1,1 @@
            -context
            -removed
            -from the beginning of the file
            import sys
            +sys.exit(0)

            "};
        assert!(!is_editable_region_correct(patch));

        let patch = indoc! {"
            @@ -1,1 +1,1 @@
            "};
        assert!(is_editable_region_correct(patch));
    }

    #[test]
    fn test_isolated_whitespace_purely_whitespace_patch() {
        let patch = indoc! {"
            @@ -1,3 +1,4 @@
             fn main() {
            +
                 println!(\"hello\");
             }
        "};
        assert!(has_isolated_whitespace_changes(patch, None));
    }

    #[test]
    fn test_isolated_whitespace_adjacent_to_real_change() {
        let patch = indoc! {"
            @@ -1,3 +1,4 @@
             fn main() {
            +
            +    let x = 1;
                 println!(\"hello\");
             }
        "};
        assert!(!has_isolated_whitespace_changes(patch, None));
    }

    #[test]
    fn test_isolated_whitespace_no_whitespace_changes() {
        let patch = indoc! {"
            @@ -1,3 +1,3 @@
             fn main() {
            -    println!(\"hello\");
            +    println!(\"world\");
             }
        "};
        assert!(!has_isolated_whitespace_changes(patch, None));
    }

    #[test]
    fn test_isolated_whitespace_deletion() {
        let patch = indoc! {"
            @@ -1,4 +1,3 @@
             fn main() {
            -
                 println!(\"hello\");
             }
        "};
        assert!(has_isolated_whitespace_changes(patch, None));
    }

    #[test]
    fn test_isolated_whitespace_mixed_groups() {
        let patch = indoc! {"
            @@ -1,7 +1,8 @@
             fn main() {
            +
                 let x = 1;
            -    let y = 2;
            +    let y = 3;

            +
                 println!(\"hello\");
             }
        "};
        assert!(has_isolated_whitespace_changes(patch, None));
    }

    #[test]
    fn test_isolated_whitespace_empty_patch() {
        let patch = "";
        assert!(!has_isolated_whitespace_changes(patch, None));
    }

    #[test]
    fn test_isolated_whitespace_skipped_on_cursor_line() {
        // The addition of a blank line at new-file line 2 should be skipped
        // because the cursor is on that line.
        let patch = indoc! {"
            @@ -1,3 +1,4 @@
             fn main() {
            +
                 println!(\"hello\");
             }
        "};
        // New-file line 2 is the added blank line
        let cursor = cursor_on_line(2);
        assert!(!has_isolated_whitespace_changes(patch, Some(cursor)));
    }

    #[test]
    fn test_isolated_whitespace_not_skipped_when_cursor_on_different_line() {
        // The blank line is at new-file line 2, but the cursor is on line 1.
        let patch = indoc! {"
            @@ -1,3 +1,4 @@
             fn main() {
            +
                 println!(\"hello\");
             }
        "};
        let cursor = cursor_on_line(1);
        assert!(has_isolated_whitespace_changes(patch, Some(cursor)));
    }

    #[test]
    fn test_isolated_whitespace_deletion_not_skipped_by_cursor() {
        // Deletions don't have a new-file line, so cursor can't suppress them.
        let patch = indoc! {"
            @@ -1,4 +1,3 @@
             fn main() {
            -
                 println!(\"hello\");
             }
        "};
        let cursor = cursor_on_line(2);
        assert!(has_isolated_whitespace_changes(patch, Some(cursor)));
    }

    #[test]
    fn test_count_patch_token_changes_real_world_rename() {
        // Real-world patch that was reported as returning 0 tokens
        let patch = "--- a/sip_call\\README.md\n+++ b/sip_call\\README.md\n@@ -1,1 +1,1 @@\n-# \n+# SIP Call\n";
        let counts = count_patch_token_changes(patch);
        // "# " vs "# SIP Call" — the "SIP" and "Call" tokens (and a whitespace token) are inserted
        assert!(
            counts.inserted_tokens > 0,
            "expected inserted tokens > 0, got {}",
            counts.inserted_tokens
        );
        assert_eq!(counts.deleted_tokens, 0);
    }

    #[test]
    fn test_count_patch_token_changes_real_world_expansion() {
        // Real-world patch: single token expanded to multiple lines
        let patch = "--- a/task1/src/app/app.html\n+++ b/task1/src/app/app.html\n@@ -1,7 +1,9 @@\n <style>\n-  m\n+  main {\n+    \n+  }\n </style>\n \n <main>\n   \n </main>\n";
        let counts = count_patch_token_changes(patch);
        assert!(
            counts.inserted_tokens > 0,
            "expected inserted tokens > 0, got {}",
            counts.inserted_tokens
        );
        assert!(
            counts.deleted_tokens > 0,
            "expected deleted tokens > 0, got {}",
            counts.deleted_tokens
        );
    }

    #[test]
    fn test_count_patch_token_changes_simple_replacement() {
        let patch = indoc! {"
            @@ -1,3 +1,3 @@
             fn main() {
            -    println!(\"hello\");
            +    println!(\"world\");
             }
        "};
        let counts = count_patch_token_changes(patch);
        assert_eq!(counts.deleted_tokens, 1, "deleted: \"hello\"");
        assert_eq!(counts.inserted_tokens, 1, "inserted: \"world\"");
    }

    #[test]
    fn test_count_patch_token_changes_insertion_only() {
        let patch = indoc! {"
            @@ -1,2 +1,3 @@
             fn main() {
            +    println!(\"hello\");
             }
        "};
        let counts = count_patch_token_changes(patch);
        assert_eq!(counts.deleted_tokens, 0);
        assert!(counts.inserted_tokens > 0);
    }

    #[test]
    fn test_count_patch_token_changes_deletion_only() {
        let patch = indoc! {"
            @@ -1,3 +1,2 @@
             fn main() {
            -    println!(\"hello\");
             }
        "};
        let counts = count_patch_token_changes(patch);
        assert!(counts.deleted_tokens > 0);
        assert_eq!(counts.inserted_tokens, 0);
    }

    #[test]
    fn test_count_patch_token_changes_empty_patch() {
        let patch = "";
        let counts = count_patch_token_changes(patch);
        assert_eq!(counts.deleted_tokens, 0);
        assert_eq!(counts.inserted_tokens, 0);
    }

    #[test]
    fn test_count_patch_token_changes_multiple_hunks() {
        let patch = indoc! {"
            @@ -1,3 +1,3 @@
             fn main() {
            -    let x = 1;
            +    let x = 2;
             }
            @@ -10,3 +10,3 @@
             fn other() {
            -    let y = 3;
            +    let y = 4;
             }
        "};
        let counts = count_patch_token_changes(patch);
        assert_eq!(counts.deleted_tokens, 2, "deleted: \"1\" and \"3\"");
        assert_eq!(counts.inserted_tokens, 2, "inserted: \"2\" and \"4\"");
    }

    #[test]
    fn test_count_patch_token_changes_multiword_change() {
        let patch = indoc! {"
            @@ -1,1 +1,1 @@
            -hello world foo
            +hello bar baz
        "};
        let counts = count_patch_token_changes(patch);
        // "world" and "foo" deleted, "bar" and "baz" inserted
        // (whitespace tokens between them may also count)
        assert!(counts.deleted_tokens >= 2);
        assert!(counts.inserted_tokens >= 2);
    }

    #[test]
    fn test_whitespace_collapse() {
        let text = "abc   \n\n\n   123";
        let collapsed = collapse_whitespace(text.chars());
        assert_eq!(
            collapsed,
            vec!['a', 'b', 'c', ' ', '\n', ' ', '1', '2', '3']
        );
    }
}
