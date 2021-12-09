mod anchor;
mod location;
mod selection;

use self::location::*;
use crate::{
    buffer::{self, Buffer, Chunk, ToOffset as _, ToPoint as _},
    BufferSnapshot, Diagnostic, File, Language,
};
use anyhow::Result;
use clock::ReplicaId;
use collections::HashMap;
use gpui::{AppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use parking_lot::{Mutex, MutexGuard};
use std::{cmp, io, ops::Range, sync::Arc, time::SystemTime};
use sum_tree::{Bias, Cursor, SumTree};
use text::{
    rope::TextDimension,
    subscription::{Subscription, Topic},
    AnchorRangeExt as _, Edit, Point, PointUtf16, Selection, SelectionSetId, TextSummary,
};
use theme::SyntaxTheme;

pub use anchor::{Anchor, AnchorRangeExt, AnchorRangeMap, AnchorRangeSet};
pub use selection::SelectionSet;

const NEWLINES: &'static [u8] = &[b'\n'; u8::MAX as usize];

#[derive(Default)]
pub struct MultiBuffer {
    snapshot: Mutex<MultiBufferSnapshot>,
    buffers: HashMap<usize, BufferState>,
    subscriptions: Topic,
}

pub trait ToOffset: 'static {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize;
}

pub trait ToPoint: 'static {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point;
}

#[derive(Debug)]
struct BufferState {
    buffer: ModelHandle<Buffer>,
    last_sync: clock::Global,
    excerpts: Vec<ExcerptId>,
}

#[derive(Clone, Default)]
pub struct MultiBufferSnapshot {
    excerpts: SumTree<Excerpt>,
}

pub struct ExcerptProperties<'a, T> {
    buffer: &'a ModelHandle<Buffer>,
    range: Range<T>,
    header_height: u8,
}

#[derive(Clone)]
struct Excerpt {
    id: ExcerptId,
    buffer: buffer::BufferSnapshot,
    range: Range<text::Anchor>,
    text_summary: TextSummary,
    header_height: u8,
}

#[derive(Clone, Debug, Default)]
struct ExcerptSummary {
    excerpt_id: ExcerptId,
    text: TextSummary,
}

pub struct MultiBufferChunks<'a> {
    range: Range<usize>,
    cursor: Cursor<'a, Excerpt, usize>,
    header_height: u8,
    excerpt_chunks: Option<buffer::BufferChunks<'a>>,
    theme: Option<&'a SyntaxTheme>,
}

pub struct MultiBufferBytes<'a> {
    chunks: MultiBufferChunks<'a>,
}

