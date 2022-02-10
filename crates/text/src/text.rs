mod anchor;
pub mod locator;
pub mod operation_queue;
mod patch;
mod point;
mod point_utf16;
#[cfg(any(test, feature = "test-support"))]
pub mod random_char_iter;
pub mod rope;
mod selection;
pub mod subscription;
#[cfg(test)]
mod tests;

pub use anchor::*;
use anyhow::Result;
use clock::ReplicaId;
use collections::{HashMap, HashSet};
use locator::Locator;
use operation_queue::OperationQueue;
pub use patch::Patch;
pub use point::*;
pub use point_utf16::*;
use postage::{oneshot, prelude::*};
#[cfg(any(test, feature = "test-support"))]
pub use random_char_iter::*;
use rope::TextDimension;
pub use rope::{Chunks, Rope, TextSummary};
pub use selection::*;
use std::{
    cmp::{self, Ordering},
    future::Future,
    iter::Iterator,
    ops::{self, Deref, Range, Sub},
    str,
    sync::Arc,
    time::{Duration, Instant},
};
pub use subscription::*;
pub use sum_tree::Bias;
use sum_tree::{FilterCursor, SumTree};

pub type TransactionId = clock::Local;

pub struct Buffer {
    snapshot: BufferSnapshot,
    history: History,
    deferred_ops: OperationQueue<Operation>,
    deferred_replicas: HashSet<ReplicaId>,
    replica_id: ReplicaId,
    remote_id: u64,
    local_clock: clock::Local,
    pub lamport_clock: clock::Lamport,
    subscriptions: Topic,
    edit_id_resolvers: HashMap<clock::Local, Vec<oneshot::Sender<()>>>,
}

#[derive(Clone, Debug)]
pub struct BufferSnapshot {
    replica_id: ReplicaId,
    visible_text: Rope,
    deleted_text: Rope,
    undo_map: UndoMap,
    fragments: SumTree<Fragment>,
    insertions: SumTree<InsertionFragment>,
    pub version: clock::Global,
}

#[derive(Clone, Debug)]
pub struct HistoryEntry {
    transaction: Transaction,
    first_edit_at: Instant,
    last_edit_at: Instant,
    suppress_grouping: bool,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: TransactionId,
    pub edit_ids: Vec<clock::Local>,
    pub start: clock::Global,
    pub end: clock::Global,
    pub ranges: Vec<Range<FullOffset>>,
}

impl HistoryEntry {
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction.id
    }

    fn push_edit(&mut self, edit: &EditOperation) {
        self.transaction.edit_ids.push(edit.timestamp.local());
        self.transaction.end.observe(edit.timestamp.local());

        let mut other_ranges = edit.ranges.iter().peekable();
        let mut new_ranges = Vec::new();
        let insertion_len = edit.new_text.as_ref().map_or(0, |t| t.len());
        let mut delta = 0;

        for mut self_range in self.transaction.ranges.iter().cloned() {
            self_range.start += delta;
            self_range.end += delta;

            while let Some(other_range) = other_ranges.peek() {
                let mut other_range = (*other_range).clone();
                other_range.start += delta;
                other_range.end += delta;

                if other_range.start <= self_range.end {
                    other_ranges.next().unwrap();
                    delta += insertion_len;

                    if other_range.end < self_range.start {
                        new_ranges.push(other_range.start..other_range.end + insertion_len);
                        self_range.start += insertion_len;
                        self_range.end += insertion_len;
                    } else {
                        self_range.start = cmp::min(self_range.start, other_range.start);
                        self_range.end = cmp::max(self_range.end, other_range.end) + insertion_len;
                    }
                } else {
                    break;
                }
            }

            new_ranges.push(self_range);
        }

        for other_range in other_ranges {
            new_ranges.push(other_range.start + delta..other_range.end + delta + insertion_len);
            delta += insertion_len;
        }

        self.transaction.ranges = new_ranges;
    }
}

#[derive(Clone)]
pub struct History {
    // TODO: Turn this into a String or Rope, maybe.
    pub base_text: Arc<str>,
    operations: HashMap<clock::Local, Operation>,
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    transaction_depth: usize,
    group_interval: Duration,
}

impl History {
    pub fn new(base_text: Arc<str>) -> Self {
        Self {
            base_text,
            operations: Default::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            transaction_depth: 0,
            group_interval: Duration::from_millis(300),
        }
    }

    fn push(&mut self, op: Operation) {
        self.operations.insert(op.local_timestamp(), op);
    }

    fn start_transaction(
        &mut self,
        start: clock::Global,
        now: Instant,
        local_clock: &mut clock::Local,
    ) -> Option<TransactionId> {
        self.transaction_depth += 1;
        if self.transaction_depth == 1 {
            let id = local_clock.tick();
            self.undo_stack.push(HistoryEntry {
                transaction: Transaction {
                    id,
                    start: start.clone(),
                    end: start,
                    edit_ids: Default::default(),
                    ranges: Default::default(),
                },
                first_edit_at: now,
                last_edit_at: now,
                suppress_grouping: false,
            });
            Some(id)
        } else {
            None
        }
    }

    fn end_transaction(&mut self, now: Instant) -> Option<&HistoryEntry> {
        assert_ne!(self.transaction_depth, 0);
        self.transaction_depth -= 1;
        if self.transaction_depth == 0 {
            if self
                .undo_stack
                .last()
                .unwrap()
                .transaction
                .ranges
                .is_empty()
            {
                self.undo_stack.pop();
                None
            } else {
                let entry = self.undo_stack.last_mut().unwrap();
                entry.last_edit_at = now;
                Some(entry)
            }
        } else {
            None
        }
    }

    fn group(&mut self) -> Option<TransactionId> {
        let mut new_len = self.undo_stack.len();
        let mut entries = self.undo_stack.iter_mut();

        if let Some(mut entry) = entries.next_back() {
            while let Some(prev_entry) = entries.next_back() {
                if !prev_entry.suppress_grouping
                    && entry.first_edit_at - prev_entry.last_edit_at <= self.group_interval
                    && entry.transaction.start == prev_entry.transaction.end
                {
                    entry = prev_entry;
                    new_len -= 1;
                } else {
                    break;
                }
            }
        }

        let (entries_to_keep, entries_to_merge) = self.undo_stack.split_at_mut(new_len);
        if let Some(last_entry) = entries_to_keep.last_mut() {
            for entry in &*entries_to_merge {
                for edit_id in &entry.transaction.edit_ids {
                    last_entry.push_edit(self.operations[edit_id].as_edit().unwrap());
                }
            }

            if let Some(entry) = entries_to_merge.last_mut() {
                last_entry.last_edit_at = entry.last_edit_at;
                last_entry.transaction.end = entry.transaction.end.clone();
            }
        }

        self.undo_stack.truncate(new_len);
        self.undo_stack.last().map(|e| e.transaction.id)
    }

    fn finalize_last_transaction(&mut self) -> Option<&Transaction> {
        self.undo_stack.last_mut().map(|entry| {
            entry.suppress_grouping = true;
            &entry.transaction
        })
    }

    fn push_transaction(&mut self, transaction: Transaction, now: Instant) {
        assert_eq!(self.transaction_depth, 0);
        self.undo_stack.push(HistoryEntry {
            transaction,
            first_edit_at: now,
            last_edit_at: now,
            suppress_grouping: false,
        });
    }

    fn push_undo(&mut self, op_id: clock::Local) {
        assert_ne!(self.transaction_depth, 0);
        if let Some(Operation::Edit(edit)) = self.operations.get(&op_id) {
            let last_transaction = self.undo_stack.last_mut().unwrap();
            last_transaction.push_edit(&edit);
        }
    }

    fn pop_undo(&mut self) -> Option<&HistoryEntry> {
        assert_eq!(self.transaction_depth, 0);
        if let Some(entry) = self.undo_stack.pop() {
            self.redo_stack.push(entry);
            self.redo_stack.last()
        } else {
            None
        }
    }

    fn remove_from_undo(&mut self, transaction_id: TransactionId) -> &[HistoryEntry] {
        assert_eq!(self.transaction_depth, 0);

        let redo_stack_start_len = self.redo_stack.len();
        if let Some(entry_ix) = self
            .undo_stack
            .iter()
            .rposition(|entry| entry.transaction.id == transaction_id)
        {
            self.redo_stack
                .extend(self.undo_stack.drain(entry_ix..).rev());
        }
        &self.redo_stack[redo_stack_start_len..]
    }

    fn forget(&mut self, transaction_id: TransactionId) {
        assert_eq!(self.transaction_depth, 0);
        if let Some(entry_ix) = self
            .undo_stack
            .iter()
            .rposition(|entry| entry.transaction.id == transaction_id)
        {
            self.undo_stack.remove(entry_ix);
        } else if let Some(entry_ix) = self
            .redo_stack
            .iter()
            .rposition(|entry| entry.transaction.id == transaction_id)
        {
            self.undo_stack.remove(entry_ix);
        }
    }

