mod anchor;
#[cfg(test)]
mod multi_buffer_tests;
mod position;

pub use anchor::{Anchor, AnchorRangeExt, Offset};
pub use position::{TypedOffset, TypedPoint, TypedRow};

use anyhow::{Result, anyhow};
use buffer_diff::{
    BufferDiff, BufferDiffEvent, BufferDiffSnapshot, DiffHunkSecondaryStatus, DiffHunkStatus,
    DiffHunkStatusKind,
};
use clock::ReplicaId;
use collections::{BTreeMap, Bound, HashMap, HashSet};
use gpui::{App, AppContext as _, Context, Entity, EntityId, EventEmitter, Task};
use itertools::Itertools;
use language::{
    AutoindentMode, Buffer, BufferChunks, BufferRow, BufferSnapshot, Capability, CharClassifier,
    CharKind, Chunk, CursorShape, DiagnosticEntry, DiskState, File, IndentSize, Language,
    LanguageScope, OffsetRangeExt, OffsetUtf16, Outline, OutlineItem, Point, PointUtf16, Selection,
    TextDimension, TextObject, ToOffset as _, ToPoint as _, TransactionId, TreeSitterOptions,
    Unclipped,
    language_settings::{IndentGuideSettings, LanguageSettings, language_settings},
};

use rope::DimensionPair;
use smallvec::SmallVec;
use smol::future::yield_now;
use std::{
    any::type_name,
    borrow::Cow,
    cell::{Cell, Ref, RefCell},
    cmp, fmt,
    future::Future,
    io,
    iter::{self, FromIterator},
    mem,
    ops::{Range, RangeBounds, Sub},
    path::{Path, PathBuf},
    rc::Rc,
    str,
    sync::Arc,
    time::{Duration, Instant},
};
use sum_tree::{Bias, Cursor, Dimension, Dimensions, SumTree, Summary, TreeMap};
use text::{
    BufferId, Edit, LineIndent, TextSummary,
    locator::Locator,
    subscription::{Subscription, Topic},
};
use theme::SyntaxTheme;
use util::post_inc;

const NEWLINES: &[u8] = &[b'\n'; u8::MAX as usize];

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExcerptId(usize);

/// One or more [`Buffers`](Buffer) being edited in a single view.
///
/// See <https://zed.dev/features#multi-buffers>
pub struct MultiBuffer {
    /// A snapshot of the [`Excerpt`]s in the MultiBuffer.
    /// Use [`MultiBuffer::snapshot`] to get a up-to-date snapshot.
    snapshot: RefCell<MultiBufferSnapshot>,
    /// Contains the state of the buffers being edited
    buffers: RefCell<HashMap<BufferId, BufferState>>,
    // only used by consumers using `set_excerpts_for_buffer`
    excerpts_by_path: BTreeMap<PathKey, Vec<ExcerptId>>,
    paths_by_excerpt: HashMap<ExcerptId, PathKey>,
    diffs: HashMap<BufferId, DiffState>,
    // all_diff_hunks_expanded: bool,
    subscriptions: Topic,
    /// If true, the multi-buffer only contains a single [`Buffer`] and a single [`Excerpt`]
    singleton: bool,
    history: History,
    title: Option<String>,
    capability: Capability,
    buffer_changed_since_sync: Rc<Cell<bool>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultiOrSingleBufferOffsetRange {
    Single(Range<usize>),
    Multi(Range<usize>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    ExcerptsAdded {
        buffer: Entity<Buffer>,
        predecessor: ExcerptId,
        excerpts: Vec<(ExcerptId, ExcerptRange<language::Anchor>)>,
    },
    ExcerptsRemoved {
        ids: Vec<ExcerptId>,
        removed_buffer_ids: Vec<BufferId>,
    },
    ExcerptsExpanded {
        ids: Vec<ExcerptId>,
    },
    ExcerptsEdited {
        excerpt_ids: Vec<ExcerptId>,
        buffer_ids: Vec<BufferId>,
    },
    DiffHunksToggled,
    Edited {
        singleton_buffer_edited: bool,
        edited_buffer: Option<Entity<Buffer>>,
    },
    TransactionUndone {
        transaction_id: TransactionId,
    },
    Reloaded,
    LanguageChanged(BufferId),
    Reparsed(BufferId),
    Saved,
    FileHandleChanged,
    DirtyChanged,
    DiagnosticsUpdated,
    BufferDiffChanged,
}

/// A diff hunk, representing a range of consequent lines in a multibuffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiBufferDiffHunk {
    /// The row range in the multibuffer where this diff hunk appears.
    pub row_range: Range<MultiBufferRow>,
    /// The buffer ID that this hunk belongs to.
    pub buffer_id: BufferId,
    /// The range of the underlying buffer that this hunk corresponds to.
    pub buffer_range: Range<text::Anchor>,
    /// The excerpt that contains the diff hunk.
    pub excerpt_id: ExcerptId,
    /// The range within the buffer's diff base that this hunk corresponds to.
    pub diff_base_byte_range: Range<usize>,
    /// Whether or not this hunk also appears in the 'secondary diff'.
    pub secondary_status: DiffHunkSecondaryStatus,
}

impl MultiBufferDiffHunk {
    pub fn status(&self) -> DiffHunkStatus {
        let kind = if self.buffer_range.start == self.buffer_range.end {
            DiffHunkStatusKind::Deleted
        } else if self.diff_base_byte_range.is_empty() {
            DiffHunkStatusKind::Added
        } else {
            DiffHunkStatusKind::Modified
        };
        DiffHunkStatus {
            kind,
            secondary: self.secondary_status,
        }
    }

    pub fn is_created_file(&self) -> bool {
        self.diff_base_byte_range == (0..0)
            && self.buffer_range == (text::Anchor::MIN..text::Anchor::MAX)
    }

    pub fn multi_buffer_range(&self) -> Range<Anchor> {
        let start = Anchor::in_buffer(self.excerpt_id, self.buffer_id, self.buffer_range.start);
        let end = Anchor::in_buffer(self.excerpt_id, self.buffer_id, self.buffer_range.end);
        start..end
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct PathKey {
    namespace: u32,
    path: Arc<Path>,
}

impl PathKey {
    pub fn namespaced(namespace: u32, path: Arc<Path>) -> Self {
        Self { namespace, path }
    }

    pub fn for_buffer(buffer: &Entity<Buffer>, cx: &App) -> Self {
        if let Some(file) = buffer.read(cx).file() {
            Self::namespaced(1, Arc::from(file.full_path(cx)))
        } else {
            Self::namespaced(0, Arc::from(PathBuf::from(buffer.entity_id().to_string())))
        }
    }

    pub fn path(&self) -> &Arc<Path> {
        &self.path
    }
}

pub type MultiBufferPoint = Point;
type ExcerptOffset = TypedOffset<Excerpt>;
type ExcerptPoint = TypedPoint<Excerpt>;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Hash, serde::Deserialize)]
#[serde(transparent)]
pub struct MultiBufferRow(pub u32);

impl MultiBufferRow {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(u32::MAX);
}

impl std::ops::Add<usize> for MultiBufferRow {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        MultiBufferRow(self.0 + rhs as u32)
    }
}

#[derive(Clone)]
struct History {
    next_transaction_id: TransactionId,
    undo_stack: Vec<Transaction>,
    redo_stack: Vec<Transaction>,
    transaction_depth: usize,
    group_interval: Duration,
}

#[derive(Clone)]
struct Transaction {
    id: TransactionId,
    buffer_transactions: HashMap<BufferId, text::TransactionId>,
    first_edit_at: Instant,
    last_edit_at: Instant,
    suppress_grouping: bool,
}

pub trait ToOffset: 'static + fmt::Debug {
    fn to_offset(&self, snapshot: &MultiBufferSnapshot) -> usize;
}

pub trait ToOffsetUtf16: 'static + fmt::Debug {
    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> OffsetUtf16;
}

pub trait ToPoint: 'static + fmt::Debug {
    fn to_point(&self, snapshot: &MultiBufferSnapshot) -> Point;
}

pub trait ToPointUtf16: 'static + fmt::Debug {
    fn to_point_utf16(&self, snapshot: &MultiBufferSnapshot) -> PointUtf16;
}

struct BufferState {
    buffer: Entity<Buffer>,
    last_version: clock::Global,
    last_non_text_state_update_count: usize,
    excerpts: Vec<Locator>,
    _subscriptions: [gpui::Subscription; 2],
}

struct DiffState {
    diff: Entity<BufferDiff>,
    _subscription: gpui::Subscription,
}

impl DiffState {
    fn new(diff: Entity<BufferDiff>, cx: &mut Context<MultiBuffer>) -> Self {
        DiffState {
            _subscription: cx.subscribe(&diff, |this, diff, event, cx| match event {
                BufferDiffEvent::DiffChanged { changed_range } => {
                    if let Some(changed_range) = changed_range.clone() {
                        this.buffer_diff_changed(diff, changed_range, cx)
                    }
                    cx.emit(Event::BufferDiffChanged);
                }
                BufferDiffEvent::LanguageChanged => this.buffer_diff_language_changed(diff, cx),
                _ => {}
            }),
            diff,
        }
    }
}

/// The contents of a [`MultiBuffer`] at a single point in time.
#[derive(Clone, Default)]
pub struct MultiBufferSnapshot {
    singleton: bool,
    excerpts: SumTree<Excerpt>,
    excerpt_ids: SumTree<ExcerptIdMapping>,
    diffs: TreeMap<BufferId, BufferDiffSnapshot>,
    diff_transforms: SumTree<DiffTransform>,
    replaced_excerpts: TreeMap<ExcerptId, ExcerptId>,
    trailing_excerpt_update_count: usize,
    all_diff_hunks_expanded: bool,
    non_text_state_update_count: usize,
    edit_count: usize,
    is_dirty: bool,
    has_deleted_file: bool,
    has_conflict: bool,
    show_headers: bool,
}

#[derive(Debug, Clone)]
enum DiffTransform {
    BufferContent {
        summary: TextSummary,
        inserted_hunk_info: Option<DiffTransformHunkInfo>,
    },
    DeletedHunk {
        summary: TextSummary,
        buffer_id: BufferId,
        hunk_info: DiffTransformHunkInfo,
        base_text_byte_range: Range<usize>,
        has_trailing_newline: bool,
    },
}

#[derive(Clone, Copy, Debug)]
struct DiffTransformHunkInfo {
    excerpt_id: ExcerptId,
    hunk_start_anchor: text::Anchor,
    hunk_secondary_status: DiffHunkSecondaryStatus,
}

impl Eq for DiffTransformHunkInfo {}

impl PartialEq for DiffTransformHunkInfo {
    fn eq(&self, other: &DiffTransformHunkInfo) -> bool {
        self.excerpt_id == other.excerpt_id && self.hunk_start_anchor == other.hunk_start_anchor
    }
}

impl std::hash::Hash for DiffTransformHunkInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.excerpt_id.hash(state);
        self.hunk_start_anchor.hash(state);
    }
}

#[derive(Clone)]
pub struct ExcerptInfo {
    pub id: ExcerptId,
    pub buffer: BufferSnapshot,
    pub buffer_id: BufferId,
    pub range: ExcerptRange<text::Anchor>,
    pub end_row: MultiBufferRow,
}

impl std::fmt::Debug for ExcerptInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("id", &self.id)
            .field("buffer_id", &self.buffer_id)
            .field("path", &self.buffer.file().map(|f| f.path()))
            .field("range", &self.range)
            .finish()
    }
}

/// A boundary between `Excerpt`s in a [`MultiBuffer`]
#[derive(Debug)]
pub struct ExcerptBoundary {
    pub prev: Option<ExcerptInfo>,
    pub next: ExcerptInfo,
    /// The row in the `MultiBuffer` where the boundary is located
    pub row: MultiBufferRow,
}

impl ExcerptBoundary {
    pub fn starts_new_buffer(&self) -> bool {
        match (self.prev.as_ref(), &self.next) {
            (None, _) => true,
            (Some(prev), next) => prev.buffer_id != next.buffer_id,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExpandInfo {
    pub direction: ExpandExcerptDirection,
    pub excerpt_id: ExcerptId,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct RowInfo {
    pub buffer_id: Option<BufferId>,
    pub buffer_row: Option<u32>,
    pub multibuffer_row: Option<MultiBufferRow>,
    pub diff_status: Option<buffer_diff::DiffHunkStatus>,
    pub expand_info: Option<ExpandInfo>,
}

/// A slice into a [`Buffer`] that is being edited in a [`MultiBuffer`].
#[derive(Clone)]
struct Excerpt {
    /// The unique identifier for this excerpt
    id: ExcerptId,
    /// The location of the excerpt in the [`MultiBuffer`]
    locator: Locator,
    /// The buffer being excerpted
    buffer_id: BufferId,
    /// A snapshot of the buffer being excerpted
    buffer: BufferSnapshot,
    /// The range of the buffer to be shown in the excerpt
    range: ExcerptRange<text::Anchor>,
    /// The last row in the excerpted slice of the buffer
    max_buffer_row: BufferRow,
    /// A summary of the text in the excerpt
    text_summary: TextSummary,
    has_trailing_newline: bool,
}

/// A public view into an `Excerpt` in a [`MultiBuffer`].
///
/// Contains methods for getting the [`Buffer`] of the excerpt,
/// as well as mapping offsets to/from buffer and multibuffer coordinates.
#[derive(Clone)]
pub struct MultiBufferExcerpt<'a> {
    excerpt: &'a Excerpt,
    diff_transforms: sum_tree::Cursor<'a, DiffTransform, DiffTransforms<usize>>,
    offset: usize,
    excerpt_offset: ExcerptDimension<usize>,
    buffer_offset: usize,
}

#[derive(Clone, Debug)]
struct ExcerptIdMapping {
    id: ExcerptId,
    locator: Locator,
}

/// A range of text from a single [`Buffer`], to be shown as an `Excerpt`.
/// These ranges are relative to the buffer itself
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExcerptRange<T> {
    /// The full range of text to be shown in the excerpt.
    pub context: Range<T>,
    /// The primary range of text to be highlighted in the excerpt.
    /// In a multi-buffer search, this would be the text that matched the search
    pub primary: Range<T>,
}

impl<T: Clone> ExcerptRange<T> {
    pub fn new(context: Range<T>) -> Self {
        Self {
            context: context.clone(),
            primary: context,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ExcerptSummary {
    excerpt_id: ExcerptId,
    /// The location of the last [`Excerpt`] being summarized
    excerpt_locator: Locator,
    widest_line_number: u32,
    text: TextSummary,
}

#[derive(Debug, Clone)]
pub struct DiffTransformSummary {
    input: TextSummary,
    output: TextSummary,
}

#[derive(Clone)]
pub struct MultiBufferRows<'a> {
    point: Point,
    is_empty: bool,
    is_singleton: bool,
    cursor: MultiBufferCursor<'a, Point>,
}

pub struct MultiBufferChunks<'a> {
    excerpts: Cursor<'a, Excerpt, ExcerptOffset>,
    diff_transforms: Cursor<'a, DiffTransform, Dimensions<usize, ExcerptOffset>>,
    diffs: &'a TreeMap<BufferId, BufferDiffSnapshot>,
    diff_base_chunks: Option<(BufferId, BufferChunks<'a>)>,
    buffer_chunk: Option<Chunk<'a>>,
    range: Range<usize>,
    excerpt_offset_range: Range<ExcerptOffset>,
    excerpt_chunks: Option<ExcerptChunks<'a>>,
    language_aware: bool,
}

pub struct ReversedMultiBufferChunks<'a> {
    cursor: MultiBufferCursor<'a, usize>,
    current_chunks: Option<rope::Chunks<'a>>,
    start: usize,
    offset: usize,
}

pub struct MultiBufferBytes<'a> {
    range: Range<usize>,
    cursor: MultiBufferCursor<'a, usize>,
    excerpt_bytes: Option<text::Bytes<'a>>,
    has_trailing_newline: bool,
    chunk: &'a [u8],
}

pub struct ReversedMultiBufferBytes<'a> {
    range: Range<usize>,
    chunks: ReversedMultiBufferChunks<'a>,
    chunk: &'a [u8],
}

#[derive(Clone)]
struct DiffTransforms<D> {
    output_dimension: OutputDimension<D>,
    excerpt_dimension: ExcerptDimension<D>,
}

impl<'a, D: TextDimension> Dimension<'a, DiffTransformSummary> for DiffTransforms<D> {
    fn zero(cx: &<DiffTransformSummary as sum_tree::Summary>::Context) -> Self {
        Self {
            output_dimension: OutputDimension::zero(cx),
            excerpt_dimension: <ExcerptDimension<D> as Dimension<'a, DiffTransformSummary>>::zero(
                cx,
            ),
        }
    }

    fn add_summary(
        &mut self,
        summary: &'a DiffTransformSummary,
        cx: &<DiffTransformSummary as sum_tree::Summary>::Context,
    ) {
        self.output_dimension.add_summary(summary, cx);
        self.excerpt_dimension.add_summary(summary, cx);
    }
}

#[derive(Clone)]
struct MultiBufferCursor<'a, D: TextDimension> {
    excerpts: Cursor<'a, Excerpt, ExcerptDimension<D>>,
    diff_transforms: Cursor<'a, DiffTransform, DiffTransforms<D>>,
    diffs: &'a TreeMap<BufferId, BufferDiffSnapshot>,
    cached_region: Option<MultiBufferRegion<'a, D>>,
}

#[derive(Clone)]
struct MultiBufferRegion<'a, D: TextDimension> {
    buffer: &'a BufferSnapshot,
    is_main_buffer: bool,
    diff_hunk_status: Option<DiffHunkStatus>,
    excerpt: &'a Excerpt,
    buffer_range: Range<D>,
    range: Range<D>,
    has_trailing_newline: bool,
}

struct ExcerptChunks<'a> {
    excerpt_id: ExcerptId,
    content_chunks: BufferChunks<'a>,
    footer_height: usize,
}

