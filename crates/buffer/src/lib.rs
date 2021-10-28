mod anchor;
mod operation_queue;
mod point;
#[cfg(any(test, feature = "test-support"))]
pub mod random_char_iter;
pub mod rope;
mod selection;
#[cfg(test)]
mod tests;

pub use anchor::*;
use anyhow::{anyhow, Result};
use clock::ReplicaId;
use operation_queue::OperationQueue;
pub use point::*;
#[cfg(any(test, feature = "test-support"))]
pub use random_char_iter::*;
pub use rope::{Chunks, Rope, TextSummary};
use rpc::proto;
pub use selection::*;
use std::{
    cmp,
    convert::TryFrom,
    iter::Iterator,
    ops::Range,
    str,
    sync::Arc,
    time::{Duration, Instant},
};
pub use sum_tree::Bias;
use sum_tree::{FilterCursor, SumTree};

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Default)]
struct DeterministicState;

#[cfg(any(test, feature = "test-support"))]
impl std::hash::BuildHasher for DeterministicState {
    type Hasher = seahash::SeaHasher;

    fn build_hasher(&self) -> Self::Hasher {
        seahash::SeaHasher::new()
    }
}

#[cfg(any(test, feature = "test-support"))]
type HashMap<K, V> = std::collections::HashMap<K, V, DeterministicState>;

#[cfg(any(test, feature = "test-support"))]
type HashSet<T> = std::collections::HashSet<T, DeterministicState>;

#[cfg(not(any(test, feature = "test-support")))]
type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(any(test, feature = "test-support")))]
type HashSet<T> = std::collections::HashSet<T>;

#[derive(Clone)]
pub struct Buffer {
    fragments: SumTree<Fragment>,
    visible_text: Rope,
    deleted_text: Rope,
    pub version: clock::Global,
    last_edit: clock::Local,
    undo_map: UndoMap,
    history: History,
    selections: HashMap<SelectionSetId, SelectionSet>,
    deferred_ops: OperationQueue,
    deferred_replicas: HashSet<ReplicaId>,
    replica_id: ReplicaId,
    remote_id: u64,
    local_clock: clock::Local,
    lamport_clock: clock::Lamport,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    start: clock::Global,
    end: clock::Global,
    edits: Vec<clock::Local>,
    ranges: Vec<Range<usize>>,
    selections_before: HashMap<SelectionSetId, Arc<AnchorRangeMap<SelectionState>>>,
    selections_after: HashMap<SelectionSetId, Arc<AnchorRangeMap<SelectionState>>>,
    first_edit_at: Instant,
    last_edit_at: Instant,
}

impl Transaction {
    pub fn starting_selection_set_ids<'a>(&'a self) -> impl Iterator<Item = SelectionSetId> + 'a {
        self.selections_before.keys().copied()
    }

    fn push_edit(&mut self, edit: &EditOperation) {
        self.edits.push(edit.timestamp.local());
        self.end.observe(edit.timestamp.local());

        let mut other_ranges = edit.ranges.iter().peekable();
        let mut new_ranges: Vec<Range<usize>> = Vec::new();
        let insertion_len = edit.new_text.as_ref().map_or(0, |t| t.len());
        let mut delta = 0;

        for mut self_range in self.ranges.iter().cloned() {
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

        self.ranges = new_ranges;
    }
}