    fn pop_redo(&mut self) -> Option<&HistoryEntry> {
        assert_eq!(self.transaction_depth, 0);
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push(entry);
            self.undo_stack.last()
        } else {
            None
        }
    }

    fn remove_from_redo(&mut self, transaction_id: TransactionId) -> &[HistoryEntry] {
        assert_eq!(self.transaction_depth, 0);

        let undo_stack_start_len = self.undo_stack.len();
        if let Some(entry_ix) = self
            .redo_stack
            .iter()
            .rposition(|entry| entry.transaction.id == transaction_id)
        {
            self.undo_stack
                .extend(self.redo_stack.drain(entry_ix..).rev());
        }
        &self.undo_stack[undo_stack_start_len..]
    }
}

#[derive(Clone, Default, Debug)]
struct UndoMap(HashMap<clock::Local, Vec<(clock::Local, u32)>>);

impl UndoMap {
    fn insert(&mut self, undo: &UndoOperation) {
        for (edit_id, count) in &undo.counts {
            self.0.entry(*edit_id).or_default().push((undo.id, *count));
        }
    }

    fn is_undone(&self, edit_id: clock::Local) -> bool {
        self.undo_count(edit_id) % 2 == 1
    }

    fn was_undone(&self, edit_id: clock::Local, version: &clock::Global) -> bool {
        let undo_count = self
            .0
            .get(&edit_id)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|(undo_id, _)| version.observed(*undo_id))
            .map(|(_, undo_count)| *undo_count)
            .max()
            .unwrap_or(0);
        undo_count % 2 == 1
    }

    fn undo_count(&self, edit_id: clock::Local) -> u32 {
        self.0
            .get(&edit_id)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|(_, undo_count)| *undo_count)
            .max()
            .unwrap_or(0)
    }
}

struct Edits<'a, D: TextDimension, F: FnMut(&FragmentSummary) -> bool> {
    visible_cursor: rope::Cursor<'a>,
    deleted_cursor: rope::Cursor<'a>,
    fragments_cursor: Option<FilterCursor<'a, F, Fragment, FragmentTextSummary>>,
    undos: &'a UndoMap,
    since: &'a clock::Global,
    old_end: D,
    new_end: D,
    range: Range<(&'a Locator, usize)>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Edit<D> {
    pub old: Range<D>,
    pub new: Range<D>,
}

impl<D> Edit<D>
where
    D: Sub<D, Output = D> + PartialEq + Copy,
{
    pub fn old_len(&self) -> D {
        self.old.end - self.old.start
    }

    pub fn new_len(&self) -> D {
        self.new.end - self.new.start
    }

    pub fn is_empty(&self) -> bool {
        self.old.start == self.old.end && self.new.start == self.new.end
    }
}

