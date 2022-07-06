pub use crate::{
    diagnostic_set::DiagnosticSet,
    highlight_map::{HighlightId, HighlightMap},
    proto, BracketPair, Grammar, Language, LanguageConfig, LanguageRegistry, PLAIN_TEXT,
};
use crate::{
    diagnostic_set::{DiagnosticEntry, DiagnosticGroup},
    outline::OutlineItem,
    CodeLabel, Outline,
};
use anyhow::{anyhow, Result};
use clock::ReplicaId;
use futures::FutureExt as _;
use gpui::{fonts::HighlightStyle, AppContext, Entity, ModelContext, MutableAppContext, Task};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use similar::{ChangeTag, TextDiff};
use smol::future::yield_now;
use std::{
    any::Any,
    cmp::{self, Ordering},
    collections::{BTreeMap, HashMap},
    ffi::OsString,
    future::Future,
    iter::{self, Iterator, Peekable},
    mem,
    ops::{Deref, DerefMut, Range},
    path::{Path, PathBuf},
    str,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    vec,
};
use sum_tree::TreeMap;
use text::operation_queue::OperationQueue;
pub use text::{Buffer as TextBuffer, BufferSnapshot as TextBufferSnapshot, Operation as _, *};
use theme::SyntaxTheme;
use tree_sitter::{InputEdit, QueryCursor, Tree};
use util::TryFutureExt as _;

#[cfg(any(test, feature = "test-support"))]
pub use {tree_sitter_rust, tree_sitter_typescript};

pub use lsp::DiagnosticSeverity;

lazy_static! {
    static ref QUERY_CURSORS: Mutex<Vec<QueryCursor>> = Default::default();
}

pub struct Buffer {
    text: TextBuffer,
    file: Option<Arc<dyn File>>,
    saved_version: clock::Global,
    saved_version_fingerprint: String,
    saved_mtime: SystemTime,
    transaction_depth: usize,
    was_dirty_before_starting_transaction: Option<bool>,
    language: Option<Arc<Language>>,
    autoindent_requests: Vec<Arc<AutoindentRequest>>,
    pending_autoindent: Option<Task<()>>,
    sync_parse_timeout: Duration,
    syntax_tree: Mutex<Option<SyntaxTree>>,
    parsing_in_background: bool,
    parse_count: usize,
    diagnostics: DiagnosticSet,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    selections_update_count: usize,
    diagnostics_update_count: usize,
    diagnostics_timestamp: clock::Lamport,
    file_update_count: usize,
    completion_triggers: Vec<String>,
    deferred_ops: OperationQueue<Operation>,
}

