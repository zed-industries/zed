use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
};

use crate::patch::{Patch, PatchLine};
use crate::patch_metrics::ClassificationMetrics;

const LINE_RELEVANCE_WINDOW: u32 = 20;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Excerpt {
    pub path: String,
    pub row_range: Range<u32>,
    pub content: String,
}

/// Line- and file-level precision/recall/F1 over TP/FP/FN counts.
///
/// Shared shape for two metrics with the same structure: how much expected
/// edit context was retrieved (`EditableContextCoverage`) and how well
/// predicted edit locations match expected ones (`PatchLocationMatch`).
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LineFileClassification {
    pub lines_tp: usize,
    pub lines_fp: usize,
    pub lines_fn: usize,
    pub lines_precision: f64,
    pub lines_recall: f64,
    pub lines_f1: f64,

    pub files_tp: usize,
    pub files_fp: usize,
    pub files_fn: usize,
    pub files_precision: f64,
    pub files_recall: f64,
    pub files_f1: f64,
}

pub type EditableContextCoverage = LineFileClassification;
pub type PatchLocationMatch = LineFileClassification;

impl LineFileClassification {
    pub fn new(
        lines_tp: usize,
        lines_fp: usize,
        lines_fn: usize,
        files_tp: usize,
        files_fp: usize,
        files_fn: usize,
    ) -> Self {
        Self {
            lines_tp,
            lines_fp,
            lines_fn,
            lines_precision: precision(lines_tp, lines_fp, lines_fn),
            lines_recall: recall(lines_tp, lines_fp, lines_fn),
            lines_f1: f1(lines_tp, lines_fp, lines_fn),
            files_tp,
            files_fp,
            files_fn,
            files_precision: precision(files_tp, files_fp, files_fn),
            files_recall: recall(files_tp, files_fp, files_fn),
            files_f1: f1(files_tp, files_fp, files_fn),
        }
    }

    pub fn lines_counts(&self) -> ClassificationMetrics {
        ClassificationMetrics {
            true_positives: self.lines_tp,
            false_positives: self.lines_fp,
            false_negatives: self.lines_fn,
        }
    }

    pub fn files_counts(&self) -> ClassificationMetrics {
        ClassificationMetrics {
            true_positives: self.files_tp,
            false_positives: self.files_fp,
            false_negatives: self.files_fn,
        }
    }
}

/// Measures how well an actual patch's edited file/line locations match an expected patch.
pub fn patch_location_match(expected_patch: &str, actual_patch: &str) -> PatchLocationMatch {
    let expected_patch = Patch::parse_unified_diff(expected_patch);
    let actual_patch = Patch::parse_unified_diff(actual_patch);
    let (expected_files, expected_anchor_lines, _) = expected_context(&expected_patch);
    let (actual_files, mut actual_anchor_lines, _) = expected_context(&actual_patch);

    normalize_actual_paths(
        &expected_files,
        &mut actual_anchor_lines,
        actual_files.iter().cloned().collect(),
    );
    let actual_files = actual_anchor_lines.keys().cloned().collect();

    let (lines_tp, lines_fp, lines_fn) =
        match_anchor_rows(&expected_anchor_lines, &actual_anchor_lines);
    let (files_tp, files_fp, files_fn) = classify_values(&expected_files, &actual_files);

    PatchLocationMatch::new(lines_tp, lines_fp, lines_fn, files_tp, files_fp, files_fn)
}

