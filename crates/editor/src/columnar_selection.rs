use language::{Bias, LanguageAwareStyling, Point};
use multi_buffer::{MBTextSummary, MultiBufferRow};
use std::ops::Range;
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

use crate::{
    DisplayPoint,
    display_map::{DisplaySnapshot, FoldPoint, Highlights, TabPoint, ToDisplayPoint as _},
};

pub(crate) struct ColumnarSelectionRows<'a> {
    snapshot: &'a DisplaySnapshot,
}

impl<'a> ColumnarSelectionRows<'a> {
    pub(crate) fn new(snapshot: &'a DisplaySnapshot) -> Self {
        Self { snapshot }
    }

    pub(crate) fn columns_for_ranges(&mut self, ranges: &[Range<Point>]) -> Vec<Range<u32>> {
        let tabs = self.snapshot.tab_snapshot();
        let mut endpoints = ranges
            .iter()
            .enumerate()
            .flat_map(|(index, range)| {
                [(range.start, false), (range.end, true)]
                    .map(|(point, end)| (tabs.point_to_tab_point(point, Bias::Left), index, end))
            })
            .collect::<Vec<_>>();
        endpoints.sort_unstable();
        let mut answers = vec![0..0; ranges.len()];
        for endpoints in endpoints.chunk_by(|left, right| left.0.row() == right.0.row()) {
            let mut text = self.row(endpoints[0].0.row());
            let mut cursor = text.forward_cursor();
            for &(point, index, end) in endpoints {
                text.floor_forward(&mut cursor, point.column());
                if end {
                    answers[index].end = cursor.column;
                } else {
                    answers[index].start = cursor.column;
                }
            }
        }
        for answer in &mut answers {
            *answer = answer.start.min(answer.end)..answer.start.max(answer.end);
        }
        answers
    }

    pub(crate) fn points_for_rows(
        &mut self,
        queries: &[(u32, Range<u32>)],
    ) -> Vec<Option<(Point, Point)>> {
        let tabs = self.snapshot.tab_snapshot();
        let max_row = tabs.max_point().row();
        let mut endpoints = Vec::new();
        for (index, (row, columns)) in queries.iter().enumerate() {
            if *row <= max_row && columns.start <= columns.end {
                endpoints.push((*row, columns.start, false, index));
                if columns.start != columns.end {
                    endpoints.push((*row, columns.end, true, index));
                }
            }
        }
        endpoints.sort_unstable();
        let mut answers = vec![None::<(Point, Point)>; queries.len()];
        for endpoints in endpoints.chunk_by(|left, right| left.0 == right.0) {
            let row = endpoints[0].0;
            let length = tabs.line_len(row);
            let mut text = self.row(row);
            let mut cursor = text.cursor(0);
            let mut column = 0;
            let mut preceding = None;
            let mut buffer_cursor = None;
            for &(_, goal, end, index) in endpoints {
                let columns = &queries[index].1;
                if (end && answers[index].is_none())
                    || (!end && columns.start != columns.end && goal >= length)
                {
                    continue;
                }
                text.advance_columns(&mut cursor, goal - column);
                column = goal;
                if !end && columns.start != columns.end && cursor.byte() == length {
                    continue;
                }
                let point = self.point_for_cursor(
                    row,
                    &mut text,
                    cursor.clone(),
                    preceding,
                    &mut buffer_cursor,
                );
                preceding = Some((cursor.byte(), point));
                answers[index] = if end {
                    answers[index]
                        .zip(point)
                        .map(|((start, _), end)| (start.min(end), start.max(end)))
                } else {
                    point.map(|point| (point, point))
                };
            }
        }
        answers
    }

    #[cfg(test)]
    pub(crate) fn columns_for_range(&mut self, range: Range<Point>) -> Range<u32> {
        self.columns_for_ranges(&[range]).pop().expect("one range")
    }

    #[cfg(test)]
    pub(crate) fn points_for_row(
        &mut self,
        row: u32,
        columns: &Range<u32>,
    ) -> Option<(Point, Point)> {
        self.points_for_rows(&[(row, columns.clone())])
            .pop()
            .flatten()
    }

    fn point_for_cursor(
        &self,
        row: u32,
        text: &mut RowText<'a>,
        mut cursor: BoundaryCursor,
        preceding: Option<(u32, Option<Point>)>,
        buffer_cursor: &mut Option<BufferRowCursor<'a>>,
    ) -> Option<Point> {
        let frontier = preceding.map_or(0, |(byte, _)| byte);
        let mut backward_buffer_cursor = None;
        if let Some(buffer_cursor) = buffer_cursor {
            buffer_cursor.reverse = None;
        }
        loop {
            if let Some((byte, point)) = preceding
                && cursor.byte() <= byte
            {
                return point;
            }
            let tab_point = TabPoint::new(row, cursor.byte());
            let tabs = self.snapshot.tab_snapshot();
            let fold_point = tabs.tab_point_to_fold_point(tab_point, Bias::Left).0;
            if let Some(range) = self
                .snapshot
                .fold_snapshot()
                .placeholder_range_at(fold_point)
                && range.start < fold_point
            {
                let start = tabs.fold_point_to_tab_point(range.start);
                text.retreat_to(&mut cursor, start.column().max(frontier));
                continue;
            }
            let point = tabs.tab_point_to_point(tab_point, Bias::Left);
            let display_point = point.to_display_point(self.snapshot);
            let hidden = self.snapshot.is_block_line(display_point.row());
            let canonical = tabs.point_to_tab_point(point, Bias::Left);
            if !hidden
                && canonical == tab_point
                && self.floor_buffer_point(buffer_cursor, &mut backward_buffer_cursor, point)
                    == point
            {
                return Some(point);
            }
            let preceding = if hidden && canonical == tab_point {
                let hidden_start = self.snapshot.display_point_to_fold_point(
                    DisplayPoint::new(display_point.row(), 0),
                    Bias::Left,
                );
                let hidden_start = tabs.fold_point_to_tab_point(hidden_start);
                if hidden_start.row() != row {
                    return None;
                }
                if hidden_start < tab_point {
                    hidden_start
                } else {
                    TabPoint::new(row, hidden_start.column().checked_sub(1)?)
                }
            } else if canonical < tab_point {
                canonical
            } else {
                let previous = if let Some(byte) = point.column.checked_sub(1) {
                    self.floor_buffer_point(
                        buffer_cursor,
                        &mut backward_buffer_cursor,
                        Point::new(point.row, byte),
                    )
                } else {
                    let row = point.row.checked_sub(1)?;
                    Point::new(
                        row,
                        self.snapshot
                            .buffer_snapshot()
                            .line_len(MultiBufferRow(row)),
                    )
                };
                tabs.point_to_tab_point(previous, Bias::Left)
            };
            if preceding.row() != row {
                return None;
            }
            if preceding < tab_point {
                text.retreat_to(&mut cursor, preceding.column().max(frontier));
            } else {
                text.previous_boundary(&mut cursor)?;
            }
        }
    }

    fn floor_buffer_point(
        &self,
        forward: &mut Option<BufferRowCursor<'a>>,
        backward: &mut Option<BufferRowCursor<'a>>,
        point: Point,
    ) -> Point {
        if forward.as_ref().is_none_or(|cursor| cursor.row < point.row) {
            *backward = forward.replace(self.buffer_row_cursor(point.row));
        }
        let cursor = if let Some(cursor) = forward.as_mut().filter(|cursor| cursor.row == point.row)
        {
            cursor
        } else {
            if backward
                .as_ref()
                .is_none_or(|cursor| cursor.row != point.row)
            {
                let cursor = backward.insert(self.buffer_row_cursor(point.row));
                cursor.reverse = Some(cursor.text.floor_at(point.column));
            }
            backward.as_mut().expect("backward buffer row initialized")
        };
        if cursor.reverse.is_none() {
            cursor.text.floor_forward(&mut cursor.forward, point.column);
            cursor.reverse = Some(cursor.forward.boundary.clone());
        }
        let boundary = cursor
            .reverse
            .as_mut()
            .expect("buffer boundary initialized");
        cursor.text.retreat_to(boundary, point.column);
        Point::new(point.row, boundary.byte())
    }

    fn buffer_row_cursor(&self, row: u32) -> BufferRowCursor<'a> {
        let text = self.buffer_row(row);
        let forward = text.forward_cursor();
        BufferRowCursor {
            row,
            text,
            forward,
            reverse: None,
        }
    }

    fn buffer_row(&self, row: u32) -> RowText<'a> {
        let buffer = self.snapshot.buffer_snapshot();
        let length = buffer.line_len(MultiBufferRow(row));
        let range = Point::new(row, 0)..Point::new(row, length);
        let summary = buffer.text_summary_for_range::<MBTextSummary, _>(range.clone());
        if summary.len.0 == summary.chars {
            RowText::Ascii(length)
        } else {
            RowText::new(length, buffer.text_for_range(range))
        }
    }

    fn row(&self, row: u32) -> RowText<'a> {
        let tabs = self.snapshot.tab_snapshot();
        let folds = self.snapshot.fold_snapshot();
        let summary = folds.text_summary_for_range(
            FoldPoint::new(row, 0)..FoldPoint::new(row, folds.line_len(row)),
        );
        let length = tabs.line_len(row);
        if summary.len.0 == summary.chars {
            RowText::Ascii(length)
        } else {
            let chunks = tabs
                .chunks(
                    TabPoint::new(row, 0)..TabPoint::new(row, length),
                    LanguageAwareStyling {
                        tree_sitter: false,
                        diagnostics: false,
                    },
                    Highlights::default(),
                )
                .map(|chunk| (chunk.text, chunk.is_tab));
            RowText::with_tabs(length, chunks)
        }
    }
}

enum RowText<'a> {
    Ascii(u32),
    Unicode(RowChunks<'a>),
}

struct RowChunks<'a> {
    length: u32,
    chunks: Vec<TextChunk<'a>>,
    remaining: Box<dyn Iterator<Item = (&'a str, bool)> + 'a>,
    #[cfg(test)]
    context_bytes: usize,
}

#[derive(Clone, Copy)]
struct TextChunk<'a> {
    start: usize,
    text: &'a str,
}

#[derive(Clone)]
struct BoundaryCursor {
    graphemes: GraphemeCursor,
    chunk_index: usize,
}

struct ForwardCursor {
    boundary: BoundaryCursor,
    next: BoundaryCursor,
    column: u32,
}

struct BufferRowCursor<'a> {
    row: u32,
    text: RowText<'a>,
    forward: ForwardCursor,
    reverse: Option<BoundaryCursor>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BoundaryQuery {
    Next,
    Previous,
    Check,
}

impl<'a> RowText<'a> {
    fn new(length: u32, chunks: impl Iterator<Item = &'a str> + 'a) -> Self {
        Self::with_tabs(length, chunks.map(|text| (text, false)))
    }

