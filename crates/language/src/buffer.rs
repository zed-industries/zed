pub use crate::{
    diagnostic_set::DiagnosticSet,
    highlight_map::{HighlightId, HighlightMap},
    proto, BracketPair, Grammar, Language, LanguageConfig, LanguageRegistry, PLAIN_TEXT,
};
use crate::{
    diagnostic_set::{DiagnosticEntry, DiagnosticGroup},
    outline::OutlineItem,
    syntax_map::{
        SyntaxMap, SyntaxMapCapture, SyntaxMapCaptures, SyntaxSnapshot, ToTreeSitterPoint,
    },
    CodeLabel, LanguageScope, Outline,
};
use anyhow::{anyhow, Result};
use clock::ReplicaId;
use fs::LineEnding;
use futures::FutureExt as _;
use gpui::{fonts::HighlightStyle, AppContext, Entity, ModelContext, Task};
use parking_lot::Mutex;
use settings::Settings;
use similar::{ChangeTag, TextDiff};
use smol::future::yield_now;
use std::{
    any::Any,
    cmp::{self, Ordering},
    collections::BTreeMap,
    ffi::OsStr,
    future::Future,
    iter::{self, Iterator, Peekable},
    mem,
    ops::{Deref, Range},
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
#[cfg(any(test, feature = "test-support"))]
use util::RandomCharIter;
use util::{RangeExt, TryFutureExt as _};

#[cfg(any(test, feature = "test-support"))]
pub use {tree_sitter_rust, tree_sitter_typescript};

pub use lsp::DiagnosticSeverity;

struct GitDiffStatus {
    diff: git::diff::BufferDiff,
    update_in_progress: bool,
    update_requested: bool,
}

pub struct Buffer {
    text: TextBuffer,
    diff_base: Option<String>,
    git_diff_status: GitDiffStatus,
    file: Option<Arc<dyn File>>,
    saved_version: clock::Global,
    saved_version_fingerprint: RopeFingerprint,
    saved_mtime: SystemTime,
    transaction_depth: usize,
    was_dirty_before_starting_transaction: Option<bool>,
    language: Option<Arc<Language>>,
    autoindent_requests: Vec<Arc<AutoindentRequest>>,
    pending_autoindent: Option<Task<()>>,
    sync_parse_timeout: Duration,
    syntax_map: Mutex<SyntaxMap>,
    parsing_in_background: bool,
    parse_count: usize,
    diagnostics: DiagnosticSet,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    selections_update_count: usize,
    diagnostics_update_count: usize,
    diagnostics_timestamp: clock::Lamport,
    file_update_count: usize,
    git_diff_update_count: usize,
    completion_triggers: Vec<String>,
    completion_triggers_timestamp: clock::Lamport,
    deferred_ops: OperationQueue<Operation>,
}

pub struct BufferSnapshot {
    text: text::BufferSnapshot,
    pub git_diff: git::diff::BufferDiff,
    pub(crate) syntax: SyntaxSnapshot,
    file: Option<Arc<dyn File>>,
    diagnostics: DiagnosticSet,
    diagnostics_update_count: usize,
    file_update_count: usize,
    git_diff_update_count: usize,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    selections_update_count: usize,
    language: Option<Arc<Language>>,
    parse_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IndentSize {
    pub len: u32,
    pub kind: IndentKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IndentKind {
    #[default]
    Space,
    Tab,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum CursorShape {
    #[default]
    Bar,
    Block,
    Underscore,
    Hollow,
}

#[derive(Clone, Debug)]
struct SelectionSet {
    line_mode: bool,
    cursor_shape: CursorShape,
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
        cursor_shape: CursorShape,
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
    fn file_name<'a>(&'a self, cx: &'a AppContext) -> &'a OsStr;

    fn is_deleted(&self) -> bool;

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
        fingerprint: RopeFingerprint,
        line_ending: LineEnding,
        mtime: SystemTime,
        cx: &mut AppContext,
    );
}

#[derive(Clone, Debug)]
pub enum AutoindentMode {
    /// Indent each line of inserted text.
    EachLine,
    /// Apply the same indentation adjustment to all of the lines
    /// in a given insertion.
    Block {
        /// The original indentation level of the first line of each
        /// insertion, if it has been copied.
        original_indent_columns: Vec<u32>,
    },
}

#[derive(Clone)]
struct AutoindentRequest {
    before_edit: BufferSnapshot,
    entries: Vec<AutoindentRequestEntry>,
    is_block_mode: bool,
}

#[derive(Clone)]
struct AutoindentRequestEntry {
    /// A range of the buffer whose indentation should be adjusted.
    range: Range<Anchor>,
    /// Whether or not these lines should be considered brand new, for the
    /// purpose of auto-indent. When text is not new, its indentation will
    /// only be adjusted if the suggested indentation level has *changed*
    /// since the edit was made.
    first_line_is_new: bool,
    indent_size: IndentSize,
    original_indent_column: Option<u32>,
}

#[derive(Debug)]
struct IndentSuggestion {
    basis_row: u32,
    delta: Ordering,
    within_error: bool,
}

struct BufferChunkHighlights<'a> {
    captures: SyntaxMapCaptures<'a>,
    next_capture: Option<SyntaxMapCapture<'a>>,
    stack: Vec<(usize, HighlightId)>,
    highlight_maps: Vec<HighlightMap>,
}

pub struct BufferChunks<'a> {
    range: Range<usize>,
    chunks: text::Chunks<'a>,
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

pub struct Diff {
    pub(crate) base_version: clock::Global,
    line_ending: LineEnding,
    edits: Vec<(Range<usize>, Arc<str>)>,
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
            None,
        )
    }

    pub fn from_file<T: Into<String>>(
        replica_id: ReplicaId,
        base_text: T,
        diff_base: Option<T>,
        file: Arc<dyn File>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(
            TextBuffer::new(replica_id, cx.model_id() as u64, base_text.into()),
            diff_base.map(|h| h.into().into_boxed_str().into()),
            Some(file),
        )
    }

    pub fn from_proto(
        replica_id: ReplicaId,
        message: proto::BufferState,
        file: Option<Arc<dyn File>>,
    ) -> Result<Self> {
        let buffer = TextBuffer::new(replica_id, message.id, message.base_text);
        let mut this = Self::build(
            buffer,
            message.diff_base.map(|text| text.into_boxed_str().into()),
            file,
        );
        this.text.set_line_ending(proto::deserialize_line_ending(
            rpc::proto::LineEnding::from_i32(message.line_ending)
                .ok_or_else(|| anyhow!("missing line_ending"))?,
        ));
        this.saved_version = proto::deserialize_version(&message.saved_version);
        this.saved_version_fingerprint =
            proto::deserialize_fingerprint(&message.saved_version_fingerprint)?;
        this.saved_mtime = message
            .saved_mtime
            .ok_or_else(|| anyhow!("invalid saved_mtime"))?
            .into();
        Ok(this)
    }

    pub fn to_proto(&self) -> proto::BufferState {
        proto::BufferState {
            id: self.remote_id(),
            file: self.file.as_ref().map(|f| f.to_proto()),
            base_text: self.base_text().to_string(),
            diff_base: self.diff_base.as_ref().map(|h| h.to_string()),
            line_ending: proto::serialize_line_ending(self.line_ending()) as i32,
            saved_version: proto::serialize_version(&self.saved_version),
            saved_version_fingerprint: proto::serialize_fingerprint(self.saved_version_fingerprint),
            saved_mtime: Some(self.saved_mtime.into()),
        }
    }

    pub fn serialize_ops(
        &self,
        since: Option<clock::Global>,
        cx: &AppContext,
    ) -> Task<Vec<proto::Operation>> {
        let mut operations = Vec::new();
        operations.extend(self.deferred_ops.iter().map(proto::serialize_operation));
        operations.extend(self.remote_selections.iter().map(|(_, set)| {
            proto::serialize_operation(&Operation::UpdateSelections {
                selections: set.selections.clone(),
                lamport_timestamp: set.lamport_timestamp,
                line_mode: set.line_mode,
                cursor_shape: set.cursor_shape,
            })
        }));
        operations.push(proto::serialize_operation(&Operation::UpdateDiagnostics {
            diagnostics: self.diagnostics.iter().cloned().collect(),
            lamport_timestamp: self.diagnostics_timestamp,
        }));
        operations.push(proto::serialize_operation(
            &Operation::UpdateCompletionTriggers {
                triggers: self.completion_triggers.clone(),
                lamport_timestamp: self.completion_triggers_timestamp,
            },
        ));

        let text_operations = self.text.operations().clone();
        cx.background().spawn(async move {
            let since = since.unwrap_or_default();
            operations.extend(
                text_operations
                    .iter()
                    .filter(|(_, op)| !since.observed(op.local_timestamp()))
                    .map(|(_, op)| proto::serialize_operation(&Operation::Buffer(op.clone()))),
            );
            operations.sort_unstable_by_key(proto::lamport_timestamp_for_operation);
            operations
        })
    }

    pub fn with_language(mut self, language: Arc<Language>, cx: &mut ModelContext<Self>) -> Self {
        self.set_language(Some(language), cx);
        self
    }

    fn build(buffer: TextBuffer, diff_base: Option<String>, file: Option<Arc<dyn File>>) -> Self {
        let saved_mtime = if let Some(file) = file.as_ref() {
            file.mtime()
        } else {
            UNIX_EPOCH
        };

        Self {
            saved_mtime,
            saved_version: buffer.version(),
            saved_version_fingerprint: buffer.as_rope().fingerprint(),
            transaction_depth: 0,
            was_dirty_before_starting_transaction: None,
            text: buffer,
            diff_base,
            git_diff_status: GitDiffStatus {
                diff: git::diff::BufferDiff::new(),
                update_in_progress: false,
                update_requested: false,
            },
            file,
            syntax_map: Mutex::new(SyntaxMap::new()),
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
            git_diff_update_count: 0,
            completion_triggers: Default::default(),
            completion_triggers_timestamp: Default::default(),
            deferred_ops: OperationQueue::new(),
        }
    }

    pub fn snapshot(&self) -> BufferSnapshot {
        let text = self.text.snapshot();
        let mut syntax_map = self.syntax_map.lock();
        syntax_map.interpolate(&text);
        let syntax = syntax_map.snapshot();

        BufferSnapshot {
            text,
            syntax,
            git_diff: self.git_diff_status.diff.clone(),
            file: self.file.clone(),
            remote_selections: self.remote_selections.clone(),
            diagnostics: self.diagnostics.clone(),
            diagnostics_update_count: self.diagnostics_update_count,
            file_update_count: self.file_update_count,
            git_diff_update_count: self.git_diff_update_count,
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

    pub fn file(&self) -> Option<&Arc<dyn File>> {
        self.file.as_ref()
    }

    pub fn saved_version(&self) -> &clock::Global {
        &self.saved_version
    }

    pub fn saved_version_fingerprint(&self) -> RopeFingerprint {
        self.saved_version_fingerprint
    }

    pub fn saved_mtime(&self) -> SystemTime {
        self.saved_mtime
    }

    pub fn set_language(&mut self, language: Option<Arc<Language>>, cx: &mut ModelContext<Self>) {
        self.syntax_map.lock().clear();
        self.language = language;
        self.reparse(cx);
    }

    pub fn set_language_registry(&mut self, language_registry: Arc<LanguageRegistry>) {
        self.syntax_map
            .lock()
            .set_language_registry(language_registry);
    }

    pub fn did_save(
        &mut self,
        version: clock::Global,
        fingerprint: RopeFingerprint,
        mtime: SystemTime,
        cx: &mut ModelContext<Self>,
    ) {
        self.saved_version = version;
        self.saved_version_fingerprint = fingerprint;
        self.saved_mtime = mtime;
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
                    if this.version() == diff.base_version {
                        this.finalize_last_transaction();
                        this.apply_diff(diff, cx);
                        if let Some(transaction) = this.finalize_last_transaction().cloned() {
                            this.did_reload(
                                this.version(),
                                this.as_rope().fingerprint(),
                                this.line_ending(),
                                new_mtime,
                                cx,
                            );
                            return Ok(Some(transaction));
                        }
                    }
                    Ok(None)
                })
            } else {
                Ok(None)
            }
        })
    }

    pub fn did_reload(
        &mut self,
        version: clock::Global,
        fingerprint: RopeFingerprint,
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
                self.saved_version_fingerprint,
                self.line_ending(),
                self.saved_mtime,
                cx,
            );
        }
        self.git_diff_recalc(cx);
        cx.emit(Event::Reloaded);
        cx.notify();
    }

    pub fn file_updated(
        &mut self,
        new_file: Arc<dyn File>,
        cx: &mut ModelContext<Self>,
    ) -> Task<()> {
        let mut file_changed = false;
        let mut task = Task::ready(());

        if let Some(old_file) = self.file.as_ref() {
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
        } else {
            file_changed = true;
        };

        if file_changed {
            self.file_update_count += 1;
            cx.emit(Event::FileHandleChanged);
            cx.notify();
        }
        self.file = Some(new_file);
        task
    }

    pub fn diff_base(&self) -> Option<&str> {
        self.diff_base.as_deref()
    }

    pub fn set_diff_base(&mut self, diff_base: Option<String>, cx: &mut ModelContext<Self>) {
        self.diff_base = diff_base;
        self.git_diff_recalc(cx);
    }

    pub fn needs_git_diff_recalc(&self) -> bool {
        self.git_diff_status.diff.needs_update(self)
    }

    pub fn git_diff_recalc(&mut self, cx: &mut ModelContext<Self>) {
        if self.git_diff_status.update_in_progress {
            self.git_diff_status.update_requested = true;
            return;
        }

        if let Some(diff_base) = &self.diff_base {
            let snapshot = self.snapshot();
            let diff_base = diff_base.clone();

            let mut diff = self.git_diff_status.diff.clone();
            let diff = cx.background().spawn(async move {
                diff.update(&diff_base, &snapshot).await;
                diff
            });

            cx.spawn_weak(|this, mut cx| async move {
                let buffer_diff = diff.await;
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        this.git_diff_status.diff = buffer_diff;
                        this.git_diff_update_count += 1;
                        cx.notify();

                        this.git_diff_status.update_in_progress = false;
                        if this.git_diff_status.update_requested {
                            this.git_diff_recalc(cx);
                        }
                    })
                }
            })
            .detach()
        } else {
            let snapshot = self.snapshot();
            self.git_diff_status.diff.clear(&snapshot);
            self.git_diff_update_count += 1;
            cx.notify();
        }
    }

    pub fn close(&mut self, cx: &mut ModelContext<Self>) {
        cx.emit(Event::Closed);
    }

    pub fn language(&self) -> Option<&Arc<Language>> {
        self.language.as_ref()
    }

    pub fn language_at<D: ToOffset>(&self, position: D) -> Option<Arc<Language>> {
        let offset = position.to_offset(self);
        self.syntax_map
            .lock()
            .layers_for_range(offset..offset, &self.text)
            .last()
            .map(|info| info.language.clone())
            .or_else(|| self.language.clone())
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

    pub fn git_diff_update_count(&self) -> usize {
        self.git_diff_update_count
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn is_parsing(&self) -> bool {
        self.parsing_in_background
    }

    pub fn contains_unknown_injections(&self) -> bool {
        self.syntax_map.lock().contains_unknown_injections()
    }

    #[cfg(test)]
    pub fn set_sync_parse_timeout(&mut self, timeout: Duration) {
        self.sync_parse_timeout = timeout;
    }

    /// Called after an edit to synchronize the buffer's main parse tree with
    /// the buffer's new underlying state.
    ///
    /// Locks the syntax map and interpolates the edits since the last reparse
    /// into the foreground syntax tree.
    ///
    /// Then takes a stable snapshot of the syntax map before unlocking it.
    /// The snapshot with the interpolated edits is sent to a background thread,
    /// where we ask Tree-sitter to perform an incremental parse.
    ///
    /// Meanwhile, in the foreground, we block the main thread for up to 1ms
    /// waiting on the parse to complete. As soon as it completes, we proceed
    /// synchronously, unless a 1ms timeout elapses.
    ///
    /// If we time out waiting on the parse, we spawn a second task waiting
    /// until the parse does complete and return with the interpolated tree still
    /// in the foreground. When the background parse completes, call back into
    /// the main thread and assign the foreground parse state.
    ///
    /// If the buffer or grammar changed since the start of the background parse,
    /// initiate an additional reparse recursively. To avoid concurrent parses
    /// for the same buffer, we only initiate a new parse if we are not already
    /// parsing in the background.
    pub fn reparse(&mut self, cx: &mut ModelContext<Self>) {
        if self.parsing_in_background {
            return;
        }
        let language = if let Some(language) = self.language.clone() {
            language
        } else {
            return;
        };

        let text = self.text_snapshot();
        let parsed_version = self.version();

        let mut syntax_map = self.syntax_map.lock();
        syntax_map.interpolate(&text);
        let language_registry = syntax_map.language_registry();
        let mut syntax_snapshot = syntax_map.snapshot();
        drop(syntax_map);

        let parse_task = cx.background().spawn({
            let language = language.clone();
            let language_registry = language_registry.clone();
            async move {
                syntax_snapshot.reparse(&text, language_registry, language);
                syntax_snapshot
            }
        });

        match cx
            .background()
            .block_with_timeout(self.sync_parse_timeout, parse_task)
        {
            Ok(new_syntax_snapshot) => {
                self.did_finish_parsing(new_syntax_snapshot, cx);
                return;
            }
            Err(parse_task) => {
                self.parsing_in_background = true;
                cx.spawn(move |this, mut cx| async move {
                    let new_syntax_map = parse_task.await;
                    this.update(&mut cx, move |this, cx| {
                        let grammar_changed =
                            this.language.as_ref().map_or(true, |current_language| {
                                !Arc::ptr_eq(&language, current_language)
                            });
                        let language_registry_changed = new_syntax_map
                            .contains_unknown_injections()
                            && language_registry.map_or(false, |registry| {
                                registry.version() != new_syntax_map.language_registry_version()
                            });
                        let parse_again = language_registry_changed
                            || grammar_changed
                            || this.version.changed_since(&parsed_version);
                        this.did_finish_parsing(new_syntax_map, cx);
                        this.parsing_in_background = false;
                        if parse_again {
                            this.reparse(cx);
                        }
                    });
                })
                .detach();
            }
        }
    }

    fn did_finish_parsing(&mut self, syntax_snapshot: SyntaxSnapshot, cx: &mut ModelContext<Self>) {
        self.parse_count += 1;
        self.syntax_map.lock().did_parse(syntax_snapshot);
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
        } else {
            self.autoindent_requests.clear();
        }
    }

    fn compute_autoindents(&self) -> Option<impl Future<Output = BTreeMap<u32, IndentSize>>> {
        let max_rows_between_yields = 100;
        let snapshot = self.snapshot();
        if snapshot.syntax.is_empty() || self.autoindent_requests.is_empty() {
            return None;
        }

        let autoindent_requests = self.autoindent_requests.clone();
        Some(async move {
            let mut indent_sizes = BTreeMap::new();
            for request in autoindent_requests {
                // Resolve each edited range to its row in the current buffer and in the
                // buffer before this batch of edits.
                let mut row_ranges = Vec::new();
                let mut old_to_new_rows = BTreeMap::new();
                let mut language_indent_sizes_by_new_row = Vec::new();
                for entry in &request.entries {
                    let position = entry.range.start;
                    let new_row = position.to_point(&snapshot).row;
                    let new_end_row = entry.range.end.to_point(&snapshot).row + 1;
                    language_indent_sizes_by_new_row.push((new_row, entry.indent_size));

                    if !entry.first_line_is_new {
                        let old_row = position.to_point(&request.before_edit).row;
                        old_to_new_rows.insert(old_row, new_row);
                    }
                    row_ranges.push((new_row..new_end_row, entry.original_indent_column));
                }

                // Build a map containing the suggested indentation for each of the edited lines
                // with respect to the state of the buffer before these edits. This map is keyed
                // by the rows for these lines in the current state of the buffer.
                let mut old_suggestions = BTreeMap::<u32, (IndentSize, bool)>::default();
                let old_edited_ranges =
                    contiguous_ranges(old_to_new_rows.keys().copied(), max_rows_between_yields);
                let mut language_indent_sizes = language_indent_sizes_by_new_row.iter().peekable();
                let mut language_indent_size = IndentSize::default();
                for old_edited_range in old_edited_ranges {
                    let suggestions = request
                        .before_edit
                        .suggest_autoindents(old_edited_range.clone())
                        .into_iter()
                        .flatten();
                    for (old_row, suggestion) in old_edited_range.zip(suggestions) {
                        if let Some(suggestion) = suggestion {
                            let new_row = *old_to_new_rows.get(&old_row).unwrap();

                            // Find the indent size based on the language for this row.
                            while let Some((row, size)) = language_indent_sizes.peek() {
                                if *row > new_row {
                                    break;
                                }
                                language_indent_size = *size;
                                language_indent_sizes.next();
                            }

                            let suggested_indent = old_to_new_rows
                                .get(&suggestion.basis_row)
                                .and_then(|from_row| {
                                    Some(old_suggestions.get(from_row).copied()?.0)
                                })
                                .unwrap_or_else(|| {
                                    request
                                        .before_edit
                                        .indent_size_for_line(suggestion.basis_row)
                                })
                                .with_delta(suggestion.delta, language_indent_size);
                            old_suggestions
                                .insert(new_row, (suggested_indent, suggestion.within_error));
                        }
                    }
                    yield_now().await;
                }

                // In block mode, only compute indentation suggestions for the first line
                // of each insertion. Otherwise, compute suggestions for every inserted line.
                let new_edited_row_ranges = contiguous_ranges(
                    row_ranges.iter().flat_map(|(range, _)| {
                        if request.is_block_mode {
                            range.start..range.start + 1
                        } else {
                            range.clone()
                        }
                    }),
                    max_rows_between_yields,
                );

                // Compute new suggestions for each line, but only include them in the result
                // if they differ from the old suggestion for that line.
                let mut language_indent_sizes = language_indent_sizes_by_new_row.iter().peekable();
                let mut language_indent_size = IndentSize::default();
                for new_edited_row_range in new_edited_row_ranges {
                    let suggestions = snapshot
                        .suggest_autoindents(new_edited_row_range.clone())
                        .into_iter()
                        .flatten();
                    for (new_row, suggestion) in new_edited_row_range.zip(suggestions) {
                        if let Some(suggestion) = suggestion {
                            // Find the indent size based on the language for this row.
                            while let Some((row, size)) = language_indent_sizes.peek() {
                                if *row > new_row {
                                    break;
                                }
                                language_indent_size = *size;
                                language_indent_sizes.next();
                            }

                            let suggested_indent = indent_sizes
                                .get(&suggestion.basis_row)
                                .copied()
                                .unwrap_or_else(|| {
                                    snapshot.indent_size_for_line(suggestion.basis_row)
                                })
                                .with_delta(suggestion.delta, language_indent_size);
                            if old_suggestions.get(&new_row).map_or(
                                true,
                                |(old_indentation, was_within_error)| {
                                    suggested_indent != *old_indentation
                                        && (!suggestion.within_error || *was_within_error)
                                },
                            ) {
                                indent_sizes.insert(new_row, suggested_indent);
                            }
                        }
                    }
                    yield_now().await;
                }

                // For each block of inserted text, adjust the indentation of the remaining
                // lines of the block by the same amount as the first line was adjusted.
                if request.is_block_mode {
                    for (row_range, original_indent_column) in
                        row_ranges
                            .into_iter()
                            .filter_map(|(range, original_indent_column)| {
                                if range.len() > 1 {
                                    Some((range, original_indent_column?))
                                } else {
                                    None
                                }
                            })
                    {
                        let new_indent = indent_sizes
                            .get(&row_range.start)
                            .copied()
                            .unwrap_or_else(|| snapshot.indent_size_for_line(row_range.start));
                        let delta = new_indent.len as i64 - original_indent_column as i64;
                        if delta != 0 {
                            for row in row_range.skip(1) {
                                indent_sizes.entry(row).or_insert_with(|| {
                                    let mut size = snapshot.indent_size_for_line(row);
                                    if size.kind == new_indent.kind {
                                        match delta.cmp(&0) {
                                            Ordering::Greater => size.len += delta as u32,
                                            Ordering::Less => {
                                                size.len = size.len.saturating_sub(-delta as u32)
                                            }
                                            Ordering::Equal => {}
                                        }
                                    }
                                    size
                                });
                            }
                        }
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

        let edits: Vec<_> = indent_sizes
            .into_iter()
            .filter_map(|(row, indent_size)| {
                let current_size = indent_size_for_line(self, row);
                Self::edit_for_indent_size_adjustment(row, current_size, indent_size)
            })
            .collect();

        self.edit(edits, None, cx);
    }

    // Create a minimal edit that will cause the the given row to be indented
    // with the given size. After applying this edit, the length of the line
    // will always be at least `new_size.len`.
    pub fn edit_for_indent_size_adjustment(
        row: u32,
        current_size: IndentSize,
        new_size: IndentSize,
    ) -> Option<(Range<Point>, String)> {
        if new_size.kind != current_size.kind {
            Some((
                Point::new(row, 0)..Point::new(row, current_size.len),
                iter::repeat(new_size.char())
                    .take(new_size.len as usize)
                    .collect::<String>(),
            ))
        } else {
            match new_size.len.cmp(&current_size.len) {
                Ordering::Greater => {
                    let point = Point::new(row, 0);
                    Some((
                        point..point,
                        iter::repeat(new_size.char())
                            .take((new_size.len - current_size.len) as usize)
                            .collect::<String>(),
                    ))
                }

                Ordering::Less => Some((
                    Point::new(row, 0)..Point::new(row, current_size.len - new_size.len),
                    String::new(),
                )),

                Ordering::Equal => None,
            }
        }
    }

    pub fn diff(&self, mut new_text: String, cx: &AppContext) -> Task<Diff> {
        let old_text = self.as_rope().clone();
        let base_version = self.version();
        cx.background().spawn(async move {
            let old_text = old_text.to_string();
            let line_ending = LineEnding::detect(&new_text);
            LineEnding::normalize(&mut new_text);
            let diff = TextDiff::from_chars(old_text.as_str(), new_text.as_str());
            let mut edits = Vec::new();
            let mut offset = 0;
            let empty: Arc<str> = "".into();
            for change in diff.iter_all_changes() {
                let value = change.value();
                let end_offset = offset + value.len();
                match change.tag() {
                    ChangeTag::Equal => {
                        offset = end_offset;
                    }
                    ChangeTag::Delete => {
                        edits.push((offset..end_offset, empty.clone()));
                        offset = end_offset;
                    }
                    ChangeTag::Insert => {
                        edits.push((offset..offset, value.into()));
                    }
                }
            }
            Diff {
                base_version,
                line_ending,
                edits,
            }
        })
    }

    /// Spawn a background task that searches the buffer for any whitespace
    /// at the ends of a lines, and returns a `Diff` that removes that whitespace.
    pub fn remove_trailing_whitespace(&self, cx: &AppContext) -> Task<Diff> {
        let old_text = self.as_rope().clone();
        let line_ending = self.line_ending();
        let base_version = self.version();
        cx.background().spawn(async move {
            let ranges = trailing_whitespace_ranges(&old_text);
            let empty = Arc::<str>::from("");
            Diff {
                base_version,
                line_ending,
                edits: ranges
                    .into_iter()
                    .map(|range| (range, empty.clone()))
                    .collect(),
            }
        })
    }

    /// Ensure that the buffer ends with a single newline character, and
    /// no other whitespace.
    pub fn ensure_final_newline(&mut self, cx: &mut ModelContext<Self>) {
        let len = self.len();
        let mut offset = len;
        for chunk in self.as_rope().reversed_chunks_in_range(0..len) {
            let non_whitespace_len = chunk
                .trim_end_matches(|c: char| c.is_ascii_whitespace())
                .len();
            offset -= chunk.len();
            offset += non_whitespace_len;
            if non_whitespace_len != 0 {
                if offset == len - 1 && chunk.get(non_whitespace_len..) == Some("\n") {
                    return;
                }
                break;
            }
        }
        self.edit([(offset..len, "\n")], None, cx);
    }

    /// Apply a diff to the buffer. If the buffer has changed since the given diff was
    /// calculated, then adjust the diff to account for those changes, and discard any
    /// parts of the diff that conflict with those changes.
    pub fn apply_diff(&mut self, diff: Diff, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        // Check for any edits to the buffer that have occurred since this diff
        // was computed.
        let snapshot = self.snapshot();
        let mut edits_since = snapshot.edits_since::<usize>(&diff.base_version).peekable();
        let mut delta = 0;
        let adjusted_edits = diff.edits.into_iter().filter_map(|(range, new_text)| {
            while let Some(edit_since) = edits_since.peek() {
                // If the edit occurs after a diff hunk, then it does not
                // affect that hunk.
                if edit_since.old.start > range.end {
                    break;
                }
                // If the edit precedes the diff hunk, then adjust the hunk
                // to reflect the edit.
                else if edit_since.old.end < range.start {
                    delta += edit_since.new_len() as i64 - edit_since.old_len() as i64;
                    edits_since.next();
                }
                // If the edit intersects a diff hunk, then discard that hunk.
                else {
                    return None;
                }
            }

            let start = (range.start as i64 + delta) as usize;
            let end = (range.end as i64 + delta) as usize;
            Some((start..end, new_text))
        });

        self.start_transaction();
        self.text.set_line_ending(diff.line_ending);
        self.edit(adjusted_edits, None, cx);
        self.end_transaction(cx)
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

    pub fn group_until_transaction(&mut self, transaction_id: TransactionId) {
        self.text.group_until_transaction(transaction_id);
    }

    pub fn forget_transaction(&mut self, transaction_id: TransactionId) {
        self.text.forget_transaction(transaction_id);
    }

    pub fn wait_for_edits(
        &mut self,
        edit_ids: impl IntoIterator<Item = clock::Local>,
    ) -> impl Future<Output = Result<()>> {
        self.text.wait_for_edits(edit_ids)
    }

    pub fn wait_for_anchors(
        &mut self,
        anchors: impl IntoIterator<Item = Anchor>,
    ) -> impl 'static + Future<Output = Result<()>> {
        self.text.wait_for_anchors(anchors)
    }

    pub fn wait_for_version(&mut self, version: clock::Global) -> impl Future<Output = Result<()>> {
        self.text.wait_for_version(version)
    }

    pub fn give_up_waiting(&mut self) {
        self.text.give_up_waiting();
    }

    pub fn set_active_selections(
        &mut self,
        selections: Arc<[Selection<Anchor>]>,
        line_mode: bool,
        cursor_shape: CursorShape,
        cx: &mut ModelContext<Self>,
    ) {
        let lamport_timestamp = self.text.lamport_clock.tick();
        self.remote_selections.insert(
            self.text.replica_id(),
            SelectionSet {
                selections: selections.clone(),
                lamport_timestamp,
                line_mode,
                cursor_shape,
            },
        );
        self.send_operation(
            Operation::UpdateSelections {
                selections,
                line_mode,
                lamport_timestamp,
                cursor_shape,
            },
            cx,
        );
    }

    pub fn remove_active_selections(&mut self, cx: &mut ModelContext<Self>) {
        if self
            .remote_selections
            .get(&self.text.replica_id())
            .map_or(true, |set| !set.selections.is_empty())
        {
            self.set_active_selections(Arc::from([]), false, Default::default(), cx);
        }
    }

    pub fn set_text<T>(&mut self, text: T, cx: &mut ModelContext<Self>) -> Option<clock::Local>
    where
        T: Into<Arc<str>>,
    {
        self.autoindent_requests.clear();
        self.edit([(0..self.len(), text)], None, cx)
    }

    pub fn edit<I, S, T>(
        &mut self,
        edits_iter: I,
        autoindent_mode: Option<AutoindentMode>,
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
        let autoindent_request = autoindent_mode
            .and_then(|mode| self.language.as_ref().map(|_| (self.snapshot(), mode)));

        let edit_operation = self.text.edit(edits.iter().cloned());
        let edit_id = edit_operation.local_timestamp();

        if let Some((before_edit, mode)) = autoindent_request {
            let mut delta = 0isize;
            let entries = edits
                .into_iter()
                .enumerate()
                .zip(&edit_operation.as_edit().unwrap().new_text)
                .map(|((ix, (range, _)), new_text)| {
                    let new_text_length = new_text.len();
                    let old_start = range.start.to_point(&before_edit);
                    let new_start = (delta + range.start as isize) as usize;
                    delta += new_text_length as isize - (range.end as isize - range.start as isize);

                    let mut range_of_insertion_to_indent = 0..new_text_length;
                    let mut first_line_is_new = false;
                    let mut original_indent_column = None;

                    // When inserting an entire line at the beginning of an existing line,
                    // treat the insertion as new.
                    if new_text.contains('\n')
                        && old_start.column <= before_edit.indent_size_for_line(old_start.row).len
                    {
                        first_line_is_new = true;
                    }

                    // When inserting text starting with a newline, avoid auto-indenting the
                    // previous line.
                    if new_text.starts_with('\n') {
                        range_of_insertion_to_indent.start += 1;
                        first_line_is_new = true;
                    }

                    // Avoid auto-indenting after the insertion.
                    if let AutoindentMode::Block {
                        original_indent_columns,
                    } = &mode
                    {
                        original_indent_column =
                            Some(original_indent_columns.get(ix).copied().unwrap_or_else(|| {
                                indent_size_for_text(
                                    new_text[range_of_insertion_to_indent.clone()].chars(),
                                )
                                .len
                            }));
                        if new_text[range_of_insertion_to_indent.clone()].ends_with('\n') {
                            range_of_insertion_to_indent.end -= 1;
                        }
                    }

                    AutoindentRequestEntry {
                        first_line_is_new,
                        original_indent_column,
                        indent_size: before_edit.language_indent_size_at(range.start, cx),
                        range: self.anchor_before(new_start + range_of_insertion_to_indent.start)
                            ..self.anchor_after(new_start + range_of_insertion_to_indent.end),
                    }
                })
                .collect();

            self.autoindent_requests.push(Arc::new(AutoindentRequest {
                before_edit,
                entries,
                is_block_mode: matches!(mode, AutoindentMode::Block { .. }),
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
                cursor_shape,
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
                        cursor_shape,
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
        self.completion_triggers_timestamp = self.text.lamport_clock.tick();
        self.send_operation(
            Operation::UpdateCompletionTriggers {
                triggers,
                lamport_timestamp: self.completion_triggers_timestamp,
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
    pub fn edit_via_marked_text(
        &mut self,
        marked_string: &str,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut ModelContext<Self>,
    ) {
        let edits = self.edits_for_marked_text(marked_string);
        self.edit(edits, autoindent_mode, cx);
    }

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
            let new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();

            edits.push((range, new_text));
        }
        log::info!("mutating buffer {} with {:?}", self.replica_id(), edits);
        self.edit(edits, None, cx);
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
        indent_size_for_line(self, row)
    }

    pub fn language_indent_size_at<T: ToOffset>(&self, position: T, cx: &AppContext) -> IndentSize {
        let language_name = self.language_at(position).map(|language| language.name());
        let settings = cx.global::<Settings>();
        if settings.hard_tabs(language_name.as_deref()) {
            IndentSize::tab()
        } else {
            IndentSize::spaces(settings.tab_size(language_name.as_deref()).get())
        }
    }

    pub fn suggested_indents(
        &self,
        rows: impl Iterator<Item = u32>,
        single_indent_size: IndentSize,
    ) -> BTreeMap<u32, IndentSize> {
        let mut result = BTreeMap::new();

        for row_range in contiguous_ranges(rows, 10) {
            let suggestions = match self.suggest_autoindents(row_range.clone()) {
                Some(suggestions) => suggestions,
                _ => break,
            };

            for (row, suggestion) in row_range.zip(suggestions) {
                let indent_size = if let Some(suggestion) = suggestion {
                    result
                        .get(&suggestion.basis_row)
                        .copied()
                        .unwrap_or_else(|| self.indent_size_for_line(suggestion.basis_row))
                        .with_delta(suggestion.delta, single_indent_size)
                } else {
                    self.indent_size_for_line(row)
                };

                result.insert(row, indent_size);
            }
        }

        result
    }

    fn suggest_autoindents(
        &self,
        row_range: Range<u32>,
    ) -> Option<impl Iterator<Item = Option<IndentSuggestion>> + '_> {
        let config = &self.language.as_ref()?.config;
        let prev_non_blank_row = self.prev_non_blank_row(row_range.start);

        // Find the suggested indentation ranges based on the syntax tree.
        let start = Point::new(prev_non_blank_row.unwrap_or(row_range.start), 0);
        let end = Point::new(row_range.end, 0);
        let range = (start..end).to_offset(&self.text);
        let mut matches = self.syntax.matches(range.clone(), &self.text, |grammar| {
            Some(&grammar.indents_config.as_ref()?.query)
        });
        let indent_configs = matches
            .grammars()
            .iter()
            .map(|grammar| grammar.indents_config.as_ref().unwrap())
            .collect::<Vec<_>>();

        let mut indent_ranges = Vec::<Range<Point>>::new();
        let mut outdent_positions = Vec::<Point>::new();
        while let Some(mat) = matches.peek() {
            let mut start: Option<Point> = None;
            let mut end: Option<Point> = None;

            let config = &indent_configs[mat.grammar_index];
            for capture in mat.captures {
                if capture.index == config.indent_capture_ix {
                    start.get_or_insert(Point::from_ts_point(capture.node.start_position()));
                    end.get_or_insert(Point::from_ts_point(capture.node.end_position()));
                } else if Some(capture.index) == config.start_capture_ix {
                    start = Some(Point::from_ts_point(capture.node.end_position()));
                } else if Some(capture.index) == config.end_capture_ix {
                    end = Some(Point::from_ts_point(capture.node.start_position()));
                } else if Some(capture.index) == config.outdent_capture_ix {
                    outdent_positions.push(Point::from_ts_point(capture.node.start_position()));
                }
            }

            matches.advance();
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

        let mut error_ranges = Vec::<Range<Point>>::new();
        let mut matches = self.syntax.matches(range.clone(), &self.text, |grammar| {
            Some(&grammar.error_query)
        });
        while let Some(mat) = matches.peek() {
            let node = mat.captures[0].node;
            let start = Point::from_ts_point(node.start_position());
            let end = Point::from_ts_point(node.end_position());
            let range = start..end;
            let ix = match error_ranges.binary_search_by_key(&range.start, |r| r.start) {
                Ok(ix) | Err(ix) => ix,
            };
            let mut end_ix = ix;
            while let Some(existing_range) = error_ranges.get(end_ix) {
                if existing_range.end < end {
                    end_ix += 1;
                } else {
                    break;
                }
            }
            error_ranges.splice(ix..end_ix, [range]);
            matches.advance();
        }

        outdent_positions.sort();
        for outdent_position in outdent_positions {
            // find the innermost indent range containing this outdent_position
            // set its end to the outdent position
            if let Some(range_to_truncate) = indent_ranges
                .iter_mut()
                .filter(|indent_range| indent_range.contains(&outdent_position))
                .last()
            {
                range_to_truncate.end = outdent_position;
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
                match indent_row.cmp(&row) {
                    Ordering::Equal => match delta {
                        Ordering::Less => outdent_from_prev_row = true,
                        Ordering::Greater => indent_from_prev_row = true,
                        _ => {}
                    },

                    Ordering::Greater => break,
                    Ordering::Less => {}
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

            let within_error = error_ranges
                .iter()
                .any(|e| e.start.row < row && e.end > row_start);

            let suggestion = if outdent_to_row == prev_row
                || (outdent_from_prev_row && indent_from_prev_row)
            {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Equal,
                    within_error,
                })
            } else if indent_from_prev_row {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Greater,
                    within_error,
                })
            } else if outdent_to_row < prev_row {
                Some(IndentSuggestion {
                    basis_row: outdent_to_row,
                    delta: Ordering::Equal,
                    within_error,
                })
            } else if outdent_from_prev_row {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Less,
                    within_error,
                })
            } else if config.auto_indent_using_last_non_empty_line || !self.is_line_blank(prev_row)
            {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Equal,
                    within_error,
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

    pub fn chunks<T: ToOffset>(&self, range: Range<T>, language_aware: bool) -> BufferChunks {
        let range = range.start.to_offset(self)..range.end.to_offset(self);

        let mut syntax = None;
        let mut diagnostic_endpoints = Vec::new();
        if language_aware {
            let captures = self.syntax.captures(range.clone(), &self.text, |grammar| {
                grammar.highlights_query.as_ref()
            });
            let highlight_maps = captures
                .grammars()
                .into_iter()
                .map(|grammar| grammar.highlight_map())
                .collect();
            syntax = Some((captures, highlight_maps));
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

        BufferChunks::new(self.text.as_rope(), range, syntax, diagnostic_endpoints)
    }

    pub fn for_each_line(&self, range: Range<Point>, mut callback: impl FnMut(u32, &str)) {
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

    pub fn language_at<D: ToOffset>(&self, position: D) -> Option<&Arc<Language>> {
        let offset = position.to_offset(self);
        self.syntax
            .layers_for_range(offset..offset, &self.text)
            .filter(|l| l.node.end_byte() > offset)
            .last()
            .map(|info| info.language)
            .or(self.language.as_ref())
    }

    pub fn language_scope_at<D: ToOffset>(&self, position: D) -> Option<LanguageScope> {
        let offset = position.to_offset(self);

        if let Some(layer_info) = self
            .syntax
            .layers_for_range(offset..offset, &self.text)
            .filter(|l| l.node.end_byte() > offset)
            .last()
        {
            Some(LanguageScope {
                language: layer_info.language.clone(),
                override_id: layer_info.override_id(offset, &self.text),
            })
        } else {
            self.language.clone().map(|language| LanguageScope {
                language,
                override_id: None,
            })
        }
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
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut result: Option<Range<usize>> = None;
        'outer: for layer in self.syntax.layers_for_range(range.clone(), &self.text) {
            let mut cursor = layer.node.walk();

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
                    continue 'outer;
                }
            }

            let left_node = cursor.node();
            let mut layer_result = left_node.byte_range();

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
                        layer_result = right_node.byte_range();
                    }
                }
            }

            if let Some(previous_result) = &result {
                if previous_result.len() < layer_result.len() {
                    continue;
                }
            }
            result = Some(layer_result);
        }

        result
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
        let position = position.to_offset(self);
        let mut items = self.outline_items_containing(
            position.saturating_sub(1)..self.len().min(position + 1),
            theme,
        )?;
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
        let mut matches = self.syntax.matches(range.clone(), &self.text, |grammar| {
            grammar.outline_config.as_ref().map(|c| &c.query)
        });
        let configs = matches
            .grammars()
            .iter()
            .map(|g| g.outline_config.as_ref().unwrap())
            .collect::<Vec<_>>();

        let mut stack = Vec::<Range<usize>>::new();
        let mut items = Vec::new();
        while let Some(mat) = matches.peek() {
            let config = &configs[mat.grammar_index];
            let item_node = mat.captures.iter().find_map(|cap| {
                if cap.index == config.item_capture_ix {
                    Some(cap.node)
                } else {
                    None
                }
            })?;

            let item_range = item_node.byte_range();
            if item_range.end < range.start || item_range.start > range.end {
                matches.advance();
                continue;
            }

            let mut buffer_ranges = Vec::new();
            for capture in mat.captures {
                let node_is_name;
                if capture.index == config.name_capture_ix {
                    node_is_name = true;
                } else if Some(capture.index) == config.context_capture_ix {
                    node_is_name = false;
                } else {
                    continue;
                }

                let mut range = capture.node.start_byte()..capture.node.end_byte();
                let start = capture.node.start_position();
                if capture.node.end_position().row > start.row {
                    range.end =
                        range.start + self.line_len(start.row as u32) as usize - start.column;
                }

                buffer_ranges.push((range, node_is_name));
            }

            if buffer_ranges.is_empty() {
                continue;
            }

            let mut text = String::new();
            let mut highlight_ranges = Vec::new();
            let mut name_ranges = Vec::new();
            let mut chunks = self.chunks(
                buffer_ranges.first().unwrap().0.start..buffer_ranges.last().unwrap().0.end,
                true,
            );
            for (buffer_range, is_name) in buffer_ranges {
                if !text.is_empty() {
                    text.push(' ');
                }
                if is_name {
                    let mut start = text.len();
                    let end = start + buffer_range.len();

                    // When multiple names are captured, then the matcheable text
                    // includes the whitespace in between the names.
                    if !name_ranges.is_empty() {
                        start -= 1;
                    }

                    name_ranges.push(start..end);
                }

                let mut offset = buffer_range.start;
                chunks.seek(offset);
                for mut chunk in chunks.by_ref() {
                    if chunk.text.len() > buffer_range.end - offset {
                        chunk.text = &chunk.text[0..(buffer_range.end - offset)];
                        offset = buffer_range.end;
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
                    if offset >= buffer_range.end {
                        break;
                    }
                }
            }

            matches.advance();
            while stack.last().map_or(false, |prev_range| {
                prev_range.start > item_range.start || prev_range.end < item_range.end
            }) {
                stack.pop();
            }
            stack.push(item_range.clone());

            items.push(OutlineItem {
                depth: stack.len() - 1,
                range: self.anchor_after(item_range.start)..self.anchor_before(item_range.end),
                text,
                highlight_ranges,
                name_ranges,
            })
        }
        Some(items)
    }

    /// Returns bracket range pairs overlapping or adjacent to `range`
    pub fn bracket_ranges<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = (Range<usize>, Range<usize>)> + 'a {
        // Find bracket pairs that *inclusively* contain the given range.
        let range = range.start.to_offset(self).saturating_sub(1)
            ..self.len().min(range.end.to_offset(self) + 1);

        let mut matches = self.syntax.matches(range.clone(), &self.text, |grammar| {
            grammar.brackets_config.as_ref().map(|c| &c.query)
        });
        let configs = matches
            .grammars()
            .iter()
            .map(|grammar| grammar.brackets_config.as_ref().unwrap())
            .collect::<Vec<_>>();

        iter::from_fn(move || {
            while let Some(mat) = matches.peek() {
                let mut open = None;
                let mut close = None;
                let config = &configs[mat.grammar_index];
                for capture in mat.captures {
                    if capture.index == config.open_capture_ix {
                        open = Some(capture.node.byte_range());
                    } else if capture.index == config.close_capture_ix {
                        close = Some(capture.node.byte_range());
                    }
                }

                matches.advance();

                let Some((open, close)) = open.zip(close) else { continue };

                let bracket_range = open.start..=close.end;
                if !bracket_range.overlaps(&range) {
                    continue;
                }

                return Some((open, close));
            }
            None
        })
    }

    #[allow(clippy::type_complexity)]
    pub fn remote_selections_in_range(
        &self,
        range: Range<Anchor>,
    ) -> impl Iterator<
        Item = (
            ReplicaId,
            bool,
            CursorShape,
            impl Iterator<Item = &Selection<Anchor>> + '_,
        ),
    > + '_ {
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
                    set.cursor_shape,
                    set.selections[start_ix..end_ix].iter(),
                )
            })
    }

    pub fn git_diff_hunks_in_row_range<'a>(
        &'a self,
        range: Range<u32>,
        reversed: bool,
    ) -> impl 'a + Iterator<Item = git::diff::DiffHunk<u32>> {
        self.git_diff.hunks_in_row_range(range, self, reversed)
    }

    pub fn git_diff_hunks_intersecting_range<'a>(
        &'a self,
        range: Range<Anchor>,
        reversed: bool,
    ) -> impl 'a + Iterator<Item = git::diff::DiffHunk<u32>> {
        self.git_diff
            .hunks_intersecting_range(range, self, reversed)
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
        self.diagnostics.range(search_range, self, true, reversed)
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

    pub fn file(&self) -> Option<&Arc<dyn File>> {
        self.file.as_ref()
    }

    pub fn resolve_file_path(&self, cx: &AppContext, include_root: bool) -> Option<PathBuf> {
        if let Some(file) = self.file() {
            if file.path().file_name().is_none() || include_root {
                Some(file.full_path(cx))
            } else {
                Some(file.path().to_path_buf())
            }
        } else {
            None
        }
    }

    pub fn file_update_count(&self) -> usize {
        self.file_update_count
    }

    pub fn git_diff_update_count(&self) -> usize {
        self.git_diff_update_count
    }
}

