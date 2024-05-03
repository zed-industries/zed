use rope::Rope;
use std::{iter, ops::Range};
use sum_tree::SumTree;
use text::{Anchor, BufferId, BufferSnapshot, OffsetRangeExt, Point};

pub use git2 as libgit;
use libgit::{DiffLineType as GitDiffLineType, DiffOptions as GitOptions, Patch as GitPatch};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiffHunkStatus {
    Added,
    Modified,
    Removed,
}

/// A diff hunk, representing a range of consequent lines in a singleton buffer, associated with a generic range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk<T> {
    /// E.g. a range in multibuffer, that has an excerpt added, singleton buffer for which has this diff hunk.
    /// Consider a singleton buffer with 10 lines, all of them are modified — so a corresponding diff hunk would have a range 0..10.
    /// And a multibuffer with the excerpt of lines 2-6 from the singleton buffer.
    /// If the multibuffer is searched for diff hunks, the associated range would be multibuffer rows, corresponding to rows 2..6 from the singleton buffer.
    /// But the hunk range would be 0..10, same for any other excerpts from the same singleton buffer.
    pub associated_range: Range<T>,
    /// Singleton buffer ID this hunk belongs to.
    pub buffer_id: BufferId,
    /// A consequent range of lines in the singleton buffer, that were changed and produced this diff hunk.
    pub buffer_range: Range<Anchor>,
    /// Original singleton buffer text before the change, that was instead of the `buffer_range`.
    pub diff_base_byte_range: Range<usize>,
}

