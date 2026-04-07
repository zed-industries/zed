mod anchor;
#[cfg(test)]
mod multi_buffer_tests;
mod path_key;
mod transaction;

use self::transaction::History;

pub use anchor::{Anchor, AnchorRangeExt};

use anchor::{AnchorSeekTarget, ExcerptAnchor};
use anyhow::{Result, anyhow};
use buffer_diff::{
    BufferDiff, BufferDiffEvent, BufferDiffSnapshot, DiffChanged, DiffHunkSecondaryStatus,
    DiffHunkStatus, DiffHunkStatusKind,
};
use clock::ReplicaId;
use collections::{BTreeMap, Bound, HashMap, HashSet};
use gpui::{App, Context, Entity, EventEmitter};
use itertools::Itertools;
use language::{
    AutoindentMode, Buffer, BufferChunks, BufferRow, BufferSnapshot, Capability, CharClassifier,
    CharKind, CharScopeContext, Chunk, CursorShape, DiagnosticEntryRef, File, IndentGuideSettings,
    IndentSize, Language, LanguageAwareStyling, LanguageScope, OffsetRangeExt, OffsetUtf16,
    Outline, OutlineItem, Point, PointUtf16, Selection, TextDimension, TextObject, ToOffset as _,
    ToPoint as _, TransactionId, TreeSitterOptions, Unclipped,
    language_settings::{AllLanguageSettings, LanguageSettings},
};

#[cfg(any(test, feature = "test-support"))]
use gpui::AppContext as _;

use rope::DimensionPair;
use settings::Settings;
use smallvec::SmallVec;
use smol::future::yield_now;
use std::{
    any::type_name,
    borrow::Cow,
    cell::{Cell, OnceCell, Ref, RefCell},
    cmp::{self, Ordering},
    fmt,
    future::Future,
    io,
    iter::{self, FromIterator},
    mem,
    ops::{self, Add, AddAssign, ControlFlow, Range, RangeBounds, Sub, SubAssign},
    rc::Rc,
    str,
    sync::{Arc, OnceLock},
    time::Duration,
};
use sum_tree::{Bias, Cursor, Dimension, Dimensions, SumTree, TreeMap};
use text::{
    BufferId, Edit, LineIndent, TextSummary,
    subscription::{Subscription, Topic},
};
use theme::SyntaxTheme;
use unicode_segmentation::UnicodeSegmentation;
use ztracing::instrument;

pub use self::path_key::PathKey;

pub static EXCERPT_CONTEXT_LINES: OnceLock<fn(&App) -> u32> = OnceLock::new();

pub fn excerpt_context_lines(cx: &App) -> u32 {
    EXCERPT_CONTEXT_LINES.get().map(|f| f(cx)).unwrap_or(2)
}

/// One or more [`Buffers`](Buffer) being edited in a single view.
///
/// See <https://zed.dev/features#multi-buffers>
pub struct MultiBuffer {
    /// A snapshot of the [`Excerpt`]s in the MultiBuffer.
    /// Use [`MultiBuffer::snapshot`] to get a up-to-date snapshot.
    snapshot: RefCell<MultiBufferSnapshot>,
    /// Contains the state of the buffers being edited
    buffers: BTreeMap<BufferId, BufferState>,
    /// Mapping from buffer IDs to their diff states
    diffs: HashMap<BufferId, DiffState>,
    subscriptions: Topic<MultiBufferOffset>,
    /// If true, the multi-buffer only contains a single [`Buffer`] and a single [`Excerpt`]
    singleton: bool,
    /// The history of the multi-buffer.
    history: History,
    /// The explicit title of the multi-buffer.
    /// If `None`, it will be derived from the underlying path or content.
    title: Option<String>,
    /// The writing capability of the multi-buffer.
    capability: Capability,
    buffer_changed_since_sync: Rc<Cell<bool>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PathKeyIndex(u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    BufferRangesUpdated {
        buffer: Entity<Buffer>,
        path_key: PathKey,
        ranges: Vec<ExcerptRange<text::Anchor>>,
    },
    BuffersRemoved {
        removed_buffer_ids: Vec<BufferId>,
    },
    BuffersEdited {
        buffer_ids: Vec<BufferId>,
    },
    DiffHunksToggled,
    Edited {
        edited_buffer: Option<Entity<Buffer>>,
        is_local: bool,
    },
    TransactionUndone {
        transaction_id: TransactionId,
    },
    Reloaded,
    LanguageChanged(BufferId, bool),
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
    /// The range within the buffer's diff base that this hunk corresponds to.
    pub diff_base_byte_range: Range<BufferOffset>,
    /// The status of this hunk (added/modified/deleted and secondary status).
    pub status: DiffHunkStatus,
    /// The word diffs for this hunk.
    pub word_diffs: Vec<Range<MultiBufferOffset>>,
    pub excerpt_range: ExcerptRange<text::Anchor>,
    pub multi_buffer_range: Range<Anchor>,
}

impl MultiBufferDiffHunk {
    pub fn status(&self) -> DiffHunkStatus {
        self.status
    }

    pub fn is_created_file(&self) -> bool {
        self.diff_base_byte_range == (BufferOffset(0)..BufferOffset(0))
            && self.buffer_range.start.is_min()
            && self.buffer_range.end.is_max()
    }
}

pub type MultiBufferPoint = Point;
/// ExcerptOffset is offset into the non-deleted text of the multibuffer
type ExcerptOffset = ExcerptDimension<MultiBufferOffset>;
/// ExcerptOffset is based on the non-deleted text of the multibuffer

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Hash, serde::Deserialize)]
#[serde(transparent)]
pub struct MultiBufferRow(pub u32);

impl MultiBufferRow {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(u32::MAX);
}

impl ops::Add<usize> for MultiBufferRow {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        MultiBufferRow(self.0 + rhs as u32)
    }
}

pub trait MultiBufferDimension: 'static + Copy + Default + std::fmt::Debug {
    type TextDimension: TextDimension;
    fn from_summary(summary: &MBTextSummary) -> Self;

    fn add_text_dim(&mut self, summary: &Self::TextDimension);

    fn add_mb_text_summary(&mut self, summary: &MBTextSummary);
}

// todo(lw): MultiBufferPoint
impl MultiBufferDimension for Point {
    type TextDimension = Point;
    fn from_summary(summary: &MBTextSummary) -> Self {
        summary.lines
    }

    fn add_text_dim(&mut self, other: &Self::TextDimension) {
        *self += *other;
    }

    fn add_mb_text_summary(&mut self, summary: &MBTextSummary) {
        *self += summary.lines;
    }
}

// todo(lw): MultiBufferPointUtf16
impl MultiBufferDimension for PointUtf16 {
    type TextDimension = PointUtf16;
    fn from_summary(summary: &MBTextSummary) -> Self {
        summary.lines_utf16()
    }

    fn add_text_dim(&mut self, other: &Self::TextDimension) {
        *self += *other;
    }

    fn add_mb_text_summary(&mut self, summary: &MBTextSummary) {
        *self += summary.lines_utf16();
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Hash, serde::Deserialize)]
pub struct MultiBufferOffset(pub usize);

impl fmt::Display for MultiBufferOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl rand::distr::uniform::SampleUniform for MultiBufferOffset {
    type Sampler = MultiBufferOffsetUniformSampler;
}

pub struct MultiBufferOffsetUniformSampler {
    sampler: rand::distr::uniform::UniformUsize,
}

impl rand::distr::uniform::UniformSampler for MultiBufferOffsetUniformSampler {
    type X = MultiBufferOffset;

    fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        let sampler = rand::distr::uniform::UniformUsize::new(low.0, high.0);
        sampler.map(|sampler| MultiBufferOffsetUniformSampler { sampler })
    }

    #[inline] // if the range is constant, this helps LLVM to do the
    // calculations at compile-time.
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distr::uniform::SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        let sampler = rand::distr::uniform::UniformUsize::new_inclusive(low.0, high.0);
        sampler.map(|sampler| MultiBufferOffsetUniformSampler { sampler })
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        MultiBufferOffset(self.sampler.sample(rng))
    }
}
impl MultiBufferDimension for MultiBufferOffset {
    type TextDimension = usize;
    fn from_summary(summary: &MBTextSummary) -> Self {
        summary.len
    }

    fn add_text_dim(&mut self, other: &Self::TextDimension) {
        self.0 += *other;
    }

    fn add_mb_text_summary(&mut self, summary: &MBTextSummary) {
        *self += summary.len;
    }
}
impl MultiBufferDimension for MultiBufferOffsetUtf16 {
    type TextDimension = OffsetUtf16;
    fn from_summary(summary: &MBTextSummary) -> Self {
        MultiBufferOffsetUtf16(summary.len_utf16)
    }

    fn add_text_dim(&mut self, other: &Self::TextDimension) {
        self.0 += *other;
    }

    fn add_mb_text_summary(&mut self, summary: &MBTextSummary) {
        self.0 += summary.len_utf16;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq, Hash, serde::Deserialize)]
pub struct BufferOffset(pub usize);

impl TextDimension for BufferOffset {
    fn from_text_summary(summary: &TextSummary) -> Self {
        BufferOffset(usize::from_text_summary(summary))
    }
    fn from_chunk(chunk: rope::ChunkSlice) -> Self {
        BufferOffset(usize::from_chunk(chunk))
    }
    fn add_assign(&mut self, other: &Self) {
        TextDimension::add_assign(&mut self.0, &other.0);
    }
}
impl<'a> sum_tree::Dimension<'a, rope::ChunkSummary> for BufferOffset {
    fn zero(cx: ()) -> Self {
        BufferOffset(<usize as sum_tree::Dimension<'a, rope::ChunkSummary>>::zero(cx))
    }

    fn add_summary(&mut self, summary: &'a rope::ChunkSummary, cx: ()) {
        usize::add_summary(&mut self.0, summary, cx);
    }
}

impl Sub for BufferOffset {
    type Output = usize;

    fn sub(self, other: BufferOffset) -> Self::Output {
        self.0 - other.0
    }
}

impl AddAssign<DimensionPair<usize, Point>> for BufferOffset {
    fn add_assign(&mut self, other: DimensionPair<usize, Point>) {
        self.0 += other.key;
    }
}

impl language::ToPoint for BufferOffset {
    fn to_point(&self, snapshot: &text::BufferSnapshot) -> Point {
        self.0.to_point(snapshot)
    }
}

impl language::ToPointUtf16 for BufferOffset {
    fn to_point_utf16(&self, snapshot: &text::BufferSnapshot) -> PointUtf16 {
        self.0.to_point_utf16(snapshot)
    }
}

impl language::ToOffset for BufferOffset {
    fn to_offset(&self, snapshot: &text::BufferSnapshot) -> usize {
        self.0.to_offset(snapshot)
    }
}

impl language::ToOffsetUtf16 for BufferOffset {
    fn to_offset_utf16(&self, snapshot: &text::BufferSnapshot) -> OffsetUtf16 {
        self.0.to_offset_utf16(snapshot)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct MultiBufferOffsetUtf16(pub OffsetUtf16);

impl ops::Add<usize> for MultiBufferOffsetUtf16 {
    type Output = MultiBufferOffsetUtf16;

    fn add(self, rhs: usize) -> Self::Output {
        MultiBufferOffsetUtf16(OffsetUtf16(self.0.0 + rhs))
    }
}

impl ops::Add<OffsetUtf16> for MultiBufferOffsetUtf16 {
    type Output = Self;

    fn add(self, rhs: OffsetUtf16) -> Self::Output {
        MultiBufferOffsetUtf16(self.0 + rhs)
    }
}

impl AddAssign<OffsetUtf16> for MultiBufferOffsetUtf16 {
    fn add_assign(&mut self, rhs: OffsetUtf16) {
        self.0 += rhs;
    }
}

impl AddAssign<usize> for MultiBufferOffsetUtf16 {
    fn add_assign(&mut self, rhs: usize) {
        self.0.0 += rhs;
    }
}

impl Sub for MultiBufferOffsetUtf16 {
    type Output = OffsetUtf16;

    fn sub(self, other: MultiBufferOffsetUtf16) -> Self::Output {
        self.0 - other.0
    }
}

impl Sub<OffsetUtf16> for MultiBufferOffsetUtf16 {
    type Output = MultiBufferOffsetUtf16;

    fn sub(self, other: OffsetUtf16) -> Self::Output {
        MultiBufferOffsetUtf16(self.0 - other)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct BufferOffsetUtf16(pub OffsetUtf16);

impl MultiBufferOffset {
    const ZERO: Self = Self(0);
    pub fn saturating_sub(self, other: MultiBufferOffset) -> usize {
        self.0.saturating_sub(other.0)
    }
    pub fn saturating_sub_usize(self, other: usize) -> MultiBufferOffset {
        MultiBufferOffset(self.0.saturating_sub(other))
    }
}

impl ops::Sub for MultiBufferOffset {
    type Output = usize;

    fn sub(self, other: MultiBufferOffset) -> Self::Output {
        self.0 - other.0
    }
}

impl ops::Sub<usize> for MultiBufferOffset {
    type Output = Self;

    fn sub(self, other: usize) -> Self::Output {
        MultiBufferOffset(self.0 - other)
    }
}

impl ops::SubAssign<usize> for MultiBufferOffset {
    fn sub_assign(&mut self, other: usize) {
        self.0 -= other;
    }
}

impl ops::Add<usize> for BufferOffset {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        BufferOffset(self.0 + rhs)
    }
}

impl ops::AddAssign<usize> for BufferOffset {
    fn add_assign(&mut self, other: usize) {
        self.0 += other;
    }
}

impl ops::Add<usize> for MultiBufferOffset {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        MultiBufferOffset(self.0 + rhs)
    }
}

impl ops::AddAssign<usize> for MultiBufferOffset {
    fn add_assign(&mut self, other: usize) {
        self.0 += other;
    }
}

impl ops::Add<isize> for MultiBufferOffset {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        MultiBufferOffset((self.0 as isize + rhs) as usize)
    }
}

impl ops::Add for MultiBufferOffset {
    type Output = Self;

    fn add(self, rhs: MultiBufferOffset) -> Self::Output {
        MultiBufferOffset(self.0 + rhs.0)
    }
}

impl ops::AddAssign<MultiBufferOffset> for MultiBufferOffset {
    fn add_assign(&mut self, other: MultiBufferOffset) {
        self.0 += other.0;
    }
}

pub trait ToOffset: 'static + fmt::Debug {
    fn to_offset(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffset;
    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffsetUtf16;
}

pub trait ToPoint: 'static + fmt::Debug {
    fn to_point(&self, snapshot: &MultiBufferSnapshot) -> Point;
    fn to_point_utf16(&self, snapshot: &MultiBufferSnapshot) -> PointUtf16;
}

struct BufferState {
    buffer: Entity<Buffer>,
    _subscriptions: [gpui::Subscription; 2],
}

struct DiffState {
    diff: Entity<BufferDiff>,
    main_buffer: Option<Entity<language::Buffer>>,
    _subscription: gpui::Subscription,
}

impl DiffState {
    fn snapshot(&self, buffer_id: BufferId, cx: &App) -> DiffStateSnapshot {
        DiffStateSnapshot {
            buffer_id,
            diff: self.diff.read(cx).snapshot(cx),
            main_buffer: self.main_buffer.as_ref().map(|b| b.read(cx).snapshot()),
        }
    }
}

#[derive(Clone)]
struct DiffStateSnapshot {
    buffer_id: BufferId,
    diff: BufferDiffSnapshot,
    main_buffer: Option<language::BufferSnapshot>,
}

impl std::ops::Deref for DiffStateSnapshot {
    type Target = BufferDiffSnapshot;

    fn deref(&self) -> &Self::Target {
        &self.diff
    }
}

#[derive(Clone, Debug, Default)]
struct DiffStateSummary {
    max_buffer_id: Option<BufferId>,
    added_rows: u32,
    removed_rows: u32,
}

impl sum_tree::ContextLessSummary for DiffStateSummary {
    fn zero() -> Self {
        Self::default()
    }

    fn add_summary(&mut self, other: &Self) {
        self.max_buffer_id = std::cmp::max(self.max_buffer_id, other.max_buffer_id);
        self.added_rows += other.added_rows;
        self.removed_rows += other.removed_rows;
    }
}

impl sum_tree::Item for DiffStateSnapshot {
    type Summary = DiffStateSummary;

    fn summary(&self, _cx: ()) -> DiffStateSummary {
        let (added_rows, removed_rows) = self.diff.changed_row_counts();
        DiffStateSummary {
            max_buffer_id: Some(self.buffer_id),
            added_rows,
            removed_rows,
        }
    }
}

impl sum_tree::KeyedItem for DiffStateSnapshot {
    type Key = Option<BufferId>;

    fn key(&self) -> Option<BufferId> {
        Some(self.buffer_id)
    }
}

impl<'a> Dimension<'a, DiffStateSummary> for Option<BufferId> {
    fn zero(_cx: ()) -> Self {
        None
    }

    fn add_summary(&mut self, summary: &DiffStateSummary, _cx: ()) {
        *self = std::cmp::max(*self, summary.max_buffer_id);
    }
}

fn find_diff_state(
    diffs: &SumTree<DiffStateSnapshot>,
    buffer_id: BufferId,
) -> Option<&DiffStateSnapshot> {
    let key = Some(buffer_id);
    let (.., item) = diffs.find::<Option<BufferId>, _>((), &key, Bias::Left);
    item.filter(|entry| entry.buffer_id == buffer_id)
}

fn remove_diff_state(diffs: &mut SumTree<DiffStateSnapshot>, buffer_id: BufferId) {
    let key = Some(buffer_id);
    let mut cursor = diffs.cursor::<Option<BufferId>>(());
    let mut new_tree = cursor.slice(&key, Bias::Left);
    if key == cursor.end() {
        cursor.next();
    }
    new_tree.append(cursor.suffix(), ());
    drop(cursor);
    *diffs = new_tree;
}

impl DiffState {
    fn new(diff: Entity<BufferDiff>, cx: &mut Context<MultiBuffer>) -> Self {
        DiffState {
            _subscription: cx.subscribe(&diff, |this, diff, event, cx| match event {
                BufferDiffEvent::DiffChanged(DiffChanged {
                    changed_range,
                    base_text_changed_range: _,
                    extended_range,
                }) => {
                    let use_extended = this.snapshot.borrow().use_extended_diff_range;
                    let range = if use_extended {
                        extended_range.clone()
                    } else {
                        changed_range.clone()
                    };
                    if let Some(range) = range {
                        this.buffer_diff_changed(diff, range, cx)
                    }
                    cx.emit(Event::BufferDiffChanged);
                }
                BufferDiffEvent::LanguageChanged => this.buffer_diff_language_changed(diff, cx),
                _ => {}
            }),
            diff,
            main_buffer: None,
        }
    }

    fn new_inverted(
        diff: Entity<BufferDiff>,
        main_buffer: Entity<language::Buffer>,
        cx: &mut Context<MultiBuffer>,
    ) -> Self {
        let weak_main_buffer = main_buffer.downgrade();
        DiffState {
            _subscription: cx.subscribe(&diff, {
                move |this, diff, event, cx| {
                    let Some(main_buffer) = weak_main_buffer.upgrade() else {
                        return;
                    };
                    match event {
                        BufferDiffEvent::DiffChanged(DiffChanged {
                            changed_range: _,
                            base_text_changed_range,
                            extended_range: _,
                        }) => {
                            this.inverted_buffer_diff_changed(
                                diff,
                                main_buffer,
                                base_text_changed_range.clone(),
                                cx,
                            );
                            cx.emit(Event::BufferDiffChanged);
                        }
                        BufferDiffEvent::LanguageChanged => {
                            this.inverted_buffer_diff_language_changed(diff, main_buffer, cx)
                        }
                        _ => {}
                    }
                }
            }),
            diff,
            main_buffer: Some(main_buffer),
        }
    }
}

#[derive(Clone)]
struct BufferStateSnapshot {
    path_key: PathKey,
    path_key_index: PathKeyIndex,
    buffer_snapshot: BufferSnapshot,
}

impl fmt::Debug for BufferStateSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufferStateSnapshot")
            .field("path_key", &self.path_key)
            .field("buffer_id", &self.buffer_snapshot.remote_id())
            .finish()
    }
}

/// The contents of a [`MultiBuffer`] at a single point in time.
#[derive(Clone, Default)]
pub struct MultiBufferSnapshot {
    excerpts: SumTree<Excerpt>,
    buffers: TreeMap<BufferId, BufferStateSnapshot>,
    path_keys_by_index: TreeMap<PathKeyIndex, PathKey>,
    indices_by_path_key: TreeMap<PathKey, PathKeyIndex>,
    diffs: SumTree<DiffStateSnapshot>,
    diff_transforms: SumTree<DiffTransform>,
    non_text_state_update_count: usize,
    edit_count: usize,
    is_dirty: bool,
    has_deleted_file: bool,
    has_conflict: bool,
    has_inverted_diff: bool,
    singleton: bool,
    trailing_excerpt_update_count: usize,
    all_diff_hunks_expanded: bool,
    show_deleted_hunks: bool,
    use_extended_diff_range: bool,
    show_headers: bool,
}

#[derive(Debug, Clone)]
enum DiffTransform {
    BufferContent {
        summary: MBTextSummary,
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
    buffer_id: BufferId,
    hunk_start_anchor: text::Anchor,
    hunk_secondary_status: DiffHunkSecondaryStatus,
    is_logically_deleted: bool,
    excerpt_end: ExcerptAnchor,
}

impl Eq for DiffTransformHunkInfo {}

impl PartialEq for DiffTransformHunkInfo {
    fn eq(&self, other: &DiffTransformHunkInfo) -> bool {
        self.buffer_id == other.buffer_id && self.hunk_start_anchor == other.hunk_start_anchor
    }
}

impl std::hash::Hash for DiffTransformHunkInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
        self.hunk_start_anchor.hash(state);
    }
}

#[derive(Clone)]
pub struct ExcerptBoundaryInfo {
    pub start_anchor: Anchor,
    pub range: ExcerptRange<text::Anchor>,
    pub end_row: MultiBufferRow,
}

impl ExcerptBoundaryInfo {
    pub fn start_text_anchor(&self) -> text::Anchor {
        self.range.context.start
    }
    pub fn buffer_id(&self) -> BufferId {
        self.start_text_anchor().buffer_id
    }
    pub fn buffer<'a>(&self, snapshot: &'a MultiBufferSnapshot) -> &'a BufferSnapshot {
        snapshot
            .buffer_for_id(self.buffer_id())
            .expect("buffer snapshot not found for excerpt boundary")
    }
}

impl std::fmt::Debug for ExcerptBoundaryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("buffer_id", &self.buffer_id())
            .field("range", &self.range)
            .finish()
    }
}

impl PartialEq for ExcerptBoundaryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.start_anchor == other.start_anchor && self.range == other.range
    }
}

impl Eq for ExcerptBoundaryInfo {}

/// A boundary between `Excerpt`s in a [`MultiBuffer`]
#[derive(Debug)]
pub struct ExcerptBoundary {
    pub prev: Option<ExcerptBoundaryInfo>,
    pub next: ExcerptBoundaryInfo,
    /// The row in the `MultiBuffer` where the boundary is located
    pub row: MultiBufferRow,
}

