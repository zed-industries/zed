mod anchor;
pub mod locator;
#[cfg(any(test, feature = "test-support"))]
pub mod network;
pub mod operation_queue;
mod patch;
mod selection;
pub mod subscription;
#[cfg(test)]
mod tests;
mod undo_map;

pub use anchor::*;
use anyhow::{Context as _, Result};
use clock::Lamport;
pub use clock::ReplicaId;
use collections::{HashMap, HashSet};
use locator::Locator;
use operation_queue::OperationQueue;
pub use patch::Patch;
use postage::{oneshot, prelude::*};

use regex::Regex;
pub use rope::*;
pub use selection::*;
use std::{
    borrow::Cow,
    cmp::{self, Ordering, Reverse},
    fmt::Display,
    future::Future,
    iter::Iterator,
    num::NonZeroU64,
    ops::{self, Deref, Range, Sub},
    str,
    sync::{Arc, LazyLock},
    time::{Duration, Instant},
};
pub use subscription::*;
pub use sum_tree::Bias;
use sum_tree::{Dimensions, FilterCursor, SumTree, TreeMap, TreeSet};
use undo_map::UndoMap;
use util::debug_panic;

#[cfg(any(test, feature = "test-support"))]
use util::RandomCharIter;

static LINE_SEPARATORS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\r\n|\r").expect("Failed to create LINE_SEPARATORS_REGEX"));

pub type TransactionId = clock::Lamport;

pub struct Buffer {
    snapshot: BufferSnapshot,
    history: History,
    deferred_ops: OperationQueue<Operation>,
    deferred_replicas: HashSet<ReplicaId>,
    pub lamport_clock: clock::Lamport,
    subscriptions: Topic<usize>,
    edit_id_resolvers: HashMap<clock::Lamport, Vec<oneshot::Sender<()>>>,
    wait_for_version_txs: Vec<(clock::Global, oneshot::Sender<()>)>,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct BufferId(NonZeroU64);

impl Display for BufferId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<NonZeroU64> for BufferId {
    fn from(id: NonZeroU64) -> Self {
        BufferId(id)
    }
}

impl BufferId {
    /// Returns Err if `id` is outside of BufferId domain.
    pub fn new(id: u64) -> anyhow::Result<Self> {
        let id = NonZeroU64::new(id).context("Buffer id cannot be 0.")?;
        Ok(Self(id))
    }

    /// Increments this buffer id, returning the old value.
    /// So that's a post-increment operator in disguise.
    pub fn next(&mut self) -> Self {
        let old = *self;
        self.0 = self.0.saturating_add(1);
        old
    }

    pub fn to_proto(self) -> u64 {
        self.into()
    }
}

impl From<BufferId> for u64 {
    fn from(id: BufferId) -> Self {
        id.0.get()
    }
}

#[derive(Clone)]
pub struct BufferSnapshot {
    replica_id: ReplicaId,
    remote_id: BufferId,
    visible_text: Rope,
    deleted_text: Rope,
    line_ending: LineEnding,
    undo_map: UndoMap,
    fragments: SumTree<Fragment>,
    insertions: SumTree<InsertionFragment>,
    insertion_slices: TreeSet<InsertionSlice>,
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
    pub edit_ids: Vec<clock::Lamport>,
    pub start: clock::Global,
}

impl Transaction {
    pub fn merge_in(&mut self, other: Transaction) {
        self.edit_ids.extend(other.edit_ids);
    }
}

impl HistoryEntry {
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction.id
    }
}

struct History {
    base_text: Rope,
    operations: TreeMap<clock::Lamport, Operation>,
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    transaction_depth: usize,
    group_interval: Duration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InsertionSlice {
    edit_id: clock::Lamport,
    insertion_id: clock::Lamport,
    range: Range<usize>,
}

impl Ord for InsertionSlice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.edit_id
            .cmp(&other.edit_id)
            .then_with(|| self.insertion_id.cmp(&other.insertion_id))
            .then_with(|| self.range.start.cmp(&other.range.start))
            .then_with(|| self.range.end.cmp(&other.range.end))
    }
}

impl PartialOrd for InsertionSlice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl InsertionSlice {
    fn from_fragment(edit_id: clock::Lamport, fragment: &Fragment) -> Self {
        Self {
            edit_id,
            insertion_id: fragment.timestamp,
            range: fragment.insertion_offset..fragment.insertion_offset + fragment.len,
        }
    }
}

impl History {
    pub fn new(base_text: Rope) -> Self {
        Self {
            base_text,
            operations: Default::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            transaction_depth: 0,
            // Don't group transactions in tests unless we opt in, because it's a footgun.
            #[cfg(any(test, feature = "test-support"))]
            group_interval: Duration::ZERO,
            #[cfg(not(any(test, feature = "test-support")))]
            group_interval: Duration::from_millis(300),
        }
    }

    fn push(&mut self, op: Operation) {
        self.operations.insert(op.timestamp(), op);
    }