#[derive(Clone)]
pub struct History {
    // TODO: Turn this into a String or Rope, maybe.
    pub base_text: Arc<str>,
    ops: HashMap<clock::Local, EditOperation>,
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
            group_interval: Duration::from_millis(300),
        }
    }

    fn push(&mut self, op: EditOperation) {
        self.ops.insert(op.timestamp.local(), op);
    }

    fn start_transaction(
        &mut self,
        start: clock::Global,
        selections_before: HashMap<SelectionSetId, Arc<AnchorRangeMap<SelectionState>>>,
        now: Instant,
    ) {
        self.transaction_depth += 1;
        if self.transaction_depth == 1 {
            self.undo_stack.push(Transaction {
                start: start.clone(),
                end: start,
                edits: Vec::new(),
                ranges: Vec::new(),
                selections_before,
                selections_after: Default::default(),
                first_edit_at: now,
                last_edit_at: now,
            });
        }
    }

    fn end_transaction(
        &mut self,
        selections_after: HashMap<SelectionSetId, Arc<AnchorRangeMap<SelectionState>>>,
        now: Instant,
    ) -> Option<&Transaction> {
        assert_ne!(self.transaction_depth, 0);
        self.transaction_depth -= 1;
        if self.transaction_depth == 0 {
            if self.undo_stack.last().unwrap().ranges.is_empty() {
                self.undo_stack.pop();
                None
            } else {
                let transaction = self.undo_stack.last_mut().unwrap();
                transaction.selections_after = selections_after;
                transaction.last_edit_at = now;
                Some(transaction)
            }
        } else {
            None
        }
    }

    fn group(&mut self) {
        let mut new_len = self.undo_stack.len();
        let mut transactions = self.undo_stack.iter_mut();

        if let Some(mut transaction) = transactions.next_back() {
            while let Some(prev_transaction) = transactions.next_back() {
                if transaction.first_edit_at - prev_transaction.last_edit_at <= self.group_interval
                    && transaction.start == prev_transaction.end
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
            for transaction in &*transactions_to_merge {
                for edit_id in &transaction.edits {
                    last_transaction.push_edit(&self.ops[edit_id]);
                }
            }

            if let Some(transaction) = transactions_to_merge.last_mut() {
                last_transaction.last_edit_at = transaction.last_edit_at;
                last_transaction
                    .selections_after
                    .extend(transaction.selections_after.drain());
                last_transaction.end = transaction.end.clone();
            }
        }

        self.undo_stack.truncate(new_len);
    }

    fn push_undo(&mut self, edit_id: clock::Local) {
        assert_ne!(self.transaction_depth, 0);
        let last_transaction = self.undo_stack.last_mut().unwrap();
        last_transaction.push_edit(&self.ops[&edit_id]);
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

struct Edits<'a, F: Fn(&FragmentSummary) -> bool> {
    visible_text: &'a Rope,
    deleted_text: &'a Rope,
    cursor: Option<FilterCursor<'a, F, Fragment, FragmentTextSummary>>,
    undos: &'a UndoMap,
    since: clock::Global,
    old_offset: usize,
    new_offset: usize,
    old_point: Point,
    new_point: Point,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Edit {
    pub old_bytes: Range<usize>,
    pub new_bytes: Range<usize>,
    pub old_lines: Range<Point>,
}

impl Edit {
    pub fn delta(&self) -> isize {
        self.inserted_bytes() as isize - self.deleted_bytes() as isize
    }

    pub fn deleted_bytes(&self) -> usize {
        self.old_bytes.end - self.old_bytes.start
    }

    pub fn inserted_bytes(&self) -> usize {
        self.new_bytes.end - self.new_bytes.start
    }

    pub fn deleted_lines(&self) -> Point {
        self.old_lines.end - self.old_lines.start
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct InsertionTimestamp {
    replica_id: ReplicaId,
    local: clock::Seq,
    lamport: clock::Seq,
}

impl InsertionTimestamp {
    fn local(&self) -> clock::Local {
        clock::Local {
            replica_id: self.replica_id,
            value: self.local,
        }
    }

    fn lamport(&self) -> clock::Lamport {
        clock::Lamport {
            replica_id: self.replica_id,
            value: self.lamport,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Fragment {
    timestamp: InsertionTimestamp,
    len: usize,
    visible: bool,
    deletions: HashSet<clock::Local>,
    max_undos: clock::Global,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FragmentSummary {
    text: FragmentTextSummary,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Edit(EditOperation),
    Undo {
        undo: UndoOperation,
        lamport_timestamp: clock::Lamport,
    },
    UpdateSelections {
        set_id: SelectionSetId,
        selections: Arc<AnchorRangeMap<SelectionState>>,
        lamport_timestamp: clock::Lamport,
    },
    RemoveSelections {
        set_id: SelectionSetId,
        lamport_timestamp: clock::Lamport,
    },
    SetActiveSelections {
        set_id: Option<SelectionSetId>,
        lamport_timestamp: clock::Lamport,
    },
    #[cfg(test)]
    Test(clock::Lamport),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditOperation {
    timestamp: InsertionTimestamp,
    version: clock::Global,
    ranges: Vec<Range<usize>>,
    new_text: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UndoOperation {
    id: clock::Local,
    counts: HashMap<clock::Local, u32>,
    ranges: Vec<Range<usize>>,
    version: clock::Global,
}

impl Buffer {
    pub fn new(replica_id: u16, remote_id: u64, history: History) -> Buffer {
        let mut fragments = SumTree::new();

        let visible_text = Rope::from(history.base_text.as_ref());
        if visible_text.len() > 0 {
            fragments.push(
                Fragment {
                    timestamp: Default::default(),
                    len: visible_text.len(),
                    visible: true,
                    deletions: Default::default(),
                    max_undos: Default::default(),
                },
                &None,
            );
        }

        Buffer {
            visible_text,
            deleted_text: Rope::new(),
            fragments,
            version: clock::Global::new(),
            last_edit: clock::Local::default(),
            undo_map: Default::default(),
            history,
            selections: HashMap::default(),
            deferred_ops: OperationQueue::new(),
            deferred_replicas: HashSet::default(),
            replica_id,
            remote_id,
            local_clock: clock::Local::new(replica_id),
            lamport_clock: clock::Lamport::new(replica_id),
        }
    }

    pub fn from_proto(replica_id: u16, message: proto::Buffer) -> Result<Self> {
        let mut buffer = Buffer::new(replica_id, message.id, History::new(message.content.into()));
        let ops = message
            .history
            .into_iter()
            .map(|op| Operation::Edit(op.into()));
        buffer.apply_ops(ops)?;
        buffer.selections = message
            .selections
            .into_iter()
            .map(|set| {
                let set = SelectionSet::try_from(set)?;
                Result::<_, anyhow::Error>::Ok((set.id, set))
            })
            .collect::<Result<_, _>>()?;
        Ok(buffer)
    }

    pub fn to_proto(&self) -> proto::Buffer {
        let ops = self.history.ops.values().map(Into::into).collect();
        proto::Buffer {
            id: self.remote_id,
            content: self.history.base_text.to_string(),
            history: ops,
            selections: self.selections.iter().map(|(_, set)| set.into()).collect(),
        }
    }

    pub fn version(&self) -> clock::Global {
        self.version.clone()
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            visible_text: self.visible_text.clone(),
            fragments: self.fragments.clone(),
            version: self.version.clone(),
        }
    }

    pub fn content<'a>(&'a self) -> Content<'a> {
        self.into()
    }

    pub fn as_rope(&self) -> &Rope {
        &self.visible_text
    }

    pub fn text_summary_for_range(&self, range: Range<usize>) -> TextSummary {
        self.content().text_summary_for_range(range)
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        self.content().anchor_at(position, bias)
    }

    pub fn anchor_range_set<E>(&self, entries: E) -> AnchorRangeSet
    where
        E: IntoIterator<Item = Range<(usize, Bias)>>,
    {
        self.content().anchor_range_set(entries)
    }

    pub fn point_for_offset(&self, offset: usize) -> Result<Point> {
        self.content().point_for_offset(offset)
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.visible_text.clip_point(point, bias)
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        self.visible_text.clip_offset(offset, bias)
    }

    pub fn replica_id(&self) -> ReplicaId {
        self.local_clock.replica_id
    }

    pub fn remote_id(&self) -> u64 {
        self.remote_id
    }

    pub fn text_summary(&self) -> TextSummary {
        self.visible_text.summary()
    }

    pub fn len(&self) -> usize {
        self.content().len()
    }

    pub fn line_len(&self, row: u32) -> u32 {
        self.content().line_len(row)
    }

    pub fn max_point(&self) -> Point {
        self.visible_text.max_point()
    }

    pub fn row_count(&self) -> u32 {
        self.max_point().row + 1
    }

    pub fn text(&self) -> String {
        self.text_for_range(0..self.len()).collect()
    }

    pub fn text_for_range<'a, T: ToOffset>(&'a self, range: Range<T>) -> Chunks<'a> {
        self.content().text_for_range(range)
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars_at(0)
    }

    pub fn chars_at<'a, T: 'a + ToOffset>(
        &'a self,
        position: T,
    ) -> impl Iterator<Item = char> + 'a {
        self.content().chars_at(position)
    }

    pub fn reversed_chars_at<'a, T: 'a + ToOffset>(
        &'a self,
        position: T,
    ) -> impl Iterator<Item = char> + 'a {
        self.content().reversed_chars_at(position)
    }

    pub fn chars_for_range<T: ToOffset>(&self, range: Range<T>) -> impl Iterator<Item = char> + '_ {
        self.text_for_range(range).flat_map(str::chars)
    }

    pub fn bytes_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = u8> + '_ {
        let offset = position.to_offset(self);
        self.visible_text.bytes_at(offset)
    }

    pub fn contains_str_at<T>(&self, position: T, needle: &str) -> bool
    where
        T: ToOffset,
    {
        let position = position.to_offset(self);
        position == self.clip_offset(position, Bias::Left)
            && self
                .bytes_at(position)
                .take(needle.len())
                .eq(needle.bytes())
    }

    pub fn deferred_ops_len(&self) -> usize {
        self.deferred_ops.len()
    }

    pub fn edit<R, I, S, T>(&mut self, ranges: R, new_text: T) -> EditOperation
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

        self.start_transaction(None).unwrap();
        let timestamp = InsertionTimestamp {
            replica_id: self.replica_id,
            local: self.local_clock.tick().value,
            lamport: self.lamport_clock.tick().value,
        };
        let edit = self.apply_local_edit(ranges.into_iter(), new_text, timestamp);

        self.history.push(edit.clone());
        self.history.push_undo(edit.timestamp.local());
        self.last_edit = edit.timestamp.local();
        self.version.observe(edit.timestamp.local());
        self.end_transaction(None);
        edit
    }

    fn apply_local_edit<S: ToOffset>(
        &mut self,
        ranges: impl ExactSizeIterator<Item = Range<S>>,
        new_text: Option<String>,
        timestamp: InsertionTimestamp,
    ) -> EditOperation {
        let mut edit = EditOperation {
            timestamp,
            version: self.version(),
            ranges: Vec::with_capacity(ranges.len()),
            new_text: None,
        };

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

            let full_range_start = range.start + old_fragments.start().deleted;

            // Preserve any portion of the current fragment that precedes this range.
            if fragment_start < range.start {
                let mut prefix = old_fragments.item().unwrap().clone();
                prefix.len = range.start - fragment_start;
                new_ropes.push_fragment(&prefix, prefix.visible);
                new_fragments.push(prefix, &None);
                fragment_start = range.start;
            }

            // Insert the new text before any existing fragments within the range.
            if let Some(new_text) = new_text.as_deref() {
                new_ropes.push_str(new_text);
                new_fragments.push(
                    Fragment {
                        timestamp,
                        len: new_text.len(),
                        deletions: Default::default(),
                        max_undos: Default::default(),
                        visible: true,
                    },
                    &None,
                );
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
                    intersection.deletions.insert(timestamp.local());
                    intersection.visible = false;
                }
                if intersection.len > 0 {
                    new_ropes.push_fragment(&intersection, fragment.visible);
                    new_fragments.push(intersection, &None);
                    fragment_start = intersection_end;
                }
                if fragment_end <= range.end {
                    old_fragments.next(&None);
                }
            }

            let full_range_end = range.end + old_fragments.start().deleted;
            edit.ranges.push(full_range_start..full_range_end);
        }

        // If the current fragment has been partially consumed, then consume the rest of it
        // and advance to the next fragment before slicing.
        if fragment_start > old_fragments.start().visible {
            let fragment_end = old_fragments.end(&None).visible;
            if fragment_end > fragment_start {
                let mut suffix = old_fragments.item().unwrap().clone();
                suffix.len = fragment_end - fragment_start;
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

        self.fragments = new_fragments;
        self.visible_text = visible_text;
        self.deleted_text = deleted_text;
        edit.new_text = new_text;
        edit
    }

    pub fn apply_ops<I: IntoIterator<Item = Operation>>(&mut self, ops: I) -> Result<()> {
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
                    self.version.observe(edit.timestamp.local());
                    self.history.push(edit);
                }
            }
            Operation::Undo {
                undo,
                lamport_timestamp,
            } => {
                if !self.version.observed(undo.id) {
                    self.apply_undo(&undo)?;
                    self.version.observe(undo.id);
                    self.lamport_clock.observe(lamport_timestamp);
                }
            }
            Operation::UpdateSelections {
                set_id,
                selections,
                lamport_timestamp,
            } => {
                if let Some(set) = self.selections.get_mut(&set_id) {
                    set.selections = selections;
                } else {
                    self.selections.insert(
                        set_id,
                        SelectionSet {
                            id: set_id,
                            selections,
                            active: false,
                        },
                    );
                }
                self.lamport_clock.observe(lamport_timestamp);
            }
            Operation::RemoveSelections {
                set_id,
                lamport_timestamp,
            } => {
                self.selections.remove(&set_id);
                self.lamport_clock.observe(lamport_timestamp);
            }
            Operation::SetActiveSelections {
                set_id,
                lamport_timestamp,
            } => {
                for (id, set) in &mut self.selections {
                    if id.replica_id == lamport_timestamp.replica_id {
                        if Some(*id) == set_id {
                            set.active = true;
                        } else {
                            set.active = false;
                        }
                    }
                }
                self.lamport_clock.observe(lamport_timestamp);
            }
            #[cfg(test)]
            Operation::Test(_) => {}
        }
        Ok(())
    }

    fn apply_remote_edit(
        &mut self,
        version: &clock::Global,
        ranges: &[Range<usize>],
        new_text: Option<&str>,
        timestamp: InsertionTimestamp,
    ) {
        if ranges.is_empty() {
            return;
        }

        let cx = Some(version.clone());
        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));
        let mut old_fragments = self.fragments.cursor::<VersionedOffset>();
        let mut new_fragments =
            old_fragments.slice(&VersionedOffset::Offset(ranges[0].start), Bias::Left, &cx);
        new_ropes.push_tree(new_fragments.summary().text);

        let mut fragment_start = old_fragments.start().offset();
        for range in ranges {
            let fragment_end = old_fragments.end(&cx).offset();

            // If the current fragment ends before this range, then jump ahead to the first fragment
            // that extends past the start of this range, reusing any intervening fragments.
            if fragment_end < range.start {
                // If the current fragment has been partially consumed, then consume the rest of it
                // and advance to the next fragment before slicing.
                if fragment_start > old_fragments.start().offset() {
                    if fragment_end > fragment_start {
                        let mut suffix = old_fragments.item().unwrap().clone();
                        suffix.len = fragment_end - fragment_start;
                        new_ropes.push_fragment(&suffix, suffix.visible);
                        new_fragments.push(suffix, &None);
                    }
                    old_fragments.next(&cx);
                }

                let slice =
                    old_fragments.slice(&VersionedOffset::Offset(range.start), Bias::Left, &cx);
                new_ropes.push_tree(slice.summary().text);
                new_fragments.push_tree(slice, &None);
                fragment_start = old_fragments.start().offset();
            }

            // If we are at the end of a non-concurrent fragment, advance to the next one.
            let fragment_end = old_fragments.end(&cx).offset();
            if fragment_end == range.start && fragment_end > fragment_start {
                let mut fragment = old_fragments.item().unwrap().clone();
                fragment.len = fragment_end - fragment_start;
                new_ropes.push_fragment(&fragment, fragment.visible);
                new_fragments.push(fragment, &None);
                old_fragments.next(&cx);
                fragment_start = old_fragments.start().offset();
            }

            // Skip over insertions that are concurrent to this edit, but have a lower lamport
            // timestamp.
            while let Some(fragment) = old_fragments.item() {
                if fragment_start == range.start
                    && fragment.timestamp.lamport() > timestamp.lamport()
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
                prefix.len = range.start - fragment_start;
                fragment_start = range.start;
                new_ropes.push_fragment(&prefix, prefix.visible);
                new_fragments.push(prefix, &None);
            }

            // Insert the new text before any existing fragments within the range.
            if let Some(new_text) = new_text {
                new_ropes.push_str(new_text);
                new_fragments.push(
                    Fragment {
                        timestamp,
                        len: new_text.len(),
                        deletions: Default::default(),
                        max_undos: Default::default(),
                        visible: true,
                    },
                    &None,
                );
            }

            // Advance through every fragment that intersects this range, marking the intersecting
            // portions as deleted.
            while fragment_start < range.end {
                let fragment = old_fragments.item().unwrap();
                let fragment_end = old_fragments.end(&cx).offset();
                let mut intersection = fragment.clone();
                let intersection_end = cmp::min(range.end, fragment_end);
                if fragment.was_visible(version, &self.undo_map) {
                    intersection.len = intersection_end - fragment_start;
                    intersection.deletions.insert(timestamp.local());
                    intersection.visible = false;
                }
                if intersection.len > 0 {
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
        if fragment_start > old_fragments.start().offset() {
            let fragment_end = old_fragments.end(&cx).offset();
            if fragment_end > fragment_start {
                let mut suffix = old_fragments.item().unwrap().clone();
                suffix.len = fragment_end - fragment_start;
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

        self.fragments = new_fragments;
        self.visible_text = visible_text;
        self.deleted_text = deleted_text;
        self.local_clock.observe(timestamp.local());
        self.lamport_clock.observe(timestamp.lamport());
    }

    fn apply_undo(&mut self, undo: &UndoOperation) -> Result<()> {
        self.undo_map.insert(undo);

        let mut cx = undo.version.clone();
        for edit_id in undo.counts.keys().copied() {
            cx.observe(edit_id);
        }
        let cx = Some(cx);

        let mut old_fragments = self.fragments.cursor::<VersionedOffset>();
        let mut new_fragments = old_fragments.slice(
            &VersionedOffset::Offset(undo.ranges[0].start),
            Bias::Right,
            &cx,
        );
        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));
        new_ropes.push_tree(new_fragments.summary().text);

        for range in &undo.ranges {
            let mut end_offset = old_fragments.end(&cx).offset();

            if end_offset < range.start {
                let preceding_fragments =
                    old_fragments.slice(&VersionedOffset::Offset(range.start), Bias::Right, &cx);
                new_ropes.push_tree(preceding_fragments.summary().text);
                new_fragments.push_tree(preceding_fragments, &None);
            }

            while end_offset <= range.end {
                if let Some(fragment) = old_fragments.item() {
                    let mut fragment = fragment.clone();
                    let fragment_was_visible = fragment.visible;

                    if fragment.was_visible(&undo.version, &self.undo_map)
                        || undo.counts.contains_key(&fragment.timestamp.local())
                    {
                        fragment.visible = fragment.is_visible(&self.undo_map);
                        fragment.max_undos.observe(undo.id);
                    }
                    new_ropes.push_fragment(&fragment, fragment_was_visible);
                    new_fragments.push(fragment, &None);

                    old_fragments.next(&cx);
                    if end_offset == old_fragments.end(&cx).offset() {
                        let unseen_fragments = old_fragments.slice(
                            &VersionedOffset::Offset(end_offset),
                            Bias::Right,
                            &cx,
                        );
                        new_ropes.push_tree(unseen_fragments.summary().text);
                        new_fragments.push_tree(unseen_fragments, &None);
                    }
                    end_offset = old_fragments.end(&cx).offset();
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
        self.fragments = new_fragments;
        self.visible_text = visible_text;
        self.deleted_text = deleted_text;
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
                Operation::Edit(edit) => self.version >= edit.version,
                Operation::Undo { undo, .. } => self.version >= undo.version,
                Operation::UpdateSelections { selections, .. } => {
                    self.version >= *selections.version()
                }
                Operation::RemoveSelections { .. } => true,
                Operation::SetActiveSelections { set_id, .. } => {
                    set_id.map_or(true, |set_id| self.selections.contains_key(&set_id))
                }
                #[cfg(test)]
                Operation::Test(_) => true,
            }
        }
    }

    pub fn peek_undo_stack(&self) -> Option<&Transaction> {
        self.history.undo_stack.last()
    }

    pub fn start_transaction(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
    ) -> Result<()> {
        self.start_transaction_at(selection_set_ids, Instant::now())
    }

    pub fn start_transaction_at(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
        now: Instant,
    ) -> Result<()> {
        let selections = selection_set_ids
            .into_iter()
            .map(|set_id| {
                let set = self
                    .selections
                    .get(&set_id)
                    .expect("invalid selection set id");
                (set_id, set.selections.clone())
            })
            .collect();
        self.history
            .start_transaction(self.version.clone(), selections, now);
        Ok(())
    }

    pub fn end_transaction(&mut self, selection_set_ids: impl IntoIterator<Item = SelectionSetId>) {
        self.end_transaction_at(selection_set_ids, Instant::now());
    }

    pub fn end_transaction_at(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
        now: Instant,
    ) -> Option<clock::Global> {
        let selections = selection_set_ids
            .into_iter()
            .map(|set_id| {
                let set = self
                    .selections
                    .get(&set_id)
                    .expect("invalid selection set id");
                (set_id, set.selections.clone())
            })
            .collect();

        if let Some(transaction) = self.history.end_transaction(selections, now) {
            let since = transaction.start.clone();
            self.history.group();
            Some(since)
        } else {
            None
        }
    }

    pub fn remove_peer(&mut self, replica_id: ReplicaId) {
        self.selections
            .retain(|set_id, _| set_id.replica_id != replica_id)
    }

    pub fn undo(&mut self) -> Vec<Operation> {
        let mut ops = Vec::new();
        if let Some(transaction) = self.history.pop_undo().cloned() {
            let selections = transaction.selections_before.clone();
            ops.push(self.undo_or_redo(transaction).unwrap());
            for (set_id, selections) in selections {
                ops.extend(self.restore_selection_set(set_id, selections));
            }
        }
        ops
    }

    pub fn redo(&mut self) -> Vec<Operation> {
        let mut ops = Vec::new();
        if let Some(transaction) = self.history.pop_redo().cloned() {
            let selections = transaction.selections_after.clone();
            ops.push(self.undo_or_redo(transaction).unwrap());
            for (set_id, selections) in selections {
                ops.extend(self.restore_selection_set(set_id, selections));
            }
        }
        ops
    }

    fn undo_or_redo(&mut self, transaction: Transaction) -> Result<Operation> {
        let mut counts = HashMap::default();
        for edit_id in transaction.edits {
            counts.insert(edit_id, self.undo_map.undo_count(edit_id) + 1);
        }

        let undo = UndoOperation {
            id: self.local_clock.tick(),
            counts,
            ranges: transaction.ranges,
            version: transaction.start.clone(),
        };
        self.apply_undo(&undo)?;
        self.version.observe(undo.id);

        Ok(Operation::Undo {
            undo,
            lamport_timestamp: self.lamport_clock.tick(),
        })
    }

    pub fn selection_set(&self, set_id: SelectionSetId) -> Result<&SelectionSet> {
        self.selections
            .get(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))
    }

    pub fn selection_sets(&self) -> impl Iterator<Item = (&SelectionSetId, &SelectionSet)> {
        self.selections.iter()
    }

    fn build_selection_anchor_range_map<T: ToOffset>(
        &self,
        selections: &[Selection<T>],
    ) -> Arc<AnchorRangeMap<SelectionState>> {
        Arc::new(
            self.content()
                .anchor_range_map(selections.iter().map(|selection| {
                    let start = selection.start.to_offset(self);
                    let end = selection.end.to_offset(self);
                    let range = (start, Bias::Left)..(end, Bias::Left);
                    let state = SelectionState {
                        id: selection.id,
                        reversed: selection.reversed,
                        goal: selection.goal,
                    };
                    (range, state)
                })),
        )
    }

    pub fn update_selection_set<T: ToOffset>(
        &mut self,
        set_id: SelectionSetId,
        selections: &[Selection<T>],
    ) -> Result<Operation> {
        let selections = self.build_selection_anchor_range_map(selections);
        let set = self
            .selections
            .get_mut(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        set.selections = selections.clone();
        Ok(Operation::UpdateSelections {
            set_id,
            selections,
            lamport_timestamp: self.lamport_clock.tick(),
        })
    }

    pub fn restore_selection_set(
        &mut self,
        set_id: SelectionSetId,
        selections: Arc<AnchorRangeMap<SelectionState>>,
    ) -> Result<Operation> {
        let set = self
            .selections
            .get_mut(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        set.selections = selections.clone();
        Ok(Operation::UpdateSelections {
            set_id,
            selections,
            lamport_timestamp: self.lamport_clock.tick(),
        })
    }

    pub fn add_selection_set<T: ToOffset>(&mut self, selections: &[Selection<T>]) -> Operation {
        let selections = self.build_selection_anchor_range_map(selections);
        let set_id = self.lamport_clock.tick();
        self.selections.insert(
            set_id,
            SelectionSet {
                id: set_id,
                selections: selections.clone(),
                active: false,
            },
        );
        Operation::UpdateSelections {
            set_id,
            selections,
            lamport_timestamp: set_id,
        }
    }

    pub fn set_active_selection_set(
        &mut self,
        set_id: Option<SelectionSetId>,
    ) -> Result<Operation> {
        if let Some(set_id) = set_id {
            assert_eq!(set_id.replica_id, self.replica_id());
        }

        for (id, set) in &mut self.selections {
            if id.replica_id == self.local_clock.replica_id {
                if Some(*id) == set_id {
                    set.active = true;
                } else {
                    set.active = false;
                }
            }
        }

        Ok(Operation::SetActiveSelections {
            set_id,
            lamport_timestamp: self.lamport_clock.tick(),
        })
    }

    pub fn remove_selection_set(&mut self, set_id: SelectionSetId) -> Result<Operation> {
        self.selections
            .remove(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        Ok(Operation::RemoveSelections {
            set_id,
            lamport_timestamp: self.lamport_clock.tick(),
        })
    }

    pub fn edits_since<'a>(&'a self, since: clock::Global) -> impl 'a + Iterator<Item = Edit> {
        let since_2 = since.clone();
        let cursor = if since == self.version {
            None
        } else {
            Some(self.fragments.filter(
                move |summary| summary.max_version.changed_since(&since_2),
                &None,
            ))
        };

        Edits {
            visible_text: &self.visible_text,
            deleted_text: &self.deleted_text,
            cursor,
            undos: &self.undo_map,
            since,
            old_offset: 0,
            new_offset: 0,
            old_point: Point::zero(),
            new_point: Point::zero(),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Buffer {
    fn random_byte_range(&mut self, start_offset: usize, rng: &mut impl rand::Rng) -> Range<usize> {
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
        (old_ranges, new_text, Operation::Edit(op))
    }

    pub fn randomly_mutate<T>(&mut self, rng: &mut T) -> Vec<Operation>
    where
        T: rand::Rng,
    {
        use rand::prelude::*;

        let mut ops = vec![self.randomly_edit(rng, 5).2];

        // Randomly add, remove or mutate selection sets.
        let replica_selection_sets = &self
            .selection_sets()
            .map(|(set_id, _)| *set_id)
            .filter(|set_id| self.replica_id == set_id.replica_id)
            .collect::<Vec<_>>();
        let set_id = replica_selection_sets.choose(rng);
        if set_id.is_some() && rng.gen_bool(1.0 / 6.0) {
            ops.push(self.remove_selection_set(*set_id.unwrap()).unwrap());
        } else {
            let mut ranges = Vec::new();
            for _ in 0..5 {
                ranges.push(self.random_byte_range(0, rng));
            }
            let new_selections = self.selections_from_ranges(ranges).unwrap();

            let op = if set_id.is_none() || rng.gen_bool(1.0 / 5.0) {
                self.add_selection_set(&new_selections)
            } else {
                self.update_selection_set(*set_id.unwrap(), &new_selections)
                    .unwrap()
            };
            ops.push(op);
        }

        ops
    }

    pub fn randomly_undo_redo(&mut self, rng: &mut impl rand::Rng) -> Vec<Operation> {
        use rand::prelude::*;

        let mut ops = Vec::new();
        for _ in 0..rng.gen_range(1..=5) {
            if let Some(transaction) = self.history.undo_stack.choose(rng).cloned() {
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

    fn selections_from_ranges<I>(&self, ranges: I) -> Result<Vec<Selection<usize>>>
    where
        I: IntoIterator<Item = Range<usize>>,
    {
        use std::sync::atomic::{self, AtomicUsize};

        static NEXT_SELECTION_ID: AtomicUsize = AtomicUsize::new(0);

        let mut ranges = ranges.into_iter().collect::<Vec<_>>();
        ranges.sort_unstable_by_key(|range| range.start);

        let mut selections = Vec::with_capacity(ranges.len());
        for range in ranges {
            if range.start > range.end {
                selections.push(Selection {
                    id: NEXT_SELECTION_ID.fetch_add(1, atomic::Ordering::SeqCst),
                    start: range.end,
                    end: range.start,
                    reversed: true,
                    goal: SelectionGoal::None,
                });
            } else {
                selections.push(Selection {
                    id: NEXT_SELECTION_ID.fetch_add(1, atomic::Ordering::SeqCst),
                    start: range.start,
                    end: range.end,
                    reversed: false,
                    goal: SelectionGoal::None,
                });
            }
        }
        Ok(selections)
    }

    pub fn selection_ranges<'a>(&'a self, set_id: SelectionSetId) -> Result<Vec<Range<usize>>> {
        Ok(self
            .selection_set(set_id)?
            .offset_selections(self)
            .map(move |selection| {
                if selection.reversed {
                    selection.end..selection.start
                } else {
                    selection.start..selection.end
                }
            })
            .collect())
    }

    pub fn all_selection_ranges<'a>(
        &'a self,
    ) -> impl 'a + Iterator<Item = (SelectionSetId, Vec<Range<usize>>)> {
        self.selections
            .keys()
            .map(move |set_id| (*set_id, self.selection_ranges(*set_id).unwrap()))
    }
}

#[derive(Clone)]
pub struct Snapshot {
    visible_text: Rope,
    fragments: SumTree<Fragment>,
    version: clock::Global,
}

impl Snapshot {
    pub fn as_rope(&self) -> &Rope {
        &self.visible_text
    }

    pub fn len(&self) -> usize {
        self.visible_text.len()
    }

    pub fn line_len(&self, row: u32) -> u32 {
        self.content().line_len(row)
    }

    pub fn indent_column_for_line(&self, row: u32) -> u32 {
        self.content().indent_column_for_line(row)
    }

    pub fn text(&self) -> Rope {
        self.visible_text.clone()
    }

    pub fn text_summary(&self) -> TextSummary {
        self.visible_text.summary()
    }

    pub fn max_point(&self) -> Point {
        self.visible_text.max_point()
    }

    pub fn text_for_range<T: ToOffset>(&self, range: Range<T>) -> Chunks {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.visible_text.chunks_in_range(range)
    }

    pub fn text_summary_for_range<T>(&self, range: Range<T>) -> TextSummary
    where
        T: ToOffset,
    {
        let range = range.start.to_offset(self.content())..range.end.to_offset(self.content());
        self.content().text_summary_for_range(range)
    }

    pub fn point_for_offset(&self, offset: usize) -> Result<Point> {
        self.content().point_for_offset(offset)
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        self.visible_text.clip_offset(offset, bias)
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.visible_text.clip_point(point, bias)
    }

    pub fn to_offset(&self, point: Point) -> usize {
        self.visible_text.to_offset(point)
    }

    pub fn to_point(&self, offset: usize) -> Point {
        self.visible_text.to_point(offset)
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.content().anchor_at(position, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.content().anchor_at(position, Bias::Right)
    }

    pub fn content(&self) -> Content {
        self.into()
    }
}

pub struct Content<'a> {
    visible_text: &'a Rope,
    fragments: &'a SumTree<Fragment>,
    version: &'a clock::Global,
}

impl<'a> From<&'a Snapshot> for Content<'a> {
    fn from(snapshot: &'a Snapshot) -> Self {
        Self {
            visible_text: &snapshot.visible_text,
            fragments: &snapshot.fragments,
            version: &snapshot.version,
        }
    }
}

impl<'a> From<&'a Buffer> for Content<'a> {
    fn from(buffer: &'a Buffer) -> Self {
        Self {
            visible_text: &buffer.visible_text,
            fragments: &buffer.fragments,
            version: &buffer.version,
        }
    }
}

impl<'a> From<&'a mut Buffer> for Content<'a> {
    fn from(buffer: &'a mut Buffer) -> Self {
        Self {
            visible_text: &buffer.visible_text,
            fragments: &buffer.fragments,
            version: &buffer.version,
        }
    }
}

impl<'a> From<&'a Content<'a>> for Content<'a> {
    fn from(content: &'a Content) -> Self {
        Self {
            visible_text: &content.visible_text,
            fragments: &content.fragments,
            version: &content.version,
        }
    }
}

impl<'a> Content<'a> {
    fn max_point(&self) -> Point {
        self.visible_text.max_point()
    }

    fn len(&self) -> usize {
        self.fragments.extent::<usize>(&None)
    }

    pub fn chars_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = char> + 'a {
        let offset = position.to_offset(self);
        self.visible_text.chars_at(offset)
    }

    pub fn reversed_chars_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = char> + 'a {
        let offset = position.to_offset(self);
        self.visible_text.reversed_chars_at(offset)
    }

    pub fn text_for_range<T: ToOffset>(&self, range: Range<T>) -> Chunks<'a> {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);
        self.visible_text.chunks_in_range(start..end)
    }

    fn line_len(&self, row: u32) -> u32 {
        let row_start_offset = Point::new(row, 0).to_offset(self);
        let row_end_offset = if row >= self.max_point().row {
            self.len()
        } else {
            Point::new(row + 1, 0).to_offset(self) - 1
        };
        (row_end_offset - row_start_offset) as u32
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

    fn summary_for_anchor(&self, anchor: &Anchor) -> TextSummary {
        let cx = Some(anchor.version.clone());
        let mut cursor = self.fragments.cursor::<(VersionedOffset, usize)>();
        cursor.seek(&VersionedOffset::Offset(anchor.offset), anchor.bias, &cx);
        let overshoot = if cursor.item().map_or(false, |fragment| fragment.visible) {
            anchor.offset - cursor.start().0.offset()
        } else {
            0
        };
        self.text_summary_for_range(0..cursor.start().1 + overshoot)
    }

    fn text_summary_for_range(&self, range: Range<usize>) -> TextSummary {
        self.visible_text.cursor(range.start).summary(range.end)
    }

    fn summaries_for_anchors<T>(
        &self,
        map: &'a AnchorMap<T>,
    ) -> impl Iterator<Item = (TextSummary, &'a T)> {
        let cx = Some(map.version.clone());
        let mut summary = TextSummary::default();
        let mut rope_cursor = self.visible_text.cursor(0);
        let mut cursor = self.fragments.cursor::<(VersionedOffset, usize)>();
        map.entries.iter().map(move |((offset, bias), value)| {
            cursor.seek_forward(&VersionedOffset::Offset(*offset), *bias, &cx);
            let overshoot = if cursor.item().map_or(false, |fragment| fragment.visible) {
                offset - cursor.start().0.offset()
            } else {
                0
            };
            summary += rope_cursor.summary(cursor.start().1 + overshoot);
            (summary.clone(), value)
        })
    }

    fn summaries_for_anchor_ranges<T>(
        &self,
        map: &'a AnchorRangeMap<T>,
    ) -> impl Iterator<Item = (Range<TextSummary>, &'a T)> {
        let cx = Some(map.version.clone());
        let mut summary = TextSummary::default();
        let mut rope_cursor = self.visible_text.cursor(0);
        let mut cursor = self.fragments.cursor::<(VersionedOffset, usize)>();
        map.entries.iter().map(move |(range, value)| {
            let Range {
                start: (start_offset, start_bias),
                end: (end_offset, end_bias),
            } = range;

            cursor.seek_forward(&VersionedOffset::Offset(*start_offset), *start_bias, &cx);
            let overshoot = if cursor.item().map_or(false, |fragment| fragment.visible) {
                start_offset - cursor.start().0.offset()
            } else {
                0
            };
            summary += rope_cursor.summary(cursor.start().1 + overshoot);
            let start_summary = summary.clone();

            cursor.seek_forward(&VersionedOffset::Offset(*end_offset), *end_bias, &cx);
            let overshoot = if cursor.item().map_or(false, |fragment| fragment.visible) {
                end_offset - cursor.start().0.offset()
            } else {
                0
            };
            summary += rope_cursor.summary(cursor.start().1 + overshoot);
            let end_summary = summary.clone();

            (start_summary..end_summary, value)
        })
    }

    fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        let offset = position.to_offset(self);
        let max_offset = self.len();
        assert!(offset <= max_offset, "offset is out of range");
        let mut cursor = self.fragments.cursor::<FragmentTextSummary>();
        cursor.seek(&offset, bias, &None);
        Anchor {
            offset: offset + cursor.start().deleted,
            bias,
            version: self.version.clone(),
        }
    }

    pub fn anchor_map<T, E>(&self, entries: E) -> AnchorMap<T>
    where
        E: IntoIterator<Item = ((usize, Bias), T)>,
    {
        let version = self.version.clone();
        let mut cursor = self.fragments.cursor::<FragmentTextSummary>();
        let entries = entries
            .into_iter()
            .map(|((offset, bias), value)| {
                cursor.seek_forward(&offset, bias, &None);
                let full_offset = cursor.start().deleted + offset;
                ((full_offset, bias), value)
            })
            .collect();

        AnchorMap { version, entries }
    }

    pub fn anchor_range_map<T, E>(&self, entries: E) -> AnchorRangeMap<T>
    where
        E: IntoIterator<Item = (Range<(usize, Bias)>, T)>,
    {
        let version = self.version.clone();
        let mut cursor = self.fragments.cursor::<FragmentTextSummary>();
        let entries = entries
            .into_iter()
            .map(|(range, value)| {
                let Range {
                    start: (start_offset, start_bias),
                    end: (end_offset, end_bias),
                } = range;
                cursor.seek_forward(&start_offset, start_bias, &None);
                let full_start_offset = cursor.start().deleted + start_offset;
                cursor.seek_forward(&end_offset, end_bias, &None);
                let full_end_offset = cursor.start().deleted + end_offset;
                (
                    (full_start_offset, start_bias)..(full_end_offset, end_bias),
                    value,
                )
            })
            .collect();

        AnchorRangeMap { version, entries }
    }

    pub fn anchor_set<E>(&self, entries: E) -> AnchorSet
    where
        E: IntoIterator<Item = (usize, Bias)>,
    {
        AnchorSet(self.anchor_map(entries.into_iter().map(|range| (range, ()))))
    }

    pub fn anchor_range_set<E>(&self, entries: E) -> AnchorRangeSet
    where
        E: IntoIterator<Item = Range<(usize, Bias)>>,
    {
        AnchorRangeSet(self.anchor_range_map(entries.into_iter().map(|range| (range, ()))))
    }

    fn full_offset_for_anchor(&self, anchor: &Anchor) -> usize {
        let cx = Some(anchor.version.clone());
        let mut cursor = self
            .fragments
            .cursor::<(VersionedOffset, FragmentTextSummary)>();
        cursor.seek(&VersionedOffset::Offset(anchor.offset), anchor.bias, &cx);
        let overshoot = if cursor.item().is_some() {
            anchor.offset - cursor.start().0.offset()
        } else {
            0
        };
        let summary = cursor.start().1;
        summary.visible + summary.deleted + overshoot
    }

    fn point_for_offset(&self, offset: usize) -> Result<Point> {
        if offset <= self.len() {
            Ok(self.text_summary_for_range(0..offset).lines)
        } else {
            Err(anyhow!("offset out of bounds"))
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

impl<'a, F: Fn(&FragmentSummary) -> bool> Iterator for Edits<'a, F> {
    type Item = Edit;

    fn next(&mut self) -> Option<Self::Item> {
        let mut change: Option<Edit> = None;
        let cursor = self.cursor.as_mut()?;

        while let Some(fragment) = cursor.item() {
            let bytes = cursor.start().visible - self.new_offset;
            let lines = self.visible_text.to_point(cursor.start().visible) - self.new_point;
            self.old_offset += bytes;
            self.old_point += &lines;
            self.new_offset += bytes;
            self.new_point += &lines;

            if !fragment.was_visible(&self.since, &self.undos) && fragment.visible {
                let fragment_lines =
                    self.visible_text.to_point(self.new_offset + fragment.len) - self.new_point;
                if let Some(ref mut change) = change {
                    if change.new_bytes.end == self.new_offset {
                        change.new_bytes.end += fragment.len;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_bytes: self.old_offset..self.old_offset,
                        new_bytes: self.new_offset..self.new_offset + fragment.len,
                        old_lines: self.old_point..self.old_point,
                    });
                }

                self.new_offset += fragment.len;
                self.new_point += &fragment_lines;
            } else if fragment.was_visible(&self.since, &self.undos) && !fragment.visible {
                let deleted_start = cursor.start().deleted;
                let fragment_lines = self.deleted_text.to_point(deleted_start + fragment.len)
                    - self.deleted_text.to_point(deleted_start);
                if let Some(ref mut change) = change {
                    if change.new_bytes.end == self.new_offset {
                        change.old_bytes.end += fragment.len;
                        change.old_lines.end += &fragment_lines;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_bytes: self.old_offset..self.old_offset + fragment.len,
                        new_bytes: self.new_offset..self.new_offset,
                        old_lines: self.old_point..self.old_point + &fragment_lines,
                    });
                }

                self.old_offset += fragment.len;
                self.old_point += &fragment_lines;
            }

            cursor.next(&None);
        }

        change
    }
}

