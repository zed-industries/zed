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
            buffer_range: self.buffer_range.clone(),
            head_range: self.head_range.clone(),
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
        // println!("{} hunks overall", self.tree.iter().count());

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
        let mut instance = BufferDiff {
            last_update_version: buffer.version().clone(),
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
        let buffer_string = buffer.as_rope().to_string();
        let buffer_bytes = buffer_string.as_bytes();

        let mut options = GitOptions::default();
        options.context_lines(0);
        let patch = match GitPatch::from_buffers(
            head_text.as_bytes(),
            None,
            buffer_bytes,
            None,
            Some(&mut options),
        ) {
            Ok(patch) => patch,
            Err(_) => todo!("This needs to be handled"),
        };

        let mut hunks = SumTree::<DiffHunk<Anchor>>::new();
        let mut delta = 0i64;
        for hunk_index in 0..patch.num_hunks() {
            for line_index in 0..patch.num_lines_in_hunk(hunk_index).unwrap() {
                let line = patch.line_in_hunk(hunk_index, line_index).unwrap();

                let hunk = match line.origin_value() {
                    libgit::DiffLineType::Addition => {
                        let buffer_start = line.content_offset();
                        let buffer_end = buffer_start as usize + line.content().len();
                        let head_offset = (buffer_start - delta) as usize;
                        delta += line.content().len() as i64;
                        DiffHunk {
                            buffer_range: buffer.anchor_before(buffer_start as usize)
                                ..buffer.anchor_after(buffer_end),
                            head_range: head_offset..head_offset,
                        }
                    }

                    libgit::DiffLineType::Deletion => {
                        let head_start = line.content_offset();
                        let head_end = head_start as usize + line.content().len();
                        let buffer_offset = (head_start + delta) as usize;
                        delta -= line.content().len() as i64;
                        DiffHunk {
                            buffer_range: buffer.anchor_before(buffer_offset)
                                ..buffer.anchor_after(buffer_offset),
                            head_range: (head_start as usize)..head_end,
                        }
                    }

                    libgit::DiffLineType::AddEOFNL => todo!(),
                    libgit::DiffLineType::ContextEOFNL => todo!(),
                    libgit::DiffLineType::DeleteEOFNL => todo!(),

                    libgit::DiffLineType::FileHeader => continue,
                    libgit::DiffLineType::HunkHeader => continue,
                    libgit::DiffLineType::Binary => continue,

                    //We specifically tell git to not give us context lines
                    libgit::DiffLineType::Context => unreachable!(),
                };

                let mut combined = false;
                hunks.update_last(
                    |last_hunk| {
                        if last_hunk.head_range.end == hunk.head_range.start {
                            last_hunk.head_range.end = hunk.head_range.end;
                            last_hunk.buffer_range.end = hunk.buffer_range.end;
                            combined = true;
                        }
                    },
                    buffer,
                );
                if !combined {
                    hunks.push(hunk, buffer);
                }
            }
        }

        self.snapshot.tree = hunks;
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
                &head_text[hunk.head_range.clone()],
                *expected_str,
                "for hunk {index}"
            );
        }
    }

    // use rand::rngs::StdRng;
    // #[gpui::test(iterations = 100)]
    // fn test_buffer_diff_random(mut rng: StdRng) {}
}