impl ExcerptBoundary {
    pub fn starts_new_buffer(&self) -> bool {
        match (self.prev.as_ref(), &self.next) {
            (None, _) => true,
            (Some(prev), next) => prev.buffer_id() != next.buffer_id(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExpandInfo {
    pub direction: ExpandExcerptDirection,
    pub start_anchor: Anchor,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct RowInfo {
    pub buffer_id: Option<BufferId>,
    pub buffer_row: Option<u32>,
    pub multibuffer_row: Option<MultiBufferRow>,
    pub diff_status: Option<buffer_diff::DiffHunkStatus>,
    pub expand_info: Option<ExpandInfo>,
    pub wrapped_buffer_row: Option<u32>,
}

/// A slice into a [`Buffer`] that is being edited in a [`MultiBuffer`].
#[derive(Clone, Debug)]
pub(crate) struct Excerpt {
    /// The location of the excerpt in the [`MultiBuffer`]
    pub(crate) path_key: PathKey,
    pub(crate) path_key_index: PathKeyIndex,
    pub(crate) buffer_id: BufferId,
    /// The range of the buffer to be shown in the excerpt
    pub(crate) range: ExcerptRange<text::Anchor>,

    /// The last row in the excerpted slice of the buffer
    pub(crate) max_buffer_row: BufferRow,
    /// A summary of the text in the excerpt
    pub(crate) text_summary: TextSummary,
    pub(crate) has_trailing_newline: bool,
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

impl ExcerptRange<text::Anchor> {
    pub fn contains(&self, t: &text::Anchor, snapshot: &BufferSnapshot) -> bool {
        self.context.start.cmp(t, snapshot).is_le() && self.context.end.cmp(t, snapshot).is_ge()
    }
}

#[derive(Clone, Debug)]
pub struct ExcerptSummary {
    path_key: PathKey,
    max_anchor: Option<text::Anchor>,
    widest_line_number: u32,
    text: MBTextSummary,
    count: usize,
}

impl ExcerptSummary {
    pub fn min() -> Self {
        ExcerptSummary {
            path_key: PathKey::min(),
            max_anchor: None,
            widest_line_number: 0,
            text: MBTextSummary::default(),
            count: 0,
        }
    }

    fn len(&self) -> ExcerptOffset {
        ExcerptDimension(self.text.len)
    }
}

#[derive(Debug, Clone)]
pub struct DiffTransformSummary {
    input: MBTextSummary,
    output: MBTextSummary,
}

/// Summary of a string of text.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct MBTextSummary {
    /// Length in bytes.
    pub len: MultiBufferOffset,
    /// Length in UTF-8.
    pub chars: usize,
    /// Length in UTF-16 code units
    pub len_utf16: OffsetUtf16,
    /// A point representing the number of lines and the length of the last line.
    ///
    /// In other words, it marks the point after the last byte in the text, (if
    /// EOF was a character, this would be its position).
    pub lines: Point,
    /// How many `char`s are in the first line
    pub first_line_chars: u32,
    /// How many `char`s are in the last line
    pub last_line_chars: u32,
    /// How many UTF-16 code units are in the last line
    pub last_line_len_utf16: u32,
    /// The row idx of the longest row
    pub longest_row: u32,
    /// How many `char`s are in the longest row
    pub longest_row_chars: u32,
}

impl From<TextSummary> for MBTextSummary {
    fn from(summary: TextSummary) -> Self {
        MBTextSummary {
            len: MultiBufferOffset(summary.len),
            chars: summary.chars,
            len_utf16: summary.len_utf16,
            lines: summary.lines,
            first_line_chars: summary.first_line_chars,
            last_line_chars: summary.last_line_chars,
            last_line_len_utf16: summary.last_line_len_utf16,
            longest_row: summary.longest_row,
            longest_row_chars: summary.longest_row_chars,
        }
    }
}

impl From<MBTextSummary> for TextSummary {
    fn from(summary: MBTextSummary) -> Self {
        TextSummary {
            len: summary.len.0,
            chars: summary.chars,
            len_utf16: summary.len_utf16,
            lines: summary.lines,
            first_line_chars: summary.first_line_chars,
            last_line_chars: summary.last_line_chars,
            last_line_len_utf16: summary.last_line_len_utf16,
            longest_row: summary.longest_row,
            longest_row_chars: summary.longest_row_chars,
        }
    }
}

impl From<&str> for MBTextSummary {
    fn from(text: &str) -> Self {
        MBTextSummary::from(TextSummary::from(text))
    }
}

impl MultiBufferDimension for MBTextSummary {
    type TextDimension = TextSummary;

    fn from_summary(summary: &MBTextSummary) -> Self {
        *summary
    }

    fn add_text_dim(&mut self, summary: &Self::TextDimension) {
        *self += *summary;
    }

    fn add_mb_text_summary(&mut self, summary: &MBTextSummary) {
        *self += *summary;
    }
}

impl AddAssign for MBTextSummary {
    fn add_assign(&mut self, other: MBTextSummary) {
        let joined_chars = self.last_line_chars + other.first_line_chars;
        if joined_chars > self.longest_row_chars {
            self.longest_row = self.lines.row;
            self.longest_row_chars = joined_chars;
        }
        if other.longest_row_chars > self.longest_row_chars {
            self.longest_row = self.lines.row + other.longest_row;
            self.longest_row_chars = other.longest_row_chars;
        }

        if self.lines.row == 0 {
            self.first_line_chars += other.first_line_chars;
        }

        if other.lines.row == 0 {
            self.last_line_chars += other.first_line_chars;
            self.last_line_len_utf16 += other.last_line_len_utf16;
        } else {
            self.last_line_chars = other.last_line_chars;
            self.last_line_len_utf16 = other.last_line_len_utf16;
        }

        self.chars += other.chars;
        self.len += other.len;
        self.len_utf16 += other.len_utf16;
        self.lines += other.lines;
    }
}

impl AddAssign<TextSummary> for MBTextSummary {
    fn add_assign(&mut self, other: TextSummary) {
        *self += MBTextSummary::from(other);
    }
}

impl MBTextSummary {
    pub fn lines_utf16(&self) -> PointUtf16 {
        PointUtf16 {
            row: self.lines.row,
            column: self.last_line_len_utf16,
        }
    }
}

impl<K, V> MultiBufferDimension for DimensionPair<K, V>
where
    K: MultiBufferDimension,
    V: MultiBufferDimension,
{
    type TextDimension = DimensionPair<K::TextDimension, V::TextDimension>;

    fn from_summary(summary: &MBTextSummary) -> Self {
        Self {
            key: K::from_summary(summary),
            value: Some(V::from_summary(summary)),
        }
    }

    fn add_text_dim(&mut self, summary: &Self::TextDimension) {
        self.key.add_text_dim(&summary.key);
        if let Some(value) = &mut self.value {
            if let Some(other_value) = summary.value.as_ref() {
                value.add_text_dim(other_value);
            }
        }
    }

    fn add_mb_text_summary(&mut self, summary: &MBTextSummary) {
        self.key.add_mb_text_summary(summary);
        if let Some(value) = &mut self.value {
            value.add_mb_text_summary(summary);
        }
    }
}

#[derive(Clone)]
pub struct MultiBufferRows<'a> {
    point: Point,
    is_empty: bool,
    is_singleton: bool,
    cursor: MultiBufferCursor<'a, Point, Point>,
}

pub struct MultiBufferChunks<'a> {
    excerpts: Cursor<'a, 'static, Excerpt, ExcerptOffset>,
    diff_transforms:
        Cursor<'a, 'static, DiffTransform, Dimensions<MultiBufferOffset, ExcerptOffset>>,
    diff_base_chunks: Option<(BufferId, BufferChunks<'a>)>,
    buffer_chunk: Option<Chunk<'a>>,
    range: Range<MultiBufferOffset>,
    excerpt_offset_range: Range<ExcerptOffset>,
    excerpt_chunks: Option<ExcerptChunks<'a>>,
    language_aware: LanguageAwareStyling,
    snapshot: &'a MultiBufferSnapshot,
}

pub struct ReversedMultiBufferChunks<'a> {
    cursor: MultiBufferCursor<'a, MultiBufferOffset, BufferOffset>,
    current_chunks: Option<rope::Chunks<'a>>,
    start: MultiBufferOffset,
    offset: MultiBufferOffset,
}

pub struct MultiBufferBytes<'a> {
    range: Range<MultiBufferOffset>,
    cursor: MultiBufferCursor<'a, MultiBufferOffset, BufferOffset>,
    excerpt_bytes: Option<text::Bytes<'a>>,
    has_trailing_newline: bool,
    chunk: &'a [u8],
}

pub struct ReversedMultiBufferBytes<'a> {
    range: Range<MultiBufferOffset>,
    chunks: ReversedMultiBufferChunks<'a>,
    chunk: &'a [u8],
}

#[derive(Clone)]
struct DiffTransforms<MBD> {
    output_dimension: OutputDimension<MBD>,
    excerpt_dimension: ExcerptDimension<MBD>,
}

impl<'a, MBD: MultiBufferDimension> Dimension<'a, DiffTransformSummary> for DiffTransforms<MBD> {
    fn zero(cx: <DiffTransformSummary as sum_tree::Summary>::Context<'_>) -> Self {
        Self {
            output_dimension: OutputDimension::zero(cx),
            excerpt_dimension: <ExcerptDimension<MBD> as Dimension<'a, DiffTransformSummary>>::zero(
                cx,
            ),
        }
    }

    fn add_summary(
        &mut self,
        summary: &'a DiffTransformSummary,
        cx: <DiffTransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        self.output_dimension.add_summary(summary, cx);
        self.excerpt_dimension.add_summary(summary, cx);
    }
}

#[derive(Clone)]
struct MultiBufferCursor<'a, MBD, BD> {
    excerpts: Cursor<'a, 'static, Excerpt, ExcerptDimension<MBD>>,
    diff_transforms: Cursor<'a, 'static, DiffTransform, DiffTransforms<MBD>>,
    cached_region: OnceCell<Option<MultiBufferRegion<'a, MBD, BD>>>,
    snapshot: &'a MultiBufferSnapshot,
}

#[derive(Clone)]
struct MultiBufferRegion<'a, MBD, BD> {
    buffer: &'a BufferSnapshot,
    is_main_buffer: bool,
    diff_hunk_status: Option<DiffHunkStatus>,
    excerpt: &'a Excerpt,
    buffer_range: Range<BD>,
    range: Range<MBD>,
    has_trailing_newline: bool,
}

struct ExcerptChunks<'a> {
    content_chunks: BufferChunks<'a>,
    end: ExcerptAnchor,
    has_footer: bool,
}