impl<D1, D2> Edit<(D1, D2)> {
    pub fn flatten(self) -> (Edit<D1>, Edit<D2>) {
        (
            Edit {
                old: self.old.start.0..self.old.end.0,
                new: self.new.start.0..self.new.end.0,
            },
            Edit {
                old: self.old.start.1..self.old.end.1,
                new: self.new.start.1..self.new.end.1,
            },
        )
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct InsertionTimestamp {
    pub replica_id: ReplicaId,
    pub local: clock::Seq,
    pub lamport: clock::Seq,
}

impl InsertionTimestamp {
    pub fn local(&self) -> clock::Local {
        clock::Local {
            replica_id: self.replica_id,
            value: self.local,
        }
    }

    pub fn lamport(&self) -> clock::Lamport {
        clock::Lamport {
            replica_id: self.replica_id,
            value: self.lamport,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Fragment {
    pub id: Locator,
    pub insertion_timestamp: InsertionTimestamp,
    pub insertion_offset: usize,
    pub len: usize,
    pub visible: bool,
    pub deletions: HashSet<clock::Local>,
    pub max_undos: clock::Global,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FragmentSummary {
    text: FragmentTextSummary,
    max_id: Locator,
    max_version: clock::Global,
    min_insertion_version: clock::Global,
    max_insertion_version: clock::Global,
}

#[derive(Copy, Default, Clone, Debug, PartialEq, Eq)]
struct FragmentTextSummary {
    visible: usize,
    deleted: usize,
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FragmentTextSummary {
    fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<clock::Global>) {
        self.visible += summary.text.visible;
        self.deleted += summary.text.deleted;
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct InsertionFragment {
    timestamp: clock::Local,
    split_offset: usize,
    fragment_id: Locator,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct InsertionFragmentKey {
    timestamp: clock::Local,
    split_offset: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Edit(EditOperation),
    Undo {
        undo: UndoOperation,
        lamport_timestamp: clock::Lamport,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditOperation {
    pub timestamp: InsertionTimestamp,
    pub version: clock::Global,
    pub ranges: Vec<Range<FullOffset>>,
    pub new_text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UndoOperation {
    pub id: clock::Local,
    pub counts: HashMap<clock::Local, u32>,
    pub ranges: Vec<Range<FullOffset>>,
    pub version: clock::Global,
}

impl Buffer {
    pub fn new(replica_id: u16, remote_id: u64, history: History) -> Buffer {
        let mut fragments = SumTree::new();
        let mut insertions = SumTree::new();

        let mut local_clock = clock::Local::new(replica_id);
        let mut lamport_clock = clock::Lamport::new(replica_id);
        let mut version = clock::Global::new();
        let visible_text = Rope::from(history.base_text.as_ref());
        if visible_text.len() > 0 {
            let insertion_timestamp = InsertionTimestamp {
                replica_id: 0,
                local: 1,
                lamport: 1,
            };
            local_clock.observe(insertion_timestamp.local());
            lamport_clock.observe(insertion_timestamp.lamport());
            version.observe(insertion_timestamp.local());
            let fragment_id = Locator::between(&Locator::min(), &Locator::max());
            let fragment = Fragment {
                id: fragment_id,
                insertion_timestamp,
                insertion_offset: 0,
                len: visible_text.len(),
                visible: true,
                deletions: Default::default(),
                max_undos: Default::default(),
            };
            insertions.push(InsertionFragment::new(&fragment), &());
            fragments.push(fragment, &None);
        }

        Buffer {
            snapshot: BufferSnapshot {
                replica_id,
                visible_text,
                deleted_text: Rope::new(),
                fragments,
                insertions,
                version,
                undo_map: Default::default(),
            },
            history,
            deferred_ops: OperationQueue::new(),
            deferred_replicas: HashSet::default(),
            replica_id,
            remote_id,
            local_clock,
            lamport_clock,
            subscriptions: Default::default(),
            edit_id_resolvers: Default::default(),
        }
    }

    pub fn version(&self) -> clock::Global {
        self.version.clone()
    }

    pub fn snapshot(&self) -> BufferSnapshot {
        self.snapshot.clone()
    }

    pub fn replica_id(&self) -> ReplicaId {
        self.local_clock.replica_id
    }

    pub fn remote_id(&self) -> u64 {
        self.remote_id
    }

    pub fn deferred_ops_len(&self) -> usize {
        self.deferred_ops.len()
    }

    pub fn transaction_group_interval(&self) -> Duration {
        self.history.group_interval
    }

    pub fn edit<R, I, S, T>(&mut self, ranges: R, new_text: T) -> Operation
    where
        R: IntoIterator<IntoIter = I>,
        I: ExactSizeIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        let new_text = new_text.into();
        let new_text_len = new_text.len();
        let new_text = if new_text_len > 0 {
            Some(new_text)
        } else {
            None
        };

        self.start_transaction();
        let timestamp = InsertionTimestamp {
            replica_id: self.replica_id,
            local: self.local_clock.tick().value,
            lamport: self.lamport_clock.tick().value,
        };
        let operation =
            Operation::Edit(self.apply_local_edit(ranges.into_iter(), new_text, timestamp));

        self.history.push(operation.clone());
        self.history.push_undo(operation.local_timestamp());
        self.snapshot.version.observe(operation.local_timestamp());
        self.end_transaction();
        operation
    }

    fn apply_local_edit<S: ToOffset>(
        &mut self,
        ranges: impl ExactSizeIterator<Item = Range<S>>,
        new_text: Option<String>,
        timestamp: InsertionTimestamp,
    ) -> EditOperation {
        let mut edits = Patch::default();
        let mut edit_op = EditOperation {
            timestamp,
            version: self.version(),
            ranges: Vec::with_capacity(ranges.len()),
            new_text: None,
        };
        let mut new_insertions = Vec::new();
        let mut insertion_offset = 0;

        let mut ranges = ranges
            .map(|range| range.start.to_offset(&*self)..range.end.to_offset(&*self))
            .peekable();

        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));
        let mut old_fragments = self.fragments.cursor::<FragmentTextSummary>();
        let mut new_fragments =
            old_fragments.slice(&ranges.peek().unwrap().start, Bias::Right, &None);
        new_ropes.push_tree(new_fragments.summary().text);

        let mut fragment_start = old_fragments.start().visible;
        for range in ranges {
            let fragment_end = old_fragments.end(&None).visible;

            // If the current fragment ends before this range, then jump ahead to the first fragment
            // that extends past the start of this range, reusing any intervening fragments.
            if fragment_end < range.start {
                // If the current fragment has been partially consumed, then consume the rest of it
                // and advance to the next fragment before slicing.
                if fragment_start > old_fragments.start().visible {
                    if fragment_end > fragment_start {
                        let mut suffix = old_fragments.item().unwrap().clone();
                        suffix.len = fragment_end - fragment_start;
                        suffix.insertion_offset += fragment_start - old_fragments.start().visible;
                        new_insertions.push(InsertionFragment::insert_new(&suffix));
                        new_ropes.push_fragment(&suffix, suffix.visible);
                        new_fragments.push(suffix, &None);
                    }
                    old_fragments.next(&None);
                }

                let slice = old_fragments.slice(&range.start, Bias::Right, &None);
                new_ropes.push_tree(slice.summary().text);
                new_fragments.push_tree(slice, &None);
                fragment_start = old_fragments.start().visible;
            }

            let full_range_start = FullOffset(range.start + old_fragments.start().deleted);

            // Preserve any portion of the current fragment that precedes this range.
            if fragment_start < range.start {
                let mut prefix = old_fragments.item().unwrap().clone();
                prefix.len = range.start - fragment_start;
                prefix.insertion_offset += fragment_start - old_fragments.start().visible;
                prefix.id = Locator::between(&new_fragments.summary().max_id, &prefix.id);
                new_insertions.push(InsertionFragment::insert_new(&prefix));
                new_ropes.push_fragment(&prefix, prefix.visible);
                new_fragments.push(prefix, &None);
                fragment_start = range.start;
            }

            // Insert the new text before any existing fragments within the range.
            if let Some(new_text) = new_text.as_deref() {
                let new_start = new_fragments.summary().text.visible;
                edits.push(Edit {
                    old: fragment_start..fragment_start,
                    new: new_start..new_start + new_text.len(),
                });
                let fragment = Fragment {
                    id: Locator::between(
                        &new_fragments.summary().max_id,
                        old_fragments
                            .item()
                            .map_or(&Locator::max(), |old_fragment| &old_fragment.id),
                    ),
                    insertion_timestamp: timestamp,
                    insertion_offset,
                    len: new_text.len(),
                    deletions: Default::default(),
                    max_undos: Default::default(),
                    visible: true,
                };
                new_insertions.push(InsertionFragment::insert_new(&fragment));
                new_ropes.push_str(new_text);
                new_fragments.push(fragment, &None);
                insertion_offset += new_text.len();
            }

            // Advance through every fragment that intersects this range, marking the intersecting
            // portions as deleted.
            while fragment_start < range.end {
                let fragment = old_fragments.item().unwrap();
                let fragment_end = old_fragments.end(&None).visible;
                let mut intersection = fragment.clone();
                let intersection_end = cmp::min(range.end, fragment_end);
                if fragment.visible {
                    intersection.len = intersection_end - fragment_start;
                    intersection.insertion_offset += fragment_start - old_fragments.start().visible;
                    intersection.id =
                        Locator::between(&new_fragments.summary().max_id, &intersection.id);
                    intersection.deletions.insert(timestamp.local());
                    intersection.visible = false;
                }
                if intersection.len > 0 {
                    if fragment.visible && !intersection.visible {
                        let new_start = new_fragments.summary().text.visible;
                        edits.push(Edit {
                            old: fragment_start..intersection_end,
                            new: new_start..new_start,
                        });
                    }
                    new_insertions.push(InsertionFragment::insert_new(&intersection));
                    new_ropes.push_fragment(&intersection, fragment.visible);
                    new_fragments.push(intersection, &None);
                    fragment_start = intersection_end;
                }
                if fragment_end <= range.end {
                    old_fragments.next(&None);
                }
            }

            let full_range_end = FullOffset(range.end + old_fragments.start().deleted);
            edit_op.ranges.push(full_range_start..full_range_end);
        }

        // If the current fragment has been partially consumed, then consume the rest of it
        // and advance to the next fragment before slicing.
        if fragment_start > old_fragments.start().visible {
            let fragment_end = old_fragments.end(&None).visible;
            if fragment_end > fragment_start {
                let mut suffix = old_fragments.item().unwrap().clone();
                suffix.len = fragment_end - fragment_start;
                suffix.insertion_offset += fragment_start - old_fragments.start().visible;
                new_insertions.push(InsertionFragment::insert_new(&suffix));
                new_ropes.push_fragment(&suffix, suffix.visible);
                new_fragments.push(suffix, &None);
            }
            old_fragments.next(&None);
        }

        let suffix = old_fragments.suffix(&None);
        new_ropes.push_tree(suffix.summary().text);
        new_fragments.push_tree(suffix, &None);
        let (visible_text, deleted_text) = new_ropes.finish();
        drop(old_fragments);

        self.snapshot.fragments = new_fragments;
        self.snapshot.insertions.edit(new_insertions, &());
        self.snapshot.visible_text = visible_text;
        self.snapshot.deleted_text = deleted_text;
        self.subscriptions.publish_mut(&edits);
        edit_op.new_text = new_text;
        edit_op
    }

    pub fn apply_ops<I: IntoIterator<Item = Operation>>(&mut self, ops: I) -> Result<()> {
        let mut deferred_ops = Vec::new();
        for op in ops {
            self.history.push(op.clone());
            if self.can_apply_op(&op) {
                self.apply_op(op)?;
            } else {
                self.deferred_replicas.insert(op.replica_id());
                deferred_ops.push(op);
            }
        }
        self.deferred_ops.insert(deferred_ops);
        self.flush_deferred_ops()?;
        Ok(())
    }

    fn apply_op(&mut self, op: Operation) -> Result<()> {
        match op {
            Operation::Edit(edit) => {
                if !self.version.observed(edit.timestamp.local()) {
                    self.apply_remote_edit(
                        &edit.version,
                        &edit.ranges,
                        edit.new_text.as_deref(),
                        edit.timestamp,
                    );
                    self.snapshot.version.observe(edit.timestamp.local());
                    self.resolve_edit(edit.timestamp.local());
                }
            }
            Operation::Undo {
                undo,
                lamport_timestamp,
            } => {
                if !self.version.observed(undo.id) {
                    self.apply_undo(&undo)?;
                    self.snapshot.version.observe(undo.id);
                    self.lamport_clock.observe(lamport_timestamp);
                }
            }
        }
        Ok(())
    }

    fn apply_remote_edit(
        &mut self,
        version: &clock::Global,
        ranges: &[Range<FullOffset>],
        new_text: Option<&str>,
        timestamp: InsertionTimestamp,
    ) {
        if ranges.is_empty() {
            return;
        }

        let mut edits = Patch::default();
        let cx = Some(version.clone());
        let mut new_insertions = Vec::new();
        let mut insertion_offset = 0;
        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));
        let mut old_fragments = self.fragments.cursor::<(VersionedFullOffset, usize)>();
        let mut new_fragments = old_fragments.slice(
            &VersionedFullOffset::Offset(ranges[0].start),
            Bias::Left,
            &cx,
        );
        new_ropes.push_tree(new_fragments.summary().text);

        let mut fragment_start = old_fragments.start().0.full_offset();
        for range in ranges {
            let fragment_end = old_fragments.end(&cx).0.full_offset();

            // If the current fragment ends before this range, then jump ahead to the first fragment
            // that extends past the start of this range, reusing any intervening fragments.
            if fragment_end < range.start {
                // If the current fragment has been partially consumed, then consume the rest of it
                // and advance to the next fragment before slicing.
                if fragment_start > old_fragments.start().0.full_offset() {
                    if fragment_end > fragment_start {
                        let mut suffix = old_fragments.item().unwrap().clone();
                        suffix.len = fragment_end.0 - fragment_start.0;
                        suffix.insertion_offset +=
                            fragment_start - old_fragments.start().0.full_offset();
                        new_insertions.push(InsertionFragment::insert_new(&suffix));
                        new_ropes.push_fragment(&suffix, suffix.visible);
                        new_fragments.push(suffix, &None);
                    }
                    old_fragments.next(&cx);
                }

                let slice =
                    old_fragments.slice(&VersionedFullOffset::Offset(range.start), Bias::Left, &cx);
                new_ropes.push_tree(slice.summary().text);
                new_fragments.push_tree(slice, &None);
                fragment_start = old_fragments.start().0.full_offset();
            }

            // If we are at the end of a non-concurrent fragment, advance to the next one.
            let fragment_end = old_fragments.end(&cx).0.full_offset();
            if fragment_end == range.start && fragment_end > fragment_start {
                let mut fragment = old_fragments.item().unwrap().clone();
                fragment.len = fragment_end.0 - fragment_start.0;
                fragment.insertion_offset += fragment_start - old_fragments.start().0.full_offset();
                new_insertions.push(InsertionFragment::insert_new(&fragment));
                new_ropes.push_fragment(&fragment, fragment.visible);
                new_fragments.push(fragment, &None);
                old_fragments.next(&cx);
                fragment_start = old_fragments.start().0.full_offset();
            }

            // Skip over insertions that are concurrent to this edit, but have a lower lamport
            // timestamp.
            while let Some(fragment) = old_fragments.item() {
                if fragment_start == range.start
                    && fragment.insertion_timestamp.lamport() > timestamp.lamport()
                {
                    new_ropes.push_fragment(fragment, fragment.visible);
                    new_fragments.push(fragment.clone(), &None);
                    old_fragments.next(&cx);
                    debug_assert_eq!(fragment_start, range.start);
                } else {
                    break;
                }
            }
            debug_assert!(fragment_start <= range.start);

            // Preserve any portion of the current fragment that precedes this range.
            if fragment_start < range.start {
                let mut prefix = old_fragments.item().unwrap().clone();
                prefix.len = range.start.0 - fragment_start.0;
                prefix.insertion_offset += fragment_start - old_fragments.start().0.full_offset();
                prefix.id = Locator::between(&new_fragments.summary().max_id, &prefix.id);
                new_insertions.push(InsertionFragment::insert_new(&prefix));
                fragment_start = range.start;
                new_ropes.push_fragment(&prefix, prefix.visible);
                new_fragments.push(prefix, &None);
            }

            // Insert the new text before any existing fragments within the range.
            if let Some(new_text) = new_text {
                let mut old_start = old_fragments.start().1;
                if old_fragments.item().map_or(false, |f| f.visible) {
                    old_start += fragment_start.0 - old_fragments.start().0.full_offset().0;
                }
                let new_start = new_fragments.summary().text.visible;
                edits.push(Edit {
                    old: old_start..old_start,
                    new: new_start..new_start + new_text.len(),
                });
                let fragment = Fragment {
                    id: Locator::between(
                        &new_fragments.summary().max_id,
                        old_fragments
                            .item()
                            .map_or(&Locator::max(), |old_fragment| &old_fragment.id),
                    ),
                    insertion_timestamp: timestamp,
                    insertion_offset,
                    len: new_text.len(),
                    deletions: Default::default(),
                    max_undos: Default::default(),
                    visible: true,
                };
                new_insertions.push(InsertionFragment::insert_new(&fragment));
                new_ropes.push_str(new_text);
                new_fragments.push(fragment, &None);
                insertion_offset += new_text.len();
            }

            // Advance through every fragment that intersects this range, marking the intersecting
            // portions as deleted.
            while fragment_start < range.end {
                let fragment = old_fragments.item().unwrap();
                let fragment_end = old_fragments.end(&cx).0.full_offset();
                let mut intersection = fragment.clone();
                let intersection_end = cmp::min(range.end, fragment_end);
                if fragment.was_visible(version, &self.undo_map) {
                    intersection.len = intersection_end.0 - fragment_start.0;
                    intersection.insertion_offset +=
                        fragment_start - old_fragments.start().0.full_offset();
                    intersection.id =
                        Locator::between(&new_fragments.summary().max_id, &intersection.id);
                    intersection.deletions.insert(timestamp.local());
                    intersection.visible = false;
                }
                if intersection.len > 0 {
                    if fragment.visible && !intersection.visible {
                        let old_start = old_fragments.start().1
                            + (fragment_start.0 - old_fragments.start().0.full_offset().0);
                        let new_start = new_fragments.summary().text.visible;
                        edits.push(Edit {
                            old: old_start..old_start + intersection.len,
                            new: new_start..new_start,
                        });
                    }
                    new_insertions.push(InsertionFragment::insert_new(&intersection));
                    new_ropes.push_fragment(&intersection, fragment.visible);
                    new_fragments.push(intersection, &None);
                    fragment_start = intersection_end;
                }
                if fragment_end <= range.end {
                    old_fragments.next(&cx);
                }
            }
        }

        // If the current fragment has been partially consumed, then consume the rest of it
        // and advance to the next fragment before slicing.
        if fragment_start > old_fragments.start().0.full_offset() {
            let fragment_end = old_fragments.end(&cx).0.full_offset();
            if fragment_end > fragment_start {
                let mut suffix = old_fragments.item().unwrap().clone();
                suffix.len = fragment_end.0 - fragment_start.0;
                suffix.insertion_offset += fragment_start - old_fragments.start().0.full_offset();
                new_insertions.push(InsertionFragment::insert_new(&suffix));
                new_ropes.push_fragment(&suffix, suffix.visible);
                new_fragments.push(suffix, &None);
            }
            old_fragments.next(&cx);
        }

        let suffix = old_fragments.suffix(&cx);
        new_ropes.push_tree(suffix.summary().text);
        new_fragments.push_tree(suffix, &None);
        let (visible_text, deleted_text) = new_ropes.finish();
        drop(old_fragments);

        self.snapshot.fragments = new_fragments;
        self.snapshot.visible_text = visible_text;
        self.snapshot.deleted_text = deleted_text;
        self.snapshot.insertions.edit(new_insertions, &());
        self.local_clock.observe(timestamp.local());
        self.lamport_clock.observe(timestamp.lamport());
        self.subscriptions.publish_mut(&edits);
    }

    fn apply_undo(&mut self, undo: &UndoOperation) -> Result<()> {
        let mut edits = Patch::default();
        self.snapshot.undo_map.insert(undo);

        let mut cx = undo.version.clone();
        for edit_id in undo.counts.keys().copied() {
            cx.observe(edit_id);
        }
        let cx = Some(cx);

        let mut old_fragments = self.fragments.cursor::<(VersionedFullOffset, usize)>();
        let mut new_fragments = old_fragments.slice(
            &VersionedFullOffset::Offset(undo.ranges[0].start),
            Bias::Right,
            &cx,
        );
        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));
        new_ropes.push_tree(new_fragments.summary().text);

        for range in &undo.ranges {
            let mut end_offset = old_fragments.end(&cx).0.full_offset();

            if end_offset < range.start {
                let preceding_fragments = old_fragments.slice(
                    &VersionedFullOffset::Offset(range.start),
                    Bias::Right,
                    &cx,
                );
                new_ropes.push_tree(preceding_fragments.summary().text);
                new_fragments.push_tree(preceding_fragments, &None);
            }

            while end_offset <= range.end {
                if let Some(fragment) = old_fragments.item() {
                    let mut fragment = fragment.clone();
                    let fragment_was_visible = fragment.visible;

                    if fragment.was_visible(&undo.version, &self.undo_map)
                        || undo
                            .counts
                            .contains_key(&fragment.insertion_timestamp.local())
                    {
                        fragment.visible = fragment.is_visible(&self.undo_map);
                        fragment.max_undos.observe(undo.id);
                    }

                    let old_start = old_fragments.start().1;
                    let new_start = new_fragments.summary().text.visible;
                    if fragment_was_visible && !fragment.visible {
                        edits.push(Edit {
                            old: old_start..old_start + fragment.len,
                            new: new_start..new_start,
                        });
                    } else if !fragment_was_visible && fragment.visible {
                        edits.push(Edit {
                            old: old_start..old_start,
                            new: new_start..new_start + fragment.len,
                        });
                    }
                    new_ropes.push_fragment(&fragment, fragment_was_visible);
                    new_fragments.push(fragment, &None);

                    old_fragments.next(&cx);
                    if end_offset == old_fragments.end(&cx).0.full_offset() {
                        let unseen_fragments = old_fragments.slice(
                            &VersionedFullOffset::Offset(end_offset),
                            Bias::Right,
                            &cx,
                        );
                        new_ropes.push_tree(unseen_fragments.summary().text);
                        new_fragments.push_tree(unseen_fragments, &None);
                    }
                    end_offset = old_fragments.end(&cx).0.full_offset();
                } else {
                    break;
                }
            }
        }

        let suffix = old_fragments.suffix(&cx);
        new_ropes.push_tree(suffix.summary().text);
        new_fragments.push_tree(suffix, &None);

        drop(old_fragments);
        let (visible_text, deleted_text) = new_ropes.finish();
        self.snapshot.fragments = new_fragments;
        self.snapshot.visible_text = visible_text;
        self.snapshot.deleted_text = deleted_text;
        self.subscriptions.publish_mut(&edits);
        Ok(())
    }

    fn flush_deferred_ops(&mut self) -> Result<()> {
        self.deferred_replicas.clear();
        let mut deferred_ops = Vec::new();
        for op in self.deferred_ops.drain().iter().cloned() {
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
                Operation::Edit(edit) => self.version.observed_all(&edit.version),
                Operation::Undo { undo, .. } => self.version.observed_all(&undo.version),
            }
        }
    }

    pub fn peek_undo_stack(&self) -> Option<&HistoryEntry> {
        self.history.undo_stack.last()
    }

    pub fn peek_redo_stack(&self) -> Option<&HistoryEntry> {
        self.history.redo_stack.last()
    }

    pub fn start_transaction(&mut self) -> Option<TransactionId> {
        self.start_transaction_at(Instant::now())
    }

    pub fn start_transaction_at(&mut self, now: Instant) -> Option<TransactionId> {
        self.history
            .start_transaction(self.version.clone(), now, &mut self.local_clock)
    }

    pub fn end_transaction(&mut self) -> Option<(TransactionId, clock::Global)> {
        self.end_transaction_at(Instant::now())
    }

    pub fn end_transaction_at(&mut self, now: Instant) -> Option<(TransactionId, clock::Global)> {
        if let Some(entry) = self.history.end_transaction(now) {
            let since = entry.transaction.start.clone();
            let id = self.history.group().unwrap();
            Some((id, since))
        } else {
            None
        }
    }

    pub fn finalize_last_transaction(&mut self) -> Option<&Transaction> {
        self.history.finalize_last_transaction()
    }

    pub fn base_text(&self) -> &Arc<str> {
        &self.history.base_text
    }

    pub fn history(&self) -> impl Iterator<Item = &Operation> {
        self.history.operations.values()
    }

    pub fn undo_history(&self) -> impl Iterator<Item = (&clock::Local, &[(clock::Local, u32)])> {
        self.undo_map
            .0
            .iter()
            .map(|(edit_id, undo_counts)| (edit_id, undo_counts.as_slice()))
    }

    pub fn undo(&mut self) -> Option<(TransactionId, Operation)> {
        if let Some(entry) = self.history.pop_undo() {
            let transaction = entry.transaction.clone();
            let transaction_id = transaction.id;
            let op = self.undo_or_redo(transaction).unwrap();
            Some((transaction_id, op))
        } else {
            None
        }
    }

    pub fn undo_to_transaction(&mut self, transaction_id: TransactionId) -> Vec<Operation> {
        let transactions = self
            .history
            .remove_from_undo(transaction_id)
            .iter()
            .map(|entry| entry.transaction.clone())
            .collect::<Vec<_>>();

        transactions
            .into_iter()
            .map(|transaction| self.undo_or_redo(transaction).unwrap())
            .collect()
    }

    pub fn forget_transaction(&mut self, transaction_id: TransactionId) {
        self.history.forget(transaction_id);
    }

    pub fn redo(&mut self) -> Option<(TransactionId, Operation)> {
        if let Some(entry) = self.history.pop_redo() {
            let transaction = entry.transaction.clone();
            let transaction_id = transaction.id;
            let op = self.undo_or_redo(transaction).unwrap();
            Some((transaction_id, op))
        } else {
            None
        }
    }

    pub fn redo_to_transaction(&mut self, transaction_id: TransactionId) -> Vec<Operation> {
        let transactions = self
            .history
            .remove_from_redo(transaction_id)
            .iter()
            .map(|entry| entry.transaction.clone())
            .collect::<Vec<_>>();

        transactions
            .into_iter()
            .map(|transaction| self.undo_or_redo(transaction).unwrap())
            .collect()
    }

    fn undo_or_redo(&mut self, transaction: Transaction) -> Result<Operation> {
        let mut counts = HashMap::default();
        for edit_id in transaction.edit_ids {
            counts.insert(edit_id, self.undo_map.undo_count(edit_id) + 1);
        }

        let undo = UndoOperation {
            id: self.local_clock.tick(),
            counts,
            ranges: transaction.ranges,
            version: transaction.start.clone(),
        };
        self.apply_undo(&undo)?;
        let operation = Operation::Undo {
            undo,
            lamport_timestamp: self.lamport_clock.tick(),
        };
        self.snapshot.version.observe(operation.local_timestamp());
        self.history.push(operation.clone());
        Ok(operation)
    }

    pub fn push_transaction(&mut self, transaction: Transaction, now: Instant) {
        self.history.push_transaction(transaction, now);
        self.history.finalize_last_transaction();
    }

    pub fn subscribe(&mut self) -> Subscription {
        self.subscriptions.subscribe()
    }

    pub fn wait_for_edits(
        &mut self,
        edit_ids: impl IntoIterator<Item = clock::Local>,
    ) -> impl 'static + Future<Output = ()> {
        let mut futures = Vec::new();
        for edit_id in edit_ids {
            if !self.version.observed(edit_id) {
                let (tx, rx) = oneshot::channel();
                self.edit_id_resolvers.entry(edit_id).or_default().push(tx);
                futures.push(rx);
            }
        }

        async move {
            for mut future in futures {
                future.recv().await;
            }
        }
    }

    fn resolve_edit(&mut self, edit_id: clock::Local) {
        for mut tx in self
            .edit_id_resolvers
            .remove(&edit_id)
            .into_iter()
            .flatten()
        {
            let _ = tx.try_send(());
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Buffer {
    pub fn check_invariants(&self) {
        // Ensure every fragment is ordered by locator in the fragment tree and corresponds
        // to an insertion fragment in the insertions tree.
        let mut prev_fragment_id = Locator::min();
        for fragment in self.snapshot.fragments.items(&None) {
            assert!(fragment.id > prev_fragment_id);
            prev_fragment_id = fragment.id.clone();

            let insertion_fragment = self
                .snapshot
                .insertions
                .get(
                    &InsertionFragmentKey {
                        timestamp: fragment.insertion_timestamp.local(),
                        split_offset: fragment.insertion_offset,
                    },
                    &(),
                )
                .unwrap();
            assert_eq!(insertion_fragment.fragment_id, fragment.id);
        }

        let mut cursor = self.snapshot.fragments.cursor::<Option<&Locator>>();
        for insertion_fragment in self.snapshot.insertions.cursor::<()>() {
            cursor.seek(&Some(&insertion_fragment.fragment_id), Bias::Left, &None);
            let fragment = cursor.item().unwrap();
            assert_eq!(insertion_fragment.fragment_id, fragment.id);
            assert_eq!(insertion_fragment.split_offset, fragment.insertion_offset);
        }

        let fragment_summary = self.snapshot.fragments.summary();
        assert_eq!(
            fragment_summary.text.visible,
            self.snapshot.visible_text.len()
        );
        assert_eq!(
            fragment_summary.text.deleted,
            self.snapshot.deleted_text.len()
        );
    }

    pub fn set_group_interval(&mut self, group_interval: Duration) {
        self.history.group_interval = group_interval;
    }

    pub fn random_byte_range(&self, start_offset: usize, rng: &mut impl rand::Rng) -> Range<usize> {
        let end = self.clip_offset(rng.gen_range(start_offset..=self.len()), Bias::Right);
        let start = self.clip_offset(rng.gen_range(start_offset..=end), Bias::Right);
        start..end
    }

    pub fn randomly_edit<T>(
        &mut self,
        rng: &mut T,
        old_range_count: usize,
    ) -> (Vec<Range<usize>>, String, Operation)
    where
        T: rand::Rng,
    {
        let mut old_ranges: Vec<Range<usize>> = Vec::new();
        for _ in 0..old_range_count {
            let last_end = old_ranges.last().map_or(0, |last_range| last_range.end + 1);
            if last_end > self.len() {
                break;
            }
            old_ranges.push(self.random_byte_range(last_end, rng));
        }
        let new_text_len = rng.gen_range(0..10);
        let new_text: String = crate::random_char_iter::RandomCharIter::new(&mut *rng)
            .take(new_text_len)
            .collect();
        log::info!(
            "mutating buffer {} at {:?}: {:?}",
            self.replica_id,
            old_ranges,
            new_text
        );
        let op = self.edit(old_ranges.iter().cloned(), new_text.as_str());
        (old_ranges, new_text, op)
    }

    pub fn randomly_undo_redo(&mut self, rng: &mut impl rand::Rng) -> Vec<Operation> {
        use rand::prelude::*;

        let mut ops = Vec::new();
        for _ in 0..rng.gen_range(1..=5) {
            if let Some(entry) = self.history.undo_stack.choose(rng) {
                let transaction = entry.transaction.clone();
                log::info!(
                    "undoing buffer {} transaction {:?}",
                    self.replica_id,
                    transaction
                );
                ops.push(self.undo_or_redo(transaction).unwrap());
            }
        }
        ops
    }
}

impl Deref for Buffer {
    type Target = BufferSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl BufferSnapshot {
    pub fn as_rope(&self) -> &Rope {
        &self.visible_text
    }

    pub fn replica_id(&self) -> ReplicaId {
        self.replica_id
    }

    pub fn row_count(&self) -> u32 {
        self.max_point().row + 1
    }

    pub fn len(&self) -> usize {
        self.visible_text.len()
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars_at(0)
    }

    pub fn chars_for_range<T: ToOffset>(&self, range: Range<T>) -> impl Iterator<Item = char> + '_ {
        self.text_for_range(range).flat_map(str::chars)
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

    pub fn text(&self) -> String {
        self.visible_text.to_string()
    }

    pub fn deleted_text(&self) -> String {
        self.deleted_text.to_string()
    }

    pub fn fragments(&self) -> impl Iterator<Item = &Fragment> {
        self.fragments.iter()
    }

    pub fn text_summary(&self) -> TextSummary {
        self.visible_text.summary()
    }

    pub fn max_point(&self) -> Point {
        self.visible_text.max_point()
    }

    pub fn point_to_offset(&self, point: Point) -> usize {
        self.visible_text.point_to_offset(point)
    }

    pub fn point_utf16_to_offset(&self, point: PointUtf16) -> usize {
        self.visible_text.point_utf16_to_offset(point)
    }

    pub fn point_utf16_to_point(&self, point: PointUtf16) -> Point {
        self.visible_text.point_utf16_to_point(point)
    }

    pub fn offset_to_point(&self, offset: usize) -> Point {
        self.visible_text.offset_to_point(offset)
    }

    pub fn offset_to_point_utf16(&self, offset: usize) -> PointUtf16 {
        self.visible_text.offset_to_point_utf16(offset)
    }

    pub fn point_to_point_utf16(&self, point: Point) -> PointUtf16 {
        self.visible_text.point_to_point_utf16(point)
    }

    pub fn version(&self) -> &clock::Global {
        &self.version
    }

    pub fn chars_at<'a, T: ToOffset>(&'a self, position: T) -> impl Iterator<Item = char> + 'a {
        let offset = position.to_offset(self);
        self.visible_text.chars_at(offset)
    }

    pub fn reversed_chars_at<'a, T: ToOffset>(
        &'a self,
        position: T,
    ) -> impl Iterator<Item = char> + 'a {
        let offset = position.to_offset(self);
        self.visible_text.reversed_chars_at(offset)
    }

    pub fn reversed_chunks_in_range<T: ToOffset>(&self, range: Range<T>) -> rope::Chunks {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.visible_text.reversed_chunks_in_range(range)
    }

    pub fn bytes_in_range<'a, T: ToOffset>(&'a self, range: Range<T>) -> rope::Bytes<'a> {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);
        self.visible_text.bytes_in_range(start..end)
    }

    pub fn text_for_range<'a, T: ToOffset>(&'a self, range: Range<T>) -> Chunks<'a> {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);
        self.visible_text.chunks_in_range(start..end)
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let row_start_offset = Point::new(row, 0).to_offset(self);
        let row_end_offset = if row >= self.max_point().row {
            self.len()
        } else {
            Point::new(row + 1, 0).to_offset(self) - 1
        };
        (row_end_offset - row_start_offset) as u32
    }

    pub fn is_line_blank(&self, row: u32) -> bool {
        self.text_for_range(Point::new(row, 0)..Point::new(row, self.line_len(row)))
            .all(|chunk| chunk.matches(|c: char| !c.is_whitespace()).next().is_none())
    }

    pub fn indent_column_for_line(&self, row: u32) -> u32 {
        let mut result = 0;
        for c in self.chars_at(Point::new(row, 0)) {
            if c == ' ' {
                result += 1;
            } else {
                break;
            }
        }
        result
    }

    pub fn text_summary_for_range<'a, D, O: ToOffset>(&'a self, range: Range<O>) -> D
    where
        D: TextDimension,
    {
        self.visible_text
            .cursor(range.start.to_offset(self))
            .summary(range.end.to_offset(self))
    }

    pub fn summaries_for_anchors<'a, D, A>(&'a self, anchors: A) -> impl 'a + Iterator<Item = D>
    where
        D: 'a + TextDimension,
        A: 'a + IntoIterator<Item = &'a Anchor>,
    {
        let anchors = anchors.into_iter();
        let mut insertion_cursor = self.insertions.cursor::<InsertionFragmentKey>();
        let mut fragment_cursor = self.fragments.cursor::<(Option<&Locator>, usize)>();
        let mut text_cursor = self.visible_text.cursor(0);
        let mut position = D::default();

        anchors.map(move |anchor| {
            if *anchor == Anchor::min() {
                return D::default();
            } else if *anchor == Anchor::max() {
                return D::from_text_summary(&self.visible_text.summary());
            }

            let anchor_key = InsertionFragmentKey {
                timestamp: anchor.timestamp,
                split_offset: anchor.offset,
            };
            insertion_cursor.seek(&anchor_key, anchor.bias, &());
            if let Some(insertion) = insertion_cursor.item() {
                let comparison = sum_tree::KeyedItem::key(insertion).cmp(&anchor_key);
                if comparison == Ordering::Greater
                    || (anchor.bias == Bias::Left
                        && comparison == Ordering::Equal
                        && anchor.offset > 0)
                {
                    insertion_cursor.prev(&());
                }
            } else {
                insertion_cursor.prev(&());
            }
            let insertion = insertion_cursor.item().expect("invalid insertion");
            assert_eq!(insertion.timestamp, anchor.timestamp, "invalid insertion");

            fragment_cursor.seek_forward(&Some(&insertion.fragment_id), Bias::Left, &None);
            let fragment = fragment_cursor.item().unwrap();
            let mut fragment_offset = fragment_cursor.start().1;
            if fragment.visible {
                fragment_offset += anchor.offset - insertion.split_offset;
            }

            position.add_assign(&text_cursor.summary(fragment_offset));
            position.clone()
        })
    }

    fn summary_for_anchor<'a, D>(&'a self, anchor: &Anchor) -> D
    where
        D: TextDimension,
    {
        if *anchor == Anchor::min() {
            D::default()
        } else if *anchor == Anchor::max() {
            D::from_text_summary(&self.visible_text.summary())
        } else {
            let anchor_key = InsertionFragmentKey {
                timestamp: anchor.timestamp,
                split_offset: anchor.offset,
            };
            let mut insertion_cursor = self.insertions.cursor::<InsertionFragmentKey>();
            insertion_cursor.seek(&anchor_key, anchor.bias, &());
            if let Some(insertion) = insertion_cursor.item() {
                let comparison = sum_tree::KeyedItem::key(insertion).cmp(&anchor_key);
                if comparison == Ordering::Greater
                    || (anchor.bias == Bias::Left
                        && comparison == Ordering::Equal
                        && anchor.offset > 0)
                {
                    insertion_cursor.prev(&());
                }
            } else {
                insertion_cursor.prev(&());
            }
            let insertion = insertion_cursor.item().expect("invalid insertion");
            assert_eq!(insertion.timestamp, anchor.timestamp, "invalid insertion");

            let mut fragment_cursor = self.fragments.cursor::<(Option<&Locator>, usize)>();
            fragment_cursor.seek(&Some(&insertion.fragment_id), Bias::Left, &None);
            let fragment = fragment_cursor.item().unwrap();
            let mut fragment_offset = fragment_cursor.start().1;
            if fragment.visible {
                fragment_offset += anchor.offset - insertion.split_offset;
            }
            self.text_summary_for_range(0..fragment_offset)
        }
    }

    fn fragment_id_for_anchor(&self, anchor: &Anchor) -> &Locator {
        if *anchor == Anchor::min() {
            &locator::MIN
        } else if *anchor == Anchor::max() {
            &locator::MAX
        } else {
            let anchor_key = InsertionFragmentKey {
                timestamp: anchor.timestamp,
                split_offset: anchor.offset,
            };
            let mut insertion_cursor = self.insertions.cursor::<InsertionFragmentKey>();
            insertion_cursor.seek(&anchor_key, anchor.bias, &());
            if let Some(insertion) = insertion_cursor.item() {
                let comparison = sum_tree::KeyedItem::key(insertion).cmp(&anchor_key);
                if comparison == Ordering::Greater
                    || (anchor.bias == Bias::Left
                        && comparison == Ordering::Equal
                        && anchor.offset > 0)
                {
                    insertion_cursor.prev(&());
                }
            } else {
                insertion_cursor.prev(&());
            }
            let insertion = insertion_cursor.item().expect("invalid insertion");
            debug_assert_eq!(insertion.timestamp, anchor.timestamp, "invalid insertion");
            &insertion.fragment_id
        }
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        let offset = position.to_offset(self);
        if bias == Bias::Left && offset == 0 {
            Anchor::min()
        } else if bias == Bias::Right && offset == self.len() {
            Anchor::max()
        } else {
            let mut fragment_cursor = self.fragments.cursor::<usize>();
            fragment_cursor.seek(&offset, bias, &None);
            let fragment = fragment_cursor.item().unwrap();
            let overshoot = offset - *fragment_cursor.start();
            Anchor {
                timestamp: fragment.insertion_timestamp.local(),
                offset: fragment.insertion_offset + overshoot,
                bias,
            }
        }
    }

    pub fn can_resolve(&self, anchor: &Anchor) -> bool {
        *anchor == Anchor::min()
            || *anchor == Anchor::max()
            || self.version.observed(anchor.timestamp)
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        self.visible_text.clip_offset(offset, bias)
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.visible_text.clip_point(point, bias)
    }

    pub fn clip_point_utf16(&self, point: PointUtf16, bias: Bias) -> PointUtf16 {
        self.visible_text.clip_point_utf16(point, bias)
    }

    pub fn edits_since<'a, D>(
        &'a self,
        since: &'a clock::Global,
    ) -> impl 'a + Iterator<Item = Edit<D>>
    where
        D: TextDimension + Ord,
    {
        self.edits_since_in_range(since, Anchor::min()..Anchor::max())
    }

    pub fn edited_ranges_for_transaction<'a, D>(
        &'a self,
        transaction: &'a Transaction,
    ) -> impl 'a + Iterator<Item = Range<D>>
    where
        D: TextDimension,
    {
        let mut cursor = self.fragments.cursor::<(VersionedFullOffset, usize)>();
        let mut rope_cursor = self.visible_text.cursor(0);
        let cx = Some(transaction.end.clone());
        let mut position = D::default();
        transaction.ranges.iter().map(move |range| {
            cursor.seek_forward(&VersionedFullOffset::Offset(range.start), Bias::Right, &cx);
            let mut start_offset = cursor.start().1;
            if cursor
                .item()
                .map_or(false, |fragment| fragment.is_visible(&self.undo_map))
            {
                start_offset += range.start - cursor.start().0.full_offset()
            }
            position.add_assign(&rope_cursor.summary(start_offset));
            let start = position.clone();

            cursor.seek_forward(&VersionedFullOffset::Offset(range.end), Bias::Left, &cx);
            let mut end_offset = cursor.start().1;
            if cursor
                .item()
                .map_or(false, |fragment| fragment.is_visible(&self.undo_map))
            {
                end_offset += range.end - cursor.start().0.full_offset();
            }
            position.add_assign(&rope_cursor.summary(end_offset));
            start..position.clone()
        })
    }

    pub fn edits_since_in_range<'a, D>(
        &'a self,
        since: &'a clock::Global,
        range: Range<Anchor>,
    ) -> impl 'a + Iterator<Item = Edit<D>>
    where
        D: TextDimension + Ord,
    {
        let fragments_cursor = if *since == self.version {
            None
        } else {
            Some(self.fragments.filter(
                move |summary| !since.observed_all(&summary.max_version),
                &None,
            ))
        };
        let mut cursor = self
            .fragments
            .cursor::<(Option<&Locator>, FragmentTextSummary)>();

        let start_fragment_id = self.fragment_id_for_anchor(&range.start);
        cursor.seek(&Some(start_fragment_id), Bias::Left, &None);
        let mut visible_start = cursor.start().1.visible;
        let mut deleted_start = cursor.start().1.deleted;
        if let Some(fragment) = cursor.item() {
            let overshoot = range.start.offset - fragment.insertion_offset;
            if fragment.visible {
                visible_start += overshoot;
            } else {
                deleted_start += overshoot;
            }
        }
        let end_fragment_id = self.fragment_id_for_anchor(&range.end);

        Edits {
            visible_cursor: self.visible_text.cursor(visible_start),
            deleted_cursor: self.deleted_text.cursor(deleted_start),
            fragments_cursor,
            undos: &self.undo_map,
            since,
            old_end: Default::default(),
            new_end: Default::default(),
            range: (start_fragment_id, range.start.offset)..(end_fragment_id, range.end.offset),
        }
    }
}