/// Measures how much expected edit context was retrieved and how much unrelated context was retrieved.
pub fn editable_context_coverage(
    expected_patch: &str,
    context: &[Excerpt],
) -> EditableContextCoverage {
    let patch = Patch::parse_unified_diff(expected_patch);
    let (expected_files, expected_anchor_lines, relevant_lines) = expected_context(&patch);
    let (retrieved_files, retrieved_lines) = retrieved_context(context);

    let (lines_tp, lines_fp) = classify_retrieved_rows(&relevant_lines, &retrieved_lines);
    let (lines_fn, expected_anchor_line_count) =
        count_missing_rows(&expected_anchor_lines, &retrieved_lines);
    let (files_tp, files_fp, files_fn) = classify_values(&expected_files, &retrieved_files);

    let lines_precision = precision(lines_tp, lines_fp, lines_fn);
    let lines_recall = line_recall(expected_anchor_line_count, lines_fn);
    let lines_f1 = f1_from_precision_and_recall(lines_precision, lines_recall);
    let files_precision = precision(files_tp, files_fp, files_fn);
    let files_recall = recall(files_tp, files_fp, files_fn);
    let files_f1 = f1_from_precision_and_recall(files_precision, files_recall);

    EditableContextCoverage {
        lines_tp,
        lines_fp,
        lines_fn,
        lines_precision,
        lines_recall,
        lines_f1,
        files_tp,
        files_fp,
        files_fn,
        files_precision,
        files_recall,
        files_f1,
    }
}

fn expected_context(
    patch: &Patch,
) -> (
    BTreeSet<String>,
    BTreeMap<String, BTreeSet<u32>>,
    BTreeMap<String, BTreeSet<u32>>,
) {
    let mut expected_files = BTreeSet::new();
    let mut expected_anchor_lines = BTreeMap::new();
    let mut relevant_lines = BTreeMap::new();

    for hunk in &patch.hunks {
        if hunk
            .lines
            .iter()
            .any(|line| matches!(line, PatchLine::Addition(_) | PatchLine::Deletion(_)))
        {
            expected_files.insert(hunk.filename.clone());
        }

        let mut old_row = hunk.old_start.saturating_sub(1).max(0) as u32;
        let mut previous_context_row = None;
        let mut index = 0;

        while index < hunk.lines.len() {
            match &hunk.lines[index] {
                PatchLine::Context(_) => {
                    previous_context_row = Some(old_row);
                    old_row = old_row.saturating_add(1);
                    index += 1;
                }
                PatchLine::Addition(_) | PatchLine::Deletion(_) => {
                    let mut deletion_rows = Vec::new();
                    let mut has_addition = false;

                    while index < hunk.lines.len() {
                        match &hunk.lines[index] {
                            PatchLine::Addition(_) => {
                                has_addition = true;
                                index += 1;
                            }
                            PatchLine::Deletion(_) => {
                                deletion_rows.push(old_row);
                                old_row = old_row.saturating_add(1);
                                index += 1;
                            }
                            _ => break,
                        }
                    }

                    if deletion_rows.is_empty() {
                        if has_addition {
                            if let Some(row) = previous_context_row {
                                insert_anchor_row(
                                    &mut expected_anchor_lines,
                                    &mut relevant_lines,
                                    &hunk.filename,
                                    row,
                                );
                            }
                            if matches!(hunk.lines.get(index), Some(PatchLine::Context(_))) {
                                insert_anchor_row(
                                    &mut expected_anchor_lines,
                                    &mut relevant_lines,
                                    &hunk.filename,
                                    old_row,
                                );
                            }
                        }
                    } else {
                        for row in deletion_rows {
                            insert_anchor_row(
                                &mut expected_anchor_lines,
                                &mut relevant_lines,
                                &hunk.filename,
                                row,
                            );
                        }
                    }

                    previous_context_row = None;
                }
                PatchLine::Garbage(_) => {
                    index += 1;
                }
            }
        }
    }

    (expected_files, expected_anchor_lines, relevant_lines)
}

fn retrieved_context(context: &[Excerpt]) -> (BTreeSet<String>, BTreeMap<String, BTreeSet<u32>>) {
    let mut retrieved_files = BTreeSet::new();
    let mut retrieved_lines = BTreeMap::new();

    for excerpt in context {
        retrieved_files.insert(excerpt.path.clone());
        let rows = retrieved_lines
            .entry(excerpt.path.clone())
            .or_insert_with(BTreeSet::new);
        rows.extend(excerpt.row_range.clone());
    }

    (retrieved_files, retrieved_lines)
}

