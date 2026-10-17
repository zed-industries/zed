use super::{
    Highlights,
    fold_map::{self, Chunk, FoldChunks, FoldEdit, FoldOffset, FoldPoint, FoldSnapshot},
};

use language::{LanguageAwareStyling, Point};
use multi_buffer::{MBTextSummary, MultiBufferOffset, MultiBufferRow, MultiBufferSnapshot};
use std::{cmp, num::NonZeroU32, ops::Range};
use sum_tree::{Bias, ContextLessSummary as _, Cursor, Dimensions, SumTree};

// Handles a tab width <= 128
const SPACES: &[u8; rope::Chunk::MASK_BITS] = &[b' '; _];
const MAX_TABS: NonZeroU32 = NonZeroU32::new(SPACES.len() as u32).unwrap();

/// Keeps track of hard tabs in a text buffer.
///
/// See the [`display_map` module documentation](crate::display_map) for more information.
pub struct TabMap(TabSnapshot);

impl TabMap {
    #[ztracing::instrument(skip_all)]
    pub fn new(fold_snapshot: FoldSnapshot, tab_size: NonZeroU32) -> (Self, TabSnapshot) {
        let mut snapshot = TabSnapshot {
            fold_snapshot,
            tab_size: tab_size.min(MAX_TABS),
            version: 0,
            transforms: SumTree::new(()),
            unrepresentable_rows: false,
            #[cfg(any(test, feature = "test-support"))]
            column_limit: u32::MAX,
            #[cfg(test)]
            indexed_bytes: 0,
        };
        snapshot.rebuild();
        (Self(snapshot.clone()), snapshot)
    }

    #[ztracing::instrument(skip_all)]
    pub fn sync(
        &mut self,
        fold_snapshot: FoldSnapshot,
        fold_edits: Vec<FoldEdit>,
        tab_size: NonZeroU32,
    ) -> (TabSnapshot, Vec<TabEdit>) {
        let tab_size = tab_size.min(MAX_TABS);
        if self.0.tab_size != tab_size
            || (self.0.has_unrepresentable_rows() && !fold_edits.is_empty())
        {
            let old_max_point = self.0.max_point();
            let mut snapshot = self.0.clone();
            snapshot.fold_snapshot = fold_snapshot;
            snapshot.tab_size = tab_size;
            snapshot.version += 1;
            snapshot.rebuild();
            let edit = TabEdit {
                old: TabPoint::zero()..old_max_point,
                new: TabPoint::zero()..snapshot.max_point(),
            };
            self.0 = snapshot;
            return (self.0.clone(), vec![edit]);
        }
        let version =
            self.0.version + usize::from(self.0.fold_snapshot.version != fold_snapshot.version);
        if fold_edits.is_empty() {
            self.0.fold_snapshot = fold_snapshot;
            self.0.version = version;
            #[cfg(test)]
            {
                self.0.indexed_bytes = 0;
            }
            return (self.0.clone(), Vec::new());
        }

        let old_snapshot = &self.0;
        let mut edits = Vec::<FoldEdit>::new();
        let mut pending = fold_edits.into_iter().peekable();
        while let Some(mut edit) = pending.next() {
            loop {
                let end = old_snapshot
                    .following_tab_end(edit.old.end)
                    .unwrap_or(edit.old.end);
                if let Some(next) = pending.next_if(|next| next.old.start <= end) {
                    edit.old.end = next.old.end;
                    edit.new.end = next.new.end;
                } else {
                    edit.new.end += end - edit.old.end;
                    edit.old.end = end;
                    break;
                }
            }
            edits.push(edit);
        }

        let mut snapshot = TabSnapshot {
            fold_snapshot,
            tab_size,
            version,
            transforms: SumTree::new(()),
            unrepresentable_rows: false,
            #[cfg(any(test, feature = "test-support"))]
            column_limit: self.0.column_limit,
            #[cfg(test)]
            indexed_bytes: 0,
        };
        let mut old_start = FoldOffset::default();
        for edit in &edits {
            snapshot.append_transforms(old_snapshot.slice_transforms(old_start..edit.old.start));
            snapshot.push_range(edit.new.start..edit.new.end);
            old_start = edit.old.end;
        }
        snapshot.append_transforms(
            old_snapshot.slice_transforms(old_start..old_snapshot.fold_snapshot.len()),
        );
        snapshot.finish_index();
        if snapshot.has_unrepresentable_rows() {
            let edit = TabEdit {
                old: TabPoint::zero()..old_snapshot.max_point(),
                new: TabPoint::zero()..snapshot.max_point(),
            };
            self.0 = snapshot;
            return (self.0.clone(), vec![edit]);
        }

        let tab_edits = edits
            .into_iter()
            .map(|edit| TabEdit {
                old: old_snapshot
                    .fold_point_to_tab_point(edit.old.start.to_point(&old_snapshot.fold_snapshot))
                    ..old_snapshot.fold_point_to_tab_point(
                        edit.old.end.to_point(&old_snapshot.fold_snapshot),
                    ),
                new: snapshot
                    .fold_point_to_tab_point(edit.new.start.to_point(&snapshot.fold_snapshot))
                    ..snapshot
                        .fold_point_to_tab_point(edit.new.end.to_point(&snapshot.fold_snapshot)),
            })
            .collect();
        self.0 = snapshot;
        (self.0.clone(), tab_edits)
    }

    #[cfg(test)]
    pub(super) fn new_with_column_limit(
        fold_snapshot: FoldSnapshot,
        tab_size: NonZeroU32,
        column_limit: u32,
    ) -> (Self, TabSnapshot) {
        let (mut map, _) = Self::new(fold_snapshot, tab_size);
        let (snapshot, _) = map.set_column_limit(column_limit);
        (map, snapshot)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(super) fn set_column_limit(&mut self, column_limit: u32) -> (TabSnapshot, Vec<TabEdit>) {
        assert!(column_limit >= (rope::Chunk::MASK_BITS * SPACES.len()) as u32);
        assert!(self.0.fold_snapshot.len().0.0 <= column_limit as usize);
        let old_max_point = self.0.max_point();
        self.0.column_limit = column_limit;
        self.0.version += 1;
        self.0.rebuild();
        let edit = TabEdit {
            old: TabPoint::zero()..old_max_point,
            new: TabPoint::zero()..self.0.max_point(),
        };
        (self.0.clone(), vec![edit])
    }
}

#[derive(Clone)]
pub struct TabSnapshot {
    pub fold_snapshot: FoldSnapshot,
    pub tab_size: NonZeroU32,
    pub version: usize,
    transforms: SumTree<TabTransform>,
    unrepresentable_rows: bool,
    #[cfg(any(test, feature = "test-support"))]
    column_limit: u32,
    #[cfg(test)]
    indexed_bytes: usize,
}

impl std::ops::Deref for TabSnapshot {
    type Target = FoldSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.fold_snapshot
    }
}