#[derive(Debug)]
struct BufferEdit {
    range: Range<BufferOffset>,
    new_text: Arc<str>,
    is_insertion: bool,
    original_indent_column: Option<u32>,
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
        Self::new_(
            capability,
            MultiBufferSnapshot {
                show_headers: true,
                show_deleted_hunks: true,
                ..MultiBufferSnapshot::default()
            },
        )
    }

    pub fn without_headers(capability: Capability) -> Self {
        Self::new_(
            capability,
            MultiBufferSnapshot {
                show_deleted_hunks: true,
                ..MultiBufferSnapshot::default()
            },
        )
    }

    pub fn singleton(buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Self {
        let mut this = Self::new_(
            buffer.read(cx).capability(),
            MultiBufferSnapshot {
                singleton: true,
                show_deleted_hunks: true,
                ..MultiBufferSnapshot::default()
            },
        );
        this.singleton = true;
        this.set_excerpts_for_path(
            PathKey::sorted(0),
            buffer.clone(),
            [Point::zero()..buffer.read(cx).max_point()],
            0,
            cx,
        );
        this
    }

    #[inline]
    pub fn new_(capability: Capability, snapshot: MultiBufferSnapshot) -> Self {
        Self {
            snapshot: RefCell::new(snapshot),
            buffers: Default::default(),
            diffs: HashMap::default(),
            subscriptions: Topic::default(),
            singleton: false,
            capability,
            title: None,
            buffer_changed_since_sync: Default::default(),
            history: History::default(),
        }
    }

    pub fn clone(&self, new_cx: &mut Context<Self>) -> Self {
        let mut buffers = BTreeMap::default();
        let buffer_changed_since_sync = Rc::new(Cell::new(false));
        for (buffer_id, buffer_state) in self.buffers.iter() {
            buffer_state.buffer.update(new_cx, |buffer, _| {
                buffer.record_changes(Rc::downgrade(&buffer_changed_since_sync));
            });
            buffers.insert(
                *buffer_id,
                BufferState {
                    buffer: buffer_state.buffer.clone(),
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
            buffers,
            diffs: diff_bases,
            subscriptions: Default::default(),
            singleton: self.singleton,
            capability: self.capability,
            history: self.history.clone(),
            title: self.title.clone(),
            buffer_changed_since_sync,
        }
    }

    pub fn set_group_interval(&mut self, group_interval: Duration, cx: &mut Context<Self>) {
        self.history.set_group_interval(group_interval);
        if self.singleton {
            for BufferState { buffer, .. } in self.buffers.values() {
                buffer.update(cx, |buffer, _| {
                    buffer.set_group_interval(group_interval);
                });
            }
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn read_only(&self) -> bool {
        !self.capability.editable()
    }

    pub fn capability(&self) -> Capability {
        self.capability
    }

    /// Returns an up-to-date snapshot of the MultiBuffer.
    #[ztracing::instrument(skip_all)]
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
            Some(self.buffers.values().next().unwrap().buffer.clone())
        } else {
            None
        }
    }

    pub fn is_singleton(&self) -> bool {
        self.singleton
    }

    pub fn subscribe(&mut self) -> Subscription<MultiBufferOffset> {
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
    pub fn len(&self, cx: &App) -> MultiBufferOffset {
        self.read(cx).len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn edit<I, S, T>(
        &mut self,
        edits: I,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut Context<Self>,
    ) where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        self.edit_internal(edits, autoindent_mode, true, cx);
    }

    pub fn edit_non_coalesce<I, S, T>(
        &mut self,
        edits: I,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut Context<Self>,
    ) where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        self.edit_internal(edits, autoindent_mode, false, cx);
    }

    fn edit_internal<I, S, T>(
        &mut self,
        edits: I,
        autoindent_mode: Option<AutoindentMode>,
        coalesce_adjacent: bool,
        cx: &mut Context<Self>,
    ) where
        I: IntoIterator<Item = (Range<S>, T)>,
        S: ToOffset,
        T: Into<Arc<str>>,
    {
        if self.read_only() || self.buffers.is_empty() {
            return;
        }
        self.sync_mut(cx);
        let edits = edits
            .into_iter()
            .map(|(range, new_text)| {
                let mut range = range.start.to_offset(self.snapshot.get_mut())
                    ..range.end.to_offset(self.snapshot.get_mut());
                if range.start > range.end {
                    mem::swap(&mut range.start, &mut range.end);
                }
                (range, new_text.into())
            })
            .collect::<Vec<_>>();

        return edit_internal(self, edits, autoindent_mode, coalesce_adjacent, cx);

        // Non-generic part of edit, hoisted out to avoid blowing up LLVM IR.
        fn edit_internal(
            this: &mut MultiBuffer,
            edits: Vec<(Range<MultiBufferOffset>, Arc<str>)>,
            mut autoindent_mode: Option<AutoindentMode>,
            coalesce_adjacent: bool,
            cx: &mut Context<MultiBuffer>,
        ) {
            let original_indent_columns = match &mut autoindent_mode {
                Some(AutoindentMode::Block {
                    original_indent_columns,
                }) => mem::take(original_indent_columns),
                _ => Default::default(),
            };

            let buffer_edits = MultiBuffer::convert_edits_to_buffer_edits(
                edits,
                this.snapshot.get_mut(),
                &original_indent_columns,
            );

            let mut buffer_ids = Vec::with_capacity(buffer_edits.len());
            for (buffer_id, mut edits) in buffer_edits {
                buffer_ids.push(buffer_id);
                edits.sort_by_key(|edit| edit.range.start);
                this.buffers[&buffer_id].buffer.update(cx, |buffer, cx| {
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
                    }) = edits.next()
                    {
                        while let Some(BufferEdit {
                            range: next_range,
                            is_insertion: next_is_insertion,
                            new_text: next_new_text,
                            ..
                        }) = edits.peek()
                        {
                            let should_coalesce = if coalesce_adjacent {
                                range.end >= next_range.start
                            } else {
                                range.end > next_range.start
                            };

                            if should_coalesce {
                                range.end = cmp::max(next_range.end, range.end);
                                is_insertion |= *next_is_insertion;
                                new_text = format!("{new_text}{next_new_text}").into();
                                edits.next();
                            } else {
                                break;
                            }
                        }

                        if is_insertion {
                            original_indent_columns.push(original_indent_column);
                            insertions.push((
                                buffer.anchor_before(range.start)..buffer.anchor_before(range.end),
                                new_text.clone(),
                            ));
                        } else if !range.is_empty() {
                            deletions.push((
                                buffer.anchor_before(range.start)..buffer.anchor_before(range.end),
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

                    if coalesce_adjacent {
                        buffer.edit(deletions, deletion_autoindent_mode, cx);
                        buffer.edit(insertions, insertion_autoindent_mode, cx);
                    } else {
                        buffer.edit_non_coalesce(deletions, deletion_autoindent_mode, cx);
                        buffer.edit_non_coalesce(insertions, insertion_autoindent_mode, cx);
                    }
                })
            }

            cx.emit(Event::BuffersEdited { buffer_ids });
        }
    }

    fn convert_edits_to_buffer_edits(
        edits: Vec<(Range<MultiBufferOffset>, Arc<str>)>,
        snapshot: &MultiBufferSnapshot,
        original_indent_columns: &[Option<u32>],
    ) -> HashMap<BufferId, Vec<BufferEdit>> {
        let mut buffer_edits: HashMap<BufferId, Vec<BufferEdit>> = Default::default();
        let mut cursor = snapshot.cursor::<MultiBufferOffset, BufferOffset>();
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

            let start_region = start_region.clone();
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

            if start_region.excerpt == end_region.excerpt {
                if start_region.buffer.capability == Capability::ReadWrite
                    && start_region.is_main_buffer
                {
                    buffer_edits
                        .entry(start_region.buffer.remote_id())
                        .or_default()
                        .push(BufferEdit {
                            range: buffer_start..buffer_end,
                            new_text,
                            is_insertion: true,
                            original_indent_column,
                        });
                }
            } else {
                let start_excerpt_range = buffer_start..start_region.buffer_range.end;
                let end_excerpt_range = end_region.buffer_range.start..buffer_end;
                if start_region.buffer.capability == Capability::ReadWrite
                    && start_region.is_main_buffer
                {
                    buffer_edits
                        .entry(start_region.buffer.remote_id())
                        .or_default()
                        .push(BufferEdit {
                            range: start_excerpt_range,
                            new_text: new_text.clone(),
                            is_insertion: true,
                            original_indent_column,
                        });
                }
                if end_region.buffer.capability == Capability::ReadWrite
                    && end_region.is_main_buffer
                {
                    buffer_edits
                        .entry(end_region.buffer.remote_id())
                        .or_default()
                        .push(BufferEdit {
                            range: end_excerpt_range,
                            new_text: new_text.clone(),
                            is_insertion: false,
                            original_indent_column,
                        });
                }
                let end_region_excerpt = end_region.excerpt.clone();

                cursor.seek(&range.start);
                cursor.next_excerpt();
                while let Some(region) = cursor.region() {
                    if region.excerpt == &end_region_excerpt {
                        break;
                    }
                    if region.buffer.capability == Capability::ReadWrite && region.is_main_buffer {
                        buffer_edits
                            .entry(region.buffer.remote_id())
                            .or_default()
                            .push(BufferEdit {
                                range: region.buffer_range.clone(),
                                new_text: new_text.clone(),
                                is_insertion: false,
                                original_indent_column,
                            });
                    }
                    cursor.next_excerpt();
                }
            }
        }
        buffer_edits
    }

    pub fn autoindent_ranges<I, S>(&mut self, ranges: I, cx: &mut Context<Self>)
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
    {
        if self.read_only() || self.buffers.is_empty() {
            return;
        }
        self.sync_mut(cx);
        let empty = Arc::<str>::from("");
        let edits = ranges
            .into_iter()
            .map(|range| {
                let mut range = range.start.to_offset(self.snapshot.get_mut())
                    ..range.end.to_offset(&self.snapshot.get_mut());
                if range.start > range.end {
                    mem::swap(&mut range.start, &mut range.end);
                }
                (range, empty.clone())
            })
            .collect::<Vec<_>>();

        return autoindent_ranges_internal(self, edits, cx);

        fn autoindent_ranges_internal(
            this: &mut MultiBuffer,
            edits: Vec<(Range<MultiBufferOffset>, Arc<str>)>,
            cx: &mut Context<MultiBuffer>,
        ) {
            let buffer_edits =
                MultiBuffer::convert_edits_to_buffer_edits(edits, this.snapshot.get_mut(), &[]);

            let mut buffer_ids = Vec::new();
            for (buffer_id, mut edits) in buffer_edits {
                buffer_ids.push(buffer_id);
                edits.sort_unstable_by_key(|edit| edit.range.start);

                let mut ranges: Vec<Range<BufferOffset>> = Vec::new();
                for edit in edits {
                    if let Some(last_range) = ranges.last_mut()
                        && edit.range.start <= last_range.end
                    {
                        last_range.end = last_range.end.max(edit.range.end);
                        continue;
                    }
                    ranges.push(edit.range);
                }

                this.buffers[&buffer_id].buffer.update(cx, |buffer, cx| {
                    buffer.autoindent_ranges(ranges, cx);
                })
            }

            cx.emit(Event::BuffersEdited { buffer_ids });
        }
    }

    pub fn set_active_selections(
        &self,
        selections: &[Selection<Anchor>],
        line_mode: bool,
        cursor_shape: CursorShape,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot(cx);
        let mut selections_by_buffer: HashMap<BufferId, Vec<Selection<text::Anchor>>> =
            Default::default();

        for selection in selections {
            for (buffer_snapshot, buffer_range, _) in
                snapshot.range_to_buffer_ranges(selection.start..selection.end)
            {
                selections_by_buffer
                    .entry(buffer_snapshot.remote_id())
                    .or_default()
                    .push(Selection {
                        id: selection.id,
                        start: buffer_snapshot
                            .anchor_at(buffer_range.start, selection.start.bias()),
                        end: buffer_snapshot.anchor_at(buffer_range.end, selection.end.bias()),
                        reversed: selection.reversed,
                        goal: selection.goal,
                    });
            }
        }

        for (buffer_id, buffer_state) in self.buffers.iter() {
            if !selections_by_buffer.contains_key(buffer_id) {
                buffer_state
                    .buffer
                    .update(cx, |buffer, cx| buffer.remove_active_selections(cx));
            }
        }

        for (buffer_id, selections) in selections_by_buffer {
            self.buffers[&buffer_id].buffer.update(cx, |buffer, cx| {
                buffer.set_active_selections(selections.into(), line_mode, cursor_shape, cx);
            });
        }
    }

    pub fn remove_active_selections(&self, cx: &mut Context<Self>) {
        for buffer in self.buffers.values() {
            buffer
                .buffer
                .update(cx, |buffer, cx| buffer.remove_active_selections(cx));
        }
    }

    #[instrument(skip_all)]
    fn merge_excerpt_ranges<'a>(
        expanded_ranges: impl IntoIterator<Item = &'a ExcerptRange<Point>> + 'a,
    ) -> Vec<ExcerptRange<Point>> {
        let mut sorted: Vec<_> = expanded_ranges.into_iter().collect();
        sorted.sort_by_key(|range| range.context.start);
        let mut merged_ranges: Vec<ExcerptRange<Point>> = Vec::new();
        for range in sorted {
            if let Some(last_range) = merged_ranges.last_mut() {
                if last_range.context.end >= range.context.start
                    || last_range.context.end.row + 1 == range.context.start.row
                {
                    last_range.context.end = range.context.end.max(last_range.context.end);
                    continue;
                }
            }
            merged_ranges.push(range.clone());
        }
        merged_ranges
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.sync_mut(cx);
        let removed_buffer_ids = std::mem::take(&mut self.buffers).into_keys().collect();
        self.diffs.clear();
        let MultiBufferSnapshot {
            excerpts,
            diffs,
            diff_transforms: _,
            non_text_state_update_count: _,
            edit_count: _,
            is_dirty,
            has_deleted_file,
            has_conflict,
            has_inverted_diff,
            singleton: _,
            trailing_excerpt_update_count,
            all_diff_hunks_expanded: _,
            show_deleted_hunks: _,
            use_extended_diff_range: _,
            show_headers: _,
            path_keys_by_index: _,
            indices_by_path_key: _,
            buffers,
        } = self.snapshot.get_mut();
        let start = ExcerptDimension(MultiBufferOffset::ZERO);
        let prev_len = ExcerptDimension(excerpts.summary().text.len);
        *excerpts = Default::default();
        *buffers = Default::default();
        *diffs = Default::default();
        *trailing_excerpt_update_count += 1;
        *is_dirty = false;
        *has_deleted_file = false;
        *has_conflict = false;
        *has_inverted_diff = false;

        let edits = Self::sync_diff_transforms(
            self.snapshot.get_mut(),
            vec![Edit {
                old: start..prev_len,
                new: start..start,
            }],
            DiffChangeKind::BufferEdited,
        );
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }
        cx.emit(Event::Edited {
            edited_buffer: None,
            is_local: true,
        });
        cx.emit(Event::BuffersRemoved { removed_buffer_ids });
        cx.notify();
    }

    // If point is at the end of the buffer, the last excerpt is returned
    pub fn point_to_buffer_offset<T: ToOffset>(
        &self,
        point: T,
        cx: &App,
    ) -> Option<(Entity<Buffer>, BufferOffset)> {
        let snapshot = self.read(cx);
        let (buffer, offset) = snapshot.point_to_buffer_offset(point)?;
        Some((
            self.buffers.get(&buffer.remote_id())?.buffer.clone(),
            offset,
        ))
    }

    // If point is at the end of the buffer, the last excerpt is returned
    pub fn point_to_buffer_point<T: ToPoint>(
        &self,
        point: T,
        cx: &App,
    ) -> Option<(Entity<Buffer>, Point)> {
        let snapshot = self.read(cx);
        let (buffer, point) = snapshot.point_to_buffer_point(point.to_point(&snapshot))?;
        Some((self.buffers.get(&buffer.remote_id())?.buffer.clone(), point))
    }

    pub fn buffer_point_to_anchor(
        &self,
        // todo(lw): We shouldn't need this?
        buffer: &Entity<Buffer>,
        point: Point,
        cx: &App,
    ) -> Option<Anchor> {
        let mut found = None;
        let buffer_snapshot = buffer.read(cx).snapshot();
        let text_anchor = buffer_snapshot.anchor_after(&point);
        let snapshot = self.snapshot(cx);
        let path_key_index = snapshot.path_key_index_for_buffer(buffer_snapshot.remote_id())?;
        for excerpt in snapshot.excerpts_for_buffer(buffer_snapshot.remote_id()) {
            if excerpt
                .context
                .start
                .cmp(&text_anchor, &buffer_snapshot)
                .is_gt()
            {
                found = Some(Anchor::in_buffer(path_key_index, excerpt.context.start));
                break;
            } else if excerpt
                .context
                .end
                .cmp(&text_anchor, &buffer_snapshot)
                .is_ge()
            {
                found = Some(Anchor::in_buffer(path_key_index, text_anchor));
                break;
            }
            found = Some(Anchor::in_buffer(path_key_index, excerpt.context.end));
        }

        found
    }

    pub fn wait_for_anchors<'a, Anchors: 'a + Iterator<Item = Anchor>>(
        &self,
        anchors: Anchors,
        cx: &mut Context<Self>,
    ) -> impl 'static + Future<Output = Result<()>> + use<Anchors> {
        let mut error = None;
        let mut futures = Vec::new();
        for anchor in anchors {
            if let Some(excerpt_anchor) = anchor.excerpt_anchor() {
                if let Some(buffer) = self.buffers.get(&excerpt_anchor.text_anchor.buffer_id) {
                    buffer.buffer.update(cx, |buffer, _| {
                        futures.push(buffer.wait_for_anchors([excerpt_anchor.text_anchor()]))
                    });
                } else {
                    error = Some(anyhow!(
                        "buffer {:?} is not part of this multi-buffer",
                        excerpt_anchor.text_anchor.buffer_id
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
    ) -> Option<(Entity<Buffer>, text::Anchor)> {
        let snapshot = self.read(cx);
        let anchor = snapshot.anchor_before(position).excerpt_anchor()?;
        let buffer = self
            .buffers
            .get(&anchor.text_anchor.buffer_id)?
            .buffer
            .clone();
        Some((buffer, anchor.text_anchor()))
    }

    fn on_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &language::BufferEvent,
        cx: &mut Context<Self>,
    ) {
        use language::BufferEvent;
        let buffer_id = buffer.read(cx).remote_id();
        cx.emit(match event {
            &BufferEvent::Edited { is_local } => Event::Edited {
                edited_buffer: Some(buffer),
                is_local,
            },
            BufferEvent::DirtyChanged => Event::DirtyChanged,
            BufferEvent::Saved => Event::Saved,
            BufferEvent::FileHandleChanged => Event::FileHandleChanged,
            BufferEvent::Reloaded => Event::Reloaded,
            BufferEvent::LanguageChanged(has_language) => {
                Event::LanguageChanged(buffer_id, *has_language)
            }
            BufferEvent::Reparsed => Event::Reparsed(buffer_id),
            BufferEvent::DiagnosticsUpdated => Event::DiagnosticsUpdated,
            BufferEvent::CapabilityChanged => {
                self.capability = buffer.read(cx).capability();
                return;
            }
            BufferEvent::Operation { .. } | BufferEvent::ReloadNeeded => return,
        });
    }

    fn buffer_diff_language_changed(&mut self, diff: Entity<BufferDiff>, cx: &mut Context<Self>) {
        let diff = diff.read(cx);
        let buffer_id = diff.buffer_id;
        let diff = DiffStateSnapshot {
            buffer_id,
            diff: diff.snapshot(cx),
            main_buffer: None,
        };
        self.snapshot.get_mut().diffs.insert_or_replace(diff, ());
    }

    fn inverted_buffer_diff_language_changed(
        &mut self,
        diff: Entity<BufferDiff>,
        main_buffer: Entity<language::Buffer>,
        cx: &mut Context<Self>,
    ) {
        let base_text_buffer_id = diff.read(cx).base_text_buffer().read(cx).remote_id();
        let main_buffer_snapshot = main_buffer.read(cx).snapshot();
        let diff = diff.read(cx);
        let diff = DiffStateSnapshot {
            buffer_id: base_text_buffer_id,
            diff: diff.snapshot(cx),
            main_buffer: Some(main_buffer_snapshot),
        };
        self.snapshot.get_mut().diffs.insert_or_replace(diff, ());
    }

    fn buffer_diff_changed(
        &mut self,
        diff: Entity<BufferDiff>,
        range: Range<text::Anchor>,
        cx: &mut Context<Self>,
    ) {
        let Some(buffer) = self.buffer(diff.read(cx).buffer_id) else {
            return;
        };
        let snapshot = self.sync_mut(cx);

        let diff = diff.read(cx);
        let buffer_id = diff.buffer_id;

        let Some(path) = snapshot.path_for_buffer(buffer_id).cloned() else {
            return;
        };
        let new_diff = DiffStateSnapshot {
            buffer_id,
            diff: diff.snapshot(cx),
            main_buffer: None,
        };
        let snapshot = self.snapshot.get_mut();
        let base_text_changed = find_diff_state(&snapshot.diffs, buffer_id)
            .is_none_or(|old_diff| !new_diff.base_texts_definitely_eq(old_diff));
        snapshot.diffs.insert_or_replace(new_diff, ());

        let buffer = buffer.read(cx);
        let diff_change_range = range.to_offset(buffer);

        let excerpt_edits = snapshot.excerpt_edits_for_diff_change(&path, diff_change_range);
        let edits = Self::sync_diff_transforms(
            snapshot,
            excerpt_edits,
            DiffChangeKind::DiffUpdated {
                base_changed: base_text_changed,
            },
        );
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }
        cx.emit(Event::Edited {
            edited_buffer: None,
            is_local: true,
        });
    }

    fn inverted_buffer_diff_changed(
        &mut self,
        diff: Entity<BufferDiff>,
        main_buffer: Entity<language::Buffer>,
        diff_change_range: Option<Range<usize>>,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.sync_mut(cx);

        let base_text_buffer_id = diff.read(cx).base_text_buffer().read(cx).remote_id();
        let Some(path) = snapshot.path_for_buffer(base_text_buffer_id).cloned() else {
            return;
        };

        let main_buffer_snapshot = main_buffer.read(cx).snapshot();
        let diff = diff.read(cx);
        let new_diff = DiffStateSnapshot {
            buffer_id: base_text_buffer_id,
            diff: diff.snapshot(cx),
            main_buffer: Some(main_buffer_snapshot),
        };
        let snapshot = self.snapshot.get_mut();
        snapshot.diffs.insert_or_replace(new_diff, ());

        let Some(diff_change_range) = diff_change_range else {
            return;
        };

        let excerpt_edits = snapshot.excerpt_edits_for_diff_change(&path, diff_change_range);
        let edits = Self::sync_diff_transforms(
            snapshot,
            excerpt_edits,
            DiffChangeKind::DiffUpdated {
                // We don't read this field for inverted diffs.
                base_changed: false,
            },
        );
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }
        cx.emit(Event::Edited {
            edited_buffer: None,
            is_local: true,
        });
    }

    pub fn all_buffers_iter(&self) -> impl Iterator<Item = Entity<Buffer>> {
        self.buffers.values().map(|state| state.buffer.clone())
    }

    pub fn all_buffers(&self) -> HashSet<Entity<Buffer>> {
        self.all_buffers_iter().collect()
    }

    pub fn buffer(&self, buffer_id: BufferId) -> Option<Entity<Buffer>> {
        self.buffers
            .get(&buffer_id)
            .map(|state| state.buffer.clone())
    }

    pub fn language_at<T: ToOffset>(&self, point: T, cx: &App) -> Option<Arc<Language>> {
        self.point_to_buffer_offset(point, cx)
            .and_then(|(buffer, offset)| buffer.read(cx).language_at(offset))
    }

    pub fn language_settings<'a>(&'a self, cx: &'a App) -> Cow<'a, LanguageSettings> {
        let snapshot = self.snapshot(cx);
        snapshot
            .excerpts
            .first()
            .and_then(|excerpt| self.buffer(excerpt.range.context.start.buffer_id))
            .map(|buffer| LanguageSettings::for_buffer(&buffer.read(cx), cx))
            .unwrap_or_else(move || self.language_settings_at(MultiBufferOffset::default(), cx))
    }

    pub fn language_settings_at<'a, T: ToOffset>(
        &'a self,
        point: T,
        cx: &'a App,
    ) -> Cow<'a, LanguageSettings> {
        if let Some((buffer, offset)) = self.point_to_buffer_offset(point, cx) {
            LanguageSettings::for_buffer_at(buffer.read(cx), offset, cx)
        } else {
            Cow::Borrowed(&AllLanguageSettings::get_global(cx).defaults)
        }
    }

    pub fn for_each_buffer(&self, f: &mut dyn FnMut(&Entity<Buffer>)) {
        self.buffers.values().for_each(|state| f(&state.buffer))
    }

    pub fn explicit_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn title<'a>(&'a self, cx: &'a App) -> Cow<'a, str> {
        if let Some(title) = self.title.as_ref() {
            return title.into();
        }

        if let Some(buffer) = self.as_singleton() {
            let buffer = buffer.read(cx);

            if let Some(file) = buffer.file() {
                return file.file_name(cx).into();
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
        for buffer_state in self.buffers.values() {
            buffer_state
                .buffer
                .update(cx, |buffer, _cx| buffer.refresh_preview());
        }
    }

    /// Whether we should preserve the preview status of a tab containing this multi-buffer.
    pub fn preserve_preview(&self, cx: &App) -> bool {
        self.buffers
            .values()
            .all(|state| state.buffer.read(cx).preserve_preview())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn is_parsing(&self, cx: &App) -> bool {
        self.as_singleton().unwrap().read(cx).is_parsing()
    }

    pub fn add_diff(&mut self, diff: Entity<BufferDiff>, cx: &mut Context<Self>) {
        let buffer_id = diff.read(cx).buffer_id;

        if let Some(existing_diff) = self.diff_for(buffer_id)
            && diff.entity_id() == existing_diff.entity_id()
        {
            return;
        }

        self.buffer_diff_changed(
            diff.clone(),
            text::Anchor::min_max_range_for_buffer(buffer_id),
            cx,
        );
        self.diffs.insert(buffer_id, DiffState::new(diff, cx));
    }

    pub fn add_inverted_diff(
        &mut self,
        diff: Entity<BufferDiff>,
        main_buffer: Entity<language::Buffer>,
        cx: &mut Context<Self>,
    ) {
        let snapshot = diff.read(cx).base_text(cx);
        let base_text_buffer_id = snapshot.remote_id();
        let diff_change_range = 0..snapshot.len();
        self.snapshot.get_mut().has_inverted_diff = true;
        self.inverted_buffer_diff_changed(
            diff.clone(),
            main_buffer.clone(),
            Some(diff_change_range),
            cx,
        );
        self.diffs.insert(
            base_text_buffer_id,
            DiffState::new_inverted(diff, main_buffer, cx),
        );
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
        self.snapshot.get_mut().all_diff_hunks_expanded = true;
        self.expand_or_collapse_diff_hunks(vec![Anchor::Min..Anchor::Max], true, cx);
    }

    pub fn all_diff_hunks_expanded(&self) -> bool {
        self.snapshot.borrow().all_diff_hunks_expanded
    }

    pub fn set_all_diff_hunks_collapsed(&mut self, cx: &mut Context<Self>) {
        self.snapshot.get_mut().all_diff_hunks_expanded = false;
        self.expand_or_collapse_diff_hunks(vec![Anchor::Min..Anchor::Max], false, cx);
    }

    pub fn set_show_deleted_hunks(&mut self, show: bool, cx: &mut Context<Self>) {
        self.snapshot.get_mut().show_deleted_hunks = show;

        self.sync_mut(cx);

        let old_len = self.snapshot.borrow().len();

        let ranges = std::iter::once((Point::zero()..Point::MAX, None));
        let _ = self.expand_or_collapse_diff_hunks_inner(ranges, true, cx);

        let new_len = self.snapshot.borrow().len();

        self.subscriptions.publish(vec![Edit {
            old: MultiBufferOffset(0)..old_len,
            new: MultiBufferOffset(0)..new_len,
        }]);

        cx.emit(Event::DiffHunksToggled);
        cx.emit(Event::Edited {
            edited_buffer: None,
            is_local: true,
        });
    }

    pub fn set_use_extended_diff_range(&mut self, use_extended: bool, _cx: &mut Context<Self>) {
        self.snapshot.get_mut().use_extended_diff_range = use_extended;
    }

    pub fn has_multiple_hunks(&self, cx: &App) -> bool {
        self.read(cx)
            .diff_hunks_in_range(Anchor::Min..Anchor::Max)
            .nth(1)
            .is_some()
    }

    pub fn single_hunk_is_expanded(&self, range: Range<Anchor>, cx: &App) -> bool {
        let snapshot = self.read(cx);
        let mut cursor = snapshot.diff_transforms.cursor::<MultiBufferOffset>(());
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
        let mut cursor = snapshot.diff_transforms.cursor::<MultiBufferOffset>(());
        for range in ranges {
            let range = range.to_point(&snapshot);
            let start = snapshot.point_to_offset(Point::new(range.start.row, 0));
            let end = (snapshot.point_to_offset(Point::new(range.end.row + 1, 0)) + 1usize)
                .min(snapshot.len());
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
        ranges: impl IntoIterator<Item = (Range<Point>, Option<Anchor>)>,
        expand: bool,
        cx: &mut Context<Self>,
    ) -> Vec<Edit<MultiBufferOffset>> {
        if self.snapshot.borrow().all_diff_hunks_expanded && !expand {
            return Vec::new();
        }
        self.sync_mut(cx);
        let mut snapshot = self.snapshot.get_mut();
        let mut excerpt_edits = Vec::new();
        let mut last_hunk_row = None;
        for (range, end_anchor) in ranges {
            for diff_hunk in snapshot.diff_hunks_in_range(range) {
                if let Some(end_anchor) = &end_anchor
                    && let Some(hunk_end_anchor) =
                        snapshot.anchor_in_excerpt(diff_hunk.excerpt_range.context.end)
                    && hunk_end_anchor.cmp(end_anchor, snapshot).is_gt()
                {
                    continue;
                }
                let hunk_range = diff_hunk.multi_buffer_range;
                if let Some(excerpt_start_anchor) =
                    snapshot.anchor_in_excerpt(diff_hunk.excerpt_range.context.start)
                    && hunk_range.start.to_point(snapshot) < excerpt_start_anchor.to_point(snapshot)
                {
                    continue;
                }
                if last_hunk_row.is_some_and(|row| row >= diff_hunk.row_range.start) {
                    continue;
                }
                let mut start = snapshot.excerpt_offset_for_anchor(&hunk_range.start);
                let mut end = snapshot.excerpt_offset_for_anchor(&hunk_range.end);
                if let Some(excerpt_end_anchor) =
                    snapshot.anchor_in_excerpt(diff_hunk.excerpt_range.context.end)
                {
                    let excerpt_end = snapshot.excerpt_offset_for_anchor(&excerpt_end_anchor);
                    start = start.min(excerpt_end);
                    end = end.min(excerpt_end);
                };
                last_hunk_row = Some(diff_hunk.row_range.start);
                excerpt_edits.push(text::Edit {
                    old: start..end,
                    new: start..end,
                });
            }
        }

        Self::sync_diff_transforms(
            &mut snapshot,
            excerpt_edits,
            DiffChangeKind::ExpandOrCollapseHunks { expand },
        )
    }

    pub fn expand_or_collapse_diff_hunks(
        &mut self,
        ranges: Vec<Range<Anchor>>,
        expand: bool,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.snapshot.borrow().clone();
        let ranges =
            ranges.iter().map(move |range| {
                let excerpt_end = snapshot.excerpt_containing(range.end..range.end).and_then(
                    |(_, excerpt_range)| snapshot.anchor_in_excerpt(excerpt_range.context.end),
                );
                let range = range.to_point(&snapshot);
                let mut peek_end = range.end;
                if range.end.row < snapshot.max_row().0 {
                    peek_end = Point::new(range.end.row + 1, 0);
                };
                (range.start..peek_end, excerpt_end)
            });
        let edits = self.expand_or_collapse_diff_hunks_inner(ranges, expand, cx);
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }
        cx.emit(Event::DiffHunksToggled);
        cx.emit(Event::Edited {
            edited_buffer: None,
            is_local: true,
        });
    }

    #[ztracing::instrument(skip_all)]
    fn sync(&self, cx: &App) {
        let changed = self.buffer_changed_since_sync.replace(false);
        if !changed {
            return;
        }
        let edits = Self::sync_from_buffer_changes(
            &mut self.snapshot.borrow_mut(),
            &self.buffers,
            &self.diffs,
            cx,
        );
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }
    }

    fn sync_mut(&mut self, cx: &App) -> &mut MultiBufferSnapshot {
        let snapshot = self.snapshot.get_mut();
        let changed = self.buffer_changed_since_sync.replace(false);
        if !changed {
            return snapshot;
        }
        let edits = Self::sync_from_buffer_changes(snapshot, &self.buffers, &self.diffs, cx);

        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }

        snapshot
    }

    fn sync_from_buffer_changes(
        snapshot: &mut MultiBufferSnapshot,
        buffers: &BTreeMap<BufferId, BufferState>,
        diffs: &HashMap<BufferId, DiffState>,
        cx: &App,
    ) -> Vec<Edit<MultiBufferOffset>> {
        let MultiBufferSnapshot {
            excerpts,
            diffs: buffer_diff,
            buffers: buffer_snapshots,
            path_keys_by_index: _,
            indices_by_path_key: _,
            diff_transforms: _,
            non_text_state_update_count,
            edit_count,
            is_dirty,
            has_deleted_file,
            has_conflict,
            has_inverted_diff: _,
            singleton: _,
            trailing_excerpt_update_count: _,
            all_diff_hunks_expanded: _,
            show_deleted_hunks: _,
            use_extended_diff_range: _,
            show_headers: _,
        } = snapshot;
        *is_dirty = false;
        *has_deleted_file = false;
        *has_conflict = false;

        if !diffs.is_empty() {
            let mut diffs_to_add = Vec::new();
            for (id, diff) in diffs {
                if find_diff_state(buffer_diff, *id).is_none_or(|existing_diff| {
                    if existing_diff.main_buffer.is_none() {
                        return false;
                    }
                    let base_text = diff.diff.read(cx).base_text_buffer().read(cx);
                    base_text.remote_id() != existing_diff.base_text().remote_id()
                        || base_text
                            .version()
                            .changed_since(existing_diff.base_text().version())
                }) {
                    if diffs_to_add.capacity() == 0 {
                        diffs_to_add.reserve(diffs.len());
                    }
                    diffs_to_add.push(sum_tree::Edit::Insert(diff.snapshot(*id, cx)));
                }
            }
            buffer_diff.edit(diffs_to_add, ());
        }

        let mut paths_to_edit = Vec::new();
        let mut non_text_state_updated = false;
        let mut edited = false;
        for buffer_state in buffers.values() {
            let buffer = buffer_state.buffer.read(cx);
            let last_snapshot = buffer_snapshots
                .get(&buffer.remote_id())
                .expect("each buffer should have a snapshot");
            let current_version = buffer.version();
            let non_text_state_update_count = buffer.non_text_state_update_count();

            let buffer_edited =
                current_version.changed_since(last_snapshot.buffer_snapshot.version());
            let buffer_non_text_state_updated = non_text_state_update_count
                > last_snapshot.buffer_snapshot.non_text_state_update_count();
            if buffer_edited || buffer_non_text_state_updated {
                paths_to_edit.push((
                    last_snapshot.path_key.clone(),
                    last_snapshot.path_key_index,
                    buffer_state.buffer.clone(),
                    if buffer_edited {
                        Some(last_snapshot.buffer_snapshot.version().clone())
                    } else {
                        None
                    },
                ));
            }

            edited |= buffer_edited;
            non_text_state_updated |= buffer_non_text_state_updated;
            *is_dirty |= buffer.is_dirty();
            *has_deleted_file |= buffer
                .file()
                .is_some_and(|file| file.disk_state().is_deleted());
            *has_conflict |= buffer.has_conflict();
        }
        if edited {
            *edit_count += 1;
        }
        if non_text_state_updated {
            *non_text_state_update_count += 1;
        }

        paths_to_edit.sort_unstable_by_key(|(path, _, _, _)| path.clone());

        let mut edits = Vec::new();
        let mut new_excerpts = SumTree::default();
        let mut cursor = excerpts.cursor::<ExcerptSummary>(());

        for (path, path_key_index, buffer, prev_version) in paths_to_edit {
            new_excerpts.append(cursor.slice(&path, Bias::Left), ());
            let buffer = buffer.read(cx);
            let buffer_id = buffer.remote_id();

            buffer_snapshots.insert(
                buffer_id,
                BufferStateSnapshot {
                    path_key: path.clone(),
                    path_key_index,
                    buffer_snapshot: buffer.snapshot(),
                },
            );

            if let Some(prev_version) = &prev_version {
                while let Some(old_excerpt) = cursor.item()
                    && &old_excerpt.path_key == &path
                {
                    edits.extend(
                        buffer
                            .edits_since_in_range::<usize>(
                                prev_version,
                                old_excerpt.range.context.clone(),
                            )
                            .map(|edit| {
                                let excerpt_old_start = cursor.start().len();
                                let excerpt_new_start =
                                    ExcerptDimension(new_excerpts.summary().text.len);
                                let old_start = excerpt_old_start + edit.old.start;
                                let old_end = excerpt_old_start + edit.old.end;
                                let new_start = excerpt_new_start + edit.new.start;
                                let new_end = excerpt_new_start + edit.new.end;
                                Edit {
                                    old: old_start..old_end,
                                    new: new_start..new_end,
                                }
                            }),
                    );

                    let excerpt = Excerpt::new(
                        old_excerpt.path_key.clone(),
                        old_excerpt.path_key_index,
                        &buffer.snapshot(),
                        old_excerpt.range.clone(),
                        old_excerpt.has_trailing_newline,
                    );
                    new_excerpts.push(excerpt, ());
                    cursor.next();
                }
            } else {
                new_excerpts.append(cursor.slice(&path, Bias::Right), ());
            };
        }
        new_excerpts.append(cursor.suffix(), ());

        drop(cursor);
        *excerpts = new_excerpts;

        Self::sync_diff_transforms(snapshot, edits, DiffChangeKind::BufferEdited)
    }

    fn sync_diff_transforms(
        snapshot: &mut MultiBufferSnapshot,
        excerpt_edits: Vec<text::Edit<ExcerptOffset>>,
        change_kind: DiffChangeKind,
    ) -> Vec<Edit<MultiBufferOffset>> {
        if excerpt_edits.is_empty() {
            return vec![];
        }

        let mut excerpts = snapshot.excerpts.cursor::<ExcerptOffset>(());
        let mut old_diff_transforms = snapshot
            .diff_transforms
            .cursor::<Dimensions<ExcerptOffset, MultiBufferOffset>>(());
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
                Self::append_diff_transforms(&mut new_diff_transforms, transforms_before_edit);
                if let Some(transform) = old_diff_transforms.item()
                    && old_diff_transforms.end().0 == edit.old.start
                    && old_diff_transforms.start().0 < edit.old.start
                {
                    Self::push_diff_transform(&mut new_diff_transforms, transform.clone());
                    old_diff_transforms.next();
                }
            }

            // Compute the start of the edit in output coordinates.
            let edit_start_overshoot = edit.old.start - old_diff_transforms.start().0;
            let edit_old_start = old_diff_transforms.start().1 + edit_start_overshoot;
            let edit_new_start =
                MultiBufferOffset((edit_old_start.0 as isize + output_delta) as usize);

            let changed_diff_hunks = Self::recompute_diff_transforms_for_edit(
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
            let edit_old_end = old_diff_transforms.start().1 + edit_old_end_overshoot;
            let edit_new_end = new_diff_transforms.summary().output.len + edit_new_end_overshoot;
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
                            if let Some(diff) = find_diff_state(&snapshot.diffs, excerpt.buffer_id)
                                && diff.main_buffer.is_some()
                            {
                                return true;
                            }
                            hunk.hunk_start_anchor
                                .is_valid(&excerpt.buffer_snapshot(&snapshot))
                        }),
                        _ => true,
                    };

                let mut excerpt_offset = edit.new.end;
                if !keep_next_old_transform {
                    excerpt_offset += old_diff_transforms.end().0 - edit.old.end;
                    old_diff_transforms.next();
                }

                old_expanded_hunks.clear();
                Self::push_buffer_content_transform(
                    snapshot,
                    &mut new_diff_transforms,
                    excerpt_offset,
                    end_of_current_insert,
                );
                at_transform_boundary = true;
            }
        }

        // Keep any transforms that are after the last edit.
        Self::append_diff_transforms(&mut new_diff_transforms, old_diff_transforms.suffix());

        // Ensure there's always at least one buffer content transform.
        if new_diff_transforms.is_empty() {
            new_diff_transforms.push(
                DiffTransform::BufferContent {
                    summary: Default::default(),
                    inserted_hunk_info: None,
                },
                (),
            );
        }

        drop(old_diff_transforms);
        drop(excerpts);
        snapshot.diff_transforms = new_diff_transforms;
        snapshot.edit_count += 1;

        #[cfg(any(test, feature = "test-support"))]
        snapshot.check_invariants();
        output_edits
    }

    fn recompute_diff_transforms_for_edit(
        edit: &Edit<ExcerptOffset>,
        excerpts: &mut Cursor<Excerpt, ExcerptOffset>,
        old_diff_transforms: &mut Cursor<
            DiffTransform,
            Dimensions<ExcerptOffset, MultiBufferOffset>,
        >,
        new_diff_transforms: &mut SumTree<DiffTransform>,
        end_of_current_insert: &mut Option<(ExcerptOffset, DiffTransformHunkInfo)>,
        old_expanded_hunks: &mut HashSet<DiffTransformHunkInfo>,
        snapshot: &MultiBufferSnapshot,
        change_kind: DiffChangeKind,
    ) -> bool {
        log::trace!(
            "recomputing diff transform for edit {:?} => {:?}",
            edit.old.start..edit.old.end,
            edit.new.start..edit.new.end
        );

        // Record which hunks were previously expanded.
        while let Some(item) = old_diff_transforms.item() {
            if let Some(hunk_info) = item.hunk_info() {
                log::trace!(
                    "previously expanded hunk at {:?}",
                    old_diff_transforms.start()
                );
                old_expanded_hunks.insert(hunk_info);
            }
            if old_diff_transforms.end().0 > edit.old.end {
                break;
            }
            old_diff_transforms.next();
        }

        // Avoid querying diff hunks if there's no possibility of hunks being expanded.
        // For inverted diffs, hunks are always shown, so we can't skip this.
        let all_diff_hunks_expanded = snapshot.all_diff_hunks_expanded;
        if old_expanded_hunks.is_empty()
            && change_kind == DiffChangeKind::BufferEdited
            && !all_diff_hunks_expanded
            && !snapshot.has_inverted_diff
        {
            return false;
        }

        // Visit each excerpt that intersects the edit.
        let mut did_expand_hunks = false;
        while let Some(excerpt) = excerpts.item() {
            // Recompute the expanded hunks in the portion of the excerpt that
            // intersects the edit.
            if let Some(diff) = find_diff_state(&snapshot.diffs, excerpt.buffer_id) {
                let buffer_snapshot = &excerpt.buffer_snapshot(&snapshot);
                let excerpt_start = *excerpts.start();
                let excerpt_end = excerpt_start + excerpt.text_summary.len;
                let excerpt_buffer_start = excerpt.range.context.start.to_offset(buffer_snapshot);
                let excerpt_buffer_end = excerpt_buffer_start + excerpt.text_summary.len;
                let edit_buffer_start =
                    excerpt_buffer_start + edit.new.start.saturating_sub(excerpt_start);
                let edit_buffer_end =
                    excerpt_buffer_start + edit.new.end.saturating_sub(excerpt_start);
                let edit_buffer_end = edit_buffer_end.min(excerpt_buffer_end);

                if let Some(main_buffer) = &diff.main_buffer {
                    for hunk in diff.hunks_intersecting_base_text_range(
                        edit_buffer_start..edit_buffer_end,
                        main_buffer,
                    ) {
                        did_expand_hunks = true;
                        let hunk_buffer_range = hunk.diff_base_byte_range.clone();
                        if hunk_buffer_range.start < excerpt_buffer_start {
                            log::trace!("skipping hunk that starts before excerpt");
                            continue;
                        }
                        let hunk_excerpt_start = excerpt_start
                            + hunk_buffer_range.start.saturating_sub(excerpt_buffer_start);
                        let hunk_excerpt_end = excerpt_end
                            .min(excerpt_start + (hunk_buffer_range.end - excerpt_buffer_start));
                        Self::push_buffer_content_transform(
                            snapshot,
                            new_diff_transforms,
                            hunk_excerpt_start,
                            *end_of_current_insert,
                        );
                        if !hunk_buffer_range.is_empty() {
                            let hunk_info = DiffTransformHunkInfo {
                                buffer_id: buffer_snapshot.remote_id(),
                                hunk_start_anchor: hunk.buffer_range.start,
                                hunk_secondary_status: hunk.secondary_status,
                                excerpt_end: excerpt.end_anchor(),
                                is_logically_deleted: true,
                            };
                            *end_of_current_insert =
                                Some((hunk_excerpt_end.min(excerpt_end), hunk_info));
                        }
                    }
                } else {
                    let edit_anchor_range = buffer_snapshot.anchor_before(edit_buffer_start)
                        ..buffer_snapshot.anchor_after(edit_buffer_end);
                    for hunk in diff.hunks_intersecting_range(edit_anchor_range, buffer_snapshot) {
                        if hunk.is_created_file() && !all_diff_hunks_expanded {
                            continue;
                        }

                        let hunk_buffer_range = hunk.buffer_range.to_offset(buffer_snapshot);
                        if hunk_buffer_range.start < excerpt_buffer_start {
                            log::trace!("skipping hunk that starts before excerpt");
                            continue;
                        }

                        let hunk_info = DiffTransformHunkInfo {
                            buffer_id: buffer_snapshot.remote_id(),
                            hunk_start_anchor: hunk.buffer_range.start,
                            hunk_secondary_status: hunk.secondary_status,
                            excerpt_end: excerpt.end_anchor(),
                            is_logically_deleted: false,
                        };

                        let hunk_excerpt_start = excerpt_start
                            + hunk_buffer_range.start.saturating_sub(excerpt_buffer_start);
                        let hunk_excerpt_end = excerpt_end
                            .min(excerpt_start + (hunk_buffer_range.end - excerpt_buffer_start));

                        Self::push_buffer_content_transform(
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
                                    || (hunk_buffer_range.end > edit_buffer_start);
                                if *expand {
                                    intersects || was_previously_expanded || all_diff_hunks_expanded
                                } else {
                                    !intersects
                                        && (was_previously_expanded || all_diff_hunks_expanded)
                                }
                            }
                            _ => was_previously_expanded || all_diff_hunks_expanded,
                        };

                        if should_expand_hunk {
                            did_expand_hunks = true;
                            log::trace!(
                                "expanding hunk {:?}",
                                hunk_excerpt_start..hunk_excerpt_end,
                            );

                            if !hunk.diff_base_byte_range.is_empty()
                                && hunk_buffer_range.start >= edit_buffer_start
                                && hunk_buffer_range.start <= excerpt_buffer_end
                                && snapshot.show_deleted_hunks
                            {
                                let base_text = diff.base_text();
                                let mut text_cursor =
                                    base_text.as_rope().cursor(hunk.diff_base_byte_range.start);
                                let mut base_text_summary = text_cursor
                                    .summary::<TextSummary>(hunk.diff_base_byte_range.end);

                                let mut has_trailing_newline = false;
                                if base_text_summary.last_line_chars > 0 {
                                    base_text_summary += TextSummary::newline();
                                    has_trailing_newline = true;
                                }

                                new_diff_transforms.push(
                                    DiffTransform::DeletedHunk {
                                        base_text_byte_range: hunk.diff_base_byte_range.clone(),
                                        summary: base_text_summary,
                                        buffer_id: buffer_snapshot.remote_id(),
                                        hunk_info,
                                        has_trailing_newline,
                                    },
                                    (),
                                );
                            }

                            if !hunk_buffer_range.is_empty() {
                                *end_of_current_insert =
                                    Some((hunk_excerpt_end.min(excerpt_end), hunk_info));
                            }
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
        new_transforms: &mut SumTree<DiffTransform>,
        subtree: SumTree<DiffTransform>,
    ) {
        if let Some(DiffTransform::BufferContent {
            inserted_hunk_info,
            summary,
        }) = subtree.first()
            && Self::extend_last_buffer_content_transform(
                new_transforms,
                *inserted_hunk_info,
                *summary,
            )
        {
            let mut cursor = subtree.cursor::<()>(());
            cursor.next();
            cursor.next();
            new_transforms.append(cursor.suffix(), ());
            return;
        }
        new_transforms.append(subtree, ());
    }

    fn push_diff_transform(new_transforms: &mut SumTree<DiffTransform>, transform: DiffTransform) {
        if let DiffTransform::BufferContent {
            inserted_hunk_info: inserted_hunk_anchor,
            summary,
        } = transform
            && Self::extend_last_buffer_content_transform(
                new_transforms,
                inserted_hunk_anchor,
                summary,
            )
        {
            return;
        }
        new_transforms.push(transform, ());
    }

    fn push_buffer_content_transform(
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
                .text_summary_for_excerpt_offset_range::<MBTextSummary>(start_offset..end_offset);

            if !Self::extend_last_buffer_content_transform(
                new_transforms,
                inserted_hunk_info,
                summary_to_add,
            ) {
                new_transforms.push(
                    DiffTransform::BufferContent {
                        summary: summary_to_add,
                        inserted_hunk_info,
                    },
                    (),
                )
            }
        }
    }

    fn extend_last_buffer_content_transform(
        new_transforms: &mut SumTree<DiffTransform>,
        new_inserted_hunk_info: Option<DiffTransformHunkInfo>,
        summary_to_add: MBTextSummary,
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
            (),
        );
        did_extend
    }

    pub fn toggle_single_diff_hunk(&mut self, range: Range<Anchor>, cx: &mut Context<Self>) {
        let snapshot = self.snapshot(cx);
        let excerpt_end = snapshot
            .excerpt_containing(range.end..range.end)
            .and_then(|(_, excerpt_range)| snapshot.anchor_in_excerpt(excerpt_range.context.end));
        let point_range = range.to_point(&snapshot);
        let expand = !self.single_hunk_is_expanded(range, cx);
        let edits =
            self.expand_or_collapse_diff_hunks_inner([(point_range, excerpt_end)], expand, cx);
        if !edits.is_empty() {
            self.subscriptions.publish(edits);
        }
        cx.emit(Event::DiffHunksToggled);
        cx.emit(Event::Edited {
            edited_buffer: None,
            is_local: true,
        });
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
        for (ix, (text, ranges)) in excerpts.into_iter().enumerate() {
            let buffer = cx.new(|cx| Buffer::local(text, cx));
            let snapshot = buffer.read(cx).snapshot();
            let excerpt_ranges = ranges
                .into_iter()
                .map(ExcerptRange::new)
                .collect::<Vec<_>>();
            multi.update(cx, |multi, cx| {
                multi.set_excerpt_ranges_for_path(
                    PathKey::sorted(ix as u64),
                    buffer,
                    &snapshot,
                    excerpt_ranges,
                    cx,
                )
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
        let mut edits: Vec<(Range<MultiBufferOffset>, Arc<str>)> = Vec::new();
        let mut last_end = None;
        for _ in 0..edit_count {
            if last_end.is_some_and(|last_end| last_end >= snapshot.len()) {
                break;
            }

            let new_start = last_end.map_or(MultiBufferOffset::ZERO, |last_end| last_end + 1usize);
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

        let max_buffers = env::var("MAX_BUFFERS")
            .map(|i| i.parse().expect("invalid `MAX_EXCERPTS` variable"))
            .unwrap_or(5);

        let mut buffers = Vec::new();
        for _ in 0..mutation_count {
            let snapshot = self.snapshot(cx);
            let buffer_ids = snapshot.all_buffer_ids().collect::<Vec<_>>();
            if buffer_ids.is_empty() || (rng.random() && buffer_ids.len() < max_buffers) {
                let buffer_handle = if rng.random() || self.buffers.is_empty() {
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
                    self.buffers.values().choose(rng).unwrap().buffer.clone()
                };

                let buffer = buffer_handle.read(cx);
                let buffer_text = buffer.text();
                let buffer_snapshot = buffer.snapshot();
                let mut next_min_start_ix = 0;
                let ranges = (0..rng.random_range(0..5))
                    .filter_map(|_| {
                        if next_min_start_ix >= buffer.len() {
                            return None;
                        }
                        let end_ix = buffer.clip_offset(
                            rng.random_range(next_min_start_ix..=buffer.len()),
                            Bias::Right,
                        );
                        let start_ix = buffer
                            .clip_offset(rng.random_range(next_min_start_ix..=end_ix), Bias::Left);
                        next_min_start_ix = buffer.text().ceil_char_boundary(end_ix + 1);
                        Some(ExcerptRange::new(start_ix..end_ix))
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

                let path_key = PathKey::for_buffer(&buffer_handle, cx);
                self.set_merged_excerpt_ranges_for_path(
                    path_key.clone(),
                    buffer_handle,
                    &buffer_snapshot,
                    ranges,
                    cx,
                );
                log::info!("Inserted with path_key: {:?}", path_key);
            } else {
                let path_key = self
                    .snapshot
                    .borrow()
                    .buffers
                    .get(&buffer_ids.choose(rng).unwrap())
                    .unwrap()
                    .path_key
                    .clone();
                log::info!("Removing excerpts {:?}", path_key);
                self.remove_excerpts(path_key, cx);
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
        self.chunks(
            MultiBufferOffset::ZERO..self.len(),
            LanguageAwareStyling {
                tree_sitter: false,
                diagnostics: false,
            },
        )
        .map(|chunk| chunk.text)
        .collect()
    }

    pub fn reversed_chars_at<T: ToOffset>(&self, position: T) -> impl Iterator<Item = char> + '_ {
        self.reversed_chunks_in_range(MultiBufferOffset::ZERO..position.to_offset(self))
            .flat_map(|c| c.chars().rev())
    }

    fn reversed_chunks_in_range(
        &self,
        range: Range<MultiBufferOffset>,
    ) -> ReversedMultiBufferChunks<'_> {
        let mut cursor = self.cursor::<MultiBufferOffset, BufferOffset>();
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
        self.chunks(
            range,
            LanguageAwareStyling {
                tree_sitter: false,
                diagnostics: false,
            },
        )
        .map(|chunk| chunk.text)
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
        self.diff_hunks_in_range(Anchor::Min..Anchor::Max)
    }

    pub fn diff_hunks_in_range<T: ToPoint>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = MultiBufferDiffHunk> + '_ {
        let query_range = range.start.to_point(self)..range.end.to_point(self);
        self.lift_buffer_metadata(query_range.clone(), move |buffer, buffer_range| {
            let diff = self.diff_state(buffer.remote_id())?;
            let iter = if let Some(main_buffer) = &diff.main_buffer {
                let buffer_start = buffer.point_to_offset(buffer_range.start);
                let buffer_end = buffer.point_to_offset(buffer_range.end);
                itertools::Either::Left(
                    diff.hunks_intersecting_base_text_range(buffer_start..buffer_end, main_buffer)
                        .map(move |hunk| (hunk, buffer, true)),
                )
            } else {
                let buffer_start = buffer.anchor_before(buffer_range.start);
                let buffer_end = buffer.anchor_after(buffer_range.end);
                itertools::Either::Right(
                    diff.hunks_intersecting_range(buffer_start..buffer_end, buffer)
                        .map(move |hunk| (hunk, buffer, false)),
                )
            };
            Some(iter.filter_map(|(hunk, buffer, is_inverted)| {
                if hunk.is_created_file() && !self.all_diff_hunks_expanded {
                    return None;
                }
                let range = if is_inverted {
                    hunk.diff_base_byte_range.to_point(&buffer)
                } else {
                    hunk.range.clone()
                };
                Some((range, (hunk, is_inverted)))
            }))
        })
        .filter_map(move |(range, (hunk, is_inverted), excerpt)| {
            let buffer_snapshot = excerpt.buffer_snapshot(self);
            if range.start != range.end && range.end == query_range.start && !hunk.range.is_empty()
            {
                return None;
            }
            let end_row = if range.end.column == 0 {
                range.end.row
            } else {
                range.end.row + 1
            };

            let word_diffs =
                (!hunk.base_word_diffs.is_empty() || !hunk.buffer_word_diffs.is_empty())
                    .then(|| {
                        let mut word_diffs = Vec::new();

                        if self.show_deleted_hunks || is_inverted {
                            let hunk_start_offset = if is_inverted {
                                Anchor::in_buffer(
                                    excerpt.path_key_index,
                                    buffer_snapshot.anchor_after(hunk.diff_base_byte_range.start),
                                )
                                .to_offset(self)
                            } else {
                                Anchor::in_buffer(excerpt.path_key_index, hunk.buffer_range.start)
                                    .to_offset(self)
                            };

                            word_diffs.extend(hunk.base_word_diffs.iter().map(|diff| {
                                hunk_start_offset + diff.start..hunk_start_offset + diff.end
                            }));
                        }

                        if !is_inverted {
                            word_diffs.extend(hunk.buffer_word_diffs.into_iter().map(|diff| {
                                Anchor::range_in_buffer(excerpt.path_key_index, diff)
                                    .to_offset(self)
                            }));
                        }
                        word_diffs
                    })
                    .unwrap_or_default();

            let buffer_range = if is_inverted {
                buffer_snapshot.anchor_after(hunk.diff_base_byte_range.start)
                    ..buffer_snapshot.anchor_before(hunk.diff_base_byte_range.end)
            } else {
                hunk.buffer_range.clone()
            };
            let status_kind = if hunk.buffer_range.start == hunk.buffer_range.end {
                DiffHunkStatusKind::Deleted
            } else if hunk.diff_base_byte_range.is_empty() {
                DiffHunkStatusKind::Added
            } else {
                DiffHunkStatusKind::Modified
            };
            let multi_buffer_range =
                Anchor::range_in_buffer(excerpt.path_key_index, buffer_range.clone());
            Some(MultiBufferDiffHunk {
                row_range: MultiBufferRow(range.start.row)..MultiBufferRow(end_row),
                buffer_id: buffer_snapshot.remote_id(),
                buffer_range,
                word_diffs,
                diff_base_byte_range: BufferOffset(hunk.diff_base_byte_range.start)
                    ..BufferOffset(hunk.diff_base_byte_range.end),
                status: DiffHunkStatus {
                    kind: status_kind,
                    secondary: hunk.secondary_status,
                },
                excerpt_range: excerpt.range.clone(),
                multi_buffer_range,
            })
        })
    }

    fn excerpts_for_range<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = &Excerpt> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.cursor::<MultiBufferOffset, BufferOffset>();
        cursor.seek(&range.start);
        std::iter::from_fn(move || {
            let region = cursor.region()?;
            if region.range.start > range.end
                || region.range.start == range.end && region.range.start > range.start
            {
                return None;
            }
            let excerpt = region.excerpt;
            cursor.next_excerpt_forwards();
            Some(excerpt)
        })
    }

    pub fn buffer_ids_for_range<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = BufferId> + '_ {
        self.excerpts_for_range(range)
            .map(|excerpt| excerpt.buffer_snapshot(self).remote_id())
    }

    /// Resolves the given [`text::Anchor`]s to [`crate::Anchor`]s if the anchor is within a visible excerpt.
    ///
    /// The passed in anchors must be ordered.
    pub fn text_anchors_to_visible_anchors(
        &self,
        anchors: impl IntoIterator<Item = text::Anchor>,
    ) -> Vec<Option<Anchor>> {
        let anchors = anchors.into_iter();
        let mut result = Vec::with_capacity(anchors.size_hint().0);
        let mut anchors = anchors.peekable();
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(());
        'anchors: while let Some(anchor) = anchors.peek() {
            let buffer_id = anchor.buffer_id;
            let mut same_buffer_anchors = anchors.peeking_take_while(|a| a.buffer_id == buffer_id);

            if let Some(buffer) = self.buffers.get(&buffer_id) {
                let path = &buffer.path_key;
                let Some(mut next) = same_buffer_anchors.next() else {
                    continue 'anchors;
                };
                cursor.seek_forward(path, Bias::Left);
                'excerpts: loop {
                    let Some(excerpt) = cursor.item() else {
                        break;
                    };
                    if &excerpt.path_key != path {
                        break;
                    }
                    let buffer_snapshot = excerpt.buffer_snapshot(self);

                    loop {
                        // anchor is before the first excerpt
                        if excerpt
                            .range
                            .context
                            .start
                            .cmp(&next, &buffer_snapshot)
                            .is_gt()
                        {
                            // so we skip it and try the next anchor
                            result.push(None);
                            match same_buffer_anchors.next() {
                                Some(anchor) => next = anchor,
                                None => continue 'anchors,
                            }
                        // anchor is within the excerpt
                        } else if excerpt
                            .range
                            .context
                            .end
                            .cmp(&next, &buffer_snapshot)
                            .is_ge()
                        {
                            // record it and all following anchors that are within
                            result.push(Some(Anchor::in_buffer(excerpt.path_key_index, next)));
                            result.extend(
                                same_buffer_anchors
                                    .peeking_take_while(|a| {
                                        excerpt.range.context.end.cmp(a, &buffer_snapshot).is_ge()
                                    })
                                    .map(|a| Some(Anchor::in_buffer(excerpt.path_key_index, a))),
                            );
                            match same_buffer_anchors.next() {
                                Some(anchor) => next = anchor,
                                None => continue 'anchors,
                            }
                        // anchor is after the excerpt, try the next one
                        } else {
                            cursor.next();
                            continue 'excerpts;
                        }
                    }
                }
                // account for `next`
                result.push(None);
            }
            result.extend(same_buffer_anchors.map(|_| None));
        }

        result
    }

    pub fn range_to_buffer_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Vec<(
        BufferSnapshot,
        Range<BufferOffset>,
        ExcerptRange<text::Anchor>,
    )> {
        let mut cursor = self.cursor::<MultiBufferOffset, BufferOffset>();
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);
        cursor.seek(&start);

        let mut result: Vec<(
            BufferSnapshot,
            Range<BufferOffset>,
            ExcerptRange<text::Anchor>,
        )> = Vec::new();
        while let Some(region) = cursor.region() {
            if region.range.start >= end {
                break;
            }
            if region.is_main_buffer {
                let start_overshoot = start.saturating_sub(region.range.start);
                let end_offset = end;
                let end_overshoot = end_offset.saturating_sub(region.range.start);
                let start = region
                    .buffer_range
                    .end
                    .min(region.buffer_range.start + start_overshoot);
                let end = region
                    .buffer_range
                    .end
                    .min(region.buffer_range.start + end_overshoot);
                let excerpt_range = region.excerpt.range.clone();
                if let Some(prev) =
                    result
                        .last_mut()
                        .filter(|(prev_buffer, prev_range, prev_excerpt)| {
                            prev_buffer.remote_id() == region.buffer.remote_id()
                                && prev_range.end == start
                                && prev_excerpt.context.start == excerpt_range.context.start
                        })
                {
                    prev.1.end = end;
                } else {
                    result.push((region.buffer.clone(), start..end, excerpt_range));
                }
            }
            cursor.next();
        }

        if let Some(excerpt) = cursor.excerpt()
            && excerpt.text_summary.len == 0
            && end == self.len()
        {
            let buffer_snapshot = excerpt.buffer_snapshot(self);

            let buffer_offset =
                BufferOffset(excerpt.range.context.start.to_offset(buffer_snapshot));
            let excerpt_range = excerpt.range.clone();
            if result
                .last_mut()
                .is_none_or(|(prev_buffer, prev_range, prev_excerpt)| {
                    prev_buffer.remote_id() != buffer_snapshot.remote_id()
                        || prev_range.end != buffer_offset
                        || prev_excerpt.context.start != excerpt_range.context.start
                })
            {
                result.push((
                    buffer_snapshot.clone(),
                    buffer_offset..buffer_offset,
                    excerpt_range,
                ));
            }
        }

        result
    }

    pub fn range_to_buffer_ranges_with_deleted_hunks<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = (&BufferSnapshot, Range<BufferOffset>, Option<Anchor>)> + '_ {
        let start = range.start.to_offset(self);
        let end = range.end.to_offset(self);

        let mut cursor = self.cursor::<MultiBufferOffset, BufferOffset>();
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

            let deleted_hunk_anchor = if region.is_main_buffer {
                None
            } else {
                Some(self.anchor_before(region.range.start))
            };
            let result = (region.buffer, start..end, deleted_hunk_anchor);
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
    fn lift_buffer_metadata<'a, MBD, M, I>(
        &'a self,
        query_range: Range<MBD>,
        get_buffer_metadata: impl 'a + Fn(&'a BufferSnapshot, Range<MBD::TextDimension>) -> Option<I>,
    ) -> impl Iterator<Item = (Range<MBD>, M, &'a Excerpt)> + 'a
    where
        I: Iterator<Item = (Range<MBD::TextDimension>, M)> + 'a,
        MBD: MultiBufferDimension
            + Ord
            + Sub<Output = MBD::TextDimension>
            + ops::Add<MBD::TextDimension, Output = MBD>
            + ops::AddAssign<MBD::TextDimension>,
        MBD::TextDimension: Sub<Output = MBD::TextDimension>
            + ops::Add<Output = MBD::TextDimension>
            + AddAssign<MBD::TextDimension>
            + Ord,
    {
        let mut current_excerpt_metadata: Option<(ExcerptRange<text::Anchor>, I)> = None;
        let mut cursor = self.cursor::<MBD, MBD::TextDimension>();

        // Find the excerpt and buffer offset where the given range ends.
        cursor.seek(&query_range.end);
        let mut range_end = None;
        while let Some(region) = cursor.region() {
            if region.is_main_buffer {
                let mut buffer_end = region.buffer_range.start;
                let overshoot = if query_range.end > region.range.start {
                    query_range.end - region.range.start
                } else {
                    <MBD::TextDimension>::default()
                };
                buffer_end = buffer_end + overshoot;
                range_end = Some((region.excerpt.range.clone(), buffer_end));
                break;
            }
            cursor.next();
        }

        cursor.seek(&query_range.start);

        if let Some(region) = cursor.region().filter(|region| !region.is_main_buffer)
            && region.range.start > MBD::default()
        {
            cursor.prev()
        } else if let Some(region) = cursor.region()
            && region.is_main_buffer
            && region.diff_hunk_status.is_some()
        {
            cursor.prev();
            if cursor.region().is_none_or(|region| region.is_main_buffer) {
                cursor.next();
            }
        }

        iter::from_fn(move || {
            loop {
                let excerpt = cursor.excerpt()?;
                let buffer_snapshot = excerpt.buffer_snapshot(self);

                // If we have already retrieved metadata for this excerpt, continue to use it.
                let metadata_iter = if let Some((_, metadata)) = current_excerpt_metadata
                    .as_mut()
                    .filter(|(excerpt_info, _)| excerpt_info == &excerpt.range)
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
                            buffer_start = buffer_start + overshoot;
                        }
                        buffer_start = buffer_start.min(region.buffer_range.end);
                    } else {
                        buffer_start = cursor.main_buffer_position()?;
                    };
                    let mut buffer_end = excerpt
                        .range
                        .context
                        .end
                        .summary::<MBD::TextDimension>(&buffer_snapshot);
                    if let Some((end_excerpt, end_buffer_offset)) = &range_end
                        && &excerpt.range == end_excerpt
                    {
                        buffer_end = buffer_end.min(*end_buffer_offset);
                    }

                    get_buffer_metadata(&buffer_snapshot, buffer_start..buffer_end).map(
                        |iterator| {
                            &mut current_excerpt_metadata
                                .insert((excerpt.range.clone(), iterator))
                                .1
                        },
                    )
                };

                // Visit each metadata item.
                if let Some((metadata_buffer_range, metadata)) =
                    metadata_iter.and_then(Iterator::next)
                {
                    // Find the multibuffer regions that contain the start and end of
                    // the metadata item's range.
                    if metadata_buffer_range.start > <MBD::TextDimension>::default() {
                        while let Some(region) = cursor.region() {
                            if (region.is_main_buffer
                                && (region.buffer_range.end >= metadata_buffer_range.start
                                    || cursor.is_at_end_of_excerpt()))
                                || (!region.is_main_buffer
                                    && region.buffer_range.start == metadata_buffer_range.start)
                            {
                                break;
                            }
                            cursor.next();
                        }
                    }
                    let start_region = cursor.region()?.clone();
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
                        start_position =
                            start_position + (metadata_buffer_range.start - region_buffer_start);
                        start_position = start_position.min(start_region.range.end);
                    }

                    let mut end_position = self.max_position();
                    if let Some(end_region) = &end_region {
                        end_position = end_region.range.start;
                        debug_assert!(end_region.is_main_buffer);
                        let region_buffer_start = end_region.buffer_range.start;
                        if metadata_buffer_range.end > region_buffer_start {
                            end_position =
                                end_position + (metadata_buffer_range.end - region_buffer_start);
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
                    if let Some((end_excerpt, _)) = &range_end
                        && &excerpt.range == end_excerpt
                    {
                        return None;
                    }
                    cursor.next_excerpt_forwards();
                }
            }
        })
    }

    pub fn diff_hunk_before<T: ToOffset>(&self, position: T) -> Option<MultiBufferRow> {
        let offset = position.to_offset(self);

        let mut cursor = self
            .cursor::<DimensionPair<MultiBufferOffset, Point>, DimensionPair<BufferOffset, Point>>(
            );
        cursor.seek(&DimensionPair {
            key: offset,
            value: None,
        });
        cursor.seek_to_start_of_current_excerpt();
        let excerpt = cursor.excerpt()?;

        let buffer = excerpt.buffer_snapshot(self);
        let excerpt_start = excerpt.range.context.start.to_offset(buffer);
        let excerpt_end = excerpt.range.context.end.to_offset(buffer);
        let current_position = match self.anchor_before(offset) {
            Anchor::Min => 0,
            Anchor::Excerpt(excerpt_anchor) => excerpt_anchor.text_anchor().to_offset(buffer),
            Anchor::Max => unreachable!(),
        };

        if let Some(diff) = self.diff_state(excerpt.buffer_id) {
            if let Some(main_buffer) = &diff.main_buffer {
                for hunk in diff
                    .hunks_intersecting_base_text_range_rev(excerpt_start..excerpt_end, main_buffer)
                {
                    if hunk.diff_base_byte_range.end >= current_position {
                        continue;
                    }
                    let hunk_start = buffer.anchor_after(hunk.diff_base_byte_range.start);
                    let start =
                        Anchor::in_buffer(excerpt.path_key_index, hunk_start).to_point(self);
                    return Some(MultiBufferRow(start.row));
                }
            } else {
                let excerpt_end = buffer.anchor_before(excerpt_end.min(current_position));
                for hunk in diff
                    .hunks_intersecting_range_rev(excerpt.range.context.start..excerpt_end, buffer)
                {
                    let hunk_end = hunk.buffer_range.end.to_offset(buffer);
                    if hunk_end >= current_position {
                        continue;
                    }
                    let start = Anchor::in_buffer(excerpt.path_key_index, hunk.buffer_range.start)
                        .to_point(self);
                    return Some(MultiBufferRow(start.row));
                }
            }
        }

        loop {
            cursor.prev_excerpt();
            let excerpt = cursor.excerpt()?;
            let buffer = excerpt.buffer_snapshot(self);

            let Some(diff) = self.diff_state(excerpt.buffer_id) else {
                continue;
            };
            if let Some(main_buffer) = &diff.main_buffer {
                let Some(hunk) = diff
                    .hunks_intersecting_base_text_range_rev(
                        excerpt.range.context.to_offset(buffer),
                        main_buffer,
                    )
                    .next()
                else {
                    continue;
                };
                let hunk_start = buffer.anchor_after(hunk.diff_base_byte_range.start);
                let start = Anchor::in_buffer(excerpt.path_key_index, hunk_start).to_point(self);
                return Some(MultiBufferRow(start.row));
            } else {
                let Some(hunk) = diff
                    .hunks_intersecting_range_rev(excerpt.range.context.clone(), buffer)
                    .next()
                else {
                    continue;
                };
                let start = Anchor::in_buffer(excerpt.path_key_index, hunk.buffer_range.start)
                    .to_point(self);
                return Some(MultiBufferRow(start.row));
            }
        }
    }

    pub fn has_diff_hunks(&self) -> bool {
        self.diffs.iter().any(|diff| !diff.is_empty())
    }

    pub fn is_inside_word<T: ToOffset>(
        &self,
        position: T,
        scope_context: Option<CharScopeContext>,
    ) -> bool {
        let position = position.to_offset(self);
        let classifier = self
            .char_classifier_at(position)
            .scope_context(scope_context);
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
        scope_context: Option<CharScopeContext>,
    ) -> (Range<MultiBufferOffset>, Option<CharKind>) {
        let mut start = start.to_offset(self);
        let mut end = start;
        let mut next_chars = self.chars_at(start).peekable();
        let mut prev_chars = self.reversed_chars_at(start).peekable();

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

    pub fn char_kind_before<T: ToOffset>(
        &self,
        start: T,
        scope_context: Option<CharScopeContext>,
    ) -> Option<CharKind> {
        let start = start.to_offset(self);
        let classifier = self.char_classifier_at(start).scope_context(scope_context);
        self.reversed_chars_at(start)
            .next()
            .map(|ch| classifier.kind(ch))
    }

    pub fn all_buffer_ids(&self) -> impl Iterator<Item = BufferId> + '_ {
        self.buffers.iter().map(|(id, _)| *id)
    }

    pub fn is_singleton(&self) -> bool {
        self.singleton
    }

    pub fn as_singleton(&self) -> Option<&BufferSnapshot> {
        if self.is_singleton() {
            Some(self.excerpts.first()?.buffer_snapshot(&self))
        } else {
            None
        }
    }

    pub fn len(&self) -> MultiBufferOffset {
        self.diff_transforms.summary().output.len
    }

    pub fn max_position<MBD: MultiBufferDimension>(&self) -> MBD {
        MBD::from_summary(&self.text_summary())
    }

    pub fn is_empty(&self) -> bool {
        self.diff_transforms.summary().output.len == MultiBufferOffset(0)
    }

    pub fn widest_line_number(&self) -> u32 {
        // widest_line_number is 0-based, so 1 is added to get the displayed line number.
        self.excerpts.summary().widest_line_number + 1
    }

    pub fn bytes_in_range<T: ToOffset>(&self, range: Range<T>) -> MultiBufferBytes<'_> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut excerpts = self.cursor::<MultiBufferOffset, BufferOffset>();
        excerpts.seek(&range.start);

        let mut chunk;
        let mut has_trailing_newline;
        let excerpt_bytes;
        if let Some(region) = excerpts.region() {
            let mut bytes = region.buffer.bytes_in_range(
                region.buffer_range.start + (range.start - region.range.start)
                    ..(region.buffer_range.start + (range.end - region.range.start))
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
        let mut cursor = self.cursor::<Point, Point>();
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
        language_aware: LanguageAwareStyling,
    ) -> MultiBufferChunks<'_> {
        let mut chunks = MultiBufferChunks {
            excerpt_offset_range: ExcerptDimension(MultiBufferOffset::ZERO)
                ..ExcerptDimension(MultiBufferOffset::ZERO),
            range: MultiBufferOffset::ZERO..MultiBufferOffset::ZERO,
            excerpts: self.excerpts.cursor(()),
            diff_transforms: self.diff_transforms.cursor(()),
            diff_base_chunks: None,
            excerpt_chunks: None,
            buffer_chunk: None,
            language_aware,
            snapshot: self,
        };
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        chunks.seek(range);
        chunks
    }

    pub fn clip_offset(&self, offset: MultiBufferOffset, bias: Bias) -> MultiBufferOffset {
        self.clip_dimension(offset, bias, text::BufferSnapshot::clip_offset)
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        self.clip_dimension(point, bias, text::BufferSnapshot::clip_point)
    }

    pub fn clip_offset_utf16(
        &self,
        offset: MultiBufferOffsetUtf16,
        bias: Bias,
    ) -> MultiBufferOffsetUtf16 {
        self.clip_dimension(offset, bias, text::BufferSnapshot::clip_offset_utf16)
    }

    pub fn clip_point_utf16(&self, point: Unclipped<PointUtf16>, bias: Bias) -> PointUtf16 {
        self.clip_dimension(point.0, bias, |buffer, point, bias| {
            buffer.clip_point_utf16(Unclipped(point), bias)
        })
    }

    pub fn offset_to_point(&self, offset: MultiBufferOffset) -> Point {
        self.convert_dimension(offset, text::BufferSnapshot::offset_to_point)
    }

    pub fn offset_to_point_utf16(&self, offset: MultiBufferOffset) -> PointUtf16 {
        self.convert_dimension(offset, text::BufferSnapshot::offset_to_point_utf16)
    }

    pub fn point_to_point_utf16(&self, point: Point) -> PointUtf16 {
        self.convert_dimension(point, text::BufferSnapshot::point_to_point_utf16)
    }

    pub fn point_utf16_to_point(&self, point: PointUtf16) -> Point {
        self.convert_dimension(point, text::BufferSnapshot::point_utf16_to_point)
    }

    #[instrument(skip_all)]
    pub fn point_to_offset(&self, point: Point) -> MultiBufferOffset {
        self.convert_dimension(point, text::BufferSnapshot::point_to_offset)
    }

    pub fn point_to_offset_utf16(&self, point: Point) -> MultiBufferOffsetUtf16 {
        self.convert_dimension(point, text::BufferSnapshot::point_to_offset_utf16)
    }

    pub fn offset_utf16_to_offset(&self, offset: MultiBufferOffsetUtf16) -> MultiBufferOffset {
        self.convert_dimension(offset, text::BufferSnapshot::offset_utf16_to_offset)
    }

    pub fn offset_to_offset_utf16(&self, offset: MultiBufferOffset) -> MultiBufferOffsetUtf16 {
        self.convert_dimension(offset, text::BufferSnapshot::offset_to_offset_utf16)
    }

    pub fn point_utf16_to_offset(&self, point: PointUtf16) -> MultiBufferOffset {
        self.convert_dimension(point, text::BufferSnapshot::point_utf16_to_offset)
    }

    pub fn point_utf16_to_offset_utf16(&self, point: PointUtf16) -> MultiBufferOffsetUtf16 {
        self.convert_dimension(point, text::BufferSnapshot::point_utf16_to_offset_utf16)
    }

    fn clip_dimension<MBD, BD>(
        &self,
        position: MBD,
        bias: Bias,
        clip_buffer_position: fn(&text::BufferSnapshot, BD, Bias) -> BD,
    ) -> MBD
    where
        MBD: MultiBufferDimension + Ord + Sub + ops::AddAssign<<MBD as Sub>::Output>,
        BD: TextDimension + Sub<Output = <MBD as Sub>::Output> + AddAssign<<MBD as Sub>::Output>,
    {
        let mut cursor = self.cursor::<MBD, BD>();
        cursor.seek(&position);
        if let Some(region) = cursor.region() {
            if position >= region.range.end {
                return region.range.end;
            }
            let overshoot = position - region.range.start;
            let mut buffer_position = region.buffer_range.start;
            buffer_position += overshoot;
            let clipped_buffer_position =
                clip_buffer_position(region.buffer, buffer_position, bias);
            let mut position = region.range.start;
            position += clipped_buffer_position - region.buffer_range.start;
            position
        } else {
            self.max_position()
        }
    }

    #[instrument(skip_all)]
    fn convert_dimension<MBR1, MBR2, BR1, BR2>(
        &self,
        key: MBR1,
        convert_buffer_dimension: fn(&text::BufferSnapshot, BR1) -> BR2,
    ) -> MBR2
    where
        MBR1: MultiBufferDimension + Ord + Sub + ops::AddAssign<<MBR1 as Sub>::Output>,
        BR1: TextDimension + Sub<Output = <MBR1 as Sub>::Output> + AddAssign<<MBR1 as Sub>::Output>,
        MBR2: MultiBufferDimension + Ord + Sub + ops::AddAssign<<MBR2 as Sub>::Output>,
        BR2: TextDimension + Sub<Output = <MBR2 as Sub>::Output> + AddAssign<<MBR2 as Sub>::Output>,
    {
        let mut cursor = self.cursor::<DimensionPair<MBR1, MBR2>, DimensionPair<BR1, BR2>>();
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
            buffer_key += key - start_key;
            let buffer_value = convert_buffer_dimension(region.buffer, buffer_key);
            let mut result = start_value;
            result += buffer_value - buffer_start_value;
            result
        } else {
            self.max_position()
        }
    }

    pub fn point_to_buffer_offset<T: ToOffset>(
        &self,
        point: T,
    ) -> Option<(&BufferSnapshot, BufferOffset)> {
        let offset = point.to_offset(self);
        let mut cursor = self.cursor::<MultiBufferOffset, BufferOffset>();
        cursor.seek(&offset);
        let region = cursor.region()?;
        let overshoot = offset - region.range.start;
        let buffer_offset = region.buffer_range.start + overshoot;
        if buffer_offset == BufferOffset(region.buffer.len() + 1)
            && region.has_trailing_newline
            && !region.is_main_buffer
        {
            let main_buffer_position = cursor.main_buffer_position()?;
            let buffer_snapshot = cursor.excerpt()?.buffer_snapshot(self);
            return Some((buffer_snapshot, main_buffer_position));
        } else if buffer_offset > BufferOffset(region.buffer.len()) {
            return None;
        }
        Some((region.buffer, buffer_offset))
    }

    pub fn point_to_buffer_point(&self, point: Point) -> Option<(&BufferSnapshot, Point)> {
        let mut cursor = self.cursor::<Point, Point>();
        cursor.seek(&point);
        let region = cursor.region()?;
        let overshoot = point - region.range.start;
        let buffer_point = region.buffer_range.start + overshoot;
        let excerpt = cursor.excerpt()?;
        if buffer_point == region.buffer.max_point() + Point::new(1, 0)
            && region.has_trailing_newline
            && !region.is_main_buffer
        {
            return Some((
                &excerpt.buffer_snapshot(self),
                cursor.main_buffer_position()?,
            ));
        } else if buffer_point > region.buffer.max_point() {
            return None;
        }
        Some((region.buffer, buffer_point))
    }

    pub fn suggested_indents(
        &self,
        rows: impl IntoIterator<Item = u32>,
        cx: &App,
    ) -> BTreeMap<MultiBufferRow, IndentSize> {
        let mut result = BTreeMap::new();
        self.suggested_indents_callback(
            rows,
            &mut |row, indent| {
                result.insert(row, indent);
                ControlFlow::Continue(())
            },
            cx,
        );
        result
    }

    // move this to be a generator once those are a thing
    pub fn suggested_indents_callback(
        &self,
        rows: impl IntoIterator<Item = u32>,
        cb: &mut dyn FnMut(MultiBufferRow, IndentSize) -> ControlFlow<()>,
        cx: &App,
    ) {
        let mut rows_for_excerpt = Vec::new();
        let mut cursor = self.cursor::<Point, Point>();
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
            for (row, indent) in buffer_indents {
                if cb(
                    MultiBufferRow(start_multibuffer_row + row - start_buffer_row),
                    indent,
                )
                .is_break()
                {
                    return;
                }
            }
        }
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

    pub fn line_len_utf16(&self, row: MultiBufferRow) -> u32 {
        self.clip_point_utf16(Unclipped(PointUtf16::new(row.0, u32::MAX)), Bias::Left)
            .column
    }

    pub fn buffer_line_for_row(
        &self,
        row: MultiBufferRow,
    ) -> Option<(&BufferSnapshot, Range<Point>)> {
        let mut cursor = self.cursor::<Point, Point>();
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

    pub fn text_summary(&self) -> MBTextSummary {
        self.diff_transforms.summary().output
    }

    pub fn text_summary_for_range<MBD, O>(&self, range: Range<O>) -> MBD
    where
        MBD: MultiBufferDimension + AddAssign,
        O: ToOffset,
    {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self
            .diff_transforms
            .cursor::<Dimensions<MultiBufferOffset, ExcerptOffset>>(());
        cursor.seek(&range.start, Bias::Right);

        let Some(first_transform) = cursor.item() else {
            return MBD::from_summary(&MBTextSummary::default());
        };

        let diff_transform_start = cursor.start().0;
        let diff_transform_end = cursor.end().0;
        let diff_start = range.start;
        let start_overshoot = diff_start - diff_transform_start;
        let end_overshoot = std::cmp::min(range.end, diff_transform_end) - diff_transform_start;

        let mut result = match first_transform {
            DiffTransform::BufferContent { .. } => {
                let excerpt_start = cursor.start().1 + start_overshoot;
                let excerpt_end = cursor.start().1 + end_overshoot;
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
                let Some(base_text) = self.diff_state(*buffer_id).map(|diff| diff.base_text())
                else {
                    panic!("{:?} is in non-existent deleted hunk", range.start)
                };

                let include_trailing_newline =
                    *has_trailing_newline && range.end >= diff_transform_end;
                if include_trailing_newline {
                    buffer_end -= 1;
                }

                let mut summary = base_text
                    .text_summary_for_range::<MBD::TextDimension, _>(buffer_start..buffer_end);

                if include_trailing_newline {
                    summary.add_assign(&<MBD::TextDimension>::from_text_summary(
                        &TextSummary::newline(),
                    ))
                }

                let mut result = MBD::default();
                result.add_text_dim(&summary);
                result
            }
        };
        if range.end < diff_transform_end {
            return result;
        }

        cursor.next();
        result.add_mb_text_summary(
            &cursor
                .summary::<_, OutputDimension<_>>(&range.end, Bias::Right)
                .0,
        );

        let Some(last_transform) = cursor.item() else {
            return result;
        };

        let overshoot = range.end - cursor.start().0;
        let suffix = match last_transform {
            DiffTransform::BufferContent { .. } => {
                let end = cursor.start().1 + overshoot;
                self.text_summary_for_excerpt_offset_range::<MBD>(cursor.start().1..end)
            }
            DiffTransform::DeletedHunk {
                base_text_byte_range,
                buffer_id,
                has_trailing_newline,
                ..
            } => {
                let buffer_end = base_text_byte_range.start + overshoot;
                let Some(base_text) = self.diff_state(*buffer_id).map(|diff| diff.base_text())
                else {
                    panic!("{:?} is in non-existent deleted hunk", range.end)
                };

                let mut suffix = base_text.text_summary_for_range::<MBD::TextDimension, _>(
                    base_text_byte_range.start..buffer_end,
                );
                if *has_trailing_newline && buffer_end == base_text_byte_range.end + 1 {
                    suffix.add_assign(&<MBD::TextDimension>::from_text_summary(
                        &TextSummary::from("\n"),
                    ))
                }

                let mut result = MBD::default();
                result.add_text_dim(&suffix);
                result
            }
        };

        result += suffix;
        result
    }

    fn text_summary_for_excerpt_offset_range<MBD>(&self, mut range: Range<ExcerptOffset>) -> MBD
    where
        MBD: MultiBufferDimension + AddAssign,
    {
        let mut summary = MBD::default();
        let mut cursor = self.excerpts.cursor::<ExcerptOffset>(());
        cursor.seek(&range.start, Bias::Right);
        if let Some(excerpt) = cursor.item() {
            let buffer_snapshot = excerpt.buffer_snapshot(self);
            let mut end_before_newline = cursor.end();
            if excerpt.has_trailing_newline {
                end_before_newline -= 1;
            }

            let excerpt_start = excerpt.range.context.start.to_offset(&buffer_snapshot);
            let start_in_excerpt = excerpt_start + (range.start - *cursor.start());
            let end_in_excerpt =
                excerpt_start + (cmp::min(end_before_newline, range.end) - *cursor.start());
            summary.add_text_dim(
                &buffer_snapshot.text_summary_for_range::<MBD::TextDimension, _>(
                    start_in_excerpt..end_in_excerpt,
                ),
            );

            if range.end > end_before_newline {
                summary.add_mb_text_summary(&MBTextSummary::from(TextSummary::newline()));
            }

            cursor.next();
        }

        if range.end > *cursor.start() {
            summary += cursor
                .summary::<_, ExcerptDimension<MBD>>(&range.end, Bias::Right)
                .0;
            if let Some(excerpt) = cursor.item() {
                let buffer_snapshot = excerpt.buffer_snapshot(self);
                range.end = cmp::max(*cursor.start(), range.end);

                let excerpt_start = excerpt.range.context.start.to_offset(&buffer_snapshot);
                let end_in_excerpt = excerpt_start + (range.end - *cursor.start());
                summary.add_text_dim(
                    &buffer_snapshot.text_summary_for_range::<MBD::TextDimension, _>(
                        excerpt_start..end_in_excerpt,
                    ),
                );
            }
        }

        summary
    }

    pub fn summary_for_anchor<MBD>(&self, anchor: &Anchor) -> MBD
    where
        MBD: MultiBufferDimension
            + Ord
            + Sub<Output = MBD::TextDimension>
            + Sub<MBD::TextDimension, Output = MBD>
            + AddAssign<MBD::TextDimension>
            + Add<MBD::TextDimension, Output = MBD>,
        MBD::TextDimension: Sub<Output = MBD::TextDimension> + Ord,
    {
        let target = anchor.seek_target(self);
        let anchor = match anchor {
            Anchor::Min => {
                return MBD::default();
            }
            Anchor::Excerpt(excerpt_anchor) => excerpt_anchor,
            Anchor::Max => {
                return MBD::from_summary(&self.text_summary());
            }
        };

        let (start, _, item) = self
            .excerpts
            .find::<ExcerptSummary, _>((), &target, Bias::Left);
        let start = MBD::from_summary(&start.text);

        let excerpt_start_position = ExcerptDimension(start);
        if self.diff_transforms.is_empty() {
            if let Some(excerpt) = item {
                if !excerpt.contains(anchor, self) {
                    return excerpt_start_position.0;
                }
                let buffer_snapshot = excerpt.buffer_snapshot(self);
                let excerpt_buffer_start = excerpt
                    .range
                    .context
                    .start
                    .summary::<MBD::TextDimension>(&buffer_snapshot);
                let excerpt_buffer_end = excerpt
                    .range
                    .context
                    .end
                    .summary::<MBD::TextDimension>(&buffer_snapshot);
                let buffer_summary = anchor
                    .text_anchor()
                    .summary::<MBD::TextDimension>(&buffer_snapshot);
                let summary = cmp::min(excerpt_buffer_end, buffer_summary);
                let mut position = excerpt_start_position;
                if summary > excerpt_buffer_start {
                    position += summary - excerpt_buffer_start;
                }

                position.0
            } else {
                excerpt_start_position.0
            }
        } else {
            let mut diff_transforms_cursor = self
                .diff_transforms
                .cursor::<Dimensions<ExcerptDimension<MBD>, OutputDimension<MBD>>>(());

            if let Some(excerpt) = item {
                if !excerpt.contains(anchor, self) {
                    diff_transforms_cursor.seek(&excerpt_start_position, Bias::Left);
                    return self.summary_for_excerpt_position_without_hunks(
                        Bias::Left,
                        excerpt_start_position,
                        &mut diff_transforms_cursor,
                    );
                }
                let buffer_snapshot = excerpt.buffer_snapshot(self);
                let excerpt_buffer_start = excerpt
                    .range
                    .context
                    .start
                    .summary::<MBD::TextDimension>(&buffer_snapshot);
                let excerpt_buffer_end = excerpt
                    .range
                    .context
                    .end
                    .summary::<MBD::TextDimension>(&buffer_snapshot);
                let buffer_summary = anchor
                    .text_anchor()
                    .summary::<MBD::TextDimension>(&buffer_snapshot);
                let summary = cmp::min(excerpt_buffer_end, buffer_summary);
                let mut position = excerpt_start_position;
                if summary > excerpt_buffer_start {
                    position += summary - excerpt_buffer_start;
                }

                diff_transforms_cursor.seek(&position, Bias::Left);
                self.summary_for_anchor_with_excerpt_position(
                    *anchor,
                    position,
                    &mut diff_transforms_cursor,
                    &buffer_snapshot,
                )
            } else {
                diff_transforms_cursor.seek(&excerpt_start_position, Bias::Left);
                self.summary_for_excerpt_position_without_hunks(
                    Bias::Right,
                    excerpt_start_position,
                    &mut diff_transforms_cursor,
                )
            }
        }
    }

    /// Maps an anchor's excerpt-space position to its output-space position by
    /// walking the diff transforms. The cursor is shared across consecutive
    /// calls, so it may already be partway through the transform list.
    fn summary_for_anchor_with_excerpt_position<MBD>(
        &self,
        anchor: ExcerptAnchor,
        excerpt_position: ExcerptDimension<MBD>,
        diff_transforms: &mut Cursor<
            DiffTransform,
            Dimensions<ExcerptDimension<MBD>, OutputDimension<MBD>>,
        >,
        excerpt_buffer: &text::BufferSnapshot,
    ) -> MBD
    where
        MBD: MultiBufferDimension + Ord + Sub + AddAssign<<MBD as Sub>::Output>,
    {
        loop {
            let transform_end_position = diff_transforms.end().0;
            let item = diff_transforms.item();
            let at_transform_end = transform_end_position == excerpt_position && item.is_some();

            // A right-biased anchor at a transform boundary belongs to the
            // *next* transform, so advance past the current one.
            if anchor.text_anchor.bias == Bias::Right && at_transform_end {
                diff_transforms.next();
                continue;
            }

            let mut position = diff_transforms.start().1;
            match item {
                Some(DiffTransform::DeletedHunk {
                    buffer_id,
                    base_text_byte_range,
                    hunk_info,
                    ..
                }) => {
                    if let Some(diff_base_anchor) = anchor.diff_base_anchor
                        && let Some(base_text) =
                            self.diff_state(*buffer_id).map(|diff| diff.base_text())
                        && diff_base_anchor.is_valid(&base_text)
                    {
                        // The anchor carries a diff-base position — resolve it
                        // to a location inside the deleted hunk.
                        let base_text_offset = diff_base_anchor.to_offset(base_text);
                        if base_text_offset >= base_text_byte_range.start
                            && base_text_offset <= base_text_byte_range.end
                        {
                            let position_in_hunk = base_text
                                .text_summary_for_range::<MBD::TextDimension, _>(
                                    base_text_byte_range.start..base_text_offset,
                                );
                            position.0.add_text_dim(&position_in_hunk);
                        } else if at_transform_end {
                            // diff_base offset falls outside this hunk's range;
                            // advance to see if the next transform is a better fit.
                            diff_transforms.next();
                            continue;
                        }
                    } else if at_transform_end
                        && anchor
                            .text_anchor()
                            .cmp(&hunk_info.hunk_start_anchor, excerpt_buffer)
                            .is_gt()
                    {
                        // The anchor has no (valid) diff-base position, so it
                        // belongs in the buffer content, not in the deleted
                        // hunk. However, after an edit deletes the text between
                        // the hunk boundary and this anchor, both resolve to
                        // the same excerpt_position—landing us here on the
                        // DeletedHunk left behind by the shared cursor. Use the
                        // CRDT ordering to detect that the anchor is strictly
                        // *past* the hunk boundary and skip to the following
                        // BufferContent.
                        diff_transforms.next();
                        continue;
                    }
                }
                _ => {
                    // On a BufferContent (or no transform). If the anchor
                    // carries a diff_base_anchor it needs a DeletedHunk, so
                    // advance to find one.
                    if at_transform_end && anchor.diff_base_anchor.is_some() {
                        diff_transforms.next();
                        continue;
                    }
                    let overshoot = excerpt_position - diff_transforms.start().0;
                    position += overshoot;
                }
            }

            return position.0;
        }
    }

    /// Like `resolve_summary_for_anchor` but optimized for min/max anchors.
    fn summary_for_excerpt_position_without_hunks<MBD>(
        &self,
        bias: Bias,
        excerpt_position: ExcerptDimension<MBD>,
        diff_transforms: &mut Cursor<
            DiffTransform,
            Dimensions<ExcerptDimension<MBD>, OutputDimension<MBD>>,
        >,
    ) -> MBD
    where
        MBD: MultiBufferDimension + Ord + Sub + AddAssign<<MBD as Sub>::Output>,
    {
        loop {
            let transform_end_position = diff_transforms.end().0;
            let item = diff_transforms.item();
            let at_transform_end = transform_end_position == excerpt_position && item.is_some();

            // A right-biased anchor at a transform boundary belongs to the
            // *next* transform, so advance past the current one.
            if bias == Bias::Right && at_transform_end {
                diff_transforms.next();
                continue;
            }

            let mut position = diff_transforms.start().1;
            if let Some(DiffTransform::BufferContent { .. }) | None = item {
                let overshoot = excerpt_position - diff_transforms.start().0;
                position += overshoot;
            }

            return position.0;
        }
    }

    fn excerpt_offset_for_anchor(&self, anchor: &Anchor) -> ExcerptOffset {
        let anchor = match anchor {
            Anchor::Min => return ExcerptOffset::default(),
            Anchor::Excerpt(excerpt_anchor) => excerpt_anchor,
            Anchor::Max => return self.excerpts.summary().len(),
        };
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(());
        let target = anchor.seek_target(self);

        cursor.seek(&target, Bias::Left);

        let mut position = cursor.start().len();
        if let Some(excerpt) = cursor.item()
            && excerpt.contains(anchor, self)
        {
            let buffer_snapshot = excerpt.buffer_snapshot(self);
            let excerpt_buffer_start =
                buffer_snapshot.offset_for_anchor(&excerpt.range.context.start);
            let excerpt_buffer_end = buffer_snapshot.offset_for_anchor(&excerpt.range.context.end);
            let buffer_position = cmp::min(
                excerpt_buffer_end,
                buffer_snapshot.offset_for_anchor(&anchor.text_anchor()),
            );
            if buffer_position > excerpt_buffer_start {
                position += buffer_position - excerpt_buffer_start;
            }
        }
        position
    }

    pub fn summaries_for_anchors<'a, MBD, I>(&'a self, anchors: I) -> Vec<MBD>
    where
        MBD: MultiBufferDimension
            + Ord
            + Sub<Output = MBD::TextDimension>
            + AddAssign<MBD::TextDimension>,
        MBD::TextDimension: Sub<Output = MBD::TextDimension> + Ord,
        I: 'a + IntoIterator<Item = &'a Anchor>,
    {
        let mut anchors = anchors.into_iter().peekable();
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(());
        let mut diff_transforms_cursor = self
            .diff_transforms
            .cursor::<Dimensions<ExcerptDimension<MBD>, OutputDimension<MBD>>>(());
        diff_transforms_cursor.next();

        let mut summaries = Vec::new();
        while let Some(anchor) = anchors.peek() {
            let target = anchor.seek_target(self);
            let excerpt_anchor = match anchor {
                Anchor::Min => {
                    summaries.push(MBD::default());
                    anchors.next();
                    continue;
                }
                Anchor::Excerpt(excerpt_anchor) => excerpt_anchor,
                Anchor::Max => {
                    summaries.push(MBD::from_summary(&self.text_summary()));
                    anchors.next();
                    continue;
                }
            };

            cursor.seek_forward(&target, Bias::Left);

            let excerpt_start_position = ExcerptDimension(MBD::from_summary(&cursor.start().text));
            if let Some(excerpt) = cursor.item() {
                let buffer_snapshot = excerpt.buffer_snapshot(self);
                if !excerpt.contains(&excerpt_anchor, self) {
                    diff_transforms_cursor.seek_forward(&excerpt_start_position, Bias::Left);
                    let position = self.summary_for_excerpt_position_without_hunks(
                        Bias::Left,
                        excerpt_start_position,
                        &mut diff_transforms_cursor,
                    );
                    summaries.push(position);
                    anchors.next();
                    continue;
                }
                let excerpt_buffer_start = excerpt
                    .range
                    .context
                    .start
                    .summary::<MBD::TextDimension>(buffer_snapshot);
                let excerpt_buffer_end = excerpt
                    .range
                    .context
                    .end
                    .summary::<MBD::TextDimension>(buffer_snapshot);
                for (buffer_summary, excerpt_anchor) in buffer_snapshot
                    .summaries_for_anchors_with_payload::<MBD::TextDimension, _, _>(
                        std::iter::from_fn(|| {
                            let excerpt_anchor = anchors.peek()?.excerpt_anchor()?;
                            if !excerpt.contains(&excerpt_anchor, self) {
                                return None;
                            }
                            anchors.next();
                            Some((excerpt_anchor.text_anchor(), excerpt_anchor))
                        }),
                    )
                {
                    let summary = cmp::min(excerpt_buffer_end, buffer_summary);
                    let mut position = excerpt_start_position;
                    if summary > excerpt_buffer_start {
                        position += summary - excerpt_buffer_start;
                    }

                    if diff_transforms_cursor.start().0 < position {
                        diff_transforms_cursor.seek_forward(&position, Bias::Left);
                    }

                    summaries.push(self.summary_for_anchor_with_excerpt_position(
                        excerpt_anchor,
                        position,
                        &mut diff_transforms_cursor,
                        &buffer_snapshot,
                    ));
                }
            } else {
                diff_transforms_cursor.seek_forward(&excerpt_start_position, Bias::Left);
                let position = self.summary_for_excerpt_position_without_hunks(
                    Bias::Right,
                    excerpt_start_position,
                    &mut diff_transforms_cursor,
                );
                summaries.push(position);
                anchors.next();
            }
        }

        summaries
    }

    pub fn dimensions_from_points<'a, MBD>(
        &'a self,
        points: impl 'a + IntoIterator<Item = Point>,
    ) -> impl 'a + Iterator<Item = MBD>
    where
        MBD: MultiBufferDimension + Sub + AddAssign<<MBD as Sub>::Output>,
    {
        let mut cursor = self.cursor::<DimensionPair<Point, MBD>, Point>();
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
                let buffer_point = region.buffer_range.start + overshoot;
                let mut position = region.range.start.value.unwrap();
                position.add_text_dim(
                    &region
                        .buffer
                        .text_summary_for_range(region.buffer_range.start..buffer_point),
                );
                if point == region.range.end.key && region.has_trailing_newline {
                    position.add_mb_text_summary(&MBTextSummary::from(TextSummary::newline()));
                }
                Some(position)
            } else {
                Some(MBD::from_summary(&self.text_summary()))
            }
        })
    }

    pub fn excerpts_for_buffer(
        &self,
        buffer_id: BufferId,
    ) -> impl Iterator<Item = ExcerptRange<text::Anchor>> {
        if let Some(buffer_state) = self.buffers.get(&buffer_id) {
            let path_key = buffer_state.path_key.clone();
            let mut cursor = self.excerpts.cursor::<PathKey>(());
            cursor.seek_forward(&path_key, Bias::Left);
            Some(iter::from_fn(move || {
                let excerpt = cursor.item()?;
                if excerpt.path_key != path_key {
                    return None;
                }
                cursor.next();
                Some(excerpt.range.clone())
            }))
        } else {
            None
        }
        .into_iter()
        .flatten()
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
            .cursor::<Dimensions<MultiBufferOffset, ExcerptOffset>>(());
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
            let diff = self.diff_state(*buffer_id).expect("missing diff");
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
            excerpt_offset += MultiBufferOffset(offset_in_transform);
        };

        let mut excerpts = self
            .excerpts
            .cursor::<Dimensions<ExcerptOffset, ExcerptSummary>>(());
        excerpts.seek(&excerpt_offset, Bias::Right);
        if excerpts.item().is_none() && excerpt_offset == excerpts.start().0 && bias == Bias::Left {
            excerpts.prev();
        }
        if let Some(excerpt) = excerpts.item() {
            let buffer_snapshot = excerpt.buffer_snapshot(self);
            let mut overshoot = excerpt_offset.saturating_sub(excerpts.start().0);
            if excerpt.has_trailing_newline && excerpt_offset == excerpts.end().0 {
                overshoot -= 1;
                bias = Bias::Right;
            }

            let buffer_start = excerpt.range.context.start.to_offset(&buffer_snapshot);
            let text_anchor = excerpt.clip_anchor(
                buffer_snapshot.anchor_at(buffer_start + overshoot, bias),
                self,
            );
            let anchor = ExcerptAnchor::in_buffer(excerpt.path_key_index, text_anchor);
            let anchor = match diff_base_anchor {
                Some(diff_base_anchor) => anchor.with_diff_base_anchor(diff_base_anchor),
                None => anchor,
            };
            anchor.into()
        } else if excerpt_offset == ExcerptDimension(MultiBufferOffset::ZERO) && bias == Bias::Left
        {
            Anchor::Min
        } else {
            Anchor::Max
        }
    }

    /// Lifts a buffer anchor to a multibuffer anchor without checking against excerpt boundaries. Returns `None` if there are no excerpts for the buffer
    pub fn anchor_in_buffer(&self, anchor: text::Anchor) -> Option<Anchor> {
        let path_key_index = self.path_key_index_for_buffer(anchor.buffer_id)?;
        Some(Anchor::in_buffer(path_key_index, anchor))
    }

    /// Creates a multibuffer anchor for the given buffer anchor, if it is contained in any excerpt.
    pub fn anchor_in_excerpt(&self, text_anchor: text::Anchor) -> Option<Anchor> {
        for excerpt in {
            let this = &self;
            let buffer_id = text_anchor.buffer_id;
            if let Some(buffer_state) = this.buffers.get(&buffer_id) {
                let path_key = buffer_state.path_key.clone();
                let mut cursor = this.excerpts.cursor::<PathKey>(());
                cursor.seek_forward(&path_key, Bias::Left);
                Some(iter::from_fn(move || {
                    let excerpt = cursor.item()?;
                    if excerpt.path_key != path_key {
                        return None;
                    }
                    cursor.next();
                    Some(excerpt)
                }))
            } else {
                None
            }
            .into_iter()
            .flatten()
        } {
            let buffer_snapshot = excerpt.buffer_snapshot(self);
            if excerpt.range.contains(&text_anchor, &buffer_snapshot) {
                return Some(Anchor::in_buffer(excerpt.path_key_index, text_anchor));
            }
        }

        None
    }

    /// Creates a multibuffer anchor for the given buffer anchor, if it is contained in any excerpt.
    pub fn buffer_anchor_range_to_anchor_range(
        &self,
        text_anchor: Range<text::Anchor>,
    ) -> Option<Range<Anchor>> {
        for excerpt in {
            let this = &self;
            let buffer_id = text_anchor.start.buffer_id;
            if let Some(buffer_state) = this.buffers.get(&buffer_id) {
                let path_key = buffer_state.path_key.clone();
                let mut cursor = this.excerpts.cursor::<PathKey>(());
                cursor.seek_forward(&path_key, Bias::Left);
                Some(iter::from_fn(move || {
                    let excerpt = cursor.item()?;
                    if excerpt.path_key != path_key {
                        return None;
                    }
                    cursor.next();
                    Some(excerpt)
                }))
            } else {
                None
            }
            .into_iter()
            .flatten()
        } {
            let buffer_snapshot = excerpt.buffer_snapshot(self);
            if excerpt.range.contains(&text_anchor.start, &buffer_snapshot)
                && excerpt.range.contains(&text_anchor.end, &buffer_snapshot)
            {
                return Some(Anchor::range_in_buffer(excerpt.path_key_index, text_anchor));
            }
        }

        None
    }

    /// Returns a buffer anchor and its buffer snapshot for the given anchor, if it is in the multibuffer.
    pub fn anchor_to_buffer_anchor(
        &self,
        anchor: Anchor,
    ) -> Option<(text::Anchor, &BufferSnapshot)> {
        match anchor {
            Anchor::Min => {
                let excerpt = self.excerpts.first()?;
                let buffer = excerpt.buffer_snapshot(self);
                Some((excerpt.range.context.start, buffer))
            }
            Anchor::Excerpt(excerpt_anchor) => {
                let buffer = self.buffer_for_id(excerpt_anchor.buffer_id())?;
                Some((excerpt_anchor.text_anchor, buffer))
            }
            Anchor::Max => {
                let excerpt = self.excerpts.last()?;
                let buffer = excerpt.buffer_snapshot(self);
                Some((excerpt.range.context.end, buffer))
            }
        }
    }

    pub fn can_resolve(&self, anchor: &Anchor) -> bool {
        match anchor {
            // todo(lw): should be `!self.is_empty()`
            Anchor::Min | Anchor::Max => true,
            Anchor::Excerpt(excerpt_anchor) => {
                let Some(target) = excerpt_anchor.try_seek_target(self) else {
                    return false;
                };
                let mut cursor = self.excerpts.cursor::<ExcerptSummary>(());
                cursor.seek(&target, Bias::Left);
                let Some(excerpt) = cursor.item() else {
                    return false;
                };
                excerpt
                    .buffer_snapshot(self)
                    .can_resolve(&excerpt_anchor.text_anchor())
            }
        }
    }

    pub fn excerpts(&self) -> impl Iterator<Item = ExcerptRange<text::Anchor>> {
        self.excerpts.iter().map(|excerpt| excerpt.range.clone())
    }

    fn cursor<'a, MBD, BD>(&'a self) -> MultiBufferCursor<'a, MBD, BD>
    where
        MBD: MultiBufferDimension + Ord + Sub + ops::AddAssign<<MBD as Sub>::Output>,
        BD: TextDimension + AddAssign<<MBD as Sub>::Output>,
    {
        let excerpts = self.excerpts.cursor(());
        let diff_transforms = self.diff_transforms.cursor(());
        MultiBufferCursor {
            excerpts,
            diff_transforms,
            cached_region: OnceCell::new(),
            snapshot: self,
        }
    }

    pub fn excerpt_before(&self, anchor: Anchor) -> Option<ExcerptRange<text::Anchor>> {
        let target = anchor.try_seek_target(&self)?;
        let mut excerpts = self.excerpts.cursor::<ExcerptSummary>(());
        excerpts.seek(&target, Bias::Left);
        excerpts.prev();
        Some(excerpts.item()?.range.clone())
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
                start_offset = MultiBufferOffset::ZERO;
                Bound::Unbounded
            }
        };
        let end = match range.end_bound() {
            Bound::Included(end) => Bound::Included(end.to_offset(self)),
            Bound::Excluded(end) => Bound::Excluded(end.to_offset(self)),
            Bound::Unbounded => Bound::Unbounded,
        };
        let bounds = (start, end);
        let mut cursor = self.cursor::<DimensionPair<MultiBufferOffset, Point>, BufferOffset>();
        cursor.seek(&DimensionPair {
            key: start_offset,
            value: None,
        });

        if cursor
            .fetch_excerpt_with_range()
            .is_some_and(|(_, range)| bounds.contains(&range.start.key))
        {
            cursor.prev_excerpt();
        } else {
            cursor.seek_to_start_of_current_excerpt();
        }
        let mut prev_excerpt = cursor
            .fetch_excerpt_with_range()
            .map(|(excerpt, _)| excerpt);

        cursor.next_excerpt_forwards();

        iter::from_fn(move || {
            loop {
                if self.singleton {
                    return None;
                }

                let (next_excerpt, next_range) = cursor.fetch_excerpt_with_range()?;
                cursor.next_excerpt_forwards();
                if !bounds.contains(&next_range.start.key) {
                    prev_excerpt = Some(next_excerpt);
                    continue;
                }

                let next_region_start = next_range.start.value.unwrap();
                let next_region_end = if let Some((_, range)) = cursor.fetch_excerpt_with_range() {
                    range.start.value.unwrap()
                } else {
                    self.max_point()
                };

                let prev = prev_excerpt.as_ref().map(|excerpt| ExcerptBoundaryInfo {
                    start_anchor: Anchor::in_buffer(
                        excerpt.path_key_index,
                        excerpt.range.context.start,
                    ),
                    range: excerpt.range.clone(),
                    end_row: MultiBufferRow(next_region_start.row),
                });

                let next = ExcerptBoundaryInfo {
                    start_anchor: Anchor::in_buffer(
                        next_excerpt.path_key_index,
                        next_excerpt.range.context.start,
                    ),
                    range: next_excerpt.range.clone(),
                    end_row: if next_excerpt.has_trailing_newline {
                        MultiBufferRow(next_region_end.row - 1)
                    } else {
                        MultiBufferRow(next_region_end.row)
                    },
                };

                let row = MultiBufferRow(next_region_start.row);

                prev_excerpt = Some(next_excerpt);

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

    /// Allows converting several ranges within the same excerpt between buffer offsets and multibuffer offsets.
    ///
    /// If the input range is contained in a single excerpt, invokes the callback with the full range of that excerpt
    /// and the input range both converted to buffer coordinates. The buffer ranges returned by the callback are lifted back
    /// to multibuffer offsets and returned.
    ///
    /// Returns `None` if the input range spans multiple excerpts.
    pub fn map_excerpt_ranges<'a, T>(
        &'a self,
        position: Range<MultiBufferOffset>,
        f: impl FnOnce(
            &'a BufferSnapshot,
            ExcerptRange<BufferOffset>,
            Range<BufferOffset>,
        ) -> Vec<(Range<BufferOffset>, T)>,
    ) -> Option<Vec<(Range<MultiBufferOffset>, T)>> {
        let mut cursor = self.cursor::<MultiBufferOffset, BufferOffset>();
        cursor.seek(&position.start);

        let region = cursor.region()?;
        if !region.is_main_buffer {
            return None;
        }
        let excerpt = cursor.excerpt()?;
        let excerpt_start = *cursor.excerpts.start();
        let input_buffer_start = cursor.buffer_position_at(&position.start)?;

        cursor.seek_forward(&position.end);
        if cursor.excerpt()? != excerpt {
            return None;
        }
        let region = cursor.region()?;
        if !region.is_main_buffer {
            return None;
        }
        let input_buffer_end = cursor.buffer_position_at(&position.end)?;
        let input_buffer_range = input_buffer_start..input_buffer_end;
        let buffer = excerpt.buffer_snapshot(self);
        let excerpt_context_range = excerpt.range.context.to_offset(buffer);
        let excerpt_context_range =
            BufferOffset(excerpt_context_range.start)..BufferOffset(excerpt_context_range.end);
        let excerpt_primary_range = excerpt.range.primary.to_offset(buffer);
        let excerpt_primary_range =
            BufferOffset(excerpt_primary_range.start)..BufferOffset(excerpt_primary_range.end);
        let results = f(
            buffer,
            ExcerptRange {
                context: excerpt_context_range.clone(),
                primary: excerpt_primary_range,
            },
            input_buffer_range,
        );
        let mut diff_transforms = cursor.diff_transforms;
        Some(
            results
                .into_iter()
                .map(|(buffer_range, metadata)| {
                    let clamped_start = buffer_range
                        .start
                        .max(excerpt_context_range.start)
                        .min(excerpt_context_range.end);
                    let clamped_end = buffer_range
                        .end
                        .max(clamped_start)
                        .min(excerpt_context_range.end);
                    let excerpt_offset_start =
                        excerpt_start + (clamped_start.0 - excerpt_context_range.start.0);
                    let excerpt_offset_end =
                        excerpt_start + (clamped_end.0 - excerpt_context_range.start.0);

                    diff_transforms.seek(&excerpt_offset_start, Bias::Right);
                    let mut output_start = diff_transforms.start().output_dimension;
                    output_start +=
                        excerpt_offset_start - diff_transforms.start().excerpt_dimension;

                    diff_transforms.seek_forward(&excerpt_offset_end, Bias::Right);
                    let mut output_end = diff_transforms.start().output_dimension;
                    output_end += excerpt_offset_end - diff_transforms.start().excerpt_dimension;

                    (output_start.0..output_end.0, metadata)
                })
                .collect(),
        )
    }

    /// Returns the smallest enclosing bracket ranges containing the given range or
    /// None if no brackets contain range or the range is not contained in a single
    /// excerpt
    ///
    /// Can optionally pass a range_filter to filter the ranges of brackets to consider
    #[ztracing::instrument(skip_all)]
    pub fn innermost_enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
        range_filter: Option<
            &dyn Fn(&BufferSnapshot, Range<BufferOffset>, Range<BufferOffset>) -> bool,
        >,
    ) -> Option<(Range<MultiBufferOffset>, Range<MultiBufferOffset>)> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let results =
            self.map_excerpt_ranges(range, |buffer, excerpt_range, input_buffer_range| {
                let filter = |open: Range<usize>, close: Range<usize>| -> bool {
                    excerpt_range.context.start.0 <= open.start
                        && close.end <= excerpt_range.context.end.0
                        && range_filter.is_none_or(|filter| {
                            filter(
                                buffer,
                                BufferOffset(open.start)..BufferOffset(close.end),
                                BufferOffset(close.start)..BufferOffset(close.end),
                            )
                        })
                };
                let Some((open, close)) =
                    buffer.innermost_enclosing_bracket_ranges(input_buffer_range, Some(&filter))
                else {
                    return Vec::new();
                };
                vec![
                    (BufferOffset(open.start)..BufferOffset(open.end), ()),
                    (BufferOffset(close.start)..BufferOffset(close.end), ()),
                ]
            })?;
        let [(open, _), (close, _)] = results.try_into().ok()?;
        Some((open, close))
    }

    /// Returns enclosing bracket ranges containing the given range or returns None if the range is
    /// not contained in a single excerpt
    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<impl Iterator<Item = (Range<MultiBufferOffset>, Range<MultiBufferOffset>)>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let results =
            self.map_excerpt_ranges(range, |buffer, excerpt_range, input_buffer_range| {
                buffer
                    .enclosing_bracket_ranges(input_buffer_range)
                    .filter(|pair| {
                        excerpt_range.context.start.0 <= pair.open_range.start
                            && pair.close_range.end <= excerpt_range.context.end.0
                    })
                    .flat_map(|pair| {
                        [
                            (
                                BufferOffset(pair.open_range.start)
                                    ..BufferOffset(pair.open_range.end),
                                (),
                            ),
                            (
                                BufferOffset(pair.close_range.start)
                                    ..BufferOffset(pair.close_range.end),
                                (),
                            ),
                        ]
                    })
                    .collect()
            })?;
        Some(results.into_iter().map(|(range, _)| range).tuples())
    }

    /// Returns enclosing bracket ranges containing the given range or returns None if the range is
    /// not contained in a single excerpt
    pub fn text_object_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
        options: TreeSitterOptions,
    ) -> impl Iterator<Item = (Range<MultiBufferOffset>, TextObject)> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        self.map_excerpt_ranges(range, |buffer, excerpt_range, input_buffer_range| {
            buffer
                .text_object_ranges(input_buffer_range, options)
                .filter(|(range, _)| {
                    excerpt_range.context.start.0 <= range.start
                        && range.end <= excerpt_range.context.end.0
                })
                .map(|(range, text_object)| {
                    (
                        BufferOffset(range.start)..BufferOffset(range.end),
                        text_object,
                    )
                })
                .collect()
        })
        .into_iter()
        .flatten()
    }

    pub fn bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<impl Iterator<Item = (Range<MultiBufferOffset>, Range<MultiBufferOffset>)>> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let results =
            self.map_excerpt_ranges(range, |buffer, excerpt_range, input_buffer_range| {
                buffer
                    .bracket_ranges(input_buffer_range)
                    .filter(|pair| {
                        excerpt_range.context.start.0 <= pair.open_range.start
                            && pair.close_range.end <= excerpt_range.context.end.0
                    })
                    .flat_map(|pair| {
                        [
                            (
                                BufferOffset(pair.open_range.start)
                                    ..BufferOffset(pair.open_range.end),
                                (),
                            ),
                            (
                                BufferOffset(pair.close_range.start)
                                    ..BufferOffset(pair.close_range.end),
                                (),
                            ),
                        ]
                    })
                    .collect()
            })?;
        Some(results.into_iter().map(|(range, _)| range).tuples())
    }

    pub fn redacted_ranges<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        redaction_enabled: impl Fn(Option<&Arc<dyn File>>) -> bool + 'a,
    ) -> impl Iterator<Item = Range<MultiBufferOffset>> + 'a {
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
    ) -> impl Iterator<Item = (Range<Anchor>, language::RunnableRange)> + '_ {
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
        .map(|(run_range, runnable, _)| {
            (
                self.anchor_after(run_range.start)..self.anchor_before(run_range.end),
                runnable,
            )
        })
    }

    pub fn line_indents(
        &self,
        start_row: MultiBufferRow,
        buffer_filter: impl Fn(&BufferSnapshot) -> bool,
    ) -> impl Iterator<Item = (MultiBufferRow, LineIndent, &BufferSnapshot)> {
        let max_point = self.max_point();
        let mut cursor = self.cursor::<Point, Point>();
        cursor.seek(&Point::new(start_row.0, 0));
        iter::from_fn(move || {
            let mut region = cursor.region()?;
            while !buffer_filter(&region.excerpt.buffer_snapshot(self)) {
                cursor.next();
                region = cursor.region()?;
            }
            let region = cursor.region()?;
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
            let region_buffer_row = region.buffer_range.start.row;
            let region_row = region.range.start.row;
            let region_buffer = region.excerpt.buffer_snapshot(self);
            cursor.next();
            Some(line_indents.map(move |(buffer_row, indent)| {
                let row = region_row + (buffer_row - region_buffer_row);
                (MultiBufferRow(row), indent, region_buffer)
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
        let mut cursor = self.cursor::<Point, Point>();
        cursor.seek(&Point::new(end_row.0, 0));
        iter::from_fn(move || {
            let mut region = cursor.region()?;
            while !buffer_filter(&region.excerpt.buffer_snapshot(self)) {
                cursor.prev();
                region = cursor.region()?;
            }
            let region = cursor.region()?;

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
            let region_buffer_row = region.buffer_range.start.row;
            let region_row = region.range.start.row;
            let region_buffer = region.excerpt.buffer_snapshot(self);
            cursor.prev();
            Some(line_indents.map(move |(buffer_row, indent)| {
                let row = region_row + (buffer_row - region_buffer_row);
                (MultiBufferRow(row), indent, region_buffer)
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
            let settings = LanguageSettings::for_buffer_snapshot(buffer, None, cx);
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
                        LanguageSettings::for_buffer_snapshot(buffer, None, cx),
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
                            settings: settings.indent_guides.clone(),
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

    pub fn language_at<T: ToOffset>(&self, offset: T) -> Option<&Arc<Language>> {
        self.point_to_buffer_offset(offset)
            .and_then(|(buffer, offset)| buffer.language_at(offset))
    }

    fn language_settings<'a>(&'a self, cx: &'a App) -> Cow<'a, LanguageSettings> {
        self.excerpts
            .first()
            .map(|excerpt| excerpt.buffer_snapshot(self))
            .map(|buffer| LanguageSettings::for_buffer_snapshot(buffer, None, cx))
            .unwrap_or_else(move || self.language_settings_at(MultiBufferOffset::ZERO, cx))
    }

    pub fn language_settings_at<'a, T: ToOffset>(
        &'a self,
        point: T,
        cx: &'a App,
    ) -> Cow<'a, LanguageSettings> {
        if let Some((buffer, offset)) = self.point_to_buffer_offset(point) {
            buffer.settings_at(offset, cx)
        } else {
            Cow::Borrowed(&AllLanguageSettings::get_global(cx).defaults)
        }
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
            .any(|excerpt| excerpt.buffer_snapshot(self).has_diagnostics())
    }

    pub fn diagnostic_group(
        &self,
        buffer_id: BufferId,
        group_id: usize,
    ) -> impl Iterator<Item = DiagnosticEntryRef<'_, Point>> + '_ {
        self.lift_buffer_metadata::<Point, _, _>(
            Point::zero()..self.max_point(),
            move |buffer, range| {
                if buffer.remote_id() != buffer_id {
                    return None;
                };
                Some(
                    buffer
                        .diagnostics_in_range(range, false)
                        .filter(move |diagnostic| diagnostic.diagnostic.group_id == group_id)
                        .map(move |DiagnosticEntryRef { diagnostic, range }| (range, diagnostic)),
                )
            },
        )
        .map(|(range, diagnostic, _)| DiagnosticEntryRef { diagnostic, range })
    }

    pub fn diagnostics_in_range<'a, MBD>(
        &'a self,
        range: Range<MBD>,
    ) -> impl Iterator<Item = DiagnosticEntryRef<'a, MBD>> + 'a
    where
        MBD::TextDimension: 'a
            + text::ToOffset
            + text::FromAnchor
            + Sub<Output = MBD::TextDimension>
            + fmt::Debug
            + ops::Add<Output = MBD::TextDimension>
            + ops::AddAssign
            + Ord,
        MBD: MultiBufferDimension
            + Ord
            + Sub<Output = MBD::TextDimension>
            + ops::Add<MBD::TextDimension, Output = MBD>
            + ops::AddAssign<MBD::TextDimension>
            + 'a,
    {
        self.lift_buffer_metadata::<MBD, _, _>(range, move |buffer, buffer_range| {
            Some(
                buffer
                    .diagnostics_in_range(buffer_range.start..buffer_range.end, false)
                    .map(|entry| (entry.range, entry.diagnostic)),
            )
        })
        .map(|(range, diagnostic, _)| DiagnosticEntryRef { diagnostic, range })
    }

    pub fn diagnostics_with_buffer_ids_in_range<'a, MBD>(
        &'a self,
        range: Range<MBD>,
    ) -> impl Iterator<Item = (BufferId, DiagnosticEntryRef<'a, MBD>)> + 'a
    where
        MBD: MultiBufferDimension
            + Ord
            + Sub<Output = MBD::TextDimension>
            + ops::Add<MBD::TextDimension, Output = MBD>
            + ops::AddAssign<MBD::TextDimension>,
        MBD::TextDimension: Sub<Output = MBD::TextDimension>
            + ops::Add<Output = MBD::TextDimension>
            + text::ToOffset
            + text::FromAnchor
            + AddAssign<MBD::TextDimension>
            + Ord,
    {
        self.lift_buffer_metadata::<MBD, _, _>(range, move |buffer, buffer_range| {
            Some(
                buffer
                    .diagnostics_in_range(buffer_range.start..buffer_range.end, false)
                    .map(|entry| (entry.range, entry.diagnostic)),
            )
        })
        .map(|(range, diagnostic, excerpt)| {
            (
                excerpt.buffer_snapshot(self).remote_id(),
                DiagnosticEntryRef { diagnostic, range },
            )
        })
    }

    pub fn syntax_ancestor<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(tree_sitter::Node<'_>, Range<MultiBufferOffset>)> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let results =
            self.map_excerpt_ranges(range, |buffer, excerpt_range, input_buffer_range| {
                let Some(node) = buffer.syntax_ancestor(input_buffer_range) else {
                    return vec![];
                };
                let node_range = node.byte_range();
                if excerpt_range.context.start.0 <= node_range.start
                    && node_range.end <= excerpt_range.context.end.0
                {
                    vec![(
                        BufferOffset(node_range.start)..BufferOffset(node_range.end),
                        node,
                    )]
                } else {
                    vec![]
                }
            })?;
        let (output_range, node) = results.into_iter().next()?;
        Some((node, output_range))
    }

    pub fn outline(&self, theme: Option<&SyntaxTheme>) -> Option<Outline<Anchor>> {
        let buffer_snapshot = self.as_singleton()?;
        let excerpt = self.excerpts.first()?;
        let path_key_index = excerpt.path_key_index;
        let outline = buffer_snapshot.outline(theme);
        Some(Outline::new(
            outline
                .items
                .into_iter()
                .map(|item| OutlineItem {
                    depth: item.depth,
                    range: Anchor::range_in_buffer(path_key_index, item.range),
                    source_range_for_text: Anchor::range_in_buffer(
                        path_key_index,
                        item.source_range_for_text,
                    ),
                    text: item.text,
                    highlight_ranges: item.highlight_ranges,
                    name_ranges: item.name_ranges,
                    body_range: item
                        .body_range
                        .map(|body_range| Anchor::range_in_buffer(path_key_index, body_range)),
                    annotation_range: item.annotation_range.map(|annotation_range| {
                        Anchor::range_in_buffer(path_key_index, annotation_range)
                    }),
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
        let target = anchor.try_seek_target(&self)?;
        let (_, _, excerpt) = self.excerpts.find((), &target, Bias::Left);
        let excerpt = excerpt?;
        let buffer_snapshot = excerpt.buffer_snapshot(self);
        Some((
            buffer_snapshot.remote_id(),
            buffer_snapshot
                .symbols_containing(
                    anchor
                        .excerpt_anchor()
                        .map(|anchor| anchor.text_anchor())
                        .unwrap_or(text::Anchor::min_for_buffer(buffer_snapshot.remote_id())),
                    theme,
                )
                .into_iter()
                .flat_map(|item| {
                    Some(OutlineItem {
                        depth: item.depth,
                        source_range_for_text: Anchor::range_in_buffer(
                            excerpt.path_key_index,
                            item.source_range_for_text,
                        ),
                        range: Anchor::range_in_buffer(excerpt.path_key_index, item.range),
                        text: item.text,
                        highlight_ranges: item.highlight_ranges,
                        name_ranges: item.name_ranges,
                        body_range: item.body_range.map(|body_range| {
                            Anchor::range_in_buffer(excerpt.path_key_index, body_range)
                        }),
                        annotation_range: item.annotation_range.map(|body_range| {
                            Anchor::range_in_buffer(excerpt.path_key_index, body_range)
                        }),
                    })
                })
                .collect(),
        ))
    }

    pub fn buffer_for_path(&self, path: &PathKey) -> Option<&BufferSnapshot> {
        let (_, _, excerpt) = self
            .excerpts
            .find::<ExcerptSummary, _>((), path, Bias::Left);
        Some(excerpt?.buffer_snapshot(self))
    }

    pub fn path_for_buffer(&self, buffer_id: BufferId) -> Option<&PathKey> {
        Some(&self.buffers.get(&buffer_id)?.path_key)
    }

    pub(crate) fn path_key_index_for_buffer(&self, buffer_id: BufferId) -> Option<PathKeyIndex> {
        let snapshot = self.buffers.get(&buffer_id)?;
        Some(snapshot.path_key_index)
    }

    fn first_excerpt_for_buffer(&self, buffer_id: BufferId) -> Option<&Excerpt> {
        let path_key = &self.buffers.get(&buffer_id)?.path_key;
        self.first_excerpt_for_path(path_key)
    }

    fn first_excerpt_for_path(&self, path_key: &PathKey) -> Option<&Excerpt> {
        let (_, _, first_excerpt) =
            self.excerpts
                .find::<ExcerptSummary, _>((), path_key, Bias::Left);
        first_excerpt
    }

    pub fn buffer_for_id(&self, id: BufferId) -> Option<&BufferSnapshot> {
        self.buffers.get(&id).map(|state| &state.buffer_snapshot)
    }

    fn try_path_for_anchor(&self, anchor: ExcerptAnchor) -> Option<PathKey> {
        self.path_keys_by_index.get(&anchor.path).cloned()
    }

    pub fn path_for_anchor(&self, anchor: ExcerptAnchor) -> PathKey {
        self.try_path_for_anchor(anchor)
            .expect("invalid anchor: path was never added to multibuffer")
    }

    /// Returns the excerpt containing range and its offset start within the multibuffer or none if `range` spans multiple excerpts
    pub fn excerpt_containing<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(&BufferSnapshot, ExcerptRange<text::Anchor>)> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.cursor::<MultiBufferOffset, BufferOffset>();
        cursor.seek(&range.start);

        let start_excerpt = cursor.excerpt()?;
        if range.end != range.start {
            cursor.seek_forward(&range.end);
            if cursor.excerpt()? != start_excerpt {
                return None;
            }
        }

        Some((
            start_excerpt.buffer_snapshot(self),
            start_excerpt.range.clone(),
        ))
    }

    pub fn selections_in_range<'a>(
        &'a self,
        range: &'a Range<Anchor>,
        include_local: bool,
    ) -> impl 'a + Iterator<Item = (ReplicaId, bool, CursorShape, Selection<Anchor>)> {
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(());
        cursor.seek(&range.start.seek_target(self), Bias::Left);
        cursor
            .take_while(move |excerpt| {
                let excerpt_start =
                    Anchor::in_buffer(excerpt.path_key_index, excerpt.range.context.start);
                excerpt_start.cmp(&range.end, self).is_le()
            })
            .flat_map(move |excerpt| {
                let buffer_snapshot = excerpt.buffer_snapshot(self);
                let mut query_range = excerpt.range.context.start..excerpt.range.context.end;
                if let Some(excerpt_anchor) = range.start.excerpt_anchor()
                    && excerpt.contains(&excerpt_anchor, self)
                {
                    query_range.start = excerpt_anchor.text_anchor();
                }
                if let Some(excerpt_anchor) = range.end.excerpt_anchor()
                    && excerpt.contains(&excerpt_anchor, self)
                {
                    query_range.end = excerpt_anchor.text_anchor();
                }

                buffer_snapshot
                    .selections_in_range(query_range, include_local)
                    .flat_map(move |(replica_id, line_mode, cursor_shape, selections)| {
                        selections.map(move |selection| {
                            let mut start =
                                Anchor::in_buffer(excerpt.path_key_index, selection.start);
                            let mut end = Anchor::in_buffer(excerpt.path_key_index, selection.end);
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
        self.diff_state(buffer_id).map(|diff| &diff.diff)
    }

    fn diff_state(&self, buffer_id: BufferId) -> Option<&DiffStateSnapshot> {
        find_diff_state(&self.diffs, buffer_id)
    }

    pub fn total_changed_lines(&self) -> (u32, u32) {
        let summary = self.diffs.summary();
        (summary.added_rows, summary.removed_rows)
    }

    pub fn all_diff_hunks_expanded(&self) -> bool {
        self.all_diff_hunks_expanded
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
                self.range_to_buffer_ranges(range)
                    .into_iter()
                    .map(|(buffer_snapshot, range, _)| {
                        buffer_snapshot.anchor_after(range.start)
                            ..buffer_snapshot.anchor_before(range.end)
                    })
            })
            .collect();
        text::debug::GlobalDebugRanges::with_locked(|debug_ranges| {
            debug_ranges.insert(key, text_ranges, format!("{value:?}").into())
        });
    }

    fn excerpt_edits_for_diff_change(
        &self,
        path: &PathKey,
        diff_change_range: Range<usize>,
    ) -> Vec<Edit<ExcerptDimension<MultiBufferOffset>>> {
        let mut excerpt_edits = Vec::new();
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(());
        cursor.seek(path, Bias::Left);
        while let Some(excerpt) = cursor.item()
            && &excerpt.path_key == path
        {
            let buffer_snapshot = excerpt.buffer_snapshot(self);
            let excerpt_buffer_range = excerpt.range.context.to_offset(buffer_snapshot);
            let excerpt_start = cursor.start().clone();
            let excerpt_len = excerpt.text_summary.len;
            cursor.next();
            if diff_change_range.end < excerpt_buffer_range.start
                || diff_change_range.start > excerpt_buffer_range.end
            {
                continue;
            }
            let diff_change_start_in_excerpt = diff_change_range
                .start
                .saturating_sub(excerpt_buffer_range.start);
            let diff_change_end_in_excerpt = diff_change_range
                .end
                .saturating_sub(excerpt_buffer_range.start);
            let edit_start = excerpt_start.len() + diff_change_start_in_excerpt.min(excerpt_len);
            let edit_end = excerpt_start.len() + diff_change_end_in_excerpt.min(excerpt_len);
            excerpt_edits.push(Edit {
                old: edit_start..edit_end,
                new: edit_start..edit_end,
            });
        }
        excerpt_edits
    }

    fn excerpts_for_path<'a>(
        &'a self,
        path_key: &'a PathKey,
    ) -> impl Iterator<Item = ExcerptRange<text::Anchor>> + 'a {
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(());
        cursor.seek(path_key, Bias::Left);
        cursor
            .take_while(move |item| &item.path_key == path_key)
            .map(|excerpt| excerpt.range.clone())
    }

    /// If the given multibuffer range is contained in a single excerpt and contains no deleted hunks,
    /// returns the corresponding buffer range.
    ///
    /// Otherwise, returns None.
    pub fn range_to_buffer_range<MBD>(
        &self,
        range: Range<MBD>,
    ) -> Option<(&BufferSnapshot, Range<MBD::TextDimension>)>
    where
        MBD: MultiBufferDimension + Ord + Sub + ops::AddAssign<<MBD as Sub>::Output>,
        MBD::TextDimension: AddAssign<<MBD as Sub>::Output>,
    {
        let mut cursor = self.cursor::<MBD, MBD::TextDimension>();
        cursor.seek(&range.start);

        let start_region = cursor.region()?.clone();

        while let Some(region) = cursor.region()
            && region.range.end < range.end
        {
            if !region.is_main_buffer {
                return None;
            }
            cursor.next();
        }

        let end_region = cursor.region()?;
        if end_region.buffer.remote_id() != start_region.buffer.remote_id() {
            return None;
        }

        let mut buffer_start = start_region.buffer_range.start;
        buffer_start += range.start - start_region.range.start;
        let mut buffer_end = end_region.buffer_range.start;
        buffer_end += range.end - end_region.range.start;

        Some((start_region.buffer, buffer_start..buffer_end))
    }

    /// If the two endpoints of the range lie in the same excerpt, return the corresponding
    /// buffer range. Intervening deleted hunks are allowed.
    pub fn anchor_range_to_buffer_anchor_range(
        &self,
        range: Range<Anchor>,
    ) -> Option<(&BufferSnapshot, Range<text::Anchor>)> {
        let mut cursor = self.excerpts.cursor::<ExcerptSummary>(());
        cursor.seek(&range.start.seek_target(&self), Bias::Left);

        let start_excerpt = cursor.item()?;

        let snapshot = start_excerpt.buffer_snapshot(&self);

        cursor.seek(&range.end.seek_target(&self), Bias::Left);

        let end_excerpt = cursor.item()?;

        if start_excerpt != end_excerpt {
            return None;
        }

        if let Anchor::Excerpt(excerpt_anchor) = range.start
            && (excerpt_anchor.path != start_excerpt.path_key_index
                || excerpt_anchor.buffer_id() != snapshot.remote_id())
        {
            return None;
        }
        if let Anchor::Excerpt(excerpt_anchor) = range.end
            && (excerpt_anchor.path != end_excerpt.path_key_index
                || excerpt_anchor.buffer_id() != snapshot.remote_id())
        {
            return None;
        }

        Some((
            snapshot,
            range.start.text_anchor_in(snapshot)..range.end.text_anchor_in(snapshot),
        ))
    }

    /// Returns all nonempty intersections of the given buffer range with excerpts in the multibuffer in order.
    ///
    /// The multibuffer ranges are split to not intersect deleted hunks.
    pub fn buffer_range_to_excerpt_ranges(
        &self,
        range: Range<text::Anchor>,
    ) -> impl Iterator<Item = Range<Anchor>> {
        assert!(range.start.buffer_id == range.end.buffer_id);

        let buffer_id = range.start.buffer_id;
        self.buffers
            .get(&buffer_id)
            .map(|buffer_state_snapshot| {
                let path_key_index = buffer_state_snapshot.path_key_index;
                let buffer_snapshot = &buffer_state_snapshot.buffer_snapshot;
                let buffer_range = range.to_offset(buffer_snapshot);

                let start = Anchor::in_buffer(path_key_index, range.start).to_offset(self);
                let mut cursor = self.cursor::<MultiBufferOffset, BufferOffset>();
                cursor.seek(&start);
                std::iter::from_fn(move || {
                    while let Some(region) = cursor.region()
                        && !region.is_main_buffer
                    {
                        cursor.next();
                    }

                    let region = cursor.region()?;
                    if region.buffer.remote_id() != buffer_id
                        || region.buffer_range.start > BufferOffset(buffer_range.end)
                    {
                        return None;
                    }

                    let start = region
                        .buffer_range
                        .start
                        .max(BufferOffset(buffer_range.start));
                    let mut end = region.buffer_range.end.min(BufferOffset(buffer_range.end));

                    cursor.next();
                    while let Some(region) = cursor.region()
                        && region.is_main_buffer
                        && region.buffer.remote_id() == buffer_id
                        && region.buffer_range.start <= end
                    {
                        end = end
                            .max(region.buffer_range.end)
                            .min(BufferOffset(buffer_range.end));
                        cursor.next();
                    }

                    let multibuffer_range = Anchor::range_in_buffer(
                        path_key_index,
                        buffer_snapshot.anchor_range_inside(start..end),
                    );
                    Some(multibuffer_range)
                })
            })
            .into_iter()
            .flatten()
    }

    pub fn buffers_with_paths<'a>(
        &'a self,
    ) -> impl 'a + Iterator<Item = (&'a BufferSnapshot, &'a PathKey)> {
        self.buffers
            .values()
            .map(|buffer| (&buffer.buffer_snapshot, &buffer.path_key))
    }

    /// Returns the number of graphemes in `range`.
    ///
    /// This counts user-visible characters like `e\u{301}` as one.
    pub fn grapheme_count_for_range(&self, range: &Range<MultiBufferOffset>) -> usize {
        self.text_for_range(range.clone())
            .collect::<String>()
            .graphemes(true)
            .count()
    }

    pub fn range_for_buffer(&self, buffer_id: BufferId) -> Option<Range<Point>> {
        let path_key = self.path_key_index_for_buffer(buffer_id)?;
        let start = Anchor::in_buffer(path_key, text::Anchor::min_for_buffer(buffer_id));
        let end = Anchor::in_buffer(path_key, text::Anchor::max_for_buffer(buffer_id));
        Some((start..end).to_point(self))
    }
}

#[cfg(any(test, feature = "test-support"))]
impl MultiBufferSnapshot {
    pub fn random_byte_range(
        &self,
        start_offset: MultiBufferOffset,
        rng: &mut impl rand::Rng,
    ) -> Range<MultiBufferOffset> {
        let end = self.clip_offset(rng.random_range(start_offset..=self.len()), Bias::Right);
        let start = self.clip_offset(rng.random_range(start_offset..=end), Bias::Right);
        start..end
    }

    #[cfg(any(test, feature = "test-support"))]
    fn check_invariants(&self) {
        let excerpts = self.excerpts.items(());

        let mut all_buffer_path_keys = HashSet::default();
        for buffer in self.buffers.values() {
            let path_key = buffer.path_key.clone();
            assert!(
                all_buffer_path_keys.insert(path_key),
                "path key reused for multiple buffers: {:#?}",
                self.buffers
            );
        }

        let all_excerpt_path_keys = HashSet::from_iter(excerpts.iter().map(|e| e.path_key.clone()));

        for (ix, excerpt) in excerpts.iter().enumerate() {
            if ix > 0 {
                let prev = &excerpts[ix - 1];

                if excerpt.path_key < prev.path_key {
                    panic!("excerpt path_keys are out-of-order: {:#?}", excerpts);
                } else if excerpt.path_key == prev.path_key {
                    assert_eq!(
                        excerpt.buffer_id, prev.buffer_id,
                        "excerpts with same path_key have different buffer_ids: {:#?}",
                        excerpts
                    );
                    if excerpt
                        .start_anchor()
                        .cmp(&prev.end_anchor(), &self)
                        .is_le()
                    {
                        panic!("excerpt anchors are out-of-order: {:#?}", excerpts);
                    }
                    if excerpt
                        .start_anchor()
                        .cmp(&excerpt.end_anchor(), &self)
                        .is_ge()
                    {
                        panic!("excerpt with backward range: {:#?}", excerpts);
                    }
                }
            }

            if ix < excerpts.len() - 1 {
                assert!(
                    excerpt.has_trailing_newline,
                    "non-trailing excerpt has no trailing newline: {:#?}",
                    excerpts
                );
            } else {
                assert!(
                    !excerpt.has_trailing_newline,
                    "trailing excerpt has trailing newline: {:#?}",
                    excerpts
                );
            }
            assert!(
                all_buffer_path_keys.contains(&excerpt.path_key),
                "excerpt path key not found in active path keys: {:#?}",
                excerpt.path_key
            );
            assert_eq!(
                self.path_keys_by_index.get(&excerpt.path_key_index),
                Some(&excerpt.path_key),
                "excerpt path key index does not match path key: {:#?}",
                excerpt.path_key,
            );
        }
        assert_eq!(all_buffer_path_keys, all_excerpt_path_keys);

        if self.diff_transforms.summary().input != self.excerpts.summary().text {
            panic!(
                "incorrect input summary. expected {:?}, got {:?}. transforms: {:+?}",
                self.excerpts.summary().text,
                self.diff_transforms.summary().input,
                self.diff_transforms.items(()),
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
                        self.diff_transforms.items(())
                    );
                }
                if summary.len == MultiBufferOffset(0) && !self.is_empty() {
                    panic!("empty buffer content transform");
                }
            }
            prev_transform = Some(item);
        }
    }
}

impl<'a, MBD, BD> MultiBufferCursor<'a, MBD, BD>
where
    MBD: MultiBufferDimension + Ord + Sub + ops::AddAssign<<MBD as Sub>::Output>,
    BD: TextDimension + AddAssign<<MBD as Sub>::Output>,
{
    #[instrument(skip_all)]
    fn seek(&mut self, position: &MBD) {
        let position = OutputDimension(*position);
        self.cached_region.take();
        self.diff_transforms.seek(&position, Bias::Right);
        if self.diff_transforms.item().is_none()
            && self.diff_transforms.start().output_dimension == position
        {
            self.diff_transforms.prev();
        }

        let mut excerpt_position = self.diff_transforms.start().excerpt_dimension;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            let overshoot = position - self.diff_transforms.start().output_dimension;
            excerpt_position += overshoot;
        }

        self.excerpts.seek(&excerpt_position, Bias::Right);
        if self.excerpts.item().is_none() && excerpt_position == *self.excerpts.start() {
            self.excerpts.prev();
        }
    }

    fn seek_forward(&mut self, position: &MBD) {
        let position = OutputDimension(*position);
        self.cached_region.take();
        self.diff_transforms.seek_forward(&position, Bias::Right);
        if self.diff_transforms.item().is_none()
            && self.diff_transforms.start().output_dimension == position
        {
            self.diff_transforms.prev();
        }

        let overshoot = position - self.diff_transforms.start().output_dimension;
        let mut excerpt_position = self.diff_transforms.start().excerpt_dimension;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            excerpt_position += overshoot;
        }

        self.excerpts.seek_forward(&excerpt_position, Bias::Right);
        if self.excerpts.item().is_none() && excerpt_position == *self.excerpts.start() {
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

        if self.diff_transforms.seek(self.excerpts.start(), Bias::Left)
            && self.diff_transforms.start().excerpt_dimension < *self.excerpts.start()
            && self.diff_transforms.next_item().is_some()
        {
            self.diff_transforms.next();
        }
    }

    fn next_excerpt_forwards(&mut self) {
        self.excerpts.next();
        self.seek_to_start_of_current_excerpt_forward();
    }

    fn seek_to_start_of_current_excerpt_forward(&mut self) {
        self.cached_region.take();

        if self
            .diff_transforms
            .seek_forward(self.excerpts.start(), Bias::Left)
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
                        .is_some_and(|excerpt| excerpt.end_anchor() != hunk_info.excerpt_end)
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

    fn region(&self) -> Option<&MultiBufferRegion<'a, MBD, BD>> {
        self.cached_region
            .get_or_init(|| self.build_region())
            .as_ref()
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

    fn is_at_end_of_excerpt(&self) -> bool {
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
                .is_some_and(|excerpt| excerpt.end_anchor() != hunk_info.excerpt_end),
        })
    }

    fn main_buffer_position(&self) -> Option<BD> {
        let excerpt = self.excerpts.item()?;
        let buffer = excerpt.buffer_snapshot(self.snapshot);
        let buffer_context_start = excerpt.range.context.start.summary::<BD>(buffer);
        let mut buffer_start = buffer_context_start;
        let overshoot = self.diff_transforms.end().excerpt_dimension - *self.excerpts.start();
        buffer_start += overshoot;
        Some(buffer_start)
    }

    fn buffer_position_at(&self, output_position: &MBD) -> Option<BD> {
        let excerpt = self.excerpts.item()?;
        let buffer = excerpt.buffer_snapshot(self.snapshot);
        let buffer_context_start = excerpt.range.context.start.summary::<BD>(buffer);
        let mut excerpt_offset = self.diff_transforms.start().excerpt_dimension;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            excerpt_offset += *output_position - self.diff_transforms.start().output_dimension.0;
        }
        let mut result = buffer_context_start;
        result += excerpt_offset - *self.excerpts.start();
        Some(result)
    }

    fn build_region(&self) -> Option<MultiBufferRegion<'a, MBD, BD>> {
        let excerpt = self.excerpts.item()?;
        match self.diff_transforms.item()? {
            DiffTransform::DeletedHunk {
                buffer_id,
                base_text_byte_range,
                has_trailing_newline,
                hunk_info,
                ..
            } => {
                let diff = find_diff_state(&self.snapshot.diffs, *buffer_id)?;
                let buffer = diff.base_text();
                let mut rope_cursor = buffer.as_rope().cursor(0);
                let buffer_start = rope_cursor.summary::<BD>(base_text_byte_range.start);
                let buffer_range_len = rope_cursor.summary::<BD>(base_text_byte_range.end);
                let mut buffer_end = buffer_start;
                TextDimension::add_assign(&mut buffer_end, &buffer_range_len);
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
                let buffer = excerpt.buffer_snapshot(self.snapshot);
                let buffer_context_start = excerpt.range.context.start.summary::<BD>(buffer);

                let mut start = self.diff_transforms.start().output_dimension.0;
                let mut buffer_start = buffer_context_start;
                if self.diff_transforms.start().excerpt_dimension < *self.excerpts.start() {
                    let overshoot =
                        *self.excerpts.start() - self.diff_transforms.start().excerpt_dimension;
                    start += overshoot;
                } else {
                    let overshoot =
                        self.diff_transforms.start().excerpt_dimension - *self.excerpts.start();
                    buffer_start += overshoot;
                }

                let mut end;
                let mut buffer_end;
                let has_trailing_newline;
                let transform_end = self.diff_transforms.end();
                if transform_end.excerpt_dimension < self.excerpts.end() {
                    let overshoot = transform_end.excerpt_dimension - *self.excerpts.start();
                    end = transform_end.output_dimension.0;
                    buffer_end = buffer_context_start;
                    buffer_end += overshoot;
                    has_trailing_newline = false;
                } else {
                    let overshoot =
                        self.excerpts.end() - self.diff_transforms.start().excerpt_dimension;
                    end = self.diff_transforms.start().output_dimension.0;
                    end += overshoot;
                    buffer_end = excerpt.range.context.end.summary::<BD>(buffer);
                    has_trailing_newline = excerpt.has_trailing_newline;
                };

                let diff_hunk_status = inserted_hunk_info.map(|info| {
                    if info.is_logically_deleted {
                        DiffHunkStatus::deleted(info.hunk_secondary_status)
                    } else {
                        DiffHunkStatus::added(info.hunk_secondary_status)
                    }
                });

                Some(MultiBufferRegion {
                    buffer,
                    excerpt,
                    has_trailing_newline,
                    is_main_buffer: true,
                    diff_hunk_status,
                    buffer_range: buffer_start..buffer_end,
                    range: start..end,
                })
            }
        }
    }

    fn fetch_excerpt_with_range(&self) -> Option<(&'a Excerpt, Range<MBD>)> {
        let excerpt = self.excerpts.item()?;
        match self.diff_transforms.item()? {
            &DiffTransform::DeletedHunk { .. } => {
                let start = self.diff_transforms.start().output_dimension.0;
                let end = self.diff_transforms.end().output_dimension.0;
                Some((excerpt, start..end))
            }
            DiffTransform::BufferContent { .. } => {
                let mut start = self.diff_transforms.start().output_dimension.0;
                if self.diff_transforms.start().excerpt_dimension < *self.excerpts.start() {
                    let overshoot =
                        *self.excerpts.start() - self.diff_transforms.start().excerpt_dimension;
                    start += overshoot;
                }

                let mut end;
                let transform_end = self.diff_transforms.end();
                if transform_end.excerpt_dimension < self.excerpts.end() {
                    end = transform_end.output_dimension.0;
                } else {
                    let overshoot =
                        self.excerpts.end() - self.diff_transforms.start().excerpt_dimension;
                    end = self.diff_transforms.start().output_dimension.0;
                    end += overshoot;
                };

                Some((excerpt, start..end))
            }
        }
    }

    fn excerpt(&self) -> Option<&'a Excerpt> {
        self.excerpts.item()
    }
}

impl Excerpt {
    fn new(
        path_key: PathKey,
        path_key_index: PathKeyIndex,
        buffer_snapshot: &BufferSnapshot,
        range: ExcerptRange<text::Anchor>,
        has_trailing_newline: bool,
    ) -> Self {
        Excerpt {
            path_key,
            path_key_index,
            buffer_id: buffer_snapshot.remote_id(),
            max_buffer_row: range.context.end.to_point(&buffer_snapshot).row,
            text_summary: buffer_snapshot.text_summary_for_range::<TextSummary, _>(
                range.context.to_offset(&buffer_snapshot),
            ),
            range,
            has_trailing_newline,
        }
    }

    fn buffer_snapshot<'a>(&self, snapshot: &'a MultiBufferSnapshot) -> &'a BufferSnapshot {
        &snapshot
            .buffers
            .get(&self.buffer_id)
            .expect("buffer snapshot not found for excerpt")
            .buffer_snapshot
    }

    fn buffer(&self, multibuffer: &MultiBuffer) -> Entity<Buffer> {
        multibuffer
            .buffer(self.buffer_id)
            .expect("buffer entity not found for excerpt")
    }

    fn chunks_in_range<'a>(
        &'a self,
        range: Range<usize>,
        language_aware: LanguageAwareStyling,
        snapshot: &'a MultiBufferSnapshot,
    ) -> ExcerptChunks<'a> {
        let buffer = self.buffer_snapshot(snapshot);
        let content_start = self.range.context.start.to_offset(buffer);
        let chunks_start = content_start + range.start;
        let chunks_end = content_start + cmp::min(range.end, self.text_summary.len);

        let has_footer = self.has_trailing_newline
            && range.start <= self.text_summary.len
            && range.end > self.text_summary.len;

        let content_chunks = buffer.chunks(chunks_start..chunks_end, language_aware);

        ExcerptChunks {
            content_chunks,
            has_footer,
            end: self.end_anchor(),
        }
    }

    fn seek_chunks(
        &self,
        excerpt_chunks: &mut ExcerptChunks,
        range: Range<usize>,
        snapshot: &MultiBufferSnapshot,
    ) {
        let buffer = self.buffer_snapshot(snapshot);
        let content_start = self.range.context.start.to_offset(buffer);
        let chunks_start = content_start + range.start;
        let chunks_end = content_start + cmp::min(range.end, self.text_summary.len);
        excerpt_chunks.content_chunks.seek(chunks_start..chunks_end);
        excerpt_chunks.has_footer = self.has_trailing_newline
            && range.start <= self.text_summary.len
            && range.end > self.text_summary.len;
    }

    fn clip_anchor(
        &self,
        text_anchor: text::Anchor,
        snapshot: &MultiBufferSnapshot,
    ) -> text::Anchor {
        let buffer = self.buffer_snapshot(snapshot);
        if text_anchor.cmp(&self.range.context.start, buffer).is_lt() {
            self.range.context.start
        } else if text_anchor.cmp(&self.range.context.end, buffer).is_gt() {
            self.range.context.end
        } else {
            text_anchor
        }
    }

    pub(crate) fn contains(&self, anchor: &ExcerptAnchor, snapshot: &MultiBufferSnapshot) -> bool {
        self.path_key_index == anchor.path
            && self.buffer_id == anchor.text_anchor.buffer_id
            && self
                .range
                .contains(&anchor.text_anchor(), self.buffer_snapshot(snapshot))
    }

    fn start_anchor(&self) -> ExcerptAnchor {
        ExcerptAnchor::in_buffer(self.path_key_index, self.range.context.start)
    }

    fn end_anchor(&self) -> ExcerptAnchor {
        ExcerptAnchor::in_buffer(self.path_key_index, self.range.context.end)
    }
}