fn normalize_actual_paths(
    expected_files: &BTreeSet<String>,
    actual_anchor_lines: &mut BTreeMap<String, BTreeSet<u32>>,
    actual_files: BTreeSet<String>,
) {
    let mut normalized = BTreeMap::new();

    for actual_path in actual_files {
        let normalized_path = normalize_actual_path(expected_files, &actual_path);
        if let Some(rows) = actual_anchor_lines.remove(&actual_path) {
            normalized
                .entry(normalized_path)
                .or_insert_with(BTreeSet::new)
                .extend(rows);
        }
    }

    *actual_anchor_lines = normalized;
}

fn normalize_actual_path(expected_files: &BTreeSet<String>, actual_path: &str) -> String {
    if expected_files.contains(actual_path) {
        return actual_path.to_string();
    }

    if let Some(stripped_path) = strip_first_path_component(actual_path) {
        if expected_files.contains(stripped_path) {
            return stripped_path.to_string();
        }
    }

    actual_path.to_string()
}

fn strip_first_path_component(path: &str) -> Option<&str> {
    path.split_once('/')
        .map(|(_, rest)| rest)
        .filter(|rest| !rest.is_empty())
}

fn match_anchor_rows(
    expected: &BTreeMap<String, BTreeSet<u32>>,
    actual: &BTreeMap<String, BTreeSet<u32>>,
) -> (usize, usize, usize) {
    let mut true_positives = 0;
    let mut false_positives = 0;
    let mut false_negatives = 0;
    let paths = expected
        .keys()
        .chain(actual.keys())
        .cloned()
        .collect::<BTreeSet<_>>();

    for path in paths {
        let expected_rows = expected.get(&path).cloned().unwrap_or_default();
        let actual_rows = actual.get(&path).cloned().unwrap_or_default();
        let matched = match_rows_with_window(&expected_rows, &actual_rows);
        true_positives += matched;
        false_positives += actual_rows.len().saturating_sub(matched);
        false_negatives += expected_rows.len().saturating_sub(matched);
    }

    (true_positives, false_positives, false_negatives)
}

fn match_rows_with_window(expected: &BTreeSet<u32>, actual: &BTreeSet<u32>) -> usize {
    let mut unmatched_expected = expected.clone();
    let mut matched = 0;

    for actual_row in actual {
        let start = actual_row.saturating_sub(LINE_RELEVANCE_WINDOW);
        let end = actual_row.saturating_add(LINE_RELEVANCE_WINDOW);
        if let Some(expected_row) = unmatched_expected.range(start..=end).next().copied() {
            unmatched_expected.remove(&expected_row);
            matched += 1;
        }
    }

    matched
}

fn insert_anchor_row(
    anchor_lines_by_file: &mut BTreeMap<String, BTreeSet<u32>>,
    relevant_lines_by_file: &mut BTreeMap<String, BTreeSet<u32>>,
    path: &str,
    row: u32,
) {
    insert_row(anchor_lines_by_file, path, row);

    let start = row.saturating_sub(LINE_RELEVANCE_WINDOW);
    let end = row.saturating_add(LINE_RELEVANCE_WINDOW);
    for relevant_row in start..=end {
        insert_row(relevant_lines_by_file, path, relevant_row);
    }
}

fn insert_row(lines_by_file: &mut BTreeMap<String, BTreeSet<u32>>, path: &str, row: u32) {
    lines_by_file
        .entry(path.to_string())
        .or_insert_with(BTreeSet::new)
        .insert(row);
}

