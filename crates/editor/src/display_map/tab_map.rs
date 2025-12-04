use super::{
    Highlights,
    fold_map::{self, Chunk, FoldChunks, FoldEdit, FoldPoint, FoldSnapshot},
};

use language::Point;
use multi_buffer::MultiBufferSnapshot;
use std::{cmp, mem, num::NonZeroU32, ops::Range};
use sum_tree::Bias;

const MAX_EXPANSION_COLUMN: u32 = 256;

// Handles a tab width <= 128
const SPACES: &[u8; rope::Chunk::MASK_BITS] = &[b' '; _];
const MAX_TABS: NonZeroU32 = NonZeroU32::new(SPACES.len() as u32).unwrap();

/// Keeps track of hard tabs in a text buffer.
///
/// See the [`display_map` module documentation](crate::display_map) for more information.
pub struct TabMap(TabSnapshot);

impl TabMap {
    pub fn new(fold_snapshot: FoldSnapshot, tab_size: NonZeroU32) -> (Self, TabSnapshot) {
        let snapshot = TabSnapshot {
            fold_snapshot,
            tab_size: tab_size.min(MAX_TABS),
            max_expansion_column: MAX_EXPANSION_COLUMN,
            version: 0,
        };
        (Self(snapshot.clone()), snapshot)
    }

    #[cfg(test)]
    pub fn set_max_expansion_column(&mut self, column: u32) -> TabSnapshot {
        self.0.max_expansion_column = column;
        self.0.clone()
    }

    pub fn sync(
        &mut self,
        fold_snapshot: FoldSnapshot,
        mut fold_edits: Vec<FoldEdit>,
        tab_size: NonZeroU32,
    ) -> (TabSnapshot, Vec<TabEdit>) {
        let old_snapshot = &mut self.0;
        let mut new_snapshot = TabSnapshot {
            fold_snapshot,
            tab_size: tab_size.min(MAX_TABS),
            max_expansion_column: old_snapshot.max_expansion_column,
            version: old_snapshot.version,
        };

        if old_snapshot.fold_snapshot.version != new_snapshot.fold_snapshot.version {
            new_snapshot.version += 1;
        }

        let tab_edits = if old_snapshot.tab_size == new_snapshot.tab_size {
            // Expand each edit to include the next tab on the same line as the edit,
            // and any subsequent tabs on that line that moved across the tab expansion
            // boundary.
            for fold_edit in &mut fold_edits {
                let old_end = fold_edit.old.end.to_point(&old_snapshot.fold_snapshot);
                let old_end_row_successor_offset = cmp::min(
                    FoldPoint::new(old_end.row() + 1, 0),
                    old_snapshot.fold_snapshot.max_point(),
                )
                .to_offset(&old_snapshot.fold_snapshot);
                let new_end = fold_edit.new.end.to_point(&new_snapshot.fold_snapshot);

                let mut offset_from_edit = 0;
                let mut first_tab_offset = None;
                let mut last_tab_with_changed_expansion_offset = None;
                'outer: for chunk in old_snapshot.fold_snapshot.chunks(
                    fold_edit.old.end..old_end_row_successor_offset,
                    false,
                    Highlights::default(),
                ) {
                    // todo(performance use tabs bitmask)
                    for (ix, _) in chunk.text.match_indices('\t') {
                        let offset_from_edit = offset_from_edit + (ix as u32);
                        if first_tab_offset.is_none() {
                            first_tab_offset = Some(offset_from_edit);
                        }

                        let old_column = old_end.column() + offset_from_edit;
                        let new_column = new_end.column() + offset_from_edit;
                        let was_expanded = old_column < old_snapshot.max_expansion_column;
                        let is_expanded = new_column < new_snapshot.max_expansion_column;
                        if was_expanded != is_expanded {
                            last_tab_with_changed_expansion_offset = Some(offset_from_edit);
                        } else if !was_expanded && !is_expanded {
                            break 'outer;
                        }
                    }

                    offset_from_edit += chunk.text.len() as u32;
                    if old_end.column() + offset_from_edit >= old_snapshot.max_expansion_column
                        && new_end.column() + offset_from_edit >= new_snapshot.max_expansion_column
                    {
                        break;
                    }
                }

                if let Some(offset) = last_tab_with_changed_expansion_offset.or(first_tab_offset) {
                    fold_edit.old.end.0 += offset as usize + 1;
                    fold_edit.new.end.0 += offset as usize + 1;
                }
            }

            let _old_alloc_ptr = fold_edits.as_ptr();
            // Combine any edits that overlap due to the expansion.
            let mut fold_edits = fold_edits.into_iter();
            if let Some(mut first_edit) = fold_edits.next() {
                // This code relies on reusing allocations from the Vec<_> - at the time of writing .flatten() prevents them.
                #[allow(clippy::filter_map_identity)]
                let mut v: Vec<_> = fold_edits
                    .scan(&mut first_edit, |state, edit| {
                        if state.old.end >= edit.old.start {
                            state.old.end = edit.old.end;
                            state.new.end = edit.new.end;
                            Some(None) // Skip this edit, it's merged
                        } else {
                            let new_state = edit;
                            let result = Some(Some(state.clone())); // Yield the previous edit
                            **state = new_state;
                            result
                        }
                    })
                    .filter_map(|x| x)
                    .collect();
                v.push(first_edit);
                debug_assert_eq!(v.as_ptr(), _old_alloc_ptr, "Fold edits were reallocated");
                v.into_iter()
                    .map(|fold_edit| {
                        let old_start = fold_edit.old.start.to_point(&old_snapshot.fold_snapshot);
                        let old_end = fold_edit.old.end.to_point(&old_snapshot.fold_snapshot);
                        let new_start = fold_edit.new.start.to_point(&new_snapshot.fold_snapshot);
                        let new_end = fold_edit.new.end.to_point(&new_snapshot.fold_snapshot);
                        TabEdit {
                            old: old_snapshot.fold_point_to_tab_point(old_start)
                                ..old_snapshot.fold_point_to_tab_point(old_end),
                            new: new_snapshot.fold_point_to_tab_point(new_start)
                                ..new_snapshot.fold_point_to_tab_point(new_end),
                        }
                    })
                    .collect()
            } else {
                vec![]
            }
        } else {
            new_snapshot.version += 1;
            vec![TabEdit {
                old: TabPoint::zero()..old_snapshot.max_point(),
                new: TabPoint::zero()..new_snapshot.max_point(),
            }]
        };
        *old_snapshot = new_snapshot;
        (old_snapshot.clone(), tab_edits)
    }
}