struct RopeBuilder<'a> {
    old_visible_cursor: rope::Cursor<'a>,
    old_deleted_cursor: rope::Cursor<'a>,
    new_visible: Rope,
    new_deleted: Rope,
}

impl<'a> RopeBuilder<'a> {
    fn new(old_visible_cursor: rope::Cursor<'a>, old_deleted_cursor: rope::Cursor<'a>) -> Self {
        Self {
            old_visible_cursor,
            old_deleted_cursor,
            new_visible: Rope::new(),
            new_deleted: Rope::new(),
        }
    }

    fn push_tree(&mut self, len: FragmentTextSummary) {
        self.push(len.visible, true, true);
        self.push(len.deleted, false, false);
    }

    fn push_fragment(&mut self, fragment: &Fragment, was_visible: bool) {
        debug_assert!(fragment.len > 0);
        self.push(fragment.len, was_visible, fragment.visible)
    }

    fn push(&mut self, len: usize, was_visible: bool, is_visible: bool) {
        let text = if was_visible {
            self.old_visible_cursor
                .slice(self.old_visible_cursor.offset() + len)
        } else {
            self.old_deleted_cursor
                .slice(self.old_deleted_cursor.offset() + len)
        };
        if is_visible {
            self.new_visible.append(text);
        } else {
            self.new_deleted.append(text);
        }
    }

