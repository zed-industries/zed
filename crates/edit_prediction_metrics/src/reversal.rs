use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

use language::{char_diff, text_diff};
use zeta_prompt::udiff::apply_diff_to_string;

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
        .flat_map(|edit| {
            if edit.old_text.is_empty() || edit.new_text.is_empty() {
                return vec![edit];
            }

            // Use character-wise diff to find exact byte ranges of changes
            let char_edits = char_diff(&edit.old_text, &edit.new_text);

            let all_deletions = !char_edits.is_empty()
                && char_edits
                    .iter()
                    .all(|(range, replacement)| !range.is_empty() && replacement.is_empty());
            let all_insertions = !char_edits.is_empty()
                && char_edits
                    .iter()
                    .all(|(range, replacement)| range.is_empty() && !replacement.is_empty());
            if all_deletions || all_insertions {
                return char_edits
                    .into_iter()
                    .map(|(range, replacement)| GranularEdit {
                        range: edit.range.start + range.start..edit.range.start + range.end,
                        old_text: edit.old_text[range].to_string(),
                        new_text: replacement.to_string(),
                    })
                    .collect();
            }

            // Otherwise, keep the original edit (mixed changes)
            vec![edit]
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

fn is_predicted_event(event: &zeta_prompt::Event) -> bool {
    match event {
        zeta_prompt::Event::BufferChange { predicted, .. } => *predicted,
    }
}

pub fn compute_prediction_reversal_ratio_from_history(
    current_content: &str,
    edit_history: &[Arc<zeta_prompt::Event>],
    excerpt_start_row: Option<u32>,
    predicted_content: &str,
    cursor_path: &Path,
) -> f32 {
    let relevant_events = filter_edit_history_by_path(edit_history, cursor_path);

    let most_recent = match relevant_events.last() {
        Some(event) if !is_predicted_event(event) => *event,
        _ => return 0.0,
    };

    let diff = extract_diff_from_event(most_recent);
    if diff.is_empty() {
        return 0.0;
    }

    if let Some(excerpt_start_row) = excerpt_start_row {
        let diffs = vec![diff];
        let overlap = compute_excerpt_aware_reversal_overlap(
            &diffs,
            current_content,
            excerpt_start_row,
            predicted_content,
        );
        return overlap.ratio();
    }

    let reversed = reverse_diff(diff);
    let with_headers = format!("--- a/file\n+++ b/file\n{}", reversed);
    let original_content = match apply_diff_to_string(&with_headers, current_content) {
        Ok(updated_content) => updated_content,
        Err(_) => apply_diff_to_string_lenient(&reversed, current_content),
    };

    let overlap = compute_reversal_overlap(&original_content, current_content, predicted_content);
    overlap.ratio()
}
