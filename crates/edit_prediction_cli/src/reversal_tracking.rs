use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

use edit_prediction::udiff::apply_diff_to_string;
use language::text_diff;

use crate::example::ExamplePromptInputs;

fn apply_diff_to_string_lenient(diff_str: &str, text: &str) -> String {
    let hunks = parse_diff_hunks(diff_str);
    let mut result = text.to_string();

    for hunk in hunks {
        let hunk_diff = format!("--- a/file\n+++ b/file\n{}", format_hunk(&hunk));
        if let Ok(updated) = apply_diff_to_string(&hunk_diff, &result) {
            result = updated;
        }
    }

    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedHunk {
    old_start: u32,
    old_count: u32,
    new_start: u32,
    new_count: u32,
    lines: Vec<HunkLine>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HunkLine {
    Context(String),
    Addition(String),
    Deletion(String),
}

fn parse_hunk_header(line: &str) -> Option<(u32, u32, u32, u32)> {
    let line = line.strip_prefix("@@ -")?;
    let (old_part, rest) = line.split_once(' ')?;
    let rest = rest.strip_prefix('+')?;
    let (new_part, _) = rest.split_once(" @@")?;

    let (old_start, old_count) = if let Some((start, count)) = old_part.split_once(',') {
        (start.parse().ok()?, count.parse().ok()?)
    } else {
        (old_part.parse().ok()?, 1)
    };

    let (new_start, new_count) = if let Some((start, count)) = new_part.split_once(',') {
        (start.parse().ok()?, count.parse().ok()?)
    } else {
        (new_part.parse().ok()?, 1)
    };

    Some((old_start, old_count, new_start, new_count))
}

fn parse_diff_hunks(diff: &str) -> Vec<ParsedHunk> {
    let mut hunks = Vec::new();
    let mut current_hunk: Option<ParsedHunk> = None;

    for line in diff.lines() {
        if let Some((old_start, old_count, new_start, new_count)) = parse_hunk_header(line) {
            if let Some(hunk) = current_hunk.take() {
                hunks.push(hunk);
            }
            current_hunk = Some(ParsedHunk {
                old_start,
                old_count,
                new_start,
                new_count,
                lines: Vec::new(),
            });
        } else if let Some(ref mut hunk) = current_hunk {
            if let Some(stripped) = line.strip_prefix('+') {
                hunk.lines.push(HunkLine::Addition(stripped.to_string()));
            } else if let Some(stripped) = line.strip_prefix('-') {
                hunk.lines.push(HunkLine::Deletion(stripped.to_string()));
            } else if let Some(stripped) = line.strip_prefix(' ') {
                hunk.lines.push(HunkLine::Context(stripped.to_string()));
            } else if line.is_empty() {
                hunk.lines.push(HunkLine::Context(String::new()));
            }
        }
    }

    if let Some(hunk) = current_hunk {
        hunks.push(hunk);
    }

    hunks
}

fn format_hunk(hunk: &ParsedHunk) -> String {
    let mut result = format!(
        "@@ -{},{} +{},{} @@\n",
        hunk.old_start, hunk.old_count, hunk.new_start, hunk.new_count
    );
    for line in &hunk.lines {
        match line {
            HunkLine::Context(text) => {
                result.push(' ');
                result.push_str(text);
                result.push('\n');
            }
            HunkLine::Addition(text) => {
                result.push('+');
                result.push_str(text);
                result.push('\n');
            }
            HunkLine::Deletion(text) => {
                result.push('-');
                result.push_str(text);
                result.push('\n');
            }
        }
    }
    result
}

fn filter_diff_hunks_by_excerpt(
    diff: &str,
    excerpt_start_row: u32,
    excerpt_row_count: u32,
) -> (String, i32) {
    let hunks = parse_diff_hunks(diff);
    let excerpt_start_0based = excerpt_start_row;
    let excerpt_end_0based = excerpt_start_row + excerpt_row_count;

    let mut filtered_hunks = Vec::new();
    let mut cumulative_line_offset: i32 = 0;

    for hunk in hunks {
        let hunk_start_0based = hunk.new_start.saturating_sub(1);
        let hunk_end_0based = hunk_start_0based + hunk.new_count;

        let additions: i32 = hunk
            .lines
            .iter()
            .filter(|l| matches!(l, HunkLine::Addition(_)))
            .count() as i32;
        let deletions: i32 = hunk
            .lines
            .iter()
            .filter(|l| matches!(l, HunkLine::Deletion(_)))
            .count() as i32;
        let hunk_line_delta = additions - deletions;

        if hunk_end_0based <= excerpt_start_0based {
            cumulative_line_offset += hunk_line_delta;
            continue;
        }

        if hunk_start_0based >= excerpt_end_0based {
            continue;
        }

        let mut filtered_lines = Vec::new();
        let mut current_row_0based = hunk_start_0based;
        let mut filtered_old_count = 0u32;
        let mut filtered_new_count = 0u32;
        let mut first_included_row: Option<u32> = None;

        for line in &hunk.lines {
            match line {
                HunkLine::Context(text) => {
                    if current_row_0based >= excerpt_start_0based
                        && current_row_0based < excerpt_end_0based
                    {
                        if first_included_row.is_none() {
                            first_included_row = Some(current_row_0based);
                        }
                        filtered_lines.push(HunkLine::Context(text.clone()));
                        filtered_old_count += 1;
                        filtered_new_count += 1;
                    }
                    current_row_0based += 1;
                }
                HunkLine::Addition(text) => {
                    if current_row_0based >= excerpt_start_0based
                        && current_row_0based < excerpt_end_0based
                    {
                        if first_included_row.is_none() {
                            first_included_row = Some(current_row_0based);
                        }
                        filtered_lines.push(HunkLine::Addition(text.clone()));
                        filtered_new_count += 1;
                    }
                    current_row_0based += 1;
                }
                HunkLine::Deletion(text) => {
                    if current_row_0based >= excerpt_start_0based
                        && current_row_0based < excerpt_end_0based
                    {
                        if first_included_row.is_none() {
                            first_included_row = Some(current_row_0based);
                        }
                        filtered_lines.push(HunkLine::Deletion(text.clone()));
                        filtered_old_count += 1;
                    }
                }
            }
        }

        if !filtered_lines.is_empty() {
            let first_row = first_included_row.unwrap_or(excerpt_start_0based);
            let new_start_1based = (first_row - excerpt_start_0based) + 1;

            filtered_hunks.push(ParsedHunk {
                old_start: new_start_1based,
                old_count: filtered_old_count,
                new_start: new_start_1based,
                new_count: filtered_new_count,
                lines: filtered_lines,
            });
        }

        cumulative_line_offset += hunk_line_delta;
    }

    let mut result = String::new();
    for hunk in &filtered_hunks {
        result.push_str(&format_hunk(hunk));
    }

    (result, cumulative_line_offset)
}

fn compute_excerpt_aware_reversal_overlap(
    edit_history_diffs: &[&str],
    excerpt_content: &str,
    excerpt_start_row: u32,
    predicted_content: &str,
) -> ReversalOverlap {
    let mut current_content = excerpt_content.to_string();
    let mut current_excerpt_start_row = excerpt_start_row;

    for diff in edit_history_diffs.iter().rev() {
        if diff.is_empty() {
            continue;
        }

        let current_row_count = current_content.lines().count() as u32;
        let (filtered_diff, _line_offset) =
            filter_diff_hunks_by_excerpt(diff, current_excerpt_start_row, current_row_count.max(1));

        if filtered_diff.is_empty() {
            let hunks = parse_diff_hunks(diff);
            for hunk in hunks {
                let hunk_end = hunk.new_start.saturating_sub(1) + hunk.new_count;
                if hunk_end <= current_excerpt_start_row {
                    let additions: u32 = hunk
                        .lines
                        .iter()
                        .filter(|l| matches!(l, HunkLine::Addition(_)))
                        .count() as u32;
                    let deletions: u32 = hunk
                        .lines
                        .iter()
                        .filter(|l| matches!(l, HunkLine::Deletion(_)))
                        .count() as u32;
                    if additions >= deletions {
                        current_excerpt_start_row =
                            current_excerpt_start_row.saturating_sub(additions - deletions);
                    } else {
                        current_excerpt_start_row += deletions - additions;
                    }
                }
            }
            continue;
        }

        let reversed = reverse_diff(&format!("--- a/file\n+++ b/file\n{}", filtered_diff));
        match apply_diff_to_string(&reversed, &current_content) {
            Ok(updated) => {
                current_content = updated;
            }
            Err(_) => {
                continue;
            }
        }

        let hunks = parse_diff_hunks(diff);
        for hunk in hunks {
            let hunk_end = hunk.new_start.saturating_sub(1) + hunk.new_count;
            if hunk_end <= current_excerpt_start_row {
                let additions: u32 = hunk
                    .lines
                    .iter()
                    .filter(|l| matches!(l, HunkLine::Addition(_)))
                    .count() as u32;
                let deletions: u32 = hunk
                    .lines
                    .iter()
                    .filter(|l| matches!(l, HunkLine::Deletion(_)))
                    .count() as u32;
                if additions >= deletions {
                    current_excerpt_start_row =
                        current_excerpt_start_row.saturating_sub(additions - deletions);
                } else {
                    current_excerpt_start_row += deletions - additions;
                }
            }
        }
    }

    compute_reversal_overlap(&current_content, excerpt_content, predicted_content)
}

fn reverse_diff(diff: &str) -> String {
    let mut result: String = diff
        .lines()
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
        .join("\n");
    if diff.ends_with('\n') {
        result.push('\n');
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GranularEdit {
    range: Range<usize>,
    old_text: String,
    new_text: String,
}

fn compute_granular_edits(old_text: &str, new_text: &str) -> Vec<GranularEdit> {
    text_diff(old_text, new_text)
        .into_iter()
        .map(|(range, new_text)| GranularEdit {
            old_text: old_text[range.clone()].to_string(),
            range,
            new_text: new_text.to_string(),
        })
        .collect()
}

#[derive(Debug, Clone)]
struct HistoryAdditionRange {
    range_in_current: Range<usize>,
}

#[derive(Debug, Clone)]
struct HistoryDeletionRange {
    deleted_text: String,
    position_in_current: usize,
}

fn compute_history_addition_ranges(history_edits: &[GranularEdit]) -> Vec<HistoryAdditionRange> {
    let mut result = Vec::new();
    let mut offset_delta: isize = 0;

    for edit in history_edits {
        if !edit.new_text.is_empty() {
            let new_start = (edit.range.start as isize + offset_delta) as usize;
            let new_end = new_start + edit.new_text.len();
            result.push(HistoryAdditionRange {
                range_in_current: new_start..new_end,
            });
        }

        offset_delta += edit.new_text.len() as isize - edit.old_text.len() as isize;
    }

    result
}

fn compute_history_deletion_ranges(history_edits: &[GranularEdit]) -> Vec<HistoryDeletionRange> {
    let mut result = Vec::new();
    let mut offset_delta: isize = 0;

    for edit in history_edits {
        if !edit.old_text.is_empty() {
            let position_in_current = (edit.range.start as isize + offset_delta) as usize;
            result.push(HistoryDeletionRange {
                deleted_text: edit.old_text.clone(),
                position_in_current,
            });
        }

        offset_delta += edit.new_text.len() as isize - edit.old_text.len() as isize;
    }

    result
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ReversalOverlap {
    chars_reversing_user_edits: usize,
    total_chars_in_prediction: usize,
}

impl ReversalOverlap {
    fn ratio(&self) -> f32 {
        if self.total_chars_in_prediction == 0 {
            0.0
        } else {
            self.chars_reversing_user_edits as f32 / self.total_chars_in_prediction as f32
        }
    }
}

/// Check if `needle` is a subsequence of `haystack` (characters appear in order, not necessarily contiguous).
fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut needle_chars = needle.chars().peekable();
    for c in haystack.chars() {
        if needle_chars.peek() == Some(&c) {
            needle_chars.next();
        }
    }
    needle_chars.peek().is_none()
}

/// Normalize edits where `old_text` appears as a subsequence within `new_text` (extension),
/// or where `new_text` appears as a subsequence within `old_text` (reduction).
///
/// For extensions: when the user's text is preserved (in order) within the prediction,
/// we only count the newly inserted characters, not the preserved ones.
/// E.g., "epr" → "eprintln!()" becomes 8 inserted chars ("intln!()")
/// E.g., "test_my_function" → "a_test_for_my_special_function_plz" becomes 18 inserted chars
///
/// For reductions: when the prediction's text is preserved (in order) within the original,
/// we only count the deleted characters, not the preserved ones.
/// E.g., "ifrom" → "from" becomes 1 deleted char ("i")
fn normalize_extension_edits(edits: Vec<GranularEdit>) -> Vec<GranularEdit> {
    edits
        .into_iter()
        .map(|edit| {
            if edit.old_text.is_empty() || edit.new_text.is_empty() {
                return edit;
            }

            if is_subsequence(&edit.old_text, &edit.new_text) {
                let inserted_char_count =
                    edit.new_text.chars().count() - edit.old_text.chars().count();
                GranularEdit {
                    range: edit.range.start..edit.range.start,
                    old_text: String::new(),
                    new_text: edit.new_text.chars().take(inserted_char_count).collect(),
                }
            } else if is_subsequence(&edit.new_text, &edit.old_text) {
                let deleted_char_count =
                    edit.old_text.chars().count() - edit.new_text.chars().count();
                let deleted_text: String = edit.old_text.chars().take(deleted_char_count).collect();
                GranularEdit {
                    range: edit.range.start..edit.range.start + deleted_text.len(),
                    old_text: deleted_text,
                    new_text: String::new(),
                }
            } else {
                edit
            }
        })
        .collect()
}

fn compute_reversal_overlap(
    original_content: &str,
    current_content: &str,
    predicted_content: &str,
) -> ReversalOverlap {
    let history_edits =
        normalize_extension_edits(compute_granular_edits(original_content, current_content));
    let prediction_edits =
        normalize_extension_edits(compute_granular_edits(current_content, predicted_content));

    let history_addition_ranges = compute_history_addition_ranges(&history_edits);
    let history_deletion_ranges = compute_history_deletion_ranges(&history_edits);

    let reversed_additions =
        compute_reversed_additions(&history_addition_ranges, &prediction_edits);
    let restored_deletions =
        compute_restored_deletions(&history_deletion_ranges, &prediction_edits);

    let total_chars_in_prediction: usize = prediction_edits
        .iter()
        .map(|e| e.new_text.chars().count() + e.old_text.chars().count())
        .sum();

    ReversalOverlap {
        chars_reversing_user_edits: reversed_additions + restored_deletions,
        total_chars_in_prediction,
    }
}

fn compute_reversed_additions(
    history_addition_ranges: &[HistoryAdditionRange],
    prediction_edits: &[GranularEdit],
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
                let relative_start = overlap_start - pred_edit.range.start;
                let relative_end = overlap_end - pred_edit.range.start;
                let overlap_text = &pred_edit.old_text[relative_start..relative_end];
                reversed_chars += overlap_text.chars().count();
            }
        }
    }

    reversed_chars
}

fn compute_restored_deletions(
    history_deletion_ranges: &[HistoryDeletionRange],
    prediction_edits: &[GranularEdit],
) -> usize {
    let mut restored = 0;

    for pred_edit in prediction_edits {
        if pred_edit.new_text.is_empty() {
            continue;
        }

        for deletion in history_deletion_ranges {
            if pred_edit.range.contains(&deletion.position_in_current)
                || deletion.position_in_current == pred_edit.range.start
            {
                restored += compute_lcs_length(&deletion.deleted_text, &pred_edit.new_text);
            }
        }
    }

    restored
}

fn compute_lcs_length(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 || n == 0 {
        return 0;
    }

    let mut prev = vec![0; n + 1];
    let mut curr = vec![0; n + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a_chars[i - 1] == b_chars[j - 1] {
                curr[j] = prev[j - 1] + 1;
            } else {
                curr[j] = prev[j].max(curr[j - 1]);
            }
        }
        std::mem::swap(&mut prev, &mut curr);
        curr.fill(0);
    }

    prev[n]
}

