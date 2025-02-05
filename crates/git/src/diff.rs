use rope::Rope;
use std::{cmp, iter, ops::Range};
use sum_tree::SumTree;
use text::{Anchor, BufferSnapshot, OffsetRangeExt, Point};

pub use git2 as libgit;
use libgit::{DiffLineType as GitDiffLineType, DiffOptions as GitOptions, Patch as GitPatch};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiffHunkStatus {
    Added,
    Modified,
    Removed,
}

/// A diff hunk resolved to rows in the buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk {
    /// The buffer range, expressed in terms of rows.
    pub row_range: Range<u32>,
    /// The range in the buffer to which this hunk corresponds.
    pub buffer_range: Range<Anchor>,
    /// The range in the buffer's diff base text to which this hunk corresponds.
    pub diff_base_byte_range: Range<usize>,
}

/// We store [`InternalDiffHunk`]s internally so we don't need to store the additional row range.
#[derive(Debug, Clone, PartialEq, Eq)]
struct InternalDiffHunk {
    buffer_range: Range<Anchor>,
    diff_base_byte_range: Range<usize>,
}

impl sum_tree::Item for InternalDiffHunk {
    type Summary = DiffHunkSummary;

    fn summary(&self, _cx: &text::BufferSnapshot) -> Self::Summary {
        DiffHunkSummary {
            buffer_range: self.buffer_range.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiffHunkSummary {
    buffer_range: Range<Anchor>,
}

impl sum_tree::Summary for DiffHunkSummary {
    type Context = text::BufferSnapshot;

    fn zero(_cx: &Self::Context) -> Self {
        Default::default()
    }

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
    tree: SumTree<InternalDiffHunk>,
}

impl BufferDiff {
    pub fn new(buffer: &BufferSnapshot) -> BufferDiff {
        BufferDiff {
            tree: SumTree::new(buffer),
        }
    }

    pub fn new_with_single_insertion(buffer: &BufferSnapshot) -> Self {
        Self {
            tree: SumTree::from_item(
                InternalDiffHunk {
                    buffer_range: Anchor::MIN..Anchor::MAX,
                    diff_base_byte_range: 0..0,
                },
                buffer,
            ),
        }
    }

    pub fn build(diff_base: Option<&str>, buffer: &text::BufferSnapshot) -> Self {
        let mut tree = SumTree::new(buffer);

        if let Some(diff_base) = diff_base {
            let buffer_text = buffer.as_rope().to_string();
            let patch = Self::diff(diff_base, &buffer_text);

            // A common case in Zed is that the empty buffer is represented as just a newline,
            // but if we just compute a naive diff you get a "preserved" line in the middle,
            // which is a bit odd.
            if buffer_text == "\n" && diff_base.ends_with("\n") && diff_base.len() > 1 {
                tree.push(
                    InternalDiffHunk {
                        buffer_range: buffer.anchor_before(0)..buffer.anchor_before(0),
                        diff_base_byte_range: 0..diff_base.len() - 1,
                    },
                    buffer,
                );
                return Self { tree };
            }

            if let Some(patch) = patch {
                let mut divergence = 0;
                for hunk_index in 0..patch.num_hunks() {
                    let hunk =
                        Self::process_patch_hunk(&patch, hunk_index, buffer, &mut divergence);
                    tree.push(hunk, buffer);
                }
            }
        }

        Self { tree }
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn hunks_in_row_range<'a>(
        &'a self,
        range: Range<u32>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        let start = buffer.anchor_before(Point::new(range.start, 0));
        let end = buffer.anchor_after(Point::new(range.end, 0));

        self.hunks_intersecting_range(start..end, buffer)
    }

    pub fn hunks_intersecting_range<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        let range = range.to_offset(buffer);

        let mut cursor = self
            .tree
            .filter::<_, DiffHunkSummary>(buffer, move |summary| {
                let summary_range = summary.buffer_range.to_offset(buffer);
                let before_start = summary_range.end < range.start;
                let after_end = summary_range.start > range.end;
                !before_start && !after_end
            });

        let anchor_iter = iter::from_fn(move || {
            cursor.next(buffer);
            cursor.item()
        })
        .flat_map(move |hunk| {
            [
                (
                    &hunk.buffer_range.start,
                    (hunk.buffer_range.start, hunk.diff_base_byte_range.start),
                ),
                (
                    &hunk.buffer_range.end,
                    (hunk.buffer_range.end, hunk.diff_base_byte_range.end),
                ),
            ]
        });

        let mut summaries = buffer.summaries_for_anchors_with_payload::<Point, _, _>(anchor_iter);
        iter::from_fn(move || loop {
            let (start_point, (start_anchor, start_base)) = summaries.next()?;
            let (mut end_point, (mut end_anchor, end_base)) = summaries.next()?;

            if !start_anchor.is_valid(buffer) {
                continue;
            }

            if end_point.column > 0 {
                end_point.row += 1;
                end_point.column = 0;
                end_anchor = buffer.anchor_before(end_point);
            }

            return Some(DiffHunk {
                row_range: start_point.row..end_point.row,
                diff_base_byte_range: start_base..end_base,
                buffer_range: start_anchor..end_anchor,
            });
        })
    }

    pub fn hunks_intersecting_range_rev<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        let mut cursor = self
            .tree
            .filter::<_, DiffHunkSummary>(buffer, move |summary| {
                let before_start = summary.buffer_range.end.cmp(&range.start, buffer).is_lt();
                let after_end = summary.buffer_range.start.cmp(&range.end, buffer).is_gt();
                !before_start && !after_end
            });

        iter::from_fn(move || {
            cursor.prev(buffer);

            let hunk = cursor.item()?;
            let range = hunk.buffer_range.to_point(buffer);
            let end_row = if range.end.column > 0 {
                range.end.row + 1
            } else {
                range.end.row
            };

            Some(DiffHunk {
                row_range: range.start.row..end_row,
                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                buffer_range: hunk.buffer_range.clone(),
            })
        })
    }

    pub fn compare(&self, old: &Self, new_snapshot: &BufferSnapshot) -> Option<Range<Anchor>> {
        let mut new_cursor = self.tree.cursor::<()>(new_snapshot);
        let mut old_cursor = old.tree.cursor::<()>(new_snapshot);
        old_cursor.next(new_snapshot);
        new_cursor.next(new_snapshot);
        let mut start = None;
        let mut end = None;

        loop {
            match (new_cursor.item(), old_cursor.item()) {
                (Some(new_hunk), Some(old_hunk)) => {
                    match new_hunk
                        .buffer_range
                        .start
                        .cmp(&old_hunk.buffer_range.start, new_snapshot)
                    {
                        cmp::Ordering::Less => {
                            start.get_or_insert(new_hunk.buffer_range.start);
                            end.replace(new_hunk.buffer_range.end);
                            new_cursor.next(new_snapshot);
                        }
                        cmp::Ordering::Equal => {
                            if new_hunk != old_hunk {
                                start.get_or_insert(new_hunk.buffer_range.start);
                                if old_hunk
                                    .buffer_range
                                    .end
                                    .cmp(&new_hunk.buffer_range.end, new_snapshot)
                                    .is_ge()
                                {
                                    end.replace(old_hunk.buffer_range.end);
                                } else {
                                    end.replace(new_hunk.buffer_range.end);
                                }
                            }

                            new_cursor.next(new_snapshot);
                            old_cursor.next(new_snapshot);
                        }
                        cmp::Ordering::Greater => {
                            start.get_or_insert(old_hunk.buffer_range.start);
                            end.replace(old_hunk.buffer_range.end);
                            old_cursor.next(new_snapshot);
                        }
                    }
                }
                (Some(new_hunk), None) => {
                    start.get_or_insert(new_hunk.buffer_range.start);
                    end.replace(new_hunk.buffer_range.end);
                    new_cursor.next(new_snapshot);
                }
                (None, Some(old_hunk)) => {
                    start.get_or_insert(old_hunk.buffer_range.start);
                    end.replace(old_hunk.buffer_range.end);
                    old_cursor.next(new_snapshot);
                }
                (None, None) => break,
            }
        }

        start.zip(end).map(|(start, end)| start..end)
    }

    #[cfg(test)]
    fn clear(&mut self, buffer: &text::BufferSnapshot) {
        self.tree = SumTree::new(buffer);
    }

    pub fn update(&mut self, diff_base: &Rope, buffer: &text::BufferSnapshot) {
        *self = Self::build(Some(&diff_base.to_string()), buffer);
    }

    #[cfg(test)]
    fn hunks<'a>(&'a self, text: &'a BufferSnapshot) -> impl 'a + Iterator<Item = DiffHunk> {
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
    ) -> InternalDiffHunk {
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
        InternalDiffHunk {
            buffer_range,
            diff_base_byte_range,
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
    Iter: Iterator<Item = DiffHunk>,
{
    let actual_hunks = diff_hunks
        .map(|hunk| {
            (
                hunk.row_range.clone(),
                &diff_base[hunk.diff_base_byte_range],
                buffer
                    .text_for_range(
                        Point::new(hunk.row_range.start, 0)..Point::new(hunk.row_range.end, 0),
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
        let mut diff = BufferDiff::new(&buffer);
        diff.update(&diff_base_rope, &buffer);
        assert_hunks(
            diff.hunks(&buffer),
            &buffer,
            &diff_base,
            &[(1..2, "two\n", "HELLO\n")],
        );

        buffer.edit([(0..0, "point five\n")]);
        diff.update(&diff_base_rope, &buffer);
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
        let mut diff = BufferDiff::new(&buffer);
        diff.update(&diff_base_rope, &buffer);
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

    #[test]
    fn test_buffer_diff_compare() {
        let base_text = "
            zero
            one
            two
            three
            four
            five
            six
            seven
            eight
            nine
        "
        .unindent();

        let buffer_text_1 = "
            one
            three
            four
            five
            SIX
            seven
            eight
            NINE
        "
        .unindent();

        let mut buffer = Buffer::new(0, BufferId::new(1).unwrap(), buffer_text_1);

        let empty_diff = BufferDiff::new(&buffer);
        let diff_1 = BufferDiff::build(Some(&base_text), &buffer);
        let range = diff_1.compare(&empty_diff, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(0, 0)..Point::new(8, 0));

        // Edit does not affect the diff.
        buffer.edit_via_marked_text(
            &"
                one
                three
                four
                five
                «SIX.5»
                seven
                eight
                NINE
            "
            .unindent(),
        );
        let diff_2 = BufferDiff::build(Some(&base_text), &buffer);
        assert_eq!(None, diff_2.compare(&diff_1, &buffer));

        // Edit turns a deletion hunk into a modification.
        buffer.edit_via_marked_text(
            &"
                one
                «THREE»
                four
                five
                SIX.5
                seven
                eight
                NINE
            "
            .unindent(),
        );
        let diff_3 = BufferDiff::build(Some(&base_text), &buffer);
        let range = diff_3.compare(&diff_2, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(1, 0)..Point::new(2, 0));

        // Edit turns a modification hunk into a deletion.
        buffer.edit_via_marked_text(
            &"
                one
                THREE
                four
                five«»
                seven
                eight
                NINE
            "
            .unindent(),
        );
        let diff_4 = BufferDiff::build(Some(&base_text), &buffer);
        let range = diff_4.compare(&diff_3, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(3, 4)..Point::new(4, 0));

        // Edit introduces a new insertion hunk.
        buffer.edit_via_marked_text(
            &"
                one
                THREE
                four«
                FOUR.5
                »five
                seven
                eight
                NINE
            "
            .unindent(),
        );
        let diff_5 = BufferDiff::build(Some(&base_text), &buffer);
        let range = diff_5.compare(&diff_4, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(3, 0)..Point::new(4, 0));

        // Edit removes a hunk.
        buffer.edit_via_marked_text(
            &"
                one
                THREE
                four
                FOUR.5
                five
                seven
                eight
                «nine»
            "
            .unindent(),
        );
        let diff_6 = BufferDiff::build(Some(&base_text), &buffer);
        let range = diff_6.compare(&diff_5, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(7, 0)..Point::new(8, 0));
    }
}