fn indent_size_for_line(text: &text::BufferSnapshot, row: u32) -> IndentSize {
    indent_size_for_text(text.chars_at(Point::new(row, 0)))
}

pub fn indent_size_for_text(text: impl Iterator<Item = char>) -> IndentSize {
    let mut result = IndentSize::spaces(0);
    for c in text {
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
            git_diff: self.git_diff.clone(),
            syntax: self.syntax.clone(),
            file: self.file.clone(),
            remote_selections: self.remote_selections.clone(),
            diagnostics: self.diagnostics.clone(),
            selections_update_count: self.selections_update_count,
            diagnostics_update_count: self.diagnostics_update_count,
            file_update_count: self.file_update_count,
            git_diff_update_count: self.git_diff_update_count,
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

unsafe impl<'a> Send for BufferChunks<'a> {}

impl<'a> BufferChunks<'a> {
    pub(crate) fn new(
        text: &'a Rope,
        range: Range<usize>,
        syntax: Option<(SyntaxMapCaptures<'a>, Vec<HighlightMap>)>,
        diagnostic_endpoints: Vec<DiagnosticEndpoint>,
    ) -> Self {
        let mut highlights = None;
        if let Some((captures, highlight_maps)) = syntax {
            highlights = Some(BufferChunkHighlights {
                captures,
                next_capture: None,
                stack: Default::default(),
                highlight_maps,
            })
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
            if let Some(capture) = &highlights.next_capture {
                if offset >= capture.node.start_byte() {
                    let next_capture_end = capture.node.end_byte();
                    if offset < next_capture_end {
                        highlights.stack.push((
                            next_capture_end,
                            highlights.highlight_maps[capture.grammar_index].get(capture.index),
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

            while let Some(capture) = highlights.next_capture.as_ref() {
                if self.range.start < capture.node.start_byte() {
                    next_capture_start = capture.node.start_byte();
                    break;
                } else {
                    let highlight_id =
                        highlights.highlight_maps[capture.grammar_index].get(capture.index);
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
            code: None,
            severity: DiagnosticSeverity::ERROR,
            message: Default::default(),
            group_id: 0,
            is_primary: false,
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

    pub fn with_delta(mut self, direction: Ordering, size: IndentSize) -> Self {
        match direction {
            Ordering::Less => {
                if self.kind == size.kind && self.len >= size.len {
                    self.len -= size.len;
                }
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                if self.len == 0 {
                    self = size;
                } else if self.kind == size.kind {
                    self.len += size.len;
                }
            }
        }
        self
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
    let mut values = values;
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

/// Find all of the ranges of whitespace that occur at the ends of lines
/// in the given rope.
///
/// This could also be done with a regex search, but this implementation
/// avoids copying text.
pub fn trailing_whitespace_ranges(rope: &Rope) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();

    let mut offset = 0;
    let mut prev_chunk_trailing_whitespace_range = 0..0;
    for chunk in rope.chunks() {
        let mut prev_line_trailing_whitespace_range = 0..0;
        for (i, line) in chunk.split('\n').enumerate() {
            let line_end_offset = offset + line.len();
            let trimmed_line_len = line.trim_end_matches(|c| matches!(c, ' ' | '\t')).len();
            let mut trailing_whitespace_range = (offset + trimmed_line_len)..line_end_offset;

            if i == 0 && trimmed_line_len == 0 {
                trailing_whitespace_range.start = prev_chunk_trailing_whitespace_range.start;
            }
            if !prev_line_trailing_whitespace_range.is_empty() {
                ranges.push(prev_line_trailing_whitespace_range);
            }

            offset = line_end_offset + 1;
            prev_line_trailing_whitespace_range = trailing_whitespace_range;
        }

        offset -= 1;
        prev_chunk_trailing_whitespace_range = prev_line_trailing_whitespace_range;
    }

    if !prev_chunk_trailing_whitespace_range.is_empty() {
        ranges.push(prev_chunk_trailing_whitespace_range);
    }

    ranges
}