#[derive(Debug)]
struct BufferEdit {
    range: Range<usize>,
    new_text: Arc<str>,
    is_insertion: bool,
    original_indent_column: Option<u32>,
    excerpt_id: ExcerptId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DiffChangeKind {
    BufferEdited,
    DiffUpdated { base_changed: bool },
    ExpandOrCollapseHunks { expand: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpandExcerptDirection {
    Up,
    Down,
    UpAndDown,
}

impl ExpandExcerptDirection {
    pub fn should_expand_up(&self) -> bool {
        match self {
            ExpandExcerptDirection::Up => true,
            ExpandExcerptDirection::Down => false,
            ExpandExcerptDirection::UpAndDown => true,
        }
    }

    pub fn should_expand_down(&self) -> bool {
        match self {
            ExpandExcerptDirection::Up => false,
            ExpandExcerptDirection::Down => true,
            ExpandExcerptDirection::UpAndDown => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndentGuide {
    pub buffer_id: BufferId,
    pub start_row: MultiBufferRow,
    pub end_row: MultiBufferRow,
    pub depth: u32,
    pub tab_size: u32,
    pub settings: IndentGuideSettings,
}

impl IndentGuide {
    pub fn indent_level(&self) -> u32 {
        self.depth * self.tab_size
    }
}

impl MultiBuffer {
    pub fn new(capability: Capability) -> Self {
        Self {
            snapshot: RefCell::new(MultiBufferSnapshot {
                show_headers: true,
                ..MultiBufferSnapshot::default()
            }),
            buffers: RefCell::default(),
            diffs: HashMap::default(),
            subscriptions: Topic::default(),
            singleton: false,
            capability,
            title: None,
            excerpts_by_path: Default::default(),
            paths_by_excerpt: Default::default(),
            buffer_changed_since_sync: Default::default(),
            history: History {
                next_transaction_id: clock::Lamport::default(),
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),
                transaction_depth: 0,
                group_interval: Duration::from_millis(300),
            },
        }
    }

    pub fn without_headers(capability: Capability) -> Self {
        Self {
            snapshot: Default::default(),
            buffers: Default::default(),
            excerpts_by_path: Default::default(),
            paths_by_excerpt: Default::default(),
            diffs: HashMap::default(),
            subscriptions: Default::default(),
            singleton: false,
            capability,
            buffer_changed_since_sync: Default::default(),
            history: History {
                next_transaction_id: Default::default(),
                undo_stack: Default::default(),
                redo_stack: Default::default(),
                transaction_depth: 0,
                group_interval: Duration::from_millis(300),
            },
            title: Default::default(),
        }
    }

    pub fn clone(&self, new_cx: &mut Context<Self>) -> Self {
        let mut buffers = HashMap::default();
        let buffer_changed_since_sync = Rc::new(Cell::new(false));
        for (buffer_id, buffer_state) in self.buffers.borrow().iter() {
            buffer_state.buffer.update(new_cx, |buffer, _| {
                buffer.record_changes(Rc::downgrade(&buffer_changed_since_sync));
            });
            buffers.insert(
                *buffer_id,
                BufferState {
                    buffer: buffer_state.buffer.clone(),
                    last_version: buffer_state.last_version.clone(),
                    last_non_text_state_update_count: buffer_state.last_non_text_state_update_count,
                    excerpts: buffer_state.excerpts.clone(),
                    _subscriptions: [
                        new_cx.observe(&buffer_state.buffer, |_, _, cx| cx.notify()),
                        new_cx.subscribe(&buffer_state.buffer, Self::on_buffer_event),
                    ],
                },
            );
        }
        let mut diff_bases = HashMap::default();
        for (buffer_id, diff) in self.diffs.iter() {
            diff_bases.insert(*buffer_id, DiffState::new(diff.diff.clone(), new_cx));
        }
        Self {
            snapshot: RefCell::new(self.snapshot.borrow().clone()),
            buffers: RefCell::new(buffers),
            excerpts_by_path: Default::default(),
            paths_by_excerpt: Default::default(),
            diffs: diff_bases,
            subscriptions: Default::default(),
            singleton: self.singleton,
            capability: self.capability,
            history: self.history.clone(),
            title: self.title.clone(),
            buffer_changed_since_sync,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn read_only(&self) -> bool {
        self.capability == Capability::ReadOnly
    }

    pub fn singleton(buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Self {
        let mut this = Self::new(buffer.read(cx).capability());
        this.singleton = true;
        this.push_excerpts(
            buffer,
            [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
            cx,
        );
        this.snapshot.borrow_mut().singleton = true;
        this
    }

    /// Returns an up-to-date snapshot of the MultiBuffer.
    pub fn snapshot(&self, cx: &App) -> MultiBufferSnapshot {
        self.sync(cx);
        self.snapshot.borrow().clone()
    }

    pub fn read(&self, cx: &App) -> Ref<'_, MultiBufferSnapshot> {
        self.sync(cx);
        self.snapshot.borrow()
    }

    pub fn as_singleton(&self) -> Option<Entity<Buffer>> {
        if self.singleton {
            Some(
                self.buffers
                    .borrow()
                    .values()
                    .next()
                    .unwrap()
                    .buffer
                    .clone(),
            )
        } else {
            None
        }
    }

    pub fn is_singleton(&self) -> bool {
        self.singleton
    }

    pub fn subscribe(&mut self) -> Subscription {
        self.subscriptions.subscribe()
    }

    pub fn is_dirty(&self, cx: &App) -> bool {
        self.read(cx).is_dirty()
    }

    pub fn has_deleted_file(&self, cx: &App) -> bool {
        self.read(cx).has_deleted_file()
    }

    pub fn has_conflict(&self, cx: &App) -> bool {
        self.read(cx).has_conflict()
    }

    // The `is_empty` signature doesn't match what clippy expects
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self, cx: &App) -> usize {
        self.read(cx).len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.borrow().is_empty()
    }

    pub fn symbols_containing<T: ToOffset>(
        &self,
        offset: T,
        theme: Option<&SyntaxTheme>,
        cx: &App,
    ) -> Option<(BufferId, Vec<OutlineItem<Anchor>>)> {
        self.read(cx).symbols_containing(offset, theme)
    }

    pub fn edit<I, S, T>(
        &self,
        edits: I,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut Context<Self>,
    ) where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        let snapshot = self.read(cx);
        let edits = edits
            .into_iter()
            .map(|(range, new_text)| {
                let mut range = range.start.to_offset(&snapshot)..range.end.to_offset(&snapshot);
                if range.start > range.end {
                    mem::swap(&mut range.start, &mut range.end);
                }
                (range, new_text.into())
            })
            .collect::<Vec<_>>();

        return edit_internal(self, snapshot, edits, autoindent_mode, cx);

        // Non-generic part of edit, hoisted out to avoid blowing up LLVM IR.
        fn edit_internal(
            this: &MultiBuffer,
            snapshot: Ref<MultiBufferSnapshot>,
            edits: Vec<(Range<usize>, Arc<str>)>,
            mut autoindent_mode: Option<AutoindentMode>,
            cx: &mut Context<MultiBuffer>,
        ) {
            if this.read_only() || this.buffers.borrow().is_empty() {
                return;
            }

            let original_indent_columns = match &mut autoindent_mode {
                Some(AutoindentMode::Block {
                    original_indent_columns,
                }) => mem::take(original_indent_columns),
                _ => Default::default(),
            };

            let (buffer_edits, edited_excerpt_ids) =
                this.convert_edits_to_buffer_edits(edits, &snapshot, &original_indent_columns);
            drop(snapshot);

            let mut buffer_ids = Vec::with_capacity(buffer_edits.len());
            for (buffer_id, mut edits) in buffer_edits {
                buffer_ids.push(buffer_id);
                edits.sort_by_key(|edit| edit.range.start);
                this.buffers.borrow()[&buffer_id]
                    .buffer
                    .update(cx, |buffer, cx| {
                        let mut edits = edits.into_iter().peekable();
                        let mut insertions = Vec::new();
                        let mut original_indent_columns = Vec::new();
                        let mut deletions = Vec::new();
                        let empty_str: Arc<str> = Arc::default();
                        while let Some(BufferEdit {
                            mut range,
                            mut new_text,
                            mut is_insertion,
                            original_indent_column,
                            excerpt_id,
                        }) = edits.next()
                        {
                            while let Some(BufferEdit {
                                range: next_range,
                                is_insertion: next_is_insertion,
                                new_text: next_new_text,
                                excerpt_id: next_excerpt_id,
                                ..
                            }) = edits.peek()
                            {
                                if range.end >= next_range.start {
                                    range.end = cmp::max(next_range.end, range.end);
                                    is_insertion |= *next_is_insertion;
                                    if excerpt_id == *next_excerpt_id {
                                        new_text = format!("{new_text}{next_new_text}").into();
                                    }
                                    edits.next();
                                } else {
                                    break;
                                }
                            }

                            if is_insertion {
                                original_indent_columns.push(original_indent_column);
                                insertions.push((
                                    buffer.anchor_before(range.start)
                                        ..buffer.anchor_before(range.end),
                                    new_text.clone(),
                                ));
                            } else if !range.is_empty() {
                                deletions.push((
                                    buffer.anchor_before(range.start)
                                        ..buffer.anchor_before(range.end),
                                    empty_str.clone(),
                                ));
                            }
                        }

                        let deletion_autoindent_mode =
                            if let Some(AutoindentMode::Block { .. }) = autoindent_mode {
                                Some(AutoindentMode::Block {
                                    original_indent_columns: Default::default(),
                                })
                            } else {
                                autoindent_mode.clone()
                            };
                        let insertion_autoindent_mode =
                            if let Some(AutoindentMode::Block { .. }) = autoindent_mode {
                                Some(AutoindentMode::Block {
                                    original_indent_columns,
                                })
                            } else {
                                autoindent_mode.clone()
                            };

                        buffer.edit(deletions, deletion_autoindent_mode, cx);
                        buffer.edit(insertions, insertion_autoindent_mode, cx);
                    })
            }

            cx.emit(Event::ExcerptsEdited {
                excerpt_ids: edited_excerpt_ids,
                buffer_ids,
            });
        }
    }

    fn convert_edits_to_buffer_edits(
        &self,
        edits: Vec<(Range<usize>, Arc<str>)>,
        snapshot: &MultiBufferSnapshot,
        original_indent_columns: &[Option<u32>],
    ) -> (HashMap<BufferId, Vec<BufferEdit>>, Vec<ExcerptId>) {
        let mut buffer_edits: HashMap<BufferId, Vec<BufferEdit>> = Default::default();
        let mut edited_excerpt_ids = Vec::new();
        let mut cursor = snapshot.cursor::<usize>();
        for (ix, (range, new_text)) in edits.into_iter().enumerate() {
            let original_indent_column = original_indent_columns.get(ix).copied().flatten();

            cursor.seek(&range.start);
            let mut start_region = cursor.region().expect("start offset out of bounds");
            if !start_region.is_main_buffer {
                cursor.next();
                if let Some(region) = cursor.region() {
                    start_region = region;
                } else {
                    continue;
                }
            }

            if range.end < start_region.range.start {
                continue;
            }

            if range.end > start_region.range.end {
                cursor.seek_forward(&range.end);
            }
            let mut end_region = cursor.region().expect("end offset out of bounds");
            if !end_region.is_main_buffer {
                cursor.prev();
                if let Some(region) = cursor.region() {
                    end_region = region;
                } else {
                    continue;
                }
            }

            if range.start > end_region.range.end {
                continue;
            }

            let start_overshoot = range.start.saturating_sub(start_region.range.start);
            let end_overshoot = range.end.saturating_sub(end_region.range.start);
            let buffer_start = (start_region.buffer_range.start + start_overshoot)
                .min(start_region.buffer_range.end);
            let buffer_end =
                (end_region.buffer_range.start + end_overshoot).min(end_region.buffer_range.end);

            if start_region.excerpt.id == end_region.excerpt.id {
                if start_region.is_main_buffer {
                    edited_excerpt_ids.push(start_region.excerpt.id);
                    buffer_edits
                        .entry(start_region.buffer.remote_id())
                        .or_default()
                        .push(BufferEdit {
                            range: buffer_start..buffer_end,
                            new_text,
                            is_insertion: true,
                            original_indent_column,
                            excerpt_id: start_region.excerpt.id,
                        });
                }
            } else {
                let start_excerpt_range = buffer_start..start_region.buffer_range.end;
                let end_excerpt_range = end_region.buffer_range.start..buffer_end;
                if start_region.is_main_buffer {
                    edited_excerpt_ids.push(start_region.excerpt.id);
                    buffer_edits
                        .entry(start_region.buffer.remote_id())
                        .or_default()
                        .push(BufferEdit {
                            range: start_excerpt_range,
                            new_text: new_text.clone(),
                            is_insertion: true,
                            original_indent_column,
                            excerpt_id: start_region.excerpt.id,
                        });
                }
                if end_region.is_main_buffer {
                    edited_excerpt_ids.push(end_region.excerpt.id);
                    buffer_edits
                        .entry(end_region.buffer.remote_id())
                        .or_default()
                        .push(BufferEdit {
                            range: end_excerpt_range,
                            new_text: new_text.clone(),
                            is_insertion: false,
                            original_indent_column,
                            excerpt_id: end_region.excerpt.id,
                        });
                }

                cursor.seek(&range.start);
                cursor.next_excerpt();
                while let Some(region) = cursor.region() {
                    if region.excerpt.id == end_region.excerpt.id {
                        break;
                    }
                    if region.is_main_buffer {
                        edited_excerpt_ids.push(region.excerpt.id);
                        buffer_edits
                            .entry(region.buffer.remote_id())
                            .or_default()
                            .push(BufferEdit {
                                range: region.buffer_range,
                                new_text: new_text.clone(),
                                is_insertion: false,
                                original_indent_column,
                                excerpt_id: region.excerpt.id,
                            });
                    }
                    cursor.next_excerpt();
                }
            }
        }
        (buffer_edits, edited_excerpt_ids)
    }

    pub fn autoindent_ranges<I, S>(&self, ranges: I, cx: &mut Context<Self>)
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
    {
        let snapshot = self.read(cx);
        let empty = Arc::<str>::from("");
        let edits = ranges
            .into_iter()
            .map(|range| {
                let mut range = range.start.to_offset(&snapshot)..range.end.to_offset(&snapshot);
                if range.start > range.end {
                    mem::swap(&mut range.start, &mut range.end);
                }
                (range, empty.clone())
            })
            .collect::<Vec<_>>();

        return autoindent_ranges_internal(self, snapshot, edits, cx);

        fn autoindent_ranges_internal(
            this: &MultiBuffer,
            snapshot: Ref<MultiBufferSnapshot>,
            edits: Vec<(Range<usize>, Arc<str>)>,
            cx: &mut Context<MultiBuffer>,
        ) {
            if this.read_only() || this.buffers.borrow().is_empty() {
                return;
            }

            let (buffer_edits, edited_excerpt_ids) =
                this.convert_edits_to_buffer_edits(edits, &snapshot, &[]);
            drop(snapshot);

            let mut buffer_ids = Vec::new();
            for (buffer_id, mut edits) in buffer_edits {
                buffer_ids.push(buffer_id);
                edits.sort_unstable_by_key(|edit| edit.range.start);

                let mut ranges: Vec<Range<usize>> = Vec::new();
                for edit in edits {
                    if let Some(last_range) = ranges.last_mut()
                        && edit.range.start <= last_range.end
                    {
                        last_range.end = last_range.end.max(edit.range.end);
                        continue;
                    }
                    ranges.push(edit.range);
                }

                this.buffers.borrow()[&buffer_id]
                    .buffer
                    .update(cx, |buffer, cx| {
                        buffer.autoindent_ranges(ranges, cx);
                    })
            }

            cx.emit(Event::ExcerptsEdited {
                excerpt_ids: edited_excerpt_ids,
                buffer_ids,
            });
        }
    }

    // Inserts newlines at the given position to create an empty line, returning the start of the new line.
    // You can also request the insertion of empty lines above and below the line starting at the returned point.
    // Panics if the given position is invalid.
    pub fn insert_empty_line(
        &mut self,
        position: impl ToPoint,
        space_above: bool,
        space_below: bool,
        cx: &mut Context<Self>,
    ) -> Point {
        let multibuffer_point = position.to_point(&self.read(cx));
        let (buffer, buffer_point, _) = self.point_to_buffer_point(multibuffer_point, cx).unwrap();
        self.start_transaction(cx);
        let empty_line_start = buffer.update(cx, |buffer, cx| {
            buffer.insert_empty_line(buffer_point, space_above, space_below, cx)
        });
        self.end_transaction(cx);
        multibuffer_point + (empty_line_start - buffer_point)
    }

    pub fn start_transaction(&mut self, cx: &mut Context<Self>) -> Option<TransactionId> {
        self.start_transaction_at(Instant::now(), cx)
    }

    pub fn start_transaction_at(
        &mut self,
        now: Instant,
        cx: &mut Context<Self>,
    ) -> Option<TransactionId> {
        if let Some(buffer) = self.as_singleton() {
            return buffer.update(cx, |buffer, _| buffer.start_transaction_at(now));
        }

        for BufferState { buffer, .. } in self.buffers.borrow().values() {
            buffer.update(cx, |buffer, _| buffer.start_transaction_at(now));
        }
        self.history.start_transaction(now)
    }

    pub fn last_transaction_id(&self, cx: &App) -> Option<TransactionId> {
        if let Some(buffer) = self.as_singleton() {
            buffer
                .read(cx)
                .peek_undo_stack()
                .map(|history_entry| history_entry.transaction_id())
        } else {
            let last_transaction = self.history.undo_stack.last()?;
            Some(last_transaction.id)
        }
    }

    pub fn end_transaction(&mut self, cx: &mut Context<Self>) -> Option<TransactionId> {
        self.end_transaction_at(Instant::now(), cx)
    }

    pub fn end_transaction_at(
        &mut self,
        now: Instant,
        cx: &mut Context<Self>,
    ) -> Option<TransactionId> {
        if let Some(buffer) = self.as_singleton() {
            return buffer.update(cx, |buffer, cx| buffer.end_transaction_at(now, cx));
        }

        let mut buffer_transactions = HashMap::default();
        for BufferState { buffer, .. } in self.buffers.borrow().values() {
            if let Some(transaction_id) =
                buffer.update(cx, |buffer, cx| buffer.end_transaction_at(now, cx))
            {
                buffer_transactions.insert(buffer.read(cx).remote_id(), transaction_id);
            }
        }

        if self.history.end_transaction(now, buffer_transactions) {
            let transaction_id = self.history.group().unwrap();
            Some(transaction_id)
        } else {
            None
        }
    }

    pub fn edited_ranges_for_transaction<D>(
        &self,
        transaction_id: TransactionId,
        cx: &App,
    ) -> Vec<Range<D>>
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        let Some(transaction) = self.history.transaction(transaction_id) else {
            return Vec::new();
        };

        let mut ranges = Vec::new();
        let snapshot = self.read(cx);
        let buffers = self.buffers.borrow();
        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>(&());

        for (buffer_id, buffer_transaction) in &transaction.buffer_transactions {
            let Some(buffer_state) = buffers.get(buffer_id) else {
                continue;
            };

            let buffer = buffer_state.buffer.read(cx);
            for range in buffer.edited_ranges_for_transaction_id::<D>(*buffer_transaction) {
                for excerpt_id in &buffer_state.excerpts {
                    cursor.seek(excerpt_id, Bias::Left);
                    if let Some(excerpt) = cursor.item()
                        && excerpt.locator == *excerpt_id
                    {
                        let excerpt_buffer_start = excerpt.range.context.start.summary::<D>(buffer);
                        let excerpt_buffer_end = excerpt.range.context.end.summary::<D>(buffer);
                        let excerpt_range = excerpt_buffer_start..excerpt_buffer_end;
                        if excerpt_range.contains(&range.start)
                            && excerpt_range.contains(&range.end)
                        {
                            let excerpt_start = D::from_text_summary(&cursor.start().text);

                            let mut start = excerpt_start;
                            start.add_assign(&(range.start - excerpt_buffer_start));
                            let mut end = excerpt_start;
                            end.add_assign(&(range.end - excerpt_buffer_start));

                            ranges.push(start..end);
                            break;
                        }
                    }
                }
            }
        }

        ranges.sort_by_key(|range| range.start);
        ranges
    }

    pub fn merge_transactions(
        &mut self,
        transaction: TransactionId,
        destination: TransactionId,
        cx: &mut Context<Self>,
    ) {
        if let Some(buffer) = self.as_singleton() {
            buffer.update(cx, |buffer, _| {
                buffer.merge_transactions(transaction, destination)
            });
        } else if let Some(transaction) = self.history.forget(transaction)
            && let Some(destination) = self.history.transaction_mut(destination)
        {
            for (buffer_id, buffer_transaction_id) in transaction.buffer_transactions {
                if let Some(destination_buffer_transaction_id) =
                    destination.buffer_transactions.get(&buffer_id)
                {
                    if let Some(state) = self.buffers.borrow().get(&buffer_id) {
                        state.buffer.update(cx, |buffer, _| {
                            buffer.merge_transactions(
                                buffer_transaction_id,
                                *destination_buffer_transaction_id,
                            )
                        });
                    }
                } else {
                    destination
                        .buffer_transactions
                        .insert(buffer_id, buffer_transaction_id);
                }
            }
        }
    }

    pub fn finalize_last_transaction(&mut self, cx: &mut Context<Self>) {
        self.history.finalize_last_transaction();
        for BufferState { buffer, .. } in self.buffers.borrow().values() {
            buffer.update(cx, |buffer, _| {
                buffer.finalize_last_transaction();
            });
        }
    }

    pub fn push_transaction<'a, T>(&mut self, buffer_transactions: T, cx: &Context<Self>)
    where
        T: IntoIterator<Item = (&'a Entity<Buffer>, &'a language::Transaction)>,
    {
        self.history
            .push_transaction(buffer_transactions, Instant::now(), cx);
        self.history.finalize_last_transaction();
    }

    pub fn group_until_transaction(
        &mut self,
        transaction_id: TransactionId,
        cx: &mut Context<Self>,
    ) {
        if let Some(buffer) = self.as_singleton() {
            buffer.update(cx, |buffer, _| {
                buffer.group_until_transaction(transaction_id)
            });
        } else {
            self.history.group_until(transaction_id);
        }
    }

    pub fn set_active_selections(
        &self,
        selections: &[Selection<Anchor>],
        line_mode: bool,
        cursor_shape: CursorShape,
        cx: &mut Context<Self>,
    ) {
        let mut selections_by_buffer: HashMap<BufferId, Vec<Selection<text::Anchor>>> =
            Default::default();
        let snapshot = self.read(cx);
        let mut cursor = snapshot.excerpts.cursor::<Option<&Locator>>(&());
        for selection in selections {
            let start_locator = snapshot.excerpt_locator_for_id(selection.start.excerpt_id);
            let end_locator = snapshot.excerpt_locator_for_id(selection.end.excerpt_id);

            cursor.seek(&Some(start_locator), Bias::Left);
            while let Some(excerpt) = cursor.item() {
                if excerpt.locator > *end_locator {
                    break;
                }

                let mut start = excerpt.range.context.start;
                let mut end = excerpt.range.context.end;
                if excerpt.id == selection.start.excerpt_id {
                    start = selection.start.text_anchor;
                }
                if excerpt.id == selection.end.excerpt_id {
                    end = selection.end.text_anchor;
                }
                selections_by_buffer
                    .entry(excerpt.buffer_id)
                    .or_default()
                    .push(Selection {
                        id: selection.id,
                        start,
                        end,
                        reversed: selection.reversed,
                        goal: selection.goal,
                    });

                cursor.next();
            }
        }

        for (buffer_id, buffer_state) in self.buffers.borrow().iter() {
            if !selections_by_buffer.contains_key(buffer_id) {
                buffer_state
                    .buffer
                    .update(cx, |buffer, cx| buffer.remove_active_selections(cx));
            }
        }

        for (buffer_id, mut selections) in selections_by_buffer {
            self.buffers.borrow()[&buffer_id]
                .buffer
                .update(cx, |buffer, cx| {
                    selections.sort_unstable_by(|a, b| a.start.cmp(&b.start, buffer));
                    let mut selections = selections.into_iter().peekable();
                    let merged_selections = Arc::from_iter(iter::from_fn(|| {
                        let mut selection = selections.next()?;
                        while let Some(next_selection) = selections.peek() {
                            if selection.end.cmp(&next_selection.start, buffer).is_ge() {
                                let next_selection = selections.next().unwrap();
                                if next_selection.end.cmp(&selection.end, buffer).is_ge() {
                                    selection.end = next_selection.end;
                                }
                            } else {
                                break;
                            }
                        }
                        Some(selection)
                    }));
                    buffer.set_active_selections(merged_selections, line_mode, cursor_shape, cx);
                });
        }
    }

    pub fn remove_active_selections(&self, cx: &mut Context<Self>) {
        for buffer in self.buffers.borrow().values() {
            buffer
                .buffer
                .update(cx, |buffer, cx| buffer.remove_active_selections(cx));
        }
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) -> Option<TransactionId> {
        let mut transaction_id = None;
        if let Some(buffer) = self.as_singleton() {
            transaction_id = buffer.update(cx, |buffer, cx| buffer.undo(cx));
        } else {
            while let Some(transaction) = self.history.pop_undo() {
                let mut undone = false;
                for (buffer_id, buffer_transaction_id) in &mut transaction.buffer_transactions {
                    if let Some(BufferState { buffer, .. }) = self.buffers.borrow().get(buffer_id) {
                        undone |= buffer.update(cx, |buffer, cx| {
                            let undo_to = *buffer_transaction_id;
                            if let Some(entry) = buffer.peek_undo_stack() {
                                *buffer_transaction_id = entry.transaction_id();
                            }
                            buffer.undo_to_transaction(undo_to, cx)
                        });
                    }
                }

                if undone {
                    transaction_id = Some(transaction.id);
                    break;
                }
            }
        }

        if let Some(transaction_id) = transaction_id {
            cx.emit(Event::TransactionUndone { transaction_id });
        }

        transaction_id
    }

    pub fn redo(&mut self, cx: &mut Context<Self>) -> Option<TransactionId> {
        if let Some(buffer) = self.as_singleton() {
            return buffer.update(cx, |buffer, cx| buffer.redo(cx));
        }

        while let Some(transaction) = self.history.pop_redo() {
            let mut redone = false;
            for (buffer_id, buffer_transaction_id) in &mut transaction.buffer_transactions {
                if let Some(BufferState { buffer, .. }) = self.buffers.borrow().get(buffer_id) {
                    redone |= buffer.update(cx, |buffer, cx| {
                        let redo_to = *buffer_transaction_id;
                        if let Some(entry) = buffer.peek_redo_stack() {
                            *buffer_transaction_id = entry.transaction_id();
                        }
                        buffer.redo_to_transaction(redo_to, cx)
                    });
                }
            }

            if redone {
                return Some(transaction.id);
            }
        }

        None
    }

    pub fn undo_transaction(&mut self, transaction_id: TransactionId, cx: &mut Context<Self>) {
        if let Some(buffer) = self.as_singleton() {
            buffer.update(cx, |buffer, cx| buffer.undo_transaction(transaction_id, cx));
        } else if let Some(transaction) = self.history.remove_from_undo(transaction_id) {
            for (buffer_id, transaction_id) in &transaction.buffer_transactions {
                if let Some(BufferState { buffer, .. }) = self.buffers.borrow().get(buffer_id) {
                    buffer.update(cx, |buffer, cx| {
                        buffer.undo_transaction(*transaction_id, cx)
                    });
                }
            }
        }
    }

    pub fn forget_transaction(&mut self, transaction_id: TransactionId, cx: &mut Context<Self>) {
        if let Some(buffer) = self.as_singleton() {
            buffer.update(cx, |buffer, _| {
                buffer.forget_transaction(transaction_id);
            });
        } else if let Some(transaction) = self.history.forget(transaction_id) {
            for (buffer_id, buffer_transaction_id) in transaction.buffer_transactions {
                if let Some(state) = self.buffers.borrow_mut().get_mut(&buffer_id) {
                    state.buffer.update(cx, |buffer, _| {
                        buffer.forget_transaction(buffer_transaction_id);
                    });
                }
            }
        }
    }

    pub fn push_excerpts<O>(
        &mut self,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = ExcerptRange<O>>,
        cx: &mut Context<Self>,
    ) -> Vec<ExcerptId>
    where
        O: text::ToOffset,
    {
        self.insert_excerpts_after(ExcerptId::max(), buffer, ranges, cx)
    }

    pub fn location_for_path(&self, path: &PathKey, cx: &App) -> Option<Anchor> {
        let excerpt_id = self.excerpts_by_path.get(path)?.first()?;
        let snapshot = self.snapshot(cx);
        let excerpt = snapshot.excerpt(*excerpt_id)?;
        Some(Anchor::in_buffer(
            *excerpt_id,
            excerpt.buffer_id,
            excerpt.range.context.start,
        ))
    }

    pub fn excerpt_paths(&self) -> impl Iterator<Item = &PathKey> {
        self.excerpts_by_path.keys()
    }

    fn expand_excerpts_with_paths(
        &mut self,
        ids: impl IntoIterator<Item = ExcerptId>,
        line_count: u32,
        direction: ExpandExcerptDirection,
        cx: &mut Context<Self>,
    ) {
        let grouped = ids
            .into_iter()
            .chunk_by(|id| self.paths_by_excerpt.get(id).cloned())
            .into_iter()
            .flat_map(|(k, v)| Some((k?, v.into_iter().collect::<Vec<_>>())))
            .collect::<Vec<_>>();
        let snapshot = self.snapshot(cx);

        for (path, ids) in grouped.into_iter() {
            let Some(excerpt_ids) = self.excerpts_by_path.get(&path) else {
                continue;
            };

            let ids_to_expand = HashSet::from_iter(ids);
            let expanded_ranges = excerpt_ids.iter().filter_map(|excerpt_id| {
                let excerpt = snapshot.excerpt(*excerpt_id)?;

                let mut context = excerpt.range.context.to_point(&excerpt.buffer);
                if ids_to_expand.contains(excerpt_id) {
                    match direction {
                        ExpandExcerptDirection::Up => {
                            context.start.row = context.start.row.saturating_sub(line_count);
                            context.start.column = 0;
                        }
                        ExpandExcerptDirection::Down => {
                            context.end.row =
                                (context.end.row + line_count).min(excerpt.buffer.max_point().row);
                            context.end.column = excerpt.buffer.line_len(context.end.row);
                        }
                        ExpandExcerptDirection::UpAndDown => {
                            context.start.row = context.start.row.saturating_sub(line_count);
                            context.start.column = 0;
                            context.end.row =
                                (context.end.row + line_count).min(excerpt.buffer.max_point().row);
                            context.end.column = excerpt.buffer.line_len(context.end.row);
                        }
                    }
                }

                Some(ExcerptRange {
                    context,
                    primary: excerpt.range.primary.to_point(&excerpt.buffer),
                })
            });
            let mut merged_ranges: Vec<ExcerptRange<Point>> = Vec::new();
            for range in expanded_ranges {
                if let Some(last_range) = merged_ranges.last_mut()
                    && last_range.context.end >= range.context.start
                {
                    last_range.context.end = range.context.end;
                    continue;
                }
                merged_ranges.push(range)
            }
            let Some(excerpt_id) = excerpt_ids.first() else {
                continue;
            };
            let Some(buffer_id) = &snapshot.buffer_id_for_excerpt(*excerpt_id) else {
                continue;
            };

            let Some(buffer) = self
                .buffers
                .borrow()
                .get(buffer_id)
                .map(|b| b.buffer.clone())
            else {
                continue;
            };

            let buffer_snapshot = buffer.read(cx).snapshot();
            self.update_path_excerpts(path.clone(), buffer, &buffer_snapshot, merged_ranges, cx);
        }
    }

    /// Sets excerpts, returns `true` if at least one new excerpt was added.
    pub fn set_excerpts_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = Range<Point>>,
        context_line_count: u32,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let excerpt_ranges = build_excerpt_ranges(ranges, context_line_count, &buffer_snapshot);

        let (new, counts) = Self::merge_excerpt_ranges(&excerpt_ranges);
        self.set_merged_excerpt_ranges_for_path(
            path,
            buffer,
            excerpt_ranges,
            &buffer_snapshot,
            new,
            counts,
            cx,
        )
    }

    pub fn set_excerpt_ranges_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        buffer_snapshot: &BufferSnapshot,
        excerpt_ranges: Vec<ExcerptRange<Point>>,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let (new, counts) = Self::merge_excerpt_ranges(&excerpt_ranges);
        self.set_merged_excerpt_ranges_for_path(
            path,
            buffer,
            excerpt_ranges,
            buffer_snapshot,
            new,
            counts,
            cx,
        )
    }

    pub fn set_anchored_excerpts_for_path(
        &self,
        buffer: Entity<Buffer>,
        ranges: Vec<Range<text::Anchor>>,
        context_line_count: u32,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Range<Anchor>>> {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let path_key = PathKey::for_buffer(&buffer, cx);
        cx.spawn(async move |multi_buffer, cx| {
            let snapshot = buffer_snapshot.clone();
            let (excerpt_ranges, new, counts) = cx
                .background_spawn(async move {
                    let ranges = ranges.into_iter().map(|range| range.to_point(&snapshot));
                    let excerpt_ranges =
                        build_excerpt_ranges(ranges, context_line_count, &snapshot);
                    let (new, counts) = Self::merge_excerpt_ranges(&excerpt_ranges);
                    (excerpt_ranges, new, counts)
                })
                .await;

            multi_buffer
                .update(cx, move |multi_buffer, cx| {
                    let (ranges, _) = multi_buffer.set_merged_excerpt_ranges_for_path(
                        path_key,
                        buffer,
                        excerpt_ranges,
                        &buffer_snapshot,
                        new,
                        counts,
                        cx,
                    );
                    ranges
                })
                .ok()
                .unwrap_or_default()
        })
    }

    /// Sets excerpts, returns `true` if at least one new excerpt was added.
    fn set_merged_excerpt_ranges_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        ranges: Vec<ExcerptRange<Point>>,
        buffer_snapshot: &BufferSnapshot,
        new: Vec<ExcerptRange<Point>>,
        counts: Vec<usize>,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let (excerpt_ids, added_a_new_excerpt) =
            self.update_path_excerpts(path, buffer, buffer_snapshot, new, cx);

        let mut result = Vec::new();
        let mut ranges = ranges.into_iter();
        for (excerpt_id, range_count) in excerpt_ids.into_iter().zip(counts.into_iter()) {
            for range in ranges.by_ref().take(range_count) {
                let range = Anchor::range_in_buffer(
                    excerpt_id,
                    buffer_snapshot.remote_id(),
                    buffer_snapshot.anchor_before(&range.primary.start)
                        ..buffer_snapshot.anchor_after(&range.primary.end),
                );
                result.push(range)
            }
        }
        (result, added_a_new_excerpt)
    }

    fn merge_excerpt_ranges<'a>(
        expanded_ranges: impl IntoIterator<Item = &'a ExcerptRange<Point>> + 'a,
    ) -> (Vec<ExcerptRange<Point>>, Vec<usize>) {
        let mut merged_ranges: Vec<ExcerptRange<Point>> = Vec::new();
        let mut counts: Vec<usize> = Vec::new();
        for range in expanded_ranges {
            if let Some(last_range) = merged_ranges.last_mut() {
                debug_assert!(
                    last_range.context.start <= range.context.start,
                    "Last range: {last_range:?} Range: {range:?}"
                );
                if last_range.context.end >= range.context.start
                    || last_range.context.end.row + 1 == range.context.start.row
                {
                    last_range.context.end = range.context.end.max(last_range.context.end);
                    *counts.last_mut().unwrap() += 1;
                    continue;
                }
            }
            merged_ranges.push(range.clone());
            counts.push(1);
        }
        (merged_ranges, counts)
    }

    fn update_path_excerpts(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        buffer_snapshot: &BufferSnapshot,
        new: Vec<ExcerptRange<Point>>,
        cx: &mut Context<Self>,
    ) -> (Vec<ExcerptId>, bool) {
        let mut insert_after = self
            .excerpts_by_path
            .range(..path.clone())
            .next_back()
            .map(|(_, value)| *value.last().unwrap())
            .unwrap_or(ExcerptId::min());

        let existing = self
            .excerpts_by_path
            .get(&path)
            .cloned()
            .unwrap_or_default();

        let mut new_iter = new.into_iter().peekable();
        let mut existing_iter = existing.into_iter().peekable();

        let mut excerpt_ids = Vec::new();
        let mut to_remove = Vec::new();
        let mut to_insert: Vec<(ExcerptId, ExcerptRange<Point>)> = Vec::new();
        let mut added_a_new_excerpt = false;
        let snapshot = self.snapshot(cx);

        let mut next_excerpt_id =
            if let Some(last_entry) = self.snapshot.borrow().excerpt_ids.last() {
                last_entry.id.0 + 1
            } else {
                1
            };

        let mut next_excerpt_id = move || ExcerptId(post_inc(&mut next_excerpt_id));

        let mut excerpts_cursor = snapshot.excerpts.cursor::<Option<&Locator>>(&());
        excerpts_cursor.next();

        loop {
            let new = new_iter.peek();
            let existing = if let Some(existing_id) = existing_iter.peek() {
                let locator = snapshot.excerpt_locator_for_id(*existing_id);
                excerpts_cursor.seek_forward(&Some(locator), Bias::Left);
                if let Some(excerpt) = excerpts_cursor.item() {
                    if excerpt.buffer_id != buffer_snapshot.remote_id() {
                        to_remove.push(*existing_id);
                        existing_iter.next();
                        continue;
                    }
                    Some((
                        *existing_id,
                        excerpt.range.context.to_point(buffer_snapshot),
                    ))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some((last_id, last)) = to_insert.last_mut() {
                if let Some(new) = new
                    && last.context.end >= new.context.start
                {
                    last.context.end = last.context.end.max(new.context.end);
                    excerpt_ids.push(*last_id);
                    new_iter.next();
                    continue;
                }
                if let Some((existing_id, existing_range)) = &existing
                    && last.context.end >= existing_range.start
                {
                    last.context.end = last.context.end.max(existing_range.end);
                    to_remove.push(*existing_id);
                    self.snapshot
                        .borrow_mut()
                        .replaced_excerpts
                        .insert(*existing_id, *last_id);
                    existing_iter.next();
                    continue;
                }
            }

            match (new, existing) {
                (None, None) => break,
                (None, Some((existing_id, _))) => {
                    existing_iter.next();
                    to_remove.push(existing_id);
                    continue;
                }
                (Some(_), None) => {
                    added_a_new_excerpt = true;
                    let new_id = next_excerpt_id();
                    excerpt_ids.push(new_id);
                    to_insert.push((new_id, new_iter.next().unwrap()));
                    continue;
                }
                (Some(new), Some((_, existing_range))) => {
                    if existing_range.end < new.context.start {
                        let existing_id = existing_iter.next().unwrap();
                        to_remove.push(existing_id);
                        continue;
                    } else if existing_range.start > new.context.end {
                        let new_id = next_excerpt_id();
                        excerpt_ids.push(new_id);
                        to_insert.push((new_id, new_iter.next().unwrap()));
                        continue;
                    }

                    if existing_range.start == new.context.start
                        && existing_range.end == new.context.end
                    {
                        self.insert_excerpts_with_ids_after(
                            insert_after,
                            buffer.clone(),
                            mem::take(&mut to_insert),
                            cx,
                        );
                        insert_after = existing_iter.next().unwrap();
                        excerpt_ids.push(insert_after);
                        new_iter.next();
                    } else {
                        let existing_id = existing_iter.next().unwrap();
                        let new_id = next_excerpt_id();
                        self.snapshot
                            .borrow_mut()
                            .replaced_excerpts
                            .insert(existing_id, new_id);
                        to_remove.push(existing_id);
                        let mut range = new_iter.next().unwrap();
                        range.context.start = range.context.start.min(existing_range.start);
                        range.context.end = range.context.end.max(existing_range.end);
                        excerpt_ids.push(new_id);
                        to_insert.push((new_id, range));
                    }
                }
            };
        }

        self.insert_excerpts_with_ids_after(insert_after, buffer, to_insert, cx);
        self.remove_excerpts(to_remove, cx);
        if excerpt_ids.is_empty() {
            self.excerpts_by_path.remove(&path);
        } else {
            for excerpt_id in &excerpt_ids {
                self.paths_by_excerpt.insert(*excerpt_id, path.clone());
            }
            self.excerpts_by_path
                .insert(path, excerpt_ids.iter().dedup().cloned().collect());
        }

        (excerpt_ids, added_a_new_excerpt)
    }

    pub fn paths(&self) -> impl Iterator<Item = PathKey> + '_ {
        self.excerpts_by_path.keys().cloned()
    }

    pub fn remove_excerpts_for_path(&mut self, path: PathKey, cx: &mut Context<Self>) {
        if let Some(to_remove) = self.excerpts_by_path.remove(&path) {
            self.remove_excerpts(to_remove, cx)
        }
    }

    pub fn insert_excerpts_after<O>(
        &mut self,
        prev_excerpt_id: ExcerptId,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = ExcerptRange<O>>,
        cx: &mut Context<Self>,
    ) -> Vec<ExcerptId>
    where
        O: text::ToOffset,
    {
        let mut ids = Vec::new();
        let mut next_excerpt_id =
            if let Some(last_entry) = self.snapshot.borrow().excerpt_ids.last() {
                last_entry.id.0 + 1
            } else {
                1
            };
        self.insert_excerpts_with_ids_after(
            prev_excerpt_id,
            buffer,
            ranges.into_iter().map(|range| {
                let id = ExcerptId(post_inc(&mut next_excerpt_id));
                ids.push(id);
                (id, range)
            }),
            cx,
        );
        ids
    }

    pub fn insert_excerpts_with_ids_after<O>(
        &mut self,
        prev_excerpt_id: ExcerptId,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = (ExcerptId, ExcerptRange<O>)>,
        cx: &mut Context<Self>,
    ) where
        O: text::ToOffset,
    {
        assert_eq!(self.history.transaction_depth, 0);
        let mut ranges = ranges.into_iter().peekable();
        if ranges.peek().is_none() {
            return Default::default();
        }

        self.sync(cx);

        let buffer_snapshot = buffer.read(cx).snapshot();
        let buffer_id = buffer_snapshot.remote_id();

        let mut buffers = self.buffers.borrow_mut();
        let buffer_state = buffers.entry(buffer_id).or_insert_with(|| {
            self.buffer_changed_since_sync.replace(true);
            buffer.update(cx, |buffer, _| {
                buffer.record_changes(Rc::downgrade(&self.buffer_changed_since_sync));
            });
            BufferState {
                last_version: buffer_snapshot.version().clone(),
                last_non_text_state_update_count: buffer_snapshot.non_text_state_update_count(),
                excerpts: Default::default(),
                _subscriptions: [
                    cx.observe(&buffer, |_, _, cx| cx.notify()),
                    cx.subscribe(&buffer, Self::on_buffer_event),
                ],
                buffer: buffer.clone(),
            }
        });

        let mut snapshot = self.snapshot.borrow_mut();

        let mut prev_locator = snapshot.excerpt_locator_for_id(prev_excerpt_id).clone();
        let mut new_excerpt_ids = mem::take(&mut snapshot.excerpt_ids);
        let mut cursor = snapshot.excerpts.cursor::<Option<&Locator>>(&());
        let mut new_excerpts = cursor.slice(&prev_locator, Bias::Right);
        prev_locator = cursor.start().unwrap_or(Locator::min_ref()).clone();

        let edit_start = ExcerptOffset::new(new_excerpts.summary().text.len);
        new_excerpts.update_last(
            |excerpt| {
                excerpt.has_trailing_newline = true;
            },
            &(),
        );

        let next_locator = if let Some(excerpt) = cursor.item() {
            excerpt.locator.clone()
        } else {
            Locator::max()
        };

        let mut excerpts = Vec::new();
        while let Some((id, range)) = ranges.next() {
            let locator = Locator::between(&prev_locator, &next_locator);
            if let Err(ix) = buffer_state.excerpts.binary_search(&locator) {
                buffer_state.excerpts.insert(ix, locator.clone());
            }
            let range = ExcerptRange {
                context: buffer_snapshot.anchor_before(&range.context.start)
                    ..buffer_snapshot.anchor_after(&range.context.end),
                primary: buffer_snapshot.anchor_before(&range.primary.start)
                    ..buffer_snapshot.anchor_after(&range.primary.end),
            };
            excerpts.push((id, range.clone()));
            let excerpt = Excerpt::new(
                id,
                locator.clone(),
                buffer_id,
                buffer_snapshot.clone(),
                range,
                ranges.peek().is_some() || cursor.item().is_some(),
            );
            new_excerpts.push(excerpt, &());
            prev_locator = locator.clone();

            if let Some(last_mapping_entry) = new_excerpt_ids.last() {
                assert!(id > last_mapping_entry.id, "excerpt ids must be increasing");
            }
            new_excerpt_ids.push(ExcerptIdMapping { id, locator }, &());
        }

        let edit_end = ExcerptOffset::new(new_excerpts.summary().text.len);

        let suffix = cursor.suffix();
        let changed_trailing_excerpt = suffix.is_empty();
        new_excerpts.append(suffix, &());
        drop(cursor);
        snapshot.excerpts = new_excerpts;
        snapshot.excerpt_ids = new_excerpt_ids;
        if changed_trailing_excerpt {
            snapshot.trailing_excerpt_update_count += 1;
        }

        self.sync_diff_transforms(
            &mut snapshot,
            vec![Edit {
                old: edit_start..edit_start,
                new: edit_start..edit_end,
            }],
            DiffChangeKind::BufferEdited,
        );
        cx.emit(Event::Edited {
            singleton_buffer_edited: false,
            edited_buffer: None,
        });
        cx.emit(Event::ExcerptsAdded {
            buffer,
            predecessor: prev_excerpt_id,
            excerpts,
        });
        cx.notify();
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.sync(cx);
        let ids = self.excerpt_ids();
        let removed_buffer_ids = self
            .buffers
            .borrow_mut()
            .drain()
            .map(|(id, _)| id)
            .collect();
        self.excerpts_by_path.clear();
        self.paths_by_excerpt.clear();
        let mut snapshot = self.snapshot.borrow_mut();
        let start = ExcerptOffset::new(0);
        let prev_len = ExcerptOffset::new(snapshot.excerpts.summary().text.len);
        snapshot.excerpts = Default::default();
        snapshot.trailing_excerpt_update_count += 1;
        snapshot.is_dirty = false;
        snapshot.has_deleted_file = false;
        snapshot.has_conflict = false;
        snapshot.replaced_excerpts.clear();

        self.sync_diff_transforms(
            &mut snapshot,
            vec![Edit {
                old: start..prev_len,
                new: start..start,
            }],
            DiffChangeKind::BufferEdited,
        );
        cx.emit(Event::Edited {
            singleton_buffer_edited: false,
            edited_buffer: None,
        });
        cx.emit(Event::ExcerptsRemoved {
            ids,
            removed_buffer_ids,
        });
        cx.notify();
    }

    pub fn excerpts_for_buffer(
        &self,
        buffer_id: BufferId,
        cx: &App,
    ) -> Vec<(ExcerptId, ExcerptRange<text::Anchor>)> {
        let mut excerpts = Vec::new();
        let snapshot = self.read(cx);
        let buffers = self.buffers.borrow();
        let mut cursor = snapshot.excerpts.cursor::<Option<&Locator>>(&());
        if let Some(locators) = buffers.get(&buffer_id).map(|state| &state.excerpts) {
            for locator in locators {
                cursor.seek_forward(&Some(locator), Bias::Left);
                if let Some(excerpt) = cursor.item()
                    && excerpt.locator == *locator
                {
                    excerpts.push((excerpt.id, excerpt.range.clone()));
                }
            }
        }

        excerpts
    }

    pub fn excerpt_ranges_for_buffer(&self, buffer_id: BufferId, cx: &App) -> Vec<Range<Point>> {
        let snapshot = self.read(cx);
        let buffers = self.buffers.borrow();
        let mut excerpts = snapshot
            .excerpts
            .cursor::<Dimensions<Option<&Locator>, ExcerptDimension<Point>>>(&());
        let mut diff_transforms = snapshot
            .diff_transforms
            .cursor::<Dimensions<ExcerptDimension<Point>, OutputDimension<Point>>>(&());
        diff_transforms.next();
        let locators = buffers
            .get(&buffer_id)
            .into_iter()
            .flat_map(|state| &state.excerpts);
        let mut result = Vec::new();
        for locator in locators {
            excerpts.seek_forward(&Some(locator), Bias::Left);
            if let Some(excerpt) = excerpts.item()
                && excerpt.locator == *locator
            {
                let excerpt_start = excerpts.start().1.clone();
                let excerpt_end = ExcerptDimension(excerpt_start.0 + excerpt.text_summary.lines);

                diff_transforms.seek_forward(&excerpt_start, Bias::Left);
                let overshoot = excerpt_start.0 - diff_transforms.start().0.0;
                let start = diff_transforms.start().1.0 + overshoot;

                diff_transforms.seek_forward(&excerpt_end, Bias::Right);
                let overshoot = excerpt_end.0 - diff_transforms.start().0.0;
                let end = diff_transforms.start().1.0 + overshoot;

                result.push(start..end)
            }
        }
        result
    }

    pub fn excerpt_buffer_ids(&self) -> Vec<BufferId> {
        self.snapshot
            .borrow()
            .excerpts
            .iter()
            .map(|entry| entry.buffer_id)
            .collect()
    }

    pub fn excerpt_ids(&self) -> Vec<ExcerptId> {
        self.snapshot
            .borrow()
            .excerpts
            .iter()
            .map(|entry| entry.id)
            .collect()
    }

    pub fn excerpt_containing(
        &self,
        position: impl ToOffset,
        cx: &App,
    ) -> Option<(ExcerptId, Entity<Buffer>, Range<text::Anchor>)> {
        let snapshot = self.read(cx);
        let offset = position.to_offset(&snapshot);

        let mut cursor = snapshot.cursor::<usize>();
        cursor.seek(&offset);
        cursor
            .excerpt()
            .or_else(|| snapshot.excerpts.last())
            .map(|excerpt| {
                (
                    excerpt.id,
                    self.buffers
                        .borrow()
                        .get(&excerpt.buffer_id)
                        .unwrap()
                        .buffer
                        .clone(),
                    excerpt.range.context.clone(),
                )
            })
    }

    pub fn buffer_for_anchor(&self, anchor: Anchor, cx: &App) -> Option<Entity<Buffer>> {
        if let Some(buffer_id) = anchor.buffer_id {
            self.buffer(buffer_id)
        } else {
            let (_, buffer, _) = self.excerpt_containing(anchor, cx)?;
            Some(buffer)
        }
    }

    // If point is at the end of the buffer, the last excerpt is returned
    pub fn point_to_buffer_offset<T: ToOffset>(
        &self,
        point: T,
        cx: &App,
    ) -> Option<(Entity<Buffer>, usize)> {
        let snapshot = self.read(cx);
        let (buffer, offset) = snapshot.point_to_buffer_offset(point)?;
        Some((
            self.buffers
                .borrow()
                .get(&buffer.remote_id())?
                .buffer
                .clone(),
            offset,
        ))
    }

    // If point is at the end of the buffer, the last excerpt is returned
    pub fn point_to_buffer_point<T: ToPoint>(
        &self,
        point: T,
        cx: &App,
    ) -> Option<(Entity<Buffer>, Point, ExcerptId)> {
        let snapshot = self.read(cx);
        let (buffer, point, is_main_buffer) =
            snapshot.point_to_buffer_point(point.to_point(&snapshot))?;
        Some((
            self.buffers
                .borrow()
                .get(&buffer.remote_id())?
                .buffer
                .clone(),
            point,
            is_main_buffer,
        ))
    }

    pub fn buffer_point_to_anchor(
        &self,
        buffer: &Entity<Buffer>,
        point: Point,
        cx: &App,
    ) -> Option<Anchor> {
        let mut found = None;
        let snapshot = buffer.read(cx).snapshot();
        for (excerpt_id, range) in self.excerpts_for_buffer(snapshot.remote_id(), cx) {
            let start = range.context.start.to_point(&snapshot);
            let end = range.context.end.to_point(&snapshot);
            if start <= point && point < end {
                found = Some((snapshot.clip_point(point, Bias::Left), excerpt_id));
                break;
            }
            if point < start {
                found = Some((start, excerpt_id));
            }
            if point > end {
                found = Some((end, excerpt_id));
            }
        }

        found.map(|(point, excerpt_id)| {
            let text_anchor = snapshot.anchor_after(point);
            Anchor::in_buffer(excerpt_id, snapshot.remote_id(), text_anchor)
        })
    }

    pub fn remove_excerpts(
        &mut self,
        excerpt_ids: impl IntoIterator<Item = ExcerptId>,
        cx: &mut Context<Self>,
    ) {
        self.sync(cx);
        let ids = excerpt_ids.into_iter().collect::<Vec<_>>();
        if ids.is_empty() {
            return;
        }

        let mut buffers = self.buffers.borrow_mut();
        let mut snapshot = self.snapshot.borrow_mut();
        let mut new_excerpts = SumTree::default();
        let mut cursor = snapshot
            .excerpts
            .cursor::<Dimensions<Option<&Locator>, ExcerptOffset>>(&());
        let mut edits = Vec::new();
        let mut excerpt_ids = ids.iter().copied().peekable();
        let mut removed_buffer_ids = Vec::new();

        while let Some(excerpt_id) = excerpt_ids.next() {
            self.paths_by_excerpt.remove(&excerpt_id);
            // Seek to the next excerpt to remove, preserving any preceding excerpts.
            let locator = snapshot.excerpt_locator_for_id(excerpt_id);
            new_excerpts.append(cursor.slice(&Some(locator), Bias::Left), &());

            if let Some(mut excerpt) = cursor.item() {
                if excerpt.id != excerpt_id {
                    continue;
                }
                let mut old_start = cursor.start().1;

                // Skip over the removed excerpt.
                'remove_excerpts: loop {
                    if let Some(buffer_state) = buffers.get_mut(&excerpt.buffer_id) {
                        buffer_state.excerpts.retain(|l| l != &excerpt.locator);
                        if buffer_state.excerpts.is_empty() {
                            log::debug!(
                                "removing buffer and diff for buffer {}",
                                excerpt.buffer_id
                            );
                            buffers.remove(&excerpt.buffer_id);
                            removed_buffer_ids.push(excerpt.buffer_id);
                        }
                    }
                    cursor.next();

                    // Skip over any subsequent excerpts that are also removed.
                    if let Some(&next_excerpt_id) = excerpt_ids.peek() {
                        let next_locator = snapshot.excerpt_locator_for_id(next_excerpt_id);
                        if let Some(next_excerpt) = cursor.item()
                            && next_excerpt.locator == *next_locator
                        {
                            excerpt_ids.next();
                            excerpt = next_excerpt;
                            continue 'remove_excerpts;
                        }
                    }

                    break;
                }

                // When removing the last excerpt, remove the trailing newline from
                // the previous excerpt.
                if cursor.item().is_none() && old_start.value > 0 {
                    old_start.value -= 1;
                    new_excerpts.update_last(|e| e.has_trailing_newline = false, &());
                }

                // Push an edit for the removal of this run of excerpts.
                let old_end = cursor.start().1;
                let new_start = ExcerptOffset::new(new_excerpts.summary().text.len);
                edits.push(Edit {
                    old: old_start..old_end,
                    new: new_start..new_start,
                });
            }
        }
        let suffix = cursor.suffix();
        let changed_trailing_excerpt = suffix.is_empty();
        new_excerpts.append(suffix, &());
        drop(cursor);
        snapshot.excerpts = new_excerpts;
        for buffer_id in &removed_buffer_ids {
            self.diffs.remove(buffer_id);
            snapshot.diffs.remove(buffer_id);
        }

        if changed_trailing_excerpt {
            snapshot.trailing_excerpt_update_count += 1;
        }

        self.sync_diff_transforms(&mut snapshot, edits, DiffChangeKind::BufferEdited);
        self.buffer_changed_since_sync.replace(true);
        cx.emit(Event::Edited {
            singleton_buffer_edited: false,
            edited_buffer: None,
        });
        cx.emit(Event::ExcerptsRemoved {
            ids,
            removed_buffer_ids,
        });
        cx.notify();
    }

    pub fn wait_for_anchors<'a, Anchors: 'a + Iterator<Item = Anchor>>(
        &self,
        anchors: Anchors,
        cx: &mut Context<Self>,
    ) -> impl 'static + Future<Output = Result<()>> + use<Anchors> {
        let borrow = self.buffers.borrow();
        let mut error = None;
        let mut futures = Vec::new();
        for anchor in anchors {
            if let Some(buffer_id) = anchor.buffer_id {
                if let Some(buffer) = borrow.get(&buffer_id) {
                    buffer.buffer.update(cx, |buffer, _| {
                        futures.push(buffer.wait_for_anchors([anchor.text_anchor]))
                    });
                } else {
                    error = Some(anyhow!(
                        "buffer {buffer_id} is not part of this multi-buffer"
                    ));
                    break;
                }
            }
        }
        async move {
            if let Some(error) = error {
                Err(error)?;
            }
            for future in futures {
                future.await?;
            }
            Ok(())
        }
    }

    pub fn text_anchor_for_position<T: ToOffset>(
        &self,
        position: T,
        cx: &App,
    ) -> Option<(Entity<Buffer>, language::Anchor)> {
        let snapshot = self.read(cx);
        let anchor = snapshot.anchor_before(position);
        let buffer = self
            .buffers
            .borrow()
            .get(&anchor.buffer_id?)?
            .buffer
            .clone();
        Some((buffer, anchor.text_anchor))
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &language::BufferEvent,
        cx: &mut Context<Self>,
    ) {
        use language::BufferEvent;
        cx.emit(match event {
            BufferEvent::Edited => Event::Edited {
                singleton_buffer_edited: true,
                edited_buffer: Some(buffer),
            },
            BufferEvent::DirtyChanged => Event::DirtyChanged,
            BufferEvent::Saved => Event::Saved,
            BufferEvent::FileHandleChanged => Event::FileHandleChanged,
            BufferEvent::Reloaded => Event::Reloaded,
            BufferEvent::LanguageChanged => Event::LanguageChanged(buffer.read(cx).remote_id()),
            BufferEvent::Reparsed => Event::Reparsed(buffer.read(cx).remote_id()),
            BufferEvent::DiagnosticsUpdated => Event::DiagnosticsUpdated,
            BufferEvent::CapabilityChanged => {
                self.capability = buffer.read(cx).capability();
                return;
            }
            BufferEvent::Operation { .. } | BufferEvent::ReloadNeeded => return,
        });
    }

    fn buffer_diff_language_changed(&mut self, diff: Entity<BufferDiff>, cx: &mut Context<Self>) {
        self.sync(cx);
        let mut snapshot = self.snapshot.borrow_mut();
        let diff = diff.read(cx);
        let buffer_id = diff.buffer_id;
        let diff = diff.snapshot(cx);
        snapshot.diffs.insert(buffer_id, diff);
    }

    fn buffer_diff_changed(
        &mut self,
        diff: Entity<BufferDiff>,
        range: Range<text::Anchor>,
        cx: &mut Context<Self>,
    ) {
        self.sync(cx);
        self.buffer_changed_since_sync.replace(true);

        let diff = diff.read(cx);
        let buffer_id = diff.buffer_id;
        let buffers = self.buffers.borrow();
        let Some(buffer_state) = buffers.get(&buffer_id) else {
            return;
        };

        let buffer = buffer_state.buffer.read(cx);
        let diff_change_range = range.to_offset(buffer);

        let new_diff = diff.snapshot(cx);
        let mut snapshot = self.snapshot.borrow_mut();
        let base_text_changed = snapshot
            .diffs
            .get(&buffer_id)
            .is_none_or(|old_diff| !new_diff.base_texts_eq(old_diff));

        snapshot.diffs.insert(buffer_id, new_diff);

        let mut excerpt_edits = Vec::new();
        for locator in &buffer_state.excerpts {
            let mut cursor = snapshot
                .excerpts
                .cursor::<Dimensions<Option<&Locator>, ExcerptOffset>>(&());
            cursor.seek_forward(&Some(locator), Bias::Left);
            if let Some(excerpt) = cursor.item()
                && excerpt.locator == *locator
            {
                let excerpt_buffer_range = excerpt.range.context.to_offset(&excerpt.buffer);
                if diff_change_range.end < excerpt_buffer_range.start
                    || diff_change_range.start > excerpt_buffer_range.end
                {
                    continue;
                }
                let excerpt_start = cursor.start().1;
                let excerpt_len = ExcerptOffset::new(excerpt.text_summary.len);
                let diff_change_start_in_excerpt = ExcerptOffset::new(
                    diff_change_range
                        .start
                        .saturating_sub(excerpt_buffer_range.start),
                );
                let diff_change_end_in_excerpt = ExcerptOffset::new(
                    diff_change_range
                        .end
                        .saturating_sub(excerpt_buffer_range.start),
                );
                let edit_start = excerpt_start + diff_change_start_in_excerpt.min(excerpt_len);
                let edit_end = excerpt_start + diff_change_end_in_excerpt.min(excerpt_len);
                excerpt_edits.push(Edit {
                    old: edit_start..edit_end,
                    new: edit_start..edit_end,
                });
            }
        }

        self.sync_diff_transforms(
            &mut snapshot,
            excerpt_edits,
            DiffChangeKind::DiffUpdated {
                base_changed: base_text_changed,
            },
        );
        cx.emit(Event::Edited {
            singleton_buffer_edited: false,
            edited_buffer: None,
        });
    }

    pub fn all_buffers(&self) -> HashSet<Entity<Buffer>> {
        self.buffers
            .borrow()
            .values()
            .map(|state| state.buffer.clone())
            .collect()
    }

    pub fn all_buffer_ids(&self) -> Vec<BufferId> {
        self.buffers.borrow().keys().copied().collect()
    }

    pub fn buffer(&self, buffer_id: BufferId) -> Option<Entity<Buffer>> {
        self.buffers
            .borrow()
            .get(&buffer_id)
            .map(|state| state.buffer.clone())
    }

    pub fn language_at<T: ToOffset>(&self, point: T, cx: &App) -> Option<Arc<Language>> {
        self.point_to_buffer_offset(point, cx)
            .and_then(|(buffer, offset)| buffer.read(cx).language_at(offset))
    }

    pub fn language_settings<'a>(&'a self, cx: &'a App) -> Cow<'a, LanguageSettings> {
        let buffer_id = self
            .snapshot
            .borrow()
            .excerpts
            .first()
            .map(|excerpt| excerpt.buffer.remote_id());
        buffer_id
            .and_then(|buffer_id| self.buffer(buffer_id))
            .map(|buffer| {
                let buffer = buffer.read(cx);
                language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx)
            })
            .unwrap_or_else(move || self.language_settings_at(0, cx))
    }

