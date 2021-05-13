mod anchor;
mod point;
mod selection;

pub use anchor::*;
pub use point::*;
use ropey::{Rope, RopeSlice};
use seahash::SeaHasher;
pub use selection::*;
use similar::{ChangeTag, TextDiff};

use crate::{
    operation_queue::{self, OperationQueue},
    sum_tree::{self, FilterCursor, SeekBias, SumTree},
    time::{self, ReplicaId},
    util::RandomCharIter,
    worktree::FileHandle,
};
use anyhow::{anyhow, Result};
use gpui::{AppContext, Entity, ModelContext, Task};
use lazy_static::lazy_static;
use rand::prelude::*;
use std::{
    cmp,
    hash::BuildHasher,
    iter::{self, Iterator},
    ops::Range,
    str,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const UNDO_GROUP_INTERVAL: Duration = Duration::from_millis(300);

#[derive(Clone, Default)]
struct DeterministicState;

impl BuildHasher for DeterministicState {
    type Hasher = SeaHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SeaHasher::new()
    }
}

#[cfg(test)]
type HashMap<K, V> = std::collections::HashMap<K, V, DeterministicState>;

#[cfg(test)]
type HashSet<T> = std::collections::HashSet<T, DeterministicState>;

#[cfg(not(test))]
type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(test))]
type HashSet<T> = std::collections::HashSet<T>;

pub struct Buffer {
    visible_text: Rope,
    deleted_text: Rope,
    fragments: SumTree<Fragment>,
    insertion_splits: HashMap<time::Local, SumTree<InsertionSplit>>,
    pub version: time::Global,
    saved_version: time::Global,
    saved_mtime: SystemTime,
    last_edit: time::Local,
    undo_map: UndoMap,
    history: History,
    file: Option<FileHandle>,
    selections: HashMap<SelectionSetId, Arc<[Selection]>>,
    pub selections_last_update: SelectionsVersion,
    deferred_ops: OperationQueue<Operation>,
    deferred_replicas: HashSet<ReplicaId>,
    replica_id: ReplicaId,
    local_clock: time::Local,
    lamport_clock: time::Lamport,
}

#[derive(Clone)]
struct Transaction {
    start: time::Global,
    buffer_was_dirty: bool,
    edits: Vec<time::Local>,
    selections_before: Option<(SelectionSetId, Arc<[Selection]>)>,
    selections_after: Option<(SelectionSetId, Arc<[Selection]>)>,
    first_edit_at: Instant,
    last_edit_at: Instant,
}

#[derive(Clone)]
pub struct History {
    // TODO: Turn this into a String or Rope, maybe.
    pub base_text: Arc<str>,
    ops: HashMap<time::Local, EditOperation>,
    undo_stack: Vec<Transaction>,
    redo_stack: Vec<Transaction>,
    transaction_depth: usize,
    group_interval: Duration,
}

impl History {
    pub fn new(base_text: Arc<str>) -> Self {
        Self {
            base_text,
            ops: Default::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            transaction_depth: 0,
            group_interval: UNDO_GROUP_INTERVAL,
        }
    }

    fn push(&mut self, op: EditOperation) {
        self.ops.insert(op.id, op);
    }

    fn start_transaction(
        &mut self,
        start: time::Global,
        buffer_was_dirty: bool,
        selections: Option<(SelectionSetId, Arc<[Selection]>)>,
        now: Instant,
    ) {
        self.transaction_depth += 1;
        if self.transaction_depth == 1 {
            self.undo_stack.push(Transaction {
                start,
                buffer_was_dirty,
                edits: Vec::new(),
                selections_before: selections,
                selections_after: None,
                first_edit_at: now,
                last_edit_at: now,
            });
        }
    }

    fn end_transaction(
        &mut self,
        selections: Option<(SelectionSetId, Arc<[Selection]>)>,
        now: Instant,
    ) -> Option<&Transaction> {
        assert_ne!(self.transaction_depth, 0);
        self.transaction_depth -= 1;
        if self.transaction_depth == 0 {
            let transaction = self.undo_stack.last_mut().unwrap();
            transaction.selections_after = selections;
            transaction.last_edit_at = now;
            Some(transaction)
        } else {
            None
        }
    }

    fn group(&mut self) {
        let mut new_len = self.undo_stack.len();
        let mut transactions = self.undo_stack.iter_mut();

        if let Some(mut transaction) = transactions.next_back() {
            for prev_transaction in transactions.next_back() {
                if transaction.first_edit_at - prev_transaction.last_edit_at <= self.group_interval
                {
                    prev_transaction.edits.append(&mut transaction.edits);
                    prev_transaction.last_edit_at = transaction.last_edit_at;
                    prev_transaction.selections_after = transaction.selections_after.take();
                    transaction = prev_transaction;
                    new_len -= 1;
                } else {
                    break;
                }
            }
        }

        self.undo_stack.truncate(new_len);
    }

    fn push_undo(&mut self, edit_id: time::Local) {
        assert_ne!(self.transaction_depth, 0);
        self.undo_stack.last_mut().unwrap().edits.push(edit_id);
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
}

#[derive(Clone, Default, Debug)]
struct UndoMap(HashMap<time::Local, Vec<UndoOperation>>);

impl UndoMap {
    fn insert(&mut self, undo: UndoOperation) {
        self.0.entry(undo.edit_id).or_default().push(undo);
    }

    fn is_undone(&self, edit_id: time::Local) -> bool {
        self.undo_count(edit_id) % 2 == 1
    }

    fn was_undone(&self, edit_id: time::Local, version: &time::Global) -> bool {
        let undo_count = self
            .0
            .get(&edit_id)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|undo| version.observed(undo.id))
            .map(|undo| undo.count)
            .max()
            .unwrap_or(0);
        undo_count % 2 == 1
    }

    fn undo_count(&self, edit_id: time::Local) -> u32 {
        self.0
            .get(&edit_id)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|undo| undo.count)
            .max()
            .unwrap_or(0)
    }
}

struct Edits<'a, F: Fn(&FragmentSummary) -> bool> {
    cursor: FilterCursor<'a, F, Fragment, usize>,
    undos: &'a UndoMap,
    since: time::Global,
    delta: isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edit {
    pub old_range: Range<usize>,
    pub new_range: Range<usize>,
}

impl Edit {
    pub fn delta(&self) -> isize {
        (self.new_range.end - self.new_range.start) as isize
            - (self.old_range.end - self.old_range.start) as isize
    }

    pub fn old_extent(&self) -> usize {
        self.old_range.end - self.old_range.start
    }
}