pub struct BufferSnapshot {
    text: text::BufferSnapshot,
    tree: Option<Tree>,
    file: Option<Arc<dyn File>>,
    diagnostics: DiagnosticSet,
    diagnostics_update_count: usize,
    file_update_count: usize,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    selections_update_count: usize,
    language: Option<Arc<Language>>,
    parse_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndentSize {
    pub len: u32,
    pub kind: IndentKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndentKind {
    Space,
    Tab,
}

#[derive(Clone, Debug)]
struct SelectionSet {
    line_mode: bool,
    selections: Arc<[Selection<Anchor>]>,
    lamport_timestamp: clock::Lamport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupId {
    source: Arc<str>,
    id: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: Option<String>,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub group_id: usize,
    pub is_valid: bool,
    pub is_primary: bool,
    pub is_disk_based: bool,
    pub is_unnecessary: bool,
}

#[derive(Clone, Debug)]
pub struct Completion {
    pub old_range: Range<Anchor>,
    pub new_text: String,
    pub label: CodeLabel,
    pub lsp_completion: lsp::CompletionItem,
}

#[derive(Clone, Debug)]
pub struct CodeAction {
    pub range: Range<Anchor>,
    pub lsp_action: lsp::CodeAction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    Buffer(text::Operation),
    UpdateDiagnostics {
        diagnostics: Arc<[DiagnosticEntry<Anchor>]>,
        lamport_timestamp: clock::Lamport,
    },
    UpdateSelections {
        selections: Arc<[Selection<Anchor>]>,
        lamport_timestamp: clock::Lamport,
        line_mode: bool,
    },
    UpdateCompletionTriggers {
        triggers: Vec<String>,
        lamport_timestamp: clock::Lamport,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Operation(Operation),
    Edited,
    DirtyChanged,
    Saved,
    FileHandleChanged,
    Reloaded,
    Reparsed,
    DiagnosticsUpdated,
    Closed,
}

pub trait File: Send + Sync {
    fn as_local(&self) -> Option<&dyn LocalFile>;

    fn is_local(&self) -> bool {
        self.as_local().is_some()
    }

    fn mtime(&self) -> SystemTime;

    /// Returns the path of this file relative to the worktree's root directory.
    fn path(&self) -> &Arc<Path>;

    /// Returns the path of this file relative to the worktree's parent directory (this means it
    /// includes the name of the worktree's root folder).
    fn full_path(&self, cx: &AppContext) -> PathBuf;

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name(&self, cx: &AppContext) -> OsString;

    fn is_deleted(&self) -> bool;

    fn save(
        &self,
        buffer_id: u64,
        text: Rope,
        version: clock::Global,
        line_ending: LineEnding,
        cx: &mut MutableAppContext,
    ) -> Task<Result<(clock::Global, String, SystemTime)>>;

    fn as_any(&self) -> &dyn Any;

    fn to_proto(&self) -> rpc::proto::File;
}

pub trait LocalFile: File {
    /// Returns the absolute path of this file.
    fn abs_path(&self, cx: &AppContext) -> PathBuf;

    fn load(&self, cx: &AppContext) -> Task<Result<String>>;

    fn buffer_reloaded(
        &self,
        buffer_id: u64,
        version: &clock::Global,
        fingerprint: String,
        line_ending: LineEnding,
        mtime: SystemTime,
        cx: &mut MutableAppContext,
    );
}

pub(crate) struct QueryCursorHandle(Option<QueryCursor>);

#[derive(Clone)]
struct SyntaxTree {
    tree: Tree,
    version: clock::Global,
}

#[derive(Clone)]
struct AutoindentRequest {
    before_edit: BufferSnapshot,
    edited: Vec<Anchor>,
    inserted: Option<Vec<Range<Anchor>>>,
    indent_size: IndentSize,
}

#[derive(Debug)]
struct IndentSuggestion {
    basis_row: u32,
    delta: Ordering,
}

pub(crate) struct TextProvider<'a>(pub(crate) &'a Rope);

struct BufferChunkHighlights<'a> {
    captures: tree_sitter::QueryCaptures<'a, 'a, TextProvider<'a>>,
    next_capture: Option<(tree_sitter::QueryMatch<'a, 'a>, usize)>,
    stack: Vec<(usize, HighlightId)>,
    highlight_map: HighlightMap,
    _query_cursor: QueryCursorHandle,
}

pub struct BufferChunks<'a> {
    range: Range<usize>,
    chunks: rope::Chunks<'a>,
    diagnostic_endpoints: Peekable<vec::IntoIter<DiagnosticEndpoint>>,
    error_depth: usize,
    warning_depth: usize,
    information_depth: usize,
    hint_depth: usize,
    unnecessary_depth: usize,
    highlights: Option<BufferChunkHighlights<'a>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Chunk<'a> {
    pub text: &'a str,
    pub syntax_highlight_id: Option<HighlightId>,
    pub highlight_style: Option<HighlightStyle>,
    pub diagnostic_severity: Option<DiagnosticSeverity>,
    pub is_unnecessary: bool,
}

pub(crate) struct Diff {
    base_version: clock::Global,
    new_text: Arc<str>,
    changes: Vec<(ChangeTag, usize)>,
    line_ending: LineEnding,
    start_offset: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct DiagnosticEndpoint {
    offset: usize,
    is_start: bool,
    severity: DiagnosticSeverity,
    is_unnecessary: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum CharKind {
    Punctuation,
    Whitespace,
    Word,
}

impl CharKind {
    pub fn coerce_punctuation(self, treat_punctuation_as_word: bool) -> Self {
        if treat_punctuation_as_word && self == CharKind::Punctuation {
            CharKind::Word
        } else {
            self
        }
    }
}

impl Buffer {
    pub fn new<T: Into<String>>(
        replica_id: ReplicaId,
        base_text: T,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(
            TextBuffer::new(replica_id, cx.model_id() as u64, base_text.into()),
            None,
        )
    }

    pub fn from_file<T: Into<String>>(
        replica_id: ReplicaId,
        base_text: T,
        file: Arc<dyn File>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(
            TextBuffer::new(replica_id, cx.model_id() as u64, base_text.into()),
            Some(file),
        )
    }

    pub fn from_proto(
        replica_id: ReplicaId,
        message: proto::BufferState,
        file: Option<Arc<dyn File>>,
        cx: &mut ModelContext<Self>,
    ) -> Result<Self> {
        let buffer = TextBuffer::new(replica_id, message.id, message.base_text);
        let mut this = Self::build(buffer, file);
        this.text.set_line_ending(proto::deserialize_line_ending(
            proto::LineEnding::from_i32(message.line_ending)
                .ok_or_else(|| anyhow!("missing line_ending"))?,
        ));
        let ops = message
            .operations
            .into_iter()
            .map(proto::deserialize_operation)
            .collect::<Result<Vec<_>>>()?;
        this.apply_ops(ops, cx)?;

        for selection_set in message.selections {
            let lamport_timestamp = clock::Lamport {
                replica_id: selection_set.replica_id as ReplicaId,
                value: selection_set.lamport_timestamp,
            };
            this.remote_selections.insert(
                selection_set.replica_id as ReplicaId,
                SelectionSet {
                    line_mode: selection_set.line_mode,
                    selections: proto::deserialize_selections(selection_set.selections),
                    lamport_timestamp,
                },
            );
            this.text.lamport_clock.observe(lamport_timestamp);
        }
        let snapshot = this.snapshot();
        let entries = proto::deserialize_diagnostics(message.diagnostics);
        this.apply_diagnostic_update(
            DiagnosticSet::from_sorted_entries(entries.iter().cloned(), &snapshot),
            clock::Lamport {
                replica_id: 0,
                value: message.diagnostics_timestamp,
            },
            cx,
        );

        this.completion_triggers = message.completion_triggers;

        Ok(this)
    }

    pub fn to_proto(&self) -> proto::BufferState {
        let mut operations = self
            .text
            .history()
            .map(|op| proto::serialize_operation(&Operation::Buffer(op.clone())))
            .chain(self.deferred_ops.iter().map(proto::serialize_operation))
            .collect::<Vec<_>>();
        operations.sort_unstable_by_key(proto::lamport_timestamp_for_operation);
        proto::BufferState {
            id: self.remote_id(),
            file: self.file.as_ref().map(|f| f.to_proto()),
            base_text: self.base_text().to_string(),
            operations,
            selections: self
                .remote_selections
                .iter()
                .map(|(replica_id, set)| proto::SelectionSet {
                    replica_id: *replica_id as u32,
                    selections: proto::serialize_selections(&set.selections),
                    lamport_timestamp: set.lamport_timestamp.value,
                    line_mode: set.line_mode,
                })
                .collect(),
            diagnostics: proto::serialize_diagnostics(self.diagnostics.iter()),
            diagnostics_timestamp: self.diagnostics_timestamp.value,
            completion_triggers: self.completion_triggers.clone(),
            line_ending: proto::serialize_line_ending(self.line_ending()) as i32,
        }
    }

    pub fn with_language(mut self, language: Arc<Language>, cx: &mut ModelContext<Self>) -> Self {
        self.set_language(Some(language), cx);
        self
    }

    fn build(buffer: TextBuffer, file: Option<Arc<dyn File>>) -> Self {
        let saved_mtime;
        if let Some(file) = file.as_ref() {
            saved_mtime = file.mtime();
        } else {
            saved_mtime = UNIX_EPOCH;
        }

        Self {
            saved_mtime,
            saved_version: buffer.version(),
            saved_version_fingerprint: buffer.as_rope().fingerprint(),
            transaction_depth: 0,
            was_dirty_before_starting_transaction: None,
            text: buffer,
            file,
            syntax_tree: Mutex::new(None),
            parsing_in_background: false,
            parse_count: 0,
            sync_parse_timeout: Duration::from_millis(1),
            autoindent_requests: Default::default(),
            pending_autoindent: Default::default(),
            language: None,
            remote_selections: Default::default(),
            selections_update_count: 0,
            diagnostics: Default::default(),
            diagnostics_update_count: 0,
            diagnostics_timestamp: Default::default(),
            file_update_count: 0,
            completion_triggers: Default::default(),
            deferred_ops: OperationQueue::new(),
        }
    }

    pub fn snapshot(&self) -> BufferSnapshot {
        BufferSnapshot {
            text: self.text.snapshot(),
            tree: self.syntax_tree(),
            file: self.file.clone(),
            remote_selections: self.remote_selections.clone(),
            diagnostics: self.diagnostics.clone(),
            diagnostics_update_count: self.diagnostics_update_count,
            file_update_count: self.file_update_count,
            language: self.language.clone(),
            parse_count: self.parse_count,
            selections_update_count: self.selections_update_count,
        }
    }

    pub fn as_text_snapshot(&self) -> &text::BufferSnapshot {
        &self.text
    }

    pub fn text_snapshot(&self) -> text::BufferSnapshot {
        self.text.snapshot()
    }

    pub fn file(&self) -> Option<&dyn File> {
        self.file.as_deref()
    }

    pub fn save(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<(clock::Global, String, SystemTime)>> {
        let file = if let Some(file) = self.file.as_ref() {
            file
        } else {
            return Task::ready(Err(anyhow!("buffer has no file")));
        };
        let text = self.as_rope().clone();
        let version = self.version();
        let save = file.save(
            self.remote_id(),
            text,
            version,
            self.line_ending(),
            cx.as_mut(),
        );
        cx.spawn(|this, mut cx| async move {
            let (version, fingerprint, mtime) = save.await?;
            this.update(&mut cx, |this, cx| {
                this.did_save(version.clone(), fingerprint.clone(), mtime, None, cx);
            });
            Ok((version, fingerprint, mtime))
        })
    }

    pub fn saved_version(&self) -> &clock::Global {
        &self.saved_version
    }

    pub fn set_language(&mut self, language: Option<Arc<Language>>, cx: &mut ModelContext<Self>) {
        *self.syntax_tree.lock() = None;
        self.language = language;
        self.reparse(cx);
    }

    pub fn did_save(
        &mut self,
        version: clock::Global,
        fingerprint: String,
        mtime: SystemTime,
        new_file: Option<Arc<dyn File>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.saved_version = version;
        self.saved_version_fingerprint = fingerprint;
        self.saved_mtime = mtime;
        if let Some(new_file) = new_file {
            self.file = Some(new_file);
            self.file_update_count += 1;
        }
        cx.emit(Event::Saved);
        cx.notify();
    }

    pub fn reload(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<Option<Transaction>>> {
        cx.spawn(|this, mut cx| async move {
            if let Some((new_mtime, new_text)) = this.read_with(&cx, |this, cx| {
                let file = this.file.as_ref()?.as_local()?;
                Some((file.mtime(), file.load(cx)))
            }) {
                let new_text = new_text.await?;
                let diff = this
                    .read_with(&cx, |this, cx| this.diff(new_text, cx))
                    .await;
                this.update(&mut cx, |this, cx| {
                    if let Some(transaction) = this.apply_diff(diff, cx).cloned() {
                        this.did_reload(
                            this.version(),
                            this.as_rope().fingerprint(),
                            this.line_ending(),
                            new_mtime,
                            cx,
                        );
                        Ok(Some(transaction))
                    } else {
                        Ok(None)
                    }
                })
            } else {
                Ok(None)
            }
        })
    }

    pub fn did_reload(
        &mut self,
        version: clock::Global,
        fingerprint: String,
        line_ending: LineEnding,
        mtime: SystemTime,
        cx: &mut ModelContext<Self>,
    ) {
        self.saved_version = version;
        self.saved_version_fingerprint = fingerprint;
        self.text.set_line_ending(line_ending);
        self.saved_mtime = mtime;
        if let Some(file) = self.file.as_ref().and_then(|f| f.as_local()) {
            file.buffer_reloaded(
                self.remote_id(),
                &self.saved_version,
                self.saved_version_fingerprint.clone(),
                self.line_ending(),
                self.saved_mtime,
                cx,
            );
        }
        cx.emit(Event::Reloaded);
        cx.notify();
    }

    pub fn file_updated(
        &mut self,
        new_file: Arc<dyn File>,
        cx: &mut ModelContext<Self>,
    ) -> Task<()> {
        let old_file = if let Some(file) = self.file.as_ref() {
            file
        } else {
            return Task::ready(());
        };
        let mut file_changed = false;
        let mut task = Task::ready(());

        if new_file.path() != old_file.path() {
            file_changed = true;
        }

        if new_file.is_deleted() {
            if !old_file.is_deleted() {
                file_changed = true;
                if !self.is_dirty() {
                    cx.emit(Event::DirtyChanged);
                }
            }
        } else {
            let new_mtime = new_file.mtime();
            if new_mtime != old_file.mtime() {
                file_changed = true;

                if !self.is_dirty() {
                    let reload = self.reload(cx).log_err().map(drop);
                    task = cx.foreground().spawn(reload);
                }
            }
        }

        if file_changed {
            self.file_update_count += 1;
            cx.emit(Event::FileHandleChanged);
            cx.notify();
        }
        self.file = Some(new_file);
        task
    }

    pub fn close(&mut self, cx: &mut ModelContext<Self>) {
        cx.emit(Event::Closed);
    }

    pub fn language(&self) -> Option<&Arc<Language>> {
        self.language.as_ref()
    }

    pub fn parse_count(&self) -> usize {
        self.parse_count
    }

    pub fn selections_update_count(&self) -> usize {
        self.selections_update_count
    }

    pub fn diagnostics_update_count(&self) -> usize {
        self.diagnostics_update_count
    }

    pub fn file_update_count(&self) -> usize {
        self.file_update_count
    }

    pub(crate) fn syntax_tree(&self) -> Option<Tree> {
        if let Some(syntax_tree) = self.syntax_tree.lock().as_mut() {
            self.interpolate_tree(syntax_tree);
            Some(syntax_tree.tree.clone())
        } else {
            None
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn is_parsing(&self) -> bool {
        self.parsing_in_background
    }

    #[cfg(test)]
    pub fn set_sync_parse_timeout(&mut self, timeout: Duration) {
        self.sync_parse_timeout = timeout;
    }

    fn reparse(&mut self, cx: &mut ModelContext<Self>) -> bool {
        if self.parsing_in_background {
            return false;
        }

        if let Some(grammar) = self.grammar().cloned() {
            let old_tree = self.syntax_tree();
            let text = self.as_rope().clone();
            let parsed_version = self.version();
            let parse_task = cx.background().spawn({
                let grammar = grammar.clone();
                async move { grammar.parse_text(&text, old_tree) }
            });

            match cx
                .background()
                .block_with_timeout(self.sync_parse_timeout, parse_task)
            {
                Ok(new_tree) => {
                    self.did_finish_parsing(new_tree, parsed_version, cx);
                    return true;
                }
                Err(parse_task) => {
                    self.parsing_in_background = true;
                    cx.spawn(move |this, mut cx| async move {
                        let new_tree = parse_task.await;
                        this.update(&mut cx, move |this, cx| {
                            let grammar_changed = this
                                .grammar()
                                .map_or(true, |curr_grammar| !Arc::ptr_eq(&grammar, curr_grammar));
                            let parse_again =
                                this.version.changed_since(&parsed_version) || grammar_changed;
                            this.parsing_in_background = false;
                            this.did_finish_parsing(new_tree, parsed_version, cx);

                            if parse_again && this.reparse(cx) {
                                return;
                            }
                        });
                    })
                    .detach();
                }
            }
        }
        false
    }

    fn interpolate_tree(&self, tree: &mut SyntaxTree) {
        for edit in self.edits_since::<(usize, Point)>(&tree.version) {
            let (bytes, lines) = edit.flatten();
            tree.tree.edit(&InputEdit {
                start_byte: bytes.new.start,
                old_end_byte: bytes.new.start + bytes.old.len(),
                new_end_byte: bytes.new.end,
                start_position: lines.new.start.to_ts_point(),
                old_end_position: (lines.new.start + (lines.old.end - lines.old.start))
                    .to_ts_point(),
                new_end_position: lines.new.end.to_ts_point(),
            });
        }
        tree.version = self.version();
    }

    fn did_finish_parsing(
        &mut self,
        tree: Tree,
        version: clock::Global,
        cx: &mut ModelContext<Self>,
    ) {
        self.parse_count += 1;
        *self.syntax_tree.lock() = Some(SyntaxTree { tree, version });
        self.request_autoindent(cx);
        cx.emit(Event::Reparsed);
        cx.notify();
    }

    pub fn update_diagnostics(&mut self, diagnostics: DiagnosticSet, cx: &mut ModelContext<Self>) {
        let lamport_timestamp = self.text.lamport_clock.tick();
        let op = Operation::UpdateDiagnostics {
            diagnostics: diagnostics.iter().cloned().collect(),
            lamport_timestamp,
        };
        self.apply_diagnostic_update(diagnostics, lamport_timestamp, cx);
        self.send_operation(op, cx);
    }

    fn request_autoindent(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(indent_sizes) = self.compute_autoindents() {
            let indent_sizes = cx.background().spawn(indent_sizes);
            match cx
                .background()
                .block_with_timeout(Duration::from_micros(500), indent_sizes)
            {
                Ok(indent_sizes) => self.apply_autoindents(indent_sizes, cx),
                Err(indent_sizes) => {
                    self.pending_autoindent = Some(cx.spawn(|this, mut cx| async move {
                        let indent_sizes = indent_sizes.await;
                        this.update(&mut cx, |this, cx| {
                            this.apply_autoindents(indent_sizes, cx);
                        });
                    }));
                }
            }
        }
    }

    fn compute_autoindents(&self) -> Option<impl Future<Output = BTreeMap<u32, IndentSize>>> {
        let max_rows_between_yields = 100;
        let snapshot = self.snapshot();
        if snapshot.language.is_none()
            || snapshot.tree.is_none()
            || self.autoindent_requests.is_empty()
        {
            return None;
        }

        let autoindent_requests = self.autoindent_requests.clone();
        Some(async move {
            let mut indent_sizes = BTreeMap::new();
            for request in autoindent_requests {
                let old_to_new_rows = request
                    .edited
                    .iter()
                    .map(|anchor| anchor.summary::<Point>(&request.before_edit).row)
                    .zip(
                        request
                            .edited
                            .iter()
                            .map(|anchor| anchor.summary::<Point>(&snapshot).row),
                    )
                    .collect::<BTreeMap<u32, u32>>();

                let mut old_suggestions = HashMap::<u32, IndentSize>::default();
                let old_edited_ranges =
                    contiguous_ranges(old_to_new_rows.keys().copied(), max_rows_between_yields);
                for old_edited_range in old_edited_ranges {
                    let suggestions = request
                        .before_edit
                        .suggest_autoindents(old_edited_range.clone())
                        .into_iter()
                        .flatten();
                    for (old_row, suggestion) in old_edited_range.zip(suggestions) {
                        if let Some(suggestion) = suggestion {
                            let mut suggested_indent = old_to_new_rows
                                .get(&suggestion.basis_row)
                                .and_then(|from_row| old_suggestions.get(from_row).copied())
                                .unwrap_or_else(|| {
                                    request
                                        .before_edit
                                        .indent_size_for_line(suggestion.basis_row)
                                });
                            if suggestion.delta.is_gt() {
                                suggested_indent += request.indent_size;
                            } else if suggestion.delta.is_lt() {
                                suggested_indent -= request.indent_size;
                            }
                            old_suggestions
                                .insert(*old_to_new_rows.get(&old_row).unwrap(), suggested_indent);
                        }
                    }
                    yield_now().await;
                }

                // At this point, old_suggestions contains the suggested indentation for all edited lines with respect to the state of the
                // buffer before the edit, but keyed by the row for these lines after the edits were applied.
                let new_edited_row_ranges =
                    contiguous_ranges(old_to_new_rows.values().copied(), max_rows_between_yields);
                for new_edited_row_range in new_edited_row_ranges {
                    let suggestions = snapshot
                        .suggest_autoindents(new_edited_row_range.clone())
                        .into_iter()
                        .flatten();
                    for (new_row, suggestion) in new_edited_row_range.zip(suggestions) {
                        if let Some(suggestion) = suggestion {
                            let mut suggested_indent = indent_sizes
                                .get(&suggestion.basis_row)
                                .copied()
                                .unwrap_or_else(|| {
                                    snapshot.indent_size_for_line(suggestion.basis_row)
                                });
                            if suggestion.delta.is_gt() {
                                suggested_indent += request.indent_size;
                            } else if suggestion.delta.is_lt() {
                                suggested_indent -= request.indent_size;
                            }
                            if old_suggestions
                                .get(&new_row)
                                .map_or(true, |old_indentation| {
                                    suggested_indent != *old_indentation
                                })
                            {
                                indent_sizes.insert(new_row, suggested_indent);
                            }
                        }
                    }
                    yield_now().await;
                }

                if let Some(inserted) = request.inserted.as_ref() {
                    let inserted_row_ranges = contiguous_ranges(
                        inserted
                            .iter()
                            .map(|range| range.to_point(&snapshot))
                            .flat_map(|range| range.start.row..range.end.row + 1),
                        max_rows_between_yields,
                    );
                    for inserted_row_range in inserted_row_ranges {
                        let suggestions = snapshot
                            .suggest_autoindents(inserted_row_range.clone())
                            .into_iter()
                            .flatten();
                        for (row, suggestion) in inserted_row_range.zip(suggestions) {
                            if let Some(suggestion) = suggestion {
                                let mut suggested_indent = indent_sizes
                                    .get(&suggestion.basis_row)
                                    .copied()
                                    .unwrap_or_else(|| {
                                        snapshot.indent_size_for_line(suggestion.basis_row)
                                    });
                                if suggestion.delta.is_gt() {
                                    suggested_indent += request.indent_size;
                                } else if suggestion.delta.is_lt() {
                                    suggested_indent -= request.indent_size;
                                }
                                indent_sizes.insert(row, suggested_indent);
                            }
                        }
                        yield_now().await;
                    }
                }
            }

            indent_sizes
        })
    }

    fn apply_autoindents(
        &mut self,
        indent_sizes: BTreeMap<u32, IndentSize>,
        cx: &mut ModelContext<Self>,
    ) {
        self.autoindent_requests.clear();
        self.start_transaction();
        for (row, indent_size) in &indent_sizes {
            self.set_indent_size_for_line(*row, *indent_size, cx);
        }
        self.end_transaction(cx);
    }

    fn set_indent_size_for_line(
        &mut self,
        row: u32,
        size: IndentSize,
        cx: &mut ModelContext<Self>,
    ) {
        let current_size = indent_size_for_line(&self, row);
        if size.kind != current_size.kind && current_size.len > 0 {
            return;
        }

        if size.len > current_size.len {
            let offset = Point::new(row, 0).to_offset(&*self);
            self.edit(
                [(
                    offset..offset,
                    iter::repeat(size.char())
                        .take((size.len - current_size.len) as usize)
                        .collect::<String>(),
                )],
                cx,
            );
        } else if size.len < current_size.len {
            self.edit(
                [(
                    Point::new(row, 0)..Point::new(row, current_size.len - size.len),
                    "",
                )],
                cx,
            );
        }
    }

    pub(crate) fn diff(&self, mut new_text: String, cx: &AppContext) -> Task<Diff> {
        let old_text = self.as_rope().clone();
        let base_version = self.version();
        cx.background().spawn(async move {
            let old_text = old_text.to_string();
            let line_ending = LineEnding::detect(&new_text);
            LineEnding::normalize(&mut new_text);
            let changes = TextDiff::from_lines(old_text.as_str(), new_text.as_str())
                .iter_all_changes()
                .map(|c| (c.tag(), c.value().len()))
                .collect::<Vec<_>>();
            Diff {
                base_version,
                new_text: new_text.into(),
                changes,
                line_ending,
                start_offset: 0,
            }
        })
    }

    pub(crate) fn apply_diff(
        &mut self,
        diff: Diff,
        cx: &mut ModelContext<Self>,
    ) -> Option<&Transaction> {
        if self.version == diff.base_version {
            self.finalize_last_transaction();
            self.start_transaction();
            self.text.set_line_ending(diff.line_ending);
            let mut offset = diff.start_offset;
            for (tag, len) in diff.changes {
                let range = offset..(offset + len);
                match tag {
                    ChangeTag::Equal => offset += len,
                    ChangeTag::Delete => {
                        self.edit([(range, "")], cx);
                    }
                    ChangeTag::Insert => {
                        self.edit(
                            [(
                                offset..offset,
                                &diff.new_text[range.start - diff.start_offset
                                    ..range.end - diff.start_offset],
                            )],
                            cx,
                        );
                        offset += len;
                    }
                }
            }
            if self.end_transaction(cx).is_some() {
                self.finalize_last_transaction()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.saved_version_fingerprint != self.as_rope().fingerprint()
            || self.file.as_ref().map_or(false, |file| file.is_deleted())
    }

    pub fn has_conflict(&self) -> bool {
        self.saved_version_fingerprint != self.as_rope().fingerprint()
            && self
                .file
                .as_ref()
                .map_or(false, |file| file.mtime() > self.saved_mtime)
    }

    pub fn subscribe(&mut self) -> Subscription {
        self.text.subscribe()
    }

    pub fn start_transaction(&mut self) -> Option<TransactionId> {
        self.start_transaction_at(Instant::now())
    }

    pub fn start_transaction_at(&mut self, now: Instant) -> Option<TransactionId> {
        self.transaction_depth += 1;
        if self.was_dirty_before_starting_transaction.is_none() {
            self.was_dirty_before_starting_transaction = Some(self.is_dirty());
        }
        self.text.start_transaction_at(now)
    }

    pub fn end_transaction(&mut self, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        self.end_transaction_at(Instant::now(), cx)
    }

    pub fn end_transaction_at(
        &mut self,
        now: Instant,
        cx: &mut ModelContext<Self>,
    ) -> Option<TransactionId> {
        assert!(self.transaction_depth > 0);
        self.transaction_depth -= 1;
        let was_dirty = if self.transaction_depth == 0 {
            self.was_dirty_before_starting_transaction.take().unwrap()
        } else {
            false
        };
        if let Some((transaction_id, start_version)) = self.text.end_transaction_at(now) {
            self.did_edit(&start_version, was_dirty, cx);
            Some(transaction_id)
        } else {
            None
        }
    }

    pub fn push_transaction(&mut self, transaction: Transaction, now: Instant) {
        self.text.push_transaction(transaction, now);
    }

    pub fn finalize_last_transaction(&mut self) -> Option<&Transaction> {
        self.text.finalize_last_transaction()
    }

    pub fn forget_transaction(&mut self, transaction_id: TransactionId) {
        self.text.forget_transaction(transaction_id);
    }

    pub fn wait_for_edits(
        &mut self,
        edit_ids: impl IntoIterator<Item = clock::Local>,
    ) -> impl Future<Output = ()> {
        self.text.wait_for_edits(edit_ids)
    }

    pub fn wait_for_anchors<'a>(
        &mut self,
        anchors: impl IntoIterator<Item = &'a Anchor>,
    ) -> impl Future<Output = ()> {
        self.text.wait_for_anchors(anchors)
    }

    pub fn wait_for_version(&mut self, version: clock::Global) -> impl Future<Output = ()> {
        self.text.wait_for_version(version)
    }

    pub fn set_active_selections(
        &mut self,
        selections: Arc<[Selection<Anchor>]>,
        line_mode: bool,
        cx: &mut ModelContext<Self>,
    ) {
        let lamport_timestamp = self.text.lamport_clock.tick();
        self.remote_selections.insert(
            self.text.replica_id(),
            SelectionSet {
                selections: selections.clone(),
                lamport_timestamp,
                line_mode,
            },
        );
        self.send_operation(
            Operation::UpdateSelections {
                selections,
                line_mode,
                lamport_timestamp,
            },
            cx,
        );
    }

    pub fn remove_active_selections(&mut self, cx: &mut ModelContext<Self>) {
        self.set_active_selections(Arc::from([]), false, cx);
    }

    pub fn set_text<T>(&mut self, text: T, cx: &mut ModelContext<Self>) -> Option<clock::Local>
    where
        T: Into<Arc<str>>,
    {
        self.edit_internal([(0..self.len(), text)], None, cx)
    }

    pub fn edit<I, S, T>(
        &mut self,
        edits_iter: I,
        cx: &mut ModelContext<Self>,
    ) -> Option<clock::Local>
    where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        self.edit_internal(edits_iter, None, cx)
    }

    pub fn edit_with_autoindent<I, S, T>(
        &mut self,
        edits_iter: I,
        indent_size: IndentSize,
        cx: &mut ModelContext<Self>,
    ) -> Option<clock::Local>
    where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        self.edit_internal(edits_iter, Some(indent_size), cx)
    }

    pub fn edit_internal<I, S, T>(
        &mut self,
        edits_iter: I,
        autoindent_size: Option<IndentSize>,
        cx: &mut ModelContext<Self>,
    ) -> Option<clock::Local>
    where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        // Skip invalid edits and coalesce contiguous ones.
        let mut edits: Vec<(Range<usize>, Arc<str>)> = Vec::new();
        for (range, new_text) in edits_iter {
            let mut range = range.start.to_offset(self)..range.end.to_offset(self);
            if range.start > range.end {
                mem::swap(&mut range.start, &mut range.end);
            }
            let new_text = new_text.into();
            if !new_text.is_empty() || !range.is_empty() {
                if let Some((prev_range, prev_text)) = edits.last_mut() {
                    if prev_range.end >= range.start {
                        prev_range.end = cmp::max(prev_range.end, range.end);
                        *prev_text = format!("{prev_text}{new_text}").into();
                    } else {
                        edits.push((range, new_text));
                    }
                } else {
                    edits.push((range, new_text));
                }
            }
        }
        if edits.is_empty() {
            return None;
        }

        self.start_transaction();
        self.pending_autoindent.take();
        let autoindent_request =
            self.language
                .as_ref()
                .and_then(|_| autoindent_size)
                .map(|autoindent_size| {
                    let before_edit = self.snapshot();
                    let edited = edits
                        .iter()
                        .filter_map(|(range, new_text)| {
                            let start = range.start.to_point(self);
                            if new_text.starts_with('\n')
                                && start.column == self.line_len(start.row)
                            {
                                None
                            } else {
                                Some(self.anchor_before(range.start))
                            }
                        })
                        .collect();
                    (before_edit, edited, autoindent_size)
                });

        let edit_operation = self.text.edit(edits.iter().cloned());
        let edit_id = edit_operation.local_timestamp();

        if let Some((before_edit, edited, size)) = autoindent_request {
            let mut delta = 0isize;

            let inserted_ranges = edits
                .into_iter()
                .zip(&edit_operation.as_edit().unwrap().new_text)
                .filter_map(|((range, _), new_text)| {
                    let first_newline_ix = new_text.find('\n')?;
                    let new_text_len = new_text.len();
                    let start = (delta + range.start as isize) as usize + first_newline_ix + 1;
                    let end = (delta + range.start as isize) as usize + new_text_len;
                    delta += new_text_len as isize - (range.end as isize - range.start as isize);
                    Some(self.anchor_before(start)..self.anchor_after(end))
                })
                .collect::<Vec<Range<Anchor>>>();

            let inserted = if inserted_ranges.is_empty() {
                None
            } else {
                Some(inserted_ranges)
            };

            self.autoindent_requests.push(Arc::new(AutoindentRequest {
                before_edit,
                edited,
                inserted,
                indent_size: size,
            }));
        }

        self.end_transaction(cx);
        self.send_operation(Operation::Buffer(edit_operation), cx);
        Some(edit_id)
    }

    fn did_edit(
        &mut self,
        old_version: &clock::Global,
        was_dirty: bool,
        cx: &mut ModelContext<Self>,
    ) {
        if self.edits_since::<usize>(old_version).next().is_none() {
            return;
        }

        self.reparse(cx);

        cx.emit(Event::Edited);
        if was_dirty != self.is_dirty() {
            cx.emit(Event::DirtyChanged);
        }
        cx.notify();
    }

    fn grammar(&self) -> Option<&Arc<Grammar>> {
        self.language.as_ref().and_then(|l| l.grammar.as_ref())
    }

    pub fn apply_ops<I: IntoIterator<Item = Operation>>(
        &mut self,
        ops: I,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        self.pending_autoindent.take();
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();
        let mut deferred_ops = Vec::new();
        let buffer_ops = ops
            .into_iter()
            .filter_map(|op| match op {
                Operation::Buffer(op) => Some(op),
                _ => {
                    if self.can_apply_op(&op) {
                        self.apply_op(op, cx);
                    } else {
                        deferred_ops.push(op);
                    }
                    None
                }
            })
            .collect::<Vec<_>>();
        self.text.apply_ops(buffer_ops)?;
        self.deferred_ops.insert(deferred_ops);
        self.flush_deferred_ops(cx);
        self.did_edit(&old_version, was_dirty, cx);
        // Notify independently of whether the buffer was edited as the operations could include a
        // selection update.
        cx.notify();
        Ok(())
    }

    fn flush_deferred_ops(&mut self, cx: &mut ModelContext<Self>) {
        let mut deferred_ops = Vec::new();
        for op in self.deferred_ops.drain().iter().cloned() {
            if self.can_apply_op(&op) {
                self.apply_op(op, cx);
            } else {
                deferred_ops.push(op);
            }
        }
        self.deferred_ops.insert(deferred_ops);
    }

    fn can_apply_op(&self, operation: &Operation) -> bool {
        match operation {
            Operation::Buffer(_) => {
                unreachable!("buffer operations should never be applied at this layer")
            }
            Operation::UpdateDiagnostics {
                diagnostics: diagnostic_set,
                ..
            } => diagnostic_set.iter().all(|diagnostic| {
                self.text.can_resolve(&diagnostic.range.start)
                    && self.text.can_resolve(&diagnostic.range.end)
            }),
            Operation::UpdateSelections { selections, .. } => selections
                .iter()
                .all(|s| self.can_resolve(&s.start) && self.can_resolve(&s.end)),
            Operation::UpdateCompletionTriggers { .. } => true,
        }
    }

    fn apply_op(&mut self, operation: Operation, cx: &mut ModelContext<Self>) {
        match operation {
            Operation::Buffer(_) => {
                unreachable!("buffer operations should never be applied at this layer")
            }
            Operation::UpdateDiagnostics {
                diagnostics: diagnostic_set,
                lamport_timestamp,
            } => {
                let snapshot = self.snapshot();
                self.apply_diagnostic_update(
                    DiagnosticSet::from_sorted_entries(diagnostic_set.iter().cloned(), &snapshot),
                    lamport_timestamp,
                    cx,
                );
            }
            Operation::UpdateSelections {
                selections,
                lamport_timestamp,
                line_mode,
            } => {
                if let Some(set) = self.remote_selections.get(&lamport_timestamp.replica_id) {
                    if set.lamport_timestamp > lamport_timestamp {
                        return;
                    }
                }

                self.remote_selections.insert(
                    lamport_timestamp.replica_id,
                    SelectionSet {
                        selections,
                        lamport_timestamp,
                        line_mode,
                    },
                );
                self.text.lamport_clock.observe(lamport_timestamp);
                self.selections_update_count += 1;
            }
            Operation::UpdateCompletionTriggers {
                triggers,
                lamport_timestamp,
            } => {
                self.completion_triggers = triggers;
                self.text.lamport_clock.observe(lamport_timestamp);
            }
        }
    }

    fn apply_diagnostic_update(
        &mut self,
        diagnostics: DiagnosticSet,
        lamport_timestamp: clock::Lamport,
        cx: &mut ModelContext<Self>,
    ) {
        if lamport_timestamp > self.diagnostics_timestamp {
            self.diagnostics = diagnostics;
            self.diagnostics_timestamp = lamport_timestamp;
            self.diagnostics_update_count += 1;
            self.text.lamport_clock.observe(lamport_timestamp);
            cx.notify();
            cx.emit(Event::DiagnosticsUpdated);
        }
    }

    fn send_operation(&mut self, operation: Operation, cx: &mut ModelContext<Self>) {
        cx.emit(Event::Operation(operation));
    }

    pub fn remove_peer(&mut self, replica_id: ReplicaId, cx: &mut ModelContext<Self>) {
        self.remote_selections.remove(&replica_id);
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        if let Some((transaction_id, operation)) = self.text.undo() {
            self.send_operation(Operation::Buffer(operation), cx);
            self.did_edit(&old_version, was_dirty, cx);
            Some(transaction_id)
        } else {
            None
        }
    }

    pub fn undo_to_transaction(
        &mut self,
        transaction_id: TransactionId,
        cx: &mut ModelContext<Self>,
    ) -> bool {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let operations = self.text.undo_to_transaction(transaction_id);
        let undone = !operations.is_empty();
        for operation in operations {
            self.send_operation(Operation::Buffer(operation), cx);
        }
        if undone {
            self.did_edit(&old_version, was_dirty, cx)
        }
        undone
    }

    pub fn redo(&mut self, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        if let Some((transaction_id, operation)) = self.text.redo() {
            self.send_operation(Operation::Buffer(operation), cx);
            self.did_edit(&old_version, was_dirty, cx);
            Some(transaction_id)
        } else {
            None
        }
    }

    pub fn redo_to_transaction(
        &mut self,
        transaction_id: TransactionId,
        cx: &mut ModelContext<Self>,
    ) -> bool {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let operations = self.text.redo_to_transaction(transaction_id);
        let redone = !operations.is_empty();
        for operation in operations {
            self.send_operation(Operation::Buffer(operation), cx);
        }
        if redone {
            self.did_edit(&old_version, was_dirty, cx)
        }
        redone
    }

    pub fn set_completion_triggers(&mut self, triggers: Vec<String>, cx: &mut ModelContext<Self>) {
        self.completion_triggers = triggers.clone();
        let lamport_timestamp = self.text.lamport_clock.tick();
        self.send_operation(
            Operation::UpdateCompletionTriggers {
                triggers,
                lamport_timestamp,
            },
            cx,
        );
        cx.notify();
    }

    pub fn completion_triggers(&self) -> &[String] {
        &self.completion_triggers
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Buffer {
    pub fn set_group_interval(&mut self, group_interval: Duration) {
        self.text.set_group_interval(group_interval);
    }

    pub fn randomly_edit<T>(
        &mut self,
        rng: &mut T,
        old_range_count: usize,
        cx: &mut ModelContext<Self>,
    ) where
        T: rand::Rng,
    {
        let mut edits: Vec<(Range<usize>, String)> = Vec::new();
        let mut last_end = None;
        for _ in 0..old_range_count {
            if last_end.map_or(false, |last_end| last_end >= self.len()) {
                break;
            }

            let new_start = last_end.map_or(0, |last_end| last_end + 1);
            let mut range = self.random_byte_range(new_start, rng);
            if rng.gen_bool(0.2) {
                mem::swap(&mut range.start, &mut range.end);
            }
            last_end = Some(range.end);

            let new_text_len = rng.gen_range(0..10);
            let new_text: String = crate::random_char_iter::RandomCharIter::new(&mut *rng)
                .take(new_text_len)
                .collect();

            edits.push((range, new_text));
        }
        log::info!("mutating buffer {} with {:?}", self.replica_id(), edits);
        self.edit(edits, cx);
    }

    pub fn randomly_undo_redo(&mut self, rng: &mut impl rand::Rng, cx: &mut ModelContext<Self>) {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let ops = self.text.randomly_undo_redo(rng);
        if !ops.is_empty() {
            for op in ops {
                self.send_operation(Operation::Buffer(op), cx);
                self.did_edit(&old_version, was_dirty, cx);
            }
        }
    }
}

impl Entity for Buffer {
    type Event = Event;
}

impl Deref for Buffer {
    type Target = TextBuffer;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl BufferSnapshot {
    pub fn indent_size_for_line(&self, row: u32) -> IndentSize {
        indent_size_for_line(&self, row)
    }

    fn suggest_autoindents<'a>(
        &'a self,
        row_range: Range<u32>,
    ) -> Option<impl Iterator<Item = Option<IndentSuggestion>> + 'a> {
        let language = self.language.as_ref()?;
        let grammar = language.grammar.as_ref()?;
        let config = &language.config;
        let prev_non_blank_row = self.prev_non_blank_row(row_range.start);

        // Find the suggested indentation ranges based on the syntax tree.
        let indents_query = grammar.indents_query.as_ref()?;
        let mut query_cursor = QueryCursorHandle::new();
        let indent_capture_ix = indents_query.capture_index_for_name("indent");
        let end_capture_ix = indents_query.capture_index_for_name("end");
        query_cursor.set_point_range(
            Point::new(prev_non_blank_row.unwrap_or(row_range.start), 0).to_ts_point()
                ..Point::new(row_range.end, 0).to_ts_point(),
        );

        let mut indent_ranges = Vec::<Range<Point>>::new();
        for mat in query_cursor.matches(
            indents_query,
            self.tree.as_ref()?.root_node(),
            TextProvider(self.as_rope()),
        ) {
            let mut start: Option<Point> = None;
            let mut end: Option<Point> = None;
            for capture in mat.captures {
                if Some(capture.index) == indent_capture_ix {
                    start.get_or_insert(Point::from_ts_point(capture.node.start_position()));
                    end.get_or_insert(Point::from_ts_point(capture.node.end_position()));
                } else if Some(capture.index) == end_capture_ix {
                    end = Some(Point::from_ts_point(capture.node.start_position().into()));
                }
            }

            if let Some((start, end)) = start.zip(end) {
                if start.row == end.row {
                    continue;
                }

                let range = start..end;
                match indent_ranges.binary_search_by_key(&range.start, |r| r.start) {
                    Err(ix) => indent_ranges.insert(ix, range),
                    Ok(ix) => {
                        let prev_range = &mut indent_ranges[ix];
                        prev_range.end = prev_range.end.max(range.end);
                    }
                }
            }
        }

        // Find the suggested indentation increases and decreased based on regexes.
        let mut indent_change_rows = Vec::<(u32, Ordering)>::new();
        self.for_each_line(
            Point::new(prev_non_blank_row.unwrap_or(row_range.start), 0)
                ..Point::new(row_range.end, 0),
            |row, line| {
                if config
                    .decrease_indent_pattern
                    .as_ref()
                    .map_or(false, |regex| regex.is_match(line))
                {
                    indent_change_rows.push((row, Ordering::Less));
                }
                if config
                    .increase_indent_pattern
                    .as_ref()
                    .map_or(false, |regex| regex.is_match(line))
                {
                    indent_change_rows.push((row + 1, Ordering::Greater));
                }
            },
        );

        let mut indent_changes = indent_change_rows.into_iter().peekable();
        let mut prev_row = if config.auto_indent_using_last_non_empty_line {
            prev_non_blank_row.unwrap_or(0)
        } else {
            row_range.start.saturating_sub(1)
        };
        let mut prev_row_start = Point::new(prev_row, self.indent_size_for_line(prev_row).len);
        Some(row_range.map(move |row| {
            let row_start = Point::new(row, self.indent_size_for_line(row).len);

            let mut indent_from_prev_row = false;
            let mut outdent_from_prev_row = false;
            let mut outdent_to_row = u32::MAX;

            while let Some((indent_row, delta)) = indent_changes.peek() {
                if *indent_row == row {
                    match delta {
                        Ordering::Less => outdent_from_prev_row = true,
                        Ordering::Greater => indent_from_prev_row = true,
                        _ => {}
                    }
                } else if *indent_row > row {
                    break;
                }
                indent_changes.next();
            }

            for range in &indent_ranges {
                if range.start.row >= row {
                    break;
                }
                if range.start.row == prev_row && range.end > row_start {
                    indent_from_prev_row = true;
                }
                if range.end > prev_row_start && range.end <= row_start {
                    outdent_to_row = outdent_to_row.min(range.start.row);
                }
            }

            let suggestion = if outdent_to_row == prev_row
                || (outdent_from_prev_row && indent_from_prev_row)
            {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Equal,
                })
            } else if indent_from_prev_row {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Greater,
                })
            } else if outdent_to_row < prev_row {
                Some(IndentSuggestion {
                    basis_row: outdent_to_row,
                    delta: Ordering::Equal,
                })
            } else if outdent_from_prev_row {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Less,
                })
            } else if config.auto_indent_using_last_non_empty_line || !self.is_line_blank(prev_row)
            {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Equal,
                })
            } else {
                None
            };

            prev_row = row;
            prev_row_start = row_start;
            suggestion
        }))
    }

    fn prev_non_blank_row(&self, mut row: u32) -> Option<u32> {
        while row > 0 {
            row -= 1;
            if !self.is_line_blank(row) {
                return Some(row);
            }
        }
        None
    }

    pub fn chunks<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        language_aware: bool,
    ) -> BufferChunks<'a> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);

        let mut tree = None;
        let mut diagnostic_endpoints = Vec::new();
        if language_aware {
            tree = self.tree.as_ref();
            for entry in self.diagnostics_in_range::<_, usize>(range.clone(), false) {
                diagnostic_endpoints.push(DiagnosticEndpoint {
                    offset: entry.range.start,
                    is_start: true,
                    severity: entry.diagnostic.severity,
                    is_unnecessary: entry.diagnostic.is_unnecessary,
                });
                diagnostic_endpoints.push(DiagnosticEndpoint {
                    offset: entry.range.end,
                    is_start: false,
                    severity: entry.diagnostic.severity,
                    is_unnecessary: entry.diagnostic.is_unnecessary,
                });
            }
            diagnostic_endpoints
                .sort_unstable_by_key(|endpoint| (endpoint.offset, !endpoint.is_start));
        }

        BufferChunks::new(
            self.text.as_rope(),
            range,
            tree,
            self.grammar(),
            diagnostic_endpoints,
        )
    }

    pub fn for_each_line<'a>(&'a self, range: Range<Point>, mut callback: impl FnMut(u32, &str)) {
        let mut line = String::new();
        let mut row = range.start.row;
        for chunk in self
            .as_rope()
            .chunks_in_range(range.to_offset(self))
            .chain(["\n"])
        {
            for (newline_ix, text) in chunk.split('\n').enumerate() {
                if newline_ix > 0 {
                    callback(row, &line);
                    row += 1;
                    line.clear();
                }
                line.push_str(text);
            }
        }
    }

    pub fn language(&self) -> Option<&Arc<Language>> {
        self.language.as_ref()
    }

    fn grammar(&self) -> Option<&Arc<Grammar>> {
        self.language
            .as_ref()
            .and_then(|language| language.grammar.as_ref())
    }

    pub fn surrounding_word<T: ToOffset>(&self, start: T) -> (Range<usize>, Option<CharKind>) {
        let mut start = start.to_offset(self);
        let mut end = start;
        let mut next_chars = self.chars_at(start).peekable();
        let mut prev_chars = self.reversed_chars_at(start).peekable();
        let word_kind = cmp::max(
            prev_chars.peek().copied().map(char_kind),
            next_chars.peek().copied().map(char_kind),
        );

        for ch in prev_chars {
            if Some(char_kind(ch)) == word_kind && ch != '\n' {
                start -= ch.len_utf8();
            } else {
                break;
            }
        }

        for ch in next_chars {
            if Some(char_kind(ch)) == word_kind && ch != '\n' {
                end += ch.len_utf8();
            } else {
                break;
            }
        }

        (start..end, word_kind)
    }

    pub fn range_for_syntax_ancestor<T: ToOffset>(&self, range: Range<T>) -> Option<Range<usize>> {
        let tree = self.tree.as_ref()?;
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = tree.root_node().walk();

        // Descend to the first leaf that touches the start of the range,
        // and if the range is non-empty, extends beyond the start.
        while cursor.goto_first_child_for_byte(range.start).is_some() {
            if !range.is_empty() && cursor.node().end_byte() == range.start {
                cursor.goto_next_sibling();
            }
        }

        // Ascend to the smallest ancestor that strictly contains the range.
        loop {
            let node_range = cursor.node().byte_range();
            if node_range.start <= range.start
                && node_range.end >= range.end
                && node_range.len() > range.len()
            {
                break;
            }
            if !cursor.goto_parent() {
                break;
            }
        }

        let left_node = cursor.node();

        // For an empty range, try to find another node immediately to the right of the range.
        if left_node.end_byte() == range.start {
            let mut right_node = None;
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    break;
                }
            }

            while cursor.node().start_byte() == range.start {
                right_node = Some(cursor.node());
                if !cursor.goto_first_child() {
                    break;
                }
            }

            // If there is a candidate node on both sides of the (empty) range, then
            // decide between the two by favoring a named node over an anonymous token.
            // If both nodes are the same in that regard, favor the right one.
            if let Some(right_node) = right_node {
                if right_node.is_named() || !left_node.is_named() {
                    return Some(right_node.byte_range());
                }
            }
        }

        Some(left_node.byte_range())
    }

    pub fn outline(&self, theme: Option<&SyntaxTheme>) -> Option<Outline<Anchor>> {
        self.outline_items_containing(0..self.len(), theme)
            .map(Outline::new)
    }

    pub fn symbols_containing<T: ToOffset>(
        &self,
        position: T,
        theme: Option<&SyntaxTheme>,
    ) -> Option<Vec<OutlineItem<Anchor>>> {
        let position = position.to_offset(&self);
        let mut items =
            self.outline_items_containing(position.saturating_sub(1)..position + 1, theme)?;
        let mut prev_depth = None;
        items.retain(|item| {
            let result = prev_depth.map_or(true, |prev_depth| item.depth > prev_depth);
            prev_depth = Some(item.depth);
            result
        });
        Some(items)
    }

    fn outline_items_containing(
        &self,
        range: Range<usize>,
        theme: Option<&SyntaxTheme>,
    ) -> Option<Vec<OutlineItem<Anchor>>> {
        let tree = self.tree.as_ref()?;
        let grammar = self
            .language
            .as_ref()
            .and_then(|language| language.grammar.as_ref())?;

        let outline_query = grammar.outline_query.as_ref()?;
        let mut cursor = QueryCursorHandle::new();
        cursor.set_byte_range(range.clone());
        let matches = cursor.matches(
            outline_query,
            tree.root_node(),
            TextProvider(self.as_rope()),
        );

        let mut chunks = self.chunks(0..self.len(), true);

        let item_capture_ix = outline_query.capture_index_for_name("item")?;
        let name_capture_ix = outline_query.capture_index_for_name("name")?;
        let context_capture_ix = outline_query
            .capture_index_for_name("context")
            .unwrap_or(u32::MAX);

        let mut stack = Vec::<Range<usize>>::new();
        let items = matches
            .filter_map(|mat| {
                let item_node = mat.nodes_for_capture_index(item_capture_ix).next()?;
                let item_range = item_node.start_byte()..item_node.end_byte();
                if item_range.end < range.start || item_range.start > range.end {
                    return None;
                }
                let mut text = String::new();
                let mut name_ranges = Vec::new();
                let mut highlight_ranges = Vec::new();

                for capture in mat.captures {
                    let node_is_name;
                    if capture.index == name_capture_ix {
                        node_is_name = true;
                    } else if capture.index == context_capture_ix {
                        node_is_name = false;
                    } else {
                        continue;
                    }

                    let range = capture.node.start_byte()..capture.node.end_byte();
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    if node_is_name {
                        let mut start = text.len();
                        let end = start + range.len();

                        // When multiple names are captured, then the matcheable text
                        // includes the whitespace in between the names.
                        if !name_ranges.is_empty() {
                            start -= 1;
                        }

                        name_ranges.push(start..end);
                    }

                    let mut offset = range.start;
                    chunks.seek(offset);
                    while let Some(mut chunk) = chunks.next() {
                        if chunk.text.len() > range.end - offset {
                            chunk.text = &chunk.text[0..(range.end - offset)];
                            offset = range.end;
                        } else {
                            offset += chunk.text.len();
                        }
                        let style = chunk
                            .syntax_highlight_id
                            .zip(theme)
                            .and_then(|(highlight, theme)| highlight.style(theme));
                        if let Some(style) = style {
                            let start = text.len();
                            let end = start + chunk.text.len();
                            highlight_ranges.push((start..end, style));
                        }
                        text.push_str(chunk.text);
                        if offset >= range.end {
                            break;
                        }
                    }
                }

                while stack.last().map_or(false, |prev_range| {
                    prev_range.start > item_range.start || prev_range.end < item_range.end
                }) {
                    stack.pop();
                }
                stack.push(item_range.clone());

                Some(OutlineItem {
                    depth: stack.len() - 1,
                    range: self.anchor_after(item_range.start)..self.anchor_before(item_range.end),
                    text,
                    highlight_ranges,
                    name_ranges,
                })
            })
            .collect::<Vec<_>>();
        Some(items)
    }

    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(Range<usize>, Range<usize>)> {
        let (grammar, tree) = self.grammar().zip(self.tree.as_ref())?;
        let brackets_query = grammar.brackets_query.as_ref()?;
        let open_capture_ix = brackets_query.capture_index_for_name("open")?;
        let close_capture_ix = brackets_query.capture_index_for_name("close")?;

        // Find bracket pairs that *inclusively* contain the given range.
        let range = range.start.to_offset(self).saturating_sub(1)..range.end.to_offset(self) + 1;
        let mut cursor = QueryCursorHandle::new();
        let matches = cursor.set_byte_range(range).matches(
            &brackets_query,
            tree.root_node(),
            TextProvider(self.as_rope()),
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

    pub fn remote_selections_in_range<'a>(
        &'a self,
        range: Range<Anchor>,
    ) -> impl 'a
           + Iterator<
        Item = (
            ReplicaId,
            bool,
            impl 'a + Iterator<Item = &'a Selection<Anchor>>,
        ),
    > {
        self.remote_selections
            .iter()
            .filter(|(replica_id, set)| {
                **replica_id != self.text.replica_id() && !set.selections.is_empty()
            })
            .map(move |(replica_id, set)| {
                let start_ix = match set.selections.binary_search_by(|probe| {
                    probe.end.cmp(&range.start, self).then(Ordering::Greater)
                }) {
                    Ok(ix) | Err(ix) => ix,
                };
                let end_ix = match set.selections.binary_search_by(|probe| {
                    probe.start.cmp(&range.end, self).then(Ordering::Less)
                }) {
                    Ok(ix) | Err(ix) => ix,
                };

                (
                    *replica_id,
                    set.line_mode,
                    set.selections[start_ix..end_ix].iter(),
                )
            })
    }

    pub fn diagnostics_in_range<'a, T, O>(
        &'a self,
        search_range: Range<T>,
        reversed: bool,
    ) -> impl 'a + Iterator<Item = DiagnosticEntry<O>>
    where
        T: 'a + Clone + ToOffset,
        O: 'a + FromAnchor,
    {
        self.diagnostics
            .range(search_range.clone(), self, true, reversed)
    }

    pub fn diagnostic_groups(&self) -> Vec<DiagnosticGroup<Anchor>> {
        let mut groups = Vec::new();
        self.diagnostics.groups(&mut groups, self);
        groups
    }

    pub fn diagnostic_group<'a, O>(
        &'a self,
        group_id: usize,
    ) -> impl 'a + Iterator<Item = DiagnosticEntry<O>>
    where
        O: 'a + FromAnchor,
    {
        self.diagnostics.group(group_id, self)
    }

    pub fn diagnostics_update_count(&self) -> usize {
        self.diagnostics_update_count
    }

    pub fn parse_count(&self) -> usize {
        self.parse_count
    }

    pub fn selections_update_count(&self) -> usize {
        self.selections_update_count
    }

    pub fn file(&self) -> Option<&dyn File> {
        self.file.as_deref()
    }

    pub fn file_update_count(&self) -> usize {
        self.file_update_count
    }
}

