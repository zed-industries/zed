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
use zed_rpc::proto;

use crate::{
    language::{Language, Tree},
    operation_queue::{self, OperationQueue},
    settings::{StyleId, ThemeMap},
    sum_tree::{self, FilterCursor, SumTree},
    time::{self, ReplicaId},
    util::Bias,
    worktree::{File, Worktree},
};
use anyhow::{anyhow, Result};
use gpui::{AppContext, Entity, ModelContext, ModelHandle, Task};
use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    cmp,
    convert::{TryFrom, TryInto},
    hash::BuildHasher,
    iter::Iterator,
    ops::{Deref, DerefMut, Range},
    path::Path,
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
    pub version: time::Global,
    saved_version: time::Global,
    saved_mtime: SystemTime,
    last_edit: time::Local,
    undo_map: UndoMap,
    history: History,
    file: Option<File>,
    language: Option<Arc<Language>>,
    syntax_tree: Mutex<Option<SyntaxTree>>,
    is_parsing: bool,
    selections: HashMap<SelectionSetId, SelectionSet>,
    deferred_ops: OperationQueue<Operation>,
    deferred_replicas: HashSet<ReplicaId>,
    replica_id: ReplicaId,
    remote_id: u64,
    local_clock: time::Local,
    lamport_clock: time::Lamport,
    #[cfg(test)]
    operations: Vec<Operation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SelectionSet {
    selections: Arc<[Selection]>,
    active: bool,
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
        self.ops.insert(op.timestamp.local(), op);
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

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct InsertionTimestamp {
    replica_id: ReplicaId,
    local: time::Seq,
    lamport: time::Seq,
}

impl InsertionTimestamp {
    fn local(&self) -> time::Local {
        time::Local {
            replica_id: self.replica_id,
            value: self.local,
        }
    }

    fn lamport(&self) -> time::Lamport {
        time::Lamport {
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
    deletions: HashSet<time::Local>,
    max_undos: time::Global,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FragmentSummary {
    text: FragmentTextSummary,
    max_version: time::Global,
    min_insertion_version: time::Global,
    max_insertion_version: time::Global,
}

#[derive(Copy, Default, Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Edit(EditOperation),
    Undo {
        undo: UndoOperation,
        lamport_timestamp: time::Lamport,
    },
    UpdateSelections {
        set_id: SelectionSetId,
        selections: Option<Arc<[Selection]>>,
        lamport_timestamp: time::Lamport,
    },
    SetActiveSelections {
        set_id: Option<SelectionSetId>,
        lamport_timestamp: time::Lamport,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditOperation {
    timestamp: InsertionTimestamp,
    version: time::Global,
    ranges: Vec<Range<usize>>,
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
        Self::build(
            replica_id,
            History::new(base_text.into()),
            None,
            cx.model_id() as u64,
            None,
            cx,
        )
    }

    pub fn from_history(
        replica_id: ReplicaId,
        history: History,
        file: Option<File>,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(
            replica_id,
            history,
            file,
            cx.model_id() as u64,
            language,
            cx,
        )
    }

    fn build(
        replica_id: ReplicaId,
        history: History,
        file: Option<File>,
        remote_id: u64,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let saved_mtime;
        if let Some(file) = file.as_ref() {
            saved_mtime = file.mtime(cx.as_ref());
        } else {
            saved_mtime = UNIX_EPOCH;
        }

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

        let mut result = Self {
            visible_text,
            deleted_text: Rope::new(),
            fragments,
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
            deferred_ops: OperationQueue::new(),
            deferred_replicas: HashSet::default(),
            replica_id,
            remote_id,
            local_clock: time::Local::new(replica_id),
            lamport_clock: time::Lamport::new(replica_id),

            #[cfg(test)]
            operations: Default::default(),
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

    pub fn from_proto(
        replica_id: ReplicaId,
        message: proto::Buffer,
        file: Option<File>,
        language: Option<Arc<Language>>,
        cx: &mut ModelContext<Self>,
    ) -> Result<Self> {
        let mut buffer = Buffer::build(
            replica_id,
            History::new(message.content.into()),
            file,
            message.id,
            language,
            cx,
        );
        let ops = message
            .history
            .into_iter()
            .map(|op| Operation::Edit(op.into()));
        buffer.apply_ops(ops, cx)?;
        Ok(buffer)
    }

    pub fn to_proto(&self, cx: &mut ModelContext<Self>) -> proto::Buffer {
        let ops = self.history.ops.values().map(Into::into).collect();
        proto::Buffer {
            id: cx.model_id() as u64,
            content: self.history.base_text.to_string(),
            history: ops,
        }
    }

    pub fn file(&self) -> Option<&File> {
        self.file.as_ref()
    }

    pub fn save(&mut self, cx: &mut ModelContext<Self>) -> Result<Task<Result<()>>> {
        let file = self
            .file
            .as_ref()
            .ok_or_else(|| anyhow!("buffer has no file"))?;

        let text = self.visible_text.clone();
        let version = self.version.clone();
        let save = file.save(text, cx.as_mut());

        Ok(cx.spawn(|this, mut cx| async move {
            save.await?;
            this.update(&mut cx, |this, cx| {
                this.did_save(version, cx).unwrap();
            });
            Ok(())
        }))
    }

    pub fn save_as(
        &mut self,
        worktree: &ModelHandle<Worktree>,
        path: impl Into<Arc<Path>>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let handle = cx.handle();
        let text = self.visible_text.clone();
        let version = self.version.clone();
        let save_as = worktree.update(cx, |worktree, cx| {
            worktree
                .as_local_mut()
                .unwrap()
                .save_buffer_as(handle, path, text, cx)
        });

        cx.spawn(|this, mut cx| async move {
            save_as.await.map(|new_file| {
                this.update(&mut cx, |this, cx| {
                    this.file = Some(new_file);
                    this.did_save(version, cx).unwrap();
                });
            })
        })
    }

    fn did_save(&mut self, version: time::Global, cx: &mut ModelContext<Self>) -> Result<()> {
        if let Some(file) = self.file.as_ref() {
            self.saved_mtime = file.mtime(cx.as_ref());
            self.saved_version = version;
            cx.emit(Event::Saved);
            Ok(())
        } else {
            Err(anyhow!("buffer has no file"))
        }
    }

    pub fn file_was_moved(&mut self, new_path: Arc<Path>, cx: &mut ModelContext<Self>) {
        self.file.as_mut().unwrap().path = new_path;
        cx.emit(Event::FileHandleChanged);
    }

    pub fn file_was_added(&mut self, cx: &mut ModelContext<Self>) {
        cx.emit(Event::FileHandleChanged);
    }

    pub fn file_was_deleted(&mut self, cx: &mut ModelContext<Self>) {
        if self.version == self.saved_version {
            cx.emit(Event::Dirtied);
        }
        cx.emit(Event::FileHandleChanged);
    }

    pub fn file_was_modified(
        &mut self,
        new_text: String,
        mtime: SystemTime,
        cx: &mut ModelContext<Self>,
    ) {
        if self.version == self.saved_version {
            cx.spawn(|this, mut cx| async move {
                let diff = this
                    .read_with(&cx, |this, cx| this.diff(new_text.into(), cx))
                    .await;
                this.update(&mut cx, |this, cx| {
                    if this.set_text_via_diff(diff, cx) {
                        this.saved_version = this.version.clone();
                        this.saved_mtime = mtime;
                        cx.emit(Event::Reloaded);
                    }
                });
            })
            .detach();
        }
        cx.emit(Event::FileHandleChanged);
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
                        .background()
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
        cx.background().spawn(async move {
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

    fn set_text_via_diff(&mut self, diff: Diff, cx: &mut ModelContext<Self>) -> bool {
        if self.version == diff.base_version {
            self.start_transaction(None, cx).unwrap();
            let mut offset = 0;
            for (tag, len) in diff.changes {
                let range = offset..(offset + len);
                match tag {
                    ChangeTag::Equal => offset += len,
                    ChangeTag::Delete => self.edit(Some(range), "", cx),
                    ChangeTag::Insert => {
                        self.edit(Some(offset..offset), &diff.new_text[range], cx);
                        offset += len;
                    }
                }
            }
            self.end_transaction(None, cx).unwrap();
            true
        } else {
            false
        }
    }

    pub fn is_dirty(&self, cx: &AppContext) -> bool {
        self.version > self.saved_version
            || self.file.as_ref().map_or(false, |file| file.is_deleted(cx))
    }

    pub fn has_conflict(&self, cx: &AppContext) -> bool {
        self.version > self.saved_version
            && self
                .file
                .as_ref()
                .map_or(false, |file| file.mtime(cx) > self.saved_mtime)
    }

    pub fn remote_id(&self) -> u64 {
        self.remote_id
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

    pub fn start_transaction(
        &mut self,
        set_id: Option<SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.start_transaction_at(set_id, Instant::now(), cx)
    }

    fn start_transaction_at(
        &mut self,
        set_id: Option<SelectionSetId>,
        now: Instant,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let selections = if let Some(set_id) = set_id {
            let set = self
                .selections
                .get(&set_id)
                .ok_or_else(|| anyhow!("invalid selection set {:?}", set_id))?;
            Some((set_id, set.selections.clone()))
        } else {
            None
        };
        self.history.start_transaction(
            self.version.clone(),
            self.is_dirty(cx.as_ref()),
            selections,
            now,
        );
        Ok(())
    }

    pub fn end_transaction(
        &mut self,
        set_id: Option<SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.end_transaction_at(set_id, Instant::now(), cx)
    }

    fn end_transaction_at(
        &mut self,
        set_id: Option<SelectionSetId>,
        now: Instant,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let selections = if let Some(set_id) = set_id {
            let set = self
                .selections
                .get(&set_id)
                .ok_or_else(|| anyhow!("invalid selection set {:?}", set_id))?;
            Some((set_id, set.selections.clone()))
        } else {
            None
        };

        if let Some(transaction) = self.history.end_transaction(selections, now) {
            let since = transaction.start.clone();
            let was_dirty = transaction.buffer_was_dirty;
            self.history.group();

            cx.notify();
            if self.edits_since(since).next().is_some() {
                self.did_edit(was_dirty, cx);
                self.reparse(cx);
            }
        }

        Ok(())
    }

    pub fn edit<I, S, T>(&mut self, ranges_iter: I, new_text: T, cx: &mut ModelContext<Self>)
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        let new_text = new_text.into();
        let new_text = if new_text.len() > 0 {
            Some(new_text)
        } else {
            None
        };
        let has_new_text = new_text.is_some();

        // Skip invalid ranges and coalesce contiguous ones.
        let mut ranges: Vec<Range<usize>> = Vec::new();
        for range in ranges_iter {
            let range = range.start.to_offset(self)..range.end.to_offset(self);
            if has_new_text || !range.is_empty() {
                if let Some(prev_range) = ranges.last_mut() {
                    if prev_range.end >= range.start {
                        prev_range.end = cmp::max(prev_range.end, range.end);
                    } else {
                        ranges.push(range);
                    }
                } else {
                    ranges.push(range);
                }
            }
        }

        if !ranges.is_empty() {
            self.start_transaction_at(None, Instant::now(), cx).unwrap();
            let timestamp = InsertionTimestamp {
                replica_id: self.replica_id,
                local: self.local_clock.tick().value,
                lamport: self.lamport_clock.tick().value,
            };
            let edit = self.apply_local_edit(&ranges, new_text, timestamp);

            self.history.push(edit.clone());
            self.history.push_undo(edit.timestamp.local());
            self.last_edit = edit.timestamp.local();
            self.version.observe(edit.timestamp.local());

            self.end_transaction_at(None, Instant::now(), cx).unwrap();
            self.send_operation(Operation::Edit(edit), cx);
        };
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
        cx: &mut ModelContext<Self>,
    ) -> SelectionSetId {
        let selections = selections.into();
        let lamport_timestamp = self.lamport_clock.tick();
        self.selections.insert(
            lamport_timestamp,
            SelectionSet {
                selections: selections.clone(),
                active: false,
            },
        );
        cx.notify();

        self.send_operation(
            Operation::UpdateSelections {
                set_id: lamport_timestamp,
                selections: Some(selections),
                lamport_timestamp,
            },
            cx,
        );

        lamport_timestamp
    }

    pub fn update_selection_set(
        &mut self,
        set_id: SelectionSetId,
        selections: impl Into<Arc<[Selection]>>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let selections = selections.into();
        let set = self
            .selections
            .get_mut(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        set.selections = selections.clone();
        let lamport_timestamp = self.lamport_clock.tick();
        cx.notify();
        self.send_operation(
            Operation::UpdateSelections {
                set_id,
                selections: Some(selections),
                lamport_timestamp,
            },
            cx,
        );
        Ok(())
    }

    pub fn set_active_selection_set(
        &mut self,
        set_id: Option<SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        if set_id.is_some() && !self.selections.contains_key(set_id.as_ref().unwrap()) {
            return Err(anyhow!("invalid selection set id {:?}", set_id));
        }

        let lamport_timestamp = self.lamport_clock.tick();
        self.send_operation(
            Operation::SetActiveSelections {
                set_id,
                lamport_timestamp,
            },
            cx,
        );
        Ok(())
    }

    pub fn remove_selection_set(
        &mut self,
        set_id: SelectionSetId,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.selections
            .remove(&set_id)
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))?;
        let lamport_timestamp = self.lamport_clock.tick();
        cx.notify();
        self.send_operation(
            Operation::UpdateSelections {
                set_id,
                selections: None,
                lamport_timestamp,
            },
            cx,
        );
        Ok(())
    }

    pub fn selections(&self, set_id: SelectionSetId) -> Result<&[Selection]> {
        self.selections
            .get(&set_id)
            .map(|s| s.selections.as_ref())
            .ok_or_else(|| anyhow!("invalid selection set id {:?}", set_id))
    }

    pub fn apply_ops<I: IntoIterator<Item = Operation>>(
        &mut self,
        ops: I,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let was_dirty = self.is_dirty(cx.as_ref());
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

        cx.notify();
        if self.edits_since(old_version).next().is_some() {
            self.did_edit(was_dirty, cx);
            self.reparse(cx);
        }

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
                    if let Some(set) = self.selections.get_mut(&set_id) {
                        set.selections = selections;
                    } else {
                        self.selections.insert(
                            set_id,
                            SelectionSet {
                                selections,
                                active: false,
                            },
                        );
                    }
                } else {
                    self.selections.remove(&set_id);
                }
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
        }
        Ok(())
    }

    fn apply_remote_edit(
        &mut self,
        version: &time::Global,
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
        let mut old_fragments = self.fragments.cursor::<VersionedOffset, VersionedOffset>();
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

    #[cfg(not(test))]
    pub fn send_operation(&mut self, operation: Operation, cx: &mut ModelContext<Self>) {
        if let Some(file) = &self.file {
            file.buffer_updated(self.remote_id, operation, cx.as_mut());
        }
    }

    #[cfg(test)]
    pub fn send_operation(&mut self, operation: Operation, _: &mut ModelContext<Self>) {
        self.operations.push(operation);
    }

    pub fn peer_left(&mut self, replica_id: ReplicaId, cx: &mut ModelContext<Self>) {
        self.selections
            .retain(|set_id, _| set_id.replica_id != replica_id);
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) {
        let was_dirty = self.is_dirty(cx.as_ref());
        let old_version = self.version.clone();

        if let Some(transaction) = self.history.pop_undo() {
            let selections = transaction.selections_before.clone();
            for edit_id in transaction.edits.clone() {
                self.undo_or_redo(edit_id, cx).unwrap();
            }

            if let Some((set_id, selections)) = selections {
                let _ = self.update_selection_set(set_id, selections, cx);
            }
        }

        cx.notify();
        if self.edits_since(old_version).next().is_some() {
            self.did_edit(was_dirty, cx);
            self.reparse(cx);
        }
    }

    pub fn redo(&mut self, cx: &mut ModelContext<Self>) {
        let was_dirty = self.is_dirty(cx.as_ref());
        let old_version = self.version.clone();

        if let Some(transaction) = self.history.pop_redo() {
            let selections = transaction.selections_after.clone();
            for edit_id in transaction.edits.clone() {
                self.undo_or_redo(edit_id, cx).unwrap();
            }

            if let Some((set_id, selections)) = selections {
                let _ = self.update_selection_set(set_id, selections, cx);
            }
        }

        cx.notify();
        if self.edits_since(old_version).next().is_some() {
            self.did_edit(was_dirty, cx);
            self.reparse(cx);
        }
    }

    fn undo_or_redo(&mut self, edit_id: time::Local, cx: &mut ModelContext<Self>) -> Result<()> {
        let undo = UndoOperation {
            id: self.local_clock.tick(),
            edit_id,
            count: self.undo_map.undo_count(edit_id) + 1,
        };
        self.apply_undo(undo)?;
        self.version.observe(undo.id);

        let operation = Operation::Undo {
            undo,
            lamport_timestamp: self.lamport_clock.tick(),
        };
        self.send_operation(operation, cx);

        Ok(())
    }

    fn apply_undo(&mut self, undo: UndoOperation) -> Result<()> {
        self.undo_map.insert(undo);
        let edit = &self.history.ops[&undo.edit_id];
        let version = Some(edit.version.clone());

        let mut old_fragments = self.fragments.cursor::<VersionedOffset, VersionedOffset>();
        old_fragments.seek(&VersionedOffset::Offset(0), Bias::Left, &version);

        let mut new_fragments = SumTree::new();
        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));

        for range in &edit.ranges {
            let mut end_offset = old_fragments.end(&version).offset();

            if end_offset < range.start {
                let preceding_fragments = old_fragments.slice(
                    &VersionedOffset::Offset(range.start),
                    Bias::Left,
                    &version,
                );
                new_ropes.push_tree(preceding_fragments.summary().text);
                new_fragments.push_tree(preceding_fragments, &None);
            }

            while end_offset <= range.end {
                if let Some(fragment) = old_fragments.item() {
                    let mut fragment = fragment.clone();
                    let fragment_was_visible = fragment.visible;
                    if fragment.was_visible(&edit.version, &self.undo_map)
                        || fragment.timestamp.local() == edit.timestamp.local()
                    {
                        fragment.visible = fragment.is_visible(&self.undo_map);
                        fragment.max_undos.observe(undo.id);
                    }
                    new_ropes.push_fragment(&fragment, fragment_was_visible);
                    new_fragments.push(fragment, &None);

                    old_fragments.next(&version);
                    end_offset = old_fragments.end(&version).offset();
                } else {
                    break;
                }
            }
        }

        let suffix = old_fragments.suffix(&version);
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
                Operation::Undo { undo, .. } => self.version.observed(undo.edit_id),
                Operation::UpdateSelections { selections, .. } => {
                    if let Some(selections) = selections {
                        selections.iter().all(|selection| {
                            let contains_start = self.version >= selection.start.version;
                            let contains_end = self.version >= selection.end.version;
                            contains_start && contains_end
                        })
                    } else {
                        true
                    }
                }
                Operation::SetActiveSelections { set_id, .. } => {
                    set_id.map_or(true, |set_id| self.selections.contains_key(&set_id))
                }
            }
        }
    }

    fn apply_local_edit(
        &mut self,
        ranges: &[Range<usize>],
        new_text: Option<String>,
        timestamp: InsertionTimestamp,
    ) -> EditOperation {
        let mut edit = EditOperation {
            timestamp,
            version: self.version(),
            ranges: Vec::with_capacity(ranges.len()),
            new_text: None,
        };

        let mut new_ropes =
            RopeBuilder::new(self.visible_text.cursor(0), self.deleted_text.cursor(0));
        let mut old_fragments = self.fragments.cursor::<usize, FragmentTextSummary>();
        let mut new_fragments = old_fragments.slice(&ranges[0].start, Bias::Right, &None);
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

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        let offset = position.to_offset(self);
        let max_offset = self.len();
        assert!(offset <= max_offset, "offset is out of range");
        let mut cursor = self.fragments.cursor::<usize, FragmentTextSummary>();
        cursor.seek(&offset, bias, &None);
        Anchor {
            offset: offset + cursor.start().deleted,
            bias,
            version: self.version(),
        }
    }

    fn summary_for_anchor(&self, anchor: &Anchor) -> TextSummary {
        let cx = Some(anchor.version.clone());
        let mut cursor = self
            .fragments
            .cursor::<VersionedOffset, (VersionedOffset, usize)>();
        cursor.seek(&VersionedOffset::Offset(anchor.offset), anchor.bias, &cx);
        let overshoot = if cursor.item().map_or(false, |fragment| fragment.visible) {
            anchor.offset - cursor.start().0.offset()
        } else {
            0
        };
        self.text_summary_for_range(0..cursor.start().1 + overshoot)
    }

    fn full_offset_for_anchor(&self, anchor: &Anchor) -> usize {
        let cx = Some(anchor.version.clone());
        let mut cursor = self
            .fragments
            .cursor::<VersionedOffset, (VersionedOffset, FragmentTextSummary)>();
        cursor.seek(&VersionedOffset::Offset(anchor.offset), anchor.bias, &cx);
        let overshoot = if cursor.item().is_some() {
            anchor.offset - cursor.start().0.offset()
        } else {
            0
        };
        let summary = cursor.start().1;
        summary.visible + summary.deleted + overshoot
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
            version: self.version.clone(),
            saved_version: self.saved_version.clone(),
            saved_mtime: self.saved_mtime,
            last_edit: self.last_edit.clone(),
            undo_map: self.undo_map.clone(),
            history: self.history.clone(),
            selections: self.selections.clone(),
            deferred_ops: self.deferred_ops.clone(),
            file: self.file.clone(),
            language: self.language.clone(),
            syntax_tree: Mutex::new(self.syntax_tree.lock().clone()),
            is_parsing: false,
            deferred_replicas: self.deferred_replicas.clone(),
            replica_id: self.replica_id,
            remote_id: self.remote_id.clone(),
            local_clock: self.local_clock.clone(),
            lamport_clock: self.lamport_clock.clone(),

            #[cfg(test)]
            operations: self.operations.clone(),
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

    fn release(&mut self, cx: &mut gpui::MutableAppContext) {
        if let Some(file) = self.file.as_ref() {
            file.buffer_removed(self.remote_id, cx);
        }
    }
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
                        change.new_range.end += fragment.len;
                        self.delta += fragment.len as isize;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_range: old_offset..old_offset,
                        new_range: new_offset..new_offset + fragment.len,
                        old_lines: Point::zero(),
                    });
                    self.delta += fragment.len as isize;
                }
            } else if fragment.was_visible(&self.since, &self.undos) && !fragment.visible {
                let deleted_start = self.cursor.start().deleted;
                let old_lines = self.deleted_text.to_point(deleted_start + fragment.len)
                    - self.deleted_text.to_point(deleted_start);
                if let Some(ref mut change) = change {
                    if change.new_range.end == new_offset {
                        change.old_range.end += fragment.len;
                        change.old_lines += &old_lines;
                        self.delta -= fragment.len as isize;
                    } else {
                        break;
                    }
                } else {
                    change = Some(Edit {
                        old_range: old_offset..old_offset + fragment.len,
                        new_range: new_offset..new_offset,
                        old_lines,
                    });
                    self.delta -= fragment.len as isize;
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

impl Fragment {
    fn is_visible(&self, undos: &UndoMap) -> bool {
        !undos.is_undone(self.timestamp.local())
            && self.deletions.iter().all(|d| undos.is_undone(*d))
    }

    fn was_visible(&self, version: &time::Global, undos: &UndoMap) -> bool {
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
        let mut max_version = time::Global::new();
        max_version.observe(self.timestamp.local());
        for deletion in &self.deletions {
            max_version.observe(*deletion);
        }
        max_version.join(&self.max_undos);

        let mut min_insertion_version = time::Global::new();
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
    type Context = Option<time::Global>;

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
            max_version: time::Global::new(),
            min_insertion_version: time::Global::new(),
            max_insertion_version: time::Global::new(),
        }
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for usize {
    fn add_summary(&mut self, summary: &FragmentSummary, _: &Option<time::Global>) {
        *self += summary.text.visible;
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

impl Operation {
    fn replica_id(&self) -> ReplicaId {
        self.lamport_timestamp().replica_id
    }

    fn lamport_timestamp(&self) -> time::Lamport {
        match self {
            Operation::Edit(edit) => edit.timestamp.lamport(),
            Operation::Undo {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::UpdateSelections {
                lamport_timestamp, ..
            } => *lamport_timestamp,
            Operation::SetActiveSelections {
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
                    edit_replica_id: undo.edit_id.replica_id as u32,
                    edit_local_timestamp: undo.edit_id.value,
                    count: undo.count,
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
                        selections: selections.as_ref().map_or(Vec::new(), |selections| {
                            selections
                                .iter()
                                .map(|selection| proto::Selection {
                                    id: selection.id as u64,
                                    start: Some((&selection.start).into()),
                                    end: Some((&selection.end).into()),
                                    reversed: selection.reversed,
                                })
                                .collect()
                        }),
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
            }),
        }
    }
}

impl<'a> Into<proto::operation::Edit> for &'a EditOperation {
    fn into(self) -> proto::operation::Edit {
        let version = self
            .version
            .iter()
            .map(|entry| proto::VectorClockEntry {
                replica_id: entry.replica_id as u32,
                timestamp: entry.value,
            })
            .collect();
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
            version,
            ranges,
            new_text: self.new_text.clone(),
        }
    }
}

impl<'a> Into<proto::Anchor> for &'a Anchor {
    fn into(self) -> proto::Anchor {
        proto::Anchor {
            version: self
                .version
                .iter()
                .map(|entry| proto::VectorClockEntry {
                    replica_id: entry.replica_id as u32,
                    timestamp: entry.value,
                })
                .collect(),
            offset: self.offset as u64,
            bias: match self.bias {
                Bias::Left => proto::anchor::Bias::Left as i32,
                Bias::Right => proto::anchor::Bias::Right as i32,
            },
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
                    lamport_timestamp: time::Lamport {
                        replica_id: undo.replica_id as ReplicaId,
                        value: undo.lamport_timestamp,
                    },
                    undo: UndoOperation {
                        id: time::Local {
                            replica_id: undo.replica_id as ReplicaId,
                            value: undo.local_timestamp,
                        },
                        edit_id: time::Local {
                            replica_id: undo.edit_replica_id as ReplicaId,
                            value: undo.edit_local_timestamp,
                        },
                        count: undo.count,
                    },
                },
                proto::operation::Variant::UpdateSelections(message) => {
                    Operation::UpdateSelections {
                        set_id: time::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value: message.local_timestamp,
                        },
                        lamport_timestamp: time::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value: message.lamport_timestamp,
                        },
                        selections: Some(
                            message
                                .selections
                                .into_iter()
                                .map(|selection| {
                                    Ok(Selection {
                                        id: selection.id as usize,
                                        start: selection
                                            .start
                                            .ok_or_else(|| anyhow!("missing selection start"))?
                                            .try_into()?,
                                        end: selection
                                            .end
                                            .ok_or_else(|| anyhow!("missing selection end"))?
                                            .try_into()?,
                                        reversed: selection.reversed,
                                        goal: SelectionGoal::None,
                                    })
                                })
                                .collect::<Result<Vec<_>, anyhow::Error>>()?
                                .into(),
                        ),
                    }
                }
                proto::operation::Variant::SetActiveSelections(message) => {
                    Operation::SetActiveSelections {
                        set_id: message.local_timestamp.map(|value| time::Lamport {
                            replica_id: message.replica_id as ReplicaId,
                            value,
                        }),
                        lamport_timestamp: time::Lamport {
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
        let mut version = time::Global::new();
        for entry in edit.version {
            version.observe(time::Local {
                replica_id: entry.replica_id as ReplicaId,
                value: entry.timestamp,
            });
        }
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
            version,
            ranges,
            new_text: edit.new_text,
        }
    }
}

impl TryFrom<proto::Anchor> for Anchor {
    type Error = anyhow::Error;

    fn try_from(message: proto::Anchor) -> Result<Self, Self::Error> {
        let mut version = time::Global::new();
        for entry in message.version {
            version.observe(time::Local {
                replica_id: entry.replica_id as ReplicaId,
                value: entry.timestamp,
            });
        }

        Ok(Self {
            offset: message.offset as usize,
            bias: if message.bias == proto::anchor::Bias::Left as i32 {
                Bias::Left
            } else if message.bias == proto::anchor::Bias::Right as i32 {
                Bias::Right
            } else {
                Err(anyhow!("invalid anchor bias {}", message.bias))?
            },
            version,
        })
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
        language::LanguageRegistry,
        test::{build_app_state, temp_tree},
        util::RandomCharIter,
        worktree::{Worktree, WorktreeHandle},
    };
    use gpui::ModelHandle;
    use rand::prelude::*;
    use serde_json::json;
    use std::{
        cell::RefCell,
        cmp::Ordering,
        env, fs, mem,
        path::Path,
        rc::Rc,
        sync::atomic::{self, AtomicUsize},
    };

    #[gpui::test]
    fn test_edit(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "abc", cx);
            assert_eq!(buffer.text(), "abc");
            buffer.edit(vec![3..3], "def", cx);
            assert_eq!(buffer.text(), "abcdef");
            buffer.edit(vec![0..0], "ghi", cx);
            assert_eq!(buffer.text(), "ghiabcdef");
            buffer.edit(vec![5..5], "jkl", cx);
            assert_eq!(buffer.text(), "ghiabjklcdef");
            buffer.edit(vec![6..7], "", cx);
            assert_eq!(buffer.text(), "ghiabjlcdef");
            buffer.edit(vec![4..9], "mno", cx);
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
        let buffer_ops = buffer1.update(cx, |buffer, cx| {
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
            buffer.edit(Some(2..4), "XYZ", cx);

            // An empty transaction does not emit any events.
            buffer.start_transaction(None, cx).unwrap();
            buffer.end_transaction(None, cx).unwrap();

            // A transaction containing two edits emits one edited event.
            now += Duration::from_secs(1);
            buffer.start_transaction_at(None, now, cx).unwrap();
            buffer.edit(Some(5..5), "u", cx);
            buffer.edit(Some(6..6), "w", cx);
            buffer.end_transaction_at(None, now, cx).unwrap();

            // Undoing a transaction emits one edited event.
            buffer.undo(cx);

            buffer.operations.clone()
        });

        // Incorporating a set of remote ops emits a single edited event,
        // followed by a dirtied event.
        buffer2.update(cx, |buffer, cx| {
            buffer.apply_ops(buffer_ops, cx).unwrap();
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
        let iterations = env::var("ITERATIONS")
            .map(|i| i.parse().expect("invalid `ITERATIONS` variable"))
            .unwrap_or(100);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
        let start_seed =
            env::var("SEED").map_or(0, |seed| seed.parse().expect("invalid `SEED` variable"));

        for seed in start_seed..start_seed + iterations {
            println!("{:?}", seed);
            let mut rng = &mut StdRng::seed_from_u64(seed);

            let reference_string_len = rng.gen_range(0..3);
            let mut reference_string = RandomCharIter::new(&mut rng)
                .take(reference_string_len)
                .collect::<String>();
            cx.add_model(|cx| {
                let mut buffer = Buffer::new(0, reference_string.as_str(), cx);
                let mut buffer_versions = Vec::new();
                log::info!(
                    "buffer text {:?}, version: {:?}",
                    buffer.text(),
                    buffer.version()
                );

                for _i in 0..operations {
                    let (old_ranges, new_text) = buffer.randomly_mutate(rng, cx);
                    for old_range in old_ranges.iter().rev() {
                        reference_string.replace_range(old_range.clone(), &new_text);
                    }
                    assert_eq!(buffer.text(), reference_string);
                    log::info!(
                        "buffer text {:?}, version: {:?}",
                        buffer.text(),
                        buffer.version()
                    );

                    if rng.gen_bool(0.25) {
                        buffer.randomly_undo_redo(rng, cx);
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
                    let edits = buffer
                        .edits_since(old_buffer.version.clone())
                        .collect::<Vec<_>>();

                    log::info!(
                        "mutating old buffer version {:?}, text: {:?}, edits since: {:?}",
                        old_buffer.version(),
                        old_buffer.text(),
                        edits,
                    );

                    let mut delta = 0_isize;
                    for Edit {
                        old_range,
                        new_range,
                        ..
                    } in edits
                    {
                        let old_len = old_range.end - old_range.start;
                        let new_len = new_range.end - new_range.start;
                        let old_start = (old_range.start as isize + delta) as usize;
                        let new_text: String = buffer.text_for_range(new_range).collect();
                        old_buffer.edit(Some(old_start..old_start + old_len), new_text, cx);

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
            buffer.edit(vec![0..0], "abcd\nefg\nhij", cx);
            buffer.edit(vec![12..12], "kl\nmno", cx);
            buffer.edit(vec![18..18], "\npqrs\n", cx);
            buffer.edit(vec![18..21], "\nPQ", cx);

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
            buffer.edit(vec![0..0], "abcd\nefgh\nij", cx);
            buffer.edit(vec![12..12], "kl\nmno", cx);
            buffer.edit(vec![18..18], "\npqrs", cx);
            buffer.edit(vec![18..21], "\nPQ", cx);

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
            buffer.edit(vec![0..0], "[workspace]\nmembers = [\n    \"xray_core\",\n    \"xray_server\",\n    \"xray_cli\",\n    \"xray_wasm\",\n]\n", cx);
            buffer.edit(vec![60..60], "\n", cx);

            let chars = buffer.chars_at(Point::new(6, 0));
            assert_eq!(chars.collect::<String>(), "    \"xray_wasm\",\n]\n");

            buffer
        });
    }

    #[gpui::test]
    fn test_anchors(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx);
            buffer.edit(vec![0..0], "abc", cx);
            let left_anchor = buffer.anchor_before(2);
            let right_anchor = buffer.anchor_after(2);

            buffer.edit(vec![1..1], "def\n", cx);
            assert_eq!(buffer.text(), "adef\nbc");
            assert_eq!(left_anchor.to_offset(&buffer), 6);
            assert_eq!(right_anchor.to_offset(&buffer), 6);
            assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
            assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

            buffer.edit(vec![2..3], "", cx);
            assert_eq!(buffer.text(), "adf\nbc");
            assert_eq!(left_anchor.to_offset(&buffer), 5);
            assert_eq!(right_anchor.to_offset(&buffer), 5);
            assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
            assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

            buffer.edit(vec![5..5], "ghi\n", cx);
            assert_eq!(buffer.text(), "adf\nbghi\nc");
            assert_eq!(left_anchor.to_offset(&buffer), 5);
            assert_eq!(right_anchor.to_offset(&buffer), 9);
            assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
            assert_eq!(right_anchor.to_point(&buffer), Point { row: 2, column: 0 });

            buffer.edit(vec![7..9], "", cx);
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

            buffer.edit(vec![0..0], "abc", cx);
            assert_eq!(buffer.text(), "abc");
            assert_eq!(before_start_anchor.to_offset(&buffer), 0);
            assert_eq!(after_end_anchor.to_offset(&buffer), 3);

            let after_start_anchor = buffer.anchor_after(0);
            let before_end_anchor = buffer.anchor_before(3);

            buffer.edit(vec![3..3], "def", cx);
            buffer.edit(vec![0..0], "ghi", cx);
            assert_eq!(buffer.text(), "ghiabcdef");
            assert_eq!(before_start_anchor.to_offset(&buffer), 0);
            assert_eq!(after_start_anchor.to_offset(&buffer), 3);
            assert_eq!(before_end_anchor.to_offset(&buffer), 6);
            assert_eq!(after_end_anchor.to_offset(&buffer), 9);
            buffer
        });
    }

    #[gpui::test]
    async fn test_is_dirty(mut cx: gpui::TestAppContext) {
        let language_registry = Arc::new(LanguageRegistry::new());

        let dir = temp_tree(json!({
            "file1": "abc",
            "file2": "def",
            "file3": "ghi",
        }));
        let tree = cx.add_model(|cx| Worktree::local(dir.path(), cx));
        tree.flush_fs_events(&cx).await;
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        let buffer1 = tree
            .update(&mut cx, |tree, cx| {
                tree.open_buffer("file1", language_registry.clone(), cx)
            })
            .await
            .unwrap();
        let events = Rc::new(RefCell::new(Vec::new()));

        // initially, the buffer isn't dirty.
        buffer1.update(&mut cx, |buffer, cx| {
            cx.subscribe(&buffer1, {
                let events = events.clone();
                move |_, event, _| events.borrow_mut().push(event.clone())
            });

            assert!(!buffer.is_dirty(cx.as_ref()));
            assert!(events.borrow().is_empty());

            buffer.edit(vec![1..2], "", cx);
        });

        // after the first edit, the buffer is dirty, and emits a dirtied event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(buffer.text() == "ac");
            assert!(buffer.is_dirty(cx.as_ref()));
            assert_eq!(*events.borrow(), &[Event::Edited, Event::Dirtied]);
            events.borrow_mut().clear();
            buffer.did_save(buffer.version(), cx).unwrap();
        });

        // after saving, the buffer is not dirty, and emits a saved event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(!buffer.is_dirty(cx.as_ref()));
            assert_eq!(*events.borrow(), &[Event::Saved]);
            events.borrow_mut().clear();

            buffer.edit(vec![1..1], "B", cx);
            buffer.edit(vec![2..2], "D", cx);
        });

        // after editing again, the buffer is dirty, and emits another dirty event.
        buffer1.update(&mut cx, |buffer, cx| {
            assert!(buffer.text() == "aBDc");
            assert!(buffer.is_dirty(cx.as_ref()));
            assert_eq!(
                *events.borrow(),
                &[Event::Edited, Event::Dirtied, Event::Edited],
            );
            events.borrow_mut().clear();

            // TODO - currently, after restoring the buffer to its
            // previously-saved state, the is still considered dirty.
            buffer.edit(vec![1..3], "", cx);
            assert!(buffer.text() == "ac");
            assert!(buffer.is_dirty(cx.as_ref()));
        });

        assert_eq!(*events.borrow(), &[Event::Edited]);

        // When a file is deleted, the buffer is considered dirty.
        let events = Rc::new(RefCell::new(Vec::new()));
        let buffer2 = tree
            .update(&mut cx, |tree, cx| {
                tree.open_buffer("file2", language_registry.clone(), cx)
            })
            .await
            .unwrap();
        buffer2.update(&mut cx, |_, cx| {
            cx.subscribe(&buffer2, {
                let events = events.clone();
                move |_, event, _| events.borrow_mut().push(event.clone())
            });
        });

        fs::remove_file(dir.path().join("file2")).unwrap();
        buffer2
            .condition(&cx, |b, cx| b.is_dirty(cx.as_ref()))
            .await;
        assert_eq!(
            *events.borrow(),
            &[Event::Dirtied, Event::FileHandleChanged]
        );

        // When a file is already dirty when deleted, we don't emit a Dirtied event.
        let events = Rc::new(RefCell::new(Vec::new()));
        let buffer3 = tree
            .update(&mut cx, |tree, cx| {
                tree.open_buffer("file3", language_registry.clone(), cx)
            })
            .await
            .unwrap();
        buffer3.update(&mut cx, |_, cx| {
            cx.subscribe(&buffer3, {
                let events = events.clone();
                move |_, event, _| events.borrow_mut().push(event.clone())
            });
        });

        tree.flush_fs_events(&cx).await;
        buffer3.update(&mut cx, |buffer, cx| {
            buffer.edit(Some(0..0), "x", cx);
        });
        events.borrow_mut().clear();
        fs::remove_file(dir.path().join("file3")).unwrap();
        buffer3
            .condition(&cx, |_, _| !events.borrow().is_empty())
            .await;
        assert_eq!(*events.borrow(), &[Event::FileHandleChanged]);
        cx.read(|cx| assert!(buffer3.read(cx).is_dirty(cx)));
    }

    #[gpui::test]
    async fn test_file_changes_on_disk(mut cx: gpui::TestAppContext) {
        let initial_contents = "aaa\nbbbbb\nc\n";
        let dir = temp_tree(json!({ "the-file": initial_contents }));
        let tree = cx.add_model(|cx| Worktree::local(dir.path(), cx));
        cx.read(|cx| tree.read(cx).as_local().unwrap().scan_complete())
            .await;

        let abs_path = dir.path().join("the-file");
        let buffer = tree
            .update(&mut cx, |tree, cx| {
                tree.open_buffer(Path::new("the-file"), Arc::new(LanguageRegistry::new()), cx)
            })
            .await
            .unwrap();

        // Add a cursor at the start of each row.
        let selection_set_id = buffer.update(&mut cx, |buffer, cx| {
            assert!(!buffer.is_dirty(cx.as_ref()));
            buffer.add_selection_set(
                (0..3)
                    .map(|row| {
                        let anchor = buffer.anchor_at(Point::new(row, 0), Bias::Right);
                        Selection {
                            id: row as usize,
                            start: anchor.clone(),
                            end: anchor,
                            reversed: false,
                            goal: SelectionGoal::None,
                        }
                    })
                    .collect::<Vec<_>>(),
                cx,
            )
        });

        // Change the file on disk, adding two new lines of text, and removing
        // one line.
        buffer.read_with(&cx, |buffer, cx| {
            assert!(!buffer.is_dirty(cx.as_ref()));
            assert!(!buffer.has_conflict(cx.as_ref()));
        });
        let new_contents = "AAAA\naaa\nBB\nbbbbb\n";
        fs::write(&abs_path, new_contents).unwrap();

        // Because the buffer was not modified, it is reloaded from disk. Its
        // contents are edited according to the diff between the old and new
        // file contents.
        buffer
            .condition(&cx, |buffer, _| buffer.text() != initial_contents)
            .await;

        buffer.update(&mut cx, |buffer, cx| {
            assert_eq!(buffer.text(), new_contents);
            assert!(!buffer.is_dirty(cx.as_ref()));
            assert!(!buffer.has_conflict(cx.as_ref()));

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
            buffer.edit(vec![0..0], " ", cx);
            assert!(buffer.is_dirty(cx.as_ref()));
        });

        // Change the file on disk again, adding blank lines to the beginning.
        fs::write(&abs_path, "\n\n\nAAAA\naaa\nBB\nbbbbb\n").unwrap();

        // Becaues the buffer is modified, it doesn't reload from disk, but is
        // marked as having a conflict.
        buffer
            .condition(&cx, |buffer, cx| buffer.has_conflict(cx.as_ref()))
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

            buffer.edit(vec![1..1], "abx", cx);
            buffer.edit(vec![3..4], "yzef", cx);
            buffer.edit(vec![3..5], "cd", cx);
            assert_eq!(buffer.text(), "1abcdef234");

            let edit1 = buffer.operations[0].clone();
            let edit2 = buffer.operations[1].clone();
            let edit3 = buffer.operations[2].clone();

            buffer.undo_or_redo(edit1.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1cdef234");
            buffer.undo_or_redo(edit1.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1abcdef234");

            buffer.undo_or_redo(edit2.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1abcdx234");
            buffer.undo_or_redo(edit3.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1abx234");
            buffer.undo_or_redo(edit2.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1abyzef234");
            buffer.undo_or_redo(edit3.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1abcdef234");

            buffer.undo_or_redo(edit3.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1abyzef234");
            buffer.undo_or_redo(edit1.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1yzef234");
            buffer.undo_or_redo(edit2.edit_id().unwrap(), cx).unwrap();
            assert_eq!(buffer.text(), "1234");

            buffer
        });
    }

    #[gpui::test]
    fn test_history(cx: &mut gpui::MutableAppContext) {
        cx.add_model(|cx| {
            let mut now = Instant::now();
            let mut buffer = Buffer::new(0, "123456", cx);

            let set_id =
                buffer.add_selection_set(buffer.selections_from_ranges(vec![4..4]).unwrap(), cx);
            buffer.start_transaction_at(Some(set_id), now, cx).unwrap();
            buffer.edit(vec![2..4], "cd", cx);
            buffer.end_transaction_at(Some(set_id), now, cx).unwrap();
            assert_eq!(buffer.text(), "12cd56");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![4..4]);

            buffer.start_transaction_at(Some(set_id), now, cx).unwrap();
            buffer
                .update_selection_set(
                    set_id,
                    buffer.selections_from_ranges(vec![1..3]).unwrap(),
                    cx,
                )
                .unwrap();
            buffer.edit(vec![4..5], "e", cx);
            buffer.end_transaction_at(Some(set_id), now, cx).unwrap();
            assert_eq!(buffer.text(), "12cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

            now += UNDO_GROUP_INTERVAL + Duration::from_millis(1);
            buffer.start_transaction_at(Some(set_id), now, cx).unwrap();
            buffer
                .update_selection_set(
                    set_id,
                    buffer.selections_from_ranges(vec![2..2]).unwrap(),
                    cx,
                )
                .unwrap();
            buffer.edit(vec![0..1], "a", cx);
            buffer.edit(vec![1..1], "b", cx);
            buffer.end_transaction_at(Some(set_id), now, cx).unwrap();
            assert_eq!(buffer.text(), "ab2cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![3..3]);

            // Last transaction happened past the group interval, undo it on its
            // own.
            buffer.undo(cx);
            assert_eq!(buffer.text(), "12cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

            // First two transactions happened within the group interval, undo them
            // together.
            buffer.undo(cx);
            assert_eq!(buffer.text(), "123456");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![4..4]);

            // Redo the first two transactions together.
            buffer.redo(cx);
            assert_eq!(buffer.text(), "12cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

            // Redo the last transaction on its own.
            buffer.redo(cx);
            assert_eq!(buffer.text(), "ab2cde6");
            assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![3..3]);

            buffer
        });
    }

    #[gpui::test]
    fn test_concurrent_edits(cx: &mut gpui::MutableAppContext) {
        let text = "abcdef";

        let buffer1 = cx.add_model(|cx| Buffer::new(1, text, cx));
        let buffer2 = cx.add_model(|cx| Buffer::new(2, text, cx));
        let buffer3 = cx.add_model(|cx| Buffer::new(3, text, cx));

        let buf1_op = buffer1.update(cx, |buffer, cx| {
            buffer.edit(vec![1..2], "12", cx);
            assert_eq!(buffer.text(), "a12cdef");
            buffer.operations.last().unwrap().clone()
        });
        let buf2_op = buffer2.update(cx, |buffer, cx| {
            buffer.edit(vec![3..4], "34", cx);
            assert_eq!(buffer.text(), "abc34ef");
            buffer.operations.last().unwrap().clone()
        });
        let buf3_op = buffer3.update(cx, |buffer, cx| {
            buffer.edit(vec![5..6], "56", cx);
            assert_eq!(buffer.text(), "abcde56");
            buffer.operations.last().unwrap().clone()
        });

        buffer1.update(cx, |buffer, _| {
            buffer.apply_op(buf2_op.clone()).unwrap();
            buffer.apply_op(buf3_op.clone()).unwrap();
        });
        buffer2.update(cx, |buffer, _| {
            buffer.apply_op(buf1_op.clone()).unwrap();
            buffer.apply_op(buf3_op.clone()).unwrap();
        });
        buffer3.update(cx, |buffer, _| {
            buffer.apply_op(buf1_op.clone()).unwrap();
            buffer.apply_op(buf2_op.clone()).unwrap();
        });

        assert_eq!(buffer1.read(cx).text(), "a12c34e56");
        assert_eq!(buffer2.read(cx).text(), "a12c34e56");
        assert_eq!(buffer3.read(cx).text(), "a12c34e56");
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
        let start_seed =
            env::var("SEED").map_or(0, |seed| seed.parse().expect("invalid `SEED` variable"));

        for seed in start_seed..start_seed + iterations {
            dbg!(seed);
            let mut rng = StdRng::seed_from_u64(seed);

            let base_text_len = rng.gen_range(0..10);
            let base_text = RandomCharIter::new(&mut rng)
                .take(base_text_len)
                .collect::<String>();
            let mut replica_ids = Vec::new();
            let mut buffers = Vec::new();
            let mut network = Network::new(StdRng::seed_from_u64(seed));

            for i in 0..peers {
                let buffer = cx.add_model(|cx| Buffer::new(i as ReplicaId, base_text.as_str(), cx));

                buffers.push(buffer);
                replica_ids.push(i as u16);
                network.add_peer(i as u16);
            }

            log::info!("initial text: {:?}", base_text);

            let mut mutation_count = operations;
            loop {
                let replica_index = rng.gen_range(0..peers);
                let replica_id = replica_ids[replica_index];
                buffers[replica_index].update(cx, |buffer, cx| match rng.gen_range(0..=100) {
                    0..=50 if mutation_count != 0 => {
                        buffer.randomly_mutate(&mut rng, cx);
                        network.broadcast(buffer.replica_id, mem::take(&mut buffer.operations));
                        log::info!("buffer {} text: {:?}", buffer.replica_id, buffer.text());
                        mutation_count -= 1;
                    }
                    51..=70 if mutation_count != 0 => {
                        buffer.randomly_undo_redo(&mut rng, cx);
                        network.broadcast(buffer.replica_id, mem::take(&mut buffer.operations));
                        mutation_count -= 1;
                    }
                    71..=100 if network.has_unreceived(replica_id) => {
                        let ops = network.receive(replica_id);
                        if !ops.is_empty() {
                            log::info!(
                                "peer {} applying {} ops from the network.",
                                replica_id,
                                ops.len()
                            );
                            buffer.apply_ops(ops, cx).unwrap();
                        }
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
                assert_eq!(
                    buffer.text(),
                    first_buffer.text(),
                    "Replica {} text != Replica 0 text",
                    buffer.replica_id
                );
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
            buf.start_transaction(None, cx).unwrap();

            let offset = buf.text().find(")").unwrap();
            buf.edit(vec![offset..offset], "b: C", cx);
            assert!(!buf.is_parsing());

            let offset = buf.text().find("}").unwrap();
            buf.edit(vec![offset..offset], " d; ", cx);
            assert!(!buf.is_parsing());

            buf.end_transaction(None, cx).unwrap();
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
            buf.edit(vec![offset..offset], ".e", cx);
            assert_eq!(buf.text(), "fn a(b: C) { d.e; }");
            assert!(buf.is_parsing());
        });
        buffer.update(&mut cx, |buf, cx| {
            let offset = buf.text().find(";").unwrap();
            buf.edit(vec![offset..offset], "(f)", cx);
            assert_eq!(buf.text(), "fn a(b: C) { d.e(f); }");
            assert!(buf.is_parsing());
        });
        buffer.update(&mut cx, |buf, cx| {
            let offset = buf.text().find("(f)").unwrap();
            buf.edit(vec![offset..offset], "::<G>", cx);
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
            buf.undo(cx);
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
            buf.redo(cx);
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
            let start = self.clip_offset(rng.gen_range(start_offset..=end), Bias::Right);
            start..end
        }

        pub fn randomly_edit<T>(
            &mut self,
            rng: &mut T,
            old_range_count: usize,
            cx: &mut ModelContext<Self>,
        ) -> (Vec<Range<usize>>, String)
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
            log::info!(
                "mutating buffer {} at {:?}: {:?}",
                self.replica_id,
                old_ranges,
                new_text
            );
            self.edit(old_ranges.iter().cloned(), new_text.as_str(), cx);
            (old_ranges, new_text)
        }

        pub fn randomly_mutate<T>(
            &mut self,
            rng: &mut T,
            cx: &mut ModelContext<Self>,
        ) -> (Vec<Range<usize>>, String)
        where
            T: Rng,
        {
            let (old_ranges, new_text) = self.randomly_edit(rng, 5, cx);

            // Randomly add, remove or mutate selection sets.
            let replica_selection_sets = &self
                .all_selections()
                .map(|(set_id, _)| *set_id)
                .filter(|set_id| self.replica_id == set_id.replica_id)
                .collect::<Vec<_>>();
            let set_id = replica_selection_sets.choose(rng);
            if set_id.is_some() && rng.gen_bool(1.0 / 6.0) {
                self.remove_selection_set(*set_id.unwrap(), cx).unwrap();
            } else {
                let mut ranges = Vec::new();
                for _ in 0..5 {
                    ranges.push(self.random_byte_range(0, rng));
                }
                let new_selections = self.selections_from_ranges(ranges).unwrap();

                if set_id.is_none() || rng.gen_bool(1.0 / 5.0) {
                    self.add_selection_set(new_selections, cx);
                } else {
                    self.update_selection_set(*set_id.unwrap(), new_selections, cx)
                        .unwrap();
                }
            }

            (old_ranges, new_text)
        }

        pub fn randomly_undo_redo(&mut self, rng: &mut impl Rng, cx: &mut ModelContext<Self>) {
            for _ in 0..rng.gen_range(1..=5) {
                if let Some(edit_id) = self.history.ops.keys().choose(rng).copied() {
                    log::info!("undoing buffer {} operation {:?}", self.replica_id, edit_id);
                    self.undo_or_redo(edit_id, cx).unwrap();
                }
            }
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
                .map(|(set_id, set)| (set_id, set.selections.as_ref()))
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
                Operation::Edit(edit) => Some(edit.timestamp.local()),
                Operation::Undo { undo, .. } => Some(undo.edit_id),
                Operation::UpdateSelections { .. } => None,
                Operation::SetActiveSelections { .. } => None,
            }
        }
    }
}