impl TabSnapshot {
    #[inline]
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        &self.fold_snapshot.inlay_snapshot.buffer
    }

    #[ztracing::instrument(skip_all)]
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

    pub(crate) fn has_unrepresentable_rows(&self) -> bool {
        self.unrepresentable_rows
    }

    pub(crate) fn max_line_len_bound(&self, rows: Range<u32>) -> u32 {
        if rows.is_empty() || rows.start > self.max_point().row() {
            return 0;
        }
        self.text_summary_for_range(
            TabPoint::new(rows.start, 0)..TabPoint::new(rows.end, 0).min(self.max_point()),
        )
        .longest_row_chars
        .saturating_mul(4)
    }

    #[ztracing::instrument(skip_all)]
    pub fn text_summary(&self) -> TextSummary {
        self.transforms.summary().output.clone()
    }

    #[ztracing::instrument(skip_all)]
    pub fn text_summary_for_range(&self, range: Range<TabPoint>) -> TextSummary {
        if range.is_empty() {
            return TextSummary::default();
        }
        if range.start == TabPoint::zero() && range.end == self.max_point() {
            return self.text_summary();
        }
        let mut summary = TextSummary::default();
        let mut cursor = self
            .transforms
            .cursor::<Dimensions<TabPoint, TransformSummary>>(());
        cursor.seek(&range.start, Bias::Right);
        if let Some(transform) = cursor.item() {
            summary = self.summarize_transform(
                transform,
                &cursor.start().1,
                range.start..range.end.min(cursor.end().0),
            );
        }
        if range.end > cursor.end().0 {
            cursor.next();
            summary += &cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right)
                .output;
            if let Some(transform) = cursor.item() {
                summary += &self.summarize_transform(
                    transform,
                    &cursor.start().1,
                    cursor.start().0..range.end,
                );
            }
        }
        summary
    }

    #[ztracing::instrument(skip_all)]
    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<TabPoint>,
        language_aware: LanguageAwareStyling,
        highlights: Highlights<'a>,
    ) -> TabChunks<'a> {
        let (input_start, expanded_char_column, to_next_stop) =
            self.tab_point_to_fold_point(range.start, Bias::Left);
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
            column: expanded_char_column,
            output_position: range.start.0,
            max_output_position: range.end.0,
            tab_size: self.tab_size,
            chunk: Chunk::default(),
            leading_tab_width: to_next_stop,
        }
    }

    #[ztracing::instrument(skip_all)]
    pub fn rows(&self, row: u32) -> fold_map::FoldRows<'_> {
        self.fold_snapshot.row_infos(row)
    }

    #[cfg(test)]
    #[ztracing::instrument(skip_all)]
    pub fn text(&self) -> String {
        self.chunks(
            TabPoint::zero()..self.max_point(),
            LanguageAwareStyling {
                tree_sitter: false,
                diagnostics: false,
            },
            Highlights::default(),
        )
        .map(|chunk| chunk.text)
        .collect()
    }

    #[ztracing::instrument(skip_all)]
    pub fn max_point(&self) -> TabPoint {
        TabPoint(self.transforms.summary().output.lines)
    }

    #[ztracing::instrument(skip_all)]
    pub fn clip_point(&self, point: TabPoint, bias: Bias) -> TabPoint {
        let point = point.min(self.max_point());
        self.fold_point_to_tab_point(
            self.fold_snapshot
                .clip_point(self.tab_point_to_fold_point(point, bias).0, bias),
        )
    }

    #[ztracing::instrument(skip_all)]
    pub fn fold_point_to_tab_point(&self, input: FoldPoint) -> TabPoint {
        self.tab_point_cursor().map(input)
    }

    #[ztracing::instrument(skip_all)]
    pub fn tab_point_cursor(&self) -> TabPointCursor<'_> {
        TabPointCursor {
            snapshot: self,
            cursor: self
                .transforms
                .cursor::<Dimensions<FoldPoint, TransformSummary>>(()),
        }
    }

    #[ztracing::instrument(skip_all)]
    pub fn tab_point_to_fold_point(&self, output: TabPoint, bias: Bias) -> (FoldPoint, u32, u32) {
        if output.row() > self.max_point().row() {
            return (
                self.fold_snapshot.max_point(),
                self.transforms.summary().output.last_line_chars,
                0,
            );
        }
        let (start, end, transform) = self
            .transforms
            .find::<Dimensions<TabPoint, TransformSummary>, _>((), &output, Bias::Right);
        let overshoot = output.0 - start.0.0;
        if transform.is_some_and(|transform| transform.has_tabs) {
            let mut input = start.1.input;
            let mut position = start.0.0;
            let mut column = start.1.output.last_line_chars;
            for chunk in self.fold_snapshot.chunks(
                FoldOffset(start.1.input_bytes)..FoldOffset(end.1.input_bytes),
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                Highlights::default(),
            ) {
                for character in chunk.text.chars() {
                    if position >= output.0 {
                        return (FoldPoint(input), column, 0);
                    }
                    if character == '\n' {
                        if position.row == output.row() {
                            input.column += output.column() - position.column;
                            return (FoldPoint(input), column, 0);
                        }
                        input += Point::new(1, 0);
                        position += Point::new(1, 0);
                        column = 0;
                        continue;
                    }
                    let bytes = character.len_utf8() as u32;
                    let width = if character == '\t' {
                        self.tab_size.get() - column % self.tab_size.get()
                    } else {
                        bytes
                    };
                    if position + Point::new(0, width) > output.0 {
                        if character == '\t' {
                            let consumed = output.column() - position.column;
                            return match bias {
                                Bias::Left => {
                                    (FoldPoint(input), column + consumed, width - consumed)
                                }
                                Bias::Right => {
                                    (FoldPoint(input + Point::new(0, 1)), column + consumed, 0)
                                }
                            };
                        }
                        return match bias {
                            Bias::Left => (FoldPoint(input), column, 0),
                            Bias::Right => (FoldPoint(input + Point::new(0, bytes)), column + 1, 0),
                        };
                    }
                    input.column += bytes;
                    position.column += width;
                    column += if character == '\t' { width } else { 1 };
                }
            }
            return (FoldPoint(input + (output.0 - position)), column, 0);
        }
        let input = FoldPoint(start.1.input + overshoot);
        let line_len = self.fold_snapshot.line_len(input.row());
        let virtual_bytes = input.column().saturating_sub(line_len);
        let input = FoldPoint::new(input.row(), input.column().min(line_len));
        let offset = input.to_offset(&self.fold_snapshot);
        let clipped = self.fold_snapshot.clip_raw_offset(offset, bias);
        let mut point = clipped.to_point(&self.fold_snapshot);
        let mut summary = start.1.output;
        summary += &TextSummary::from(
            self.fold_snapshot
                .text_summary_for_offsets(FoldOffset(start.1.input_bytes)..clipped),
        );
        point.0.column += virtual_bytes;
        (point, summary.last_line_chars, 0)
    }

    pub fn buffer_row_to_tab_row(&self, buffer_row: MultiBufferRow) -> u32 {
        self.point_to_tab_point(Point::new(buffer_row.0, 0), Bias::Left)
            .row()
    }

    #[ztracing::instrument(skip_all)]
    pub fn point_to_tab_point(&self, point: Point, bias: Bias) -> TabPoint {
        let inlay_point = self.fold_snapshot.inlay_snapshot.to_inlay_point(point);
        let fold_point = self.fold_snapshot.to_fold_point(inlay_point, bias);
        self.fold_point_to_tab_point(fold_point)
    }

    #[ztracing::instrument(skip_all)]
    pub fn tab_point_to_point(&self, point: TabPoint, bias: Bias) -> Point {
        let fold_point = self.tab_point_to_fold_point(point, bias).0;
        let inlay_point = fold_point.to_inlay_point(&self.fold_snapshot);
        self.fold_snapshot
            .inlay_snapshot
            .to_buffer_point(inlay_point)
    }

    fn rebuild(&mut self) {
        self.transforms = SumTree::new(());
        self.unrepresentable_rows = false;
        #[cfg(test)]
        {
            self.indexed_bytes = 0;
        }
        self.push_range(FoldOffset::default()..self.fold_snapshot.len());
        self.finish_index();
    }

    fn finish_index(&mut self) {
        if self.unrepresentable_rows {
            let range = FoldOffset::default()..self.fold_snapshot.len();
            let summary = self.fold_snapshot.text_summary_for_offsets(range.clone());
            self.transforms = SumTree::from_item(self.isomorphic_transform(range, summary), ());
        }
    }

    fn column_limit(&self) -> u32 {
        #[cfg(any(test, feature = "test-support"))]
        {
            self.column_limit
        }
        #[cfg(not(any(test, feature = "test-support")))]
        {
            u32::MAX
        }
    }

    fn append_transforms(&mut self, suffix: SumTree<TabTransform>) {
        if self.unrepresentable_rows {
            return;
        }
        if can_append(
            self.transforms.summary(),
            suffix.summary(),
            self.column_limit(),
        ) {
            append_transforms(&mut self.transforms, suffix);
        } else {
            self.unrepresentable_rows = true;
        }
    }

    fn push_indexed_transform(&mut self, transform: TabTransform) {
        if can_append(
            self.transforms.summary(),
            &transform.summary,
            self.column_limit(),
        ) {
            push_transform(&mut self.transforms, transform);
        } else {
            self.unrepresentable_rows = true;
        }
    }

    fn isomorphic_transform(
        &self,
        range: Range<FoldOffset>,
        summary: MBTextSummary,
    ) -> TabTransform {
        let first_line_bytes = if summary.lines.row == 0 {
            summary.lines.column
        } else {
            let row = range.start.to_point(&self.fold_snapshot).row();
            let end = FoldPoint::new(row + 1, 0).to_offset(&self.fold_snapshot);
            (end - range.start - 1) as u32
        };
        TabTransform::isomorphic(summary, first_line_bytes)
    }

    fn push_range(&mut self, range: Range<FoldOffset>) {
        if range.is_empty() || self.unrepresentable_rows {
            return;
        }
        let summary = self.fold_snapshot.text_summary_for_offsets(range.clone());
        if !summary.has_tabs || self.tab_size.get() == 1 {
            self.push_indexed_transform(self.isomorphic_transform(range, summary));
        } else if range.end - range.start > rope::Chunk::MASK_BITS {
            let middle = self
                .fold_snapshot
                .clip_raw_offset(range.start + (range.end - range.start) / 2, Bias::Right);
            self.push_range(range.start..middle);
            self.push_range(middle..range.end);
        } else {
            #[cfg(test)]
            {
                self.indexed_bytes += range.end - range.start;
            }
            let transform =
                self.summarize_input_range(range, self.transforms.summary().output.last_line_chars);
            self.push_indexed_transform(transform);
        }
    }

    fn following_tab_end(&self, offset: FoldOffset) -> Option<FoldOffset> {
        let mut cursor = self
            .transforms
            .cursor::<Dimensions<FoldOffset, FoldPoint>>(());
        cursor.seek(&offset, Bias::Right);
        let row = offset.to_point(&self.fold_snapshot).row();
        while let Some(transform) = cursor.item() {
            if transform.has_tabs {
                let mut offset = offset.max(cursor.start().0);
                for chunk in self.fold_snapshot.chunks(
                    offset..cursor.end().0,
                    LanguageAwareStyling {
                        tree_sitter: false,
                        diagnostics: false,
                    },
                    Highlights::default(),
                ) {
                    if chunk.tabs != 0
                        && chunk.tabs.trailing_zeros() < chunk.newlines.trailing_zeros()
                    {
                        return Some(offset + (chunk.tabs.trailing_zeros() as usize + 1));
                    }
                    if chunk.newlines != 0 {
                        return None;
                    }
                    offset += chunk.text.len();
                }
            } else if cursor.end().1.row() > row {
                return None;
            }
            cursor.next();
        }
        None
    }

    fn summarize_input_range(&self, range: Range<FoldOffset>, column: u32) -> TabTransform {
        let summary = self.fold_snapshot.text_summary_for_offsets(range.clone());
        if !summary.has_tabs || self.tab_size.get() == 1 {
            return self.isomorphic_transform(range, summary);
        }
        debug_assert!(range.end - range.start <= rope::Chunk::MASK_BITS);
        let tab_size = self.tab_size.get();
        let mut column = u64::from(column);
        let mut output = TextSummary::default();
        let mut first_line_bytes = 0;
        for chunk in self.fold_snapshot.chunks(
            range,
            LanguageAwareStyling {
                tree_sitter: false,
                diagnostics: false,
            },
            Highlights::default(),
        ) {
            let mut start = 0;
            for (offset, _) in chunk.text.match_indices('\t') {
                let prefix = TextSummary::from(&chunk.text[start..offset]);
                column = if prefix.lines.row == 0 {
                    column + u64::from(prefix.last_line_chars)
                } else {
                    u64::from(prefix.last_line_chars)
                };
                if output.lines.row == 0 {
                    first_line_bytes += chunk.text[start..offset]
                        .find('\n')
                        .unwrap_or(offset - start) as u32;
                }
                output += &prefix;
                let width = tab_size - (column % u64::from(tab_size)) as u32;
                if output.lines.row == 0 {
                    first_line_bytes += width;
                }
                output += &TextSummary::spaces(width);
                column += u64::from(width);
                start = offset + 1;
            }
            let suffix = TextSummary::from(&chunk.text[start..]);
            column = if suffix.lines.row == 0 {
                column + u64::from(suffix.last_line_chars)
            } else {
                u64::from(suffix.last_line_chars)
            };
            if output.lines.row == 0 {
                first_line_bytes += chunk.text[start..]
                    .find('\n')
                    .unwrap_or(chunk.text.len() - start) as u32;
            }
            output += &suffix;
        }
        TabTransform {
            summary: TransformSummary {
                input: summary.lines,
                input_bytes: summary.len,
                output,
                first_line_bytes,
            },
            has_tabs: true,
        }
    }

    fn expanded_column_at(&self, offset: FoldOffset) -> u32 {
        let (start, _, _) = self
            .transforms
            .find::<Dimensions<FoldOffset, TransformSummary>, _>((), &offset, Bias::Right);
        let mut output = start.1.output;
        let prefix = self.summarize_input_range(start.0..offset, output.last_line_chars);
        output += &prefix.summary.output;
        output.last_line_chars
    }

    fn slice_transforms(&self, range: Range<FoldOffset>) -> SumTree<TabTransform> {
        let mut result = SumTree::new(());
        if range.is_empty() {
            return result;
        }
        let mut cursor = self.transforms.cursor::<FoldOffset>(());
        cursor.seek(&range.start, Bias::Right);
        if *cursor.start() < range.start {
            push_transform(
                &mut result,
                self.summarize_input_range(
                    range.start..range.end.min(cursor.end()),
                    self.expanded_column_at(range.start),
                ),
            );
            if range.end <= cursor.end() {
                return result;
            }
            cursor.next();
        }
        append_transforms(&mut result, cursor.slice(&range.end, Bias::Right));
        if *cursor.start() < range.end {
            push_transform(
                &mut result,
                self.summarize_input_range(
                    *cursor.start()..range.end,
                    self.expanded_column_at(*cursor.start()),
                ),
            );
        }
        result
    }

    fn summarize_transform(
        &self,
        transform: &TabTransform,
        start: &TransformSummary,
        range: Range<TabPoint>,
    ) -> TextSummary {
        if range.start.0 == start.output.lines
            && range.end.0 == start.output.lines + transform.summary.output.lines
        {
            return transform.summary.output.clone();
        }
        if transform.has_tabs {
            let mut summary = TextSummary::default();
            for chunk in self.chunks(
                range,
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                Highlights::default(),
            ) {
                summary += &if chunk.is_tab {
                    TextSummary::spaces(chunk.text.len() as u32)
                } else {
                    TextSummary::from(chunk.text)
                };
            }
            summary
        } else {
            TextSummary::from(self.fold_snapshot.text_summary_for_range(
                FoldPoint(start.input + (range.start.0 - start.output.lines))
                    ..FoldPoint(start.input + (range.end.0 - start.output.lines)),
            ))
        }
    }
}