    fn push_str(&mut self, text: &str) {
        self.new_visible.push(text);
    }

    fn finish(mut self) -> (Rope, Rope) {
        self.new_visible.append(self.old_visible_cursor.suffix());
        self.new_deleted.append(self.old_deleted_cursor.suffix());
        (self.new_visible, self.new_deleted)
    }
}

impl<'a, D: TextDimension + Ord, F: FnMut(&FragmentSummary) -> bool> Iterator for Edits<'a, D, F> {
    type Item = Edit<D>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pending_edit: Option<Edit<D>> = None;
        let cursor = self.fragments_cursor.as_mut()?;

        while let Some(fragment) = cursor.item() {
            if fragment.id < *self.range.start.0 {
                cursor.next(&None);
                continue;
            } else if fragment.id > *self.range.end.0 {
                break;
            }

            if cursor.start().visible > self.visible_cursor.offset() {
                let summary = self.visible_cursor.summary(cursor.start().visible);
                self.old_end.add_assign(&summary);
                self.new_end.add_assign(&summary);
            }

            if pending_edit
                .as_ref()
                .map_or(false, |change| change.new.end < self.new_end)
            {
                break;
            }

            if !fragment.was_visible(&self.since, &self.undos) && fragment.visible {
                let mut visible_end = cursor.end(&None).visible;
                if fragment.id == *self.range.end.0 {
                    visible_end = cmp::min(
                        visible_end,
                        cursor.start().visible + (self.range.end.1 - fragment.insertion_offset),
                    );
                }

                let fragment_summary = self.visible_cursor.summary(visible_end);
                let mut new_end = self.new_end.clone();
                new_end.add_assign(&fragment_summary);
                if let Some(pending_edit) = pending_edit.as_mut() {
                    pending_edit.new.end = new_end.clone();
                } else {
                    pending_edit = Some(Edit {
                        old: self.old_end.clone()..self.old_end.clone(),
                        new: self.new_end.clone()..new_end.clone(),
                    });
                }

                self.new_end = new_end;
            } else if fragment.was_visible(&self.since, &self.undos) && !fragment.visible {
                let mut deleted_end = cursor.end(&None).deleted;
                if fragment.id == *self.range.end.0 {
                    deleted_end = cmp::min(
                        deleted_end,
                        cursor.start().deleted + (self.range.end.1 - fragment.insertion_offset),
                    );
                }

                if cursor.start().deleted > self.deleted_cursor.offset() {
                    self.deleted_cursor.seek_forward(cursor.start().deleted);
                }
                let fragment_summary = self.deleted_cursor.summary(deleted_end);
                let mut old_end = self.old_end.clone();
                old_end.add_assign(&fragment_summary);
                if let Some(pending_edit) = pending_edit.as_mut() {
                    pending_edit.old.end = old_end.clone();
                } else {
                    pending_edit = Some(Edit {
                        old: self.old_end.clone()..old_end.clone(),
                        new: self.new_end.clone()..self.new_end.clone(),
                    });
                }

                self.old_end = old_end;
            }

            cursor.next(&None);
        }

