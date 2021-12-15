mod anchor;

pub use anchor::{Anchor, AnchorRangeExt};
use anyhow::Result;
use clock::ReplicaId;
use collections::{HashMap, HashSet};
use gpui::{AppContext, ElementBox, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use language::{
    Buffer, BufferChunks, BufferSnapshot, Chunk, DiagnosticEntry, Event, File, Language, Selection,
    ToOffset as _, ToPoint as _, TransactionId,
};
use std::{
    cell::{Ref, RefCell},
    cmp, io,
    iter::{self, FromIterator, Peekable},
    ops::{Range, Sub},
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use sum_tree::{Bias, Cursor, SumTree};
use text::{
    locator::Locator,
    rope::TextDimension,
    subscription::{Subscription, Topic},
    AnchorRangeExt as _, Edit, Point, PointUtf16, TextSummary,
};
use theme::SyntaxTheme;
use util::post_inc;

const NEWLINES: &'static [u8] = &[b'\n'; u8::MAX as usize];

pub type ExcerptId = Locator;

pub struct MultiBuffer {
    snapshot: RefCell<MultiBufferSnapshot>,
    buffers: HashMap<usize, BufferState>,
    subscriptions: Topic,
    singleton: bool,
    replica_id: ReplicaId,
    history: History,
}

struct History {
    next_transaction_id: usize,
    undo_stack: Vec<Transaction>,
    redo_stack: Vec<Transaction>,
    transaction_depth: usize,
    group_interval: Duration,
}

struct Transaction {
    id: usize,
    buffer_transactions: HashSet<(usize, text::TransactionId)>,
    first_edit_at: Instant,
    last_edit_at: Instant,
}

pub trait ToOffset: 'static + std::fmt::Debug {
    fn to_offset(&self, snapshot: &MultiBufferSnapshot) -> usize;
}

pub trait ToPoint: 'static + std::fmt::Debug {
    fn to_point(&self, snapshot: &MultiBufferSnapshot) -> Point;
}

pub trait FromAnchor: 'static {
    fn from_anchor(anchor: &Anchor, snapshot: &MultiBufferSnapshot) -> Self;
}

#[derive(Debug)]
struct BufferState {
    buffer: ModelHandle<Buffer>,
    last_version: clock::Global,
    last_parse_count: usize,
    last_diagnostics_update_count: usize,
    excerpts: Vec<ExcerptId>,
}

#[derive(Clone, Default)]
pub struct MultiBufferSnapshot {
    excerpts: SumTree<Excerpt>,
    parse_count: usize,
    diagnostics_update_count: usize,
}

pub type RenderHeaderFn = Arc<dyn 'static + Send + Sync + Fn(&AppContext) -> ElementBox>;

pub struct ExcerptProperties<'a, T> {
    pub buffer: &'a ModelHandle<Buffer>,
    pub range: Range<T>,
    pub header_height: u8,
    pub render_header: Option<RenderHeaderFn>,
}

#[derive(Clone)]
struct Excerpt {
    id: ExcerptId,
    buffer_id: usize,
    buffer: BufferSnapshot,
    range: Range<text::Anchor>,
    render_header: Option<RenderHeaderFn>,
    text_summary: TextSummary,
    header_height: u8,
    has_trailing_newline: bool,
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
    has_trailing_newline: bool,
    excerpt_chunks: Option<BufferChunks<'a>>,
    theme: Option<&'a SyntaxTheme>,
}

pub struct MultiBufferBytes<'a> {
    chunks: Peekable<MultiBufferChunks<'a>>,
}

impl MultiBuffer {
    pub fn new(replica_id: ReplicaId) -> Self {
        Self {
            snapshot: Default::default(),
            buffers: Default::default(),
            subscriptions: Default::default(),
            singleton: false,
            replica_id,
            history: History {
                next_transaction_id: Default::default(),
                undo_stack: Default::default(),
                redo_stack: Default::default(),
                transaction_depth: 0,
                group_interval: Duration::from_millis(300),
            },
        }
    }

    pub fn singleton(buffer: ModelHandle<Buffer>, cx: &mut ModelContext<Self>) -> Self {
        let mut this = Self::new(buffer.read(cx).replica_id());
        this.singleton = true;
        this.push_excerpt(
            ExcerptProperties {
                buffer: &buffer,
                range: text::Anchor::min()..text::Anchor::max(),
                header_height: 0,
                render_header: None,
            },
            cx,
        );
        this
    }

