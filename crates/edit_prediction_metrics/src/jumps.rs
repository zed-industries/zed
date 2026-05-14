use std::ops::Range;

use crate::patch::{Patch, PatchLine};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Excerpt {
    pub path: String,
    pub row_range: Range<u32>,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EditableContextCoverage {
    pub changed_lines_reachable: usize,
    pub total_changed_lines: usize,
    pub score: f64,
}

impl EditableContextCoverage {
    pub fn new(changed_lines_reachable: usize, total_changed_lines: usize) -> Self {
        let score = if total_changed_lines == 0 {
            1.0
        } else {
            changed_lines_reachable as f64 / total_changed_lines as f64
        };
        Self {
            changed_lines_reachable,
            total_changed_lines,
            score,
        }
    }
}

/// Measures how much of the expected edits are covered by the context.
pub fn editable_context_coverage(
    expected_patch: &str,
    context: &[Excerpt],
) -> EditableContextCoverage {
    let patch = Patch::parse_unified_diff(expected_patch);
    let mut changed_lines_reachable = 0;
    let mut total_changed_lines = 0;
    for hunk in patch.hunks {
        let mut old_row = hunk.old_start.saturating_sub(1).max(0) as u32;
        for line in hunk.lines {
            match line {
                PatchLine::Addition(_) => {
                    total_changed_lines += 1;
                    if context_contains_insertion_point(context, &hunk.filename, old_row) {
                        changed_lines_reachable += 1;
                    }
                }
                PatchLine::Deletion(_) => {
                    total_changed_lines += 1;
                    if context_contains_line(context, &hunk.filename, old_row) {
                        changed_lines_reachable += 1;
                    }
                    old_row = old_row.saturating_add(1);
                }
                PatchLine::Context(_) => {
                    old_row = old_row.saturating_add(1);
                }
                _ => {}
            }
        }
    }

    EditableContextCoverage::new(changed_lines_reachable, total_changed_lines)
}

fn context_contains_line(context: &[Excerpt], filename: &str, row: u32) -> bool {
    context
        .iter()
        .any(|excerpt| excerpt.path == filename && excerpt.row_range.contains(&row))
}

fn context_contains_insertion_point(context: &[Excerpt], filename: &str, row: u32) -> bool {
    context.iter().any(|excerpt| {
        excerpt.path == filename && excerpt.row_range.start <= row && row <= excerpt.row_range.end
    })
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
    fn deletion_is_reachable_when_zero_based_excerpt_contains_diff_line() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -2,1 +2,0 @@
            -let value = 1;
        "};

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 1..2)]);

        assert_eq!(score.changed_lines_reachable, 1);
        assert_eq!(score.total_changed_lines, 1);
        assert_eq!(score.score, 1.0);
    }

    #[test]
    fn searches_all_excerpts_for_matching_path() {
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

        assert_eq!(score.changed_lines_reachable, 1);
        assert_eq!(score.total_changed_lines, 1);
        assert_eq!(score.score, 1.0);
    }

    #[test]
    fn replacement_addition_is_reachable_at_deleted_line_boundary() {
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

        assert_eq!(score.changed_lines_reachable, 2);
        assert_eq!(score.total_changed_lines, 2);
        assert_eq!(score.score, 1.0);
    }

    #[test]
    fn insertion_is_reachable_at_excerpt_end_boundary() {
        let patch = indoc! {"
            --- a/src/main.rs
            +++ b/src/main.rs
            @@ -1,2 +1,3 @@
             fn main() {
            +    let value = 1;
             }
        "};

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 0..1)]);

        assert_eq!(score.changed_lines_reachable, 1);
        assert_eq!(score.total_changed_lines, 1);
        assert_eq!(score.score, 1.0);
    }

    #[test]
    fn counts_unreachable_changed_lines() {
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

        let score = editable_context_coverage(patch, &[excerpt("src/main.rs", 0..1)]);

        assert_eq!(score.changed_lines_reachable, 2);
        assert_eq!(score.total_changed_lines, 4);
        assert_eq!(score.score, 0.5);
    }

    #[test]
    fn empty_patch_has_perfect_reachability() {
        let score = editable_context_coverage(
            indoc! {"
            "},
            &[],
        );

        assert_eq!(score.changed_lines_reachable, 0);
        assert_eq!(score.total_changed_lines, 0);
        assert_eq!(score.score, 1.0);
    }
}