fn classify_retrieved_rows(
    relevant: &BTreeMap<String, BTreeSet<u32>>,
    retrieved: &BTreeMap<String, BTreeSet<u32>>,
) -> (usize, usize) {
    let mut true_positives = 0;
    let mut false_positives = 0;

    for (path, rows) in retrieved {
        for row in rows {
            if relevant
                .get(path)
                .is_some_and(|relevant_rows| relevant_rows.contains(row))
            {
                true_positives += 1;
            } else {
                false_positives += 1;
            }
        }
    }

    (true_positives, false_positives)
}

fn count_missing_rows(
    expected: &BTreeMap<String, BTreeSet<u32>>,
    retrieved: &BTreeMap<String, BTreeSet<u32>>,
) -> (usize, usize) {
    let mut false_negatives = 0;
    let mut expected_count = 0;

    for (path, rows) in expected {
        for row in rows {
            expected_count += 1;
            if !retrieved
                .get(path)
                .is_some_and(|retrieved_rows| retrieved_rows.contains(row))
            {
                false_negatives += 1;
            }
        }
    }

    (false_negatives, expected_count)
}

fn classify_values<T: Ord>(
    expected: &BTreeSet<T>,
    retrieved: &BTreeSet<T>,
) -> (usize, usize, usize) {
    let true_positives = expected.intersection(retrieved).count();
    let false_positives = retrieved.difference(expected).count();
    let false_negatives = expected.difference(retrieved).count();
    (true_positives, false_positives, false_negatives)
}

fn precision(true_positives: usize, false_positives: usize, false_negatives: usize) -> f64 {
    if true_positives + false_positives + false_negatives == 0 {
        return 1.0;
    }

    let denominator = true_positives + false_positives;
    if denominator == 0 {
        1.0
    } else {
        true_positives as f64 / denominator as f64
    }
}

fn recall(true_positives: usize, false_positives: usize, false_negatives: usize) -> f64 {
    if true_positives + false_positives + false_negatives == 0 {
        return 1.0;
    }

    let denominator = true_positives + false_negatives;
    if denominator == 0 {
        1.0
    } else {
        true_positives as f64 / denominator as f64
    }
}

fn line_recall(expected_anchor_line_count: usize, false_negatives: usize) -> f64 {
    if expected_anchor_line_count == 0 {
        1.0
    } else {
        (expected_anchor_line_count - false_negatives) as f64 / expected_anchor_line_count as f64
    }
}

fn f1(true_positives: usize, false_positives: usize, false_negatives: usize) -> f64 {
    let precision = precision(true_positives, false_positives, false_negatives);
    let recall = recall(true_positives, false_positives, false_negatives);

    f1_from_precision_and_recall(precision, recall)
}