impl MultiBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn singleton(buffer: ModelHandle<Buffer>, cx: &mut ModelContext<Self>) -> Self {
        let mut this = Self::new();
        this.push(
            ExcerptProperties {
                buffer: &buffer,
                range: text::Anchor::min()..text::Anchor::max(),
                header_height: 0,
            },
            cx,
        );
        this
    }

    pub fn build_simple(text: &str, cx: &mut MutableAppContext) -> ModelHandle<Self> {
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        cx.add_model(|cx| Self::singleton(buffer, cx))
    }

    pub fn snapshot(&self, cx: &AppContext) -> MultiBufferSnapshot {
        self.sync(cx);
        self.snapshot.lock().clone()
    }

    pub fn as_snapshot(&self) -> MutexGuard<MultiBufferSnapshot> {
        self.snapshot.lock()
    }

    pub fn as_singleton(&self) -> Option<&ModelHandle<Buffer>> {
        if self.buffers.len() == 1 {
            return Some(&self.buffers.values().next().unwrap().buffer);
        } else {
            None
        }
    }

    pub fn subscribe(&mut self) -> Subscription {
        self.subscriptions.subscribe()
    }

    pub fn edit<I, S, T>(&mut self, ranges_iter: I, new_text: T, cx: &mut ModelContext<Self>)
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        self.edit_internal(ranges_iter, new_text, false, cx)
    }

    pub fn edit_with_autoindent<I, S, T>(
        &mut self,
        ranges_iter: I,
        new_text: T,
        cx: &mut ModelContext<Self>,
    ) where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        self.edit_internal(ranges_iter, new_text, true, cx)
    }

    pub fn edit_internal<I, S, T>(
        &mut self,
        ranges_iter: I,
        new_text: T,
        autoindent: bool,
        cx: &mut ModelContext<Self>,
    ) where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        todo!()
    }

    pub fn start_transaction(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn end_transaction(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) {
        todo!()
    }

    pub fn redo(&mut self, cx: &mut ModelContext<Self>) {
        todo!()
    }

    pub fn selection_set(&self, set_id: SelectionSetId) -> Result<&SelectionSet> {
        todo!()
    }

    pub fn add_selection_set<T: ToOffset>(
        &mut self,
        selections: &[Selection<T>],
        cx: &mut ModelContext<Self>,
    ) -> SelectionSetId {
        todo!()
    }

    pub fn remove_selection_set(
        &mut self,
        set_id: SelectionSetId,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn update_selection_set<T: ToOffset>(
        &mut self,
        set_id: SelectionSetId,
        selections: &[Selection<T>],
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn set_active_selection_set(
        &mut self,
        set_id: Option<SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn selection_sets(&self) -> impl Iterator<Item = (&SelectionSetId, &SelectionSet)> {
        todo!();
        None.into_iter()
    }

    pub fn push<O>(&mut self, props: ExcerptProperties<O>, cx: &mut ModelContext<Self>) -> ExcerptId
    where
        O: text::ToOffset,
    {
        self.sync(cx);

        let buffer = props.buffer.read(cx);
        let range = buffer.anchor_before(props.range.start)..buffer.anchor_after(props.range.end);
        let mut snapshot = self.snapshot.lock();
        let prev_id = snapshot.excerpts.last().map(|e| &e.id);
        let id = ExcerptId::between(prev_id.unwrap_or(&ExcerptId::min()), &ExcerptId::max());

        let edit_start = snapshot.excerpts.summary().text.bytes;
        let excerpt = Excerpt::new(id.clone(), buffer.snapshot(), range, props.header_height);
        let edit = Edit {
            old: edit_start..edit_start,
            new: edit_start..edit_start + excerpt.text_summary.bytes,
        };
        snapshot.excerpts.push(excerpt, &());
        self.buffers
            .entry(props.buffer.id())
            .or_insert_with(|| BufferState {
                buffer: props.buffer.clone(),
                last_sync: buffer.version(),
                excerpts: Default::default(),
            })
            .excerpts
            .push(id.clone());

        self.subscriptions.publish_mut([edit]);

        id
    }

    pub fn save(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> Result<Task<Result<(clock::Global, SystemTime)>>> {
        todo!()
    }

    pub fn file<'a>(&self, cx: &'a AppContext) -> Option<&'a dyn File> {
        self.as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
    }

    pub fn is_dirty(&self) -> bool {
        todo!()
    }

    pub fn has_conflict(&self) -> bool {
        todo!()
    }

    pub fn is_parsing(&self, _: &AppContext) -> bool {
        todo!()
    }

    fn sync(&self, cx: &AppContext) {
        let mut snapshot = self.snapshot.lock();
        let mut excerpts_to_edit = Vec::new();
        for buffer_state in self.buffers.values() {
            if buffer_state
                .buffer
                .read(cx)
                .version()
                .gt(&buffer_state.last_sync)
            {
                excerpts_to_edit.extend(
                    buffer_state
                        .excerpts
                        .iter()
                        .map(|excerpt_id| (excerpt_id, buffer_state)),
                );
            }
        }
        excerpts_to_edit.sort_unstable_by_key(|(excerpt_id, _)| *excerpt_id);

        let mut edits = Vec::new();
        let mut new_excerpts = SumTree::new();
        let mut cursor = snapshot.excerpts.cursor::<(ExcerptId, usize)>();

        for (id, buffer_state) in excerpts_to_edit {
            new_excerpts.push_tree(cursor.slice(id, Bias::Left, &()), &());
            let old_excerpt = cursor.item().unwrap();
            let buffer = buffer_state.buffer.read(cx);

            edits.extend(
                buffer
                    .edits_since_in_range::<usize>(
                        old_excerpt.buffer.version(),
                        old_excerpt.range.clone(),
                    )
                    .map(|mut edit| {
                        let excerpt_old_start =
                            cursor.start().1 + old_excerpt.header_height as usize;
                        let excerpt_new_start =
                            new_excerpts.summary().text.bytes + old_excerpt.header_height as usize;
                        edit.old.start += excerpt_old_start;
                        edit.old.end += excerpt_old_start;
                        edit.new.start += excerpt_new_start;
                        edit.new.end += excerpt_new_start;
                        edit
                    }),
            );

            new_excerpts.push(
                Excerpt::new(
                    id.clone(),
                    buffer.snapshot(),
                    old_excerpt.range.clone(),
                    old_excerpt.header_height,
                ),
                &(),
            );

            cursor.next(&());
        }
        new_excerpts.push_tree(cursor.suffix(&()), &());

        drop(cursor);
        snapshot.excerpts = new_excerpts;

        self.subscriptions.publish(edits);
    }
}

// Methods delegating to the snapshot
impl MultiBuffer {
    pub fn replica_id(&self) -> ReplicaId {
        self.snapshot.lock().replica_id()
    }

    pub fn text(&self) -> String {
        self.snapshot.lock().text()
    }

    pub fn text_for_range<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = &'a str> {
        todo!();
        [].into_iter()
    }

    pub fn max_point(&self) -> Point {
        self.snapshot.lock().max_point()
    }

    pub fn len(&self) -> usize {
        self.snapshot.lock().len()
    }

    pub fn line_len(&self, row: u32) -> u32 {
        self.snapshot.lock().line_len(row)
    }

    pub fn is_line_blank(&self, row: u32) -> bool {
        self.snapshot.lock().is_line_blank(row)
    }

    pub fn indent_column_for_line(&self, row: u32) -> u32 {
        self.snapshot.lock().indent_column_for_line(row)
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.snapshot.lock().anchor_before(position)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.snapshot.lock().anchor_after(position)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        self.snapshot.lock().anchor_at(position, bias)
    }

    pub fn anchor_range_set<E>(
        &self,
        start_bias: Bias,
        end_bias: Bias,
        entries: E,
    ) -> AnchorRangeSet
    where
        E: IntoIterator<Item = Range<usize>>,
    {
        todo!()
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        self.snapshot.lock().clip_offset(offset, bias)
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.snapshot.lock().clip_point(point, bias)
    }

    pub fn language<'a>(&self) -> Option<&'a Arc<Language>> {
        todo!()
    }

    pub fn parse_count(&self) -> usize {
        self.snapshot.lock().parse_count()
    }

    pub fn diagnostics_update_count(&self) -> usize {
        self.snapshot.lock().diagnostics_update_count()
    }

    pub fn diagnostics_in_range<'a, T, O>(
        &'a self,
        search_range: Range<T>,
    ) -> impl Iterator<Item = (Range<O>, &Diagnostic)> + 'a
    where
        T: 'a + ToOffset,
        O: 'a,
    {
        todo!();
        None.into_iter()
    }
}