    pub fn build_simple(text: &str, cx: &mut MutableAppContext) -> ModelHandle<Self> {
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));
        cx.add_model(|cx| Self::singleton(buffer, cx))
    }

    pub fn replica_id(&self) -> ReplicaId {
        self.replica_id
    }

    pub fn snapshot(&self, cx: &AppContext) -> MultiBufferSnapshot {
        self.sync(cx);
        self.snapshot.borrow().clone()
    }

    pub fn read(&self, cx: &AppContext) -> Ref<MultiBufferSnapshot> {
        self.sync(cx);
        self.snapshot.borrow()
    }

    pub fn as_singleton(&self) -> Option<&ModelHandle<Buffer>> {
        if self.singleton {
            return Some(&self.buffers.values().next().unwrap().buffer);
        } else {
            None
        }
    }

    pub fn subscribe(&mut self) -> Subscription {
        self.subscriptions.subscribe()
    }

    pub fn edit<I, S, T>(&mut self, ranges: I, new_text: T, cx: &mut ModelContext<Self>)
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        self.edit_internal(ranges, new_text, false, cx)
    }

    pub fn edit_with_autoindent<I, S, T>(
        &mut self,
        ranges: I,
        new_text: T,
        cx: &mut ModelContext<Self>,
    ) where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        self.edit_internal(ranges, new_text, true, cx)
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
        if let Some(buffer) = self.as_singleton() {
            let snapshot = self.read(cx);
            let ranges = ranges_iter
                .into_iter()
                .map(|range| range.start.to_offset(&snapshot)..range.end.to_offset(&snapshot));
            return buffer.update(cx, |buffer, cx| {
                if autoindent {
                    buffer.edit_with_autoindent(ranges, new_text, cx)
                } else {
                    buffer.edit(ranges, new_text, cx)
                }
            });
        }

        let snapshot = self.read(cx);
        let mut buffer_edits: HashMap<usize, Vec<(Range<usize>, bool)>> = Default::default();
        let mut cursor = snapshot.excerpts.cursor::<usize>();
        for range in ranges_iter {
            let start = range.start.to_offset(&snapshot);
            let end = range.end.to_offset(&snapshot);
            cursor.seek(&start, Bias::Right, &());
            let start_excerpt = cursor.item().expect("start offset out of bounds");
            let start_overshoot =
                (start - cursor.start()).saturating_sub(start_excerpt.header_height as usize);
            let buffer_start =
                start_excerpt.range.start.to_offset(&start_excerpt.buffer) + start_overshoot;

            cursor.seek(&end, Bias::Right, &());
            let end_excerpt = cursor.item().expect("end offset out of bounds");
            let end_overshoot =
                (end - cursor.start()).saturating_sub(end_excerpt.header_height as usize);
            let buffer_end = end_excerpt.range.start.to_offset(&end_excerpt.buffer) + end_overshoot;

            if start_excerpt.id == end_excerpt.id {
                buffer_edits
                    .entry(start_excerpt.buffer_id)
                    .or_insert(Vec::new())
                    .push((buffer_start..buffer_end, true));
            } else {
                let start_excerpt_range =
                    buffer_start..start_excerpt.range.end.to_offset(&start_excerpt.buffer);
                let end_excerpt_range =
                    end_excerpt.range.start.to_offset(&end_excerpt.buffer)..buffer_end;
                buffer_edits
                    .entry(start_excerpt.buffer_id)
                    .or_insert(Vec::new())
                    .push((start_excerpt_range, true));
                buffer_edits
                    .entry(end_excerpt.buffer_id)
                    .or_insert(Vec::new())
                    .push((end_excerpt_range, false));

                cursor.seek(&start, Bias::Right, &());
                cursor.next(&());
                while let Some(excerpt) = cursor.item() {
                    if excerpt.id == end_excerpt.id {
                        break;
                    }

                    let excerpt_range = start_excerpt.range.end.to_offset(&start_excerpt.buffer)
                        ..start_excerpt.range.end.to_offset(&start_excerpt.buffer);
                    buffer_edits
                        .entry(excerpt.buffer_id)
                        .or_insert(Vec::new())
                        .push((excerpt_range, false));
                    cursor.next(&());
                }
            }
        }

        let new_text = new_text.into();
        for (buffer_id, mut edits) in buffer_edits {
            edits.sort_unstable_by_key(|(range, _)| range.start);
            self.buffers[&buffer_id].buffer.update(cx, |buffer, cx| {
                let mut edits = edits.into_iter().peekable();
                let mut insertions = Vec::new();
                let mut deletions = Vec::new();
                while let Some((mut range, mut is_insertion)) = edits.next() {
                    while let Some((next_range, next_is_insertion)) = edits.peek() {
                        if range.end >= next_range.start {
                            range.end = cmp::max(next_range.end, range.end);
                            is_insertion |= *next_is_insertion;
                            edits.next();
                        } else {
                            break;
                        }
                    }

                    if is_insertion {
                        insertions.push(
                            buffer.anchor_before(range.start)..buffer.anchor_before(range.end),
                        );
                    } else {
                        deletions.push(
                            buffer.anchor_before(range.start)..buffer.anchor_before(range.end),
                        );
                    }
                }

                if autoindent {
                    buffer.edit_with_autoindent(deletions, "", cx);
                    buffer.edit_with_autoindent(insertions, new_text.clone(), cx);
                } else {
                    buffer.edit(deletions, "", cx);
                    buffer.edit(insertions, new_text.clone(), cx);
                }
            })
        }
    }

    pub fn start_transaction(&mut self, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        self.start_transaction_at(Instant::now(), cx)
    }

    pub(crate) fn start_transaction_at(
        &mut self,
        now: Instant,
        cx: &mut ModelContext<Self>,
    ) -> Option<TransactionId> {
        if let Some(buffer) = self.as_singleton() {
            return buffer.update(cx, |buffer, _| buffer.start_transaction_at(now));
        }

        for BufferState { buffer, .. } in self.buffers.values() {
            buffer.update(cx, |buffer, _| buffer.start_transaction_at(now));
        }
        self.history.start_transaction(now)
    }

    pub fn end_transaction(&mut self, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        self.end_transaction_at(Instant::now(), cx)
    }

    pub(crate) fn end_transaction_at(
        &mut self,
        now: Instant,
        cx: &mut ModelContext<Self>,
    ) -> Option<TransactionId> {
        if let Some(buffer) = self.as_singleton() {
            return buffer.update(cx, |buffer, cx| buffer.end_transaction_at(now, cx));
        }

        let mut buffer_transactions = HashSet::default();
        for BufferState { buffer, .. } in self.buffers.values() {
            if let Some(transaction_id) =
                buffer.update(cx, |buffer, cx| buffer.end_transaction_at(now, cx))
            {
                buffer_transactions.insert((buffer.id(), transaction_id));
            }
        }

        if self.history.end_transaction(now, buffer_transactions) {
            let transaction_id = self.history.group().unwrap();
            Some(transaction_id)
        } else {
            None
        }
    }

    pub fn set_active_selections(
        &mut self,
        selections: &[Selection<Anchor>],
        cx: &mut ModelContext<Self>,
    ) {
        let mut selections_by_buffer: HashMap<usize, Vec<Selection<text::Anchor>>> =
            Default::default();
        let snapshot = self.read(cx);
        let mut cursor = snapshot.excerpts.cursor::<Option<&ExcerptId>>();
        for selection in selections {
            cursor.seek(&Some(&selection.start.excerpt_id), Bias::Left, &());
            while let Some(excerpt) = cursor.item() {
                if excerpt.id > selection.end.excerpt_id {
                    break;
                }

                let mut start = excerpt.range.start.clone();
                let mut end = excerpt.range.end.clone();
                if excerpt.id == selection.start.excerpt_id {
                    start = selection.start.text_anchor.clone();
                }
                if excerpt.id == selection.end.excerpt_id {
                    end = selection.end.text_anchor.clone();
                }
                selections_by_buffer
                    .entry(excerpt.buffer_id)
                    .or_default()
                    .push(Selection {
                        id: selection.id,
                        start,
                        end,
                        reversed: selection.reversed,
                        goal: selection.goal,
                    });

                cursor.next(&());
            }
        }

        for (buffer_id, mut selections) in selections_by_buffer {
            self.buffers[&buffer_id].buffer.update(cx, |buffer, cx| {
                selections.sort_unstable_by(|a, b| a.start.cmp(&b.start, buffer).unwrap());
                let mut selections = selections.into_iter().peekable();
                let merged_selections = Arc::from_iter(iter::from_fn(|| {
                    let mut selection = selections.next()?;
                    while let Some(next_selection) = selections.peek() {
                        if selection
                            .end
                            .cmp(&next_selection.start, buffer)
                            .unwrap()
                            .is_ge()
                        {
                            let next_selection = selections.next().unwrap();
                            if next_selection
                                .end
                                .cmp(&selection.end, buffer)
                                .unwrap()
                                .is_ge()
                            {
                                selection.end = next_selection.end;
                            }
                        } else {
                            break;
                        }
                    }
                    Some(selection)
                }));
                buffer.set_active_selections(merged_selections, cx);
            });
        }
    }

    pub fn remove_active_selections(&mut self, cx: &mut ModelContext<Self>) {
        for buffer in self.buffers.values() {
            buffer
                .buffer
                .update(cx, |buffer, cx| buffer.remove_active_selections(cx));
        }
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        if let Some(buffer) = self.as_singleton() {
            return buffer.update(cx, |buffer, cx| buffer.undo(cx));
        }

        while let Some(transaction) = self.history.pop_undo() {
            let mut undone = false;
            for (buffer_id, buffer_transaction_id) in &transaction.buffer_transactions {
                if let Some(BufferState { buffer, .. }) = self.buffers.get(&buffer_id) {
                    undone |= buffer.update(cx, |buf, cx| {
                        buf.undo_transaction(*buffer_transaction_id, cx)
                    });
                }
            }

            if undone {
                return Some(transaction.id);
            }
        }

        None
    }

    pub fn redo(&mut self, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        if let Some(buffer) = self.as_singleton() {
            return buffer.update(cx, |buffer, cx| buffer.redo(cx));
        }

        while let Some(transaction) = self.history.pop_redo() {
            let mut redone = false;
            for (buffer_id, buffer_transaction_id) in &transaction.buffer_transactions {
                if let Some(BufferState { buffer, .. }) = self.buffers.get(&buffer_id) {
                    redone |= buffer.update(cx, |buf, cx| {
                        buf.redo_transaction(*buffer_transaction_id, cx)
                    });
                }
            }

            if redone {
                return Some(transaction.id);
            }
        }

        None
    }

    pub fn push_excerpt<O>(
        &mut self,
        props: ExcerptProperties<O>,
        cx: &mut ModelContext<Self>,
    ) -> ExcerptId
    where
        O: text::ToOffset,
    {
        assert_eq!(self.history.transaction_depth, 0);
        self.sync(cx);

        let buffer = props.buffer.clone();
        cx.subscribe(&buffer, Self::on_buffer_event).detach();

        let buffer_snapshot = buffer.read(cx).snapshot();
        let range = buffer_snapshot.anchor_before(&props.range.start)
            ..buffer_snapshot.anchor_after(&props.range.end);
        let last_version = buffer_snapshot.version().clone();
        let last_parse_count = buffer_snapshot.parse_count();
        let last_diagnostics_update_count = buffer_snapshot.diagnostics_update_count();

        let mut snapshot = self.snapshot.borrow_mut();
        let prev_id = snapshot.excerpts.last().map(|e| &e.id);
        let id = ExcerptId::between(prev_id.unwrap_or(&ExcerptId::min()), &ExcerptId::max());

        let edit_start = snapshot.excerpts.summary().text.bytes;
        let excerpt = Excerpt::new(
            id.clone(),
            buffer.id(),
            buffer_snapshot,
            range,
            props.header_height,
            props.render_header,
            !self.singleton,
        );
        let edit = Edit {
            old: edit_start..edit_start,
            new: edit_start..edit_start + excerpt.text_summary.bytes,
        };
        snapshot.excerpts.push(excerpt, &());
        self.buffers
            .entry(props.buffer.id())
            .or_insert_with(|| BufferState {
                buffer,
                last_version,
                last_parse_count,
                last_diagnostics_update_count,
                excerpts: Default::default(),
            })
            .excerpts
            .push(id.clone());

        self.subscriptions.publish_mut([edit]);

        cx.notify();

        id
    }

    fn on_buffer_event(
        &mut self,
        _: ModelHandle<Buffer>,
        event: &Event,
        cx: &mut ModelContext<Self>,
    ) {
        cx.emit(event.clone());
    }

    pub fn save(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> Result<Task<Result<(clock::Global, SystemTime)>>> {
        self.as_singleton()
            .unwrap()
            .update(cx, |buffer, cx| buffer.save(cx))
    }

    pub fn language<'a>(&self, cx: &'a AppContext) -> Option<&'a Arc<Language>> {
        self.buffers
            .values()
            .next()
            .and_then(|state| state.buffer.read(cx).language())
    }

    pub fn file<'a>(&self, cx: &'a AppContext) -> Option<&'a dyn File> {
        self.as_singleton().unwrap().read(cx).file()
    }

    pub fn is_dirty(&self, cx: &AppContext) -> bool {
        self.as_singleton().unwrap().read(cx).is_dirty()
    }

    pub fn has_conflict(&self, cx: &AppContext) -> bool {
        self.as_singleton().unwrap().read(cx).has_conflict()
    }

    pub fn is_parsing(&self, cx: &AppContext) -> bool {
        self.as_singleton().unwrap().read(cx).is_parsing()
    }

    fn sync(&self, cx: &AppContext) {
        let mut snapshot = self.snapshot.borrow_mut();
        let mut excerpts_to_edit = Vec::new();
        let mut reparsed = false;
        let mut diagnostics_updated = false;
        for buffer_state in self.buffers.values() {
            let buffer = buffer_state.buffer.read(cx);
            let buffer_edited = buffer.version().gt(&buffer_state.last_version);
            let buffer_reparsed = buffer.parse_count() > buffer_state.last_parse_count;
            let buffer_diagnostics_updated =
                buffer.diagnostics_update_count() > buffer_state.last_diagnostics_update_count;
            if buffer_edited || buffer_reparsed || buffer_diagnostics_updated {
                excerpts_to_edit.extend(
                    buffer_state
                        .excerpts
                        .iter()
                        .map(|excerpt_id| (excerpt_id, buffer_state, buffer_edited)),
                );
            }

            reparsed |= buffer_reparsed;
            diagnostics_updated |= buffer_diagnostics_updated;
        }
        if reparsed {
            snapshot.parse_count += 1;
        }
        if diagnostics_updated {
            snapshot.diagnostics_update_count += 1;
        }
        excerpts_to_edit.sort_unstable_by_key(|(excerpt_id, _, _)| *excerpt_id);

        let mut edits = Vec::new();
        let mut new_excerpts = SumTree::new();
        let mut cursor = snapshot.excerpts.cursor::<(Option<&ExcerptId>, usize)>();

        for (id, buffer_state, buffer_edited) in excerpts_to_edit {
            new_excerpts.push_tree(cursor.slice(&Some(id), Bias::Left, &()), &());
            let old_excerpt = cursor.item().unwrap();
            let buffer = buffer_state.buffer.read(cx);

            let mut new_excerpt;
            if buffer_edited {
                edits.extend(
                    buffer
                        .edits_since_in_range::<usize>(
                            old_excerpt.buffer.version(),
                            old_excerpt.range.clone(),
                        )
                        .map(|mut edit| {
                            let excerpt_old_start =
                                cursor.start().1 + old_excerpt.header_height as usize;
                            let excerpt_new_start = new_excerpts.summary().text.bytes
                                + old_excerpt.header_height as usize;
                            edit.old.start += excerpt_old_start;
                            edit.old.end += excerpt_old_start;
                            edit.new.start += excerpt_new_start;
                            edit.new.end += excerpt_new_start;
                            edit
                        }),
                );

                new_excerpt = Excerpt::new(
                    id.clone(),
                    buffer_state.buffer.id(),
                    buffer.snapshot(),
                    old_excerpt.range.clone(),
                    old_excerpt.header_height,
                    old_excerpt.render_header.clone(),
                    !self.singleton,
                );
            } else {
                new_excerpt = old_excerpt.clone();
                new_excerpt.buffer = buffer.snapshot();
            }

            new_excerpts.push(new_excerpt, &());
            cursor.next(&());
        }
        new_excerpts.push_tree(cursor.suffix(&()), &());

        drop(cursor);
        snapshot.excerpts = new_excerpts;

        self.subscriptions.publish(edits);
    }
}