#[derive(Clone)]
pub struct TabSnapshot {
    pub fold_snapshot: FoldSnapshot,
    pub tab_size: NonZeroU32,
    pub max_expansion_column: u32,
    pub version: usize,
}

impl std::ops::Deref for TabSnapshot {
    type Target = FoldSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.fold_snapshot
    }
}

impl TabSnapshot {
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        &self.fold_snapshot.inlay_snapshot.buffer
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let max_point = self.max_point();
        if row < max_point.row() {
            self.fold_point_to_tab_point(FoldPoint::new(row, self.fold_snapshot.line_len(row)))
                .0
                .column
        } else {
            max_point.column()
        }
    }

    pub fn text_summary(&self) -> TextSummary {
        self.text_summary_for_range(TabPoint::zero()..self.max_point())
    }

    pub fn text_summary_for_range(&self, range: Range<TabPoint>) -> TextSummary {
        let input_start = self.tab_point_to_fold_point(range.start, Bias::Left).0;
        let input_end = self.tab_point_to_fold_point(range.end, Bias::Right).0;
        let input_summary = self
            .fold_snapshot
            .text_summary_for_range(input_start..input_end);

        let line_end = if range.start.row() == range.end.row() {
            range.end
        } else {
            self.max_point()
        };
        let first_line_chars = self
            .chunks(range.start..line_end, false, Highlights::default())
            .flat_map(|chunk| chunk.text.chars())
            .take_while(|&c| c != '\n')
            .count() as u32;

        let last_line_chars = if range.start.row() == range.end.row() {
            first_line_chars
        } else {
            self.chunks(
                TabPoint::new(range.end.row(), 0)..range.end,
                false,
                Highlights::default(),
            )
            .flat_map(|chunk| chunk.text.chars())
            .count() as u32
        };

        TextSummary {
            lines: range.end.0 - range.start.0,
            first_line_chars,
            last_line_chars,
            longest_row: input_summary.longest_row,
            longest_row_chars: input_summary.longest_row_chars,
        }
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<TabPoint>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> TabChunks<'a> {
        let (input_start, expanded_char_column, to_next_stop) =
            self.tab_point_to_fold_point(range.start, Bias::Left);
        let input_column = input_start.column();
        let input_start = input_start.to_offset(&self.fold_snapshot);
        let input_end = self
            .tab_point_to_fold_point(range.end, Bias::Right)
            .0
            .to_offset(&self.fold_snapshot);
        let to_next_stop = if range.start.0 + Point::new(0, to_next_stop) > range.end.0 {
            range.end.column() - range.start.column()
        } else {
            to_next_stop
        };

        TabChunks {
            snapshot: self,
            fold_chunks: self.fold_snapshot.chunks(
                input_start..input_end,
                language_aware,
                highlights,
            ),
            input_column,
            column: expanded_char_column,
            max_expansion_column: self.max_expansion_column,
            output_position: range.start.0,
            max_output_position: range.end.0,
            tab_size: self.tab_size,
            chunk: Chunk {
                text: unsafe { std::str::from_utf8_unchecked(&SPACES[..to_next_stop as usize]) },
                is_tab: true,
                ..Default::default()
            },
            inside_leading_tab: to_next_stop > 0,
        }
    }

    pub fn rows(&self, row: u32) -> fold_map::FoldRows<'_> {
        self.fold_snapshot.row_infos(row)
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(
            TabPoint::zero()..self.max_point(),
            false,
            Highlights::default(),
        )
        .map(|chunk| chunk.text)
        .collect()
    }

    pub fn max_point(&self) -> TabPoint {
        self.fold_point_to_tab_point(self.fold_snapshot.max_point())
    }

    pub fn clip_point(&self, point: TabPoint, bias: Bias) -> TabPoint {
        self.fold_point_to_tab_point(
            self.fold_snapshot
                .clip_point(self.tab_point_to_fold_point(point, bias).0, bias),
        )
    }

    pub fn fold_point_to_tab_point(&self, input: FoldPoint) -> TabPoint {
        let chunks = self.fold_snapshot.chunks_at(FoldPoint::new(input.row(), 0));
        let tab_cursor = TabStopCursor::new(chunks);
        let expanded = self.expand_tabs(tab_cursor, input.column());
        TabPoint::new(input.row(), expanded)
    }

    pub fn tab_point_cursor(&self) -> TabPointCursor<'_> {
        TabPointCursor { this: self }
    }

    pub fn tab_point_to_fold_point(&self, output: TabPoint, bias: Bias) -> (FoldPoint, u32, u32) {
        let chunks = self
            .fold_snapshot
            .chunks_at(FoldPoint::new(output.row(), 0));

        let tab_cursor = TabStopCursor::new(chunks);
        let expanded = output.column();
        let (collapsed, expanded_char_column, to_next_stop) =
            self.collapse_tabs(tab_cursor, expanded, bias);

        (
            FoldPoint::new(output.row(), collapsed),
            expanded_char_column,
            to_next_stop,
        )
    }

    pub fn point_to_tab_point(&self, point: Point, bias: Bias) -> TabPoint {
        let inlay_point = self.fold_snapshot.inlay_snapshot.to_inlay_point(point);
        let fold_point = self.fold_snapshot.to_fold_point(inlay_point, bias);
        self.fold_point_to_tab_point(fold_point)
    }

    pub fn tab_point_to_point(&self, point: TabPoint, bias: Bias) -> Point {
        let fold_point = self.tab_point_to_fold_point(point, bias).0;
        let inlay_point = fold_point.to_inlay_point(&self.fold_snapshot);
        self.fold_snapshot
            .inlay_snapshot
            .to_buffer_point(inlay_point)
    }

    fn expand_tabs<'a, I>(&self, mut cursor: TabStopCursor<'a, I>, column: u32) -> u32
    where
        I: Iterator<Item = Chunk<'a>>,
    {
        let tab_size = self.tab_size.get();

        let end_column = column.min(self.max_expansion_column);
        let mut seek_target = end_column;
        let mut tab_count = 0;
        let mut expanded_tab_len = 0;

        while let Some(tab_stop) = cursor.seek(seek_target) {
            let expanded_chars_old = tab_stop.char_offset + expanded_tab_len - tab_count;
            let tab_len = tab_size - ((expanded_chars_old - 1) % tab_size);
            tab_count += 1;
            expanded_tab_len += tab_len;

            seek_target = end_column - cursor.byte_offset;
        }

        let left_over_char_bytes = if !cursor.is_char_boundary() {
            cursor.bytes_until_next_char().unwrap_or(0) as u32
        } else {
            0
        };

        let collapsed_bytes = cursor.byte_offset() + left_over_char_bytes;
        let expanded_bytes =
            cursor.byte_offset() + expanded_tab_len - tab_count + left_over_char_bytes;

        expanded_bytes + column.saturating_sub(collapsed_bytes)
    }

    fn collapse_tabs<'a, I>(
        &self,
        mut cursor: TabStopCursor<'a, I>,
        column: u32,
        bias: Bias,
    ) -> (u32, u32, u32)
    where
        I: Iterator<Item = Chunk<'a>>,
    {
        let tab_size = self.tab_size.get();
        let mut collapsed_column = column;
        let mut seek_target = column.min(self.max_expansion_column);
        let mut tab_count = 0;
        let mut expanded_tab_len = 0;

        while let Some(tab_stop) = cursor.seek(seek_target) {
            // Calculate how much we want to expand this tab stop (into spaces)
            let expanded_chars_old = tab_stop.char_offset + expanded_tab_len - tab_count;
            let tab_len = tab_size - ((expanded_chars_old - 1) % tab_size);
            // Increment tab count
            tab_count += 1;
            // The count of how many spaces we've added to this line in place of tab bytes
            expanded_tab_len += tab_len;

            // The count of bytes at this point in the iteration while considering tab_count and previous expansions
            let expanded_bytes = tab_stop.byte_offset + expanded_tab_len - tab_count;

            // Did we expand past the search target?
            if expanded_bytes > column {
                let mut expanded_chars = tab_stop.char_offset + expanded_tab_len - tab_count;
                // We expanded past the search target, so need to account for the offshoot
                expanded_chars -= expanded_bytes - column;
                return match bias {
                    Bias::Left => (
                        cursor.byte_offset() - 1,
                        expanded_chars,
                        expanded_bytes - column,
                    ),
                    Bias::Right => (cursor.byte_offset(), expanded_chars, 0),
                };
            } else {
                // otherwise we only want to move the cursor collapse column forward
                collapsed_column = collapsed_column - tab_len + 1;
                seek_target = (collapsed_column - cursor.byte_offset)
                    .min(self.max_expansion_column - cursor.byte_offset);
            }
        }

        let collapsed_bytes = cursor.byte_offset();
        let expanded_bytes = cursor.byte_offset() + expanded_tab_len - tab_count;
        let expanded_chars = cursor.char_offset() + expanded_tab_len - tab_count;
        (
            collapsed_bytes + column.saturating_sub(expanded_bytes),
            expanded_chars,
            0,
        )
    }
}