struct Diff {
    base_version: time::Global,
    new_text: Arc<str>,
    changes: Vec<(ChangeTag, usize)>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Insertion {
    id: time::Local,
    parent_id: time::Local,
    offset_in_parent: usize,
    lamport_timestamp: time::Lamport,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Fragment {
    id: FragmentId,
    insertion: Arc<Insertion>,
    range_in_insertion: Range<usize>,
    deletions: HashSet<time::Local>,
    max_undos: time::Global,
    visible: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FragmentSummary {
    text: FragmentTextSummary,
    max_fragment_id: FragmentId,
    max_version: time::Global,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct FragmentTextSummary {
    visible: usize,
    deleted: usize,
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FragmentTextSummary {
    fn add_summary(&mut self, summary: &'a FragmentSummary) {
        self.visible += summary.text.visible;
        self.deleted += summary.text.deleted;
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    pub chars: usize,
    pub bytes: usize,
    pub lines: Point,
}

impl<'a> From<RopeSlice<'a>> for TextSummary {
    fn from(slice: RopeSlice<'a>) -> Self {
        let last_row = slice.len_lines() - 1;
        let last_column = slice.line(last_row).len_chars();
        Self {
            chars: slice.len_chars(),
            bytes: slice.len_bytes(),
            lines: Point::new(last_row as u32, last_column as u32),
        }
    }
}

impl<'a> std::ops::AddAssign<&'a Self> for TextSummary {
    fn add_assign(&mut self, other: &'a Self) {
        // let joined_line_len = self.lines.column + other.first_line_len;
        // if joined_line_len > self.rightmost_point.column {
        //     self.rightmost_point = Point::new(self.lines.row, joined_line_len);
        // }
        // if other.rightmost_point.column > self.rightmost_point.column {
        //     self.rightmost_point = self.lines + &other.rightmost_point;
        // }

        // if self.lines.row == 0 {
        //     self.first_line_len += other.first_line_len;
        // }

        self.chars += other.chars;
        self.bytes += other.bytes;
        self.lines += &other.lines;
    }
}

impl std::ops::AddAssign<Self> for TextSummary {
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct InsertionSplit {
    extent: usize,
    fragment_id: FragmentId,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct InsertionSplitSummary {
    extent: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Edit {
        edit: EditOperation,
        lamport_timestamp: time::Lamport,
    },
    Undo {
        undo: UndoOperation,
        lamport_timestamp: time::Lamport,
    },
    UpdateSelections {
        set_id: SelectionSetId,
        selections: Option<Arc<[Selection]>>,
        lamport_timestamp: time::Lamport,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditOperation {
    id: time::Local,
    start_id: time::Local,
    start_offset: usize,
    end_id: time::Local,
    end_offset: usize,
    version_in_range: time::Global,
    new_text: Option<String>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UndoOperation {
    id: time::Local,
    edit_id: time::Local,
    count: u32,
}

impl Buffer {
    pub fn new<T: Into<Arc<str>>>(
        replica_id: ReplicaId,
        base_text: T,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(replica_id, History::new(base_text.into()), None, ctx)
    }

    pub fn from_history(
        replica_id: ReplicaId,
        history: History,
        file: Option<FileHandle>,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(replica_id, history, file, ctx)
    }

    fn build(
        replica_id: ReplicaId,
        history: History,
        file: Option<FileHandle>,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        let saved_mtime;
        if let Some(file) = file.as_ref() {
            saved_mtime = file.mtime();
            file.observe_from_model(ctx, |this, file, ctx| {
                let version = this.version.clone();
                if this.version == this.saved_version {
                    if file.is_deleted() {
                        ctx.emit(Event::Dirtied);
                    } else {
                        ctx.spawn(|handle, mut ctx| async move {
                            let (current_version, history) = handle.read_with(&ctx, |this, ctx| {
                                (this.version.clone(), file.load_history(ctx.as_ref()))
                            });
                            if let (Ok(history), true) = (history.await, current_version == version)
                            {
                                let diff = handle
                                    .read_with(&ctx, |this, ctx| this.diff(history.base_text, ctx))
                                    .await;
                                handle.update(&mut ctx, |this, ctx| {
                                    if let Some(_ops) = this.set_text_via_diff(diff, ctx) {
                                        this.saved_version = this.version.clone();
                                        this.saved_mtime = file.mtime();
                                        ctx.emit(Event::Reloaded);
                                    }
                                });
                            }
                        })
                        .detach();
                    }
                }
                ctx.emit(Event::FileHandleChanged);
            });
        } else {
            saved_mtime = UNIX_EPOCH;
        }

        let mut visible_text = Rope::new();
        let mut insertion_splits = HashMap::default();
        let mut fragments = SumTree::new();

        let base_text = Rope::from(history.base_text.as_ref());
        let base_insertion = Arc::new(Insertion {
            id: time::Local::default(),
            parent_id: time::Local::default(),
            offset_in_parent: 0,
            lamport_timestamp: time::Lamport::default(),
        });

        insertion_splits.insert(
            base_insertion.id,
            SumTree::from_item(
                InsertionSplit {
                    fragment_id: FragmentId::min_value().clone(),
                    extent: 0,
                },
                &(),
            ),
        );
        fragments.push(
            Fragment::new(
                FragmentId::min_value().clone(),
                base_insertion.clone(),
                0..0,
            ),
            &(),
        );

        if base_text.len_chars() > 0 {
            let base_fragment_id =
                FragmentId::between(&FragmentId::min_value(), &FragmentId::max_value());
            let range_in_insertion = 0..base_text.len_chars();

            visible_text = base_text.clone();
            insertion_splits.get_mut(&base_insertion.id).unwrap().push(
                InsertionSplit {
                    fragment_id: base_fragment_id.clone(),
                    extent: range_in_insertion.end,
                },
                &(),
            );
            fragments.push(
                Fragment::new(base_fragment_id, base_insertion, range_in_insertion.clone()),
                &(),
            );
        }

        Self {
            visible_text,
            deleted_text: Rope::new(),
            fragments,
            insertion_splits,
            version: time::Global::new(),
            saved_version: time::Global::new(),
            last_edit: time::Local::default(),
            undo_map: Default::default(),
            history,
            file,
            saved_mtime,
            selections: HashMap::default(),
            selections_last_update: 0,
            deferred_ops: OperationQueue::new(),
            deferred_replicas: HashSet::default(),
            replica_id,
            local_clock: time::Local::new(replica_id),
            lamport_clock: time::Lamport::new(replica_id),
        }
    }

    pub fn snapshot(&self) -> Rope {
        self.visible_text.clone()
    }

    pub fn file(&self) -> Option<&FileHandle> {
        self.file.as_ref()
    }

    pub fn save(
        &mut self,
        new_file: Option<FileHandle>,
        ctx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let snapshot = self.snapshot();
        let version = self.version.clone();
        let file = self.file.clone();

        ctx.spawn(|handle, mut ctx| async move {
            if let Some(file) = new_file.as_ref().or(file.as_ref()) {
                let result = ctx.read(|ctx| file.save(snapshot, ctx.as_ref())).await;
                if result.is_ok() {
                    handle.update(&mut ctx, |me, ctx| me.did_save(version, new_file, ctx));
                }
                result
            } else {
                Ok(())
            }
        })
    }

    fn did_save(
        &mut self,
        version: time::Global,
        file: Option<FileHandle>,
        ctx: &mut ModelContext<Buffer>,
    ) {
        if file.is_some() {
            self.file = file;
        }
        if let Some(file) = &self.file {
            self.saved_mtime = file.mtime();
        }
        self.saved_version = version;
        ctx.emit(Event::Saved);
    }

    fn diff(&self, new_text: Arc<str>, ctx: &AppContext) -> Task<Diff> {
        // TODO: it would be nice to not allocate here.
        let old_text = self.text();
        let base_version = self.version();
        ctx.background_executor().spawn(async move {
            let changes = TextDiff::from_lines(old_text.as_str(), new_text.as_ref())
                .iter_all_changes()
                .map(|c| (c.tag(), c.value().len()))
                .collect::<Vec<_>>();
            Diff {
                base_version,
                new_text,
                changes,
            }
        })
    }

    fn set_text_via_diff(
        &mut self,
        diff: Diff,
        ctx: &mut ModelContext<Self>,
    ) -> Option<Vec<Operation>> {
        if self.version == diff.base_version {
            self.start_transaction(None).unwrap();
            let mut operations = Vec::new();
            let mut offset = 0;
            for (tag, len) in diff.changes {
                let range = offset..(offset + len);
                match tag {
                    ChangeTag::Equal => offset += len,
                    ChangeTag::Delete => operations
                        .extend_from_slice(&self.edit(Some(range), "", Some(ctx)).unwrap()),
                    ChangeTag::Insert => {
                        operations.extend_from_slice(
                            &self
                                .edit(Some(offset..offset), &diff.new_text[range], Some(ctx))
                                .unwrap(),
                        );
                        offset += len;
                    }
                }
            }
            self.end_transaction(None, Some(ctx)).unwrap();
            Some(operations)
        } else {
            None
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.version > self.saved_version || self.file.as_ref().map_or(false, |f| f.is_deleted())
    }

    pub fn has_conflict(&self) -> bool {
        self.version > self.saved_version
            && self
                .file
                .as_ref()
                .map_or(false, |f| f.mtime() > self.saved_mtime)
    }

    pub fn version(&self) -> time::Global {
        self.version.clone()
    }

    pub fn text_summary(&self) -> TextSummary {
        TextSummary::from(self.visible_text.slice(..))
    }

    pub fn text_summary_for_range(&self, range: Range<usize>) -> TextSummary {
        TextSummary::from(self.visible_text.slice(range))
    }

    pub fn len(&self) -> usize {
        self.fragments.extent::<usize>()
    }

    pub fn line_len(&self, row: u32) -> Result<u32> {
        let row_start_offset = Point::new(row, 0).to_offset(self)?;
        let row_end_offset = if row >= self.max_point().row {
            self.len()
        } else {
            Point::new(row + 1, 0).to_offset(self)? - 1
        };

        Ok((row_end_offset - row_start_offset) as u32)
    }

    pub fn rightmost_point(&self) -> Point {
        todo!()
        // self.fragments.summary().text_summary.rightmost_point
    }

    pub fn rightmost_point_in_range(&self, range: Range<usize>) -> Point {
        todo!()
        // let mut summary = TextSummary::default();

        // let mut cursor = self.fragments.cursor::<usize, usize>();
        // cursor.seek(&range.start, SeekBias::Right, &());

        // if let Some(fragment) = cursor.item() {
        //     let summary_start = cmp::max(*cursor.start(), range.start) - cursor.start();
        //     let summary_end = cmp::min(range.end - cursor.start(), fragment.len());
        //     summary += fragment.text.slice(summary_start..summary_end).summary();
        //     cursor.next();
        // }

        // if range.end > *cursor.start() {
        //     summary += cursor.summary::<TextSummary>(&range.end, SeekBias::Right, &());

        //     if let Some(fragment) = cursor.item() {
        //         let summary_start = cmp::max(*cursor.start(), range.start) - cursor.start();
        //         let summary_end = cmp::min(range.end - cursor.start(), fragment.len());
        //         summary += fragment.text.slice(summary_start..summary_end).summary();
        //     }
        // }

        // summary.rightmost_point
    }

    pub fn max_point(&self) -> Point {
        self.text_summary().lines
    }

    pub fn line(&self, row: u32) -> Result<String> {
        Ok(self
            .chars_at(Point::new(row, 0))?
            .take_while(|c| *c != '\n')
            .collect())
    }

    pub fn text(&self) -> String {
        self.chars().collect()
    }

    pub fn text_for_range<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> Result<impl 'a + Iterator<Item = char>> {
        let start = range.start.to_offset(self)?;
        let end = range.end.to_offset(self)?;
        Ok(self.chars_at(start)?.take(end - start))
    }

    pub fn chars(&self) -> ropey::iter::Chars {
        self.chars_at(0).unwrap()
    }

    pub fn chars_at<T: ToOffset>(&self, position: T) -> Result<ropey::iter::Chars> {
        let offset = position.to_offset(self)?;
        Ok(self.visible_text.chars_at(offset))
    }

    pub fn selections_changed_since(&self, since: SelectionsVersion) -> bool {
        self.selections_last_update != since
    }

    pub fn edits_since<'a>(&'a self, since: time::Global) -> impl 'a + Iterator<Item = Edit> {
        let since_2 = since.clone();
        let cursor = self
            .fragments
            .filter(move |summary| summary.max_version.changed_since(&since_2));

        Edits {
            cursor,
            undos: &self.undo_map,
            since,
            delta: 0,
        }
    }

    pub fn deferred_ops_len(&self) -> usize {
        self.deferred_ops.len()
    }

    pub fn start_transaction(&mut self, set_id: Option<SelectionSetId>) -> Result<()> {
        self.start_transaction_at(set_id, Instant::now())
    }

    fn start_transaction_at(&mut self, set_id: Option<SelectionSetId>, now: Instant) -> Result<()> {
        let selections = if let Some(set_id) = set_id {
            let selections = self
                .selections
                .get(&set_id)
                .ok_or_else(|| anyhow!("invalid selection set {:?}", set_id))?;
            Some((set_id, selections.clone()))
        } else {
            None
        };
        self.history
            .start_transaction(self.version.clone(), self.is_dirty(), selections, now);
        Ok(())
    }

    pub fn end_transaction(
        &mut self,
        set_id: Option<SelectionSetId>,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<()> {
        self.end_transaction_at(set_id, Instant::now(), ctx)
    }

    fn end_transaction_at(
        &mut self,
        set_id: Option<SelectionSetId>,
        now: Instant,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<()> {
        let selections = if let Some(set_id) = set_id {
            let selections = self
                .selections
                .get(&set_id)
                .ok_or_else(|| anyhow!("invalid selection set {:?}", set_id))?;
            Some((set_id, selections.clone()))
        } else {
            None
        };

        if let Some(transaction) = self.history.end_transaction(selections, now) {
            let since = transaction.start.clone();
            let was_dirty = transaction.buffer_was_dirty;
            self.history.group();

            if let Some(ctx) = ctx {
                ctx.notify();

                if self.edits_since(since).next().is_some() {
                    self.did_edit(was_dirty, ctx);
                }
            }
        }

        Ok(())
    }

    pub fn edit<I, S, T>(
        &mut self,
        old_ranges: I,
        new_text: T,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<Vec<Operation>>
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        self.start_transaction_at(None, Instant::now())?;

        let new_text = new_text.into();
        let new_text = if new_text.len() > 0 {
            Some(new_text)
        } else {
            None
        };

        let old_ranges = old_ranges
            .into_iter()
            .map(|range| Ok(range.start.to_offset(self)?..range.end.to_offset(self)?))
            .collect::<Result<Vec<Range<usize>>>>()?;

        let has_new_text = new_text.is_some();
        let ops = self.splice_fragments(
            old_ranges
                .into_iter()
                .filter(|old_range| has_new_text || old_range.end > old_range.start),
            new_text.into(),
        );

        for op in &ops {
            if let Operation::Edit { edit, .. } = op {
                self.history.push(edit.clone());
                self.history.push_undo(edit.id);
            }
        }

        if let Some(op) = ops.last() {
            if let Operation::Edit { edit, .. } = op {
                self.last_edit = edit.id;
                self.version.observe(edit.id);
            } else {
                unreachable!()
            }
        }

        self.end_transaction_at(None, Instant::now(), ctx)?;

        Ok(ops)
    }

    fn did_edit(&self, was_dirty: bool, ctx: &mut ModelContext<Self>) {
        ctx.emit(Event::Edited);
        if !was_dirty {
            ctx.emit(Event::Dirtied);
        }
    }

    pub fn simulate_typing<T: Rng>(&mut self, rng: &mut T) {
        let end = rng.gen_range(0..self.len() + 1);
        let start = rng.gen_range(0..end + 1);
        let mut range = start..end;

        let new_text_len = rng.gen_range(0..100);
        let new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();

        for char in new_text.chars() {
            self.edit(Some(range.clone()), char.to_string().as_str(), None)
                .unwrap();
            range = range.end + 1..range.end + 1;
        }
    }

    pub fn randomly_edit<T>(
        &mut self,
        rng: &mut T,
        old_range_count: usize,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> (Vec<Range<usize>>, String, Vec<Operation>)
    where
        T: Rng,
    {
        let mut old_ranges: Vec<Range<usize>> = Vec::new();
        for _ in 0..old_range_count {
            let last_end = old_ranges.last().map_or(0, |last_range| last_range.end + 1);
            if last_end > self.len() {
                break;
            }
            let end = rng.gen_range(last_end..self.len() + 1);
            let start = rng.gen_range(last_end..end + 1);
            old_ranges.push(start..end);
        }
        let new_text_len = rng.gen_range(0..10);
        let new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();

        let operations = self
            .edit(old_ranges.iter().cloned(), new_text.as_str(), ctx)
            .unwrap();

        (old_ranges, new_text, operations)
    }

    pub fn add_selection_set(
        &mut self,
        selections: impl Into<Arc<[Selection]>>,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> (SelectionSetId, Operation) {
        let selections = selections.into();
        let lamport_timestamp = self.lamport_clock.tick();
        self.selections
            .insert(lamport_timestamp, Arc::clone(&selections));
        self.selections_last_update += 1;

        if let Some(ctx) = ctx {
            ctx.notify();
        }

        (
            lamport_timestamp,
            Operation::UpdateSelections {
                set_id: lamport_timestamp,
                selections: Some(selections),
                lamport_timestamp,
            },
        )
    }

    pub fn update_selection_set(
        &mut self,
        set_id: SelectionSetId,
        selections: impl Into<Arc<[Selection]>>,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<Operation> {
        let selections = selections.into();
        self.selections.insert(set_id, selections.clone());

        let lamport_timestamp = self.lamport_clock.tick();
        self.selections_last_update += 1;

        if let Some(ctx) = ctx {
            ctx.notify();
        }

        Ok(Operation::UpdateSelections {
            set_id,
            selections: Some(selections),
            lamport_timestamp,
        })
    }

    pub fn remove_selection_set(
        &mut self,
        set_id: SelectionSetId,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<Operation> {
        self.selections
            .remove(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        let lamport_timestamp = self.lamport_clock.tick();
        self.selections_last_update += 1;

        if let Some(ctx) = ctx {
            ctx.notify();
        }

        Ok(Operation::UpdateSelections {
            set_id,
            selections: None,
            lamport_timestamp,
        })
    }

    pub fn selections(&self, set_id: SelectionSetId) -> Result<&[Selection]> {
        self.selections
            .get(&set_id)
            .map(|s| s.as_ref())
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))
    }

    pub fn apply_ops<I: IntoIterator<Item = Operation>>(
        &mut self,
        ops: I,
        ctx: Option<&mut ModelContext<Self>>,
    ) -> Result<()> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let mut deferred_ops = Vec::new();
        for op in ops {
            if self.can_apply_op(&op) {
                self.apply_op(op)?;
            } else {
                self.deferred_replicas.insert(op.replica_id());
                deferred_ops.push(op);
            }
        }
        self.deferred_ops.insert(deferred_ops);
        self.flush_deferred_ops()?;

        if let Some(ctx) = ctx {
            ctx.notify();
            if self.edits_since(old_version).next().is_some() {
                self.did_edit(was_dirty, ctx);
            }
        }

        Ok(())
    }

    fn apply_op(&mut self, op: Operation) -> Result<()> {
        match op {
            Operation::Edit {
                edit,
                lamport_timestamp,
                ..
            } => {
                if !self.version.observed(edit.id) {
                    self.apply_edit(
                        edit.start_id,
                        edit.start_offset,
                        edit.end_id,
                        edit.end_offset,
                        edit.new_text.as_deref(),
                        &edit.version_in_range,
                        edit.id,
                        lamport_timestamp,
                    )?;
                    self.version.observe(edit.id);
                    self.history.push(edit);
                }
            }
            Operation::Undo {
                undo,
                lamport_timestamp,
            } => {
                if !self.version.observed(undo.id) {
                    self.apply_undo(undo)?;
                    self.version.observe(undo.id);
                    self.lamport_clock.observe(lamport_timestamp);
                }
            }
            Operation::UpdateSelections {
                set_id,
                selections,
                lamport_timestamp,
            } => {
                if let Some(selections) = selections {
                    self.selections.insert(set_id, selections);
                } else {
                    self.selections.remove(&set_id);
                }
                self.lamport_clock.observe(lamport_timestamp);
                self.selections_last_update += 1;
            }
        }
        Ok(())
    }

    fn apply_edit(
        &mut self,
        start_id: time::Local,
        start_offset: usize,
        end_id: time::Local,
        end_offset: usize,
        mut new_text: Option<&str>,
        version_in_range: &time::Global,
        local_timestamp: time::Local,
        lamport_timestamp: time::Lamport,
    ) -> Result<()> {
        let start_fragment_id = self.resolve_fragment_id(start_id, start_offset)?;
        let end_fragment_id = self.resolve_fragment_id(end_id, end_offset)?;

        let old_fragments = self.fragments.clone();
        let last_id = old_fragments.extent::<FragmentIdRef>().0.unwrap();
        let last_id_ref = FragmentIdRef::new(&last_id);

        let mut cursor = old_fragments.cursor::<FragmentIdRef, FragmentTextSummary>();
        let mut new_fragments =
            cursor.slice(&FragmentIdRef::new(&start_fragment_id), SeekBias::Left, &());
        let mut new_visible_text = self.visible_text.clone();
        let mut new_deleted_text = self.deleted_text.clone();

        let start_fragment = cursor.item().unwrap();
        if start_offset == start_fragment.range_in_insertion.end {
            // TODO: maybe don't recompute this fragment and its summary.
            new_fragments.push(cursor.item().unwrap().clone(), &());
            cursor.next();
        }

        while let Some(fragment) = cursor.item() {
            if new_text.is_none() && fragment.id > end_fragment_id {
                break;
            }

            let mut fragment = fragment.clone();

            if fragment.id == start_fragment_id || fragment.id == end_fragment_id {
                let split_start = if start_fragment_id == fragment.id {
                    start_offset
                } else {
                    fragment.range_in_insertion.start
                };
                let split_end = if end_fragment_id == fragment.id {
                    end_offset
                } else {
                    fragment.range_in_insertion.end
                };
                let (before_range, within_range, after_range) = self.split_fragment(
                    cursor.prev_item().as_ref().unwrap(),
                    &fragment,
                    split_start..split_end,
                );
                let insertion = if let Some(new_text) = new_text {
                    Some(self.build_fragment_to_insert(
                        before_range.as_ref().or(cursor.prev_item()).unwrap(),
                        within_range.as_ref().or(after_range.as_ref()),
                        new_text,
                        local_timestamp,
                        lamport_timestamp,
                    ))
                } else {
                    None
                };
                if let Some(fragment) = before_range {
                    new_fragments.push(fragment, &());
                }
                if let Some(fragment) = insertion {
                    new_visible_text.insert(
                        new_fragments.summary().text.visible,
                        new_text.take().unwrap(),
                    );
                    new_fragments.push(fragment, &());
                }
                if let Some(mut fragment) = within_range {
                    if fragment.was_visible(&version_in_range, &self.undo_map) {
                        fragment.deletions.insert(local_timestamp);
                        fragment.visible = false;

                        // TODO: avoid calling to_string on rope slice.
                        let deleted_start = new_fragments.summary().text.visible;
                        let deleted_range = deleted_start..deleted_start + fragment.len();
                        new_deleted_text.insert(
                            new_fragments.summary().text.deleted,
                            &new_visible_text.slice(deleted_range.clone()).to_string(),
                        );
                        new_visible_text.remove(deleted_range);
                    }

                    new_fragments.push(fragment, &());
                }
                if let Some(fragment) = after_range {
                    new_fragments.push(fragment, &());
                }
            } else {
                if new_text.is_some() && lamport_timestamp > fragment.insertion.lamport_timestamp {
                    let new_text = new_text.take().unwrap();
                    let fragment = self.build_fragment_to_insert(
                        cursor.prev_item().as_ref().unwrap(),
                        Some(&fragment),
                        new_text,
                        local_timestamp,
                        lamport_timestamp,
                    );
                    new_visible_text.insert(new_fragments.summary().text.visible, new_text);
                    new_fragments.push(fragment, &());
                }

                if fragment.id < end_fragment_id
                    && fragment.was_visible(&version_in_range, &self.undo_map)
                {
                    fragment.deletions.insert(local_timestamp);
                    fragment.visible = false;

                    // TODO: avoid calling to_string on rope slice.
                    let deleted_start = new_fragments.summary().text.visible;
                    let deleted_range = deleted_start..deleted_start + fragment.len();
                    new_deleted_text.insert(
                        new_fragments.summary().text.deleted,
                        &new_visible_text.slice(deleted_range.clone()).to_string(),
                    );
                    new_visible_text.remove(deleted_range);
                }

                new_fragments.push(fragment, &());
            }

            cursor.next();
        }

        if let Some(new_text) = new_text {
            let fragment = self.build_fragment_to_insert(
                cursor.prev_item().as_ref().unwrap(),
                None,
                new_text,
                local_timestamp,
                lamport_timestamp,
            );
            new_visible_text.insert(new_fragments.summary().text.visible, new_text);
            new_fragments.push(fragment, &());
        }

        new_fragments.push_tree(cursor.slice(&last_id_ref, SeekBias::Right, &()), &());
        self.visible_text = new_visible_text;
        self.deleted_text = new_deleted_text;
        self.fragments = new_fragments;
        self.local_clock.observe(local_timestamp);
        self.lamport_clock.observe(lamport_timestamp);
        Ok(())
    }

    pub fn undo(&mut self, mut ctx: Option<&mut ModelContext<Self>>) -> Vec<Operation> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let mut ops = Vec::new();
        if let Some(transaction) = self.history.pop_undo() {
            let selections = transaction.selections_before.clone();
            for edit_id in transaction.edits.clone() {
                ops.push(self.undo_or_redo(edit_id).unwrap());
            }

            if let Some((set_id, selections)) = selections {
                let _ = self.update_selection_set(set_id, selections, ctx.as_deref_mut());
            }
        }

        if let Some(ctx) = ctx {
            ctx.notify();
            if self.edits_since(old_version).next().is_some() {
                self.did_edit(was_dirty, ctx);
            }
        }

        ops
    }

    pub fn redo(&mut self, mut ctx: Option<&mut ModelContext<Self>>) -> Vec<Operation> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let mut ops = Vec::new();
        if let Some(transaction) = self.history.pop_redo() {
            let selections = transaction.selections_after.clone();
            for edit_id in transaction.edits.clone() {
                ops.push(self.undo_or_redo(edit_id).unwrap());
            }

            if let Some((set_id, selections)) = selections {
                let _ = self.update_selection_set(set_id, selections, ctx.as_deref_mut());
            }
        }

        if let Some(ctx) = ctx {
            ctx.notify();
            if self.edits_since(old_version).next().is_some() {
                self.did_edit(was_dirty, ctx);
            }
        }

        ops
    }

    fn undo_or_redo(&mut self, edit_id: time::Local) -> Result<Operation> {
        let undo = UndoOperation {
            id: self.local_clock.tick(),
            edit_id,
            count: self.undo_map.undo_count(edit_id) + 1,
        };
        self.apply_undo(undo)?;
        self.version.observe(undo.id);

        Ok(Operation::Undo {
            undo,
            lamport_timestamp: self.lamport_clock.tick(),
        })
    }

    fn apply_undo(&mut self, undo: UndoOperation) -> Result<()> {
        let mut new_fragments;
        let mut new_visible_text = self.visible_text.clone();
        let mut new_deleted_text = self.deleted_text.clone();

        self.undo_map.insert(undo);
        let edit = &self.history.ops[&undo.edit_id];
        let start_fragment_id = self.resolve_fragment_id(edit.start_id, edit.start_offset)?;
        let end_fragment_id = self.resolve_fragment_id(edit.end_id, edit.end_offset)?;
        let mut cursor = self.fragments.cursor::<FragmentIdRef, ()>();

        if edit.start_id == edit.end_id && edit.start_offset == edit.end_offset {
            let splits = &self.insertion_splits[&undo.edit_id];
            let mut insertion_splits = splits.cursor::<(), ()>().map(|s| &s.fragment_id).peekable();

            let first_split_id = insertion_splits.next().unwrap();
            new_fragments = cursor.slice(&FragmentIdRef::new(first_split_id), SeekBias::Left, &());

            loop {
                let mut fragment = cursor.item().unwrap().clone();
                let was_visible = fragment.visible;
                fragment.visible = fragment.is_visible(&self.undo_map);
                fragment.max_undos.observe(undo.id);

                // TODO: avoid calling to_string on rope slice.
                if fragment.visible && !was_visible {
                    let visible_start = new_fragments.summary().text.deleted;
                    let visible_range = visible_start..visible_start + fragment.len();
                    new_visible_text.insert(
                        new_fragments.summary().text.visible,
                        &new_deleted_text.slice(visible_range.clone()).to_string(),
                    );
                    new_deleted_text.remove(visible_range);
                } else if !fragment.visible && was_visible {
                    let deleted_start = new_fragments.summary().text.visible;
                    let deleted_range = deleted_start..deleted_start + fragment.len();
                    new_deleted_text.insert(
                        new_fragments.summary().text.deleted,
                        &new_visible_text.slice(deleted_range.clone()).to_string(),
                    );
                    new_visible_text.remove(deleted_range);
                }

                new_fragments.push(fragment, &());
                cursor.next();
                if let Some(split_id) = insertion_splits.next() {
                    new_fragments.push_tree(
                        cursor.slice(&FragmentIdRef::new(split_id), SeekBias::Left, &()),
                        &(),
                    );
                } else {
                    break;
                }
            }
        } else {
            new_fragments =
                cursor.slice(&FragmentIdRef::new(&start_fragment_id), SeekBias::Left, &());
            while let Some(fragment) = cursor.item() {
                if fragment.id > end_fragment_id {
                    break;
                } else {
                    let mut fragment = fragment.clone();
                    if edit.version_in_range.observed(fragment.insertion.id)
                        || fragment.insertion.id == undo.edit_id
                    {
                        let was_visible = fragment.visible;
                        fragment.visible = fragment.is_visible(&self.undo_map);
                        fragment.max_undos.observe(undo.id);

                        // TODO: avoid calling to_string on rope slice.
                        if fragment.visible && !was_visible {
                            let visible_start = new_fragments.summary().text.deleted;
                            let visible_range = visible_start..visible_start + fragment.len();
                            new_visible_text.insert(
                                new_fragments.summary().text.visible,
                                &new_deleted_text.slice(visible_range.clone()).to_string(),
                            );
                            new_deleted_text.remove(visible_range);
                        } else if !fragment.visible && was_visible {
                            let deleted_start = new_fragments.summary().text.visible;
                            let deleted_range = deleted_start..deleted_start + fragment.len();
                            new_deleted_text.insert(
                                new_fragments.summary().text.deleted,
                                &new_visible_text.slice(deleted_range.clone()).to_string(),
                            );
                            new_visible_text.remove(deleted_range);
                        }
                    }

                    new_fragments.push(fragment, &());
                    cursor.next();
                }
            }
        }

        new_fragments.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        self.visible_text = new_visible_text;
        self.deleted_text = new_deleted_text;
        self.fragments = new_fragments;

        Ok(())
    }

    fn flush_deferred_ops(&mut self) -> Result<()> {
        self.deferred_replicas.clear();
        let mut deferred_ops = Vec::new();
        for op in self.deferred_ops.drain().cursor().cloned() {
            if self.can_apply_op(&op) {
                self.apply_op(op)?;
            } else {
                self.deferred_replicas.insert(op.replica_id());
                deferred_ops.push(op);
            }
        }
        self.deferred_ops.insert(deferred_ops);
        Ok(())
    }

    fn can_apply_op(&self, op: &Operation) -> bool {
        if self.deferred_replicas.contains(&op.replica_id()) {
            false
        } else {
            match op {
                Operation::Edit { edit, .. } => {
                    self.version.observed(edit.start_id)
                        && self.version.observed(edit.end_id)
                        && edit.version_in_range <= self.version
                }
                Operation::Undo { undo, .. } => self.version.observed(undo.edit_id),
                Operation::UpdateSelections { selections, .. } => {
                    if let Some(selections) = selections {
                        selections.iter().all(|selection| {
                            let contains_start = match selection.start {
                                Anchor::Middle { insertion_id, .. } => {
                                    self.version.observed(insertion_id)
                                }
                                _ => true,
                            };
                            let contains_end = match selection.end {
                                Anchor::Middle { insertion_id, .. } => {
                                    self.version.observed(insertion_id)
                                }
                                _ => true,
                            };
                            contains_start && contains_end
                        })
                    } else {
                        true
                    }
                }
            }
        }
    }

    fn resolve_fragment_id(&self, edit_id: time::Local, offset: usize) -> Result<FragmentId> {
        let split_tree = self
            .insertion_splits
            .get(&edit_id)
            .ok_or_else(|| anyhow!("invalid operation"))?;
        let mut cursor = split_tree.cursor::<usize, ()>();
        cursor.seek(&offset, SeekBias::Left, &());
        Ok(cursor
            .item()
            .ok_or_else(|| anyhow!("invalid operation"))?
            .fragment_id
            .clone())
    }

    fn splice_fragments<I>(&mut self, mut old_ranges: I, new_text: Option<String>) -> Vec<Operation>
    where
        I: Iterator<Item = Range<usize>>,
    {
        let mut cur_range = old_ranges.next();
        if cur_range.is_none() {
            return Vec::new();
        }

        let mut ops = Vec::with_capacity(old_ranges.size_hint().0);

        let old_fragments = self.fragments.clone();
        let mut cursor = old_fragments.cursor::<usize, usize>();
        let mut new_fragments = SumTree::new();
        let mut new_visible_text = self.visible_text.clone();
        let mut new_deleted_text = self.deleted_text.clone();

        new_fragments.push_tree(
            cursor.slice(&cur_range.as_ref().unwrap().start, SeekBias::Right, &()),
            &(),
        );

        let mut start_id = None;
        let mut start_offset = None;
        let mut end_id = None;
        let mut end_offset = None;
        let mut version_in_range = time::Global::new();

        let mut local_timestamp = self.local_clock.tick();
        let mut lamport_timestamp = self.lamport_clock.tick();

        while cur_range.is_some() && cursor.item().is_some() {
            let mut fragment = cursor.item().unwrap().clone();
            let fragment_summary = cursor.item_summary().unwrap();
            let mut fragment_start = *cursor.start();
            let mut fragment_end = fragment_start + fragment.visible_len();

            let old_split_tree = self
                .insertion_splits
                .remove(&fragment.insertion.id)
                .unwrap();
            let mut splits_cursor = old_split_tree.cursor::<usize, ()>();
            let mut new_split_tree =
                splits_cursor.slice(&fragment.range_in_insertion.start, SeekBias::Right, &());

            // Find all splices that start or end within the current fragment. Then, split the
            // fragment and reassemble it in both trees accounting for the deleted and the newly
            // inserted text.
            while cur_range.as_ref().map_or(false, |r| r.start < fragment_end) {
                let range = cur_range.clone().unwrap();
                if range.start > fragment_start {
                    let mut prefix = fragment.clone();
                    prefix.range_in_insertion.end =
                        prefix.range_in_insertion.start + (range.start - fragment_start);
                    prefix.id =
                        FragmentId::between(&new_fragments.last().unwrap().id, &fragment.id);
                    fragment.range_in_insertion.start = prefix.range_in_insertion.end;
                    new_fragments.push(prefix.clone(), &());
                    new_split_tree.push(
                        InsertionSplit {
                            extent: prefix.range_in_insertion.end - prefix.range_in_insertion.start,
                            fragment_id: prefix.id,
                        },
                        &(),
                    );
                    fragment_start = range.start;
                }

                if range.end == fragment_start {
                    end_id = Some(new_fragments.last().unwrap().insertion.id);
                    end_offset = Some(new_fragments.last().unwrap().range_in_insertion.end);
                } else if range.end == fragment_end {
                    end_id = Some(fragment.insertion.id);
                    end_offset = Some(fragment.range_in_insertion.end);
                }

                if range.start == fragment_start {
                    start_id = Some(new_fragments.last().unwrap().insertion.id);
                    start_offset = Some(new_fragments.last().unwrap().range_in_insertion.end);

                    if let Some(new_text) = new_text.clone() {
                        let new_fragment = self.build_fragment_to_insert(
                            &new_fragments.last().unwrap(),
                            Some(&fragment),
                            &new_text,
                            local_timestamp,
                            lamport_timestamp,
                        );
                        new_visible_text.insert(new_fragments.summary().text.visible, &new_text);
                        new_fragments.push(new_fragment, &());
                    }
                }

                if range.end < fragment_end {
                    if range.end > fragment_start {
                        let mut prefix = fragment.clone();
                        prefix.range_in_insertion.end =
                            prefix.range_in_insertion.start + (range.end - fragment_start);
                        prefix.id =
                            FragmentId::between(&new_fragments.last().unwrap().id, &fragment.id);
                        version_in_range.observe_all(&fragment_summary.max_version);
                        if fragment.visible {
                            prefix.deletions.insert(local_timestamp);
                            prefix.visible = false;

                            // TODO: avoid calling to_string on rope slice.
                            let deleted_start = new_fragments.summary().text.visible;
                            let deleted_range = deleted_start..deleted_start + prefix.len();
                            new_deleted_text.insert(
                                new_fragments.summary().text.deleted,
                                &new_visible_text.slice(deleted_range.clone()).to_string(),
                            );
                            new_visible_text.remove(deleted_range);
                        }
                        fragment.range_in_insertion.start = prefix.range_in_insertion.end;
                        new_fragments.push(prefix.clone(), &());
                        new_split_tree.push(
                            InsertionSplit {
                                extent: prefix.range_in_insertion.end
                                    - prefix.range_in_insertion.start,
                                fragment_id: prefix.id,
                            },
                            &(),
                        );
                        fragment_start = range.end;
                        end_id = Some(fragment.insertion.id);
                        end_offset = Some(fragment.range_in_insertion.start);
                    }
                } else {
                    version_in_range.observe_all(&fragment_summary.max_version);
                    if fragment.visible {
                        fragment.deletions.insert(local_timestamp);
                        fragment.visible = false;

                        // TODO: avoid calling to_string on rope slice.
                        let deleted_start = new_fragments.summary().text.visible;
                        let deleted_range = deleted_start..deleted_start + fragment.len();
                        new_deleted_text.insert(
                            new_fragments.summary().text.deleted,
                            &new_visible_text.slice(deleted_range.clone()).to_string(),
                        );
                        new_visible_text.remove(deleted_range);
                    }
                }

                // If the splice ends inside this fragment, we can advance to the next splice and
                // check if it also intersects the current fragment. Otherwise we break out of the
                // loop and find the first fragment that the splice does not contain fully.
                if range.end <= fragment_end {
                    ops.push(Operation::Edit {
                        edit: EditOperation {
                            id: local_timestamp,
                            start_id: start_id.unwrap(),
                            start_offset: start_offset.unwrap(),
                            end_id: end_id.unwrap(),
                            end_offset: end_offset.unwrap(),
                            version_in_range,
                            new_text: new_text.clone(),
                        },
                        lamport_timestamp,
                    });

                    start_id = None;
                    start_offset = None;
                    end_id = None;
                    end_offset = None;
                    version_in_range = time::Global::new();
                    cur_range = old_ranges.next();
                    if cur_range.is_some() {
                        local_timestamp = self.local_clock.tick();
                        lamport_timestamp = self.lamport_clock.tick();
                    }
                } else {
                    break;
                }
            }
            new_split_tree.push(
                InsertionSplit {
                    extent: fragment.range_in_insertion.end - fragment.range_in_insertion.start,
                    fragment_id: fragment.id.clone(),
                },
                &(),
            );
            splits_cursor.next();
            new_split_tree.push_tree(
                splits_cursor.slice(&old_split_tree.extent::<usize>(), SeekBias::Right, &()),
                &(),
            );
            self.insertion_splits
                .insert(fragment.insertion.id, new_split_tree);
            new_fragments.push(fragment, &());

            // Scan forward until we find a fragment that is not fully contained by the current splice.
            cursor.next();
            if let Some(range) = cur_range.clone() {
                while let Some(fragment) = cursor.item() {
                    let fragment_summary = cursor.item_summary().unwrap();
                    fragment_start = *cursor.start();
                    fragment_end = fragment_start + fragment.visible_len();
                    if range.start < fragment_start && range.end >= fragment_end {
                        let mut new_fragment = fragment.clone();
                        version_in_range.observe_all(&fragment_summary.max_version);
                        if new_fragment.visible {
                            new_fragment.deletions.insert(local_timestamp);
                            new_fragment.visible = false;

                            // TODO: avoid calling to_string on rope slice.
                            let deleted_start = new_fragments.summary().text.visible;
                            let deleted_range = deleted_start..deleted_start + new_fragment.len();
                            new_deleted_text.insert(
                                new_fragments.summary().text.deleted,
                                &new_visible_text.slice(deleted_range.clone()).to_string(),
                            );
                            new_visible_text.remove(deleted_range);
                        }
                        new_fragments.push(new_fragment, &());
                        cursor.next();

                        if range.end == fragment_end {
                            end_id = Some(fragment.insertion.id);
                            end_offset = Some(fragment.range_in_insertion.end);
                            ops.push(Operation::Edit {
                                edit: EditOperation {
                                    id: local_timestamp,
                                    start_id: start_id.unwrap(),
                                    start_offset: start_offset.unwrap(),
                                    end_id: end_id.unwrap(),
                                    end_offset: end_offset.unwrap(),
                                    version_in_range,
                                    new_text: new_text.clone(),
                                },
                                lamport_timestamp,
                            });

                            start_id = None;
                            start_offset = None;
                            end_id = None;
                            end_offset = None;
                            version_in_range = time::Global::new();

                            cur_range = old_ranges.next();
                            if cur_range.is_some() {
                                local_timestamp = self.local_clock.tick();
                                lamport_timestamp = self.lamport_clock.tick();
                            }
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // If the splice we are currently evaluating starts after the end of the fragment
                // that the cursor is parked at, we should seek to the next splice's start range
                // and push all the fragments in between into the new tree.
                if cur_range.as_ref().map_or(false, |r| r.start > fragment_end) {
                    new_fragments.push_tree(
                        cursor.slice(&cur_range.as_ref().unwrap().start, SeekBias::Right, &()),
                        &(),
                    );
                }
            }
        }

        // Handle range that is at the end of the buffer if it exists. There should never be
        // multiple because ranges must be disjoint.
        if cur_range.is_some() {
            debug_assert_eq!(old_ranges.next(), None);
            let last_fragment = new_fragments.last().unwrap();
            ops.push(Operation::Edit {
                edit: EditOperation {
                    id: local_timestamp,
                    start_id: last_fragment.insertion.id,
                    start_offset: last_fragment.range_in_insertion.end,
                    end_id: last_fragment.insertion.id,
                    end_offset: last_fragment.range_in_insertion.end,
                    version_in_range: time::Global::new(),
                    // TODO: avoid cloning the String.
                    new_text: new_text.clone(),
                },
                lamport_timestamp,
            });

            if let Some(new_text) = new_text {
                let new_fragment = self.build_fragment_to_insert(
                    &last_fragment,
                    None,
                    &new_text,
                    local_timestamp,
                    lamport_timestamp,
                );
                new_visible_text.insert(new_fragments.summary().text.visible, &new_text);
                new_fragments.push(new_fragment, &());
            }
        } else {
            new_fragments.push_tree(
                cursor.slice(&old_fragments.extent::<usize>(), SeekBias::Right, &()),
                &(),
            );
        }

        self.visible_text = new_visible_text;
        self.deleted_text = new_deleted_text;
        self.fragments = new_fragments;
        ops
    }

    fn split_fragment(
        &mut self,
        prev_fragment: &Fragment,
        fragment: &Fragment,
        range: Range<usize>,
    ) -> (Option<Fragment>, Option<Fragment>, Option<Fragment>) {
        debug_assert!(range.start >= fragment.range_in_insertion.start);
        debug_assert!(range.start <= fragment.range_in_insertion.end);
        debug_assert!(range.end <= fragment.range_in_insertion.end);
        debug_assert!(range.end >= fragment.range_in_insertion.start);

        if range.end == fragment.range_in_insertion.start {
            (None, None, Some(fragment.clone()))
        } else if range.start == fragment.range_in_insertion.end {
            (Some(fragment.clone()), None, None)
        } else if range.start == fragment.range_in_insertion.start
            && range.end == fragment.range_in_insertion.end
        {
            (None, Some(fragment.clone()), None)
        } else {
            let mut prefix = fragment.clone();

            let after_range = if range.end < fragment.range_in_insertion.end {
                let mut suffix = prefix.clone();
                suffix.range_in_insertion.start = range.end;
                prefix.range_in_insertion.end = range.end;
                prefix.id = FragmentId::between(&prev_fragment.id, &suffix.id);
                Some(suffix)
            } else {
                None
            };

            let within_range = if range.start != range.end {
                let mut suffix = prefix.clone();
                suffix.range_in_insertion.start = range.start;
                prefix.range_in_insertion.end = range.start;
                prefix.id = FragmentId::between(&prev_fragment.id, &suffix.id);
                Some(suffix)
            } else {
                None
            };

            let before_range = if range.start > fragment.range_in_insertion.start {
                Some(prefix)
            } else {
                None
            };

            let old_split_tree = self
                .insertion_splits
                .remove(&fragment.insertion.id)
                .unwrap();
            let mut cursor = old_split_tree.cursor::<usize, ()>();
            let mut new_split_tree =
                cursor.slice(&fragment.range_in_insertion.start, SeekBias::Right, &());

            if let Some(ref fragment) = before_range {
                new_split_tree.push(
                    InsertionSplit {
                        extent: range.start - fragment.range_in_insertion.start,
                        fragment_id: fragment.id.clone(),
                    },
                    &(),
                );
            }

            if let Some(ref fragment) = within_range {
                new_split_tree.push(
                    InsertionSplit {
                        extent: range.end - range.start,
                        fragment_id: fragment.id.clone(),
                    },
                    &(),
                );
            }

            if let Some(ref fragment) = after_range {
                new_split_tree.push(
                    InsertionSplit {
                        extent: fragment.range_in_insertion.end - range.end,
                        fragment_id: fragment.id.clone(),
                    },
                    &(),
                );
            }

            cursor.next();
            new_split_tree.push_tree(
                cursor.slice(&old_split_tree.extent::<usize>(), SeekBias::Right, &()),
                &(),
            );

            self.insertion_splits
                .insert(fragment.insertion.id, new_split_tree);

            (before_range, within_range, after_range)
        }
    }

    fn build_fragment_to_insert(
        &mut self,
        prev_fragment: &Fragment,
        next_fragment: Option<&Fragment>,
        text: &str,
        insertion_id: time::Local,
        lamport_timestamp: time::Lamport,
    ) -> Fragment {
        let new_fragment_id = FragmentId::between(
            &prev_fragment.id,
            next_fragment
                .map(|f| &f.id)
                .unwrap_or(&FragmentId::max_value()),
        );

        // TODO: extent could be expressed in bytes, which would save a linear scan.
        let range_in_insertion = 0..text.chars().count();
        let mut split_tree = SumTree::new();
        split_tree.push(
            InsertionSplit {
                extent: range_in_insertion.len(),
                fragment_id: new_fragment_id.clone(),
            },
            &(),
        );
        self.insertion_splits.insert(insertion_id, split_tree);

        Fragment::new(
            new_fragment_id,
            Arc::new(Insertion {
                id: insertion_id,
                parent_id: prev_fragment.insertion.id,
                offset_in_parent: prev_fragment.range_in_insertion.end,
                lamport_timestamp,
            }),
            range_in_insertion,
        )
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Result<Anchor> {
        self.anchor_at(position, AnchorBias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Result<Anchor> {
        self.anchor_at(position, AnchorBias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: AnchorBias) -> Result<Anchor> {
        let offset = position.to_offset(self)?;
        let max_offset = self.len();
        if offset > max_offset {
            return Err(anyhow!("offset is out of range"));
        }

        let seek_bias;
        match bias {
            AnchorBias::Left => {
                if offset == 0 {
                    return Ok(Anchor::Start);
                } else {
                    seek_bias = SeekBias::Left;
                }
            }
            AnchorBias::Right => {
                if offset == max_offset {
                    return Ok(Anchor::End);
                } else {
                    seek_bias = SeekBias::Right;
                }
            }
        };

        let mut cursor = self.fragments.cursor::<usize, usize>();
        cursor.seek(&offset, seek_bias, &());
        let fragment = cursor.item().unwrap();
        let offset_in_fragment = offset - cursor.start();
        let offset_in_insertion = fragment.range_in_insertion.start + offset_in_fragment;
        let anchor = Anchor::Middle {
            insertion_id: fragment.insertion.id,
            offset: offset_in_insertion,
            bias,
        };
        Ok(anchor)
    }

    fn fragment_id_for_anchor(&self, anchor: &Anchor) -> Result<&FragmentId> {
        match anchor {
            Anchor::Start => Ok(FragmentId::max_value()),
            Anchor::End => Ok(FragmentId::min_value()),
            Anchor::Middle {
                insertion_id,
                offset,
                bias,
                ..
            } => {
                let seek_bias = match bias {
                    AnchorBias::Left => SeekBias::Left,
                    AnchorBias::Right => SeekBias::Right,
                };

                let splits = self
                    .insertion_splits
                    .get(&insertion_id)
                    .ok_or_else(|| anyhow!("split does not exist for insertion id"))?;
                let mut splits_cursor = splits.cursor::<usize, ()>();
                splits_cursor.seek(offset, seek_bias, &());
                splits_cursor
                    .item()
                    .ok_or_else(|| anyhow!("split offset is out of range"))
                    .map(|split| &split.fragment_id)
            }
        }
    }

    fn summary_for_anchor(&self, anchor: &Anchor) -> Result<TextSummary> {
        match anchor {
            Anchor::Start => Ok(TextSummary::default()),
            Anchor::End => Ok(self.text_summary()),
            Anchor::Middle {
                insertion_id,
                offset,
                bias,
            } => {
                let seek_bias = match bias {
                    AnchorBias::Left => SeekBias::Left,
                    AnchorBias::Right => SeekBias::Right,
                };

                let splits = self
                    .insertion_splits
                    .get(&insertion_id)
                    .ok_or_else(|| anyhow!("split does not exist for insertion id"))?;
                let mut splits_cursor = splits.cursor::<usize, ()>();
                splits_cursor.seek(offset, seek_bias, &());
                let split = splits_cursor
                    .item()
                    .ok_or_else(|| anyhow!("split offset is out of range"))?;

                let mut fragments_cursor = self
                    .fragments
                    .cursor::<FragmentIdRef, FragmentTextSummary>();
                fragments_cursor.seek(&FragmentIdRef::new(&split.fragment_id), SeekBias::Left, &());
                let fragment = fragments_cursor
                    .item()
                    .ok_or_else(|| anyhow!("fragment id does not exist"))?;

                let mut ix = fragments_cursor.start().clone().visible;
                if fragment.visible {
                    ix += offset - fragment.range_in_insertion.start;
                }
                Ok(self.text_summary_for_range(0..ix))
            }
        }
    }

    pub fn point_for_offset(&self, offset: usize) -> Result<Point> {
        if offset <= self.len() {
            Ok(self.text_summary_for_range(0..offset).lines)
        } else {
            Err(anyhow!("offset out of bounds"))
        }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self {
            visible_text: self.visible_text.clone(),
            deleted_text: self.deleted_text.clone(),
            fragments: self.fragments.clone(),
            insertion_splits: self.insertion_splits.clone(),
            version: self.version.clone(),
            saved_version: self.saved_version.clone(),
            saved_mtime: self.saved_mtime,
            last_edit: self.last_edit.clone(),
            undo_map: self.undo_map.clone(),
            history: self.history.clone(),
            selections: self.selections.clone(),
            selections_last_update: self.selections_last_update.clone(),
            deferred_ops: self.deferred_ops.clone(),
            file: self.file.clone(),
            deferred_replicas: self.deferred_replicas.clone(),
            replica_id: self.replica_id,
            local_clock: self.local_clock.clone(),
            lamport_clock: self.lamport_clock.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Edited,
    Dirtied,
    Saved,
    FileHandleChanged,
    Reloaded,
}

impl Entity for Buffer {
    type Event = Event;
}

impl<'a, F: Fn(&FragmentSummary) -> bool> Iterator for Edits<'a, F> {
    type Item = Edit;

    fn next(&mut self) -> Option<Self::Item> {
        let mut change: Option<Edit> = None;

        while let Some(fragment) = self.cursor.item() {
            let new_offset = *self.cursor.start();
            let old_offset = (new_offset as isize - self.delta) as usize;

            if !fragment.was_visible(&self.since, &self.undos) && fragment.visible {
                if let Some(ref mut change) = change {
                    if change.new_range.end == new_offset {
                        change.new_range.end += fragment.len();
                        self.delta += fragment.len() as isize;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_range: old_offset..old_offset,
                        new_range: new_offset..new_offset + fragment.len(),
                    });
                    self.delta += fragment.len() as isize;
                }
            } else if fragment.was_visible(&self.since, &self.undos) && !fragment.visible {
                if let Some(ref mut change) = change {
                    if change.new_range.end == new_offset {
                        change.old_range.end += fragment.len();
                        self.delta -= fragment.len() as isize;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_range: old_offset..old_offset + fragment.len(),
                        new_range: new_offset..new_offset,
                    });
                    self.delta -= fragment.len() as isize;
                }
            }

            self.cursor.next();
        }

        change
    }
}

// pub fn diff(a: &[u16], b: &[u16]) -> Vec<Edit> {
//     struct EditCollector<'a> {
//         a: &'a [u16],
//         b: &'a [u16],
//         position: Point,
//         changes: Vec<Edit>,
//     }
//
//     impl<'a> diffs::Diff for EditCollector<'a> {
//         type Error = ();
//
//         fn equal(&mut self, old: usize, _: usize, len: usize) -> Result<(), ()> {
//             self.position += &Text::extent(&self.a[old..old + len]);
//             Ok(())
//         }
//
//         fn delete(&mut self, old: usize, len: usize) -> Result<(), ()> {
//             self.changes.push(Edit {
//                 range: self.position..self.position + &Text::extent(&self.a[old..old + len]),
//                 chars: Vec::new(),
//                 new_char_count: Point::zero(),
//             });
//             Ok(())
//         }
//
//         fn insert(&mut self, _: usize, new: usize, new_len: usize) -> Result<(), ()> {
//             let new_char_count = Text::extent(&self.b[new..new + new_len]);
//             self.changes.push(Edit {
//                 range: self.position..self.position,
//                 chars: Vec::from(&self.b[new..new + new_len]),
//                 new_char_count,
//             });
//             self.position += &new_char_count;
//             Ok(())
//         }
//
//         fn replace(
//             &mut self,
//             old: usize,
//             old_len: usize,
//             new: usize,
//             new_len: usize,
//         ) -> Result<(), ()> {
//             let old_extent = text::extent(&self.a[old..old + old_len]);
//             let new_char_count = text::extent(&self.b[new..new + new_len]);
//             self.changes.push(Edit {
//                 range: self.position..self.position + &old_extent,
//                 chars: Vec::from(&self.b[new..new + new_len]),
//                 new_char_count,
//             });
//             self.position += &new_char_count;
//             Ok(())
//         }
//     }
//
//     let mut collector = diffs::Replace::new(EditCollector {
//         a,
//         b,
//         position: Point::zero(),
//         changes: Vec::new(),
//     });
//     diffs::myers::diff(&mut collector, a, 0, a.len(), b, 0, b.len()).unwrap();
//     collector.into_inner().changes
// }

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct FragmentId(Arc<[u16]>);

lazy_static! {
    static ref FRAGMENT_ID_EMPTY: FragmentId = FragmentId(Arc::from([]));
    static ref FRAGMENT_ID_MIN_VALUE: FragmentId = FragmentId(Arc::from([0 as u16]));
    static ref FRAGMENT_ID_MAX_VALUE: FragmentId = FragmentId(Arc::from([u16::max_value()]));
}

impl Default for FragmentId {
    fn default() -> Self {
        FRAGMENT_ID_EMPTY.clone()
    }
}

impl FragmentId {
    fn min_value() -> &'static Self {
        &FRAGMENT_ID_MIN_VALUE
    }

    fn max_value() -> &'static Self {
        &FRAGMENT_ID_MAX_VALUE
    }

    fn between(left: &Self, right: &Self) -> Self {
        Self::between_with_max(left, right, u16::max_value())
    }

    fn between_with_max(left: &Self, right: &Self, max_value: u16) -> Self {
        let mut new_entries = Vec::new();

        let left_entries = left.0.iter().cloned().chain(iter::repeat(0));
        let right_entries = right.0.iter().cloned().chain(iter::repeat(max_value));
        for (l, r) in left_entries.zip(right_entries) {
            let interval = r - l;
            if interval > 1 {
                new_entries.push(l + cmp::max(1, cmp::min(8, interval / 2)));
                break;
            } else {
                new_entries.push(l);
            }
        }

        FragmentId(Arc::from(new_entries))
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Default)]
struct FragmentIdRef<'a>(Option<&'a FragmentId>);

impl<'a> FragmentIdRef<'a> {
    fn new(id: &'a FragmentId) -> Self {
        Self(Some(id))
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FragmentIdRef<'a> {
    fn add_summary(&mut self, summary: &'a FragmentSummary) {
        self.0 = Some(&summary.max_fragment_id)
    }
}

impl Fragment {
    fn new(id: FragmentId, insertion: Arc<Insertion>, range_in_insertion: Range<usize>) -> Self {
        Self {
            id,
            insertion,
            range_in_insertion,
            deletions: Default::default(),
            max_undos: Default::default(),
            visible: true,
        }
    }

    fn is_visible(&self, undos: &UndoMap) -> bool {
        !undos.is_undone(self.insertion.id) && self.deletions.iter().all(|d| undos.is_undone(*d))
    }

    fn was_visible(&self, version: &time::Global, undos: &UndoMap) -> bool {
        (version.observed(self.insertion.id) && !undos.was_undone(self.insertion.id, version))
            && self
                .deletions
                .iter()
                .all(|d| !version.observed(*d) || undos.was_undone(*d, version))
    }

    fn len(&self) -> usize {
        self.range_in_insertion.len()
    }

    fn visible_len(&self) -> usize {
        if self.visible {
            self.range_in_insertion.len()
        } else {
            0
        }
    }
}

impl sum_tree::Item for Fragment {
    type Summary = FragmentSummary;

    fn summary(&self) -> Self::Summary {
        let mut max_version = time::Global::new();
        max_version.observe(self.insertion.id);
        for deletion in &self.deletions {
            max_version.observe(*deletion);
        }
        max_version.observe_all(&self.max_undos);

        if self.visible {
            FragmentSummary {
                text: FragmentTextSummary {
                    visible: self.len(),
                    deleted: 0,
                },
                max_fragment_id: self.id.clone(),
                max_version,
            }
        } else {
            FragmentSummary {
                text: FragmentTextSummary {
                    visible: 0,
                    deleted: self.len(),
                },
                max_fragment_id: self.id.clone(),
                max_version,
            }
        }
    }
}

impl sum_tree::Summary for FragmentSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.text.visible += &other.text.visible;
        self.text.deleted += &other.text.deleted;
        debug_assert!(self.max_fragment_id <= other.max_fragment_id);
        self.max_fragment_id = other.max_fragment_id.clone();
        self.max_version.observe_all(&other.max_version);
    }
}

impl Default for FragmentSummary {
    fn default() -> Self {
        FragmentSummary {
            text: FragmentTextSummary::default(),
            max_fragment_id: FragmentId::min_value().clone(),
            max_version: time::Global::new(),
        }
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for usize {
    fn add_summary(&mut self, summary: &FragmentSummary) {
        *self += summary.text.visible;
    }
}

impl sum_tree::Item for InsertionSplit {
    type Summary = InsertionSplitSummary;

    fn summary(&self) -> Self::Summary {
        InsertionSplitSummary {
            extent: self.extent,
        }
    }
}

impl sum_tree::Summary for InsertionSplitSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.extent += other.extent;
    }
}

impl Default for InsertionSplitSummary {
    fn default() -> Self {
        InsertionSplitSummary { extent: 0 }
    }
}

impl<'a> sum_tree::Dimension<'a, InsertionSplitSummary> for usize {
    fn add_summary(&mut self, summary: &InsertionSplitSummary) {
        *self += &summary.extent;
    }
}

impl Operation {
    fn replica_id(&self) -> ReplicaId {
        self.lamport_timestamp().replica_id
    }

    fn lamport_timestamp(&self) -> time::Lamport {
        match self {
            Operation::Edit {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::Undo {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::UpdateSelections {
                lamport_timestamp, ..
            } => *lamport_timestamp,
        }
    }

    pub fn is_edit(&self) -> bool {
        match self {
            Operation::Edit { .. } => true,
            _ => false,
        }
    }
}

impl operation_queue::Operation for Operation {
    fn timestamp(&self) -> time::Lamport {
        self.lamport_timestamp()
    }
}

pub trait ToOffset {
    fn to_offset(&self, buffer: &Buffer) -> Result<usize>;
}

impl ToOffset for Point {
    fn to_offset(&self, buffer: &Buffer) -> Result<usize> {
        if *self <= buffer.max_point() {
            // TODO: return an error if line is shorter than column.
            Ok(buffer.visible_text.line_to_char(self.row as usize) + self.column as usize)
        } else {
            Err(anyhow!("point is out of bounds"))
        }
    }
}

impl ToOffset for usize {
    fn to_offset(&self, _: &Buffer) -> Result<usize> {
        Ok(*self)
    }
}

impl ToOffset for Anchor {
    fn to_offset(&self, buffer: &Buffer) -> Result<usize> {
        Ok(buffer.summary_for_anchor(self)?.chars)
    }
}

impl<'a> ToOffset for &'a Anchor {
    fn to_offset(&self, buffer: &Buffer) -> Result<usize> {
        Ok(buffer.summary_for_anchor(self)?.chars)
    }
}

pub trait ToPoint {
    fn to_point(&self, buffer: &Buffer) -> Result<Point>;
}

impl ToPoint for Anchor {
    fn to_point(&self, buffer: &Buffer) -> Result<Point> {
        Ok(buffer.summary_for_anchor(self)?.lines)
    }
}

impl ToPoint for usize {
    fn to_point(&self, buffer: &Buffer) -> Result<Point> {
        if *self <= buffer.len() {
            let row = buffer.visible_text.char_to_line(*self);
            let column = *self - buffer.visible_text.line_to_char(row);
            Ok(Point::new(row as u32, column as u32))
        } else {
            Err(anyhow!("offset is out of bounds"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test::temp_tree,
        worktree::{Worktree, WorktreeHandle},
    };
    use cmp::Ordering;
    use gpui::App;
    use serde_json::json;
    use std::{
        cell::RefCell,
        collections::BTreeMap,
        fs,
        rc::Rc,
        sync::atomic::{self, AtomicUsize},
    };

    #[gpui::test]
    fn test_edit(ctx: &mut gpui::MutableAppContext) {
        ctx.add_model(|ctx| {
            let mut buffer = Buffer::new(0, "abc", ctx);
            assert_eq!(buffer.text(), "abc");
            buffer.edit(vec![3..3], "def", None).unwrap();
            assert_eq!(buffer.text(), "abcdef");
            buffer.edit(vec![0..0], "ghi", None).unwrap();
            assert_eq!(buffer.text(), "ghiabcdef");
            buffer.edit(vec![5..5], "jkl", None).unwrap();
            assert_eq!(buffer.text(), "ghiabjklcdef");
            buffer.edit(vec![6..7], "", None).unwrap();
            assert_eq!(buffer.text(), "ghiabjlcdef");
            buffer.edit(vec![4..9], "mno", None).unwrap();
            assert_eq!(buffer.text(), "ghiamnoef");
            buffer
        });
    }

    #[gpui::test]
    fn test_edit_events(app: &mut gpui::MutableAppContext) {
        let mut now = Instant::now();
        let buffer_1_events = Rc::new(RefCell::new(Vec::new()));
        let buffer_2_events = Rc::new(RefCell::new(Vec::new()));

        let buffer1 = app.add_model(|ctx| Buffer::new(0, "abcdef", ctx));
        let buffer2 = app.add_model(|ctx| Buffer::new(1, "abcdef", ctx));
        let mut buffer_ops = Vec::new();
        buffer1.update(app, |buffer, ctx| {
            let buffer_1_events = buffer_1_events.clone();
            ctx.subscribe(&buffer1, move |_, event, _| {
                buffer_1_events.borrow_mut().push(event.clone())
            });
            let buffer_2_events = buffer_2_events.clone();
            ctx.subscribe(&buffer2, move |_, event, _| {
                buffer_2_events.borrow_mut().push(event.clone())
            });

            // An edit emits an edited event, followed by a dirtied event,
            // since the buffer was previously in a clean state.
            let ops = buffer.edit(Some(2..4), "XYZ", Some(ctx)).unwrap();
            buffer_ops.extend_from_slice(&ops);

            // An empty transaction does not emit any events.
            buffer.start_transaction(None).unwrap();
            buffer.end_transaction(None, Some(ctx)).unwrap();

            // A transaction containing two edits emits one edited event.
            now += Duration::from_secs(1);
            buffer.start_transaction_at(None, now).unwrap();
            let ops = buffer.edit(Some(5..5), "u", Some(ctx)).unwrap();
            buffer_ops.extend_from_slice(&ops);
            let ops = buffer.edit(Some(6..6), "w", Some(ctx)).unwrap();
            buffer_ops.extend_from_slice(&ops);
            buffer.end_transaction_at(None, now, Some(ctx)).unwrap();

            // Undoing a transaction emits one edited event.
            let ops = buffer.undo(Some(ctx));
            buffer_ops.extend_from_slice(&ops);
        });

        // Incorporating a set of remote ops emits a single edited event,
        // followed by a dirtied event.
        buffer2.update(app, |buffer, ctx| {
            buffer.apply_ops(buffer_ops, Some(ctx)).unwrap();
        });

        let buffer_1_events = buffer_1_events.borrow();
        assert_eq!(
            *buffer_1_events,
            vec![Event::Edited, Event::Dirtied, Event::Edited, Event::Edited]
        );

        let buffer_2_events = buffer_2_events.borrow();
        assert_eq!(*buffer_2_events, vec![Event::Edited, Event::Dirtied]);
    }

    #[gpui::test]
    fn test_random_edits(ctx: &mut gpui::MutableAppContext) {
        for seed in 0..100 {
            println!("{:?}", seed);
            let mut rng = &mut StdRng::seed_from_u64(seed);

            let reference_string_len = rng.gen_range(0..3);
            let mut reference_string = RandomCharIter::new(&mut rng)
                .take(reference_string_len)
                .collect::<String>();
            ctx.add_model(|ctx| {
                let mut buffer = Buffer::new(0, reference_string.as_str(), ctx);
                let mut buffer_versions = Vec::new();
                for _i in 0..10 {
                    let (old_ranges, new_text, _) = buffer.randomly_mutate(rng, None);
                    for old_range in old_ranges.iter().rev() {
                        reference_string = [
                            &reference_string[0..old_range.start],
                            new_text.as_str(),
                            &reference_string[old_range.end..],
                        ]
                        .concat();
                    }
                    assert_eq!(buffer.text(), reference_string);

                    if rng.gen_bool(0.25) {
                        buffer.randomly_undo_redo(rng);
                        reference_string = buffer.text();
                    }

                    {
                        let line_lengths = line_lengths_in_range(&buffer, 0..buffer.len());

                        for (len, rows) in &line_lengths {
                            for row in rows {
                                assert_eq!(buffer.line_len(*row).unwrap(), *len);
                            }
                        }

                        let (longest_column, longest_rows) =
                            line_lengths.iter().next_back().unwrap();
                        // let rightmost_point = buffer.rightmost_point();
                        // assert_eq!(rightmost_point.column, *longest_column);
                        // assert!(longest_rows.contains(&rightmost_point.row));
                    }

                    for _ in 0..5 {
                        let end = rng.gen_range(0..buffer.len() + 1);
                        let start = rng.gen_range(0..end + 1);

                        let line_lengths = line_lengths_in_range(&buffer, start..end);
                        let (longest_column, longest_rows) =
                            line_lengths.iter().next_back().unwrap();
                        let range_sum = buffer.text_summary_for_range(start..end);
                        // TODO: re-enable when we have rightmost point again.
                        // assert_eq!(range_sum.rightmost_point.column, *longest_column);
                        // assert!(longest_rows.contains(&range_sum.rightmost_point.row));
                        let range_text = &buffer.text()[start..end];
                        assert_eq!(range_sum.chars, range_text.chars().count());
                        assert_eq!(range_sum.bytes, range_text.len());
                    }

                    if rng.gen_bool(0.3) {
                        buffer_versions.push(buffer.clone());
                    }
                }

                for mut old_buffer in buffer_versions {
                    let mut delta = 0_isize;
                    for Edit {
                        old_range,
                        new_range,
                    } in buffer.edits_since(old_buffer.version.clone())
                    {
                        let old_len = old_range.end - old_range.start;
                        let new_len = new_range.end - new_range.start;
                        let old_start = (old_range.start as isize + delta) as usize;
                        let new_text: String = buffer.text_for_range(new_range).unwrap().collect();
                        old_buffer
                            .edit(Some(old_start..old_start + old_len), new_text, None)
                            .unwrap();

                        delta += new_len as isize - old_len as isize;
                    }
                    assert_eq!(old_buffer.text(), buffer.text());
                }

                buffer
            });
        }
    }

    #[gpui::test]
    fn test_line_len(ctx: &mut gpui::MutableAppContext) {
        ctx.add_model(|ctx| {
            let mut buffer = Buffer::new(0, "", ctx);
            buffer.edit(vec![0..0], "abcd\nefg\nhij", None).unwrap();
            buffer.edit(vec![12..12], "kl\nmno", None).unwrap();
            buffer.edit(vec![18..18], "\npqrs\n", None).unwrap();
            buffer.edit(vec![18..21], "\nPQ", None).unwrap();

            assert_eq!(buffer.line_len(0).unwrap(), 4);
            assert_eq!(buffer.line_len(1).unwrap(), 3);
            assert_eq!(buffer.line_len(2).unwrap(), 5);
            assert_eq!(buffer.line_len(3).unwrap(), 3);
            assert_eq!(buffer.line_len(4).unwrap(), 4);
            assert_eq!(buffer.line_len(5).unwrap(), 0);
            assert!(buffer.line_len(6).is_err());
            buffer
        });
    }

    #[gpui::test]
    fn test_rightmost_point(ctx: &mut gpui::MutableAppContext) {
        ctx.add_model(|ctx| {
            let mut buffer = Buffer::new(0, "", ctx);
            assert_eq!(buffer.rightmost_point().row, 0);
            buffer.edit(vec![0..0], "abcd\nefg\nhij", None).unwrap();
            assert_eq!(buffer.rightmost_point().row, 0);
            buffer.edit(vec![12..12], "kl\nmno", None).unwrap();
            assert_eq!(buffer.rightmost_point().row, 2);
            buffer.edit(vec![18..18], "\npqrs", None).unwrap();
            assert_eq!(buffer.rightmost_point().row, 2);
            buffer.edit(vec![10..12], "", None).unwrap();
            assert_eq!(buffer.rightmost_point().row, 0);
            buffer.edit(vec![24..24], "tuv", None).unwrap();
            assert_eq!(buffer.rightmost_point().row, 4);
            buffer
        });
    }

    #[gpui::test]
    fn test_text_summary_for_range(ctx: &mut gpui::MutableAppContext) {
        ctx.add_model(|ctx| {
            let buffer = Buffer::new(0, "ab\nefg\nhklm\nnopqrs\ntuvwxyz", ctx);
            assert_eq!(
                buffer.text_summary_for_range(1..3),
                TextSummary {
                    chars: 2,
                    bytes: 2,
                    lines: Point::new(1, 0)
                }
            );
            assert_eq!(
                buffer.text_summary_for_range(1..12),
                TextSummary {
                    chars: 11,
                    bytes: 11,
                    lines: Point::new(3, 0)
                }
            );
            assert_eq!(
                buffer.text_summary_for_range(0..20),
                TextSummary {
                    chars: 20,
                    bytes: 20,
                    lines: Point::new(4, 1)
                }
            );
            assert_eq!(
                buffer.text_summary_for_range(0..22),
                TextSummary {
                    chars: 22,
                    bytes: 22,
                    lines: Point::new(4, 3)
                }
            );
            assert_eq!(
                buffer.text_summary_for_range(7..22),
                TextSummary {
                    chars: 15,
                    bytes: 15,
                    lines: Point::new(2, 3)
                }
            );
            buffer
        });
    }

    #[gpui::test]
    fn test_chars_at(ctx: &mut gpui::MutableAppContext) {
        ctx.add_model(|ctx| {
            let mut buffer = Buffer::new(0, "", ctx);
            buffer.edit(vec![0..0], "abcd\nefgh\nij", None).unwrap();
            buffer.edit(vec![12..12], "kl\nmno", None).unwrap();
            buffer.edit(vec![18..18], "\npqrs", None).unwrap();
            buffer.edit(vec![18..21], "\nPQ", None).unwrap();

            let chars = buffer.chars_at(Point::new(0, 0)).unwrap();
            assert_eq!(chars.collect::<String>(), "abcd\nefgh\nijkl\nmno\nPQrs");

            let chars = buffer.chars_at(Point::new(1, 0)).unwrap();
            assert_eq!(chars.collect::<String>(), "efgh\nijkl\nmno\nPQrs");

            let chars = buffer.chars_at(Point::new(2, 0)).unwrap();
            assert_eq!(chars.collect::<String>(), "ijkl\nmno\nPQrs");

            let chars = buffer.chars_at(Point::new(3, 0)).unwrap();
            assert_eq!(chars.collect::<String>(), "mno\nPQrs");

            let chars = buffer.chars_at(Point::new(4, 0)).unwrap();
            assert_eq!(chars.collect::<String>(), "PQrs");

            // Regression test:
            let mut buffer = Buffer::new(0, "", ctx);
            buffer.edit(vec![0..0], "[workspace]\nmembers = [\n    \"xray_core\",\n    \"xray_server\",\n    \"xray_cli\",\n    \"xray_wasm\",\n]\n", None).unwrap();
            buffer.edit(vec![60..60], "\n", None).unwrap();

            let chars = buffer.chars_at(Point::new(6, 0)).unwrap();
            assert_eq!(chars.collect::<String>(), "    \"xray_wasm\",\n]\n");

            buffer
        });
    }

    #[test]
    fn test_fragment_ids() {
        for seed in 0..10 {
            let rng = &mut StdRng::seed_from_u64(seed);

            let mut ids = vec![FragmentId(Arc::from([0])), FragmentId(Arc::from([4]))];
            for _i in 0..100 {
                let index = rng.gen_range(1..ids.len());

                let left = ids[index - 1].clone();
                let right = ids[index].clone();
                ids.insert(index, FragmentId::between_with_max(&left, &right, 4));

                let mut sorted_ids = ids.clone();
                sorted_ids.sort();
                assert_eq!(ids, sorted_ids);
            }
        }
    }

    #[gpui::test]
    fn test_anchors(ctx: &mut gpui::MutableAppContext) {
        ctx.add_model(|ctx| {
            let mut buffer = Buffer::new(0, "", ctx);
            buffer.edit(vec![0..0], "abc", None).unwrap();
            let left_anchor = buffer.anchor_before(2).unwrap();
            let right_anchor = buffer.anchor_after(2).unwrap();

            buffer.edit(vec![1..1], "def\n", None).unwrap();
            assert_eq!(buffer.text(), "adef\nbc");
            assert_eq!(left_anchor.to_offset(&buffer).unwrap(), 6);
            assert_eq!(right_anchor.to_offset(&buffer).unwrap(), 6);
            assert_eq!(
                left_anchor.to_point(&buffer).unwrap(),
                Point { row: 1, column: 1 }
            );
            assert_eq!(
                right_anchor.to_point(&buffer).unwrap(),
                Point { row: 1, column: 1 }
            );

            buffer.edit(vec![2..3], "", None).unwrap();
            assert_eq!(buffer.text(), "adf\nbc");
            assert_eq!(left_anchor.to_offset(&buffer).unwrap(), 5);
            assert_eq!(right_anchor.to_offset(&buffer).unwrap(), 5);
            assert_eq!(
                left_anchor.to_point(&buffer).unwrap(),
                Point { row: 1, column: 1 }
            );
            assert_eq!(
                right_anchor.to_point(&buffer).unwrap(),
                Point { row: 1, column: 1 }
            );

            buffer.edit(vec![5..5], "ghi\n", None).unwrap();
            assert_eq!(buffer.text(), "adf\nbghi\nc");
            assert_eq!(left_anchor.to_offset(&buffer).unwrap(), 5);
            assert_eq!(right_anchor.to_offset(&buffer).unwrap(), 9);
            assert_eq!(
                left_anchor.to_point(&buffer).unwrap(),
                Point { row: 1, column: 1 }
            );
            assert_eq!(
                right_anchor.to_point(&buffer).unwrap(),
                Point { row: 2, column: 0 }
            );

            buffer.edit(vec![7..9], "", None).unwrap();
            assert_eq!(buffer.text(), "adf\nbghc");
            assert_eq!(left_anchor.to_offset(&buffer).unwrap(), 5);
            assert_eq!(right_anchor.to_offset(&buffer).unwrap(), 7);
            assert_eq!(
                left_anchor.to_point(&buffer).unwrap(),
                Point { row: 1, column: 1 },
            );
            assert_eq!(
                right_anchor.to_point(&buffer).unwrap(),
                Point { row: 1, column: 3 }
            );

            // Ensure anchoring to a point is equivalent to anchoring to an offset.
            assert_eq!(
                buffer.anchor_before(Point { row: 0, column: 0 }).unwrap(),
                buffer.anchor_before(0).unwrap()
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 0, column: 1 }).unwrap(),
                buffer.anchor_before(1).unwrap()
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 0, column: 2 }).unwrap(),
                buffer.anchor_before(2).unwrap()
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 0, column: 3 }).unwrap(),
                buffer.anchor_before(3).unwrap()
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 0 }).unwrap(),
                buffer.anchor_before(4).unwrap()
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 1 }).unwrap(),
                buffer.anchor_before(5).unwrap()
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 2 }).unwrap(),
                buffer.anchor_before(6).unwrap()
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 3 }).unwrap(),
                buffer.anchor_before(7).unwrap()
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 4 }).unwrap(),
                buffer.anchor_before(8).unwrap()
            );

            // Comparison between anchors.
            let anchor_at_offset_0 = buffer.anchor_before(0).unwrap();
            let anchor_at_offset_1 = buffer.anchor_before(1).unwrap();
            let anchor_at_offset_2 = buffer.anchor_before(2).unwrap();

            assert_eq!(
                anchor_at_offset_0
                    .cmp(&anchor_at_offset_0, &buffer)
                    .unwrap(),
                Ordering::Equal
            );
            assert_eq!(
                anchor_at_offset_1
                    .cmp(&anchor_at_offset_1, &buffer)
                    .unwrap(),
                Ordering::Equal
            );
            assert_eq!(
                anchor_at_offset_2
                    .cmp(&anchor_at_offset_2, &buffer)
                    .unwrap(),
                Ordering::Equal
            );

            assert_eq!(
                anchor_at_offset_0
                    .cmp(&anchor_at_offset_1, &buffer)
                    .unwrap(),
                Ordering::Less
            );
            assert_eq!(
                anchor_at_offset_1
                    .cmp(&anchor_at_offset_2, &buffer)
                    .unwrap(),
                Ordering::Less
            );
            assert_eq!(
                anchor_at_offset_0
                    .cmp(&anchor_at_offset_2, &buffer)
                    .unwrap(),
                Ordering::Less
            );

            assert_eq!(
                anchor_at_offset_1
                    .cmp(&anchor_at_offset_0, &buffer)
                    .unwrap(),
                Ordering::Greater
            );
            assert_eq!(
                anchor_at_offset_2
                    .cmp(&anchor_at_offset_1, &buffer)
                    .unwrap(),
                Ordering::Greater
            );
            assert_eq!(
                anchor_at_offset_2
                    .cmp(&anchor_at_offset_0, &buffer)
                    .unwrap(),
                Ordering::Greater
            );
            buffer
        });
    }

    #[gpui::test]
    fn test_anchors_at_start_and_end(ctx: &mut gpui::MutableAppContext) {
        ctx.add_model(|ctx| {
            let mut buffer = Buffer::new(0, "", ctx);
            let before_start_anchor = buffer.anchor_before(0).unwrap();
            let after_end_anchor = buffer.anchor_after(0).unwrap();

            buffer.edit(vec![0..0], "abc", None).unwrap();
            assert_eq!(buffer.text(), "abc");
            assert_eq!(before_start_anchor.to_offset(&buffer).unwrap(), 0);
            assert_eq!(after_end_anchor.to_offset(&buffer).unwrap(), 3);

            let after_start_anchor = buffer.anchor_after(0).unwrap();
            let before_end_anchor = buffer.anchor_before(3).unwrap();

            buffer.edit(vec![3..3], "def", None).unwrap();
            buffer.edit(vec![0..0], "ghi", None).unwrap();
            assert_eq!(buffer.text(), "ghiabcdef");
            assert_eq!(before_start_anchor.to_offset(&buffer).unwrap(), 0);
            assert_eq!(after_start_anchor.to_offset(&buffer).unwrap(), 3);
            assert_eq!(before_end_anchor.to_offset(&buffer).unwrap(), 6);
            assert_eq!(after_end_anchor.to_offset(&buffer).unwrap(), 9);
            buffer
        });
    }

    #[test]
    fn test_is_dirty() {
        App::test_async((), |mut app| async move {
            let dir = temp_tree(json!({
                "file1": "",
                "file2": "",
                "file3": "",
            }));
            let tree = app.add_model(|ctx| Worktree::new(dir.path(), ctx));
            tree.flush_fs_events(&app).await;
            app.read(|ctx| tree.read(ctx).scan_complete()).await;

            let file1 = app.update(|ctx| tree.file("file1", ctx)).await;
            let buffer1 = app.add_model(|ctx| {
                Buffer::from_history(0, History::new("abc".into()), Some(file1), ctx)
            });
            let events = Rc::new(RefCell::new(Vec::new()));

            // initially, the buffer isn't dirty.
            buffer1.update(&mut app, |buffer, ctx| {
                ctx.subscribe(&buffer1, {
                    let events = events.clone();
                    move |_, event, _| events.borrow_mut().push(event.clone())
                });

                assert!(!buffer.is_dirty());
                assert!(events.borrow().is_empty());

                buffer.edit(vec![1..2], "", Some(ctx)).unwrap();
            });

            // after the first edit, the buffer is dirty, and emits a dirtied event.
            buffer1.update(&mut app, |buffer, ctx| {
                assert!(buffer.text() == "ac");
                assert!(buffer.is_dirty());
                assert_eq!(*events.borrow(), &[Event::Edited, Event::Dirtied]);
                events.borrow_mut().clear();

                buffer.did_save(buffer.version(), None, ctx);
            });

            // after saving, the buffer is not dirty, and emits a saved event.
            buffer1.update(&mut app, |buffer, ctx| {
                assert!(!buffer.is_dirty());
                assert_eq!(*events.borrow(), &[Event::Saved]);
                events.borrow_mut().clear();

                buffer.edit(vec![1..1], "B", Some(ctx)).unwrap();
                buffer.edit(vec![2..2], "D", Some(ctx)).unwrap();
            });

            // after editing again, the buffer is dirty, and emits another dirty event.
            buffer1.update(&mut app, |buffer, ctx| {
                assert!(buffer.text() == "aBDc");
                assert!(buffer.is_dirty());
                assert_eq!(
                    *events.borrow(),
                    &[Event::Edited, Event::Dirtied, Event::Edited],
                );
                events.borrow_mut().clear();

                // TODO - currently, after restoring the buffer to its
                // previously-saved state, the is still considered dirty.
                buffer.edit(vec![1..3], "", Some(ctx)).unwrap();
                assert!(buffer.text() == "ac");
                assert!(buffer.is_dirty());
            });

            assert_eq!(*events.borrow(), &[Event::Edited]);

            // When a file is deleted, the buffer is considered dirty.
            let events = Rc::new(RefCell::new(Vec::new()));
            let file2 = app.update(|ctx| tree.file("file2", ctx)).await;
            let buffer2 = app.add_model(|ctx: &mut ModelContext<Buffer>| {
                ctx.subscribe(&ctx.handle(), {
                    let events = events.clone();
                    move |_, event, _| events.borrow_mut().push(event.clone())
                });

                Buffer::from_history(0, History::new("abc".into()), Some(file2), ctx)
            });

            fs::remove_file(dir.path().join("file2")).unwrap();
            buffer2
                .condition_with_duration(Duration::from_millis(500), &app, |b, _| b.is_dirty())
                .await;
            assert_eq!(
                *events.borrow(),
                &[Event::Dirtied, Event::FileHandleChanged]
            );

            // When a file is already dirty when deleted, we don't emit a Dirtied event.
            let events = Rc::new(RefCell::new(Vec::new()));
            let file3 = app.update(|ctx| tree.file("file3", ctx)).await;
            let buffer3 = app.add_model(|ctx: &mut ModelContext<Buffer>| {
                ctx.subscribe(&ctx.handle(), {
                    let events = events.clone();
                    move |_, event, _| events.borrow_mut().push(event.clone())
                });

                Buffer::from_history(0, History::new("abc".into()), Some(file3), ctx)
            });

            tree.flush_fs_events(&app).await;
            buffer3.update(&mut app, |buffer, ctx| {
                buffer.edit(Some(0..0), "x", Some(ctx)).unwrap();
            });
            events.borrow_mut().clear();
            fs::remove_file(dir.path().join("file3")).unwrap();
            buffer3
                .condition_with_duration(Duration::from_millis(500), &app, |_, _| {
                    !events.borrow().is_empty()
                })
                .await;
            assert_eq!(*events.borrow(), &[Event::FileHandleChanged]);
            app.read(|ctx| assert!(buffer3.read(ctx).is_dirty()));
        });
    }

    #[gpui::test]
    async fn test_file_changes_on_disk(mut app: gpui::TestAppContext) {
        let initial_contents = "aaa\nbbbbb\nc\n";
        let dir = temp_tree(json!({ "the-file": initial_contents }));
        let tree = app.add_model(|ctx| Worktree::new(dir.path(), ctx));
        app.read(|ctx| tree.read(ctx).scan_complete()).await;

        let abs_path = dir.path().join("the-file");
        let file = app.update(|ctx| tree.file("the-file", ctx)).await;
        let buffer = app.add_model(|ctx| {
            Buffer::from_history(0, History::new(initial_contents.into()), Some(file), ctx)
        });

        // Add a cursor at the start of each row.
        let (selection_set_id, _) = buffer.update(&mut app, |buffer, ctx| {
            assert!(!buffer.is_dirty());
            buffer.add_selection_set(
                (0..3)
                    .map(|row| {
                        let anchor = buffer
                            .anchor_at(Point::new(row, 0), AnchorBias::Right)
                            .unwrap();
                        Selection {
                            id: row as usize,
                            start: anchor.clone(),
                            end: anchor,
                            reversed: false,
                            goal: SelectionGoal::None,
                        }
                    })
                    .collect::<Vec<_>>(),
                Some(ctx),
            )
        });

        // Change the file on disk, adding two new lines of text, and removing
        // one line.
        buffer.read_with(&app, |buffer, _| {
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        let new_contents = "AAAA\naaa\nBB\nbbbbb\n";
        fs::write(&abs_path, new_contents).unwrap();

        // Because the buffer was not modified, it is reloaded from disk. Its
        // contents are edited according to the diff between the old and new
        // file contents.
        buffer
            .condition_with_duration(Duration::from_millis(500), &app, |buffer, _| {
                buffer.text() != initial_contents
            })
            .await;

        buffer.update(&mut app, |buffer, _| {
            assert_eq!(buffer.text(), new_contents);
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());

            let selections = buffer.selections(selection_set_id).unwrap();
            let cursor_positions = selections
                .iter()
                .map(|selection| {
                    assert_eq!(selection.start, selection.end);
                    selection.start.to_point(&buffer).unwrap()
                })
                .collect::<Vec<_>>();
            assert_eq!(
                cursor_positions,
                &[Point::new(1, 0), Point::new(3, 0), Point::new(4, 0),]
            );
        });

        // Modify the buffer
        buffer.update(&mut app, |buffer, ctx| {
            buffer.edit(vec![0..0], " ", Some(ctx)).unwrap();
            assert!(buffer.is_dirty());
        });

        // Change the file on disk again, adding blank lines to the beginning.
        fs::write(&abs_path, "\n\n\nAAAA\naaa\nBB\nbbbbb\n").unwrap();

        // Becaues the buffer is modified, it doesn't reload from disk, but is
        // marked as having a conflict.
        buffer
            .condition_with_duration(Duration::from_millis(500), &app, |buffer, _| {
                buffer.has_conflict()
            })
            .await;
    }

    #[gpui::test]
    async fn test_set_text_via_diff(mut app: gpui::TestAppContext) {
        let text = "a\nbb\nccc\ndddd\neeeee\nffffff\n";
        let buffer = app.add_model(|ctx| Buffer::new(0, text, ctx));

        let text = "a\nccc\ndddd\nffffff\n";
        let diff = buffer
            .read_with(&app, |b, ctx| b.diff(text.into(), ctx))
            .await;
        buffer.update(&mut app, |b, ctx| b.set_text_via_diff(diff, ctx));
        app.read(|ctx| assert_eq!(buffer.read(ctx).text(), text));

        let text = "a\n1\n\nccc\ndd2dd\nffffff\n";
        let diff = buffer
            .read_with(&app, |b, ctx| b.diff(text.into(), ctx))
            .await;
        buffer.update(&mut app, |b, ctx| b.set_text_via_diff(diff, ctx));
        app.read(|ctx| assert_eq!(buffer.read(ctx).text(), text));
    }

    #[gpui::test]
    fn test_undo_redo(app: &mut gpui::MutableAppContext) {
        app.add_model(|ctx| {
            let mut buffer = Buffer::new(0, "1234", ctx);

            let edit1 = buffer.edit(vec![1..1], "abx", None).unwrap();
            let edit2 = buffer.edit(vec![3..4], "yzef", None).unwrap();
            let edit3 = buffer.edit(vec![3..5], "cd", None).unwrap();
            assert_eq!(buffer.text(), "1abcdef234");

            buffer.undo_or_redo(edit1[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1cdef234");
            buffer.undo_or_redo(edit1[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1abcdef234");

            buffer.undo_or_redo(edit2[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1abcdx234");
            buffer.undo_or_redo(edit3[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1abx234");
            buffer.undo_or_redo(edit2[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1abyzef234");
            buffer.undo_or_redo(edit3[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1abcdef234");

            buffer.undo_or_redo(edit3[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1abyzef234");
            buffer.undo_or_redo(edit1[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1yzef234");
            buffer.undo_or_redo(edit2[0].edit_id().unwrap()).unwrap();
            assert_eq!(buffer.text(), "1234");

            buffer
        });
    }

    #[gpui::test]
    fn test_history(app: &mut gpui::MutableAppContext) {
        app.add_model(|ctx| {
            let mut now = Instant::now();
            let mut buffer = Buffer::new(0, "123456", ctx);

            let (set_id, _) =
                buffer.add_selection_set(buffer.selections_from_ranges(vec![4..4]).unwrap(), None);
            buffer.start_transaction_at(Some(set_id), now).unwrap();
            buffer.edit(vec![2..4], "cd", None).unwrap();
            buffer.end_transaction_at(Some(set_id), now, None).unwrap();
            assert_eq!(buffer.text(), "12cd56");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![4..4]);

            buffer.start_transaction_at(Some(set_id), now).unwrap();
            buffer
                .update_selection_set(
                    set_id,
                    buffer.selections_from_ranges(vec![1..3]).unwrap(),
                    None,
                )
                .unwrap();
            buffer.edit(vec![4..5], "e", None).unwrap();
            buffer.end_transaction_at(Some(set_id), now, None).unwrap();
            assert_eq!(buffer.text(), "12cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

            now += UNDO_GROUP_INTERVAL + Duration::from_millis(1);
            buffer.start_transaction_at(Some(set_id), now).unwrap();
            buffer
                .update_selection_set(
                    set_id,
                    buffer.selections_from_ranges(vec![2..2]).unwrap(),
                    None,
                )
                .unwrap();
            buffer.edit(vec![0..1], "a", None).unwrap();
            buffer.edit(vec![1..1], "b", None).unwrap();
            buffer.end_transaction_at(Some(set_id), now, None).unwrap();
            assert_eq!(buffer.text(), "ab2cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![3..3]);

            // Last transaction happened past the group interval, undo it on its
            // own.
            buffer.undo(None);
            assert_eq!(buffer.text(), "12cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

            // First two transactions happened within the group interval, undo them
            // together.
            buffer.undo(None);
            assert_eq!(buffer.text(), "123456");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![4..4]);

            // Redo the first two transactions together.
            buffer.redo(None);
            assert_eq!(buffer.text(), "12cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

            // Redo the last transaction on its own.
            buffer.redo(None);
            assert_eq!(buffer.text(), "ab2cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![3..3]);

            buffer
        });
    }

    #[gpui::test]
    fn test_random_concurrent_edits(ctx: &mut gpui::MutableAppContext) {
        use crate::test::Network;

        const PEERS: usize = 5;

        for seed in 0..100 {
            println!("{:?}", seed);
            let mut rng = &mut StdRng::seed_from_u64(seed);

            let base_text_len = rng.gen_range(0..10);
            let base_text = RandomCharIter::new(&mut rng)
                .take(base_text_len)
                .collect::<String>();
            let mut replica_ids = Vec::new();
            let mut buffers = Vec::new();
            let mut network = Network::new();
            for i in 0..PEERS {
                let buffer =
                    ctx.add_model(|ctx| Buffer::new(i as ReplicaId, base_text.as_str(), ctx));
                buffers.push(buffer);
                replica_ids.push(i as u16);
                network.add_peer(i as u16);
            }

            let mut mutation_count = 10;
            loop {
                let replica_index = rng.gen_range(0..PEERS);
                let replica_id = replica_ids[replica_index];
                buffers[replica_index].update(ctx, |buffer, _| match rng.gen_range(0..=100) {
                    0..=50 if mutation_count != 0 => {
                        let (_, _, ops) = buffer.randomly_mutate(&mut rng, None);
                        network.broadcast(replica_id, ops, &mut rng);
                        mutation_count -= 1;
                    }
                    51..=70 if mutation_count != 0 => {
                        let ops = buffer.randomly_undo_redo(&mut rng);
                        network.broadcast(replica_id, ops, &mut rng);
                        mutation_count -= 1;
                    }
                    71..=100 if network.has_unreceived(replica_id) => {
                        buffer
                            .apply_ops(network.receive(replica_id, &mut rng), None)
                            .unwrap();
                    }
                    _ => {}
                });

                if mutation_count == 0 && network.is_idle() {
                    break;
                }
            }

            let first_buffer = buffers[0].read(ctx);
            for buffer in &buffers[1..] {
                let buffer = buffer.read(ctx);
                assert_eq!(buffer.text(), first_buffer.text());
                assert_eq!(
                    buffer.all_selections().collect::<HashMap<_, _>>(),
                    first_buffer.all_selections().collect::<HashMap<_, _>>()
                );
                assert_eq!(
                    buffer.all_selection_ranges().collect::<HashMap<_, _>>(),
                    first_buffer
                        .all_selection_ranges()
                        .collect::<HashMap<_, _>>()
                );
            }
        }
    }

    impl Buffer {
        pub fn randomly_mutate<T>(
            &mut self,
            rng: &mut T,
            mut ctx: Option<&mut ModelContext<Self>>,
        ) -> (Vec<Range<usize>>, String, Vec<Operation>)
        where
            T: Rng,
        {
            // Randomly edit
            let (old_ranges, new_text, mut operations) =
                self.randomly_edit(rng, 5, ctx.as_deref_mut());

            // Randomly add, remove or mutate selection sets.
            let replica_selection_sets = &self
                .all_selections()
                .map(|(set_id, _)| *set_id)
                .filter(|set_id| self.replica_id == set_id.replica_id)
                .collect::<Vec<_>>();
            let set_id = replica_selection_sets.choose(rng);
            if set_id.is_some() && rng.gen_bool(1.0 / 6.0) {
                let op = self.remove_selection_set(*set_id.unwrap(), None).unwrap();
                operations.push(op);
            } else {
                let mut ranges = Vec::new();
                for _ in 0..5 {
                    let start = rng.gen_range(0..self.len() + 1);
                    let end = rng.gen_range(0..self.len() + 1);
                    ranges.push(start..end);
                }
                let new_selections = self.selections_from_ranges(ranges).unwrap();

                let op = if set_id.is_none() || rng.gen_bool(1.0 / 5.0) {
                    self.add_selection_set(new_selections, None).1
                } else {
                    self.update_selection_set(*set_id.unwrap(), new_selections, None)
                        .unwrap()
                };
                operations.push(op);
            }

            (old_ranges, new_text, operations)
        }

        pub fn randomly_undo_redo(&mut self, rng: &mut impl Rng) -> Vec<Operation> {
            let mut ops = Vec::new();
            for _ in 0..rng.gen_range(1..5) {
                if let Some(edit_id) = self.history.ops.keys().choose(rng).copied() {
                    ops.push(self.undo_or_redo(edit_id).unwrap());
                }
            }
            ops
        }

        fn selections_from_ranges<I>(&self, ranges: I) -> Result<Vec<Selection>>
        where
            I: IntoIterator<Item = Range<usize>>,
        {
            static NEXT_SELECTION_ID: AtomicUsize = AtomicUsize::new(0);

            let mut ranges = ranges.into_iter().collect::<Vec<_>>();
            ranges.sort_unstable_by_key(|range| range.start);

            let mut selections = Vec::with_capacity(ranges.len());
            for range in ranges {
                if range.start > range.end {
                    selections.push(Selection {
                        id: NEXT_SELECTION_ID.fetch_add(1, atomic::Ordering::SeqCst),
                        start: self.anchor_before(range.end)?,
                        end: self.anchor_before(range.start)?,
                        reversed: true,
                        goal: SelectionGoal::None,
                    });
                } else {
                    selections.push(Selection {
                        id: NEXT_SELECTION_ID.fetch_add(1, atomic::Ordering::SeqCst),
                        start: self.anchor_after(range.start)?,
                        end: self.anchor_before(range.end)?,
                        reversed: false,
                        goal: SelectionGoal::None,
                    });
                }
            }
            Ok(selections)
        }

        pub fn selection_ranges<'a>(&'a self, set_id: SelectionSetId) -> Result<Vec<Range<usize>>> {
            Ok(self
                .selections(set_id)?
                .iter()
                .map(move |selection| {
                    let start = selection.start.to_offset(self).unwrap();
                    let end = selection.end.to_offset(self).unwrap();
                    if selection.reversed {
                        end..start
                    } else {
                        start..end
                    }
                })
                .collect())
        }

        pub fn all_selections(&self) -> impl Iterator<Item = (&SelectionSetId, &[Selection])> {
            self.selections
                .iter()
                .map(|(set_id, selections)| (set_id, selections.as_ref()))
        }

        pub fn all_selection_ranges<'a>(
            &'a self,
        ) -> impl 'a + Iterator<Item = (SelectionSetId, Vec<Range<usize>>)> {
            self.selections
                .keys()
                .map(move |set_id| (*set_id, self.selection_ranges(*set_id).unwrap()))
        }
    }

    impl Operation {
        fn edit_id(&self) -> Option<time::Local> {
            match self {
                Operation::Edit { edit, .. } => Some(edit.id),
                Operation::Undo { undo, .. } => Some(undo.edit_id),
                Operation::UpdateSelections { .. } => None,
            }
        }
    }

    fn line_lengths_in_range(buffer: &Buffer, range: Range<usize>) -> BTreeMap<u32, HashSet<u32>> {
        let mut lengths = BTreeMap::new();
        for (row, line) in buffer.text()[range].lines().enumerate() {
            lengths
                .entry(line.len() as u32)
                .or_insert(HashSet::default())
                .insert(row as u32);
        }
        if lengths.is_empty() {
            let mut rows = HashSet::default();
            rows.insert(0);
            lengths.insert(0, rows);
        }
        lengths
    }
}