        pending_edit
    }
}

impl Fragment {
    fn is_visible(&self, undos: &UndoMap) -> bool {
        !undos.is_undone(self.insertion_timestamp.local())
            && self.deletions.iter().all(|d| undos.is_undone(*d))
    }

    fn was_visible(&self, version: &clock::Global, undos: &UndoMap) -> bool {
        (version.observed(self.insertion_timestamp.local())
            && !undos.was_undone(self.insertion_timestamp.local(), version))
            && self
                .deletions
                .iter()
                .all(|d| !version.observed(*d) || undos.was_undone(*d, version))
    }
}

impl sum_tree::Item for Fragment {
    type Summary = FragmentSummary;

    fn summary(&self) -> Self::Summary {
        let mut max_version = clock::Global::new();
        max_version.observe(self.insertion_timestamp.local());
        for deletion in &self.deletions {
            max_version.observe(*deletion);
        }
        max_version.join(&self.max_undos);

        let mut min_insertion_version = clock::Global::new();
        min_insertion_version.observe(self.insertion_timestamp.local());
        let max_insertion_version = min_insertion_version.clone();
        if self.visible {
            FragmentSummary {
                max_id: self.id.clone(),
                text: FragmentTextSummary {
                    visible: self.len,
                    deleted: 0,
                },
                max_version,
                min_insertion_version,
                max_insertion_version,
            }
        } else {
            FragmentSummary {
                max_id: self.id.clone(),
                text: FragmentTextSummary {
                    visible: 0,
                    deleted: self.len,
                },
                max_version,
                min_insertion_version,
                max_insertion_version,
            }
        }
    }
}

