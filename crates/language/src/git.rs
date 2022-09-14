use std::ops::Range;

use sum_tree::{Bias, SumTree};
use text::{Anchor, BufferSnapshot, OffsetRangeExt, Point, Rope, ToOffset, ToPoint};

pub use git2 as libgit;
use libgit::{
    DiffLine as GitDiffLine, DiffLineType as GitDiffLineType, DiffOptions as GitOptions,
    Patch as GitPatch,
};

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

struct HunkLineIter<'a, 'b> {
    patch: &'a GitPatch<'b>,
    hunk_index: usize,
    line_index: usize,
}

impl<'a, 'b> HunkLineIter<'a, 'b> {
    fn new(patch: &'a GitPatch<'b>, hunk_index: usize) -> Self {
        HunkLineIter {
            patch,
            hunk_index,
            line_index: 0,
        }
    }
}

impl<'a, 'b> std::iter::Iterator for HunkLineIter<'a, 'b> {
    type Item = GitDiffLine<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line_index >= self.patch.num_lines_in_hunk(self.hunk_index).unwrap() {
            return None;
        }

        let line_index = self.line_index;
        self.line_index += 1;
        Some(
            self.patch
                .line_in_hunk(self.hunk_index, line_index)
                .unwrap(),
        )
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
    last_update_version: clock::Global,
    snapshot: BufferDiffSnapshot,
}

impl BufferDiff {
    pub fn new(head_text: &Option<String>, buffer: &text::BufferSnapshot) -> BufferDiff {
        let mut tree = SumTree::new();

        if let Some(head_text) = head_text {
            let buffer_text = buffer.as_rope().to_string();
            let patch = Self::diff(&head_text, &buffer_text);

            if let Some(patch) = patch {
                for hunk_index in 0..patch.num_hunks() {
                    let hunk = Self::process_patch_hunk(&patch, hunk_index, buffer);
                    tree.push(hunk, buffer);
                }
            }
        }

        BufferDiff {
            last_update_version: buffer.version().clone(),
            snapshot: BufferDiffSnapshot { tree },
        }
    }

    pub fn snapshot(&self) -> BufferDiffSnapshot {
        self.snapshot.clone()
    }

    pub fn update(&mut self, head_text: &str, buffer: &text::BufferSnapshot) {
        let mut tree = SumTree::new();

        let buffer_text = buffer.as_rope().to_string();
        let patch = Self::diff(&head_text, &buffer_text);

        if let Some(patch) = patch {
            for hunk_index in 0..patch.num_hunks() {
                let hunk = Self::process_patch_hunk(&patch, hunk_index, buffer);
                tree.push(hunk, buffer);
            }
        }

        self.last_update_version = buffer.version().clone();
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

    fn group_edit_ranges(&self, buffer: &text::BufferSnapshot) -> Vec<Range<u32>> {
        const EXPAND_BY: u32 = 20;
        const COMBINE_DISTANCE: u32 = 5;

        // let mut cursor = self.snapshot.tree.cursor::<HunkBufferStart>();

        let mut ranges = Vec::<Range<u32>>::new();

        for edit in buffer.edits_since::<Point>(&self.last_update_version) {
            let buffer_start = edit.new.start.row.saturating_sub(EXPAND_BY);
            let buffer_end = (edit.new.end.row + EXPAND_BY).min(buffer.row_count());

            match ranges.last_mut() {
                Some(last_range) if last_range.end.abs_diff(buffer_end) <= COMBINE_DISTANCE => {
                    last_range.start = last_range.start.min(buffer_start);
                    last_range.end = last_range.end.max(buffer_end);
                }

                _ => ranges.push(buffer_start..buffer_end),
            }
        }

        ranges
    }

    fn process_patch_hunk<'a>(
        patch: &GitPatch<'a>,
        hunk_index: usize,
        buffer: &text::BufferSnapshot,
    ) -> DiffHunk<Anchor> {
        let line_item_count = patch.num_lines_in_hunk(hunk_index).unwrap();
        assert!(line_item_count > 0);

        let mut first_deletion_buffer_row: Option<u32> = None;
        let mut buffer_byte_range: Option<Range<usize>> = None;
        let mut head_byte_range: Option<Range<usize>> = None;

        for line_index in 0..line_item_count {
            let line = patch.line_in_hunk(hunk_index, line_index).unwrap();
            let kind = line.origin_value();
            let content_offset = line.content_offset() as isize;
            let content_len = line.content().len() as isize;

            match (kind, &mut buffer_byte_range, &mut head_byte_range) {
                (GitDiffLineType::Addition, None, _) => {
                    let end = content_offset + content_len;
                    buffer_byte_range = Some(content_offset as usize..end as usize);
                }

                (GitDiffLineType::Addition, Some(buffer_byte_range), _) => {
                    let end = content_offset + content_len;
                    buffer_byte_range.end = end as usize;
                }

                (GitDiffLineType::Deletion, _, None) => {
                    let end = content_offset + content_len;
                    head_byte_range = Some(content_offset as usize..end as usize);
                }

                (GitDiffLineType::Deletion, _, Some(head_byte_range)) => {
                    let end = content_offset + content_len;
                    head_byte_range.end = end as usize;
                }

                _ => {}
            }

            if kind == GitDiffLineType::Deletion && first_deletion_buffer_row.is_none() {
                //old_lineno is guarenteed to be Some for deletions
                //libgit gives us line numbers that are 1-indexed but also returns a 0 for some states
                let row = line.old_lineno().unwrap().saturating_sub(1);
                first_deletion_buffer_row = Some(row);
            }
        }

        //unwrap_or deletion without addition
        let buffer_byte_range = buffer_byte_range.unwrap_or_else(|| {
            //we cannot have an addition-less hunk without deletion(s) or else there would be no hunk
            let row = first_deletion_buffer_row.unwrap();
            let anchor = buffer.anchor_before(Point::new(row, 0));
            let offset = anchor.to_offset(buffer);
            offset..offset
        });

        //unwrap_or addition without deletion
        let head_byte_range = head_byte_range.unwrap_or(0..0);

        DiffHunk {
            buffer_range: buffer.anchor_before(buffer_byte_range.start)
                ..buffer.anchor_before(buffer_byte_range.end),
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