#[cfg(any(test, feature = "test-support"))]
impl MultiBuffer {
    pub fn randomly_edit<R: rand::Rng>(&mut self, _: &mut R, _: usize, _: &mut ModelContext<Self>) {
        todo!()
    }

    pub fn randomly_mutate<R: rand::Rng>(&mut self, rng: &mut R, cx: &mut ModelContext<Self>) {
        todo!()
    }
}

impl Entity for MultiBuffer {
    type Event = super::Event;
}

impl MultiBufferSnapshot {
    pub fn replica_id(&self) -> ReplicaId {
        todo!()
    }

    pub fn text(&self) -> String {
        self.chunks(0..self.len(), None)
            .map(|chunk| chunk.text)
            .collect()
    }

    pub fn reversed_chars_at<'a, T: ToOffset>(
        &'a self,
        position: T,
    ) -> impl Iterator<Item = char> + 'a {
        todo!();
        None.into_iter()
    }

    pub fn chars_at<'a, T: ToOffset>(&'a self, position: T) -> impl Iterator<Item = char> + 'a {
        let offset = position.to_offset(self);
        self.text_for_range(offset..self.len())
            .flat_map(|chunk| chunk.chars())
    }

    pub fn text_for_range<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = &'a str> {
        self.chunks(range, None).map(|chunk| chunk.text)
    }

    pub fn is_line_blank(&self, row: u32) -> bool {
        self.text_for_range(Point::new(row, 0)..Point::new(row, self.line_len(row)))
            .all(|chunk| chunk.matches(|c: char| !c.is_whitespace()).next().is_none())
    }

    pub fn contains_str_at<T>(&self, _: T, _: &str) -> bool
    where
        T: ToOffset,
    {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.excerpts.summary().text.bytes
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        let mut cursor = self.excerpts.cursor::<usize>();
        cursor.seek(&offset, Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let start_after_header = *cursor.start() + excerpt.header_height as usize;
            if offset < start_after_header {
                *cursor.start()
            } else {
                let excerpt_start = excerpt.range.start.to_offset(&excerpt.buffer);
                let buffer_offset = excerpt
                    .buffer
                    .clip_offset(excerpt_start + (offset - start_after_header), bias);
                let offset_in_excerpt = if buffer_offset > excerpt_start {
                    buffer_offset - excerpt_start
                } else {
                    0
                };
                start_after_header + offset_in_excerpt
            }
        } else {
            self.excerpts.summary().text.bytes
        }
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        let mut cursor = self.excerpts.cursor::<Point>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let start_after_header = *cursor.start() + Point::new(excerpt.header_height as u32, 0);
            if point < start_after_header {
                *cursor.start()
            } else {
                let excerpt_start = excerpt.range.start.to_point(&excerpt.buffer);
                let buffer_point = excerpt
                    .buffer
                    .clip_point(excerpt_start + (point - start_after_header), bias);
                let point_in_excerpt = if buffer_point > excerpt_start {
                    buffer_point - excerpt_start
                } else {
                    Point::zero()
                };
                start_after_header + point_in_excerpt
            }
        } else {
            self.excerpts.summary().text.lines
        }
    }

    pub fn clip_point_utf16(&self, point: PointUtf16, bias: Bias) -> PointUtf16 {
        let mut cursor = self.excerpts.cursor::<PointUtf16>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let start_after_header =
                *cursor.start() + PointUtf16::new(excerpt.header_height as u32, 0);
            if point < start_after_header {
                *cursor.start()
            } else {
                let excerpt_start = excerpt
                    .buffer
                    .offset_to_point_utf16(excerpt.range.start.to_offset(&excerpt.buffer));
                let buffer_point = excerpt
                    .buffer
                    .clip_point_utf16(excerpt_start + (point - start_after_header), bias);
                let point_in_excerpt = if buffer_point > excerpt_start {
                    buffer_point - excerpt_start
                } else {
                    PointUtf16::new(0, 0)
                };
                start_after_header + point_in_excerpt
            }
        } else {
            self.excerpts.summary().text.lines_utf16
        }
    }

    pub fn bytes_in_range<'a, T: ToOffset>(&'a self, range: Range<T>) -> MultiBufferBytes<'a> {
        todo!()
    }

    pub fn chunks<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        theme: Option<&'a SyntaxTheme>,
    ) -> MultiBufferChunks<'a> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.excerpts.cursor::<usize>();
        cursor.seek(&range.start, Bias::Right, &());

        let mut header_height: u8 = 0;
        let excerpt_chunks = cursor.item().map(|excerpt| {
            let buffer_range = excerpt.range.to_offset(&excerpt.buffer);
            header_height = excerpt.header_height;

            let buffer_start;
            let start_overshoot = range.start - cursor.start();
            if start_overshoot < excerpt.header_height as usize {
                header_height -= start_overshoot as u8;
                buffer_start = buffer_range.start;
            } else {
                buffer_start =
                    buffer_range.start + start_overshoot - excerpt.header_height as usize;
                header_height = 0;
            }

            let buffer_end;
            let end_overshoot = range.end - cursor.start();
            if end_overshoot < excerpt.header_height as usize {
                header_height -= excerpt.header_height - end_overshoot as u8;
                buffer_end = buffer_start;
            } else {
                buffer_end = cmp::min(
                    buffer_range.end,
                    buffer_range.start + end_overshoot - excerpt.header_height as usize,
                );
            }

            excerpt.buffer.chunks(buffer_start..buffer_end, theme)
        });

        MultiBufferChunks {
            range,
            cursor,
            header_height,
            excerpt_chunks,
            theme,
        }
    }

    pub fn offset_to_point(&self, offset: usize) -> Point {
        let mut cursor = self.excerpts.cursor::<(usize, Point)>();
        cursor.seek(&offset, Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let (start_offset, start_point) = cursor.start();
            let overshoot = offset - start_offset;
            let header_height = excerpt.header_height as usize;
            if overshoot < header_height {
                *start_point
            } else {
                let excerpt_start_offset = excerpt.range.start.to_offset(&excerpt.buffer);
                let excerpt_start_point = excerpt.range.start.to_point(&excerpt.buffer);
                let buffer_point = excerpt
                    .buffer
                    .offset_to_point(excerpt_start_offset + (overshoot - header_height));
                *start_point
                    + Point::new(header_height as u32, 0)
                    + (buffer_point - excerpt_start_point)
            }
        } else {
            self.excerpts.summary().text.lines
        }
    }

    pub fn point_to_offset(&self, point: Point) -> usize {
        let mut cursor = self.excerpts.cursor::<(Point, usize)>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let (start_point, start_offset) = cursor.start();
            let overshoot = point - start_point;
            let header_height = Point::new(excerpt.header_height as u32, 0);
            if overshoot < header_height {
                *start_offset
            } else {
                let excerpt_start_offset = excerpt.range.start.to_offset(&excerpt.buffer);
                let excerpt_start_point = excerpt.range.start.to_point(&excerpt.buffer);
                let buffer_offset = excerpt
                    .buffer
                    .point_to_offset(excerpt_start_point + (overshoot - header_height));
                *start_offset + excerpt.header_height as usize + buffer_offset
                    - excerpt_start_offset
            }
        } else {
            self.excerpts.summary().text.bytes
        }
    }

    pub fn point_utf16_to_offset(&self, point: PointUtf16) -> usize {
        let mut cursor = self.excerpts.cursor::<(PointUtf16, usize)>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let (start_point, start_offset) = cursor.start();
            let overshoot = point - start_point;
            let header_height = PointUtf16::new(excerpt.header_height as u32, 0);
            if overshoot < header_height {
                *start_offset
            } else {
                let excerpt_start_offset = excerpt.range.start.to_offset(&excerpt.buffer);
                let excerpt_start_point = excerpt
                    .buffer
                    .offset_to_point_utf16(excerpt.range.start.to_offset(&excerpt.buffer));
                let buffer_offset = excerpt
                    .buffer
                    .point_utf16_to_offset(excerpt_start_point + (overshoot - header_height));
                *start_offset
                    + excerpt.header_height as usize
                    + (buffer_offset - excerpt_start_offset)
            }
        } else {
            self.excerpts.summary().text.bytes
        }
    }

    pub fn indent_column_for_line(&self, row: u32) -> u32 {
        todo!()
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let mut cursor = self.excerpts.cursor::<Point>();
        cursor.seek(&Point::new(row, 0), Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let overshoot = row - cursor.start().row;
            let header_height = excerpt.header_height as u32;
            if overshoot < header_height {
                0
            } else {
                let excerpt_start = excerpt.range.start.to_point(&excerpt.buffer);
                let excerpt_end = excerpt.range.end.to_point(&excerpt.buffer);
                let buffer_row = excerpt_start.row + overshoot - header_height;
                let mut len = excerpt.buffer.line_len(buffer_row);
                if buffer_row == excerpt_end.row {
                    len = excerpt_end.column;
                }
                if buffer_row == excerpt_start.row {
                    len -= excerpt_start.column
                }
                len
            }
        } else {
            0
        }
    }

    pub fn max_point(&self) -> Point {
        self.text_summary().lines
    }

    pub fn text_summary(&self) -> TextSummary {
        self.excerpts.summary().text
    }

    pub fn text_summary_for_range<'a, D, O>(&'a self, range: Range<O>) -> D
    where
        D: TextDimension,
        O: ToOffset,
    {
        let mut summary = D::default();
        let mut range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.excerpts.cursor::<usize>();
        cursor.seek(&range.start, Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let start_after_header = cursor.start() + excerpt.header_height as usize;
            if range.start < start_after_header {
                let header_len = cmp::min(range.end, start_after_header) - range.start;
                summary.add_assign(&D::from_text_summary(&TextSummary {
                    bytes: header_len,
                    lines: Point::new(header_len as u32, 0),
                    lines_utf16: PointUtf16::new(header_len as u32, 0),
                    first_line_chars: 0,
                    last_line_chars: 0,
                    longest_row: 0,
                    longest_row_chars: 0,
                }));
                range.start = start_after_header;
                range.end = cmp::max(range.start, range.end);
            }

            let end_before_newline = cursor.end(&()) - 1;
            let excerpt_start = excerpt.range.start.to_offset(&excerpt.buffer);
            let start_in_excerpt = excerpt_start + (range.start - start_after_header);
            let end_in_excerpt =
                excerpt_start + (cmp::min(end_before_newline, range.end) - start_after_header);
            summary.add_assign(
                &excerpt
                    .buffer
                    .text_summary_for_range(start_in_excerpt..end_in_excerpt),
            );

            if range.end > end_before_newline {
                summary.add_assign(&D::from_text_summary(&TextSummary {
                    bytes: 1,
                    lines: Point::new(1 as u32, 0),
                    lines_utf16: PointUtf16::new(1 as u32, 0),
                    first_line_chars: 0,
                    last_line_chars: 0,
                    longest_row: 0,
                    longest_row_chars: 0,
                }));
            }

            cursor.next(&());
        }

        if range.end > *cursor.start() {
            summary.add_assign(&D::from_text_summary(&cursor.summary::<_, TextSummary>(
                &range.end,
                Bias::Right,
                &(),
            )));
            if let Some(excerpt) = cursor.item() {
                let start_after_header = cursor.start() + excerpt.header_height as usize;
                let header_len =
                    cmp::min(range.end - cursor.start(), excerpt.header_height as usize);
                summary.add_assign(&D::from_text_summary(&TextSummary {
                    bytes: header_len,
                    lines: Point::new(header_len as u32, 0),
                    lines_utf16: PointUtf16::new(header_len as u32, 0),
                    first_line_chars: 0,
                    last_line_chars: 0,
                    longest_row: 0,
                    longest_row_chars: 0,
                }));
                range.end = cmp::max(start_after_header, range.end);

                let excerpt_start = excerpt.range.start.to_offset(&excerpt.buffer);
                let end_in_excerpt = excerpt_start + (range.end - start_after_header);
                summary.add_assign(
                    &excerpt
                        .buffer
                        .text_summary_for_range(excerpt_start..end_in_excerpt),
                );
                cursor.next(&());
            }
        }

        summary
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        todo!()
    }

    pub fn parse_count(&self) -> usize {
        todo!()
    }

    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(Range<usize>, Range<usize>)> {
        todo!()
    }

    pub fn diagnostics_update_count(&self) -> usize {
        todo!()
    }

    pub fn language<'a>(&self) -> Option<&'a Arc<Language>> {
        todo!()
    }

    pub fn diagnostic_group<'a, O>(
        &'a self,
        group_id: usize,
    ) -> impl Iterator<Item = (Range<O>, &Diagnostic)> + 'a
    where
        O: 'a,
    {
        todo!();
        None.into_iter()
    }

    pub fn diagnostics_in_range<'a, T, O>(
        &'a self,
        search_range: Range<T>,
    ) -> impl Iterator<Item = (Range<O>, &Diagnostic)> + 'a
    where
        T: 'a + ToOffset,
        O: 'a,
    {
        todo!();
        None.into_iter()
    }

    pub fn range_for_syntax_ancestor<T: ToOffset>(&self, range: Range<T>) -> Option<Range<usize>> {
        todo!()
    }

    fn buffer_snapshot_for_excerpt<'a>(
        &'a self,
        excerpt_id: &ExcerptId,
    ) -> Option<&'a BufferSnapshot> {
        let mut cursor = self.excerpts.cursor::<ExcerptId>();
        cursor.seek(excerpt_id, Bias::Left, &());
        if let Some(excerpt) = cursor.item() {
            if cursor.start() == excerpt_id {
                return Some(&excerpt.buffer);
            }
        }
        None
    }
}