impl PartialEq for Excerpt {
    fn eq(&self, other: &Self) -> bool {
        self.path_key_index == other.path_key_index
            && self.buffer_id == other.buffer_id
            && self.range.context == other.range.context
    }
}

impl sum_tree::Item for Excerpt {
    type Summary = ExcerptSummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        let mut text = self.text_summary;
        if self.has_trailing_newline {
            text += TextSummary::from("\n");
        }
        ExcerptSummary {
            path_key: self.path_key.clone(),
            max_anchor: Some(self.range.context.end),
            widest_line_number: self.max_buffer_row,
            text: text.into(),
            count: 1,
        }
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

    fn summary(&self, _: <Self::Summary as sum_tree::Summary>::Context<'_>) -> Self::Summary {
        match self {
            DiffTransform::BufferContent { summary, .. } => DiffTransformSummary {
                input: *summary,
                output: *summary,
            },
            &DiffTransform::DeletedHunk { summary, .. } => DiffTransformSummary {
                input: MBTextSummary::default(),
                output: summary.into(),
            },
        }
    }
}

impl DiffTransformSummary {
    fn excerpt_len(&self) -> ExcerptOffset {
        ExcerptDimension(self.input.len)
    }
}

impl sum_tree::ContextLessSummary for DiffTransformSummary {
    fn zero() -> Self {
        DiffTransformSummary {
            input: MBTextSummary::default(),
            output: MBTextSummary::default(),
        }
    }

