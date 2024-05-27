pub use crate::{
    diagnostic_set::DiagnosticSet,
    highlight_map::{HighlightId, HighlightMap},
    markdown::ParsedMarkdown,
    proto, Grammar, Language, LanguageRegistry,
};
use crate::{
    diagnostic_set::{DiagnosticEntry, DiagnosticGroup},
    language_settings::{language_settings, LanguageSettings},
    markdown::parse_markdown,
    outline::OutlineItem,
    syntax_map::{
        SyntaxLayer, SyntaxMap, SyntaxMapCapture, SyntaxMapCaptures, SyntaxMapMatches,
        SyntaxSnapshot, ToTreeSitterPoint,
    },
    task_context::RunnableRange,
    LanguageScope, Outline, RunnableCapture, RunnableTag,
};
use anyhow::{anyhow, Context, Result};
pub use clock::ReplicaId;
use futures::channel::oneshot;
use gpui::{
    AnyElement, AppContext, EventEmitter, HighlightStyle, ModelContext, Task, TaskLabel,
    WindowContext,
};
use lazy_static::lazy_static;
use lsp::LanguageServerId;
use parking_lot::Mutex;
use similar::{ChangeTag, TextDiff};
use smallvec::SmallVec;
use smol::future::yield_now;
use std::{
    any::Any,
    cmp::{self, Ordering},
    collections::BTreeMap,
    ffi::OsStr,
    fmt,
    future::Future,
    iter::{self, Iterator, Peekable},
    mem,
    ops::{Deref, Range},
    path::{Path, PathBuf},
    str,
    sync::Arc,
    time::{Duration, Instant, SystemTime},
    vec,
};
use sum_tree::TreeMap;
use text::operation_queue::OperationQueue;
use text::*;
pub use text::{
    Anchor, Bias, Buffer as TextBuffer, BufferId, BufferSnapshot as TextBufferSnapshot, Edit,
    OffsetRangeExt, OffsetUtf16, Patch, Point, PointUtf16, Rope, Selection, SelectionGoal,
    Subscription, TextDimension, TextSummary, ToOffset, ToOffsetUtf16, ToPoint, ToPointUtf16,
    Transaction, TransactionId, Unclipped,
};
use theme::SyntaxTheme;
#[cfg(any(test, feature = "test-support"))]
use util::RandomCharIter;
use util::RangeExt;

#[cfg(any(test, feature = "test-support"))]
pub use {tree_sitter_rust, tree_sitter_typescript};

pub use lsp::DiagnosticSeverity;

lazy_static! {
    /// A label for the background task spawned by the buffer to compute
    /// a diff against the contents of its file.
    pub static ref BUFFER_DIFF_TASK: TaskLabel = TaskLabel::new();
}

/// Indicate whether a [Buffer] has permissions to edit.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Capability {
    /// The buffer is a mutable replica.
    ReadWrite,
    /// The buffer is a read-only replica.
    ReadOnly,
}

pub type BufferRow = u32;

/// An in-memory representation of a source code file, including its text,
/// syntax trees, git status, and diagnostics.
pub struct Buffer {
    text: TextBuffer,
    diff_base: Option<Rope>,
    git_diff: git::diff::BufferDiff,
    file: Option<Arc<dyn File>>,
    /// The mtime of the file when this buffer was last loaded from
    /// or saved to disk.
    saved_mtime: Option<SystemTime>,
    /// The version vector when this buffer was last loaded from
    /// or saved to disk.
    saved_version: clock::Global,
    transaction_depth: usize,
    was_dirty_before_starting_transaction: Option<bool>,
    reload_task: Option<Task<Result<()>>>,
    language: Option<Arc<Language>>,
    autoindent_requests: Vec<Arc<AutoindentRequest>>,
    pending_autoindent: Option<Task<()>>,
    sync_parse_timeout: Duration,
    syntax_map: Mutex<SyntaxMap>,
    parsing_in_background: bool,
    parse_count: usize,
    diagnostics: SmallVec<[(LanguageServerId, DiagnosticSet); 2]>,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    selections_update_count: usize,
    diagnostics_update_count: usize,
    diagnostics_timestamp: clock::Lamport,
    file_update_count: usize,
    git_diff_update_count: usize,
    completion_triggers: Vec<String>,
    completion_triggers_timestamp: clock::Lamport,
    deferred_ops: OperationQueue<Operation>,
    capability: Capability,
    has_conflict: bool,
    diff_base_version: usize,
}

/// An immutable, cheaply cloneable representation of a fixed
/// state of a buffer.
pub struct BufferSnapshot {
    text: text::BufferSnapshot,
    git_diff: git::diff::BufferDiff,
    pub(crate) syntax: SyntaxSnapshot,
    file: Option<Arc<dyn File>>,
    diagnostics: SmallVec<[(LanguageServerId, DiagnosticSet); 2]>,
    diagnostics_update_count: usize,
    file_update_count: usize,
    git_diff_update_count: usize,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    selections_update_count: usize,
    language: Option<Arc<Language>>,
    parse_count: usize,
}

/// The kind and amount of indentation in a particular line. For now,
/// assumes that indentation is all the same character.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IndentSize {
    /// The number of bytes that comprise the indentation.
    pub len: u32,
    /// The kind of whitespace used for indentation.
    pub kind: IndentKind,
}

/// A whitespace character that's used for indentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IndentKind {
    /// An ASCII space character.
    #[default]
    Space,
    /// An ASCII tab character.
    Tab,
}

/// The shape of a selection cursor.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum CursorShape {
    /// A vertical bar
    #[default]
    Bar,
    /// A block that surrounds the following character
    Block,
    /// An underline that runs along the following character
    Underscore,
    /// A box drawn around the following character
    Hollow,
}

#[derive(Clone, Debug)]
struct SelectionSet {
    line_mode: bool,
    cursor_shape: CursorShape,
    selections: Arc<[Selection<Anchor>]>,
    lamport_timestamp: clock::Lamport,
}

/// A diagnostic associated with a certain range of a buffer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// The name of the service that produced this diagnostic.
    pub source: Option<String>,
    /// A machine-readable code that identifies this diagnostic.
    pub code: Option<String>,
    /// Whether this diagnostic is a hint, warning, or error.
    pub severity: DiagnosticSeverity,
    /// The human-readable message associated with this diagnostic.
    pub message: String,
    /// An id that identifies the group to which this diagnostic belongs.
    ///
    /// When a language server produces a diagnostic with
    /// one or more associated diagnostics, those diagnostics are all
    /// assigned a single group id.
    pub group_id: usize,
    /// Whether this diagnostic is the primary diagnostic for its group.
    ///
    /// In a given group, the primary diagnostic is the top-level diagnostic
    /// returned by the language server. The non-primary diagnostics are the
    /// associated diagnostics.
    pub is_primary: bool,
    /// Whether this diagnostic is considered to originate from an analysis of
    /// files on disk, as opposed to any unsaved buffer contents. This is a
    /// property of a given diagnostic source, and is configured for a given
    /// language server via the [`LspAdapter::disk_based_diagnostic_sources`](crate::LspAdapter::disk_based_diagnostic_sources) method
    /// for the language server.
    pub is_disk_based: bool,
    /// Whether this diagnostic marks unnecessary code.
    pub is_unnecessary: bool,
}

/// TODO - move this into the `project` crate and make it private.
pub async fn prepare_completion_documentation(
    documentation: &lsp::Documentation,
    language_registry: &Arc<LanguageRegistry>,
    language: Option<Arc<Language>>,
) -> Documentation {
    match documentation {
        lsp::Documentation::String(text) => {
            if text.lines().count() <= 1 {
                Documentation::SingleLine(text.clone())
            } else {
                Documentation::MultiLinePlainText(text.clone())
            }
        }

        lsp::Documentation::MarkupContent(lsp::MarkupContent { kind, value }) => match kind {
            lsp::MarkupKind::PlainText => {
                if value.lines().count() <= 1 {
                    Documentation::SingleLine(value.clone())
                } else {
                    Documentation::MultiLinePlainText(value.clone())
                }
            }

            lsp::MarkupKind::Markdown => {
                let parsed = parse_markdown(value, language_registry, language).await;
                Documentation::MultiLineMarkdown(parsed)
            }
        },
    }
}

/// Documentation associated with a [`Completion`].
#[derive(Clone, Debug)]
pub enum Documentation {
    /// There is no documentation for this completion.
    Undocumented,
    /// A single line of documentation.
    SingleLine(String),
    /// Multiple lines of plain text documentation.
    MultiLinePlainText(String),
    /// Markdown documentation.
    MultiLineMarkdown(ParsedMarkdown),
}