    pub fn language_settings_at<'a, T: ToOffset>(
        &'a self,
        point: T,
        cx: &'a App,
    ) -> Cow<'a, LanguageSettings> {
        let mut language = None;
        let mut file = None;
        if let Some((buffer, offset)) = self.point_to_buffer_offset(point, cx) {
            let buffer = buffer.read(cx);
            language = buffer.language_at(offset);
            file = buffer.file();
        }
        language_settings(language.map(|l| l.name()), file, cx)
    }

    pub fn for_each_buffer(&self, mut f: impl FnMut(&Entity<Buffer>)) {
        self.buffers
            .borrow()
            .values()
            .for_each(|state| f(&state.buffer))
    }

    pub fn title<'a>(&'a self, cx: &'a App) -> Cow<'a, str> {
        if let Some(title) = self.title.as_ref() {
            return title.into();
        }

        if let Some(buffer) = self.as_singleton() {
            let buffer = buffer.read(cx);

            if let Some(file) = buffer.file() {
                return file.file_name(cx).to_string_lossy();
            }

            if let Some(title) = self.buffer_content_title(buffer) {
                return title;
            }
        };

        "untitled".into()
    }

    fn buffer_content_title(&self, buffer: &Buffer) -> Option<Cow<'_, str>> {
        let mut is_leading_whitespace = true;
        let mut count = 0;
        let mut prev_was_space = false;
        let mut title = String::new();

        for ch in buffer.snapshot().chars() {
            if is_leading_whitespace && ch.is_whitespace() {
                continue;
            }

            is_leading_whitespace = false;

            if ch == '\n' || count >= 40 {
                break;
            }

            if ch.is_whitespace() {
                if !prev_was_space {
                    title.push(' ');
                    count += 1;
                    prev_was_space = true;
                }
            } else {
                title.push(ch);
                count += 1;
                prev_was_space = false;
            }
        }

        let title = title.trim_end().to_string();

        if title.is_empty() {
            return None;
        }

        Some(title.into())
    }

    pub fn set_title(&mut self, title: String, cx: &mut Context<Self>) {
        self.title = Some(title);
        cx.notify();
    }

    /// Preserve preview tabs containing this multibuffer until additional edits occur.
    pub fn refresh_preview(&self, cx: &mut Context<Self>) {
        for buffer_state in self.buffers.borrow().values() {
            buffer_state
                .buffer
                .update(cx, |buffer, _cx| buffer.refresh_preview());
        }
    }

    /// Whether we should preserve the preview status of a tab containing this multi-buffer.
    pub fn preserve_preview(&self, cx: &App) -> bool {
        self.buffers
            .borrow()
            .values()
            .all(|state| state.buffer.read(cx).preserve_preview())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn is_parsing(&self, cx: &App) -> bool {
        self.as_singleton().unwrap().read(cx).is_parsing()
    }

    pub fn add_diff(&mut self, diff: Entity<BufferDiff>, cx: &mut Context<Self>) {
        let buffer_id = diff.read(cx).buffer_id;
        self.buffer_diff_changed(diff.clone(), text::Anchor::MIN..text::Anchor::MAX, cx);
        self.diffs.insert(buffer_id, DiffState::new(diff, cx));
    }

    pub fn diff_for(&self, buffer_id: BufferId) -> Option<Entity<BufferDiff>> {
        self.diffs.get(&buffer_id).map(|state| state.diff.clone())
    }

    pub fn expand_diff_hunks(&mut self, ranges: Vec<Range<Anchor>>, cx: &mut Context<Self>) {
        self.expand_or_collapse_diff_hunks(ranges, true, cx);
    }

    pub fn collapse_diff_hunks(&mut self, ranges: Vec<Range<Anchor>>, cx: &mut Context<Self>) {
        self.expand_or_collapse_diff_hunks(ranges, false, cx);
    }

    pub fn set_all_diff_hunks_expanded(&mut self, cx: &mut Context<Self>) {
        self.snapshot.borrow_mut().all_diff_hunks_expanded = true;
        self.expand_or_collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], true, cx);
    }

    pub fn all_diff_hunks_expanded(&self) -> bool {
        self.snapshot.borrow().all_diff_hunks_expanded
    }

    pub fn set_all_diff_hunks_collapsed(&mut self, cx: &mut Context<Self>) {
        self.snapshot.borrow_mut().all_diff_hunks_expanded = false;
        self.expand_or_collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], false, cx);
    }

    pub fn has_multiple_hunks(&self, cx: &App) -> bool {
        self.read(cx)
            .diff_hunks_in_range(Anchor::min()..Anchor::max())
            .nth(1)
            .is_some()
    }

    pub fn single_hunk_is_expanded(&self, range: Range<Anchor>, cx: &App) -> bool {
        let snapshot = self.read(cx);
        let mut cursor = snapshot.diff_transforms.cursor::<usize>(&());
        let offset_range = range.to_offset(&snapshot);
        cursor.seek(&offset_range.start, Bias::Left);
        while let Some(item) = cursor.item() {
            if *cursor.start() >= offset_range.end && *cursor.start() > offset_range.start {
                break;
            }
            if item.hunk_info().is_some() {
                return true;
            }
            cursor.next();
        }
        false
    }

    pub fn has_expanded_diff_hunks_in_ranges(&self, ranges: &[Range<Anchor>], cx: &App) -> bool {
        let snapshot = self.read(cx);
        let mut cursor = snapshot.diff_transforms.cursor::<usize>(&());
        for range in ranges {
            let range = range.to_point(&snapshot);
            let start = snapshot.point_to_offset(Point::new(range.start.row, 0));
            let end = snapshot.point_to_offset(Point::new(range.end.row + 1, 0));
            let start = start.saturating_sub(1);
            let end = snapshot.len().min(end + 1);
            cursor.seek(&start, Bias::Right);
            while let Some(item) = cursor.item() {
                if *cursor.start() >= end {
                    break;
                }
                if item.hunk_info().is_some() {
                    return true;
                }
                cursor.next();
            }
        }
        false
    }

    pub fn expand_or_collapse_diff_hunks_inner(
        &mut self,
        ranges: impl IntoIterator<Item = (Range<Point>, ExcerptId)>,
        expand: bool,
        cx: &mut Context<Self>,
    ) {
        if self.snapshot.borrow().all_diff_hunks_expanded && !expand {
            return;
        }
        self.sync(cx);
        let mut snapshot = self.snapshot.borrow_mut();
        let mut excerpt_edits = Vec::new();
        let mut last_hunk_row = None;
        for (range, end_excerpt_id) in ranges {
            for diff_hunk in snapshot.diff_hunks_in_range(range) {
                if diff_hunk.excerpt_id.cmp(&end_excerpt_id, &snapshot).is_gt() {
                    continue;
                }
                if last_hunk_row.is_some_and(|row| row >= diff_hunk.row_range.start) {
                    continue;
                }
                let start = Anchor::in_buffer(
                    diff_hunk.excerpt_id,
                    diff_hunk.buffer_id,
                    diff_hunk.buffer_range.start,
                );
                let end = Anchor::in_buffer(
                    diff_hunk.excerpt_id,
                    diff_hunk.buffer_id,
                    diff_hunk.buffer_range.end,
                );
                let start = snapshot.excerpt_offset_for_anchor(&start);
                let end = snapshot.excerpt_offset_for_anchor(&end);
                last_hunk_row = Some(diff_hunk.row_range.start);
                excerpt_edits.push(text::Edit {
                    old: start..end,
                    new: start..end,
                });
            }
        }

        self.sync_diff_transforms(
            &mut snapshot,
            excerpt_edits,
            DiffChangeKind::ExpandOrCollapseHunks { expand },
        );
        cx.emit(Event::DiffHunksToggled);
        cx.emit(Event::Edited {
            singleton_buffer_edited: false,
            edited_buffer: None,
        });
    }

    pub fn expand_or_collapse_diff_hunks(
        &mut self,
        ranges: Vec<Range<Anchor>>,
        expand: bool,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot.borrow().clone();
        let ranges = ranges.iter().map(move |range| {
            let end_excerpt_id = range.end.excerpt_id;
            let range = range.to_point(&snapshot);
            let mut peek_end = range.end;
            if range.end.row < snapshot.max_row().0 {
                peek_end = Point::new(range.end.row + 1, 0);
            };
            (range.start..peek_end, end_excerpt_id)
        });
        self.expand_or_collapse_diff_hunks_inner(ranges, expand, cx);
    }

    pub fn resize_excerpt(
        &mut self,
        id: ExcerptId,
        range: Range<text::Anchor>,
        cx: &mut Context<Self>,
    ) {
        self.sync(cx);

        let mut snapshot = self.snapshot.borrow_mut();
        let locator = snapshot.excerpt_locator_for_id(id);
        let mut new_excerpts = SumTree::default();
        let mut cursor = snapshot
            .excerpts
            .cursor::<Dimensions<Option<&Locator>, ExcerptOffset>>(&());
        let mut edits = Vec::<Edit<ExcerptOffset>>::new();

        let prefix = cursor.slice(&Some(locator), Bias::Left);
        new_excerpts.append(prefix, &());

        let mut excerpt = cursor.item().unwrap().clone();
        let old_text_len = ExcerptOffset::new(excerpt.text_summary.len);

        excerpt.range.context.start = range.start;
        excerpt.range.context.end = range.end;
        excerpt.max_buffer_row = range.end.to_point(&excerpt.buffer).row;

        excerpt.text_summary = excerpt
            .buffer
            .text_summary_for_range(excerpt.range.context.clone());

        let new_start_offset = ExcerptOffset::new(new_excerpts.summary().text.len);
        let old_start_offset = cursor.start().1;
        let new_text_len = ExcerptOffset::new(excerpt.text_summary.len);
        let edit = Edit {
            old: old_start_offset..old_start_offset + old_text_len,
            new: new_start_offset..new_start_offset + new_text_len,
        };

        if let Some(last_edit) = edits.last_mut() {
            if last_edit.old.end == edit.old.start {
                last_edit.old.end = edit.old.end;
                last_edit.new.end = edit.new.end;
            } else {
                edits.push(edit);
            }
        } else {
            edits.push(edit);
        }

        new_excerpts.push(excerpt, &());

        cursor.next();

        new_excerpts.append(cursor.suffix(), &());

        drop(cursor);
        snapshot.excerpts = new_excerpts;

        self.sync_diff_transforms(&mut snapshot, edits, DiffChangeKind::BufferEdited);
        cx.emit(Event::Edited {
            singleton_buffer_edited: false,
            edited_buffer: None,
        });
        cx.emit(Event::ExcerptsExpanded { ids: vec![id] });
        cx.notify();
    }

    pub fn expand_excerpts(
        &mut self,
        ids: impl IntoIterator<Item = ExcerptId>,
        line_count: u32,
        direction: ExpandExcerptDirection,
        cx: &mut Context<Self>,
    ) {
        if line_count == 0 {
            return;
        }
        self.sync(cx);
        if !self.excerpts_by_path.is_empty() {
            self.expand_excerpts_with_paths(ids, line_count, direction, cx);
            return;
        }
        let mut snapshot = self.snapshot.borrow_mut();

        let ids = ids.into_iter().collect::<Vec<_>>();
        let locators = snapshot.excerpt_locators_for_ids(ids.iter().copied());
        let mut new_excerpts = SumTree::default();
        let mut cursor = snapshot
            .excerpts
            .cursor::<Dimensions<Option<&Locator>, ExcerptOffset>>(&());
        let mut edits = Vec::<Edit<ExcerptOffset>>::new();

        for locator in &locators {
            let prefix = cursor.slice(&Some(locator), Bias::Left);
            new_excerpts.append(prefix, &());

            let mut excerpt = cursor.item().unwrap().clone();
            let old_text_len = ExcerptOffset::new(excerpt.text_summary.len);

            let up_line_count = if direction.should_expand_up() {
                line_count
            } else {
                0
            };

            let start_row = excerpt
                .range
                .context
                .start
                .to_point(&excerpt.buffer)
                .row
                .saturating_sub(up_line_count);
            let start_point = Point::new(start_row, 0);
            excerpt.range.context.start = excerpt.buffer.anchor_before(start_point);

            let down_line_count = if direction.should_expand_down() {
                line_count
            } else {
                0
            };

            let mut end_point = excerpt.buffer.clip_point(
                excerpt.range.context.end.to_point(&excerpt.buffer)
                    + Point::new(down_line_count, 0),
                Bias::Left,
            );
            end_point.column = excerpt.buffer.line_len(end_point.row);
            excerpt.range.context.end = excerpt.buffer.anchor_after(end_point);
            excerpt.max_buffer_row = end_point.row;

            excerpt.text_summary = excerpt
                .buffer
                .text_summary_for_range(excerpt.range.context.clone());

            let new_start_offset = ExcerptOffset::new(new_excerpts.summary().text.len);
            let old_start_offset = cursor.start().1;
            let new_text_len = ExcerptOffset::new(excerpt.text_summary.len);
            let edit = Edit {
                old: old_start_offset..old_start_offset + old_text_len,
                new: new_start_offset..new_start_offset + new_text_len,
            };

            if let Some(last_edit) = edits.last_mut() {
                if last_edit.old.end == edit.old.start {
                    last_edit.old.end = edit.old.end;
                    last_edit.new.end = edit.new.end;
                } else {
                    edits.push(edit);
                }
            } else {
                edits.push(edit);
            }

            new_excerpts.push(excerpt, &());

            cursor.next();
        }

        new_excerpts.append(cursor.suffix(), &());

        drop(cursor);
        snapshot.excerpts = new_excerpts;

        self.sync_diff_transforms(&mut snapshot, edits, DiffChangeKind::BufferEdited);
        cx.emit(Event::Edited {
            singleton_buffer_edited: false,
            edited_buffer: None,
        });
        cx.emit(Event::ExcerptsExpanded { ids });
        cx.notify();
    }

    fn sync(&self, cx: &App) {
        let changed = self.buffer_changed_since_sync.replace(false);
        if !changed {
            return;
        }

        let mut snapshot = self.snapshot.borrow_mut();
        let mut excerpts_to_edit = Vec::new();
        let mut non_text_state_updated = false;
        let mut is_dirty = false;
        let mut has_deleted_file = false;
        let mut has_conflict = false;
        let mut edited = false;
        let mut buffers = self.buffers.borrow_mut();
        for buffer_state in buffers.values_mut() {
            let buffer = buffer_state.buffer.read(cx);
            let version = buffer.version();
            let non_text_state_update_count = buffer.non_text_state_update_count();

            let buffer_edited = version.changed_since(&buffer_state.last_version);
            let buffer_non_text_state_updated =
                non_text_state_update_count > buffer_state.last_non_text_state_update_count;
            if buffer_edited || buffer_non_text_state_updated {
                buffer_state.last_version = version;
                buffer_state.last_non_text_state_update_count = non_text_state_update_count;
                excerpts_to_edit.extend(
                    buffer_state
                        .excerpts
                        .iter()
                        .map(|locator| (locator, buffer_state.buffer.clone(), buffer_edited)),
                );
            }

            edited |= buffer_edited;
            non_text_state_updated |= buffer_non_text_state_updated;
            is_dirty |= buffer.is_dirty();
            has_deleted_file |= buffer
                .file()
                .is_some_and(|file| file.disk_state() == DiskState::Deleted);
            has_conflict |= buffer.has_conflict();
        }
        if edited {
            snapshot.edit_count += 1;
        }
        if non_text_state_updated {
            snapshot.non_text_state_update_count += 1;
        }
        snapshot.is_dirty = is_dirty;
        snapshot.has_deleted_file = has_deleted_file;
        snapshot.has_conflict = has_conflict;

        for (id, diff) in self.diffs.iter() {
            if snapshot.diffs.get(id).is_none() {
                snapshot.diffs.insert(*id, diff.diff.read(cx).snapshot(cx));
            }
        }

        excerpts_to_edit.sort_unstable_by_key(|(locator, _, _)| *locator);

        let mut edits = Vec::new();
        let mut new_excerpts = SumTree::default();
        let mut cursor = snapshot
            .excerpts
            .cursor::<Dimensions<Option<&Locator>, ExcerptOffset>>(&());

        for (locator, buffer, buffer_edited) in excerpts_to_edit {
            new_excerpts.append(cursor.slice(&Some(locator), Bias::Left), &());
            let old_excerpt = cursor.item().unwrap();
            let buffer = buffer.read(cx);
            let buffer_id = buffer.remote_id();

            let mut new_excerpt;
            if buffer_edited {
                edits.extend(
                    buffer
                        .edits_since_in_range::<usize>(
                            old_excerpt.buffer.version(),
                            old_excerpt.range.context.clone(),
                        )
                        .map(|edit| {
                            let excerpt_old_start = cursor.start().1;
                            let excerpt_new_start =
                                ExcerptOffset::new(new_excerpts.summary().text.len);
                            let old_start = excerpt_old_start + ExcerptOffset::new(edit.old.start);
                            let old_end = excerpt_old_start + ExcerptOffset::new(edit.old.end);
                            let new_start = excerpt_new_start + ExcerptOffset::new(edit.new.start);
                            let new_end = excerpt_new_start + ExcerptOffset::new(edit.new.end);
                            Edit {
                                old: old_start..old_end,
                                new: new_start..new_end,
                            }
                        }),
                );

                new_excerpt = Excerpt::new(
                    old_excerpt.id,
                    locator.clone(),
                    buffer_id,
                    buffer.snapshot(),
                    old_excerpt.range.clone(),
                    old_excerpt.has_trailing_newline,
                );
            } else {
                new_excerpt = old_excerpt.clone();
                new_excerpt.buffer = buffer.snapshot();
            }

            new_excerpts.push(new_excerpt, &());
            cursor.next();
        }
        new_excerpts.append(cursor.suffix(), &());

        drop(cursor);
        snapshot.excerpts = new_excerpts;

        self.sync_diff_transforms(&mut snapshot, edits, DiffChangeKind::BufferEdited);
    }

    fn sync_diff_transforms(
        &self,
        snapshot: &mut MultiBufferSnapshot,
        excerpt_edits: Vec<text::Edit<ExcerptOffset>>,
        change_kind: DiffChangeKind,
    ) {
        if excerpt_edits.is_empty() {
            return;
        }

        let mut excerpts = snapshot.excerpts.cursor::<ExcerptOffset>(&());
        let mut old_diff_transforms = snapshot
            .diff_transforms
            .cursor::<Dimensions<ExcerptOffset, usize>>(&());
        let mut new_diff_transforms = SumTree::default();
        let mut old_expanded_hunks = HashSet::default();
        let mut output_edits = Vec::new();
        let mut output_delta = 0_isize;
        let mut at_transform_boundary = true;
        let mut end_of_current_insert = None;

        let mut excerpt_edits = excerpt_edits.into_iter().peekable();
        while let Some(edit) = excerpt_edits.next() {
            excerpts.seek_forward(&edit.new.start, Bias::Right);
            if excerpts.item().is_none() && *excerpts.start() == edit.new.start {
                excerpts.prev();
            }

            // Keep any transforms that are before the edit.
            if at_transform_boundary {
                at_transform_boundary = false;
                let transforms_before_edit = old_diff_transforms.slice(&edit.old.start, Bias::Left);
                self.append_diff_transforms(&mut new_diff_transforms, transforms_before_edit);
                if let Some(transform) = old_diff_transforms.item()
                    && old_diff_transforms.end().0 == edit.old.start
                    && old_diff_transforms.start().0 < edit.old.start
                {
                    self.push_diff_transform(&mut new_diff_transforms, transform.clone());
                    old_diff_transforms.next();
                }
            }

            // Compute the start of the edit in output coordinates.
            let edit_start_overshoot = (edit.old.start - old_diff_transforms.start().0).value;
            let edit_old_start = old_diff_transforms.start().1 + edit_start_overshoot;
            let edit_new_start = (edit_old_start as isize + output_delta) as usize;

            let changed_diff_hunks = self.recompute_diff_transforms_for_edit(
                &edit,
                &mut excerpts,
                &mut old_diff_transforms,
                &mut new_diff_transforms,
                &mut end_of_current_insert,
                &mut old_expanded_hunks,
                snapshot,
                change_kind,
            );

            // Compute the end of the edit in output coordinates.
            let edit_old_end_overshoot = edit.old.end - old_diff_transforms.start().0;
            let edit_new_end_overshoot = edit.new.end - new_diff_transforms.summary().excerpt_len();
            let edit_old_end = old_diff_transforms.start().1 + edit_old_end_overshoot.value;
            let edit_new_end =
                new_diff_transforms.summary().output.len + edit_new_end_overshoot.value;
            let output_edit = Edit {
                old: edit_old_start..edit_old_end,
                new: edit_new_start..edit_new_end,
            };

            output_delta += (output_edit.new.end - output_edit.new.start) as isize;
            output_delta -= (output_edit.old.end - output_edit.old.start) as isize;
            if changed_diff_hunks || matches!(change_kind, DiffChangeKind::BufferEdited) {
                output_edits.push(output_edit);
            }

            // If this is the last edit that intersects the current diff transform,
            // then recreate the content up to the end of this transform, to prepare
            // for reusing additional slices of the old transforms.
            if excerpt_edits
                .peek()
                .is_none_or(|next_edit| next_edit.old.start >= old_diff_transforms.end().0)
            {
                let keep_next_old_transform = (old_diff_transforms.start().0 >= edit.old.end)
                    && match old_diff_transforms.item() {
                        Some(DiffTransform::BufferContent {
                            inserted_hunk_info: Some(hunk),
                            ..
                        }) => excerpts.item().is_some_and(|excerpt| {
                            hunk.hunk_start_anchor.is_valid(&excerpt.buffer)
                        }),
                        _ => true,
                    };

                let mut excerpt_offset = edit.new.end;
                if !keep_next_old_transform {
                    excerpt_offset += old_diff_transforms.end().0 - edit.old.end;
                    old_diff_transforms.next();
                }

                old_expanded_hunks.clear();
                self.push_buffer_content_transform(
                    snapshot,
                    &mut new_diff_transforms,
                    excerpt_offset,
                    end_of_current_insert,
                );
                at_transform_boundary = true;
            }
        }

        // Keep any transforms that are after the last edit.
        self.append_diff_transforms(&mut new_diff_transforms, old_diff_transforms.suffix());

        // Ensure there's always at least one buffer content transform.
        if new_diff_transforms.is_empty() {
            new_diff_transforms.push(
                DiffTransform::BufferContent {
                    summary: Default::default(),
                    inserted_hunk_info: None,
                },
                &(),
            );
        }

        self.subscriptions.publish(output_edits);
        drop(old_diff_transforms);
        drop(excerpts);
        snapshot.diff_transforms = new_diff_transforms;
        snapshot.edit_count += 1;

        #[cfg(any(test, feature = "test-support"))]
        snapshot.check_invariants();
    }

    fn recompute_diff_transforms_for_edit(
        &self,
        edit: &Edit<TypedOffset<Excerpt>>,
        excerpts: &mut Cursor<Excerpt, TypedOffset<Excerpt>>,
        old_diff_transforms: &mut Cursor<DiffTransform, Dimensions<TypedOffset<Excerpt>, usize>>,
        new_diff_transforms: &mut SumTree<DiffTransform>,
        end_of_current_insert: &mut Option<(TypedOffset<Excerpt>, DiffTransformHunkInfo)>,
        old_expanded_hunks: &mut HashSet<DiffTransformHunkInfo>,
        snapshot: &MultiBufferSnapshot,
        change_kind: DiffChangeKind,
    ) -> bool {
        log::trace!(
            "recomputing diff transform for edit {:?} => {:?}",
            edit.old.start.value..edit.old.end.value,
            edit.new.start.value..edit.new.end.value
        );

        // Record which hunks were previously expanded.
        while let Some(item) = old_diff_transforms.item() {
            if let Some(hunk_info) = item.hunk_info() {
                log::trace!(
                    "previously expanded hunk at {}",
                    old_diff_transforms.start().0
                );
                old_expanded_hunks.insert(hunk_info);
            }
            if old_diff_transforms.end().0 > edit.old.end {
                break;
            }
            old_diff_transforms.next();
        }

        // Avoid querying diff hunks if there's no possibility of hunks being expanded.
        let all_diff_hunks_expanded = snapshot.all_diff_hunks_expanded;
        if old_expanded_hunks.is_empty()
            && change_kind == DiffChangeKind::BufferEdited
            && !all_diff_hunks_expanded
        {
            return false;
        }

        // Visit each excerpt that intersects the edit.
        let mut did_expand_hunks = false;
        while let Some(excerpt) = excerpts.item() {
            // Recompute the expanded hunks in the portion of the excerpt that
            // intersects the edit.
            if let Some(diff) = snapshot.diffs.get(&excerpt.buffer_id) {
                let buffer = &excerpt.buffer;
                let excerpt_start = *excerpts.start();
                let excerpt_end = excerpt_start + ExcerptOffset::new(excerpt.text_summary.len);
                let excerpt_buffer_start = excerpt.range.context.start.to_offset(buffer);
                let excerpt_buffer_end = excerpt_buffer_start + excerpt.text_summary.len;
                let edit_buffer_start =
                    excerpt_buffer_start + edit.new.start.value.saturating_sub(excerpt_start.value);
                let edit_buffer_end =
                    excerpt_buffer_start + edit.new.end.value.saturating_sub(excerpt_start.value);
                let edit_buffer_end = edit_buffer_end.min(excerpt_buffer_end);
                let edit_anchor_range =
                    buffer.anchor_before(edit_buffer_start)..buffer.anchor_after(edit_buffer_end);

                for hunk in diff.hunks_intersecting_range(edit_anchor_range, buffer) {
                    if hunk.is_created_file() && !all_diff_hunks_expanded {
                        continue;
                    }

                    let hunk_buffer_range = hunk.buffer_range.to_offset(buffer);
                    if hunk_buffer_range.start < excerpt_buffer_start {
                        log::trace!("skipping hunk that starts before excerpt");
                        continue;
                    }

                    let hunk_info = DiffTransformHunkInfo {
                        excerpt_id: excerpt.id,
                        hunk_start_anchor: hunk.buffer_range.start,
                        hunk_secondary_status: hunk.secondary_status,
                    };

                    let hunk_excerpt_start = excerpt_start
                        + ExcerptOffset::new(
                            hunk_buffer_range.start.saturating_sub(excerpt_buffer_start),
                        );
                    let hunk_excerpt_end = excerpt_end.min(
                        excerpt_start
                            + ExcerptOffset::new(hunk_buffer_range.end - excerpt_buffer_start),
                    );

                    self.push_buffer_content_transform(
                        snapshot,
                        new_diff_transforms,
                        hunk_excerpt_start,
                        *end_of_current_insert,
                    );

                    // For every existing hunk, determine if it was previously expanded
                    // and if it should currently be expanded.
                    let was_previously_expanded = old_expanded_hunks.contains(&hunk_info);
                    let should_expand_hunk = match &change_kind {
                        DiffChangeKind::DiffUpdated { base_changed: true } => {
                            was_previously_expanded || all_diff_hunks_expanded
                        }
                        DiffChangeKind::ExpandOrCollapseHunks { expand } => {
                            let intersects = hunk_buffer_range.is_empty()
                                || hunk_buffer_range.end > edit_buffer_start;
                            if *expand {
                                intersects || was_previously_expanded || all_diff_hunks_expanded
                            } else {
                                !intersects && (was_previously_expanded || all_diff_hunks_expanded)
                            }
                        }
                        _ => was_previously_expanded || all_diff_hunks_expanded,
                    };

                    if should_expand_hunk {
                        did_expand_hunks = true;
                        log::trace!(
                            "expanding hunk {:?}, excerpt:{:?}",
                            hunk_excerpt_start.value..hunk_excerpt_end.value,
                            excerpt.id
                        );

                        if !hunk.diff_base_byte_range.is_empty()
                            && hunk_buffer_range.start >= edit_buffer_start
                            && hunk_buffer_range.start <= excerpt_buffer_end
                        {
                            let base_text = diff.base_text();
                            let mut text_cursor =
                                base_text.as_rope().cursor(hunk.diff_base_byte_range.start);
                            let mut base_text_summary =
                                text_cursor.summary::<TextSummary>(hunk.diff_base_byte_range.end);

                            let mut has_trailing_newline = false;
                            if base_text_summary.last_line_chars > 0 {
                                base_text_summary += TextSummary::newline();
                                has_trailing_newline = true;
                            }

                            new_diff_transforms.push(
                                DiffTransform::DeletedHunk {
                                    base_text_byte_range: hunk.diff_base_byte_range.clone(),
                                    summary: base_text_summary,
                                    buffer_id: excerpt.buffer_id,
                                    hunk_info,
                                    has_trailing_newline,
                                },
                                &(),
                            );
                        }

                        if !hunk_buffer_range.is_empty() {
                            *end_of_current_insert =
                                Some((hunk_excerpt_end.min(excerpt_end), hunk_info));
                        }
                    }
                }
            }

            if excerpts.end() <= edit.new.end {
                excerpts.next();
            } else {
                break;
            }
        }

        did_expand_hunks || !old_expanded_hunks.is_empty()
    }

    fn append_diff_transforms(
        &self,
        new_transforms: &mut SumTree<DiffTransform>,
        subtree: SumTree<DiffTransform>,
    ) {
        if let Some(DiffTransform::BufferContent {
            inserted_hunk_info,
            summary,
        }) = subtree.first()
            && self.extend_last_buffer_content_transform(
                new_transforms,
                *inserted_hunk_info,
                *summary,
            )
        {
            let mut cursor = subtree.cursor::<()>(&());
            cursor.next();
            cursor.next();
            new_transforms.append(cursor.suffix(), &());
            return;
        }
        new_transforms.append(subtree, &());
    }

    fn push_diff_transform(
        &self,
        new_transforms: &mut SumTree<DiffTransform>,
        transform: DiffTransform,
    ) {
        if let DiffTransform::BufferContent {
            inserted_hunk_info: inserted_hunk_anchor,
            summary,
        } = transform
            && self.extend_last_buffer_content_transform(
                new_transforms,
                inserted_hunk_anchor,
                summary,
            )
        {
            return;
        }
        new_transforms.push(transform, &());
    }

    fn push_buffer_content_transform(
        &self,
        old_snapshot: &MultiBufferSnapshot,
        new_transforms: &mut SumTree<DiffTransform>,
        end_offset: ExcerptOffset,
        current_inserted_hunk: Option<(ExcerptOffset, DiffTransformHunkInfo)>,
    ) {
        let inserted_region = current_inserted_hunk.map(|(insertion_end_offset, hunk_info)| {
            (end_offset.min(insertion_end_offset), Some(hunk_info))
        });
        let unchanged_region = [(end_offset, None)];

        for (end_offset, inserted_hunk_info) in inserted_region.into_iter().chain(unchanged_region)
        {
            let start_offset = new_transforms.summary().excerpt_len();
            if end_offset <= start_offset {
                continue;
            }
            let summary_to_add = old_snapshot
                .text_summary_for_excerpt_offset_range::<TextSummary>(start_offset..end_offset);

            if !self.extend_last_buffer_content_transform(
                new_transforms,
                inserted_hunk_info,
                summary_to_add,
            ) {
                new_transforms.push(
                    DiffTransform::BufferContent {
                        summary: summary_to_add,
                        inserted_hunk_info,
                    },
                    &(),
                )
            }
        }
    }

    fn extend_last_buffer_content_transform(
        &self,
        new_transforms: &mut SumTree<DiffTransform>,
        new_inserted_hunk_info: Option<DiffTransformHunkInfo>,
        summary_to_add: TextSummary,
    ) -> bool {
        let mut did_extend = false;
        new_transforms.update_last(
            |last_transform| {
                if let DiffTransform::BufferContent {
                    summary,
                    inserted_hunk_info: inserted_hunk_anchor,
                } = last_transform
                    && *inserted_hunk_anchor == new_inserted_hunk_info
                {
                    *summary += summary_to_add;
                    did_extend = true;
                }
            },
            &(),
        );
        did_extend
    }
}