    fn start_transaction(
        &mut self,
        start: clock::Global,
        now: Instant,
        clock: &mut clock::Lamport,
    ) -> Option<TransactionId> {
        self.transaction_depth += 1;
        if self.transaction_depth == 1 {
            let id = clock.tick();
            self.undo_stack.push(HistoryEntry {
                transaction: Transaction {
                    id,
                    start,
                    edit_ids: Default::default(),
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
                .edit_ids
                .is_empty()
            {
                self.undo_stack.pop();
                None
            } else {
                self.redo_stack.clear();
                let entry = self.undo_stack.last_mut().unwrap();
                entry.last_edit_at = now;
                Some(entry)
            }
        } else {
            None
        }
    }

    fn group(&mut self) -> Option<TransactionId> {
        let mut count = 0;
        let mut entries = self.undo_stack.iter();
        if let Some(mut entry) = entries.next_back() {
            while let Some(prev_entry) = entries.next_back() {
                if !prev_entry.suppress_grouping
                    && entry.first_edit_at - prev_entry.last_edit_at < self.group_interval
                {
                    entry = prev_entry;
                    count += 1;
                } else {
                    break;
                }
            }
        }
        self.group_trailing(count)
    }

    fn group_until(&mut self, transaction_id: TransactionId) {
        let mut count = 0;
        for entry in self.undo_stack.iter().rev() {
            if entry.transaction_id() == transaction_id {
                self.group_trailing(count);
                break;
            } else if entry.suppress_grouping {
                break;
            } else {
                count += 1;
            }
        }
    }

    fn group_trailing(&mut self, n: usize) -> Option<TransactionId> {
        let new_len = self.undo_stack.len() - n;
        let (entries_to_keep, entries_to_merge) = self.undo_stack.split_at_mut(new_len);
        if let Some(last_entry) = entries_to_keep.last_mut() {
            for entry in &*entries_to_merge {
                for edit_id in &entry.transaction.edit_ids {
                    last_entry.transaction.edit_ids.push(*edit_id);
                }
            }

            if let Some(entry) = entries_to_merge.last_mut() {
                last_entry.last_edit_at = entry.last_edit_at;
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

    /// Differs from `push_transaction` in that it does not clear the redo
    /// stack. Intended to be used to create a parent transaction to merge
    /// potential child transactions into.
    ///
    /// The caller is responsible for removing it from the undo history using
    /// `forget_transaction` if no edits are merged into it. Otherwise, if edits
    /// are merged into this transaction, the caller is responsible for ensuring
    /// the redo stack is cleared. The easiest way to ensure the redo stack is
    /// cleared is to create transactions with the usual `start_transaction` and
    /// `end_transaction` methods and merging the resulting transactions into
    /// the transaction created by this method
    fn push_empty_transaction(
        &mut self,
        start: clock::Global,
        now: Instant,
        clock: &mut clock::Lamport,
    ) -> TransactionId {
        assert_eq!(self.transaction_depth, 0);
        let id = clock.tick();
        let transaction = Transaction {
            id,
            start,
            edit_ids: Vec::new(),
        };
        self.undo_stack.push(HistoryEntry {
            transaction,
            first_edit_at: now,
            last_edit_at: now,
            suppress_grouping: false,
        });
        id
    }

    fn push_undo(&mut self, op_id: clock::Lamport) {
        assert_ne!(self.transaction_depth, 0);
        if let Some(Operation::Edit(_)) = self.operations.get(&op_id) {
            let last_transaction = self.undo_stack.last_mut().unwrap();
            last_transaction.transaction.edit_ids.push(op_id);
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

    fn remove_from_undo(&mut self, transaction_id: TransactionId) -> Option<&HistoryEntry> {
        assert_eq!(self.transaction_depth, 0);

        let entry_ix = self
            .undo_stack
            .iter()
            .rposition(|entry| entry.transaction.id == transaction_id)?;
        let entry = self.undo_stack.remove(entry_ix);
        self.redo_stack.push(entry);
        self.redo_stack.last()
    }

    fn remove_from_undo_until(&mut self, transaction_id: TransactionId) -> &[HistoryEntry] {
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

    fn forget(&mut self, transaction_id: TransactionId) -> Option<Transaction> {
        assert_eq!(self.transaction_depth, 0);
        if let Some(entry_ix) = self
            .undo_stack
            .iter()
            .rposition(|entry| entry.transaction.id == transaction_id)
        {
            Some(self.undo_stack.remove(entry_ix).transaction)
        } else if let Some(entry_ix) = self
            .redo_stack
            .iter()
            .rposition(|entry| entry.transaction.id == transaction_id)
        {
            Some(self.redo_stack.remove(entry_ix).transaction)
        } else {
            None
        }
    }

    fn transaction(&self, transaction_id: TransactionId) -> Option<&Transaction> {
        let entry = self
            .undo_stack
            .iter()
            .rfind(|entry| entry.transaction.id == transaction_id)
            .or_else(|| {
                self.redo_stack
                    .iter()
                    .rfind(|entry| entry.transaction.id == transaction_id)
            })?;
        Some(&entry.transaction)
    }

    fn transaction_mut(&mut self, transaction_id: TransactionId) -> Option<&mut Transaction> {
        let entry = self
            .undo_stack
            .iter_mut()
            .rfind(|entry| entry.transaction.id == transaction_id)
            .or_else(|| {
                self.redo_stack
                    .iter_mut()
                    .rfind(|entry| entry.transaction.id == transaction_id)
            })?;
        Some(&mut entry.transaction)
    }

    fn merge_transactions(&mut self, transaction: TransactionId, destination: TransactionId) {
        if let Some(transaction) = self.forget(transaction)
            && let Some(destination) = self.transaction_mut(destination)
        {
            destination.edit_ids.extend(transaction.edit_ids);
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

struct Edits<'a, D: TextDimension, F: FnMut(&FragmentSummary) -> bool> {
    visible_cursor: rope::Cursor<'a>,
    deleted_cursor: rope::Cursor<'a>,
    fragments_cursor: Option<FilterCursor<'a, 'static, F, Fragment, FragmentTextSummary>>,
    undos: &'a UndoMap,
    since: &'a clock::Global,
    old_end: D,
    new_end: D,
    range: Range<(&'a Locator, usize)>,
    buffer_id: BufferId,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Edit<D> {
    pub old: Range<D>,
    pub new: Range<D>,
}
impl<D> Edit<D>
where
    D: PartialEq,
{
    pub fn is_empty(&self) -> bool {
        self.old.start == self.old.end && self.new.start == self.new.end
    }
}

impl<D, DDelta> Edit<D>
where
    D: Sub<D, Output = DDelta> + Copy,
{
    pub fn old_len(&self) -> DDelta {
        self.old.end - self.old.start
    }

    pub fn new_len(&self) -> DDelta {
        self.new.end - self.new.start
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

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Fragment {
    pub id: Locator,
    pub timestamp: clock::Lamport,
    pub insertion_offset: usize,
    pub len: usize,
    pub visible: bool,
    pub deletions: HashSet<clock::Lamport>,
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
    fn zero(_: &Option<clock::Global>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<clock::Global>) {
        self.visible += summary.text.visible;
        self.deleted += summary.text.deleted;
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct InsertionFragment {
    timestamp: clock::Lamport,
    split_offset: usize,
    fragment_id: Locator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct InsertionFragmentKey {
    timestamp: clock::Lamport,
    split_offset: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Edit(EditOperation),
    Undo(UndoOperation),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditOperation {
    pub timestamp: clock::Lamport,
    pub version: clock::Global,
    pub ranges: Vec<Range<FullOffset>>,
    pub new_text: Vec<Arc<str>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UndoOperation {
    pub timestamp: clock::Lamport,
    pub version: clock::Global,
    pub counts: HashMap<clock::Lamport, u32>,
}

/// Stores information about the indentation of a line (tabs and spaces).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineIndent {
    pub tabs: u32,
    pub spaces: u32,
    pub line_blank: bool,
}

impl LineIndent {
    pub fn from_chunks(chunks: &mut Chunks) -> Self {
        let mut tabs = 0;
        let mut spaces = 0;
        let mut line_blank = true;

        'outer: while let Some(chunk) = chunks.peek() {
            for ch in chunk.chars() {
                if ch == '\t' {
                    tabs += 1;
                } else if ch == ' ' {
                    spaces += 1;
                } else {
                    if ch != '\n' {
                        line_blank = false;
                    }
                    break 'outer;
                }
            }

            chunks.next();
        }

        Self {
            tabs,
            spaces,
            line_blank,
        }
    }

    /// Constructs a new `LineIndent` which only contains spaces.
    pub fn spaces(spaces: u32) -> Self {
        Self {
            tabs: 0,
            spaces,
            line_blank: true,
        }
    }

    /// Constructs a new `LineIndent` which only contains tabs.
    pub fn tabs(tabs: u32) -> Self {
        Self {
            tabs,
            spaces: 0,
            line_blank: true,
        }
    }

    /// Indicates whether the line is empty.
    pub fn is_line_empty(&self) -> bool {
        self.tabs == 0 && self.spaces == 0 && self.line_blank
    }

    /// Indicates whether the line is blank (contains only whitespace).
    pub fn is_line_blank(&self) -> bool {
        self.line_blank
    }

    /// Returns the number of indentation characters (tabs or spaces).
    pub fn raw_len(&self) -> u32 {
        self.tabs + self.spaces
    }

    /// Returns the number of indentation characters (tabs or spaces), taking tab size into account.
    pub fn len(&self, tab_size: u32) -> u32 {
        self.tabs * tab_size + self.spaces
    }
}

impl From<&str> for LineIndent {
    fn from(value: &str) -> Self {
        Self::from_iter(value.chars())
    }
}

impl FromIterator<char> for LineIndent {
    fn from_iter<T: IntoIterator<Item = char>>(chars: T) -> Self {
        let mut tabs = 0;
        let mut spaces = 0;
        let mut line_blank = true;
        for c in chars {
            if c == '\t' {
                tabs += 1;
            } else if c == ' ' {
                spaces += 1;
            } else {
                if c != '\n' {
                    line_blank = false;
                }
                break;
            }
        }
        Self {
            tabs,
            spaces,
            line_blank,
        }
    }
}

impl Buffer {
    pub fn new(replica_id: ReplicaId, remote_id: BufferId, base_text: impl Into<String>) -> Buffer {
        let mut base_text = base_text.into();
        let line_ending = LineEnding::detect(&base_text);
        LineEnding::normalize(&mut base_text);
        Self::new_normalized(replica_id, remote_id, line_ending, Rope::from(&*base_text))
    }

    pub fn new_normalized(
        replica_id: ReplicaId,
        remote_id: BufferId,
        line_ending: LineEnding,
        normalized: Rope,
    ) -> Buffer {
        let history = History::new(normalized);
        let mut fragments = SumTree::new(&None);
        let mut insertions = SumTree::default();

        let mut lamport_clock = clock::Lamport::new(replica_id);
        let mut version = clock::Global::new();

        let visible_text = history.base_text.clone();
        if !visible_text.is_empty() {
            let insertion_timestamp = clock::Lamport::new(ReplicaId::LOCAL);
            lamport_clock.observe(insertion_timestamp);
            version.observe(insertion_timestamp);
            let fragment_id = Locator::between(&Locator::min(), &Locator::max());
            let fragment = Fragment {
                id: fragment_id,
                timestamp: insertion_timestamp,
                insertion_offset: 0,
                len: visible_text.len(),
                visible: true,
                deletions: Default::default(),
                max_undos: Default::default(),
            };
            insertions.push(InsertionFragment::new(&fragment), ());
            fragments.push(fragment, &None);
        }

        Buffer {
            snapshot: BufferSnapshot {
                replica_id,
                remote_id,
                visible_text,
                deleted_text: Rope::new(),
                line_ending,
                fragments,
                insertions,
                version,
                undo_map: Default::default(),
                insertion_slices: Default::default(),
            },
            history,
            deferred_ops: OperationQueue::new(),
            deferred_replicas: HashSet::default(),
            lamport_clock,
            subscriptions: Default::default(),
            edit_id_resolvers: Default::default(),
            wait_for_version_txs: Default::default(),
        }
    }

    pub fn version(&self) -> clock::Global {
        self.version.clone()
    }

    pub fn snapshot(&self) -> BufferSnapshot {
        self.snapshot.clone()
    }

    pub fn branch(&self) -> Self {
        Self {
            snapshot: self.snapshot.clone(),
            history: History::new(self.base_text().clone()),
            deferred_ops: OperationQueue::new(),
            deferred_replicas: HashSet::default(),
            lamport_clock: clock::Lamport::new(ReplicaId::LOCAL_BRANCH),
            subscriptions: Default::default(),
            edit_id_resolvers: Default::default(),
            wait_for_version_txs: Default::default(),
        }
    }

    pub fn replica_id(&self) -> ReplicaId {
        self.lamport_clock.replica_id
    }

    pub fn remote_id(&self) -> BufferId {
        self.remote_id
    }

    pub fn deferred_ops_len(&self) -> usize {
        self.deferred_ops.len()
    }

    pub fn transaction_group_interval(&self) -> Duration {
        self.history.group_interval
    }

    pub fn edit<R, I, S, T>(&mut self, edits: R) -> Operation
    where
        R: IntoIterator<IntoIter = I>,
        I: ExactSizeIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        let edits = edits
            .into_iter()
            .map(|(range, new_text)| (range, new_text.into()));

        self.start_transaction();
        let timestamp = self.lamport_clock.tick();
        let operation = Operation::Edit(self.apply_local_edit(edits, timestamp));

        self.history.push(operation.clone());
        self.history.push_undo(operation.timestamp());
        self.snapshot.version.observe(operation.timestamp());
        self.end_transaction();
        operation
    }

    fn apply_local_edit<S: ToOffset, T: Into<Arc<str>>>(
        &mut self,
        edits: impl ExactSizeIterator<Item = (Range<S>, T)>,
        timestamp: clock::Lamport,
    ) -> EditOperation {
        let mut edits_patch = Patch::default();
        let mut edit_op = EditOperation {
            timestamp,
            version: self.version(),
            ranges: Vec::with_capacity(edits.len()),
            new_text: Vec::with_capacity(edits.len()),
        };
        let mut new_insertions = Vec::new();
        let mut insertion_offset = 0;
        let mut insertion_slices = Vec::new();

        let mut edits = edits
            .map(|(range, new_text)| (range.to_offset(&*self), new_text))
            .peekable();

        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));
        let mut old_fragments = self.fragments.cursor::<FragmentTextSummary>(&None);
        let mut new_fragments = old_fragments.slice(&edits.peek().unwrap().0.start, Bias::Right);
        new_ropes.append(new_fragments.summary().text);

        let mut fragment_start = old_fragments.start().visible;
        for (range, new_text) in edits {
            let new_text = LineEnding::normalize_arc(new_text.into());
            let fragment_end = old_fragments.end().visible;

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
                    old_fragments.next();
                }

                let slice = old_fragments.slice(&range.start, Bias::Right);
                new_ropes.append(slice.summary().text);
                new_fragments.append(slice, &None);
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
            if !new_text.is_empty() {
                let new_start = new_fragments.summary().text.visible;

                let fragment = Fragment {
                    id: Locator::between(
                        &new_fragments.summary().max_id,
                        old_fragments
                            .item()
                            .map_or(&Locator::max(), |old_fragment| &old_fragment.id),
                    ),
                    timestamp,
                    insertion_offset,
                    len: new_text.len(),
                    deletions: Default::default(),
                    max_undos: Default::default(),
                    visible: true,
                };
                edits_patch.push(Edit {
                    old: fragment_start..fragment_start,
                    new: new_start..new_start + new_text.len(),
                });
                insertion_slices.push(InsertionSlice::from_fragment(timestamp, &fragment));
                new_insertions.push(InsertionFragment::insert_new(&fragment));
                new_ropes.push_str(new_text.as_ref());
                new_fragments.push(fragment, &None);
                insertion_offset += new_text.len();
            }

            // Advance through every fragment that intersects this range, marking the intersecting
            // portions as deleted.
            while fragment_start < range.end {
                let fragment = old_fragments.item().unwrap();
                let fragment_end = old_fragments.end().visible;
                let mut intersection = fragment.clone();
                let intersection_end = cmp::min(range.end, fragment_end);
                if fragment.visible {
                    intersection.len = intersection_end - fragment_start;
                    intersection.insertion_offset += fragment_start - old_fragments.start().visible;
                    intersection.id =
                        Locator::between(&new_fragments.summary().max_id, &intersection.id);
                    intersection.deletions.insert(timestamp);
                    intersection.visible = false;
                }
                if intersection.len > 0 {
                    if fragment.visible && !intersection.visible {
                        let new_start = new_fragments.summary().text.visible;
                        edits_patch.push(Edit {
                            old: fragment_start..intersection_end,
                            new: new_start..new_start,
                        });
                        insertion_slices
                            .push(InsertionSlice::from_fragment(timestamp, &intersection));
                    }
                    new_insertions.push(InsertionFragment::insert_new(&intersection));
                    new_ropes.push_fragment(&intersection, fragment.visible);
                    new_fragments.push(intersection, &None);
                    fragment_start = intersection_end;
                }
                if fragment_end <= range.end {
                    old_fragments.next();
                }
            }

            let full_range_end = FullOffset(range.end + old_fragments.start().deleted);
            edit_op.ranges.push(full_range_start..full_range_end);
            edit_op.new_text.push(new_text);
        }

        // If the current fragment has been partially consumed, then consume the rest of it
        // and advance to the next fragment before slicing.
        if fragment_start > old_fragments.start().visible {
            let fragment_end = old_fragments.end().visible;
            if fragment_end > fragment_start {
                let mut suffix = old_fragments.item().unwrap().clone();
                suffix.len = fragment_end - fragment_start;
                suffix.insertion_offset += fragment_start - old_fragments.start().visible;
                new_insertions.push(InsertionFragment::insert_new(&suffix));
                new_ropes.push_fragment(&suffix, suffix.visible);
                new_fragments.push(suffix, &None);
            }
            old_fragments.next();
        }

        let suffix = old_fragments.suffix();
        new_ropes.append(suffix.summary().text);
        new_fragments.append(suffix, &None);
        let (visible_text, deleted_text) = new_ropes.finish();
        drop(old_fragments);

        self.snapshot.fragments = new_fragments;
        self.snapshot.insertions.edit(new_insertions, ());
        self.snapshot.visible_text = visible_text;
        self.snapshot.deleted_text = deleted_text;
        self.subscriptions.publish_mut(&edits_patch);
        self.snapshot.insertion_slices.extend(insertion_slices);
        edit_op
    }

    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.snapshot.line_ending = line_ending;
    }

    pub fn apply_ops<I: IntoIterator<Item = Operation>>(&mut self, ops: I) {
        let mut deferred_ops = Vec::new();
        for op in ops {
            self.history.push(op.clone());
            if self.can_apply_op(&op) {
                self.apply_op(op);
            } else {
                self.deferred_replicas.insert(op.replica_id());
                deferred_ops.push(op);
            }
        }
        self.deferred_ops.insert(deferred_ops);
        self.flush_deferred_ops();
    }

    fn apply_op(&mut self, op: Operation) {
        match op {
            Operation::Edit(edit) => {
                if !self.version.observed(edit.timestamp) {
                    self.apply_remote_edit(
                        &edit.version,
                        &edit.ranges,
                        &edit.new_text,
                        edit.timestamp,
                    );
                    self.snapshot.version.observe(edit.timestamp);
                    self.lamport_clock.observe(edit.timestamp);
                    self.resolve_edit(edit.timestamp);
                }
            }
            Operation::Undo(undo) => {
                if !self.version.observed(undo.timestamp) {
                    self.apply_undo(&undo);
                    self.snapshot.version.observe(undo.timestamp);
                    self.lamport_clock.observe(undo.timestamp);
                }
            }
        }
        self.wait_for_version_txs.retain_mut(|(version, tx)| {
            if self.snapshot.version().observed_all(version) {
                tx.try_send(()).ok();
                false
            } else {
                true
            }
        });
    }

    fn apply_remote_edit(
        &mut self,
        version: &clock::Global,
        ranges: &[Range<FullOffset>],
        new_text: &[Arc<str>],
        timestamp: clock::Lamport,
    ) {
        if ranges.is_empty() {
            return;
        }

        let edits = ranges.iter().zip(new_text.iter());
        let mut edits_patch = Patch::default();
        let mut insertion_slices = Vec::new();
        let cx = Some(version.clone());
        let mut new_insertions = Vec::new();
        let mut insertion_offset = 0;
        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));
        let mut old_fragments = self
            .fragments
            .cursor::<Dimensions<VersionedFullOffset, usize>>(&cx);
        let mut new_fragments =
            old_fragments.slice(&VersionedFullOffset::Offset(ranges[0].start), Bias::Left);
        new_ropes.append(new_fragments.summary().text);

        let mut fragment_start = old_fragments.start().0.full_offset();
        for (range, new_text) in edits {
            let fragment_end = old_fragments.end().0.full_offset();

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
                    old_fragments.next();
                }

                let slice =
                    old_fragments.slice(&VersionedFullOffset::Offset(range.start), Bias::Left);
                new_ropes.append(slice.summary().text);
                new_fragments.append(slice, &None);
                fragment_start = old_fragments.start().0.full_offset();
            }

            // If we are at the end of a non-concurrent fragment, advance to the next one.
            let fragment_end = old_fragments.end().0.full_offset();
            if fragment_end == range.start && fragment_end > fragment_start {
                let mut fragment = old_fragments.item().unwrap().clone();
                fragment.len = fragment_end.0 - fragment_start.0;
                fragment.insertion_offset += fragment_start - old_fragments.start().0.full_offset();
                new_insertions.push(InsertionFragment::insert_new(&fragment));
                new_ropes.push_fragment(&fragment, fragment.visible);
                new_fragments.push(fragment, &None);
                old_fragments.next();
                fragment_start = old_fragments.start().0.full_offset();
            }

            // Skip over insertions that are concurrent to this edit, but have a lower lamport
            // timestamp.
            while let Some(fragment) = old_fragments.item() {
                if fragment_start == range.start && fragment.timestamp > timestamp {
                    new_ropes.push_fragment(fragment, fragment.visible);
                    new_fragments.push(fragment.clone(), &None);
                    old_fragments.next();
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
            if !new_text.is_empty() {
                let mut old_start = old_fragments.start().1;
                if old_fragments.item().is_some_and(|f| f.visible) {
                    old_start += fragment_start.0 - old_fragments.start().0.full_offset().0;
                }
                let new_start = new_fragments.summary().text.visible;
                let fragment = Fragment {
                    id: Locator::between(
                        &new_fragments.summary().max_id,
                        old_fragments
                            .item()
                            .map_or(&Locator::max(), |old_fragment| &old_fragment.id),
                    ),
                    timestamp,
                    insertion_offset,
                    len: new_text.len(),
                    deletions: Default::default(),
                    max_undos: Default::default(),
                    visible: true,
                };
                edits_patch.push(Edit {
                    old: old_start..old_start,
                    new: new_start..new_start + new_text.len(),
                });
                insertion_slices.push(InsertionSlice::from_fragment(timestamp, &fragment));
                new_insertions.push(InsertionFragment::insert_new(&fragment));
                new_ropes.push_str(new_text);
                new_fragments.push(fragment, &None);
                insertion_offset += new_text.len();
            }

            // Advance through every fragment that intersects this range, marking the intersecting
            // portions as deleted.
            while fragment_start < range.end {
                let fragment = old_fragments.item().unwrap();
                let fragment_end = old_fragments.end().0.full_offset();
                let mut intersection = fragment.clone();
                let intersection_end = cmp::min(range.end, fragment_end);
                if fragment.was_visible(version, &self.undo_map) {
                    intersection.len = intersection_end.0 - fragment_start.0;
                    intersection.insertion_offset +=
                        fragment_start - old_fragments.start().0.full_offset();
                    intersection.id =
                        Locator::between(&new_fragments.summary().max_id, &intersection.id);
                    intersection.deletions.insert(timestamp);
                    intersection.visible = false;
                    insertion_slices.push(InsertionSlice::from_fragment(timestamp, &intersection));
                }
                if intersection.len > 0 {
                    if fragment.visible && !intersection.visible {
                        let old_start = old_fragments.start().1
                            + (fragment_start.0 - old_fragments.start().0.full_offset().0);
                        let new_start = new_fragments.summary().text.visible;
                        edits_patch.push(Edit {
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
                    old_fragments.next();
                }
            }
        }

        // If the current fragment has been partially consumed, then consume the rest of it
        // and advance to the next fragment before slicing.
        if fragment_start > old_fragments.start().0.full_offset() {
            let fragment_end = old_fragments.end().0.full_offset();
            if fragment_end > fragment_start {
                let mut suffix = old_fragments.item().unwrap().clone();
                suffix.len = fragment_end.0 - fragment_start.0;
                suffix.insertion_offset += fragment_start - old_fragments.start().0.full_offset();
                new_insertions.push(InsertionFragment::insert_new(&suffix));
                new_ropes.push_fragment(&suffix, suffix.visible);
                new_fragments.push(suffix, &None);
            }
            old_fragments.next();
        }

        let suffix = old_fragments.suffix();
        new_ropes.append(suffix.summary().text);
        new_fragments.append(suffix, &None);
        let (visible_text, deleted_text) = new_ropes.finish();
        drop(old_fragments);

        self.snapshot.fragments = new_fragments;
        self.snapshot.visible_text = visible_text;
        self.snapshot.deleted_text = deleted_text;
        self.snapshot.insertions.edit(new_insertions, ());
        self.snapshot.insertion_slices.extend(insertion_slices);
        self.subscriptions.publish_mut(&edits_patch)
    }

    fn fragment_ids_for_edits<'a>(
        &'a self,
        edit_ids: impl Iterator<Item = &'a clock::Lamport>,
    ) -> Vec<&'a Locator> {
        // Get all of the insertion slices changed by the given edits.
        let mut insertion_slices = Vec::new();
        for edit_id in edit_ids {
            let insertion_slice = InsertionSlice {
                edit_id: *edit_id,
                insertion_id: clock::Lamport::MIN,
                range: 0..0,
            };
            let slices = self
                .snapshot
                .insertion_slices
                .iter_from(&insertion_slice)
                .take_while(|slice| slice.edit_id == *edit_id);
            insertion_slices.extend(slices)
        }
        insertion_slices
            .sort_unstable_by_key(|s| (s.insertion_id, s.range.start, Reverse(s.range.end)));

        // Get all of the fragments corresponding to these insertion slices.
        let mut fragment_ids = Vec::new();
        let mut insertions_cursor = self.insertions.cursor::<InsertionFragmentKey>(());
        for insertion_slice in &insertion_slices {
            if insertion_slice.insertion_id != insertions_cursor.start().timestamp
                || insertion_slice.range.start > insertions_cursor.start().split_offset
            {
                insertions_cursor.seek_forward(
                    &InsertionFragmentKey {
                        timestamp: insertion_slice.insertion_id,
                        split_offset: insertion_slice.range.start,
                    },
                    Bias::Left,
                );
            }
            while let Some(item) = insertions_cursor.item() {
                if item.timestamp != insertion_slice.insertion_id
                    || item.split_offset >= insertion_slice.range.end
                {
                    break;
                }
                fragment_ids.push(&item.fragment_id);
                insertions_cursor.next();
            }
        }
        fragment_ids.sort_unstable();
        fragment_ids
    }

    fn apply_undo(&mut self, undo: &UndoOperation) {
        self.snapshot.undo_map.insert(undo);

        let mut edits = Patch::default();
        let mut old_fragments = self
            .fragments
            .cursor::<Dimensions<Option<&Locator>, usize>>(&None);
        let mut new_fragments = SumTree::new(&None);
        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));

        for fragment_id in self.fragment_ids_for_edits(undo.counts.keys()) {
            let preceding_fragments = old_fragments.slice(&Some(fragment_id), Bias::Left);
            new_ropes.append(preceding_fragments.summary().text);
            new_fragments.append(preceding_fragments, &None);

            if let Some(fragment) = old_fragments.item() {
                let mut fragment = fragment.clone();
                let fragment_was_visible = fragment.visible;

                fragment.visible = fragment.is_visible(&self.undo_map);
                fragment.max_undos.observe(undo.timestamp);

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

                old_fragments.next();
            }
        }

        let suffix = old_fragments.suffix();
        new_ropes.append(suffix.summary().text);
        new_fragments.append(suffix, &None);

        drop(old_fragments);
        let (visible_text, deleted_text) = new_ropes.finish();
        self.snapshot.fragments = new_fragments;
        self.snapshot.visible_text = visible_text;
        self.snapshot.deleted_text = deleted_text;
        self.subscriptions.publish_mut(&edits);
    }

    fn flush_deferred_ops(&mut self) {
        self.deferred_replicas.clear();
        let mut deferred_ops = Vec::new();
        for op in self.deferred_ops.drain().iter().cloned() {
            if self.can_apply_op(&op) {
                self.apply_op(op);
            } else {
                self.deferred_replicas.insert(op.replica_id());
                deferred_ops.push(op);
            }
        }
        self.deferred_ops.insert(deferred_ops);
    }

    fn can_apply_op(&self, op: &Operation) -> bool {
        if self.deferred_replicas.contains(&op.replica_id()) {
            false
        } else {
            self.version.observed_all(match op {
                Operation::Edit(edit) => &edit.version,
                Operation::Undo(undo) => &undo.version,
            })
        }
    }

    pub fn has_deferred_ops(&self) -> bool {
        !self.deferred_ops.is_empty()
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
            .start_transaction(self.version.clone(), now, &mut self.lamport_clock)
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

    pub fn group_until_transaction(&mut self, transaction_id: TransactionId) {
        self.history.group_until(transaction_id);
    }

    pub fn base_text(&self) -> &Rope {
        &self.history.base_text
    }

    pub fn operations(&self) -> &TreeMap<clock::Lamport, Operation> {
        &self.history.operations
    }

    pub fn undo(&mut self) -> Option<(TransactionId, Operation)> {
        if let Some(entry) = self.history.pop_undo() {
            let transaction = entry.transaction.clone();
            let transaction_id = transaction.id;
            let op = self.undo_or_redo(transaction);
            Some((transaction_id, op))
        } else {
            None
        }
    }

    pub fn undo_transaction(&mut self, transaction_id: TransactionId) -> Option<Operation> {
        let transaction = self
            .history
            .remove_from_undo(transaction_id)?
            .transaction
            .clone();
        Some(self.undo_or_redo(transaction))
    }

    pub fn undo_to_transaction(&mut self, transaction_id: TransactionId) -> Vec<Operation> {
        let transactions = self
            .history
            .remove_from_undo_until(transaction_id)
            .iter()
            .map(|entry| entry.transaction.clone())
            .collect::<Vec<_>>();

        transactions
            .into_iter()
            .map(|transaction| self.undo_or_redo(transaction))
            .collect()
    }

    pub fn forget_transaction(&mut self, transaction_id: TransactionId) -> Option<Transaction> {
        self.history.forget(transaction_id)
    }

    pub fn get_transaction(&self, transaction_id: TransactionId) -> Option<&Transaction> {
        self.history.transaction(transaction_id)
    }

    pub fn merge_transactions(&mut self, transaction: TransactionId, destination: TransactionId) {
        self.history.merge_transactions(transaction, destination);
    }

    pub fn redo(&mut self) -> Option<(TransactionId, Operation)> {
        if let Some(entry) = self.history.pop_redo() {
            let transaction = entry.transaction.clone();
            let transaction_id = transaction.id;
            let op = self.undo_or_redo(transaction);
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
            .map(|transaction| self.undo_or_redo(transaction))
            .collect()
    }

    fn undo_or_redo(&mut self, transaction: Transaction) -> Operation {
        let mut counts = HashMap::default();
        for edit_id in transaction.edit_ids {
            counts.insert(edit_id, self.undo_map.undo_count(edit_id).saturating_add(1));
        }

        let operation = self.undo_operations(counts);
        self.history.push(operation.clone());
        operation
    }

    pub fn undo_operations(&mut self, counts: HashMap<clock::Lamport, u32>) -> Operation {
        let timestamp = self.lamport_clock.tick();
        let version = self.version();
        self.snapshot.version.observe(timestamp);
        let undo = UndoOperation {
            timestamp,
            version,
            counts,
        };
        self.apply_undo(&undo);
        Operation::Undo(undo)
    }

    pub fn push_transaction(&mut self, transaction: Transaction, now: Instant) {
        self.history.push_transaction(transaction, now);
    }

    /// Differs from `push_transaction` in that it does not clear the redo stack.
    /// The caller responsible for
    /// Differs from `push_transaction` in that it does not clear the redo
    /// stack. Intended to be used to create a parent transaction to merge
    /// potential child transactions into.
    ///
    /// The caller is responsible for removing it from the undo history using
    /// `forget_transaction` if no edits are merged into it. Otherwise, if edits
    /// are merged into this transaction, the caller is responsible for ensuring
    /// the redo stack is cleared. The easiest way to ensure the redo stack is
    /// cleared is to create transactions with the usual `start_transaction` and
    /// `end_transaction` methods and merging the resulting transactions into
    /// the transaction created by this method
    pub fn push_empty_transaction(&mut self, now: Instant) -> TransactionId {
        self.history
            .push_empty_transaction(self.version.clone(), now, &mut self.lamport_clock)
    }

    pub fn edited_ranges_for_transaction_id<D>(
        &self,
        transaction_id: TransactionId,
    ) -> impl '_ + Iterator<Item = Range<D>>
    where
        D: TextDimension,
    {
        self.history
            .transaction(transaction_id)
            .into_iter()
            .flat_map(|transaction| self.edited_ranges_for_transaction(transaction))
    }

    pub fn edited_ranges_for_edit_ids<'a, D>(
        &'a self,
        edit_ids: impl IntoIterator<Item = &'a clock::Lamport>,
    ) -> impl 'a + Iterator<Item = Range<D>>
    where
        D: TextDimension,
    {
        // get fragment ranges
        let mut cursor = self
            .fragments
            .cursor::<Dimensions<Option<&Locator>, usize>>(&None);
        let offset_ranges = self
            .fragment_ids_for_edits(edit_ids.into_iter())
            .into_iter()
            .filter_map(move |fragment_id| {
                cursor.seek_forward(&Some(fragment_id), Bias::Left);
                let fragment = cursor.item()?;
                let start_offset = cursor.start().1;
                let end_offset = start_offset + if fragment.visible { fragment.len } else { 0 };
                Some(start_offset..end_offset)
            });

        // combine adjacent ranges
        let mut prev_range: Option<Range<usize>> = None;
        let disjoint_ranges = offset_ranges
            .map(Some)
            .chain([None])
            .filter_map(move |range| {
                if let Some((range, prev_range)) = range.as_ref().zip(prev_range.as_mut())
                    && prev_range.end == range.start
                {
                    prev_range.end = range.end;
                    return None;
                }
                let result = prev_range.clone();
                prev_range = range;
                result
            });

        // convert to the desired text dimension.
        let mut position = D::zero(());
        let mut rope_cursor = self.visible_text.cursor(0);
        disjoint_ranges.map(move |range| {
            position.add_assign(&rope_cursor.summary(range.start));
            let start = position;
            position.add_assign(&rope_cursor.summary(range.end));
            let end = position;
            start..end
        })
    }

    pub fn edited_ranges_for_transaction<'a, D>(
        &'a self,
        transaction: &'a Transaction,
    ) -> impl 'a + Iterator<Item = Range<D>>
    where
        D: TextDimension,
    {
        self.edited_ranges_for_edit_ids(&transaction.edit_ids)
    }

    pub fn subscribe(&mut self) -> Subscription<usize> {
        self.subscriptions.subscribe()
    }

    pub fn wait_for_edits<It: IntoIterator<Item = clock::Lamport>>(
        &mut self,
        edit_ids: It,
    ) -> impl 'static + Future<Output = Result<()>> + use<It> {
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
                if future.recv().await.is_none() {
                    anyhow::bail!("gave up waiting for edits");
                }
            }
            Ok(())
        }
    }

    pub fn wait_for_anchors<It: IntoIterator<Item = Anchor>>(
        &mut self,
        anchors: It,
    ) -> impl 'static + Future<Output = Result<()>> + use<It> {
        let mut futures = Vec::new();
        for anchor in anchors {
            if !self.version.observed(anchor.timestamp) && !anchor.is_max() && !anchor.is_min() {
                let (tx, rx) = oneshot::channel();
                self.edit_id_resolvers
                    .entry(anchor.timestamp)
                    .or_default()
                    .push(tx);
                futures.push(rx);
            }
        }

        async move {
            for mut future in futures {
                if future.recv().await.is_none() {
                    anyhow::bail!("gave up waiting for anchors");
                }
            }
            Ok(())
        }
    }

    pub fn wait_for_version(
        &mut self,
        version: clock::Global,
    ) -> impl Future<Output = Result<()>> + use<> {
        let mut rx = None;
        if !self.snapshot.version.observed_all(&version) {
            let channel = oneshot::channel();
            self.wait_for_version_txs.push((version, channel.0));
            rx = Some(channel.1);
        }
        async move {
            if let Some(mut rx) = rx
                && rx.recv().await.is_none()
            {
                anyhow::bail!("gave up waiting for version");
            }
            Ok(())
        }
    }

    pub fn give_up_waiting(&mut self) {
        self.edit_id_resolvers.clear();
        self.wait_for_version_txs.clear();
    }

    fn resolve_edit(&mut self, edit_id: clock::Lamport) {
        for mut tx in self
            .edit_id_resolvers
            .remove(&edit_id)
            .into_iter()
            .flatten()
        {
            tx.try_send(()).ok();
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Buffer {
    #[track_caller]
    pub fn edit_via_marked_text(&mut self, marked_string: &str) {
        let edits = self.edits_for_marked_text(marked_string);
        self.edit(edits);
    }

    #[track_caller]
    pub fn edits_for_marked_text(&self, marked_string: &str) -> Vec<(Range<usize>, String)> {
        let old_text = self.text();
        let (new_text, mut ranges) = util::test::marked_text_ranges(marked_string, false);
        if ranges.is_empty() {
            ranges.push(0..new_text.len());
        }

        assert_eq!(
            old_text[..ranges[0].start],
            new_text[..ranges[0].start],
            "invalid edit"
        );

        let mut delta = 0;
        let mut edits = Vec::new();
        let mut ranges = ranges.into_iter().peekable();

        while let Some(inserted_range) = ranges.next() {
            let new_start = inserted_range.start;
            let old_start = (new_start as isize - delta) as usize;

            let following_text = if let Some(next_range) = ranges.peek() {
                &new_text[inserted_range.end..next_range.start]
            } else {
                &new_text[inserted_range.end..]
            };

            let inserted_len = inserted_range.len();
            let deleted_len = old_text[old_start..]
                .find(following_text)
                .expect("invalid edit");

            let old_range = old_start..old_start + deleted_len;
            edits.push((old_range, new_text[inserted_range].to_string()));
            delta += inserted_len as isize - deleted_len as isize;
        }

        assert_eq!(
            old_text.len() as isize + delta,
            new_text.len() as isize,
            "invalid edit"
        );

        edits
    }

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
                        timestamp: fragment.timestamp,
                        split_offset: fragment.insertion_offset,
                    },
                    (),
                )
                .unwrap();
            assert_eq!(
                insertion_fragment.fragment_id, fragment.id,
                "fragment: {:?}\ninsertion: {:?}",
                fragment, insertion_fragment
            );
        }

        let mut cursor = self.snapshot.fragments.cursor::<Option<&Locator>>(&None);
        for insertion_fragment in self.snapshot.insertions.cursor::<()>(()) {
            cursor.seek(&Some(&insertion_fragment.fragment_id), Bias::Left);
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

        assert!(!self.text().contains("\r\n"));
    }

    pub fn set_group_interval(&mut self, group_interval: Duration) {
        self.history.group_interval = group_interval;
    }

    pub fn random_byte_range(&self, start_offset: usize, rng: &mut impl rand::Rng) -> Range<usize> {
        let end = self.clip_offset(rng.random_range(start_offset..=self.len()), Bias::Right);
        let start = self.clip_offset(rng.random_range(start_offset..=end), Bias::Right);
        start..end
    }

    pub fn get_random_edits<T>(
        &self,
        rng: &mut T,
        edit_count: usize,
    ) -> Vec<(Range<usize>, Arc<str>)>
    where
        T: rand::Rng,
    {
        let mut edits: Vec<(Range<usize>, Arc<str>)> = Vec::new();
        let mut last_end = None;
        for _ in 0..edit_count {
            if last_end.is_some_and(|last_end| last_end >= self.len()) {
                break;
            }
            let new_start = last_end.map_or(0, |last_end| last_end + 1);
            let range = self.random_byte_range(new_start, rng);
            last_end = Some(range.end);

            let new_text_len = rng.random_range(0..10);
            let new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();

            edits.push((range, new_text.into()));
        }
        edits
    }

    pub fn randomly_edit<T>(
        &mut self,
        rng: &mut T,
        edit_count: usize,
    ) -> (Vec<(Range<usize>, Arc<str>)>, Operation)
    where
        T: rand::Rng,
    {
        let mut edits = self.get_random_edits(rng, edit_count);
        log::info!("mutating buffer {:?} with {:?}", self.replica_id, edits);

        let op = self.edit(edits.iter().cloned());
        if let Operation::Edit(edit) = &op {
            assert_eq!(edits.len(), edit.new_text.len());
            for (edit, new_text) in edits.iter_mut().zip(&edit.new_text) {
                edit.1 = new_text.clone();
            }
        } else {
            unreachable!()
        }

        (edits, op)
    }

    pub fn randomly_undo_redo(&mut self, rng: &mut impl rand::Rng) -> Vec<Operation> {
        use rand::prelude::*;

        let mut ops = Vec::new();
        for _ in 0..rng.random_range(1..=5) {
            if let Some(entry) = self.history.undo_stack.choose(rng) {
                let transaction = entry.transaction.clone();
                log::info!(
                    "undoing buffer {:?} transaction {:?}",
                    self.replica_id,
                    transaction
                );
                ops.push(self.undo_or_redo(transaction));
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

    pub fn rope_for_version(&self, version: &clock::Global) -> Rope {
        let mut rope = Rope::new();

        let mut cursor = self
            .fragments
            .filter::<_, FragmentTextSummary>(&None, move |summary| {
                !version.observed_all(&summary.max_version)
            });
        cursor.next();

        let mut visible_cursor = self.visible_text.cursor(0);
        let mut deleted_cursor = self.deleted_text.cursor(0);

        while let Some(fragment) = cursor.item() {
            if cursor.start().visible > visible_cursor.offset() {
                let text = visible_cursor.slice(cursor.start().visible);
                rope.append(text);
            }

            if fragment.was_visible(version, &self.undo_map) {
                if fragment.visible {
                    let text = visible_cursor.slice(cursor.end().visible);
                    rope.append(text);
                } else {
                    deleted_cursor.seek_forward(cursor.start().deleted);
                    let text = deleted_cursor.slice(cursor.end().deleted);
                    rope.append(text);
                }
            } else if fragment.visible {
                visible_cursor.seek_forward(cursor.end().visible);
            }

            cursor.next();
        }

        if cursor.start().visible > visible_cursor.offset() {
            let text = visible_cursor.slice(cursor.start().visible);
            rope.append(text);
        }

        rope
    }

    pub fn remote_id(&self) -> BufferId {
        self.remote_id
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars_at(0)
    }

    pub fn chars_for_range<T: ToOffset>(&self, range: Range<T>) -> impl Iterator<Item = char> + '_ {
        self.text_for_range(range).flat_map(str::chars)
    }

    pub fn reversed_chars_for_range<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = char> + '_ {
        self.reversed_chunks_in_range(range)
            .flat_map(|chunk| chunk.chars().rev())
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

    pub fn common_prefix_at<T>(&self, position: T, needle: &str) -> Range<T>
    where
        T: ToOffset + TextDimension,
    {
        let offset = position.to_offset(self);
        let common_prefix_len = needle
            .char_indices()
            .map(|(index, _)| index)
            .chain([needle.len()])
            .take_while(|&len| len <= offset)
            .filter(|&len| {
                let left = self
                    .chars_for_range(offset - len..offset)
                    .flat_map(char::to_lowercase);
                let right = needle[..len].chars().flat_map(char::to_lowercase);
                left.eq(right)
            })
            .last()
            .unwrap_or(0);
        let start_offset = offset - common_prefix_len;
        let start = self.text_summary_for_range(0..start_offset);
        start..position
    }

    pub fn text(&self) -> String {
        self.visible_text.to_string()
    }

    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
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

    pub fn max_point_utf16(&self) -> PointUtf16 {
        self.visible_text.max_point_utf16()
    }

    pub fn point_to_offset(&self, point: Point) -> usize {
        self.visible_text.point_to_offset(point)
    }

    pub fn point_to_offset_utf16(&self, point: Point) -> OffsetUtf16 {
        self.visible_text.point_to_offset_utf16(point)
    }

    pub fn point_utf16_to_offset_utf16(&self, point: PointUtf16) -> OffsetUtf16 {
        self.visible_text.point_utf16_to_offset_utf16(point)
    }

    pub fn point_utf16_to_offset(&self, point: PointUtf16) -> usize {
        self.visible_text.point_utf16_to_offset(point)
    }

    pub fn unclipped_point_utf16_to_offset(&self, point: Unclipped<PointUtf16>) -> usize {
        self.visible_text.unclipped_point_utf16_to_offset(point)
    }

    pub fn unclipped_point_utf16_to_point(&self, point: Unclipped<PointUtf16>) -> Point {
        self.visible_text.unclipped_point_utf16_to_point(point)
    }

    pub fn offset_utf16_to_offset(&self, offset: OffsetUtf16) -> usize {
        self.visible_text.offset_utf16_to_offset(offset)
    }

    pub fn offset_to_offset_utf16(&self, offset: usize) -> OffsetUtf16 {
        self.visible_text.offset_to_offset_utf16(offset)
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

    pub fn point_utf16_to_point(&self, point: PointUtf16) -> Point {
        self.visible_text.point_utf16_to_point(point)
    }

    pub fn version(&self) -> &clock::Global {
        &self.version
    }

    pub fn chars_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = char> + '_ {
        let offset = position.to_offset(self);
        self.visible_text.chars_at(offset)
    }

    pub fn reversed_chars_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = char> + '_ {
        let offset = position.to_offset(self);
        self.visible_text.reversed_chars_at(offset)
    }

    pub fn reversed_chunks_in_range<T: ToOffset>(&self, range: Range<T>) -> rope::Chunks<'_> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.visible_text.reversed_chunks_in_range(range)
    }

    pub fn bytes_in_range<T: ToOffset>(&self, range: Range<T>) -> rope::Bytes<'_> {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);
        self.visible_text.bytes_in_range(start..end)
    }

    pub fn reversed_bytes_in_range<T: ToOffset>(&self, range: Range<T>) -> rope::Bytes<'_> {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);
        self.visible_text.reversed_bytes_in_range(start..end)
    }

    pub fn text_for_range<T: ToOffset>(&self, range: Range<T>) -> Chunks<'_> {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);
        self.visible_text.chunks_in_range(start..end)
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let row_start_offset = Point::new(row, 0).to_offset(self);
        let row_end_offset = if row >= self.max_point().row {
            self.len()
        } else {
            Point::new(row + 1, 0).to_previous_offset(self)
        };
        (row_end_offset - row_start_offset) as u32
    }

    pub fn line_indents_in_row_range(
        &self,
        row_range: Range<u32>,
    ) -> impl Iterator<Item = (u32, LineIndent)> + '_ {
        let start = Point::new(row_range.start, 0).to_offset(self);
        let end = Point::new(row_range.end, self.line_len(row_range.end)).to_offset(self);

        let mut chunks = self.as_rope().chunks_in_range(start..end);
        let mut row = row_range.start;
        let mut done = false;
        std::iter::from_fn(move || {
            if done {
                None
            } else {
                let indent = (row, LineIndent::from_chunks(&mut chunks));
                done = !chunks.next_line();
                row += 1;
                Some(indent)
            }
        })
    }

    /// Returns the line indents in the given row range, exclusive of end row, in reversed order.
    pub fn reversed_line_indents_in_row_range(
        &self,
        row_range: Range<u32>,
    ) -> impl Iterator<Item = (u32, LineIndent)> + '_ {
        let start = Point::new(row_range.start, 0).to_offset(self);

        let end_point;
        let end;
        if row_range.end > row_range.start {
            end_point = Point::new(row_range.end - 1, self.line_len(row_range.end - 1));
            end = end_point.to_offset(self);
        } else {
            end_point = Point::new(row_range.start, 0);
            end = start;
        };

        let mut chunks = self.as_rope().chunks_in_range(start..end);
        // Move the cursor to the start of the last line if it's not empty.
        chunks.seek(end);
        if end_point.column > 0 {
            chunks.prev_line();
        }

        let mut row = end_point.row;
        let mut done = false;
        std::iter::from_fn(move || {
            if done {
                None
            } else {
                let initial_offset = chunks.offset();
                let indent = (row, LineIndent::from_chunks(&mut chunks));
                if chunks.offset() > initial_offset {
                    chunks.prev_line();
                }
                done = !chunks.prev_line();
                if !done {
                    row -= 1;
                }

                Some(indent)
            }
        })
    }

    pub fn line_indent_for_row(&self, row: u32) -> LineIndent {
        LineIndent::from_iter(self.chars_at(Point::new(row, 0)))
    }

    pub fn is_line_blank(&self, row: u32) -> bool {
        self.text_for_range(Point::new(row, 0)..Point::new(row, self.line_len(row)))
            .all(|chunk| chunk.matches(|c: char| !c.is_whitespace()).next().is_none())
    }

    pub fn text_summary_for_range<D, O: ToOffset>(&self, range: Range<O>) -> D
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
        self.summaries_for_anchors_with_payload::<D, _, ()>(anchors.map(|a| (a, ())))
            .map(|d| d.0)
    }