pub fn indent_size_for_line(text: &text::BufferSnapshot, row: u32) -> IndentSize {
    let mut result = IndentSize::spaces(0);
    for c in text.chars_at(Point::new(row, 0)) {
        let kind = match c {
            ' ' => IndentKind::Space,
            '\t' => IndentKind::Tab,
            _ => break,
        };
        if result.len == 0 {
            result.kind = kind;
        }
        result.len += 1;
    }
    result
}

impl Clone for BufferSnapshot {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            tree: self.tree.clone(),
            file: self.file.clone(),
            remote_selections: self.remote_selections.clone(),
            diagnostics: self.diagnostics.clone(),
            selections_update_count: self.selections_update_count,
            diagnostics_update_count: self.diagnostics_update_count,
            file_update_count: self.file_update_count,
            language: self.language.clone(),
            parse_count: self.parse_count,
        }
    }
}

impl Deref for BufferSnapshot {
    type Target = text::BufferSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl<'a> tree_sitter::TextProvider<'a> for TextProvider<'a> {
    type I = ByteChunks<'a>;

    fn text(&mut self, node: tree_sitter::Node) -> Self::I {
        ByteChunks(self.0.chunks_in_range(node.byte_range()))
    }
}

pub(crate) struct ByteChunks<'a>(rope::Chunks<'a>);

