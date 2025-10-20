use crate::{
    DebuggerTextObject, LanguageScope, Outline, OutlineConfig, RunnableCapture, RunnableTag,
    TextObject, TreeSitterOptions,
    diagnostic_set::{DiagnosticEntry, DiagnosticEntryRef, DiagnosticGroup},
    language_settings::{LanguageSettings, language_settings},
    outline::OutlineItem,
    syntax_map::{
        SyntaxLayer, SyntaxMap, SyntaxMapCapture, SyntaxMapCaptures, SyntaxMapMatch,
        SyntaxMapMatches, SyntaxSnapshot, ToTreeSitterPoint,
    },
    task_context::RunnableRange,
    text_diff::text_diff,
};
pub use crate::{
    Grammar, Language, LanguageRegistry,
    diagnostic_set::DiagnosticSet,
    highlight_map::{HighlightId, HighlightMap},
    proto,
};
use anyhow::{Context as _, Result};
use clock::Lamport;
pub use clock::ReplicaId;
use collections::HashMap;
use fs::MTime;
use futures::channel::oneshot;
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, HighlightStyle, SharedString, StyledText,
    Task, TaskLabel, TextStyle,
};

use lsp::{LanguageServerId, NumberOrString};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::WorktreeId;
use smallvec::SmallVec;
use smol::future::yield_now;
use std::{
    any::Any,
    borrow::Cow,
    cell::Cell,
    cmp::{self, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet},
    future::Future,
    iter::{self, Iterator, Peekable},
    mem,
    num::NonZeroU32,
    ops::{Deref, Range},
    path::PathBuf,
    rc,
    sync::{Arc, LazyLock},
    time::{Duration, Instant},
    vec,
};
use sum_tree::TreeMap;
use text::operation_queue::OperationQueue;
use text::*;
pub use text::{
    Anchor, Bias, Buffer as TextBuffer, BufferId, BufferSnapshot as TextBufferSnapshot, Edit,
    LineIndent, OffsetRangeExt, OffsetUtf16, Patch, Point, PointUtf16, Rope, Selection,
    SelectionGoal, Subscription, TextDimension, TextSummary, ToOffset, ToOffsetUtf16, ToPoint,
    ToPointUtf16, Transaction, TransactionId, Unclipped,
};
use theme::{ActiveTheme as _, SyntaxTheme};
#[cfg(any(test, feature = "test-support"))]
use util::RandomCharIter;
use util::{RangeExt, debug_panic, maybe, paths::PathStyle, rel_path::RelPath};

#[cfg(any(test, feature = "test-support"))]
pub use {tree_sitter_python, tree_sitter_rust, tree_sitter_typescript};

pub use lsp::DiagnosticSeverity;

/// A label for the background task spawned by the buffer to compute
/// a diff against the contents of its file.
pub static BUFFER_DIFF_TASK: LazyLock<TaskLabel> = LazyLock::new(TaskLabel::new);

/// Indicate whether a [`Buffer`] has permissions to edit.
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
    branch_state: Option<BufferBranchState>,
    /// Filesystem state, `None` when there is no path.
    file: Option<Arc<dyn File>>,
    /// The mtime of the file when this buffer was last loaded from
    /// or saved to disk.
    saved_mtime: Option<MTime>,
    /// The version vector when this buffer was last loaded from
    /// or saved to disk.
    saved_version: clock::Global,
    preview_version: clock::Global,
    transaction_depth: usize,
    was_dirty_before_starting_transaction: Option<bool>,
    reload_task: Option<Task<Result<()>>>,
    language: Option<Arc<Language>>,
    autoindent_requests: Vec<Arc<AutoindentRequest>>,
    wait_for_autoindent_txs: Vec<oneshot::Sender<()>>,
    pending_autoindent: Option<Task<()>>,
    sync_parse_timeout: Duration,
    syntax_map: Mutex<SyntaxMap>,
    reparse: Option<Task<()>>,
    parse_status: (watch::Sender<ParseStatus>, watch::Receiver<ParseStatus>),
    non_text_state_update_count: usize,
    diagnostics: SmallVec<[(LanguageServerId, DiagnosticSet); 2]>,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    diagnostics_timestamp: clock::Lamport,
    completion_triggers: BTreeSet<String>,
    completion_triggers_per_language_server: HashMap<LanguageServerId, BTreeSet<String>>,
    completion_triggers_timestamp: clock::Lamport,
    deferred_ops: OperationQueue<Operation>,
    capability: Capability,
    has_conflict: bool,
    /// Memoize calls to has_changes_since(saved_version).
    /// The contents of a cell are (self.version, has_changes) at the time of a last call.
    has_unsaved_edits: Cell<(clock::Global, bool)>,
    change_bits: Vec<rc::Weak<Cell<bool>>>,
    _subscriptions: Vec<gpui::Subscription>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParseStatus {
    Idle,
    Parsing,
}

struct BufferBranchState {
    base_buffer: Entity<Buffer>,
    merged_operations: Vec<Lamport>,
}

/// An immutable, cheaply cloneable representation of a fixed
/// state of a buffer.
pub struct BufferSnapshot {
    pub text: text::BufferSnapshot,
    pub syntax: SyntaxSnapshot,
    file: Option<Arc<dyn File>>,
    diagnostics: SmallVec<[(LanguageServerId, DiagnosticSet); 2]>,
    remote_selections: TreeMap<ReplicaId, SelectionSet>,
    language: Option<Arc<Language>>,
    non_text_state_update_count: usize,
}

/// The kind and amount of indentation in a particular line. For now,
/// assumes that indentation is all the same character.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct IndentSize {
    /// The number of bytes that comprise the indentation.
    pub len: u32,
    /// The kind of whitespace used for indentation.
    pub kind: IndentKind,
}

/// A whitespace character that's used for indentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum IndentKind {
    /// An ASCII space character.
    #[default]
    Space,
    /// An ASCII tab character.
    Tab,
}

/// The shape of a selection cursor.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum CursorShape {
    /// A vertical bar
    #[default]
    Bar,
    /// A block that surrounds the following character
    Block,
    /// An underline that runs along the following character
    Underline,
    /// A box drawn around the following character
    Hollow,
}

impl From<settings::CursorShape> for CursorShape {
    fn from(shape: settings::CursorShape) -> Self {
        match shape {
            settings::CursorShape::Bar => CursorShape::Bar,
            settings::CursorShape::Block => CursorShape::Block,
            settings::CursorShape::Underline => CursorShape::Underline,
            settings::CursorShape::Hollow => CursorShape::Hollow,
        }
    }
}

#[derive(Clone, Debug)]
struct SelectionSet {
    line_mode: bool,
    cursor_shape: CursorShape,
    selections: Arc<[Selection<Anchor>]>,
    lamport_timestamp: clock::Lamport,
}

/// A diagnostic associated with a certain range of a buffer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The name of the service that produced this diagnostic.
    pub source: Option<String>,
    /// A machine-readable code that identifies this diagnostic.
    pub code: Option<NumberOrString>,
    pub code_description: Option<lsp::Uri>,
    /// Whether this diagnostic is a hint, warning, or error.
    pub severity: DiagnosticSeverity,
    /// The human-readable message associated with this diagnostic.
    pub message: String,
    /// The human-readable message (in markdown format)
    pub markdown: Option<String>,
    /// An id that identifies the group to which this diagnostic belongs.
    ///
    /// When a language server produces a diagnostic with
    /// one or more associated diagnostics, those diagnostics are all
    /// assigned a single group ID.
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
    /// Quick separation of diagnostics groups based by their source.
    pub source_kind: DiagnosticSourceKind,
    /// Data from language server that produced this diagnostic. Passed back to the LS when we request code actions for this diagnostic.
    pub data: Option<Value>,
    /// Whether to underline the corresponding text range in the editor.
    pub underline: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSourceKind {
    Pulled,
    Pushed,
    Other,
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
        /// The language server ID.
        server_id: LanguageServerId,
    },

    /// An update to the line ending type of this buffer.
    UpdateLineEnding {
        /// The line ending type.
        line_ending: LineEnding,
        /// The buffer's lamport timestamp.
        lamport_timestamp: clock::Lamport,
    },
}

/// An event that occurs in a buffer.
#[derive(Clone, Debug, PartialEq)]
pub enum BufferEvent {
    /// The buffer was changed in a way that must be
    /// propagated to its other replicas.
    Operation {
        operation: Operation,
        is_local: bool,
    },
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
    /// The buffer is in need of a reload
    ReloadNeeded,
    /// The buffer's language was changed.
    LanguageChanged,
    /// The buffer's syntax trees were updated.
    Reparsed,
    /// The buffer's diagnostics were updated.
    DiagnosticsUpdated,
    /// The buffer gained or lost editing capabilities.
    CapabilityChanged,
}

/// The file associated with a buffer.
pub trait File: Send + Sync + Any {
    /// Returns the [`LocalFile`] associated with this file, if the
    /// file is local.
    fn as_local(&self) -> Option<&dyn LocalFile>;

    /// Returns whether this file is local.
    fn is_local(&self) -> bool {
        self.as_local().is_some()
    }

    /// Returns whether the file is new, exists in storage, or has been deleted. Includes metadata
    /// only available in some states, such as modification time.
    fn disk_state(&self) -> DiskState;

    /// Returns the path of this file relative to the worktree's root directory.
    fn path(&self) -> &Arc<RelPath>;

    /// Returns the path of this file relative to the worktree's parent directory (this means it
    /// includes the name of the worktree's root folder).
    fn full_path(&self, cx: &App) -> PathBuf;

    /// Returns the path style of this file.
    fn path_style(&self, cx: &App) -> PathStyle;

    /// Returns the last component of this handle's absolute path. If this handle refers to the root
    /// of its worktree, then this method will return the name of the worktree itself.
    fn file_name<'a>(&'a self, cx: &'a App) -> &'a str;

    /// Returns the id of the worktree to which this file belongs.
    ///
    /// This is needed for looking up project-specific settings.
    fn worktree_id(&self, cx: &App) -> WorktreeId;

    /// Converts this file into a protobuf message.
    fn to_proto(&self, cx: &App) -> rpc::proto::File;

    /// Return whether Zed considers this to be a private file.
    fn is_private(&self) -> bool;
}

/// The file's storage status - whether it's stored (`Present`), and if so when it was last
/// modified. In the case where the file is not stored, it can be either `New` or `Deleted`. In the
/// UI these two states are distinguished. For example, the buffer tab does not display a deletion
/// indicator for new files.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DiskState {
    /// File created in Zed that has not been saved.
    New,
    /// File present on the filesystem.
    Present { mtime: MTime },
    /// Deleted file that was previously present.
    Deleted,
}

impl DiskState {
    /// Returns the file's last known modification time on disk.
    pub fn mtime(self) -> Option<MTime> {
        match self {
            DiskState::New => None,
            DiskState::Present { mtime } => Some(mtime),
            DiskState::Deleted => None,
        }
    }

    pub fn exists(&self) -> bool {
        match self {
            DiskState::New => false,
            DiskState::Present { .. } => true,
            DiskState::Deleted => false,
        }
    }
}

/// The file associated with a buffer, in the case where the file is on the local disk.
pub trait LocalFile: File {
    /// Returns the absolute path of this file
    fn abs_path(&self, cx: &App) -> PathBuf;

    /// Loads the file contents from disk and returns them as a UTF-8 encoded string.
    fn load(&self, cx: &App) -> Task<Result<String>>;

    /// Loads the file's contents from disk.
    fn load_bytes(&self, cx: &App) -> Task<Result<Vec<u8>>>;
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
        /// The original indentation column of the first line of each
        /// insertion, if it has been copied.
        ///
        /// Knowing this makes it possible to preserve the relative indentation
        /// of every line in the insertion from when it was copied.
        ///
        /// If the original indent column is `a`, and the first line of insertion
        /// is then auto-indented to column `b`, then every other line of
        /// the insertion will be auto-indented to column `b - a`
        original_indent_columns: Vec<Option<u32>>,
    },
}

#[derive(Clone)]
struct AutoindentRequest {
    before_edit: BufferSnapshot,
    entries: Vec<AutoindentRequestEntry>,
    is_block_mode: bool,
    ignore_empty_lines: bool,
}

#[derive(Debug, Clone)]
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
    buffer_snapshot: Option<&'a BufferSnapshot>,
    range: Range<usize>,
    chunks: text::Chunks<'a>,
    diagnostic_endpoints: Option<Peekable<vec::IntoIter<DiagnosticEndpoint>>>,
    error_depth: usize,
    warning_depth: usize,
    information_depth: usize,
    hint_depth: usize,
    unnecessary_depth: usize,
    underline: bool,
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
    /// A bitset of which characters are tabs in this string.
    pub tabs: u128,
    /// Bitmap of character indices in this chunk
    pub chars: u128,
    /// Whether this chunk of text was originally a tab character.
    pub is_inlay: bool,
    /// Whether to underline the corresponding text range in the editor.
    pub underline: bool,
}

/// A set of edits to a given version of a buffer, computed asynchronously.
#[derive(Debug)]
pub struct Diff {
    pub base_version: clock::Global,
    pub line_ending: LineEnding,
    pub edits: Vec<(Range<usize>, Arc<str>)>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DiagnosticEndpoint {
    offset: usize,
    is_start: bool,
    underline: bool,
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

/// Context for character classification within a specific scope.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CharScopeContext {
    /// Character classification for completion queries.
    ///
    /// This context treats certain characters as word constituents that would
    /// normally be considered punctuation, such as '-' in Tailwind classes
    /// ("bg-yellow-100") or '.' in import paths ("foo.ts").
    Completion,
    /// Character classification for linked edits.
    ///
    /// This context handles characters that should be treated as part of
    /// identifiers during linked editing operations, such as '.' in JSX
    /// component names like `<Animated.View>`.
    LinkedEdit,
}

/// A runnable is a set of data about a region that could be resolved into a task
pub struct Runnable {
    pub tags: SmallVec<[RunnableTag; 1]>,
    pub language: Arc<Language>,
    pub buffer: BufferId,
}

#[derive(Default, Clone, Debug)]
pub struct HighlightedText {
    pub text: SharedString,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
}

#[derive(Default, Debug)]
struct HighlightedTextBuilder {
    pub text: String,
    highlights: Vec<(Range<usize>, HighlightStyle)>,
}

impl HighlightedText {
    pub fn from_buffer_range<T: ToOffset>(
        range: Range<T>,
        snapshot: &text::BufferSnapshot,
        syntax_snapshot: &SyntaxSnapshot,
        override_style: Option<HighlightStyle>,
        syntax_theme: &SyntaxTheme,
    ) -> Self {
        let mut highlighted_text = HighlightedTextBuilder::default();
        highlighted_text.add_text_from_buffer_range(
            range,
            snapshot,
            syntax_snapshot,
            override_style,
            syntax_theme,
        );
        highlighted_text.build()
    }