    pub fn summaries_for_anchors_with_payload<'a, D, A, T>(
        &'a self,
        anchors: A,
    ) -> impl 'a + Iterator<Item = (D, T)>
    where
        D: 'a + TextDimension,
        A: 'a + IntoIterator<Item = (&'a Anchor, T)>,
    {
        let anchors = anchors.into_iter();
        let mut fragment_cursor = self
            .fragments
            .cursor::<Dimensions<Option<&Locator>, usize>>(&None);
        let mut text_cursor = self.visible_text.cursor(0);
        let mut position = D::zero(());

        anchors.map(move |(anchor, payload)| {
            if anchor.is_min() {
                return (D::zero(()), payload);
            } else if anchor.is_max() {
                return (D::from_text_summary(&self.visible_text.summary()), payload);
            }

            let Some(insertion) = self.try_find_fragment(anchor) else {
                panic!(
                    "invalid insertion for buffer {}@{:?} with anchor {:?}",
                    self.remote_id(),
                    self.version,
                    anchor
                );
            };
            assert_eq!(
                insertion.timestamp,
                anchor.timestamp,
                "invalid insertion for buffer {}@{:?} and anchor {:?}",
                self.remote_id(),
                self.version,
                anchor
            );

            fragment_cursor.seek_forward(&Some(&insertion.fragment_id), Bias::Left);
            let fragment = fragment_cursor.item().unwrap();
            let mut fragment_offset = fragment_cursor.start().1;
            if fragment.visible {
                fragment_offset += anchor.offset - insertion.split_offset;
            }

            position.add_assign(&text_cursor.summary(fragment_offset));
            (position, payload)
        })
    }

    pub fn summary_for_anchor<D>(&self, anchor: &Anchor) -> D
    where
        D: TextDimension,
    {
        self.text_summary_for_range(0..self.offset_for_anchor(anchor))
    }

    pub fn offset_for_anchor(&self, anchor: &Anchor) -> usize {
        if anchor.is_min() {
            0
        } else if anchor.is_max() {
            self.visible_text.len()
        } else {
            debug_assert_eq!(anchor.buffer_id, Some(self.remote_id));
            debug_assert!(
                self.version.observed(anchor.timestamp),
                "Anchor timestamp {:?} not observed by buffer {:?}",
                anchor.timestamp,
                self.version
            );
            let item = self.try_find_fragment(anchor);
            let Some(insertion) = item.filter(|insertion| insertion.timestamp == anchor.timestamp)
            else {
                self.panic_bad_anchor(anchor);
            };

            let (start, _, item) = self
                .fragments
                .find::<Dimensions<Option<&Locator>, usize>, _>(
                    &None,
                    &Some(&insertion.fragment_id),
                    Bias::Left,
                );
            let fragment = item.unwrap();
            let mut fragment_offset = start.1;
            if fragment.visible {
                fragment_offset += anchor.offset - insertion.split_offset;
            }
            fragment_offset
        }
    }

    #[cold]
    fn panic_bad_anchor(&self, anchor: &Anchor) -> ! {
        if anchor.buffer_id.is_some_and(|id| id != self.remote_id) {
            panic!(
                "invalid anchor - buffer id does not match: anchor {anchor:?}; buffer id: {}, version: {:?}",
                self.remote_id, self.version
            );
        } else if !self.version.observed(anchor.timestamp) {
            panic!(
                "invalid anchor - snapshot has not observed lamport: {:?}; version: {:?}",
                anchor, self.version
            );
        } else {
            panic!(
                "invalid anchor {:?}. buffer id: {}, version: {:?}",
                anchor, self.remote_id, self.version
            );
        }
    }

    fn fragment_id_for_anchor(&self, anchor: &Anchor) -> &Locator {
        self.try_fragment_id_for_anchor(anchor)
            .unwrap_or_else(|| self.panic_bad_anchor(anchor))
    }

    fn try_fragment_id_for_anchor(&self, anchor: &Anchor) -> Option<&Locator> {
        if anchor.is_min() {
            Some(Locator::min_ref())
        } else if anchor.is_max() {
            Some(Locator::max_ref())
        } else {
            let item = self.try_find_fragment(anchor);
            item.filter(|insertion| {
                !cfg!(debug_assertions) || insertion.timestamp == anchor.timestamp
            })
            .map(|insertion| &insertion.fragment_id)
        }
    }

    fn try_find_fragment(&self, anchor: &Anchor) -> Option<&InsertionFragment> {
        let anchor_key = InsertionFragmentKey {
            timestamp: anchor.timestamp,
            split_offset: anchor.offset,
        };
        match self.insertions.find_with_prev::<InsertionFragmentKey, _>(
            (),
            &anchor_key,
            anchor.bias,
        ) {
            (_, _, Some((prev, insertion))) => {
                let comparison = sum_tree::KeyedItem::key(insertion).cmp(&anchor_key);
                if comparison == Ordering::Greater
                    || (anchor.bias == Bias::Left
                        && comparison == Ordering::Equal
                        && anchor.offset > 0)
                {
                    prev
                } else {
                    Some(insertion)
                }
            }
            _ => self.insertions.last(),
        }
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        self.anchor_at_offset(position.to_offset(self), bias)
    }

    fn anchor_at_offset(&self, mut offset: usize, bias: Bias) -> Anchor {
        if bias == Bias::Left && offset == 0 {
            Anchor::min_for_buffer(self.remote_id)
        } else if bias == Bias::Right
            && ((!cfg!(debug_assertions) && offset >= self.len()) || offset == self.len())
        {
            Anchor::max_for_buffer(self.remote_id)
        } else {
            if self
                .visible_text
                .assert_char_boundary::<{ cfg!(debug_assertions) }>(offset)
            {
                offset = match bias {
                    Bias::Left => self.visible_text.floor_char_boundary(offset),
                    Bias::Right => self.visible_text.ceil_char_boundary(offset),
                };
            }
            let (start, _, item) = self.fragments.find::<usize, _>(&None, &offset, bias);
            let Some(fragment) = item else {
                // We got a bad offset, likely out of bounds
                debug_panic!(
                    "Failed to find fragment at offset {} (len: {})",
                    offset,
                    self.len()
                );
                return Anchor::max_for_buffer(self.remote_id);
            };
            let overshoot = offset - start;
            Anchor {
                timestamp: fragment.timestamp,
                offset: fragment.insertion_offset + overshoot,
                bias,
                buffer_id: Some(self.remote_id),
            }
        }
    }

    pub fn can_resolve(&self, anchor: &Anchor) -> bool {
        anchor.is_min()
            || anchor.is_max()
            || (Some(self.remote_id) == anchor.buffer_id && self.version.observed(anchor.timestamp))
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        self.visible_text.clip_offset(offset, bias)
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.visible_text.clip_point(point, bias)
    }

    pub fn clip_offset_utf16(&self, offset: OffsetUtf16, bias: Bias) -> OffsetUtf16 {
        self.visible_text.clip_offset_utf16(offset, bias)
    }

    pub fn clip_point_utf16(&self, point: Unclipped<PointUtf16>, bias: Bias) -> PointUtf16 {
        self.visible_text.clip_point_utf16(point, bias)
    }

    pub fn edits_since<'a, D>(
        &'a self,
        since: &'a clock::Global,
    ) -> impl 'a + Iterator<Item = Edit<D>>
    where
        D: TextDimension + Ord,
    {
        self.edits_since_in_range(since, Anchor::MIN..Anchor::MAX)
    }

    pub fn anchored_edits_since<'a, D>(
        &'a self,
        since: &'a clock::Global,
    ) -> impl 'a + Iterator<Item = (Edit<D>, Range<Anchor>)>
    where
        D: TextDimension + Ord,
    {
        self.anchored_edits_since_in_range(since, Anchor::MIN..Anchor::MAX)
    }

    pub fn edits_since_in_range<'a, D>(
        &'a self,
        since: &'a clock::Global,
        range: Range<Anchor>,
    ) -> impl 'a + Iterator<Item = Edit<D>>
    where
        D: TextDimension + Ord,
    {
        self.anchored_edits_since_in_range(since, range)
            .map(|item| item.0)
    }

    pub fn anchored_edits_since_in_range<'a, D>(
        &'a self,
        since: &'a clock::Global,
        range: Range<Anchor>,
    ) -> impl 'a + Iterator<Item = (Edit<D>, Range<Anchor>)>
    where
        D: TextDimension + Ord,
    {
        let fragments_cursor = if *since == self.version {
            None
        } else {
            let mut cursor = self.fragments.filter(&None, move |summary| {
                !since.observed_all(&summary.max_version)
            });
            cursor.next();
            Some(cursor)
        };
        let start_fragment_id = self.fragment_id_for_anchor(&range.start);
        let (start, _, item) = self
            .fragments
            .find::<Dimensions<Option<&Locator>, FragmentTextSummary>, _>(
                &None,
                &Some(start_fragment_id),
                Bias::Left,
            );
        let mut visible_start = start.1.visible;
        let mut deleted_start = start.1.deleted;
        if let Some(fragment) = item {
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
            old_end: D::zero(()),
            new_end: D::zero(()),
            range: (start_fragment_id, range.start.offset)..(end_fragment_id, range.end.offset),
            buffer_id: self.remote_id,
        }
    }

    pub fn has_edits_since_in_range(&self, since: &clock::Global, range: Range<Anchor>) -> bool {
        if *since != self.version {
            let start_fragment_id = self.fragment_id_for_anchor(&range.start);
            let end_fragment_id = self.fragment_id_for_anchor(&range.end);
            let mut cursor = self.fragments.filter::<_, usize>(&None, move |summary| {
                !since.observed_all(&summary.max_version)
            });
            cursor.next();
            while let Some(fragment) = cursor.item() {
                if fragment.id > *end_fragment_id {
                    break;
                }
                if fragment.id > *start_fragment_id {
                    let was_visible = fragment.was_visible(since, &self.undo_map);
                    let is_visible = fragment.visible;
                    if was_visible != is_visible {
                        return true;
                    }
                }
                cursor.next();
            }
        }
        false
    }

    pub fn has_edits_since(&self, since: &clock::Global) -> bool {
        if *since != self.version {
            let mut cursor = self.fragments.filter::<_, usize>(&None, move |summary| {
                !since.observed_all(&summary.max_version)
            });
            cursor.next();
            while let Some(fragment) = cursor.item() {
                let was_visible = fragment.was_visible(since, &self.undo_map);
                let is_visible = fragment.visible;
                if was_visible != is_visible {
                    return true;
                }
                cursor.next();
            }
        }
        false
    }

    pub fn range_to_version(&self, range: Range<usize>, version: &clock::Global) -> Range<usize> {
        let mut offsets = self.offsets_to_version([range.start, range.end], version);
        offsets.next().unwrap()..offsets.next().unwrap()
    }

    /// Converts the given sequence of offsets into their corresponding offsets
    /// at a prior version of this buffer.
    pub fn offsets_to_version<'a>(
        &'a self,
        offsets: impl 'a + IntoIterator<Item = usize>,
        version: &'a clock::Global,
    ) -> impl 'a + Iterator<Item = usize> {
        let mut edits = self.edits_since(version).peekable();
        let mut last_old_end = 0;
        let mut last_new_end = 0;
        offsets.into_iter().map(move |new_offset| {
            while let Some(edit) = edits.peek() {
                if edit.new.start > new_offset {
                    break;
                }

                if edit.new.end <= new_offset {
                    last_new_end = edit.new.end;
                    last_old_end = edit.old.end;
                    edits.next();
                    continue;
                }

                let overshoot = new_offset - edit.new.start;
                return (edit.old.start + overshoot).min(edit.old.end);
            }

            last_old_end + new_offset.saturating_sub(last_new_end)
        })
    }

    /// Visually annotates a position or range with the `Debug` representation of a value. The
    /// callsite of this function is used as a key - previous annotations will be removed.
    #[cfg(debug_assertions)]
    #[track_caller]
    pub fn debug<R, V>(&self, ranges: &R, value: V)
    where
        R: debug::ToDebugRanges,
        V: std::fmt::Debug,
    {
        self.debug_with_key(std::panic::Location::caller(), ranges, value);
    }

    /// Visually annotates a position or range with the `Debug` representation of a value. Previous
    /// debug annotations with the same key will be removed. The key is also used to determine the
    /// annotation's color.
    #[cfg(debug_assertions)]
    pub fn debug_with_key<K, R, V>(&self, key: &K, ranges: &R, value: V)
    where
        K: std::hash::Hash + 'static,
        R: debug::ToDebugRanges,
        V: std::fmt::Debug,
    {
        let ranges = ranges
            .to_debug_ranges(self)
            .into_iter()
            .map(|range| self.anchor_after(range.start)..self.anchor_before(range.end))
            .collect();
        debug::GlobalDebugRanges::with_locked(|debug_ranges| {
            debug_ranges.insert(key, ranges, format!("{value:?}").into());
        });
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

    fn append(&mut self, len: FragmentTextSummary) {
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

impl<D: TextDimension + Ord, F: FnMut(&FragmentSummary) -> bool> Iterator for Edits<'_, D, F> {
    type Item = (Edit<D>, Range<Anchor>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut pending_edit: Option<Self::Item> = None;
        let cursor = self.fragments_cursor.as_mut()?;

        while let Some(fragment) = cursor.item() {
            if fragment.id < *self.range.start.0 {
                cursor.next();
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
                .is_some_and(|(change, _)| change.new.end < self.new_end)
            {
                break;
            }

            let start_anchor = Anchor {
                timestamp: fragment.timestamp,
                offset: fragment.insertion_offset,
                bias: Bias::Right,
                buffer_id: Some(self.buffer_id),
            };
            let end_anchor = Anchor {
                timestamp: fragment.timestamp,
                offset: fragment.insertion_offset + fragment.len,
                bias: Bias::Left,
                buffer_id: Some(self.buffer_id),
            };

            if !fragment.was_visible(self.since, self.undos) && fragment.visible {
                let mut visible_end = cursor.end().visible;
                if fragment.id == *self.range.end.0 {
                    visible_end = cmp::min(
                        visible_end,
                        cursor.start().visible + (self.range.end.1 - fragment.insertion_offset),
                    );
                }

                let fragment_summary = self.visible_cursor.summary(visible_end);
                let mut new_end = self.new_end;
                new_end.add_assign(&fragment_summary);
                if let Some((edit, range)) = pending_edit.as_mut() {
                    edit.new.end = new_end;
                    range.end = end_anchor;
                } else {
                    pending_edit = Some((
                        Edit {
                            old: self.old_end..self.old_end,
                            new: self.new_end..new_end,
                        },
                        start_anchor..end_anchor,
                    ));
                }

                self.new_end = new_end;
            } else if fragment.was_visible(self.since, self.undos) && !fragment.visible {
                let mut deleted_end = cursor.end().deleted;
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
                let mut old_end = self.old_end;
                old_end.add_assign(&fragment_summary);
                if let Some((edit, range)) = pending_edit.as_mut() {
                    edit.old.end = old_end;
                    range.end = end_anchor;
                } else {
                    pending_edit = Some((
                        Edit {
                            old: self.old_end..old_end,
                            new: self.new_end..self.new_end,
                        },
                        start_anchor..end_anchor,
                    ));
                }

                self.old_end = old_end;
            }

            cursor.next();
        }

        pending_edit
    }
}

impl Fragment {
    fn is_visible(&self, undos: &UndoMap) -> bool {
        !undos.is_undone(self.timestamp) && self.deletions.iter().all(|d| undos.is_undone(*d))
    }

    fn was_visible(&self, version: &clock::Global, undos: &UndoMap) -> bool {
        (version.observed(self.timestamp) && !undos.was_undone(self.timestamp, version))
            && self
                .deletions
                .iter()
                .all(|d| !version.observed(*d) || undos.was_undone(*d, version))
    }
}

impl sum_tree::Item for Fragment {
    type Summary = FragmentSummary;

    fn summary(&self, _cx: &Option<clock::Global>) -> Self::Summary {
        let mut max_version = clock::Global::new();
        max_version.observe(self.timestamp);
        for deletion in &self.deletions {
            max_version.observe(*deletion);
        }
        max_version.join(&self.max_undos);

        let mut min_insertion_version = clock::Global::new();
        min_insertion_version.observe(self.timestamp);
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
    type Context<'a> = &'a Option<clock::Global>;

    fn zero(_cx: Self::Context<'_>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self, _: Self::Context<'_>) {
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

    fn summary(&self, _cx: ()) -> Self::Summary {
        InsertionFragmentKey {
            timestamp: self.timestamp,
            split_offset: self.split_offset,
        }
    }
}

impl sum_tree::KeyedItem for InsertionFragment {
    type Key = InsertionFragmentKey;

    fn key(&self) -> Self::Key {
        sum_tree::Item::summary(self, ())
    }
}

impl InsertionFragment {
    fn new(fragment: &Fragment) -> Self {
        Self {
            timestamp: fragment.timestamp,
            split_offset: fragment.insertion_offset,
            fragment_id: fragment.id.clone(),
        }
    }

    fn insert_new(fragment: &Fragment) -> sum_tree::Edit<Self> {
        sum_tree::Edit::Insert(Self::new(fragment))
    }
}

impl sum_tree::ContextLessSummary for InsertionFragmentKey {
    fn zero() -> Self {
        InsertionFragmentKey {
            timestamp: Lamport::MIN,
            split_offset: 0,
        }
    }

    fn add_summary(&mut self, summary: &Self) {
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

impl sum_tree::Dimension<'_, FragmentSummary> for usize {
    fn zero(_: &Option<clock::Global>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &FragmentSummary, _: &Option<clock::Global>) {
        *self += summary.text.visible;
    }
}

impl sum_tree::Dimension<'_, FragmentSummary> for FullOffset {
    fn zero(_: &Option<clock::Global>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &FragmentSummary, _: &Option<clock::Global>) {
        self.0 += summary.text.visible + summary.text.deleted;
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for Option<&'a Locator> {
    fn zero(_: &Option<clock::Global>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<clock::Global>) {
        *self = Some(&summary.max_id);
    }
}

impl sum_tree::SeekTarget<'_, FragmentSummary, FragmentTextSummary> for usize {
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
    fn zero(_cx: &Option<clock::Global>) -> Self {
        Default::default()
    }

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

impl sum_tree::SeekTarget<'_, FragmentSummary, Self> for VersionedFullOffset {
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

    pub fn timestamp(&self) -> clock::Lamport {
        match self {
            Operation::Edit(edit) => edit.timestamp,
            Operation::Undo(undo) => undo.timestamp,
        }
    }

    pub fn as_edit(&self) -> Option<&EditOperation> {
        match self {
            Operation::Edit(edit) => Some(edit),
            _ => None,
        }
    }

    pub fn is_edit(&self) -> bool {
        matches!(self, Operation::Edit { .. })
    }
}

impl operation_queue::Operation for Operation {
    fn lamport_timestamp(&self) -> clock::Lamport {
        match self {
            Operation::Edit(edit) => edit.timestamp,
            Operation::Undo(undo) => undo.timestamp,
        }
    }
}

pub trait ToOffset {
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize;
    /// Turns this point into the next offset in the buffer that comes after this, respecting utf8 boundaries.
    fn to_next_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot
            .visible_text
            .ceil_char_boundary(self.to_offset(snapshot) + 1)
    }
    /// Turns this point into the previous offset in the buffer that comes before this, respecting utf8 boundaries.
    fn to_previous_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot
            .visible_text
            .floor_char_boundary(self.to_offset(snapshot).saturating_sub(1))
    }
}

impl ToOffset for Point {
    #[inline]
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot.point_to_offset(*self)
    }
}

impl ToOffset for usize {
    #[track_caller]
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize {
        if snapshot
            .as_rope()
            .assert_char_boundary::<{ cfg!(debug_assertions) }>(*self)
        {
            snapshot.as_rope().floor_char_boundary(*self)
        } else {
            *self
        }
    }
}

impl ToOffset for Anchor {
    #[inline]
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot.summary_for_anchor(self)
    }
}