fn filter_edit_history_by_path<'a>(
    edit_history: &'a [Arc<zeta_prompt::Event>],
    cursor_path: &std::path::Path,
) -> Vec<&'a zeta_prompt::Event> {
    edit_history
        .iter()
        .filter(|event| match event.as_ref() {
            zeta_prompt::Event::BufferChange { path, .. } => {
                let event_path = path.as_ref();
                if event_path == cursor_path {
                    return true;
                }
                let stripped = event_path
                    .components()
                    .skip(1)
                    .collect::<std::path::PathBuf>();
                stripped == cursor_path
            }
        })
        .map(|arc| arc.as_ref())
        .collect()
}

fn extract_diff_from_event(event: &zeta_prompt::Event) -> &str {
    match event {
        zeta_prompt::Event::BufferChange { diff, .. } => diff.as_str(),
    }
}

pub fn compute_prediction_reversal_ratio(
    prompt_inputs: &ExamplePromptInputs,
    predicted_content: &str,
    cursor_path: &Path,
) -> f32 {
    let current_content = &prompt_inputs.content;

    let edit_history: &[Arc<zeta_prompt::Event>] = &prompt_inputs.edit_history;
    let relevant_events = filter_edit_history_by_path(edit_history, cursor_path);

    if let Some(excerpt_start_row) = prompt_inputs.excerpt_start_row {
        let diffs: Vec<&str> = relevant_events
            .iter()
            .map(|e| extract_diff_from_event(e))
            .collect();
        let overlap = compute_excerpt_aware_reversal_overlap(
            &diffs,
            current_content,
            excerpt_start_row,
            predicted_content,
        );
        return overlap.ratio();
    }

    let mut original_content = current_content.to_string();
    for event in relevant_events.into_iter().rev() {
        let diff = extract_diff_from_event(event);
        if diff.is_empty() {
            continue;
        }
        let reversed = reverse_diff(diff);
        let with_headers = format!("--- a/file\n+++ b/file\n{}", reversed);
        match apply_diff_to_string(&with_headers, &original_content) {
            Ok(updated_content) => original_content = updated_content,
            Err(_) => {
                original_content = apply_diff_to_string_lenient(&reversed, &original_content);
            }
        }
    }

    let overlap = compute_reversal_overlap(&original_content, current_content, predicted_content);
    overlap.ratio()
}