/// An operation used to synchronize this buffer with its other replicas.
#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    /// A text operation.
    Buffer(text::Operation),

    /// An update to the buffer's diagnostics.
    UpdateDiagnostics {
        /// The id of the language server that produced the new diagnostics.
        server_id: LanguageServerId,
        /// The diagnostics.
        diagnostics: Arc<[DiagnosticEntry<Anchor>]>,
        /// The buffer's lamport timestamp.
        lamport_timestamp: clock::Lamport,
    },

    /// An update to the most recent selections in this buffer.
    UpdateSelections {
        /// The selections.
        selections: Arc<[Selection<Anchor>]>,
        /// The buffer's lamport timestamp.
        lamport_timestamp: clock::Lamport,
        /// Whether the selections are in 'line mode'.
        line_mode: bool,
        /// The [`CursorShape`] associated with these selections.
        cursor_shape: CursorShape,
    },

    /// An update to the characters that should trigger autocompletion
    /// for this buffer.
    UpdateCompletionTriggers {
        /// The characters that trigger autocompletion.
        triggers: Vec<String>,
        /// The buffer's lamport timestamp.
        lamport_timestamp: clock::Lamport,
    },
}

/// An event that occurs in a buffer.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// The buffer was changed in a way that must be
    /// propagated to its other replicas.
    Operation(Operation),
    /// The buffer was edited.
    Edited,
    /// The buffer's `dirty` bit changed.
    DirtyChanged,
    /// The buffer was saved.
    Saved,
    /// The buffer's file was changed on disk.
    FileHandleChanged,
    /// The buffer was reloaded.
    Reloaded,
    /// The buffer's diff_base changed.
    DiffBaseChanged,
    /// Buffer's excerpts for a certain diff base were recalculated.
    DiffUpdated,
    /// The buffer's language was changed.
    LanguageChanged,
    /// The buffer's syntax trees were updated.
    Reparsed,
    /// The buffer's diagnostics were updated.
    DiagnosticsUpdated,
    /// The buffer gained or lost editing capabilities.
    CapabilityChanged,
    /// The buffer was explicitly requested to close.
    Closed,
}

/// The file associated with a buffer.
pub trait File: Send + Sync {
    /// Returns the [`LocalFile`] associated with this file, if the
    /// file is local.
    fn as_local(&self) -> Option<&dyn LocalFile>;

    /// Returns whether this file is local.
    fn is_local(&self) -> bool {
        self.as_local().is_some()
    }

    /// Returns the file's mtime.
    fn mtime(&self) -> Option<SystemTime>;

    /// Returns the path of this file relative to the worktree's root directory.
    fn path(&self) -> &Arc<Path>;

    /// Returns the path of this file relative to the worktree's parent directory (this means it
    /// includes the name of the worktree's root folder).
    fn full_path(&self, cx: &AppContext) -> PathBuf;

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name<'a>(&'a self, cx: &'a AppContext) -> &'a OsStr;

    /// Returns the id of the worktree to which this file belongs.
    ///
    /// This is needed for looking up project-specific settings.
    fn worktree_id(&self) -> usize;

    /// Returns whether the file has been deleted.
    fn is_deleted(&self) -> bool;

    /// Returns whether the file existed on disk at one point
    fn is_created(&self) -> bool {
        self.mtime().is_some()
    }

    /// Converts this file into an [`Any`] trait object.
    fn as_any(&self) -> &dyn Any;

    /// Converts this file into a protobuf message.
    fn to_proto(&self) -> rpc::proto::File;

    /// Return whether Zed considers this to be a private file.
    fn is_private(&self) -> bool;
}

/// The file associated with a buffer, in the case where the file is on the local disk.
pub trait LocalFile: File {
    /// Returns the absolute path of this file.
    fn abs_path(&self, cx: &AppContext) -> PathBuf;

    /// Loads the file's contents from disk.
    fn load(&self, cx: &AppContext) -> Task<Result<String>>;

    /// Called when the buffer is reloaded from disk.
    fn buffer_reloaded(
        &self,
        buffer_id: BufferId,
        version: &clock::Global,
        line_ending: LineEnding,
        mtime: Option<SystemTime>,
        cx: &mut AppContext,
    );

    /// Returns true if the file should not be shared with collaborators.
    fn is_private(&self, _: &AppContext) -> bool {
        false
    }
}

/// The auto-indent behavior associated with an editing operation.
/// For some editing operations, each affected line of text has its
/// indentation recomputed. For other operations, the entire block
/// of edited text is adjusted uniformly.
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

/// An iterator that yields chunks of a buffer's text, along with their
/// syntax highlights and diagnostic status.
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

/// A chunk of a buffer's text, along with its syntax highlight and
/// diagnostic status.
#[derive(Clone, Debug, Default)]
pub struct Chunk<'a> {
    /// The text of the chunk.
    pub text: &'a str,
    /// The syntax highlighting style of the chunk.
    pub syntax_highlight_id: Option<HighlightId>,
    /// The highlight style that has been applied to this chunk in
    /// the editor.
    pub highlight_style: Option<HighlightStyle>,
    /// The severity of diagnostic associated with this chunk, if any.
    pub diagnostic_severity: Option<DiagnosticSeverity>,
    /// Whether this chunk of text is marked as unnecessary.
    pub is_unnecessary: bool,
    /// Whether this chunk of text was originally a tab character.
    pub is_tab: bool,
    /// An optional recipe for how the chunk should be presented.
    pub renderer: Option<ChunkRenderer>,
}

/// A recipe for how the chunk should be presented.
#[derive(Clone)]
pub struct ChunkRenderer {
    /// creates a custom element to represent this chunk.
    pub render: Arc<dyn Send + Sync + Fn(&mut WindowContext) -> AnyElement>,
    /// If true, the element is constrained to the shaped width of the text.
    pub constrain_width: bool,
}

impl fmt::Debug for ChunkRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ChunkRenderer")
            .field("constrain_width", &self.constrain_width)
            .finish()
    }
}

/// A set of edits to a given version of a buffer, computed asynchronously.
#[derive(Debug)]
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

/// A class of characters, used for characterizing a run of text.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum CharKind {
    /// Whitespace.
    Whitespace,
    /// Punctuation.
    Punctuation,
    /// Word.
    Word,
}

/// A runnable is a set of data about a region that could be resolved into a task
pub struct Runnable {
    pub tags: SmallVec<[RunnableTag; 1]>,
    pub language: Arc<Language>,
    pub buffer: BufferId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndentGuide {
    pub buffer_id: BufferId,
    pub start_row: BufferRow,
    pub end_row: BufferRow,
    pub depth: u32,
    pub tab_size: u32,
}

impl IndentGuide {
    pub fn new(
        buffer_id: BufferId,
        start_row: BufferRow,
        end_row: BufferRow,
        depth: u32,
        tab_size: u32,
    ) -> Self {
        Self {
            buffer_id,
            start_row,
            end_row,
            depth,
            tab_size,
        }
    }

    pub fn indent_level(&self) -> u32 {
        self.depth * self.tab_size
    }
}

impl Buffer {
    /// Create a new buffer with the given base text.
    pub fn local<T: Into<String>>(base_text: T, cx: &mut ModelContext<Self>) -> Self {
        Self::build(
            TextBuffer::new(0, cx.entity_id().as_non_zero_u64().into(), base_text.into()),
            None,
            None,
            Capability::ReadWrite,
        )
    }

    /// Create a new buffer with the given base text that has proper line endings and other normalization applied.
    pub fn local_normalized(
        base_text_normalized: Rope,
        line_ending: LineEnding,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self::build(
            TextBuffer::new_normalized(
                0,
                cx.entity_id().as_non_zero_u64().into(),
                line_ending,
                base_text_normalized,
            ),
            None,
            None,
            Capability::ReadWrite,
        )
    }

    /// Create a new buffer that is a replica of a remote buffer.
    pub fn remote(
        remote_id: BufferId,
        replica_id: ReplicaId,
        capability: Capability,
        base_text: impl Into<String>,
    ) -> Self {
        Self::build(
            TextBuffer::new(replica_id, remote_id, base_text.into()),
            None,
            None,
            capability,
        )
    }

    /// Create a new buffer that is a replica of a remote buffer, populating its
    /// state from the given protobuf message.
    pub fn from_proto(
        replica_id: ReplicaId,
        capability: Capability,
        message: proto::BufferState,
        file: Option<Arc<dyn File>>,
    ) -> Result<Self> {
        let buffer_id = BufferId::new(message.id)
            .with_context(|| anyhow!("Could not deserialize buffer_id"))?;
        let buffer = TextBuffer::new(replica_id, buffer_id, message.base_text);
        let mut this = Self::build(buffer, message.diff_base, file, capability);
        this.text.set_line_ending(proto::deserialize_line_ending(
            rpc::proto::LineEnding::from_i32(message.line_ending)
                .ok_or_else(|| anyhow!("missing line_ending"))?,
        ));
        this.saved_version = proto::deserialize_version(&message.saved_version);
        this.saved_mtime = message.saved_mtime.map(|time| time.into());
        Ok(this)
    }

