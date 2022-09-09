use std::ops::Range;

use sum_tree::{Bias, SumTree};
use text::{Anchor, BufferSnapshot, Point, Rope};

pub use git2 as libgit;
use libgit::{DiffOptions as GitOptions, Patch as GitPatch};

#[derive(Debug, Clone, Copy)]
pub enum DiffHunkStatus {
    Added,
    Modified,
    Removed,
}

#[derive(Debug, Clone)]
pub struct DiffHunk<T> {
    pub buffer_range: Range<T>,
    pub head_range: Range<usize>,
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
            head_range: self.head_range.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiffHunkSummary {
    head_range: Range<usize>,
}

impl sum_tree::Summary for DiffHunkSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.head_range.start = self.head_range.start.min(other.head_range.start);
        self.head_range.end = self.head_range.end.max(other.head_range.end);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct HunkHeadEnd(usize);

impl<'a> sum_tree::Dimension<'a, DiffHunkSummary> for HunkHeadEnd {
    fn add_summary(&mut self, summary: &'a DiffHunkSummary, _: &()) {
        self.0 = summary.head_range.end;
    }

    fn from_summary(summary: &'a DiffHunkSummary, _: &()) -> Self {
        HunkHeadEnd(summary.head_range.end)
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
            let old_start = hunk.old_start() as usize - 1;
            let old_end = old_start + hunk.old_lines() as usize;
            old_start..old_end
        };

        self.index += 1;
        Some(DiffHunk {
            buffer_range,
            head_range,
        })
    }
}

pub struct BufferDiff {
    last_update_version: clock::Global,
    hunks: SumTree<DiffHunk<Anchor>>,
}

impl BufferDiff {
    pub fn new(head_text: &Option<String>, buffer: &text::BufferSnapshot) -> BufferDiff {
        let hunks = if let Some(head_text) = head_text {
            let buffer_string = buffer.as_rope().to_string();
            let buffer_bytes = buffer_string.as_bytes();
            let iter = HunkIter::diff(head_text.as_bytes(), buffer_bytes);
            if let Some(mut iter) = iter {
                println!("some iter");
                let mut hunks = SumTree::new();
                while let Some(hunk) = iter.next(buffer) {
                    println!("hunk");
                    hunks.push(hunk, &());
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
            hunks,
        }
    }

    pub fn hunks(&self) -> &SumTree<DiffHunk<Anchor>> {
        &self.hunks
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
        let mut cursor = self.hunks.cursor::<HunkHeadEnd>();

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
                let prefix = cursor.slice(&HunkHeadEnd(hunk.head_range.end), Bias::Right, &());
                println!("prefix len: {}", prefix.iter().count());
                new_hunks.extend(prefix.iter().cloned(), &());

                new_hunks.push(hunk.clone(), &());

                cursor.seek(&HunkHeadEnd(hunk.head_range.end), Bias::Right, &());
                println!("item: {:?}", cursor.item());
                if let Some(item) = cursor.item() {
                    if item.head_range.end <= hunk.head_range.end {
                        println!("skipping");
                        cursor.next(&());
                    }
                }
            }
        }

        new_hunks.extend(
            cursor
                .suffix(&())
                .iter()
                .map(|i| {
                    println!("extending with {i:?}");
                    i
                })
                .cloned(),
            &(),
        );
        drop(cursor);

        self.hunks = new_hunks;
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