// todo(lw): Implement TabPointCursor properly
pub struct TabPointCursor<'this> {
    this: &'this TabSnapshot,
}

impl TabPointCursor<'_> {
    pub fn map(&mut self, point: FoldPoint) -> TabPoint {
        self.this.fold_point_to_tab_point(point)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct TabPoint(pub Point);

impl TabPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn row(self) -> u32 {
        self.0.row
    }

    pub fn column(self) -> u32 {
        self.0.column
    }
}

impl From<Point> for TabPoint {
    fn from(point: Point) -> Self {
        Self(point)
    }
}

pub type TabEdit = text::Edit<TabPoint>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    pub lines: Point,
    pub first_line_chars: u32,
    pub last_line_chars: u32,
    pub longest_row: u32,
    pub longest_row_chars: u32,
}

impl<'a> From<&'a str> for TextSummary {
    fn from(text: &'a str) -> Self {
        let sum = text::TextSummary::from(text);

        TextSummary {
            lines: sum.lines,
            first_line_chars: sum.first_line_chars,
            last_line_chars: sum.last_line_chars,
            longest_row: sum.longest_row,
            longest_row_chars: sum.longest_row_chars,
        }
    }
}

impl<'a> std::ops::AddAssign<&'a Self> for TextSummary {
    fn add_assign(&mut self, other: &'a Self) {
        let joined_chars = self.last_line_chars + other.first_line_chars;
        if joined_chars > self.longest_row_chars {
            self.longest_row = self.lines.row;
            self.longest_row_chars = joined_chars;
        }
        if other.longest_row_chars > self.longest_row_chars {
            self.longest_row = self.lines.row + other.longest_row;
            self.longest_row_chars = other.longest_row_chars;
        }

        if self.lines.row == 0 {
            self.first_line_chars += other.first_line_chars;
        }

        if other.lines.row == 0 {
            self.last_line_chars += other.first_line_chars;
        } else {
            self.last_line_chars = other.last_line_chars;
        }

        self.lines += &other.lines;
    }
}

pub struct TabChunks<'a> {
    snapshot: &'a TabSnapshot,
    max_expansion_column: u32,
    max_output_position: Point,
    tab_size: NonZeroU32,
    // region: iteration state
    fold_chunks: FoldChunks<'a>,
    chunk: Chunk<'a>,
    column: u32,
    output_position: Point,
    input_column: u32,
    inside_leading_tab: bool,
    // endregion: iteration state
}

impl TabChunks<'_> {
    pub(crate) fn seek(&mut self, range: Range<TabPoint>) {
        let (input_start, expanded_char_column, to_next_stop) = self
            .snapshot
            .tab_point_to_fold_point(range.start, Bias::Left);
        let input_column = input_start.column();
        let input_start = input_start.to_offset(&self.snapshot.fold_snapshot);
        let input_end = self
            .snapshot
            .tab_point_to_fold_point(range.end, Bias::Right)
            .0
            .to_offset(&self.snapshot.fold_snapshot);
        let to_next_stop = if range.start.0 + Point::new(0, to_next_stop) > range.end.0 {
            range.end.column() - range.start.column()
        } else {
            to_next_stop
        };

        self.fold_chunks.seek(input_start..input_end);
        self.input_column = input_column;
        self.column = expanded_char_column;
        self.output_position = range.start.0;
        self.max_output_position = range.end.0;
        self.chunk = Chunk {
            text: unsafe { std::str::from_utf8_unchecked(&SPACES[..to_next_stop as usize]) },
            is_tab: true,
            chars: 1u128.unbounded_shl(to_next_stop) - 1,
            ..Default::default()
        };
        self.inside_leading_tab = to_next_stop > 0;
    }
}