pub struct TabPointCursor<'a> {
    snapshot: &'a TabSnapshot,
    cursor: Cursor<'a, 'static, TabTransform, Dimensions<FoldPoint, TransformSummary>>,
}

impl TabPointCursor<'_> {
    pub fn reset(&mut self) {
        self.cursor.reset();
    }

    #[ztracing::instrument(skip_all)]
    pub fn map(&mut self, point: FoldPoint) -> TabPoint {
        let snapshot = &self.snapshot.fold_snapshot;
        if point.row() > snapshot.max_point().row() {
            return self.snapshot.max_point();
        }
        let line_len = snapshot.line_len(point.row());
        let virtual_bytes = point.column().saturating_sub(line_len);
        let offset = FoldPoint::new(point.row(), point.column().min(line_len)).to_offset(snapshot);
        let point = snapshot
            .clip_raw_offset(offset, Bias::Right)
            .to_point(snapshot);
        if self.cursor.did_seek() && point >= self.cursor.start().0 {
            self.cursor.seek_forward(&point, Bias::Right);
        } else {
            self.cursor.seek(&point, Bias::Right);
        }
        let start = &self.cursor.start().1;
        let overshoot = if self
            .cursor
            .item()
            .is_some_and(|transform| transform.has_tabs)
        {
            self.snapshot
                .summarize_input_range(
                    FoldOffset(start.input_bytes)..point.to_offset(snapshot),
                    start.output.last_line_chars,
                )
                .summary
                .output
                .lines
        } else {
            point.0 - start.input
        };
        let mut output = start.output.lines + overshoot;
        output.column = output.column.saturating_add(virtual_bytes);
        TabPoint(output)
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

impl TextSummary {
    fn spaces(width: u32) -> Self {
        Self {
            lines: Point::new(0, width),
            first_line_chars: width,
            last_line_chars: width,
            longest_row: 0,
            longest_row_chars: width,
        }
    }
}

impl From<MBTextSummary> for TextSummary {
    fn from(summary: MBTextSummary) -> Self {
        Self {
            lines: summary.lines,
            first_line_chars: summary.first_line_chars,
            last_line_chars: summary.last_line_chars,
            longest_row: summary.longest_row,
            longest_row_chars: summary.longest_row_chars,
        }
    }
}

impl<'a> From<&'a str> for TextSummary {
    #[ztracing::instrument(skip_all)]
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
    #[ztracing::instrument(skip_all)]
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

impl sum_tree::ContextLessSummary for TextSummary {
    fn zero() -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        *self += summary;
    }
}

pub struct TabChunks<'a> {
    snapshot: &'a TabSnapshot,
    max_output_position: Point,
    tab_size: NonZeroU32,
    // region: iteration state
    fold_chunks: FoldChunks<'a>,
    chunk: Chunk<'a>,
    column: u32,
    output_position: Point,
    leading_tab_width: u32,
    // endregion: iteration state
}