#[cfg(any(test, feature = "test-support"))]
impl MultiBuffer {
    pub fn randomly_edit<R: rand::Rng>(
        &mut self,
        rng: &mut R,
        count: usize,
        cx: &mut ModelContext<Self>,
    ) {
        self.as_singleton()
            .unwrap()
            .update(cx, |buffer, cx| buffer.randomly_edit(rng, count, cx));
        self.sync(cx);
    }
}

impl Entity for MultiBuffer {
    type Event = language::Event;
}

impl MultiBufferSnapshot {
    pub fn text(&self) -> String {
        self.chunks(0..self.len(), None)
            .map(|chunk| chunk.text)
            .collect()
    }

    pub fn excerpt_headers_in_range<'a>(
        &'a self,
        range: Range<u32>,
    ) -> impl 'a + Iterator<Item = (Range<u32>, RenderHeaderFn)> {
        let mut cursor = self.excerpts.cursor::<Point>();
        cursor.seek(&Point::new(range.start, 0), Bias::Right, &());

        if let Some(excerpt) = cursor.item() {
            if range.start >= cursor.start().row + excerpt.header_height as u32 {
                cursor.next(&());
            }
        }

        iter::from_fn(move || {
            while let Some(excerpt) = cursor.item() {
                if cursor.start().row >= range.end {
                    break;
                }

                if let Some(render) = excerpt.render_header.clone() {
                    let start = cursor.start().row;
                    let end = start + excerpt.header_height as u32;
                    cursor.next(&());
                    return Some((start..end, render));
                } else {
                    cursor.next(&());
                }
            }
            None
        })
    }

    pub fn reversed_chars_at<'a, T: ToOffset>(
        &'a self,
        position: T,
    ) -> impl Iterator<Item = char> + 'a {
        // TODO
        let offset = position.to_offset(self);
        self.as_singleton().unwrap().reversed_chars_at(offset)
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

    pub fn contains_str_at<T>(&self, position: T, needle: &str) -> bool
    where
        T: ToOffset,
    {
        let position = position.to_offset(self);
        position == self.clip_offset(position, Bias::Left)
            && self
                .bytes_in_range(position..self.len())
                .flatten()
                .copied()
                .take(needle.len())
                .eq(needle.bytes())
    }

    fn as_singleton(&self) -> Option<&BufferSnapshot> {
        let mut excerpts = self.excerpts.iter();
        let buffer = excerpts.next().map(|excerpt| &excerpt.buffer);
        if excerpts.next().is_none() {
            buffer
        } else {
            None
        }
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
        MultiBufferBytes {
            chunks: self.chunks(range, None).peekable(),
        }
    }

    pub fn chunks<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        theme: Option<&'a SyntaxTheme>,
    ) -> MultiBufferChunks<'a> {
        let mut result = MultiBufferChunks {
            range: 0..range.end.to_offset(self),
            cursor: self.excerpts.cursor::<usize>(),
            header_height: 0,
            excerpt_chunks: None,
            has_trailing_newline: false,
            theme,
        };
        result.seek(range.start.to_offset(self));
        result
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
        if let Some((buffer, range)) = self.buffer_line_for_row(row) {
            buffer
                .indent_column_for_line(range.start.row)
                .min(range.end.column)
                .saturating_sub(range.start.column)
        } else {
            0
        }
    }

    pub fn line_len(&self, row: u32) -> u32 {
        if let Some((_, range)) = self.buffer_line_for_row(row) {
            range.end.column - range.start.column
        } else {
            0
        }
    }

    fn buffer_line_for_row(&self, row: u32) -> Option<(&BufferSnapshot, Range<Point>)> {
        let mut cursor = self.excerpts.cursor::<Point>();
        cursor.seek(&Point::new(row, 0), Bias::Right, &());
        if let Some(excerpt) = cursor.item() {
            let overshoot = row - cursor.start().row;
            let header_height = excerpt.header_height as u32;
            if overshoot >= header_height {
                let excerpt_start = excerpt.range.start.to_point(&excerpt.buffer);
                let excerpt_end = excerpt.range.end.to_point(&excerpt.buffer);
                let buffer_row = excerpt_start.row + overshoot - header_height;
                let line_start = Point::new(buffer_row, 0);
                let line_end = Point::new(buffer_row, excerpt.buffer.line_len(buffer_row));
                return Some((
                    &excerpt.buffer,
                    line_start.max(excerpt_start)..line_end.min(excerpt_end),
                ));
            }
        }
        None
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

            let mut end_before_newline = cursor.end(&());
            if excerpt.has_trailing_newline {
                end_before_newline -= 1;
            }

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

    pub fn summary_for_anchor<D>(&self, anchor: &Anchor) -> D
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>();
        cursor.seek(&Some(&anchor.excerpt_id), Bias::Left, &());
        if let Some(excerpt) = cursor.item() {
            if excerpt.id == anchor.excerpt_id {
                let mut excerpt_start = D::from_text_summary(&cursor.start().text);
                excerpt_start.add_summary(&excerpt.header_summary(), &());
                let excerpt_buffer_start = excerpt.range.start.summary::<D>(&excerpt.buffer);
                let buffer_point = anchor.text_anchor.summary::<D>(&excerpt.buffer);
                if buffer_point > excerpt_buffer_start {
                    excerpt_start.add_assign(&(buffer_point - excerpt_buffer_start));
                }
                return excerpt_start;
            }
        }
        D::from_text_summary(&cursor.start().text)
    }

    pub fn summaries_for_anchors<'a, D, I>(&'a self, anchors: I) -> Vec<D>
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
        I: 'a + IntoIterator<Item = &'a Anchor>,
    {
        let mut anchors = anchors.into_iter().peekable();
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>();
        let mut summaries = Vec::new();
        while let Some(anchor) = anchors.peek() {
            let excerpt_id = &anchor.excerpt_id;
            let excerpt_anchors = iter::from_fn(|| {
                let anchor = anchors.peek()?;
                if anchor.excerpt_id == *excerpt_id {
                    Some(&anchors.next().unwrap().text_anchor)
                } else {
                    None
                }
            });

            cursor.seek_forward(&Some(excerpt_id), Bias::Left, &());
            if let Some(excerpt) = cursor.item() {
                if excerpt.id == *excerpt_id {
                    let mut excerpt_start = D::from_text_summary(&cursor.start().text);
                    excerpt_start.add_summary(&excerpt.header_summary(), &());
                    let excerpt_buffer_start = excerpt.range.start.summary::<D>(&excerpt.buffer);
                    summaries.extend(
                        excerpt
                            .buffer
                            .summaries_for_anchors::<D, _>(excerpt_anchors)
                            .map(move |summary| {
                                let mut excerpt_start = excerpt_start.clone();
                                let excerpt_buffer_start = excerpt_buffer_start.clone();
                                if summary > excerpt_buffer_start {
                                    excerpt_start.add_assign(&(summary - excerpt_buffer_start));
                                }
                                excerpt_start
                            }),
                    );
                    continue;
                }
            }

            let summary = D::from_text_summary(&cursor.start().text);
            summaries.extend(excerpt_anchors.map(|_| summary.clone()));
        }

        summaries
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        let offset = position.to_offset(self);
        let mut cursor = self.excerpts.cursor::<(usize, Option<&ExcerptId>)>();
        cursor.seek(&offset, Bias::Right, &());
        if cursor.item().is_none() && offset == cursor.start().0 && bias == Bias::Left {
            cursor.prev(&());
        }
        if let Some(excerpt) = cursor.item() {
            let start_after_header = cursor.start().0 + excerpt.header_height as usize;
            let buffer_start = excerpt.range.start.to_offset(&excerpt.buffer);
            let overshoot = offset.saturating_sub(start_after_header);
            Anchor {
                excerpt_id: excerpt.id.clone(),
                text_anchor: excerpt.buffer.anchor_at(buffer_start + overshoot, bias),
            }
        } else if offset == 0 && bias == Bias::Left {
            Anchor::min()
        } else {
            Anchor::max()
        }
    }

    pub fn parse_count(&self) -> usize {
        self.parse_count
    }

    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(Range<usize>, Range<usize>)> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.as_singleton().unwrap().enclosing_bracket_ranges(range)
    }

    pub fn diagnostics_update_count(&self) -> usize {
        self.diagnostics_update_count
    }

    pub fn language(&self) -> Option<&Arc<Language>> {
        self.excerpts
            .iter()
            .next()
            .and_then(|excerpt| excerpt.buffer.language())
    }

    pub fn diagnostic_group<'a, O>(
        &'a self,
        group_id: usize,
    ) -> impl Iterator<Item = DiagnosticEntry<O>> + 'a
    where
        O: text::FromAnchor + 'a,
    {
        self.as_singleton().unwrap().diagnostic_group(group_id)
    }

    pub fn diagnostics_in_range<'a, T, O>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = DiagnosticEntry<O>> + 'a
    where
        T: 'a + ToOffset,
        O: 'a + text::FromAnchor,
    {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.as_singleton().unwrap().diagnostics_in_range(range)
    }

    pub fn range_for_syntax_ancestor<T: ToOffset>(&self, range: Range<T>) -> Option<Range<usize>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.as_singleton()
            .unwrap()
            .range_for_syntax_ancestor(range)
    }

    fn buffer_snapshot_for_excerpt<'a>(
        &'a self,
        excerpt_id: &'a ExcerptId,
    ) -> Option<&'a BufferSnapshot> {
        let mut cursor = self.excerpts.cursor::<Option<&ExcerptId>>();
        cursor.seek(&Some(excerpt_id), Bias::Left, &());
        if let Some(excerpt) = cursor.item() {
            if excerpt.id == *excerpt_id {
                return Some(&excerpt.buffer);
            }
        }
        None
    }

    pub fn remote_selections_in_range<'a>(
        &'a self,
        range: &'a Range<Anchor>,
    ) -> impl 'a + Iterator<Item = (ReplicaId, Selection<Anchor>)> {
        let mut cursor = self.excerpts.cursor::<Option<&ExcerptId>>();
        cursor.seek(&Some(&range.start.excerpt_id), Bias::Left, &());
        cursor
            .take_while(move |excerpt| excerpt.id <= range.end.excerpt_id)
            .flat_map(move |excerpt| {
                let mut query_range = excerpt.range.start.clone()..excerpt.range.end.clone();
                if excerpt.id == range.start.excerpt_id {
                    query_range.start = range.start.text_anchor.clone();
                }
                if excerpt.id == range.end.excerpt_id {
                    query_range.end = range.end.text_anchor.clone();
                }

                excerpt
                    .buffer
                    .remote_selections_in_range(query_range)
                    .flat_map(move |(replica_id, selections)| {
                        selections.map(move |selection| {
                            let mut start = Anchor {
                                excerpt_id: excerpt.id.clone(),
                                text_anchor: selection.start.clone(),
                            };
                            let mut end = Anchor {
                                excerpt_id: excerpt.id.clone(),
                                text_anchor: selection.end.clone(),
                            };
                            if range.start.cmp(&start, self).unwrap().is_gt() {
                                start = range.start.clone();
                            }
                            if range.end.cmp(&end, self).unwrap().is_lt() {
                                end = range.end.clone();
                            }

                            (
                                replica_id,
                                Selection {
                                    id: selection.id,
                                    start,
                                    end,
                                    reversed: selection.reversed,
                                    goal: selection.goal,
                                },
                            )
                        })
                    })
            })
    }
}