impl<T: ToOffset> ToOffset for &T {
    #[inline]
    fn to_offset(&self, content: &BufferSnapshot) -> usize {
        (*self).to_offset(content)
    }
}

impl ToOffset for PointUtf16 {
    #[inline]
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot.point_utf16_to_offset(*self)
    }
}

impl ToOffset for Unclipped<PointUtf16> {
    #[inline]
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot.unclipped_point_utf16_to_offset(*self)
    }
}

pub trait ToPoint {
    fn to_point(&self, snapshot: &BufferSnapshot) -> Point;
}

impl ToPoint for Anchor {
    #[inline]
    fn to_point(&self, snapshot: &BufferSnapshot) -> Point {
        snapshot.summary_for_anchor(self)
    }
}

impl ToPoint for usize {
    #[inline]
    fn to_point(&self, snapshot: &BufferSnapshot) -> Point {
        snapshot.offset_to_point(*self)
    }
}

impl ToPoint for Point {
    #[inline]
    fn to_point(&self, _: &BufferSnapshot) -> Point {
        *self
    }
}

impl ToPoint for Unclipped<PointUtf16> {
    #[inline]
    fn to_point(&self, snapshot: &BufferSnapshot) -> Point {
        snapshot.unclipped_point_utf16_to_point(*self)
    }
}