    /// Serialize the buffer's state to a protobuf message.
    pub fn to_proto(&self) -> proto::BufferState {
        proto::BufferState {
            id: self.remote_id().into(),
            file: self.file.as_ref().map(|f| f.to_proto()),
            base_text: self.base_text().to_string(),
            diff_base: self.diff_base.as_ref().map(|h| h.to_string()),
            line_ending: proto::serialize_line_ending(self.line_ending()) as i32,
            saved_version: proto::serialize_version(&self.saved_version),
            saved_mtime: self.saved_mtime.map(|time| time.into()),
        }
    }

    /// Serialize as protobufs all of the changes to the buffer since the given version.
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

        for (server_id, diagnostics) in &self.diagnostics {
            operations.push(proto::serialize_operation(&Operation::UpdateDiagnostics {
                lamport_timestamp: self.diagnostics_timestamp,
                server_id: *server_id,
                diagnostics: diagnostics.iter().cloned().collect(),
            }));
        }

        operations.push(proto::serialize_operation(
            &Operation::UpdateCompletionTriggers {
                triggers: self.completion_triggers.clone(),
                lamport_timestamp: self.completion_triggers_timestamp,
            },
        ));

        let text_operations = self.text.operations().clone();
        cx.background_executor().spawn(async move {
            let since = since.unwrap_or_default();
            operations.extend(
                text_operations
                    .iter()
                    .filter(|(_, op)| !since.observed(op.timestamp()))
                    .map(|(_, op)| proto::serialize_operation(&Operation::Buffer(op.clone()))),
            );
            operations.sort_unstable_by_key(proto::lamport_timestamp_for_operation);
            operations
        })
    }

    /// Assign a language to the buffer, returning the buffer.
    pub fn with_language(mut self, language: Arc<Language>, cx: &mut ModelContext<Self>) -> Self {
        self.set_language(Some(language), cx);
        self
    }

    /// Returns the [Capability] of this buffer.
    pub fn capability(&self) -> Capability {
        self.capability
    }

    /// Whether this buffer can only be read.
    pub fn read_only(&self) -> bool {
        self.capability == Capability::ReadOnly
    }

    /// Builds a [Buffer] with the given underlying [TextBuffer], diff base, [File] and [Capability].
    pub fn build(
        buffer: TextBuffer,
        diff_base: Option<String>,
        file: Option<Arc<dyn File>>,
        capability: Capability,
    ) -> Self {
        let saved_mtime = file.as_ref().and_then(|file| file.mtime());

        Self {
            saved_mtime,
            saved_version: buffer.version(),
            reload_task: None,
            transaction_depth: 0,
            was_dirty_before_starting_transaction: None,
            text: buffer,
            diff_base: diff_base
                .map(|mut raw_diff_base| {
                    LineEnding::normalize(&mut raw_diff_base);
                    raw_diff_base
                })
                .map(Rope::from),
            diff_base_version: 0,
            git_diff: git::diff::BufferDiff::new(),
            file,
            capability,
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
            has_conflict: false,
        }
    }

    /// Retrieve a snapshot of the buffer's current state. This is computationally
    /// cheap, and allows reading from the buffer on a background thread.
    pub fn snapshot(&self) -> BufferSnapshot {
        let text = self.text.snapshot();
        let mut syntax_map = self.syntax_map.lock();
        syntax_map.interpolate(&text);
        let syntax = syntax_map.snapshot();

        BufferSnapshot {
            text,
            syntax,
            git_diff: self.git_diff.clone(),
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

    #[cfg(test)]
    pub(crate) fn as_text_snapshot(&self) -> &text::BufferSnapshot {
        &self.text
    }

    /// Retrieve a snapshot of the buffer's raw text, without any
    /// language-related state like the syntax tree or diagnostics.
    pub fn text_snapshot(&self) -> text::BufferSnapshot {
        self.text.snapshot()
    }

    /// The file associated with the buffer, if any.
    pub fn file(&self) -> Option<&Arc<dyn File>> {
        self.file.as_ref()
    }

    /// The version of the buffer that was last saved or reloaded from disk.
    pub fn saved_version(&self) -> &clock::Global {
        &self.saved_version
    }

    /// The mtime of the buffer's file when the buffer was last saved or reloaded from disk.
    pub fn saved_mtime(&self) -> Option<SystemTime> {
        self.saved_mtime
    }

    /// Assign a language to the buffer.
    pub fn set_language(&mut self, language: Option<Arc<Language>>, cx: &mut ModelContext<Self>) {
        self.parse_count += 1;
        self.syntax_map.lock().clear();
        self.language = language;
        self.reparse(cx);
        cx.emit(Event::LanguageChanged);
    }

    /// Assign a language registry to the buffer. This allows the buffer to retrieve
    /// other languages if parts of the buffer are written in different languages.
    pub fn set_language_registry(&mut self, language_registry: Arc<LanguageRegistry>) {
        self.syntax_map
            .lock()
            .set_language_registry(language_registry);
    }

    /// Assign the buffer a new [Capability].
    pub fn set_capability(&mut self, capability: Capability, cx: &mut ModelContext<Self>) {
        self.capability = capability;
        cx.emit(Event::CapabilityChanged)
    }

    /// This method is called to signal that the buffer has been saved.
    pub fn did_save(
        &mut self,
        version: clock::Global,
        mtime: Option<SystemTime>,
        cx: &mut ModelContext<Self>,
    ) {
        self.saved_version = version;
        self.has_conflict = false;
        self.saved_mtime = mtime;
        cx.emit(Event::Saved);
        cx.notify();
    }

    /// Reloads the contents of the buffer from disk.
    pub fn reload(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> oneshot::Receiver<Option<Transaction>> {
        let (tx, rx) = futures::channel::oneshot::channel();
        let prev_version = self.text.version();
        self.reload_task = Some(cx.spawn(|this, mut cx| async move {
            let Some((new_mtime, new_text)) = this.update(&mut cx, |this, cx| {
                let file = this.file.as_ref()?.as_local()?;
                Some((file.mtime(), file.load(cx)))
            })?
            else {
                return Ok(());
            };

            let new_text = new_text.await?;
            let diff = this
                .update(&mut cx, |this, cx| this.diff(new_text.clone(), cx))?
                .await;
            this.update(&mut cx, |this, cx| {
                if this.version() == diff.base_version {
                    this.finalize_last_transaction();
                    this.apply_diff(diff, cx);
                    tx.send(this.finalize_last_transaction().cloned()).ok();
                    this.has_conflict = false;
                    this.did_reload(this.version(), this.line_ending(), new_mtime, cx);
                } else {
                    if !diff.edits.is_empty()
                        || this
                            .edits_since::<usize>(&diff.base_version)
                            .next()
                            .is_some()
                    {
                        this.has_conflict = true;
                    }

                    this.did_reload(prev_version, this.line_ending(), this.saved_mtime, cx);
                }

                this.reload_task.take();
            })
        }));
        rx
    }

    /// This method is called to signal that the buffer has been reloaded.
    pub fn did_reload(
        &mut self,
        version: clock::Global,
        line_ending: LineEnding,
        mtime: Option<SystemTime>,
        cx: &mut ModelContext<Self>,
    ) {
        self.saved_version = version;
        self.text.set_line_ending(line_ending);
        self.saved_mtime = mtime;
        if let Some(file) = self.file.as_ref().and_then(|f| f.as_local()) {
            file.buffer_reloaded(
                self.remote_id(),
                &self.saved_version,
                self.line_ending(),
                self.saved_mtime,
                cx,
            );
        }
        cx.emit(Event::Reloaded);
        cx.notify();
    }

    /// Updates the [File] backing this buffer. This should be called when
    /// the file has changed or has been deleted.
    pub fn file_updated(&mut self, new_file: Arc<dyn File>, cx: &mut ModelContext<Self>) {
        let mut file_changed = false;

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
                        self.reload(cx).close();
                    }
                }
            }
        } else {
            file_changed = true;
        };