impl History {
    fn start_transaction(&mut self, now: Instant) -> Option<TransactionId> {
        self.transaction_depth += 1;
        if self.transaction_depth == 1 {
            let id = post_inc(&mut self.next_transaction_id);
            self.undo_stack.push(Transaction {
                id,
                buffer_transactions: Default::default(),
                first_edit_at: now,
                last_edit_at: now,
            });
            Some(id)
        } else {
            None
        }
    }

    fn end_transaction(
        &mut self,
        now: Instant,
        buffer_transactions: HashSet<(usize, TransactionId)>,
    ) -> bool {
        assert_ne!(self.transaction_depth, 0);
        self.transaction_depth -= 1;
        if self.transaction_depth == 0 {
            if buffer_transactions.is_empty() {
                self.undo_stack.pop();
                false
            } else {
                let transaction = self.undo_stack.last_mut().unwrap();
                transaction.last_edit_at = now;
                transaction.buffer_transactions.extend(buffer_transactions);
                true
            }
        } else {
            false
        }
    }

    fn pop_undo(&mut self) -> Option<&Transaction> {
        assert_eq!(self.transaction_depth, 0);
        if let Some(transaction) = self.undo_stack.pop() {
            self.redo_stack.push(transaction);
            self.redo_stack.last()
        } else {
            None
        }
    }