impl<'a> Iterator for ByteChunks<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(str::as_bytes)
    }
}

unsafe impl<'a> Send for BufferChunks<'a> {}

impl<'a> BufferChunks<'a> {
    pub(crate) fn new(
        text: &'a Rope,
        range: Range<usize>,
        tree: Option<&'a Tree>,
        grammar: Option<&'a Arc<Grammar>>,
        diagnostic_endpoints: Vec<DiagnosticEndpoint>,
    ) -> Self {
        let mut highlights = None;
        if let Some((grammar, tree)) = grammar.zip(tree) {
            if let Some(highlights_query) = grammar.highlights_query.as_ref() {
                let mut query_cursor = QueryCursorHandle::new();

                // TODO - add a Tree-sitter API to remove the need for this.
                let cursor = unsafe {
                    std::mem::transmute::<_, &'static mut QueryCursor>(query_cursor.deref_mut())
                };
                let captures = cursor.set_byte_range(range.clone()).captures(
                    highlights_query,
                    tree.root_node(),
                    TextProvider(text),
                );
                highlights = Some(BufferChunkHighlights {
                    captures,
                    next_capture: None,
                    stack: Default::default(),
                    highlight_map: grammar.highlight_map(),
                    _query_cursor: query_cursor,
                })
            }
        }

        let diagnostic_endpoints = diagnostic_endpoints.into_iter().peekable();
        let chunks = text.chunks_in_range(range.clone());

        BufferChunks {
            range,
            chunks,
            diagnostic_endpoints,
            error_depth: 0,
            warning_depth: 0,
            information_depth: 0,
            hint_depth: 0,
            unnecessary_depth: 0,
            highlights,
        }
    }

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
                            highlights.highlight_map.get(capture.index),
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

