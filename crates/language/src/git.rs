use std::ops::Range;

use sum_tree::{Bias, SumTree};
use text::{Anchor, BufferSnapshot, OffsetRangeExt, Point, Rope, ToPoint};

pub use git2 as libgit;
use libgit::{DiffOptions as GitOptions, Patch as GitPatch};

#[derive(Debug, Clone, Copy)]
pub enum DiffHunkStatus {
    Added,
    Modified,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk<T> {
    pub buffer_range: Range<T>,
    pub head_range: Range<u32>,
}

impl DiffHunk<u32> {
    pub fn status(&self) -> DiffHunkStatus {
        if self.head_range.is_empty() {
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
            head_range: self.head_range.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiffHunkSummary {
    buffer_range: Range<Anchor>,
    head_range: Range<u32>,
}

impl sum_tree::Summary for DiffHunkSummary {
    type Context = text::BufferSnapshot;

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.head_range.start = self.head_range.start.min(other.head_range.start);
        self.head_range.end = self.head_range.end.max(other.head_range.end);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HunkHeadEnd(u32);

impl<'a> sum_tree::Dimension<'a, DiffHunkSummary> for HunkHeadEnd {
    fn add_summary(&mut self, summary: &'a DiffHunkSummary, _: &text::BufferSnapshot) {
        self.0 = summary.head_range.end;
    }

    fn from_summary(summary: &'a DiffHunkSummary, _: &text::BufferSnapshot) -> Self {
        HunkHeadEnd(summary.head_range.end)
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

struct HunkIter<'a> {
    index: usize,
    patch: GitPatch<'a>,
}

impl<'a> HunkIter<'a> {
    fn diff(head: &'a [u8], current: &'a [u8]) -> Option<Self> {
        let mut options = GitOptions::default();
        options.context_lines(0);
        let patch = match GitPatch::from_buffers(head, None, current, None, Some(&mut options)) {
            Ok(patch) => patch,
            Err(_) => return None,
        };

        Some(HunkIter { index: 0, patch })
    }

    fn next(&mut self, buffer: &BufferSnapshot) -> Option<DiffHunk<Anchor>> {
        if self.index >= self.patch.num_hunks() {
            return None;
        }

        let (hunk, _) = match self.patch.hunk(self.index) {
            Ok(it) => it,
            Err(_) => return None,
        };

        let new_start = hunk.new_start() - 1;
        let new_end = new_start + hunk.new_lines();
        let start_anchor = buffer.anchor_at(Point::new(new_start, 0), Bias::Left);
        let end_anchor = buffer.anchor_at(Point::new(new_end, 0), Bias::Left);
        let buffer_range = start_anchor..end_anchor;

        //This is probably wrong? When does this trigger? Should buffer range also do this?
        let head_range = if hunk.old_start() == 0 {
            0..0
        } else {
            let old_start = hunk.old_start() - 1;
            let old_end = old_start + hunk.old_lines();
            old_start..old_end
        };

        self.index += 1;
        Some(DiffHunk {
            buffer_range,
            head_range,
        })
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
        println!("{} hunks overall", self.tree.iter().count());

        self.tree.iter().filter_map(move |hunk| {
            let range = hunk.buffer_range.to_point(&buffer);

            if range.start.row < query_row_range.end && query_row_range.start < range.end.row {
                let end_row = if range.end.column > 0 {
                    range.end.row + 1
                } else {
                    range.end.row
                };

                Some(DiffHunk {
                    buffer_range: range.start.row..end_row,
                    head_range: hunk.head_range.clone(),
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
        let hunks = if let Some(head_text) = head_text {
            let buffer_string = buffer.as_rope().to_string();
            let buffer_bytes = buffer_string.as_bytes();

            let iter = HunkIter::diff(head_text.as_bytes(), buffer_bytes);
            if let Some(mut iter) = iter {
                let mut hunks = SumTree::new();
                while let Some(hunk) = iter.next(buffer) {
                    hunks.push(hunk, buffer);
                }
                hunks
            } else {
                SumTree::new()
            }
        } else {
            SumTree::new()
        };

        BufferDiff {
            last_update_version: buffer.version().clone(),
            snapshot: BufferDiffSnapshot { tree: hunks },
        }
    }

    pub fn snapshot(&self) -> BufferDiffSnapshot {
        self.snapshot.clone()
    }

    pub fn update(&mut self, head: &Rope, buffer: &text::BufferSnapshot) {
        let expand_by = 20;
        let combine_distance = 5;

        struct EditRange {
            head_start: u32,
            head_end: u32,
            buffer_start: u32,
            buffer_end: u32,
        }

        let mut ranges = Vec::<EditRange>::new();

        for edit in buffer.edits_since::<Point>(&self.last_update_version) {
            //This bit is extremely wrong, this is not where these row lines should come from
            let head_start = edit.old.start.row.saturating_sub(expand_by);
            let head_end = (edit.old.end.row + expand_by).min(head.summary().lines.row + 1);

            let buffer_start = edit.new.start.row.saturating_sub(expand_by);
            let buffer_end = (edit.new.end.row + expand_by).min(buffer.row_count());

            if let Some(last_range) = ranges.last_mut() {
                let head_distance = last_range.head_end.abs_diff(head_end);
                let buffer_distance = last_range.buffer_end.abs_diff(buffer_end);

                if head_distance <= combine_distance || buffer_distance <= combine_distance {
                    last_range.head_start = last_range.head_start.min(head_start);
                    last_range.head_end = last_range.head_end.max(head_end);

                    last_range.buffer_start = last_range.buffer_start.min(buffer_start);
                    last_range.buffer_end = last_range.buffer_end.max(buffer_end);
                } else {
                    ranges.push(EditRange {
                        head_start,
                        head_end,
                        buffer_start,
                        buffer_end,
                    });
                }
            } else {
                ranges.push(EditRange {
                    head_start,
                    head_end,
                    buffer_start,
                    buffer_end,
                });
            }
        }

        self.last_update_version = buffer.version().clone();

        let mut new_hunks = SumTree::new();
        let mut cursor = self.snapshot.tree.cursor::<HunkHeadEnd>();

        for range in ranges {
            let head_range = range.head_start..range.head_end;
            let head_slice = head.slice_rows(head_range.clone());
            let head_str = head_slice.to_string();

            let buffer_range = range.buffer_start..range.buffer_end;
            let buffer_slice = buffer.as_rope().slice_rows(buffer_range.clone());
            let buffer_str = buffer_slice.to_string();

            println!("diffing head {:?}, buffer {:?}", head_range, buffer_range);

            let mut iter = match HunkIter::diff(head_str.as_bytes(), buffer_str.as_bytes()) {
                Some(iter) => iter,
                None => continue,
            };

            while let Some(hunk) = iter.next(buffer) {
                println!("hunk");
                let prefix = cursor.slice(&HunkHeadEnd(hunk.head_range.end), Bias::Right, buffer);
                println!("prefix len: {}", prefix.iter().count());
                new_hunks.extend(prefix.iter().cloned(), buffer);

                new_hunks.push(hunk.clone(), buffer);

                cursor.seek(&HunkHeadEnd(hunk.head_range.end), Bias::Right, buffer);
                println!("item: {:?}", cursor.item());
                if let Some(item) = cursor.item() {
                    if item.head_range.end <= hunk.head_range.end {
                        println!("skipping");
                        cursor.next(buffer);
                    }
                }
            }
        }

        new_hunks.extend(
            cursor
                .suffix(buffer)
                .iter()
                .map(|i| {
                    println!("extending with {i:?}");
                    i
                })
                .cloned(),
            buffer,
        );
        drop(cursor);

        self.snapshot.tree = new_hunks;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GitDiffEdit {
    Added(u32),
    Modified(u32),
    Removed(u32),
}

impl GitDiffEdit {
    pub fn line(self) -> u32 {
        use GitDiffEdit::*;

        match self {
            Added(line) | Modified(line) | Removed(line) => line,
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
        assert_eq!(
            diff.snapshot.hunks(&buffer).collect::<Vec<_>>(),
            &[DiffHunk {
                buffer_range: 1..2,
                head_range: 1..2
            }]
        );

        buffer.edit([(0..0, "point five\n")]);
        assert_eq!(
            diff.snapshot.hunks(&buffer).collect::<Vec<_>>(),
            &[DiffHunk {
                buffer_range: 2..3,
                head_range: 1..2
            }]
        );
    }

    // use rand::rngs::StdRng;
    // #[gpui::test(iterations = 100)]
    // fn test_buffer_diff_random(mut rng: StdRng) {}
}