fn build_excerpt_ranges(
    ranges: impl IntoIterator<Item = Range<Point>>,
    context_line_count: u32,
    buffer_snapshot: &BufferSnapshot,
) -> Vec<ExcerptRange<Point>> {
    ranges
        .into_iter()
        .map(|range| {
            let start_row = range.start.row.saturating_sub(context_line_count);
            let start = Point::new(start_row, 0);
            let end_row = (range.end.row + context_line_count).min(buffer_snapshot.max_point().row);
            let end = Point::new(end_row, buffer_snapshot.line_len(end_row));
            ExcerptRange {
                context: start..end,
                primary: range,
            }
        })
        .collect()
}

#[cfg(any(test, feature = "test-support"))]
impl MultiBuffer {
    pub fn build_simple(text: &str, cx: &mut gpui::App) -> Entity<Self> {
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        cx.new(|cx| Self::singleton(buffer, cx))
    }

    pub fn build_multi<const COUNT: usize>(
        excerpts: [(&str, Vec<Range<Point>>); COUNT],
        cx: &mut gpui::App,
    ) -> Entity<Self> {
        let multi = cx.new(|_| Self::new(Capability::ReadWrite));
        for (text, ranges) in excerpts {
            let buffer = cx.new(|cx| Buffer::local(text, cx));
            let excerpt_ranges = ranges.into_iter().map(ExcerptRange::new);
            multi.update(cx, |multi, cx| {
                multi.push_excerpts(buffer, excerpt_ranges, cx)
            });
        }

        multi
    }

    pub fn build_from_buffer(buffer: Entity<Buffer>, cx: &mut gpui::App) -> Entity<Self> {
        cx.new(|cx| Self::singleton(buffer, cx))
    }

    pub fn build_random(rng: &mut impl rand::Rng, cx: &mut gpui::App) -> Entity<Self> {
        cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            let mutation_count = rng.random_range(1..=5);
            multibuffer.randomly_edit_excerpts(rng, mutation_count, cx);
            multibuffer
        })
    }

    pub fn randomly_edit(
        &mut self,
        rng: &mut impl rand::Rng,
        edit_count: usize,
        cx: &mut Context<Self>,
    ) {
        use util::RandomCharIter;

        let snapshot = self.read(cx);
        let mut edits: Vec<(Range<usize>, Arc<str>)> = Vec::new();
        let mut last_end = None;
        for _ in 0..edit_count {
            if last_end.is_some_and(|last_end| last_end >= snapshot.len()) {
                break;
            }

            let new_start = last_end.map_or(0, |last_end| last_end + 1);
            let end =
                snapshot.clip_offset(rng.random_range(new_start..=snapshot.len()), Bias::Right);
            let start = snapshot.clip_offset(rng.random_range(new_start..=end), Bias::Right);
            last_end = Some(end);

            let mut range = start..end;
            if rng.random_bool(0.2) {
                mem::swap(&mut range.start, &mut range.end);
            }

            let new_text_len = rng.random_range(0..10);
            let new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();

            edits.push((range, new_text.into()));
        }
        log::info!("mutating multi-buffer with {:?}", edits);
        drop(snapshot);

        self.edit(edits, None, cx);
    }

    pub fn randomly_edit_excerpts(
        &mut self,
        rng: &mut impl rand::Rng,
        mutation_count: usize,
        cx: &mut Context<Self>,
    ) {
        use rand::prelude::*;
        use std::env;
        use util::RandomCharIter;

        let max_excerpts = env::var("MAX_EXCERPTS")
            .map(|i| i.parse().expect("invalid `MAX_EXCERPTS` variable"))
            .unwrap_or(5);

        let mut buffers = Vec::new();
        for _ in 0..mutation_count {
            if rng.random_bool(0.05) {
                log::info!("Clearing multi-buffer");
                self.clear(cx);
                continue;
            } else if rng.random_bool(0.1) && !self.excerpt_ids().is_empty() {
                let ids = self.excerpt_ids();
                let mut excerpts = HashSet::default();
                for _ in 0..rng.random_range(0..ids.len()) {
                    excerpts.extend(ids.choose(rng).copied());
                }

                let line_count = rng.random_range(0..5);

                log::info!("Expanding excerpts {excerpts:?} by {line_count} lines");

                self.expand_excerpts(
                    excerpts.iter().cloned(),
                    line_count,
                    ExpandExcerptDirection::UpAndDown,
                    cx,
                );
                continue;
            }

            let excerpt_ids = self.excerpt_ids();
            if excerpt_ids.is_empty() || (rng.random() && excerpt_ids.len() < max_excerpts) {
                let buffer_handle = if rng.random() || self.buffers.borrow().is_empty() {
                    let text = RandomCharIter::new(&mut *rng).take(10).collect::<String>();
                    buffers.push(cx.new(|cx| Buffer::local(text, cx)));
                    let buffer = buffers.last().unwrap().read(cx);
                    log::info!(
                        "Creating new buffer {} with text: {:?}",
                        buffer.remote_id(),
                        buffer.text()
                    );
                    buffers.last().unwrap().clone()
                } else {
                    self.buffers
                        .borrow()
                        .values()
                        .choose(rng)
                        .unwrap()
                        .buffer
                        .clone()
                };

                let buffer = buffer_handle.read(cx);
                let buffer_text = buffer.text();
                let ranges = (0..rng.random_range(0..5))
                    .map(|_| {
                        let end_ix =
                            buffer.clip_offset(rng.random_range(0..=buffer.len()), Bias::Right);
                        let start_ix = buffer.clip_offset(rng.random_range(0..=end_ix), Bias::Left);
                        ExcerptRange::new(start_ix..end_ix)
                    })
                    .collect::<Vec<_>>();
                log::info!(
                    "Inserting excerpts from buffer {} and ranges {:?}: {:?}",
                    buffer_handle.read(cx).remote_id(),
                    ranges.iter().map(|r| &r.context).collect::<Vec<_>>(),
                    ranges
                        .iter()
                        .map(|r| &buffer_text[r.context.clone()])
                        .collect::<Vec<_>>()
                );

                let excerpt_id = self.push_excerpts(buffer_handle.clone(), ranges, cx);
                log::info!("Inserted with ids: {:?}", excerpt_id);
            } else {
                let remove_count = rng.random_range(1..=excerpt_ids.len());
                let mut excerpts_to_remove = excerpt_ids
                    .choose_multiple(rng, remove_count)
                    .cloned()
                    .collect::<Vec<_>>();
                let snapshot = self.snapshot.borrow();
                excerpts_to_remove.sort_unstable_by(|a, b| a.cmp(b, &snapshot));
                drop(snapshot);
                log::info!("Removing excerpts {:?}", excerpts_to_remove);
                self.remove_excerpts(excerpts_to_remove, cx);
            }
        }
    }

    pub fn randomly_mutate(
        &mut self,
        rng: &mut impl rand::Rng,
        mutation_count: usize,
        cx: &mut Context<Self>,
    ) {
        use rand::prelude::*;

        if rng.random_bool(0.7) || self.singleton {
            let buffer = self
                .buffers
                .borrow()
                .values()
                .choose(rng)
                .map(|state| state.buffer.clone());

            if let Some(buffer) = buffer {
                buffer.update(cx, |buffer, cx| {
                    if rng.random() {
                        buffer.randomly_edit(rng, mutation_count, cx);
                    } else {
                        buffer.randomly_undo_redo(rng, cx);
                    }
                });
            } else {
                self.randomly_edit(rng, mutation_count, cx);
            }
        } else {
            self.randomly_edit_excerpts(rng, mutation_count, cx);
        }

        self.check_invariants(cx);
    }

    fn check_invariants(&self, cx: &App) {
        self.read(cx).check_invariants();
    }
}

impl EventEmitter<Event> for MultiBuffer {}

impl MultiBufferSnapshot {
    pub fn text(&self) -> String {
        self.chunks(0..self.len(), false)
            .map(|chunk| chunk.text)
            .collect()
    }