    fn with_tabs(length: u32, chunks: impl Iterator<Item = (&'a str, bool)> + 'a) -> Self {
        Self::Unicode(RowChunks {
            length,
            chunks: Vec::new(),
            remaining: Box::new(chunks),
            #[cfg(test)]
            context_bytes: 0,
        })
    }

    fn cursor(&self, byte: u32) -> BoundaryCursor {
        let length = match self {
            Self::Ascii(length) => *length,
            Self::Unicode(chunks) => chunks.length,
        };
        BoundaryCursor {
            graphemes: GraphemeCursor::new(byte.min(length) as usize, length as usize, true),
            chunk_index: 0,
        }
    }

    fn forward_cursor(&self) -> ForwardCursor {
        let boundary = self.cursor(0);
        ForwardCursor {
            next: boundary.clone(),
            boundary,
            column: 0,
        }
    }

    fn floor_forward(&mut self, cursor: &mut ForwardCursor, byte: u32) {
        debug_assert!(byte >= cursor.boundary.byte());
        if let Self::Ascii(length) = self {
            cursor.column = byte.min(*length);
            cursor.boundary.graphemes.set_cursor(cursor.column as usize);
            return;
        }
        while cursor.boundary.byte() < byte {
            if cursor.next.byte() == cursor.boundary.byte()
                && self.next_boundary(&mut cursor.next).is_none()
            {
                break;
            }
            if cursor.next.byte() > byte {
                break;
            }
            cursor.boundary = cursor.next.clone();
            cursor.column += 1;
        }
    }

    #[cfg(test)]
    fn columns_for_bytes(&mut self, bytes: Range<u32>) -> Range<u32> {
        let mut cursor = self.forward_cursor();
        self.floor_forward(&mut cursor, bytes.start);
        let start = cursor.column;
        self.floor_forward(&mut cursor, bytes.end);
        start..cursor.column
    }

    fn advance_columns(&mut self, cursor: &mut BoundaryCursor, count: u32) {
        if let Self::Ascii(length) = self {
            cursor
                .graphemes
                .set_cursor(cursor.byte().saturating_add(count).min(*length) as usize);
        } else {
            for _ in 0..count {
                if self.next_boundary(cursor).is_none() {
                    break;
                }
            }
        }
    }

    fn floor_at(&mut self, byte: u32) -> BoundaryCursor {
        let mut cursor = self.cursor(byte);
        if let Self::Unicode(chunks) = self
            && cursor.byte() > 0
            && cursor.byte() < chunks.length
        {
            cursor.chunk_index = chunks.locate(cursor.byte() as usize);
            let chunk = chunks.chunk(cursor.chunk_index);
            let mut offset = cursor.byte() as usize - chunk.start;
            while !chunk.text.is_char_boundary(offset) {
                offset -= 1;
            }
            cursor.graphemes.set_cursor(chunk.start + offset);
            if chunks.resolve(&mut cursor, BoundaryQuery::Check).is_none() {
                chunks
                    .resolve(&mut cursor, BoundaryQuery::Previous)
                    .expect("non-boundary has a preceding boundary");
            }
        }
        cursor
    }

    fn retreat_to(&mut self, cursor: &mut BoundaryCursor, byte: u32) {
        if let Self::Ascii(_) = self {
            cursor
                .graphemes
                .set_cursor(cursor.byte().min(byte) as usize);
        } else {
            while cursor.byte() > byte {
                if self.previous_boundary(cursor).is_none() {
                    break;
                }
            }
        }
    }

    fn next_boundary(&mut self, cursor: &mut BoundaryCursor) -> Option<u32> {
        match self {
            Self::Ascii(length) => {
                if cursor.byte() >= *length {
                    return None;
                }
                cursor.graphemes.set_cursor(cursor.byte() as usize + 1);
                Some(cursor.byte())
            }
            Self::Unicode(chunks) => chunks.resolve(cursor, BoundaryQuery::Next),
        }
    }

    fn previous_boundary(&mut self, cursor: &mut BoundaryCursor) -> Option<u32> {
        match self {
            Self::Ascii(_) => {
                let byte = cursor.byte().checked_sub(1)?;
                cursor.graphemes.set_cursor(byte as usize);
                Some(byte)
            }
            Self::Unicode(chunks) => chunks.resolve(cursor, BoundaryQuery::Previous),
        }
    }
}

impl<'a> RowChunks<'a> {
    fn chunk(&mut self, index: usize) -> TextChunk<'a> {
        const TAB_CHARACTERS: [u8; 128] = [b'\t'; 128];
        while self.chunks.len() <= index {
            let (text, is_tab) = self
                .remaining
                .find(|(text, _)| !text.is_empty())
                .expect("row chunks must cover the row");
            let text = if is_tab {
                std::str::from_utf8(&TAB_CHARACTERS[..text.len()]).expect("tab controls are UTF-8")
            } else {
                text
            };
            let start = self
                .chunks
                .last()
                .map_or(0, |chunk| chunk.start + chunk.text.len());
            self.chunks.push(TextChunk { start, text });
            #[cfg(test)]
            tests::ROW_WORK.with(|work| {
                let (boundaries, chunks) = work.get();
                work.set((boundaries, chunks + 1));
            });
        }
        self.chunks[index]
    }

    fn locate(&mut self, byte: usize) -> usize {
        while self
            .chunks
            .last()
            .is_none_or(|chunk| chunk.start + chunk.text.len() <= byte)
        {
            self.chunk(self.chunks.len());
        }
        self.chunks.partition_point(|chunk| chunk.start <= byte) - 1
    }

    fn resolve(&mut self, cursor: &mut BoundaryCursor, query: BoundaryQuery) -> Option<u32> {
        #[cfg(test)]
        tests::ROW_WORK.with(|work| {
            let (boundaries, chunks) = work.get();
            work.set((boundaries + 1, chunks));
        });
        loop {
            let offset = cursor.graphemes.cur_cursor();
            if query == BoundaryQuery::Check && (offset == 0 || offset == self.length as usize) {
                return Some(offset as u32);
            }
            if (query == BoundaryQuery::Next && offset == self.length as usize)
                || (query == BoundaryQuery::Previous && offset == 0)
            {
                return None;
            }
            let chunk = loop {
                let chunk = self.chunk(cursor.chunk_index);
                if offset < chunk.start
                    || (query == BoundaryQuery::Previous && offset == chunk.start)
                {
                    cursor.chunk_index -= 1;
                } else if offset > chunk.start + chunk.text.len()
                    || (query != BoundaryQuery::Previous
                        && offset == chunk.start + chunk.text.len())
                {
                    cursor.chunk_index += 1;
                } else {
                    break chunk;
                }
            };
            let mut bridge = [0; 8];
            let (text, start) =
                if query != BoundaryQuery::Previous && offset == chunk.start && offset > 0 {
                    let previous = self.chunk(cursor.chunk_index - 1);
                    let previous = previous.text.chars().next_back().expect("nonempty chunk");
                    let following = chunk.text.chars().next().expect("nonempty chunk");
                    let previous_length = previous.encode_utf8(&mut bridge).len();
                    let following_length =
                        following.encode_utf8(&mut bridge[previous_length..]).len();
                    (
                        std::str::from_utf8(&bridge[..previous_length + following_length])
                            .expect("encoded scalars are UTF-8"),
                        offset - previous_length,
                    )
                } else {
                    (chunk.text, chunk.start)
                };
            loop {
                let result = match query {
                    BoundaryQuery::Next => cursor.graphemes.next_boundary(text, start),
                    BoundaryQuery::Previous => cursor.graphemes.prev_boundary(text, start),
                    BoundaryQuery::Check => cursor
                        .graphemes
                        .is_boundary(text, start)
                        .map(|boundary| boundary.then_some(cursor.graphemes.cur_cursor())),
                };
                match result {
                    Ok(boundary) => return boundary.map(|byte| byte as u32),
                    Err(GraphemeIncomplete::PreContext(end)) => {
                        let index = self.locate(end - 1);
                        let context = self.chunk(index);
                        let context_text = &context.text[..end - context.start];
                        #[cfg(test)]
                        {
                            self.context_bytes += context_text.len();
                        }
                        cursor
                            .graphemes
                            .provide_context(context_text, context.start);
                    }
                    Err(GraphemeIncomplete::NextChunk | GraphemeIncomplete::PrevChunk) => break,
                    Err(error) => unreachable!("row grapheme cursor: {error:?}"),
                }
            }
        }
    }
}

impl BoundaryCursor {
    fn byte(&self) -> u32 {
        self.graphemes.cur_cursor() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AddSelectionAbove, AddSelectionBelow, SelectionEffects,
        display_map::{
            BlockPlacement, BlockProperties, BlockStyle, Crease, DisplayMap, FoldPlaceholder,
        },
        inlays::Inlay,
        test::{editor_test_context::EditorTestContext, test_font},
    };
    use collections::HashSet;
    use gpui::{
        AppContext as _, BorrowAppContext as _, Element as _, Entity, Pixels, SharedString,
        TestAppContext, div, px,
    };
    use language::{Buffer, Capability};
    use multi_buffer::{Anchor, ExcerptRange, MultiBuffer, MultiBufferOffset, PathKey};
    use project::project_settings::DiagnosticSeverity;
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::{cell::Cell, env, iter, num::NonZeroU32, sync::Arc};
    use theme::LoadThemes;
    use unicode_segmentation::UnicodeSegmentation as _;

    thread_local! {
        pub(super) static ROW_WORK: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
    }

    #[gpui::test]
    async fn test_columnar_selection_actions_batch_work(cx: &mut TestAppContext) {
        crate::editor_tests::init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let rows = [("é.", 3, 2), ("e\u{301}.", 4, 3), ("🦀.", 5, 4)];
        for count in [32, 128] {
            let text = rows
                .iter()
                .map(|(unit, _, _)| unit.repeat(count))
                .collect::<Vec<_>>()
                .join("\n");
            for above in [false, true] {
                for (nonempty, reversed) in [(false, false), (true, false), (true, true)] {
                    let indices = if nonempty { 0..count } else { 1..count + 1 };
                    let expected = |added| {
                        rows.iter()
                            .enumerate()
                            .filter(|(row, _)| {
                                if above {
                                    *row >= 2 - added
                                } else {
                                    *row <= added
                                }
                            })
                            .flat_map(|(row, &(_, stride, width))| {
                                separated_columnar_ranges(
                                    row as u32,
                                    indices.clone(),
                                    stride,
                                    if nonempty { width } else { 0 },
                                    reversed,
                                )
                            })
                            .collect::<Vec<_>>()
                    };
                    set_columnar_action_state(&mut cx, &text, &expected(0));
                    for (direction, added) in [
                        (above, 1),
                        (above, 2),
                        (above, 2),
                        (!above, 1),
                        (!above, 0),
                        (above, 1),
                    ] {
                        let expected = expected(added);
                        assert_eq!(expected.len(), (added + 1) * count);
                        assert_columnar_action_work(&mut cx, direction, &expected, 4 * (2 * count));
                    }
                }
            }
        }
    }

    #[gpui::test]
    async fn test_columnar_selection_actions_join_skipped_rows_work(cx: &mut TestAppContext) {
        crate::editor_tests::init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        for count in [32, 128] {
            for above in [false, true] {
                let row = |row| if above { 6 - row } else { row };
                let mut lines = [
                    "é.".repeat(count + 1),
                    "é".to_owned(),
                    "e\u{301}.".repeat(count / 2 + 1),
                    String::new(),
                    "🦀.".to_owned(),
                    "🦀.".repeat(count + 1),
                    "e\u{301}.".repeat(count + 1),
                ];
                if above {
                    lines.reverse();
                }
                let text = lines.join("\n");
                for reversed in [false, true] {
                    let sources =
                        separated_columnar_ranges(row(0), count / 2 + 1..count + 1, 3, 2, reversed)
                            .into_iter()
                            .chain(separated_columnar_ranges(
                                row(2),
                                1..count / 2 + 1,
                                4,
                                3,
                                reversed,
                            ))
                            .collect::<Vec<_>>();
                    let expected = |added| {
                        let mut expected = sources.clone();
                        for (target, stride, width) in
                            [(5, 5, 4), (6, 4, 3)].into_iter().take(added)
                        {
                            expected.extend(separated_columnar_ranges(
                                row(target),
                                1..count + 1,
                                stride,
                                width,
                                reversed,
                            ));
                        }
                        expected.sort_unstable_by_key(|range| range.start.min(range.end));
                        expected
                    };
                    set_columnar_action_state(&mut cx, &text, &expected(0));
                    for (direction, added) in [
                        (above, 1),
                        (above, 2),
                        (above, 2),
                        (!above, 1),
                        (!above, 0),
                        (above, 1),
                    ] {
                        let expected = expected(added);
                        assert_eq!(expected.len(), (added + 1) * count);
                        assert_columnar_action_work(
                            &mut cx,
                            direction,
                            &expected,
                            5 * (2 * (count + 1)),
                        );
                    }
                }
            }
        }
    }