    fn pop_redo(&mut self) -> Option<&Transaction> {
        assert_eq!(self.transaction_depth, 0);
        if let Some(transaction) = self.redo_stack.pop() {
            self.undo_stack.push(transaction);
            self.undo_stack.last()
        } else {
            None
        }
    }

    fn group(&mut self) -> Option<TransactionId> {
        let mut new_len = self.undo_stack.len();
        let mut transactions = self.undo_stack.iter_mut();

        if let Some(mut transaction) = transactions.next_back() {
            while let Some(prev_transaction) = transactions.next_back() {
                if transaction.first_edit_at - prev_transaction.last_edit_at <= self.group_interval
                {
                    transaction = prev_transaction;
                    new_len -= 1;
                } else {
                    break;
                }
            }
        }

        let (transactions_to_keep, transactions_to_merge) = self.undo_stack.split_at_mut(new_len);
        if let Some(last_transaction) = transactions_to_keep.last_mut() {
            if let Some(transaction) = transactions_to_merge.last() {
                last_transaction.last_edit_at = transaction.last_edit_at;
            }
        }

        self.undo_stack.truncate(new_len);
        self.undo_stack.last().map(|t| t.id)
    }
}

impl Excerpt {
    fn new(
        id: ExcerptId,
        buffer_id: usize,
        buffer: BufferSnapshot,
        range: Range<text::Anchor>,
        header_height: u8,
        render_header: Option<RenderHeaderFn>,
        has_trailing_newline: bool,
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
        if has_trailing_newline {
            text_summary.last_line_chars = 0;
            text_summary.lines.row += 1;
            text_summary.lines.column = 0;
            text_summary.lines_utf16.row += 1;
            text_summary.lines_utf16.column = 0;
            text_summary.bytes += 1;
        }

        Excerpt {
            id,
            buffer_id,
            buffer,
            range,
            text_summary,
            header_height,
            render_header,
            has_trailing_newline,
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

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, ExcerptSummary> for Option<&'a ExcerptId> {
    fn cmp(&self, cursor_location: &ExcerptSummary, _: &()) -> cmp::Ordering {
        Ord::cmp(self, &Some(&cursor_location.excerpt_id))
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

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for Option<&'a ExcerptId> {
    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        *self = Some(&summary.excerpt_id);
    }
}

impl<'a> MultiBufferChunks<'a> {
    pub fn offset(&self) -> usize {
        self.range.start
    }

    pub fn seek(&mut self, offset: usize) {
        self.range.start = offset;
        self.cursor.seek_forward(&offset, Bias::Right, &());
        self.header_height = 0;
        self.excerpt_chunks = None;
        if let Some(excerpt) = self.cursor.item() {
            let buffer_range = excerpt.range.to_offset(&excerpt.buffer);
            self.header_height = excerpt.header_height;
            self.has_trailing_newline = excerpt.has_trailing_newline;

            let buffer_start;
            let start_overshoot = self.range.start - self.cursor.start();
            if start_overshoot < excerpt.header_height as usize {
                self.header_height -= start_overshoot as u8;
                buffer_start = buffer_range.start;
            } else {
                buffer_start =
                    buffer_range.start + start_overshoot - excerpt.header_height as usize;
                self.header_height = 0;
            }

            let buffer_end;
            let end_overshoot = self.range.end - self.cursor.start();
            if end_overshoot < excerpt.header_height as usize {
                self.header_height -= excerpt.header_height - end_overshoot as u8;
                buffer_end = buffer_start;
            } else {
                buffer_end = cmp::min(
                    buffer_range.end,
                    buffer_range.start + end_overshoot - excerpt.header_height as usize,
                );
            }

            self.excerpt_chunks = Some(excerpt.buffer.chunks(buffer_start..buffer_end, self.theme));
        }
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
                self.range.start += self.header_height as usize;
                self.header_height = 0;
                return Some(chunk);
            }

            if let Some(excerpt_chunks) = self.excerpt_chunks.as_mut() {
                if let Some(chunk) = excerpt_chunks.next() {
                    self.range.start += chunk.text.len();
                    return Some(chunk);
                }
                self.excerpt_chunks.take();
                if self.has_trailing_newline && self.cursor.end(&()) <= self.range.end {
                    self.range.start += 1;
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
            self.has_trailing_newline = excerpt.has_trailing_newline;
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
        self.chunks.next().map(|chunk| chunk.text.as_bytes())
    }
}

impl<'a> io::Read for MultiBufferBytes<'a> {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
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
    use gpui::{elements::Empty, Element, MutableAppContext};
    use language::Buffer;
    use rand::prelude::*;
    use std::env;
    use text::{Point, RandomCharIter};
    use util::test::sample_text;

    #[gpui::test]
    fn test_singleton_multibuffer(cx: &mut MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6, 'a'), cx));
        let multibuffer = cx.add_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        assert_eq!(
            multibuffer.read(cx).snapshot(cx).text(),
            buffer.read(cx).text()
        );

        buffer.update(cx, |buffer, cx| buffer.edit([1..3], "XXX", cx));
        assert_eq!(
            multibuffer.read(cx).snapshot(cx).text(),
            buffer.read(cx).text()
        );
    }