impl TabChunks<'_> {
    #[ztracing::instrument(skip_all)]
    pub(crate) fn seek(&mut self, range: Range<TabPoint>) {
        let (input_start, expanded_char_column, to_next_stop) = self
            .snapshot
            .tab_point_to_fold_point(range.start, Bias::Left);
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
        self.column = expanded_char_column;
        self.output_position = range.start.0;
        self.max_output_position = range.end.0;
        self.chunk = Chunk::default();
        self.leading_tab_width = to_next_stop;
    }
}

impl<'a> Iterator for TabChunks<'a> {
    type Item = Chunk<'a>;

    #[ztracing::instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.output_position >= self.max_output_position {
            return None;
        }
        if self.snapshot.has_unrepresentable_rows() {
            return self.fold_chunks.next();
        }
        while self.chunk.text.is_empty() {
            self.chunk = self.fold_chunks.next()?;
        }

        if self.chunk.tabs & 1 != 0 {
            self.chunk.text = &self.chunk.text[1..];
            self.chunk.tabs >>= 1;
            self.chunk.chars >>= 1;
            self.chunk.newlines >>= 1;

            let mut len = if self.leading_tab_width > 0 {
                std::mem::take(&mut self.leading_tab_width)
            } else {
                self.tab_size.get() - self.column % self.tab_size.get()
            };
            let next_output_position = cmp::min(
                self.output_position + Point::new(0, len),
                self.max_output_position,
            );
            len = next_output_position.column - self.output_position.column;
            self.column += len;
            self.output_position = next_output_position;

            return Some(Chunk {
                text: unsafe { std::str::from_utf8_unchecked(&SPACES[..len as usize]) },
                is_tab: true,
                chars: u128::MAX.unbounded_shr(u128::BITS - len),
                tabs: 0,
                newlines: 0,
                ..self.chunk.clone()
            });
        }

        // Fast path: no tabs in the remaining chunk, return it directly
        if self.chunk.tabs == 0 {
            let chunk = self.chunk.clone();
            self.chunk.text = "";
            self.chunk.tabs = 0;
            self.chunk.chars = 0;
            self.chunk.newlines = 0;
            let chunk_len = chunk.text.len() as u32;

            let newline_count = chunk.newlines.count_ones();
            if newline_count > 0 {
                let last_newline_bit = 128 - chunk.newlines.leading_zeros();
                let chars_after_last_newline =
                    chunk.chars.unbounded_shr(last_newline_bit).count_ones();
                let bytes_after_last_newline = chunk_len - last_newline_bit;

                self.column = chars_after_last_newline;
                self.output_position = Point::new(
                    self.output_position.row + newline_count,
                    bytes_after_last_newline,
                );
            } else {
                let char_count = chunk.chars.count_ones();
                self.column += char_count;
                self.output_position.column += chunk_len;
            }

            return Some(chunk);
        }

        // Split at the next tab position
        let prefix_len = self.chunk.tabs.trailing_zeros() as usize;
        let (prefix, suffix) = self.chunk.text.split_at(prefix_len);

        let mask = 1u128.unbounded_shl(prefix_len as u32).wrapping_sub(1);
        let prefix_chars = self.chunk.chars & mask;
        let prefix_newlines = self.chunk.newlines & mask;

        self.chunk.text = suffix;
        self.chunk.tabs = self.chunk.tabs.unbounded_shr(prefix_len as u32);
        self.chunk.chars = self.chunk.chars.unbounded_shr(prefix_len as u32);
        self.chunk.newlines = self.chunk.newlines.unbounded_shr(prefix_len as u32);

        let newline_count = prefix_newlines.count_ones();
        if newline_count > 0 {
            let last_newline_bit = 128 - prefix_newlines.leading_zeros();
            let chars_after_last_newline =
                prefix_chars.unbounded_shr(last_newline_bit).count_ones();
            let bytes_after_last_newline = prefix_len as u32 - last_newline_bit;

            self.column = chars_after_last_newline;
            self.output_position = Point::new(
                self.output_position.row + newline_count,
                bytes_after_last_newline,
            );
        } else {
            let char_count = prefix_chars.count_ones();
            self.column += char_count;
            self.output_position.column += prefix_len as u32;
        }

        Some(Chunk {
            text: prefix,
            chars: prefix_chars,
            tabs: 0,
            newlines: prefix_newlines,
            ..self.chunk.clone()
        })
    }
}

#[derive(Clone)]
struct TabTransform {
    summary: TransformSummary,
    has_tabs: bool,
}

impl TabTransform {
    fn isomorphic(summary: MBTextSummary, first_line_bytes: u32) -> Self {
        Self {
            summary: TransformSummary {
                input: summary.lines,
                input_bytes: summary.len,
                output: TextSummary::from(summary),
                first_line_bytes,
            },
            has_tabs: false,
        }
    }
}

impl sum_tree::Item for TabTransform {
    type Summary = TransformSummary;

    fn summary(&self, _: ()) -> TransformSummary {
        self.summary.clone()
    }
}

#[derive(Clone, Default)]
struct TransformSummary {
    input: Point,
    input_bytes: MultiBufferOffset,
    output: TextSummary,
    first_line_bytes: u32,
}

impl sum_tree::ContextLessSummary for TransformSummary {
    fn zero() -> Self {
        Self::default()
    }