fn f1_from_precision_and_recall(precision: f64, recall: f64) -> f64 {
    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn excerpt(path: &str, row_range: Range<u32>) -> Excerpt {
        Excerpt {
            path: path.to_string(),
            row_range,
            content: String::new(),
        }
    }

    #[test]
    fn deletion_counts_deleted_old_line_as_true_positive() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -2,1 +2,0 @@
            -let value = 1;
        "};

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 1..2)]);

        assert_eq!(score, EditableContextCoverage::new(1, 0, 0, 1, 0, 0));
    }

    #[test]
    fn retrieved_lines_inside_relevance_window_are_true_positives() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -4,1 +4,0 @@
            -let value = 4;
        "};

        let score = editable_context_coverage(
            patch,
            &[excerpt("src/main.rs", 0..1), excerpt("src/main.rs", 3..4)],
        );

        assert_eq!(score, EditableContextCoverage::new(2, 0, 0, 1, 0, 0));
    }

    #[test]
    fn replacement_counts_deleted_old_line_without_addition_anchor() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,3 +1,3 @@
             fn main() {
            -    let value = 1;
            +    let value = 2;
             }
        "};

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 1..2)]);

        assert_eq!(score, EditableContextCoverage::new(1, 0, 0, 1, 0, 0));
    }

    #[test]
    fn pure_insertion_counts_previous_and_next_old_lines_as_expected_context() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,2 +1,3 @@
             line 1
            +inserted
             line 2
        "};

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 0..1)]);

        assert_eq!(score, EditableContextCoverage::new(1, 0, 1, 1, 0, 0));
    }

    #[test]
    fn pure_insertion_at_file_boundary_uses_available_neighboring_context() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,1 +1,2 @@
            +inserted
             line 1
        "};

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 0..1)]);

        assert_eq!(score, EditableContextCoverage::new(1, 0, 0, 1, 0, 0));
    }

    #[test]
    fn counts_false_negatives_and_file_false_positives() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,3 +1,3 @@
            -let first = 1;
            +let first = 2;
             let middle = 3;
            -let last = 4;
            +let last = 5;
        "};

        let score = editable_context_coverage(
            patch,
            &[excerpt("src/main.rs", 0..1), excerpt("src/lib.rs", 0..1)],
        );

        assert_eq!(score, EditableContextCoverage::new(1, 1, 1, 1, 1, 0));
    }

    #[test]
    fn overlapping_excerpts_are_counted_once() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -2,1 +2,0 @@
            -let value = 1;
        "};

        let score = editable_context_coverage(
            patch,
            &[excerpt("src/main.rs", 0..2), excerpt("src/main.rs", 1..3)],
        );

        assert_eq!(score, EditableContextCoverage::new(3, 0, 0, 1, 0, 0));
    }

    #[test]
    fn nearby_lines_do_not_satisfy_line_recall_without_exact_anchor_lines() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,2 +1,3 @@
             line 1
            +inserted
             line 2
        "};

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 2..3)]);

        assert_eq!(score.lines_tp, 1);
        assert_eq!(score.lines_fp, 0);
        assert_eq!(score.lines_fn, 2);
        assert_eq!(score.lines_precision, 1.0);
        assert_eq!(score.lines_recall, 0.0);
        assert_eq!(score.lines_f1, 0.0);
    }

    #[test]
    fn retrieved_lines_outside_relevance_window_are_false_positives() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,1 +1,0 @@
            -line 1
        "};

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 21..22)]);

        assert_eq!(score, EditableContextCoverage::new(0, 1, 1, 1, 0, 0));
    }

    #[test]
    fn empty_patch_with_no_context_has_perfect_f1() {
        let score = editable_context_coverage(
            indoc! {"
            "},
            &[],
        );

        assert_eq!(score, EditableContextCoverage::new(0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn patch_location_match_counts_file_and_nearby_line_matches() {
        let expected = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -50,1 +50,1 @@
            -let value = 1;
            +let value = 2;
        "};
        let actual = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -55,1 +55,1 @@
            -let value = 1;
            +let value = 2;
        "};

        let score = patch_location_match(expected, actual);

        assert_eq!(score, PatchLocationMatch::new(1, 0, 0, 1, 0, 0));
    }

    #[test]
    fn patch_location_match_counts_missing_and_extra_files() {
        let expected = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,1 +1,1 @@
            -let value = 1;
            +let value = 2;
        "};
        let actual = indoc! {"
            --- a/src/lib.rs
            +++ b/src/lib.rs
            @@ -1,1 +1,1 @@
            -let value = 1;
            +let value = 2;
        "};

        let score = patch_location_match(expected, actual);

        assert_eq!(score, PatchLocationMatch::new(0, 1, 1, 0, 1, 1));
    }

    #[test]
    fn patch_location_match_normalizes_project_prefixed_actual_path() {
        let expected = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,1 +1,1 @@
            -let value = 1;
            +let value = 2;
        "};
        let actual = indoc! {"
            --- a/project/src/main.rs
            +++ b/project/src/main.rs
            @@ -1,1 +1,1 @@
            -let value = 1;
            +let value = 2;
        "};

        let score = patch_location_match(expected, actual);

        assert_eq!(score, PatchLocationMatch::new(1, 0, 0, 1, 0, 0));
    }
}