    fn update_diagnostic_depths(&mut self, endpoint: DiagnosticEndpoint) {
        let depth = match endpoint.severity {
            DiagnosticSeverity::ERROR => &mut self.error_depth,
            DiagnosticSeverity::WARNING => &mut self.warning_depth,
            DiagnosticSeverity::INFORMATION => &mut self.information_depth,
            DiagnosticSeverity::HINT => &mut self.hint_depth,
            _ => return,
        };
        if endpoint.is_start {
            *depth += 1;
        } else {
            *depth -= 1;
        }

        if endpoint.is_unnecessary {
            if endpoint.is_start {
                self.unnecessary_depth += 1;
            } else {
                self.unnecessary_depth -= 1;
            }
        }
    }

    fn current_diagnostic_severity(&self) -> Option<DiagnosticSeverity> {
        if self.error_depth > 0 {
            Some(DiagnosticSeverity::ERROR)
        } else if self.warning_depth > 0 {
            Some(DiagnosticSeverity::WARNING)
        } else if self.information_depth > 0 {
            Some(DiagnosticSeverity::INFORMATION)
        } else if self.hint_depth > 0 {
            Some(DiagnosticSeverity::HINT)
        } else {
            None
        }
    }

    fn current_code_is_unnecessary(&self) -> bool {
        self.unnecessary_depth > 0
    }
}