    fn add_summary(&mut self, other: &Self) {
        if self.output.lines.row == 0 {
            self.first_line_bytes += other.first_line_bytes;
        }
        self.input += other.input;
        self.input_bytes += other.input_bytes;
        self.output += &other.output;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for FoldOffset {
    fn zero(_: ()) -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        self.0 += summary.input_bytes;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for FoldPoint {
    fn zero(_: ()) -> Self {
        Self::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        self.0 += summary.input;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for TabPoint {
    fn zero(_: ()) -> Self {
        Self::zero()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        self.0 += summary.output.lines;
    }
}

fn push_transform(transforms: &mut SumTree<TabTransform>, transform: TabTransform) {
    if transform.summary.input_bytes == MultiBufferOffset(0) {
        return;
    }
    if transforms
        .last()
        .is_some_and(|last| can_merge(last, &transform))
    {
        transforms.update_last(
            |last| {
                last.summary.add_summary(&transform.summary);
                last.has_tabs |= transform.has_tabs;
            },
            (),
        );
    } else {
        transforms.push(transform, ());
    }
}

fn append_transforms(transforms: &mut SumTree<TabTransform>, suffix: SumTree<TabTransform>) {
    if let Some((last, first)) = transforms.last().zip(suffix.first())
        && can_merge(last, first)
    {
        transforms.update_last(
            |last| {
                last.summary.add_summary(&first.summary);
                last.has_tabs |= first.has_tabs;
            },
            (),
        );
        let mut cursor = suffix.cursor::<FoldOffset>(());
        cursor.seek(&FoldOffset(first.summary.input_bytes), Bias::Right);
        transforms.append(cursor.suffix(), ());
    } else {
        transforms.append(suffix, ());
    }
}

fn can_append(left: &TransformSummary, right: &TransformSummary, column_limit: u32) -> bool {
    u64::from(left.output.lines.column) + u64::from(right.first_line_bytes)
        <= u64::from(column_limit)
}

fn can_merge(left: &TabTransform, right: &TabTransform) -> bool {
    (!left.has_tabs && !right.has_tabs)
        || left.summary.input_bytes.0 + right.summary.input_bytes.0 <= rope::Chunk::MASK_BITS
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;
    use crate::{
        Inlay, MultiBuffer,
        display_map::{
            fold_map::{FoldMap, FoldPlaceholder},
            inlay_map::InlayMap,
        },
    };
    use multi_buffer::MultiBufferOffset;
    use project::InlayId;
    use rand::{Rng, prelude::StdRng};
    use util;

    #[test]
    fn test_expanded_byte_column_limit() {
        let suffix = TransformSummary {
            output: TextSummary::from("é\n"),
            first_line_bytes: 2,
            ..TransformSummary::default()
        };
        let mut prefix = TransformSummary {
            output: TextSummary::spaces(u32::MAX - 2),
            first_line_bytes: u32::MAX - 2,
            ..TransformSummary::default()
        };
        assert!(can_append(&prefix, &suffix, u32::MAX));
        sum_tree::ContextLessSummary::add_summary(&mut prefix, &suffix);
        assert_eq!(prefix.first_line_bytes, u32::MAX);
        assert_eq!(prefix.output.first_line_chars, u32::MAX - 1);
        assert_eq!(prefix.output.lines, Point::new(1, 0));
        let mut prefix = TransformSummary {
            output: TextSummary::spaces(u32::MAX - 1),
            first_line_bytes: u32::MAX - 1,
            ..TransformSummary::default()
        };
        assert!(!can_append(&prefix, &suffix, u32::MAX));
        let suffix = TransformSummary {
            output: TextSummary::spaces(1),
            first_line_bytes: 1,
            ..TransformSummary::default()
        };
        assert!(can_append(&prefix, &suffix, u32::MAX));
        sum_tree::ContextLessSummary::add_summary(&mut prefix, &suffix);
        assert_eq!(prefix.output.lines, Point::new(0, u32::MAX));
        assert!(!can_append(&prefix, &suffix, u32::MAX));
    }

    #[gpui::test]
    fn test_unrepresentable_tab_rows_recover(cx: &mut gpui::App) {
        let input = format!("{}{}\nend", "\t".repeat(255), "x".repeat(127));
        let buffer = MultiBuffer::build_simple(&input, cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer.read(cx).snapshot(cx));
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let tab_size = NonZeroU32::new(128).unwrap();
        let (mut map, initial) = TabMap::new_with_column_limit(fold_snapshot, tab_size, 32_767);
        assert!(!initial.has_unrepresentable_rows());
        assert_eq!(initial.line_len(0), 32_767);
        let mut previous = initial.clone();
        for column_limit in [32_766, 32_767, 32_767] {
            let (snapshot, edits) = map.set_column_limit(column_limit);
            assert_eq!(snapshot.version, previous.version + 1);
            assert_eq!(map.0.version, snapshot.version);
            assert_eq!(snapshot.has_unrepresentable_rows(), column_limit < 32_767);
            assert_eq!(
                edits,
                vec![TabEdit {
                    old: TabPoint::zero()..previous.max_point(),
                    new: TabPoint::zero()..snapshot.max_point(),
                }]
            );
            previous = snapshot;
        }
        for (end, replacement, invalid) in [(382, "é", true), (384, "", false)] {
            let buffer_snapshot = buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(Point::new(0, 382)..Point::new(0, end), replacement)],
                    None,
                    cx,
                );
                buffer.snapshot(cx)
            });
            let (inlay_snapshot, inlay_edits) =
                inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
            let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
            let (snapshot, edits) = map.sync(fold_snapshot.clone(), fold_edits, tab_size);
            assert_eq!(snapshot.has_unrepresentable_rows(), invalid);
            assert_eq!(
                edits,
                vec![TabEdit {
                    old: TabPoint::zero()..previous.max_point(),
                    new: TabPoint::zero()..snapshot.max_point(),
                }]
            );
            if invalid {
                assert_eq!(snapshot.text(), snapshot.fold_snapshot.text());
                assert_eq!(snapshot.max_point().0, snapshot.fold_snapshot.max_point().0);
                assert_eq!(
                    snapshot.fold_point_to_tab_point(FoldPoint::new(0, 383)),
                    TabPoint::new(0, 384)
                );
                let (unchanged, edits) = map.sync(fold_snapshot.clone(), Vec::new(), tab_size);
                assert!(unchanged.has_unrepresentable_rows());
                assert!(edits.is_empty());
                for size in [1, 128] {
                    let (changed, edits) = map.sync(
                        fold_snapshot.clone(),
                        Vec::new(),
                        NonZeroU32::new(size).unwrap(),
                    );
                    assert_eq!(changed.has_unrepresentable_rows(), size == 128);
                    assert_eq!(edits.len(), 1);
                    if size == 1 {
                        assert_row_summaries(&changed);
                    }
                }
            } else {
                assert_row_summaries(&snapshot);
                assert_eq!(snapshot.text(), initial.text());
            }
            previous = snapshot;
        }
        assert!(!initial.has_unrepresentable_rows());
        assert_eq!(initial.text(), expand_text(&input, 128));
    }

    #[gpui::test]
    fn test_text_summary_with_expanded_tabs(cx: &mut gpui::App) {
        let long_line = "é".repeat(65_536);
        for (tab_size, spaces_after_five) in [(1, 1), (4, 3), (128, 123)] {
            let spaces = " ".repeat(tab_size as usize);
            for (input, expected) in [
                (
                    format!("x\n{}END\n{}", "\t".repeat(128), "x".repeat(132)),
                    format!("x\n{}END\n{}", spaces.repeat(128), "x".repeat(132)),
                ),
                (
                    format!("x\n\t{long_line}\n{}\n", "x".repeat(65_538)),
                    format!("x\n{spaces}{long_line}\n{}\n", "x".repeat(65_538)),
                ),
                (
                    "x\naaaaa\t\nxxxxxxxx\n".to_string(),
                    format!("x\naaaaa{}\nxxxxxxxx\n", " ".repeat(spaces_after_five)),
                ),
                (
                    format!("x\na{}\t\n{}", "é".repeat(128), "x".repeat(131)),
                    format!(
                        "x\na{}{}\n{}",
                        "é".repeat(128),
                        " ".repeat(tab_size as usize - 129 % tab_size as usize),
                        "x".repeat(131)
                    ),
                ),
            ] {
                let buffer = MultiBuffer::build_simple(&input, cx);
                let (_, inlay_snapshot) = InlayMap::new(buffer.read(cx).snapshot(cx));
                let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
                let (_, snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(tab_size).unwrap());
                assert_eq!(
                    snapshot.text_summary(),
                    TextSummary::from(expected.as_str())
                );
                assert_eq!(
                    snapshot.text_summary_for_range(TabPoint::new(1, 1)..TabPoint::new(1, 1)),
                    TextSummary::default(),
                );
                assert_row_summaries(&snapshot);
            }
        }
        let buffer = MultiBuffer::build_simple(&format!("\t{}hello\n\t\n", "x".repeat(127)), cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer.read(cx).snapshot(cx));
        let (mut fold_map, _) = FoldMap::new(inlay_snapshot.clone());
        let (mut writer, _, _) = fold_map.write(inlay_snapshot.clone(), Vec::new());
        writer.fold(vec![(
            MultiBufferOffset(128)..MultiBufferOffset(133),
            FoldPlaceholder {
                collapsed_text: Some(gpui::SharedString::from("é".repeat(1024))),
                ..FoldPlaceholder::test()
            },
        )]);
        let (fold_snapshot, _) = fold_map.read(inlay_snapshot, Vec::new());
        let (_, snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(4).unwrap());
        assert_row_summaries(&snapshot);
    }

    #[gpui::test]
    fn test_exact_tab_index(cx: &mut gpui::App) {
        let prefix = "e\u{301}界🦀\u{200d}".repeat(8192);
        let input = format!("{prefix}\tZ{}\n{prefix}\tEND", "\t".repeat(256));
        let buffer = MultiBuffer::build_simple(&input, cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer.read(cx).snapshot(cx));
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut map, _) = TabMap::new(fold_snapshot.clone(), NonZeroU32::new(1).unwrap());
        for tab_size in 1..=128 {
            let (snapshot, _) = map.sync(
                fold_snapshot.clone(),
                Vec::new(),
                NonZeroU32::new(tab_size).unwrap(),
            );
            assert_row_summaries(&snapshot);
            if tab_size == 4 {
                for column in 0..=13 {
                    let point = FoldPoint::new(0, column);
                    assert_eq!(
                        snapshot.fold_point_to_tab_point(point),
                        snapshot.expected_to_tab_point(point)
                    );
                    for bias in [Bias::Left, Bias::Right] {
                        let point = TabPoint::new(0, column);
                        assert_eq!(
                            snapshot.tab_point_to_fold_point(point, bias),
                            snapshot.expected_to_fold_point(point, bias)
                        );
                    }
                }
            }
            let expected = expand_text(&input, tab_size);
            let expected = text::Rope::from(expected.as_str());
            for row in 0..=1 {
                let before_tab = FoldPoint::new(row, prefix.len() as u32);
                let after_tab = FoldPoint::new(row, prefix.len() as u32 + 1);
                let width = tab_size - (5 * 8192) % tab_size;
                assert_eq!(
                    snapshot.fold_point_to_tab_point(before_tab),
                    TabPoint::new(row, prefix.len() as u32)
                );
                assert_eq!(
                    snapshot.fold_point_to_tab_point(after_tab),
                    TabPoint::new(row, prefix.len() as u32 + width)
                );
                for delta in [0, width / 2, width] {
                    let start = TabPoint::new(row, prefix.len() as u32 + delta);
                    let end = TabPoint::new(row, prefix.len() as u32 + width + 1);
                    for bias in [Bias::Left, Bias::Right] {
                        assert_eq!(
                            snapshot.tab_point_to_fold_point(start, bias),
                            snapshot.expected_to_fold_point(start, bias)
                        );
                    }
                    let expected_text = expected
                        .chunks_in_range(
                            expected.point_to_offset(start.0)..expected.point_to_offset(end.0),
                        )
                        .collect::<String>();
                    let actual_text = snapshot
                        .chunks(
                            start..end,
                            LanguageAwareStyling {
                                tree_sitter: false,
                                diagnostics: false,
                            },
                            Highlights::default(),
                        )
                        .map(|chunk| chunk.text)
                        .collect::<String>();
                    assert_eq!(actual_text, expected_text);
                    assert_eq!(
                        snapshot.text_summary_for_range(start..end),
                        TextSummary::from(expected_text.as_str())
                    );
                }
            }
        }
    }

