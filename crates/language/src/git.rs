use std::ops::Range;

use sum_tree::SumTree;
use text::{Anchor, BufferSnapshot, OffsetRangeExt, Point, ToPoint};

pub use git2 as libgit;
use libgit::{DiffLineType as GitDiffLineType, DiffOptions as GitOptions, Patch as GitPatch};

#[derive(Debug, Clone, Copy)]
pub enum DiffHunkStatus {
    Added,
    Modified,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk<T> {
    pub buffer_range: Range<T>,
    pub head_byte_range: Range<usize>,
}

impl DiffHunk<u32> {
    pub fn status(&self) -> DiffHunkStatus {
        if self.head_byte_range.is_empty() {
            DiffHunkStatus::Added
        } else if self.buffer_range.is_empty() {
            DiffHunkStatus::Removed
        } else {
            DiffHunkStatus::Modified
        }
    }
}

impl sum_tree::Item for DiffHunk<Anchor> {
    type Summary = DiffHunkSummary;

    fn summary(&self) -> Self::Summary {
        DiffHunkSummary {
            buffer_range: self.buffer_range.clone(),
            head_range: self.head_byte_range.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiffHunkSummary {
    buffer_range: Range<Anchor>,
    head_range: Range<usize>,
}

impl sum_tree::Summary for DiffHunkSummary {
    type Context = text::BufferSnapshot;

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.head_range.start = self.head_range.start.min(other.head_range.start);
        self.head_range.end = self.head_range.end.max(other.head_range.end);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HunkHeadEnd(usize);

impl<'a> sum_tree::Dimension<'a, DiffHunkSummary> for HunkHeadEnd {
    fn add_summary(&mut self, summary: &'a DiffHunkSummary, _: &text::BufferSnapshot) {
        self.0 = summary.head_range.end;
    }

    fn from_summary(summary: &'a DiffHunkSummary, _: &text::BufferSnapshot) -> Self {
        HunkHeadEnd(summary.head_range.end)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HunkBufferStart(u32);

impl<'a> sum_tree::Dimension<'a, DiffHunkSummary> for HunkBufferStart {
    fn add_summary(&mut self, summary: &'a DiffHunkSummary, buffer: &text::BufferSnapshot) {
        self.0 = summary.buffer_range.start.to_point(buffer).row;
    }

    fn from_summary(summary: &'a DiffHunkSummary, buffer: &text::BufferSnapshot) -> Self {
        HunkBufferStart(summary.buffer_range.start.to_point(buffer).row)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HunkBufferEnd(u32);

impl<'a> sum_tree::Dimension<'a, DiffHunkSummary> for HunkBufferEnd {
    fn add_summary(&mut self, summary: &'a DiffHunkSummary, buffer: &text::BufferSnapshot) {
        self.0 = summary.buffer_range.end.to_point(buffer).row;
    }

    fn from_summary(summary: &'a DiffHunkSummary, buffer: &text::BufferSnapshot) -> Self {
        HunkBufferEnd(summary.buffer_range.end.to_point(buffer).row)
    }
}

#[derive(Clone)]
pub struct BufferDiffSnapshot {
    tree: SumTree<DiffHunk<Anchor>>,
}

impl BufferDiffSnapshot {
    pub fn hunks_in_range<'a>(
        &'a self,
        query_row_range: Range<u32>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk<u32>> {
        self.tree.iter().filter_map(move |hunk| {
            let range = hunk.buffer_range.to_point(&buffer);

            if range.start.row <= query_row_range.end && query_row_range.start <= range.end.row {
                let end_row = if range.end.column > 0 {
                    range.end.row + 1
                } else {
                    range.end.row
                };

                Some(DiffHunk {
                    buffer_range: range.start.row..end_row,
                    head_byte_range: hunk.head_byte_range.clone(),
                })
            } else {
                None
            }
        })
    }

    #[cfg(test)]
    fn hunks<'a>(&'a self, text: &'a BufferSnapshot) -> impl 'a + Iterator<Item = DiffHunk<u32>> {
        self.hunks_in_range(0..u32::MAX, text)
    }
}

pub struct BufferDiff {
    snapshot: BufferDiffSnapshot,
}

impl BufferDiff {
    pub fn new(head_text: &Option<String>, buffer: &text::BufferSnapshot) -> BufferDiff {
        let mut instance = BufferDiff {
            snapshot: BufferDiffSnapshot {
                tree: SumTree::new(),
            },
        };

        if let Some(head_text) = head_text {
            instance.update(head_text, buffer);
        }

        instance
    }

    pub fn snapshot(&self) -> BufferDiffSnapshot {
        self.snapshot.clone()
    }

    pub fn update(&mut self, head_text: &str, buffer: &text::BufferSnapshot) {
        let mut tree = SumTree::new();

        let buffer_text = buffer.as_rope().to_string();
        let patch = Self::diff(&head_text, &buffer_text);

        if let Some(patch) = patch {
            let mut divergence = 0;
            for hunk_index in 0..patch.num_hunks() {
                let hunk = Self::process_patch_hunk(&patch, hunk_index, buffer, &mut divergence);
                tree.push(hunk, buffer);
            }
        }

        self.snapshot.tree = tree;
    }

    fn diff<'a>(head: &'a str, current: &'a str) -> Option<GitPatch<'a>> {
        let mut options = GitOptions::default();
        options.context_lines(0);

        let patch = GitPatch::from_buffers(
            head.as_bytes(),
            None,
            current.as_bytes(),
            None,
            Some(&mut options),
        );

        match patch {
            Ok(patch) => Some(patch),

            Err(err) => {
                log::error!("`GitPatch::from_buffers` failed: {}", err);
                None
            }
        }
    }

    fn process_patch_hunk<'a>(
        patch: &GitPatch<'a>,
        hunk_index: usize,
        buffer: &text::BufferSnapshot,
        buffer_row_divergence: &mut i64,
    ) -> DiffHunk<Anchor> {
        let line_item_count = patch.num_lines_in_hunk(hunk_index).unwrap();
        assert!(line_item_count > 0);

        let mut first_deletion_buffer_row: Option<u32> = None;
        let mut buffer_row_range: Option<Range<u32>> = None;
        let mut head_byte_range: Option<Range<usize>> = None;

        for line_index in 0..line_item_count {
            let line = patch.line_in_hunk(hunk_index, line_index).unwrap();
            let kind = line.origin_value();
            let content_offset = line.content_offset() as isize;
            let content_len = line.content().len() as isize;

            if kind == GitDiffLineType::Addition {
                *buffer_row_divergence += 1;
                let row = line.new_lineno().unwrap().saturating_sub(1);

                match &mut buffer_row_range {
                    Some(buffer_row_range) => buffer_row_range.end = row + 1,
                    None => buffer_row_range = Some(row..row + 1),
                }
            }

            if kind == GitDiffLineType::Deletion {
                *buffer_row_divergence -= 1;
                let end = content_offset + content_len;

                match &mut head_byte_range {
                    Some(head_byte_range) => head_byte_range.end = end as usize,
                    None => head_byte_range = Some(content_offset as usize..end as usize),
                }

                if first_deletion_buffer_row.is_none() {
                    let old_row = line.old_lineno().unwrap().saturating_sub(1);
                    let row = old_row as i64 + *buffer_row_divergence;
                    first_deletion_buffer_row = Some(row as u32);
                }
            }
        }

        //unwrap_or deletion without addition
        let buffer_row_range = buffer_row_range.unwrap_or_else(|| {
            //we cannot have an addition-less hunk without deletion(s) or else there would be no hunk
            let row = first_deletion_buffer_row.unwrap();
            row..row
        });

        //unwrap_or addition without deletion
        let head_byte_range = head_byte_range.unwrap_or(0..0);

        let start = Point::new(buffer_row_range.start, 0);
        let end = Point::new(buffer_row_range.end, 0);
        let buffer_range = buffer.anchor_before(start)..buffer.anchor_before(end);
        DiffHunk {
            buffer_range,
            head_byte_range,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use text::Buffer;
    use unindent::Unindent as _;

    #[gpui::test]
    fn test_buffer_diff_simple() {
        let head_text = "
            one
            two
            three
        "
        .unindent();

        let buffer_text = "
            one
            hello
            three
        "
        .unindent();

        let mut buffer = Buffer::new(0, 0, buffer_text);
        let diff = BufferDiff::new(&Some(head_text.clone()), &buffer);
        assert_hunks(&diff, &buffer, &head_text, &[(1..2, "two\n")]);

        buffer.edit([(0..0, "point five\n")]);
        assert_hunks(&diff, &buffer, &head_text, &[(2..3, "two\n")]);
    }

    #[track_caller]
    fn assert_hunks(
        diff: &BufferDiff,
        buffer: &BufferSnapshot,
        head_text: &str,
        expected_hunks: &[(Range<u32>, &str)],
    ) {
        let hunks = diff.snapshot.hunks(buffer).collect::<Vec<_>>();
        assert_eq!(
            hunks.len(),
            expected_hunks.len(),
            "actual hunks are {hunks:#?}"
        );

        let diff_iter = hunks.iter().enumerate();
        for ((index, hunk), (expected_range, expected_str)) in diff_iter.zip(expected_hunks) {
            assert_eq!(&hunk.buffer_range, expected_range, "for hunk {index}");
            assert_eq!(
                &head_text[hunk.head_byte_range.clone()],
                *expected_str,
                "for hunk {index}"
            );
        }
    }

    // use rand::rngs::StdRng;
    // #[gpui::test(iterations = 100)]
    // fn test_buffer_diff_random(mut rng: StdRng) {}
}