        self.file = Some(new_file);
        if file_changed {
            self.file_update_count += 1;
            cx.emit(Event::FileHandleChanged);
            cx.notify();
        }
    }

    /// Returns the current diff base, see [Buffer::set_diff_base].
    pub fn diff_base(&self) -> Option<&Rope> {
        self.diff_base.as_ref()
    }

    /// Sets the text that will be used to compute a Git diff
    /// against the buffer text.
    pub fn set_diff_base(&mut self, diff_base: Option<String>, cx: &mut ModelContext<Self>) {
        self.diff_base = diff_base
            .map(|mut raw_diff_base| {
                LineEnding::normalize(&mut raw_diff_base);
                raw_diff_base
            })
            .map(Rope::from);
        self.diff_base_version += 1;
        if let Some(recalc_task) = self.git_diff_recalc(cx) {
            cx.spawn(|buffer, mut cx| async move {
                recalc_task.await;
                buffer
                    .update(&mut cx, |_, cx| {
                        cx.emit(Event::DiffBaseChanged);
                    })
                    .ok();
            })
            .detach();
        }
    }

    /// Returns a number, unique per diff base set to the buffer.
    pub fn diff_base_version(&self) -> usize {
        self.diff_base_version
    }

    /// Recomputes the Git diff status.
    pub fn git_diff_recalc(&mut self, cx: &mut ModelContext<Self>) -> Option<Task<()>> {
        let diff_base = self.diff_base.clone()?;
        let snapshot = self.snapshot();

        let mut diff = self.git_diff.clone();
        let diff = cx.background_executor().spawn(async move {
            diff.update(&diff_base, &snapshot).await;
            diff
        });

        Some(cx.spawn(|this, mut cx| async move {
            let buffer_diff = diff.await;
            this.update(&mut cx, |this, cx| {
                this.git_diff = buffer_diff;
                this.git_diff_update_count += 1;
                cx.emit(Event::DiffUpdated);
            })
            .ok();
        }))
    }

    /// Returns the primary [Language] assigned to this [Buffer].
    pub fn language(&self) -> Option<&Arc<Language>> {
        self.language.as_ref()
    }

    /// Returns the [Language] at the given location.
    pub fn language_at<D: ToOffset>(&self, position: D) -> Option<Arc<Language>> {
        let offset = position.to_offset(self);
        self.syntax_map
            .lock()
            .layers_for_range(offset..offset, &self.text)
            .last()
            .map(|info| info.language.clone())
            .or_else(|| self.language.clone())
    }

    /// The number of times the buffer was parsed.
    pub fn parse_count(&self) -> usize {
        self.parse_count
    }

    /// The number of times selections were updated.
    pub fn selections_update_count(&self) -> usize {
        self.selections_update_count
    }

    /// The number of times diagnostics were updated.
    pub fn diagnostics_update_count(&self) -> usize {
        self.diagnostics_update_count
    }

    /// The number of times the underlying file was updated.
    pub fn file_update_count(&self) -> usize {
        self.file_update_count
    }

    /// The number of times the git diff status was updated.
    pub fn git_diff_update_count(&self) -> usize {
        self.git_diff_update_count
    }

    /// Whether the buffer is being parsed in the background.
    #[cfg(any(test, feature = "test-support"))]
    pub fn is_parsing(&self) -> bool {
        self.parsing_in_background
    }

    /// Indicates whether the buffer contains any regions that may be
    /// written in a language that hasn't been loaded yet.
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

        let parse_task = cx.background_executor().spawn({
            let language = language.clone();
            let language_registry = language_registry.clone();
            async move {
                syntax_snapshot.reparse(&text, language_registry, language);
                syntax_snapshot
            }
        });

        match cx
            .background_executor()
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
                    })
                    .ok();
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

    /// Assign to the buffer a set of diagnostics created by a given language server.
    pub fn update_diagnostics(
        &mut self,
        server_id: LanguageServerId,
        diagnostics: DiagnosticSet,
        cx: &mut ModelContext<Self>,
    ) {
        let lamport_timestamp = self.text.lamport_clock.tick();
        let op = Operation::UpdateDiagnostics {
            server_id,
            diagnostics: diagnostics.iter().cloned().collect(),
            lamport_timestamp,
        };
        self.apply_diagnostic_update(server_id, diagnostics, lamport_timestamp, cx);
        self.send_operation(op, cx);
    }

    fn request_autoindent(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(indent_sizes) = self.compute_autoindents() {
            let indent_sizes = cx.background_executor().spawn(indent_sizes);
            match cx
                .background_executor()
                .block_with_timeout(Duration::from_micros(500), indent_sizes)
            {
                Ok(indent_sizes) => self.apply_autoindents(indent_sizes, cx),
                Err(indent_sizes) => {
                    self.pending_autoindent = Some(cx.spawn(|this, mut cx| async move {
                        let indent_sizes = indent_sizes.await;
                        this.update(&mut cx, |this, cx| {
                            this.apply_autoindents(indent_sizes, cx);
                        })
                        .ok();
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

    /// Create a minimal edit that will cause the given row to be indented
    /// with the given size. After applying this edit, the length of the line
    /// will always be at least `new_size.len`.
    pub fn edit_for_indent_size_adjustment(
        row: u32,
        current_size: IndentSize,
        new_size: IndentSize,
    ) -> Option<(Range<Point>, String)> {
        if new_size.kind == current_size.kind {
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
        } else {
            Some((
                Point::new(row, 0)..Point::new(row, current_size.len),
                iter::repeat(new_size.char())
                    .take(new_size.len as usize)
                    .collect::<String>(),
            ))
        }
    }

    /// Spawns a background task that asynchronously computes a `Diff` between the buffer's text
    /// and the given new text.
    pub fn diff(&self, mut new_text: String, cx: &AppContext) -> Task<Diff> {
        let old_text = self.as_rope().clone();
        let base_version = self.version();
        cx.background_executor()
            .spawn_labeled(*BUFFER_DIFF_TASK, async move {
                let old_text = old_text.to_string();
                let line_ending = LineEnding::detect(&new_text);
                LineEnding::normalize(&mut new_text);

                let diff = TextDiff::from_chars(old_text.as_str(), new_text.as_str());
                let empty: Arc<str> = "".into();

                let mut edits = Vec::new();
                let mut old_offset = 0;
                let mut new_offset = 0;
                let mut last_edit: Option<(Range<usize>, Range<usize>)> = None;
                for change in diff.iter_all_changes().map(Some).chain([None]) {
                    if let Some(change) = &change {
                        let len = change.value().len();
                        match change.tag() {
                            ChangeTag::Equal => {
                                old_offset += len;
                                new_offset += len;
                            }
                            ChangeTag::Delete => {
                                let old_end_offset = old_offset + len;
                                if let Some((last_old_range, _)) = &mut last_edit {
                                    last_old_range.end = old_end_offset;
                                } else {
                                    last_edit =
                                        Some((old_offset..old_end_offset, new_offset..new_offset));
                                }
                                old_offset = old_end_offset;
                            }
                            ChangeTag::Insert => {
                                let new_end_offset = new_offset + len;
                                if let Some((_, last_new_range)) = &mut last_edit {
                                    last_new_range.end = new_end_offset;
                                } else {
                                    last_edit =
                                        Some((old_offset..old_offset, new_offset..new_end_offset));
                                }
                                new_offset = new_end_offset;
                            }
                        }
                    }

                    if let Some((old_range, new_range)) = &last_edit {
                        if old_offset > old_range.end
                            || new_offset > new_range.end
                            || change.is_none()
                        {
                            let text = if new_range.is_empty() {
                                empty.clone()
                            } else {
                                new_text[new_range.clone()].into()
                            };
                            edits.push((old_range.clone(), text));
                            last_edit.take();
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

    /// Spawns a background task that searches the buffer for any whitespace
    /// at the ends of a lines, and returns a `Diff` that removes that whitespace.
    pub fn remove_trailing_whitespace(&self, cx: &AppContext) -> Task<Diff> {
        let old_text = self.as_rope().clone();
        let line_ending = self.line_ending();
        let base_version = self.version();
        cx.background_executor().spawn(async move {
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

    /// Ensures that the buffer ends with a single newline character, and
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

    /// Applies a diff to the buffer. If the buffer has changed since the given diff was
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

    /// Checks if the buffer has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.has_conflict
            || self.has_edits_since(&self.saved_version)
            || self
                .file
                .as_ref()
                .map_or(false, |file| file.is_deleted() || !file.is_created())
    }

    /// Checks if the buffer and its file have both changed since the buffer
    /// was last saved or reloaded.
    pub fn has_conflict(&self) -> bool {
        self.has_conflict
            || self.file.as_ref().map_or(false, |file| {
                file.mtime() > self.saved_mtime && self.has_edits_since(&self.saved_version)
            })
    }

    /// Gets a [`Subscription`] that tracks all of the changes to the buffer's text.
    pub fn subscribe(&mut self) -> Subscription {
        self.text.subscribe()
    }

    /// Starts a transaction, if one is not already in-progress. When undoing or
    /// redoing edits, all of the edits performed within a transaction are undone
    /// or redone together.
    pub fn start_transaction(&mut self) -> Option<TransactionId> {
        self.start_transaction_at(Instant::now())
    }

    /// Starts a transaction, providing the current time. Subsequent transactions
    /// that occur within a short period of time will be grouped together. This
    /// is controlled by the buffer's undo grouping duration.
    pub fn start_transaction_at(&mut self, now: Instant) -> Option<TransactionId> {
        self.transaction_depth += 1;
        if self.was_dirty_before_starting_transaction.is_none() {
            self.was_dirty_before_starting_transaction = Some(self.is_dirty());
        }
        self.text.start_transaction_at(now)
    }

    /// Terminates the current transaction, if this is the outermost transaction.
    pub fn end_transaction(&mut self, cx: &mut ModelContext<Self>) -> Option<TransactionId> {
        self.end_transaction_at(Instant::now(), cx)
    }

    /// Terminates the current transaction, providing the current time. Subsequent transactions
    /// that occur within a short period of time will be grouped together. This
    /// is controlled by the buffer's undo grouping duration.
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

    /// Manually add a transaction to the buffer's undo history.
    pub fn push_transaction(&mut self, transaction: Transaction, now: Instant) {
        self.text.push_transaction(transaction, now);
    }

    /// Prevent the last transaction from being grouped with any subsequent transactions,
    /// even if they occur with the buffer's undo grouping duration.
    pub fn finalize_last_transaction(&mut self) -> Option<&Transaction> {
        self.text.finalize_last_transaction()
    }

    /// Manually group all changes since a given transaction.
    pub fn group_until_transaction(&mut self, transaction_id: TransactionId) {
        self.text.group_until_transaction(transaction_id);
    }

    /// Manually remove a transaction from the buffer's undo history
    pub fn forget_transaction(&mut self, transaction_id: TransactionId) {
        self.text.forget_transaction(transaction_id);
    }

    /// Manually merge two adjacent transactions in the buffer's undo history.
    pub fn merge_transactions(&mut self, transaction: TransactionId, destination: TransactionId) {
        self.text.merge_transactions(transaction, destination);
    }

    /// Waits for the buffer to receive operations with the given timestamps.
    pub fn wait_for_edits(
        &mut self,
        edit_ids: impl IntoIterator<Item = clock::Lamport>,
    ) -> impl Future<Output = Result<()>> {
        self.text.wait_for_edits(edit_ids)
    }

    /// Waits for the buffer to receive the operations necessary for resolving the given anchors.
    pub fn wait_for_anchors(
        &mut self,
        anchors: impl IntoIterator<Item = Anchor>,
    ) -> impl 'static + Future<Output = Result<()>> {
        self.text.wait_for_anchors(anchors)
    }

    /// Waits for the buffer to receive operations up to the given version.
    pub fn wait_for_version(&mut self, version: clock::Global) -> impl Future<Output = Result<()>> {
        self.text.wait_for_version(version)
    }

    /// Forces all futures returned by [`Buffer::wait_for_version`], [`Buffer::wait_for_edits`], or
    /// [`Buffer::wait_for_version`] to resolve with an error.
    pub fn give_up_waiting(&mut self) {
        self.text.give_up_waiting();
    }

    /// Stores a set of selections that should be broadcasted to all of the buffer's replicas.
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

    /// Clears the selections, so that other replicas of the buffer do not see any selections for
    /// this replica.
    pub fn remove_active_selections(&mut self, cx: &mut ModelContext<Self>) {
        if self
            .remote_selections
            .get(&self.text.replica_id())
            .map_or(true, |set| !set.selections.is_empty())
        {
            self.set_active_selections(Arc::from([]), false, Default::default(), cx);
        }
    }

    /// Replaces the buffer's entire text.
    pub fn set_text<T>(&mut self, text: T, cx: &mut ModelContext<Self>) -> Option<clock::Lamport>
    where
        T: Into<Arc<str>>,
    {
        self.autoindent_requests.clear();
        self.edit([(0..self.len(), text)], None, cx)
    }

    /// Applies the given edits to the buffer. Each edit is specified as a range of text to
    /// delete, and a string of text to insert at that location.
    ///
    /// If an [`AutoindentMode`] is provided, then the buffer will enqueue an auto-indent
    /// request for the edited ranges, which will be processed when the buffer finishes
    /// parsing.
    ///
    /// Parsing takes place at the end of a transaction, and may compute synchronously
    /// or asynchronously, depending on the changes.
    pub fn edit<I, S, T>(
        &mut self,
        edits_iter: I,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut ModelContext<Self>,
    ) -> Option<clock::Lamport>
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
        let edit_id = edit_operation.timestamp();

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

    /// Applies the given remote operations to the buffer.
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
                server_id,
                diagnostics: diagnostic_set,
                lamport_timestamp,
            } => {
                let snapshot = self.snapshot();
                self.apply_diagnostic_update(
                    server_id,
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
        server_id: LanguageServerId,
        diagnostics: DiagnosticSet,
        lamport_timestamp: clock::Lamport,
        cx: &mut ModelContext<Self>,
    ) {
        if lamport_timestamp > self.diagnostics_timestamp {
            let ix = self.diagnostics.binary_search_by_key(&server_id, |e| e.0);
            if diagnostics.len() == 0 {
                if let Ok(ix) = ix {
                    self.diagnostics.remove(ix);
                }
            } else {
                match ix {
                    Err(ix) => self.diagnostics.insert(ix, (server_id, diagnostics)),
                    Ok(ix) => self.diagnostics[ix].1 = diagnostics,
                };
            }
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

    /// Removes the selections for a given peer.
    pub fn remove_peer(&mut self, replica_id: ReplicaId, cx: &mut ModelContext<Self>) {
        self.remote_selections.remove(&replica_id);
        cx.notify();
    }

    /// Undoes the most recent transaction.
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

    /// Manually undoes a specific transaction in the buffer's undo history.
    pub fn undo_transaction(
        &mut self,
        transaction_id: TransactionId,
        cx: &mut ModelContext<Self>,
    ) -> bool {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();
        if let Some(operation) = self.text.undo_transaction(transaction_id) {
            self.send_operation(Operation::Buffer(operation), cx);
            self.did_edit(&old_version, was_dirty, cx);
            true
        } else {
            false
        }
    }

    /// Manually undoes all changes after a given transaction in the buffer's undo history.
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

    /// Manually redoes a specific transaction in the buffer's redo history.
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

    /// Manually undoes all changes until a given transaction in the buffer's redo history.
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

    /// Override current completion triggers with the user-provided completion triggers.
    pub fn set_completion_triggers(&mut self, triggers: Vec<String>, cx: &mut ModelContext<Self>) {
        self.completion_triggers.clone_from(&triggers);
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

    /// Returns a list of strings which trigger a completion menu for this language.
    /// Usually this is driven by LSP server which returns a list of trigger characters for completions.
    pub fn completion_triggers(&self) -> &[String] {
        &self.completion_triggers
    }
}

#[doc(hidden)]
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

impl EventEmitter<Event> for Buffer {}

impl Deref for Buffer {
    type Target = TextBuffer;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl BufferSnapshot {
    /// Returns [`IndentSize`] for a given line that respects user settings and /// language preferences.
    pub fn indent_size_for_line(&self, row: u32) -> IndentSize {
        indent_size_for_line(self, row)
    }
    /// Returns [`IndentSize`] for a given position that respects user settings
    /// and language preferences.
    pub fn language_indent_size_at<T: ToOffset>(&self, position: T, cx: &AppContext) -> IndentSize {
        let settings = language_settings(self.language_at(position), self.file(), cx);
        if settings.hard_tabs {
            IndentSize::tab()
        } else {
            IndentSize::spaces(settings.tab_size.get())
        }
    }

    /// Retrieve the suggested indent size for all of the given rows. The unit of indentation
    /// is passed in as `single_indent_size`.
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

    /// Iterates over chunks of text in the given range of the buffer. Text is chunked
    /// in an arbitrary way due to being stored in a [`Rope`](text::Rope). The text is also
    /// returned in chunks where each chunk has a single syntax highlighting style and
    /// diagnostic status.
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

    /// Invokes the given callback for each line of text in the given range of the buffer.
    /// Uses callback to avoid allocating a string for each line.
    fn for_each_line(&self, range: Range<Point>, mut callback: impl FnMut(u32, &str)) {
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

    /// Iterates over every [`SyntaxLayer`] in the buffer.
    pub fn syntax_layers(&self) -> impl Iterator<Item = SyntaxLayer> + '_ {
        self.syntax.layers_for_range(0..self.len(), &self.text)
    }

    pub fn syntax_layer_at<D: ToOffset>(&self, position: D) -> Option<SyntaxLayer> {
        let offset = position.to_offset(self);
        self.syntax
            .layers_for_range(offset..offset, &self.text)
            .filter(|l| l.node().end_byte() > offset)
            .last()
    }

    /// Returns the main [Language]
    pub fn language(&self) -> Option<&Arc<Language>> {
        self.language.as_ref()
    }

    /// Returns the [Language] at the given location.
    pub fn language_at<D: ToOffset>(&self, position: D) -> Option<&Arc<Language>> {
        self.syntax_layer_at(position)
            .map(|info| info.language)
            .or(self.language.as_ref())
    }

    /// Returns the settings for the language at the given location.
    pub fn settings_at<'a, D: ToOffset>(
        &self,
        position: D,
        cx: &'a AppContext,
    ) -> &'a LanguageSettings {
        language_settings(self.language_at(position), self.file.as_ref(), cx)
    }

    /// Returns the [LanguageScope] at the given location.
    pub fn language_scope_at<D: ToOffset>(&self, position: D) -> Option<LanguageScope> {
        let offset = position.to_offset(self);
        let mut scope = None;
        let mut smallest_range: Option<Range<usize>> = None;

        // Use the layer that has the smallest node intersecting the given point.
        for layer in self.syntax.layers_for_range(offset..offset, &self.text) {
            let mut cursor = layer.node().walk();

            let mut range = None;
            loop {
                let child_range = cursor.node().byte_range();
                if !child_range.to_inclusive().contains(&offset) {
                    break;
                }

                range = Some(child_range);
                if cursor.goto_first_child_for_byte(offset).is_none() {
                    break;
                }
            }

            if let Some(range) = range {
                if smallest_range
                    .as_ref()
                    .map_or(true, |smallest_range| range.len() < smallest_range.len())
                {
                    smallest_range = Some(range);
                    scope = Some(LanguageScope {
                        language: layer.language.clone(),
                        override_id: layer.override_id(offset, &self.text),
                    });
                }
            }
        }

        scope.or_else(|| {
            self.language.clone().map(|language| LanguageScope {
                language,
                override_id: None,
            })
        })
    }

    /// Returns a tuple of the range and character kind of the word
    /// surrounding the given position.
    pub fn surrounding_word<T: ToOffset>(&self, start: T) -> (Range<usize>, Option<CharKind>) {
        let mut start = start.to_offset(self);
        let mut end = start;
        let mut next_chars = self.chars_at(start).peekable();
        let mut prev_chars = self.reversed_chars_at(start).peekable();

        let scope = self.language_scope_at(start);
        let kind = |c| char_kind(&scope, c);
        let word_kind = cmp::max(
            prev_chars.peek().copied().map(kind),
            next_chars.peek().copied().map(kind),
        );

        for ch in prev_chars {
            if Some(kind(ch)) == word_kind && ch != '\n' {
                start -= ch.len_utf8();
            } else {
                break;
            }
        }

        for ch in next_chars {
            if Some(kind(ch)) == word_kind && ch != '\n' {
                end += ch.len_utf8();
            } else {
                break;
            }
        }

        (start..end, word_kind)
    }

    /// Returns the range for the closes syntax node enclosing the given range.
    pub fn range_for_syntax_ancestor<T: ToOffset>(&self, range: Range<T>) -> Option<Range<usize>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut result: Option<Range<usize>> = None;
        'outer: for layer in self.syntax.layers_for_range(range.clone(), &self.text) {
            let mut cursor = layer.node().walk();

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

    /// Returns the outline for the buffer.
    ///
    /// This method allows passing an optional [SyntaxTheme] to
    /// syntax-highlight the returned symbols.
    pub fn outline(&self, theme: Option<&SyntaxTheme>) -> Option<Outline<Anchor>> {
        self.outline_items_containing(0..self.len(), true, theme)
            .map(Outline::new)
    }

    /// Returns all the symbols that contain the given position.
    ///
    /// This method allows passing an optional [SyntaxTheme] to
    /// syntax-highlight the returned symbols.
    pub fn symbols_containing<T: ToOffset>(
        &self,
        position: T,
        theme: Option<&SyntaxTheme>,
    ) -> Option<Vec<OutlineItem<Anchor>>> {
        let position = position.to_offset(self);
        let mut items = self.outline_items_containing(
            position.saturating_sub(1)..self.len().min(position + 1),
            false,
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
        include_extra_context: bool,
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
                } else if Some(capture.index) == config.context_capture_ix
                    || (Some(capture.index) == config.extra_context_capture_ix
                        && include_extra_context)
                {
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

                if !range.is_empty() {
                    buffer_ranges.push((range, node_is_name));
                }
            }

            if buffer_ranges.is_empty() {
                matches.advance();
                continue;
            }

            let mut text = String::new();
            let mut highlight_ranges = Vec::new();
            let mut name_ranges = Vec::new();
            let mut chunks = self.chunks(
                buffer_ranges.first().unwrap().0.start..buffer_ranges.last().unwrap().0.end,
                true,
            );
            let mut last_buffer_range_end = 0;
            for (buffer_range, is_name) in buffer_ranges {
                if !text.is_empty() && buffer_range.start > last_buffer_range_end {
                    text.push(' ');
                }
                last_buffer_range_end = buffer_range.end;
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

    /// For each grammar in the language, runs the provided
    /// [tree_sitter::Query] against the given range.
    pub fn matches(
        &self,
        range: Range<usize>,
        query: fn(&Grammar) -> Option<&tree_sitter::Query>,
    ) -> SyntaxMapMatches {
        self.syntax.matches(range, self, query)
    }

    /// Returns bracket range pairs overlapping or adjacent to `range`
    pub fn bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = (Range<usize>, Range<usize>)> + '_ {
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

                let Some((open, close)) = open.zip(close) else {
                    continue;
                };

                let bracket_range = open.start..=close.end;
                if !bracket_range.overlaps(&range) {
                    continue;
                }

                return Some((open, close));
            }
            None
        })
    }

    /// Returns enclosing bracket ranges containing the given range
    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = (Range<usize>, Range<usize>)> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);

        self.bracket_ranges(range.clone())
            .filter(move |(open, close)| open.start <= range.start && close.end >= range.end)
    }

    /// Returns the smallest enclosing bracket ranges containing the given range or None if no brackets contain range
    ///
    /// Can optionally pass a range_filter to filter the ranges of brackets to consider
    pub fn innermost_enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
        range_filter: Option<&dyn Fn(Range<usize>, Range<usize>) -> bool>,
    ) -> Option<(Range<usize>, Range<usize>)> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);

        // Get the ranges of the innermost pair of brackets.
        let mut result: Option<(Range<usize>, Range<usize>)> = None;

        for (open, close) in self.enclosing_bracket_ranges(range.clone()) {
            if let Some(range_filter) = range_filter {
                if !range_filter(open.clone(), close.clone()) {
                    continue;
                }
            }

            let len = close.end - open.start;

            if let Some((existing_open, existing_close)) = &result {
                let existing_len = existing_close.end - existing_open.start;
                if len > existing_len {
                    continue;
                }
            }

            result = Some((open, close));
        }

        result
    }

    /// Returns anchor ranges for any matches of the redaction query.
    /// The buffer can be associated with multiple languages, and the redaction query associated with each
    /// will be run on the relevant section of the buffer.
    pub fn redacted_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = Range<usize>> + '_ {
        let offset_range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut syntax_matches = self.syntax.matches(offset_range, self, |grammar| {
            grammar
                .redactions_config
                .as_ref()
                .map(|config| &config.query)
        });

        let configs = syntax_matches
            .grammars()
            .iter()
            .map(|grammar| grammar.redactions_config.as_ref())
            .collect::<Vec<_>>();

        iter::from_fn(move || {
            let redacted_range = syntax_matches
                .peek()
                .and_then(|mat| {
                    configs[mat.grammar_index].and_then(|config| {
                        mat.captures
                            .iter()
                            .find(|capture| capture.index == config.redaction_capture_ix)
                    })
                })
                .map(|mat| mat.node.byte_range());
            syntax_matches.advance();
            redacted_range
        })
    }

    pub fn runnable_ranges(
        &self,
        range: Range<Anchor>,
    ) -> impl Iterator<Item = RunnableRange> + '_ {
        let offset_range = range.start.to_offset(self)..range.end.to_offset(self);

        let mut syntax_matches = self.syntax.matches(offset_range, self, |grammar| {
            grammar.runnable_config.as_ref().map(|config| &config.query)
        });

        let test_configs = syntax_matches
            .grammars()
            .iter()
            .map(|grammar| grammar.runnable_config.as_ref())
            .collect::<Vec<_>>();

        iter::from_fn(move || loop {
            let mat = syntax_matches.peek()?;

            let test_range = test_configs[mat.grammar_index].and_then(|test_configs| {
                let mut run_range = None;
                let full_range = mat.captures.iter().fold(
                    Range {
                        start: usize::MAX,
                        end: 0,
                    },
                    |mut acc, next| {
                        let byte_range = next.node.byte_range();
                        if acc.start > byte_range.start {
                            acc.start = byte_range.start;
                        }
                        if acc.end < byte_range.end {
                            acc.end = byte_range.end;
                        }
                        acc
                    },
                );
                if full_range.start > full_range.end {
                    // We did not find a full spanning range of this match.
                    return None;
                }
                let extra_captures: SmallVec<[_; 1]> =
                    SmallVec::from_iter(mat.captures.iter().filter_map(|capture| {
                        test_configs
                            .extra_captures
                            .get(capture.index as usize)
                            .cloned()
                            .and_then(|tag_name| match tag_name {
                                RunnableCapture::Named(name) => {
                                    Some((capture.node.byte_range(), name))
                                }
                                RunnableCapture::Run => {
                                    let _ = run_range.insert(capture.node.byte_range());
                                    None
                                }
                            })
                    }));
                let run_range = run_range?;
                let tags = test_configs
                    .query
                    .property_settings(mat.pattern_index)
                    .iter()
                    .filter_map(|property| {
                        if *property.key == *"tag" {
                            property
                                .value
                                .as_ref()
                                .map(|value| RunnableTag(value.to_string().into()))
                        } else {
                            None
                        }
                    })
                    .collect();
                let extra_captures = extra_captures
                    .into_iter()
                    .map(|(range, name)| {
                        (
                            name.to_string(),
                            self.text_for_range(range.clone()).collect::<String>(),
                        )
                    })
                    .collect();
                // All tags should have the same range.
                Some(RunnableRange {
                    run_range,
                    full_range,
                    runnable: Runnable {
                        tags,
                        language: mat.language,
                        buffer: self.remote_id(),
                    },
                    extra_captures,
                    buffer_id: self.remote_id(),
                })
            });

            syntax_matches.advance();
            if test_range.is_some() {
                // It's fine for us to short-circuit on .peek()? returning None. We don't want to return None from this iter if we
                // had a capture that did not contain a run marker, hence we'll just loop around for the next capture.
                return test_range;
            }
        })
    }

    pub fn indent_guides_in_range(
        &self,
        range: Range<Anchor>,
        cx: &AppContext,
    ) -> Vec<IndentGuide> {
        fn tab_size_for_row(this: &BufferSnapshot, row: BufferRow, cx: &AppContext) -> u32 {
            let language = this.language_at(Point::new(row, 0));
            language_settings(language, None, cx).tab_size.get() as u32
        }

        let start_row = range.start.to_point(self).row;
        let end_row = range.end.to_point(self).row;
        let row_range = start_row..end_row + 1;

        let mut row_indents = self.line_indents_in_row_range(row_range.clone());

        let mut result_vec = Vec::new();
        let mut indent_stack = SmallVec::<[IndentGuide; 8]>::new();

        // TODO: This should be calculated for every row but it is pretty expensive
        let tab_size = tab_size_for_row(self, start_row, cx);

        while let Some((first_row, mut line_indent)) = row_indents.next() {
            let current_depth = indent_stack.len() as u32;

            // When encountering empty, continue until found useful line indent
            // then add to the indent stack with the depth found
            let mut found_indent = false;
            let mut last_row = first_row;
            if line_indent.is_line_empty() {
                let mut trailing_row = end_row;
                while !found_indent {
                    let (target_row, new_line_indent) =
                        if let Some(display_row) = row_indents.next() {
                            display_row
                        } else {
                            // This means we reached the end of the given range and found empty lines at the end.
                            // We need to traverse further until we find a non-empty line to know if we need to add
                            // an indent guide for the last visible indent.
                            trailing_row += 1;

                            const TRAILING_ROW_SEARCH_LIMIT: u32 = 25;
                            if trailing_row > self.max_point().row
                                || trailing_row > end_row + TRAILING_ROW_SEARCH_LIMIT
                            {
                                break;
                            }
                            let new_line_indent = self.line_indent_for_row(trailing_row);
                            (trailing_row, new_line_indent)
                        };

                    if new_line_indent.is_line_empty() {
                        continue;
                    }
                    last_row = target_row.min(end_row);
                    line_indent = new_line_indent;
                    found_indent = true;
                    break;
                }
            } else {
                found_indent = true
            }

            let depth = if found_indent {
                line_indent.len(tab_size) / tab_size
                    + ((line_indent.len(tab_size) % tab_size) > 0) as u32
            } else {
                current_depth
            };

            if depth < current_depth {
                for _ in 0..(current_depth - depth) {
                    let mut indent = indent_stack.pop().unwrap();
                    if last_row != first_row {
                        // In this case, we landed on an empty row, had to seek forward,
                        // and discovered that the indent we where on is ending.
                        // This means that the last display row must
                        // be on line that ends this indent range, so we
                        // should display the range up to the first non-empty line
                        indent.end_row = first_row.saturating_sub(1);
                    }

                    result_vec.push(indent)
                }
            } else if depth > current_depth {
                for next_depth in current_depth..depth {
                    indent_stack.push(IndentGuide {
                        buffer_id: self.remote_id(),
                        start_row: first_row,
                        end_row: last_row,
                        depth: next_depth,
                        tab_size,
                    });
                }
            }

            for indent in indent_stack.iter_mut() {
                indent.end_row = last_row;
            }
        }

        result_vec.extend(indent_stack);

        result_vec
    }

    pub async fn enclosing_indent(
        &self,
        mut buffer_row: BufferRow,
    ) -> Option<(Range<BufferRow>, LineIndent)> {
        let max_row = self.max_point().row;
        if buffer_row >= max_row {
            return None;
        }

        let mut target_indent = self.line_indent_for_row(buffer_row);

        // If the current row is at the start of an indented block, we want to return this
        // block as the enclosing indent.
        if !target_indent.is_line_empty() && buffer_row < max_row {
            let next_line_indent = self.line_indent_for_row(buffer_row + 1);
            if !next_line_indent.is_line_empty()
                && target_indent.raw_len() < next_line_indent.raw_len()
            {
                target_indent = next_line_indent;
                buffer_row += 1;
            }
        }

        const SEARCH_ROW_LIMIT: u32 = 25000;
        const SEARCH_WHITESPACE_ROW_LIMIT: u32 = 2500;
        const YIELD_INTERVAL: u32 = 100;

        let mut accessed_row_counter = 0;

        // If there is a blank line at the current row, search for the next non indented lines
        if target_indent.is_line_empty() {
            let start = buffer_row.saturating_sub(SEARCH_WHITESPACE_ROW_LIMIT);
            let end = (max_row + 1).min(buffer_row + SEARCH_WHITESPACE_ROW_LIMIT);

            let mut non_empty_line_above = None;
            for (row, indent) in self
                .text
                .reversed_line_indents_in_row_range(start..buffer_row)
            {
                accessed_row_counter += 1;
                if accessed_row_counter == YIELD_INTERVAL {
                    accessed_row_counter = 0;
                    yield_now().await;
                }
                if !indent.is_line_empty() {
                    non_empty_line_above = Some((row, indent));
                    break;
                }
            }

            let mut non_empty_line_below = None;
            for (row, indent) in self.text.line_indents_in_row_range((buffer_row + 1)..end) {
                accessed_row_counter += 1;
                if accessed_row_counter == YIELD_INTERVAL {
                    accessed_row_counter = 0;
                    yield_now().await;
                }
                if !indent.is_line_empty() {
                    non_empty_line_below = Some((row, indent));
                    break;
                }
            }

            let (row, indent) = match (non_empty_line_above, non_empty_line_below) {
                (Some((above_row, above_indent)), Some((below_row, below_indent))) => {
                    if above_indent.raw_len() >= below_indent.raw_len() {
                        (above_row, above_indent)
                    } else {
                        (below_row, below_indent)
                    }
                }
                (Some(above), None) => above,
                (None, Some(below)) => below,
                _ => return None,
            };

            target_indent = indent;
            buffer_row = row;
        }

        let start = buffer_row.saturating_sub(SEARCH_ROW_LIMIT);
        let end = (max_row + 1).min(buffer_row + SEARCH_ROW_LIMIT);

        let mut start_indent = None;
        for (row, indent) in self
            .text
            .reversed_line_indents_in_row_range(start..buffer_row)
        {
            accessed_row_counter += 1;
            if accessed_row_counter == YIELD_INTERVAL {
                accessed_row_counter = 0;
                yield_now().await;
            }
            if !indent.is_line_empty() && indent.raw_len() < target_indent.raw_len() {
                start_indent = Some((row, indent));
                break;
            }
        }
        let (start_row, start_indent_size) = start_indent?;

        let mut end_indent = (end, None);
        for (row, indent) in self.text.line_indents_in_row_range((buffer_row + 1)..end) {
            accessed_row_counter += 1;
            if accessed_row_counter == YIELD_INTERVAL {
                accessed_row_counter = 0;
                yield_now().await;
            }
            if !indent.is_line_empty() && indent.raw_len() < target_indent.raw_len() {
                end_indent = (row.saturating_sub(1), Some(indent));
                break;
            }
        }
        let (end_row, end_indent_size) = end_indent;

        let indent = if let Some(end_indent_size) = end_indent_size {
            if start_indent_size.raw_len() > end_indent_size.raw_len() {
                start_indent_size
            } else {
                end_indent_size
            }
        } else {
            start_indent_size
        };

        Some((start_row..end_row, indent))
    }

    /// Returns selections for remote peers intersecting the given range.
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

    /// Whether the buffer contains any git changes.
    pub fn has_git_diff(&self) -> bool {
        !self.git_diff.is_empty()
    }

    /// Returns all the Git diff hunks intersecting the given
    /// row range.
    pub fn git_diff_hunks_in_row_range(
        &self,
        range: Range<BufferRow>,
    ) -> impl '_ + Iterator<Item = git::diff::DiffHunk<u32>> {
        self.git_diff.hunks_in_row_range(range, self)
    }

    /// Returns all the Git diff hunks intersecting the given
    /// range.
    pub fn git_diff_hunks_intersecting_range(
        &self,
        range: Range<Anchor>,
    ) -> impl '_ + Iterator<Item = git::diff::DiffHunk<u32>> {
        self.git_diff.hunks_intersecting_range(range, self)
    }

    /// Returns all the Git diff hunks intersecting the given
    /// range, in reverse order.
    pub fn git_diff_hunks_intersecting_range_rev(
        &self,
        range: Range<Anchor>,
    ) -> impl '_ + Iterator<Item = git::diff::DiffHunk<u32>> {
        self.git_diff.hunks_intersecting_range_rev(range, self)
    }

    /// Returns if the buffer contains any diagnostics.
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Returns all the diagnostics intersecting the given range.
    pub fn diagnostics_in_range<'a, T, O>(
        &'a self,
        search_range: Range<T>,
        reversed: bool,
    ) -> impl 'a + Iterator<Item = DiagnosticEntry<O>>
    where
        T: 'a + Clone + ToOffset,
        O: 'a + FromAnchor + Ord,
    {
        let mut iterators: Vec<_> = self
            .diagnostics
            .iter()
            .map(|(_, collection)| {
                collection
                    .range::<T, O>(search_range.clone(), self, true, reversed)
                    .peekable()
            })
            .collect();

        std::iter::from_fn(move || {
            let (next_ix, _) = iterators
                .iter_mut()
                .enumerate()
                .flat_map(|(ix, iter)| Some((ix, iter.peek()?)))
                .min_by(|(_, a), (_, b)| {
                    let cmp = a
                        .range
                        .start
                        .cmp(&b.range.start)
                        // when range is equal, sort by diagnostic severity
                        .then(a.diagnostic.severity.cmp(&b.diagnostic.severity))
                        // and stabilize order with group_id
                        .then(a.diagnostic.group_id.cmp(&b.diagnostic.group_id));
                    if reversed {
                        cmp.reverse()
                    } else {
                        cmp
                    }
                })?;
            iterators[next_ix].next()
        })
    }

    /// Returns all the diagnostic groups associated with the given
    /// language server id. If no language server id is provided,
    /// all diagnostics groups are returned.
    pub fn diagnostic_groups(
        &self,
        language_server_id: Option<LanguageServerId>,
    ) -> Vec<(LanguageServerId, DiagnosticGroup<Anchor>)> {
        let mut groups = Vec::new();

        if let Some(language_server_id) = language_server_id {
            if let Ok(ix) = self
                .diagnostics
                .binary_search_by_key(&language_server_id, |e| e.0)
            {
                self.diagnostics[ix]
                    .1
                    .groups(language_server_id, &mut groups, self);
            }
        } else {
            for (language_server_id, diagnostics) in self.diagnostics.iter() {
                diagnostics.groups(*language_server_id, &mut groups, self);
            }
        }

        groups.sort_by(|(id_a, group_a), (id_b, group_b)| {
            let a_start = &group_a.entries[group_a.primary_ix].range.start;
            let b_start = &group_b.entries[group_b.primary_ix].range.start;
            a_start.cmp(b_start, self).then_with(|| id_a.cmp(id_b))
        });

        groups
    }

    /// Returns an iterator over the diagnostics for the given group.
    pub fn diagnostic_group<'a, O>(
        &'a self,
        group_id: usize,
    ) -> impl 'a + Iterator<Item = DiagnosticEntry<O>>
    where
        O: 'a + FromAnchor,
    {
        self.diagnostics
            .iter()
            .flat_map(move |(_, set)| set.group(group_id, self))
    }

    /// The number of times diagnostics were updated.
    pub fn diagnostics_update_count(&self) -> usize {
        self.diagnostics_update_count
    }

    /// The number of times the buffer was parsed.
    pub fn parse_count(&self) -> usize {
        self.parse_count
    }

    /// The number of times selections were updated.
    pub fn selections_update_count(&self) -> usize {
        self.selections_update_count
    }

    /// Returns a snapshot of underlying file.
    pub fn file(&self) -> Option<&Arc<dyn File>> {
        self.file.as_ref()
    }

    /// Resolves the file path (relative to the worktree root) associated with the underlying file.
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

    /// The number of times the underlying file was updated.
    pub fn file_update_count(&self) -> usize {
        self.file_update_count
    }

    /// The number of times the git diff status was updated.
    pub fn git_diff_update_count(&self) -> usize {
        self.git_diff_update_count
    }
}

fn indent_size_for_line(text: &text::BufferSnapshot, row: u32) -> IndentSize {
    indent_size_for_text(text.chars_at(Point::new(row, 0)))
}

fn indent_size_for_text(text: impl Iterator<Item = char>) -> IndentSize {
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

    /// Seeks to the given byte offset in the buffer.
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

    /// The current byte offset in the buffer.
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
                diagnostic_severity: self.current_diagnostic_severity(),
                is_unnecessary: self.current_code_is_unnecessary(),
                ..Default::default()
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
            source: Default::default(),
            code: None,
            severity: DiagnosticSeverity::ERROR,
            message: Default::default(),
            group_id: 0,
            is_primary: false,
            is_disk_based: false,
            is_unnecessary: false,
        }
    }
}