pub trait ToPointUtf16 {
    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> PointUtf16;
}

impl ToPointUtf16 for Anchor {
    #[inline]
    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> PointUtf16 {
        snapshot.summary_for_anchor(self)
    }
}

impl ToPointUtf16 for usize {
    #[inline]
    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> PointUtf16 {
        snapshot.offset_to_point_utf16(*self)
    }
}

impl ToPointUtf16 for PointUtf16 {
    #[inline]
    fn to_point_utf16(&self, _: &BufferSnapshot) -> PointUtf16 {
        *self
    }
}

impl ToPointUtf16 for Point {
    #[inline]
    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> PointUtf16 {
        snapshot.point_to_point_utf16(*self)
    }
}

pub trait ToOffsetUtf16 {
    fn to_offset_utf16(&self, snapshot: &BufferSnapshot) -> OffsetUtf16;
}

impl ToOffsetUtf16 for Anchor {
    #[inline]
    fn to_offset_utf16(&self, snapshot: &BufferSnapshot) -> OffsetUtf16 {
        snapshot.summary_for_anchor(self)
    }
}

impl ToOffsetUtf16 for usize {
    #[inline]
    fn to_offset_utf16(&self, snapshot: &BufferSnapshot) -> OffsetUtf16 {
        snapshot.offset_to_offset_utf16(*self)
    }
}