    pub fn reversed_chars_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = char> + '_ {
        self.reversed_chunks_in_range(0..position.to_offset(self))
            .flat_map(|c| c.chars().rev())
    }

    fn reversed_chunks_in_range(&self, range: Range<usize>) -> ReversedMultiBufferChunks<'_> {
        let mut cursor = self.cursor::<usize>();
        cursor.seek(&range.end);
        let current_chunks = cursor.region().as_ref().map(|region| {
            let start_overshoot = range.start.saturating_sub(region.range.start);
            let end_overshoot = range.end - region.range.start;
            let end = (region.buffer_range.start + end_overshoot).min(region.buffer_range.end);
            let start = region.buffer_range.start + start_overshoot;
            region.buffer.reversed_chunks_in_range(start..end)
        });
        ReversedMultiBufferChunks {
            cursor,
            current_chunks,
            start: range.start,
            offset: range.end,
        }
    }

    pub fn chars_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = char> + '_ {
        let offset = position.to_offset(self);
        self.text_for_range(offset..self.len())
            .flat_map(|chunk| chunk.chars())
    }

    pub fn text_for_range<T: ToOffset>(&self, range: Range<T>) -> impl Iterator<Item = &str> + '_ {
        self.chunks(range, false).map(|chunk| chunk.text)
    }

    pub fn is_line_blank(&self, row: MultiBufferRow) -> bool {
        self.text_for_range(Point::new(row.0, 0)..Point::new(row.0, self.line_len(row)))
            .all(|chunk| chunk.matches(|c: char| !c.is_whitespace()).next().is_none())
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

    pub fn diff_hunks(&self) -> impl Iterator<Item = MultiBufferDiffHunk> + '_ {
        self.diff_hunks_in_range(Anchor::min()..Anchor::max())
    }

    pub fn diff_hunks_in_range<T: ToPoint>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = MultiBufferDiffHunk> + '_ {
        let query_range = range.start.to_point(self)..range.end.to_point(self);
        self.lift_buffer_metadata(query_range.clone(), move |buffer, buffer_range| {
            let diff = self.diffs.get(&buffer.remote_id())?;
            let buffer_start = buffer.anchor_before(buffer_range.start);
            let buffer_end = buffer.anchor_after(buffer_range.end);
            Some(
                diff.hunks_intersecting_range(buffer_start..buffer_end, buffer)
                    .filter_map(|hunk| {
                        if hunk.is_created_file() && !self.all_diff_hunks_expanded {
                            return None;
                        }
                        Some((hunk.range.clone(), hunk))
                    }),
            )
        })
        .filter_map(move |(range, hunk, excerpt)| {
            if range.start != range.end && range.end == query_range.start && !hunk.range.is_empty()
            {
                return None;
            }
            let end_row = if range.end.column == 0 {
                range.end.row
            } else {
                range.end.row + 1
            };
            Some(MultiBufferDiffHunk {
                row_range: MultiBufferRow(range.start.row)..MultiBufferRow(end_row),
                buffer_id: excerpt.buffer_id,
                excerpt_id: excerpt.id,
                buffer_range: hunk.buffer_range.clone(),
                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                secondary_status: hunk.secondary_status,
            })
        })
    }

    pub fn excerpt_ids_for_range<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = ExcerptId> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.cursor::<usize>();
        cursor.seek(&range.start);
        std::iter::from_fn(move || {
            let region = cursor.region()?;
            if region.range.start >= range.end {
                return None;
            }
            cursor.next_excerpt();
            Some(region.excerpt.id)
        })
    }

    pub fn buffer_ids_for_range<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = BufferId> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.cursor::<usize>();
        cursor.seek(&range.start);
        std::iter::from_fn(move || {
            let region = cursor.region()?;
            if region.range.start > range.end
                || region.range.start == range.end && region.range.start > range.start
            {
                return None;
            }
            cursor.next_excerpt();
            Some(region.excerpt.buffer_id)
        })
    }

    pub fn ranges_to_buffer_ranges<T: ToOffset>(
        &self,
        ranges: impl Iterator<Item = Range<T>>,
    ) -> impl Iterator<Item = (&BufferSnapshot, Range<usize>, ExcerptId)> {
        ranges.flat_map(|range| self.range_to_buffer_ranges(range).into_iter())
    }

    pub fn range_to_buffer_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Vec<(&BufferSnapshot, Range<usize>, ExcerptId)> {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);

        let mut cursor = self.cursor::<usize>();
        cursor.seek(&start);

        let mut result: Vec<(&BufferSnapshot, Range<usize>, ExcerptId)> = Vec::new();
        while let Some(region) = cursor.region() {
            if region.range.start > end {
                break;
            }
            if region.is_main_buffer {
                let start_overshoot = start.saturating_sub(region.range.start);
                let end_overshoot = end.saturating_sub(region.range.start);
                let start = region
                    .buffer_range
                    .end
                    .min(region.buffer_range.start + start_overshoot);
                let end = region
                    .buffer_range
                    .end
                    .min(region.buffer_range.start + end_overshoot);
                if let Some(prev) = result.last_mut().filter(|(_, prev_range, excerpt_id)| {
                    *excerpt_id == region.excerpt.id && prev_range.end == start
                }) {
                    prev.1.end = end;
                } else {
                    result.push((region.buffer, start..end, region.excerpt.id));
                }
            }
            cursor.next();
        }
        result
    }

    pub fn range_to_buffer_ranges_with_deleted_hunks<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = (&BufferSnapshot, Range<usize>, ExcerptId, Option<Anchor>)> + '_ {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);

        let mut cursor = self.cursor::<usize>();
        cursor.seek(&start);

        std::iter::from_fn(move || {
            let region = cursor.region()?;
            if region.range.start > end {
                return None;
            }
            let start_overshoot = start.saturating_sub(region.range.start);
            let end_overshoot = end.saturating_sub(region.range.start);
            let start = region
                .buffer_range
                .end
                .min(region.buffer_range.start + start_overshoot);
            let end = region
                .buffer_range
                .end
                .min(region.buffer_range.start + end_overshoot);

            let region_excerpt_id = region.excerpt.id;
            let deleted_hunk_anchor = if region.is_main_buffer {
                None
            } else {
                Some(self.anchor_before(region.range.start))
            };
            let result = (
                region.buffer,
                start..end,
                region_excerpt_id,
                deleted_hunk_anchor,
            );
            cursor.next();
            Some(result)
        })
    }

    /// Retrieves buffer metadata for the given range, and converts it into multi-buffer
    /// coordinates.
    ///
    /// The given callback will be called for every excerpt intersecting the given range. It will
    /// be passed the excerpt's buffer and the buffer range that the input range intersects.
    /// The callback should return an iterator of metadata items from that buffer, each paired
    /// with a buffer range.
    ///
    /// The returned iterator yields each of these metadata items, paired with its range in
    /// multi-buffer coordinates.
    fn lift_buffer_metadata<'a, D, M, I>(
        &'a self,
        query_range: Range<D>,
        get_buffer_metadata: impl 'a + Fn(&'a BufferSnapshot, Range<D>) -> Option<I>,
    ) -> impl Iterator<Item = (Range<D>, M, &'a Excerpt)> + 'a
    where
        I: Iterator<Item = (Range<D>, M)> + 'a,
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        let max_position = D::from_text_summary(&self.text_summary());
        let mut current_excerpt_metadata: Option<(ExcerptId, I)> = None;
        let mut cursor = self.cursor::<D>();

        // Find the excerpt and buffer offset where the given range ends.
        cursor.seek(&query_range.end);
        let mut range_end = None;
        while let Some(region) = cursor.region() {
            if region.is_main_buffer {
                let mut buffer_end = region.buffer_range.start;
                let overshoot = if query_range.end > region.range.start {
                    query_range.end - region.range.start
                } else {
                    D::default()
                };
                buffer_end.add_assign(&overshoot);
                range_end = Some((region.excerpt.id, buffer_end));
                break;
            }
            cursor.next();
        }

        cursor.seek(&query_range.start);

        if let Some(region) = cursor.region().filter(|region| !region.is_main_buffer)
            && region.range.start > D::zero(&())
        {
            cursor.prev()
        }

        iter::from_fn(move || {
            loop {
                let excerpt = cursor.excerpt()?;

                // If we have already retrieved metadata for this excerpt, continue to use it.
                let metadata_iter = if let Some((_, metadata)) = current_excerpt_metadata
                    .as_mut()
                    .filter(|(excerpt_id, _)| *excerpt_id == excerpt.id)
                {
                    Some(metadata)
                }
                // Otherwise, compute the intersection of the input range with the excerpt's range,
                // and retrieve the metadata for the resulting range.
                else {
                    let region = cursor.region()?;
                    let mut buffer_start;
                    if region.is_main_buffer {
                        buffer_start = region.buffer_range.start;
                        if query_range.start > region.range.start {
                            let overshoot = query_range.start - region.range.start;
                            buffer_start.add_assign(&overshoot);
                        }
                        buffer_start = buffer_start.min(region.buffer_range.end);
                    } else {
                        buffer_start = cursor.main_buffer_position()?;
                    };
                    let mut buffer_end = excerpt.range.context.end.summary::<D>(&excerpt.buffer);
                    if let Some((end_excerpt_id, end_buffer_offset)) = range_end
                        && excerpt.id == end_excerpt_id
                    {
                        buffer_end = buffer_end.min(end_buffer_offset);
                    }

                    get_buffer_metadata(&excerpt.buffer, buffer_start..buffer_end).map(|iterator| {
                        &mut current_excerpt_metadata.insert((excerpt.id, iterator)).1
                    })
                };

                // Visit each metadata item.
                if let Some((metadata_buffer_range, metadata)) =
                    metadata_iter.and_then(Iterator::next)
                {
                    // Find the multibuffer regions that contain the start and end of
                    // the metadata item's range.
                    if metadata_buffer_range.start > D::default() {
                        while let Some(region) = cursor.region() {
                            if region.is_main_buffer
                                && (region.buffer_range.end >= metadata_buffer_range.start
                                    || cursor.is_at_end_of_excerpt())
                            {
                                break;
                            }
                            cursor.next();
                        }
                    }
                    let start_region = cursor.region()?;
                    while let Some(region) = cursor.region() {
                        if region.is_main_buffer
                            && (region.buffer_range.end > metadata_buffer_range.end
                                || cursor.is_at_end_of_excerpt())
                        {
                            break;
                        }
                        cursor.next();
                    }
                    let end_region = cursor.region();

                    // Convert the metadata item's range into multibuffer coordinates.
                    let mut start_position = start_region.range.start;
                    let region_buffer_start = start_region.buffer_range.start;
                    if start_region.is_main_buffer
                        && metadata_buffer_range.start > region_buffer_start
                    {
                        start_position
                            .add_assign(&(metadata_buffer_range.start - region_buffer_start));
                        start_position = start_position.min(start_region.range.end);
                    }

                    let mut end_position = max_position;
                    if let Some(end_region) = &end_region {
                        end_position = end_region.range.start;
                        debug_assert!(end_region.is_main_buffer);
                        let region_buffer_start = end_region.buffer_range.start;
                        if metadata_buffer_range.end > region_buffer_start {
                            end_position
                                .add_assign(&(metadata_buffer_range.end - region_buffer_start));
                        }
                        end_position = end_position.min(end_region.range.end);
                    }

                    if start_position <= query_range.end && end_position >= query_range.start {
                        return Some((start_position..end_position, metadata, excerpt));
                    }
                }
                // When there are no more metadata items for this excerpt, move to the next excerpt.
                else {
                    current_excerpt_metadata.take();
                    if let Some((end_excerpt_id, _)) = range_end
                        && excerpt.id == end_excerpt_id
                    {
                        return None;
                    }
                    cursor.next_excerpt();
                }
            }
        })
    }

    pub fn diff_hunk_before<T: ToOffset>(&self, position: T) -> Option<MultiBufferRow> {
        let offset = position.to_offset(self);

        let mut cursor = self.cursor::<DimensionPair<usize, Point>>();
        cursor.seek(&DimensionPair {
            key: offset,
            value: None,
        });
        cursor.seek_to_start_of_current_excerpt();
        let excerpt = cursor.excerpt()?;

        let excerpt_end = excerpt.range.context.end.to_offset(&excerpt.buffer);
        let current_position = self
            .anchor_before(offset)
            .text_anchor
            .to_offset(&excerpt.buffer);
        let excerpt_end = excerpt
            .buffer
            .anchor_before(excerpt_end.min(current_position));

        if let Some(diff) = self.diffs.get(&excerpt.buffer_id) {
            for hunk in diff.hunks_intersecting_range_rev(
                excerpt.range.context.start..excerpt_end,
                &excerpt.buffer,
            ) {
                let hunk_end = hunk.buffer_range.end.to_offset(&excerpt.buffer);
                if hunk_end >= current_position {
                    continue;
                }
                let start =
                    Anchor::in_buffer(excerpt.id, excerpt.buffer_id, hunk.buffer_range.start)
                        .to_point(self);
                return Some(MultiBufferRow(start.row));
            }
        }

        loop {
            cursor.prev_excerpt();
            let excerpt = cursor.excerpt()?;

            let Some(diff) = self.diffs.get(&excerpt.buffer_id) else {
                continue;
            };
            let mut hunks =
                diff.hunks_intersecting_range_rev(excerpt.range.context.clone(), &excerpt.buffer);
            let Some(hunk) = hunks.next() else {
                continue;
            };
            let start = Anchor::in_buffer(excerpt.id, excerpt.buffer_id, hunk.buffer_range.start)
                .to_point(self);
            return Some(MultiBufferRow(start.row));
        }
    }

    pub fn has_diff_hunks(&self) -> bool {
        self.diffs.values().any(|diff| !diff.is_empty())
    }

    pub fn is_inside_word<T: ToOffset>(&self, position: T, for_completion: bool) -> bool {
        let position = position.to_offset(self);
        let classifier = self
            .char_classifier_at(position)
            .for_completion(for_completion);
        let next_char_kind = self.chars_at(position).next().map(|c| classifier.kind(c));
        let prev_char_kind = self
            .reversed_chars_at(position)
            .next()
            .map(|c| classifier.kind(c));
        prev_char_kind.zip(next_char_kind) == Some((CharKind::Word, CharKind::Word))
    }

    pub fn surrounding_word<T: ToOffset>(
        &self,
        start: T,
        for_completion: bool,
    ) -> (Range<usize>, Option<CharKind>) {
        let mut start = start.to_offset(self);
        let mut end = start;
        let mut next_chars = self.chars_at(start).peekable();
        let mut prev_chars = self.reversed_chars_at(start).peekable();

        let classifier = self
            .char_classifier_at(start)
            .for_completion(for_completion);

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

    pub fn char_kind_before<T: ToOffset>(
        &self,
        start: T,
        for_completion: bool,
    ) -> Option<CharKind> {
        let start = start.to_offset(self);
        let classifier = self
            .char_classifier_at(start)
            .for_completion(for_completion);
        self.reversed_chars_at(start)
            .next()
            .map(|ch| classifier.kind(ch))
    }

    pub fn is_singleton(&self) -> bool {
        self.singleton
    }

    pub fn as_singleton(&self) -> Option<(&ExcerptId, BufferId, &BufferSnapshot)> {
        if self.singleton {
            self.excerpts
                .iter()
                .next()
                .map(|e| (&e.id, e.buffer_id, &e.buffer))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.diff_transforms.summary().output.len
    }

    pub fn is_empty(&self) -> bool {
        self.excerpts.summary().text.len == 0
    }

    pub fn widest_line_number(&self) -> u32 {
        // widest_line_number is 0-based, so 1 is added to get the displayed line number.
        self.excerpts.summary().widest_line_number + 1
    }

    pub fn bytes_in_range<T: ToOffset>(&self, range: Range<T>) -> MultiBufferBytes<'_> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut excerpts = self.cursor::<usize>();
        excerpts.seek(&range.start);

        let mut chunk;
        let mut has_trailing_newline;
        let excerpt_bytes;
        if let Some(region) = excerpts.region() {
            let mut bytes = region.buffer.bytes_in_range(
                region.buffer_range.start + range.start - region.range.start
                    ..(region.buffer_range.start + range.end - region.range.start)
                        .min(region.buffer_range.end),
            );
            chunk = bytes.next().unwrap_or(&[][..]);
            excerpt_bytes = Some(bytes);
            has_trailing_newline = region.has_trailing_newline && range.end >= region.range.end;
            if chunk.is_empty() && has_trailing_newline {
                chunk = b"\n";
                has_trailing_newline = false;
            }
        } else {
            chunk = &[][..];
            excerpt_bytes = None;
            has_trailing_newline = false;
        };

        MultiBufferBytes {
            range,
            cursor: excerpts,
            excerpt_bytes,
            has_trailing_newline,
            chunk,
        }
    }

    pub fn reversed_bytes_in_range<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> ReversedMultiBufferBytes<'_> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut chunks = self.reversed_chunks_in_range(range.clone());
        let chunk = chunks.next().map_or(&[][..], |c| c.as_bytes());
        ReversedMultiBufferBytes {
            range,
            chunks,
            chunk,
        }
    }

    pub fn row_infos(&self, start_row: MultiBufferRow) -> MultiBufferRows<'_> {
        let mut cursor = self.cursor::<Point>();
        cursor.seek(&Point::new(start_row.0, 0));
        let mut result = MultiBufferRows {
            point: Point::new(0, 0),
            is_empty: self.excerpts.is_empty(),
            is_singleton: self.is_singleton(),
            cursor,
        };
        result.seek(start_row);
        result
    }

    pub fn chunks<T: ToOffset>(
        &self,
        range: Range<T>,
        language_aware: bool,
    ) -> MultiBufferChunks<'_> {
        let mut chunks = MultiBufferChunks {
            excerpt_offset_range: ExcerptOffset::new(0)..ExcerptOffset::new(0),
            range: 0..0,
            excerpts: self.excerpts.cursor(&()),
            diff_transforms: self.diff_transforms.cursor(&()),
            diffs: &self.diffs,
            diff_base_chunks: None,
            excerpt_chunks: None,
            buffer_chunk: None,
            language_aware,
        };
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        chunks.seek(range);
        chunks
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        self.clip_dimension(offset, bias, text::BufferSnapshot::clip_offset)
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.clip_dimension(point, bias, text::BufferSnapshot::clip_point)
    }

    pub fn clip_offset_utf16(&self, offset: OffsetUtf16, bias: Bias) -> OffsetUtf16 {
        self.clip_dimension(offset, bias, text::BufferSnapshot::clip_offset_utf16)
    }

    pub fn clip_point_utf16(&self, point: Unclipped<PointUtf16>, bias: Bias) -> PointUtf16 {
        self.clip_dimension(point.0, bias, |buffer, point, bias| {
            buffer.clip_point_utf16(Unclipped(point), bias)
        })
    }

    pub fn offset_to_point(&self, offset: usize) -> Point {
        self.convert_dimension(offset, text::BufferSnapshot::offset_to_point)
    }

    pub fn offset_to_point_utf16(&self, offset: usize) -> PointUtf16 {
        self.convert_dimension(offset, text::BufferSnapshot::offset_to_point_utf16)
    }

    pub fn point_to_point_utf16(&self, point: Point) -> PointUtf16 {
        self.convert_dimension(point, text::BufferSnapshot::point_to_point_utf16)
    }

    pub fn point_to_offset(&self, point: Point) -> usize {
        self.convert_dimension(point, text::BufferSnapshot::point_to_offset)
    }

    pub fn offset_utf16_to_offset(&self, offset: OffsetUtf16) -> usize {
        self.convert_dimension(offset, text::BufferSnapshot::offset_utf16_to_offset)
    }

    pub fn offset_to_offset_utf16(&self, offset: usize) -> OffsetUtf16 {
        self.convert_dimension(offset, text::BufferSnapshot::offset_to_offset_utf16)
    }

    pub fn point_utf16_to_offset(&self, point: PointUtf16) -> usize {
        self.convert_dimension(point, text::BufferSnapshot::point_utf16_to_offset)
    }

    fn clip_dimension<D>(
        &self,
        position: D,
        bias: Bias,
        clip_buffer_position: fn(&text::BufferSnapshot, D, Bias) -> D,
    ) -> D
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        let mut cursor = self.cursor();
        cursor.seek(&position);
        if let Some(region) = cursor.region() {
            if position >= region.range.end {
                return region.range.end;
            }
            let overshoot = position - region.range.start;
            let mut buffer_position = region.buffer_range.start;
            buffer_position.add_assign(&overshoot);
            let clipped_buffer_position =
                clip_buffer_position(region.buffer, buffer_position, bias);
            let mut position = region.range.start;
            position.add_assign(&(clipped_buffer_position - region.buffer_range.start));
            position
        } else {
            D::from_text_summary(&self.text_summary())
        }
    }

    fn convert_dimension<D1, D2>(
        &self,
        key: D1,
        convert_buffer_dimension: fn(&text::BufferSnapshot, D1) -> D2,
    ) -> D2
    where
        D1: TextDimension + Ord + Sub<D1, Output = D1>,
        D2: TextDimension + Ord + Sub<D2, Output = D2>,
    {
        let mut cursor = self.cursor::<DimensionPair<D1, D2>>();
        cursor.seek(&DimensionPair { key, value: None });
        if let Some(region) = cursor.region() {
            if key >= region.range.end.key {
                return region.range.end.value.unwrap();
            }
            let start_key = region.range.start.key;
            let start_value = region.range.start.value.unwrap();
            let buffer_start_key = region.buffer_range.start.key;
            let buffer_start_value = region.buffer_range.start.value.unwrap();
            let mut buffer_key = buffer_start_key;
            buffer_key.add_assign(&(key - start_key));
            let buffer_value = convert_buffer_dimension(region.buffer, buffer_key);
            let mut result = start_value;
            result.add_assign(&(buffer_value - buffer_start_value));
            result
        } else {
            D2::from_text_summary(&self.text_summary())
        }
    }

    pub fn point_to_buffer_offset<T: ToOffset>(
        &self,
        point: T,
    ) -> Option<(&BufferSnapshot, usize)> {
        let offset = point.to_offset(self);
        let mut cursor = self.cursor::<usize>();
        cursor.seek(&offset);
        let region = cursor.region()?;
        let overshoot = offset - region.range.start;
        let buffer_offset = region.buffer_range.start + overshoot;
        if buffer_offset == region.buffer.len() + 1
            && region.has_trailing_newline
            && !region.is_main_buffer
        {
            return Some((&cursor.excerpt()?.buffer, cursor.main_buffer_position()?));
        } else if buffer_offset > region.buffer.len() {
            return None;
        }
        Some((region.buffer, buffer_offset))
    }

    pub fn point_to_buffer_point(
        &self,
        point: Point,
    ) -> Option<(&BufferSnapshot, Point, ExcerptId)> {
        let mut cursor = self.cursor::<Point>();
        cursor.seek(&point);
        let region = cursor.region()?;
        let overshoot = point - region.range.start;
        let buffer_point = region.buffer_range.start + overshoot;
        let excerpt = cursor.excerpt()?;
        if buffer_point == region.buffer.max_point() + Point::new(1, 0)
            && region.has_trailing_newline
            && !region.is_main_buffer
        {
            return Some((&excerpt.buffer, cursor.main_buffer_position()?, excerpt.id));
        } else if buffer_point > region.buffer.max_point() {
            return None;
        }
        Some((region.buffer, buffer_point, excerpt.id))
    }

    pub fn suggested_indents(
        &self,
        rows: impl IntoIterator<Item = u32>,
        cx: &App,
    ) -> BTreeMap<MultiBufferRow, IndentSize> {
        let mut result = BTreeMap::new();

        let mut rows_for_excerpt = Vec::new();
        let mut cursor = self.cursor::<Point>();
        let mut rows = rows.into_iter().peekable();
        let mut prev_row = u32::MAX;
        let mut prev_language_indent_size = IndentSize::default();

        while let Some(row) = rows.next() {
            cursor.seek(&Point::new(row, 0));
            let Some(region) = cursor.region() else {
                continue;
            };

            // Retrieve the language and indent size once for each disjoint region being indented.
            let single_indent_size = if row.saturating_sub(1) == prev_row {
                prev_language_indent_size
            } else {
                region
                    .buffer
                    .language_indent_size_at(Point::new(row, 0), cx)
            };
            prev_language_indent_size = single_indent_size;
            prev_row = row;

            let start_buffer_row = region.buffer_range.start.row;
            let start_multibuffer_row = region.range.start.row;
            let end_multibuffer_row = region.range.end.row;

            rows_for_excerpt.push(row);
            while let Some(next_row) = rows.peek().copied() {
                if end_multibuffer_row > next_row {
                    rows_for_excerpt.push(next_row);
                    rows.next();
                } else {
                    break;
                }
            }

            let buffer_rows = rows_for_excerpt
                .drain(..)
                .map(|row| start_buffer_row + row - start_multibuffer_row);
            let buffer_indents = region
                .buffer
                .suggested_indents(buffer_rows, single_indent_size);
            let multibuffer_indents = buffer_indents.into_iter().map(|(row, indent)| {
                (
                    MultiBufferRow(start_multibuffer_row + row - start_buffer_row),
                    indent,
                )
            });
            result.extend(multibuffer_indents);
        }

        result
    }

    pub fn indent_size_for_line(&self, row: MultiBufferRow) -> IndentSize {
        if let Some((buffer, range)) = self.buffer_line_for_row(row) {
            let mut size = buffer.indent_size_for_line(range.start.row);
            size.len = size
                .len
                .min(range.end.column)
                .saturating_sub(range.start.column);
            size
        } else {
            IndentSize::spaces(0)
        }
    }

    pub fn line_indent_for_row(&self, row: MultiBufferRow) -> LineIndent {
        if let Some((buffer, range)) = self.buffer_line_for_row(row) {
            LineIndent::from_iter(buffer.text_for_range(range).flat_map(|s| s.chars()))
        } else {
            LineIndent::spaces(0)
        }
    }

    pub fn indent_and_comment_for_line(&self, row: MultiBufferRow, cx: &App) -> String {
        let mut indent = self.indent_size_for_line(row).chars().collect::<String>();

        if self.language_settings(cx).extend_comment_on_newline
            && let Some(language_scope) = self.language_scope_at(Point::new(row.0, 0))
        {
            let delimiters = language_scope.line_comment_prefixes();
            for delimiter in delimiters {
                if *self
                    .chars_at(Point::new(row.0, indent.len() as u32))
                    .take(delimiter.chars().count())
                    .collect::<String>()
                    .as_str()
                    == **delimiter
                {
                    indent.push_str(delimiter);
                    break;
                }
            }
        }

        indent
    }

    pub fn is_line_whitespace_upto<T>(&self, position: T) -> bool
    where
        T: ToOffset,
    {
        for char in self.reversed_chars_at(position) {
            if !char.is_whitespace() {
                return false;
            }
            if char == '\n' {
                return true;
            }
        }
        true
    }

    pub fn prev_non_blank_row(&self, mut row: MultiBufferRow) -> Option<MultiBufferRow> {
        while row.0 > 0 {
            row.0 -= 1;
            if !self.is_line_blank(row) {
                return Some(row);
            }
        }
        None
    }

    pub fn line_len(&self, row: MultiBufferRow) -> u32 {
        if let Some((_, range)) = self.buffer_line_for_row(row) {
            range.end.column - range.start.column
        } else {
            0
        }
    }

    pub fn buffer_line_for_row(
        &self,
        row: MultiBufferRow,
    ) -> Option<(&BufferSnapshot, Range<Point>)> {
        let mut cursor = self.cursor::<Point>();
        let point = Point::new(row.0, 0);
        cursor.seek(&point);
        let region = cursor.region()?;
        let overshoot = point.min(region.range.end) - region.range.start;
        let buffer_point = region.buffer_range.start + overshoot;
        if buffer_point.row > region.buffer_range.end.row {
            return None;
        }
        let line_start = Point::new(buffer_point.row, 0).max(region.buffer_range.start);
        let line_end = Point::new(buffer_point.row, region.buffer.line_len(buffer_point.row))
            .min(region.buffer_range.end);
        Some((region.buffer, line_start..line_end))
    }

    pub fn max_point(&self) -> Point {
        self.text_summary().lines
    }

    pub fn max_row(&self) -> MultiBufferRow {
        MultiBufferRow(self.text_summary().lines.row)
    }

    pub fn text_summary(&self) -> TextSummary {
        self.diff_transforms.summary().output
    }

    pub fn text_summary_for_range<D, O>(&self, range: Range<O>) -> D
    where
        D: TextDimension,
        O: ToOffset,
    {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self
            .diff_transforms
            .cursor::<Dimensions<usize, ExcerptOffset>>(&());
        cursor.seek(&range.start, Bias::Right);

        let Some(first_transform) = cursor.item() else {
            return D::from_text_summary(&TextSummary::default());
        };

        let diff_transform_start = cursor.start().0;
        let diff_transform_end = cursor.end().0;
        let diff_start = range.start;
        let start_overshoot = diff_start - diff_transform_start;
        let end_overshoot = std::cmp::min(range.end, diff_transform_end) - diff_transform_start;

        let mut result = match first_transform {
            DiffTransform::BufferContent { .. } => {
                let excerpt_start = cursor.start().1 + ExcerptOffset::new(start_overshoot);
                let excerpt_end = cursor.start().1 + ExcerptOffset::new(end_overshoot);
                self.text_summary_for_excerpt_offset_range(excerpt_start..excerpt_end)
            }
            DiffTransform::DeletedHunk {
                buffer_id,
                base_text_byte_range,
                has_trailing_newline,
                ..
            } => {
                let buffer_start = base_text_byte_range.start + start_overshoot;
                let mut buffer_end = base_text_byte_range.start + end_overshoot;
                let Some(base_text) = self.diffs.get(buffer_id).map(|diff| diff.base_text()) else {
                    panic!("{:?} is in non-existent deleted hunk", range.start)
                };

                let include_trailing_newline =
                    *has_trailing_newline && range.end >= diff_transform_end;
                if include_trailing_newline {
                    buffer_end -= 1;
                }

                let mut summary =
                    base_text.text_summary_for_range::<D, _>(buffer_start..buffer_end);

                if include_trailing_newline {
                    summary.add_assign(&D::from_text_summary(&TextSummary::newline()))
                }

                summary
            }
        };
        if range.end < diff_transform_end {
            return result;
        }

        cursor.next();
        result.add_assign(&D::from_text_summary(
            &cursor.summary(&range.end, Bias::Right),
        ));

        let Some(last_transform) = cursor.item() else {
            return result;
        };

        let overshoot = range.end - cursor.start().0;
        let suffix = match last_transform {
            DiffTransform::BufferContent { .. } => {
                let end = cursor.start().1 + ExcerptOffset::new(overshoot);
                self.text_summary_for_excerpt_offset_range::<D>(cursor.start().1..end)
            }
            DiffTransform::DeletedHunk {
                base_text_byte_range,
                buffer_id,
                has_trailing_newline,
                ..
            } => {
                let buffer_end = base_text_byte_range.start + overshoot;
                let Some(base_text) = self.diffs.get(buffer_id).map(|diff| diff.base_text()) else {
                    panic!("{:?} is in non-existent deleted hunk", range.end)
                };

                let mut suffix = base_text
                    .text_summary_for_range::<D, _>(base_text_byte_range.start..buffer_end);
                if *has_trailing_newline && buffer_end == base_text_byte_range.end + 1 {
                    suffix.add_assign(&D::from_text_summary(&TextSummary::newline()))
                }
                suffix
            }
        };

        result.add_assign(&suffix);
        result
    }

    fn text_summary_for_excerpt_offset_range<D>(&self, mut range: Range<ExcerptOffset>) -> D
    where
        D: TextDimension,
    {
        // let mut range = range.start..range.end;
        let mut summary = D::zero(&());
        let mut cursor = self.excerpts.cursor::<ExcerptOffset>(&());
        cursor.seek(&range.start, Bias::Right);
        if let Some(excerpt) = cursor.item() {
            let mut end_before_newline = cursor.end();
            if excerpt.has_trailing_newline {
                end_before_newline -= ExcerptOffset::new(1);
            }

            let excerpt_start = excerpt.range.context.start.to_offset(&excerpt.buffer);
            let start_in_excerpt = excerpt_start + (range.start - *cursor.start()).value;
            let end_in_excerpt =
                excerpt_start + (cmp::min(end_before_newline, range.end) - *cursor.start()).value;
            summary.add_assign(
                &excerpt
                    .buffer
                    .text_summary_for_range(start_in_excerpt..end_in_excerpt),
            );

            if range.end > end_before_newline {
                summary.add_assign(&D::from_text_summary(&TextSummary::from("\n")));
            }

            cursor.next();
        }

        if range.end > *cursor.start() {
            summary.add_assign(
                &cursor
                    .summary::<_, ExcerptDimension<D>>(&range.end, Bias::Right)
                    .0,
            );
            if let Some(excerpt) = cursor.item() {
                range.end = cmp::max(*cursor.start(), range.end);

                let excerpt_start = excerpt.range.context.start.to_offset(&excerpt.buffer);
                let end_in_excerpt = excerpt_start + (range.end - *cursor.start()).value;
                summary.add_assign(
                    &excerpt
                        .buffer
                        .text_summary_for_range(excerpt_start..end_in_excerpt),
                );
            }
        }

        summary
    }

    pub fn summary_for_anchor<D>(&self, anchor: &Anchor) -> D
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        self.summaries_for_anchors([anchor])[0]
    }

    fn resolve_summary_for_anchor<D>(
        &self,
        anchor: &Anchor,
        excerpt_position: D,
        diff_transforms: &mut Cursor<
            DiffTransform,
            Dimensions<ExcerptDimension<D>, OutputDimension<D>>,
        >,
    ) -> D
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        loop {
            let transform_end_position = diff_transforms.end().0.0;
            let at_transform_end =
                excerpt_position == transform_end_position && diff_transforms.item().is_some();
            if at_transform_end && anchor.text_anchor.bias == Bias::Right {
                diff_transforms.next();
                continue;
            }

            let mut position = diff_transforms.start().1.0;
            match diff_transforms.item() {
                Some(DiffTransform::DeletedHunk {
                    buffer_id,
                    base_text_byte_range,
                    ..
                }) => {
                    if let Some(diff_base_anchor) = &anchor.diff_base_anchor
                        && let Some(base_text) =
                            self.diffs.get(buffer_id).map(|diff| diff.base_text())
                        && base_text.can_resolve(diff_base_anchor)
                    {
                        let base_text_offset = diff_base_anchor.to_offset(base_text);
                        if base_text_offset >= base_text_byte_range.start
                            && base_text_offset <= base_text_byte_range.end
                        {
                            let position_in_hunk = base_text.text_summary_for_range::<D, _>(
                                base_text_byte_range.start..base_text_offset,
                            );
                            position.add_assign(&position_in_hunk);
                        } else if at_transform_end {
                            diff_transforms.next();
                            continue;
                        }
                    }
                }
                _ => {
                    if at_transform_end && anchor.diff_base_anchor.is_some() {
                        diff_transforms.next();
                        continue;
                    }
                    let overshoot = excerpt_position - diff_transforms.start().0.0;
                    position.add_assign(&overshoot);
                }
            }

            return position;
        }
    }

    fn excerpt_offset_for_anchor(&self, anchor: &Anchor) -> ExcerptOffset {
        let mut cursor = self
            .excerpts
            .cursor::<Dimensions<Option<&Locator>, ExcerptOffset>>(&());
        let locator = self.excerpt_locator_for_id(anchor.excerpt_id);

        cursor.seek(&Some(locator), Bias::Left);
        if cursor.item().is_none() {
            cursor.next();
        }

        let mut position = cursor.start().1;
        if let Some(excerpt) = cursor.item()
            && excerpt.id == anchor.excerpt_id
        {
            let excerpt_buffer_start = excerpt
                .buffer
                .offset_for_anchor(&excerpt.range.context.start);
            let excerpt_buffer_end = excerpt.buffer.offset_for_anchor(&excerpt.range.context.end);
            let buffer_position = cmp::min(
                excerpt_buffer_end,
                excerpt.buffer.offset_for_anchor(&anchor.text_anchor),
            );
            if buffer_position > excerpt_buffer_start {
                position.value += buffer_position - excerpt_buffer_start;
            }
        }
        position
    }

    pub fn latest_excerpt_id(&self, mut excerpt_id: ExcerptId) -> ExcerptId {
        while let Some(replacement) = self.replaced_excerpts.get(&excerpt_id) {
            excerpt_id = *replacement;
        }
        excerpt_id
    }

    pub fn summaries_for_anchors<'a, D, I>(&'a self, anchors: I) -> Vec<D>
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
        I: 'a + IntoIterator<Item = &'a Anchor>,
    {
        let mut anchors = anchors.into_iter().peekable();
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(&());
        let mut diff_transforms_cursor = self
            .diff_transforms
            .cursor::<Dimensions<ExcerptDimension<D>, OutputDimension<D>>>(&());
        diff_transforms_cursor.next();

        let mut summaries = Vec::new();
        while let Some(anchor) = anchors.peek() {
            let excerpt_id = self.latest_excerpt_id(anchor.excerpt_id);
            let excerpt_anchors = iter::from_fn(|| {
                let anchor = anchors.peek()?;
                if self.latest_excerpt_id(anchor.excerpt_id) == excerpt_id {
                    Some(anchors.next().unwrap())
                } else {
                    None
                }
            });

            let locator = self.excerpt_locator_for_id(excerpt_id);
            cursor.seek_forward(locator, Bias::Left);
            if cursor.item().is_none() {
                cursor.next();
            }

            let excerpt_start_position = D::from_text_summary(&cursor.start().text);
            if let Some(excerpt) = cursor.item() {
                if excerpt.id != excerpt_id {
                    let position = self.resolve_summary_for_anchor(
                        &Anchor::min(),
                        excerpt_start_position,
                        &mut diff_transforms_cursor,
                    );
                    summaries.extend(excerpt_anchors.map(|_| position));
                    continue;
                }
                let excerpt_buffer_start =
                    excerpt.range.context.start.summary::<D>(&excerpt.buffer);
                let excerpt_buffer_end = excerpt.range.context.end.summary::<D>(&excerpt.buffer);
                for (buffer_summary, anchor) in excerpt
                    .buffer
                    .summaries_for_anchors_with_payload::<D, _, _>(
                        excerpt_anchors.map(|a| (&a.text_anchor, a)),
                    )
                {
                    let summary = cmp::min(excerpt_buffer_end, buffer_summary);
                    let mut position = excerpt_start_position;
                    if summary > excerpt_buffer_start {
                        position.add_assign(&(summary - excerpt_buffer_start));
                    }

                    if position > diff_transforms_cursor.start().0.0 {
                        diff_transforms_cursor
                            .seek_forward(&ExcerptDimension(position), Bias::Left);
                    }

                    summaries.push(self.resolve_summary_for_anchor(
                        anchor,
                        position,
                        &mut diff_transforms_cursor,
                    ));
                }
            } else {
                diff_transforms_cursor
                    .seek_forward(&ExcerptDimension(excerpt_start_position), Bias::Left);
                let position = self.resolve_summary_for_anchor(
                    &Anchor::max(),
                    excerpt_start_position,
                    &mut diff_transforms_cursor,
                );
                summaries.extend(excerpt_anchors.map(|_| position));
            }
        }

        summaries
    }

    pub fn dimensions_from_points<'a, D>(
        &'a self,
        points: impl 'a + IntoIterator<Item = Point>,
    ) -> impl 'a + Iterator<Item = D>
    where
        D: TextDimension + Sub<D, Output = D>,
    {
        let mut cursor = self.cursor::<DimensionPair<Point, D>>();
        cursor.seek(&DimensionPair {
            key: Point::default(),
            value: None,
        });
        let mut points = points.into_iter();
        iter::from_fn(move || {
            let point = points.next()?;

            cursor.seek_forward(&DimensionPair {
                key: point,
                value: None,
            });

            if let Some(region) = cursor.region() {
                let overshoot = point - region.range.start.key;
                let buffer_point = region.buffer_range.start.key + overshoot;
                let mut position = region.range.start.value.unwrap();
                position.add_assign(
                    &region
                        .buffer
                        .text_summary_for_range(region.buffer_range.start.key..buffer_point),
                );
                if point == region.range.end.key && region.has_trailing_newline {
                    position.add_assign(&D::from_text_summary(&TextSummary::newline()));
                }
                Some(position)
            } else {
                Some(D::from_text_summary(&self.text_summary()))
            }
        })
    }

    pub fn refresh_anchors<'a, I>(&'a self, anchors: I) -> Vec<(usize, Anchor, bool)>
    where
        I: 'a + IntoIterator<Item = &'a Anchor>,
    {
        let mut anchors = anchors.into_iter().enumerate().peekable();
        let mut cursor = self.excerpts.cursor::<Option<&Locator>>(&());
        cursor.next();

        let mut result = Vec::new();

        while let Some((_, anchor)) = anchors.peek() {
            let old_excerpt_id = anchor.excerpt_id;

            // Find the location where this anchor's excerpt should be.
            let old_locator = self.excerpt_locator_for_id(old_excerpt_id);
            cursor.seek_forward(&Some(old_locator), Bias::Left);

            if cursor.item().is_none() {
                cursor.next();
            }

            let next_excerpt = cursor.item();
            let prev_excerpt = cursor.prev_item();

            // Process all of the anchors for this excerpt.
            while let Some((_, anchor)) = anchors.peek() {
                if anchor.excerpt_id != old_excerpt_id {
                    break;
                }
                let (anchor_ix, anchor) = anchors.next().unwrap();
                let mut anchor = *anchor;

                // Leave min and max anchors unchanged if invalid or
                // if the old excerpt still exists at this location
                let mut kept_position = next_excerpt
                    .is_some_and(|e| e.id == old_excerpt_id && e.contains(&anchor))
                    || old_excerpt_id == ExcerptId::max()
                    || old_excerpt_id == ExcerptId::min();

                // If the old excerpt no longer exists at this location, then attempt to
                // find an equivalent position for this anchor in an adjacent excerpt.
                if !kept_position {
                    for excerpt in [next_excerpt, prev_excerpt].iter().filter_map(|e| *e) {
                        if excerpt.contains(&anchor) {
                            anchor.excerpt_id = excerpt.id;
                            kept_position = true;
                            break;
                        }
                    }
                }

                // If there's no adjacent excerpt that contains the anchor's position,
                // then report that the anchor has lost its position.
                if !kept_position {
                    anchor = if let Some(excerpt) = next_excerpt {
                        let mut text_anchor = excerpt
                            .range
                            .context
                            .start
                            .bias(anchor.text_anchor.bias, &excerpt.buffer);
                        if text_anchor
                            .cmp(&excerpt.range.context.end, &excerpt.buffer)
                            .is_gt()
                        {
                            text_anchor = excerpt.range.context.end;
                        }
                        Anchor {
                            buffer_id: Some(excerpt.buffer_id),
                            excerpt_id: excerpt.id,
                            text_anchor,
                            diff_base_anchor: None,
                        }
                    } else if let Some(excerpt) = prev_excerpt {
                        let mut text_anchor = excerpt
                            .range
                            .context
                            .end
                            .bias(anchor.text_anchor.bias, &excerpt.buffer);
                        if text_anchor
                            .cmp(&excerpt.range.context.start, &excerpt.buffer)
                            .is_lt()
                        {
                            text_anchor = excerpt.range.context.start;
                        }
                        Anchor {
                            buffer_id: Some(excerpt.buffer_id),
                            excerpt_id: excerpt.id,
                            text_anchor,
                            diff_base_anchor: None,
                        }
                    } else if anchor.text_anchor.bias == Bias::Left {
                        Anchor::min()
                    } else {
                        Anchor::max()
                    };
                }

                result.push((anchor_ix, anchor, kept_position));
            }
        }
        result.sort_unstable_by(|a, b| a.1.cmp(&b.1, self));
        result
    }

    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, mut bias: Bias) -> Anchor {
        let offset = position.to_offset(self);

        // Find the given position in the diff transforms. Determine the corresponding
        // offset in the excerpts, and whether the position is within a deleted hunk.
        let mut diff_transforms = self
            .diff_transforms
            .cursor::<Dimensions<usize, ExcerptOffset>>(&());
        diff_transforms.seek(&offset, Bias::Right);

        if offset == diff_transforms.start().0
            && bias == Bias::Left
            && let Some(prev_item) = diff_transforms.prev_item()
            && let DiffTransform::DeletedHunk { .. } = prev_item
        {
            diff_transforms.prev();
        }
        let offset_in_transform = offset - diff_transforms.start().0;
        let mut excerpt_offset = diff_transforms.start().1;
        let mut diff_base_anchor = None;
        if let Some(DiffTransform::DeletedHunk {
            buffer_id,
            base_text_byte_range,
            has_trailing_newline,
            ..
        }) = diff_transforms.item()
        {
            let diff = self.diffs.get(buffer_id).expect("missing diff");
            if offset_in_transform > base_text_byte_range.len() {
                debug_assert!(*has_trailing_newline);
                bias = Bias::Right;
            } else {
                diff_base_anchor = Some(
                    diff.base_text()
                        .anchor_at(base_text_byte_range.start + offset_in_transform, bias),
                );
                bias = Bias::Left;
            }
        } else {
            excerpt_offset += ExcerptOffset::new(offset_in_transform);
        };

        let mut excerpts = self
            .excerpts
            .cursor::<Dimensions<ExcerptOffset, Option<ExcerptId>>>(&());
        excerpts.seek(&excerpt_offset, Bias::Right);
        if excerpts.item().is_none() && excerpt_offset == excerpts.start().0 && bias == Bias::Left {
            excerpts.prev();
        }
        if let Some(excerpt) = excerpts.item() {
            let mut overshoot = excerpt_offset.saturating_sub(excerpts.start().0).value;
            if excerpt.has_trailing_newline && excerpt_offset == excerpts.end().0 {
                overshoot -= 1;
                bias = Bias::Right;
            }

            let buffer_start = excerpt.range.context.start.to_offset(&excerpt.buffer);
            let text_anchor =
                excerpt.clip_anchor(excerpt.buffer.anchor_at(buffer_start + overshoot, bias));
            Anchor {
                buffer_id: Some(excerpt.buffer_id),
                excerpt_id: excerpt.id,
                text_anchor,
                diff_base_anchor,
            }
        } else {
            let mut anchor = if excerpt_offset.is_zero() && bias == Bias::Left {
                Anchor::min()
            } else {
                Anchor::max()
            };
            // TODO this is a hack, remove it
            if let Some((excerpt_id, _, _)) = self.as_singleton() {
                anchor.excerpt_id = *excerpt_id;
            }
            anchor
        }
    }

    /// Returns an anchor for the given excerpt and text anchor,
    /// returns None if the excerpt_id is no longer valid.
    pub fn anchor_in_excerpt(
        &self,
        excerpt_id: ExcerptId,
        text_anchor: text::Anchor,
    ) -> Option<Anchor> {
        let excerpt_id = self.latest_excerpt_id(excerpt_id);
        let locator = self.excerpt_locator_for_id(excerpt_id);
        let mut cursor = self.excerpts.cursor::<Option<&Locator>>(&());
        cursor.seek(locator, Bias::Left);
        if let Some(excerpt) = cursor.item()
            && excerpt.id == excerpt_id
        {
            let text_anchor = excerpt.clip_anchor(text_anchor);
            drop(cursor);
            return Some(Anchor {
                buffer_id: Some(excerpt.buffer_id),
                excerpt_id,
                text_anchor,
                diff_base_anchor: None,
            });
        }
        None
    }

    pub fn context_range_for_excerpt(&self, excerpt_id: ExcerptId) -> Option<Range<text::Anchor>> {
        Some(self.excerpt(excerpt_id)?.range.context.clone())
    }

    pub fn can_resolve(&self, anchor: &Anchor) -> bool {
        if anchor.excerpt_id == ExcerptId::min() || anchor.excerpt_id == ExcerptId::max() {
            true
        } else if let Some(excerpt) = self.excerpt(anchor.excerpt_id) {
            excerpt.buffer.can_resolve(&anchor.text_anchor)
        } else {
            false
        }
    }

    pub fn excerpts(
        &self,
    ) -> impl Iterator<Item = (ExcerptId, &BufferSnapshot, ExcerptRange<text::Anchor>)> {
        self.excerpts
            .iter()
            .map(|excerpt| (excerpt.id, &excerpt.buffer, excerpt.range.clone()))
    }

    fn cursor<D: TextDimension + Default>(&self) -> MultiBufferCursor<'_, D> {
        let excerpts = self.excerpts.cursor(&());
        let diff_transforms = self.diff_transforms.cursor(&());
        MultiBufferCursor {
            excerpts,
            diff_transforms,
            diffs: &self.diffs,
            cached_region: None,
        }
    }

    pub fn excerpt_before(&self, id: ExcerptId) -> Option<MultiBufferExcerpt<'_>> {
        let start_locator = self.excerpt_locator_for_id(id);
        let mut excerpts = self
            .excerpts
            .cursor::<Dimensions<Option<&Locator>, ExcerptDimension<usize>>>(&());
        excerpts.seek(&Some(start_locator), Bias::Left);
        excerpts.prev();

        let mut diff_transforms = self.diff_transforms.cursor::<DiffTransforms<usize>>(&());
        diff_transforms.seek(&excerpts.start().1, Bias::Left);
        if diff_transforms.end().excerpt_dimension < excerpts.start().1 {
            diff_transforms.next();
        }

        let excerpt = excerpts.item()?;
        Some(MultiBufferExcerpt {
            excerpt,
            offset: diff_transforms.start().output_dimension.0,
            buffer_offset: excerpt.range.context.start.to_offset(&excerpt.buffer),
            excerpt_offset: excerpts.start().1.clone(),
            diff_transforms,
        })
    }

    pub fn excerpt_boundaries_in_range<R, T>(
        &self,
        range: R,
    ) -> impl Iterator<Item = ExcerptBoundary> + '_
    where
        R: RangeBounds<T>,
        T: ToOffset,
    {
        let start_offset;
        let start = match range.start_bound() {
            Bound::Included(start) => {
                start_offset = start.to_offset(self);
                Bound::Included(start_offset)
            }
            Bound::Excluded(_) => {
                panic!("not supported")
            }
            Bound::Unbounded => {
                start_offset = 0;
                Bound::Unbounded
            }
        };
        let end = match range.end_bound() {
            Bound::Included(end) => Bound::Included(end.to_offset(self)),
            Bound::Excluded(end) => Bound::Excluded(end.to_offset(self)),
            Bound::Unbounded => Bound::Unbounded,
        };
        let bounds = (start, end);

        let mut cursor = self.cursor::<DimensionPair<usize, Point>>();
        cursor.seek(&DimensionPair {
            key: start_offset,
            value: None,
        });

        if cursor
            .region()
            .is_some_and(|region| bounds.contains(&region.range.start.key))
        {
            cursor.prev_excerpt();
        } else {
            cursor.seek_to_start_of_current_excerpt();
        }
        let mut prev_region = cursor.region();

        cursor.next_excerpt();

        iter::from_fn(move || {
            loop {
                if self.singleton {
                    return None;
                }

                let next_region = cursor.region()?;
                cursor.next_excerpt();
                if !bounds.contains(&next_region.range.start.key) {
                    prev_region = Some(next_region);
                    continue;
                }

                let next_region_start = next_region.range.start.value.unwrap();
                let next_region_end = if let Some(region) = cursor.region() {
                    region.range.start.value.unwrap()
                } else {
                    self.max_point()
                };

                let prev = prev_region.as_ref().map(|region| ExcerptInfo {
                    id: region.excerpt.id,
                    buffer: region.excerpt.buffer.clone(),
                    buffer_id: region.excerpt.buffer_id,
                    range: region.excerpt.range.clone(),
                    end_row: MultiBufferRow(next_region_start.row),
                });

                let next = ExcerptInfo {
                    id: next_region.excerpt.id,
                    buffer: next_region.excerpt.buffer.clone(),
                    buffer_id: next_region.excerpt.buffer_id,
                    range: next_region.excerpt.range.clone(),
                    end_row: if next_region.excerpt.has_trailing_newline {
                        MultiBufferRow(next_region_end.row - 1)
                    } else {
                        MultiBufferRow(next_region_end.row)
                    },
                };

                let row = MultiBufferRow(next_region_start.row);

                prev_region = Some(next_region);

                return Some(ExcerptBoundary { row, prev, next });
            }
        })
    }

    pub fn edit_count(&self) -> usize {
        self.edit_count
    }

    pub fn non_text_state_update_count(&self) -> usize {
        self.non_text_state_update_count
    }

    /// Returns the smallest enclosing bracket ranges containing the given range or
    /// None if no brackets contain range or the range is not contained in a single
    /// excerpt
    ///
    /// Can optionally pass a range_filter to filter the ranges of brackets to consider
    pub fn innermost_enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
        range_filter: Option<&dyn Fn(&BufferSnapshot, Range<usize>, Range<usize>) -> bool>,
    ) -> Option<(Range<usize>, Range<usize>)> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut excerpt = self.excerpt_containing(range.clone())?;
        let buffer = excerpt.buffer();
        let excerpt_buffer_range = excerpt.buffer_range();

        // Filter to ranges contained in the excerpt
        let range_filter = |open: Range<usize>, close: Range<usize>| -> bool {
            excerpt_buffer_range.contains(&open.start)
                && excerpt_buffer_range.contains(&close.end)
                && range_filter.is_none_or(|filter| filter(buffer, open, close))
        };

        let (open, close) = excerpt.buffer().innermost_enclosing_bracket_ranges(
            excerpt.map_range_to_buffer(range),
            Some(&range_filter),
        )?;

        Some((
            excerpt.map_range_from_buffer(open),
            excerpt.map_range_from_buffer(close),
        ))
    }

    /// Returns enclosing bracket ranges containing the given range or returns None if the range is
    /// not contained in a single excerpt
    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<impl Iterator<Item = (Range<usize>, Range<usize>)> + '_> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut excerpt = self.excerpt_containing(range.clone())?;

        Some(
            excerpt
                .buffer()
                .enclosing_bracket_ranges(excerpt.map_range_to_buffer(range))
                .filter_map(move |pair| {
                    if excerpt.contains_buffer_range(pair.open_range.start..pair.close_range.end) {
                        Some((
                            excerpt.map_range_from_buffer(pair.open_range),
                            excerpt.map_range_from_buffer(pair.close_range),
                        ))
                    } else {
                        None
                    }
                }),
        )
    }

    /// Returns enclosing bracket ranges containing the given range or returns None if the range is
    /// not contained in a single excerpt
    pub fn text_object_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
        options: TreeSitterOptions,
    ) -> impl Iterator<Item = (Range<usize>, TextObject)> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.excerpt_containing(range.clone())
            .map(|mut excerpt| {
                excerpt
                    .buffer()
                    .text_object_ranges(excerpt.map_range_to_buffer(range), options)
                    .filter_map(move |(range, text_object)| {
                        if excerpt.contains_buffer_range(range.clone()) {
                            Some((excerpt.map_range_from_buffer(range), text_object))
                        } else {
                            None
                        }
                    })
            })
            .into_iter()
            .flatten()
    }

    /// Returns bracket range pairs overlapping the given `range` or returns None if the `range` is
    /// not contained in a single excerpt
    pub fn bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<impl Iterator<Item = (Range<usize>, Range<usize>)> + '_> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut excerpt = self.excerpt_containing(range.clone())?;

        Some(
            excerpt
                .buffer()
                .bracket_ranges(excerpt.map_range_to_buffer(range))
                .filter_map(move |pair| {
                    let buffer_range = pair.open_range.start..pair.close_range.end;
                    if excerpt.contains_buffer_range(buffer_range) {
                        Some((
                            excerpt.map_range_from_buffer(pair.open_range),
                            excerpt.map_range_from_buffer(pair.close_range),
                        ))
                    } else {
                        None
                    }
                }),
        )
    }

    pub fn redacted_ranges<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        redaction_enabled: impl Fn(Option<&Arc<dyn File>>) -> bool + 'a,
    ) -> impl Iterator<Item = Range<usize>> + 'a {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.lift_buffer_metadata(range, move |buffer, range| {
            if redaction_enabled(buffer.file()) {
                Some(buffer.redacted_ranges(range).map(|range| (range, ())))
            } else {
                None
            }
        })
        .map(|(range, _, _)| range)
    }

    pub fn runnable_ranges(
        &self,
        range: Range<Anchor>,
    ) -> impl Iterator<Item = language::RunnableRange> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.lift_buffer_metadata(range, move |buffer, range| {
            Some(
                buffer
                    .runnable_ranges(range.clone())
                    .filter(move |runnable| {
                        runnable.run_range.start >= range.start
                            && runnable.run_range.end < range.end
                    })
                    .map(|runnable| (runnable.run_range.clone(), runnable)),
            )
        })
        .map(|(run_range, runnable, _)| language::RunnableRange {
            run_range,
            ..runnable
        })
    }

    pub fn line_indents(
        &self,
        start_row: MultiBufferRow,
        buffer_filter: impl Fn(&BufferSnapshot) -> bool,
    ) -> impl Iterator<Item = (MultiBufferRow, LineIndent, &BufferSnapshot)> {
        let max_point = self.max_point();
        let mut cursor = self.cursor::<Point>();
        cursor.seek(&Point::new(start_row.0, 0));
        iter::from_fn(move || {
            let mut region = cursor.region()?;
            while !buffer_filter(&region.excerpt.buffer) {
                cursor.next();
                region = cursor.region()?;
            }
            let overshoot = start_row.0.saturating_sub(region.range.start.row);
            let buffer_start_row =
                (region.buffer_range.start.row + overshoot).min(region.buffer_range.end.row);

            let buffer_end_row = if region.is_main_buffer
                && (region.has_trailing_newline || region.range.end == max_point)
            {
                region.buffer_range.end.row
            } else {
                region.buffer_range.end.row.saturating_sub(1)
            };

            let line_indents = region
                .buffer
                .line_indents_in_row_range(buffer_start_row..buffer_end_row);
            cursor.next();
            Some(line_indents.map(move |(buffer_row, indent)| {
                let row = region.range.start.row + (buffer_row - region.buffer_range.start.row);
                (MultiBufferRow(row), indent, &region.excerpt.buffer)
            }))
        })
        .flatten()
    }

    pub fn reversed_line_indents(
        &self,
        end_row: MultiBufferRow,
        buffer_filter: impl Fn(&BufferSnapshot) -> bool,
    ) -> impl Iterator<Item = (MultiBufferRow, LineIndent, &BufferSnapshot)> {
        let max_point = self.max_point();
        let mut cursor = self.cursor::<Point>();
        cursor.seek(&Point::new(end_row.0, 0));
        iter::from_fn(move || {
            let mut region = cursor.region()?;
            while !buffer_filter(&region.excerpt.buffer) {
                cursor.prev();
                region = cursor.region()?;
            }

            let buffer_start_row = region.buffer_range.start.row;
            let buffer_end_row = if region.is_main_buffer
                && (region.has_trailing_newline || region.range.end == max_point)
            {
                region.buffer_range.end.row + 1
            } else {
                region.buffer_range.end.row
            };

            let overshoot = end_row.0 - region.range.start.row;
            let buffer_end_row =
                (region.buffer_range.start.row + overshoot + 1).min(buffer_end_row);

            let line_indents = region
                .buffer
                .reversed_line_indents_in_row_range(buffer_start_row..buffer_end_row);
            cursor.prev();
            Some(line_indents.map(move |(buffer_row, indent)| {
                let row = region.range.start.row + (buffer_row - region.buffer_range.start.row);
                (MultiBufferRow(row), indent, &region.excerpt.buffer)
            }))
        })
        .flatten()
    }

    pub async fn enclosing_indent(
        &self,
        mut target_row: MultiBufferRow,
    ) -> Option<(Range<MultiBufferRow>, LineIndent)> {
        let max_row = MultiBufferRow(self.max_point().row);
        if target_row >= max_row {
            return None;
        }

        let mut target_indent = self.line_indent_for_row(target_row);

        // If the current row is at the start of an indented block, we want to return this
        // block as the enclosing indent.
        if !target_indent.is_line_empty() && target_row < max_row {
            let next_line_indent = self.line_indent_for_row(MultiBufferRow(target_row.0 + 1));
            if !next_line_indent.is_line_empty()
                && target_indent.raw_len() < next_line_indent.raw_len()
            {
                target_indent = next_line_indent;
                target_row.0 += 1;
            }
        }

        const SEARCH_ROW_LIMIT: u32 = 25000;
        const SEARCH_WHITESPACE_ROW_LIMIT: u32 = 2500;
        const YIELD_INTERVAL: u32 = 100;

        let mut accessed_row_counter = 0;

        // If there is a blank line at the current row, search for the next non indented lines
        if target_indent.is_line_empty() {
            let start = MultiBufferRow(target_row.0.saturating_sub(SEARCH_WHITESPACE_ROW_LIMIT));
            let end =
                MultiBufferRow((max_row.0 + 1).min(target_row.0 + SEARCH_WHITESPACE_ROW_LIMIT));

            let mut non_empty_line_above = None;
            for (row, indent, _) in self.reversed_line_indents(target_row, |_| true) {
                if row < start {
                    break;
                }
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
            for (row, indent, _) in self.line_indents(target_row, |_| true) {
                if row > end {
                    break;
                }
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
            target_row = row;
        }

        let start = MultiBufferRow(target_row.0.saturating_sub(SEARCH_ROW_LIMIT));
        let end = MultiBufferRow((max_row.0 + 1).min(target_row.0 + SEARCH_ROW_LIMIT));

        let mut start_indent = None;
        for (row, indent, _) in self.reversed_line_indents(target_row, |_| true) {
            if row < start {
                break;
            }
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
        for (row, indent, _) in self.line_indents(target_row, |_| true) {
            if row > end {
                break;
            }
            accessed_row_counter += 1;
            if accessed_row_counter == YIELD_INTERVAL {
                accessed_row_counter = 0;
                yield_now().await;
            }
            if !indent.is_line_empty() && indent.raw_len() < target_indent.raw_len() {
                end_indent = (MultiBufferRow(row.0.saturating_sub(1)), Some(indent));
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

    pub fn indent_guides_in_range<T: ToPoint>(
        &self,
        range: Range<T>,
        ignore_disabled_for_language: bool,
        cx: &App,
    ) -> impl Iterator<Item = IndentGuide> {
        let range = range.start.to_point(self)..range.end.to_point(self);
        let start_row = MultiBufferRow(range.start.row);
        let end_row = MultiBufferRow(range.end.row);

        let mut row_indents = self.line_indents(start_row, |buffer| {
            let settings =
                language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx);
            settings.indent_guides.enabled || ignore_disabled_for_language
        });

        let mut result = Vec::new();
        let mut indent_stack = SmallVec::<[IndentGuide; 8]>::new();

        let mut prev_settings = None;
        while let Some((first_row, mut line_indent, buffer)) = row_indents.next() {
            if first_row > end_row {
                break;
            }
            let current_depth = indent_stack.len() as u32;

            // Avoid retrieving the language settings repeatedly for every buffer row.
            if let Some((prev_buffer_id, _)) = &prev_settings
                && prev_buffer_id != &buffer.remote_id()
            {
                prev_settings.take();
            }
            let settings = &prev_settings
                .get_or_insert_with(|| {
                    (
                        buffer.remote_id(),
                        language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx),
                    )
                })
                .1;
            let tab_size = settings.tab_size.get();

            // When encountering empty, continue until found useful line indent
            // then add to the indent stack with the depth found
            let mut found_indent = false;
            let mut last_row = first_row;
            if line_indent.is_line_blank() {
                while !found_indent {
                    let Some((target_row, new_line_indent, _)) = row_indents.next() else {
                        break;
                    };
                    const TRAILING_ROW_SEARCH_LIMIT: u32 = 25;
                    if target_row > MultiBufferRow(end_row.0 + TRAILING_ROW_SEARCH_LIMIT) {
                        break;
                    }

                    if new_line_indent.is_line_blank() {
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
            } else {
                0
            };

            match depth.cmp(&current_depth) {
                cmp::Ordering::Less => {
                    for _ in 0..(current_depth - depth) {
                        let mut indent = indent_stack.pop().unwrap();
                        if last_row != first_row {
                            // In this case, we landed on an empty row, had to seek forward,
                            // and discovered that the indent we where on is ending.
                            // This means that the last display row must
                            // be on line that ends this indent range, so we
                            // should display the range up to the first non-empty line
                            indent.end_row = MultiBufferRow(first_row.0.saturating_sub(1));
                        }

                        result.push(indent)
                    }
                }
                cmp::Ordering::Greater => {
                    for next_depth in current_depth..depth {
                        indent_stack.push(IndentGuide {
                            buffer_id: buffer.remote_id(),
                            start_row: first_row,
                            end_row: last_row,
                            depth: next_depth,
                            tab_size,
                            settings: settings.indent_guides,
                        });
                    }
                }
                _ => {}
            }

            for indent in indent_stack.iter_mut() {
                indent.end_row = last_row;
            }
        }

        result.extend(indent_stack);
        result.into_iter()
    }

    pub fn trailing_excerpt_update_count(&self) -> usize {
        self.trailing_excerpt_update_count
    }

    pub fn file_at<T: ToOffset>(&self, point: T) -> Option<&Arc<dyn File>> {
        self.point_to_buffer_offset(point)
            .and_then(|(buffer, _)| buffer.file())
    }

    pub fn language_at<T: ToOffset>(&self, point: T) -> Option<&Arc<Language>> {
        self.point_to_buffer_offset(point)
            .and_then(|(buffer, offset)| buffer.language_at(offset))
    }

    fn language_settings<'a>(&'a self, cx: &'a App) -> Cow<'a, LanguageSettings> {
        self.excerpts
            .first()
            .map(|excerpt| &excerpt.buffer)
            .map(|buffer| {
                language_settings(
                    buffer.language().map(|language| language.name()),
                    buffer.file(),
                    cx,
                )
            })
            .unwrap_or_else(move || self.language_settings_at(0, cx))
    }

    pub fn language_settings_at<'a, T: ToOffset>(
        &'a self,
        point: T,
        cx: &'a App,
    ) -> Cow<'a, LanguageSettings> {
        let mut language = None;
        let mut file = None;
        if let Some((buffer, offset)) = self.point_to_buffer_offset(point) {
            language = buffer.language_at(offset);
            file = buffer.file();
        }
        language_settings(language.map(|l| l.name()), file, cx)
    }

    pub fn language_scope_at<T: ToOffset>(&self, point: T) -> Option<LanguageScope> {
        self.point_to_buffer_offset(point)
            .and_then(|(buffer, offset)| buffer.language_scope_at(offset))
    }

    pub fn char_classifier_at<T: ToOffset>(&self, point: T) -> CharClassifier {
        self.point_to_buffer_offset(point)
            .map(|(buffer, offset)| buffer.char_classifier_at(offset))
            .unwrap_or_default()
    }

    pub fn language_indent_size_at<T: ToOffset>(
        &self,
        position: T,
        cx: &App,
    ) -> Option<IndentSize> {
        let (buffer_snapshot, offset) = self.point_to_buffer_offset(position)?;
        Some(buffer_snapshot.language_indent_size_at(offset, cx))
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn has_deleted_file(&self) -> bool {
        self.has_deleted_file
    }

    pub fn has_conflict(&self) -> bool {
        self.has_conflict
    }

    pub fn has_diagnostics(&self) -> bool {
        self.excerpts
            .iter()
            .any(|excerpt| excerpt.buffer.has_diagnostics())
    }

    pub fn diagnostic_group(
        &self,
        buffer_id: BufferId,
        group_id: usize,
    ) -> impl Iterator<Item = DiagnosticEntry<Point>> + '_ {
        self.lift_buffer_metadata(Point::zero()..self.max_point(), move |buffer, range| {
            if buffer.remote_id() != buffer_id {
                return None;
            };
            Some(
                buffer
                    .diagnostics_in_range(range, false)
                    .filter(move |diagnostic| diagnostic.diagnostic.group_id == group_id)
                    .map(move |DiagnosticEntry { diagnostic, range }| (range, diagnostic)),
            )
        })
        .map(|(range, diagnostic, _)| DiagnosticEntry { diagnostic, range })
    }

    pub fn diagnostics_in_range<'a, T>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = DiagnosticEntry<T>> + 'a
    where
        T: 'a
            + text::ToOffset
            + text::FromAnchor
            + TextDimension
            + Ord
            + Sub<T, Output = T>
            + fmt::Debug,
    {
        self.lift_buffer_metadata(range, move |buffer, buffer_range| {
            Some(
                buffer
                    .diagnostics_in_range(buffer_range.start..buffer_range.end, false)
                    .map(|entry| (entry.range, entry.diagnostic)),
            )
        })
        .map(|(range, diagnostic, _)| DiagnosticEntry { diagnostic, range })
    }

    pub fn diagnostics_with_buffer_ids_in_range<'a, T>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = (BufferId, DiagnosticEntry<T>)> + 'a
    where
        T: 'a
            + text::ToOffset
            + text::FromAnchor
            + TextDimension
            + Ord
            + Sub<T, Output = T>
            + fmt::Debug,
    {
        self.lift_buffer_metadata(range, move |buffer, buffer_range| {
            Some(
                buffer
                    .diagnostics_in_range(buffer_range.start..buffer_range.end, false)
                    .map(|entry| (entry.range, entry.diagnostic)),
            )
        })
        .map(|(range, diagnostic, b)| (b.buffer_id, DiagnosticEntry { diagnostic, range }))
    }

    pub fn syntax_ancestor<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(tree_sitter::Node<'_>, MultiOrSingleBufferOffsetRange)> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut excerpt = self.excerpt_containing(range.clone())?;
        let node = excerpt
            .buffer()
            .syntax_ancestor(excerpt.map_range_to_buffer(range))?;
        let node_range = node.byte_range();
        let range = if excerpt.contains_buffer_range(node_range.clone()) {
            MultiOrSingleBufferOffsetRange::Multi(excerpt.map_range_from_buffer(node_range))
        } else {
            MultiOrSingleBufferOffsetRange::Single(node_range)
        };
        Some((node, range))
    }

    pub fn syntax_next_sibling<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<tree_sitter::Node<'_>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut excerpt = self.excerpt_containing(range.clone())?;
        excerpt
            .buffer()
            .syntax_next_sibling(excerpt.map_range_to_buffer(range))
    }

    pub fn syntax_prev_sibling<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<tree_sitter::Node<'_>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut excerpt = self.excerpt_containing(range.clone())?;
        excerpt
            .buffer()
            .syntax_prev_sibling(excerpt.map_range_to_buffer(range))
    }

    pub fn outline(&self, theme: Option<&SyntaxTheme>) -> Option<Outline<Anchor>> {
        let (excerpt_id, _, buffer) = self.as_singleton()?;
        let outline = buffer.outline(theme);
        Some(Outline::new(
            outline
                .items
                .into_iter()
                .flat_map(|item| {
                    Some(OutlineItem {
                        depth: item.depth,
                        range: self.anchor_in_excerpt(*excerpt_id, item.range.start)?
                            ..self.anchor_in_excerpt(*excerpt_id, item.range.end)?,
                        text: item.text,
                        highlight_ranges: item.highlight_ranges,
                        name_ranges: item.name_ranges,
                        body_range: item.body_range.and_then(|body_range| {
                            Some(
                                self.anchor_in_excerpt(*excerpt_id, body_range.start)?
                                    ..self.anchor_in_excerpt(*excerpt_id, body_range.end)?,
                            )
                        }),
                        annotation_range: item.annotation_range.and_then(|annotation_range| {
                            Some(
                                self.anchor_in_excerpt(*excerpt_id, annotation_range.start)?
                                    ..self.anchor_in_excerpt(*excerpt_id, annotation_range.end)?,
                            )
                        }),
                    })
                })
                .collect(),
        ))
    }

    pub fn symbols_containing<T: ToOffset>(
        &self,
        offset: T,
        theme: Option<&SyntaxTheme>,
    ) -> Option<(BufferId, Vec<OutlineItem<Anchor>>)> {
        let anchor = self.anchor_before(offset);
        let excerpt_id = anchor.excerpt_id;
        let excerpt = self.excerpt(excerpt_id)?;
        Some((
            excerpt.buffer_id,
            excerpt
                .buffer
                .symbols_containing(anchor.text_anchor, theme)
                .into_iter()
                .flat_map(|item| {
                    Some(OutlineItem {
                        depth: item.depth,
                        range: self.anchor_in_excerpt(excerpt_id, item.range.start)?
                            ..self.anchor_in_excerpt(excerpt_id, item.range.end)?,
                        text: item.text,
                        highlight_ranges: item.highlight_ranges,
                        name_ranges: item.name_ranges,
                        body_range: item.body_range.and_then(|body_range| {
                            Some(
                                self.anchor_in_excerpt(excerpt_id, body_range.start)?
                                    ..self.anchor_in_excerpt(excerpt_id, body_range.end)?,
                            )
                        }),
                        annotation_range: item.annotation_range.and_then(|body_range| {
                            Some(
                                self.anchor_in_excerpt(excerpt_id, body_range.start)?
                                    ..self.anchor_in_excerpt(excerpt_id, body_range.end)?,
                            )
                        }),
                    })
                })
                .collect(),
        ))
    }

    fn excerpt_locator_for_id(&self, id: ExcerptId) -> &Locator {
        if id == ExcerptId::min() {
            Locator::min_ref()
        } else if id == ExcerptId::max() {
            Locator::max_ref()
        } else {
            let mut cursor = self.excerpt_ids.cursor::<ExcerptId>(&());
            cursor.seek(&id, Bias::Left);
            if let Some(entry) = cursor.item()
                && entry.id == id
            {
                return &entry.locator;
            }
            panic!("invalid excerpt id {id:?}")
        }
    }

    /// Returns the locators referenced by the given excerpt IDs, sorted by locator.
    fn excerpt_locators_for_ids(
        &self,
        ids: impl IntoIterator<Item = ExcerptId>,
    ) -> SmallVec<[Locator; 1]> {
        let mut sorted_ids = ids.into_iter().collect::<SmallVec<[_; 1]>>();
        sorted_ids.sort_unstable();
        let mut locators = SmallVec::new();

        while sorted_ids.last() == Some(&ExcerptId::max()) {
            sorted_ids.pop();
            if let Some(mapping) = self.excerpt_ids.last() {
                locators.push(mapping.locator.clone());
            }
        }

        let mut sorted_ids = sorted_ids.into_iter().dedup().peekable();
        if sorted_ids.peek() == Some(&ExcerptId::min()) {
            sorted_ids.next();
            if let Some(mapping) = self.excerpt_ids.first() {
                locators.push(mapping.locator.clone());
            }
        }

        let mut cursor = self.excerpt_ids.cursor::<ExcerptId>(&());
        for id in sorted_ids {
            if cursor.seek_forward(&id, Bias::Left) {
                locators.push(cursor.item().unwrap().locator.clone());
            } else {
                panic!("invalid excerpt id {:?}", id);
            }
        }

        locators.sort_unstable();
        locators
    }

    pub fn buffer_id_for_excerpt(&self, excerpt_id: ExcerptId) -> Option<BufferId> {
        Some(self.excerpt(excerpt_id)?.buffer_id)
    }

    pub fn buffer_for_excerpt(&self, excerpt_id: ExcerptId) -> Option<&BufferSnapshot> {
        Some(&self.excerpt(excerpt_id)?.buffer)
    }

    pub fn range_for_excerpt(&self, excerpt_id: ExcerptId) -> Option<Range<Point>> {
        let mut cursor = self
            .excerpts
            .cursor::<Dimensions<Option<&Locator>, ExcerptDimension<Point>>>(&());
        let locator = self.excerpt_locator_for_id(excerpt_id);
        if cursor.seek(&Some(locator), Bias::Left) {
            let start = cursor.start().1.clone();
            let end = cursor.end().1;
            let mut diff_transforms = self
                .diff_transforms
                .cursor::<Dimensions<ExcerptDimension<Point>, OutputDimension<Point>>>(&());
            diff_transforms.seek(&start, Bias::Left);
            let overshoot = start.0 - diff_transforms.start().0.0;
            let start = diff_transforms.start().1.0 + overshoot;
            diff_transforms.seek(&end, Bias::Right);
            let overshoot = end.0 - diff_transforms.start().0.0;
            let end = diff_transforms.start().1.0 + overshoot;
            Some(start..end)
        } else {
            None
        }
    }

    pub fn buffer_range_for_excerpt(&self, excerpt_id: ExcerptId) -> Option<Range<text::Anchor>> {
        let mut cursor = self.excerpts.cursor::<Option<&Locator>>(&());
        let locator = self.excerpt_locator_for_id(excerpt_id);
        if cursor.seek(&Some(locator), Bias::Left)
            && let Some(excerpt) = cursor.item()
        {
            return Some(excerpt.range.context.clone());
        }
        None
    }

    fn excerpt(&self, excerpt_id: ExcerptId) -> Option<&Excerpt> {
        let mut cursor = self.excerpts.cursor::<Option<&Locator>>(&());
        let locator = self.excerpt_locator_for_id(excerpt_id);
        cursor.seek(&Some(locator), Bias::Left);
        if let Some(excerpt) = cursor.item()
            && excerpt.id == excerpt_id
        {
            return Some(excerpt);
        }
        None
    }

    /// Returns the excerpt containing range and its offset start within the multibuffer or none if `range` spans multiple excerpts
    pub fn excerpt_containing<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<MultiBufferExcerpt<'_>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.cursor::<usize>();
        cursor.seek(&range.start);

        let start_excerpt = cursor.excerpt()?;
        if range.end != range.start {
            cursor.seek_forward(&range.end);
            if cursor.excerpt()?.id != start_excerpt.id {
                return None;
            }
        }

        cursor.seek_to_start_of_current_excerpt();
        let region = cursor.region()?;
        let offset = region.range.start;
        let buffer_offset = start_excerpt.buffer_start_offset();
        let excerpt_offset = cursor.excerpts.start().clone();
        Some(MultiBufferExcerpt {
            diff_transforms: cursor.diff_transforms,
            excerpt: start_excerpt,
            offset,
            buffer_offset,
            excerpt_offset,
        })
    }

    pub fn buffer_id_for_anchor(&self, anchor: Anchor) -> Option<BufferId> {
        if let Some(id) = anchor.buffer_id {
            return Some(id);
        }
        let excerpt = self.excerpt_containing(anchor..anchor)?;
        Some(excerpt.buffer_id())
    }

    pub fn selections_in_range<'a>(
        &'a self,
        range: &'a Range<Anchor>,
        include_local: bool,
    ) -> impl 'a + Iterator<Item = (ReplicaId, bool, CursorShape, Selection<Anchor>)> {
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(&());
        let start_locator = self.excerpt_locator_for_id(range.start.excerpt_id);
        let end_locator = self.excerpt_locator_for_id(range.end.excerpt_id);
        cursor.seek(start_locator, Bias::Left);
        cursor
            .take_while(move |excerpt| excerpt.locator <= *end_locator)
            .flat_map(move |excerpt| {
                let mut query_range = excerpt.range.context.start..excerpt.range.context.end;
                if excerpt.id == range.start.excerpt_id {
                    query_range.start = range.start.text_anchor;
                }
                if excerpt.id == range.end.excerpt_id {
                    query_range.end = range.end.text_anchor;
                }

                excerpt
                    .buffer
                    .selections_in_range(query_range, include_local)
                    .flat_map(move |(replica_id, line_mode, cursor_shape, selections)| {
                        selections.map(move |selection| {
                            let mut start = Anchor {
                                buffer_id: Some(excerpt.buffer_id),
                                excerpt_id: excerpt.id,
                                text_anchor: selection.start,
                                diff_base_anchor: None,
                            };
                            let mut end = Anchor {
                                buffer_id: Some(excerpt.buffer_id),
                                excerpt_id: excerpt.id,
                                text_anchor: selection.end,
                                diff_base_anchor: None,
                            };
                            if range.start.cmp(&start, self).is_gt() {
                                start = range.start;
                            }
                            if range.end.cmp(&end, self).is_lt() {
                                end = range.end;
                            }

                            (
                                replica_id,
                                line_mode,
                                cursor_shape,
                                Selection {
                                    id: selection.id,
                                    start,
                                    end,
                                    reversed: selection.reversed,
                                    goal: selection.goal,
                                },
                            )
                        })
                    })
            })
    }

    pub fn show_headers(&self) -> bool {
        self.show_headers
    }

    pub fn diff_for_buffer_id(&self, buffer_id: BufferId) -> Option<&BufferDiffSnapshot> {
        self.diffs.get(&buffer_id)
    }

    /// Visually annotates a position or range with the `Debug` representation of a value. The
    /// callsite of this function is used as a key - previous annotations will be removed.
    #[cfg(debug_assertions)]
    #[track_caller]
    pub fn debug<V, R>(&self, ranges: &R, value: V)
    where
        R: debug::ToMultiBufferDebugRanges,
        V: std::fmt::Debug,
    {
        self.debug_with_key(std::panic::Location::caller(), ranges, value);
    }

    /// Visually annotates a position or range with the `Debug` representation of a value. Previous
    /// debug annotations with the same key will be removed. The key is also used to determine the
    /// annotation's color.
    #[cfg(debug_assertions)]
    #[track_caller]
    pub fn debug_with_key<K, R, V>(&self, key: &K, ranges: &R, value: V)
    where
        K: std::hash::Hash + 'static,
        R: debug::ToMultiBufferDebugRanges,
        V: std::fmt::Debug,
    {
        let text_ranges = ranges
            .to_multi_buffer_debug_ranges(self)
            .into_iter()
            .flat_map(|range| {
                self.range_to_buffer_ranges(range).into_iter().map(
                    |(buffer, range, _excerpt_id)| {
                        buffer.anchor_after(range.start)..buffer.anchor_before(range.end)
                    },
                )
            })
            .collect();
        text::debug::GlobalDebugRanges::with_locked(|debug_ranges| {
            debug_ranges.insert(key, text_ranges, format!("{value:?}").into())
        });
    }
}

