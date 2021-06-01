mod anchor;
mod point;
pub mod rope;
mod selection;

pub use anchor::*;
use parking_lot::Mutex;
pub use point::*;
pub use rope::{Chunks, Rope, TextSummary};
use seahash::SeaHasher;
pub use selection::*;
use similar::{ChangeTag, TextDiff};
use tree_sitter::{InputEdit, Parser, QueryCursor};

use crate::{
    editor::Bias,
    language::{Language, Tree},
    operation_queue::{self, OperationQueue},
    settings::{StyleId, ThemeMap},
    sum_tree::{self, FilterCursor, SeekBias, SumTree},
    time::{self, ReplicaId},
    worktree::FileHandle,
};
use anyhow::{anyhow, Result};
use gpui::{AppContext, Entity, ModelContext, Task};
use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    cmp,
    hash::BuildHasher,
    iter::{self, Iterator},
    mem,
    ops::{Deref, DerefMut, Range},
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

thread_local! {
    static PARSER: RefCell<Parser> = RefCell::new(Parser::new());
}

lazy_static! {
    static ref QUERY_CURSORS: Mutex<Vec<QueryCursor>> = Default::default();
}

struct QueryCursorHandle(Option<QueryCursor>);

impl QueryCursorHandle {
    fn new() -> Self {
        QueryCursorHandle(Some(
            QUERY_CURSORS
                .lock()
                .pop()
                .unwrap_or_else(|| QueryCursor::new()),
        ))
    }
}