    fn add_summary(&mut self, other: &Self) {
        self.input += other.input;
        self.output += other.output;
    }
}

impl sum_tree::Dimension<'_, ExcerptSummary> for PathKey {
    fn zero(_: <ExcerptSummary as sum_tree::Summary>::Context<'_>) -> Self {
        PathKey::min()
    }

    fn add_summary(
        &mut self,
        summary: &'_ ExcerptSummary,
        _cx: <ExcerptSummary as sum_tree::Summary>::Context<'_>,
    ) {
        *self = summary.path_key.clone();
    }
}

impl sum_tree::Dimension<'_, ExcerptSummary> for MultiBufferOffset {
    fn zero(_: <ExcerptSummary as sum_tree::Summary>::Context<'_>) -> Self {
        MultiBufferOffset::ZERO
    }

    fn add_summary(
        &mut self,
        summary: &'_ ExcerptSummary,
        _cx: <ExcerptSummary as sum_tree::Summary>::Context<'_>,
    ) {
        *self += summary.text.len
    }
}

impl sum_tree::ContextLessSummary for ExcerptSummary {
    fn zero() -> Self {
        Self::min()
    }

    fn add_summary(&mut self, summary: &Self) {
        debug_assert!(
            summary.path_key >= self.path_key,
            "Path keys must be in ascending order: {:?} > {:?}",
            summary.path_key,
            self.path_key
        );

        self.path_key = summary.path_key.clone();
        self.max_anchor = summary.max_anchor;
        self.text += summary.text;
        self.widest_line_number = cmp::max(self.widest_line_number, summary.widest_line_number);
        self.count += summary.count;
    }
}