#[cfg(any(test, feature = "test-support"))]
impl MultiBufferSnapshot {
    pub fn random_byte_range(&self, start_offset: usize, rng: &mut impl rand::Rng) -> Range<usize> {
        let end = self.clip_offset(rng.random_range(start_offset..=self.len()), Bias::Right);
        let start = self.clip_offset(rng.random_range(start_offset..=end), Bias::Right);
        start..end
    }

    #[cfg(any(test, feature = "test-support"))]
    fn check_invariants(&self) {
        let excerpts = self.excerpts.items(&());
        let excerpt_ids = self.excerpt_ids.items(&());

        for (ix, excerpt) in excerpts.iter().enumerate() {
            if ix == 0 {
                if excerpt.locator <= Locator::min() {
                    panic!("invalid first excerpt locator {:?}", excerpt.locator);
                }
            } else if excerpt.locator <= excerpts[ix - 1].locator {
                panic!("excerpts are out-of-order: {:?}", excerpts);
            }
        }

        for (ix, entry) in excerpt_ids.iter().enumerate() {
            if ix == 0 {
                if entry.id.cmp(&ExcerptId::min(), self).is_le() {
                    panic!("invalid first excerpt id {:?}", entry.id);
                }
            } else if entry.id <= excerpt_ids[ix - 1].id {
                panic!("excerpt ids are out-of-order: {:?}", excerpt_ids);
            }
        }

        if self.diff_transforms.summary().input != self.excerpts.summary().text {
            panic!(
                "incorrect input summary. expected {:?}, got {:?}. transforms: {:+?}",
                self.excerpts.summary().text.len,
                self.diff_transforms.summary().input,
                self.diff_transforms.items(&()),
            );
        }

        let mut prev_transform: Option<&DiffTransform> = None;
        for item in self.diff_transforms.iter() {
            if let DiffTransform::BufferContent {
                summary,
                inserted_hunk_info,
            } = item
            {
                if let Some(DiffTransform::BufferContent {
                    inserted_hunk_info: prev_inserted_hunk_info,
                    ..
                }) = prev_transform
                    && *inserted_hunk_info == *prev_inserted_hunk_info
                {
                    panic!(
                        "multiple adjacent buffer content transforms with is_inserted_hunk = {inserted_hunk_info:?}. transforms: {:+?}",
                        self.diff_transforms.items(&())
                    );
                }
                if summary.len == 0 && !self.is_empty() {
                    panic!("empty buffer content transform");
                }
            }
            prev_transform = Some(item);
        }
    }
}