impl Excerpt {
    fn new(
        id: ExcerptId,
        buffer: buffer::BufferSnapshot,
        range: Range<text::Anchor>,
        header_height: u8,
    ) -> Self {
        let mut text_summary =
            buffer.text_summary_for_range::<TextSummary, _>(range.to_offset(&buffer));
        if header_height > 0 {
            text_summary.first_line_chars = 0;
            text_summary.lines.row += header_height as u32;
            text_summary.lines_utf16.row += header_height as u32;
            text_summary.bytes += header_height as usize;
            text_summary.longest_row += header_height as u32;
        }
        text_summary.last_line_chars = 0;
        text_summary.lines.row += 1;
        text_summary.lines.column = 0;
        text_summary.lines_utf16.row += 1;
        text_summary.lines_utf16.column = 0;
        text_summary.bytes += 1;

        Excerpt {
            id,
            buffer,
            range,
            text_summary,
            header_height,
        }
    }

    fn header_summary(&self) -> TextSummary {
        TextSummary {
            bytes: self.header_height as usize,
            lines: Point::new(self.header_height as u32, 0),
            lines_utf16: PointUtf16::new(self.header_height as u32, 0),
            first_line_chars: 0,
            last_line_chars: 0,
            longest_row: 0,
            longest_row_chars: 0,
        }
    }
}