impl sum_tree::Summary for FragmentSummary {
    type Context = Option<clock::Global>;

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.max_id.assign(&other.max_id);
        self.text.visible += &other.text.visible;
        self.text.deleted += &other.text.deleted;
        self.max_version.join(&other.max_version);
        self.min_insertion_version
            .meet(&other.min_insertion_version);
        self.max_insertion_version
            .join(&other.max_insertion_version);
    }
}

impl Default for FragmentSummary {
    fn default() -> Self {
        FragmentSummary {
            max_id: Locator::min(),
            text: FragmentTextSummary::default(),
            max_version: clock::Global::new(),
            min_insertion_version: clock::Global::new(),
            max_insertion_version: clock::Global::new(),
        }
    }
}

impl sum_tree::Item for InsertionFragment {
    type Summary = InsertionFragmentKey;

    fn summary(&self) -> Self::Summary {
        InsertionFragmentKey {
            timestamp: self.timestamp,
            split_offset: self.split_offset,
        }
    }
}

impl sum_tree::KeyedItem for InsertionFragment {
    type Key = InsertionFragmentKey;

    fn key(&self) -> Self::Key {
        sum_tree::Item::summary(self)
    }
}

impl InsertionFragment {
    fn new(fragment: &Fragment) -> Self {
        Self {
            timestamp: fragment.insertion_timestamp.local(),
            split_offset: fragment.insertion_offset,
            fragment_id: fragment.id.clone(),
        }
    }

