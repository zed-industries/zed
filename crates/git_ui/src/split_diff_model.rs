use crate::split_diff_settings::{IntralineMode, SplitDiffSettings, WhitespaceMode};
use buffer_diff::{BufferDiffSnapshot, DiffHunk as BufferDiffHunk};

use git::repository::RepoPath;
use gpui::{AsyncApp, Entity, Task};
use language::Buffer;
use std::{ops::Range, sync::LazyLock};
use text::ToOffset as _;
use util::rel_path::RelPath;

pub static COMPUTE_DIFF_TASK: LazyLock<gpui::TaskLabel> = LazyLock::new(gpui::TaskLabel::new);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiffSpec {
    pub path: RepoPath,
    pub revision: Option<String>, // None for working directory
}

impl DiffSpec {
    pub fn working_directory(path: RepoPath) -> Self {
        Self {
            path,
            revision: None,
        }
    }

    pub fn revision(path: RepoPath, revision: String) -> Self {
        Self {
            path,
            revision: Some(revision),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiffHunk {
    pub left_range: Range<u32>,
    pub right_range: Range<u32>,
    pub status: DiffHunkStatus,
    pub intra_line_changes: Vec<IntraLineChange>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffHunkStatus {
    Added,
    Removed,
    Modified,
}

#[derive(Clone, Debug)]
pub struct IntraLineChange {
    pub left_range: Option<Range<u32>>,  // None if added
    pub right_range: Option<Range<u32>>, // None if removed
    pub change_type: IntraLineChangeType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntraLineChangeType {
    Addition,
    Removal,
    Modification,
}

#[derive(Clone, Debug)]
pub struct SplitDiffModel {
    pub left_spec: DiffSpec,
    pub right_spec: DiffSpec,
    pub hunks: Vec<DiffHunk>,
    pub left_content: String,
    pub right_content: String,
    pub options: DiffOptions,
}

impl SplitDiffModel {
    pub fn new(left_content: String, right_content: String) -> Self {
        Self {
            left_spec: DiffSpec::working_directory(Default::default()),
            right_spec: DiffSpec::revision(Default::default(), "HEAD".to_string()),
            hunks: Vec::new(),
            left_content,
            right_content,
            options: DiffOptions::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiffOptions {
    pub context_lines: u32,
    pub ignore_whitespace: WhitespaceMode,
    pub intraline: IntralineMode,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            context_lines: 3,
            ignore_whitespace: WhitespaceMode::None,
            intraline: IntralineMode::Word,
        }
    }
}

impl DiffOptions {
    pub fn from_settings(settings: &SplitDiffSettings) -> Self {
        Self {
            context_lines: settings.context_lines,
            ignore_whitespace: settings.ignore_whitespace.clone(),
            intraline: settings.intraline.clone(),
        }
    }
}

pub struct DiffComputation {
    left_buffer: Entity<Buffer>,
    right_buffer: Entity<Buffer>,
    options: DiffOptions,
}

impl DiffComputation {
    pub fn new(
        left_buffer: Entity<Buffer>,
        right_buffer: Entity<Buffer>,
        options: DiffOptions,
    ) -> Self {
        Self {
            left_buffer,
            right_buffer,
            options,
        }
    }

    pub fn compute(self, cx: &mut AsyncApp) -> Task<anyhow::Result<SplitDiffModel>> {
        cx.spawn(async move |cx| {
            let left_snapshot = self
                .left_buffer
                .read_with(cx, |buffer, _| buffer.snapshot())?;
            let right_snapshot = self
                .right_buffer
                .read_with(cx, |buffer, _| buffer.snapshot())?;

            let left_content = left_snapshot.text();
            let right_content = right_snapshot.text();

            // Use buffer_diff to compute the diff
            let diff_snapshot = cx
                .update(|cx| {
                    BufferDiffSnapshot::new_with_base_buffer(
                        right_snapshot.text.clone(),
                        Some(left_snapshot.text().into()),
                        left_snapshot.clone(),
                        cx,
                    )
                })?
                .await;

            let full_range = language::Anchor::MIN..language::Anchor::MAX;
            let buffer_hunks = diff_snapshot
                .hunks_intersecting_range(full_range, &right_snapshot)
                .collect::<Vec<_>>();

            // Convert buffer hunks to our model hunks
            let mut hunks = Vec::new();
            for hunk in buffer_hunks {
                let diff_hunk = self.convert_buffer_hunk_to_diff_hunk(
                    &hunk,
                    &left_content,
                    &right_content,
                    &left_snapshot,
                    &right_snapshot,
                );
                hunks.push(diff_hunk);
            }

            Ok(SplitDiffModel {
                left_spec: DiffSpec::working_directory(RepoPath::from(
                    RelPath::unix("left").unwrap(),
                )), // TODO: proper spec
                right_spec: DiffSpec::working_directory(RepoPath::from(
                    RelPath::unix("right").unwrap(),
                )),
                hunks,
                left_content: left_content.to_string(),
                right_content: right_content.to_string(),
                options: self.options,
            })
        })
    }

    fn convert_buffer_hunk_to_diff_hunk(
        &self,
        buffer_hunk: &BufferDiffHunk,
        left_content: &str,
        right_content: &str,
        left_snapshot: &language::BufferSnapshot,
        right_snapshot: &text::BufferSnapshot,
    ) -> DiffHunk {
        // Calculate line ranges from buffer ranges
        let left_start_offset = buffer_hunk.diff_base_byte_range.start;
        let left_end_offset = buffer_hunk.diff_base_byte_range.end;
        let left_start_line = left_content[..left_start_offset]
            .chars()
            .filter(|&c| c == '\n')
            .count() as u32;
        let left_end_line = left_content[..left_end_offset]
            .chars()
            .filter(|&c| c == '\n')
            .count() as u32;

        let right_start_offset = buffer_hunk.buffer_range.start.to_offset(right_snapshot);
        let right_end_offset = buffer_hunk.buffer_range.end.to_offset(right_snapshot);
        let right_start_line = right_content[..right_start_offset]
            .chars()
            .filter(|&c| c == '\n')
            .count() as u32;
        let right_end_line = right_content[..right_end_offset]
            .chars()
            .filter(|&c| c == '\n')
            .count() as u32;

        let buffer_range_empty =
            buffer_hunk.buffer_range.start.offset == buffer_hunk.buffer_range.end.offset;
        let status = match (
            buffer_hunk.diff_base_byte_range.is_empty(),
            buffer_range_empty,
        ) {
            (true, false) => DiffHunkStatus::Added,
            (false, true) => DiffHunkStatus::Removed,
            (false, false) => DiffHunkStatus::Modified,
            (true, true) => unreachable!("Empty hunk"),
        };

        let intra_line_changes = if self.options.intraline != IntralineMode::None {
            self.compute_intra_line_changes(
                buffer_hunk,
                left_content,
                right_content,
                left_snapshot,
                right_snapshot,
            )
        } else {
            Vec::new()
        };

        DiffHunk {
            left_range: left_start_line..left_end_line,
            right_range: right_start_line..right_end_line,
            status,
            intra_line_changes,
        }
    }

    fn compute_intra_line_changes(
        &self,
        buffer_hunk: &BufferDiffHunk,
        left_content: &str,
        right_content: &str,
        _left_snapshot: &language::BufferSnapshot,
        right_snapshot: &text::BufferSnapshot,
    ) -> Vec<IntraLineChange> {
        // Simple word-level diff for intra-line changes
        // This is a placeholder - a more sophisticated algorithm would be needed for production
        let left_text = &left_content[buffer_hunk.diff_base_byte_range.clone()];
        let right_start_offset = buffer_hunk.buffer_range.start.to_offset(right_snapshot);
        let right_end_offset = buffer_hunk.buffer_range.end.to_offset(right_snapshot);
        let right_text = &right_content[right_start_offset..right_end_offset];

        let mut changes = Vec::new();

        if self.options.intraline == IntralineMode::Word {
            // Simple word-based diff
            let left_words: Vec<&str> = left_text.split_whitespace().collect();
            let right_words: Vec<&str> = right_text.split_whitespace().collect();

            let mut left_idx = 0;
            let mut right_idx = 0;

            while left_idx < left_words.len() && right_idx < right_words.len() {
                if left_words[left_idx] == right_words[right_idx] {
                    left_idx += 1;
                    right_idx += 1;
                } else {
                    // Find the next matching word
                    let mut found_match = false;
                    for i in (left_idx + 1)..left_words.len() {
                        if left_words[i] == right_words[right_idx] {
                            // Words from left_idx to i-1 are removed
                            if left_idx < i {
                                changes.push(IntraLineChange {
                                    left_range: Some(self.word_range(left_text, left_idx, i)),
                                    right_range: None,
                                    change_type: IntraLineChangeType::Removal,
                                });
                            }
                            left_idx = i;
                            found_match = true;
                            break;
                        }
                    }

                    if !found_match {
                        for i in (right_idx + 1)..right_words.len() {
                            if left_words[left_idx] == right_words[i] {
                                // Words from right_idx to i-1 are added
                                if right_idx < i {
                                    changes.push(IntraLineChange {
                                        left_range: None,
                                        right_range: Some(
                                            self.word_range(right_text, right_idx, i),
                                        ),
                                        change_type: IntraLineChangeType::Addition,
                                    });
                                }
                                right_idx = i;
                                found_match = true;
                                break;
                            }
                        }
                    }

                    if !found_match {
                        // Words are different - treat as modification
                        changes.push(IntraLineChange {
                            left_range: Some(self.word_range(left_text, left_idx, left_idx + 1)),
                            right_range: Some(self.word_range(
                                right_text,
                                right_idx,
                                right_idx + 1,
                            )),
                            change_type: IntraLineChangeType::Modification,
                        });
                        left_idx += 1;
                        right_idx += 1;
                    }
                }
            }

            // Handle remaining words
            if left_idx < left_words.len() {
                changes.push(IntraLineChange {
                    left_range: Some(self.word_range(left_text, left_idx, left_words.len())),
                    right_range: None,
                    change_type: IntraLineChangeType::Removal,
                });
            }

            if right_idx < right_words.len() {
                changes.push(IntraLineChange {
                    left_range: None,
                    right_range: Some(self.word_range(right_text, right_idx, right_words.len())),
                    change_type: IntraLineChangeType::Addition,
                });
            }
        }

        changes
    }

    fn word_range(&self, text: &str, start_word: usize, end_word: usize) -> Range<u32> {
        let words: Vec<&str> = text.split_whitespace().collect();
        if start_word >= words.len() {
            return text.len() as u32..text.len() as u32;
        }

        let mut start_pos = 0;
        let mut end_pos = text.len();

        // Find start position
        let mut word_count = 0;
        let mut chars = text.char_indices();
        while let Some((pos, ch)) = chars.next() {
            if ch.is_whitespace() {
                word_count += 1;
                if word_count == start_word {
                    start_pos = pos + ch.len_utf8();
                    break;
                }
            }
        }

        // Find end position
        word_count = 0;
        let mut chars = text.char_indices();
        while let Some((pos, ch)) = chars.next() {
            if ch.is_whitespace() {
                word_count += 1;
                if word_count == end_word {
                    end_pos = pos;
                    break;
                }
            }
        }

        start_pos as u32..end_pos as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_spec_creation() {
        let spec = DiffSpec::working_directory(RepoPath::from(RelPath::unix("test.txt").unwrap()));
        assert_eq!(
            spec.path,
            RepoPath::from(RelPath::unix("test.txt").unwrap())
        );
        assert!(spec.revision.is_none());

        let spec = DiffSpec::revision(
            RepoPath::from(RelPath::unix("test.txt").unwrap()),
            "HEAD".to_string(),
        );
        assert_eq!(
            spec.path,
            RepoPath::from(RelPath::unix("test.txt").unwrap())
        );
        assert_eq!(spec.revision, Some("HEAD".to_string()));
    }

    #[test]
    fn test_diff_options_from_settings() {
        let settings = SplitDiffSettings::default();
        let options = DiffOptions::from_settings(&settings);
        assert_eq!(options.context_lines, 3);
        assert_eq!(options.ignore_whitespace, WhitespaceMode::None);
        assert_eq!(options.intraline, IntralineMode::Word);
    }
}