    pub fn to_styled_text(&self, default_style: &TextStyle) -> StyledText {
        gpui::StyledText::new(self.text.clone())
            .with_default_highlights(default_style, self.highlights.iter().cloned())
    }

    /// Returns the first line without leading whitespace unless highlighted
    /// and a boolean indicating if there are more lines after
    pub fn first_line_preview(self) -> (Self, bool) {
        let newline_ix = self.text.find('\n').unwrap_or(self.text.len());
        let first_line = &self.text[..newline_ix];

        // Trim leading whitespace, unless an edit starts prior to it.
        let mut preview_start_ix = first_line.len() - first_line.trim_start().len();
        if let Some((first_highlight_range, _)) = self.highlights.first() {
            preview_start_ix = preview_start_ix.min(first_highlight_range.start);
        }

        let preview_text = &first_line[preview_start_ix..];
        let preview_highlights = self
            .highlights
            .into_iter()
            .skip_while(|(range, _)| range.end <= preview_start_ix)
            .take_while(|(range, _)| range.start < newline_ix)
            .filter_map(|(mut range, highlight)| {
                range.start = range.start.saturating_sub(preview_start_ix);
                range.end = range.end.min(newline_ix).saturating_sub(preview_start_ix);
                if range.is_empty() {
                    None
                } else {
                    Some((range, highlight))
                }
            });

        let preview = Self {
            text: SharedString::new(preview_text),
            highlights: preview_highlights.collect(),
        };

        (preview, self.text.len() > newline_ix)
    }
}

impl HighlightedTextBuilder {
    pub fn build(self) -> HighlightedText {
        HighlightedText {
            text: self.text.into(),
            highlights: self.highlights,
        }
    }

    pub fn add_text_from_buffer_range<T: ToOffset>(
        &mut self,
        range: Range<T>,
        snapshot: &text::BufferSnapshot,
        syntax_snapshot: &SyntaxSnapshot,
        override_style: Option<HighlightStyle>,
        syntax_theme: &SyntaxTheme,
    ) {
        let range = range.to_offset(snapshot);
        for chunk in Self::highlighted_chunks(range, snapshot, syntax_snapshot) {
            let start = self.text.len();
            self.text.push_str(chunk.text);
            let end = self.text.len();

            if let Some(highlight_style) = chunk
                .syntax_highlight_id
                .and_then(|id| id.style(syntax_theme))
            {
                let highlight_style = override_style.map_or(highlight_style, |override_style| {
                    highlight_style.highlight(override_style)
                });
                self.highlights.push((start..end, highlight_style));
            } else if let Some(override_style) = override_style {
                self.highlights.push((start..end, override_style));
            }
        }
    }

    fn highlighted_chunks<'a>(
        range: Range<usize>,
        snapshot: &'a text::BufferSnapshot,
        syntax_snapshot: &'a SyntaxSnapshot,
    ) -> BufferChunks<'a> {
        let captures = syntax_snapshot.captures(range.clone(), snapshot, |grammar| {
            grammar
                .highlights_config
                .as_ref()
                .map(|config| &config.query)
        });

        let highlight_maps = captures
            .grammars()
            .iter()
            .map(|grammar| grammar.highlight_map())
            .collect();

        BufferChunks::new(
            snapshot.as_rope(),
            range,
            Some((captures, highlight_maps)),
            false,
            None,
        )
    }
}

#[derive(Clone)]
pub struct EditPreview {
    old_snapshot: text::BufferSnapshot,
    applied_edits_snapshot: text::BufferSnapshot,
    syntax_snapshot: SyntaxSnapshot,
}

impl EditPreview {
    pub fn highlight_edits(
        &self,
        current_snapshot: &BufferSnapshot,
        edits: &[(Range<Anchor>, String)],
        include_deletions: bool,
        cx: &App,
    ) -> HighlightedText {
        let Some(visible_range_in_preview_snapshot) = self.compute_visible_range(edits) else {
            return HighlightedText::default();
        };

        let mut highlighted_text = HighlightedTextBuilder::default();

        let mut offset_in_preview_snapshot = visible_range_in_preview_snapshot.start;

        let insertion_highlight_style = HighlightStyle {
            background_color: Some(cx.theme().status().created_background),
            ..Default::default()
        };
        let deletion_highlight_style = HighlightStyle {
            background_color: Some(cx.theme().status().deleted_background),
            ..Default::default()
        };
        let syntax_theme = cx.theme().syntax();

        for (range, edit_text) in edits {
            let edit_new_end_in_preview_snapshot = range
                .end
                .bias_right(&self.old_snapshot)
                .to_offset(&self.applied_edits_snapshot);
            let edit_start_in_preview_snapshot = edit_new_end_in_preview_snapshot - edit_text.len();

            let unchanged_range_in_preview_snapshot =
                offset_in_preview_snapshot..edit_start_in_preview_snapshot;
            if !unchanged_range_in_preview_snapshot.is_empty() {
                highlighted_text.add_text_from_buffer_range(
                    unchanged_range_in_preview_snapshot,
                    &self.applied_edits_snapshot,
                    &self.syntax_snapshot,
                    None,
                    syntax_theme,
                );
            }

            let range_in_current_snapshot = range.to_offset(current_snapshot);
            if include_deletions && !range_in_current_snapshot.is_empty() {
                highlighted_text.add_text_from_buffer_range(
                    range_in_current_snapshot,
                    &current_snapshot.text,
                    &current_snapshot.syntax,
                    Some(deletion_highlight_style),
                    syntax_theme,
                );
            }

            if !edit_text.is_empty() {
                highlighted_text.add_text_from_buffer_range(
                    edit_start_in_preview_snapshot..edit_new_end_in_preview_snapshot,
                    &self.applied_edits_snapshot,
                    &self.syntax_snapshot,
                    Some(insertion_highlight_style),
                    syntax_theme,
                );
            }

            offset_in_preview_snapshot = edit_new_end_in_preview_snapshot;
        }

        highlighted_text.add_text_from_buffer_range(
            offset_in_preview_snapshot..visible_range_in_preview_snapshot.end,
            &self.applied_edits_snapshot,
            &self.syntax_snapshot,
            None,
            syntax_theme,
        );

        highlighted_text.build()
    }