impl IndentSize {
    /// Returns an [IndentSize] representing the given spaces.
    pub fn spaces(len: u32) -> Self {
        Self {
            len,
            kind: IndentKind::Space,
        }
    }

    /// Returns an [IndentSize] representing a tab.
    pub fn tab() -> Self {
        Self {
            len: 1,
            kind: IndentKind::Tab,
        }
    }

    /// An iterator over the characters represented by this [IndentSize].
    pub fn chars(&self) -> impl Iterator<Item = char> {
        iter::repeat(self.char()).take(self.len as usize)
    }

    /// The character representation of this [IndentSize].
    pub fn char(&self) -> char {
        match self.kind {
            IndentKind::Space => ' ',
            IndentKind::Tab => '\t',
        }
    }

    /// Consumes the current [IndentSize] and returns a new one that has
    /// been shrunk or enlarged by the given size along the given direction.
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

#[cfg(any(test, feature = "test-support"))]
pub struct TestFile {
    pub path: Arc<Path>,
    pub root_name: String,
}

#[cfg(any(test, feature = "test-support"))]
impl File for TestFile {
    fn path(&self) -> &Arc<Path> {
        &self.path
    }

    fn full_path(&self, _: &gpui::AppContext) -> PathBuf {
        PathBuf::from(&self.root_name).join(self.path.as_ref())
    }