impl<'a> Iterator for TabChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.text.is_empty() {
            if let Some(chunk) = self.fold_chunks.next() {
                self.chunk = chunk;
                if self.inside_leading_tab {
                    self.chunk.text = &self.chunk.text[1..];
                    self.inside_leading_tab = false;
                    self.input_column += 1;
                }
            } else {
                return None;
            }
        }

        //todo(improve performance by using tab cursor)
        for (ix, c) in self.chunk.text.char_indices() {
            match c {
                '\t' if ix > 0 => {
                    let (prefix, suffix) = self.chunk.text.split_at(ix);

                    let mask = 1u128.unbounded_shl(ix as u32).wrapping_sub(1);
                    let chars = self.chunk.chars & mask;
                    let tabs = self.chunk.tabs & mask;
                    self.chunk.tabs = self.chunk.tabs.unbounded_shr(ix as u32);
                    self.chunk.chars = self.chunk.chars.unbounded_shr(ix as u32);
                    self.chunk.text = suffix;
                    return Some(Chunk {
                        text: prefix,
                        chars,
                        tabs,
                        ..self.chunk.clone()
                    });
                }
                '\t' => {
                    self.chunk.text = &self.chunk.text[1..];
                    self.chunk.tabs >>= 1;
                    self.chunk.chars >>= 1;
                    let tab_size = if self.input_column < self.max_expansion_column {
                        self.tab_size.get()
                    } else {
                        1
                    };
                    let mut len = tab_size - self.column % tab_size;
                    let next_output_position = cmp::min(
                        self.output_position + Point::new(0, len),
                        self.max_output_position,
                    );
                    len = next_output_position.column - self.output_position.column;
                    self.column += len;
                    self.input_column += 1;
                    self.output_position = next_output_position;
                    return Some(Chunk {
                        text: unsafe { std::str::from_utf8_unchecked(&SPACES[..len as usize]) },
                        is_tab: true,
                        chars: 1u128.unbounded_shl(len) - 1,
                        tabs: 0,
                        ..self.chunk.clone()
                    });
                }
                '\n' => {
                    self.column = 0;
                    self.input_column = 0;
                    self.output_position += Point::new(1, 0);
                }
                _ => {
                    self.column += 1;
                    if !self.inside_leading_tab {
                        self.input_column += c.len_utf8() as u32;
                    }
                    self.output_position.column += c.len_utf8() as u32;
                }
            }
        }

        Some(mem::take(&mut self.chunk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MultiBuffer,
        display_map::{
            fold_map::{FoldMap, FoldOffset},
            inlay_map::InlayMap,
        },
    };
    use multi_buffer::MultiBufferOffset;
    use rand::{Rng, prelude::StdRng};
    use util;

    impl TabSnapshot {
        fn expected_collapse_tabs(
            &self,
            chars: impl Iterator<Item = char>,
            column: u32,
            bias: Bias,
        ) -> (u32, u32, u32) {
            let tab_size = self.tab_size.get();

            let mut expanded_bytes = 0;
            let mut expanded_chars = 0;
            let mut collapsed_bytes = 0;
            for c in chars {
                if expanded_bytes >= column {
                    break;
                }
                if collapsed_bytes >= self.max_expansion_column {
                    break;
                }

                if c == '\t' {
                    let tab_len = tab_size - (expanded_chars % tab_size);
                    expanded_chars += tab_len;
                    expanded_bytes += tab_len;
                    if expanded_bytes > column {
                        expanded_chars -= expanded_bytes - column;
                        return match bias {
                            Bias::Left => {
                                (collapsed_bytes, expanded_chars, expanded_bytes - column)
                            }
                            Bias::Right => (collapsed_bytes + 1, expanded_chars, 0),
                        };
                    }
                } else {
                    expanded_chars += 1;
                    expanded_bytes += c.len_utf8() as u32;
                }

                if expanded_bytes > column && matches!(bias, Bias::Left) {
                    expanded_chars -= 1;
                    break;
                }

                collapsed_bytes += c.len_utf8() as u32;
            }

            (
                collapsed_bytes + column.saturating_sub(expanded_bytes),
                expanded_chars,
                0,
            )
        }

        pub fn expected_to_tab_point(&self, input: FoldPoint) -> TabPoint {
            let chars = self.fold_snapshot.chars_at(FoldPoint::new(input.row(), 0));
            let expanded = self.expected_expand_tabs(chars, input.column());
            TabPoint::new(input.row(), expanded)
        }

        fn expected_expand_tabs(&self, chars: impl Iterator<Item = char>, column: u32) -> u32 {
            let tab_size = self.tab_size.get();

            let mut expanded_chars = 0;
            let mut expanded_bytes = 0;
            let mut collapsed_bytes = 0;
            let end_column = column.min(self.max_expansion_column);
            for c in chars {
                if collapsed_bytes >= end_column {
                    break;
                }
                if c == '\t' {
                    let tab_len = tab_size - expanded_chars % tab_size;
                    expanded_bytes += tab_len;
                    expanded_chars += tab_len;
                } else {
                    expanded_bytes += c.len_utf8() as u32;
                    expanded_chars += 1;
                }
                collapsed_bytes += c.len_utf8() as u32;
            }

            expanded_bytes + column.saturating_sub(collapsed_bytes)
        }

        fn expected_to_fold_point(&self, output: TabPoint, bias: Bias) -> (FoldPoint, u32, u32) {
            let chars = self.fold_snapshot.chars_at(FoldPoint::new(output.row(), 0));
            let expanded = output.column();
            let (collapsed, expanded_char_column, to_next_stop) =
                self.expected_collapse_tabs(chars, expanded, bias);
            (
                FoldPoint::new(output.row(), collapsed),
                expanded_char_column,
                to_next_stop,
            )
        }
    }

    #[gpui::test]
    fn test_expand_tabs(cx: &mut gpui::App) {
        let test_values = [
            ("κg🏀 f\nwo🏀❌by🍐❎β🍗c\tβ❎ \ncλ🎉", 17),
            (" \twςe", 4),
            ("fε", 1),
            ("i❎\t", 3),
        ];
        let buffer = MultiBuffer::build_simple("", cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        for (text, column) in test_values {
            let mut tabs = 0u128;
            let mut chars = 0u128;
            for (idx, c) in text.char_indices() {
                if c == '\t' {
                    tabs |= 1 << idx;
                }
                chars |= 1 << idx;
            }

            let chunks = [Chunk {
                text,
                tabs,
                chars,
                ..Default::default()
            }];

            let cursor = TabStopCursor::new(chunks);

            assert_eq!(
                tab_snapshot.expected_expand_tabs(text.chars(), column),
                tab_snapshot.expand_tabs(cursor, column)
            );
        }
    }

    #[gpui::test]
    fn test_collapse_tabs(cx: &mut gpui::App) {
        let input = "A\tBC\tDEF\tG\tHI\tJ\tK\tL\tM";

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        for (ix, _) in input.char_indices() {
            let range = TabPoint::new(0, ix as u32)..tab_snapshot.max_point();

            assert_eq!(
                tab_snapshot.expected_to_fold_point(range.start, Bias::Left),
                tab_snapshot.tab_point_to_fold_point(range.start, Bias::Left),
                "Failed with tab_point at column {ix}"
            );
            assert_eq!(
                tab_snapshot.expected_to_fold_point(range.start, Bias::Right),
                tab_snapshot.tab_point_to_fold_point(range.start, Bias::Right),
                "Failed with tab_point at column {ix}"
            );

            assert_eq!(
                tab_snapshot.expected_to_fold_point(range.end, Bias::Left),
                tab_snapshot.tab_point_to_fold_point(range.end, Bias::Left),
                "Failed with tab_point at column {ix}"
            );
            assert_eq!(
                tab_snapshot.expected_to_fold_point(range.end, Bias::Right),
                tab_snapshot.tab_point_to_fold_point(range.end, Bias::Right),
                "Failed with tab_point at column {ix}"
            );
        }
    }

    #[gpui::test]
    fn test_to_fold_point_panic_reproduction(cx: &mut gpui::App) {
        // This test reproduces a specific panic where to_fold_point returns incorrect results
        let _text = "use macro_rules_attribute::apply;\nuse serde_json::Value;\nuse smol::{\n    io::AsyncReadExt,\n    process::{Command, Stdio},\n};\nuse smol_macros::main;\nuse std::io;\n\nfn test_random() {\n    // Generate a random value\n    let random_value = std::time::SystemTime::now()\n        .duration_since(std::time::UNIX_EPOCH)\n        .unwrap()\n        .as_secs()\n        % 100;\n\n    // Create some complex nested data structures\n    let mut vector = Vec::new();\n    for i in 0..random_value {\n        vector.push(i);\n    }\n    ";

        let text = "γ\tw⭐\n🍐🍗 \t";
        let buffer = MultiBuffer::build_simple(text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        // This should panic with the expected vs actual mismatch
        let tab_point = TabPoint::new(0, 9);
        let result = tab_snapshot.tab_point_to_fold_point(tab_point, Bias::Left);
        let expected = tab_snapshot.expected_to_fold_point(tab_point, Bias::Left);

        assert_eq!(result, expected);
    }

    #[gpui::test(iterations = 100)]
    fn test_collapse_tabs_random(cx: &mut gpui::App, mut rng: StdRng) {
        // Generate random input string with up to 200 characters including tabs
        // to stay within the MAX_EXPANSION_COLUMN limit of 256
        let len = rng.random_range(0..=2048);
        let tab_size = NonZeroU32::new(rng.random_range(1..=4)).unwrap();
        let mut input = String::with_capacity(len);

        for _ in 0..len {
            if rng.random_bool(0.1) {
                // 10% chance of inserting a tab
                input.push('\t');
            } else {
                // 90% chance of inserting a random ASCII character (excluding tab, newline, carriage return)
                let ch = loop {
                    let ascii_code = rng.random_range(32..=126); // printable ASCII range
                    let ch = ascii_code as u8 as char;
                    if ch != '\t' {
                        break ch;
                    }
                };
                input.push(ch);
            }
        }

        let buffer = MultiBuffer::build_simple(&input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, mut tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());
        tab_snapshot.max_expansion_column = rng.random_range(0..323);
        tab_snapshot.tab_size = tab_size;

        for (ix, _) in input.char_indices() {
            let range = TabPoint::new(0, ix as u32)..tab_snapshot.max_point();

            assert_eq!(
                tab_snapshot.expected_to_fold_point(range.start, Bias::Left),
                tab_snapshot.tab_point_to_fold_point(range.start, Bias::Left),
                "Failed with input: {}, with idx: {ix}",
                input
            );
            assert_eq!(
                tab_snapshot.expected_to_fold_point(range.start, Bias::Right),
                tab_snapshot.tab_point_to_fold_point(range.start, Bias::Right),
                "Failed with input: {}, with idx: {ix}",
                input
            );

            assert_eq!(
                tab_snapshot.expected_to_fold_point(range.end, Bias::Left),
                tab_snapshot.tab_point_to_fold_point(range.end, Bias::Left),
                "Failed with input: {}, with idx: {ix}",
                input
            );
            assert_eq!(
                tab_snapshot.expected_to_fold_point(range.end, Bias::Right),
                tab_snapshot.tab_point_to_fold_point(range.end, Bias::Right),
                "Failed with input: {}, with idx: {ix}",
                input
            );
        }
    }

    #[gpui::test]
    fn test_long_lines(cx: &mut gpui::App) {
        let max_expansion_column = 12;
        let input = "A\tBC\tDEF\tG\tHI\tJ\tK\tL\tM";
        let output = "A   BC  DEF G   HI J K L M";

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, mut tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        tab_snapshot.max_expansion_column = max_expansion_column;
        assert_eq!(tab_snapshot.text(), output);

        for (ix, c) in input.char_indices() {
            assert_eq!(
                tab_snapshot
                    .chunks(
                        TabPoint::new(0, ix as u32)..tab_snapshot.max_point(),
                        false,
                        Highlights::default(),
                    )
                    .map(|c| c.text)
                    .collect::<String>(),
                &output[ix..],
                "text from index {ix}"
            );

            if c != '\t' {
                let input_point = Point::new(0, ix as u32);
                let output_point = Point::new(0, output.find(c).unwrap() as u32);
                assert_eq!(
                    tab_snapshot.fold_point_to_tab_point(FoldPoint(input_point)),
                    TabPoint(output_point),
                    "to_tab_point({input_point:?})"
                );
                assert_eq!(
                    tab_snapshot
                        .tab_point_to_fold_point(TabPoint(output_point), Bias::Left)
                        .0,
                    FoldPoint(input_point),
                    "to_fold_point({output_point:?})"
                );
            }
        }
    }

    #[gpui::test]
    fn test_long_lines_with_character_spanning_max_expansion_column(cx: &mut gpui::App) {
        let max_expansion_column = 8;
        let input = "abcdefg⋯hij";

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, mut tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        tab_snapshot.max_expansion_column = max_expansion_column;
        assert_eq!(tab_snapshot.text(), input);
    }

    #[gpui::test]
    fn test_marking_tabs(cx: &mut gpui::App) {
        let input = "\t \thello";

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        assert_eq!(
            chunks(&tab_snapshot, TabPoint::zero()),
            vec![
                ("    ".to_string(), true),
                (" ".to_string(), false),
                ("   ".to_string(), true),
                ("hello".to_string(), false),
            ]
        );
        assert_eq!(
            chunks(&tab_snapshot, TabPoint::new(0, 2)),
            vec![
                ("  ".to_string(), true),
                (" ".to_string(), false),
                ("   ".to_string(), true),
                ("hello".to_string(), false),
            ]
        );

        fn chunks(snapshot: &TabSnapshot, start: TabPoint) -> Vec<(String, bool)> {
            let mut chunks = Vec::new();
            let mut was_tab = false;
            let mut text = String::new();
            for chunk in snapshot.chunks(start..snapshot.max_point(), false, Highlights::default())
            {
                if chunk.is_tab != was_tab {
                    if !text.is_empty() {
                        chunks.push((mem::take(&mut text), was_tab));
                    }
                    was_tab = chunk.is_tab;
                }
                text.push_str(chunk.text);
            }

            if !text.is_empty() {
                chunks.push((text, was_tab));
            }
            chunks
        }
    }

    #[gpui::test(iterations = 100)]
    fn test_random_tabs(cx: &mut gpui::App, mut rng: StdRng) {
        let tab_size = NonZeroU32::new(rng.random_range(1..=4)).unwrap();
        let len = rng.random_range(0..30);
        let buffer = if rng.random() {
            let text = util::RandomCharIter::new(&mut rng)
                .take(len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        log::info!("Buffer text: {:?}", buffer_snapshot.text());

        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        log::info!("InlayMap text: {:?}", inlay_snapshot.text());
        let (mut fold_map, _) = FoldMap::new(inlay_snapshot.clone());
        fold_map.randomly_mutate(&mut rng);
        let (fold_snapshot, _) = fold_map.read(inlay_snapshot, vec![]);
        log::info!("FoldMap text: {:?}", fold_snapshot.text());
        let (inlay_snapshot, _) = inlay_map.randomly_mutate(&mut 0, &mut rng);
        log::info!("InlayMap text: {:?}", inlay_snapshot.text());

        let (mut tab_map, _) = TabMap::new(fold_snapshot, tab_size);
        let tabs_snapshot = tab_map.set_max_expansion_column(32);

        let text = text::Rope::from(tabs_snapshot.text().as_str());
        log::info!(
            "TabMap text (tab size: {}): {:?}",
            tab_size,
            tabs_snapshot.text(),
        );

        for _ in 0..5 {
            let end_row = rng.random_range(0..=text.max_point().row);
            let end_column = rng.random_range(0..=text.line_len(end_row));
            let mut end = TabPoint(text.clip_point(Point::new(end_row, end_column), Bias::Right));
            let start_row = rng.random_range(0..=text.max_point().row);
            let start_column = rng.random_range(0..=text.line_len(start_row));
            let mut start =
                TabPoint(text.clip_point(Point::new(start_row, start_column), Bias::Left));
            if start > end {
                mem::swap(&mut start, &mut end);
            }

            let expected_text = text
                .chunks_in_range(text.point_to_offset(start.0)..text.point_to_offset(end.0))
                .collect::<String>();
            let expected_summary = TextSummary::from(expected_text.as_str());
            assert_eq!(
                tabs_snapshot
                    .chunks(start..end, false, Highlights::default())
                    .map(|c| c.text)
                    .collect::<String>(),
                expected_text,
                "chunks({:?}..{:?})",
                start,
                end
            );

            let mut actual_summary = tabs_snapshot.text_summary_for_range(start..end);
            if tab_size.get() > 1 && inlay_snapshot.text().contains('\t') {
                actual_summary.longest_row = expected_summary.longest_row;
                actual_summary.longest_row_chars = expected_summary.longest_row_chars;
            }
            assert_eq!(actual_summary, expected_summary);
        }

        for row in 0..=text.max_point().row {
            assert_eq!(
                tabs_snapshot.line_len(row),
                text.line_len(row),
                "line_len({row})"
            );
        }
    }

    #[gpui::test(iterations = 100)]
    fn test_to_tab_point_random(cx: &mut gpui::App, mut rng: StdRng) {
        let tab_size = NonZeroU32::new(rng.random_range(1..=16)).unwrap();
        let len = rng.random_range(0..=2000);

        // Generate random text using RandomCharIter
        let text = util::RandomCharIter::new(&mut rng)
            .take(len)
            .collect::<String>();

        // Create buffer and tab map
        let buffer = MultiBuffer::build_simple(&text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut tab_map, _) = TabMap::new(fold_snapshot, tab_size);

        let mut next_inlay_id = 0;
        let (inlay_snapshot, inlay_edits) = inlay_map.randomly_mutate(&mut next_inlay_id, &mut rng);
        let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
        let max_fold_point = fold_snapshot.max_point();
        let (mut tab_snapshot, _) = tab_map.sync(fold_snapshot.clone(), fold_edits, tab_size);

        // Test random fold points
        for _ in 0..50 {
            tab_snapshot.max_expansion_column = rng.random_range(0..=256);
            // Generate random fold point
            let row = rng.random_range(0..=max_fold_point.row());
            let max_column = if row < max_fold_point.row() {
                fold_snapshot.line_len(row)
            } else {
                max_fold_point.column()
            };
            let column = rng.random_range(0..=max_column + 10);
            let fold_point = FoldPoint::new(row, column);

            let actual = tab_snapshot.fold_point_to_tab_point(fold_point);
            let expected = tab_snapshot.expected_to_tab_point(fold_point);

            assert_eq!(
                actual, expected,
                "to_tab_point mismatch for fold_point {:?} in text {:?}",
                fold_point, text
            );
        }
    }

    #[gpui::test]
    fn test_tab_stop_cursor_utf8(cx: &mut gpui::App) {
        let text = "\tfoo\tbarbarbar\t\tbaz\n";
        let buffer = MultiBuffer::build_simple(text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let chunks = fold_snapshot.chunks(
            FoldOffset(MultiBufferOffset(0))..fold_snapshot.len(),
            false,
            Default::default(),
        );
        let mut cursor = TabStopCursor::new(chunks);
        assert!(cursor.seek(0).is_none());
        let mut tab_stops = Vec::new();

        let mut all_tab_stops = Vec::new();
        let mut byte_offset = 0;
        for (offset, ch) in buffer.read(cx).snapshot(cx).text().char_indices() {
            byte_offset += ch.len_utf8() as u32;

            if ch == '\t' {
                all_tab_stops.push(TabStop {
                    byte_offset,
                    char_offset: offset as u32 + 1,
                });
            }
        }

        while let Some(tab_stop) = cursor.seek(u32::MAX) {
            tab_stops.push(tab_stop);
        }
        pretty_assertions::assert_eq!(tab_stops.as_slice(), all_tab_stops.as_slice(),);

        assert_eq!(cursor.byte_offset(), byte_offset);
    }

    #[gpui::test]
    fn test_tab_stop_with_end_range_utf8(cx: &mut gpui::App) {
        let input = "A\tBC\t"; // DEF\tG\tHI\tJ\tK\tL\tM

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);

        let chunks = fold_snapshot.chunks_at(FoldPoint::new(0, 0));
        let mut cursor = TabStopCursor::new(chunks);

        let mut actual_tab_stops = Vec::new();

        let mut expected_tab_stops = Vec::new();
        let mut byte_offset = 0;
        for (offset, ch) in buffer.read(cx).snapshot(cx).text().char_indices() {
            byte_offset += ch.len_utf8() as u32;

            if ch == '\t' {
                expected_tab_stops.push(TabStop {
                    byte_offset,
                    char_offset: offset as u32 + 1,
                });
            }
        }

        while let Some(tab_stop) = cursor.seek(u32::MAX) {
            actual_tab_stops.push(tab_stop);
        }
        pretty_assertions::assert_eq!(actual_tab_stops.as_slice(), expected_tab_stops.as_slice(),);

        assert_eq!(cursor.byte_offset(), byte_offset);
    }

    #[gpui::test(iterations = 100)]
    fn test_tab_stop_cursor_random_utf8(cx: &mut gpui::App, mut rng: StdRng) {
        // Generate random input string with up to 512 characters including tabs
        let len = rng.random_range(0..=2048);
        let mut input = String::with_capacity(len);

        let mut skip_tabs = rng.random_bool(0.10);
        for idx in 0..len {
            if idx % 128 == 0 {
                skip_tabs = rng.random_bool(0.10);
            }

            if rng.random_bool(0.15) && !skip_tabs {
                input.push('\t');
            } else {
                let ch = loop {
                    let ascii_code = rng.random_range(32..=126); // printable ASCII range
                    let ch = ascii_code as u8 as char;
                    if ch != '\t' {
                        break ch;
                    }
                };
                input.push(ch);
            }
        }

        // Build the buffer and create cursor
        let buffer = MultiBuffer::build_simple(&input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);

        // First, collect all expected tab positions
        let mut all_tab_stops = Vec::new();
        let mut byte_offset = 1;
        let mut char_offset = 1;
        for ch in buffer_snapshot.text().chars() {
            if ch == '\t' {
                all_tab_stops.push(TabStop {
                    byte_offset,
                    char_offset,
                });
            }
            byte_offset += ch.len_utf8() as u32;
            char_offset += 1;
        }

        // Test with various distances
        let distances = vec![1, 5, 10, 50, 100, u32::MAX];
        // let distances = vec![150];

        for distance in distances {
            let chunks = fold_snapshot.chunks_at(FoldPoint::new(0, 0));
            let mut cursor = TabStopCursor::new(chunks);

            let mut found_tab_stops = Vec::new();
            let mut position = distance;
            while let Some(tab_stop) = cursor.seek(position) {
                found_tab_stops.push(tab_stop);
                position = distance - tab_stop.byte_offset;
            }

            let expected_found_tab_stops: Vec<_> = all_tab_stops
                .iter()
                .take_while(|tab_stop| tab_stop.byte_offset <= distance)
                .cloned()
                .collect();

            pretty_assertions::assert_eq!(
                found_tab_stops,
                expected_found_tab_stops,
                "TabStopCursor output mismatch for distance {}. Input: {:?}",
                distance,
                input
            );

            let final_position = cursor.byte_offset();
            if !found_tab_stops.is_empty() {
                let last_tab_stop = found_tab_stops.last().unwrap();
                assert!(
                    final_position >= last_tab_stop.byte_offset,
                    "Cursor final position {} is before last tab stop {}. Input: {:?}",
                    final_position,
                    last_tab_stop.byte_offset,
                    input
                );
            }
        }
    }

    #[gpui::test]
    fn test_tab_stop_cursor_utf16(cx: &mut gpui::App) {
        let text = "\r\t😁foo\tb😀arbar🤯bar\t\tbaz\n";
        let buffer = MultiBuffer::build_simple(text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let chunks = fold_snapshot.chunks(
            FoldOffset(MultiBufferOffset(0))..fold_snapshot.len(),
            false,
            Default::default(),
        );
        let mut cursor = TabStopCursor::new(chunks);
        assert!(cursor.seek(0).is_none());

        let mut expected_tab_stops = Vec::new();
        let mut byte_offset = 0;
        for (i, ch) in fold_snapshot.chars_at(FoldPoint::new(0, 0)).enumerate() {
            byte_offset += ch.len_utf8() as u32;

            if ch == '\t' {
                expected_tab_stops.push(TabStop {
                    byte_offset,
                    char_offset: i as u32 + 1,
                });
            }
        }

        let mut actual_tab_stops = Vec::new();
        while let Some(tab_stop) = cursor.seek(u32::MAX) {
            actual_tab_stops.push(tab_stop);
        }

        pretty_assertions::assert_eq!(actual_tab_stops.as_slice(), expected_tab_stops.as_slice(),);

        assert_eq!(cursor.byte_offset(), byte_offset);
    }

    #[gpui::test(iterations = 100)]
    fn test_tab_stop_cursor_random_utf16(cx: &mut gpui::App, mut rng: StdRng) {
        // Generate random input string with up to 512 characters including tabs
        let len = rng.random_range(0..=2048);
        let input = util::RandomCharIter::new(&mut rng)
            .take(len)
            .collect::<String>();

        // Build the buffer and create cursor
        let buffer = MultiBuffer::build_simple(&input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);

        // First, collect all expected tab positions
        let mut all_tab_stops = Vec::new();
        let mut byte_offset = 0;
        for (i, ch) in buffer_snapshot.text().chars().enumerate() {
            byte_offset += ch.len_utf8() as u32;
            if ch == '\t' {
                all_tab_stops.push(TabStop {
                    byte_offset,
                    char_offset: i as u32 + 1,
                });
            }
        }

        // Test with various distances
        // let distances = vec![1, 5, 10, 50, 100, u32::MAX];
        let distances = vec![150];

        for distance in distances {
            let chunks = fold_snapshot.chunks_at(FoldPoint::new(0, 0));
            let mut cursor = TabStopCursor::new(chunks);

            let mut found_tab_stops = Vec::new();
            let mut position = distance;
            while let Some(tab_stop) = cursor.seek(position) {
                found_tab_stops.push(tab_stop);
                position = distance - tab_stop.byte_offset;
            }

            let expected_found_tab_stops: Vec<_> = all_tab_stops
                .iter()
                .take_while(|tab_stop| tab_stop.byte_offset <= distance)
                .cloned()
                .collect();

            pretty_assertions::assert_eq!(
                found_tab_stops,
                expected_found_tab_stops,
                "TabStopCursor output mismatch for distance {}. Input: {:?}",
                distance,
                input
            );

            let final_position = cursor.byte_offset();
            if !found_tab_stops.is_empty() {
                let last_tab_stop = found_tab_stops.last().unwrap();
                assert!(
                    final_position >= last_tab_stop.byte_offset,
                    "Cursor final position {} is before last tab stop {}. Input: {:?}",
                    final_position,
                    last_tab_stop.byte_offset,
                    input
                );
            }
        }
    }
}

struct TabStopCursor<'a, I>
where
    I: Iterator<Item = Chunk<'a>>,
{
    chunks: I,
    byte_offset: u32,
    char_offset: u32,
    /// Chunk
    /// last tab position iterated through
    current_chunk: Option<(Chunk<'a>, u32)>,
}

impl<'a, I> TabStopCursor<'a, I>
where
    I: Iterator<Item = Chunk<'a>>,
{
    fn new(chunks: impl IntoIterator<Item = Chunk<'a>, IntoIter = I>) -> Self {
        Self {
            chunks: chunks.into_iter(),
            byte_offset: 0,
            char_offset: 0,
            current_chunk: None,
        }
    }

    fn bytes_until_next_char(&self) -> Option<usize> {
        self.current_chunk.as_ref().and_then(|(chunk, idx)| {
            let mut idx = *idx;
            let mut diff = 0;
            while idx > 0 && chunk.chars & (1u128.unbounded_shl(idx)) == 0 {
                idx -= 1;
                diff += 1;
            }

            if chunk.chars & (1 << idx) != 0 {
                Some(
                    (chunk.text[idx as usize..].chars().next()?)
                        .len_utf8()
                        .saturating_sub(diff),
                )
            } else {
                None
            }
        })
    }

    fn is_char_boundary(&self) -> bool {
        self.current_chunk
            .as_ref()
            .is_some_and(|(chunk, idx)| (chunk.chars & 1u128.unbounded_shl(*idx)) != 0)
    }

    /// distance: length to move forward while searching for the next tab stop
    fn seek(&mut self, distance: u32) -> Option<TabStop> {
        if distance == 0 {
            return None;
        }

        let mut distance_traversed = 0;

        while let Some((mut chunk, chunk_position)) = self
            .current_chunk
            .take()
            .or_else(|| self.chunks.next().zip(Some(0)))
        {
            if chunk.tabs == 0 {
                let chunk_distance = chunk.text.len() as u32 - chunk_position;
                if chunk_distance + distance_traversed >= distance {
                    let overshoot = distance_traversed.abs_diff(distance);

                    self.byte_offset += overshoot;
                    self.char_offset += get_char_offset(
                        chunk_position..(chunk_position + overshoot).saturating_sub(1),
                        chunk.chars,
                    );

                    if chunk_position + overshoot < 128 {
                        self.current_chunk = Some((chunk, chunk_position + overshoot));
                    }

                    return None;
                }

                self.byte_offset += chunk_distance;
                self.char_offset += get_char_offset(
                    chunk_position..(chunk_position + chunk_distance).saturating_sub(1),
                    chunk.chars,
                );
                distance_traversed += chunk_distance;
                continue;
            }
            let tab_position = chunk.tabs.trailing_zeros() + 1;

            if distance_traversed + tab_position - chunk_position > distance {
                let cursor_position = distance_traversed.abs_diff(distance);

                self.char_offset += get_char_offset(
                    chunk_position..(chunk_position + cursor_position - 1),
                    chunk.chars,
                );
                self.current_chunk = Some((chunk, cursor_position + chunk_position));
                self.byte_offset += cursor_position;

                return None;
            }

            self.byte_offset += tab_position - chunk_position;
            self.char_offset += get_char_offset(chunk_position..(tab_position - 1), chunk.chars);

            let tabstop = TabStop {
                char_offset: self.char_offset,
                byte_offset: self.byte_offset,
            };

            chunk.tabs = (chunk.tabs - 1) & chunk.tabs;

            if tab_position as usize != chunk.text.len() {
                self.current_chunk = Some((chunk, tab_position));
            }

            return Some(tabstop);
        }

        None
    }

    fn byte_offset(&self) -> u32 {
        self.byte_offset
    }

    fn char_offset(&self) -> u32 {
        self.char_offset
    }
}

#[inline(always)]
fn get_char_offset(range: Range<u32>, bit_map: u128) -> u32 {
    if range.start == range.end {
        return if (1u128 << range.start) & bit_map == 0 {
            0
        } else {
            1
        };
    }
    let end_shift: u128 = 127u128 - range.end as u128;
    let mut bit_mask = (u128::MAX >> range.start) << range.start;
    bit_mask = (bit_mask << end_shift) >> end_shift;
    let bit_map = bit_map & bit_mask;

    bit_map.count_ones()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TabStop {
    char_offset: u32,
    byte_offset: u32,
}