    #[gpui::test]
    fn test_sparse_tab_index_edits(cx: &mut gpui::App) {
        let prefix = "x".repeat(1_048_576);
        let input = format!("{prefix}\tZ{}\nTAIL\t", "\t".repeat(1024));
        let buffer = MultiBuffer::build_simple(&input, cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer.read(cx).snapshot(cx));
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let tab_size = NonZeroU32::new(128).unwrap();
        let (mut map, initial_snapshot) = TabMap::new(fold_snapshot, tab_size);
        let mut previous = initial_snapshot.clone();
        for edits in [
            vec![(Point::zero()..Point::zero(), "é")],
            vec![
                (Point::zero()..Point::new(0, 2), ""),
                (Point::new(0, 100)..Point::new(0, 100), "\t"),
            ],
            vec![(Point::zero()..Point::new(0, 99), "e\u{301}\n")],
            vec![(Point::new(0, 3)..Point::new(1, 0), "")],
            Vec::new(),
        ] {
            let buffer_snapshot = buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
                buffer.snapshot(cx)
            });
            let (inlay_snapshot, inlay_edits) =
                inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
            let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
            let (snapshot, edits) = map.sync(fold_snapshot, fold_edits, tab_size);
            assert!(snapshot.indexed_bytes <= 256);
            assert_row_summaries(&snapshot);
            let mut patched = text::Rope::from(previous.text().as_str());
            let expected = text::Rope::from(
                expand_text(&snapshot.fold_snapshot.text(), tab_size.get()).as_str(),
            );
            for edit in edits.iter().rev() {
                let replacement = expected
                    .chunks_in_range(
                        expected.point_to_offset(edit.new.start.0)
                            ..expected.point_to_offset(edit.new.end.0),
                    )
                    .collect::<String>();
                patched.replace(
                    patched.point_to_offset(edit.old.start.0)
                        ..patched.point_to_offset(edit.old.end.0),
                    &replacement,
                );
            }
            assert_eq!(patched.to_string(), expected.to_string());
            previous = snapshot;
        }
        assert_eq!(initial_snapshot.text(), expand_text(&input, tab_size.get()));
    }

    #[gpui::test]
    fn test_expand_tabs(cx: &mut gpui::App) {
        let input = "A\tBC\tDEF\tG\tHI\tJ\tK\tL\tM";

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        for (ix, _) in input.char_indices() {
            let fold_point = FoldPoint::new(0, ix as u32);

            assert_eq!(
                tab_snapshot.expected_to_tab_point(fold_point),
                tab_snapshot.fold_point_to_tab_point(fold_point),
                "Failed with fold_point at column {ix}"
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
        let text = "γ\tw⭐\n🍐🍗 \t";
        let buffer = MultiBuffer::build_simple(text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        let tab_point = TabPoint::new(0, 9);
        let result = tab_snapshot.tab_point_to_fold_point(tab_point, Bias::Left);
        let expected = tab_snapshot.expected_to_fold_point(tab_point, Bias::Left);

        assert_eq!(result, expected);
        for (input, output) in [(8, 10), (u32::MAX - 2, u32::MAX), (u32::MAX, u32::MAX)] {
            assert_eq!(
                tab_snapshot.fold_point_to_tab_point(FoldPoint::new(0, input)),
                TabPoint::new(0, output)
            );
        }
        assert_eq!(
            tab_snapshot
                .tab_point_to_fold_point(TabPoint::new(0, u32::MAX), Bias::Left)
                .0,
            FoldPoint::new(0, u32::MAX - 2)
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_collapse_tabs_random(cx: &mut gpui::App, mut rng: StdRng) {
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
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, tab_size);

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
            for chunk in snapshot.chunks(
                start..snapshot.max_point(),
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                Highlights::default(),
            ) {
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

    #[gpui::test]
    fn test_max_tab_size_full_chunks(cx: &mut gpui::App) {
        for tab_size in [127, 128, 129] {
            let buffer = MultiBuffer::build_simple("\tα\t🏀", cx);
            let buffer_snapshot = buffer.read(cx).snapshot(cx);
            let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
            let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
            let (_, tab_snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(tab_size).unwrap());
            let width = tab_size.min(128);
            let expected = format!(
                "{}α{}🏀",
                " ".repeat(width as usize),
                " ".repeat(width as usize - 1)
            );
            let mut chunks = tab_snapshot.chunks(
                TabPoint::zero()..tab_snapshot.max_point(),
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                Highlights::default(),
            );
            assert_max_tab_chunk_bitmaps(&mut chunks, &expected);
            assert_eq!(tab_snapshot.text(), expected);
            assert_eq!(
                tab_snapshot.point_to_tab_point(Point::new(0, 4), Bias::Left),
                TabPoint::new(0, 2 * width + 1),
            );
            assert_eq!(
                tab_snapshot.tab_point_to_point(TabPoint::new(0, 2 * width + 1), Bias::Left),
                Point::new(0, 4),
            );
        }
    }

    #[gpui::test]
    fn test_max_tab_size_partial_chunks_and_seek(cx: &mut gpui::App) {
        let buffer = MultiBuffer::build_simple("\t🏀", cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(128).unwrap());
        let expected = format!("{}🏀", " ".repeat(128));
        let mut reused = tab_snapshot.chunks(
            TabPoint::zero()..tab_snapshot.max_point(),
            LanguageAwareStyling {
                tree_sitter: false,
                diagnostics: false,
            },
            Highlights::default(),
        );
        for start in 0..=128 {
            for end in [start, (start + 1).min(128), 128, 132] {
                let range = TabPoint::new(0, start)..TabPoint::new(0, end);
                let mut chunks = tab_snapshot.chunks(
                    range.clone(),
                    LanguageAwareStyling {
                        tree_sitter: false,
                        diagnostics: false,
                    },
                    Highlights::default(),
                );
                assert_max_tab_chunk_bitmaps(&mut chunks, &expected[start as usize..end as usize]);
                reused.seek(range);
                assert_max_tab_chunk_bitmaps(&mut reused, &expected[start as usize..end as usize]);
            }
        }
    }

    #[gpui::test]
    fn test_empty_chunk_after_leading_tab_trim(cx: &mut gpui::App) {
        // We fold "hello" (offsets 1..6) so the fold map creates a
        // transform boundary at offset 1, producing a 1-byte fold chunk
        // for the tab.
        let text = "\thello";
        let buffer = MultiBuffer::build_simple(text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let mut fold_map = FoldMap::new(inlay_snapshot.clone()).0;

        let (mut writer, _, _) = fold_map.write(inlay_snapshot.clone(), vec![]);
        writer.fold(vec![(
            MultiBufferOffset(1)..MultiBufferOffset(6),
            FoldPlaceholder::test(),
        )]);
        let (fold_snapshot, _) = fold_map.read(inlay_snapshot, vec![]);

        let tab_size = NonZeroU32::new(4).unwrap();
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, tab_size);

        let max = tab_snapshot.max_point();
        let start = TabPoint::new(0, 2);
        let end = max;

        let result = tab_snapshot
            .chunks(
                start..end,
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                Highlights::default(),
            )
            .map(|c| c.text)
            .collect::<String>();
        assert_eq!(result, "  ⋯");
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

        let (_, tabs_snapshot) = TabMap::new(fold_snapshot, tab_size);

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
                    .chunks(
                        start..end,
                        LanguageAwareStyling {
                            tree_sitter: false,
                            diagnostics: false,
                        },
                        Highlights::default()
                    )
                    .map(|c| c.text)
                    .collect::<String>(),
                expected_text,
                "chunks({:?}..{:?})",
                start,
                end
            );

            assert_eq!(
                tabs_snapshot.text_summary_for_range(start..end),
                expected_summary
            );
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
        tab_map.sync(fold_snapshot.clone(), fold_edits, tab_size);

        // Test random fold points
        for _ in 0..50 {
            let tab_snapshot = &tab_map.0;
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

    #[gpui::test(iterations = 100)]
    fn test_random_tab_point_cursor(cx: &mut gpui::App, mut rng: StdRng) {
        let tab_size = NonZeroU32::new(rng.random_range(1..=16)).unwrap();
        let len = rng.random_range(0..=2000);
        let text = util::RandomCharIter::new(&mut rng)
            .take(len)
            .collect::<String>();

        let buffer = MultiBuffer::build_simple(&text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut tab_map, _) = TabMap::new(fold_snapshot, tab_size);

        let mut next_inlay_id = 0;
        let (inlay_snapshot, inlay_edits) = inlay_map.randomly_mutate(&mut next_inlay_id, &mut rng);
        let (mut fold_snapshot, mut fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
        tab_map.sync(fold_snapshot.clone(), fold_edits, tab_size);
        for _ in 0..rng.random_range(0..=2) {
            for (snapshot, edits) in fold_map.randomly_mutate(&mut rng) {
                fold_snapshot = snapshot;
                fold_edits = edits;
                tab_map.sync(fold_snapshot.clone(), fold_edits, tab_size);
            }
        }
        let tab_snapshot = &tab_map.0;
        log::info!("FoldMap text: {:?}", fold_snapshot.text());

        let max_fold_point = fold_snapshot.max_point();
        let mut fold_points = Vec::new();
        for _ in 0..50 {
            let row = rng.random_range(0..=max_fold_point.row());
            let max_column = if row < max_fold_point.row() {
                fold_snapshot.line_len(row)
            } else {
                max_fold_point.column()
            };
            let fold_point = FoldPoint::new(row, rng.random_range(0..=max_column + 10));
            fold_points.push(fold_point);
            if rng.random_bool(0.2) {
                fold_points.push(fold_point);
            }
        }
        fold_points.sort();

        let assert_maps = |cursor: &mut TabPointCursor, fold_point: FoldPoint, context: &str| {
            let actual = cursor.map(fold_point);
            assert_eq!(
                actual,
                tab_snapshot.expected_to_tab_point(fold_point),
                "{context}: cursor mismatch for {fold_point:?}"
            );
            assert_eq!(
                actual,
                tab_snapshot.fold_point_to_tab_point(fold_point),
                "{context}: cursor disagrees with fold_point_to_tab_point for {fold_point:?}"
            );
        };

        let mut cursor = tab_snapshot.tab_point_cursor();
        for fold_point in &fold_points {
            assert_maps(&mut cursor, *fold_point, "forward");
        }

        cursor.reset();
        for fold_point in &fold_points {
            assert_maps(&mut cursor, *fold_point, "after reset");
        }

        for _ in 0..20 {
            let ix = rng.random_range(0..fold_points.len());
            assert_maps(&mut cursor, fold_points[ix], "unordered");
        }
    }

    #[gpui::test]
    fn test_tab_map_sync_edits(cx: &mut gpui::App) {
        let buffer = MultiBuffer::build_simple("a\tb\nc\td\ne\tf\n", cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let tab_size = NonZeroU32::new(4).unwrap();
        let (mut tab_map, tab_snapshot) = TabMap::new(fold_snapshot, tab_size);
        assert_eq!(tab_snapshot.text(), "a   b\nc   d\ne   f\n");

        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    (Point::new(0, 0)..Point::new(0, 0), "x"),
                    (Point::new(1, 0)..Point::new(1, 0), "yz"),
                    (Point::new(2, 3)..Point::new(2, 3), "w"),
                ],
                None,
                cx,
            );
            buffer.snapshot(cx)
        });
        let (inlay_snapshot, inlay_edits) =
            inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
        let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
        let (tab_snapshot, tab_edits) = tab_map.sync(fold_snapshot, fold_edits, tab_size);
        assert_eq!(tab_snapshot.text(), "xa  b\nyzc d\ne   fw\n");
        assert_eq!(
            tab_edits,
            vec![
                TabEdit {
                    old: TabPoint::new(0, 0)..TabPoint::new(0, 4),
                    new: TabPoint::new(0, 0)..TabPoint::new(0, 4),
                },
                TabEdit {
                    old: TabPoint::new(1, 0)..TabPoint::new(1, 4),
                    new: TabPoint::new(1, 0)..TabPoint::new(1, 4),
                },
                TabEdit {
                    old: TabPoint::new(2, 5)..TabPoint::new(2, 5),
                    new: TabPoint::new(2, 5)..TabPoint::new(2, 6),
                },
            ]
        );
    }

    #[gpui::test]
    fn test_rendered_inlay_tab_index(cx: &mut gpui::App) {
        let prefix = "e\u{301}界🦀".repeat(33);
        let placeholder = format!("{prefix}\tA\n{}\tB", "é".repeat(129));
        let input = format!("L{placeholder}\tR");
        let buffer = MultiBuffer::build_simple("L\tR", cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut map, initial_snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(4).unwrap());
        let (inlay_snapshot, inlay_edits) = inlay_map.splice(
            &[],
            vec![Inlay::mock_repl_result(
                0,
                buffer_snapshot.anchor_before(Point::new(0, 1)),
                &placeholder,
            )],
        );
        let (fold_snapshot, mut fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
        for tab_size in [4, 128] {
            let (snapshot, _) = map.sync(
                fold_snapshot.clone(),
                mem::take(&mut fold_edits),
                NonZeroU32::new(tab_size).unwrap(),
            );
            assert_eq!(snapshot.fold_snapshot.text(), input);
            assert_row_summaries(&snapshot);
            for column in [2, 3, 5, 7, 8, 10, 127, 128, 129, 255, 256, 257] {
                assert_eq!(
                    snapshot.fold_point_to_tab_point(FoldPoint::new(0, column as u32)),
                    TabPoint::new(0, input.ceil_char_boundary(column) as u32),
                );
                for bias in [Bias::Left, Bias::Right] {
                    let boundary = if bias == Bias::Left {
                        input.floor_char_boundary(column)
                    } else {
                        input.ceil_char_boundary(column)
                    };
                    assert_eq!(
                        snapshot.tab_point_to_fold_point(TabPoint::new(0, column as u32), bias),
                        (
                            FoldPoint::new(0, boundary as u32),
                            input[..boundary].chars().count() as u32,
                            0
                        ),
                    );
                }
            }
            for bias in [Bias::Left, Bias::Right] {
                assert_eq!(
                    snapshot.clip_point(TabPoint(Point::MAX), bias),
                    snapshot.max_point()
                );
                assert_eq!(
                    snapshot
                        .tab_point_to_fold_point(TabPoint(Point::MAX), bias)
                        .0,
                    snapshot.fold_snapshot.max_point(),
                );
            }
            assert_eq!(
                snapshot.fold_point_to_tab_point(FoldPoint(Point::MAX)),
                snapshot.max_point()
            );

            let width = tab_size - 133 % tab_size;
            let start = TabPoint::new(0, prefix.len() as u32 + 2);
            let end = TabPoint::new(0, prefix.len() as u32 + 1 + width + 1);
            let expected = format!("{}A", " ".repeat(width as usize - 1));
            let mut chunks = snapshot.chunks(
                start..end,
                LanguageAwareStyling {
                    tree_sitter: false,
                    diagnostics: false,
                },
                Highlights::default(),
            );
            for iteration in 0..2 {
                if iteration > 0 {
                    chunks.seek(start..end);
                }
                let actual = chunks
                    .by_ref()
                    .map(|chunk| {
                        assert!(!chunk.text.is_empty());
                        assert!(
                            chunk.renderer.as_ref().map(|renderer| renderer.id)
                                == Some(fold_map::ChunkRendererId::Inlay(InlayId::ReplResult(0)))
                        );
                        chunk.text
                    })
                    .collect::<String>();
                assert_eq!(actual, expected);
            }
            chunks.seek(start..start);
            assert_eq!(chunks.next().map(|chunk| chunk.text), None);
        }
        assert_eq!(initial_snapshot.text(), "L   R");
    }

    #[gpui::test]
    fn test_tab_summary_inlay_fold_invalidation(cx: &mut gpui::App) {
        let input = format!(
            "{}{}界{}Z\n{}",
            "x\n".repeat(63),
            "é".repeat(63),
            "x".repeat(126),
            "x\n".repeat(65)
        );
        let buffer = MultiBuffer::build_simple(&input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, mut inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot.clone());
        let tab_size = NonZeroU32::new(4).unwrap();
        let (mut tab_map, initial_snapshot) = TabMap::new(fold_snapshot, tab_size);
        assert_row_summaries(&initial_snapshot);

        for (id, column, text) in [(0, 255, "\t"), (1, 126, "é\t")] {
            let (snapshot, inlay_edits) = inlay_map.splice(
                &[],
                vec![Inlay::mock_hint(
                    id,
                    buffer_snapshot.anchor_before(Point::new(63, column)),
                    text,
                )],
            );
            inlay_snapshot = snapshot;
            let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot.clone(), inlay_edits);
            let (snapshot, _) = tab_map.sync(fold_snapshot, fold_edits, tab_size);
            assert_row_summaries(&snapshot);
        }
        let (mut writer, _, _) = fold_map.write(inlay_snapshot.clone(), Vec::new());
        let (fold_snapshot, fold_edits) = writer.fold([(
            Point::new(63, 0)..Point::new(64, 0),
            FoldPlaceholder {
                collapsed_text: Some(gpui::SharedString::from(format!(
                    "{}\t\n{}\tEND",
                    "é".repeat(129),
                    "x".repeat(129)
                ))),
                ..FoldPlaceholder::test()
            },
        )]);
        let (snapshot, _) = tab_map.sync(fold_snapshot.clone(), fold_edits, tab_size);
        assert_row_summaries(&snapshot);
        let (_, rebuilt) = TabMap::new(fold_snapshot, tab_size);
        assert_eq!(rebuilt.text_summary(), snapshot.text_summary());

        let (mut writer, _, _) = fold_map.write(inlay_snapshot, Vec::new());
        let (fold_snapshot, fold_edits) =
            writer.unfold_intersecting([Point::new(63, 0)..Point::new(64, 0)], true);
        let (snapshot, _) = tab_map.sync(fold_snapshot, fold_edits, tab_size);
        assert_row_summaries(&snapshot);
        let (inlay_snapshot, inlay_edits) =
            inlay_map.splice(&[InlayId::Hint(0), InlayId::Hint(1)], Vec::new());
        let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
        let (snapshot, _) = tab_map.sync(fold_snapshot, fold_edits, tab_size);
        assert_row_summaries(&snapshot);
        assert_eq!(snapshot.text(), initial_snapshot.text());
        assert_eq!(snapshot.text_summary(), initial_snapshot.text_summary());
    }

    #[gpui::test(iterations = 100)]
    fn test_random_tab_map_sync(cx: &mut gpui::App, mut rng: StdRng) {
        let operations = std::env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut tab_size = NonZeroU32::new(rng.random_range(1..=4)).unwrap();
        let len = rng.random_range(0..200);
        let text = util::RandomCharIter::new(&mut rng)
            .take(len)
            .collect::<String>();
        let buffer = MultiBuffer::build_simple(&text, cx);
        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (mut fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (mut tab_map, initial_snapshot) = TabMap::new(fold_snapshot, tab_size);
        log::info!("TabMap text: {:?}", initial_snapshot.text());

        let mut next_inlay_id = 0;
        let mut snapshot_edits = Vec::new();
        for _ in 0..operations {
            let mut buffer_edits = Vec::new();
            match rng.random_range(0..100) {
                0..=9 => {
                    tab_size = NonZeroU32::new(rng.random_range(1..=4)).unwrap();
                    log::info!("Setting tab size to {tab_size}");
                }
                10..=39 => {
                    for (fold_snapshot, fold_edits) in fold_map.randomly_mutate(&mut rng) {
                        snapshot_edits.push(tab_map.sync(fold_snapshot, fold_edits, tab_size));
                    }
                }
                40..=59 => {
                    let (inlay_snapshot, inlay_edits) =
                        inlay_map.randomly_mutate(&mut next_inlay_id, &mut rng);
                    let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
                    snapshot_edits.push(tab_map.sync(fold_snapshot, fold_edits, tab_size));
                }
                _ => buffer.update(cx, |buffer, cx| {
                    let subscription = buffer.subscribe();
                    let edit_count = rng.random_range(1..=5);
                    buffer.randomly_mutate(&mut rng, edit_count, cx);
                    buffer_snapshot = buffer.snapshot(cx);
                    buffer_edits.extend(subscription.consume());
                }),
            }

            let (inlay_snapshot, inlay_edits) =
                inlay_map.sync(buffer_snapshot.clone(), buffer_edits);
            let (fold_snapshot, fold_edits) = fold_map.read(inlay_snapshot, inlay_edits);
            let (tab_snapshot, tab_edits) = tab_map.sync(fold_snapshot, fold_edits, tab_size);
            log::info!("TabMap text: {:?}", tab_snapshot.text());
            snapshot_edits.push((tab_snapshot, tab_edits));
        }

        let mut expected_text = text::Rope::from(initial_snapshot.text().as_str());
        for (tab_snapshot, tab_edits) in snapshot_edits {
            let snapshot_text = tab_snapshot.text();
            assert_row_summaries(&tab_snapshot);
            let snapshot_text = text::Rope::from(snapshot_text.as_str());
            for edit in &tab_edits {
                let old_start = expected_text.point_to_offset(edit.new.start.0);
                let old_end = expected_text
                    .point_to_offset(edit.new.start.0 + (edit.old.end.0 - edit.old.start.0));
                let new_start = snapshot_text.point_to_offset(edit.new.start.0);
                let new_end = snapshot_text.point_to_offset(edit.new.end.0);
                let new_text = snapshot_text
                    .chunks_in_range(new_start..new_end)
                    .collect::<String>();
                expected_text.replace(old_start..old_end, &new_text);
            }
            assert_eq!(
                expected_text.to_string(),
                snapshot_text.to_string(),
                "tab edits {tab_edits:?} do not transform the previous text into the new text"
            );
        }
    }

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
                if expanded_bytes >= column || c == '\n' {
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
            for c in chars {
                if collapsed_bytes >= column || c == '\n' {
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

    fn assert_row_summaries(snapshot: &TabSnapshot) {
        let expected = expand_text(&snapshot.fold_snapshot.text(), snapshot.tab_size.get());
        assert_eq!(snapshot.text(), expected);
        assert_eq!(
            snapshot.text_summary(),
            TextSummary::from(expected.as_str())
        );
        for (row, line) in expected.split('\n').enumerate() {
            let row = row as u32;
            assert_eq!(snapshot.line_len(row), line.len() as u32);
            assert_eq!(
                snapshot.text_summary_for_range(
                    TabPoint::new(row, 0)..TabPoint::new(row, line.len() as u32)
                ),
                TextSummary::from(line)
            );
        }
        let tab_count = snapshot
            .fold_snapshot
            .text()
            .bytes()
            .filter(|byte| *byte == b'\t')
            .count();
        assert!(snapshot.transforms.iter().count() <= 2 * tab_count + 1);
        assert!(
            snapshot.transforms.iter().count() <= 2 * (snapshot.fold_snapshot.len().0.0 / 129) + 1
        );
        let mut previous = None;
        for transform in snapshot.transforms.iter() {
            if transform.has_tabs {
                assert!(transform.summary.input_bytes.0 <= 128);
            }
            if let Some(previous) = previous {
                assert!(!can_merge(previous, transform));
            }
            previous = Some(transform);
        }
    }

    fn expand_text(input: &str, tab_size: u32) -> String {
        let mut output = String::new();
        let mut column = 0;
        for character in input.chars() {
            match character {
                '\t' => {
                    let end = (column / tab_size + 1) * tab_size;
                    output.extend(std::iter::repeat_n(' ', (end - column) as usize));
                    column = end;
                }
                '\n' => {
                    output.push(character);
                    column = 0;
                }
                _ => {
                    output.push(character);
                    column += 1;
                }
            }
        }
        output
    }

    fn assert_max_tab_chunk_bitmaps(chunks: &mut TabChunks<'_>, expected_text: &str) {
        let initial_mask = chunks
            .chunk
            .text
            .char_indices()
            .fold(0u128, |mask, (index, _)| mask | (1u128 << index));
        assert_eq!(chunks.chunk.chars, initial_mask);
        let text = chunks
            .map(|chunk| {
                let expected_mask = chunk
                    .text
                    .char_indices()
                    .fold(0u128, |mask, (index, _)| mask | (1u128 << index));
                assert_eq!(chunk.chars, expected_mask, "chunk {:?}", chunk.text);
                assert_eq!(chunk.tabs, 0);
                assert_eq!(chunk.newlines, 0);
                chunk.text
            })
            .collect::<String>();
        assert_eq!(text, expected_text);
    }
}