impl sum_tree::SeekTarget<'_, ExcerptSummary, ExcerptSummary> for AnchorSeekTarget {
    fn cmp(
        &self,
        cursor_location: &ExcerptSummary,
        _cx: <ExcerptSummary as sum_tree::Summary>::Context<'_>,
    ) -> cmp::Ordering {
        match self {
            AnchorSeekTarget::Excerpt {
                path_key,
                anchor,
                snapshot,
            } => {
                let path_comparison = Ord::cmp(path_key, &cursor_location.path_key);
                if path_comparison.is_ne() {
                    path_comparison
                } else if let Some(snapshot) = snapshot {
                    if anchor.text_anchor.buffer_id != snapshot.remote_id() {
                        Ordering::Greater
                    } else if let Some(max_anchor) = cursor_location.max_anchor {
                        debug_assert_eq!(max_anchor.buffer_id, snapshot.remote_id());
                        anchor.text_anchor().cmp(&max_anchor, snapshot)
                    } else {
                        Ordering::Greater
                    }
                } else {
                    // shouldn't happen because we expect this buffer not to have any excerpts
                    // (otherwise snapshot would have been Some)
                    Ordering::Equal
                }
            }
            // This should be dead code because Empty is only constructed for an empty snapshot
            AnchorSeekTarget::Empty => Ordering::Equal,
        }
    }
}