impl sum_tree::Item for Excerpt {
    type Summary = ExcerptSummary;

    fn summary(&self) -> Self::Summary {
        ExcerptSummary {
            excerpt_id: self.id.clone(),
            text: self.text_summary.clone(),
        }
    }
}

impl sum_tree::Summary for ExcerptSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(summary.excerpt_id > self.excerpt_id);
        self.excerpt_id = summary.excerpt_id.clone();
        self.text.add_summary(&summary.text, &());
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for TextSummary {
    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        *self += &summary.text;
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for usize {
    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        *self += summary.text.bytes;
    }
}

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, ExcerptSummary> for usize {
    fn cmp(&self, cursor_location: &ExcerptSummary, _: &()) -> cmp::Ordering {
        Ord::cmp(self, &cursor_location.text.bytes)
    }
}

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, ExcerptSummary> for Location {
    fn cmp(&self, cursor_location: &ExcerptSummary, _: &()) -> cmp::Ordering {
        Ord::cmp(self, &cursor_location.excerpt_id)
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for Point {
    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        *self += summary.text.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for PointUtf16 {
    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        *self += summary.text.lines_utf16
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for Location {
    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        debug_assert!(summary.excerpt_id > *self);
        *self = summary.excerpt_id.clone();
    }
}

impl<'a> MultiBufferChunks<'a> {
    pub fn offset(&self) -> usize {
        todo!()
    }

    pub fn seek(&mut self, offset: usize) {
        todo!()
    }
}

impl<'a> Iterator for MultiBufferChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.header_height > 0 {
                let chunk = Chunk {
                    text: unsafe {
                        std::str::from_utf8_unchecked(&NEWLINES[..self.header_height as usize])
                    },
                    ..Default::default()
                };
                self.header_height = 0;
                return Some(chunk);
            }

            if let Some(excerpt_chunks) = self.excerpt_chunks.as_mut() {
                if let Some(chunk) = excerpt_chunks.next() {
                    return Some(chunk);
                }
                self.excerpt_chunks.take();
                if self.cursor.end(&()) <= self.range.end {
                    return Some(Chunk {
                        text: "\n",
                        ..Default::default()
                    });
                }
            }

            self.cursor.next(&());
            if *self.cursor.start() >= self.range.end {
                return None;
            }

            let excerpt = self.cursor.item()?;
            let buffer_range = excerpt.range.to_offset(&excerpt.buffer);

            let buffer_end = cmp::min(
                buffer_range.end,
                buffer_range.start + self.range.end
                    - excerpt.header_height as usize
                    - self.cursor.start(),
            );

            self.header_height = excerpt.header_height;
            self.excerpt_chunks = Some(
                excerpt
                    .buffer
                    .chunks(buffer_range.start..buffer_end, self.theme),
            );
        }
    }
}