impl DiffHunk<u32> {
    pub fn status(&self) -> DiffHunkStatus {
        if self.diff_base_byte_range.is_empty() {
            DiffHunkStatus::Added
        } else if self.associated_range.is_empty() {
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
            buffer_range: self.associated_range.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiffHunkSummary {
    buffer_range: Range<Anchor>,
}

impl sum_tree::Summary for DiffHunkSummary {
    type Context = text::BufferSnapshot;

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        self.buffer_range.start = self
            .buffer_range
            .start
            .min(&other.buffer_range.start, buffer);
        self.buffer_range.end = self.buffer_range.end.max(&other.buffer_range.end, buffer);
    }
}

#[derive(Debug, Clone)]
pub struct BufferDiff {
    last_buffer_version: Option<clock::Global>,
    tree: SumTree<DiffHunk<Anchor>>,
}

impl BufferDiff {
    pub fn new() -> BufferDiff {
        BufferDiff {
            last_buffer_version: None,
            tree: SumTree::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn hunks_in_row_range<'a>(
        &'a self,
        range: Range<u32>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk<u32>> {
        let start = buffer.anchor_before(Point::new(range.start, 0));
        let end = buffer.anchor_after(Point::new(range.end, 0));

        self.hunks_intersecting_range(start..end, buffer)
    }

    pub fn hunks_intersecting_range<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk<u32>> {
        let mut cursor = self.tree.filter::<_, DiffHunkSummary>(move |summary| {
            let before_start = summary.buffer_range.end.cmp(&range.start, buffer).is_lt();
            let after_end = summary.buffer_range.start.cmp(&range.end, buffer).is_gt();
            !before_start && !after_end
        });

        let anchor_iter = std::iter::from_fn(move || {
            cursor.next(buffer);
            cursor.item()
        })
        .flat_map(move |hunk| {
            [
                (
                    &hunk.associated_range.start,
                    hunk.diff_base_byte_range.start,
                ),
                (&hunk.associated_range.end, hunk.diff_base_byte_range.end),
            ]
            .into_iter()
        });

        let mut summaries = buffer.summaries_for_anchors_with_payload::<Point, _, _>(anchor_iter);
        iter::from_fn(move || {
            let (start_point, start_base) = summaries.next()?;
            let (mut end_point, end_base) = summaries.next()?;

            if end_point.column > 0 {
                end_point.row += 1;
                end_point.column = 0;
            }

            Some(DiffHunk {
                associated_range: start_point.row..end_point.row,
                diff_base_byte_range: start_base..end_base,
                buffer_range: buffer.anchor_before(start_point)..buffer.anchor_after(end_point),
                buffer_id: buffer.remote_id(),
            })
        })
    }

    pub fn hunks_intersecting_range_rev<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk<u32>> {
        let mut cursor = self.tree.filter::<_, DiffHunkSummary>(move |summary| {
            let before_start = summary.buffer_range.end.cmp(&range.start, buffer).is_lt();
            let after_end = summary.buffer_range.start.cmp(&range.end, buffer).is_gt();
            !before_start && !after_end
        });

        std::iter::from_fn(move || {
            cursor.prev(buffer);

            let hunk = cursor.item()?;
            let range = hunk.associated_range.to_point(buffer);
            let end_row = if range.end.column > 0 {
                range.end.row + 1
            } else {
                range.end.row
            };

            Some(DiffHunk {
                associated_range: range.start.row..end_row,
                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                buffer_range: hunk.buffer_range.clone(),
                buffer_id: hunk.buffer_id,
            })
        })
    }

    #[cfg(test)]
    fn clear(&mut self, buffer: &text::BufferSnapshot) {
        self.last_buffer_version = Some(buffer.version().clone());
        self.tree = SumTree::new();
    }

    pub async fn update(&mut self, diff_base: &Rope, buffer: &text::BufferSnapshot) {
        let mut tree = SumTree::new();

        let diff_base_text = diff_base.to_string();
        let buffer_text = buffer.as_rope().to_string();
        let patch = Self::diff(&diff_base_text, &buffer_text);

        if let Some(patch) = patch {
            let mut divergence = 0;
            for hunk_index in 0..patch.num_hunks() {
                let hunk = Self::process_patch_hunk(&patch, hunk_index, buffer, &mut divergence);
                tree.push(hunk, buffer);
            }
        }

        self.tree = tree;
        self.last_buffer_version = Some(buffer.version().clone());
    }

    #[cfg(test)]
    fn hunks<'a>(&'a self, text: &'a BufferSnapshot) -> impl 'a + Iterator<Item = DiffHunk<u32>> {
        let start = text.anchor_before(Point::new(0, 0));
        let end = text.anchor_after(Point::new(u32::MAX, u32::MAX));
        self.hunks_intersecting_range(start..end, text)
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

    fn process_patch_hunk(
        patch: &GitPatch<'_>,
        hunk_index: usize,
        buffer: &text::BufferSnapshot,
        buffer_row_divergence: &mut i64,
    ) -> DiffHunk<Anchor> {
        let line_item_count = patch.num_lines_in_hunk(hunk_index).unwrap();
        assert!(line_item_count > 0);

        let mut first_deletion_buffer_row: Option<u32> = None;
        let mut buffer_row_range: Option<Range<u32>> = None;
        let mut diff_base_byte_range: Option<Range<usize>> = None;

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
                let end = content_offset + content_len;

                match &mut diff_base_byte_range {
                    Some(head_byte_range) => head_byte_range.end = end as usize,
                    None => diff_base_byte_range = Some(content_offset as usize..end as usize),
                }

                if first_deletion_buffer_row.is_none() {
                    let old_row = line.old_lineno().unwrap().saturating_sub(1);
                    let row = old_row as i64 + *buffer_row_divergence;
                    first_deletion_buffer_row = Some(row as u32);
                }

                *buffer_row_divergence -= 1;
            }
        }

        //unwrap_or deletion without addition
        let buffer_row_range = buffer_row_range.unwrap_or_else(|| {
            //we cannot have an addition-less hunk without deletion(s) or else there would be no hunk
            let row = first_deletion_buffer_row.unwrap();
            row..row
        });

        //unwrap_or addition without deletion
        let diff_base_byte_range = diff_base_byte_range.unwrap_or(0..0);

        let start = Point::new(buffer_row_range.start, 0);
        let end = Point::new(buffer_row_range.end, 0);
        let buffer_range = buffer.anchor_before(start)..buffer.anchor_before(end);
        DiffHunk {
            associated_range: buffer_range.clone(),
            buffer_range,
            diff_base_byte_range,
            buffer_id: buffer.remote_id(),
        }
    }
}

/// Range (crossing new lines), old, new
#[cfg(any(test, feature = "test-support"))]
#[track_caller]
pub fn assert_hunks<Iter>(
    diff_hunks: Iter,
    buffer: &BufferSnapshot,
    diff_base: &str,
    expected_hunks: &[(Range<u32>, &str, &str)],
) where
    Iter: Iterator<Item = DiffHunk<u32>>,
{
    let actual_hunks = diff_hunks
        .map(|hunk| {
            (
                hunk.associated_range.clone(),
                &diff_base[hunk.diff_base_byte_range],
                buffer
                    .text_for_range(
                        Point::new(hunk.associated_range.start, 0)
                            ..Point::new(hunk.associated_range.end, 0),
                    )
                    .collect::<String>(),
            )
        })
        .collect::<Vec<_>>();

    let expected_hunks: Vec<_> = expected_hunks
        .iter()
        .map(|(r, s, h)| (r.clone(), *s, h.to_string()))
        .collect();

    assert_eq!(actual_hunks, expected_hunks);
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    use text::{Buffer, BufferId};
    use unindent::Unindent as _;

    #[test]
    fn test_buffer_diff_simple() {
        let diff_base = "
            one
            two
            three
        "
        .unindent();
        let diff_base_rope = Rope::from(diff_base.clone());

        let buffer_text = "
            one
            HELLO
            three
        "
        .unindent();

        let mut buffer = Buffer::new(0, BufferId::new(1).unwrap(), buffer_text);
        let mut diff = BufferDiff::new();
        smol::block_on(diff.update(&diff_base_rope, &buffer));
        assert_hunks(
            diff.hunks(&buffer),
            &buffer,
            &diff_base,
            &[(1..2, "two\n", "HELLO\n")],
        );

        buffer.edit([(0..0, "point five\n")]);
        smol::block_on(diff.update(&diff_base_rope, &buffer));
        assert_hunks(
            diff.hunks(&buffer),
            &buffer,
            &diff_base,
            &[(0..1, "", "point five\n"), (2..3, "two\n", "HELLO\n")],
        );

        diff.clear(&buffer);
        assert_hunks(diff.hunks(&buffer), &buffer, &diff_base, &[]);
    }

    #[test]
    fn test_buffer_diff_range() {
        let diff_base = "
            one
            two
            three
            four
            five
            six
            seven
            eight
            nine
            ten
        "
        .unindent();
        let diff_base_rope = Rope::from(diff_base.clone());

        let buffer_text = "
            A
            one
            B
            two
            C
            three
            HELLO
            four
            five
            SIXTEEN
            seven
            eight
            WORLD
            nine

            ten

        "
        .unindent();

        let buffer = Buffer::new(0, BufferId::new(1).unwrap(), buffer_text);
        let mut diff = BufferDiff::new();
        smol::block_on(diff.update(&diff_base_rope, &buffer));
        assert_eq!(diff.hunks(&buffer).count(), 8);

        assert_hunks(
            diff.hunks_in_row_range(7..12, &buffer),
            &buffer,
            &diff_base,
            &[
                (6..7, "", "HELLO\n"),
                (9..10, "six\n", "SIXTEEN\n"),
                (12..13, "", "WORLD\n"),
            ],
        );
    }
}