impl sum_tree::SeekTarget<'_, ExcerptSummary, ExcerptSummary> for PathKey {
    fn cmp(
        &self,
        cursor_location: &ExcerptSummary,
        _cx: <ExcerptSummary as sum_tree::Summary>::Context<'_>,
    ) -> cmp::Ordering {
        Ord::cmp(self, &cursor_location.path_key)
    }
}

impl<'a, MBD> sum_tree::Dimension<'a, ExcerptSummary> for ExcerptDimension<MBD>
where
    MBD: MultiBufferDimension + Default,
{
    fn zero(_: ()) -> Self {
        ExcerptDimension(MBD::default())
    }

    fn add_summary(&mut self, summary: &'a ExcerptSummary, _: ()) {
        MultiBufferDimension::add_mb_text_summary(&mut self.0, &summary.text)
    }
}

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Debug)]
struct OutputDimension<T>(T);

impl<T: PartialEq> PartialEq<T> for OutputDimension<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for OutputDimension<T> {
    fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<R, T, U> ops::Sub<OutputDimension<U>> for OutputDimension<T>
where
    T: ops::Sub<U, Output = R>,
{
    type Output = R;

    fn sub(self, other: OutputDimension<U>) -> Self::Output {
        self.0 - other.0
    }
}

impl<R, T, U> ops::Add<U> for OutputDimension<T>
where
    T: ops::Add<U, Output = R>,
{
    type Output = OutputDimension<R>;

    fn add(self, other: U) -> Self::Output {
        OutputDimension(self.0 + other)
    }
}

impl<T, U> AddAssign<U> for OutputDimension<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, other: U) {
        self.0 += other;
    }
}