impl<'a> Iterator for MultiBufferBytes<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> io::Read for MultiBufferBytes<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}

impl ToOffset for Point {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        snapshot.point_to_offset(*self)
    }
}

impl ToOffset for PointUtf16 {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        snapshot.point_utf16_to_offset(*self)
    }
}

impl ToOffset for usize {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        assert!(*self <= snapshot.len(), "offset is out of range");
        *self
    }
}

impl ToPoint for usize {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point {
        snapshot.offset_to_point(*self)
    }
}

impl ToPoint for Point {
    fn to_point<'a>(&self, _: &MultiBufferSnapshot) -> Point {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use gpui::MutableAppContext;
    use rand::prelude::*;
    use std::env;
    use text::{Point, RandomCharIter};
    use util::test::sample_text;

    #[gpui::test]
    fn test_excerpt_buffer(cx: &mut MutableAppContext) {
        let buffer_1 = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6, 'a'), cx));
        let buffer_2 = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6, 'g'), cx));

        let list = cx.add_model(|_| MultiBuffer::new());

        let subscription = list.update(cx, |list, cx| {
            let subscription = list.subscribe();
            list.push(
                ExcerptProperties {
                    buffer: &buffer_1,
                    range: Point::new(1, 2)..Point::new(2, 5),
                    header_height: 2,
                },
                cx,
            );
            assert_eq!(
                subscription.consume().into_inner(),
                [Edit {
                    old: 0..0,
                    new: 0..13
                }]
            );

            list.push(
                ExcerptProperties {
                    buffer: &buffer_1,
                    range: Point::new(3, 3)..Point::new(4, 4),
                    header_height: 1,
                },
                cx,
            );
            list.push(
                ExcerptProperties {
                    buffer: &buffer_2,
                    range: Point::new(3, 1)..Point::new(3, 3),
                    header_height: 3,
                },
                cx,
            );
            assert_eq!(
                subscription.consume().into_inner(),
                [Edit {
                    old: 13..13,
                    new: 13..29
                }]
            );

            subscription
        });

        assert_eq!(
            list.read(cx).snapshot(cx).text(),
            concat!(
                "\n",      // Preserve newlines
                "\n",      //
                "bbbb\n",  //
                "ccccc\n", //
                "\n",      //
                "ddd\n",   //
                "eeee\n",  //
                "\n",      //
                "\n",      //
                "\n",      //
                "jj\n"     //
            )
        );

        buffer_1.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    Point::new(0, 0)..Point::new(0, 0),
                    Point::new(2, 1)..Point::new(2, 3),
                ],
                "\n",
                cx,
            );
        });

        assert_eq!(
            list.read(cx).snapshot(cx).text(),
            concat!(
                "\n",     // Preserve newlines
                "\n",     //
                "bbbb\n", //
                "c\n",    //
                "cc\n",   //
                "\n",     //
                "ddd\n",  //
                "eeee\n", //
                "\n",     //
                "\n",     //
                "\n",     //
                "jj\n"    //
            )
        );

        assert_eq!(
            subscription.consume().into_inner(),
            [Edit {
                old: 8..10,
                new: 8..9
            }]
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_excerpts(cx: &mut MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut buffers: Vec<ModelHandle<Buffer>> = Vec::new();
        let list = cx.add_model(|_| MultiBuffer::new());
        let mut excerpt_ids = Vec::new();
        let mut expected_excerpts = Vec::new();
        let mut old_versions = Vec::new();

        for _ in 0..operations {
            match rng.gen_range(0..100) {
                0..=19 if !buffers.is_empty() => {
                    let buffer = buffers.choose(&mut rng).unwrap();
                    buffer.update(cx, |buf, cx| buf.randomly_edit(&mut rng, 1, cx));
                }
                _ => {
                    let buffer_handle = if buffers.is_empty() || rng.gen_bool(0.4) {
                        let base_text = RandomCharIter::new(&mut rng).take(10).collect::<String>();
                        buffers.push(cx.add_model(|cx| Buffer::new(0, base_text, cx)));
                        buffers.last().unwrap()
                    } else {
                        buffers.choose(&mut rng).unwrap()
                    };

                    let buffer = buffer_handle.read(cx);
                    let end_ix = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Bias::Right);
                    let start_ix = buffer.clip_offset(rng.gen_range(0..=end_ix), Bias::Left);
                    let header_height = rng.gen_range(0..=5);
                    let anchor_range = buffer.anchor_before(start_ix)..buffer.anchor_after(end_ix);
                    log::info!(
                        "Pushing excerpt wih header {}, buffer {}: {:?}[{:?}] = {:?}",
                        header_height,
                        buffer_handle.id(),
                        buffer.text(),
                        start_ix..end_ix,
                        &buffer.text()[start_ix..end_ix]
                    );

                    let excerpt_id = list.update(cx, |list, cx| {
                        list.push(
                            ExcerptProperties {
                                buffer: &buffer_handle,
                                range: start_ix..end_ix,
                                header_height,
                            },
                            cx,
                        )
                    });
                    excerpt_ids.push(excerpt_id);
                    expected_excerpts.push((buffer_handle.clone(), anchor_range, header_height));
                }
            }

            if rng.gen_bool(0.3) {
                list.update(cx, |list, cx| {
                    old_versions.push((list.snapshot(cx), list.subscribe()));
                })
            }

            let snapshot = list.read(cx).snapshot(cx);

            let mut excerpt_starts = Vec::new();
            let mut expected_text = String::new();
            for (buffer, range, header_height) in &expected_excerpts {
                let buffer = buffer.read(cx);
                let buffer_range = range.to_offset(buffer);

                for _ in 0..*header_height {
                    expected_text.push('\n');
                }

                excerpt_starts.push(TextSummary::from(expected_text.as_str()));
                expected_text.extend(buffer.text_for_range(buffer_range.clone()));
                expected_text.push('\n');
            }

            assert_eq!(snapshot.text(), expected_text);

            let mut excerpt_starts = excerpt_starts.into_iter();
            for (buffer, range, _) in &expected_excerpts {
                let buffer_id = buffer.id();
                let buffer = buffer.read(cx);
                let buffer_range = range.to_offset(buffer);
                let buffer_start_point = buffer.offset_to_point(buffer_range.start);
                let buffer_start_point_utf16 =
                    buffer.text_summary_for_range::<PointUtf16, _>(0..buffer_range.start);

                let excerpt_start = excerpt_starts.next().unwrap();
                let mut offset = excerpt_start.bytes;
                let mut buffer_offset = buffer_range.start;
                let mut point = excerpt_start.lines;
                let mut buffer_point = buffer_start_point;
                let mut point_utf16 = excerpt_start.lines_utf16;
                let mut buffer_point_utf16 = buffer_start_point_utf16;
                for byte in buffer.bytes_in_range(buffer_range.clone()).flatten() {
                    let left_offset = snapshot.clip_offset(offset, Bias::Left);
                    let right_offset = snapshot.clip_offset(offset, Bias::Right);
                    let buffer_left_offset = buffer.clip_offset(buffer_offset, Bias::Left);
                    let buffer_right_offset = buffer.clip_offset(buffer_offset, Bias::Right);
                    assert_eq!(
                        left_offset,
                        excerpt_start.bytes + (buffer_left_offset - buffer_range.start),
                        "clip_offset({:?}, Left). buffer: {:?}, buffer offset: {:?}",
                        offset,
                        buffer_id,
                        buffer_offset,
                    );
                    assert_eq!(
                        right_offset,
                        excerpt_start.bytes + (buffer_right_offset - buffer_range.start),
                        "clip_offset({:?}, Right). buffer: {:?}, buffer offset: {:?}",
                        offset,
                        buffer_id,
                        buffer_offset,
                    );

                    let left_point = snapshot.clip_point(point, Bias::Left);
                    let right_point = snapshot.clip_point(point, Bias::Right);
                    let buffer_left_point = buffer.clip_point(buffer_point, Bias::Left);
                    let buffer_right_point = buffer.clip_point(buffer_point, Bias::Right);
                    assert_eq!(
                        left_point,
                        excerpt_start.lines + (buffer_left_point - buffer_start_point),
                        "clip_point({:?}, Left). buffer: {:?}, buffer point: {:?}",
                        point,
                        buffer_id,
                        buffer_point,
                    );
                    assert_eq!(
                        right_point,
                        excerpt_start.lines + (buffer_right_point - buffer_start_point),
                        "clip_point({:?}, Right). buffer: {:?}, buffer point: {:?}",
                        point,
                        buffer_id,
                        buffer_point,
                    );

                    let left_point_utf16 = snapshot.clip_point_utf16(point_utf16, Bias::Left);
                    let right_point_utf16 = snapshot.clip_point_utf16(point_utf16, Bias::Right);
                    let buffer_left_point_utf16 =
                        buffer.clip_point_utf16(buffer_point_utf16, Bias::Left);
                    let buffer_right_point_utf16 =
                        buffer.clip_point_utf16(buffer_point_utf16, Bias::Right);
                    assert_eq!(
                        left_point_utf16,
                        excerpt_start.lines_utf16
                            + (buffer_left_point_utf16 - buffer_start_point_utf16),
                        "clip_point_utf16({:?}, Left). buffer: {:?}, buffer point_utf16: {:?}",
                        point_utf16,
                        buffer_id,
                        buffer_point_utf16,
                    );
                    assert_eq!(
                        right_point_utf16,
                        excerpt_start.lines_utf16
                            + (buffer_right_point_utf16 - buffer_start_point_utf16),
                        "clip_point_utf16({:?}, Right). buffer: {:?}, buffer point_utf16: {:?}",
                        point_utf16,
                        buffer_id,
                        buffer_point_utf16,
                    );

                    assert_eq!(
                        snapshot.point_to_offset(left_point),
                        left_offset,
                        "point_to_offset({:?})",
                        left_point,
                    );
                    assert_eq!(
                        snapshot.offset_to_point(left_offset),
                        left_point,
                        "offset_to_point({:?})",
                        left_offset,
                    );

                    offset += 1;
                    buffer_offset += 1;
                    if *byte == b'\n' {
                        point += Point::new(1, 0);
                        point_utf16 += PointUtf16::new(1, 0);
                        buffer_point += Point::new(1, 0);
                        buffer_point_utf16 += PointUtf16::new(1, 0);
                    } else {
                        point += Point::new(0, 1);
                        point_utf16 += PointUtf16::new(0, 1);
                        buffer_point += Point::new(0, 1);
                        buffer_point_utf16 += PointUtf16::new(0, 1);
                    }
                }
            }

            for (row, line) in expected_text.split('\n').enumerate() {
                assert_eq!(
                    snapshot.line_len(row as u32),
                    line.len() as u32,
                    "line_len({}).",
                    row
                );
            }

            for _ in 0..10 {
                let end_ix = snapshot.clip_offset(rng.gen_range(0..=snapshot.len()), Bias::Right);
                let start_ix = snapshot.clip_offset(rng.gen_range(0..=end_ix), Bias::Left);

                assert_eq!(
                    snapshot
                        .text_for_range(start_ix..end_ix)
                        .collect::<String>(),
                    &expected_text[start_ix..end_ix],
                    "incorrect text for range {:?}",
                    start_ix..end_ix
                );

                let expected_summary = TextSummary::from(&expected_text[start_ix..end_ix]);
                assert_eq!(
                    snapshot.text_summary_for_range::<TextSummary, _>(start_ix..end_ix),
                    expected_summary,
                    "incorrect summary for range {:?}",
                    start_ix..end_ix
                );
            }
        }

        let snapshot = list.read(cx).snapshot(cx);
        for (old_snapshot, subscription) in old_versions {
            let edits = subscription.consume().into_inner();

            log::info!(
                "applying edits since old text: {:?}: {:?}",
                old_snapshot.text(),
                edits,
            );

            let mut text = old_snapshot.text();
            for edit in edits {
                let new_text: String = snapshot.text_for_range(edit.new.clone()).collect();
                text.replace_range(edit.new.start..edit.new.start + edit.old.len(), &new_text);
            }
            assert_eq!(text.to_string(), snapshot.text());
        }
    }
}