    #[gpui::test]
    fn test_excerpt_buffer(cx: &mut MutableAppContext) {
        let buffer_1 = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6, 'a'), cx));
        let buffer_2 = cx.add_model(|cx| Buffer::new(0, sample_text(6, 6, 'g'), cx));
        let multibuffer = cx.add_model(|_| MultiBuffer::new(0));

        let subscription = multibuffer.update(cx, |multibuffer, cx| {
            let subscription = multibuffer.subscribe();
            multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer_1,
                    range: Point::new(1, 2)..Point::new(2, 5),
                    header_height: 2,
                    render_header: Some(Arc::new(|_| Empty::new().named("header 1"))),
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

            multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer_1,
                    range: Point::new(3, 3)..Point::new(4, 4),
                    header_height: 1,
                    render_header: Some(Arc::new(|_| Empty::new().named("header 2"))),
                },
                cx,
            );
            multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer_2,
                    range: Point::new(3, 1)..Point::new(3, 3),
                    header_height: 3,
                    render_header: Some(Arc::new(|_| Empty::new().named("header 3"))),
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
            multibuffer.read(cx).snapshot(cx).text(),
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

        {
            let snapshot = multibuffer.read(cx).read(cx);
            assert_eq!(
                snapshot
                    .excerpt_headers_in_range(0..snapshot.max_point().row + 1)
                    .map(|(rows, render)| (rows, render(cx).name().unwrap().to_string()))
                    .collect::<Vec<_>>(),
                &[
                    (0..2, "header 1".into()),
                    (4..5, "header 2".into()),
                    (7..10, "header 3".into())
                ]
            );

            assert_eq!(
                snapshot
                    .excerpt_headers_in_range(1..5)
                    .map(|(rows, render)| (rows, render(cx).name().unwrap().to_string()))
                    .collect::<Vec<_>>(),
                &[(0..2, "header 1".into()), (4..5, "header 2".into())]
            );

            assert_eq!(
                snapshot
                    .excerpt_headers_in_range(2..8)
                    .map(|(rows, render)| (rows, render(cx).name().unwrap().to_string()))
                    .collect::<Vec<_>>(),
                &[(4..5, "header 2".into()), (7..10, "header 3".into())]
            );
        }

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
            multibuffer.read(cx).snapshot(cx).text(),
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

    #[gpui::test]
    fn test_singleton_multibuffer_anchors(cx: &mut MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "abcd", cx));
        let multibuffer = cx.add_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let old_snapshot = multibuffer.read(cx).snapshot(cx);
        buffer.update(cx, |buffer, cx| {
            buffer.edit([0..0], "X", cx);
            buffer.edit([5..5], "Y", cx);
        });
        let new_snapshot = multibuffer.read(cx).snapshot(cx);

        assert_eq!(old_snapshot.text(), "abcd");
        assert_eq!(new_snapshot.text(), "XabcdY");

        assert_eq!(old_snapshot.anchor_before(0).to_offset(&new_snapshot), 0);
        assert_eq!(old_snapshot.anchor_after(0).to_offset(&new_snapshot), 1);
        assert_eq!(old_snapshot.anchor_before(4).to_offset(&new_snapshot), 5);
        assert_eq!(old_snapshot.anchor_after(4).to_offset(&new_snapshot), 6);
    }

    #[gpui::test]
    fn test_multibuffer_anchors(cx: &mut MutableAppContext) {
        let buffer_1 = cx.add_model(|cx| Buffer::new(0, "abcd", cx));
        let buffer_2 = cx.add_model(|cx| Buffer::new(0, "efghi", cx));
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer_1,
                    range: 0..4,
                    header_height: 1,
                    render_header: None,
                },
                cx,
            );
            multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer_2,
                    range: 0..5,
                    header_height: 1,
                    render_header: None,
                },
                cx,
            );
            multibuffer
        });
        let old_snapshot = multibuffer.read(cx).snapshot(cx);

        buffer_1.update(cx, |buffer, cx| {
            buffer.edit([0..0], "W", cx);
            buffer.edit([5..5], "X", cx);
        });
        buffer_2.update(cx, |buffer, cx| {
            buffer.edit([0..0], "Y", cx);
            buffer.edit([6..0], "Z", cx);
        });
        let new_snapshot = multibuffer.read(cx).snapshot(cx);

        assert_eq!(old_snapshot.text(), "\nabcd\n\nefghi\n");
        assert_eq!(new_snapshot.text(), "\nWabcdX\n\nYefghiZ\n");

        assert_eq!(old_snapshot.anchor_before(0).to_offset(&new_snapshot), 0);
        assert_eq!(old_snapshot.anchor_after(0).to_offset(&new_snapshot), 1);
        assert_eq!(old_snapshot.anchor_before(1).to_offset(&new_snapshot), 0);
        assert_eq!(old_snapshot.anchor_after(1).to_offset(&new_snapshot), 1);
        assert_eq!(old_snapshot.anchor_before(7).to_offset(&new_snapshot), 9);
        assert_eq!(old_snapshot.anchor_after(7).to_offset(&new_snapshot), 10);
    }

    #[gpui::test(iterations = 100)]
    fn test_random_excerpts(cx: &mut MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut buffers: Vec<ModelHandle<Buffer>> = Vec::new();
        let list = cx.add_model(|_| MultiBuffer::new(0));
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
                        list.push_excerpt(
                            ExcerptProperties {
                                buffer: &buffer_handle,
                                range: start_ix..end_ix,
                                header_height,
                                render_header: None,
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

    #[gpui::test]
    fn test_history(cx: &mut MutableAppContext) {
        let buffer_1 = cx.add_model(|cx| Buffer::new(0, "1234", cx));
        let buffer_2 = cx.add_model(|cx| Buffer::new(0, "5678", cx));
        let multibuffer = cx.add_model(|_| MultiBuffer::new(0));
        let group_interval = multibuffer.read(cx).history.group_interval;
        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer_1,
                    range: 0..buffer_1.read(cx).len(),
                    header_height: 0,
                    render_header: None,
                },
                cx,
            );
            multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer_2,
                    range: 0..buffer_2.read(cx).len(),
                    header_height: 0,
                    render_header: None,
                },
                cx,
            );
        });

        let mut now = Instant::now();

        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.start_transaction_at(now, cx);
            multibuffer.edit(
                [
                    Point::new(0, 0)..Point::new(0, 0),
                    Point::new(1, 0)..Point::new(1, 0),
                ],
                "A",
                cx,
            );
            multibuffer.edit(
                [
                    Point::new(0, 1)..Point::new(0, 1),
                    Point::new(1, 1)..Point::new(1, 1),
                ],
                "B",
                cx,
            );
            multibuffer.end_transaction_at(now, cx);
            assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678\n");

            now += 2 * group_interval;
            multibuffer.start_transaction_at(now, cx);
            multibuffer.edit([2..2], "C", cx);
            multibuffer.end_transaction_at(now, cx);
            assert_eq!(multibuffer.read(cx).text(), "ABC1234\nAB5678\n");

            multibuffer.undo(cx);
            assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678\n");

            multibuffer.undo(cx);
            assert_eq!(multibuffer.read(cx).text(), "1234\n5678\n");

            multibuffer.redo(cx);
            assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678\n");

            multibuffer.redo(cx);
            assert_eq!(multibuffer.read(cx).text(), "ABC1234\nAB5678\n");

            buffer_1.update(cx, |buffer_1, cx| buffer_1.undo(cx));
            assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678\n");

            multibuffer.undo(cx);
            assert_eq!(multibuffer.read(cx).text(), "1234\n5678\n");

            multibuffer.redo(cx);
            assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678\n");

            multibuffer.redo(cx);
            assert_eq!(multibuffer.read(cx).text(), "ABC1234\nAB5678\n");

            multibuffer.undo(cx);
            assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678\n");

            buffer_1.update(cx, |buffer_1, cx| buffer_1.redo(cx));
            assert_eq!(multibuffer.read(cx).text(), "ABC1234\nAB5678\n");

            multibuffer.undo(cx);
            assert_eq!(multibuffer.read(cx).text(), "C1234\n5678\n");
        });
    }
}