    #[gpui::test]
    async fn test_columnar_selection_actions_skip_replaced_source_rows(cx: &mut TestAppContext) {
        crate::editor_tests::init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        for hidden_count in [1, 1000] {
            let text = iter::repeat_n("éab", hidden_count as usize + 3)
                .collect::<Vec<_>>()
                .join("\n");
            for above in [false, true] {
                let row = |row| if above { hidden_count + 2 - row } else { row };
                for ends_inside_replacement in [false, true] {
                    let end_row = hidden_count + u32::from(!ends_inside_replacement);
                    let source = Point::new(row(0).min(row(end_row)), 2)
                        ..Point::new(row(0).max(row(end_row)), 3);
                    set_columnar_action_state(&mut cx, &text, &[source]);
                    let blocks = cx.update_editor(|editor, _, cx| {
                        let buffer = editor.buffer.read(cx).snapshot(cx);
                        editor.insert_blocks(
                            [BlockProperties {
                                placement: BlockPlacement::Replace(
                                    buffer
                                        .anchor_after(Point::new(row(1).min(row(hidden_count)), 0))
                                        ..=buffer.anchor_before(Point::new(
                                            row(1).max(row(hidden_count)),
                                            4,
                                        )),
                                ),
                                height: Some(1),
                                style: BlockStyle::Fixed,
                                render: Arc::new(|_| div().into_any()),
                                priority: 0,
                            }],
                            None,
                            cx,
                        )
                    });
                    let mut expected = [0, hidden_count + 1, hidden_count + 2]
                        .into_iter()
                        .take(if ends_inside_replacement { 2 } else { 3 })
                        .map(|source_row| {
                            Point::new(row(source_row), 2)..Point::new(row(source_row), 3)
                        })
                        .collect::<Vec<_>>();
                    expected.sort_unstable_by_key(|range| range.start);
                    assert_columnar_action_work(&mut cx, above, &expected, 32);
                    cx.update_editor(|editor, _, cx| {
                        editor.remove_blocks(blocks.into_iter().collect(), None, cx);
                    });
                }
            }
        }
    }

    #[gpui::test]
    fn test_columnar_selection_batch_work(cx: &mut TestAppContext) {
        init_test(cx);
        let length = 4096;
        let line = "é".repeat(length);
        let snapshot = plain_state(&format!("{line}\n{line}"), cx)
            .build_map(cx)
            .update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        let mut ranges = (0..length as u32)
            .flat_map(|column| {
                (0..2)
                    .map(move |row| Point::new(row, column * 2)..Point::new(row, length as u32 * 2))
            })
            .collect::<Vec<_>>();
        let mut expected_columns = (0..length as u32)
            .flat_map(|column| [column..length as u32, column..length as u32])
            .collect::<Vec<_>>();
        for _ in 0..2 {
            ROW_WORK.set((0, 0));
            let answers = query.columns_for_ranges(&ranges);
            assert_eq!(answers.len(), expected_columns.len());
            for (index, (answer, expected)) in answers.iter().zip(&expected_columns).enumerate() {
                assert_eq!(answer, expected, "range {index}");
            }
            let (boundaries, chunks) = ROW_WORK.get();
            assert_eq!(boundaries, 2 * length);
            assert!(chunks <= 2 * length);
            ranges.reverse();
            expected_columns.reverse();
        }
        let requests = (0..length as u32)
            .flat_map(|column| (0..2).map(move |row| (row, column..length as u32)))
            .collect::<Vec<_>>();
        let expected_points = (0..length as u32)
            .flat_map(|column| {
                (0..2).map(move |row| {
                    Some((
                        Point::new(row, column * 2),
                        Point::new(row, length as u32 * 2),
                    ))
                })
            })
            .collect::<Vec<_>>();
        assert_batch_points(&mut query, requests, expected_points, 4 * length);
        assert_eq!(query.columns_for_ranges(&[]), Vec::new());
        assert_eq!(query.points_for_rows(&[]), Vec::new());
    }

    #[gpui::test]
    fn test_columnar_selection_batch_fold_projection(cx: &mut TestAppContext) {
        init_test(cx);
        let length = 128;
        let prefix = "é".repeat(length);
        let indicators = "🇦".repeat(length);
        let mut state = plain_state(&format!("{prefix}\n{indicators}\n{indicators}"), cx);
        let buffer = state.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        state.folds = vec![
            buffer.anchor_after(Point::new(0, length as u32 * 2))
                ..buffer.anchor_before(Point::new(1, 4)),
            buffer.anchor_after(Point::new(1, length as u32 * 4))
                ..buffer.anchor_before(Point::new(2, 4)),
        ];
        let snapshot = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
        let visible_indicators = "🇦".repeat(length - 1);
        assert_eq!(
            snapshot.tab_snapshot().text(),
            format!("{prefix}⋯{visible_indicators}⋯{visible_indicators}")
        );
        let end_column = length as u32 * 2 + 2;
        let end = Point::new(2, length as u32 * 4);
        let requests = (0..=end_column)
            .flat_map(|column| [(0, column..column), (0, column..end_column)])
            .collect::<Vec<_>>();
        let expected = (0..=end_column)
            .flat_map(|column| {
                let point = if column <= length as u32 {
                    Point::new(0, column * 2)
                } else if column <= length as u32 * 3 / 2 {
                    Point::new(0, length as u32 * 2)
                } else if column < end_column {
                    Point::new(1, length as u32 * 4)
                } else {
                    end
                };
                [Some((point, point)), Some((point, end))]
            })
            .collect::<Vec<_>>();
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_batch_points(&mut query, requests, expected, 16 * length);
    }

    #[gpui::test]
    fn test_columnar_selection_batch_rejects_before_end(cx: &mut TestAppContext) {
        init_test(cx);
        let mut state = plain_state("x\né\n", cx);
        let buffer = state.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        state.inlays.push(Inlay::mock_hint(
            0,
            buffer.anchor_before(Point::new(1, 0)),
            &format!("é{}\n", "a".repeat(1_048_576)),
        ));
        let snapshot = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        let mut requests = vec![(1, 0..u32::MAX); 4096];
        requests.extend([(2, 1..u32::MAX), (0, 2..u32::MAX), (u32::MAX, 0..0)]);
        for _ in 0..2 {
            ROW_WORK.set((0, 0));
            assert_eq!(query.points_for_rows(&requests), vec![None; requests.len()]);
            assert_eq!(ROW_WORK.get(), (1, 1));
            requests.reverse();
        }
    }