impl<T, U> SubAssign<U> for OutputDimension<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, other: U) {
        self.0 -= other;
    }
}

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Debug, Default)]
struct ExcerptDimension<T>(T);

impl<T: PartialEq> PartialEq<T> for ExcerptDimension<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for ExcerptDimension<T> {
    fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl ExcerptOffset {
    fn saturating_sub(self, other: ExcerptOffset) -> usize {
        self.0.saturating_sub(other.0)
    }
}

impl<R, T, U> ops::Sub<ExcerptDimension<U>> for ExcerptDimension<T>
where
    T: ops::Sub<U, Output = R>,
{
    type Output = R;

    fn sub(self, other: ExcerptDimension<U>) -> Self::Output {
        self.0 - other.0
    }
}

impl<R, T, U> ops::Add<U> for ExcerptDimension<T>
where
    T: ops::Add<U, Output = R>,
{
    type Output = ExcerptDimension<R>;

    fn add(self, other: U) -> Self::Output {
        ExcerptDimension(self.0 + other)
    }
}

impl<T, U> AddAssign<U> for ExcerptDimension<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, other: U) {
        self.0 += other;
    }
}

impl<T, U> SubAssign<U> for ExcerptDimension<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, other: U) {
        self.0 -= other;
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for MultiBufferOffset {
    fn zero(_: ()) -> Self {
        MultiBufferOffset::ZERO
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: ()) {
        *self += summary.output.len;
    }
}

impl<MBD> sum_tree::SeekTarget<'_, DiffTransformSummary, DiffTransformSummary>
    for ExcerptDimension<MBD>
where
    MBD: MultiBufferDimension + Ord,
{
    fn cmp(&self, cursor_location: &DiffTransformSummary, _: ()) -> cmp::Ordering {
        Ord::cmp(&self.0, &MBD::from_summary(&cursor_location.input))
    }
}

impl<'a, MBD> sum_tree::SeekTarget<'a, DiffTransformSummary, DiffTransforms<MBD>>
    for ExcerptDimension<MBD>
where
    MBD: MultiBufferDimension + Ord,
{
    fn cmp(&self, cursor_location: &DiffTransforms<MBD>, _: ()) -> cmp::Ordering {
        Ord::cmp(&self.0, &cursor_location.excerpt_dimension.0)
    }
}

impl<'a, MBD: MultiBufferDimension> sum_tree::Dimension<'a, DiffTransformSummary>
    for ExcerptDimension<MBD>
{
    fn zero(_: ()) -> Self {
        ExcerptDimension(MBD::default())
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: ()) {
        self.0.add_mb_text_summary(&summary.input)
    }
}

impl<'a, MBD> sum_tree::SeekTarget<'a, DiffTransformSummary, DiffTransforms<MBD>>
    for OutputDimension<MBD>
where
    MBD: MultiBufferDimension + Ord,
{
    fn cmp(&self, cursor_location: &DiffTransforms<MBD>, _: ()) -> cmp::Ordering {
        Ord::cmp(&self.0, &cursor_location.output_dimension.0)
    }
}

impl<'a, MBD: MultiBufferDimension> sum_tree::Dimension<'a, DiffTransformSummary>
    for OutputDimension<MBD>
{
    fn zero(_: ()) -> Self {
        OutputDimension(MBD::default())
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: ()) {
        self.0.add_mb_text_summary(&summary.output)
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
                wrapped_buffer_row: None,
            });
        }

        let mut region = self.cursor.region()?.clone();
        while self.point >= region.range.end {
            self.cursor.next();
            if let Some(next_region) = self.cursor.region() {
                region = next_region.clone();
            } else if self.point == self.cursor.diff_transforms.end().output_dimension.0 {
                let multibuffer_row = MultiBufferRow(self.point.row);
                let last_excerpt = self
                    .cursor
                    .excerpts
                    .item()
                    .or(self.cursor.excerpts.prev_item())?;
                let buffer_snapshot = last_excerpt.buffer_snapshot(self.cursor.snapshot);
                let last_row = last_excerpt.range.context.end.to_point(buffer_snapshot).row;

                let first_row = last_excerpt
                    .range
                    .context
                    .start
                    .to_point(buffer_snapshot)
                    .row;

                let expand_info = if self.is_singleton {
                    None
                } else {
                    let needs_expand_up = first_row == last_row
                        && last_row > 0
                        && !region.diff_hunk_status.is_some_and(|d| d.is_deleted());
                    let needs_expand_down = last_row < buffer_snapshot.max_point().row;

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
                        start_anchor: Anchor::Excerpt(last_excerpt.start_anchor()),
                    })
                };
                self.point += Point::new(1, 0);
                return Some(RowInfo {
                    buffer_id: Some(last_excerpt.buffer_id),
                    buffer_row: Some(last_row),
                    multibuffer_row: Some(multibuffer_row),
                    diff_status: None,
                    wrapped_buffer_row: None,
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
                start_anchor: Anchor::Excerpt(region.excerpt.start_anchor()),
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
            wrapped_buffer_row: None,
        });
        self.point += Point::new(1, 0);
        result
    }
}

impl<'a> MultiBufferChunks<'a> {
    pub fn offset(&self) -> MultiBufferOffset {
        self.range.start
    }

    pub fn seek(&mut self, range: Range<MultiBufferOffset>) {
        self.diff_transforms.seek(&range.end, Bias::Right);
        let mut excerpt_end = self.diff_transforms.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            let overshoot = range.end - self.diff_transforms.start().0;
            excerpt_end += overshoot;
        }

        self.diff_transforms.seek(&range.start, Bias::Right);
        let mut excerpt_start = self.diff_transforms.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = self.diff_transforms.item() {
            let overshoot = range.start - self.diff_transforms.start().0;
            excerpt_start += overshoot;
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
                .filter(|chunks| excerpt.end_anchor() == chunks.end)
            {
                excerpt.seek_chunks(
                    excerpt_chunks,
                    (self.excerpt_offset_range.start - excerpt_start)
                        ..(self.excerpt_offset_range.end - excerpt_start),
                    self.snapshot,
                );
            } else {
                self.excerpt_chunks = Some(excerpt.chunks_in_range(
                    (self.excerpt_offset_range.start - excerpt_start)
                        ..(self.excerpt_offset_range.end - excerpt_start),
                    self.language_aware,
                    self.snapshot,
                ));
            }
        } else {
            self.excerpt_chunks = None;
        }
    }

    #[ztracing::instrument(skip_all)]
    fn next_excerpt_chunk(&mut self) -> Option<Chunk<'a>> {
        loop {
            if self.excerpt_offset_range.is_empty() {
                return None;
            } else if let Some(chunk) = self.excerpt_chunks.as_mut()?.next() {
                self.excerpt_offset_range.start += chunk.text.len();
                return Some(chunk);
            } else {
                self.excerpts.next();
                let excerpt = self.excerpts.item()?;
                self.excerpt_chunks = Some(excerpt.chunks_in_range(
                    0..(self.excerpt_offset_range.end - *self.excerpts.start()),
                    self.language_aware,
                    self.snapshot,
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

    #[ztracing::instrument(skip_all)]
    fn next(&mut self) -> Option<Chunk<'a>> {
        if self.range.start >= self.range.end {
            return None;
        }
        if self.range.start == self.diff_transforms.end().0 {
            self.diff_transforms.next();
        }

        let diff_transform_start = self.diff_transforms.start().0;
        let diff_transform_end = self.diff_transforms.end().0;
        debug_assert!(
            self.range.start < diff_transform_end,
            "{:?} < {:?} of ({1:?}..{2:?})",
            self.range.start,
            diff_transform_end,
            diff_transform_start
        );

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
                    let mask = 1u128.unbounded_shl(split_idx as u32).wrapping_sub(1);
                    let chars = chunk.chars & mask;
                    let tabs = chunk.tabs & mask;
                    let newlines = chunk.newlines & mask;

                    chunk.text = after;
                    chunk.chars = chunk.chars >> split_idx;
                    chunk.tabs = chunk.tabs >> split_idx;
                    chunk.newlines = chunk.newlines >> split_idx;

                    Some(Chunk {
                        text: before,
                        chars,
                        tabs,
                        newlines,
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
                    base_text_byte_range.start + (self.range.start - diff_transform_start);
                let base_text_end =
                    base_text_byte_range.start + (self.range.end - diff_transform_start);
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
                    let base_buffer =
                        &find_diff_state(&self.snapshot.diffs, *buffer_id)?.base_text();
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
                        newlines: 1u128,
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
                            ..(region.buffer_range.start + (self.range.end - region.range.start))
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

        if self.has_footer {
            let text = "\n";
            let chars = 0b1;
            let newlines = 0b1;
            self.has_footer = false;
            return Some(Chunk {
                text,
                chars,
                newlines,
                ..Default::default()
            });
        }

        None
    }
}

impl ToOffset for Point {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffset {
        snapshot.point_to_offset(*self)
    }
    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffsetUtf16 {
        snapshot.point_to_offset_utf16(*self)
    }
}

impl ToOffset for MultiBufferOffset {
    #[track_caller]
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffset {
        assert!(
            *self <= snapshot.len(),
            "offset {} is greater than the snapshot.len() {}",
            self.0,
            snapshot.len().0,
        );
        *self
    }
    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffsetUtf16 {
        snapshot.offset_to_offset_utf16(*self)
    }
}

impl ToOffset for MultiBufferOffsetUtf16 {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffset {
        snapshot.offset_utf16_to_offset(*self)
    }

    fn to_offset_utf16(&self, _snapshot: &MultiBufferSnapshot) -> MultiBufferOffsetUtf16 {
        *self
    }
}

impl ToOffset for PointUtf16 {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffset {
        snapshot.point_utf16_to_offset(*self)
    }
    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffsetUtf16 {
        snapshot.point_utf16_to_offset_utf16(*self)
    }
}

impl ToPoint for MultiBufferOffset {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point {
        snapshot.offset_to_point(*self)
    }
    fn to_point_utf16<'a>(&self, snapshot: &MultiBufferSnapshot) -> PointUtf16 {
        snapshot.offset_to_point_utf16(*self)
    }
}

impl ToPoint for Point {
    fn to_point<'a>(&self, _: &MultiBufferSnapshot) -> Point {
        *self
    }
    fn to_point_utf16<'a>(&self, snapshot: &MultiBufferSnapshot) -> PointUtf16 {
        snapshot.point_to_point_utf16(*self)
    }
}

impl ToPoint for PointUtf16 {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point {
        snapshot.point_utf16_to_point(*self)
    }
    fn to_point_utf16<'a>(&self, _: &MultiBufferSnapshot) -> PointUtf16 {
        *self
    }
}

#[cfg(debug_assertions)]
pub mod debug {
    use super::*;

    pub trait ToMultiBufferDebugRanges {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<MultiBufferOffset>>;
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for T {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<MultiBufferOffset>> {
            [self.to_offset(snapshot)].to_multi_buffer_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for Range<T> {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<MultiBufferOffset>> {
            [self.start.to_offset(snapshot)..self.end.to_offset(snapshot)]
                .to_multi_buffer_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for Vec<T> {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<MultiBufferOffset>> {
            self.as_slice().to_multi_buffer_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for Vec<Range<T>> {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<MultiBufferOffset>> {
            self.as_slice().to_multi_buffer_debug_ranges(snapshot)
        }
    }

    impl<T: ToOffset> ToMultiBufferDebugRanges for [T] {
        fn to_multi_buffer_debug_ranges(
            &self,
            snapshot: &MultiBufferSnapshot,
        ) -> Vec<Range<MultiBufferOffset>> {
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
        ) -> Vec<Range<MultiBufferOffset>> {
            self.iter()
                .map(|range| range.start.to_offset(snapshot)..range.end.to_offset(snapshot))
                .collect()
        }
    }
}
