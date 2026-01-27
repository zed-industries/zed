use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

use edit_prediction::udiff::apply_diff_to_string;
use language::text_diff;

use crate::example::ExamplePromptInputs;

pub fn reverse_diff(diff: &str) -> String {
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
pub struct GranularEdit {
    pub range: Range<usize>,
    pub old_text: String,
    pub new_text: String,
}

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

#[derive(Debug, Clone)]
pub struct HistoryAdditionRange {
    pub range_in_current: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct HistoryDeletionRange {
    pub deleted_text: String,
}

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
            });
        }

        offset_delta += edit.new_text.len() as isize - edit.old_text.len() as isize;
    }

    result
}

pub fn compute_history_deletion_ranges(
    history_edits: &[GranularEdit],
) -> Vec<HistoryDeletionRange> {
    history_edits
        .iter()
        .filter(|edit| !edit.old_text.is_empty())
        .map(|edit| HistoryDeletionRange {
            deleted_text: edit.old_text.clone(),
        })
        .collect()
}

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
        compute_reversed_additions(&history_addition_ranges, &prediction_edits);
    let restored_deletions =
        compute_restored_deletions(&history_deletion_ranges, &prediction_edits);

    let prediction_added_chars: usize = prediction_edits.iter().map(|e| e.new_text.len()).sum();
    let prediction_deleted_chars: usize = prediction_edits.iter().map(|e| e.old_text.len()).sum();

    ReversalOverlap {
        chars_reversing_user_edits: reversed_additions + restored_deletions,
        total_chars_in_prediction: prediction_added_chars + prediction_deleted_chars,
    }
}

pub fn compute_reversed_additions(
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
                reversed_chars += overlap_end - overlap_start;
            }
        }
    }

    reversed_chars
}

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

    compute_lcs_length(&history_deleted_text, &prediction_added_text)
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

pub fn filter_edit_history_by_path<'a>(
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

pub fn extract_diff_from_event(event: &zeta_prompt::Event) -> &str {
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
            Err(err) => {
                log::warn!(
                    "Failed to reconstruct original content for reversal tracking: Failed to apply reversed diff: {:#}",
                    err
                );
                return 0.0;
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
                original: "a\nb\nc",
                current: "a\nnew line\nb\nc",
                predicted: "a\nb\nc",
                expected_reversal_chars: 9,
                expected_total_chars: 9,
            },
            Case {
                name: "user_deletes_line_prediction_restores_it",
                original: "a\ndeleted\nb",
                current: "a\nb",
                predicted: "a\ndeleted\nb",
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
                original: "line1\nline2\nline3",
                current: "LINE1\nline2\nline3",
                predicted: "LINE1\nline2\nLINE3",
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
                original: "keep\ndelete_me\nkeep2",
                current: "keep\nadded\nkeep2",
                predicted: "keep\ndelete_me\nkeep2",
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
        let forward_diff = "\
--- a/file.rs
+++ b/file.rs
@@ -1,3 +1,4 @@
 fn main() {
+    let x = 42;
     println!(\"hello\");
}";

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
    fn test_filter_edit_history_by_path() {
        // Test that filter_edit_history_by_path correctly matches paths when
        // the edit history has paths with a repo prefix (e.g., "repo/src/file.rs")
        // but the cursor_path doesn't have the repo prefix (e.g., "src/file.rs")
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
        let diff_with_trailing_newline = "--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new\n";
        let reversed = reverse_diff(diff_with_trailing_newline);
        assert!(
            reversed.ends_with('\n'),
            "Reversed diff should preserve trailing newline"
        );

        let diff_without_trailing_newline = "--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new";
        let reversed = reverse_diff(diff_without_trailing_newline);
        assert!(
            !reversed.ends_with('\n'),
            "Reversed diff should not add trailing newline if original didn't have one"
        );
    }
}