impl<'a, D> MultiBufferCursor<'a, D>
where
    D: TextDimension + Ord + Sub<D, Output = D>,
{
    fn seek(&mut self, position: &D) {
        self.cached_region.take();
        self.diff_transforms
            .seek(&OutputDimension(*position), Bias::Right);
        if self.diff_transforms.item().is_none()
            && *position == self.diff_transforms.start().output_dimension.0
        {
            self.diff_transforms.prev();
        }

        let mut excerpt_position = self.diff_transforms.start().excerpt_dimension.0;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            let overshoot = *position - self.diff_transforms.start().output_dimension.0;
            excerpt_position.add_assign(&overshoot);
        }

        self.excerpts
            .seek(&ExcerptDimension(excerpt_position), Bias::Right);
        if self.excerpts.item().is_none() && excerpt_position == self.excerpts.start().0 {
            self.excerpts.prev();
        }
    }

    fn seek_forward(&mut self, position: &D) {
        self.cached_region.take();
        self.diff_transforms
            .seek_forward(&OutputDimension(*position), Bias::Right);
        if self.diff_transforms.item().is_none()
            && *position == self.diff_transforms.start().output_dimension.0
        {
            self.diff_transforms.prev();
        }

        let overshoot = *position - self.diff_transforms.start().output_dimension.0;
        let mut excerpt_position = self.diff_transforms.start().excerpt_dimension.0;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            excerpt_position.add_assign(&overshoot);
        }

        self.excerpts
            .seek_forward(&ExcerptDimension(excerpt_position), Bias::Right);
        if self.excerpts.item().is_none() && excerpt_position == self.excerpts.start().0 {
            self.excerpts.prev();
        }
    }

    fn next_excerpt(&mut self) {
        self.excerpts.next();
        self.seek_to_start_of_current_excerpt();
    }

    fn prev_excerpt(&mut self) {
        self.excerpts.prev();
        self.seek_to_start_of_current_excerpt();
    }

    fn seek_to_start_of_current_excerpt(&mut self) {
        self.cached_region.take();
        self.diff_transforms.seek(self.excerpts.start(), Bias::Left);
        if self.diff_transforms.end().excerpt_dimension == *self.excerpts.start()
            && self.diff_transforms.start().excerpt_dimension < *self.excerpts.start()
            && self.diff_transforms.next_item().is_some()
        {
            self.diff_transforms.next();
        }
    }

    fn next(&mut self) {
        self.cached_region.take();
        match self
            .diff_transforms
            .end()
            .excerpt_dimension
            .cmp(&self.excerpts.end())
        {
            cmp::Ordering::Less => self.diff_transforms.next(),
            cmp::Ordering::Greater => self.excerpts.next(),
            cmp::Ordering::Equal => {
                self.diff_transforms.next();
                if self.diff_transforms.end().excerpt_dimension > self.excerpts.end()
                    || self.diff_transforms.item().is_none()
                {
                    self.excerpts.next();
                } else if let Some(DiffTransform::DeletedHunk { hunk_info, .. }) =
                    self.diff_transforms.item()
                    && self
                        .excerpts
                        .item()
                        .is_some_and(|excerpt| excerpt.id != hunk_info.excerpt_id)
                {
                    self.excerpts.next();
                }
            }
        }
    }

    fn prev(&mut self) {
        self.cached_region.take();
        match self
            .diff_transforms
            .start()
            .excerpt_dimension
            .cmp(self.excerpts.start())
        {
            cmp::Ordering::Less => self.excerpts.prev(),
            cmp::Ordering::Greater => self.diff_transforms.prev(),
            cmp::Ordering::Equal => {
                self.diff_transforms.prev();
                if self.diff_transforms.start().excerpt_dimension < *self.excerpts.start()
                    || self.diff_transforms.item().is_none()
                {
                    self.excerpts.prev();
                }
            }
        }
    }

    fn region(&mut self) -> Option<MultiBufferRegion<'a, D>> {
        if self.cached_region.is_none() {
            self.cached_region = self.build_region();
        }
        self.cached_region.clone()
    }

    fn is_at_start_of_excerpt(&mut self) -> bool {
        if self.diff_transforms.start().excerpt_dimension > *self.excerpts.start() {
            return false;
        } else if self.diff_transforms.start().excerpt_dimension < *self.excerpts.start() {
            return true;
        }

        self.diff_transforms.prev();
        let prev_transform = self.diff_transforms.item();
        self.diff_transforms.next();

        prev_transform.is_none_or(|next_transform| {
            matches!(next_transform, DiffTransform::BufferContent { .. })
        })
    }

    fn is_at_end_of_excerpt(&mut self) -> bool {
        if self.diff_transforms.end().excerpt_dimension < self.excerpts.end() {
            return false;
        } else if self.diff_transforms.end().excerpt_dimension > self.excerpts.end()
            || self.diff_transforms.item().is_none()
        {
            return true;
        }

        let next_transform = self.diff_transforms.next_item();
        next_transform.is_none_or(|next_transform| match next_transform {
            DiffTransform::BufferContent { .. } => true,
            DiffTransform::DeletedHunk { hunk_info, .. } => self
                .excerpts
                .item()
                .is_some_and(|excerpt| excerpt.id != hunk_info.excerpt_id),
        })
    }

    fn main_buffer_position(&self) -> Option<D> {
        let excerpt = self.excerpts.item()?;
        let buffer = &excerpt.buffer;
        let buffer_context_start = excerpt.range.context.start.summary::<D>(buffer);
        let mut buffer_start = buffer_context_start;
        let overshoot = self.diff_transforms.end().excerpt_dimension.0 - self.excerpts.start().0;
        buffer_start.add_assign(&overshoot);
        Some(buffer_start)
    }

    fn build_region(&self) -> Option<MultiBufferRegion<'a, D>> {
        let excerpt = self.excerpts.item()?;
        match self.diff_transforms.item()? {
            DiffTransform::DeletedHunk {
                buffer_id,
                base_text_byte_range,
                has_trailing_newline,
                hunk_info,
                ..
            } => {
                let diff = self.diffs.get(buffer_id)?;
                let buffer = diff.base_text();
                let mut rope_cursor = buffer.as_rope().cursor(0);
                let buffer_start = rope_cursor.summary::<D>(base_text_byte_range.start);
                let buffer_range_len = rope_cursor.summary::<D>(base_text_byte_range.end);
                let mut buffer_end = buffer_start;
                buffer_end.add_assign(&buffer_range_len);
                let start = self.diff_transforms.start().output_dimension.0;
                let end = self.diff_transforms.end().output_dimension.0;
                Some(MultiBufferRegion {
                    buffer,
                    excerpt,
                    has_trailing_newline: *has_trailing_newline,
                    is_main_buffer: false,
                    diff_hunk_status: Some(DiffHunkStatus::deleted(
                        hunk_info.hunk_secondary_status,
                    )),
                    buffer_range: buffer_start..buffer_end,
                    range: start..end,
                })
            }
            DiffTransform::BufferContent {
                inserted_hunk_info, ..
            } => {
                let buffer = &excerpt.buffer;
                let buffer_context_start = excerpt.range.context.start.summary::<D>(buffer);

                let mut start = self.diff_transforms.start().output_dimension.0;
                let mut buffer_start = buffer_context_start;
                if self.diff_transforms.start().excerpt_dimension < *self.excerpts.start() {
                    let overshoot =
                        self.excerpts.start().0 - self.diff_transforms.start().excerpt_dimension.0;
                    start.add_assign(&overshoot);
                } else {
                    let overshoot =
                        self.diff_transforms.start().excerpt_dimension.0 - self.excerpts.start().0;
                    buffer_start.add_assign(&overshoot);
                }

                let mut end;
                let mut buffer_end;
                let has_trailing_newline;
                if self.diff_transforms.end().excerpt_dimension.0 < self.excerpts.end().0 {
                    let overshoot =
                        self.diff_transforms.end().excerpt_dimension.0 - self.excerpts.start().0;
                    end = self.diff_transforms.end().output_dimension.0;
                    buffer_end = buffer_context_start;
                    buffer_end.add_assign(&overshoot);
                    has_trailing_newline = false;
                } else {
                    let overshoot =
                        self.excerpts.end().0 - self.diff_transforms.start().excerpt_dimension.0;
                    end = self.diff_transforms.start().output_dimension.0;
                    end.add_assign(&overshoot);
                    buffer_end = excerpt.range.context.end.summary::<D>(buffer);
                    has_trailing_newline = excerpt.has_trailing_newline;
                };

                Some(MultiBufferRegion {
                    buffer,
                    excerpt,
                    has_trailing_newline,
                    is_main_buffer: true,
                    diff_hunk_status: inserted_hunk_info
                        .map(|info| DiffHunkStatus::added(info.hunk_secondary_status)),
                    buffer_range: buffer_start..buffer_end,
                    range: start..end,
                })
            }
        }
    }

    fn excerpt(&self) -> Option<&'a Excerpt> {
        self.excerpts.item()
    }
}

impl History {
    fn start_transaction(&mut self, now: Instant) -> Option<TransactionId> {
        self.transaction_depth += 1;
        if self.transaction_depth == 1 {
            let id = self.next_transaction_id.tick();
            self.undo_stack.push(Transaction {
                id,
                buffer_transactions: Default::default(),
                first_edit_at: now,
                last_edit_at: now,
                suppress_grouping: false,
            });
            Some(id)
        } else {
            None
        }
    }

    fn end_transaction(
        &mut self,
        now: Instant,
        buffer_transactions: HashMap<BufferId, TransactionId>,
    ) -> bool {
        assert_ne!(self.transaction_depth, 0);
        self.transaction_depth -= 1;
        if self.transaction_depth == 0 {
            if buffer_transactions.is_empty() {
                self.undo_stack.pop();
                false
            } else {
                self.redo_stack.clear();
                let transaction = self.undo_stack.last_mut().unwrap();
                transaction.last_edit_at = now;
                for (buffer_id, transaction_id) in buffer_transactions {
                    transaction
                        .buffer_transactions
                        .entry(buffer_id)
                        .or_insert(transaction_id);
                }
                true
            }
        } else {
            false
        }
    }