#[cfg(test)]
mod tests {
    use super::*;
    use edit_prediction::udiff::apply_diff_to_string;
    use indoc::indoc;

    #[test]
    fn test_reversal_overlap() {
        struct Case {
            name: &'static str,
            original: &'static str,
            current: &'static str,
            predicted: &'static str,
            expected_reversal_chars: usize,
            expected_total_chars: usize,
        }

        let cases = [
            Case {
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
            },
            Case {
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
            },
            Case {
                name: "user_deletes_text_prediction_restores_partial",
                original: "hello beautiful world",
                current: "hello world",
                predicted: "hello beautiful world",
                expected_reversal_chars: 10,
                expected_total_chars: 10,
            },
            Case {
                name: "user_deletes_foo_prediction_adds_bar",
                original: "foo",
                current: "",
                predicted: "bar",
                expected_reversal_chars: 0,
                expected_total_chars: 3,
            },
            Case {
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
            },
            Case {
                name: "no_history_edits",
                original: "same",
                current: "same",
                predicted: "different",
                expected_reversal_chars: 0,
                expected_total_chars: 13,
            },
            Case {
                name: "user_replaces_text_prediction_reverses",
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
            },
            Case {
                name: "user_modifies_word_prediction_modifies_differently",
                original: "the quick brown fox",
                current: "the slow brown fox",
                predicted: "the fast brown fox",
                expected_reversal_chars: 4,
                expected_total_chars: 8,
            },
            Case {
                name: "user finishes function name (suffix)",
                original: "",
                current: "epr",
                predicted: "eprintln!()",
                expected_reversal_chars: 0,
                expected_total_chars: 8,
            },
            Case {
                name: "user starts function name (prefix)",
                original: "",
                current: "my_function()",
                predicted: "test_my_function()",
                expected_reversal_chars: 0,
                expected_total_chars: 5,
            },
            Case {
                name: "user types partial, prediction extends in multiple places",
                original: "",
                current: "test_my_function",
                predicted: "a_test_for_my_special_function_plz",
                expected_reversal_chars: 0,
                expected_total_chars: 18,
            },
            // Edge cases for subsequence matching
            Case {
                name: "subsequence with interleaved underscores",
                original: "",
                current: "a_b_c",
                predicted: "_a__b__c__",
                expected_reversal_chars: 0,
                expected_total_chars: 5,
            },
            Case {
                name: "not a subsequence - different characters",
                original: "",
                current: "abc",
                predicted: "xyz",
                expected_reversal_chars: 3,
                expected_total_chars: 6,
            },
            Case {
                name: "not a subsequence - wrong order",
                original: "",
                current: "abc",
                predicted: "cba",
                expected_reversal_chars: 3,
                expected_total_chars: 6,
            },
            Case {
                name: "partial subsequence - only some chars match",
                original: "",
                current: "abcd",
                predicted: "axbx",
                expected_reversal_chars: 4,
                expected_total_chars: 8,
            },
            // Common completion patterns
            Case {
                name: "completing a method call",
                original: "",
                current: "vec.pu",
                predicted: "vec.push(item)",
                expected_reversal_chars: 0,
                expected_total_chars: 8,
            },
            Case {
                name: "completing an import statement",
                original: "",
                current: "use std::col",
                predicted: "use std::collections::HashMap",
                expected_reversal_chars: 0,
                expected_total_chars: 17,
            },
            Case {
                name: "completing a struct field",
                original: "",
                current: "name: St",
                predicted: "name: String",
                expected_reversal_chars: 0,
                expected_total_chars: 4,
            },
            Case {
                name: "prediction replaces with completely different text",
                original: "",
                current: "hello",
                predicted: "world",
                expected_reversal_chars: 5,
                expected_total_chars: 10,
            },
            Case {
                name: "empty prediction removes user text",
                original: "",
                current: "mistake",
                predicted: "",
                expected_reversal_chars: 7,
                expected_total_chars: 7,
            },
            Case {
                name: "fixing typo is not reversal",
                original: "",
                current: "<dv",
                predicted: "<div>",
                expected_reversal_chars: 0,
                expected_total_chars: 2,
            },
            Case {
                name: "infix insertion not reversal",
                original: indoc! {"
                    from my_project import Foo
                "},
                current: indoc! {"
                    ifrom my_project import Foo
                "},
                predicted: indoc! {"
                    import
                    from my_project import Foo
                "},
                expected_reversal_chars: 0,
                expected_total_chars: 6,
            },
            Case {
                name: "non-word based reversal",
                original: "from",
                current: "ifrom",
                predicted: "from",
                expected_reversal_chars: 1,
                expected_total_chars: 1,
            },
            Case {
                name: "multiple insertions no reversal",
                original: "print(\"Hello, World!\")",
                current: "sys.(\"Hello, World!\")",
                predicted: "sys.stdout.write(\"Hello, World!\\n\")",
                expected_reversal_chars: 0,
                expected_total_chars: 14,
            },
        ];

        for case in &cases {
            let overlap = compute_reversal_overlap(case.original, case.current, case.predicted);
            assert_eq!(
                overlap.chars_reversing_user_edits, case.expected_reversal_chars,
                "Test '{}': expected {} reversal chars, got {}",
                case.name, case.expected_reversal_chars, overlap.chars_reversing_user_edits
            );
            assert_eq!(
                overlap.total_chars_in_prediction, case.expected_total_chars,
                "Test '{}': expected {} total chars, got {}",
                case.name, case.expected_total_chars, overlap.total_chars_in_prediction
            );
        }
    }

    #[test]
    fn test_reverse_diff() {
        let forward_diff = indoc! {"
            --- a/file.rs
            +++ b/file.rs
            @@ -1,3 +1,4 @@
             fn main() {
            +    let x = 42;
                 println!(\"hello\");
            }"};

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
        let original = indoc! {"
            first line
            hello world
            last line
        "};
        let modified = indoc! {"
            first line
            hello beautiful world
            last line
        "};

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
    fn test_filter_edit_history_by_path() {
        // Test that filter_edit_history_by_path correctly matches paths when
        // the edit history has paths with a repo prefix (e.g., "repo/src/file.rs")
        // but the cursor_path doesn't have the repo prefix (e.g., "src/file.rs")
        let events = vec![
            Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("myrepo/src/file.rs")),
                old_path: Arc::from(Path::new("myrepo/src/file.rs")),
                diff: indoc! {"
                    @@ -1 +1 @@
                    -old
                    +new"}
                .into(),
                predicted: false,
                in_open_source_repo: true,
            }),
            Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("myrepo/other.rs")),
                old_path: Arc::from(Path::new("myrepo/other.rs")),
                diff: indoc! {"
                    @@ -1 +1 @@
                    -a
                    +b"}
                .into(),
                predicted: false,
                in_open_source_repo: true,
            }),
            Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("src/file.rs")),
                old_path: Arc::from(Path::new("src/file.rs")),
                diff: indoc! {"
                    @@ -1 +1 @@
                    -x
                    +y"}
                .into(),
                predicted: false,
                in_open_source_repo: true,
            }),
        ];

        // "myrepo/src/file.rs" stripped -> "src/file.rs" matches cursor_path
        // "src/file.rs" exact match
        let cursor_path = Path::new("src/file.rs");
        let filtered = filter_edit_history_by_path(&events, cursor_path);
        assert_eq!(
            filtered.len(),
            2,
            "Should match myrepo/src/file.rs (stripped) and src/file.rs (exact)"
        );

        // "myrepo/src/file.rs" stripped -> "src/file.rs" != "file.rs"
        // "src/file.rs" stripped -> "file.rs" == "file.rs"
        let cursor_path = Path::new("file.rs");
        let filtered = filter_edit_history_by_path(&events, cursor_path);
        assert_eq!(
            filtered.len(),
            1,
            "Should only match src/file.rs (stripped to file.rs)"
        );

        // "myrepo/other.rs" stripped -> "other.rs" == "other.rs"
        let cursor_path = Path::new("other.rs");
        let filtered = filter_edit_history_by_path(&events, cursor_path);
        assert_eq!(filtered.len(), 1, "Should match only myrepo/other.rs");
    }

    #[test]
    fn test_reverse_diff_preserves_trailing_newline() {
        let diff_with_trailing_newline = indoc! {"
            --- a/file
            +++ b/file
            @@ -1 +1 @@
            -old
            +new
        "};
        let reversed = reverse_diff(diff_with_trailing_newline);
        assert!(
            reversed.ends_with('\n'),
            "Reversed diff should preserve trailing newline"
        );

        let diff_without_trailing_newline = indoc! {"
            --- a/file
            +++ b/file
            @@ -1 +1 @@
            -old
            +new"};
        let reversed = reverse_diff(diff_without_trailing_newline);
        assert!(
            !reversed.ends_with('\n'),
            "Reversed diff should not add trailing newline if original didn't have one"
        );
    }

    #[test]
    fn test_filter_hunks_by_excerpt_region() {
        struct Case {
            name: &'static str,
            diff: &'static str,
            excerpt_start_row: u32,
            excerpt_row_count: u32,
            expected_filtered_diff: &'static str,
            expected_line_offset: i32,
        }

        let cases = [
            Case {
                name: "hunk_entirely_before_excerpt",
                diff: indoc! {"
                    @@ -1,3 +1,4 @@
                     line1
                    +inserted
                     line2
                     line3
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 5,
                expected_filtered_diff: "",
                expected_line_offset: 1,
            },
            Case {
                name: "hunk_entirely_inside_excerpt",
                diff: indoc! {"
                    @@ -12,3 +12,4 @@
                     line12
                    +inserted
                     line13
                     line14
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 10,
                expected_filtered_diff: indoc! {"
                    @@ -2,3 +2,4 @@
                     line12
                    +inserted
                     line13
                     line14
                "},
                expected_line_offset: 1,
            },
            Case {
                name: "hunk_entirely_after_excerpt",
                diff: indoc! {"
                    @@ -50,3 +50,4 @@
                     line50
                    +inserted
                     line51
                     line52
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 5,
                expected_filtered_diff: "",
                expected_line_offset: 0,
            },
            Case {
                name: "hunk_straddles_excerpt_start",
                diff: indoc! {"
                    @@ -8,5 +8,6 @@
                     line8
                     line9
                    +inserted
                     line10
                     line11
                     line12
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 10,
                expected_filtered_diff: indoc! {"
                    @@ -1,3 +1,3 @@
                     line10
                     line11
                     line12
                "},
                expected_line_offset: 1,
            },
            Case {
                name: "hunk_straddles_excerpt_end",
                diff: indoc! {"
                    @@ -18,5 +18,6 @@
                     line18
                     line19
                    +inserted
                     line20
                     line21
                     line22
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 10,
                expected_filtered_diff: indoc! {"
                    @@ -8,2 +8,3 @@
                     line18
                     line19
                    +inserted
                "},
                expected_line_offset: 1,
            },
            Case {
                name: "multiple_hunks_mixed",
                diff: indoc! {"
                    @@ -1,2 +1,3 @@
                     line1
                    +before_excerpt
                     line2
                    @@ -12,2 +13,3 @@
                     line12
                    +inside_excerpt
                     line13
                    @@ -50,2 +52,3 @@
                     line50
                    +after_excerpt
                     line51
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 10,
                expected_filtered_diff: indoc! {"
                    @@ -3,2 +3,3 @@
                     line12
                    +inside_excerpt
                     line13
                "},
                expected_line_offset: 2,
            },
            Case {
                name: "deletion_before_excerpt",
                diff: indoc! {"
                    @@ -1,4 +1,3 @@
                     line1
                    -deleted
                     line2
                     line3
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 5,
                expected_filtered_diff: "",
                expected_line_offset: -1,
            },
            Case {
                name: "deletion_inside_excerpt",
                diff: indoc! {"
                    @@ -12,4 +12,3 @@
                     line12
                    -deleted
                     line13
                     line14
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 10,
                expected_filtered_diff: indoc! {"
                    @@ -2,4 +2,3 @@
                     line12
                    -deleted
                     line13
                     line14
                "},
                expected_line_offset: -1,
            },
            Case {
                name: "empty_diff",
                diff: "",
                excerpt_start_row: 10,
                excerpt_row_count: 5,
                expected_filtered_diff: "",
                expected_line_offset: 0,
            },
            Case {
                name: "hunk_spans_entire_excerpt",
                diff: indoc! {"
                    @@ -8,10 +8,12 @@
                     line8
                     line9
                     line10
                     line11
                    +inserted1
                     line12
                     line13
                    +inserted2
                     line14
                     line15
                     line16
                     line17
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 5,
                expected_filtered_diff: indoc! {"
                    @@ -1,3 +1,5 @@
                     line11
                    +inserted1
                     line12
                     line13
                    +inserted2
                "},
                expected_line_offset: 2,
            },
            Case {
                name: "replacement_inside_excerpt",
                diff: indoc! {"
                    @@ -12,3 +12,3 @@
                     line12
                    -old_text
                    +new_text
                     line14
                "},
                excerpt_start_row: 10,
                excerpt_row_count: 10,
                expected_filtered_diff: indoc! {"
                    @@ -2,3 +2,3 @@
                     line12
                    -old_text
                    +new_text
                     line14
                "},
                expected_line_offset: 0,
            },
        ];

        for case in &cases {
            let (filtered, line_offset) = filter_diff_hunks_by_excerpt(
                case.diff,
                case.excerpt_start_row,
                case.excerpt_row_count,
            );
            assert_eq!(
                filtered, case.expected_filtered_diff,
                "Test '{}': filtered diff mismatch.\nExpected:\n{}\nGot:\n{}",
                case.name, case.expected_filtered_diff, filtered
            );
            assert_eq!(
                line_offset, case.expected_line_offset,
                "Test '{}': line offset mismatch. Expected {}, got {}",
                case.name, case.expected_line_offset, line_offset
            );
        }
    }

    #[test]
    fn test_excerpt_aware_reversal_tracking() {
        struct Case {
            name: &'static str,
            edit_history_diffs: Vec<&'static str>,
            excerpt_content: &'static str,
            excerpt_start_row: u32,
            predicted_content: &'static str,
            expected_reversal_chars: usize,
            expected_total_chars: usize,
        }

        let cases = [
            Case {
                name: "edit_outside_excerpt_no_reversal",
                edit_history_diffs: vec![indoc! {"
                    @@ -1,2 +1,3 @@
                     line1
                    +added_outside
                     line2
                "}],
                excerpt_content: indoc! {"
                    line10
                    line11
                    line12
                "},
                excerpt_start_row: 10,
                predicted_content: indoc! {"
                    line10
                    modified
                    line12
                "},
                expected_reversal_chars: 0,
                expected_total_chars: 14,
            },
            Case {
                name: "edit_inside_excerpt_with_reversal",
                edit_history_diffs: vec![indoc! {"
                    @@ -10,3 +10,4 @@
                     line10
                    +user_added
                     line11
                     line12
                "}],
                excerpt_content: indoc! {"
                    line10
                    user_added
                    line11
                    line12
                "},
                excerpt_start_row: 10,
                predicted_content: indoc! {"
                    line10
                    line11
                    line12
                "},
                expected_reversal_chars: 11,
                expected_total_chars: 11,
            },
            Case {
                name: "straddling_edit_partial_reversal",
                edit_history_diffs: vec![indoc! {"
                    @@ -8,6 +8,8 @@
                     line8
                     line9
                    +before_excerpt
                     line10
                    +inside_excerpt
                     line11
                     line12
                     line13
                "}],
                excerpt_content: indoc! {"
                    line10
                    inside_excerpt
                    line11
                    line12
                    line13
                "},
                excerpt_start_row: 10,
                predicted_content: indoc! {"
                    line10
                    line11
                    line12
                    line13
                "},
                expected_reversal_chars: 15,
                expected_total_chars: 15,
            },
            Case {
                name: "multiple_edits_mixed_locations",
                edit_history_diffs: vec![
                    indoc! {"
                        @@ -1,2 +1,3 @@
                         line1
                        +outside1
                         line2
                    "},
                    indoc! {"
                        @@ -11,2 +12,3 @@
                         line11
                        +inside1
                         line12
                    "},
                ],
                excerpt_content: indoc! {"
                    line10
                    line11
                    inside1
                    line12
                    line13
                "},
                excerpt_start_row: 10,
                predicted_content: indoc! {"
                    line10
                    line11
                    line12
                    line13
                "},
                expected_reversal_chars: 8,
                expected_total_chars: 8,
            },
            Case {
                name: "no_edit_history",
                edit_history_diffs: vec![],
                excerpt_content: indoc! {"
                    line10
                    line11
                    line12
                "},
                excerpt_start_row: 10,
                predicted_content: indoc! {"
                    line10
                    modified
                    line12
                "},
                expected_reversal_chars: 0,
                expected_total_chars: 14,
            },
            Case {
                name: "edit_after_excerpt_no_effect",
                edit_history_diffs: vec![indoc! {"
                    @@ -50,2 +50,3 @@
                     line50
                    +added_after
                     line51
                "}],
                excerpt_content: indoc! {"
                    line10
                    line11
                    line12
                "},
                excerpt_start_row: 10,
                predicted_content: indoc! {"
                    line10
                    changed
                    line12
                "},
                expected_reversal_chars: 0,
                expected_total_chars: 13,
            },
            Case {
                name: "line_offset_tracking_across_hunks",
                edit_history_diffs: vec![
                    indoc! {"
                        @@ -1,2 +1,4 @@
                         line1
                        +added1
                        +added2
                         line2
                    "},
                    indoc! {"
                        @@ -12,2 +14,3 @@
                         line12
                        +inside_after_offset
                         line13
                    "},
                ],
                excerpt_content: indoc! {"
                    line10
                    line11
                    line12
                    inside_after_offset
                    line13
                "},
                excerpt_start_row: 10,
                predicted_content: indoc! {"
                    line10
                    line11
                    line12
                    line13
                "},
                expected_reversal_chars: 20,
                expected_total_chars: 20,
            },
        ];

        for case in &cases {
            let overlap = compute_excerpt_aware_reversal_overlap(
                &case.edit_history_diffs,
                case.excerpt_content,
                case.excerpt_start_row,
                case.predicted_content,
            );
            assert_eq!(
                overlap.chars_reversing_user_edits, case.expected_reversal_chars,
                "Test '{}': expected {} reversal chars, got {}",
                case.name, case.expected_reversal_chars, overlap.chars_reversing_user_edits
            );
            assert_eq!(
                overlap.total_chars_in_prediction, case.expected_total_chars,
                "Test '{}': expected {} total chars, got {}",
                case.name, case.expected_total_chars, overlap.total_chars_in_prediction
            );
        }
    }

    #[test]
    fn test_lenient_diff_application() {
        struct Case {
            name: &'static str,
            diff: &'static str,
            content: &'static str,
            expected_result: &'static str,
        }

        let cases = [
            Case {
                name: "hunk_context_not_found_skipped",
                diff: indoc! {"
                    @@ -1,3 +1,4 @@
                     context_not_in_content
                    +added_line
                     more_context
                     final_context
                "},
                content: indoc! {"
                    completely
                    different
                    content
                "},
                expected_result: indoc! {"
                    completely
                    different
                    content
                "},
            },
            Case {
                name: "hunk_context_found_applied",
                diff: indoc! {"
                    @@ -1,3 +1,4 @@
                     line1
                    +inserted
                     line2
                     line3
                "},
                content: indoc! {"
                    line1
                    line2
                    line3
                "},
                expected_result: indoc! {"
                    line1
                    inserted
                    line2
                    line3
                "},
            },
            Case {
                name: "multiple_hunks_partial_match",
                diff: indoc! {"
                    @@ -1,2 +1,3 @@
                     not_found
                    +skipped
                     also_not_found
                    @@ -5,2 +6,3 @@
                     line5
                    +applied
                     line6
                "},
                content: indoc! {"
                    line1
                    line2
                    line3
                    line4
                    line5
                    line6
                "},
                expected_result: indoc! {"
                    line1
                    line2
                    line3
                    line4
                    line5
                    applied
                    line6
                "},
            },
            Case {
                name: "empty_diff",
                diff: "",
                content: indoc! {"
                    unchanged
                    content
                "},
                expected_result: indoc! {"
                    unchanged
                    content
                "},
            },
        ];

        for case in &cases {
            let result = apply_diff_to_string_lenient(case.diff, case.content);
            assert_eq!(
                result, case.expected_result,
                "Test '{}': expected:\n{}\ngot:\n{}",
                case.name, case.expected_result, result
            );
        }
    }

    #[test]
    fn test_unicode_reversal_overlap() {
        struct Case {
            name: &'static str,
            original: &'static str,
            current: &'static str,
            predicted: &'static str,
            expected_reversal_chars: usize,
            expected_total_chars: usize,
        }

        let cases = [
            Case {
                name: "unicode_extension_cjk",
                original: "",
                current: "日",       // 1 char
                predicted: "日本語", // 3 chars, adds 2 chars
                expected_reversal_chars: 0,
                expected_total_chars: 2, // "本語" = 2 chars added
            },
            Case {
                name: "unicode_extension_emoji",
                original: "",
                current: "🎉",       // 1 char
                predicted: "🎉🎊🎈", // 3 chars, adds 2 chars
                expected_reversal_chars: 0,
                expected_total_chars: 2, // "🎊🎈" = 2 chars added
            },
            Case {
                name: "unicode_deletion_restored",
                original: "héllo wörld",    // 11 chars
                current: "héllo",           // 5 chars
                predicted: "héllo wörld",   // restores " wörld" = 6 chars
                expected_reversal_chars: 6, // LCS(" wörld", " wörld") = 6 chars
                expected_total_chars: 6,
            },
            Case {
                name: "unicode_addition_reversed",
                original: "café",           // 4 chars
                current: "café latté",      // 10 chars, added " latté" = 6 chars
                predicted: "café",          // removes " latté"
                expected_reversal_chars: 6, // 6 chars removed
                expected_total_chars: 6,
            },
            Case {
                name: "mixed_ascii_unicode",
                original: "",
                current: "test日本",         // 6 chars
                predicted: "test日本語です", // 9 chars
                expected_reversal_chars: 0,
                expected_total_chars: 3, // 3 new chars after subsequence normalization
            },
            Case {
                name: "unicode_replacement_not_subsequence",
                original: "",
                current: "日本",            // 2 chars
                predicted: "中国",          // 2 chars, different
                expected_reversal_chars: 2, // removes "日本" = 2 chars
                expected_total_chars: 4,    // 2 removed + 2 added
            },
        ];

        for case in &cases {
            let overlap = compute_reversal_overlap(case.original, case.current, case.predicted);
            assert_eq!(
                overlap.chars_reversing_user_edits, case.expected_reversal_chars,
                "Test '{}': expected {} reversal chars, got {}",
                case.name, case.expected_reversal_chars, overlap.chars_reversing_user_edits
            );
            assert_eq!(
                overlap.total_chars_in_prediction, case.expected_total_chars,
                "Test '{}': expected {} total chars, got {}",
                case.name, case.expected_total_chars, overlap.total_chars_in_prediction
            );
        }
    }

    #[test]
    fn test_is_subsequence() {
        assert!(is_subsequence("", "anything"));
        assert!(is_subsequence("", ""));
        assert!(is_subsequence("abc", "abc"));
        assert!(is_subsequence("abc", "aXbXc"));
        assert!(is_subsequence("ac", "abc"));
        assert!(!is_subsequence("abc", "ab"));
        assert!(!is_subsequence("abc", "cba"));
        assert!(!is_subsequence("abc", ""));
        assert!(is_subsequence("日本", "日X本Y語"));
        assert!(!is_subsequence("日本語", "日本"));
    }

    #[test]
    fn test_compute_lcs_length() {
        assert_eq!(compute_lcs_length("", ""), 0);
        assert_eq!(compute_lcs_length("abc", ""), 0);
        assert_eq!(compute_lcs_length("", "abc"), 0);
        assert_eq!(compute_lcs_length("abc", "abc"), 3);
        assert_eq!(compute_lcs_length("abc", "def"), 0);
        assert_eq!(compute_lcs_length("abcdef", "ace"), 3);
        assert_eq!(compute_lcs_length("AGGTAB", "GXTXAYB"), 4);
        assert_eq!(compute_lcs_length("日本語", "日語"), 2);
    }

    #[test]
    fn test_compute_prediction_reversal_ratio_full_file() {
        let prompt_inputs = ExamplePromptInputs {
            content: indoc! {"
                line1
                user_added
                line2
            "}
            .to_string(),
            cursor_row: 0,
            cursor_column: 0,
            cursor_offset: 0,
            edit_history: vec![Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("src/test.rs")),
                old_path: Arc::from(Path::new("src/test.rs")),
                diff: indoc! {"
                    @@ -1,2 +1,3 @@
                     line1
                    +user_added
                     line2
                "}
                .into(),
                predicted: false,
                in_open_source_repo: false,
            })],
            excerpt_start_row: None,
            related_files: None,
        };

        let predicted = indoc! {"
            line1
            line2
        "};
        let ratio =
            compute_prediction_reversal_ratio(&prompt_inputs, predicted, Path::new("src/test.rs"));

        assert!(
            ratio > 0.9,
            "Expected high reversal ratio when prediction removes user addition, got {}",
            ratio
        );
    }

    #[test]
    fn test_compute_prediction_reversal_ratio_with_excerpt() {
        let prompt_inputs = ExamplePromptInputs {
            content: indoc! {"
                line10
                user_added
                line11
            "}
            .to_string(),
            cursor_row: 0,
            cursor_column: 0,
            cursor_offset: 0,
            edit_history: vec![Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("src/test.rs")),
                old_path: Arc::from(Path::new("src/test.rs")),
                diff: indoc! {"
                    @@ -10,2 +10,3 @@
                     line10
                    +user_added
                     line11
                "}
                .into(),
                predicted: false,
                in_open_source_repo: false,
            })],
            excerpt_start_row: Some(10),
            related_files: None,
        };

        let predicted = indoc! {"
            line10
            line11
        "};
        let ratio =
            compute_prediction_reversal_ratio(&prompt_inputs, predicted, Path::new("src/test.rs"));

        assert!(
            ratio > 0.9,
            "Expected high reversal ratio for excerpt-aware computation, got {}",
            ratio
        );
    }

    #[test]
    fn test_compute_prediction_reversal_ratio_no_history() {
        let prompt_inputs = ExamplePromptInputs {
            content: indoc! {"
                original content
            "}
            .to_string(),
            cursor_row: 0,
            cursor_column: 0,
            cursor_offset: 0,
            edit_history: vec![],
            excerpt_start_row: None,
            related_files: None,
        };

        let predicted = indoc! {"
            completely different
        "};
        let ratio =
            compute_prediction_reversal_ratio(&prompt_inputs, predicted, Path::new("src/test.rs"));

        assert_eq!(
            ratio, 0.0,
            "Expected zero reversal ratio with no edit history"
        );
    }

    #[test]
    fn test_compute_prediction_reversal_ratio_path_filtering() {
        let prompt_inputs = ExamplePromptInputs {
            content: indoc! {"
                line1
                user_added
                line2
            "}
            .to_string(),
            cursor_row: 0,
            cursor_column: 0,
            cursor_offset: 0,
            edit_history: vec![Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("src/other.rs")),
                old_path: Arc::from(Path::new("src/other.rs")),
                diff: indoc! {"
                    @@ -1,2 +1,3 @@
                     line1
                    +user_added
                     line2
                "}
                .into(),
                predicted: false,
                in_open_source_repo: false,
            })],
            excerpt_start_row: None,
            related_files: None,
        };

        let predicted = indoc! {"
            line1
            line2
        "};
        let ratio =
            compute_prediction_reversal_ratio(&prompt_inputs, predicted, Path::new("src/test.rs"));

        assert_eq!(
            ratio, 0.0,
            "Expected zero reversal when edit history is for different file"
        );
    }

    #[test]
    fn test_compute_prediction_reversal_ratio_lenient_fallback() {
        let prompt_inputs = ExamplePromptInputs {
            content: indoc! {"
                actual_line1
                user_added
                actual_line2
            "}
            .to_string(),
            cursor_row: 0,
            cursor_column: 0,
            cursor_offset: 0,
            edit_history: vec![Arc::new(zeta_prompt::Event::BufferChange {
                path: Arc::from(Path::new("src/test.rs")),
                old_path: Arc::from(Path::new("src/test.rs")),
                diff: indoc! {"
                    @@ -1,2 +1,3 @@
                     wrong_context
                    +user_added
                     more_wrong
                "}
                .into(),
                predicted: false,
                in_open_source_repo: false,
            })],
            excerpt_start_row: None,
            related_files: None,
        };

        let predicted = indoc! {"
            actual_line1
            actual_line2
        "};
        let ratio =
            compute_prediction_reversal_ratio(&prompt_inputs, predicted, Path::new("src/test.rs"));

        assert!(
            ratio >= 0.0 && ratio <= 1.0,
            "Ratio should be valid even with lenient fallback, got {}",
            ratio
        );
    }

    #[test]
    fn test_excerpt_aware_reversal_error_recovery() {
        let diffs = vec![indoc! {"
            @@ -1,2 +1,3 @@
             nonexistent_context
            +added
             more_nonexistent
        "}];
        let excerpt_content = indoc! {"
            completely
            different
            content
        "};
        let predicted_content = indoc! {"
            completely
            modified
            content
        "};

        let overlap =
            compute_excerpt_aware_reversal_overlap(&diffs, excerpt_content, 0, predicted_content);

        assert!(
            overlap.ratio() >= 0.0 && overlap.ratio() <= 1.0,
            "Should handle failed diff application gracefully"
        );
    }

    #[test]
    fn test_multiple_sequential_diffs() {
        let prompt_inputs = ExamplePromptInputs {
            content: indoc! {"
                line1
                first_add
                second_add
                line2
            "}
            .to_string(),
            cursor_row: 0,
            cursor_column: 0,
            cursor_offset: 0,
            edit_history: vec![
                Arc::new(zeta_prompt::Event::BufferChange {
                    path: Arc::from(Path::new("src/test.rs")),
                    old_path: Arc::from(Path::new("src/test.rs")),
                    diff: indoc! {"
                        @@ -1,2 +1,3 @@
                         line1
                        +first_add
                         line2
                    "}
                    .into(),
                    predicted: false,
                    in_open_source_repo: false,
                }),
                Arc::new(zeta_prompt::Event::BufferChange {
                    path: Arc::from(Path::new("src/test.rs")),
                    old_path: Arc::from(Path::new("src/test.rs")),
                    diff: indoc! {"
                        @@ -2,2 +2,3 @@
                         first_add
                        +second_add
                         line2
                    "}
                    .into(),
                    predicted: false,
                    in_open_source_repo: false,
                }),
            ],
            excerpt_start_row: None,
            related_files: None,
        };

        let predicted = indoc! {"
            line1
            line2
        "};
        let ratio =
            compute_prediction_reversal_ratio(&prompt_inputs, predicted, Path::new("src/test.rs"));

        assert!(
            ratio > 0.9,
            "Expected high reversal ratio when reversing multiple sequential edits, got {}",
            ratio
        );
    }
}