    fn compute_visible_range(&self, edits: &[(Range<Anchor>, String)]) -> Option<Range<usize>> {
        let (first, _) = edits.first()?;
        let (last, _) = edits.last()?;

        let start = first
            .start
            .bias_left(&self.old_snapshot)
            .to_point(&self.applied_edits_snapshot);
        let end = last
            .end
            .bias_right(&self.old_snapshot)
            .to_point(&self.applied_edits_snapshot);

        // Ensure that the first line of the first edit and the last line of the last edit are always fully visible
        let range = Point::new(start.row, 0)
            ..Point::new(end.row, self.applied_edits_snapshot.line_len(end.row));

        Some(range.to_offset(&self.applied_edits_snapshot))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BracketMatch {
    pub open_range: Range<usize>,
    pub close_range: Range<usize>,
    pub newline_only: bool,
}

impl Buffer {
    /// Create a new buffer with the given base text.
    pub fn local<T: Into<String>>(base_text: T, cx: &Context<Self>) -> Self {
        Self::build(
            TextBuffer::new(
                ReplicaId::LOCAL,
                cx.entity_id().as_non_zero_u64().into(),
                base_text.into(),
            ),
            None,
            Capability::ReadWrite,
        )
    }

    /// Create a new buffer with the given base text that has proper line endings and other normalization applied.
    pub fn local_normalized(
        base_text_normalized: Rope,
        line_ending: LineEnding,
        cx: &Context<Self>,
    ) -> Self {
        Self::build(
            TextBuffer::new_normalized(
                ReplicaId::LOCAL,
                cx.entity_id().as_non_zero_u64().into(),
                line_ending,
                base_text_normalized,
            ),
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
        let buffer_id = BufferId::new(message.id).context("Could not deserialize buffer_id")?;
        let buffer = TextBuffer::new(replica_id, buffer_id, message.base_text);
        let mut this = Self::build(buffer, file, capability);
        this.text.set_line_ending(proto::deserialize_line_ending(
            rpc::proto::LineEnding::from_i32(message.line_ending).context("missing line_ending")?,
        ));
        this.saved_version = proto::deserialize_version(&message.saved_version);
        this.saved_mtime = message.saved_mtime.map(|time| time.into());
        Ok(this)
    }

    /// Serialize the buffer's state to a protobuf message.
    pub fn to_proto(&self, cx: &App) -> proto::BufferState {
        proto::BufferState {
            id: self.remote_id().into(),
            file: self.file.as_ref().map(|f| f.to_proto(cx)),
            base_text: self.base_text().to_string(),
            line_ending: proto::serialize_line_ending(self.line_ending()) as i32,
            saved_version: proto::serialize_version(&self.saved_version),
            saved_mtime: self.saved_mtime.map(|time| time.into()),
        }
    }

    /// Serialize as protobufs all of the changes to the buffer since the given version.
    pub fn serialize_ops(
        &self,
        since: Option<clock::Global>,
        cx: &App,
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

        for (server_id, completions) in &self.completion_triggers_per_language_server {
            operations.push(proto::serialize_operation(
                &Operation::UpdateCompletionTriggers {
                    triggers: completions.iter().cloned().collect(),
                    lamport_timestamp: self.completion_triggers_timestamp,
                    server_id: *server_id,
                },
            ));
        }

        let text_operations = self.text.operations().clone();
        cx.background_spawn(async move {
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
    pub fn with_language(mut self, language: Arc<Language>, cx: &mut Context<Self>) -> Self {
        self.set_language(Some(language), cx);
        self
    }

    /// Returns the [`Capability`] of this buffer.
    pub fn capability(&self) -> Capability {
        self.capability
    }

    /// Whether this buffer can only be read.
    pub fn read_only(&self) -> bool {
        self.capability == Capability::ReadOnly
    }

    /// Builds a [`Buffer`] with the given underlying [`TextBuffer`], diff base, [`File`] and [`Capability`].
    pub fn build(buffer: TextBuffer, file: Option<Arc<dyn File>>, capability: Capability) -> Self {
        let saved_mtime = file.as_ref().and_then(|file| file.disk_state().mtime());
        let snapshot = buffer.snapshot();
        let syntax_map = Mutex::new(SyntaxMap::new(&snapshot));
        Self {
            saved_mtime,
            saved_version: buffer.version(),
            preview_version: buffer.version(),
            reload_task: None,
            transaction_depth: 0,
            was_dirty_before_starting_transaction: None,
            has_unsaved_edits: Cell::new((buffer.version(), false)),
            text: buffer,
            branch_state: None,
            file,
            capability,
            syntax_map,
            reparse: None,
            non_text_state_update_count: 0,
            sync_parse_timeout: Duration::from_millis(1),
            parse_status: watch::channel(ParseStatus::Idle),
            autoindent_requests: Default::default(),
            wait_for_autoindent_txs: Default::default(),
            pending_autoindent: Default::default(),
            language: None,
            remote_selections: Default::default(),
            diagnostics: Default::default(),
            diagnostics_timestamp: Lamport::MIN,
            completion_triggers: Default::default(),
            completion_triggers_per_language_server: Default::default(),
            completion_triggers_timestamp: Lamport::MIN,
            deferred_ops: OperationQueue::new(),
            has_conflict: false,
            change_bits: Default::default(),
            _subscriptions: Vec::new(),
        }
    }

    pub fn build_snapshot(
        text: Rope,
        language: Option<Arc<Language>>,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut App,
    ) -> impl Future<Output = BufferSnapshot> + use<> {
        let entity_id = cx.reserve_entity::<Self>().entity_id();
        let buffer_id = entity_id.as_non_zero_u64().into();
        async move {
            let text =
                TextBuffer::new_normalized(ReplicaId::LOCAL, buffer_id, Default::default(), text)
                    .snapshot();
            let mut syntax = SyntaxMap::new(&text).snapshot();
            if let Some(language) = language.clone() {
                let language_registry = language_registry.clone();
                syntax.reparse(&text, language_registry, language);
            }
            BufferSnapshot {
                text,
                syntax,
                file: None,
                diagnostics: Default::default(),
                remote_selections: Default::default(),
                language,
                non_text_state_update_count: 0,
            }
        }
    }

    pub fn build_empty_snapshot(cx: &mut App) -> BufferSnapshot {
        let entity_id = cx.reserve_entity::<Self>().entity_id();
        let buffer_id = entity_id.as_non_zero_u64().into();
        let text = TextBuffer::new_normalized(
            ReplicaId::LOCAL,
            buffer_id,
            Default::default(),
            Rope::new(),
        )
        .snapshot();
        let syntax = SyntaxMap::new(&text).snapshot();
        BufferSnapshot {
            text,
            syntax,
            file: None,
            diagnostics: Default::default(),
            remote_selections: Default::default(),
            language: None,
            non_text_state_update_count: 0,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn build_snapshot_sync(
        text: Rope,
        language: Option<Arc<Language>>,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut App,
    ) -> BufferSnapshot {
        let entity_id = cx.reserve_entity::<Self>().entity_id();
        let buffer_id = entity_id.as_non_zero_u64().into();
        let text =
            TextBuffer::new_normalized(ReplicaId::LOCAL, buffer_id, Default::default(), text)
                .snapshot();
        let mut syntax = SyntaxMap::new(&text).snapshot();
        if let Some(language) = language.clone() {
            syntax.reparse(&text, language_registry, language);
        }
        BufferSnapshot {
            text,
            syntax,
            file: None,
            diagnostics: Default::default(),
            remote_selections: Default::default(),
            language,
            non_text_state_update_count: 0,
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
            file: self.file.clone(),
            remote_selections: self.remote_selections.clone(),
            diagnostics: self.diagnostics.clone(),
            language: self.language.clone(),
            non_text_state_update_count: self.non_text_state_update_count,
        }
    }

    pub fn branch(&mut self, cx: &mut Context<Self>) -> Entity<Self> {
        let this = cx.entity();
        cx.new(|cx| {
            let mut branch = Self {
                branch_state: Some(BufferBranchState {
                    base_buffer: this.clone(),
                    merged_operations: Default::default(),
                }),
                language: self.language.clone(),
                has_conflict: self.has_conflict,
                has_unsaved_edits: Cell::new(self.has_unsaved_edits.get_mut().clone()),
                _subscriptions: vec![cx.subscribe(&this, Self::on_base_buffer_event)],
                ..Self::build(self.text.branch(), self.file.clone(), self.capability())
            };
            if let Some(language_registry) = self.language_registry() {
                branch.set_language_registry(language_registry);
            }

            // Reparse the branch buffer so that we get syntax highlighting immediately.
            branch.reparse(cx);

            branch
        })
    }

    pub fn preview_edits(
        &self,
        edits: Arc<[(Range<Anchor>, String)]>,
        cx: &App,
    ) -> Task<EditPreview> {
        let registry = self.language_registry();
        let language = self.language().cloned();
        let old_snapshot = self.text.snapshot();
        let mut branch_buffer = self.text.branch();
        let mut syntax_snapshot = self.syntax_map.lock().snapshot();
        cx.background_spawn(async move {
            if !edits.is_empty() {
                if let Some(language) = language.clone() {
                    syntax_snapshot.reparse(&old_snapshot, registry.clone(), language);
                }

                branch_buffer.edit(edits.iter().cloned());
                let snapshot = branch_buffer.snapshot();
                syntax_snapshot.interpolate(&snapshot);

                if let Some(language) = language {
                    syntax_snapshot.reparse(&snapshot, registry, language);
                }
            }
            EditPreview {
                old_snapshot,
                applied_edits_snapshot: branch_buffer.snapshot(),
                syntax_snapshot,
            }
        })
    }

    /// Applies all of the changes in this buffer that intersect any of the
    /// given `ranges` to its base buffer.
    ///
    /// If `ranges` is empty, then all changes will be applied. This buffer must
    /// be a branch buffer to call this method.
    pub fn merge_into_base(&mut self, ranges: Vec<Range<usize>>, cx: &mut Context<Self>) {
        let Some(base_buffer) = self.base_buffer() else {
            debug_panic!("not a branch buffer");
            return;
        };

        let mut ranges = if ranges.is_empty() {
            &[0..usize::MAX]
        } else {
            ranges.as_slice()
        }
        .iter()
        .peekable();

        let mut edits = Vec::new();
        for edit in self.edits_since::<usize>(&base_buffer.read(cx).version()) {
            let mut is_included = false;
            while let Some(range) = ranges.peek() {
                if range.end < edit.new.start {
                    ranges.next().unwrap();
                } else {
                    if range.start <= edit.new.end {
                        is_included = true;
                    }
                    break;
                }
            }

            if is_included {
                edits.push((
                    edit.old.clone(),
                    self.text_for_range(edit.new.clone()).collect::<String>(),
                ));
            }
        }

        let operation = base_buffer.update(cx, |base_buffer, cx| {
            // cx.emit(BufferEvent::DiffBaseChanged);
            base_buffer.edit(edits, None, cx)
        });

        if let Some(operation) = operation
            && let Some(BufferBranchState {
                merged_operations, ..
            }) = &mut self.branch_state
        {
            merged_operations.push(operation);
        }
    }

    fn on_base_buffer_event(
        &mut self,
        _: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) {
        let BufferEvent::Operation { operation, .. } = event else {
            return;
        };
        let Some(BufferBranchState {
            merged_operations, ..
        }) = &mut self.branch_state
        else {
            return;
        };

        let mut operation_to_undo = None;
        if let Operation::Buffer(text::Operation::Edit(operation)) = &operation
            && let Ok(ix) = merged_operations.binary_search(&operation.timestamp)
        {
            merged_operations.remove(ix);
            operation_to_undo = Some(operation.timestamp);
        }

        self.apply_ops([operation.clone()], cx);

        if let Some(timestamp) = operation_to_undo {
            let counts = [(timestamp, u32::MAX)].into_iter().collect();
            self.undo_operations(counts, cx);
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
    pub fn saved_mtime(&self) -> Option<MTime> {
        self.saved_mtime
    }

    /// Assign a language to the buffer.
    pub fn set_language(&mut self, language: Option<Arc<Language>>, cx: &mut Context<Self>) {
        self.non_text_state_update_count += 1;
        self.syntax_map.lock().clear(&self.text);
        self.language = language;
        self.was_changed();
        self.reparse(cx);
        cx.emit(BufferEvent::LanguageChanged);
    }

    /// Assign a language registry to the buffer. This allows the buffer to retrieve
    /// other languages if parts of the buffer are written in different languages.
    pub fn set_language_registry(&self, language_registry: Arc<LanguageRegistry>) {
        self.syntax_map
            .lock()
            .set_language_registry(language_registry);
    }

    pub fn language_registry(&self) -> Option<Arc<LanguageRegistry>> {
        self.syntax_map.lock().language_registry()
    }

    /// Assign the line ending type to the buffer.
    pub fn set_line_ending(&mut self, line_ending: LineEnding, cx: &mut Context<Self>) {
        self.text.set_line_ending(line_ending);

        let lamport_timestamp = self.text.lamport_clock.tick();
        self.send_operation(
            Operation::UpdateLineEnding {
                line_ending,
                lamport_timestamp,
            },
            true,
            cx,
        );
    }

    /// Assign the buffer a new [`Capability`].
    pub fn set_capability(&mut self, capability: Capability, cx: &mut Context<Self>) {
        if self.capability != capability {
            self.capability = capability;
            cx.emit(BufferEvent::CapabilityChanged)
        }
    }

    /// This method is called to signal that the buffer has been saved.
    pub fn did_save(
        &mut self,
        version: clock::Global,
        mtime: Option<MTime>,
        cx: &mut Context<Self>,
    ) {
        self.saved_version = version.clone();
        self.has_unsaved_edits.set((version, false));
        self.has_conflict = false;
        self.saved_mtime = mtime;
        self.was_changed();
        cx.emit(BufferEvent::Saved);
        cx.notify();
    }

    /// Reloads the contents of the buffer from disk.
    pub fn reload(&mut self, cx: &Context<Self>) -> oneshot::Receiver<Option<Transaction>> {
        let (tx, rx) = futures::channel::oneshot::channel();
        let prev_version = self.text.version();
        self.reload_task = Some(cx.spawn(async move |this, cx| {
            let Some((new_mtime, new_text)) = this.update(cx, |this, cx| {
                let file = this.file.as_ref()?.as_local()?;

                Some((file.disk_state().mtime(), file.load(cx)))
            })?
            else {
                return Ok(());
            };

            let new_text = new_text.await?;
            let diff = this
                .update(cx, |this, cx| this.diff(new_text.clone(), cx))?
                .await;
            this.update(cx, |this, cx| {
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
        mtime: Option<MTime>,
        cx: &mut Context<Self>,
    ) {
        self.saved_version = version;
        self.has_unsaved_edits
            .set((self.saved_version.clone(), false));
        self.text.set_line_ending(line_ending);
        self.saved_mtime = mtime;
        cx.emit(BufferEvent::Reloaded);
        cx.notify();
    }

    /// Updates the [`File`] backing this buffer. This should be called when
    /// the file has changed or has been deleted.
    pub fn file_updated(&mut self, new_file: Arc<dyn File>, cx: &mut Context<Self>) {
        let was_dirty = self.is_dirty();
        let mut file_changed = false;

        if let Some(old_file) = self.file.as_ref() {
            if new_file.path() != old_file.path() {
                file_changed = true;
            }

            let old_state = old_file.disk_state();
            let new_state = new_file.disk_state();
            if old_state != new_state {
                file_changed = true;
                if !was_dirty && matches!(new_state, DiskState::Present { .. }) {
                    cx.emit(BufferEvent::ReloadNeeded)
                }
            }
        } else {
            file_changed = true;
        };

        self.file = Some(new_file);
        if file_changed {
            self.was_changed();
            self.non_text_state_update_count += 1;
            if was_dirty != self.is_dirty() {
                cx.emit(BufferEvent::DirtyChanged);
            }
            cx.emit(BufferEvent::FileHandleChanged);
            cx.notify();
        }
    }

    pub fn base_buffer(&self) -> Option<Entity<Self>> {
        Some(self.branch_state.as_ref()?.base_buffer.clone())
    }

    /// Returns the primary [`Language`] assigned to this [`Buffer`].
    pub fn language(&self) -> Option<&Arc<Language>> {
        self.language.as_ref()
    }

    /// Returns the [`Language`] at the given location.
    pub fn language_at<D: ToOffset>(&self, position: D) -> Option<Arc<Language>> {
        let offset = position.to_offset(self);
        let mut is_first = true;
        let start_anchor = self.anchor_before(offset);
        let end_anchor = self.anchor_after(offset);
        self.syntax_map
            .lock()
            .layers_for_range(offset..offset, &self.text, false)
            .filter(|layer| {
                if is_first {
                    is_first = false;
                    return true;
                }

                layer
                    .included_sub_ranges
                    .map(|sub_ranges| {
                        sub_ranges.iter().any(|sub_range| {
                            let is_before_start = sub_range.end.cmp(&start_anchor, self).is_lt();
                            let is_after_end = sub_range.start.cmp(&end_anchor, self).is_gt();
                            !is_before_start && !is_after_end
                        })
                    })
                    .unwrap_or(true)
            })
            .last()
            .map(|info| info.language.clone())
            .or_else(|| self.language.clone())
    }

    /// Returns each [`Language`] for the active syntax layers at the given location.
    pub fn languages_at<D: ToOffset>(&self, position: D) -> Vec<Arc<Language>> {
        let offset = position.to_offset(self);
        let mut languages: Vec<Arc<Language>> = self
            .syntax_map
            .lock()
            .layers_for_range(offset..offset, &self.text, false)
            .map(|info| info.language.clone())
            .collect();

        if languages.is_empty()
            && let Some(buffer_language) = self.language()
        {
            languages.push(buffer_language.clone());
        }

        languages
    }

    /// An integer version number that accounts for all updates besides
    /// the buffer's text itself (which is versioned via a version vector).
    pub fn non_text_state_update_count(&self) -> usize {
        self.non_text_state_update_count
    }

    /// Whether the buffer is being parsed in the background.
    #[cfg(any(test, feature = "test-support"))]
    pub fn is_parsing(&self) -> bool {
        self.reparse.is_some()
    }

    /// Indicates whether the buffer contains any regions that may be
    /// written in a language that hasn't been loaded yet.
    pub fn contains_unknown_injections(&self) -> bool {
        self.syntax_map.lock().contains_unknown_injections()
    }

    #[cfg(any(test, feature = "test-support"))]
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
    pub fn reparse(&mut self, cx: &mut Context<Self>) {
        if self.reparse.is_some() {
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

        let parse_task = cx.background_spawn({
            let language = language.clone();
            let language_registry = language_registry.clone();
            async move {
                syntax_snapshot.reparse(&text, language_registry, language);
                syntax_snapshot
            }
        });

        self.parse_status.0.send(ParseStatus::Parsing).unwrap();
        match cx
            .background_executor()
            .block_with_timeout(self.sync_parse_timeout, parse_task)
        {
            Ok(new_syntax_snapshot) => {
                self.did_finish_parsing(new_syntax_snapshot, cx);
                self.reparse = None;
            }
            Err(parse_task) => {
                self.reparse = Some(cx.spawn(async move |this, cx| {
                    let new_syntax_map = parse_task.await;
                    this.update(cx, move |this, cx| {
                        let grammar_changed =
                            this.language.as_ref().is_none_or(|current_language| {
                                !Arc::ptr_eq(&language, current_language)
                            });
                        let language_registry_changed = new_syntax_map
                            .contains_unknown_injections()
                            && language_registry.is_some_and(|registry| {
                                registry.version() != new_syntax_map.language_registry_version()
                            });
                        let parse_again = language_registry_changed
                            || grammar_changed
                            || this.version.changed_since(&parsed_version);
                        this.did_finish_parsing(new_syntax_map, cx);
                        this.reparse = None;
                        if parse_again {
                            this.reparse(cx);
                        }
                    })
                    .ok();
                }));
            }
        }
    }

    fn did_finish_parsing(&mut self, syntax_snapshot: SyntaxSnapshot, cx: &mut Context<Self>) {
        self.was_changed();
        self.non_text_state_update_count += 1;
        self.syntax_map.lock().did_parse(syntax_snapshot);
        self.request_autoindent(cx);
        self.parse_status.0.send(ParseStatus::Idle).unwrap();
        cx.emit(BufferEvent::Reparsed);
        cx.notify();
    }

    pub fn parse_status(&self) -> watch::Receiver<ParseStatus> {
        self.parse_status.1.clone()
    }

    /// Assign to the buffer a set of diagnostics created by a given language server.
    pub fn update_diagnostics(
        &mut self,
        server_id: LanguageServerId,
        diagnostics: DiagnosticSet,
        cx: &mut Context<Self>,
    ) {
        let lamport_timestamp = self.text.lamport_clock.tick();
        let op = Operation::UpdateDiagnostics {
            server_id,
            diagnostics: diagnostics.iter().cloned().collect(),
            lamport_timestamp,
        };

        self.apply_diagnostic_update(server_id, diagnostics, lamport_timestamp, cx);
        self.send_operation(op, true, cx);
    }

    pub fn buffer_diagnostics(
        &self,
        for_server: Option<LanguageServerId>,
    ) -> Vec<&DiagnosticEntry<Anchor>> {
        match for_server {
            Some(server_id) => match self.diagnostics.binary_search_by_key(&server_id, |v| v.0) {
                Ok(idx) => self.diagnostics[idx].1.iter().collect(),
                Err(_) => Vec::new(),
            },
            None => self
                .diagnostics
                .iter()
                .flat_map(|(_, diagnostic_set)| diagnostic_set.iter())
                .collect(),
        }
    }

    fn request_autoindent(&mut self, cx: &mut Context<Self>) {
        if let Some(indent_sizes) = self.compute_autoindents() {
            let indent_sizes = cx.background_spawn(indent_sizes);
            match cx
                .background_executor()
                .block_with_timeout(Duration::from_micros(500), indent_sizes)
            {
                Ok(indent_sizes) => self.apply_autoindents(indent_sizes, cx),
                Err(indent_sizes) => {
                    self.pending_autoindent = Some(cx.spawn(async move |this, cx| {
                        let indent_sizes = indent_sizes.await;
                        this.update(cx, |this, cx| {
                            this.apply_autoindents(indent_sizes, cx);
                        })
                        .ok();
                    }));
                }
            }
        } else {
            self.autoindent_requests.clear();
            for tx in self.wait_for_autoindent_txs.drain(..) {
                tx.send(()).ok();
            }
        }
    }

    fn compute_autoindents(
        &self,
    ) -> Option<impl Future<Output = BTreeMap<u32, IndentSize>> + use<>> {
        let max_rows_between_yields = 100;
        let snapshot = self.snapshot();
        if snapshot.syntax.is_empty() || self.autoindent_requests.is_empty() {
            return None;
        }

        let autoindent_requests = self.autoindent_requests.clone();
        Some(async move {
            let mut indent_sizes = BTreeMap::<u32, (IndentSize, bool)>::new();
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

                // Compute new suggestions for each line, but only include them in the result
                // if they differ from the old suggestion for that line.
                let mut language_indent_sizes = language_indent_sizes_by_new_row.iter().peekable();
                let mut language_indent_size = IndentSize::default();
                for (row_range, original_indent_column) in row_ranges {
                    let new_edited_row_range = if request.is_block_mode {
                        row_range.start..row_range.start + 1
                    } else {
                        row_range.clone()
                    };

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
                                .map(|e| e.0)
                                .unwrap_or_else(|| {
                                    snapshot.indent_size_for_line(suggestion.basis_row)
                                })
                                .with_delta(suggestion.delta, language_indent_size);

                            if old_suggestions.get(&new_row).is_none_or(
                                |(old_indentation, was_within_error)| {
                                    suggested_indent != *old_indentation
                                        && (!suggestion.within_error || *was_within_error)
                                },
                            ) {
                                indent_sizes.insert(
                                    new_row,
                                    (suggested_indent, request.ignore_empty_lines),
                                );
                            }
                        }
                    }

                    if let (true, Some(original_indent_column)) =
                        (request.is_block_mode, original_indent_column)
                    {
                        let new_indent =
                            if let Some((indent, _)) = indent_sizes.get(&row_range.start) {
                                *indent
                            } else {
                                snapshot.indent_size_for_line(row_range.start)
                            };
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
                                    (size, request.ignore_empty_lines)
                                });
                            }
                        }
                    }

                    yield_now().await;
                }
            }

            indent_sizes
                .into_iter()
                .filter_map(|(row, (indent, ignore_empty_lines))| {
                    if ignore_empty_lines && snapshot.line_len(row) == 0 {
                        None
                    } else {
                        Some((row, indent))
                    }
                })
                .collect()
        })
    }

    fn apply_autoindents(
        &mut self,
        indent_sizes: BTreeMap<u32, IndentSize>,
        cx: &mut Context<Self>,
    ) {
        self.autoindent_requests.clear();
        for tx in self.wait_for_autoindent_txs.drain(..) {
            tx.send(()).ok();
        }

        let edits: Vec<_> = indent_sizes
            .into_iter()
            .filter_map(|(row, indent_size)| {
                let current_size = indent_size_for_line(self, row);
                Self::edit_for_indent_size_adjustment(row, current_size, indent_size)
            })
            .collect();

        let preserve_preview = self.preserve_preview();
        self.edit(edits, None, cx);
        if preserve_preview {
            self.refresh_preview();
        }
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
    pub fn diff(&self, mut new_text: String, cx: &App) -> Task<Diff> {
        let old_text = self.as_rope().clone();
        let base_version = self.version();
        cx.background_executor()
            .spawn_labeled(*BUFFER_DIFF_TASK, async move {
                let old_text = old_text.to_string();
                let line_ending = LineEnding::detect(&new_text);
                LineEnding::normalize(&mut new_text);
                let edits = text_diff(&old_text, &new_text);
                Diff {
                    base_version,
                    line_ending,
                    edits,
                }
            })
    }

    /// Spawns a background task that searches the buffer for any whitespace
    /// at the ends of a lines, and returns a `Diff` that removes that whitespace.
    pub fn remove_trailing_whitespace(&self, cx: &App) -> Task<Diff> {
        let old_text = self.as_rope().clone();
        let line_ending = self.line_ending();
        let base_version = self.version();
        cx.background_spawn(async move {
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
    /// no other whitespace. Skips if the buffer is empty.
    pub fn ensure_final_newline(&mut self, cx: &mut Context<Self>) {
        let len = self.len();
        if len == 0 {
            return;
        }
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
    pub fn apply_diff(&mut self, diff: Diff, cx: &mut Context<Self>) -> Option<TransactionId> {
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

    fn has_unsaved_edits(&self) -> bool {
        let (last_version, has_unsaved_edits) = self.has_unsaved_edits.take();

        if last_version == self.version {
            self.has_unsaved_edits
                .set((last_version, has_unsaved_edits));
            return has_unsaved_edits;
        }

        let has_edits = self.has_edits_since(&self.saved_version);
        self.has_unsaved_edits
            .set((self.version.clone(), has_edits));
        has_edits
    }

    /// Checks if the buffer has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        if self.capability == Capability::ReadOnly {
            return false;
        }
        if self.has_conflict {
            return true;
        }
        match self.file.as_ref().map(|f| f.disk_state()) {
            Some(DiskState::New) | Some(DiskState::Deleted) => {
                !self.is_empty() && self.has_unsaved_edits()
            }
            _ => self.has_unsaved_edits(),
        }
    }

    /// Checks if the buffer and its file have both changed since the buffer
    /// was last saved or reloaded.
    pub fn has_conflict(&self) -> bool {
        if self.has_conflict {
            return true;
        }
        let Some(file) = self.file.as_ref() else {
            return false;
        };
        match file.disk_state() {
            DiskState::New => false,
            DiskState::Present { mtime } => match self.saved_mtime {
                Some(saved_mtime) => {
                    mtime.bad_is_greater_than(saved_mtime) && self.has_unsaved_edits()
                }
                None => true,
            },
            DiskState::Deleted => false,
        }
    }

    /// Gets a [`Subscription`] that tracks all of the changes to the buffer's text.
    pub fn subscribe(&mut self) -> Subscription {
        self.text.subscribe()
    }

    /// Adds a bit to the list of bits that are set when the buffer's text changes.
    ///
    /// This allows downstream code to check if the buffer's text has changed without
    /// waiting for an effect cycle, which would be required if using eents.
    pub fn record_changes(&mut self, bit: rc::Weak<Cell<bool>>) {
        if let Err(ix) = self
            .change_bits
            .binary_search_by_key(&rc::Weak::as_ptr(&bit), rc::Weak::as_ptr)
        {
            self.change_bits.insert(ix, bit);
        }
    }

    fn was_changed(&mut self) {
        self.change_bits.retain(|change_bit| {
            change_bit.upgrade().is_some_and(|bit| {
                bit.replace(true);
                true
            })
        });
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
    pub fn end_transaction(&mut self, cx: &mut Context<Self>) -> Option<TransactionId> {
        self.end_transaction_at(Instant::now(), cx)
    }

    /// Terminates the current transaction, providing the current time. Subsequent transactions
    /// that occur within a short period of time will be grouped together. This
    /// is controlled by the buffer's undo grouping duration.
    pub fn end_transaction_at(
        &mut self,
        now: Instant,
        cx: &mut Context<Self>,
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
        self.text.push_empty_transaction(now)
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
    pub fn forget_transaction(&mut self, transaction_id: TransactionId) -> Option<Transaction> {
        self.text.forget_transaction(transaction_id)
    }

    /// Retrieve a transaction from the buffer's undo history
    pub fn get_transaction(&self, transaction_id: TransactionId) -> Option<&Transaction> {
        self.text.get_transaction(transaction_id)
    }

    /// Manually merge two transactions in the buffer's undo history.
    pub fn merge_transactions(&mut self, transaction: TransactionId, destination: TransactionId) {
        self.text.merge_transactions(transaction, destination);
    }

    /// Waits for the buffer to receive operations with the given timestamps.
    pub fn wait_for_edits<It: IntoIterator<Item = clock::Lamport>>(
        &mut self,
        edit_ids: It,
    ) -> impl Future<Output = Result<()>> + use<It> {
        self.text.wait_for_edits(edit_ids)
    }

    /// Waits for the buffer to receive the operations necessary for resolving the given anchors.
    pub fn wait_for_anchors<It: IntoIterator<Item = Anchor>>(
        &mut self,
        anchors: It,
    ) -> impl 'static + Future<Output = Result<()>> + use<It> {
        self.text.wait_for_anchors(anchors)
    }

    /// Waits for the buffer to receive operations up to the given version.
    pub fn wait_for_version(
        &mut self,
        version: clock::Global,
    ) -> impl Future<Output = Result<()>> + use<> {
        self.text.wait_for_version(version)
    }

    /// Forces all futures returned by [`Buffer::wait_for_version`], [`Buffer::wait_for_edits`], or
    /// [`Buffer::wait_for_version`] to resolve with an error.
    pub fn give_up_waiting(&mut self) {
        self.text.give_up_waiting();
    }

    pub fn wait_for_autoindent_applied(&mut self) -> Option<oneshot::Receiver<()>> {
        let mut rx = None;
        if !self.autoindent_requests.is_empty() {
            let channel = oneshot::channel();
            self.wait_for_autoindent_txs.push(channel.0);
            rx = Some(channel.1);
        }
        rx
    }

    /// Stores a set of selections that should be broadcasted to all of the buffer's replicas.
    pub fn set_active_selections(
        &mut self,
        selections: Arc<[Selection<Anchor>]>,
        line_mode: bool,
        cursor_shape: CursorShape,
        cx: &mut Context<Self>,
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
            true,
            cx,
        );
        self.non_text_state_update_count += 1;
        cx.notify();
    }

    /// Clears the selections, so that other replicas of the buffer do not see any selections for
    /// this replica.
    pub fn remove_active_selections(&mut self, cx: &mut Context<Self>) {
        if self
            .remote_selections
            .get(&self.text.replica_id())
            .is_none_or(|set| !set.selections.is_empty())
        {
            self.set_active_selections(Arc::default(), false, Default::default(), cx);
        }
    }

    pub fn set_agent_selections(
        &mut self,
        selections: Arc<[Selection<Anchor>]>,
        line_mode: bool,
        cursor_shape: CursorShape,
        cx: &mut Context<Self>,
    ) {
        let lamport_timestamp = self.text.lamport_clock.tick();
        self.remote_selections.insert(
            ReplicaId::AGENT,
            SelectionSet {
                selections,
                lamport_timestamp,
                line_mode,
                cursor_shape,
            },
        );
        self.non_text_state_update_count += 1;
        cx.notify();
    }

    pub fn remove_agent_selections(&mut self, cx: &mut Context<Self>) {
        self.set_agent_selections(Arc::default(), false, Default::default(), cx);
    }

    /// Replaces the buffer's entire text.
    pub fn set_text<T>(&mut self, text: T, cx: &mut Context<Self>) -> Option<clock::Lamport>
    where
        T: Into<Arc<str>>,
    {
        self.autoindent_requests.clear();
        self.edit([(0..self.len(), text)], None, cx)
    }

    /// Appends the given text to the end of the buffer.
    pub fn append<T>(&mut self, text: T, cx: &mut Context<Self>) -> Option<clock::Lamport>
    where
        T: Into<Arc<str>>,
    {
        self.edit([(self.len()..self.len(), text)], None, cx)
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
        cx: &mut Context<Self>,
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
                if let Some((prev_range, prev_text)) = edits.last_mut()
                    && prev_range.end >= range.start
                {
                    prev_range.end = cmp::max(prev_range.end, range.end);
                    *prev_text = format!("{prev_text}{new_text}").into();
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
            let mut previous_setting = None;
            let entries: Vec<_> = edits
                .into_iter()
                .enumerate()
                .zip(&edit_operation.as_edit().unwrap().new_text)
                .filter(|((_, (range, _)), _)| {
                    let language = before_edit.language_at(range.start);
                    let language_id = language.map(|l| l.id());
                    if let Some((cached_language_id, auto_indent)) = previous_setting
                        && cached_language_id == language_id
                    {
                        auto_indent
                    } else {
                        // The auto-indent setting is not present in editorconfigs, hence
                        // we can avoid passing the file here.
                        let auto_indent =
                            language_settings(language.map(|l| l.name()), None, cx).auto_indent;
                        previous_setting = Some((language_id, auto_indent));
                        auto_indent
                    }
                })
                .map(|((ix, (range, _)), new_text)| {
                    let new_text_length = new_text.len();
                    let old_start = range.start.to_point(&before_edit);
                    let new_start = (delta + range.start as isize) as usize;
                    let range_len = range.end - range.start;
                    delta += new_text_length as isize - range_len as isize;

                    // Decide what range of the insertion to auto-indent, and whether
                    // the first line of the insertion should be considered a newly-inserted line
                    // or an edit to an existing line.
                    let mut range_of_insertion_to_indent = 0..new_text_length;
                    let mut first_line_is_new = true;

                    let old_line_start = before_edit.indent_size_for_line(old_start.row).len;
                    let old_line_end = before_edit.line_len(old_start.row);

                    if old_start.column > old_line_start {
                        first_line_is_new = false;
                    }

                    if !new_text.contains('\n')
                        && (old_start.column + (range_len as u32) < old_line_end
                            || old_line_end == old_line_start)
                    {
                        first_line_is_new = false;
                    }

                    // When inserting text starting with a newline, avoid auto-indenting the
                    // previous line.
                    if new_text.starts_with('\n') {
                        range_of_insertion_to_indent.start += 1;
                        first_line_is_new = true;
                    }

                    let mut original_indent_column = None;
                    if let AutoindentMode::Block {
                        original_indent_columns,
                    } = &mode
                    {
                        original_indent_column = Some(if new_text.starts_with('\n') {
                            indent_size_for_text(
                                new_text[range_of_insertion_to_indent.clone()].chars(),
                            )
                            .len
                        } else {
                            original_indent_columns
                                .get(ix)
                                .copied()
                                .flatten()
                                .unwrap_or_else(|| {
                                    indent_size_for_text(
                                        new_text[range_of_insertion_to_indent.clone()].chars(),
                                    )
                                    .len
                                })
                        });

                        // Avoid auto-indenting the line after the edit.
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

            if !entries.is_empty() {
                self.autoindent_requests.push(Arc::new(AutoindentRequest {
                    before_edit,
                    entries,
                    is_block_mode: matches!(mode, AutoindentMode::Block { .. }),
                    ignore_empty_lines: false,
                }));
            }
        }

        self.end_transaction(cx);
        self.send_operation(Operation::Buffer(edit_operation), true, cx);
        Some(edit_id)
    }

    fn did_edit(&mut self, old_version: &clock::Global, was_dirty: bool, cx: &mut Context<Self>) {
        self.was_changed();

        if self.edits_since::<usize>(old_version).next().is_none() {
            return;
        }

        self.reparse(cx);
        cx.emit(BufferEvent::Edited);
        if was_dirty != self.is_dirty() {
            cx.emit(BufferEvent::DirtyChanged);
        }
        cx.notify();
    }

    pub fn autoindent_ranges<I, T>(&mut self, ranges: I, cx: &mut Context<Self>)
    where
        I: IntoIterator<Item = Range<T>>,
        T: ToOffset + Copy,
    {
        let before_edit = self.snapshot();
        let entries = ranges
            .into_iter()
            .map(|range| AutoindentRequestEntry {
                range: before_edit.anchor_before(range.start)..before_edit.anchor_after(range.end),
                first_line_is_new: true,
                indent_size: before_edit.language_indent_size_at(range.start, cx),
                original_indent_column: None,
            })
            .collect();
        self.autoindent_requests.push(Arc::new(AutoindentRequest {
            before_edit,
            entries,
            is_block_mode: false,
            ignore_empty_lines: true,
        }));
        self.request_autoindent(cx);
    }

    // Inserts newlines at the given position to create an empty line, returning the start of the new line.
    // You can also request the insertion of empty lines above and below the line starting at the returned point.
    pub fn insert_empty_line(
        &mut self,
        position: impl ToPoint,
        space_above: bool,
        space_below: bool,
        cx: &mut Context<Self>,
    ) -> Point {
        let mut position = position.to_point(self);

        self.start_transaction();

        self.edit(
            [(position..position, "\n")],
            Some(AutoindentMode::EachLine),
            cx,
        );

        if position.column > 0 {
            position += Point::new(1, 0);
        }

        if !self.is_line_blank(position.row) {
            self.edit(
                [(position..position, "\n")],
                Some(AutoindentMode::EachLine),
                cx,
            );
        }

        if space_above && position.row > 0 && !self.is_line_blank(position.row - 1) {
            self.edit(
                [(position..position, "\n")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            position.row += 1;
        }

        if space_below
            && (position.row == self.max_point().row || !self.is_line_blank(position.row + 1))
        {
            self.edit(
                [(position..position, "\n")],
                Some(AutoindentMode::EachLine),
                cx,
            );
        }

        self.end_transaction(cx);

        position
    }

    /// Applies the given remote operations to the buffer.
    pub fn apply_ops<I: IntoIterator<Item = Operation>>(&mut self, ops: I, cx: &mut Context<Self>) {
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
        for operation in buffer_ops.iter() {
            self.send_operation(Operation::Buffer(operation.clone()), false, cx);
        }
        self.text.apply_ops(buffer_ops);
        self.deferred_ops.insert(deferred_ops);
        self.flush_deferred_ops(cx);
        self.did_edit(&old_version, was_dirty, cx);
        // Notify independently of whether the buffer was edited as the operations could include a
        // selection update.
        cx.notify();
    }

    fn flush_deferred_ops(&mut self, cx: &mut Context<Self>) {
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

    pub fn has_deferred_ops(&self) -> bool {
        !self.deferred_ops.is_empty() || self.text.has_deferred_ops()
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
            Operation::UpdateCompletionTriggers { .. } | Operation::UpdateLineEnding { .. } => true,
        }
    }

    fn apply_op(&mut self, operation: Operation, cx: &mut Context<Self>) {
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
                if let Some(set) = self.remote_selections.get(&lamport_timestamp.replica_id)
                    && set.lamport_timestamp > lamport_timestamp
                {
                    return;
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
                self.non_text_state_update_count += 1;
            }
            Operation::UpdateCompletionTriggers {
                triggers,
                lamport_timestamp,
                server_id,
            } => {
                if triggers.is_empty() {
                    self.completion_triggers_per_language_server
                        .remove(&server_id);
                    self.completion_triggers = self
                        .completion_triggers_per_language_server
                        .values()
                        .flat_map(|triggers| triggers.iter().cloned())
                        .collect();
                } else {
                    self.completion_triggers_per_language_server
                        .insert(server_id, triggers.iter().cloned().collect());
                    self.completion_triggers.extend(triggers);
                }
                self.text.lamport_clock.observe(lamport_timestamp);
            }
            Operation::UpdateLineEnding {
                line_ending,
                lamport_timestamp,
            } => {
                self.text.set_line_ending(line_ending);
                self.text.lamport_clock.observe(lamport_timestamp);
            }
        }
    }

    fn apply_diagnostic_update(
        &mut self,
        server_id: LanguageServerId,
        diagnostics: DiagnosticSet,
        lamport_timestamp: clock::Lamport,
        cx: &mut Context<Self>,
    ) {
        if lamport_timestamp > self.diagnostics_timestamp {
            let ix = self.diagnostics.binary_search_by_key(&server_id, |e| e.0);
            if diagnostics.is_empty() {
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
            self.non_text_state_update_count += 1;
            self.text.lamport_clock.observe(lamport_timestamp);
            cx.notify();
            cx.emit(BufferEvent::DiagnosticsUpdated);
        }
    }

    fn send_operation(&mut self, operation: Operation, is_local: bool, cx: &mut Context<Self>) {
        self.was_changed();
        cx.emit(BufferEvent::Operation {
            operation,
            is_local,
        });
    }

    /// Removes the selections for a given peer.
    pub fn remove_peer(&mut self, replica_id: ReplicaId, cx: &mut Context<Self>) {
        self.remote_selections.remove(&replica_id);
        cx.notify();
    }

    /// Undoes the most recent transaction.
    pub fn undo(&mut self, cx: &mut Context<Self>) -> Option<TransactionId> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        if let Some((transaction_id, operation)) = self.text.undo() {
            self.send_operation(Operation::Buffer(operation), true, cx);
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
        cx: &mut Context<Self>,
    ) -> bool {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();
        if let Some(operation) = self.text.undo_transaction(transaction_id) {
            self.send_operation(Operation::Buffer(operation), true, cx);
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
        cx: &mut Context<Self>,
    ) -> bool {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let operations = self.text.undo_to_transaction(transaction_id);
        let undone = !operations.is_empty();
        for operation in operations {
            self.send_operation(Operation::Buffer(operation), true, cx);
        }
        if undone {
            self.did_edit(&old_version, was_dirty, cx)
        }
        undone
    }

    pub fn undo_operations(&mut self, counts: HashMap<Lamport, u32>, cx: &mut Context<Buffer>) {
        let was_dirty = self.is_dirty();
        let operation = self.text.undo_operations(counts);
        let old_version = self.version.clone();
        self.send_operation(Operation::Buffer(operation), true, cx);
        self.did_edit(&old_version, was_dirty, cx);
    }

    /// Manually redoes a specific transaction in the buffer's redo history.
    pub fn redo(&mut self, cx: &mut Context<Self>) -> Option<TransactionId> {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        if let Some((transaction_id, operation)) = self.text.redo() {
            self.send_operation(Operation::Buffer(operation), true, cx);
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
        cx: &mut Context<Self>,
    ) -> bool {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let operations = self.text.redo_to_transaction(transaction_id);
        let redone = !operations.is_empty();
        for operation in operations {
            self.send_operation(Operation::Buffer(operation), true, cx);
        }
        if redone {
            self.did_edit(&old_version, was_dirty, cx)
        }
        redone
    }

    /// Override current completion triggers with the user-provided completion triggers.
    pub fn set_completion_triggers(
        &mut self,
        server_id: LanguageServerId,
        triggers: BTreeSet<String>,
        cx: &mut Context<Self>,
    ) {
        self.completion_triggers_timestamp = self.text.lamport_clock.tick();
        if triggers.is_empty() {
            self.completion_triggers_per_language_server
                .remove(&server_id);
            self.completion_triggers = self
                .completion_triggers_per_language_server
                .values()
                .flat_map(|triggers| triggers.iter().cloned())
                .collect();
        } else {
            self.completion_triggers_per_language_server
                .insert(server_id, triggers.clone());
            self.completion_triggers.extend(triggers.iter().cloned());
        }
        self.send_operation(
            Operation::UpdateCompletionTriggers {
                triggers: triggers.into_iter().collect(),
                lamport_timestamp: self.completion_triggers_timestamp,
                server_id,
            },
            true,
            cx,
        );
        cx.notify();
    }

    /// Returns a list of strings which trigger a completion menu for this language.
    /// Usually this is driven by LSP server which returns a list of trigger characters for completions.
    pub fn completion_triggers(&self) -> &BTreeSet<String> {
        &self.completion_triggers
    }

    /// Call this directly after performing edits to prevent the preview tab
    /// from being dismissed by those edits. It causes `should_dismiss_preview`
    /// to return false until there are additional edits.
    pub fn refresh_preview(&mut self) {
        self.preview_version = self.version.clone();
    }

    /// Whether we should preserve the preview status of a tab containing this buffer.
    pub fn preserve_preview(&self) -> bool {
        !self.has_edits_since(&self.preview_version)
    }
}

#[doc(hidden)]
#[cfg(any(test, feature = "test-support"))]
impl Buffer {
    pub fn edit_via_marked_text(
        &mut self,
        marked_string: &str,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut Context<Self>,
    ) {
        let edits = self.edits_for_marked_text(marked_string);
        self.edit(edits, autoindent_mode, cx);
    }

    pub fn set_group_interval(&mut self, group_interval: Duration) {
        self.text.set_group_interval(group_interval);
    }

    pub fn randomly_edit<T>(&mut self, rng: &mut T, old_range_count: usize, cx: &mut Context<Self>)
    where
        T: rand::Rng,
    {
        let mut edits: Vec<(Range<usize>, String)> = Vec::new();
        let mut last_end = None;
        for _ in 0..old_range_count {
            if last_end.is_some_and(|last_end| last_end >= self.len()) {
                break;
            }

            let new_start = last_end.map_or(0, |last_end| last_end + 1);
            let mut range = self.random_byte_range(new_start, rng);
            if rng.random_bool(0.2) {
                mem::swap(&mut range.start, &mut range.end);
            }
            last_end = Some(range.end);

            let new_text_len = rng.random_range(0..10);
            let mut new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();
            new_text = new_text.to_uppercase();

            edits.push((range, new_text));
        }
        log::info!("mutating buffer {:?} with {:?}", self.replica_id(), edits);
        self.edit(edits, None, cx);
    }

    pub fn randomly_undo_redo(&mut self, rng: &mut impl rand::Rng, cx: &mut Context<Self>) {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let ops = self.text.randomly_undo_redo(rng);
        if !ops.is_empty() {
            for op in ops {
                self.send_operation(Operation::Buffer(op), true, cx);
                self.did_edit(&old_version, was_dirty, cx);
            }
        }
    }
}

impl EventEmitter<BufferEvent> for Buffer {}

impl Deref for Buffer {
    type Target = TextBuffer;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl BufferSnapshot {
    /// Returns [`IndentSize`] for a given line that respects user settings and
    /// language preferences.
    pub fn indent_size_for_line(&self, row: u32) -> IndentSize {
        indent_size_for_line(self, row)
    }

    /// Returns [`IndentSize`] for a given position that respects user settings
    /// and language preferences.
    pub fn language_indent_size_at<T: ToOffset>(&self, position: T, cx: &App) -> IndentSize {
        let settings = language_settings(
            self.language_at(position).map(|l| l.name()),
            self.file(),
            cx,
        );
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

        #[derive(Debug, Clone)]
        struct StartPosition {
            start: Point,
            suffix: SharedString,
        }

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
        let mut start_positions = Vec::<StartPosition>::new();
        let mut outdent_positions = Vec::<Point>::new();
        while let Some(mat) = matches.peek() {
            let mut start: Option<Point> = None;
            let mut end: Option<Point> = None;

            let config = indent_configs[mat.grammar_index];
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
                } else if let Some(suffix) = config.suffixed_start_captures.get(&capture.index) {
                    start_positions.push(StartPosition {
                        start: Point::from_ts_point(capture.node.start_position()),
                        suffix: suffix.clone(),
                    });
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
        let mut matches = self
            .syntax
            .matches(range, &self.text, |grammar| grammar.error_query.as_ref());
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
                .next_back()
            {
                range_to_truncate.end = outdent_position;
            }
        }

        start_positions.sort_by_key(|b| b.start);

        // Find the suggested indentation increases and decreased based on regexes.
        let mut regex_outdent_map = HashMap::default();
        let mut last_seen_suffix: HashMap<String, Vec<Point>> = HashMap::default();
        let mut start_positions_iter = start_positions.iter().peekable();

        let mut indent_change_rows = Vec::<(u32, Ordering)>::new();
        self.for_each_line(
            Point::new(prev_non_blank_row.unwrap_or(row_range.start), 0)
                ..Point::new(row_range.end, 0),
            |row, line| {
                if config
                    .decrease_indent_pattern
                    .as_ref()
                    .is_some_and(|regex| regex.is_match(line))
                {
                    indent_change_rows.push((row, Ordering::Less));
                }
                if config
                    .increase_indent_pattern
                    .as_ref()
                    .is_some_and(|regex| regex.is_match(line))
                {
                    indent_change_rows.push((row + 1, Ordering::Greater));
                }
                while let Some(pos) = start_positions_iter.peek() {
                    if pos.start.row < row {
                        let pos = start_positions_iter.next().unwrap();
                        last_seen_suffix
                            .entry(pos.suffix.to_string())
                            .or_default()
                            .push(pos.start);
                    } else {
                        break;
                    }
                }
                for rule in &config.decrease_indent_patterns {
                    if rule.pattern.as_ref().is_some_and(|r| r.is_match(line)) {
                        let row_start_column = self.indent_size_for_line(row).len;
                        let basis_row = rule
                            .valid_after
                            .iter()
                            .filter_map(|valid_suffix| last_seen_suffix.get(valid_suffix))
                            .flatten()
                            .filter(|start_point| start_point.column <= row_start_column)
                            .max_by_key(|start_point| start_point.row);
                        if let Some(outdent_to_row) = basis_row {
                            regex_outdent_map.insert(row, outdent_to_row.row);
                        }
                        break;
                    }
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
            let mut from_regex = false;

            while let Some((indent_row, delta)) = indent_changes.peek() {
                match indent_row.cmp(&row) {
                    Ordering::Equal => match delta {
                        Ordering::Less => {
                            from_regex = true;
                            outdent_from_prev_row = true
                        }
                        Ordering::Greater => {
                            indent_from_prev_row = true;
                            from_regex = true
                        }
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

            if let Some(basis_row) = regex_outdent_map.get(&row) {
                indent_from_prev_row = false;
                outdent_to_row = *basis_row;
                from_regex = true;
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
                    within_error: within_error && !from_regex,
                })
            } else if indent_from_prev_row {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Greater,
                    within_error: within_error && !from_regex,
                })
            } else if outdent_to_row < prev_row {
                Some(IndentSuggestion {
                    basis_row: outdent_to_row,
                    delta: Ordering::Equal,
                    within_error: within_error && !from_regex,
                })
            } else if outdent_from_prev_row {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Less,
                    within_error: within_error && !from_regex,
                })
            } else if config.auto_indent_using_last_non_empty_line || !self.is_line_blank(prev_row)
            {
                Some(IndentSuggestion {
                    basis_row: prev_row,
                    delta: Ordering::Equal,
                    within_error: within_error && !from_regex,
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

    fn get_highlights(&self, range: Range<usize>) -> (SyntaxMapCaptures<'_>, Vec<HighlightMap>) {
        let captures = self.syntax.captures(range, &self.text, |grammar| {
            grammar
                .highlights_config
                .as_ref()
                .map(|config| &config.query)
        });
        let highlight_maps = captures
            .grammars()
            .iter()
            .map(|grammar| grammar.highlight_map())
            .collect();
        (captures, highlight_maps)
    }

    /// Iterates over chunks of text in the given range of the buffer. Text is chunked
    /// in an arbitrary way due to being stored in a [`Rope`](text::Rope). The text is also
    /// returned in chunks where each chunk has a single syntax highlighting style and
    /// diagnostic status.
    pub fn chunks<T: ToOffset>(&self, range: Range<T>, language_aware: bool) -> BufferChunks<'_> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);

        let mut syntax = None;
        if language_aware {
            syntax = Some(self.get_highlights(range.clone()));
        }
        // We want to look at diagnostic spans only when iterating over language-annotated chunks.
        let diagnostics = language_aware;
        BufferChunks::new(self.text.as_rope(), range, syntax, diagnostics, Some(self))
    }

    pub fn highlighted_text_for_range<T: ToOffset>(
        &self,
        range: Range<T>,
        override_style: Option<HighlightStyle>,
        syntax_theme: &SyntaxTheme,
    ) -> HighlightedText {
        HighlightedText::from_buffer_range(
            range,
            &self.text,
            &self.syntax,
            override_style,
            syntax_theme,
        )
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
    pub fn syntax_layers(&self) -> impl Iterator<Item = SyntaxLayer<'_>> + '_ {
        self.syntax_layers_for_range(0..self.len(), true)
    }

    pub fn syntax_layer_at<D: ToOffset>(&self, position: D) -> Option<SyntaxLayer<'_>> {
        let offset = position.to_offset(self);
        self.syntax_layers_for_range(offset..offset, false)
            .filter(|l| l.node().end_byte() > offset)
            .last()
    }

    pub fn syntax_layers_for_range<D: ToOffset>(
        &self,
        range: Range<D>,
        include_hidden: bool,
    ) -> impl Iterator<Item = SyntaxLayer<'_>> + '_ {
        self.syntax
            .layers_for_range(range, &self.text, include_hidden)
    }

    pub fn smallest_syntax_layer_containing<D: ToOffset>(
        &self,
        range: Range<D>,
    ) -> Option<SyntaxLayer<'_>> {
        let range = range.to_offset(self);
        self.syntax
            .layers_for_range(range, &self.text, false)
            .max_by(|a, b| {
                if a.depth != b.depth {
                    a.depth.cmp(&b.depth)
                } else if a.offset.0 != b.offset.0 {
                    a.offset.0.cmp(&b.offset.0)
                } else {
                    a.node().end_byte().cmp(&b.node().end_byte()).reverse()
                }
            })
    }

    /// Returns the main [`Language`].
    pub fn language(&self) -> Option<&Arc<Language>> {
        self.language.as_ref()
    }

    /// Returns the [`Language`] at the given location.
    pub fn language_at<D: ToOffset>(&self, position: D) -> Option<&Arc<Language>> {
        self.syntax_layer_at(position)
            .map(|info| info.language)
            .or(self.language.as_ref())
    }

    /// Returns the settings for the language at the given location.
    pub fn settings_at<'a, D: ToOffset>(
        &'a self,
        position: D,
        cx: &'a App,
    ) -> Cow<'a, LanguageSettings> {
        language_settings(
            self.language_at(position).map(|l| l.name()),
            self.file.as_ref(),
            cx,
        )
    }

    pub fn char_classifier_at<T: ToOffset>(&self, point: T) -> CharClassifier {
        CharClassifier::new(self.language_scope_at(point))
    }

    /// Returns the [`LanguageScope`] at the given location.
    pub fn language_scope_at<D: ToOffset>(&self, position: D) -> Option<LanguageScope> {
        let offset = position.to_offset(self);
        let mut scope = None;
        let mut smallest_range_and_depth: Option<(Range<usize>, usize)> = None;

        // Use the layer that has the smallest node intersecting the given point.
        for layer in self
            .syntax
            .layers_for_range(offset..offset, &self.text, false)
        {
            let mut cursor = layer.node().walk();

            let mut range = None;
            loop {
                let child_range = cursor.node().byte_range();
                if !child_range.contains(&offset) {
                    break;
                }

                range = Some(child_range);
                if cursor.goto_first_child_for_byte(offset).is_none() {
                    break;
                }
            }

            if let Some(range) = range
                && smallest_range_and_depth.as_ref().is_none_or(
                    |(smallest_range, smallest_range_depth)| {
                        if layer.depth > *smallest_range_depth {
                            true
                        } else if layer.depth == *smallest_range_depth {
                            range.len() < smallest_range.len()
                        } else {
                            false
                        }
                    },
                )
            {
                smallest_range_and_depth = Some((range, layer.depth));
                scope = Some(LanguageScope {
                    language: layer.language.clone(),
                    override_id: layer.override_id(offset, &self.text),
                });
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
    pub fn surrounding_word<T: ToOffset>(
        &self,
        start: T,
        scope_context: Option<CharScopeContext>,
    ) -> (Range<usize>, Option<CharKind>) {
        let mut start = start.to_offset(self);
        let mut end = start;
        let mut next_chars = self.chars_at(start).take(128).peekable();
        let mut prev_chars = self.reversed_chars_at(start).take(128).peekable();

        let classifier = self.char_classifier_at(start).scope_context(scope_context);
        let word_kind = cmp::max(
            prev_chars.peek().copied().map(|c| classifier.kind(c)),
            next_chars.peek().copied().map(|c| classifier.kind(c)),
        );

        for ch in prev_chars {
            if Some(classifier.kind(ch)) == word_kind && ch != '\n' {
                start -= ch.len_utf8();
            } else {
                break;
            }
        }

        for ch in next_chars {
            if Some(classifier.kind(ch)) == word_kind && ch != '\n' {
                end += ch.len_utf8();
            } else {
                break;
            }
        }

        (start..end, word_kind)
    }

    /// Moves the TreeCursor to the smallest descendant or ancestor syntax node enclosing the given
    /// range. When `require_larger` is true, the node found must be larger than the query range.
    ///
    /// Returns true if a node was found, and false otherwise. In the `false` case the cursor will
    /// be moved to the root of the tree.
    fn goto_node_enclosing_range(
        cursor: &mut tree_sitter::TreeCursor,
        query_range: &Range<usize>,
        require_larger: bool,
    ) -> bool {
        let mut ascending = false;
        loop {
            let mut range = cursor.node().byte_range();
            if query_range.is_empty() {
                // When the query range is empty and the current node starts after it, move to the
                // previous sibling to find the node the containing node.
                if range.start > query_range.start {
                    cursor.goto_previous_sibling();
                    range = cursor.node().byte_range();
                }
            } else {
                // When the query range is non-empty and the current node ends exactly at the start,
                // move to the next sibling to find a node that extends beyond the start.
                if range.end == query_range.start {
                    cursor.goto_next_sibling();
                    range = cursor.node().byte_range();
                }
            }

            let encloses = range.contains_inclusive(query_range)
                && (!require_larger || range.len() > query_range.len());
            if !encloses {
                ascending = true;
                if !cursor.goto_parent() {
                    return false;
                }
                continue;
            } else if ascending {
                return true;
            }

            // Descend into the current node.
            if cursor
                .goto_first_child_for_byte(query_range.start)
                .is_none()
            {
                return true;
            }
        }
    }

    pub fn syntax_ancestor<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> Option<tree_sitter::Node<'a>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut result: Option<tree_sitter::Node<'a>> = None;
        for layer in self
            .syntax
            .layers_for_range(range.clone(), &self.text, true)
        {
            let mut cursor = layer.node().walk();

            // Find the node that both contains the range and is larger than it.
            if !Self::goto_node_enclosing_range(&mut cursor, &range, true) {
                continue;
            }

            let left_node = cursor.node();
            let mut layer_result = left_node;

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
                if let Some(right_node) = right_node
                    && (right_node.is_named() || !left_node.is_named())
                {
                    layer_result = right_node;
                }
            }

            if let Some(previous_result) = &result
                && previous_result.byte_range().len() < layer_result.byte_range().len()
            {
                continue;
            }
            result = Some(layer_result);
        }

        result
    }

    /// Find the previous sibling syntax node at the given range.
    ///
    /// This function locates the syntax node that precedes the node containing
    /// the given range. It searches hierarchically by:
    /// 1. Finding the node that contains the given range
    /// 2. Looking for the previous sibling at the same tree level
    /// 3. If no sibling is found, moving up to parent levels and searching for siblings
    ///
    /// Returns `None` if there is no previous sibling at any ancestor level.
    pub fn syntax_prev_sibling<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> Option<tree_sitter::Node<'a>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut result: Option<tree_sitter::Node<'a>> = None;

        for layer in self
            .syntax
            .layers_for_range(range.clone(), &self.text, true)
        {
            let mut cursor = layer.node().walk();

            // Find the node that contains the range
            if !Self::goto_node_enclosing_range(&mut cursor, &range, false) {
                continue;
            }

            // Look for the previous sibling, moving up ancestor levels if needed
            loop {
                if cursor.goto_previous_sibling() {
                    let layer_result = cursor.node();

                    if let Some(previous_result) = &result {
                        if previous_result.byte_range().end < layer_result.byte_range().end {
                            continue;
                        }
                    }
                    result = Some(layer_result);
                    break;
                }

                // No sibling found at this level, try moving up to parent
                if !cursor.goto_parent() {
                    break;
                }
            }
        }

        result
    }

    /// Find the next sibling syntax node at the given range.
    ///
    /// This function locates the syntax node that follows the node containing
    /// the given range. It searches hierarchically by:
    /// 1. Finding the node that contains the given range
    /// 2. Looking for the next sibling at the same tree level
    /// 3. If no sibling is found, moving up to parent levels and searching for siblings
    ///
    /// Returns `None` if there is no next sibling at any ancestor level.
    pub fn syntax_next_sibling<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> Option<tree_sitter::Node<'a>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut result: Option<tree_sitter::Node<'a>> = None;

        for layer in self
            .syntax
            .layers_for_range(range.clone(), &self.text, true)
        {
            let mut cursor = layer.node().walk();

            // Find the node that contains the range
            if !Self::goto_node_enclosing_range(&mut cursor, &range, false) {
                continue;
            }

            // Look for the next sibling, moving up ancestor levels if needed
            loop {
                if cursor.goto_next_sibling() {
                    let layer_result = cursor.node();

                    if let Some(previous_result) = &result {
                        if previous_result.byte_range().start > layer_result.byte_range().start {
                            continue;
                        }
                    }
                    result = Some(layer_result);
                    break;
                }

                // No sibling found at this level, try moving up to parent
                if !cursor.goto_parent() {
                    break;
                }
            }
        }

        result
    }

    /// Returns the root syntax node within the given row
    pub fn syntax_root_ancestor(&self, position: Anchor) -> Option<tree_sitter::Node<'_>> {
        let start_offset = position.to_offset(self);

        let row = self.summary_for_anchor::<text::PointUtf16>(&position).row as usize;

        let layer = self
            .syntax
            .layers_for_range(start_offset..start_offset, &self.text, true)
            .next()?;

        let mut cursor = layer.node().walk();

        // Descend to the first leaf that touches the start of the range.
        while cursor.goto_first_child_for_byte(start_offset).is_some() {
            if cursor.node().end_byte() == start_offset {
                cursor.goto_next_sibling();
            }
        }

        // Ascend to the root node within the same row.
        while cursor.goto_parent() {
            if cursor.node().start_position().row != row {
                break;
            }
        }

        Some(cursor.node())
    }

    /// Returns the outline for the buffer.
    ///
    /// This method allows passing an optional [`SyntaxTheme`] to
    /// syntax-highlight the returned symbols.
    pub fn outline(&self, theme: Option<&SyntaxTheme>) -> Outline<Anchor> {
        Outline::new(self.outline_items_containing(0..self.len(), true, theme))
    }

    /// Returns all the symbols that contain the given position.
    ///
    /// This method allows passing an optional [`SyntaxTheme`] to
    /// syntax-highlight the returned symbols.
    pub fn symbols_containing<T: ToOffset>(
        &self,
        position: T,
        theme: Option<&SyntaxTheme>,
    ) -> Vec<OutlineItem<Anchor>> {
        let position = position.to_offset(self);
        let start = self.clip_offset(position.saturating_sub(1), Bias::Left);
        let end = self.clip_offset(position + 1, Bias::Right);
        let mut items = self.outline_items_containing(start..end, false, theme);
        let mut prev_depth = None;
        items.retain(|item| {
            let result = prev_depth.is_none_or(|prev_depth| item.depth > prev_depth);
            prev_depth = Some(item.depth);
            result
        });
        items
    }

    pub fn outline_range_containing<T: ToOffset>(&self, range: Range<T>) -> Option<Range<Point>> {
        let range = range.to_offset(self);
        let mut matches = self.syntax.matches(range.clone(), &self.text, |grammar| {
            grammar.outline_config.as_ref().map(|c| &c.query)
        });
        let configs = matches
            .grammars()
            .iter()
            .map(|g| g.outline_config.as_ref().unwrap())
            .collect::<Vec<_>>();

        while let Some(mat) = matches.peek() {
            let config = &configs[mat.grammar_index];
            let containing_item_node = maybe!({
                let item_node = mat.captures.iter().find_map(|cap| {
                    if cap.index == config.item_capture_ix {
                        Some(cap.node)
                    } else {
                        None
                    }
                })?;

                let item_byte_range = item_node.byte_range();
                if item_byte_range.end < range.start || item_byte_range.start > range.end {
                    None
                } else {
                    Some(item_node)
                }
            });

            if let Some(item_node) = containing_item_node {
                return Some(
                    Point::from_ts_point(item_node.start_position())
                        ..Point::from_ts_point(item_node.end_position()),
                );
            }

            matches.advance();
        }
        None
    }

    pub fn outline_items_containing<T: ToOffset>(
        &self,
        range: Range<T>,
        include_extra_context: bool,
        theme: Option<&SyntaxTheme>,
    ) -> Vec<OutlineItem<Anchor>> {
        let range = range.to_offset(self);
        let mut matches = self.syntax.matches(range.clone(), &self.text, |grammar| {
            grammar.outline_config.as_ref().map(|c| &c.query)
        });

        let mut items = Vec::new();
        let mut annotation_row_ranges: Vec<Range<u32>> = Vec::new();
        while let Some(mat) = matches.peek() {
            let config = matches.grammars()[mat.grammar_index]
                .outline_config
                .as_ref()
                .unwrap();
            if let Some(item) =
                self.next_outline_item(config, &mat, &range, include_extra_context, theme)
            {
                items.push(item);
            } else if let Some(capture) = mat
                .captures
                .iter()
                .find(|capture| Some(capture.index) == config.annotation_capture_ix)
            {
                let capture_range = capture.node.start_position()..capture.node.end_position();
                let mut capture_row_range =
                    capture_range.start.row as u32..capture_range.end.row as u32;
                if capture_range.end.row > capture_range.start.row && capture_range.end.column == 0
                {
                    capture_row_range.end -= 1;
                }
                if let Some(last_row_range) = annotation_row_ranges.last_mut() {
                    if last_row_range.end >= capture_row_range.start.saturating_sub(1) {
                        last_row_range.end = capture_row_range.end;
                    } else {
                        annotation_row_ranges.push(capture_row_range);
                    }
                } else {
                    annotation_row_ranges.push(capture_row_range);
                }
            }
            matches.advance();
        }

        items.sort_by_key(|item| (item.range.start, Reverse(item.range.end)));

        // Assign depths based on containment relationships and convert to anchors.
        let mut item_ends_stack = Vec::<Point>::new();
        let mut anchor_items = Vec::new();
        let mut annotation_row_ranges = annotation_row_ranges.into_iter().peekable();
        for item in items {
            while let Some(last_end) = item_ends_stack.last().copied() {
                if last_end < item.range.end {
                    item_ends_stack.pop();
                } else {
                    break;
                }
            }

            let mut annotation_row_range = None;
            while let Some(next_annotation_row_range) = annotation_row_ranges.peek() {
                let row_preceding_item = item.range.start.row.saturating_sub(1);
                if next_annotation_row_range.end < row_preceding_item {
                    annotation_row_ranges.next();
                } else {
                    if next_annotation_row_range.end == row_preceding_item {
                        annotation_row_range = Some(next_annotation_row_range.clone());
                        annotation_row_ranges.next();
                    }
                    break;
                }
            }

            anchor_items.push(OutlineItem {
                depth: item_ends_stack.len(),
                range: self.anchor_after(item.range.start)..self.anchor_before(item.range.end),
                text: item.text,
                highlight_ranges: item.highlight_ranges,
                name_ranges: item.name_ranges,
                body_range: item
                    .body_range
                    .map(|r| self.anchor_after(r.start)..self.anchor_before(r.end)),
                annotation_range: annotation_row_range.map(|annotation_range| {
                    self.anchor_after(Point::new(annotation_range.start, 0))
                        ..self.anchor_before(Point::new(
                            annotation_range.end,
                            self.line_len(annotation_range.end),
                        ))
                }),
            });
            item_ends_stack.push(item.range.end);
        }

        anchor_items
    }

    fn next_outline_item(
        &self,
        config: &OutlineConfig,
        mat: &SyntaxMapMatch,
        range: &Range<usize>,
        include_extra_context: bool,
        theme: Option<&SyntaxTheme>,
    ) -> Option<OutlineItem<Point>> {
        let item_node = mat.captures.iter().find_map(|cap| {
            if cap.index == config.item_capture_ix {
                Some(cap.node)
            } else {
                None
            }
        })?;

        let item_byte_range = item_node.byte_range();
        if item_byte_range.end < range.start || item_byte_range.start > range.end {
            return None;
        }
        let item_point_range = Point::from_ts_point(item_node.start_position())
            ..Point::from_ts_point(item_node.end_position());

        let mut open_point = None;
        let mut close_point = None;

        let mut buffer_ranges = Vec::new();
        let mut add_to_buffer_ranges = |node: tree_sitter::Node, node_is_name| {
            let mut range = node.start_byte()..node.end_byte();
            let start = node.start_position();
            if node.end_position().row > start.row {
                range.end = range.start + self.line_len(start.row as u32) as usize - start.column;
            }

            if !range.is_empty() {
                buffer_ranges.push((range, node_is_name));
            }
        };

        for capture in mat.captures {
            if capture.index == config.name_capture_ix {
                add_to_buffer_ranges(capture.node, true);
            } else if Some(capture.index) == config.context_capture_ix
                || (Some(capture.index) == config.extra_context_capture_ix && include_extra_context)
            {
                add_to_buffer_ranges(capture.node, false);
            } else {
                if Some(capture.index) == config.open_capture_ix {
                    open_point = Some(Point::from_ts_point(capture.node.end_position()));
                } else if Some(capture.index) == config.close_capture_ix {
                    close_point = Some(Point::from_ts_point(capture.node.start_position()));
                }
            }
        }

        if buffer_ranges.is_empty() {
            return None;
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
            let space_added = !text.is_empty() && buffer_range.start > last_buffer_range_end;
            if space_added {
                text.push(' ');
            }
            let before_append_len = text.len();
            let mut offset = buffer_range.start;
            chunks.seek(buffer_range.clone());
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
            if is_name {
                let after_append_len = text.len();
                let start = if space_added && !name_ranges.is_empty() {
                    before_append_len - 1
                } else {
                    before_append_len
                };
                name_ranges.push(start..after_append_len);
            }
            last_buffer_range_end = buffer_range.end;
        }

        Some(OutlineItem {
            depth: 0, // We'll calculate the depth later
            range: item_point_range,
            text,
            highlight_ranges,
            name_ranges,
            body_range: open_point.zip(close_point).map(|(start, end)| start..end),
            annotation_range: None,
        })
    }

    pub fn function_body_fold_ranges<T: ToOffset>(
        &self,
        within: Range<T>,
    ) -> impl Iterator<Item = Range<usize>> + '_ {
        self.text_object_ranges(within, TreeSitterOptions::default())
            .filter_map(|(range, obj)| (obj == TextObject::InsideFunction).then_some(range))
    }

    /// For each grammar in the language, runs the provided
    /// [`tree_sitter::Query`] against the given range.
    pub fn matches(
        &self,
        range: Range<usize>,
        query: fn(&Grammar) -> Option<&tree_sitter::Query>,
    ) -> SyntaxMapMatches<'_> {
        self.syntax.matches(range, self, query)
    }

    pub fn all_bracket_ranges(
        &self,
        range: Range<usize>,
    ) -> impl Iterator<Item = BracketMatch> + '_ {
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
                let pattern = &config.patterns[mat.pattern_index];
                for capture in mat.captures {
                    if capture.index == config.open_capture_ix {
                        open = Some(capture.node.byte_range());
                    } else if capture.index == config.close_capture_ix {
                        close = Some(capture.node.byte_range());
                    }
                }

                matches.advance();

                let Some((open_range, close_range)) = open.zip(close) else {
                    continue;
                };

                let bracket_range = open_range.start..=close_range.end;
                if !bracket_range.overlaps(&range) {
                    continue;
                }

                return Some(BracketMatch {
                    open_range,
                    close_range,
                    newline_only: pattern.newline_only,
                });
            }
            None
        })
    }

    /// Returns bracket range pairs overlapping or adjacent to `range`
    pub fn bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = BracketMatch> + '_ {
        // Find bracket pairs that *inclusively* contain the given range.
        let range = range.start.to_previous_offset(self)..range.end.to_next_offset(self);
        self.all_bracket_ranges(range)
            .filter(|pair| !pair.newline_only)
    }

    pub fn debug_variables_query<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = (Range<usize>, DebuggerTextObject)> + '_ {
        let range = range.start.to_previous_offset(self)..range.end.to_next_offset(self);

        let mut matches = self.syntax.matches_with_options(
            range.clone(),
            &self.text,
            TreeSitterOptions::default(),
            |grammar| grammar.debug_variables_config.as_ref().map(|c| &c.query),
        );

        let configs = matches
            .grammars()
            .iter()
            .map(|grammar| grammar.debug_variables_config.as_ref())
            .collect::<Vec<_>>();

        let mut captures = Vec::<(Range<usize>, DebuggerTextObject)>::new();

        iter::from_fn(move || {
            loop {
                while let Some(capture) = captures.pop() {
                    if capture.0.overlaps(&range) {
                        return Some(capture);
                    }
                }

                let mat = matches.peek()?;

                let Some(config) = configs[mat.grammar_index].as_ref() else {
                    matches.advance();
                    continue;
                };

                for capture in mat.captures {
                    let Some(ix) = config
                        .objects_by_capture_ix
                        .binary_search_by_key(&capture.index, |e| e.0)
                        .ok()
                    else {
                        continue;
                    };
                    let text_object = config.objects_by_capture_ix[ix].1;
                    let byte_range = capture.node.byte_range();

                    let mut found = false;
                    for (range, existing) in captures.iter_mut() {
                        if existing == &text_object {
                            range.start = range.start.min(byte_range.start);
                            range.end = range.end.max(byte_range.end);
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        captures.push((byte_range, text_object));
                    }
                }

                matches.advance();
            }
        })
    }

    pub fn text_object_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
        options: TreeSitterOptions,
    ) -> impl Iterator<Item = (Range<usize>, TextObject)> + '_ {
        let range =
            range.start.to_previous_offset(self)..self.len().min(range.end.to_next_offset(self));

        let mut matches =
            self.syntax
                .matches_with_options(range.clone(), &self.text, options, |grammar| {
                    grammar.text_object_config.as_ref().map(|c| &c.query)
                });

        let configs = matches
            .grammars()
            .iter()
            .map(|grammar| grammar.text_object_config.as_ref())
            .collect::<Vec<_>>();

        let mut captures = Vec::<(Range<usize>, TextObject)>::new();

        iter::from_fn(move || {
            loop {
                while let Some(capture) = captures.pop() {
                    if capture.0.overlaps(&range) {
                        return Some(capture);
                    }
                }

                let mat = matches.peek()?;

                let Some(config) = configs[mat.grammar_index].as_ref() else {
                    matches.advance();
                    continue;
                };

                for capture in mat.captures {
                    let Some(ix) = config
                        .text_objects_by_capture_ix
                        .binary_search_by_key(&capture.index, |e| e.0)
                        .ok()
                    else {
                        continue;
                    };
                    let text_object = config.text_objects_by_capture_ix[ix].1;
                    let byte_range = capture.node.byte_range();

                    let mut found = false;
                    for (range, existing) in captures.iter_mut() {
                        if existing == &text_object {
                            range.start = range.start.min(byte_range.start);
                            range.end = range.end.max(byte_range.end);
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        captures.push((byte_range, text_object));
                    }
                }

                matches.advance();
            }
        })
    }

    /// Returns enclosing bracket ranges containing the given range
    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = BracketMatch> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);

        self.bracket_ranges(range.clone()).filter(move |pair| {
            pair.open_range.start <= range.start && pair.close_range.end >= range.end
        })
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

        for pair in self.enclosing_bracket_ranges(range) {
            if let Some(range_filter) = range_filter
                && !range_filter(pair.open_range.clone(), pair.close_range.clone())
            {
                continue;
            }

            let len = pair.close_range.end - pair.open_range.start;

            if let Some((existing_open, existing_close)) = &result {
                let existing_len = existing_close.end - existing_open.start;
                if len > existing_len {
                    continue;
                }
            }

            result = Some((pair.open_range, pair.close_range));
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

    pub fn injections_intersecting_range<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = (Range<usize>, &Arc<Language>)> + '_ {
        let offset_range = range.start.to_offset(self)..range.end.to_offset(self);

        let mut syntax_matches = self.syntax.matches(offset_range, self, |grammar| {
            grammar
                .injection_config
                .as_ref()
                .map(|config| &config.query)
        });

        let configs = syntax_matches
            .grammars()
            .iter()
            .map(|grammar| grammar.injection_config.as_ref())
            .collect::<Vec<_>>();

        iter::from_fn(move || {
            let ranges = syntax_matches.peek().and_then(|mat| {
                let config = &configs[mat.grammar_index]?;
                let content_capture_range = mat.captures.iter().find_map(|capture| {
                    if capture.index == config.content_capture_ix {
                        Some(capture.node.byte_range())
                    } else {
                        None
                    }
                })?;
                let language = self.language_at(content_capture_range.start)?;
                Some((content_capture_range, language))
            });
            syntax_matches.advance();
            ranges
        })
    }

    pub fn runnable_ranges(
        &self,
        offset_range: Range<usize>,
    ) -> impl Iterator<Item = RunnableRange> + '_ {
        let mut syntax_matches = self.syntax.matches(offset_range, self, |grammar| {
            grammar.runnable_config.as_ref().map(|config| &config.query)
        });

        let test_configs = syntax_matches
            .grammars()
            .iter()
            .map(|grammar| grammar.runnable_config.as_ref())
            .collect::<Vec<_>>();

        iter::from_fn(move || {
            loop {
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
                                self.text_for_range(range).collect::<String>(),
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
            }
        })
    }

    /// Returns selections for remote peers intersecting the given range.
    #[allow(clippy::type_complexity)]
    pub fn selections_in_range(
        &self,
        range: Range<Anchor>,
        include_local: bool,
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
            .filter(move |(replica_id, set)| {
                (include_local || **replica_id != self.text.replica_id())
                    && !set.selections.is_empty()
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

    /// Returns if the buffer contains any diagnostics.
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Returns all the diagnostics intersecting the given range.
    pub fn diagnostics_in_range<'a, T, O>(
        &'a self,
        search_range: Range<T>,
        reversed: bool,
    ) -> impl 'a + Iterator<Item = DiagnosticEntryRef<'a, O>>
    where
        T: 'a + Clone + ToOffset,
        O: 'a + FromAnchor,
    {
        let mut iterators: Vec<_> = self
            .diagnostics
            .iter()
            .map(|(_, collection)| {
                collection
                    .range::<T, text::Anchor>(search_range.clone(), self, true, reversed)
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
                        .cmp(&b.range.start, self)
                        // when range is equal, sort by diagnostic severity
                        .then(a.diagnostic.severity.cmp(&b.diagnostic.severity))
                        // and stabilize order with group_id
                        .then(a.diagnostic.group_id.cmp(&b.diagnostic.group_id));
                    if reversed { cmp.reverse() } else { cmp }
                })?;
            iterators[next_ix]
                .next()
                .map(
                    |DiagnosticEntryRef { range, diagnostic }| DiagnosticEntryRef {
                        diagnostic,
                        range: FromAnchor::from_anchor(&range.start, self)
                            ..FromAnchor::from_anchor(&range.end, self),
                    },
                )
        })
    }

    /// Raw access to the diagnostic sets. Typically `diagnostic_groups` or `diagnostic_group`
    /// should be used instead.
    pub fn diagnostic_sets(&self) -> &SmallVec<[(LanguageServerId, DiagnosticSet); 2]> {
        &self.diagnostics
    }

    /// Returns all the diagnostic groups associated with the given
    /// language server ID. If no language server ID is provided,
    /// all diagnostics groups are returned.
    pub fn diagnostic_groups(
        &self,
        language_server_id: Option<LanguageServerId>,
    ) -> Vec<(LanguageServerId, DiagnosticGroup<'_, Anchor>)> {
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
    pub fn diagnostic_group<O>(
        &self,
        group_id: usize,
    ) -> impl Iterator<Item = DiagnosticEntryRef<'_, O>> + use<'_, O>
    where
        O: FromAnchor + 'static,
    {
        self.diagnostics
            .iter()
            .flat_map(move |(_, set)| set.group(group_id, self))
    }

    /// An integer version number that accounts for all updates besides
    /// the buffer's text itself (which is versioned via a version vector).
    pub fn non_text_state_update_count(&self) -> usize {
        self.non_text_state_update_count
    }

    /// An integer version that changes when the buffer's syntax changes.
    pub fn syntax_update_count(&self) -> usize {
        self.syntax.update_count()
    }

    /// Returns a snapshot of underlying file.
    pub fn file(&self) -> Option<&Arc<dyn File>> {
        self.file.as_ref()
    }

    pub fn resolve_file_path(&self, include_root: bool, cx: &App) -> Option<String> {
        if let Some(file) = self.file() {
            if file.path().file_name().is_none() || include_root {
                Some(file.full_path(cx).to_string_lossy().into_owned())
            } else {
                Some(file.path().display(file.path_style(cx)).to_string())
            }
        } else {
            None
        }
    }

    pub fn words_in_range(&self, query: WordsQuery) -> BTreeMap<String, Range<Anchor>> {
        let query_str = query.fuzzy_contents;
        if query_str.is_some_and(|query| query.is_empty()) {
            return BTreeMap::default();
        }

        let classifier = CharClassifier::new(self.language.clone().map(|language| LanguageScope {
            language,
            override_id: None,
        }));

        let mut query_ix = 0;
        let query_chars = query_str.map(|query| query.chars().collect::<Vec<_>>());
        let query_len = query_chars.as_ref().map_or(0, |query| query.len());

        let mut words = BTreeMap::default();
        let mut current_word_start_ix = None;
        let mut chunk_ix = query.range.start;
        for chunk in self.chunks(query.range, false) {
            for (i, c) in chunk.text.char_indices() {
                let ix = chunk_ix + i;
                if classifier.is_word(c) {
                    if current_word_start_ix.is_none() {
                        current_word_start_ix = Some(ix);
                    }

                    if let Some(query_chars) = &query_chars
                        && query_ix < query_len
                        && c.to_lowercase().eq(query_chars[query_ix].to_lowercase())
                    {
                        query_ix += 1;
                    }
                    continue;
                } else if let Some(word_start) = current_word_start_ix.take()
                    && query_ix == query_len
                {
                    let word_range = self.anchor_before(word_start)..self.anchor_after(ix);
                    let mut word_text = self.text_for_range(word_start..ix).peekable();
                    let first_char = word_text
                        .peek()
                        .and_then(|first_chunk| first_chunk.chars().next());
                    // Skip empty and "words" starting with digits as a heuristic to reduce useless completions
                    if !query.skip_digits
                        || first_char.is_none_or(|first_char| !first_char.is_digit(10))
                    {
                        words.insert(word_text.collect(), word_range);
                    }
                }
                query_ix = 0;
            }
            chunk_ix += chunk.text.len();
        }

        words
    }
}

pub struct WordsQuery<'a> {
    /// Only returns words with all chars from the fuzzy string in them.
    pub fuzzy_contents: Option<&'a str>,
    /// Skips words that start with a digit.
    pub skip_digits: bool,
    /// Buffer offset range, to look for words.
    pub range: Range<usize>,
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
            syntax: self.syntax.clone(),
            file: self.file.clone(),
            remote_selections: self.remote_selections.clone(),
            diagnostics: self.diagnostics.clone(),
            language: self.language.clone(),
            non_text_state_update_count: self.non_text_state_update_count,
        }
    }
}

impl Deref for BufferSnapshot {
    type Target = text::BufferSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

unsafe impl Send for BufferChunks<'_> {}

impl<'a> BufferChunks<'a> {
    pub(crate) fn new(
        text: &'a Rope,
        range: Range<usize>,
        syntax: Option<(SyntaxMapCaptures<'a>, Vec<HighlightMap>)>,
        diagnostics: bool,
        buffer_snapshot: Option<&'a BufferSnapshot>,
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

        let diagnostic_endpoints = diagnostics.then(|| Vec::new().into_iter().peekable());
        let chunks = text.chunks_in_range(range.clone());

        let mut this = BufferChunks {
            range,
            buffer_snapshot,
            chunks,
            diagnostic_endpoints,
            error_depth: 0,
            warning_depth: 0,
            information_depth: 0,
            hint_depth: 0,
            unnecessary_depth: 0,
            underline: true,
            highlights,
        };
        this.initialize_diagnostic_endpoints();
        this
    }

    /// Seeks to the given byte offset in the buffer.
    pub fn seek(&mut self, range: Range<usize>) {
        let old_range = std::mem::replace(&mut self.range, range.clone());
        self.chunks.set_range(self.range.clone());
        if let Some(highlights) = self.highlights.as_mut() {
            if old_range.start <= self.range.start && old_range.end >= self.range.end {
                // Reuse existing highlights stack, as the new range is a subrange of the old one.
                highlights
                    .stack
                    .retain(|(end_offset, _)| *end_offset > range.start);
                if let Some(capture) = &highlights.next_capture
                    && range.start >= capture.node.start_byte()
                {
                    let next_capture_end = capture.node.end_byte();
                    if range.start < next_capture_end {
                        highlights.stack.push((
                            next_capture_end,
                            highlights.highlight_maps[capture.grammar_index].get(capture.index),
                        ));
                    }
                    highlights.next_capture.take();
                }
            } else if let Some(snapshot) = self.buffer_snapshot {
                let (captures, highlight_maps) = snapshot.get_highlights(self.range.clone());
                *highlights = BufferChunkHighlights {
                    captures,
                    next_capture: None,
                    stack: Default::default(),
                    highlight_maps,
                };
            } else {
                // We cannot obtain new highlights for a language-aware buffer iterator, as we don't have a buffer snapshot.
                // Seeking such BufferChunks is not supported.
                debug_assert!(
                    false,
                    "Attempted to seek on a language-aware buffer iterator without associated buffer snapshot"
                );
            }

            highlights.captures.set_byte_range(self.range.clone());
            self.initialize_diagnostic_endpoints();
        }
    }

    fn initialize_diagnostic_endpoints(&mut self) {
        if let Some(diagnostics) = self.diagnostic_endpoints.as_mut()
            && let Some(buffer) = self.buffer_snapshot
        {
            let mut diagnostic_endpoints = Vec::new();
            for entry in buffer.diagnostics_in_range::<_, usize>(self.range.clone(), false) {
                diagnostic_endpoints.push(DiagnosticEndpoint {
                    offset: entry.range.start,
                    is_start: true,
                    severity: entry.diagnostic.severity,
                    is_unnecessary: entry.diagnostic.is_unnecessary,
                    underline: entry.diagnostic.underline,
                });
                diagnostic_endpoints.push(DiagnosticEndpoint {
                    offset: entry.range.end,
                    is_start: false,
                    severity: entry.diagnostic.severity,
                    is_unnecessary: entry.diagnostic.is_unnecessary,
                    underline: entry.diagnostic.underline,
                });
            }
            diagnostic_endpoints
                .sort_unstable_by_key(|endpoint| (endpoint.offset, !endpoint.is_start));
            *diagnostics = diagnostic_endpoints.into_iter().peekable();
            self.hint_depth = 0;
            self.error_depth = 0;
            self.warning_depth = 0;
            self.information_depth = 0;
        }
    }

    /// The current byte offset in the buffer.
    pub fn offset(&self) -> usize {
        self.range.start
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
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

        let mut diagnostic_endpoints = std::mem::take(&mut self.diagnostic_endpoints);
        if let Some(diagnostic_endpoints) = diagnostic_endpoints.as_mut() {
            while let Some(endpoint) = diagnostic_endpoints.peek().copied() {
                if endpoint.offset <= self.range.start {
                    self.update_diagnostic_depths(endpoint);
                    diagnostic_endpoints.next();
                    self.underline = endpoint.underline;
                } else {
                    next_diagnostic_endpoint = endpoint.offset;
                    break;
                }
            }
        }
        self.diagnostic_endpoints = diagnostic_endpoints;

        if let Some(ChunkBitmaps {
            text: chunk,
            chars: chars_map,
            tabs,
        }) = self.chunks.peek_tabs()
        {
            let chunk_start = self.range.start;
            let mut chunk_end = (self.chunks.offset() + chunk.len())
                .min(next_capture_start)
                .min(next_diagnostic_endpoint);
            let mut highlight_id = None;
            if let Some(highlights) = self.highlights.as_ref()
                && let Some((parent_capture_end, parent_highlight_id)) = highlights.stack.last()
            {
                chunk_end = chunk_end.min(*parent_capture_end);
                highlight_id = Some(*parent_highlight_id);
            }

            let slice =
                &chunk[chunk_start - self.chunks.offset()..chunk_end - self.chunks.offset()];
            let bit_end = chunk_end - self.chunks.offset();

            let mask = if bit_end >= 128 {
                u128::MAX
            } else {
                (1u128 << bit_end) - 1
            };
            let tabs = (tabs >> (chunk_start - self.chunks.offset())) & mask;
            let chars_map = (chars_map >> (chunk_start - self.chunks.offset())) & mask;

            self.range.start = chunk_end;
            if self.range.start == self.chunks.offset() + chunk.len() {
                self.chunks.next().unwrap();
            }

            Some(Chunk {
                text: slice,
                syntax_highlight_id: highlight_id,
                underline: self.underline,
                diagnostic_severity: self.current_diagnostic_severity(),
                is_unnecessary: self.current_code_is_unnecessary(),
                tabs,
                chars: chars_map,
                ..Chunk::default()
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
            }
            | Operation::UpdateLineEnding {
                lamport_timestamp, ..
            } => *lamport_timestamp,
        }
    }
}

impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            source: Default::default(),
            source_kind: DiagnosticSourceKind::Other,
            code: None,
            code_description: None,
            severity: DiagnosticSeverity::ERROR,
            message: Default::default(),
            markdown: None,
            group_id: 0,
            is_primary: false,
            is_disk_based: false,
            is_unnecessary: false,
            underline: true,
            data: None,
        }
    }
}

impl IndentSize {
    /// Returns an [`IndentSize`] representing the given spaces.
    pub fn spaces(len: u32) -> Self {
        Self {
            len,
            kind: IndentKind::Space,
        }
    }

    /// Returns an [`IndentSize`] representing a tab.
    pub fn tab() -> Self {
        Self {
            len: 1,
            kind: IndentKind::Tab,
        }
    }

    /// An iterator over the characters represented by this [`IndentSize`].
    pub fn chars(&self) -> impl Iterator<Item = char> {
        iter::repeat(self.char()).take(self.len as usize)
    }

    /// The character representation of this [`IndentSize`].
    pub fn char(&self) -> char {
        match self.kind {
            IndentKind::Space => ' ',
            IndentKind::Tab => '\t',
        }
    }

    /// Consumes the current [`IndentSize`] and returns a new one that has
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

    pub fn len_with_expanded_tabs(&self, tab_size: NonZeroU32) -> usize {
        match self.kind {
            IndentKind::Space => self.len as usize,
            IndentKind::Tab => self.len as usize * tab_size.get() as usize,
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct TestFile {
    pub path: Arc<RelPath>,
    pub root_name: String,
    pub local_root: Option<PathBuf>,
}

#[cfg(any(test, feature = "test-support"))]
impl File for TestFile {
    fn path(&self) -> &Arc<RelPath> {
        &self.path
    }

    fn full_path(&self, _: &gpui::App) -> PathBuf {
        PathBuf::from(self.root_name.clone()).join(self.path.as_std_path())
    }

    fn as_local(&self) -> Option<&dyn LocalFile> {
        if self.local_root.is_some() {
            Some(self)
        } else {
            None
        }
    }

    fn disk_state(&self) -> DiskState {
        unimplemented!()
    }

    fn file_name<'a>(&'a self, _: &'a gpui::App) -> &'a str {
        self.path().file_name().unwrap_or(self.root_name.as_ref())
    }

    fn worktree_id(&self, _: &App) -> WorktreeId {
        WorktreeId::from_usize(0)
    }

    fn to_proto(&self, _: &App) -> rpc::proto::File {
        unimplemented!()
    }

    fn is_private(&self) -> bool {
        false
    }

    fn path_style(&self, _cx: &App) -> PathStyle {
        PathStyle::local()
    }
}

#[cfg(any(test, feature = "test-support"))]
impl LocalFile for TestFile {
    fn abs_path(&self, _cx: &App) -> PathBuf {
        PathBuf::from(self.local_root.as_ref().unwrap())
            .join(&self.root_name)
            .join(self.path.as_std_path())
    }

    fn load(&self, _cx: &App) -> Task<Result<String>> {
        unimplemented!()
    }

    fn load_bytes(&self, _cx: &App) -> Task<Result<Vec<u8>>> {
        unimplemented!()
    }
}

pub(crate) fn contiguous_ranges(
    values: impl Iterator<Item = u32>,
    max_len: usize,
) -> impl Iterator<Item = Range<u32>> {
    let mut values = values;
    let mut current_range: Option<Range<u32>> = None;
    std::iter::from_fn(move || {
        loop {
            if let Some(value) = values.next() {
                if let Some(range) = &mut current_range
                    && value == range.end
                    && range.len() < max_len
                {
                    range.end += 1;
                    continue;
                }

                let prev_range = current_range.clone();
                current_range = Some(value..(value + 1));
                if prev_range.is_some() {
                    return prev_range;
                }
            } else {
                return current_range.take();
            }
        }
    })
}

#[derive(Default, Debug)]
pub struct CharClassifier {
    scope: Option<LanguageScope>,
    scope_context: Option<CharScopeContext>,
    ignore_punctuation: bool,
}

impl CharClassifier {
    pub fn new(scope: Option<LanguageScope>) -> Self {
        Self {
            scope,
            scope_context: None,
            ignore_punctuation: false,
        }
    }

    pub fn scope_context(self, scope_context: Option<CharScopeContext>) -> Self {
        Self {
            scope_context,
            ..self
        }
    }

    pub fn ignore_punctuation(self, ignore_punctuation: bool) -> Self {
        Self {
            ignore_punctuation,
            ..self
        }
    }

    pub fn is_whitespace(&self, c: char) -> bool {
        self.kind(c) == CharKind::Whitespace
    }

    pub fn is_word(&self, c: char) -> bool {
        self.kind(c) == CharKind::Word
    }

    pub fn is_punctuation(&self, c: char) -> bool {
        self.kind(c) == CharKind::Punctuation
    }

    pub fn kind_with(&self, c: char, ignore_punctuation: bool) -> CharKind {
        if c.is_alphanumeric() || c == '_' {
            return CharKind::Word;
        }

        if let Some(scope) = &self.scope {
            let characters = match self.scope_context {
                Some(CharScopeContext::Completion) => scope.completion_query_characters(),
                Some(CharScopeContext::LinkedEdit) => scope.linked_edit_characters(),
                None => scope.word_characters(),
            };
            if let Some(characters) = characters
                && characters.contains(&c)
            {
                return CharKind::Word;
            }
        }

        if c.is_whitespace() {
            return CharKind::Whitespace;
        }

        if ignore_punctuation {
            CharKind::Word
        } else {
            CharKind::Punctuation
        }
    }

    pub fn kind(&self, c: char) -> CharKind {
        self.kind_with(c, self.ignore_punctuation)
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
            let trimmed_line_len = line.trim_end_matches([' ', '\t']).len();
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