    #[gpui::test]
    fn test_columnar_selection_batch_virtual_frontier_work(cx: &mut TestAppContext) {
        init_test(cx);
        for (folded, length, prefix_length) in [(false, 4096, 1), (true, 64, 1), (true, 4096, 256)]
        {
            let prefix = "a".repeat(prefix_length);
            let placeholder = "é".repeat(length);
            let point = Point::new(0, prefix_length as u32);
            let mut state = plain_state(&format!("{prefix}xb"), cx);
            if !folded {
                let buffer = state.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
                state.inlays.push(Inlay::mock_hint(
                    0,
                    buffer.anchor_after(point),
                    &placeholder,
                ));
            }
            let map = state.build_map(cx);
            if folded {
                map.update(cx, |map, cx| {
                    map.fold(
                        vec![Crease::simple(
                            point..Point::new(0, point.column + 1),
                            FoldPlaceholder {
                                collapsed_text: Some(SharedString::from(placeholder.clone())),
                                ..FoldPlaceholder::test()
                            },
                        )],
                        cx,
                    )
                });
            }
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                snapshot.tab_snapshot().text(),
                if folded {
                    format!("{prefix}{placeholder}b")
                } else {
                    format!("{prefix}{placeholder}xb")
                }
            );
            let end = point.column + length as u32 - 1;
            let requests = (point.column..=end)
                .flat_map(|column| [(0, column..column), (0, column..end)])
                .collect::<Vec<_>>();
            let expected = vec![Some((point, point)); requests.len()];
            assert_batch_points(
                &mut ColumnarSelectionRows::new(&snapshot),
                requests,
                expected,
                8 * length,
            );
        }
    }

    #[gpui::test]
    fn test_columnar_selection_batch_frontier_across_original_rows(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        init_test(cx);
        let lines = [
            "ééé",
            "🇦🇶🇦🇶🇦🇶🇦",
            "🇦🇶🇦🇶🇦🇶🇦🇶",
            "\t\u{301}界",
            "\u{600}e\u{301}",
            "👩🏽\u{200d}💻",
            "क्\u{200d}ष",
            "\u{1100}\u{1161}\u{11a8}",
            "\u{301}\u{301}",
            "a\0\u{200b}b",
            "é界🏀",
            "✈\u{fe0f}1\u{fe0f}\u{20e3}",
            "का",
            "",
        ];
        for tab_size in [1, 128] {
            set_tab_size(tab_size, cx);
            for bias in [None, Some(Bias::Left), Some(Bias::Right)] {
                let mut state = plain_state(&lines.join("\n"), cx);
                let buffer = state.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
                for (row, line) in lines.iter().enumerate().skip(1) {
                    let Some(first) = line.chars().next() else {
                        continue;
                    };
                    let point = Point::new(row as u32, first.len_utf8() as u32);
                    if row % 4 != 0 {
                        state.folds.push(
                            buffer.anchor_after(Point::new(
                                row as u32 - 1,
                                lines[row - 1].len() as u32,
                            ))..buffer.anchor_before(point),
                        );
                    }
                    if let Some(bias) = bias {
                        state.inlays.push(Inlay::mock_hint(
                            row,
                            buffer.anchor_at(point, bias),
                            ["🇶", "\u{301}", "\t\n\u{600}"][row % 3],
                        ));
                    }
                }
                let snapshot = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
                let expanded = snapshot
                    .fold_snapshot()
                    .text()
                    .split('\n')
                    .map(|line| plain_row(line, tab_size))
                    .collect::<Vec<_>>();
                assert_eq!(
                    snapshot.tab_snapshot().text(),
                    expanded
                        .iter()
                        .map(|row| row.text.as_str())
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                let raw = snapshot.buffer_snapshot().text();
                let candidates = expanded
                    .iter()
                    .enumerate()
                    .map(|(row, expanded)| {
                        enumerated_points(&snapshot, &raw, row as u32, &expanded.boundaries)
                    })
                    .collect::<Vec<_>>();
                let mut requests = expanded
                    .iter()
                    .enumerate()
                    .flat_map(|(row, expanded)| {
                        (0..=expanded.len + 1)
                            .chain(iter::once(u32::MAX))
                            .flat_map(move |start| {
                                [0, start, start.saturating_add(1), expanded.len, u32::MAX]
                                    .map(|end| (row as u32, start..end))
                            })
                    })
                    .collect::<Vec<_>>();
                requests.extend([(u32::MAX, 0..0), (expanded.len() as u32, 0..1)]);
                requests.extend_from_within(..);
                requests.shuffle(&mut rng);
                let mut query = ColumnarSelectionRows::new(&snapshot);
                for _ in 0..4 {
                    let answers = query.points_for_rows(&requests);
                    assert_eq!(answers.len(), requests.len());
                    for ((row, columns), answer) in requests.iter().zip(answers) {
                        let expected = expanded.get(*row as usize).and_then(|expanded| {
                            nearest_points(columns, expanded.len, &candidates[*row as usize])
                        });
                        assert_eq!(
                            answer, expected,
                            "tab size {tab_size}, bias {bias:?}, row {row}, columns {columns:?}"
                        );
                    }
                    requests.reverse();
                    requests = requests.into_iter().step_by(7).collect();
                }
                let points = lines
                    .iter()
                    .enumerate()
                    .flat_map(|(row, line)| {
                        scalar_boundaries(line)
                            .into_iter()
                            .map(move |byte| Point::new(row as u32, byte as u32))
                    })
                    .collect::<Vec<_>>();
                let mut ranges = points
                    .iter()
                    .flat_map(|start| points.iter().map(move |end| *start..*end))
                    .collect::<Vec<_>>();
                ranges.extend_from_within(..);
                ranges.shuffle(&mut rng);
                let answers = query.columns_for_ranges(&ranges);
                assert_eq!(answers.len(), ranges.len());
                for (range, answer) in ranges.iter().zip(answers) {
                    let column = |point| {
                        let tab = snapshot
                            .tab_snapshot()
                            .point_to_tab_point(point, Bias::Left);
                        expanded[tab.row() as usize]
                            .boundaries
                            .partition_point(|byte| *byte <= tab.column() as usize)
                            as u32
                            - 1
                    };
                    let start = column(range.start);
                    let end = column(range.end);
                    assert_eq!(
                        answer,
                        start.min(end)..start.max(end),
                        "tab size {tab_size}, bias {bias:?}, source {range:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_column_boundaries_across_chunks() {
        for (text, expected) in [
            ("", vec![0]),
            ("abc", vec![0, 1, 2, 3]),
            ("e\u{301}x", vec![0, 3, 4]),
            ("\u{301}\u{301}x", vec![0, 4, 5]),
            ("👩\u{200d}💻x", vec![0, 11, 12]),
            ("🇦🇶🇦🇶🇦x", vec![0, 8, 16, 20, 21]),
            ("क्\u{200d}षx", vec![0, 12, 13]),
            ("\r\nx", vec![0, 2, 3]),
            ("\u{600}🇦🇶🇦x", vec![0, 10, 14, 15]),
            ("🇦\u{301}🇶🇦x", vec![0, 6, 14, 15]),
        ] {
            for split in scalar_boundaries(text) {
                let mut query = RowText::new(
                    text.len() as u32,
                    ["", &text[..split], "", &text[split..], ""].into_iter(),
                );
                assert_row_boundaries(&mut query, &expected);
            }
            let boundaries = scalar_boundaries(text);
            let mut query = RowText::new(
                text.len() as u32,
                boundaries.windows(2).map(|range| &text[range[0]..range[1]]),
            );
            assert_row_boundaries(&mut query, &expected);
            for start in 0..=text.len() {
                for end in start..=text.len() {
                    assert_eq!(
                        query.columns_for_bytes(start as u32..end as u32),
                        (expected.partition_point(|boundary| *boundary <= start) - 1) as u32
                            ..(expected.partition_point(|boundary| *boundary <= end) - 1) as u32,
                        "{text:?}, range {start}..{end}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_column_boundaries_tab_chunks() {
        let prefix = "\u{600}";
        let suffix = "\u{301}e\u{301}👩\u{200d}💻🇦🇶\0\u{200b}x";
        for width in [1, 4, 128] {
            let spaces = " ".repeat(width);
            let suffix_start = prefix.len() + width;
            let len = suffix_start + suffix.len();
            let expected = iter::once(0)
                .chain(prefix.len()..=suffix_start)
                .chain([2, 5, 16, 24, 25, 28, 29].map(|offset| suffix_start + offset))
                .collect::<Vec<_>>();
            for tab_split in 0..=width {
                for split in scalar_boundaries(suffix) {
                    let mut query = RowText::with_tabs(
                        len as u32,
                        [
                            (prefix, false),
                            (&spaces[..tab_split], true),
                            ("", false),
                            (&spaces[tab_split..], true),
                            (&suffix[..split], false),
                            ("", true),
                            (&suffix[split..], false),
                        ]
                        .into_iter(),
                    );
                    assert_row_boundaries(&mut query, &expected);
                }
            }
        }
    }

    #[test]
    fn test_column_boundaries_tab_before_large_grapheme() {
        let suffix = format!("{}z", "\u{301}".repeat(1_048_576));
        let mut query = RowText::with_tabs(
            4 + suffix.len() as u32,
            [("    ", true), (suffix.as_str(), false)].into_iter(),
        );
        let mut cursor = query.cursor(0);
        query.advance_columns(&mut cursor, 4);
        assert_eq!(cursor.byte(), 4);
        let RowText::Unicode(chunks) = &query else {
            panic!("Unicode row")
        };
        assert_eq!(chunks.chunks.len(), 2);
        assert_eq!(chunks.chunks[1].text.as_ptr(), suffix.as_ptr());
        assert_eq!(chunks.context_bytes, 0);
        assert_eq!(query.next_boundary(&mut cursor), Some(2_097_156));
        assert_eq!(query.next_boundary(&mut cursor), Some(2_097_157));
        assert_eq!(query.columns_for_bytes(2_097_155..2_097_155), 4..4);
        query.retreat_to(&mut cursor, 2_097_155);
        assert_eq!(cursor.byte(), 4);
    }

    #[test]
    fn test_row_cursor_forks_and_reverse_floors() {
        let text = "🇦🇶🇦🇶🇦e\u{301}👩\u{200d}💻क्\u{200d}षz";
        let expected = [0, 8, 16, 20, 23, 34, 46, 47];
        let boundaries = scalar_boundaries(text);
        let mut query = RowText::new(
            text.len() as u32,
            boundaries.windows(2).map(|range| &text[range[0]..range[1]]),
        );
        assert_row_boundaries(&mut query, &expected);
        let mut forward = query.cursor(0);
        for pair in expected.windows(2) {
            assert_eq!(forward.byte(), pair[0] as u32);
            let mut backward = forward.clone();
            for byte in (0..pair[0]).rev() {
                query.retreat_to(&mut backward, byte as u32);
                let index = expected.partition_point(|boundary| *boundary <= byte) - 1;
                assert_eq!(backward.byte(), expected[index] as u32);
            }
            assert_eq!(query.next_boundary(&mut forward), Some(pair[1] as u32));
            assert_eq!(backward.byte(), 0);
        }
    }

    #[test]
    fn test_row_cursor_giant_split_clusters() {
        for text in [
            format!("e{}z", "\u{301}".repeat(4096)),
            format!("👩{}\u{200d}💻z", "\u{301}".repeat(4096)),
            format!("क{}षz", "्\u{200d}".repeat(4096)),
        ] {
            let boundaries = scalar_boundaries(&text);
            let mut query = RowText::new(
                text.len() as u32,
                boundaries.windows(2).map(|range| &text[range[0]..range[1]]),
            );
            let end = text.len() as u32;
            let mut cursor = query.cursor(0);
            assert_eq!(query.next_boundary(&mut cursor), Some(end - 1));
            let mut fork = cursor.clone();
            assert_eq!(query.next_boundary(&mut cursor), Some(end));
            assert_eq!(query.previous_boundary(&mut fork), Some(0));
            for byte in [1, end / 2, end - 2] {
                assert_eq!(query.floor_at(byte).byte(), 0);
            }
            assert_eq!(query.floor_at(end - 1).byte(), end - 1);
            assert_eq!(query.previous_boundary(&mut cursor), Some(end - 1));
            assert_eq!(query.previous_boundary(&mut cursor), Some(0));
            let RowText::Unicode(chunks) = &query else {
                panic!("Unicode row")
            };
            assert_eq!(chunks.chunks.len(), boundaries.len() - 1);
            assert_eq!(chunks.chunks[0].text.as_ptr(), text.as_ptr());
        }
    }

    #[test]
    fn test_row_cursor_lazy_polling() {
        let suffix = format!("{}é", "a".repeat(1_048_576));
        let polls = Cell::new(0);
        let chunks = ["éabc", suffix.as_str()].into_iter().inspect(|_| {
            polls.set(polls.get() + 1);
        });
        let mut query = RowText::new(5 + suffix.len() as u32, chunks);
        assert_eq!(query.columns_for_bytes(0..0), 0..0);
        assert_eq!(query.floor_at(0).byte(), 0);
        let mut cursor = query.cursor(0);
        query.advance_columns(&mut cursor, 0);
        assert_eq!(polls.get(), 0);
        query.advance_columns(&mut cursor, 2);
        assert_eq!(cursor.byte(), 3);
        assert_eq!(polls.get(), 1);
        let mut fork = cursor.clone();
        query.retreat_to(&mut fork, 1);
        assert_eq!(fork.byte(), 0);
        assert_eq!(query.next_boundary(&mut cursor), Some(4));
        assert_eq!(polls.get(), 1);
        assert_eq!(query.columns_for_bytes(0..2), 0..1);
        assert_eq!(polls.get(), 1);
        let RowText::Unicode(chunks) = &query else {
            panic!("Unicode row")
        };
        assert_eq!(chunks.chunks.len(), 1);
        assert_eq!(chunks.context_bytes, 0);
    }

    #[test]
    fn test_row_cursor_reverse_regional_context_is_linear() {
        for symbols in [127, 128, 1023, 1024, 8191, 8192] {
            let text = "🇦".repeat(symbols);
            for chunk_symbols in [1, 2, 7, symbols] {
                for end in [text.len(), text.len() - 1, text.len() - 4] {
                    let chunks = text.as_bytes().chunks(4 * chunk_symbols).map(|chunk| {
                        std::str::from_utf8(chunk).expect("regional indicator chunks")
                    });
                    let mut query = RowText::new(text.len() as u32, chunks);
                    let mut cursor = query.floor_at(end as u32);
                    assert_eq!(
                        cursor.byte(),
                        if end == text.len() { end } else { end / 8 * 8 } as u32
                    );
                    let mut expected = cursor.byte();
                    while expected > 0 {
                        expected = (expected - 1) / 8 * 8;
                        assert_eq!(query.previous_boundary(&mut cursor), Some(expected));
                    }
                    assert_eq!(query.previous_boundary(&mut cursor), None);
                    let RowText::Unicode(chunks) = &query else {
                        panic!("Unicode row")
                    };
                    if chunk_symbols == 1 {
                        assert!(chunks.context_bytes > 0);
                    }
                    assert!(
                        chunks.context_bytes <= text.len() * 2,
                        "symbols {symbols}, chunk symbols {chunk_symbols}, end {end}, context bytes {}",
                        chunks.context_bytes
                    );
                }
            }
        }
    }

    #[gpui::test]
    fn test_columnar_selection_right_inlay_shifts_regional_parity(cx: &mut TestAppContext) {
        init_test(cx);
        for symbols in [4, 64, 8192] {
            let text = format!("{}z", "🇦".repeat(symbols));
            let mut state = plain_state(&text, cx);
            let buffer = state.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
            state
                .inlays
                .push(Inlay::mock_hint(0, buffer.anchor_after(Point::zero()), "🇶"));
            let snapshot = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(snapshot.tab_snapshot().text(), format!("🇶{text}"));
            let mut query = ColumnarSelectionRows::new(&snapshot);
            let last_flag = symbols as u32 / 2;
            let requests = (1..=last_flag)
                .flat_map(|column| [(0, column..column), (0, column..last_flag + 1)])
                .collect::<Vec<_>>();
            let end = Point::new(0, symbols as u32 * 4);
            let expected = (1..=last_flag)
                .flat_map(|_| {
                    [
                        Some((Point::zero(), Point::zero())),
                        Some((Point::zero(), end)),
                    ]
                })
                .collect::<Vec<_>>();
            assert_batch_points(&mut query, requests, expected, 8 * symbols);
            assert_eq!(
                query.points_for_row(0, &(last_flag..last_flag + 1)),
                Some((Point::zero(), end))
            );
            assert_eq!(
                query.points_for_row(0, &(last_flag + 1..last_flag + 1)),
                Some((end, end))
            );
        }
    }

    #[gpui::test]
    fn test_columnar_selection_tab_control_boundaries(cx: &mut TestAppContext) {
        init_test(cx);
        let expected = plain_row("\t\u{301}x", 4);
        assert_eq!(expected.text, "    \u{301}x");
        assert_eq!(expected.len, 6);
        assert_eq!(expected.columns, [(0, 0), (1, 4), (3, 5), (4, 6)]);
        assert_eq!(expected.points, [(0, 0), (4, 1), (5, 3), (6, 4)]);
        assert_eq!(expected.boundaries, [0, 1, 2, 3, 4, 6, 7]);
        for tab_size in [1, 2, 4, 8, 127, 128] {
            set_tab_size(tab_size, cx);
            let after_prepend_tab = 1 + tab_size - 1 % tab_size;
            for (line, endpoints) in [
                (
                    "\t\u{301}x",
                    vec![(0, 0), (1, tab_size), (3, tab_size + 1), (4, tab_size + 2)],
                ),
                (
                    "\u{600}\t\u{301}x",
                    vec![
                        (0, 0),
                        (2, 1),
                        (3, after_prepend_tab),
                        (5, after_prepend_tab + 1),
                        (6, after_prepend_tab + 2),
                    ],
                ),
            ] {
                let snapshot = plain_state(&format!("{line}\n{line}"), cx)
                    .build_map(cx)
                    .update(cx, |map, cx| map.snapshot(cx));
                let mut query = ColumnarSelectionRows::new(&snapshot);
                for source_row in [0, 1] {
                    for (index, &(start, start_column)) in endpoints.iter().enumerate() {
                        for &(end, end_column) in &endpoints[index..] {
                            for reversed in [false, true] {
                                let start_point = Point::new(source_row, start);
                                let end_point = Point::new(source_row, end);
                                let columns = query.columns_for_range(if reversed {
                                    end_point..start_point
                                } else {
                                    start_point..end_point
                                });
                                assert_eq!(columns, start_column..end_column);
                                let target_row = 1 - source_row;
                                assert_eq!(
                                    query.points_for_row(target_row, &columns),
                                    Some((
                                        Point::new(target_row, start),
                                        Point::new(target_row, end)
                                    )),
                                    "line {line:?}, tab size {tab_size}, source row {source_row}, range {start}..{end}, reversed {reversed}"
                                );
                            }
                        }
                    }
                }
                assert_plain_rows(&snapshot, tab_size);
            }
        }
    }

    #[gpui::test]
    fn test_columnar_selection_tab_control_boundaries_with_virtual_text(cx: &mut TestAppContext) {
        init_test(cx);
        let mut state = plain_state("\tQx\n\tx", cx);
        let buffer = state.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        state.inlays.push(Inlay::mock_hint(
            0,
            buffer.anchor_after(Point::new(1, 1)),
            "\u{301}",
        ));
        let map = state.build_map(cx);
        map.update(cx, |map, cx| {
            map.fold(
                vec![Crease::simple(
                    Point::new(0, 1)..Point::new(0, 2),
                    FoldPlaceholder {
                        collapsed_text: Some(SharedString::from("\u{301}")),
                        ..FoldPlaceholder::test()
                    },
                )],
                cx,
            )
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.tab_snapshot().text(), "    \u{301}x\n    \u{301}x");
        let mut query = ColumnarSelectionRows::new(&snapshot);
        for row in [0, 1] {
            assert_eq!(
                query.columns_for_range(Point::new(row, 1)..Point::new(row, 1)),
                4..4
            );
            assert_eq!(
                query.points_for_row(row, &(4..4)),
                Some((Point::new(row, 1), Point::new(row, 1)))
            );
            assert_eq!(
                query.points_for_row(row, &(4..5)),
                Some((Point::new(row, 1), Point::new(row, 2 - row)))
            );
        }
    }

    #[gpui::test]
    fn test_columnar_selection_lazy_unicode_prefix(cx: &mut TestAppContext) {
        init_test(cx);
        let line = format!("{}é", "a".repeat(1_048_576));
        let snapshot = plain_state(&format!("{line}\n{line}"), cx)
            .build_map(cx)
            .update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(query.columns_for_range(Point::zero()..Point::zero()), 0..0);
        assert_eq!(
            query.points_for_row(1, &(0..0)),
            Some((Point::new(1, 0), Point::new(1, 0)))
        );
        for row in [query.row(1), query.buffer_row(1)] {
            let RowText::Unicode(chunks) = row else {
                panic!("Unicode row")
            };
            assert_eq!(chunks.chunks.len(), 0);
        }
        assert_eq!(
            query.points_for_row(1, &(1..1)),
            Some((Point::new(1, 1), Point::new(1, 1)))
        );
        for mut row in [query.row(1), query.buffer_row(1)] {
            let mut cursor = row.cursor(0);
            row.advance_columns(&mut cursor, 1);
            assert_eq!(cursor.byte(), 1);
            let RowText::Unicode(chunks) = row else {
                panic!("Unicode row")
            };
            assert_eq!(chunks.chunks.len(), 1);
        }
        assert_eq!(
            query.points_for_row(1, &(1_048_576..1_048_576)),
            Some((Point::new(1, 1_048_576), Point::new(1, 1_048_576)))
        );
    }

    #[gpui::test]
    fn test_columnar_selection_skips_long_virtual_row(cx: &mut TestAppContext) {
        init_test(cx);
        let mut state = plain_state(&format!("x\n{}", "a".repeat(1_048_576)), cx);
        let buffer = state.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        state.inlays.push(Inlay::mock_hint(
            0,
            buffer.anchor_before(Point::new(1, 0)),
            &format!("{}\n", "a".repeat(1_048_576)),
        ));
        let snapshot = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(query.points_for_row(1, &(1_048_576..1_048_576)), None);
        assert_eq!(
            query.points_for_row(2, &(1_048_576..1_048_576)),
            Some((Point::new(1, 1_048_576), Point::new(1, 1_048_576)))
        );
    }

    #[gpui::test]
    fn test_columnar_selection_graphemes_and_edges(cx: &mut TestAppContext) {
        init_test(cx);
        let state = plain_state("e\u{301}\néé\n\n\t\u{301}x\n👩\u{200d}💻z\n", cx);
        let map = state.build_map(cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(
            query.columns_for_range(Point::new(0, 0)..Point::new(0, 3)),
            0..1
        );
        assert_eq!(
            query.columns_for_range(Point::new(1, 0)..Point::new(1, 4)),
            0..2
        );
        assert_eq!(
            query.columns_for_range(Point::new(1, 4)..Point::new(0, 0)),
            0..2
        );
        assert_eq!(
            query.points_for_row(0, &(0..1)),
            Some((Point::new(0, 0), Point::new(0, 3)))
        );
        assert_eq!(
            query.points_for_row(1, &(0..1)),
            Some((Point::new(1, 0), Point::new(1, 2)))
        );
        assert_eq!(query.points_for_row(0, &(1..2)), None);
        assert_eq!(
            query.points_for_row(0, &(u32::MAX..u32::MAX)),
            Some((Point::new(0, 3), Point::new(0, 3)))
        );
        assert_eq!(
            query.points_for_row(2, &(0..0)),
            Some((Point::new(2, 0), Point::new(2, 0)))
        );
        assert_eq!(query.points_for_row(2, &(0..1)), None);
        assert_eq!(
            query.points_for_row(3, &(3..4)),
            Some((Point::new(3, 0), Point::new(3, 1)))
        );
        assert_eq!(
            query.points_for_row(4, &(0..1)),
            Some((Point::new(4, 0), Point::new(4, 11)))
        );
        assert_eq!(
            query.points_for_row(5, &(u32::MAX..u32::MAX)),
            Some((Point::new(5, 0), Point::new(5, 0)))
        );
        assert_eq!(query.points_for_row(6, &(0..0)), None);
        assert_eq!(query.points_for_row(u32::MAX, &(0..0)), None);
        assert_plain_rows(&snapshot, 4);
        let empty = plain_state("", cx)
            .build_map(cx)
            .update(cx, |map, cx| map.snapshot(cx));
        assert_plain_rows(&empty, 4);
    }

    #[gpui::test]
    fn test_columnar_selection_ascii_fast_path(cx: &mut TestAppContext) {
        init_test(cx);
        let state = plain_state(&format!("{}\nx", "a".repeat(1_048_576)), cx);
        let map = state.build_map(cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(query.points_for_row(1, &(2..3)), None);
        assert_eq!(size_of_val(&query), size_of::<&DisplaySnapshot>());
        assert_eq!(
            query.columns_for_range(Point::zero()..Point::new(0, 1_048_576)),
            0..1_048_576
        );
        let RowText::Ascii(len) = query.row(0) else {
            panic!("ASCII row allocated Unicode boundaries")
        };
        assert_eq!(len, 1_048_576);
        assert_eq!(
            query.points_for_row(0, &(1_048_575..u32::MAX)),
            Some((Point::new(0, 1_048_575), Point::new(0, 1_048_576)))
        );
        let RowText::Ascii(len) = query.buffer_row(0) else {
            panic!("ASCII buffer row allocated Unicode boundaries")
        };
        assert_eq!(len, 1_048_576);
    }

    #[gpui::test]
    fn test_columnar_selection_tab_expansion_boundary(cx: &mut TestAppContext) {
        init_test(cx);
        let prefixes = (255..=257)
            .map(|column| format!("{}{}", "α".repeat(127), "a".repeat(column - 254)))
            .chain([
                format!("{}α", "a".repeat(255)),
                format!("{}e\u{301}", "a".repeat(254)),
            ])
            .collect::<Vec<_>>();
        let text = prefixes
            .iter()
            .map(|prefix| format!("{prefix}\t\u{301}β"))
            .collect::<Vec<_>>()
            .join("\n");
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let state = MapState::new(cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx)));
        let map = state.build_map(cx);
        for tab_size in [1, 3, 128] {
            set_tab_size(tab_size, cx);
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_plain_rows(&snapshot, tab_size);
            if tab_size == 128 {
                assert_eq!(
                    snapshot.tab_snapshot().text(),
                    format!(
                        "{}{}\u{301}β\n{} \u{301}β\n{} \u{301}β\n{} \u{301}β\n{} \u{301}β",
                        prefixes[0],
                        " ".repeat(128),
                        prefixes[1],
                        prefixes[2],
                        prefixes[3],
                        prefixes[4]
                    )
                );
            }
            buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "a")], None, cx));
            assert_plain_rows(&map.update(cx, |map, cx| map.snapshot(cx)), tab_size);
            buffer.update(cx, |buffer, cx| buffer.edit([(0..1, "")], None, cx));
            let restored = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                restored.tab_snapshot().text(),
                snapshot.tab_snapshot().text()
            );
            assert_plain_rows(&restored, tab_size);
        }
    }

    #[gpui::test]
    fn test_columnar_selection_folds_and_inlays_without_frames(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        init_test(cx);
        let mut state = plain_state("e\u{301}x\nab\ncd", cx);
        let map = state.build_map(cx);
        map.update(cx, |map, cx| {
            map.fold(
                vec![Crease::simple(
                    Point::new(0, 1)..Point::new(0, 3),
                    FoldPlaceholder::test(),
                )],
                cx,
            )
        });
        let folded = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(folded.tab_snapshot().text(), "e⋯x\nab\ncd");
        assert_eq!(
            ColumnarSelectionRows::new(&folded).points_for_row(0, &(1..2)),
            Some((Point::new(0, 0), Point::new(0, 3)))
        );
        state.folds = folded
            .folds_in_range(Anchor::Min..Anchor::Max)
            .map(|fold| fold.range.0.clone())
            .collect();
        for (index, bias) in [Bias::Left, Bias::Right].into_iter().enumerate() {
            state.inlays.push(Inlay::mock_hint(
                index,
                folded.buffer_snapshot().anchor_at(Point::new(1, 1), bias),
                "\u{301}\n\tR",
            ));
        }
        map.update(cx, |map, cx| {
            map.splice_inlays(&[], state.inlays.clone(), cx)
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            snapshot.tab_snapshot().text(),
            "e⋯x\na\u{301}\n    R\u{301}\n    Rb\ncd"
        );
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(query.points_for_row(2, &(0..0)), None);
        assert_eq!(query.points_for_row(2, &(0..u32::MAX)), None);
        let rebuilt = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
        check_snapshot(&snapshot, &rebuilt, true, None, 4, &mut rng, 0);
    }

    #[gpui::test]
    fn test_columnar_selection_long_fold_placeholders(cx: &mut TestAppContext) {
        init_test(cx);
        for placeholder in ["...", "long placeholder", "ééé", "👩\u{200d}💻xyz"] {
            for edited in [false, true] {
                let buffer = cx.new(|cx| Buffer::local(if edited { "a\nxb" } else { "axb" }, cx));
                let state = MapState::new(cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx)));
                let map = state.build_map(cx);
                map.update(cx, |map, cx| {
                    map.fold(
                        vec![Crease::simple(
                            Point::new(0, 1)..if edited {
                                Point::new(1, 1)
                            } else {
                                Point::new(0, 2)
                            },
                            FoldPlaceholder {
                                collapsed_text: Some(SharedString::from(placeholder)),
                                ..FoldPlaceholder::test()
                            },
                        )],
                        cx,
                    );
                });
                if edited {
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit([(Point::new(0, 1)..Point::new(1, 0), "")], None, cx)
                    });
                }
                let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
                assert_eq!(snapshot.tab_snapshot().text(), format!("a{placeholder}b"));
                let mut query = ColumnarSelectionRows::new(&snapshot);
                let end = 1 + placeholder.graphemes(true).count() as u32;
                for column in 1..end {
                    assert_eq!(
                        query.points_for_row(0, &(column..column)),
                        Some((Point::new(0, 1), Point::new(0, 1))),
                        "{placeholder:?}, column {column}, edited {edited}"
                    );
                }
                assert_eq!(
                    query.points_for_row(0, &(end..end)),
                    Some((Point::new(0, 2), Point::new(0, 2)))
                );
            }
        }
    }

    #[gpui::test]
    fn test_columnar_selection_fold_grapheme_edges(cx: &mut TestAppContext) {
        init_test(cx);
        for (text, placeholder, column, expected) in
            [("axb", "\u{301}..", 1, 0), ("ax\u{301}b", "..", 3, 4)]
        {
            let state = plain_state(text, cx);
            let map = state.build_map(cx);
            map.update(cx, |map, cx| {
                map.fold(
                    vec![Crease::simple(
                        Point::new(0, 1)..Point::new(0, 2),
                        FoldPlaceholder {
                            collapsed_text: Some(SharedString::from(placeholder)),
                            ..FoldPlaceholder::test()
                        },
                    )],
                    cx,
                )
            });
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                ColumnarSelectionRows::new(&snapshot).points_for_row(0, &(column..column)),
                Some((Point::new(0, expected), Point::new(0, expected))),
                "{text:?}, {placeholder:?}"
            );
        }
        for merge in [false, true] {
            let map = plain_state("axyb", cx).build_map(cx);
            map.update(cx, |map, cx| {
                map.fold(
                    [(1, ".."), (2, "ZZ")]
                        .into_iter()
                        .map(|(start, placeholder)| {
                            Crease::simple(
                                Point::new(0, start)..Point::new(0, start + 1),
                                FoldPlaceholder {
                                    merge_adjacent: merge,
                                    collapsed_text: Some(SharedString::from(placeholder)),
                                    ..FoldPlaceholder::test()
                                },
                            )
                        })
                        .collect(),
                    cx,
                )
            });
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                snapshot.tab_snapshot().text(),
                if merge { "a..b" } else { "a..ZZb" }
            );
            let expected = if merge {
                vec![0, 1, 1, 3, 4]
            } else {
                vec![0, 1, 1, 2, 2, 3, 4]
            };
            let mut query = ColumnarSelectionRows::new(&snapshot);
            for (column, byte) in expected.into_iter().enumerate() {
                let column = column as u32;
                assert_eq!(
                    query.points_for_row(0, &(column..column)),
                    Some((Point::new(0, byte), Point::new(0, byte))),
                    "merge={merge}, column={column}"
                );
            }
        }
    }

    #[gpui::test]
    fn test_columnar_selection_fold_ending_inside_grapheme(cx: &mut TestAppContext) {
        init_test(cx);
        let map = plain_state("aX\u{1100}\u{1161}z", cx).build_map(cx);
        map.update(cx, |map, cx| {
            map.fold(
                vec![Crease::simple(
                    Point::new(0, 1)..Point::new(0, 5),
                    FoldPlaceholder::test(),
                )],
                cx,
            )
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.tab_snapshot().text(), "a⋯\u{1161}z");
        let mut query = ColumnarSelectionRows::new(&snapshot);
        for (columns, expected) in [
            (0..0, (Point::new(0, 0), Point::new(0, 0))),
            (1..1, (Point::new(0, 1), Point::new(0, 1))),
            (2..2, (Point::new(0, 1), Point::new(0, 1))),
            (3..3, (Point::new(0, 8), Point::new(0, 8))),
            (4..4, (Point::new(0, 9), Point::new(0, 9))),
            (1..3, (Point::new(0, 1), Point::new(0, 8))),
            (2..4, (Point::new(0, 1), Point::new(0, 9))),
        ] {
            assert_eq!(
                query.points_for_row(0, &columns),
                Some(expected),
                "columns {columns:?}"
            );
        }
    }

    #[test]
    fn test_column_boundaries_large_grapheme() {
        let text = format!("e{}z", "\u{301}".repeat(1024));
        let mut query = RowText::new(text.len() as u32, iter::once(text.as_str()));
        let mut cursor = query.cursor(0);
        assert_eq!(cursor.byte(), 0);
        assert_eq!(query.next_boundary(&mut cursor), Some(2049));
        assert_eq!(query.next_boundary(&mut cursor), Some(2050));
        assert_eq!(query.columns_for_bytes(2048..2049), 0..1);
        assert_eq!(query.floor_at(2048).byte(), 0);
        assert_eq!(query.floor_at(2049).byte(), 2049);
    }

    #[gpui::test(iterations = 20)]
    fn test_columnar_selection_during_pending_rewrap(cx: &mut TestAppContext, mut rng: StdRng) {
        init_test(cx);
        cx.background_executor.set_block_on_ticks(0..=0);
        let text = (0..120)
            .map(|row| format!("row{row} {}", random_text(&mut rng, 6)))
            .collect::<Vec<_>>()
            .join("\n");
        let mut state = plain_state(&text, cx);
        let map = state.build_map(cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let buffer = snapshot.buffer_snapshot();
        let hidden_start = rng.random_range(0..buffer.max_point().row);
        let hidden_end = rng.random_range(hidden_start..=buffer.max_point().row);
        state.block = Some((
            BlockPlacement::Replace(
                buffer.anchor_before(Point::new(hidden_start, 0))
                    ..=buffer.anchor_after(Point::new(
                        hidden_end,
                        buffer.line_len(MultiBufferRow(hidden_end)),
                    )),
            ),
            1,
        ));
        map.update(cx, |map, cx| {
            map.insert_blocks([state.block_properties().expect("block")], cx)
        });
        state.wrap_width = Some(px(30.0));
        map.update(cx, |map, cx| map.set_wrap_width(state.wrap_width, cx));
        assert!(map.update(cx, |map, cx| map.is_rewrapping(cx)));
        let interpolated = map.update(cx, |map, cx| map.snapshot(cx));
        assert!(map.update(cx, |map, cx| map.is_rewrapping(cx)));
        check_snapshot(&interpolated, &interpolated, true, None, 4, &mut rng, 0);
        let rebuilt = state.build_map(cx);
        cx.run_until_parked();
        assert!(!map.update(cx, |map, cx| map.is_rewrapping(cx)));
        let rebuilt = rebuilt.update(cx, |map, cx| map.snapshot(cx));
        let settled = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(hidden_rows(&settled), hidden_rows(&rebuilt));
        check_snapshot(&settled, &rebuilt, true, None, 4, &mut rng, 1);
        check_snapshot(
            &interpolated,
            &settled,
            hidden_rows(&interpolated) == hidden_rows(&settled),
            None,
            4,
            &mut rng,
            2,
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_columnar_selections(cx: &mut TestAppContext, mut rng: StdRng) {
        init_test(cx);
        cx.background_executor.set_block_on_ticks(0..=50);
        let operations = env::var("OPERATIONS")
            .map(|value| value.parse::<usize>().expect("invalid OPERATIONS"))
            .unwrap_or(15);
        let mut texts = (0..rng.random_range(1..=2))
            .map(|_| format!("a{}z", random_text(&mut rng, 12)))
            .collect::<Vec<_>>();
        let buffers = texts
            .iter()
            .map(|text| cx.new(|cx| Buffer::local(text.clone(), cx)))
            .collect::<Vec<_>>();
        let multibuffer = cx.new(|cx| {
            if buffers.len() == 1 {
                return MultiBuffer::singleton(buffers[0].clone(), cx);
            }
            let mut buffer = MultiBuffer::new(Capability::ReadWrite);
            for (index, source) in buffers.iter().enumerate() {
                let snapshot = source.read(cx).snapshot();
                buffer.set_excerpt_ranges_for_path(
                    PathKey::sorted(index as u64),
                    source.clone(),
                    &snapshot,
                    vec![ExcerptRange::new(Point::zero()..snapshot.max_point())],
                    cx,
                );
            }
            buffer
        });
        let mut state = MapState::new(multibuffer);
        let map = state.build_map(cx);
        let mut block_ids = HashSet::default();
        let mut kinds = [0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6];
        kinds.shuffle(&mut rng);
        let mut snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        check_snapshot(
            &snapshot,
            &snapshot,
            true,
            Some(&texts.join("\n")),
            4,
            &mut rng,
            0,
        );
        for operation in 0..operations {
            let kind = kinds[operation % kinds.len()];
            log::info!(
                "operation {operation}: kind {kind}, texts {texts:?}, tabs {}, folds {:?}, inlays {:?}, block {:?}, wrap {:?}",
                state.tab_size,
                state.folds,
                state.inlays,
                state.block,
                state.wrap_width
            );
            let replaced_before = state.replaces_rows();
            match kind {
                0 => {
                    let index = rng.random_range(0..buffers.len());
                    let text = &mut texts[index];
                    let boundaries = text
                        .char_indices()
                        .map(|(offset, _)| offset)
                        .skip(1)
                        .collect::<Vec<_>>();
                    let start = rng.random_range(0..boundaries.len());
                    let mut range =
                        boundaries[start]..boundaries[rng.random_range(start..boundaries.len())];
                    let mut replacement = random_text(&mut rng, 4);
                    if text.get(range.clone()) == Some(replacement.as_str()) {
                        replacement.push('a');
                    }
                    let mut expected = text.clone();
                    expected.replace_range(range.clone(), &replacement);
                    if expected.len() > 256
                        || expected.matches('\t').count() > 4
                        || expected.matches('\n').count() > 12
                    {
                        range = 1..text.len() - 1;
                        replacement.clear();
                        expected = String::from("az");
                    }
                    assert_ne!(expected.as_str(), text.as_str());
                    log::info!("edit buffer {index}: {range:?} -> {replacement:?}");
                    buffers[index].update(cx, |buffer, cx| {
                        buffer.edit([(range, replacement)], None, cx)
                    });
                    *text = expected;
                    if state.replaces_rows() {
                        map.update(cx, |map, cx| {
                            map.remove_blocks(std::mem::take(&mut block_ids), cx);
                            block_ids.extend(
                                map.insert_blocks([state.block_properties().expect("block")], cx),
                            );
                        });
                    }
                }
                1 => {
                    let size = rng.random_range(1..128);
                    state.tab_size = size + u32::from(size >= state.tab_size);
                    set_tab_size(state.tab_size, cx);
                }
                2 => {
                    if state.folds.is_empty() {
                        let index = rng.random_range(0..texts.len());
                        let base = texts
                            .iter()
                            .take(index)
                            .map(|text| text.len() + 1)
                            .sum::<usize>();
                        let boundaries = scalar_boundaries(&texts[index]);
                        let start = rng.random_range(0..boundaries.len() - 1);
                        let end = rng.random_range(start + 1..boundaries.len());
                        let buffer = snapshot.buffer_snapshot();
                        let range = buffer.anchor_after(MultiBufferOffset(base + boundaries[start]))
                            ..buffer.anchor_before(MultiBufferOffset(base + boundaries[end]));
                        map.update(cx, |map, cx| {
                            map.fold(vec![Crease::simple(range, FoldPlaceholder::test())], cx)
                        });
                    } else {
                        map.update(cx, |map, cx| {
                            map.unfold_intersecting([Anchor::Min..Anchor::Max], true, cx)
                        });
                        if state.replaces_rows() {
                            block_ids.clear();
                            state.block = None;
                        }
                    }
                }
                3 | 6 => {
                    if state.inlays.is_empty() {
                        let text = snapshot.buffer_snapshot().text();
                        let boundaries = scalar_boundaries(&text);
                        let offset =
                            MultiBufferOffset(boundaries[rng.random_range(0..boundaries.len())]);
                        for (index, bias) in [Bias::Left, Bias::Right].into_iter().enumerate() {
                            let text = format!("\n{}\t", random_text(&mut rng, 3));
                            state.inlays.push(Inlay::mock_hint(
                                index,
                                snapshot.buffer_snapshot().anchor_at(offset, bias),
                                text.as_str(),
                            ));
                        }
                        map.update(cx, |map, cx| {
                            map.splice_inlays(&[], state.inlays.clone(), cx)
                        });
                    } else {
                        let ids = state
                            .inlays
                            .iter()
                            .map(|inlay| inlay.id)
                            .collect::<Vec<_>>();
                        map.update(cx, |map, cx| map.splice_inlays(&ids, Vec::new(), cx));
                        state.inlays.clear();
                    }
                }
                4 => {
                    state.wrap_width = if state.wrap_width.is_some() {
                        None
                    } else {
                        Some(px([0.0, 12.0, 48.0, 300.0][rng.random_range(0..4)]))
                    };
                    map.update(cx, |map, cx| map.set_wrap_width(state.wrap_width, cx));
                }
                5 => {
                    if state.block.is_some() {
                        map.update(cx, |map, cx| {
                            map.remove_blocks(std::mem::take(&mut block_ids), cx)
                        });
                        state.block = None;
                    } else {
                        let text = snapshot.buffer_snapshot().text();
                        let boundaries = scalar_boundaries(&text);
                        let start = rng.random_range(0..boundaries.len());
                        let buffer = snapshot.buffer_snapshot();
                        let position = buffer.anchor_after(MultiBufferOffset(boundaries[start]));
                        let placement = match rng.random_range(0..3) {
                            0 => BlockPlacement::Above(position),
                            1 => BlockPlacement::Below(position),
                            _ => {
                                let start = rng.random_range(0..boundaries.len() - 1);
                                let end = rng.random_range(start + 1..boundaries.len());
                                BlockPlacement::Replace(
                                    buffer.anchor_before(MultiBufferOffset(boundaries[start]))
                                        ..=buffer.anchor_after(MultiBufferOffset(boundaries[end])),
                                )
                            }
                        };
                        state.block = Some((placement, rng.random_range(1..=3)));
                        block_ids.extend(map.update(cx, |map, cx| {
                            map.insert_blocks([state.block_properties().expect("block")], cx)
                        }));
                    }
                }
                _ => unreachable!(),
            }
            let previous = snapshot;
            snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            state.folds = snapshot
                .folds_in_range(Anchor::Min..Anchor::Max)
                .map(|fold| fold.range.0.clone())
                .collect();
            let text = texts.join("\n");
            assert_eq!(
                snapshot.buffer_snapshot().text(),
                text,
                "operation {operation}"
            );
            let plain = (state.folds.is_empty()
                && state
                    .inlays
                    .iter()
                    .all(|inlay| !inlay.position.is_valid(snapshot.buffer_snapshot()))
                && !state.replaces_rows())
            .then_some(text.as_str());
            let rewrapping = map.update(cx, |map, cx| map.is_rewrapping(cx));
            let rebuilt = state.build_map(cx);
            cx.run_until_parked();
            let rebuilt = rebuilt.update(cx, |map, cx| map.snapshot(cx));
            let settled = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                hidden_rows(&settled),
                hidden_rows(&rebuilt),
                "operation {operation}: hidden rows"
            );
            check_snapshot(
                &snapshot,
                &rebuilt,
                !state.replaces_rows() || !rewrapping,
                plain,
                state.tab_size,
                &mut rng,
                operation,
            );
            if (kind == 4 || kind == 5) && !replaced_before && !state.replaces_rows() {
                check_snapshot(
                    &snapshot,
                    &previous,
                    true,
                    plain,
                    state.tab_size,
                    &mut rng,
                    operation,
                );
            }
            check_snapshot(
                &settled,
                &rebuilt,
                true,
                plain,
                state.tab_size,
                &mut rng,
                operation,
            );
            snapshot = settled;
        }
    }

    struct MapState {
        buffer: Entity<MultiBuffer>,
        tab_size: u32,
        folds: Vec<Range<Anchor>>,
        inlays: Vec<Inlay>,
        block: Option<(BlockPlacement<Anchor>, u32)>,
        wrap_width: Option<Pixels>,
    }

    impl MapState {
        fn new(buffer: Entity<MultiBuffer>) -> Self {
            Self {
                buffer,
                tab_size: 4,
                folds: Vec::new(),
                inlays: Vec::new(),
                block: None,
                wrap_width: None,
            }
        }

        fn build_map(&self, cx: &mut TestAppContext) -> Entity<DisplayMap> {
            cx.new(|cx| {
                let mut map = DisplayMap::new(
                    self.buffer.clone(),
                    test_font(),
                    px(10.0),
                    self.wrap_width,
                    1,
                    1,
                    FoldPlaceholder::test(),
                    DiagnosticSeverity::Warning,
                    cx,
                );
                map.splice_inlays(&[], self.inlays.clone(), cx);
                if !self.folds.is_empty() {
                    map.fold(
                        self.folds
                            .iter()
                            .cloned()
                            .map(|range| Crease::simple(range, FoldPlaceholder::test()))
                            .collect(),
                        cx,
                    );
                }
                if let Some(block) = self.block_properties() {
                    map.insert_blocks([block], cx);
                }
                map
            })
        }

        fn replaces_rows(&self) -> bool {
            matches!(self.block, Some((BlockPlacement::Replace(_), _)))
        }

        fn block_properties(&self) -> Option<BlockProperties<Anchor>> {
            self.block
                .as_ref()
                .map(|(placement, height)| BlockProperties {
                    placement: placement.clone(),
                    style: BlockStyle::Fixed,
                    height: Some(*height),
                    render: Arc::new(|_| div().into_any()),
                    priority: 0,
                })
        }
    }

    struct PlainRow {
        text: String,
        len: u32,
        columns: Vec<(u32, u32)>,
        points: Vec<(u32, u32)>,
        boundaries: Vec<usize>,
    }

    fn assert_batch_points(
        query: &mut ColumnarSelectionRows<'_>,
        mut requests: Vec<(u32, Range<u32>)>,
        mut expected: Vec<Option<(Point, Point)>>,
        work_bound: usize,
    ) {
        for _ in 0..2 {
            ROW_WORK.set((0, 0));
            let answers = query.points_for_rows(&requests);
            assert_eq!(answers.len(), expected.len());
            for (index, (answer, expected)) in answers.iter().zip(&expected).enumerate() {
                assert_eq!(answer, expected, "request {index}: {:?}", requests[index]);
            }
            let (boundaries, chunks) = ROW_WORK.get();
            assert!(boundaries <= work_bound, "{boundaries} > {work_bound}");
            assert!(chunks <= work_bound, "{chunks} > {work_bound}");
            requests.reverse();
            expected.reverse();
        }
    }

    fn enumerated_points(
        snapshot: &DisplaySnapshot,
        raw: &str,
        row: u32,
        boundaries: &[usize],
    ) -> Vec<(u32, Point)> {
        raw.split('\n')
            .enumerate()
            .flat_map(|(row, line)| {
                grapheme_boundaries(line)
                    .into_iter()
                    .map(move |byte| Point::new(row as u32, byte as u32))
            })
            .filter_map(|point| {
                let tab = snapshot
                    .tab_snapshot()
                    .point_to_tab_point(point, Bias::Left);
                let display = point.to_display_point(snapshot);
                if tab.row() != row
                    || snapshot.tab_snapshot().tab_point_to_point(tab, Bias::Left) != point
                    || display.to_point(snapshot) != point
                    || snapshot.is_block_line(display.row())
                {
                    return None;
                }
                boundaries
                    .binary_search(&(tab.column() as usize))
                    .ok()
                    .map(|column| (column as u32, point))
            })
            .collect()
    }

    fn nearest_points(
        columns: &Range<u32>,
        length: u32,
        candidates: &[(u32, Point)],
    ) -> Option<(Point, Point)> {
        if columns.start > columns.end || (columns.start != columns.end && columns.start >= length)
        {
            return None;
        }
        let point = |column| {
            candidates
                .iter()
                .filter(|(candidate, _)| *candidate <= column)
                .max_by_key(|(candidate, _)| *candidate)
                .map(|(_, point)| *point)
        };
        point(columns.start).zip(point(columns.end))
    }

    fn hidden_rows(snapshot: &DisplaySnapshot) -> Vec<bool> {
        (0..=snapshot.buffer_snapshot().max_point().row)
            .map(|row| snapshot.is_block_line(Point::new(row, 0).to_display_point(snapshot).row()))
            .collect()
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            crate::init(cx);
            theme_settings::init(LoadThemes::JustBase, cx);
        });
        set_tab_size(4, cx);
    }

    fn set_tab_size(size: u32, cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.tab_size = NonZeroU32::new(size);
                })
            })
        });
    }

    fn plain_state(text: &str, cx: &mut TestAppContext) -> MapState {
        MapState::new(cx.update(|cx| MultiBuffer::build_simple(text, cx)))
    }

    fn random_text(rng: &mut StdRng, tokens: usize) -> String {
        let mut text = String::new();
        let mut tabs = 0;
        for _ in 0..rng.random_range(0..=tokens) {
            let mut token = *[
                "a",
                " ",
                "\t",
                "\n",
                "α",
                "界",
                "🏀",
                "é",
                "e\u{301}",
                "o\u{308}\u{301}",
                "\u{301}",
                "👩\u{200d}💻",
                "👍🏽",
                "🇦🇶",
                "✈\u{fe0f}",
            ]
            .choose(rng)
            .expect("tokens");
            if token == "\t" {
                if tabs == 2 {
                    token = "a";
                } else {
                    tabs += 1;
                }
            }
            text.push_str(token);
        }
        text
    }

    fn assert_row_boundaries(query: &mut RowText<'_>, expected: &[usize]) {
        let length = *expected.last().expect("end boundary");
        for byte in 0..=length + 1 {
            let column = expected.partition_point(|boundary| *boundary <= byte) - 1;
            assert_eq!(
                query.columns_for_bytes(byte as u32..byte as u32),
                column as u32..column as u32,
                "byte {byte}, expected {expected:?}"
            );
            assert_eq!(
                query.floor_at(byte as u32).byte(),
                expected[column] as u32,
                "byte {byte}, expected {expected:?}"
            );
        }
        let mut cursor = query.cursor(0);
        for byte in expected.iter().skip(1) {
            assert_eq!(query.next_boundary(&mut cursor), Some(*byte as u32));
        }
        assert_eq!(query.next_boundary(&mut cursor), None);
        for byte in expected.iter().rev().skip(1) {
            assert_eq!(query.previous_boundary(&mut cursor), Some(*byte as u32));
        }
        assert_eq!(query.previous_boundary(&mut cursor), None);
        let mut cursor = query.floor_at(u32::MAX);
        assert_eq!(cursor.byte(), length as u32);
        for byte in expected.iter().rev().skip(1) {
            assert_eq!(query.previous_boundary(&mut cursor), Some(*byte as u32));
        }
        query.advance_columns(&mut cursor, u32::MAX);
        assert_eq!(cursor.byte(), length as u32);
    }

    fn scalar_boundaries(text: &str) -> Vec<usize> {
        text.char_indices()
            .map(|(offset, _)| offset)
            .chain(iter::once(text.len()))
            .collect()
    }

    fn grapheme_boundaries(text: &str) -> Vec<usize> {
        text.grapheme_indices(true)
            .map(|(offset, _)| offset)
            .chain(iter::once(text.len()))
            .collect()
    }

    fn plain_row(line: &str, tab_size: u32) -> PlainRow {
        let mut text = String::new();
        let mut columns = Vec::new();
        let mut points = Vec::new();
        let mut boundaries = Vec::new();
        let mut column = 0;
        let mut character_column = 0;
        for (byte, grapheme) in line.grapheme_indices(true) {
            points.push((column, byte as u32));
            columns.extend(
                grapheme
                    .char_indices()
                    .map(|(offset, _)| ((byte + offset) as u32, column)),
            );
            if grapheme == "\t" {
                let width = if byte < 256 {
                    tab_size - character_column % tab_size
                } else {
                    1
                };
                boundaries.extend(text.len()..text.len() + width as usize);
                text.extend(iter::repeat_n(' ', width as usize));
                character_column += width;
                column += width;
            } else {
                boundaries.push(text.len());
                text.push_str(grapheme);
                character_column += grapheme.chars().count() as u32;
                column += 1;
            }
        }
        columns.push((line.len() as u32, column));
        points.push((column, line.len() as u32));
        boundaries.push(text.len());
        PlainRow {
            text,
            len: column,
            columns,
            points,
            boundaries,
        }
    }

    fn expected_points(
        row: u32,
        columns: &Range<u32>,
        expected: &PlainRow,
    ) -> Option<(Point, Point)> {
        if columns.start != columns.end && columns.start >= expected.len {
            return None;
        }
        let point = |column| {
            Point::new(
                row,
                expected
                    .points
                    .iter()
                    .rev()
                    .find(|(candidate, _)| *candidate <= column)
                    .expect("row start")
                    .1,
            )
        };
        Some((point(columns.start), point(columns.end)))
    }

    fn assert_plain_rows(snapshot: &DisplaySnapshot, tab_size: u32) {
        let raw = snapshot.buffer_snapshot().text();
        let expected = raw
            .split('\n')
            .map(|line| plain_row(line, tab_size))
            .collect::<Vec<_>>();
        assert_eq!(
            snapshot.tab_snapshot().text(),
            expected
                .iter()
                .map(|row| row.text.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        );
        let mut query = ColumnarSelectionRows::new(snapshot);
        for (row, expected) in expected.iter().enumerate() {
            let row = row as u32;
            assert_eq!(
                query.row(row).columns_for_bytes(0..u32::MAX).end,
                expected.len
            );
            for &(byte, column) in &expected.columns {
                assert_eq!(
                    query.columns_for_range(Point::new(row, byte)..Point::new(row, byte)),
                    column..column
                );
            }
            for column in 0..=expected.len + 1 {
                for columns in [column..column, column..column + 1, column..u32::MAX] {
                    assert_eq!(
                        query.points_for_row(row, &columns),
                        expected_points(row, &columns, expected),
                        "row {row}, columns {columns:?}, text {:?}",
                        expected.text
                    );
                }
            }
        }
    }

    fn check_snapshot(
        snapshot: &DisplaySnapshot,
        rebuilt: &DisplaySnapshot,
        blocks_agree: bool,
        plain: Option<&str>,
        tab_size: u32,
        rng: &mut StdRng,
        operation: usize,
    ) {
        let text = snapshot.tab_snapshot().text();
        assert_eq!(
            text,
            rebuilt.tab_snapshot().text(),
            "operation {operation}: tab text"
        );
        assert_eq!(
            snapshot.fold_snapshot().text(),
            rebuilt.fold_snapshot().text(),
            "operation {operation}: fold text"
        );
        assert_eq!(
            snapshot.inlay_snapshot().text(),
            rebuilt.inlay_snapshot().text(),
            "operation {operation}: inlay text"
        );
        let raw = snapshot.buffer_snapshot().text();
        assert_eq!(raw, rebuilt.buffer_snapshot().text());
        if let Some(plain) = plain {
            assert_eq!(raw, plain);
        }
        let expected = plain.map(|plain| {
            plain
                .split('\n')
                .map(|line| plain_row(line, tab_size))
                .collect::<Vec<_>>()
        });
        if let Some(expected) = &expected {
            assert_eq!(
                text,
                expected
                    .iter()
                    .map(|row| row.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\n"),
                "operation {operation}: independent expansion"
            );
        }
        let lines = text.split('\n').collect::<Vec<_>>();
        let raw_lines = raw.split('\n').collect::<Vec<_>>();
        let raw_boundaries = grapheme_boundaries(&raw);
        let mut query = ColumnarSelectionRows::new(snapshot);
        let mut rebuilt_query = ColumnarSelectionRows::new(rebuilt);
        assert_eq!(
            snapshot.tab_snapshot().max_point().row() as usize + 1,
            lines.len()
        );
        let sources = scalar_boundaries(&raw);
        for (start, end) in [
            (0, raw.len()),
            (
                sources[rng.random_range(0..sources.len())],
                sources[rng.random_range(0..sources.len())],
            ),
        ] {
            let point = |offset| {
                Point::new(
                    raw[..offset].matches('\n').count() as u32,
                    raw[..offset].rsplit('\n').next().expect("line").len() as u32,
                )
            };
            let range = point(start)..point(end);
            let columns = query.columns_for_range(range.clone());
            assert_eq!(
                columns,
                rebuilt_query.columns_for_range(range.clone()),
                "operation {operation}: source {range:?}"
            );
            if let Some(expected) = &expected {
                let column = |point: Point| {
                    expected[point.row as usize]
                        .columns
                        .iter()
                        .find(|(byte, _)| *byte == point.column)
                        .expect("source byte")
                        .1
                };
                let (start, end) = (column(range.start), column(range.end));
                assert_eq!(
                    columns,
                    start.min(end)..start.max(end),
                    "operation {operation}: independent source {range:?}"
                );
            }
        }
        let folded = snapshot.fold_snapshot().text();
        let expanded = folded
            .split('\n')
            .map(|line| plain_row(line, tab_size))
            .collect::<Vec<_>>();
        let mut rows = vec![
            0,
            lines.len() - 1,
            rng.random_range(0..lines.len()),
            rng.random_range(0..lines.len()),
        ];
        rows.sort_unstable();
        rows.dedup();
        for row in rows {
            assert_eq!(lines[row], expanded[row].text);
            let boundaries = &expanded[row].boundaries;
            let len = boundaries.len() as u32 - 1;
            assert_eq!(
                snapshot.tab_snapshot().line_len(row as u32),
                lines[row].len() as u32
            );
            assert_eq!(
                query.row(row as u32).columns_for_bytes(0..u32::MAX).end,
                len,
                "operation {operation}: row {row}"
            );
            assert_eq!(
                rebuilt_query
                    .row(row as u32)
                    .columns_for_bytes(0..u32::MAX)
                    .end,
                len
            );
            let candidates = enumerated_points(snapshot, &raw, row as u32, boundaries);
            let middle = rng.random_range(0..=len + 1);
            let requests = [
                0..0,
                0..u32::MAX,
                middle..middle,
                middle..middle + 1,
                middle..rng.random_range(middle..=len + 1),
                len..len,
                len..len + 1,
                u32::MAX..u32::MAX,
            ]
            .map(|columns| (row as u32, columns));
            let answers = query.points_for_rows(&requests);
            let rebuilt_answers = rebuilt_query.points_for_rows(&requests);
            for (((_, columns), points), rebuilt_points) in
                requests.into_iter().zip(answers).zip(rebuilt_answers)
            {
                if blocks_agree {
                    assert_eq!(
                        points, rebuilt_points,
                        "operation {operation}: row {row}, columns {columns:?}"
                    );
                }
                if let Some(expected) = &expected {
                    assert_eq!(
                        points,
                        expected_points(row as u32, &columns, &expected[row]),
                        "operation {operation}: independent row {row}, columns {columns:?}"
                    );
                }
                let expected = nearest_points(&columns, len, &candidates);
                assert_eq!(
                    points, expected,
                    "operation {operation}: enumerated raw boundaries, row {row}, columns {columns:?}, raw {raw:?}, tab {text:?}"
                );
                if let Some((start, end)) = points {
                    assert!(start <= end);
                    for point in [start, end] {
                        let line = raw_lines
                            .get(point.row as usize)
                            .expect("raw row in bounds");
                        assert!(point.column as usize <= line.len());
                        let offset = raw_lines
                            .iter()
                            .take(point.row as usize)
                            .map(|line| line.len() + 1)
                            .sum::<usize>()
                            + point.column as usize;
                        assert_eq!(
                            raw_boundaries
                                .get(raw_boundaries.partition_point(|boundary| *boundary < offset))
                                .copied(),
                            Some(offset),
                            "operation {operation}: full-buffer grapheme {point:?}, raw {raw:?}"
                        );
                        let forward = snapshot
                            .tab_snapshot()
                            .point_to_tab_point(point, Bias::Left);
                        assert_eq!(forward.row(), row as u32);
                        assert_eq!(
                            snapshot
                                .tab_snapshot()
                                .tab_point_to_point(forward, Bias::Left),
                            point
                        );
                        let column = boundaries
                            .binary_search(&(forward.column() as usize))
                            .expect("tab grapheme boundary")
                            as u32;
                        assert_eq!(
                            query.points_for_row(row as u32, &(column..column)),
                            Some((point, point))
                        );
                        let display = point.to_display_point(snapshot);
                        assert!(!snapshot.is_block_line(display.row()));
                        assert_eq!(display.to_point(snapshot), point);
                    }
                }
            }
        }
        assert_eq!(query.points_for_row(lines.len() as u32, &(0..0)), None);
    }

    fn separated_columnar_ranges(
        row: u32,
        indices: Range<usize>,
        stride: u32,
        width: u32,
        reversed: bool,
    ) -> Vec<Range<Point>> {
        indices
            .map(|index| {
                let start = Point::new(row, index as u32 * stride);
                let end = Point::new(row, start.column + width);
                if reversed { end..start } else { start..end }
            })
            .collect()
    }

    fn set_columnar_action_state(
        cx: &mut EditorTestContext,
        text: &str,
        expected: &[Range<Point>],
    ) {
        cx.update_editor(|editor, window, cx| {
            editor.set_text(text, window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges(expected.iter().cloned());
            });
            assert_eq!(
                editor
                    .selections
                    .ranges::<Point>(&editor.display_snapshot(cx)),
                expected,
            );
        });
    }

    #[track_caller]
    fn assert_columnar_action_work(
        cx: &mut EditorTestContext,
        above: bool,
        expected: &[Range<Point>],
        work_bound: usize,
    ) {
        cx.update_editor(|editor, window, cx| {
            ROW_WORK.set((0, 0));
            if above {
                editor.add_selection_above(
                    &AddSelectionAbove {
                        skip_soft_wrap: true,
                    },
                    window,
                    cx,
                );
            } else {
                editor.add_selection_below(
                    &AddSelectionBelow {
                        skip_soft_wrap: true,
                    },
                    window,
                    cx,
                );
            }
            let (boundaries, chunks) = ROW_WORK.replace((0, 0));
            assert!(boundaries > 0);
            assert!(chunks > 0);
            assert!(
                boundaries + chunks <= work_bound,
                "above {above}: {boundaries} boundaries + {chunks} chunks > {work_bound}",
            );
            let actual = editor
                .selections
                .ranges::<Point>(&editor.display_snapshot(cx));
            assert_eq!(actual.len(), expected.len(), "above {above}");
            for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(actual, expected, "above {above}, selection {index}");
            }
        });
    }
}