impl<'a> Iterator for BufferChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_capture_start = usize::MAX;
        let mut next_diagnostic_endpoint = usize::MAX;

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
                    let highlight_id = highlights.highlight_map.get(capture.index);
                    highlights
                        .stack
                        .push((capture.node.end_byte(), highlight_id));
                    highlights.next_capture = highlights.captures.next();
                }
            }
        }

        while let Some(endpoint) = self.diagnostic_endpoints.peek().copied() {
            if endpoint.offset <= self.range.start {
                self.update_diagnostic_depths(endpoint);
                self.diagnostic_endpoints.next();
            } else {
                next_diagnostic_endpoint = endpoint.offset;
                break;
            }
        }

        if let Some(chunk) = self.chunks.peek() {
            let chunk_start = self.range.start;
            let mut chunk_end = (self.chunks.offset() + chunk.len())
                .min(next_capture_start)
                .min(next_diagnostic_endpoint);
            let mut highlight_id = None;
            if let Some(highlights) = self.highlights.as_ref() {
                if let Some((parent_capture_end, parent_highlight_id)) = highlights.stack.last() {
                    chunk_end = chunk_end.min(*parent_capture_end);
                    highlight_id = Some(*parent_highlight_id);
                }
            }

            let slice =
                &chunk[chunk_start - self.chunks.offset()..chunk_end - self.chunks.offset()];
            self.range.start = chunk_end;
            if self.range.start == self.chunks.offset() + chunk.len() {
                self.chunks.next().unwrap();
            }

            Some(Chunk {
                text: slice,
                syntax_highlight_id: highlight_id,
                highlight_style: None,
                diagnostic_severity: self.current_diagnostic_severity(),
                is_unnecessary: self.current_code_is_unnecessary(),
            })
        } else {
            None
        }
    }
}