impl ToOffsetUtf16 for OffsetUtf16 {
    #[inline]
    fn to_offset_utf16(&self, _snapshot: &BufferSnapshot) -> OffsetUtf16 {
        *self
    }
}

pub trait FromAnchor {
    fn from_anchor(anchor: &Anchor, snapshot: &BufferSnapshot) -> Self;
}

impl FromAnchor for Anchor {
    #[inline]
    fn from_anchor(anchor: &Anchor, _snapshot: &BufferSnapshot) -> Self {
        *anchor
    }
}

impl FromAnchor for Point {
    #[inline]
    fn from_anchor(anchor: &Anchor, snapshot: &BufferSnapshot) -> Self {
        snapshot.summary_for_anchor(anchor)
    }
}

impl FromAnchor for PointUtf16 {
    #[inline]
    fn from_anchor(anchor: &Anchor, snapshot: &BufferSnapshot) -> Self {
        snapshot.summary_for_anchor(anchor)
    }
}

impl FromAnchor for usize {
    #[inline]
    fn from_anchor(anchor: &Anchor, snapshot: &BufferSnapshot) -> Self {
        snapshot.summary_for_anchor(anchor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineEnding {
    Unix,
    Windows,
}

impl Default for LineEnding {
    fn default() -> Self {
        #[cfg(unix)]
        return Self::Unix;

        #[cfg(not(unix))]
        return Self::Windows;
    }
}

impl LineEnding {
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Unix => "\n",
            LineEnding::Windows => "\r\n",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            LineEnding::Unix => "LF",
            LineEnding::Windows => "CRLF",
        }
    }

    pub fn detect(text: &str) -> Self {
        let mut max_ix = cmp::min(text.len(), 1000);
        while !text.is_char_boundary(max_ix) {
            max_ix -= 1;
        }

        if let Some(ix) = text[..max_ix].find(['\n']) {
            if ix > 0 && text.as_bytes()[ix - 1] == b'\r' {
                Self::Windows
            } else {
                Self::Unix
            }
        } else {
            Self::default()
        }
    }

    pub fn normalize(text: &mut String) {
        if let Cow::Owned(replaced) = LINE_SEPARATORS_REGEX.replace_all(text, "\n") {
            *text = replaced;
        }
    }

    pub fn normalize_arc(text: Arc<str>) -> Arc<str> {
        if let Cow::Owned(replaced) = LINE_SEPARATORS_REGEX.replace_all(&text, "\n") {
            replaced.into()
        } else {
            text
        }
    }

    pub fn normalize_cow(text: Cow<str>) -> Cow<str> {
        if let Cow::Owned(replaced) = LINE_SEPARATORS_REGEX.replace_all(&text, "\n") {
            replaced.into()
        } else {
            text
        }
    }
}

pub fn chunks_with_line_ending(rope: &Rope, line_ending: LineEnding) -> impl Iterator<Item = &str> {
    rope.chunks().flat_map(move |chunk| {
        let mut newline = false;
        let end_with_newline = chunk.ends_with('\n').then_some(line_ending.as_str());
        chunk
            .lines()
            .flat_map(move |line| {
                let ending = if newline {
                    Some(line_ending.as_str())
                } else {
                    None
                };
                newline = true;
                ending.into_iter().chain([line])
            })
            .chain(end_with_newline)
    })
}

#[cfg(debug_assertions)]
pub mod debug {
    use super::*;
    use parking_lot::Mutex;
    use std::any::TypeId;
    use std::hash::{Hash, Hasher};