    fn insert_new(fragment: &Fragment) -> sum_tree::Edit<Self> {
        sum_tree::Edit::Insert(Self::new(fragment))
    }
}

impl sum_tree::Summary for InsertionFragmentKey {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        *self = *summary;
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FullOffset(pub usize);

impl ops::AddAssign<usize> for FullOffset {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl ops::Add<usize> for FullOffset {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::Sub for FullOffset {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for usize {
    fn add_summary(&mut self, summary: &FragmentSummary, _: &Option<clock::Global>) {
        *self += summary.text.visible;
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FullOffset {
    fn add_summary(&mut self, summary: &FragmentSummary, _: &Option<clock::Global>) {
        self.0 += summary.text.visible + summary.text.deleted;
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for Option<&'a Locator> {
    fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<clock::Global>) {
        *self = Some(&summary.max_id);
    }
}

impl<'a> sum_tree::SeekTarget<'a, FragmentSummary, FragmentTextSummary> for usize {
    fn cmp(
        &self,
        cursor_location: &FragmentTextSummary,
        _: &Option<clock::Global>,
    ) -> cmp::Ordering {
        Ord::cmp(self, &cursor_location.visible)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum VersionedFullOffset {
    Offset(FullOffset),
    Invalid,
}

impl VersionedFullOffset {
    fn full_offset(&self) -> FullOffset {
        if let Self::Offset(position) = self {
            *position
        } else {
            panic!("invalid version")
        }
    }
}

impl Default for VersionedFullOffset {
    fn default() -> Self {
        Self::Offset(Default::default())
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for VersionedFullOffset {
    fn add_summary(&mut self, summary: &'a FragmentSummary, cx: &Option<clock::Global>) {
        if let Self::Offset(offset) = self {
            let version = cx.as_ref().unwrap();
            if version.observed_all(&summary.max_insertion_version) {
                *offset += summary.text.visible + summary.text.deleted;
            } else if version.observed_any(&summary.min_insertion_version) {
                *self = Self::Invalid;
            }
        }
    }
}

impl<'a> sum_tree::SeekTarget<'a, FragmentSummary, Self> for VersionedFullOffset {
    fn cmp(&self, cursor_position: &Self, _: &Option<clock::Global>) -> cmp::Ordering {
        match (self, cursor_position) {
            (Self::Offset(a), Self::Offset(b)) => Ord::cmp(a, b),
            (Self::Offset(_), Self::Invalid) => cmp::Ordering::Less,
            (Self::Invalid, _) => unreachable!(),
        }
    }
}

impl Operation {
    fn replica_id(&self) -> ReplicaId {
        operation_queue::Operation::lamport_timestamp(self).replica_id
    }

    pub fn local_timestamp(&self) -> clock::Local {
        match self {
            Operation::Edit(edit) => edit.timestamp.local(),
            Operation::Undo { undo, .. } => undo.id,
        }
    }

    pub fn as_edit(&self) -> Option<&EditOperation> {
        match self {
            Operation::Edit(edit) => Some(edit),
            _ => None,
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
    fn lamport_timestamp(&self) -> clock::Lamport {
        match self {
            Operation::Edit(edit) => edit.timestamp.lamport(),
            Operation::Undo {
                lamport_timestamp, ..
            } => *lamport_timestamp,
        }
    }
}

pub trait ToOffset {
    fn to_offset<'a>(&self, snapshot: &BufferSnapshot) -> usize;
}

impl ToOffset for Point {
    fn to_offset<'a>(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot.point_to_offset(*self)
    }
}

impl ToOffset for PointUtf16 {
    fn to_offset<'a>(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot.point_utf16_to_offset(*self)
    }
}

impl ToOffset for usize {
    fn to_offset<'a>(&self, snapshot: &BufferSnapshot) -> usize {
        assert!(*self <= snapshot.len(), "offset is out of range");
        *self
    }
}

impl ToOffset for Anchor {
    fn to_offset<'a>(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot.summary_for_anchor(self)
    }
}

impl<'a, T: ToOffset> ToOffset for &'a T {
    fn to_offset(&self, content: &BufferSnapshot) -> usize {
        (*self).to_offset(content)
    }
}

pub trait ToPoint {
    fn to_point<'a>(&self, snapshot: &BufferSnapshot) -> Point;
}

impl ToPoint for Anchor {
    fn to_point<'a>(&self, snapshot: &BufferSnapshot) -> Point {
        snapshot.summary_for_anchor(self)
    }
}

impl ToPoint for usize {
    fn to_point<'a>(&self, snapshot: &BufferSnapshot) -> Point {
        snapshot.offset_to_point(*self)
    }
}

impl ToPoint for PointUtf16 {
    fn to_point<'a>(&self, snapshot: &BufferSnapshot) -> Point {
        snapshot.point_utf16_to_point(*self)
    }
}

impl ToPoint for Point {
    fn to_point<'a>(&self, _: &BufferSnapshot) -> Point {
        *self
    }
}

pub trait ToPointUtf16 {
    fn to_point_utf16<'a>(&self, snapshot: &BufferSnapshot) -> PointUtf16;
}

impl ToPointUtf16 for Anchor {
    fn to_point_utf16<'a>(&self, snapshot: &BufferSnapshot) -> PointUtf16 {
        snapshot.summary_for_anchor(self)
    }
}

impl ToPointUtf16 for usize {
    fn to_point_utf16<'a>(&self, snapshot: &BufferSnapshot) -> PointUtf16 {
        snapshot.offset_to_point_utf16(*self)
    }
}

impl ToPointUtf16 for PointUtf16 {
    fn to_point_utf16<'a>(&self, _: &BufferSnapshot) -> PointUtf16 {
        *self
    }
}

impl ToPointUtf16 for Point {
    fn to_point_utf16<'a>(&self, snapshot: &BufferSnapshot) -> PointUtf16 {
        snapshot.point_to_point_utf16(*self)
    }
}

pub trait Clip {
    fn clip(&self, bias: Bias, snapshot: &BufferSnapshot) -> Self;
}

impl Clip for usize {
    fn clip(&self, bias: Bias, snapshot: &BufferSnapshot) -> Self {
        snapshot.clip_offset(*self, bias)
    }
}

impl Clip for Point {
    fn clip(&self, bias: Bias, snapshot: &BufferSnapshot) -> Self {
        snapshot.clip_point(*self, bias)
    }
}

impl Clip for PointUtf16 {
    fn clip(&self, bias: Bias, snapshot: &BufferSnapshot) -> Self {
        snapshot.clip_point_utf16(*self, bias)
    }
}

pub trait FromAnchor {
    fn from_anchor(anchor: &Anchor, snapshot: &BufferSnapshot) -> Self;
}

impl FromAnchor for Point {
    fn from_anchor(anchor: &Anchor, snapshot: &BufferSnapshot) -> Self {
        snapshot.summary_for_anchor(anchor)
    }
}

impl FromAnchor for PointUtf16 {
    fn from_anchor(anchor: &Anchor, snapshot: &BufferSnapshot) -> Self {
        snapshot.summary_for_anchor(anchor)
    }
}

impl FromAnchor for usize {
    fn from_anchor(anchor: &Anchor, snapshot: &BufferSnapshot) -> Self {
        snapshot.summary_for_anchor(anchor)
    }
}