impl Fragment {
    fn is_visible(&self, undos: &UndoMap) -> bool {
        !undos.is_undone(self.timestamp.local())
            && self.deletions.iter().all(|d| undos.is_undone(*d))
    }

    fn was_visible(&self, version: &clock::Global, undos: &UndoMap) -> bool {
        (version.observed(self.timestamp.local())
            && !undos.was_undone(self.timestamp.local(), version))
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
        max_version.observe(self.timestamp.local());
        for deletion in &self.deletions {
            max_version.observe(*deletion);
        }
        max_version.join(&self.max_undos);

        let mut min_insertion_version = clock::Global::new();
        min_insertion_version.observe(self.timestamp.local());
        let max_insertion_version = min_insertion_version.clone();
        if self.visible {
            FragmentSummary {
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
            text: FragmentTextSummary::default(),
            max_version: clock::Global::new(),
            min_insertion_version: clock::Global::new(),
            max_insertion_version: clock::Global::new(),
        }
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for usize {
    fn add_summary(&mut self, summary: &FragmentSummary, _: &Option<clock::Global>) {
        *self += summary.text.visible;
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
enum VersionedOffset {
    Offset(usize),
    InvalidVersion,
}

impl VersionedOffset {
    fn offset(&self) -> usize {
        if let Self::Offset(offset) = self {
            *offset
        } else {
            panic!("invalid version")
        }
    }
}

impl Default for VersionedOffset {
    fn default() -> Self {
        Self::Offset(0)
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for VersionedOffset {
    fn add_summary(&mut self, summary: &'a FragmentSummary, cx: &Option<clock::Global>) {
        if let Self::Offset(offset) = self {
            let version = cx.as_ref().unwrap();
            if *version >= summary.max_insertion_version {
                *offset += summary.text.visible + summary.text.deleted;
            } else if !summary
                .min_insertion_version
                .iter()
                .all(|t| !version.observed(*t))
            {
                *self = Self::InvalidVersion;
            }
        }
    }
}

impl<'a> sum_tree::SeekTarget<'a, FragmentSummary, Self> for VersionedOffset {
    fn cmp(&self, other: &Self, _: &Option<clock::Global>) -> cmp::Ordering {
        match (self, other) {
            (Self::Offset(a), Self::Offset(b)) => Ord::cmp(a, b),
            (Self::Offset(_), Self::InvalidVersion) => cmp::Ordering::Less,
            (Self::InvalidVersion, _) => unreachable!(),
        }
    }
}

impl Operation {
    fn replica_id(&self) -> ReplicaId {
        self.lamport_timestamp().replica_id
    }

    fn lamport_timestamp(&self) -> clock::Lamport {
        match self {
            Operation::Edit(edit) => edit.timestamp.lamport(),
            Operation::Undo {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::UpdateSelections {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::RemoveSelections {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::SetActiveSelections {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            #[cfg(test)]
            Operation::Test(lamport_timestamp) => *lamport_timestamp,
        }
    }

    pub fn is_edit(&self) -> bool {
        match self {
            Operation::Edit { .. } => true,
            _ => false,
        }
    }
}

impl<'a> Into<proto::Operation> for &'a Operation {
    fn into(self) -> proto::Operation {
        proto::Operation {
            variant: Some(match self {
                Operation::Edit(edit) => proto::operation::Variant::Edit(edit.into()),
                Operation::Undo {
                    undo,
                    lamport_timestamp,
                } => proto::operation::Variant::Undo(proto::operation::Undo {
                    replica_id: undo.id.replica_id as u32,
                    local_timestamp: undo.id.value,
                    lamport_timestamp: lamport_timestamp.value,
                    ranges: undo
                        .ranges
                        .iter()
                        .map(|r| proto::Range {
                            start: r.start as u64,
                            end: r.end as u64,
                        })
                        .collect(),
                    counts: undo
                        .counts
                        .iter()
                        .map(|(edit_id, count)| proto::operation::UndoCount {
                            replica_id: edit_id.replica_id as u32,
                            local_timestamp: edit_id.value,
                            count: *count,
                        })
                        .collect(),
                    version: From::from(&undo.version),
                }),
                Operation::UpdateSelections {
                    set_id,
                    selections,
                    lamport_timestamp,
                } => proto::operation::Variant::UpdateSelections(
                    proto::operation::UpdateSelections {
                        replica_id: set_id.replica_id as u32,
                        local_timestamp: set_id.value,
                        lamport_timestamp: lamport_timestamp.value,
                        version: selections.version().into(),
                        selections: selections
                            .raw_entries()
                            .iter()
                            .map(|(range, state)| proto::Selection {
                                id: state.id as u64,
                                start: range.start.0 as u64,
                                end: range.end.0 as u64,
                                reversed: state.reversed,
                            })
                            .collect(),
                    },
                ),
                Operation::RemoveSelections {
                    set_id,
                    lamport_timestamp,
                } => proto::operation::Variant::RemoveSelections(
                    proto::operation::RemoveSelections {
                        replica_id: set_id.replica_id as u32,
                        local_timestamp: set_id.value,
                        lamport_timestamp: lamport_timestamp.value,
                    },
                ),
                Operation::SetActiveSelections {
                    set_id,
                    lamport_timestamp,
                } => proto::operation::Variant::SetActiveSelections(
                    proto::operation::SetActiveSelections {
                        replica_id: lamport_timestamp.replica_id as u32,
                        local_timestamp: set_id.map(|set_id| set_id.value),
                        lamport_timestamp: lamport_timestamp.value,
                    },
                ),
                #[cfg(test)]
                Operation::Test(_) => unimplemented!(),
            }),
        }
    }
}

impl<'a> Into<proto::operation::Edit> for &'a EditOperation {
    fn into(self) -> proto::operation::Edit {
        let ranges = self
            .ranges
            .iter()
            .map(|range| proto::Range {
                start: range.start as u64,
                end: range.end as u64,
            })
            .collect();
        proto::operation::Edit {
            replica_id: self.timestamp.replica_id as u32,
            local_timestamp: self.timestamp.local,
            lamport_timestamp: self.timestamp.lamport,
            version: From::from(&self.version),
            ranges,
            new_text: self.new_text.clone(),
        }
    }
}

impl TryFrom<proto::Operation> for Operation {
    type Error = anyhow::Error;

    fn try_from(message: proto::Operation) -> Result<Self, Self::Error> {
        Ok(
            match message
                .variant
                .ok_or_else(|| anyhow!("missing operation variant"))?
            {
                proto::operation::Variant::Edit(edit) => Operation::Edit(edit.into()),
                proto::operation::Variant::Undo(undo) => Operation::Undo {
                    lamport_timestamp: clock::Lamport {
                        replica_id: undo.replica_id as ReplicaId,
                        value: undo.lamport_timestamp,
                    },
                    undo: UndoOperation {
                        id: clock::Local {
                            replica_id: undo.replica_id as ReplicaId,
                            value: undo.local_timestamp,
                        },
                        counts: undo
                            .counts
                            .into_iter()
                            .map(|c| {
                                (
                                    clock::Local {
                                        replica_id: c.replica_id as ReplicaId,
                                        value: c.local_timestamp,
                                    },
                                    c.count,
                                )
                            })
                            .collect(),
                        ranges: undo
                            .ranges
                            .into_iter()
                            .map(|r| r.start as usize..r.end as usize)
                            .collect(),
                        version: undo.version.into(),
                    },
                },
                proto::operation::Variant::UpdateSelections(message) => {
                    let version = message.version.into();
                    let entries = message
                        .selections
                        .iter()
                        .map(|selection| {
                            let range = (selection.start as usize, Bias::Left)
                                ..(selection.end as usize, Bias::Right);
                            let state = SelectionState {
                                id: selection.id as usize,
                                reversed: selection.reversed,
                                goal: SelectionGoal::None,
                            };
                            (range, state)
                        })
                        .collect();
                    let selections = AnchorRangeMap::from_raw(version, entries);

                    Operation::UpdateSelections {
                        set_id: clock::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value: message.local_timestamp,
                        },
                        lamport_timestamp: clock::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value: message.lamport_timestamp,
                        },
                        selections: Arc::from(selections),
                    }
                }
                proto::operation::Variant::RemoveSelections(message) => {
                    Operation::RemoveSelections {
                        set_id: clock::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value: message.local_timestamp,
                        },
                        lamport_timestamp: clock::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value: message.lamport_timestamp,
                        },
                    }
                }
                proto::operation::Variant::SetActiveSelections(message) => {
                    Operation::SetActiveSelections {
                        set_id: message.local_timestamp.map(|value| clock::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value,
                        }),
                        lamport_timestamp: clock::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value: message.lamport_timestamp,
                        },
                    }
                }
            },
        )
    }
}

impl From<proto::operation::Edit> for EditOperation {
    fn from(edit: proto::operation::Edit) -> Self {
        let ranges = edit
            .ranges
            .into_iter()
            .map(|range| range.start as usize..range.end as usize)
            .collect();
        EditOperation {
            timestamp: InsertionTimestamp {
                replica_id: edit.replica_id as ReplicaId,
                local: edit.local_timestamp,
                lamport: edit.lamport_timestamp,
            },
            version: edit.version.into(),
            ranges,
            new_text: edit.new_text,
        }
    }
}

pub trait ToOffset {
    fn to_offset<'a>(&self, content: impl Into<Content<'a>>) -> usize;
}

impl ToOffset for Point {
    fn to_offset<'a>(&self, content: impl Into<Content<'a>>) -> usize {
        content.into().visible_text.to_offset(*self)
    }
}

impl ToOffset for usize {
    fn to_offset<'a>(&self, _: impl Into<Content<'a>>) -> usize {
        *self
    }
}

impl ToOffset for Anchor {
    fn to_offset<'a>(&self, content: impl Into<Content<'a>>) -> usize {
        content.into().summary_for_anchor(self).bytes
    }
}

impl<'a> ToOffset for &'a Anchor {
    fn to_offset<'b>(&self, content: impl Into<Content<'b>>) -> usize {
        content.into().summary_for_anchor(self).bytes
    }
}

pub trait ToPoint {
    fn to_point<'a>(&self, content: impl Into<Content<'a>>) -> Point;
}

impl ToPoint for Anchor {
    fn to_point<'a>(&self, content: impl Into<Content<'a>>) -> Point {
        content.into().summary_for_anchor(self).lines
    }
}

impl ToPoint for usize {
    fn to_point<'a>(&self, content: impl Into<Content<'a>>) -> Point {
        content.into().visible_text.to_point(*self)
    }
}

impl ToPoint for Point {
    fn to_point<'a>(&self, _: impl Into<Content<'a>>) -> Point {
        *self
    }
}