    static GLOBAL_DEBUG_RANGES: Mutex<Option<GlobalDebugRanges>> = Mutex::new(None);

    pub struct GlobalDebugRanges {
        pub ranges: Vec<DebugRange>,
        key_to_occurrence_index: HashMap<Key, usize>,
        next_occurrence_index: usize,
    }

    pub struct DebugRange {
        key: Key,
        pub ranges: Vec<Range<Anchor>>,
        pub value: Arc<str>,
        pub occurrence_index: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Key {
        type_id: TypeId,
        hash: u64,
    }

    impl GlobalDebugRanges {
        pub fn with_locked<R>(f: impl FnOnce(&mut Self) -> R) -> R {
            let mut state = GLOBAL_DEBUG_RANGES.lock();
            if state.is_none() {
                *state = Some(GlobalDebugRanges {
                    ranges: Vec::new(),
                    key_to_occurrence_index: HashMap::default(),
                    next_occurrence_index: 0,
                });
            }
            if let Some(global_debug_ranges) = state.as_mut() {
                f(global_debug_ranges)
            } else {
                unreachable!()
            }
        }

        pub fn insert<K: Hash + 'static>(
            &mut self,
            key: &K,
            ranges: Vec<Range<Anchor>>,
            value: Arc<str>,
        ) {
            let occurrence_index = *self
                .key_to_occurrence_index
                .entry(Key::new(key))
                .or_insert_with(|| {
                    let occurrence_index = self.next_occurrence_index;
                    self.next_occurrence_index += 1;
                    occurrence_index
                });
            let key = Key::new(key);
            let existing = self
                .ranges
                .iter()
                .enumerate()
                .rfind(|(_, existing)| existing.key == key);
            if let Some((existing_ix, _)) = existing {
                self.ranges.remove(existing_ix);
            }
            self.ranges.push(DebugRange {
                ranges,
                key,
                value,
                occurrence_index,
            });
        }