impl QueryCursorHandle {
    pub(crate) fn new() -> Self {
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
        cursor.set_point_range(Point::zero().to_ts_point()..Point::MAX.to_ts_point());
        QUERY_CURSORS.lock().push(cursor)
    }
}

trait ToTreeSitterPoint {
    fn to_ts_point(self) -> tree_sitter::Point;
    fn from_ts_point(point: tree_sitter::Point) -> Self;
}

impl ToTreeSitterPoint for Point {
    fn to_ts_point(self) -> tree_sitter::Point {
        tree_sitter::Point::new(self.row as usize, self.column as usize)
    }

    fn from_ts_point(point: tree_sitter::Point) -> Self {
        Point::new(point.row as u32, point.column as u32)
    }
}

impl operation_queue::Operation for Operation {
    fn lamport_timestamp(&self) -> clock::Lamport {
        match self {
            Operation::Buffer(_) => {
                unreachable!("buffer operations should never be deferred at this layer")
            }
            Operation::UpdateDiagnostics {
                lamport_timestamp, ..
            }
            | Operation::UpdateSelections {
                lamport_timestamp, ..
            }
            | Operation::UpdateCompletionTriggers {
                lamport_timestamp, ..
            } => *lamport_timestamp,
        }
    }
}

impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            code: Default::default(),
            severity: DiagnosticSeverity::ERROR,
            message: Default::default(),
            group_id: Default::default(),
            is_primary: Default::default(),
            is_valid: true,
            is_disk_based: false,
            is_unnecessary: false,
        }
    }
}