    fn as_local(&self) -> Option<&dyn LocalFile> {
        None
    }

    fn mtime(&self) -> Option<SystemTime> {
        unimplemented!()
    }

    fn file_name<'a>(&'a self, _: &'a gpui::AppContext) -> &'a std::ffi::OsStr {
        self.path().file_name().unwrap_or(self.root_name.as_ref())
    }

    fn worktree_id(&self) -> usize {
        0
    }

    fn is_deleted(&self) -> bool {
        unimplemented!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        unimplemented!()
    }

    fn to_proto(&self) -> rpc::proto::File {
        unimplemented!()
    }

    fn is_private(&self) -> bool {
        false
    }
}

pub(crate) fn contiguous_ranges(
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

/// Returns the [CharKind] for the given character. When a scope is provided,
/// the function checks if the character is considered a word character
/// based on the language scope's word character settings.
pub fn char_kind(scope: &Option<LanguageScope>, c: char) -> CharKind {
    if c.is_whitespace() {
        return CharKind::Whitespace;
    } else if c.is_alphanumeric() || c == '_' {
        return CharKind::Word;
    }

    if let Some(scope) = scope {
        if let Some(characters) = scope.word_characters() {
            if characters.contains(&c) {
                return CharKind::Word;
            }
        }
    }

    CharKind::Punctuation
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