        pub fn remove<K: Hash + 'static>(&mut self, key: &K) {
            self.remove_impl(&Key::new(key));
        }

        fn remove_impl(&mut self, key: &Key) {
            let existing = self
                .ranges
                .iter()
                .enumerate()
                .rfind(|(_, existing)| &existing.key == key);
            if let Some((existing_ix, _)) = existing {
                self.ranges.remove(existing_ix);
            }
        }

        pub fn remove_all_with_key_type<K: 'static>(&mut self) {
            self.ranges
                .retain(|item| item.key.type_id != TypeId::of::<K>());
        }
    }

    impl Key {
        fn new<K: Hash + 'static>(key: &K) -> Self {
            let type_id = TypeId::of::<K>();
            let mut hasher = collections::FxHasher::default();
            key.hash(&mut hasher);
            Key {
                type_id,
                hash: hasher.finish(),
            }
        }
    }

    pub trait ToDebugRanges {
        fn to_debug_ranges(&self, snapshot: &BufferSnapshot) -> Vec<Range<usize>>;
    }

    impl<T: ToOffset> ToDebugRanges for T {
        fn to_debug_ranges(&self, snapshot: &BufferSnapshot) -> Vec<Range<usize>> {
            [self.to_offset(snapshot)].to_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset + Clone> ToDebugRanges for Range<T> {
        fn to_debug_ranges(&self, snapshot: &BufferSnapshot) -> Vec<Range<usize>> {
            [self.clone()].to_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToDebugRanges for Vec<T> {
        fn to_debug_ranges(&self, snapshot: &BufferSnapshot) -> Vec<Range<usize>> {
            self.as_slice().to_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToDebugRanges for Vec<Range<T>> {
        fn to_debug_ranges(&self, snapshot: &BufferSnapshot) -> Vec<Range<usize>> {
            self.as_slice().to_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToDebugRanges for [T] {
        fn to_debug_ranges(&self, snapshot: &BufferSnapshot) -> Vec<Range<usize>> {
            self.iter()
                .map(|item| {
                    let offset = item.to_offset(snapshot);
                    offset..offset
                })
                .collect()
        }
    }

    impl<T: ToOffset> ToDebugRanges for [Range<T>] {
        fn to_debug_ranges(&self, snapshot: &BufferSnapshot) -> Vec<Range<usize>> {
            self.iter()
                .map(|range| range.start.to_offset(snapshot)..range.end.to_offset(snapshot))
                .collect()
        }
    }
}