    fn push_transaction<'a, T>(
        &mut self,
        buffer_transactions: T,
        now: Instant,
        cx: &Context<MultiBuffer>,
    ) where
        T: IntoIterator<Item = (&'a Entity<Buffer>, &'a language::Transaction)>,
    {
        assert_eq!(self.transaction_depth, 0);
        let transaction = Transaction {
            id: self.next_transaction_id.tick(),
            buffer_transactions: buffer_transactions
                .into_iter()
                .map(|(buffer, transaction)| (buffer.read(cx).remote_id(), transaction.id))
                .collect(),
            first_edit_at: now,
            last_edit_at: now,
            suppress_grouping: false,
        };
        if !transaction.buffer_transactions.is_empty() {
            self.undo_stack.push(transaction);
            self.redo_stack.clear();
        }
    }

    fn finalize_last_transaction(&mut self) {
        if let Some(transaction) = self.undo_stack.last_mut() {
            transaction.suppress_grouping = true;
        }
    }

    fn forget(&mut self, transaction_id: TransactionId) -> Option<Transaction> {
        if let Some(ix) = self
            .undo_stack
            .iter()
            .rposition(|transaction| transaction.id == transaction_id)
        {
            Some(self.undo_stack.remove(ix))
        } else if let Some(ix) = self
            .redo_stack
            .iter()
            .rposition(|transaction| transaction.id == transaction_id)
        {
            Some(self.redo_stack.remove(ix))
        } else {
            None
        }
    }

    fn transaction(&self, transaction_id: TransactionId) -> Option<&Transaction> {
        self.undo_stack
            .iter()
            .find(|transaction| transaction.id == transaction_id)
            .or_else(|| {
                self.redo_stack
                    .iter()
                    .find(|transaction| transaction.id == transaction_id)
            })
    }

    fn transaction_mut(&mut self, transaction_id: TransactionId) -> Option<&mut Transaction> {
        self.undo_stack
            .iter_mut()
            .find(|transaction| transaction.id == transaction_id)
            .or_else(|| {
                self.redo_stack
                    .iter_mut()
                    .find(|transaction| transaction.id == transaction_id)
            })
    }

    fn pop_undo(&mut self) -> Option<&mut Transaction> {
        assert_eq!(self.transaction_depth, 0);
        if let Some(transaction) = self.undo_stack.pop() {
            self.redo_stack.push(transaction);
            self.redo_stack.last_mut()
        } else {
            None
        }
    }

    fn pop_redo(&mut self) -> Option<&mut Transaction> {
        assert_eq!(self.transaction_depth, 0);
        if let Some(transaction) = self.redo_stack.pop() {
            self.undo_stack.push(transaction);
            self.undo_stack.last_mut()
        } else {
            None
        }
    }

    fn remove_from_undo(&mut self, transaction_id: TransactionId) -> Option<&Transaction> {
        let ix = self
            .undo_stack
            .iter()
            .rposition(|transaction| transaction.id == transaction_id)?;
        let transaction = self.undo_stack.remove(ix);
        self.redo_stack.push(transaction);
        self.redo_stack.last()
    }

    fn group(&mut self) -> Option<TransactionId> {
        let mut count = 0;
        let mut transactions = self.undo_stack.iter();
        if let Some(mut transaction) = transactions.next_back() {
            while let Some(prev_transaction) = transactions.next_back() {
                if !prev_transaction.suppress_grouping
                    && transaction.first_edit_at - prev_transaction.last_edit_at
                        <= self.group_interval
                {
                    transaction = prev_transaction;
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
        for transaction in self.undo_stack.iter().rev() {
            if transaction.id == transaction_id {
                self.group_trailing(count);
                break;
            } else if transaction.suppress_grouping {
                break;
            } else {
                count += 1;
            }
        }
    }

    fn group_trailing(&mut self, n: usize) -> Option<TransactionId> {
        let new_len = self.undo_stack.len() - n;
        let (transactions_to_keep, transactions_to_merge) = self.undo_stack.split_at_mut(new_len);
        if let Some(last_transaction) = transactions_to_keep.last_mut() {
            if let Some(transaction) = transactions_to_merge.last() {
                last_transaction.last_edit_at = transaction.last_edit_at;
            }
            for to_merge in transactions_to_merge {
                for (buffer_id, transaction_id) in &to_merge.buffer_transactions {
                    last_transaction
                        .buffer_transactions
                        .entry(*buffer_id)
                        .or_insert(*transaction_id);
                }
            }
        }

        self.undo_stack.truncate(new_len);
        self.undo_stack.last().map(|t| t.id)
    }
}

impl Excerpt {
    fn new(
        id: ExcerptId,
        locator: Locator,
        buffer_id: BufferId,
        buffer: BufferSnapshot,
        range: ExcerptRange<text::Anchor>,
        has_trailing_newline: bool,
    ) -> Self {
        Excerpt {
            id,
            locator,
            max_buffer_row: range.context.end.to_point(&buffer).row,
            text_summary: buffer
                .text_summary_for_range::<TextSummary, _>(range.context.to_offset(&buffer)),
            buffer_id,
            buffer,
            range,
            has_trailing_newline,
        }
    }

    fn chunks_in_range(&self, range: Range<usize>, language_aware: bool) -> ExcerptChunks<'_> {
        let content_start = self.range.context.start.to_offset(&self.buffer);
        let chunks_start = content_start + range.start;
        let chunks_end = content_start + cmp::min(range.end, self.text_summary.len);

        let footer_height = if self.has_trailing_newline
            && range.start <= self.text_summary.len
            && range.end > self.text_summary.len
        {
            1
        } else {
            0
        };

        let content_chunks = self.buffer.chunks(chunks_start..chunks_end, language_aware);

        ExcerptChunks {
            excerpt_id: self.id,
            content_chunks,
            footer_height,
        }
    }

    fn seek_chunks(&self, excerpt_chunks: &mut ExcerptChunks, range: Range<usize>) {
        let content_start = self.range.context.start.to_offset(&self.buffer);
        let chunks_start = content_start + range.start;
        let chunks_end = content_start + cmp::min(range.end, self.text_summary.len);
        excerpt_chunks.content_chunks.seek(chunks_start..chunks_end);
        excerpt_chunks.footer_height = if self.has_trailing_newline
            && range.start <= self.text_summary.len
            && range.end > self.text_summary.len
        {
            1
        } else {
            0
        };
    }

    fn clip_anchor(&self, text_anchor: text::Anchor) -> text::Anchor {
        if text_anchor
            .cmp(&self.range.context.start, &self.buffer)
            .is_lt()
        {
            self.range.context.start
        } else if text_anchor
            .cmp(&self.range.context.end, &self.buffer)
            .is_gt()
        {
            self.range.context.end
        } else {
            text_anchor
        }
    }

    fn contains(&self, anchor: &Anchor) -> bool {
        (anchor.buffer_id == None || anchor.buffer_id == Some(self.buffer_id))
            && self
                .range
                .context
                .start
                .cmp(&anchor.text_anchor, &self.buffer)
                .is_le()
            && self
                .range
                .context
                .end
                .cmp(&anchor.text_anchor, &self.buffer)
                .is_ge()
    }

    /// The [`Excerpt`]'s start offset in its [`Buffer`]
    fn buffer_start_offset(&self) -> usize {
        self.range.context.start.to_offset(&self.buffer)
    }

    /// The [`Excerpt`]'s end offset in its [`Buffer`]
    fn buffer_end_offset(&self) -> usize {
        self.buffer_start_offset() + self.text_summary.len
    }
}

impl<'a> MultiBufferExcerpt<'a> {
    pub fn id(&self) -> ExcerptId {
        self.excerpt.id
    }

    pub fn buffer_id(&self) -> BufferId {
        self.excerpt.buffer_id
    }

    pub fn start_anchor(&self) -> Anchor {
        Anchor {
            buffer_id: Some(self.excerpt.buffer_id),
            excerpt_id: self.excerpt.id,
            text_anchor: self.excerpt.range.context.start,
            diff_base_anchor: None,
        }
    }

    pub fn end_anchor(&self) -> Anchor {
        Anchor {
            buffer_id: Some(self.excerpt.buffer_id),
            excerpt_id: self.excerpt.id,
            text_anchor: self.excerpt.range.context.end,
            diff_base_anchor: None,
        }
    }

    pub fn buffer(&self) -> &'a BufferSnapshot {
        &self.excerpt.buffer
    }

    pub fn buffer_range(&self) -> Range<usize> {
        self.buffer_offset
            ..self
                .excerpt
                .range
                .context
                .end
                .to_offset(&self.excerpt.buffer.text)
    }

    pub fn start_offset(&self) -> usize {
        self.offset
    }

    /// Maps an offset within the [`MultiBuffer`] to an offset within the [`Buffer`]
    pub fn map_offset_to_buffer(&mut self, offset: usize) -> usize {
        self.map_range_to_buffer(offset..offset).start
    }

    /// Maps a range within the [`MultiBuffer`] to a range within the [`Buffer`]
    pub fn map_range_to_buffer(&mut self, range: Range<usize>) -> Range<usize> {
        self.diff_transforms
            .seek(&OutputDimension(range.start), Bias::Right);
        let start = self.map_offset_to_buffer_internal(range.start);
        let end = if range.end > range.start {
            self.diff_transforms
                .seek_forward(&OutputDimension(range.end), Bias::Right);
            self.map_offset_to_buffer_internal(range.end)
        } else {
            start
        };
        start..end
    }

    fn map_offset_to_buffer_internal(&self, offset: usize) -> usize {
        let mut excerpt_offset = self.diff_transforms.start().excerpt_dimension.clone();
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            excerpt_offset.0 += offset - self.diff_transforms.start().output_dimension.0;
        };
        let offset_in_excerpt = excerpt_offset.0.saturating_sub(self.excerpt_offset.0);
        self.buffer_offset + offset_in_excerpt
    }

    /// Map an offset within the [`Buffer`] to an offset within the [`MultiBuffer`]
    pub fn map_offset_from_buffer(&mut self, buffer_offset: usize) -> usize {
        self.map_range_from_buffer(buffer_offset..buffer_offset)
            .start
    }

    /// Map a range within the [`Buffer`] to a range within the [`MultiBuffer`]
    pub fn map_range_from_buffer(&mut self, buffer_range: Range<usize>) -> Range<usize> {
        if buffer_range.start < self.buffer_offset {
            log::warn!(
                "Attempting to map a range from a buffer offset that starts before the current buffer offset"
            );
            return buffer_range;
        }
        let overshoot = buffer_range.start - self.buffer_offset;
        let excerpt_offset = ExcerptDimension(self.excerpt_offset.0 + overshoot);
        self.diff_transforms.seek(&excerpt_offset, Bias::Right);
        if excerpt_offset.0 < self.diff_transforms.start().excerpt_dimension.0 {
            log::warn!(
                "Attempting to map a range from a buffer offset that starts before the current buffer offset"
            );
            return buffer_range;
        }
        let overshoot = excerpt_offset.0 - self.diff_transforms.start().excerpt_dimension.0;
        let start = self.diff_transforms.start().output_dimension.0 + overshoot;

        let end = if buffer_range.end > buffer_range.start {
            let overshoot = buffer_range.end - self.buffer_offset;
            let excerpt_offset = ExcerptDimension(self.excerpt_offset.0 + overshoot);
            self.diff_transforms
                .seek_forward(&excerpt_offset, Bias::Right);
            let overshoot = excerpt_offset.0 - self.diff_transforms.start().excerpt_dimension.0;
            self.diff_transforms.start().output_dimension.0 + overshoot
        } else {
            start
        };

        start..end
    }

    /// Returns true if the entirety of the given range is in the buffer's excerpt
    pub fn contains_buffer_range(&self, range: Range<usize>) -> bool {
        range.start >= self.excerpt.buffer_start_offset()
            && range.end <= self.excerpt.buffer_end_offset()
    }

    pub fn max_buffer_row(&self) -> u32 {
        self.excerpt.max_buffer_row
    }
}

impl ExcerptId {
    pub fn min() -> Self {
        Self(0)
    }

    pub fn max() -> Self {
        Self(usize::MAX)
    }

    pub fn to_proto(self) -> u64 {
        self.0 as _
    }

    pub fn from_proto(proto: u64) -> Self {
        Self(proto as _)
    }

    pub fn cmp(&self, other: &Self, snapshot: &MultiBufferSnapshot) -> cmp::Ordering {
        let a = snapshot.excerpt_locator_for_id(*self);
        let b = snapshot.excerpt_locator_for_id(*other);
        a.cmp(b).then_with(|| self.0.cmp(&other.0))
    }
}

impl From<ExcerptId> for usize {
    fn from(val: ExcerptId) -> Self {
        val.0
    }
}

impl fmt::Debug for Excerpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Excerpt")
            .field("id", &self.id)
            .field("locator", &self.locator)
            .field("buffer_id", &self.buffer_id)
            .field("range", &self.range)
            .field("text_summary", &self.text_summary)
            .field("has_trailing_newline", &self.has_trailing_newline)
            .finish()
    }
}

impl sum_tree::Item for Excerpt {
    type Summary = ExcerptSummary;

    fn summary(&self, _cx: &()) -> Self::Summary {
        let mut text = self.text_summary;
        if self.has_trailing_newline {
            text += TextSummary::from("\n");
        }
        ExcerptSummary {
            excerpt_id: self.id,
            excerpt_locator: self.locator.clone(),
            widest_line_number: self.max_buffer_row,
            text,
        }
    }
}

impl sum_tree::Item for ExcerptIdMapping {
    type Summary = ExcerptId;

    fn summary(&self, _cx: &()) -> Self::Summary {
        self.id
    }
}

impl sum_tree::KeyedItem for ExcerptIdMapping {
    type Key = ExcerptId;

    fn key(&self) -> Self::Key {
        self.id
    }
}

impl DiffTransform {
    fn hunk_info(&self) -> Option<DiffTransformHunkInfo> {
        match self {
            DiffTransform::DeletedHunk { hunk_info, .. } => Some(*hunk_info),
            DiffTransform::BufferContent {
                inserted_hunk_info, ..
            } => *inserted_hunk_info,
        }
    }
}

impl sum_tree::Item for DiffTransform {
    type Summary = DiffTransformSummary;

    fn summary(&self, _: &<Self::Summary as sum_tree::Summary>::Context) -> Self::Summary {
        match self {
            DiffTransform::BufferContent { summary, .. } => DiffTransformSummary {
                input: *summary,
                output: *summary,
            },
            DiffTransform::DeletedHunk { summary, .. } => DiffTransformSummary {
                input: TextSummary::default(),
                output: *summary,
            },
        }
    }
}

impl DiffTransformSummary {
    fn excerpt_len(&self) -> ExcerptOffset {
        ExcerptOffset::new(self.input.len)
    }
}

impl sum_tree::Summary for DiffTransformSummary {
    type Context = ();

    fn zero(_: &Self::Context) -> Self {
        DiffTransformSummary {
            input: TextSummary::default(),
            output: TextSummary::default(),
        }
    }

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.input += &summary.input;
        self.output += &summary.output;
    }
}

impl sum_tree::Summary for ExcerptId {
    type Context = ();

    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self, _: &()) {
        *self = *other;
    }
}

impl sum_tree::Summary for ExcerptSummary {
    type Context = ();

    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self, _: &()) {
        debug_assert!(summary.excerpt_locator > self.excerpt_locator);
        self.excerpt_locator = summary.excerpt_locator.clone();
        Summary::add_summary(&mut self.text, &summary.text, &());
        self.widest_line_number = cmp::max(self.widest_line_number, summary.widest_line_number);
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for ExcerptOffset {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        self.value += summary.text.len;
    }
}

impl sum_tree::SeekTarget<'_, ExcerptSummary, ExcerptSummary> for ExcerptOffset {
    fn cmp(&self, cursor_location: &ExcerptSummary, _: &()) -> cmp::Ordering {
        Ord::cmp(&self.value, &cursor_location.text.len)
    }
}

impl<'a> sum_tree::SeekTarget<'a, ExcerptSummary, Option<&'a Locator>> for Locator {
    fn cmp(&self, cursor_location: &Option<&'a Locator>, _: &()) -> cmp::Ordering {
        Ord::cmp(&Some(self), cursor_location)
    }
}

impl sum_tree::SeekTarget<'_, ExcerptSummary, ExcerptSummary> for Locator {
    fn cmp(&self, cursor_location: &ExcerptSummary, _: &()) -> cmp::Ordering {
        Ord::cmp(self, &cursor_location.excerpt_locator)
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for ExcerptPoint {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        self.value += summary.text.lines;
    }
}

impl<'a, D: TextDimension + Default> sum_tree::Dimension<'a, ExcerptSummary>
    for ExcerptDimension<D>
{
    fn zero(_: &()) -> Self {
        ExcerptDimension(D::default())
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        self.0.add_assign(&D::from_text_summary(&summary.text))
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for Option<&'a Locator> {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        *self = Some(&summary.excerpt_locator);
    }
}

impl<'a> sum_tree::Dimension<'a, ExcerptSummary> for Option<ExcerptId> {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: &()) {
        *self = Some(summary.excerpt_id);
    }
}

#[derive(Clone, PartialOrd, Ord, Eq, PartialEq, Debug)]
struct ExcerptDimension<T>(T);

#[derive(Clone, PartialOrd, Ord, Eq, PartialEq, Debug)]
struct OutputDimension<T>(T);

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for ExcerptOffset {
    fn zero(_: &()) -> Self {
        ExcerptOffset::new(0)
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.value += summary.input.len;
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for ExcerptPoint {
    fn zero(_: &()) -> Self {
        ExcerptPoint::new(0, 0)
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.value += summary.input.lines;
    }
}

impl<D: TextDimension + Ord> sum_tree::SeekTarget<'_, DiffTransformSummary, DiffTransformSummary>
    for ExcerptDimension<D>
{
    fn cmp(&self, cursor_location: &DiffTransformSummary, _: &()) -> cmp::Ordering {
        Ord::cmp(&self.0, &D::from_text_summary(&cursor_location.input))
    }
}

impl<D: TextDimension + Ord> sum_tree::SeekTarget<'_, DiffTransformSummary, DiffTransforms<D>>
    for ExcerptDimension<D>
{
    fn cmp(&self, cursor_location: &DiffTransforms<D>, _: &()) -> cmp::Ordering {
        Ord::cmp(&self.0, &cursor_location.excerpt_dimension.0)
    }
}

impl<'a, D: TextDimension> sum_tree::Dimension<'a, DiffTransformSummary> for ExcerptDimension<D> {
    fn zero(_: &()) -> Self {
        ExcerptDimension(D::default())
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0.add_assign(&D::from_text_summary(&summary.input))
    }
}

impl<D: TextDimension + Ord> sum_tree::SeekTarget<'_, DiffTransformSummary, DiffTransforms<D>>
    for OutputDimension<D>
{
    fn cmp(&self, cursor_location: &DiffTransforms<D>, _: &()) -> cmp::Ordering {
        Ord::cmp(&self.0, &cursor_location.output_dimension.0)
    }
}

impl<'a, D: TextDimension> sum_tree::Dimension<'a, DiffTransformSummary> for OutputDimension<D> {
    fn zero(_: &()) -> Self {
        OutputDimension(D::default())
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0.add_assign(&D::from_text_summary(&summary.output))
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for TextSummary {
    fn zero(_: &()) -> Self {
        TextSummary::default()
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        *self += summary.output
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for usize {
    fn zero(_: &()) -> Self {
        0
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        *self += summary.output.len
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for Point {
    fn zero(_: &()) -> Self {
        Point::new(0, 0)
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        *self += summary.output.lines
    }
}

impl MultiBufferRows<'_> {
    pub fn seek(&mut self, MultiBufferRow(row): MultiBufferRow) {
        self.point = Point::new(row, 0);
        self.cursor.seek(&self.point);
    }
}

impl Iterator for MultiBufferRows<'_> {
    type Item = RowInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty && self.point.row == 0 {
            self.point += Point::new(1, 0);
            return Some(RowInfo {
                buffer_id: None,
                buffer_row: Some(0),
                multibuffer_row: Some(MultiBufferRow(0)),
                diff_status: None,
                expand_info: None,
            });
        }

        let mut region = self.cursor.region()?;
        while self.point >= region.range.end {
            self.cursor.next();
            if let Some(next_region) = self.cursor.region() {
                region = next_region;
            } else if self.point == self.cursor.diff_transforms.end().output_dimension.0 {
                let multibuffer_row = MultiBufferRow(self.point.row);
                let last_excerpt = self
                    .cursor
                    .excerpts
                    .item()
                    .or(self.cursor.excerpts.prev_item())?;
                let last_row = last_excerpt
                    .range
                    .context
                    .end
                    .to_point(&last_excerpt.buffer)
                    .row;

                let first_row = last_excerpt
                    .range
                    .context
                    .start
                    .to_point(&last_excerpt.buffer)
                    .row;

                let expand_info = if self.is_singleton {
                    None
                } else {
                    let needs_expand_up = first_row == last_row
                        && last_row > 0
                        && !region.diff_hunk_status.is_some_and(|d| d.is_deleted());
                    let needs_expand_down = last_row < last_excerpt.buffer.max_point().row;

                    if needs_expand_up && needs_expand_down {
                        Some(ExpandExcerptDirection::UpAndDown)
                    } else if needs_expand_up {
                        Some(ExpandExcerptDirection::Up)
                    } else if needs_expand_down {
                        Some(ExpandExcerptDirection::Down)
                    } else {
                        None
                    }
                    .map(|direction| ExpandInfo {
                        direction,
                        excerpt_id: last_excerpt.id,
                    })
                };
                self.point += Point::new(1, 0);
                return Some(RowInfo {
                    buffer_id: Some(last_excerpt.buffer_id),
                    buffer_row: Some(last_row),
                    multibuffer_row: Some(multibuffer_row),
                    diff_status: None,
                    expand_info,
                });
            } else {
                return None;
            };
        }

        let overshoot = self.point - region.range.start;
        let buffer_point = region.buffer_range.start + overshoot;
        let expand_info = if self.is_singleton {
            None
        } else {
            let needs_expand_up = self.point.row == region.range.start.row
                && self.cursor.is_at_start_of_excerpt()
                && buffer_point.row > 0;
            let needs_expand_down = (region.excerpt.has_trailing_newline
                && self.point.row + 1 == region.range.end.row
                || !region.excerpt.has_trailing_newline && self.point.row == region.range.end.row)
                && self.cursor.is_at_end_of_excerpt()
                && buffer_point.row < region.buffer.max_point().row;

            if needs_expand_up && needs_expand_down {
                Some(ExpandExcerptDirection::UpAndDown)
            } else if needs_expand_up {
                Some(ExpandExcerptDirection::Up)
            } else if needs_expand_down {
                Some(ExpandExcerptDirection::Down)
            } else {
                None
            }
            .map(|direction| ExpandInfo {
                direction,
                excerpt_id: region.excerpt.id,
            })
        };

        let result = Some(RowInfo {
            buffer_id: Some(region.buffer.remote_id()),
            buffer_row: Some(buffer_point.row),
            multibuffer_row: Some(MultiBufferRow(self.point.row)),
            diff_status: region
                .diff_hunk_status
                .filter(|_| self.point < region.range.end),
            expand_info,
        });
        self.point += Point::new(1, 0);
        result
    }
}

impl<'a> MultiBufferChunks<'a> {
    pub fn offset(&self) -> usize {
        self.range.start
    }

    pub fn seek(&mut self, range: Range<usize>) {
        self.diff_transforms.seek(&range.end, Bias::Right);
        let mut excerpt_end = self.diff_transforms.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            let overshoot = range.end - self.diff_transforms.start().0;
            excerpt_end.value += overshoot;
        }

        self.diff_transforms.seek(&range.start, Bias::Right);
        let mut excerpt_start = self.diff_transforms.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            let overshoot = range.start - self.diff_transforms.start().0;
            excerpt_start.value += overshoot;
        }

        self.seek_to_excerpt_offset_range(excerpt_start..excerpt_end);
        self.buffer_chunk.take();
        self.range = range;
    }

    fn seek_to_excerpt_offset_range(&mut self, new_range: Range<ExcerptOffset>) {
        self.excerpt_offset_range = new_range.clone();
        self.excerpts.seek(&new_range.start, Bias::Right);
        if let Some(excerpt) = self.excerpts.item() {
            let excerpt_start = *self.excerpts.start();
            if let Some(excerpt_chunks) = self
                .excerpt_chunks
                .as_mut()
                .filter(|chunks| excerpt.id == chunks.excerpt_id)
            {
                excerpt.seek_chunks(
                    excerpt_chunks,
                    (self.excerpt_offset_range.start - excerpt_start).value
                        ..(self.excerpt_offset_range.end - excerpt_start).value,
                );
            } else {
                self.excerpt_chunks = Some(excerpt.chunks_in_range(
                    (self.excerpt_offset_range.start - excerpt_start).value
                        ..(self.excerpt_offset_range.end - excerpt_start).value,
                    self.language_aware,
                ));
            }
        } else {
            self.excerpt_chunks = None;
        }
    }

    fn next_excerpt_chunk(&mut self) -> Option<Chunk<'a>> {
        loop {
            if self.excerpt_offset_range.is_empty() {
                return None;
            } else if let Some(chunk) = self.excerpt_chunks.as_mut()?.next() {
                self.excerpt_offset_range.start.value += chunk.text.len();
                return Some(chunk);
            } else {
                self.excerpts.next();
                let excerpt = self.excerpts.item()?;
                self.excerpt_chunks = Some(excerpt.chunks_in_range(
                    0..(self.excerpt_offset_range.end - *self.excerpts.start()).value,
                    self.language_aware,
                ));
            }
        }
    }
}

impl<'a> Iterator for ReversedMultiBufferChunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut region = self.cursor.region()?;
        if self.offset == region.range.start {
            self.cursor.prev();
            region = self.cursor.region()?;
            let start_overshoot = self.start.saturating_sub(region.range.start);
            self.current_chunks = Some(region.buffer.reversed_chunks_in_range(
                region.buffer_range.start + start_overshoot..region.buffer_range.end,
            ));
        }

        if self.offset == region.range.end && region.has_trailing_newline {
            self.offset -= 1;
            Some("\n")
        } else {
            let chunk = self.current_chunks.as_mut().unwrap().next()?;
            self.offset -= chunk.len();
            Some(chunk)
        }
    }
}

impl<'a> Iterator for MultiBufferChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Chunk<'a>> {
        if self.range.start >= self.range.end {
            return None;
        }
        if self.range.start == self.diff_transforms.end().0 {
            self.diff_transforms.next();
        }

        let diff_transform_start = self.diff_transforms.start().0;
        let diff_transform_end = self.diff_transforms.end().0;
        debug_assert!(self.range.start < diff_transform_end);

        let diff_transform = self.diff_transforms.item()?;
        match diff_transform {
            DiffTransform::BufferContent { .. } => {
                let chunk = if let Some(chunk) = &mut self.buffer_chunk {
                    chunk
                } else {
                    let chunk = self.next_excerpt_chunk().unwrap();
                    self.buffer_chunk.insert(chunk)
                };

                let chunk_end = self.range.start + chunk.text.len();
                let diff_transform_end = diff_transform_end.min(self.range.end);

                if diff_transform_end < chunk_end {
                    let split_idx = diff_transform_end - self.range.start;
                    let (before, after) = chunk.text.split_at(split_idx);
                    self.range.start = diff_transform_end;
                    let mask = (1 << split_idx) - 1;
                    let chars = chunk.chars & mask;
                    let tabs = chunk.tabs & mask;

                    chunk.text = after;
                    chunk.chars = chunk.chars >> split_idx;
                    chunk.tabs = chunk.tabs >> split_idx;

                    Some(Chunk {
                        text: before,
                        chars,
                        tabs,
                        ..chunk.clone()
                    })
                } else {
                    self.range.start = chunk_end;
                    self.buffer_chunk.take()
                }
            }
            DiffTransform::DeletedHunk {
                buffer_id,
                base_text_byte_range,
                has_trailing_newline,
                ..
            } => {
                let base_text_start =
                    base_text_byte_range.start + self.range.start - diff_transform_start;
                let base_text_end =
                    base_text_byte_range.start + self.range.end - diff_transform_start;
                let base_text_end = base_text_end.min(base_text_byte_range.end);

                let mut chunks = if let Some((_, mut chunks)) = self
                    .diff_base_chunks
                    .take()
                    .filter(|(id, _)| id == buffer_id)
                {
                    if chunks.range().start != base_text_start || chunks.range().end < base_text_end
                    {
                        chunks.seek(base_text_start..base_text_end);
                    }
                    chunks
                } else {
                    let base_buffer = &self.diffs.get(buffer_id)?.base_text();
                    base_buffer.chunks(base_text_start..base_text_end, self.language_aware)
                };

                let chunk = if let Some(chunk) = chunks.next() {
                    self.range.start += chunk.text.len();
                    self.diff_base_chunks = Some((*buffer_id, chunks));
                    chunk
                } else {
                    debug_assert!(has_trailing_newline);
                    self.range.start += "\n".len();
                    Chunk {
                        text: "\n",
                        chars: 1u128,
                        ..Default::default()
                    }
                };
                Some(chunk)
            }
        }
    }
}

impl MultiBufferBytes<'_> {
    fn consume(&mut self, len: usize) {
        self.range.start += len;
        self.chunk = &self.chunk[len..];

        if !self.range.is_empty() && self.chunk.is_empty() {
            if let Some(chunk) = self.excerpt_bytes.as_mut().and_then(|bytes| bytes.next()) {
                self.chunk = chunk;
            } else if self.has_trailing_newline {
                self.has_trailing_newline = false;
                self.chunk = b"\n";
            } else {
                self.cursor.next();
                if let Some(region) = self.cursor.region() {
                    let mut excerpt_bytes = region.buffer.bytes_in_range(
                        region.buffer_range.start
                            ..(region.buffer_range.start + self.range.end - region.range.start)
                                .min(region.buffer_range.end),
                    );
                    self.chunk = excerpt_bytes.next().unwrap_or(&[]);
                    self.excerpt_bytes = Some(excerpt_bytes);
                    self.has_trailing_newline =
                        region.has_trailing_newline && self.range.end >= region.range.end;
                    if self.chunk.is_empty() && self.has_trailing_newline {
                        self.has_trailing_newline = false;
                        self.chunk = b"\n";
                    }
                }
            }
        }
    }
}

impl<'a> Iterator for MultiBufferBytes<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunk;
        if chunk.is_empty() {
            None
        } else {
            self.consume(chunk.len());
            Some(chunk)
        }
    }
}

impl io::Read for MultiBufferBytes<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = cmp::min(buf.len(), self.chunk.len());
        buf[..len].copy_from_slice(&self.chunk[..len]);
        if len > 0 {
            self.consume(len);
        }
        Ok(len)
    }
}

impl io::Read for ReversedMultiBufferBytes<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = cmp::min(buf.len(), self.chunk.len());
        buf[..len].copy_from_slice(&self.chunk[..len]);
        buf[..len].reverse();
        if len > 0 {
            self.range.end -= len;
            self.chunk = &self.chunk[..self.chunk.len() - len];
            if !self.range.is_empty()
                && self.chunk.is_empty()
                && let Some(chunk) = self.chunks.next()
            {
                self.chunk = chunk.as_bytes();
            }
        }
        Ok(len)
    }
}

impl<'a> Iterator for ExcerptChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chunk) = self.content_chunks.next() {
            return Some(chunk);
        }

        if self.footer_height > 0 {
            let text = unsafe { str::from_utf8_unchecked(&NEWLINES[..self.footer_height]) };
            let chars = (1 << self.footer_height) - 1;
            self.footer_height = 0;
            return Some(Chunk {
                text,
                chars,
                ..Default::default()
            });
        }

        None
    }
}

impl ToOffset for Point {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        snapshot.point_to_offset(*self)
    }
}

impl ToOffset for usize {
    #[track_caller]
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        assert!(
            *self <= snapshot.len(),
            "offset {} is greater than the snapshot.len() {}",
            *self,
            snapshot.len(),
        );
        *self
    }
}

impl ToOffset for OffsetUtf16 {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        snapshot.offset_utf16_to_offset(*self)
    }
}

impl ToOffset for PointUtf16 {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        snapshot.point_utf16_to_offset(*self)
    }
}

impl ToOffsetUtf16 for OffsetUtf16 {
    fn to_offset_utf16(&self, _snapshot: &MultiBufferSnapshot) -> OffsetUtf16 {
        *self
    }
}

impl ToOffsetUtf16 for usize {
    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> OffsetUtf16 {
        snapshot.offset_to_offset_utf16(*self)
    }
}

impl ToPoint for usize {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point {
        snapshot.offset_to_point(*self)
    }
}

impl ToPoint for Point {
    fn to_point<'a>(&self, _: &MultiBufferSnapshot) -> Point {
        *self
    }
}

impl ToPointUtf16 for usize {
    fn to_point_utf16<'a>(&self, snapshot: &MultiBufferSnapshot) -> PointUtf16 {
        snapshot.offset_to_point_utf16(*self)
    }
}

impl ToPointUtf16 for Point {
    fn to_point_utf16<'a>(&self, snapshot: &MultiBufferSnapshot) -> PointUtf16 {
        snapshot.point_to_point_utf16(*self)
    }
}

impl ToPointUtf16 for PointUtf16 {
    fn to_point_utf16<'a>(&self, _: &MultiBufferSnapshot) -> PointUtf16 {
        *self
    }
}

impl From<ExcerptId> for EntityId {
    fn from(id: ExcerptId) -> Self {
        EntityId::from(id.0 as u64)
    }
}

#[cfg(debug_assertions)]
pub mod debug {
    use super::*;

    pub trait ToMultiBufferDebugRanges {
        fn to_multi_buffer_debug_ranges(&self, snapshot: &MultiBufferSnapshot)
        -> Vec<Range<usize>>;
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for T {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<usize>> {
            [self.to_offset(snapshot)].to_multi_buffer_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for Range<T> {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<usize>> {
            [self.start.to_offset(snapshot)..self.end.to_offset(snapshot)]
                .to_multi_buffer_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for Vec<T> {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<usize>> {
            self.as_slice().to_multi_buffer_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for Vec<Range<T>> {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<usize>> {
            self.as_slice().to_multi_buffer_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for [T] {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<usize>> {
            self.iter()
                .map(|item| {
                    let offset = item.to_offset(snapshot);
                    offset..offset
                })
                .collect()
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for [Range<T>] {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<usize>> {
            self.iter()
                .map(|range| range.start.to_offset(snapshot)..range.end.to_offset(snapshot))
                .collect()
        }
    }
}