impl IndentSize {
    pub fn spaces(len: u32) -> Self {
        Self {
            len,
            kind: IndentKind::Space,
        }
    }

    pub fn tab() -> Self {
        Self {
            len: 1,
            kind: IndentKind::Tab,
        }
    }

    pub fn chars(&self) -> impl Iterator<Item = char> {
        iter::repeat(self.char()).take(self.len as usize)
    }

    pub fn char(&self) -> char {
        match self.kind {
            IndentKind::Space => ' ',
            IndentKind::Tab => '\t',
        }
    }
}

impl std::ops::AddAssign for IndentSize {
    fn add_assign(&mut self, other: IndentSize) {
        if self.len == 0 {
            *self = other;
        } else if self.kind == other.kind {
            self.len += other.len;
        }
    }
}

impl std::ops::SubAssign for IndentSize {
    fn sub_assign(&mut self, other: IndentSize) {
        if self.kind == other.kind && self.len >= other.len {
            self.len -= other.len;
        }
    }
}

impl Completion {
    pub fn sort_key(&self) -> (usize, &str) {
        let kind_key = match self.lsp_completion.kind {
            Some(lsp::CompletionItemKind::VARIABLE) => 0,
            _ => 1,
        };
        (kind_key, &self.label.text[self.label.filter_range.clone()])
    }

    pub fn is_snippet(&self) -> bool {
        self.lsp_completion.insert_text_format == Some(lsp::InsertTextFormat::SNIPPET)
    }
}

pub fn contiguous_ranges(
    values: impl Iterator<Item = u32>,
    max_len: usize,
) -> impl Iterator<Item = Range<u32>> {
    let mut values = values.into_iter();
    let mut current_range: Option<Range<u32>> = None;
    std::iter::from_fn(move || loop {
        if let Some(value) = values.next() {
            if let Some(range) = &mut current_range {
                if value == range.end && range.len() < max_len {
                    range.end += 1;
                    continue;
                }
            }

            let prev_range = current_range.clone();
            current_range = Some(value..(value + 1));
            if prev_range.is_some() {
                return prev_range;
            }
        } else {
            return current_range.take();
        }
    })
}

pub fn char_kind(c: char) -> CharKind {
    if c.is_whitespace() {
        CharKind::Whitespace
    } else if c.is_alphanumeric() || c == '_' {
        CharKind::Word
    } else {
        CharKind::Punctuation
    }
}