impl Deref for QueryCursorHandle {
    type Target = QueryCursor;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for QueryCursorHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

impl Drop for QueryCursorHandle {
    fn drop(&mut self) {
        let mut cursor = self.0.take().unwrap();
        cursor.set_byte_range(0..usize::MAX);
        cursor.set_point_range(Point::zero().into()..Point::MAX.into());
        QUERY_CURSORS.lock().push(cursor)
    }
}

pub struct Buffer {
    fragments: SumTree<Fragment>,
    visible_text: Rope,
    deleted_text: Rope,
    insertion_splits: HashMap<time::Local, SumTree<InsertionSplit>>,
    pub version: time::Global,
    saved_version: time::Global,
    saved_mtime: SystemTime,
    last_edit: time::Local,
    undo_map: UndoMap,
    history: History,
    file: Option<FileHandle>,
    language: Option<Arc<Language>>,
    syntax_tree: Mutex<Option<SyntaxTree>>,
    is_parsing: bool,
    selections: HashMap<SelectionSetId, Arc<[Selection]>>,
    pub selections_last_update: SelectionsVersion,
    deferred_ops: OperationQueue<Operation>,
    deferred_replicas: HashSet<ReplicaId>,
    replica_id: ReplicaId,
    local_clock: time::Local,
    lamport_clock: time::Lamport,
}

#[derive(Clone)]
struct SyntaxTree {
    tree: Tree,
    parsed: bool,
    version: time::Global,
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
    deleted_text: &'a Rope,
    cursor: FilterCursor<'a, F, Fragment, FragmentTextSummary>,
    undos: &'a UndoMap,
    since: time::Global,
    delta: isize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Edit {
    pub old_range: Range<usize>,
    pub new_range: Range<usize>,
    pub old_lines: Point,
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
    min_insertion_version: time::Global,
    max_insertion_version: time::Global,
    count: usize,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct FragmentTextSummary {
    visible: usize,
    deleted: usize,
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FragmentTextSummary {
    fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<time::Global>) {
        self.visible += summary.text.visible;
        self.deleted += summary.text.deleted;
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
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(replica_id, History::new(base_text.into()), None, None, cx)
    }

    pub fn from_history(
        replica_id: ReplicaId,
        history: History,
        file: Option<FileHandle>,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(replica_id, history, file, language, cx)
    }

    fn build(
        replica_id: ReplicaId,
        history: History,
        file: Option<FileHandle>,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let saved_mtime;
        if let Some(file) = file.as_ref() {
            saved_mtime = file.mtime();
            file.observe_from_model(cx, |this, file, cx| {
                let version = this.version.clone();
                if this.version == this.saved_version {
                    if file.is_deleted() {
                        cx.emit(Event::Dirtied);
                    } else {
                        cx.spawn(|handle, mut cx| async move {
                            let (current_version, history) = handle.read_with(&cx, |this, cx| {
                                (this.version.clone(), file.load_history(cx.as_ref()))
                            });
                            if let (Ok(history), true) = (history.await, current_version == version)
                            {
                                let diff = handle
                                    .read_with(&cx, |this, cx| this.diff(history.base_text, cx))
                                    .await;
                                handle.update(&mut cx, |this, cx| {
                                    if let Some(_ops) = this.set_text_via_diff(diff, cx) {
                                        this.saved_version = this.version.clone();
                                        this.saved_mtime = file.mtime();
                                        cx.emit(Event::Reloaded);
                                    }
                                });
                            }
                        })
                        .detach();
                    }
                }
                cx.emit(Event::FileHandleChanged);
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
            &None,
        );

        if base_text.len() > 0 {
            let base_fragment_id =
                FragmentId::between(&FragmentId::min_value(), &FragmentId::max_value());
            let range_in_insertion = 0..base_text.len();

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
                &None,
            );
        }

        let mut result = Self {
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
            syntax_tree: Mutex::new(None),
            is_parsing: false,
            language,
            saved_mtime,
            selections: HashMap::default(),
            selections_last_update: 0,
            deferred_ops: OperationQueue::new(),
            deferred_replicas: HashSet::default(),
            replica_id,
            local_clock: time::Local::new(replica_id),
            lamport_clock: time::Lamport::new(replica_id),
        };
        result.reparse(cx);
        result
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            text: self.visible_text.clone(),
            tree: self.syntax_tree(),
            language: self.language.clone(),
            query_cursor: QueryCursorHandle::new(),
        }
    }

    pub fn file(&self) -> Option<&FileHandle> {
        self.file.as_ref()
    }

    pub fn save(
        &mut self,
        new_file: Option<FileHandle>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let text = self.visible_text.clone();
        let version = self.version.clone();
        let file = self.file.clone();

        cx.spawn(|handle, mut cx| async move {
            if let Some(file) = new_file.as_ref().or(file.as_ref()) {
                let result = cx.read(|cx| file.save(text, cx.as_ref())).await;
                if result.is_ok() {
                    handle.update(&mut cx, |me, cx| me.did_save(version, new_file, cx));
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
        cx: &mut ModelContext<Buffer>,
    ) {
        if file.is_some() {
            self.file = file;
        }
        if let Some(file) = &self.file {
            self.saved_mtime = file.mtime();
        }
        self.saved_version = version;
        cx.emit(Event::Saved);
    }

    pub fn syntax_tree(&self) -> Option<Tree> {
        if let Some(syntax_tree) = self.syntax_tree.lock().as_mut() {
            let mut edited = false;
            let mut delta = 0_isize;
            for Edit {
                old_range,
                new_range,
                old_lines,
            } in self.edits_since(syntax_tree.version.clone())
            {
                let start_offset = (old_range.start as isize + delta) as usize;
                let start_point = self.visible_text.to_point(start_offset);
                let old_bytes = old_range.end - old_range.start;
                let new_bytes = new_range.end - new_range.start;
                syntax_tree.tree.edit(&InputEdit {
                    start_byte: start_offset,
                    old_end_byte: start_offset + old_bytes,
                    new_end_byte: start_offset + new_bytes,
                    start_position: start_point.into(),
                    old_end_position: (start_point + old_lines).into(),
                    new_end_position: self.visible_text.to_point(start_offset + new_bytes).into(),
                });
                delta += new_bytes as isize - old_bytes as isize;
                edited = true;
            }
            syntax_tree.parsed &= !edited;
            syntax_tree.version = self.version();
            Some(syntax_tree.tree.clone())
        } else {
            None
        }
    }

    pub fn is_parsing(&self) -> bool {
        self.is_parsing
    }

    fn should_reparse(&self) -> bool {
        if let Some(syntax_tree) = self.syntax_tree.lock().as_ref() {
            !syntax_tree.parsed || syntax_tree.version != self.version
        } else {
            self.language.is_some()
        }
    }

    fn reparse(&mut self, cx: &mut ModelContext<Self>) {
        // Avoid spawning a new parsing task if the buffer is already being reparsed
        // due to an earlier edit.
        if self.is_parsing {
            return;
        }

        if let Some(language) = self.language.clone() {
            self.is_parsing = true;
            cx.spawn(|handle, mut cx| async move {
                while handle.read_with(&cx, |this, _| this.should_reparse()) {
                    // The parse tree is out of date, so grab the syntax tree to synchronously
                    // splice all the edits that have happened since the last parse.
                    let new_tree = handle.update(&mut cx, |this, _| this.syntax_tree());
                    let (new_text, new_version) = handle
                        .read_with(&cx, |this, _| (this.visible_text.clone(), this.version()));

                    // Parse the current text in a background thread.
                    let new_tree = cx
                        .background_executor()
                        .spawn({
                            let language = language.clone();
                            async move { Self::parse_text(&new_text, new_tree, &language) }
                        })
                        .await;

                    handle.update(&mut cx, |this, cx| {
                        *this.syntax_tree.lock() = Some(SyntaxTree {
                            tree: new_tree,
                            parsed: true,
                            version: new_version,
                        });
                        cx.emit(Event::Reparsed);
                        cx.notify();
                    });
                }
                handle.update(&mut cx, |this, _| this.is_parsing = false);
            })
            .detach();
        }
    }

    fn parse_text(text: &Rope, old_tree: Option<Tree>, language: &Language) -> Tree {
        PARSER.with(|parser| {
            let mut parser = parser.borrow_mut();
            parser
                .set_language(language.grammar)
                .expect("incompatible grammar");
            let mut chunks = text.chunks_in_range(0..text.len());
            let tree = parser
                .parse_with(
                    &mut move |offset, _| {
                        chunks.seek(offset);
                        chunks.next().unwrap_or("").as_bytes()
                    },
                    old_tree.as_ref(),
                )
                .unwrap();
            tree
        })
    }

    pub fn range_for_syntax_ancestor<T: ToOffset>(&self, range: Range<T>) -> Option<Range<usize>> {
        if let Some(tree) = self.syntax_tree() {
            let root = tree.root_node();
            let range = range.start.to_offset(self)..range.end.to_offset(self);
            let mut node = root.descendant_for_byte_range(range.start, range.end);
            while node.map_or(false, |n| n.byte_range() == range) {
                node = node.unwrap().parent();
            }
            node.map(|n| n.byte_range())
        } else {
            None
        }
    }

    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(Range<usize>, Range<usize>)> {
        let (lang, tree) = self.language.as_ref().zip(self.syntax_tree())?;
        let open_capture_ix = lang.brackets_query.capture_index_for_name("open")?;
        let close_capture_ix = lang.brackets_query.capture_index_for_name("close")?;

        // Find bracket pairs that *inclusively* contain the given range.
        let range = range.start.to_offset(self).saturating_sub(1)..range.end.to_offset(self) + 1;
        let mut cursor = QueryCursorHandle::new();
        let matches = cursor.set_byte_range(range).matches(
            &lang.brackets_query,
            tree.root_node(),
            TextProvider(&self.visible_text),
        );

        // Get the ranges of the innermost pair of brackets.
        matches
            .filter_map(|mat| {
                let open = mat.nodes_for_capture_index(open_capture_ix).next()?;
                let close = mat.nodes_for_capture_index(close_capture_ix).next()?;
                Some((open.byte_range(), close.byte_range()))
            })
            .min_by_key(|(open_range, close_range)| close_range.end - open_range.start)
    }

    fn diff(&self, new_text: Arc<str>, cx: &AppContext) -> Task<Diff> {
        // TODO: it would be nice to not allocate here.
        let old_text = self.text();
        let base_version = self.version();
        cx.background_executor().spawn(async move {
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
        cx: &mut ModelContext<Self>,
    ) -> Option<Vec<Operation>> {
        if self.version == diff.base_version {
            self.start_transaction(None).unwrap();
            let mut operations = Vec::new();
            let mut offset = 0;
            for (tag, len) in diff.changes {
                let range = offset..(offset + len);
                match tag {
                    ChangeTag::Equal => offset += len,
                    ChangeTag::Delete => {
                        operations.extend_from_slice(&self.edit(Some(range), "", Some(cx)).unwrap())
                    }
                    ChangeTag::Insert => {
                        operations.extend_from_slice(
                            &self
                                .edit(Some(offset..offset), &diff.new_text[range], Some(cx))
                                .unwrap(),
                        );
                        offset += len;
                    }
                }
            }
            self.end_transaction(None, Some(cx)).unwrap();
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
        self.visible_text.summary()
    }

    pub fn text_summary_for_range(&self, range: Range<usize>) -> TextSummary {
        self.visible_text.cursor(range.start).summary(range.end)
    }

    pub fn len(&self) -> usize {
        self.fragments.extent::<usize>(&None)
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
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);
        self.visible_text.chunks_in_range(start..end)
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars_at(0)
    }

    pub fn chars_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = char> + '_ {
        let offset = position.to_offset(self);
        self.visible_text.chars_at(offset)
    }

    pub fn selections_changed_since(&self, since: SelectionsVersion) -> bool {
        self.selections_last_update != since
    }

    pub fn edits_since<'a>(&'a self, since: time::Global) -> impl 'a + Iterator<Item = Edit> {
        let since_2 = since.clone();
        let cursor = self.fragments.filter(
            move |summary| summary.max_version.changed_since(&since_2),
            &None,
        );

        Edits {
            deleted_text: &self.deleted_text,
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
        cx: Option<&mut ModelContext<Self>>,
    ) -> Result<()> {
        self.end_transaction_at(set_id, Instant::now(), cx)
    }

    fn end_transaction_at(
        &mut self,
        set_id: Option<SelectionSetId>,
        now: Instant,
        cx: Option<&mut ModelContext<Self>>,
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

            if let Some(cx) = cx {
                cx.notify();

                if self.edits_since(since).next().is_some() {
                    self.did_edit(was_dirty, cx);
                    self.reparse(cx);
                }
            }
        }

        Ok(())
    }

    pub fn edit<I, S, T>(
        &mut self,
        old_ranges: I,
        new_text: T,
        cx: Option<&mut ModelContext<Self>>,
    ) -> Result<Vec<Operation>>
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        self.start_transaction_at(None, Instant::now())?;

        let new_text = new_text.into();
        let old_ranges = old_ranges
            .into_iter()
            .map(|range| range.start.to_offset(self)..range.end.to_offset(self))
            .collect::<Vec<Range<usize>>>();

        let new_text = if new_text.len() > 0 {
            Some(new_text)
        } else {
            None
        };

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

        self.end_transaction_at(None, Instant::now(), cx)?;

        Ok(ops)
    }

    fn did_edit(&self, was_dirty: bool, cx: &mut ModelContext<Self>) {
        cx.emit(Event::Edited);
        if !was_dirty {
            cx.emit(Event::Dirtied);
        }
    }

    pub fn add_selection_set(
        &mut self,
        selections: impl Into<Arc<[Selection]>>,
        cx: Option<&mut ModelContext<Self>>,
    ) -> (SelectionSetId, Operation) {
        let selections = selections.into();
        let lamport_timestamp = self.lamport_clock.tick();
        self.selections
            .insert(lamport_timestamp, Arc::clone(&selections));
        self.selections_last_update += 1;

        if let Some(cx) = cx {
            cx.notify();
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
        cx: Option<&mut ModelContext<Self>>,
    ) -> Result<Operation> {
        let selections = selections.into();
        self.selections.insert(set_id, selections.clone());

        let lamport_timestamp = self.lamport_clock.tick();
        self.selections_last_update += 1;

        if let Some(cx) = cx {
            cx.notify();
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
        cx: Option<&mut ModelContext<Self>>,
    ) -> Result<Operation> {
        self.selections
            .remove(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        let lamport_timestamp = self.lamport_clock.tick();
        self.selections_last_update += 1;

        if let Some(cx) = cx {
            cx.notify();
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
        cx: Option<&mut ModelContext<Self>>,
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

        if let Some(cx) = cx {
            cx.notify();
            if self.edits_since(old_version).next().is_some() {
                self.did_edit(was_dirty, cx);
                self.reparse(cx);
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

        let mut old_visible_text = Rope::new();
        let mut old_deleted_text = Rope::new();
        let mut old_fragments = SumTree::new();
        mem::swap(&mut old_visible_text, &mut self.visible_text);
        mem::swap(&mut old_deleted_text, &mut self.deleted_text);
        mem::swap(&mut old_fragments, &mut self.fragments);

        let mut fragments_cursor = old_fragments.cursor::<FragmentIdRef, FragmentTextSummary>();

        let mut new_fragments = fragments_cursor.slice(
            &FragmentIdRef::new(&start_fragment_id),
            SeekBias::Left,
            &None,
        );
        let mut new_ropes =
            RopeBuilder::new(old_visible_text.cursor(0), old_deleted_text.cursor(0));
        new_ropes.push_tree(new_fragments.summary().text);

        let start_fragment = fragments_cursor.item().unwrap();
        if start_offset == start_fragment.range_in_insertion.end {
            let fragment = fragments_cursor.item().unwrap().clone();
            new_ropes.push_fragment(&fragment, fragment.visible);
            new_fragments.push(fragment, &None);
            fragments_cursor.next(&None);
        }

        while let Some(fragment) = fragments_cursor.item() {
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
                    fragments_cursor.prev_item().as_ref().unwrap(),
                    &fragment,
                    split_start..split_end,
                );
                let insertion = if let Some(new_text) = new_text {
                    let prev_fragment = fragments_cursor.prev_item();
                    Some(self.build_fragment_to_insert(
                        before_range.as_ref().or(prev_fragment).unwrap(),
                        within_range.as_ref().or(after_range.as_ref()),
                        new_text,
                        local_timestamp,
                        lamport_timestamp,
                    ))
                } else {
                    None
                };
                if let Some(fragment) = before_range {
                    new_ropes.push_fragment(&fragment, fragment.visible);
                    new_fragments.push(fragment, &None);
                }
                if let Some(fragment) = insertion {
                    new_ropes.push_str(new_text.take().unwrap());
                    new_fragments.push(fragment, &None);
                }
                if let Some(mut fragment) = within_range {
                    let fragment_was_visible = fragment.visible;
                    if fragment.was_visible(&version_in_range, &self.undo_map) {
                        fragment.deletions.insert(local_timestamp);
                        if fragment.visible {
                            fragment.visible = false;
                        }
                    }

                    new_ropes.push_fragment(&fragment, fragment_was_visible);
                    new_fragments.push(fragment, &None);
                }
                if let Some(fragment) = after_range {
                    new_ropes.push_fragment(&fragment, fragment.visible);
                    new_fragments.push(fragment, &None);
                }
            } else {
                if new_text.is_some() && lamport_timestamp > fragment.insertion.lamport_timestamp {
                    let new_text = new_text.take().unwrap();
                    let fragment = self.build_fragment_to_insert(
                        fragments_cursor.prev_item().as_ref().unwrap(),
                        Some(&fragment),
                        new_text,
                        local_timestamp,
                        lamport_timestamp,
                    );
                    new_ropes.push_str(new_text);
                    new_fragments.push(fragment, &None);
                }

                let fragment_was_visible = fragment.visible;
                if fragment.id < end_fragment_id
                    && fragment.was_visible(&version_in_range, &self.undo_map)
                {
                    fragment.deletions.insert(local_timestamp);
                    if fragment.visible {
                        fragment.visible = false;
                    }
                }

                new_ropes.push_fragment(&fragment, fragment_was_visible);
                new_fragments.push(fragment, &None);
            }

            fragments_cursor.next(&None);
        }

        if let Some(new_text) = new_text {
            let fragment = self.build_fragment_to_insert(
                fragments_cursor.prev_item().as_ref().unwrap(),
                None,
                new_text,
                local_timestamp,
                lamport_timestamp,
            );
            new_ropes.push_str(new_text);
            new_fragments.push(fragment, &None);
        }

        let (visible_text, deleted_text) = new_ropes.finish();
        new_fragments.push_tree(fragments_cursor.suffix(&None), &None);

        self.fragments = new_fragments;
        self.visible_text = visible_text;
        self.deleted_text = deleted_text;
        self.local_clock.observe(local_timestamp);
        self.lamport_clock.observe(lamport_timestamp);
        Ok(())
    }

    pub fn undo(&mut self, mut cx: Option<&mut ModelContext<Self>>) -> Vec<Operation> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let mut ops = Vec::new();
        if let Some(transaction) = self.history.pop_undo() {
            let selections = transaction.selections_before.clone();
            for edit_id in transaction.edits.clone() {
                ops.push(self.undo_or_redo(edit_id).unwrap());
            }

            if let Some((set_id, selections)) = selections {
                let _ = self.update_selection_set(set_id, selections, cx.as_deref_mut());
            }
        }

        if let Some(cx) = cx {
            cx.notify();
            if self.edits_since(old_version).next().is_some() {
                self.did_edit(was_dirty, cx);
                self.reparse(cx);
            }
        }

        ops
    }

    pub fn redo(&mut self, mut cx: Option<&mut ModelContext<Self>>) -> Vec<Operation> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let mut ops = Vec::new();
        if let Some(transaction) = self.history.pop_redo() {
            let selections = transaction.selections_after.clone();
            for edit_id in transaction.edits.clone() {
                ops.push(self.undo_or_redo(edit_id).unwrap());
            }

            if let Some((set_id, selections)) = selections {
                let _ = self.update_selection_set(set_id, selections, cx.as_deref_mut());
            }
        }

        if let Some(cx) = cx {
            cx.notify();
            if self.edits_since(old_version).next().is_some() {
                self.did_edit(was_dirty, cx);
                self.reparse(cx);
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
        let mut old_visible_text = Rope::new();
        let mut old_deleted_text = Rope::new();
        mem::swap(&mut old_visible_text, &mut self.visible_text);
        mem::swap(&mut old_deleted_text, &mut self.deleted_text);
        let mut new_ropes =
            RopeBuilder::new(old_visible_text.cursor(0), old_deleted_text.cursor(0));

        self.undo_map.insert(undo);
        let edit = &self.history.ops[&undo.edit_id];
        let start_fragment_id = self.resolve_fragment_id(edit.start_id, edit.start_offset)?;
        let end_fragment_id = self.resolve_fragment_id(edit.end_id, edit.end_offset)?;

        let mut fragments_cursor = self.fragments.cursor::<FragmentIdRef, ()>();

        if edit.start_id == edit.end_id && edit.start_offset == edit.end_offset {
            let splits = &self.insertion_splits[&undo.edit_id];
            let mut insertion_splits = splits.cursor::<(), ()>().map(|s| &s.fragment_id).peekable();

            let first_split_id = insertion_splits.next().unwrap();
            new_fragments =
                fragments_cursor.slice(&FragmentIdRef::new(first_split_id), SeekBias::Left, &None);
            new_ropes.push_tree(new_fragments.summary().text);

            loop {
                let mut fragment = fragments_cursor.item().unwrap().clone();
                let was_visible = fragment.visible;
                fragment.visible = fragment.is_visible(&self.undo_map);
                fragment.max_undos.observe(undo.id);

                new_ropes.push_fragment(&fragment, was_visible);
                new_fragments.push(fragment.clone(), &None);

                fragments_cursor.next(&None);
                if let Some(split_id) = insertion_splits.next() {
                    let slice = fragments_cursor.slice(
                        &FragmentIdRef::new(split_id),
                        SeekBias::Left,
                        &None,
                    );
                    new_ropes.push_tree(slice.summary().text);
                    new_fragments.push_tree(slice, &None);
                } else {
                    break;
                }
            }
        } else {
            new_fragments = fragments_cursor.slice(
                &FragmentIdRef::new(&start_fragment_id),
                SeekBias::Left,
                &None,
            );
            new_ropes.push_tree(new_fragments.summary().text);

            while let Some(fragment) = fragments_cursor.item() {
                if fragment.id > end_fragment_id {
                    break;
                } else {
                    let mut fragment = fragment.clone();
                    let fragment_was_visible = fragment.visible;
                    if edit.version_in_range.observed(fragment.insertion.id)
                        || fragment.insertion.id == undo.edit_id
                    {
                        fragment.visible = fragment.is_visible(&self.undo_map);
                        fragment.max_undos.observe(undo.id);
                    }

                    new_ropes.push_fragment(&fragment, fragment_was_visible);
                    new_fragments.push(fragment, &None);
                    fragments_cursor.next(&None);
                }
            }
        }

        new_fragments.push_tree(fragments_cursor.suffix(&None), &None);
        let (visible_text, deleted_text) = new_ropes.finish();
        drop(fragments_cursor);

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
                Operation::Edit { edit, .. } => {
                    self.version.observed(edit.start_id)
                        && self.version.observed(edit.end_id)
                        && edit.version_in_range <= self.version
                }
                Operation::Undo { undo, .. } => self.version.observed(undo.edit_id),
                Operation::UpdateSelections { selections, .. } => {
                    if let Some(selections) = selections {
                        selections.iter().all(|selection| {
                            let contains_start = match &selection.start {
                                Anchor::Middle { version, .. } => self.version >= *version,
                                _ => true,
                            };
                            let contains_end = match &selection.end {
                                Anchor::Middle { version, .. } => self.version >= *version,
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

        let mut old_fragments = SumTree::new();
        let mut old_visible_text = Rope::new();
        let mut old_deleted_text = Rope::new();
        mem::swap(&mut old_visible_text, &mut self.visible_text);
        mem::swap(&mut old_deleted_text, &mut self.deleted_text);
        mem::swap(&mut old_fragments, &mut self.fragments);

        let mut fragments_cursor = old_fragments.cursor::<usize, usize>();
        let mut new_fragments =
            fragments_cursor.slice(&cur_range.as_ref().unwrap().start, SeekBias::Right, &None);

        let mut new_ropes =
            RopeBuilder::new(old_visible_text.cursor(0), old_deleted_text.cursor(0));
        new_ropes.push_tree(new_fragments.summary().text);

        let mut start_id = None;
        let mut start_offset = None;
        let mut end_id = None;
        let mut end_offset = None;
        let mut version_in_range = time::Global::new();

        let mut local_timestamp = self.local_clock.tick();
        let mut lamport_timestamp = self.lamport_clock.tick();

        while cur_range.is_some() && fragments_cursor.item().is_some() {
            let mut fragment = fragments_cursor.item().unwrap().clone();
            let fragment_summary = fragments_cursor.item_summary().unwrap();
            let mut fragment_start = *fragments_cursor.start();
            let mut fragment_end = fragment_start + fragment.visible_len();
            let fragment_was_visible = fragment.visible;

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

                    new_ropes.push_fragment(&prefix, prefix.visible);
                    new_fragments.push(prefix.clone(), &None);
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

                        new_ropes.push_str(&new_text);
                        new_fragments.push(new_fragment, &None);
                    }
                }

                if range.end < fragment_end {
                    if range.end > fragment_start {
                        let mut prefix = fragment.clone();
                        prefix.range_in_insertion.end =
                            prefix.range_in_insertion.start + (range.end - fragment_start);
                        prefix.id =
                            FragmentId::between(&new_fragments.last().unwrap().id, &fragment.id);
                        version_in_range.join(&fragment_summary.max_version);
                        if prefix.visible {
                            prefix.deletions.insert(local_timestamp);
                            prefix.visible = false;
                        }
                        fragment.range_in_insertion.start = prefix.range_in_insertion.end;
                        new_ropes.push_fragment(&prefix, fragment_was_visible);
                        new_fragments.push(prefix.clone(), &None);
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
                    version_in_range.join(&fragment_summary.max_version);
                    if fragment.visible {
                        fragment.deletions.insert(local_timestamp);
                        fragment.visible = false;
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
            splits_cursor.next(&());
            new_split_tree.push_tree(
                splits_cursor.slice(&old_split_tree.extent::<usize>(&()), SeekBias::Right, &()),
                &(),
            );
            self.insertion_splits
                .insert(fragment.insertion.id, new_split_tree);

            new_ropes.push_fragment(&fragment, fragment_was_visible);
            new_fragments.push(fragment, &None);

            // Scan forward until we find a fragment that is not fully contained by the current splice.
            fragments_cursor.next(&None);
            if let Some(range) = cur_range.clone() {
                while let Some(fragment) = fragments_cursor.item() {
                    let fragment_summary = fragments_cursor.item_summary().unwrap();
                    let fragment_was_visible = fragment.visible;
                    fragment_start = *fragments_cursor.start();
                    fragment_end = fragment_start + fragment.visible_len();
                    if range.start < fragment_start && range.end >= fragment_end {
                        let mut new_fragment = fragment.clone();
                        version_in_range.join(&fragment_summary.max_version);
                        if new_fragment.visible {
                            new_fragment.deletions.insert(local_timestamp);
                            new_fragment.visible = false;
                        }

                        new_ropes.push_fragment(&new_fragment, fragment_was_visible);
                        new_fragments.push(new_fragment, &None);
                        fragments_cursor.next(&None);

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
                    let slice = fragments_cursor.slice(
                        &cur_range.as_ref().unwrap().start,
                        SeekBias::Right,
                        &None,
                    );
                    new_ropes.push_tree(slice.summary().text);
                    new_fragments.push_tree(slice, &None);
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

                new_ropes.push_str(&new_text);
                new_fragments.push(new_fragment, &None);
            }
        }

        new_fragments.push_tree(fragments_cursor.suffix(&None), &None);
        let (visible_text, deleted_text) = new_ropes.finish();

        self.fragments = new_fragments;
        self.visible_text = visible_text;
        self.deleted_text = deleted_text;
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

            cursor.next(&());
            new_split_tree.push_tree(
                cursor.slice(&old_split_tree.extent::<usize>(&()), SeekBias::Right, &()),
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

        let range_in_insertion = 0..text.len();
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

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, AnchorBias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, AnchorBias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: AnchorBias) -> Anchor {
        let offset = position.to_offset(self);
        let max_offset = self.len();
        assert!(offset <= max_offset, "offset is out of range");

        if offset == 0 && bias == AnchorBias::Left {
            Anchor::Start
        } else if offset == max_offset && bias == AnchorBias::Right {
            Anchor::End
        } else {
            let mut cursor = self.fragments.cursor::<usize, FragmentTextSummary>();
            cursor.seek(&offset, bias.to_seek_bias(), &None);
            Anchor::Middle {
                offset: offset + cursor.start().deleted,
                bias,
                version: self.version(),
            }
        }
    }

    fn summary_for_anchor(&self, anchor: &Anchor) -> TextSummary {
        match anchor {
            Anchor::Start => TextSummary::default(),
            Anchor::End => self.text_summary(),
            Anchor::Middle {
                offset,
                bias,
                version,
            } => {
                let mut cursor = self
                    .fragments
                    .cursor::<VersionedOffset, (VersionedOffset, usize)>();
                cursor.seek(
                    &VersionedOffset::Offset(*offset),
                    bias.to_seek_bias(),
                    &Some(version.clone()),
                );
                let fragment = cursor.item().unwrap();
                let overshoot = if fragment.visible {
                    offset - cursor.start().0.offset()
                } else {
                    0
                };

                self.text_summary_for_range(0..cursor.start().1 + overshoot)
            }
        }
    }

    fn fragment_ix_for_anchor(&self, anchor: &Anchor) -> (usize, usize) {
        match anchor {
            Anchor::Start => (0, 0),
            Anchor::End => (
                self.fragments.extent::<FragmentCount>(&None).0,
                self.fragments.last().map_or(0, |f| f.visible_len()),
            ),
            Anchor::Middle {
                offset,
                bias,
                version,
            } => {
                let mut cursor = self
                    .fragments
                    .cursor::<VersionedOffset, (VersionedOffset, FragmentCount)>();
                cursor.seek(
                    &VersionedOffset::Offset(*offset),
                    bias.to_seek_bias(),
                    &Some(version.clone()),
                );
                let count = cursor.start().1;
                (count.0, offset - cursor.start().0.offset())
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

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.visible_text.clip_point(point, bias)
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        self.visible_text.clip_offset(offset, bias)
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self {
            fragments: self.fragments.clone(),
            visible_text: self.visible_text.clone(),
            deleted_text: self.deleted_text.clone(),
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
            language: self.language.clone(),
            syntax_tree: Mutex::new(self.syntax_tree.lock().clone()),
            is_parsing: false,
            deferred_replicas: self.deferred_replicas.clone(),
            replica_id: self.replica_id,
            local_clock: self.local_clock.clone(),
            lamport_clock: self.lamport_clock.clone(),
        }
    }
}

pub struct Snapshot {
    text: Rope,
    tree: Option<Tree>,
    language: Option<Arc<Language>>,
    query_cursor: QueryCursorHandle,
}

impl Snapshot {
    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn text(&self) -> Rope {
        self.text.clone()
    }

    pub fn text_for_range(&self, range: Range<usize>) -> Chunks {
        self.text.chunks_in_range(range)
    }

    pub fn highlighted_text_for_range(&mut self, range: Range<usize>) -> HighlightedChunks {
        let chunks = self.text.chunks_in_range(range.clone());
        if let Some((language, tree)) = self.language.as_ref().zip(self.tree.as_ref()) {
            let captures = self.query_cursor.set_byte_range(range.clone()).captures(
                &language.highlight_query,
                tree.root_node(),
                TextProvider(&self.text),
            );

            HighlightedChunks {
                range,
                chunks,
                highlights: Some(Highlights {
                    captures,
                    next_capture: None,
                    stack: Default::default(),
                    theme_mapping: language.theme_mapping(),
                }),
            }
        } else {
            HighlightedChunks {
                range,
                chunks,
                highlights: None,
            }
        }
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        self.text.clip_offset(offset, bias)
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.text.clip_point(point, bias)
    }

    pub fn to_offset(&self, point: Point) -> usize {
        self.text.to_offset(point)
    }

    pub fn to_point(&self, offset: usize) -> Point {
        self.text.to_point(offset)
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
        self.push(fragment.len(), was_visible, fragment.visible)
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Edited,
    Dirtied,
    Saved,
    FileHandleChanged,
    Reloaded,
    Reparsed,
}

impl Entity for Buffer {
    type Event = Event;
}

impl<'a, F: Fn(&FragmentSummary) -> bool> Iterator for Edits<'a, F> {
    type Item = Edit;

    fn next(&mut self) -> Option<Self::Item> {
        let mut change: Option<Edit> = None;

        while let Some(fragment) = self.cursor.item() {
            let new_offset = self.cursor.start().visible;
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
                        old_lines: Point::zero(),
                    });
                    self.delta += fragment.len() as isize;
                }
            } else if fragment.was_visible(&self.since, &self.undos) && !fragment.visible {
                let deleted_start = self.cursor.start().deleted;
                let old_lines = self.deleted_text.to_point(deleted_start + fragment.len())
                    - self.deleted_text.to_point(deleted_start);
                if let Some(ref mut change) = change {
                    if change.new_range.end == new_offset {
                        change.old_range.end += fragment.len();
                        change.old_lines += &old_lines;
                        self.delta -= fragment.len() as isize;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_range: old_offset..old_offset + fragment.len(),
                        new_range: new_offset..new_offset,
                        old_lines,
                    });
                    self.delta -= fragment.len() as isize;
                }
            }

            self.cursor.next(&None);
        }

        change
    }
}

struct ByteChunks<'a>(rope::Chunks<'a>);

impl<'a> Iterator for ByteChunks<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(str::as_bytes)
    }
}

struct TextProvider<'a>(&'a Rope);

impl<'a> tree_sitter::TextProvider<'a> for TextProvider<'a> {
    type I = ByteChunks<'a>;

    fn text(&mut self, node: tree_sitter::Node) -> Self::I {
        ByteChunks(self.0.chunks_in_range(node.byte_range()))
    }
}

struct Highlights<'a> {
    captures: tree_sitter::QueryCaptures<'a, 'a, TextProvider<'a>>,
    next_capture: Option<(tree_sitter::QueryMatch<'a, 'a>, usize)>,
    stack: Vec<(usize, StyleId)>,
    theme_mapping: ThemeMap,
}

pub struct HighlightedChunks<'a> {
    range: Range<usize>,
    chunks: Chunks<'a>,
    highlights: Option<Highlights<'a>>,
}

impl<'a> HighlightedChunks<'a> {
    pub fn seek(&mut self, offset: usize) {
        self.range.start = offset;
        self.chunks.seek(self.range.start);
        if let Some(highlights) = self.highlights.as_mut() {
            highlights
                .stack
                .retain(|(end_offset, _)| *end_offset > offset);
            if let Some((mat, capture_ix)) = &highlights.next_capture {
                let capture = mat.captures[*capture_ix as usize];
                if offset >= capture.node.start_byte() {
                    let next_capture_end = capture.node.end_byte();
                    if offset < next_capture_end {
                        highlights.stack.push((
                            next_capture_end,
                            highlights.theme_mapping.get(capture.index),
                        ));
                    }
                    highlights.next_capture.take();
                }
            }
            highlights.captures.set_byte_range(self.range.clone());
        }
    }

    pub fn offset(&self) -> usize {
        self.range.start
    }
}

impl<'a> Iterator for HighlightedChunks<'a> {
    type Item = (&'a str, StyleId);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_capture_start = usize::MAX;

        if let Some(highlights) = self.highlights.as_mut() {
            while let Some((parent_capture_end, _)) = highlights.stack.last() {
                if *parent_capture_end <= self.range.start {
                    highlights.stack.pop();
                } else {
                    break;
                }
            }

            if highlights.next_capture.is_none() {
                highlights.next_capture = highlights.captures.next();
            }

            while let Some((mat, capture_ix)) = highlights.next_capture.as_ref() {
                let capture = mat.captures[*capture_ix as usize];
                if self.range.start < capture.node.start_byte() {
                    next_capture_start = capture.node.start_byte();
                    break;
                } else {
                    let style_id = highlights.theme_mapping.get(capture.index);
                    highlights.stack.push((capture.node.end_byte(), style_id));
                    highlights.next_capture = highlights.captures.next();
                }
            }
        }

        if let Some(chunk) = self.chunks.peek() {
            let chunk_start = self.range.start;
            let mut chunk_end = (self.chunks.offset() + chunk.len()).min(next_capture_start);
            let mut style_id = StyleId::default();
            if let Some((parent_capture_end, parent_style_id)) =
                self.highlights.as_ref().and_then(|h| h.stack.last())
            {
                chunk_end = chunk_end.min(*parent_capture_end);
                style_id = *parent_style_id;
            }

            let slice =
                &chunk[chunk_start - self.chunks.offset()..chunk_end - self.chunks.offset()];
            self.range.start = chunk_end;
            if self.range.start == self.chunks.offset() + chunk.len() {
                self.chunks.next().unwrap();
            }

            Some((slice, style_id))
        } else {
            None
        }
    }
}

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
    fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<time::Global>) {
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
        max_version.join(&self.max_undos);

        let mut min_insertion_version = time::Global::new();
        min_insertion_version.observe(self.insertion.id);
        let max_insertion_version = min_insertion_version.clone();
        if self.visible {
            FragmentSummary {
                text: FragmentTextSummary {
                    visible: self.len(),
                    deleted: 0,
                },
                max_fragment_id: self.id.clone(),
                max_version,
                min_insertion_version,
                max_insertion_version,
                count: 1,
            }
        } else {
            FragmentSummary {
                text: FragmentTextSummary {
                    visible: 0,
                    deleted: self.len(),
                },
                max_fragment_id: self.id.clone(),
                max_version,
                min_insertion_version,
                max_insertion_version,
                count: 1,
            }
        }
    }
}

impl sum_tree::Summary for FragmentSummary {
    type Context = Option<time::Global>;

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.text.visible += &other.text.visible;
        self.text.deleted += &other.text.deleted;
        debug_assert!(self.max_fragment_id <= other.max_fragment_id);
        self.max_fragment_id = other.max_fragment_id.clone();
        self.max_version.join(&other.max_version);
        self.min_insertion_version
            .meet(&other.min_insertion_version);
        self.max_insertion_version
            .join(&other.max_insertion_version);
        self.count += other.count;
    }
}

impl Default for FragmentSummary {
    fn default() -> Self {
        FragmentSummary {
            text: FragmentTextSummary::default(),
            max_fragment_id: FragmentId::min_value().clone(),
            max_version: time::Global::new(),
            min_insertion_version: time::Global::new(),
            max_insertion_version: time::Global::new(),
            count: 0,
        }
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for usize {
    fn add_summary(&mut self, summary: &FragmentSummary, _: &Option<time::Global>) {
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
    fn add_summary(&mut self, summary: &InsertionSplitSummary, _: &()) {
        *self += summary.extent;
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
    fn add_summary(&mut self, summary: &'a FragmentSummary, cx: &Option<time::Global>) {
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
        } else {
            unreachable!();
        }
    }
}

impl<'a> sum_tree::SeekDimension<'a, FragmentSummary> for VersionedOffset {
    fn cmp(&self, other: &Self, _: &Option<time::Global>) -> cmp::Ordering {
        match (self, other) {
            (Self::Offset(a), Self::Offset(b)) => Ord::cmp(a, b),
            (Self::Offset(_), Self::InvalidVersion) => cmp::Ordering::Less,
            (Self::InvalidVersion, _) => unreachable!(),
        }
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for (VersionedOffset, usize) {
    fn add_summary(&mut self, summary: &'a FragmentSummary, cx: &Option<time::Global>) {
        self.0.add_summary(summary, cx);
        self.1 += summary.text.visible;
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for (VersionedOffset, FragmentCount) {
    fn add_summary(&mut self, summary: &'a FragmentSummary, cx: &Option<time::Global>) {
        self.0.add_summary(summary, cx);
        self.1 .0 += summary.count;
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct FragmentCount(usize);

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FragmentCount {
    fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<time::Global>) {
        self.0 += summary.count;
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
    fn to_offset(&self, buffer: &Buffer) -> usize;
}

impl ToOffset for Point {
    fn to_offset(&self, buffer: &Buffer) -> usize {
        buffer.visible_text.to_offset(*self)
    }
}

impl ToOffset for usize {
    fn to_offset(&self, _: &Buffer) -> usize {
        *self
    }
}

impl ToOffset for Anchor {
    fn to_offset(&self, buffer: &Buffer) -> usize {
        buffer.summary_for_anchor(self).bytes
    }
}

impl<'a> ToOffset for &'a Anchor {
    fn to_offset(&self, buffer: &Buffer) -> usize {
        buffer.summary_for_anchor(self).bytes
    }
}

pub trait ToPoint {
    fn to_point(&self, buffer: &Buffer) -> Point;
}

impl ToPoint for Anchor {
    fn to_point(&self, buffer: &Buffer) -> Point {
        buffer.summary_for_anchor(self).lines
    }
}

impl ToPoint for usize {
    fn to_point(&self, buffer: &Buffer) -> Point {
        buffer.visible_text.to_point(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test::{build_app_state, temp_tree},
        util::RandomCharIter,
        worktree::{Worktree, WorktreeHandle},
    };
    use gpui::{App, ModelHandle};
    use rand::prelude::*;
    use serde_json::json;
    use std::{
        cell::RefCell,
        cmp::Ordering,
        env, fs,
        rc::Rc,
        sync::atomic::{self, AtomicUsize},
    };

    #[gpui::test]
    fn test_edit(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "abc", cx);
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
    fn test_edit_events(cx: &mut gpui::MutableAppContext) {
        let mut now = Instant::now();
        let buffer_1_events = Rc::new(RefCell::new(Vec::new()));
        let buffer_2_events = Rc::new(RefCell::new(Vec::new()));

        let buffer1 = cx.add_model(|cx| Buffer::new(0, "abcdef", cx));
        let buffer2 = cx.add_model(|cx| Buffer::new(1, "abcdef", cx));
        let mut buffer_ops = Vec::new();
        buffer1.update(cx, |buffer, cx| {
            let buffer_1_events = buffer_1_events.clone();
            cx.subscribe(&buffer1, move |_, event, _| {
                buffer_1_events.borrow_mut().push(event.clone())
            });
            let buffer_2_events = buffer_2_events.clone();
            cx.subscribe(&buffer2, move |_, event, _| {
                buffer_2_events.borrow_mut().push(event.clone())
            });

            // An edit emits an edited event, followed by a dirtied event,
            // since the buffer was previously in a clean state.
            let ops = buffer.edit(Some(2..4), "XYZ", Some(cx)).unwrap();
            buffer_ops.extend_from_slice(&ops);

            // An empty transaction does not emit any events.
            buffer.start_transaction(None).unwrap();
            buffer.end_transaction(None, Some(cx)).unwrap();

            // A transaction containing two edits emits one edited event.
            now += Duration::from_secs(1);
            buffer.start_transaction_at(None, now).unwrap();
            let ops = buffer.edit(Some(5..5), "u", Some(cx)).unwrap();
            buffer_ops.extend_from_slice(&ops);
            let ops = buffer.edit(Some(6..6), "w", Some(cx)).unwrap();
            buffer_ops.extend_from_slice(&ops);
            buffer.end_transaction_at(None, now, Some(cx)).unwrap();

            // Undoing a transaction emits one edited event.
            let ops = buffer.undo(Some(cx));
            buffer_ops.extend_from_slice(&ops);
        });

        // Incorporating a set of remote ops emits a single edited event,
        // followed by a dirtied event.
        buffer2.update(cx, |buffer, cx| {
            buffer.apply_ops(buffer_ops, Some(cx)).unwrap();
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
    fn test_random_edits(cx: &mut gpui::MutableAppContext) {
        for seed in 0..100 {
            println!("{:?}", seed);
            let mut rng = &mut StdRng::seed_from_u64(seed);

            let reference_string_len = rng.gen_range(0..3);
            let mut reference_string = RandomCharIter::new(&mut rng)
                .take(reference_string_len)
                .collect::<String>();
            cx.add_model(|cx| {
                let mut buffer = Buffer::new(0, reference_string.as_str(), cx);
                let mut buffer_versions = Vec::new();
                for _i in 0..10 {
                    let (old_ranges, new_text, _) = buffer.randomly_mutate(rng, None);
                    for old_range in old_ranges.iter().rev() {
                        reference_string.replace_range(old_range.clone(), &new_text);
                    }
                    assert_eq!(buffer.text(), reference_string);

                    if rng.gen_bool(0.25) {
                        buffer.randomly_undo_redo(rng);
                        reference_string = buffer.text();
                    }

                    let range = buffer.random_byte_range(0, rng);
                    assert_eq!(
                        buffer.text_summary_for_range(range.clone()),
                        TextSummary::from(&reference_string[range])
                    );

                    if rng.gen_bool(0.3) {
                        buffer_versions.push(buffer.clone());
                    }
                }

                for mut old_buffer in buffer_versions {
                    let mut delta = 0_isize;
                    for Edit {
                        old_range,
                        new_range,
                        ..
                    } in buffer.edits_since(old_buffer.version.clone())
                    {
                        let old_len = old_range.end - old_range.start;
                        let new_len = new_range.end - new_range.start;
                        let old_start = (old_range.start as isize + delta) as usize;
                        let new_text: String = buffer.text_for_range(new_range).collect();
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
    fn test_line_len(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx);
            buffer.edit(vec![0..0], "abcd\nefg\nhij", None).unwrap();
            buffer.edit(vec![12..12], "kl\nmno", None).unwrap();
            buffer.edit(vec![18..18], "\npqrs\n", None).unwrap();
            buffer.edit(vec![18..21], "\nPQ", None).unwrap();

            assert_eq!(buffer.line_len(0), 4);
            assert_eq!(buffer.line_len(1), 3);
            assert_eq!(buffer.line_len(2), 5);
            assert_eq!(buffer.line_len(3), 3);
            assert_eq!(buffer.line_len(4), 4);
            assert_eq!(buffer.line_len(5), 0);
            buffer
        });
    }

    #[gpui::test]
    fn test_text_summary_for_range(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let buffer = Buffer::new(0, "ab\nefg\nhklm\nnopqrs\ntuvwxyz", cx);
            assert_eq!(
                buffer.text_summary_for_range(1..3),
                TextSummary {
                    bytes: 2,
                    lines: Point::new(1, 0),
                    first_line_chars: 1,
                    last_line_chars: 0,
                    longest_row: 0,
                    longest_row_chars: 1,
                }
            );
            assert_eq!(
                buffer.text_summary_for_range(1..12),
                TextSummary {
                    bytes: 11,
                    lines: Point::new(3, 0),
                    first_line_chars: 1,
                    last_line_chars: 0,
                    longest_row: 2,
                    longest_row_chars: 4,
                }
            );
            assert_eq!(
                buffer.text_summary_for_range(0..20),
                TextSummary {
                    bytes: 20,
                    lines: Point::new(4, 1),
                    first_line_chars: 2,
                    last_line_chars: 1,
                    longest_row: 3,
                    longest_row_chars: 6,
                }
            );
            assert_eq!(
                buffer.text_summary_for_range(0..22),
                TextSummary {
                    bytes: 22,
                    lines: Point::new(4, 3),
                    first_line_chars: 2,
                    last_line_chars: 3,
                    longest_row: 3,
                    longest_row_chars: 6,
                }
            );
            assert_eq!(
                buffer.text_summary_for_range(7..22),
                TextSummary {
                    bytes: 15,
                    lines: Point::new(2, 3),
                    first_line_chars: 4,
                    last_line_chars: 3,
                    longest_row: 1,
                    longest_row_chars: 6,
                }
            );
            buffer
        });
    }

    #[gpui::test]
    fn test_chars_at(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx);
            buffer.edit(vec![0..0], "abcd\nefgh\nij", None).unwrap();
            buffer.edit(vec![12..12], "kl\nmno", None).unwrap();
            buffer.edit(vec![18..18], "\npqrs", None).unwrap();
            buffer.edit(vec![18..21], "\nPQ", None).unwrap();

            let chars = buffer.chars_at(Point::new(0, 0));
            assert_eq!(chars.collect::<String>(), "abcd\nefgh\nijkl\nmno\nPQrs");

            let chars = buffer.chars_at(Point::new(1, 0));
            assert_eq!(chars.collect::<String>(), "efgh\nijkl\nmno\nPQrs");

            let chars = buffer.chars_at(Point::new(2, 0));
            assert_eq!(chars.collect::<String>(), "ijkl\nmno\nPQrs");

            let chars = buffer.chars_at(Point::new(3, 0));
            assert_eq!(chars.collect::<String>(), "mno\nPQrs");

            let chars = buffer.chars_at(Point::new(4, 0));
            assert_eq!(chars.collect::<String>(), "PQrs");

            // Regression test:
            let mut buffer = Buffer::new(0, "", cx);
            buffer.edit(vec![0..0], "[workspace]\nmembers = [\n    \"xray_core\",\n    \"xray_server\",\n    \"xray_cli\",\n    \"xray_wasm\",\n]\n", None).unwrap();
            buffer.edit(vec![60..60], "\n", None).unwrap();

            let chars = buffer.chars_at(Point::new(6, 0));
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
    fn test_anchors(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx);
            buffer.edit(vec![0..0], "abc", None).unwrap();
            let left_anchor = buffer.anchor_before(2);
            let right_anchor = buffer.anchor_after(2);

            buffer.edit(vec![1..1], "def\n", None).unwrap();
            assert_eq!(buffer.text(), "adef\nbc");
            assert_eq!(left_anchor.to_offset(&buffer), 6);
            assert_eq!(right_anchor.to_offset(&buffer), 6);
            assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
            assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

            buffer.edit(vec![2..3], "", None).unwrap();
            assert_eq!(buffer.text(), "adf\nbc");
            assert_eq!(left_anchor.to_offset(&buffer), 5);
            assert_eq!(right_anchor.to_offset(&buffer), 5);
            assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
            assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

            buffer.edit(vec![5..5], "ghi\n", None).unwrap();
            assert_eq!(buffer.text(), "adf\nbghi\nc");
            assert_eq!(left_anchor.to_offset(&buffer), 5);
            assert_eq!(right_anchor.to_offset(&buffer), 9);
            assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
            assert_eq!(right_anchor.to_point(&buffer), Point { row: 2, column: 0 });

            buffer.edit(vec![7..9], "", None).unwrap();
            assert_eq!(buffer.text(), "adf\nbghc");
            assert_eq!(left_anchor.to_offset(&buffer), 5);
            assert_eq!(right_anchor.to_offset(&buffer), 7);
            assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 },);
            assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 3 });

            // Ensure anchoring to a point is equivalent to anchoring to an offset.
            assert_eq!(
                buffer.anchor_before(Point { row: 0, column: 0 }),
                buffer.anchor_before(0)
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 0, column: 1 }),
                buffer.anchor_before(1)
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 0, column: 2 }),
                buffer.anchor_before(2)
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 0, column: 3 }),
                buffer.anchor_before(3)
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 0 }),
                buffer.anchor_before(4)
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 1 }),
                buffer.anchor_before(5)
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 2 }),
                buffer.anchor_before(6)
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 3 }),
                buffer.anchor_before(7)
            );
            assert_eq!(
                buffer.anchor_before(Point { row: 1, column: 4 }),
                buffer.anchor_before(8)
            );

            // Comparison between anchors.
            let anchor_at_offset_0 = buffer.anchor_before(0);
            let anchor_at_offset_1 = buffer.anchor_before(1);
            let anchor_at_offset_2 = buffer.anchor_before(2);

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
    fn test_anchors_at_start_and_end(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx);
            let before_start_anchor = buffer.anchor_before(0);
            let after_end_anchor = buffer.anchor_after(0);

            buffer.edit(vec![0..0], "abc", None).unwrap();
            assert_eq!(buffer.text(), "abc");
            assert_eq!(before_start_anchor.to_offset(&buffer), 0);
            assert_eq!(after_end_anchor.to_offset(&buffer), 3);

            let after_start_anchor = buffer.anchor_after(0);
            let before_end_anchor = buffer.anchor_before(3);

            buffer.edit(vec![3..3], "def", None).unwrap();
            buffer.edit(vec![0..0], "ghi", None).unwrap();
            assert_eq!(buffer.text(), "ghiabcdef");
            assert_eq!(before_start_anchor.to_offset(&buffer), 0);
            assert_eq!(after_start_anchor.to_offset(&buffer), 3);
            assert_eq!(before_end_anchor.to_offset(&buffer), 6);
            assert_eq!(after_end_anchor.to_offset(&buffer), 9);
            buffer
        });
    }

    #[test]
    fn test_is_dirty() {
        App::test_async((), |mut cx| async move {
            let dir = temp_tree(json!({
                "file1": "",
                "file2": "",
                "file3": "",
            }));
            let tree = cx.add_model(|cx| Worktree::new(dir.path(), cx));
            tree.flush_fs_events(&cx).await;
            cx.read(|cx| tree.read(cx).scan_complete()).await;

            let file1 = cx.update(|cx| tree.file("file1", cx)).await;
            let buffer1 = cx.add_model(|cx| {
                Buffer::from_history(0, History::new("abc".into()), Some(file1), None, cx)
            });
            let events = Rc::new(RefCell::new(Vec::new()));

            // initially, the buffer isn't dirty.
            buffer1.update(&mut cx, |buffer, cx| {
                cx.subscribe(&buffer1, {
                    let events = events.clone();
                    move |_, event, _| events.borrow_mut().push(event.clone())
                });

                assert!(!buffer.is_dirty());
                assert!(events.borrow().is_empty());

                buffer.edit(vec![1..2], "", Some(cx)).unwrap();
            });

            // after the first edit, the buffer is dirty, and emits a dirtied event.
            buffer1.update(&mut cx, |buffer, cx| {
                assert!(buffer.text() == "ac");
                assert!(buffer.is_dirty());
                assert_eq!(*events.borrow(), &[Event::Edited, Event::Dirtied]);
                events.borrow_mut().clear();

                buffer.did_save(buffer.version(), None, cx);
            });

            // after saving, the buffer is not dirty, and emits a saved event.
            buffer1.update(&mut cx, |buffer, cx| {
                assert!(!buffer.is_dirty());
                assert_eq!(*events.borrow(), &[Event::Saved]);
                events.borrow_mut().clear();

                buffer.edit(vec![1..1], "B", Some(cx)).unwrap();
                buffer.edit(vec![2..2], "D", Some(cx)).unwrap();
            });

            // after editing again, the buffer is dirty, and emits another dirty event.
            buffer1.update(&mut cx, |buffer, cx| {
                assert!(buffer.text() == "aBDc");
                assert!(buffer.is_dirty());
                assert_eq!(
                    *events.borrow(),
                    &[Event::Edited, Event::Dirtied, Event::Edited],
                );
                events.borrow_mut().clear();

                // TODO - currently, after restoring the buffer to its
                // previously-saved state, the is still considered dirty.
                buffer.edit(vec![1..3], "", Some(cx)).unwrap();
                assert!(buffer.text() == "ac");
                assert!(buffer.is_dirty());
            });

            assert_eq!(*events.borrow(), &[Event::Edited]);

            // When a file is deleted, the buffer is considered dirty.
            let events = Rc::new(RefCell::new(Vec::new()));
            let file2 = cx.update(|cx| tree.file("file2", cx)).await;
            let buffer2 = cx.add_model(|cx: &mut ModelContext<Buffer>| {
                cx.subscribe(&cx.handle(), {
                    let events = events.clone();
                    move |_, event, _| events.borrow_mut().push(event.clone())
                });

                Buffer::from_history(0, History::new("abc".into()), Some(file2), None, cx)
            });

            fs::remove_file(dir.path().join("file2")).unwrap();
            buffer2.condition(&cx, |b, _| b.is_dirty()).await;
            assert_eq!(
                *events.borrow(),
                &[Event::Dirtied, Event::FileHandleChanged]
            );

            // When a file is already dirty when deleted, we don't emit a Dirtied event.
            let events = Rc::new(RefCell::new(Vec::new()));
            let file3 = cx.update(|cx| tree.file("file3", cx)).await;
            let buffer3 = cx.add_model(|cx: &mut ModelContext<Buffer>| {
                cx.subscribe(&cx.handle(), {
                    let events = events.clone();
                    move |_, event, _| events.borrow_mut().push(event.clone())
                });

                Buffer::from_history(0, History::new("abc".into()), Some(file3), None, cx)
            });

            tree.flush_fs_events(&cx).await;
            buffer3.update(&mut cx, |buffer, cx| {
                buffer.edit(Some(0..0), "x", Some(cx)).unwrap();
            });
            events.borrow_mut().clear();
            fs::remove_file(dir.path().join("file3")).unwrap();
            buffer3
                .condition(&cx, |_, _| !events.borrow().is_empty())
                .await;
            assert_eq!(*events.borrow(), &[Event::FileHandleChanged]);
            cx.read(|cx| assert!(buffer3.read(cx).is_dirty()));
        });
    }

    #[gpui::test]
    async fn test_file_changes_on_disk(mut cx: gpui::TestAppContext) {
        let initial_contents = "aaa\nbbbbb\nc\n";
        let dir = temp_tree(json!({ "the-file": initial_contents }));
        let tree = cx.add_model(|cx| Worktree::new(dir.path(), cx));
        cx.read(|cx| tree.read(cx).scan_complete()).await;

        let abs_path = dir.path().join("the-file");
        let file = cx.update(|cx| tree.file("the-file", cx)).await;
        let buffer = cx.add_model(|cx| {
            Buffer::from_history(
                0,
                History::new(initial_contents.into()),
                Some(file),
                None,
                cx,
            )
        });

        // Add a cursor at the start of each row.
        let (selection_set_id, _) = buffer.update(&mut cx, |buffer, cx| {
            assert!(!buffer.is_dirty());
            buffer.add_selection_set(
                (0..3)
                    .map(|row| {
                        let anchor = buffer.anchor_at(Point::new(row, 0), AnchorBias::Right);
                        Selection {
                            id: row as usize,
                            start: anchor.clone(),
                            end: anchor,
                            reversed: false,
                            goal: SelectionGoal::None,
                        }
                    })
                    .collect::<Vec<_>>(),
                Some(cx),
            )
        });

        // Change the file on disk, adding two new lines of text, and removing
        // one line.
        buffer.read_with(&cx, |buffer, _| {
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());
        });
        let new_contents = "AAAA\naaa\nBB\nbbbbb\n";
        fs::write(&abs_path, new_contents).unwrap();

        // Because the buffer was not modified, it is reloaded from disk. Its
        // contents are edited according to the diff between the old and new
        // file contents.
        buffer
            .condition(&cx, |buffer, _| buffer.text() != initial_contents)
            .await;

        buffer.update(&mut cx, |buffer, _| {
            assert_eq!(buffer.text(), new_contents);
            assert!(!buffer.is_dirty());
            assert!(!buffer.has_conflict());

            let selections = buffer.selections(selection_set_id).unwrap();
            let cursor_positions = selections
                .iter()
                .map(|selection| {
                    assert_eq!(selection.start, selection.end);
                    selection.start.to_point(&buffer)
                })
                .collect::<Vec<_>>();
            assert_eq!(
                cursor_positions,
                &[Point::new(1, 0), Point::new(3, 0), Point::new(4, 0),]
            );
        });

        // Modify the buffer
        buffer.update(&mut cx, |buffer, cx| {
            buffer.edit(vec![0..0], " ", Some(cx)).unwrap();
            assert!(buffer.is_dirty());
        });

        // Change the file on disk again, adding blank lines to the beginning.
        fs::write(&abs_path, "\n\n\nAAAA\naaa\nBB\nbbbbb\n").unwrap();

        // Becaues the buffer is modified, it doesn't reload from disk, but is
        // marked as having a conflict.
        buffer
            .condition(&cx, |buffer, _| buffer.has_conflict())
            .await;
    }

    #[gpui::test]
    async fn test_set_text_via_diff(mut cx: gpui::TestAppContext) {
        let text = "a\nbb\nccc\ndddd\neeeee\nffffff\n";
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));

        let text = "a\nccc\ndddd\nffffff\n";
        let diff = buffer.read_with(&cx, |b, cx| b.diff(text.into(), cx)).await;
        buffer.update(&mut cx, |b, cx| b.set_text_via_diff(diff, cx));
        cx.read(|cx| assert_eq!(buffer.read(cx).text(), text));

        let text = "a\n1\n\nccc\ndd2dd\nffffff\n";
        let diff = buffer.read_with(&cx, |b, cx| b.diff(text.into(), cx)).await;
        buffer.update(&mut cx, |b, cx| b.set_text_via_diff(diff, cx));
        cx.read(|cx| assert_eq!(buffer.read(cx).text(), text));
    }

    #[gpui::test]
    fn test_undo_redo(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "1234", cx);

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
    fn test_history(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut now = Instant::now();
            let mut buffer = Buffer::new(0, "123456", cx);

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
    fn test_random_concurrent_edits(cx: &mut gpui::MutableAppContext) {
        use crate::test::Network;

        let peers = env::var("PEERS")
            .map(|i| i.parse().expect("invalid `PEERS` variable"))
            .unwrap_or(5);
        let iterations = env::var("ITERATIONS")
            .map(|i| i.parse().expect("invalid `ITERATIONS` variable"))
            .unwrap_or(100);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
        let seed_range = if let Ok(seed) = env::var("SEED") {
            let seed = seed.parse().expect("invalid `SEED` variable");
            seed..seed + 1
        } else {
            0..iterations
        };

        for seed in seed_range {
            dbg!(seed);
            let mut rng = &mut StdRng::seed_from_u64(seed);

            let base_text_len = rng.gen_range(0..10);
            let base_text = RandomCharIter::new(&mut rng)
                .take(base_text_len)
                .collect::<String>();
            let mut replica_ids = Vec::new();
            let mut buffers = Vec::new();
            let mut network = Network::new();
            for i in 0..peers {
                let buffer = cx.add_model(|cx| Buffer::new(i as ReplicaId, base_text.as_str(), cx));
                buffers.push(buffer);
                replica_ids.push(i as u16);
                network.add_peer(i as u16);
            }

            let mut mutation_count = operations;
            loop {
                let replica_index = rng.gen_range(0..peers);
                let replica_id = replica_ids[replica_index];
                buffers[replica_index].update(cx, |buffer, _| match rng.gen_range(0..=100) {
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

            let first_buffer = buffers[0].read(cx);
            for buffer in &buffers[1..] {
                let buffer = buffer.read(cx);
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

    #[gpui::test]
    async fn test_reparse(mut cx: gpui::TestAppContext) {
        let app_state = cx.read(build_app_state);
        let rust_lang = app_state.language_registry.select_language("test.rs");
        assert!(rust_lang.is_some());

        let buffer = cx.add_model(|cx| {
            let text = "fn a() {}".into();
            let buffer = Buffer::from_history(0, History::new(text), None, rust_lang.cloned(), cx);
            assert!(buffer.is_parsing());
            assert!(buffer.syntax_tree().is_none());
            buffer
        });

        // Wait for the initial text to parse
        buffer
            .condition(&cx, |buffer, _| !buffer.is_parsing())
            .await;
        assert_eq!(
            get_tree_sexp(&buffer, &cx),
            concat!(
                "(source_file (function_item name: (identifier) ",
                "parameters: (parameters) ",
                "body: (block)))"
            )
        );

        // Perform some edits (add parameter and variable reference)
        // Parsing doesn't begin until the transaction is complete
        buffer.update(&mut cx, |buf, cx| {
            buf.start_transaction(None).unwrap();

            let offset = buf.text().find(")").unwrap();
            buf.edit(vec![offset..offset], "b: C", Some(cx)).unwrap();
            assert!(!buf.is_parsing());

            let offset = buf.text().find("}").unwrap();
            buf.edit(vec![offset..offset], " d; ", Some(cx)).unwrap();
            assert!(!buf.is_parsing());

            buf.end_transaction(None, Some(cx)).unwrap();
            assert_eq!(buf.text(), "fn a(b: C) { d; }");
            assert!(buf.is_parsing());
        });
        buffer
            .condition(&cx, |buffer, _| !buffer.is_parsing())
            .await;
        assert_eq!(
            get_tree_sexp(&buffer, &cx),
            concat!(
                "(source_file (function_item name: (identifier) ",
                    "parameters: (parameters (parameter pattern: (identifier) type: (type_identifier))) ",
                    "body: (block (identifier))))"
            )
        );

        // Perform a series of edits without waiting for the current parse to complete:
        // * turn identifier into a field expression
        // * turn field expression into a method call
        // * add a turbofish to the method call
        buffer.update(&mut cx, |buf, cx| {
            let offset = buf.text().find(";").unwrap();
            buf.edit(vec![offset..offset], ".e", Some(cx)).unwrap();
            assert_eq!(buf.text(), "fn a(b: C) { d.e; }");
            assert!(buf.is_parsing());
        });
        buffer.update(&mut cx, |buf, cx| {
            let offset = buf.text().find(";").unwrap();
            buf.edit(vec![offset..offset], "(f)", Some(cx)).unwrap();
            assert_eq!(buf.text(), "fn a(b: C) { d.e(f); }");
            assert!(buf.is_parsing());
        });
        buffer.update(&mut cx, |buf, cx| {
            let offset = buf.text().find("(f)").unwrap();
            buf.edit(vec![offset..offset], "::<G>", Some(cx)).unwrap();
            assert_eq!(buf.text(), "fn a(b: C) { d.e::<G>(f); }");
            assert!(buf.is_parsing());
        });
        buffer
            .condition(&cx, |buffer, _| !buffer.is_parsing())
            .await;
        assert_eq!(
            get_tree_sexp(&buffer, &cx),
            concat!(
                "(source_file (function_item name: (identifier) ",
                    "parameters: (parameters (parameter pattern: (identifier) type: (type_identifier))) ",
                    "body: (block (call_expression ",
                        "function: (generic_function ",
                            "function: (field_expression value: (identifier) field: (field_identifier)) ",
                            "type_arguments: (type_arguments (type_identifier))) ",
                            "arguments: (arguments (identifier))))))",
            )
        );

        buffer.update(&mut cx, |buf, cx| {
            buf.undo(Some(cx));
            assert_eq!(buf.text(), "fn a() {}");
            assert!(buf.is_parsing());
        });
        buffer
            .condition(&cx, |buffer, _| !buffer.is_parsing())
            .await;
        assert_eq!(
            get_tree_sexp(&buffer, &cx),
            concat!(
                "(source_file (function_item name: (identifier) ",
                "parameters: (parameters) ",
                "body: (block)))"
            )
        );

        buffer.update(&mut cx, |buf, cx| {
            buf.redo(Some(cx));
            assert_eq!(buf.text(), "fn a(b: C) { d.e::<G>(f); }");
            assert!(buf.is_parsing());
        });
        buffer
            .condition(&cx, |buffer, _| !buffer.is_parsing())
            .await;
        assert_eq!(
            get_tree_sexp(&buffer, &cx),
            concat!(
                "(source_file (function_item name: (identifier) ",
                    "parameters: (parameters (parameter pattern: (identifier) type: (type_identifier))) ",
                    "body: (block (call_expression ",
                        "function: (generic_function ",
                            "function: (field_expression value: (identifier) field: (field_identifier)) ",
                            "type_arguments: (type_arguments (type_identifier))) ",
                            "arguments: (arguments (identifier))))))",
            )
        );

        fn get_tree_sexp(buffer: &ModelHandle<Buffer>, cx: &gpui::TestAppContext) -> String {
            buffer.read_with(cx, |buffer, _| {
                buffer.syntax_tree().unwrap().root_node().to_sexp()
            })
        }
    }

    #[gpui::test]
    async fn test_enclosing_bracket_ranges(mut cx: gpui::TestAppContext) {
        use unindent::Unindent as _;

        let app_state = cx.read(build_app_state);
        let rust_lang = app_state.language_registry.select_language("test.rs");
        assert!(rust_lang.is_some());

        let buffer = cx.add_model(|cx| {
            let text = "
                mod x {
                    mod y {

                    }
                }
            "
            .unindent()
            .into();
            Buffer::from_history(0, History::new(text), None, rust_lang.cloned(), cx)
        });
        buffer
            .condition(&cx, |buffer, _| !buffer.is_parsing())
            .await;
        buffer.read_with(&cx, |buf, _| {
            assert_eq!(
                buf.enclosing_bracket_point_ranges(Point::new(1, 6)..Point::new(1, 6)),
                Some((
                    Point::new(0, 6)..Point::new(0, 7),
                    Point::new(4, 0)..Point::new(4, 1)
                ))
            );
            assert_eq!(
                buf.enclosing_bracket_point_ranges(Point::new(1, 10)..Point::new(1, 10)),
                Some((
                    Point::new(1, 10)..Point::new(1, 11),
                    Point::new(3, 4)..Point::new(3, 5)
                ))
            );
            assert_eq!(
                buf.enclosing_bracket_point_ranges(Point::new(3, 5)..Point::new(3, 5)),
                Some((
                    Point::new(1, 10)..Point::new(1, 11),
                    Point::new(3, 4)..Point::new(3, 5)
                ))
            );
        });
    }

    impl Buffer {
        fn random_byte_range(&mut self, start_offset: usize, rng: &mut impl Rng) -> Range<usize> {
            let end = self.clip_offset(rng.gen_range(start_offset..=self.len()), Bias::Right);
            let start = self.clip_offset(rng.gen_range(start_offset..=end), Bias::Left);
            start..end
        }

        pub fn randomly_edit<T>(
            &mut self,
            rng: &mut T,
            old_range_count: usize,
            cx: Option<&mut ModelContext<Self>>,
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
                old_ranges.push(self.random_byte_range(last_end, rng));
            }
            let new_text_len = rng.gen_range(0..10);
            let new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();

            let operations = self
                .edit(old_ranges.iter().cloned(), new_text.as_str(), cx)
                .unwrap();

            (old_ranges, new_text, operations)
        }

        pub fn randomly_mutate<T>(
            &mut self,
            rng: &mut T,
            mut cx: Option<&mut ModelContext<Self>>,
        ) -> (Vec<Range<usize>>, String, Vec<Operation>)
        where
            T: Rng,
        {
            // Randomly edit
            let (old_ranges, new_text, mut operations) =
                self.randomly_edit(rng, 5, cx.as_deref_mut());
            log::info!("Mutating buffer at {:?}: {:?}", old_ranges, new_text);

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
                    ranges.push(self.random_byte_range(0, rng));
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
                        start: self.anchor_before(range.end),
                        end: self.anchor_before(range.start),
                        reversed: true,
                        goal: SelectionGoal::None,
                    });
                } else {
                    selections.push(Selection {
                        id: NEXT_SELECTION_ID.fetch_add(1, atomic::Ordering::SeqCst),
                        start: self.anchor_after(range.start),
                        end: self.anchor_before(range.end),
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
                    let start = selection.start.to_offset(self);
                    let end = selection.end.to_offset(self);
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

        pub fn enclosing_bracket_point_ranges<T: ToOffset>(
            &self,
            range: Range<T>,
        ) -> Option<(Range<Point>, Range<Point>)> {
            self.enclosing_bracket_ranges(range).map(|(start, end)| {
                let point_start = start.start.to_point(self)..start.end.to_point(self);
                let point_end = end.start.to_point(self)..end.end.to_point(self);
                (point_start, point_end)
            })
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
}
